//! The seat-honest observation type.
//!
//! walt-strat's observation record (the play list since the kernel decision
//! point, root first -- `info.rs`) is the canonical information carrier. A
//! descriptor update consumes one play at a time, so the unit observation
//! here is a (seat, tile) pair -- a *derived view* of that record together
//! with the kernel's public data (declaration, viewer-leads convention),
//! never a parallel authority. `record_plays` is the canonical derivation.
//!
//! Nothing in this module can express a hidden hand: a seat's unplayed tiles
//! are not constructible from observations, which is what keeps every
//! `ControlSkeleton::step` seat-honest by type.

use walt_core::{Decl, Domino, Seat, Trick};

/// One observed play: the acting seat and the tile it showed. Both are
/// legal-to-see: the tile is on the table and the seat is fixed by the
/// rotation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ObservedPlay {
    pub seat: Seat,
    pub tile: Domino,
}

/// Derives the seated plays of an observation record by replaying it from
/// the kernel decision point, where the viewer leads (the
/// `Kernel::from_receipt_trick` convention): seats rotate from each trick's
/// leader, and each completed trick's winner leads the next.
///
/// Panics if a completed trick repeats a tile -- a malformed record, not a
/// reachable observation.
pub fn record_plays(decl: Decl, viewer: Seat, record: &[Domino]) -> Vec<ObservedPlay> {
    let mut out = Vec::with_capacity(record.len());
    let mut leader = viewer;
    let mut tiles = [Domino::ALL[0]; 4];
    let mut k = 0usize;
    for &tile in record {
        out.push(ObservedPlay {
            seat: leader.plus(k),
            tile,
        });
        tiles[k] = tile;
        k += 1;
        if k == 4 {
            leader = Trick::new(leader, tiles)
                .expect("a record's trick holds distinct tiles")
                .winner(decl);
            k = 0;
        }
    }
    out
}
