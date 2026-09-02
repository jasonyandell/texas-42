//! EXPLORATORY seat-solver library — the sampling-stack machinery of
//! SCENARIO-PLAYER.md, extracted from the probe binaries (bidcurve.rs
//! lineage, itself the divergence.rs/level2.rs line). Sits below every
//! evidentiary tier; nothing above the Ideas tier cites it.
//!
//! One solver, three axes of configuration:
//!   - `Field`: Dice at the bottom, `Level(k)` above it (the field model
//!     is a parameter — level-k minds best-respond to level-(k−1) minds).
//!   - `bid`: the pmake objective's threshold pair (make ⇔ banked_T1 ≥ bid
//!     ⇔ banked_T0 ≤ 42 − bid), so bidding and play share the machinery.
//!   - `parallel` (cargo feature + runtime flag): rayon fan-out on root
//!     candidates, bucket recursion, and per-node policy batches. Compiled
//!     out entirely without the "parallel" feature (the WASM build).
//!
//! Invariants (SCENARIO-PLAYER.md): PiKey carries the modeled mind's
//! ENTIRE information state, banked totals included (§3.4); every cache
//! entry is a pure function of its key, which is what makes results
//! invariant across thread counts and call orders (§8). Exact rationals
//! only — no floats.

pub mod act;
pub mod adaptive;
pub mod bundle;
pub mod calibrate;
pub mod controller;
pub mod covers;
pub mod doom;
pub mod evidence;
pub mod exposure;
pub mod extraction;
pub mod factor_belief;
pub mod field;
pub mod field_swap;
pub mod frontier;
pub mod godgap;
pub mod grammar;
pub mod hazard;
pub mod laydown;
pub mod model_belief;
pub mod model_recursion;
pub mod motif;
pub mod opening;
pub mod policy;
pub mod proof_state;
pub mod refine;
pub mod residual;
pub mod root_interval;
pub mod targeted;
pub mod upper_cs;
pub mod wakeup;
pub mod waking;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// Wall-clock budget for one evaluation. On native targets this is a real
/// monotonic deadline; on wasm32 there is no monotonic clock without a JS
/// import, so it never expires — budget there is carried by the sample
/// counts (n_outer, n0), not wall time.
#[derive(Clone, Copy)]
pub struct Deadline {
    #[cfg(not(target_arch = "wasm32"))]
    at: Instant,
}

impl Deadline {
    #[must_use]
    pub fn after(budget: Duration) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Deadline {
                at: Instant::now() + budget,
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = budget;
            Deadline {}
        }
    }

    #[must_use]
    pub fn passed(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Instant::now() >= self.at
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
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
    let mut m = 0u32;
    for dm in set.iter() {
        m |= bit(dm);
    }
    m
}

pub fn set_of(mask: u32) -> DominoSet {
    let mut s = DominoSet::default();
    let mut m = mask;
    while m != 0 {
        let i = m.trailing_zeros() as usize;
        s.insert(Domino::from_index(i).expect("index < 28"));
        m &= m - 1;
    }
    s
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
#[derive(Clone, Copy)]
pub enum Field {
    Dice,
    Level(usize),
}

/// The modeled mind's entire information state, banked totals included
/// (SCENARIO-PLAYER.md §3.1/§3.4).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PiKey {
    pub seat: u8,
    pub hand: u32,
    pub played: u32,
    pub leader: u8,
    pub plays: Vec<u8>,
    pub banked_t1: u8,
    pub banked_t0: u8,
}

const PI_SHARDS: usize = 64;

type PiShard = Mutex<HashMap<(u8, PiKey), u8>>;

/// State shared by every solver in one evaluation: declaration, bid
/// thresholds, boundary frame, budget, and the cross-level policy cache.
pub struct Shared {
    pub dcl: Decl,
    /// make ⇔ banked_t1 ≥ bid ⇔ banked_t0 ≤ 42 − bid.
    pub bid: u8,
    /// n_inner[k] = belief sample size of a modeled level-k mind.
    pub n_inner: Vec<usize>,
    pub boundary_played: u32,
    pub boundary_hand_size: usize,
    pub deadline: Deadline,
    pi_cache: Vec<PiShard>,
    pub pi_calls: AtomicU64,
    pub nodes: AtomicU64,
    /// Break instrumentation on the `solve_viewer` loop, folded from each
    /// solver's locals by `flush_nodes`: children actually solved vs legal
    /// moves available. An instrument only — read by probes and tests,
    /// never by a receipt bin.
    pub viewer_children: AtomicU64,
    pub viewer_legal: AtomicU64,
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
        Shared {
            dcl,
            bid,
            n_inner,
            boundary_played,
            boundary_hand_size,
            deadline,
            pi_cache: (0..PI_SHARDS).map(|_| Mutex::new(HashMap::new())).collect(),
            pi_calls: AtomicU64::new(0),
            nodes: AtomicU64::new(0),
            viewer_children: AtomicU64::new(0),
            viewer_legal: AtomicU64::new(0),
            dead: AtomicBool::new(false),
        }
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

    fn pi_shard(&self, k: u8, pk: &PiKey) -> &PiShard {
        let mut h = DefaultHasher::new();
        (k, pk).hash(&mut h);
        &self.pi_cache[(h.finish() as usize) & (PI_SHARDS - 1)]
    }

    pub fn pi_cache_len(&self) -> usize {
        self.pi_cache
            .iter()
            .map(|s| s.lock().expect("pi shard poisoned").len())
            .sum()
    }
}

struct Intern {
    list: Vec<Arc<Vec<u32>>>,
    map: HashMap<Arc<Vec<u32>>, u32>,
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
    memo: Mutex<HashMap<Key, BigRational>>,
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
        let all: Arc<Vec<u32>> = Arc::new((0..worlds.len() as u32).collect());
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
            local_viewer_children: AtomicU64::new(0),
            local_viewer_legal: AtomicU64::new(0),
            ordering: MoveOrdering::CaptureFirst,
        }
    }

    /// Enable rayon fan-out (no-op in a build without the "parallel"
    /// feature — the flag is kept so caller code is identical).
    #[must_use]
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }

    /// Select the `solve_viewer` visit order (default `CaptureFirst`).
    /// For the equivalence gate and instrument A/B arms only — value
    /// results are identical under every selector by E-A15.
    #[must_use]
    pub fn with_ordering(mut self, ordering: MoveOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    pub fn memo_len(&self) -> usize {
        self.memo.lock().expect("memo poisoned").len()
    }

    pub fn alive_sets(&self) -> usize {
        self.intern.lock().expect("intern poisoned").list.len()
    }

    fn intern(&self, v: Vec<u32>) -> u32 {
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

    fn alive_of(&self, id: u32) -> Arc<Vec<u32>> {
        Arc::clone(&self.intern.lock().expect("intern poisoned").list[id as usize])
    }

    fn bump_node(&self) -> bool {
        if self.sh.dead.load(Ordering::Relaxed) {
            return false;
        }
        let n = self.local_nodes.fetch_add(1, Ordering::Relaxed) + 1;
        if n & 0xFFFF == 0 {
            if self.sh.deadline.passed() {
                self.sh.dead.store(true, Ordering::Relaxed);
                return false;
            }
            self.sh.nodes.fetch_add(0x1_0000, Ordering::Relaxed);
        }
        true
    }

    /// Flush the sub-64k remainder of this solver's node count into the
    /// global total, and this solver's `solve_viewer` break counters whole.
    /// Call exactly once, after this solver's last solve.
    pub fn flush_nodes(&self) {
        self.sh.nodes.fetch_add(
            self.local_nodes.load(Ordering::Relaxed) & 0xFFFF,
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

    pub fn child_after_play(&self, key: &Key, tile: Domino, alive: u32) -> Key {
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

    pub fn solve(&self, key: &Key) -> Option<BigRational> {
        if !self.bump_node() {
            return None;
        }
        if key.banked_t1 >= self.sh.bid {
            return Some(BigRational::one());
        }
        if key.banked_t0 > 42 - self.sh.bid {
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
    fn viewer_visit_priority(&self, key: &Key, led: Option<Context>, tile: Domino) -> u32 {
        let dcl = self.sh.dcl;
        let Some(q) = led else {
            // Leading: strongest lead first — called tier over natural,
            // declaration rank within the tier.
            let k = dcl.trick_key(tile, dcl.led_context(tile));
            return 100 * k.tier as u32 + u32::from(k.rank.value());
        };
        let on_table = || {
            key.plays
                .iter()
                .map(|&p| Domino::from_index(usize::from(p)).expect("played tile"))
        };
        // The trick's standing winner: first strict maximum in play
        // order, the `Trick::winner` convention.
        let mut best = None;
        let mut winner_at = 0;
        for (i, d) in on_table().enumerate() {
            let k = dcl.trick_key(d, q);
            if best.as_ref().is_none_or(|b| k > *b) {
                best = Some(k);
                winner_at = i;
            }
        }
        let best = best.expect("a led context implies a play");
        if dcl.trick_key(tile, q) > best {
            // Wins the trick as it stands: richest capture first (count
            // on the table plus the tile's own).
            1_000 + on_table().map(Domino::count).sum::<u32>() + tile.count()
        } else {
            let winner = Seat::from_index((usize::from(key.leader) + winner_at) % 4)
                .expect("winner seat index");
            if winner.team() == self.viewer.team() {
                // The trick currently falls to the viewer's own team:
                // feed it count first.
                20 + tile.count()
            } else {
                // It falls to the opponents: give up the least count
                // first.
                10 - tile.count()
            }
        }
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
        let priority = |t: Domino| match self.ordering {
            MoveOrdering::CaptureFirst => self.viewer_visit_priority(key, led, t),
            // A constant priority leaves only the index tie-break: the
            // historical ascending order, kept for the equivalence gate.
            MoveOrdering::TileIndex => 0,
        };
        let mut order: Vec<(u32, Domino)> = legal.iter().map(|t| (priority(t), t)).collect();
        order.sort_unstable_by_key(|&(p, t)| (std::cmp::Reverse(p), t.index()));
        order.into_iter().map(|(_, t)| t).collect()
    }

    fn solve_viewer(&self, key: &Key, led: Option<Context>) -> Option<BigRational> {
        let hand = self.viewer_hand0 & !key.played;
        let legal = legal_plays(self.sh.dcl, set_of(hand), led);
        // Reorder, never cull (E-A15): the same legal set in heuristic
        // order. This loop is value-only — max/min over children, no
        // action returned — so no tie-break is exposed; the order only
        // moves where the Boolean break lands.
        let order = self.viewer_visit_order(key, led, legal);
        self.local_viewer_legal
            .fetch_add(order.len() as u64, Ordering::Relaxed);
        let mut best: Option<BigRational> = None;
        for &tile in &order {
            self.local_viewer_children.fetch_add(1, Ordering::Relaxed);
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
        #[cfg(feature = "parallel")]
        if self.parallel && distinct.len() > 1 {
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
        let serial = || -> Vec<Option<BigRational>> {
            children
                .iter()
                .map(|(_, child)| self.solve(child))
                .collect()
        };
        #[cfg(feature = "parallel")]
        let vals: Vec<Option<BigRational>> = if self.parallel && children.len() > 1 {
            children
                .par_iter()
                .map(|(_, child)| self.solve(child))
                .collect()
        } else {
            serial()
        };
        #[cfg(not(feature = "parallel"))]
        let vals: Vec<Option<BigRational>> = serial();
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

    /// The level-k policy at a modeled seat's information state (pure in
    /// (k, PiKey); level-0 seeding bit-identical across the stack).
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
        // Inner minds inherit the host's visit-order selector so an A/B
        // arm is whole-stack, not host-only (value-invariant either way).
        let inner = Solver::new(
            Arc::clone(&self.sh),
            seat,
            hand,
            maximize,
            inner_worlds,
            inner_seeds,
            inner_field,
        )
        .with_ordering(self.ordering);
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
        p @ 0..=6 => Decl::ALL[p],
        7 => Decl::DoublesTrump,
        9 => Decl::NoTrump,
        other => panic!("declaration id {other} is not a straight-42 declaration"),
    }
}

pub fn arena_decl_id(d: Decl) -> usize {
    match d {
        Decl::PipTrump(p) => usize::from(p.value()),
        Decl::DoublesTrump => 7,
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
    for &(actor_arena, tile_id) in pairs {
        let actor = (actor_arena + r) % 4;
        let expect = (usize::from(st.leader) + st.plays.len()) % 4;
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
        if st.plays.len() == 4 {
            let doms = [
                Domino::from_index(usize::from(st.plays[0])).expect("p0"),
                Domino::from_index(usize::from(st.plays[1])).expect("p1"),
                Domino::from_index(usize::from(st.plays[2])).expect("p2"),
                Domino::from_index(usize::from(st.plays[3])).expect("p3"),
            ];
            let trick = Trick::new(
                Seat::from_index(usize::from(st.leader)).expect("leader"),
                doms,
            )
            .expect("distinct");
            let winner = trick.winner(dcl);
            let pts = trick.points() as u8;
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
    let all_tiles = mask_bits(legal);
    let mut opts = evaluate(&all_tiles, n_outer, rng)?;
    let mut n_cur = n_outer;
    loop {
        let best = if maximize {
            opts.iter().map(|(_, v)| v.clone()).max()
        } else {
            opts.iter().map(|(_, v)| v.clone()).min()
        }
        .expect("legal play");
        let tied: Vec<u8> = opts
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(t, _)| *t)
            .collect();
        if tied.len() == 1 || n_cur >= n_outer * 16 {
            break;
        }
        n_cur *= 4;
        let refined = evaluate(&tied, n_cur, rng)?;
        for (t, v) in refined {
            let slot = opts
                .iter_mut()
                .find(|(ot, _)| *ot == t)
                .expect("tied tile present");
            slot.1 = v;
        }
    }
    Ok(opts)
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

/// Step a Key forward by one play (banked-correct trick resolution; no void
/// tracking — modeled minds do not condition on voids).
pub(crate) fn key_step(key: &mut Key, dcl: Decl, tile: Domino) {
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
    const RACE_KMIN: usize = 6;
    const RACE_DELTA: (u64, u64) = (1, 128);
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
    let slot = |t: u8| cands.iter().position(|&c| c == t).expect("candidate");
    let mut live: Vec<u8> = cands.clone();
    // Per candidate: per-block values (aligned across live candidates).
    let mut blocks_of: Vec<Vec<BigRational>> = vec![Vec::new(); cands.len()];
    let mut consumed: Vec<usize> = vec![0; cands.len()];
    let mut eliminated: Vec<(u8, usize)> = Vec::new();
    let mut used = 0usize;
    while used < n_max && live.len() > 1 {
        let b = block.min(n_max - used);
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
        for (t, v) in block_vals.ok_or(Level1Refusal::Deadline)? {
            blocks_of[slot(t)].push(v);
            consumed[slot(t)] += b;
        }
        used += b;
        // Leader by exact running mean over the common live prefix (equal
        // block counts among live candidates), lowest tile on exact ties.
        let sum_of = |t: u8| -> BigRational {
            blocks_of[slot(t)]
                .iter()
                .fold(BigRational::zero(), |a, v| a + v)
        };
        let leader = live
            .iter()
            .copied()
            .reduce(|best, cand| {
                let (bs, cs) = (sum_of(best), sum_of(cand));
                let better = if maximize { cs > bs } else { cs < bs };
                if better {
                    cand
                } else {
                    best
                }
            })
            .expect("live nonempty");
        let mut still: Vec<u8> = Vec::with_capacity(live.len());
        for &r in &live {
            if r == leader {
                still.push(r);
                continue;
            }
            let lb = &blocks_of[slot(leader)];
            let rb = &blocks_of[slot(r)];
            let mut n_plus = 0usize;
            let mut n_minus = 0usize;
            for (lv, rv) in lb.iter().zip(rb) {
                let leader_won = if maximize { lv > rv } else { lv < rv };
                let rival_won = if maximize { rv > lv } else { rv < lv };
                if leader_won {
                    n_plus += 1;
                } else if rival_won {
                    n_minus += 1;
                }
            }
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
    let mean_of = |t: u8| -> BigRational {
        let bs = &blocks_of[slot(t)];
        let sum = bs.iter().fold(BigRational::zero(), |a, v| a + v);
        sum / BigRational::from_integer(BigInt::from(bs.len().max(1)))
    };
    let choice = live
        .iter()
        .copied()
        .reduce(|best, cand| {
            let (bm, cm) = (mean_of(best), mean_of(cand));
            let better = if maximize { cm > bm } else { cm < bm };
            if better {
                cand
            } else {
                best
            }
        })
        .expect("live nonempty");
    let values = live
        .iter()
        .map(|&t| (t, mean_of(t), consumed[slot(t)]))
        .collect();
    Ok(BlockRace {
        choice,
        worlds_used: used,
        values,
        eliminated,
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
    let race = level1_raced(
        dcl,
        bid,
        seat,
        hand,
        legal,
        key,
        sizes,
        voids,
        trick_start_played,
        boundary_hand_size,
        n_race,
        n0,
        8,
        per_move_secs,
        rng,
    )?;
    if race.values.len() <= 1 {
        return Ok(race.choice);
    }
    let maximize = seat.team() == Team::T1;
    let best = race
        .values
        .iter()
        .map(|(_, v, _)| v.clone())
        .reduce(|a, b| if maximize { a.max(b) } else { a.min(b) })
        .expect("nonempty");
    let tied: Vec<u8> = race
        .values
        .iter()
        .filter(|(_, v, _)| *v == best)
        .map(|(t, _, _)| *t)
        .collect();
    if tied.len() == 1 {
        return Ok(race.choice);
    }
    let tied_mask = tied.iter().fold(0u32, |a, &t| a | (1u32 << t));
    let opts = level1_evaluate(
        dcl,
        bid,
        seat,
        hand,
        tied_mask,
        key,
        sizes,
        voids,
        trick_start_played,
        boundary_hand_size,
        n_refine,
        n0,
        per_move_secs,
        rng,
    )?;
    Ok(best_of(&opts, maximize))
}
