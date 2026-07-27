//! Rules configuration and the bid domain.
//!
//! Implements Exec §3 (`RuleConfig`) and the auction bid forms and ordering
//! of Rules §3 (R-AUC-03..05) / Math §4.3.

/// A point-bid amount in `30..=41` (R-AUC-03).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct PointAmount(u8);

impl PointAmount {
    /// Validating constructor for `30..=41`.
    pub const fn new(value: u8) -> Option<PointAmount> {
        if value >= 30 && value <= 41 {
            Some(PointAmount(value))
        } else {
            None
        }
    }

    /// The numeric point threshold.
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// A mark-bid amount, a positive integer (R-AUC-04).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct MarkAmount(u32);

impl MarkAmount {
    /// Validating constructor for positive mark amounts.
    pub const fn new(value: u32) -> Option<MarkAmount> {
        if value >= 1 {
            Some(MarkAmount(value))
        } else {
            None
        }
    }

    /// The number of marks staked.
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// A nonpass bid value with the total comparison order
/// `P(30) < … < P(41) < M(1) < M(2) < …` (R-AUC-05; Math §4.3).
///
/// The derived ordering is exactly that order: `Point` precedes `Mark`, and
/// each compares by its numeric amount.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum BidValue {
    /// A point bid `P(n)`, threshold `n`, stake 1 (R-CONTRACT-01).
    Point(PointAmount),
    /// A mark bid `M(m)`, threshold 42, stake `m` (R-CONTRACT-02).
    Mark(MarkAmount),
}

/// One auction action: pass, or a nonpass bid (Exec §7 `Bid`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AuctionAction {
    /// Pass — legal at every incomplete auction node (R-AUC-09).
    Pass,
    /// A nonpass bid.
    Bid(BidValue),
}

/// Rules configuration (Exec §3 `RuleConfig`). The all-pass rule
/// (RESHAKE_NEXT), deal law (uniform ordered deals), and cross-deal law
/// (independent deals) are fixed adopted rules of this profile and are not
/// configurable fields.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RulesConfig {
    max_mark_bid: u32,
    match_target: u32,
}

impl RulesConfig {
    /// Validating constructor: both values must be at least 1
    /// (R-CONFIG-01/02).
    pub const fn new(max_mark_bid: u32, match_target: u32) -> Option<RulesConfig> {
        if max_mark_bid >= 1 && match_target >= 1 {
            Some(RulesConfig {
                max_mark_bid,
                match_target,
            })
        } else {
            None
        }
    }

    /// The configured maximum mark bid `m_max` (R-CONFIG-02).
    pub const fn max_mark_bid(self) -> u32 {
        self.max_mark_bid
    }

    /// The match target `T` (R-CONFIG-01).
    pub const fn match_target(self) -> u32 {
        self.match_target
    }

    /// The reachable mark ceiling `min(m_max, 5)` (R-AUC-12; Math §4.3
    /// structural reachable ceiling).
    pub const fn reachable_max_mark_bid(self) -> u32 {
        if self.max_mark_bid < 5 {
            self.max_mark_bid
        } else {
            5
        }
    }
}
