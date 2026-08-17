//! Emit the canonical M0 semantic-table bytes on standard output.
//!
//! The byte stream, not Rust's in-memory container layout, is the table ABI.
//! Digest tools consume this example's output without an intermediate file.

use std::io::{self, Write};

use walt_gpu_spec::SemanticTables;

fn main() -> io::Result<()> {
    io::stdout().write_all(&SemanticTables::from_walt_core().canonical_bytes())
}
