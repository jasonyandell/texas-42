//! Gates for the targeted field-1 controller (`solver::targeted`) —
//! parent `walt/math/targeted_level2_field_stability_v0.1.md` §8 Stages
//! 1–5 assembled; rulings L2-A1..A7, PANEL-A7/A8, TRIPLE-A2;
//! obligations O31/O32/O34/O38 of `walt/SCENARIO-PLAYER.md` §10.
//!
//! DECLARED TEST EPOCH PAIR (one (σ0, σ1) pair per experiment epoch):
//! σ0 = Level0 { n0 = 2 }, σ1 = Level1 { n_outer = 2, n0 = 2 }, frozen
//! focal candidates at declared schedule [2, 2] — the same cheap pair the
//! sibling screen gates declare. Roots from the frozen `verify_player`
//! receipt: hand 4 trick 6 (fiber 90), hand 8 trick 5 (fiber 92), hand
//! 10 trick 6 (fiber 19).
//!
//! The compile_fail typing locks (a steering lower witness is never a
//! screen bound; a typed refusal carries no number; a δ-valid baseline
//! is not an exact one) are doctests on the `solver::targeted` module.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::Domino;
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::evidence::ScopedDelta;
use walt::solver::exposure::{exact_split_reach, ExposureRung, RootActionExposureUpper};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    exact_frozen_action_values, ActionBound, ActionExposureUpper, AdmissibleScreen, BaselineTier,
    FieldSwapKind,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::targeted::{
    delta_frozen_baseline, legal_root_actions, steering_admissible_for_gate, targeted_root,
    DirectionalPhase, EscalationStop, RefusalReason, RungBudget, StageFourOutcome, TargetedConfig,
    TargetedRisk,
};

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

/// The frozen candidate family in legal-set order.
fn family(root: &CanonicalRoot, position: &RootPosition) -> (Vec<Domino>, Vec<FrozenPolicy>) {
    let actions: Vec<Domino> = legal_root_actions(root, position).iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(position, *a)).collect();
    (actions, policies)
}

fn exact_config(scope: &str) -> TargetedConfig {
    TargetedConfig {
        budget: RungBudget {
            exact_fiber_cap: 4096,
            baseline_prefix: 0,
            e3_prefix: 0,
            directional: true,
        },
        risk: None,
        epsilon: Some(q(1, 20)),
        epoch: 0,
        scope: scope.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Worked specimen 1 — the screen prunes. A behaviorally equal field pair
// (two distinct FieldIds over the same behavior — the L2-E0 "fields never
// disagree" fixture family) makes rung E0 fire on every action, so the
// screen is exactly the σ0 argmax and the singleton costs no σ1 work and
// no E4 solve.
// ---------------------------------------------------------------------------

#[test]
fn equal_field_pair_prunes_to_the_sigma0_argmax_without_e4_or_sigma1_spend() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let field_a = FieldModel::new(field0_spec());
    let field_b = FieldModel::new(FieldSpec {
        construction: "level0-modeled-mind-v1-relabeled".to_string(),
        ..field0_spec()
    });
    assert_ne!(field_a.field_id(), field_b.field_id());
    let (actions, policies) = family(&root, &position);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let report = targeted_root(
        &root,
        &position,
        &candidates,
        &field_a,
        &field_b,
        &exact_config("gate-equal"),
    );
    assert_eq!(report.tier, BaselineTier::ExactFrozenSet);
    // Every action's bound reached zero at rung E0.
    for row in &report.rows {
        assert_eq!(row.exposure.rung(), ExposureRung::E0);
        assert!(row.exposure.screenable_upper().is_zero());
    }
    // With R = 0 everywhere the admissible set is exactly the σ0 argmax
    // set; on this root the σ0 frozen values have a strict winner, so
    // the screen prunes to a singleton and stops.
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field_a, "gate-equal-check");
    let best0 = baseline0.values.iter().max().expect("actions").clone();
    let winners: Vec<Domino> = baseline0
        .actions
        .iter()
        .zip(&baseline0.values)
        .filter(|(_, v)| **v == best0)
        .map(|(a, _)| *a)
        .collect();
    assert_eq!(
        winners.len(),
        1,
        "KNOWN CHECK VALUE: receipt-h4-t6 has a strict σ0 argmax at the cheap pair"
    );
    assert_eq!(report.survivors(), winners);
    assert_eq!(report.stop, EscalationStop::Pruned);
    assert_eq!(report.kind, FieldSwapKind::FieldStableExactFrozenSet);
    // No E4 solve and no σ1 evaluation were paid for.
    assert!(report.spend.iter().all(|p| p.phase != "rung-e4"));
    let StageFourOutcome::ExactSurvivors {
        evaluation,
        ladders,
    } = &report.stage4
    else {
        panic!("the exact route reports the exact Stage-4 outcome");
    };
    assert!(
        evaluation.values1.is_none(),
        "a singleton consumes no σ1 work"
    );
    assert!(ladders.is_empty(), "no σ1 work, no σ1 explanation surface");
    assert!(report.refusals.is_empty());
    assert!(
        report.risk_spent.is_none(),
        "the exact route consumes no sampling risk"
    );
}

// ---------------------------------------------------------------------------
// Worked specimen 2 — the real declared pair across the parity roots:
// soundness (excluded ⇒ strictly σ1-nonoptimal, replayed independently),
// honest degradation (spend-controlled escalation with its typed stop),
// and the steering lemma verified against actual E4 wherever steering
// refused the spend.
// ---------------------------------------------------------------------------

#[test]
fn controller_is_sound_and_steering_refusals_are_verified_against_e4() {
    let r = receipt();
    let mut steering_refusal_seen = false;
    for (hand_id, trick_no, fiber) in [(4usize, 6usize, 90u128), (8, 5, 92), (10, 6, 19)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let (actions, policies) = family(&root, &position);
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        let report = targeted_root(
            &root,
            &position,
            &candidates,
            &field0,
            &field1,
            &exact_config(&format!("gate-real-h{hand_id}-t{trick_no}")),
        );
        assert_eq!(report.tier, BaselineTier::ExactFrozenSet);
        assert_eq!(
            report.rows.len(),
            actions.len(),
            "O38: every action has a row"
        );
        // Every bound is rung-labeled and in [0, 1].
        for row in &report.rows {
            assert!(matches!(
                row.exposure.rung(),
                ExposureRung::E0 | ExposureRung::E1 | ExposureRung::E2 | ExposureRung::E4
            ));
            assert!(row.exposure.screenable_upper() <= &BigRational::one());
        }
        // Exclusion soundness replayed with an independent full σ1 pass.
        let baseline1 = exact_frozen_action_values(
            &root,
            &position,
            &candidates,
            &field1,
            "gate-real-sigma1-audit",
        );
        let best1 = baseline1.values.iter().max().expect("actions").clone();
        let survivors = report.survivors();
        for action in &actions {
            if !survivors.contains(action) {
                assert!(
                    *baseline1.value(*action) < best1,
                    "excluded {action} must be strictly σ1-nonoptimal"
                );
            }
        }
        // The Stage-4 outcome is confined to the survivors and its σ1
        // values agree with the independent pass.
        let StageFourOutcome::ExactSurvivors {
            evaluation,
            ladders,
        } = &report.stage4
        else {
            panic!("the exact route reports the exact Stage-4 outcome");
        };
        assert_eq!(evaluation.survivors, survivors);
        if let Some(values1) = &evaluation.values1 {
            for (action, value) in values1.actions.iter().zip(&values1.values) {
                assert_eq!(value, baseline1.value(*action), "survivor σ1 parity");
            }
            // Every survivor carries its explanation ladder, and the
            // ladder's c is the exact value wake V₁(ρ) − V₀(ρ).
            assert_eq!(ladders.len(), survivors.len());
            let baseline0 = exact_frozen_action_values(
                &root,
                &position,
                &candidates,
                &field0,
                "gate-real-sigma0-audit",
            );
            for l in ladders {
                let c = baseline1.value(l.action) - baseline0.value(l.action);
                assert_eq!(l.ladder.c(), c, "the ladder's c IS the exact value wake");
            }
        } else {
            assert!(ladders.is_empty());
        }
        // The steering lemma, verified: where steering refused E4 spend,
        // the actual exact E4 bounds must leave the admissible set
        // unchanged (the monotone sandwich 𝓐₁(ℓ) ⊆ 𝓐₁(R*) ⊆ 𝓐₁(R^cur)).
        if report.stop == EscalationStop::ProvablyUseless {
            steering_refusal_seen = true;
            assert!(
                report.spend.iter().all(|p| p.phase != "rung-e4"),
                "a provably-useless stop paid for no E4 solve"
            );
            let baseline0 = exact_frozen_action_values(
                &root,
                &position,
                &candidates,
                &field0,
                "gate-lemma-sigma0",
            );
            let e4: Vec<ActionExposureUpper> = actions
                .iter()
                .map(|action| ActionExposureUpper {
                    action: *action,
                    bound: exact_split_reach(&root, &position, *action, &field0, &field1)
                        .e4_upper(),
                })
                .collect();
            let screen_e4 = AdmissibleScreen::compute(
                legal_root_actions(&root, &position),
                BaselineTier::ExactFrozenSet,
                &baseline0.point_bounds(),
                &e4,
                field0.field_id(),
                field1.field_id(),
                report.root_id,
            );
            assert_eq!(
                screen_e4.admissible(),
                survivors,
                "the lemma holds: exact E4 could not have shrunk the admissible set"
            );
        }
        // Directional honesty: the phase is typed, and where it ran the
        // directional screen only pruned MORE.
        match report.directional_phase {
            DirectionalPhase::Ran => {
                let dir = report
                    .directional
                    .as_ref()
                    .expect("a ran phase has a screen");
                for action in dir.admissible() {
                    assert!(survivors.contains(&action));
                }
            }
            DirectionalPhase::NotAdmitted | DirectionalPhase::SkippedProvablyUseless => {
                assert!(report.directional.is_none());
            }
        }
        // Spend is recorded for every phase that ran, cheapest first.
        let phases: Vec<&str> = report.spend.iter().map(|p| p.phase).collect();
        assert_eq!(phases[0], "baseline-sigma0");
        assert_eq!(phases[1], "rung-e1");
        assert!(report.spend.iter().any(|p| p.phase == "stage4-sigma1"));
    }
    assert!(
        steering_refusal_seen,
        "some parity root exercises the provably-useless steering refusal; \
         if this ever fails the roots or the declared pair must change"
    );
}

// ---------------------------------------------------------------------------
// Honest degradation — over the exact cap with no declared sampled route
// the outcome is a typed refusal, never a silently degenerate bound.
// ---------------------------------------------------------------------------

#[test]
fn over_cap_without_a_declared_sampled_route_is_a_typed_refusal() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    assert_eq!(root.count(), 19);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let (actions, policies) = family(&root, &position);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let config = TargetedConfig {
        budget: RungBudget {
            exact_fiber_cap: 10,
            baseline_prefix: 0,
            e3_prefix: 0,
            directional: false,
        },
        risk: None,
        epsilon: None,
        epoch: 0,
        scope: "gate-refusal".to_string(),
    };
    let report = targeted_root(&root, &position, &candidates, &field0, &field1, &config);
    assert_eq!(report.tier, BaselineTier::Unresolved);
    assert_eq!(report.kind, FieldSwapKind::FieldUnresolved);
    assert_eq!(report.stop, EscalationStop::Refused);
    assert!(
        report.screen.is_none(),
        "no screen exists without a baseline"
    );
    assert!(report.rows.is_empty(), "no numbers accompany the refusal");
    assert!(report.risk_spent.is_none());
    assert!(report
        .refusals
        .iter()
        .any(|f| f.reason == RefusalReason::ExactUnaffordable { fiber: 19, cap: 10 }));
    assert!(report
        .refusals
        .iter()
        .any(|f| f.reason == RefusalReason::SampledRouteUndeclared));
    let StageFourOutcome::NotRun(refusal) = &report.stage4 else {
        panic!("stage 4 reports its refusal");
    };
    assert_eq!(refusal.reason, RefusalReason::SampledRouteUndeclared);
}

// ---------------------------------------------------------------------------
// Honest degradation — the declared sampled route: δ-tier baselines whose
// realized intervals cover the exact values on this deterministic stream,
// rung-labeled E1/E3 bounds only, and the exact-rational risk ledger
// within the declared screen budget.
// ---------------------------------------------------------------------------

#[test]
fn sampled_route_runs_at_the_delta_tier_with_a_scoped_risk_ledger() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let (actions, policies) = family(&root, &position);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let config = TargetedConfig {
        budget: RungBudget {
            exact_fiber_cap: 10,
            baseline_prefix: 48,
            e3_prefix: 8,
            directional: false,
        },
        risk: Some(TargetedRisk {
            screen_budget: ScopedDelta::new("gate-sampled:screen", q(1, 50)),
            per_baseline_side: q(1, 800),
            per_e3: q(1, 400),
        }),
        epsilon: None,
        epoch: 7,
        scope: "gate-sampled".to_string(),
    };
    let report = targeted_root(&root, &position, &candidates, &field0, &field1, &config);
    assert_eq!(report.tier, BaselineTier::DeltaFrozenSet);
    assert_eq!(report.rows.len(), actions.len());
    // Every bound is E1 (degenerate, stated) or E3 (δ-valid upper CS).
    for row in &report.rows {
        assert!(matches!(
            row.exposure.rung(),
            ExposureRung::E1 | ExposureRung::E3
        ));
        assert!(
            row.steering.is_none(),
            "no exact lower witness at this scale"
        );
    }
    // The realized δ intervals cover the exact values on this frozen
    // deterministic stream (a KNOWN CHECK on the fixed stream, not a
    // theorem about every stream — coverage is probabilistic at δ).
    let baseline1 = exact_frozen_action_values(
        &root,
        &position,
        &candidates,
        &field1,
        "gate-sampled-sigma1-audit",
    );
    let baseline0 = exact_frozen_action_values(
        &root,
        &position,
        &candidates,
        &field0,
        "gate-sampled-sigma0-audit",
    );
    for row in &report.rows {
        let v0 = baseline0.value(row.action);
        assert!(
            &row.lower0 <= v0 && v0 <= &row.upper0,
            "the realized σ0 interval covers the exact value at {}",
            row.action
        );
    }
    // No stability claim outruns the tier.
    assert!(matches!(
        report.kind,
        FieldSwapKind::FieldStableDeltaFrozenSet
            | FieldSwapKind::FieldSensitive
            | FieldSwapKind::FieldDecisionChanged
            | FieldSwapKind::FieldUnresolved
    ));
    // The risk ledger: exact-rational total within the declared budget.
    let spent = report
        .risk_spent
        .as_ref()
        .expect("the sampled route spends risk");
    assert!(spent <= &q(1, 50));
    assert!(spent > &BigRational::zero());
    // Stage-4 honesty at the δ tier.
    match &report.stage4 {
        StageFourOutcome::DeltaSingleton { selected } => {
            assert_eq!(report.survivors(), vec![*selected]);
        }
        StageFourOutcome::DeltaSurvivors {
            sigma1,
            settled0,
            selected1,
        } => {
            // Settled selections, where present, agree with interval
            // separation; and the σ1 intervals cover the exact σ1 values
            // on this stream.
            for action in &sigma1.actions {
                let v1 = baseline1.value(*action);
                assert!(
                    sigma1.lower(*action) <= v1 && v1 <= sigma1.upper(*action),
                    "the realized σ1 interval covers the exact value at {action}"
                );
            }
            if let Some(a) = selected1 {
                for b in &sigma1.actions {
                    if b != a {
                        assert!(sigma1.lower(*a) > sigma1.upper(*b));
                    }
                }
            }
            let _ = settled0;
        }
        StageFourOutcome::ExactSurvivors { .. } | StageFourOutcome::NotRun(_) => {
            panic!("the sampled route reports a δ-tier Stage-4 outcome")
        }
    }
    // The over-cap refusal is still on the record: the route itself is a
    // typed degradation, not a silent downgrade.
    assert!(report
        .refusals
        .iter()
        .any(|f| matches!(f.reason, RefusalReason::ExactUnaffordable { .. })));
}

// ---------------------------------------------------------------------------
// The steering arithmetic agrees with the screen authority on identical
// valid-upper inputs (the parity gate for the spend-steering hypothetical).
// ---------------------------------------------------------------------------

#[test]
fn steering_arithmetic_agrees_with_the_screen_on_valid_uppers() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let (actions_f, policies) = family(&root, &position);
    assert_eq!(actions, actions_f);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-steering");
    let bounds0 = baseline0.point_bounds();
    // A spread of upper values, including both endpoints.
    let uppers: Vec<BigRational> = (0..actions.len())
        .map(|k| q(i64::try_from(k).expect("small"), 4).min(q(1, 1)))
        .collect();
    let exposures: Vec<ActionExposureUpper> = actions
        .iter()
        .zip(&uppers)
        .map(|(a, u)| ActionExposureUpper {
            action: *a,
            bound: RootActionExposureUpper::from_rung(ExposureRung::E2, u.clone()),
        })
        .collect();
    let screen = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds0,
        &exposures,
        field0.field_id(),
        field1.field_id(),
        0,
    );
    assert_eq!(
        steering_admissible_for_gate(&bounds0, &uppers),
        screen.admissible(),
        "the steering hypothetical IS the screen arithmetic on valid uppers"
    );
}

// ---------------------------------------------------------------------------
// TRIPLE-A2 §1.8 — a risk plan whose entries overrun the declared screen
// budget aborts loudly instead of spending it.
// ---------------------------------------------------------------------------

#[test]
fn a_risk_overrun_panics_instead_of_spending() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let (actions, policies) = family(&root, &position);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let config = TargetedConfig {
        budget: RungBudget {
            exact_fiber_cap: 10,
            baseline_prefix: 16,
            e3_prefix: 0,
            directional: false,
        },
        risk: Some(TargetedRisk {
            // Two actions × two sides × 1/4 = 1 > 1/100: an overrun.
            screen_budget: ScopedDelta::new("gate-overrun:screen", q(1, 100)),
            per_baseline_side: q(1, 4),
            per_e3: q(1, 4),
        }),
        epsilon: None,
        epoch: 0,
        scope: "gate-overrun".to_string(),
    };
    assert!(catch_unwind(AssertUnwindSafe(|| {
        targeted_root(&root, &position, &candidates, &field0, &field1, &config)
    }))
    .is_err());
}

// ---------------------------------------------------------------------------
// The δ-valid baseline producer: interval shape, scoped entries, and the
// honest open selection.
// ---------------------------------------------------------------------------

#[test]
fn delta_baseline_intervals_are_scoped_grid_valid_and_honestly_open() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    let field0 = FieldModel::new(field0_spec());
    let (actions, policies) = family(&root, &position);
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let baseline = delta_frozen_baseline(
        &root,
        &position,
        &candidates,
        &field0,
        3,
        32,
        &q(1, 200),
        "gate-delta",
    );
    assert_eq!(
        baseline.grid,
        root.count(),
        "the grid is the exact fiber count"
    );
    assert_eq!(baseline.worlds, 32);
    let bounds = baseline.bounds();
    assert_eq!(bounds.len(), actions.len());
    for b in &bounds {
        assert!(BigRational::zero() <= b.lower);
        assert!(b.lower <= b.upper);
        assert!(b.upper <= BigRational::one());
        // Endpoints live on the grid G_N: denominator divides N.
        let n = BigInt::from(baseline.grid);
        assert_eq!((&n % b.lower.denom()), BigInt::from(0));
        assert_eq!((&n % b.upper.denom()), BigInt::from(0));
    }
    // Two one-sided entries per action, all scopes distinct.
    let entries = baseline.risk_entries();
    assert_eq!(entries.len(), 2 * actions.len());
    for (i, e) in entries.iter().enumerate() {
        for other in &entries[i + 1..] {
            assert_ne!(e.scope(), other.scope());
        }
    }
    // The settled selection is honest: it exists exactly when one lower
    // endpoint strictly clears every rival upper endpoint.
    let settled = baseline.settled_argmax();
    let separated = actions.iter().find(|a| {
        actions
            .iter()
            .filter(|b| b != a)
            .all(|b| baseline.lower(**a) > baseline.upper(*b))
    });
    assert_eq!(settled, separated.copied());
    let _ = ActionBound {
        action: actions[0],
        lower: q(0, 1),
        upper: q(1, 1),
    };
}
