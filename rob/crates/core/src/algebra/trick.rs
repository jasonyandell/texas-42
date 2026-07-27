//! Trick resolution.
//!
//! Implements the unique-winner theorem surface (Math §3.6; ALG-12) as the
//! actor-preserving `resolve_trick` contract of Exec §5.

use crate::domino::DominoId;
use crate::seat::Seat;

/// One actor-attributed play (Exec §10 `Play`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Play {
    /// The acting seat.
    pub actor: Seat,
    /// The played domino.
    pub domino: DominoId,
}

/// Result of resolving a completed trick (Exec §5 `TrickResult`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrickResult {
    /// The winning actor (actor-preserving; ALG-12).
    pub winner: Seat,
    /// Trick award: `1 + sum(countPoints(d))` over the four plays
    /// (R-SCORE-03).
    pub points: u8,
}

/// Explicit trick-resolution errors (Exec §5, §24). Invalid inputs are never
/// coerced into nearby valid states.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrickError {
    /// The trick does not contain exactly four plays.
    WrongLength,
    /// Two plays use the same physical domino.
    DuplicateDomino,
    /// Actors are not in clockwise order from the leader.
    MalformedActorSequence,
    /// No unique maximal trick key (impossible for distinct dominoes by the
    /// unique-winner theorem, Math §3.6; kept as an explicit error path).
    NoUniqueMaximum,
}
