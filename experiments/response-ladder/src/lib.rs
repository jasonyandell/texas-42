//! Experimental finite-bundle anytime best response. Not the production player.
pub mod mechanics;
pub mod compiled;
pub mod compiled_family;
pub mod compiled_field;
pub mod compiled_player;
pub mod core;
pub mod policy;
pub mod rollout;
#[cfg(feature = "gpu")]
pub mod gpu;
pub mod model;
pub mod player;

#[cfg(feature = "gpu")]
pub mod gpu_epochs;

pub mod trace_union;
