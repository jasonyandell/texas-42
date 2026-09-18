//! A bounded declaration survey at the cheapest legal auction bid.
//! Prices are L1 model estimates, not calibrated odds of winning at the table.
//! Only a completed sweep of ALL declarations replaces an earlier sweep.
use super::{emit, Request, Seed, PLAYER_ID};
use serde::{Deserialize, Serialize};
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
    20_000
}
fn worlds() -> usize {
    160
}

const ROUNDS: [usize; 4] = [4, 12, 40, 160];

/// Independent, deterministic jobs for hosts with a pool of ordinary workers.
/// The host schedules declarations; this module still owns all bidding math.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceCall {
    auction_price: Auction,
    decl: u64,
    worlds: usize,
    budget_ms: u64,
}
#[derive(Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct Identity {
    hand: Vec<u64>,
    seat: u64,
    bid: u64,
    seed: u64,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    schema: String,
    auction: Identity,
    worlds: usize,
    inner_worlds: usize,
    price: (u64, String, String),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MergeCall {
    auction_merge: Auction,
    worlds: usize,
    receipts: Vec<Receipt>,
}

fn request(a: Auction) -> Result<(Request, u64), String> {
    let req = Request {
        decl: 0,
        bid: a.bid,
        bidder: a.seat,
        seat: a.seat,
        hand: a.hand,
        plays: vec![],
        seed: a.seed,
    };
    // Same strict own/public validation as play, before any result/checkpoint.
    walt::solver::partnership_wire::run(&format!("status\n{}", req.text()?))?;
    let seed = match &req.seed {
        Seed::Integer(n) => *n,
        Seed::Decimal(s) => s.parse::<u64>().map_err(|_| "invalid seed")?,
    };
    Ok((req, seed))
}
fn identity(req: &Request, seed: u64) -> Identity {
    Identity {
        hand: req.hand.clone(),
        seat: req.seat,
        bid: req.bid,
        seed,
    }
}
fn empty(req: &Request, seed: u64) -> Value {
    json!({"schema":"walt-auction-v1","player_version":PLAYER_ID,
        "hand":req.hand,"seat":req.seat,"bid":req.bid,"seed":seed,
        "decl":0,"eligible":false,"prices":[],"worlds":0,"inner_worlds":8,
        "threshold":[3,4],"route":"unpriced-pass","elapsed_us":0,"phases":[]})
}

pub fn price_call(call: PriceCall) -> Result<Value, String> {
    if !DECLS.contains(&call.decl)
        || !ROUNDS.contains(&call.worlds)
        || !(5..=20_000).contains(&call.budget_ms)
    {
        return Err("invalid auction job".into());
    }
    let (mut req, seed) = request(call.auction_price)?;
    req.decl = call.decl;
    let row = price(&req, call.worlds, call.budget_ms, seed)?;
    Ok(
        json!({"schema":"walt-auction-price-v1","auction":identity(&req, seed),
        "worlds":call.worlds,"inner_worlds":8,"price":row}),
    )
}

/// No partial, duplicated, mixed-sample or mixed-position survey is rankable.
/// Sorting before selection makes the public-seeded tie independent of order.
pub fn merge(call: MergeCall) -> Result<Value, String> {
    let (req, seed) = request(call.auction_merge)?;
    let mut value = empty(&req, seed);
    if call.worlds == 0 && call.receipts.is_empty() {
        return Ok(value);
    }
    if !ROUNDS.contains(&call.worlds) || call.receipts.len() != DECLS.len() {
        return Err("incomplete auction survey".into());
    }
    let expected = identity(&req, seed);
    let mut prices = Vec::new();
    for receipt in call.receipts {
        if receipt.schema != "walt-auction-price-v1"
            || receipt.auction != expected
            || receipt.worlds != call.worlds
            || receipt.inner_worlds != 8
        {
            return Err("mismatched auction receipt".into());
        }
        prices.push(json!(receipt.price));
    }
    complete(&mut value, &req, seed, call.worlds, prices)?;
    value["phases"] =
        json!([{"worlds":call.worlds,"completed_declarations":9,"status":"completed"}]);
    Ok(value)
}

// Only the best opening value is needed for bidding. The shared solver can
// stop once that value is 1; play's full action vector cannot take that shortcut.
fn price(req: &Request, n: usize, ms: u64, seed: u64) -> Result<Value, String> {
    Ok(price_with_work(req, n, ms, seed)?.0)
}

fn price_with_work(req: &Request, n: usize, ms: u64, seed: u64) -> Result<(Value, Value), String> {
    price_with_cache(req, n, ms, seed, None).map(|(price, work, _)| (price, work))
}

fn price_with_cache(req: &Request, n: usize, ms: u64, seed: u64, previous: Option<&mut Shared>)
    -> Result<(Value, Value, Shared), String>
{
    let started = Instant::now();
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
    let mut shared = Shared::new(
        solver::decl_of(req.decl as usize),
        req.bid as u8,
        vec![8, 2],
        0,
        7,
        Deadline::after(Duration::from_millis(ms)),
    );
    let carried = previous.map_or(0, |old| shared.take_policy_cache_from(old));
    let sh = Arc::new(shared);
    let solver = Solver::new(
        Arc::clone(&sh),
        Seat::from_index(leader as usize).unwrap(),
        hand,
        true,
        worlds,
        vec![],
        Field::SeatLevels([0; 4]),
    )
    .parallel();
    let v = solver.solve(&key).ok_or("deadline")?;
    solver.flush_nodes();
    let work = json!({"elapsed_us":started.elapsed().as_micros() as u64,
        "nodes":sh.nodes.load(std::sync::atomic::Ordering::Relaxed),
        "pi_calls":sh.pi_calls_by_level(),"inner_worlds":sh.inner_worlds_by_level(),
        "policy_cache_entries":sh.pi_cache_len(),"search_cache_entries":solver.memo_len(),
        "carried_policy_entries":carried});
    drop(solver);
    let shared = Arc::try_unwrap(sh).unwrap_or_else(|_| panic!("completed auction still owns a solver"));
    Ok((json!([
        req.decl,
        v.numer().to_string(),
        v.denom().to_string()
    ]), work, shared))
}

/// Native offline price producer. The same evaluator as the live auction,
/// with explicit small/deep sample sizes and work counters. Never a new policy.
#[cfg(not(target_arch = "wasm32"))]
pub fn kiln(text: &str) -> Value {
    KilnPricer::default().price(text)
}

/// One bounded carry slot per native worker. Different policy contexts start
/// cold; process restart merely loses this optional acceleration.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub struct KilnPricer {
    previous: Option<Shared>,
}

#[cfg(not(target_arch = "wasm32"))]
impl KilnPricer {
    pub fn price(&mut self, text: &str) -> Value {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Job { auction: Auction, decl: u64, worlds: usize, budget_ms: u64 }
        let mut run = || -> Result<Value, String> {
            let job: Job = serde_json::from_str(text).map_err(|e| e.to_string())?;
            if !DECLS.contains(&job.decl) || ![4,8,12,40,160].contains(&job.worlds)
                || !(5..=600_000).contains(&job.budget_ms) { return Err("invalid kiln job".into()); }
            let (mut req, seed) = request(job.auction)?;
            req.decl = job.decl;
            // Bound retention by entry count. A single active solve retains its
            // ordinary working set; no collection of previous requests accumulates.
            if self.previous.as_ref().is_some_and(|s| s.pi_cache_len() > 100_000) {
                self.previous = None;
            }
            let (price, work, shared) = price_with_cache(&req, job.worlds, job.budget_ms, seed, self.previous.as_mut())?;
            self.previous = Some(shared);
            Ok(json!({"schema":"kiln-price-v1","auction":identity(&req,seed),
                "worlds":job.worlds,"inner_worlds":8,"price":price,"work":work}))
        };
        run().unwrap_or_else(|error| json!({"error":error}))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod carry_tests {
    use super::*;
    use std::sync::atomic::Ordering;

    fn job(worlds: usize) -> Value {
        json!({"auction":{"hand":[1,6,8,19,20,23,27],"seat":0,"bid":30,"seed":420914},
            "decl":5,"worlds":worlds,"budget_ms":60000})
    }

    #[test]
    fn retained_answers_match_cold_prices_across_refinement_and_changed_requests() {
        let mut pricer = KilnPricer::default();
        let mut carried = 0;
        let mut jobs = vec![job(8), job(40), job(160), job(8)];
        let mut changed = job(8); changed["decl"] = json!(6); jobs.push(changed.clone());
        changed["auction"]["bid"] = json!(36); jobs.push(changed.clone());
        changed["auction"]["seed"] = json!(0); jobs.push(changed.clone());
        changed["auction"]["seat"] = json!(2); jobs.push(changed.clone());
        changed["auction"]["hand"] = json!([0,2,5,7,14,16,26]); jobs.push(changed);
        for (index, job) in jobs.iter().enumerate() {
            let cold = kiln(&job.to_string());
            let warm = pricer.price(&job.to_string());
            assert!(cold.get("error").is_none(), "{cold}");
            assert!(warm.get("error").is_none(), "{warm}");
            for key in ["auction", "worlds", "inner_worlds", "price"] {
                assert_eq!(warm[key], cold[key], "request {index}, {key}");
            }
            carried += warm["work"]["carried_policy_entries"].as_u64().unwrap();
            if [0, 4, 5].contains(&index) {
                assert_eq!(warm["work"]["carried_policy_entries"], 0);
            }
        }
        assert!(carried > 0, "test must actually carry completed answers");
        assert!(pricer.price("{}").get("error").is_some());
        let valid = job(8);
        assert_eq!(pricer.price(&valid.to_string())["price"], kiln(&valid.to_string())["price"]);
    }

    #[test]
    fn every_policy_context_change_refuses_reuse_and_budget_state_is_fresh() {
        let (mut req, seed) = request(serde_json::from_value(job(8)["auction"].clone()).unwrap()).unwrap();
        req.decl = 5;
        let (_, _, mut old) = price_with_cache(&req, 8, 60000, seed, None).unwrap();
        let entries = old.pi_cache_len();
        assert!(entries > 0);
        let fresh = || Shared::new(solver::decl_of(5),30,vec![8,2],0,7,Deadline::after(Duration::from_secs(60)));
        for dimension in 0..7 {
            let mut next = fresh();
            match dimension {
                0 => next.dcl = solver::decl_of(6),
                1 => next.bid = 31,
                2 => next.n_inner = vec![4,2],
                3 => next.boundary_played = 15,
                4 => next.boundary_hand_size = 6,
                5 => next = next.with_inner_belief(solver::inner_belief::InnerBelief::VoidsCounted),
                6 => next = next.with_modeled_selection(solver::selection::Rule::RaceRefine),
                _ => unreachable!(),
            }
            assert_eq!(next.take_policy_cache_from(&mut old), 0, "dimension {dimension}");
            assert_eq!(next.pi_cache_len(), 0);
            assert_eq!(old.pi_cache_len(), entries);
        }
        old.dead.store(true, Ordering::Relaxed);
        old.deadline = Deadline::after(Duration::ZERO);
        let mut next = fresh();
        assert_eq!(next.take_policy_cache_from(&mut old), entries);
        assert_eq!(old.pi_cache_len(), 0, "ownership moves; storage is not duplicated");
        assert_eq!(next.nodes.load(Ordering::Relaxed), 0);
        assert_eq!(next.pi_calls.load(Ordering::Relaxed), 0);
        assert!(!next.dead.load(Ordering::Relaxed));
        assert!(!next.deadline.passed());
    }
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
    if !(100..=20_000).contains(&call.budget_ms) || !ROUNDS.contains(&call.worlds) {
        return Err("invalid auction budget".into());
    }
    let (mut req, seed) = request(call.auction)?;
    let mut value = empty(&req, seed);
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    for n in ROUNDS.into_iter().filter(|n| *n <= call.worlds) {
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
        complete(&mut value, &req, seed, n, prices)?;
        emit(&mut value, start, call.budget_ms, &mut checkpoint);
    }
    emit(&mut value, start, call.budget_ms, &mut checkpoint);
    Ok(value)
}

fn complete(
    value: &mut Value,
    req: &Request,
    seed: u64,
    n: usize,
    mut prices: Vec<Value>,
) -> Result<(), String> {
    prices.sort_by_key(|row| row[0].as_u64());
    if prices.iter().map(|row| row[0].as_u64()).collect::<Vec<_>>() != DECLS.map(Some) {
        return Err("incomplete or duplicated declarations".into());
    }
    for row in &prices {
        fraction(row)?;
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
    Ok(())
}
