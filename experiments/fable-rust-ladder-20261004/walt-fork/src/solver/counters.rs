//! Diagnostic counters with an optional worker-sharded implementation.
//!
//! `fetch_add`/`fetch_max` intentionally return `()`: unlike AtomicU64, no
//! linearizable previous global value is available when sharded. These
//! counters must not control search, cancellation, or sampling. Loads after
//! workers join are exact (sum modulo 2^64, or maximum); concurrent loads
//! are diagnostic snapshots, not one atomic global observation. Ordering
//! applies to the individual atomics, never synchronization across slots.
//!
//! A Rayon worker uses its pool index when it fits the allocation; other
//! threads use a fallback atomic slot. Sharing across pools remains correct
//! even if worker indices collide, although it can increase contention.

use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "sharded-counters")]
#[repr(align(128))]
struct Slot(AtomicU64);

/// A sum counter (`MAX=false`) or maximum counter (`MAX=true`).
pub struct Counter<const MAX: bool> {
    #[cfg(not(feature = "sharded-counters"))]
    value: AtomicU64,
    #[cfg(feature = "sharded-counters")]
    slots: Box<[Slot]>,
}

pub type SumCounter = Counter<false>;
pub type MaxCounter = Counter<true>;

impl<const MAX: bool> Counter<MAX> {
    pub fn new(initial: u64) -> Self {
        #[cfg(not(feature = "sharded-counters"))]
        {
            Self {
                value: AtomicU64::new(initial),
            }
        }
        #[cfg(feature = "sharded-counters")]
        {
            #[cfg(feature = "parallel")]
            let workers = rayon::current_num_threads();
            #[cfg(not(feature = "parallel"))]
            let workers = 0;
            let slots = (0..=workers)
                .map(|index| Slot(AtomicU64::new(if index == workers { initial } else { 0 })))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            Self { slots }
        }
    }

    #[inline]
    fn writer(&self) -> &AtomicU64 {
        #[cfg(not(feature = "sharded-counters"))]
        {
            &self.value
        }
        #[cfg(feature = "sharded-counters")]
        {
            let fallback = self.slots.len() - 1;
            #[cfg(feature = "parallel")]
            let index = rayon::current_thread_index()
                .filter(|&index| index < fallback)
                .unwrap_or(fallback);
            #[cfg(not(feature = "parallel"))]
            let index = fallback;
            &self.slots[index].0
        }
    }

    /// Exact after writers join; a non-atomic snapshot during concurrent work.
    #[inline]
    pub fn load(&self, ordering: Ordering) -> u64 {
        #[cfg(not(feature = "sharded-counters"))]
        {
            self.value.load(ordering)
        }
        #[cfg(feature = "sharded-counters")]
        {
            self.slots.iter().fold(0u64, |total, slot| {
                let value = slot.0.load(ordering);
                if MAX {
                    total.max(value)
                } else {
                    total.wrapping_add(value)
                }
            })
        }
    }
}

impl SumCounter {
    /// Add to this writer's slot. There is intentionally no previous-value return.
    #[inline]
    pub fn fetch_add(&self, value: u64, ordering: Ordering) {
        self.writer().fetch_add(value, ordering);
    }
}

impl MaxCounter {
    /// Raise this writer's maximum. There is no previous-value return.
    #[inline]
    pub fn fetch_max(&self, value: u64, ordering: Ordering) {
        self.writer().fetch_max(value, ordering);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_max_and_initial_values() {
        let sum = SumCounter::new(7);
        let maximum = MaxCounter::new(12);
        sum.fetch_add(11, Ordering::Relaxed);
        sum.fetch_add(13, Ordering::Relaxed);
        maximum.fetch_max(8, Ordering::Relaxed);
        maximum.fetch_max(19, Ordering::Relaxed);
        assert_eq!(sum.load(Ordering::Relaxed), 31);
        assert_eq!(maximum.load(Ordering::Relaxed), 19);
    }

    #[test]
    fn sum_matches_atomic_wrapping() {
        let sum = SumCounter::new(u64::MAX);
        sum.fetch_add(2, Ordering::Relaxed);
        assert_eq!(sum.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn ordinary_threads_share_fallback_without_losing_updates() {
        let sum = SumCounter::new(5);
        let maximum = MaxCounter::new(0);
        std::thread::scope(|scope| {
            for worker in 0..4u64 {
                let sum = &sum;
                let maximum = &maximum;
                scope.spawn(move || {
                    for i in 0..1000 {
                        sum.fetch_add(worker + 1, Ordering::Relaxed);
                        maximum.fetch_max(worker * 1000 + i, Ordering::Relaxed);
                    }
                });
            }
        });
        assert_eq!(sum.load(Ordering::Relaxed), 10_005);
        assert_eq!(maximum.load(Ordering::Relaxed), 3999);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn rayon_workers_and_external_writer_aggregate_exactly() {
        use rayon::prelude::*;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let (sum, maximum) = pool.install(|| (SumCounter::new(3), MaxCounter::new(2)));
        pool.install(|| {
            (0..10_000u64).into_par_iter().for_each(|i| {
                sum.fetch_add(i, Ordering::Relaxed);
                maximum.fetch_max(i, Ordering::Relaxed);
            });
        });
        sum.fetch_add(7, Ordering::Relaxed);
        maximum.fetch_max(10_001, Ordering::Relaxed);
        assert_eq!(sum.load(Ordering::Relaxed), 49_995_010);
        assert_eq!(maximum.load(Ordering::Relaxed), 10_001);
    }

    #[cfg(all(feature = "parallel", feature = "sharded-counters"))]
    #[test]
    fn each_worker_updates_its_own_padded_slot() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let sum = pool.install(|| SumCounter::new(0));
        pool.broadcast(|_| sum.fetch_add(1, Ordering::Relaxed));
        assert_eq!(sum.slots.len(), 5);
        for slot in &sum.slots[..4] {
            assert_eq!(slot.0.load(Ordering::Relaxed), 1);
        }
        assert_eq!(sum.slots[4].0.load(Ordering::Relaxed), 0);
        sum.fetch_add(2, Ordering::Relaxed);
        assert_eq!(sum.slots[4].0.load(Ordering::Relaxed), 2);
        assert_eq!(sum.load(Ordering::Relaxed), 6);
    }

    #[cfg(all(feature = "parallel", feature = "sharded-counters"))]
    #[test]
    fn different_pool_sizes_remain_safe() {
        use rayon::prelude::*;
        let small = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        let large = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let sum = small.install(|| SumCounter::new(0));
        assert_eq!(sum.slots.len(), 2);
        large.install(|| {
            (0..10_000)
                .into_par_iter()
                .for_each(|_| sum.fetch_add(1, Ordering::Relaxed))
        });
        assert_eq!(sum.load(Ordering::Relaxed), 10_000);
    }

    #[cfg(feature = "sharded-counters")]
    #[test]
    fn slots_have_128_byte_alignment_and_stride() {
        assert_eq!(std::mem::align_of::<Slot>(), 128);
        assert_eq!(std::mem::size_of::<Slot>(), 128);
    }
}
