use walt::policy_search::{
    self, partner_review,
    prices::{FiniteOracle, NodeKind, PriceWork},
    HashField, Search, Work,
};
use walt::rules::{legal_plays, Domino, DominoSet};

const MISSED: &str = "decl 3\nbid 30\nbidder 0\nseat 2\nhand 0 2 3 15 17 21 25\nplays 0 7 1 24 2 3 3 6 1 5 2 17 3 23 0 12 1 4 2 21 3 22 0 8 0 9 1 19 2 25 3 18 0 13 1 16\nseed 420600\n";

#[test]
fn requested_action_values_match_independent_finite_teacher_and_keep_memo_scope() {
    let mut checked = 0;
    for seed in 1..40 {
        let Ok(f) = policy_search::fixture(seed, "P6".parse().unwrap(), 2, None) else {
            continue;
        };
        if policy_search::State::from_root(&f.exercise.position)
            .success(&f.exercise.position)
            .is_some()
        {
            continue;
        }
        let worlds: Vec<_> = f.exercise.root.worlds().take(5).collect();
        let oracle = FiniteOracle::from_worlds(
            &f.exercise,
            &HashField,
            worlds.clone(),
            &mut PriceWork::new(1_000_000),
        )
        .unwrap();
        let NodeKind::Focal { actions } = &oracle.decision(&[]).unwrap().kind else {
            unreachable!()
        };
        let mut search = Search::new(&f.exercise, &HashField);
        for world in worlds {
            search.append(world).unwrap();
        }
        let full = search.solve(&mut Work::new(1_000_000)).unwrap();
        for action in actions {
            let selected: DominoSet = [action.tile].into_iter().collect();
            let values = search
                .compare_root(selected, &mut Work::new(1_000_000))
                .unwrap();
            assert_eq!(values[&action.tile], action.exact_makes);
            let again = search.solve(&mut Work::new(1_000_000)).unwrap();
            assert_eq!(again.makes, full.makes);
            assert_eq!(again.edges[0].0, full.edges[0].0);
        }
        checked += 1;
    }
    assert!(checked >= 5);
}

#[test]
fn refusal_returns_no_partial_comparison_and_resume_retains_correct_values() {
    let f = policy_search::request::from_text(MISSED).unwrap();
    let mut search = Search::new(&f.exercise, &HashField);
    for world in f.exercise.root.worlds().take(8) {
        search.append(world).unwrap();
    }
    let legal = legal_plays(
        f.exercise.position.decl,
        f.exercise.root.kernel().viewer_hand(),
        f.exercise.frame.led_context(),
    );
    assert!(search.compare_root(legal, &mut Work::new(1)).is_err());
    let resumed = search.compare_root(legal, &mut Work::new(100_000)).unwrap();
    assert_eq!(resumed.len(), legal.len());
    assert!(search
        .compare_root(DominoSet::EMPTY, &mut Work::new(100_000))
        .is_err());
    let illegal: DominoSet = [Domino::from_index(27).unwrap()].into_iter().collect();
    assert!(search
        .compare_root(illegal, &mut Work::new(100_000))
        .is_err());
}

#[test]
fn review_recovers_recorded_missed_offer_with_exact_comparison() {
    let f = policy_search::request::from_text(MISSED).unwrap();
    let r = partner_review::review(&f.exercise, Domino::from_index(2).unwrap()).unwrap();
    assert_eq!(r.status, "changed");
    assert_eq!(r.choice.index(), 15);
    assert_eq!(r.worlds, 210);
    assert_eq!(r.values[&Domino::from_index(2).unwrap()], 131);
    assert_eq!(r.values[&Domino::from_index(15).unwrap()], 153);
    assert_eq!(r.values.len(), 2, "only baseline and offers are compared");
}

#[test]
fn existing_offer_is_preserved_without_search() {
    let f = policy_search::request::from_text(MISSED).unwrap();
    let r = partner_review::review(&f.exercise, Domino::from_index(15).unwrap()).unwrap();
    assert_eq!(r.status, "inactive");
    assert_eq!(r.choice.index(), 15);
    assert_eq!(r.work.nodes, 0);
    assert!(r.values.is_empty());
}
