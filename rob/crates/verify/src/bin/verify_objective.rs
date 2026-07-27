//! Stage S2 receipt binary (BRIEF §8 table S2, §9).
//!
//! Prints the deterministic S2 receipt; CI diffs the output byte-for-byte
//! against `rob/receipts/verify_objective.txt` (INV-5).

fn main() {
    print!("{}", rob_verify::s2::receipt());
}
