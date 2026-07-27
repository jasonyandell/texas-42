//! Seeded deterministic randomness for the player crate.
//!
//! rob-core stays RNG-free by design; all randomness lives here. The choice
//! source implements the core's exact-rational contract by rejection
//! sampling — never by biased modulo reduction.

use num_bigint::BigUint;
use num_traits::Zero;
use rob_core::ExactRationalChoiceSource;

/// SplitMix64: a small deterministic integer PRNG (integer arithmetic only,
/// INV-4).
#[derive(Clone, Debug)]
pub struct SplitMix64(u64);

impl SplitMix64 {
    /// Seeded constructor.
    pub fn new(seed: u64) -> SplitMix64 {
        SplitMix64(seed)
    }

    /// Next raw 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Exactly uniform value in `0..n` by rejection sampling (`n > 0`):
    /// draws are discarded outside the largest multiple of `n`, so every
    /// residue is exactly equally likely.
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let draw = self.next_u64();
            if draw < zone {
                return draw % n;
            }
        }
    }
}

/// An exact integer-weight choice source over a seeded PRNG
/// (implements Exec §16 `ExactRationalChoiceSource`): index `i` is returned
/// with exactly `weights[i] / Σ weights` by drawing an exactly uniform point
/// below the total. Native totals fit `u64` (every unrestricted fiber count
/// is at most 399,072,960; the implementation asserts the bound).
#[derive(Clone, Debug)]
pub struct SeededExactSource(pub SplitMix64);

impl ExactRationalChoiceSource for SeededExactSource {
    fn choose(&mut self, weights: &[BigUint]) -> usize {
        let total: BigUint = weights.iter().sum();
        assert!(total > BigUint::zero(), "positive total weight required");
        let total: u64 = u64::try_from(&total).expect("native fiber counts fit u64");
        let mut point = self.0.below(total);
        for (i, weight) in weights.iter().enumerate() {
            let w = u64::try_from(weight).expect("weight below total fits u64");
            if point < w {
                return i;
            }
            point -= w;
        }
        unreachable!("the uniform point lies below the total weight");
    }
}
