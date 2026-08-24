//! Gates for the second field-swap vertical slice: the exposure rung
//! producers E0/E1/E2 and the exact split-reach route (rung E4) in
//! `solver::exposure`, and the admissible level-2 action set in
//! `solver::field_swap` — parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §5, §7, §8 Stages
//! 1–3 (§21 steps 6–8); rulings L2-A1..A7; obligations O31/O32/O34/O38 of
//! `walt/SCENARIO-PLAYER.md` §10.
//!
//! DECLARED TEST EPOCH PAIR (one (σ0, σ1) pair per experiment epoch —
//! slice 1's open question resolved by declaration): the tests declare
//! σ0 = Level0 { n0 = 2 } and σ1 = Level1 { n_outer = 2, n0 = 2 }; frozen
//! focal candidates run at declared schedule [2, 2]. The probe epoch
//! declares a different pair (see `walt/probes/fieldswap_screen/`); each
//! epoch's pair is carried by its FieldIds in every result.
//!
//! Exact parity roots (O32's route: exact small-fiber parity), all from
//! the frozen `verify_player` receipt: hand 4 trick 6 (fiber 90 — slice
//! 1's small root), hand 8 trick 5 (fiber 92), hand 10 trick 6 (fiber
//! 19). At each root the viewer leads, so every held tile is a legal root
//! action.

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
use walt::solver::exposure::{
    clairvoyant_reach, exact_split_reach, frozen_policy_exposure, rung_e1, ExposureRung,
    ForcedNonFocalCover, RootActionExposureUpper, StructuralSplitCover, TrivialSplitCover,
    WorldDomain,
};
use walt::solver::field::{FieldId, FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    exact_frozen_action_values, ActionBound, ActionExposureUpper, AdmissibleScreen, BaselineTier,
    ExactFrozenBaseline, FieldSwapKind,
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

/// The legal root actions of the viewer's decision at the root state.
fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

/// The complete per-root screen pipeline the parity gates share: frozen
/// candidates, exact σ0 baseline, per-action E2 and E4 exposure bounds,
/// and the exact σ1 values for the SAME frozen candidates.
struct ParityRun {
    legal: DominoSet,
    actions: Vec<Domino>,
    baseline0: ExactFrozenBaseline,
    baseline1: ExactFrozenBaseline,
    e2: Vec<ActionExposureUpper>,
    e4: Vec<ActionExposureUpper>,
    /// Exact R_a per action, aligned with `actions`.
    r_exact: Vec<BigRational>,
    field0_id: FieldId,
    field1_id: FieldId,
    root_id: u64,
}

fn parity_run(root: &CanonicalRoot, position: &RootPosition) -> ParityRun {
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let legal = legal_root_actions(root, position);
    assert!(legal.len() >= 2, "a screened parity root has a real choice");
    let actions: Vec<Domino> = legal.iter().collect();
    // Acceptance item 3: the focal candidates are frozen once, before any
    // cross-field evidence, and reused across σ0, σ1, and the rungs.
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(position, *a)).collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let baseline0 = exact_frozen_action_values(root, position, &candidates, &field0, "gate-sigma0");
    let baseline1 = exact_frozen_action_values(root, position, &candidates, &field1, "gate-sigma1");
    let mut e2 = Vec::new();
    let mut e4 = Vec::new();
    let mut r_exact = Vec::new();
    for action in &actions {
        let reach = clairvoyant_reach(root, position, *action, &field0, &field1);
        let solve = exact_split_reach(root, position, *action, &field0, &field1);
        r_exact.push(solve.r());
        e2.push(ActionExposureUpper {
            action: *action,
            bound: reach.e2_upper(),
        });
        e4.push(ActionExposureUpper {
            action: *action,
            bound: solve.e4_upper(),
        });
    }
    ParityRun {
        legal,
        actions,
        baseline0,
        baseline1,
        e2,
        e4,
        r_exact,
        field0_id: field0.field_id(),
        field1_id: field1.field_id(),
        root_id: walt::solver::adaptive::root_identity(root, position),
    }
}

// ---------------------------------------------------------------------------
// Rung E0 (§7.1): fires exactly when no reachable non-focal state
// disagrees, and its bound is exactly zero.
// ---------------------------------------------------------------------------

#[test]
fn e0_fires_on_a_behaviorally_equal_field_pair_and_is_exactly_zero() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    // Two DISTINCT FieldIds (the construction string is identity) over the
    // same behavior: the L2-E0 fixture family's "fields never disagree".
    let field_a = FieldModel::new(field0_spec());
    let field_b = FieldModel::new(FieldSpec {
        construction: "level0-modeled-mind-v1-relabeled".to_string(),
        ..field0_spec()
    });
    assert_ne!(field_a.field_id(), field_b.field_id());
    for action in legal_root_actions(&root, &position).iter() {
        let reach = clairvoyant_reach(&root, &position, action, &field_a, &field_b);
        assert_eq!(reach.reach_worlds, 0, "equal behavior never reaches F");
        let e0 = reach.e0_upper().expect("E0 fires on zero reach");
        assert_eq!(e0.rung(), ExposureRung::E0);
        assert_eq!(e0.screenable_upper(), &BigRational::zero());
        // The exact solve agrees: R_a = 0 exactly.
        let solve = exact_split_reach(&root, &position, action, &field_a, &field_b);
        assert_eq!(solve.frontier_worlds, 0);
        assert_eq!(solve.r(), BigRational::zero());
    }
}

#[test]
fn e0_makes_no_claim_when_a_disagreement_is_reachable() {
    let r = receipt();
    let mut disagreement_seen = false;
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        for action in legal_root_actions(&root, &position).iter() {
            let reach = clairvoyant_reach(&root, &position, action, &field0, &field1);
            // E0 fires exactly when the E2 mass is zero — never both ways.
            assert_eq!(reach.e0_upper().is_some(), reach.reach_worlds == 0);
            if reach.reach_worlds > 0 {
                disagreement_seen = true;
            }
        }
    }
    assert!(
        disagreement_seen,
        "the declared test pair disagrees somewhere on the parity roots; \
         if this ever fails the roots or the declared pair must change"
    );
}

// ---------------------------------------------------------------------------
// The rung ladder is ordered (§7): E1-trivial ≥ E2 ≥ E4 = exact R_a ≥ the
// exact exposure of any frozen ρ_a ∈ Π_a — all exact rationals.
// ---------------------------------------------------------------------------

#[test]
fn rung_bounds_dominate_in_ladder_order_on_every_parity_root() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber, "the declared parity fiber");
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let trivial = rung_e1(&root, &TrivialSplitCover);
        assert_eq!(trivial.rung(), ExposureRung::E1);
        assert_eq!(trivial.screenable_upper(), &BigRational::one());
        for action in legal_root_actions(&root, &position).iter() {
            let reach = clairvoyant_reach(&root, &position, action, &field0, &field1);
            let solve = exact_split_reach(&root, &position, action, &field0, &field1);
            let e2 = reach.e2_upper();
            let e4 = solve.e4_upper();
            assert_eq!(e2.rung(), ExposureRung::E2);
            assert_eq!(e4.rung(), ExposureRung::E4);
            // Clairvoyant strategy fusion is safe-direction only: the
            // cover mass dominates the exact optimum (§7.3).
            assert!(e2.screenable_upper() >= e4.screenable_upper());
            assert!(trivial.screenable_upper() >= e2.screenable_upper());
            assert!(u64::from(solve.frontier_worlds <= reach.reach_worlds) == 1);
            // A frozen pinned continuation is one member of Π_a: its exact
            // fixed-policy exposure is a LOWER witness to R_a (§7.4).
            let rho = pinned(&position, action);
            let exposure = frozen_policy_exposure(
                &root,
                &position,
                &rho,
                &field0,
                &field1,
                WorldDomain::ExactFiber,
            );
            assert!(exposure.d_hat() <= solve.r());
        }
    }
}

// ---------------------------------------------------------------------------
// Rung E1 structural covers (§7.2): exact counted mass; the forced cover
// proves zero where every non-focal decision is forced.
// ---------------------------------------------------------------------------

#[test]
fn structural_covers_are_rung_e1_with_exact_counted_mass() {
    let r = receipt();
    // Horizon 1: every hidden capacity is 1, so every non-focal decision
    // is forced and the forced cover proves R_a = 0 at rung E1.
    let (root7, position7) = root_at(&r, 4, 7);
    assert!(root7.kernel().hidden().iter().all(|h| h.capacity <= 1));
    let forced = rung_e1(&root7, &ForcedNonFocalCover);
    assert_eq!(forced.rung(), ExposureRung::E1);
    assert_eq!(forced.screenable_upper(), &BigRational::zero());
    // The reach walk agrees exactly.
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    for action in legal_root_actions(&root7, &position7).iter() {
        let reach = clairvoyant_reach(&root7, &position7, action, &field0, &field1);
        assert_eq!(reach.reach_worlds, 0);
        let solve = exact_split_reach(&root7, &position7, action, &field0, &field1);
        assert_eq!(solve.frontier_worlds, 0);
    }
    // At a deeper root the same predicate is honestly loose: mass 1.
    let (root5, _) = root_at(&r, 8, 5);
    assert!(root5.kernel().hidden().iter().any(|h| h.capacity > 1));
    let loose = rung_e1(&root5, &ForcedNonFocalCover);
    assert_eq!(loose.screenable_upper(), &BigRational::one());
    assert_eq!(ForcedNonFocalCover.id(), "forced-non-focal-cover-v1");
    assert_eq!(TrivialSplitCover.id(), "trivial-cover-v1");
}

// ---------------------------------------------------------------------------
// O32 — the exact small-fiber parity gate: exact Q^(0), exact R_a, exact
// Q^(1) for the frozen candidate sets; every excluded action truly
// σ1-nonoptimal; L2-T2/T3/T4 replayed with exact numbers.
// ---------------------------------------------------------------------------

#[test]
fn the_admissible_screen_is_sound_on_every_exact_parity_root() {
    let r = receipt();
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let run = parity_run(&root, &position);
        let v0 = |a: Domino| run.baseline0.value(a).clone();
        let v1 = |a: Domino| run.baseline1.value(a).clone();
        // L2-T2 with exact numbers, per action: |Q^(1) − Q^(0)| ≤ R_a for
        // the frozen candidates (d_ρ ≤ R_a, so the exact frozen values
        // obey the root Lipschitz bound).
        for (action, r_a) in run.actions.iter().zip(&run.r_exact) {
            let correction = v1(*action) - v0(*action);
            let magnitude = if correction < BigRational::zero() {
                -correction
            } else {
                correction
            };
            assert!(
                magnitude <= *r_a,
                "L2-T2: |Q1 - Q0| <= R_a exactly at {action}"
            );
        }
        // Screens at both rungs: the exact-value screen (E4) and the
        // looser clairvoyant screen (E2). Both must be sound; the looser
        // one may only admit MORE.
        for (exposures, label) in [(&run.e4, "E4"), (&run.e2, "E2")] {
            let screen = AdmissibleScreen::compute(
                run.legal,
                BaselineTier::ExactFrozenSet,
                &run.baseline0.point_bounds(),
                exposures,
                run.field0_id,
                run.field1_id,
                run.root_id,
            );
            // The bar recomputed independently: B = max_a (v0[a] − R_a^U).
            let bar = run
                .actions
                .iter()
                .map(|a| {
                    v0(*a)
                        - exposures
                            .iter()
                            .find(|e| e.action == *a)
                            .expect("bound")
                            .bound
                            .screenable_upper()
                })
                .max()
                .expect("actions");
            assert_eq!(screen.bar(), bar, "{label} bar arithmetic");
            let admissible = screen.admissible();
            assert!(!admissible.is_empty());
            // The best σ1 frozen value and its witnesses.
            let best1 = run.actions.iter().map(|a| v1(*a)).max().expect("actions");
            // L2-T4 soundness (O32): every excluded action is truly
            // σ1-nonoptimal within the frozen candidate set — strictly.
            for action in run.actions.iter() {
                if !admissible.contains(action) {
                    assert!(
                        v1(*action) < best1,
                        "{label}: excluded {action} must be strictly σ1-nonoptimal"
                    );
                }
            }
            // Equivalently: every σ1-best action is admitted.
            for action in run.actions.iter() {
                if v1(*action) == best1 {
                    assert!(admissible.contains(action));
                }
            }
            // Result kind: singleton ⇒ stability at the frozen-set tier,
            // else FieldSensitive; the serialization carries the tag and
            // both FieldIds (acceptance item 2).
            let kind = screen.kind();
            if admissible.len() == 1 {
                assert_eq!(kind, FieldSwapKind::FieldStableExactFrozenSet);
            } else {
                assert_eq!(kind, FieldSwapKind::FieldSensitive);
            }
            let serialized = screen.to_string();
            assert!(serialized.starts_with("AdmissibleScreen{kind="));
            assert!(serialized.contains(kind.tag()));
            assert!(serialized.contains(&run.field0_id.to_string()));
            assert!(serialized.contains(&run.field1_id.to_string()));
            assert!(serialized.contains("rung="));
        }
        // The E2 screen admits a superset of the E4 screen (looseness
        // costs pruning power, never soundness — §5).
        let screen_e4 = AdmissibleScreen::compute(
            run.legal,
            BaselineTier::ExactFrozenSet,
            &run.baseline0.point_bounds(),
            &run.e4,
            run.field0_id,
            run.field1_id,
            run.root_id,
        );
        let screen_e2 = AdmissibleScreen::compute(
            run.legal,
            BaselineTier::ExactFrozenSet,
            &run.baseline0.point_bounds(),
            &run.e2,
            run.field0_id,
            run.field1_id,
            run.root_id,
        );
        for action in screen_e4.admissible() {
            assert!(screen_e2.admissible().contains(&action));
        }
        // L2-T3 replay: when the σ0 winner's margin beats R_a + R_b for
        // every rival, the winner is strictly optimal under σ1.
        let best0 = run.actions.iter().map(|a| v0(*a)).max().expect("actions");
        let winners0: Vec<Domino> = run
            .actions
            .iter()
            .copied()
            .filter(|a| v0(*a) == best0)
            .collect();
        if winners0.len() == 1 {
            let a = winners0[0];
            let r_of = |x: Domino| {
                run.actions
                    .iter()
                    .position(|y| *y == x)
                    .map(|k| run.r_exact[k].clone())
                    .expect("action")
            };
            let dominates = run
                .actions
                .iter()
                .all(|b| *b == a || v0(a) - v0(*b) > r_of(a) + r_of(*b));
            if dominates {
                for b in run.actions.iter().filter(|b| **b != a) {
                    assert!(
                        v1(a) > v1(*b),
                        "L2-T3: a margin above R_a + R_b survives the swap"
                    );
                }
                assert_eq!(screen_e4.admissible(), vec![a]);
                assert_eq!(screen_e4.kind(), FieldSwapKind::FieldStableExactFrozenSet);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// O38 — the all-action admission audit: every legal action gets a bound
// before any exclusion; nothing extra, nothing twice.
// ---------------------------------------------------------------------------

#[test]
fn every_legal_action_receives_a_bound_before_any_exclusion() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let bound = |a: Domino| ActionBound {
        action: a,
        lower: q(1, 2),
        upper: q(1, 2),
    };
    let exposure = |a: Domino| ActionExposureUpper {
        action: a,
        bound: RootActionExposureUpper::from_rung(ExposureRung::E2, q(1, 4)),
    };
    let bounds: Vec<ActionBound> = actions.iter().map(|a| bound(*a)).collect();
    let exposures: Vec<ActionExposureUpper> = actions.iter().map(|a| exposure(*a)).collect();
    let (field0_id, field1_id) = (
        FieldModel::new(field0_spec()).field_id(),
        FieldModel::new(field1_spec()).field_id(),
    );
    // The complete inputs pass.
    let screen = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        &exposures,
        field0_id,
        field1_id,
        0,
    );
    assert_eq!(
        screen.admissible().len(),
        actions.len(),
        "equal bounds admit all"
    );
    // A missing exposure bound is rejected — no action can be screened
    // out by omission.
    let missing = &exposures[1..];
    assert!(catch_unwind(AssertUnwindSafe(|| AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        missing,
        field0_id,
        field1_id,
        0,
    )))
    .is_err());
    // A missing baseline bound is rejected.
    let missing = &bounds[1..];
    assert!(catch_unwind(AssertUnwindSafe(|| AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        missing,
        &exposures,
        field0_id,
        field1_id,
        0,
    )))
    .is_err());
    // A bound naming a non-legal action is rejected.
    let alien = Domino::from_index(
        (0..28)
            .find(|i| !legal.contains(Domino::from_index(*i).expect("tile")))
            .expect("some tile is not legal here"),
    )
    .expect("tile");
    let mut extra = bounds.clone();
    extra[0] = bound(alien);
    assert!(catch_unwind(AssertUnwindSafe(|| AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &extra,
        &exposures,
        field0_id,
        field1_id,
        0,
    )))
    .is_err());
    // A duplicated bound is rejected.
    let mut duplicated = exposures.clone();
    duplicated[1] = exposure(actions[0]);
    assert!(catch_unwind(AssertUnwindSafe(|| AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        &duplicated,
        field0_id,
        field1_id,
        0,
    )))
    .is_err());
}

// ---------------------------------------------------------------------------
// §5.1 / §12.2 — slack arithmetic, exactly, and its screening meaning.
// ---------------------------------------------------------------------------

#[test]
fn slack_arithmetic_is_exact_and_positive_slack_excludes_the_rival() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let run = parity_run(&root, &position);
    let screen = AdmissibleScreen::compute(
        run.legal,
        BaselineTier::ExactFrozenSet,
        &run.baseline0.point_bounds(),
        &run.e4,
        run.field0_id,
        run.field1_id,
        run.root_id,
    );
    let table = screen.slack_table();
    assert_eq!(table.len(), run.actions.len() * (run.actions.len() - 1));
    for entry in &table {
        // S_{a,b} = L_a^(0) − U_b^(0) − R_a^U − R_b^U, recomputed from
        // the rows independently.
        let ra = screen.row(entry.a);
        let rb = screen.row(entry.b);
        let expected = &ra.lower0
            - &rb.upper0
            - ra.exposure.screenable_upper()
            - rb.exposure.screenable_upper();
        assert_eq!(entry.slack, expected);
        assert_eq!(entry.slack, screen.slack(entry.a, entry.b));
        // Positive slack means L_a^(1) > U_b^(1): the rival cannot reach
        // the bar and is excluded.
        if entry.slack > BigRational::zero() {
            assert!(!screen.admitted(entry.b));
        }
    }
    // If the σ0 winner holds positive slack against every rival, the
    // admissible set is exactly that winner (§5.1 ⇒ L2-T3 shape).
    let best0 = run
        .actions
        .iter()
        .map(|a| run.baseline0.value(*a).clone())
        .max()
        .expect("actions");
    let winners0: Vec<Domino> = run
        .actions
        .iter()
        .copied()
        .filter(|a| *run.baseline0.value(*a) == best0)
        .collect();
    if let [a] = winners0[..] {
        let all_positive = run
            .actions
            .iter()
            .filter(|b| **b != a)
            .all(|b| screen.slack(a, *b) > BigRational::zero());
        if all_positive {
            assert_eq!(screen.admissible(), vec![a]);
        }
    }
}

// ---------------------------------------------------------------------------
// L2-A3 — the seven field-swap result kinds are mechanically distinct in
// serialization, and no claim outruns the baseline tier.
// ---------------------------------------------------------------------------

#[test]
fn field_swap_result_kinds_are_mechanically_distinct_in_serialization() {
    for (i, a) in FieldSwapKind::ALL.iter().enumerate() {
        for b in &FieldSwapKind::ALL[i + 1..] {
            assert_ne!(a.tag(), b.tag());
            assert_ne!(a.to_string(), b.to_string());
        }
        assert_eq!(a.to_string(), a.tag());
    }
    // An unresolved baseline never yields a stability claim, whatever the
    // interval geometry says.
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let mut bounds = Vec::new();
    let mut exposures = Vec::new();
    for (k, action) in actions.iter().enumerate() {
        // A geometry that would scream "stable" at an exact tier: one
        // action at 1, the rest at 0, all exposure bounds 0.
        let v = if k == 0 { q(1, 1) } else { q(0, 1) };
        bounds.push(ActionBound {
            action: *action,
            lower: v.clone(),
            upper: v,
        });
        exposures.push(ActionExposureUpper {
            action: *action,
            bound: RootActionExposureUpper::from_rung(ExposureRung::E4, q(0, 1)),
        });
    }
    let (field0_id, field1_id) = (
        FieldModel::new(field0_spec()).field_id(),
        FieldModel::new(field1_spec()).field_id(),
    );
    let unresolved = AdmissibleScreen::compute(
        legal,
        BaselineTier::Unresolved,
        &bounds,
        &exposures,
        field0_id,
        field1_id,
        0,
    );
    assert_eq!(unresolved.admissible().len(), 1);
    assert_eq!(unresolved.kind(), FieldSwapKind::FieldUnresolved);
    // The same geometry at the exact tier claims stability at that tier.
    let exact = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &bounds,
        &exposures,
        field0_id,
        field1_id,
        0,
    );
    assert_eq!(exact.kind(), FieldSwapKind::FieldStableExactFrozenSet);
    let delta = AdmissibleScreen::compute(
        legal,
        BaselineTier::DeltaFrozenSet,
        &bounds,
        &exposures,
        field0_id,
        field1_id,
        0,
    );
    assert_eq!(delta.kind(), FieldSwapKind::FieldStableDeltaFrozenSet);
    let exact_root = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactRoot,
        &bounds,
        &exposures,
        field0_id,
        field1_id,
        0,
    );
    assert_eq!(exact_root.kind(), FieldSwapKind::FieldStableExactRoot);
}

// ---------------------------------------------------------------------------
// O34 / L2-A4 — a sampled lower witness to R_a is never an upper bound:
// the mathematical demonstration, and the type-level lock restated.
// ---------------------------------------------------------------------------

#[test]
fn a_sampled_lower_witness_is_never_an_upper_bound() {
    let r = receipt();
    let mut demonstrated = false;
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        for action in legal_root_actions(&root, &position).iter() {
            let solve = exact_split_reach(&root, &position, action, &field0, &field1);
            if solve.frontier_worlds == 0 {
                continue;
            }
            // R_a > 0 here. A frozen ρ_a whose exact-fiber exposure is
            // below 1 has an unexposed world; a "sample" consisting of
            // that world alone observes d̂ = 0 — a valid LOWER witness and
            // a catastrophically wrong upper bound. The type system is
            // what stands between that observation and the screen.
            let rho = pinned(&position, action);
            let exposure = frozen_policy_exposure(
                &root,
                &position,
                &rho,
                &field0,
                &field1,
                WorldDomain::ExactFiber,
            );
            if exposure.rows.iter().any(|row| row.split.is_none()) {
                assert!(solve.r() > BigRational::zero());
                // The fixed-policy tier's serialization carries no rung
                // and no screenable field; there is no conversion to
                // RootActionExposureUpper anywhere in the API (L2-A4).
                let serialized = exposure.to_string();
                assert!(serialized.starts_with("FrozenPolicyExposure{"));
                assert!(!serialized.contains("rung="));
                demonstrated = true;
            }
        }
    }
    assert!(
        demonstrated,
        "some parity root/action has 0 < R_a with an unexposed world; \
         if this ever fails the roots or the declared pair must change"
    );
    // The screen's input type wraps RootActionExposureUpper and nothing
    // else — this function signature existing is the compile-time lock.
    fn only_screenable(bound: RootActionExposureUpper) -> ActionExposureUpper {
        ActionExposureUpper {
            action: Domino::from_index(0).expect("tile"),
            bound,
        }
    }
    let upper = only_screenable(RootActionExposureUpper::from_rung(
        ExposureRung::E2,
        q(1, 3),
    ));
    assert_eq!(upper.bound.rung(), ExposureRung::E2);
}
