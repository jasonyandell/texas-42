//! The common continuation policy — the structural form of the one law.
//!
//! Math §11.4: "Choosing the best action separately in each hidden world and
//! then averaging is strategy fusion." The evaluator therefore fixes ONE
//! policy object before any world is drawn and replays a *clone of the same
//! object* (identical private tape) for every seat in every simulated world.
//! A policy sees only the public rollout view and the legal set; there is no
//! representable way for it to learn which sampled world it is in, and no
//! per-world optimization hook exists anywhere in the player API.

use rob_core::{Declaration, DominoId, Play, Seat};

use crate::rng::SplitMix64;

/// The public information a continuation policy may consult: declaration,
/// trick residue, running score, and the acting seat. Deliberately excludes
/// every hidden hand and any world identity.
pub struct RolloutView<'a> {
    /// The declared interpretation.
    pub declaration: Declaration,
    /// The current trick leader.
    pub leader: Seat,
    /// The current partial trick.
    pub trick: &'a [Play],
    /// Banked points by partnership.
    pub points: [u32; 2],
    /// The seat about to act.
    pub actor: Seat,
}

/// A common continuation policy: one legal-action choice as a function of
/// the public view, the legal set, and the policy's own private tape —
/// nothing else (§11.4; UTIL-04A/B: private randomization independent of the
/// hidden world). `Clone` is required so the evaluator can replay the
/// *identical* tape in every simulated world.
pub trait RolloutPolicy: Clone {
    /// Choose an index into `legal` (canonical identity order, nonempty).
    fn choose(&mut self, view: &RolloutView<'_>, legal: &[DominoId]) -> usize;
}

/// The v0 continuation field: uniform random over the legal set, from a
/// seeded private tape. A placeholder field, not a modeled policy.
#[derive(Clone, Debug)]
pub struct RandomLegal(pub SplitMix64);

impl RolloutPolicy for RandomLegal {
    fn choose(&mut self, _view: &RolloutView<'_>, legal: &[DominoId]) -> usize {
        assert!(!legal.is_empty());
        self.0.below(legal.len() as u64) as usize
    }
}
