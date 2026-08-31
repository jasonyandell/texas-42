//! `solver::refine` — the counted-belief Slice G: the §50 integrated
//! refinement controller. One root, one declared field, one work-item
//! loop that unifies the ladder's authorities — Slice A's sampled δ root
//! intervals (`root_interval`), Slice D's exact fixed-policy recursion
//! and Slice E's exact grammar best response (`factor_belief`), the §36
//! full-action-set escalation endpoint
//! ([`response_success_mass`](crate::solver::factor_belief::response_success_mass)
//! — the enumeration fallback), and the existing `ScopedDelta` risk
//! arithmetic — under the §34 steering rule, §35 decision-oriented
//! scheduling, and the §37 soundness invariant.
//!
//! EXPLORATORY tier. Mathematical source:
//! `walt/math/counted_belief_sandwich_v0.1.md` Part VIII (§32–37), §50
//! Slice G, with §34's steering generalization and §38's completeness
//! shape; design register `walt/FACTOR-BELIEF.md`.
//!
//! THE LAW (§36, specialized to one root): per legal root action keep a
//! typed interval `[L_a, U_a]` — every lower is the exact or δ-valid
//! value of a lawful information-consistent policy or restricted lawful
//! class, every upper covers the complete policy region it names
//! (§37.1–2). The bar is `B = max_a L_a`; an action is EXCLUDED exactly
//! when `U_a < B` (§34), and exclusions are permanent because lowers
//! only rise and uppers only fall (the monotone update, enforced here).
//! The root DECISION WIDTH is the declared scalar
//! `D = (|survivors| − 1) + Σ_{a ∈ survivors} (U_a − B)` — zero exactly
//! when one action survives with nothing left to separate. The §35
//! scheduler ranks each work item by its best-case reduction of `D` per
//! declared forecast cost (exact rationals, cross-multiplied — no
//! floats); the §34 rule REFUSES an item whose best case cannot change
//! the survivor set or the width — recorded, never run, never charged.
//!
//! WORK ITEMS (the §33 subset this slice builds): `SampledLower` /
//! `SampledUpper` — Slice A's δ-valid frozen-policy witness and
//! optimization-lock empirical-max upper at a declared prefix, each
//! consuming a distinct `ScopedDelta` against the declared root scope
//! (the ledger fence refuses an item that would breach it);
//! `ExactFixed` / `ExactGrammar` — exact lowers by the §23/§48
//! factorized recursions under the declared focal sources;
//! `EscalateExact` — the full-action-set recursion, collapsing the
//! interval to the exact point `Q_a` (§38's singleton-domain authority);
//! `ConsequenceCensus` — the §49 hand-class instrument, carried as a
//! work item precisely to demonstrate §34: its best case narrows FIELD
//! branch intervals, never a root bound, so its declared width reduction
//! is zero and the steering rule refuses it as presently useless at
//! every bar (the census runs as a reported coordinate in the probe,
//! outside the loop). Items not built: the remaining §33 kinds
//! (`SplitPolicyCylinder`, `CountThreatCover`, `EnumerateResidual`, …)
//! have no producers on this ladder yet and are named here only as the
//! honest boundary.
//!
//! BUDGET AND DETERMINISM: the budget is integer work units; every item
//! declares an integer forecast (a documented heuristic — support-hand
//! domain × plays left for recursions, the prefix for sampled items —
//! §33's "estimated cost", never a correctness input) and the controller
//! charges the FORECAST, never wall time, so a run is a pure function of
//! its inputs (a budget refusal reroutes the scheduler to the next
//! ranked item, so a budgeted trace is deterministic but NOT a prefix of
//! the ample one). On exhaustion the result is the honest surviving set
//! with a NAMED fallback rule that is never promoted to a settled result
//! (§37.9).
//!
//! RESULT TYPING: `Settled` (one survivor), `Equivalent` (several
//! survivors, all deterministic point intervals at the bar), `Unresolved`
//! (budget or risk starvation; surviving set plus named fallback). The
//! proof class is `Exact` when every exclusion compared exact bounds and
//! `DeltaQualified` when any exclusion consumed a sampled side — a δ
//! event on a decisive comparison is a δ-qualified decision, never
//! silently exact (§37.4, §42's constructor discipline).
//!
//! WHAT THIS SLICE IS NOT. The controller schedules and accounts; it
//! manufactures no bound (§37.8) — every number it holds is produced by
//! an authority that predates it or by [`response_success_mass`], which
//! the Slice G gates hold to extensional parity with the bundled exact
//! authority `exposure::exact_root_value` at every gated root. No
//! cross-root reuse, no cost-model learning, no default-player change —
//! the existing controller player remains the fallback surface until
//! arena and conformance gates authorize a change (§50).

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::rules::legal_plays;
use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use crate::solver::evidence::ScopedDelta;
use crate::solver::factor_belief::{
    grammar_success_mass, response_success_mass, viewer_success_mass, ExactCoverOracle,
    FactorBelief, FactorWeights, RecursionStats, ResponseStats,
};
use crate::solver::field::{FieldModel, FieldSpec};
use crate::solver::grammar::{CountPreservation, PolicyGrammar};
use crate::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use crate::solver::root_interval::{
    frozen_policy_lower, pmake_empirical_max_upper, PolicyProvenance, RootActionLower,
    RootActionUpper,
};
use crate::solver::upper_cs::assert_screen_risk_allocation;

// ---------------------------------------------------------------------------
// Typed bounds (§42's constructor discipline).
// ---------------------------------------------------------------------------

/// A lower bound on `Q_a`, typed by its authority (§42): sampled bounds
/// carry their full Slice A record (stream, grid, provenance, δ); exact
/// bounds carry the integer success MASS over the root `Z` and the
/// identity of the policy or class that attains it.
#[derive(Clone, Debug)]
pub enum LowerBound {
    /// No work yet: `L_a = 0` (every mass is nonnegative).
    Vacuous,
    /// Slice A's δ-valid frozen-policy witness.
    Sampled(RootActionLower),
    /// Slice D: the exact value of one frozen focal policy (§47).
    ExactPolicy { mass: u128, policy: String },
    /// Slice E: the exact grammar optimum `Q^G_a` (§48).
    ExactGrammar { mass: u128, grammar: String },
    /// The escalated exact `Q_a` itself (§36 EscalateExact).
    ExactResponse { mass: u128 },
}

/// An upper bound on `Q_a`, typed by the policy region it covers (§42).
#[derive(Clone, Debug)]
pub enum UpperBound {
    /// No work yet: `U_a = 1` (the trivial cover of every policy).
    Vacuous,
    /// Slice A's δ-valid optimization-lock empirical-max upper.
    Sampled(RootActionUpper),
    /// The escalated exact `Q_a` itself — the point cover.
    ExactResponse { mass: u128 },
}

/// One root action's typed interval `[L_a, U_a]` over the shared root
/// mass `Z` (a focal play changes no factor, so every root child carries
/// the root's own `Z` — asserted at construction).
#[derive(Clone, Debug)]
pub struct ActionInterval {
    pub action: Domino,
    pub z: u128,
    pub lower: LowerBound,
    pub upper: UpperBound,
}

fn ratio(mass: u128, z: u128) -> BigRational {
    assert!(mass <= z, "a success mass never exceeds the partition mass");
    BigRational::new(BigInt::from(mass), BigInt::from(z))
}

impl ActionInterval {
    pub fn lower_value(&self) -> BigRational {
        match &self.lower {
            LowerBound::Vacuous => BigRational::zero(),
            LowerBound::Sampled(l) => l.lower(),
            LowerBound::ExactPolicy { mass, .. }
            | LowerBound::ExactGrammar { mass, .. }
            | LowerBound::ExactResponse { mass } => ratio(*mass, self.z),
        }
    }

    pub fn upper_value(&self) -> BigRational {
        match &self.upper {
            UpperBound::Vacuous => BigRational::one(),
            UpperBound::Sampled(u) => u.upper(),
            UpperBound::ExactResponse { mass } => ratio(*mass, self.z),
        }
    }

    pub fn lower_is_exact(&self) -> bool {
        !matches!(self.lower, LowerBound::Vacuous | LowerBound::Sampled(_))
    }

    pub fn upper_is_exact(&self) -> bool {
        matches!(self.upper, UpperBound::ExactResponse { .. })
    }

    /// A point interval: the two endpoint values coincide.
    pub fn is_point(&self) -> bool {
        self.lower_value() == self.upper_value()
    }

    /// A point interval pinned WITHOUT a sampled side: `L ≤ Q ≤ U` with
    /// `L = U` and both endpoints deterministically valid (exact-typed
    /// or the trivial vacuous cover), so `Q` equals the point exactly —
    /// no δ qualification. A vacuous side counts: `[1, 1]` from an
    /// exact grammar lower under the trivial upper pins a certain make,
    /// and `[0, 0]` from an escalated zero under the vacuous lower pins
    /// a certain miss.
    pub fn deterministic_point(&self) -> bool {
        self.is_point()
            && !matches!(self.lower, LowerBound::Sampled(_))
            && !matches!(self.upper, UpperBound::Sampled(_))
    }

    pub fn width(&self) -> BigRational {
        self.upper_value() - self.lower_value()
    }
}

// ---------------------------------------------------------------------------
// Work items, refusals, the trace (§32's explanation trace, §33).
// ---------------------------------------------------------------------------

/// The §33 work-item subset this slice builds (module doc).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkItem {
    SampledLower(Domino),
    SampledUpper(Domino),
    ExactFixed(Domino),
    ExactGrammar(Domino),
    EscalateExact(Domino),
    ConsequenceCensus,
}

impl WorkItem {
    fn action(self) -> Option<Domino> {
        match self {
            WorkItem::SampledLower(a)
            | WorkItem::SampledUpper(a)
            | WorkItem::ExactFixed(a)
            | WorkItem::ExactGrammar(a)
            | WorkItem::EscalateExact(a) => Some(a),
            WorkItem::ConsequenceCensus => None,
        }
    }

    /// Deterministic scheduling key: kind ordinal, then tile index.
    fn key(self) -> (u8, u8) {
        let (kind, action) = match self {
            WorkItem::SampledLower(a) => (0u8, Some(a)),
            WorkItem::SampledUpper(a) => (1, Some(a)),
            WorkItem::ExactFixed(a) => (2, Some(a)),
            WorkItem::ExactGrammar(a) => (3, Some(a)),
            WorkItem::EscalateExact(a) => (4, Some(a)),
            WorkItem::ConsequenceCensus => (5, None),
        };
        (kind, action.map_or(255, |a| a.index() as u8))
    }
}

impl fmt::Display for WorkItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkItem::SampledLower(a) => write!(f, "sampled-lower({a})"),
            WorkItem::SampledUpper(a) => write!(f, "sampled-upper({a})"),
            WorkItem::ExactFixed(a) => write!(f, "exact-fixed({a})"),
            WorkItem::ExactGrammar(a) => write!(f, "exact-grammar({a})"),
            WorkItem::EscalateExact(a) => write!(f, "escalate-exact({a})"),
            WorkItem::ConsequenceCensus => write!(f, "consequence-census"),
        }
    }
}

/// Why the controller refused a work item (§34, the ledger fence, the
/// budget fence). Refused items never run and never charge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalReason {
    /// The item's action is already excluded — no bound of an excluded
    /// action can change the surviving set (§34).
    ExcludedAction,
    /// The item's best case changes neither the surviving set nor the
    /// decision width at the current bar (§34's "presently useless").
    PresentlyUseless,
    /// The declared forecast exceeds the remaining work budget.
    ExceedsBudget,
    /// The item's δ would breach the declared root risk scope.
    ExceedsRiskScope,
}

/// The record of one run item: its declared forecast was charged and the
/// touched action's interval moved (or provably stayed — recorded either
/// way).
#[derive(Clone, Debug)]
pub struct RanEvent {
    pub item: WorkItem,
    pub cost: u64,
    pub lower_before: BigRational,
    pub upper_before: BigRational,
    pub lower_after: BigRational,
    pub upper_after: BigRational,
    pub bar_after: BigRational,
    pub survivors_after: DominoSet,
}

/// One §32 explanation-trace event.
#[derive(Clone, Debug)]
pub enum TraceEvent {
    /// An item ran.
    Ran(Box<RanEvent>),
    /// An item was refused (recorded once per item per reason).
    Refused {
        item: WorkItem,
        reason: RefusalReason,
    },
    /// An action fell below the bar. `delta_decisive` marks whether a
    /// sampled side took part in the decisive comparison.
    Excluded {
        action: Domino,
        bar_holder: Domino,
        delta_decisive: bool,
    },
}

// ---------------------------------------------------------------------------
// Results (§36 step 12, §37.9).
// ---------------------------------------------------------------------------

/// Whether every decisive exclusion compared exact bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProofClass {
    Exact,
    DeltaQualified,
}

/// The controller's typed outcome. A fallback choice lives ONLY in
/// `Unresolved` and is never promoted (§37.9).
#[derive(Clone, Debug)]
pub enum RefineResult {
    /// One action survives.
    Settled { action: Domino, proof: ProofClass },
    /// Several actions survive with exact point intervals at the bar —
    /// an honest exact tie.
    Equivalent {
        actions: DominoSet,
        value: BigRational,
        proof: ProofClass,
    },
    /// Work ran out (budget or risk scope) before the root settled: the
    /// honest surviving set, with the NAMED fallback rule.
    Unresolved {
        survivors: DominoSet,
        fallback: Domino,
        rule: &'static str,
    },
}

/// The full record of one controller run.
#[derive(Clone, Debug)]
pub struct RefineOutcome {
    /// Final typed intervals, in root-action (tile-index) order — every
    /// legal action, excluded ones included.
    pub intervals: Vec<ActionInterval>,
    pub excluded: DominoSet,
    pub survivors: DominoSet,
    pub bar: BigRational,
    pub result: RefineResult,
    pub trace: Vec<TraceEvent>,
    /// Work units charged (forecasts of run items — deterministic).
    pub work_spent: u64,
    pub budget: u64,
    /// The exact rational δ consumed, re-asserted through
    /// `assert_screen_risk_allocation` against the declared scope.
    pub risk_spent: BigRational,
    pub delta_decisive: bool,
}

/// The declared controller configuration. `prefix = 0` disables the
/// sampled tier (the exact-only ladder); `budget` is integer work units
/// against declared forecasts.
#[derive(Clone, Debug)]
pub struct RefineConfig {
    pub budget: u64,
    pub prefix: u64,
    /// δ per sampled endpoint (each endpoint a distinct scope).
    pub delta: BigRational,
    /// The root risk-scope budget the endpoint δs must fit inside.
    pub scope_budget: BigRational,
}

// ---------------------------------------------------------------------------
// Forecasts (§33's declared estimated cost — a heuristic, never a
// correctness input).
// ---------------------------------------------------------------------------

fn choose(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut acc: u128 = 1;
    for i in 0..k {
        acc = acc
            .checked_mul(u128::from(n - i))
            .expect("a forecast binomial fits u128")
            / u128::from(i + 1);
    }
    u64::try_from(acc).expect("a forecast binomial fits u64")
}

/// The largest hidden-seat root-hand domain — the recursion's cost
/// driver (§22: hands stand in for worlds).
fn hand_domain(belief: &FactorBelief) -> u64 {
    belief
        .factors()
        .iter()
        .map(|f| match f.weights() {
            FactorWeights::UniformLawful { allowed } => choose(
                u64::try_from(allowed.len()).expect("fits"),
                u64::try_from(f.capacity()).expect("fits"),
            ),
            FactorWeights::Table(rows) => u64::try_from(rows.len()).expect("fits"),
        })
        .max()
        .expect("a root belief holds hidden seats")
}

fn forecast(
    item: WorkItem,
    cfg: &RefineConfig,
    hands: u64,
    plays_left: u64,
    root_hand: u64,
) -> u64 {
    match item {
        WorkItem::SampledLower(_) | WorkItem::SampledUpper(_) => cfg.prefix,
        WorkItem::ExactFixed(_) => hands.saturating_mul(plays_left),
        WorkItem::ExactGrammar(_) => hands.saturating_mul(plays_left).saturating_mul(3),
        WorkItem::EscalateExact(_) => hands.saturating_mul(plays_left).saturating_mul(root_hand),
        WorkItem::ConsequenceCensus => hands,
    }
}

// ---------------------------------------------------------------------------
// The controller (§36).
// ---------------------------------------------------------------------------

/// The Slice A witness construction, reused verbatim as the §36 step-2
/// seed: play the pinned root action, then the level-1 continuation at
/// the declared inner schedule — provenance FIXED (declared a priori,
/// no on-stream selection).
fn pinned_level1(position: &RootPosition, tile: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
        solver_source: "walt-level1-continuation-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![2, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned: tile },
    })
}

struct ActionState {
    interval: ActionInterval,
    excluded: bool,
}

fn bar_of(states: &[ActionState]) -> (BigRational, Domino, bool) {
    let mut best: Option<(BigRational, Domino, bool)> = None;
    for s in states {
        let v = s.interval.lower_value();
        let better = match &best {
            None => true,
            Some((bv, _, _)) => v > *bv,
        };
        if better {
            best = Some((v, s.interval.action, s.interval.lower_is_exact()));
        }
    }
    best.expect("a root holds a legal action")
}

/// The declared decision-width scalar (module doc):
/// `D = (|survivors| − 1) + Σ_{a ∈ survivors} (U_a − B)`.
fn decision_width(
    lowers: &[BigRational],
    uppers: &[BigRational],
    excluded: &[bool],
) -> (usize, BigRational) {
    let bar = lowers.iter().cloned().max().expect("actions");
    let mut survivors = 0usize;
    let mut excess = BigRational::zero();
    for i in 0..lowers.len() {
        if !excluded[i] && uppers[i] >= bar {
            survivors += 1;
            excess += &uppers[i] - &bar;
        }
    }
    (survivors, excess)
}

fn width_scalar(survivors: usize, excess: &BigRational) -> BigRational {
    BigRational::from_integer(BigInt::from(survivors.saturating_sub(1))) + excess
}

/// Run the §50 controller on one root under one declared field. The
/// outcome is a pure function of its inputs (module doc: determinism).
pub fn refine_root(
    root: &CanonicalRoot,
    position: &RootPosition,
    spec: &FieldSpec,
    oracle: &dyn ExactCoverOracle,
    cfg: &RefineConfig,
) -> RefineOutcome {
    assert!(cfg.budget >= 1, "a controller run declares a work budget");
    let field_sampled = FieldModel::new(spec.clone());
    let field_factor = FieldModel::new(spec.clone());
    let belief = FactorBelief::uniform_root(root, position, &field_factor);
    let z = oracle.mass(&belief);
    assert!(z >= 1, "a root fiber holds a world");

    // The grammar sources (the Slice E declaration).
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let count = CountPreservation::new();
    let grammar = PolicyGrammar::new(vec![&low, &high, &count]);

    // Legal root actions in tile-index order.
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    let actions: Vec<Domino> = legal.iter().collect();

    // Forecast context (declared heuristics).
    let hands = hand_domain(&belief);
    let total_plays = root.kernel().viewer_hand().len()
        + belief
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let plays_left = u64::try_from(total_plays - 1).expect("fits");
    let root_hand = u64::try_from(root.kernel().viewer_hand().len()).expect("fits");

    let mut states: Vec<ActionState> = actions
        .iter()
        .map(|a| ActionState {
            interval: ActionInterval {
                action: *a,
                z,
                lower: LowerBound::Vacuous,
                upper: UpperBound::Vacuous,
            },
            excluded: false,
        })
        .collect();

    // The finite item universe (§38: exhaustible).
    let mut remaining: Vec<WorkItem> = Vec::new();
    for a in &actions {
        if cfg.prefix >= 1 {
            remaining.push(WorkItem::SampledLower(*a));
            remaining.push(WorkItem::SampledUpper(*a));
        }
        remaining.push(WorkItem::ExactFixed(*a));
        remaining.push(WorkItem::ExactGrammar(*a));
        remaining.push(WorkItem::EscalateExact(*a));
    }
    remaining.push(WorkItem::ConsequenceCensus);
    remaining.sort_by_key(|i| i.key());

    let mut trace: Vec<TraceEvent> = Vec::new();
    let mut refusals_recorded: Vec<(WorkItem, RefusalReason)> = Vec::new();
    let mut spent: u64 = 0;
    let mut risk_entries: Vec<ScopedDelta> = Vec::new();
    let mut risk_spent = BigRational::zero();
    let mut delta_decisive = false;
    let mut starved = false;
    let mut last_bar = BigRational::zero();
    let root_id = belief.root_id();

    let result = loop {
        // §36 steps 4–5: bar, exclusions, the admissible set.
        let (bar, bar_holder, bar_exact) = bar_of(&states);
        assert!(bar >= last_bar, "the bar is monotone (§37)");
        last_bar = bar.clone();
        for s in &mut states {
            if !s.excluded && s.interval.upper_value() < bar {
                s.excluded = true;
                let event_delta = !(s.interval.upper_is_exact() && bar_exact);
                delta_decisive |= event_delta;
                trace.push(TraceEvent::Excluded {
                    action: s.interval.action,
                    bar_holder,
                    delta_decisive: event_delta,
                });
            }
        }
        let survivors: Vec<usize> = (0..states.len()).filter(|i| !states[*i].excluded).collect();
        assert!(!survivors.is_empty(), "the bar holder always survives");
        if survivors.len() == 1 {
            break RefineResult::Settled {
                action: states[survivors[0]].interval.action,
                proof: if delta_decisive {
                    ProofClass::DeltaQualified
                } else {
                    ProofClass::Exact
                },
            };
        }
        if survivors
            .iter()
            .all(|&i| states[i].interval.deterministic_point())
        {
            let mut set = DominoSet::EMPTY;
            for &i in &survivors {
                set.insert(states[i].interval.action);
            }
            break RefineResult::Equivalent {
                actions: set,
                value: bar.clone(),
                proof: if delta_decisive {
                    ProofClass::DeltaQualified
                } else {
                    ProofClass::Exact
                },
            };
        }

        // §34/§35: evaluate every remaining item — refusal, then rank.
        let lowers: Vec<BigRational> = states.iter().map(|s| s.interval.lower_value()).collect();
        let uppers: Vec<BigRational> = states.iter().map(|s| s.interval.upper_value()).collect();
        let excluded_now: Vec<bool> = states.iter().map(|s| s.excluded).collect();
        let (cur_survivors, cur_excess) = decision_width(&lowers, &uppers, &excluded_now);
        let cur_width = width_scalar(cur_survivors, &cur_excess);

        let mut best_pick: Option<(BigRational, u64, WorkItem)> = None;
        let mut refuse_now: Vec<(WorkItem, RefusalReason)> = Vec::new();
        for item in &remaining {
            let item = *item;
            if let Some(a) = item.action() {
                let idx = actions.iter().position(|x| *x == a).expect("a root action");
                if states[idx].excluded {
                    refuse_now.push((item, RefusalReason::ExcludedAction));
                    continue;
                }
                // Best-case hypotheticals over VALUES only (§34): the
                // item's reachable extremes, applied to a copy.
                let mut reduction = BigRational::zero();
                let hyps: Vec<(BigRational, BigRational)> = match item {
                    WorkItem::SampledLower(_)
                    | WorkItem::ExactFixed(_)
                    | WorkItem::ExactGrammar(_) => vec![(uppers[idx].clone(), uppers[idx].clone())],
                    WorkItem::SampledUpper(_) => vec![(lowers[idx].clone(), lowers[idx].clone())],
                    WorkItem::EscalateExact(_) => vec![
                        (uppers[idx].clone(), uppers[idx].clone()),
                        (lowers[idx].clone(), lowers[idx].clone()),
                    ],
                    WorkItem::ConsequenceCensus => unreachable!("census has no action"),
                };
                for (hl, hu) in hyps {
                    let mut l2 = lowers.clone();
                    let mut u2 = uppers.clone();
                    match item {
                        WorkItem::SampledUpper(_) => u2[idx] = hu,
                        WorkItem::EscalateExact(_) => {
                            l2[idx] = hl;
                            u2[idx] = hu;
                        }
                        _ => l2[idx] = hl,
                    }
                    let (hs, he) = decision_width(&l2, &u2, &excluded_now);
                    let hw = width_scalar(hs, &he);
                    if &cur_width - &hw > reduction {
                        reduction = &cur_width - &hw;
                    }
                }
                // §34's width refusal, with one declared exception: an
                // escalation on a surviving point that a SAMPLED side
                // pinned is proof-strengthening work (δ → exact), not
                // width work, and stays runnable at zero rank.
                let proof_strengthens = matches!(item, WorkItem::EscalateExact(_))
                    && !states[idx].interval.deterministic_point();
                if reduction.is_zero() && !proof_strengthens {
                    refuse_now.push((item, RefusalReason::PresentlyUseless));
                    continue;
                }
                let cost = forecast(item, cfg, hands, plays_left, root_hand);
                if cost > cfg.budget - spent {
                    refuse_now.push((item, RefusalReason::ExceedsBudget));
                    starved = true;
                    continue;
                }
                if matches!(item, WorkItem::SampledLower(_) | WorkItem::SampledUpper(_))
                    && &risk_spent + &cfg.delta > cfg.scope_budget
                {
                    refuse_now.push((item, RefusalReason::ExceedsRiskScope));
                    starved = true;
                    continue;
                }
                // §35 rank: reduction / cost, exact cross-multiplied;
                // deterministic key breaks ties.
                let better = match &best_pick {
                    None => true,
                    Some((br, bc, bi)) => {
                        let lhs = &reduction * BigRational::from_integer(BigInt::from(*bc));
                        let rhs = br * BigRational::from_integer(BigInt::from(cost));
                        lhs > rhs || (lhs == rhs && item.key() < bi.key())
                    }
                };
                if better {
                    best_pick = Some((reduction, cost, item));
                }
            } else {
                // ConsequenceCensus: best case narrows field branch
                // intervals, never a root bound — declared zero width
                // reduction, refused by §34 at every bar.
                refuse_now.push((item, RefusalReason::PresentlyUseless));
            }
        }
        for (item, reason) in refuse_now {
            if !refusals_recorded.contains(&(item, reason)) {
                refusals_recorded.push((item, reason));
                trace.push(TraceEvent::Refused { item, reason });
            }
        }

        let Some((_, cost, item)) = best_pick else {
            // No runnable item. Starvation ends honestly (§36 step 12);
            // otherwise every item is presently useless while several
            // non-point survivors remain — unreachable, but the honest
            // branch is the same surviving set.
            let mut set = DominoSet::EMPTY;
            for &i in &survivors {
                set.insert(states[i].interval.action);
            }
            let fallback = set.iter().next().expect("a survivor");
            let _ = starved;
            break RefineResult::Unresolved {
                survivors: set,
                fallback,
                rule: "lowest-tile-among-survivors",
            };
        };

        // Run the item (§36 steps 6–10, one at a time).
        let a = item.action().expect("runnable items carry an action");
        let idx = actions.iter().position(|x| *x == a).expect("a root action");
        let lower_before = states[idx].interval.lower_value();
        let upper_before = states[idx].interval.upper_value();
        match item {
            WorkItem::SampledLower(_) => {
                let scope =
                    ScopedDelta::new(format!("refine-{root_id}/{a}/lower"), cfg.delta.clone());
                risk_entries.push(scope.clone());
                risk_spent += cfg.delta.clone();
                let policy = pinned_level1(position, a);
                let l = frozen_policy_lower(
                    root,
                    position,
                    &policy,
                    &field_sampled,
                    PolicyProvenance::Fixed,
                    1,
                    cfg.prefix,
                    scope,
                );
                assert_eq!(
                    l.action, a,
                    "the pinned witness plays its pinned root action"
                );
                install_lower(&mut states[idx].interval, LowerBound::Sampled(l));
            }
            WorkItem::SampledUpper(_) => {
                let scope =
                    ScopedDelta::new(format!("refine-{root_id}/{a}/upper"), cfg.delta.clone());
                risk_entries.push(scope.clone());
                risk_spent += cfg.delta.clone();
                let u = pmake_empirical_max_upper(
                    root,
                    position,
                    a,
                    &field_sampled,
                    0,
                    cfg.prefix,
                    scope,
                );
                install_upper(&mut states[idx].interval, UpperBound::Sampled(u));
            }
            WorkItem::ExactFixed(_) => {
                let child = belief.focal_play(a);
                assert_eq!(oracle.mass(&child), z, "a focal play changes no factor");
                let mut stats = RecursionStats::default();
                let mass = viewer_success_mass(oracle, &child, &low, &field_factor, &mut stats);
                install_lower(
                    &mut states[idx].interval,
                    LowerBound::ExactPolicy {
                        mass,
                        policy: low.id().to_string(),
                    },
                );
            }
            WorkItem::ExactGrammar(_) => {
                let child = belief.focal_play(a);
                let mut stats = ResponseStats::default();
                let mass =
                    grammar_success_mass(oracle, &child, &grammar, &field_factor, &mut stats);
                install_lower(
                    &mut states[idx].interval,
                    LowerBound::ExactGrammar {
                        mass,
                        grammar: grammar.id().to_string(),
                    },
                );
            }
            WorkItem::EscalateExact(_) => {
                let child = belief.focal_play(a);
                let mut stats = ResponseStats::default();
                let mass = response_success_mass(oracle, &child, &field_factor, &mut stats);
                let q = ratio(mass, z);
                assert!(
                    states[idx].interval.lower_value() <= q,
                    "a valid lower never exceeds the exact value — a sampled \
                     violation is the declared δ event on this stream"
                );
                assert!(
                    states[idx].interval.upper_value() >= q,
                    "a valid upper never falls below the exact value — a sampled \
                     violation is the declared δ event on this stream"
                );
                states[idx].interval.lower = LowerBound::ExactResponse { mass };
                states[idx].interval.upper = UpperBound::ExactResponse { mass };
            }
            WorkItem::ConsequenceCensus => unreachable!("refused at every bar"),
        }
        spent += cost;
        remaining.retain(|i| *i != item);

        // §37 monotone check, then the trace entry.
        let lower_after = states[idx].interval.lower_value();
        let upper_after = states[idx].interval.upper_value();
        assert!(lower_after >= lower_before, "lowers only rise (§37)");
        assert!(upper_after <= upper_before, "uppers only fall (§37)");
        assert!(lower_after <= upper_after, "an interval stays an interval");
        let (bar_now, _, _) = bar_of(&states);
        let mut surv_now = DominoSet::EMPTY;
        for s in &states {
            if !s.excluded && s.interval.upper_value() >= bar_now {
                surv_now.insert(s.interval.action);
            }
        }
        trace.push(TraceEvent::Ran(Box::new(RanEvent {
            item,
            cost,
            lower_before,
            upper_before,
            lower_after,
            upper_after,
            bar_after: bar_now,
            survivors_after: surv_now,
        })));
    };

    // The ledger, re-asserted through the shared arithmetic (§37.4).
    let scope = ScopedDelta::new(format!("refine-{root_id}/root"), cfg.scope_budget.clone());
    let entries: Vec<&ScopedDelta> = risk_entries.iter().collect();
    let total = assert_screen_risk_allocation(&scope, &entries);
    assert_eq!(total, risk_spent, "the ledger sum is the running sum");

    let (bar, _, _) = bar_of(&states);
    let mut survivors = DominoSet::EMPTY;
    let mut excluded = DominoSet::EMPTY;
    for s in &states {
        if s.excluded {
            excluded.insert(s.interval.action);
        } else if s.interval.upper_value() >= bar {
            survivors.insert(s.interval.action);
        }
    }
    RefineOutcome {
        intervals: states.into_iter().map(|s| s.interval).collect(),
        excluded,
        survivors,
        bar,
        result,
        trace,
        work_spent: spent,
        budget: cfg.budget,
        risk_spent,
        delta_decisive,
    }
}

/// Install a candidate lower iff it improves: strictly higher value, or
/// the same value under a strictly stronger proof (exact over sampled or
/// vacuous). Anything else is dropped — bounds never regress (§37).
fn install_lower(interval: &mut ActionInterval, cand: LowerBound) {
    let cand_interval = ActionInterval {
        action: interval.action,
        z: interval.z,
        lower: cand.clone(),
        upper: UpperBound::Vacuous,
    };
    let cv = cand_interval.lower_value();
    let cur = interval.lower_value();
    if cv > cur || (cv == cur && cand_interval.lower_is_exact() && !interval.lower_is_exact()) {
        interval.lower = cand;
    }
}

/// The upper twin of [`install_lower`].
fn install_upper(interval: &mut ActionInterval, cand: UpperBound) {
    let cand_interval = ActionInterval {
        action: interval.action,
        z: interval.z,
        lower: LowerBound::Vacuous,
        upper: cand.clone(),
    };
    let cv = cand_interval.upper_value();
    let cur = interval.upper_value();
    if cv < cur || (cv == cur && cand_interval.upper_is_exact() && !interval.upper_is_exact()) {
        interval.upper = cand;
    }
}
