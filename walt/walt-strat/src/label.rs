//! Declared-knob labels for the operators registry (v0.4 §7.7, §10.3) and
//! the belief-side weighting declaration (§2.4).
//!
//! Every graded verdict downstream of the registry (S5 conflicts, lessons)
//! must carry which operator produced its values and which world weighting
//! its expectations used. The labels are data, not documentation: a verdict
//! type that stores them cannot be quoted without them. Worldwise dominance
//! is weighting-free but never operator-free — the values compared are still
//! one operator's values, and §7.6's policy-gluing gaps are action-specific,
//! so PI dominance does not imply H dominance.

use core::fmt;

/// Which solution operator produced a value (§7.7's registry). Only `Pi` is
/// consumed by the S5a walker; the other operators exist in this crate and
/// the enum admits them so a verdict can name them without a type change.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum OperatorLabel {
    /// Worldwise perfect-information backward induction (§9.9; `scalar`,
    /// `pi`). Averaging it over a fiber is the information-relaxed
    /// diagnostic of §7.6/§12.4, never the seat's hidden value.
    Pi,
    /// The actual hidden-information fixed-field treatment (§10.3 H).
    H,
    /// Continuation-revelation with the field held fixed (§10.3 C).
    C,
    /// Root-revelation with the field held fixed (§10.3 F).
    F,
}

/// Which world weighting an expectation used. Support gives the allowed
/// domain; the weighting is a declared belief on it (§2.4), never something
/// support determines.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum WeightingLabel {
    /// Every fiber world weighted equally.
    UniformOverFiber,
}

impl fmt::Display for OperatorLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorLabel::Pi => write!(f, "PI"),
            OperatorLabel::H => write!(f, "H"),
            OperatorLabel::C => write!(f, "C"),
            OperatorLabel::F => write!(f, "F"),
        }
    }
}

impl fmt::Display for WeightingLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeightingLabel::UniformOverFiber => write!(f, "uniform-over-fiber"),
        }
    }
}
