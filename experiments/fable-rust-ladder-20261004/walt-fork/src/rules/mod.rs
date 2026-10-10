//! walt-core -- the Straight 42 rules layer.
//!
//! Implements the base object of `walt/math/unified_information_geometry_v0.4.md`
//! §1: pips, the 28 dominoes, seats and teams, the nine Straight declarations,
//! effective contexts, the rule algebra (incidence, follow, tier, rank, trick
//! key, BEATS/THREAT), count and trick scoring, legality, and history replay.
//!
//! Imports nothing. Every derived view (effective incidence, legal set, void
//! set, remaining hands) is a function of the semantic state, never stored
//! beside it.

pub mod context;
pub mod decl;
pub mod domino;
pub mod pip;
pub mod receipt;
pub mod replay;
#[allow(clippy::module_inception)] // fold artifact: walt-core/src/rules.rs kept its name
pub mod rules;
pub mod seat;
pub mod set;

pub use context::Context;
pub use decl::{Decl, DeclClass};
pub use domino::Domino;
pub use pip::Pip;
pub use rules::{legal_plays, Rank, Tier, Trick, TrickKey, DOUBLES, NATURAL};
pub use seat::{Seat, Team};
pub use set::{ContextSet, DominoSet};

/// The count decoration sums to 35 over the universe (v0.4 §1.4).
pub const TOTAL_COUNT_POINTS: u32 = 35;
