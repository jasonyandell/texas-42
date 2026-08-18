//! EXPLORATORY WEB TABLE — sits below every evidentiary tier and is cited by
//! nothing above it. The interactive table (playtable.rs) with a browser
//! front end: seats drawn around a table, domino glyphs, play clockwise.
//!
//! New here: the bidder PICKS TRUMP. Still dropped-on-you at 30 (no
//! auction), but the bid seat evaluates all nine declarations (P0..P6,
//! doubles, no-trump) over a shared belief sample — common random numbers
//! across declarations — and names the argmax-P(make) trump. Saturation
//! ties across declarations are re-examined on 4x fresh samples up to 16x,
//! the same "look closer" rule the play level uses.
//!
//! One human seat (switchable per hand), three level-1 seats. Nobody peeks:
//! the AIs sample beliefs from their own chairs; `hint` runs the SAME honest
//! evaluation from the human's chair. Single process, localhost only.
//!
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};

const FULL_MASK: u32 = 0x0FFF_FFFF;
const BIDDER: usize = 1;

/// Frozen seed for level-0 inner sampling (MUST match level1.rs so the field
/// seats here play exactly the policy the level-1 solver models).
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
            banked_t1: key.banked_t1,
            banked_t0: key.banked_t0,
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

/// Basis points (0..=10000) of a probability; display-side division only.
fn bp(v: &BigRational) -> u32 {
    let s = (v * BigRational::from_integer(BigInt::from(10_000)))
        .to_integer()
        .to_string();
    s.parse().unwrap_or(0)
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

fn tile_str(idx: u8) -> String {
    let dm = Domino::from_index(usize::from(idx)).expect("tile < 28");
    format!("{}-{}", dm.hi().value(), dm.lo().value())
}

fn decl_name(i: usize) -> &'static str {
    [
        "blanks", "aces", "deuces", "tres", "fours", "fives", "sixes", "doubles", "no-trump",
    ][i]
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
// Game state
// ---------------------------------------------------------------------------

struct PlayRec {
    seat: u8,
    tile: u8,
    forced: bool,
    opts: Vec<(u8, u32)>,
}

struct TrickRec {
    plays: Vec<PlayRec>,
    winner: u8,
    pts: u8,
}

#[derive(PartialEq)]
enum Phase {
    Trump,
    Play,
    Done,
}

struct Game {
    human: usize,
    hand_no: u64,
    seed: u64,
    n_outer: usize,
    n0: usize,
    per_move_secs: u64,
    rng: SplitMix64,
    hands: [u32; 4],
    dcl: Option<Decl>,
    decl_idx: Option<usize>,
    trump_worlds: Vec<[u32; 4]>,
    trump_vals: Vec<(usize, u8, BigRational)>,
    phase: Phase,
    played: u32,
    leader: u8,
    cur: Vec<PlayRec>,
    banked_t1: u8,
    banked_t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    tricks: Vec<TrickRec>,
    hint: Option<Vec<(u8, u32)>>,
    msgs: Vec<String>,
    makes: u32,
    sets: u32,
}

impl Game {
    fn new(human: usize, seed: u64, n_outer: usize, n0: usize, per_move_secs: u64) -> Game {
        let mut g = Game {
            human,
            hand_no: 0,
            seed,
            n_outer,
            n0,
            per_move_secs,
            rng: SplitMix64(seed),
            hands: [0; 4],
            dcl: None,
            decl_idx: None,
            trump_worlds: Vec::new(),
            trump_vals: Vec::new(),
            phase: Phase::Trump,
            played: 0,
            leader: BIDDER as u8,
            cur: Vec::new(),
            banked_t1: 0,
            banked_t0: 0,
            voids: [0; 4],
            trick_start_played: 0,
            tricks: Vec::new(),
            hint: None,
            msgs: Vec::new(),
            makes: 0,
            sets: 0,
        };
        g.new_hand(human);
        g
    }

    fn new_hand(&mut self, human: usize) {
        self.human = human;
        self.hand_no += 1;
        self.rng = SplitMix64(self.seed ^ mix(self.hand_no));
        let mut tiles: Vec<u8> = (0..28).collect();
        for i in (1..tiles.len()).rev() {
            let j = self.rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
        self.hands = [
            mask_slice(&tiles[0..7]),
            mask_slice(&tiles[7..14]),
            mask_slice(&tiles[14..21]),
            mask_slice(&tiles[21..28]),
        ];
        self.dcl = None;
        self.decl_idx = None;
        self.trump_vals = Vec::new();
        self.trump_worlds = sample_belief(
            BIDDER,
            self.hands[BIDDER],
            0,
            [7; 4],
            [0; 4],
            self.n_outer,
            &mut self.rng,
        );
        self.phase = Phase::Trump;
        self.played = 0;
        self.leader = BIDDER as u8;
        self.cur = Vec::new();
        self.banked_t1 = 0;
        self.banked_t0 = 0;
        self.voids = [0; 4];
        self.trick_start_played = 0;
        self.tricks = Vec::new();
        self.hint = None;
        self.msgs.push(format!(
            "hand {}: dealt. S{BIDDER} holds the 30 bid (dropped on).",
            self.hand_no
        ));
    }

    /// Evaluate one declaration for the bid seat over the given worlds:
    /// best opening lead and its P(make).
    fn eval_decl(&mut self, decl_idx: usize, worlds: Vec<[u32; 4]>) -> (u8, BigRational) {
        let dcl = Decl::ALL[decl_idx];
        let deadline = Instant::now() + std::time::Duration::from_secs(self.per_move_secs);
        let hand = self.hands[BIDDER];
        let seat = Seat::from_index(BIDDER).expect("bid seat");
        let mut solver = Solver::new(
            dcl,
            seat,
            hand,
            true,
            worlds,
            Vec::new(),
            FieldModel::Policy,
            0,
            7,
            self.n0,
            deadline,
        );
        let root = Key {
            played: 0,
            leader: BIDDER as u8,
            plays: Vec::new(),
            banked_t1: 0,
            banked_t0: 0,
            alive: 0,
        };
        let mut best: Option<(u8, BigRational)> = None;
        for t in mask_bits(hand) {
            let tile = Domino::from_index(usize::from(t)).expect("tile");
            let child = solver.child_after_play(&root, tile, 0);
            match solver.solve(&child) {
                Some(v) => {
                    if best.as_ref().is_none_or(|(_, b)| v > *b) {
                        best = Some((t, v));
                    }
                }
                None => break,
            }
        }
        best.unwrap_or((mask_bits(hand)[0], BigRational::zero()))
    }

    /// One trump-phase step: evaluate the next declaration, or (once all
    /// nine are in and the bidder is an AI) refine ties and announce.
    fn step_trump(&mut self) {
        if self.trump_vals.len() < Decl::COUNT {
            let i = self.trump_vals.len();
            let worlds = self.trump_worlds.clone();
            let (t, v) = self.eval_decl(i, worlds);
            self.trump_vals.push((i, t, v));
            return;
        }
        if self.human == BIDDER {
            return; // human names trump via /pick
        }
        // Saturation ties across declarations: look closer on fresh samples.
        let mut n_cur = self.n_outer;
        loop {
            let best = self
                .trump_vals
                .iter()
                .map(|(_, _, v)| v.clone())
                .max()
                .expect("nine evals");
            let tied: Vec<usize> = self
                .trump_vals
                .iter()
                .filter(|(_, _, v)| *v == best)
                .map(|(i, _, _)| *i)
                .collect();
            if tied.len() == 1 || n_cur >= self.n_outer * 16 {
                break;
            }
            n_cur *= 4;
            self.msgs.push(format!(
                "trump tie at the top ({}) — looking closer at n={n_cur}...",
                tied.iter()
                    .map(|&i| decl_name(i))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            let worlds = sample_belief(
                BIDDER,
                self.hands[BIDDER],
                0,
                [7; 4],
                [0; 4],
                n_cur,
                &mut self.rng,
            );
            for &i in &tied {
                let (t, v) = self.eval_decl(i, worlds.clone());
                let slot = self
                    .trump_vals
                    .iter_mut()
                    .find(|(j, _, _)| *j == i)
                    .expect("tied decl present");
                slot.1 = t;
                slot.2 = v;
            }
        }
        let (i, t, v) = self
            .trump_vals
            .iter()
            .cloned()
            .reduce(|best, cand| if cand.2 > best.2 { cand } else { best })
            .expect("nine evals");
        self.announce(i, Some((t, &v)));
    }

    fn announce(&mut self, decl_idx: usize, lead: Option<(u8, &BigRational)>) {
        self.dcl = Some(Decl::ALL[decl_idx]);
        self.decl_idx = Some(decl_idx);
        self.phase = Phase::Play;
        let who = if self.human == BIDDER {
            "you".to_string()
        } else {
            format!("S{BIDDER} walt")
        };
        match lead {
            Some((t, v)) => self.msgs.push(format!(
                "{who} name{} {} — planned lead {}, P(make) ~ {}.{:02}%",
                if self.human == BIDDER { "" } else { "s" },
                decl_name(decl_idx),
                tile_str(t),
                bp(v) / 100,
                bp(v) % 100
            )),
            None => self
                .msgs
                .push(format!("{who} named {}", decl_name(decl_idx))),
        }
    }

    fn turn(&self) -> Option<usize> {
        if self.phase != Phase::Play {
            return None;
        }
        Some((usize::from(self.leader) + self.cur.len()) % 4)
    }

    fn sizes_now(&self) -> [usize; 4] {
        let mut sz = [7 - self.tricks.len(); 4];
        for i in 0..self.cur.len() {
            sz[(usize::from(self.leader) + i) % 4] -= 1;
        }
        sz
    }

    fn key_now(&self) -> Key {
        Key {
            played: self.played,
            leader: self.leader,
            plays: self.cur.iter().map(|p| p.tile).collect(),
            banked_t1: self.banked_t1,
            banked_t0: self.banked_t0,
            alive: 0,
        }
    }

    fn led_now(&self, dcl: Decl) -> Option<Context> {
        self.cur
            .first()
            .map(|p| dcl.led_context(Domino::from_index(usize::from(p.tile)).expect("led")))
    }

    fn evaluate_seat(&mut self, seat_i: usize) -> Option<Vec<(u8, BigRational)>> {
        let dcl = self.dcl.expect("declared");
        let seat = Seat::from_index(seat_i).expect("seat");
        let hand = self.hands[seat_i] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        let key = self.key_now();
        let sizes = self.sizes_now();
        let voids = self.voids;
        let tsp = self.trick_start_played;
        let bhs = 7 - self.tricks.len();
        let (n_outer, n0, pm) = (self.n_outer, self.n0, self.per_move_secs);
        level1_evaluate(
            dcl,
            seat,
            hand,
            legal,
            &key,
            sizes,
            voids,
            tsp,
            bhs,
            n_outer,
            n0,
            pm,
            &mut self.rng,
        )
    }

    /// One play-phase step: the seat to act must be an AI; think and play.
    fn step_play(&mut self) {
        let Some(seat_i) = self.turn() else { return };
        if seat_i == self.human {
            return;
        }
        let dcl = self.dcl.expect("declared");
        let hand = self.hands[seat_i] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        let (tile, forced, opts) = if legal.count_ones() == 1 {
            (legal.trailing_zeros() as u8, true, Vec::new())
        } else {
            let seat = Seat::from_index(seat_i).expect("seat");
            match self.evaluate_seat(seat_i) {
                Some(o) => {
                    let c = best_of(&o, seat.team() == Team::T1);
                    let obp: Vec<(u8, u32)> = o.iter().map(|(t, v)| (*t, bp(v))).collect();
                    (c, false, obp)
                }
                None => {
                    self.msgs
                        .push(format!("S{seat_i} eval timed out; playing lowest legal"));
                    (legal.trailing_zeros() as u8, false, Vec::new())
                }
            }
        };
        self.apply_play(seat_i, tile, forced, opts);
    }

    fn apply_play(&mut self, seat_i: usize, tile_idx: u8, forced: bool, opts: Vec<(u8, u32)>) {
        let dcl = self.dcl.expect("declared");
        let tile = Domino::from_index(usize::from(tile_idx)).expect("tile");
        if let Some(led) = self.led_now(dcl) {
            if !dcl.follows(tile, led) {
                self.voids[seat_i] |= mask_of(dcl.effective_incidence(led));
            }
        }
        self.played |= 1u32 << tile_idx;
        self.cur.push(PlayRec {
            seat: seat_i as u8,
            tile: tile_idx,
            forced,
            opts,
        });
        self.hint = None;
        if self.cur.len() < 4 {
            return;
        }
        let plays: Vec<u8> = self.cur.iter().map(|p| p.tile).collect();
        let doms = [
            Domino::from_index(usize::from(plays[0])).expect("p0"),
            Domino::from_index(usize::from(plays[1])).expect("p1"),
            Domino::from_index(usize::from(plays[2])).expect("p2"),
            Domino::from_index(usize::from(plays[3])).expect("p3"),
        ];
        let trick = Trick::new(
            Seat::from_index(usize::from(self.leader)).expect("leader"),
            doms,
        )
        .expect("distinct");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        if winner.team() == Team::T1 {
            self.banked_t1 += pts;
        } else {
            self.banked_t0 += pts;
        }
        let rec = TrickRec {
            plays: std::mem::take(&mut self.cur),
            winner: winner.index() as u8,
            pts,
        };
        self.tricks.push(rec);
        self.msgs.push(format!(
            ">> S{} takes trick {} (+{pts}) — count T1 {} / T0 {}",
            winner.index(),
            self.tricks.len(),
            self.banked_t1,
            self.banked_t0
        ));
        self.leader = winner.index() as u8;
        self.trick_start_played = self.played;
        if self.tricks.len() == 7 {
            assert_eq!(self.banked_t1 + self.banked_t0, 42, "all points banked");
            self.phase = Phase::Done;
            if self.banked_t1 >= 30 {
                self.makes += 1;
                self.msgs.push(format!(
                    "=== T1 MAKES the 30 bid: {} to {} ===",
                    self.banked_t1, self.banked_t0
                ));
            } else {
                self.sets += 1;
                self.msgs.push(format!(
                    "=== T1 is SET: only {} of 30 (T0 took {}) ===",
                    self.banked_t1, self.banked_t0
                ));
            }
        }
    }

    fn human_play(&mut self, tile_idx: u8) {
        if self.turn() != Some(self.human) {
            return;
        }
        let dcl = self.dcl.expect("declared");
        let hand = self.hands[self.human] & !self.played;
        let legal = mask_of(legal_plays(dcl, set_of(hand), self.led_now(dcl)));
        if legal & (1u32 << tile_idx) == 0 {
            return;
        }
        let opts = self.hint.take().unwrap_or_default();
        self.apply_play(self.human, tile_idx, legal.count_ones() == 1, opts);
    }

    fn human_hint(&mut self) {
        if self.turn() != Some(self.human) || self.hint.is_some() {
            return;
        }
        let seat = Seat::from_index(self.human).expect("seat");
        let maximize = seat.team() == Team::T1;
        if let Some(o) = self.evaluate_seat(self.human) {
            let mut sorted: Vec<(u8, u32)> = o.iter().map(|(t, v)| (*t, bp(v))).collect();
            sorted.sort_by(|a, b| {
                if maximize {
                    b.1.cmp(&a.1)
                } else {
                    a.1.cmp(&b.1)
                }
            });
            self.hint = Some(sorted);
        } else {
            self.msgs.push("hint timed out; play on".to_string());
        }
    }

    fn human_auto(&mut self) {
        if self.turn() != Some(self.human) {
            return;
        }
        self.human_hint();
        if let Some(h) = &self.hint {
            let best = h[0].0;
            self.human_play(best);
        }
    }

    fn human_pick(&mut self, decl_idx: usize) {
        if self.phase != Phase::Trump || self.human != BIDDER || decl_idx >= Decl::COUNT {
            return;
        }
        self.announce(decl_idx, None);
    }

    fn step(&mut self) {
        match self.phase {
            Phase::Trump => self.step_trump(),
            Phase::Play => self.step_play(),
            Phase::Done => {}
        }
    }

    // -- JSON ---------------------------------------------------------------

    fn json_state(&self) -> String {
        let mut s = String::with_capacity(4096);
        s.push('{');
        s.push_str(&format!(
            "\"phase\":\"{}\",",
            match self.phase {
                Phase::Trump => "trump",
                Phase::Play => "play",
                Phase::Done => "done",
            }
        ));
        s.push_str(&format!("\"hand_no\":{},", self.hand_no));
        s.push_str(&format!("\"human\":{},", self.human));
        s.push_str(&format!("\"bidder\":{BIDDER},"));
        match self.decl_idx {
            Some(i) => s.push_str(&format!("\"decl\":{i},")),
            None => s.push_str("\"decl\":null,"),
        }
        s.push_str("\"evals\":[");
        for (k, (i, t, v)) in self.trump_vals.iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push_str(&format!("{{\"d\":{i},\"bp\":{},\"t\":{t}}}", bp(v)));
        }
        s.push_str("],");
        match self.turn() {
            Some(t) => s.push_str(&format!("\"turn\":{t},")),
            None => s.push_str("\"turn\":null,"),
        }
        let sizes: Vec<String> = (0..4)
            .map(|i| (self.hands[i] & !self.played).count_ones().to_string())
            .collect();
        s.push_str(&format!("\"sizes\":[{}],", sizes.join(",")));
        s.push_str(&format!(
            "\"hand\":{},",
            json_tiles(self.hands[self.human] & !self.played)
        ));
        let legal = if self.turn() == Some(self.human) {
            let dcl = self.dcl.expect("declared in play");
            mask_of(legal_plays(
                dcl,
                set_of(self.hands[self.human] & !self.played),
                self.led_now(dcl),
            ))
        } else {
            0
        };
        s.push_str(&format!("\"legal\":{},", json_tiles(legal)));
        s.push_str(&format!(
            "\"banked\":[{},{}],",
            self.banked_t1, self.banked_t0
        ));
        s.push_str(&format!("\"makes\":{},\"sets\":{},", self.makes, self.sets));
        s.push_str("\"tricks\":[");
        for (k, tr) in self.tricks.iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push_str(&format!(
                "{{\"plays\":{},\"w\":{},\"p\":{}}}",
                json_plays(&tr.plays),
                tr.winner,
                tr.pts
            ));
        }
        s.push_str("],");
        s.push_str(&format!("\"cur\":{},", json_plays(&self.cur)));
        if self.phase == Phase::Done {
            let deal: Vec<String> = (0..4).map(|i| json_tiles(self.hands[i])).collect();
            s.push_str(&format!("\"deal\":[{}],", deal.join(",")));
            s.push_str(&format!("\"result\":{},", i32::from(self.banked_t1 >= 30)));
        } else {
            s.push_str("\"deal\":null,\"result\":null,");
        }
        match &self.hint {
            Some(h) => {
                let items: Vec<String> = h.iter().map(|(t, b)| format!("[{t},{b}]")).collect();
                s.push_str(&format!("\"hint\":[{}],", items.join(",")));
            }
            None => s.push_str("\"hint\":null,"),
        }
        s.push_str("\"msg\":[");
        let start = self.msgs.len().saturating_sub(40);
        for (k, m) in self.msgs[start..].iter().enumerate() {
            if k > 0 {
                s.push(',');
            }
            s.push('"');
            for c in m.chars() {
                match c {
                    '"' => s.push_str("\\\""),
                    '\\' => s.push_str("\\\\"),
                    _ => s.push(c),
                }
            }
            s.push('"');
        }
        s.push_str("]}");
        s
    }
}

fn json_tiles(mask: u32) -> String {
    let items: Vec<String> = mask_bits(mask).iter().map(|t| t.to_string()).collect();
    format!("[{}]", items.join(","))
}

fn json_plays(plays: &[PlayRec]) -> String {
    let items: Vec<String> = plays
        .iter()
        .map(|p| {
            let opts: Vec<String> = p.opts.iter().map(|(t, b)| format!("[{t},{b}]")).collect();
            format!(
                "{{\"s\":{},\"t\":{},\"f\":{},\"o\":[{}]}}",
                p.seat,
                p.tile,
                i32::from(p.forced),
                opts.join(",")
            )
        })
        .collect();
    format!("[{}]", items.join(","))
}

// ---------------------------------------------------------------------------
// HTTP
// ---------------------------------------------------------------------------

const PAGE: &str = include_str!("webtable.html");

fn respond(stream: &mut TcpStream, status: &str, ctype: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn query_num(query: &str, name: &str) -> Option<u64> {
    for pair in query.split('&') {
        if let Some(v) = pair.strip_prefix(name) {
            if let Some(v) = v.strip_prefix('=') {
                return v.parse().ok();
            }
        }
    }
    None
}

fn handle(game: &mut Game, stream: &mut TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return;
    }
    loop {
        let mut h = String::new();
        match reader.read_line(&mut h) {
            Ok(0) | Err(_) => break,
            Ok(_) if h == "\r\n" || h == "\n" => break,
            Ok(_) => {}
        }
    }
    let path_full = line.split_whitespace().nth(1).unwrap_or("/");
    let (path, query) = match path_full.split_once('?') {
        Some((p, q)) => (p, q),
        None => (path_full, ""),
    };
    match path {
        "/" => respond(stream, "200 OK", "text/html; charset=utf-8", PAGE),
        "/state" => respond(stream, "200 OK", "application/json", &game.json_state()),
        "/step" => {
            game.step();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/play" => {
            if let Some(t) = query_num(query, "t") {
                if t < 28 {
                    game.human_play(t as u8);
                }
            }
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/pick" => {
            if let Some(d) = query_num(query, "d") {
                game.human_pick(d as usize);
            }
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/hint" => {
            game.human_hint();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/auto" => {
            game.human_auto();
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        "/new" => {
            let seat = query_num(query, "seat")
                .map(|s| (s as usize).min(3))
                .unwrap_or(game.human);
            game.new_hand(seat);
            respond(stream, "200 OK", "application/json", &game.json_state());
        }
        _ => respond(stream, "404 Not Found", "text/plain", "not found"),
    }
}

fn main() {
    // Pip trump ALL-ordering sanity: Decl::ALL[i] is PipTrump(pip i) for i<7.
    for (i, d) in Decl::ALL.iter().enumerate().take(7) {
        assert_eq!(
            *d,
            Decl::PipTrump(Pip::new(i as u8).expect("pip")),
            "Decl::ALL pip order"
        );
    }
    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args
        .get(1)
        .map(|s| s.parse().expect("port"))
        .unwrap_or(4242);
    let n_outer: usize = args
        .get(2)
        .map(|s| s.parse().expect("belief sample size"))
        .unwrap_or(100);
    let n0: usize = args.get(3).map(|s| s.parse().expect("n0")).unwrap_or(8);
    let seed: u64 = args.get(4).map(|s| s.parse().expect("seed")).unwrap_or(42);
    let human: usize = args
        .get(5)
        .map(|s| s.parse().expect("seat 0..=3"))
        .unwrap_or(0);
    assert!(human < 4, "seat must be 0..=3");
    let per_move_secs: u64 = 120;

    let mut game = Game::new(human, seed, n_outer, n0, per_move_secs);
    let listener = TcpListener::bind(("127.0.0.1", port)).expect("bind localhost port");
    println!("walt web table — EXPLORATORY; estimates, never receipts");
    println!("open http://127.0.0.1:{port}  (you are S{human}; switch seats per hand in the UI)");
    println!("n_outer={n_outer} n0={n0} seed={seed}");
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => handle(&mut game, &mut s),
            Err(_) => continue,
        }
    }
}
