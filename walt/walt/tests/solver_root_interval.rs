//! Gates for the counted-belief Slice A [L2 thread]: root intervals and
//! survivor sets — the pmake empirical-max upper (CBS-A2), the
//! frozen-policy lower witness with the §6 discovery/evaluation lock, the
//! Theorem 2.1 survivor arithmetic, the typed decision ladder, and the
//! risk wiring.
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! Parts I–II (§1–8), adopted by rulings CBS-A1..A9
//! (`walt/CENSUS-RULINGS.md`, "The counted-belief-sandwich adjudication
//! (2026-08-30)"); intake companion
//! `walt/math/counted_belief_sandwich_v0.1_intake.md`.
//!
//! DECLARED TEST EPOCH: one field σ = Level0 { n0 = 2 } (the fixed-field
//! best response is a ONE-field object); frozen pinned level-1
//! continuations at declared schedule [2, 2] as lower-witness policies.
//! Frozen `verify_player` receipt roots: hand 4 trick 6 (fiber 90) and
//! hand 10 trick 6 (fiber 19); the viewer leads at each. Upper stream
//! epoch 0, evaluation stream epoch 1, discovery stream epoch 2 — the
//! realized gate values are deterministic on these frozen streams.

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
use walt::solver::evidence::ScopedDelta;
use walt::solver::exposure::{exact_root_value, sampled_root_optimum};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::root_interval::{
    assert_root_risk_allocation, decide, discover_lower_policy, frozen_policy_lower,
    grid_lower_endpoint, nested_prefix_lowers, pmake_empirical_max_upper, policy_root_action,
    survivors, HeuristicFallback, PolicyProvenance, RootActionInterval, RootActionUpper,
    RootDecision,
};
use walt::solver::upper_cs::{grid_upper_endpoint, POLICY_CLASS_INFO_CONSISTENT};

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

/// The declared one-field σ of this gate epoch.
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

/// One pinned frozen focal candidate for a legal root action, at the
/// declared [2, 2] schedule (the same lower-witness shape the e3 gates
/// freeze).
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

/// Build the full interval cover of one root under the declared epochs.
fn intervals_at(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &FieldModel,
    prefix: u64,
    scope: &str,
) -> Vec<RootActionInterval> {
    let mut out = Vec::new();
    for action in legal_root_actions(root, position).iter() {
        let upper = pmake_empirical_max_upper(
            root,
            position,
            action,
            field,
            0,
            prefix,
            ScopedDelta::new(format!("{scope}/{action}/upper"), q(1, 20)),
        );
        let policy = pinned(position, action);
        let lower = frozen_policy_lower(
            root,
            position,
            &policy,
            field,
            PolicyProvenance::Fixed,
            1,
            prefix,
            ScopedDelta::new(format!("{scope}/{action}/lower"), q(1, 20)),
        );
        assert_eq!(lower.action, action, "the pinned policy plays its pin");
        out.push(RootActionInterval::new(lower, upper));
    }
    out
}

// ---------------------------------------------------------------------------
// The mirror endpoint: definitional identity, monotone running maximum,
// and the adjudicated finite-horizon sweep on the lower side (the
// verifier's fixed-policy lower check, worst overcoverage 11/128 < 1/4).
// ---------------------------------------------------------------------------

#[test]
fn the_mirror_endpoint_is_the_complement_and_the_lower_sweep_matches_the_adjudication() {
    // Definitional identity on a small grid.
    for n in 1..=6u64 {
        for s in 0..=n {
            assert_eq!(
                grid_lower_endpoint(s, n, 8, &q(1, 4)),
                BigRational::one() - grid_upper_endpoint(n - s, n, 8, &q(1, 4))
            );
        }
    }
    // The running maximum is exactly the maximum of the raw endpoints.
    let counts = [0u64, 1, 2, 2, 3, 3, 3, 4];
    let lowers = nested_prefix_lowers(&counts, 8, &q(1, 4));
    for pair in lowers.windows(2) {
        assert!(pair[0] <= pair[1], "the nested witness never decreases");
    }
    for (i, reported) in lowers.iter().enumerate() {
        let raw_max = (0..=i)
            .map(|j| {
                grid_lower_endpoint(counts[j], u64::try_from(j + 1).expect("fits"), 8, &q(1, 4))
            })
            .max()
            .expect("a nonempty prefix");
        assert_eq!(*reported, raw_max);
    }
    // The adjudicated Part-1 lower sweep (verifier re-run, session
    // evidence 2026-08-30): every FIXED policy on four worlds, every
    // length-four stream, δ = 1/4 on the grid G_4 — the running lower
    // witness overshoots the true mean on at most δ of streams, and the
    // worst table hits exactly 11/128.
    let delta = q(1, 4);
    let l: Vec<Vec<BigRational>> = (1..=4u64)
        .map(|t| {
            (0..=t)
                .map(|s| grid_lower_endpoint(s, t, 4, &delta))
                .collect()
        })
        .collect();
    let mut worst = 0u32;
    for d in 0u32..16 {
        let v = q(i64::from(d.count_ones()), 4);
        let mut overcovered = 0u32;
        for stream in 0u32..256 {
            let mut s = 0u64;
            let mut bound = BigRational::zero();
            let mut over = false;
            for t in 1..=4usize {
                let w = (stream >> (2 * (t - 1))) & 3;
                s += u64::from((d >> w) & 1);
                let lt = &l[t - 1][usize::try_from(s).expect("fits")];
                if *lt > bound {
                    bound = lt.clone();
                }
                if bound > v {
                    over = true;
                }
            }
            if over {
                overcovered += 1;
            }
        }
        assert!(q(i64::from(overcovered), 256) <= delta);
        worst = worst.max(overcovered);
    }
    assert_eq!(q(i64::from(worst), 256), q(11, 128));
}

// ---------------------------------------------------------------------------
// §44 gate 1–3 — on every affordable exact-root fixture: L ≤ Q ≤ U for
// every action, every exact optimizer survives, and a singleton survivor
// is the exact root action. The counts re-derive from the independent
// producer; the reported bounds re-derive from the counts.
// ---------------------------------------------------------------------------

#[test]
fn intervals_cover_the_exact_values_and_the_exact_optimizer_survives() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in [(4usize, 6usize, 90u128), (10, 6, 19)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber, "the declared parity fiber");
        let field = FieldModel::new(field_spec());
        let legal = legal_root_actions(&root, &position);
        let prefix = 8u64;
        let intervals = intervals_at(
            &root,
            &position,
            &field,
            prefix,
            &format!("gate-ri-{hand_id}-{trick_no}"),
        );
        // The exact authority per action, from the independent Stage-1
        // producer.
        let mut exact: Vec<(Domino, BigRational)> = Vec::new();
        for action in legal.iter() {
            exact.push((
                action,
                exact_root_value(&root, &position, action, &field).value(),
            ));
        }
        let q_max = exact.iter().map(|(_, v)| v.clone()).max().expect("actions");
        for interval in &intervals {
            let q_a = exact
                .iter()
                .find(|(a, _)| *a == interval.action())
                .map(|(_, v)| v.clone())
                .expect("an exact value per action");
            // §44 gate 1: the realized interval covers the exact value on
            // these frozen streams.
            assert!(
                interval.lower_value() <= q_a,
                "L ≤ Q at {} on h{hand_id}-t{trick_no}: L={} Q={}",
                interval.action(),
                interval.lower_value(),
                q_a
            );
            assert!(
                q_a <= interval.upper_value(),
                "Q ≤ U at {} on h{hand_id}-t{trick_no}: Q={} U={}",
                interval.action(),
                q_a,
                interval.upper_value()
            );
            // The upper's counts ARE the sampled pmake optima, prefix by
            // prefix, from the independent producer.
            for t in 1..=prefix {
                assert_eq!(
                    interval.upper.counts()[usize::try_from(t - 1).expect("fits")],
                    sampled_root_optimum(&root, &position, interval.action(), &field, 0, t)
                );
            }
            // Identity coordinates ride every carrier.
            assert_eq!(interval.upper.grid, fiber);
            assert_eq!(interval.upper.policy_class, POLICY_CLASS_INFO_CONSISTENT);
            assert_eq!(interval.upper.root_id, root_identity(&root, &position));
            assert_eq!(interval.lower.grid, fiber);
        }
        // §44 gates 2–3: every exact optimizer survives; a singleton
        // survivor is the exact root action.
        let decision = decide(&intervals, legal);
        let surviving = decision.surviving();
        for (action, v) in &exact {
            if v == &q_max {
                assert!(
                    surviving.contains(*action),
                    "the exact optimizer {action} survives on h{hand_id}-t{trick_no}"
                );
            }
        }
        if let RootDecision::DeltaRootWinner { action, .. } = &decision {
            let (best, _) = exact
                .iter()
                .find(|(_, v)| v == &q_max)
                .expect("an optimizer");
            assert_eq!(action, best, "a singleton survivor is the exact optimum");
        }
        // Batch/pause invariance: the producers are pure functions of the
        // declared identity — recomputation is bit-identical.
        let again = intervals_at(
            &root,
            &position,
            &field,
            prefix,
            &format!("gate-ri-{hand_id}-{trick_no}"),
        );
        for (a, b) in intervals.iter().zip(again.iter()) {
            assert_eq!(a.upper, b.upper);
            assert_eq!(a.lower, b.lower);
        }
    }
}

// ---------------------------------------------------------------------------
// §6 — the discovery/evaluation lock: a same-stream selected lower
// witness is unconstructible; the separated route constructs and carries
// its provenance.
// ---------------------------------------------------------------------------

#[test]
fn a_same_stream_selected_lower_witness_is_unconstructible() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    let field = FieldModel::new(field_spec());
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
    let candidates: Vec<&FrozenPolicy> = policies.iter().collect();
    // Discovery on epoch 2 selects a strongest candidate.
    let winner = discover_lower_policy(&root, &position, &candidates, &field, 2, 4);
    // Evaluating the discovered policy ON ITS DISCOVERY EPOCH is refused
    // by the carrier: a same-stream selected argmax is not a lower
    // witness.
    let refused = catch_unwind(AssertUnwindSafe(|| {
        frozen_policy_lower(
            &root,
            &position,
            candidates[winner],
            &field,
            PolicyProvenance::Discovered { epoch: 2 },
            2,
            4,
            ScopedDelta::new("gate-ri-lock/same-stream", q(1, 20)),
        )
    }));
    assert!(refused.is_err(), "the §6 lock refuses the same stream");
    // The separated route constructs, and the result carries its
    // provenance on the label.
    let lower = frozen_policy_lower(
        &root,
        &position,
        candidates[winner],
        &field,
        PolicyProvenance::Discovered { epoch: 2 },
        3,
        4,
        ScopedDelta::new("gate-ri-lock/separated", q(1, 20)),
    );
    assert_eq!(lower.provenance, PolicyProvenance::Discovered { epoch: 2 });
    assert!(lower.to_string().contains("provenance=discovered@2"));
    assert!(lower.lower() >= BigRational::zero());
    // The policy's root action is its pin — the witness binds to the
    // action it actually plays.
    assert_eq!(
        policy_root_action(&root, &position, candidates[winner]),
        actions[winner]
    );
}

// ---------------------------------------------------------------------------
// The carriers reject malformed count paths (the shape half of Corollary
// 5.2's one-way rule; the semantic half is the producer's obligation).
// ---------------------------------------------------------------------------

#[test]
fn malformed_count_paths_are_rejected() {
    let field = FieldModel::new(field_spec()).field_id();
    let tile = Domino::from_index(0).expect("tile");
    for bad in [vec![2u64, 1], vec![0, 2], vec![2], vec![]] {
        let refused = catch_unwind(AssertUnwindSafe(|| {
            RootActionUpper::from_prefix_counts(
                tile,
                field,
                0,
                0,
                4,
                POLICY_CLASS_INFO_CONSISTENT,
                ScopedDelta::new("gate-ri-shape/upper", q(1, 20)),
                bad.clone(),
            )
        }));
        assert!(refused.is_err(), "a malformed count path is refused");
    }
    // The lawful shape constructs and its bounds sit in [0, 1].
    let upper = RootActionUpper::from_prefix_counts(
        tile,
        field,
        0,
        0,
        4,
        POLICY_CLASS_INFO_CONSISTENT,
        ScopedDelta::new("gate-ri-shape/ok", q(1, 20)),
        vec![1, 2, 2, 3],
    );
    assert!(upper.upper() >= BigRational::zero() && upper.upper() <= BigRational::one());
}

// ---------------------------------------------------------------------------
// The interval pairing asserts one claim identity and distinct risk
// scopes; the risk arithmetic sums exactly inside the decision budget.
// ---------------------------------------------------------------------------

#[test]
fn interval_pairing_and_risk_allocation_are_enforced() {
    let r = receipt();
    let (root, position) = root_at(&r, 10, 6);
    let field = FieldModel::new(field_spec());
    let intervals = intervals_at(&root, &position, &field, 4, "gate-ri-risk");
    // Two entries per interval, all distinct scopes, exact rational sum
    // inside the declared decision budget.
    let n = i64::try_from(intervals.len()).expect("fits");
    let budget = ScopedDelta::new("gate-ri-risk", q(n, 10));
    let total = assert_root_risk_allocation(&budget, &intervals);
    assert_eq!(total, q(n, 10));
    // A shared scope across the pair is refused at pairing time.
    let action = intervals[0].action();
    let policy = pinned(&position, action);
    let shared = ScopedDelta::new("gate-ri-risk/shared", q(1, 20));
    let lower = frozen_policy_lower(
        &root,
        &position,
        &policy,
        &field,
        PolicyProvenance::Fixed,
        1,
        4,
        shared.clone(),
    );
    let upper = pmake_empirical_max_upper(&root, &position, action, &field, 0, 4, shared);
    let refused = catch_unwind(AssertUnwindSafe(|| RootActionInterval::new(lower, upper)));
    assert!(refused.is_err(), "a shared risk scope is refused");
}

// ---------------------------------------------------------------------------
// Result typing: a starved budget yields UnresolvedRootSet, never a
// forced winner; a partial cover types nothing; the fallback chooses
// within survivors and is labeled a fallback.
// ---------------------------------------------------------------------------

#[test]
fn starved_budgets_stay_unresolved_and_the_fallback_stays_labeled() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let field = FieldModel::new(field_spec());
    let legal = legal_root_actions(&root, &position);
    // One world, tight δ: no exclusion is possible — the honest output is
    // the unresolved full set, never a forced winner.
    let mut intervals = Vec::new();
    for action in legal.iter() {
        let upper = pmake_empirical_max_upper(
            &root,
            &position,
            action,
            &field,
            0,
            1,
            ScopedDelta::new(format!("gate-ri-cap/{action}/upper"), q(1, 100)),
        );
        let policy = pinned(&position, action);
        let lower = frozen_policy_lower(
            &root,
            &position,
            &policy,
            &field,
            PolicyProvenance::Fixed,
            1,
            1,
            ScopedDelta::new(format!("gate-ri-cap/{action}/lower"), q(1, 100)),
        );
        intervals.push(RootActionInterval::new(lower, upper));
    }
    let decision = decide(&intervals, legal);
    assert!(
        matches!(decision, RootDecision::UnresolvedRootSet { .. }),
        "a starved budget yields UnresolvedRootSet, got {decision}"
    );
    // The fallback chooses within the surviving set and says what it is.
    let fallback = HeuristicFallback::lowest_tile(&decision, legal);
    assert!(decision.surviving().contains(fallback.choice));
    assert!(fallback.to_string().contains("fallback"));
    // A partial cover types nothing: dropping one interval is refused.
    let partial = &intervals[1..];
    let refused = catch_unwind(AssertUnwindSafe(|| decide(partial, legal)));
    assert!(refused.is_err(), "a partial cover is refused");
    // The survivor arithmetic itself: the bar is the maximum lower
    // witness and every survivor's upper clears it.
    let s = survivors(&intervals);
    for interval in &intervals {
        assert!(interval.lower_value() <= s.bar);
        if s.survivors.contains(interval.action()) {
            assert!(interval.upper_value() >= s.bar);
        } else {
            assert!(interval.upper_value() < s.bar);
        }
    }
}
