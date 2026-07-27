//! Stage S4 receipt binary (BRIEF §8 table S4, §9).
//!
//! Prints the deterministic S4 receipt; CI diffs the output byte-for-byte
//! against `rob/receipts/verify_normal_form.txt` (INV-5).

fn main() {
    print!("{}", rob_verify::s4::receipt());
}
