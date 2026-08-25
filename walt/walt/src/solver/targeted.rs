//! `solver::targeted` — the targeted field-1 controller: parent §8
//! Stages 1–5 assembled into one per-root pipeline (the consumer the
//! rung/screen/producer stack was built to feed).
//!
//! EXPLORATORY tier. Implements parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §8 (the targeted
//! level-2 controller), §12.1–12.2 (cost decomposition and routing by
//! stability debt), under rulings L2-A1..A7 and PANEL-A7/A8 and
//! TRIPLE-A2/A3 (`walt/CENSUS-RULINGS.md`) and obligations
//! O31/O32/O34/O38 of `walt/SCENARIO-PLAYER.md` §10. This module
//! ASSEMBLES the built machinery and reimplements none of it: baselines
//! through `solver::field_swap::exact_frozen_action_values` and the
//! CE evidence inversion ([`crate::solver::upper_cs::grid_upper_endpoint`]),
//! rungs through `solver::exposure` (E0/E1/E2/E4, directional) and
//! `solver::upper_cs` (the δ-valid E3), the screen through
//! `solver::field_swap::AdmissibleScreen` / `DirectionalScreen`, Stage-4
//! survivor work through `solver::field_swap::survivor_stage4`, and the
//! explanation surface through `CancellationLadder` and the slice-3
//! six-label vocabulary. One authority per concept.
//!
//! **What the controller adds** — schedule-controlled rung spend
//! (the parent's §17.2 falsifier direction: at h8-t4 the rungs cost
//! about half the naive σ1 pass and pruned nothing):
//!
//! - **Cheapest rung first.** E1 structural covers, then the shared
//!   E0/E2 reach walk, then exact E4, then (where admitted) the
//!   directional rungs — each phase's spend recorded in integer
//!   microseconds, every bound typed with its rung.
//! - **Escalation only while a prune is still possible** — and the test
//!   is a theorem, not a heuristic. The admissible set is monotone
//!   nondecreasing in each exposure bound (raising `R_a^U` raises
//!   `U_a^(1)`, lowers `L_a^(1)`, and can only lower the bar), so for
//!   any exact values `R*` with `ℓ ≤ R* ≤ R^cur` pointwise,
//!   `𝓐₁(ℓ) ⊆ 𝓐₁(R*) ⊆ 𝓐₁(R^cur)`. An exact fixed-policy exposure
//!   `d_ρ_a` of the frozen candidate is a lawful LOWER witness to `R_a`
//!   (§7.4), so when `𝓐₁` at the lower witnesses equals `𝓐₁` at the
//!   current bounds, tightening to exact E4 provably cannot shrink the
//!   admissible set and the spend is refused as
//!   [`EscalationStop::ProvablyUseless`]. Lower witnesses STEER SPEND
//!   ONLY — [`SteeringLower`] has no screen accessor and no conversion
//!   to [`RootActionExposureUpper`]; the screen consumes sound uppers
//!   and nothing else (L2-A4/O31, unchanged).
//! - **Honest degradation at unaffordable fibers.** Over the declared
//!   exact-fiber cap (a resource limit, never a settlement rule) the
//!   route is: δ-valid fixed-policy baseline intervals
//!   ([`delta_frozen_baseline`] — the parent §8 Stage-1
//!   `DeltaFrozenSet` tier, produced by the same CE one-mean inversion
//!   TRIPLE-A2 sanctioned), the degenerate E1 bound (mass exactly 1 by
//!   definition of the trivial cover — stated, never counted at this
//!   scale), the sampled δ-valid E3 route where the zero-hypothetical
//!   shows a prune is possible, and typed refusals
//!   ([`TypedRefusal`], no numeric accessor) everywhere else. Never a
//!   silently degenerate bound.
//!
//! Every sampled quantity carries a [`ScopedDelta`]; risks across
//! distinct screen inputs sum against the declared screen budget through
//! [`assert_screen_risk_allocation`] (TRIPLE-A2 §1.8). Caps and prefixes
//! are resource limits, never settlement rules.
//!
//! **Boundary (CE-A7/§20.16):** this module is an instrument + library
//! layer. It does not touch the live player, `solver::act`'s action
//! policy, or any default; no arena claim, no strength claim, and
//! nothing here changes what any consumer plays by default.
//!
//! Stage-4 authority note: σ1 work is confined to the SYMMETRIC screen's
//! survivors through the shipped [`survivor_stage4`] (L2-T4's route).
//! The directional screen (PANEL-A8: admitted only where the symmetric
//! screen prunes nothing) is computed as an instrument and its strictly
//! tighter exclusions are reported; confining Stage-4 spend to them is a
//! declared later slice.
//!
//! Type locks (compile_fail where typing is the property):
//!
//! A steering lower witness is never a screen bound:
//!
//! ```compile_fail
//! use walt::solver::field_swap::ActionExposureUpper;
//! use walt::solver::targeted::SteeringLower;
//! fn wants_screen_input(_: ActionExposureUpper) {}
//! fn f(s: SteeringLower, a: walt::rules::Domino) {
//!     wants_screen_input(ActionExposureUpper { action: a, bound: s });
//! }
//! ```
//!
//! A typed refusal carries no number:
//!
//! ```compile_fail
//! use num_rational::BigRational;
//! use walt::solver::targeted::TypedRefusal;
//! fn f(r: &TypedRefusal) -> BigRational {
//!     r.bound()
//! }
//! ```
//!
//! A δ-valid baseline is not an exact one (the exact Stage-4 route
//! cannot consume it):
//!
//! ```compile_fail
//! use walt::solver::field_swap::ExactFrozenBaseline;
//! use walt::solver::targeted::DeltaFrozenBaseline;
//! fn wants_exact(_: &ExactFrozenBaseline) {}
//! fn f(d: &DeltaFrozenBaseline) {
//!     wants_exact(d)
//! }
//! ```

use std::collections::BTreeMap;
use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::rules::legal_plays;
use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{replay_viewer_success, root_identity, CanonicalRoot, RootPosition};
use crate::solver::evidence::ScopedDelta;
use crate::solver::exposure::{
    clairvoyant_reach, directional_reach, exact_split_reach, frozen_policy_exposure, rung_e1,
    ExposureRung, ForcedNonFocalCover, FrozenPolicyExposure, RootActionExposureUpper, WorldDomain,
};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::field_swap::{
    exact_frozen_action_values, fixed_policy_cancellation_kind, survivor_stage4, ActionBound,
    ActionDirectionalUpper, ActionExposureUpper, AdmissibleScreen, BaselineTier, CancellationKind,
    CancellationLadder, DirectionalScreen, FieldSwapKind, SurvivorEvaluation,
};
use crate::solver::policy::FrozenPolicy;
use crate::solver::upper_cs::{
    assert_screen_risk_allocation, e3_split_reach_upper, grid_upper_endpoint,
};

// ---------------------------------------------------------------------------
// Phase timing — integer microseconds, instrument grade (no wall clock on
// wasm32, where the spend fields honestly read zero).
// ---------------------------------------------------------------------------

struct PhaseTimer {
    #[cfg(not(target_arch = "wasm32"))]
    started: std::time::Instant,
}

impl PhaseTimer {
    fn start() -> PhaseTimer {
        PhaseTimer {
            #[cfg(not(target_arch = "wasm32"))]
            started: std::time::Instant::now(),
        }
    }

    fn micros(&self) -> u64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            u64::try_from(self.started.elapsed().as_micros()).expect("a phase fits u64 micros")
        }
        #[cfg(target_arch = "wasm32")]
        {
            0
        }
    }
}

/// One pipeline phase's recorded spend: integer microseconds and how many
/// items (actions, worlds, survivors) the phase produced. §12.1's cost
/// decomposition made a first-class output — the rung spend is part of
/// the result, not a side channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseSpend {
    pub phase: &'static str,
    pub micros: u64,
    pub items: u64,
}

// ---------------------------------------------------------------------------
// Resource budget and declared risk (limits, never settlement rules).
// ---------------------------------------------------------------------------

/// The controller's declared resource limits. Every field is a resource
/// limit and never a settlement rule: crossing a cap changes which route
/// runs and what tier the result carries, never what any number means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RungBudget {
    /// Fibers at or under this count take the exact route (complete-fiber
    /// baselines, rungs E1/E0/E2/E4, exact Stage 4); above it the
    /// sampled route runs.
    pub exact_fiber_cap: u128,
    /// Declared stream-prefix length for the δ-valid baselines of the
    /// sampled route. Zero disables the sampled route entirely (the
    /// over-cap outcome is then a typed refusal).
    pub baseline_prefix: u64,
    /// Declared stream-prefix length for the sampled δ-valid E3 exposure
    /// route. Zero disables rung E3 (a typed refusal where it would have
    /// been needed).
    pub e3_prefix: u64,
    /// Run the directional rungs where the symmetric screen prunes
    /// nothing (PANEL-A8's admission regime) and steering shows a
    /// directional prune is still possible.
    pub directional: bool,
}

/// The declared risk plan of the sampled routes. Every sampled quantity's
/// δ gets its own scope; the exact routes consume no sampling risk.
#[derive(Clone, Debug)]
pub struct TargetedRisk {
    /// The per-root screen budget every scoped entry sums against
    /// (TRIPLE-A2 §1.8, via [`assert_screen_risk_allocation`]).
    pub screen_budget: ScopedDelta,
    /// δ per action per one-sided baseline endpoint (each interval is two
    /// one-sided statements with distinct scopes).
    pub per_baseline_side: BigRational,
    /// δ per action for the symmetric E3 upper confidence sequence.
    pub per_e3: BigRational,
}

// ---------------------------------------------------------------------------
// Typed refusals — refusals are records, never degraded numbers.
// ---------------------------------------------------------------------------

/// Why a route was refused. Deliberately carries identities and limits,
/// never a substitute bound.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RefusalReason {
    /// The fiber exceeds the declared exact cap, so every complete-fiber
    /// producer is out of budget at this root.
    ExactUnaffordable { fiber: u128, cap: u128 },
    /// The sampled route was not declared (no risk plan or a zero
    /// baseline prefix), so nothing lawful can replace the exact route.
    SampledRouteUndeclared,
    /// Rung E3 was disabled while the zero-hypothetical showed a prune
    /// was still possible — the screen keeps its degenerate E1 bounds
    /// and says so.
    E3RouteDisabled,
}

impl fmt::Display for RefusalReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefusalReason::ExactUnaffordable { fiber, cap } => {
                write!(f, "exact-unaffordable{{fiber={fiber};cap={cap}}}")
            }
            RefusalReason::SampledRouteUndeclared => write!(f, "sampled-route-undeclared"),
            RefusalReason::E3RouteDisabled => write!(f, "e3-route-disabled"),
        }
    }
}

/// A typed refusal: which stage declined and why. There is no numeric
/// accessor on purpose — a refusal can never be read as a bound.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedRefusal {
    pub stage: &'static str,
    pub reason: RefusalReason,
}

impl fmt::Display for TypedRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TypedRefusal{{stage={};reason={}}}",
            self.stage, self.reason
        )
    }
}

// ---------------------------------------------------------------------------
// Spend steering — lower witnesses that never touch the screen.
// ---------------------------------------------------------------------------

/// An exact LOWER witness to `R_a` (the frozen candidate's fixed-policy
/// exposure `d_ρ_a ≤ R_a`, §7.4), used ONLY to decide whether further
/// rung spend can still prune. It has no `screenable_upper`, no rung, and
/// no conversion to [`RootActionExposureUpper`]: a lower witness is never
/// an upper bound (O34), and this type is what keeps the steering value
/// out of the screen mechanically.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteeringLower {
    pub action: Domino,
    pub producer: &'static str,
    value: BigRational,
}

impl SteeringLower {
    /// The steering value, named for what it is lawful for. Consumed by
    /// the escalation test and the report; never by screen arithmetic.
    pub fn value_for_steering(&self) -> &BigRational {
        &self.value
    }
}

/// The spend-steering hypothetical: the admissible set the L2-T4
/// arithmetic WOULD produce at the given per-action exposure values.
/// This is not a screen result — it exists so the monotonicity lemma in
/// the module docs can refuse provably useless escalation — and it is
/// gate-tested to agree with [`AdmissibleScreen`] on identical inputs.
/// `bounds0` and `values` are in the same action order.
fn steering_admissible(bounds0: &[ActionBound], values: &[BigRational]) -> Vec<Domino> {
    assert_eq!(bounds0.len(), values.len(), "one steering value per action");
    let lower1: Vec<BigRational> = bounds0
        .iter()
        .zip(values)
        .map(|(b, r)| &b.lower - r)
        .collect();
    let upper1: Vec<BigRational> = bounds0
        .iter()
        .zip(values)
        .map(|(b, r)| &b.upper + r)
        .collect();
    let bar = lower1.iter().max().expect("a screened action").clone();
    bounds0
        .iter()
        .zip(&upper1)
        .filter(|(_, u)| **u >= bar)
        .map(|(b, _)| b.action)
        .collect()
}

/// The directional sibling of [`steering_admissible`], on the
/// (plus, minus) pairs: `L^(1) = L^(0) − R⁻`, `U^(1) = U^(0) + R⁺`.
fn directional_steering_admissible(
    bounds0: &[ActionBound],
    plus: &[BigRational],
    minus: &[BigRational],
) -> Vec<Domino> {
    assert_eq!(bounds0.len(), plus.len(), "one plus value per action");
    assert_eq!(bounds0.len(), minus.len(), "one minus value per action");
    let lower1: Vec<BigRational> = bounds0
        .iter()
        .zip(minus)
        .map(|(b, r)| &b.lower - r)
        .collect();
    let upper1: Vec<BigRational> = bounds0
        .iter()
        .zip(plus)
        .map(|(b, r)| &b.upper + r)
        .collect();
    let bar = lower1.iter().max().expect("a screened action").clone();
    bounds0
        .iter()
        .zip(&upper1)
        .filter(|(_, u)| **u >= bar)
        .map(|(b, _)| b.action)
        .collect()
}

/// Public, test-gated view of the steering arithmetic so the parity gate
/// (`steering_admissible` agrees with [`AdmissibleScreen`] on identical
/// valid-upper inputs) lives in the test suite, not in trust.
pub fn steering_admissible_for_gate(
    bounds0: &[ActionBound],
    values: &[BigRational],
) -> Vec<Domino> {
    steering_admissible(bounds0, values)
}

// ---------------------------------------------------------------------------
// Stage 1 — the δ-valid frozen baseline (the parent §8 Stage-1
// `DeltaFrozenSet` tier's producer).
// ---------------------------------------------------------------------------

/// δ-valid fixed-policy value intervals for a frozen candidate family
/// under ONE field, over a declared indexed stream prefix: per action,
/// `V_field(ρ_a) ∈ [L_a, U_a]` except with probability at most the two
/// declared one-sided risks. Produced by inverting the CE one-mean
/// evidence process on the grid `G_N` ([`grid_upper_endpoint`], the
/// TRIPLE-A2 machinery consumed for a single fixed mean — no maximum
/// here, so no selection question at all): the upper endpoint on the win
/// count, the lower endpoint as one minus the upper endpoint on the fail
/// count, each side its own [`ScopedDelta`]. The true mean lies on `G_N`
/// because `V = wins/N` over the uniform fiber with `N` from the exact
/// fiber counter.
///
/// Mechanically distinct from `ExactFrozenBaseline`: the exact Stage-4
/// route cannot consume this type, and its tier is
/// [`BaselineTier::DeltaFrozenSet`] — no field-stability claim outruns
/// it.
#[derive(Clone, Debug)]
pub struct DeltaFrozenBaseline {
    pub actions: Vec<Domino>,
    /// Wins per action over the declared prefix, aligned with `actions`.
    pub win_counts: Vec<u64>,
    /// The declared prefix length.
    pub worlds: u64,
    /// The declared stream epoch.
    pub epoch: u64,
    /// `N = |Φ(I)|` from the exact fiber counter (grid validity).
    pub grid: u128,
    lowers: Vec<BigRational>,
    uppers: Vec<BigRational>,
    deltas: Vec<(ScopedDelta, ScopedDelta)>,
}

impl DeltaFrozenBaseline {
    /// The interval bounds in screen shape, action order preserved.
    pub fn bounds(&self) -> Vec<ActionBound> {
        self.actions
            .iter()
            .zip(self.lowers.iter().zip(&self.uppers))
            .map(|(a, (l, u))| ActionBound {
                action: *a,
                lower: l.clone(),
                upper: u.clone(),
            })
            .collect()
    }

    pub fn lower(&self, action: Domino) -> &BigRational {
        let k = self.index_of(action);
        &self.lowers[k]
    }

    pub fn upper(&self, action: Domino) -> &BigRational {
        let k = self.index_of(action);
        &self.uppers[k]
    }

    /// The risk-ledger entries this baseline consumed: two one-sided
    /// scoped deltas per action.
    pub fn risk_entries(&self) -> Vec<&ScopedDelta> {
        self.deltas.iter().flat_map(|(u, l)| [u, l]).collect()
    }

    /// The δ-settled selection, if the intervals separate: the action
    /// whose lower endpoint strictly exceeds every rival's upper
    /// endpoint. `None` is the honest open state, not a degraded pick.
    pub fn settled_argmax(&self) -> Option<Domino> {
        self.actions.iter().enumerate().find_map(|(k, a)| {
            let separated = self
                .actions
                .iter()
                .enumerate()
                .all(|(j, _)| j == k || self.lowers[k] > self.uppers[j]);
            separated.then_some(*a)
        })
    }

    fn index_of(&self, action: Domino) -> usize {
        self.actions
            .iter()
            .position(|a| *a == action)
            .expect("a baselined action")
    }
}

/// Produce a [`DeltaFrozenBaseline`]: replay every candidate on the same
/// declared indexed world stream (the kernel's exactly-uniform
/// with-replacement sampler) under the named field, then invert the CE
/// one-mean process at both ends. `per_side_delta` is the declared risk
/// of EACH one-sided endpoint; scopes derive from `scope_prefix`, the
/// field, the action, and the side, so distinct statements carry
/// distinct scopes by construction.
#[allow(clippy::too_many_arguments)]
pub fn delta_frozen_baseline(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[(Domino, &FrozenPolicy)],
    field: &FieldModel,
    epoch: u64,
    worlds: u64,
    per_side_delta: &BigRational,
    scope_prefix: &str,
) -> DeltaFrozenBaseline {
    assert!(worlds >= 1, "a declared prefix holds at least one world");
    assert!(
        per_side_delta > &BigRational::zero() && per_side_delta < &BigRational::one(),
        "a risk budget lies strictly inside (0,1)"
    );
    assert!(!candidates.is_empty(), "a baseline names an action");
    let viewer = root.kernel().viewer();
    let root_id = root_identity(root, position);
    let grid = root.count();
    let mut win_counts = vec![0u64; candidates.len()];
    for i in 0..worlds {
        let world = root.world_at(root_id, epoch, i);
        for (k, (_, rho)) in candidates.iter().enumerate() {
            if replay_viewer_success(position, viewer, &world, *rho, field) {
                win_counts[k] += 1;
            }
        }
    }
    let mut lowers = Vec::with_capacity(candidates.len());
    let mut uppers = Vec::with_capacity(candidates.len());
    let mut deltas = Vec::with_capacity(candidates.len());
    for (k, (action, _)) in candidates.iter().enumerate() {
        let upper_scope = ScopedDelta::new(
            format!("{scope_prefix}:{}:{action}:upper", field.field_id()),
            per_side_delta.clone(),
        );
        let lower_scope = ScopedDelta::new(
            format!("{scope_prefix}:{}:{action}:lower", field.field_id()),
            per_side_delta.clone(),
        );
        let upper = grid_upper_endpoint(win_counts[k], worlds, grid, per_side_delta);
        let lower = BigRational::one()
            - grid_upper_endpoint(worlds - win_counts[k], worlds, grid, per_side_delta);
        assert!(
            BigRational::zero() <= lower && lower <= upper && upper <= BigRational::one(),
            "a δ-valid interval is a valid probability interval"
        );
        lowers.push(lower);
        uppers.push(upper);
        deltas.push((upper_scope, lower_scope));
    }
    DeltaFrozenBaseline {
        actions: candidates.iter().map(|(a, _)| *a).collect(),
        win_counts,
        worlds,
        epoch,
        grid,
        lowers,
        uppers,
        deltas,
    }
}

// ---------------------------------------------------------------------------
// The typed per-root report.
// ---------------------------------------------------------------------------

/// Why the rung ladder stopped where it did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EscalationStop {
    /// The admissible set reached a singleton — nothing left to buy.
    Pruned,
    /// The steering sandwich proved further tightening cannot shrink the
    /// admissible set (module-doc lemma); the remaining rungs were
    /// refused as provably useless spend.
    ProvablyUseless,
    /// Every rung the budget affords ran; survivors remain.
    LadderComplete,
    /// A typed refusal ended the pipeline (see the report's refusals).
    Refused,
}

impl EscalationStop {
    pub fn tag(self) -> &'static str {
        match self {
            EscalationStop::Pruned => "pruned",
            EscalationStop::ProvablyUseless => "provably-useless",
            EscalationStop::LadderComplete => "ladder-complete",
            EscalationStop::Refused => "refused",
        }
    }
}

/// How the directional phase ended (PANEL-A8's admission regime).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionalPhase {
    /// The budget did not ask for it, or the symmetric screen pruned
    /// something (the directional rungs are admitted only where the
    /// symmetric bound prunes nothing).
    NotAdmitted,
    /// Steering proved the directional screen could not prune either;
    /// the coupled-to-terminal walks were refused as provably useless.
    SkippedProvablyUseless,
    /// The directional rungs ran; the report carries the screen.
    Ran,
}

impl DirectionalPhase {
    pub fn tag(self) -> &'static str {
        match self {
            DirectionalPhase::NotAdmitted => "not-admitted",
            DirectionalPhase::SkippedProvablyUseless => "skipped-provably-useless",
            DirectionalPhase::Ran => "ran",
        }
    }
}

/// One action's row in the report: its Stage-1 interval, the exposure
/// bound the screen consumed (always rung-labeled), its steering lower
/// witness where one was computed, and its admission.
#[derive(Clone, Debug)]
pub struct ActionRow {
    pub action: Domino,
    pub lower0: BigRational,
    pub upper0: BigRational,
    pub exposure: RootActionExposureUpper,
    pub steering: Option<SteeringLower>,
    pub admitted: bool,
}

/// One survivor's σ1 explanation surface: the exact cancellation ladder
/// (d, r, c⁺, c⁻, c — PANEL-A7, never collapsed) and its six-label kind.
/// `ladder.c()` IS the exact value wake `V₁(ρ) − V₀(ρ)` for the frozen
/// survivor; the decision wake is the report kind
/// (`FieldDecisionChanged`).
#[derive(Clone, Debug)]
pub struct SurvivorLadder {
    pub action: Domino,
    pub ladder: CancellationLadder,
    pub label: CancellationKind,
}

/// What Stage 4 did — mechanically distinct outcomes per tier, `None`
/// states honest.
pub enum StageFourOutcome {
    /// Exact tier: the shipped Stage-4 route (σ1 exact frozen values for
    /// the symmetric survivors ONLY; a singleton consumed no σ1 work at
    /// all), plus the per-survivor explanation ladders where σ1 work
    /// actually ran.
    ExactSurvivors {
        evaluation: Box<SurvivorEvaluation>,
        ladders: Vec<SurvivorLadder>,
    },
    /// δ tier, one survivor: the screen settled it; no σ1 work ran.
    DeltaSingleton { selected: Domino },
    /// δ tier, several survivors: δ-valid σ1 intervals for the survivors
    /// only. `settled0`/`selected1` are the δ-settled selections under
    /// each field where the intervals separate — `None` is an honest
    /// open state at the declared budget, never a degraded pick.
    DeltaSurvivors {
        sigma1: DeltaFrozenBaseline,
        settled0: Option<Domino>,
        selected1: Option<Domino>,
    },
    /// Stage 4 never ran (see the refusal).
    NotRun(TypedRefusal),
}

/// The controller's typed per-root report — parent §8 Stage 5. All exact
/// rationals; every bound rung-labeled; every sampled quantity δ-scoped;
/// spend recorded per phase; refusals typed.
pub struct TargetedRootReport {
    pub root_id: u64,
    pub field0: FieldId,
    pub field1: FieldId,
    pub fiber: u128,
    /// The Stage-1 tier every claim is relative to. `Unresolved` means a
    /// typed refusal ended the pipeline before any screen existed.
    pub tier: BaselineTier,
    pub rows: Vec<ActionRow>,
    /// The final symmetric screen (absent only under a Stage-1 refusal).
    pub screen: Option<AdmissibleScreen>,
    /// The directional screen where the phase ran.
    pub directional: Option<DirectionalScreen>,
    pub directional_phase: DirectionalPhase,
    /// The L2-A3 result kind: the screen's, or `FieldDecisionChanged`
    /// from Stage 4.
    pub kind: FieldSwapKind,
    pub stop: EscalationStop,
    pub spend: Vec<PhaseSpend>,
    pub stage4: StageFourOutcome,
    pub refusals: Vec<TypedRefusal>,
    /// The exact total sampling risk consumed against the declared
    /// screen budget (`None` on the exact route, which consumes none).
    pub risk_spent: Option<BigRational>,
}

impl TargetedRootReport {
    /// The survivors the final screen admitted (empty only under a
    /// Stage-1 refusal).
    pub fn survivors(&self) -> Vec<Domino> {
        self.screen
            .as_ref()
            .map_or_else(Vec::new, |s| s.admissible())
    }
}

// ---------------------------------------------------------------------------
// Controller configuration.
// ---------------------------------------------------------------------------

/// The declared per-run configuration: resource budget, risk plan for
/// the sampled routes, the slice-3 ε for the `EpsilonEquivalent` label
/// (None = the label is unreachable), the stream epoch, and the scope
/// prefix every ledger entry derives from.
#[derive(Clone, Debug)]
pub struct TargetedConfig {
    pub budget: RungBudget,
    pub risk: Option<TargetedRisk>,
    pub epsilon: Option<BigRational>,
    pub epoch: u64,
    pub scope: String,
}

/// The legal root actions of the viewer's decision at the root state —
/// the one derivation every consumer of this module shares.
pub fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

// ---------------------------------------------------------------------------
// The controller.
// ---------------------------------------------------------------------------

/// Run the targeted field-1 pipeline at one root: Stage-1 baseline →
/// rung ladder (cheapest first, escalation only while a prune is still
/// possible) → the L2-T4 screen → survivor-only σ1 work → the typed
/// report. `candidates` is the frozen focal family, one pinned candidate
/// per legal root action in legal-set order, frozen before any
/// cross-field evidence (acceptance item 3).
pub fn targeted_root(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[(Domino, &FrozenPolicy)],
    field0: &FieldModel,
    field1: &FieldModel,
    config: &TargetedConfig,
) -> TargetedRootReport {
    let legal = legal_root_actions(root, position);
    let actions: Vec<Domino> = legal.iter().collect();
    assert_eq!(
        candidates.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
        actions,
        "O38: one frozen candidate per legal root action, legal-set order"
    );
    assert!(
        actions.len() >= 2,
        "a screened decision compares at least two actions"
    );
    let fiber = root.count();
    if fiber <= config.budget.exact_fiber_cap {
        exact_route(
            root, position, candidates, &actions, legal, field0, field1, config,
        )
    } else {
        sampled_route(
            root, position, candidates, &actions, legal, fiber, field0, field1, config,
        )
    }
}

/// Build the screen input rows from the per-action working bounds.
fn screen_inputs(
    actions: &[Domino],
    current: &[RootActionExposureUpper],
) -> Vec<ActionExposureUpper> {
    actions
        .iter()
        .zip(current)
        .map(|(a, b)| ActionExposureUpper {
            action: *a,
            bound: b.clone(),
        })
        .collect()
}

/// Keep the tighter of two sound uppers (either rung may win; the kept
/// bound keeps its own rung label — no promotion).
fn tighter(
    current: RootActionExposureUpper,
    candidate: RootActionExposureUpper,
) -> RootActionExposureUpper {
    if candidate.screenable_upper() < current.screenable_upper() {
        candidate
    } else {
        current
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn exact_route(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[(Domino, &FrozenPolicy)],
    actions: &[Domino],
    legal: DominoSet,
    field0: &FieldModel,
    field1: &FieldModel,
    config: &TargetedConfig,
) -> TargetedRootReport {
    let root_id = root_identity(root, position);
    let fiber = root.count();
    let mut spend: Vec<PhaseSpend> = Vec::new();
    let m = u64::try_from(actions.len()).expect("a legal set fits u64");

    // Stage 1 — exact frozen-set baseline under σ0 (point intervals).
    let t = PhaseTimer::start();
    let baseline0 = exact_frozen_action_values(
        root,
        position,
        candidates,
        field0,
        &format!("{}:sigma0", config.scope),
    );
    spend.push(PhaseSpend {
        phase: "baseline-sigma0",
        micros: t.micros(),
        items: m,
    });
    let bounds0 = baseline0.point_bounds();

    // Stage 2, rung E1 — the counted structural cover (action-independent,
    // so one count bounds every legal action).
    let t = PhaseTimer::start();
    let forced = rung_e1(root, &ForcedNonFocalCover);
    spend.push(PhaseSpend {
        phase: "rung-e1",
        micros: t.micros(),
        items: 1,
    });
    let mut current: Vec<RootActionExposureUpper> = vec![forced; actions.len()];

    let screen_now = |current: &[RootActionExposureUpper]| {
        AdmissibleScreen::compute(
            legal,
            BaselineTier::ExactFrozenSet,
            &bounds0,
            &screen_inputs(actions, current),
            field0.field_id(),
            field1.field_id(),
            root_id,
        )
    };

    let mut screen = screen_now(&current);
    let mut stop: Option<EscalationStop> =
        (screen.admissible().len() == 1).then_some(EscalationStop::Pruned);

    // Stage 2, rungs E0/E2 — one shared pre-split reach walk per action.
    let mut exposures: BTreeMap<Domino, FrozenPolicyExposure> = BTreeMap::new();
    let mut steering: Vec<Option<SteeringLower>> = vec![None; actions.len()];
    if stop.is_none() {
        let t = PhaseTimer::start();
        for (k, action) in actions.iter().enumerate() {
            let reach = clairvoyant_reach(root, position, *action, field0, field1);
            let bound = reach.e0_upper().unwrap_or_else(|| reach.e2_upper());
            current[k] = tighter(current[k].clone(), bound);
        }
        spend.push(PhaseSpend {
            phase: "rung-e2",
            micros: t.micros(),
            items: m,
        });
        screen = screen_now(&current);
        if screen.admissible().len() == 1 {
            stop = Some(EscalationStop::Pruned);
        }
    }

    // Steering + rung E4 — escalate only while a prune is still possible.
    if stop.is_none() {
        let admitted = screen.admissible();
        let t = PhaseTimer::start();
        for (k, (action, rho)) in candidates.iter().enumerate() {
            if !admitted.contains(action) || current[k].screenable_upper().is_zero() {
                continue;
            }
            let exposure = frozen_policy_exposure(
                root,
                position,
                rho,
                field0,
                field1,
                WorldDomain::ExactFiber,
            );
            steering[k] = Some(SteeringLower {
                action: *action,
                producer: "exact-frozen-lower-witness",
                value: exposure.d_hat(),
            });
            exposures.insert(*action, exposure);
        }
        let steered = u64::try_from(exposures.len()).expect("fits");
        spend.push(PhaseSpend {
            phase: "steering",
            micros: t.micros(),
            items: steered,
        });
        // The most optimistic lawful tightening: each steered action can
        // reach at best its lower witness; every other action at best 0.
        let lows: Vec<BigRational> = steering
            .iter()
            .map(|s| {
                s.as_ref()
                    .map_or_else(BigRational::zero, |s| s.value_for_steering().clone())
            })
            .collect();
        let hypothetical = steering_admissible(&bounds0, &lows);
        if hypothetical == admitted {
            stop = Some(EscalationStop::ProvablyUseless);
        } else {
            let t = PhaseTimer::start();
            let mut solved = 0u64;
            for (k, action) in actions.iter().enumerate() {
                if lows[k] < *current[k].screenable_upper() {
                    let solve = exact_split_reach(root, position, *action, field0, field1);
                    current[k] = tighter(current[k].clone(), solve.e4_upper());
                    solved += 1;
                }
            }
            spend.push(PhaseSpend {
                phase: "rung-e4",
                micros: t.micros(),
                items: solved,
            });
            screen = screen_now(&current);
            stop = Some(if screen.admissible().len() == 1 {
                EscalationStop::Pruned
            } else {
                EscalationStop::LadderComplete
            });
        }
    }
    let stop = stop.expect("the exact ladder always stops");

    // The directional phase — admitted only where the symmetric screen
    // prunes nothing (PANEL-A8), and only when steering shows the
    // directional screen could still prune.
    let mut directional: Option<DirectionalScreen> = None;
    let mut directional_phase = DirectionalPhase::NotAdmitted;
    if config.budget.directional
        && stop != EscalationStop::Pruned
        && screen.admissible().len() == actions.len()
    {
        let plus_lows: Vec<BigRational> = actions
            .iter()
            .map(|a| {
                exposures.get(a).map_or_else(BigRational::zero, |e| {
                    BigRational::new(BigInt::from(e.corrections_plus), BigInt::from(e.worlds))
                })
            })
            .collect();
        let minus_lows: Vec<BigRational> = actions
            .iter()
            .map(|a| {
                exposures.get(a).map_or_else(BigRational::zero, |e| {
                    BigRational::new(BigInt::from(e.corrections_minus), BigInt::from(e.worlds))
                })
            })
            .collect();
        let hypothetical = directional_steering_admissible(&bounds0, &plus_lows, &minus_lows);
        if hypothetical == screen.admissible() {
            directional_phase = DirectionalPhase::SkippedProvablyUseless;
        } else {
            let t = PhaseTimer::start();
            let dir_bounds: Vec<ActionDirectionalUpper> = actions
                .iter()
                .map(|action| {
                    let dir = directional_reach(root, position, *action, field0, field1);
                    ActionDirectionalUpper {
                        action: *action,
                        bound: dir.directional_upper(),
                    }
                })
                .collect();
            spend.push(PhaseSpend {
                phase: "rung-directional",
                micros: t.micros(),
                items: m,
            });
            let dir_screen = DirectionalScreen::compute(
                legal,
                BaselineTier::ExactFrozenSet,
                &bounds0,
                &dir_bounds,
                field0.field_id(),
                field1.field_id(),
                root_id,
            );
            for action in dir_screen.admissible() {
                assert!(
                    screen.admissible().contains(&action),
                    "the directional screen only ever prunes MORE"
                );
            }
            directional = Some(dir_screen);
            directional_phase = DirectionalPhase::Ran;
        }
    }

    // Stage 4 — σ1 work confined to the symmetric survivors through the
    // shipped route (a singleton consumes no σ1 work at all).
    let t = PhaseTimer::start();
    let evaluation = survivor_stage4(
        root,
        position,
        &screen,
        &baseline0,
        candidates,
        field1,
        &format!("{}:stage4", config.scope),
    );
    spend.push(PhaseSpend {
        phase: "stage4-sigma1",
        micros: t.micros(),
        items: u64::try_from(evaluation.survivors.len()).expect("fits"),
    });
    // The explanation surface, only where σ1 work actually ran: the
    // survivors' exact ladders and six-label kinds.
    let mut ladders: Vec<SurvivorLadder> = Vec::new();
    if evaluation.values1.is_some() {
        let t = PhaseTimer::start();
        for action in &evaluation.survivors {
            let exposure = exposures.remove(action).unwrap_or_else(|| {
                let rho = candidates
                    .iter()
                    .find(|(a, _)| a == action)
                    .map(|(_, p)| *p)
                    .expect("a survivor has its frozen candidate");
                frozen_policy_exposure(root, position, rho, field0, field1, WorldDomain::ExactFiber)
            });
            let ladder = CancellationLadder::from_exposure(&exposure);
            let label = fixed_policy_cancellation_kind(&ladder, config.epsilon.as_ref());
            ladders.push(SurvivorLadder {
                action: *action,
                ladder,
                label,
            });
        }
        spend.push(PhaseSpend {
            phase: "stage4-ladders",
            micros: t.micros(),
            items: u64::try_from(ladders.len()).expect("fits"),
        });
    }
    let kind = evaluation.kind;

    let admitted = screen.admissible();
    let rows: Vec<ActionRow> = actions
        .iter()
        .enumerate()
        .map(|(k, action)| ActionRow {
            action: *action,
            lower0: bounds0[k].lower.clone(),
            upper0: bounds0[k].upper.clone(),
            exposure: current[k].clone(),
            steering: steering[k].clone(),
            admitted: admitted.contains(action),
        })
        .collect();

    TargetedRootReport {
        root_id,
        field0: field0.field_id(),
        field1: field1.field_id(),
        fiber,
        tier: BaselineTier::ExactFrozenSet,
        rows,
        screen: Some(screen),
        directional,
        directional_phase,
        kind,
        stop,
        spend,
        stage4: StageFourOutcome::ExactSurvivors {
            evaluation: Box::new(evaluation),
            ladders,
        },
        refusals: Vec::new(),
        risk_spent: None,
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn sampled_route(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[(Domino, &FrozenPolicy)],
    actions: &[Domino],
    legal: DominoSet,
    fiber: u128,
    field0: &FieldModel,
    field1: &FieldModel,
    config: &TargetedConfig,
) -> TargetedRootReport {
    let root_id = root_identity(root, position);
    let mut spend: Vec<PhaseSpend> = Vec::new();
    let mut refusals: Vec<TypedRefusal> = Vec::new();
    let m = u64::try_from(actions.len()).expect("a legal set fits u64");
    let over_cap = TypedRefusal {
        stage: "route",
        reason: RefusalReason::ExactUnaffordable {
            fiber,
            cap: config.budget.exact_fiber_cap,
        },
    };
    refusals.push(over_cap.clone());

    let (Some(risk), true) = (config.risk.as_ref(), config.budget.baseline_prefix >= 1) else {
        // No lawful sampled route was declared: a typed refusal, never a
        // degenerate bound pretending to be a result.
        let refusal = TypedRefusal {
            stage: "baseline",
            reason: RefusalReason::SampledRouteUndeclared,
        };
        refusals.push(refusal.clone());
        return TargetedRootReport {
            root_id,
            field0: field0.field_id(),
            field1: field1.field_id(),
            fiber,
            tier: BaselineTier::Unresolved,
            rows: Vec::new(),
            screen: None,
            directional: None,
            directional_phase: DirectionalPhase::NotAdmitted,
            kind: FieldSwapKind::FieldUnresolved,
            stop: EscalationStop::Refused,
            spend,
            stage4: StageFourOutcome::NotRun(refusal),
            refusals,
            risk_spent: None,
        };
    };

    // Stage 1 — δ-valid frozen baseline under σ0.
    let t = PhaseTimer::start();
    let baseline0 = delta_frozen_baseline(
        root,
        position,
        candidates,
        field0,
        config.epoch,
        config.budget.baseline_prefix,
        &risk.per_baseline_side,
        &format!("{}:sigma0", config.scope),
    );
    spend.push(PhaseSpend {
        phase: "baseline-sigma0-delta",
        micros: t.micros(),
        items: config.budget.baseline_prefix,
    });
    let bounds0 = baseline0.bounds();

    // Rung E1, degenerate form: the trivial cover's mass is exactly 1 by
    // definition (P ≡ 1), stated without a fiber count — counting the
    // full fiber is exactly what the cap refused. §8.1: the screen
    // lawfully starts from the degenerate bound.
    let mut current: Vec<RootActionExposureUpper> =
        vec![
            RootActionExposureUpper::from_rung(ExposureRung::E1, BigRational::one(),);
            actions.len()
        ];

    let screen_now = |current: &[RootActionExposureUpper]| {
        AdmissibleScreen::compute(
            legal,
            BaselineTier::DeltaFrozenSet,
            &bounds0,
            &screen_inputs(actions, current),
            field0.field_id(),
            field1.field_id(),
            root_id,
        )
    };

    let mut screen = screen_now(&current);
    let mut stop: Option<EscalationStop> =
        (screen.admissible().len() == 1).then_some(EscalationStop::Pruned);
    let mut e3_entries: Vec<ScopedDelta> = Vec::new();

    // Rung E3 — only when the zero-hypothetical shows exposure bounds
    // could prune at all under these baseline intervals.
    if stop.is_none() {
        let zeros = vec![BigRational::zero(); actions.len()];
        let hypothetical = steering_admissible(&bounds0, &zeros);
        if hypothetical == screen.admissible() {
            // Overlapping intervals already admit everything the most
            // optimistic exposure bounds would: every E3 walk would be
            // provably useless spend.
            stop = Some(EscalationStop::ProvablyUseless);
        } else if config.budget.e3_prefix == 0 {
            refusals.push(TypedRefusal {
                stage: "rung-e3",
                reason: RefusalReason::E3RouteDisabled,
            });
            stop = Some(EscalationStop::Refused);
        } else {
            let t = PhaseTimer::start();
            for (k, action) in actions.iter().enumerate() {
                let delta =
                    ScopedDelta::new(format!("{}:e3:{action}", config.scope), risk.per_e3.clone());
                let e3 = e3_split_reach_upper(
                    root,
                    position,
                    *action,
                    field0,
                    field1,
                    config.epoch,
                    config.budget.e3_prefix,
                    delta.clone(),
                );
                e3_entries.push(delta);
                current[k] = tighter(current[k].clone(), e3.screen_upper());
            }
            spend.push(PhaseSpend {
                phase: "rung-e3",
                micros: t.micros(),
                items: m,
            });
            screen = screen_now(&current);
            stop = Some(if screen.admissible().len() == 1 {
                EscalationStop::Pruned
            } else {
                EscalationStop::LadderComplete
            });
        }
    }
    let stop = stop.expect("the sampled ladder always stops");

    // Stage 4 — δ-valid σ1 intervals for the survivors only.
    let survivors = screen.admissible();
    let mut risk_entries: Vec<ScopedDelta> =
        baseline0.risk_entries().into_iter().cloned().collect();
    risk_entries.extend(e3_entries);
    let (stage4, kind) = if survivors.len() == 1 {
        (
            StageFourOutcome::DeltaSingleton {
                selected: survivors[0],
            },
            screen.kind(),
        )
    } else {
        let surv_candidates: Vec<(Domino, &FrozenPolicy)> = candidates
            .iter()
            .filter(|(a, _)| survivors.contains(a))
            .copied()
            .collect();
        let t = PhaseTimer::start();
        let sigma1 = delta_frozen_baseline(
            root,
            position,
            &surv_candidates,
            field1,
            config.epoch,
            config.budget.baseline_prefix,
            &risk.per_baseline_side,
            &format!("{}:stage4-sigma1", config.scope),
        );
        spend.push(PhaseSpend {
            phase: "stage4-sigma1-delta",
            micros: t.micros(),
            items: u64::try_from(survivors.len()).expect("fits"),
        });
        risk_entries.extend(sigma1.risk_entries().into_iter().cloned());
        let settled0 = baseline0.settled_argmax();
        let selected1 = sigma1.settled_argmax();
        let kind = match (settled0, selected1) {
            (Some(a), Some(b)) if a != b => FieldSwapKind::FieldDecisionChanged,
            _ => screen.kind(),
        };
        (
            StageFourOutcome::DeltaSurvivors {
                sigma1,
                settled0,
                selected1,
            },
            kind,
        )
    };

    // The §1.8 ledger arithmetic: distinct scopes, exact-rational sum at
    // most the declared screen budget.
    let entry_refs: Vec<&ScopedDelta> = risk_entries.iter().collect();
    let risk_spent = assert_screen_risk_allocation(&risk.screen_budget, &entry_refs);

    let admitted = screen.admissible();
    let rows: Vec<ActionRow> = actions
        .iter()
        .enumerate()
        .map(|(k, action)| ActionRow {
            action: *action,
            lower0: bounds0[k].lower.clone(),
            upper0: bounds0[k].upper.clone(),
            exposure: current[k].clone(),
            steering: None,
            admitted: admitted.contains(action),
        })
        .collect();

    TargetedRootReport {
        root_id,
        field0: field0.field_id(),
        field1: field1.field_id(),
        fiber,
        tier: BaselineTier::DeltaFrozenSet,
        rows,
        screen: Some(screen),
        directional: None,
        directional_phase: DirectionalPhase::NotAdmitted,
        kind,
        stop,
        spend,
        stage4,
        refusals,
        risk_spent: Some(risk_spent),
    }
}
