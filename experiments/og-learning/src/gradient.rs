//! The outcome-gradient estimator (parent §3): from on-policy complete
//! deals, grad_j J = E[ Y * sum_t score_j(I_t, a_t) ], with a leave-one-out
//! baseline (measurable "before the action": deal i's baseline depends only
//! on the OTHER deals' outcomes, so the conditional-zero identity holds and
//! the estimator stays unbiased).
//!
//! Per-deal score sums are exact; before cross-deal accumulation each is
//! quantized to the declared dyadic grid 2^-32. That quantization touches
//! only this PROPOSAL statistic - never a probability, never the promotion
//! ledger, which consumes exact make/set counts only.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use crate::rollout::DealRecord;

pub const QUANT_BITS: u32 = 32;

/// Round to the nearest multiple of 2^-QUANT_BITS, half away from zero.
pub fn quantize(x: &BigRational) -> BigRational {
    let scale = BigInt::one() << QUANT_BITS;
    let scaled = x * BigRational::new(scale.clone(), BigInt::one());
    let (mut q, r) = scaled.numer().div_rem(scaled.denom());
    let twice = (&r * BigInt::from(2)).abs();
    if twice >= scaled.denom().abs() {
        q += scaled.numer().signum();
    }
    BigRational::new(q, scale)
}

pub struct GradientEstimate {
    pub per_clause: Vec<BigRational>,
    pub mean_y: BigRational,
    pub n: u64,
}

pub fn estimate(records: &[DealRecord], n_clauses: usize) -> Result<GradientEstimate, String> {
    let n = records.len();
    if n < 2 {
        return Err("gradient needs at least two deals".into());
    }
    let mut sum_y = BigRational::zero();
    for r in records {
        if r.y {
            sum_y += BigRational::one();
        }
    }
    let n_rat = BigRational::from_integer(BigInt::from(n));
    let n_minus_one = BigRational::from_integer(BigInt::from(n - 1));
    let mut grad = vec![BigRational::zero(); n_clauses];
    for r in records {
        let y = if r.y {
            BigRational::one()
        } else {
            BigRational::zero()
        };
        let baseline = (&sum_y - &y) / &n_minus_one;
        let advantage = &y - &baseline;
        for (j, s) in r.score_sums.iter().enumerate() {
            grad[j] += &advantage * quantize(s);
        }
    }
    for g in grad.iter_mut() {
        *g /= &n_rat;
    }
    Ok(GradientEstimate {
        per_clause: grad,
        mean_y: &sum_y / &n_rat,
        n: n as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::BigRational;

    fn rec(y: bool, scores: Vec<BigRational>) -> DealRecord {
        DealRecord {
            y,
            score_sums: scores,
            learner_decisions: 1,
            plies: 1,
            inference_work: 0,
        }
    }

    fn rat(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    #[test]
    fn constant_outcome_gives_exactly_zero_gradient() {
        // Law: with Y constant, the LOO advantage is exactly zero deal-wise.
        let records: Vec<DealRecord> = (0..8)
            .map(|i| rec(true, vec![rat(i, 7), rat(-i, 5)]))
            .collect();
        let g = estimate(&records, 2).unwrap();
        assert!(g.per_clause.iter().all(|x| x.is_zero()));
        assert_eq!(g.mean_y, rat(1, 1));
    }

    #[test]
    fn quantization_error_is_bounded_and_signed_correctly() {
        // Law: |quantize(x) - x| <= 2^-(QUANT_BITS+1).
        let half_ulp = BigRational::new(BigInt::one(), BigInt::one() << (QUANT_BITS + 1));
        for (n, d) in [(1i64, 3i64), (-7, 11), (22, 7), (-1, 3_000_000)] {
            let x = rat(n, d);
            let q = quantize(&x);
            assert!((q - &x).abs() <= half_ulp);
        }
        // PINNED strictness witness: a representable value is unchanged.
        let exact = BigRational::new(BigInt::from(5), BigInt::one() << 10);
        assert_eq!(quantize(&exact), exact);
    }

    #[test]
    fn a_clause_correlated_with_success_gets_positive_gradient() {
        // Score +1 on winning deals, -1 on losing deals for clause 0.
        let mut records = Vec::new();
        for _ in 0..4 {
            records.push(rec(true, vec![rat(1, 1)]));
            records.push(rec(false, vec![rat(-1, 1)]));
        }
        let g = estimate(&records, 1).unwrap();
        assert!(g.per_clause[0].is_positive());
    }
}
