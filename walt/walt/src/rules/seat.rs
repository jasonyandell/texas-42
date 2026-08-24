//! Seats `Z/4Z` and the two partnerships (v0.4 §1.1).

use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Seat {
    S0,
    S1,
    S2,
    S3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Team {
    T0,
    T1,
}

impl Seat {
    pub const COUNT: usize = 4;
    pub const ALL: [Seat; 4] = [Seat::S0, Seat::S1, Seat::S2, Seat::S3];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn from_index(i: usize) -> Option<Seat> {
        match i {
            0 => Some(Seat::S0),
            1 => Some(Seat::S1),
            2 => Some(Seat::S2),
            3 => Some(Seat::S3),
            _ => None,
        }
    }

    /// `s+ = s + 1 (mod 4)`.
    pub const fn successor(self) -> Seat {
        match self {
            Seat::S0 => Seat::S1,
            Seat::S1 => Seat::S2,
            Seat::S2 => Seat::S3,
            Seat::S3 => Seat::S0,
        }
    }

    /// The `n`-th seat clockwise from this one.
    pub const fn plus(self, n: usize) -> Seat {
        match Seat::from_index((self.index() + n) % 4) {
            Some(s) => s,
            None => unreachable!(),
        }
    }

    /// `theta(s) = s mod 2`.
    pub const fn team(self) -> Team {
        match self {
            Seat::S0 | Seat::S2 => Team::T0,
            Seat::S1 | Seat::S3 => Team::T1,
        }
    }
}

impl Team {
    pub const ALL: [Team; 2] = [Team::T0, Team::T1];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn other(self) -> Team {
        match self {
            Team::T0 => Team::T1,
            Team::T1 => Team::T0,
        }
    }

    pub const fn seats(self) -> [Seat; 2] {
        match self {
            Team::T0 => [Seat::S0, Seat::S2],
            Team::T1 => [Seat::S1, Seat::S3],
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S{}", self.index())
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}", self.index())
    }
}
