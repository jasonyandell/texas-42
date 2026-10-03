use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{io::{self, BufRead}, time::{Duration, Instant}};
use walt::{rules::Decl, solver::{decl_of, SplitMix64}};
use walt_response_ladder::{mechanics::*, player::{self, PlayerConfig}};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    decl: usize, bid: u8, bidder: u8, seat: u8,
    hand: Vec<u8>, original_hand: Vec<u8>, history: Vec<[u8; 2]>,
    seed: u64, budget_ms: f64,
    #[serde(default)] exact: bool,
    #[serde(default)] config: PlayerConfig,
}
fn mask(ts: &[u8]) -> Result<u32,String> {
    let mut m = 0;
    for &t in ts { if t >= 28 || m & (1 << t) != 0 { return Err("bad/repeated tile".into()); } m |= 1 << t; }
    Ok(m)
}
fn decl(id: usize) -> Result<Decl,String> {
    if id <= 7 || id == 9 { Ok(decl_of(id)) } else { Err("declaration must be 0..7 or 9".into()) }
}
fn answer(req: Request, evaluator: Option<&mut dyn walt_response_ladder::core::PolicyEvaluator>) -> Result<player::Decision,String> {
    if req.bidder >= 4 || req.seat >= 4 || !req.budget_ms.is_finite()
        || !(0.0..=86_400_000.0).contains(&req.budget_ms) {
        return Err("invalid seat/budget".into());
    }
    let start = Instant::now();
    let decl = decl(req.decl)?;
    let r = u8::from(req.bidder % 2 == 0);
    let mut public = PublicState::opening((req.bidder + r) % 4);
    for [actor, tile] in req.history {
        if actor >= 4 || (actor+r)%4 != public.actor() || tile >= 28 || public.played & (1 << tile) != 0 {
            return Err("invalid public history".into());
        }
        public = public.after(decl, tile);
    }
    let remaining = Duration::from_secs_f64(req.budget_ms/1000.).saturating_sub(start.elapsed());
    let mut result = player::decide_with_evaluator(decl,req.bid,(req.seat+r)%4,mask(&req.hand)?,mask(&req.original_hand)?,
        &public,req.seed,remaining,req.exact,&req.config,evaluator)?;
    result.elapsed_ms = start.elapsed().as_secs_f64()*1000.;
    Ok(result)
}
fn worker() -> Result<(),String> {
    for line in io::stdin().lock().lines() {
        let line = line.map_err(|e|e.to_string())?;
        let value = serde_json::from_str::<Request>(&line).map_err(|e|e.to_string()).and_then(|r|answer(r,None));
        match value {
            Ok(v) => println!("{}",serde_json::to_string(&v).unwrap()),
            Err(e) => println!("{}",json!({"error":e})),
        }
    }
    Ok(())
}
#[cfg(feature = "gpu")]
fn worker_gpu() -> Result<(), String> {
    use std::io::Write;
    let mut gpu = walt_response_ladder::gpu_epochs::EpochEvaluator::new()?;
    println!("{}",json!({"ready":true,"backend":"gpu-epochs","adapter":gpu.adapter_name(),
        "initialization_ms":gpu.initialization_ms,"cpu_query_threads":rayon::current_num_threads()}));
    io::stdout().flush().map_err(|e|e.to_string())?;
    for line in io::stdin().lock().lines() {
        let line = line.map_err(|e|e.to_string())?;
        gpu.reset_stats();
        let result = serde_json::from_str::<Request>(&line).map_err(|e|e.to_string())
            .and_then(|req|answer(req,Some(&mut gpu)));
        let mut output = match result {
            Ok(v) => serde_json::to_value(v).map_err(|e|e.to_string())?,
            Err(e) => json!({"error":e}),
        };
        output["compute"] = json!({"backend":"gpu-epochs","stats":gpu.stats});
        println!("{}",output);
        io::stdout().flush().map_err(|e|e.to_string())?;
    }
    Ok(())
}
fn arg<T: std::str::FromStr>(args:&[String], key:&str, default:T) -> Result<T,String> {
    if let Some(i) = args.iter().position(|a|a==key) {
        args.get(i+1).ok_or_else(||format!("missing {key}"))?.parse().map_err(|_|format!("bad {key}"))
    } else { Ok(default) }
}
#[derive(Serialize)]
struct Play { actor:u8, tile:u8, allowance_ms:f64, decision:player::Decision }
fn game(args: &[String]) -> Result<(),String> {
    let seed = arg(args,"--seed",910001u64)?;
    let decl_id = arg(args,"--decl",6usize)?;
    let dcl = decl(decl_id)?;
    let bidder = arg(args,"--bidder",1u8)?;
    let bid = arg(args,"--bid",30u8)?;
    let bank_ms = arg(args,"--ms",500f64)?;
    let team = arg(args,"--candidate-team",String::from("both"))?;
    if bidder>=4 || !(1..=42).contains(&bid) || !bank_ms.is_finite()
        || !(0.0..=86_400_000.0).contains(&bank_ms) ||
        !["both","declarer","defender","neither"].contains(&team.as_str()) { return Err("bad game contract/team/budget".into()); }
    let config = PlayerConfig { field: arg(args,"--field",String::from("partner"))?,
        outer: arg(args,"--outer",40)?, n0:arg(args,"--n0",8)?, n1:arg(args,"--n1",2)?,
        plans:arg(args,"--plans",8)?, horizon:arg(args,"--horizon",7)?,
        work:arg(args,"--work",2_000_000u64)? };
    let start = Instant::now();
    let r = u8::from(bidder%2==0);
    let mut deck: Vec<u8> = (0..28).collect();
    let mut rng = SplitMix64(seed);
    for i in (1..28).rev() { let j=rng.below((i+1)as u64)as usize; deck.swap(i,j); }
    let arena_hands:Vec<Vec<u8>> = deck.chunks(7).map(|h|h.to_vec()).collect();
    let mut original = [0u32;4];
    for a in 0..4 { original[(a+r as usize)%4]=mask(&arena_hands[a])?; }
    let mut public = PublicState::opening((bidder+r)%4);
    let mut banks = [bank_ms/2.;2];
    let mut plays = Vec::new();
    let mut overrun_ms = [0f64;2];
    for _ in 0..28 {
        let actor = public.actor();
        let actor_team = (actor%2)as usize;
        let left = (original[actor_team] & !public.played).count_ones()
            + (original[actor_team+2] & !public.played).count_ones();
        let allowance_ms = banks[actor_team].max(0.)/f64::from(left);
        let candidate = match team.as_str() { "both"=>true, "declarer"=>actor_team==1,
            "defender"=>actor_team==0,_=>false };
        let hand = original[actor as usize] & !public.played;
        let before = Instant::now();
        let mut decision = player::decide(dcl,bid,actor,hand,original[actor as usize],&public,
            seed ^ 0xCA11_AB1E,Duration::from_secs_f64(allowance_ms/1000.),!candidate,&config)?;
        decision.elapsed_ms = before.elapsed().as_secs_f64()*1000.;
        if decision.elapsed_ms > banks[actor_team] { overrun_ms[actor_team] += decision.elapsed_ms-banks[actor_team].max(0.); }
        banks[actor_team] = (banks[actor_team]-decision.elapsed_ms).max(0.);
        let tile = decision.tile;
        if public.legal(dcl,hand)&(1<<tile)==0 { return Err("player emitted illegal move".into()); }
        public = public.after(dcl,tile);
        plays.push(Play{actor:(actor+4-r)%4,tile,allowance_ms,decision});
    }
    if public.played.count_ones()!=28 || public.banked_t1+public.banked_t0!=42 { return Err("incomplete game".into()); }
    let elapsed_ms=start.elapsed().as_secs_f64()*1000.;
    let fallback_count=plays.iter().filter(|p|p.decision.fallback).count();
    let certified_count=plays.iter().filter(|p|p.decision.report.as_ref().is_some_and(|r|r.canonical_certified)).count();
    println!("{}",json!({"schema":"response-ladder-game-v1","seed":seed,"decl":decl_id,
        "bidder":bidder,"bid":bid,"hands":arena_hands,"candidate_team":team,"config":config,
        "bank_ms_per_team":bank_ms/2.,"remaining_bank_ms":banks,"overrun_ms":overrun_ms,
        "elapsed_ms":elapsed_ms,"made":public.banked_t1>=bid,"bidder_points":public.banked_t1,
        "fallback_count":fallback_count,"certified_count":certified_count,"plays":plays,
        "reference":"frozen-v34-exact-serial-with-canonical-reserve",
        "note":"Exploratory finite-bundle response; no stronger-play claim. Bank includes sampling, solving, destruction and fallback; game also includes deal/referee/receipts. JSON serialization and binary load excluded."}));
    Ok(())
}
fn main() {
    let args:Vec<String>=std::env::args().collect();
    let result=match args.get(1).map(String::as_str) {
        Some("worker")=>worker(), Some("game")=>game(&args[2..]),
        #[cfg(feature = "gpu")]
        Some("worker-gpu")=>worker_gpu(),
        _=>Err("usage: response-player worker | worker-gpu (requires gpu feature) | game [--seed N --decl 0..7|9 --bidder 0..3 --bid 30 --ms 500 --candidate-team both|declarer|defender|neither --field dice|l0|partner|all-l1 --outer 40 --n0 8 --n1 2 --plans 8 --horizon 7]".into())
    };
    if let Err(e)=result { eprintln!("{e}"); std::process::exit(1); }
}
