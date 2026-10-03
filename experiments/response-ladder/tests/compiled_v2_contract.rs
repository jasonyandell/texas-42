use walt::kernel::{Hidden, Kernel};
use walt::rules::rules::legal_plays;
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::scheme::{Budget, DecisionProvenance, Frame, PolicyInput, Registry};

use walt_response_ladder::compiled::{
    clause_actions, clause_actions_for_schema, clause_actions_v2, Actor, CLAUSE_COUNT,
    CLAUSE_COUNT_V2, SCHEMA, SCHEMA_V2,
};
use walt_response_ladder::mechanics::{domino, PublicState};

fn mask(tiles: impl IntoIterator<Item = u8>) -> u32 {
    tiles.into_iter().map(|tile| 1u32 << tile).sum()
}

fn scheme_choice(
    decl: Decl,
    viewer: u8,
    original: &[u32; 4],
    public: &PublicState,
    history: &[(Seat, Domino)],
    clause: usize,
) -> (u8, DecisionProvenance) {
    let actor = Actor::new(SCHEMA_V2, vec![clause]).unwrap();
    let program = actor.policy_program().unwrap();
    let compiled = program.compile(&Registry::standard()).unwrap();
    let viewer_seat = Seat::from_index(viewer as usize).unwrap();
    let viewer_hand = DominoSet::from_bits(original[viewer as usize] & !public.played).unwrap();
    let mut hidden = Vec::new();
    let mut pool = DominoSet::EMPTY;
    for seat in Seat::ALL {
        if seat == viewer_seat { continue; }
        let remaining = DominoSet::from_bits(original[seat.index()] & !public.played).unwrap();
        let voids: ContextSet = Context::ALL.into_iter().filter(|q| {
            let incidence = decl.effective_incidence(*q).bits();
            incidence != 0 && public.voids[seat.index()] & incidence == incidence
        }).collect();
        hidden.push(Hidden { seat, capacity: remaining.len(), voids });
        pool = pool.union(remaining);
    }
    let kernel = Kernel::new(decl, viewer_seat, viewer_hand, pool, hidden.try_into().unwrap()).unwrap();
    let frame = Frame::new(
        kernel,
        Seat::from_index(public.leader as usize).unwrap(),
        public.plays.iter().copied().map(domino).collect(),
        DominoSet::from_bits(public.played).unwrap(),
    ).unwrap();
    let input = PolicyInput::new(&frame, history, [public.banked_t0 as u32, public.banked_t1 as u32], 30, Team::T1).unwrap();
    let mut budget = Budget::new(100_000);
    let mut controller = compiled.initialize(&input, &mut budget).unwrap();
    let trace = compiled.choose_traced(&mut controller, &input, &mut budget).unwrap();
    (trace.action.index() as u8, trace.provenance)
}

#[test]
fn v2_scalar_matches_v1_prefix_and_scheme_for_all_declarations() {
    let original = [mask(0..7), mask(7..14), mask(14..21), mask(21..28)];
    for decl in Decl::ALL {
        for leader in 0..4u8 {
            let mut remaining = original;
            let mut public = PublicState::opening(leader);
            let mut history = Vec::new();
            while history.len() < Domino::COUNT {
                let viewer = public.actor();
                let hand = remaining[viewer as usize] & !public.played;
                let v1 = clause_actions(decl, viewer, hand, &public).unwrap();
                let v2 = clause_actions_v2(decl, viewer, hand, &public).unwrap();
                assert_eq!(&v2[..CLAUSE_COUNT], &v1[..]);
                let dispatched = clause_actions_for_schema(SCHEMA_V2, decl, viewer, hand, &public).unwrap();
                assert_eq!(dispatched, v2.to_vec());
                for clause in [14usize, 15usize] {
                    let choice = Actor::new(SCHEMA_V2, vec![clause]).unwrap().choose(decl, viewer, hand, &public).unwrap();
                    let scheme = scheme_choice(decl, viewer, &original, &public, &history, clause);
                    assert_eq!(choice.action, scheme.0, "{decl:?} leader {leader} clause {clause}");
                    assert_eq!(choice.provenance, scheme.1, "{decl:?} leader {leader} clause {clause}");
                    assert_eq!(v2[clause], Some(scheme.0).filter(|_| !matches!(scheme.1, DecisionProvenance::Fallback)));
                }
                let legal = legal_plays(decl, DominoSet::from_bits(hand).unwrap(), public.plays.first().map(|tile| decl.led_context(domino(*tile))));
                let tile = legal.iter().next().unwrap().index() as u8;
                remaining[viewer as usize] &= !(1u32 << tile);
                history.push((Seat::from_index(viewer as usize).unwrap(), domino(tile)));
                public = public.after(decl, tile);
            }
        }
    }
}

#[test]
fn v2_schema_dispatch_and_legacy_rejection_are_explicit() {
    assert_eq!(CLAUSE_COUNT_V2, 16);
    assert_eq!(clause_actions_for_schema(SCHEMA, Decl::NoTrump, 0, 1, &PublicState::opening(0)).unwrap().len(), CLAUSE_COUNT);
    assert_eq!(clause_actions_for_schema(SCHEMA_V2, Decl::NoTrump, 0, 1, &PublicState::opening(0)).unwrap().len(), CLAUSE_COUNT_V2);
    assert!(Actor::new(SCHEMA, vec![14]).is_err());
    assert!(Actor::new(SCHEMA, vec![15]).is_err());
    assert!(Actor::new(SCHEMA_V2, vec![14, 15, 0]).is_ok());
    assert!(Actor::new(SCHEMA_V2, vec![16]).is_err());
    assert!(clause_actions_for_schema("unknown", Decl::NoTrump, 0, 1, &PublicState::opening(0)).is_err());
}

#[cfg(feature = "gpu")]
#[test]
fn gpu_mixed_v1_v2_compiled_tails_keep_the_same_trace_contract() {
    use std::time::Duration;
    use walt::solver::SplitMix64;
    use walt_response_ladder::compiled_field::CompiledField;
    use walt_response_ladder::gpu_epochs::EpochEvaluator;
    use walt_response_ladder::mechanics::{tiles, Budget, DiceField, Field};
    use walt_response_ladder::model;
    use walt_response_ladder::policy::{self, Policy};

    let mut rng = SplitMix64(0x42_16);
    let mut deck: Vec<u8> = (0..28).collect();
    for i in (1..28).rev() { deck.swap(i, rng.below((i + 1) as u64) as usize); }
    let hands: [u32; 4] = std::array::from_fn(|seat| deck[seat * 7..seat * 7 + 7].iter().fold(0, |m, &t| m | (1 << t)));
    let decl = Decl::NoTrump;
    let mut public = PublicState::opening(1);
    for _ in 0..9 { let t = tiles(public.legal(decl, hands[public.actor() as usize]))[0]; public = public.after(decl, t); }
    let viewer = public.actor();
    let mut sample_budget = Budget::new(20_000_000, Duration::from_secs(10));
    let mut problem = model::sample_problem(decl, 30, viewer, hands[viewer as usize] & !public.played, &public, 2, 0x77, "v2-mixed".into(), &mut sample_budget).unwrap().unwrap();
    let v1 = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
    let v2 = Actor::new(SCHEMA_V2, vec![14, 15, 2]).unwrap();
    let mut field = CompiledField::partner(viewer, &v1, &v2).unwrap();
    problem.field_revision = field.revision().to_owned();
    let root = tiles(problem.root.legal(problem.decl, problem.hand))[0];
    let policy = Policy::forced_with_tail(&problem, root, tiles(problem.hand & !(1 << root)), v2.clone()).unwrap();
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut budget = Budget::new(20_000_000, Duration::from_secs(30));
    let traces = gpu.traces(&problem, std::slice::from_ref(&policy), &mut field, &mut budget).unwrap().unwrap();
    let mut cpu_field = CompiledField::partner(viewer, &v1, &v2).unwrap();
    for (scenario, trace) in problem.scenarios.iter().zip(&traces[0]) {
        let replay = policy::replay_scenario(&problem, scenario, &policy, &mut cpu_field, &mut Budget::new(20_000_000, Duration::from_secs(30))).unwrap().unwrap();
        assert_eq!(trace.payoff, replay.payoff);
        assert_eq!(trace.continuation, replay.terminal.history[problem.root.history.len()..]);
    }
    problem.field_revision = "historical-dice-v1".into();
    let mut dice = DiceField;
    let mut gpu = EpochEvaluator::new().unwrap();
    let mut budget = Budget::new(20_000_000, Duration::from_secs(30));
    assert!(gpu.traces(&problem, std::slice::from_ref(&policy), &mut dice, &mut budget).unwrap().is_some());
}
