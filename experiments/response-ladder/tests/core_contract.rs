use num_traits::ToPrimitive;
use std::{sync::Arc, time::Duration};
use walt::rules::Seat;
use walt::solver::{self, Deadline, Shared, Solver, SplitMix64};
use walt_response_ladder::core::{solve, Config};
use walt_response_ladder::mechanics::{
    tiles, Budget, DiceField, Field, FieldQuery, Problem, PublicState, Scenario,
};
use walt_response_ladder::policy::{price, replay_scenario, Decision, Policy};

fn budget() -> Budget {
    Budget::new(20_000_000, Duration::from_secs(30))
}

// Start with an actual legal prefix, then retain a declared residual bundle.
// It is a finite modeled belief, not a claim of reachability-conditioned belief.
fn fixture(declaration: usize, remaining: usize, partial: usize, seed: u64) -> Problem {
    let decl = solver::decl_of(declaration);
    let mut rng = SplitMix64(seed);
    let mut deck: Vec<u8> = (0..28).collect();
    for i in (1..deck.len()).rev() {
        deck.swap(i, rng.below((i + 1) as u64) as usize);
    }
    let hands: [u32; 4] = std::array::from_fn(|seat| {
        deck[seat * 7..seat * 7 + 7]
            .iter()
            .fold(0, |m, &t| m | (1 << t))
    });
    let mut root = PublicState::opening(rng.below(4) as u8);
    for _ in 0..4 * (7 - remaining) + partial {
        let legal = tiles(root.legal(decl, hands[root.actor() as usize]));
        let action = legal[rng.below(legal.len() as u64) as usize];
        root = root.after(decl, action);
    }
    let viewer = root.actor();
    let hand = hands[viewer as usize] & !root.played;
    let outstanding = 42 - root.banked_t0 - root.banked_t1;
    let bid = root.banked_t1 + (outstanding / 2).max(1);
    let sizes: [usize; 4] =
        std::array::from_fn(|seat| (hands[seat] & !root.played).count_ones() as usize);
    let mut unknown = tiles(solver::FULL_MASK & !(root.played | hand));
    let mut scenarios = Vec::new();
    for id in 0..4 {
        for i in (1..unknown.len()).rev() {
            unknown.swap(i, rng.below((i + 1) as u64) as usize);
        }
        let mut world = [0; 4];
        world[viewer as usize] = hand;
        let mut at = 0;
        for seat in 0..4 {
            if seat == viewer as usize {
                continue;
            }
            for &t in &unknown[at..at + sizes[seat]] {
                world[seat] |= 1 << t;
            }
            at += sizes[seat];
        }
        scenarios.push(Scenario {
            hands: world,
            tape: rng.next_u64(),
            weight: id + 1,
        });
    }
    // Same physical deal, distinct original sample, tape and integer mass.
    scenarios[1].hands = scenarios[0].hands;
    Problem {
        decl,
        bid,
        viewer,
        hand,
        root,
        scenarios,
        field_revision: "historical-dice-v1".into(),
    }
}

fn canonical(problem: &Problem) -> Vec<(u8, u64)> {
    let mut worlds = Vec::new();
    let mut tapes = Vec::new();
    for scenario in &problem.scenarios {
        for _ in 0..scenario.weight {
            worlds.push(scenario.hands);
            tapes.push(scenario.tape);
        }
    }
    let shared = Arc::new(Shared::new(
        problem.decl,
        problem.bid,
        vec![worlds.len()],
        problem.root.played,
        problem.hand.count_ones() as usize,
        Deadline::after(Duration::from_secs(30)),
    ));
    let solver = Solver::new(
        shared,
        Seat::from_index(problem.viewer as usize).unwrap(),
        problem.hand,
        problem.viewer % 2 == 1,
        worlds,
        tapes,
        solver::Field::Dice,
    );
    let values = solver
        .action_values(
            &problem.root.key(),
            &tiles(problem.root.legal(problem.decl, problem.hand)),
        )
        .unwrap();
    values
        .into_iter()
        .map(|(action, value)| {
            let scaled = value.numer().to_u64().unwrap() * problem.mass();
            let denominator = value.denom().to_u64().unwrap();
            assert_eq!(scaled % denominator, 0);
            let t1 = scaled / denominator;
            (
                action,
                if problem.viewer % 2 == 1 {
                    t1
                } else {
                    problem.mass() - t1
                },
            )
        })
        .collect()
}

fn exact_config() -> Config {
    Config {
        max_horizon: 7,
        priority_plans: 2,
        scenario_upper: true,
        stop_when_certified: false,
    }
}

#[test]
fn exact_vectors_and_extracted_policies_match_frozen_walt_across_declarations() {
    let mut viewers = [false; 4];
    let mut actions_checked = 0;
    for declaration in [0, 1, 2, 3, 4, 5, 6, 7, 9] {
        for partial in 0..4 {
            let problem = fixture(
                declaration,
                3,
                partial,
                420601 + declaration as u64 * 17 + partial as u64,
            );
            viewers[problem.viewer as usize] = true;
            let expected = canonical(&problem);
            let report = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
            assert!(!report.interrupted);
            assert!(report.exact_vector && report.canonical_certified);
            assert_eq!(report.regret_bound, 0);
            for (bound, (action, value)) in report.actions.iter().zip(expected) {
                actions_checked += 1;
                assert_eq!(
                    (bound.action, bound.lower, bound.upper),
                    (action, value, value),
                    "declaration {declaration}, partial {partial}"
                );
                assert!(bound.priced);
                assert_eq!(
                    price(&problem, &bound.policy, &mut DiceField, &mut budget()).unwrap(),
                    Some(value)
                );
                assert_eq!(
                    bound
                        .policy
                        .choose(problem.decl, &problem.root, problem.hand),
                    Some(action)
                );
            }
            let roundtrip: walt_response_ladder::core::Report =
                serde_json::from_str(&serde_json::to_string(&report).unwrap()).unwrap();
            assert_eq!(roundtrip.incumbent, report.incumbent);
        }
    }
    assert!(viewers.into_iter().all(|v| v));
    assert!(actions_checked >= 50);
}

#[test]
fn expired_budget_returns_total_unpriced_reserve_and_zero_floor() {
    let problem = fixture(0, 3, 0, 4);
    let report = solve(
        &problem,
        &mut DiceField,
        &mut Budget::new(0, Duration::ZERO),
        &exact_config(),
    )
    .unwrap();
    assert!(report.interrupted);
    assert_eq!(report.work_used, 0);
    assert_eq!(report.incumbent_value, None);
    assert_eq!(report.regret_bound, problem.mass());
    for bound in &report.actions {
        assert_eq!(
            (bound.lower, bound.upper, bound.priced),
            (0, problem.mass(), false)
        );
        assert_eq!(
            bound
                .policy
                .choose(problem.decl, &problem.root, problem.hand),
            Some(bound.action)
        );
    }
    assert!(
        price(&problem, &report.incumbent, &mut DiceField, &mut budget())
            .unwrap()
            .is_some()
    );
}

#[test]
fn interruption_preserves_completed_transaction_and_bounds_improve_monotonically() {
    let problem = fixture(2, 3, 0, 77);
    let first = tiles(problem.hand)[0];
    let policy = Policy::forced(&problem, first, tiles(problem.hand & !(1 << first)));
    let mut measured = budget();
    let value = price(&problem, &policy, &mut DiceField, &mut measured)
        .unwrap()
        .unwrap();
    let config = Config {
        priority_plans: 1,
        ..exact_config()
    };
    let report = solve(
        &problem,
        &mut DiceField,
        &mut Budget::new(measured.used + 1, Duration::from_secs(30)),
        &config,
    )
    .unwrap();
    assert!(report.interrupted);
    assert_eq!(report.actions[0].policy, policy);
    assert_eq!(report.actions[0].lower, value);
    assert!(report.actions[0].priced);
    assert!(report.actions.iter().skip(1).all(|a| !a.priced));

    let expected = canonical(&problem);
    let mut previous = None;
    for limit in [0, 1, 64, 256, 1024, 4096, 1_000_000] {
        let report = solve(
            &problem,
            &mut DiceField,
            &mut Budget::new(limit, Duration::from_secs(30)),
            &config,
        )
        .unwrap();
        for (a, (_, value)) in report.actions.iter().zip(&expected) {
            assert!(a.lower <= *value && *value <= a.upper);
            if a.priced {
                assert_eq!(
                    price(&problem, &a.policy, &mut DiceField, &mut budget()).unwrap(),
                    Some(a.lower)
                );
            }
        }
        if let Some(old) = previous {
            let old: walt_response_ladder::core::Report = old;
            for (before, after) in old.actions.iter().zip(&report.actions) {
                assert!(after.lower >= before.lower && after.upper <= before.upper);
            }
        }
        previous = Some(report);
    }
    assert!(previous.unwrap().exact_vector);
}

#[test]
fn first_plan_round_prices_every_root_before_a_second_plan() {
    let problem = fixture(2, 3, 0, 77);
    let mut measured = budget();
    let witnesses: Vec<_> = tiles(problem.hand)
        .into_iter()
        .map(|action| {
            let policy = Policy::forced(&problem, action, tiles(problem.hand & !(1 << action)));
            let value = price(&problem, &policy, &mut DiceField, &mut measured)
                .unwrap()
                .unwrap();
            (policy, value)
        })
        .collect();
    let mut allowance = Budget::new(measured.used + 1, Duration::from_secs(30));
    let report = solve(&problem, &mut DiceField, &mut allowance, &exact_config()).unwrap();
    assert!(report.interrupted);
    for (bound, (policy, value)) in report.actions.iter().zip(witnesses) {
        assert!(bound.priced);
        assert_eq!((bound.lower, &bound.policy), (value, &policy));
    }
}

#[test]
fn full_public_history_is_the_policy_key_and_default_is_total_off_sample() {
    let problem = fixture(3, 3, 0, 93);
    let legal = tiles(problem.root.legal(problem.decl, problem.hand));
    let mut other = problem.root.clone();
    // Distinct full histories with the same reduced record must not alias.
    other.history.swap(0, 1);
    let mut decisions = vec![
        Decision {
            history: problem.root.history.clone(),
            action: legal[0],
        },
        Decision {
            history: other.history.clone(),
            action: legal[1],
        },
    ];
    decisions.sort_by(|a, b| a.history.cmp(&b.history));
    let policy = Policy {
        viewer: problem.viewer,
        priority: vec![],
        decisions,
        compiled_tail: None,
    };
    assert_eq!(
        policy.choose(problem.decl, &problem.root, problem.hand),
        Some(legal[0])
    );
    assert_eq!(
        policy.choose(problem.decl, &other, problem.hand),
        Some(legal[1])
    );

    let trained = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    let mut fresh = problem.scenarios[0].clone();
    fresh.tape ^= 0x84217;
    let replay = replay_scenario(
        &problem,
        &fresh,
        &trained.incumbent,
        &mut DiceField,
        &mut budget(),
    )
    .unwrap()
    .unwrap();
    assert!(replay.payoff <= 1);
    let mut fallback = trained.incumbent.clone();
    fallback.decisions.clear();
    fallback.priority.clear();
    assert!(
        replay_scenario(&problem, &fresh, &fallback, &mut DiceField, &mut budget())
            .unwrap()
            .is_some()
    );
}

#[test]
fn frozen_revision_and_illegal_field_actions_are_refused() {
    struct Changed {
        changed: bool,
    }
    impl Field for Changed {
        fn revision(&self) -> &str {
            if self.changed {
                "changed"
            } else {
                "historical-dice-v1"
            }
        }
        fn choose(&mut self, q: FieldQuery<'_>, b: &mut Budget) -> Option<u8> {
            self.changed = true;
            DiceField.choose(q, b)
        }
    }
    struct Illegal;
    impl Field for Illegal {
        fn revision(&self) -> &str {
            "historical-dice-v1"
        }
        fn choose(&mut self, _: FieldQuery<'_>, _: &mut Budget) -> Option<u8> {
            Some(255)
        }
    }
    let problem = fixture(0, 3, 0, 4);
    assert!(solve(
        &problem,
        &mut Changed { changed: false },
        &mut budget(),
        &exact_config()
    )
    .unwrap_err()
    .contains("revision"));
    assert!(
        solve(&problem, &mut Illegal, &mut budget(), &exact_config())
            .unwrap_err()
            .contains("illegal")
    );
    let mut different = problem;
    different.field_revision = "new-target".into();
    assert!(solve(&different, &mut DiceField, &mut budget(), &exact_config()).is_err());
}

#[test]
fn identical_original_ids_add_mass_without_creating_new_observations() {
    let mut problem = fixture(5, 3, 0, 51);
    let original = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    for scenario in &mut problem.scenarios {
        scenario.weight *= 2;
    }
    let doubled = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    let mut split = problem.clone();
    split.scenarios = problem
        .scenarios
        .iter()
        .flat_map(|s| {
            (0..s.weight)
                .map(|_| Scenario {
                    weight: 1,
                    ..s.clone()
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let split = solve(&split, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    for ((a, b), c) in original
        .actions
        .iter()
        .zip(&doubled.actions)
        .zip(&split.actions)
    {
        assert_eq!(b.lower, 2 * a.lower);
        assert_eq!(b.lower, c.lower);
        assert_eq!(b.upper, c.upper);
    }
    assert_eq!(original.chosen, doubled.chosen);
    assert_eq!(doubled.chosen, split.chosen);
}

#[test]
fn trivial_frontier_upper_covers_exact_values_with_a_restricted_pool() {
    let problem = fixture(7, 4, 0, 17);
    let config = Config {
        max_horizon: 0,
        priority_plans: 1,
        scenario_upper: false,
        stop_when_certified: false,
    };
    let report = solve(&problem, &mut DiceField, &mut budget(), &config).unwrap();
    for (bound, (_, value)) in report.actions.iter().zip(canonical(&problem)) {
        assert!(bound.lower <= value && value <= bound.upper);
        assert_eq!(
            price(&problem, &bound.policy, &mut DiceField, &mut budget()).unwrap(),
            Some(bound.lower)
        );
    }
}

#[test]
fn canonical_ties_choose_the_least_tile() {
    let mut problem = fixture(0, 3, 0, 4);
    if problem.viewer % 2 == 1 {
        problem.root.banked_t1 = problem.bid;
    } else {
        problem.root.banked_t0 = 43 - problem.bid;
    }
    let report = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    assert!(report.canonical_certified && report.exact_vector);
    assert_eq!(report.chosen, tiles(problem.hand)[0]);
    assert!(report
        .actions
        .iter()
        .all(|a| a.lower == problem.mass() && a.upper == problem.mass()));
}

#[test]
fn adaptive_loss_witness_beats_the_complete_priority_family() {
    let problem = fixture(9, 4, 0, 3);
    // With four own tiles and fixed root, six plans are ALL tail permutations.
    let config = Config {
        max_horizon: 0,
        priority_plans: 6,
        scenario_upper: true,
        stop_when_certified: false,
    };
    let plans = solve(&problem, &mut DiceField, &mut budget(), &config).unwrap();
    let bound = plans.actions.iter().find(|a| a.action == 8).unwrap();
    assert_eq!((bound.lower, bound.upper), (7, 9));
    assert_eq!(
        canonical(&problem).into_iter().find(|(a, _)| *a == 8),
        Some((8, 9))
    );
    let exact = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    let adaptive = exact.actions.iter().find(|a| a.action == 8).unwrap();
    assert_eq!((adaptive.lower, adaptive.upper), (9, 9));
    assert_eq!(
        price(&problem, &adaptive.policy, &mut DiceField, &mut budget()).unwrap(),
        Some(9)
    );
    assert!(adaptive.policy.decisions.len() > 1);

    let restricted = solve(
        &problem,
        &mut DiceField,
        &mut budget(),
        &Config {
            priority_plans: 1,
            scenario_upper: false,
            ..config
        },
    )
    .unwrap();
    let restricted = restricted.actions.iter().find(|a| a.action == 8).unwrap();
    assert!(restricted.lower <= 7 && restricted.upper >= 9);
}

#[test]
fn fusion_witness_and_strict_lower_index_tie_exclusion() {
    let problem = fixture(9, 4, 0, 0);
    let config = Config {
        max_horizon: 0,
        priority_plans: 6,
        scenario_upper: true,
        stop_when_certified: false,
    };
    let relaxed = solve(&problem, &mut DiceField, &mut budget(), &config).unwrap();
    let bound = relaxed.actions.iter().find(|a| a.action == 9).unwrap();
    assert_eq!((bound.lower, bound.upper), (9, 10));
    assert_eq!(
        canonical(&problem).into_iter().find(|(a, _)| *a == 9),
        Some((9, 9))
    );
    let exact = solve(&problem, &mut DiceField, &mut budget(), &exact_config()).unwrap();
    let lawful = exact.actions.iter().find(|a| a.action == 9).unwrap();
    assert_eq!((lawful.lower, lawful.upper), (9, 9));

    let tied = solve(
        &problem,
        &mut DiceField,
        &mut budget(),
        &Config {
            priority_plans: 1,
            scenario_upper: false,
            ..config
        },
    )
    .unwrap();
    assert_eq!(tied.chosen, 10);
    assert!(tied.optimal_certified);
    assert!(!tied.canonical_certified); // Tile 9 still has upper 10.
    assert!(!tied.exact_vector);
    assert_eq!(tied.regret_bound, 0); // Incumbent wins every modeled scenario.
    assert!(exact.canonical_certified);
    assert_eq!(exact.chosen, 10);
}
