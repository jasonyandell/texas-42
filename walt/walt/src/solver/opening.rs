//! The §65 opening-root iterative run (anytime proof-state Phase 8 —
//! the parent program's last phase): one root, one append-only
//! [`ProofState`], a LADDER of declared budget stops, and the complete
//! §65 report panel at every stop. Each stop executes the §65 steps in
//! their declared order against the same state:
//!
//! 1. the zero-budget top state is the CALLER's `ProofState::open`
//!    (§25: every action alive at `[0, 1]` — the initial theorem, not
//!    a failure);
//! 2. zero-cost closure is recomputed inside every panel (§26);
//! 3. sampled root bounds import at the stop's declared prefix — the
//!    Slice A endpoints (`frozen_policy_lower`,
//!    `pmake_empirical_max_upper`) under FRESH per-stop δ scopes,
//!    adapted through the ordinary §48 adapter
//!    [`facts_from_refine_interval`]: the pinned-level1 witness IS the
//!    §65 "cheap executable policy" (its sampled lower is executable —
//!    a materialized lawful policy);
//! 4. score profiles, partial F refinement of root bounds, residual
//!    Bellman intervals, and highest-debt refinement are the FRONTIER's
//!    work items (§39–§43), bought or refused honestly at the stop's
//!    declared Z-budget — at the opening root every exact item stands
//!    on the far side of the affordability cliff and the §34 refusals
//!    are the finding, exactly as RefineV1's Section D recorded;
//! 5. count-threat covers apply through their producer AFTER the
//!    frontier pass (a cover is relative to an incumbent profile — the
//!    §62 decline path is the honest result while no profile exists);
//! 6. the §49 consequence census runs as a REPORTED COORDINATE at the
//!    Phase 4 declared stage ([`RESIDUAL_STAGE`]) — it narrows field
//!    branch intervals, never a root bound, so it installs nothing
//!    (RefineV1's `ConsequenceCensus` doctrine, kept).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §65 (the run and its report list), §25–§31 (states, closure, bars,
//! certified regret), §33–§35/§39–§43 (items, refusals, steering),
//! §48 (the import adapter), §49 (the census), §66.3/§66.14 (sampled
//! provenance; exhaustion returns a sound state and never manufactures
//! a winner), §67.4–5 (deterministic serialization; resume ≡
//! uninterrupted refinement — gated), under ruling APS-A9 (phases
//! in-crate, RefineV1 frozen).
//!
//! THE VERDICT TYPES (§65's last report line). `Exact` — the closure
//! settled or tied exactly with no δ-decisive comparison;
//! `DeltaQualified` — settled or tied through a sampled side;
//! `EpsilonOptimal` — an executable witness exists and
//! `Γ = U* − B_exec ≤ ε` at the ladder's declared ε; `Unresolved` —
//! the honest surviving set (§37.9: the fallback is the
//! recommendation's named policy, never promoted).
//!
//! THE FIRST TARGET (§65, verbatim): not a seven-trick exact opening
//! solution — a materially smaller correct survivor set or a useful
//! certified-regret recommendation under a playable budget.
//!
//! WHAT THIS DRIVER IS NOT. It reads no clock (wall time is the
//! probe's column); it manufactures no bound (§37.8 — every number
//! comes from a producing authority that predates it); it runs no
//! laydown walk (§65 lists none, and the §16 universal walk is an
//! endgame instrument — the opening-depth structural producer is
//! declared future work at Phase 7's boundary); it never modifies
//! `solver::refine` (frozen at freeze 58 — the Slice A witness
//! declaration is RESTATED below as configuration, not reached through
//! the frozen controller). New-core beside `solver::proof_state` and
//! its siblings (§47): imported by nothing but the crate root,
//! deletable as one boundary (§67.10).

use num_rational::BigRational;
use num_traits::Zero;

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::covers::CountThreatProducer;
use crate::solver::evidence::ScopedDelta;
use crate::solver::factor_belief::{refine_to_action_exact, ExactCoverOracle, FactorBelief};
use crate::solver::field::FieldModel;
use crate::solver::frontier::{Frontier, FrontierReport, SolveGoal};
use crate::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use crate::solver::proof_state::{
    facts_from_refine_interval, Fact, ProofProducer, ProofState, ProofTag, Recommendation,
};
use crate::solver::refine::{ActionInterval, LowerBound, UpperBound};
use crate::solver::residual::RESIDUAL_STAGE;
use crate::solver::root_interval::{
    frozen_policy_lower, pmake_empirical_max_upper, PolicyProvenance,
};
use crate::solver::upper_cs::assert_screen_risk_allocation;

/// The declared sampled-stream epochs — the Slice A/RefineV1
/// declaration, kept: the optimization-lock upper reads epoch 0, the
/// pinned-witness evaluation reads epoch 1 (distinct streams, so the
/// lock never evaluates on its own selection worlds).
const UPPER_EPOCH: u64 = 0;
const EVAL_EPOCH: u64 = 1;

/// One declared budget stop of the §65 ladder.
#[derive(Clone, Debug)]
pub struct OpeningStopSpec {
    pub label: String,
    /// Sampled-tier prefix for this stop's import; 0 = no sampled
    /// work at this stop. Each stop's endpoints consume fresh δ
    /// scopes named by this prefix, so re-running a stop against a
    /// resumed state imports nothing twice (the presence guard).
    pub sampled_prefix: u64,
    /// δ per sampled endpoint at this stop (each endpoint a distinct
    /// scope; the ladder's total is asserted against
    /// [`OpeningLadder::scope_budget`] at every stop).
    pub endpoint_delta: BigRational,
    /// Run the §49 consequence census at this stop (a reported
    /// coordinate — installs nothing).
    pub census: bool,
    /// The stop's frontier budget in the §40 Z units (0 = no frontier
    /// pass). At the opening root any budget below `Z` buys nothing —
    /// the recorded refusals are the §34 finding.
    pub frontier_budget: u128,
}

/// The §49 census reading at one root action's first field decision:
/// the [`RESIDUAL_STAGE`] row of the CEGAR record (or the endpoint,
/// when the loop finishes earlier).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CensusReading {
    pub action: Domino,
    /// The stage actually read (min of [`RESIDUAL_STAGE`] and the
    /// last stage).
    pub stage: usize,
    pub classes: u64,
    pub exact_classes: u64,
    /// Mass in action-uniform classes — §65's "exact F mass".
    pub exact_mass: u128,
    /// §65's "unresolved F mass" (`exact + residual = Z` per stage).
    pub residual_mass: u128,
}

/// §65's last report line: what kind of result this stop holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopVerdict {
    Exact,
    DeltaQualified,
    EpsilonOptimal,
    Unresolved,
}

/// The §65 report panel at one stop — every quantity a derived view of
/// the proof state at the stop's end (wall time is the probe's).
/// `PartialEq` is load-bearing: the §67.5 resume gate compares a
/// resumed stop's panel against the uninterrupted one field by field.
#[derive(Clone, Debug, PartialEq)]
pub struct StopReport {
    pub label: String,
    /// Facts in the store after this stop.
    pub facts: usize,
    pub survivors: Vec<Domino>,
    pub excluded: Vec<Domino>,
    pub proof_bar: BigRational,
    /// `B_exec` (the vacuous 0 at zero executable work).
    pub exec_bar: BigRational,
    pub global_upper: BigRational,
    pub certified_regret: BigRational,
    /// The §7 straddle of the recommendation's witness (profile
    /// witnesses report exactly 0; bound-fact witnesses none).
    pub contract_sensitive_residual: Option<BigRational>,
    /// The §39 `StrengthenToExact` debt: the surviving pmake width
    /// sum (score-unit width debt awaits score-interval fact kinds —
    /// reported in the objective's own units, never converted).
    pub width_debt: BigRational,
    pub census: Vec<CensusReading>,
    /// Materialized-policy fact families in the store, counted by
    /// distinct (action, witness identity): executable bound lowers
    /// by authority, profiles and envelopes by policy id.
    pub policy_cylinders: usize,
    /// Cover facts in the store (§65's "count-threat cells").
    pub count_threat_cells: usize,
    /// The exact rational δ sum over every sampled fact's scope —
    /// recomputed FROM THE STORE (a derived view, like everything
    /// else) and asserted against the declared root scope budget.
    pub risk_spent: BigRational,
    pub risk_scopes: Vec<String>,
    /// Sampled work charged at THIS stop, in RefineV1's declared
    /// forecast unit (prefix per endpoint). Never mixed with the
    /// frontier's Z units (§39's fence: unlike units are never
    /// compared).
    pub sampled_work: u64,
    /// The stop's frontier pass, when one ran: executed purchases,
    /// terminal refusals, exact Z spend.
    pub frontier: Option<FrontierReport>,
    /// Covers installed by this stop's producer pass.
    pub covers_installed: usize,
    pub recommendation: Option<Recommendation>,
    pub delta_decisive: bool,
    pub verdict: StopVerdict,
}

/// The Phase 8 ladder over one root: the evaluation context every stop
/// shares. The field serves both declared roles (the sampled replays
/// and the factor recursions) under one identity.
pub struct OpeningLadder<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a FieldModel,
    /// The declared root risk scope: every sampled endpoint's δ sums
    /// against this budget (asserted through the shared arithmetic at
    /// every stop — no new ledger).
    pub scope_budget: BigRational,
    /// The declared ε of the `EpsilonOptimal` verdict.
    pub epsilon: BigRational,
}

/// The declared δ-scope name of one sampled endpoint: root, stop
/// prefix, action, side — distinct per stop so successive stops
/// consume distinct risk and the presence guard can see a stop in a
/// resumed store.
pub fn opening_scope(root_id: u64, prefix: u64, action: Domino, side: &str) -> String {
    format!("opening-{root_id}/p{prefix}/{action}/{side}")
}

/// The Slice A pinned witness declaration, restated verbatim from the
/// frozen RefineV1 (freeze 58 — a declared CONFIGURATION referencing
/// the same policy library, not controller logic): play the pinned
/// root action, then the level-1 continuation at the declared inner
/// schedule, provenance fixed a priori.
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

impl OpeningLadder<'_> {
    /// Has this stop's sampled import already reached the store? A
    /// resumed state carries its facts, and the scope names carry the
    /// stop prefix — the guard that makes resume ≡ uninterrupted
    /// (§67.5, gated).
    fn stop_imported(&self, state: &ProofState, prefix: u64) -> bool {
        let root_id = root_identity(self.root, self.position);
        let marker = format!("opening-{root_id}/p{prefix}/");
        state.facts().iter().any(|sf| {
            matches!(&sf.fact,
                Fact::Bound(b) if matches!(&b.proof, ProofTag::Sampled { scope, .. } if scope.starts_with(&marker)))
        })
    }

    /// Import the sampled root bounds at `prefix` (§65 steps 3–4): per
    /// legal action one pinned-level1 witness lower (executable — a
    /// materialized lawful policy) and one optimization-lock
    /// empirical-max upper, each under its own fresh δ scope, adapted
    /// through the ordinary §48 adapter. Returns the declared work
    /// charged (prefix per endpoint — RefineV1's forecast unit).
    fn import_sampled(&self, state: &mut ProofState, prefix: u64, delta: &BigRational) -> u64 {
        let root_id = root_identity(self.root, self.position);
        let identity = state.identity.clone();
        let belief = FactorBelief::uniform_root(self.root, self.position, self.field);
        let z = self.oracle.mass(&belief);
        let mut charged = 0u64;
        for a in &state.legal.clone() {
            let lower = frozen_policy_lower(
                self.root,
                self.position,
                &pinned_level1(self.position, *a),
                self.field,
                PolicyProvenance::Fixed,
                EVAL_EPOCH,
                prefix,
                ScopedDelta::new(opening_scope(root_id, prefix, *a, "lower"), delta.clone()),
            );
            assert_eq!(lower.action, *a, "the pinned witness plays its tile");
            let upper = pmake_empirical_max_upper(
                self.root,
                self.position,
                *a,
                self.field,
                UPPER_EPOCH,
                prefix,
                ScopedDelta::new(opening_scope(root_id, prefix, *a, "upper"), delta.clone()),
            );
            let interval = ActionInterval {
                action: *a,
                z,
                lower: LowerBound::Sampled(lower),
                upper: UpperBound::Sampled(upper),
            };
            for fact in facts_from_refine_interval(&interval) {
                state
                    .install(&identity, fact)
                    .expect("a sampled endpoint installs");
            }
            charged += 2 * prefix;
        }
        charged
    }

    /// The store's risk ledger as a derived view: one reconstructed
    /// [`ScopedDelta`] per sampled fact, re-asserted through the
    /// shared allocation arithmetic against the declared root scope
    /// (distinct scopes, exact rational sum within budget).
    fn risk_ledger(&self, state: &ProofState) -> (BigRational, Vec<String>) {
        let root_id = root_identity(self.root, self.position);
        let mut entries: Vec<ScopedDelta> = Vec::new();
        for sf in state.facts() {
            if let Fact::Bound(b) = &sf.fact {
                if let ProofTag::Sampled { scope, delta } = &b.proof {
                    entries.push(ScopedDelta::new(scope.clone(), delta.clone()));
                }
            }
        }
        let budget = ScopedDelta::new(format!("opening-{root_id}/root"), self.scope_budget.clone());
        let refs: Vec<&ScopedDelta> = entries.iter().collect();
        let total = assert_screen_risk_allocation(&budget, &refs);
        (
            total,
            entries.into_iter().map(|e| e.scope().to_string()).collect(),
        )
    }

    /// The §49 census coordinate (§65's F masses): per legal action,
    /// the CEGAR record at that action's first field decision, read at
    /// the declared [`RESIDUAL_STAGE`] — one vocabulary with Phase 4's
    /// staged evaluation. Installs nothing (the census narrows field
    /// branch intervals, never a root bound).
    fn census(&self, state: &ProofState) -> Vec<CensusReading> {
        let belief = FactorBelief::uniform_root(self.root, self.position, self.field);
        state
            .legal
            .iter()
            .map(|a| {
                let outcome = refine_to_action_exact(
                    self.oracle,
                    &belief.focal_play(*a),
                    self.field as &dyn SlicePolicy,
                );
                let stage = RESIDUAL_STAGE.min(outcome.stages.len() - 1);
                let row = &outcome.stages[stage];
                CensusReading {
                    action: *a,
                    stage,
                    classes: row.classes,
                    exact_classes: row.exact_classes,
                    exact_mass: row.exact_mass,
                    residual_mass: row.residual_mass,
                }
            })
            .collect()
    }

    /// Distinct materialized-policy fact families (the report's
    /// "policy cylinders"): executable bound lowers by (action,
    /// authority), profiles and envelopes by (action, policy id).
    fn policy_cylinders(&self, state: &ProofState) -> usize {
        let mut keys: Vec<(Domino, String)> = Vec::new();
        for sf in state.facts() {
            let key = match &sf.fact {
                Fact::Bound(b) if b.executable => Some((b.action, b.authority.clone())),
                Fact::Profile(p) => Some((p.action, p.policy_id.clone())),
                Fact::Envelope(e) => Some((e.action, e.policy_id.clone())),
                _ => None,
            };
            if let Some(k) = key {
                if !keys.contains(&k) {
                    keys.push(k);
                }
            }
        }
        keys.len()
    }

    /// Execute one declared stop against the state and report the §65
    /// panel. Deterministic: a pure function of the state and the
    /// spec (no clock — the probe stamps wall time).
    pub fn run_stop(&self, state: &mut ProofState, spec: &OpeningStopSpec) -> StopReport {
        assert_eq!(
            state.identity.root_id,
            root_identity(self.root, self.position),
            "the ladder's context is the state's root"
        );
        // §65 steps 3–4: sampled import, guarded for resume.
        let sampled_work =
            if spec.sampled_prefix > 0 && !self.stop_imported(state, spec.sampled_prefix) {
                self.import_sampled(state, spec.sampled_prefix, &spec.endpoint_delta)
            } else {
                0
            };
        // §65 steps 5–7 and 9: the frontier's items, bought or refused
        // at the stop's declared Z budget.
        let frontier = if spec.frontier_budget > 0 {
            let f = Frontier {
                oracle: self.oracle,
                root: self.root,
                position: self.position,
                field: self.field as &dyn SlicePolicy,
            };
            Some(f.advance(state, &SolveGoal::SelectAction, spec.frontier_budget))
        } else {
            None
        };
        // §65 step 8: covers, after the frontier's profile work (§62's
        // decline path while no incumbent profile exists). Installs are
        // guarded by fact equality: the producer re-derives the same
        // cover at every stop, and the append-only store should hold
        // each fact once (the same idempotence the frontier's presence
        // guards give its items).
        let producer = CountThreatProducer {
            oracle: self.oracle,
            root: self.root,
            position: self.position,
            field: self.field as &dyn SlicePolicy,
        };
        let identity = state.identity.clone();
        let mut covers_installed = 0usize;
        for fact in producer.produce(state) {
            if state.facts().iter().any(|sf| sf.fact == fact) {
                continue;
            }
            state.install(&identity, fact).expect("a cover installs");
            covers_installed += 1;
        }
        // The census coordinate (installs nothing).
        let census = if spec.census {
            self.census(state)
        } else {
            Vec::new()
        };
        // §65 step 10: the report panel — every quantity a derived
        // view of the state as it now stands.
        let report = state.closure();
        let (risk_spent, risk_scopes) = self.risk_ledger(state);
        let exec_bar = report
            .exec
            .as_ref()
            .map(|w| w.value.clone())
            .unwrap_or_else(BigRational::zero);
        let width_debt = SolveGoal::StrengthenToExact.debt(state, &report);
        let recommendation = state.recommend();
        let contract_sensitive_residual = recommendation
            .as_ref()
            .and_then(|r| r.contract_sensitive_residual.clone());
        let settled = matches!(
            report.result,
            crate::solver::proof_state::StateResult::Settled { .. }
                | crate::solver::proof_state::StateResult::Equivalent { .. }
        );
        let verdict = if settled && !report.delta_decisive {
            StopVerdict::Exact
        } else if settled {
            StopVerdict::DeltaQualified
        } else if report.exec.is_some() && report.certified_regret <= self.epsilon {
            StopVerdict::EpsilonOptimal
        } else {
            StopVerdict::Unresolved
        };
        let count_threat_cells = state
            .facts()
            .iter()
            .filter(|sf| matches!(&sf.fact, Fact::Cover(_)))
            .count();
        StopReport {
            label: spec.label.clone(),
            facts: state.facts().len(),
            survivors: report.survivors.clone(),
            excluded: report.excluded.clone(),
            proof_bar: report.bar.clone(),
            exec_bar,
            global_upper: report.u_star.clone(),
            certified_regret: report.certified_regret.clone(),
            contract_sensitive_residual,
            width_debt,
            census,
            policy_cylinders: self.policy_cylinders(state),
            count_threat_cells,
            risk_spent,
            risk_scopes,
            sampled_work,
            frontier,
            covers_installed,
            recommendation,
            delta_decisive: report.delta_decisive,
            verdict,
        }
    }
}
