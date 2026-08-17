//! Safe host-side admission boundary for the freeze-57 M3 Metal ABI.
//!
//! The shared C-layout records remain private.  Callers receive only checked
//! word wrappers and checked launch/plan values; no struct memory is a hashing
//! or persistence representation.

#![deny(unsafe_code)]

pub mod abi;
pub mod error;
pub mod launch;
pub mod ledger;

pub use abi::{
    ControlWords, FieldSlotWords, PackedState, ParticleWords, ReductionPairWords, ReductionWords,
};
pub use error::{M3MetalError, Result};
pub use launch::{
    CheckedFieldLaunch, CheckedReductionLaunch, CheckedReductionPlan, ReductionRangeSlice,
};
pub use ledger::{AllocationAdmission, ReportedAllocations};
