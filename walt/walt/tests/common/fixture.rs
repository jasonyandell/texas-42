//! Recompute-once fixture support for the gate suites (BRIEF-CI1).
//!
//! A suite whose gates all read the same expensive oracle values (an
//! exact `Q_a`, a census, a controller run) computes them ONCE per
//! test-binary process into a `LazyLock` fixture — a derived view of the
//! suite's declared inputs, immutable after construction — and every gate
//! reads the fixture and asserts exactly what it asserted before.
//! Independence that matters is between CODE PATHS, never between
//! recomputations of one function; a gate whose law is "two runs agree"
//! keeps one fresh run and compares it to the fixture's.
//!
//! Construction is scheduled across the machine's threads: the values
//! are pure functions of their keys, so the order they are computed in
//! is not observable. Nothing here changes which assertions run.
//!
//! Included per suite via `#[path = "common/fixture.rs"] mod fixture;`.

#![allow(dead_code)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// `f` at every key, once each, in key order — computed concurrently on
/// up to `available_parallelism` threads. Put the heaviest keys first:
/// the makespan then approaches the longest single key.
pub fn compute_all<K: Sync, T: Send>(keys: &[K], f: impl Fn(&K) -> T + Sync) -> Vec<T> {
    let next = AtomicUsize::new(0);
    let workers = thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(keys.len().max(1));
    let mut slots: Vec<Option<T>> = (0..keys.len()).map(|_| None).collect();
    thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut computed = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= keys.len() {
                            break;
                        }
                        computed.push((i, f(&keys[i])));
                    }
                    computed
                })
            })
            .collect();
        for handle in handles {
            for (i, value) in handle.join().expect("a fixture worker completes") {
                slots[i] = Some(value);
            }
        }
    });
    slots
        .into_iter()
        .map(|slot| slot.expect("every fixture key was computed"))
        .collect()
}
