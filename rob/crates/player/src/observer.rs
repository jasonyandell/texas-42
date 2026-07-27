//! Passive match observation for trace emission.
//!
//! An observer receives read-only context at each lifecycle moment of a
//! self-play match. It never influences play: the driver's RNG usage,
//! decision nonces, and transcript are identical with or without one (the
//! frozen self-play receipt stays byte-identical).

use rob_core::{
    AuctionAction, DealWorld, Declaration, HandResult, MechanicalState,
    ReachableContractedPlayState, Seat, TrickResult,
};

use crate::player::DecisionReport;

/// Context at contracted-hand start (after the placeholder auction and
/// declaration).
pub struct HandStartContext<'a> {
    /// Deal-attempt index.
    pub attempt_index: u64,
    /// The shaker of this attempt.
    pub shaker: Seat,
    /// The winning bidder.
    pub bidder: Seat,
    /// The declared interpretation.
    pub declaration: Declaration,
    /// The complete actor-attributed auction actions.
    pub auction_actions: &'a [(Seat, AuctionAction)],
    /// The omniscient dealt world (truth; viewers never see this).
    pub deal: &'a DealWorld,
}

/// Context around one root decision (states are BEFORE the play unless
/// named otherwise).
pub struct DecisionContext<'a> {
    /// Deal-attempt index.
    pub attempt_index: u64,
    /// Play index within the hand, `0..28`.
    pub play_index: u64,
    /// The acting seat.
    pub actor: Seat,
    /// The actor's decision evidence (legal set, totals, worlds, choice).
    pub report: &'a DecisionReport,
    /// All four per-seat viewer states before the play (each seat's exact
    /// private information — the source for perspective masking).
    pub viewers: &'a [MechanicalState; 4],
    /// The omniscient objective state before the play.
    pub objective_before: &'a ReachableContractedPlayState,
    /// The omniscient objective state after the play.
    pub objective_after: &'a ReachableContractedPlayState,
    /// The exact trick result when this play completed a trick.
    pub trick_result: Option<TrickResult>,
}

/// Context at hand settlement.
pub struct HandEndContext<'a> {
    /// Deal-attempt index.
    pub attempt_index: u64,
    /// The settled result.
    pub result: &'a HandResult,
    /// Marks after settlement.
    pub marks_after: [u32; 2],
    /// The declared interpretation.
    pub declaration: Declaration,
    /// The winning bidder.
    pub bidder: Seat,
}

/// A passive observer of one self-play match. All methods default to
/// no-ops; `()` is the null observer.
pub trait MatchObserver {
    /// A contracted hand is starting.
    fn on_hand_start(&mut self, _context: &HandStartContext<'_>) {}
    /// One root decision was made and applied.
    fn on_decision(&mut self, _context: &DecisionContext<'_>) {}
    /// A hand settled.
    fn on_hand_end(&mut self, _context: &HandEndContext<'_>) {}
    /// The match completed.
    fn on_match_end(&mut self, _final_marks: [u32; 2], _hands: u32) {}
}

impl MatchObserver for () {}
