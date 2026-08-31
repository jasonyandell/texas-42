//! Gates for the counted-belief Slice F [L2 thread]: the §49 consequence
//! CEGAR — hand classes instrumented at the field-classification
//! bottleneck, refined by witness pairs to the action-exact endpoint.
//! Gate 1 is Theorem 30.1's shape: residual class mass falls
//! monotonically, exact mass rises, the per-branch intervals `[L_t, U_t]`
//! nest as the critical set grows, and the endpoint is action-exact with
//! point intervals. Gate 2 is endpoint parity: the fully refined
//! abstraction reproduces the exact contraction's branch masses tile for
//! tile. Gate 3 is §49's witness requirement: every refinement carries a
//! valid pair — same class under the pre-stage critical set, provably
//! different field actions (re-derived through the field itself on a
//! hand-built record), a fresh discriminator held by exactly one of the
//! two hands, and a real split under the post-stage critical set. Gate 4
//! is non-vacuity: the base §49 vocabulary already concentrates positive
//! mass in action-exact classes, classes genuinely aggregate hands, and
//! the refinement loop actually fires somewhere in the gated corpus.
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! §27–31 (hand abstractions, the class verifier, counterexample-guided
//! refinement, the critical-tile interpretation) and §49 (Slice F),
//! adopted by rulings CBS-A6 and CBS-A9 (`walt/CENSUS-RULINGS.md`);
//! design register `walt/FACTOR-BELIEF.md`.
//!
//! DECLARED TEST EPOCH: deterministic fields only — the trivial
//! `FixedPreference` field and the σ0 Level0 { n0 = 2 } modeled mind
//! (the Slice B/C/D/E declared fields). Frozen `verify_player` receipt
//! roots: the six enumerable roots of the prior slices (hands 4/5/10/12
//! at trick 6, hands 3/8 at trick 5) plus the trick-4 roots of hands
//! 3/4/8/12, where the acting supports are large enough for the base
//! vocabulary to leave residual and the CEGAR loop to fire.

mod common;

use common::receipt;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::Domino;
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{
    class_signature, refine_to_action_exact, CegarOutcome, ExactCoverOracle, FactorBelief,
    SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};

/// The gated roots: the six enumerable frozen receipt roots plus the
/// trick-4 roots, where acting supports are large enough to leave the
/// base vocabulary residual mass.
const GATED_ROOTS: [(usize, usize); 10] = [
    (12, 6),
    (10, 6),
    (5, 6),
    (4, 6),
    (8, 5),
    (3, 5),
    (3, 4),
    (4, 4),
    (8, 4),
    (12, 4),
];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// The lowest-index legal focal tile at the root — the fixed opening
/// play behind every gated contraction.
fn lowest_focal(root: &CanonicalRoot, position: &RootPosition) -> Domino {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    legal.iter().next().expect("a legal focal tile")
}

/// The σ0 field of the declared epoch, fresh per call.
fn level0_field() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

/// One gated contraction: the uniform root belief after the fixed focal
/// play, and its refinement record under `field`.
fn refined_at(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    field: &dyn SlicePolicy,
) -> (FactorBelief, CegarOutcome) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, field).focal_play(focal);
    let outcome = refine_to_action_exact(&SupportOracle, &belief, field);
    (belief, outcome)
}

/// Gate 1 — Theorem 30.1's monotone narrowing on every gated root under
/// both declared fields: residual mass nonincreasing, exact mass
/// nondecreasing, classes nondecreasing, the critical set growing by
/// exactly the witnessed discriminator per stage, the branch tile list
/// constant with nested intervals, and an action-exact endpoint with
/// point intervals.
#[test]
fn refinement_narrows_monotonically_to_the_exact_endpoint() {
    let r = receipt();
    for (hand_id, trick_no) in GATED_ROOTS {
        for field in [
            &FixedPreference::lowest_first("field:lowest-first") as &dyn SlicePolicy,
            &level0_field() as &dyn SlicePolicy,
        ] {
            let (belief, outcome) = refined_at(&r, hand_id, trick_no, field);
            let z = SupportOracle.mass(&belief);
            assert!(!outcome.stages.is_empty(), "at least the base stage");
            assert!(
                outcome.stages.len() <= 29,
                "at most 28 refinements over the 28-tile alphabet"
            );
            assert_eq!(
                outcome.witnesses.len() + 1,
                outcome.stages.len(),
                "one witness per refinement (§49)"
            );
            assert!(
                outcome.stages[0].critical.is_empty(),
                "stage 0 is the bare §49 vocabulary"
            );
            let tiles: Vec<Domino> = outcome.stages[0]
                .branch_intervals
                .iter()
                .map(|(t, _, _)| *t)
                .collect();
            for (i, stage) in outcome.stages.iter().enumerate() {
                assert_eq!(
                    stage
                        .exact_mass
                        .checked_add(stage.residual_mass)
                        .expect("an exact mass fits u128"),
                    z,
                    "exact and residual mass partition Z at every stage"
                );
                assert_eq!(
                    stage
                        .branch_intervals
                        .iter()
                        .map(|(t, _, _)| *t)
                        .collect::<Vec<_>>(),
                    tiles,
                    "the branch tile list is the observed action set, constant \
                     across stages"
                );
                let lower_sum: u128 = stage.branch_intervals.iter().map(|(_, l, _)| *l).sum();
                assert_eq!(
                    lower_sum, stage.exact_mass,
                    "the exact lower masses partition the exact mass"
                );
                for (_, lower, upper) in &stage.branch_intervals {
                    assert!(lower <= upper, "an interval is ordered");
                    assert!(
                        upper - lower <= stage.residual_mass,
                        "interval width is bounded by the residual"
                    );
                }
                if i > 0 {
                    let prev = &outcome.stages[i - 1];
                    assert!(
                        stage.exact_mass >= prev.exact_mass,
                        "exact mass is nondecreasing (Theorem 30.1)"
                    );
                    assert!(
                        stage.residual_mass <= prev.residual_mass,
                        "residual mass is nonincreasing (Theorem 30.1)"
                    );
                    assert!(
                        stage.classes >= prev.classes,
                        "a finer critical set never merges classes"
                    );
                    let witness = &outcome.witnesses[i - 1];
                    let mut grown = prev.critical;
                    assert!(
                        grown.insert(witness.discriminator),
                        "the discriminator is fresh"
                    );
                    assert_eq!(
                        stage.critical, grown,
                        "the critical set grows by exactly the witnessed tile"
                    );
                    for (k, (_, lower, upper)) in stage.branch_intervals.iter().enumerate() {
                        let (_, prev_lower, prev_upper) = prev.branch_intervals[k];
                        assert!(*lower >= prev_lower, "lower bounds rise (nesting)");
                        assert!(*upper <= prev_upper, "upper bounds fall (nesting)");
                    }
                }
            }
            let last = outcome.stages.last().expect("a final stage");
            assert_eq!(last.residual_mass, 0, "the endpoint is action-exact");
            assert_eq!(
                last.exact_classes, last.classes,
                "every endpoint class is exact"
            );
            for (_, lower, upper) in &last.branch_intervals {
                assert_eq!(lower, upper, "endpoint intervals are points");
            }
        }
    }
}

/// Gate 2 — endpoint parity with the exact contraction: the fully
/// refined abstraction's per-tile masses equal `branch_masses` exactly,
/// on every gated root under both declared fields.
#[test]
fn the_refined_endpoint_reproduces_the_exact_contraction() {
    let r = receipt();
    for (hand_id, trick_no) in GATED_ROOTS {
        for field in [
            &FixedPreference::lowest_first("field:lowest-first") as &dyn SlicePolicy,
            &level0_field() as &dyn SlicePolicy,
        ] {
            let (belief, outcome) = refined_at(&r, hand_id, trick_no, field);
            let exact = SupportOracle.branch_masses(&belief, field);
            assert_eq!(
                outcome.branch_masses, exact,
                "h{hand_id}-t{trick_no}: the refined endpoint IS the exact \
                 contraction"
            );
        }
    }
}

/// Gate 3 — §49's witness requirement, re-derived independently: for
/// every refinement, the two hands share the pre-stage class signature
/// (recomputed here through the public [`class_signature`]), the field
/// itself (re-consulted on a hand-built record) chooses the two recorded
/// DIFFERENT actions, the discriminator is held by exactly one hand and
/// is fresh, and the post-stage signatures genuinely split.
#[test]
fn every_refinement_carries_a_valid_witness_pair() {
    let r = receipt();
    let mut checked: u64 = 0;
    for (hand_id, trick_no) in GATED_ROOTS {
        for field in [
            &FixedPreference::lowest_first("field:lowest-first") as &dyn SlicePolicy,
            &level0_field() as &dyn SlicePolicy,
        ] {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let focal = lowest_focal(&root, &position);
            let belief = FactorBelief::uniform_root(&root, &position, field).focal_play(focal);
            let outcome = refine_to_action_exact(&SupportOracle, &belief, field);
            let decl = position.decl;
            let trick_plays = vec![focal];
            let history = vec![focal];
            let record = PublicRecord {
                leader: position.leader,
                trick_plays: &trick_plays,
                banked: position.banked,
                root: &position,
                history: &history,
            };
            for (i, w) in outcome.witnesses.iter().enumerate() {
                checked += 1;
                let pre = outcome.stages[i].critical;
                let post = outcome.stages[i + 1].critical;
                let left = class_signature(decl, w.left_hand, &trick_plays, pre);
                let right = class_signature(decl, w.right_hand, &trick_plays, pre);
                assert_eq!(left, w.signature, "the left hand is in the split class");
                assert_eq!(right, w.signature, "the right hand is in the split class");
                assert_ne!(
                    w.left_action, w.right_action,
                    "a witness pair disagrees on the field action"
                );
                for (hand, action) in [(w.left_hand, w.left_action), (w.right_hand, w.right_action)]
                {
                    let led = Some(decl.led_context(focal));
                    let legal = legal_plays(decl, hand, led);
                    assert_eq!(
                        field.choose(decl, hand, legal, &record),
                        action,
                        "the field itself reproduces the witnessed action"
                    );
                }
                assert_ne!(
                    w.left_hand.contains(w.discriminator),
                    w.right_hand.contains(w.discriminator),
                    "the discriminator separates the pair"
                );
                assert!(!pre.contains(w.discriminator), "the discriminator is fresh");
                assert!(post.contains(w.discriminator), "the discriminator entered");
                assert_ne!(
                    class_signature(decl, w.left_hand, &trick_plays, post),
                    class_signature(decl, w.right_hand, &trick_plays, post),
                    "the refinement genuinely splits the pair"
                );
            }
        }
    }
    assert!(
        checked > 0,
        "the gated corpus exercises the refinement loop"
    );
}

/// Gate 4 — non-vacuity of the base vocabulary and of aggregation: on
/// every gated σ0 contraction classes never exceed hands; somewhere the
/// bare vocabulary already resolves positive mass exactly; somewhere the
/// endpoint has strictly fewer classes than hands (classes aggregate);
/// and somewhere the loop needed at least one refinement.
#[test]
fn the_base_vocabulary_carries_mass_and_classes_aggregate() {
    let r = receipt();
    let mut base_exact_somewhere = false;
    let mut aggregates_somewhere = false;
    let mut refines_somewhere = false;
    for (hand_id, trick_no) in GATED_ROOTS {
        let field = level0_field();
        let (_, outcome) = refined_at(&r, hand_id, trick_no, &field);
        for stage in &outcome.stages {
            assert!(
                stage.classes <= outcome.hands,
                "a partition never exceeds its ground set"
            );
        }
        if outcome.stages[0].exact_mass > 0 {
            base_exact_somewhere = true;
        }
        let last = outcome.stages.last().expect("a final stage");
        if last.classes < outcome.hands {
            aggregates_somewhere = true;
        }
        if !outcome.witnesses.is_empty() {
            refines_somewhere = true;
        }
    }
    assert!(
        base_exact_somewhere,
        "the bare §49 vocabulary resolves positive mass somewhere"
    );
    assert!(
        aggregates_somewhere,
        "action-exact classes aggregate more than one hand somewhere"
    );
    assert!(
        refines_somewhere,
        "the witnessed refinement loop fires somewhere in the corpus"
    );
}
