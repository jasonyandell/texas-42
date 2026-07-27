//! Contract certification and settlement arithmetic.
//!
//! Implements Exec §8 (`Contract`, `contractFromAuction`, `settle`) and
//! Math §4.5.

use crate::declaration::Declaration;
use crate::objective::auction::{AuctionResult, AuctionState, AuctionWin};
use crate::objective::ObjectiveError;
use crate::rules::{BidValue, RulesConfig};
use crate::seat::{Seat, Team};

/// A certified contract: bidder, winning bid, and declaration (Exec §8).
/// Constructed only through [`contract_from_auction`]: a raw record with
/// plausible fields is not evidence that its bid was reachable.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Contract {
    bidder: Seat,
    winning_bid: BidValue,
    declaration: Declaration,
}

impl Contract {
    /// The winning bidder (leads trick one, R-LEAD-01).
    pub fn bidder(&self) -> Seat {
        self.bidder
    }

    /// The winning nonpass bid.
    pub fn winning_bid(&self) -> BidValue {
        self.winning_bid
    }

    /// The declared trump interpretation (R-DECL-01).
    pub fn declaration(&self) -> Declaration {
        self.declaration
    }

    /// The declaring partnership (Math §2.3 orientation).
    pub fn declaring_team(&self) -> Team {
        self.bidder.team()
    }

    /// Contract threshold: `n` for `P(n)`, 42 for `M(m)` (Exec §8;
    /// R-CONTRACT-01/02).
    pub fn threshold(&self) -> u32 {
        match self.winning_bid {
            BidValue::Point(n) => n.value() as u32,
            BidValue::Mark(_) => 42,
        }
    }

    /// Contract stake in marks: 1 for `P(n)`, `m` for `M(m)` (Exec §8).
    pub fn stake(&self) -> u32 {
        match self.winning_bid {
            BidValue::Point(_) => 1,
            BidValue::Mark(m) => m.value(),
        }
    }
}

/// Validate a complete auction replay under `config` and certify its result
/// (Exec §8 `contractFromAuction` validation core).
fn validate_completed_auction(
    auction: &AuctionState,
    config: RulesConfig,
) -> Result<AuctionResult, ObjectiveError> {
    let mut replay = AuctionState::new(auction.shaker());
    for &(seat, action) in auction.actions() {
        if replay.next_actor() != Some(seat) {
            return Err(ObjectiveError::InvalidContract);
        }
        replay = replay
            .apply(action, config)
            .map_err(|_| ObjectiveError::InvalidContract)?;
    }
    if !replay.is_complete() {
        return Err(ObjectiveError::InvalidContract);
    }
    replay.result()
}

/// Certify a contract from an auction win and a declaration (Exec §8
/// `contractFromAuction`): validates that the completed auction is legal
/// under `config` and that `win` is its actual result. The nine-declaration
/// Straight domain is enforced by the `Declaration` type itself.
pub fn contract_from_auction(
    win: &AuctionWin,
    declaration: Declaration,
    config: RulesConfig,
) -> Result<Contract, ObjectiveError> {
    match validate_completed_auction(win.completed_auction(), config)? {
        AuctionResult::AllPass => Err(ObjectiveError::InvalidContract),
        AuctionResult::Win(actual) => {
            if actual.bidder() != win.bidder() || actual.winning_bid() != win.winning_bid() {
                return Err(ObjectiveError::InvalidContract);
            }
            Ok(Contract {
                bidder: win.bidder(),
                winning_bid: win.winning_bid(),
                declaration,
            })
        }
    }
}

/// Settlement award: which partnership receives how many marks (Exec §8
/// `HandAward`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HandAward {
    /// The partnership receiving the stake.
    pub team: Team,
    /// The stake in marks.
    pub marks: u32,
    /// Whether the declaring partnership made its contract.
    pub made: bool,
}

/// Settle a contract against final points by team (Exec §8 `settle`;
/// R-SETTLE-01..04). Requires the 42-point conservation invariant.
pub fn settle(
    contract: &Contract,
    final_points_by_team: [u32; 2],
) -> Result<HandAward, ObjectiveError> {
    if final_points_by_team[0] + final_points_by_team[1] != 42 {
        return Err(ObjectiveError::InvariantViolation);
    }
    let declaring = contract.declaring_team();
    let made = final_points_by_team[declaring.index()] >= contract.threshold();
    Ok(HandAward {
        team: if made {
            declaring
        } else {
            declaring.opponent()
        },
        marks: contract.stake(),
        made,
    })
}
