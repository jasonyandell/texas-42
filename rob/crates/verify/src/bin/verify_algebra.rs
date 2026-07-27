//! Stage S1 receipt binary (BRIEF §8 table S1, §9).
//!
//! Prints the deterministic S1 receipt; CI diffs the output byte-for-byte
//! against `rob/receipts/verify_algebra.txt` (INV-5).

fn main() {
    print!("{}", rob_verify::s1::receipt());
}
