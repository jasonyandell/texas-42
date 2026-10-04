//! Local complete-player bridge's late component. Public/own request only.
use native_frontier::{Caps, Context, Nature, Query, Service, State};
use serde_json::{json, Value};
use std::{io::{self, BufRead}, time::{Duration, Instant}};
use walt::solver::{self, Contract, SplitMix64};

fn scalar(r: &Value, key: &str) -> Result<u64, String> {
    r[key].as_u64().ok_or_else(|| format!("missing/invalid {key}"))
}
fn list(r: &Value, key: &str) -> Result<Vec<u64>, String> {
    r[key].as_array().ok_or_else(|| format!("missing/invalid {key}"))?
        .iter().map(|v| v.as_u64().ok_or_else(|| format!("invalid {key}"))).collect()
}
fn run(input: Value) -> Result<Value, String> {
    let start = Instant::now();
    let r = input.get("request").ok_or("missing request")?;
    let keys = r.as_object().ok_or("request object")?;
    if keys.len() != 7 || keys.keys().any(|k| !["decl","bid","bidder","seat","hand","plays","seed"].contains(&k.as_str())) {
        return Err("exactly seven public/own request fields required".into());
    }
    let decl = scalar(r,"decl")? as usize;
    let bid = scalar(r,"bid")? as u8;
    let bidder = scalar(r,"bidder")? as usize;
    let seat = scalar(r,"seat")? as usize;
    let seed = scalar(r,"seed")?;
    let hand = list(r,"hand")?;
    let plays = list(r,"plays")?;
    // Shared deployed strict validator reconstructs ownership, turns, voids,
    // own historical legality and exact hidden capacity feasibility.
    let words = |xs: &[u64]| xs.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
    let text = format!("status\ndecl {}\nbid {}\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {}\n",
        scalar(r,"decl")?,scalar(r,"bid")?,scalar(r,"bidder")?,scalar(r,"seat")?,words(&hand),words(&plays),seed);
    let status: Value = serde_json::from_str(&solver::partnership_wire::run(&text)?).map_err(|e|e.to_string())?;
    if !(32..=48).contains(&plays.len()) || status["legal"].as_array().unwrap().len() < 2 {
        return Ok(json!({"ineligible":true,"status":status}));
    }
    let pairs = plays.chunks(2).map(|p|(p[0] as usize,p[1] as usize)).collect::<Vec<_>>();
    let st = solver::replay(solver::decl_of(decl), bidder, &pairs);
    let key = solver::Key { played:st.played, leader:st.leader, plays:st.plays.clone(), banked_t1:st.banked_t1, banked_t0:st.banked_t0, voids:None, alive:0 };
    if (Contract::Straight {bid}).terminal(&key).is_some() {return Ok(json!({"ineligible":true,"status":status}));}
    let actor=(seat+st.r)%4;
    let original = hand.iter().fold(0u32,|m,&t|m|(1u32<<t));
    let own=original & !st.played;
    let sizes=Contract::Straight {bid}.sizes(&key,0,7);
    let sample_start=Instant::now();
    let mut rng=SplitMix64(seed ^ solver::mix(original as u64) ^ solver::record_hash(&key));
    // Same shuffle-and-reject stream as production outer sampling. Deadline
    // and attempt ceiling change only explicit refusal, never a complete draw.
    let mut tiles=solver::mask_bits(solver::FULL_MASK & !st.played & !own);
    let others=(0..4).filter(|&s|s!=actor).collect::<Vec<_>>();
    let mut worlds=Vec::new();let mut attempts=0;
    while worlds.len()<40 {
        attempts+=1;
        if attempts>100000 || sample_start.elapsed()>Duration::from_secs(1) {return Err("outer sample cap".into());}
        for i in (1..tiles.len()).rev() {let j=rng.below((i+1) as u64) as usize;tiles.swap(i,j);}
        let mut w=[0;4];w[actor]=own;let mut off=0;let mut accept=true;
        for &s in &others {w[s]=tiles[off..off+sizes[s]].iter().fold(0,|m,&t|m|(1u32<<t));off+=sizes[s];if w[s]&st.voids[s]!=0 {accept=false;break;}}
        if accept {worlds.push(w);}
    }
    let sample_us=sample_start.elapsed().as_micros();
    let context=Context {decl,bid,budgets:vec![4,2,2,2],boundary:st.trick_start_played,boundary_size:7-st.completed,counted:false};
    let q=Query {context,state:State::from_key(&key),actor:actor as u8,hand:own,worlds:worlds.clone(),nature:Nature::Policy(2)};
    let variant=input["variant"].as_str().unwrap_or("resumable");
    let mut service=Service::new(Caps {rows:500000,work:20000000,queries:50000,cache_entries:50000,seconds:2})?;
    service=match variant {"resumable"=>service.with_resumable_choices(),"arena"=>service.with_arena_choices(),"lazy"=>service.with_bounded_choices(),"native"=>service,_=>return Err("unknown kernel variant".into())};
    let answer=if variant=="native" {native_frontier::reference(&q,false,2)?.0} else {service.root_answers(&[q.clone()])?.remove(0)};
    if input["validate"].as_bool()==Some(true) {
        let expected=native_frontier::reference(&q,false,3)?.0;
        if answer!=expected {return Err("independent native complete-vector mismatch".into());}
        let mut check_rng=SplitMix64(seed ^ solver::mix(original as u64) ^ solver::record_hash(&key));
        let expected_worlds=solver::sample_belief(actor,own,key.played,sizes,st.voids,40,&mut check_rng).map_err(|e|format!("{e:?}"))?;
        if worlds!=expected_worlds || rng.0!=check_rng.0 {return Err("production outer stream mismatch".into());}
    }
    Ok(json!({"choice":answer.choice,"legal":status["legal"],"points":status["points"],"leader":status["leader"],"trick":status["trick"],
        "route":"late-l3-resumable-40-4-2-2","evaluation":answer,"policy":"uniform-support/voidless-inner/Field::Level(2)/ascending-ties",
        "outer_attempts":attempts,"outer_sample_us":sample_us,"elapsed_us":start.elapsed().as_micros(),"stats":service.stats,
        "kernel_variant":variant,"validated":input["validate"].as_bool()==Some(true),"worlds":if input["validate"].as_bool()==Some(true) {Some(worlds)} else {None}}))
}
fn main() {
    for line in io::stdin().lock().lines() {
        let result=line.map_err(|e|e.to_string()).and_then(|s|serde_json::from_str(&s).map_err(|e|e.to_string())).and_then(run);
        println!("{}",match result {Ok(v)=>v,Err(e)=>json!({"error":e})});
    }
}
