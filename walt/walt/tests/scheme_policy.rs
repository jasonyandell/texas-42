use walt::policy_search::{self, HashField, Search, TablePolicy, Work};
use walt::rules::{Domino, DominoSet, Team};
use walt::scheme::{
    Budget, ExactRule, Fallback, PolicyInput, PolicyKey, PolicyProgram, PolicyRule, Registry,
    RigidBinding, Selector,
};

fn tile(source: &str) -> Domino {
    source.parse().unwrap()
}
fn program(key: PolicyKey) -> PolicyProgram {
    PolicyProgram {
        name: "table-and-relation".into(),
        initial_mode: "fresh".into(),
        bindings: vec![],
        exact_rules: vec![ExactRule {
            key,
            action: tile("6-6"),
        }],
        rules: vec![PolicyRule {
            name: "own-legal".into(),
            in_mode: "fresh".into(),
            next_mode: "persistent".into(),
            selector: Selector::FirstOutput,
            guard: "(fix (roles (domino action)) (out action) (case (own-legal action)))"
                .parse()
                .unwrap(),
        }],
        fallback: Fallback::LowestLegal,
    }
}

#[test]
fn text_roundtrip_preserves_exact_information_key() {
    let fixture = policy_search::fixture(29, "P6".parse().unwrap(), 7, None).unwrap();
    let input = PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        [0, 0],
        31,
        Team::T0,
    )
    .unwrap();
    let source = program(PolicyKey::from_input(&input));
    assert_eq!(source.to_string().parse::<PolicyProgram>().unwrap(), source);
}

#[test]
fn exact_rule_precedes_relation_and_is_checked_for_legality() {
    let fixture = policy_search::fixture(23, "P6".parse().unwrap(), 7, None).unwrap();
    let frame = &fixture.exercise.frame;
    let input = PolicyInput::new(frame, &fixture.history, [0, 0], 30, Team::T0).unwrap();
    let mut source = program(PolicyKey::from_input(&input));
    source.exact_rules[0].action = frame
        .kernel()
        .viewer_hand()
        .iter()
        .nth(frame.kernel().viewer_hand().len() - 1)
        .unwrap();
    let compiled = source.compile(&Registry::standard()).unwrap();
    let mut budget = Budget::new(100_000);
    let mut controller = compiled.initialize(&input, &mut budget).unwrap();
    assert_eq!(
        compiled
            .choose(&mut controller, &input, &mut budget)
            .unwrap(),
        source.exact_rules[0].action
    );

    source.exact_rules[0].action = DominoSet::FULL
        .difference(frame.kernel().viewer_hand())
        .iter()
        .next()
        .unwrap();
    assert!(source.compile(&Registry::standard()).is_err());
}

#[test]
fn world_predicates_are_refused_and_fallback_is_total() {
    let fixture = policy_search::fixture(31, "P6".parse().unwrap(), 7, None).unwrap();
    let frame = &fixture.exercise.frame;
    let input = PolicyInput::new(frame, &fixture.history, [0, 0], 30, Team::T0).unwrap();
    let mut source = program(PolicyKey::from_input(&input));
    source.exact_rules.clear();
    source.rules.clear();
    let compiled = source.compile(&Registry::standard()).unwrap();
    let mut budget = Budget::new(100_000);
    let mut controller = compiled.initialize(&input, &mut budget).unwrap();
    assert_eq!(
        compiled
            .choose(&mut controller, &input, &mut budget)
            .unwrap(),
        tile("0-0")
    );

    source.rules.push(PolicyRule {
        name: "peek".into(),
        in_mode: "fresh".into(),
        next_mode: "fresh".into(),
        selector: Selector::FirstOutput,
        guard: "(fix (roles (domino action)) (out action) (case (holds S1 action)))"
            .parse()
            .unwrap(),
    });
    assert!(source.compile(&Registry::standard()).is_err());
}

#[test]
fn sampled_table_exports_and_replays_through_serialized_program() {
    let fixture = policy_search::fixture(17, "P6".parse().unwrap(), 2, None).unwrap();
    let world = fixture.exercise.root.worlds().next().unwrap();
    let mut search = Search::new(&fixture.exercise, &HashField);
    search.append(world).unwrap();
    let node = search.solve(&mut Work::new(1_000_000)).unwrap();
    let table = TablePolicy::from_node(&node);
    let program = policy_search::program::export(&fixture, &table, "exported-table").unwrap();
    let text = program.to_string();
    let parsed: PolicyProgram = text.parse().unwrap();
    let compiled = parsed.compile(&Registry::standard()).unwrap();
    let direct = policy_search::replay(&fixture.exercise, &HashField, &world, &table).unwrap();
    let interchange =
        policy_search::program::replay_program(&fixture, &HashField, &world, &compiled).unwrap();
    assert_eq!(interchange, direct);
}

#[test]
fn controller_provenance_and_mode_updates_are_transactional() {
    let fixture = policy_search::fixture(37, "P6".parse().unwrap(), 7, None).unwrap();
    let input = PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        [0, 0],
        30,
        Team::T0,
    )
    .unwrap();
    let mut first = program(PolicyKey::from_input(&input));
    first.exact_rules.clear();
    let mut second = first.clone();
    second.name = "other-program".into();
    let first = first.compile(&Registry::standard()).unwrap();
    let second = second.compile(&Registry::standard()).unwrap();
    let mut controller = first.initialize(&input, &mut Budget::new(10_000)).unwrap();
    assert!(second
        .choose(&mut controller, &input, &mut Budget::new(10_000))
        .is_err());
    assert_eq!(controller.mode(), "fresh");
    assert!(first
        .choose(&mut controller, &input, &mut Budget::new(0))
        .is_err());
    assert_eq!(controller.mode(), "fresh");
}

#[test]
fn malformed_schema_history_and_empty_binding_are_refused() {
    for source in [
        "(policy p (initial a) (initial b) (fallback lowest-legal))",
        "(policy p (initial a) (fallback lowest-legal) (mystery x))",
    ] {
        assert!(source.parse::<PolicyProgram>().is_err(), "{source}");
    }

    let fixture = policy_search::fixture(41, "P6".parse().unwrap(), 6, None).unwrap();
    let mut wrong = fixture.history.clone();
    wrong[0].0 = wrong[0].0.plus(1);
    assert!(PolicyInput::new(
        &fixture.exercise.frame,
        &wrong,
        fixture.exercise.position.banked,
        30,
        Team::T0
    )
    .is_err());

    let input = PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        fixture.exercise.position.banked,
        30,
        Team::T0,
    )
    .unwrap();
    let mut source = program(PolicyKey::from_input(&input));
    source.exact_rules.clear();
    source.bindings.push(RigidBinding {
        name: "missing".into(),
        query: "(fix (roles (domino d)) (out d))".parse().unwrap(),
    });
    let compiled = source.compile(&Registry::standard()).unwrap();
    assert!(compiled
        .initialize(&input, &mut Budget::new(10_000))
        .is_err());
}
