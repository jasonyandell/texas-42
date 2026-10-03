#![cfg(feature = "gpu")]

use std::time::Duration;

use walt::rules::Decl;
use walt::solver::SplitMix64;
use walt_response_ladder::compiled::{Actor, SCHEMA};
use walt_response_ladder::compiled_field::CompiledField;
use walt_response_ladder::gpu_epochs::EpochEvaluator;
use walt_response_ladder::mechanics::{tiles, Budget, DiceField, Field, Problem, PublicState};
use walt_response_ladder::model;
use walt_response_ladder::policy::{self, Policy};

fn budget() -> Budget {
    Budget::new(20_000_000, Duration::from_secs(30))
}

fn fixture(
    decl: Decl,
    partial: usize,
    seed: u64,
    actor: &Actor,
) -> (Problem, CompiledField, Policy) {
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
    let field = CompiledField::all(actor).unwrap();
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
    problem.scenarios[1].hands = problem.scenarios[0].hands;
    for (index, scenario) in problem.scenarios.iter_mut().enumerate() {
        scenario.weight = [1, 3, 7, 2][index];
    }
    let root = tiles(problem.root.legal(problem.decl, problem.hand))[0];
    let policy = Policy::forced_with_tail(
        &problem,
        root,
        tiles(problem.hand & !(1 << root)),
        actor.clone(),
    )
    .unwrap();
    (problem, field, policy)
}

#[test]
fn compiled_tail_gpu_matches_cpu_weighted_full_traces() {
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut seen_teams = [false; 2];
    for clause in 0..14 {
        let actor = Actor::new(SCHEMA, vec![clause]).unwrap();
        for (decl_index, &decl) in Decl::ALL.iter().enumerate() {
            for partial in 0..4 {
                let seed = 0x7a00
                    + clause as u64 * 0x100
                    + decl_index as u64 * 8
                    + partial as u64;
                let (problem, mut field, policy) = fixture(decl, partial, seed, &actor);
                seen_teams[(problem.viewer % 2) as usize] = true;
                let mut run_budget = budget();
                gpu.reset_stats();
                let traces = gpu
                    .traces(
                        &problem,
                        std::slice::from_ref(&policy),
                        &mut field,
                        &mut run_budget,
                    )
                    .unwrap()
                    .expect("compiled tail GPU batch completes");
                let label = format!("clause {clause} decl {decl:?} partial {partial}");
                assert_eq!(gpu.stats.field_requests, 0, "{label}");
                assert_eq!(gpu.stats.unique_field_queries, 0, "{label}");
                let mut cpu_field = CompiledField::all(&actor).unwrap();
                for (scenario, trace) in problem.scenarios.iter().zip(&traces[0]) {
                    let replay = policy::replay_scenario(
                        &problem,
                        scenario,
                        &policy,
                        &mut cpu_field,
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
    assert!(seen_teams.into_iter().all(|seen| seen));
}

#[test]
fn compiled_tail_gpu_cancellation_preserves_device_recovery() {
    let actor = Actor::new(SCHEMA, vec![8, 3]).unwrap();
    let (problem, mut field, policy) = fixture(Decl::NoTrump, 2, 0xabc1, &actor);
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut cancelled = Budget::new(1, Duration::from_secs(30));
    assert!(gpu
        .traces(
            &problem,
            std::slice::from_ref(&policy),
            &mut field,
            &mut cancelled,
        )
        .unwrap()
        .is_none());
    assert_eq!(gpu.stats.field_requests, 0);
    assert_eq!(gpu.stats.unique_field_queries, 0);

    let mut field = CompiledField::all(&actor).unwrap();
    assert!(gpu
        .traces(
            &problem,
            std::slice::from_ref(&policy),
            &mut field,
            &mut budget(),
        )
        .unwrap()
        .is_some());
}

#[test]
fn compiled_tail_gpu_is_self_contained_with_a_legacy_field() {
    let actor = Actor::new(SCHEMA, vec![8, 13]).unwrap();
    let (mut problem, _, policy) = fixture(Decl::NoTrump, 1, 0x7123, &actor);
    problem.field_revision = "historical-dice-v1".into();
    let mut gpu_field = DiceField;
    let mut gpu = EpochEvaluator::new().unwrap();
    let traces = gpu
        .traces(
            &problem,
            std::slice::from_ref(&policy),
            &mut gpu_field,
            &mut budget(),
        )
        .unwrap()
        .expect("legacy-field tail batch completes");
    assert!(gpu.stats.field_requests > 0);
    let mut cpu_field = DiceField;
    for (scenario, trace) in problem.scenarios.iter().zip(&traces[0]) {
        let replay = policy::replay_scenario(
            &problem,
            scenario,
            &policy,
            &mut cpu_field,
            &mut budget(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(trace.payoff, replay.payoff);
        assert_eq!(
            trace.continuation,
            replay.terminal.history[problem.root.history.len()..]
        );
    }
}
