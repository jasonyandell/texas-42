//! World sampling: the player as the belief module's first customer.
//!
//! The belief itself is rob-core's support layer (derived cells + exact
//! count-ratio sampler); this module only adapts identities and seeds. By
//! construction a sampled world lies in the exact fiber — zero impossible
//! worlds, and in particular no sampled hand ever violates a derived void
//! (`inv_sampled_world_voids`).

use rob_core::{sample_uniform_world, DominoSet, RemainderWorld, RuleDerivedCellSystem};

use crate::rng::{SeededExactSource, SplitMix64};

/// Draw `count` exactly uniform worlds from the fiber of `cells`
/// (Math §7.8 count-ratio sampler; CELL-10E/F), mapped back to domino
/// identities.
pub fn sample_worlds(
    cells: &RuleDerivedCellSystem,
    count: usize,
    rng: &mut SplitMix64,
) -> Vec<RemainderWorld> {
    let (abstract_cells, tile_order) = cells.to_abstract();
    let mut source = SeededExactSource(SplitMix64::new(rng.next_u64()));
    (0..count)
        .map(|_| {
            let world = sample_uniform_world(&abstract_cells, &mut source);
            let sampled = RemainderWorld {
                hidden_hands: core::array::from_fn(|s| {
                    DominoSet::from_ids(world[s].iter().map(|&t| tile_order[t]))
                }),
            };
            assert!(
                cells.fiber_contains(&sampled),
                "sampled worlds lie in the exact fiber by construction"
            );
            sampled
        })
        .collect()
}
