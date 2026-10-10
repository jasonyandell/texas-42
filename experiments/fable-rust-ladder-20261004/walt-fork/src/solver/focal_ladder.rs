//! The focal-horizon ladder, slice FH2 — the parent's §23 sound
//! interruption, §19 preserved facts, §24 gap measurements and §25
//! continuation substitution (`walt/math/focal_horizon_sandwich_v0.1.md`,
//! cited by title; the object is the FOCAL-HORIZON HIERARCHY, FH-A2) as
//! narrowed by the companion's proofs P8, P11, P12 and rulings FH-A3,
//! FH-A9, FH-A11 with **Proposition FH-int** (`walt/CENSUS-RULINGS.md`).
//!
//! THE OBJECT. A [`FocalLadder`] is an append-only store of NODE FACTS
//! over one root under one identity (FH1's five coordinates minus the
//! horizon, [`LadderIdentity`]). A node fact ([`NodeFact`]) is
//! `[L(C), U(C)]` in mass form with THE POLICY THAT ATTAINS `L(C)` stored
//! beside it (a `FocalChoices`-shaped sub-table rooted at `C`; the tail
//! everywhere else — every lower fact carries its witness, FH-A9) and the
//! residual horizon each side was established at. A node never priced
//! holds NO upper (the trivial 1 is a placeholder, never a fact — FH-A3)
//! and the lower `0` with policy = the tail; such a node has no entry in
//! the store at all.
//!
//! INSTALLATION IS INTERSECTION, NEVER REPLACEMENT (FH-int). A completed
//! node's new fact meets its prior: lower = max, keeping the winning
//! policy (the PRIOR wins ties — a policy is replaced only by a strict
//! improvement, which is what makes the suffix memo invisible in the
//! choice tables); upper = min. Nothing partial is ever installed: a
//! node's fact is written only when its whole subtree completed at its
//! residual horizon, so the fact set is a function of the SET OF
//! COMPLETED NODES and intersection is idempotent and order-independent
//! — the derived-view law that makes resume ≡ uninterrupted (gated
//! bytewise, the §67.5 discipline).
//!
//! THE PASS ([`FocalLadder::advance`]). FH1's recursion at horizon `k`
//! under a WORK BUDGET — field plus tail reads as the unit (exact,
//! deterministic) and the node fiber cap. Walking a node at residual
//! horizon `j`: decided → the §5 arithmetic; already completed at a
//! residual `≥ j` → its stored fact, no reads (the resume rule); a
//! collapsed receipt under the FULL belief identity → returned instead
//! of descending (§25, P12, [`SuffixMemo`]); reads at the ceiling → the
//! pass STOPS here, deterministically; hidden → Σ over EVERY positive-
//! mass branch at the same `j`; focal `j = 0` → FH1's frontier pricing
//! (`Engine::price_frontier`, one implementation — compose, never copy)
//! or the typed cap refusal, which leaves the ENCLOSING ROOT CHILD
//! unfinished and lets the pass continue at the next root child (FH-A3);
//! focal `j ≥ 1` → max over EVERY legal action at `j − 1`, argmax under
//! `TieRule::LowestTileIndex`. When a stop unwinds, every node whose
//! parent was entered and which did not complete is listed in the
//! RESIDUAL FRONTIER with its retained fact — a typed boundary, never a
//! truncated number (§41(7)).
//!
//! THE ROOT IS A DERIVED VIEW ([`FocalLadder::root_view`]): per legal
//! root action the stored fact at the root child (or the placeholder);
//! bar `= max_a L_a`; survivors `= {a : U_a absent or U_a ≥ bar}`;
//! `Settled{b}` iff every other action has an upper below `L_b`;
//! `Equivalent` iff every survivor is collapsed (FH-tie); an action whose
//! upper is absent has no interval and blocks `Settled`; `U*` and `Γ`
//! exist only when every action's upper does. Nothing here is stored
//! twice. `Γ_{k+1} ≤ Γ_k` holds unconditionally because no fact is ever
//! discarded (P8(ii)).
//!
//! THE PROOF-STATE PRODUCER ([`FocalHorizonProducer`]) emits one
//! `Fact::Bound` per root action and side from the derived view: lowers
//! under `focal-horizon:<tail id>:k=<k>:lower`, `executable` exactly when
//! the STORED policy re-prices to the value through the independent
//! evaluator (checked at production, never assumed); uppers under
//! `focal-horizon:god:k=<k>:upper`, never executable. Facts from an
//! interrupted pass carry the RETAINED values.
//!
//! EXPLORATORY tier throughout — below every evidentiary tier, cited by
//! nothing above it. No floats; the rational appears only at the report.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

use num_rational::BigRational;

use crate::rules::Domino;
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{
    viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
};
use crate::solver::focal_horizon::{
    history_key, ratio, viewer_legal, ActionInterval, Engine, FocalChoices, FocalRefusal,
    FocalSpend, FocalVerdict,
};
use crate::solver::horizon::CountingField;
use crate::solver::policy::content_digest;
use crate::solver::proof_state::{BoundFact, Fact, ProofProducer, ProofState, ProofTag};

/// FH1's five coordinates minus the horizon (FH-A4): a ladder's facts
/// are per (root, field, contract, tail) and never projected across any
/// of them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LadderIdentity {
    pub root_id: u64,
    pub field_id: String,
    pub contract: u32,
    pub tail_id: String,
}

/// The evaluation context a pass runs under. Must match the ladder's
/// identity (asserted at every `advance`).
pub struct LadderContext<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub lower_tail: &'a dyn SlicePolicy,
    pub field: &'a dyn SlicePolicy,
}

impl LadderContext<'_> {
    pub fn identity(&self) -> LadderIdentity {
        LadderIdentity {
            root_id: root_identity(self.root, self.position),
            field_id: self.field.id().to_string(),
            contract: self.position.bid,
            tail_id: self.lower_tail.id().to_string(),
        }
    }

    fn total_plays(&self) -> usize {
        let kernel = self.root.kernel();
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>()
    }
}

/// One pass's work budget: the read ceiling (field + tail reads; the
/// pass stops at the first node entered with reads at or above it — a
/// frontier evaluation is never cut inside, so the spend may exceed the
/// ceiling by one evaluation) and the node fiber cap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkBudget {
    pub read_ceiling: u64,
    pub node_fiber_cap: u128,
}

/// One node's fact: the interval in mass form, the policy attaining the
/// lower, the residual horizons each side was established at, and the
/// highest residual horizon at which the node's subtree completed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeFact {
    /// `Z(C)·L(C)`.
    pub lower_mass: u128,
    /// The residual horizon the winning lower was established at.
    pub lower_horizon: usize,
    /// The choice table attaining `lower_mass`, keyed by post-root
    /// history over the node's subtree; the tail below and off it.
    pub policy: BTreeMap<Vec<u8>, Domino>,
    /// `Z(C)·U(C)` — always present on a stored fact (a fact exists only
    /// for a completed node, and a completed node was priced).
    pub upper_mass: u128,
    /// The residual horizon the winning upper was established at.
    pub upper_horizon: usize,
    /// The highest residual horizon at which this node completed.
    pub completed_at: usize,
}

impl NodeFact {
    /// Collapsed: `L = U`, hence `= Q` (exact for every later horizon).
    pub fn collapsed(&self) -> bool {
        self.lower_mass == self.upper_mass
    }

    fn value(&self) -> Value {
        Value {
            lower: self.lower_mass,
            upper: self.upper_mass,
            policy: self.policy.clone(),
        }
    }
}

/// One subtree's completed value inside a pass.
struct Value {
    lower: u128,
    upper: u128,
    policy: BTreeMap<Vec<u8>, Domino>,
}

/// Why a node sits in the residual frontier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResidualCause {
    /// The pass stopped here: reads reached the ceiling on entry.
    Stopped,
    /// A `k`-th-frontier node whose fiber exceeds the cap (FH-A3).
    Unaffordable { fiber: u128, cap: u128 },
    /// Entered, but a descendant stopped or refused before it completed.
    Enclosing,
    /// Never entered: a later sibling on the unwinding path.
    Unvisited,
}

/// One unfinished node of an interrupted pass with its retained fact
/// (`None` = the placeholder: lower 0 with the tail, no upper).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidualNode {
    pub history: Vec<Domino>,
    /// `Z(C)`.
    pub mass: u128,
    pub retained: Option<NodeFact>,
    pub cause: ResidualCause,
}

/// One root action's derived view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LadderAction {
    pub action: Domino,
    /// `Z·L_a` — the placeholder 0 when the child holds no fact.
    pub lower_mass: u128,
    /// `None` = the placeholder (no fact at the child).
    pub lower_horizon: Option<usize>,
    /// `None` = no upper fact: no interval, blocks `Settled`.
    pub upper_mass: Option<u128>,
    pub upper_horizon: Option<usize>,
    pub root_mass: u128,
}

impl LadderAction {
    /// The action interval, when an upper fact exists.
    pub fn interval(&self) -> Option<ActionInterval> {
        self.upper_mass.map(|u| ActionInterval {
            action: self.action,
            lower_mass: self.lower_mass,
            upper_mass: u,
            root_mass: self.root_mass,
            lower: ratio(self.lower_mass, self.root_mass),
            upper: ratio(u, self.root_mass),
        })
    }

    pub fn collapsed(&self) -> bool {
        self.upper_mass == Some(self.lower_mass)
    }

    pub fn lower(&self) -> BigRational {
        ratio(self.lower_mass, self.root_mass)
    }

    pub fn upper(&self) -> Option<BigRational> {
        self.upper_mass.map(|u| ratio(u, self.root_mass))
    }
}

/// The root's derived view — a pure function of the fact set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LadderView {
    pub actions: Vec<LadderAction>,
    pub root_mass: u128,
    /// `Z·B = max_a lower_mass`.
    pub bar_mass: u128,
    /// The bar's argmax under the lowest-tile rule — the action the
    /// derived policy plays.
    pub bar_action: Domino,
    pub survivors: Vec<Domino>,
    pub verdict: FocalVerdict,
    /// The derived total policy's table: every root child's stored table
    /// and the root choice; bind with the tail for the total policy.
    pub policy: FocalChoices,
    /// `Z·L_exec = Z·V(π) = bar_mass` (gate PS2 checks it through the
    /// independent evaluator).
    pub executable_lower_mass: u128,
    /// `Z·U*` — present only when every action's upper is.
    pub global_upper_mass: Option<u128>,
    /// `Γ = U* − L_exec` — present only when `U*` is.
    pub certified_regret: Option<BigRational>,
    /// The lowest `completed_at` over the root children — the horizon
    /// the whole root is established at; `None` while some child holds
    /// no fact.
    pub horizon: Option<usize>,
}

impl LadderView {
    pub fn bar(&self) -> BigRational {
        ratio(self.bar_mass, self.root_mass)
    }

    pub fn global_upper(&self) -> Option<BigRational> {
        self.global_upper_mass.map(|u| ratio(u, self.root_mass))
    }

    pub fn action(&self, a: Domino) -> Option<&LadderAction> {
        self.actions.iter().find(|x| x.action == a)
    }
}

/// One pass's report: the derived view after it, the exact spend, the
/// budget it ran under, the suffix hits and the fact-store movement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PassReport {
    pub horizon: usize,
    pub view: LadderView,
    pub spend: FocalSpend,
    /// Field + tail reads this pass.
    pub reads_spent: u64,
    pub ceiling: u64,
    pub suffix_hits: u64,
    pub suffix_lookups: u64,
    /// Nodes that received their first fact this pass.
    pub facts_new: usize,
    /// Nodes revisited this pass whose fact strictly tightened.
    pub facts_tightened: usize,
    /// Nodes revisited this pass (tightened or not).
    pub facts_revisited: usize,
    /// Root children completed this pass, in tile order.
    pub children_completed: Vec<Domino>,
}

/// The `advance` outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Every root child completed at horizon `k`.
    Completed { report: PassReport },
    /// The pass stopped (reads at the ceiling) or refused (a fiber above
    /// the cap) somewhere; the residual frontier lists every unfinished
    /// node with its retained fact; `stopping_node` names where the
    /// budget stop happened (`None` when only cap refusals occurred).
    Interrupted {
        report: PassReport,
        residual_frontier: Vec<ResidualNode>,
        stopping_node: Option<Vec<Domino>>,
        /// Every cap refusal: the node and its fiber.
        unaffordable: Vec<(Vec<Domino>, u128)>,
    },
}

impl Outcome {
    pub fn report(&self) -> &PassReport {
        match self {
            Outcome::Completed { report } | Outcome::Interrupted { report, .. } => report,
        }
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Outcome::Completed { .. })
    }
}

// ---------------------------------------------------------------------------
// The suffix memo (§25, P12).
// ---------------------------------------------------------------------------

/// A collapsed receipt: `L = U = Q` at a node, with the policy attaining
/// it, established at a residual horizon.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuffixReceipt {
    pub value_mass: u128,
    pub policy: BTreeMap<Vec<u8>, Domino>,
    pub horizon: usize,
}

/// Collapsed receipts keyed by the belief's FULL identity — bucketed by
/// post-root history and matched by `FactorBelief`'s componentwise
/// equality (root, position incl. contract, history, field id, and the
/// posterior with its weights). Never a looser key: the record alone is
/// the PiKey defect (CBS-A6, FH-A9).
#[derive(Default, Debug)]
pub struct SuffixMemo {
    entries: HashMap<Vec<u8>, Vec<(FactorBelief, SuffixReceipt)>>,
    pub hits: u64,
    pub lookups: u64,
    pub receipts: u64,
    /// The post-root history of the first hit ever (the pinned witness).
    pub first_hit: Option<Vec<Domino>>,
    /// A frozen memo answers lookups and accepts no receipt — for
    /// consulting one identity's receipts from another ladder.
    frozen: bool,
}

impl SuffixMemo {
    pub fn new() -> SuffixMemo {
        SuffixMemo::default()
    }

    /// The receipt at exactly this belief, counting the lookup.
    pub fn lookup(&mut self, belief: &FactorBelief) -> Option<&SuffixReceipt> {
        self.lookups += 1;
        let bucket = self.entries.get(&history_key(belief.history()))?;
        let found = bucket.iter().find(|(b, _)| b == belief).map(|(_, r)| r);
        if found.is_some() {
            self.hits += 1;
            if self.first_hit.is_none() {
                self.first_hit = Some(belief.history().to_vec());
            }
        }
        found
    }

    /// Stop accepting receipts (lookups still answer).
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Install a receipt at this belief unless one is already held (or
    /// the memo is frozen).
    pub fn insert(&mut self, belief: &FactorBelief, receipt: SuffixReceipt) {
        if self.frozen {
            return;
        }
        let bucket = self
            .entries
            .entry(history_key(belief.history()))
            .or_default();
        if bucket.iter().any(|(b, _)| b == belief) {
            return;
        }
        bucket.push((belief.clone(), receipt));
        self.receipts += 1;
    }

    /// Whether a receipt is held at exactly this belief, without
    /// counting a lookup.
    pub fn holds(&self, belief: &FactorBelief) -> bool {
        self.entries
            .get(&history_key(belief.history()))
            .is_some_and(|b| b.iter().any(|(x, _)| x == belief))
    }
}

// ---------------------------------------------------------------------------
// The ladder.
// ---------------------------------------------------------------------------

/// The per-root ladder: the identity, the root frame, and the fact store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocalLadder {
    identity: LadderIdentity,
    root_mass: u128,
    /// Legal root actions in tile order.
    legal: Vec<Domino>,
    /// Node facts keyed by post-root history.
    facts: BTreeMap<Vec<u8>, NodeFact>,
    /// Horizons attempted, in order.
    passes: Vec<usize>,
}

enum Stop {
    Budget { history: Vec<Domino> },
    Unaffordable { history: Vec<Domino>, fiber: u128 },
}

/// How a fact arrived: priced by the walk (a collapse becomes a memo
/// receipt), returned by a receipt (keeps the receipt's horizon label,
/// never re-inserted), or the decided arithmetic (never a receipt).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Install {
    Priced,
    Receipt(usize),
    Decided,
}

/// One pass's mutable state.
struct Pass<'a, 'b> {
    ladder: &'a mut FocalLadder,
    engine: Engine<'b>,
    memo: Option<&'a mut SuffixMemo>,
    ceiling: u64,
    frontier: Vec<ResidualNode>,
    hits: u64,
    lookups: u64,
    facts_new: usize,
    facts_tightened: usize,
    facts_revisited: usize,
}

impl FocalLadder {
    /// The empty ladder over one root: no facts, every root action at
    /// the placeholder.
    pub fn open(ctx: &LadderContext<'_>) -> FocalLadder {
        let belief = FactorBelief::uniform_root(ctx.root, ctx.position, ctx.field);
        let z = ctx.oracle.mass(&belief);
        assert!(z > 0, "a root has positive belief mass");
        let legal: Vec<Domino> = viewer_legal(&belief).iter().collect();
        FocalLadder {
            identity: ctx.identity(),
            root_mass: z,
            legal,
            facts: BTreeMap::new(),
            passes: Vec::new(),
        }
    }

    pub fn identity(&self) -> &LadderIdentity {
        &self.identity
    }

    pub fn root_mass(&self) -> u128 {
        self.root_mass
    }

    pub fn legal(&self) -> &[Domino] {
        &self.legal
    }

    pub fn passes(&self) -> &[usize] {
        &self.passes
    }

    /// The stored fact at one post-root history, if the node completed.
    pub fn fact_at(&self, history: &[Domino]) -> Option<&NodeFact> {
        self.facts.get(&history_key(history))
    }

    /// Every stored fact, by history key.
    pub fn facts(&self) -> &BTreeMap<Vec<u8>, NodeFact> {
        &self.facts
    }

    /// Stored facts that have collapsed.
    pub fn collapsed_count(&self) -> usize {
        self.facts.values().filter(|f| f.collapsed()).count()
    }

    /// The §23/FH-int pass at horizon `k` under a budget; a resume is
    /// the same call again with more budget.
    pub fn advance(
        &mut self,
        ctx: &LadderContext<'_>,
        k: usize,
        budget: &WorkBudget,
        memo: Option<&mut SuffixMemo>,
    ) -> Outcome {
        assert_eq!(
            ctx.identity(),
            self.identity,
            "a pass runs under the ladder's own identity (FH-A4, FH-A9)"
        );
        let counted_field = CountingField {
            inner: ctx.field,
            reads: std::cell::Cell::new(0),
        };
        let counted_tail = CountingField {
            inner: ctx.lower_tail,
            reads: std::cell::Cell::new(0),
        };
        let belief = FactorBelief::uniform_root(ctx.root, ctx.position, &counted_field);
        let engine = Engine {
            oracle: ctx.oracle,
            field: &counted_field,
            tail: &counted_tail,
            cap: budget.node_fiber_cap,
            total_plays: ctx.total_plays(),
            spend: FocalSpend::default(),
        };
        self.passes.push(k);
        let legal = self.legal.clone();
        let z = self.root_mass;
        let mut pass = Pass {
            ladder: self,
            engine,
            memo,
            ceiling: budget.read_ceiling,
            frontier: Vec::new(),
            hits: 0,
            lookups: 0,
            facts_new: 0,
            facts_tightened: 0,
            facts_revisited: 0,
        };
        let mut completed = Vec::new();
        let mut stopping_node = None;
        let mut unaffordable = Vec::new();
        for (i, a) in legal.iter().enumerate() {
            let child = belief.focal_play(*a);
            match pass.walk(&child, k) {
                Ok(_) => completed.push(*a),
                Err(Stop::Budget { history }) => {
                    stopping_node = Some(history);
                    for b in &legal[i + 1..] {
                        let retained = pass.ladder.facts.get(&history_key(&[*b])).cloned();
                        pass.frontier.push(ResidualNode {
                            history: vec![*b],
                            mass: z,
                            retained,
                            cause: ResidualCause::Unvisited,
                        });
                    }
                    break;
                }
                Err(Stop::Unaffordable { history, fiber }) => {
                    unaffordable.push((history, fiber));
                }
            }
        }
        let Pass {
            engine,
            mut frontier,
            hits,
            lookups,
            facts_new,
            facts_tightened,
            facts_revisited,
            ..
        } = pass;
        let mut spend = engine.spend;
        spend.field_reads = counted_field.reads.get();
        spend.tail_reads = counted_tail.reads.get();
        let reads_spent = spend.field_reads.saturating_add(spend.tail_reads);
        let report = PassReport {
            horizon: k,
            view: self.root_view(),
            spend,
            reads_spent,
            ceiling: budget.read_ceiling,
            suffix_hits: hits,
            suffix_lookups: lookups,
            facts_new,
            facts_tightened,
            facts_revisited,
            children_completed: completed.clone(),
        };
        if completed.len() == legal.len() {
            assert!(
                frontier.is_empty(),
                "a completed pass leaves no residual frontier"
            );
            Outcome::Completed { report }
        } else {
            frontier.sort_by_key(|n| history_key(&n.history));
            assert!(
                frontier.windows(2).all(|w| w[0].history != w[1].history),
                "a residual node is listed once"
            );
            Outcome::Interrupted {
                report,
                residual_frontier: frontier,
                stopping_node,
                unaffordable,
            }
        }
    }

    /// The root's derived view (module doc) — recomputed from the facts
    /// on every call, never stored.
    pub fn root_view(&self) -> LadderView {
        let z = self.root_mass;
        let mut actions = Vec::with_capacity(self.legal.len());
        let mut table: BTreeMap<Vec<u8>, Domino> = BTreeMap::new();
        let mut horizon: Option<usize> = Some(usize::MAX);
        for a in &self.legal {
            match self.facts.get(&history_key(&[*a])) {
                Some(f) => {
                    for (key, v) in &f.policy {
                        let prior = table.insert(key.clone(), *v);
                        assert!(prior.is_none(), "root actions extend disjoint histories");
                    }
                    horizon = horizon.map(|h| h.min(f.completed_at));
                    actions.push(LadderAction {
                        action: *a,
                        lower_mass: f.lower_mass,
                        lower_horizon: Some(f.lower_horizon),
                        upper_mass: Some(f.upper_mass),
                        upper_horizon: Some(f.upper_horizon),
                        root_mass: z,
                    });
                }
                None => {
                    horizon = None;
                    actions.push(LadderAction {
                        action: *a,
                        lower_mass: 0,
                        lower_horizon: None,
                        upper_mass: None,
                        upper_horizon: None,
                        root_mass: z,
                    });
                }
            }
        }
        let bar_mass = actions
            .iter()
            .map(|a| a.lower_mass)
            .max()
            .expect("a root action exists");
        let bar_action = actions
            .iter()
            .find(|a| a.lower_mass == bar_mass)
            .expect("the bar is attained")
            .action;
        let prior = table.insert(history_key(&[]), bar_action);
        assert!(prior.is_none(), "the root is the empty history");
        let survivors: Vec<Domino> = actions
            .iter()
            .filter(|a| a.upper_mass.is_none_or(|u| u >= bar_mass))
            .map(|a| a.action)
            .collect();
        let settled = actions
            .iter()
            .all(|a| a.action == bar_action || a.upper_mass.is_some_and(|u| bar_mass > u));
        let verdict = if settled {
            FocalVerdict::Settled { action: bar_action }
        } else {
            let survivors_collapsed = actions
                .iter()
                .filter(|a| a.upper_mass.is_none_or(|u| u >= bar_mass))
                .all(LadderAction::collapsed);
            if survivors_collapsed {
                FocalVerdict::Equivalent {
                    actions: actions
                        .iter()
                        .filter(|a| a.collapsed() && a.lower_mass == bar_mass)
                        .map(|a| a.action)
                        .collect(),
                    value_mass: bar_mass,
                }
            } else {
                FocalVerdict::Unresolved {
                    survivors: survivors.clone(),
                }
            }
        };
        let global_upper_mass = actions
            .iter()
            .map(|a| a.upper_mass)
            .try_fold(0u128, |m, u| u.map(|u| m.max(u)));
        let certified_regret = global_upper_mass.map(|u| ratio(u - bar_mass, z));
        let policy = FocalChoices::new(horizon.unwrap_or(0), &self.identity.tail_id, table);
        LadderView {
            actions,
            root_mass: z,
            bar_mass,
            bar_action,
            survivors,
            verdict,
            policy,
            executable_lower_mass: bar_mass,
            global_upper_mass,
            certified_regret,
            horizon,
        }
    }

    /// The bytewise render of the fact set (the §67.5 comparison object):
    /// the identity, the root frame, then every fact in history order
    /// with its policy's content digest. Spend and pass history are NOT
    /// part of it (the spend is compared as a sum).
    pub fn render(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "focal-ladder-v1");
        let _ = writeln!(
            out,
            "identity root={} field={} contract={} tail={}",
            self.identity.root_id,
            self.identity.field_id,
            self.identity.contract,
            self.identity.tail_id
        );
        let legal: Vec<String> = self.legal.iter().map(|d| format!("{d}")).collect();
        let _ = writeln!(out, "root Z={} legal={}", self.root_mass, legal.join(","));
        for (key, f) in &self.facts {
            let names: Vec<String> = key
                .iter()
                .map(|i| {
                    Domino::from_index(usize::from(*i))
                        .map_or_else(|| "?".to_string(), |d| format!("{d}"))
                })
                .collect();
            let _ = writeln!(
                out,
                "fact [{}] L={}@{} U={}@{} done={} pi={}x{}",
                names.join(" "),
                f.lower_mass,
                f.lower_horizon,
                f.upper_mass,
                f.upper_horizon,
                f.completed_at,
                f.policy.len(),
                table_digest(&f.policy)
            );
        }
        out
    }
}

fn table_digest(table: &BTreeMap<Vec<u8>, Domino>) -> String {
    let mut bytes = Vec::new();
    for (k, v) in table {
        bytes.extend_from_slice(k);
        bytes.push(0xff);
        bytes.push(u8::try_from(v.index()).expect("a tile index fits u8"));
        bytes.push(0xfe);
    }
    let digest = content_digest(&bytes);
    let mut hex = String::new();
    for b in &digest[..16] {
        hex.push_str(&format!("{b:02x}"));
    }
    hex
}

impl Pass<'_, '_> {
    /// The pass recursion (module doc). `j` is the residual horizon.
    fn walk(&mut self, belief: &FactorBelief, j: usize) -> Result<Value, Stop> {
        let viewer = belief.kernel().viewer();
        let state = belief.public_state();
        let depth = belief.history().len();
        let at_terminal = depth == self.engine.total_plays;
        if let Some(u) = decided_success(belief.position(), viewer, state.banked, at_terminal) {
            if at_terminal {
                self.engine.spend.decided_terminal += 1;
            } else {
                self.engine.spend.decided_early += 1;
            }
            let z = self.engine.oracle.mass(belief);
            let v = if u { z } else { 0 };
            // A decided node's fact is the §5 arithmetic: exact, read-free,
            // stored so the root view is exact at decided children; never
            // a memo receipt (the decided check precedes every lookup).
            let key = history_key(belief.history());
            return Ok(self.install(
                belief,
                &key,
                Value {
                    lower: v,
                    upper: v,
                    policy: BTreeMap::new(),
                },
                j,
                Install::Decided,
            ));
        }
        assert!(
            depth < self.engine.total_plays,
            "an undecided state has plays left (rules invariant)"
        );
        let key = history_key(belief.history());
        // The resume rule: completed at this residual or deeper → the
        // stored fact, no reads.
        if let Some(f) = self.ladder.facts.get(&key) {
            if f.completed_at >= j {
                return Ok(f.value());
            }
        }
        // §25: a collapsed receipt under the FULL identity.
        let receipt: Option<SuffixReceipt> = match self.memo.as_deref_mut() {
            Some(memo) => {
                self.lookups += 1;
                memo.lookup(belief).cloned()
            }
            None => None,
        };
        if let Some(r) = receipt {
            self.hits += 1;
            let value = Value {
                lower: r.value_mass,
                upper: r.value_mass,
                policy: r.policy,
            };
            return Ok(self.install(belief, &key, value, j, Install::Receipt(r.horizon)));
        }
        if self.engine.reads() >= self.ceiling {
            let history = belief.history().to_vec();
            let mass = self.engine.oracle.mass(belief);
            self.push_residual(history.clone(), mass, ResidualCause::Stopped);
            return Err(Stop::Budget { history });
        }
        let seat = state.leader.plus(state.plays.len());
        if seat != viewer {
            self.engine.spend.hidden_nodes += 1;
            let branches = self.engine.oracle.branch_masses(belief, self.engine.field);
            let mut lower: u128 = 0;
            let mut upper: u128 = 0;
            let mut policy: BTreeMap<Vec<u8>, Domino> = BTreeMap::new();
            for (i, (tile, _)) in branches.iter().enumerate() {
                self.engine.spend.conditionings += 1;
                let child = self
                    .engine
                    .oracle
                    .condition(belief, *tile, self.engine.field);
                match self.walk(&child, j) {
                    Ok(v) => {
                        lower = lower.checked_add(v.lower).expect("an exact mass fits u128");
                        upper = upper.checked_add(v.upper).expect("an exact mass fits u128");
                        for (k2, d) in v.policy {
                            let prior = policy.insert(k2, d);
                            assert!(prior.is_none(), "hidden branches extend disjoint histories");
                        }
                    }
                    Err(stop) => {
                        for (tile2, mass2) in &branches[i + 1..] {
                            let mut h = belief.history().to_vec();
                            h.push(*tile2);
                            self.push_residual(h, *mass2, ResidualCause::Unvisited);
                        }
                        let mass = branches.iter().map(|(_, m)| *m).sum::<u128>();
                        self.push_residual(
                            belief.history().to_vec(),
                            mass,
                            ResidualCause::Enclosing,
                        );
                        return Err(stop);
                    }
                }
            }
            return Ok(self.install(
                belief,
                &key,
                Value {
                    lower,
                    upper,
                    policy,
                },
                j,
                Install::Priced,
            ));
        }
        let legal = viewer_legal(belief);
        if j == 0 {
            match self.engine.price_frontier(belief, legal.len(), depth) {
                Ok((lower, upper)) => {
                    return Ok(self.install(
                        belief,
                        &key,
                        Value {
                            lower,
                            upper,
                            policy: BTreeMap::new(),
                        },
                        0,
                        Install::Priced,
                    ));
                }
                Err(FocalRefusal::UpperUnaffordable {
                    history,
                    fiber,
                    cap,
                }) => {
                    self.push_residual(
                        history.clone(),
                        fiber,
                        ResidualCause::Unaffordable { fiber, cap },
                    );
                    return Err(Stop::Unaffordable { history, fiber });
                }
            }
        }
        self.engine.spend.focal_nodes += 1;
        let tiles: Vec<Domino> = legal.iter().collect();
        let mut best: Option<(u128, Domino, BTreeMap<Vec<u8>, Domino>)> = None;
        let mut upper: Option<u128> = None;
        for (i, tile) in tiles.iter().enumerate() {
            match self.walk(&belief.focal_play(*tile), j - 1) {
                Ok(v) => {
                    upper = Some(upper.map_or(v.upper, |u| u.max(v.upper)));
                    let take = match &best {
                        None => true,
                        Some((b, _, _)) => v.lower > *b,
                    };
                    if take {
                        best = Some((v.lower, *tile, v.policy));
                    }
                }
                Err(stop) => {
                    let mass = self.engine.oracle.mass(belief);
                    for tile2 in &tiles[i + 1..] {
                        let mut h = belief.history().to_vec();
                        h.push(*tile2);
                        self.push_residual(h, mass, ResidualCause::Unvisited);
                    }
                    self.push_residual(belief.history().to_vec(), mass, ResidualCause::Enclosing);
                    return Err(stop);
                }
            }
        }
        let (lower, tile, mut policy) = best.expect("a legal set holds an action");
        let prior = policy.insert(key.clone(), tile);
        assert!(prior.is_none(), "a history names one focal node");
        Ok(self.install(
            belief,
            &key,
            Value {
                lower,
                upper: upper.expect("a legal set holds an action"),
                policy,
            },
            j,
            Install::Priced,
        ))
    }

    fn push_residual(&mut self, history: Vec<Domino>, mass: u128, cause: ResidualCause) {
        let retained = self.ladder.facts.get(&history_key(&history)).cloned();
        self.frontier.push(ResidualNode {
            history,
            mass,
            retained,
            cause,
        });
    }

    /// Intersection install (module doc); returns the fact's value — the
    /// parent composes from the FACT, never from the raw new value.
    fn install(
        &mut self,
        belief: &FactorBelief,
        key: &[u8],
        new: Value,
        j: usize,
        kind: Install,
    ) -> Value {
        assert!(new.lower <= new.upper, "a fact is an interval (FH-int)");
        let label = match kind {
            Install::Receipt(h) => h,
            Install::Priced | Install::Decided => j,
        };
        if let Some(f) = self.ladder.facts.get_mut(key) {
            self.facts_revisited += 1;
            let mut tightened = false;
            if new.lower > f.lower_mass {
                f.lower_mass = new.lower;
                f.policy = new.policy;
                f.lower_horizon = label;
                tightened = true;
            }
            if new.upper < f.upper_mass {
                f.upper_mass = new.upper;
                f.upper_horizon = label;
                tightened = true;
            }
            if tightened {
                self.facts_tightened += 1;
            }
            f.completed_at = f.completed_at.max(j);
            assert!(
                f.lower_mass <= f.upper_mass,
                "the intersection of two intervals containing Q is an interval (FH-int)"
            );
        } else {
            self.facts_new += 1;
            self.ladder.facts.insert(
                key.to_vec(),
                NodeFact {
                    lower_mass: new.lower,
                    lower_horizon: label,
                    policy: new.policy,
                    upper_mass: new.upper,
                    upper_horizon: label,
                    completed_at: j,
                },
            );
        }
        let fact = self
            .ladder
            .facts
            .get(key)
            .expect("the fact was just installed");
        let value = fact.value();
        if fact.collapsed() && kind == Install::Priced {
            let receipt = SuffixReceipt {
                value_mass: fact.lower_mass,
                policy: fact.policy.clone(),
                horizon: fact.lower_horizon,
            };
            if let Some(memo) = self.memo.as_deref_mut() {
                memo.insert(belief, receipt);
            }
        }
        value
    }
}

// ---------------------------------------------------------------------------
// The proof-state producer.
// ---------------------------------------------------------------------------

/// The ladder as a [`ProofProducer`] (module doc). The context is the
/// one the ladder's passes ran under: the executability check re-prices
/// each stored policy through [`viewer_success_mass`] at production.
pub struct FocalHorizonProducer<'a> {
    pub ladder: &'a FocalLadder,
    pub ctx: &'a LadderContext<'a>,
}

impl FocalHorizonProducer<'_> {
    /// Re-price the stored policy at one root child through the
    /// independent evaluator: `Some(mass)` when a fact exists.
    pub fn reprice(&self, action: Domino) -> Option<u128> {
        let fact = self.ladder.fact_at(&[action])?;
        let belief = FactorBelief::uniform_root(self.ctx.root, self.ctx.position, self.ctx.field);
        let choices = FocalChoices::new(
            fact.lower_horizon,
            self.ctx.lower_tail.id(),
            fact.policy.clone(),
        );
        let policy = choices.with_tail(self.ctx.lower_tail);
        let mut rs = RecursionStats::default();
        Some(viewer_success_mass(
            self.ctx.oracle,
            &belief.focal_play(action),
            &policy,
            self.ctx.field,
            &mut rs,
        ))
    }
}

impl ProofProducer for FocalHorizonProducer<'_> {
    fn name(&self) -> &str {
        "focal-horizon-ladder"
    }

    fn produce(&self, _state: &ProofState) -> Vec<Fact> {
        assert_eq!(
            self.ctx.identity(),
            *self.ladder.identity(),
            "the producer's context is the ladder's identity"
        );
        let view = self.ladder.root_view();
        let z = view.root_mass;
        let mut out = Vec::new();
        for a in &view.actions {
            if let Some(k) = a.lower_horizon {
                if a.lower_mass > 0 {
                    let priced = self
                        .reprice(a.action)
                        .expect("a lower fact exists at this child");
                    assert!(
                        priced >= a.lower_mass,
                        "a stored policy never prices below its fact (FH-int executability)"
                    );
                    let executable = priced == a.lower_mass;
                    out.push(Fact::Bound(BoundFact::lower(
                        a.action,
                        ratio(a.lower_mass, z),
                        &format!(
                            "focal-horizon:{}:k={k}:lower",
                            self.ladder.identity().tail_id
                        ),
                        executable,
                        ProofTag::Deterministic,
                    )));
                }
            }
            if let (Some(u), Some(k)) = (a.upper_mass, a.upper_horizon) {
                out.push(Fact::Bound(BoundFact::upper(
                    a.action,
                    ratio(u, z),
                    &format!("focal-horizon:god:k={k}:upper"),
                    ProofTag::Deterministic,
                )));
            }
        }
        out
    }
}
