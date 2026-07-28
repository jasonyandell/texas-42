//! The normative exact-window budget formula
//! (BRIEF_PLAYER_01 §7 window; INV-P6 WINDOW-EXACTNESS).
//!
//! The window depth `H` at a decision is the largest number of tricks whose
//! conservative work estimate — fiber count times the product of viewer
//! branching bounds `b_i = max(1, hand − i)` — fits the normative budget
//! `B = 2²⁸`, floored at one trick and capped at the tricks remaining.
//! `B` is a constant of the brief; changing it is an amendment, never a
//! call-site decision (§10.2). Callers cannot pass a depth: the solver
//! computes `H` from this formula alone.
//!
//! Amendment 2026-07-28 (recorded in BRIEF_PLAYER_01 §7): `B` lowered from
//! `2³²` to `2²⁸`. The original value priced one work unit at ~1 ns; a real
//! streamed world-segment costs ~50–100 ns (σ evaluations through the key
//! tables), so `2³²`-scale windows are hours, not minutes. At `2²⁸` every
//! streamed solve is seconds-scale, and decisions whose one-trick sweep
//! exceeds the budget take the exact response-class counting engine
//! instead ([`crate::solver`]) — exactness is never traded, only the
//! arithmetic route.

/// The normative work budget `B = 2²⁸` (BRIEF_PLAYER_01 §7, amended
/// 2026-07-28).
pub const WINDOW_BUDGET: u128 = 1 << 28;

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
    window_depth_with(fiber_count, hand_size, WINDOW_BUDGET)
}

/// [`window_depth`] under an explicit budget — the window-ablation receipt
/// surface (`r_mat_window_ablation`, B/2 and 2B). Play always uses the
/// normative constant through [`window_depth`].
pub fn window_depth_with(fiber_count: u64, hand_size: usize, budget: u128) -> usize {
    assert!(hand_size >= 1, "a decision requires a tile to play");
    let mut h = 1;
    while h < hand_size && window_estimate(fiber_count, hand_size, h + 1) <= budget {
        h += 1;
    }
    h
}
