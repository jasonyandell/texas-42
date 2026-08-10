//! walt-strat -- the operators registry, S2 slice.
//!
//! PLAN.md's crate map places decision nodes, information partitions, and the
//! operators registry (PI minimax, fixed-field H, revealed C/F) here, above
//! core, kernel, and geom. S2 lands the first operator: worldwise
//! perfect-information symbolic parametric backward induction (v0.4 §9.9,
//! §10.8) with its fiber census, plus the degenerate fixed-field evaluation
//! for kernels where the focal seat never chooses after the root. Information
//! partitions and the general H/C/F treatments are S3.
//!
//! Imports core, kernel, and geom -- the strict import direction of §16.2.

pub mod census;
pub mod direction;
pub mod field;
pub mod pi;

pub use census::{pi_census, PiCensus};
pub use direction::Direction;
pub use field::{fixed_field_root_lines, PostRootChoice};
pub use pi::pi_root_values;
