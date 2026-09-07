use walt::policy_search::{self, HashField, Search, TablePolicy, Work};
use walt::rules::{legal_plays, Team};
use walt::scheme::{
    Budget, DecisionProvenance, ExactRule, Fallback, PolicyInput, PolicyKey, PolicyProgram,
    PolicyRule, Registry, Selector,
};

fn input(fixture: &policy_search::Fixture) -> PolicyInput<'_> {
    PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        fixture.exercise.position.banked,
        fixture.exercise.position.bid as u8,
        fixture.exercise.position.declaring_team,
    )
    .unwrap()
}

fn empty_program(name: &str) -> PolicyProgram {
    PolicyProgram {
        name: name.into(),
        initial_mode: "steady".into(),
        bindings: vec![],
        exact_rules: vec![],
        rules: vec![],
        fallback: Fallback::LowestLegal,
    }
}

fn own_legal_program(name: &str) -> PolicyProgram {
    let mut program = empty_program(name);
    program.rules.push(PolicyRule {
        name: "own-legal".into(),
        in_mode: "steady".into(),
        next_mode: "steady".into(),
        selector: Selector::FirstOutput,
        guard: "(fix (roles (domino action)) (out action) (case (own-legal action)))"
            .parse()
            .unwrap(),
    });
    program
}

#[test]
fn choose_trace_distinguishes_exact_relation_and_final_fallback() {
    let exact_fixture = policy_search::fixture(23, "P6".parse().unwrap(), 7, None).unwrap();
    let other_fixture = policy_search::fixture(37, "P6".parse().unwrap(), 7, None).unwrap();
    let exact_input = input(&exact_fixture);
    let other_input = input(&other_fixture);
    let exact_action = exact_fixture
        .exercise
        .frame
        .kernel()
        .viewer_hand()
        .iter()
        .next()
        .unwrap();
    let mut table = empty_program("frozen-table");
    table.exact_rules.push(ExactRule {
        key: PolicyKey::from_input(&exact_input),
        action: exact_action,
    });
    let hybrid = policy_search::program::combine_exact_table_with_relational(
        &table,
        &own_legal_program("shared-relation"),
        "hybrid",
    )
    .unwrap();
    let compiled = hybrid.compile(&Registry::standard()).unwrap();

    let mut exact_controller = compiled
        .initialize(&exact_input, &mut Budget::new(100_000))
        .unwrap();
    let exact = compiled
        .choose_traced(
            &mut exact_controller,
            &exact_input,
            &mut Budget::new(100_000),
        )
        .unwrap();
    assert_eq!(exact.action, exact_action);
    assert_eq!(exact.provenance, DecisionProvenance::Exact);
    assert_eq!(exact.mode_before, "steady");
    assert_eq!(exact.mode_after, "steady");

    let mut relational_controller = compiled
        .initialize(&other_input, &mut Budget::new(100_000))
        .unwrap();
    let mut relational_budget = Budget::new(100_000);
    let relational = compiled
        .choose_traced(
            &mut relational_controller,
            &other_input,
            &mut relational_budget,
        )
        .unwrap();
    assert_eq!(
        relational.provenance,
        DecisionProvenance::RelationalRule {
            name: "own-legal".into()
        }
    );
    assert!(relational_budget.spent() > 0);

    let fallback = empty_program("fallback")
        .compile(&Registry::standard())
        .unwrap();
    let mut fallback_controller = fallback
        .initialize(&other_input, &mut Budget::new(100_000))
        .unwrap();
    let traced = fallback
        .choose_traced(
            &mut fallback_controller,
            &other_input,
            &mut Budget::new(100_000),
        )
        .unwrap();
    assert_eq!(traced.provenance, DecisionProvenance::Fallback);

    let mut ordinary_controller = fallback
        .initialize(&other_input, &mut Budget::new(100_000))
        .unwrap();
    assert_eq!(
        fallback
            .choose(
                &mut ordinary_controller,
                &other_input,
                &mut Budget::new(100_000),
            )
            .unwrap(),
        traced.action
    );
    assert_eq!(ordinary_controller.state(), fallback_controller.state());
}

#[test]
fn traced_budget_refusal_is_atomic_and_world_access_stays_excluded() {
    let fixture = policy_search::fixture(31, "P6".parse().unwrap(), 7, None).unwrap();
    let input = input(&fixture);
    let mut stateful = own_legal_program("budgeted");
    stateful.rules[0].next_mode = "changed".into();
    let compiled = stateful.compile(&Registry::standard()).unwrap();
    let mut controller = compiled
        .initialize(&input, &mut Budget::new(100_000))
        .unwrap();
    let before = controller.state();
    let mut budget = Budget::new(1);
    assert!(compiled
        .choose_traced(&mut controller, &input, &mut budget)
        .is_err());
    assert_eq!(controller.state(), before);
    assert!(
        budget.spent() > 0,
        "completed work remains honestly charged"
    );
    let transition = compiled
        .choose_traced(&mut controller, &input, &mut Budget::new(100_000))
        .unwrap();
    assert_eq!(transition.mode_before, "steady");
    assert_eq!(transition.mode_after, "changed");
    assert_eq!(controller.mode(), "changed");

    let mut peeking = empty_program("peeking");
    peeking.rules.push(PolicyRule {
        name: "peek".into(),
        in_mode: "steady".into(),
        next_mode: "steady".into(),
        selector: Selector::FirstOutput,
        guard: "(fix (roles (domino action)) (out action) (case (holds S1 action)))"
            .parse()
            .unwrap(),
    });
    assert!(peeking.compile(&Registry::standard()).is_err());
}

#[test]
fn hybrid_helper_refuses_stateful_or_contaminated_sources() {
    let fixture = policy_search::fixture(17, "P6".parse().unwrap(), 7, None).unwrap();
    let input = input(&fixture);
    let action = fixture
        .exercise
        .frame
        .kernel()
        .viewer_hand()
        .iter()
        .next()
        .unwrap();
    let mut exact = empty_program("exact");
    exact.exact_rules.push(ExactRule {
        key: PolicyKey::from_input(&input),
        action,
    });
    let relational = own_legal_program("relational");
    let combined = policy_search::program::combine_exact_table_with_relational(
        &exact,
        &relational,
        "combined",
    )
    .unwrap();
    assert_eq!(combined.exact_rules, exact.exact_rules);
    assert_eq!(combined.rules, relational.rules);

    let mut contaminated_exact = exact.clone();
    contaminated_exact.rules = relational.rules.clone();
    assert!(policy_search::program::combine_exact_table_with_relational(
        &contaminated_exact,
        &relational,
        "bad-exact"
    )
    .is_err());

    let mut contaminated_relation = relational.clone();
    contaminated_relation.exact_rules = exact.exact_rules.clone();
    assert!(policy_search::program::combine_exact_table_with_relational(
        &exact,
        &contaminated_relation,
        "bad-relation"
    )
    .is_err());

    let mut stateful = relational;
    stateful.rules[0].next_mode = "changed".into();
    assert!(policy_search::program::combine_exact_table_with_relational(
        &exact,
        &stateful,
        "bad-state"
    )
    .is_err());
}

#[test]
fn independent_replay_reports_depth_provenance_work_and_controller_state() {
    let fixture = policy_search::fixture(17, "P6".parse().unwrap(), 2, None).unwrap();
    let world = fixture.exercise.root.worlds().next().unwrap();
    let mut search = Search::new(&fixture.exercise, &HashField);
    search.append(world).unwrap();
    let node = search.solve(&mut Work::new(1_000_000)).unwrap();
    let table = TablePolicy::from_node(&node);
    let exact = policy_search::program::export(&fixture, &table, "exact").unwrap();
    let hybrid = policy_search::program::combine_exact_table_with_relational(
        &exact,
        &own_legal_program("relation"),
        "hybrid-replay",
    )
    .unwrap();
    let compiled = hybrid.compile(&Registry::standard()).unwrap();
    let replay =
        policy_search::program::replay_program_traced(&fixture, &HashField, &world, &compiled)
            .unwrap();
    let direct = policy_search::replay(&fixture.exercise, &HashField, &world, &table).unwrap();
    assert_eq!((replay.made, replay.plays.clone()), direct);
    assert_eq!(replay.initialization_work, 0);
    assert_eq!(replay.total_policy_work, 0);
    assert!(!replay.focal_decisions.is_empty());
    for (depth, decision) in replay.focal_decisions.iter().enumerate() {
        assert_eq!(decision.focal_decision_depth, depth);
        assert_eq!(decision.provenance, DecisionProvenance::Exact);
        assert_eq!(decision.policy_work_used, 0);
        assert_eq!(decision.controller_before, decision.controller_after);
        assert_eq!(decision.controller_before.mode, "steady");
    }

    let mut state = policy_search::State::from_root(&fixture.exercise.position);
    let mut focal = replay.focal_decisions.iter();
    for (seat, tile) in &replay.plays {
        assert_eq!(*seat, state.actor());
        let hand = world.hand(*seat).difference(state.played);
        let led = state
            .prefix
            .first()
            .map(|lead| fixture.exercise.position.decl.led_context(*lead));
        assert!(legal_plays(fixture.exercise.position.decl, hand, led).contains(*tile));
        if *seat == fixture.exercise.root.kernel().viewer() {
            let decision = focal.next().expect("every focal play has a trace row");
            assert_eq!(decision.action, *tile);
            assert_eq!(
                decision.contract_resolved,
                state.success(&fixture.exercise.position)
            );
        }
        state = state.step(fixture.exercise.position.decl, *tile);
    }
    assert!(focal.next().is_none());
    assert_eq!(state.played.len(), 28);
}

#[test]
fn packet_partner_count_candidate_is_a_lawful_current_syntax_program() {
    let source = include_str!(
        "../../../experiments/partnership/packet/texas42_relational_learning/partner-count-candidate.scheme"
    );
    let program: PolicyProgram = source.parse().unwrap();
    assert!(program.exact_rules.is_empty());
    assert!(program.bindings.is_empty());
    assert!(program
        .rules
        .iter()
        .all(|rule| rule.in_mode == "play" && rule.next_mode == "play"));
    program.compile(&Registry::standard()).unwrap();
}

#[test]
fn contract_resolution_is_reported_from_banked_public_state() {
    let fixture = policy_search::fixture(23, "P6".parse().unwrap(), 7, None).unwrap();
    let unresolved = PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        fixture.exercise.position.banked,
        42,
        Team::T0,
    )
    .unwrap();
    let compiled = empty_program("contract-status")
        .compile(&Registry::standard())
        .unwrap();
    let mut controller = compiled
        .initialize(&unresolved, &mut Budget::new(100_000))
        .unwrap();
    let trace = compiled
        .choose_traced(&mut controller, &unresolved, &mut Budget::new(100_000))
        .unwrap();
    let total = unresolved.banked.iter().sum::<u32>();
    let declared = unresolved.banked[Team::T0.index()];
    let expected = if declared >= 42 {
        Some(true)
    } else if declared + (42 - total) < 42 {
        Some(false)
    } else {
        None
    };
    assert_eq!(trace.contract_resolved, expected);
}
