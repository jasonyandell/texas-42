//! Exact uniform sampling from `Phi(C)`, and the exact-rational fingerprint
//! used to validate the sampler.
//!
//! Uniformity is carried by the counting DP: every branch is weighted by the
//! exact integer number of completions below it, and the draw is an integer
//! `randrange` over that total. No floating point enters a probability, and
//! the PRNG only ever selects.

use num_bigint::BigInt;
use num_rational::BigRational;
use walt_core::{Domino, DominoSet};

use crate::fiber::FiberDp;
use crate::kernel::{Kernel, World, HIDDEN_SEATS};

/// SplitMix64. Local so the workspace carries no RNG dependency; the sequence
/// is a fixed function of the seed, so every sampled corpus is reproducible.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub const fn new(seed: u64) -> SplitMix64 {
        SplitMix64 { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_u128(&mut self) -> u128 {
        (u128::from(self.next_u64()) << 64) | u128::from(self.next_u64())
    }

    /// Uniform on `0..bound`, by rejection so the result carries no modulo
    /// bias. Panics on `bound == 0`.
    pub fn below(&mut self, bound: u128) -> u128 {
        assert!(bound > 0, "empty range");
        let limit = u128::MAX - (u128::MAX % bound);
        loop {
            let r = self.next_u128();
            if r < limit {
                return r % bound;
            }
        }
    }

    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            let j = self.below(i as u128 + 1) as usize;
            xs.swap(i, j);
        }
    }
}

impl Kernel {
    /// One exactly-uniform draw from `Phi(C)`, or `None` when the fiber is
    /// empty.
    pub fn sample(&self, rng: &mut SplitMix64) -> Option<World> {
        self.sample_with(&FiberDp::new(self), rng)
    }

    /// As `sample`, reusing a DP already built for this kernel.
    pub fn sample_with(&self, dp: &FiberDp, rng: &mut SplitMix64) -> Option<World> {
        let mut remaining = dp.capacities();
        let mut cells = [DominoSet::EMPTY; HIDDEN_SEATS];
        for (i, (pattern, tiles)) in dp.groups().iter().enumerate() {
            let n = tiles.len();
            let mut cumulative: Vec<([usize; HIDDEN_SEATS], u128)> = Vec::new();
            let mut total: u128 = 0;
            for c in crate::fiber::splits(n, *pattern) {
                if (0..HIDDEN_SEATS).any(|j| c[j] > remaining[j]) {
                    continue;
                }
                let rest = core::array::from_fn(|j| remaining[j] - c[j]);
                let below = dp.completions(i + 1, rest);
                if below == 0 {
                    continue;
                }
                total += crate::fiber::multinomial(n, c) * below;
                cumulative.push((c, total));
            }
            if total == 0 {
                return None;
            }
            let r = rng.below(total);
            let chosen = cumulative
                .iter()
                .find(|(_, cum)| r < *cum)
                .map(|(c, _)| *c)
                .expect("the cumulative weights cover 0..total");

            let mut bag: Vec<Domino> = tiles.clone();
            rng.shuffle(&mut bag);
            let mut at = 0;
            for (j, cell) in cells.iter_mut().enumerate() {
                for tile in &bag[at..at + chosen[j]] {
                    cell.insert(*tile);
                }
                at += chosen[j];
                remaining[j] -= chosen[j];
            }
        }
        Some(self.world(cells))
    }
}

/// `E[distinct] = N (1 - (1 - 1/N)^n) = N - (N-1)^n / N^(n-1)`, exact.
///
/// The expected number of distinct worlds among `draws` uniform draws with
/// replacement from a fiber of size `fiber`. A sampler that is not uniform, or
/// that is stuck, misses this bracket.
pub fn expected_distinct(fiber: u128, draws: u32) -> BigRational {
    assert!(fiber > 0, "empty fiber");
    if draws == 0 {
        return BigRational::from(BigInt::from(0));
    }
    let n = BigInt::from(fiber);
    let numerator = (&n - BigInt::from(1)).pow(draws);
    let denominator = n.pow(draws - 1);
    BigRational::from(n) - BigRational::new(numerator, denominator)
}
