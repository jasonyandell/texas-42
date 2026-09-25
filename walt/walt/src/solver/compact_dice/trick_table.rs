//! Completed-trick outcomes in actor order, with no runtime initialization.
//!
//! Each entry packs the winner offset in two low bits and the trick points in
//! the upper bits. Invalid repeated-tile entries are filled too, but no legal
//! search state can reach them. The dense table is 9 * 28^4 = 5,531,904 bytes.
//! Regenerate and verify it with `cargo run -p walt --example generate_trick_table`
//! and the same command followed by `-- --check`.

const SIDE: usize = 28;
const PER_DECL: usize = SIDE * SIDE * SIDE * SIDE;
const TOTAL: usize = 9 * PER_DECL;

static TABLE: &[u8; TOTAL] = include_bytes!("trick_table.bin");

#[inline]
pub(super) fn resolve(declaration: usize, packed_plays: u32) -> (u8, u8) {
    debug_assert!(declaration < 9);
    let a = (packed_plays & 31) as usize;
    let b = ((packed_plays >> 5) & 31) as usize;
    let c = ((packed_plays >> 10) & 31) as usize;
    let d = ((packed_plays >> 15) & 31) as usize;
    debug_assert!(a < SIDE && b < SIDE && c < SIDE && d < SIDE);
    let entry = TABLE[declaration * PER_DECL + (((a * SIDE + b) * SIDE + c) * SIDE + d)];

    (entry & 3, entry >> 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::rules::Trick;
    use crate::rules::{Decl, Domino, Seat};

    #[test]
    fn every_distinct_ordered_trick_matches_canonical_rules() {
        let mut checked = 0u64;
        for (di, decl) in Decl::STRAIGHT.into_iter().enumerate() {
            for a in 0..28 {
                for b in 0..28 {
                    if b == a {
                        continue;
                    }
                    for c in 0..28 {
                        if c == a || c == b {
                            continue;
                        }
                        for d in 0..28 {
                            if d == a || d == b || d == c {
                                continue;
                            }
                            let dominoes = [a, b, c, d].map(|i| Domino::ALL[i]);
                            let trick = Trick::new(Seat::S0, dominoes).unwrap();
                            let packed = a as u32
                                | ((b as u32) << 5)
                                | ((c as u32) << 10)
                                | ((d as u32) << 15);
                            let (winner, points) = resolve(di, packed);
                            assert_eq!(winner as usize, trick.winner(decl).index());
                            assert_eq!(u32::from(points), trick.points());
                            checked += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(checked, 9 * 28 * 27 * 26 * 25);
    }
}
