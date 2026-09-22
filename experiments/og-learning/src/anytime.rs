//! Anytime-valid promotion via the adjudicated CE machinery: two bounded-
//! mean betting mixtures (CE-T4/CE-T5, `walt::solver::evidence::
//! BoundedMeanMixture` - one authority, never a copy) over the paired
//! difference stream D in {-1, 0, +1}.
//!
//!   - promote side  (CE-T4, H0: E[D] <= tau): evidence >= 2/alpha_k
//!     rejects the null anytime, so E[D] > tau - PROMOTE.
//!   - futility side (CE-T5, H0: E[D] >= tau): evidence >= 2/alpha_k
//!     rejects anytime - NotPromoted.
//!
//! One alpha_k = delta/(k(k+1)) per frozen candidate (sum_k = delta), half
//! to each side; Ville's inequality makes stopping valid at EVERY n, so
//! there is no per-checkpoint alpha spend and small true edges are
//! harvested by simply continuing the stream.
//!
//! Because D takes three values, each component's wealth is
//! f(-1)^a * f(0)^b * f(+1)^c; we track only the counts and evaluate by
//! exact fast exponentiation. A gate proves this equals walt's own
//! `BoundedMeanMixture` fed observation by observation.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Pow, Zero};
use walt::solver::evidence::{affine_factor, MeanNull};

/// The declared lambda grid (equal weights). Every factor is nonnegative on
/// [-1, 1] for both sides at tau = 1/100: AtMost needs lambda <= 1/(1+tau),
/// AtLeast needs lambda <= 1/(1-tau); max grid value 1/2 clears both.
pub fn lambda_grid() -> Vec<BigRational> {
    [64i64, 32, 16, 8, 4, 2]
        .iter()
        .map(|d| BigRational::new(BigInt::one(), BigInt::from(*d)))
        .collect()
}

/// Counts of paired differences observed so far.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffCounts {
    pub minus: u64,
    pub zero: u64,
    pub plus: u64,
}

impl DiffCounts {
    pub fn push(&mut self, d: i64) {
        match d {
            -1 => self.minus += 1,
            0 => self.zero += 1,
            1 => self.plus += 1,
            other => panic!("paired make/set difference out of range: {other}"),
        }
    }

    pub fn n(&self) -> u64 {
        self.minus + self.zero + self.plus
    }

    pub fn sum(&self) -> i64 {
        self.plus as i64 - self.minus as i64
    }
}

fn value(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}

/// Exact mixture evidence for one side from counts, by fast exponentiation:
/// (1/|grid|) * sum_lambda f(-1)^a f(0)^b f(+1)^c. Equals walt's
/// `BoundedMeanMixture::evidence` after the same observations (gated below).
pub fn evidence(null: MeanNull, tau: &BigRational, counts: &DiffCounts) -> BigRational {
    let grid = lambda_grid();
    let weight = BigRational::new(BigInt::one(), BigInt::from(grid.len() as u64));
    let mut total = BigRational::zero();
    for lambda in &grid {
        let mut product = BigRational::one();
        for (v, count) in [
            (-1i64, counts.minus),
            (0, counts.zero),
            (1, counts.plus),
        ] {
            if count == 0 {
                continue;
            }
            let factor = affine_factor(null, lambda, tau, &value(v));
            product *= factor.pow(count as i32);
        }
        total += &weight * product;
    }
    total
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnytimeVerdict {
    Promoted,
    NotPromoted,
    Continue,
}

/// Judge the stream at any point: candidate k's whole-stream risk is
/// alpha_k = delta/(k(k+1)), half per side; a side rejects when its
/// evidence reaches 2/alpha_k.
pub fn judge(
    delta: &BigRational,
    tau: &BigRational,
    k: u64,
    counts: &DiffCounts,
) -> AnytimeVerdict {
    assert!(k >= 1);
    let alpha_k =
        delta / BigRational::from_integer(BigInt::from(k) * BigInt::from(k + 1));
    let threshold = BigRational::from_integer(BigInt::from(2)) / alpha_k;
    if evidence(MeanNull::AtMost, tau, counts) >= threshold {
        return AnytimeVerdict::Promoted;
    }
    if evidence(MeanNull::AtLeast, tau, counts) >= threshold {
        return AnytimeVerdict::NotPromoted;
    }
    AnytimeVerdict::Continue
}

#[cfg(test)]
mod tests {
    use super::*;
    use walt::solver::evidence::BoundedMeanMixture;

    fn tau() -> BigRational {
        BigRational::new(BigInt::one(), BigInt::from(100))
    }

    #[test]
    fn count_evidence_equals_the_adjudicated_authority_exactly() {
        // Law: the count-based fast-exponent evidence IS walt's
        // BoundedMeanMixture evidence, observation by observation, for
        // both sides, on an interleaved stream.
        let grid = lambda_grid();
        let weight = BigRational::new(BigInt::one(), BigInt::from(grid.len() as u64));
        let mixture: Vec<(BigRational, BigRational)> =
            grid.iter().map(|l| (weight.clone(), l.clone())).collect();
        for null in [MeanNull::AtMost, MeanNull::AtLeast] {
            let mut authority = BoundedMeanMixture::new(
                null,
                BigRational::from_integer(BigInt::from(-1)),
                BigRational::one(),
                tau(),
                &mixture,
            )
            .expect("lawful declared mixture");
            let mut counts = DiffCounts::default();
            let stream: Vec<i64> = (0..200)
                .map(|i| match i % 5 {
                    0 | 1 => 1,
                    2 => -1,
                    _ => 0,
                })
                .collect();
            for d in stream {
                authority.observe(&value(d));
                counts.push(d);
                // spot-check every 40 observations plus the very first
                if counts.n() % 40 == 0 || counts.n() == 1 {
                    assert_eq!(evidence(null, &tau(), &counts), authority.evidence());
                }
            }
        }
    }

    #[test]
    fn evidence_starts_at_one_and_a_positive_edge_promotes() {
        let counts = DiffCounts::default();
        assert!(evidence(MeanNull::AtMost, &tau(), &counts).is_one());
        // A stream with a strong positive edge (mean 1/8, far above tau)
        // promotes at k = 1 well before the cap.
        let delta = BigRational::new(BigInt::one(), BigInt::from(20));
        let mut c = DiffCounts::default();
        let mut verdict = AnytimeVerdict::Continue;
        for i in 0..8192u64 {
            c.push(match i % 16 {
                0 | 1 | 2 => 1,
                3 => -1,
                _ => 0,
            });
            if i % 512 == 511 {
                verdict = judge(&delta, &tau(), 1, &c);
                if verdict != AnytimeVerdict::Continue {
                    break;
                }
            }
        }
        assert_eq!(verdict, AnytimeVerdict::Promoted);
    }

    #[test]
    fn a_clearly_negative_stream_resolves_not_promoted() {
        // PINNED strictness witness: mean -1/8 crosses the futility side,
        // never the promote side.
        let delta = BigRational::new(BigInt::one(), BigInt::from(20));
        let mut c = DiffCounts::default();
        let mut verdict = AnytimeVerdict::Continue;
        for i in 0..8192u64 {
            c.push(match i % 16 {
                0 | 1 | 2 => -1,
                3 => 1,
                _ => 0,
            });
            if i % 512 == 511 {
                verdict = judge(&delta, &tau(), 1, &c);
                if verdict != AnytimeVerdict::Continue {
                    break;
                }
            }
        }
        assert_eq!(verdict, AnytimeVerdict::NotPromoted);
    }
}
