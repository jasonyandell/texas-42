//! walt subprocess bridge — an exploratory, non-normative surface that lets
//! an external harness (the mk5 arena) seat walt for one decision per line
//! of stdin. Instrument tooling; creates no receipts and no claims.
//!
//! Speaks the SAME line protocol as rob_bridge, so the arena's existing
//! subprocess adapter works unchanged (`rob:<path-to-walt_bridge>`):
//!
//! request:  `seat decl bidder h0 h1 h2 h3 h4 h5 h6 n (actor domino)*n`
//!   - `seat`   viewer seat 0..4 (the seat walt decides for)
//!   - `decl`   declaration id: 0..=6 pip trump, 7 doubles-trump, 9 no-trump
//!   - `bidder` the auction winner's seat (leads trick one)
//!   - `h0..h6` the viewer's seven ORIGINALLY DEALT domino ids in the
//!     canonical triangular order `(0,0)=0, (1,0)=1, ..., (6,6)=27`
//!   - `n` then `n` chronological `(actor, domino)` pairs of every play so
//!     far, including the viewer's own
//!
//! reply: `domino leader points0 points1` — the chosen domino id, then
//! walt's own independently derived current trick leader and team point
//! totals after the replayed history (team = seat % 2), so the harness can
//! assert rules conformance (walt-core vs its engine) on every decision.
//!
//! Extra request kind (walt names trump when dropped on at 30):
//!
//! request:  `declare bidder h0 h1 h2 h3 h4 h5 h6`
//! reply:    a single declaration id — argmax P(make 30) over the bidder's
//!           belief sample; pip trumps 0..6 only unless WALT_DECLARE_FULL=1
//!           (then doubles=7 and no-trump=9 join the candidates).
//!
//! The contract is bid-value-blind and always a P(30): walt's objective IS
//! P(make 30), the Boolean the arena's mark scoring pays out on. The
//! bidding team must be internal T1 (seats 1,3), so when the arena bidder
//! sits on an even seat every seat label is rotated by +1 internally
//! (turn order and partnerships are preserved) and rotated back in replies.
//!
//! Decisions are level-1 (playtable.rs policy, identical seeds and
//! saturation-tie refinement) and deterministic per information state.
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Seat, Team};

const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Frozen seed for level-0 inner sampling (MUST match level1.rs so walt
/// models exactly the level-0 policy the rest of the family does).
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;
/// Frozen seed for the bridge's outer belief sampling (e digits).
const BRIDGE_SEED: u64 = 0xB7E1_5162_8AED_2A6B;

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

type Alive = Rc<Vec<u32>>;

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

enum FieldModel {
    Dice,
    Policy,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PiKey {
    seat: u8,
    hand: u32,
    played: u32,
    leader: u8,
    plays: Vec<u8>,
}

struct Solver {
    dcl: Decl,
    viewer: Seat,
    viewer_hand0: u32,
    maximize: bool,
    worlds: Vec<[u32; 4]>,
    seeds: Vec<u64>,
    field_model: FieldModel,
    /// Played mask at the start of the current trick (all seats equal-sized).
    boundary_played: u32,
    boundary_hand_size: usize,
    n0: usize,
    pi_cache: HashMap<PiKey, u8>,
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

    fn solve(&mut self, key: &Key) -> Option<BigRational> {
        if self.dead {
            return None;
        }
        self.nodes += 1;
        if self.nodes & 0xFFFF == 0 && Instant::now() >= self.deadline {
            self.dead = true;
            return None;
        }
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
        assert_eq!(redistributed, alive_len, "field partition conservation");
        Some(total)
    }

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

    /// The level-0 policy (identical construction to level1.rs, same seeds).
    fn pi0(&mut self, key: &Key, seat: Seat, hand: u32, legal_mask: u32) -> Option<u8> {
        let pk = PiKey {
            seat: seat.index() as u8,
            hand,
            played: key.played,
            leader: key.leader,
            plays: key.plays.clone(),
        };
        if let Some(&t) = self.pi_cache.get(&pk) {
            return Some(t);
        }
        let choice = self
            .pi0_evaluate(key, seat, hand, legal_mask)?
            .into_iter()
            .reduce(|best, cand| {
                let better = if seat.team() == Team::T1 {
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
            .expect("legal play exists")
            .0;
        self.pi_cache.insert(pk, choice);
        Some(choice)
    }

    /// Level-0 evaluation of every legal play at a seat's information state:
    /// inner Dice solve, frozen seeds. Ascending tile order (so reduce keeps
    /// the lowest index on ties).
    fn pi0_evaluate(
        &mut self,
        key: &Key,
        seat: Seat,
        hand: u32,
        legal_mask: u32,
    ) -> Option<Vec<(u8, BigRational)>> {
        let sizes = self.hand_sizes_at(key);
        let unseen = FULL_MASK & !key.played & !hand;
        let others: Vec<usize> = (0..4).filter(|&s| s != seat.index()).collect();
        let need: usize = others.iter().map(|&s| sizes[s]).sum();
        assert_eq!(unseen.count_ones() as usize, need, "unseen tiles fit sizes");
        let mut rng = SplitMix64(
            INNER_SEED ^ mix(seat.index() as u64) ^ mix(u64::from(hand)) ^ record_hash(key),
        );
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
        let mut out: Vec<(u8, BigRational)> = Vec::new();
        let mut lm = legal_mask;
        while lm != 0 {
            let tile_idx = lm.trailing_zeros() as u8;
            lm &= lm - 1;
            let tile = Domino::from_index(usize::from(tile_idx)).expect("tile < 28");
            let child = inner.child_after_play(&root, tile, 0);
            let v = inner.solve(&child)?;
            out.push((tile_idx, v));
        }
        self.nodes += inner.nodes;
        Some(out)
    }
}

/// Sample a seat's belief: deals consistent with the viewer's remaining
/// hand, the played mask, the record's hand sizes, and every void observed
/// so far in play (any seat can be the viewer).
fn sample_belief(
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

/// The level-1 evaluation with saturation-tie refinement (identical policy
/// to playtable.rs). Returns every legal option's estimate.
#[allow(clippy::too_many_arguments)]
fn level1_evaluate(
    dcl: Decl,
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
) -> Option<Vec<(u8, BigRational)>> {
    let deadline = Instant::now() + std::time::Duration::from_secs(per_move_secs);
    let maximize = seat.team() == Team::T1;
    let evaluate = |tiles: &[u8], n: usize, rng: &mut SplitMix64| {
        let worlds = sample_belief(seat.index(), hand, key.played, sizes, voids, n, rng);
        let mut solver = Solver::new(
            dcl,
            seat,
            hand,
            maximize,
            worlds,
            Vec::new(),
            FieldModel::Policy,
            trick_start_played,
            boundary_hand_size,
            n0,
            deadline,
        );
        let mut out: Vec<(u8, BigRational)> = Vec::new();
        for &t in tiles {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(key, tile, 0);
            match solver.solve(&child) {
                Some(v) => out.push((t, v)),
                None => return None,
            }
        }
        Some(out)
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
    Some(opts)
}

fn best_of(opts: &[(u8, BigRational)], maximize: bool) -> u8 {
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
// Bridge
// ---------------------------------------------------------------------------

fn decl_of(arena_id: usize) -> Decl {
    match arena_id {
        p @ 0..=6 => Decl::ALL[p],
        7 => Decl::DoublesTrump,
        9 => Decl::NoTrump,
        other => panic!("declaration id {other} is not a straight-42 declaration"),
    }
}

fn arena_decl_id(d: Decl) -> usize {
    match d {
        Decl::PipTrump(p) => usize::from(p.value()),
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 9,
    }
}

/// The replayed public record, in INTERNAL seat labels (arena + r mod 4,
/// with r chosen so the bidding team is internal T1 = seats 1,3).
struct Replayed {
    r: usize,
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    banked_t1: u8,
    banked_t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    completed: usize,
}

fn replay(dcl: Decl, bidder_arena: usize, pairs: &[(usize, usize)]) -> Replayed {
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

struct Config {
    n_outer: usize,
    n0: usize,
    per_move_secs: u64,
    n_declare: usize,
}

fn decide(nums: &[usize], cfg: &Config) -> (usize, usize, u8, u8) {
    assert!(nums.len() >= 11, "request needs seat decl bidder h0..h6 n");
    let dcl = decl_of(nums[1]);
    let bidder_arena = nums[2];
    let n_history = nums[10];
    assert_eq!(
        nums.len(),
        11 + 2 * n_history,
        "history carries exactly n (actor, domino) pairs"
    );
    let pairs: Vec<(usize, usize)> = nums[11..].chunks_exact(2).map(|c| (c[0], c[1])).collect();
    let st = replay(dcl, bidder_arena, &pairs);

    let viewer_i = (nums[0] + st.r) % 4;
    let seat = Seat::from_index(viewer_i).expect("seat 0..4");
    let mut hand0: u32 = 0;
    for &raw in &nums[3..10] {
        hand0 |= bit(Domino::from_index(raw).expect("domino id 0..28"));
    }
    assert_eq!(hand0.count_ones(), 7, "seven distinct dealt tiles");
    let expect = (usize::from(st.leader) + st.plays.len()) % 4;
    assert_eq!(viewer_i, expect, "viewer is the seat to act");

    let hand = hand0 & !st.played;
    let led: Option<Context> = st
        .plays
        .first()
        .map(|&i| dcl.led_context(Domino::from_index(usize::from(i)).expect("led tile")));
    let legal = mask_of(legal_plays(dcl, set_of(hand), led));
    let chosen = if legal.count_ones() == 1 {
        legal.trailing_zeros() as u8
    } else {
        let key = Key {
            played: st.played,
            leader: st.leader,
            plays: st.plays.clone(),
            banked_t1: st.banked_t1,
            banked_t0: st.banked_t0,
            alive: 0,
        };
        let mut sizes = [7 - st.completed; 4];
        for i in 0..st.plays.len() {
            sizes[(usize::from(st.leader) + i) % 4] -= 1;
        }
        let mut rng = SplitMix64(BRIDGE_SEED ^ mix(u64::from(hand0)) ^ record_hash(&key));
        match level1_evaluate(
            dcl,
            seat,
            hand,
            legal,
            &key,
            sizes,
            st.voids,
            st.trick_start_played,
            7 - st.completed,
            cfg.n_outer,
            cfg.n0,
            cfg.per_move_secs,
            &mut rng,
        ) {
            Some(opts) => best_of(&opts, seat.team() == Team::T1),
            None => {
                eprintln!("walt_bridge: eval deadline hit; playing lowest legal");
                legal.trailing_zeros() as u8
            }
        }
    };

    // Reply in arena labels. Arena team0 = arena seats {0,2}; under rotation
    // r those are internal T1 exactly when r == 1.
    let leader_arena = (usize::from(st.leader) + 4 - st.r) % 4;
    let (points0, points1) = if st.r == 1 {
        (st.banked_t1, st.banked_t0)
    } else {
        (st.banked_t0, st.banked_t1)
    };
    (usize::from(chosen), leader_arena, points0, points1)
}

fn declare(nums: &[usize], cfg: &Config, full: bool) -> usize {
    assert_eq!(nums.len(), 8, "declare needs bidder h0..h6");
    let bidder_arena = nums[0];
    let r = if bidder_arena.is_multiple_of(2) { 1 } else { 0 };
    let bidder_i = (bidder_arena + r) % 4;
    let seat = Seat::from_index(bidder_i).expect("bid seat");
    let mut hand0: u32 = 0;
    for &raw in &nums[1..8] {
        hand0 |= bit(Domino::from_index(raw).expect("domino id 0..28"));
    }
    assert_eq!(hand0.count_ones(), 7, "seven distinct dealt tiles");

    let candidates: Vec<Decl> = if full {
        vec![
            Decl::ALL[0],
            Decl::ALL[1],
            Decl::ALL[2],
            Decl::ALL[3],
            Decl::ALL[4],
            Decl::ALL[5],
            Decl::ALL[6],
            Decl::DoublesTrump,
            Decl::NoTrump,
        ]
    } else {
        Decl::ALL[..7].to_vec()
    };

    let mut rng = SplitMix64(BRIDGE_SEED ^ mix(u64::from(hand0)) ^ mix(0xDEC1));
    let deadline = Instant::now() + std::time::Duration::from_secs(cfg.per_move_secs);
    let root = Key {
        played: 0,
        leader: bidder_i as u8,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    let eval = |dcl: Decl, worlds: Vec<[u32; 4]>| -> BigRational {
        let mut solver = Solver::new(
            dcl,
            seat,
            hand0,
            true,
            worlds,
            Vec::new(),
            FieldModel::Policy,
            0,
            7,
            cfg.n0,
            deadline,
        );
        let mut best = BigRational::zero();
        for t in mask_bits(hand0) {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(&root, tile, 0);
            match solver.solve(&child) {
                Some(v) => {
                    if v > best {
                        best = v;
                    }
                }
                None => break,
            }
        }
        best
    };

    let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], cfg.n_declare, &mut rng);
    let mut vals: Vec<(Decl, BigRational)> = candidates
        .iter()
        .map(|&d| (d, eval(d, worlds.clone())))
        .collect();
    // Saturation ties across declarations: look closer on fresh samples.
    let mut n_cur = cfg.n_declare;
    loop {
        let best = vals
            .iter()
            .map(|(_, v)| v.clone())
            .max()
            .expect("candidates");
        let tied: Vec<Decl> = vals
            .iter()
            .filter(|(_, v)| *v == best)
            .map(|(d, _)| *d)
            .collect();
        if tied.len() == 1 || n_cur >= cfg.n_declare * 16 {
            break;
        }
        n_cur *= 4;
        let worlds = sample_belief(bidder_i, hand0, 0, [7; 4], [0; 4], n_cur, &mut rng);
        for d in tied {
            let v = eval(d, worlds.clone());
            let slot = vals.iter_mut().find(|(x, _)| *x == d).expect("tied decl");
            slot.1 = v;
        }
    }
    let chosen = vals
        .into_iter()
        .reduce(|best, cand| if cand.1 > best.1 { cand } else { best })
        .expect("candidates")
        .0;
    arena_decl_id(chosen)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cfg = Config {
        n_outer: args
            .get(1)
            .map(|s| s.parse().expect("n_outer"))
            .unwrap_or(50),
        n0: args.get(2).map(|s| s.parse().expect("n0")).unwrap_or(8),
        per_move_secs: args
            .get(3)
            .map(|s| s.parse().expect("per_move_secs"))
            .unwrap_or(120),
        n_declare: args
            .get(4)
            .map(|s| s.parse().expect("n_declare"))
            .unwrap_or(100),
    };
    let declare_full = std::env::var("WALT_DECLARE_FULL").is_ok_and(|v| v == "1");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let line = line.expect("stdin line");
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("declare ") {
            let nums: Vec<usize> = rest
                .split_whitespace()
                .map(|t| t.parse().expect("integer token"))
                .collect();
            let decl_id = declare(&nums, &cfg, declare_full);
            writeln!(out, "{decl_id}").expect("stdout write");
        } else {
            let nums: Vec<usize> = trimmed
                .split_whitespace()
                .map(|t| t.parse().expect("integer token"))
                .collect();
            let (chosen, leader, p0, p1) = decide(&nums, &cfg);
            writeln!(out, "{chosen} {leader} {p0} {p1}").expect("stdout write");
        }
        out.flush().expect("stdout flush");
    }
}
