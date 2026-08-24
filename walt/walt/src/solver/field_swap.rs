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
//! ([`FieldSwapKind`]); this slice produces the three `FieldStable*`
//! kinds, `FieldSensitive` (survivors remain — field-1 work is confined
//! to them), and `FieldUnresolved`. `FieldDecisionChanged` and
//! `HeuristicFallback` are Stage-4/consumer outputs of a later slice and
//! exist here as types so no consumer is built against a smaller ladder.

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{CanonicalRoot, RootPosition};
use crate::solver::controller::{exact_frozen_set, CandidateSet, RiskPlan, SetResult, SetSpec};
use crate::solver::evidence::ScopedDelta;
use crate::solver::exposure::RootActionExposureUpper;
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::policy::{ActionRule, FrozenPolicy};

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
