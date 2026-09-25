//! EXPLORATORY seat-solver library — the sampling-stack machinery of
//! SCENARIO-PLAYER.md, extracted from the probe binaries (bidcurve.rs
//! lineage, itself the divergence.rs/level2.rs line). Sits below every
//! evidentiary tier; nothing above the Ideas tier cites it.
//!
//! One solver, independent axes of configuration:
//!   - `Field`: Dice at the bottom, `Level(k)` above it (the field model
//!     is a parameter — level-k minds best-respond to level-(k−1) minds).
//!   - `InnerBelief`: legacy voidless or public-void-conditioned counted
//!     sampling inside every modeled level. Legacy is the default.
//!   - `bid`: the pmake objective's threshold pair (make ⇔ banked_T1 ≥ bid
//!     ⇔ banked_T0 ≤ 42 − bid), so bidding and play share the machinery.
//!   - `parallel` (cargo feature + runtime flag): rayon fan-out on root
//!     candidates, bucket recursion, and per-node policy batches. Compiled
//!     out entirely without the "parallel" feature (the WASM build).
//!
//! Invariants (SCENARIO-PLAYER.md): PiKey carries the modeled mind's
//! represented information state, including banked totals and the selected
//! void state. Every cache entry is a pure function of its key, which is what makes results
//! invariant across thread counts and call orders (§8). Exact rationals
//! only — no floats.

pub mod act;
pub mod adaptive;
pub mod bundle;
mod cache;
pub mod calibrate;
#[cfg(feature = "sharded-root-memo")]
mod root_memo;
pub mod contract;
pub use contract::Contract;

#[cfg(feature = "sharded-root-memo")]
use root_memo::RootMemo;
#[cfg(feature = "bypass-l0-cache")]
mod uncached_l0;

pub mod counters;
#[cfg(feature = "compact-dice")]
use counters::MaxCounter;
use counters::SumCounter;
#[cfg(feature = "compact-dice")]
mod compact_dice;

#[cfg(feature = "compact-policy")]
mod compact_policy;
use cache::{CacheHasher, CacheMap, MemoKey, PolicyKey};
pub mod controller;
pub mod covers;
pub mod doom;
pub mod evidence;
pub mod exposure;
pub mod extraction;
pub mod factor_belief;
pub mod field;
pub mod field_swap;
pub mod focal_horizon;
pub mod focal_ladder;
pub mod frontier;
pub mod godgap;
pub mod grammar;
pub mod hazard;
pub mod horizon;
pub mod inner_belief;
#[cfg(feature = "fast-rng")]
mod rng_small;
pub use inner_belief::InnerBelief;
pub mod laydown;
pub mod model_belief;
pub mod model_recursion;
pub mod nello_counterexample;
pub mod motif;
pub mod opening;
pub mod partnership;
pub mod policy;
pub mod proof_state;
pub mod refine;
pub mod residual;
pub mod root_interval;
pub mod selection;
mod support;
use support::{Alive, SmallSupport};
#[cfg(feature = "mask64-support-arena")]
mod mask64_arena;
#[cfg(feature = "mask64-support-arena")]
use mask64_arena::Mask64Arena;
pub mod targeted;
pub mod unified;
pub mod upper_cs;
pub mod wakeup;
pub mod waking;

use crate::clock::Instant;
#[cfg(not(feature = "fast-policy"))]
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Monotonic wall budget on both native and browser hosts.
#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    at: Instant,
}
impl Deadline {
    #[must_use]
    pub fn after(budget: Duration) -> Self {
        Self {
            at: Instant::now() + budget,
        }
    }
    #[must_use]
    pub fn passed(&self) -> bool {
        Instant::now() >= self.at
    }
}

use num_bigint::BigInt;
use num_rational::BigRational;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::rules::rules::{legal_plays, Trick};
use crate::rules::{Context, Decl, Domino, DominoSet, Seat, Team};

/// All 28 tiles.
pub const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Frozen seed for inner-mind sampling (identical across the whole stack:
/// level1.rs, level2.rs, walt_bridge.rs, divergence.rs, bidcurve.rs).
pub const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: u64) -> u64 {
        #[cfg(feature = "fast-rng")]
        if (1..=28).contains(&n) {
            return rng_small::below(self, n);
        }
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next_u64();
            if v < zone {
                return v % n;
            }
        }
    }
}

pub fn mix(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn bit(dm: Domino) -> u32 {
    1u32 << dm.index()
}

pub fn mask_of(set: DominoSet) -> u32 {
    set.bits()
}

pub fn set_of(mask: u32) -> DominoSet {
    DominoSet::from_bits(mask).expect("indices < 28")
}

pub fn mask_bits(mask: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(mask.count_ones() as usize);
    let mut m = mask;
    while m != 0 {
        v.push(m.trailing_zeros() as u8);
        m &= m - 1;
    }
    v
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Key {
    /// None preserves legacy identity; Some tracks public void deductions.
    pub voids: Option<[u32; 4]>,
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
    pub alive: u32,
}

pub fn record_hash(key: &Key) -> u64 {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    Dice,
    Level(usize),
    /// Per-seat modeled levels. The viewer entry is ignored because the
    /// viewer always takes the maximizing/minimizing branch directly.
    SeatLevels([usize; 4]),
}

/// The modeled mind's represented information state, banked totals included
/// (SCENARIO-PLAYER.md §3.1/§3.4).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PiKey {
    pub voids: Option<[u32; 4]>,
    pub seat: u8,
    pub hand: u32,
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
}

const PI_SHARDS: usize = 64;

#[cfg_attr(feature = "aligned-cache", repr(align(128)))]
struct PiShard(Mutex<CacheMap<(u8, PolicyKey), u8>>);

impl std::ops::Deref for PiShard {
    type Target = Mutex<CacheMap<(u8, PolicyKey), u8>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// State shared by every solver in one evaluation: declaration, bid
/// thresholds, boundary frame, budget, and the cross-level policy cache.
pub struct Shared {
    inner_belief: InnerBelief,
    modeled_selection: selection::Rule,
    inner_worlds_by_level: Vec<SumCounter>,
    pub dcl: Decl,
    pub contract: Contract,
    /// Straight target; Nel-O carries its mark stake without a points target.
    pub bid: u8,
    /// n_inner[k] = belief sample size of a modeled level-k mind.
    pub n_inner: Vec<usize>,
    pub boundary_played: u32,
    pub boundary_hand_size: usize,
    pub deadline: Deadline,
    pi_cache: Vec<PiShard>,
    pub pi_calls: SumCounter,
    pi_calls_by_level: Vec<SumCounter>,

    pub nodes: SumCounter,
    /// Break instrumentation on the `solve_viewer` loop, folded from each
    /// solver's locals by `flush_nodes`: children actually solved vs legal
    /// moves available. An instrument only — read by probes and tests,
    /// never by a receipt bin.
    pub viewer_children: SumCounter,
    pub viewer_legal: SumCounter,
    /// Opt-in compact Dice calls, recursive visits, and largest remaining
    /// trick count seen. These measure completed and refused calls alike.
    #[cfg(feature = "compact-dice")]
    pub compact_dice_calls: SumCounter,
    #[cfg(feature = "compact-dice")]
    pub compact_dice_nodes: SumCounter,
    #[cfg(feature = "compact-dice")]
    pub compact_dice_max_tricks: MaxCounter,
    #[cfg(feature = "bounded-choice")]
    pub bounded_choice_probes: SumCounter,
    #[cfg(feature = "bounded-choice")]
    pub bounded_choice_rejections: SumCounter,
    pub dead: AtomicBool,
}

impl Shared {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dcl: Decl,
        bid: u8,
        n_inner: Vec<usize>,
        boundary_played: u32,
        boundary_hand_size: usize,
        deadline: Deadline,
    ) -> Self {
        let levels = n_inner.len();
        let pi_calls_by_level = (0..levels).map(|_| SumCounter::new(0)).collect();
        Shared {
            inner_belief: InnerBelief::Voidless,
            modeled_selection: selection::Rule::Fixed,
            inner_worlds_by_level: (0..levels).map(|_| SumCounter::new(0)).collect(),
            dcl,
            contract: Contract::Straight { bid },
            bid,
            n_inner,
            boundary_played,
            boundary_hand_size,
            deadline,
            pi_cache: (0..PI_SHARDS)
                .map(|_| PiShard(Mutex::new(CacheMap::default())))
                .collect(),
            pi_calls: SumCounter::new(0),
            pi_calls_by_level,

            nodes: SumCounter::new(0),
            viewer_children: SumCounter::new(0),
            viewer_legal: SumCounter::new(0),
            #[cfg(feature = "compact-dice")]
            compact_dice_calls: SumCounter::new(0),
            #[cfg(feature = "compact-dice")]
            compact_dice_nodes: SumCounter::new(0),
            #[cfg(feature = "compact-dice")]
            compact_dice_max_tricks: MaxCounter::new(0),
            #[cfg(feature = "bounded-choice")]
            bounded_choice_probes: SumCounter::new(0),
            #[cfg(feature = "bounded-choice")]
            bounded_choice_rejections: SumCounter::new(0),
            dead: AtomicBool::new(false),
        }
    }

    fn straight_fast_paths(&self) -> bool {
        !self.contract.is_nello() && self.dcl.is_straight()
    }

    pub fn with_contract(mut self, contract: Contract) -> Self {
        assert_eq!(self.pi_cache_len(), 0);
        if let Contract::Nello { declarer } = contract {
            assert_eq!(self.dcl, Decl::DoublesSuit);
            assert_eq!(declarer.team(), Team::T1);
        }
        self.contract = contract;
        self
    }

    /// Select before sharing an evaluation. One cache never mixes belief
    /// strategies; existing constructors retain historical behavior.
    #[must_use]
    pub fn with_inner_belief(mut self, strategy: InnerBelief) -> Self {
        assert_eq!(self.pi_cache_len(), 0, "select belief before evaluating");
        self.inner_belief = strategy;
        self
    }

    /// Cache identity includes this immutable evaluation-scoped policy.
    #[must_use]
    pub fn with_modeled_selection(mut self, rule: selection::Rule) -> Self {
        assert_eq!(self.pi_cache_len(), 0, "select policy before evaluating");
        self.modeled_selection = rule;
        self
    }

    /// Completed sampled worlds, including every refinement/racing batch.
    pub fn inner_worlds_by_level(&self) -> Vec<u64> {
        self.inner_worlds_by_level
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .collect()
    }

    pub const fn inner_belief(&self) -> InnerBelief {
        self.inner_belief
    }

    /// Folded `solve_viewer` break counters: (children actually solved,
    /// legal moves available). Ratio 1/1 means the Boolean break never
    /// fired; under rayon a memo miss can be recomputed concurrently, so
    /// the totals are an instrument, not a determinism freeze.
    pub fn viewer_break_totals(&self) -> (u64, u64) {
        (
            self.viewer_children.load(Ordering::Relaxed),
            self.viewer_legal.load(Ordering::Relaxed),
        )
    }

    fn pi_shard(&self, k: u8, pk: &PolicyKey) -> &PiShard {
        let mut h = CacheHasher::default();
        (k, pk).hash(&mut h);
        &self.pi_cache[(h.finish() as usize) & (PI_SHARDS - 1)]
    }

    pub fn pi_cache_len(&self) -> usize {
        self.pi_cache
            .iter()
            .map(|s| s.lock().expect("pi shard poisoned").len())
            .sum()
    }

    // A balanced boundary (B,H) reconstructs later hand sizes solely through
    // C = H + |B|/active seats. The inactive Nel-O hand always has seven tiles.
    // Do not transfer from malformed or incompatible frames.
    // See walt/CPU-SPEEDUPS.md for the cache transfer condition.
    #[cfg(feature = "hand-cache")]
    fn normalized_boundary(&self) -> Option<usize> {
        let played = self.boundary_played.count_ones() as usize;
        if self.boundary_played & !FULL_MASK != 0 || !played.is_multiple_of(self.contract.trick_size()) {
            return None;
        }
        self.boundary_hand_size.checked_add(played / self.contract.trick_size())
    }

    fn same_policy_boundary(&self, previous: &Shared) -> bool {
        #[cfg(feature = "hand-cache")]
        {
            self.normalized_boundary()
                .is_some_and(|c| previous.normalized_boundary() == Some(c))
        }
        #[cfg(not(feature = "hand-cache"))]
        {
            self.boundary_played == previous.boundary_played
                && self.boundary_hand_size == previous.boundary_hand_size
        }
    }

    /// Move completed modeled-policy answers into a fresh evaluation of the
    /// same policy context. Outer samples and deadlines are deliberately absent:
    /// `pi` seeds its own belief from (level, seat, hand, public record), and
    /// inserts only completed answers. Budgets/counters/death flags stay fresh.
    /// Exclusive ownership of both contexts prevents use during a live solve.
    pub fn take_policy_cache_from(&mut self, previous: &mut Shared) -> usize {
        assert_eq!(self.pi_cache_len(), 0, "reuse only before evaluating");
        if self.contract != previous.contract
            || self.dcl != previous.dcl
            || self.bid != previous.bid
            || self.n_inner != previous.n_inner
            || !self.same_policy_boundary(previous)
            || self.inner_belief != previous.inner_belief
            || self.modeled_selection != previous.modeled_selection
        {
            return 0;
        }
        std::mem::swap(&mut self.pi_cache, &mut previous.pi_cache);
        self.pi_cache_len()
    }

    /// Cache-miss computations performed at each modeled level. A level-k
    /// miss invokes the level's frozen selection schedule. Sample totals are
    /// measured separately because a schedule can evaluate several bundles.
    pub fn pi_calls_by_level(&self) -> Vec<u64> {
        self.pi_calls_by_level
            .iter()
            .map(|n| n.load(Ordering::Relaxed))
            .collect()
    }
}

struct Intern {
    list: Vec<Arc<Vec<u32>>>,
    map: CacheMap<Arc<Vec<u32>>, u32>,
}

/// The `solve_viewer` visit-order selector (reorder-not-cull;
/// CENSUS-RULINGS.md E-A15: the ORDER of evaluation is lawful to change,
/// the SET is not). `CaptureFirst` is the one default; `TileIndex` — the
/// historical ascending order — is retained solely for the equivalence
/// gate and A/B instrument runs, never as a public configuration surface.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveOrdering {
    CaptureFirst,
    TileIndex,
}

pub struct Solver {
    pub sh: Arc<Shared>,
    viewer: Seat,
    viewer_hand0: u32,
    maximize: bool,
    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    parallel: bool,
    worlds: Vec<[u32; 4]>,
    /// Per-scenario dice seeds (Dice field only; empty otherwise).
    seeds: Vec<u64>,
    field: Field,
    intern: Mutex<Intern>,
    small_support: Option<SmallSupport>,
    #[cfg(feature = "mask64-support-arena")]
    mask64_support: Option<Mask64Arena>,
    // All worlds have equal mass, including duplicate sampled worlds. Each
    // node's value is an integer success count over its alive set. Convert to
    // a rational only at the public boundary (experiments/kiln/COUNTED-VALUES.md).
    #[cfg(not(feature = "sharded-root-memo"))]
    memo: Mutex<CacheMap<MemoKey, u64>>,
    #[cfg(feature = "sharded-root-memo")]
    memo: RootMemo,
    local_nodes: AtomicU64,
    local_viewer_children: AtomicU64,
    local_viewer_legal: AtomicU64,
    ordering: MoveOrdering,
}

impl Solver {
    pub fn new(
        sh: Arc<Shared>,
        viewer: Seat,
        viewer_hand0: u32,
        maximize: bool,
        worlds: Vec<[u32; 4]>,
        seeds: Vec<u64>,
        field: Field,
    ) -> Self {
        match field {
            Field::Dice => {}
            Field::Level(level) => assert!(
                level < sh.n_inner.len(),
                "the uniform field level has a declared inner sample count"
            ),
            Field::SeatLevels(levels) => assert!(
                levels.iter().all(|&level| level < sh.n_inner.len()),
                "every seat field level has a declared inner sample count"
            ),
        }
        let small_support = SmallSupport::new(worlds.len());
        #[cfg(feature = "mask64-support-arena")]
        let mask64_support = Mask64Arena::new(worlds.len());
        #[cfg(feature = "mask64-support-arena")]
        let large_support = small_support.is_none() && mask64_support.is_none();
        #[cfg(not(feature = "mask64-support-arena"))]
        let large_support = small_support.is_none();
        #[cfg(feature = "sharded-root-memo")]
        let memo = RootMemo::new(worlds.len());
        let mut map = CacheMap::default();
        let list = if !large_support {
            Vec::new()
        } else {
            let all: Arc<Vec<u32>> = Arc::new((0..worlds.len() as u32).collect());
            map.insert(Arc::clone(&all), 0u32);
            vec![all]
        };
        Solver {
            sh,
            viewer,
            viewer_hand0,
            maximize,
            parallel: false,
            worlds,
            seeds,
            field,
            intern: Mutex::new(Intern { list, map }),
            small_support,
            #[cfg(feature = "mask64-support-arena")]
            mask64_support,
            #[cfg(not(feature = "sharded-root-memo"))]
            memo: Mutex::new(CacheMap::default()),
            #[cfg(feature = "sharded-root-memo")]
            memo,
            local_nodes: AtomicU64::new(0),
            local_viewer_children: AtomicU64::new(0),
            local_viewer_legal: AtomicU64::new(0),
            ordering: MoveOrdering::CaptureFirst,
        }
    }

    /// Enable rayon fan-out (no-op in a build without the "parallel"
    /// feature — the flag is kept so caller code is identical).
    #[must_use]
    pub fn parallel(self) -> Self {
        // A one-thread production worker gains nothing from nested Rayon
        // dispatch; retain the serial path and its allocation savings.
        #[cfg(feature = "parallel")]
        {
            Self {
                parallel: rayon::current_num_threads() > 1,
                ..self
            }
        }
        #[cfg(not(feature = "parallel"))]
        {
            self
        }
    }

    /// Select the `solve_viewer` visit order (default `CaptureFirst`).
    /// For the equivalence gate and instrument A/B arms only — value
    /// results are identical under every selector by E-A15.
    #[must_use]
    pub fn with_ordering(mut self, ordering: MoveOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    #[cfg(feature = "parallel")]
    fn field_prewarm_parallel(&self, key: &Key, alive_len: usize, distinct: usize) -> bool {
        if !self.parallel || distinct < 2 {
            return false;
        }
        #[cfg(feature = "adaptive-parallel")]
        {
            let _ = key;
            alive_len > 2 && distinct >= 4
        }
        #[cfg(not(feature = "adaptive-parallel"))]
        {
            let _ = (key, alive_len);
            true
        }
    }

    pub fn memo_len(&self) -> usize {
        #[cfg(not(feature = "sharded-root-memo"))]
        {
            self.memo.lock().expect("memo poisoned").len()
        }
        #[cfg(feature = "sharded-root-memo")]
        {
            self.memo.len()
        }
    }

    pub fn alive_sets(&self) -> usize {
        if let Some(small) = &self.small_support {
            return small.seen_count();
        }
        #[cfg(feature = "mask64-support-arena")]
        if let Some(mask64) = &self.mask64_support {
            return mask64.seen_count();
        }
        self.intern.lock().expect("intern poisoned").list.len()
    }

    fn intern(&self, v: Vec<u32>) -> u32 {
        if let Some(small) = &self.small_support {
            return small.encode_ids(&v);
        }
        #[cfg(feature = "mask64-support-arena")]
        if let Some(mask64) = &self.mask64_support {
            return mask64.intern_ids(&v);
        }
        let rc: Arc<Vec<u32>> = Arc::new(v);
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
        if let Some(small) = &self.small_support {
            return small.decode(id);
        }
        #[cfg(feature = "mask64-support-arena")]
        if let Some(mask64) = &self.mask64_support {
            return mask64.decode(id);
        }
        Alive::Large(Arc::clone(
            &self.intern.lock().expect("intern poisoned").list[id as usize],
        ))
    }

    fn bump_node(&self) -> bool {
        if self.sh.dead.load(Ordering::Relaxed) {
            return false;
        }
        let n = self.local_nodes.fetch_add(1, Ordering::Relaxed) + 1;
        if n == 1 || n & 0xFF == 0 {
            if self.sh.deadline.passed() {
                self.sh.dead.store(true, Ordering::Relaxed);
                return false;
            }
            if n & 0xFF == 0 {
                self.sh.nodes.fetch_add(0x100, Ordering::Relaxed);
            }
        }
        true
    }

    /// Flush the sub-256 remainder of this solver's node count into the
    /// global total, and this solver's `solve_viewer` break counters whole.
    /// Call exactly once, after this solver's last solve.
    pub fn flush_nodes(&self) {
        self.sh.nodes.fetch_add(
            self.local_nodes.load(Ordering::Relaxed) & 0xFF,
            Ordering::Relaxed,
        );
        self.sh.viewer_children.fetch_add(
            self.local_viewer_children.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
        self.sh.viewer_legal.fetch_add(
            self.local_viewer_legal.load(Ordering::Relaxed),
            Ordering::Relaxed,
        );
    }

    /// This solver's own `solve_viewer` break counters: (children actually
    /// solved, legal moves available). See `Shared::viewer_break_totals`.
    pub fn viewer_break_counters(&self) -> (u64, u64) {
        (
            self.local_viewer_children.load(Ordering::Relaxed),
            self.local_viewer_legal.load(Ordering::Relaxed),
        )
    }

    fn check_belief_key(&self, key: &Key) {
        assert_eq!(
            key.voids.is_some(),
            self.sh.inner_belief == InnerBelief::VoidsCounted,
            "search key and evaluation belief strategy must agree"
        );
    }

    pub fn child_after_play(&self, key: &Key, tile: Domino, alive: u32) -> Key {
        self.check_belief_key(key);
        if self.sh.contract.is_nello() {
            let mut next = key.clone();
            next.alive = alive;
            self.sh.contract.step(&mut next, self.sh.dcl, tile);
            return next;
        }
        let voids = inner_belief::after_play(key, self.sh.dcl, tile);
        let played = key.played | bit(tile);
        if key.plays.len() == 3 {
            // Closing a trick needs no temporary heap-allocated play record.
            let doms = [
                Domino::from_index(usize::from(key.plays[0])).expect("p0"),
                Domino::from_index(usize::from(key.plays[1])).expect("p1"),
                Domino::from_index(usize::from(key.plays[2])).expect("p2"),
                tile,
            ];
            let leader = Seat::from_index(usize::from(key.leader)).expect("leader");
            let trick = Trick::new(leader, doms).expect("distinct tiles in trick");
            let winner = trick.winner(self.sh.dcl);
            let value = trick.points() as u8;
            let t1_won = winner.team() == Team::T1;
            Key {
                voids,
                played,
                leader: winner.index() as u8,
                plays: Vec::new(),
                banked_t1: key.banked_t1 + if t1_won { value } else { 0 },
                banked_t0: key.banked_t0 + if t1_won { 0 } else { value },
                alive,
            }
        } else {
            let mut plays = Vec::with_capacity(key.plays.len() + 1);
            plays.extend_from_slice(&key.plays);
            plays.push(tile.index() as u8);
            Key {
                voids,
                played,
                leader: key.leader,
                plays,
                banked_t1: key.banked_t1,
                banked_t0: key.banked_t0,
                alive,
            }
        }
    }

    pub fn solve(&self, key: &Key) -> Option<BigRational> {
        let wins = self.solve_count(key)?;
        Some(BigRational::new(
            BigInt::from(wins),
            BigInt::from(self.alive_of(key.alive).len()),
        ))
    }

    fn solve_count(&self, key: &Key) -> Option<u64> {
        self.check_belief_key(key);
        if !self.bump_node() {
            return None;
        }
        if let Some(made) = self.sh.contract.terminal(key) {
            return Some(if made { self.alive_of(key.alive).len() as u64 } else { 0 });
        }
        let memo_key = MemoKey::from(key);
        #[cfg(not(feature = "sharded-root-memo"))]
        let cached = self
            .memo
            .lock()
            .expect("memo poisoned")
            .get(&memo_key)
            .copied();
        #[cfg(feature = "sharded-root-memo")]
        let cached = self.memo.get(&memo_key);
        if let Some(value) = cached {
            return Some(value);
        }
        assert_ne!(key.played, FULL_MASK, "terminal states are always decided");
        let seat =
            self.sh.contract.actor(key.leader as usize, key.plays.len());
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
                Field::SeatLevels(levels) => {
                    self.solve_field_policy(key, seat, led, levels[seat.index()])?
                }
            }
        };
        #[cfg(not(feature = "sharded-root-memo"))]
        self.memo
            .lock()
            .expect("memo poisoned")
            .insert(memo_key, val);
        #[cfg(feature = "sharded-root-memo")]
        self.memo.insert(memo_key, val);
        Some(val)
    }

    /// First-order visit priority for the viewer's legal tiles — aimed
    /// at the Boolean pmake break in `solve_viewer`. Higher visits
    /// earlier. One priority serves maximizer and minimizer alike: the
    /// viewer's team banks any trick the viewer wins, and banked count
    /// is what saturates the payoff in BOTH directions (banked_t1 toward
    /// make, banked_t0 toward set), so "win the trick as it stands,
    /// richest capture first; otherwise give up the least count" chases
    /// the earliest exact decision either way. Pure in (dcl, key, led,
    /// tile), integer-only, no RNG, no shared tables — the visit order
    /// is deterministic run-to-run under rayon.
    fn viewer_visit_order_into<'a>(
        &self,
        key: &Key,
        led: Option<Context>,
        legal: DominoSet,
        storage: &'a mut [(u32, Domino); Domino::COUNT],
    ) -> &'a [(u32, Domino)] {
        let dcl = self.sh.dcl;
        // The standing winner/count are shared by all candidate priorities.
        // With zero or one candidate there is no ordering work to perform.
        let needs_priority = legal.len() > 1 && self.ordering == MoveOrdering::CaptureFirst && self.sh.straight_fast_paths();
        let table = if needs_priority {
            led.map(|q| {
                let mut best = None;
                let mut winner_at = 0;
                let mut count = 0;
                for (i, &p) in key.plays.iter().enumerate() {
                    let tile = Domino::from_index(usize::from(p)).expect("played tile");
                    let k = dcl.trick_key(tile, q);
                    count += tile.count();
                    // First strict maximum, matching Trick::winner.
                    if best.as_ref().is_none_or(|b| k > *b) {
                        best = Some(k);
                        winner_at = i;
                    }
                }
                let winner = Seat::from_index((usize::from(key.leader) + winner_at) % 4)
                    .expect("winner seat index");
                (
                    q,
                    best.expect("a led context implies a play"),
                    count,
                    winner.team() == self.viewer.team(),
                )
            })
        } else {
            None
        };
        let priority = |tile: Domino| {
            if !needs_priority {
                return 0;
            }
            if let Some((q, best, count, own_team)) = table {
                if dcl.trick_key(tile, q) > best {
                    1_000 + count + tile.count()
                } else if own_team {
                    20 + tile.count()
                } else {
                    10 - tile.count()
                }
            } else {
                // Leading: called tier first, then declaration rank.
                let k = dcl.trick_key(tile, dcl.led_context(tile));
                100 * k.tier as u32 + u32::from(k.rank.value())
            }
        };
        let order = &mut storage[..legal.len()];
        for (slot, tile) in order.iter_mut().zip(legal.iter()) {
            *slot = (priority(tile), tile);
        }
        order.sort_unstable_by_key(|&(p, t)| (std::cmp::Reverse(p), t.index()));
        order
    }

    /// The `solve_viewer` visit order (reorder-not-cull;
    /// CENSUS-RULINGS.md E-A15: the ORDER of evaluation is lawful to
    /// change, the SET is not): a canonical permutation of `legal` —
    /// priority descending, ascending tile index on exact ties, so the
    /// permutation itself is deterministic. Public so gates and probes
    /// exercise the one authority `solve_viewer` consumes.
    pub fn viewer_visit_order(
        &self,
        key: &Key,
        led: Option<Context>,
        legal: DominoSet,
    ) -> Vec<Domino> {
        let mut storage = [(0, Domino::ALL[0]); Domino::COUNT];
        self.viewer_visit_order_into(key, led, legal, &mut storage)
            .iter()
            .map(|&(_, tile)| tile)
            .collect()
    }

    fn solve_viewer(&self, key: &Key, led: Option<Context>) -> Option<u64> {
        let hand = self.viewer_hand0 & !key.played;
        let legal = legal_plays(self.sh.dcl, set_of(hand), led);
        // Reorder, never cull (E-A15): the same legal set in heuristic
        // order. This loop is value-only — max/min over children, no
        // action returned — so no tie-break is exposed; the order only
        // moves where the Boolean break lands.
        let mut storage = [(0, Domino::ALL[0]); Domino::COUNT];
        let order = self.viewer_visit_order_into(key, led, legal, &mut storage);
        self.local_viewer_legal
            .fetch_add(order.len() as u64, Ordering::Relaxed);
        let mut best: Option<u64> = None;
        let mass = self.alive_of(key.alive).len() as u64;
        for &(_, tile) in order {
            self.local_viewer_children.fetch_add(1, Ordering::Relaxed);
            let child = self.child_after_play(key, tile, key.alive);
            let v = self.solve_count(&child)?;
            let better = best
                .as_ref()
                .is_none_or(|b| if self.maximize { v > *b } else { v < *b });
            if better {
                let decided = if self.maximize { v == mass } else { v == 0 };
                best = Some(v);
                if decided {
                    break;
                }
            }
        }
        Some(best.expect("viewer always has a legal play"))
    }

    fn solve_field_dice(&self, key: &Key, seat: Seat, led: Option<Context>) -> Option<u64> {
        let alive = self.alive_of(key.alive);
        let mut record = None;
        if let Some(small) = &self.small_support {
            if !self.parallel || (cfg!(feature = "adaptive-parallel") && self.worlds.len() <= 2) {
                // The common eight-world modeled mind needs no bucket/list
                // allocation. Sample identity, tile visit order and mass remain
                // exactly the same as the general path below.
                let mut buckets = [0u8; 28];
                let mut occupied = 0u32;
                for sid in alive.iter() {
                    let hand = self.worlds[sid as usize][seat.index()] & !key.played;
                    let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
                    debug_assert!(lm != 0);
                    let choices = lm.count_ones();
                    let tile = if choices == 1 {
                        lm.trailing_zeros()
                    } else {
                        let rh = *record.get_or_insert_with(|| record_hash(key));
                        let idx = SplitMix64(self.seeds[sid as usize] ^ rh)
                            .below(u64::from(choices)) as u32;
                        nth_set_bit(lm, idx)
                    };
                    buckets[tile as usize] |= 1u8 << sid;
                    occupied |= 1u32 << tile;
                }
                let mut total = 0u64;
                let mut redistributed = 0;
                // Ascending set bits preserve the full scan's visit order.
                // A small support can occupy at most eight of the 28 buckets.
                while occupied != 0 {
                    let tile = occupied.trailing_zeros() as usize;
                    occupied &= occupied - 1;
                    let mask = buckets[tile];
                    redistributed += mask.count_ones() as usize;
                    let child = self.child_after_play(
                        key,
                        Domino::from_index(tile).expect("tile < 28"),
                        small.encode(mask),
                    );
                    total += self.solve_count(&child)?;
                }
                assert_eq!(redistributed, alive.len(), "field partition conservation");
                assert!(
                    total <= alive.len() as u64,
                    "success mass cannot exceed support mass"
                );
                return Some(total);
            }
        }
        let mut buckets: [Vec<u32>; 28] = std::array::from_fn(|_| Vec::new());
        for sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            let choices = lm.count_ones();
            // This Dice stream is local to this state/sample and discarded.
            // A sole legal tile needs neither a draw nor the record hash.
            let tile = if choices == 1 {
                lm.trailing_zeros()
            } else {
                let rh = *record.get_or_insert_with(|| record_hash(key));
                let idx =
                    SplitMix64(self.seeds[sid as usize] ^ rh).below(u64::from(choices)) as u32;
                nth_set_bit(lm, idx)
            };
            buckets[tile as usize].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    fn solve_field_policy(
        &self,
        key: &Key,
        seat: Seat,
        led: Option<Context>,
        k: usize,
    ) -> Option<u64> {
        let alive = self.alive_of(key.alive);
        #[cfg(feature = "fast-policy")]
        if let Some(small) = &self.small_support {
            if !self.parallel || (cfg!(feature = "adaptive-parallel") && self.worlds.len() <= 2) {
                // At most eight sample IDs. Preserve the serial reference's
                // first-seen modeled-hand order, then classify every sample
                // into ascending action buckets without heap containers.
                let mut hands = [0u32; 8];
                let mut legal = [0u32; 8];
                let mut choices = [0u8; 8];
                let mut distinct_len = 0usize;
                for sid in alive.iter() {
                    let hand = self.worlds[sid as usize][seat.index()] & !key.played;
                    let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
                    debug_assert_ne!(lm, 0);
                    if let Some(i) = (0..distinct_len).find(|&i| hands[i] == hand) {
                        debug_assert_eq!(legal[i], lm);
                    } else {
                        hands[distinct_len] = hand;
                        legal[distinct_len] = lm;
                        distinct_len += 1;
                    }
                }
                for i in 0..distinct_len {
                    let lm = legal[i];
                    choices[i] = if lm.count_ones() == 1 {
                        lm.trailing_zeros() as u8
                    } else {
                        self.pi(k, key, seat, hands[i], lm)?
                    };
                }
                let mut buckets = [0u8; 28];
                let mut occupied = 0u32;
                for sid in alive.iter() {
                    let hand = self.worlds[sid as usize][seat.index()] & !key.played;
                    let i = (0..distinct_len)
                        .find(|&i| hands[i] == hand)
                        .expect("every alive hand was classified");
                    let tile = usize::from(choices[i]);
                    buckets[tile] |= 1u8 << sid;
                    occupied |= 1u32 << tile;
                }
                let mut redistributed = 0usize;
                let mut total = 0u64;
                while occupied != 0 {
                    let tile = occupied.trailing_zeros() as usize;
                    occupied &= occupied - 1;
                    let mask = buckets[tile];
                    redistributed += mask.count_ones() as usize;
                    let child = self.child_after_play(
                        key,
                        Domino::from_index(tile).expect("tile < 28"),
                        small.encode(mask),
                    );
                    total += self.solve_count(&child)?;
                }
                assert_eq!(redistributed, alive.len(), "field partition conservation");
                assert!(
                    total <= alive.len() as u64,
                    "success mass cannot exceed support mass"
                );
                return Some(total);
            }
        }
        let mut per_sid: Vec<(u32, u32)> = Vec::with_capacity(alive.len());
        #[cfg(not(feature = "fast-policy"))]
        let mut distinct: HashMap<u32, u32> = HashMap::new();
        #[cfg(feature = "fast-policy")]
        let mut distinct: CacheMap<u32, u32> = CacheMap::default();
        for sid in alive.iter() {
            let hand = self.worlds[sid as usize][seat.index()] & !key.played;
            let lm = mask_of(legal_plays(self.sh.dcl, set_of(hand), led));
            debug_assert!(lm != 0);
            if lm.count_ones() > 1 {
                distinct.entry(hand).or_insert(lm);
            }
            per_sid.push((hand, lm));
        }
        #[cfg(all(feature = "parallel", not(feature = "fast-policy")))]
        if self.field_prewarm_parallel(key, alive.len(), distinct.len()) {
            let reqs: Vec<(u32, u32)> = distinct.iter().map(|(&h, &lm)| (h, lm)).collect();
            let alive_count = reqs
                .par_iter()
                .filter(|&&(hand, lm)| self.pi(k, key, seat, hand, lm).is_some())
                .count();
            if alive_count != reqs.len() {
                return None;
            }
        }
        #[cfg(feature = "fast-policy")]
        let precomputed = {
            #[cfg(feature = "parallel")]
            if self.field_prewarm_parallel(key, alive.len(), distinct.len()) {
                let reqs: Vec<(u32, u32)> = distinct.iter().map(|(&h, &lm)| (h, lm)).collect();
                let choices: Option<Vec<(u32, u8)>> = reqs
                    .par_iter()
                    .map(|&(hand, lm)| Some((hand, self.pi(k, key, seat, hand, lm)?)))
                    .collect();
                // Keep the completed answers instead of immediately asking
                // the shared policy cache for each of them a second time.
                for (hand, tile) in choices? {
                    distinct.insert(hand, u32::from(tile));
                }
                true
            } else {
                false
            }
            #[cfg(not(feature = "parallel"))]
            {
                false
            }
        };
        let mut buckets: [Vec<u32>; 28] = std::array::from_fn(|_| Vec::new());
        for (i, sid) in alive.iter().enumerate() {
            let (hand, lm) = per_sid[i];
            let tile = if lm.count_ones() == 1 {
                lm.trailing_zeros() as u8
            } else {
                #[cfg(feature = "fast-policy")]
                if precomputed {
                    *distinct
                        .get(&hand)
                        .expect("completed policy for each distinct hand") as u8
                } else {
                    self.pi(k, key, seat, hand, lm)?
                }
                #[cfg(not(feature = "fast-policy"))]
                self.pi(k, key, seat, hand, lm)?
            };
            buckets[usize::from(tile)].push(sid);
        }
        self.combine_buckets(key, alive.len(), buckets)
    }

    fn combine_buckets(&self, key: &Key, alive_len: usize, buckets: [Vec<u32>; 28]) -> Option<u64> {
        let mut children: Vec<Key> = Vec::with_capacity(alive_len.min(28));
        let mut redistributed: usize = 0;

        for (tile, bucket) in buckets.into_iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }

            redistributed += bucket.len();
            let child_alive = self.intern(bucket);
            children.push(self.child_after_play(
                key,
                Domino::from_index(tile).expect("tile < 28"),
                child_alive,
            ));
        }
        assert_eq!(redistributed, alive_len, "field partition conservation");
        let serial = || -> Option<u64> {
            children
                .iter()
                .try_fold(0, |total, child| Some(total + self.solve_count(child)?))
        };
        #[cfg(feature = "parallel")]
        let total = if self.parallel && children.len() > 1
            // Preserve the original width until only one or two sample IDs
            // remain; those descendants finish serially on this Solver.
            && (!cfg!(feature = "adaptive-parallel") || alive_len > 2)
        {
            let vals: Vec<Option<u64>> = children
                .par_iter()
                .map(|child| self.solve_count(child))
                .collect();
            vals.into_iter()
                .try_fold(0u64, |total, v| Some(total + v?))?
        } else {
            serial()?
        };

        #[cfg(not(feature = "parallel"))]
        let total = serial()?;
        assert!(
            total <= alive_len as u64,
            "success mass cannot exceed support mass"
        );
        Some(total)
    }

    fn hand_sizes_at(&self, key: &Key) -> [usize; 4] {
        self.sh.contract.sizes(key, self.sh.boundary_played, self.sh.boundary_hand_size)
    }

    /// The level-k policy at a modeled seat's information state (pure in
    /// (k, PiKey); level-0 seeding bit-identical across the stack).
    fn pi(&self, k: usize, key: &Key, seat: Seat, hand: u32, legal_mask: u32) -> Option<u8> {
        self.check_belief_key(key);
        #[cfg(feature = "bypass-l0-cache")]
        if k == 0 && self.sh.straight_fast_paths() {
            if let Some(choice) = self.uncached_l0(key, seat, hand, legal_mask) {
                return choice;
            }
        }

        let cache_key = PolicyKey::from_state(
            key,
            seat,
            hand,
            self.sh.inner_belief == InnerBelief::VoidsCounted,
        );
        let kb = k as u8;
        if let Some(&t) = self
            .sh
            .pi_shard(kb, &cache_key)
            .lock()
            .expect("pi shard poisoned")
            .get(&(kb, cache_key))
        {
            return Some(t);
        }

        if self.sh.deadline.passed() {
            self.sh.dead.store(true, Ordering::Relaxed);
            return None;
        }
        self.sh.pi_calls.fetch_add(1, Ordering::Relaxed);
        self.sh.pi_calls_by_level[k].fetch_add(1, Ordering::Relaxed);
        let n_k = self.sh.n_inner[k];
        let sizes = self.hand_sizes_at(key);
        let level_tag = if k == 0 { 0 } else { mix(0x4C32 ^ k as u64) };
        let mut rng = SplitMix64(
            INNER_SEED
                ^ level_tag
                ^ mix(seat.index() as u64)
                ^ mix(u64::from(hand))
                ^ record_hash(key),
        );
        let maximize = seat.team() == Team::T1;
        #[cfg(feature = "stack-dice")]
        if k == 0 && self.sh.straight_fast_paths() && (1..=8).contains(&n_k) && self.sh.inner_belief == InnerBelief::Voidless {
            let choice = compact_dice::prepared_choice(
                &self.sh,
                key,
                seat.index(),
                hand,
                maximize,
                sizes,
                n_k,
                &mut rng,
                legal_mask,
            );
            let choice = match choice {
                Some(tile)
                    if !self.sh.dead.load(Ordering::Relaxed) && !self.sh.deadline.passed() =>
                {
                    tile
                }
                _ => {
                    self.sh.dead.store(true, Ordering::Relaxed);
                    return None;
                }
            };
            self.sh
                .pi_shard(kb, &cache_key)
                .lock()
                .expect("pi shard poisoned")
                .insert((kb, cache_key), choice);
            return Some(choice);
        }
        let root = Key {
            voids: key.voids,
            played: key.played,
            leader: key.leader,
            plays: key.plays.clone(),
            banked_t1: key.banked_t1,
            banked_t0: key.banked_t0,
            alive: 0,
        };
        // L0 retains its fixed Dice response. L1 and higher use the same
        // selector as the real player, at their independently frozen budgets.
        let rule = if k == 0 {
            selection::Rule::Fixed
        } else {
            self.sh.modeled_selection
        };
        #[cfg(feature = "bounded-choice")]
        if k == 0 && self.sh.straight_fast_paths() && (1..=8).contains(&n_k) {
            let worlds = self.sh.inner_belief.sample(
                self.sh.dcl,
                seat,
                hand,
                key,
                sizes,
                n_k,
                &mut rng,
                self.sh.deadline,
            )?;
            self.sh.inner_worlds_by_level[k].fetch_add(n_k as u64, Ordering::Relaxed);
            let seeds = (0..n_k).map(|_| rng.next_u64()).collect();
            let dice = Solver::new(
                Arc::clone(&self.sh),
                seat,
                hand,
                maximize,
                worlds,
                seeds,
                Field::Dice,
            )
            .with_ordering(self.ordering);
            let choice = compact_dice::fixed_choice(&dice, &root, &mask_bits(legal_mask));
            dice.flush_nodes();
            let choice = match choice {
                Some(tile)
                    if !self.sh.dead.load(Ordering::Relaxed) && !self.sh.deadline.passed() =>
                {
                    tile
                }
                _ => {
                    self.sh.dead.store(true, Ordering::Relaxed);
                    return None;
                }
            };
            self.sh
                .pi_shard(kb, &cache_key)
                .lock()
                .expect("pi shard poisoned")
                .insert((kb, cache_key), choice);
            return Some(choice);
        }
        #[cfg(feature = "fixed-policy-choice")]
        if k > 0 && self.sh.straight_fast_paths() && rule == selection::Rule::Fixed {
            let worlds = self.sh.inner_belief.sample(
                self.sh.dcl,
                seat,
                hand,
                key,
                sizes,
                n_k,
                &mut rng,
                self.sh.deadline,
            )?;
            self.sh.inner_worlds_by_level[k].fetch_add(n_k as u64, Ordering::Relaxed);
            let modeled = Solver::new(
                Arc::clone(&self.sh),
                seat,
                hand,
                maximize,
                worlds,
                Vec::new(),
                Field::Level(k - 1),
            )
            .with_ordering(self.ordering);
            let candidates = mask_bits(legal_mask);
            #[cfg(feature = "compact-policy")]
            let choice = compact_policy::fixed_choice(&modeled, &root, &candidates)
                .unwrap_or_else(|| modeled.fixed_policy_choice(&root, &candidates));
            #[cfg(not(feature = "compact-policy"))]
            let choice = modeled.fixed_policy_choice(&root, &candidates);
            let choice = match choice {
                Some(tile)
                    if !self.sh.dead.load(Ordering::Relaxed) && !self.sh.deadline.passed() =>
                {
                    tile
                }
                _ => {
                    self.sh.dead.store(true, Ordering::Relaxed);
                    return None;
                }
            };
            self.sh
                .pi_shard(kb, &cache_key)
                .lock()
                .expect("pi shard poisoned")
                .insert((kb, cache_key), choice);
            return Some(choice);
        }
        let chosen = selection::select(
            rule,
            &mask_bits(legal_mask),
            maximize,
            n_k,
            |tiles, n, _| {
                let worlds = self
                    .sh
                    .inner_belief
                    .sample(
                        self.sh.dcl,
                        seat,
                        hand,
                        key,
                        sizes,
                        n,
                        &mut rng,
                        self.sh.deadline,
                    )
                    .ok_or(())?;
                self.sh.inner_worlds_by_level[k].fetch_add(n as u64, Ordering::Relaxed);
                let (field, seeds) = if k == 0 {
                    (Field::Dice, (0..n).map(|_| rng.next_u64()).collect())
                } else {
                    (Field::Level(k - 1), Vec::new())
                };
                Solver::new(
                    Arc::clone(&self.sh),
                    seat,
                    hand,
                    maximize,
                    worlds,
                    seeds,
                    field,
                )
                .with_ordering(self.ordering)
                .action_values(&root, tiles)
                .ok_or(())
            },
        );
        let choice = match chosen {
            Ok(result) if !self.sh.deadline.passed() => result.choice,
            _ => {
                self.sh.dead.store(true, Ordering::Relaxed);
                return None;
            }
        };
        self.sh
            .pi_shard(kb, &cache_key)
            .lock()
            .expect("pi shard poisoned")
            .insert((kb, cache_key), choice);
        Some(choice)
    }

    /// Fixed modeled-mind choice from one ordered sampled bundle. The public
    /// root still uses action_values, which reports every exact rational value.
    #[cfg(feature = "fixed-policy-choice")]
    fn fixed_policy_choice(&self, key: &Key, tiles: &[u8]) -> Option<u8> {
        assert!(!tiles.is_empty());
        debug_assert!(tiles.windows(2).all(|w| w[0] < w[1]));
        let result = (|| {
            let mass = self.alive_of(key.alive).len() as u64;
            let mut best: Option<(u8, u64)> = None;
            for &id in tiles {
                if self.sh.deadline.passed() {
                    return None;
                }
                let tile = Domino::from_index(id as usize).expect("legal tile");
                let child = self.child_after_play(key, tile, key.alive);
                let value = self.solve_count(&child)?;
                if best.as_ref().is_none_or(|&(_, prior)| {
                    if self.maximize {
                        value > prior
                    } else {
                        value < prior
                    }
                }) {
                    best = Some((id, value));
                }
                // Strictly unbeatable value; ascending candidates retain the
                // first tile on every tie, including unvisited later ties.
                if (self.maximize && value == mass) || (!self.maximize && value == 0) {
                    break;
                }
            }
            if self.sh.deadline.passed() {
                None
            } else {
                best.map(|(tile, _)| tile)
            }
        })();
        self.flush_nodes();
        result
    }

    /// Evaluate a complete ordered root comparison on this solver's common
    /// worlds. Statistics are flushed on success AND refusal.
    pub fn action_values(&self, key: &Key, tiles: &[u8]) -> Option<selection::Values> {
        #[cfg(feature = "compact-dice")]
        if let Some(values) = compact_dice::action_values(self, key, tiles) {
            // The compact recurrence counts successful sample IDs. Keep the
            // public exact rational boundary and the caller's candidate order.
            let result = values.map(|counts| {
                let denominator = BigInt::from(self.worlds.len());
                tiles
                    .iter()
                    .copied()
                    .zip(counts)
                    .map(|(tile, count)| {
                        (
                            tile,
                            BigRational::new(BigInt::from(count), denominator.clone()),
                        )
                    })
                    .collect()
            });
            self.flush_nodes();
            return result;
        }
        #[cfg(all(feature = "parallel", feature = "parallel-roots"))]
        if self.parallel && tiles.len() > 1 {
            // Each legal root candidate uses the same frozen bundle. Indexed
            // parallel collection preserves the exact caller order; memo and
            // policy tables already synchronize their pure completed values.
            let values: Vec<Option<(u8, BigRational)>> = tiles
                .par_iter()
                .map(|&id| {
                    if self.sh.deadline.passed() {
                        return None;
                    }
                    let tile = Domino::from_index(id as usize).expect("legal tile");
                    let child = self.child_after_play(key, tile, 0);
                    Some((id, self.solve(&child)?))
                })
                .collect();
            let result = if self.sh.deadline.passed() {
                None
            } else {
                values.into_iter().collect()
            };
            self.flush_nodes();
            return result;
        }
        let result = (|| {
            let mut values = Vec::with_capacity(tiles.len());
            for &id in tiles {
                if self.sh.deadline.passed() {
                    return None;
                }
                let tile = Domino::from_index(id as usize).expect("legal tile");
                let child = self.child_after_play(key, tile, 0);
                values.push((id, self.solve(&child)?));
            }
            if self.sh.deadline.passed() {
                None
            } else {
                Some(values)
            }
        })();
        self.flush_nodes();
        result
    }

    /// Public exposure of the modeled level-k field policy at an information
    /// state — exactly the `pi` the solver consults internally (pure in
    /// (k, PiKey), cached on the `Shared`). For replay/audit tooling (the
    /// tilt audit): one authority, never a copy. Note the level-0 field is
    /// deterministic in (seat, hand, record) — there is no external tape.
    pub fn modeled_choice(
        &self,
        k: usize,
        key: &Key,
        seat: Seat,
        hand: u32,
        legal_mask: u32,
    ) -> Option<u8> {
        self.pi(k, key, seat, hand, legal_mask)
    }
}

/// A declared belief frame whose lawful-completion fiber is EMPTY: no
/// deal of the unseen pool gives each other seat its declared number of
/// tiles while respecting that seat's deduced voids. The shuffle-and-reject
/// sampler's acceptance region is empty there, so it is a refusal, never a
/// cost — the frame has no answer to sample.
///
/// The refusal carries the frame verbatim plus the blocking set that
/// decides it, so a caller can name exactly which seats' voids collided
/// with which tiles without re-deriving anything.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfeasibleFrame {
    pub viewer: usize,
    pub viewer_hand: u32,
    pub played: u32,
    pub sizes: [usize; 4],
    pub voids: [u32; 4],
    /// The seats the confined tiles must fit into.
    pub blocking_seats: Vec<usize>,
    /// Unseen tiles whose only lawful destinations lie in
    /// `blocking_seats` — every other hidden seat is void in all of them.
    pub confined: u32,
    /// The room those seats have, plus the pool this frame never deals.
    pub room: usize,
}

impl fmt::Display for InfeasibleFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unsatisfiable belief frame: viewer S{} hand {:07x} played {:07x} \
             sizes {:?} voids [{:07x}, {:07x}, {:07x}, {:07x}] — {} unseen tiles \
             ({:07x}) are confined to seats {:?}, which hold room for {}",
            self.viewer,
            self.viewer_hand,
            self.played,
            self.sizes,
            self.voids[0],
            self.voids[1],
            self.voids[2],
            self.voids[3],
            self.confined.count_ones(),
            self.confined,
            self.blocking_seats,
            self.room,
        )
    }
}

/// Exact feasibility of a declared belief frame, by counting alone.
///
/// The sampler deals the unseen pool out to the seats other than `viewer`,
/// `sizes[s]` tiles apiece, rejecting any deal that hands a seat one of its
/// deduced voids. That is a degree-constrained bipartite assignment (tiles
/// to seats), so Hall's condition in its deficiency form decides it
/// exactly: the frame is feasible iff, for every subset `S` of the other
/// seats, the unseen tiles that NO seat outside `S` may hold do not
/// outnumber the room inside `S` — where the room counts `S`'s declared
/// sizes plus the leftover the sampler's prefix slicing never deals.
///
/// Three other seats means eight subsets, so this is a fixed amount of
/// integer counting with no search — and it is the same acceptance region
/// the sampler tests one draw at a time, not a conservative approximation
/// of it (the equivalence against exhaustive exact-partition search is
/// gated in `tests/solver_sigma1_repair.rs`).
pub fn belief_frame_feasibility(
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    voids: [u32; 4],
) -> Result<(), InfeasibleFrame> {
    let unseen = FULL_MASK & !played & !viewer_hand;
    let others: Vec<usize> = (0..4).filter(|&s| s != viewer).collect();
    let need: usize = others.iter().map(|&s| sizes[s]).sum();
    let pool = unseen.count_ones() as usize;
    assert!(
        need <= pool,
        "a belief frame deals at most the unseen pool: {need} tiles wanted, \
         {pool} unseen"
    );
    let leftover = pool - need;
    for subset in 0u32..(1u32 << others.len()) {
        let mut room = leftover;
        // A tile is confined to `subset` when every seat OUTSIDE it is
        // void in that tile; the intersection over an empty complement is
        // all tiles, which is the trivially satisfied subset.
        let mut confined = unseen;
        for (slot, &seat) in others.iter().enumerate() {
            if subset & (1 << slot) == 0 {
                confined &= voids[seat];
            } else {
                room += sizes[seat];
            }
        }
        if confined.count_ones() as usize > room {
            return Err(InfeasibleFrame {
                viewer,
                viewer_hand,
                played,
                sizes,
                voids,
                blocking_seats: others
                    .iter()
                    .enumerate()
                    .filter(|(slot, _)| subset & (1 << slot) != 0)
                    .map(|(_, &seat)| seat)
                    .collect(),
                confined,
                room,
            });
        }
    }
    Ok(())
}

/// Draw from a belief frame carrying NO deduced voids — the auction and
/// pre-play frames, where nothing has yet been observed to constrain any
/// seat.
///
/// TOTAL BY CONSTRUCTION, and that is the point of having it. With every
/// void mask zero the sampler's rejection test `w[s] & voids[s] != 0`
/// cannot fire, so the first shuffle is accepted and the acceptance region
/// is the entire deal space; [`belief_frame_feasibility`] cannot refuse
/// such a frame. The proof lives here, once, instead of as an `expect` at
/// each of the dozen call sites that draw open frames — which is what lets
/// the live player's auction path carry no error branch at all, rather
/// than one it can argue is unreachable. The remaining precondition is
/// arity, asserted inside the sampler exactly as it was before the repair:
/// the declared sizes must fit the unseen pool.
pub fn sample_open_belief(
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    n: usize,
    rng: &mut SplitMix64,
) -> Vec<[u32; 4]> {
    sample_belief(viewer, viewer_hand, played, sizes, [0; 4], n, rng)
        .expect("a frame with no deduced voids accepts every deal")
}

/// Void-conditioned belief sampler: uniform on the lawful-completion fiber
/// by shuffle-and-reject (SCENARIO-PLAYER.md §4.2).
///
/// TERMINATION. The rejection loop is unbounded by design — it is the
/// uniformity of the draw — so it terminates exactly when the acceptance
/// region is nonempty. [`belief_frame_feasibility`] decides that first, by
/// counting, and an empty region returns [`InfeasibleFrame`] instead of
/// spinning. The precheck consumes no randomness and rejects no feasible
/// frame, so on a feasible frame the draw sequence is bit-identical to the
/// unguarded loop's: same stream, same accept path, same worlds
/// (gated against a before-side capture in `tests/solver_sigma1_repair.rs`).
pub fn sample_belief(
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    voids: [u32; 4],
    n: usize,
    rng: &mut SplitMix64,
) -> Result<Vec<[u32; 4]>, InfeasibleFrame> {
    belief_frame_feasibility(viewer, viewer_hand, played, sizes, voids)?;
    let unseen = FULL_MASK & !played & !viewer_hand;
    let mut tiles = mask_bits(unseen);
    let others: Vec<usize> = (0..4).filter(|&s| s != viewer).collect();
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    let mut out: Vec<[u32; 4]> = Vec::with_capacity(n);
    while out.len() < n {
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mut w = [0u32; 4];
        w[viewer] = viewer_hand;
        let mut off = 0;
        let mut ok = true;
        for &s in &others {
            w[s] = mask_slice(&tiles[off..off + sizes[s]]);
            off += sizes[s];
            if w[s] & voids[s] != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(w);
        }
    }
    Ok(out)
}

/// Basis points of an exact probability, integer-rounded toward zero.
pub fn bp(v: &BigRational) -> i64 {
    let scaled = (v * BigRational::from_integer(BigInt::from(10_000))).to_integer();
    scaled.to_string().parse().unwrap_or(0)
}

/// Arena declaration id → walt-core Decl (0..=6 pip trump, 7 doubles-trump,
/// 9 no-trump — the mk5 arena's ids, shared by walt_bridge and the WASM
/// surface).
pub fn decl_of(arena_id: usize) -> Decl {
    match arena_id {
        p @ 0..=6 => Decl::STRAIGHT[p],
        7 => Decl::DoublesTrump,
        8 => Decl::DoublesSuit,
        9 => Decl::NoTrump,
        other => panic!("declaration id {other} is outside the suit algebra"),
    }
}

pub fn arena_decl_id(d: Decl) -> usize {
    match d {
        Decl::PipTrump(p) => usize::from(p.value()),
        Decl::DoublesTrump => 7,
        Decl::DoublesSuit => 8,
        Decl::NoTrump => 9,
    }
}

/// The replayed public record, in INTERNAL seat labels (arena + r mod 4,
/// with r chosen so the bidding team is internal T1 = seats 1,3). The
/// audited rotation of walt_bridge.rs.
pub struct Replayed {
    pub r: usize,
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
    pub voids: [u32; 4],
    pub trick_start_played: u32,
    pub completed: usize,
}

/// Replay a chronological arena-frame record `(actor, tile)*` into internal
/// labels: turn order asserted, voids derived from failures to follow,
/// banked totals per completed trick. `bidder_arena` leads trick one.
pub fn replay(dcl: Decl, bidder_arena: usize, pairs: &[(usize, usize)]) -> Replayed {
    replay_contract(dcl, bidder_arena, pairs, false)
}

pub fn replay_contract(dcl: Decl, bidder_arena: usize, pairs: &[(usize, usize)], nello: bool) -> Replayed {
    let r = if bidder_arena.is_multiple_of(2) { 1 } else { 0 };
    let mut st = Replayed {
        r,
        played: 0,
        leader: ((bidder_arena + r) % 4) as u8,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        voids: [0; 4],
        trick_start_played: 0,
        completed: 0,
    };
    let contract = if nello { Contract::Nello { declarer: Seat::from_index((bidder_arena + r) % 4).unwrap() } }
        else { Contract::Straight { bid: 42 } };
    for &(actor_arena, tile_id) in pairs {
        let actor = (actor_arena + r) % 4;
        let expect = contract.actor(st.leader as usize, st.plays.len()).index();
        assert_eq!(actor, expect, "history follows turn order");
        let tile = Domino::from_index(tile_id).expect("tile id 0..28");
        assert_eq!(st.played & bit(tile), 0, "tile played once");
        if let Some(&led_id) = st.plays.first() {
            let led = dcl.led_context(Domino::from_index(usize::from(led_id)).expect("led tile"));
            if !dcl.follows(tile, led) {
                st.voids[actor] |= mask_of(dcl.effective_incidence(led));
            }
        }
        st.played |= bit(tile);
        st.plays.push(tile.index() as u8);
        if st.plays.len() == contract.trick_size() {
            let winner = contract.winner(dcl, st.leader as usize, &st.plays);
            let pts = 1 + st.plays.iter().map(|&t| Domino::from_index(t as usize).unwrap().count() as u8).sum::<u8>();
            if winner.team() == Team::T1 {
                st.banked_t1 += pts;
            } else {
                st.banked_t0 += pts;
            }
            st.leader = winner.index() as u8;
            st.plays.clear();
            st.completed += 1;
            st.trick_start_played = st.played;
        }
    }
    st
}

/// Why a level-1 evaluation produced no estimates. The two reasons are
/// kept apart on purpose: a deadline says the answer was too expensive
/// here, an infeasible frame says there is no answer to compute.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Level1Refusal {
    /// The per-move wall-clock budget expired mid-evaluation. This is the
    /// pre-repair `None`, renamed and nothing more.
    Deadline,
    /// The declared belief frame has an empty lawful-completion fiber, so
    /// the sampler has nothing to draw ([`belief_frame_feasibility`]).
    InfeasibleFrame(InfeasibleFrame),
}

impl fmt::Display for Level1Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level1Refusal::Deadline => write!(f, "the per-move deadline expired"),
            Level1Refusal::InfeasibleFrame(frame) => write!(f, "{frame}"),
        }
    }
}

impl From<InfeasibleFrame> for Level1Refusal {
    fn from(frame: InfeasibleFrame) -> Level1Refusal {
        Level1Refusal::InfeasibleFrame(frame)
    }
}

/// The level-1 evaluation with saturation-tie refinement (the playtable.rs
/// policy — one authority, shared by webtable and the WASM surface).
/// Returns every legal option's estimate under the contract's bid
/// thresholds, or the typed reason there are none ([`Level1Refusal`]).
///
/// STILL TRIPLICATED (walt_bridge.rs, playtable.rs) — a named debt the
/// σ1-repair slice deliberately did not pay. Only the sampler's refusal
/// travels through here; the bodies are otherwise untouched.
#[allow(clippy::too_many_arguments)]
pub fn level1_evaluate(
    dcl: Decl,
    bid: u8,
    seat: Seat,
    hand: u32,
    legal: u32,
    key: &Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    n_outer: usize,
    n0: usize,
    per_move_secs: u64,
    rng: &mut SplitMix64,
) -> Result<Vec<(u8, BigRational)>, Level1Refusal> {
    let deadline = Deadline::after(Duration::from_secs(per_move_secs));
    let maximize = seat.team() == Team::T1;
    let evaluate = |tiles: &[u8], n: usize, rng: &mut SplitMix64| {
        let worlds = sample_belief(seat.index(), hand, key.played, sizes, voids, n, rng)?;
        let sh = Arc::new(Shared::new(
            dcl,
            bid,
            vec![n0],
            trick_start_played,
            boundary_hand_size,
            deadline,
        ));
        let solver = Solver::new(
            sh,
            seat,
            hand,
            maximize,
            worlds,
            Vec::new(),
            Field::Level(0),
        )
        .parallel();
        let mut out: Vec<(u8, BigRational)> = Vec::new();
        for &t in tiles {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(key, tile, 0);
            match solver.solve(&child) {
                Some(v) => out.push((t, v)),
                None => return Err(Level1Refusal::Deadline),
            }
        }
        Ok(out)
    };
    selection::select(
        selection::Rule::Refine,
        &mask_bits(legal),
        maximize,
        n_outer,
        |tiles, n, _| evaluate(tiles, n, rng),
    )
    .map(|result| result.values)
}

/// Cross-fiber pricing (the cheap first-order UI detector of
/// LEVEL2-PROBE.md): price a to-act seat's options from ANOTHER seat's
/// fiber — same machinery, different root viewer, no new mathematics.
/// Worlds are drawn from the viewer's void-conditioned lawful-completion
/// fiber (fiber = support of the information state, never a belief); each
/// option `t` is priced as the solver value of the child after the actor
/// plays `t`, over the alive subset of worlds in which that play is
/// lawful (the actor's world hand holds `t` and `t` is legal there — the
/// lawfulness conditioning of SCENARIO-PLAYER.md §4, not policy-Bayes).
/// The root viewer's own future decisions best-respond for its team; the
/// actor and every other seat are modeled level-0 minds, exactly as in
/// `level1_evaluate` — so a row where this column and the actor's own
/// column disagree is the human-visible flag of information asymmetry.
///
/// Returns `(tile, price, supporting-world count)` per option; a tile the
/// sampled fiber cannot realize prices `None` at support 0. Single common
/// sample across options, no saturation-tie refinement — this is a
/// pricing panel, not a decision procedure. Outer `None` iff the deadline
/// died mid-evaluation. Exploratory tier; estimates, never receipts.
#[allow(clippy::too_many_arguments)]
pub fn viewer_fiber_evaluate(
    dcl: Decl,
    bid: u8,
    actor: Seat,
    viewer: Seat,
    viewer_hand: u32,
    options: &[u8],
    key: &Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    n_outer: usize,
    n0: usize,
    per_move_secs: u64,
    rng: &mut SplitMix64,
) -> Result<Vec<(u8, Option<BigRational>, usize)>, Level1Refusal> {
    let deadline = Deadline::after(Duration::from_secs(per_move_secs));
    let maximize = viewer.team() == Team::T1;
    let worlds = sample_belief(
        viewer.index(),
        viewer_hand,
        key.played,
        sizes,
        voids,
        n_outer,
        rng,
    )?;
    let led: Option<Context> = key
        .plays
        .first()
        .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led tile")));
    let sh = Arc::new(Shared::new(
        dcl,
        bid,
        vec![n0],
        trick_start_played,
        boundary_hand_size,
        deadline,
    ));
    let solver = Solver::new(
        sh,
        viewer,
        viewer_hand,
        maximize,
        worlds,
        Vec::new(),
        Field::Level(0),
    )
    .parallel();
    let mut out: Vec<(u8, Option<BigRational>, usize)> = Vec::with_capacity(options.len());
    for &t in options {
        let tm = 1u32 << t;
        let support: Vec<u32> = (0..solver.worlds.len() as u32)
            .filter(|&sid| {
                let hand_w = solver.worlds[sid as usize][actor.index()] & !key.played;
                hand_w & tm != 0 && mask_of(legal_plays(dcl, set_of(hand_w), led)) & tm != 0
            })
            .collect();
        if support.is_empty() {
            out.push((t, None, 0));
            continue;
        }
        let n_sup = support.len();
        let alive = solver.intern(support);
        let tile = Domino::from_index(usize::from(t)).expect("tile < 28");
        let child = solver.child_after_play(key, tile, alive);
        match solver.solve(&child) {
            Some(v) => out.push((t, Some(v), n_sup)),
            None => {
                solver.flush_nodes();
                return Err(Level1Refusal::Deadline);
            }
        }
    }
    solver.flush_nodes();
    Ok(out)
}

/// Argmax (or argmin for T0 seats) over evaluated options, first-listed on
/// exact ties — the ascending-tile-order convention of the whole stack.
pub fn best_of(opts: &[(u8, BigRational)], maximize: bool) -> u8 {
    opts.iter()
        .cloned()
        .reduce(|best, cand| {
            let better = if maximize {
                cand.1 > best.1
            } else {
                cand.1 < best.1
            };
            if better {
                cand
            } else {
                best
            }
        })
        .expect("legal play")
        .0
}

// ---------------------------------------------------------------------------
// The racing evaluator (EXPLORATORY) — the tilt audit applied to the seat.
//
// Signed-pivotal geometry (math/signed_pivotal_geometry_v0.1.md, adjudicated
// CENSUS-RULINGS.md SP-A1..SP-A12): for two candidate root actions replayed
// on the SAME worlds, only pivotal worlds (exactly one candidate succeeds)
// carry information, and the winner's sign settles after enough pivotal
// observations lean one way. `level1_race` races every legal root action on
// a paired world panel: each candidate's continuation is played by the
// modeled level-1 mind (`modeled_choice(1, ..)` — cached, state-pure, no
// tape), the field by level-0 minds, and rivals are eliminated at block
// checkpoints by an exact binomial sign test against the current leader.
// Deterministic per (request, rng): replays consume no randomness at all.
//
// Semantics differ from `level1_evaluate`: this estimates the value OF THE
// FROZEN CONTINUATION POLICY per action (a lower-witness-shaped object, the
// thing the seat will actually do), not the bundle-coupled solver value.
// Exploratory play policy — never a receipt, never quotable above tier.
// ---------------------------------------------------------------------------

/// Exactly decide P(Bin(k, 1/2) >= m) <= delta_num/delta_den, no floats:
/// sum_{x=m}^{k} C(k,x) * delta_den <= delta_num * 2^k.
fn binom_tail_leq(k: usize, m: usize, delta_num: u64, delta_den: u64) -> bool {
    use num_bigint::BigUint;
    assert!(m <= k, "tail start within support");
    let mut c = BigUint::from(1u32); // C(k, k)
    let mut tail = BigUint::from(0u32);
    let mut x = k;
    loop {
        tail += &c;
        if x == m {
            break;
        }
        // C(k, x-1) = C(k, x) * x / (k - x + 1)
        c = c * BigUint::from(x as u64) / BigUint::from((k - x + 1) as u64);
        x -= 1;
    }
    tail * BigUint::from(delta_den) <= BigUint::from(delta_num) * (BigUint::from(1u32) << k)
}

/// Step a Key forward, preserving the selected void-tracking semantics.
pub(crate) fn key_step(key: &mut Key, dcl: Decl, tile: Domino) {
    key.voids = inner_belief::after_play(key, dcl, tile);
    key.plays.push(tile.index() as u8);
    key.played |= bit(tile);
    if key.plays.len() == 4 {
        let doms = [
            Domino::from_index(usize::from(key.plays[0])).expect("p0"),
            Domino::from_index(usize::from(key.plays[1])).expect("p1"),
            Domino::from_index(usize::from(key.plays[2])).expect("p2"),
            Domino::from_index(usize::from(key.plays[3])).expect("p3"),
        ];
        let leader = Seat::from_index(usize::from(key.leader)).expect("leader");
        let trick = Trick::new(leader, doms).expect("distinct tiles");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        if winner.team() == Team::T1 {
            key.banked_t1 += pts;
        } else {
            key.banked_t0 += pts;
        }
        key.leader = winner.index() as u8;
        key.plays.clear();
    }
}

/// Replay one world to terminal from `root`: the focal seat plays `first`,
/// then its continuation is the modeled level-1 mind; other seats are
/// level-0 minds. `world` gives every seat's remaining hand at the root
/// (world[viewer] must equal the viewer's true remaining hand). Returns
/// whether T1 made the bid, or None on deadline death.
fn race_replay(
    host: &Solver,
    dcl: Decl,
    bid: u8,
    viewer: usize,
    root: &Key,
    world: [u32; 4],
    first: u8,
) -> Option<bool> {
    let mut key = Key {
        voids: root.voids,
        played: root.played,
        leader: root.leader,
        plays: root.plays.clone(),
        banked_t1: root.banked_t1,
        banked_t0: root.banked_t0,
        alive: 0,
    };
    let mut rem = world;
    let mut forced_first = Some(first);
    while key.played != FULL_MASK {
        let seat_idx = (usize::from(key.leader) + key.plays.len()) % 4;
        let hand = rem[seat_idx];
        let led: Option<Context> = key
            .plays
            .first()
            .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led")));
        let legal = mask_of(legal_plays(dcl, set_of(hand), led));
        let choice: u8 = if seat_idx == viewer {
            if let Some(f) = forced_first.take() {
                debug_assert!(legal & (1u32 << f) != 0, "forced root action is legal");
                f
            } else if legal.count_ones() == 1 {
                legal.trailing_zeros() as u8
            } else {
                let seat = Seat::from_index(seat_idx).expect("seat");
                host.modeled_choice(1, &key, seat, hand, legal)?
            }
        } else if legal.count_ones() == 1 {
            legal.trailing_zeros() as u8
        } else {
            let seat = Seat::from_index(seat_idx).expect("seat");
            host.modeled_choice(0, &key, seat, hand, legal)?
        };
        let tile = Domino::from_index(usize::from(choice)).expect("tile");
        rem[seat_idx] &= !bit(tile);
        key_step(&mut key, dcl, tile);
    }
    Some(key.banked_t1 >= bid)
}

/// Race outcome: the chosen tile plus the evidence that settled it.
pub struct RaceReport {
    pub choice: u8,
    /// Worlds actually consumed (== the panel prefix every LIVE candidate
    /// was replayed on; eliminated candidates stopped at their exit).
    pub worlds_used: usize,
    /// (tile, successes over its replayed prefix, prefix length).
    pub tally: Vec<(u8, u32, u32)>,
    /// (tile, worlds seen when eliminated).
    pub eliminated: Vec<(u8, usize)>,
}

/// The racing decision procedure. Draws up to `n_max` belief worlds (the
/// only randomness — replays are deterministic), replays every live
/// candidate on each world in blocks of `RACE_BLOCK`, and at each block
/// boundary eliminates rivals whose pivotal deficit against the current
/// leader is settled by an exact binomial sign test at level
/// RACE_DELTA = 1/128 per comparison (predeclared checkpoint schedule —
/// the O14 shape; per-decision heuristic, no family-wise claim). Ends when
/// one candidate survives or the panel is exhausted; final choice is the
/// success argmax with the stack's lowest-tile tie convention.
///
/// A candidate's "success" is make for T1 seats and set for T0 seats, so
/// the argmax is always a maximization.
#[allow(clippy::too_many_arguments)]
pub fn level1_race(
    dcl: Decl,
    bid: u8,
    seat: Seat,
    hand: u32,
    legal: u32,
    key: &Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    n_max: usize,
    n0: usize,
    n_self: usize,
    per_move_secs: u64,
    rng: &mut SplitMix64,
) -> Result<RaceReport, Level1Refusal> {
    const RACE_BLOCK: usize = 16;
    const RACE_KMIN: usize = 8;
    const RACE_DELTA: (u64, u64) = (1, 128);
    let viewer = seat.index();
    let maximize = seat.team() == Team::T1;
    let cands: Vec<u8> = mask_bits(legal);
    if cands.len() == 1 {
        return Ok(RaceReport {
            choice: cands[0],
            worlds_used: 0,
            tally: vec![(cands[0], 0, 0)],
            eliminated: Vec::new(),
        });
    }
    let deadline = Deadline::after(Duration::from_secs(per_move_secs));
    let worlds = sample_belief(viewer, hand, key.played, sizes, voids, n_max, rng)?;
    let sh = Arc::new(Shared::new(
        dcl,
        bid,
        vec![n0, n_self],
        trick_start_played,
        boundary_hand_size,
        deadline,
    ));
    let host = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        maximize,
        Vec::new(),
        Vec::new(),
        Field::Level(1),
    );
    let mut live: Vec<u8> = cands.clone();
    // bits[c][w] = success of candidate c on world w (aligned prefixes).
    let mut bits: Vec<Vec<bool>> = vec![Vec::new(); cands.len()];
    let slot = |t: u8, cands: &[u8]| cands.iter().position(|&c| c == t).expect("candidate");
    let mut eliminated: Vec<(u8, usize)> = Vec::new();
    let mut used = 0usize;
    while used < worlds.len() && live.len() > 1 {
        let hi = (used + RACE_BLOCK).min(worlds.len());
        let jobs: Vec<(u8, usize)> = live
            .iter()
            .flat_map(|&c| (used..hi).map(move |w| (c, w)))
            .collect();
        let run = |&(c, w): &(u8, usize)| -> Option<(u8, usize, bool)> {
            let made = race_replay(&host, dcl, bid, viewer, key, worlds[w], c)?;
            Some((c, w, made == maximize))
        };
        #[cfg(feature = "parallel")]
        let results: Option<Vec<(u8, usize, bool)>> = jobs.par_iter().map(run).collect();
        #[cfg(not(feature = "parallel"))]
        let results: Option<Vec<(u8, usize, bool)>> = jobs.iter().map(run).collect();
        let mut results = results.ok_or(Level1Refusal::Deadline)?;
        results.sort_by_key(|&(c, w, _)| (slot(c, &cands), w));
        for (c, _, success) in results {
            bits[slot(c, &cands)].push(success);
        }
        used = hi;
        // Leader: max successes over the common prefix, lowest tile on ties.
        let score = |t: u8| bits[slot(t, &cands)].iter().filter(|&&s| s).count();
        let leader = *live
            .iter()
            .min_by_key(|&&t| (std::cmp::Reverse(score(t)), t))
            .expect("live nonempty");
        let mut still: Vec<u8> = Vec::with_capacity(live.len());
        for &r in &live {
            if r == leader {
                still.push(r);
                continue;
            }
            let lb = &bits[slot(leader, &cands)];
            let rb = &bits[slot(r, &cands)];
            let n_plus = lb.iter().zip(rb).filter(|(l, r)| **l && !**r).count();
            let n_minus = lb.iter().zip(rb).filter(|(l, r)| !**l && **r).count();
            let k = n_plus + n_minus;
            let settled = k >= RACE_KMIN && binom_tail_leq(k, n_plus, RACE_DELTA.0, RACE_DELTA.1);
            if settled {
                eliminated.push((r, used));
            } else {
                still.push(r);
            }
        }
        live = still;
    }
    let score = |t: u8| bits[slot(t, &cands)].iter().filter(|&&s| s).count();
    let choice = *live
        .iter()
        .min_by_key(|&&t| (std::cmp::Reverse(score(t)), t))
        .expect("live nonempty");
    let tally = cands
        .iter()
        .map(|&c| {
            let b = &bits[slot(c, &cands)];
            (c, b.iter().filter(|&&s| s).count() as u32, b.len() as u32)
        })
        .collect();
    Ok(RaceReport {
        choice,
        worlds_used: used,
        tally,
        eliminated,
    })
}

/// Block-race outcome: the chosen tile, the evidence, and exact values for
/// the candidates still alive at the end.
pub struct BlockRace {
    pub choice: u8,
    pub worlds_used: usize,
    /// (tile, mean value over its processed blocks, worlds it consumed).
    pub values: Vec<(u8, BigRational, usize)>,
    /// (tile, worlds seen when eliminated).
    pub eliminated: Vec<(u8, usize)>,
}

/// The block-racing evaluator — the tilt geometry applied INSIDE the
/// level-1 estimator family, keeping its semantics per block: candidates
/// are bundle-solved on small CRN world blocks (every candidate sees the
/// SAME blocks), and rivals are eliminated at block boundaries by the
/// exact binomial sign test on paired per-block value differences —
/// pivotal blocks are those where the two candidates' block values differ,
/// and the same q/tau logic governs how fast a pair settles. Losers stop
/// consuming worlds at elimination; near-ties run to the cap and are
/// decided by the exact running mean (lowest tile on exact ties).
///
/// The estimator per candidate is the mean of iid copies of the
/// block-sized level-1 estimator (smaller bundles than one n_max solve —
/// a slightly different idealization, shared by all candidates under CRN).
/// One `Shared` spans the whole call, so the modeled-mind cache carries
/// across blocks and candidates. EXPLORATORY play policy; never a receipt.
#[allow(clippy::too_many_arguments)]
pub fn level1_raced(
    dcl: Decl,
    bid: u8,
    seat: Seat,
    hand: u32,
    legal: u32,
    key: &Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    n_max: usize,
    n0: usize,
    block: usize,
    per_move_secs: u64,
    rng: &mut SplitMix64,
) -> Result<BlockRace, Level1Refusal> {
    let maximize = seat.team() == Team::T1;
    let cands: Vec<u8> = mask_bits(legal);
    if cands.len() == 1 {
        return Ok(BlockRace {
            choice: cands[0],
            worlds_used: 0,
            values: Vec::new(),
            eliminated: Vec::new(),
        });
    }
    let deadline = Deadline::after(Duration::from_secs(per_move_secs));
    let sh = Arc::new(Shared::new(
        dcl,
        bid,
        vec![n0],
        trick_start_played,
        boundary_hand_size,
        deadline,
    ));
    selection::race(&cands, maximize, n_max, block, |live, b, _| {
        let worlds = sample_belief(seat.index(), hand, key.played, sizes, voids, b, rng)?;
        let eval_one = |&t: &u8| -> Option<(u8, BigRational)> {
            let solver = Solver::new(
                Arc::clone(&sh),
                seat,
                hand,
                maximize,
                worlds.clone(),
                Vec::new(),
                Field::Level(0),
            );
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(key, tile, 0);
            let v = solver.solve(&child)?;
            solver.flush_nodes();
            Some((t, v))
        };
        #[cfg(feature = "parallel")]
        let block_vals: Option<Vec<(u8, BigRational)>> = live.par_iter().map(eval_one).collect();
        #[cfg(not(feature = "parallel"))]
        let block_vals: Option<Vec<(u8, BigRational)>> = live.iter().map(eval_one).collect();
        block_vals.ok_or(Level1Refusal::Deadline)
    })
}

/// Race, then refine: the practical composition. The block race eliminates
/// dominated candidates on cheap CRN blocks (spending worlds only where
/// pairs are still undecided); if a unique strict winner survives, done.
/// Otherwise the surviving tie set — the saturation regime the race cannot
/// split — goes to `level1_evaluate`'s existing saturation-tie refinement,
/// restricted to the survivors. Returns the chosen tile and the worlds the
/// race consumed (refinement worlds are level1_evaluate's own affair).
/// EXPLORATORY play policy; never a receipt.
#[allow(clippy::too_many_arguments)]
pub fn level1_race_refined(
    dcl: Decl,
    bid: u8,
    seat: Seat,
    hand: u32,
    legal: u32,
    key: &Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    n_race: usize,
    n_refine: usize,
    n0: usize,
    per_move_secs: u64,
    rng: &mut SplitMix64,
) -> Result<u8, Level1Refusal> {
    let maximize = seat.team() == Team::T1;
    let deadline = Deadline::after(Duration::from_secs(per_move_secs));
    let make_shared = || {
        Arc::new(Shared::new(
            dcl,
            bid,
            vec![n0],
            trick_start_played,
            boundary_hand_size,
            deadline,
        ))
    };
    let race_shared = make_shared();
    selection::race_refine(
        &mask_bits(legal),
        maximize,
        n_race,
        n_refine,
        |tiles, n, kind| {
            let worlds = sample_belief(seat.index(), hand, key.played, sizes, voids, n, rng)?;
            let shared = if kind == selection::Batch::Block {
                Arc::clone(&race_shared)
            } else {
                make_shared()
            };
            Solver::new(
                shared,
                seat,
                hand,
                maximize,
                worlds,
                Vec::new(),
                Field::Level(0),
            )
            .parallel()
            .action_values(key, tiles)
            .ok_or(Level1Refusal::Deadline)
        },
    )
    .map(|result| result.choice)
}

pub mod partnership_wire;

#[cfg(all(test, feature = "mask64-support-arena"))]
mod mask64_support_tests {
    use super::*;

    #[test]
    fn solver_keeps_root_zero_and_falls_back_above_sixty_four_worlds() {
        for n in [9, 40, 64, 65] {
            let shared = Arc::new(Shared::new(
                decl_of(6),
                30,
                vec![1],
                0,
                7,
                Deadline::after(Duration::from_secs(10)),
            ));
            let solver = Solver::new(
                shared,
                Seat::S0,
                0,
                false,
                vec![[0; 4]; n],
                Vec::new(),
                Field::Dice,
            );
            assert!(solver.small_support.is_none());
            assert_eq!(solver.mask64_support.is_some(), n <= 64);
            assert_eq!(solver.alive_sets(), 1);
            assert_eq!(
                solver.alive_of(0).iter().collect::<Vec<_>>(),
                (0..n as u32).collect::<Vec<_>>()
            );
            let subset = vec![0, (n / 2) as u32, (n - 1) as u32];
            let id = solver.intern(subset.clone());
            assert_ne!(id, 0);
            assert_eq!(solver.intern(subset.clone()), id);
            assert_eq!(solver.alive_of(id).iter().collect::<Vec<_>>(), subset);
            assert_eq!(solver.alive_sets(), 2);
        }
    }
}

#[cfg(all(test, feature = "fast-policy", feature = "parallel"))]
mod fast_policy_tests {
    use super::*;

    #[test]
    fn small_serial_policy_matches_parallel_field_at_partial_trick() {
        // Five completed no-count tricks leave all count tiles live. The two
        // samples share the focal hand but distribute the field hands
        // differently; sample IDs remain distinct even at equal outcomes.
        let remaining = [0u8, 8, 11, 15, 20, 25, 26, 27];
        let unplayed = remaining.iter().fold(0u32, |mask, &t| mask | (1u32 << t));
        let boundary_played = FULL_MASK & !unplayed;
        let worlds = vec![
            [
                bit(Domino::ALL[0]) | bit(Domino::ALL[27]),
                bit(Domino::ALL[8]) | bit(Domino::ALL[25]),
                bit(Domino::ALL[11]) | bit(Domino::ALL[20]),
                bit(Domino::ALL[15]) | bit(Domino::ALL[26]),
            ],
            [
                bit(Domino::ALL[0]) | bit(Domino::ALL[27]),
                bit(Domino::ALL[8]) | bit(Domino::ALL[11]),
                bit(Domino::ALL[20]) | bit(Domino::ALL[25]),
                bit(Domino::ALL[15]) | bit(Domino::ALL[26]),
            ],
        ];
        let key = Key {
            voids: None,
            played: boundary_played | bit(Domino::ALL[8]),
            leader: 1,
            plays: vec![8],
            banked_t1: 3,
            banked_t0: 2,
            alive: 0,
        };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        for decl_id in [6, 9] {
            let eval = |parallel: bool| {
                let shared = Arc::new(Shared::new(
                    decl_of(decl_id),
                    30,
                    vec![1],
                    boundary_played,
                    2,
                    Deadline::after(Duration::from_secs(10)),
                ));
                let solver = Solver::new(
                    shared,
                    Seat::S0,
                    worlds[0][0],
                    false,
                    worlds.clone(),
                    Vec::new(),
                    Field::Level(0),
                );
                let solver = if parallel { solver.parallel() } else { solver };
                solver.solve(&key).expect("complete field value")
            };
            let serial = eval(false);
            let parallel = pool.install(|| eval(true));
            assert_eq!(serial, parallel, "declaration {decl_id}");
        }
    }
}

#[cfg(all(test, feature = "parallel", feature = "adaptive-parallel"))]
mod parallel_schedule_tests {
    use super::*;

    #[test]
    fn root_vectors_match_serial_at_support_one_two_eight_and_forty() {
        // Five completed no-count tricks leave these eight count-bearing
        // tiles. The acting seat has two legal options after tile 8 was led.
        let remaining = [0u8, 8, 11, 15, 20, 25, 26, 27];
        let unplayed = remaining.iter().fold(0u32, |mask, &t| mask | (1u32 << t));
        let boundary = FULL_MASK & !unplayed;
        let hand = bit(Domino::ALL[11]) | bit(Domino::ALL[20]);
        let key = Key {
            voids: None,
            played: boundary | bit(Domino::ALL[8]),
            leader: 1,
            plays: vec![8],
            banked_t1: 3,
            banked_t0: 2,
            alive: 0,
        };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        for decl_id in [6, 9] {
            let decl = decl_of(decl_id);
            assert_eq!(
                mask_of(legal_plays(
                    decl,
                    set_of(hand),
                    Some(decl.led_context(Domino::ALL[8]))
                )),
                hand
            );
            let sampler = Shared::new(
                decl,
                30,
                vec![1],
                boundary,
                2,
                Deadline::after(Duration::from_secs(30)),
            );
            let mut rng = SplitMix64(0xC0A2_5EED);
            let sizes = [2, 1, 2, 2];
            let worlds = sampler
                .inner_belief
                .sample(
                    decl,
                    Seat::S2,
                    hand,
                    &key,
                    sizes,
                    40,
                    &mut rng,
                    sampler.deadline,
                )
                .expect("forty lawful worlds");
            for support in [1, 2, 8, 40] {
                let evaluate = |parallel: bool| {
                    let sh = Arc::new(Shared::new(
                        decl,
                        30,
                        vec![1],
                        boundary,
                        2,
                        Deadline::after(Duration::from_secs(30)),
                    ));
                    let solver = Solver::new(
                        sh,
                        Seat::S2,
                        hand,
                        false,
                        worlds[..support].to_vec(),
                        Vec::new(),
                        Field::Level(0),
                    );
                    let solver = if parallel { solver.parallel() } else { solver };
                    solver
                        .action_values(&key, &[11, 20])
                        .expect("complete root vector")
                };
                let serial = evaluate(false);
                let parallel = pool.install(|| evaluate(true));
                assert_eq!(parallel, serial, "declaration {decl_id}, support {support}");
            }
        }
    }
}

#[cfg(all(test, feature = "fixed-policy-choice"))]
mod fixed_policy_choice_tests {
    use super::*;

    fn position(second_field_play: bool) -> (u32, Key, Seat, u32) {
        let remaining = [0u8, 8, 11, 15, 20, 25, 26, 27];
        let unplayed = remaining.iter().fold(0u32, |mask, &t| mask | (1u32 << t));
        let boundary = FULL_MASK & !unplayed;
        // These eight tiles hold all 35 count points. Five completed tricks
        // therefore banked exactly their five trick points (3 to T1, 2 to T0).
        let outstanding = remaining
            .iter()
            .map(|&id| Domino::ALL[id as usize].count())
            .sum::<u32>()
            + 2; // two trick points remain
        assert_eq!(outstanding, 37);
        let mut key = Key {
            voids: None,
            played: boundary | bit(Domino::ALL[8]),
            leader: 1,
            plays: vec![8],
            banked_t1: 3,
            banked_t0: 2,
            alive: 0,
        };
        assert_eq!(
            u32::from(key.banked_t1) + u32::from(key.banked_t0) + outstanding,
            42
        );
        assert!(key.banked_t1 < 30 && key.banked_t0 <= 42 - 30);
        if second_field_play {
            key.played |= bit(Domino::ALL[11]);
            key.plays.push(11);
            (
                boundary,
                key,
                Seat::S3,
                bit(Domino::ALL[15]) | bit(Domino::ALL[26]),
            )
        } else {
            (
                boundary,
                key,
                Seat::S2,
                bit(Domino::ALL[11]) | bit(Domino::ALL[20]),
            )
        }
    }

    fn shared(decl: Decl, boundary: u32, deadline: Duration) -> Arc<Shared> {
        Arc::new(Shared::new(
            decl,
            30,
            vec![1, 2, 2],
            boundary,
            2,
            Deadline::after(deadline),
        ))
    }

    #[test]
    fn modeled_fixed_choice_matches_complete_values_at_nonterminal_levels() {
        for (decl_id, k, maximize) in [
            (6, 1, false),
            (6, 1, true),
            (9, 1, false),
            (9, 1, true),
            (6, 2, false),
            (6, 2, true),
        ] {
            let decl = decl_of(decl_id);
            let (boundary, key, seat, hand) = position(maximize);
            assert_eq!(seat.team() == Team::T1, maximize);
            let led = decl.led_context(Domino::ALL[8]);
            assert_eq!(mask_of(legal_plays(decl, set_of(hand), Some(led))), hand);
            let choices = mask_bits(hand);
            let reference_sh = shared(decl, boundary, Duration::from_secs(10));
            let reference_host = Solver::new(
                Arc::clone(&reference_sh),
                Seat::S0,
                0,
                false,
                Vec::new(),
                Vec::new(),
                Field::Level(0),
            );
            let sizes = reference_host.hand_sizes_at(&key);
            let mut rng = SplitMix64(
                INNER_SEED
                    ^ mix(0x4C32 ^ k as u64)
                    ^ mix(seat.index() as u64)
                    ^ mix(u64::from(hand))
                    ^ record_hash(&key),
            );
            let worlds = reference_sh
                .inner_belief
                .sample(
                    decl,
                    seat,
                    hand,
                    &key,
                    sizes,
                    reference_sh.n_inner[k],
                    &mut rng,
                    reference_sh.deadline,
                )
                .expect("complete fixed sample");
            let root = Key {
                alive: 0,
                ..key.clone()
            };
            #[cfg(feature = "compact-policy")]
            let compact_worlds = worlds.clone();
            let reference = Solver::new(
                Arc::clone(&reference_sh),
                seat,
                hand,
                maximize,
                worlds,
                Vec::new(),
                Field::Level(k - 1),
            )
            .action_values(&root, &choices)
            .expect("complete old fixed vector");
            let expected = best_of(&reference, maximize);

            #[cfg(feature = "compact-policy")]
            {
                let compact_sh = shared(decl, boundary, Duration::from_secs(10));
                let compact = Solver::new(
                    compact_sh,
                    seat,
                    hand,
                    maximize,
                    compact_worlds,
                    Vec::new(),
                    Field::Level(k - 1),
                );
                assert_eq!(
                    compact_policy::fixed_choice(&compact, &root, &choices),
                    Some(Some(expected)),
                    "direct compact choice decl={decl_id} level={k} maximize={maximize}"
                );
            }

            let candidate_sh = shared(decl, boundary, Duration::from_secs(10));
            let host = Solver::new(
                candidate_sh,
                Seat::S0,
                0,
                false,
                Vec::new(),
                Vec::new(),
                Field::Level(0),
            );
            let actual = host
                .modeled_choice(k, &key, seat, hand, hand)
                .expect("choice-only modeled policy");
            assert_eq!(
                actual, expected,
                "decl={decl_id} level={k} maximize={maximize}"
            );
        }
    }

    #[test]
    fn choice_only_refuses_expired_deadline() {
        let (boundary, key, seat, hand) = position(false);
        let sh = shared(decl_of(6), boundary, Duration::ZERO);
        let worlds = vec![[
            bit(Domino::ALL[0]) | bit(Domino::ALL[27]),
            bit(Domino::ALL[8]) | bit(Domino::ALL[25]),
            hand,
            bit(Domino::ALL[15]) | bit(Domino::ALL[26]),
        ]];
        let solver = Solver::new(sh, seat, hand, false, worlds, Vec::new(), Field::Level(0));
        assert_eq!(solver.fixed_policy_choice(&key, &mask_bits(hand)), None);
        #[cfg(feature = "compact-policy")]
        assert_eq!(
            compact_policy::fixed_choice(&solver, &key, &mask_bits(hand)),
            Some(None)
        );
    }
}
