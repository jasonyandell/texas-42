//! Stage S9 receipt binary (BRIEF_SLICE_02 §9 table S9).
//! Exchange tier: x- lines draw on ledger entry 004.

fn main() {
    print!("{}", rob_verify::s9::receipt());
}
