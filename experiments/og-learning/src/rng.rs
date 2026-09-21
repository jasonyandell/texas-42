//! Deterministic seeded randomness with EXACT rational-probability sampling.
//!
//! SplitMix64 supplies the bit stream. Categorical draws bring the rational
//! action weights to a common denominator and draw a uniform BigInt below the
//! integer total by top-bit rejection, so the sampled law IS the declared
//! rational law, not a float approximation of it.

use num_bigint::{BigInt, BigUint, Sign};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        SplitMix64 { state: seed }
    }

    /// Derive an independent stream for a labeled purpose (deal index,
    /// decision index, arm) without correlating with the parent stream.
    pub fn derive(&self, label: u64) -> SplitMix64 {
        let mut child = SplitMix64::new(
            self.state ^ label.wrapping_mul(0x9E37_79B9_7F4A_7C15),
        );
        child.next_u64();
        child
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform BigUint in [0, bound) by rejection over ceil(bits/64) words.
    fn below(&mut self, bound: &BigUint) -> BigUint {
        assert!(!bound.is_zero());
        let bits = bound.bits();
        let words = bits.div_ceil(64) as usize;
        let top_mask: u64 = if bits.is_multiple_of(64) {
            u64::MAX
        } else {
            (1u64 << (bits % 64)) - 1
        };
        loop {
            let mut raw: Vec<u64> = (0..words).map(|_| self.next_u64()).collect();
            if let Some(last) = raw.last_mut() {
                *last &= top_mask;
            }
            let candidate = BigUint::from_slice(
                &raw.iter()
                    .flat_map(|w| [(w & 0xFFFF_FFFF) as u32, (w >> 32) as u32])
                    .collect::<Vec<u32>>(),
            );
            if &candidate < bound {
                return candidate;
            }
        }
    }

    /// Sample an index with probability weights[i] / sum(weights), exactly.
    /// Weights must be nonnegative rationals with a positive sum.
    pub fn sample_rational(&mut self, weights: &[BigRational]) -> usize {
        assert!(!weights.is_empty());
        let mut denom = BigInt::one();
        for w in weights {
            assert!(!w.is_negative());
            denom = num_integer::lcm(denom, w.denom().clone());
        }
        let ints: Vec<BigInt> = weights
            .iter()
            .map(|w| w.numer() * (&denom / w.denom()))
            .collect();
        let total: BigInt = ints.iter().sum();
        assert!(total.is_positive(), "sample needs positive total weight");
        let (_, total_mag) = total.into_parts();
        let draw = self.below(&total_mag);
        let mut acc = BigUint::zero();
        for (i, w) in ints.iter().enumerate() {
            let (sign, mag) = w.clone().into_parts();
            if sign != Sign::Minus {
                acc += mag;
            }
            if draw < acc {
                return i;
            }
        }
        unreachable!("draw below total must land in a bucket")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn rat(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    #[test]
    fn deterministic_and_exact_frequencies_converge() {
        // Law: the sampler is a function of the seed (determinism)…
        let w = [rat(1, 2), rat(1, 3), rat(1, 6)];
        let mut a = SplitMix64::new(42);
        let mut b = SplitMix64::new(42);
        let seq_a: Vec<usize> = (0..64).map(|_| a.sample_rational(&w)).collect();
        let seq_b: Vec<usize> = (0..64).map(|_| b.sample_rational(&w)).collect();
        assert_eq!(seq_a, seq_b);
        // …and its empirical law tracks the declared rational law loosely
        // (a smoke bound, not a statistical claim).
        let mut counts = [0u32; 3];
        let mut r = SplitMix64::new(7);
        for _ in 0..6000 {
            counts[r.sample_rational(&w)] += 1;
        }
        assert!(counts[0] > 2600 && counts[0] < 3400);
        assert!(counts[1] > 1700 && counts[1] < 2300);
        assert!(counts[2] > 700 && counts[2] < 1300);
    }

    #[test]
    fn zero_weight_bucket_is_never_drawn() {
        // PINNED strictness witness: a zero-probability action never fires.
        let w = [rat(0, 1), rat(1, 1)];
        let mut r = SplitMix64::new(1);
        for _ in 0..256 {
            assert_eq!(r.sample_rational(&w), 1);
        }
    }

    #[test]
    fn derive_yields_decorrelated_streams() {
        let root = SplitMix64::new(9);
        let mut c1 = root.derive(1);
        let mut c2 = root.derive(2);
        let s1: Vec<u64> = (0..8).map(|_| c1.next_u64()).collect();
        let s2: Vec<u64> = (0..8).map(|_| c2.next_u64()).collect();
        assert_ne!(s1, s2);
    }
}
