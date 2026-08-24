//! The ground-truth bridge: all 13 hands of rob's `verify_player.txt`
//! re-derived from walt-core's rules alone (READ-ONLY on the receipt).

use walt::rules::receipt::{locate_verify_player, parse_file};
use walt::rules::replay::{replay_receipt, HAND_TOTAL_POINTS, TRICKS_PER_HAND};
use walt::rules::{Decl, DeclClass, DominoSet, Seat};

fn receipt() -> walt::rules::receipt::Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

#[test]
fn all_thirteen_hands_replay() {
    let r = receipt();
    assert_eq!(r.hands.len(), 13);
    assert_eq!(r.hands_played, 13);
    let replayed = replay_receipt(&r).expect("every hand replays under the rules");
    assert_eq!(replayed.hands.len(), 13);
    assert_eq!(replayed.final_marks, r.final_marks);
    assert_eq!(replayed.final_marks, [7, 6]);

    for (hand, out) in r.hands.iter().zip(&replayed.hands) {
        assert_eq!(hand.tricks.len(), TRICKS_PER_HAND);
        assert_eq!(out.trick_winners.len(), TRICKS_PER_HAND);
        assert_eq!(out.team_points[0] + out.team_points[1], HAND_TOTAL_POINTS);
        assert_eq!(out.team_tricks[0] + out.team_tricks[1], 7);
        let dealt: DominoSet = out
            .deal
            .iter()
            .fold(DominoSet::EMPTY, |acc, h| acc.union(*h));
        assert_eq!(dealt, DominoSet::FULL);
        for h in out.deal {
            assert_eq!(h.len(), 7);
        }
        assert_eq!(hand.tricks[0].plays[0].0, hand.bidder, "the bidder leads");
    }
}

/// Honest coverage: the receipt corpus exercises pip trump only, so
/// doubles-trump and no-trump remain unexercised by this bridge. Their
/// behaviour is pinned only by the exhaustive structural tests.
#[test]
fn receipt_declaration_coverage_is_pip_trump_only() {
    let r = receipt();
    let mut pip_trump = 0;
    for hand in &r.hands {
        match hand.decl.class() {
            DeclClass::PipTrump => pip_trump += 1,
            other => panic!("unexpected declaration class {other:?} in the receipt"),
        }
    }
    assert_eq!(pip_trump, 13);
    let mut by_decl = std::collections::BTreeMap::new();
    for hand in &r.hands {
        *by_decl.entry(hand.decl).or_insert(0usize) += 1;
    }
    // The receipt's own non-normative statistics block.
    let pip = |v: u8| Decl::PipTrump(walt::rules::Pip::new(v).expect("pip"));
    assert_eq!(by_decl.get(&pip(0)), Some(&2));
    assert_eq!(by_decl.get(&pip(1)), Some(&1));
    assert_eq!(by_decl.get(&pip(3)), Some(&1));
    assert_eq!(by_decl.get(&pip(4)), Some(&3));
    assert_eq!(by_decl.get(&pip(5)), Some(&4));
    assert_eq!(by_decl.get(&pip(6)), Some(&2));
    assert_eq!(by_decl.get(&pip(2)), None);
}

/// Derived views agree with the replay at every trick boundary.
#[test]
fn state_and_voids_before_each_trick_are_consistent() {
    let r = receipt();
    for hand in &r.hands {
        for trick_no in 1..=TRICKS_PER_HAND {
            let (hands, leader) =
                walt::rules::replay::state_before_trick(hand, trick_no).expect("a valid hand");
            let remaining = 8 - trick_no;
            for (i, h) in hands.iter().enumerate() {
                assert_eq!(h.len(), remaining, "hand {} trick {trick_no} S{i}", hand.id);
            }
            assert_eq!(leader, hand.tricks[trick_no - 1].plays[0].0);

            // The true world satisfies every void the history has revealed.
            let voids = walt::rules::replay::voids_before_trick(hand, trick_no);
            for seat in Seat::ALL {
                for q in voids[seat.index()].iter() {
                    for d in hands[seat.index()].iter() {
                        assert!(
                            !hand.decl.follows(d, q),
                            "hand {} trick {trick_no}: {seat} shown void in {q} but holds {d}",
                            hand.id
                        );
                    }
                }
            }
        }
        let (empty, _) = walt::rules::replay::state_before_trick(hand, TRICKS_PER_HAND + 1)
            .expect("a valid hand");
        for h in empty {
            assert!(h.is_empty());
        }
    }
}
