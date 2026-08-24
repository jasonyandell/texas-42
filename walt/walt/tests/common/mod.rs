//! Shared fixtures for the walt-kernel integration tests: the receipt, the
//! kernels built from it, and the exp5 probe corpus of fiber sizes.
//!
//! The exp5 numbers are exploratory tier -- an independent Python
//! implementation of the same construction, used here as regression pins, not
//! as authority.

#![allow(dead_code)]

use walt::kernel::{Kernel, World, HIDDEN_SEATS};
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt, ReceiptHand};
use walt::rules::replay::state_before_trick;
use walt::rules::DominoSet;

/// `(hand, trick, |Phi(C)|, unconstrained)` for every kernel exp5 reported.
/// Trick 6 is horizon 2, trick 3 is horizon 5.
pub const EXP5_FIBERS: &[(usize, usize, u128, u128)] = &[
    (0, 3, 756756, 756756),
    (0, 4, 34650, 34650),
    (0, 5, 1680, 1680),
    (0, 6, 90, 90),
    (1, 3, 756756, 756756),
    (1, 4, 34650, 34650),
    (1, 5, 1680, 1680),
    (1, 6, 90, 90),
    (2, 3, 756756, 756756),
    (2, 4, 23100, 34650),
    (2, 5, 1680, 1680),
    (2, 6, 36, 90),
    (3, 3, 756756, 756756),
    (3, 4, 11550, 34650),
    (3, 5, 200, 1680),
    (3, 6, 36, 90),
    (4, 3, 756756, 756756),
    (4, 4, 34650, 34650),
    (4, 5, 1680, 1680),
    (4, 6, 90, 90),
    (5, 3, 324324, 756756),
    (5, 4, 14700, 34650),
    (5, 5, 560, 1680),
    (5, 6, 27, 90),
    (6, 3, 756756, 756756),
    (6, 4, 34650, 34650),
    (6, 5, 1680, 1680),
    (6, 6, 90, 90),
    (7, 3, 504504, 756756),
    (7, 4, 23100, 34650),
    (7, 5, 1680, 1680),
    (7, 6, 90, 90),
    (8, 3, 59976, 756756),
    (8, 4, 1200, 34650),
    (8, 5, 92, 1680),
    (8, 6, 7, 90),
    (9, 3, 504504, 756756),
    (9, 4, 34650, 34650),
    (9, 5, 1680, 1680),
    (9, 6, 30, 90),
    (10, 3, 756756, 756756),
    (10, 4, 8820, 34650),
    (10, 5, 700, 1680),
    (10, 6, 19, 90),
    (11, 3, 64638, 756756),
    (11, 4, 23100, 34650),
    (11, 5, 1120, 1680),
    (11, 6, 36, 90),
    (12, 3, 756756, 756756),
    (12, 4, 34650, 34650),
    (12, 5, 1680, 1680),
    (12, 6, 6, 90),
];

pub fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// The deal the receipt actually held at the start of the trick.
pub fn true_world(kernel: &Kernel, hand: &ReceiptHand, trick_no: usize) -> World {
    let (hands, _) = state_before_trick(hand, trick_no).expect("a valid hand");
    let cells: [DominoSet; HIDDEN_SEATS] =
        core::array::from_fn(|i| hands[kernel.hidden()[i].seat.index()]);
    kernel.world(cells)
}

pub fn kernel_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (Kernel, &ReceiptHand) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    (kernel, hand)
}
