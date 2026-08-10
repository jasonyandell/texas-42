//! walt-geom -- exact one-parameter policy geometry.
//!
//! Implements the finite-first slice of v0.4 §8--§9: exact rationals, affine
//! lines in one valuation parameter, continuous piecewise-linear envelopes on
//! the ray with the endpoint-ownership discipline, argmax correspondences,
//! capture features, and finite feature sets with their support functions
//! (§9.2). The polytope of §9.1 is carried as its finite generating point set
//! (§16.1); support evaluation never needs the hull.
//!
//! Imports walt-core only. Exact integers and rationals throughout.

pub mod correspond;
pub mod envelope;
pub mod feature;
pub mod line;
pub mod rat;

pub use correspond::{argmax_correspondence, ArgmaxCorrespondence};
pub use envelope::{Envelope, Piece};
pub use feature::{FeatureSet, FeatureVec};
pub use line::Line;
pub use rat::{q, qi, Q};
