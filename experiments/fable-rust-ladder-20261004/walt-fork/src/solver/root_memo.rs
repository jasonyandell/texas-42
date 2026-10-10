//! Opt-in sharding of the general solver's exact count memo.
//!
//! Only large support bundles use shards. The key and value are identical to
//! the single-map solver memo; a hash selects a lock, while `CacheMap` still
//! compares complete `MemoKey`s to resolve collisions. Every lookup copies
//! the integer and releases its lock before the caller recurses.

use super::cache::{CacheHasher, CacheMap, MemoKey};
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

const SHARDS: usize = 64;
const SHARD_THRESHOLD_WORLDS: usize = 8;

#[repr(align(128))]
pub(super) struct MemoShard(Mutex<CacheMap<MemoKey, u64>>);

pub(super) enum RootMemo {
    Single(Mutex<CacheMap<MemoKey, u64>>),
    Sharded(Box<[MemoShard]>),
}

impl RootMemo {
    pub(super) fn new(world_count: usize) -> Self {
        if world_count <= SHARD_THRESHOLD_WORLDS {
            return Self::Single(Mutex::new(CacheMap::default()));
        }
        Self::Sharded(
            (0..SHARDS)
                .map(|_| MemoShard(Mutex::new(CacheMap::default())))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }

    #[inline]
    fn shard_index(key: &MemoKey) -> usize {
        let mut hasher = CacheHasher::default();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (SHARDS - 1)
    }

    #[inline]
    pub(super) fn get(&self, key: &MemoKey) -> Option<u64> {
        match self {
            Self::Single(map) => map.lock().expect("memo poisoned").get(key).copied(),
            Self::Sharded(shards) => shards[Self::shard_index(key)]
                .0
                .lock()
                .expect("memo shard poisoned")
                .get(key)
                .copied(),
        }
    }

    #[inline]
    pub(super) fn insert(&self, key: MemoKey, value: u64) {
        match self {
            Self::Single(map) => {
                map.lock().expect("memo poisoned").insert(key, value);
            }
            Self::Sharded(shards) => {
                shards[Self::shard_index(&key)]
                    .0
                    .lock()
                    .expect("memo shard poisoned")
                    .insert(key, value);
            }
        }
    }

    pub(super) fn len(&self) -> usize {
        match self {
            Self::Single(map) => map.lock().expect("memo poisoned").len(),
            Self::Sharded(shards) => shards
                .iter()
                .map(|shard| shard.0.lock().expect("memo shard poisoned").len())
                .sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::Key;
    use std::sync::Arc;

    fn key(played: u32) -> MemoKey {
        MemoKey::from(&Key {
            voids: None,
            played,
            leader: 1,
            plays: vec![0, 4],
            banked_t1: 5,
            banked_t0: 3,
            alive: 7,
        })
    }

    #[test]
    fn small_bundle_keeps_single_map_and_full_key_equality() {
        let memo = RootMemo::new(8);
        assert!(matches!(&memo, RootMemo::Single(_)));
        let a = key(1);
        let b = key(2);
        memo.insert(a, u64::MAX - 1);
        memo.insert(b, 42);
        assert_eq!(memo.get(&a), Some(u64::MAX - 1));
        assert_eq!(memo.get(&b), Some(42));
        assert_eq!(memo.len(), 2);
    }

    #[test]
    fn large_bundle_shards_concurrent_keys_and_resolves_shard_collisions() {
        let memo = Arc::new(RootMemo::new(40));
        assert!(matches!(&*memo, RootMemo::Sharded(_)));
        let mut seen = [None; SHARDS];
        let (first, colliding) = (0..=SHARDS as u32)
            .find_map(|played| {
                let candidate = key(played);
                let slot = &mut seen[RootMemo::shard_index(&candidate)];
                match slot {
                    Some(prior) => Some((*prior, candidate)),
                    None => {
                        *slot = Some(candidate);
                        None
                    }
                }
            })
            .expect("65 distinct keys force one collision in 64 shards");
        assert!(first != colliding);
        memo.insert(first, u64::MAX);
        memo.insert(colliding, u64::MAX - 1);
        assert_eq!(memo.get(&first), Some(u64::MAX));
        assert_eq!(memo.get(&colliding), Some(u64::MAX - 1));

        let keys = Arc::new((1024..1280).map(key).collect::<Vec<_>>());
        let mut workers = Vec::new();
        for _ in 0..8 {
            let memo = Arc::clone(&memo);
            let keys = Arc::clone(&keys);
            workers.push(std::thread::spawn(move || {
                for (i, &key) in keys.iter().enumerate() {
                    let value = u64::MAX - 1000 - i as u64;
                    memo.insert(key, value);
                    assert_eq!(memo.get(&key), Some(value));
                }
            }));
        }
        for worker in workers {
            worker.join().expect("worker completed");
        }
        assert_eq!(memo.len(), keys.len() + 2);
        for (i, key) in keys.iter().enumerate() {
            assert_eq!(memo.get(key), Some(u64::MAX - 1000 - i as u64));
        }
    }
}
