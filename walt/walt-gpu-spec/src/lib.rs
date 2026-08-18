//! Portable, exact host-side contracts for the GPU-native Walt solver.
//!
//! This crate deliberately contains no Metal binding or shader source.  Its
//! first responsibility is to make the byte, integer-width, scale, and
//! semantic-table contracts independently testable on the CPU.

#![forbid(unsafe_code)]

pub mod digest;
pub mod mass;
pub mod tables;

pub use digest::{sha256, Sha256State, SHA256_BYTES};
pub use mass::{
    ExactMass, ExactMassError, FieldProfileId, MeasureRoleId, OpeningLikelihoodCoeff,
    OpeningResponseFrame, OpeningResponseFrameError, PriorProfileId, ScaleFrame, ScaledOpeningMass,
    SupportCount, U256Mass, UtilityProfileId, FIELD_SCALE, OPENING_RESPONSE_FIELD_EXPONENT,
    SCALE_FRAME_BYTES, TRICK1_FULL_HORIZON_EXPONENT, U256_BYTES, U256_LIMBS,
};
pub use tables::{SemanticTables, TABLE_FORMAT_VERSION};
