//! Stage S3 receipt binary (BRIEF §8 table S3, §9).
//!
//! Prints the deterministic S3 receipt; CI diffs the output byte-for-byte
//! against `rob/receipts/verify_support.txt` (INV-5).

fn main() {
    print!("{}", rob_verify::s3::receipt());
}
