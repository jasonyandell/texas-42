//! Sampler validation: every draw is a fiber member, and the collision
//! fingerprint sits where exact uniform sampling puts it.
//!
//! The bracket is a fingerprint, not a hypothesis test. The seed is fixed, so
//! the run is deterministic; what the bracket buys is that a sampler which is
//! stuck, skewed, or silently ignoring a void falls outside it.

mod common;

use std::collections::HashMap;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{expected_distinct, Kernel, SplitMix64, World};

use common::{kernel_at, receipt, true_world};

fn rational(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

/// Draws `n` worlds and returns the per-world tally.
fn draw(kernel: &Kernel, n: usize, seed: u64) -> HashMap<[walt::rules::DominoSet; 4], usize> {
    let mut rng = SplitMix64::new(seed);
    let mut tally = HashMap::new();
    for _ in 0..n {
        let w: World = kernel.sample(&mut rng).expect("a nonempty fiber");
        assert!(kernel.contains(&w), "sampled a world outside the fiber");
        *tally.entry(w.hands()).or_insert(0usize) += 1;
    }
    tally
}

/// `E[distinct] = N - (N-1)^n / N^(n-1)`, in exact rationals.
#[test]
fn expected_distinct_is_exact() {
    // N = 2, n = 3: 2 - 1/4 = 7/4.
    assert_eq!(
        expected_distinct(2, 3),
        BigRational::new(BigInt::from(7), BigInt::from(4))
    );
    // One draw can only ever show one world.
    for n in 1..50u128 {
        assert_eq!(expected_distinct(n, 1), rational(1));
    }
    // The expectation is bounded by both the fiber and the draw count.
    for n in [6u128, 19, 90, 1680] {
        for draws in [1u32, 5, 200] {
            let e = expected_distinct(n, draws);
            assert!(e <= rational(n as i64));
            assert!(e <= rational(i64::from(draws)));
            assert!(e > rational(0));
        }
    }
}

/// h0t6: fiber 90, 200 draws. The exact expectation is ~80.2; a stuck sampler
/// lands near 1 and a sampler that ignores a constraint reports worlds that
/// `contains` rejects.
#[test]
fn collision_fingerprint_on_hand_0_trick_6() {
    let r = receipt();
    let (kernel, _) = kernel_at(&r, 0, 6);
    assert_eq!(kernel.count(), 90);

    const DRAWS: usize = 200;
    let tally = draw(&kernel, DRAWS, 0x42);
    let distinct = rational(tally.len() as i64);
    let expected = expected_distinct(90, DRAWS as u32);
    let slack = rational(12);
    assert!(
        distinct >= &expected - &slack && distinct <= &expected + &slack,
        "distinct {distinct} outside {expected} +/- {slack}"
    );
    assert_eq!(tally.values().sum::<usize>(), DRAWS);
}

/// A small fiber is covered, and every world is drawn at a plausible rate.
/// h12t6 has 6 worlds; 6000 draws puts each at 1000 in expectation.
#[test]
fn small_fiber_is_covered_uniformly() {
    let r = receipt();
    let (kernel, hand) = kernel_at(&r, 12, 6);
    assert_eq!(kernel.count(), 6);

    const DRAWS: usize = 6000;
    let tally = draw(&kernel, DRAWS, 0x9E37);
    assert_eq!(tally.len(), 6, "every world must be reachable");
    for (world, n) in &tally {
        assert!(
            (800..=1200).contains(n),
            "world {world:?} drawn {n} times, expected about {}",
            DRAWS / 6
        );
    }
    let truth = true_world(&kernel, hand, 6);
    assert!(
        tally.contains_key(&truth.hands()),
        "the true world is drawn"
    );

    // The enumeration and the sampler agree on the support.
    let enumerated: std::collections::HashSet<_> = kernel.worlds().map(|w| w.hands()).collect();
    assert_eq!(enumerated, tally.keys().copied().collect());
}

/// Every sampled world of a void-constrained kernel respects the voids.
#[test]
fn sampling_respects_voids_across_the_corpus() {
    let r = receipt();
    let mut rng = SplitMix64::new(7);
    for hand_id in 0..13 {
        for trick_no in 3..=6 {
            let (kernel, _) = kernel_at(&r, hand_id, trick_no);
            for _ in 0..8 {
                let w = kernel.sample(&mut rng).expect("a nonempty fiber");
                assert!(kernel.contains(&w), "h{hand_id}t{trick_no}: {w:?}");
                for (i, h) in kernel.hidden().iter().enumerate() {
                    for q in h.voids.iter() {
                        for d in w.hand(h.seat).iter() {
                            assert!(!kernel.decl().follows(d, q));
                        }
                    }
                    assert!(w.hand(h.seat).is_subset_of(kernel.allowed(i)));
                }
            }
        }
    }
}

/// The PRNG selects without modulo bias and is a pure function of the seed.
#[test]
fn prng_is_reproducible_and_unbiased_enough_to_select() {
    let a: Vec<u64> = {
        let mut rng = SplitMix64::new(1);
        (0..8).map(|_| rng.next_u64()).collect()
    };
    let b: Vec<u64> = {
        let mut rng = SplitMix64::new(1);
        (0..8).map(|_| rng.next_u64()).collect()
    };
    assert_eq!(a, b, "the same seed must replay");
    let c: Vec<u64> = {
        let mut rng = SplitMix64::new(2);
        (0..8).map(|_| rng.next_u64()).collect()
    };
    assert_ne!(a, c);

    let mut rng = SplitMix64::new(0xABCD);
    let mut buckets = [0usize; 7];
    for _ in 0..7000 {
        buckets[rng.below(7) as usize] += 1;
    }
    for n in buckets {
        assert!((850..=1150).contains(&n), "below(7) buckets: {buckets:?}");
    }
    assert_eq!(buckets.iter().sum::<usize>(), 7000);

    let mut ordered: Vec<u32> = (0..20).collect();
    let before = ordered.clone();
    rng.shuffle(&mut ordered);
    assert_ne!(ordered, before);
    ordered.sort_unstable();
    assert_eq!(ordered, before, "shuffle is a permutation");
}
