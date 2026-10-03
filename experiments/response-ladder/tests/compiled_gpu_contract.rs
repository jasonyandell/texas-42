#![cfg(feature = "gpu")]

use std::time::Duration;

use walt::rules::Decl;
use walt::solver::SplitMix64;
use walt_response_ladder::mechanics::PublicState;
use walt_response_ladder::policy::Policy;
use walt_response_ladder::{
    compiled::{Actor, CLAUSE_COUNT, SCHEMA},
    compiled_field::CompiledField,
    gpu_epochs::EpochEvaluator,
    mechanics::{tiles, Budget, Field, Problem},
    model, policy,
};

fn budget() -> Budget {
    Budget::new(20_000_000, Duration::from_secs(30))
}

fn fixture(
    decl: Decl,
    partial: usize,
    seed: u64,
    c0: &Actor,
    c1: &Actor,
) -> (Problem, CompiledField, Vec<Policy>) {
    let mut rng = SplitMix64(seed);
    let mut deck: Vec<u8> = (0..28).collect();
    for i in (1..28).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        deck.swap(i, j);
    }
    let hands: [u32; 4] = std::array::from_fn(|seat| {
        deck[seat * 7..seat * 7 + 7]
            .iter()
            .fold(0, |mask, &tile| mask | (1 << tile))
    });
    let mut public = PublicState::opening((seed % 4) as u8);
    let depth = 2 + (seed as usize % 6);
    for _ in 0..4 * (7 - depth) + partial {
        let legal = tiles(public.legal(decl, hands[public.actor() as usize]));
        let tile = legal[rng.below(legal.len() as u64) as usize];
        public = public.after(decl, tile);
    }
    let viewer = public.actor();
    let bid = public.banked_t1 + (42 - public.banked_t1 - public.banked_t0).div_ceil(2);
    let field = CompiledField::partner(viewer, c0, c1).unwrap();
    let identity = field.revision().to_owned();
    let mut sample_budget = budget();
    let mut problem = model::sample_problem(
        decl,
        bid,
        viewer,
        hands[viewer as usize] & !public.played,
        &public,
        4,
        seed ^ 0x5eed,
        identity,
        &mut sample_budget,
    )
    .unwrap()
    .unwrap();
    // Keep duplicate physical allocations as separate weighted columns.
    problem.scenarios[1].hands = problem.scenarios[0].hands;
    for (i, scenario) in problem.scenarios.iter_mut().enumerate() {
        scenario.weight = [1, 3, 7, 2][i];
    }
    let policies = vec![Policy::priority(viewer, tiles(problem.hand))];
    (problem, field, policies)
}

#[test]
fn compiled_epoch_matches_cpu_full_traces_across_declarations_positions_and_clauses() {
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut seen_viewer_teams = [false; 2];
    for clause in 0..CLAUSE_COUNT {
        let c0 = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
        let c1 = Actor::new(
            SCHEMA,
            vec![clause, (clause + 1) % CLAUSE_COUNT, (clause + 2) % CLAUSE_COUNT],
        )
        .unwrap();
        for (decl_index, &decl) in Decl::ALL.iter().enumerate() {
            for partial in 0..4 {
                let seed = 0x9100
                    + clause as u64 * 0x100
                    + decl_index as u64 * 8
                    + partial as u64;
                let (problem, mut compiled, policies) =
                    fixture(decl, partial, seed, &c0, &c1);
                seen_viewer_teams[(problem.viewer % 2) as usize] = true;
                gpu.reset_stats();
                let mut run_budget = budget();
                let traces = gpu
                    .traces(&problem, &policies, &mut compiled, &mut run_budget)
                    .unwrap()
                    .expect("compiled GPU batch completes");
                let label = format!("clause {clause} decl {decl:?} partial {partial}");
                assert_eq!(gpu.stats.epochs, 1, "{label}");
                assert_eq!(gpu.stats.field_requests, 0, "{label}");
                assert_eq!(gpu.stats.unique_field_queries, 0, "{label}");

                let mut cpu = CompiledField::partner(problem.viewer, &c0, &c1).unwrap();
                for (scenario, trace) in problem.scenarios.iter().zip(&traces[0]) {
                    let replay = policy::replay_scenario(
                        &problem,
                        scenario,
                        &policies[0],
                        &mut cpu,
                        &mut budget(),
                    )
                    .unwrap()
                    .unwrap();
                    assert_eq!(trace.payoff, replay.payoff, "{label}");
                    assert_eq!(
                        trace.continuation,
                        replay.terminal.history[problem.root.history.len()..],
                        "{label}"
                    );
                }
            }
        }
    }
    assert!(seen_viewer_teams.into_iter().all(|seen| seen));
}

#[test]
fn compiled_epoch_cancellation_has_no_cpu_field_query_and_device_recovers() {
    let c0 = Actor::new(SCHEMA, vec![8, 3]).unwrap();
    let c1 = Actor::new(SCHEMA, vec![12, 1]).unwrap();
    let (problem, mut compiled, policies) = fixture(Decl::NoTrump, 2, 0xabc1, &c0, &c1);
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut cancelled = Budget::new(1, Duration::from_secs(30));
    assert!(gpu
        .traces(&problem, &policies, &mut compiled, &mut cancelled)
        .unwrap()
        .is_none());
    assert_eq!(gpu.stats.epochs, 1);
    assert_eq!(gpu.stats.field_requests, 0);
    assert_eq!(gpu.stats.unique_field_queries, 0);

    let mut compiled = CompiledField::partner(problem.viewer, &c0, &c1).unwrap();
    let mut next_budget = budget();
    assert!(gpu
        .traces(&problem, &policies, &mut compiled, &mut next_budget)
        .unwrap()
        .is_some());
    assert_eq!(gpu.stats.field_requests, 0);
    assert_eq!(gpu.stats.unique_field_queries, 0);
}
