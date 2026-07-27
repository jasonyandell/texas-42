//! Exact fiber counting: generating-function coefficient, deletion
//! recurrence, and the instrumented occupancy dynamic program.
//!
//! Implements Math §7.8 (CELL-10A/B/H/I) with exact `BigUint` arithmetic
//! (INV-4).

use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::support::cells::{AbstractCells, HIDDEN_SEATS};

/// Route 1 — generating-function coefficient (CELL-10A):
/// `|Φ| = [∏ x_s^{k_s}] ∏_{d∈U} (Σ_{s: d∈P_s} x_s)`, expanded as an exact
/// multivariate polynomial without capacity pruning.
pub fn count_generating_function(cells: &AbstractCells) -> BigUint {
    let mut poly: HashMap<[usize; 3], BigUint> = HashMap::new();
    poly.insert([0, 0, 0], BigUint::one());
    for tile in 0..cells.universe() {
        let mut next: HashMap<[usize; 3], BigUint> = HashMap::new();
        for (exponents, coefficient) in &poly {
            for s in 0..HIDDEN_SEATS {
                if cells.possible(s)[tile] {
                    let mut e = *exponents;
                    e[s] += 1;
                    *next.entry(e).or_insert_with(BigUint::zero) += coefficient;
                }
            }
        }
        poly = next;
    }
    let target = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
    poly.remove(&target).unwrap_or_else(BigUint::zero)
}

/// Route 2 — deletion recurrence (CELL-10B): partition the fiber by the
/// holder of the first remaining tile, `N(C) = Σ_s N(C^{d→s})`, with count
/// one at the valid empty system.
pub fn count_deletion_recurrence(cells: &AbstractCells) -> BigUint {
    if cells.universe() == 0 {
        return if (0..HIDDEN_SEATS).all(|s| cells.capacity(s) == 0) {
            BigUint::one()
        } else {
            BigUint::zero()
        };
    }
    let mut total = BigUint::zero();
    for s in 0..HIDDEN_SEATS {
        if cells.capacity(s) > 0 && cells.possible(s)[0] {
            let successor = cells
                .removal_update(s, 0)
                .expect("allowed tile with positive capacity");
            total += count_deletion_recurrence(&successor);
        }
    }
    total
}

/// Instrumentation of one occupancy-DP run (CELL-10I/I1): exact operation
/// counts matching the proved bounds.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OccupancyDpStats {
    /// The exact fiber count.
    pub count: BigUint,
    /// Distinct live occupancy vectors across the whole run.
    pub states_visited: u64,
    /// Seat tests at nonterminal live vectors (3 per processed vector).
    pub candidate_checks: u64,
    /// Checks passing the capacity condition, before allowed-edge testing.
    pub capacity_eligible_updates: u64,
    /// Maximum number of live occupancy vectors in any one layer.
    pub max_live_layer: u64,
    /// Live occupancy vectors per layer (layer 0 is the origin).
    pub layer_sizes: Vec<u64>,
}

/// Route 3 — occupancy-vector dynamic program (CELL-10H), instrumented with
/// the exact operation counters of CELL-10I.
pub fn count_occupancy_dp(cells: &AbstractCells) -> OccupancyDpStats {
    let capacity = [cells.capacity(0), cells.capacity(1), cells.capacity(2)];
    let mut layer: HashMap<[usize; 3], BigUint> = HashMap::new();
    layer.insert([0, 0, 0], BigUint::one());
    let mut states_visited = 1u64;
    let mut candidate_checks = 0u64;
    let mut capacity_eligible_updates = 0u64;
    let mut max_live_layer = 1u64;
    let mut layer_sizes = vec![1u64];
    for tile in 0..cells.universe() {
        let mut next: HashMap<[usize; 3], BigUint> = HashMap::new();
        for (occupancy, coefficient) in &layer {
            for s in 0..HIDDEN_SEATS {
                candidate_checks += 1;
                if occupancy[s] < capacity[s] {
                    capacity_eligible_updates += 1;
                    if cells.possible(s)[tile] {
                        let mut e = *occupancy;
                        e[s] += 1;
                        *next.entry(e).or_insert_with(BigUint::zero) += coefficient;
                    }
                }
            }
        }
        states_visited += next.len() as u64;
        max_live_layer = max_live_layer.max(next.len() as u64);
        layer_sizes.push(next.len() as u64);
        layer = next;
    }
    let count = layer.remove(&capacity).unwrap_or_else(BigUint::zero);
    OccupancyDpStats {
        count,
        states_visited,
        candidate_checks,
        capacity_eligible_updates,
        max_live_layer,
        layer_sizes,
    }
}

/// The exact fiber cardinality `N(C)` by the bounded native method
/// (Exec §16: native counting uses the capacity DP, never world
/// enumeration).
pub fn assignment_count(cells: &AbstractCells) -> BigUint {
    count_occupancy_dp(cells).count
}
