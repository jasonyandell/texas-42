//! Corpus driving: the 13-hand receipt as the walker's transcript corpus.
//!
//! Coverage caveat (recorded in S1, restated wherever the corpus is used):
//! `rob/receipts/verify_player.txt` is pip-trump only — no P2, no
//! doubles-trump, no no-trump — so every corpus statistic downstream
//! inherits that scope.

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::Seat;

use crate::walker::{walk_seat, HandSeatWalk, WalkerConfig};

/// Parses the rob self-play receipt (read-only ground truth for the rules
/// bridge; walker inputs, never walker authority).
pub fn load_receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// Walks every (hand, seat) pair of the receipt under one config, hand-major
/// then seat order.
pub fn walk_corpus(receipt: &Receipt, config: &WalkerConfig) -> Vec<HandSeatWalk> {
    let mut out = Vec::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            out.push(walk_seat(hand, seat, config));
        }
    }
    out
}
