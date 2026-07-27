//! Suit contexts: called/powered sets, effective suits, led suit, follow.
//!
//! Implements Math §3.2–§3.4 (ALG-05..08) and the lead-context structure
//! used by reachability (Math §7.13.2; REACH-05).

use crate::declaration::Declaration;
use crate::domino::{all_ids, domino_from_id, natural_incidence, DominoSet, DOMINO_COUNT};
use crate::pip::{Pip, PIPS};

/// A led suit context: one of the seven natural pip contexts or the called
/// suit `7` (Exec §2 `LedSuit`; Math §3.3 `Q`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LedSuit {
    /// Natural pip context `q ∈ P`.
    Natural(Pip),
    /// The called suit, index 7 (Math §3.3).
    Called,
}

/// Number of suit contexts (`|Q| = 8`, Math §3.3).
pub const CONTEXT_COUNT: usize = 8;

impl LedSuit {
    /// Storage index in `0..8`: natural contexts at their pip value, called
    /// suit at 7 (Math §3.3 coordinates).
    pub fn context_index(self) -> usize {
        match self {
            LedSuit::Natural(p) => p.value() as usize,
            LedSuit::Called => 7,
        }
    }

    /// Inverse of [`LedSuit::context_index`].
    pub fn from_context_index(index: usize) -> Option<LedSuit> {
        match index {
            0..=6 => Some(LedSuit::Natural(PIPS[index])),
            7 => Some(LedSuit::Called),
            _ => None,
        }
    }

    /// All eight contexts in index order.
    pub fn all() -> [LedSuit; CONTEXT_COUNT] {
        [
            LedSuit::Natural(PIPS[0]),
            LedSuit::Natural(PIPS[1]),
            LedSuit::Natural(PIPS[2]),
            LedSuit::Natural(PIPS[3]),
            LedSuit::Natural(PIPS[4]),
            LedSuit::Natural(PIPS[5]),
            LedSuit::Natural(PIPS[6]),
            LedSuit::Called,
        ]
    }
}

/// A set of suit contexts (e.g. a domino's effective suits, a void mask).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct LedSuitSet {
    member: [bool; CONTEXT_COUNT],
}

impl LedSuitSet {
    /// The empty set.
    pub const fn empty() -> LedSuitSet {
        LedSuitSet {
            member: [false; CONTEXT_COUNT],
        }
    }

    /// Membership test.
    pub fn contains(&self, q: LedSuit) -> bool {
        self.member[q.context_index()]
    }

    /// Insert one context.
    pub fn insert(&mut self, q: LedSuit) {
        self.member[q.context_index()] = true;
    }

    /// Exact cardinality.
    pub fn len(&self) -> usize {
        self.member.iter().filter(|&&m| m).count()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        !self.member.iter().any(|&m| m)
    }

    /// Iterate members in context-index order.
    pub fn iter(&self) -> impl Iterator<Item = LedSuit> + '_ {
        self.member
            .iter()
            .enumerate()
            .filter(|&(_, &m)| m)
            .filter_map(|(i, _)| LedSuit::from_context_index(i))
    }
}

/// Precomputed suit-relational tables for one declaration.
pub(crate) struct SuitTables {
    /// Called set `κ_δ` (Math §3.2).
    pub called: DominoSet,
    /// Powered set `π_δ` (Math §3.2).
    pub powered: DominoSet,
    /// Follow relation `F_δ(d, q) = [d ∈ σ̂_q^δ]` (Math §3.4).
    pub follow: [[bool; CONTEXT_COUNT]; DOMINO_COUNT],
    /// Led suit `ℓ_δ(d)` (Math §3.4).
    pub led_suit: [LedSuit; DOMINO_COUNT],
}

/// Build the suit tables from the ingest definitions (Math §3.2–§3.4).
pub(crate) fn build_suit_tables(declaration: Declaration) -> SuitTables {
    let doubles = DominoSet::from_ids(all_ids().filter(|&id| domino_from_id(id).is_double()));
    let called = match declaration {
        Declaration::PipTrump(p) => natural_incidence(p),
        Declaration::DoublesTrump => doubles,
        Declaration::NoTrump => DominoSet::empty(),
    };
    // In Straight 42 every nonempty called set is powered (Math §3.2).
    let powered = called;

    let mut follow = [[false; CONTEXT_COUNT]; DOMINO_COUNT];
    for q in 0..7 {
        // Effective natural suit: σ̂_q = σ_q \ κ (Math §3.3).
        let effective = natural_incidence(PIPS[q]).difference(&called);
        for id in effective.iter() {
            follow[id.index()][q] = true;
        }
    }
    // Effective called suit: σ̂_7 = κ (Math §3.3).
    for id in called.iter() {
        follow[id.index()][7] = true;
    }

    let mut led_suit = [LedSuit::Called; DOMINO_COUNT];
    for id in all_ids() {
        led_suit[id.index()] = if called.contains(id) {
            LedSuit::Called
        } else {
            LedSuit::Natural(domino_from_id(id).high())
        };
    }

    SuitTables {
        called,
        powered,
        follow,
        led_suit,
    }
}
