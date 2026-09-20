//! Exact strength reduction for the frozen small-bound SplitMix draw.
//!
//! For d>=2 let m=floor(2^64/d). For every u64 x, q=high64(x*m)
//! underestimates floor(x/d) by at most one. Therefore r=x-q*d is in
//! [0,2d), and subtracting d once when necessary yields exactly x%d.
//! The rejection zone and RNG consumption remain the historical ones.

use super::SplitMix64;

const DIVISORS: [(u64, u64); 29] = {
    let mut result = [(0, 0); 29];
    let mut n = 1usize;
    while n <= 28 {
        let d = n as u64;
        let reciprocal = if n == 1 {
            0
        } else {
            ((1u128 << 64) / n as u128) as u64
        };
        result[n] = (u64::MAX - u64::MAX % d, reciprocal);
        n += 1;
    }
    result
};

#[inline]
pub(super) fn below(rng: &mut SplitMix64, n: u64) -> u64 {
    // Dice decisions only need bounds <=7. Constant divisors avoid the
    // reciprocal-table load and high multiply for powers of two, while
    // preserving the historical rejection zone and stream consumption.
    let (zone, reciprocal) = DIVISORS[n as usize];
    loop {
        let value = rng.next_u64();
        if value >= zone {
            continue;
        }
        if n == 1 {
            return 0;
        }
        let quotient = ((u128::from(value) * u128::from(reciprocal)) >> 64) as u64;
        // q*n <= value, by the proof above. Wrapping arithmetic expresses
        // the proved machine operation without redundant overflow branches.
        let remainder = value.wrapping_sub(quotient.wrapping_mul(n));
        return if remainder >= n {
            remainder - n
        } else {
            remainder
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(rng: &mut SplitMix64, n: u64) -> u64 {
        let zone = u64::MAX - u64::MAX % n;
        loop {
            let value = rng.next_u64();
            if value < zone {
                return value % n;
            }
        }
    }

    fn unxor(v: u64, shift: u32) -> u64 {
        let mut x = v;
        let mut p = shift;
        while p < 64 {
            x ^= v >> p;
            p += shift;
        }
        x
    }

    fn inverse_odd(a: u64) -> u64 {
        let mut x = 1u64;
        for _ in 0..6 {
            x = x.wrapping_mul(2u64.wrapping_sub(a.wrapping_mul(x)));
        }
        assert_eq!(a.wrapping_mul(x), 1);
        x
    }

    fn state_for_first(value: u64) -> u64 {
        let mut x = unxor(value, 31);
        x = x.wrapping_mul(inverse_odd(0x94D0_49BB_1331_11EB));
        x = unxor(x, 27);
        x = x.wrapping_mul(inverse_odd(0xBF58_476D_1CE4_E5B9));
        x = unxor(x, 30);
        x.wrapping_sub(0x9E37_79B9_7F4A_7C15)
    }

    #[test]
    fn frozen_outputs_and_rng_state_include_forced_rejections() {
        for n in 1..=28 {
            let zone = u64::MAX - u64::MAX % n;
            for first in [0, n - 1, n, zone - 1, zone, u64::MAX] {
                let seed = state_for_first(first);
                assert_eq!(SplitMix64(seed).next_u64(), first);
                let mut a = SplitMix64(seed);
                let mut b = SplitMix64(seed);
                for _ in 0..1000 {
                    assert_eq!(a.below(n), reference(&mut b, n), "n={n}, first={first}");
                    assert_eq!(a.0, b.0, "identical rejection count and RNG state");
                }
            }
        }
        for n in [29, 64, 1_000_001, u64::MAX] {
            let mut a = SplitMix64(918273);
            let mut b = SplitMix64(918273);
            for _ in 0..1000 {
                assert_eq!(a.below(n), reference(&mut b, n));
                assert_eq!(a.0, b.0);
            }
        }
    }
}
