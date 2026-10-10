//! Rust ladder tools on the forked walt: log level-0 calls from real games,
//! label them at high resolution, benchmark, and play paired head-to-head.
mod belief; // belief head + belief-weighted worlds (flagged variant, `l2b:` arm only)
mod net;
mod resnet; // STU residual student body
mod stu_tokens; // STU tile-token student (Sonnet designer)
mod turbo;
use net::Net;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde_json::{json, Value};
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};
use walt::rules::Seat;
use walt::solver::{self, net_hook, Contract, Deadline, Field, InnerBelief, Key, MoveOrdering, Shared, Solver, SplitMix64, INNER_SEED};

const HI: [u8; 28] = [0,1,1,2,2,2,3,3,3,3,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6];
const LO: [u8; 28] = [0,0,1,0,1,2,0,1,2,3,0,1,2,3,4,0,1,2,3,4,5,0,1,2,3,4,5,6];

fn deal(seed: u64) -> ([Vec<u64>; 4], usize, usize) {
    let mut r = SplitMix64(seed ^ 0x9E37_79B9_7F4A_7C15);
    let mut t: Vec<u64> = (0..28).collect();
    for i in (1..28).rev() { let j = r.below(i as u64 + 1) as usize; t.swap(i, j); }
    let hands: [Vec<u64>; 4] = core::array::from_fn(|q| { let mut h = t[7 * q..7 * q + 7].to_vec(); h.sort(); h });
    // Contract as in the recovered walt.c: seat and pip with the most trumps, then the double, then doubles.
    let mut best = (-1i64, 0usize, 0usize);
    for s in 0..4 { for p in 0..7u8 {
        let (mut cnt, mut dbl, mut hasd) = (0i64, 0i64, 0i64);
        for &x in &hands[s] { let x = x as usize; if HI[x] == p || LO[x] == p { cnt += 1; } if HI[x] == LO[x] { dbl += 1; if HI[x] == p { hasd = 1; } } }
        let sc = cnt * 100 + hasd * 10 + dbl;
        if sc > best.0 || (sc == best.0 && (s > best.1 || (s == best.1 && p as usize > best.2))) { best = (sc, s, p as usize); }
    } }
    (hands, best.1, best.2)
}

fn wire(decl: usize, bidder: usize, seat: usize, hand: &[u64], plays: &[u64], seed: u64, n: usize, n0: usize, budget: u64, voids: bool) -> String {
    let w = |xs: &[u64]| xs.iter().map(u64::to_string).collect::<Vec<_>>().join(" ");
    format!("baseline\ndecl {decl}\nbid 30\nbidder {bidder}\nseat {seat}\nhand {}\nplays {}\nseed {seed}\nn {n}\nn0 {n0}\nn1 2\nbudget_ms {budget}\ninner_belief {}\nselection 0\nmodeled_selection 0\n", w(hand), w(plays), if voids { 1 } else { 0 })
}

/// Shared on-disk memo of pinned-production decisions in h2h (`--prod-cache DIR`). Production's choice is a
/// deterministic function of the wire text, which is a function of (seed, decl, bidder, actor, actor's hand, plays)
/// with the fixed protocol constants (n 40, n0 8, budget 14 s, voids off); the memo is keyed by exactly that tuple,
/// so a hit replays the identical decision. Each process appends its new decisions to its own file; every process
/// loads every file in the directory at start, so arms and rounds share work (openings dominate production's cost).
struct ProdCache { map: std::collections::HashMap<(u64, u8, u8, u8, u32, Vec<u8>), (u8, u32)>, out: std::io::BufWriter<std::fs::File>, hits: u64, misses: u64 }
const PROD_CACHE_HEADER: &[u8; 32] = b"LADDER-PC1 n40 n0=8 b14000 v0  \n";
static PROD_CACHE: std::sync::Mutex<Option<ProdCache>> = std::sync::Mutex::new(None);
impl ProdCache {
    fn open(dir: &str, label: &str) -> ProdCache {
        std::fs::create_dir_all(dir).unwrap(); let mut map = std::collections::HashMap::new();
        let mut names: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "pc")).collect(); names.sort();
        for path in names {
            let bytes = std::fs::read(&path).unwrap_or_default(); if bytes.len() < 32 || &bytes[..32] != PROD_CACHE_HEADER { eprintln!("prod-cache: skipping {} (header)", path.display()); continue; }
            for r in bytes[32..].chunks_exact(77) { // a truncated tail (process killed mid-write) is ignored by chunks_exact
                let n = r[0] as usize; let seed = u64::from_le_bytes(r[1..9].try_into().unwrap()); let hand = u32::from_le_bytes(r[12..16].try_into().unwrap());
                map.insert((seed, r[9], r[10], r[11], hand, r[16..16 + n].to_vec()), (r[72], u32::from_le_bytes(r[73..77].try_into().unwrap())));
            }
        }
        let path = format!("{dir}/{label}-{}.pc", std::process::id()); let mut f = std::fs::File::create(&path).unwrap(); f.write_all(PROD_CACHE_HEADER).unwrap();
        ProdCache { map, out: std::io::BufWriter::new(f), hits: 0, misses: 0 }
    }
    fn put(&mut self, k: (u64, u8, u8, u8, u32, Vec<u8>), v: (u8, u32)) {
        let mut r = [0u8; 77]; r[0] = k.5.len() as u8; r[1..9].copy_from_slice(&k.0.to_le_bytes()); r[9] = k.1; r[10] = k.2; r[11] = k.3; r[12..16].copy_from_slice(&k.4.to_le_bytes()); r[16..16 + k.5.len()].copy_from_slice(&k.5); r[72] = v.0; r[73..77].copy_from_slice(&v.1.to_le_bytes());
        self.out.write_all(&r).unwrap(); self.map.insert(k, v);
    }
}

/// Two-ring arms with the net inside the inner ring. `l2z:OUTER:INNER:PATH`: the net replaces the dice (level-0 moves)
/// inside the full-depth inner searches. `l2w:OUTER:INNER:TRICKS:PATH` (VARIANT, different semantics): the inner
/// searches additionally stop after TRICKS completed tricks and value each world by the net's declarer-make estimate.
fn l2_arm(decl: usize, netp: &str) -> turbo::L2Player {
    if let Some(rest) = netp.strip_prefix("l2z:") {
        let f: Vec<&str> = rest.splitn(3, ':').collect(); let p = turbo::L2Player::parse(decl, &format!("l2:{}:{}:1", f[0], f[1]));
        let net: &'static Net = Box::leak(Box::new(Net::load(f[2], decl as u8).unwrap())); p.engine.lock().unwrap().set_leaf_net(net); p
    } else if let Some(rest) = netp.strip_prefix("l2w:") {
        let f: Vec<&str> = rest.splitn(4, ':').collect(); let p = turbo::L2Player::parse(decl, &format!("l2:{}:{}:1", f[0], f[1]));
        let net: &'static Net = Box::leak(Box::new(Net::load(f[3], decl as u8).unwrap())); let e = p.engine.lock().unwrap(); e.set_leaf_net(net); e.set_leaf_value_net(net, f[2].parse().unwrap()); drop(e); p
    } else { panic!("not a two-ring arm: {netp}") }
}
/// VARIANT (belief-weighted worlds): `l2b:OUTER:BELIEF:POLICY[:MULT[:ALPHA[:IPF]]]` = the `l2n:OUTER:POLICY` arm (one-ring
/// level-2 search, modeled others play POLICY) whose top-level sampled worlds are reweighted by the belief head
/// BELIEF (belief.rs): MULT = 1 (default) weights the OUTER uniform worlds (importance sampling, weighted make sums);
/// MULT > 1 draws MULT x OUTER uniform candidates and systematically resamples OUTER of them by weight. ALPHA (default
/// 1) tempers the log weights. IPF (default 0) > 0 replaces the naive per-tile ratio P_belief/P_uniform by the tilt fitted
/// with IPF sweeps so the tilted support's marginals equal the belief (belief::ipf_log_tilt). With no `l2b:` arm nothing in the engine changes (the hook pointer stays null).
fn l2b_arm(decl: usize, netp: &str) -> turbo::L2Player {
    let f: Vec<&str> = netp.strip_prefix("l2b:").unwrap().split(':').collect();
    let outer: usize = f[0].parse().unwrap(); let mult: usize = f.get(3).map_or(1, |x| x.parse().unwrap()); let alpha: f64 = f.get(4).map_or(1.0, |x| x.parse().unwrap()); let ipf: usize = f.get(5).map_or(0, |x| x.parse().unwrap());
    let p = turbo::L2Player::parse(decl, &format!("l2:{}:1", outer * mult));
    let net: &'static Net = Box::leak(Box::new(Net::load(f[2], decl as u8).unwrap()));
    let wb: &'static turbo::WorldBelief = Box::leak(Box::new(turbo::WorldBelief { net: belief::BeliefNet::load(f[1], decl as u8).unwrap(), alpha, ipf }));
    { let e = p.engine.lock().unwrap(); e.set_net_policy(1, net); e.set_world_belief(wb, if mult > 1 { outer } else { 0 }); }
    p
}
/// Play one full hand. `hybrid(seat)` says whether that seat's decisions use the installed net.
/// Whole-player arms: the hybrid partnership's decision comes from something other than Walt's outer search.
enum Whole<'a> { None, L2(&'a turbo::L2Player), Net(&'a Net), Asc, NetCheck(&'a Net, &'a turbo::L2Player), NetForced(&'a Net, u8) }
static CHECK_AGREE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0); static CHECK_N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn play(hands: &[Vec<u64>; 4], bidder: usize, decl: usize, seed: u64, net: Option<&Arc<dyn net_hook::NetPolicy>>, hybrid: &dyn Fn(usize) -> bool, log: bool, budget_ms: u64, voids: bool, whole: Whole, on_decision: impl FnMut(&Value, &[u64], usize, u128)) -> (Vec<u64>, [u64; 2], bool) {
    play_bid(hands, bidder, decl, 30, seed, net, hybrid, log, budget_ms, voids, whole, on_decision)
}
/// `play` at contract `bid` (30..=42; the production path is pinned to 30). `bid` > 42 plays all 28 tiles (full points, for
/// rollout histograms); `made` is then false.
fn play_bid(hands: &[Vec<u64>; 4], bidder: usize, decl: usize, bid: u8, seed: u64, net: Option<&Arc<dyn net_hook::NetPolicy>>, hybrid: &dyn Fn(usize) -> bool, log: bool, budget_ms: u64, voids: bool, whole: Whole, mut on_decision: impl FnMut(&Value, &[u64], usize, u128)) -> (Vec<u64>, [u64; 2], bool) {
    let dcl = solver::decl_of(decl);
    let mut pairs: Vec<(usize, usize)> = Vec::new(); let mut plays: Vec<u64> = Vec::new();
    for _ in 0..28 {
        let st = solver::replay(dcl, bidder, &pairs);
        let key = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: None, alive: 0 };
        if bid <= 42 && (Contract::Straight { bid }).terminal(&key).is_some() { break; }
        let leader_arena = (st.leader as usize + 4 - st.r) % 4; let actor = (leader_arena + st.plays.len()) % 4;
        let own: Vec<u64> = hands[actor].iter().copied().filter(|&t| st.played >> t & 1 == 0).collect();
        let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
        let legal = walt::rules::legal_plays(dcl, solver::set_of(own.iter().fold(0u32, |m, &t| m | 1 << t)), led);
        let legal_mask = solver::mask_of(legal);
        let tile = if legal_mask.count_ones() == 1 { legal_mask.trailing_zeros() as u64 } else if hybrid(actor) && !matches!(whole, Whole::None) {
            // Whole-player arms decide from the same public state (key frame, voids from replay).
            let own_mask = own.iter().fold(0u32, |m, &t| m | 1 << t); let t0 = Instant::now();
            let t = match whole {
                Whole::L2(p2) => p2.choose(&turbo::pack(st.played, st.leader, &st.plays, st.banked_t1, st.banked_t0, Some(st.voids)), own_mask, legal_mask),
                Whole::Net(n) => { let k = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: Some(st.voids), alive: 0 }; net_hook::NetPolicy::choose(n, &k, Seat::from_index((actor + st.r) % 4).unwrap(), own_mask, legal_mask) }
                Whole::Asc => legal_mask.trailing_zeros() as u8,
                Whole::NetForced(n, first) => if plays.is_empty() { assert!(legal_mask >> first & 1 == 1); first } else {
                    // Rollout labels (rollrelabel): the first play is the tile being labelled, every later decision is the net's.
                    let k = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: Some(st.voids), alive: 0 };
                    net_hook::NetPolicy::choose(n, &k, Seat::from_index((actor + st.r) % 4).unwrap(), own_mask, legal_mask) },
                Whole::NetCheck(n, p2) => {
                    // Plays the net's choice; also asks the teacher on the identical state and counts agreement (arm self-check).
                    let k = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: Some(st.voids), alive: 0 };
                    let tn = net_hook::NetPolicy::choose(n, &k, Seat::from_index((actor + st.r) % 4).unwrap(), own_mask, legal_mask);
                    let tt = p2.choose(&turbo::pack(st.played, st.leader, &st.plays, st.banked_t1, st.banked_t0, Some(st.voids)), own_mask, legal_mask);
                    CHECK_N.fetch_add(1, std::sync::atomic::Ordering::Relaxed); if tn == tt { CHECK_AGREE.fetch_add(1, std::sync::atomic::Ordering::Relaxed); }
                    tn
                }
                Whole::None => unreachable!(),
            };
            let us = t0.elapsed().as_micros(); on_decision(&json!({"choice": t}), &plays, actor, us);
            t as u64
        } else {
            // Production decisions (not hybrid) may come from the shared memo; the key is the full wire input.
            let ck = if !hybrid(actor) && PROD_CACHE.lock().unwrap().is_some() { Some((seed, decl as u8, bidder as u8, actor as u8, hands[actor].iter().fold(0u32, |m, &t| m | 1 << t), plays.iter().map(|&x| x as u8).collect::<Vec<u8>>())) } else { None };
            let cached = ck.as_ref().and_then(|k| { let mut g = PROD_CACHE.lock().unwrap(); let c = g.as_mut().unwrap(); match c.map.get(k) { Some(&v) => { c.hits += 1; Some(v) } None => { c.misses += 1; None } } });
            if let Some((choice, us)) = cached {
                // `us` is the time production took when this decision was first computed (so timing summaries stay comparable).
                on_decision(&json!({"choice": choice, "cached": true}), &plays, actor, us as u128);
                choice as u64
            } else {
            assert!(bid == 30 || hybrid(actor), "production decisions are pinned to bid 30");
            net_hook::install_net(if hybrid(actor) { net.cloned() } else { None });
            if log { net_hook::start_log(); }
            let t0 = Instant::now();
            let text = solver::partnership_wire::run(&wire(decl, bidder, actor, &hands[actor], &plays, seed, 40, 8, if hybrid(actor) { budget_ms } else { 14_000 }, voids && hybrid(actor))).expect("baseline");
            let v: Value = serde_json::from_str(&text).unwrap(); let us = t0.elapsed().as_micros();
            net_hook::install_net(None);
            if let Some(k) = ck { PROD_CACHE.lock().unwrap().as_mut().unwrap().put(k, (v["choice"].as_u64().unwrap() as u8, us.min(u32::MAX as u128) as u32)); }
            on_decision(&v, &plays, actor, us);
            v["choice"].as_u64().unwrap()
            }
        };
        assert!(legal_mask >> tile & 1 == 1, "illegal choice");
        pairs.push((actor, tile as usize)); plays.extend([actor as u64, tile]);
    }
    let st = solver::replay(dcl, bidder, &pairs);
    let pts = [st.banked_t1 as u64, st.banked_t0 as u64]; // [bidding team, defenders]
    (plays, pts, bid <= 42 && st.banked_t1 >= bid)
}

fn label_call(decl: usize, c: &net_hook::Call, worlds: usize, ref_seed: Option<u64>) -> (Vec<(u8, u16)>, u8) {
    let dcl = solver::decl_of(decl); let bid = 30u8;
    let table: u32 = c.plays.iter().fold(0, |m, &t| m | 1 << t);
    let boundary_played = c.played & !table; let completed = (boundary_played.count_ones() / 4) as usize; let boundary_size = 7 - completed;
    let belief = if c.voids.is_some() { InnerBelief::VoidsCounted } else { InnerBelief::Voidless };
    let key = Key { played: c.played, leader: c.leader, plays: c.plays.clone(), banked_t1: c.banked_t1, banked_t0: c.banked_t0, voids: c.voids, alive: 0 };
    let contract = Contract::Straight { bid }; let sizes = contract.sizes(&key, boundary_played, boundary_size);
    let seat = Seat::from_index(c.seat as usize).unwrap(); let maximize = c.seat % 2 == 1;
    let shared = Arc::new(Shared::new(dcl, bid, vec![worlds, 2], boundary_played, boundary_size, Deadline::after(Duration::from_secs(60))).with_inner_belief(belief));
    let extra = ref_seed.map_or(0, |s| solver::mix(0x5245_4653 ^ s));
    let mut rng = SplitMix64(INNER_SEED ^ extra ^ solver::mix(c.seat as u64) ^ solver::mix(u64::from(c.hand)) ^ solver::record_hash(&key));
    let ws = belief.sample(dcl, seat, c.hand, &key, sizes, worlds, &mut rng, shared.deadline).expect("sample");
    let seeds: Vec<u64> = (0..worlds).map(|_| rng.next_u64()).collect();
    let dice = Solver::new(Arc::clone(&shared), seat, c.hand, maximize, ws, seeds, Field::Dice).with_ordering(MoveOrdering::CaptureFirst);
    let legal = solver::mask_bits(c.legal);
    let values = dice.action_values(&key, &legal).expect("values"); dice.flush_nodes();
    let choice = solver::best_of(&values, maximize);
    let counts = values.iter().map(|(t, v)| { let n = v * BigRational::from_integer(BigInt::from(worlds as u64)); assert!(n.is_integer()); (*t, n.to_integer().to_u16().unwrap()) }).collect();
    (counts, choice)
}

fn production_choice(decl: usize, c: &net_hook::Call) -> u8 {
    let dcl = solver::decl_of(decl);
    let table: u32 = c.plays.iter().fold(0, |m, &t| m | 1 << t);
    let boundary_played = c.played & !table; let boundary_size = 7 - (boundary_played.count_ones() / 4) as usize;
    let key = Key { played: c.played, leader: c.leader, plays: c.plays.clone(), banked_t1: c.banked_t1, banked_t0: c.banked_t0, voids: c.voids, alive: 0 };
    let shared = Arc::new(Shared::new(dcl, 30, vec![8, 2], boundary_played, boundary_size, Deadline::after(Duration::from_secs(60))).with_inner_belief(if c.voids.is_some() { InnerBelief::VoidsCounted } else { InnerBelief::Voidless }));
    let seat = Seat::from_index(c.seat as usize).unwrap();
    let host = Solver::new(Arc::clone(&shared), seat, c.hand, c.seat % 2 == 1, Vec::new(), Vec::new(), Field::Level(0));
    let t = host.modeled_choice(0, &key, seat, c.hand, c.legal).expect("production choice"); host.flush_nodes(); t
}

/// Control: the exact level-0 mind at a higher world count (slow; the fidelity ceiling).
struct HiRes { decl: usize, worlds: usize }
impl net_hook::NetPolicy for HiRes {
    fn choose(&self, key: &Key, seat: Seat, hand: u32, legal: u32) -> u8 {
        let c = net_hook::Call { seat: seat.index() as u8, hand, played: key.played, leader: key.leader, plays: key.plays.clone(), banked_t1: key.banked_t1, banked_t0: key.banked_t0, legal, voids: key.voids };
        label_call(self.decl, &c, self.worlds, None).1
    }
}

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned()).unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect(); let cmd = args.get(1).map(String::as_str).unwrap_or("");
    match cmd {
        // log: self-play native games, record every unique level-0 call per decision.
        "log" => {
            let seed0: u64 = arg(&args, "--seed", "1").parse().unwrap(); let games: u64 = arg(&args, "--games", "4").parse().unwrap(); let out = arg(&args, "--out", "calls.jsonl"); let voids = args.iter().any(|a| a == "--voids");
            let mut f = std::fs::File::create(&out).unwrap(); let start = Instant::now(); let mut total_calls = 0u64; let mut unique = 0u64; let mut decisions = 0u64;
            for g in 0..games {
                let (hands, bidder, decl) = deal(seed0 + g); let seed = seed0 * 1_000_003 + g;
                let mut recs: Vec<Value> = Vec::new();
                let (_plays, pts, made) = play(&hands, bidder, decl, seed, None, &|_| voids, true, 14_000, voids, Whole::None, |v, plays, actor, us| {
                    let (calls, total) = net_hook::take_log(); total_calls += total; unique += calls.len() as u64; decisions += 1;
                    recs.push(json!({"game": g, "deal_seed": seed0 + g, "decl": decl, "bidder": bidder, "actor": actor, "plays": plays, "choice": v["choice"], "decision_us": us as u64, "calls_total": total,
                        "voids_mode": voids, "calls": calls.iter().map(|c| json!([c.seat, c.hand, c.played, c.leader, c.plays, c.banked_t1, c.banked_t0, c.legal, c.voids])).collect::<Vec<_>>()}));
                });
                for r in recs { writeln!(f, "{}", r).unwrap(); }
                writeln!(f, "{}", json!({"game": g, "result": {"points": pts, "made": made, "decl": decl, "bidder": bidder}})).unwrap();
            }
            println!("{}", json!({"games": games, "decisions": decisions, "level0_calls": total_calls, "unique_calls": unique, "seconds": start.elapsed().as_secs_f64()}));
        }
        // label: read calls.jsonl, write fixed 84-byte records with k/N at --worlds plus the 8-world production choice.
        "label" => {
            let inp = arg(&args, "--in", "calls.jsonl"); let out = arg(&args, "--out", "labels.bin"); let worlds: usize = arg(&args, "--worlds", "160").parse().unwrap();
            let limit: usize = arg(&args, "--limit", "0").parse().unwrap(); let skip: usize = arg(&args, "--skip", "0").parse().unwrap(); let cap: usize = arg(&args, "--cap", "0").parse().unwrap();
            let engine = arg(&args, "--engine", "walt"); let delta = turbo::parse_delta(&arg(&args, "--delta", "1")); let mut pool = turbo::Pool::new(vec![worlds], delta, turbo::FIELD_SEED, 4_000_000); let mut fails = 0usize;
            // Record scale: walt labels store k of N worlds; turbo labels store round(value / N * SCALE) of SCALE so fractional (delta < 1) values survive.
            const SCALE: u16 = 10_000;
            let text = std::fs::read_to_string(&inp).unwrap(); let mut f = std::fs::File::create(&out).unwrap(); let start = Instant::now();
            let mut seen = std::collections::HashSet::new(); let mut n = 0usize; let mut idx = 0usize; let mut prod_agree = 0usize; let mut label_us = 0u128;
            'outer: for line in text.lines() {
                let v: Value = serde_json::from_str(line).unwrap(); if v.get("calls").is_none() { continue; }
                let decl = v["decl"].as_u64().unwrap() as usize;
                let all = v["calls"].as_array().unwrap(); let mut order: Vec<usize> = (0..all.len()).collect();
                if cap > 0 && all.len() > cap { let mut r = SplitMix64(0xC0FFEE ^ (idx as u64 * 7919) ^ v["game"].as_u64().unwrap()); for i in (1..order.len()).rev() { let j = r.below(i as u64 + 1) as usize; order.swap(i, j); } order.truncate(cap); }
                for &ci in &order { let c = &all[ci];
                    let a = c.as_array().unwrap();
                    let call = net_hook::Call { seat: a[0].as_u64().unwrap() as u8, hand: a[1].as_u64().unwrap() as u32, played: a[2].as_u64().unwrap() as u32, leader: a[3].as_u64().unwrap() as u8, plays: a[4].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u8).collect(), banked_t1: a[5].as_u64().unwrap() as u8, banked_t0: a[6].as_u64().unwrap() as u8, legal: a[7].as_u64().unwrap() as u32, voids: a.get(8).and_then(|v| v.as_array()).map(|v| [v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32, v[2].as_u64().unwrap() as u32, v[3].as_u64().unwrap() as u32]) };
                    if !seen.insert((decl, call.clone())) { continue; }
                    if idx < skip { idx += 1; continue; } idx += 1;
                    let t0 = Instant::now();
                    let (counts, choice, nrec): (Vec<(u8, u16)>, u8, u16) = if engine == "turbo" {
                        let p = turbo::pack(call.played, call.leader, &call.plays, call.banked_t1, call.banked_t0, call.voids);
                        match pool.get(decl).action(1, &p, call.hand, true) {
                            Ok((t, v)) => ((0..28u8).filter(|&t| call.legal >> t & 1 == 1).map(|t| (t, (v[t as usize] / worlds as f64 * SCALE as f64).round() as u16)).collect(), t, SCALE),
                            Err(_) => { fails += 1; continue; }
                        }
                    } else { let (c, ch) = label_call(decl, &call, worlds, None); (c, ch, worlds as u16) };
                    label_us += t0.elapsed().as_micros();
                    let prod = production_choice(decl, &call); if prod == choice { prod_agree += 1; }
                    let mut rec = [0u8; 100];
                    rec[0] = decl as u8; rec[1] = call.seat; rec[2] = (call.seat % 2 == 1) as u8; rec[3] = call.leader; rec[4] = call.plays.len() as u8;
                    for p in 0..4 { rec[5 + p] = *call.plays.get(p).unwrap_or(&255); }
                    rec[9] = call.banked_t1; rec[10] = call.banked_t0;
                    rec[11..15].copy_from_slice(&call.hand.to_le_bytes()); rec[15..19].copy_from_slice(&call.played.to_le_bytes()); rec[19..23].copy_from_slice(&call.legal.to_le_bytes());
                    rec[23..25].copy_from_slice(&nrec.to_le_bytes());
                    for (t, k) in &counts { rec[25 + 2 * *t as usize..27 + 2 * *t as usize].copy_from_slice(&k.to_le_bytes()); }
                    rec[81] = choice; rec[82] = prod; rec[83] = call.voids.is_some() as u8; if let Some(v) = call.voids { for q in 0..4 { rec[84 + 4 * q..88 + 4 * q].copy_from_slice(&v[q].to_le_bytes()); } } f.write_all(&rec).unwrap(); n += 1;
                    if limit > 0 && n >= limit { break 'outer; }
                }
            }
            println!("{}", json!({"records": n, "worlds": worlds, "engine": engine, "delta": delta, "failures": fails, "label_us_mean": if n > 0 { label_us as f64 / n as f64 } else { 0.0 }, "prod_agree_with_hires_choice": prod_agree, "seconds": start.elapsed().as_secs_f64()}));
        }
        // bench: outer baseline decision wall time native vs hybrid on the same roots.
        "bench" => {
            let seed0: u64 = arg(&args, "--seed", "1").parse().unwrap(); let games: u64 = arg(&args, "--games", "2").parse().unwrap();
            let netp = arg(&args, "--net", ""); let mut rows = Vec::new();
            for g in 0..games {
                let (hands, bidder, decl) = deal(seed0 + g); let seed = seed0 * 1_000_003 + g;
                let net = if netp.is_empty() { None } else { Some(Arc::new(Net::load(&netp, decl as u8).unwrap())) };
                let mut nat: Vec<u128> = Vec::new(); let mut hyb: Vec<u128> = Vec::new(); let mut agree = 0usize; let mut n = 0usize;
                let mut hyb_choices: Vec<u64> = Vec::new();
                let (plays, _, _) = play(&hands, bidder, decl, seed, None, &|_| false, false, 14_000, false, Whole::None, |v, _, _, us| { nat.push(us); hyb_choices.push(v["choice"].as_u64().unwrap()); });
                if let Some(nt) = &net {
                    // Replay the native game's public record; at each native decision point, ask the hybrid too.
                    let dcl = solver::decl_of(decl); let mut pairs: Vec<(usize, usize)> = Vec::new(); let mut pl: Vec<u64> = Vec::new(); let mut k = 0usize;
                    for i in (0..plays.len()).step_by(2) {
                        let actor = plays[i] as usize; let st = solver::replay(dcl, bidder, &pairs);
                        let own: Vec<u64> = hands[actor].iter().copied().filter(|&t| st.played >> t & 1 == 0).collect();
                        let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
                        let lm = solver::mask_of(walt::rules::legal_plays(dcl, solver::set_of(own.iter().fold(0u32, |m, &t| m | 1 << t)), led));
                        if lm.count_ones() > 1 && (Contract::Straight { bid: 30 }).terminal(&Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: None, alive: 0 }).is_none() {
                            net_hook::install_net(Some(nt.clone() as Arc<dyn net_hook::NetPolicy>)); let t0 = Instant::now();
                            let v: Value = serde_json::from_str(&solver::partnership_wire::run(&wire(decl, bidder, actor, &hands[actor], &pl, seed, 40, 8, 14_000, false)).unwrap()).unwrap();
                            hyb.push(t0.elapsed().as_micros()); net_hook::install_net(None); let calls = net_hook::net_calls();
                            if v["choice"].as_u64().unwrap() == hyb_choices[k] { agree += 1; } n += 1; let _ = calls;
                            k += 1;
                        }
                        pairs.push((actor, plays[i + 1] as usize)); pl.extend([plays[i], plays[i + 1]]);
                    }
                }
                rows.push(json!({"game": g, "native_decision_us": nat, "hybrid_decision_us": hyb, "outer_choice_agree": agree, "decisions": n}));
            }
            println!("{}", json!({"rows": rows}));
        }
        // h2h: paired mirrored games, hybrid partnership vs native partnership, this worker's slice of deals.
        "h2h" => {
            let seed0: u64 = arg(&args, "--seed", "1").parse().unwrap(); let deals: u64 = arg(&args, "--deals", "4").parse().unwrap();
            let worker: u64 = arg(&args, "--worker", "0").parse().unwrap(); let workers: u64 = arg(&args, "--workers", "1").parse().unwrap();
            let netp = arg(&args, "--net", ""); let out = arg(&args, "--out", "h2h.jsonl"); let voids = args.iter().any(|a| a == "--voids") || netp == "voids" || netp.starts_with("turbo") || netp.starts_with("l2:") || (netp.starts_with("l2n:") || netp.starts_with("l2s:")) || netp.starts_with("l2z:") || netp.starts_with("l2w:") || netp.starts_with("l2b:") || netp.starts_with("np"); let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap();
            // --prod-cache DIR (default off): shared memo of production's decisions, see ProdCache.
            let mixed = args.iter().any(|a| a == "--mixed");
            let pcdir = arg(&args, "--prod-cache", ""); if !pcdir.is_empty() { let label: String = format!("{}-w{worker}", netp.replace(['/', ':'], "_")); *PROD_CACHE.lock().unwrap() = Some(ProdCache::open(&pcdir, &label)); }
            let start = Instant::now(); let mut leaf = [0u64; 4];
            // Resume: skip (deal, side) pairs already written by an earlier capped round.
            let done: std::collections::HashSet<(u64, u64)> = std::fs::read_to_string(&out).unwrap_or_default().lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()).map(|v| (v["deal"].as_u64().unwrap(), v["side"].as_u64().unwrap())).collect();
            for d in (0..deals).filter(|d| d % workers == worker) {
                let (hands, bidder, decl) = deal(seed0 + d); let seed = seed0 * 1_000_003 + d; let net: Option<Arc<dyn net_hook::NetPolicy>> = if netp == "none" || netp == "voids" || netp.starts_with("l2:") || (netp.starts_with("l2n:") || netp.starts_with("l2s:")) || netp.starts_with("l2z:") || netp.starts_with("l2w:") || netp.starts_with("l2b:") || netp.starts_with("np") { None } else { Some(if netp == "ascending" { Arc::new(net::Ascending) } else if netp.starts_with("hires") { Arc::new(HiRes { decl, worlds: netp[5..].parse().unwrap() }) } else if let Some(rest) = netp.strip_prefix("turboz:") { let (n, path) = rest.split_once(':').unwrap(); let mut t = turbo::TurboInner::new(decl, n.parse().unwrap(), 1.0, 0); t.leaf = Some(Box::leak(Box::new(Net::load(path, decl as u8).unwrap()))); Arc::new(t) } else if netp.starts_with("turbo") { Arc::new(turbo::TurboInner::parse(decl, &netp)) } else { Arc::new(Net::load(&netp, decl as u8).unwrap()) }) };
                let l2 = if netp.starts_with("l2:") { Some(turbo::L2Player::parse(decl, &netp)) } else if netp.starts_with("l2z:") || netp.starts_with("l2w:") { Some(l2_arm(decl, &netp)) } else if let Some(rest) = netp.strip_prefix("l2s:") { let (outer, rest) = rest.split_once(':').unwrap(); let (tau, path) = rest.split_once(':').unwrap(); let p = turbo::L2Player::parse(decl, &format!("l2:{outer}:1")); let net: &'static Net = Box::leak(Box::new(Net::load(path, decl as u8).unwrap())); let sn: &'static turbo::SampledNet = Box::leak(Box::new(turbo::SampledNet { net, tau: tau.parse().unwrap() })); p.engine.lock().unwrap().set_net_policy_sampled(1, sn); Some(p) // VARIANT: l2s:OUTER:TAU:NET, modeled others sample the pmake vector
                } else if let Some(rest) = netp.strip_prefix("l2n:") { let (outer, path) = rest.split_once(':').unwrap(); let p = turbo::L2Player::parse(decl, &format!("l2:{outer}:1")); let net: &'static Net = Box::leak(Box::new(Net::load(path, decl as u8).unwrap())); p.engine.lock().unwrap().set_net_policy(1, net); Some(p) } else if netp.starts_with("l2b:") { Some(l2b_arm(decl, &netp)) } else { None }; let npn = if let Some(path) = netp.strip_prefix("np:").or(netp.strip_prefix("npcheck:")) { if path == "asc" { None } else { Some(Net::load(path, decl as u8).unwrap()) } } else { None };
                let checker = if netp.starts_with("npcheck:") { Some(turbo::L2Player::parse(decl, "l2:160:8")) } else { None };
                for side in 0..2usize {
                    if done.contains(&(d, side as u64)) { continue; }
                    // --mixed (diagnostic, default off): only one seat of the hybrid partnership is the hybrid (seat `side` on even
                    // deals, `side + 2` on odd ones); its partner is production. Outcome is still the partnership's.
                    let hseat = (side + 2 * (d as usize % 2)) % 4; let hyb = move |s: usize| if mixed { s == hseat } else { s % 2 == side }; let mut us = Vec::new(); let mut nus = Vec::new(); let mut ncached = 0u64;
                    let t0 = Instant::now();
                    let budget = if netp.starts_with("hires") || netp.starts_with("turbo") { 20_000 } else { 14_000 }; let (plays, pts, made) = play(&hands, bidder, decl, seed, net.as_ref(), &hyb, false, budget, voids, match (&l2, &npn, &checker) { (Some(p), _, _) => Whole::L2(p), (_, Some(n), Some(c)) => Whole::NetCheck(n, c), (_, Some(n), None) => Whole::Net(n), _ => if netp == "np:asc" { Whole::Asc } else { Whole::None } }, |v, _, actor, u| if hyb(actor) { us.push(u as u64) } else { nus.push(u as u64); if v.get("cached").is_some() { ncached += 1; } });
                    let hybrid_declares = bidder % 2 == side; let hybrid_won = if hybrid_declares { made } else { !made };
                    let fallbacks = l2.as_ref().map_or(0, |p| p.fallbacks.swap(0, std::sync::atomic::Ordering::Relaxed));
                    writeln!(f, "{}", json!({"deal": d, "deal_seed": seed0 + d, "side": side, "decl": decl, "bidder": bidder, "hybrid_declares": hybrid_declares, "made": made, "points": pts, "hybrid_won": hybrid_won, "plays": plays, "hybrid_us": us, "native_us": nus, "game_seconds": t0.elapsed().as_secs_f64(), "fallbacks": fallbacks, "native_cached": ncached})).unwrap();
                    if let Some(c) = PROD_CACHE.lock().unwrap().as_mut() { c.out.flush().unwrap(); }
                }
                if let Some(p) = &l2 { let m = p.engine.lock().unwrap().metrics(); leaf[0] += m[6]; leaf[1] += m[7]; leaf[2] += m[8]; leaf[3] = leaf[3].max(m[9]); }
            }
            let (hits, misses) = PROD_CACHE.lock().unwrap().as_ref().map_or((0, 0), |c| (c.hits, c.misses));
            let ess: Vec<Value> = belief::ESS.lock().unwrap().iter().enumerate().filter(|(_, e)| e[0] > 0.0).map(|(t, e)| json!({"trick": t, "decisions": e[0], "mean_ess_frac": e[1] / e[0], "min_ess_frac": e[2]})).collect();
            if !ess.is_empty() { eprintln!("{}", json!({"belief_ess": ess})); }
            println!("{}", json!({"worker": worker, "seconds": start.elapsed().as_secs_f64(), "belief_ess": ess, "check_decisions": CHECK_N.load(std::sync::atomic::Ordering::Relaxed), "check_agree": CHECK_AGREE.load(std::sync::atomic::Ordering::Relaxed), "prod_cache_hits": hits, "prod_cache_misses": misses, "leaf_memo": {"calls": leaf[0], "hits": leaf[1], "clears": leaf[2], "max_entries": leaf[3]}}));
        }
        // turbo-eval: label quality per microsecond. For sampled logged calls (one depth bucket = one trick),
        // compare each estimator's pmake vector against a high-sample reference; the frozen selector decides ties.
        "turbo-eval" => {
            let inp = arg(&args, "--in", "data/callsv-val.jsonl"); let per: usize = arg(&args, "--per", "200").parse().unwrap(); let bucket: usize = arg(&args, "--bucket", "0").parse().unwrap();
            let out = arg(&args, "--out", "eval.jsonl"); let refn: usize = arg(&args, "--ref", "4096").parse().unwrap(); let seed: u64 = arg(&args, "--seed", "77").parse().unwrap();
            let arms: Vec<(String, usize, f64, u64)> = arg(&args, "--arms", "8:1,16:1,24:1,32:1,64:1,160:1,8:1/8,24:1/8,8:1/24,8:0,24:0").split(',').map(|a| { let (n, d) = a.split_once(':').unwrap(); if let Some(cap) = n.strip_prefix('e') { (a.to_string(), 8, turbo::parse_delta(d), cap.parse().unwrap()) } else { (a.to_string(), n.parse().unwrap(), turbo::parse_delta(d), 0) } }).collect();
            let walt_arms: Vec<usize> = arg(&args, "--walt", "8,160").split(',').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
            // Reservoir-sample `per` unique multi-legal calls whose depth/4 == bucket.
            let text = std::fs::read_to_string(&inp).unwrap(); let mut seen = std::collections::HashSet::new(); let mut sample: Vec<(usize, net_hook::Call)> = Vec::new(); let mut count = 0u64; let mut r = SplitMix64(seed);
            for line in text.lines() {
                let v: Value = serde_json::from_str(line).unwrap(); if v.get("calls").is_none() { continue; } let decl = v["decl"].as_u64().unwrap() as usize;
                for c in v["calls"].as_array().unwrap() { let a = c.as_array().unwrap();
                    let call = net_hook::Call { seat: a[0].as_u64().unwrap() as u8, hand: a[1].as_u64().unwrap() as u32, played: a[2].as_u64().unwrap() as u32, leader: a[3].as_u64().unwrap() as u8, plays: a[4].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u8).collect(), banked_t1: a[5].as_u64().unwrap() as u8, banked_t0: a[6].as_u64().unwrap() as u8, legal: a[7].as_u64().unwrap() as u32, voids: a.get(8).and_then(|v| v.as_array()).map(|v| [v[0].as_u64().unwrap() as u32, v[1].as_u64().unwrap() as u32, v[2].as_u64().unwrap() as u32, v[3].as_u64().unwrap() as u32]) };
                    if call.legal.count_ones() < 2 || (call.played.count_ones() / 4) as usize != bucket || !seen.insert((decl, call.clone())) { continue; }
                    count += 1; if sample.len() < per { sample.push((decl, call)); } else { let j = r.below(count) as usize; if j < per { sample[j] = (decl, call); } }
                }
            }
            let mut f = std::fs::File::create(&out).unwrap(); let start = Instant::now();
            let mut refpool = turbo::Pool::new(vec![refn], 1.0, 0xA5A5, 8_000_000); let mut pools: Vec<turbo::Pool> = arms.iter().map(|(_, n, d, ec)| { let mut p = turbo::Pool::new(vec![*n], *d, turbo::FIELD_SEED, 8_000_000); p.enum_cap = *ec; p }).collect();
            for (decl, call) in &sample {
                let p = turbo::pack(call.played, call.leader, &call.plays, call.banked_t1, call.banked_t0, call.voids);
                let t0 = Instant::now(); let (_rt, rv) = refpool.get(*decl).action(1, &p, call.hand, true).expect("reference"); let ref_us = t0.elapsed().as_micros();
                let legal: Vec<u8> = (0..28u8).filter(|&t| call.legal >> t & 1 == 1).collect();
                let refv: Vec<f64> = legal.iter().map(|&t| rv[t as usize] / refn as f64).collect();
                let support = turbo::support_size(&p, call.hand);
                let mut row = json!({"decl": decl, "seat": call.seat, "depth": call.played.count_ones(), "nlegal": legal.len(), "legal": legal, "support": support, "ref": refv, "ref_us": ref_us as u64, "arms": {}});
                for (i, (name, n, _d, ec)) in arms.iter().enumerate() {
                    let t0 = Instant::now(); let res = pools[i].get(*decl).action(1, &p, call.hand, true); let us = t0.elapsed().as_micros();
                    let worlds = if *ec > 0 && support <= *ec { support as f64 } else { *n as f64 };
                    row["arms"][name] = match res { Ok((t, v)) => json!({"us": us as u64, "choice": t, "worlds": worlds, "v": legal.iter().map(|&t| v[t as usize] / worlds).collect::<Vec<_>>()}), Err(m) => json!({"us": us as u64, "error": m}) };
                }
                for &w in &walt_arms {
                    let t0 = Instant::now(); let (counts, choice) = label_call(*decl, call, w, None); let us = t0.elapsed().as_micros();
                    let mut v = vec![0f64; 28]; for (t, k) in counts { v[t as usize] = k as f64 / w as f64; }
                    row["arms"][format!("walt{w}")] = json!({"us": us as u64, "choice": choice, "v": legal.iter().map(|&t| v[t as usize]).collect::<Vec<_>>()});
                }
                writeln!(f, "{}", row).unwrap();
            }
            println!("{}", json!({"bucket": bucket, "sampled": sample.len(), "population": count, "seconds": start.elapsed().as_secs_f64()}));
        }
        // log2: self-play by the void-aware turbo level-2 teacher (inner n0 worlds, outer n worlds); every
        // multi-legal state gets its full root vector (count of outer worlds where the declarer makes, per
        // legal play) written in the 100-byte label format; moves follow the teacher except eps random.
        "log2" => {
            let seed0: u64 = arg(&args, "--seed", "1").parse().unwrap(); let games: u64 = arg(&args, "--games", "1000000").parse().unwrap(); let out = arg(&args, "--out", "labels2.bin");
            let inner: usize = arg(&args, "--inner", "8").parse().unwrap(); let outer: usize = arg(&args, "--outer", "160").parse().unwrap(); let delta = turbo::parse_delta(&arg(&args, "--delta", "1")); let eps: f64 = arg(&args, "--eps", "0.1").parse().unwrap();
            let worker: u64 = arg(&args, "--worker", "0").parse().unwrap(); let workers: u64 = arg(&args, "--workers", "1").parse().unwrap(); let model = arg(&args, "--model", ""); let playp = arg(&args, "--play", "");
            // --stats prints engine counters to stderr. --stop-at SECS (default 280): no new game starts after this many
            // seconds (the 295 s cap wrapper kills the process; a 40/8 game takes ~0.4 s, a 160/8 game ~1.5 s).
            // The engine runs without its policy cache: answers are deterministic functions of the query key, and the
            // cache hit 55 of 17M level-1 queries in a 200-game measurement, so the labels are byte-identical (verified).
            // VARIANT (default off): --belief BEL.w [--belief-mult M] [--belief-alpha A] labels with the belief-weighted
            // teacher (see l2b_arm). M > 1: M x outer uniform candidates resampled to outer worlds, labels stay exact
            // counts of `outer` (N = outer). M = 1: importance-weighted make sums, stored as round(10000 x weighted
            // fraction) with N = 10000 (the turbo labeler's scale; the fraction uses the nominal mass outer x 4096).
            let belp = arg(&args, "--belief", ""); let bmult: usize = if belp.is_empty() { 1 } else { arg(&args, "--belief-mult", "1").parse().unwrap() }; let balpha: f64 = arg(&args, "--belief-alpha", "1").parse().unwrap();
            let bweighted = !belp.is_empty() && bmult == 1;
            // VARIANT (LAD6 deal mixture, default off): --bid-worlds K > 0 makes every game's bid level the highest the --play
            // net's K-world rollout makes with probability >= 1/2 (then a -1/0/+1 jitter), and with probability --mix the contract
            // is a random bidder seat with the trump the rollout prefers (1 in 5: a random trump). Labels then cover hands anyone
            // might bid on at the level they would bid; the bid is stored in record byte 2 bits 1..5 (bit 0 stays maximize).
            let mix: f64 = arg(&args, "--mix", "0").parse().unwrap(); let bid_worlds: u64 = arg(&args, "--bid-worlds", "0").parse().unwrap();
            let bid_uniform: f64 = arg(&args, "--bid-uniform", "0").parse().unwrap();
            // --enumerate CAP (default 0 = off): when the void-consistent support has at most CAP hidden deals, the outer ring uses every
            // one of them once (exact average over worlds, N = support size) instead of `outer` samples with replacement (late tricks).
            let enumerate_cap: u64 = arg(&args, "--enumerate", "0").parse().unwrap();
            // --outer-schedule n0,n1,...,n6 (default: --outer at every trick): outer worlds by trick of the labeled state (the early game is
            // where 160 samples of a 399M-deal support are noisy; late supports are small and --enumerate makes them exact).
            let sched: Vec<usize> = { let s = arg(&args, "--outer-schedule", ""); if s.is_empty() { vec![outer; 7] } else { let v: Vec<usize> = s.split(',').map(|x| x.parse().unwrap()).collect(); assert_eq!(v.len(), 7, "--outer-schedule needs 7 entries"); v } };
            let field_seed: u64 = arg(&args, "--field-seed", &turbo::FIELD_SEED.to_string()).parse().unwrap(); // the engine's world-sampling seed (a different one = an independent sample) // probability the bid level is uniform over 30..42 instead (high contracts are rare at break-even)
            let pnets: Vec<Net> = if bid_worlds > 0 { let pp = playp.strip_prefix("np:").expect("--bid-worlds needs --play np:PATH"); (0..7).map(|p| Net::load(pp, p as u8).unwrap()).collect() } else { Vec::new() };
            let stats = args.iter().any(|a| a == "--stats"); let stop_at: u64 = arg(&args, "--stop-at", "280").parse().unwrap(); let mut engines: std::collections::HashMap<usize, turbo::Engine> = std::collections::HashMap::new();
            let mut f = std::io::BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap()); let start = Instant::now(); let mut states = 0u64; let mut label_us = 0u128;
            for g in (0..games).filter(|g| g % workers == worker) {
                let (hands, mut bidder, mut decl) = deal(seed0 + g); let mut bid = 30u8;
                if bid_worlds > 0 {
                    let mut crng = SplitMix64(solver::mix(seed0 + g) ^ 0x6B1D);
                    let rollout = |s: usize, p: usize, r: &mut SplitMix64| -> [u32; 43] {
                        let hand = hands[s].iter().fold(0u32, |m, &t| m | 1 << t); let others: Vec<u64> = (0..28).filter(|&t| hand >> t & 1 == 0).collect(); let mut hist = [0u32; 43];
                        for _ in 0..bid_worlds {
                            let mut o = others.clone(); for i in (1..21).rev() { let j = r.below(i as u64 + 1) as usize; o.swap(i, j); }
                            let mut hs: [Vec<u64>; 4] = core::array::from_fn(|_| Vec::new()); hs[s] = hands[s].clone(); let mut q = 0;
                            for z in 0..4 { if z != s { hs[z] = o[7 * q..7 * q + 7].to_vec(); hs[z].sort(); q += 1; } }
                            let (_, pts, _) = play_bid(&hs, s, p, 255, 0, None, &|_| true, false, 0, true, Whole::Net(&pnets[p]), |_, _, _, _| {}); hist[pts[0] as usize] += 1;
                        }
                        hist
                    };
                    let hist = if crng.below(1_000_000) < (mix * 1_000_000.0) as u64 {
                        bidder = crng.below(4) as usize; let hs: Vec<[u32; 43]> = (0..7).map(|p| rollout(bidder, p, &mut crng)).collect();
                        let best = (0..7).max_by_key(|&p| hs[p][30..].iter().sum::<u32>()).unwrap();
                        decl = if crng.below(5) == 0 { crng.below(7) as usize } else { best }; hs[decl]
                    } else { rollout(bidder, decl, &mut crng) };
                    let mut level = 30usize; for b in 30..=42usize { if hist[b..].iter().sum::<u32>() as u64 * 2 >= bid_worlds { level = b; } }
                    let jitter = match crng.below(4) { 0 => -1, 3 => 1, _ => 0 }; bid = (level as i64 + jitter).clamp(30, 42) as u8;
                    if crng.below(1_000_000) < (bid_uniform * 1_000_000.0) as u64 { bid = 30 + crng.below(13) as u8; }
                }
                let dcl = solver::decl_of(decl); let engine = turbo::Engine::with_caps(decl, &[inner, outer * bmult], delta, field_seed, 4_000_000, enumerate_cap, 0); engine.set_bid(bid);
                if !model.is_empty() { let net: &'static Net = Box::leak(Box::new(Net::load_bid(&model, decl as u8, bid).unwrap())); engine.set_net_policy(1, net); }
                if !belp.is_empty() { let wb: &'static turbo::WorldBelief = Box::leak(Box::new(turbo::WorldBelief { net: belief::BeliefNet::load(&belp, decl as u8).unwrap(), alpha: balpha, ipf: 0 })); engine.set_world_belief(wb, if bmult > 1 { outer } else { 0 }); }
                let keep = if stats { engines.insert(decl, engine); None } else { Some(engine) }; let engine: &turbo::Engine = keep.as_ref().unwrap_or_else(|| &engines[&decl]);
                let player: Option<Net> = playp.strip_prefix("np:").map(|path| Net::load_bid(path, decl as u8, bid).unwrap());
                let mut rng = SplitMix64(solver::mix(seed0 + g) ^ 0x1092); let mut pairs: Vec<(usize, usize)> = Vec::new();
                for _ in 0..28 {
                    let st = solver::replay(dcl, bidder, &pairs);
                    let key = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: Some(st.voids), alive: 0 };
                    if (Contract::Straight { bid }).terminal(&key).is_some() { break; }
                    let seat_key = (st.leader as usize + st.plays.len()) % 4; let actor = (seat_key + 4 - st.r) % 4;
                    let own_mask = hands[actor].iter().fold(0u32, |m, &t| if st.played >> t & 1 == 0 { m | 1 << t } else { m });
                    let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
                    let legal = solver::mask_of(walt::rules::legal_plays(dcl, solver::set_of(own_mask), led));
                    let tile = if legal.count_ones() == 1 { legal.trailing_zeros() as u8 } else {
                        let p = turbo::pack(st.played, st.leader, &st.plays, st.banked_t1, st.banked_t0, Some(st.voids));
                        let trick = (st.played.count_ones() as usize) / 4; let n_outer = sched[trick.min(6)] * bmult; engine.set_samples(2, n_outer);
                        let t0 = Instant::now(); let (choice, v) = engine.action(2, &p, own_mask, true).expect("teacher"); label_us += t0.elapsed().as_micros(); states += 1;
                        let mut rec = [0u8; 100];
                        rec[0] = decl as u8; rec[1] = seat_key as u8; rec[2] = (seat_key % 2 == 1) as u8 | ((bid - 30) << 1); rec[3] = st.leader; rec[4] = st.plays.len() as u8;
                        for q in 0..4 { rec[5 + q] = *st.plays.get(q).unwrap_or(&255); }
                        rec[9] = st.banked_t1; rec[10] = st.banked_t0;
                        rec[11..15].copy_from_slice(&own_mask.to_le_bytes()); rec[15..19].copy_from_slice(&st.played.to_le_bytes()); rec[19..23].copy_from_slice(&legal.to_le_bytes());
                        let nw: u16 = if bweighted { 10_000 } else { engine.last_worlds() as u16 }; // the worlds actually used: the schedule's n, or the support size when enumerated
                        rec[23..25].copy_from_slice(&nw.to_le_bytes());
                        if bweighted { for t in 0..28usize { if legal >> t & 1 == 1 { let k = (v[t] / (outer as f64 * 4096.0) * 10_000.0).round().min(10_000.0); rec[25 + 2 * t..27 + 2 * t].copy_from_slice(&(k as u16).to_le_bytes()); } } } else {
                        for t in 0..28usize { if legal >> t & 1 == 1 { let k = v[t]; assert!(k.is_finite() && (k - k.round()).abs() < 1e-9, "non-integer count"); rec[25 + 2 * t..27 + 2 * t].copy_from_slice(&(k.round() as u16).to_le_bytes()); } } }
                        rec[81] = choice; rec[82] = choice; rec[83] = 1; for q in 0..4 { rec[84 + 4 * q..88 + 4 * q].copy_from_slice(&st.voids[q].to_le_bytes()); }
                        f.write_all(&rec).unwrap();
                        if rng.below(1_000_000) < (eps * 1_000_000.0) as u64 { let n = legal.count_ones(); let k = rng.below(n as u64) as u32; let mut m = legal; for _ in 0..k { m &= m - 1; } m.trailing_zeros() as u8 }
                        else if let Some(pl) = &player { net_hook::NetPolicy::choose(pl, &key, Seat::from_index(seat_key).unwrap(), own_mask, legal) } else { choice }
                    };
                    pairs.push((actor, tile as usize));
                }
                f.flush().unwrap();
                if start.elapsed() > Duration::from_secs(stop_at) { break; }
            }
            if stats { let mut tot = vec![[0u64; 8]; 2]; for e in engines.values() { for (l, s) in e.stats().iter().enumerate() { for k in 0..8 { tot[l][k] += s[k]; } } } eprintln!("{}", json!({"stats_per_level": tot, "fields": ["queries", "cache_hits", "forced", "nodes", "fiber_visits", "peak", "sampled_deals", "sample_cache_hits"], "engines": engines.len()})); }
            println!("{}", json!({"worker": worker, "states": states, "label_us_mean": if states > 0 { label_us as f64 / states as f64 } else { 0.0 }, "seconds": start.elapsed().as_secs_f64()}));
        }
        // belief-log: self-play by a policy net in all four seats (eps random), writing every multi-legal decision state
        // with the TRUE hidden hands, 128-byte records: bytes 0..100 = the label record (N = 0, counts 0, choice = the
        // tile played, prod = the net's greedy pick), 100..112 = remaining hand masks of the relative seats mover+1..3
        // (key frame), 112..120 = deal seed (u64), 120 = 1 if the play was the eps-random one, 121 = trick number.
        "belief-log" => {
            let seed0: u64 = arg(&args, "--seed", "1").parse().unwrap(); let games: u64 = arg(&args, "--games", "1000000").parse().unwrap(); let out = arg(&args, "--out", "belief.bin");
            let eps: f64 = arg(&args, "--eps", "0.1").parse().unwrap(); let worker: u64 = arg(&args, "--worker", "0").parse().unwrap(); let workers: u64 = arg(&args, "--workers", "1").parse().unwrap();
            let playp = arg(&args, "--play", ""); let path = playp.strip_prefix("np:").expect("--play np:PATH").to_string(); let stop_at: u64 = arg(&args, "--stop-at", "280").parse().unwrap();
            let mut nets: std::collections::HashMap<usize, Net> = std::collections::HashMap::new();
            let mut f = std::io::BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap()); let start = Instant::now(); let (mut states, mut ngames) = (0u64, 0u64);
            for g in (0..games).filter(|g| g % workers == worker) {
                let (hands, bidder, decl) = deal(seed0 + g); let dcl = solver::decl_of(decl);
                let net = nets.entry(decl).or_insert_with(|| Net::load(&path, decl as u8).unwrap());
                let mut rng = SplitMix64(solver::mix(seed0 + g) ^ 0xBE11_EF00); let mut pairs: Vec<(usize, usize)> = Vec::new();
                for _ in 0..28 {
                    let st = solver::replay(dcl, bidder, &pairs);
                    let key = Key { played: st.played, leader: st.leader, plays: st.plays.clone(), banked_t1: st.banked_t1, banked_t0: st.banked_t0, voids: Some(st.voids), alive: 0 };
                    if (Contract::Straight { bid: 30 }).terminal(&key).is_some() { break; }
                    let seat_key = (st.leader as usize + st.plays.len()) % 4; let actor = (seat_key + 4 - st.r) % 4;
                    let rem = |a: usize| hands[a].iter().fold(0u32, |m, &t| if st.played >> t & 1 == 0 { m | 1 << t } else { m });
                    let own_mask = rem(actor);
                    let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
                    let legal = solver::mask_of(walt::rules::legal_plays(dcl, solver::set_of(own_mask), led));
                    let tile = if legal.count_ones() == 1 { legal.trailing_zeros() as u8 } else {
                        let greedy = net_hook::NetPolicy::choose(net, &key, Seat::from_index(seat_key).unwrap(), own_mask, legal);
                        let random = rng.below(1_000_000) < (eps * 1_000_000.0) as u64;
                        let tile = if random { let n = legal.count_ones(); let k = rng.below(n as u64) as u32; let mut m = legal; for _ in 0..k { m &= m - 1; } m.trailing_zeros() as u8 } else { greedy };
                        let mut rec = [0u8; 128];
                        rec[0] = decl as u8; rec[1] = seat_key as u8; rec[2] = (seat_key % 2 == 1) as u8; rec[3] = st.leader; rec[4] = st.plays.len() as u8;
                        for q in 0..4 { rec[5 + q] = *st.plays.get(q).unwrap_or(&255); }
                        rec[9] = st.banked_t1; rec[10] = st.banked_t0;
                        rec[11..15].copy_from_slice(&own_mask.to_le_bytes()); rec[15..19].copy_from_slice(&st.played.to_le_bytes()); rec[19..23].copy_from_slice(&legal.to_le_bytes());
                        rec[81] = tile; rec[82] = greedy; rec[83] = 1; for q in 0..4 { rec[84 + 4 * q..88 + 4 * q].copy_from_slice(&st.voids[q].to_le_bytes()); }
                        let mut all = own_mask;
                        for k in 0..3 { let a = ((seat_key + 1 + k) % 4 + 4 - st.r) % 4; let m = rem(a); all |= m; rec[100 + 4 * k..104 + 4 * k].copy_from_slice(&m.to_le_bytes()); }
                        assert_eq!(all | st.played, 0x0fff_ffff, "hands and played do not partition the tiles");
                        rec[112..120].copy_from_slice(&(seed0 + g).to_le_bytes()); rec[120] = random as u8; rec[121] = ((st.played.count_ones() as usize - st.plays.len()) / 4) as u8;
                        f.write_all(&rec).unwrap(); states += 1; tile
                    };
                    pairs.push((actor, tile as usize));
                }
                ngames += 1;
                if start.elapsed() > Duration::from_secs(stop_at) { break; }
            }
            f.flush().unwrap();
            println!("{}", json!({"worker": worker, "games": ngames, "states": states, "seconds": start.elapsed().as_secs_f64()}));
        }
        // belief-from-games: belief-log records (same 128-byte layout; prod = 255, eps flag 0) from recorded h2h games
        // (--games comma-separated games-*.jsonl files): every multi-legal decision of the hybrid seats (actor % 2 ==
        // side), i.e. the states a hybrid arm faces against production. --exclude-seed S skips deals of the h2h seed S
        // (deal_seed - deal == S). --all-seats also writes the production seats' decisions.
        "belief-from-games" => {
            let files = arg(&args, "--games", ""); let out = arg(&args, "--out", "belief-games.bin"); let excl: i64 = arg(&args, "--exclude-seed", "-1").parse().unwrap(); let all_seats = args.iter().any(|a| a == "--all-seats");
            let mut f = std::io::BufWriter::new(std::fs::File::create(&out).unwrap()); let (mut states, mut ngames, mut skipped) = (0u64, 0u64, 0u64);
            for path in files.split(',').filter(|p| !p.is_empty()) { for line in std::fs::read_to_string(path).unwrap().lines() {
                let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
                let (d, ds, side) = (v["deal"].as_u64().unwrap(), v["deal_seed"].as_u64().unwrap(), v["side"].as_u64().unwrap() as usize);
                if ds as i64 - d as i64 == excl { skipped += 1; continue; }
                let (hands, bidder, decl) = deal(ds); assert_eq!((bidder, decl), (v["bidder"].as_u64().unwrap() as usize, v["decl"].as_u64().unwrap() as usize), "deal mismatch");
                let dcl = solver::decl_of(decl); let rec_plays: Vec<usize> = v["plays"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as usize).collect();
                let mut pairs: Vec<(usize, usize)> = Vec::new();
                for i in (0..rec_plays.len()).step_by(2) {
                    let (actor, tile) = (rec_plays[i], rec_plays[i + 1]); let st = solver::replay(dcl, bidder, &pairs);
                    let seat_key = (st.leader as usize + st.plays.len()) % 4; assert_eq!((seat_key + 4 - st.r) % 4, actor, "actor mismatch");
                    let rem = |a: usize| hands[a].iter().fold(0u32, |m, &t| if st.played >> t & 1 == 0 { m | 1 << t } else { m });
                    let own_mask = rem(actor);
                    let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
                    let legal = solver::mask_of(walt::rules::legal_plays(dcl, solver::set_of(own_mask), led));
                    if legal.count_ones() > 1 && (all_seats || actor % 2 == side) {
                        let mut rec = [0u8; 128];
                        rec[0] = decl as u8; rec[1] = seat_key as u8; rec[2] = (seat_key % 2 == 1) as u8; rec[3] = st.leader; rec[4] = st.plays.len() as u8;
                        for q in 0..4 { rec[5 + q] = *st.plays.get(q).unwrap_or(&255); }
                        rec[9] = st.banked_t1; rec[10] = st.banked_t0;
                        rec[11..15].copy_from_slice(&own_mask.to_le_bytes()); rec[15..19].copy_from_slice(&st.played.to_le_bytes()); rec[19..23].copy_from_slice(&legal.to_le_bytes());
                        rec[81] = tile as u8; rec[82] = 255; rec[83] = 1; for q in 0..4 { rec[84 + 4 * q..88 + 4 * q].copy_from_slice(&st.voids[q].to_le_bytes()); }
                        for k in 0..3 { let a = ((seat_key + 1 + k) % 4 + 4 - st.r) % 4; rec[100 + 4 * k..104 + 4 * k].copy_from_slice(&rem(a).to_le_bytes()); }
                        rec[112..120].copy_from_slice(&ds.to_le_bytes()); rec[121] = ((st.played.count_ones() as usize - st.plays.len()) / 4) as u8;
                        f.write_all(&rec).unwrap(); states += 1;
                    }
                    pairs.push((actor, tile));
                }
                ngames += 1;
            } }
            f.flush().unwrap(); println!("{}", json!({"games": ngames, "skipped": skipped, "states": states}));
        }
        // belief-eval: score a belief head (--net BEL.w) on belief-log records (--data FILE, --limit N): per trick, log
        // loss and accuracy of the true holder of every unseen tile under the belief, under the exact uniform-support
        // marginal (what turbo's sampler assumes) and under naive 1/#allowed seats; calibration (10 bins of predicted
        // probability over all allowed (tile, seat) pairs) for belief and uniform. --dump FILE writes the raw 84 logits
        // per record (parity check). --net none scores the baselines only.
        "belief-eval" => {
            let netp = arg(&args, "--net", ""); let data = std::fs::read(arg(&args, "--data", "")).unwrap(); let limit: usize = arg(&args, "--limit", "100000000").parse().unwrap();
            let dp = arg(&args, "--dump", ""); let mut dump = if dp.is_empty() { None } else { Some(std::io::BufWriter::new(std::fs::File::create(&dp).unwrap())) };
            let mut nets: std::collections::HashMap<u8, belief::BeliefNet> = std::collections::HashMap::new();
            // per trick: [tiles, ll_belief, ll_uniform, ll_naive, acc_belief, acc_uniform, acc_naive, states]
            let mut by = [[0f64; 8]; 7]; let ipf: usize = arg(&args, "--ipf", "8").parse().unwrap(); let tiltd = args.iter().any(|a| a == "--tilt");
            // --tilt: also the log loss of the marginals the search actually draws from: naive ratio tilt and IPF tilt
            let mut tl = [[0f64; 3]; 7]; let mut cal = [[[0f64; 3]; 10]; 2];
            for rec in data.chunks_exact(128).take(limit) {
                let decl = rec[0]; let u32at = |i: usize| u32::from_le_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]]);
                let npl = rec[4] as usize; let key = Key { played: u32at(15), leader: rec[3], plays: rec[5..5 + npl].to_vec(), banked_t1: rec[9], banked_t0: rec[10], voids: Some([u32at(84), u32at(88), u32at(92), u32at(96)]), alive: 0 };
                let seat = Seat::from_index(rec[1] as usize).unwrap(); let hand = u32at(11); let others = [u32at(100), u32at(104), u32at(108)];
                let fr = belief::frame(&key, seat, hand); let pu = belief::uniform_marginals(&fr);
                let pb = if netp == "none" { pu } else { let net = nets.entry(decl).or_insert_with(|| belief::BeliefNet::load(&netp, decl).unwrap());
                    if let Some(d) = dump.as_mut() { writeln!(d, "{}", json!(net.logits(&key, seat, hand).to_vec())).unwrap(); } net.probs(&key, seat, hand, &fr) };
                let trick = rec[121] as usize; by[trick][7] += 1.0;
                let (qn, qi) = if tiltd && netp != "none" { (belief::marginals_of_log_tilt(&fr, &belief::log_ratio(&pb, &pu, &fr, 1.0)), belief::marginals_of_log_tilt(&fr, &belief::ipf_log_tilt(&pb, &pu, &fr, ipf, 1.0))) } else { (pu, pu) };
                let mut m = fr.unseen; while m != 0 { let t = m.trailing_zeros() as usize; m &= m - 1;
                    let k = (0..3).find(|&k| others[k] >> t & 1 == 1).expect("unseen tile with no holder"); assert!(fr.allowed[t] >> k & 1 == 1, "true holder not allowed by the frame");
                    let nal = fr.allowed[t].count_ones() as f64; let am = |p: &[f64; 3]| (0..3).filter(|&j| fr.allowed[t] >> j & 1 == 1).fold(None::<usize>, |b, j| match b { Some(b) if p[b] >= p[j] => Some(b), _ => Some(j) }).unwrap();
                    let naive = { let mut q = [0f64; 3]; for j in 0..3 { if fr.allowed[t] >> j & 1 == 1 { q[j] = 1.0 / nal; } } q };
                    tl[trick][0] += 1.0; tl[trick][1] -= qn[t][k].max(1e-12).ln(); tl[trick][2] -= qi[t][k].max(1e-12).ln();
                    let e = &mut by[trick]; e[0] += 1.0; e[1] -= pb[t][k].max(1e-12).ln(); e[2] -= pu[t][k].ln(); e[3] += nal.ln();
                    e[4] += (am(&pb[t]) == k) as u8 as f64; e[5] += (am(&pu[t]) == k) as u8 as f64; e[6] += (am(&naive) == k) as u8 as f64;
                    for j in 0..3 { if fr.allowed[t] >> j & 1 == 1 && fr.allowed[t].count_ones() > 1 { for (c, p) in [(0, pb[t][j]), (1, pu[t][j])] { let b = ((p * 10.0) as usize).min(9); cal[c][b][0] += 1.0; cal[c][b][1] += p; cal[c][b][2] += (j == k) as u8 as f64; } } }
                }
            }
            let mut tot = [0f64; 8]; for e in &by { for i in 0..8 { tot[i] += e[i]; } }
            let row = |e: &[f64; 8]| json!({"states": e[7], "tiles": e[0], "ll_belief": e[1] / e[0], "ll_uniform": e[2] / e[0], "ll_naive": e[3] / e[0], "acc_belief": e[4] / e[0], "acc_uniform": e[5] / e[0], "acc_naive": e[6] / e[0]});
            let calj = |c: usize| (0..10).filter(|&b| cal[c][b][0] > 0.0).map(|b| json!({"bin": format!("{:.1}-{:.1}", b as f64 / 10.0, (b + 1) as f64 / 10.0), "n": cal[c][b][0], "mean_pred": cal[c][b][1] / cal[c][b][0], "freq": cal[c][b][2] / cal[c][b][0]})).collect::<Vec<_>>();
            let tilt: Vec<Value> = (0..7).filter(|&t| tl[t][0] > 0.0 && tiltd).map(|t| json!({"trick": t, "ll_naive_tilt": tl[t][1] / tl[t][0], "ll_ipf_tilt": tl[t][2] / tl[t][0]})).collect();
            println!("{}", json!({"net": netp, "tilt": tilt, "by_trick": (0..7).filter(|&t| by[t][0] > 0.0).map(|t| { let mut r = row(&by[t]); r["trick"] = json!(t); r }).collect::<Vec<_>>(), "all": row(&tot), "calibration_belief": calj(0), "calibration_uniform": calj(1)}));
        }
        // agree: Rust-forward check of a net against label records (choice agreement with the stored teacher choice).
        // netbench: forward-pass cost in isolation. --net PATH [--iters N] [--decl D]; synthetic mid-game states
        // (random played mask / hand / trick), reports ns per scores() call and a checksum of the chosen tiles.
        // l2bench: time single L2-player decisions by replaying recorded h2h games (games-*.jsonl rows), so inner-world
        // settings whose full games would not fit a capped round can still be measured, and every answer is checked
        // against the tile the same arm recorded in the game (`recorded`). --games FILE --net l2z:O:I:path --out FILE
        // [--max-per-game K] [--skip K0] [--seconds S] [--worker w --workers n]; resumable (done (deal, side, idx) keys are skipped).
        "l2bench" => {
            let gf = arg(&args, "--games", ""); let netp = arg(&args, "--net", ""); let out = arg(&args, "--out", "l2bench.jsonl");
            let maxk: usize = arg(&args, "--max-per-game", "1000").parse().unwrap(); let budget: f64 = arg(&args, "--seconds", "270").parse().unwrap();
            let skip: u64 = arg(&args, "--skip", "0").parse().unwrap(); // skip the first K0 hybrid decisions of every game (later decisions are cheaper)
            let worker: usize = arg(&args, "--worker", "0").parse().unwrap(); let workers: usize = arg(&args, "--workers", "1").parse().unwrap();
            let rows: Vec<Value> = std::fs::read_to_string(&gf).unwrap().lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
            let done: std::collections::HashSet<(u64, u64, u64)> = std::fs::read_to_string(&out).unwrap_or_default().lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()).map(|v| (v["deal"].as_u64().unwrap(), v["side"].as_u64().unwrap(), v["idx"].as_u64().unwrap())).collect();
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap();
            let start = Instant::now(); let mut leaf = [0u64; 4]; let (mut n, mut agree) = (0usize, 0usize);
            'rows: for (ri, row) in rows.iter().enumerate() {
                if ri % workers != worker { continue; }
                let (d, side, decl, bidder) = (row["deal"].as_u64().unwrap(), row["side"].as_u64().unwrap(), row["decl"].as_u64().unwrap() as usize, row["bidder"].as_u64().unwrap() as usize);
                let rec: Vec<u64> = row["plays"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
                let (hands, b2, d2) = deal(row["deal_seed"].as_u64().unwrap()); assert_eq!((b2, d2), (bidder, decl), "deal mismatch");
                let want: Vec<usize> = (0..rec.len() / 2).filter(|&i| rec[2 * i] % 2 == side).collect(); let _ = want;
                let dcl = solver::decl_of(decl); let mut pairs: Vec<(usize, usize)> = Vec::new(); let mut k = 0u64; let mut p2: Option<turbo::L2Player> = None;
                for i in (0..rec.len()).step_by(2) {
                    let actor = rec[i] as usize; let st = solver::replay(dcl, bidder, &pairs);
                    let own: Vec<u64> = hands[actor].iter().copied().filter(|&t| st.played >> t & 1 == 0).collect();
                    let led = st.plays.first().map(|&t| dcl.led_context(walt::rules::Domino::from_index(t as usize).unwrap()));
                    let own_mask = own.iter().fold(0u32, |m, &t| m | 1 << t); let legal_mask = solver::mask_of(walt::rules::legal_plays(dcl, solver::set_of(own_mask), led));
                    if actor as u64 % 2 == side && legal_mask.count_ones() > 1 {
                        if k >= skip && ((k - skip) as usize) < maxk && !done.contains(&(d, side, k)) {
                            if start.elapsed().as_secs_f64() > budget { break 'rows; }
                            let p = p2.get_or_insert_with(|| l2_arm(decl, &netp));
                            let m0 = p.engine.lock().unwrap().metrics(); let t0 = Instant::now();
                            let t = p.choose(&turbo::pack(st.played, st.leader, &st.plays, st.banked_t1, st.banked_t0, Some(st.voids)), own_mask, legal_mask);
                            let us = t0.elapsed().as_micros() as u64; let m1 = p.engine.lock().unwrap().metrics();
                            n += 1; if t as u64 == rec[i + 1] { agree += 1; }
                            writeln!(f, "{}", json!({"deal": d, "side": side, "idx": k, "depth": st.played.count_ones(), "legal": legal_mask.count_ones(), "tile": t, "recorded": rec[i + 1], "us": us, "hook_calls": m1[6] - m0[6], "hook_hits": m1[7] - m0[7]})).unwrap(); f.flush().unwrap();
                        }
                        k += 1;
                    }
                    pairs.push((actor, rec[i + 1] as usize));
                }
                if let Some(p) = &p2 { let m = p.engine.lock().unwrap().metrics(); leaf[0] += m[6]; leaf[1] += m[7]; leaf[2] += m[8]; leaf[3] = leaf[3].max(m[9]); }
            }
            println!("{}", json!({"worker": worker, "decisions": n, "agree_with_recorded": agree, "seconds": start.elapsed().as_secs_f64(), "leaf_memo": {"calls": leaf[0], "hits": leaf[1], "clears": leaf[2], "max_entries": leaf[3]}}));
        }
        "netbench" => {
            let path = arg(&args, "--net", ""); let iters: usize = arg(&args, "--iters", "200000").parse().unwrap(); let decl: u8 = arg(&args, "--decl", "3").parse().unwrap();
            let net = Net::load(&path, decl).unwrap(); let mut rng = 0x9e3779b97f4a7c15u64; let mut next = || { rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17; rng };
            let mut cases: Vec<(Key, u32, u32, Seat)> = Vec::new();
            for _ in 0..4096 {
                let depth = (next() % 24) as usize; let trick = depth / 4; let len = depth % 4;
                let mut tiles: Vec<u8> = (0..28).collect(); for i in (1..28).rev() { let j = (next() % (i as u64 + 1)) as usize; tiles.swap(i, j); }
                let played: u32 = tiles[..trick * 4 + len].iter().fold(0, |m, &t| m | 1 << t); let plays: Vec<u8> = tiles[trick * 4..trick * 4 + len].to_vec();
                let own = 7 - trick; let hand: u32 = tiles[trick * 4 + len..trick * 4 + len + own].iter().fold(0, |m, &t| m | 1 << t);
                let leader = (next() % 4) as u8; let seat = Seat::from_index((leader as usize + len) % 4).unwrap();
                // voids: each other seat is void in 0..=2 suits (suit masks of the declaration), realistic for mid-game
                let suit_mask = |q: u8| -> u32 { let mut m = 0u32; for t in 0..28u8 { let (hi, lo) = (net::hi(t), net::lo(t)); let tr = hi == decl || lo == decl; if q == 7 { if tr { m |= 1 << t; } } else if !tr && (hi == q || lo == q) { m |= 1 << t; } } m };
                let mut voids = [0u32; 4]; let nv = arg(&args, "--voids", "2").parse::<u64>().unwrap(); for v in voids.iter_mut() { for _ in 0..next() % (nv + 1) { *v |= suit_mask((next() % 8) as u8); } }
                cases.push((Key { played, leader, plays, banked_t1: (next() % 30) as u8, banked_t0: (next() % 13) as u8, voids: Some(voids), alive: 0 }, hand, hand, seat));
            }
            let t0 = Instant::now(); let mut sum = 0u64;
            for i in 0..iters { let (k, hand, legal, seat) = &cases[i % cases.len()]; sum += net_hook::NetPolicy::choose(&net, k, *seat, *hand, *legal) as u64; }
            let ns = t0.elapsed().as_nanos() as f64 / iters as f64;
            // ReLU sparsity of the first hidden layer over the cases (fraction of nonzero activations), via Net::hidden_nonzero
            let nz: f64 = cases.iter().map(|(k, hand, _, seat)| net.hidden_nonzero(k, *seat, *hand) as f64 / net.h as f64).sum::<f64>() / cases.len() as f64;
            println!("{}", json!({"net": path, "h": net.h, "h2": net.h2, "generic": net.generic, "iters": iters, "ns_per_call": ns, "checksum": sum, "hidden_nonzero_frac": nz}));
        }
        "agree" => {
            let netp = arg(&args, "--net", ""); let labels = std::fs::read(arg(&args, "--labels", "")).unwrap(); let limit: usize = arg(&args, "--limit", "20000").parse().unwrap();
            let mut nets: std::collections::HashMap<(u8, u8), Net> = std::collections::HashMap::new(); let (mut n, mut agree, mut rnd) = (0usize, 0usize, 0f64); let dp = arg(&args, "--dump", ""); let mut dump = if dp.is_empty() { None } else { Some(std::fs::File::create(&dp).unwrap()) };
            for rec in labels.chunks_exact(100).take(limit) {
                let decl = rec[0]; let bid = 30 + (rec[2] >> 1); let net = nets.entry((decl, bid)).or_insert_with(|| Net::load_bid(&netp, decl, bid).unwrap()); // byte 2 bits 1..5: the contract (LAD6)
                let npl = rec[4] as usize; let plays: Vec<u8> = rec[5..5 + npl].to_vec(); let u32at = |i: usize| u32::from_le_bytes([rec[i], rec[i + 1], rec[i + 2], rec[i + 3]]);
                let voids = if rec[83] == 1 { Some([u32at(84), u32at(88), u32at(92), u32at(96)]) } else { None };
                let key = Key { played: u32at(15), leader: rec[3], plays, banked_t1: rec[9], banked_t0: rec[10], voids, alive: 0 };
                let legal = u32at(19); let t = net_hook::NetPolicy::choose(net, &key, Seat::from_index(rec[1] as usize).unwrap(), u32at(11), legal);
                n += 1; if t == rec[81] { agree += 1; } rnd += 1.0 / legal.count_ones() as f64;
                if let Some(d) = dump.as_mut() { let sc = net.scores(&key, Seat::from_index(rec[1] as usize).unwrap(), u32at(11), legal); writeln!(d, "{}", json!({"i": n - 1, "choice": t, "scores": (0..28).filter(|&q| legal >> q & 1 == 1).map(|q| sc[q]).collect::<Vec<f32>>()})).unwrap(); }
            }
            println!("{}", json!({"records": n, "agree": agree as f64 / n as f64, "random_baseline": rnd / n as f64}));
        }
        "rollrelabel" => {
            // Outcome labels across worlds with players like itself (exploratory, 2026-10-08, Jason: "an inner mind that passes a
            // bideval"). Reads a labels2 file, keeps its trick-0 rows (nothing played, mover = leader = bidder), and relabels each:
            // for every tile in the hand, K uniform redeals of the 21 unseen tiles, the tile is led and the net plays all four seats
            // to the contract's end; counts[t] = worlds made at the row's bid, N = K. The record is otherwise copied (bid, decl,
            // seat, hand, legal, voids); choice = argmax. --worker i --workers n stride the rows; --threads inside; resumable by
            // row count of --out.
            let path = arg(&args, "--net", ""); let labels = arg(&args, "--labels", ""); let out = arg(&args, "--out", "rollrelabel.bin");
            let worlds: u64 = arg(&args, "--worlds", "64").parse().unwrap(); let threads: usize = arg(&args, "--threads", "1").parse().unwrap();
            let worker: usize = arg(&args, "--worker", "0").parse().unwrap(); let workers: usize = arg(&args, "--workers", "1").parse().unwrap();
            let limit: usize = arg(&args, "--limit", "1000000000").parse().unwrap(); let seed: u64 = arg(&args, "--seed", "0").parse().unwrap();
            let data = std::fs::read(&labels).unwrap(); let nrec = data.len() / 100;
            let u32at = |r: &[u8], i: usize| u32::from_le_bytes([r[i], r[i + 1], r[i + 2], r[i + 3]]);
            let rows: Vec<usize> = (0..nrec).filter(|&i| { let r = &data[100 * i..100 * i + 100]; r[4] == 0 && u32at(r, 15) == 0 }).collect();
            let mine: Vec<usize> = rows.iter().copied().enumerate().filter(|(j, _)| j % workers == worker).map(|(_, i)| i).take(limit).collect();
            let done = std::fs::metadata(&out).map(|m| m.len() as usize / 100).unwrap_or(0);
            let todo: Vec<usize> = mine[done.min(mine.len())..].to_vec();
            eprintln!("{}", json!({"rows_total": nrec, "root_rows": rows.len(), "mine": mine.len(), "done": done, "todo": todo.len()}));
            let mut nets: std::collections::HashMap<(u8, u8), Net> = std::collections::HashMap::new();
            for &i in &todo { let r = &data[100 * i..100 * i + 100]; let k = (r[0], 30 + (r[2] >> 1)); if !nets.contains_key(&k) { nets.insert(k, Net::load_bid(&path, k.0, k.1).unwrap()); } }
            let f = std::sync::Mutex::new(std::io::BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap()));
            let next = std::sync::atomic::AtomicUsize::new(0); let t0 = Instant::now(); let count = std::sync::atomic::AtomicUsize::new(0);
            std::thread::scope(|sc| { for _ in 0..threads { sc.spawn(|| loop {
                let j = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed); if j >= todo.len() { break; } let i = todo[j];
                let mut rec = [0u8; 100]; rec.copy_from_slice(&data[100 * i..100 * i + 100]);
                let decl = rec[0] as usize; let bid = 30 + (rec[2] >> 1); let hand = u32at(&rec, 11); let legal = u32at(&rec, 19);
                let net = &nets[&(rec[0], bid)];
                let others: Vec<u64> = (0..28).filter(|&t| hand >> t & 1 == 0).collect(); let own: Vec<u64> = (0..28).filter(|&t| hand >> t & 1 == 1).collect();
                let mut r = SplitMix64((i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0x5151_D00D ^ seed.wrapping_mul(0xD1B5_4A32_D192_ED03));
                let mut counts = [0u16; 28];
                for w in 0..worlds {
                    let mut o = others.clone(); for q in (1..21).rev() { let z = r.below(q as u64 + 1) as usize; o.swap(q, z); }
                    let mut hs: [Vec<u64>; 4] = core::array::from_fn(|_| Vec::new()); hs[0] = own.clone();
                    for z in 1..4 { hs[z] = o[7 * (z - 1)..7 * z].to_vec(); hs[z].sort(); }
                    let _ = w;
                    for t in 0..28u8 { if legal >> t & 1 == 1 {
                        let (_, _, made) = play_bid(&hs, 0, decl, bid, 0, None, &|_| true, false, 0, true, Whole::NetForced(net, t), |_, _, _, _| {});
                        if made { counts[t as usize] += 1; }
                    } }
                }
                let mut best = 0u8; let mut bk = -1i32;
                for t in 0..28usize { if legal >> t & 1 == 1 { rec[25 + 2 * t..27 + 2 * t].copy_from_slice(&counts[t].to_le_bytes()); if counts[t] as i32 > bk { bk = counts[t] as i32; best = t as u8; } } else { rec[25 + 2 * t..27 + 2 * t].copy_from_slice(&0u16.to_le_bytes()); } }
                rec[23..25].copy_from_slice(&(worlds as u16).to_le_bytes()); rec[81] = best; rec[82] = best;
                f.lock().unwrap().write_all(&rec).unwrap();
                let c = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1; if c % 200 == 0 { eprintln!("{}", json!({"rows": c, "secs": t0.elapsed().as_secs_f64()})); }
            }); } });
            f.lock().unwrap().flush().unwrap(); eprintln!("{}", json!({"rows": count.load(std::sync::atomic::Ordering::Relaxed), "secs": t0.elapsed().as_secs_f64(), "out": out}));
        }
        "bideval" => {
            // Bid probe (exploratory). For every (uniform deal, bidder seat, trump): the standalone net estimate of
            // P(make 30) at trick 0 (max over the 7 opening leads of sigmoid(score), no search, 7 forwards per hand),
            // a K-world net-self-play rollout (histogram of the bidding team's points over uniform redeals of the 21
            // unseen tiles), and the realized net-self-play outcome on the actual deal. The deal rule's contract is
            // recorded alongside. Resumable: deals already in --out are skipped.
            let path = arg(&args, "--net", ""); let seed0: u64 = arg(&args, "--seed0", "9000").parse().unwrap(); let deals: u64 = arg(&args, "--deals", "100").parse().unwrap();
            let worlds: u64 = arg(&args, "--worlds", "64").parse().unwrap(); let out = arg(&args, "--out", "bideval.jsonl"); let workers: usize = arg(&args, "--workers", "4").parse().unwrap();
            // --prod: realize every contract with production Walt in all four seats (no rollouts); the deal's real outcome under the deployed player.
            let prod = args.iter().any(|a| a == "--prod");
            // --bid B (30..42, default 30): the contract the standalone estimate is asked about (encoding-5 nets read it); rollouts
            // and realized net games play all 28 tiles so the histogram covers every level; "made" = points >= B. --prod is pinned to 30.
            let bid: u8 = arg(&args, "--bid", "30").parse().unwrap(); assert!(!prod || bid == 30, "--prod is pinned to bid 30");
            let nets: Vec<Net> = (0..7).map(|p| Net::load_bid(&path, p as u8, bid).unwrap()).collect();
            let done: std::collections::HashSet<u64> = std::fs::read_to_string(&out).ok().map(|s| s.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()).filter_map(|v| v["deal"].as_u64()).collect()).unwrap_or_default();
            let todo: Vec<u64> = (0..deals).map(|d| seed0 + d).filter(|d| !done.contains(d)).collect();
            let f = std::sync::Mutex::new(std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap());
            let next = std::sync::atomic::AtomicUsize::new(0);
            std::thread::scope(|sc| { for _ in 0..workers { sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed); if i >= todo.len() { break; } let d = todo[i];
                let (hands, rb, rd) = deal(d); let mut rows = Vec::new();
                for s in 0..4 { for p in 0..7 {
                    let net = &nets[p]; let dcl = solver::decl_of(p); let st = solver::replay(dcl, s, &[]);
                    let key = Key { played: 0, leader: st.leader, plays: vec![], banked_t1: 0, banked_t0: 0, voids: Some([0; 4]), alive: 0 };
                    let hand = hands[s].iter().fold(0u32, |m, &t| m | 1 << t);
                    let sc = net.scores(&key, Seat::from_index((s + st.r) % 4).unwrap(), hand, hand);
                    let leads: Vec<f64> = (0..28).filter(|&t| hand >> t & 1 == 1).map(|t| 1.0 / (1.0 + (-(sc[t] as f64)).exp())).collect();
                    let standalone = leads.iter().cloned().fold(0f64, f64::max);
                    if prod {
                        let (_, pts, made) = play(&hands, s, p, d * 1_000_003 + (s * 7 + p) as u64, None, &|_| false, false, 14_000, false, Whole::None, |_, _, _, _| {});
                        rows.push(json!({"seat": s, "decl": p, "standalone": standalone, "leads": leads, "hist": Vec::<u32>::new(), "real_pts": pts[0], "real_made": made})); continue;
                    }
                    let mut hist = [0u32; 43]; let mut r = SplitMix64((d * 1_000_003 + (s * 7 + p) as u64) ^ 0xA5A5_5A5A);
                    let others: Vec<u64> = (0..28).filter(|&t| hand >> t & 1 == 0).collect();
                    for _ in 0..worlds {
                        let mut o = others.clone(); for i in (1..21).rev() { let j = r.below(i as u64 + 1) as usize; o.swap(i, j); }
                        let mut hs: [Vec<u64>; 4] = core::array::from_fn(|_| Vec::new()); hs[s] = hands[s].clone(); let mut q = 0;
                        for z in 0..4 { if z != s { hs[z] = o[7 * q..7 * q + 7].to_vec(); hs[z].sort(); q += 1; } }
                        let (_, pts, _) = play_bid(&hs, s, p, 255, 0, None, &|_| true, false, 0, true, Whole::Net(net), |_, _, _, _| {});
                        hist[pts[0] as usize] += 1;
                    }
                    let (_, pts, _) = play_bid(&hands, s, p, 255, 0, None, &|_| true, false, 0, true, Whole::Net(net), |_, _, _, _| {}); let made = pts[0] >= bid as u64;
                    rows.push(json!({"seat": s, "decl": p, "standalone": standalone, "leads": leads, "hist": hist.to_vec(), "real_pts": pts[0], "real_made": made}));
                } }
                writeln!(f.lock().unwrap(), "{}", json!({"deal": d, "hands": hands, "rule_bidder": rb, "rule_decl": rd, "rows": rows})).unwrap();
            }); } });
        }
        _ => eprintln!("usage: ladder log|label|bench|h2h|turbo-eval|log2|agree ..."),
    }
}
