//! The domino universe: 28 tiles, identity, incidences, count decoration.
//!
//! Implements Math §2.1–§2.2 and §2.4 (ALG-01..; rec ALG-20/21): the
//! two-multiset universe `D = Sym²(P)` — equivalently the edge set of the
//! complete looped `K₇` — with the natural incidence covering and the
//! antidiagonal count decoration.

use crate::pip::{Pip, PIPS};

/// A physical domino: the canonical unordered pair `(high, low)` with
/// `0 <= low <= high <= 6` (Exec §2). Implements a member of `D` (Math §2.1).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Domino {
    high: Pip,
    low: Pip,
}

impl Domino {
    /// Canonicalizing constructor from two unordered ends.
    pub fn new(a: Pip, b: Pip) -> Domino {
        if a.value() >= b.value() {
            Domino { high: a, low: b }
        } else {
            Domino { high: b, low: a }
        }
    }

    /// The higher end (`high` of Math §2.1).
    pub const fn high(self) -> Pip {
        self.high
    }

    /// The lower end.
    pub const fn low(self) -> Pip {
        self.low
    }

    /// Literal pip membership `p ∈ d` (Math §2.2 incidence).
    pub fn contains(self, pip: Pip) -> bool {
        self.high == pip || self.low == pip
    }

    /// Whether the domino is a double (a loop of the looped `K₇`; Math §2.1).
    pub fn is_double(self) -> bool {
        self.high == self.low
    }

    /// `sum(d)` of Math §2.1.
    pub fn pip_sum(self) -> u8 {
        self.high.value() + self.low.value()
    }

    /// The Straight count decoration `c(d)` (Math §2.4; rec ALG-21):
    /// the antidiagonal formula `c(d) = sum(d)` exactly when the sum is
    /// 5 or 10, else 0. This formula is the normative executable identity;
    /// the explicit five-tile list is display only (rec Exec §4).
    pub fn count_points(self) -> u8 {
        let s = self.pip_sum();
        if s == 5 || s == 10 {
            s
        } else {
            0
        }
    }
}

/// Stable global identity of a domino under the canonical triangular order
/// `(0,0), (1,0), (1,1), (2,0), …, (6,6)` (Exec §2).
///
/// INV-7 NO-RANK-FROM-ID: identifier magnitude never determines game rank,
/// so this newtype intentionally implements no `Ord`/`PartialOrd`. Ordering
/// exists only on declaration-relative trick keys.
///
/// ```compile_fail
/// use rob_core::domino::DominoId;
/// fn forbidden(a: DominoId, b: DominoId) -> bool { a < b }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DominoId(u8);

/// Number of dominoes in the universe (Math §2.1).
pub const DOMINO_COUNT: usize = 28;

impl DominoId {
    /// The identity index in `0..28`, for storage addressing only.
    /// Storage order has no game meaning (Exec §1.10).
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// Validating constructor from a storage index.
    pub const fn from_index(index: usize) -> Option<DominoId> {
        if index < DOMINO_COUNT {
            Some(DominoId(index as u8))
        } else {
            None
        }
    }
}

/// The 28-tile universe in canonical identity order (Math §2.1; Exec §4).
pub const DOMINOES: [Domino; 28] = build_universe();

const fn build_universe() -> [Domino; 28] {
    let mut arr = [Domino {
        high: PIPS[0],
        low: PIPS[0],
    }; 28];
    let mut h = 0;
    while h < 7 {
        let mut l = 0;
        while l <= h {
            arr[h * (h + 1) / 2 + l] = Domino {
                high: PIPS[h],
                low: PIPS[l],
            };
            l += 1;
        }
        h += 1;
    }
    arr
}

/// The documented bijection `D -> DominoId` (Exec §4 `idOf`).
pub fn domino_id(domino: Domino) -> DominoId {
    let h = domino.high().value() as usize;
    let l = domino.low().value() as usize;
    DominoId((h * (h + 1) / 2 + l) as u8)
}

/// The inverse bijection `DominoId -> D` (Exec §4 `dominoOf`).
pub fn domino_from_id(id: DominoId) -> Domino {
    DOMINOES[id.index()]
}

/// All 28 identities in canonical identity order.
pub fn all_ids() -> impl Iterator<Item = DominoId> {
    (0..DOMINO_COUNT as u8).map(DominoId)
}

/// A set of dominoes, represented extensionally as a membership vector.
///
/// Sets-first representation (proof-assistant-plan "sets first"): no packed
/// bitmask codec is used as a primary representation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct DominoSet {
    member: [bool; DOMINO_COUNT],
}

impl DominoSet {
    /// The empty set.
    pub const fn empty() -> DominoSet {
        DominoSet {
            member: [false; DOMINO_COUNT],
        }
    }

    /// The whole universe.
    pub const fn full() -> DominoSet {
        DominoSet {
            member: [true; DOMINO_COUNT],
        }
    }

    /// Build from any iterator of identities.
    pub fn from_ids(ids: impl IntoIterator<Item = DominoId>) -> DominoSet {
        let mut set = DominoSet::empty();
        for id in ids {
            set.insert(id);
        }
        set
    }

    /// Membership test.
    pub fn contains(&self, id: DominoId) -> bool {
        self.member[id.index()]
    }

    /// Insert one identity.
    pub fn insert(&mut self, id: DominoId) {
        self.member[id.index()] = true;
    }

    /// Remove one identity.
    pub fn remove(&mut self, id: DominoId) {
        self.member[id.index()] = false;
    }

    /// Exact cardinality.
    pub fn len(&self) -> usize {
        self.member.iter().filter(|&&m| m).count()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        !self.member.iter().any(|&m| m)
    }

    /// Iterate members in canonical identity order (storage order only;
    /// no game meaning, Exec §1.10).
    pub fn iter(&self) -> impl Iterator<Item = DominoId> + '_ {
        self.member
            .iter()
            .enumerate()
            .filter(|&(_, &m)| m)
            .map(|(i, _)| DominoId(i as u8))
    }

    /// Set union.
    pub fn union(&self, other: &DominoSet) -> DominoSet {
        let mut out = *self;
        for (m, o) in out.member.iter_mut().zip(other.member.iter()) {
            *m = *m || *o;
        }
        out
    }

    /// Set intersection.
    pub fn intersection(&self, other: &DominoSet) -> DominoSet {
        let mut out = *self;
        for (m, o) in out.member.iter_mut().zip(other.member.iter()) {
            *m = *m && *o;
        }
        out
    }

    /// Set difference `self \ other`.
    pub fn difference(&self, other: &DominoSet) -> DominoSet {
        let mut out = *self;
        for (m, o) in out.member.iter_mut().zip(other.member.iter()) {
            *m = *m && !*o;
        }
        out
    }

    /// Subset test.
    pub fn is_subset(&self, other: &DominoSet) -> bool {
        self.member
            .iter()
            .zip(other.member.iter())
            .all(|(&m, &o)| !m || o)
    }

    /// Disjointness test.
    pub fn is_disjoint(&self, other: &DominoSet) -> bool {
        self.member
            .iter()
            .zip(other.member.iter())
            .all(|(&m, &o)| !(m && o))
    }
}

/// The natural pip-incidence set `σ_p = {d : p ∈ d}` — the closed star of
/// vertex `p` in the looped-`K₇` presentation (Math §2.2; rec ALG-20).
pub fn natural_incidence(pip: Pip) -> DominoSet {
    DominoSet::from_ids(all_ids().filter(|&id| domino_from_id(id).contains(pip)))
}
