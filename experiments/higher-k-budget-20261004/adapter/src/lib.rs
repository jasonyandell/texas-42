//! Exploratory k ladder; unmodified pinned native recursion and phone policy.
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::{sync::{atomic::Ordering, Arc}, time::Duration};
use walt::{clock::Instant, rules::Seat, solver::{self, Contract, Deadline, Field, InnerBelief, Shared, Solver, SplitMix64}};
const INNER: [usize; 6] = [4,2,2,2,2,2];
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


pub fn ladder(input: &Value, mut checkpoint: impl FnMut(&Value)) -> Result<Value,String> {
    let start=Instant::now();
    let object=input.as_object().ok_or("request object")?;
    if object.keys().any(|k| !["action","request","k","budget_ms","validate"].contains(&k.as_str())) {return Err("unknown ladder fields".into());}
    let raw_k=scalar(input,"k")?;
    if !(1..=5).contains(&raw_k) {return Err("invalid k/budget".into());}
    let k=raw_k as usize;
    let ms=scalar(input,"budget_ms")?;
    if ms>30_000 {return Err("invalid k/budget".into());}
    let allotted=Duration::from_millis(ms);
    let r=input.get("request").ok_or("missing request")?;
    let public=status(r)?;
    let status_us=start.elapsed().as_micros();
    let plays=list(r,"plays")?;
    let legal=public["legal"].as_array().ok_or("status legal")?.iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>();
    let mut result=json!({"choice":legal[0],"legal":legal,"points":public["points"],"leader":public["leader"],"trick":public["trick"],
        "route":"ladder-legal-checkpoint","completed":false,"k":k,"field_level":k-1,"inner_budgets":INNER,
        "outer_worlds":40,"budget_ms":ms,"status_us":status_us,"policy":"uniform40/Voidless-inner/fixed-selection/ascending-ties"});
    checkpoint(&result);
    if !(24..=48).contains(&plays.len()) || legal.len()<2 {result["ineligible"]=json!(true);result["elapsed_us"]=json!(start.elapsed().as_micros());return Ok(result);}
    let decl=scalar(r,"decl")? as usize;
    let bid=scalar(r,"bid")? as u8;
    let bidder=scalar(r,"bidder")? as usize;
    let seat=scalar(r,"seat")? as usize;
    let st=solver::replay(solver::decl_of(decl),bidder,&plays.chunks(2).map(|p|(p[0] as usize,p[1] as usize)).collect::<Vec<_>>());
    let key=solver::Key{played:st.played,leader:st.leader,plays:st.plays.clone(),banked_t1:st.banked_t1,banked_t0:st.banked_t0,voids:None,alive:0};
    if (Contract::Straight{bid}).terminal(&key).is_some() {result["ineligible"]=json!(true);result["elapsed_us"]=json!(start.elapsed().as_micros());return Ok(result);}
    let actor=(seat+st.r)%4;
    let original=list(r,"hand")?.iter().fold(0u32,|m,&t|m|(1u32<<t));
    let own=original & !st.played;
    let sizes=Contract::Straight{bid}.sizes(&key,0,7);
    let mut rng=SplitMix64(scalar(r,"seed")? ^ solver::mix(original as u64) ^ solver::record_hash(&key));
    let mut tiles=solver::mask_bits(solver::FULL_MASK & !st.played & !own);
    let others=(0..4).filter(|&s|s!=actor).collect::<Vec<_>>();
    let sampling=Instant::now();let mut worlds=Vec::new();let mut attempts=0;
    while worlds.len()<40 {
        if attempts>=100_000 || start.elapsed()>=allotted {
            result["refusal"]=json!("outer-sampling-budget");
            result["outer_attempts"]=json!(attempts);result["outer_accepted"]=json!(worlds.len());
            result["outer_sample_us"]=json!(sampling.elapsed().as_micros());result["elapsed_us"]=json!(start.elapsed().as_micros());return Ok(result);
        }
        attempts+=1;
        for i in (1..tiles.len()).rev() {let j=rng.below((i+1) as u64) as usize;tiles.swap(i,j);}
        let mut world=[0;4];world[actor]=own;let mut off=0;let mut accept=true;
        for &s in &others {world[s]=tiles[off..off+sizes[s]].iter().fold(0,|m,&t|m|(1u32<<t));off+=sizes[s];if world[s]&st.voids[s]!=0 {accept=false;break;}}
        if accept {worlds.push(world);}
    }
    result["outer_attempts"]=json!(attempts);result["outer_accepted"]=json!(40);
    result["outer_sample_us"]=json!(sampling.elapsed().as_micros());
    let remaining=allotted.saturating_sub(start.elapsed());
    let shared=Arc::new(Shared::new(solver::decl_of(decl),bid,INNER.to_vec(),st.trick_start_played,7-st.completed,Deadline::after(remaining)).with_inner_belief(InnerBelief::Voidless));
    let evaluator=Solver::new(Arc::clone(&shared),Seat::from_index(actor).unwrap(),own,actor%2==1,worlds.clone(),Vec::new(),Field::Level(k-1));
    let tick=Instant::now();let values=evaluator.action_values(&key,&legal);
    result["solve_us"]=json!(tick.elapsed().as_micros());result["nodes"]=json!(shared.nodes.load(Ordering::Relaxed));
    result["pi_calls_by_level"]=json!(shared.pi_calls_by_level());result["inner_worlds_by_level"]=json!(shared.inner_worlds_by_level());
    result["policy_cache_entries"]=json!(shared.pi_cache_len());
    let validate=input["validate"].as_bool()==Some(true);
    if validate {
        let mut expected_rng=SplitMix64(scalar(r,"seed")? ^ solver::mix(original as u64) ^ solver::record_hash(&key));
        let expected=solver::sample_belief(actor,own,key.played,sizes,st.voids,40,&mut expected_rng).map_err(|e|format!("{e:?}"))?;
        if worlds!=expected || rng.0!=expected_rng.0 {return Err("production sample/RNG mismatch".into());}
        result["worlds"]=json!(worlds);result["rng_final"]=json!(rng.0.to_string());result["validated"]=json!(true);
    }
    if let Some(values)=values {
        // Audit validation is deliberately outside performance runs. Exact solve
        // completion is governed by its solver deadline; serialization charged
        // by the caller may exceed the allowance and is reported separately.
        let counts=values.iter().map(|(_,v)|{let n=v*BigRational::from_integer(BigInt::from(40));assert!(n.is_integer());n.to_integer().to_u64().unwrap()}).collect::<Vec<_>>();
        let choice=solver::best_of(&values,actor%2==1);
        result["evaluation"]=json!({"tiles":legal,"counts":counts,"choice":choice});
        result["choice"]=json!(choice);result["completed"]=json!(true);result["route"]=json!("ladder-complete");
    } else {result["refusal"]=json!("solver-deadline");}
    result["elapsed_us"]=json!(start.elapsed().as_micros());Ok(result)
}
pub fn handle(text:&str,mut checkpoint:impl FnMut(&Value))->Value {
    if text.len()>16384 {return json!({"error":"request too large"});}
    let input:Value=match serde_json::from_str(text){Ok(v)=>v,Err(e)=>return json!({"error":e.to_string()})};
    if input["action"]=="phone" {
        let mut call=input.clone();call.as_object_mut().unwrap().remove("action");
        return walt_player::handle(&call.to_string(),&mut checkpoint);
    }
    if input["action"]!="ladder" {return json!({"error":"unknown action"});}
    ladder(&input,&mut checkpoint).unwrap_or_else(|e|json!({"error":e}))
}
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
