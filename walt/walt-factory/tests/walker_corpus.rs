//! The walker over the whole receipt corpus — 13 hands x 4 seats — at the
//! declared CI config (`WalkerConfig::ci()`: tricks 3-7, exhaustive at or
//! below 40,000 worlds, recorded 64-draw samples above). Summary statistics
//! are frozen as walt-tier regression pins in `tests/data/ci_corpus_pins.txt`
//! (regenerate via `cargo run --release --example gen_fixtures`, never
//! hand-edit).
//!
//! Coverage caveat (recorded in S1, restated here because the pins inherit
//! it): the receipt corpus is pip-trump only — no P2, no doubles-trump, no
//! no-trump — so every number below validates the pip-trump path and
//! nothing else.
//!
//! Pins are evidence at a declared config, never axioms; a sampled basis in
//! any line is marked in the walk records themselves and graded `Sampled`
//! in every conflict emitted from it.

use walt_core::Seat;
use walt_factory::{corpus_pin_line, load_receipt, walk_seat, EvidenceBasis, Grade, WalkerConfig};
use walt_geom::qi;

const PINS: &str = include_str!("data/ci_corpus_pins.txt");

#[test]
fn corpus_summaries_match_the_pins() {
    let receipt = load_receipt();
    let config = WalkerConfig::ci();
    let mut lines = Vec::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let walk = walk_seat(hand, seat, &config);

            // Structural invariants, checked on every walk before pinning:
            for d in &walk.decisions {
                // An exhaustive basis solved exactly the fiber.
                if let EvidenceBasis::Exhaustive { worlds } = d.basis {
                    assert_eq!(worlds, d.fiber);
                }
                // Regret is nonnegative and zero exactly when the chosen
                // action attains the best expectation.
                assert!(d.regret >= qi(0));
                let best = d.values.iter().max().expect("actions");
                assert_eq!(d.regret == qi(0), d.values[d.chosen_index] == *best);
                // The dominance primitive partitions the evaluated worlds
                // and is antisymmetric under transposition.
                let n = d.actions.len();
                for i in 0..n {
                    for j in 0..n {
                        let t = d.triple(i, j);
                        let r = d.triple(j, i);
                        assert_eq!((t.gt, t.eq, t.lt), (r.lt, r.eq, r.gt));
                        if i == j {
                            assert_eq!(t.gt, 0);
                            assert_eq!(t.lt, 0);
                        }
                    }
                }
                // Worldwise dominance implies expectation order under the
                // full-support uniform weighting (exhaustive basis only).
                if matches!(d.basis, EvidenceBasis::Exhaustive { .. }) {
                    for (i, j) in &d.dominance {
                        assert!(d.values[*i] > d.values[*j]);
                    }
                    if d.chosen_dominated {
                        assert!(d.regret > qi(0));
                    }
                }
            }
            // Conflict grades quote their basis honestly.
            for c in walk.conflicts() {
                let d = walk
                    .decisions
                    .iter()
                    .find(|d| d.trick_no == c.trick_no)
                    .expect("conflict names a walked decision");
                match d.basis {
                    EvidenceBasis::Exhaustive { .. } => assert!(matches!(
                        c.grade,
                        Grade::WorldwiseDominance { .. } | Grade::ExactExpectation { .. }
                    )),
                    EvidenceBasis::Sampled { .. } => {
                        assert!(matches!(c.grade, Grade::Sampled { .. }));
                    }
                }
                assert!(!c.better.is_empty());
            }
            // A lost-verdict is only ever built on an exhaustive basis,
            // and it is exactly the live-world count hitting zero.
            if let Some(v) = &walk.lost_from {
                let d = walk
                    .decisions
                    .iter()
                    .find(|d| d.trick_no == v.trick_no)
                    .expect("verdict names a walked decision");
                assert!(matches!(d.basis, EvidenceBasis::Exhaustive { .. }));
                assert_eq!(d.live_worlds, 0);
                assert!(d.all_actions_lose());
            }

            lines.push(corpus_pin_line(hand, &walk));
        }
    }
    let got = lines.join("\n") + "\n";
    assert_eq!(got, PINS, "regenerate via the gen_fixtures example");
}
