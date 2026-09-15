//! A bounded declaration survey at the cheapest legal auction bid.
//! Prices are L1 model estimates, not calibrated odds of winning at the table.
//! Only a completed sweep of ALL declarations replaces an earlier sweep.
use super::{emit, Request, Seed, PLAYER_ID};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{sync::Arc, time::Duration};
use walt::clock::Instant;
use walt::{
    rules::Seat,
    solver::{self, Deadline, Field, Key, Shared, Solver, SplitMix64},
};

const DECLS: [u64; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 9];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Auction {
    hand: Vec<u64>,
    seat: u64,
    bid: u64,
    seed: Seed,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    auction: Auction,
    #[serde(default = "budget")]
    budget_ms: u64,
    #[serde(default = "worlds")]
    worlds: usize,
}
fn budget() -> u64 {
    4500
}
fn worlds() -> usize {
    40
}

// Only the best opening value is needed for bidding. The shared solver can
// stop once that value is 1; play's full action vector cannot take that shortcut.
fn price(req: &Request, n: usize, ms: u64, seed: u64) -> Result<Value, String> {
    let hand = req.hand.iter().fold(0u32, |mask, t| mask | (1u32 << t));
    let leader = (req.seat + u64::from(req.seat % 2 == 0)) as u8;
    let key = Key {
        voids: None,
        played: 0,
        leader,
        plays: vec![],
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    let mut rng = SplitMix64(seed ^ solver::mix(u64::from(hand)) ^ solver::record_hash(&key));
    let worlds = solver::sample_open_belief(leader as usize, hand, 0, [7; 4], n, &mut rng);
    let sh = Arc::new(Shared::new(
        solver::decl_of(req.decl as usize),
        req.bid as u8,
        vec![8, 2],
        0,
        7,
        Deadline::after(Duration::from_millis(ms)),
    ));
    let solver = Solver::new(
        sh,
        Seat::from_index(leader as usize).unwrap(),
        hand,
        true,
        worlds,
        vec![],
        Field::SeatLevels([0; 4]),
    )
    .parallel();
    let v = solver.solve(&key).ok_or("deadline")?;
    Ok(json!([
        req.decl,
        v.numer().to_string(),
        v.denom().to_string()
    ]))
}

// Small exact fractions emitted by the completed finite-sample evaluator.
fn fraction(row: &Value) -> Result<(u64, u64), String> {
    let read = |i| {
        row[i]
            .as_str()
            .ok_or("missing price")?
            .parse::<u64>()
            .map_err(|_| "invalid price")
    };
    let (n, d) = (read(1)?, read(2)?);
    if d == 0 || n > d {
        return Err("invalid price".into());
    }
    Ok((n, d))
}
fn better(a: &Value, b: &Value) -> Result<bool, String> {
    let (an, ad) = fraction(a)?;
    let (bn, bd) = fraction(b)?;
    Ok(u128::from(an) * u128::from(bd) > u128::from(bn) * u128::from(ad))
}

pub fn decide(call: Call, mut checkpoint: impl FnMut(&Value)) -> Result<Value, String> {
    let start = Instant::now();
    if !(100..=14000).contains(&call.budget_ms) || ![4, 12, 40].contains(&call.worlds) {
        return Err("invalid auction budget".into());
    }
    let a = call.auction;
    let mut req = Request {
        decl: 0,
        bid: a.bid,
        bidder: a.seat,
        seat: a.seat,
        hand: a.hand,
        plays: vec![],
        seed: a.seed,
    };
    // Same strict own/public validation as play, before any checkpoint.
    walt::solver::partnership_wire::run(&format!("status\n{}", req.text()?))?;
    let seed = match &req.seed {
        Seed::Integer(n) => *n,
        Seed::Decimal(s) => s.parse::<u64>().map_err(|_| "invalid seed")?,
    };
    let mut value = json!({"schema":"walt-auction-v1","player_version":PLAYER_ID,
        "hand":req.hand,"seat":req.seat,"bid":req.bid,"seed":seed,
        "decl":0,"eligible":false,"prices":[],"worlds":0,"inner_worlds":8,
        "threshold":[3,4],"route":"unpriced-pass","elapsed_us":0,"phases":[]});
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    for n in [4, 12, 40].into_iter().filter(|n| *n <= call.worlds) {
        let mut prices = Vec::new();
        let mut failure = None;
        for decl in DECLS {
            let remaining = call
                .budget_ms
                .saturating_sub(start.elapsed().as_millis() as u64)
                .saturating_sub(80);
            let ms = remaining;
            if ms < 5 {
                failure = Some("no-time".to_owned());
                break;
            }
            req.decl = decl;
            match price(&req, n, ms, seed) {
                Ok(row) => prices.push(row),
                Err(error) => {
                    failure = Some(error);
                    break;
                }
            }
        }
        value["phases"].as_array_mut().unwrap().push(json!({"worlds":n,"completed_declarations":prices.len(),"status":failure.as_deref().unwrap_or("completed")}));
        if failure.is_some() {
            break;
        }
        let mut best = 0;
        for i in 1..prices.len() {
            if better(&prices[i], &prices[best])? {
                best = i;
            }
        }
        // Exact sample ties need no invented preference for blanks (first id).
        // The public seed fixes a repeatable choice among equally priced trumps.
        let tied = (0..prices.len())
            .filter(|&i| !better(&prices[best], &prices[i]).unwrap())
            .collect::<Vec<_>>();
        let mut tie_rng = SplitMix64(seed ^ solver::mix(req.hand.iter().sum()) ^ 0x424944544945);
        best = tied[tie_rng.below(tied.len() as u64) as usize];
        let (num, den) = fraction(&prices[best])?;
        value["decl"] = prices[best][0].clone();
        value["eligible"] = json!(u128::from(num) * 4 >= u128::from(den) * 3);
        value["prices"] = json!(prices);
        value["worlds"] = json!(n);
        value["route"] = json!("priced");
        emit(&mut value, start, call.budget_ms, &mut checkpoint);
    }
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    Ok(value)
}
