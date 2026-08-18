//! Supervised, sequential freeze-56 M2 Metal evidence runner.
//!
//! The portable reference and Metal crates own semantic validation and GPU
//! provenance respectively.  This crate owns only closed host observation,
//! child framing, supervision, and canonical receipt assembly.

#![forbid(unsafe_code)]

pub mod assembly;
pub mod child;
pub mod observation;
pub mod protocol;
