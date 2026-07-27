//! Stage S10 (stretch) receipt binary (BRIEF_SLICE_02 §9 table S10).
//! Exchange tier: x- lines draw on ledger entry 001.

fn main() {
    print!("{}", rob_verify::s10::receipt());
}
