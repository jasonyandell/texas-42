//! Conservative exact-rational upper bounds for the promotion ledger.
//!
//! The Hoeffding radius r = sqrt(2 ln(2/alpha) / n) is irrational; the ledger
//! never needs its exact value, only an UPPER bound, because overstating the
//! radius can only delay a promotion or a futility stop, never invalidate the
//! risk budget. Everything here is exact BigRational arithmetic; no floats.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

/// A rational strictly above ln 2 (0.693147180559945...).
fn ln2_upper() -> BigRational {
    BigRational::new(BigInt::from(693_148u32), BigInt::from(1_000_000u32))
}

/// An upper bound on ln x for rational x >= 1: write x = 2^e * m with
/// m in [1, 2); then ln x = e ln 2 + ln m <= e (upper ln 2) + (m - 1),
/// using ln m <= m - 1. Exact rational, always >= the true value.
pub fn ln_upper(x: &BigRational) -> BigRational {
    assert!(*x >= BigRational::one(), "ln_upper needs x >= 1");
    let two = BigRational::new(BigInt::from(2u32), BigInt::one());
    let mut e: u64 = 0;
    let mut m = x.clone();
    while m >= two {
        m /= &two;
        e += 1;
    }
    BigRational::new(BigInt::from(e), BigInt::one()) * ln2_upper() + (m - BigRational::one())
}

/// ceil of a nonnegative rational as a BigInt.
fn ceil_big(x: &BigRational) -> BigInt {
    assert!(!x.is_negative());
    let (q, r) = x.numer().div_rem(x.denom());
    if r.is_zero() {
        q
    } else {
        q + BigInt::one()
    }
}

/// Floor integer square root (num-bigint's inherent sqrt on BigInt).
fn isqrt(n: &BigInt) -> BigInt {
    assert!(!n.is_negative());
    n.sqrt()
}

/// An upper bound y on sqrt(x) with y^2 >= x, resolved to 1/SCALE.
pub fn sqrt_upper(x: &BigRational) -> BigRational {
    assert!(!x.is_negative());
    let scale = BigInt::from(1_000_000u32);
    let m = ceil_big(&(x * BigRational::new(&scale * &scale, BigInt::one())));
    let mut s = isqrt(&m);
    if &s * &s < m {
        s += BigInt::one();
    }
    BigRational::new(s, scale)
}

/// Upper bound on the two-sided empirical-Bernstein radius (Maurer-Pontil):
/// with probability >= 1 - beta,
///   |mean - mu| <= sqrt(2 V L / n) + 7 R L / (3 (n - 1)),
/// where V is the EXACT sample variance, R the variable's range, and
/// L = ln(4/beta) (one-sided at beta/2 in each direction). Every quantity
/// here is an exact-rational upper bound; overstating the radius only
/// delays decisions, never spends unbudgeted risk.
pub fn bernstein_radius_upper(
    beta: &BigRational,
    n: u64,
    sample_variance: &BigRational,
    range: u32,
) -> BigRational {
    assert!(beta > &BigRational::zero() && beta < &BigRational::one());
    assert!(n >= 2, "a sample variance needs two observations");
    assert!(!sample_variance.is_negative());
    let l = ln_upper(&(BigRational::new(BigInt::from(4u32), BigInt::one()) / beta));
    let two = BigRational::new(BigInt::from(2u32), BigInt::one());
    let term1 = sqrt_upper(
        &(&two * sample_variance * &l / BigRational::new(BigInt::from(n), BigInt::one())),
    );
    let term2 = BigRational::new(BigInt::from(7 * range), BigInt::from(3u32))
        * &l
        / BigRational::new(BigInt::from(n - 1), BigInt::one());
    term1 + term2
}

/// Upper bound on the two-sided Hoeffding radius sqrt(2 ln(2/alpha) / n) for
/// means of n i.i.d. variables in [-1, 1].
pub fn hoeffding_radius_upper(alpha: &BigRational, n: u64) -> BigRational {
    assert!(alpha > &BigRational::zero() && alpha < &BigRational::one());
    assert!(n > 0);
    let two = BigRational::new(BigInt::from(2u32), BigInt::one());
    let inside = &two * ln_upper(&(&two / alpha))
        / BigRational::new(BigInt::from(n), BigInt::one());
    sqrt_upper(&inside)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rat(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    #[test]
    fn sqrt_upper_dominates_exactly() {
        // Law: the returned y always satisfies y^2 >= x.
        for (n, d) in [(2, 1), (3, 7), (1, 3), (17, 4), (1, 1_000_000)] {
            let x = rat(n, d);
            let y = sqrt_upper(&x);
            assert!(&y * &y >= x);
        }
        // PINNED strictness witness: at x = 4 the bound is exact.
        assert_eq!(sqrt_upper(&rat(4, 1)), rat(2, 1));
    }

    #[test]
    fn ln_upper_dominates_and_stays_tight() {
        // Law: ln_upper(2^k) >= k * (a rational strictly below ln 2).
        let ln2_lower = rat(693_147, 1_000_000);
        for k in 1u32..12 {
            let x = rat(1i64 << k, 1);
            let up = ln_upper(&x);
            let low = BigRational::new(BigInt::from(k), BigInt::one()) * &ln2_lower;
            assert!(up >= low);
            // Tightness law: the mantissa slack is below 0.09, so the bound
            // stays within (true ln + 0.09).
            assert!(up <= low + rat(9, 100) + rat(k as i64, 100_000));
        }
        // Known-value law: ln 960 = 6.8669...; the bound must cover it and
        // stay under 7.2.
        let b = ln_upper(&rat(960, 1));
        assert!(b >= rat(6_867, 1_000) && b <= rat(72, 10));
        // PINNED strictness witness: ln_upper(1) = 0 exactly.
        assert!(ln_upper(&rat(1, 1)).is_zero());
    }

    #[test]
    fn bernstein_radius_is_variance_sensitive_and_monotone() {
        // Law: at fixed n and beta, smaller sample variance gives a smaller
        // radius; at fixed variance, larger n gives a smaller radius.
        let beta = rat(1, 100);
        let lo = bernstein_radius_upper(&beta, 1024, &rat(1, 100), 2);
        let hi = bernstein_radius_upper(&beta, 1024, &rat(1, 4), 2);
        assert!(lo < hi);
        let later = bernstein_radius_upper(&beta, 4096, &rat(1, 4), 2);
        assert!(later < hi);
        // PINNED strictness witness: zero variance leaves exactly the
        // deterministic 7RL/(3(n-1)) term.
        let z = bernstein_radius_upper(&beta, 1024, &rat(0, 1), 4);
        let l = ln_upper(&rat(400, 1));
        assert_eq!(z, rat(28, 3) * l / rat(1023, 1));
    }

    #[test]
    fn radius_shrinks_with_n_and_covers_a_known_point() {
        // Law: monotone nonincreasing in n.
        let alpha = rat(1, 20);
        let r256 = hoeffding_radius_upper(&alpha, 256);
        let r1024 = hoeffding_radius_upper(&alpha, 1024);
        assert!(r1024 <= r256);
        // PINNED witness: alpha = 1/2, n = 8 -> true radius sqrt(2 ln 4 / 8)
        // = sqrt(ln 2 * 4/8)... exactly sqrt(ln(4)/4) ~ 0.58871; the upper
        // bound must be at least that and within a loose factor of it.
        let r = hoeffding_radius_upper(&rat(1, 2), 8);
        assert!(&r * &r >= rat(693_147, 2_000_000)); // >= (2*ln2_lower)/4 = ln(4)_lower/4... see below
        assert!(r <= rat(1, 1)); // sanity ceiling
    }
}
