//! Exact synthetic schedules: independent expected work and winners.
use num_rational::BigRational;
use walt::solver::selection::{self, Batch, Rule};

fn q(n: i32, d: i32) -> BigRational {
    BigRational::new(n.into(), d.into())
}

#[test]
fn fixed_ties_use_tile_order_without_extra_sampling() {
    let mut calls = 0;
    let result = selection::select(Rule::Fixed, &[2, 9], true, 40, |tiles, n, kind| {
        calls += 1;
        assert_eq!((n, kind), (40, Batch::Bundle));
        Ok::<_, ()>(tiles.iter().map(|&t| (t, q(1, 1))).collect())
    })
    .unwrap();
    assert_eq!((result.choice, result.worlds, calls), (2, 40, 1));
}

#[test]
fn refinement_reconsiders_formerly_lower_candidates() {
    let mut calls = vec![];
    let result = selection::select(Rule::Refine, &[0, 1, 2], true, 2, |tiles, n, _| {
        calls.push((tiles.to_vec(), n));
        Ok::<_, ()>(
            tiles
                .iter()
                .map(|&t| {
                    (
                        t,
                        if n == 2 {
                            if t == 2 {
                                q(1, 2)
                            } else {
                                q(1, 1)
                            }
                        } else {
                            q(0, 1)
                        },
                    )
                })
                .collect(),
        )
    })
    .unwrap();
    assert_eq!(calls, vec![(vec![0, 1, 2], 2), (vec![0, 1], 8)]);
    assert_eq!(result.choice, 2);
}

#[test]
fn unresolved_ties_exhaust_exactly_the_historical_fresh_bundles() {
    let mut sizes = vec![];
    let result = selection::select(Rule::Refine, &[3, 7], false, 40, |tiles, n, _| {
        sizes.push(n);
        Ok::<_, ()>(tiles.iter().map(|&t| (t, q(0, 1))).collect())
    })
    .unwrap();
    assert_eq!(sizes, [40, 160, 640]);
    assert_eq!((result.choice, result.worlds), (3, 840));
}

#[test]
fn racing_uses_paired_blocks_and_both_objectives() {
    for maximize in [true, false] {
        let mut sizes = vec![];
        let result =
            selection::select(Rule::RaceRefine, &[1, 8], maximize, 40, |tiles, n, kind| {
                sizes.push(n);
                assert_eq!(kind, Batch::Block);
                assert_eq!(tiles, [1, 8]);
                Ok::<_, ()>(vec![(1, q(0, 1)), (8, q(1, 1))])
            })
            .unwrap();
        // Seven unanimous pivotal blocks: 2^-7 meets 1/128.
        assert_eq!(sizes, [8; 7]);
        assert_eq!(result.worlds, 56);
        assert_eq!(result.choice, if maximize { 8 } else { 1 });
    }
}

#[test]
fn saturation_race_refines_only_tied_survivors() {
    let mut bundle = vec![];
    let result = selection::select(Rule::RaceRefine, &[1, 2, 3], true, 40, |tiles, n, kind| {
        if kind == Batch::Bundle {
            bundle.push((tiles.to_vec(), n));
        }
        Ok::<_, ()>(
            tiles
                .iter()
                .map(|&t| {
                    (
                        t,
                        if t == 1 {
                            q(0, 1)
                        } else if kind == Batch::Block {
                            q(1, 1)
                        } else if t == 2 {
                            q(1, 2)
                        } else {
                            q(3, 4)
                        },
                    )
                })
                .collect(),
        )
    })
    .unwrap();
    assert_eq!(bundle, [(vec![2, 3], 40)]);
    assert_eq!((result.choice, result.worlds), (3, 120));
}

#[test]
fn refusal_discards_previous_completed_rounds() {
    let mut calls = 0;
    let result = selection::select(Rule::Refine, &[1, 2], true, 2, |tiles, _, _| {
        calls += 1;
        if calls == 2 {
            Err("interrupted")
        } else {
            Ok(tiles.iter().map(|&t| (t, q(1, 1))).collect())
        }
    });
    assert_eq!(result.err(), Some("interrupted"));
}

#[test]
fn forced_race_never_requests_an_evaluation() {
    let result = selection::select(
        Rule::RaceRefine,
        &[4],
        true,
        40,
        |_, _, _| -> Result<_, ()> {
            panic!("forced move must not sample");
        },
    )
    .unwrap();
    assert_eq!((result.choice, result.worlds), (4, 0));
}
