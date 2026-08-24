//! Validation of the arbitrary-decision-point kernel constructor
//! (`ReceiptDecision`, v0.4 §2.1) against the receipt corpus: all 13 hands,
//! all 4 seats, all 7 decisions each — 364 decision points.
//!
//! Coverage caveat (S1): the receipt is pip-trump only, so these assertions
//! validate the pip-trump path; DT and NT kernels are exercised nowhere in
//! this corpus.
//!
//! Three assertions:
//! (a) at every trick-start point where the focal seat leads, the general
//!     constructor agrees exactly with `Kernel::from_receipt_trick`;
//! (b) the receipt's actual deal lies in the fiber at EVERY decision point —
//!     §2.1's fiber is the set of hidden remainders consistent with the
//!     actor-attributed legal public prefix, and the true world is always
//!     consistent with its own prefix;
//! (c) fiber counts are monotonically nonincreasing along each seat's seven
//!     decisions of one hand. Why the spec implies it: each later prefix
//!     extends an earlier one, and restoring the since-played tiles to the
//!     seats that (legally, replay-verified) played them injects the later
//!     fiber into the earlier one — every void constraint at the earlier
//!     point is a subset of the later constraints on still-hidden tiles, and
//!     a tile a seat later played cannot be excluded for that seat earlier,
//!     because the seat provably held it.

use walt::kernel::{Kernel, ReceiptDecision};
use walt::rules::receipt::{locate_verify_player, parse_file};
use walt::rules::replay::TRICKS_PER_HAND;
use walt::rules::Seat;

#[test]
fn decision_kernels_validate_across_the_whole_receipt() {
    let receipt = parse_file(&locate_verify_player().expect("receipt")).expect("parses");
    assert_eq!(receipt.hands.len(), 13);
    let mut decision_points = 0usize;
    let mut lead_agreements = 0usize;
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let decisions =
                ReceiptDecision::all_for_seat(hand, seat).expect("every decision constructs");
            assert_eq!(decisions.len(), TRICKS_PER_HAND);
            let mut previous_count: Option<u128> = None;
            for d in &decisions {
                decision_points += 1;
                assert_eq!(d.kernel.viewer(), seat);
                assert_eq!(d.prefix.len(), d.ply);
                assert_eq!(d.leader.plus(d.ply), seat);

                // (a) trick-start, viewer leading: exact agreement.
                if d.ply == 0 {
                    let baseline =
                        Kernel::from_receipt_trick(hand, d.trick_no).expect("trick-start kernel");
                    if baseline.viewer() == seat {
                        assert_eq!(d.kernel, baseline, "hand {} trick {}", hand.id, d.trick_no);
                        lead_agreements += 1;
                    }
                }

                // (b) the true world inhabits the fiber, at every point.
                let truth = d.true_world(hand).expect("replay");
                assert!(
                    d.kernel.contains(&truth),
                    "hand {} seat {} trick {}: true world outside the fiber",
                    hand.id,
                    seat,
                    d.trick_no
                );

                // (c) counts nonincreasing along the transcript.
                let count = d.kernel.count();
                assert!(count > 0);
                if let Some(prev) = previous_count {
                    assert!(
                        count <= prev,
                        "hand {} seat {} trick {}: fiber grew {prev} -> {count}",
                        hand.id,
                        seat,
                        d.trick_no
                    );
                }
                previous_count = Some(count);
            }
        }
    }
    assert_eq!(decision_points, 13 * 4 * 7);
    // Every trick has a leader, and that leader's ply-0 decision point is a
    // from_receipt_trick agreement: one per trick per hand.
    assert_eq!(lead_agreements, 13 * 7);
}
