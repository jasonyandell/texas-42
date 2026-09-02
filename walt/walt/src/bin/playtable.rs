//! EXPLORATORY INTERACTIVE TABLE — sits below every evidentiary tier and is
//! cited by nothing above it. A human pulls up a chair.
//!
//! One human seat (chosen at launch), three level-1 seats: each AI seat
//! samples a belief from its own chair (own hand, public record, observed
//! voids) and best-responds against level-0 models of the others — T1
//! maximizes P(make 30), T0 minimizes it, saturation ties re-examined on 4x
//! samples up to 16x. Nobody peeks: the human's hand is just another hidden
//! hand the AIs sample over, and the `hint` command runs the SAME honest
//! level-1 evaluation from the human's chair only.
//!
//! Contract is frozen to the receipt-hand-8 declaration: trump fives, P30
//! bid by T1, S1 leads trick 1 (bidding is a later mountain). Deals are
//! fresh each hand; by default S1 keeps the receipt hand, `fresh` re-deals
//! all four seats.
//!
//! CONTROLLER SEATING (CE thread): a `ctrl` argument seats the §16.4
//! evidence/decision-controller player (`solver::act`) at every AI seat;
//! `cap=N` sets its world cap (default 128 — an interactive think-time
//! budget; a low cap only produces more honest Unresolved → level-1
//! fallback decisions, never wrong settlements). Every controller play
//! prints WHICH ROUTE chose the tile; a level-1 fallback among
//! survivors/ties is an ordering choice outside the correctness boundary
//! and is never presented as a settled winner.
//!
//! RNG discipline (O27 fix, §12.3 audit): the deal stream and the belief
//! streams are domain-separated — the deal rng deals and does nothing
//! else; every AI/hint evaluation derives a per-decision stream from
//! (constant, session seed, own dealt hand, record hash) — the
//! walt_bridge information-consistent pattern, audited CLEAN.
//!
//! ESTIMATES, never receipts; not a P-A21 statement. No floats.

use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::solver::act::{act as controller_act, delta_run_default, ActConfig};
use walt::solver::adaptive::DrivenState;
// The void-conditioned belief sampler and its dependencies are the
// LIBRARY's one authority (`solver::sample_belief` — the σ1-repair slice
// deduplicated the five byte-identical copies onto it). Importing the
// sampler forces the RNG type with it: a local `SplitMix64` would be a
// distinct Rust type, so the seed paths in this binary run on the
// library's stream — the same algorithm, hash-identical, so no draw
// changes.
use walt::solver::{mask_bits, sample_belief, Level1Refusal, SplitMix64, FULL_MASK};

/// Frozen seed for level-0 inner sampling (MUST match level1.rs so the field
/// seats here play exactly the policy S1's solver models).
const INNER_SEED: u64 = 0x243F_6A88_85A3_08D3;

/// Frozen seed for the per-decision belief streams (O27: domain-separated
/// from the deal stream and from every other surface constant).
const TABLE_BELIEF_SEED: u64 = 0x9216_D5D9_8979_FB1B;

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

fn tile_str(idx: u8) -> String {
    let dm = Domino::from_index(usize::from(idx)).expect("tile < 28");
    format!("{}-{}", dm.hi().value(), dm.lo().value())
}

fn hand_str(mask: u32) -> String {
    mask_bits(mask)
        .into_iter()
        .map(tile_str)
        .collect::<Vec<_>>()
        .join("  ")
}

fn pct(v: &BigRational) -> String {
    let b = bp(v);
    format!("{}.{:02}%", b / 100, b % 100)
}

fn seat_name(s: usize, human: usize) -> String {
    let base = match s {
        0 => "S0",
        1 => "S1 walt",
        2 => "S2",
        _ => "S3",
    };
    if s == human {
        format!("{base} (YOU)")
    } else {
        base.to_string()
    }
}

fn read_line_lower() -> String {
    use std::io::Write;
    std::io::stdout().flush().ok();
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) {
        Ok(0) | Err(_) => "quit".to_string(),
        Ok(_) => s.trim().to_lowercase(),
    }
}

fn parse_tile(s: &str) -> Option<u8> {
    let ds: Vec<u8> = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u8 - b'0')
        .collect();
    if ds.len() != 2 || ds[0] > 6 || ds[1] > 6 {
        return None;
    }
    let (hi, lo) = if ds[0] >= ds[1] {
        (ds[0], ds[1])
    } else {
        (ds[1], ds[0])
    };
    Some(d(hi, lo).index() as u8)
}

/// The level-1 evaluation with saturation-tie refinement (identical policy
/// to the informed-table playouts). Returns every legal option's estimate,
/// or the typed reason there are none — a deadline, or the library
/// sampler's refusal of a frame with no lawful completion. STILL
/// TRIPLICATED with `solver::level1_evaluate` and walt_bridge.rs's copy: a
/// named debt, and the σ1-repair slice deliberately did not pay it.
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
) -> Result<Vec<(u8, BigRational)>, Level1Refusal> {
    let deadline = Instant::now() + std::time::Duration::from_secs(per_move_secs);
    let maximize = seat.team() == Team::T1;
    let evaluate = |tiles: &[u8], n: usize, rng: &mut SplitMix64| {
        let worlds = sample_belief(seat.index(), hand, key.played, sizes, voids, n, rng)?;
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

/// The u32 effective-incidence masks of the observed void contexts — a
/// derived view of the stored context sets (one authority, never two).
fn void_masks(dcl: Decl, voids: &[ContextSet; 4]) -> [u32; 4] {
    core::array::from_fn(|s| {
        voids[s]
            .iter()
            .fold(0u32, |acc, q| acc | mask_of(dcl.effective_incidence(q)))
    })
}

/// One interactive hand. Returns false if the human quit mid-hand.
#[allow(clippy::too_many_lines)]
#[allow(clippy::too_many_arguments)]
fn play_hand(
    dcl: Decl,
    human: usize,
    hand_no: u64,
    seed: u64,
    fresh: bool,
    n_outer: usize,
    n0: usize,
    per_move_secs: u64,
    ctrl: bool,
    world_cap: u64,
) -> bool {
    // The DEAL stream — O27: it deals and does nothing else.
    let mut deal_rng = SplitMix64(seed ^ mix(hand_no));
    let hands: [u32; 4] = if fresh {
        let mut tiles: Vec<u8> = (0..28).collect();
        for i in (1..tiles.len()).rev() {
            let j = deal_rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
        [
            mask_slice(&tiles[0..7]),
            mask_slice(&tiles[7..14]),
            mask_slice(&tiles[14..21]),
            mask_slice(&tiles[21..28]),
        ]
    } else {
        let s1_full = s1_initial_mask();
        let mut h = sample_belief(1, s1_full, 0, [7, 7, 7, 7], [0u32; 4], 1, &mut deal_rng)
            .expect("a void-free frame is feasible: every deal of the unseen pool is lawful")
            .pop()
            .expect("one deal");
        h[1] = s1_full;
        h
    };

    println!();
    println!("=== hand {hand_no}: trump FIVES, T1 (S1+S3) must make 30; S1 leads ===");
    println!(
        "you are {} on team {}",
        seat_name(human, human),
        if human == 1 || human == 3 {
            "T1 (making the bid)"
        } else {
            "T0 (setting the bid)"
        }
    );
    println!("your hand: {}", hand_str(hands[human]));
    println!("commands: a tile like 6-2 or 62, `hint`, `auto`, `quit`");

    let mut played: u32 = 0;
    let mut leader: u8 = 1;
    let mut banked_t1: u8 = 0;
    let mut banked_t0: u8 = 0;
    // Observed void CONTEXTS — the stored authority; incidence masks are
    // the derived view `void_masks`.
    let mut voids = [ContextSet::EMPTY; 4];
    let mut trick_start_played: u32 = 0;
    for completed in 0..7 {
        println!();
        println!(
            "--- trick {} (count: T1 {banked_t1} / T0 {banked_t0}) ---",
            completed + 1
        );
        let mut plays: Vec<u8> = Vec::new();
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
            let sizes = {
                let mut sz = [7 - completed; 4];
                for i in 0..plays.len() {
                    sz[(usize::from(leader) + i) % 4] -= 1;
                }
                sz
            };
            let chosen: u8 = if seat_i == human {
                let mut hint: Option<Vec<(u8, BigRational)>> = None;
                loop {
                    println!("your hand: {}", hand_str(hand));
                    println!("legal:     {}", hand_str(legal));
                    print!("your play> ");
                    let input = read_line_lower();
                    let done: Option<u8> = match input.as_str() {
                        "quit" => return false,
                        "hint" | "auto" => {
                            if hint.is_none() {
                                println!("  (thinking from your chair only — no peeking...)");
                                // O27: a per-decision belief stream (own
                                // dealt hand + record hash), never the
                                // deal stream.
                                let mut drng = SplitMix64(
                                    TABLE_BELIEF_SEED
                                        ^ mix(seed)
                                        ^ mix(u64::from(hands[seat_i]))
                                        ^ record_hash(&key),
                                );
                                hint = match level1_evaluate(
                                    dcl,
                                    seat,
                                    hand,
                                    legal,
                                    &key,
                                    sizes,
                                    void_masks(dcl, &voids),
                                    trick_start_played,
                                    7 - completed,
                                    n_outer,
                                    n0,
                                    per_move_secs,
                                    &mut drng,
                                ) {
                                    Ok(opts) => Some(opts),
                                    Err(refusal) => {
                                        println!("  (hint refused: {refusal})");
                                        None
                                    }
                                };
                            }
                            match &hint {
                                Some(opts) => {
                                    let maxing = seat.team() == Team::T1;
                                    println!(
                                        "  best first (your team wants P(make) {}):",
                                        if maxing { "HIGH" } else { "LOW" }
                                    );
                                    let mut sorted = opts.clone();
                                    sorted.sort_by(|a, b| {
                                        if maxing {
                                            b.1.cmp(&a.1)
                                        } else {
                                            a.1.cmp(&b.1)
                                        }
                                    });
                                    for (t, v) in &sorted {
                                        println!("    {}  P(make) ~ {}", tile_str(*t), pct(v));
                                    }
                                    if input == "auto" {
                                        let c = best_of(opts, seat.team() == Team::T1);
                                        println!("  auto-playing {}", tile_str(c));
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                None => {
                                    println!("  (hint timed out; play on)");
                                    None
                                }
                            }
                        }
                        other => match parse_tile(other) {
                            Some(t) if legal & (1u32 << t) != 0 => Some(t),
                            Some(t) if hand & (1u32 << t) != 0 => {
                                println!(
                                    "  {} is in your hand but not legal here (must follow)",
                                    tile_str(t)
                                );
                                None
                            }
                            Some(_) => {
                                println!("  that tile is not in your hand");
                                None
                            }
                            None => {
                                println!(
                                    "  didn't catch that — tile like 6-2, or `hint`/`auto`/`quit`"
                                );
                                None
                            }
                        },
                    };
                    if let Some(c) = done {
                        break c;
                    }
                }
            } else if legal.count_ones() == 1 {
                let c = legal.trailing_zeros() as u8;
                println!(
                    "{} plays {} (forced)",
                    seat_name(seat_i, human),
                    tile_str(c)
                );
                c
            } else if ctrl {
                // The §16.4 controller player (CE thread): settled winner
                // played; honest tie / δ-survivors ranked by the live
                // level-1 ordering — the printed route says which.
                let t0 = Instant::now();
                let trick_tiles: Vec<Domino> = plays
                    .iter()
                    .map(|&t| Domino::from_index(usize::from(t)).expect("tile"))
                    .collect();
                let mut banked = [0u32; 2];
                banked[Team::T1.index()] = u32::from(banked_t1);
                banked[Team::T0.index()] = u32::from(banked_t0);
                let state = DrivenState {
                    decl: dcl,
                    bid: 30,
                    declaring_team: Team::T1,
                    viewer_hand: set_of(hand),
                    leader: Seat::from_index(usize::from(leader)).expect("leader"),
                    trick_plays: &trick_tiles,
                    banked,
                    prior_played: set_of(trick_start_played),
                    voids,
                };
                let d = u64::from(played.count_ones()) + 1;
                let cfg = ActConfig {
                    world_cap,
                    fallback_n_outer: n_outer,
                    fallback_n0: n0,
                    ..ActConfig::interactive()
                };
                let decision = controller_act(
                    &state,
                    &cfg,
                    &format!("run:playtable-h{hand_no}"),
                    d,
                    &delta_run_default(),
                );
                let ms = t0.elapsed().as_millis();
                let among: Vec<String> = decision
                    .among
                    .iter()
                    .map(|t| tile_str(t.index() as u8))
                    .collect();
                let detail = decision.evaluation.as_ref().map_or_else(
                    || "forced".to_string(),
                    |e| {
                        if let walt::solver::controller::SetResult::ExactFrozenSet {
                            fiber, ..
                        } = &e.result
                        {
                            format!(
                                "{} {}, fiber {fiber}",
                                decision.controller_route,
                                e.result.tag()
                            )
                        } else {
                            format!(
                                "{} {}, {} worlds",
                                decision.controller_route,
                                e.result.tag(),
                                e.consumed
                            )
                        }
                    },
                );
                println!(
                    "{} [controller] plays {}  ({}.{:01}s)  route {} among [{}] ({detail})",
                    seat_name(seat_i, human),
                    tile_str(decision.tile.index() as u8),
                    ms / 1000,
                    (ms % 1000) / 100,
                    decision.route.label(),
                    among.join(" "),
                );
                decision.tile.index() as u8
            } else {
                let t0 = Instant::now();
                // O27: a per-decision belief stream, never the deal stream.
                let mut drng = SplitMix64(
                    TABLE_BELIEF_SEED
                        ^ mix(seed)
                        ^ mix(u64::from(hands[seat_i]))
                        ^ record_hash(&key),
                );
                let opts = level1_evaluate(
                    dcl,
                    seat,
                    hand,
                    legal,
                    &key,
                    sizes,
                    void_masks(dcl, &voids),
                    trick_start_played,
                    7 - completed,
                    n_outer,
                    n0,
                    per_move_secs,
                    &mut drng,
                );
                let c = match &opts {
                    Ok(o) => best_of(o, seat.team() == Team::T1),
                    Err(refusal) => {
                        println!("  (evaluation refused: {refusal}; playing lowest legal)");
                        legal.trailing_zeros() as u8
                    }
                };
                let ms = t0.elapsed().as_millis();
                println!(
                    "{} plays {}  ({}.{:01}s)",
                    seat_name(seat_i, human),
                    tile_str(c),
                    ms / 1000,
                    (ms % 1000) / 100
                );
                c
            };
            let tile = Domino::from_index(usize::from(chosen)).expect("tile");
            if pos == 0 {
                led = Some(dcl.led_context(tile));
            } else if !dcl.follows(tile, led.expect("led set")) {
                voids[seat_i].insert(led.expect("led set"));
            }
            played |= 1u32 << chosen;
            plays.push(chosen);
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
        println!(
            "  >> {} takes the trick (+{pts})  |  count: T1 {banked_t1} / T0 {banked_t0}",
            seat_name(winner.index(), human)
        );
        leader = winner.index() as u8;
        trick_start_played = played;
    }
    assert_eq!(banked_t1 + banked_t0, 42, "all points banked");
    println!();
    if banked_t1 >= 30 {
        println!("=== T1 MAKES the 30 bid: {banked_t1} to {banked_t0} ===");
    } else {
        println!("=== T1 is SET: only {banked_t1} of 30 (T0 took {banked_t0}) ===");
    }
    println!("the deal was:");
    for s in [1usize, 3, 0, 2] {
        println!("  {}: {}", seat_name(s, human), hand_str(hands[s]));
    }
    true
}

fn main() {
    let dcl = decl();
    let raw: Vec<String> = std::env::args().collect();
    // Flags may sit anywhere: `fresh` re-deals all four seats, `ctrl`
    // seats the §16.4 controller player at the AI seats (CE thread),
    // `cap=N` sets its world cap (default 128).
    let fresh = raw.iter().any(|a| a == "fresh");
    let ctrl = raw.iter().any(|a| a == "ctrl");
    let world_cap: u64 = raw
        .iter()
        .find_map(|a| a.strip_prefix("cap="))
        .map(|v| v.parse().expect("cap=<worlds>"))
        .unwrap_or(128);
    let args: Vec<&String> = raw
        .iter()
        .filter(|a| *a != "fresh" && *a != "ctrl" && !a.starts_with("cap="))
        .collect();
    let human: usize = args
        .get(1)
        .map(|s| s.parse().expect("seat 0..=3"))
        .unwrap_or(0);
    assert!(human < 4, "seat must be 0..=3");
    let n_outer: usize = args
        .get(2)
        .map(|s| s.parse().expect("belief sample size"))
        .unwrap_or(100);
    let n0: usize = args.get(3).map(|s| s.parse().expect("n0")).unwrap_or(8);
    let seed: u64 = args.get(4).map(|s| s.parse().expect("seed")).unwrap_or(42);
    let per_move_secs: u64 = 180;

    println!("walt table — EXPLORATORY interactive 42 (trump fives, P30 by T1, S1 leads)");
    println!("three level-1 seats + you; nobody peeks; `hint` = honest eval from your chair");
    println!("estimates only, never receipts; not a P-A21 statement");
    if ctrl {
        println!(
            "controller seats ON (CE thread): world_cap={world_cap} — a think-time budget; \
             low caps only yield more honest Unresolved -> level-1 fallbacks, never wrong \
             settlements. Routes print per play."
        );
    }
    let mut hand_no: u64 = 0;
    loop {
        if !play_hand(
            dcl,
            human,
            hand_no,
            seed,
            fresh,
            n_outer,
            n0,
            per_move_secs,
            ctrl,
            world_cap,
        ) {
            println!("(left the table mid-hand)");
            break;
        }
        print!("another hand? (y/n)> ");
        if read_line_lower() != "y" {
            break;
        }
        hand_no += 1;
    }
    println!("thanks for playing. walt tips its hat.");
}
