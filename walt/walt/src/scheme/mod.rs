//! Scheme/Fix: expressive, typed relations over exact Texas 42 worlds.
//!
//! Implements v0.4 §§3–5 as an executable query language, not a replacement
//! state or a compression claim. Imports only rules/kernel and exact arithmetic.
//! A Fix returns SETS of output bindings; hidden witnesses and overlapping
//! branches never duplicate answers or world mass. See `walt/scheme/README.md`.
//! All results are exploratory finite-domain computations.

mod belief;
mod dynamics;
mod eval;
mod policy;
mod registry;
mod syntax;

pub use belief::{
    Belief, Certainty, Comparison, Difference, QuerySummary, Selection, SelectionLaw,
};
pub use dynamics::{
    anchor_back, compare_answers, play_is_legal, step_belief, step_frame, step_world,
    transport_answers, unit_likelihood, AnswerDynamics, BeliefStep, ObservedPlay, PlayClass,
};
pub use eval::{Answer, Answers, Budget, CompiledFix, Frame};
pub use policy::{
    CompiledPolicy, ExactRule, Fallback, PolicyController, PolicyInput, PolicyKey, PolicyProgram,
    PolicyRule, RigidBinding, Selector,
};
pub use registry::{Access, Predicate, PredicateContext, PredicateSpec, Registry};
pub use syntax::{Atom, Fix, Role, Scheme, Sort, Term, Value};

/// Invalid syntax, invalid semantics, or a refused computation. No partial
/// query/summary is returned on error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

fn error(message: impl Into<String>) -> Error {
    Error(message.into())
}
