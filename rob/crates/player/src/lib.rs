//! rob-player: the evening player — fixed-field information-set best
//! response at the root (Math §11.4), Monte Carlo over the exact fiber.
//!
//! v0 scope (by design): play only; bidding is a documented placeholder;
//! belief is the exact uniform fiber (no auction-likelihood tilt yet — that
//! upgrade's regression test is the 90-world witness, not part of v0); no
//! retained-evidence record, no equilibrium machinery. The one law: one
//! common continuation policy plays all seats in every simulated world —
//! average-then-max, never max-then-average.
//!
//! rob-core stays RNG-free; every random tape lives here, seeded and
//! deterministic.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod bidding;
pub mod match_driver;
pub mod observer;
pub mod player;
pub mod policy;
pub mod rng;
pub mod rollout;
pub mod trace;
pub mod worlds;

pub use bidding::{placeholder_auction_script, placeholder_declaration};
pub use match_driver::{run_match, self_play_match, MatchTranscript, SelfPlayStats};
pub use observer::{DecisionContext, HandEndContext, HandStartContext, MatchObserver};
pub use player::{viewer_legal, DecisionReport, MonteCarloPlayer, UtilityLens};
pub use policy::{RandomLegal, RolloutPolicy, RolloutView};
pub use rng::{SeededExactSource, SplitMix64};
pub use rollout::{finish_hand, RolloutPosition};
pub use worlds::sample_worlds;
