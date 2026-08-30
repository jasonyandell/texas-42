//! Gates for the counted-belief Slice B [L2 thread]: the two-policy
//! grammar and the residual split — §11 induced grammars, the §12
//! decomposition `Q_a = max(Q^G_a, Q^dev_a)` as a walked identity,
//! first-deviation witnesses (CBS-A4's lazy cylinders), and the §8
//! residual-upper identity (the sampled residual bound coincides with the
//! full-class bound; coverage is only ever a residual bound).
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! Part III (§9, §11, §12) and §45, adopted by ruling CBS-A4
//! (`walt/CENSUS-RULINGS.md`); intake companion
//! `walt/math/counted_belief_sandwich_v0.1_intake.md`.
//!
//! DECLARED TEST EPOCH: one field σ = Level0 { n0 = 2 } (the same
//! declared field as the Slice A gates); grammar sources drawn from
//! `FixedPreference::lowest_first` / `highest_first`, the
//! `CountPreservation` safety policy, and one pinned level-1 continuation
//! at declared schedule [2, 2]. Frozen `verify_player` receipt roots:
//! hand 12 trick 6 (fiber 6), hand 10 trick 6 (19), hand 5 trick 6 (27),
//! hand 4 trick 6 (90), hand 8 trick 5 (92); the realized gate values are
//! deterministic on these frozen fixtures and streams.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Decl, Domino, DominoSet};
use walt::solver::adaptive::{
    replay_viewer_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::evidence::ScopedDelta;
use walt::solver::exposure::{exact_root_value, sampled_root_optimum};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{
    exact_grammar_split, first_deviation, grammar_census, residual_empirical_max_upper,
    sampled_grammar_split, CountPreservation, GrammarVerdict, PolicyGrammar,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::root_interval::pmake_empirical_max_upper;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// The declared one-field σ of this gate epoch (the Slice A field).
fn field_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// One pinned frozen level-1 continuation at the declared [2, 2]
/// schedule (the Slice A lower-witness shape, here a grammar source).
fn pinned(position: &RootPosition, tile: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
        solver_source: "walt-level1-continuation-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![2, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned: tile },
    })
}

fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

/// Check one split's verdict against its raw counts, and — on a
/// counterexample — the first-deviation witness's internal consistency.
fn check_verdict(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
    split: &walt::solver::grammar::GrammarSplitValue,
) -> bool {
    let free = split.free_count();
    let gram = split.grammar_count();
    let dev = split.deviation_count();
    match split.verdict() {
        GrammarVerdict::RootOffGrammar => {
            assert!(gram.is_none() && dev == Some(free));
            false
        }
        GrammarVerdict::Closes => {
            assert!(gram == Some(free) && dev.is_none_or(|d| d < free));
            false
        }
        GrammarVerdict::Ties => {
            assert!(gram == Some(free) && dev == Some(free));
            false
        }
        GrammarVerdict::Counterexample => {
            assert!(gram.is_some() && gram < dev && dev == Some(free));
            let witness = first_deviation(root, position, split.action, field, grammar)
                .expect("a counterexample carries a first-deviation witness");
            assert!(
                witness.legal.contains(witness.deviation),
                "the deviation is legal at its state"
            );
            assert!(
                !witness.state_grammar.contains(witness.deviation),
                "the deviation is off-grammar at its state"
            );
            assert!(
                witness.state_grammar.is_subset_of(witness.legal),
                "the grammar at the state is drawn from its legal set"
            );
            assert_eq!(witness.depth, witness.history.len());
            true
        }
    }
}

/// Gate 1 — the §12 decomposition against the shipped exact optimizer:
/// on every fixture and every legal root action, the walk's `free` equals
/// `exact_root_value`'s optimum (cross-implementation parity), the
/// Theorem 9.1 identity holds (constructor- and nodewise-asserted on
/// every call), and the verdict matches the counts. The realized sweep
/// finding is itself frozen: under the two-preference grammar these
/// trick-5/6 fixtures produce NO exact counterexample (the grammar ties
/// or closes every in-grammar action), while the SINGLETON lowest-first
/// grammar does produce one — §45's "first off-grammar information
/// states in exact counterexamples" has a nonvacuous subject, and where
/// it lands depends on the grammar, not just the fixture.
#[test]
fn exact_split_matches_the_exact_optimizer_and_realizes_a_counterexample() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let highest = FixedPreference::highest_first("preference:highest-v1");
    let two = PolicyGrammar::new(vec![&lowest, &highest]);
    let single = PolicyGrammar::new(vec![&lowest]);
    let mut two_counterexamples = 0u64;
    let mut single_counterexamples = 0u64;
    for (hand_id, trick_no) in [(12usize, 6usize), (10, 6), (5, 6), (4, 6), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        for action in legal_root_actions(&root, &position).iter() {
            let exact = exact_root_value(&root, &position, action, &field);
            for grammar in [&two, &single] {
                let split = exact_grammar_split(&root, &position, action, &field, grammar);
                assert_eq!(
                    split.free_count(),
                    exact.win_worlds,
                    "the walk's unrestricted optimum is the exact root value"
                );
                let saw = check_verdict(&root, &position, &field, grammar, &split);
                if std::ptr::eq(grammar, &two) {
                    two_counterexamples += u64::from(saw);
                } else {
                    single_counterexamples += u64::from(saw);
                }
            }
        }
    }
    assert_eq!(
        two_counterexamples, 0,
        "frozen sweep finding: the two-preference grammar leaves no exact counterexample here"
    );
    assert!(
        single_counterexamples >= 1,
        "the singleton grammar realizes at least one exact counterexample in the sweep"
    );
}

/// Gate 2 — the singleton grammar is one policy: with `G` induced by
/// `lowest_first` alone, the grammar class is behaviorally that single
/// policy, so `Q^G` equals its §6 replay success count over the fiber.
#[test]
fn singleton_grammar_value_equals_the_frozen_replay_count() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let grammar = PolicyGrammar::new(vec![&lowest]);
    for (hand_id, trick_no) in [(4usize, 6usize), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        // The singleton grammar admits exactly the source's own root
        // action; split at that action.
        let action = {
            let led = position
                .trick_plays
                .first()
                .map(|d| position.decl.led_context(*d));
            let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
            let record = PublicRecord {
                leader: position.leader,
                trick_plays: &position.trick_plays,
                banked: position.banked,
                root: &position,
                history: &[],
            };
            lowest.choose(position.decl, root.kernel().viewer_hand(), legal, &record)
        };
        let split = exact_grammar_split(&root, &position, action, &field, &grammar);
        assert!(split.root_in_grammar);
        let replayed: u64 = root
            .worlds()
            .filter(|world| replay_viewer_success(&position, viewer, world, &lowest, &field))
            .count()
            .try_into()
            .expect("fits");
        assert_eq!(
            split.grammar_count(),
            Some(replayed),
            "a singleton grammar's restricted optimum is its source's replay count"
        );
    }
}

/// Gate 3 — sampled parity with the Slice A empirical optimizer: on a
/// declared stream prefix, the walk's `free` count equals
/// `sampled_root_optimum` at every prefix length, and the on-sample
/// deviation optimum never exceeds it.
#[test]
fn sampled_split_matches_the_sampled_optimizer_by_prefix() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let highest = FixedPreference::highest_first("preference:highest-v1");
    let grammar = PolicyGrammar::new(vec![&lowest, &highest]);
    let (root, position) = root_at(&r, 10, 6);
    let action = legal_root_actions(&root, &position)
        .iter()
        .next()
        .expect("a legal root action");
    for t in 1..=24u64 {
        let split = sampled_grammar_split(&root, &position, action, &field, &grammar, 0, t);
        assert_eq!(
            split.free_count(),
            sampled_root_optimum(&root, &position, action, &field, 0, t),
            "the walk's sampled optimum is the Slice A empirical optimum"
        );
        if let Some(d) = split.deviation_count() {
            assert!(d <= split.free_count());
        }
    }
}

/// Gate 4 — grammar monotonicity in sources (Π^G grows with G): adding a
/// source never lowers the restricted optimum, and it never exceeds the
/// free optimum.
#[test]
fn grammar_value_is_monotone_in_sources() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let highest = FixedPreference::highest_first("preference:highest-v1");
    let safety = CountPreservation::new();
    let (root, position) = root_at(&r, 4, 6);
    let g1 = PolicyGrammar::new(vec![&lowest]);
    let g2 = PolicyGrammar::new(vec![&lowest, &highest]);
    let g3 = PolicyGrammar::new(vec![&lowest, &highest, &safety]);
    // Split at the lowest source's root action — in-grammar for all three.
    let action = {
        let led = position
            .trick_plays
            .first()
            .map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
        let record = PublicRecord {
            leader: position.leader,
            trick_plays: &position.trick_plays,
            banked: position.banked,
            root: &position,
            history: &[],
        };
        lowest.choose(position.decl, root.kernel().viewer_hand(), legal, &record)
    };
    let s1 = exact_grammar_split(&root, &position, action, &field, &g1);
    let s2 = exact_grammar_split(&root, &position, action, &field, &g2);
    let s3 = exact_grammar_split(&root, &position, action, &field, &g3);
    assert_eq!(s1.free_count(), s2.free_count());
    assert_eq!(s2.free_count(), s3.free_count());
    let (q1, q2, q3) = (
        s1.grammar_count().expect("in-grammar"),
        s2.grammar_count().expect("in-grammar"),
        s3.grammar_count().expect("in-grammar"),
    );
    assert!(q1 <= q2 && q2 <= q3, "Q^G is monotone in the source set");
    assert!(q3 <= s3.free_count(), "Q^G never exceeds Q");
    // The census sees the same monotonicity in the action sets, and the
    // grammar never exceeds the legal room.
    let c1 = grammar_census(&root, &position, action, &field, &g1);
    let c3 = grammar_census(&root, &position, action, &field, &g3);
    assert_eq!(c1.focal_states, c3.focal_states);
    assert_eq!(
        c1.grammar_action_total, c1.focal_states,
        "a singleton grammar admits exactly one action per state"
    );
    assert!(c1.grammar_action_total <= c3.grammar_action_total);
    assert!(c3.grammar_action_total <= c3.legal_action_total);
    assert!(c3.saturated_states <= c3.focal_states);
}

/// Gate 5 — an off-grammar root action is all residual: `gram` is
/// absent, `dev = free`, the verdict says so, and the first-deviation
/// witness is the root action itself at depth zero.
#[test]
fn off_grammar_root_action_is_all_residual() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let highest = FixedPreference::highest_first("preference:highest-v1");
    let grammar = PolicyGrammar::new(vec![&lowest, &highest]);
    // Find a fixture action strictly between the two preference choices.
    let mut exercised = false;
    for (hand_id, trick_no) in [(4usize, 6usize), (8, 5), (5, 6)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let led = position
            .trick_plays
            .first()
            .map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
        let record = PublicRecord {
            leader: position.leader,
            trick_plays: &position.trick_plays,
            banked: position.banked,
            root: &position,
            history: &[],
        };
        let at_root = grammar.actions(position.decl, root.kernel().viewer_hand(), legal, &record);
        let Some(action) = legal.iter().find(|d| !at_root.contains(*d)) else {
            continue;
        };
        exercised = true;
        let split = exact_grammar_split(&root, &position, action, &field, &grammar);
        assert!(!split.root_in_grammar);
        assert_eq!(split.verdict(), GrammarVerdict::RootOffGrammar);
        assert_eq!(split.grammar_count(), None);
        assert_eq!(split.deviation_count(), Some(split.free_count()));
        let witness = first_deviation(&root, &position, action, &field, &grammar)
            .expect("an off-grammar root action is its own witness");
        assert_eq!(witness.depth, 0);
        assert!(witness.history.is_empty());
        assert_eq!(witness.deviation, action);
        assert_eq!(witness.state_grammar, at_root);
    }
    assert!(
        exercised,
        "some fixture leaves a legal root action outside the two-preference grammar"
    );
}

/// Gate 6 — the §8 identity: the residual empirical-max upper coincides,
/// count for count and bound for bound, with the Slice A full-class
/// upper. Partitioning by itself tightens nothing; the sampled route can
/// bound the residual only at the full class's value.
#[test]
fn residual_upper_is_the_full_class_upper() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("preference:lowest-v1");
    let highest = FixedPreference::highest_first("preference:highest-v1");
    let grammar = PolicyGrammar::new(vec![&lowest, &highest]);
    let (root, position) = root_at(&r, 10, 6);
    let action = legal_root_actions(&root, &position)
        .iter()
        .next()
        .expect("a legal root action");
    let residual = residual_empirical_max_upper(
        &root,
        &position,
        action,
        &field,
        &grammar,
        0,
        16,
        ScopedDelta::new("gate6:residual-upper", q(1, 100)),
    );
    let full = pmake_empirical_max_upper(
        &root,
        &position,
        action,
        &field,
        0,
        16,
        ScopedDelta::new("gate6:full-upper", q(1, 100)),
    );
    assert_eq!(residual.counts(), full.counts());
    assert_eq!(residual.upper(), full.upper());
    assert_eq!(residual.policy_class, full.policy_class);
}

/// Gate 7 — the level-1 continuation as a grammar source: the frozen
/// policy is a member of Π^G, so the restricted optimum dominates its
/// replay count; and the walk agrees with the exact optimizer under this
/// costlier source too.
#[test]
fn level1_continuation_source_is_dominated_by_its_grammar() {
    let r = receipt();
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&r, 10, 6);
    let viewer = root.kernel().viewer();
    let action = legal_root_actions(&root, &position)
        .iter()
        .next()
        .expect("a legal root action");
    let level1 = pinned(&position, action);
    let safety = CountPreservation::new();
    let grammar = PolicyGrammar::new(vec![&level1, &safety]);
    let split = exact_grammar_split(&root, &position, action, &field, &grammar);
    assert!(
        split.root_in_grammar,
        "the pinned continuation plays its pinned tile at the root"
    );
    let replayed: u64 = root
        .worlds()
        .filter(|world| replay_viewer_success(&position, viewer, world, &level1, &field))
        .count()
        .try_into()
        .expect("fits");
    assert!(
        split.grammar_count() >= Some(replayed),
        "a source policy's value never exceeds its grammar's restricted optimum"
    );
    let exact = exact_root_value(&root, &position, action, &field);
    assert_eq!(split.free_count(), exact.win_worlds);
}

/// Gate 8 — refusals: a grammar with no sources is refused; a source
/// choosing an illegal action is refused at the query.
#[test]
fn grammar_refusals() {
    let r = receipt();
    let (root, position) = root_at(&r, 12, 6);
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            PolicyGrammar::new(vec![]);
        }))
        .is_err(),
        "an empty source list can never keep G(I) nonempty"
    );
    /// A hostile source: claims the first tile of the full set, legal or
    /// not — the grammar query must refuse it.
    struct Hostile;
    impl SlicePolicy for Hostile {
        fn id(&self) -> &str {
            "hostile:first-tile-v1"
        }
        fn choose(
            &self,
            _decl: Decl,
            _hand: DominoSet,
            _legal: DominoSet,
            _record: &PublicRecord<'_>,
        ) -> Domino {
            Domino::from_index(0).expect("tile 0")
        }
    }
    let hostile = Hostile;
    let grammar = PolicyGrammar::new(vec![&hostile]);
    let hand = root.kernel().viewer_hand();
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, hand, led);
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &[],
    };
    if legal.contains(Domino::from_index(0).expect("tile 0")) {
        // The hostile choice happens to be legal here; nothing to refuse.
        return;
    }
    assert!(
        catch_unwind(AssertUnwindSafe(|| {
            grammar.actions(position.decl, hand, legal, &record);
        }))
        .is_err(),
        "a source choosing an illegal action is refused"
    );
}
