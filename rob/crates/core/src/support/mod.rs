//! Exact hidden-information support: capacity cells, fibers, typed updates
//! (Math §§6–7; Exec §§14–17).

pub mod cells;

/// Explicit support-layer errors (Exec §24).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SupportError {
    /// A cell system violates its structural invariants.
    InvariantViolation,
    /// An observed play is impossible against the current support
    /// (Exec §17: an empty typed preimage is an impossible observation).
    ImpossibleObservation,
    /// A support operation was invoked in the wrong phase.
    PhaseMismatch,
    /// A viewer play is illegal against the known viewer hand.
    IllegalPlay,
    /// A cell system required to be feasible is not (Exec §15
    /// `InfeasibleCellSystem`).
    InfeasibleCellSystem,
}
