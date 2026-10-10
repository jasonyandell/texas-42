//! Exploratory vector-returning seam on the pinned production level-0 modeled
//! mind (walt `cb1ef3b2`, byte-identical at HEAD). Reproduces the generic tail
//! of `Solver::pi(0, ..)`: Voidless 8-world bundle, per-world dice seeds,
//! `Field::Dice` recursion, `action_values`, `best_of`. Production code is not
//! modified; `modeled_choice(0, ..)` is the differential oracle.
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::{sync::Arc, time::Duration};
use walt::{
    rules::Seat,
    solver::{self, Contract, Deadline, Field, InnerBelief, Key, MoveOrdering, Shared, Solver, SplitMix64, INNER_SEED},
};

pub const TEACHER: &str = "WALT-INNER-L0MIND-n8-DICE-VOIDLESS-FIXED-BID30-STRAIGHT-DECLMAKE-v1";
pub const REFERENCE: &str = "WALT-INNER-L0MIND-n8-DICE-VOIDLESS-FIXED-BID30-STRAIGHT-DECLMAKE-refseed-v1";
const N0: usize = 8;

fn scalar(r: &Value, key: &str) -> Result<u64, String> {
    r[key].as_u64().ok_or_else(|| format!("missing/invalid {key}"))
}
fn list(r: &Value, key: &str) -> Result<Vec<u64>, String> {
    r[key].as_array().ok_or_else(|| format!("missing/invalid {key}"))?.iter().map(|v| v.as_u64().ok_or_else(|| format!("invalid {key}"))).collect()
}
/// Strict own/public request boundary: exactly the seven phone fields.
fn status(r: &Value) -> Result<Value, String> {
    let keys = r.as_object().ok_or("request object")?;
    if keys.len() != 7 || keys.keys().any(|k| !["decl", "bid", "bidder", "seat", "hand", "plays", "seed"].contains(&k.as_str())) {
        return Err("exactly seven public/own request fields required".into());
    }
    let words = |xs: &[u64]| xs.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
    let text = format!("status\ndecl {}\nbid {}\nbidder {}\nseat {}\nhand {}\nplays {}\nseed {}\n",
        scalar(r, "decl")?, scalar(r, "bid")?, scalar(r, "bidder")?, scalar(r, "seat")?, words(&list(r, "hand")?), words(&list(r, "plays")?), scalar(r, "seed")?);
    serde_json::from_str(&solver::partnership_wire::run(&text)?).map_err(|e| e.to_string())
}

/// One inner-mind evaluation. `ref_seed` (optional) selects the declared
/// reference estimator: an extra mixed term in the production RNG seed.
/// `check` additionally runs the production fast-path choice for comparison.
pub fn inner(input: &Value) -> Result<Value, String> {
    let start = std::time::Instant::now();
    let r = input.get("request").ok_or("missing request")?;
    let public = status(r)?;
    let budget = input.get("budget_ms").map_or(Ok(2000), |v| v.as_u64().ok_or("invalid budget"))?;
    let decl = scalar(r, "decl")? as usize;
    let bid = scalar(r, "bid")? as u8;
    if bid != 30 { return Err("this seam is frozen at bid 30".into()); }
    let bidder = scalar(r, "bidder")? as usize;
    let seat = scalar(r, "seat")? as usize;
    let hand = list(r, "hand")?;
    let plays = list(r, "plays")?;
    let dcl = solver::decl_of(decl);
    let pairs: Vec<(usize, usize)> = plays.chunks(2).map(|p| (p[0] as usize, p[1] as usize)).collect();
    let st = solver::replay(dcl, bidder, &pairs);
    let key = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: None, alive: 0 };
    let contract = Contract::Straight { bid };
    if contract.terminal(&key).is_some() { return Ok(json!({"ineligible": "settled", "status": public})); }
    let actor = (seat + st.r) % 4;
    if contract.actor(st.leader as usize, st.plays.len()).index() != actor { return Err("not this seat's turn".into()); }
    let original = hand.iter().fold(0u32, |m, &t| m | (1u32 << t));
    let own = original & !st.played;
    let legal: Vec<u8> = public["legal"].as_array().ok_or("legal")?.iter().map(|v| v.as_u64().unwrap() as u8).collect();
    let legal_mask = legal.iter().fold(0u32, |m, &t| m | (1u32 << t));
    if legal_mask & !own != 0 { return Err("legal set outside own hand".into()); }
    let boundary_played = st.trick_start_played;
    let boundary_size = 7 - st.completed;
    let sizes = contract.sizes(&key, boundary_played, boundary_size);
    let deadline = Deadline::after(Duration::from_millis(budget));
    let shared = Arc::new(Shared::new(dcl, bid, vec![N0, 2], boundary_played, boundary_size, deadline).with_inner_belief(InnerBelief::Voidless));
    let seat_i = Seat::from_index(actor).ok_or("seat")?;
    let maximize = actor % 2 == 1;
    let ref_seed = input.get("ref_seed").and_then(Value::as_u64);
    let extra = ref_seed.map_or(0, |s| solver::mix(0x5245_4653 ^ s));
    let mut rng = SplitMix64(INNER_SEED ^ extra ^ solver::mix(actor as u64) ^ solver::mix(u64::from(own)) ^ solver::record_hash(&key));
    let worlds = InnerBelief::Voidless.sample(dcl, seat_i, own, &key, sizes, N0, &mut rng, deadline).ok_or("deadline during sampling")?;
    let seeds: Vec<u64> = (0..N0).map(|_| rng.next_u64()).collect();
    let dice = Solver::new(Arc::clone(&shared), seat_i, own, maximize, worlds.clone(), seeds, Field::Dice).with_ordering(MoveOrdering::CaptureFirst);
    let values = dice.action_values(&key, &legal).ok_or("deadline during evaluation")?;
    dice.flush_nodes();
    let choice = solver::best_of(&values, maximize);
    let counts: Vec<u64> = values.iter().map(|(_, v)| {
        let n = v * BigRational::from_integer(BigInt::from(N0 as u64));
        assert!(n.is_integer(), "inner values are multiples of 1/8");
        n.to_integer().to_u64().unwrap()
    }).collect();
    let production = if input["check"].as_bool() == Some(true) && ref_seed.is_none() {
        let host = Solver::new(Arc::clone(&shared), seat_i, own, maximize, worlds.clone(), Vec::new(), Field::Level(0));
        let c = host.modeled_choice(0, &key, seat_i, own, legal_mask);
        host.flush_nodes();
        Some(c.ok_or("production choice deadline")?)
    } else { None };
    Ok(json!({
        "teacher": if ref_seed.is_some() { REFERENCE } else { TEACHER }, "ref_seed": ref_seed,
        "legal": legal, "values": values.iter().map(|(t, v)| json!([t, v.numer().to_string(), v.denom().to_string()])).collect::<Vec<_>>(),
        "counts": counts, "worlds": N0, "choice": choice, "maximize": maximize, "declaring": maximize, "internal_seat": actor,
        "production_choice": production, "agree": production.map(|p| p == choice),
        "status": public, "elapsed_us": start.elapsed().as_micros() as u64,
        "features": if cfg!(feature = "cpu-speedups") { "cpu-speedups" } else { "reference-paths" },
    }))
}

pub fn handle(text: &str) -> Value {
    if text.len() > 16_384 { return json!({"error": "request too large"}); }
    match serde_json::from_str::<Value>(text).map_err(|e| e.to_string()).and_then(|v| inner(&v)) {
        Ok(v) => v,
        Err(e) => json!({"error": e}),
    }
}
