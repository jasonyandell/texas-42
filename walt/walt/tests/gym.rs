//! The gym's own-hand/public-history bridge, Scheme descriptor, exact values,
//! and the zero-completion regression first exposed by a third-hand offer.
use walt::gym::{self, ExerciseRoot, GymField};
use walt::rules::{Domino, DominoSet, Seat};
use walt::scheme::{Budget, Fix};
use walt::solver::adaptive::FixedPreference;
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};

const HOLD_HAND: [usize; 7] = [4, 14, 17, 20, 23, 24, 27];
const HOLD_HISTORY: [usize; 44] = [
    0, 2, 1, 16, 2, 4, 3, 11, 0, 6, 1, 3, 2, 17, 3, 21, 3, 22, 0, 9, 1, 19, 2, 23, 2, 14, 3, 25, 0,
    18, 1, 13, 2, 27, 3, 26, 0, 1, 1, 7, 0, 10, 1, 12,
];

fn hand(ids: &[usize]) -> DominoSet {
    ids.iter()
        .map(|&i| Domino::from_index(i).unwrap())
        .collect()
}
fn history(ids: &[usize], rotate: usize) -> Vec<(Seat, Domino)> {
    ids.chunks_exact(2)
        .map(|p| {
            (
                Seat::from_index((p[0] + rotate) % 4).unwrap(),
                Domino::from_index(p[1]).unwrap(),
            )
        })
        .collect()
}
fn hold(rotate: usize) -> ExerciseRoot {
    gym::from_request(
        walt::solver::decl_of(0),
        Seat::S0.plus(rotate),
        Seat::S2.plus(rotate),
        hand(&HOLD_HAND),
        &history(&HOLD_HISTORY, rotate),
    )
    .unwrap()
}

#[test]
fn public_bridge_and_offer_query_are_seat_covariant_and_world_invariant() {
    let query = gym::OFFER_QUERY
        .parse::<Fix>()
        .unwrap()
        .compile(&gym::registry())
        .unwrap();
    for r in 0..4 {
        let ex = hold(r);
        assert_eq!(ex.root.count(), 6);
        assert_eq!(ex.position.leader, Seat::S0.plus(r));
        assert_eq!(ex.root.kernel().viewer(), Seat::S2.plus(r));
        assert_eq!(ex.position.banked[Seat::S0.plus(r).team().index()], 19);
        assert_eq!(ex.root.kernel().viewer_hand(), hand(&[20, 24]));
        assert_eq!(
            ex.frame.prefix(),
            &[
                Domino::from_index(10).unwrap(),
                Domino::from_index(12).unwrap()
            ]
        );
        let answers: Vec<_> = ex
            .root
            .worlds()
            .map(|w| {
                query
                    .evaluate(&ex.frame, &w, &mut Budget::new(10000))
                    .unwrap()
            })
            .collect();
        assert_eq!(answers[0].len(), 1);
        assert!(answers.iter().all(|a| *a == answers[0]));
    }
}

#[test]
fn hold_count_has_strict_make_advantage_and_complete_replay_keys() {
    let ex = hold(0);
    let result = gym::assess(&ex, &GymField::new(Seat::S2, 40), 6).unwrap();
    assert_eq!(result.offers, [20]);
    assert_eq!(result.best, [24]);
    assert_eq!(
        result
            .actions
            .iter()
            .map(|a| (a.tile, a.success_mass))
            .collect::<Vec<_>>(),
        [(20, 2), (24, 4)]
    );
    for action in result.actions {
        assert_eq!(action.traces.len(), 6);
        assert_eq!(action.bins.iter().sum::<u128>(), 6);
        assert!(action
            .traces
            .iter()
            .all(|t| t.plays.len() == 12 && t.banked.iter().sum::<u32>() == 42));
    }
    assert!(gym::assess(&ex, &GymField::new(Seat::S2, 40), 5).is_err());
}

#[test]
fn third_hand_offer_regression_never_queries_an_uncompletable_partner_hand() {
    let plays = [
        0, 5, 1, 3, 2, 17, 3, 22, 0, 23, 1, 6, 2, 8, 3, 9, 0, 12, 1, 27, 2, 10, 3, 1, 0, 21, 1, 26,
        2, 24, 3, 0, 1, 20, 2, 2,
    ];
    let ex = gym::from_request(
        walt::solver::decl_of(2),
        Seat::S0,
        Seat::S3,
        hand(&[0, 1, 7, 9, 15, 18, 22]),
        &history(&plays, 0),
    )
    .unwrap();
    let result = gym::assess(&ex, &GymField::new(Seat::S3, 40), 36).unwrap();
    assert_eq!(result.worlds, 36);
    assert_eq!(result.best, [15]);
    assert_eq!(
        result
            .actions
            .iter()
            .map(|a| (a.tile, a.success_mass))
            .collect::<Vec<_>>(),
        [(15, 30), (18, 8)]
    );
}

#[test]
fn malformed_or_private_inconsistent_history_is_rejected() {
    let decl = walt::solver::decl_of(0);
    let mut plays = history(&HOLD_HISTORY, 0);
    plays[0].0 = Seat::S1;
    assert!(gym::from_request(decl, Seat::S0, Seat::S2, hand(&HOLD_HAND), &plays).is_err());
    assert!(gym::from_request(
        decl,
        Seat::S0,
        Seat::S3,
        hand(&HOLD_HAND),
        &history(&HOLD_HISTORY, 0)
    )
    .is_err());
    assert!(gym::from_request(
        decl,
        Seat::S0,
        Seat::S2,
        hand(&HOLD_HAND[..6]),
        &history(&HOLD_HISTORY, 0)
    )
    .is_err());
    plays = history(&HOLD_HISTORY, 0);
    plays[0].1 = Domino::from_index(4).unwrap(); // another seat played our tile
    assert!(gym::from_request(decl, Seat::S0, Seat::S2, hand(&HOLD_HAND), &plays).is_err());
}

#[test]
fn positive_support_pruning_preserves_factor_weights_not_marginal_weights() {
    let ex = hold(0);
    let field = FixedPreference::lowest_first("gym-weighted-regression");
    let mut belief = FactorBelief::uniform_root(&ex.root, &ex.position, &field)
        .focal_play(Domino::from_index(20).unwrap());
    let actor = belief.seat_to_move();
    let factor = belief.factors().iter().find(|f| f.seat() == actor).unwrap();
    belief = belief.with_factor_table(
        actor,
        factor.support().into_iter().map(|(h, _)| (h, 2)).collect(),
    );
    let other = belief.factors().iter().find(|f| f.seat() != actor).unwrap();
    belief = belief.with_factor_table(
        other.seat(),
        other.support().into_iter().map(|(h, _)| (h, 3)).collect(),
    );
    let oracle = SupportOracle;
    assert_eq!(oracle.mass(&belief), 6 * 2 * 3);
    for (tile, mass) in oracle.branch_masses(&belief, &field) {
        let child = oracle.condition(&belief, tile, &field);
        assert_eq!(oracle.mass(&child), mass);
        for (before, after) in belief.factors().iter().zip(child.factors()) {
            if after.seat() == actor {
                assert!(after.support().iter().all(|(_, weight)| *weight == 2));
            } else {
                assert_eq!(before, after);
            }
        }
    }
}

#[test]
fn ordinary_scheme_offer_matches_the_original_predicate_in_every_rotation() {
    let source = include_str!("../../gym/queries/offer-count.scheme");
    for rotate in 0..4 {
        let ex = hold(rotate);
        let found = gym::match_query(&ex, source, 1, 100_000).unwrap();
        assert!(found.public);
        assert_eq!(found.presence, [(20, 6)]);
        let old = gym::match_query(&ex, gym::OFFER_QUERY, 1, 100_000).unwrap();
        assert_eq!(found.presence, old.presence);
        assert_ne!(found.identity, old.identity); // expression identity, not equivalence
    }
}

#[test]
fn hidden_relation_reports_full_fiber_presence_without_conditioning_it() {
    let ex = hold(0);
    let hidden = ex
        .root
        .worlds()
        .next()
        .unwrap()
        .hand(Seat::S0)
        .iter()
        .next()
        .unwrap();
    let source = format!("(fix (roles (domino action)) (out action) (case (own-legal action) (holds S0 {hidden:?})))");
    let found = gym::match_query(&ex, &source, 6, 100_000).unwrap();
    assert!(!found.public);
    let count = ex
        .root
        .worlds()
        .filter(|w| w.hand(Seat::S0).contains(hidden))
        .count() as u128;
    assert!(count > 0 && count < 6);
    assert_eq!(found.worlds, 6);
    assert_eq!(found.presence, [(20, count), (24, count)]);
    assert_eq!(ex.root.count(), 6);
    assert!(gym::match_query(&ex, &source, 5, 100_000).is_err());
}

#[test]
fn query_contract_budget_and_source_errors_refuse_partial_matches() {
    let ex = hold(0);
    for bad in [
        "(fix (roles (chair me)) (out me) (case (viewer me)))",
        "(fix (roles (domino action)) (out action) (case (tile action 0-0)))",
    ] {
        assert!(gym::match_query(&ex, bad, 6, 100_000).is_err());
    }
    assert!(gym::match_query(
        &ex,
        include_str!("../../gym/queries/offer-count.scheme"),
        6,
        1
    )
    .is_err());
    for source in [
        include_str!("../../gym/queries/overtake-partner.scheme"),
        include_str!("../../gym/queries/lead-to-partner-boss.scheme"),
    ] {
        assert!(gym::compile_query(source).is_ok());
    }
}
