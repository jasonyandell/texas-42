//! Scalar execution of the frozen fourteen-clause relational actor.
//!
//! This is deliberately a small adapter around the existing Scheme policy
//! language.  The scalar predicates below are the direct current-state
//! Viewer semantics of `walt::scheme::Registry::standard`; the exported
//! [`PolicyProgram`] is the oracle representation of the same actor.

use serde::{Deserialize, Serialize};
use walt::rules::{Context, Decl, Domino, DominoSet};
use walt::scheme::{
    Atom, DecisionProvenance, Fallback, Fix, PolicyProgram, PolicyRule, Role, Scheme, Selector,
    Sort, Term, Value,
};

use crate::mechanics::PublicState;

/// The exact grammar and predicate semantics used by the frozen reference
/// learner.  This is part of the serialized actor identity.
pub const SCHEMA: &str = "scheme-relational-actor-v1";
pub const SCHEMA_V2: &str = "scheme-relational-actor-v2";
pub const SCHEMA_LEAD_FOLLOW: &str = "scheme-relational-lead-follow-v1";
pub const GRAMMAR_VERSION: &str = "scheme-relational-actor-v1/grammar-v1/straight-v0.4";
pub const GRAMMAR_VERSION_V2: &str = "scheme-relational-actor-v2/grammar-v2/straight-v0.4";
pub const CLAUSE_COUNT: usize = 14;
pub const CLAUSE_COUNT_V2: usize = 16;

/// Clause order is part of the v1 wire contract.  The odd entries qualify the
/// corresponding selector by `partner(viewer, current-winner)`.
pub const CLAUSE_NAMES: [&str; CLAUSE_COUNT] = [
    "legal",
    "partner-winning-legal",
    "count-0",
    "partner-winning-count-0",
    "count-5",
    "partner-winning-count-5",
    "count-10",
    "partner-winning-count-10",
    "master",
    "partner-winning-master",
    "follow-led",
    "partner-winning-follow-led",
    "boss-led",
    "partner-winning-boss-led",
];

pub const CLAUSE_NAMES_V2: [&str; CLAUSE_COUNT_V2] = [
    "legal",
    "partner-winning-legal",
    "count-0",
    "partner-winning-count-0",
    "count-5",
    "partner-winning-count-5",
    "count-10",
    "partner-winning-count-10",
    "master",
    "partner-winning-master",
    "follow-led",
    "partner-winning-follow-led",
    "boss-led",
    "partner-winning-boss-led",
    "beat-current-winner",
    "partner-winning-beat-current-winner",
];

/// A compact shared actor.  It is stateless: clauses are inspected in this
/// order and the first clause with an output selects the least legal tile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    pub schema: String,
    /// Following program, or the entire program for a legacy actor.
    pub clauses: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lead_clauses: Option<Vec<usize>>,
}

/// The scalar choice and the same provenance vocabulary used by Scheme.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    pub action: u8,
    pub provenance: DecisionProvenance,
}

impl Actor {
    /// Construct and validate an actor from its wire representation.
    pub fn new(schema: impl Into<String>, clauses: Vec<usize>) -> Result<Self, String> {
        let actor = Self {
            schema: schema.into(),
            clauses,
            lead_clauses: None,
        };
        actor.validate()?;
        Ok(actor)
    }

    /// The empty actor is the lawful lowest-legal baseline.
    pub fn empty() -> Self {
        Self {
            schema: SCHEMA.to_owned(),
            clauses: Vec::new(),
            lead_clauses: None,
        }
    }

    pub fn lead_follow(lead: Vec<usize>, follow: Vec<usize>) -> Result<Self, String> {
        let actor = Self { schema: SCHEMA_LEAD_FOLLOW.into(), clauses: follow,
            lead_clauses: Some(lead) };
        actor.validate()?;
        Ok(actor)
    }

    /// The public actor is on turn, so an empty current trick means leading.
    /// Callers validate the immutable artifact before taking this fast path.
    pub fn clauses_for_phase(&self, leading: bool) -> &[usize] {
        if leading && self.schema == SCHEMA_LEAD_FOLLOW {
            self.lead_clauses.as_deref().unwrap_or(&[])
        } else { &self.clauses }
    }

    pub fn validate(&self) -> Result<(), String> {
        let clause_count = match self.schema.as_str() {
            SCHEMA => CLAUSE_COUNT,
            SCHEMA_V2 | SCHEMA_LEAD_FOLLOW => CLAUSE_COUNT_V2,
            _ => 0,
        };
        if clause_count == 0 {
            return Err(format!(
                "unsupported compiled actor schema {:?}", self.schema
            ));
        }
        if (self.schema == SCHEMA_LEAD_FOLLOW) != self.lead_clauses.is_some() {
            return Err("lead_clauses is required only for a lead/follow actor".into());
        }
        for clauses in std::iter::once(&self.clauses).chain(self.lead_clauses.iter()) {
            if clauses.len() > 3 {
                return Err("compiled actor phase may contain at most three clauses".to_owned());
            }
            for (position, &clause) in clauses.iter().enumerate() {
                if clause >= clause_count {
                    return Err(format!("compiled actor clause {clause} at position {position} is out of range"));
                }
                if clauses[..position].contains(&clause) {
                    return Err(format!("compiled actor repeats clause {clause}"));
                }
            }
        }
        Ok(())
    }

    /// Execute this actor on lawful own-hand/public input.
    pub fn choose(
        &self,
        decl: Decl,
        viewer: u8,
        hand: u32,
        public: &PublicState,
    ) -> Result<Choice, String> {
        self.validate()?;
        let feature_schema = if self.schema == SCHEMA_LEAD_FOLLOW { SCHEMA_V2 } else { &self.schema };
        let actions = clause_actions_for_schema(feature_schema, decl, viewer, hand, public)?;
        let leading = public.plays.is_empty();
        for (position, &clause) in self.clauses_for_phase(leading).iter().enumerate() {
            if let Some(action) = actions[clause] {
                return Ok(Choice {
                    action,
                    provenance: DecisionProvenance::RelationalRule {
                        name: self.rule_name(leading, position, clause),
                    },
                });
            }
        }
        let legal = public.legal(decl, hand);
        let action = legal
            .trailing_zeros()
            .try_into()
            .map_err(|_| "legal action does not fit in a tile index".to_owned())?;
        Ok(Choice {
            action,
            provenance: DecisionProvenance::Fallback,
        })
    }

    /// Action-only evaluator for the already validated `CompiledField` path.
    /// It intentionally does not construct provenance or revalidate the full
    /// public history.  Invalid internal inputs return `None` rather than
    /// reaching a domino-index assertion.
    pub(crate) fn action_unchecked(
        &self,
        decl: Decl,
        viewer: u8,
        hand: u32,
        public: &PublicState,
    ) -> Option<u8> {
        if viewer >= 4
            || public.leader >= 4
            || public.plays.len() > 3
            || public.played & !walt::solver::FULL_MASK != 0
            || hand == 0
            || hand & !walt::solver::FULL_MASK != 0
            || hand & public.played != 0
            || public.actor() != viewer
        {
            return None;
        }
        let mut partial = 0u32;
        for &tile in &public.plays {
            if tile >= Domino::COUNT as u8 {
                return None;
            }
            let bit = 1u32.checked_shl(u32::from(tile))?;
            if partial & bit != 0 || public.played & bit == 0 {
                return None;
            }
            partial |= bit;
        }
        let legal = public.legal(decl, hand);
        if legal == 0 {
            return None;
        }
        let live = DominoSet::from_bits(walt::solver::FULL_MASK ^ public.played)?;
        let partner_winning = partner_currently_winning(decl, viewer, public);
        let led = public
            .plays
            .first()
            .copied()
            .map(|tile| decl.led_context(domino(tile)));
        let current_winner = current_winning_tile(decl, public);
        let clause_count = match self.schema.as_str() {
            SCHEMA => CLAUSE_COUNT,
            SCHEMA_V2 | SCHEMA_LEAD_FOLLOW => CLAUSE_COUNT_V2,
            _ => return None,
        };
        if self.schema == SCHEMA_LEAD_FOLLOW && self.lead_clauses.is_none() { return None; }
        for &clause in self.clauses_for_phase(public.plays.is_empty()) {
            if clause >= clause_count {
                return None;
            }
            if let Some(action) = clause_action_unchecked(
                decl,
                legal,
                live,
                partner_winning,
                led,
                current_winner,
                clause,
            ) {
                return Some(action);
            }
        }
        Some(legal.trailing_zeros() as u8)
    }

    /// Export the actor through the existing Scheme policy language.
    pub fn policy_program(&self) -> Result<PolicyProgram, String> {
        self.policy_program_named(if self.schema == SCHEMA {
            "compiled-actor-v1"
        } else if self.schema == SCHEMA_LEAD_FOLLOW {
            "compiled-actor-lead-follow-v1"
        } else {
            "compiled-actor-v2"
        })
    }

    /// Alias kept explicit for callers that treat this as a compiler step.
    pub fn to_policy_program(&self) -> Result<PolicyProgram, String> {
        self.policy_program()
    }

    pub fn policy_program_named(&self, name: &str) -> Result<PolicyProgram, String> {
        self.validate()?;
        if !valid_policy_name(name) {
            return Err(format!("invalid Scheme policy name {name:?}"));
        }
        let phases: &[bool] = if self.schema == SCHEMA_LEAD_FOLLOW { &[true, false] } else { &[false] };
        let mut rules = Vec::new();
        for &leading in phases {
            for (position, &clause) in self.clauses_for_phase(leading).iter().enumerate() {
                let mut guard = if self.schema == SCHEMA { clause_guard(clause) } else { clause_guard_v2(clause) };
                if self.schema == SCHEMA_LEAD_FOLLOW {
                    if !guard.roles.iter().any(|r| r.name == "viewer") {
                        guard.roles.push(role("viewer", Sort::Chair));
                    }
                    for case in &mut guard.cases {
                        if !case.atoms.iter().any(|a| a.predicate == "viewer" && a.args == vec![var("viewer")]) {
                            case.atoms.push(atom("viewer", vec![var("viewer")]));
                        }
                        // On-turn viewer equals the current leader exactly at
                        // an empty trick. Negation is on an already bound chair.
                        let mut phase = atom("leader", vec![var("viewer")]);
                        phase.negated = !leading;
                        case.atoms.push(phase);
                    }
                }
                rules.push(PolicyRule { name: self.rule_name(leading, position, clause),
                    in_mode: "shared".into(), next_mode: "shared".into(),
                    selector: Selector::FirstOutput, guard });
            }
        }
        Ok(PolicyProgram {
            name: name.to_owned(),
            initial_mode: "shared".to_owned(),
            bindings: Vec::new(),
            exact_rules: Vec::new(),
            rules,
            fallback: Fallback::LowestLegal,
        })
    }

    fn rule_name(&self, leading: bool, position: usize, clause: usize) -> String {
        let name = rule_name_for_schema(&self.schema, position, clause);
        if self.schema == SCHEMA_LEAD_FOLLOW {
            format!("{}-{name}", if leading { "lead" } else { "follow" })
        } else { name }
    }
}

/// Evaluate all fourteen clauses.  Each slot is the exact output that Scheme
/// would expose for the corresponding standalone `FirstOutput` rule.
pub fn clause_actions(
    decl: Decl,
    viewer: u8,
    hand: u32,
    public: &PublicState,
) -> Result<[Option<u8>; CLAUSE_COUNT], String> {
    validate_input(decl, viewer, hand, public)?;
    let legal = public.legal(decl, hand);
    let live = DominoSet::from_bits(walt::solver::FULL_MASK ^ public.played)
        .ok_or_else(|| "public played mask contains a tile outside the deck".to_owned())?;
    let partner_winning = partner_currently_winning(decl, viewer, public);
    let current_winner = current_winning_tile(decl, public);
    let led = public
        .plays
        .first()
        .copied()
        .map(|tile| decl.led_context(domino(tile)));

    let mut out = [None; CLAUSE_COUNT];
    for clause in 0..CLAUSE_COUNT {
        out[clause] = clause_action_unchecked(
            decl,
            legal,
            live,
            partner_winning,
            led,
            current_winner,
            clause,
        );
    }
    Ok(out)
}

/// Evaluate the v2 sixteen-slot grammar. The first fourteen slots retain the
/// v1 meanings and IDs; slots 14/15 are beat-current-winner, with slot 15
/// qualified by partner-currently-winning.
pub fn clause_actions_v2(
    decl: Decl,
    viewer: u8,
    hand: u32,
    public: &PublicState,
) -> Result<[Option<u8>; CLAUSE_COUNT_V2], String> {
    validate_input(decl, viewer, hand, public)?;
    let legal = public.legal(decl, hand);
    let live = DominoSet::from_bits(walt::solver::FULL_MASK ^ public.played)
        .ok_or_else(|| "public played mask contains a tile outside the deck".to_owned())?;
    let partner_winning = partner_currently_winning(decl, viewer, public);
    let led = public
        .plays
        .first()
        .copied()
        .map(|tile| decl.led_context(domino(tile)));
    let current_winner = current_winning_tile(decl, public);
    let mut out = [None; CLAUSE_COUNT_V2];
    for clause in 0..CLAUSE_COUNT_V2 {
        out[clause] = clause_action_unchecked(
            decl,
            legal,
            live,
            partner_winning,
            led,
            current_winner,
            clause,
        );
    }
    Ok(out)
}

/// Schema-dispatched clause output for callers that load serialized actors.
/// The v1 array remains available through [`clause_actions`] unchanged.
pub fn clause_actions_for_schema(
    schema: &str,
    decl: Decl,
    viewer: u8,
    hand: u32,
    public: &PublicState,
) -> Result<Vec<Option<u8>>, String> {
    match schema {
        SCHEMA => Ok(clause_actions(decl, viewer, hand, public)?.to_vec()),
        SCHEMA_V2 => Ok(clause_actions_v2(decl, viewer, hand, public)?.to_vec()),
        _ => Err(format!("unsupported compiled actor schema {schema:?}")),
    }
}

fn clause_action_unchecked(
    decl: Decl,
    legal: u32,
    live: DominoSet,
    partner_winning: Option<bool>,
    led: Option<Context>,
    current_winner: Option<(Domino, Context)>,
    clause: usize,
) -> Option<u8> {
    if clause >= CLAUSE_COUNT_V2 || (clause % 2 == 1 && partner_winning != Some(true)) {
        return None;
    }
    let selector = clause / 2;
    least_matching(legal, |tile| {
        let d = domino(tile);
        match selector {
            0 => true,
            1 => d.count() == 0,
            2 => d.count() == 5,
            3 => d.count() == 10,
            4 => live.contains(d) && decl.threat(d).intersection(live).is_empty(),
            5 => led.is_some_and(|q| decl.follows(d, q)),
            6 => led.is_some_and(|q| {
                live.contains(d)
                    && decl.follows(d, q)
                    && decl.beats(q, d).intersection(live).is_empty()
            }),
            7 => current_winner
                .is_some_and(|(winner, q)| decl.trick_key(d, q) > decl.trick_key(winner, q)),
            _ => unreachable!("sixteen clauses are eight selector pairs"),
        }
    })
}

fn least_matching<F>(legal: u32, mut predicate: F) -> Option<u8>
where
    F: FnMut(u8) -> bool,
{
    let mut remaining = legal;
    while remaining != 0 {
        let tile = remaining.trailing_zeros() as u8;
        remaining &= remaining - 1;
        if predicate(tile) {
            return Some(tile);
        }
    }
    None
}

fn validate_input(_decl: Decl, viewer: u8, hand: u32, public: &PublicState) -> Result<(), String> {
    if viewer >= 4 {
        return Err("viewer must be one of seats 0..=3".to_owned());
    }
    if public.leader >= 4 || public.plays.len() > 3 {
        return Err("public leader/current trick is invalid".to_owned());
    }
    if public.played & !walt::solver::FULL_MASK != 0 || hand & !walt::solver::FULL_MASK != 0 {
        return Err("hand/public state contains a tile outside the deck".to_owned());
    }
    if hand == 0 {
        return Err("actor hand must be nonempty".to_owned());
    }
    if hand & public.played != 0 {
        return Err("actor hand overlaps public played tiles".to_owned());
    }
    if public.actor() != viewer {
        return Err(format!(
            "public actor is {}, but viewer is {viewer}",
            public.actor()
        ));
    }
    if public.history.len() != public.played.count_ones() as usize {
        return Err("public history length disagrees with played mask".to_owned());
    }
    let mut history_mask = 0u32;
    for &tile in &public.history {
        if tile >= Domino::COUNT as u8 {
            return Err("public history contains an invalid tile".to_owned());
        }
        let bit = 1u32 << tile;
        if history_mask & bit != 0 {
            return Err("public history repeats a tile".to_owned());
        }
        history_mask |= bit;
    }
    if history_mask != public.played {
        return Err("public history and played mask disagree".to_owned());
    }
    if public.history.len() < public.plays.len()
        || public.history[public.history.len() - public.plays.len()..] != public.plays
    {
        return Err("current trick must be the suffix of public history".to_owned());
    }
    if u16::from(public.banked_t0) + u16::from(public.banked_t1) > 42 {
        return Err("banked score exceeds the 42-point hand".to_owned());
    }
    Ok(())
}

fn partner_currently_winning(decl: Decl, viewer: u8, public: &PublicState) -> Option<bool> {
    let first = public.plays.first().copied()?;
    let led = decl.led_context(domino(first));
    let winner_offset = public
        .plays
        .iter()
        .enumerate()
        .max_by_key(|(_, &tile)| decl.trick_key(domino(tile), led))
        .map(|(offset, _)| offset as u8)?;
    let winner = (public.leader + winner_offset) % 4;
    Some(winner == (viewer + 2) % 4)
}

fn current_winning_tile(decl: Decl, public: &PublicState) -> Option<(Domino, Context)> {
    let first = public.plays.first().copied()?;
    let led = decl.led_context(domino(first));
    let tile = public
        .plays
        .iter()
        .copied()
        .max_by_key(|&tile| decl.trick_key(domino(tile), led))?;
    Some((domino(tile), led))
}

fn domino(tile: u8) -> Domino {
    Domino::from_index(tile as usize).expect("validated tile index")
}

fn rule_name_for_schema(schema: &str, position: usize, clause: usize) -> String {
    let names = if schema == SCHEMA {
        &CLAUSE_NAMES[..]
    } else {
        &CLAUSE_NAMES_V2[..]
    };
    format!("r{position}-{}", names[clause])
}

fn valid_policy_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
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

/// Build the same guard as `policy_search::relational::relational_grammar`.
fn clause_guard(clause: usize) -> Fix {
    let selector = clause / 2;
    let qualified = clause % 2 == 1;
    let mut roles = vec![role("action", Sort::Domino)];
    let mut atoms = vec![atom("own-legal", vec![var("action")])];
    match selector {
        0 => {}
        1..=3 => atoms.push(atom(
            "count",
            vec![
                var("action"),
                Term::Literal(Value::Number([0, 5, 10][selector - 1])),
            ],
        )),
        4 => atoms.push(atom("master", vec![var("action")])),
        5 => {
            roles.push(role("led", Sort::Context));
            atoms.push(atom("led-context", vec![var("led")]));
            atoms.push(atom("in", vec![var("action"), var("led")]));
        }
        6 => {
            roles.push(role("led", Sort::Context));
            atoms.push(atom("led-context", vec![var("led")]));
            atoms.push(atom("boss", vec![var("action"), var("led")]));
        }
        _ => unreachable!("fourteen clauses are seven selector pairs"),
    }
    if qualified {
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

fn clause_guard_v2(clause: usize) -> Fix {
    if clause < CLAUSE_COUNT {
        return clause_guard(clause);
    }
    let qualified = clause % 2 == 1;
    let mut roles = vec![
        role("action", Sort::Domino),
        role("led", Sort::Context),
        role("winner", Sort::Chair),
        role("winning_tile", Sort::Domino),
    ];
    let mut atoms = vec![
        atom("own-legal", vec![var("action")]),
        atom("led-context", vec![var("led")]),
        atom("current-winner", vec![var("winner")]),
        atom("trick-play", vec![var("winner"), var("winning_tile")]),
        atom(
            "beats",
            vec![var("action"), var("winning_tile"), var("led")],
        ),
    ];
    if clause == 15 && qualified {
        roles.push(role("viewer", Sort::Chair));
        atoms.push(atom("viewer", vec![var("viewer")]));
        atoms.push(atom("partner", vec![var("viewer"), var("winner")]));
    } else if clause != 14 {
        unreachable!("v2 clause must be beat-current-winner pair");
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
