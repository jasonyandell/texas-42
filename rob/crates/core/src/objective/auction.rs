//! The one-round auction machine and its exhaustive tree.
//!
//! Implements Exec §7 (`AuctionState`, `AuctionWin`, legal bids) and the
//! census surface of R-AUC-12 / Math §4.3.

use crate::objective::deal::DealWorld;
use crate::objective::ObjectiveError;
use crate::rules::{AuctionAction, BidValue, MarkAmount, PointAmount, RulesConfig};
use crate::seat::Seat;

/// The auction state: shaker plus at most four actor-attributed actions in
/// clockwise prefix order (Exec §7).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AuctionState {
    shaker: Seat,
    actions: Vec<(Seat, AuctionAction)>,
}

/// A certified auction result: the last nonpass bidder and bid, with the
/// completed auction retained for validation (Exec §7 `AuctionWin`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AuctionWin {
    bidder: Seat,
    winning_bid: BidValue,
    completed_auction: AuctionState,
}

impl AuctionWin {
    /// The winning bidder (R-AUC-10).
    pub fn bidder(&self) -> Seat {
        self.bidder
    }

    /// The winning nonpass bid.
    pub fn winning_bid(&self) -> BidValue {
        self.winning_bid
    }

    /// The completed auction this result was certified from.
    pub fn completed_auction(&self) -> &AuctionState {
        &self.completed_auction
    }
}

/// Result of a completed auction (Exec §7 `result`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AuctionResult {
    /// All four players passed (R-AUC-11).
    AllPass,
    /// At least one nonpass bid was made.
    Win(AuctionWin),
}

impl AuctionState {
    /// A fresh auction for one shaker with no actions.
    pub fn new(shaker: Seat) -> AuctionState {
        AuctionState {
            shaker,
            actions: Vec::new(),
        }
    }

    /// The shaker of this attempt.
    pub fn shaker(&self) -> Seat {
        self.shaker
    }

    /// The recorded actions in prefix order.
    pub fn actions(&self) -> &[(Seat, AuctionAction)] {
        &self.actions
    }

    /// The next seat to act: `shaker + 1 + len(actions) mod 4`, or `None`
    /// when complete (Exec §7; R-AUC-01/02).
    pub fn next_actor(&self) -> Option<Seat> {
        if self.is_complete() {
            None
        } else {
            Some(self.shaker.offset(1 + self.actions.len() as u8))
        }
    }

    /// The most recent nonpass bid, if any (Exec §7 `currentHighBid`).
    pub fn current_high_bid(&self) -> Option<BidValue> {
        self.actions
            .iter()
            .rev()
            .find_map(|&(_, action)| match action {
                AuctionAction::Bid(v) => Some(v),
                AuctionAction::Pass => None,
            })
    }

    /// Whether a mark bid has been made.
    fn mark_bid_exists(&self) -> bool {
        self.actions
            .iter()
            .any(|&(_, a)| matches!(a, AuctionAction::Bid(BidValue::Mark(_))))
    }

    /// Whether all four actions have occurred (Exec §7 `isComplete`).
    pub fn is_complete(&self) -> bool {
        self.actions.len() == 4
    }

    /// The exact legal action set at this node (Exec §7 `legalBids`;
    /// R-AUC-06/07/08/09), in deterministic order: pass, points ascending,
    /// marks ascending.
    pub fn legal_actions(&self, config: RulesConfig) -> Vec<AuctionAction> {
        if self.is_complete() {
            return Vec::new();
        }
        let mut legal = vec![AuctionAction::Pass];
        let high = self.current_high_bid();
        // Point bids strictly exceeding the current high bid (R-AUC-06).
        for n in 30..=41 {
            let point = BidValue::Point(PointAmount::new(n).expect("30..=41"));
            if high.is_none_or(|h| point > h) {
                legal.push(AuctionAction::Bid(point));
            }
        }
        // Mark bids: before any mark bid at most two marks (R-AUC-07);
        // afterwards exactly one more than the current mark (R-AUC-08).
        let candidates: Vec<u32> = if self.mark_bid_exists() {
            match high {
                Some(BidValue::Mark(m)) => vec![m.value() + 1],
                _ => unreachable!("a mark bid is never outbid by a point bid"),
            }
        } else {
            vec![1, 2]
        };
        for m in candidates {
            if m <= config.max_mark_bid() {
                let mark = BidValue::Mark(MarkAmount::new(m).expect("m >= 1"));
                if high.is_none_or(|h| mark > h) {
                    legal.push(AuctionAction::Bid(mark));
                }
            }
        }
        legal
    }

    /// Append one validated action for the next actor (Exec §10.1
    /// `applyAuctionAction` core).
    pub fn apply(
        &self,
        action: AuctionAction,
        config: RulesConfig,
    ) -> Result<AuctionState, ObjectiveError> {
        let Some(actor) = self.next_actor() else {
            return Err(ObjectiveError::InvalidAuctionAction);
        };
        if !self.legal_actions(config).contains(&action) {
            return Err(ObjectiveError::InvalidAuctionAction);
        }
        let mut next = self.clone();
        next.actions.push((actor, action));
        Ok(next)
    }

    /// The result of a completed auction: the last nonpass bidder wins at
    /// that bid, four passes produce all-pass (Exec §7; R-AUC-10/11).
    pub fn result(&self) -> Result<AuctionResult, ObjectiveError> {
        if !self.is_complete() {
            return Err(ObjectiveError::PhaseMismatch);
        }
        let win = self
            .actions
            .iter()
            .rev()
            .find_map(|&(seat, action)| match action {
                AuctionAction::Bid(v) => Some((seat, v)),
                AuctionAction::Pass => None,
            });
        Ok(match win {
            None => AuctionResult::AllPass,
            Some((bidder, winning_bid)) => AuctionResult::Win(AuctionWin {
                bidder,
                winning_bid,
                completed_auction: self.clone(),
            }),
        })
    }
}

/// One objective deal attempt: index, deal, and auction (Exec §7).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectiveDealAttempt {
    /// The attempt index.
    pub deal_attempt_index: u64,
    /// The dealt world.
    pub deal: DealWorld,
    /// The auction state.
    pub auction: AuctionState,
}

/// Enumerate every complete legal auction history for one config, as action
/// sequences (the tree shape is shaker-independent). Exhaustion surface for
/// R-AUC-12's census.
pub fn enumerate_terminal_histories(config: RulesConfig) -> Vec<Vec<AuctionAction>> {
    let mut out = Vec::new();
    let mut stack = vec![AuctionState::new(Seat::ALL[0])];
    while let Some(state) = stack.pop() {
        if state.is_complete() {
            out.push(state.actions().iter().map(|&(_, a)| a).collect());
            continue;
        }
        for action in state.legal_actions(config) {
            stack.push(state.apply(action, config).expect("legal action applies"));
        }
    }
    out
}
