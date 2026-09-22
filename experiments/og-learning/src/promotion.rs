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

/// Exact per-batch statistics for a stream of integer-valued differences.
#[derive(Clone, Debug, Default)]
pub struct BatchStats {
    pub n: u64,
    pub sum: i64,
    pub sum_sq: u64,
}

impl BatchStats {
    pub fn push(&mut self, d: i64) {
        self.n += 1;
        self.sum += d;
        self.sum_sq += (d * d) as u64;
    }

    pub fn mean(&self) -> BigRational {
        BigRational::new(BigInt::from(self.sum), BigInt::from(self.n))
    }

    /// Exact sample variance (n - 1 in the denominator); needs n >= 2.
    pub fn sample_variance(&self) -> BigRational {
        assert!(self.n >= 2);
        let n = BigRational::from_integer(BigInt::from(self.n));
        let mean = self.mean();
        (BigRational::from_integer(BigInt::from(self.sum_sq)) - &n * &mean * &mean)
            / (n - BigRational::one())
    }
}

/// The multifidelity evidence rule (parent §6 wired into §8): the unbiased
/// two-batch estimator
///     Delta_hat = mean_N(D_L) + mean_M(D_H - D_L)
/// over INDEPENDENT batches (disjoint declared seed ranges), with an exact
/// empirical-Bernstein radius per batch at half the checkpoint's alpha.
/// D_L in [-1,1] (range 2); D_H - D_L in [-2,2] (range 4). Promote when
/// Delta_hat - r > tau; resolve NotPromoted when Delta_hat + r < tau;
/// exhausting checkpoints is UNRESOLVED.
#[derive(Clone, Debug)]
pub struct MfEvidenceRule {
    pub delta: BigRational,
    pub tau: BigRational,
    /// Checkpoints as (cheap pairs N_j, correction pairs M_j).
    pub checkpoints: Vec<(u64, u64)>,
}

impl MfEvidenceRule {
    /// The og-v4 gym rule: delta = 1/20 across the campaign; the declared
    /// practical improvement threshold is tau = 1/25 (the gym target's
    /// resolution economics are recorded, not hidden).
    pub fn gym() -> Self {
        MfEvidenceRule {
            delta: BigRational::new(BigInt::from(1), BigInt::from(20)),
            tau: BigRational::new(BigInt::from(1), BigInt::from(25)),
            checkpoints: vec![(16_384, 256), (65_536, 1_024), (65_536, 4_096)],
        }
    }

    pub fn alpha(&self, k: u64, j: u64) -> BigRational {
        assert!(k >= 1 && j >= 1);
        &self.delta
            / BigRational::new(
                BigInt::from(k) * BigInt::from(k + 1) * BigInt::from(j) * BigInt::from(j + 1),
                BigInt::one(),
            )
    }

    /// Judge candidate k at checkpoint j from the two batches' exact stats.
    pub fn judge(&self, k: u64, j: u64, cheap: &BatchStats, corr: &BatchStats) -> Verdict {
        let (n_j, m_j) = self.checkpoints[(j - 1) as usize];
        assert_eq!(cheap.n, n_j);
        assert_eq!(corr.n, m_j);
        let alpha = self.alpha(k, j);
        let beta = &alpha / BigRational::from_integer(BigInt::from(2));
        let r = crate::bounds::bernstein_radius_upper(&beta, cheap.n, &cheap.sample_variance(), 2)
            + crate::bounds::bernstein_radius_upper(&beta, corr.n, &corr.sample_variance(), 4);
        let estimate = cheap.mean() + corr.mean();
        if &estimate - &r > self.tau {
            return Verdict::Promoted;
        }
        if &estimate + &r < self.tau {
            return Verdict::NotPromoted;
        }
        match self.checkpoints.get(j as usize) {
            Some(_) => Verdict::Continue {
                next_checkpoint: j + 1,
            },
            None => Verdict::Unresolved,
        }
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
    fn batch_stats_are_exact_and_mf_rule_is_variance_sensitive() {
        // Law: sample variance of {1,1,0,0} is 1/3 exactly.
        let mut b = BatchStats::default();
        for d in [1i64, 1, 0, 0] {
            b.push(d);
        }
        assert_eq!(b.mean(), rat(1, 2));
        assert_eq!(b.sample_variance(), rat(1, 3));

        // A strong, low-variance advantage promotes at the final gym
        // checkpoint: cheap mean 1/8, corrections nearly all zero (high
        // proxy correlation keeps the correction variance tiny).
        let rule = MfEvidenceRule::gym();
        let (n3, m3) = rule.checkpoints[2];
        let mut cheap = BatchStats::default();
        for i in 0..n3 {
            cheap.push(if i % 8 == 0 { 1 } else { 0 });
        }
        let mut corr = BatchStats::default();
        for i in 0..m3 {
            corr.push(if i % 128 == 0 { 1 } else { 0 });
        }
        match rule.judge(1, 3, &cheap, &corr) {
            Verdict::Promoted => {}
            v => panic!("expected promotion, got {v:?}"),
        }
        // PINNED strictness witness: the same cheap batch with corrections
        // strongly against it (the proxy overstated the gain) resolves
        // NotPromoted rather than promoting.
        let mut anti = BatchStats::default();
        for i in 0..m3 {
            anti.push(if i % 4 == 0 { -1 } else { 0 });
        }
        match rule.judge(1, 3, &cheap, &anti) {
            Verdict::NotPromoted => {}
            v => panic!("expected NotPromoted, got {v:?}"),
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
