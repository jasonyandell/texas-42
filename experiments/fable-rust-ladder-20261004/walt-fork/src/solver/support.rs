//! Allocation-free support identity for up to eight sampled world IDs.
//! IDs identify samples, not distinct deals: duplicate deals keep separate bits.
//! Complement encoding reserves id 0 for full support, matching public roots.
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

pub(super) struct SmallSupport {
    full: u8,
    seen: [AtomicU64; 4],
}
impl SmallSupport {
    pub fn new(n: usize) -> Option<Self> {
        if !(1..=8).contains(&n) {
            return None;
        }
        Some(Self {
            full: ((1u16 << n) - 1) as u8,
            seen: [
                AtomicU64::new(1),
                AtomicU64::new(0),
                AtomicU64::new(0),
                AtomicU64::new(0),
            ],
        })
    }
    pub fn encode(&self, mask: u8) -> u32 {
        assert_eq!(
            mask & !self.full,
            0,
            "sample IDs must belong to this solver"
        );
        let id = usize::from(mask ^ self.full);
        self.seen[id / 64].fetch_or(1u64 << (id % 64), Ordering::Relaxed);
        id as u32
    }
    pub fn encode_ids(&self, ids: &[u32]) -> u32 {
        assert!(
            ids.windows(2).all(|w| w[0] < w[1]),
            "support IDs are strictly ascending"
        );
        let mask = ids.iter().fold(0u8, |mask, &sid| {
            assert!(sid < self.full.count_ones(), "sample belongs to solver");
            mask | (1u8 << sid)
        });
        self.encode(mask)
    }
    pub fn decode(&self, id: u32) -> Alive {
        assert!(id <= u32::from(self.full), "support ID belongs to solver");
        Alive::Small((id as u8) ^ self.full)
    }
    pub fn seen_count(&self) -> usize {
        self.seen
            .iter()
            .map(|x| x.load(Ordering::Relaxed).count_ones() as usize)
            .sum()
    }
}

pub(super) enum Alive {
    Small(u8),
    #[cfg(feature = "mask64-support-arena")]
    Mask64(u64),
    Large(Arc<Vec<u32>>),
}
impl Alive {
    pub fn len(&self) -> usize {
        match self {
            Self::Small(mask) => mask.count_ones() as usize,
            #[cfg(feature = "mask64-support-arena")]
            Self::Mask64(mask) => mask.count_ones() as usize,
            Self::Large(ids) => ids.len(),
        }
    }
    pub fn iter(&self) -> AliveIter<'_> {
        match self {
            Self::Small(mask) => AliveIter::Small(*mask),
            #[cfg(feature = "mask64-support-arena")]
            Self::Mask64(mask) => AliveIter::Mask64(*mask),
            Self::Large(ids) => AliveIter::Large(ids.iter()),
        }
    }
}
pub(super) enum AliveIter<'a> {
    Small(u8),
    #[cfg(feature = "mask64-support-arena")]
    Mask64(u64),
    Large(std::slice::Iter<'a, u32>),
}
impl Iterator for AliveIter<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        match self {
            Self::Small(mask) if *mask == 0 => None,
            Self::Small(mask) => {
                let sid = mask.trailing_zeros();
                *mask &= *mask - 1;
                Some(sid)
            }
            #[cfg(feature = "mask64-support-arena")]
            Self::Mask64(mask) if *mask == 0 => None,
            #[cfg(feature = "mask64-support-arena")]
            Self::Mask64(mask) => {
                let sid = mask.trailing_zeros();
                *mask &= *mask - 1;
                Some(sid)
            }
            Self::Large(ids) => ids.next().copied(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_small_support_roundtrips_in_original_sample_order() {
        assert!(SmallSupport::new(0).is_none());
        assert!(SmallSupport::new(9).is_none());
        for n in 1..=8 {
            let support = SmallSupport::new(n).unwrap();
            assert_eq!(support.seen_count(), 1);
            assert_eq!(
                support.decode(0).iter().collect::<Vec<_>>(),
                (0..n as u32).collect::<Vec<_>>()
            );
            for mask in 0..(1u16 << n) {
                let expected = (0..n as u32)
                    .filter(|sid| mask & (1u16 << sid) != 0)
                    .collect::<Vec<_>>();
                let id = support.encode_ids(&expected);
                assert_eq!(support.decode(id).iter().collect::<Vec<_>>(), expected);
                assert_eq!(support.decode(id).len(), expected.len());
                assert_eq!(support.encode(mask as u8), id);
            }
            assert_eq!(support.seen_count(), 1 << n);
        }
    }
}
