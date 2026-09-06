//! Bounded native evaluator for fixed, information-consistent seat-level fields.
//!
//! The focal seat evaluates every legal root action on one common set of
//! sampled worlds. `PartnerOnly` raises only the seat across the table to
//! level 1; both opponents remain level 0. Modeled level-1 minds continue to
//! best respond to the uniform level-0 field through `Solver::pi`.

use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};

use num_rational::BigRational;

use crate::rules::{Decl, Domino, Seat, Team};
use crate::solver::{
    belief_frame_feasibility, best_of, mask_bits, Deadline, Field, InfeasibleFrame, Key, Shared,
    Solver, SplitMix64, FULL_MASK,
};

/// Which fixed field the focal best response faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldProfile {
    /// Every modeled seat uses level 0.
    Baseline,
    /// Only the focal seat's partner uses level 1; both opponents use level 0.
    PartnerOnly,
    /// Every modeled seat uses level 1.
    AllLevel1,
}

impl FieldProfile {
    /// Materialize the per-seat levels. The focal entry is zero but is never
    /// consulted by `Solver`, which handles the focal seat directly.
    #[must_use]
    pub fn seat_levels(self, focal: Seat) -> [usize; 4] {
        let mut levels = match self {
            FieldProfile::Baseline => [0; 4],
            FieldProfile::PartnerOnly => [0; 4],
            FieldProfile::AllLevel1 => [1; 4],
        };
        if self == FieldProfile::PartnerOnly {
            levels[focal.plus(2).index()] = 1;
        }
        levels[focal.index()] = 0;
        levels
    }
}

/// Fixed work schedule for one root decision.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Independent of seat levels; applies to every modeled mind.
    pub inner_belief: super::InnerBelief,
    pub profile: FieldProfile,
    pub n_outer: usize,
    /// Belief worlds used by each level-1 modeled-policy cache miss.
    pub n1: usize,
    /// Belief worlds used by each level-0 modeled-policy cache miss.
    pub n0: usize,
    /// Fully mixed outer stream seed. The caller derives this as
    /// `phone_seed ^ mix(original_hand) ^ record_hash(key)`.
    pub seed: u64,
    pub deadline: Deadline,
}

/// One completed root-action comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionValue {
    pub tile: Domino,
    pub value: BigRational,
}

/// Actual work performed by an evaluation. Inner worlds count only cache
/// misses that were really materialized, separated by modeled level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkStats {
    pub outer_worlds: u64,
    pub outer_draw_attempts: u64,
    pub pi_calls_by_level: Vec<u64>,
    pub inner_worlds_by_level: Vec<u64>,
    pub nodes: u64,
    pub elapsed: Duration,
}

/// A complete comparison. No partial action vector is returned on timeout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evaluation {
    pub profile: FieldProfile,
    pub actions: Vec<ActionValue>,
    pub stats: WorkStats,
    choice: u8,
}

impl Evaluation {
    /// Deterministic argmax/argmin with the solver's ascending-tile tie rule.
    #[must_use]
    pub fn best(&self) -> u8 {
        self.choice
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefusalReason {
    Deadline,
    InfeasibleFrame(InfeasibleFrame),
}

/// Typed failure plus work completed before the failure. Action comparisons
/// remain absent because an incomplete comparison must never select a move.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub reason: RefusalReason,
    pub stats: WorkStats,
}

struct SampleFailure {
    reason: RefusalReason,
    attempts: u64,
    accepted: u64,
}

fn stats(
    sh: Option<&Shared>,
    cfg: &Config,
    outer_worlds: u64,
    outer_draw_attempts: u64,
    start: Instant,
) -> WorkStats {
    let pi_calls_by_level = sh.map_or_else(Vec::new, Shared::pi_calls_by_level);
    let inner_worlds_by_level = pi_calls_by_level
        .iter()
        .enumerate()
        .map(|(level, calls)| {
            let sample_count = if level == 0 { cfg.n0 } else { cfg.n1 };
            calls.saturating_mul(sample_count as u64)
        })
        .collect();
    WorkStats {
        outer_worlds,
        outer_draw_attempts,
        pi_calls_by_level,
        inner_worlds_by_level,
        nodes: sh.map_or(0, |s| s.nodes.load(Ordering::Relaxed)),
        elapsed: start.elapsed(),
    }
}

/// Candidate-private form of the established uniform shuffle-and-reject
/// sampler. It preserves that sampler's stream exactly while adding a first
/// and every-256-attempt deadline check.
#[allow(clippy::too_many_arguments)]
fn sample_belief_bounded(
    viewer: usize,
    viewer_hand: u32,
    played: u32,
    sizes: [usize; 4],
    voids: [u32; 4],
    n: usize,
    rng: &mut SplitMix64,
    deadline: Deadline,
) -> Result<(Vec<[u32; 4]>, u64), SampleFailure> {
    if deadline.passed() {
        return Err(SampleFailure {
            reason: RefusalReason::Deadline,
            attempts: 0,
            accepted: 0,
        });
    }
    belief_frame_feasibility(viewer, viewer_hand, played, sizes, voids).map_err(|frame| {
        SampleFailure {
            reason: RefusalReason::InfeasibleFrame(frame),
            attempts: 0,
            accepted: 0,
        }
    })?;
    if deadline.passed() {
        return Err(SampleFailure {
            reason: RefusalReason::Deadline,
            attempts: 0,
            accepted: 0,
        });
    }
    let unseen = FULL_MASK & !played & !viewer_hand;
    let mut tiles = mask_bits(unseen);
    let others: Vec<usize> = (0..4).filter(|&s| s != viewer).collect();
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    let mut out = Vec::with_capacity(n);
    let mut attempts = 0u64;
    while out.len() < n {
        attempts += 1;
        if (attempts == 1 || attempts & 0xFF == 0) && deadline.passed() {
            return Err(SampleFailure {
                reason: RefusalReason::Deadline,
                attempts,
                accepted: out.len() as u64,
            });
        }
        for i in (1..tiles.len()).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            tiles.swap(i, j);
        }
        let mut world = [0u32; 4];
        world[viewer] = viewer_hand;
        let mut offset = 0;
        let mut accepted = true;
        for &seat in &others {
            world[seat] = mask_slice(&tiles[offset..offset + sizes[seat]]);
            offset += sizes[seat];
            if world[seat] & voids[seat] != 0 {
                accepted = false;
                break;
            }
        }
        if accepted {
            out.push(world);
        }
    }
    if deadline.passed() {
        return Err(SampleFailure {
            reason: RefusalReason::Deadline,
            attempts,
            accepted: out.len() as u64,
        });
    }
    Ok((out, attempts))
}

/// Evaluate every legal focal action on fixed common random worlds.
///
/// `cfg.seed` is the complete common-random-world stream seed. Callers use
/// `phone_seed ^ mix(original_hand) ^ record_hash(key)` so the evaluator
/// cannot accidentally substitute the current remaining hand.
#[allow(clippy::too_many_arguments)]
pub fn evaluate(
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
    cfg: &Config,
) -> Result<Evaluation, Refusal> {
    assert!(
        cfg.n_outer > 0 && cfg.n1 > 0 && cfg.n0 > 0,
        "sample counts are positive"
    );
    assert_eq!(hand & key.played, 0, "remaining hand is unplayed");
    assert_eq!(
        legal & !hand,
        0,
        "every declared action is in the focal hand"
    );
    assert_ne!(legal, 0, "the focal seat has a legal action");

    let start = Instant::now();
    let deadline = cfg.deadline;
    let mut rng = SplitMix64(cfg.seed);
    let (worlds, attempts) = match sample_belief_bounded(
        seat.index(),
        hand,
        key.played,
        sizes,
        voids,
        cfg.n_outer,
        &mut rng,
        deadline,
    ) {
        Ok(sampled) => sampled,
        Err(failure) => {
            return Err(Refusal {
                reason: failure.reason,
                stats: stats(None, cfg, failure.accepted, failure.attempts, start),
            });
        }
    };
    let sh = Arc::new(
        Shared::new(
            dcl,
            bid,
            vec![cfg.n0, cfg.n1],
            trick_start_played,
            boundary_hand_size,
            deadline,
        )
        .with_inner_belief(cfg.inner_belief),
    );
    let mut root = key.clone();
    root.voids = cfg.inner_belief.root_voids(voids);
    let solver = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        seat.team() == Team::T1,
        worlds,
        Vec::new(),
        Field::SeatLevels(cfg.profile.seat_levels(seat)),
    )
    .parallel();
    let mut actions = Vec::with_capacity(legal.count_ones() as usize);
    for tile_index in mask_bits(legal) {
        if deadline.passed() {
            sh.dead.store(true, Ordering::Relaxed);
            solver.flush_nodes();
            return Err(Refusal {
                reason: RefusalReason::Deadline,
                stats: stats(Some(&sh), cfg, cfg.n_outer as u64, attempts, start),
            });
        }
        let tile = Domino::from_index(usize::from(tile_index)).expect("tile < 28");
        let child = solver.child_after_play(&root, tile, 0);
        let Some(value) = solver.solve(&child) else {
            solver.flush_nodes();
            return Err(Refusal {
                reason: RefusalReason::Deadline,
                stats: stats(Some(&sh), cfg, cfg.n_outer as u64, attempts, start),
            });
        };
        if deadline.passed() {
            sh.dead.store(true, Ordering::Relaxed);
            solver.flush_nodes();
            return Err(Refusal {
                reason: RefusalReason::Deadline,
                stats: stats(Some(&sh), cfg, cfg.n_outer as u64, attempts, start),
            });
        }
        actions.push(ActionValue { tile, value });
    }
    solver.flush_nodes();
    let opts: Vec<(u8, BigRational)> = actions
        .iter()
        .map(|a| (a.tile.index() as u8, a.value.clone()))
        .collect();
    let choice = best_of(&opts, seat.team() == Team::T1);
    Ok(Evaluation {
        profile: cfg.profile,
        actions,
        stats: stats(Some(&sh), cfg, cfg.n_outer as u64, attempts, start),
        choice,
    })
}
