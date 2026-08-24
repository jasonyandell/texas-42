//! Gates for §22 step 8: the V5 flip repair and the per-fixed-pair E0
//! calibration arithmetic (`solver::calibrate`). Parent:
//! `walt/math/calculated_evidence_v0.1.md` §19 V5/V6, §7, §8.4; rulings
//! CE-A1..A8, L2-A6. Everything here is regression evidence at
//! exploratory tier; nothing is promoted.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::rules::{Domino, DominoSet, Seat};
use walt::solver::adaptive::{exact_frozen_pair, root_identity, SlicePolicy};
use walt::solver::calibrate::{
    assert_cap_ladder, d_half_bounds, dp_settlement_forecast, exact_set_outcomes,
    information_rate_bounds, leading_order_forecast_bounds, ln_bounds, pair_coordinates,
    receipt_root, reconstruct_flip, shadow_scopes, shadow_tuple, CapLadderVerdict,
    CountTimingSpec, PredictiveLaw, FLIP_FIXTURES,
};
use walt::solver::controller::{
    epoch_identity, evaluate_set, CandidateSet, RiskPlan, SetResult, SetSpec,
};
use walt::solver::evidence::{decision_delta, edge_threshold, h_plus_min, ScopedDelta};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, Level0Field, TieRule,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

// ---------------------------------------------------------------------------
// Series bounds: exact rational sandwiches for D_{1/2}(τ) and ln.
// ---------------------------------------------------------------------------

/// The D-bounds are ordered, shrink with depth, vanish exactly at τ = 0,
/// are even in τ, and at |τ| = 1 the interval must be consistent with the
/// independently-derived atanh interval for ln 2 (D_{1/2}(1) = ln 2): two
/// different series, one number, overlapping sandwiches.
#[test]
fn d_half_bounds_are_ordered_shrinking_and_consistent_at_ln_two() {
    let (lo0, hi0) = d_half_bounds(&BigRational::zero(), 8);
    assert!(lo0.is_zero() && hi0.is_zero());
    let tau = q(3, 5);
    let (lo_a, hi_a) = d_half_bounds(&tau, 4);
    let (lo_b, hi_b) = d_half_bounds(&tau, 16);
    assert!(lo_a > BigRational::zero());
    assert!(lo_a <= hi_a && lo_b <= hi_b);
    assert!(lo_b >= lo_a && hi_b <= hi_a, "deeper series tighten");
    let (lo_neg, hi_neg) = d_half_bounds(&-tau, 4);
    assert_eq!((lo_neg, hi_neg), (lo_a, hi_a), "D is even in τ");
    let one = BigRational::one();
    let (d1_lo, d1_hi) = d_half_bounds(&one, 64);
    let (ln2_lo, ln2_hi) = ln_bounds(&q(2, 1), 64);
    assert!(d1_lo <= ln2_hi && ln2_lo <= d1_hi, "both sandwich ln 2");
}

/// The atanh interval for ln 2 pins the fifth decimal at modest depth;
/// ln 1 = 0 exactly; and the ln 4 interval must be consistent with twice
/// the ln 2 interval (additivity of the true value).
#[test]
fn ln_bounds_anchor_and_additivity() {
    let (lo1, hi1) = ln_bounds(&BigRational::one(), 8);
    assert!(lo1.is_zero() && hi1.is_zero());
    let (lo2, hi2) = ln_bounds(&q(2, 1), 40);
    assert!(lo2 <= hi2);
    assert!(lo2 > q(6931, 10000), "ln 2 > 0.6931");
    assert!(hi2 < q(6932, 10000), "ln 2 < 0.6932");
    let (lo4, hi4) = ln_bounds(&q(4, 1), 40);
    let twice = (q(2, 1) * &lo2, q(2, 1) * &hi2);
    assert!(lo4 <= twice.1 && twice.0 <= hi4, "ln 4 = 2 ln 2 overlap");
}

/// Regime 4 honestly yields no forecast; elsewhere the interval is
/// ordered and positive, and a larger tilt at the same pivotal mass
/// yields a lower-or-equal information floor (D is nondecreasing in |τ|).
#[test]
fn information_rate_and_leading_order_regimes() {
    let t = edge_threshold(2, &q(1, 200));
    assert_eq!(t, q(400, 1));
    assert!(information_rate_bounds(&BigRational::zero(), &q(1, 2), 8).is_none());
    assert!(information_rate_bounds(&q(1, 2), &BigRational::zero(), 8).is_none());
    assert!(leading_order_forecast_bounds(&t, &q(1, 2), &BigRational::zero(), 8).is_none());
    let (lo, hi) = leading_order_forecast_bounds(&t, &q(1, 4), &q(1, 2), 16).expect("regime 3");
    assert!(BigRational::zero() < lo && lo <= hi);
    let (rate_small_lo, _) = information_rate_bounds(&q(1, 4), &q(1, 2), 16).expect("rate");
    let (rate_large_lo, _) = information_rate_bounds(&q(1, 4), &q(9, 10), 16).expect("rate");
    assert!(rate_large_lo > rate_small_lo, "more tilt, more information");
}

// ---------------------------------------------------------------------------
// The §8.4 exact forecast DP.
// ---------------------------------------------------------------------------

/// Pure-tilt anchors, exact: with p̃+ = 1 the crossing at any γ is the
/// exact pivot minimum h+_min(0,0;T); with p̃+ = 1/2 and p̃0 = 1/2 the
/// γ = 1/2 crossing is exactly 2·h+_min − 1, because
/// `P(Bin(2j−1, 1/2) ≥ j) = 1/2` exactly by symmetry.
#[test]
fn dp_forecast_matches_pivot_minimum_in_the_pure_tilt_regime() {
    let t = q(128, 1);
    let j = h_plus_min(0, 0, &t);
    assert_eq!(j, 10, "E+_{{10,0}} = 2047/11 > 128 > E+_{{9,0}}");
    let pure = PredictiveLaw::new(BigRational::one(), BigRational::zero());
    let forecast = dp_settlement_forecast(&pure, &t, (0, 0), &q(9, 10), 64);
    assert_eq!(forecast.crossing, Some(j));
    let lazy = PredictiveLaw::new(q(1, 2), BigRational::zero());
    let forecast = dp_settlement_forecast(&lazy, &t, (0, 0), &q(1, 2), 64);
    assert_eq!(forecast.crossing, Some(2 * j - 1));
    assert_eq!(forecast.f_at_end, q(1, 2), "binomial symmetry, exactly");
}

/// With no pivotal probability the forecast never crosses and the exact
/// absorbed mass is zero — regime 4 stays honest in the DP too. A settled
/// start crosses at zero.
#[test]
fn dp_forecast_never_settles_without_pivots() {
    let t = q(128, 1);
    let dead = PredictiveLaw::new(BigRational::zero(), BigRational::zero());
    let forecast = dp_settlement_forecast(&dead, &t, (0, 0), &q(1, 2), 32);
    assert_eq!(forecast.crossing, None);
    assert!(forecast.f_at_end.is_zero());
    let pure = PredictiveLaw::new(BigRational::one(), BigRational::zero());
    let settled = dp_settlement_forecast(&pure, &t, (20, 0), &q(1, 2), 32);
    assert_eq!(settled.crossing, Some(0));
    assert!(settled.f_at_end.is_one());
}

// ---------------------------------------------------------------------------
// Exact coordinates versus the exact frozen-pair endpoint.
// ---------------------------------------------------------------------------

fn preference_tuple(
    position: &walt::solver::adaptive::RootPosition,
    order: Vec<Domino>,
) -> FreezeTuple {
    FreezeTuple {
        solver_source: "calibrate-test-preference-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "fixed-preference".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::None,
        discovery_stream: "none".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::FirstInPreference,
        practical_equivalence: None,
        policy_library: "calibrate-test-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::Preference(order),
    }
}

fn ascending() -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index(i).expect("tile < 28"))
        .collect()
}

fn descending() -> Vec<Domino> {
    ascending().into_iter().rev().collect()
}

/// The exact per-pair coordinates and the step-6 exact frozen-pair
/// endpoint are two readouts of one enumeration: `n` is the fiber,
/// `a − b` is the win-count difference, and `q ≥ |g|` with `g = q·τ`.
#[test]
fn pair_coordinates_cross_check_against_exact_frozen_pair() {
    let r = receipt();
    let (root, position) = receipt_root(&r.hands[0], 6);
    let low = FrozenPolicy::new(preference_tuple(&position, ascending()));
    let high = FrozenPolicy::new(preference_tuple(&position, descending()));
    let field = walt::solver::adaptive::FixedPreference::lowest_first("field:lowest-v1");
    let outcomes = exact_set_outcomes(
        &root,
        &position,
        &[&low as &dyn SlicePolicy, &high as &dyn SlicePolicy],
        &field,
    );
    let coords = pair_coordinates(&outcomes[0], &outcomes[1]);
    let exact = exact_frozen_pair(
        &root,
        &position,
        &low,
        &high,
        &field,
        &std::collections::HashMap::new(),
    );
    let walt::solver::adaptive::ResultKind::ExactFrozenSet {
        wins_a,
        wins_b,
        fiber,
        ..
    } = exact.result
    else {
        panic!("the exact endpoint returns ExactFrozenSet");
    };
    assert_eq!(coords.n, fiber);
    assert_eq!(
        BigInt::from(coords.a) - BigInt::from(coords.b),
        BigInt::from(wins_a) - BigInt::from(wins_b),
        "pivotal difference equals win-count difference"
    );
    assert_eq!(coords.n0 + coords.a + coords.b, coords.n);
    let g_abs = if coords.g < BigRational::zero() {
        -coords.g.clone()
    } else {
        coords.g.clone()
    };
    assert!(coords.q >= g_abs, "q >= |g|");
    if let Some(tau) = &coords.tau {
        assert_eq!(&coords.q * tau, coords.g, "g = q tau exactly");
    }
}

// ---------------------------------------------------------------------------
// The flip fixtures: reconstruction fidelity (§19 V5).
// ---------------------------------------------------------------------------

/// Every flip fixture reconstructs by rules replay into EXACTLY the
/// shadow run's decision: same legal set and m (asserted inside), same
/// fiber count (asserted inside), and — the byte-level anchor — the same
/// §5.3 evaluation epoch, which folds the root identity, the candidate
/// PolicyIds, and the declared δ. A reconstruction that differed anywhere
/// would land on a different stream; this pins V5's re-runs to the
/// recorded specimens.
#[test]
fn flip_fixtures_reconstruct_and_reproduce_the_shadow_epochs() {
    for f in &FLIP_FIXTURES {
        let flip = reconstruct_flip(f);
        let candidates_owned: Vec<FrozenPolicy> = flip
            .legal_tiles
            .iter()
            .map(|t| FrozenPolicy::new(shadow_tuple(&flip.position, *t)))
            .collect();
        let candidates = CandidateSet::new(candidates_owned.iter().collect());
        let (run_scope, dec_scope) = shadow_scopes(f);
        let dec_delta = decision_delta(f.d, &q(1, 100));
        let plan = RiskPlan::strict(ScopedDelta::new(dec_scope, dec_delta))
            .under_run(ScopedDelta::new(run_scope, q(1, 100)), f.d);
        let root_id = root_identity(&flip.root, &flip.position);
        let epoch = epoch_identity(root_id, &candidates, plan.decision());
        assert_eq!(
            epoch.to_string(),
            f.epoch,
            "the reconstruction reproduces the shadow epoch ({} hand {})",
            f.mode,
            f.hand
        );
    }
}

/// On the three small-fiber fixtures the exact frozen-set win counts
/// reproduce the shadow records exactly (the 1750-world fixture is
/// covered by the committed v5flip probe run, which asserts the same).
#[test]
fn flip_fixture_exact_wins_reproduce_on_the_small_fibers() {
    for f in FLIP_FIXTURES.iter().filter(|f| f.fiber <= 100) {
        let flip = reconstruct_flip(f);
        let candidates_owned: Vec<FrozenPolicy> = flip
            .legal_tiles
            .iter()
            .map(|t| FrozenPolicy::new(shadow_tuple(&flip.position, *t)))
            .collect();
        let refs: Vec<&dyn SlicePolicy> = candidates_owned
            .iter()
            .map(|p| p as &dyn SlicePolicy)
            .collect();
        let field = Level0Field::new(2);
        let outcomes = exact_set_outcomes(&flip.root, &flip.position, &refs, &field);
        let wins: Vec<u128> = outcomes
            .iter()
            .map(|u| u.iter().filter(|x| **x).count() as u128)
            .collect();
        assert_eq!(
            wins.as_slice(),
            f.exact_wins,
            "exact wins reproduce ({} hand {})",
            f.mode,
            f.hand
        );
    }
}

// ---------------------------------------------------------------------------
// The V5 law: settled means settled under stream extension.
// ---------------------------------------------------------------------------

/// Mechanical demonstration of the V5 stopping-time property on a real
/// root: find a receipt decision where a cheap frozen pair δ-settles,
/// then re-run the SAME epoch and stream at a ladder of caps including
/// the historical coordinates 40 and 160 and assert the monotone story —
/// `Unresolved` strictly below the settlement index, and the IDENTICAL
/// `DeltaSettled` (same winner, same index) at every cap above it. The
/// evidence process is a stopping time on one stream; a cap can truncate
/// the story but can never rewrite it.
#[test]
fn cap_ladder_settles_and_stays_settled() {
    let r = receipt();
    let field = walt::solver::adaptive::FixedPreference::lowest_first("field:lowest-v1");
    let mut demonstrated = false;
    'hands: for hand in &r.hands {
        for trick_no in [5usize, 4] {
            let (root, position) = receipt_root(hand, trick_no);
            if root.count() < 4 {
                continue;
            }
            let low = FrozenPolicy::new(preference_tuple(&position, ascending()));
            let high = FrozenPolicy::new(preference_tuple(&position, descending()));
            let candidates = CandidateSet::new(vec![&low, &high]);
            let evaluate_at = |cap: u64| {
                let plan = RiskPlan::strict(ScopedDelta::new(
                    format!("decision:calibrate-v5-h{}-t{trick_no}", hand.id),
                    q(1, 2),
                ));
                let spec = SetSpec {
                    root: &root,
                    position: &position,
                    candidates: &candidates,
                    field: &field,
                    plan,
                    world_cap: cap,
                    batch: 8,
                    escalation: None,
                };
                evaluate_set(&spec)
            };
            let probe = evaluate_at(256);
            let SetResult::DeltaSettled {
                winner, settled_at, ..
            } = probe.result
            else {
                continue;
            };
            // The pair settled at index s on this stream. Assert the
            // full ladder story.
            let s = settled_at;
            let mut ladder: Vec<(u64, SetResult)> = Vec::new();
            for cap in [40, 160, s.max(1), s + 1, 2 * (s + 1), 4 * (s + 1)] {
                if ladder.iter().any(|(c, _)| *c == cap) {
                    continue;
                }
                ladder.push((cap, evaluate_at(cap).result));
            }
            ladder.sort_by_key(|(cap, _)| *cap);
            for (cap, result) in &ladder {
                match result {
                    SetResult::DeltaSettled {
                        winner: w,
                        settled_at: at,
                        ..
                    } => {
                        assert!(*cap > s, "settlement needs the stream to reach index s");
                        assert_eq!((*w, *at), (winner, s), "settled means settled");
                    }
                    SetResult::Unresolved { .. } => {
                        assert!(*cap <= s, "an honest Unresolved only below the crossing");
                    }
                    other => panic!("unexpected ladder result {}", other.tag()),
                }
            }
            let refs: Vec<(u64, &SetResult)> =
                ladder.iter().map(|(c, res)| (*c, res)).collect();
            match assert_cap_ladder(&refs) {
                CapLadderVerdict::SettledStable {
                    winner: w,
                    settled_at: at,
                    ..
                } => assert_eq!((w, at), (winner, s)),
                other => panic!("expected SettledStable, got {other:?}"),
            }
            // Settlement obeys the exact pivot minimum from the initial
            // state: crossing T needs at least h+_min(0,0;T) pivots.
            let threshold = edge_threshold(2, &q(1, 2));
            let final_counts = probe
                .pair_counts
                .first()
                .expect("one pair");
            assert!(
                final_counts.a + final_counts.b >= h_plus_min(0, 0, &threshold),
                "settlement carries at least the exact minimum pivotal work"
            );
            demonstrated = true;
            break 'hands;
        }
    }
    assert!(
        demonstrated,
        "some receipt decision must let a cheap frozen pair settle"
    );
}

/// The count-timing family reconstructs the specimen SHAPE exactly: sixes
/// trump, bid 30, the bidder's 6-6 lead standing, S3 (the partner) to act
/// at trick 1 ply 2 with legal exactly {6-2, 6-4}, and a trick-1-sized
/// fiber (tens of millions — far beyond exact enumeration, which is why
/// the ladder outcome there is adaptive-only).
#[test]
fn count_timing_family_reconstructs_the_specimen_shape() {
    let spec = CountTimingSpec::new(0, 2);
    let flip = spec.root();
    assert_eq!(flip.focal, Seat::S3);
    assert_eq!(flip.legal_tiles.len(), 2);
    assert!(flip.root.count() > 1_000_000, "a trick-1 fiber");
    let again = CountTimingSpec::new(0, 2);
    assert_eq!(again.deal, spec.deal, "the family is deterministic");
    assert_eq!(again.s2_tile, spec.s2_tile);
}

/// AMBIGUITY/BLOCKED MARKER (CLAUDE.md ambiguity protocol). Parent §19 V5
/// says "reconstruct the live count-timing position where the 40-world
/// and 160-world choices differed." The LITERAL position is not
/// reconstructible from this repository: the 2026-08-23 plunge review
/// hands' game seeds live plunge-side, and ruling L2-A6
/// (`walt/CENSUS-RULINGS.md`) cards their reconstruction as
/// [[gran-anchor-reconstruction]]. Until those seeds land, the V5 gate
/// runs (a) the count-timing SHAPE family and (b) the step-7 shadow run's
/// four exact-route disagreements — flip-shaped specimens with full
/// in-repo provenance. Unignore when the plunge seeds arrive.
#[test]
#[ignore = "blocked: plunge-side game seeds (L2-A6 [[gran-anchor-reconstruction]])"]
fn v5_literal_count_timing_position_reconstructs() {
    panic!("the literal plunge position's seeds are not in this repository");
}
