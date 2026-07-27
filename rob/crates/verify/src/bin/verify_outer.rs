//! Stage S7 receipt binary (BRIEF_SLICE_02 §9 table S7).
//! Exchange tier: x- lines draw on ledger entries 002 and 005.

fn main() {
    print!("{}", rob_verify::s7::receipt());
}
