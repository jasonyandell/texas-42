//! Bounded production-core demand probe. Does not implement phone review/profile.
use std::{io::{self, BufRead}, sync::{Arc, atomic::Ordering}, time::{Duration, Instant}};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::Deserialize;
use serde_json::{json, Value};
use walt::{rules::{Domino, Seat, Team, legal_plays}, solver::{self, Key, Shared, Solver, Field, Deadline, SplitMix64}};

#[derive(Deserialize)]
struct Request { decl: usize, bid: u8, bidder: usize, seat: usize, hand: Vec<usize>, plays: Vec<usize> }
#[derive(Deserialize)]
struct Input { request: Request, outer: usize, level: usize, parallel: bool, seconds: u64 }

fn run(input: Input) -> Result<Value, String> {
    if input.outer == 0 || input.outer > 40 || input.level > 3 || input.seconds == 0 || input.seconds > 10 {
        return Err("probe budget invalid".into());
    }
    // Driver constructs and independently replays every request before this adapter.
    let req=input.request; let dcl=solver::decl_of(req.decl);
    let original=req.hand.iter().fold(0u32,|m,&t|m|(1<<t));
    let pairs:Vec<_>=req.plays.chunks(2).map(|p|(p[0],p[1])).collect();
    let st=solver::replay(dcl,req.bidder,&pairs);
    let seat=Seat::from_index((req.seat+st.r)%4).unwrap();
    let hand=original & !st.played;
    let key=Key{voids:None,played:st.played,leader:st.leader,plays:st.plays.clone(),
        banked_t1:st.banked_t1,banked_t0:st.banked_t0,alive:0};
    let led=key.plays.first().map(|t|dcl.led_context(Domino::from_index(*t as usize).unwrap()));
    let tiles=solver::mask_bits(solver::mask_of(legal_plays(dcl,solver::set_of(hand),led)));
    let contract=solver::Contract::Straight{bid:req.bid};
    let sizes=contract.sizes(&key,st.trick_start_played,7-st.completed);
    let deadline=Deadline::after(Duration::from_secs(input.seconds));
    let mut rng=SplitMix64(7042104 ^ solver::mix(u64::from(original)) ^ solver::record_hash(&key));
    let worlds=solver::sample_belief(seat.index(),hand,st.played,sizes,st.voids,input.outer,&mut rng)
        .map_err(|e|format!("{e:?}"))?;
    let sh=Arc::new(Shared::new(dcl,req.bid,vec![4,2,2,2],st.trick_start_played,7-st.completed,deadline));
    let solver=Solver::new(Arc::clone(&sh),seat,hand,seat.team()==Team::T1,worlds,Vec::new(),Field::Level(input.level));
    let solver=if input.parallel {solver.parallel()} else {solver};
    let start=Instant::now();
    let answer=solver.action_values(&key,&tiles);
    let elapsed=start.elapsed().as_micros();
    let counts=answer.map(|values|values.iter().map(|(_,r)|{
        let v=r*BigRational::from_integer(BigInt::from(input.outer));assert!(v.is_integer());
        let n=v.to_integer().to_u64().unwrap(); if seat.team()==Team::T1 {n} else {input.outer as u64-n}
    }).collect::<Vec<_>>());
    Ok(json!({"level":input.level,"parallel":input.parallel,"outer":input.outer,"counts":counts,
        "tiles":tiles,"solve_us":elapsed,"nodes":sh.nodes.load(Ordering::Relaxed),
        "pi_calls":sh.pi_calls_by_level(),"inner_worlds":sh.inner_worlds_by_level(),
        "retained_policy_cache_entries":sh.pi_cache_len(),"inner_budgets":[4,2,2,2]}))
}
fn main(){
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    for line in io::stdin().lock().lines(){
        let value=serde_json::from_str::<Input>(&line.unwrap()).map_err(|e|e.to_string()).and_then(run);
        println!("{}",match value {Ok(v)=>v,Err(e)=>json!({"error":e})});
    }
}
