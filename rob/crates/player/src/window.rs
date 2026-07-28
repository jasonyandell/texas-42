//! The normative exact-window budget formula
//! (BRIEF_PLAYER_01 §7 window; INV-P6 WINDOW-EXACTNESS).
//!
//! The window depth `H` at a decision is the largest number of tricks whose
//! conservative work estimate — fiber count times the product of viewer
//! branching bounds `b_i = max(1, hand − i)` — fits the normative budget
//! `B = 2³²`, floored at one trick and capped at the tricks remaining.
//! `B` is a constant of the brief; changing it is an amendment, never a
//! call-site decision (§10.2). Callers cannot pass a depth: the solver
//! computes `H` from this formula alone.

/// The normative work budget `B = 2³²` (BRIEF_PLAYER_01 §7).
pub const WINDOW_BUDGET: u128 = 1 << 32;

/// The conservative work estimate for a window of `depth` tricks: fiber
/// count times `Π_{i<depth} max(1, hand − i)` (exact integer arithmetic).
pub fn window_estimate(fiber_count: u64, hand_size: usize, depth: usize) -> u128 {
    let mut estimate = fiber_count as u128;
    for i in 0..depth {
        estimate = estimate.saturating_mul(hand_size.saturating_sub(i).max(1) as u128);
    }
    estimate
}

/// The normative window depth `H` for a decision with the given exact fiber
/// count and viewer hand size: the largest `h ≤ hand_size` with
/// `window_estimate(fiber, hand, h) ≤ B`, floored at 1 (INV-P6).
pub fn window_depth(fiber_count: u64, hand_size: usize) -> usize {
    assert!(hand_size >= 1, "a decision requires a tile to play");
    let mut h = 1;
    while h < hand_size && window_estimate(fiber_count, hand_size, h + 1) <= WINDOW_BUDGET {
        h += 1;
    }
    h
}
