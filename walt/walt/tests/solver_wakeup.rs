//! Gate for `solver::wakeup` — the level-2 detection layer (§22 step 9;
//! LEVEL2-PROBE.md as amended by CE-A6, role fixed by L2-A5).
//!
//! Worked-specimen assertions: the exact paired route re-derives against
//! the independent step-8 calibrate producers on a real fiber; the §14.4
//! response-without-value fixture is mechanically demonstrated; every
//! engine's crossing semantics are driven on synthetic streams; the
//! sampled route is deterministic on a fixed epoch and its engine
//! evidence re-derives from the carried audit rows; refusal paths are
//! typed. The exact-zero/practical-zero and response-vs-value locks are
//! compile-time (the module's compile_fail doctests) — this file gates
//! the runtime halves.

use std::panic::{catch_unwind, AssertUnwindSafe};

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::rules::{ContextSet, Domino, DominoSet, Pip, Seat};
use walt::solver::adaptive::{root_identity, SlicePolicy};
use walt::solver::calibrate::{
    exact_set_outcomes, pair_coordinates, reconstruct_flip, shadow_tuple, FlipRoot, FLIP_FIXTURES,
};
use walt::solver::evidence::{affine_factor, MeanNull, ScopedDelta};
use walt::solver::exposure::{
    frozen_policy_exposure, FirstSplit, FrozenPolicyExposure, WorldDomain, WorldRow,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, FrozenPolicy, InnerSchedule, Level0Field, TieRule};
use walt::solver::wakeup::{
    exact_paired_detection, refuse_exact_if_oversized, refuse_sampled_if_underfunded,
    sampled_paired_detection, DecisionWakeUp, DetectionRiskPlan, Direction, DirectionProbe,
    ExactPairSelection, InfoVerdict, PairDecisionProbe, PairWinner, PracticalZeroProbe,
    ResponseProbe, ResponseWakeUp, SampledDetectionSpec, SampledPairedDetection, ValueWakeUp,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn tile(hi: u8, lo: u8) -> Domino {
    Domino::new(Pip::new(hi).expect("pip"), Pip::new(lo).expect("pip"))
}

fn field0() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1 (gate)".to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

fn field1() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1 (gate)".to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

/// The gate's flip specimen: receipt h7 d5, fiber 28 — the smallest of
/// the step-8 fixtures. Candidates at a fast [2, 2] gate schedule.
fn h7() -> (FlipRoot, Vec<FrozenPolicy>) {
    let flip = reconstruct_flip(&FLIP_FIXTURES[1]);
    let candidates: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| {
            let mut tuple = shadow_tuple(&flip.position, *t);
            tuple.inner_schedule = InnerSchedule::Declared(vec![2, 2]);
            FrozenPolicy::new(tuple)
        })
        .collect();
    (flip, candidates)
}

fn mixture() -> Vec<(BigRational, BigRational)> {
    vec![
        (q(1, 4), q(1, 8)),
        (q(1, 4), q(1, 4)),
        (q(1, 4), q(1, 2)),
        (q(1, 4), q(3, 4)),
    ]
}

// ---------------------------------------------------------------------------
// The exact route re-derives against the independent producers on h7.
// ---------------------------------------------------------------------------

#[test]
fn exact_paired_detection_rederives_on_h7() {
    let (flip, candidates) = h7();
    let f0 = field0();
    let f1 = field1();
    let exposures: Vec<FrozenPolicyExposure> = candidates[..2]
        .iter()
        .map(|rho| {
            frozen_policy_exposure(
                &flip.root,
                &flip.position,
                rho,
                &f0,
                &f1,
                WorldDomain::ExactFiber,
            )
        })
        .collect();
    let eps = q(1, 20);
    let detection = exact_paired_detection(
        &exposures[0],
        &exposures[1],
        flip.legal_tiles[0],
        flip.legal_tiles[1],
        &eps,
    );
    // The σ0 leg equals the independent step-8 producer: the calibrate
    // enumeration under the ONE level-0 authority (`Level0Field::new(2)`,
    // exactly step 8's evaluation field) — this also gates that the
    // materialized σ0 FieldModel is that authority, not a copy.
    let level0 = Level0Field::new(2);
    let refs: Vec<&dyn SlicePolicy> = candidates[..2]
        .iter()
        .map(|p| p as &dyn SlicePolicy)
        .collect();
    let outcomes = exact_set_outcomes(&flip.root, &flip.position, &refs, &level0);
    let coords0_independent = pair_coordinates(&outcomes[0], &outcomes[1]);
    assert_eq!(
        detection.coords0, coords0_independent,
        "the paired walk's σ0 coordinates equal the calibrate producer's"
    );
    // The record's wake-ups are functions of its coordinates.
    let gap_change = &detection.coords1.g - &detection.coords0.g;
    match &detection.value {
        ValueWakeUp::Exact {
            gap_change: g,
            wake,
        } => {
            assert_eq!(*g, gap_change);
            assert_eq!(*wake, !gap_change.is_zero());
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    match &detection.response {
        ResponseWakeUp::Exact { dq, positive, .. } => {
            assert_eq!(*dq, &detection.coords1.q - &detection.coords0.q);
            assert_eq!(*positive, *dq > BigRational::zero());
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    match &detection.decision {
        DecisionWakeUp::Exact {
            winner0,
            winner1,
            changed,
        } => {
            let expect = |c: &walt::solver::calibrate::PairCoordinates| {
                use core::cmp::Ordering;
                match c.a.cmp(&c.b) {
                    Ordering::Greater => ExactPairSelection::A,
                    Ordering::Less => ExactPairSelection::B,
                    Ordering::Equal => ExactPairSelection::ExactTie,
                }
            };
            assert_eq!(*winner0, expect(&detection.coords0));
            assert_eq!(*winner1, expect(&detection.coords1));
            assert_eq!(*changed, winner0 != winner1);
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    // The fiber and identities travel on the record.
    assert_eq!(u128::from(detection.fiber), flip.root.count());
    assert_eq!(detection.field0, f0.field_id());
    assert_eq!(detection.field1, f1.field_id());
    assert_eq!(detection.root_id, root_identity(&flip.root, &flip.position));
    // The Z histogram is a census of the fiber.
    let z_total: u64 = detection.z_counts.iter().sum();
    assert_eq!(z_total, detection.fiber);
}

// ---------------------------------------------------------------------------
// §14.4 — response wake-up without value wake-up, mechanically.
// ---------------------------------------------------------------------------

/// A synthetic complete-fiber exposure over four worlds (the type's
/// fields are public; the DOMAIN discipline is what the producer
/// asserts, and this fixture honors it: changed outcomes only on split
/// worlds, tallies recomputable from rows).
fn synthetic_exposure(
    policy: &FrozenPolicy,
    fields: (&FieldModel, &FieldModel),
    root_id: u64,
    u: &[(bool, bool)],
) -> FrozenPolicyExposure {
    let split_for = |w: usize, u0: bool, u1: bool| -> Option<FirstSplit> {
        (u0 != u1).then(|| FirstSplit {
            seat: Seat::S0,
            trick: 5,
            ply: w % 4,
            tile0: tile(1, 0),
            tile1: tile(2, 0),
            hand: DominoSet::EMPTY,
            history: vec![],
        })
    };
    let rows: Vec<WorldRow> = u
        .iter()
        .enumerate()
        .map(|(w, (u0, u1))| WorldRow {
            index: w as u64,
            world: [w as u32, 0, 0, 0],
            u0: *u0,
            u1: *u1,
            split: split_for(w, *u0, *u1),
        })
        .collect();
    let exposed = rows.iter().filter(|r| r.split.is_some()).count() as u64;
    let plus = rows.iter().filter(|r| r.u1 && !r.u0).count() as u64;
    let minus = rows.iter().filter(|r| !r.u1 && r.u0).count() as u64;
    FrozenPolicyExposure {
        policy: policy.policy_id(),
        field0: fields.0.field_id(),
        field1: fields.1.field_id(),
        root_id,
        domain: WorldDomain::ExactFiber,
        worlds: rows.len() as u64,
        exposed,
        corrections_plus: plus,
        corrections_minus: minus,
        rows,
    }
}

#[test]
fn response_wake_without_value_wake_is_the_balanced_case() {
    let (flip, candidates) = h7();
    let f0 = field0();
    let f1 = field1();
    let root_id = 0x517e_9000;
    // Candidate a: identical under both fields on every world. Candidate
    // b: the σ1 field flips one world each way. Then q₀ = 0 exactly,
    // q₁ = 1/2 with τ₁ = 0, g₁ = 0 — §14.4's balanced wake.
    let exposure_a = synthetic_exposure(
        &candidates[0],
        (&f0, &f1),
        root_id,
        &[(true, true), (false, false), (true, true), (false, false)],
    );
    let exposure_b = synthetic_exposure(
        &candidates[1],
        (&f0, &f1),
        root_id,
        &[(true, false), (false, true), (true, true), (false, false)],
    );
    let eps = q(1, 20);
    let detection = exact_paired_detection(
        &exposure_a,
        &exposure_b,
        flip.legal_tiles[0],
        flip.legal_tiles[1],
        &eps,
    );
    // Exact zero under σ0 — pronounceable exactly BECAUSE this is the
    // enumeration route (§14.7).
    assert!(detection.coords0.q.is_zero());
    assert!(detection.coords0.g.is_zero());
    assert_eq!(detection.coords1.q, q(1, 2));
    assert_eq!(detection.coords1.tau, Some(q(0, 1)));
    // Response wakes...
    match &detection.response {
        ResponseWakeUp::Exact {
            positive,
            exceeds_eps,
            dq,
            ..
        } => {
            assert!(*positive && *exceeds_eps);
            assert_eq!(*dq, q(1, 2));
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    // ...while value does NOT: the disagreements balance exactly.
    match &detection.value {
        ValueWakeUp::Exact { gap_change, wake } => {
            assert!(gap_change.is_zero());
            assert!(!wake);
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    // And the decision stays an exact tie under both fields.
    match &detection.decision {
        DecisionWakeUp::Exact {
            winner0,
            winner1,
            changed,
        } => {
            assert_eq!(*winner0, ExactPairSelection::ExactTie);
            assert_eq!(*winner1, ExactPairSelection::ExactTie);
            assert!(!changed);
        }
        other => panic!("the exact route produced {}", other.tag()),
    }
    // Both rates exactly zero: regime 4 on both sides of the swap.
    assert_eq!(
        detection.information.verdict,
        InfoVerdict::BothZeroRateExact
    );
    // The §14.6 Z histogram holds one +1 and one −1 world.
    assert_eq!(detection.z_counts, [0, 1, 2, 1, 0]);
}

// ---------------------------------------------------------------------------
// Engine crossing semantics on synthetic streams.
// ---------------------------------------------------------------------------

#[test]
fn practical_zero_witness_appears_only_by_crossing() {
    let mixture = mixture();
    let mut probe = PracticalZeroProbe::new(
        q(1, 20),
        ScopedDelta::new("gate:practical-zero", q(1, 100)),
        &mixture,
    )
    .expect("a lawful mixture");
    assert!(probe.witness().is_none());
    let mut crossed_at = None;
    for i in 0..400u64 {
        probe.observe(false);
        if crossed_at.is_none() && probe.witness().is_some() {
            crossed_at = Some(i + 1);
        }
    }
    let witness = probe.witness().expect("a nonpivotal stream crosses");
    assert_eq!(Some(witness.settled_at()), crossed_at);
    assert!(*witness.evidence() >= q(100, 1));
    assert_eq!(*witness.eps_q(), q(1, 20));
    // Further folding never un-mints the witness.
    probe.observe(true);
    assert!(probe.witness().is_some());
}

#[test]
fn response_probe_establishes_on_a_wide_response_gap() {
    let mixture = mixture();
    let mut probe = ResponseProbe::new(
        q(1, 20),
        ScopedDelta::new("gate:response", q(1, 100)),
        &mixture,
    )
    .expect("a lawful mixture");
    for _ in 0..64 {
        probe.observe(true, false);
    }
    let established = probe.established().expect("W = 1 crosses fast");
    assert!(established.settled_at() <= 64);
    assert!(*established.evidence() >= q(100, 1));
    // A balanced stream establishes nothing.
    let mut balanced = ResponseProbe::new(
        q(1, 20),
        ScopedDelta::new("gate:response-balanced", q(1, 100)),
        &mixture,
    )
    .expect("a lawful mixture");
    for _ in 0..64 {
        balanced.observe(true, true);
        balanced.observe(false, false);
    }
    assert!(balanced.established().is_none());
}

#[test]
fn direction_probe_settles_the_true_direction() {
    let mixture = mixture();
    let mut up = DirectionProbe::new(ScopedDelta::new("gate:value-up", q(1, 100)), &mixture)
        .expect("a lawful mixture");
    for _ in 0..64 {
        up.observe(2);
    }
    assert_eq!(
        up.settled().expect("Z = +2 crosses").direction(),
        Direction::Positive
    );
    let mut down = DirectionProbe::new(ScopedDelta::new("gate:value-down", q(1, 100)), &mixture)
        .expect("a lawful mixture");
    for _ in 0..64 {
        down.observe(-2);
    }
    assert_eq!(
        down.settled().expect("Z = −2 crosses").direction(),
        Direction::Negative
    );
    // A zero stream settles nothing, in either engine.
    let mut zero = DirectionProbe::new(ScopedDelta::new("gate:value-zero", q(1, 100)), &mixture)
        .expect("a lawful mixture");
    for _ in 0..256 {
        zero.observe(0);
    }
    assert!(zero.settled().is_none());
    // Out-of-range Z is a contract violation.
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let mut p = DirectionProbe::new(ScopedDelta::new("gate:value-bad", q(1, 100)), &mixture)
            .expect("a lawful mixture");
        p.observe(3);
    }))
    .is_err());
}

#[test]
fn pair_decision_probe_settles_at_the_exact_minimum_pivots() {
    // δ = 1/200 at m = 2 gives the edge threshold 400 and
    // h±_min(0,0) = 12 (the step-8 initial evidence state) — twelve
    // unanimous pivots settle, eleven do not.
    let mut probe = PairDecisionProbe::new(ScopedDelta::new("gate:decision", q(1, 200)));
    for i in 0..11 {
        probe.observe(1);
        assert!(probe.settled().is_none(), "not settled at {} pivots", i + 1);
    }
    probe.observe(1);
    let settled = probe.settled().expect("twelve unanimous pivots settle");
    assert_eq!(settled.winner(), PairWinner::A);
    assert_eq!(settled.settled_at(), 12);
    assert_eq!(probe.counts(), (12, 0));
    let mut other = PairDecisionProbe::new(ScopedDelta::new("gate:decision-b", q(1, 200)));
    for _ in 0..12 {
        other.observe(-1);
    }
    assert_eq!(
        other.settled().expect("the mirror settles").winner(),
        PairWinner::B
    );
    // Nonpivotal worlds never settle a decision.
    let mut idle = PairDecisionProbe::new(ScopedDelta::new("gate:decision-idle", q(1, 200)));
    for _ in 0..1000 {
        idle.observe(0);
    }
    assert!(idle.settled().is_none());
}

// ---------------------------------------------------------------------------
// The sampled route: determinism and evidence re-derivation on h7.
// ---------------------------------------------------------------------------

fn sampled_on_h7(epoch: u64, world_cap: u64) -> (SampledPairedDetection, FlipRoot) {
    let (flip, candidates) = h7();
    let f0 = field0();
    let f1 = field1();
    let plan = DetectionRiskPlan {
        eps_q: q(1, 20),
        delta_decision: ScopedDelta::new("gate:h7:pair-decision", q(1, 200)),
        delta_value: ScopedDelta::new("gate:h7:value-direction", q(1, 100)),
        delta_response: ScopedDelta::new("gate:h7:response", q(1, 100)),
        delta_practical_zero: ScopedDelta::new("gate:h7:practical-zero-q0", q(1, 100)),
        mixture: mixture(),
    };
    let detection = sampled_paired_detection(&SampledDetectionSpec {
        root: &flip.root,
        position: &flip.position,
        tile_a: flip.legal_tiles[0],
        tile_b: flip.legal_tiles[1],
        policy_a: &candidates[0],
        policy_b: &candidates[1],
        field0: &f0,
        field1: &f1,
        epoch,
        world_cap,
        plan: &plan,
    });
    (detection, flip)
}

#[test]
fn sampled_paired_detection_is_deterministic_and_rederives() {
    let (first, _flip) = sampled_on_h7(0x009e_7a11, 16);
    let (second, _flip) = sampled_on_h7(0x009e_7a11, 16);
    // Deterministic: same epoch, same records.
    assert_eq!(first.consumed, second.consumed);
    assert_eq!(first.coords0, second.coords0);
    assert_eq!(first.coords1, second.coords1);
    assert_eq!(first.z_counts, second.z_counts);
    assert_eq!(first.exposure_a, second.exposure_a);
    assert_eq!(first.exposure_b, second.exposure_b);
    assert_eq!(first.response.tag(), second.response.tag());
    assert_eq!(first.value.tag(), second.value.tag());
    assert_eq!(first.decision.tag(), second.decision.tag());
    // Lawful counts: pivots fit the consumed prefix; the exposures carry
    // exactly the consumed rows; splits aggregate from the rows.
    assert!(first.consumed >= 1 && first.consumed <= 16);
    assert!(first.coords0.a + first.coords0.b <= first.consumed);
    assert!(first.coords1.a + first.coords1.b <= first.consumed);
    assert_eq!(first.exposure_a.worlds, first.consumed);
    assert_eq!(first.splits_a.exposed, first.exposure_a.exposed);
    assert_eq!(first.splits_b.exposed, first.exposure_b.exposed);
    let z_total: u64 = first.z_counts.iter().sum();
    assert_eq!(z_total, first.consumed);
    // The value engine's evidence re-derives from the carried audit rows
    // (the engines are deterministic folds of the row sequence).
    if let ValueWakeUp::SampledOpen {
        evidence_up,
        evidence_down,
        ..
    } = &first.value
    {
        let mut products: Vec<(BigRational, BigRational, BigRational)> = mixture()
            .into_iter()
            .map(|(w, l)| (w, l, q(1, 1)))
            .collect();
        let mut products_down = products.clone();
        for (ra, rb) in first.exposure_a.rows.iter().zip(&first.exposure_b.rows) {
            let y0 = i8::from(ra.u0) - i8::from(rb.u0);
            let y1 = i8::from(ra.u1) - i8::from(rb.u1);
            let x = BigRational::new(BigInt::from(y1 - y0), BigInt::from(2));
            for (_, l, p) in &mut products {
                *p *= affine_factor(MeanNull::AtMost, l, &q(0, 1), &x);
            }
            for (_, l, p) in &mut products_down {
                *p *= affine_factor(MeanNull::AtLeast, l, &q(0, 1), &x);
            }
        }
        let fold = |ps: &[(BigRational, BigRational, BigRational)]| {
            ps.iter()
                .fold(BigRational::zero(), |acc, (w, _, p)| acc + w * p)
        };
        assert_eq!(*evidence_up, fold(&products));
        assert_eq!(*evidence_down, fold(&products_down));
    }
}

// ---------------------------------------------------------------------------
// Typed refusals.
// ---------------------------------------------------------------------------

#[test]
fn refusals_are_typed_records_not_degraded_numbers() {
    let f0 = field0();
    let f1 = field1();
    let refusal = refuse_sampled_if_underfunded(7, f0.field_id(), f1.field_id(), 8, 64)
        .expect_err("an underfunded cap refuses");
    assert_eq!(refusal.route, "sampled");
    assert!(refusal.reason.contains("below the declared minimum"));
    assert!(refuse_sampled_if_underfunded(7, f0.field_id(), f1.field_id(), 64, 64).is_ok());
    let refusal = refuse_exact_if_oversized(7, f0.field_id(), f1.field_id(), 10_000, 4096)
        .expect_err("an oversized fiber refuses the exact route");
    assert_eq!(refusal.route, "exact");
    assert!(refusal.reason.contains("exceeds the declared enumeration"));
    assert!(refuse_exact_if_oversized(7, f0.field_id(), f1.field_id(), 4096, 4096).is_ok());
}

// ---------------------------------------------------------------------------
// The literal count-timing position stays blocked (L2-A6): the marker
// test lives in solver_calibrate.rs (`#[ignore]`d,
// `v5_literal_count_timing_position_reconstructs`); the six-member shape
// family is the honest stand-in this probe samples. A ContextSet import
// keeps the fixture vocabulary in scope for future literal wiring.
// ---------------------------------------------------------------------------

#[test]
fn count_timing_family_is_the_declared_standin() {
    // The family member reconstructs with the specimen's shape: S3 to act
    // at trick 1 ply 2 with legal exactly {6-2, 6-4}.
    let spec = walt::solver::calibrate::CountTimingSpec::new(0, 8);
    let flip = spec.root();
    assert_eq!(flip.legal_tiles, vec![tile(6, 2), tile(6, 4)]);
    assert_eq!(flip.focal, Seat::S3);
    let _ = ContextSet::EMPTY;
}
