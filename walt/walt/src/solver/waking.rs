//! `solver::waking` — the waking seat: the first walt player variant
//! that plays with a thinking-teammate model, escalating only where it
//! can matter.
//!
//! EXPLORATORY tier; CE thread for the baseline (CE = sampling depth)
//! and L2 thread for the escalation (L2 = model choice) — every number
//! read off this surface is labeled with its thread. Sits below every
//! evidentiary tier and is cited by nothing above it. Estimates, never
//! receipts; not a P-A21 statement. No floats anywhere.
//!
//! Operates under: the CE-A6 wake-up split (parent
//! `walt/math/calculated_evidence_v0.1.md` §14 — response, value, and
//! decision wake-ups are distinct and never collapsed); the CE-A7/§20.16
//! default-player fence (this module is a VARIANT surface: nothing here
//! touches `solver::act`'s action policy, `controller_bridge`, or any
//! default — the old player remains the default until arena gates and
//! Jason's word); and the L2-A4 screen discipline (the escalation
//! consumes `solver::targeted`'s screen machinery verbatim — sound
//! uppers only, lower witnesses steer spend and never touch the screen).
//! Parent sections: calculated_evidence §14/§16.4/§6;
//! targeted_level2_field_stability §8 Stages 1–5.
//!
//! ## The seat, per decision
//!
//! 1. **Baseline (CE thread).** `solver::act::act` runs exactly as the
//!    live bridge runs it — same candidates, same risk convention, same
//!    action policy. The σ0 baseline choice is ALWAYS computed and is
//!    the fallback for every path below. A forced play (one legal tile)
//!    plays immediately and runs no detection.
//! 2. **Wake check (hard-budgeted).** An inline paired detection between
//!    act's chosen tile and its strongest rival under the DECLARED EPOCH
//!    PAIR: σ0 = `Level0 { n0 = 2 }`, σ1 = `Level1 { n_outer = 4,
//!    n0 = 2 }`, frozen candidates at declared schedule [8, 2]
//!    (`ActionRule::PinnedThenLevel1`, the same `continuation_tuple`
//!    act freezes). σ0 here is the SAME field act's evaluation actually
//!    runs against (`Level0Field::new(2)` behind `ActConfig::n0_frozen =
//!    2`), which makes the σ0 detection leg an asserted reproduction of
//!    the baseline's own field — the step-9 pattern. Small fibers (at or
//!    under the declared exact cap) take the exact route
//!    (`frozen_policy_exposure` over the complete fiber +
//!    `exact_paired_detection`), which is cheaper than sampling there
//!    and settles exactly; larger fibers take `sampled_paired_detection`
//!    over a declared world budget ([`WakingConfig::wake_world_budget`],
//!    24 paired worlds by default — a resource limit, never a settlement
//!    rule).
//! 3. **The wake rule (positive evidence only).** The seat wakes exactly
//!    when the σ1 detection leg POSITIVELY selects the rival over act's
//!    chosen tile: `winner1 = B` on the exact route, a δ-settled σ1 pair
//!    decision with winner B on the sampled route. Everything else is no
//!    wake: σ1 selecting the baseline (settled agreement), an exact σ1
//!    tie, or a within-budget-unsettled probe — HONEST-OUTCOME
//!    DISCIPLINE: unsettled means the seat plays exactly today's level-0
//!    choice, recorded as such. Degradation is the current player, never
//!    fake certainty.
//! 4. **Pairing strategy (declared).** The strongest rival is derived
//!    from act's own σ0 evaluation ordering: on an `ExactFrozenSet`
//!    result, the non-chosen candidate with the most exact wins (ties
//!    toward the lowest candidate index); on a `DeltaSettled` result,
//!    the LAST candidate the controller eliminated (the last rival
//!    standing under act's own evidence process); on a fallback route
//!    with a recorded level-1 ranking, the best non-chosen member of the
//!    fallback set under that ranking; on a fallback route without one
//!    (singleton survivor), again the last-eliminated candidate.
//! 5. **Escalation on wake (L2 thread).** One frozen focal candidate
//!    per legal root action (legal-set order, O38) through
//!    `solver::targeted::targeted_root` under the declared budget, one
//!    σ1 [`FieldModel`] held for the whole hand (the insert-only field
//!    action cache is how σ1 cost amortizes; the escalation shares the
//!    wake check's stream epoch so its survivor work replays cached σ1
//!    states). Routing: `DeltaSingleton` and a δ-settled `selected1`
//!    play the selection; `ExactSurvivors` plays the singleton survivor
//!    or a STRICT σ1 argmax; every open state and every typed refusal
//!    falls back to the σ0 baseline choice, RECORDED as a fallback —
//!    typed refusals and honest open states are never degraded into
//!    picks.
//!
//! ## Risk accounting
//!
//! The waking layer declares its own run-level budget
//! ([`WakingConfig::delta_wake_run`]) and splits it per decision by
//! act's `decision_delta` convention (`δ_d = δ_run/(d(d+1))`,
//! telescoping). Every waking scope is prefixed `wake:` and [`decide`]
//! asserts the caller's run scope is not — so waking scopes are
//! mechanically disjoint from act's internal scopes. Per decision the
//! budget splits in half: the detection half funds the five
//! [`DetectionRiskPlan`] engines (each declared at δ_d/12; the pair
//! decision spends twice, once per field), the escalation half is the
//! `targeted_root` screen budget with per-action risks declared at
//! `δ_esc/(8m)` so the §1.8 ledger sum stays within budget at any legal
//! count m.
//!
//! ## Epoch fence
//!
//! The `l2_controller` probe's declared epoch pair is DIFFERENT
//! (σ0 = `Level0 { n0 = 8 }` there): numbers do not compose across the
//! two surfaces. This module's records are their own census; comparisons
//! to the l2_controller probe are epoch-labeled or not made.
//!
//! ## The census
//!
//! Every decision — including forced plays — emits one typed
//! [`WakingCensus`] record: ordinals, fiber, path taken, wake evidence
//! kind, escalation outcome, the σ0 and played choices, the agreement
//! flag, and integer-microsecond spend per phase plus worlds consumed by
//! the wake check. These records are the affordability census the
//! CPU-vs-GPU fork turns on.
//!
//! Type locks (compile_fail where typing is the property):
//!
//! A wake is a crossing witness, never a construction — there is no way
//! to build a play-driving [`WakeEvidence`] from an unsettled probe or
//! from nothing:
//!
//! ```compile_fail
//! use walt::rules::Domino;
//! use walt::solver::waking::WakeEvidence;
//! fn forge_a_wake_from_an_unsettled_probe(rival: Domino) -> WakeEvidence {
//!     // Private fields, no public constructor: this does not compile.
//!     WakeEvidence { kind: "forged", rival }
//! }
//! ```
//!
//! A recorded fallback carries no number — no numeric accessor exists,
//! so a refusal-backed fallback can never be read as an evaluation:
//!
//! ```compile_fail
//! use num_rational::BigRational;
//! use walt::solver::waking::RecordedFallback;
//! fn read_a_number_off_a_fallback(f: &RecordedFallback) -> BigRational {
//!     // No such method exists on the fallback type: this does not
//!     // compile.
//!     f.value()
//! }
//! ```

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{Domino, Team};
use crate::solver::act::{act, continuation_tuple, ActConfig, ActDecision, ActRoute};
use crate::solver::adaptive::{driven_root, CanonicalRoot, DrivenState, RootPosition};
use crate::solver::arena_decl_id;
use crate::solver::best_of;
use crate::solver::controller::SetResult;
use crate::solver::evidence::{decision_delta, ScopedDelta};
use crate::solver::exposure::{frozen_policy_exposure, WorldDomain};
use crate::solver::field::{FieldKind, FieldModel, FieldSpec};
use crate::solver::policy::{DecisionMode, FrozenPolicy, TieRule};
use crate::solver::targeted::{
    targeted_root, RungBudget, StageFourOutcome, TargetedConfig, TargetedRisk, TypedRefusal,
};
use crate::solver::wakeup::{
    exact_paired_detection, sampled_paired_detection, DecisionWakeUp, DetectionRiskPlan,
    ExactPairSelection, PairWinner, SampledDecisionKind, SampledDetectionSpec,
};

/// The declared common stream epoch of the wake check AND the escalation
/// (one epoch: the escalation's survivor replays revisit the wake
/// check's cached σ1 states).
pub const WAKE_EPOCH: u64 = 0;

// ---------------------------------------------------------------------------
// Phase timing — integer microseconds (no wall clock on wasm32, where the
// spend fields honestly read zero).
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

// ---------------------------------------------------------------------------
// Configuration — every count a declared approximation parameter (CE-A5),
// every cap a resource limit and never a settlement rule.
// ---------------------------------------------------------------------------

/// The declared knobs of one waking seat.
#[derive(Clone, Debug)]
pub struct WakingConfig {
    /// The σ0 baseline player's knobs, passed to `solver::act` verbatim.
    /// `n_outer_frozen`/`n0_frozen` also freeze the detection pair and
    /// the escalation's focal candidates (one candidate identity across
    /// all three consumers).
    pub act: ActConfig,
    /// σ1's declared outer count (`Level1 { n_outer, n0 }`).
    pub sigma1_n_outer: u64,
    /// σ1's declared inner count.
    pub sigma1_n0: u64,
    /// The wake check's declared paired-world budget on the sampled
    /// route.
    pub wake_world_budget: u64,
    /// Fibers at or under this count take the exact wake route (full
    /// enumeration — cheaper than sampling there, and settles exactly).
    pub wake_exact_fiber_cap: u128,
    /// The escalation's exact-fiber cap (`RungBudget::exact_fiber_cap`).
    pub escalation_exact_fiber_cap: u128,
    /// The escalation's δ-baseline stream-prefix length.
    pub escalation_baseline_prefix: u64,
    /// The escalation's E3 stream-prefix length.
    pub escalation_e3_prefix: u64,
    /// The waking layer's OWN run-level risk budget, split per decision
    /// by the `decision_delta` convention. Disjoint from act's run
    /// budget by scope prefix.
    pub delta_wake_run: BigRational,
    /// The declared response/practical-zero tolerance of the detection
    /// engines.
    pub eps_q: BigRational,
}

impl WakingConfig {
    /// The live declared configuration: act's interactive defaults
    /// (world cap 128, frozen schedule [8, 2]), σ1 = `Level1 { 4, 2 }`,
    /// wake budget 24 paired worlds, exact wake cap 1024, escalation
    /// caps at the l2_controller probe's declared resource limits
    /// (fiber 4096, baseline prefix 128, E3 prefix 24).
    ///
    /// The exact wake cap was retuned 64 → 1024 after the smoke census
    /// priced the routes: under the telescoping risk convention the
    /// 24-world sampled probe's settlement thresholds need a net
    /// pivotal margin the budget cannot hold (its honest outcome is
    /// almost always open — recorded, never forced), while the exact
    /// route settles ALWAYS and costs fractions of a second up to
    /// fiber ~1000. The wake gate's real coverage is the exact route;
    /// the sampled budget stays a declared cost bound above it.
    #[must_use]
    pub fn live() -> WakingConfig {
        WakingConfig {
            act: ActConfig::interactive(),
            sigma1_n_outer: 4,
            sigma1_n0: 2,
            wake_world_budget: 24,
            wake_exact_fiber_cap: 1024,
            escalation_exact_fiber_cap: 4096,
            escalation_baseline_prefix: 128,
            escalation_e3_prefix: 24,
            delta_wake_run: BigRational::new(BigInt::from(1), BigInt::from(20)),
            eps_q: BigRational::new(BigInt::from(1), BigInt::from(20)),
        }
    }
}

/// The declared σ0 field spec: the SAME level-0 modeled mind act's
/// evaluation field runs (`Level0Field::new(n0_frozen)`), materialized
/// through the one field-identity interface. The construction string is
/// the field-library convention's, so equal kinds yield equal
/// `FieldId`s across this module and the sibling probes.
#[must_use]
pub fn sigma0_spec(cfg: &WakingConfig) -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 {
            n0: cfg.act.n0_frozen,
        },
        construction: "level0-modeled-mind-v1 (Solver::modeled_choice; \
                       frozen INNER_SEED belief worlds)"
            .to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// The declared σ1 field spec (`Level1` at the declared inner schedule).
#[must_use]
pub fn sigma1_spec(cfg: &WakingConfig) -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 {
            n_outer: cfg.sigma1_n_outer,
            n0: cfg.sigma1_n0,
        },
        construction: "level1-modeled-mind-v1 (solver::level1_evaluate; \
                       saturation-tie refinement 4x per round capped at 16x; \
                       per-state FIELD_DOMAIN seed)"
            .to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// The waking layer's per-decision scope stem. Every waking
/// [`ScopedDelta`] scope extends this stem, and [`decide`] asserts the
/// caller's run scope does not start with the `wake:` prefix — the
/// disjointness from act's internal scopes is mechanical, not a
/// convention.
#[must_use]
pub fn waking_decision_scope(run_scope: &str, d: u64) -> String {
    format!("wake:{run_scope}:d{d}")
}

// ---------------------------------------------------------------------------
// Wake evidence — a crossing witness, never a construction.
// ---------------------------------------------------------------------------

/// POSITIVE evidence of a decision-level difference: the σ1 detection
/// leg selected the rival over act's chosen tile (exactly on the exact
/// route, δ-settled on the sampled route). Private fields, no public
/// constructor — the only producer is the wake check, and only on a
/// settled selection of the rival (the module-doc compile_fail lock).
/// An unsettled probe can never inhabit this type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WakeEvidence {
    kind: &'static str,
    rival: Domino,
}

impl WakeEvidence {
    /// The mechanical evidence tag
    /// (`exact-sigma1-selects-rival` / `sampled-sigma1-settles-rival`).
    #[must_use]
    pub fn kind(&self) -> &'static str {
        self.kind
    }

    /// The rival the σ1 leg selected.
    #[must_use]
    pub fn rival(&self) -> Domino {
        self.rival
    }
}

/// The wake check's typed verdict. Only [`WakeVerdict::Wake`] carries
/// escalation-driving evidence; both no-wake verdicts route to the σ0
/// baseline choice, recorded as such.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WakeVerdict {
    /// The σ1 leg positively selected the rival: escalate.
    Wake(WakeEvidence),
    /// The detection settled WITHOUT selecting the rival (σ1 agrees with
    /// the baseline, or an exact σ1 tie): play the σ0 choice.
    NoWakeSettled {
        /// The settled kind's mechanical tag.
        kind: &'static str,
    },
    /// Within-budget unsettled: the honest open state. Play the σ0
    /// choice — never fake certainty.
    NoWakeOpen {
        /// Paired worlds consumed at the cap.
        consumed: u64,
    },
}

// ---------------------------------------------------------------------------
// Escalation routing — refusals and open states are recorded fallbacks,
// never picks.
// ---------------------------------------------------------------------------

/// Why an escalated decision fell back to the σ0 baseline choice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FallbackReason {
    /// The pipeline refused (typed; see the carried refusal).
    Refused(TypedRefusal),
    /// δ-tier survivors with no δ-settled σ1 selection — the honest
    /// open state at the declared budget.
    OpenSurvivors {
        /// How many survivors stayed open.
        survivors: usize,
    },
    /// Exact-tier survivors without a strict σ1 argmax (an exact tie is
    /// an ordering question, not a settlement).
    ExactOpen {
        /// How many survivors tied.
        survivors: usize,
    },
}

impl FallbackReason {
    /// The mechanical type tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            FallbackReason::Refused(_) => "fallback-refused",
            FallbackReason::OpenSurvivors { .. } => "fallback-open-survivors",
            FallbackReason::ExactOpen { .. } => "fallback-exact-open",
        }
    }
}

/// A RECORDED fallback to the σ0 baseline choice. Deliberately carries
/// the typed reason and nothing else — no numeric accessor exists (the
/// module-doc compile_fail lock), so a fallback can never be read as an
/// evaluation of anything.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedFallback {
    reason: FallbackReason,
}

impl RecordedFallback {
    /// The typed reason.
    #[must_use]
    pub fn reason(&self) -> &FallbackReason {
        &self.reason
    }
}

/// The escalation's routing verdict on one [`StageFourOutcome`].
pub enum EscalationOutcome {
    /// The evaluation settled a selection; play it.
    Selected {
        /// The settled tile.
        tile: Domino,
        /// Which settlement route selected it.
        via: &'static str,
    },
    /// No settled selection: the σ0 baseline choice plays, recorded.
    Fallback(RecordedFallback),
}

/// THE ESCALATION ROUTING as a pure function of the Stage-4 outcome —
/// every variant maps to exactly one route (no wildcard arm, so a new
/// outcome kind is a compile error here, never a silent fallthrough).
/// Settlements route to play; refusals and open states route to the
/// recorded fallback.
#[must_use]
pub fn route_stage_four(outcome: &StageFourOutcome) -> EscalationOutcome {
    match outcome {
        StageFourOutcome::DeltaSingleton { selected } => EscalationOutcome::Selected {
            tile: *selected,
            via: "delta-singleton",
        },
        StageFourOutcome::DeltaSurvivors {
            selected1: Some(tile),
            ..
        } => EscalationOutcome::Selected {
            tile: *tile,
            via: "delta-selected1",
        },
        StageFourOutcome::DeltaSurvivors {
            sigma1,
            selected1: None,
            ..
        } => EscalationOutcome::Fallback(RecordedFallback {
            reason: FallbackReason::OpenSurvivors {
                survivors: sigma1.actions.len(),
            },
        }),
        StageFourOutcome::ExactSurvivors { evaluation, .. } => {
            if evaluation.survivors.len() == 1 {
                return EscalationOutcome::Selected {
                    tile: evaluation.survivors[0],
                    via: "exact-singleton",
                };
            }
            let Some(values1) = evaluation.values1.as_ref() else {
                return EscalationOutcome::Fallback(RecordedFallback {
                    reason: FallbackReason::ExactOpen {
                        survivors: evaluation.survivors.len(),
                    },
                });
            };
            // A settled exact selection is a STRICT argmax; an exact tie
            // is an honest open state at this tier, never index-broken
            // here.
            let best = values1.values.iter().max().expect("a survivor has a value");
            let winners: Vec<Domino> = values1
                .actions
                .iter()
                .zip(&values1.values)
                .filter(|(_, v)| *v == best)
                .map(|(a, _)| *a)
                .collect();
            if winners.len() == 1 {
                EscalationOutcome::Selected {
                    tile: winners[0],
                    via: "exact-argmax",
                }
            } else {
                EscalationOutcome::Fallback(RecordedFallback {
                    reason: FallbackReason::ExactOpen {
                        survivors: evaluation.survivors.len(),
                    },
                })
            }
        }
        StageFourOutcome::NotRun(refusal) => EscalationOutcome::Fallback(RecordedFallback {
            reason: FallbackReason::Refused(refusal.clone()),
        }),
    }
}

// ---------------------------------------------------------------------------
// The census record.
// ---------------------------------------------------------------------------

/// Which path the decision took.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakingPath {
    /// One legal tile; nothing ran.
    Forced,
    /// The wake check settled without selecting the rival.
    NoWakeSettled,
    /// The wake check stayed open within its budget (the honest open
    /// state — the σ0 choice plays).
    NoWakeOpen,
    /// Positive evidence; the escalation ran.
    Wake,
}

impl WakingPath {
    /// The mechanical path tag.
    #[must_use]
    pub fn tag(self) -> &'static str {
        match self {
            WakingPath::Forced => "forced",
            WakingPath::NoWakeSettled => "no-wake-settled",
            WakingPath::NoWakeOpen => "no-wake-budget-exhausted",
            WakingPath::Wake => "wake",
        }
    }

    fn from_tag(tag: &str) -> Option<WakingPath> {
        match tag {
            "forced" => Some(WakingPath::Forced),
            "no-wake-settled" => Some(WakingPath::NoWakeSettled),
            "no-wake-budget-exhausted" => Some(WakingPath::NoWakeOpen),
            "wake" => Some(WakingPath::Wake),
            _ => None,
        }
    }
}

/// The escalation's census slice: the Stage-4 outcome's variant tag,
/// the ladder stop, the routing verdict, the settled selection where
/// one existed, and every typed refusal's serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EscalationCensus {
    /// `StageFourOutcome` variant tag: `exact-survivors` /
    /// `delta-singleton` / `delta-survivors` / `not-run`.
    pub outcome: String,
    /// The [`crate::solver::targeted::EscalationStop`] tag.
    pub stop: String,
    /// The routing verdict tag: the `via` of a settled selection, or
    /// the fallback reason tag.
    pub via: String,
    /// The settled selection, if the routing selected one.
    pub selected: Option<Domino>,
    /// The controller's §12.1 per-phase spend vector, verbatim from the
    /// [`crate::solver::targeted::TargetedRootReport`]:
    /// `(phase, integer microseconds, items)` per pipeline phase that
    /// ran. This is the targeting data for where escalation microseconds
    /// actually go.
    pub spend: Vec<(String, u64, u64)>,
    /// Typed refusals, serialized.
    pub refusals: Vec<String>,
}

/// One decision's census record — the affordability numbers the
/// CPU-vs-GPU fork turns on. All spend integers are microseconds; any
/// ratio derived from this record is an exact rational downstream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WakingCensus {
    /// The decision ordinal within the run (plies played + 1).
    pub d: u64,
    /// Trick number, 1-based.
    pub trick: u64,
    /// The viewer's internal seat index.
    pub seat: u8,
    /// The declaration's arena id.
    pub decl: u8,
    /// Exact `|Φ(C)|` at the decision root.
    pub fiber: u128,
    /// Legal tile count.
    pub legal: u8,
    /// The path taken.
    pub path: WakingPath,
    /// The wake check's evidence tag (`None` exactly on a forced play).
    pub wake_kind: Option<String>,
    /// The strongest rival the detection paired against (`None` exactly
    /// on a forced play).
    pub rival: Option<Domino>,
    /// The escalation slice (`Some` exactly on the wake path).
    pub escalation: Option<EscalationCensus>,
    /// act's σ0 baseline choice.
    pub sigma0: Domino,
    /// act's route label for the baseline choice.
    pub sigma0_route: String,
    /// The tile the seat played.
    pub played: Domino,
    /// `played == sigma0`.
    pub agreed: bool,
    /// Baseline phase spend, integer microseconds.
    pub baseline_us: u64,
    /// Wake-check phase spend, integer microseconds.
    pub wake_us: u64,
    /// Escalation phase spend, integer microseconds (zero off the wake
    /// path).
    pub escalation_us: u64,
    /// Worlds the wake check consumed (paired worlds on the sampled
    /// route, the enumerated fiber on the exact route, zero on forced).
    pub wake_worlds: u64,
}

fn opt_tile_field(d: Option<Domino>) -> String {
    d.map_or("null".to_string(), |t| t.index().to_string())
}

impl WakingCensus {
    /// One JSONL line. Tiles serialize as indices (the bridge-log
    /// convention); `fiber` as a quoted decimal (u128); `ctx` names the
    /// producing surface (a run scope or a hand label) and is not part
    /// of the record's identity.
    #[must_use]
    pub fn to_jsonl(&self, ctx: &str) -> String {
        let escalation = self.escalation.as_ref().map_or("null".to_string(), |e| {
            let refusals: Vec<String> = e.refusals.iter().map(|r| format!("\"{r}\"")).collect();
            let spend: Vec<String> = e
                .spend
                .iter()
                .map(|(phase, micros, items)| {
                    format!("{{\"phase\":\"{phase}\",\"micros\":{micros},\"items\":{items}}}")
                })
                .collect();
            format!(
                "{{\"outcome\":\"{}\",\"stop\":\"{}\",\"via\":\"{}\",\"selected\":{},\
                 \"spend\":[{}],\"refusals\":[{}]}}",
                e.outcome,
                e.stop,
                e.via,
                opt_tile_field(e.selected),
                spend.join(","),
                refusals.join(",")
            )
        });
        format!(
            "{{\"ctx\":\"{ctx}\",\"d\":{},\"trick\":{},\"seat\":{},\"decl\":{},\
             \"fiber\":\"{}\",\"legal\":{},\"path\":\"{}\",\"wake_kind\":{},\
             \"rival\":{},\"sigma0\":{},\"sigma0_route\":\"{}\",\"played\":{},\
             \"agreed\":{},\"baseline_us\":{},\"wake_us\":{},\"escalation_us\":{},\
             \"wake_worlds\":{},\"escalation\":{escalation}}}",
            self.d,
            self.trick,
            self.seat,
            self.decl,
            self.fiber,
            self.legal,
            self.path.tag(),
            self.wake_kind
                .as_ref()
                .map_or("null".to_string(), |k| format!("\"{k}\"")),
            opt_tile_field(self.rival),
            self.sigma0.index(),
            self.sigma0_route,
            self.played.index(),
            self.agreed,
            self.baseline_us,
            self.wake_us,
            self.escalation_us,
            self.wake_worlds,
        )
    }

    /// Parse one line produced by [`WakingCensus::to_jsonl`] — the
    /// round-trip half of the census format contract. Not a general
    /// JSON parser: it reads exactly this record shape.
    #[must_use]
    pub fn parse_jsonl(line: &str) -> Option<WakingCensus> {
        let raw = |key: &str| -> Option<&str> {
            let pat = format!("\"{key}\":");
            let start = line.find(&pat)? + pat.len();
            let rest = &line[start..];
            let mut depth = 0usize;
            let mut in_str = false;
            for (i, c) in rest.char_indices() {
                match c {
                    '"' => in_str = !in_str,
                    '{' | '[' if !in_str => depth += 1,
                    '}' | ']' if !in_str => {
                        if depth == 0 {
                            return Some(&rest[..i]);
                        }
                        depth -= 1;
                    }
                    ',' if !in_str && depth == 0 => return Some(&rest[..i]),
                    _ => {}
                }
            }
            None
        };
        let string = |key: &str| -> Option<String> {
            let v = raw(key)?;
            Some(v.strip_prefix('"')?.strip_suffix('"')?.to_string())
        };
        let opt_string = |key: &str| -> Option<Option<String>> {
            let v = raw(key)?;
            if v == "null" {
                return Some(None);
            }
            Some(Some(v.strip_prefix('"')?.strip_suffix('"')?.to_string()))
        };
        let tile = |v: &str| -> Option<Domino> { Domino::from_index(v.parse().ok()?) };
        let opt_tile = |key: &str| -> Option<Option<Domino>> {
            let v = raw(key)?;
            if v == "null" {
                return Some(None);
            }
            Some(Some(tile(v)?))
        };
        let escalation = match raw("escalation")? {
            "null" => None,
            body => {
                let sub = |key: &str| -> Option<String> {
                    let pat = format!("\"{key}\":\"");
                    let start = body.find(&pat)? + pat.len();
                    let end = body[start..].find('"')? + start;
                    Some(body[start..end].to_string())
                };
                let selected = {
                    let pat = "\"selected\":";
                    let start = body.find(pat)? + pat.len();
                    let end = body[start..].find([',', '}']).map(|k| k + start)?;
                    match &body[start..end] {
                        "null" => None,
                        v => Some(tile(v)?),
                    }
                };
                let spend = {
                    let pat = "\"spend\":[";
                    let start = body.find(pat)? + pat.len();
                    let end = body[start..].find(']')? + start;
                    let inner = &body[start..end];
                    let mut entries: Vec<(String, u64, u64)> = Vec::new();
                    for entry in inner.split("},{") {
                        let entry = entry.trim_matches(['{', '}']);
                        if entry.is_empty() {
                            continue;
                        }
                        let phase = {
                            let pat = "\"phase\":\"";
                            let s = entry.find(pat)? + pat.len();
                            let e = entry[s..].find('"')? + s;
                            entry[s..e].to_string()
                        };
                        let number = |key: &str| -> Option<u64> {
                            let pat = format!("\"{key}\":");
                            let s = entry.find(&pat)? + pat.len();
                            let e = entry[s..]
                                .find(|c: char| !c.is_ascii_digit())
                                .map_or(entry.len(), |k| k + s);
                            entry[s..e].parse().ok()
                        };
                        entries.push((phase, number("micros")?, number("items")?));
                    }
                    entries
                };
                let refusals = {
                    let pat = "\"refusals\":[";
                    let start = body.find(pat)? + pat.len();
                    let end = body[start..].find(']')? + start;
                    let inner = &body[start..end];
                    if inner.is_empty() {
                        Vec::new()
                    } else {
                        inner
                            .split("\",\"")
                            .map(|s| s.trim_matches('"').to_string())
                            .collect()
                    }
                };
                Some(EscalationCensus {
                    outcome: sub("outcome")?,
                    stop: sub("stop")?,
                    via: sub("via")?,
                    selected,
                    spend,
                    refusals,
                })
            }
        };
        Some(WakingCensus {
            d: raw("d")?.parse().ok()?,
            trick: raw("trick")?.parse().ok()?,
            seat: raw("seat")?.parse().ok()?,
            decl: raw("decl")?.parse().ok()?,
            fiber: string("fiber")?.parse().ok()?,
            legal: raw("legal")?.parse().ok()?,
            path: WakingPath::from_tag(&string("path")?)?,
            wake_kind: opt_string("wake_kind")?,
            rival: opt_tile("rival")?,
            escalation,
            sigma0: tile(raw("sigma0")?)?,
            sigma0_route: string("sigma0_route")?,
            played: tile(raw("played")?)?,
            agreed: raw("agreed")?.parse().ok()?,
            baseline_us: raw("baseline_us")?.parse().ok()?,
            wake_us: raw("wake_us")?.parse().ok()?,
            escalation_us: raw("escalation_us")?.parse().ok()?,
            wake_worlds: raw("wake_worlds")?.parse().ok()?,
        })
    }
}

// ---------------------------------------------------------------------------
// The strongest rival — act's own σ0 evaluation ordering, declared in the
// module docs.
// ---------------------------------------------------------------------------

/// Derive the strongest rival to act's chosen tile from the baseline's
/// own evaluation (the pairing strategy declared in the module docs).
/// `None` exactly when the play was forced.
#[must_use]
pub fn strongest_rival(decision: &ActDecision, viewer_team: Team) -> Option<Domino> {
    let evaluation = decision.evaluation.as_ref()?;
    if decision.legal.len() < 2 {
        return None;
    }
    let chosen = decision.tile;
    if let SetResult::ExactFrozenSet { wins, .. } = &evaluation.result {
        // Exact ordering over the whole candidate set: the non-chosen
        // action with the most exact wins, ties toward the lowest
        // candidate index.
        let rival = decision
            .legal
            .iter()
            .zip(wins)
            .filter(|(t, _)| **t != chosen)
            .max_by(|(_, wa), (_, wb)| wa.cmp(wb))
            .map(|(t, _)| *t);
        return rival;
    }
    if let Some(opts) = decision.fallback_opts.as_ref() {
        // A recorded level-1 ranking over the fallback set: the best
        // non-chosen member under the live ordering.
        let rivals: Vec<(u8, BigRational)> = opts
            .iter()
            .filter(|(t, _)| usize::from(*t) != chosen.index())
            .cloned()
            .collect();
        if !rivals.is_empty() {
            let best = best_of(&rivals, viewer_team == Team::T1);
            return Some(Domino::from_index(usize::from(best)).expect("tile < 28"));
        }
    }
    // δ-settled winner or a singleton survivor: the LAST candidate the
    // controller eliminated is the last rival standing under act's own
    // evidence process.
    evaluation
        .eliminations
        .last()
        .map(|e| decision.legal[e.candidate])
}

// ---------------------------------------------------------------------------
// The wake check.
// ---------------------------------------------------------------------------

fn detection_plan(
    scope_stem: &str,
    delta_decision_share: &BigRational,
    eps_q: &BigRational,
) -> DetectionRiskPlan {
    let q = |n: i64, m: i64| BigRational::new(BigInt::from(n), BigInt::from(m));
    DetectionRiskPlan {
        eps_q: eps_q.clone(),
        delta_decision: ScopedDelta::new(
            format!("{scope_stem}:pair-decision"),
            delta_decision_share.clone(),
        ),
        delta_value: ScopedDelta::new(
            format!("{scope_stem}:value-direction"),
            delta_decision_share.clone(),
        ),
        delta_response: ScopedDelta::new(
            format!("{scope_stem}:response"),
            delta_decision_share.clone(),
        ),
        delta_practical_zero: ScopedDelta::new(
            format!("{scope_stem}:practical-zero-q0"),
            delta_decision_share.clone(),
        ),
        // The declared betting mixture of the sibling wakeup probes.
        mixture: vec![
            (q(1, 4), q(1, 8)),
            (q(1, 4), q(1, 4)),
            (q(1, 4), q(1, 2)),
            (q(1, 4), q(3, 4)),
        ],
    }
}

/// The wake check against caller-held field models (the seat holds ONE
/// σ1 model per hand; [`wake_check_once`] wraps this for instrument use).
#[allow(clippy::too_many_arguments)]
fn wake_check_with_fields(
    root: &CanonicalRoot,
    position: &RootPosition,
    chosen: Domino,
    rival: Domino,
    policy_a: &FrozenPolicy,
    policy_b: &FrozenPolicy,
    field0: &FieldModel,
    field1: &FieldModel,
    cfg: &WakingConfig,
    run_scope: &str,
    d: u64,
) -> (WakeVerdict, u64) {
    let fiber = root.count();
    if fiber <= cfg.wake_exact_fiber_cap {
        // The exact route: complete enumeration, exact selections, no
        // sampling risk.
        let exposure_a = frozen_policy_exposure(
            root,
            position,
            policy_a,
            field0,
            field1,
            WorldDomain::ExactFiber,
        );
        let exposure_b = frozen_policy_exposure(
            root,
            position,
            policy_b,
            field0,
            field1,
            WorldDomain::ExactFiber,
        );
        let detection = exact_paired_detection(&exposure_a, &exposure_b, chosen, rival, &cfg.eps_q);
        let DecisionWakeUp::Exact { winner1, .. } = detection.decision else {
            unreachable!("the exact route produces the exact decision record");
        };
        let verdict = match winner1 {
            ExactPairSelection::B => WakeVerdict::Wake(WakeEvidence {
                kind: "exact-sigma1-selects-rival",
                rival,
            }),
            ExactPairSelection::A => WakeVerdict::NoWakeSettled {
                kind: "exact-sigma1-selects-baseline",
            },
            ExactPairSelection::ExactTie => WakeVerdict::NoWakeSettled {
                kind: "exact-sigma1-tie",
            },
        };
        return (verdict, detection.fiber);
    }
    // The sampled route: the dig-until-settled paired detection at the
    // declared world budget. Each engine's declared risk is δ_d/12 (the
    // pair decision spends twice, once per field), keeping the detection
    // half of the decision budget: 5 × δ_d/12 ≤ δ_d/2.
    let delta_d = decision_delta(d, &cfg.delta_wake_run);
    let share = &delta_d / BigRational::from_integer(BigInt::from(12));
    let scope_stem = waking_decision_scope(run_scope, d);
    let plan = detection_plan(&scope_stem, &share, &cfg.eps_q);
    let spec = SampledDetectionSpec {
        root,
        position,
        tile_a: chosen,
        tile_b: rival,
        policy_a,
        policy_b,
        field0,
        field1,
        epoch: WAKE_EPOCH,
        world_cap: cfg.wake_world_budget,
        plan: &plan,
    };
    let detection = sampled_paired_detection(&spec);
    let DecisionWakeUp::Sampled(kind) = &detection.decision else {
        unreachable!("the sampled route produces the sampled decision record");
    };
    let settle1_winner = match kind {
        SampledDecisionKind::Changed { settle1, .. }
        | SampledDecisionKind::NewlySettled { settle1 }
        | SampledDecisionKind::SameWinner { settle1, .. } => Some(settle1.winner()),
        SampledDecisionKind::NewlyOpen { .. } | SampledDecisionKind::BothOpen => None,
    };
    let verdict = match settle1_winner {
        Some(PairWinner::B) => WakeVerdict::Wake(WakeEvidence {
            kind: "sampled-sigma1-settles-rival",
            rival,
        }),
        Some(PairWinner::A) => WakeVerdict::NoWakeSettled {
            kind: "sampled-sigma1-settles-baseline",
        },
        None => WakeVerdict::NoWakeOpen {
            consumed: detection.consumed,
        },
    };
    (verdict, detection.consumed)
}

// ---------------------------------------------------------------------------
// The waking seat.
// ---------------------------------------------------------------------------

/// One decided play: the tile and the census record behind it.
pub struct WakingDecision {
    /// The tile the seat plays (always legal).
    pub tile: Domino,
    /// The decision's census record.
    pub census: WakingCensus,
}

/// The waking seat: act's σ0 baseline, the hard-budgeted wake check, and
/// the wake-gated escalation, with ONE (σ0, σ1) field-model pair held
/// for the whole hand (the insert-only field action caches are the σ1
/// cost amortization).
pub struct WakingSeat {
    cfg: WakingConfig,
    field0: FieldModel,
    field1: FieldModel,
}

impl WakingSeat {
    /// A seat for one hand under the declared configuration.
    #[must_use]
    pub fn new(cfg: WakingConfig) -> WakingSeat {
        let field0 = FieldModel::new(sigma0_spec(&cfg));
        let field1 = FieldModel::new(sigma1_spec(&cfg));
        WakingSeat {
            cfg,
            field0,
            field1,
        }
    }

    /// The declared configuration.
    #[must_use]
    pub fn config(&self) -> &WakingConfig {
        &self.cfg
    }

    /// σ1 information states materialized so far (cache growth is the
    /// amortization instrument).
    #[must_use]
    pub fn sigma1_cache_len(&self) -> usize {
        self.field1.cache_len()
    }

    /// Decide one play. `run_scope` follows act's convention (one scope
    /// per hand; must NOT carry the waking layer's `wake:` prefix — the
    /// scope disjointness is asserted, not assumed); `d` is the decision
    /// ordinal (plies played + 1).
    #[must_use]
    pub fn decide(&self, state: &DrivenState<'_>, run_scope: &str, d: u64) -> WakingDecision {
        assert!(
            !run_scope.starts_with("wake:"),
            "the caller's run scope must stay disjoint from the waking layer's wake: scopes"
        );
        let viewer = state.leader.plus(state.trick_plays.len());
        let trick = u64::try_from(state.prior_played.len() / 4).expect("tricks fit u64") + 1;

        let timer = PhaseTimer::start();
        let baseline = act(
            state,
            &self.cfg.act,
            run_scope,
            d,
            &crate::solver::act::delta_run_default(),
        );
        let baseline_us = timer.micros();

        let (root, position) = driven_root(state).expect("a driven decision has a lawful kernel");
        let fiber = root.count();
        let mut census = WakingCensus {
            d,
            trick,
            seat: u8::try_from(viewer.index()).expect("seat < 4"),
            decl: u8::try_from(arena_decl_id(state.decl)).expect("decl id fits u8"),
            fiber,
            legal: u8::try_from(baseline.legal.len()).expect("legal <= 7"),
            path: WakingPath::Forced,
            wake_kind: None,
            rival: None,
            escalation: None,
            sigma0: baseline.tile,
            sigma0_route: baseline.route.label().to_string(),
            played: baseline.tile,
            agreed: true,
            baseline_us,
            wake_us: 0,
            escalation_us: 0,
            wake_worlds: 0,
        };

        if baseline.route == ActRoute::Forced || baseline.legal.len() < 2 {
            return WakingDecision {
                tile: baseline.tile,
                census,
            };
        }

        // The wake check, against the hand-held field pair.
        let rival = strongest_rival(&baseline, viewer.team())
            .expect("a non-forced decision names a strongest rival");
        let tuple = |pinned: Domino| {
            FrozenPolicy::new(continuation_tuple(
                state.decl,
                state.bid,
                state.declaring_team,
                self.cfg.act.n_outer_frozen,
                self.cfg.act.n0_frozen,
                pinned,
            ))
        };
        let policy_a = tuple(baseline.tile);
        let policy_b = tuple(rival);
        let timer = PhaseTimer::start();
        let (verdict, wake_worlds) = wake_check_with_fields(
            &root,
            &position,
            baseline.tile,
            rival,
            &policy_a,
            &policy_b,
            &self.field0,
            &self.field1,
            &self.cfg,
            run_scope,
            d,
        );
        census.wake_us = timer.micros();
        census.wake_worlds = wake_worlds;
        census.rival = Some(rival);

        match verdict {
            WakeVerdict::Wake(evidence) => {
                census.path = WakingPath::Wake;
                census.wake_kind = Some(evidence.kind().to_string());
                self.escalate(&root, &position, &baseline, &evidence, run_scope, d, census)
            }
            WakeVerdict::NoWakeSettled { kind } => {
                census.path = WakingPath::NoWakeSettled;
                census.wake_kind = Some(kind.to_string());
                WakingDecision {
                    tile: baseline.tile,
                    census,
                }
            }
            WakeVerdict::NoWakeOpen { consumed } => {
                census.path = WakingPath::NoWakeOpen;
                census.wake_kind = Some("sampled-open".to_string());
                census.wake_worlds = consumed;
                WakingDecision {
                    tile: baseline.tile,
                    census,
                }
            }
        }
    }

    /// The wake-gated escalation. Entry REQUIRES a [`WakeEvidence`] —
    /// unforgeable outside the wake check's settled-rival crossing, so
    /// no path exists from an unsettled probe to escalated play.
    #[allow(clippy::too_many_arguments)]
    fn escalate(
        &self,
        root: &CanonicalRoot,
        position: &RootPosition,
        baseline: &ActDecision,
        evidence: &WakeEvidence,
        run_scope: &str,
        d: u64,
        mut census: WakingCensus,
    ) -> WakingDecision {
        assert!(
            baseline.legal.contains(&evidence.rival()),
            "the wake evidence names a legal rival"
        );
        // One frozen focal candidate per legal root action, legal-set
        // order (O38), through the targeted controller.
        let timer = PhaseTimer::start();
        let policies: Vec<FrozenPolicy> = baseline
            .legal
            .iter()
            .map(|t| {
                FrozenPolicy::new(continuation_tuple(
                    position.decl,
                    position.bid,
                    position.declaring_team,
                    self.cfg.act.n_outer_frozen,
                    self.cfg.act.n0_frozen,
                    *t,
                ))
            })
            .collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> = baseline
            .legal
            .iter()
            .copied()
            .zip(policies.iter())
            .collect();
        let report = targeted_root(
            root,
            position,
            &candidates,
            &self.field0,
            &self.field1,
            &self.escalation_config(run_scope, d, baseline.legal.len()),
        );
        let routed = route_stage_four(&report.stage4);
        census.escalation_us = timer.micros();

        let outcome_tag = match &report.stage4 {
            StageFourOutcome::ExactSurvivors { .. } => "exact-survivors",
            StageFourOutcome::DeltaSingleton { .. } => "delta-singleton",
            StageFourOutcome::DeltaSurvivors { .. } => "delta-survivors",
            StageFourOutcome::NotRun(_) => "not-run",
        };
        let (via, selected) = match &routed {
            EscalationOutcome::Selected { tile, via } => (*via, Some(*tile)),
            EscalationOutcome::Fallback(fallback) => (fallback.reason().tag(), None),
        };
        census.escalation = Some(EscalationCensus {
            outcome: outcome_tag.to_string(),
            stop: report.stop.tag().to_string(),
            via: via.to_string(),
            selected,
            spend: report
                .spend
                .iter()
                .map(|p| (p.phase.to_string(), p.micros, p.items))
                .collect(),
            refusals: report.refusals.iter().map(ToString::to_string).collect(),
        });

        let tile = match routed {
            EscalationOutcome::Selected { tile, .. } => {
                assert!(
                    baseline.legal.contains(&tile),
                    "an escalated selection is a legal root action"
                );
                tile
            }
            EscalationOutcome::Fallback(_) => baseline.tile,
        };
        census.played = tile;
        census.agreed = tile == baseline.tile;
        WakingDecision { tile, census }
    }

    /// The escalation's declared configuration for one decision: the
    /// escalation half of the decision budget as the screen budget,
    /// per-action risks at `δ_esc/(8m)` (ledger-safe at any legal count),
    /// the wake epoch's stream, and the `wake:`-prefixed scope.
    fn escalation_config(&self, run_scope: &str, d: u64, m: usize) -> TargetedConfig {
        let delta_d = decision_delta(d, &self.cfg.delta_wake_run);
        let delta_esc = &delta_d / BigRational::from_integer(BigInt::from(2));
        let per_entry = &delta_esc / BigRational::from_integer(BigInt::from(8) * BigInt::from(m));
        let scope = waking_decision_scope(run_scope, d);
        TargetedConfig {
            budget: RungBudget {
                exact_fiber_cap: self.cfg.escalation_exact_fiber_cap,
                baseline_prefix: self.cfg.escalation_baseline_prefix,
                e3_prefix: self.cfg.escalation_e3_prefix,
                directional: false,
            },
            risk: Some(TargetedRisk {
                screen_budget: ScopedDelta::new(format!("{scope}:screen"), delta_esc),
                per_baseline_side: per_entry.clone(),
                per_e3: per_entry,
            }),
            epsilon: Some(self.cfg.eps_q.clone()),
            epoch: WAKE_EPOCH,
            scope,
        }
    }
}

/// A one-shot wake check for instrument use (constructs throwaway field
/// models; the seat itself always uses its hand-held pair).
#[must_use]
pub fn wake_check_once(
    root: &CanonicalRoot,
    position: &RootPosition,
    chosen: Domino,
    rival: Domino,
    cfg: &WakingConfig,
    run_scope: &str,
    d: u64,
) -> (WakeVerdict, u64) {
    let tuple = |pinned: Domino| {
        FrozenPolicy::new(continuation_tuple(
            position.decl,
            position.bid,
            position.declaring_team,
            cfg.act.n_outer_frozen,
            cfg.act.n0_frozen,
            pinned,
        ))
    };
    let policy_a = tuple(chosen);
    let policy_b = tuple(rival);
    let field0 = FieldModel::new(sigma0_spec(cfg));
    let field1 = FieldModel::new(sigma1_spec(cfg));
    wake_check_with_fields(
        root, position, chosen, rival, &policy_a, &policy_b, &field0, &field1, cfg, run_scope, d,
    )
}
