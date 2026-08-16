//! The 28 dominoes of `Sym^2(P)` (v0.4 §1.1) and the count decoration (§1.4).

use core::fmt;
use core::str::FromStr;

use crate::pip::Pip;

/// A domino, stored in its stable physical identity `h:l` with `h >= l`.
/// The ordering is a naming convention, not a physical orientation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Domino {
    hi: Pip,
    lo: Pip,
}

const fn tri_index(hi: u8, lo: u8) -> usize {
    (hi * (hi + 1) / 2 + lo) as usize
}

impl Domino {
    pub const COUNT: usize = 28;

    pub const ALL: [Domino; Self::COUNT] = {
        let mut out = [Domino {
            hi: Pip(0),
            lo: Pip(0),
        }; Self::COUNT];
        let mut hi = 0u8;
        while hi <= Pip::MAX {
            let mut lo = 0u8;
            while lo <= hi {
                out[tri_index(hi, lo)] = Domino {
                    hi: Pip(hi),
                    lo: Pip(lo),
                };
                lo += 1;
            }
            hi += 1;
        }
        out
    };

    /// Normalizes to `h:l` with `h >= l`; the two ends are an unordered pair.
    pub const fn new(a: Pip, b: Pip) -> Domino {
        if a.value() >= b.value() {
            Domino { hi: a, lo: b }
        } else {
            Domino { hi: b, lo: a }
        }
    }

    pub const fn hi(self) -> Pip {
        self.hi
    }

    pub const fn lo(self) -> Pip {
        self.lo
    }

    pub const fn is_double(self) -> bool {
        self.hi.value() == self.lo.value()
    }

    pub const fn has(self, p: Pip) -> bool {
        self.hi.value() == p.value() || self.lo.value() == p.value()
    }

    pub const fn pip_sum(self) -> u8 {
        self.hi.value() + self.lo.value()
    }

    /// The ordinary Straight count decoration `c(d)` (v0.4 §1.4).
    pub const fn count(self) -> u32 {
        match self.pip_sum() {
            5 => 5,
            10 => 10,
            _ => 0,
        }
    }

    pub const fn index(self) -> usize {
        tri_index(self.hi.value(), self.lo.value())
    }

    pub const fn from_index(i: usize) -> Option<Domino> {
        if i < Self::COUNT {
            Some(Self::ALL[i])
        } else {
            None
        }
    }
}

impl fmt::Display for Domino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.hi, self.lo)
    }
}

impl fmt::Debug for Domino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.hi, self.lo)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseDominoError(pub String);

impl fmt::Display for ParseDominoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a domino: {}", self.0)
    }
}

impl std::error::Error for ParseDominoError {}

impl FromStr for Domino {
    type Err = ParseDominoError;

    fn from_str(s: &str) -> Result<Domino, ParseDominoError> {
        let err = || ParseDominoError(s.to_string());
        let (a, b) = s.split_once('-').ok_or_else(err)?;
        let a: u8 = a.parse().map_err(|_| err())?;
        let b: u8 = b.parse().map_err(|_| err())?;
        let a = Pip::new(a).ok_or_else(err)?;
        let b = Pip::new(b).ok_or_else(err)?;
        Ok(Domino::new(a, b))
    }
}
