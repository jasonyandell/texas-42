//! Complete deal worlds and the exact chance-domain counts.
//!
//! Implements Exec §6 (`DealWorld`, `UniformOrderedDealLaw` count) and the
//! Math §6–§7 domain sizes. A complete initial deal is never silently
//! substituted for a current hidden-hand assignment (Exec §1.11; INV-9).

use num_bigint::BigUint;
use num_traits::One;

use crate::domino::{DominoSet, DOMINO_COUNT};
use crate::objective::ObjectiveError;
use crate::seat::Seat;

/// A complete initial deal: four labeled seven-domino hands partitioning the
/// universe (Exec §6 `DealWorld`; R-DEAL-01).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DealWorld {
    initial_hands: [DominoSet; 4],
}

impl DealWorld {
    /// Validating constructor: each hand has exactly 7 dominoes, hands are
    /// pairwise disjoint, and their union is the whole universe (Exec §6).
    pub fn new(initial_hands: [DominoSet; 4]) -> Result<DealWorld, ObjectiveError> {
        let mut union = DominoSet::empty();
        for (i, hand) in initial_hands.iter().enumerate() {
            if hand.len() != 7 {
                return Err(ObjectiveError::InvalidDeal);
            }
            for other in initial_hands.iter().skip(i + 1) {
                if !hand.is_disjoint(other) {
                    return Err(ObjectiveError::InvalidDeal);
                }
            }
            union = union.union(hand);
        }
        if union.len() != DOMINO_COUNT {
            return Err(ObjectiveError::InvalidDeal);
        }
        Ok(DealWorld { initial_hands })
    }

    /// The hand initially dealt to one seat.
    pub fn hand(&self, seat: Seat) -> &DominoSet {
        &self.initial_hands[seat.index()]
    }
}

/// Exact binomial coefficient `C(n, k)` as a `BigUint`.
fn binomial(n: u64, k: u64) -> BigUint {
    let mut result = BigUint::one();
    for i in 0..k.min(n) {
        result = result * BigUint::from(n - i) / BigUint::from(i + 1);
    }
    if k > n {
        BigUint::from(0u32)
    } else {
        result
    }
}

/// The ordered-deal domain size `28!/(7!)^4`, computed exactly from the deal
/// definition as the multinomial number of ordered four-hand partitions
/// (Math §6; Exec §6 `orderedDealCount`).
pub fn ordered_deal_count() -> BigUint {
    binomial(28, 7) * binomial(21, 7) * binomial(14, 7) * binomial(7, 7)
}

/// The conditional hidden-assignment count for one viewer, `21!/(7!)^3`:
/// ordered three-hand partitions of the 21 unseen tiles (Math §7).
pub fn hidden_assignment_count() -> BigUint {
    binomial(21, 7) * binomial(14, 7) * binomial(7, 7)
}
