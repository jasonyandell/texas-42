//! walt-skeleton -- the dynamic control skeleton layer, S4 slice.
//!
//! PLAN.md's crate map places THE deliverable here: the `ControlSkeleton`
//! trait (typed relational descriptor state + closed update `d' = step(d,
//! obs)`, v0.4 §12.5), the static soundness checker (§12.1 factorization
//! `R* = R-bar o D` on a finite root domain), and the exhaustive controlled
//! lumpability checker (§12.6 on finite kernels: legal-set agreement and
//! (feature-increment, observation, successor-class) kernel agreement within
//! descriptor classes). The day-one passenger is a static descriptor wrapped
//! as a degenerate-update transducer and *marked* static (`UpdateKind`).
//!
//! Everything here is exploratory tier. Checker verdicts are computed
//! evidence about one finite kernel under the fixed uniform-legal field --
//! never axioms, never promoted statuses (TRUST-01). A descriptor is a
//! transducer, not a labeling: the trait's `step` signature makes a
//! non-closed update unconstructible, so "dynamic-from-the-jump" is a type
//! discipline, not a promise.
//!
//! Imports core, kernel, geom, and strat -- the strict import direction of
//! §16.2. Scheme/Fix as a descriptor query language (§12.7) arrives later
//! inside this crate's vocabulary; it will import this physics, never the
//! reverse.

pub mod atoms;
pub mod lumpability;
pub mod obs;
pub mod skeleton;
pub mod soundness;
pub mod synth;

pub use atoms::{Atom, AtomDescriptor, AtomState, ChassisState, CompositeState};
pub use lumpability::{
    check_lumpability, KernelTree, LumpabilityFailure, LumpabilityReport, Node, Row,
};
pub use obs::{record_plays, ObservedPlay};
pub use skeleton::{fold_record, ControlSkeleton, StaticWrap, UpdateKind};
pub use soundness::{check_soundness, PurityCounterexample, SoundnessReport};
pub use synth::{class_ids, registry, sound_search, MinimalSound, SoundSearch};
