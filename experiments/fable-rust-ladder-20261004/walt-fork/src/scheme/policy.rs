use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use crate::kernel::{Hidden, Kernel};
use crate::rules::{legal_plays, Context, ContextSet, Decl, Domino, DominoSet, Seat, Team, Trick};

use super::{error, Access, Budget, CompiledFix, Fix, Frame, Registry, Result, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyProgram {
    pub name: String,
    pub initial_mode: String,
    pub bindings: Vec<RigidBinding>,
    pub exact_rules: Vec<ExactRule>,
    pub rules: Vec<PolicyRule>,
    pub fallback: Fallback,
}

#[derive(Clone, Debug)]
pub struct PolicyInput<'a> {
    pub frame: &'a Frame,
    pub history: &'a [(Seat, Domino)],
    pub banked: [u32; 2],
    pub bid: u8,
    pub declaring_team: Team,
}

impl<'a> PolicyInput<'a> {
    pub fn new(
        frame: &'a Frame,
        history: &'a [(Seat, Domino)],
        banked: [u32; 2],
        bid: u8,
        declaring_team: Team,
    ) -> Result<Self> {
        let history_set: DominoSet = history.iter().map(|(_, tile)| *tile).collect();
        if history_set.len() != history.len() || history_set != frame.played() {
            return Err(error(
                "policy history must list every played domino exactly once",
            ));
        }
        let suffix = history.len().saturating_sub(frame.prefix().len());
        if frame
            .prefix()
            .iter()
            .enumerate()
            .any(|(i, tile)| history.get(suffix + i) != Some(&(frame.leader().plus(i), *tile)))
        {
            return Err(error(
                "current trick prefix must be the suffix of policy history",
            ));
        }
        let input = Self {
            frame,
            history,
            banked,
            bid,
            declaring_team,
        };
        validate_key(&PolicyKey::from_input(&input))?;
        Ok(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyKey {
    pub decl: Decl,
    pub viewer: Seat,
    pub hand: DominoSet,
    pub pool: DominoSet,
    pub hidden: Vec<(Seat, usize, Vec<Context>)>,
    pub leader: Seat,
    pub prefix: Vec<Domino>,
    pub played: DominoSet,
    pub history: Vec<(Seat, Domino)>,
    pub banked: [u32; 2],
    pub bid: u8,
    pub declaring_team: Team,
}

impl PolicyKey {
    pub fn from_input(input: &PolicyInput<'_>) -> Self {
        let frame = input.frame;
        Self {
            decl: frame.kernel().decl(),
            viewer: frame.kernel().viewer(),
            hand: frame.kernel().viewer_hand(),
            pool: frame.kernel().pool(),
            hidden: frame
                .kernel()
                .hidden()
                .iter()
                .map(|h| (h.seat, h.capacity, h.voids.iter().collect()))
                .collect(),
            leader: frame.leader(),
            prefix: frame.prefix().to_vec(),
            played: frame.played(),
            history: input.history.to_vec(),
            banked: input.banked,
            bid: input.bid,
            declaring_team: input.declaring_team,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactRule {
    pub key: PolicyKey,
    pub action: Domino,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RigidBinding {
    pub name: String,
    pub query: Fix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyRule {
    pub name: String,
    pub in_mode: String,
    pub next_mode: String,
    pub selector: Selector,
    pub guard: Fix,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selector {
    /// Select the least legal domino in the guard's one-column answer relation.
    FirstOutput,
    Literal(Domino),
    Rigid(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallback {
    LowestLegal,
}

#[derive(Clone)]
pub struct CompiledPolicy {
    identity: Arc<str>,
    source: PolicyProgram,
    bindings: Vec<(String, CompiledFix)>,
    exact: BTreeMap<PolicyKey, Domino>,
    rules: Vec<(PolicyRule, CompiledFix)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyController {
    program_identity: Arc<str>,
    mode: String,
    rigid: BTreeMap<String, Value>,
}

/// Why an executable policy selected one action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionProvenance {
    /// The full viewer-information key was present in the frozen exact table.
    Exact,
    /// The named ordered relational rule was the first rule to select legally.
    RelationalRule { name: String },
    /// No exact key or relational rule selected, so the total fallback ran.
    Fallback,
}

/// The auditable result of one policy decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyDecisionTrace {
    pub action: Domino,
    pub provenance: DecisionProvenance,
    pub mode_before: String,
    pub mode_after: String,
    /// `Some(true)` means the declaring side has already made its contract;
    /// `Some(false)` means the remaining unbanked points cannot make it.
    pub contract_resolved: Option<bool>,
}

/// The inspectable part of controller state relevant to later decisions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyControllerState {
    pub mode: String,
    pub rigid: BTreeMap<String, Value>,
}

impl PolicyProgram {
    pub fn compile(&self, registry: &Registry) -> Result<CompiledPolicy> {
        if !super::syntax::valid_name(&self.name) || !super::syntax::valid_name(&self.initial_mode)
        {
            return Err(error("policy name and initial mode must be valid names"));
        }
        let mut names = BTreeSet::new();
        let mut bindings = Vec::new();
        for binding in &self.bindings {
            if !super::syntax::valid_name(&binding.name) || !names.insert(binding.name.clone()) {
                return Err(error(format!(
                    "invalid or duplicate rigid binding {}",
                    binding.name
                )));
            }
            let query = binding.query.compile(registry)?;
            require_viewer(&query, &format!("binding {}", binding.name))?;
            if query.outputs().len() != 1 {
                return Err(error(format!(
                    "binding {} must have one output",
                    binding.name
                )));
            }
            bindings.push((binding.name.clone(), query));
        }
        names.clear();
        let mut exact = BTreeMap::new();
        for rule in &self.exact_rules {
            let frame = validate_key(&rule.key)?;
            if frame.next_actor() != Some(rule.key.viewer)
                || !legal_plays(rule.key.decl, rule.key.hand, frame.led_context())
                    .contains(rule.action)
            {
                return Err(error(format!(
                    "exact policy action {} is illegal at its key",
                    rule.action
                )));
            }
            if exact.insert(rule.key.clone(), rule.action).is_some() {
                return Err(error("duplicate exact policy key"));
            }
        }
        let mut rules = Vec::new();
        for rule in &self.rules {
            if !super::syntax::valid_name(&rule.name)
                || !super::syntax::valid_name(&rule.in_mode)
                || !super::syntax::valid_name(&rule.next_mode)
                || !names.insert(rule.name.clone())
            {
                return Err(error(format!(
                    "invalid or duplicate policy rule {}",
                    rule.name
                )));
            }
            if let Selector::Rigid(name) = &rule.selector {
                if !self.bindings.iter().any(|binding| binding.name == *name) {
                    return Err(error(format!(
                        "rule {} names unknown binding {name}",
                        rule.name
                    )));
                }
            }
            let guard = rule.guard.compile(registry)?;
            require_viewer(&guard, &format!("rule {}", rule.name))?;
            let wanted = usize::from(matches!(rule.selector, Selector::FirstOutput));
            if guard.outputs().len() != wanted {
                return Err(error(format!(
                    "rule {} guard must have {wanted} output(s) for its selector",
                    rule.name
                )));
            }
            if wanted == 1 && guard.outputs()[0].sort != super::Sort::Domino {
                return Err(error(format!(
                    "rule {} action output must be domino",
                    rule.name
                )));
            }
            rules.push((rule.clone(), guard));
        }
        let mut identity = format!("scheme-policy-v1\n{}", self);
        for (_, query) in &bindings {
            identity.push_str("\nbinding-query: ");
            identity.push_str(&query.identity());
        }
        for (_, query) in &rules {
            identity.push_str("\nrule-query: ");
            identity.push_str(&query.identity());
        }
        Ok(CompiledPolicy {
            identity: Arc::from(identity),
            source: self.clone(),
            bindings,
            exact,
            rules,
        })
    }
}

fn require_viewer(query: &CompiledFix, owner: &str) -> Result<()> {
    if let Some(spec) = query
        .predicate_specs()
        .iter()
        .find(|spec| spec.access == Access::World)
    {
        return Err(error(format!(
            "{owner} uses World predicate {}; executable policies require Viewer access",
            spec.name
        )));
    }
    Ok(())
}

fn validate_key(key: &PolicyKey) -> Result<Frame> {
    if key.hidden.len() != 3 {
        return Err(error("exact key must describe three hidden seats"));
    }
    let mut seen_seats = BTreeSet::new();
    let mut hidden = Vec::new();
    for (seat, capacity, voids) in &key.hidden {
        if *seat == key.viewer || !seen_seats.insert(*seat) {
            return Err(error(
                "exact key hidden seats must be distinct and exclude viewer",
            ));
        }
        let void_set: ContextSet = voids.iter().copied().collect();
        if void_set.len() != voids.len() {
            return Err(error("exact key repeats a hidden void context"));
        }
        hidden.push(Hidden {
            seat: *seat,
            capacity: *capacity,
            voids: void_set,
        });
    }
    let kernel = Kernel::new(
        key.decl,
        key.viewer,
        key.hand,
        key.pool,
        hidden.try_into().expect("three hidden seats"),
    )
    .map_err(|e| error(format!("invalid exact key kernel: {e}")))?;
    if kernel.count() == 0 {
        return Err(error("exact key kernel has empty support"));
    }
    let frame = Frame::new(kernel, key.leader, key.prefix.clone(), key.played)
        .map_err(|e| error(format!("invalid exact key frame: {e}")))?;
    if frame.kernel().live().union(key.played) != DominoSet::FULL {
        return Err(error(
            "exact key live and played sets must partition the full deck",
        ));
    }

    let history_set: DominoSet = key.history.iter().map(|(_, tile)| *tile).collect();
    if history_set.len() != key.history.len() || history_set != key.played {
        return Err(error(
            "exact key history must list every played domino exactly once",
        ));
    }
    let mut remaining = [7usize; 4];
    let mut inferred_voids = [ContextSet::EMPTY; 4];
    let mut banked = [0u32; 2];
    let mut prefix = Vec::new();
    let mut leader = key.history.first().map_or(key.leader, |(seat, _)| *seat);
    let mut viewer_remaining = key.hand.union(
        key.history
            .iter()
            .filter_map(|(seat, tile)| (*seat == key.viewer).then_some(*tile))
            .collect(),
    );
    for (seat, tile) in &key.history {
        if *seat != leader.plus(prefix.len()) || remaining[seat.index()] == 0 {
            return Err(error(
                "exact key history violates turn order or seat capacity",
            ));
        }
        let led = prefix.first().map(|d| key.decl.led_context(*d));
        if *seat == key.viewer {
            if !legal_plays(key.decl, viewer_remaining, led).contains(*tile) {
                return Err(error(
                    "exact key contains an illegal historical viewer play",
                ));
            }
            viewer_remaining.remove(*tile);
        }
        if let Some(led) = led {
            if inferred_voids[seat.index()].contains(led) && key.decl.follows(*tile, led) {
                return Err(error(
                    "exact key history contradicts an earlier public void",
                ));
            }
            if !key.decl.follows(*tile, led) {
                inferred_voids[seat.index()].insert(led);
            }
        }
        remaining[seat.index()] -= 1;
        prefix.push(*tile);
        if prefix.len() == 4 {
            let trick = Trick::new(leader, prefix.clone().try_into().expect("four plays"))
                .map_err(|_| error("exact key history repeats a trick tile"))?;
            leader = trick.winner(key.decl);
            banked[leader.team().index()] += trick.points();
            prefix.clear();
        }
    }
    if leader != key.leader || prefix != key.prefix || banked != key.banked {
        return Err(error(
            "exact key leader, prefix, or banked score disagrees with history",
        ));
    }
    if remaining[key.viewer.index()] != key.hand.len() {
        return Err(error("exact key viewer hand size disagrees with history"));
    }
    for (seat, capacity, voids) in &key.hidden {
        let supplied: ContextSet = voids.iter().copied().collect();
        if remaining[seat.index()] != *capacity || inferred_voids[seat.index()] != supplied {
            return Err(error(
                "exact key hidden capacity or voids disagree with history",
            ));
        }
    }
    Ok(frame)
}

impl CompiledPolicy {
    pub fn source(&self) -> &PolicyProgram {
        &self.source
    }
    pub fn identity(&self) -> &str {
        self.identity.as_ref()
    }

    pub fn initialize(
        &self,
        input: &PolicyInput<'_>,
        budget: &mut Budget,
    ) -> Result<PolicyController> {
        let mut rigid = BTreeMap::new();
        for (name, query) in &self.bindings {
            let answers = query.evaluate_viewer(input.frame, budget)?;
            if answers.len() != 1 {
                return Err(error(format!(
                    "rigid binding {name} expected exactly one answer, got {}",
                    answers.len()
                )));
            }
            let answer = answers.iter().next().expect("one answer");
            rigid.insert(name.clone(), answer.0[0]);
        }
        Ok(PolicyController {
            program_identity: self.identity.clone(),
            mode: self.source.initial_mode.clone(),
            rigid,
        })
    }

    pub fn choose(
        &self,
        controller: &mut PolicyController,
        input: &PolicyInput<'_>,
        budget: &mut Budget,
    ) -> Result<Domino> {
        self.choose_traced(controller, input, budget)
            .map(|trace| trace.action)
    }

    /// Choose exactly as [`Self::choose`] does, while recording which policy
    /// layer answered and the controller/contract state at that decision.
    ///
    /// A refusal, including budget exhaustion partway through an ordered rule
    /// scan, restores the controller to its entry state. Work already performed
    /// remains charged to `budget`.
    pub fn choose_traced(
        &self,
        controller: &mut PolicyController,
        input: &PolicyInput<'_>,
        budget: &mut Budget,
    ) -> Result<PolicyDecisionTrace> {
        let checkpoint = controller.clone();
        let result = self.choose_traced_inner(controller, input, budget);
        if result.is_err() {
            *controller = checkpoint;
        }
        result
    }

    fn choose_traced_inner(
        &self,
        controller: &mut PolicyController,
        input: &PolicyInput<'_>,
        budget: &mut Budget,
    ) -> Result<PolicyDecisionTrace> {
        let frame = input.frame;
        if !Arc::ptr_eq(&controller.program_identity, &self.identity)
            && controller.program_identity.as_ref() != self.identity.as_ref()
        {
            return Err(error(
                "policy controller belongs to a different compiled program",
            ));
        }
        if frame.next_actor() != Some(frame.kernel().viewer()) {
            return Err(error("policy can act only when the frame viewer is next"));
        }
        let legal = legal_plays(
            frame.kernel().decl(),
            frame.kernel().viewer_hand(),
            frame.led_context(),
        );
        if legal.is_empty() {
            return Err(error("policy has no legal action"));
        }
        let mode_before = controller.mode.clone();
        let contract_resolved = contract_resolution(input);
        if let Some(tile) = self.exact.get(&PolicyKey::from_input(input)) {
            if !legal.contains(*tile) {
                return Err(error(format!("exact policy action {tile} is illegal")));
            }
            return Ok(PolicyDecisionTrace {
                action: *tile,
                provenance: DecisionProvenance::Exact,
                mode_before: mode_before.clone(),
                mode_after: mode_before,
                contract_resolved,
            });
        }
        for (rule, guard) in &self.rules {
            if rule.in_mode != controller.mode {
                continue;
            }
            let answers = guard.evaluate_viewer(frame, budget)?;
            let selected = match &rule.selector {
                Selector::FirstOutput => answers
                    .iter()
                    .filter_map(|answer| match answer.0.as_slice() {
                        [Value::Domino(tile)] if legal.contains(*tile) => Some(*tile),
                        _ => None,
                    })
                    .min(),
                Selector::Literal(tile) => {
                    (!answers.is_empty() && legal.contains(*tile)).then_some(*tile)
                }
                Selector::Rigid(name) => match controller.rigid.get(name) {
                    Some(Value::Domino(tile)) if !answers.is_empty() && legal.contains(*tile) => {
                        Some(*tile)
                    }
                    Some(Value::Domino(_)) => None,
                    Some(_) => return Err(error(format!("rigid binding {name} is not a domino"))),
                    None => return Err(error(format!("missing rigid binding {name}"))),
                },
            };
            if let Some(tile) = selected {
                controller.mode = rule.next_mode.clone();
                return Ok(PolicyDecisionTrace {
                    action: tile,
                    provenance: DecisionProvenance::RelationalRule {
                        name: rule.name.clone(),
                    },
                    mode_before,
                    mode_after: controller.mode.clone(),
                    contract_resolved,
                });
            }
        }
        let action = match self.source.fallback {
            Fallback::LowestLegal => legal
                .iter()
                .min()
                .ok_or_else(|| error("policy has no legal action")),
        }?;
        Ok(PolicyDecisionTrace {
            action,
            provenance: DecisionProvenance::Fallback,
            mode_before: mode_before.clone(),
            mode_after: mode_before,
            contract_resolved,
        })
    }
}

fn contract_resolution(input: &PolicyInput<'_>) -> Option<bool> {
    let total = input.banked[0] + input.banked[1];
    debug_assert!(total <= 42, "validated policy history conserves points");
    let declared = input.banked[input.declaring_team.index()];
    let bid = u32::from(input.bid);
    if declared >= bid {
        Some(true)
    } else if declared + (42 - total) < bid {
        Some(false)
    } else {
        None
    }
}

impl PolicyController {
    pub fn mode(&self) -> &str {
        &self.mode
    }
    pub fn rigid(&self, name: &str) -> Option<Value> {
        self.rigid.get(name).copied()
    }
    pub fn state(&self) -> PolicyControllerState {
        PolicyControllerState {
            mode: self.mode.clone(),
            rigid: self.rigid.clone(),
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Word(String),
    List(Vec<Expr>),
}
impl Expr {
    fn word(&self) -> Result<&str> {
        if let Self::Word(s) = self {
            Ok(s)
        } else {
            Err(error("expected word"))
        }
    }
    fn list(&self) -> Result<&[Expr]> {
        if let Self::List(xs) = self {
            Ok(xs)
        } else {
            Err(error("expected form"))
        }
    }
    fn tagged(&self, tag: &str) -> Result<&[Expr]> {
        let xs = self.list()?;
        if xs.first().map(Self::word).transpose()? != Some(tag) {
            return Err(error(format!("expected ({tag} ...)")));
        }
        Ok(&xs[1..])
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word(s) => f.write_str(s),
            Self::List(xs) => {
                f.write_str("(")?;
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }
                    x.fmt(f)?;
                }
                f.write_str(")")
            }
        }
    }
}
fn read(tokens: &[String], at: &mut usize, depth: usize) -> Result<Expr> {
    if depth > 40 {
        return Err(error("policy nesting exceeds 40"));
    }
    let token = tokens
        .get(*at)
        .ok_or_else(|| error("unexpected end of policy"))?;
    *at += 1;
    if token == ")" {
        return Err(error("unexpected closing parenthesis"));
    }
    if token != "(" {
        return Ok(Expr::Word(token.clone()));
    }
    let mut xs = Vec::new();
    while tokens.get(*at).map(String::as_str) != Some(")") {
        xs.push(read(tokens, at, depth + 1)?);
    }
    *at += 1;
    Ok(Expr::List(xs))
}
fn one<'a>(xs: &'a [Expr], tag: &str) -> Result<&'a Expr> {
    let bodies: Vec<_> = xs.iter().filter_map(|x| x.tagged(tag).ok()).collect();
    if bodies.len() != 1 {
        return Err(error(format!("expected exactly one ({tag} ...)")));
    }
    let body = bodies[0];
    if body.len() != 1 {
        return Err(error(format!("{tag} takes one value")));
    }
    Ok(&body[0])
}

impl FromStr for PolicyProgram {
    type Err = super::Error;
    fn from_str(source: &str) -> Result<Self> {
        if source.len() > 4_194_304 {
            return Err(error("policy exceeds 4 MiB"));
        }
        let clean = source
            .lines()
            .map(|line| line.split(';').next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");
        let tokens: Vec<_> = clean
            .replace('(', " ( ")
            .replace(')', " ) ")
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        let mut at = 0;
        let tree = read(&tokens, &mut at, 0)?;
        if at != tokens.len() {
            return Err(error("trailing forms after policy"));
        }
        let forms = tree.tagged("policy")?;
        let name = forms
            .first()
            .ok_or_else(|| error("policy requires a name"))?
            .word()?
            .to_owned();
        let mut form_counts = BTreeMap::new();
        for form in &forms[1..] {
            let fields = form.list()?;
            let tag = fields
                .first()
                .ok_or_else(|| error("empty policy form"))?
                .word()?;
            if !["initial", "fallback", "bind", "exact", "rule"].contains(&tag) {
                return Err(error(format!("unknown policy form {tag}")));
            }
            *form_counts.entry(tag).or_insert(0usize) += 1;
        }
        if form_counts.get("initial") != Some(&1) || form_counts.get("fallback") != Some(&1) {
            return Err(error(
                "policy requires exactly one initial and one fallback form",
            ));
        }
        let initial_mode = one(&forms[1..], "initial")?.word()?.to_owned();
        if one(&forms[1..], "fallback")?.word()? != "lowest-legal" {
            return Err(error("fallback must be lowest-legal"));
        }
        let mut bindings = Vec::new();
        let mut rules = Vec::new();
        for form in &forms[1..] {
            if let Ok(body) = form.tagged("bind") {
                if body.len() != 2 {
                    return Err(error("bind syntax is (bind name (fix ...))"));
                }
                bindings.push(RigidBinding {
                    name: body[0].word()?.to_owned(),
                    query: body[1].to_string().parse()?,
                });
            } else if let Ok(body) = form.tagged("rule") {
                if body.len() != 5 {
                    return Err(error("rule syntax is (rule name (in mode) (next mode) (select ...) (guard (fix ...)))"));
                }
                let name = body[0].word()?.to_owned();
                let in_mode = one(&body[1..], "in")?.word()?.to_owned();
                let next_mode = one(&body[1..], "next")?.word()?.to_owned();
                let select_forms: Vec<_> = body
                    .iter()
                    .filter_map(|x| x.tagged("select").ok())
                    .collect();
                if select_forms.len() != 1 {
                    return Err(error("rule requires exactly one select"));
                }
                let select = select_forms[0];
                let selector = match select {
                    [Expr::Word(x)] if x == "answer" => Selector::FirstOutput,
                    [Expr::Word(kind), Expr::Word(value)] if kind == "tile" => Selector::Literal(
                        value
                            .parse()
                            .map_err(|_| error("invalid selector domino"))?,
                    ),
                    [Expr::Word(kind), Expr::Word(value)] if kind == "rigid" => {
                        Selector::Rigid(value.clone())
                    }
                    _ => return Err(error("select must be answer, tile D, or rigid NAME")),
                };
                let guard = one(&body[1..], "guard")?.to_string().parse()?;
                rules.push(PolicyRule {
                    name,
                    in_mode,
                    next_mode,
                    selector,
                    guard,
                });
            }
        }
        let mut exact_rules = Vec::new();
        for form in &forms[1..] {
            if let Ok(body) = form.tagged("exact") {
                if body.len() != 2 {
                    return Err(error("exact syntax is (exact (key ...) (play D))"));
                }
                exact_rules.push(ExactRule {
                    key: parse_key(&body[0])?,
                    action: body[1]
                        .tagged("play")?
                        .first()
                        .ok_or_else(|| error("play requires domino"))?
                        .word()?
                        .parse()
                        .map_err(|_| error("invalid exact action"))?,
                });
            }
        }
        Ok(Self {
            name,
            initial_mode,
            bindings,
            exact_rules,
            rules,
            fallback: Fallback::LowestLegal,
        })
    }
}

impl fmt::Display for PolicyProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(policy {}", self.name)?;
        writeln!(f, "  (initial {})", self.initial_mode)?;
        writeln!(f, "  (fallback lowest-legal)")?;
        for binding in &self.bindings {
            writeln!(f, "  (bind {} {})", binding.name, binding.query)?;
        }
        for rule in &self.exact_rules {
            writeln!(
                f,
                "  (exact {} (play {}))",
                KeyDisplay(&rule.key),
                rule.action
            )?;
        }
        for rule in &self.rules {
            let selector = match &rule.selector {
                Selector::FirstOutput => "answer".to_owned(),
                Selector::Literal(d) => format!("tile {d}"),
                Selector::Rigid(n) => format!("rigid {n}"),
            };
            writeln!(
                f,
                "  (rule {} (in {}) (next {}) (select {}) (guard {}))",
                rule.name, rule.in_mode, rule.next_mode, selector, rule.guard
            )?;
        }
        write!(f, ")")
    }
}

fn words(xs: &[Expr], tag: &str) -> Result<Vec<String>> {
    let matches: Vec<_> = xs.iter().filter_map(|x| x.tagged(tag).ok()).collect();
    if matches.len() != 1 {
        return Err(error(format!("expected exactly one ({tag} ...)")));
    }
    matches[0]
        .iter()
        .map(|x| x.word().map(str::to_owned))
        .collect::<Result<_>>()
}
fn parse_key(expr: &Expr) -> Result<PolicyKey> {
    let xs = expr.tagged("key")?;
    let allowed = [
        "decl",
        "viewer",
        "hand-bits",
        "pool-bits",
        "hidden",
        "leader",
        "prefix",
        "played-bits",
        "history",
        "banked",
        "bid",
        "declaring-team",
    ];
    if xs.len() != allowed.len()
        || xs.iter().any(|form| {
            form.list()
                .ok()
                .and_then(|fields| fields.first())
                .and_then(|head| head.word().ok())
                .is_none_or(|tag| !allowed.contains(&tag))
        })
    {
        return Err(error("exact key has missing, duplicate, or unknown fields"));
    }
    let scalar = |tag: &str| -> Result<String> {
        let ws = words(xs, tag)?;
        if ws.len() != 1 {
            Err(error(format!("{tag} requires one value")))
        } else {
            Ok(ws[0].clone())
        }
    };
    let parse_dominoes = |tag: &str| -> Result<Vec<Domino>> {
        words(xs, tag)?
            .into_iter()
            .map(|x| {
                x.parse()
                    .map_err(|_| error(format!("invalid domino in {tag}")))
            })
            .collect()
    };
    let hand = DominoSet::from_bits(
        scalar("hand-bits")?
            .parse()
            .map_err(|_| error("invalid hand bits"))?,
    )
    .ok_or_else(|| error("hand bits outside domino universe"))?;
    let played = DominoSet::from_bits(
        scalar("played-bits")?
            .parse()
            .map_err(|_| error("invalid played bits"))?,
    )
    .ok_or_else(|| error("played bits outside domino universe"))?;
    let banked = words(xs, "banked")?;
    if banked.len() != 2 {
        return Err(error("banked requires two values"));
    }
    Ok(PolicyKey {
        decl: scalar("decl")?
            .parse()
            .map_err(|_| error("invalid declaration"))?,
        viewer: parse_seat(&scalar("viewer")?).ok_or_else(|| error("invalid viewer"))?,
        hand,
        pool: DominoSet::from_bits(
            scalar("pool-bits")?
                .parse()
                .map_err(|_| error("invalid pool bits"))?,
        )
        .ok_or_else(|| error("pool bits outside domino universe"))?,
        hidden: parse_hidden(xs)?,
        leader: parse_seat(&scalar("leader")?).ok_or_else(|| error("invalid leader"))?,
        prefix: parse_dominoes("prefix")?,
        played,
        history: parse_history(xs)?,
        banked: [
            banked[0]
                .parse()
                .map_err(|_| error("invalid banked score"))?,
            banked[1]
                .parse()
                .map_err(|_| error("invalid banked score"))?,
        ],
        bid: scalar("bid")?.parse().map_err(|_| error("invalid bid"))?,
        declaring_team: match scalar("declaring-team")?.as_str() {
            "T0" => Team::T0,
            "T1" => Team::T1,
            _ => return Err(error("invalid declaring team")),
        },
    })
}
struct KeyDisplay<'a>(&'a PolicyKey);
impl fmt::Display for KeyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let k = self.0;
        write!(
            f,
            "(key (decl {}) (viewer {}) (hand-bits {}) (pool-bits {}) (hidden",
            k.decl,
            k.viewer,
            k.hand.bits(),
            k.pool.bits(),
        )?;
        for (seat, capacity, voids) in &k.hidden {
            write!(f, " ({seat} {capacity} (voids")?;
            for q in voids {
                write!(f, " {q}")?;
            }
            write!(f, "))")?;
        }
        write!(f, ") (leader {}) (prefix", k.leader)?;
        for d in &k.prefix {
            write!(f, " {d}")?;
        }
        write!(f, ") (played-bits {}) (history", k.played.bits())?;
        for (seat, d) in &k.history {
            write!(f, " ({seat} {d})")?;
        }
        write!(
            f,
            ") (banked {} {}) (bid {}) (declaring-team {}))",
            k.banked[0], k.banked[1], k.bid, k.declaring_team
        )
    }
}

fn parse_history(xs: &[Expr]) -> Result<Vec<(Seat, Domino)>> {
    let body = xs
        .iter()
        .find_map(|x| x.tagged("history").ok())
        .ok_or_else(|| error("missing (history ...)"))?;
    body.iter()
        .map(|entry| {
            let pair = entry.list()?;
            if pair.len() != 2 {
                return Err(error("history entry must be (chair domino)"));
            }
            Ok((
                parse_seat(pair[0].word()?).ok_or_else(|| error("invalid history chair"))?,
                pair[1]
                    .word()?
                    .parse()
                    .map_err(|_| error("invalid history domino"))?,
            ))
        })
        .collect()
}

fn parse_seat(source: &str) -> Option<Seat> {
    source
        .strip_prefix('S')?
        .parse::<usize>()
        .ok()
        .and_then(Seat::from_index)
}

fn parse_hidden(xs: &[Expr]) -> Result<Vec<(Seat, usize, Vec<Context>)>> {
    let body = xs
        .iter()
        .find_map(|x| x.tagged("hidden").ok())
        .ok_or_else(|| error("missing (hidden ...)"))?;
    body.iter()
        .map(|entry| {
            let fields = entry.list()?;
            if fields.len() != 3 {
                return Err(error("hidden entry must be (chair capacity (voids ...))"));
            }
            let seat =
                parse_seat(fields[0].word()?).ok_or_else(|| error("invalid hidden chair"))?;
            let capacity = fields[1]
                .word()?
                .parse()
                .map_err(|_| error("invalid hidden capacity"))?;
            let voids = fields[2]
                .tagged("voids")?
                .iter()
                .map(|x| parse_context(x.word()?).ok_or_else(|| error("invalid void context")))
                .collect::<Result<_>>()?;
            Ok((seat, capacity, voids))
        })
        .collect()
}

fn parse_context(source: &str) -> Option<Context> {
    let rest = source.strip_prefix('q')?;
    if rest == "*" {
        Some(Context::Called)
    } else {
        rest.parse::<usize>().ok().and_then(Context::from_index)
    }
}
