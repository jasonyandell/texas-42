//! The designated byte-frozen walk: hand 0, seat S1 (the bidder), whole
//! transcript, `WalkerConfig::fixture()`. walt's first receipt-shaped
//! artifact — exploratory tier, marked in its own header. Byte equality is
//! the assertion; regenerate via `cargo run --release --example
//! gen_fixtures`, never hand-edit.
//!
//! The pair is chosen so the artifact exercises everything at once: the
//! bidder leads trick 1, so its first decision point is exactly the dealt
//! kernel (ply 0, no prefix), the trick-1/2/3 fibers exceed the CI
//! threshold (recorded-seed samples, marked), and tricks 4-7 are
//! exhaustive.

use walt_core::Seat;
use walt_factory::{load_receipt, render_walk, walk_seat, WalkerConfig};

const FROZEN: &str = include_str!("data/walk_h0_S1.txt");

#[test]
fn designated_walk_is_byte_stable() {
    let receipt = load_receipt();
    let hand = &receipt.hands[0];
    assert_eq!(hand.bidder, Seat::S1);
    let config = WalkerConfig::fixture();
    let walk = walk_seat(hand, Seat::S1, &config);
    let text = render_walk(hand, &walk, &config);
    assert_eq!(text, FROZEN, "regenerate via the gen_fixtures example");
}
