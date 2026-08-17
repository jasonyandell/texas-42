//! EXPLORATORY SCENARIO SOLVER — sits below every evidentiary tier and is
//! cited by nothing above it (wiki/ideas tier; sampled outputs are ESTIMATES,
//! below even probe numbers).
//!
//! External sampling for the lawful pmake solve of receipt hand 8: where the
//! ladder samples only the DEAL (and then takes the exact expectation over
//! every field reply — the 28-play public tree that killed t<=2), this binary
//! samples BOTH random axes of the H treatment:
//!
//!   scenario = (deal drawn uniform from the void-consistent support,
//!               a frozen per-scenario seed for the field's uniform draws)
//!
//! Given a scenario, every field play is a deterministic function of the
//! public record (a hash of scenario seed and record picks uniformly from
//! that seat's legal set). The solve is then exact ON THE SAMPLE:
//!   - S1 nodes are keyed by the public record and S1's hand ONLY — every
//!     scenario consistent with the record shares one decision. No strategy
//!     fusion: the player never peeks at a world.
//!   - field nodes partition the alive scenarios by their determined play;
//!     the branch weight is the sample frequency (no 1/k reweighting).
//!
//! The reachable public tree collapses from ~10^9 nodes (measured, killed
//! t<=2) to roughly n_scenarios x S1's own choice paths, heavily shared.
//!
//! Estimator caveats (stated, not hidden):
//!   - values are sample averages: unbiased per fixed S1 strategy, and the
//!     root max over noisy estimates is optimistically biased, exactly as in
//!     the ladder's sampled runs — trust plays, grade values;
//!   - the field draw is keyed on the REDUCED record (played, leader,
//!     in-trick plays); two play orders reaching the same reduction share the
//!     draw — a common-random-numbers artifact, marginally uniform at every
//!     node, pseudo-independent along any single line of play (nodes on a
//!     path are always distinct);
//!   - deals are drawn WITH replacement (i.i.d. scenarios).
//!
//! Devices carried over from the ladder unchanged (ruled SOUND by
//! walt-math-12 for the exact solve; they are exact-on-the-sample here):
//! decided cutoffs (banked_T1 >= 30 => 1, banked_T0 > 12 => 0), viewer early
//! exit at value 1, pmake key reduction. Posterior interning degenerates to
//! interning alive scenario-id sets (all weights 1).
//!
//! Arithmetic: integer counts and BigRational values. No floats.
//!
//! This is NOT the freeze-57 M3 gate; nothing here is quotable above
//! exploratory tier; a completed t=1 run is a probe output, not a P-A21
//! statement.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt_m3_carrier::VIEWER;

/// All 28 tiles.
const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Refuse to materialize a support larger than this (worlds, upper bound).
const SUPPORT_CAP: u128 = 30_000_000;

/// Frozen scenario seed (distinct from the ladder's SAMPLE_SEED so scenario
/// runs and world-sample runs are visibly different frozen streams). A run is
/// a deterministic function of (t, n, this seed) — a probe-internal
/// determinism freeze, not an ingest number.
const SCENARIO_SEED: u64 = 0xD1B5_4A32_D192_ED03;

/// SplitMix64: integer-only deterministic PRNG.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Unbiased uniform draw in 0..n via rejection.
    fn below(&mut self, n: u64) -> u64 {
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next_u64();
            if v < zone {
                return v % n;
            }
        }
    }
}

/// SplitMix64 finalizer as a one-shot mixer for record hashing.
fn mix(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn d(hi: u8, lo: u8) -> Domino {
    Domino::new(Pip::new(hi).expect("hi pip"), Pip::new(lo).expect("lo pip"))
}

/// Trump fives: the carrier declaration (receipt hand 8, "called P5").
fn decl() -> Decl {
    Decl::PipTrump(Pip::new(5).expect("pip 5 exists"))
}

fn bit(dm: Domino) -> u32 {
    1u32 << dm.index()
}

fn mask_of(set: DominoSet) -> u32 {
    let mut m = 0u32;
    for dm in set.iter() {
        m |= bit(dm);
    }
    m
}

fn set_of(mask: u32) -> DominoSet {
    let mut s = DominoSet::default();
    let mut m = mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        s.insert(Domino::from_index(i).expect("index < 28"));
        m &= m - 1;
    }
    s
}

fn binom(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let mut acc: u128 = 1;
    for i in 0..k {
        acc = acc * (n - i) / (i + 1);
    }
    acc
}

/// The three public prefix tricks of receipt hand 8, plays in seat order from
/// each trick's leader (S1 leads trick 1 as bidder; later leaders replayed).
fn prefix_tricks() -> [[Domino; 4]; 3] {
    [
        [d(5, 2), d(6, 3), d(5, 1), d(6, 5)],
        [d(4, 2), d(5, 4), d(4, 4), d(3, 2)],
        [d(1, 1), d(2, 2), d(1, 0), d(6, 1)],
    ]
}

/// S1's full initial hand: prefix plays 52/54/11 plus the frozen trick-4 hand.
fn s1_initial_mask() -> u32 {
    [
        d(5, 2),
        d(5, 4),
        d(1, 1),
        d(2, 1),
        d(3, 1),
        d(3, 3),
        d(5, 5),
    ]
    .into_iter()
    .map(bit)
    .fold(0, |a, b| a | b)
}

/// Public state of receipt hand 8 at the start of trick `t` (t in 1..=4).
struct Boundary {
    t: usize,
    leader: u8,
    played: u32,
    banked_t1: u8,
    banked_t0: u8,
    s1_initial: u32,
    pool: u32,
    hand_size: usize,
    /// Per-seat mask of tiles the seat is known (by replay) not to hold.
    voids: [u32; 4],
}

fn build_boundary(dcl: Decl, t: usize) -> Boundary {
    assert!((1..=4).contains(&t));
    let s1_initial = s1_initial_mask();
    let mut played = 0u32;
    let mut banked_t1 = 0u8;
    let mut banked_t0 = 0u8;
    let mut leader = VIEWER.index() as u8;
    let mut voids = [0u32; 4];
    for trick_doms in prefix_tricks().iter().take(t - 1) {
        let lead_seat = Seat::from_index(usize::from(leader)).expect("leader");
        let trick = Trick::new(lead_seat, *trick_doms).expect("distinct prefix tiles");
        let ctx = trick.led(dcl);
        let ctx_mask = mask_of(dcl.effective_incidence(ctx));
        for (k, (seat, tile)) in trick.plays().into_iter().enumerate() {
            assert_eq!(bit(tile) & voids[seat.index()], 0, "void-consistent prefix");
            if k > 0 && !dcl.follows(tile, ctx) {
                voids[seat.index()] |= ctx_mask;
            }
            played |= bit(tile);
        }
        let winner = trick.winner(dcl);
        let value = trick.points() as u8;
        if winner.team() == Team::T1 {
            banked_t1 += value;
        } else {
            banked_t0 += value;
        }
        leader = winner.index() as u8;
    }
    let pool = FULL_MASK & !played & !s1_initial;
    let hand_size = 8 - t;
    assert_eq!(pool.count_ones() as usize, 3 * hand_size, "pool tiles");
    assert_eq!(
        (s1_initial & !played).count_ones() as usize,
        hand_size,
        "every seat holds 8 - t tiles at the boundary"
    );
    Boundary {
        t,
        leader,
        played,
        banked_t1,
        banked_t0,
        s1_initial,
        pool,
        hand_size,
        voids,
    }
}

fn mask_bits(mask: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(mask.count_ones() as usize);
    let mut m = mask;
    while m != 0 {
        v.push(m.trailing_zeros() as u8);
        m &= m - 1;
    }
    v
}

fn combos(items: &[u8], k: usize, f: &mut impl FnMut(u32)) {
    fn rec(items: &[u8], k: usize, start: usize, acc: u32, f: &mut impl FnMut(u32)) {
        if k == 0 {
            f(acc);
            return;
        }
        let mut i = start;
        while i + k <= items.len() {
            rec(items, k - 1, i + 1, acc | (1u32 << items[i]), f);
            i += 1;
        }
    }
    rec(items, k, 0, 0, f);
}

/// Enumerates the void-consistent support at the boundary, or returns the raw
/// assignment count if it exceeds the materialization cap.
fn enumerate_support(b: &Boundary) -> Result<Vec<[u32; 4]>, u128> {
    let n = u128::from(b.pool.count_ones());
    let hs = b.hand_size as u128;
    let raw = binom(n, hs) * binom(n - hs, hs);
    if raw > SUPPORT_CAP {
        return Err(raw);
    }
    let s1_now = b.s1_initial & !b.played;
    let s2_allowed = mask_bits(b.pool & !b.voids[2]);
    let mut out: Vec<[u32; 4]> = Vec::new();
    combos(&s2_allowed, b.hand_size, &mut |s2_mask| {
        let rem = b.pool & !s2_mask;
        let s3_allowed = mask_bits(rem & !b.voids[3]);
        combos(&s3_allowed, b.hand_size, &mut |s3_mask| {
            let s0_mask = rem & !s3_mask;
            if s0_mask & b.voids[0] == 0 {
                out.push([s0_mask, s1_now, s2_mask, s3_mask]);
            }
        });
    });
    Ok(out)
}

/// One void-consistent deal drawn uniformly by shuffle-and-reject (used when
/// the support is too large to materialize; at t=1 there are no voids and
/// every shuffle is accepted).
fn draw_world(b: &Boundary, tiles: &mut [u8], rng: &mut SplitMix64) -> [u32; 4] {
    let s1_now = b.s1_initial & !b.played;
    let hs = b.hand_size;
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    loop {
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let s2 = mask_slice(&tiles[0..hs]);
        let s3 = mask_slice(&tiles[hs..2 * hs]);
        let s0 = mask_slice(&tiles[2 * hs..3 * hs]);
        if s2 & b.voids[2] != 0 || s3 & b.voids[3] != 0 || s0 & b.voids[0] != 0 {
            continue;
        }
        return [s0, s1_now, s2, s3];
    }
}

/// Interned alive set: ascending scenario ordinals, all weight 1.
type Alive = Rc<Vec<u32>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    /// Interned alive-scenario-set id — part of the information state.
    alive: u32,
}

/// Deterministic hash of the reduced public record (the field-draw key).
fn record_hash(key: &Key) -> u64 {
    let mut h = mix(u64::from(key.played));
    h = mix(h ^ (u64::from(key.leader) << 32));
    for &p in &key.plays {
        h = mix(h ^ (0x100 | u64::from(p)));
    }
    h
}

fn nth_set_bit(mask: u32, n: u32) -> u32 {
    let mut m = mask;
    for _ in 0..n {
        m &= m - 1;
    }
    m.trailing_zeros()
}

struct Solver<'a> {
    dcl: Decl,
    b: &'a Boundary,
    scen_worlds: &'a [[u32; 4]],
    scen_seeds: &'a [u64],
    interned: Vec<Alive>,
    intern_map: HashMap<Alive, u32>,
    memo: HashMap<Key, BigRational>,
    nodes: u64,
    deadline: Instant,
    dead: bool,
}

impl<'a> Solver<'a> {
    fn new(
        dcl: Decl,
        b: &'a Boundary,
        scen_worlds: &'a [[u32; 4]],
        scen_seeds: &'a [u64],
        deadline: Instant,
    ) -> Self {
        let all: Alive = Rc::new((0..scen_worlds.len() as u32).collect());
        let mut intern_map = HashMap::new();
        intern_map.insert(Rc::clone(&all), 0u32);
        Solver {
            dcl,
            b,
            scen_worlds,
            scen_seeds,
            interned: vec![all],
            intern_map,
            memo: HashMap::new(),
            nodes: 0,
            deadline,
            dead: false,
        }
    }

    fn intern(&mut self, v: Vec<u32>) -> u32 {
        let rc: Alive = Rc::new(v);
        if let Some(&id) = self.intern_map.get(&rc) {
            return id;
        }
        let id = self.interned.len() as u32;
        self.interned.push(Rc::clone(&rc));
        self.intern_map.insert(rc, id);
        id
    }

    fn child_after_play(&self, key: &Key, tile: Domino, alive: u32) -> Key {
        let mut plays = key.plays.clone();
        plays.push(tile.index() as u8);
        let played = key.played | bit(tile);
        if plays.len() == 4 {
            let doms = [
                Domino::from_index(usize::from(plays[0])).expect("p0"),
                Domino::from_index(usize::from(plays[1])).expect("p1"),
                Domino::from_index(usize::from(plays[2])).expect("p2"),
                Domino::from_index(usize::from(plays[3])).expect("p3"),
            ];
            let leader = Seat::from_index(usize::from(key.leader)).expect("leader");
            let trick = Trick::new(leader, doms).expect("distinct tiles in trick");
            let winner = trick.winner(self.dcl);
            let value = trick.points() as u8;
            let t1_won = winner.team() == Team::T1;
            Key {
                played,
                leader: winner.index() as u8,
                plays: Vec::new(),
                banked_t1: key.banked_t1 + if t1_won { value } else { 0 },
                banked_t0: key.banked_t0 + if t1_won { 0 } else { value },
                alive,
            }
        } else {
            Key {
                played,
                leader: key.leader,
                plays,
                banked_t1: key.banked_t1,
                banked_t0: key.banked_t0,
                alive,
            }
        }
    }

    /// Returns the on-sample P30-make probability at this information state,
    /// or None if the wall-clock budget died mid-solve.
    fn solve(&mut self, key: &Key) -> Option<BigRational> {
        if self.dead {
            return None;
        }
        self.nodes += 1;
        if self.nodes & 0xFFFF == 0 && Instant::now() >= self.deadline {
            self.dead = true;
            return None;
        }
        if self.nodes.is_multiple_of(20_000_000) {
            eprintln!(
                "progress: t={} nodes={} memo={} alive-sets={}",
                self.b.t,
                self.nodes,
                self.memo.len(),
                self.interned.len()
            );
        }
        // Decided cutoffs: banking is monotone and the hand totals 42.
        if key.banked_t1 >= 30 {
            return Some(BigRational::one());
        }
        if key.banked_t0 > 12 {
            return Some(BigRational::zero());
        }
        if let Some(v) = self.memo.get(key) {
            return Some(v.clone());
        }
        assert_ne!(key.played, FULL_MASK, "terminal states are always decided");
        let seat =
            Seat::from_index((usize::from(key.leader) + key.plays.len()) % 4).expect("seat index");
        let led: Option<Context> = key.plays.first().map(|&i| {
            self.dcl
                .led_context(Domino::from_index(usize::from(i)).expect("led index"))
        });
        let val = if seat == VIEWER {
            self.solve_viewer(key, led)?
        } else {
            self.solve_field(key, seat, led)?
        };
        self.memo.insert(key.clone(), val.clone());
        Some(val)
    }

    /// S1 decides on the public record and own hand; the alive set is
    /// unchanged (S1's play carries no evidence about hidden hands).
    fn solve_viewer(&mut self, key: &Key, led: Option<Context>) -> Option<BigRational> {
        let hand = self.b.s1_initial & !key.played;
        let legal = legal_plays(self.dcl, set_of(hand), led);
        let mut best: Option<BigRational> = None;
        for tile in legal.iter() {
            let child = self.child_after_play(key, tile, key.alive);
            let v = self.solve(&child)?;
            let better = best.as_ref().is_none_or(|b| v > *b);
            if better {
                let is_one = v.is_one();
                best = Some(v);
                if is_one {
                    // Certain make on the sample: no sibling can improve.
                    break;
                }
            }
        }
        Some(best.expect("viewer always has a legal play"))
    }

    /// A field seat's play is determined per scenario: a frozen hash of
    /// (scenario seed, reduced record) draws uniformly from that scenario's
    /// legal set. Alive scenarios partition by the drawn tile; branch weight
    /// is the sample frequency.
    fn solve_field(&mut self, key: &Key, seat: Seat, led: Option<Context>) -> Option<BigRational> {
        let alive = Rc::clone(&self.interned[key.alive as usize]);
        let rh = record_hash(key);
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 28];
        for &sid in alive.iter() {
            let hand = self.scen_worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            let idx = SplitMix64(self.scen_seeds[sid as usize] ^ rh)
                .below(u64::from(lm.count_ones())) as u32;
            buckets[nth_set_bit(lm, idx) as usize].push(sid);
        }
        let denom = BigInt::from(alive.len());
        let mut total = BigRational::zero();
        let mut redistributed: usize = 0;
        for (tile, bucket) in buckets.into_iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }
            redistributed += bucket.len();
            let reach = BigInt::from(bucket.len());
            let child_alive = self.intern(bucket);
            let child = self.child_after_play(
                key,
                Domino::from_index(tile).expect("tile < 28"),
                child_alive,
            );
            let v = self.solve(&child)?;
            if !v.is_zero() {
                total += v * BigRational::new(reach, denom.clone());
            }
        }
        // Partition receipt: every alive scenario lands in exactly one bucket.
        assert_eq!(redistributed, alive.len(), "field partition conservation");
        Some(total)
    }
}

fn show(v: &BigRational) -> String {
    let bp = (v * BigRational::from_integer(BigInt::from(10_000)))
        .to_integer()
        .to_string();
    let bp: u32 = bp.parse().unwrap_or(0);
    format!(
        "{}/{} (~{}.{:02}%)",
        v.numer(),
        v.denom(),
        bp / 100,
        bp % 100
    )
}

fn run_boundary(dcl: Decl, t: usize, budget_secs: u64, n_scenarios: usize) {
    println!("== boundary t={t}: scenario solve from the start of trick {t}, n={n_scenarios}");
    let b = build_boundary(dcl, t);
    let start = Instant::now();
    let seed = SCENARIO_SEED ^ t as u64;
    let mut rng = SplitMix64(seed);
    let scen_worlds: Vec<[u32; 4]> = match enumerate_support(&b) {
        Ok(support) => {
            println!(
                "  support: {} void-consistent worlds; drawing {} deals with replacement",
                support.len(),
                n_scenarios
            );
            (0..n_scenarios)
                .map(|_| support[rng.below(support.len() as u64) as usize])
                .collect()
        }
        Err(raw) => {
            println!(
                "  support: {raw} raw assignments (over the {SUPPORT_CAP}-world cap); drawing {n_scenarios} deals directly"
            );
            let mut tiles = mask_bits(b.pool);
            (0..n_scenarios)
                .map(|_| draw_world(&b, &mut tiles, &mut rng))
                .collect()
        }
    };
    let scen_seeds: Vec<u64> = (0..n_scenarios).map(|_| rng.next_u64()).collect();
    println!(
        "  SAMPLED SCENARIOS (deal + field dice): seed=0x{seed:016X} — all values below are ESTIMATES, not exact"
    );
    let deadline = start + std::time::Duration::from_secs(budget_secs);
    let mut solver = Solver::new(dcl, &b, &scen_worlds, &scen_seeds, deadline);
    let root = Key {
        played: b.played,
        leader: b.leader,
        plays: Vec::new(),
        banked_t1: b.banked_t1,
        banked_t0: b.banked_t0,
        alive: 0,
    };
    println!(
        "  root: leader S{}, banked T1={} T0={}, budget {}s",
        b.leader, b.banked_t1, b.banked_t0, budget_secs
    );
    if usize::from(b.leader) == VIEWER.index() {
        // S1 leads: report the on-sample value of every legal lead + argmax.
        let hand = b.s1_initial & !b.played;
        let legal = legal_plays(dcl, set_of(hand), None);
        let mut values: Vec<(Domino, BigRational)> = Vec::new();
        for tile in legal.iter() {
            let child = solver.child_after_play(&root, tile, 0);
            match solver.solve(&child) {
                Some(v) => values.push((tile, v)),
                None => {
                    report_death(&solver, t, start);
                    return;
                }
            }
        }
        let best = values
            .iter()
            .map(|(_, v)| v.clone())
            .max()
            .expect("legal leads");
        for (tile, v) in &values {
            println!(
                "  lead {}{}  P(make) = {}{}",
                tile.hi().value(),
                tile.lo().value(),
                show(v),
                if *v == best { "   <-- opt" } else { "" }
            );
        }
        let ties: Vec<String> = values
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(tile, _)| format!("{}{}", tile.hi().value(), tile.lo().value()))
            .collect();
        println!(
            "  scenario pmake play at trick {t}: {}{}",
            ties.join(" / "),
            if ties.len() > 1 {
                "  (tie on sample)"
            } else {
                ""
            }
        );
    } else {
        match solver.solve(&root) {
            Some(v) => println!("  P(make) before trick {t} = {}", show(&v)),
            None => {
                report_death(&solver, t, start);
                return;
            }
        }
    }
    let ms = start.elapsed().as_millis();
    println!(
        "  stats: {} nodes, {} memo entries, {} alive-sets, {}.{:03}s",
        solver.nodes,
        solver.memo.len(),
        solver.interned.len(),
        ms / 1000,
        ms % 1000
    );
    println!();
}

fn report_death(solver: &Solver, t: usize, start: Instant) {
    let ms = start.elapsed().as_millis();
    println!(
        "  DIED at t={t}: wall-clock budget exceeded after {} nodes, {} memo entries, {} alive-sets, {}.{:03}s",
        solver.nodes,
        solver.memo.len(),
        solver.interned.len(),
        ms / 1000,
        ms % 1000
    );
    println!();
}

fn main() {
    let dcl = decl();
    let args: Vec<String> = std::env::args().collect();
    let only_t: Option<usize> = args.get(1).map(|s| s.parse().expect("t in 1..=4"));
    let budget_secs: u64 = args
        .get(2)
        .map(|s| s.parse().expect("budget seconds"))
        .unwrap_or(600);
    let n_scenarios: usize = args
        .get(3)
        .map(|s| s.parse().expect("scenario count"))
        .unwrap_or(10_000);

    println!("walt-m3-probe scenario solver: EXPLORATORY external-sampling pmake estimates");
    println!(
        "hand 8, trump fives, P30 by T1; scenarios sample deal AND field dice; solve exact on-sample"
    );
    println!("treatment H only: S1 lawful perfect recall keyed on public record — no fusion");
    println!("tie rule: all argmax members reported; no convention selects among them silently");
    println!();

    match only_t {
        Some(t) => run_boundary(dcl, t, budget_secs, n_scenarios),
        None => {
            for t in (1..=4).rev() {
                run_boundary(dcl, t, budget_secs, n_scenarios);
            }
        }
    }
    println!(
        "nothing above exploratory tier; sampled values are never receipts; not a P-A21 statement"
    );
}
