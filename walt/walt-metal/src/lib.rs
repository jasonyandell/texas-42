#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// `MTLCreateSystemDefaultDevice` requires CoreGraphics linkage. No
// CoreGraphics API surface is enabled or used.
use objc2_core_graphics as _;

mod abi;
#[allow(unsafe_code)]
mod bridge;
mod error;
mod runtime;

pub use abi::{OFFICIAL_ARITHMETIC_CASES, OPENING_ARENA_RECORDS, OPENING_SLOT_CAP};
pub use error::{CommandState, MetalError};
pub use runtime::{
    AcceptedMetalArithmeticNegativeV1, AcceptedMetalArithmeticV1, AcceptedMetalOpeningNegativeV1,
    AcceptedMetalOpeningTaskV1, AllocationHighWater, ArithmeticRunIntegrity, CommandEvent,
    CommandTerminal, DeviceProfile, MaximumSmokeReport, MetalRuntime, OpeningNegativeIntegrity,
    OpeningRunIntegrity, PipelineLimits,
};
