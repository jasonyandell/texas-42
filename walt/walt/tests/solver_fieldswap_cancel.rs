//! Gates for the third field-swap vertical slice [L2 thread]: the
//! cancellation ladder, pairwise benefit/hazard masses, the six-label
//! cancellation vocabulary with the dominance type-lock, the directional
//! rungs R⁺/R⁻ with the sandwich and winner-stability theorems, the
//! extended rung ladder, the sampled E3 type distinction, Stage-4
//! survivor-only field-1 optimization, the ExactRoot baseline tier, the Λ
//! evidence processes, and first-split trace aggregation.
//!
//! Mathematical source: Part VI of the x:019–023 panel response
//! (`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
//! §§31–42), adopted for slice 3 by rulings PANEL-A7/A8
//! (`walt/CENSUS-RULINGS.md`, "The panel-response adjudication
//! (2026-08-24)").
//!
//! DECLARED TEST EPOCH PAIR (unchanged from slice 2): σ0 = Level0
//! { n0 = 2 }, σ1 = Level1 { n_outer = 2, n0 = 2 }; frozen focal
//! candidates at declared schedule [2, 2]. Exact parity roots from the
//! frozen `verify_player` receipt: hand 4 trick 6 (fiber 90), hand 8
//! trick 5 (fiber 92), hand 10 trick 6 (fiber 19). At each root the
//! viewer leads, so every held tile is a legal root action.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::evidence::{affine_factor, MeanNull};
use walt::solver::exposure::{
    directional_reach, exact_root_value, exact_split_reach, frozen_policy_exposure,
    sampled_split_reach, ExposureRung, FirstSplit, FrozenPolicyExposure,
    RootActionDirectionalUpper, RootActionExposureUpper, WorldDomain, WorldRow,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    correction_pivotal_evidence, exact_frozen_action_values, exact_pairwise_masses,
    exact_root_bounds, field_split_traces, fixed_policy_cancellation_kind, pair_lift,
    sampled_pairwise_masses, ActionBound, ActionDirectionalUpper, ActionExposureUpper,
    AdmissibleScreen, BaselineTier, CancellationKind, CancellationLadder, DirectionalScreen,
    ExactPairwiseMasses, FieldSwapKind, PairLiftProcess, SplitAggregate,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The declared exact parity roots: (hand, trick, fiber).
const PARITY_ROOTS: [(usize, usize, u128); 3] = [(4, 6, 90), (8, 5, 92), (10, 6, 19)];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// σ0 of the declared test epoch pair.
fn field0_spec() -> FieldSpec {
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

/// σ1 of the declared test epoch pair.
fn field1_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// One pinned frozen focal candidate for a legal root action, at the
/// declared [2, 2] schedule.
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

fn abs(x: BigRational) -> BigRational {
    if x < BigRational::zero() {
        -x
    } else {
        x
    }
}

// ---------------------------------------------------------------------------
// §31 / PANEL-A7 — the cancellation ladder holds exactly and the three
// zeros nest without collapsing.
// ---------------------------------------------------------------------------

#[test]
fn cancellation_ladder_holds_exactly_and_retains_all_five_components() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber, "the declared parity fiber");
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        for action in legal_root_actions(&root, &position).iter() {
            let rho = pinned(&position, action);
            let exposure = frozen_policy_exposure(
                &root,
                &position,
                &rho,
                &field0,
                &field1,
                WorldDomain::ExactFiber,
            );
            let ladder = CancellationLadder::from_exposure(&exposure);
            // The ladder's masses re-derive the exposure's own accessors.
            assert_eq!(ladder.d(), exposure.d_hat());
            assert_eq!(ladder.r(), exposure.c_abs_hat());
            assert_eq!(ladder.c(), exposure.c_hat());
            // §31 restated externally: |c| ≤ r ≤ d.
            assert!(abs(ladder.c()) <= ladder.r() && ladder.r() <= ladder.d());
            // The serialization retains ALL of (d, r, c⁺, c⁻, c) — never a
            // collapsed net (PANEL-A7).
            let s = ladder.to_string();
            for needle in ["d=", "r=", "c_plus=", "c_minus=", "c="] {
                assert!(s.contains(needle), "the ladder display retains {needle}");
            }
        }
    }
}

/// A synthetic exposure over hand-built rows: the classification logic is
/// gated on every branch, with the split shape borrowed from a real run
/// (the aggregates are recomputed from the rows by construction).
fn synthetic_exposure(
    template: &FrozenPolicyExposure,
    split: &FirstSplit,
    domain: WorldDomain,
    spec: &[(bool, bool, bool)],
) -> FrozenPolicyExposure {
    let rows: Vec<WorldRow> = spec
        .iter()
        .enumerate()
        .map(|(i, (exposed, u0, u1))| {
            assert!(*exposed || u0 == u1, "L2-T1 shapes the synthetic rows");
            WorldRow {
                index: u64::try_from(i).expect("fits"),
                world: [0, 1, 2, u32::try_from(i).expect("fits")],
                u0: *u0,
                u1: *u1,
                split: exposed.then(|| split.clone()),
            }
        })
        .collect();
    FrozenPolicyExposure {
        policy: template.policy,
        field0: template.field0,
        field1: template.field1,
        root_id: template.root_id,
        domain,
        worlds: u64::try_from(rows.len()).expect("fits"),
        exposed: u64::try_from(rows.iter().filter(|r| r.split.is_some()).count()).expect("fits"),
        corrections_plus: u64::try_from(rows.iter().filter(|r| !r.u0 && r.u1).count())
            .expect("fits"),
        corrections_minus: u64::try_from(rows.iter().filter(|r| r.u0 && !r.u1).count())
            .expect("fits"),
        rows,
    }
}

#[test]
fn cancellation_kinds_classify_each_zero_distinctly_and_sampled_domains_resolve_nothing() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    // A real exposed specimen supplies the template row shape.
    let action = legal_root_actions(&root, &position)
        .iter()
        .next()
        .expect("a legal action");
    let rho = pinned(&position, action);
    let template = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    let split = template
        .rows
        .iter()
        .find_map(|row| row.split.clone())
        .expect("the split-heavy parity root exposes some world");
    let classify = |spec: &[(bool, bool, bool)], domain: WorldDomain, eps: Option<&BigRational>| {
        let exposure = synthetic_exposure(&template, &split, domain, spec);
        fixed_policy_cancellation_kind(&CancellationLadder::from_exposure(&exposure), eps)
    };
    let exact = WorldDomain::ExactFiber;
    // d = 0: behavioral irrelevance.
    let quiet = [(false, true, true), (false, false, false)];
    assert_eq!(
        classify(&quiet, exact.clone(), None),
        CancellationKind::NoFieldExposure
    );
    // d > 0, r = 0: outcome irrelevance is NOT behavioral irrelevance.
    let stable = [(true, true, true), (false, false, false)];
    assert_eq!(
        classify(&stable, exact.clone(), None),
        CancellationKind::OutcomeStable
    );
    // r > 0, c = 0: value neutrality is NOT outcome irrelevance.
    let neutral = [(true, false, true), (true, true, false)];
    assert_eq!(
        classify(&neutral, exact.clone(), None),
        CancellationKind::ValueNeutral
    );
    // |c| = 1/4 < ε = 1/2: ε-equivalence needs a DECLARED ε...
    let lopsided = [
        (true, false, true),
        (false, true, true),
        (false, true, true),
        (false, false, false),
    ];
    let eps = q(1, 2);
    assert_eq!(
        classify(&lopsided, exact.clone(), Some(&eps)),
        CancellationKind::EpsilonEquivalent
    );
    // ...an undeclared or too-small ε resolves nothing.
    assert_eq!(
        classify(&lopsided, exact.clone(), None),
        CancellationKind::Unresolved
    );
    let tight = q(1, 8);
    assert_eq!(
        classify(&lopsided, exact, Some(&tight)),
        CancellationKind::Unresolved
    );
    // PANEL-A7: a sampled domain establishes NO zero — the very same
    // counts that read NoFieldExposure on the exact fiber are Unresolved
    // on a stream prefix.
    let sampled = WorldDomain::StreamPrefix {
        epoch: 0,
        worlds: 2,
    };
    assert_eq!(
        classify(&quiet, sampled, None),
        CancellationKind::Unresolved
    );
    // A REAL behaviorally-equal pair (distinct FieldIds, same behavior)
    // classifies NoFieldExposure over the exact fiber.
    let relabeled = FieldModel::new(FieldSpec {
        construction: "level0-modeled-mind-v1-relabeled".to_string(),
        ..field0_spec()
    });
    let exposure = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &relabeled,
        WorldDomain::ExactFiber,
    );
    let ladder = CancellationLadder::from_exposure(&exposure);
    assert!(ladder.behavioral_irrelevance());
    assert_eq!(
        fixed_policy_cancellation_kind(&ladder, None),
        CancellationKind::NoFieldExposure
    );
}

// ---------------------------------------------------------------------------
// §33–§34 / PANEL-A7 — pairwise masses are a census, (B, H, q, g) are all
// retained, g cross-checks the exact frozen baselines, and Dominated is
// reachable only through the exact type.
// ---------------------------------------------------------------------------

#[test]
fn pairwise_masses_census_g_identity_and_the_dominance_lock() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
        let a = actions[0];
        let b = actions[1];
        let rho_a = pinned(&position, a);
        let rho_b = pinned(&position, b);
        let candidates: Vec<(Domino, &FrozenPolicy)> = vec![(a, &rho_a), (b, &rho_b)];
        for spec in [field0_spec(), field1_spec()] {
            let field = FieldModel::new(spec);
            let masses = exact_pairwise_masses(&root, &position, &rho_a, &rho_b, &field);
            // The census: B + H + both-make + both-fail = |Φ|.
            assert_eq!(
                u128::from(masses.benefit_worlds())
                    + u128::from(masses.hazard_worlds())
                    + u128::from(masses.both_make_worlds())
                    + u128::from(masses.both_fail_worlds()),
                fiber
            );
            assert_eq!(masses.fiber(), fiber);
            // g = B − H and q = B + H, exactly.
            assert_eq!(masses.g(), masses.b() - masses.h());
            assert_eq!(masses.q(), masses.b() + masses.h());
            assert!(abs(masses.g()) <= masses.q(), "|g| ≤ q pointwise");
            // The gap identity against an independent producer: g(a,b) =
            // V(ρ_a) − V(ρ_b) from the controller's cold exact endpoint.
            let baseline =
                exact_frozen_action_values(&root, &position, &candidates, &field, "gate-pairwise");
            assert_eq!(
                masses.g(),
                baseline.value(a) - baseline.value(b),
                "the pairwise gap equals the exact frozen-value difference"
            );
            // §34: Dominated exactly when H = 0 ∧ B > 0 — never otherwise.
            let kind = masses.dominance_kind();
            if masses.hazard_worlds() == 0 && masses.benefit_worlds() > 0 {
                assert_eq!(kind, CancellationKind::Dominated);
            } else {
                assert_eq!(kind, CancellationKind::Unresolved);
            }
            // A policy against itself: B = H = 0 — an exact tie is NOT
            // dominance (§34's B > 0 requirement).
            let tie = exact_pairwise_masses(&root, &position, &rho_a, &rho_a, &field);
            assert_eq!(tie.benefit_worlds(), 0);
            assert_eq!(tie.hazard_worlds(), 0);
            assert_eq!(tie.dominance_kind(), CancellationKind::Unresolved);
            // The sampled sibling: exact counts over the prefix, marked as
            // an estimate, and — the PANEL-A7 type lock — it has NO
            // dominance route: `dominance_kind` exists only on
            // `ExactPairwiseMasses` (the signature below is the
            // compile-time demonstration).
            let sampled = sampled_pairwise_masses(&root, &position, &rho_a, &rho_b, &field, 0, 8);
            assert_eq!(
                sampled.benefit_worlds()
                    + sampled.hazard_worlds()
                    + sampled.both_make_worlds()
                    + sampled.both_fail_worlds(),
                8
            );
            let shown = sampled.to_string();
            assert!(shown.starts_with("SampledPairwiseMasses{estimate;"));
            assert!(!shown.contains("Dominated"));
        }
    }
    // Only the exact type can even be ASKED about dominance.
    fn only_exact_masses_reach_dominance(masses: &ExactPairwiseMasses) -> CancellationKind {
        masses.dominance_kind()
    }
    let _ = only_exact_masses_reach_dominance;
}

#[test]
fn the_six_cancellation_labels_are_mechanically_distinct() {
    for (i, a) in CancellationKind::ALL.iter().enumerate() {
        for b in &CancellationKind::ALL[i + 1..] {
            assert_ne!(a.tag(), b.tag());
            assert_ne!(a.to_string(), b.to_string());
        }
        assert_eq!(a.to_string(), a.tag());
    }
}

// ---------------------------------------------------------------------------
// §35 / §38 / PANEL-A8 — the directional rungs: extended ladder, agreement
// with E4 on the shared walk, per-policy directional masses bounded, and
// the frozen-tier sandwich.
// ---------------------------------------------------------------------------

#[test]
fn directional_rungs_obey_the_extended_ladder_and_the_sandwich() {
    let r = receipt();
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
        let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        let baseline0 =
            exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-dir-s0");
        let baseline1 =
            exact_frozen_action_values(&root, &position, &candidates, &field1, "gate-dir-s1");
        for (action, rho) in &candidates {
            let dir = directional_reach(&root, &position, *action, &field0, &field1);
            let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
            // The shared walk agrees with the independent E4 producer
            // exactly: R^exposure IS rung E4's count.
            assert_eq!(dir.exposure_worlds, solve.frontier_worlds);
            assert_eq!(dir.exposure_mass(), solve.r());
            // The extended rung ladder, restated externally (PANEL-A8):
            // R± ≤ R^outcome ≤ R^exposure.
            assert!(dir.plus_upper() <= dir.outcome_upper());
            assert!(dir.minus_upper() <= dir.outcome_upper());
            assert!(dir.outcome_upper() <= dir.exposure_mass());
            // A frozen ρ_a ∈ Π_a: its exact directional masses are LOWER
            // witnesses to the directional suprema.
            let exposure = frozen_policy_exposure(
                &root,
                &position,
                rho,
                &field0,
                &field1,
                WorldDomain::ExactFiber,
            );
            let ladder = CancellationLadder::from_exposure(&exposure);
            assert!(ladder.c_plus_mass() <= dir.plus_upper());
            assert!(ladder.c_minus_mass() <= dir.minus_upper());
            assert!(ladder.r() <= dir.outcome_upper());
            // The §35 sandwich at the frozen tier, with exact numbers:
            // V0 − (R⁻)^U ≤ V1 ≤ V0 + (R⁺)^U.
            let v0 = baseline0.value(*action);
            let v1 = baseline1.value(*action);
            assert!(
                v0 - dir.minus_upper() <= *v1 && *v1 <= v0 + dir.plus_upper(),
                "the directional sandwich holds exactly at {action}"
            );
            // The directional pair can be dramatically tighter than the
            // symmetric bound and is never looser (each side ≤ R_a).
            assert!(dir.plus_upper() <= solve.r());
            assert!(dir.minus_upper() <= solve.r());
        }
    }
}

// ---------------------------------------------------------------------------
// §36–§37 / PANEL-A8 — the directional screen: O38 audit inherited, bar
// arithmetic, subset of the symmetric screen, exclusion soundness, and
// directional winner stability gated on exact σ1 values.
// ---------------------------------------------------------------------------

#[test]
fn the_directional_screen_is_sound_and_admits_a_subset_of_the_symmetric_screen() {
    let r = receipt();
    let mut some_positive_slack = false;
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let legal = legal_root_actions(&root, &position);
        let actions: Vec<Domino> = legal.iter().collect();
        let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        let baseline0 =
            exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-ds-s0");
        let baseline1 =
            exact_frozen_action_values(&root, &position, &candidates, &field1, "gate-ds-s1");
        let mut e4 = Vec::new();
        let mut directionals = Vec::new();
        for action in &actions {
            let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
            e4.push(ActionExposureUpper {
                action: *action,
                bound: solve.e4_upper(),
            });
            let dir = directional_reach(&root, &position, *action, &field0, &field1);
            directionals.push(ActionDirectionalUpper {
                action: *action,
                bound: dir.directional_upper(),
            });
        }
        let root_id = walt::solver::adaptive::root_identity(&root, &position);
        let symmetric = AdmissibleScreen::compute(
            legal,
            BaselineTier::ExactFrozenSet,
            &baseline0.point_bounds(),
            &e4,
            field0.field_id(),
            field1.field_id(),
            root_id,
        );
        let directional = DirectionalScreen::compute(
            legal,
            BaselineTier::ExactFrozenSet,
            &baseline0.point_bounds(),
            &directionals,
            field0.field_id(),
            field1.field_id(),
            root_id,
        );
        // The bar recomputed independently: B̄ = max_a (v0[a] − (R⁻_a)^U).
        let bar = actions
            .iter()
            .map(|a| {
                baseline0.value(*a)
                    - directionals
                        .iter()
                        .find(|d| d.action == *a)
                        .expect("bound")
                        .bound
                        .screenable_minus()
            })
            .max()
            .expect("actions");
        assert_eq!(directional.bar(), bar, "directional bar arithmetic");
        // Directional admissible ⊆ symmetric admissible: (R±)^U ≤ R^U per
        // action, so directional intervals are nested in symmetric ones.
        let admitted_dir = directional.admissible();
        let admitted_sym = symmetric.admissible();
        for action in &admitted_dir {
            assert!(
                admitted_sym.contains(action),
                "the directional screen only ever prunes MORE"
            );
        }
        // Exclusion soundness against exact σ1 values: every directionally
        // excluded action is strictly σ1-nonoptimal in the frozen set.
        let best1 = actions
            .iter()
            .map(|a| baseline1.value(*a).clone())
            .max()
            .expect("actions");
        for action in &actions {
            if !admitted_dir.contains(action) {
                assert!(
                    *baseline1.value(*action) < best1,
                    "a directionally excluded action is strictly σ1-nonoptimal"
                );
            }
        }
        // §36 — directional winner stability, gated with exact numbers:
        // positive directional slack S⃗(a,b) implies V1(a) > V1(b), and the
        // rival cannot be admitted.
        let table = directional.slack_table();
        assert_eq!(table.len(), actions.len() * (actions.len() - 1));
        for entry in &table {
            let recomputed = baseline0.value(entry.a)
                - baseline0.value(entry.b)
                - directional.row(entry.a).directional.screenable_minus()
                - directional.row(entry.b).directional.screenable_plus();
            assert_eq!(entry.slack, recomputed, "directional slack arithmetic");
            assert_eq!(
                entry.slack > BigRational::zero(),
                directional.winner_stable_over(entry.a, entry.b)
            );
            if entry.slack > BigRational::zero() {
                some_positive_slack = true;
                assert!(
                    baseline1.value(entry.a) > baseline1.value(entry.b),
                    "§36: a margin above R⁻_a + R⁺_b survives the swap"
                );
                assert!(!directional.admitted(entry.b));
            }
        }
        // The kind vocabulary and tier discipline are the symmetric
        // screen's, verbatim.
        let kind = directional.kind();
        if admitted_dir.len() == 1 {
            assert_eq!(kind, FieldSwapKind::FieldStableExactFrozenSet);
        } else {
            assert_eq!(kind, FieldSwapKind::FieldSensitive);
        }
        let shown = directional.to_string();
        assert!(shown.starts_with("DirectionalScreen{kind="));
        assert!(shown.contains("RootActionDirectionalUpper{plus="));
    }
    assert!(
        some_positive_slack,
        "some parity pair holds positive directional slack; if this ever \
         fails the roots or the declared pair must change"
    );
}

#[test]
fn the_directional_screen_inherits_the_o38_admission_audit() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let bound = |a: Domino| ActionBound {
        action: a,
        lower: q(1, 2),
        upper: q(1, 2),
    };
    let directional = |a: Domino| ActionDirectionalUpper {
        action: a,
        bound: RootActionDirectionalUpper::from_bounds(q(1, 8), q(1, 4)),
    };
    let bounds: Vec<ActionBound> = actions.iter().map(|a| bound(*a)).collect();
    let directionals: Vec<ActionDirectionalUpper> =
        actions.iter().map(|a| directional(*a)).collect();
    let (f0, f1) = (
        FieldModel::new(field0_spec()).field_id(),
        FieldModel::new(field1_spec()).field_id(),
    );
    let screen = DirectionalScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        &directionals,
        f0,
        f1,
        0,
    );
    assert_eq!(screen.admissible().len(), actions.len());
    // A missing directional bound is rejected — no action is screened out
    // by omission.
    let missing = &directionals[1..];
    assert!(catch_unwind(AssertUnwindSafe(|| DirectionalScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        missing,
        f0,
        f1,
        0,
    )))
    .is_err());
    // A duplicated directional bound is rejected.
    let mut duplicated = directionals.clone();
    duplicated[1] = directional(actions[0]);
    assert!(catch_unwind(AssertUnwindSafe(|| DirectionalScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        &duplicated,
        f0,
        f1,
        0,
    )))
    .is_err());
    // An unresolved baseline never yields a stability claim here either.
    let unresolved = DirectionalScreen::compute(
        legal,
        BaselineTier::Unresolved,
        &bounds,
        &directionals,
        f0,
        f1,
        0,
    );
    assert_eq!(unresolved.kind(), FieldSwapKind::FieldUnresolved);
}

// ---------------------------------------------------------------------------
// §7.4 / PANEL-A7 — the sampled E3 rung is a mechanically distinct type
// from exact E4 and offers no screening route.
// ---------------------------------------------------------------------------

#[test]
fn sampled_e3_is_typed_distinct_from_exact_e4() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    for action in legal_root_actions(&root, &position).iter() {
        let sampled = sampled_split_reach(&root, &position, action, &field0, &field1, 0, 16);
        assert_eq!(sampled.worlds, 16);
        assert!(sampled.frontier_worlds <= sampled.worlds);
        assert!(
            sampled.estimate() >= BigRational::zero() && sampled.estimate() <= BigRational::one()
        );
        assert_eq!(
            sampled.domain(),
            WorldDomain::StreamPrefix {
                epoch: 0,
                worlds: 16
            }
        );
        // The serialization names the sampled rung and the estimate tier;
        // it can never be confused with the exact solve's.
        let shown = sampled.to_string();
        assert!(shown.starts_with("SplitReachSampled{rung=E3-sampled;estimate="));
        let exact = exact_split_reach(&root, &position, action, &field0, &field1);
        assert!(exact.to_string().starts_with("SplitReachExact"));
        // No sound fiber statement relates the sampled optimum to R_a in
        // either direction — which is exactly why the type has no
        // `screenable_upper` and no conversion to RootActionExposureUpper
        // (the only screen entry, restated here as the compile-time lock).
        fn only_screenable(bound: RootActionExposureUpper) -> ActionExposureUpper {
            ActionExposureUpper {
                action: Domino::from_index(0).expect("tile"),
                bound,
            }
        }
        let _ = only_screenable(RootActionExposureUpper::from_rung(
            ExposureRung::E4,
            q(1, 3),
        ));
    }
}

// ---------------------------------------------------------------------------
// §8 Stage 1 (ExactRoot tier) + §35/§36 with exact optimized values: the
// producer dominates every frozen candidate, obeys L2-T2 against E4, the
// exact-root sandwich, and exact-root winner stability.
// ---------------------------------------------------------------------------

#[test]
fn the_exact_root_tier_produces_sound_baselines_and_the_exact_sandwich() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let legal = legal_root_actions(&root, &position);
        let actions: Vec<Domino> = legal.iter().collect();
        let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        let frozen0 =
            exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-er-s0");
        let frozen1 =
            exact_frozen_action_values(&root, &position, &candidates, &field1, "gate-er-s1");
        let bounds0 = exact_root_bounds(&root, &position, legal, &field0);
        let mut q0 = Vec::new();
        let mut q1 = Vec::new();
        let mut directionals = Vec::new();
        for action in &actions {
            let v0 = exact_root_value(&root, &position, *action, &field0);
            let v1 = exact_root_value(&root, &position, *action, &field1);
            assert_eq!(v0.fiber, fiber);
            // The convenience bounds are the producer's point values.
            let bound = bounds0.iter().find(|b| b.action == *action).expect("bound");
            assert_eq!(bound.lower, v0.value());
            assert_eq!(bound.upper, v0.value());
            // The optimizer dominates every information-consistent frozen
            // candidate under the same field.
            assert!(v0.value() >= *frozen0.value(*action));
            assert!(v1.value() >= *frozen1.value(*action));
            let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
            // L2-T2 with exact optimized values: |Q1 − Q0| ≤ R_a.
            assert!(abs(v1.value() - v0.value()) <= solve.r());
            let dir = directional_reach(&root, &position, *action, &field0, &field1);
            // §35 — the exact-root sandwich: Q0 − (R⁻)^U ≤ Q1 ≤ Q0 + (R⁺)^U.
            assert!(v0.value() - dir.minus_upper() <= v1.value());
            assert!(v1.value() <= v0.value() + dir.plus_upper());
            q0.push(v0.value());
            q1.push(v1.value());
            directionals.push(ActionDirectionalUpper {
                action: *action,
                bound: dir.directional_upper(),
            });
        }
        // §36 with exact optimized values: the winner-stability premise
        // transfers the σ0 order to σ1, wherever it holds.
        for (i, a) in actions.iter().enumerate() {
            for (j, b) in actions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let margin = &q0[i] - &q0[j];
                let threshold = directionals[i].bound.screenable_minus()
                    + directionals[j].bound.screenable_plus();
                if margin > threshold {
                    assert!(
                        q1[i] > q1[j],
                        "§36 at the exact-root tier: {a} stays ahead of {b}"
                    );
                }
            }
        }
        // The ExactRoot-tier directional screen claims stability at the
        // right tier and no higher.
        let screen = DirectionalScreen::compute(
            legal,
            BaselineTier::ExactRoot,
            &bounds0,
            &directionals,
            field0.field_id(),
            field1.field_id(),
            walt::solver::adaptive::root_identity(&root, &position),
        );
        let kind = screen.kind();
        if screen.admissible().len() == 1 {
            assert_eq!(kind, FieldSwapKind::FieldStableExactRoot);
        } else {
            assert_eq!(kind, FieldSwapKind::FieldSensitive);
        }
        // Exclusion soundness at the exact-root tier: an excluded action
        // is strictly σ1-suboptimal among the exact optimized values.
        let best1 = q1.iter().max().expect("actions").clone();
        for (i, action) in actions.iter().enumerate() {
            if !screen.admitted(*action) {
                assert!(q1[i] < best1);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// §8 Stage 4 — field-1 work runs on survivors only; the settled/selected
// comparison produces FieldDecisionChanged exactly when the choice moved.
// ---------------------------------------------------------------------------

#[test]
fn stage4_runs_field1_work_on_survivors_only_and_types_the_decision() {
    let r = receipt();
    let mut multi_survivor_seen = false;
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let legal = legal_root_actions(&root, &position);
        let actions: Vec<Domino> = legal.iter().collect();
        let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        let baseline0 =
            exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-s4-s0");
        let mut e4 = Vec::new();
        for action in &actions {
            let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
            e4.push(ActionExposureUpper {
                action: *action,
                bound: solve.e4_upper(),
            });
        }
        let screen = AdmissibleScreen::compute(
            legal,
            BaselineTier::ExactFrozenSet,
            &baseline0.point_bounds(),
            &e4,
            field0.field_id(),
            field1.field_id(),
            walt::solver::adaptive::root_identity(&root, &position),
        );
        let stage4 = walt::solver::field_swap::survivor_stage4(
            &root,
            &position,
            &screen,
            &baseline0,
            &candidates,
            &field1,
            "gate-s4-s1",
        );
        // The survivors ARE the admissible set, in order.
        assert_eq!(stage4.survivors, screen.admissible());
        // settled0 recomputed: first σ0-argmax in candidate order.
        let best0 = baseline0.values.iter().max().expect("actions");
        let settled0 = *baseline0
            .actions
            .iter()
            .zip(&baseline0.values)
            .find(|(_, v)| *v == best0)
            .map(|(a, _)| a)
            .expect("attained");
        assert_eq!(stage4.settled0, settled0);
        match &stage4.values1 {
            None => {
                // Singleton: zero σ1 work, the stability kind stands.
                assert_eq!(stage4.survivors.len(), 1);
                assert_eq!(stage4.selected1, stage4.survivors[0]);
                assert_eq!(stage4.kind, screen.kind());
                assert!(!stage4.decision_changed());
            }
            Some(values1) => {
                multi_survivor_seen = true;
                // Field-1 work covered exactly the survivors — nothing
                // excluded consumed any σ1 budget.
                assert_eq!(values1.actions, stage4.survivors);
                // The Stage-4 selection equals the full σ1 pass's
                // selection: exclusion soundness (O32) guarantees every
                // σ1-best action survived, and both selections take the
                // first argmax in the shared order.
                let full1 = exact_frozen_action_values(
                    &root,
                    &position,
                    &candidates,
                    &field1,
                    "gate-s4-full",
                );
                let best1 = full1.values.iter().max().expect("actions");
                let full_selected = *full1
                    .actions
                    .iter()
                    .zip(&full1.values)
                    .find(|(_, v)| *v == best1)
                    .map(|(a, _)| a)
                    .expect("attained");
                assert_eq!(stage4.selected1, full_selected);
                // The typed kind: FieldDecisionChanged iff the settled
                // choice moved, else the screen's own kind.
                if stage4.decision_changed() {
                    assert_eq!(stage4.kind, FieldSwapKind::FieldDecisionChanged);
                } else {
                    assert_eq!(stage4.kind, screen.kind());
                }
            }
        }
    }
    assert!(
        multi_survivor_seen,
        "some parity root keeps several survivors; if this ever fails the \
         roots or the declared pair must change"
    );
}

// ---------------------------------------------------------------------------
// §3.3 / §9.1–9.2 — the pair lift Λ with its exposure bound, and the Λ
// evidence processes driven through the CE engine (consumed, not
// reimplemented).
// ---------------------------------------------------------------------------

#[test]
fn pair_lift_and_its_evidence_processes_are_exact() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let a = actions[0];
    let b = actions[1];
    let rho_a = pinned(&position, a);
    let rho_b = pinned(&position, b);
    let exposure_a = frozen_policy_exposure(
        &root,
        &position,
        &rho_a,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    let exposure_b = frozen_policy_exposure(
        &root,
        &position,
        &rho_b,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    let ladder_a = CancellationLadder::from_exposure(&exposure_a);
    let ladder_b = CancellationLadder::from_exposure(&exposure_b);
    let lift = pair_lift(&ladder_a, &ladder_b);
    // Λ = c_a − c_b, and |Λ| ≤ d_a + d_b (asserted at construction,
    // restated here).
    assert_eq!(lift.lambda, ladder_a.c() - ladder_b.c());
    assert!(abs(lift.lambda.clone()) <= lift.bound);
    assert_eq!(lift.bound, ladder_a.d() + ladder_b.d());
    // The cross-producer identity: Λ equals the frozen-value correction
    // difference from the controller's exact endpoint.
    let candidates: Vec<(Domino, &FrozenPolicy)> = vec![(a, &rho_a), (b, &rho_b)];
    let v0 = exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-l-s0");
    let v1 = exact_frozen_action_values(&root, &position, &candidates, &field1, "gate-l-s1");
    assert_eq!(
        lift.lambda,
        (v1.value(a) - v0.value(a)) - (v1.value(b) - v0.value(b)),
        "Λ re-derives from two independent exact producers"
    );
    // §9.1 — the signed-pivotal process on C_ρ consumes the CE machinery
    // verbatim.
    assert_eq!(
        correction_pivotal_evidence(&ladder_a),
        walt::solver::evidence::pivotal_evidence(ladder_a.c_plus, ladder_a.c_minus)
    );
    // §9.2 — the pair-lift bounded-mean process on X = Z/2: fold every
    // world's (C_a, C_b) pair (the two exposures enumerate the same
    // fiber in the same order — asserted), then re-derive the mixture
    // evidence independently.
    let lambda = q(1, 2);
    let threshold = q(0, 1);
    let mut process = PairLiftProcess::new(
        MeanNull::AtMost,
        &threshold,
        &[(BigRational::one(), lambda.clone())],
    )
    .expect("a lawful mixture");
    let mut expected = BigRational::one();
    for (row_a, row_b) in exposure_a.rows.iter().zip(&exposure_b.rows) {
        assert_eq!(row_a.world, row_b.world, "one shared enumeration order");
        let ca = i8::from(row_a.u1) - i8::from(row_a.u0);
        let cb = i8::from(row_b.u1) - i8::from(row_b.u0);
        process.observe(ca, cb);
        let x = BigRational::new(BigInt::from(ca - cb), BigInt::from(2));
        expected *= affine_factor(MeanNull::AtMost, &lambda, &(&threshold / q(2, 1)), &x);
    }
    assert_eq!(process.observations(), exposure_a.worlds);
    assert_eq!(process.evidence(), expected, "the mixture re-derives");
    // An observation outside {-1, 0, 1} is a contract violation.
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let mut p =
            PairLiftProcess::new(MeanNull::AtMost, &q(0, 1), &[(BigRational::one(), q(1, 2))])
                .expect("a lawful mixture");
        p.observe(2, 0);
    }))
    .is_err());
}

// ---------------------------------------------------------------------------
// §10 / §10.1 — first-split traces for every correction-pivotal world and
// the aggregate explanation, re-derived from the rows.
// ---------------------------------------------------------------------------

#[test]
fn split_traces_cover_every_changed_world_and_the_aggregate_rederives() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let mut some_changed_world = false;
    for action in legal_root_actions(&root, &position).iter() {
        let rho = pinned(&position, action);
        let exposure = frozen_policy_exposure(
            &root,
            &position,
            &rho,
            &field0,
            &field1,
            WorldDomain::ExactFiber,
        );
        let traces = field_split_traces(&exposure, action);
        assert_eq!(
            u64::try_from(traces.len()).expect("fits"),
            exposure.corrections_plus + exposure.corrections_minus,
            "one trace per correction-pivotal world"
        );
        for trace in &traces {
            assert_ne!(trace.favors(), 0, "a trace world changed its outcome");
            assert_eq!(trace.action, action);
            assert_eq!(trace.policy, exposure.policy);
        }
        if !traces.is_empty() {
            some_changed_world = true;
        }
        let aggregate = SplitAggregate::from_exposure(&exposure);
        assert_eq!(aggregate.exposed, exposure.exposed);
        assert_eq!(aggregate.plus, exposure.corrections_plus);
        assert_eq!(aggregate.minus, exposure.corrections_minus);
        assert_eq!(aggregate.by_seat.iter().sum::<u64>(), aggregate.exposed);
        assert_eq!(
            aggregate.by_trick.iter().map(|(_, n)| n).sum::<u64>(),
            aggregate.exposed
        );
        match aggregate.conditional_outcome_difference() {
            Some(diff) => {
                assert!(aggregate.exposed > 0);
                assert_eq!(
                    diff,
                    BigRational::new(
                        BigInt::from(aggregate.plus) - BigInt::from(aggregate.minus),
                        BigInt::from(aggregate.exposed)
                    )
                );
            }
            None => assert_eq!(aggregate.exposed, 0),
        }
    }
    assert!(
        some_changed_world,
        "the split-heavy parity root changes some outcome; if this ever \
         fails the roots or the declared pair must change"
    );
}
