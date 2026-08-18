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

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Seat, Team};

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
    pub deadline: Instant,
    pi_cache: Vec<PiShard>,
    pub pi_calls: AtomicU64,
    pub nodes: AtomicU64,
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
        deadline: Instant,
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
            dead: AtomicBool::new(false),
        }
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
        }
    }

    /// Enable rayon fan-out (no-op in a build without the "parallel"
    /// feature — the flag is kept so caller code is identical).
    #[must_use]
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
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
            if Instant::now() >= self.sh.deadline {
                self.sh.dead.store(true, Ordering::Relaxed);
                return false;
            }
            self.sh.nodes.fetch_add(0x1_0000, Ordering::Relaxed);
        }
        true
    }

    /// Flush the sub-64k remainder of this solver's node count into the
    /// global total. Call exactly once, after this solver's last solve.
    pub fn flush_nodes(&self) {
        self.sh.nodes.fetch_add(
            self.local_nodes.load(Ordering::Relaxed) & 0xFFFF,
            Ordering::Relaxed,
        );
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

/// Void-conditioned belief sampler: uniform on the lawful-completion fiber
/// by shuffle-and-reject (SCENARIO-PLAYER.md §4.2).
pub fn sample_belief(
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    voids: [u32; 4],
    n: usize,
    rng: &mut SplitMix64,
) -> Vec<[u32; 4]> {
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
    out
}

/// Basis points of an exact probability, integer-rounded toward zero.
pub fn bp(v: &BigRational) -> i64 {
    let scaled = (v * BigRational::from_integer(BigInt::from(10_000))).to_integer();
    scaled.to_string().parse().unwrap_or(0)
}
