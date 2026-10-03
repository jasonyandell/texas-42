//! Staged contract tests for the planned Actor::lead_follow constructor.
//! Source only: copy into tests/ after implementation review; not yet executed.
use std::time::Duration;
use walt::kernel::{Hidden, Kernel};
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::scheme::{Budget as SchemeBudget, Frame, PolicyInput, Registry};
use walt::solver::SplitMix64;
use walt_response_ladder::compiled::{Actor, SCHEMA, SCHEMA_V2};
use walt_response_ladder::compiled_family::Family;
use walt_response_ladder::compiled_field::CompiledField;
use walt_response_ladder::mechanics::{domino, tiles, Budget, Field, FieldQuery, PublicState};
use walt_response_ladder::policy::{Decision, Policy};

const LEAD_FOLLOW: &str = "scheme-relational-lead-follow-v1";

fn hands(seed: u64) -> [u32; 4] {
    let mut rng = SplitMix64(seed);
    let mut deck: Vec<u8> = (0..28).collect();
    for i in (1..deck.len()).rev() {
        deck.swap(i, rng.below((i + 1) as u64) as usize);
    }
    std::array::from_fn(|s| deck[s * 7..s * 7 + 7].iter().fold(0, |h, &t| h | (1 << t)))
}

fn with_scheme_input<R>(
    decl: Decl, original: &[u32; 4], public: &PublicState,
    history: &[(Seat, Domino)], f: impl for<'a> FnOnce(&PolicyInput<'a>) -> R,
) -> R {
    let viewer = Seat::from_index(public.actor() as usize).unwrap();
    let hand = DominoSet::from_bits(original[viewer.index()] & !public.played).unwrap();
    let mut hidden = Vec::new();
    let mut pool = DominoSet::EMPTY;
    for seat in Seat::ALL {
        if seat == viewer { continue; }
        let remaining = DominoSet::from_bits(original[seat.index()] & !public.played).unwrap();
        let voids: ContextSet = Context::ALL.into_iter().filter(|q| {
            let incidence = decl.effective_incidence(*q).bits();
            incidence != 0 && public.voids[seat.index()] & incidence == incidence
        }).collect();
        hidden.push(Hidden { seat, capacity: remaining.len(), voids });
        pool = pool.union(remaining);
    }
    let kernel = Kernel::new(decl, viewer, hand, pool, hidden.try_into().unwrap()).unwrap();
    let frame = Frame::new(kernel, Seat::from_index(public.leader as usize).unwrap(),
        public.plays.iter().copied().map(domino).collect(),
        DominoSet::from_bits(public.played).unwrap()).unwrap();
    let input = PolicyInput::new(&frame, history,
        [public.banked_t0 as u32, public.banked_t1 as u32], 30, Team::T1).unwrap();
    f(&input)
}

fn programs() -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut pairs: Vec<_> = (0..16).map(|c| (vec![c], vec![(c + 7) % 16])).collect();
    pairs.extend([
        (vec![], vec![]),
        (vec![], vec![5, 8, 2]),
        (vec![12, 1, 8], vec![]),
        (vec![12, 1, 8], vec![14, 15, 2]),
        // Same clauses may appear in both branches; uniqueness is per list.
        (vec![5, 8, 2], vec![5, 8, 2]),
    ]);
    pairs
}

#[test]
fn independent_scheme_strict_fast_and_selected_legacy_branch_agree_on_all_prefixes() {
    let pairs = programs();
    let actors: Vec<_> = pairs.iter().map(|(l, f)| Actor::lead_follow(l.clone(), f.clone()).unwrap()).collect();
    let exports: Vec<_> = actors.iter().map(|a|
        a.policy_program().unwrap().compile(&Registry::standard()).unwrap()).collect();
    let branches: Vec<_> = pairs.iter().map(|(l, f)| (
        Actor::new(SCHEMA_V2, l.clone()).unwrap(), Actor::new(SCHEMA_V2, f.clone()).unwrap(),
    )).collect();
    let mut counts = [0usize; 4];
    let mut distinct_branches = [0usize; 2];
    for (decl_index, decl) in Decl::ALL.into_iter().enumerate() {
        for leader in 0..4u8 {
            let original = hands(0x1ead_f011 + decl_index as u64 * 101 + u64::from(leader));
            let mut public = PublicState::opening(leader);
            let mut history = Vec::new();
            for ply in 0..28 {
                counts[public.plays.len()] += 1;
                let viewer = public.actor();
                let hand = original[viewer as usize] & !public.played;
                let leading = public.plays.is_empty();
                // The Scheme viewer/leader guards have this equivalence only
                // on valid on-turn inputs, which every case constructs.
                assert_eq!(viewer == public.leader, leading);
                for (i, actor) in actors.iter().enumerate() {
                    let (lead, follow) = &branches[i];
                    let expected = if leading { lead } else { follow }
                        .choose(decl, viewer, hand, &public).unwrap();
                    let other = if leading { follow } else { lead }
                        .choose(decl, viewer, hand, &public).unwrap();
                    distinct_branches[usize::from(!leading)] += usize::from(expected.action != other.action);
                    let strict = actor.choose(decl, viewer, hand, &public).unwrap();
                    assert_eq!(strict.action, expected.action, "{decl:?} leader{leader} ply{ply} actor{i}");
                    let scheme = with_scheme_input(decl, &original, &public, &history, |input| {
                        let mut b = SchemeBudget::new(100_000);
                        let mut controller = exports[i].initialize(input, &mut b).unwrap();
                        exports[i].choose_traced(&mut controller, input, &mut b).unwrap()
                    });
                    assert_eq!(strict.action, scheme.action.index() as u8);
                    assert_eq!(strict.provenance, scheme.provenance);
                    let mut field = CompiledField::all(actor).unwrap();
                    for tape in [0, u64::MAX] {
                        let fast = field.choose(FieldQuery { decl, bid: 30, seat: viewer,
                            hand, public: &public, tape }, &mut Budget::new(2, Duration::from_secs(1)));
                        assert_eq!(fast, Some(expected.action));
                    }
                    // Policy::choose is the fast focal-tail path. A materialized
                    // information-set decision must continue to override it.
                    let mut policy = Policy::priority(viewer, tiles(hand).into_iter().rev().collect());
                    policy.compiled_tail = Some(actor.clone());
                    assert_eq!(policy.choose(decl, &public, hand), Some(expected.action));
                    let override_tile = *tiles(public.legal(decl, hand)).last().unwrap();
                    policy.decisions.push(Decision { history: public.history.clone(), action: override_tile });
                    assert_eq!(policy.choose(decl, &public, hand), Some(override_tile));
                }
                let tile = tiles(public.legal(decl, hand))[0];
                history.push((Seat::from_index(viewer as usize).unwrap(), domino(tile)));
                public = public.after(decl, tile);
            }
        }
    }
    assert_eq!(counts, [9 * 4 * 7; 4]);
    assert!(distinct_branches.into_iter().all(|n| n > 0), "both branch guards must have distinguishing witnesses");
}

#[test]
fn mixed_family_scheme_guards_compose_with_lead_follow_guards() {
    let family = Family::Roles {
        declaring: Actor::lead_follow(vec![12, 1, 8], vec![14, 15, 2]).unwrap(),
        defending: Actor::new(SCHEMA, vec![5, 8, 2]).unwrap(),
    };
    let roundtrip: Family = serde_json::from_value(serde_json::to_value(&family).unwrap()).unwrap();
    assert_eq!(roundtrip, family);
    let exported = family.policy_program_named("lead-follow-role-contract").unwrap()
        .compile(&Registry::standard()).unwrap();
    for (di, decl) in Decl::ALL.into_iter().enumerate() {
        for leader in 0..4u8 {
            let original = hands(0xfa11_1ead + di as u64 * 29 + u64::from(leader));
            let mut public = PublicState::opening(leader);
            let mut history = Vec::new();
            let mut field = CompiledField::all_family(&family).unwrap();
            for _ in 0..28 {
                let viewer = public.actor();
                let hand = original[viewer as usize] & !public.played;
                let expected = family.actor(viewer).unwrap().choose(decl, viewer, hand, &public).unwrap().action;
                let actual = with_scheme_input(decl, &original, &public, &history, |input| {
                    let mut b = SchemeBudget::new(100_000);
                    let mut controller = exported.initialize(input, &mut b).unwrap();
                    exported.choose_traced(&mut controller, input, &mut b).unwrap().action.index() as u8
                });
                assert_eq!(actual, expected);
                assert_eq!(field.choose(FieldQuery { decl, bid: 30, seat: viewer,
                    hand, public: &public, tape: 17 }, &mut Budget::new(2, Duration::from_secs(1))), Some(expected));
                let tile = tiles(public.legal(decl, hand))[0];
                history.push((Seat::from_index(viewer as usize).unwrap(), domino(tile)));
                public = public.after(decl, tile);
            }
        }
    }
}

fn rejects_wire(value: serde_json::Value) -> bool {
    match serde_json::from_value::<Actor>(value) {
        Err(_) => true,
        Ok(actor) => actor.validate().is_err(),
    }
}

#[test]
fn wire_schema_validation_and_legacy_compatibility_are_explicit() {
    let actor = Actor::lead_follow(vec![15, 0, 7], vec![14, 15, 2]).unwrap();
    let wire = serde_json::to_value(&actor).unwrap();
    assert_eq!(wire["schema"], LEAD_FOLLOW);
    assert_eq!(wire["clauses"], serde_json::json!([14, 15, 2]));
    assert_eq!(wire["lead_clauses"], serde_json::json!([15, 0, 7]));
    assert_eq!(serde_json::from_value::<Actor>(wire).unwrap(), actor);
    for schema in [SCHEMA, SCHEMA_V2] {
        let legacy: Actor = serde_json::from_value(serde_json::json!({"schema":schema,"clauses":[0]})).unwrap();
        legacy.validate().unwrap();
        assert_eq!(legacy, Actor::new(schema, vec![0]).unwrap());
        let encoded = serde_json::to_value(legacy).unwrap();
        assert!(encoded.get("lead_clauses").is_none_or(|v| v.is_null()));
        assert!(rejects_wire(serde_json::json!({"schema":schema,"clauses":[],"lead_clauses":[]})));
    }
    assert!(rejects_wire(serde_json::json!({"schema":"unknown","clauses":[],"lead_clauses":[]})));
    assert!(rejects_wire(serde_json::json!({"schema":LEAD_FOLLOW,"clauses":[]})));
    assert!(rejects_wire(serde_json::json!({"schema":LEAD_FOLLOW,"clauses":[],"lead_clauses":null})));
    for invalid in [vec![16], vec![0, 0], vec![0, 1, 2, 3]] {
        assert!(Actor::lead_follow(invalid.clone(), vec![]).is_err());
        assert!(Actor::lead_follow(vec![], invalid.clone()).is_err());
        assert!(rejects_wire(serde_json::json!({"schema":LEAD_FOLLOW,"clauses":[],"lead_clauses":invalid})));
        assert!(rejects_wire(serde_json::json!({"schema":LEAD_FOLLOW,"clauses":invalid,"lead_clauses":[]})));
    }
    assert!(Actor::lead_follow(vec![0, 1, 2], vec![0, 1, 2]).is_ok());
    assert!(Actor::lead_follow(vec![], vec![]).is_ok());
}

#[cfg(feature = "gpu")]
mod gpu {
    use super::*;
    use walt_response_ladder::core::PolicyEvaluator;
    use walt_response_ladder::gpu_epochs::EpochEvaluator;
    use walt_response_ladder::mechanics::{DiceField, Problem};
    use walt_response_ladder::model;
    use walt_response_ladder::policy;

    fn budget() -> Budget { Budget::new(20_000_000, Duration::from_secs(30)) }

    fn fixture(decl: Decl, partial: usize, seed: u64, identity: String) -> Problem {
        let original = hands(seed);
        let mut public = PublicState::opening((seed % 4) as u8);
        // Three or four tricks left, including every partial-root shape.
        let depth = 3 + (seed as usize % 2);
        for _ in 0..4 * (7 - depth) + partial {
            let actor = public.actor();
            let tile = tiles(public.legal(decl, original[actor as usize]))[0];
            public = public.after(decl, tile);
        }
        let viewer = public.actor();
        let bid = public.banked_t1 + (42 - public.banked_t1 - public.banked_t0).div_ceil(2);
        let mut problem = model::sample_problem(decl, bid, viewer,
            original[viewer as usize] & !public.played, &public, 4, seed ^ 0x5eed,
            identity, &mut budget()).unwrap().unwrap();
        problem.scenarios[1].hands = problem.scenarios[0].hands;
        for (i, scenario) in problem.scenarios.iter_mut().enumerate() { scenario.weight = [1, 3, 7, 2][i]; }
        problem
    }

    #[test]
    fn mixed_role_fields_and_focal_tails_match_complete_gpu_traces_and_weighted_prices() {
        let split_a = Actor::lead_follow(vec![12, 1, 8], vec![14, 15, 2]).unwrap();
        let split_b = Actor::lead_follow(vec![5, 8, 2], vec![15, 14, 0]).unwrap();
        let legacy_v1 = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
        let legacy_v2 = Actor::new(SCHEMA_V2, vec![14, 15, 2]).unwrap();
        let c0 = Family::Roles { declaring: legacy_v1.clone(), defending: split_a.clone() };
        let c1 = Family::Roles { declaring: split_b.clone(), defending: legacy_v2.clone() };
        let mut gpu = EpochEvaluator::new().unwrap();
        let mut roles = [false; 2];
        for (di, decl) in Decl::ALL.into_iter().enumerate() {
            for partial in 0..4 {
                let mut problem = fixture(decl, partial, 0x1ead_0000 + di as u64 * 17 + partial as u64, "pending".into());
                roles[(problem.viewer % 2) as usize] = true;
                for (opponents, partner) in [(&c0, &c1), (&c1, &c0)] {
                    let mut field = CompiledField::partner_family(problem.viewer, opponents, partner).unwrap();
                    problem.field_revision = field.revision().to_owned();
                    let root = tiles(problem.root.legal(decl, problem.hand))[0];
                    let order = tiles(problem.hand & !(1 << root));
                    let mut policies = vec![Policy::forced(&problem, root, order.clone())];
                    for tail in [&split_a, &split_b, &legacy_v1, &legacy_v2] {
                        policies.push(Policy::forced_with_tail(&problem, root, order.clone(), tail.clone()).unwrap());
                    }
                    gpu.reset_stats();
                    let traces = gpu.traces(&problem, &policies, &mut field, &mut budget()).unwrap().unwrap();
                    assert_eq!(gpu.stats.field_requests, 0, "compiled field must remain self-contained");
                    assert_eq!(gpu.stats.unique_field_queries, 0);
                    let mut expected_prices = Vec::new();
                    for (policy, traces) in policies.iter().zip(&traces) {
                        let mut scalar = CompiledField::partner_family(problem.viewer, opponents, partner).unwrap();
                        let mut weighted = 0;
                        for (scenario, trace) in problem.scenarios.iter().zip(traces) {
                            let replay = policy::replay_scenario(&problem, scenario, policy, &mut scalar, &mut budget()).unwrap().unwrap();
                            assert_eq!(trace.payoff, replay.payoff, "{decl:?} partial{partial}");
                            assert_eq!(trace.continuation, replay.terminal.history[problem.root.history.len()..]);
                            weighted += scenario.weight * replay.payoff;
                        }
                        expected_prices.push(weighted);
                    }
                    let prices = gpu.prices(&problem, &policies, &mut field, &mut budget()).unwrap().unwrap();
                    assert_eq!(prices, expected_prices);
                }
            }
        }
        assert!(roles.into_iter().all(|seen| seen));
    }

    #[test]
    fn lead_follow_focal_tail_with_dice_field_retains_original_tapes() {
        let tail = Actor::lead_follow(vec![8, 3], vec![14, 15, 2]).unwrap();
        let problem = fixture(Decl::NoTrump, 2, 0x1ead_7777, "historical-dice-v1".into());
        let root = tiles(problem.root.legal(problem.decl, problem.hand))[0];
        let policy = Policy::forced_with_tail(&problem, root, tiles(problem.hand & !(1 << root)), tail).unwrap();
        let mut gpu = EpochEvaluator::new().unwrap();
        let got = gpu.traces(&problem, std::slice::from_ref(&policy), &mut DiceField, &mut budget()).unwrap().unwrap();
        for (scenario, trace) in problem.scenarios.iter().zip(&got[0]) {
            let replay = policy::replay_scenario(&problem, scenario, &policy, &mut DiceField, &mut budget()).unwrap().unwrap();
            assert_eq!(trace.payoff, replay.payoff);
            assert_eq!(trace.continuation, replay.terminal.history[problem.root.history.len()..]);
        }
    }
}
