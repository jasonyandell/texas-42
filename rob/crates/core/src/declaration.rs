//! The nine Straight 42 declarations.
//!
//! Implements the declaration domain `Δ_straight = P ∪ {DT, NT}`
//! (Math §3.1; Rules R-DECL-01).

use crate::pip::{Pip, PIPS};

/// One of the nine Straight 42 declarations (R-DECL-01; Math §3.1).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Declaration {
    /// Pip `p` is trump: every domino containing `p` is called and powered
    /// (R-SUIT-01).
    PipTrump(Pip),
    /// The seven doubles form the trump suit (R-SUIT-02).
    DoublesTrump,
    /// No-trump / follow-me: nothing is called, nothing is powered
    /// (R-SUIT-03).
    NoTrump,
}

/// The nine Straight declarations in canonical order: pip trumps 0..6, then
/// doubles-trump, then no-trump (Math §3.1).
pub const GAME_DECLARATIONS: [Declaration; 9] = [
    Declaration::PipTrump(PIPS[0]),
    Declaration::PipTrump(PIPS[1]),
    Declaration::PipTrump(PIPS[2]),
    Declaration::PipTrump(PIPS[3]),
    Declaration::PipTrump(PIPS[4]),
    Declaration::PipTrump(PIPS[5]),
    Declaration::PipTrump(PIPS[6]),
    Declaration::DoublesTrump,
    Declaration::NoTrump,
];
