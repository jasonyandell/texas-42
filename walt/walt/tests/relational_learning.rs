use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use walt::policy_search;
use walt::policy_search::relational::{
    evaluate_program_cost, relational_grammar, DecisionExample, RelationalLearner, RelationalLimits,
};
use walt::rules::{legal_plays, Decl, Domino};
use walt::scheme::{Access, PolicyProgram, Registry, Term, Value};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn example(seed: u64, cost: impl Fn(Domino) -> BigRational) -> Result<DecisionExample, String> {
    let fixture = policy_search::fixture(seed, "P6".parse::<Decl>().unwrap(), 7, None)?;
    let frame = fixture.exercise.frame.clone();
    let legal = legal_plays(
        frame.kernel().decl(),
        frame.kernel().viewer_hand(),
        frame.led_context(),
    );
    let costs = legal.iter().map(|tile| (tile, cost(tile))).collect();
    DecisionExample::new(
        frame,
        fixture.history,
        fixture.exercise.position.banked,
        fixture.exercise.position.bid as u8,
        fixture.exercise.position.declaring_team,
        costs,
        BigRational::one(),
    )
}

#[test]
fn tied_action_costs_do_not_force_a_relational_split() {
    let examples = vec![example(17, |_| BigRational::zero()).unwrap()];
    let fit = RelationalLearner::new(RelationalLimits::default())
        .unwrap()
        .fit(&examples, "ties")
        .unwrap();
    assert!(fit.program.rules.is_empty());
    assert!(fit.program.exact_rules.is_empty());
    assert!(fit.program.bindings.is_empty());
    assert_eq!(fit.metrics.selected_weighted_cost, BigRational::zero());
    assert_eq!(fit.metrics.selected_ast_nodes, 1);
}

#[test]
fn changing_only_a_state_common_upper_cannot_rerank_programs() {
    let base = (1..9).map(|seed| example(seed, |d| {
        if d.count()==5 {q(0,1)} else {q(1,4)}
    }).unwrap()).collect::<Vec<_>>();
    let shifted = (1..9).map(|seed| example(seed, |d| {
        let loss=if d.count()==5 {q(0,1)} else {q(1,4)};
        loss+q((seed%2+1) as i64,4)
    }).unwrap()).collect::<Vec<_>>();
    let learner=RelationalLearner::new(RelationalLimits::default()).unwrap();
    let a=learner.fit(&base,"upper-invariance").unwrap();
    let b=learner.fit(&shifted,"upper-invariance").unwrap();
    assert_eq!(a.program,b.program);
    assert_eq!(a.candidates.iter().map(|c|&c.digest).collect::<Vec<_>>(),b.candidates.iter().map(|c|&c.digest).collect::<Vec<_>>());
}

#[test]
fn frozen_grammar_is_current_viewer_only_and_contains_no_physical_tile_literal() {
    let registry = Registry::standard();
    let allowed = [
        "own-legal",
        "count",
        "master",
        "led-context",
        "in",
        "boss",
        "viewer",
        "current-winner",
        "partner",
    ];
    let grammar = relational_grammar();
    assert_eq!(grammar.len(), 14);
    for clause in grammar {
        let compiled = clause.guard.compile(&registry).unwrap();
        assert!(compiled
            .predicate_specs()
            .iter()
            .all(|spec| spec.access == Access::Viewer && spec.horizon_plies == 0));
        for case in &clause.guard.cases {
            for atom in &case.atoms {
                assert!(allowed.contains(&atom.predicate.as_str()));
                assert!(atom
                    .args
                    .iter()
                    .all(|term| !matches!(term, Term::Literal(Value::Domino(_)))));
            }
        }
    }
}

#[test]
fn learned_program_roundtrips_and_runtime_cost_matches_cached_training_cost() {
    let (seed, example) = (1..500)
        .find_map(|seed| {
            let fixture = policy_search::fixture(seed, "P6".parse().unwrap(), 7, None).ok()?;
            let frame = fixture.exercise.frame.clone();
            let legal = legal_plays(
                frame.kernel().decl(),
                frame.kernel().viewer_hand(),
                frame.led_context(),
            );
            let first = legal.iter().next()?;
            if first.count() == 5 || !legal.iter().any(|tile| tile.count() == 5) {
                return None;
            }
            let costs: BTreeMap<_, _> = legal
                .iter()
                .map(|tile| {
                    (
                        tile,
                        if tile.count() == 5 {
                            BigRational::zero()
                        } else {
                            BigRational::one()
                        },
                    )
                })
                .collect();
            let row = DecisionExample::new(
                frame,
                fixture.history,
                fixture.exercise.position.banked,
                fixture.exercise.position.bid as u8,
                fixture.exercise.position.declaring_team,
                costs,
                q(1, 3),
            )
            .ok()?;
            Some((seed, row))
        })
        .expect("a fixture with both a lower non-count and a five-count legal action");
    let examples = vec![example];
    let limits = RelationalLimits {
        max_clauses: 2,
        max_ast_nodes: 64,
        beam_width: 1,
        max_search_work: 2_000,
        max_inference_work_per_example: 100_000,
    };
    let fit = RelationalLearner::new(limits)
        .unwrap()
        .fit(&examples, "count-cost")
        .unwrap();
    assert!(!fit.program.rules.is_empty(), "seed {seed}");
    assert!(fit.program.rules.len() <= limits.max_clauses);
    assert!(fit.metrics.selected_ast_nodes <= limits.max_ast_nodes);
    assert_eq!(fit.metrics.selected_weighted_cost, BigRational::zero());
    assert!(fit
        .candidates
        .iter()
        .any(|candidate| candidate.program.rules.is_empty()));
    let text = fit.program.to_string();
    let reparsed: PolicyProgram = text.parse().unwrap();
    assert_eq!(reparsed, fit.program);
    assert_eq!(policy_search::program_digest(&text), fit.digest);
    assert_eq!(
        evaluate_program_cost(&reparsed, &examples, 100_000).unwrap(),
        fit.metrics.selected_weighted_cost
    );
}

#[test]
fn search_caps_and_results_are_deterministic() {
    let rows = vec![example(23, |_| BigRational::zero()).unwrap()];
    let limits = RelationalLimits {
        max_clauses: 3,
        max_ast_nodes: 80,
        beam_width: 4,
        max_search_work: 2,
        max_inference_work_per_example: 100_000,
    };
    let first = RelationalLearner::new(limits)
        .unwrap()
        .fit(&rows, "bounded")
        .unwrap();
    let second = RelationalLearner::new(limits)
        .unwrap()
        .fit(&rows, "bounded")
        .unwrap();
    assert!(first.metrics.search_cap_hit);
    assert_eq!(first.metrics.search_work, 2);
    assert!(first
        .candidates
        .iter()
        .any(|candidate| candidate.program.rules.is_empty()));
    assert_eq!(first, second);
}

#[test]
fn examples_require_complete_bounded_costs() {
    let fixture = policy_search::fixture(31, "P6".parse().unwrap(), 7, None).unwrap();
    assert!(DecisionExample::new(
        fixture.exercise.frame,
        fixture.history,
        fixture.exercise.position.banked,
        fixture.exercise.position.bid as u8,
        fixture.exercise.position.declaring_team,
        BTreeMap::new(),
        BigRational::one(),
    )
    .is_err());
}
