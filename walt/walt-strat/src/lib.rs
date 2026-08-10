//! walt-strat -- the operators registry, S2+S3 slices.
//!
//! PLAN.md's crate map places decision nodes, information partitions, and the
//! operators registry here, above core, kernel, and geom. The registry now
//! holds the four named operators of v0.4 §7.7/§10.3, kept deliberately
//! distinct (§10.8: theorems for one operator never silently transfer):
//!
//! - **PI** (`pi`): worldwise perfect-information symbolic parametric
//!   backward induction (§9.9) with its fiber census (`census`);
//! - **H** (`hidden`): the actual hidden-information fixed-field treatment --
//!   viewer choices on pooled information states against the fiber;
//! - **C**/**F** (`revealed`): continuation- and root-revelation with the
//!   field held fixed, aggregated at the support level;
//! - prices between them (`price`): the §10.5 information prices and the
//!   §10.6 zero-information reading;
//! - **scalar PI** (`scalar`): the same PI operator at one integer valuation,
//!   cached at trick boundaries, carrying the exp5 probe-suite censuses
//!   (`q_trick`/`q_points` value classes, optimal-action classes).
//!
//! `info` carries the substrate: decision nodes over fiber-worlds, the
//! canonical perfect-recall information partition (§10.1), and
//! information-consistent policies as maps from opaque information-state ids
//! (§7.2 -- world-peeking is unconstructible by type).
//!
//! S2's `field::fixed_field_root_lines` (degenerate fixed-field evaluation,
//! refusing any kernel with a post-root focal choice) is superseded by the
//! general H operator and deleted in S3.
//!
//! Imports core, kernel, and geom -- the strict import direction of §16.2.

pub mod census;
pub mod direction;
pub mod hidden;
pub mod info;
pub mod label;
pub mod pi;
pub mod price;
pub mod revealed;
pub mod scalar;

pub use census::{pi_census, PiCensus};
pub use direction::Direction;
pub use hidden::hidden_root_values;
pub use info::{policy_value, InfoPartition, InfoStateId, Policy};
pub use label::{OperatorLabel, WeightingLabel};
pub use pi::pi_root_values;
pub use price::{information_prices, upper_value, zero_information, InfoPrices};
pub use revealed::{revealed_summary, revealed_world_root_values, RevealedSummary};
pub use scalar::{scalar_census, scalar_fiber_census, ScalarCensus, ScalarPi, ScalarValuation};
