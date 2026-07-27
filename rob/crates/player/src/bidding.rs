//! Placeholder bidding — deliberately NOT a modeled policy field.
//!
//! v0 scope is play only. This heuristic exists solely to always produce a
//! legal auction and a deterministic declaration; it reads no opponent
//! model, and the uniform-fiber belief deliberately does not read it back
//! (the auction-likelihood tilt is a later, separately tested upgrade —
//! Math §10.4's 90-world witness is the regression test for that step, not
//! for v0).

use rob_core::{
    natural_incidence, AuctionAction, BidValue, Declaration, DominoSet, PointAmount, PIPS,
};

/// The fixed placeholder auction: the seat left of the shaker bids `P(30)`,
/// everyone else passes — always legal (R-AUC-06/07), always a winner.
pub fn placeholder_auction_script() -> [AuctionAction; 4] {
    [
        AuctionAction::Bid(BidValue::Point(PointAmount::new(30).expect("30"))),
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ]
}

/// The fixed placeholder declaration: the pip trump with the most tiles in
/// the bidder's hand, ties toward the higher pip. Doubles-trump and
/// no-trump are never chosen by the placeholder.
pub fn placeholder_declaration(hand: &DominoSet) -> Declaration {
    let best = PIPS
        .iter()
        .map(|&p| (hand.intersection(&natural_incidence(p)).len(), p))
        .max_by_key(|&(count, p)| (count, p.value()))
        .expect("seven pips")
        .1;
    Declaration::PipTrump(best)
}
