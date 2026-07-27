//! Declaration-relative rank, tier, and trick key.
//!
//! Implements Math §3.5 (ALG-09..11): the total rank ADT with `TOP` above
//! every integer, the contextual tier 2/1/0, and the lexicographic trick key
//! with `(0,0)` sloughs.

use core::cmp::Ordering;

use crate::declaration::Declaration;
use crate::domino::{all_ids, domino_from_id, DOMINO_COUNT};
use crate::pip::Pip;

/// Declaration-relative total rank `r_δ(d)` (Math §3.5), as an ordered ADT
/// with no numeric sentinel (INV-4).
///
/// - `Top` is above every integer rank (`⊤` of Math §3.5);
/// - `DoublePip(p)` is the doubles-trump rank of double `p:p`;
/// - `PipSum(n)` is the mixed-domino rank `sum(d)`.
///
/// `DoublePip` and `PipSum` compare by their numeric value (both live in the
/// integer part of `R = {0..12} ∪ {⊤}`); a cross-variant numeric tie is
/// broken structurally, but no single trick tier ever contains both variants
/// (doubles-trump tier 2 is all `DoublePip`, every other nonzero tier is
/// `Top`/`PipSum` only), so the tiebreak is never game-observable.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Rank {
    /// Mixed-domino rank: the pip sum (Math §3.5).
    PipSum(u8),
    /// Doubles-trump rank of double `p:p` (Math §3.5).
    DoublePip(Pip),
    /// `⊤`: above every integer rank (Math §3.5).
    Top,
}

impl Rank {
    fn numeric_and_tag(self) -> (u8, u8) {
        match self {
            Rank::PipSum(n) => (n, 0),
            Rank::DoublePip(p) => (p.value(), 1),
            Rank::Top => (u8::MAX, 2),
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        match (self, other) {
            (Rank::Top, Rank::Top) => Ordering::Equal,
            (Rank::Top, _) => Ordering::Greater,
            (_, Rank::Top) => Ordering::Less,
            _ => self.numeric_and_tag().cmp(&other.numeric_and_tag()),
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The lexicographic contextual trick key `τ_δ(d, q)` (Math §3.5; ALG-11).
///
/// `Slough` is the shared bottom key `(0,0)`: tier-zero dominoes are
/// intentionally tied at the bottom because the lead guarantees tier zero is
/// never the winning tier (Math §3.5). Derived ordering compares `Slough`
/// below every `Ranked` key, then `(tier, rank)` lexicographically.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TrickKey {
    /// The `(0,0)` key shared by every tier-zero domino.
    Slough,
    /// A nonzero-tier key, compared by `(tier, rank)`.
    Ranked {
        /// Contextual tier: 2 powered, 1 unpowered follower (Math §3.5).
        tier: u8,
        /// Declaration-relative total rank (Math §3.5).
        rank: Rank,
    },
}

/// Build the declaration-relative rank table (Math §3.5).
pub(crate) fn build_ranks(declaration: Declaration) -> [Rank; DOMINO_COUNT] {
    let mut ranks = [Rank::PipSum(0); DOMINO_COUNT];
    for id in all_ids() {
        let d = domino_from_id(id);
        ranks[id.index()] = if d.is_double() {
            match declaration {
                Declaration::DoublesTrump => Rank::DoublePip(d.high()),
                _ => Rank::Top,
            }
        } else {
            Rank::PipSum(d.pip_sum())
        };
    }
    ranks
}
