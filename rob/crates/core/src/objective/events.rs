//! Primitive public events and private deal observations.
//!
//! Implements Exec §13 (`BasePublicEvent`, `PrivateDealObservation`).
//! Events are emitted as explicit constructor return data (Exec §10.1) and
//! are the single source of truth for derived public facts (R-INFO-02A;
//! INV-8 ONE-SOURCE-OF-TRUTH).

use crate::declaration::Declaration;
use crate::domino::{DominoId, DominoSet};
use crate::rules::AuctionAction;
use crate::seat::Seat;

/// One primitive public event (Exec §13 `BasePublicEvent`). Derived facts
/// (winner, score, settlement, …) are deterministic functions of the base
/// stream, never independent observations.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BasePublicEvent {
    /// Match creation with its target and first shaker.
    MatchStarted {
        /// Match target `T`.
        target: u32,
        /// The first shaker.
        initial_shaker: Seat,
    },
    /// A new deal attempt began.
    DealStarted {
        /// Index of the deal attempt.
        deal_attempt_index: u64,
        /// The shaker of this attempt.
        shaker: Seat,
    },
    /// One auction action with its actor.
    Bid {
        /// Index of the deal attempt.
        deal_attempt_index: u64,
        /// The acting seat.
        actor: Seat,
        /// Pass or a nonpass bid.
        action: AuctionAction,
    },
    /// The public declaration by the auction winner.
    Declaration {
        /// Index of the deal attempt.
        deal_attempt_index: u64,
        /// The declaring bidder.
        actor: Seat,
        /// The chosen declaration.
        declaration: Declaration,
    },
    /// One played domino with its actor.
    Play {
        /// Index of the deal attempt.
        deal_attempt_index: u64,
        /// The acting seat.
        actor: Seat,
        /// The played domino.
        domino: DominoId,
    },
}

/// One private deal observation: a player's own hand for one attempt
/// (Exec §13 `PrivateDealObservation`; R-INFO-01).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PrivateDealObservation {
    /// Index of the deal attempt.
    pub deal_attempt_index: u64,
    /// The observing seat.
    pub seat: Seat,
    /// The privately observed seven-domino hand.
    pub hand: DominoSet,
}
