//! One-off determinism verification: the designated walk rendered under one
//! worker thread and under full parallelism must be byte-identical. The
//! structural argument (exact commutative folds) predicts it; this checks it.

use walt_core::Seat;
use walt_factory::{load_receipt, render_walk, walk_seat, WalkerConfig};

fn main() {
    let receipt = load_receipt();
    let hand = &receipt.hands[0];
    let mut serial = WalkerConfig::fixture();
    serial.threads = 1;
    let parallel = WalkerConfig::fixture();
    let a = render_walk(hand, &walk_seat(hand, Seat::S1, &serial), &serial);
    let b = render_walk(hand, &walk_seat(hand, Seat::S1, &parallel), &parallel);
    // The config line differs only if the configs did; they differ in
    // nothing rendered, so byte equality is the whole check.
    assert_eq!(a, b, "thread schedule leaked into the output");
    println!("thread-independence: byte-identical under 1 and N workers");
}
