//! `solver::field_swap` — the admissible level-2 action set and
//! field-stability slack (§21 step 8).
//!
//! EXPLORATORY tier. Implements parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §5 (interval form,
//! the admissible set 𝓐₁, L2-T4, stability slack), §8 Stages 1–3 (baseline
//! bounds for every legal root action, all-action exposure bounds, the
//! screen), and §12.2 (routing by stability debt), under rulings L2-A1..A7
//! (`walt/CENSUS-RULINGS.md`) and obligations O31/O32/O38 of
//! `walt/SCENARIO-PLAYER.md` §10.
//!
//! Structural commitments:
//!
//! - **Only [`RootActionExposureUpper`] enters the screen (L2-A4, O31).**
//!   The exposure input type wraps it and nothing else; the arithmetic
//!   reads bounds through `screenable_upper`, the accessor no other
//!   exposure tier has. A `FrozenPolicyExposure` cannot arrive here by
//!   type.
//! - **Every legal action gets a bound before any exclusion (O38).** The
//!   constructor takes the complete legal set and asserts both the
//!   baseline bounds and the exposure bounds cover it exactly — no
//!   missing action, no extra, no duplicate. Heuristic orderings may
//!   sequence evaluation elsewhere; they cannot remove an action from
//!   this screen.
//! - **Derived views, never stored state.** The screen stores its rows
//!   (action, baseline interval, exposure bound) and derives `L^(1)`,
//!   `U^(1)`, the bar `B̄`, the admissible set, the slack table, and the
//!   result kind as functions of the rows — no second authority.
//! - **No claim outruns the baseline tier (§8 Stage 1).** The result kind
//!   is a function of (tier, |𝓐₁|): an `Unresolved` baseline can never
//!   yield a `FieldStable*` kind.
//!
//! The seven L2-A3 result-kind semantics are all mechanically present
//! ([`FieldSwapKind`]); the second slice produced the three `FieldStable*`
//! kinds, `FieldSensitive` (survivors remain — field-1 work is confined
//! to them), and `FieldUnresolved`. Slice 3's Stage-4 survivor route
//! ([`survivor_stage4`]) now produces `FieldDecisionChanged`;
//! `HeuristicFallback` remains a later consumer's output and exists as a
//! type so no consumer is built against a smaller ladder.
//!
//! SLICE 3 [L2 thread] — the cancellation ladder, pairwise masses, the
//! six-label cancellation vocabulary, directional screening, Stage-4
//! survivor-only field-1 optimization, first-split trace aggregation, and
//! the Λ evidence processes; adopted from Part VI of the x:019–023 panel
//! response (`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
//! §§31–42) under rulings PANEL-A7/A8 (`walt/CENSUS-RULINGS.md`). Thread
//! labels per PANEL-A7: the ladder and directional bounds are L2 objects,
//! the pairwise masses are CE-pairwise objects, the dominance route is
//! objective-level.
//!
//! **The interpretation rule (response §42; PANEL-A7 BINDING), verbatim:**
//!
//! > Cancellation may justify a value statement under one declared
//! > objective, belief, and model. It never by itself proves pathwise
//! > safety, structural irrelevance, dominance, or stability under
//! > reweighting.
//!
//! Three distinct zeros stay distinct in type and label (PANEL-A7):
//! behavioral irrelevance (`d = 0`), outcome irrelevance (`r = 0`), and
//! value neutrality (`c = 0`) are three different statements, never one
//! "close" label. `Dominated` is reachable ONLY through exact enumeration
//! (`H = 0 ∧ B > 0` counted over the whole fiber; the ruling also admits
//! a validly-bounded route, which has no producer in this slice):
//! [`SampledPairwiseMasses`] has no dominance method by construction, so
//! a sampled zero hazard count cannot produce the label.

use std::collections::BTreeMap;
use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{replay_viewer_success, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::controller::{exact_frozen_set, CandidateSet, RiskPlan, SetResult, SetSpec};
use crate::solver::evidence::{
    pivotal_evidence, BoundedMeanMixture, MeanNull, MixtureError, ScopedDelta,
};
use crate::solver::exposure::{
    exact_root_value, FirstSplit, FrozenPolicyExposure, RootActionDirectionalUpper,
    RootActionExposureUpper, WorldDomain,
};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::policy::{ActionRule, FrozenPolicy, PolicyId};

// ---------------------------------------------------------------------------
// Stage-1 baseline tiers (§8 Stage 1).
// ---------------------------------------------------------------------------

/// The result tier of the Stage-1 baseline the screen's inputs came from.
/// No field-stability claim outruns this tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaselineTier {
    /// Exact root values: `Q_a^(0)` itself, complete-fiber optimization.
    ExactRoot,
    /// Exact frozen-set values: `V_0(ρ_a)` for one named frozen candidate
    /// per action, complete-fiber enumeration. The screen's statement is
    /// then about the frozen candidate set, never the optimized root.
    ExactFrozenSet,
    /// δ-valid fixed-policy intervals.
    DeltaFrozenSet,
    /// No valid baseline: the screen can only report `FieldUnresolved`.
    Unresolved,
}

impl BaselineTier {
    pub fn label(self) -> &'static str {
        match self {
            BaselineTier::ExactRoot => "exact-root",
            BaselineTier::ExactFrozenSet => "exact-frozen-set",
            BaselineTier::DeltaFrozenSet => "delta-frozen-set",
            BaselineTier::Unresolved => "unresolved",
        }
    }
}

// ---------------------------------------------------------------------------
// Screen inputs.
// ---------------------------------------------------------------------------

/// A valid field-0 value interval for one legal root action:
/// `Q_a^(0) ∈ [lower, upper]` at the declared baseline tier (a point
/// interval when the tier is exact).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBound {
    pub action: Domino,
    pub lower: BigRational,
    pub upper: BigRational,
}

/// One legal root action's exposure bound. Wraps
/// [`RootActionExposureUpper`] and NOTHING else — the L2-A4/O31 lock:
/// no fixed-policy or library exposure can be given to the screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionExposureUpper {
    pub action: Domino,
    pub bound: RootActionExposureUpper,
}

// ---------------------------------------------------------------------------
// The screen rows and derived interval arithmetic (§5).
// ---------------------------------------------------------------------------

/// One action's screen row: the stored authority is (action, baseline
/// interval, exposure bound); the field-1 interval is a derived view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScreenRow {
    pub action: Domino,
    pub lower0: BigRational,
    pub upper0: BigRational,
    pub exposure: RootActionExposureUpper,
}

impl ScreenRow {
    /// §5 — `L_a^(1) = L_a^(0) − R_a^U` (the parent's formula verbatim;
    /// deliberately not clamped to [0, 1]).
    pub fn lower1(&self) -> BigRational {
        &self.lower0 - self.exposure.screenable_upper()
    }

    /// §5 — `U_a^(1) = U_a^(0) + R_a^U`.
    pub fn upper1(&self) -> BigRational {
        &self.upper0 + self.exposure.screenable_upper()
    }
}

// ---------------------------------------------------------------------------
// Result kinds (§8 Stage 5; L2-A3 BINDING semantics, Rust naming free).
// ---------------------------------------------------------------------------

/// The seven binding field-swap result semantics. Mechanically distinct in
/// type and serialization; no UI or bridge may flatten them (L2-A3,
/// acceptance item 24).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldSwapKind {
    /// `|𝓐₁| = 1` over exact root baselines.
    FieldStableExactRoot,
    /// `|𝓐₁| = 1` over exact frozen-set baselines: the frozen candidate
    /// set's selection survives the field upgrade. Not an exact
    /// field-stable root (§15.3).
    FieldStableExactFrozenSet,
    /// `|𝓐₁| = 1` over δ-valid frozen-set baselines.
    FieldStableDeltaFrozenSet,
    /// `|𝓐₁| > 1`: the field upgrade can still move the decision;
    /// field-1 work is confined to the survivors (§8 Stage 4).
    FieldSensitive,
    /// A Stage-4 comparison found the settled root choice changed under
    /// σ1. Not produced by the screen — a later slice's consumer output.
    FieldDecisionChanged,
    /// The baseline tier supports no stability claim.
    FieldUnresolved,
    /// An explicitly named fallback chose after `FieldUnresolved`. Not
    /// produced by the screen — a later slice's consumer output.
    HeuristicFallback,
}

impl FieldSwapKind {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(self) -> &'static str {
        match self {
            FieldSwapKind::FieldStableExactRoot => "FieldStableExactRoot",
            FieldSwapKind::FieldStableExactFrozenSet => "FieldStableExactFrozenSet",
            FieldSwapKind::FieldStableDeltaFrozenSet => "FieldStableDeltaFrozenSet",
            FieldSwapKind::FieldSensitive => "FieldSensitive",
            FieldSwapKind::FieldDecisionChanged => "FieldDecisionChanged",
            FieldSwapKind::FieldUnresolved => "FieldUnresolved",
            FieldSwapKind::HeuristicFallback => "HeuristicFallback",
        }
    }

    pub const ALL: [FieldSwapKind; 7] = [
        FieldSwapKind::FieldStableExactRoot,
        FieldSwapKind::FieldStableExactFrozenSet,
        FieldSwapKind::FieldStableDeltaFrozenSet,
        FieldSwapKind::FieldSensitive,
        FieldSwapKind::FieldDecisionChanged,
        FieldSwapKind::FieldUnresolved,
        FieldSwapKind::HeuristicFallback,
    ];
}

impl fmt::Display for FieldSwapKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

// ---------------------------------------------------------------------------
// Stability slack (§5.1, §12.2).
// ---------------------------------------------------------------------------

/// §5.1 — `S_{a,b} = L_a^(0) − U_b^(0) − R_a^U − R_b^U` for one ordered
/// pair. Positive: the pair is field-stable; negative: the field upgrade
/// can still alter the ordering — the most negative slack names where the
/// next unit of field-level compute belongs (§12.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StabilitySlack {
    pub a: Domino,
    pub b: Domino,
    pub slack: BigRational,
}

// ---------------------------------------------------------------------------
// The admissible screen (§5, §8 Stages 1–3; L2-T4; O32/O38).
// ---------------------------------------------------------------------------

/// The Stage-3 screen: per-action field-1 intervals, the bar
/// `B̄ = max_a L_a^(1)`, and the admissible set
/// `𝓐₁ = {a : U_a^(1) ≥ B̄}` — L2-T4 implemented exactly. Everything
/// beyond the stored rows is a derived view.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmissibleScreen {
    /// `FieldId(σ0)` — every cross-field result names both field
    /// identities (§8 Stage 0; acceptance item 2).
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The Stage-1 baseline tier of every `[L^(0), U^(0)]` input.
    pub tier: BaselineTier,
    rows: Vec<ScreenRow>,
}

impl AdmissibleScreen {
    /// Build the screen. Asserts the O38 all-action admission audit: the
    /// baseline bounds and the exposure bounds each cover the complete
    /// legal set exactly — every legal action gets a bound before any
    /// exclusion, nothing else gets one, and nothing is listed twice.
    pub fn compute(
        legal: DominoSet,
        tier: BaselineTier,
        bounds: &[ActionBound],
        exposures: &[ActionExposureUpper],
        field0: FieldId,
        field1: FieldId,
        root_id: u64,
    ) -> AdmissibleScreen {
        assert!(!legal.is_empty(), "a screened decision has a legal action");
        assert_eq!(
            bounds.len(),
            legal.len(),
            "O38: exactly one baseline bound per legal action"
        );
        assert_eq!(
            exposures.len(),
            legal.len(),
            "O38: exactly one exposure bound per legal action"
        );
        let zero = BigRational::zero();
        let one = BigRational::one();
        let rows: Vec<ScreenRow> = legal
            .iter()
            .map(|action| {
                let matching: Vec<&ActionBound> =
                    bounds.iter().filter(|b| b.action == action).collect();
                assert_eq!(
                    matching.len(),
                    1,
                    "O38: one baseline bound names legal action {action}"
                );
                let bound = matching[0];
                let matching: Vec<&ActionExposureUpper> =
                    exposures.iter().filter(|e| e.action == action).collect();
                assert_eq!(
                    matching.len(),
                    1,
                    "O38: one exposure bound names legal action {action}"
                );
                let exposure = matching[0];
                assert!(
                    zero <= bound.lower && bound.lower <= bound.upper && bound.upper <= one,
                    "a baseline interval is a valid probability interval"
                );
                ScreenRow {
                    action,
                    lower0: bound.lower.clone(),
                    upper0: bound.upper.clone(),
                    exposure: exposure.bound.clone(),
                }
            })
            .collect();
        AdmissibleScreen {
            field0,
            field1,
            root_id,
            tier,
            rows,
        }
    }

    pub fn rows(&self) -> &[ScreenRow] {
        &self.rows
    }

    pub fn row(&self, action: Domino) -> &ScreenRow {
        self.rows
            .iter()
            .find(|r| r.action == action)
            .expect("a screened action")
    }

    /// §5 — the bar `B̄ = max_a L_a^(1)`.
    pub fn bar(&self) -> BigRational {
        self.rows
            .iter()
            .map(ScreenRow::lower1)
            .max()
            .expect("a screened decision has a legal action")
    }

    /// L2-T4 membership: `a ∈ 𝓐₁ ⇔ U_a^(1) ≥ B̄`.
    pub fn admitted(&self, action: Domino) -> bool {
        self.row(action).upper1() >= self.bar()
    }

    /// §5 — the admissible level-2 action set `𝓐₁`, in legal-set order.
    /// Never empty: the bar's witness row has `U^(1) ≥ L^(1) = B̄`.
    pub fn admissible(&self) -> Vec<Domino> {
        let bar = self.bar();
        let set: Vec<Domino> = self
            .rows
            .iter()
            .filter(|r| r.upper1() >= bar)
            .map(|r| r.action)
            .collect();
        assert!(
            !set.is_empty(),
            "the admissible set holds the bar's witness"
        );
        set
    }

    /// The typed result kind — a derived view of (tier, |𝓐₁|). An
    /// `Unresolved` baseline never yields a stability claim; a singleton
    /// admissible set claims stability AT the baseline tier and no higher.
    pub fn kind(&self) -> FieldSwapKind {
        if self.tier == BaselineTier::Unresolved {
            return FieldSwapKind::FieldUnresolved;
        }
        if self.admissible().len() == 1 {
            match self.tier {
                BaselineTier::ExactRoot => FieldSwapKind::FieldStableExactRoot,
                BaselineTier::ExactFrozenSet => FieldSwapKind::FieldStableExactFrozenSet,
                BaselineTier::DeltaFrozenSet => FieldSwapKind::FieldStableDeltaFrozenSet,
                BaselineTier::Unresolved => unreachable!("returned above"),
            }
        } else {
            FieldSwapKind::FieldSensitive
        }
    }

    /// §5.1 — the stability slack of one ordered pair,
    /// `S_{a,b} = L_a^(0) − U_b^(0) − R_a^U − R_b^U`.
    pub fn slack(&self, a: Domino, b: Domino) -> BigRational {
        assert_ne!(a, b, "slack compares two distinct actions");
        let ra = self.row(a);
        let rb = self.row(b);
        &ra.lower0 - &rb.upper0 - ra.exposure.screenable_upper() - rb.exposure.screenable_upper()
    }

    /// §12.2 — the complete ordered-pair slack table, in legal-set order.
    /// The most negative entries name where the next unit of field-level
    /// compute belongs.
    pub fn slack_table(&self) -> Vec<StabilitySlack> {
        let mut table = Vec::new();
        for ra in &self.rows {
            for rb in &self.rows {
                if ra.action == rb.action {
                    continue;
                }
                table.push(StabilitySlack {
                    a: ra.action,
                    b: rb.action,
                    slack: self.slack(ra.action, rb.action),
                });
            }
        }
        table
    }
}

impl fmt::Display for AdmissibleScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AdmissibleScreen{{kind={};tier={};field0={};field1={};root={:#018x};bar={};rows=[",
            self.kind(),
            self.tier.label(),
            self.field0,
            self.field1,
            self.root_id,
            self.bar()
        )?;
        for (k, row) in self.rows.iter().enumerate() {
            if k > 0 {
                write!(f, ",")?;
            }
            write!(
                f,
                "{}:[{},{}]&{}->[{},{}]:{}",
                row.action,
                row.lower0,
                row.upper0,
                row.exposure,
                row.lower1(),
                row.upper1(),
                if self.admitted(row.action) {
                    "admitted"
                } else {
                    "excluded"
                }
            )?;
        }
        write!(f, "]}}")
    }
}

// ---------------------------------------------------------------------------
// The Stage-1 exact frozen-set baseline (§8 Stage 1, ExactFrozenSet tier).
// ---------------------------------------------------------------------------

/// Exact frozen-set values for a per-action candidate family under one
/// named field: `V_field(ρ_a)` for every legal root action's pinned
/// frozen candidate, computed by the `solver::controller` cold exact
/// endpoint (one enumeration pass, every candidate on every world).
pub struct ExactFrozenBaseline {
    /// Actions in candidate order.
    pub actions: Vec<Domino>,
    /// Exact `V_field(ρ_a) = wins_a / fiber` per action, same order.
    pub values: Vec<BigRational>,
    pub fiber: u128,
    /// The controller's `ExactFrozenSet` result, carried whole.
    pub result: SetResult,
}

impl ExactFrozenBaseline {
    /// The Stage-3 point intervals: at the exact tier,
    /// `L^(0) = U^(0) = V_0(ρ_a)`.
    pub fn point_bounds(&self) -> Vec<ActionBound> {
        self.actions
            .iter()
            .zip(&self.values)
            .map(|(action, value)| ActionBound {
                action: *action,
                lower: value.clone(),
                upper: value.clone(),
            })
            .collect()
    }

    pub fn value(&self, action: Domino) -> &BigRational {
        let at = self
            .actions
            .iter()
            .position(|a| *a == action)
            .expect("a baselined action");
        &self.values[at]
    }
}

/// §8 Stage 1 at the `ExactFrozenSet` tier: evaluate one pinned frozen
/// candidate per legal root action over the complete fiber under the named
/// field, through `solver::controller`'s cold exact endpoint. Asserts each
/// candidate actually pins its named action (the frozen candidate family
/// IS the fixed set the screen's claims are about).
pub fn exact_frozen_action_values(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[(Domino, &FrozenPolicy)],
    field: &FieldModel,
    scope: &str,
) -> ExactFrozenBaseline {
    assert!(
        candidates.len() >= 2,
        "a screened decision compares at least two actions"
    );
    for (action, policy) in candidates {
        match &policy.tuple().action_rule {
            ActionRule::PinnedThenLevel1 { pinned } => assert_eq!(
                pinned, action,
                "a stage-1 candidate pins its named root action"
            ),
            ActionRule::Preference(_) => {
                panic!("a stage-1 candidate pins its named root action")
            }
        }
    }
    let set = CandidateSet::new(candidates.iter().map(|(_, p)| *p).collect());
    let spec = SetSpec {
        root,
        position,
        candidates: &set,
        field,
        // The exact endpoint spends no sampling risk (§6.1); the declared
        // scope budget exists so the ledger stays reconstructable.
        plan: RiskPlan::strict(ScopedDelta::new(
            scope,
            BigRational::new(BigInt::from(1), BigInt::from(10)),
        )),
        world_cap: 0,
        batch: 1,
        escalation: None,
    };
    let evaluation = exact_frozen_set(&spec);
    let SetResult::ExactFrozenSet { wins, fiber, .. } = &evaluation.result else {
        unreachable!("the cold exact endpoint returns ExactFrozenSet")
    };
    let values: Vec<BigRational> = wins
        .iter()
        .map(|w| BigRational::new(BigInt::from(*w), BigInt::from(*fiber)))
        .collect();
    ExactFrozenBaseline {
        actions: candidates.iter().map(|(a, _)| *a).collect(),
        values,
        fiber: *fiber,
        result: evaluation.result,
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — the fixed-policy cancellation ladder (response §31; PANEL-A7).
// [L2]
// ---------------------------------------------------------------------------

/// Response §31 — the fixed-policy cancellation report, retaining ALL of
/// `(d, r, c⁺, c⁻, c)` (PANEL-A7: never collapsed into a net). A derived
/// view of one [`FrozenPolicyExposure`]: every tally is recomputed from
/// the per-world rows at construction and the ladder `|c| ≤ r ≤ d` plus
/// the three-zero nesting `d = 0 ⇒ r = 0 ⇒ c = 0` are asserted exactly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CancellationLadder {
    /// The one named frozen focal policy ρ.
    pub policy: PolicyId,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The declared world set — part of the claim: only `ExactFiber`
    /// establishes any of the three zeros (PANEL-A7's sampled-zero
    /// discipline).
    pub domain: WorldDomain,
    /// Worlds evaluated.
    pub worlds: u64,
    /// `d` numerator: worlds reaching the field-disagreement frontier.
    pub exposed: u64,
    /// `r` numerator: worlds with `u1 ≠ u0` (equals `c⁺ + c⁻` pointwise
    /// for Boolean payoff — asserted).
    pub outcome_changed: u64,
    /// `c⁺` numerator.
    pub c_plus: u64,
    /// `c⁻` numerator.
    pub c_minus: u64,
}

impl CancellationLadder {
    /// Build the ladder from one exposure result, recomputing every tally
    /// from the audit rows (derived views, never a second authority).
    pub fn from_exposure(exposure: &FrozenPolicyExposure) -> CancellationLadder {
        let changed = u64::try_from(exposure.rows.iter().filter(|row| row.u0 != row.u1).count())
            .expect("fits");
        assert!(
            exposure
                .rows
                .iter()
                .all(|row| row.u0 == row.u1 || row.split.is_some()),
            "L2-T1: a changed outcome occurs only on an exposed world"
        );
        assert_eq!(
            changed,
            exposure.corrections_plus + exposure.corrections_minus,
            "Boolean payoff: O_ρ = C_ρ⁺ + C_ρ⁻ pointwise, so the sums agree"
        );
        let ladder = CancellationLadder {
            policy: exposure.policy,
            field0: exposure.field0,
            field1: exposure.field1,
            root_id: exposure.root_id,
            domain: exposure.domain.clone(),
            worlds: exposure.worlds,
            exposed: exposure.exposed,
            outcome_changed: changed,
            c_plus: exposure.corrections_plus,
            c_minus: exposure.corrections_minus,
        };
        // §31: |c| ≤ r ≤ d, exactly.
        let c_abs = {
            let c = ladder.c();
            if c < BigRational::zero() {
                -c
            } else {
                c
            }
        };
        assert!(
            c_abs <= ladder.r() && ladder.r() <= ladder.d(),
            "the cancellation ladder |c| ≤ r ≤ d holds exactly"
        );
        // The three zeros nest (they are DISTINCT statements; the nesting
        // is the ladder's, not a license to collapse them).
        if ladder.behavioral_irrelevance() {
            assert!(ladder.outcome_irrelevance(), "d = 0 forces r = 0");
        }
        if ladder.outcome_irrelevance() {
            assert!(ladder.value_neutrality(), "r = 0 forces c = 0");
        }
        ladder
    }

    /// The field-exposure mass `d_ρ` over the declared domain.
    pub fn d(&self) -> BigRational {
        BigRational::new(BigInt::from(self.exposed), BigInt::from(self.worlds))
    }

    /// The outcome-change mass `r_ρ = E[1{u1 ≠ u0}]`.
    pub fn r(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.outcome_changed),
            BigInt::from(self.worlds),
        )
    }

    /// `c_ρ⁺`.
    pub fn c_plus_mass(&self) -> BigRational {
        BigRational::new(BigInt::from(self.c_plus), BigInt::from(self.worlds))
    }

    /// `c_ρ⁻`.
    pub fn c_minus_mass(&self) -> BigRational {
        BigRational::new(BigInt::from(self.c_minus), BigInt::from(self.worlds))
    }

    /// The net value correction `c_ρ = c_ρ⁺ − c_ρ⁻`.
    pub fn c(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.c_plus) - BigInt::from(self.c_minus),
            BigInt::from(self.worlds),
        )
    }

    /// Zero scale 1 — **behavioral irrelevance**: the fields never even
    /// act differently (`d = 0`).
    pub fn behavioral_irrelevance(&self) -> bool {
        self.exposed == 0
    }

    /// Zero scale 2 — **terminal-outcome irrelevance**: behavioral
    /// differences never alter make/fail (`r = 0`).
    pub fn outcome_irrelevance(&self) -> bool {
        self.outcome_changed == 0
    }

    /// Zero scale 3 — **value neutrality**: positive and negative terminal
    /// changes cancel exactly in expectation (`c = 0`); `r` may be
    /// positive.
    pub fn value_neutrality(&self) -> bool {
        self.c_plus == self.c_minus
    }
}

impl fmt::Display for CancellationLadder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CancellationLadder{{policy={};field0={};field1={};root={:#018x};domain={};\
             d={};r={};c_plus={};c_minus={};c={}}}",
            self.policy,
            self.field0,
            self.field1,
            self.root_id,
            self.domain,
            self.d(),
            self.r(),
            self.c_plus_mass(),
            self.c_minus_mass(),
            self.c()
        )
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — the six-label cancellation vocabulary (response §41;
// PANEL-A7 — Rust naming free, distinctions binding).
// ---------------------------------------------------------------------------

/// The six binding cancellation result labels. Mechanically distinct in
/// type and serialization; no report may flatten them. `Dominated` is a
/// PAIRWISE, objective-level statement (`H(a|b) = 0 ∧ B(a|b) > 0`); the
/// first four are fixed-policy cancellation statements; `Unresolved`
/// means none of the others has been ESTABLISHED — which is all a sampled
/// domain can ever report here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CancellationKind {
    /// `d = 0` established exactly: behavioral irrelevance.
    NoFieldExposure,
    /// `r = 0` established exactly (with `d > 0`): outcome irrelevance.
    OutcomeStable,
    /// `c = 0` established exactly (with `r > 0`): value neutrality.
    ValueNeutral,
    /// `|c| < ε` certified against a DECLARED ε (exact `c`, exact
    /// comparison).
    EpsilonEquivalent,
    /// `H(a|b) = 0 ∧ B(a|b) > 0` established by exact enumeration —
    /// strict dominance, one-sided unforced risk, never "cancellation"
    /// (response §34).
    Dominated,
    /// None of the above established.
    Unresolved,
}

impl CancellationKind {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(self) -> &'static str {
        match self {
            CancellationKind::NoFieldExposure => "NoFieldExposure",
            CancellationKind::OutcomeStable => "OutcomeStable",
            CancellationKind::ValueNeutral => "ValueNeutral",
            CancellationKind::EpsilonEquivalent => "EpsilonEquivalent",
            CancellationKind::Dominated => "Dominated",
            CancellationKind::Unresolved => "Unresolved",
        }
    }

    pub const ALL: [CancellationKind; 6] = [
        CancellationKind::NoFieldExposure,
        CancellationKind::OutcomeStable,
        CancellationKind::ValueNeutral,
        CancellationKind::EpsilonEquivalent,
        CancellationKind::Dominated,
        CancellationKind::Unresolved,
    ];
}

impl fmt::Display for CancellationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag())
    }
}

/// Classify one fixed-policy ladder into the vocabulary, most-specific
/// first. A non-`ExactFiber` domain establishes NOTHING (PANEL-A7:
/// sampled zeros never prove a zero) and always reports `Unresolved`.
/// `EpsilonEquivalent` requires a DECLARED ε; the comparison is exact.
pub fn fixed_policy_cancellation_kind(
    ladder: &CancellationLadder,
    epsilon: Option<&BigRational>,
) -> CancellationKind {
    if ladder.domain != WorldDomain::ExactFiber {
        return CancellationKind::Unresolved;
    }
    if ladder.behavioral_irrelevance() {
        return CancellationKind::NoFieldExposure;
    }
    if ladder.outcome_irrelevance() {
        return CancellationKind::OutcomeStable;
    }
    if ladder.value_neutrality() {
        return CancellationKind::ValueNeutral;
    }
    if let Some(eps) = epsilon {
        assert!(
            *eps > BigRational::zero(),
            "a declared equivalence ε is positive"
        );
        let c_abs = {
            let c = ladder.c();
            if c < BigRational::zero() {
                -c
            } else {
                c
            }
        };
        if c_abs < *eps {
            return CancellationKind::EpsilonEquivalent;
        }
    }
    CancellationKind::Unresolved
}

// ---------------------------------------------------------------------------
// SLICE 3 — pairwise benefit and hazard masses (response §33–§34;
// PANEL-A7). [CE pairwise; dominance objective-level]
// ---------------------------------------------------------------------------

/// Response §33 — the pairwise report for two frozen policies under ONE
/// field, retaining ALL of `(B, H, q, g)` (PANEL-A7: a small `|g|` from
/// near-agreement and from heavy exchange are different objects, never
/// one "close" label). Exact-enumeration tier: the constructor counted
/// the COMPLETE fiber, so `H = 0` here is established, and
/// [`Self::dominance_kind`] is the vocabulary's only route to
/// `Dominated`. Fields are private so no hand-built value can impersonate
/// the exact tier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPairwiseMasses {
    policy_a: String,
    policy_b: String,
    field: FieldId,
    root_id: u64,
    fiber: u128,
    benefit: u64,
    hazard: u64,
    both_make: u64,
    both_fail: u64,
}

impl ExactPairwiseMasses {
    pub fn policy_a(&self) -> &str {
        &self.policy_a
    }

    pub fn policy_b(&self) -> &str {
        &self.policy_b
    }

    pub fn field(&self) -> FieldId {
        self.field
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    pub fn fiber(&self) -> u128 {
        self.fiber
    }

    /// Worlds with `u_a = 1, u_b = 0`.
    pub fn benefit_worlds(&self) -> u64 {
        self.benefit
    }

    /// Worlds with `u_a = 0, u_b = 1`.
    pub fn hazard_worlds(&self) -> u64 {
        self.hazard
    }

    pub fn both_make_worlds(&self) -> u64 {
        self.both_make
    }

    pub fn both_fail_worlds(&self) -> u64 {
        self.both_fail
    }

    /// `B(a|b)`, exact under the uniform fiber measure.
    pub fn b(&self) -> BigRational {
        BigRational::new(BigInt::from(self.benefit), BigInt::from(self.fiber))
    }

    /// `H(a|b)`.
    pub fn h(&self) -> BigRational {
        BigRational::new(BigInt::from(self.hazard), BigInt::from(self.fiber))
    }

    /// The gap `g(a,b) = B − H` (signed).
    pub fn g(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.benefit) - BigInt::from(self.hazard),
            BigInt::from(self.fiber),
        )
    }

    /// The exchange mass `q(a,b) = B + H`.
    pub fn q(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.benefit) + BigInt::from(self.hazard),
            BigInt::from(self.fiber),
        )
    }

    /// Response §34 — the dominance route, the vocabulary's ONLY producer
    /// of `Dominated`: `H(a|b) = 0 ∧ B(a|b) > 0` counted over the whole
    /// fiber is strict dominance in expected value — one-sided unforced
    /// risk, never "cancellation". Anything else is `Unresolved` here.
    /// Dominance pruning changes no objective; choosing among
    /// non-dominated exact ties by any secondary criterion must be
    /// declared separately (§34).
    pub fn dominance_kind(&self) -> CancellationKind {
        if self.hazard == 0 && self.benefit > 0 {
            CancellationKind::Dominated
        } else {
            CancellationKind::Unresolved
        }
    }
}

impl fmt::Display for ExactPairwiseMasses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ExactPairwiseMasses{{a={};b={};field={};root={:#018x};fiber={};\
             B={};H={};q={};g={}}}",
            self.policy_a,
            self.policy_b,
            self.field,
            self.root_id,
            self.fiber,
            self.b(),
            self.h(),
            self.q(),
            self.g()
        )
    }
}

/// The sampled sibling of [`ExactPairwiseMasses`] over a declared
/// stream-prefix: exact counts over the enumerated sample, ESTIMATES of
/// the masses. Deliberately has NO `dominance_kind` — a finite sample
/// with zero observed hazards does not prove `H = 0` (response §34;
/// PANEL-A7), and this type's shape is what makes that a compile-time
/// impossibility rather than a convention.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampledPairwiseMasses {
    policy_a: String,
    policy_b: String,
    field: FieldId,
    root_id: u64,
    epoch: u64,
    worlds: u64,
    benefit: u64,
    hazard: u64,
    both_make: u64,
    both_fail: u64,
}

impl SampledPairwiseMasses {
    pub fn policy_a(&self) -> &str {
        &self.policy_a
    }

    pub fn policy_b(&self) -> &str {
        &self.policy_b
    }

    pub fn field(&self) -> FieldId {
        self.field
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    /// The declared domain, part of the claim.
    pub fn domain(&self) -> WorldDomain {
        WorldDomain::StreamPrefix {
            epoch: self.epoch,
            worlds: self.worlds,
        }
    }

    pub fn benefit_worlds(&self) -> u64 {
        self.benefit
    }

    pub fn hazard_worlds(&self) -> u64 {
        self.hazard
    }

    pub fn both_make_worlds(&self) -> u64 {
        self.both_make
    }

    pub fn both_fail_worlds(&self) -> u64 {
        self.both_fail
    }

    /// The sample estimate of `B(a|b)` — exact over the enumerated
    /// prefix, an estimate of the fiber mass.
    pub fn b_estimate(&self) -> BigRational {
        BigRational::new(BigInt::from(self.benefit), BigInt::from(self.worlds))
    }

    /// The sample estimate of `H(a|b)`.
    pub fn h_estimate(&self) -> BigRational {
        BigRational::new(BigInt::from(self.hazard), BigInt::from(self.worlds))
    }

    /// The sample estimate of `g(a,b)`.
    pub fn g_estimate(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.benefit) - BigInt::from(self.hazard),
            BigInt::from(self.worlds),
        )
    }

    /// The sample estimate of `q(a,b)`.
    pub fn q_estimate(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.benefit) + BigInt::from(self.hazard),
            BigInt::from(self.worlds),
        )
    }
}

impl fmt::Display for SampledPairwiseMasses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SampledPairwiseMasses{{estimate;a={};b={};field={};root={:#018x};domain={};\
             B_hat={};H_hat={};q_hat={};g_hat={}}}",
            self.policy_a,
            self.policy_b,
            self.field,
            self.root_id,
            self.domain(),
            self.b_estimate(),
            self.h_estimate(),
            self.q_estimate(),
            self.g_estimate()
        )
    }
}

/// Count the pairwise masses of two policies under one field by EXACT
/// enumeration of the complete fiber (the §34 dominance route's required
/// evidence). The four cells are a census: `B + H + both-make +
/// both-fail = |Φ|`, asserted.
pub fn exact_pairwise_masses(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &FieldModel,
) -> ExactPairwiseMasses {
    let viewer = root.kernel().viewer();
    let root_id = crate::solver::adaptive::root_identity(root, position);
    let mut benefit = 0u64;
    let mut hazard = 0u64;
    let mut both_make = 0u64;
    let mut both_fail = 0u64;
    let mut visited = 0u128;
    for world in root.worlds() {
        let ua = replay_viewer_success(position, viewer, &world, policy_a, field);
        let ub = replay_viewer_success(position, viewer, &world, policy_b, field);
        match (ua, ub) {
            (true, false) => benefit += 1,
            (false, true) => hazard += 1,
            (true, true) => both_make += 1,
            (false, false) => both_fail += 1,
        }
        visited += 1;
    }
    assert_eq!(
        visited,
        root.count(),
        "exact pairwise masses enumerate the whole fiber exactly once"
    );
    assert_eq!(
        u128::from(benefit) + u128::from(hazard) + u128::from(both_make) + u128::from(both_fail),
        visited,
        "the four pairwise cells are a census of the fiber"
    );
    ExactPairwiseMasses {
        policy_a: policy_a.id().to_string(),
        policy_b: policy_b.id().to_string(),
        field: field.field_id(),
        root_id,
        fiber: visited,
        benefit,
        hazard,
        both_make,
        both_fail,
    }
}

/// Count the pairwise masses over the indexed evidence-stream prefix
/// `0..worlds` at `epoch` — estimates only, typed so (response §34).
pub fn sampled_pairwise_masses(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &FieldModel,
    epoch: u64,
    worlds: u64,
) -> SampledPairwiseMasses {
    assert!(worlds >= 1, "a declared prefix holds at least one world");
    let viewer = root.kernel().viewer();
    let root_id = crate::solver::adaptive::root_identity(root, position);
    let mut benefit = 0u64;
    let mut hazard = 0u64;
    let mut both_make = 0u64;
    let mut both_fail = 0u64;
    for i in 0..worlds {
        let world = root.world_at(root_id, epoch, i);
        let ua = replay_viewer_success(position, viewer, &world, policy_a, field);
        let ub = replay_viewer_success(position, viewer, &world, policy_b, field);
        match (ua, ub) {
            (true, false) => benefit += 1,
            (false, true) => hazard += 1,
            (true, true) => both_make += 1,
            (false, false) => both_fail += 1,
        }
    }
    assert_eq!(
        benefit + hazard + both_make + both_fail,
        worlds,
        "the four pairwise cells are a census of the sample"
    );
    SampledPairwiseMasses {
        policy_a: policy_a.id().to_string(),
        policy_b: policy_b.id().to_string(),
        field: field.field_id(),
        root_id,
        epoch,
        worlds,
        benefit,
        hazard,
        both_make,
        both_fail,
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — the pair lift Λ and its evidence processes (parent §3.3,
// §9.1–9.2; response §32). [L2 lift; CE evidence machinery consumed
// one-directionally]
// ---------------------------------------------------------------------------

/// The fixed-pair field lift `Λ_{a,b} = c_{ρ_a} − c_{ρ_b}` with the §3.3
/// bound `|Λ| ≤ d_{ρ_a} + d_{ρ_b}` asserted at construction. A
/// fixed-PAIR statement: no decision claim (that is the screens' job, on
/// valid root-action bounds only).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairLift {
    pub policy_a: PolicyId,
    pub policy_b: PolicyId,
    pub lambda: BigRational,
    /// The §3.3 exposure bound `d_a + d_b`.
    pub bound: BigRational,
}

/// Compute the pair lift from two ladders of the SAME root, field pair,
/// and declared domain (asserted — a lift across domains is not a
/// statement).
pub fn pair_lift(a: &CancellationLadder, b: &CancellationLadder) -> PairLift {
    assert_eq!(a.root_id, b.root_id, "one root");
    assert_eq!(a.field0, b.field0, "one σ0");
    assert_eq!(a.field1, b.field1, "one σ1");
    assert_eq!(a.domain, b.domain, "one declared world domain");
    let lambda = a.c() - b.c();
    let bound = a.d() + b.d();
    let lambda_abs = if lambda < BigRational::zero() {
        -lambda.clone()
    } else {
        lambda.clone()
    };
    assert!(
        lambda_abs <= bound,
        "the §3.3 fixed-pair correction bound |Λ| ≤ d_a + d_b holds exactly"
    );
    PairLift {
        policy_a: a.policy,
        policy_b: b.policy,
        lambda,
        bound,
    }
}

/// §9.1 — the signed-pivotal evidence process applied to the fixed-policy
/// correction variable `C_ρ`: the exact pivotal evidence on the
/// `(c⁺, c⁻)` counts, consumed verbatim from `solver::evidence` (the CE
/// machinery is consumed, never reimplemented).
pub fn correction_pivotal_evidence(ladder: &CancellationLadder) -> BigRational {
    pivotal_evidence(ladder.c_plus, ladder.c_minus)
}

/// §9.2 — the pair-lift bounded-mean evidence process: per world,
/// `Z = C_{ρ_a} − C_{ρ_b} ∈ {−2..2}` with `E[Z] = Λ_{a,b}`, driven
/// through the CE bounded-mean engine on `X = Z/2 ∈ [−1, 1]`. `Z` is
/// never reduced to its sign — difference magnitude matters (§9.2).
pub struct PairLiftProcess {
    mixture: BoundedMeanMixture,
}

impl PairLiftProcess {
    /// A lawful process testing the declared null about `Λ` (the
    /// threshold is given on the Λ scale and halved onto the `X` scale).
    pub fn new(
        null: MeanNull,
        lambda_threshold: &BigRational,
        mixture: &[(BigRational, BigRational)],
    ) -> Result<PairLiftProcess, MixtureError> {
        let two = BigRational::from_integer(BigInt::from(2));
        BoundedMeanMixture::new(
            null,
            -BigRational::one(),
            BigRational::one(),
            lambda_threshold / two,
            mixture,
        )
        .map(|mixture| PairLiftProcess { mixture })
    }

    /// Fold one world's pair of correction variables (`C_{ρ_a}`,
    /// `C_{ρ_b}`, each in `{−1, 0, 1}` — asserted).
    pub fn observe(&mut self, correction_a: i8, correction_b: i8) {
        assert!(
            (-1..=1).contains(&correction_a) && (-1..=1).contains(&correction_b),
            "a Boolean-payoff correction variable lies in {{-1, 0, 1}}"
        );
        let z = i16::from(correction_a) - i16::from(correction_b);
        self.mixture
            .observe(&BigRational::new(BigInt::from(z), BigInt::from(2)));
    }

    /// The current exact mixture evidence.
    pub fn evidence(&self) -> BigRational {
        self.mixture.evidence()
    }

    pub fn observations(&self) -> u64 {
        self.mixture.observations()
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — the directional screen (response §35–§37; PANEL-A8). [L2]
// ---------------------------------------------------------------------------

/// One legal root action's directional bound pair. Wraps
/// [`RootActionDirectionalUpper`] and NOTHING else — the O31/O34 lock in
/// its directional form: no fixed-policy or sampled observation can be
/// given to the directional screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionDirectionalUpper {
    pub action: Domino,
    pub bound: RootActionDirectionalUpper,
}

/// One action's directional screen row: `L^(1) = L^(0) − (R⁻)^U`,
/// `U^(1) = U^(0) + (R⁺)^U` (response §37) — derived views of the stored
/// row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionalRow {
    pub action: Domino,
    pub lower0: BigRational,
    pub upper0: BigRational,
    pub directional: RootActionDirectionalUpper,
}

impl DirectionalRow {
    /// §37 — `L_a^(1) = L_a^(0) − (R_a^-)^U`: only "the action gets
    /// worse" can lower it.
    pub fn lower1(&self) -> BigRational {
        &self.lower0 - self.directional.screenable_minus()
    }

    /// §37 — `U_a^(1) = U_a^(0) + (R_a^+)^U`: only "the action gets
    /// better" can raise it.
    pub fn upper1(&self) -> BigRational {
        &self.upper0 + self.directional.screenable_plus()
    }
}

/// Response §37 — the L2-T4 bar construction over DIRECTIONAL intervals:
/// `B̄ = max_a L_a^(1)`, `𝓐₁ = {a : U_a^(1) ≥ B̄}`, with the same O38
/// all-action admission audit and tier discipline as the symmetric
/// [`AdmissibleScreen`]. Because `(R^±)^U ≤ R^U`, the directional
/// admissible set is a subset of the symmetric one wherever both are
/// computed from the same baselines (gated in tests, not assumed).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionalScreen {
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The Stage-1 baseline tier of every `[L^(0), U^(0)]` input.
    pub tier: BaselineTier,
    rows: Vec<DirectionalRow>,
}

impl DirectionalScreen {
    /// Build the screen; the O38 audit is verbatim from the symmetric
    /// screen: every legal action gets a baseline bound and a directional
    /// bound before any exclusion, nothing extra, nothing twice.
    pub fn compute(
        legal: DominoSet,
        tier: BaselineTier,
        bounds: &[ActionBound],
        directionals: &[ActionDirectionalUpper],
        field0: FieldId,
        field1: FieldId,
        root_id: u64,
    ) -> DirectionalScreen {
        assert!(!legal.is_empty(), "a screened decision has a legal action");
        assert_eq!(
            bounds.len(),
            legal.len(),
            "O38: exactly one baseline bound per legal action"
        );
        assert_eq!(
            directionals.len(),
            legal.len(),
            "O38: exactly one directional bound per legal action"
        );
        let zero = BigRational::zero();
        let one = BigRational::one();
        let rows: Vec<DirectionalRow> = legal
            .iter()
            .map(|action| {
                let matching: Vec<&ActionBound> =
                    bounds.iter().filter(|b| b.action == action).collect();
                assert_eq!(
                    matching.len(),
                    1,
                    "O38: one baseline bound names legal action {action}"
                );
                let bound = matching[0];
                let matching: Vec<&ActionDirectionalUpper> =
                    directionals.iter().filter(|d| d.action == action).collect();
                assert_eq!(
                    matching.len(),
                    1,
                    "O38: one directional bound names legal action {action}"
                );
                let directional = matching[0];
                assert!(
                    zero <= bound.lower && bound.lower <= bound.upper && bound.upper <= one,
                    "a baseline interval is a valid probability interval"
                );
                DirectionalRow {
                    action,
                    lower0: bound.lower.clone(),
                    upper0: bound.upper.clone(),
                    directional: directional.bound.clone(),
                }
            })
            .collect();
        DirectionalScreen {
            field0,
            field1,
            root_id,
            tier,
            rows,
        }
    }

    pub fn rows(&self) -> &[DirectionalRow] {
        &self.rows
    }

    pub fn row(&self, action: Domino) -> &DirectionalRow {
        self.rows
            .iter()
            .find(|r| r.action == action)
            .expect("a screened action")
    }

    /// §37 — the bar `B̄ = max_a L_a^(1)`.
    pub fn bar(&self) -> BigRational {
        self.rows
            .iter()
            .map(DirectionalRow::lower1)
            .max()
            .expect("a screened decision has a legal action")
    }

    /// Membership: `a ∈ 𝓐₁ ⇔ U_a^(1) ≥ B̄`.
    pub fn admitted(&self, action: Domino) -> bool {
        self.row(action).upper1() >= self.bar()
    }

    /// The admissible set, in legal-set order; never empty (the bar's
    /// witness row has `U^(1) ≥ L^(1) = B̄`).
    pub fn admissible(&self) -> Vec<Domino> {
        let bar = self.bar();
        let set: Vec<Domino> = self
            .rows
            .iter()
            .filter(|r| r.upper1() >= bar)
            .map(|r| r.action)
            .collect();
        assert!(
            !set.is_empty(),
            "the admissible set holds the bar's witness"
        );
        set
    }

    /// The typed result kind — the same seven-kind vocabulary and the
    /// same tier discipline as the symmetric screen (L2-A3).
    pub fn kind(&self) -> FieldSwapKind {
        if self.tier == BaselineTier::Unresolved {
            return FieldSwapKind::FieldUnresolved;
        }
        if self.admissible().len() == 1 {
            match self.tier {
                BaselineTier::ExactRoot => FieldSwapKind::FieldStableExactRoot,
                BaselineTier::ExactFrozenSet => FieldSwapKind::FieldStableExactFrozenSet,
                BaselineTier::DeltaFrozenSet => FieldSwapKind::FieldStableDeltaFrozenSet,
                BaselineTier::Unresolved => unreachable!("returned above"),
            }
        } else {
            FieldSwapKind::FieldSensitive
        }
    }

    /// Response §36 — the directional slack of one ordered pair:
    /// `L_a^(0) − U_b^(0) − (R_a^-)^U − (R_b^+)^U`. Only two directions
    /// can overturn a winner — the winner gets worse or the rival gets
    /// better — so this pays for exactly those, where the symmetric slack
    /// pays for impossible directions too.
    pub fn directional_slack(&self, a: Domino, b: Domino) -> BigRational {
        assert_ne!(a, b, "slack compares two distinct actions");
        let ra = self.row(a);
        let rb = self.row(b);
        &ra.lower0
            - &rb.upper0
            - ra.directional.screenable_minus()
            - rb.directional.screenable_plus()
    }

    /// Response §36 — the directional winner-stability premise for the
    /// ordered pair: positive directional slack implies
    /// `Q_a^(1) > Q_b^(1)` (gated against exact σ1 values in tests,
    /// wherever both exist).
    pub fn winner_stable_over(&self, a: Domino, b: Domino) -> bool {
        self.directional_slack(a, b) > BigRational::zero()
    }

    /// The complete ordered-pair directional slack table, in legal-set
    /// order.
    pub fn slack_table(&self) -> Vec<StabilitySlack> {
        let mut table = Vec::new();
        for ra in &self.rows {
            for rb in &self.rows {
                if ra.action == rb.action {
                    continue;
                }
                table.push(StabilitySlack {
                    a: ra.action,
                    b: rb.action,
                    slack: self.directional_slack(ra.action, rb.action),
                });
            }
        }
        table
    }
}

impl fmt::Display for DirectionalScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DirectionalScreen{{kind={};tier={};field0={};field1={};root={:#018x};bar={};rows=[",
            self.kind(),
            self.tier.label(),
            self.field0,
            self.field1,
            self.root_id,
            self.bar()
        )?;
        for (k, row) in self.rows.iter().enumerate() {
            if k > 0 {
                write!(f, ",")?;
            }
            write!(
                f,
                "{}:[{},{}]&{}->[{},{}]:{}",
                row.action,
                row.lower0,
                row.upper0,
                row.directional,
                row.lower1(),
                row.upper1(),
                if self.admitted(row.action) {
                    "admitted"
                } else {
                    "excluded"
                }
            )?;
        }
        write!(f, "]}}")
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — Stage 4: field-1 work only on survivors (parent §8 Stage 4).
// [L2]
// ---------------------------------------------------------------------------

/// The Stage-4 result: exact σ1 frozen-set values for the SURVIVORS only
/// (`None` when the admissible set was already a singleton — zero σ1
/// work), the σ0-settled action, the σ1-selected action, and the typed
/// Stage-5 kind. Excluded actions consumed no field-1 budget — asserted
/// by construction (`values1` covers exactly the survivors).
pub struct SurvivorEvaluation {
    /// The admissible set the evaluation ran on, in legal-set order.
    pub survivors: Vec<Domino>,
    /// σ1 exact frozen values for the survivors only.
    pub values1: Option<ExactFrozenBaseline>,
    /// The σ0-settled action: first σ0-argmax in candidate order.
    pub settled0: Domino,
    /// The selected action after Stage 4: the singleton survivor, or the
    /// first σ1-argmax among survivors in survivor order.
    pub selected1: Domino,
    /// `FieldDecisionChanged` when the settled choice moved; otherwise
    /// the screen's own kind.
    pub kind: FieldSwapKind,
}

impl SurvivorEvaluation {
    pub fn decision_changed(&self) -> bool {
        self.selected1 != self.settled0
    }
}

/// Run §8 Stage 4 at the `ExactFrozenSet` tier: exact σ1 values for the
/// screen's admissible set ONLY. Actions outside `𝓐₁` remain excluded by
/// the field-stability bound and consume no field-1 budget — mechanically
/// true here because they are never handed to the σ1 evaluator at all.
pub fn survivor_stage4(
    root: &CanonicalRoot,
    position: &RootPosition,
    screen: &AdmissibleScreen,
    baseline0: &ExactFrozenBaseline,
    candidates: &[(Domino, &FrozenPolicy)],
    field1: &FieldModel,
    scope: &str,
) -> SurvivorEvaluation {
    assert_eq!(
        screen.tier,
        BaselineTier::ExactFrozenSet,
        "this Stage-4 route is the exact-frozen-set tier's"
    );
    let admissible = screen.admissible();
    // The σ0-settled action: first σ0-argmax in candidate order (the
    // declared deterministic selection rule of this slice).
    let best0 = baseline0
        .values
        .iter()
        .max()
        .expect("a baselined action")
        .clone();
    let settled0 = *baseline0
        .actions
        .iter()
        .zip(&baseline0.values)
        .find(|(_, v)| **v == best0)
        .map(|(a, _)| a)
        .expect("the maximum is attained");
    let survivors: Vec<(Domino, &FrozenPolicy)> = candidates
        .iter()
        .filter(|(a, _)| admissible.contains(a))
        .copied()
        .collect();
    assert_eq!(
        survivors.len(),
        admissible.len(),
        "every survivor has its frozen candidate (O38 coverage carried into Stage 4)"
    );
    if let [(selected, _)] = survivors[..] {
        // A singleton admissible set: with point baselines the σ0-settled
        // action is always admitted (U^(1) ≥ Q^(0)_max ≥ B̄), so the
        // singleton IS the settled action, and no σ1 work runs at all.
        assert_eq!(
            selected, settled0,
            "a singleton admissible set holds the σ0-settled action"
        );
        return SurvivorEvaluation {
            survivors: vec![selected],
            values1: None,
            settled0,
            selected1: selected,
            kind: screen.kind(),
        };
    }
    let values1 = exact_frozen_action_values(root, position, &survivors, field1, scope);
    let best1 = values1
        .values
        .iter()
        .max()
        .expect("a survivor value")
        .clone();
    let selected1 = *values1
        .actions
        .iter()
        .zip(&values1.values)
        .find(|(_, v)| **v == best1)
        .map(|(a, _)| a)
        .expect("the maximum is attained");
    let kind = if selected1 == settled0 {
        screen.kind()
    } else {
        FieldSwapKind::FieldDecisionChanged
    };
    SurvivorEvaluation {
        survivors: survivors.iter().map(|(a, _)| *a).collect(),
        values1: Some(values1),
        settled0,
        selected1,
        kind,
    }
}

// ---------------------------------------------------------------------------
// SLICE 3 — the Stage-1 ExactRoot baseline route (§8 Stage 1, ExactRoot
// tier; §15.3). [L2]
// ---------------------------------------------------------------------------

/// §8 Stage 1 at the `ExactRoot` tier: the exact optimized value
/// `Q_a^(field)` for every legal root action under the named field,
/// through `solver::exposure`'s exact root optimizer. Point intervals for
/// the screens; a singleton admissible set over these IS
/// `FieldStableExactRoot`.
pub fn exact_root_bounds(
    root: &CanonicalRoot,
    position: &RootPosition,
    legal: DominoSet,
    field: &FieldModel,
) -> Vec<ActionBound> {
    assert!(!legal.is_empty(), "a screened decision has a legal action");
    legal
        .iter()
        .map(|action| {
            let value = exact_root_value(root, position, action, field).value();
            ActionBound {
                action,
                lower: value.clone(),
                upper: value,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// SLICE 3 — first-split traces and their aggregation (parent §10, §10.1).
// [L2 explanation surface]
// ---------------------------------------------------------------------------

/// Parent §10 — the per-world explanation record for a world contributing
/// to a material field correction: identities, the first split (which
/// carries the common public record `R*`, the acting seat, that seat's
/// private hand, and both chosen tiles), and the terminal outcome under
/// each field. Structural motif tags (§10 item 14) are deliberately NOT
/// fields of this record: the first-split morphology classifier and the
/// raw suffix enrichment live in `solver::motif` (x:024 Part 3,
/// TRIPLE-A6/A7), and motif labels are derived views there, never
/// persisted trace state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldSplitTrace {
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// Canonical world identity (four hand bitmasks, seat-indexed).
    pub world: [u32; 4],
    /// The focal root action.
    pub action: Domino,
    /// The frozen focal PolicyId.
    pub policy: PolicyId,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The first field split (seat, trick/ply, both tiles, the acting
    /// seat's hand, the common public record).
    pub split: FirstSplit,
    /// Terminal make indicator under (ρ, σ0).
    pub u0: bool,
    /// Terminal make indicator under (ρ, σ1).
    pub u1: bool,
}

impl FieldSplitTrace {
    /// Whether the correction favors (+1) or harms (−1) the root action.
    pub fn favors(&self) -> i8 {
        i8::from(self.u1) - i8::from(self.u0)
    }
}

/// The correction-pivotal traces of one exposure result: one
/// [`FieldSplitTrace`] per world whose terminal outcome CHANGED (L2-T1
/// guarantees each such world carries a first split — asserted).
pub fn field_split_traces(exposure: &FrozenPolicyExposure, action: Domino) -> Vec<FieldSplitTrace> {
    exposure
        .rows
        .iter()
        .filter(|row| row.u0 != row.u1)
        .map(|row| FieldSplitTrace {
            root_id: exposure.root_id,
            world: row.world,
            action,
            policy: exposure.policy,
            field0: exposure.field0,
            field1: exposure.field1,
            split: row
                .split
                .clone()
                .expect("L2-T1: a changed outcome occurs only on an exposed world"),
            u0: row.u0,
            u1: row.u1,
        })
        .collect()
}

/// Parent §10.1 — the aggregate explanation of one exposure result:
/// split mass, signed correction masses, first-split seat and trick
/// histograms, and the conditional outcome difference. Every tally is
/// recomputed from the audit rows and asserted against the exposure's own
/// (derived views, never a second authority).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitAggregate {
    pub worlds: u64,
    /// Mass reaching any field split.
    pub exposed: u64,
    /// Mass of positive corrections.
    pub plus: u64,
    /// Mass of negative corrections.
    pub minus: u64,
    /// First-split counts by acting seat (absolute solver seats S0..S3).
    pub by_seat: [u64; 4],
    /// First-split counts by 1-based trick, ascending.
    pub by_trick: Vec<(usize, u64)>,
}

impl SplitAggregate {
    pub fn from_exposure(exposure: &FrozenPolicyExposure) -> SplitAggregate {
        let mut exposed = 0u64;
        let mut plus = 0u64;
        let mut minus = 0u64;
        let mut by_seat = [0u64; 4];
        let mut by_trick: BTreeMap<usize, u64> = BTreeMap::new();
        for row in &exposure.rows {
            if let Some(split) = &row.split {
                exposed += 1;
                by_seat[split.seat.index()] += 1;
                *by_trick.entry(split.trick).or_insert(0) += 1;
            }
            match (row.u0, row.u1) {
                (false, true) => plus += 1,
                (true, false) => minus += 1,
                _ => {}
            }
        }
        assert_eq!(exposed, exposure.exposed, "the split tally re-derives");
        assert_eq!(
            (plus, minus),
            (exposure.corrections_plus, exposure.corrections_minus),
            "the signed correction tallies re-derive"
        );
        assert_eq!(
            by_seat.iter().sum::<u64>(),
            exposed,
            "the seat histogram is a census of the splits"
        );
        assert_eq!(
            by_trick.values().sum::<u64>(),
            exposed,
            "the trick histogram is a census of the splits"
        );
        SplitAggregate {
            worlds: exposure.worlds,
            exposed,
            plus,
            minus,
            by_seat,
            by_trick: by_trick.into_iter().collect(),
        }
    }

    /// §10.1 — the conditional outcome difference
    /// `(c⁺ − c⁻) / d` (per split-reaching world); `None` when nothing
    /// split.
    pub fn conditional_outcome_difference(&self) -> Option<BigRational> {
        (self.exposed > 0).then(|| {
            BigRational::new(
                BigInt::from(self.plus) - BigInt::from(self.minus),
                BigInt::from(self.exposed),
            )
        })
    }
}
