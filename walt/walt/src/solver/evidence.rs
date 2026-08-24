//! `solver::evidence` — the exact evidence arithmetic authority.
//!
//! EXPLORATORY tier. Implements CE-T1 through CE-T5 of the verbatim parent
//! `walt/math/calculated_evidence_v0.1.md` (intake companion beside it;
//! adjudicated `walt/CENSUS-RULINGS.md` CE-A1..A8): anytime-valid
//! exact-rational evidence processes, the risk-ledger allocations of §5/§6,
//! evidence debt and best-case pivot counts of §8, and exact threshold
//! comparisons.
//!
//! THIS MODULE MUST NOT KNOW TEXAS 42 RULES (parent §16.3). It imports no
//! rules, kernel, or solver machinery: observations arrive as bare integers
//! and rationals, and the module never learns what they measure. All
//! arithmetic is exact — `BigInt`/`BigRational`, no floats anywhere — and a
//! threshold comparison is an integer cross-multiplication.
//!
//! An evidence process here is the mathematical object of parent §3: a
//! nonnegative supermartingale starting at one under its declared null, so
//! stopping the first time it crosses `1/α` is sequentially valid with no
//! peeking correction (Ville). The observations determine how long that
//! takes; no fixed sample count appears anywhere in this module (CE-A5).

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::fmt;

/// Exact binomial coefficient `C(n, k)`.
///
/// The running product is exact at every step: the product of `i+1`
/// consecutive integers is divisible by `(i+1)!`.
pub fn binomial(n: u64, k: u64) -> BigInt {
    if k > n {
        return BigInt::zero();
    }
    let k = k.min(n - k);
    let mut out = BigInt::one();
    for i in 0..k {
        out = out * BigInt::from(n - i) / BigInt::from(i + 1);
    }
    out
}

/// CE-T1 — the upper-threshold evidence value `E>_{s,f}(c)` after `s`
/// successes and `f` failures, testing `H0: p ≤ c` against `p > c`.
///
/// Exact §3.1 finite sum with `R = (1-c)/c`:
/// `E = Σ_{i=0}^{s} C(s,i) R^i · i! f! / (i+f+1)!`, evaluated by the exact
/// term recurrence `term_0 = 1/(f+1)`, `term_{i+1} = term_i · R(s-i)/(i+f+2)`.
pub fn upper_threshold_evidence(s: u64, f: u64, c: &BigRational) -> BigRational {
    assert!(
        c > &BigRational::zero() && c < &BigRational::one(),
        "a Bernoulli threshold lies strictly inside (0,1)"
    );
    let r = (BigRational::one() - c) / c;
    let mut term = BigRational::new(BigInt::one(), BigInt::from(f + 1));
    let mut sum = term.clone();
    for i in 0..s {
        term = term * &r * BigRational::new(BigInt::from(s - i), BigInt::from(i + f + 2));
        sum += &term;
    }
    sum
}

/// CE-T2 — the lower-threshold evidence value `E<_{s,f}(c)`, testing
/// `H0: p ≥ c` against `p < c`. By the parent's boxed definition,
/// `E<_{s,f}(c) = E>_{f,s}(1-c)` (the intake verified this equals the
/// independently constructed lower-test mixture).
pub fn lower_threshold_evidence(s: u64, f: u64, c: &BigRational) -> BigRational {
    upper_threshold_evidence(f, s, &(BigRational::one() - c))
}

/// CE-T3 — exact pivotal-direction evidence `E+_{a,b}` after `a` pivotal
/// wins for the tested direction and `b` against, via the §4.1 closed
/// integer form (the parent names this the preferred implementation):
///
/// `E+_{a,b} = (Σ_{x=0}^{a} C(k+1,x)) / ((k+1)·C(k,a))` with `k = a+b`.
///
/// Nonpivotal observations leave the value unchanged and are already
/// covered by the raw-world supermartingale argument of §4 — waiting
/// through them creates no fake directional evidence. The evidence for the
/// opposite direction is `E+_{b,a}`.
pub fn pivotal_evidence(a: u64, b: u64) -> BigRational {
    let k = a + b;
    let mut coeff = BigInt::one(); // C(k+1, 0)
    let mut prefix = BigInt::one();
    for x in 0..a {
        // C(k+1, x+1) = C(k+1, x) · (k+1-x)/(x+1), exact at every step.
        coeff = coeff * BigInt::from(k + 1 - x) / BigInt::from(x + 1);
        prefix += &coeff;
    }
    let denom = BigInt::from(k + 1) * binomial(k, a);
    BigRational::new(prefix, denom)
}

/// The exact one-step mean of the Bernoulli likelihood factor `L_r(B)`
/// under success probability `p`:
/// `E[L_r(B)] = p·r/c + (1-p)(1-r)/(1-c)`.
///
/// The V2 gate checks this equals `1 + (p-c)(r-c)/(c(1-c))` and is at most
/// one whenever `p ≤ c ≤ r` (and, by symmetry, whenever `r ≤ c ≤ p`).
pub fn bernoulli_one_step_mean(p: &BigRational, c: &BigRational, r: &BigRational) -> BigRational {
    assert!(
        c > &BigRational::zero() && c < &BigRational::one(),
        "a Bernoulli threshold lies strictly inside (0,1)"
    );
    p * r / c + (BigRational::one() - p) * (BigRational::one() - r) / (BigRational::one() - c)
}

/// Which one-sided mean hypothesis a bounded-mean betting process tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeanNull {
    /// CE-T4: tests `H0: E[X] ≤ c` (evidence accumulates when the mean is
    /// actually above `c`); factors `1 + λ(X - c)`.
    AtMost,
    /// CE-T5: tests `H0: E[X] ≥ c`; factors `1 - λ(X - c)`.
    AtLeast,
}

/// A constructor-time defect in a declared mixture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MixtureError {
    EmptyMixture,
    /// The declared observation range must satisfy `L < U`.
    InvalidRange,
    NegativeWeight,
    WeightsNotNormalized,
    NegativeLambda,
    /// Some factor could go negative somewhere in the declared `[L, U]`:
    /// the λ exceeds the CE-T4/T5 range for this threshold and range.
    FactorMayGoNegative,
}

/// The one-step betting factor: `1 + λ(x - c)` for [`MeanNull::AtMost`],
/// `1 - λ(x - c)` for [`MeanNull::AtLeast`]. Public so validation gates can
/// check nonnegativity and one-step expectations analytically.
pub fn affine_factor(
    null: MeanNull,
    lambda: &BigRational,
    threshold: &BigRational,
    x: &BigRational,
) -> BigRational {
    let step = lambda * (x - threshold);
    match null {
        MeanNull::AtMost => BigRational::one() + step,
        MeanNull::AtLeast => BigRational::one() - step,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Component {
    weight: BigRational,
    lambda: BigRational,
    product: BigRational,
}

/// CE-T4/CE-T5 — a finite rational mixture of bounded-mean betting
/// processes over observations declared to lie in `[L, U]`.
///
/// Construction enforces the full validity contract exactly: weights
/// nonnegative and summing to one, every `λ ≥ 0`, and every factor
/// nonnegative over the whole declared range (the factor is affine in `X`,
/// so the two endpoints decide; this is exactly the parent's
/// `0 ≤ λ ≤ 1/(c-L)` resp. `0 ≤ λ ≤ 1/(U-c)` where those bounds bind).
/// Under its null the mixture value is a nonnegative supermartingale
/// starting at one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedMeanMixture {
    null: MeanNull,
    lower: BigRational,
    upper: BigRational,
    threshold: BigRational,
    components: Vec<Component>,
    observations: u64,
}

impl BoundedMeanMixture {
    /// A lawful mixture, or the first defect found. `mixture` lists
    /// `(weight, λ)` pairs.
    pub fn new(
        null: MeanNull,
        lower: BigRational,
        upper: BigRational,
        threshold: BigRational,
        mixture: &[(BigRational, BigRational)],
    ) -> Result<BoundedMeanMixture, MixtureError> {
        if mixture.is_empty() {
            return Err(MixtureError::EmptyMixture);
        }
        if lower >= upper {
            return Err(MixtureError::InvalidRange);
        }
        let mut weight_sum = BigRational::zero();
        for (weight, lambda) in mixture {
            if weight.is_negative() {
                return Err(MixtureError::NegativeWeight);
            }
            if lambda.is_negative() {
                return Err(MixtureError::NegativeLambda);
            }
            for x in [&lower, &upper] {
                if affine_factor(null, lambda, &threshold, x).is_negative() {
                    return Err(MixtureError::FactorMayGoNegative);
                }
            }
            weight_sum += weight;
        }
        if !weight_sum.is_one() {
            return Err(MixtureError::WeightsNotNormalized);
        }
        Ok(BoundedMeanMixture {
            null,
            lower,
            upper,
            threshold,
            components: mixture
                .iter()
                .map(|(weight, lambda)| Component {
                    weight: weight.clone(),
                    lambda: lambda.clone(),
                    product: BigRational::one(),
                })
                .collect(),
            observations: 0,
        })
    }

    /// Fold one observation into every component. The observation must lie
    /// in the declared range — that is the caller's declared contract, so a
    /// violation is a panic, not an error value.
    pub fn observe(&mut self, x: &BigRational) {
        assert!(
            &self.lower <= x && x <= &self.upper,
            "an observation left the declared range"
        );
        for component in &mut self.components {
            let factor = affine_factor(self.null, &component.lambda, &self.threshold, x);
            component.product *= factor;
        }
        self.observations += 1;
    }

    /// The current exact mixture evidence `Σ_j w_j M_n(λ_j; c)`.
    pub fn evidence(&self) -> BigRational {
        self.components
            .iter()
            .fold(BigRational::zero(), |acc, component| {
                acc + &component.weight * &component.product
            })
    }

    pub fn observations(&self) -> u64 {
        self.observations
    }
}

/// A risk budget that never travels without its scope (parent §6: "a δ
/// value without its scope is meaningless"). Serialized by `Display` as
/// `delta[<scope>]=<num>/<den>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopedDelta {
    scope: String,
    delta: BigRational,
}

impl ScopedDelta {
    pub fn new(scope: impl Into<String>, delta: BigRational) -> ScopedDelta {
        assert!(
            delta > BigRational::zero() && delta < BigRational::one(),
            "a risk budget lies strictly inside (0,1)"
        );
        let scope = scope.into();
        assert!(!scope.is_empty(), "a risk budget carries its scope");
        ScopedDelta { scope, delta }
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn delta(&self) -> &BigRational {
        &self.delta
    }
}

impl fmt::Display for ScopedDelta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "delta[{}]={}", self.scope, self.delta)
    }
}

/// §5 — the common directed-edge evidence threshold for `m` frozen
/// candidates under the equal all-pairs allocation:
/// `T_edge = m(m-1)/δ_dec`.
pub fn edge_threshold(m: u64, delta_dec: &BigRational) -> BigRational {
    assert!(m >= 2, "a comparison needs at least two candidates");
    assert!(
        delta_dec > &BigRational::zero(),
        "a risk budget is positive"
    );
    BigRational::from_integer(BigInt::from(m * (m - 1))) / delta_dec
}

/// §6 — the run-level allocation for the `d`-th decision event:
/// `δ_d = δ_run/(d(d+1))`, which telescopes to `δ_run` over all `d ≥ 1`.
pub fn decision_delta(d: u64, delta_run: &BigRational) -> BigRational {
    assert!(d >= 1, "decision events are numbered from one");
    assert!(
        delta_run > &BigRational::zero(),
        "a risk budget is positive"
    );
    delta_run / BigRational::from_integer(BigInt::from(d) * BigInt::from(d + 1))
}

/// §8.1 — the exact evidence debt `R_debt = T/E`. Settlement is
/// `R_debt ≤ 1`; no logarithm enters the correctness path.
pub fn evidence_debt(threshold: &BigRational, evidence: &BigRational) -> BigRational {
    assert!(
        evidence > &BigRational::zero(),
        "evidence processes are positive"
    );
    threshold / evidence
}

/// The exact threshold comparison `E ≥ T`, as an integer
/// cross-multiplication. `BigRational` normalizes denominators positive, so
/// no sign case analysis is needed.
pub fn crossed(evidence: &BigRational, threshold: &BigRational) -> bool {
    evidence.numer() * threshold.denom() >= threshold.numer() * evidence.denom()
}

/// §8.2 — `h+_min(a,b;T)`: the exact minimum number of additional
/// favorable pivotal observations that could settle the positive
/// direction, by monotone search. `E+_{a+h,b}` is nondecreasing in `h`
/// (appending a favorable pivot multiplies the §4 integrand by
/// `(1+t) ≥ 1`) and unbounded, so doubling plus binary search is exact.
pub fn h_plus_min(a: u64, b: u64, threshold: &BigRational) -> u64 {
    if crossed(&pivotal_evidence(a, b), threshold) {
        return 0;
    }
    let mut hi = 1u64;
    while !crossed(&pivotal_evidence(a + hi, b), threshold) {
        hi *= 2;
        assert!(hi < 1u64 << 40, "the threshold is unreachably far");
    }
    let mut lo = hi / 2;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if crossed(&pivotal_evidence(a + mid, b), threshold) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

/// §8.2 — `h-_min(a,b;T) = min{h: E+_{b+h,a} ≥ T}`: the mirror bound for
/// the negative direction.
pub fn h_minus_min(a: u64, b: u64, threshold: &BigRational) -> u64 {
    h_plus_min(b, a, threshold)
}
