//! V-gates for `solver::evidence` — the exact-arithmetic acceptance of the
//! calculated-evidence slice (parent `walt/math/calculated_evidence_v0.1.md`
//! §19: V1, V2, V3, V7; rulings CE-A1/A5). Everything here is an exact
//! rational assertion; no tolerance appears anywhere.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::One;

use walt::solver::evidence::{
    affine_factor, bernoulli_one_step_mean, crossed, decision_delta, edge_threshold, evidence_debt,
    h_minus_min, h_plus_min, lower_threshold_evidence, pivotal_evidence, upper_threshold_evidence,
    BoundedMeanMixture, MeanNull, MixtureError, ScopedDelta,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

// ---------------------------------------------------------------------------
// V1 — exact formula anchors.
// ---------------------------------------------------------------------------

/// The §4.1 closed integer form equals the §3.1 finite sum at c = 1/2 on
/// the full grid 0 ≤ a, b ≤ 100.
#[test]
fn v1_closed_form_matches_finite_sum_at_one_half_on_the_full_grid() {
    let half = q(1, 2);
    for a in 0..=100u64 {
        for b in 0..=100u64 {
            assert_eq!(
                pivotal_evidence(a, b),
                upper_threshold_evidence(a, b, &half),
                "closed form vs finite sum at ({a},{b})"
            );
        }
    }
}

/// All nine §4.1 anchors, as exact rational assertions.
#[test]
fn v1_all_nine_anchors_are_exact() {
    let anchors: &[(u64, u64, i64, i64)] = &[
        (0, 0, 1, 1),
        (1, 0, 3, 2),
        (0, 1, 1, 2),
        (2, 0, 7, 3),
        (1, 1, 2, 3),
        (2, 1, 11, 12),
        (3, 0, 15, 4),
        (9, 0, 1023, 10),
        (10, 0, 2047, 11),
    ];
    assert_eq!(anchors.len(), 9);
    for &(a, b, num, den) in anchors {
        assert_eq!(pivotal_evidence(a, b), q(num, den), "anchor E+_({a},{b})");
    }
}

/// The unanimous row `E+_{a,0} = (2^{a+1}-1)/(a+1)` through a = 100, and
/// the calculated pivotal requirement at α = 1/128: nine consecutive
/// favorable pivots are insufficient, ten are sufficient — reproduced both
/// by direct comparison and by blind monotone search.
#[test]
fn v1_unanimous_row_and_the_alpha_1_128_pivotal_requirement() {
    for a in 0..=100u64 {
        let expected = BigRational::new(
            (BigInt::one() << (a as usize + 1)) - BigInt::one(),
            BigInt::from(a + 1),
        );
        assert_eq!(pivotal_evidence(a, 0), expected, "E+_({a},0)");
    }
    let threshold = BigRational::from_integer(BigInt::from(128));
    assert!(
        !crossed(&pivotal_evidence(9, 0), &threshold),
        "nine unanimous pivots are insufficient at alpha = 1/128"
    );
    assert!(
        crossed(&pivotal_evidence(10, 0), &threshold),
        "ten unanimous pivots are sufficient at alpha = 1/128"
    );
    assert_eq!(h_plus_min(0, 0, &threshold), 10);
    assert_eq!(h_minus_min(0, 0, &threshold), 10);
    assert_eq!(h_plus_min(9, 0, &threshold), 1);
    assert_eq!(h_plus_min(10, 0, &threshold), 0);
}

/// CE-T2 is the boxed reflection of CE-T1: `E<_{s,f}(c) = E>_{f,s}(1-c)`.
#[test]
fn ce_t2_is_the_reflected_upper_process() {
    for s in 0..=12u64 {
        for f in 0..=12u64 {
            for (n, d) in [(1i64, 3i64), (1, 2), (11, 16), (9, 10)] {
                let c = q(n, d);
                assert_eq!(
                    lower_threshold_evidence(s, f, &c),
                    upper_threshold_evidence(f, s, &(BigRational::one() - &c)),
                    "({s},{f}) at c={n}/{d}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// V2 — one-step supermartingale identities.
// ---------------------------------------------------------------------------

/// `E[L_r(B)] = 1 + (p-c)(r-c)/(c(1-c))`, and it is at most one whenever
/// `p ≤ c ≤ r` — and, by the same product-sign symmetry, whenever
/// `r ≤ c ≤ p` (the lower direction).
#[test]
fn v2_one_step_identity_and_supermartingale_bound_on_a_rational_grid() {
    let one = BigRational::one();
    let mut bounded_cases = 0u32;
    for den in [7i64, 10, 12] {
        for p_num in 0..=den {
            for c_num in 1..den {
                for r_num in 0..=den {
                    let p = q(p_num, den);
                    let c = q(c_num, den);
                    let r = q(r_num, den);
                    let lhs = bernoulli_one_step_mean(&p, &c, &r);
                    let rhs = &one + (&p - &c) * (&r - &c) / (&c * (&one - &c));
                    assert_eq!(lhs, rhs, "identity at p={p} c={c} r={r}");
                    if (p <= c && c <= r) || (r <= c && c <= p) {
                        assert!(lhs <= one, "supermartingale bound at p={p} c={c} r={r}");
                        bounded_cases += 1;
                    }
                }
            }
        }
    }
    assert!(bounded_cases > 500, "the grid exercised both directions");
}

// ---------------------------------------------------------------------------
// V3 — bounded-mean mixture validity.
// ---------------------------------------------------------------------------

/// Weight normalization, λ-range validity, and factor nonnegativity are
/// enforced at construction; the one-step expectation under the null is at
/// most one, verified analytically on a grid of two-point laws.
#[test]
fn v3_bounded_mean_mixture_validity_and_one_step_expectation() {
    let one = BigRational::one();
    let l = q(-1, 1);
    let u = q(1, 1);
    let c = q(0, 1);
    // A lawful mixture: λ ranges over [0, 1/(c-L)] = [0, 1]; the boundary
    // λ = 1 is admissible (its factor is zero at X = L, never negative).
    let lawful = [(q(1, 2), q(1, 2)), (q(1, 4), q(1, 1)), (q(1, 4), q(0, 1))];
    let process =
        BoundedMeanMixture::new(MeanNull::AtMost, l.clone(), u.clone(), c.clone(), &lawful)
            .expect("a lawful mixture constructs");
    assert_eq!(process.evidence(), one, "an evidence process starts at one");
    assert_eq!(process.observations(), 0);

    // Every declared defect is rejected at construction.
    let reject = |mixture: &[(BigRational, BigRational)], expected: MixtureError| {
        assert_eq!(
            BoundedMeanMixture::new(MeanNull::AtMost, l.clone(), u.clone(), c.clone(), mixture)
                .expect_err("an unlawful mixture is rejected"),
            expected
        );
    };
    reject(&[], MixtureError::EmptyMixture);
    reject(&[(q(1, 2), q(1, 2))], MixtureError::WeightsNotNormalized);
    reject(
        &[(q(3, 2), q(1, 2)), (q(-1, 2), q(1, 2))],
        MixtureError::NegativeWeight,
    );
    reject(&[(one.clone(), q(-1, 2))], MixtureError::NegativeLambda);
    // λ = 2 > 1/(c-L) = 1: the factor at X = L would be 1 + 2(-1) = -1.
    reject(&[(one.clone(), q(2, 1))], MixtureError::FactorMayGoNegative);
    assert_eq!(
        BoundedMeanMixture::new(MeanNull::AtMost, u.clone(), l.clone(), c.clone(), &lawful)
            .expect_err("an inverted range is rejected"),
        MixtureError::InvalidRange
    );
    // The CE-T5 direction has the mirrored λ range: 1/(U-c) = 1 here.
    assert!(BoundedMeanMixture::new(
        MeanNull::AtLeast,
        l.clone(),
        u.clone(),
        c.clone(),
        &[(one.clone(), q(2, 1))]
    )
    .is_err());

    // One-step expectation under the null, analytically: X ∈ {L, U} with
    // P(X = U) = t has mean L + (U-L)t, and
    // E[factor] = (1-t)·factor(L) + t·factor(U) = 1 + λ(mean - c) ≤ 1
    // whenever mean ≤ c.
    for t_num in 0..=8i64 {
        let t = q(t_num, 8);
        let mean = &l + (&u - &l) * &t;
        for lambda in [q(0, 1), q(1, 4), q(1, 2), q(1, 1)] {
            let expectation = (&one - &t) * affine_factor(MeanNull::AtMost, &lambda, &c, &l)
                + &t * affine_factor(MeanNull::AtMost, &lambda, &c, &u);
            assert_eq!(expectation, &one + &lambda * (&mean - &c));
            if mean <= c {
                assert!(
                    expectation <= one,
                    "null one-step bound at t={t} lambda={lambda}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// V7 — the §10.1 sign-vs-mean counterexample. PERMANENT GUARD.
// ---------------------------------------------------------------------------

/// PERMANENT GUARD (parent §10.1, gate V7; ruling CE-A5): the fixture
/// `X = +1/8 w.p. 3/4, -1/2 w.p. 1/4` has sign-majority favoring the
/// positive direction while its exact mean is -1/32. On a stream realizing
/// exactly those frequencies (three +1/8 then one -1/2, repeating — a
/// derandomized draw with the counterexample's empirical measure),
/// sign-counting evidence would settle the positive direction, and the
/// bounded-mean process at c = 0 must not. Any component that treats
/// sign-majority as evidence of a positive mean must fail this gate.
#[test]
fn v7_sign_majority_counterexample_bounded_mean_never_settles_the_positive_direction() {
    let plus = q(1, 8);
    let minus = q(-1, 2);
    // The fixture's exact mean is -1/32: the positive direction is WRONG.
    assert_eq!(q(3, 4) * &plus + q(1, 4) * &minus, q(-1, 32));

    let threshold = BigRational::from_integer(BigInt::from(128));
    let mixture = [
        (q(1, 4), q(1, 2)),
        (q(1, 4), q(1, 1)),
        (q(1, 4), q(3, 2)),
        (q(1, 4), q(2, 1)),
    ];
    let mut process = BoundedMeanMixture::new(
        MeanNull::AtMost,
        minus.clone(),
        plus.clone(),
        q(0, 1),
        &mixture,
    )
    .expect("a lawful mixture over [-1/2, 1/8]");

    let mut sign_plus = 0u64;
    let mut sign_minus = 0u64;
    let mut sign_settled_at: Option<u64> = None;
    for i in 0..512u64 {
        let x = if i % 4 == 3 { &minus } else { &plus };
        process.observe(x);
        if i % 4 == 3 {
            sign_minus += 1;
        } else {
            sign_plus += 1;
        }
        if sign_settled_at.is_none()
            && crossed(&pivotal_evidence(sign_plus, sign_minus), &threshold)
        {
            sign_settled_at = Some(i);
        }
        assert!(
            !crossed(&process.evidence(), &threshold),
            "the bounded-mean process must not settle the positive direction (step {i})"
        );
    }
    // Sign-counting WOULD have confidently settled the wrong direction …
    let settled_at = sign_settled_at
        .expect("sign-counting settles the (wrong) positive direction on this stream");
    assert!(settled_at < 512);
    // … while the bounded-mean process, which uses magnitude, ends below
    // its own starting value.
    assert!(process.evidence() < BigRational::one());
    assert_eq!(process.observations(), 512);
}

// ---------------------------------------------------------------------------
// The risk ledger and refinement arithmetic (§5, §6, §8).
// ---------------------------------------------------------------------------

#[test]
fn risk_ledger_allocations_and_evidence_debt_are_exact() {
    // §5: T_edge = m(m-1)/δ_dec.
    assert_eq!(edge_threshold(2, &q(1, 100)), q(200, 1));
    assert_eq!(edge_threshold(5, &q(1, 100)), q(2000, 1));
    assert_eq!(edge_threshold(2, &q(1, 128)), q(256, 1));
    // §6: δ_d = δ_run/(d(d+1)) telescopes below δ_run forever.
    let run = q(1, 20);
    let mut spent = BigRational::new(BigInt::from(0), BigInt::from(1));
    for d in 1..=1000u64 {
        spent += decision_delta(d, &run);
    }
    assert_eq!(spent, &run * q(1000, 1001));
    assert!(spent < run);
    // §8.1: R_debt = T/E; settlement is R_debt ≤ 1.
    assert_eq!(evidence_debt(&q(200, 1), &q(50, 1)), q(4, 1));
    assert!(evidence_debt(&q(200, 1), &q(200, 1)) <= BigRational::one());
    // §6: a δ never travels without its scope, and serializes with it.
    let scoped = ScopedDelta::new("decision:test-pair", q(1, 100));
    assert_eq!(scoped.scope(), "decision:test-pair");
    assert_eq!(scoped.delta(), &q(1, 100));
    assert_eq!(scoped.to_string(), "delta[decision:test-pair]=1/100");
}
