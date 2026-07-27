//! World sampling: the player as the belief module's first customer.
//!
//! The belief itself is rob-core's support layer (derived cells + exact
//! count-ratio sampler); since slice 02 the identity translation lives in
//! core as `sample_native_world` (CELL-10E/F), and this module only seeds
//! the exact choice source. By construction a sampled world lies in the
//! exact fiber — zero impossible worlds, and in particular no sampled hand
//! ever violates a derived void (`inv_sampled_world_voids`).

use rob_core::{sample_native_world, RemainderWorld, RuleDerivedCellSystem};

use crate::rng::{SeededExactSource, SplitMix64};

/// Draw `count` exactly uniform worlds from the fiber of `cells`
/// (Math §7.8 count-ratio sampler; CELL-10E/F).
pub fn sample_worlds(
    cells: &RuleDerivedCellSystem,
    count: usize,
    rng: &mut SplitMix64,
) -> Vec<RemainderWorld> {
    let mut source = SeededExactSource(SplitMix64::new(rng.next_u64()));
    (0..count)
        .map(|_| {
            let sampled = sample_native_world(cells, &mut source);
            assert!(
                cells.fiber_contains(&sampled),
                "sampled worlds lie in the exact fiber by construction"
            );
            sampled
        })
        .collect()
}
