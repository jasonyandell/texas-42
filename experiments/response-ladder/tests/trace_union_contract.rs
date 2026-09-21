use std::{collections::BTreeSet, time::Duration};
use walt::solver::{self, SplitMix64};
use walt_response_ladder::{
    core::{solve, Config},
    mechanics::{tiles, Budget, DiceField, Field, Problem, PublicState, Scenario},
    policy::price,
    rollout::{evaluate_traces, priority_plans},
    trace_union::{fold, ExactResponse, Trace, TraceBundle},
};

fn budget() -> Budget {
    Budget::new(20_000_000, Duration::from_secs(30))
}

// Byte-for-byte fixture construction from core_contract.rs, so the established
// adaptive and fusion witnesses below identify the same frozen weighted jobs.
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

fn config() -> Config {
    Config {
        max_horizon: 7,
        priority_plans: 2,
        scenario_upper: true,
        stop_when_certified: false,
    }
}
fn bundle(p: &Problem, field: &mut impl Field) -> TraceBundle {
    let plans = priority_plans(p, None).unwrap();
    let rows = evaluate_traces(p, &plans, field, &mut budget())
        .unwrap()
        .into_iter()
        .map(|r| r.into_iter().map(Trace::from).collect())
        .collect();
    TraceBundle::new(p, rows)
}
fn compare(p: &Problem, field: &mut impl Field, b: &TraceBundle) -> ExactResponse {
    let result = fold(p, b, &mut budget()).unwrap().unwrap();
    let exact = solve(p, field, &mut budget(), &config()).unwrap();
    assert!(exact.exact_vector);
    let expected: Vec<_> = exact
        .actions
        .iter()
        .map(|a| {
            assert_eq!(a.lower, a.upper);
            (a.action, a.lower)
        })
        .collect();
    assert_eq!(result.action_values, expected);
    assert_eq!(result.chosen, exact.chosen);
    assert_eq!(
        result.policy.choose(p.decl, &p.root, p.hand),
        Some(result.chosen)
    );
    assert!(result.policy.compiled_tail.is_none());
    assert_eq!(
        price(p, &result.policy, field, &mut budget()).unwrap(),
        Some(result.value)
    );
    result
}
fn row_best_and_revealed(p: &Problem, b: &TraceBundle, root: u8) -> (u64, u64) {
    let rows: Vec<_> = b
        .rows
        .iter()
        .filter(|r| r[0].continuation.first() == Some(&root))
        .collect();
    let lower = rows
        .iter()
        .map(|r| {
            r.iter()
                .zip(&p.scenarios)
                .map(|(t, s)| t.payoff * s.weight)
                .sum::<u64>()
        })
        .max()
        .unwrap();
    let upper = p
        .scenarios
        .iter()
        .enumerate()
        .map(|(id, s)| s.weight * rows.iter().map(|r| r[id].payoff).max().unwrap())
        .sum();
    (lower, upper)
}

#[test]
fn trace_union_recovers_adaptation_beyond_every_fixed_priority() {
    let p = fixture(9, 4, 0, 3);
    let b = bundle(&p, &mut DiceField);
    assert_eq!(row_best_and_revealed(&p, &b, 8), (7, 9));
    let r = compare(&p, &mut DiceField, &b);
    assert_eq!(
        r.action_values.iter().find(|&&(a, _)| a == 8),
        Some(&(8, 9))
    );
    assert!(r
        .policy
        .decisions
        .iter()
        .all(|d| d.history.starts_with(&p.root.history)));
}

#[test]
fn trace_union_rejects_strategy_fusion_and_preserves_canonical_ties() {
    let p = fixture(9, 4, 0, 0);
    let b = bundle(&p, &mut DiceField);
    assert_eq!(row_best_and_revealed(&p, &b, 9), (9, 10));
    let r = compare(&p, &mut DiceField, &b);
    assert_eq!(
        r.action_values.iter().find(|&&(a, _)| a == 9),
        Some(&(9, 9))
    );
    let best = r.action_values.iter().map(|&(_, v)| v).max().unwrap();
    assert_eq!(
        r.chosen,
        r.action_values.iter().find(|&&(_, v)| v == best).unwrap().0
    );
}

#[test]
fn trace_union_matches_complete_cpu_vectors_for_all_declarations_and_partials() {
    let mut teams = [false; 2];
    for declaration in [0, 1, 2, 3, 4, 5, 6, 7, 9] {
        for partial in 0..4 {
            let p = fixture(
                declaration,
                3,
                partial,
                420601 + declaration as u64 * 17 + partial as u64,
            );
            teams[(p.viewer & 1) as usize] = true;
            let b = bundle(&p, &mut DiceField);
            compare(&p, &mut DiceField, &b);
        }
    }
    assert!(teams.into_iter().all(|v| v));
}

#[test]
fn trace_union_original_weighted_duplicates_are_not_lane_multiplicity() {
    let p = fixture(6, 3, 2, 7251);
    assert_eq!(p.scenarios[0].hands, p.scenarios[1].hands);
    assert_ne!(p.scenarios[0].tape, p.scenarios[1].tape);
    assert_ne!(p.scenarios[0].weight, p.scenarios[1].weight);
    let mut b = bundle(&p, &mut DiceField);
    let before = compare(&p, &mut DiceField, &b);
    let repeated = b.rows.clone();
    b.rows.extend(repeated);
    let after = compare(&p, &mut DiceField, &b);
    assert_eq!(before.action_values, after.action_values);
    assert_eq!(before.policy, after.policy);
    assert_eq!(before.unique_nodes, after.unique_nodes);
    assert_eq!(before.mass, 10);
    assert_eq!(after.lanes, before.lanes * 2);
}

#[test]
fn trace_union_accepts_original_bit63_and_refuses_more_columns() {
    let mut p = fixture(6, 2, 1, 8871);
    let template = p.scenarios[0].clone();
    p.scenarios = (0..64)
        .map(|id| Scenario {
            hands: template.hands,
            tape: template.tape ^ id,
            weight: 1 + id % 3,
        })
        .collect();
    let b = bundle(&p, &mut DiceField);
    compare(&p, &mut DiceField, &b);
    p.scenarios.push(template);
    assert!(fold(&p, &b, &mut budget()).unwrap_err().contains("64"));
}

#[test]
fn trace_union_refuses_changed_problem_identity() {
    let p = fixture(9, 3, 1, 42);
    let b = bundle(&p, &mut DiceField);
    let mut variants = Vec::new();
    let mut q = p.clone();
    q.field_revision.push_str("/changed");
    variants.push(q);
    let mut q = p.clone();
    q.scenarios[0].weight += 1;
    variants.push(q);
    let mut q = p.clone();
    q.scenarios[0].tape ^= 1;
    variants.push(q);
    let mut q = p.clone();
    q.scenarios.swap(0, 1);
    variants.push(q);
    let mut q = p.clone();
    q.root.history.reverse();
    variants.push(q);
    for q in variants {
        assert!(fold(&q, &b, &mut budget()).is_err());
    }
}

#[test]
fn trace_union_refuses_missing_root_and_descendant_coverage() {
    let p = fixture(9, 4, 0, 3);
    let b = bundle(&p, &mut DiceField);
    let mut missing_root = b.clone();
    missing_root.rows.retain(|r| r[0].continuation[0] != 8);
    assert!(fold(&p, &missing_root, &mut budget()).is_err());
    let mut one_per_root = b.clone();
    let mut seen = BTreeSet::new();
    one_per_root
        .rows
        .retain(|r| seen.insert(r[0].continuation[0]));
    assert!(fold(&p, &one_per_root, &mut budget()).is_err());
    let mut missing_column = b.clone();
    missing_column.rows[0].pop();
    assert!(fold(&p, &missing_column, &mut budget()).is_err());
}

#[test]
fn trace_union_refuses_corrupt_illegal_and_premature_terminal_traces() {
    let p = fixture(9, 4, 0, 3);
    let b = bundle(&p, &mut DiceField);
    let mut changed = b.clone();
    changed.rows[0][0].payoff ^= 1;
    assert!(fold(&p, &changed, &mut budget()).is_err());
    let mut changed = b.clone();
    changed.rows[0][0].continuation[0] = 28;
    assert!(fold(&p, &changed, &mut budget()).is_err());
    let mut changed = b.clone();
    changed.rows[0][0].continuation.pop();
    assert!(fold(&p, &changed, &mut budget()).is_err());
    let mut changed = b.clone();
    changed.rows[0][0].continuation.push(0);
    assert!(fold(&p, &changed, &mut budget()).is_err());
    // A valid tile in the wrong player's hand is still illegal, even when it
    // has not previously been played in the public trace.
    let mut changed = b.clone();
    let other = tiles(p.scenarios[0].hands[((p.viewer + 1) & 3) as usize])[0];
    changed.rows[0][0].continuation[0] = other;
    assert!(fold(&p, &changed, &mut budget()).is_err());
}

#[test]
fn trace_union_refuses_two_field_actions_for_the_same_original_column() {
    let p = fixture(9, 4, 0, 3);
    let b = bundle(&p, &mut DiceField);
    let plans = priority_plans(&p, None).unwrap();
    let mut contradicted = false;
    for salt in 1..65 {
        let mut alternate = p.clone();
        alternate.scenarios[0].tape ^= salt;
        let rows = evaluate_traces(&alternate, &plans[..1], &mut DiceField, &mut budget()).unwrap();
        let replacement = Trace::from(rows.into_iter().next().unwrap().into_iter().next().unwrap());
        if replacement != b.rows[0][0] {
            let mut changed = b.clone();
            let mut row = b.rows[0].clone();
            row[0] = replacement;
            changed.rows.push(row);
            assert!(fold(&p, &changed, &mut budget()).is_err());
            contradicted = true;
            break;
        }
    }
    assert!(
        contradicted,
        "directed tape perturbation must expose a field partition conflict"
    );
}

#[test]
fn trace_union_cancellation_never_publishes_a_partial_vector_or_policy() {
    let p = fixture(9, 3, 1, 72);
    let b = bundle(&p, &mut DiceField);
    let mut measured = budget();
    let complete = fold(&p, &b, &mut measured).unwrap().unwrap();
    for limit in [0, 1, measured.used / 2, measured.used] {
        let mut capped = Budget::new(limit, Duration::from_secs(30));
        assert!(fold(&p, &b, &mut capped).unwrap().is_none());
    }
    let mut enough = Budget::new(measured.used + 1, Duration::from_secs(30));
    assert_eq!(
        fold(&p, &b, &mut enough).unwrap().unwrap().action_values,
        complete.action_values
    );
}

#[test]
fn trace_union_compiled_field_matches_cpu_and_reprices_total_policy() {
    use walt_response_ladder::{
        compiled::{Actor, SCHEMA},
        compiled_field::CompiledField,
    };
    let c0 = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
    let c1 = Actor::new(SCHEMA, vec![3, 1, 6]).unwrap();
    let mut p = fixture(6, 3, 2, 6721);
    let mut field = CompiledField::partner(p.viewer, &c0, &c1).unwrap();
    p.field_revision = field.revision().to_owned();
    let b = bundle(&p, &mut field);
    compare(&p, &mut field, &b);
}

#[cfg(feature = "gpu")]
#[test]
fn trace_union_existing_gpu_epochs_match_cpu_union_with_compiled_field() {
    use walt_response_ladder::policy::Policy;
    use walt_response_ladder::{
        compiled::{Actor, SCHEMA},
        compiled_field::CompiledField,
        gpu_epochs::EpochEvaluator,
    };
    let c0 = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
    let c1 = Actor::new(SCHEMA, vec![3, 1, 6]).unwrap();
    let mut gpu = EpochEvaluator::new().unwrap();
    for partial in [0, 2] {
        let mut p = fixture(6, 3, partial, 6781 + partial as u64);
        let mut field = CompiledField::partner(p.viewer, &c0, &c1).unwrap();
        p.field_revision = field.revision().to_owned();
        let plans = priority_plans(&p, None).unwrap();
        let policies: Vec<_> = plans
            .iter()
            .map(|order| Policy::priority(p.viewer, order.clone()))
            .collect();
        let rows = gpu
            .traces(&p, &policies, &mut field, &mut budget())
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|r| r.into_iter().map(Trace::from).collect())
            .collect();
        let gpu_bundle = TraceBundle::new(&p, rows);
        let cpu_bundle = bundle(&p, &mut field);
        assert_eq!(gpu_bundle.rows, cpu_bundle.rows);
        let a = compare(&p, &mut field, &gpu_bundle);
        let b = compare(&p, &mut field, &cpu_bundle);
        assert_eq!(a.action_values, b.action_values);
        assert_eq!(a.policy, b.policy);
    }
}
