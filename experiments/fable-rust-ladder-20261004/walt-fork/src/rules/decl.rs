//! Original suit algebra: pip trumps, doubles-trump, doubles-suit, no-trump.
//! `STRAIGHT` retains the nine-declaration formal/auction subdomain.

use core::fmt;
use core::str::FromStr;

use crate::rules::pip::Pip;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Decl {
    PipTrump(Pip),
    DoublesTrump,
    NoTrump,
    DoublesSuit,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DeclClass {
    PipTrump,
    DoublesTrump,
    NoTrump,
    DoublesSuit,
}

impl Decl {
    pub const COUNT: usize = 10;
    pub const STRAIGHT_COUNT: usize = 9;

    pub const STRAIGHT: [Decl; Self::STRAIGHT_COUNT] = [
        Decl::PipTrump(Pip::ALL[0]),
        Decl::PipTrump(Pip::ALL[1]),
        Decl::PipTrump(Pip::ALL[2]),
        Decl::PipTrump(Pip::ALL[3]),
        Decl::PipTrump(Pip::ALL[4]),
        Decl::PipTrump(Pip::ALL[5]),
        Decl::PipTrump(Pip::ALL[6]),
        Decl::DoublesTrump,
        Decl::NoTrump,
    ];

    pub const ALL: [Decl; Self::COUNT] = [
        Self::STRAIGHT[0],
        Self::STRAIGHT[1],
        Self::STRAIGHT[2],
        Self::STRAIGHT[3],
        Self::STRAIGHT[4],
        Self::STRAIGHT[5],
        Self::STRAIGHT[6],
        Self::DoublesTrump,
        Self::NoTrump,
        Self::DoublesSuit,
    ];

    pub const fn is_straight(self) -> bool {
        !matches!(self, Self::DoublesSuit)
    }

    pub const fn class(self) -> DeclClass {
        match self {
            Decl::PipTrump(_) => DeclClass::PipTrump,
            Decl::DoublesTrump => DeclClass::DoublesTrump,
            Decl::NoTrump => DeclClass::NoTrump,
            Decl::DoublesSuit => DeclClass::DoublesSuit,
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::PipTrump(p) => write!(f, "P{p}"),
            Decl::DoublesTrump => f.write_str("DT"),
            Decl::NoTrump => f.write_str("NT"),
            Decl::DoublesSuit => f.write_str("DS"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseDeclError(pub String);

impl fmt::Display for ParseDeclError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a declaration: {}", self.0)
    }
}

impl std::error::Error for ParseDeclError {}

impl FromStr for Decl {
    type Err = ParseDeclError;

    fn from_str(s: &str) -> Result<Decl, ParseDeclError> {
        match s {
            "D" | "DT" => return Ok(Decl::DoublesTrump),
            "N" | "NT" => return Ok(Decl::NoTrump),
            "DS" => return Ok(Decl::DoublesSuit),
            _ => {}
        }
        let err = || ParseDeclError(s.to_string());
        let rest = s.strip_prefix('P').ok_or_else(err)?;
        let v: u8 = rest.parse().map_err(|_| err())?;
        Ok(Decl::PipTrump(Pip::new(v).ok_or_else(err)?))
    }
}
