//! EXPLORATORY LEVEL-2 DIVERGENCE MINER — sits below every evidentiary tier
//! and is cited by nothing above it (wiki/ideas tier; outputs are ESTIMATES).
//!
//! The question this tool serves (Jason, 2026-08-17 night): partnership "is
//! the kind of thing that only matters when it is needed" — so don't play a
//! multithousand-hand head-to-head; PLAY HANDS AND FIND THE ONES WHERE IT
//! MAKES A DIFFERENCE. Self-play random deals with four level-1 walts and
//! shadow one seat with a level-2 walt at every real decision, logging every
//! comparison. The output is a JSONL corpus of positions where the partner-
//! coordinating mind disagrees with the solipsist-field mind, sorted later
//! by the level-2 value gap. Those positions are the raw material for the
//! morning's scenario-generation conversation: instead of specifying
//! "partnership-needed" a priori, mine instances and read the grammar off
//! them.
//!
//! Three regimes, cycled by hand index (internal frame: bidder is ALWAYS
//! internal S1, bid P30, so the bidding team is internal T1 — the bridge's
//! rotation convention):
//!   hand % 3 == 0: shadow internal S1 — walt holds the contract (self-bid)
//!   hand % 3 == 1: shadow internal S3 — walt supports partner's contract
//!   hand % 3 == 2: shadow internal S2 — walt defends
//! Prediction to test: divergence concentrates in partner-bid and defense
//! (signaling/coordination regimes), not self-bid.
//!
//! The trajectory is played by level-1 at every seat (bridge-strength
//! n_outer, void-conditioned beliefs); the shadow NEVER steers, so
//! divergences don't compound and every logged position sits on a realistic
//! level-1 game path. Level-1 and level-2 evaluate the SAME sampled worlds
//! (common random numbers) at each decision; saturation ties refine on
//! fresh 4x samples once (never index-broken silently — the tie sets are
//! logged). Declarations use a fixed pip heuristic (longest suit, double,
//! high pip), the same simplification the arena's dropped-30 protocol uses
//! for both teams.
//!
//! Solver core: the rayon-parallel, banked-correct PiKey machinery of
//! level2.rs (commit f5fff91) generalized to arbitrary boundaries. Modeled
//! minds: level-1 evals use n_inner=[8] (bridge parity); level-2 evals use
//! n_inner=[4,8] (ladder parity). Divergence is scored on level-2's table:
//! divergent iff v2(level-1's choice) < max v2, gap in basis points.
//!
//! Arithmetic: integer counts and BigRational values. No floats (basis
//! points are exact integer rounding). Nothing here is quotable above
//! exploratory tier; not a P-A21 statement.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Write as IoWrite;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use rayon::prelude::*;

use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};
// The void-conditioned belief sampler and its dependencies are the
// LIBRARY's one authority (`solver::sample_belief` — the σ1-repair slice
// deduplicated the five byte-identical copies onto it). Importing the
// sampler forces the RNG type with it: a local `SplitMix64` would be a
// distinct Rust type, so the seed paths in this binary run on the
// library's stream — the same algorithm, hash-identical, so no draw
// changes.
use walt::solver::{mask_bits, sample_belief, SplitMix64, FULL_MASK};

/// All 28 tiles.
/// Frozen seed for inner-mind sampling (identical to level1/level2/bridge).
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

/// Frozen miner stream seed (fresh constant; hands are functions of it).
const MINER_SEED: u64 = 0x9216_D5D9_8979_FB1B;

fn mix(h: u64) -> u64 {
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
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
/// level-k policy above it.
#[derive(Clone, Copy)]
enum Field {
    Dice,
    Level(usize),
}

/// Cache key for a modeled mind's decision: its entire information state,
/// INCLUDING the banked totals (the pmake objective conditions on them;
/// they are not derivable from the reduced record — see level2.rs).
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
}

/// Interned alive sets: id 0 is always the full sample.
struct Intern {
    list: Vec<Arc<Vec<u32>>>,
    map: HashMap<Arc<Vec<u32>>, u32>,
}

struct Solver {
    sh: Arc<Shared>,
    viewer: Seat,
    viewer_hand0: u32,
    maximize: bool,
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
    fn new(
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

    fn parallel(mut self) -> Self {
        self.parallel = true;
        self
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

    /// The level-k policy at a modeled seat's information state. Level-0
    /// seeding is bit-identical with level1.rs/walt_bridge pi0.
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

// ---------------------------------------------------------------------------
// Belief sampling (void-conditioned, identical scheme to walt_bridge)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Self-play state
// ---------------------------------------------------------------------------

struct State {
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    completed: usize,
}

fn advance(st: &mut State, dcl: Decl, tile: Domino) {
    let seat = (usize::from(st.leader) + st.plays.len()) % 4;
    if let Some(&led0) = st.plays.first() {
        let led = dcl.led_context(Domino::from_index(usize::from(led0)).expect("led"));
        if !dcl.follows(tile, led) {
            st.voids[seat] |= mask_of(dcl.effective_incidence(led));
        }
    }
    st.plays.push(tile.index() as u8);
    st.played |= bit(tile);
    if st.plays.len() == 4 {
        let doms = [
            Domino::from_index(usize::from(st.plays[0])).expect("p0"),
            Domino::from_index(usize::from(st.plays[1])).expect("p1"),
            Domino::from_index(usize::from(st.plays[2])).expect("p2"),
            Domino::from_index(usize::from(st.plays[3])).expect("p3"),
        ];
        let leader = Seat::from_index(usize::from(st.leader)).expect("leader");
        let trick = Trick::new(leader, doms).expect("distinct tiles");
        let winner = trick.winner(dcl);
        let value = trick.points() as u8;
        if winner.team() == Team::T1 {
            st.banked_t1 += value;
        } else {
            st.banked_t0 += value;
        }
        st.leader = winner.index() as u8;
        st.plays.clear();
        st.trick_start_played = st.played;
        st.completed += 1;
    }
}

fn key_of(st: &State) -> Key {
    Key {
        played: st.played,
        leader: st.leader,
        plays: st.plays.clone(),
        banked_t1: st.banked_t1,
        banked_t0: st.banked_t0,
        alive: 0,
    }
}

fn sizes_at(st: &State) -> [usize; 4] {
    let mut sizes = [7 - st.completed; 4];
    for i in 0..st.plays.len() {
        sizes[(usize::from(st.leader) + i) % 4] -= 1;
    }
    sizes
}

// ---------------------------------------------------------------------------
// Evaluation (level parameterized; CRN base worlds; one 4x tie refinement)
// ---------------------------------------------------------------------------

/// Evaluate every legal option for `actor` as a level-`lvl` walt (lvl 1 or
/// 2). Returns (tile, value) per option, or None on budget death.
#[allow(clippy::too_many_arguments)]
fn eval_options(
    dcl: Decl,
    actor: Seat,
    hand: u32,
    legal: u32,
    st: &State,
    lvl: usize,
    n_outer: usize,
    budget_secs: u64,
    base_worlds: &[[u32; 4]],
    rng: &mut SplitMix64,
) -> Option<Vec<(u8, BigRational)>> {
    let deadline = Instant::now() + std::time::Duration::from_secs(budget_secs);
    let maximize = actor.team() == Team::T1;
    let n_inner = if lvl == 2 { vec![4, 8] } else { vec![8] };
    let field = Field::Level(lvl - 1);
    let key = key_of(st);
    let evaluate = |tiles: &[u8], worlds: Vec<[u32; 4]>| -> Option<Vec<(u8, BigRational)>> {
        let sh = Arc::new(Shared {
            dcl,
            n_inner: n_inner.clone(),
            boundary_played: st.trick_start_played,
            boundary_hand_size: 7 - st.completed,
            deadline,
            pi_cache: (0..PI_SHARDS).map(|_| Mutex::new(HashMap::new())).collect(),
            pi_calls: AtomicU64::new(0),
            nodes: AtomicU64::new(0),
            dead: AtomicBool::new(false),
        });
        let solver = Solver::new(sh, actor, hand, maximize, worlds, Vec::new(), field).parallel();
        let results: Vec<Option<BigRational>> = tiles
            .par_iter()
            .map(|&t| {
                let tile = Domino::from_index(usize::from(t)).expect("tile");
                let child = solver.child_after_play(&key, tile, 0);
                solver.solve(&child)
            })
            .collect();
        solver.flush_nodes();
        let mut out = Vec::with_capacity(tiles.len());
        for (&t, v) in tiles.iter().zip(results) {
            out.push((t, v?));
        }
        Some(out)
    };
    let all_tiles = mask_bits(legal);
    let mut opts = evaluate(&all_tiles, base_worlds.to_vec())?;
    // One 4x refinement round on a saturation tie (logged upstream; never
    // silently index-broken).
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
    if tied.len() > 1 {
        let worlds4 = match sample_belief(
            actor.index(),
            hand,
            st.played,
            sizes_at(st),
            st.voids,
            n_outer * 4,
            rng,
        ) {
            Ok(worlds) => worlds,
            Err(frame) => {
                eprintln!("divergence: refinement draw refused ({frame})");
                return None;
            }
        };
        let refined = eval_base(dcl, actor, hand, &tied, st, lvl, deadline, worlds4)?;
        for (t, v) in refined {
            let slot = opts
                .iter_mut()
                .find(|(ot, _)| *ot == t)
                .expect("tied tile present");
            slot.1 = v;
        }
    }
    Some(opts)
}

/// Bare evaluation of `tiles` over `worlds` (used by the refinement round).
#[allow(clippy::too_many_arguments)]
fn eval_base(
    dcl: Decl,
    actor: Seat,
    hand: u32,
    tiles: &[u8],
    st: &State,
    lvl: usize,
    deadline: Instant,
    worlds: Vec<[u32; 4]>,
) -> Option<Vec<(u8, BigRational)>> {
    let maximize = actor.team() == Team::T1;
    let n_inner = if lvl == 2 { vec![4, 8] } else { vec![8] };
    let field = Field::Level(lvl - 1);
    let key = key_of(st);
    let sh = Arc::new(Shared {
        dcl,
        n_inner,
        boundary_played: st.trick_start_played,
        boundary_hand_size: 7 - st.completed,
        deadline,
        pi_cache: (0..PI_SHARDS).map(|_| Mutex::new(HashMap::new())).collect(),
        pi_calls: AtomicU64::new(0),
        nodes: AtomicU64::new(0),
        dead: AtomicBool::new(false),
    });
    let solver = Solver::new(sh, actor, hand, maximize, worlds, Vec::new(), field).parallel();
    let results: Vec<Option<BigRational>> = tiles
        .par_iter()
        .map(|&t| {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(&key, tile, 0);
            solver.solve(&child)
        })
        .collect();
    solver.flush_nodes();
    let mut out = Vec::with_capacity(tiles.len());
    for (&t, v) in tiles.iter().zip(results) {
        out.push((t, v?));
    }
    Some(out)
}

fn argmax_set(opts: &[(u8, BigRational)], maximize: bool) -> Vec<u8> {
    let best = if maximize {
        opts.iter().map(|(_, v)| v.clone()).max()
    } else {
        opts.iter().map(|(_, v)| v.clone()).min()
    }
    .expect("legal play");
    opts.iter()
        .filter(|(_, v)| *v == best)
        .map(|(t, _)| *t)
        .collect()
}

fn bp(v: &BigRational) -> i64 {
    let scaled = (v * BigRational::from_integer(BigInt::from(10_000))).to_integer();
    scaled.to_string().parse().unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Driver
// ---------------------------------------------------------------------------

fn tile_name(t: u8) -> String {
    let dm = Domino::from_index(usize::from(t)).expect("tile");
    format!("{}{}", dm.hi().value(), dm.lo().value())
}

fn opts_json(opts: &[(u8, BigRational)]) -> String {
    let items: Vec<String> = opts
        .iter()
        .map(|(t, v)| {
            format!(
                "[\"{}\",{},\"{}/{}\"]",
                tile_name(*t),
                bp(v),
                v.numer(),
                v.denom()
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

fn names_json(tiles: &[u8]) -> String {
    let items: Vec<String> = tiles
        .iter()
        .map(|&t| format!("\"{}\"", tile_name(t)))
        .collect();
    format!("[{}]", items.join(","))
}

/// Fixed pip-trump heuristic for the internal bidder (protocol
/// simplification, mirroring the arena's dropped-30 declaration heuristic):
/// longest suit, then holding the double, then higher pip.
fn declare_heuristic(hand: u32) -> Decl {
    let mut best: Option<(usize, bool, u8)> = None;
    for p in 0u8..=6 {
        let pip = Pip::new(p).expect("pip");
        let mut len = 0usize;
        let mut has_double = false;
        for t in mask_bits(hand) {
            let dm = Domino::from_index(usize::from(t)).expect("tile");
            if dm.hi() == pip || dm.lo() == pip {
                len += 1;
                if dm.hi() == dm.lo() {
                    has_double = true;
                }
            }
        }
        let cand = (len, has_double, p);
        let better = best.is_none_or(|b| cand > b);
        if better {
            best = Some(cand);
        }
    }
    let (_, _, p) = best.expect("some pip");
    Decl::PipTrump(Pip::new(p).expect("pip"))
}

fn regime_name(shadow: usize) -> &'static str {
    match shadow {
        1 => "self-bid",
        3 => "partner-bid",
        2 => "defense",
        _ => unreachable!("shadow seat"),
    }
}

#[allow(clippy::too_many_lines)]
fn run_hand(h: u64, n_outer: usize, budget_l1: u64, budget_l2: u64, out: &mut impl IoWrite) {
    let mut rng = SplitMix64(MINER_SEED ^ mix(h));
    // Deal: internal S1 is always the bidder (P30, bidding team = T1).
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mut hands = [0u32; 4];
    for (s, hand) in hands.iter_mut().enumerate() {
        for &t in &tiles[7 * s..7 * (s + 1)] {
            *hand |= 1u32 << t;
        }
    }
    let shadow = [1usize, 3, 2][(h % 3) as usize];
    let dcl = declare_heuristic(hands[1]);
    let trump = match dcl {
        Decl::PipTrump(p) => p.value(),
        _ => unreachable!("heuristic is pip-only"),
    };
    let mut st = State {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        voids: [0; 4],
        trick_start_played: 0,
        completed: 0,
    };
    let hands_str: Vec<String> = (0..4).map(|s| names_json(&mask_bits(hands[s]))).collect();
    let mut n_decisions = 0usize;
    let mut n_divergent = 0usize;
    let mut record: Vec<String> = Vec::new();
    while st.played != FULL_MASK {
        let seat_idx = (usize::from(st.leader) + st.plays.len()) % 4;
        let seat = Seat::from_index(seat_idx).expect("seat");
        let hand = hands[seat_idx] & !st.played;
        let led: Option<Context> = st
            .plays
            .first()
            .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led index")));
        let legal = mask_of(legal_plays(dcl, set_of(hand), led));
        let choice: u8;
        if legal.count_ones() == 1 {
            choice = legal.trailing_zeros() as u8;
        } else {
            // The actor's real decision: level-1, bridge-strength.
            let t_l1 = Instant::now();
            let base = match sample_belief(
                seat_idx,
                hand,
                st.played,
                sizes_at(&st),
                st.voids,
                n_outer,
                &mut rng,
            ) {
                Ok(worlds) => worlds,
                Err(frame) => {
                    eprintln!("divergence: hand {h} belief draw refused ({frame}) — hand dropped");
                    return;
                }
            };
            let l1 = eval_options(
                dcl, seat, hand, legal, &st, 1, n_outer, budget_l1, &base, &mut rng,
            );
            let l1_ms = t_l1.elapsed().as_millis();
            let (l1_opts, l1_choice, l1_tie) = match l1 {
                Some(opts) => {
                    let ties = argmax_set(&opts, seat.team() == Team::T1);
                    (Some(opts), ties[0], ties)
                }
                None => (None, legal.trailing_zeros() as u8, Vec::new()),
            };
            choice = l1_choice;
            if let (true, Some(l1_opts)) = (seat_idx == shadow, l1_opts) {
                // Shadow: the same decision as a level-2 walt, same base worlds.
                let t_l2 = Instant::now();
                let l2 = eval_options(
                    dcl, seat, hand, legal, &st, 2, n_outer, budget_l2, &base, &mut rng,
                );
                let l2_ms = t_l2.elapsed().as_millis();
                n_decisions += 1;
                let line = match l2 {
                    Some(l2_opts) => {
                        let maximize = seat.team() == Team::T1;
                        let l2_tie = argmax_set(&l2_opts, maximize);
                        let l2_best = if maximize {
                            l2_opts.iter().map(|(_, v)| v.clone()).max()
                        } else {
                            l2_opts.iter().map(|(_, v)| v.clone()).min()
                        }
                        .expect("legal");
                        let v2_of_l1 = l2_opts
                            .iter()
                            .find(|(t, _)| *t == l1_choice)
                            .expect("l1 choice is legal")
                            .1
                            .clone();
                        let divergent = if maximize {
                            v2_of_l1 < l2_best
                        } else {
                            v2_of_l1 > l2_best
                        };
                        let gap = if maximize {
                            bp(&l2_best) - bp(&v2_of_l1)
                        } else {
                            bp(&v2_of_l1) - bp(&l2_best)
                        };
                        if divergent {
                            n_divergent += 1;
                        }
                        format!(
                            "{{\"hand\":{h},\"regime\":\"{}\",\"shadow\":{shadow},\"trump\":{trump},\"trick\":{},\"pos\":{},\"seat_hand\":{},\"record\":[{}],\"l1\":{},\"l2\":{},\"l1_choice\":\"{}\",\"l1_tie\":{},\"l2_tie\":{},\"divergent\":{},\"gap_bp\":{},\"l1_ms\":{l1_ms},\"l2_ms\":{l2_ms}}}",
                            regime_name(shadow),
                            st.completed + 1,
                            st.plays.len(),
                            names_json(&mask_bits(hand)),
                            record.join(","),
                            opts_json(&l1_opts),
                            opts_json(&l2_opts),
                            tile_name(l1_choice),
                            names_json(&l1_tie),
                            names_json(&l2_tie),
                            divergent,
                            gap,
                        )
                    }
                    None => format!(
                        "{{\"hand\":{h},\"regime\":\"{}\",\"shadow\":{shadow},\"trump\":{trump},\"trick\":{},\"pos\":{},\"l2\":\"timeout\",\"l1_choice\":\"{}\",\"l1_ms\":{l1_ms}}}",
                        regime_name(shadow),
                        st.completed + 1,
                        st.plays.len(),
                        tile_name(l1_choice),
                    ),
                };
                writeln!(out, "{line}").expect("write");
                out.flush().expect("flush");
            }
        }
        record.push(format!(
            "{{\"seat\":{seat_idx},\"tile\":\"{}\"}}",
            tile_name(choice)
        ));
        advance(
            &mut st,
            dcl,
            Domino::from_index(usize::from(choice)).expect("choice"),
        );
    }
    let made = st.banked_t1 >= 30;
    writeln!(
        out,
        "{{\"hand\":{h},\"summary\":true,\"regime\":\"{}\",\"trump\":{trump},\"made\":{made},\"banked_t1\":{},\"banked_t0\":{},\"decisions\":{n_decisions},\"divergent\":{n_divergent},\"hands\":[{}]}}",
        regime_name(shadow),
        st.banked_t1,
        st.banked_t0,
        hands_str.join(","),
    )
    .expect("write");
    out.flush().expect("flush");
    eprintln!(
        "hand {h} ({}) done: made={made} banked {}:{} decisions={n_decisions} divergent={n_divergent}",
        regime_name(shadow),
        st.banked_t1,
        st.banked_t0,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_hands: u64 = args
        .get(1)
        .map(|s| s.parse().expect("hand count"))
        .unwrap_or(200);
    let start: u64 = args
        .get(2)
        .map(|s| s.parse().expect("start hand"))
        .unwrap_or(0);
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer sample size"))
        .unwrap_or(50);
    let budget_l1: u64 = args
        .get(4)
        .map(|s| s.parse().expect("level-1 budget secs"))
        .unwrap_or(60);
    let budget_l2: u64 = args
        .get(5)
        .map(|s| s.parse().expect("level-2 budget secs"))
        .unwrap_or(300);
    eprintln!(
        "divergence miner: EXPLORATORY level-1 self-play with level-2 shadow; hands {start}..{} n_outer={n_outer} budgets l1={budget_l1}s l2={budget_l2}s, {} rayon threads",
        start + n_hands,
        rayon::current_num_threads()
    );
    eprintln!(
        "nothing above exploratory tier; sampled values are ESTIMATES; not a P-A21 statement"
    );
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for h in start..start + n_hands {
        run_hand(h, n_outer, budget_l1, budget_l2, &mut out);
    }
}
