//! The S5 spine's typed conflict vocabulary (PLAN.md, adopted 2026-08-10).
//!
//! A conflict is a refuted line: a regretted decision under *declared*
//! semantics, or an S4 checker failure. This module is the one vocabulary:
//! the walker's regret conflicts live beside the existing checker witnesses
//! (`PurityCounterexample`, `LumpabilityFailure`), unchanged, under a single
//! sum type. A checker witness is a refutation; positive PI-averaged regret
//! is evidence under a declared relaxation — the `Grade` type keeps the two
//! kinds distinguishable and makes quoting a verdict without its operator
//! label impossible: every variant carries the label as a field.

use core::fmt;

use walt_core::{Domino, Seat};
use walt_geom::Q;
use walt_skeleton::{LumpabilityFailure, PurityCounterexample};
use walt_strat::{OperatorLabel, WeightingLabel};

/// How strong a conflict's verdict is, and under which declared knobs. The
/// grade is a *pair* — (dominance-vs-expectation) x (operator label) — never
/// a bare strength: worldwise dominance under one operator does not imply
/// dominance under another, because §7.6's policy-gluing gaps are
/// action-specific.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Grade {
    /// The relation holds in every world of the exhaustively enumerated
    /// fiber. Weighting-free, operator-labeled.
    WorldwiseDominance { operator: OperatorLabel },
    /// An exact expectation over the exhaustively enumerated fiber under a
    /// declared weighting.
    ExactExpectation {
        operator: OperatorLabel,
        weighting: WeightingLabel,
    },
    /// An expectation over a recorded uniform sample of the fiber. Evidence
    /// only, always marked; never quotable as either grade above.
    Sampled {
        operator: OperatorLabel,
        weighting: WeightingLabel,
        seed: u64,
        draws: u32,
    },
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grade::WorldwiseDominance { operator } => {
                write!(f, "worldwise-dominance ({operator})")
            }
            Grade::ExactExpectation {
                operator,
                weighting,
            } => write!(f, "exact-expectation ({operator}, {weighting})"),
            Grade::Sampled {
                operator,
                weighting,
                seed,
                draws,
            } => write!(
                f,
                "sampled ({operator}, {weighting}, seed {seed:#018x}, draws {draws})"
            ),
        }
    }
}

/// A regretted decision: the walker found the transcript's chosen action
/// worth strictly less than the best legal alternative, under the grade's
/// declared semantics.
#[derive(Clone, Debug)]
pub struct RegretConflict {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    /// The seat's position in the trick: 0 led, 1..=3 followed.
    pub ply: usize,
    pub chosen: Domino,
    /// The legal actions with strictly higher graded value than `chosen`.
    pub better: Vec<Domino>,
    /// Best graded value minus the chosen action's, in valuation units.
    pub regret: Q,
    pub grade: Grade,
    /// Exact fiber size at the decision (the counting DP's, even when the
    /// evaluation sampled).
    pub fiber: u128,
}

/// The one conflict vocabulary of the S5 spine. `St, Z, Y` are the
/// descriptor-state and target types of the S4 checkers, unchanged.
#[derive(Clone, Debug)]
pub enum Conflict<St, Z, Y> {
    /// A regretted transcript decision (this module).
    Regret(RegretConflict),
    /// A §12.1 factorization failure witness (walt-skeleton, S4).
    Purity(PurityCounterexample<Z, Y>),
    /// A §12.6 lumpability failure witness (walt-skeleton, S4).
    Lumpability(LumpabilityFailure<St>),
}
