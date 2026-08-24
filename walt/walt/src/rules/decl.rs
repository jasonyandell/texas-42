//! The Straight declaration set `Delta_Str = P u {DT, NT}` (v0.4 §1.2).

use core::fmt;
use core::str::FromStr;

use crate::rules::pip::Pip;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Decl {
    PipTrump(Pip),
    DoublesTrump,
    NoTrump,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum DeclClass {
    PipTrump,
    DoublesTrump,
    NoTrump,
}

impl Decl {
    pub const COUNT: usize = 9;

    pub const ALL: [Decl; Self::COUNT] = [
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

    pub const fn class(self) -> DeclClass {
        match self {
            Decl::PipTrump(_) => DeclClass::PipTrump,
            Decl::DoublesTrump => DeclClass::DoublesTrump,
            Decl::NoTrump => DeclClass::NoTrump,
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decl::PipTrump(p) => write!(f, "P{p}"),
            Decl::DoublesTrump => f.write_str("DT"),
            Decl::NoTrump => f.write_str("NT"),
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
            _ => {}
        }
        let err = || ParseDeclError(s.to_string());
        let rest = s.strip_prefix('P').ok_or_else(err)?;
        let v: u8 = rest.parse().map_err(|_| err())?;
        Ok(Decl::PipTrump(Pip::new(v).ok_or_else(err)?))
    }
}
