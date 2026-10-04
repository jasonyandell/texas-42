//! Exploratory same-policy native recursion and portable complete-player ABI.
//! Production walt/walt-player are unmodified and source-v2 pinned separately.
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::{sync::{atomic::Ordering, Arc}, time::Duration};
use walt::{clock::Instant, rules::Seat, solver::{self, Contract, Deadline, Field, InnerBelief, Shared, Solver, SplitMix64}};

pub const ROUTE: &str = "late-l3-native-40-4-2-2";
fn scalar(r: &Value, key: &str) -> Result<u64, String> {
    r[key].as_u64().ok_or_else(|| format!("missing/invalid {key}"))
}
fn list(r: &Value, key: &str) -> Result<Vec<u64>, String> {
    r[key].as_array().ok_or_else(|| format!("missing/invalid {key}"))?
        .iter().map(|v| v.as_u64().ok_or_else(|| format!("invalid {key}"))).collect()
}

fn status(r: &Value) -> Result<Value, String> {
    let keys = r.as_object().ok_or("request object")?;
    if keys.len() != 7 || keys.keys().any(|k| !["decl", "bid", "bidder", "seat", "hand", "plays", "seed"].contains(&k.as_str())) {
        return Err("exactly seven public/own request fields required".into());
    }
    let hand = list(r, "hand")?;
    let plays = list(r, "plays")?;
    let words = |xs: &[u64]| xs.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
    let text = format!("status\ndecl {}\nbid {}\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {}\n",
        scalar(r,"decl")?,scalar(r,"bid")?,scalar(r,"bidder")?,scalar(r,"seat")?,words(&hand),words(&plays),scalar(r,"seed")?);
    serde_json::from_str(&solver::partnership_wire::run(&text)?).map_err(|e| e.to_string())
}

/// Same complete policy as inherited native_frontier::reference(...,false,2).
/// Failure returns no partially evaluated vector. Sampling order/RNG unchanged.
pub fn late(input: &Value) -> Result<Value, String> {
    let start = Instant::now();
    let allotted = Duration::from_millis(input.get("budget_ms").map_or(Ok(4000), |v| v.as_u64().ok_or("invalid late budget"))?);
    let r = input.get("request").ok_or("missing request")?;
    let public = status(r)?;
    let status_us = start.elapsed().as_micros();
    let decl = scalar(r,"decl")? as usize;
    let bid = scalar(r,"bid")? as u8;
    let bidder = scalar(r,"bidder")? as usize;
    let seat = scalar(r,"seat")? as usize;
    let seed = scalar(r,"seed")?;
    let hand = list(r,"hand")?;
    let plays = list(r,"plays")?;
    if !(32..=48).contains(&plays.len()) || public["legal"].as_array().unwrap().len() < 2 {
        return Ok(json!({"ineligible":true,"status":public}));
    }
    let st = solver::replay(solver::decl_of(decl), bidder, &plays.chunks(2).map(|p| (p[0] as usize, p[1] as usize)).collect::<Vec<_>>());
    let key = solver::Key { played:st.played, leader:st.leader, plays:st.plays.clone(), banked_t1:st.banked_t1, banked_t0:st.banked_t0, voids:None, alive:0 };
    if (Contract::Straight {bid}).terminal(&key).is_some() {return Ok(json!({"ineligible":true,"status":public}));}
    let actor = (seat + st.r) % 4;
    let original = hand.iter().fold(0u32, |m, &t| m | (1u32 << t));
    let own = original & !st.played;
    let sizes = Contract::Straight {bid}.sizes(&key, 0, 7);
    let mut rng = SplitMix64(seed ^ solver::mix(original as u64) ^ solver::record_hash(&key));
    let mut tiles = solver::mask_bits(solver::FULL_MASK & !st.played & !own);
    let others = (0..4).filter(|&s| s != actor).collect::<Vec<_>>();
    let sampling = Instant::now();
    let mut worlds = Vec::new();
    let mut attempts = 0;
    while worlds.len() < 40 {
        attempts += 1;
        if attempts > 100_000 || sampling.elapsed() > Duration::from_secs(1) || start.elapsed() >= allotted {return Err("outer sample cap".into());}
        for i in (1..tiles.len()).rev() {let j = rng.below((i+1) as u64) as usize; tiles.swap(i,j);}
        let mut world = [0;4]; world[actor] = own;
        let mut off = 0; let mut accept = true;
        for &s in &others {
            world[s] = tiles[off..off+sizes[s]].iter().fold(0, |m,&t| m | (1u32<<t));
            off += sizes[s];
            if world[s] & st.voids[s] != 0 {accept = false; break;}
        }
        if accept {worlds.push(world);}
    }
    let sample_us = sampling.elapsed().as_micros();
    let remaining = allotted.saturating_sub(start.elapsed());
    if remaining.is_zero() {return Err("no late solver time".into());}
    let shared = Arc::new(Shared::new(solver::decl_of(decl), bid, vec![4,2,2,2], st.trick_start_played, 7-st.completed,
        Deadline::after(Duration::from_secs(2).min(remaining))).with_inner_belief(InnerBelief::Voidless));
    let evaluator = Solver::new(Arc::clone(&shared), Seat::from_index(actor).unwrap(), own, actor%2==1, worlds.clone(), Vec::new(), Field::Level(2));
    let legal = public["legal"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u8).collect::<Vec<_>>();
    let tick = Instant::now();
    let values = evaluator.action_values(&key, &legal).ok_or("pinned native reference deadline")?;
    let solve_us = tick.elapsed().as_micros();
    let counts: Vec<u64> = values.iter().map(|(_,v)| {
        let n = v * BigRational::from_integer(BigInt::from(40));
        assert!(n.is_integer()); n.to_integer().to_u64().unwrap()
    }).collect();
    let choice = solver::best_of(&values, actor%2==1);
    let validate = input["validate"].as_bool() == Some(true);
    if validate {
        let mut expected_rng = SplitMix64(seed ^ solver::mix(original as u64) ^ solver::record_hash(&key));
        let expected_worlds = solver::sample_belief(actor,own,key.played,sizes,st.voids,40,&mut expected_rng).map_err(|e| format!("{e:?}"))?;
        if worlds != expected_worlds || rng.0 != expected_rng.0 {return Err("production sample/RNG mismatch".into());}
    }
    Ok(json!({"choice":choice,"legal":public["legal"],"points":public["points"],"leader":public["leader"],"trick":public["trick"],
        "route":ROUTE,"evaluation":{"tiles":legal,"counts":counts,"choice":choice},
        "policy":"uniform-support/voidless-inner/Field::Level(2)/ascending-ties",
        "kernel_variant":"native","status_us":status_us,"outer_attempts":attempts,"outer_sample_us":sample_us,"solve_us":solve_us,
        "elapsed_us":start.elapsed().as_micros(),"nodes":shared.nodes.load(Ordering::Relaxed),
        "pi_calls_by_level":shared.pi_calls_by_level(),"inner_worlds_by_level":shared.inner_worlds_by_level(),
        "validated":validate,"worlds":if validate {Some(worlds)} else {None},"rng_final":if validate {Some(rng.0.to_string())} else {None}}))
}

/// Complete play-only hybrid: early/forced/settled/refused requests delegate to
/// source-identical pinned production player, same profile and remaining budget.
pub fn handle(text: &str, mut checkpoint: impl FnMut(&Value)) -> Value {
    if text.len() > 16_384 {return json!({"error":"request too large"});}
    let mut input: Value = match serde_json::from_str(text) {Ok(v) => v, Err(e) => return json!({"error":e.to_string()})};
    if input["action"] == "late" {
        return late(&input).unwrap_or_else(|e| json!({"error":e}));
    }
    let start = Instant::now();
    let phone_only = input["backend"] == "phone";
    if input.get("backend").is_some_and(|v| v != "phone" && v != "native") {return json!({"error":"unknown backend"});}
    if let Some(map) = input.as_object_mut() {map.remove("backend");}
    // Exact phone parsing/validation applies to the fallback. Reject complete
    // call options before a late solve, so invalid options cannot bypass it.
    let fields = ["request","worlds","partner","budget_ms"];
    if !input.as_object().is_some_and(|m| m.keys().all(|k| fields.contains(&k.as_str()))) {
        return json!({"error":"invalid complete call fields"});
    }
    let worlds = input.get("worlds").map_or(Some(40), Value::as_u64);
    let budget = input.get("budget_ms").map_or(Some(14000), Value::as_u64);
    let partner = input.get("partner").map_or(Some(true), Value::as_bool);
    if worlds.is_none() || budget.is_none() || partner.is_none() || !(1..=640).contains(&worlds.unwrap()) || !(100..=20000).contains(&budget.unwrap()) || (partner==Some(true) && worlds!=Some(40)) {
        return json!({"error":"invalid complete profile"});
    }
    let mut refusal = None;
    let mut retained = None;
    if !phone_only {
        match status(&input["request"]) {
            Ok(st) => {
                let legal = st["legal"].as_array().unwrap();
                let initial = json!({"choice":legal[0],"legal":legal,"points":st["points"],"leader":st["leader"],"trick":st["trick"],"route":"legal-fallback"});
                checkpoint(&initial);
                retained = Some(initial);
                let mut late_input = input.clone();
                late_input["budget_ms"] = json!(budget.unwrap().saturating_sub(start.elapsed().as_millis() as u64));
                match late(&late_input) {
                    Ok(v) if v["ineligible"] != true => {checkpoint(&v); return v;}
                    Ok(_) => (),
                    Err(e) => refusal = Some(e),
                }
            }
            Err(e) => return json!({"error":e}),
        }
    }
    if refusal.is_some() {
        let remaining = budget.unwrap().saturating_sub(start.elapsed().as_millis() as u64);
        if remaining < 100 {
            let mut result = retained.unwrap();
            result["late_refusal"] = json!(refusal.unwrap());
            result["elapsed_us"] = json!(start.elapsed().as_micros());
            return result;
        }
        input["budget_ms"] = json!(remaining);
    }
    let mut result = walt_player::handle(&input.to_string(), &mut checkpoint);
    if let Some(e) = refusal {result["late_refusal"] = json!(e);}
    result
}

// Same bounded ABI and host clock as pinned walt-player. Host copies checkpoint
// bytes synchronously; calls are fresh instances and cannot reenter the solver.
#[cfg(target_arch = "wasm32")]
mod abi {
    use std::{cell::RefCell, time::Duration};
    #[link(wasm_import_module = "walt_host")]
    extern "C" {fn now_us() -> u64; fn checkpoint(ptr: u32, len: u32);}
    fn now() -> Duration {Duration::from_micros(unsafe {now_us()})}
    thread_local! {
        static INPUT: RefCell<Vec<u8>> = const {RefCell::new(Vec::new())};
        static OUTPUT: RefCell<Vec<u8>> = const {RefCell::new(Vec::new())};
    }
    #[no_mangle]
    pub extern "C" fn native_late_in_prepare(len: u32) -> u32 {
        assert!(len <= 16384);
        INPUT.with(|b| {let mut v=b.borrow_mut();v.resize(len as usize,0);v.as_ptr() as u32})
    }
    #[no_mangle]
    pub extern "C" fn native_late_call() -> u32 {
        walt::clock::install(now);
        let text = INPUT.with(|b| String::from_utf8_lossy(&b.borrow()).into_owned());
        let value = super::handle(&text, |v| {let s=v.to_string();unsafe {checkpoint(s.as_ptr() as u32,s.len() as u32)}});
        OUTPUT.with(|b| {let mut v=b.borrow_mut();*v=value.to_string().into_bytes();v.len() as u32})
    }
    #[no_mangle]
    pub extern "C" fn native_late_out_ptr() -> u32 {OUTPUT.with(|b| b.borrow().as_ptr() as u32)}
}
