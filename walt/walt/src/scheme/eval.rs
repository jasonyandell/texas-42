use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::kernel::{Kernel, World};
use crate::rules::{Context, Domino, DominoSet, Seat};

use super::registry::{Access, Predicate, PredicateContext, PredicateSpec, Registry};
use super::syntax::{valid_name, Fix, Role, Sort, Term, Value};
use super::{error, Result};

/// An explicit finite budget shared across a whole query/summary. No clocks.
#[derive(Clone, Debug)]
pub struct Budget {
    limit: u64,
    spent: u64,
}
impl Budget {
    pub fn new(limit: u64) -> Self {
        Self { limit, spent: 0 }
    }
    pub fn spent(&self) -> u64 {
        self.spent
    }
    pub fn remaining(&self) -> u64 {
        self.limit - self.spent
    }
    pub fn spend(&mut self, n: u64) -> Result<()> {
        if n > self.remaining() {
            return Err(error(format!(
                "Scheme work budget exhausted ({}/{})",
                self.spent, self.limit
            )));
        }
        self.spent += n;
        Ok(())
    }
}

/// Exact mechanical support plus explicit public residue. This constructor
/// checks coherence, not reachability from a complete deal. Played means
/// explicitly supplied history, never automatically the complement of Live.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    kernel: Kernel,
    leader: Seat,
    prefix: Vec<Domino>,
    played: DominoSet,
}
impl Frame {
    pub fn new(
        kernel: Kernel,
        leader: Seat,
        prefix: Vec<Domino>,
        played: DominoSet,
    ) -> Result<Self> {
        let unique: DominoSet = prefix.iter().copied().collect();
        if prefix.len() > 3
            || unique.len() != prefix.len()
            || !unique.is_subset_of(played)
            || !played.is_disjoint(kernel.live())
        {
            return Err(error("incoherent public residue: prefix must be unique, length <=3, and played must exclude live tiles"));
        }
        Ok(Self {
            kernel,
            leader,
            prefix,
            played,
        })
    }
    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }
    pub fn leader(&self) -> Seat {
        self.leader
    }
    pub fn prefix(&self) -> &[Domino] {
        &self.prefix
    }
    pub fn played(&self) -> DominoSet {
        self.played
    }
    pub fn led_context(&self) -> Option<Context> {
        self.prefix
            .first()
            .map(|d| self.kernel.decl().led_context(*d))
    }
    pub fn capacity(&self, seat: Seat) -> usize {
        if seat == self.kernel.viewer() {
            self.kernel.viewer_hand().len()
        } else {
            self.kernel
                .hidden()
                .iter()
                .find(|h| h.seat == seat)
                .expect("all seats represented")
                .capacity
        }
    }
    pub fn next_actor(&self) -> Option<Seat> {
        let seat = self.leader.plus(self.prefix.len());
        (self.capacity(seat) > 0).then_some(seat)
    }
    pub fn current_winner(&self) -> Option<Seat> {
        let q = self.led_context()?;
        self.prefix
            .iter()
            .enumerate()
            .max_by_key(|(_, d)| self.kernel.decl().trick_key(**d, q))
            .map(|(i, _)| self.leader.plus(i))
    }
}

/// Values in the query's declared output order, never existential bindings.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Answer(pub Vec<Value>);
pub type Answers = BTreeSet<Answer>;

#[derive(Clone)]
enum BoundTerm {
    Variable(usize),
    Literal(Value),
}
#[derive(Clone)]
struct CompiledAtom {
    predicate: Arc<dyn Predicate>,
    spec: PredicateSpec,
    terms: Vec<BoundTerm>,
    negated: bool,
}
#[derive(Clone)]
struct Case {
    sorts: Vec<Sort>,
    outputs: Vec<usize>,
    atoms: Vec<CompiledAtom>,
}
#[derive(Clone)]
pub struct CompiledFix {
    source: Fix,
    outputs: Vec<Role>,
    cases: Vec<Case>,
    specs: Vec<PredicateSpec>,
}

impl Fix {
    pub fn compile(&self, registry: &Registry) -> Result<CompiledFix> {
        if self.roles.len() > 64 {
            return Err(error("at most 64 roles are supported"));
        }
        let mut names = BTreeMap::new();
        for (i, role) in self.roles.iter().enumerate() {
            if !valid_name(&role.name)
                || Value::literal(&role.name).is_some()
                || !role.sort.is_role()
            {
                return Err(error(format!("invalid role {}:{}", role.name, role.sort)));
            }
            if names.insert(role.name.clone(), i).is_some() {
                return Err(error(format!("duplicate role {}", role.name)));
            }
        }
        let resolve = |name: &str| {
            names
                .get(name)
                .copied()
                .ok_or_else(|| error(format!("undeclared role {name}")))
        };
        let mut outputs = Vec::new();
        let mut output_set = BTreeSet::new();
        for name in &self.outputs {
            let i = resolve(name)?;
            if !output_set.insert(i) {
                return Err(error(format!("duplicate output {name}")));
            }
            outputs.push(i);
        }
        let mut cases = Vec::new();
        let mut used = BTreeMap::new();
        for scheme in &self.cases {
            let mut classes: Vec<usize> = (0..self.roles.len()).collect();
            for group in &scheme.equal {
                if group.len() < 2 {
                    return Err(error("same requires at least two names"));
                }
                let first = resolve(&group[0])?;
                for name in &group[1..] {
                    let next = resolve(name)?;
                    if self.roles[first].sort != self.roles[next].sort {
                        return Err(error("equality classes cannot mix role sorts"));
                    }
                    let from = classes[next];
                    let to = classes[first];
                    for c in &mut classes {
                        if *c == from {
                            *c = to;
                        }
                    }
                }
            }
            let mut dense = BTreeMap::new();
            let mut sorts = Vec::new();
            for (i, class) in classes.iter_mut().enumerate() {
                let next = dense.len();
                let idx = *dense.entry(*class).or_insert_with(|| {
                    sorts.push(self.roles[i].sort);
                    next
                });
                *class = idx;
            }
            let mut atoms = Vec::new();
            for atom in &scheme.atoms {
                let predicate = registry
                    .predicates
                    .get(&atom.predicate)
                    .ok_or_else(|| error(format!("unregistered predicate {}", atom.predicate)))?
                    .clone();
                let spec = predicate.spec().clone();
                if spec.parameters.len() != atom.args.len() {
                    return Err(error(format!(
                        "{} expects {} arguments, got {}",
                        spec.name,
                        spec.parameters.len(),
                        atom.args.len()
                    )));
                }
                let mut terms = Vec::new();
                for (arg, expected) in atom.args.iter().zip(&spec.parameters) {
                    let (term, actual) = match arg {
                        Term::Role(name) => {
                            let i = resolve(name)?;
                            (BoundTerm::Variable(classes[i]), self.roles[i].sort)
                        }
                        Term::Literal(value) => (BoundTerm::Literal(*value), value.sort()),
                    };
                    if actual != *expected {
                        return Err(error(format!(
                            "{} expects {expected}, got {actual}",
                            spec.name
                        )));
                    }
                    terms.push(term);
                }
                used.insert(spec.name.clone(), spec.clone());
                atoms.push(CompiledAtom {
                    predicate,
                    spec,
                    terms,
                    negated: atom.negated,
                });
            }
            cases.push(Case {
                sorts,
                outputs: outputs.iter().map(|i| classes[*i]).collect(),
                atoms,
            });
        }
        Ok(CompiledFix {
            source: self.clone(),
            outputs: outputs.iter().map(|i| self.roles[*i].clone()).collect(),
            cases,
            specs: used.into_values().collect(),
        })
    }
}

impl CompiledAtom {
    fn ready(&self, values: &[Option<Value>]) -> bool {
        self.terms.iter().all(|t| match t {
            BoundTerm::Literal(_) => true,
            BoundTerm::Variable(i) => values[*i].is_some(),
        })
    }
    fn check(
        &self,
        frame: &Frame,
        world: &World,
        values: &[Option<Value>],
        budget: &mut Budget,
    ) -> Result<bool> {
        budget.spend(1)?;
        let args: Vec<_> = self
            .terms
            .iter()
            .map(|t| match t {
                BoundTerm::Variable(i) => values[*i].expect("ready atom"),
                BoundTerm::Literal(v) => *v,
            })
            .collect();
        let context = match self.spec.access {
            Access::Viewer => PredicateContext::Viewer(frame),
            Access::World => PredicateContext::World(frame, world),
        };
        Ok(self
            .predicate
            .evaluate(context, &args, budget)?
            .is_some_and(|value| value != self.negated))
    }
}

impl CompiledFix {
    pub fn source(&self) -> &Fix {
        &self.source
    }
    pub fn outputs(&self) -> &[Role] {
        &self.outputs
    }
    pub fn predicate_specs(&self) -> &[PredicateSpec] {
        &self.specs
    }
    /// A stable, complete textual identity, not a hash or a universal semantic
    /// normal form. Includes the syntax version and every used predicate spec.
    pub fn identity(&self) -> String {
        format!(
            "scheme-expression-v1\n{}\nregistry: {:?}",
            self.source, self.specs
        )
    }
    pub fn evaluate(&self, frame: &Frame, world: &World, budget: &mut Budget) -> Result<Answers> {
        budget.spend(1)?;
        if !frame.kernel.contains(world) {
            return Err(error("world is outside this frame's support"));
        }
        let mut answers = Answers::new();
        for case in &self.cases {
            case.evaluate(frame, world, budget, &mut answers)?;
        }
        Ok(answers)
    }
}

impl Case {
    fn evaluate(
        &self,
        frame: &Frame,
        world: &World,
        budget: &mut Budget,
        answers: &mut Answers,
    ) -> Result<()> {
        let mut values = vec![None; self.sorts.len()];
        // Constant atoms are checked once. Unary filters shrink candidate domains
        // before joining, then all remaining atoms are checked as soon as ready.
        let mut constants = Vec::new();
        let mut unary = vec![Vec::new(); self.sorts.len()];
        let mut remaining = Vec::new();
        for (i, atom) in self.atoms.iter().enumerate() {
            let vars: BTreeSet<_> = atom
                .terms
                .iter()
                .filter_map(|t| match t {
                    BoundTerm::Variable(i) => Some(*i),
                    _ => None,
                })
                .collect();
            match vars.len() {
                0 => constants.push(i),
                1 => unary[*vars.first().expect("one variable")].push(i),
                _ => remaining.push(i),
            }
        }
        for i in constants {
            if !self.atoms[i].check(frame, world, &values, budget)? {
                return Ok(());
            }
        }
        let mut domains = Vec::new();
        for (i, sort) in self.sorts.iter().enumerate() {
            let mut domain = Vec::new();
            for candidate in sort.values() {
                budget.spend(1)?;
                values[i] = Some(candidate);
                let mut valid = true;
                for a in &unary[i] {
                    if !self.atoms[*a].check(frame, world, &values, budget)? {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    domain.push(candidate);
                }
            }
            values[i] = None;
            if domain.is_empty() {
                return Ok(());
            }
            domains.push(domain);
        }
        let mut order: Vec<usize> = (0..self.sorts.len()).collect();
        order.sort_by_key(|i| (domains[*i].len(), *i));
        Search {
            case: self,
            frame,
            world,
            budget,
            answers,
            domains: &domains,
            order: &order,
        }
        .walk(0, &mut values, &remaining)
    }
}

struct Search<'a> {
    case: &'a Case,
    frame: &'a Frame,
    world: &'a World,
    budget: &'a mut Budget,
    answers: &'a mut Answers,
    domains: &'a [Vec<Value>],
    order: &'a [usize],
}
impl Search<'_> {
    fn walk(
        &mut self,
        depth: usize,
        values: &mut [Option<Value>],
        pending: &[usize],
    ) -> Result<()> {
        self.budget.spend(1)?;
        // Once a projected answer has a witness, additional witnesses for that
        // same answer cannot change the relation, even across Fix branches.
        let projected: Option<Vec<Value>> = self.case.outputs.iter().map(|i| values[*i]).collect();
        if let Some(row) = projected {
            if self.answers.contains(&Answer(row)) {
                return Ok(());
            }
        }
        let mut rest = Vec::new();
        for i in pending {
            let atom = &self.case.atoms[*i];
            if atom.ready(values) {
                if !atom.check(self.frame, self.world, values, self.budget)? {
                    return Ok(());
                }
            } else {
                rest.push(*i);
            }
        }
        if depth == self.order.len() {
            self.answers.insert(Answer(
                self.case
                    .outputs
                    .iter()
                    .map(|i| values[*i].expect("complete binding"))
                    .collect(),
            ));
            return Ok(());
        }
        let i = self.order[depth];
        for candidate in &self.domains[i] {
            // Equality has already quotiented names; distinct same-sort classes
            // are injective. Value's type tag keeps the three sorts separate.
            if values.iter().flatten().any(|v| v == candidate) {
                continue;
            }
            values[i] = Some(*candidate);
            self.walk(depth + 1, values, &rest)?;
            values[i] = None;
        }
        Ok(())
    }
}
