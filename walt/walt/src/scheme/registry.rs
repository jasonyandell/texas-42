use std::collections::BTreeMap;
use std::sync::Arc;

use crate::kernel::World;
use crate::rules::rules::legal_plays;
use crate::rules::{DominoSet, Seat};

use super::{error, Budget, Frame, Result, Sort, Value};

/// Available information, not a claim that every returned binding is known.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Access {
    /// Own hand and the kernel/public residue; no realized hidden holdings.
    Viewer,
    /// Offline examiner: a concrete hidden assignment is available.
    World,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredicateSpec {
    pub name: String,
    /// Explicit semantic version. Changing a computation changes this value.
    pub version: String,
    pub parameters: Vec<Sort>,
    /// Zero for current-state relations; positive for bounded continuations.
    pub horizon_plies: u32,
    pub access: Access,
}

#[derive(Clone, Copy)]
pub enum PredicateContext<'a> {
    Viewer(&'a Frame),
    World(&'a Frame, &'a World),
}
impl<'a> PredicateContext<'a> {
    pub fn frame(self) -> &'a Frame {
        match self {
            Self::Viewer(f) | Self::World(f, _) => f,
        }
    }
    pub fn world(self) -> Option<&'a World> {
        match self {
            Self::Viewer(_) => None,
            Self::World(_, w) => Some(w),
        }
    }
}

/// A trusted registered computation. It must be deterministic, respect its
/// declared horizon, charge inner work, and never consult a target solver,
/// response labels, or undeclared captured state. The runtime type-checks args
/// and withholds World from Viewer predicates. It cannot audit closure captures.
pub trait Predicate: Send + Sync {
    fn spec(&self) -> &PredicateSpec;
    /// None means undefined: neither this atom nor its negation holds.
    fn evaluate(
        &self,
        context: PredicateContext<'_>,
        args: &[Value],
        budget: &mut Budget,
    ) -> Result<Option<bool>>;
}

#[derive(Clone, Default)]
pub struct Registry {
    pub(crate) predicates: BTreeMap<String, Arc<dyn Predicate>>,
}
impl Registry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, predicate: impl Predicate + 'static) -> Result<()> {
        let spec = predicate.spec();
        if !super::syntax::valid_name(&spec.name)
            || ["same", "not", "fix", "roles", "out", "case"].contains(&spec.name.as_str())
            || spec.version.trim().is_empty()
        {
            return Err(error(
                "predicate requires a nonreserved name and semantic version",
            ));
        }
        if self.predicates.contains_key(&spec.name) {
            return Err(error(format!("predicate {} already registered", spec.name)));
        }
        self.predicates
            .insert(spec.name.clone(), Arc::new(predicate));
        Ok(())
    }
    pub fn specs(&self) -> Vec<PredicateSpec> {
        self.predicates.values().map(|p| p.spec().clone()).collect()
    }
    pub fn standard() -> Self {
        use Access::{Viewer, World};
        use Sort::{Chair as C, Context as Q, Domino as D, Number as N, Team as T};
        let mut registry = Self::new();
        for (name, parameters, access) in [
            ("live", vec![D], Viewer),
            ("holds", vec![C, D], World),
            ("in", vec![D, Q], Viewer),
            ("double", vec![D], Viewer),
            ("beats", vec![D, D, Q], Viewer),
            ("boss", vec![D, Q], Viewer),
            ("master", vec![D], Viewer),
            ("void", vec![C, Q], World),
            ("quota", vec![C, N], Viewer),
            ("tile", vec![D, D], Viewer),
            ("chair", vec![C, C], Viewer),
            ("context", vec![Q, Q], Viewer),
            ("team", vec![C, T], Viewer),
            ("partner", vec![C, C], Viewer),
            ("opponent", vec![C, C], Viewer),
            ("successor", vec![C, C], Viewer),
            ("viewer", vec![C], Viewer),
            ("leader", vec![C], Viewer),
            ("next-actor", vec![C], Viewer),
            ("led-context", vec![Q], Viewer),
            ("current-winner", vec![C], Viewer),
            ("played", vec![D], Viewer),
            ("count", vec![D, N], Viewer),
            ("legal", vec![C, D], World),
            ("forced", vec![C, D], World),
        ] {
            registry
                .register(Builtin(PredicateSpec {
                    name: name.to_owned(),
                    version: "scheme-v1/straight-v0.4".to_owned(),
                    parameters,
                    horizon_plies: 0,
                    access,
                }))
                .expect("standard registry names are unique and valid");
        }
        registry
    }
}

struct Builtin(PredicateSpec);
impl Predicate for Builtin {
    fn spec(&self) -> &PredicateSpec {
        &self.0
    }
    fn evaluate(
        &self,
        context: PredicateContext<'_>,
        args: &[Value],
        _: &mut Budget,
    ) -> Result<Option<bool>> {
        use Value::{Chair as C, Context as Q, Domino as D, Number as N, Team as T};
        let frame = context.frame();
        let kernel = frame.kernel();
        let decl = kernel.decl();
        let hand = |seat: Seat| -> Result<DominoSet> {
            context
                .world()
                .map(|w| w.hand(seat))
                .ok_or_else(|| error("world predicate requires a world"))
        };
        let value = match (self.0.name.as_str(), args) {
            ("live", [D(d)]) => kernel.live().contains(*d),
            ("holds", [C(c), D(d)]) => hand(*c)?.contains(*d),
            ("in", [D(d), Q(q)]) => decl.follows(*d, *q),
            ("double", [D(d)]) => d.is_double(),
            ("beats", [D(a), D(b), Q(q)]) => decl.trick_key(*a, *q) > decl.trick_key(*b, *q),
            ("boss", [D(d), Q(q)]) => {
                kernel.live().contains(*d)
                    && decl.follows(*d, *q)
                    && decl.beats(*q, *d).intersection(kernel.live()).is_empty()
            }
            ("master", [D(d)]) => {
                kernel.live().contains(*d) && decl.threat(*d).intersection(kernel.live()).is_empty()
            }
            ("void", [C(c), Q(q)]) => hand(*c)?
                .intersection(decl.effective_incidence(*q))
                .is_empty(),
            ("quota", [C(c), N(n)]) => frame.capacity(*c) == *n as usize,
            ("tile", [D(a), D(b)]) => a == b,
            ("chair", [C(a), C(b)]) => a == b,
            ("context", [Q(a), Q(b)]) => a == b,
            ("team", [C(c), T(t)]) => c.team() == *t,
            ("partner", [C(a), C(b)]) => a.plus(2) == *b,
            ("opponent", [C(a), C(b)]) => a.team() != b.team(),
            ("successor", [C(a), C(b)]) => a.successor() == *b,
            ("viewer", [C(c)]) => *c == kernel.viewer(),
            ("leader", [C(c)]) => *c == frame.leader(),
            ("next-actor", [C(c)]) => frame.next_actor() == Some(*c),
            ("led-context", [Q(q)]) => match frame.led_context() {
                Some(led) => *q == led,
                None => return Ok(None),
            },
            ("current-winner", [C(c)]) => match frame.current_winner() {
                Some(winner) => *c == winner,
                None => return Ok(None),
            },
            ("played", [D(d)]) => frame.played().contains(*d),
            ("count", [D(d), N(n)]) => d.count() == *n,
            ("legal" | "forced", [C(c), D(d)]) => {
                if frame.next_actor() != Some(*c) {
                    return Ok(None);
                }
                let legal = legal_plays(decl, hand(*c)?, frame.led_context());
                if self.0.name == "forced" {
                    legal == DominoSet::single(*d)
                } else {
                    legal.contains(*d)
                }
            }
            _ => return Err(error("builtin argument mismatch (invalid compiled query)")),
        };
        Ok(Some(value))
    }
}
