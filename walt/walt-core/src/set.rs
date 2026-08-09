//! Bitset views over the 28 dominoes and the 8 contexts.

use core::fmt;

use crate::context::Context;
use crate::domino::Domino;

/// A subset of the 28 dominoes, one bit per stable domino index.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DominoSet(u32);

impl DominoSet {
    pub const EMPTY: DominoSet = DominoSet(0);
    pub const FULL: DominoSet = DominoSet((1u32 << Domino::COUNT) - 1);

    pub const fn from_bits(bits: u32) -> Option<DominoSet> {
        if bits & !Self::FULL.0 == 0 {
            Some(DominoSet(bits))
        } else {
            None
        }
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn single(d: Domino) -> DominoSet {
        DominoSet(1u32 << d.index())
    }

    pub const fn contains(self, d: Domino) -> bool {
        self.0 & (1u32 << d.index()) != 0
    }

    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn union(self, other: DominoSet) -> DominoSet {
        DominoSet(self.0 | other.0)
    }

    pub const fn intersection(self, other: DominoSet) -> DominoSet {
        DominoSet(self.0 & other.0)
    }

    pub const fn difference(self, other: DominoSet) -> DominoSet {
        DominoSet(self.0 & !other.0)
    }

    pub const fn is_subset_of(self, other: DominoSet) -> bool {
        self.0 & !other.0 == 0
    }

    pub const fn is_disjoint(self, other: DominoSet) -> bool {
        self.0 & other.0 == 0
    }

    pub fn insert(&mut self, d: Domino) -> bool {
        let had = self.contains(d);
        self.0 |= 1u32 << d.index();
        !had
    }

    pub fn remove(&mut self, d: Domino) -> bool {
        let had = self.contains(d);
        self.0 &= !(1u32 << d.index());
        had
    }

    pub fn iter(self) -> DominoSetIter {
        DominoSetIter(self.0)
    }

    /// Total count decoration `sum c(d)` over the set (v0.4 §1.4).
    pub fn count_points(self) -> u32 {
        self.iter().map(Domino::count).sum()
    }
}

impl FromIterator<Domino> for DominoSet {
    fn from_iter<I: IntoIterator<Item = Domino>>(iter: I) -> DominoSet {
        let mut out = DominoSet::EMPTY;
        for d in iter {
            out.insert(d);
        }
        out
    }
}

impl IntoIterator for DominoSet {
    type Item = Domino;
    type IntoIter = DominoSetIter;

    fn into_iter(self) -> DominoSetIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct DominoSetIter(u32);

impl Iterator for DominoSetIter {
    type Item = Domino;

    fn next(&mut self) -> Option<Domino> {
        if self.0 == 0 {
            return None;
        }
        let i = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1;
        Domino::from_index(i)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.0.count_ones() as usize;
        (n, Some(n))
    }
}

impl ExactSizeIterator for DominoSetIter {}

impl fmt::Debug for DominoSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        for (i, d) in self.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{d}")?;
        }
        f.write_str("}")
    }
}

/// A subset of the eight contexts `Q = {0..6, called}`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ContextSet(u8);

impl ContextSet {
    pub const EMPTY: ContextSet = ContextSet(0);
    pub const FULL: ContextSet = ContextSet(0xFF);

    pub const fn single(q: Context) -> ContextSet {
        ContextSet(1u8 << q.index())
    }

    pub const fn contains(self, q: Context) -> bool {
        self.0 & (1u8 << q.index()) != 0
    }

    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn union(self, other: ContextSet) -> ContextSet {
        ContextSet(self.0 | other.0)
    }

    pub fn insert(&mut self, q: Context) -> bool {
        let had = self.contains(q);
        self.0 |= 1u8 << q.index();
        !had
    }

    pub fn iter(self) -> impl Iterator<Item = Context> {
        Context::ALL.into_iter().filter(move |q| self.contains(*q))
    }
}

impl FromIterator<Context> for ContextSet {
    fn from_iter<I: IntoIterator<Item = Context>>(iter: I) -> ContextSet {
        let mut out = ContextSet::EMPTY;
        for q in iter {
            out.insert(q);
        }
        out
    }
}

impl fmt::Debug for ContextSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        for (i, q) in self.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }
            write!(f, "{q}")?;
        }
        f.write_str("}")
    }
}
