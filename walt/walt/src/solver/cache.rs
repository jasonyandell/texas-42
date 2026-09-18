//! Private search-table representation. Full equality still resolves hash
//! collisions; these hashes never seed samples, choose actions, or identify
//! persisted evidence. Public information-state types remain unchanged.
use super::{Key, PiKey};
use std::{collections::HashMap, hash::{BuildHasherDefault, Hasher}};

#[derive(Default)]
pub(super) struct CacheHasher(u64);
impl CacheHasher {
    #[inline]
    fn word(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
}
impl Hasher for CacheHasher {
    #[inline] fn finish(&self) -> u64 { self.0 }
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks=bytes.chunks_exact(8);
        for chunk in &mut chunks { self.word(u64::from_le_bytes(chunk.try_into().expect("eight bytes"))); }
        let rest=chunks.remainder();
        if !rest.is_empty() {
            let mut tail=[0u8;8]; tail[..rest.len()].copy_from_slice(rest);
            self.word(u64::from_le_bytes(tail));
        }
    }
    #[inline] fn write_u8(&mut self,v:u8) { self.word(u64::from(v)); }
    #[inline] fn write_u16(&mut self,v:u16) { self.word(u64::from(v)); }
    #[inline] fn write_u32(&mut self,v:u32) { self.word(u64::from(v)); }
    #[inline] fn write_u64(&mut self,v:u64) { self.word(v); }
    #[inline] fn write_usize(&mut self,v:usize) { self.word(v as u64); }
}
pub(super) type CacheMap<K,V> = HashMap<K,V,BuildHasherDefault<CacheHasher>>;

// An unfinished trick has at most three tiles. The leading sentinel encodes
// length, including empty and leading zero tiles: no field is discarded.
fn packed_plays(plays: &[u8]) -> u16 {
    assert!(plays.len()<=3,"a search key holds only an unfinished trick");
    plays.iter().fold(1u16, |value,&tile| {
        assert!(tile<28,"valid domino index");
        (value<<5)|u16::from(tile)
    })
}
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub(super) struct MemoKey {
    voids: Option<[u32;4]>, played:u32, alive:u32, plays:u16,
    leader:u8, banked_t1:u8, banked_t0:u8,
}
impl From<&Key> for MemoKey {
    fn from(k:&Key) -> Self {
        Self {voids:k.voids,played:k.played,alive:k.alive,plays:packed_plays(&k.plays),
            leader:k.leader,banked_t1:k.banked_t1,banked_t0:k.banked_t0}
    }
}
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub(super) struct PolicyKey {
    voids:Option<[u32;4]>, hand:u32, played:u32, plays:u16,
    seat:u8, leader:u8, banked_t1:u8, banked_t0:u8,
}
impl From<&PiKey> for PolicyKey {
    fn from(k:&PiKey) -> Self {
        Self {voids:k.voids,hand:k.hand,played:k.played,plays:packed_plays(&k.plays),
            seat:k.seat,leader:k.leader,banked_t1:k.banked_t1,banked_t0:k.banked_t0}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn packed_tricks_are_injective_including_zero_and_length() {
        let mut seen=HashSet::new();
        assert!(seen.insert(packed_plays(&[])));
        for a in 0..28 {
            assert!(seen.insert(packed_plays(&[a])));
            for b in 0..28 {
                assert!(seen.insert(packed_plays(&[a,b])));
                for c in 0..28 { assert!(seen.insert(packed_plays(&[a,b,c]))); }
            }
        }
        assert_eq!(seen.len(),1+28+28*28+28*28*28);
    }
}
