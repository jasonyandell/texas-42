//! Pure codec for the freeze-56 M2 Metal parity receipt.
//!
//! This crate deliberately has no Metal, Objective-C, filesystem, clock, or
//! process dependency.  It turns already-observed evidence into the one closed
//! byte representation fixed by `GPU-NATIVE-TRICK1-M2.md` and rejects every
//! noncanonical representation while parsing.

#![forbid(unsafe_code)]

mod receipt;
mod records;
mod transport;
mod wire;

pub use receipt::*;
pub use records::*;
pub use transport::*;
pub use wire::*;

/// SHA-256 of the binding M2 contract audited for this codec.
pub const CONTRACT_SHA256: Digest = [
    0xaa, 0xcb, 0x6d, 0xf5, 0xe9, 0x10, 0x6b, 0x3b, 0x6b, 0xf0, 0x0c, 0xcf, 0xb4, 0x96, 0xc7, 0x1f,
    0x76, 0x2c, 0x0f, 0xb4, 0x64, 0x4c, 0x13, 0xa1, 0x7f, 0x76, 0xd2, 0xac, 0x2f, 0x03, 0x26, 0xe3,
];

/// Exact byte length of the binding contract named by [`CONTRACT_SHA256`].
pub const CONTRACT_BYTES: u64 = 46_133;
