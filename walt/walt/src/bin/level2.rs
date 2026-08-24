//! EXPLORATORY LEVEL-2 SOLVER — sits below every evidentiary tier and is
//! cited by nothing above it (wiki/ideas tier; outputs are ESTIMATES).
//!
//! level1.rs iterated once more: the field seats stop being solipsists and
//! start being LEVEL-1 MINDS. Each field seat, at its own information
//! state, best-responds to a field of level-0 minds over its own small
//! belief sample — i.e. each modeled opponent and partner is exactly the
//! player that beat the eq champion, in miniature. S1 best-responds to
//! that. This is the first level at which S1's model of its partner
//! contains partnership: the modeled partner coordinates back, so plays
//! can carry value THROUGH the partner's reading of them.
//!
//! The field model is now a parameter (Jason's framing): Dice at the
//! bottom, Level(k) above it. A level-k mind's solver has a Level(k-1)
//! field; level 0's has Dice. Stacking further is one line — the piper is
//! the product of the per-level sample sizes.
//!
//! Modeled minds at every level use the no-void size-consistent inner
//! belief (the stated level-0 simplification, carried up unchanged); the
//! outer S1 solve still conditions its worlds on the replayed boundary
//! voids. Level-0 minds are seeded bit-identically with level1.rs, so any
//! divergence from the level-1 answer is the model upgrade, not a reseed.
//!
//! Devices carried unchanged (ruled SOUND for the exact solve;
//! exact-on-the-sample here): decided cutoffs, viewer early exit, pmake
//! key reduction, interned alive sets, alive-set partition at field nodes
//! (Bayes on policy-consistent deals — now against level-1 behavior).
//!
//! PARALLEL: the outer solve fans out on rayon — root candidate leads,
//! field-node bucket recursion, and per-node batches of modeled-mind
//! policy calls. Modeled minds are the leaf tasks and stay serial. Every
//! policy value is a pure function of its full information-state key
//! (PiKey now includes the banked totals — see its doc; the serial probes
//! aliased these) and all sums are exact rationals, so RESULT lines are
//! identical at any thread count; only the stats can drift (two threads
//! may race the same transposed subtree before the memo fills,
//! duplicating work).
//!
//! Arithmetic: integer counts and BigRational values. No floats.
//! Nothing here is quotable above exploratory tier; not a P-A21 statement.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use rayon::prelude::*;

use walt::carrier::VIEWER;
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};

/// All 28 tiles.
const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Refuse to materialize a support larger than this (worlds, upper bound).
const SUPPORT_CAP: u128 = 30_000_000;

/// Frozen outer-deal seed (same stream discipline as level1.rs).
const OUTER_SEED: u64 = 0x8CB9_2BA7_2F3D_8DD7;

/// Frozen seed for inner-mind sampling. Level-0 minds mix exactly as in
/// level1.rs; level-k minds (k >= 1) add a level tag to the mix.
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

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

type Alive = Arc<Vec<u32>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Key {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    alive: u32,
}

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

/// How the field seats behave inside a solver: dice at the bottom, a
/// level-k policy above it. THE FIELD MODEL IS A PARAMETER.
#[derive(Clone, Copy)]
enum Field {
    Dice,
    Level(usize),
}

/// Cache key for a modeled mind's decision: its entire information state.
/// The banked totals are part of that state (the pmake objective conditions
/// on them) and are NOT derivable from the reduced record — omitting them
/// (as the serial probes did) aliased policies across banked contexts,
/// first-come-wins. Including them makes every cache entry a pure function
/// of its key, which is also what makes the parallel solve deterministic.
#[derive(Clone, PartialEq, Eq, Hash)]
struct PiKey {
    seat: u8,
    hand: u32,
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
}

/// Shard count for the concurrent policy cache (power of two).
const PI_SHARDS: usize = 64;

type PiShard = Mutex<HashMap<(u8, PiKey), u8>>;

/// State shared by every solver in the recursion: the global budget, the
/// cross-level policy cache, and the per-level inner sample sizes.
struct Shared {
    dcl: Decl,
    /// n_inner[k] = belief sample size of a modeled level-k mind.
    n_inner: Vec<usize>,
    boundary_played: u32,
    boundary_hand_size: usize,
    deadline: Instant,
    pi_cache: Vec<PiShard>,
    pi_calls: AtomicU64,
    nodes: AtomicU64,
    dead: AtomicBool,
}

impl Shared {
    fn pi_shard(&self, k: u8, pk: &PiKey) -> &PiShard {
        let mut h = DefaultHasher::new();
        (k, pk).hash(&mut h);
        &self.pi_cache[(h.finish() as usize) & (PI_SHARDS - 1)]
    }

    fn pi_cache_len(&self) -> usize {
        self.pi_cache
            .iter()
            .map(|s| s.lock().expect("pi shard poisoned").len())
            .sum()
    }
}

/// Interned alive sets: id 0 is always the full sample.
struct Intern {
    list: Vec<Alive>,
    map: HashMap<Alive, u32>,
}

struct Solver {
    sh: Arc<Shared>,
    viewer: Seat,
    viewer_hand0: u32,
    maximize: bool,
    /// Fan subtrees and policy batches out on rayon. Outer solver only;
    /// modeled minds are the leaf tasks and stay serial.
    parallel: bool,
    worlds: Vec<[u32; 4]>,
    /// Per-scenario dice seeds (Dice field only; empty otherwise).
    seeds: Vec<u64>,
    field: Field,
    intern: Mutex<Intern>,
    memo: Mutex<HashMap<Key, BigRational>>,
    /// Nodes visited by this solver; flushed to the global count in 64k
    /// chunks (and finally by `flush_nodes`) to keep 18 cores off one
    /// cache line.
    local_nodes: AtomicU64,
}

impl Solver {
    fn new(
        sh: Arc<Shared>,
        viewer: Seat,
        viewer_hand0: u32,
        maximize: bool,
        worlds: Vec<[u32; 4]>,
        seeds: Vec<u64>,
        field: Field,
    ) -> Self {
        let all: Alive = Arc::new((0..worlds.len() as u32).collect());
        let mut map = HashMap::new();
        map.insert(Arc::clone(&all), 0u32);
        Solver {
            sh,
            viewer,
            viewer_hand0,
            maximize,
            parallel: false,
            worlds,
            seeds,
            field,
            intern: Mutex::new(Intern {
                list: vec![all],
                map,
            }),
            memo: Mutex::new(HashMap::new()),
            local_nodes: AtomicU64::new(0),
        }
    }

    fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }

    fn intern(&self, v: Vec<u32>) -> u32 {
        let rc: Alive = Arc::new(v);
        let mut st = self.intern.lock().expect("intern poisoned");
        if let Some(&id) = st.map.get(&rc) {
            return id;
        }
        let id = st.list.len() as u32;
        st.list.push(Arc::clone(&rc));
        st.map.insert(rc, id);
        id
    }

    fn alive_of(&self, id: u32) -> Alive {
        Arc::clone(&self.intern.lock().expect("intern poisoned").list[id as usize])
    }

    /// Count one node; check budget on 64k-node boundaries of this solver's
    /// local count. Returns false when the global budget is dead.
    fn bump_node(&self) -> bool {
        if self.sh.dead.load(Ordering::Relaxed) {
            return false;
        }
        let n = self.local_nodes.fetch_add(1, Ordering::Relaxed) + 1;
        if n & 0xFFFF == 0 {
            if Instant::now() >= self.sh.deadline {
                self.sh.dead.store(true, Ordering::Relaxed);
                return false;
            }
            let before = self.sh.nodes.fetch_add(0x1_0000, Ordering::Relaxed);
            let after = before + 0x1_0000;
            if before / 50_000_000 != after / 50_000_000 {
                eprintln!(
                    "progress: nodes={} pi-calls={} pi-cache={}",
                    after,
                    self.sh.pi_calls.load(Ordering::Relaxed),
                    self.sh.pi_cache_len()
                );
            }
        }
        true
    }

    /// Flush the sub-64k remainder of this solver's node count into the
    /// global total. Call exactly once, after this solver's last solve.
    fn flush_nodes(&self) {
        self.sh.nodes.fetch_add(
            self.local_nodes.load(Ordering::Relaxed) & 0xFFFF,
            Ordering::Relaxed,
        );
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
            let winner = trick.winner(self.sh.dcl);
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

    fn solve(&self, key: &Key) -> Option<BigRational> {
        if !self.bump_node() {
            return None;
        }
        if key.banked_t1 >= 30 {
            return Some(BigRational::one());
        }
        if key.banked_t0 > 12 {
            return Some(BigRational::zero());
        }
        if let Some(v) = self.memo.lock().expect("memo poisoned").get(key) {
            return Some(v.clone());
        }
        assert_ne!(key.played, FULL_MASK, "terminal states are always decided");
        let seat =
            Seat::from_index((usize::from(key.leader) + key.plays.len()) % 4).expect("seat index");
        let led: Option<Context> = key.plays.first().map(|&i| {
            self.sh
                .dcl
                .led_context(Domino::from_index(usize::from(i)).expect("led index"))
        });
        let val = if seat == self.viewer {
            self.solve_viewer(key, led)?
        } else {
            match self.field {
                Field::Dice => self.solve_field_dice(key, seat, led)?,
                Field::Level(k) => self.solve_field_policy(key, seat, led, k)?,
            }
        };
        self.memo
            .lock()
            .expect("memo poisoned")
            .insert(key.clone(), val.clone());
        Some(val)
    }

    fn solve_viewer(&self, key: &Key, led: Option<Context>) -> Option<BigRational> {
        let hand = self.viewer_hand0 & !key.played;
        let legal = legal_plays(self.sh.dcl, set_of(hand), led);
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

    fn solve_field_dice(&self, key: &Key, seat: Seat, led: Option<Context>) -> Option<BigRational> {
        let alive = self.alive_of(key.alive);
        let rh = record_hash(key);
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 28];
        for &sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            let idx =
                SplitMix64(self.seeds[sid as usize] ^ rh).below(u64::from(lm.count_ones())) as u32;
            buckets[nth_set_bit(lm, idx) as usize].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    fn solve_field_policy(
        &self,
        key: &Key,
        seat: Seat,
        led: Option<Context>,
        k: usize,
    ) -> Option<BigRational> {
        let alive = self.alive_of(key.alive);
        // Per-world hands and legal masks; the distinct undecided hands are
        // the modeled minds this node needs.
        let mut per_sid: Vec<(u32, u32)> = Vec::with_capacity(alive.len());
        let mut distinct: HashMap<u32, u32> = HashMap::new();
        for &sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            if lm.count_ones() > 1 {
                distinct.entry(hand).or_insert(lm);
            }
            per_sid.push((hand, lm));
        }
        if self.parallel && distinct.len() > 1 {
            // Warm the shared policy cache for this node's minds in
            // parallel; values are key-deterministic, so fill order is
            // irrelevant.
            let reqs: Vec<(u32, u32)> = distinct.iter().map(|(&h, &lm)| (h, lm)).collect();
            let alive_count = reqs
                .par_iter()
                .filter(|&&(hand, lm)| self.pi(k, key, seat, hand, lm).is_some())
                .count();
            if alive_count != reqs.len() {
                return None;
            }
        }
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 28];
        for (i, &sid) in alive.iter().enumerate() {
            let (hand, lm) = per_sid[i];
            let tile = if lm.count_ones() == 1 {
                lm.trailing_zeros() as u8
            } else {
                self.pi(k, key, seat, hand, lm)?
            };
            buckets[usize::from(tile)].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    fn combine_buckets(
        &self,
        key: &Key,
        alive_len: usize,
        buckets: Vec<Vec<u32>>,
    ) -> Option<BigRational> {
        let denom = BigInt::from(alive_len);
        let mut children: Vec<(BigInt, Key)> = Vec::new();
        let mut redistributed: usize = 0;
        for (tile, bucket) in buckets.into_iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }
            redistributed += bucket.len();
            let reach = BigInt::from(bucket.len());
            let child_alive = self.intern(bucket);
            children.push((
                reach,
                self.child_after_play(
                    key,
                    Domino::from_index(tile).expect("tile < 28"),
                    child_alive,
                ),
            ));
        }
        assert_eq!(redistributed, alive_len, "field partition conservation");
        let vals: Vec<Option<BigRational>> = if self.parallel && children.len() > 1 {
            children
                .par_iter()
                .map(|(_, child)| self.solve(child))
                .collect()
        } else {
            children
                .iter()
                .map(|(_, child)| self.solve(child))
                .collect()
        };
        let mut total = BigRational::zero();
        for ((reach, _), v) in children.iter().zip(vals) {
            let v = v?;
            if !v.is_zero() {
                total += v * BigRational::new(reach.clone(), denom.clone());
            }
        }
        Some(total)
    }

    fn hand_sizes_at(&self, key: &Key) -> [usize; 4] {
        let played_since = (key.played.count_ones() - self.sh.boundary_played.count_ones())
            as usize
            - key.plays.len();
        assert_eq!(played_since % 4, 0, "completed tricks are whole");
        let completed = played_since / 4;
        let mut sizes = [self.sh.boundary_hand_size - completed; 4];
        for i in 0..key.plays.len() {
            sizes[(usize::from(key.leader) + i) % 4] -= 1;
        }
        sizes
    }

    /// The level-k policy at a modeled seat's information state: a level-k
    /// mind samples n_inner[k] no-void size-consistent deals from its own
    /// chair and best-responds to a Level(k-1) field (Dice below level 0).
    /// Level-0 seeding is bit-identical with level1.rs.
    fn pi(&self, k: usize, key: &Key, seat: Seat, hand: u32, legal_mask: u32) -> Option<u8> {
        let pk = PiKey {
            seat: seat.index() as u8,
            hand,
            played: key.played,
            leader: key.leader,
            plays: key.plays.clone(),
            banked_t1: key.banked_t1,
            banked_t0: key.banked_t0,
        };
        let kb = k as u8;
        if let Some(&t) = self
            .sh
            .pi_shard(kb, &pk)
            .lock()
            .expect("pi shard poisoned")
            .get(&(kb, pk.clone()))
        {
            return Some(t);
        }
        self.sh.pi_calls.fetch_add(1, Ordering::Relaxed);
        let n_k = self.sh.n_inner[k];
        let sizes = self.hand_sizes_at(key);
        let unseen = FULL_MASK & !key.played & !hand;
        let others: Vec<usize> = (0..4).filter(|&s| s != seat.index()).collect();
        let need: usize = others.iter().map(|&s| sizes[s]).sum();
        assert_eq!(unseen.count_ones() as usize, need, "unseen tiles fit sizes");
        let level_tag = if k == 0 { 0 } else { mix(0x4C32 ^ k as u64) };
        let mut rng = SplitMix64(
            INNER_SEED
                ^ level_tag
                ^ mix(seat.index() as u64)
                ^ mix(u64::from(hand))
                ^ record_hash(key),
        );
        let mut tiles = mask_bits(unseen);
        let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
        let mut inner_worlds: Vec<[u32; 4]> = Vec::with_capacity(n_k);
        for _ in 0..n_k {
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
        let (inner_field, inner_seeds) = if k == 0 {
            let seeds: Vec<u64> = (0..n_k).map(|_| rng.next_u64()).collect();
            (Field::Dice, seeds)
        } else {
            (Field::Level(k - 1), Vec::new())
        };
        let maximize = seat.team() == Team::T1;
        let inner = Solver::new(
            Arc::clone(&self.sh),
            seat,
            hand,
            maximize,
            inner_worlds,
            inner_seeds,
            inner_field,
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
        let mut died = false;
        while lm != 0 {
            let tile_idx = lm.trailing_zeros() as u8;
            lm &= lm - 1;
            let tile = Domino::from_index(usize::from(tile_idx)).expect("tile < 28");
            let child = inner.child_after_play(&root, tile, 0);
            match inner.solve(&child) {
                Some(v) => {
                    let better = best
                        .as_ref()
                        .is_none_or(|(b, _)| if maximize { v > *b } else { v < *b });
                    if better {
                        best = Some((v, tile_idx));
                    }
                }
                None => {
                    died = true;
                    break;
                }
            }
        }
        inner.flush_nodes();
        if died {
            return None;
        }
        let (_, choice) = best.expect("legal play exists");
        self.sh
            .pi_shard(kb, &pk)
            .lock()
            .expect("pi shard poisoned")
            .insert((kb, pk), choice);
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

fn run_boundary(dcl: Decl, t: usize, budget_secs: u64, n_outer: usize, n1: usize, n0: usize) {
    println!(
        "== boundary t={t}: LEVEL-2 solve from the start of trick {t}, n_outer={n_outer}, n1={n1}, n0={n0}"
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
                    "  support: {} void-consistent deals — running ALL of them (deterministic level-1 field: no dice axis)",
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
            "  level-2 values are EXACT over the support GIVEN the level-1 field policy; approximations live inside the modeled minds (n1={n1}, n0={n0}, no-void inner beliefs)"
        );
    } else {
        println!(
            "  SAMPLED DEALS: seed=0x{:016X} — values are ESTIMATES",
            OUTER_SEED ^ t as u64
        );
    }
    let sh = Arc::new(Shared {
        dcl,
        n_inner: vec![n0, n1],
        boundary_played: b.played,
        boundary_hand_size: b.hand_size,
        deadline: start + std::time::Duration::from_secs(budget_secs),
        pi_cache: (0..PI_SHARDS).map(|_| Mutex::new(HashMap::new())).collect(),
        pi_calls: AtomicU64::new(0),
        nodes: AtomicU64::new(0),
        dead: AtomicBool::new(false),
    });
    let n_deals = outer_worlds.len();
    let solver = Solver::new(
        Arc::clone(&sh),
        VIEWER,
        b.s1_initial & !b.played,
        true,
        outer_worlds,
        Vec::new(),
        Field::Level(1),
    )
    .parallel();
    let root = Key {
        played: b.played,
        leader: b.leader,
        plays: Vec::new(),
        banked_t1: b.banked_t1,
        banked_t0: b.banked_t0,
        alive: 0,
    };
    println!(
        "  root: leader S{}, banked T1={} T0={}, {} deals, budget {}s, {} rayon threads",
        b.leader,
        b.banked_t1,
        b.banked_t0,
        n_deals,
        budget_secs,
        rayon::current_num_threads()
    );
    if usize::from(b.leader) == VIEWER.index() {
        let hand = b.s1_initial & !b.played;
        let legal = legal_plays(dcl, set_of(hand), None);
        let tiles: Vec<Domino> = legal.iter().collect();
        // Root candidate leads have disjoint futures (the lead is in
        // `played`), so they parallelize without memo interference.
        let results: Vec<Option<BigRational>> = tiles
            .par_iter()
            .map(|&tile| {
                let child = solver.child_after_play(&root, tile, 0);
                solver.solve(&child)
            })
            .collect();
        solver.flush_nodes();
        let mut values: Vec<(Domino, BigRational)> = Vec::new();
        for (tile, v) in tiles.into_iter().zip(results) {
            match v {
                Some(v) => values.push((tile, v)),
                None => {
                    report_death(&sh, t, start);
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
            "  level-2 pmake play at trick {t}: {}{}",
            ties.join(" / "),
            if ties.len() > 1 { "  (tie)" } else { "" }
        );
    } else {
        let v = solver.solve(&root);
        solver.flush_nodes();
        match v {
            Some(v) => println!("  P(make) before trick {t} = {}", show(&v)),
            None => {
                report_death(&sh, t, start);
                return;
            }
        }
    }
    let ms = start.elapsed().as_millis();
    println!(
        "  stats: {} nodes (all levels), {} outer memo entries, {} alive-sets, {} pi evaluations ({} cached), {}.{:03}s",
        sh.nodes.load(Ordering::Relaxed),
        solver.memo.lock().expect("memo poisoned").len(),
        solver.intern.lock().expect("intern poisoned").list.len(),
        sh.pi_calls.load(Ordering::Relaxed),
        sh.pi_cache_len(),
        ms / 1000,
        ms % 1000
    );
    println!();
}

fn report_death(sh: &Shared, t: usize, start: Instant) {
    let ms = start.elapsed().as_millis();
    println!(
        "  DIED at t={t}: wall-clock budget exceeded after {} nodes, {} pi evaluations, {}.{:03}s",
        sh.nodes.load(Ordering::Relaxed),
        sh.pi_calls.load(Ordering::Relaxed),
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
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer deal count"))
        .unwrap_or(2000);
    let n1: usize = args
        .get(4)
        .map(|s| s.parse().expect("level-1 mind sample size"))
        .unwrap_or(8);
    let n0: usize = args
        .get(5)
        .map(|s| s.parse().expect("level-0 mind sample size"))
        .unwrap_or(4);

    println!("walt-m3-probe level-2 solver: EXPLORATORY twice-iterated scenario player");
    println!(
        "hand 8, trump fives, P30 by T1; field seats are LEVEL-1 MINDS (each best-responds over its"
    );
    println!(
        "own n1-world belief to a level-0 field, which best-responds over n0 worlds to dice);"
    );
    println!(
        "S1 best-responds to that — the first level whose model of the partner coordinates back."
    );
    println!("S1 keyed on public record + own hand only — no fusion at any level of the stack.");
    println!();

    match only_t {
        Some(t) => run_boundary(dcl, t, budget_secs, n_outer, n1, n0),
        None => {
            for t in (1..=4).rev() {
                run_boundary(dcl, t, budget_secs, n_outer, n1, n0);
            }
        }
    }
    println!(
        "nothing above exploratory tier; level-2 values are never receipts; not a P-A21 statement"
    );
}
