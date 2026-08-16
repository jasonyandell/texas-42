//! Regenerates the walker's frozen fixtures (never hand-edit them):
//!   - `tests/data/ci_corpus_pins.txt`: one summary line per (hand, seat)
//!     under `WalkerConfig::ci()`;
//!   - `tests/data/walk_h0_S1.txt`: the designated full-transcript walk
//!     (hand 0, seat S1 — the bidder) under `WalkerConfig::fixture()`.
//!
//! Both are walt-tier regression pins at declared configs, exploratory tier.

use std::path::Path;

use walt_core::Seat;
use walt_factory::{load_receipt, render_walk, walk_seat, WalkerConfig};

fn main() {
    let receipt = load_receipt();
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    std::fs::create_dir_all(&data).expect("tests/data");

    let config = WalkerConfig::ci();
    let mut pins = String::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let walk = walk_seat(hand, seat, &config);
            pins.push_str(&walt_factory::corpus_pin_line(hand, &walk));
            pins.push('\n');
        }
    }
    std::fs::write(data.join("ci_corpus_pins.txt"), &pins).expect("write pins");

    let hand = &receipt.hands[0];
    let config = WalkerConfig::fixture();
    let walk = walk_seat(hand, Seat::S1, &config);
    let text = render_walk(hand, &walk, &config);
    std::fs::write(data.join("walk_h0_S1.txt"), &text).expect("write fixture");
    println!("fixtures regenerated");
}
