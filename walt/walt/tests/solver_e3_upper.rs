//! Gates for slice 4a [L2 thread]: the δ-valid admissible-upper E3
//! producer — the max-preserving upper confidence sequence over the
//! empirical-optimum split-reach count, its directional variants, the
//! §1.9 typed result and its screen admission, and the §1.8 risk wiring.
//!
//! Mathematical source: Part 1 of the x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`
//! §§1.1–1.10, proof ledger P1–P4), adopted by rulings TRIPLE-A2/A3
//! (`walt/CENSUS-RULINGS.md`, "The deferred-producers adjudication
//! (2026-08-25)"); intake companion
//! `walt/math/response_deferred_producers_triple_v0.1_intake.md`.
//!
//! DECLARED TEST EPOCH PAIR (unchanged from slices 2–3): σ0 = Level0
//! { n0 = 2 }, σ1 = Level1 { n_outer = 2, n0 = 2 }; frozen focal
//! candidates at declared schedule [2, 2]. Frozen `verify_player` receipt
//! roots: hand 4 trick 6 (fiber 90) and hand 10 trick 6 (fiber 19); the
//! viewer leads at each, so every held tile is a legal root action.

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
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::evidence::{lower_threshold_evidence, ScopedDelta};
use walt::solver::exposure::{
    sampled_directional_count, sampled_split_reach, DirectionalObjective, ExposureRung,
    RootActionExposureUpper,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    exact_frozen_action_values, ActionDirectionalUpper, ActionExposureUpper, AdmissibleScreen,
    BaselineTier, DirectionalScreen, FieldSwapKind,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::upper_cs::{
    assert_screen_risk_allocation, directional_screen_upper, e3_directional_upper,
    e3_split_reach_upper, grid_upper_endpoint, nested_prefix_uppers, E3Direction, SplitReachUpper,
    POLICY_CLASS_INFO_CONSISTENT,
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

// ---------------------------------------------------------------------------
// §1.7 — the response's worked specimen, as an exact fixture.
// ---------------------------------------------------------------------------

#[test]
fn the_worked_specimen_is_exact() {
    // The specimen arithmetic re-derived at intake: E⁻_{2,2}(3/4) =
    // E>_{2,2}(1/4) = 1/3 + 1/2 + 3/10 = 17/15 < 4 = 1/δ, so 3/4
    // survives at s = f = 2.
    assert_eq!(lower_threshold_evidence(2, 2, &q(3, 4)), q(17, 15));
    let delta = q(1, 4);
    // U_{1/4,4}(2, 4) = 3/4: the point 1 is rejected because two
    // failures were observed; 3/4 survives.
    assert_eq!(grid_upper_endpoint(2, 4, 4, &delta), q(3, 4));
    // The fused sequence has four successes in four observations, so the
    // point 1 remains possible: the fused-E2 endpoint is 1.
    assert_eq!(grid_upper_endpoint(4, 4, 4, &delta), BigRational::one());
    // Two policies (1,1,0,0)/(0,0,1,1) on four equiprobable worlds, on
    // the stream ω0,ω1,ω2,ω3: the empirical-optimum prefix counts are
    // S*_t = 1,2,2,2 and the fused counts are F_t = 1,2,3,4.
    let e3 = nested_prefix_uppers(&[1, 2, 2, 2], 4, &delta);
    let e2 = nested_prefix_uppers(&[1, 2, 3, 4], 4, &delta);
    assert_eq!(e3.last(), Some(&q(3, 4)));
    assert_eq!(e2.last(), Some(&BigRational::one()));
    // R = 1/2 ≤ E3 = 3/4 < fused E2 = 1 — the specimen stands.
    assert!(q(1, 2) <= q(3, 4));
    assert!(q(3, 4) < BigRational::one());
    // The typed carrier re-derives the same bound from the same counts.
    let field0 = FieldModel::new(field0_spec()).field_id();
    let field1 = FieldModel::new(field1_spec()).field_id();
    let specimen = SplitReachUpper::from_prefix_counts(
        Domino::from_index(0).expect("tile"),
        E3Direction::Symmetric,
        field0,
        field1,
        0,
        0,
        4,
        POLICY_CLASS_INFO_CONSISTENT,
        ScopedDelta::new("specimen", delta),
        vec![1, 2, 2, 2],
    );
    assert_eq!(specimen.upper(), q(3, 4));
    assert_eq!(specimen.method(), "empirical-optimum-upper-cs");
}

// ---------------------------------------------------------------------------
// §1.2 — endpoint monotonicity in s, monotonicity of E⁻ in c (the
// bisection's justification), and agreement with the defining linear scan.
// ---------------------------------------------------------------------------

#[test]
fn the_endpoint_is_monotone_and_bisection_agrees_with_the_linear_scan() {
    let grid: u128 = 8;
    for delta in [q(1, 4), q(1, 10)] {
        let threshold = BigRational::one() / &delta;
        for n in 1..=6u64 {
            let mut previous: Option<BigRational> = None;
            for s in 0..=n {
                let u = grid_upper_endpoint(s, n, grid, &delta);
                // §1.2 monotonicity: U_{δ,N}(s, n) is nondecreasing in s
                // — the step that collapses the policy family to one
                // integer count.
                if let Some(p) = &previous {
                    assert!(p <= &u, "U is nondecreasing in s at n = {n}");
                }
                previous = Some(u.clone());
                // The bisection result IS the definition: the largest
                // grid c whose lower-tail evidence has not crossed 1/δ,
                // by exhaustive scan with the endpoint conventions
                // (c = 0 never rejected; c = 1 possible iff s = n).
                let mut expected = BigRational::zero();
                for k in 1..=grid {
                    let c = BigRational::new(BigInt::from(k), BigInt::from(grid));
                    let survives = if k == grid {
                        s == n
                    } else if s == n {
                        true
                    } else {
                        lower_threshold_evidence(s, n - s, &c) < threshold
                    };
                    if survives {
                        expected = c;
                    }
                }
                assert_eq!(u, expected, "bisection equals the scan at ({s}, {n})");
            }
            // E⁻_{s,f}(c) is nondecreasing in c on the interior grid —
            // the downward-closed survival set the bisection relies on.
            for s in 0..=n {
                let f = n - s;
                let mut previous_e: Option<BigRational> = None;
                for k in 1..grid {
                    let c = BigRational::new(BigInt::from(k), BigInt::from(grid));
                    let e = lower_threshold_evidence(s, f, &c);
                    if let Some(p) = &previous_e {
                        assert!(p <= &e, "E⁻ is nondecreasing in c at ({s}, {f})");
                    }
                    previous_e = Some(e);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// P4 + the adjudicated finite-horizon sweep: every two-policy Boolean
// table on four worlds, every length-four stream — pathwise E3 ≤ fused E2
// on every prefix, and the worst undercoverage is exactly 11/128 < 1/4.
// ---------------------------------------------------------------------------

#[test]
fn pathwise_e3_never_exceeds_fused_e2_and_the_sweep_matches_the_adjudication() {
    let delta = q(1, 4);
    // U_{1/4,4}(s, t) precomputed on the specimen grid.
    let u: Vec<Vec<BigRational>> = (1..=4u64)
        .map(|t| {
            (0..=t)
                .map(|s| grid_upper_endpoint(s, t, 4, &delta))
                .collect()
        })
        .collect();
    let mut worst = 0u32;
    for d0 in 0u32..16 {
        for d1 in 0u32..16 {
            let mean = |d: u32| q(i64::from(d.count_ones()), 4);
            let r_true = mean(d0).max(mean(d1));
            let mut undercovered = 0u32;
            for stream in 0u32..256 {
                let mut s0 = 0u64;
                let mut s1 = 0u64;
                let mut fused = 0u64;
                let mut e3_bound = BigRational::one();
                let mut e2_bound = BigRational::one();
                let mut under = false;
                for t in 1..=4usize {
                    let w = (stream >> (2 * (t - 1))) & 3;
                    s0 += u64::from((d0 >> w) & 1);
                    s1 += u64::from((d1 >> w) & 1);
                    fused += u64::from(((d0 | d1) >> w) & 1);
                    let s_star = s0.max(s1);
                    let ut = &u[t - 1][usize::try_from(s_star).expect("fits")];
                    if *ut < e3_bound {
                        e3_bound = ut.clone();
                    }
                    let ft = &u[t - 1][usize::try_from(fused).expect("fits")];
                    if *ft < e2_bound {
                        e2_bound = ft.clone();
                    }
                    // P4: the nested E3 bound is pathwise no larger than
                    // the nested fused-E2 bound, prefix by prefix.
                    assert!(e3_bound <= e2_bound);
                    if r_true > e3_bound {
                        under = true;
                    }
                }
                if under {
                    undercovered += 1;
                }
            }
            // Finite-horizon undercoverage at most δ on every table.
            assert!(q(i64::from(undercovered), 256) <= delta);
            worst = worst.max(undercovered);
        }
    }
    // The adjudicated worst finite-horizon undercoverage (13/13-PASS
    // verifier re-run, session evidence): 11/128 < 1/4.
    assert_eq!(q(i64::from(worst), 256), q(11, 128));
}

// ---------------------------------------------------------------------------
// §1.4 — the reported bound is nonincreasing in the prefix.
// ---------------------------------------------------------------------------

#[test]
fn the_reported_bound_is_nonincreasing_in_the_prefix() {
    let delta = q(1, 4);
    for counts in [
        vec![1, 2, 2, 2],
        vec![0, 0, 1, 1],
        vec![1, 2, 3, 4],
        vec![0, 1, 2, 2, 3, 3, 3, 4],
    ] {
        let uppers = nested_prefix_uppers(&counts, 8, &delta);
        assert_eq!(uppers.len(), counts.len());
        for pair in uppers.windows(2) {
            assert!(pair[1] <= pair[0], "the nested bound never increases");
        }
        // The running minimum is exactly the minimum of the raw
        // endpoints over every prefix so far.
        for (i, reported) in uppers.iter().enumerate() {
            let raw_min = (0..=i)
                .map(|j| {
                    grid_upper_endpoint(counts[j], u64::try_from(j + 1).expect("fits"), 8, &delta)
                })
                .min()
                .expect("a nonempty prefix");
            assert_eq!(*reported, raw_min);
        }
    }
}

// ---------------------------------------------------------------------------
// §1.9 / §7.4 — the admissible-upper type enters the screen at rung E3
// with its risk entries summing inside the declared budget; the ESTIMATE
// sibling still cannot; a directional bound refuses the symmetric route.
// ---------------------------------------------------------------------------

#[test]
fn the_admissible_upper_enters_the_screen_and_the_estimate_sibling_cannot() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    assert_eq!(root.count(), 90, "the declared parity fiber");
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-e3-s0");
    // §1.8: one declared screen budget, one risk entry per screen input,
    // summed exactly inside the budget — the existing ScopedDelta
    // discipline, no new ledger.
    let budget = ScopedDelta::new("gate-e3-screen", q(1, 10));
    let mut uppers = Vec::new();
    let mut entries = Vec::new();
    for action in &actions {
        let delta = ScopedDelta::new(format!("gate-e3-screen/{action}/sym"), q(1, 100));
        let e3 = e3_split_reach_upper(&root, &position, *action, &field0, &field1, 0, 6, delta);
        // The grid is the exact fiber size — grid validity (§1.2).
        assert_eq!(e3.grid, root.count());
        assert_eq!(e3.direction, E3Direction::Symmetric);
        assert_eq!(e3.policy_class, POLICY_CLASS_INFO_CONSISTENT);
        // The screen admission: a rung-E3 RootActionExposureUpper whose
        // screenable value IS the reported bound.
        let bound = e3.screen_upper();
        assert_eq!(bound.rung(), ExposureRung::E3);
        assert_eq!(*bound.screenable_upper(), e3.upper());
        entries.push(e3.delta().clone());
        uppers.push(ActionExposureUpper {
            action: *action,
            bound,
        });
    }
    let entry_refs: Vec<&ScopedDelta> = entries.iter().collect();
    let total = assert_screen_risk_allocation(&budget, &entry_refs);
    assert!(total <= q(1, 10));
    let screen = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &baseline0.point_bounds(),
        &uppers,
        field0.field_id(),
        field1.field_id(),
        root_identity(&root, &position),
    );
    // The screen is well-typed over E3 inputs: the standard kind
    // vocabulary at the standard tier discipline, and the display names
    // the rung on every row.
    assert!(matches!(
        screen.kind(),
        FieldSwapKind::FieldStableExactFrozenSet | FieldSwapKind::FieldSensitive
    ));
    assert!(screen.to_string().contains("rung=E3"));
    // A directional bound is not an upper bound on R_a and refuses the
    // symmetric screen route.
    let plus = e3_directional_upper(
        &root,
        &position,
        actions[0],
        E3Direction::Plus,
        &field0,
        &field1,
        0,
        4,
        ScopedDelta::new("gate-e3-refuse/plus", q(1, 100)),
    );
    assert!(catch_unwind(AssertUnwindSafe(|| plus.screen_upper())).is_err());
    // The compile-time half of the lock, restated: the screen input wraps
    // RootActionExposureUpper and nothing else, SplitReachUpper::screen_upper
    // produces one, and SplitReachSampled (the ESTIMATE sibling, which
    // stays) has no such method — no expression of that type can appear
    // below.
    fn only_screenable(bound: RootActionExposureUpper) -> ActionExposureUpper {
        ActionExposureUpper {
            action: Domino::from_index(0).expect("tile"),
            bound,
        }
    }
    let _ = only_screenable;
}

// ---------------------------------------------------------------------------
// §1.5 / TRIPLE-A3 — directional E3: separate solves, separate risk
// entries, counts re-derived from the sampled directional walk, bounded by
// the symmetric count, and feeding the directional screen.
// ---------------------------------------------------------------------------

#[test]
fn directional_e3_bounds_are_separate_solves_with_separate_risk_entries() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field0, "gate-e3d-s0");
    let prefix = 6u64;
    let mut directionals = Vec::new();
    for action in &actions {
        let sym = e3_split_reach_upper(
            &root,
            &position,
            *action,
            &field0,
            &field1,
            0,
            prefix,
            ScopedDelta::new(format!("gate-e3d/{action}/sym"), q(1, 60)),
        );
        let plus = e3_directional_upper(
            &root,
            &position,
            *action,
            E3Direction::Plus,
            &field0,
            &field1,
            0,
            prefix,
            ScopedDelta::new(format!("gate-e3d/{action}/plus"), q(1, 60)),
        );
        let minus = e3_directional_upper(
            &root,
            &position,
            *action,
            E3Direction::Minus,
            &field0,
            &field1,
            0,
            prefix,
            ScopedDelta::new(format!("gate-e3d/{action}/minus"), q(1, 60)),
        );
        assert_eq!(plus.method(), "fused-directional-optimum-upper-cs");
        // The counts re-derive from the sampled directional walk, prefix
        // by prefix, and a directional event needs a split: the
        // directional count never exceeds the symmetric count.
        for t in 1..=prefix {
            let i = usize::try_from(t - 1).expect("fits");
            assert_eq!(
                plus.counts()[i],
                sampled_directional_count(
                    &root,
                    &position,
                    *action,
                    DirectionalObjective::Plus,
                    &field0,
                    &field1,
                    0,
                    t,
                )
            );
            assert!(plus.counts()[i] <= sym.counts()[i]);
            assert!(minus.counts()[i] <= sym.counts()[i]);
        }
        // Hence — endpoint monotonicity through the prefix minimum — the
        // directional uppers never exceed the symmetric E3 upper.
        assert!(plus.upper() <= sym.upper());
        assert!(minus.upper() <= sym.upper());
        // The typed directional pair: separate solves, separate risk
        // entries, one claim identity.
        let pair = directional_screen_upper(&plus, &minus);
        assert_eq!(*pair.screenable_plus(), plus.upper());
        assert_eq!(*pair.screenable_minus(), minus.upper());
        // A shared risk scope across the pair is refused (TRIPLE-A3:
        // separate ledger entries).
        let minus_same_scope = SplitReachUpper::from_prefix_counts(
            *action,
            E3Direction::Minus,
            field0.field_id(),
            field1.field_id(),
            plus.root_id,
            0,
            plus.grid,
            POLICY_CLASS_INFO_CONSISTENT,
            plus.delta().clone(),
            minus.counts().to_vec(),
        );
        assert!(catch_unwind(AssertUnwindSafe(|| directional_screen_upper(
            &plus,
            &minus_same_scope
        )))
        .is_err());
        directionals.push(ActionDirectionalUpper {
            action: *action,
            bound: pair,
        });
    }
    // The pairs feed the directional screen at the declared tier.
    let screen = DirectionalScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &baseline0.point_bounds(),
        &directionals,
        field0.field_id(),
        field1.field_id(),
        root_identity(&root, &position),
    );
    assert!(matches!(
        screen.kind(),
        FieldSwapKind::FieldStableExactFrozenSet | FieldSwapKind::FieldSensitive
    ));
}

// ---------------------------------------------------------------------------
// §1.8 — risks across distinct screen inputs sum against the screen
// budget, exactly.
// ---------------------------------------------------------------------------

#[test]
fn screen_risk_entries_sum_within_the_declared_budget() {
    let budget = ScopedDelta::new("screen", q(1, 20));
    let a = ScopedDelta::new("screen/2-1/sym", q(1, 60));
    let b = ScopedDelta::new("screen/3-1/plus", q(1, 60));
    let c = ScopedDelta::new("screen/3-1/minus", q(1, 60));
    // 3 · (1/60) = 1/20: the allocation exactly fills the budget.
    assert_eq!(
        assert_screen_risk_allocation(&budget, &[&a, &b, &c]),
        q(1, 20)
    );
    // One more entry overruns the budget and is refused.
    let d = ScopedDelta::new("screen/5-5/sym", q(1, 100));
    assert!(catch_unwind(AssertUnwindSafe(|| {
        assert_screen_risk_allocation(&budget, &[&a, &b, &c, &d])
    }))
    .is_err());
    // Two entries sharing one scope are refused: distinct screen inputs
    // carry distinct risk entries.
    let duplicate = ScopedDelta::new("screen/2-1/sym", q(1, 100));
    assert!(catch_unwind(AssertUnwindSafe(|| {
        assert_screen_risk_allocation(&budget, &[&a, &duplicate])
    }))
    .is_err());
}

// ---------------------------------------------------------------------------
// Integration on the frozen receipt roots: the producer's counts are the
// sampled split-reach optima prefix by prefix, the reported bound
// re-derives as the §1.4 running minimum, and the label carries the full
// §1.9 tuple.
// ---------------------------------------------------------------------------

#[test]
fn the_producer_re_derives_from_its_prefix_counts_on_frozen_roots() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in [(4usize, 6usize, 90u128), (10, 6, 19)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber, "the declared parity fiber");
        let field0 = FieldModel::new(field0_spec());
        let field1 = FieldModel::new(field1_spec());
        let action = legal_root_actions(&root, &position)
            .iter()
            .next()
            .expect("a legal action");
        let prefix = 8u64;
        let delta = q(1, 20);
        let e3 = e3_split_reach_upper(
            &root,
            &position,
            action,
            &field0,
            &field1,
            0,
            prefix,
            ScopedDelta::new(format!("gate-e3i-{hand_id}-{trick_no}"), delta.clone()),
        );
        assert_eq!(e3.prefix(), prefix);
        assert_eq!(e3.grid, fiber);
        assert_eq!(e3.epoch, 0);
        assert_eq!(e3.root_id, root_identity(&root, &position));
        // The counts ARE the sampled split-reach optima (S*_t), prefix by
        // prefix, from the independent producer.
        for t in 1..=prefix {
            let sampled = sampled_split_reach(&root, &position, action, &field0, &field1, 0, t);
            assert_eq!(
                e3.counts()[usize::try_from(t - 1).expect("fits")],
                sampled.frontier_worlds
            );
        }
        // The reported bound is the §1.4 running minimum of the raw grid
        // endpoints, re-derived here from scratch.
        let uppers = e3.prefix_uppers();
        for (i, reported) in uppers.iter().enumerate() {
            let raw_min = (0..=i)
                .map(|j| {
                    grid_upper_endpoint(
                        e3.counts()[j],
                        u64::try_from(j + 1).expect("fits"),
                        fiber,
                        &delta,
                    )
                })
                .min()
                .expect("a nonempty prefix");
            assert_eq!(reported, &raw_min);
        }
        assert_eq!(e3.upper(), uppers.last().expect("nonempty").clone());
        assert!(e3.upper() >= BigRational::zero() && e3.upper() <= BigRational::one());
        // The §1.9 label: rung, direction, δ with scope, epoch, prefix,
        // grid, policy class, method — all on the one serialization.
        let shown = e3.to_string();
        assert!(shown.starts_with("SplitReachUpper{rung=E3;direction=symmetric;"));
        let grid_label = format!("grid={fiber}");
        for needle in [
            "epoch=0",
            "prefix=8",
            grid_label.as_str(),
            "policy_class=info-consistent-continuations-v1",
            "method=empirical-optimum-upper-cs",
            "delta[",
        ] {
            assert!(shown.contains(needle), "the label carries {needle}");
        }
    }
}
