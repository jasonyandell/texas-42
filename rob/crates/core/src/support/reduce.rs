//! Marginal holder support and the canonical support reduction.
//!
//! Implements Math §7.9 (CELL-10J/K/L): the exact edge-support criterion
//! (forced-successor Hall feasibility) and the fiber-preserving,
//! contractive, idempotent canonical reduction `red(C)`.

use crate::support::cells::{AbstractCells, HIDDEN_SEATS};

/// Exact marginal-edge criterion (CELL-10K): `d ∈ P_s*` iff `d ∈ P_s`,
/// `k_s > 0`, and the forced successor `C^{d→s}` is Hall-feasible.
pub fn marginal_allowed(cells: &AbstractCells, seat: usize, tile: usize) -> bool {
    if cells.capacity(seat) == 0 || !cells.possible(seat)[tile] {
        return false;
    }
    cells
        .removal_update(seat, tile)
        .map(|successor| successor.is_feasible())
        .unwrap_or(false)
}

/// The canonical support reduction `red(C)` (CELL-10L): replace every
/// allowed set by the exact marginal holder support, keeping pool and
/// capacities.
pub fn reduce(cells: &AbstractCells) -> AbstractCells {
    let possible = core::array::from_fn(|s| {
        (0..cells.universe())
            .map(|tile| marginal_allowed(cells, s, tile))
            .collect::<Vec<bool>>()
    });
    let capacity = core::array::from_fn(|s| cells.capacity(s));
    AbstractCells::new(cells.universe(), possible, capacity)
        .expect("reduction preserves the structural schema")
}

/// Marginal holder support by direct world projection (the independent
/// route of CELL-10J's verification): `P_s* = ⋃_{ω∈Φ} H_s(ω)`.
pub fn marginal_by_projection(cells: &AbstractCells) -> [Vec<bool>; HIDDEN_SEATS] {
    let mut marginal = core::array::from_fn(|_| vec![false; cells.universe()]);
    for world in cells.worlds() {
        for (s, hand) in world.iter().enumerate() {
            for &tile in hand {
                marginal[s][tile] = true;
            }
        }
    }
    marginal
}
