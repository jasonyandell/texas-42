use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt::policy_search::prices::{
    event_scheme_source, FiniteOracle, NodeKind, PriceProgram, PriceWork, PriorAuthority,
    EVENT_COUNT,
};
use walt::policy_search::{fixture, HashField, Search, State, Work};
use walt::rules::{Decl, Domino};
use walt::scheme::{step_frame, step_world, Budget, Fix, Frame, ObservedPlay, PlayClass};
use walt::solver::adaptive::FixedPreference;

fn small(tiles: usize) -> walt::policy_search::Fixture {
    let decl = "P6".parse::<Decl>().unwrap();
    (1..200)
        .find_map(|seed| fixture(seed, decl, tiles, None).ok())
        .expect("a deterministic legal small fixture")
}

fn capped_exact(tiles: usize, cap: u128) -> walt::policy_search::Fixture {
    let decl = "P6".parse::<Decl>().unwrap();
    (1..10_000)
        .find_map(|seed| {
            let fixture = fixture(seed, decl, tiles, None).ok()?;
            let legal = walt::rules::legal_plays(
                fixture.exercise.position.decl,
                fixture.exercise.root.kernel().viewer_hand(),
                fixture.exercise.frame.led_context(),
            );
            (fixture.exercise.root.count() <= cap
                && legal.len() >= 2
                && State::from_root(&fixture.exercise.position)
                    .success(&fixture.exercise.position)
                    .is_none())
            .then_some(fixture)
        })
        .expect("a capped exact root with a real choice")
}

fn successor_at(
    fixture: &walt::policy_search::Fixture,
    history: &[Domino],
    initial: walt::kernel::World,
) -> (Frame, walt::kernel::World) {
    let mut frame = fixture.exercise.frame.clone();
    let mut world = initial;
    let mut state = State::from_root(&fixture.exercise.position);
    let mut budget = Budget::new(1_000_000);
    for &tile in history {
        let actor = state.actor();
        let class = match frame.led_context() {
            None => PlayClass::Lead,
            Some(q) if frame.kernel().decl().follows(tile, q) => PlayClass::Follow(q),
            Some(q) => PlayClass::Slough(q),
        };
        let observation = ObservedPlay { actor, tile, class };
        world = step_world(&frame, &world, observation).unwrap();
        frame = step_frame(&frame, observation, &mut budget).unwrap();
        state = state.step(fixture.exercise.position.decl, tile);
    }
    (frame, world)
}

#[test]
fn exact_tree_matches_independent_search_and_retains_all_root_actions() {
    let fixture = capped_exact(3, 512);
    let worlds = fixture.exercise.root.worlds().take(8).collect::<Vec<_>>();
    let oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        worlds.clone(),
        &mut PriceWork::new(20_000_000),
    )
    .unwrap();
    assert_eq!(
        oracle.authority(),
        PriorAuthority::FiniteEmpirical { samples: 8 }
    );
    let root = oracle.decision(&[]).unwrap();
    let NodeKind::Focal { actions } = &root.kind else {
        panic!("root is a focal decision")
    };
    assert!(actions.len() >= 2);

    let mut search = Search::new(&fixture.exercise, &HashField);
    for world in worlds {
        search.append(world).unwrap();
    }
    let expected = search.solve(&mut Work::new(20_000_000)).unwrap().makes;
    assert_eq!(actions.iter().map(|a| a.exact_makes).max(), Some(expected));
    assert_eq!(root.exact_makes, expected);
}

#[test]
fn native_event_basis_is_extensionally_equal_to_its_scheme_fixes() {
    let fixture = small(3);
    let worlds = fixture.exercise.root.worlds().take(6).collect::<Vec<_>>();
    let oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        worlds.clone(),
        &mut PriceWork::new(20_000_000),
    )
    .unwrap();
    let registry = walt::gym::registry();
    for node in oracle.nodes() {
        let NodeKind::Focal { actions } = &node.kind else {
            continue;
        };
        for action in actions {
            let queries = (0..EVENT_COUNT)
                .map(|event| {
                    event_scheme_source(event, action.tile)
                        .unwrap()
                        .parse::<Fix>()
                        .unwrap()
                        .compile(&registry)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            for &world_id in &node.active_ids {
                let native = oracle
                    .event_values(node.id, action.tile, world_id, &mut PriceWork::new(10_000))
                    .unwrap();
                let (frame, world) =
                    successor_at(&fixture, &node.state.history, oracle.worlds()[world_id]);
                for (event, native_value) in native.iter().enumerate().take(EVENT_COUNT) {
                    let answers = queries[event]
                        .evaluate(&frame, &world, &mut Budget::new(1_000_000))
                        .unwrap();
                    assert_eq!(!answers.is_empty(), *native_value, "event {event}");
                }
            }
        }
    }
}

#[test]
fn every_full_history_center_is_exact_and_priced_upper_dominates_lawful_q() {
    let fixture = capped_exact(3, 512);
    let oracle = FiniteOracle::exact_root_fiber(
        &fixture.exercise,
        &HashField,
        512,
        &mut PriceWork::new(50_000_000),
    )
    .unwrap();
    assert!(matches!(
        oracle.authority(),
        PriorAuthority::ExactRootFiber { .. }
    ));
    let price = PriceProgram::new([0, 1, -1, 1]);
    let centers = oracle
        .centers(&price, &mut PriceWork::new(50_000_000))
        .unwrap();
    oracle
        .audit_centers(&price, &centers, &mut PriceWork::new(50_000_000))
        .unwrap();
    let lawful = FixedPreference::lowest_first("price-test-lowest-v1");
    let rows = oracle
        .action_values_for_history(
            &[],
            Some((&price, &centers)),
            Some(&lawful),
            &mut PriceWork::new(50_000_000),
        )
        .unwrap();
    for row in &rows {
        let lower = row.lawful_program_q.as_ref().unwrap();
        let priced = row.priced_upper.as_ref().unwrap();
        assert!(lower <= &row.exact_lawful_q);
        assert!(&row.exact_lawful_q <= priced);
        assert!(priced <= &row.perfect_information_upper);
        assert!(priced <= &BigRational::one());
    }
    let chosen = rows[0].tile;
    let cost = walt::policy_search::prices::ActionBounds::cost_upper(&rows, chosen).unwrap();
    assert!(cost >= BigRational::zero() && cost <= BigRational::one());
}

#[test]
fn fixed_root_action_has_no_root_only_price() {
    let fixture = small(1);
    let oracle = FiniteOracle::exact_root_fiber(
        &fixture.exercise,
        &HashField,
        512,
        &mut PriceWork::new(5_000_000),
    )
    .unwrap();
    let price = PriceProgram::new([1, -1, 1, -1]);
    let centers = oracle
        .centers(&price, &mut PriceWork::new(5_000_000))
        .unwrap();
    let rows = oracle
        .action_values_for_history(
            &[],
            Some((&price, &centers)),
            None,
            &mut PriceWork::new(5_000_000),
        )
        .unwrap();
    for row in rows {
        assert_eq!(row.priced_upper.unwrap(), row.perfect_information_upper);
    }
}

#[test]
fn duplicate_worlds_keep_multiplicity_in_information_states_and_centers() {
    let fixture = small(3);
    let candidates = fixture.exercise.root.worlds().take(64).collect::<Vec<_>>();
    let probe = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        candidates.clone(),
        &mut PriceWork::new(20_000_000),
    )
    .unwrap();
    let root = probe.decision(&[]).unwrap();
    let NodeKind::Focal { actions } = &root.kind else {
        panic!("root is focal")
    };
    let mut witness = None;
    'outer: for action in actions {
        for event in 0..EVENT_COUNT {
            let mut zero = None;
            let mut one = None;
            for &world_id in &root.active_ids {
                let value = probe
                    .event_values(root.id, action.tile, world_id, &mut PriceWork::new(100))
                    .unwrap()[event];
                if value {
                    one = Some(world_id);
                } else {
                    zero = Some(world_id);
                }
            }
            if let (Some(zero), Some(one)) = (zero, one) {
                witness = Some((action.tile, event, candidates[zero], candidates[one]));
                break 'outer;
            }
        }
    }
    let (action, event, zero, one) = witness.expect("a nonconstant root event");
    let oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        vec![zero, one, one],
        &mut PriceWork::new(5_000_000),
    )
    .unwrap();
    assert_eq!(oracle.decision(&[]).unwrap().active_ids, vec![0, 1, 2]);
    let price = PriceProgram::new([1, 1, 1, 1]);
    let centers = oracle
        .centers(&price, &mut PriceWork::new(5_000_000))
        .unwrap();
    assert_eq!(
        centers.center(oracle.decision(&[]).unwrap().id, action, event),
        Some(&BigRational::new(2.into(), 3.into()))
    );
    oracle
        .audit_centers(&price, &centers, &mut PriceWork::new(5_000_000))
        .unwrap();
}

#[test]
fn mismatched_centers_and_exhausted_budgets_refuse_without_a_bound() {
    let fixture = small(3);
    let worlds = fixture.exercise.root.worlds().take(4).collect::<Vec<_>>();
    let oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        worlds.clone(),
        &mut PriceWork::new(10_000_000),
    )
    .unwrap();
    let first = PriceProgram::new([1, 0, 0, 0]);
    let second = PriceProgram::new([0, 1, 0, 0]);
    let centers = oracle
        .centers(&first, &mut PriceWork::new(10_000_000))
        .unwrap();
    let mismatch = oracle.action_values_for_history(
        &[],
        Some((&second, &centers)),
        None,
        &mut PriceWork::new(10_000_000),
    );
    assert!(mismatch
        .unwrap_err()
        .contains("wrong root, field, prior, basis, or coefficients"));

    let other_oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        worlds,
        &mut PriceWork::new(10_000_000),
    )
    .unwrap();
    let foreign = other_oracle.action_values_for_history(
        &[],
        Some((&first, &centers)),
        None,
        &mut PriceWork::new(10_000_000),
    );
    assert!(foreign
        .unwrap_err()
        .contains("wrong root, field, prior, basis, or coefficients"));

    let centers = oracle
        .centers(&first, &mut PriceWork::new(10_000_000))
        .unwrap();
    let refusal = oracle.action_values_for_history(
        &[],
        Some((&first, &centers)),
        None,
        &mut PriceWork::new(1),
    );
    assert!(refusal.unwrap_err().contains("budget exhausted"));

    let mut tiny = PriceWork::new(1);
    assert!(PriceProgram::ternary_library(&mut tiny).is_err());
    assert_eq!(tiny.coefficient_candidates, 1);
}

#[test]
fn malformed_history_is_refused_instead_of_rebased() {
    let fixture = small(3);
    let oracle = FiniteOracle::from_worlds(
        &fixture.exercise,
        &HashField,
        fixture.exercise.root.worlds().take(2).collect(),
        &mut PriceWork::new(5_000_000),
    )
    .unwrap();
    let impossible = Domino::from_index(27).unwrap();
    assert!(oracle.decision(&[impossible]).is_err());
}

#[test]
fn pooling_observably_different_states_is_an_invalid_centering_counterexample() {
    // Public X is uniform and the lawful action A=X succeeds surely.  If an
    // examiner illegally pools the two observable X states, each action's
    // reward center is 1/2 and the adjusted maximum is only 1/2.  Correct
    // full-information-state centering makes the public-only price zero.
    let half = BigRational::new(1.into(), 2.into());
    let lawful = BigRational::one();
    let pooled_purported_upper = [false, true]
        .into_iter()
        .map(|x| {
            [false, true]
                .into_iter()
                .map(|action| {
                    let reward = BigRational::from_integer((action == x).into());
                    reward.clone() - (reward - half.clone())
                })
                .max()
                .unwrap()
        })
        .sum::<BigRational>()
        / BigInt::from(2);
    assert_eq!(pooled_purported_upper, half);
    assert!(pooled_purported_upper < lawful);

    let correctly_centered_upper = [false, true]
        .into_iter()
        .map(|x| {
            [false, true]
                .into_iter()
                .map(|action| BigRational::from_integer((action == x).into()))
                .max()
                .unwrap()
        })
        .sum::<BigRational>()
        / BigInt::from(2);
    assert_eq!(correctly_centered_upper, lawful);
}
