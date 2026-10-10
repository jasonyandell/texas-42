//! Interned support for 9–64 original sampled world IDs.
//!
//! One bit represents each original sample position, even when two sampled
//! deals have identical hands. Intern IDs remain u32, with ID zero denoting
//! the full root support, just as in the Arc<Vec<u32>> representation.

use super::cache::CacheMap;
use super::support::Alive;
use std::sync::Mutex;

struct InternedMasks {
    list: Vec<u64>,
    map: CacheMap<u64, u32>,
}

pub(super) struct Mask64Arena {
    full: u64,
    worlds: usize,
    interned: Mutex<InternedMasks>,
}

impl Mask64Arena {
    pub(super) fn new(worlds: usize) -> Option<Self> {
        if !(9..=64).contains(&worlds) {
            return None;
        }
        let full = if worlds == 64 {
            u64::MAX
        } else {
            (1u64 << worlds) - 1
        };
        let mut map = CacheMap::default();
        map.insert(full, 0);
        Some(Self {
            full,
            worlds,
            interned: Mutex::new(InternedMasks {
                list: vec![full],
                map,
            }),
        })
    }

    pub(super) fn intern_ids(&self, ids: &[u32]) -> u32 {
        assert!(
            ids.windows(2).all(|pair| pair[0] < pair[1]),
            "support IDs are strictly ascending"
        );
        let mut mask = 0u64;
        for &sid in ids {
            assert!((sid as usize) < self.worlds, "sample ID belongs to solver");
            mask |= 1u64 << sid;
        }
        self.intern_mask(mask)
    }

    fn intern_mask(&self, mask: u64) -> u32 {
        assert_eq!(mask & !self.full, 0, "sample IDs belong to solver");
        let mut table = self.interned.lock().expect("support arena poisoned");
        if let Some(&id) = table.map.get(&mask) {
            return id;
        }
        let id = u32::try_from(table.list.len()).expect("support intern IDs fit in u32");
        table.list.push(mask);
        table.map.insert(mask, id);
        id
    }

    pub(super) fn decode(&self, id: u32) -> Alive {
        let table = self.interned.lock().expect("support arena poisoned");
        Alive::Mask64(
            *table
                .list
                .get(id as usize)
                .expect("support ID belongs to solver"),
        )
    }

    pub(super) fn seen_count(&self) -> usize {
        self.interned
            .lock()
            .expect("support arena poisoned")
            .list
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn roundtrip_original_positions_at_nine_forty_and_sixty_four() {
        assert!(Mask64Arena::new(8).is_none());
        assert!(Mask64Arena::new(65).is_none());
        for n in [9, 40, 64] {
            let arena = Mask64Arena::new(n).unwrap();
            assert_eq!(arena.seen_count(), 1);
            assert_eq!(
                arena.decode(0).iter().collect::<Vec<_>>(),
                (0..n as u32).collect::<Vec<_>>()
            );
            let ids = [0, 1, (n - 1) as u32];
            let id = arena.intern_ids(&ids);
            assert_ne!(id, 0);
            assert_eq!(arena.intern_ids(&ids), id);
            assert_eq!(arena.decode(id).iter().collect::<Vec<_>>(), ids.to_vec());
            assert_eq!(arena.decode(id).len(), ids.len());
            // Equal deals at two positions still occupy two sample-ID bits.
            let duplicate_deal_positions = arena.intern_ids(&[1, 2]);
            assert_eq!(arena.decode(duplicate_deal_positions).len(), 2);
            assert_eq!(
                arena
                    .decode(duplicate_deal_positions)
                    .iter()
                    .collect::<Vec<_>>(),
                vec![1, 2]
            );
            let empty = arena.intern_ids(&[]);
            assert_eq!(arena.intern_ids(&[]), empty);
            assert_eq!(arena.decode(empty).len(), 0);
        }
    }

    #[test]
    fn concurrent_interning_returns_one_id_per_mask() {
        for n in [9, 40, 64] {
            let arena = Arc::new(Mask64Arena::new(n).unwrap());
            let last = (n - 1) as u32;
            let supports = [[0, 3, last], [1, 3, last], [0, 4, last], [1, 4, last]];
            let mut workers = Vec::new();
            for _ in 0..8 {
                let arena = Arc::clone(&arena);
                workers.push(std::thread::spawn(move || {
                    for _ in 0..128 {
                        for ids in supports {
                            let id = arena.intern_ids(&ids);
                            assert_eq!(arena.decode(id).iter().collect::<Vec<_>>(), ids.to_vec());
                        }
                    }
                }));
            }
            for worker in workers {
                worker.join().expect("worker completed");
            }
            assert_eq!(arena.seen_count(), supports.len() + 1);
            let ids = supports.map(|support| arena.intern_ids(&support));
            assert!(ids.iter().all(|&id| id != 0));
            assert!(ids.iter().enumerate().all(|(i, id)| !ids[..i].contains(id)));
        }
    }
}
