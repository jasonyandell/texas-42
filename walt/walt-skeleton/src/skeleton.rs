//! The `ControlSkeleton` trait (v0.4 §12.5) and the static-passenger
//! wrapper.
//!
//! A dynamic control skeleton is a typed relational state `D_t` with update
//! `D_{t+1} = delta_D(D_t, a_t, o_{t+1})` (§12.5). Here both the seat's own
//! action and the field's replies arrive as `ObservedPlay`s, so the update
//! law is one function `step(d, obs)` folded over the observation record --
//! walt-strat's record already interleaves them in exactly that order.
//!
//! Two disciplines are carried by the types, not by promises:
//!
//! - **Closure.** `step` sees the descriptor state and one observed play,
//!   nothing else. A "recompute from the latent world" update is
//!   unconstructible behind this signature; the only legal degenerate form
//!   is the constant update, and that form is *marked* (`UpdateKind`).
//! - **Latent reads happen once.** `init` evaluates the descriptor on a
//!   latent root state (kernel + fiber world) -- the God's-eye read the
//!   checkers need. Everything after the root flows through `step`.
//!
//! Soundness (§12.1) and lumpability (§12.6) are then checkable properties
//! of an implementation: `soundness::check_soundness` and
//! `lumpability::check_lumpability` take any `ControlSkeleton` and return
//! exhaustively computed verdicts on a finite kernel.

use walt_kernel::{Kernel, World};

use crate::obs::{record_plays, ObservedPlay};

/// How an implementation's update law relates to time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdateKind {
    /// A genuine transducer: `step` computes the successor state from the
    /// current state and the observed play alone, and the state is meant to
    /// stay meaningful as the hand advances.
    Transducer,
    /// A marked static passenger: the state is frozen at its root
    /// evaluation (constant update). Legal, but graded as such -- the
    /// search objective prefers closed updates (PLAN.md,
    /// "Dynamic-from-the-jump").
    StaticPassenger,
}

/// A descriptor as a transducer: typed relational state, one latent root
/// evaluation, a closed update law.
pub trait ControlSkeleton {
    /// The typed relational descriptor state. Equality and ordering are the
    /// class structure the checkers quotient by, so `State` must carry no
    /// identity beyond its projected content.
    type State: Clone + Ord + core::fmt::Debug;

    /// A human-readable name for reports and pins.
    fn name(&self) -> String;

    /// Whether the update law is a genuine transducer or a marked static
    /// passenger.
    fn kind(&self) -> UpdateKind;

    /// Evaluates the descriptor on one latent root state: the kernel (the
    /// public view) and one fiber world (the latent completion). The only
    /// latent read in the interface.
    fn init(&self, kernel: &Kernel, world: &World) -> Self::State;

    /// The closed update law `d' = step(d, obs)`.
    fn step(&self, d: &Self::State, obs: ObservedPlay) -> Self::State;
}

/// The descriptor state at a decision node: the root evaluation folded
/// through the node's observation record (viewer plays and field plays
/// alike, in record order).
pub fn fold_record<S: ControlSkeleton>(
    skeleton: &S,
    kernel: &Kernel,
    world: &World,
    record: &[walt_core::Domino],
) -> S::State {
    let mut d = skeleton.init(kernel, world);
    for play in record_plays(kernel.decl(), kernel.viewer(), record) {
        d = skeleton.step(&d, play);
    }
    d
}

/// The degenerate-update wrapper: freezes any descriptor at its root
/// evaluation. This is the day-one passenger form -- a §14.4-style static
/// descriptor riding the dynamic harness, explicitly marked static.
pub struct StaticWrap<S>(pub S);

impl<S: ControlSkeleton> ControlSkeleton for StaticWrap<S> {
    type State = S::State;

    fn name(&self) -> String {
        format!("static[{}]", self.0.name())
    }

    fn kind(&self) -> UpdateKind {
        UpdateKind::StaticPassenger
    }

    fn init(&self, kernel: &Kernel, world: &World) -> Self::State {
        self.0.init(kernel, world)
    }

    fn step(&self, d: &Self::State, _obs: ObservedPlay) -> Self::State {
        d.clone()
    }
}
