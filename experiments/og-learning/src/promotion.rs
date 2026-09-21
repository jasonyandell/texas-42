//! The registered evidence rule: risk-budgeted promotion over paired bundles.
//!
//! Candidate k (1-based, in freeze order) is graded on a fresh stream of
//! paired bundle differences D in [-1, 1] at deterministic checkpoints
//! j = 1, 2, ... with alpha[k, j] = delta / (k (k+1) j (j+1)); the double sum
//! is exactly delta, so a union bound holds every interval simultaneously.
//! Promote when mean(D) - r > tau; stop for futility when mean(D) + r < tau;
//! exhausting the budget is UNRESOLVED - a typed outcome, never a loss.

use crate::bounds::hoeffding_radius_upper;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Lower confidence bound on E[D] exceeded tau at this checkpoint.
    Promoted,
    /// Upper confidence bound fell below tau: resolved, not promoted.
    NotPromoted,
    /// Budget exhausted with the interval still straddling tau.
    Unresolved,
    /// Stream still open; next checkpoint is at `n` bundles.
    Continue { next_checkpoint: u64 },
}

#[derive(Clone, Debug)]
pub struct EvidenceRule {
    /// Total interval-failure risk across the whole run.
    pub delta: BigRational,
    /// Practical improvement threshold on E[Y_cand - Y_inc].
    pub tau: BigRational,
    /// Checkpoint schedule: bundle counts, strictly increasing.
    pub checkpoints: Vec<u64>,
}

impl EvidenceRule {
    pub fn standard() -> Self {
        EvidenceRule {
            delta: BigRational::new(BigInt::from(1), BigInt::from(20)),
            tau: BigRational::new(BigInt::from(1), BigInt::from(100)),
            checkpoints: vec![256, 1024, 4096, 16_384, 65_536],
        }
    }

    /// alpha allocated to candidate k at checkpoint j (both 1-based).
    pub fn alpha(&self, k: u64, j: u64) -> BigRational {
        assert!(k >= 1 && j >= 1);
        &self.delta
            / BigRational::new(
                BigInt::from(k) * BigInt::from(k + 1) * BigInt::from(j) * BigInt::from(j + 1),
                BigInt::one(),
            )
    }

    /// Judge candidate k after `n` bundles with paired-difference sum `sum_d`,
    /// where `n` must be the j-th checkpoint (1-based j).
    pub fn judge(&self, k: u64, j: u64, n: u64, sum_d: &BigRational) -> Verdict {
        assert_eq!(self.checkpoints[(j - 1) as usize], n);
        let mean = sum_d / BigRational::new(BigInt::from(n), BigInt::one());
        assert!(mean.clone().abs() <= BigRational::one() + BigRational::new(BigInt::one(), BigInt::from(1_000_000)));
        let r = hoeffding_radius_upper(&self.alpha(k, j), n);
        if &mean - &r > self.tau {
            return Verdict::Promoted;
        }
        if &mean + &r < self.tau {
            return Verdict::NotPromoted;
        }
        match self.checkpoints.get(j as usize) {
            Some(next) => Verdict::Continue { next_checkpoint: *next },
            None => Verdict::Unresolved,
        }
    }

    /// The whole run's alpha spend never exceeds delta (exact partial sums).
    pub fn alpha_budget_partial(&self, max_k: u64, max_j: u64) -> BigRational {
        let mut s = BigRational::zero();
        for k in 1..=max_k {
            for j in 1..=max_j {
                s += self.alpha(k, j);
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rat(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    #[test]
    fn alpha_partial_sums_close_under_delta() {
        // Law: sum over k <= K, j <= J is delta (1 - 1/(K+1)) (1 - 1/(J+1)).
        let rule = EvidenceRule::standard();
        for (kk, jj) in [(1u64, 1u64), (3, 2), (6, 6)] {
            let s = rule.alpha_budget_partial(kk, jj);
            let closed = &rule.delta
                * (BigRational::one() - rat(1, (kk + 1) as i64))
                * (BigRational::one() - rat(1, (jj + 1) as i64));
            assert_eq!(s, closed);
            assert!(s <= rule.delta);
        }
    }

    #[test]
    fn strong_candidate_promotes_and_weak_resolves() {
        let rule = EvidenceRule::standard();
        // Overwhelming paired advantage: mean D = 0.5 at n = 4096.
        let sum = rat(2048, 1);
        match rule.judge(1, 3, 4096, &sum) {
            Verdict::Promoted => {}
            v => panic!("expected promotion, got {v:?}"),
        }
        // Strongly negative: mean D = -0.5 resolves as NotPromoted.
        let sum = rat(-2048, 1);
        match rule.judge(1, 3, 4096, &sum) {
            Verdict::NotPromoted => {}
            v => panic!("expected resolution, got {v:?}"),
        }
    }

    #[test]
    fn straddling_stream_continues_then_exhausts_unresolved() {
        // PINNED strictness witness: an exactly-tau mean can never promote.
        let rule = EvidenceRule::standard();
        let n0 = rule.checkpoints[0];
        let sum = &rule.tau * BigRational::new(BigInt::from(n0), BigInt::one());
        match rule.judge(1, 1, n0, &sum) {
            Verdict::Continue { next_checkpoint } => {
                assert_eq!(next_checkpoint, rule.checkpoints[1]);
            }
            v => panic!("expected continue, got {v:?}"),
        }
        let last_j = rule.checkpoints.len() as u64;
        let n_last = *rule.checkpoints.last().unwrap();
        let sum = &rule.tau * BigRational::new(BigInt::from(n_last), BigInt::one());
        assert_eq!(rule.judge(1, last_j, n_last, &sum), Verdict::Unresolved);
    }
}
