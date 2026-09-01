//! The §39–§43 work frontier (anytime proof-state Phase 1, the Part IX
//! half — the §58 skeleton itself is the §49 spike, already landed):
//! declared solve goals, work items as proof transformers with §42
//! safe steering bounds, §41 closure-aware selection with the declared
//! macro-plan, and §43's containment of the exact solve as a
//! degenerate schedule. `Frontier::advance` is the anytime loop: at
//! each step it enumerates the candidate items, refuses the ones whose
//! declared value bound is zero for the goal (§34's refusal doctrine
//! as amended by §41), buys the best bound-per-forecast-cost item it
//! can afford, installs the item's facts through the ordinary
//! `ProofState::install` fence, recomputes the closure, and asserts
//! the §42 law — the realized debt reduction never exceeds the
//! declared bound. It stops with `met` (debt zero), an honest refusal
//! (no candidate has positive potential), or budget exhaustion —
//! never a weakened state (§43: a poor forecast wastes time; it
//! cannot weaken the proof state).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §39 (declared solve goals), §40 (work items as proof
//! transformers), §41 (closure-aware usefulness and the macro-plan),
//! §42 (upper bounds on possible work value), §43 (iteration contains
//! the exact solve), §35 (width-per-cost selection), under ruling
//! APS-A8 (closure-aware usefulness amends the §34 steering) and
//! APS-A9 (phases in-crate, RefineV1 frozen).
//!
//! THE §41 MACRO-PLAN IS LOAD-BEARING FROM THE FIRST STEP. From the
//! top state every per-action `ExactValue` has a PROVABLY zero
//! standalone effect on `U*` — lowering one action's upper cannot
//! lower the max while any other upper is vacuous — so a scheduler
//! without the declared macro (`ExactValueSurvivors`, all surviving
//! actions as one candidate) stalls on the upper side with every
//! individually-correct refusal. §41's sentence "this prevents a
//! necessary first step from being refused because it has zero
//! standalone root-width effect" is not advice here; it is the only
//! way the upper side ever moves.
//!
//! COSTS ARE DECLARED FORECASTS, NOT MEASUREMENTS. The unit is the
//! root fiber mass `Z` per full fixed-policy walk, `3Z` per
//! max-recursion walk (extraction, exact value) — a crude declared
//! cost model in the §40 sense, recorded as such. Budgets are in the
//! same units. Nothing here reads a clock.
//!
//! WHAT THIS FRONTIER IS NOT. No sampled tier: every item here is
//! deterministic (RefineV1's two-tier ladder stays behind its own
//! frozen interface; importing its facts is the caller's move, not a
//! work item). No cross-root anything, no cost-model learning, no
//! laydown/count goals (Phases 5/7 own those), and
//! `PriceRecommendedPolicy`/`ProveLaydown`/`ExplainCountRisk` from
//! the §39 list are unbuilt — the four goals below are the ones whose
//! debts today's fact types can move. New-core beside
//! `solver::proof_state` and `solver::extraction` (§47's greenfield
//! boundary): imported by nothing but the crate root, deletable with
//! its siblings without touching RefineV1 or the recursions (§67.10).

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use crate::solver::extraction::extraction_fact_for_action;
use crate::solver::factor_belief::{
    response_success_mass, viewer_score_profile, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats,
};
use crate::solver::proof_state::{
    BoundFact, BoundSide, ClosureReport, Fact, ProofState, ProofTag, ScoreProfileFact, StateResult,
};

/// The declared baseline policy id (the σ0 tie-rule house policy, the
/// same id the Phase 3 probe used — one vocabulary).
const BASELINE_POLICY_ID: &str = "lowest-first-after-root-action";

/// The declared authority of a §36 exact-value fact: the full-legal
/// best-response optimum as an upper AND a proof-bar-only lower (the
/// §30 split — the value is attainable, but nothing executable exists
/// until extraction materializes it).
const EXACT_AUTHORITY: &str = "response-exact-v1";

/// A §39 declared solve goal. One primary goal per `advance` call —
/// unlike objectives are never mixed into one scalar (§39's fence).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveGoal {
    /// Settle the root action: one survivor, or an exact tie class
    /// (§28 — `Settled` and `Equivalent` both satisfy it).
    SelectAction,
    /// Certify `Γ = U* − B_exec ≤ ε`: an executable recommendation
    /// with certified regret at most epsilon.
    RecommendEpsilonPolicy { epsilon: BigRational },
    /// Every surviving interval a point: `L_a = U_a` on the survivor
    /// set (the §43 degenerate schedule's own goal).
    StrengthenToExact,
    /// A score profile fact for every legal root action.
    ComputeFullScoreProfile,
}

impl SolveGoal {
    /// The goal's debt (§39): an exact nonnegative rational, zero
    /// exactly when the goal is met. Unlike goals have unlike units
    /// (survivor counts, regret mass, interval width, missing
    /// profiles) — the § 39 fence means they are never compared, only
    /// each with its own zero.
    pub fn debt(&self, state: &ProofState, report: &ClosureReport) -> BigRational {
        match self {
            SolveGoal::SelectAction => match &report.result {
                StateResult::Settled { .. } | StateResult::Equivalent { .. } => BigRational::zero(),
                StateResult::Unresolved { survivors } => {
                    BigRational::from_integer(BigInt::from(survivors.len() as u64 - 1))
                }
            },
            SolveGoal::RecommendEpsilonPolicy { epsilon } => {
                let gap = &report.certified_regret - epsilon;
                if gap > BigRational::zero() {
                    gap
                } else {
                    BigRational::zero()
                }
            }
            SolveGoal::StrengthenToExact => report
                .views
                .iter()
                .filter(|v| !v.excluded)
                .map(|v| &v.upper - &v.lower)
                .fold(BigRational::zero(), |a, w| a + w),
            SolveGoal::ComputeFullScoreProfile => {
                let missing = state
                    .legal
                    .iter()
                    .filter(|a| !has_profile(state, **a))
                    .count();
                BigRational::from_integer(BigInt::from(missing as u64))
            }
        }
    }
}

/// A §40 work item. Each kind names its produced fact type and proof
/// class in the docs; scope is one root action (or the survivor set
/// for the §41 macro); the proof class of every item here is
/// deterministic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorkItem {
    /// The declared baseline profile after `action` (lowest-first
    /// continuation): a cheap executable lower. Cost `Z`.
    BaselineProfile { action: Domino },
    /// The §36 exact value of `action`: upper and proof-bar-only
    /// lower at the exact best-response optimum. Cost `3Z`.
    ExactValue { action: Domino },
    /// The §63 argmax extraction of `action`: the executable profile
    /// at the exact optimum. Cost `3Z`.
    ExtractArgmax { action: Domino },
    /// The §41 declared macro-plan: `ExactValue` on EVERY current
    /// survivor as one candidate, so the upper side can move at all
    /// (module doc). Cost `3Z · |survivors|`.
    ExactValueSurvivors,
}

/// Why a candidate was not bought this step (§34 refusal doctrine,
/// §41-amended: potential is evaluated on the CLOSED state).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The §42 bound says the item cannot reduce this goal's debt.
    ZeroPotential,
    /// The item's facts are already in the state.
    AlreadyPresent,
    /// The forecast cost exceeds the remaining budget.
    Unaffordable,
}

/// One executed step of the schedule: the item, its declared forecast
/// cost, its declared §42 bound, and the realized debt movement (the
/// §42 law `debt_before − debt_after ≤ bound` is asserted at
/// execution and gated).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutedItem {
    pub item: WorkItem,
    pub cost: u128,
    pub bound: BigRational,
    pub debt_before: BigRational,
    pub debt_after: BigRational,
}

/// The `advance` outcome: the schedule as executed, the terminal
/// refusals (recorded only for the final step, where nothing was
/// bought), the exact spend, and the honest verdict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrontierReport {
    pub executed: Vec<ExecutedItem>,
    /// The final step's refusals, one per enumerated candidate, in
    /// the declared candidate order — present exactly when the loop
    /// stopped without meeting the goal by buying.
    pub refusals: Vec<(WorkItem, Refusal)>,
    pub spent: u128,
    pub debt: BigRational,
    pub met: bool,
}

fn has_profile(state: &ProofState, action: Domino) -> bool {
    state
        .facts()
        .iter()
        .any(|sf| matches!(&sf.fact, Fact::Profile(p) if p.action == action))
}

fn has_baseline(state: &ProofState, action: Domino) -> bool {
    state.facts().iter().any(
        |sf| matches!(&sf.fact, Fact::Profile(p) if p.action == action && p.policy_id == BASELINE_POLICY_ID),
    )
}

fn has_exact(state: &ProofState, action: Domino) -> bool {
    state.facts().iter().any(|sf| {
        matches!(&sf.fact, Fact::Bound(b) if b.action == action && b.authority == EXACT_AUTHORITY && b.side == BoundSide::Upper)
    })
}

fn has_argmax(state: &ProofState, action: Domino) -> bool {
    state.facts().iter().any(
        |sf| matches!(&sf.fact, Fact::Profile(p) if p.action == action && p.policy_id.starts_with("argmax-")),
    )
}

/// The Phase 1 frontier over one root. Holds the same evaluation
/// context as the §63 producer; every fact it installs goes through
/// the ordinary identity fence.
pub struct Frontier<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn crate::solver::adaptive::SlicePolicy,
}

impl Frontier<'_> {
    /// The declared forecast cost of one item, in fiber-walk units
    /// (module doc: `Z` per fixed-policy walk, `3Z` per max walk).
    fn forecast(&self, item: &WorkItem, report: &ClosureReport) -> u128 {
        let z = self.root_mass();
        match item {
            WorkItem::BaselineProfile { .. } => z,
            WorkItem::ExactValue { .. } | WorkItem::ExtractArgmax { .. } => {
                3u128.checked_mul(z).expect("a forecast fits u128")
            }
            WorkItem::ExactValueSurvivors => 3u128
                .checked_mul(z)
                .and_then(|c| c.checked_mul(report.survivors.len() as u128))
                .expect("a forecast fits u128"),
        }
    }

    fn root_mass(&self) -> u128 {
        let belief = FactorBelief::uniform_root(self.root, self.position, self.field);
        self.oracle.mass(&belief)
    }

    /// The §42 safe steering bound: an upper bound on how much this
    /// item can reduce this goal's debt, as an exact rational. Zero
    /// is a refusal. Conservative is lawful (§42: "they may be
    /// conservative"); exceeding the realized reduction never is
    /// (asserted at execution).
    fn potential(
        &self,
        item: &WorkItem,
        goal: &SolveGoal,
        state: &ProofState,
        report: &ClosureReport,
        debt: &BigRational,
    ) -> BigRational {
        let zero = BigRational::zero();
        if *debt == zero {
            return zero;
        }
        let view = |a: Domino| report.views.iter().find(|v| v.action == a).expect("legal");
        match goal {
            SolveGoal::SelectAction => {
                // Debt is survivor count − 1. A lower-raiser on `a`
                // can raise the bar to at most `U_a`, excluding
                // others; an upper on `a` can exclude `a` and its
                // lower can raise the bar. Conservative integer
                // bounds; zero when the move provably cannot bite.
                let all = debt.clone();
                match item {
                    WorkItem::BaselineProfile { action } | WorkItem::ExtractArgmax { action } => {
                        let v = view(*action);
                        if v.upper > report.bar {
                            all
                        } else {
                            zero
                        }
                    }
                    WorkItem::ExactValue { action } => {
                        let v = view(*action);
                        if v.excluded {
                            zero
                        } else {
                            all
                        }
                    }
                    WorkItem::ExactValueSurvivors => all,
                }
            }
            SolveGoal::RecommendEpsilonPolicy { .. } => {
                // Debt is max(0, Γ − ε). Lower-raisers move B_exec by
                // at most U_a − B_exec; exact uppers move U* by at
                // most U* − max(L_a, max_{b≠a} U_b) — provably ZERO
                // while any other upper matches U* (the §41 stall the
                // macro exists for).
                let exec = report
                    .exec
                    .as_ref()
                    .map(|w| w.value.clone())
                    .unwrap_or_else(BigRational::zero);
                let cap = |x: BigRational| if x > *debt { debt.clone() } else { x };
                match item {
                    WorkItem::BaselineProfile { action } | WorkItem::ExtractArgmax { action } => {
                        let v = view(*action);
                        if v.upper > exec {
                            cap(&v.upper - &exec)
                        } else {
                            zero
                        }
                    }
                    WorkItem::ExactValue { action } => {
                        let v = view(*action);
                        let others = report
                            .views
                            .iter()
                            .filter(|w| w.action != *action)
                            .map(|w| w.upper.clone())
                            .max()
                            .unwrap_or_else(BigRational::zero);
                        let floor = if v.lower > others {
                            v.lower.clone()
                        } else {
                            others
                        };
                        if report.u_star > floor {
                            cap(&report.u_star - &floor)
                        } else {
                            zero
                        }
                    }
                    WorkItem::ExactValueSurvivors => {
                        if report.u_star > report.bar {
                            cap(&report.u_star - &report.bar)
                        } else {
                            zero
                        }
                    }
                }
            }
            SolveGoal::StrengthenToExact => {
                // Debt is the surviving width sum. Any per-action item
                // can shrink `a`'s own width by at most `U_a − L_a`,
                // AND — because a raised lower can lift the bar to at
                // most `U_a` — newly exclude only survivors whose
                // upper sits strictly below `U_a`, removing their
                // widths too. The sound bound is the sum of both
                // parts; conservative is lawful, undershooting is not
                // (asserted at execution).
                let per_action = |a: Domino| {
                    let v = view(a);
                    if v.excluded {
                        return zero.clone();
                    }
                    let own = &v.upper - &v.lower;
                    let excludable = report
                        .views
                        .iter()
                        .filter(|w| !w.excluded && w.action != a && w.upper < v.upper)
                        .map(|w| &w.upper - &w.lower)
                        .fold(BigRational::zero(), |acc, w| acc + w);
                    own + excludable
                };
                match item {
                    WorkItem::ExactValue { action }
                    | WorkItem::BaselineProfile { action }
                    | WorkItem::ExtractArgmax { action } => per_action(*action),
                    WorkItem::ExactValueSurvivors => debt.clone(),
                }
            }
            SolveGoal::ComputeFullScoreProfile => match item {
                WorkItem::BaselineProfile { action } | WorkItem::ExtractArgmax { action } => {
                    if has_profile(state, *action) {
                        zero
                    } else {
                        BigRational::from_integer(BigInt::from(1))
                    }
                }
                WorkItem::ExactValue { .. } | WorkItem::ExactValueSurvivors => zero,
            },
        }
    }

    /// The candidate list, in the declared deterministic order: per
    /// action ascending tile index — baseline, exact, extract — then
    /// the §41 macro last.
    fn candidates(&self, state: &ProofState) -> Vec<WorkItem> {
        let mut out = Vec::new();
        for a in &state.legal {
            out.push(WorkItem::BaselineProfile { action: *a });
            out.push(WorkItem::ExactValue { action: *a });
            out.push(WorkItem::ExtractArgmax { action: *a });
        }
        out.push(WorkItem::ExactValueSurvivors);
        out
    }

    fn already_present(&self, item: &WorkItem, state: &ProofState, report: &ClosureReport) -> bool {
        match item {
            WorkItem::BaselineProfile { action } => has_baseline(state, *action),
            WorkItem::ExactValue { action } => has_exact(state, *action),
            WorkItem::ExtractArgmax { action } => has_argmax(state, *action),
            WorkItem::ExactValueSurvivors => report.survivors.iter().all(|a| has_exact(state, *a)),
        }
    }

    /// Execute one item: compute its facts and install them through
    /// the ordinary fence.
    fn execute(&self, item: &WorkItem, state: &mut ProofState, report: &ClosureReport) {
        let identity = state.identity.clone();
        match item {
            WorkItem::BaselineProfile { action } => {
                let low = FixedPreference::lowest_first("focal:lowest-first");
                let child = FactorBelief::uniform_root(self.root, self.position, self.field)
                    .focal_play(*action);
                let mut ps = RecursionStats::default();
                let profile = viewer_score_profile(self.oracle, &child, &low, self.field, &mut ps);
                state
                    .install(
                        &identity,
                        Fact::Profile(Box::new(ScoreProfileFact {
                            action: *action,
                            policy_id: BASELINE_POLICY_ID.to_string(),
                            bins: profile.bins,
                        })),
                    )
                    .expect("a baseline profile installs");
            }
            WorkItem::ExactValue { action } => self.install_exact(state, *action),
            WorkItem::ExtractArgmax { action } => {
                let fact = extraction_fact_for_action(
                    self.oracle,
                    self.root,
                    self.position,
                    self.field,
                    identity.contract,
                    &identity.utility_id,
                    *action,
                );
                state
                    .install(&identity, fact)
                    .expect("an extraction fact installs");
            }
            WorkItem::ExactValueSurvivors => {
                for a in &report.survivors {
                    if !has_exact(state, *a) {
                        self.install_exact(state, *a);
                    }
                }
            }
        }
    }

    fn install_exact(&self, state: &mut ProofState, action: Domino) {
        let identity = state.identity.clone();
        let child =
            FactorBelief::uniform_root(self.root, self.position, self.field).focal_play(action);
        let mut rs = ResponseStats::default();
        let exact = response_success_mass(self.oracle, &child, self.field, &mut rs);
        let z = self.oracle.mass(&child);
        let value = BigRational::new(BigInt::from(exact), BigInt::from(z));
        state
            .install(
                &identity,
                Fact::Bound(BoundFact::upper(
                    action,
                    value.clone(),
                    EXACT_AUTHORITY,
                    ProofTag::Deterministic,
                )),
            )
            .expect("an exact upper installs");
        // The §30 split, kept: the optimum is attainable, so it is a
        // valid LOWER too — but nothing executable exists until
        // extraction materializes the argmax, so executable = false.
        state
            .install(
                &identity,
                Fact::Bound(BoundFact::lower(
                    action,
                    value,
                    EXACT_AUTHORITY,
                    false,
                    ProofTag::Deterministic,
                )),
            )
            .expect("an exact lower installs");
    }

    /// Execute one item outside the loop, closing first — the entry
    /// the refusal-honesty gate uses (executing a refused
    /// zero-potential item must move its goal's debt by exactly
    /// nothing) and the probe's manual steps.
    pub fn execute_item(&self, item: &WorkItem, state: &mut ProofState) {
        let report = state.closure();
        self.execute(item, state, &report);
    }

    /// The §35/§41/§42 anytime loop (module doc). Deterministic:
    /// candidates in declared order, best bound-per-cost by exact
    /// cross-multiplied comparison, first-in-order on ties.
    pub fn advance(
        &self,
        state: &mut ProofState,
        goal: &SolveGoal,
        budget: u128,
    ) -> FrontierReport {
        assert_eq!(
            state.identity.root_id,
            root_identity(self.root, self.position),
            "the frontier's context is the state's root"
        );
        let mut executed = Vec::new();
        let mut spent: u128 = 0;
        // Termination: each purchase installs facts its presence
        // guards then refuse; the candidate set is finite.
        let ceiling = 3 * state.legal.len() + 1;
        loop {
            assert!(
                executed.len() <= ceiling,
                "the presence guards bound the schedule length"
            );
            let report = state.closure();
            let debt = goal.debt(state, &report);
            if debt == BigRational::zero() {
                return FrontierReport {
                    executed,
                    refusals: Vec::new(),
                    spent,
                    debt,
                    met: true,
                };
            }
            let mut refusals = Vec::new();
            let mut best: Option<(WorkItem, u128, BigRational)> = None;
            for item in self.candidates(state) {
                if self.already_present(&item, state, &report) {
                    refusals.push((item, Refusal::AlreadyPresent));
                    continue;
                }
                let bound = self.potential(&item, goal, state, &report, &debt);
                if bound == BigRational::zero() {
                    refusals.push((item, Refusal::ZeroPotential));
                    continue;
                }
                let cost = self.forecast(&item, &report);
                if spent.checked_add(cost).expect("spend fits u128") > budget {
                    refusals.push((item, Refusal::Unaffordable));
                    continue;
                }
                // bound/cost comparison by exact cross-multiplication;
                // strict improvement only — first in order wins ties.
                let better = match &best {
                    None => true,
                    Some((_, bc, bb)) => {
                        &bound * BigRational::from_integer(BigInt::from(*bc))
                            > bb * BigRational::from_integer(BigInt::from(cost))
                    }
                };
                if better {
                    best = Some((item, cost, bound));
                }
            }
            let Some((item, cost, bound)) = best else {
                return FrontierReport {
                    executed,
                    refusals,
                    spent,
                    debt,
                    met: false,
                };
            };
            self.execute(&item, state, &report);
            spent += cost;
            let after = state.closure();
            let debt_after = goal.debt(state, &after);
            // The §42 law: the realized reduction never exceeds the
            // declared bound (and debt never rises — §9 monotone
            // refinement through the closure).
            assert!(
                debt_after <= debt,
                "monotone refinement: installing facts never raises a goal debt"
            );
            assert!(
                &debt - &debt_after <= bound,
                "the §42 law: realized reduction within the declared bound"
            );
            executed.push(ExecutedItem {
                item,
                cost,
                bound,
                debt_before: debt,
                debt_after,
            });
        }
    }
}
