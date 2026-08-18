//! EXPLORATORY BID-CURVE PROBE — sits below every evidentiary tier and is
//! cited by nothing above it (wiki/ideas tier; outputs are ESTIMATES).
//!
//! The substrate of baseline bidding (SCENARIO-PLAYER.md §6.2): the pmake
//! objective parameterizes by bid level b — make ⇔ banked(T1) ≥ b ⇔
//! banked(T0) ≤ 42−b — so the same level-1 solver prices every (declaration,
//! bid) cell at the auction point. This probe deals random 7-tile hands to
//! the internal bidder S1 and computes P(make b) for all nine declarations
//! and b in 30..=42, over COMMON RANDOM WORLDS per hand (one belief sample
//! shared by every cell, so curves are comparable within a hand).
//!
//! The seat PLAYS TO THE BID at every cell: viewer decisions, decided
//! cutoffs, and every modeled mind's objective all use b's thresholds.
//! Consequence (spec note): P(make b) is NOT guaranteed monotone in b —
//! raising b changes the modeled field's behavior too, so adjacent cells
//! are values in slightly different games. Monotonicity is checked softly
//! and violations are reported, not asserted.
//!
//! A baseline bidder is a rule over these curves (e.g. "bid the highest b
//! with P(make b) ≥ θ, else pass"); rule selection is analysis downstream
//! of this output, not baked in here.
//!
//! Solver: the banked-correct, rayon-parallel level-k machinery
//! (divergence.rs lineage, level-1 configuration: field = level-0 minds,
//! n_inner = [8]). Arithmetic: integer counts and BigRational values, no
//! floats. Nothing here is quotable above exploratory tier; not a P-A21
//! statement.

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

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};

/// All 28 tiles.
const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Frozen seed for inner-mind sampling (identical to the whole stack).
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

/// Frozen bid-curve stream seed (fresh constant).
const BID_SEED: u64 = 0x4528_21E6_38D0_1377;

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

fn mask_bits(mask: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(mask.count_ones() as usize);
    let mut m = mask;
    while m != 0 {
        v.push(m.trailing_zeros() as u8);
        m &= m - 1;
    }
    v
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

#[derive(Clone, Copy)]
enum Field {
    Dice,
    Level(usize),
}

/// The mind's entire information state, banked totals included
/// (SCENARIO-PLAYER.md §3.1/§3.4).
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

const PI_SHARDS: usize = 64;

type PiShard = Mutex<HashMap<(u8, PiKey), u8>>;

struct Shared {
    dcl: Decl,
    /// The bid level: make ⇔ banked_t1 ≥ bid ⇔ banked_t0 ≤ 42 − bid.
    bid: u8,
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

fn sample_belief(viewer_hand: u32, n: usize, rng: &mut SplitMix64) -> Vec<[u32; 4]> {
    // Auction point: nothing played, no voids — the belief is the full
    // sizes-fiber over the other 21 tiles.
    let unseen = FULL_MASK & !viewer_hand;
    let mut tiles = mask_bits(unseen);
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    let mut out: Vec<[u32; 4]> = Vec::with_capacity(n);
    for _ in 0..n {
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mut w = [0u32; 4];
        w[1] = viewer_hand;
        w[2] = mask_slice(&tiles[0..7]);
        w[3] = mask_slice(&tiles[7..14]);
        w[0] = mask_slice(&tiles[14..21]);
        out.push(w);
    }
    out
}

fn all_decls() -> Vec<(String, Decl)> {
    let mut v: Vec<(String, Decl)> = (0u8..=6)
        .map(|p| (format!("P{p}"), Decl::PipTrump(Pip::new(p).expect("pip"))))
        .collect();
    v.push(("DT".to_string(), Decl::DoublesTrump));
    v.push(("NT".to_string(), Decl::NoTrump));
    v
}

fn bp(v: &BigRational) -> i64 {
    let scaled = (v * BigRational::from_integer(BigInt::from(10_000))).to_integer();
    scaled.to_string().parse().unwrap_or(0)
}

fn tile_name(t: u8) -> String {
    let dm = Domino::from_index(usize::from(t)).expect("tile");
    format!("{}{}", dm.hi().value(), dm.lo().value())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_hands: u64 = args
        .get(1)
        .map(|s| s.parse().expect("hand count"))
        .unwrap_or(10);
    let start: u64 = args
        .get(2)
        .map(|s| s.parse().expect("start hand"))
        .unwrap_or(0);
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer sample size"))
        .unwrap_or(50);
    let budget: u64 = args
        .get(4)
        .map(|s| s.parse().expect("per-cell budget secs"))
        .unwrap_or(120);
    println!("bid-curve probe: EXPLORATORY P(make b) per (declaration, bid) at the auction point");
    println!(
        "level-1 seat (field = level-0 minds, n0=8), n_outer={n_outer} CRN per hand, bids 30..=42, {} rayon threads",
        rayon::current_num_threads()
    );
    println!("SAMPLED — values are ESTIMATES; monotonicity in b is checked softly (see header)");
    println!();
    for h in start..start + n_hands {
        let mut rng = SplitMix64(BID_SEED ^ mix(h));
        let mut tiles: Vec<u8> = (0..28).collect();
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let hand: u32 = tiles[0..7].iter().fold(0, |a, &t| a | (1u32 << t));
        let names: Vec<String> = mask_bits(hand).iter().map(|&t| tile_name(t)).collect();
        println!("hand {h}: [{}]", names.join(" "));
        let worlds = sample_belief(hand, n_outer, &mut rng);
        let t0 = Instant::now();
        for (dname, dcl) in all_decls() {
            let mut row: Vec<String> = Vec::new();
            let mut prev: Option<BigRational> = None;
            let mut mono_violations = 0usize;
            for b in 30u8..=42 {
                let sh = Arc::new(Shared {
                    dcl,
                    bid: b,
                    n_inner: vec![8],
                    boundary_played: 0,
                    boundary_hand_size: 7,
                    deadline: Instant::now() + std::time::Duration::from_secs(budget),
                    pi_cache: (0..PI_SHARDS).map(|_| Mutex::new(HashMap::new())).collect(),
                    pi_calls: AtomicU64::new(0),
                    nodes: AtomicU64::new(0),
                    dead: AtomicBool::new(false),
                });
                let solver = Solver::new(
                    Arc::clone(&sh),
                    Seat::from_index(1).expect("seat 1"),
                    hand,
                    true,
                    worlds.clone(),
                    Vec::new(),
                    Field::Level(0),
                )
                .parallel();
                let root = Key {
                    played: 0,
                    leader: 1,
                    plays: Vec::new(),
                    banked_t1: 0,
                    banked_t0: 0,
                    alive: 0,
                };
                let v = solver.solve(&root);
                solver.flush_nodes();
                match v {
                    Some(v) => {
                        if let Some(p) = &prev {
                            if v > *p {
                                mono_violations += 1;
                            }
                        }
                        row.push(format!("{b}:{:>5}", bp(&v)));
                        prev = Some(v);
                    }
                    None => {
                        row.push(format!("{b}:  DIED"));
                        prev = None;
                    }
                }
            }
            println!(
                "  {dname:>2}  {}{}",
                row.join(" "),
                if mono_violations > 0 {
                    format!("   [mono-viol x{mono_violations}]")
                } else {
                    String::new()
                }
            );
        }
        let ms = t0.elapsed().as_millis();
        println!("  ({}.{:03}s)", ms / 1000, ms % 1000);
        println!();
    }
    println!(
        "nothing above exploratory tier; bid-curve values are never receipts; not a P-A21 statement"
    );
}
