use std::time::Duration;

use walt::rules::Decl;

use walt_response_ladder::compiled::{Actor, SCHEMA};
use walt_response_ladder::compiled_field::CompiledField;
use walt_response_ladder::core::{self, Config, PolicyEvaluator};
use walt_response_ladder::mechanics::{Budget, Field, Problem, PublicState};
use walt_response_ladder::model;
use walt_response_ladder::policy::Policy;

struct EqualRows {
    calls: usize,
    first_rows: usize,
}

impl PolicyEvaluator for EqualRows {
    fn prices(
        &mut self,
        problem: &Problem,
        policies: &[Policy],
        _field: &mut dyn Field,
        _budget: &mut Budget,
    ) -> Result<Option<Vec<u64>>, String> {
        self.calls += 1;
        if self.calls == 1 {
            self.first_rows = policies.len();
        }
        Ok(Some(vec![problem.mass(); policies.len()]))
    }
}

#[test]
fn tail_solver_uses_one_forced_tail_row_per_root_and_preserves_actor() {
    let actor = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
    let mut field = CompiledField::all(&actor).unwrap();
    let mut sampling = Budget::new(100_000, Duration::from_secs(30));
    let problem = model::sample_problem(
        Decl::NoTrump,
        30,
        0,
        (0..7).fold(0, |mask, tile| mask | (1u32 << tile)),
        &PublicState::opening(0),
        2,
        17,
        field.revision().into(),
        &mut sampling,
    )
    .unwrap()
    .unwrap();
    let legal = problem.root.legal(problem.decl, problem.hand).count_ones() as usize;
    let config = Config {
        max_horizon: 0,
        priority_plans: 4,
        scenario_upper: true,
        stop_when_certified: true,
    };
    let mut evaluator = EqualRows {
        calls: 0,
        first_rows: 0,
    };
    let mut budget = Budget::new(100_000, Duration::from_secs(30));
    let report = core::solve_with_tail_evaluator(
        &problem,
        &mut field,
        &mut budget,
        &config,
        Some(&actor),
        Some(&mut evaluator),
    )
    .unwrap();
    assert_eq!(evaluator.first_rows, legal);
    assert_eq!(report.chosen, problem.root.legal(problem.decl, problem.hand).trailing_zeros() as u8);
    assert!(report.canonical_certified);
    assert!(report
        .actions
        .iter()
        .all(|bound| bound.policy.compiled_tail.as_ref() == Some(&actor)));
}
