//! Stage S8 receipt binary (BRIEF_SLICE_02 §9 table S8).
//! Exchange tier: x- lines draw on ledger entry 002.

fn main() {
    print!("{}", rob_verify::s8::receipt());
}
