//! rob — the whole-hand plan player (BRIEF_PLAYER_01 §7 rob; INV-P6).
//!
//! rob plays **every** decision from trick 1: at each of his turns he
//! solves his information set exactly ([`crate::solver`]) and plays the
//! materialized plan's first move — the rolling re-solve discipline. The
//! plan is the inspectable object (PLAN-NOT-TILE): `decide` returns the
//! whole [`Plan`], and the caller takes `.root.action` off the top. There
//! is no baseline delegation anywhere in rob's runtime; the slice-01
//! Monte Carlo player survives only as the paired-match opponent
//! (§1 naming law).

use rob_core::MechanicalState;

use crate::plan::Plan;
use crate::player::UtilityLens;
use crate::solver::solve;

/// rob's play-phase configuration. Bidding remains the placeholder
/// (§4 out-of-scope); the lens is `Points` in every receipt.
#[derive(Clone, Copy, Debug)]
pub struct Rob {
    /// The utility lens (`Points` unless a full-depth-only study says
    /// otherwise — `ContractSuccess` at a truncated window is a typed
    /// solve error).
    pub lens: UtilityLens,
}

impl Rob {
    /// The `Points`-lens player.
    pub fn new() -> Rob {
        Rob {
            lens: UtilityLens::Points,
        }
    }

    /// Decide at the viewer's own mechanical state: one exact solve, the
    /// whole plan back. Deterministic — same state, same plan (INV-P3).
    pub fn decide(&self, state: &MechanicalState) -> Plan {
        solve(state, self.lens).expect("the Points lens solves at every window")
    }
}

impl Default for Rob {
    fn default() -> Rob {
        Rob::new()
    }
}
