//! Bounded cost-sensitive construction of one shared relational actor.
//!
//! This is the deliberately small Slice B/C constructor.  It owns a frozen
//! `Registry::standard`, searches only current-state Viewer predicates, emits
//! one-mode `FirstOutput` rules, and never emits bindings or exact rules.
//! Clause names describe mechanical relations; they are not tactical claims.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::{legal_plays, Domino, Seat, Team};
use crate::scheme::{
    Access, Atom, Budget, CompiledFix, Fallback, Fix, Frame, PolicyInput, PolicyKey, PolicyProgram,
    PolicyRule, Registry, Role, Scheme, Selector, Sort, Term, Value,
};

use super::program_digest;

/// Identifies both the grammar and the exact standard-predicate semantics it
/// was built against.  Change this when either changes.
pub const RELATIONAL_GRAMMAR_VERSION: &str = "scheme-relational-actor-v1/grammar-v1/straight-v0.4";

const MODE: &str = "shared";

/// The action relation returned by a clause before optional qualification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectorPrimitive {
    Legal,
    Count(u32),
    Master,
    FollowLed,
    BossLed,
}

/// A lawful current-trick qualification.  Undefined current-winner facts do
/// not match, so `PartnerCurrentlyWinning` cannot fire on an empty trick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Qualification {
    Any,
    PartnerCurrentlyWinning,
}

/// One member of the fixed, finite clause library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClauseTemplate {
    pub id: String,
    pub selector: SelectorPrimitive,
    pub qualification: Qualification,
    pub guard: Fix,
    pub ast_nodes: usize,
}

/// Return the complete ordered library searched by this version.
pub fn relational_grammar() -> Vec<ClauseTemplate> {
    let selectors = [
        SelectorPrimitive::Legal,
        SelectorPrimitive::Count(0),
        SelectorPrimitive::Count(5),
        SelectorPrimitive::Count(10),
        SelectorPrimitive::Master,
        SelectorPrimitive::FollowLed,
        SelectorPrimitive::BossLed,
    ];
    let qualifications = [Qualification::Any, Qualification::PartnerCurrentlyWinning];
    let mut clauses = Vec::new();
    for selector in selectors {
        for qualification in qualifications {
            let id = clause_id(selector, qualification);
            let guard = clause_guard(selector, qualification);
            let ast_nodes = ast_nodes(&guard);
            clauses.push(ClauseTemplate {
                id,
                selector,
                qualification,
                guard,
                ast_nodes,
            });
        }
    }
    clauses
}

fn clause_id(selector: SelectorPrimitive, qualification: Qualification) -> String {
    let base = match selector {
        SelectorPrimitive::Legal => "legal".to_owned(),
        SelectorPrimitive::Count(n) => format!("count-{n}"),
        SelectorPrimitive::Master => "master".to_owned(),
        SelectorPrimitive::FollowLed => "follow-led".to_owned(),
        SelectorPrimitive::BossLed => "boss-led".to_owned(),
    };
    match qualification {
        Qualification::Any => base,
        Qualification::PartnerCurrentlyWinning => format!("partner-winning-{base}"),
    }
}

fn role(name: &str, sort: Sort) -> Role {
    Role {
        name: name.to_owned(),
        sort,
    }
}

fn var(name: &str) -> Term {
    Term::Role(name.to_owned())
}

fn atom(name: &str, args: Vec<Term>) -> Atom {
    Atom {
        predicate: name.to_owned(),
        args,
        negated: false,
    }
}

fn clause_guard(selector: SelectorPrimitive, qualification: Qualification) -> Fix {
    let mut roles = vec![role("action", Sort::Domino)];
    let mut atoms = vec![atom("own-legal", vec![var("action")])];
    match selector {
        SelectorPrimitive::Legal => {}
        SelectorPrimitive::Count(n) => atoms.push(atom(
            "count",
            vec![var("action"), Term::Literal(Value::Number(n))],
        )),
        SelectorPrimitive::Master => atoms.push(atom("master", vec![var("action")])),
        SelectorPrimitive::FollowLed => {
            roles.push(role("led", Sort::Context));
            atoms.push(atom("led-context", vec![var("led")]));
            atoms.push(atom("in", vec![var("action"), var("led")]));
        }
        SelectorPrimitive::BossLed => {
            roles.push(role("led", Sort::Context));
            atoms.push(atom("led-context", vec![var("led")]));
            atoms.push(atom("boss", vec![var("action"), var("led")]));
        }
    }
    if qualification == Qualification::PartnerCurrentlyWinning {
        roles.push(role("viewer", Sort::Chair));
        roles.push(role("winner", Sort::Chair));
        atoms.push(atom("viewer", vec![var("viewer")]));
        atoms.push(atom("current-winner", vec![var("winner")]));
        atoms.push(atom("partner", vec![var("viewer"), var("winner")]));
    }
    Fix {
        roles,
        outputs: vec!["action".to_owned()],
        cases: vec![Scheme {
            equal: vec![],
            atoms,
        }],
    }
}

/// Structural accounting used by the constructor's explicit AST cap.
/// Nodes are: the Fix root, role declarations, output references, cases,
/// atoms, equality groups/references, and atom terms.
fn ast_nodes(fix: &Fix) -> usize {
    1 + fix.roles.len()
        + fix.outputs.len()
        + fix
            .cases
            .iter()
            .map(|case| {
                1 + case.atoms.len()
                    + case.atoms.iter().map(|atom| atom.args.len()).sum::<usize>()
                    + case.equal.len()
                    + case.equal.iter().map(Vec::len).sum::<usize>()
            })
            .sum::<usize>()
}

/// One cost-sensitive row.  Its private fields preserve the validation done
/// by `new`; the cache key is the complete owned policy observation, including
/// full actor-attributed history, score, contract and declaring side.
#[derive(Clone, Debug)]
pub struct DecisionExample {
    frame: Frame,
    history: Vec<(Seat, Domino)>,
    banked: [u32; 2],
    bid: u8,
    declaring_team: Team,
    costs: BTreeMap<Domino, BigRational>,
    group_weight: BigRational,
    key: PolicyKey,
}

impl DecisionExample {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        frame: Frame,
        history: Vec<(Seat, Domino)>,
        banked: [u32; 2],
        bid: u8,
        declaring_team: Team,
        costs: BTreeMap<Domino, BigRational>,
        group_weight: BigRational,
    ) -> Result<Self, String> {
        let input = PolicyInput::new(&frame, &history, banked, bid, declaring_team)
            .map_err(|e| format!("invalid relational example observation: {e}"))?;
        if frame.next_actor() != Some(frame.kernel().viewer()) {
            return Err("relational example viewer must be the next actor".to_owned());
        }
        let legal = legal_plays(
            frame.kernel().decl(),
            frame.kernel().viewer_hand(),
            frame.led_context(),
        );
        let supplied: BTreeSet<_> = costs.keys().copied().collect();
        let expected: BTreeSet<_> = legal.iter().collect();
        if supplied != expected {
            return Err(
                "relational example costs must cover every legal action exactly once".to_owned(),
            );
        }
        if costs
            .values()
            .any(|cost| cost < &BigRational::zero() || cost > &BigRational::one())
        {
            return Err("relational example costs must lie in [0,1]".to_owned());
        }
        if group_weight < BigRational::zero() {
            return Err("relational example group weight must be nonnegative".to_owned());
        }
        let key = PolicyKey::from_input(&input);
        Ok(Self {
            frame,
            history,
            banked,
            bid,
            declaring_team,
            costs,
            group_weight,
            key,
        })
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }
    pub fn history(&self) -> &[(Seat, Domino)] {
        &self.history
    }
    pub fn banked(&self) -> [u32; 2] {
        self.banked
    }
    pub fn bid(&self) -> u8 {
        self.bid
    }
    pub fn declaring_team(&self) -> Team {
        self.declaring_team
    }
    pub fn costs(&self) -> &BTreeMap<Domino, BigRational> {
        &self.costs
    }
    pub fn group_weight(&self) -> &BigRational {
        &self.group_weight
    }

    fn input(&self) -> Result<PolicyInput<'_>, String> {
        PolicyInput::new(
            &self.frame,
            &self.history,
            self.banked,
            self.bid,
            self.declaring_team,
        )
        .map_err(|e| e.to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelationalLimits {
    pub max_clauses: usize,
    pub max_ast_nodes: usize,
    pub beam_width: usize,
    /// Unique ordered programs whose empirical cost may be evaluated.
    pub max_search_work: u64,
    /// Scheme work available for all distinct clause evaluations at one
    /// complete observation during one fit.
    pub max_inference_work_per_example: u64,
}

impl Default for RelationalLimits {
    fn default() -> Self {
        Self {
            max_clauses: 4,
            max_ast_nodes: 96,
            beam_width: 32,
            max_search_work: 20_000,
            max_inference_work_per_example: 100_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationalCandidate {
    pub program: PolicyProgram,
    pub digest: String,
    pub weighted_cost: BigRational,
    pub clause_count: usize,
    pub ast_nodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LearningMetrics {
    pub grammar_version: String,
    pub examples: usize,
    pub library_clauses: usize,
    pub programs_scored: u64,
    pub neighbor_proposals: u64,
    pub search_work: u64,
    pub clause_cache_evaluations: u64,
    pub clause_cache_hits: u64,
    pub inference_work: u64,
    pub budget_refusals: u64,
    pub search_cap_hit: bool,
    pub total_group_weight: BigRational,
    pub selected_weighted_cost: BigRational,
    pub selected_clause_count: usize,
    pub selected_ast_nodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationalFit {
    pub program: PolicyProgram,
    pub digest: String,
    /// Deterministically ordered beam finalists for independent development
    /// rollout selection.  The first is the training-surrogate winner.
    pub candidates: Vec<RelationalCandidate>,
    pub metrics: LearningMetrics,
}

struct LibraryClause {
    template: ClauseTemplate,
    compiled: CompiledFix,
}

/// The frozen constructor.  It accepts no caller-supplied predicates.
pub struct RelationalLearner {
    limits: RelationalLimits,
    library: Vec<LibraryClause>,
}

impl RelationalLearner {
    pub fn new(limits: RelationalLimits) -> Result<Self, String> {
        if limits.max_clauses == 0
            || limits.max_ast_nodes == 0
            || limits.beam_width == 0
            || limits.max_search_work == 0
            || limits.max_inference_work_per_example == 0
        {
            return Err("all relational learner limits must be positive".to_owned());
        }
        let registry = Registry::standard();
        let mut library = Vec::new();
        for template in relational_grammar() {
            let compiled = template
                .guard
                .compile(&registry)
                .map_err(|e| format!("invalid frozen relational clause {}: {e}", template.id))?;
            if compiled.outputs().len() != 1 || compiled.outputs()[0].sort != Sort::Domino {
                return Err(format!(
                    "frozen relational clause {} must return one domino",
                    template.id
                ));
            }
            if let Some(spec) = compiled
                .predicate_specs()
                .iter()
                .find(|spec| access_is_not_current_viewer(spec.access, spec.horizon_plies))
            {
                return Err(format!(
                    "frozen relational clause {} uses non-current Viewer predicate {}",
                    template.id, spec.name
                ));
            }
            library.push(LibraryClause { template, compiled });
        }
        Ok(Self { limits, library })
    }

    pub fn limits(&self) -> RelationalLimits {
        self.limits
    }

    pub fn fit(&self, examples: &[DecisionExample], name: &str) -> Result<RelationalFit, String> {
        if examples.is_empty() {
            return Err("relational fitting requires at least one decision example".to_owned());
        }
        if !valid_policy_name(name) {
            return Err("relational policy name must be a Scheme identifier".to_owned());
        }

        let mut context = FitContext::new(self, examples);
        let empty = Vec::<usize>::new();
        let empty_cost = context.score(&empty)?;
        let mut scores = BTreeMap::from([(empty.clone(), empty_cost)]);
        let mut beam = vec![empty];
        let mut cap_hit = false;

        loop {
            let mut proposals = BTreeSet::new();
            for program in &beam {
                propose_neighbors(
                    program,
                    &self.library,
                    self.limits,
                    &mut proposals,
                    &mut context.neighbor_proposals,
                );
            }
            let unseen: Vec<_> = proposals
                .into_iter()
                .filter(|program| !scores.contains_key(program))
                .collect();
            if unseen.is_empty() {
                break;
            }
            let mut evaluated_any = false;
            for program in unseen {
                if context.search_work >= self.limits.max_search_work {
                    cap_hit = true;
                    break;
                }
                let cost = context.score(&program)?;
                scores.insert(program, cost);
                evaluated_any = true;
            }
            let mut ranked: Vec<_> = scores.keys().cloned().collect();
            ranked.sort_by(|a, b| compare_programs(a, b, &scores, &self.library));
            ranked.truncate(self.limits.beam_width);
            let changed = ranked != beam;
            beam = ranked;
            if cap_hit || !evaluated_any || !changed {
                break;
            }
        }

        beam.sort_by(|a, b| compare_programs(a, b, &scores, &self.library));
        let mut candidates: Vec<_> = beam
            .iter()
            .map(|ids| {
                candidate(
                    name,
                    ids,
                    scores.get(ids).expect("beam programs are scored").clone(),
                    &self.library,
                )
            })
            .collect();
        if !beam.iter().any(Vec::is_empty) {
            candidates.push(candidate(
                name,
                &[],
                scores
                    .get(&Vec::new())
                    .expect("the fallback baseline is always scored")
                    .clone(),
                &self.library,
            ));
        }
        let selected = candidates
            .first()
            .cloned()
            .expect("the empty program remains in a nonempty beam");
        let total_group_weight = examples.iter().map(|x| x.group_weight.clone()).sum();
        let metrics = LearningMetrics {
            grammar_version: RELATIONAL_GRAMMAR_VERSION.to_owned(),
            examples: examples.len(),
            library_clauses: self.library.len(),
            programs_scored: scores.len() as u64,
            neighbor_proposals: context.neighbor_proposals,
            search_work: context.search_work,
            clause_cache_evaluations: context.clause_cache_evaluations,
            clause_cache_hits: context.clause_cache_hits,
            inference_work: context.inference_work,
            budget_refusals: context.budget_refusals,
            search_cap_hit: cap_hit,
            total_group_weight,
            selected_weighted_cost: selected.weighted_cost.clone(),
            selected_clause_count: selected.clause_count,
            selected_ast_nodes: selected.ast_nodes,
        };
        Ok(RelationalFit {
            program: selected.program,
            digest: selected.digest,
            candidates,
            metrics,
        })
    }
}

fn access_is_not_current_viewer(access: Access, horizon_plies: u32) -> bool {
    access != Access::Viewer || horizon_plies != 0
}

fn valid_policy_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

struct FitContext<'a> {
    learner: &'a RelationalLearner,
    examples: &'a [DecisionExample],
    outputs: BTreeMap<(usize, PolicyKey), Option<Domino>>,
    observation_budgets: BTreeMap<PolicyKey, Budget>,
    search_work: u64,
    neighbor_proposals: u64,
    clause_cache_evaluations: u64,
    clause_cache_hits: u64,
    inference_work: u64,
    budget_refusals: u64,
}

impl<'a> FitContext<'a> {
    fn new(learner: &'a RelationalLearner, examples: &'a [DecisionExample]) -> Self {
        let observation_budgets = examples
            .iter()
            .map(|example| {
                (
                    example.key.clone(),
                    Budget::new(learner.limits.max_inference_work_per_example),
                )
            })
            .collect();
        Self {
            learner,
            examples,
            outputs: BTreeMap::new(),
            observation_budgets,
            search_work: 0,
            neighbor_proposals: 0,
            clause_cache_evaluations: 0,
            clause_cache_hits: 0,
            inference_work: 0,
            budget_refusals: 0,
        }
    }

    fn output(&mut self, clause: usize, example: usize) -> Result<Option<Domino>, String> {
        let row = &self.examples[example];
        let cache_key = (clause, row.key.clone());
        if let Some(output) = self.outputs.get(&cache_key) {
            self.clause_cache_hits += 1;
            return Ok(*output);
        }
        let budget = self
            .observation_budgets
            .get_mut(&row.key)
            .expect("every example observation has a budget");
        let before = budget.spent();
        let answers = match self.learner.library[clause]
            .compiled
            .evaluate_viewer(&row.frame, budget)
        {
            Ok(answers) => answers,
            Err(error) => {
                self.inference_work += budget.spent() - before;
                self.budget_refusals += 1;
                return Err(format!(
                    "relational clause inference refused at {}: {error}",
                    self.learner.library[clause].template.id
                ));
            }
        };
        self.inference_work += budget.spent() - before;
        self.clause_cache_evaluations += 1;
        let legal = legal_plays(
            row.frame.kernel().decl(),
            row.frame.kernel().viewer_hand(),
            row.frame.led_context(),
        );
        let selected = answers
            .iter()
            .filter_map(|answer| match answer.0.as_slice() {
                [Value::Domino(tile)] if legal.contains(*tile) => Some(*tile),
                _ => None,
            })
            .min();
        self.outputs.insert(cache_key, selected);
        Ok(selected)
    }

    fn score(&mut self, program: &[usize]) -> Result<BigRational, String> {
        if self.search_work >= self.learner.limits.max_search_work {
            return Err("internal relational search work overrun".to_owned());
        }
        self.search_work += 1;
        let mut total = BigRational::zero();
        for example_index in 0..self.examples.len() {
            let row = &self.examples[example_index];
            let fallback = row
                .costs
                .keys()
                .next()
                .copied()
                .expect("validated examples have a legal action");
            let mut action = fallback;
            for clause in program {
                if let Some(selected) = self.output(*clause, example_index)? {
                    action = selected;
                    break;
                }
            }
            total += &row.group_weight * &row.costs[&action];
        }
        Ok(total)
    }
}

fn propose_neighbors(
    program: &[usize],
    library: &[LibraryClause],
    limits: RelationalLimits,
    out: &mut BTreeSet<Vec<usize>>,
    proposals: &mut u64,
) {
    if program.len() < limits.max_clauses {
        for clause in 0..library.len() {
            if program.contains(&clause) {
                continue;
            }
            for at in 0..=program.len() {
                let mut next = program.to_vec();
                next.insert(at, clause);
                *proposals += 1;
                if program_ast_nodes(&next, library) <= limits.max_ast_nodes {
                    out.insert(next);
                }
            }
        }
    }
    for at in 0..program.len() {
        let mut next = program.to_vec();
        next.remove(at);
        *proposals += 1;
        out.insert(next);
    }
    for from in 0..program.len() {
        for to in 0..program.len() {
            if from == to {
                continue;
            }
            let mut next = program.to_vec();
            let clause = next.remove(from);
            next.insert(to, clause);
            *proposals += 1;
            out.insert(next);
        }
    }
}

fn program_ast_nodes(program: &[usize], library: &[LibraryClause]) -> usize {
    1 + program
        .iter()
        .map(|id| library[*id].template.ast_nodes)
        .sum::<usize>()
}

fn compare_programs(
    a: &[usize],
    b: &[usize],
    scores: &BTreeMap<Vec<usize>, BigRational>,
    library: &[LibraryClause],
) -> Ordering {
    scores[a]
        .cmp(&scores[b])
        .then_with(|| a.len().cmp(&b.len()))
        .then_with(|| program_ast_nodes(a, library).cmp(&program_ast_nodes(b, library)))
        .then_with(|| a.cmp(b))
}

fn policy(name: &str, clauses: &[usize], library: &[LibraryClause]) -> PolicyProgram {
    PolicyProgram {
        name: name.to_owned(),
        initial_mode: MODE.to_owned(),
        bindings: vec![],
        exact_rules: vec![],
        rules: clauses
            .iter()
            .enumerate()
            .map(|(position, clause)| PolicyRule {
                name: format!("r{position}-{}", library[*clause].template.id),
                in_mode: MODE.to_owned(),
                next_mode: MODE.to_owned(),
                selector: Selector::FirstOutput,
                guard: library[*clause].template.guard.clone(),
            })
            .collect(),
        fallback: Fallback::LowestLegal,
    }
}

fn candidate(
    name: &str,
    clauses: &[usize],
    weighted_cost: BigRational,
    library: &[LibraryClause],
) -> RelationalCandidate {
    let program = policy(name, clauses, library);
    let digest = program_digest(&program.to_string());
    RelationalCandidate {
        program,
        digest,
        weighted_cost,
        clause_count: clauses.len(),
        ast_nodes: program_ast_nodes(clauses, library),
    }
}

/// Audit a program against held-out cost rows using the executable policy
/// runtime and a fresh frozen standard registry.  Development promotion still
/// belongs to complete whole-policy rollouts, not this row surrogate.
pub fn evaluate_program_cost(
    program: &PolicyProgram,
    examples: &[DecisionExample],
    max_inference_work_per_example: u64,
) -> Result<BigRational, String> {
    if max_inference_work_per_example == 0 {
        return Err("relational audit inference limit must be positive".to_owned());
    }
    if !program.bindings.is_empty()
        || !program.exact_rules.is_empty()
        || program.initial_mode != MODE
        || program.rules.iter().any(|rule| {
            rule.in_mode != MODE
                || rule.next_mode != MODE
                || !matches!(rule.selector, Selector::FirstOutput)
        })
    {
        return Err("relational audit accepts only one-mode shared FirstOutput programs without bindings or exact rules".to_owned());
    }
    let compiled = program
        .compile(&Registry::standard())
        .map_err(|e| format!("relational audit program did not compile: {e}"))?;
    let mut total = BigRational::zero();
    for example in examples {
        let input = example.input()?;
        let mut budget = Budget::new(max_inference_work_per_example);
        let mut controller = compiled
            .initialize(&input, &mut budget)
            .map_err(|e| e.to_string())?;
        let action = compiled
            .choose(&mut controller, &input, &mut budget)
            .map_err(|e| e.to_string())?;
        let cost = example.costs.get(&action).ok_or_else(|| {
            format!("relational audit selected action {action} without a supplied cost")
        })?;
        total += &example.group_weight * cost;
    }
    Ok(total)
}
