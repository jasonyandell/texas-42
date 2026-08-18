//! EXPLORATORY LEVEL-1 SOLVER — sits below every evidentiary tier and is
//! cited by nothing above it (wiki/ideas tier; outputs are ESTIMATES).
//!
//! The scenario solver iterated once: the field seats stop being dice and
//! start being players. Each field seat plays the LEVEL-0 policy — the
//! lawful scenario player from its own chair: it sees its own hand and the
//! public record, believes the unseen tiles are dealt uniformly (sizes
//! consistent with the record; no void inference — a stated level-0
//! simplification), models every other seat as uniform dice, and picks the
//! play that maximizes P(T1 makes 30) if it sits on T1 or minimizes it if it
//! sits on T0. Ties break to the lowest domino index (stated, deterministic).
//!
//! S1 (the outer viewer) best-responds to that field, lawful perfect recall,
//! keyed on the public record and its own hand only — no fusion, no peeking.
//!
//! Because the level-0 policy is DETERMINISTIC, the field has no dice axis:
//! an outer scenario is a deal alone. When the boundary support is small
//! enough to enumerate (t=4: 1,200 worlds) the outer solve runs the FULL
//! support — exact best response to the level-0 field, no outer sampling at
//! all. The only approximation left is inside the level-0 seats themselves
//! (inner sample size n0, and their no-void belief).
//!
//! Belief-from-behavior falls out of the same machinery that handled dice:
//! at a field node the alive deals partition by what the policy ACTUALLY
//! plays in each of them — conditioning on policy-consistent deals is Bayes
//! on the sample. A partner's play is now evidence.
//!
//! Devices carried over unchanged (ruled SOUND for the exact solve;
//! exact-on-the-sample here): decided cutoffs, viewer early exit (at 1 for a
//! maximizer, at 0 for a minimizer), pmake key reduction, interned alive
//! sets.
//!
//! Arithmetic: integer counts and BigRational values. No floats.
//!
//! This is NOT the freeze-57 M3 gate; nothing here is quotable above
//! exploratory tier; not a P-A21 statement.

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

/// Frozen outer-deal seed (distinct from the ladder's and the scenario
/// solver's streams). A run is a deterministic function of
/// (t, n_outer, n0, this seed) — a probe-internal determinism freeze.
const OUTER_SEED: u64 = 0x8CB9_2BA7_2F3D_8DD7;

/// Frozen seed for level-0 inner sampling (mixed with seat, hand, record).
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

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

/// SplitMix64 finalizer as a one-shot mixer.
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

/// One void-consistent deal drawn uniformly by shuffle-and-reject.
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

/// Deterministic hash of the reduced public record.
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

/// How the field seats behave inside a solver.
enum FieldModel {
    /// Uniform dice, one frozen draw per (scenario, record) — level 0's model
    /// of everyone else.
    Dice,
    /// Every field seat plays the level-0 policy (deterministic).
    Policy,
}

/// Cache key for a level-0 policy evaluation: the seat's entire information
/// state (its chair, its remaining hand, the reduced public record).
#[derive(Clone, PartialEq, Eq, Hash)]
struct PiKey {
    seat: u8,
    hand: u32,
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    // Banked totals are part of the mind's information state (the pmake
    // objective conditions on them; not derivable from the reduced record).
    // Omitting them aliased policies first-come-wins — fixed 2026-08-18
    // after the 3x384 pool closed; see level2.rs PiKey doc.
    banked_t1: u8,
    banked_t0: u8,
}

struct Solver {
    dcl: Decl,
    viewer: Seat,
    /// Viewer's hand at solver start (tiles it still held then).
    viewer_hand0: u32,
    /// true: viewer maximizes P(T1 makes); false: viewer minimizes it.
    maximize: bool,
    worlds: Vec<[u32; 4]>,
    /// Per-scenario dice seeds (Dice mode only; empty in Policy mode).
    seeds: Vec<u64>,
    field_model: FieldModel,
    /// Policy mode only: boundary played-mask and per-seat boundary hand size
    /// (for deriving mid-record hand sizes), inner sample size, pi0 cache.
    boundary_played: u32,
    boundary_hand_size: usize,
    n0: usize,
    pi_cache: HashMap<PiKey, u8>,
    pi_calls: u64,
    interned: Vec<Alive>,
    intern_map: HashMap<Alive, u32>,
    memo: HashMap<Key, BigRational>,
    nodes: u64,
    deadline: Instant,
    dead: bool,
}

impl Solver {
    #[allow(clippy::too_many_arguments)]
    fn new(
        dcl: Decl,
        viewer: Seat,
        viewer_hand0: u32,
        maximize: bool,
        worlds: Vec<[u32; 4]>,
        seeds: Vec<u64>,
        field_model: FieldModel,
        boundary_played: u32,
        boundary_hand_size: usize,
        n0: usize,
        deadline: Instant,
    ) -> Self {
        let all: Alive = Rc::new((0..worlds.len() as u32).collect());
        let mut intern_map = HashMap::new();
        intern_map.insert(Rc::clone(&all), 0u32);
        Solver {
            dcl,
            viewer,
            viewer_hand0,
            maximize,
            worlds,
            seeds,
            field_model,
            boundary_played,
            boundary_hand_size,
            n0,
            pi_cache: HashMap::new(),
            pi_calls: 0,
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

    /// Returns P(T1 makes 30) at this information state under this solver's
    /// treatment, or None if the wall-clock budget died mid-solve.
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
                "progress: nodes={} memo={} alive-sets={} pi0-calls={}",
                self.nodes,
                self.memo.len(),
                self.interned.len(),
                self.pi_calls
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
        let val = if seat == self.viewer {
            self.solve_viewer(key, led)?
        } else {
            match self.field_model {
                FieldModel::Dice => self.solve_field_dice(key, seat, led)?,
                FieldModel::Policy => self.solve_field_policy(key, seat, led)?,
            }
        };
        self.memo.insert(key.clone(), val.clone());
        Some(val)
    }

    /// The viewer decides on the public record and own hand; the alive set is
    /// unchanged. A maximizer stops at 1, a minimizer at 0.
    fn solve_viewer(&mut self, key: &Key, led: Option<Context>) -> Option<BigRational> {
        let hand = self.viewer_hand0 & !key.played;
        let legal = legal_plays(self.dcl, set_of(hand), led);
        let mut best: Option<BigRational> = None;
        for tile in legal.iter() {
            let child = self.child_after_play(key, tile, key.alive);
            let v = self.solve(&child)?;
            let better = best
                .as_ref()
                .is_none_or(|b| if self.maximize { v > *b } else { v < *b });
            if better {
                let decided = if self.maximize {
                    v.is_one()
                } else {
                    v.is_zero()
                };
                best = Some(v);
                if decided {
                    break;
                }
            }
        }
        Some(best.expect("viewer always has a legal play"))
    }

    /// Dice field: a frozen hash of (scenario seed, reduced record) draws
    /// uniformly from that scenario's legal set.
    fn solve_field_dice(
        &mut self,
        key: &Key,
        seat: Seat,
        led: Option<Context>,
    ) -> Option<BigRational> {
        let alive = Rc::clone(&self.interned[key.alive as usize]);
        let rh = record_hash(key);
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 28];
        for &sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            let idx =
                SplitMix64(self.seeds[sid as usize] ^ rh).below(u64::from(lm.count_ones())) as u32;
            buckets[nth_set_bit(lm, idx) as usize].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    /// Policy field: each alive deal's seat plays the level-0 policy — a
    /// deterministic function of (seat, its hand, the reduced record).
    fn solve_field_policy(
        &mut self,
        key: &Key,
        seat: Seat,
        led: Option<Context>,
    ) -> Option<BigRational> {
        let alive = Rc::clone(&self.interned[key.alive as usize]);
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 28];
        for &sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            let tile = if lm.count_ones() == 1 {
                lm.trailing_zeros() as u8
            } else {
                self.pi0(key, seat, hand, lm)?
            };
            buckets[usize::from(tile)].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    fn combine_buckets(
        &mut self,
        key: &Key,
        alive_len: usize,
        buckets: Vec<Vec<u32>>,
    ) -> Option<BigRational> {
        let denom = BigInt::from(alive_len);
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
        assert_eq!(redistributed, alive_len, "field partition conservation");
        Some(total)
    }

    /// Per-seat remaining hand sizes at a record (derivable from the reduced
    /// key: completed tricks since the boundary plus who has played in the
    /// current trick).
    fn hand_sizes_at(&self, key: &Key) -> [usize; 4] {
        let played_since = (key.played.count_ones() - self.boundary_played.count_ones()) as usize
            - key.plays.len();
        assert_eq!(played_since % 4, 0, "completed tricks are whole");
        let completed = played_since / 4;
        let mut sizes = [self.boundary_hand_size - completed; 4];
        for i in 0..key.plays.len() {
            sizes[(usize::from(key.leader) + i) % 4] -= 1;
        }
        sizes
    }

    /// The level-0 policy: seat looks at (own hand, reduced record), believes
    /// the unseen tiles are dealt uniformly at the record's hand sizes (no
    /// void inference — stated level-0 simplification), models every other
    /// seat as dice, and max/minimizes P(T1 makes) by team. Ties break to the
    /// lowest domino index. Deterministic; cached by full information state.
    fn pi0(&mut self, key: &Key, seat: Seat, hand: u32, legal_mask: u32) -> Option<u8> {
        let pk = PiKey {
            seat: seat.index() as u8,
            hand,
            played: key.played,
            leader: key.leader,
            plays: key.plays.clone(),
            banked_t1: key.banked_t1,
            banked_t0: key.banked_t0,
        };
        if let Some(&t) = self.pi_cache.get(&pk) {
            return Some(t);
        }
        self.pi_calls += 1;
        let sizes = self.hand_sizes_at(key);
        let unseen = FULL_MASK & !key.played & !hand;
        let others: Vec<usize> = (0..4).filter(|&s| s != seat.index()).collect();
        let need: usize = others.iter().map(|&s| sizes[s]).sum();
        assert_eq!(unseen.count_ones() as usize, need, "unseen tiles fit sizes");
        let mut rng =
            SplitMix64(INNER_SEED ^ mix(pk.seat as u64) ^ mix(u64::from(hand)) ^ record_hash(key));
        let mut tiles = mask_bits(unseen);
        let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
        let mut inner_worlds: Vec<[u32; 4]> = Vec::with_capacity(self.n0);
        for _ in 0..self.n0 {
            for i in (1..tiles.len()).rev() {
                let j = rng.below((i + 1) as u64) as usize;
                tiles.swap(i, j);
            }
            let mut w = [0u32; 4];
            w[seat.index()] = hand;
            let mut off = 0;
            for &s in &others {
                w[s] = mask_slice(&tiles[off..off + sizes[s]]);
                off += sizes[s];
            }
            inner_worlds.push(w);
        }
        let inner_seeds: Vec<u64> = (0..self.n0).map(|_| rng.next_u64()).collect();
        let maximize = seat.team() == Team::T1;
        let mut inner = Solver::new(
            self.dcl,
            seat,
            hand,
            maximize,
            inner_worlds,
            inner_seeds,
            FieldModel::Dice,
            self.boundary_played,
            self.boundary_hand_size,
            0,
            self.deadline,
        );
        let root = Key {
            played: key.played,
            leader: key.leader,
            plays: key.plays.clone(),
            banked_t1: key.banked_t1,
            banked_t0: key.banked_t0,
            alive: 0,
        };
        let mut best: Option<(BigRational, u8)> = None;
        let mut lm = legal_mask;
        while lm != 0 {
            let tile_idx = lm.trailing_zeros() as u8;
            lm &= lm - 1;
            let tile = Domino::from_index(usize::from(tile_idx)).expect("tile < 28");
            let child = inner.child_after_play(&root, tile, 0);
            let v = inner.solve(&child)?;
            let better = best
                .as_ref()
                .is_none_or(|(b, _)| if maximize { v > *b } else { v < *b });
            if better {
                best = Some((v, tile_idx));
            }
        }
        self.nodes += inner.nodes;
        let (_, choice) = best.expect("legal play exists");
        self.pi_cache.insert(pk, choice);
        Some(choice)
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

fn run_boundary(dcl: Decl, t: usize, budget_secs: u64, n_outer: usize, n0: usize) {
    println!(
        "== boundary t={t}: LEVEL-1 solve from the start of trick {t}, n_outer={n_outer}, n0={n0}"
    );
    let b = build_boundary(dcl, t);
    assert_eq!(b.t, t, "boundary matches the requested trick");
    let start = Instant::now();
    let mut rng = SplitMix64(OUTER_SEED ^ t as u64);
    let mut exact_support = false;
    let outer_worlds: Vec<[u32; 4]> = match enumerate_support(&b) {
        Ok(support) => {
            if n_outer >= support.len() {
                exact_support = true;
                println!(
                    "  support: {} void-consistent deals — running ALL of them (no outer sampling; the field policy is deterministic, so there is no dice axis either)",
                    support.len()
                );
                support
            } else {
                println!(
                    "  support: {} void-consistent deals; drawing {} with replacement",
                    support.len(),
                    n_outer
                );
                (0..n_outer)
                    .map(|_| support[rng.below(support.len() as u64) as usize])
                    .collect()
            }
        }
        Err(raw) => {
            println!(
                "  support: {raw} raw assignments (over the {SUPPORT_CAP}-world cap); drawing {n_outer} deals directly"
            );
            let mut tiles = mask_bits(b.pool);
            (0..n_outer)
                .map(|_| draw_world(&b, &mut tiles, &mut rng))
                .collect()
        }
    };
    if exact_support {
        println!(
            "  level-1 values are EXACT over the support GIVEN the level-0 field policy; the only approximation is inside level-0 seats (n0={n0}, no-void inner belief)"
        );
    } else {
        println!(
            "  SAMPLED DEALS: seed=0x{:016X} — values are ESTIMATES",
            OUTER_SEED ^ t as u64
        );
    }
    let deadline = start + std::time::Duration::from_secs(budget_secs);
    let n_deals = outer_worlds.len();
    let mut solver = Solver::new(
        dcl,
        VIEWER,
        b.s1_initial & !b.played,
        true,
        outer_worlds,
        Vec::new(),
        FieldModel::Policy,
        b.played,
        b.hand_size,
        n0,
        deadline,
    );
    let root = Key {
        played: b.played,
        leader: b.leader,
        plays: Vec::new(),
        banked_t1: b.banked_t1,
        banked_t0: b.banked_t0,
        alive: 0,
    };
    println!(
        "  root: leader S{}, banked T1={} T0={}, {} deals, budget {}s",
        b.leader, b.banked_t1, b.banked_t0, n_deals, budget_secs
    );
    if usize::from(b.leader) == VIEWER.index() {
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
            "  level-1 pmake play at trick {t}: {}{}",
            ties.join(" / "),
            if ties.len() > 1 { "  (tie)" } else { "" }
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
        "  stats: {} nodes (incl. inner), {} memo entries, {} alive-sets, {} pi0 evaluations, {}.{:03}s",
        solver.nodes,
        solver.memo.len(),
        solver.interned.len(),
        solver.pi_calls,
        ms / 1000,
        ms % 1000
    );
    println!();
}

fn report_death(solver: &Solver, t: usize, start: Instant) {
    let ms = start.elapsed().as_millis();
    println!(
        "  DIED at t={t}: wall-clock budget exceeded after {} nodes, {} memo entries, {} pi0 evaluations, {}.{:03}s",
        solver.nodes,
        solver.memo.len(),
        solver.pi_calls,
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
        .unwrap_or(300);
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer deal count"))
        .unwrap_or(2000);
    let n0: usize = args
        .get(4)
        .map(|s| s.parse().expect("inner sample size"))
        .unwrap_or(64);

    println!("walt-m3-probe level-1 solver: EXPLORATORY iterated scenario player");
    println!(
        "hand 8, trump fives, P30 by T1; field seats play the level-0 policy (T1 max / T0 min pmake,"
    );
    println!(
        "uniform no-void inner belief, dice model of others, lowest-index tie break); S1 best-responds"
    );
    println!("S1 keyed on public record + own hand only — no fusion; alive-set partition = Bayes on policy-consistent deals");
    println!();

    match only_t {
        Some(t) => run_boundary(dcl, t, budget_secs, n_outer, n0),
        None => {
            for t in (1..=4).rev() {
                run_boundary(dcl, t, budget_secs, n_outer, n0);
            }
        }
    }
    println!(
        "nothing above exploratory tier; level-1 values are never receipts; not a P-A21 statement"
    );
}
