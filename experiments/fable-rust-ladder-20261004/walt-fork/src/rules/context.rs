//! Effective contexts `Q = {0,...,6,7}`, where 7 names the called suit
//! (v0.4 §1.2).

use core::fmt;

use crate::rules::pip::Pip;

/// A context is either one of the seven natural pips or the called suit.
/// `Called` is a distinct inhabitant, never a pip: absorption makes it a
/// different thing from any natural suit.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Context {
    Natural(Pip),
    Called,
}

impl Context {
    pub const COUNT: usize = 8;
    pub const CALLED_INDEX: usize = 7;

    pub const ALL: [Context; Self::COUNT] = [
        Context::Natural(Pip::ALL[0]),
        Context::Natural(Pip::ALL[1]),
        Context::Natural(Pip::ALL[2]),
        Context::Natural(Pip::ALL[3]),
        Context::Natural(Pip::ALL[4]),
        Context::Natural(Pip::ALL[5]),
        Context::Natural(Pip::ALL[6]),
        Context::Called,
    ];

    pub const fn index(self) -> usize {
        match self {
            Context::Natural(p) => p.value() as usize,
            Context::Called => Self::CALLED_INDEX,
        }
    }

    pub const fn from_index(i: usize) -> Option<Context> {
        if i == Self::CALLED_INDEX {
            Some(Context::Called)
        } else if i < Self::CALLED_INDEX {
            match Pip::new(i as u8) {
                Some(p) => Some(Context::Natural(p)),
                None => None,
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Context::Natural(p) => write!(f, "q{p}"),
            Context::Called => f.write_str("q*"),
        }
    }
}
