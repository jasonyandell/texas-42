//! EXPLORATORY LADDER — sits below every evidentiary tier and is cited by
//! nothing above it (wiki/ideas tier; CLAUDE.md "Exploratory stays exploratory").
//!
//! Walks the exact lawful perfect-recall solve of receipt hand 8 from the
//! frozen trick-4 boundary (the M3 carrier) backward toward trick 1, one trick
//! boundary at a time, to find out empirically where exact enumeration dies.
//! Objective is P30 make only — the 2026-08-17 ruling: the game is making
//! your bid, and the Boolean payoff admits exact pruning.
//!
//! Boundary convention (matches the freeze-57 M3 contract at t=4): at trick
//! boundary t the posterior over hidden assignments is UNIFORM over the
//! void-consistent support. This is the frozen M3 treatment, not the
//! likelihood-weighted posterior of the random-field model over the prefix; at
//! t=1 there is no prefix and the two coincide exactly.
//!
//! Exactness-preserving devices (each argued in the accompanying walt-math
//! question, none yet ruled):
//!   - decided cutoffs: banked_T1 >= 30 => value 1; banked_T0 > 12 => value 0
//!     (banking is monotone, total is 42, so both are outcome-determined);
//!   - viewer early exit: a child worth 1 cannot be improved on, so sibling
//!     scans stop (max of values bounded above by 1);
//!   - gcd-normalized interned posteriors: the posterior enters the recursion
//!     only projectively (value depends on weight ratios), so dividing the
//!     integer weight vector by its gcd merges proportional posteriors;
//!   - adaptive per-node scale: at a field node each world's weight is scaled
//!     by L/k_w with L = lcm of the alive legal-set sizes, keeping every
//!     weight integral without a global 12^n scale.
//!
//! Arithmetic: integer weights (u128, overflow-checked in every profile) and
//! BigRational node values. No floats exist in this crate.
//!
//! This is NOT the freeze-57 M3 gate: no Metal parity, no receipt, no net
//! census. Values at t<4 are new exploratory numbers with no adjudication.
//! Nothing here is quotable above exploratory tier, and the t=1 attempt, if
//! it completes, is still not a P-A21 statement — it is a probe output.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt_m3_carrier::{M3Carrier, LEGAL_ROOT_MASK, RAW_RECEIPT_BYTES, SUPPORT_COUNT, VIEWER};

const RAW_RECEIPT: &[u8] = include_bytes!("../../../../rob/receipts/verify_player.txt");

/// All 28 tiles.
const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Refuse to materialize a support larger than this (worlds, upper bound).
const SUPPORT_CAP: u128 = 30_000_000;

/// Frozen sampling seed (arbitrary constant, the SplitMix64 gamma). A sampled
/// run is a deterministic function of (t, n, this seed) — same discipline as
/// FROZEN_WITH_VOIDS: a probe-internal determinism freeze, not an ingest
/// number. Sampled outputs are ESTIMATES and are labeled as such.
const SAMPLE_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

/// SplitMix64: integer-only deterministic PRNG for support sampling.
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

/// Uniform sample of n distinct worlds from a materialized support
/// (partial Fisher-Yates; deterministic under the frozen seed).
fn sample_worlds(mut worlds: Vec<[u32; 4]>, n: usize, rng: &mut SplitMix64) -> Vec<[u32; 4]> {
    if n >= worlds.len() {
        return worlds;
    }
    for i in 0..n {
        let j = i + rng.below((worlds.len() - i) as u64) as usize;
        worlds.swap(i, j);
    }
    worlds.truncate(n);
    worlds
}

/// Uniform sample of n distinct void-consistent worlds drawn directly (for
/// boundaries whose support is too large to materialize): shuffle the hidden
/// pool, deal hand_size/hand_size/hand_size to S2/S3/S0, reject void
/// violations and duplicates. Uniform over the void-consistent support.
fn generate_sample(b: &Boundary, n: usize, rng: &mut SplitMix64) -> Vec<[u32; 4]> {
    let s1_now = b.s1_initial & !b.played;
    let mut tiles = mask_bits(b.pool);
    let hs = b.hand_size;
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut out: Vec<[u32; 4]> = Vec::with_capacity(n);
    while out.len() < n {
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
        if !seen.insert((s2, s3)) {
            continue;
        }
        out.push([s0, s1_now, s2, s3]);
    }
    out
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

fn gcd(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
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
            // Replay consistency: no prefix play may violate a void already
            // established for that seat by an earlier prefix play.
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
    if t == 4 {
        assert_eq!(leader, VIEWER.index() as u8);
        assert_eq!((banked_t1, banked_t0), (7, 1), "frozen prefix bank");
        assert_eq!(s1_initial & !played, LEGAL_ROOT_MASK, "frozen S1 hand");
    }
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

/// Enumerates the void-consistent support at the boundary: hidden pool split
/// hand_size/hand_size/hand_size among S2, S3, S0. Returns None (with the raw
/// count) if the unfiltered assignment count exceeds the materialization cap.
fn enumerate_support(b: &Boundary) -> Result<Vec<[u32; 4]>, u128> {
    let n = u128::from(b.pool.count_ones());
    let hs = b.hand_size as u128;
    let raw = binom(n, hs) * binom(n - hs, hs);
    if raw > SUPPORT_CAP {
        return Err(raw);
    }
    let s1_now = b.s1_initial & !b.played;
    let pool_bits = mask_bits(b.pool);
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
    debug_assert!(pool_bits.len() == 3 * b.hand_size);
    Ok(out)
}

/// A gcd-normalized posterior: (support ordinal, integer weight), gcd 1.
type Alive = Rc<Vec<(u32, u128)>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    /// Interned posterior id — part of the information state.
    alive: u32,
}

struct Solver<'a> {
    dcl: Decl,
    b: &'a Boundary,
    worlds: &'a [[u32; 4]],
    interned: Vec<Alive>,
    masses: Vec<u128>,
    intern_map: HashMap<Alive, u32>,
    memo: HashMap<Key, BigRational>,
    nodes: u64,
    deadline: Instant,
    dead: bool,
}

impl<'a> Solver<'a> {
    fn new(dcl: Decl, b: &'a Boundary, worlds: &'a [[u32; 4]], deadline: Instant) -> Self {
        let uniform: Alive = Rc::new((0..worlds.len() as u32).map(|i| (i, 1u128)).collect());
        let mass = worlds.len() as u128;
        let mut intern_map = HashMap::new();
        intern_map.insert(Rc::clone(&uniform), 0u32);
        Solver {
            dcl,
            b,
            worlds,
            interned: vec![uniform],
            masses: vec![mass],
            intern_map,
            memo: HashMap::new(),
            nodes: 0,
            deadline,
            dead: false,
        }
    }

    fn intern(&mut self, mut v: Vec<(u32, u128)>) -> u32 {
        let mut g = 0u128;
        for &(_, w) in &v {
            g = gcd(g, w);
            if g == 1 {
                break;
            }
        }
        if g > 1 {
            for e in &mut v {
                e.1 /= g;
            }
        }
        let rc: Alive = Rc::new(v);
        if let Some(&id) = self.intern_map.get(&rc) {
            return id;
        }
        let id = self.interned.len() as u32;
        let mass: u128 = rc.iter().map(|e| e.1).sum();
        self.interned.push(Rc::clone(&rc));
        self.masses.push(mass);
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

    /// Returns the exact P30-make probability at this information state, or
    /// None if the wall-clock budget died mid-solve.
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
                "progress: t={} nodes={} memo={} posteriors={}",
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

    /// S1 decides on the public record and own hand; posterior is unchanged.
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
                    // Certain make: no sibling can improve on probability 1.
                    break;
                }
            }
        }
        Some(best.expect("viewer always has a legal play"))
    }

    /// A field seat plays uniformly from its per-world legal set. Weights are
    /// scaled by L/k_w (L = lcm of alive legal-set sizes) to stay integral.
    fn solve_field(&mut self, key: &Key, seat: Seat, led: Option<Context>) -> Option<BigRational> {
        let alive = Rc::clone(&self.interned[key.alive as usize]);
        let mass = self.masses[key.alive as usize];
        let mut legal_masks: Vec<u32> = Vec::with_capacity(alive.len());
        let mut scale: u128 = 1;
        for &(w_idx, _) in alive.iter() {
            let hand = self.worlds[w_idx as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            scale = lcm(scale, u128::from(lm.count_ones()));
            legal_masks.push(lm);
        }
        let mut buckets: Vec<Vec<(u32, u128)>> = vec![Vec::new(); 28];
        for (&(w_idx, weight), &lm) in alive.iter().zip(&legal_masks) {
            let scaled = weight * (scale / u128::from(lm.count_ones()));
            let mut m = lm;
            while m != 0 {
                let tile = m.trailing_zeros() as usize;
                buckets[tile].push((w_idx, scaled));
                m &= m - 1;
            }
        }
        let denom = BigInt::from(scale) * BigInt::from(mass);
        let mut total = BigRational::zero();
        let mut redistributed: u128 = 0;
        for (tile, bucket) in buckets.into_iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }
            let reach: u128 = bucket.iter().map(|e| e.1).sum();
            redistributed += reach;
            let child_alive = self.intern(bucket);
            let child = self.child_after_play(
                key,
                Domino::from_index(tile).expect("tile < 28"),
                child_alive,
            );
            let v = self.solve(&child)?;
            if !v.is_zero() {
                total += v * BigRational::new(BigInt::from(reach), denom.clone());
            }
        }
        // Mass receipt: every world redistributes exactly scale-times itself.
        assert_eq!(
            redistributed,
            scale.checked_mul(mass).expect("receipt product fits"),
            "field mass conservation"
        );
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

fn frozen_t4_expectations() -> Vec<(Domino, BigRational)> {
    let frac = |n: i64, dn: i64| BigRational::new(BigInt::from(n), BigInt::from(dn));
    vec![
        (d(2, 1), frac(5_204_203, 5_529_600)),
        (d(3, 1), frac(5_033_873, 5_529_600)),
        (d(3, 3), frac(16_078_667, 16_588_800)),
        (d(5, 5), frac(7_770_169, 8_294_400)),
    ]
}

fn run_boundary(
    dcl: Decl,
    t: usize,
    budget_secs: u64,
    sample: Option<usize>,
    carrier_worlds: &[[u32; 4]],
) {
    println!("== boundary t={t}: solve from the start of trick {t}");
    let b = build_boundary(dcl, t);
    let start = Instant::now();
    let seed = SAMPLE_SEED ^ t as u64;
    let mut rng = SplitMix64(seed);
    let mut sampled = false;
    let worlds = match enumerate_support(&b) {
        Ok(w) => {
            println!(
                "  support: {} void-consistent worlds (pool {} tiles, {} per hidden seat)",
                w.len(),
                b.pool.count_ones(),
                b.hand_size
            );
            match sample {
                Some(n) if n < w.len() => {
                    sampled = true;
                    sample_worlds(w, n, &mut rng)
                }
                _ => w,
            }
        }
        Err(raw) => match sample {
            Some(n) => {
                println!(
                    "  support: {raw} raw assignments (over the {SUPPORT_CAP}-world cap); drawing sample directly"
                );
                sampled = true;
                generate_sample(&b, n, &mut rng)
            }
            None => {
                println!("  support: {raw} raw assignments exceeds the {SUPPORT_CAP}-world cap");
                println!("  DIED at t={t}: support materialization (before any solving)");
                println!();
                return;
            }
        },
    };
    if sampled {
        println!(
            "  SAMPLED SUPPORT: n={} worlds, seed=0x{:016X} — all values below are ESTIMATES, not exact",
            worlds.len(),
            seed
        );
    }
    if t == 4 && !sampled {
        let mut mine = worlds.clone();
        let mut theirs = carrier_worlds.to_vec();
        mine.sort_unstable();
        theirs.sort_unstable();
        assert_eq!(mine, theirs, "t=4 support must equal the frozen M3 carrier");
        println!("  cross-check: support equals the frozen 1,200-world M3 carrier exactly");
    }
    let deadline = start + std::time::Duration::from_secs(budget_secs);
    let mut solver = Solver::new(dcl, &b, &worlds, deadline);
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
        // S1 leads: report the exact value of every legal lead plus argmax.
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
                if *v == best { "   <-- optimal" } else { "" }
            );
        }
        let ties: Vec<String> = values
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(tile, _)| format!("{}{}", tile.hi().value(), tile.lo().value()))
            .collect();
        println!(
            "  lawful pmake play at trick {t}: {}{}",
            ties.join(" / "),
            if ties.len() > 1 { "  (exact tie)" } else { "" }
        );
        if t == 4 && !sampled {
            for (tile, expected) in frozen_t4_expectations() {
                let got = &values
                    .iter()
                    .find(|(x, _)| *x == tile)
                    .expect("frozen lead present")
                    .1;
                assert_eq!(
                    got, &expected,
                    "t=4 must reproduce the frozen main-probe M3B fractions"
                );
            }
            println!("  cross-check: all four t=4 values equal the frozen first-play fractions");
        }
    } else {
        // A field seat leads trick t: report the exact value of the position.
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
        "  stats: {} nodes, {} memo entries, {} distinct posteriors, {}.{:03}s",
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
        "  DIED at t={t}: wall-clock budget exceeded after {} nodes, {} memo entries, {} distinct posteriors, {}.{:03}s",
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
    let sample: Option<usize> = args.get(3).map(|s| s.parse().expect("sample size"));

    assert_eq!(RAW_RECEIPT.len(), RAW_RECEIPT_BYTES);
    let carrier = M3Carrier::from_receipt_bytes(RAW_RECEIPT).expect("frozen carrier must admit");
    let records = carrier.support().records();
    assert_eq!(records.len(), SUPPORT_COUNT);
    let carrier_worlds: Vec<[u32; 4]> = records
        .iter()
        .map(|r| {
            let h = r.hands();
            [mask_of(h[0]), mask_of(h[1]), mask_of(h[2]), mask_of(h[3])]
        })
        .collect();

    println!("walt-m3-probe ladder: EXPLORATORY exact pmake walk toward trick 1");
    println!(
        "hand 8, trump fives, P30 by T1; boundary posterior uniform over support (M3 convention)"
    );
    println!("treatment H only: S1 lawful perfect recall, field uniform-random-legal");
    println!("tie rule: all argmax members reported; no convention selects among them silently");
    println!();

    match only_t {
        Some(t) => run_boundary(dcl, t, budget_secs, sample, &carrier_worlds),
        None => {
            for t in (1..=4).rev() {
                run_boundary(dcl, t, budget_secs, sample, &carrier_worlds);
            }
        }
    }
    println!(
        "nothing above exploratory tier; not a trick-1 statement (P-A21); no receipt is emitted"
    );
}
