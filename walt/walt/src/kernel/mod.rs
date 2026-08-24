//! walt-kernel -- the viewer kernel and its current-remainder fiber.
//!
//! Implements v0.4 §2.1: a viewer's known hand, the hidden live pool, the
//! per-hidden-seat capacities, and the observable void constraints; the fiber
//! `Phi(C)` of the resulting dependent capacity-cell system, with exact
//! enumeration, exact counting, and exact uniform sampling.
//!
//! Imports walt-core only. All arithmetic is integer or rational; the PRNG
//! selects, it never computes a value.

pub mod decision;
pub mod fiber;
#[allow(clippy::module_inception)] // fold artifact: walt-kernel/src/kernel.rs kept its name
pub mod kernel;
pub mod sample;

pub use decision::ReceiptDecision;
pub use fiber::{FiberDp, FiberIter};
pub use kernel::{Hidden, Kernel, KernelError, World, HIDDEN_SEATS};
pub use sample::{expected_distinct, SplitMix64};
