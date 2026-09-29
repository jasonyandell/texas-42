//! EXPLORATORY PROBE — below every evidentiary tier, cited by nothing above it.
//!
//! Head-to-head: walt level-1 (the live `solver::level1_evaluate` path) versus
//! a flat Monte Carlo player with TRUE random rollouts and no tickertape:
//! sample worlds from the seat's void-conditioned fiber (the same
//! `sample_belief` walt uses), and for each legal tile play the rest of the
//! hand out with EVERY seat (own seat included) choosing uniformly among its
//! legal tiles, fresh randomness per rollout. Pick the tile with the best
//! make count (exact integer counts; no floats).
//!
//! Mirrored pairs at bid 30: same deal, same contract, partnerships swapped.
//!
//! Usage: rollout_h2h A B deals first_seed [worlds rollouts]
//!   players: walt | waltK (K dice tapes per level-0 world, e.g. walt16;
//!   `walt` = walt1 = the frozen tickertape) | mc | random
//!
//! Tickertape ablation (walt1 vs waltK): every non-forced walt decision is
//! also SHADOW-evaluated under the other K, so the output counts how often
//! the tape count changes the move, alongside the mirrored game result and
//! the wall time per decision at each K.

use walt::rules::rules::legal_plays;
use walt::rules::{Context, Decl, Domino, Pip, Seat, Team};
use walt::solver::{
    best_of, level1_evaluate, mask_bits, mask_of, mix, record_hash, sample_belief, set_of, Key,
    SplitMix64, FULL_MASK,
};

const BID: u8 = 30;
const WALT_SEED: u64 = 0x5851_F42D_4C95_7F2D;
const MC_SEED: u64 = 0x1405_7B7E_F767_814F;
const RANDOM_SEED: u64 = 0x2545_F491_4F6C_DD1D;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Player {
    Walt(usize),
    Mc,
    Random,
}

struct Cfg {
    worlds: usize,
    rollouts: usize,
    n_outer: usize,
    n0: usize,
    /// Player A's tape count (to split timing by K).
    ka: usize,
}

fn parse(p: &str) -> Player {
    match p {
        "walt" => Player::Walt(1),
        w if w.starts_with("walt") => Player::Walt(w[4..].parse().expect("walt<K>")),
        "mc" => Player::Mc,
        "random" => Player::Random,
        _ => panic!("player is walt | mc | random"),
    }
}

fn tile(id: u8) -> Domino {
    Domino::from_index(usize::from(id)).expect("tile id")
}

fn led_of(dcl: Decl, plays: &[u8]) -> Option<Context> {
    plays.first().map(|&t| dcl.led_context(tile(t)))
}

fn legal_mask(dcl: Decl, hand: u32, plays: &[u8]) -> u32 {
    mask_of(legal_plays(dcl, set_of(hand), led_of(dcl, plays)))
}

fn pick_uniform(mask: u32, rng: &mut SplitMix64) -> u8 {
    let bits = mask_bits(mask);
    bits[rng.below(bits.len() as u64) as usize]
}

/// One uniform-random playout from a full-information world. Returns made.
#[allow(clippy::too_many_arguments)]
fn rollout(
    dcl: Decl,
    world: &[u32; 4],
    mut played: u32,
    mut leader: u8,
    mut plays: Vec<u8>,
    mut t1: u8,
    mut t0: u8,
    rng: &mut SplitMix64,
) -> bool {
    let contract = walt::solver::Contract::Straight { bid: BID };
    loop {
        if t1 >= BID {
            return true;
        }
        if t0 > 42 - BID {
            return false;
        }
        if plays.len() == 4 {
            let w = contract.winner(dcl, usize::from(leader), &plays);
            let pts = 1 + plays.iter().map(|&t| tile(t).count() as u8).sum::<u8>();
            if w.team() == Team::T1 {
                t1 += pts;
            } else {
                t0 += pts;
            }
            leader = w.index() as u8;
            plays.clear();
            continue;
        }
        let seat = (usize::from(leader) + plays.len()) % 4;
        let hand = world[seat] & !played;
        let t = pick_uniform(legal_mask(dcl, hand, &plays), rng);
        played |= 1 << t;
        plays.push(t);
    }
}

struct Table {
    dcl: Decl,
    hands: [u32; 4],
    played: u32,
    leader: u8,
    plays: Vec<u8>,
    t1: u8,
    t0: u8,
    voids: [u32; 4],
    trick_start_played: u32,
    completed: usize,
}

impl Table {
    fn key(&self) -> Key {
        Key {
            voids: None,
            played: self.played,
            leader: self.leader,
            plays: self.plays.clone(),
            banked_t1: self.t1,
            banked_t0: self.t0,
            alive: 0,
        }
    }
    fn sizes(&self) -> [usize; 4] {
        let mut sz = [7 - self.completed; 4];
        for i in 0..self.plays.len() {
            sz[(usize::from(self.leader) + i) % 4] -= 1;
        }
        sz
    }
}

/// stats: [walt decisions, shadow disagreements, walt refusals,
///         ms at K of A, decisions at K of A, ms at K of B, decisions at K of B]
type Stats = [u64; 7];

fn walt_choice(tb: &Table, seat: usize, hand: u32, legal: u32, cfg: &Cfg, tapes: usize, stats: &mut Stats) -> (u8, u64) {
    let key = tb.key();
    let s = Seat::from_index(seat).expect("seat");
    let maximize = s.team() == Team::T1;
    let info_seed = mix(u64::from(tb.hands[seat])) ^ record_hash(&key);
    let mut rng = SplitMix64(WALT_SEED ^ info_seed);
    walt::solver::DICE_TAPES.store(tapes, std::sync::atomic::Ordering::Relaxed);
    let t0 = std::time::Instant::now();
    let r = level1_evaluate(
        tb.dcl,
        BID,
        s,
        hand,
        legal,
        &key,
        tb.sizes(),
        tb.voids,
        tb.trick_start_played,
        7 - tb.completed,
        cfg.n_outer,
        cfg.n0,
        60,
        &mut rng,
    );
    let ms = t0.elapsed().as_millis() as u64;
    let c = match r {
        Ok(opts) => best_of(&opts, maximize),
        Err(_) => {
            stats[2] += 1;
            legal.trailing_zeros() as u8
        }
    };
    (c, ms)
}

fn decide(p: Player, other: Player, tb: &Table, seat: usize, cfg: &Cfg, stats: &mut Stats) -> u8 {
    let hand = tb.hands[seat] & !tb.played;
    let legal = legal_mask(tb.dcl, hand, &tb.plays);
    if legal.count_ones() == 1 {
        return legal.trailing_zeros() as u8;
    }
    let key = tb.key();
    let s = Seat::from_index(seat).expect("seat");
    let maximize = s.team() == Team::T1;
    let info_seed = mix(u64::from(tb.hands[seat])) ^ record_hash(&key);
    match p {
        Player::Random => {
            let mut rng = SplitMix64(RANDOM_SEED ^ info_seed);
            pick_uniform(legal, &mut rng)
        }
        Player::Walt(k) => {
            let (c, ms) = walt_choice(tb, seat, hand, legal, cfg, k, stats);
            stats[0] += 1;
            let slot = if k == cfg.ka { 3 } else { 5 };
            stats[slot] += ms;
            stats[slot + 1] += 1;
            if let Player::Walt(j) = other {
                if j != k {
                    let (shadow, ms2) = walt_choice(tb, seat, hand, legal, cfg, j, stats);
                    let slot = if j == cfg.ka { 3 } else { 5 };
                    stats[slot] += ms2;
                    stats[slot + 1] += 1;
                    if shadow != c {
                        stats[1] += 1;
                    }
                }
            }
            c
        }
        Player::Mc => {
            let mut rng = SplitMix64(MC_SEED ^ info_seed);
            let worlds = sample_belief(
                seat,
                hand,
                tb.played,
                tb.sizes(),
                tb.voids,
                cfg.worlds,
                &mut rng,
            )
            .expect("true deal keeps the fiber nonempty");
            let seeds: Vec<u64> = (0..cfg.worlds * cfg.rollouts)
                .map(|_| rng.next_u64())
                .collect();
            let mut best: Option<(u8, usize)> = None;
            for t in mask_bits(legal) {
                let mut makes = 0usize;
                for (wi, w) in worlds.iter().enumerate() {
                    for r in 0..cfg.rollouts {
                        let mut rr = SplitMix64(seeds[wi * cfg.rollouts + r]);
                        let mut plays = tb.plays.clone();
                        plays.push(t);
                        if rollout(
                            tb.dcl,
                            w,
                            tb.played | (1 << t),
                            tb.leader,
                            plays,
                            tb.t1,
                            tb.t0,
                            &mut rr,
                        ) {
                            makes += 1;
                        }
                    }
                }
                let better = match best {
                    None => true,
                    Some((_, m)) => {
                        if maximize {
                            makes > m
                        } else {
                            makes < m
                        }
                    }
                };
                if better {
                    best = Some((t, makes));
                }
            }
            best.expect("legal nonempty").0
        }
    }
}

/// Plays one hand; `t1_player` sits in seats 1,3 (declaring), `t0_player` in 0,2.
fn play(dcl: Decl, hands: [u32; 4], t1p: Player, t0p: Player, cfg: &Cfg, stats: &mut Stats) -> bool {
    let contract = walt::solver::Contract::Straight { bid: BID };
    let mut tb = Table {
        dcl,
        hands,
        played: 0,
        leader: 1,
        plays: Vec::new(),
        t1: 0,
        t0: 0,
        voids: [0; 4],
        trick_start_played: 0,
        completed: 0,
    };
    loop {
        if tb.t1 >= BID {
            return true;
        }
        if tb.t0 > 42 - BID {
            return false;
        }
        let seat = (usize::from(tb.leader) + tb.plays.len()) % 4;
        let (p, o) = if seat % 2 == 1 { (t1p, t0p) } else { (t0p, t1p) };
        let t = decide(p, o, &tb, seat, cfg, stats);
        assert!(legal_mask(dcl, tb.hands[seat] & !tb.played, &tb.plays) & (1 << t) != 0);
        if let Some(led) = led_of(dcl, &tb.plays) {
            if !dcl.follows(tile(t), led) {
                tb.voids[seat] |= mask_of(dcl.effective_incidence(led));
            }
        }
        tb.played |= 1 << t;
        tb.plays.push(t);
        if tb.plays.len() == 4 {
            let w = contract.winner(dcl, usize::from(tb.leader), &tb.plays);
            let pts = 1 + tb.plays.iter().map(|&x| tile(x).count() as u8).sum::<u8>();
            if w.team() == Team::T1 {
                tb.t1 += pts;
            } else {
                tb.t0 += pts;
            }
            tb.leader = w.index() as u8;
            tb.plays.clear();
            tb.completed += 1;
            tb.trick_start_played = tb.played;
        }
    }
}

/// Deal and choose the contract: the seat/pip-trump with the most trumps,
/// then holding the trump double, then most doubles (the battery's forced
/// bidding heuristic). Returns internal hands with the bidder at seat 1.
fn deal(seed: u64) -> (Decl, [u32; 4], u8) {
    let mut rng = SplitMix64(mix(seed ^ 0x9E37_79B9_7F4A_7C15));
    let mut tiles = mask_bits(FULL_MASK);
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mut phys = [0u32; 4];
    for (i, &t) in tiles.iter().enumerate() {
        phys[i / 7] |= 1 << t;
    }
    let mut best = (0usize, 0u8, (0u32, 0u32, 0u32));
    for (s, &h) in phys.iter().enumerate() {
        for p in 0..7u8 {
            let mut trumps = 0;
            let mut dbl = 0;
            let mut doubles = 0;
            for t in mask_bits(h) {
                let d = tile(t);
                let (hi, lo) = (d.hi().value(), d.lo().value());
                if hi == p || lo == p {
                    trumps += 1;
                }
                if hi == lo {
                    doubles += 1;
                    if hi == p {
                        dbl = 1;
                    }
                }
            }
            let score = (trumps, dbl, doubles);
            if score > best.2 {
                best = (s, p, score);
            }
        }
    }
    let (bidder, pip, _) = best;
    let mut hands = [0u32; 4];
    for (i, h) in hands.iter_mut().enumerate() {
        *h = phys[(bidder + i + 3) % 4];
    }
    (Decl::PipTrump(Pip::new(pip).expect("pip")), hands, pip)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let pa = parse(&a[1]);
    let pb = parse(&a[2]);
    let deals: u64 = a[3].parse().expect("deals");
    let first: u64 = a[4].parse().expect("first seed");
    let cfg = Cfg {
        worlds: a.get(5).map_or(100, |x| x.parse().expect("worlds")),
        rollouts: a.get(6).map_or(20, |x| x.parse().expect("rollouts")),
        n_outer: 50,
        n0: 8,
        ka: match pa {
            Player::Walt(k) => k,
            _ => 0,
        },
    };
    let mut stats: Stats = [0; 7];
    let (mut aw, mut bw, mut ties) = (0, 0, 0);
    let (mut a_decl_made, mut b_decl_made) = (0, 0);
    for seed in first..first + deals {
        let (dcl, hands, pip) = deal(seed);
        let t0 = std::time::Instant::now();
        let a_made = play(dcl, hands, pa, pb, &cfg, &mut stats); // A declares
        let b_made = play(dcl, hands, pb, pa, &cfg, &mut stats); // B declares
        a_decl_made += u32::from(a_made);
        b_decl_made += u32::from(b_made);
        let d = i32::from(a_made) - i32::from(b_made);
        match d {
            1 => aw += 1,
            -1 => bw += 1,
            _ => ties += 1,
        }
        println!(
            "seed {seed} trump {pip}: A-declares made={a_made} B-declares made={b_made} pair={d:+} ({} ms)",
            t0.elapsed().as_millis()
        );
    }
    println!(
        "TOTAL {} vs {} over {deals} pairs: A wins {aw} / B wins {bw} / ties {ties}; \
         declaring makes A {a_decl_made}/{deals}, B {b_decl_made}/{deals}; walt refusals {}",
        a[1], a[2], stats[2]
    );
    println!(
        "walt decisions {} (nonforced), shadow disagreements {}; ms/decision A-K {} B-K {}",
        stats[0],
        stats[1],
        stats[3] / stats[4].max(1),
        stats[5] / stats[6].max(1)
    );
}
