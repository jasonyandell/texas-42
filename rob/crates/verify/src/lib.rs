//! rob-verify: receipt binaries and independent verification harnesses
//! (BRIEF §8–§9).
//!
//! The prose-rule resolver here is deliberately independent of the algebra
//! implementation (BRIEF §1 D4): it consumes only domino identity and the
//! declaration enum.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod prose_resolver;
pub mod receipt;
pub mod s1;
