//! The objective hand machine (Exec §§6–13): deals, auction, contract,
//! certified contracted play, settlement, and primitive events.

pub mod auction;
pub mod contract;
pub mod deal;
pub mod events;
pub mod play;

/// Explicit objective-layer errors (Exec §24). Invalid inputs are never
/// coerced into nearby valid states.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectiveError {
    /// A deal violates the seven-per-seat partition invariants.
    InvalidDeal,
    /// An auction action is out of turn or not in the legal bid set.
    InvalidAuctionAction,
    /// A contract failed auction-replay certification.
    InvalidContract,
    /// A play is not in the exact legal play set.
    IllegalPlay,
    /// A lifecycle operation was invoked in the wrong phase.
    PhaseMismatch,
    /// A structural or conservation invariant failed.
    InvariantViolation,
}
