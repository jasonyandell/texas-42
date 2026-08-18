//! EXPLORATORY PLAYOUT GENERATOR — sits below every evidentiary tier and is
//! cited by nothing above it. Emits JSON games for the walt viewer.
//!
//! Full games of receipt hand 8's declaration (trump fives, P30 by T1) from
//! trick 1: S1 is walt (level-1: lawful best response over sampled deals
//! against the level-0 field policy, no peeking — its belief is sampled from
//! its own hand, the public record, observed voids, and record hand sizes);
//! S0/S2/S3 play the level-0 policy on their TRUE hands (own hand + record,
//! uniform no-void inner belief, dice model of others, T1 max / T0 min pmake,
//! lowest-index tie break). The deal behind each game is drawn fresh with S1's
//! hand frozen to the receipt hand.
//!
//! Every decision is logged with the decider's evaluated options (exact
//! rational on their sample, basis points for display). ESTIMATES, never
//! receipts; not a P-A21 statement. No floats.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt_core::rules::{legal_plays, Trick};
use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt_m3_carrier::VIEWER;

const FULL_MASK: u32 = 0x0FFF_FFFF;

/// Frozen game seed (distinct stream from ladder/scenario/level1).
const GAME_SEED: u64 = 0x6A09_E667_F3BC_C908;

/// Frozen seed for level-0 inner sampling (MUST match level1.rs so the field
/// seats here play exactly the policy S1's solver models).
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
        let mut rng =
            SplitMix64(INNER_SEED ^ mix(pk_seat(seat)) ^ mix(u64::from(hand)) ^ record_hash(key));
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

fn pk_seat(seat: Seat) -> u64 {
    seat.index() as u64
}

fn bp(v: &BigRational) -> u32 {
    let s = (v * BigRational::from_integer(BigInt::from(10_000)))
        .to_integer()
        .to_string();
    s.parse().unwrap_or(0)
}

fn tile_json(idx: u8) -> String {
    let dm = Domino::from_index(usize::from(idx)).expect("tile < 28");
    format!("[{},{}]", dm.hi().value(), dm.lo().value())
}

fn opts_json(opts: &[(u8, BigRational)]) -> String {
    let parts: Vec<String> = opts
        .iter()
        .map(|(t, v)| {
            format!(
                "{{\"t\":{},\"n\":\"{}\",\"d\":\"{}\",\"bp\":{}}}",
                tile_json(*t),
                v.numer(),
                v.denom(),
                bp(v)
            )
        })
        .collect();
    format!("[{}]", parts.join(","))
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

fn hand_json(mask: u32) -> String {
    let parts: Vec<String> = mask_bits(mask).into_iter().map(tile_json).collect();
    format!("[{}]", parts.join(","))
}

#[allow(clippy::too_many_lines)]
fn play_game(
    dcl: Decl,
    game_idx: usize,
    n_outer: usize,
    n0: usize,
    all_level1: bool,
    deadline: Instant,
) -> Option<String> {
    let s1_full = s1_initial_mask();
    let mut rng = SplitMix64(GAME_SEED ^ mix(game_idx as u64));
    // Deal the other three hands uniformly (no voids exist pre-play).
    let deal = sample_belief(1, s1_full, 0, [7, 7, 7, 7], [0u32; 4], 1, &mut rng)
        .pop()
        .expect("one deal");
    let mut hands = deal; // [s0, s1, s2, s3] full 7-tile hands
    hands[1] = s1_full;

    let mut played: u32 = 0;
    let mut leader: u8 = 1; // S1 leads trick 1 as bidder
    let mut banked_t1: u8 = 0;
    let mut banked_t0: u8 = 0;
    let mut voids = [0u32; 4];
    let mut trick_start_played: u32 = 0;
    // A per-game host solver for the field seats' pi0 (cache persists).
    let mut host = Solver::new(
        dcl,
        VIEWER,
        0,
        true,
        Vec::new(),
        Vec::new(),
        FieldModel::Policy,
        0,
        7,
        n0,
        deadline,
    );
    let mut tricks_json: Vec<String> = Vec::new();
    for completed in 0..7 {
        let mut plays: Vec<u8> = Vec::new();
        let mut plays_json: Vec<String> = Vec::new();
        let mut led: Option<Context> = None;
        for pos in 0..4 {
            let seat_i = (usize::from(leader) + pos) % 4;
            let seat = Seat::from_index(seat_i).expect("seat");
            let hand = hands[seat_i] & !played;
            let legal = mask_of(legal_plays(dcl, set_of(hand), led));
            let key = Key {
                played,
                leader,
                plays: plays.clone(),
                banked_t1,
                banked_t0,
                alive: 0,
            };
            host.boundary_played = trick_start_played;
            host.boundary_hand_size = 7 - completed;
            let level1_here = seat == VIEWER || all_level1;
            let (chosen, opts, walt): (u8, Vec<(u8, BigRational)>, bool) =
                if level1_here && legal.count_ones() > 1 {
                    // Level-1 evaluation over a fresh belief sample from this
                    // seat's chair: own hand, record, observed voids. T1 seats
                    // maximize pmake, T0 seats minimize it.
                    //
                    // Saturation tie-break ("look closer"): a value of 1 on a
                    // small sample only means UNBEATEN IN THE SAMPLE — it cannot
                    // distinguish the master trump (can't lose in any world)
                    // from a lead that merely got lucky (support != belief).
                    // When options tie at the top, re-evaluate ONLY the tied
                    // ones on a 4x fresh sample, repeat up to 16x; certainty
                    // that is real survives scrutiny.
                    let maximize = seat.team() == Team::T1;
                    let sizes = host.hand_sizes_at(&key);
                    let evaluate = |tiles: &[u8], n: usize, rng: &mut SplitMix64| {
                        let worlds = sample_belief(seat_i, hand, played, sizes, voids, n, rng);
                        let mut solver = Solver::new(
                            dcl,
                            seat,
                            hand,
                            maximize,
                            worlds,
                            Vec::new(),
                            FieldModel::Policy,
                            trick_start_played,
                            7 - completed,
                            n0,
                            deadline,
                        );
                        let mut out: Vec<(u8, BigRational)> = Vec::new();
                        for &t in tiles {
                            let tile = Domino::from_index(usize::from(t)).expect("tile");
                            let child = solver.child_after_play(&key, tile, 0);
                            match solver.solve(&child) {
                                Some(v) => out.push((t, v)),
                                None => return None,
                            }
                        }
                        Some(out)
                    };
                    let all_tiles = mask_bits(legal);
                    let mut opts = evaluate(&all_tiles, n_outer, &mut rng)?;
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
                        let refined = evaluate(&tied, n_cur, &mut rng)?;
                        for (t, v) in refined {
                            let slot = opts
                                .iter_mut()
                                .find(|(ot, _)| *ot == t)
                                .expect("tied tile present");
                            slot.1 = v;
                        }
                    }
                    let chosen = opts
                        .iter()
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
                        .0;
                    (chosen, opts, seat == VIEWER)
                } else if legal.count_ones() == 1 {
                    (legal.trailing_zeros() as u8, Vec::new(), false)
                } else {
                    // Field: the level-0 policy on its true hand, options logged.
                    let opts = host.pi0_evaluate(&key, seat, hand, legal)?;
                    let chosen = opts
                        .iter()
                        .cloned()
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
                        .expect("legal play")
                        .0;
                    (chosen, opts, false)
                };
            let tile = Domino::from_index(usize::from(chosen)).expect("tile");
            if pos == 0 {
                led = Some(dcl.led_context(tile));
            } else if !dcl.follows(tile, led.expect("led set")) {
                voids[seat_i] |= mask_of(dcl.effective_incidence(led.expect("led set")));
            }
            played |= 1u32 << chosen;
            plays.push(chosen);
            plays_json.push(format!(
                "{{\"seat\":{},\"tile\":{},\"walt\":{},\"opts\":{}}}",
                seat_i,
                tile_json(chosen),
                walt,
                opts_json(&opts)
            ));
        }
        let doms = [
            Domino::from_index(usize::from(plays[0])).expect("p0"),
            Domino::from_index(usize::from(plays[1])).expect("p1"),
            Domino::from_index(usize::from(plays[2])).expect("p2"),
            Domino::from_index(usize::from(plays[3])).expect("p3"),
        ];
        let trick = Trick::new(Seat::from_index(usize::from(leader)).expect("leader"), doms)
            .expect("distinct");
        let winner = trick.winner(dcl);
        let pts = trick.points() as u8;
        if winner.team() == Team::T1 {
            banked_t1 += pts;
        } else {
            banked_t0 += pts;
        }
        tricks_json.push(format!(
            "{{\"leader\":{},\"winner\":{},\"points\":{},\"banked\":[{},{}],\"plays\":[{}]}}",
            leader,
            winner.index(),
            pts,
            banked_t1,
            banked_t0,
            plays_json.join(",")
        ));
        leader = winner.index() as u8;
        trick_start_played = played;
    }
    assert_eq!(banked_t1 + banked_t0, 42, "all points banked");
    let made = banked_t1 >= 30;
    Some(format!(
        "{{\"game\":{},\"deal\":{{\"S0\":{},\"S1\":{},\"S2\":{},\"S3\":{}}},\"made\":{},\"banked\":[{},{}],\"tricks\":[{}]}}",
        game_idx,
        hand_json(hands[0]),
        hand_json(hands[1]),
        hand_json(hands[2]),
        hand_json(hands[3]),
        made,
        banked_t1,
        banked_t0,
        tricks_json.join(",")
    ))
}

fn main() {
    let dcl = decl();
    let args: Vec<String> = std::env::args().collect();
    let n_games: usize = args.get(1).map(|s| s.parse().expect("games")).unwrap_or(3);
    let budget_secs: u64 = args
        .get(2)
        .map(|s| s.parse().expect("budget seconds"))
        .unwrap_or(300);
    let n_outer: usize = args
        .get(3)
        .map(|s| s.parse().expect("outer deals"))
        .unwrap_or(200);
    let n0: usize = args.get(4).map(|s| s.parse().expect("n0")).unwrap_or(8);
    // "all1": every seat plays level 1 from its own chair (informed table).
    let all_level1 = args.get(5).map(|s| s == "all1").unwrap_or(false);
    let deadline = Instant::now() + std::time::Duration::from_secs(budget_secs);
    let mut games: Vec<String> = Vec::new();
    for g in 0..n_games {
        eprintln!("playing game {g}...");
        match play_game(dcl, g, n_outer, n0, all_level1, deadline) {
            Some(j) => games.push(j),
            None => {
                eprintln!("budget died mid-game {g}; emitting completed games only");
                break;
            }
        }
    }
    println!("[{}]", games.join(","));
}
