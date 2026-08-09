//! Pips: the seven ends of the double-six universe (v0.4 §1.1).

use core::fmt;

/// A pip value in `0..=6`. The inner value is private: `Pip` cannot name a
/// pip outside the double-six universe.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pip(pub(crate) u8);

impl Pip {
    pub const COUNT: usize = 7;
    pub const MAX: u8 = 6;

    pub const ALL: [Pip; Self::COUNT] = [Pip(0), Pip(1), Pip(2), Pip(3), Pip(4), Pip(5), Pip(6)];

    pub const fn new(v: u8) -> Option<Pip> {
        if v <= Self::MAX {
            Some(Pip(v))
        } else {
            None
        }
    }

    pub const fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Display for Pip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Pip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pip({})", self.0)
    }
}
