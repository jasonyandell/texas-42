//! The unified walt player, slice UP0 — one decision function, every
//! exact instrument, provenance always.
//!
//! WHAT THIS IS. [`UnifiedPlayer::decide`] answers at every legal state
//! of a hand: seat, public record and own hand in, one legal action out,
//! together with a [`Provenance`] that names the instrument that produced
//! it, the field consultations it spent, and every typed refusal it fell
//! through on the way. The player never panics on an instrument result
//! and never silently truncates: the cascade is TOTAL and ends at a raw
//! σ0 consultation that always answers.
//!
//! THE TWO RECURSIONS. Jason's frame, now twice measured (U0's fusion
//! horizon, MB1's trick-4 fusion price): 42 is two recursions running in
//! opposite directions — enumerable exactness walking BACKWARD from
//! terminal, and sampled/structural play walking FORWARD from now. UP0
//! is the first artifact that plays both and knows at every depth which
//! one it is standing in: every [`Provenance`] carries its
//! [`Recursion`], and [`Recursion::direction`] answers the question in
//! one word.
//!
//! THE CASCADE (§34/§35 declared-budget discipline, deepest certainty
//! first; each tier is ENTERED only on affordability and EXITED only by
//! a typed refusal):
//!
//! ```text
//! (a) DecidedArithmetic   the pmake indicator is already settled for
//!                         every continuation, or one legal tile exists
//! (b) EndgameExact        the store already proves this root (consume a
//!                         zero-regret necessary outer profile), else the
//!                         exact world-space recursion where the fiber
//!                         affords enumeration
//! (c) MiddlegameMixture   MB1's exact model-space response under the
//!                         carried posterior and a declared read ceiling
//! (d) CertifiedRegret     the §33 recommendation off whatever facts the
//!                         store already holds, within a declared Γ
//! (e) FieldFallback       the declared σ0 field's own choice — total
//! ```
//!
//! The sequencing is MEASURED, not guessed. U0 censused the fusion
//! horizon at trick 5 (t5/t6 substantively fusion-free, t4 carrying a
//! positive God gap) and MB1 located the affordability wall between
//! trick 4 and trick 3 (t6 microseconds, t5 seconds, t4 minutes, t3 not
//! within seven million field consultations). Tier (b) is therefore the
//! endgame instrument, tier (c) the middlegame one, and past the wall
//! both refuse and the player falls through — which is the honest
//! behaviour, not a failure.
//!
//! PROVENANCE IS UNFORGEABLE BY CONSTRUCTION. There is no `tier` field to
//! lie in: [`Provenance::tier`] is a DERIVED VIEW of [`Evidence`], whose
//! variants each carry exactly what their tier can prove — the
//! enumeration's exact mass pair, the consumed fact's id, the mixture's
//! weighted pair together with the ledger spend, the §33 block, or
//! nothing but the field's own name.
//!
//! Precisely where the fence sits, since the distinction matters: an
//! [`Evidence`] value is a public sum type and a reader must be able to
//! MATCH on it, so its variants are constructible like any Rust enum's.
//! What cannot be constructed outside this module is a [`Provenance`] or
//! a [`Decision`] — both hold private members and neither has a public
//! constructor, so the only decisions in existence are the ones
//! [`UnifiedPlayer::decide`] returned, and each carries the evidence its
//! own cascade produced. A fabricated `Evidence` is a value nobody can
//! attach to a decision; that is the [`crate::solver::model_recursion::CoupledFact`]
//! pattern, applied one level out.
//!
//! THE JOIN (§76). A [`ModelBelief`] is constructed at the seat's first
//! decision and carried down its line: [`ModelBelief::focal_play`] when
//! that seat acts, [`ModelBelief::observe`] when another does. The
//! posterior is CARRIED, never recomputed, and the player stores no
//! derived view of it — every reported marginal is read back off the one
//! carried belief. Where the budget declares a join reading, tier (b)
//! also prices the mixture and records both answers side by side: MB1
//! measured that values move before argmaxes do, so both are recorded
//! and neither is presented as the other.
//!
//! THE LIBRARY CAN BE FALSIFIED, AND SAYS SO. A model belief's support is
//! the actions its declared type library would play. UP0 is not in that
//! library, so when UP0 seats play each other an observation can leave
//! the support entirely. [`ModelBelief::observe`] asserts positive
//! augmented mass, so UP0 checks the merged branch table FIRST and, on a
//! miss, retires the line with a typed [`Falsification`] rather than
//! reaching the assertion. The posterior is not repaired, re-seeded or
//! widened — a falsified library is a finding, and the tier that needed
//! it refuses from then on.
//!
//! WHAT THIS MODULE CONSUMES AND NEVER TOUCHES. `refine.rs` (freeze 58),
//! `doom.rs` (§47) and `godgap.rs` (U0) are consume-only and are not
//! imported here at all — U0's receipts reach the player as ordinary
//! §49 facts in a [`ReceiptStore`] the caller seeds, which is what makes
//! "consumed, not recomputed" checkable.
//!
//! `model_belief.rs` and `model_recursion.rs` are FROZEN consume-only for
//! this slice, together with both their gate files (the brief's
//! amendment, on the doom→U0 and godgap→MB1 precedent: the freeze is what
//! makes gate UP5's inheritance claim checkable rather than asserted).
//! UP0 needed no additive surface on either. It imports exactly five
//! pre-existing public items from `model_belief` — [`BehaviorType`],
//! [`MixtureRefusal`], [`MixtureStats`], [`ModelBelief`],
//! [`SeatTypePrior`] — and calls only `from_independent_prior`,
//! `focal_play`, `observe`, `branch_masses`, `mixture_response_budgeted`,
//! `ledger`, `history`, `seat_to_move` and `profiles`, every one of them
//! shipped by MB0/MB1. It does not import `model_recursion` at all; the
//! reference above is a doc link, and MB1's `trace_heaviest_line` is
//! reached only from the gate file, as the independent authority UP3
//! checks the carried posterior against.
//!
//! The OLD PLAYER (walt_bridge, playout, playtable, webtable,
//! level1_evaluate) is untouched: UP0 is a new artifact beside it, and
//! bridging, defaults and arena evaluation are future slices on Jason's
//! word.
//!
//! NO `expect` ON AN INSTRUMENT RESULT. Every instrument refusal in this
//! module is matched and typed into [`TierRefusal`]. The module holds no
//! `unwrap`, `panic!`, `unreachable!` or `todo!` at all, and every
//! `expect` it does hold is annotated `(rules invariant)` — a property
//! of the rules of 42, never of an instrument's answer. Gate UP1 greps
//! for exactly that.
//!
//! EXPLORATORY tier — below every evidentiary tier, cited by nothing
//! above it. Nothing here is a play-strength claim.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{legal_plays, Decl, Domino, DominoSet, Seat};
use crate::solver::adaptive::{
    decided_success, driven_root, root_identity, CanonicalRoot, DrivenState, PublicRecord,
    RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{
    extract_success_policy, viewer_success_mass, ExactCoverOracle, ExtractionSource, FactorBelief,
    RecursionStats, ResponseStats,
};
use crate::solver::model_belief::{
    BehaviorType, MixtureRefusal, MixtureStats, ModelBelief, SeatTypePrior,
};
use crate::solver::proof_state::{
    BoundFact, Fact, ProofState, ProofTag, Reject, SemanticsIdentity,
};

// ---------------------------------------------------------------------------
// Which recursion, and which tier.
// ---------------------------------------------------------------------------

/// Which recursion produced an answer. The DIRECTION is the load-bearing
/// coordinate — backward from terminal, or forward from now — and the
/// space says which of the two backward recursions it was: the world-space
/// fiber recursion over `Φ(C)`, or the model-space bundle recursion over
/// `Ξ = Ω × Θ`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recursion {
    /// Enumerable exactness walking backward over the world fiber
    /// (tiers a, b and d — the arithmetic, the exact response, and the
    /// facts a previous exact walk deposited).
    BackwardWorld,
    /// Enumerable exactness walking backward over `Ξ = Ω × Θ` — MB1's
    /// bundle recursion under the carried model posterior (tier c).
    BackwardModel,
    /// Sampled/structural play walking forward from now: the declared
    /// field's own consultation (tier e).
    Forward,
}

impl Recursion {
    /// The one word that answers "which of the two recursions is this".
    pub fn direction(&self) -> &'static str {
        match self {
            Recursion::BackwardWorld | Recursion::BackwardModel => "backward",
            Recursion::Forward => "forward",
        }
    }

    /// The space the recursion walks.
    pub fn space(&self) -> &'static str {
        match self {
            Recursion::BackwardWorld => "world",
            Recursion::BackwardModel => "model",
            Recursion::Forward => "play",
        }
    }
}

/// The cascade's five tiers. Never stored on a decision — always derived
/// from the [`Evidence`] the tier actually produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    DecidedArithmetic,
    EndgameExact,
    MiddlegameMixture,
    CertifiedRegret,
    FieldFallback,
}

impl Tier {
    /// The brief's own letter for this tier.
    pub fn letter(&self) -> char {
        match self {
            Tier::DecidedArithmetic => 'a',
            Tier::EndgameExact => 'b',
            Tier::MiddlegameMixture => 'c',
            Tier::CertifiedRegret => 'd',
            Tier::FieldFallback => 'e',
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Tier::DecidedArithmetic => "decided-arithmetic",
            Tier::EndgameExact => "endgame-exact",
            Tier::MiddlegameMixture => "middlegame-mixture",
            Tier::CertifiedRegret => "certified-regret",
            Tier::FieldFallback => "field-fallback",
        }
    }

    /// Every tier, in cascade order — the census axis of the transcript.
    pub const ALL: [Tier; 5] = [
        Tier::DecidedArithmetic,
        Tier::EndgameExact,
        Tier::MiddlegameMixture,
        Tier::CertifiedRegret,
        Tier::FieldFallback,
    ];
}

// ---------------------------------------------------------------------------
// The declared budget.
// ---------------------------------------------------------------------------

/// One move's declared budget. Every quantity is either a STRUCTURAL
/// affordability predicate on the fiber — free to check, checked before
/// any spend — or an ENFORCED ceiling in field consultations, the unit
/// MB1's `ReadLedger` measures at the dispatch itself. Never wall-clock:
/// a wall-clock budget makes a decision a function of the machine.
///
/// The two kinds are not interchangeable and the refusals say which one
/// fired. The world-space recursions of tier (b) take no ceiling — they
/// are full walks over the fiber with no abort point that leaves a
/// meaningful partial value — so their affordability is the declared
/// fiber cap and their spend is MEASURED and reported afterwards. Tier
/// (c) carries MB1's real enforced ceiling and its refusal is MB1's own,
/// with the measured spend inside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MoveBudget {
    /// A declared name for this rung of the ladder; appears verbatim in
    /// every decision's frame so a transcript row names its own budget.
    pub label: String,
    /// Fiber mass at or below which the exact world-space recursion is
    /// attempted (structural, free to check).
    pub enumeration_fiber_cap: u128,
    /// Fiber mass at or below which the exact model-space recursion is
    /// attempted (structural). Separate from the read ceiling because a
    /// coordinate past the wall spends the whole ceiling before refusing
    /// — MB1 measured 31 minutes and 35M reads for five trick-3 refusals
    /// — and a player wants the free pre-check.
    pub mixture_fiber_cap: u128,
    /// Field consultations tier (c) may spend at this move (enforced by
    /// `mixture_response_budgeted`; the refusal carries the measurement).
    pub mixture_read_cap: u64,
    /// The largest certified regret `Γ = U* − B_exec` tier (d) will
    /// recommend under. Exact rational; `0` accepts only certified
    /// optimality.
    pub regret_acceptance: BigRational,
    /// Spend the model-space walk for the RECORD even at a state tier (b)
    /// already answered, so the join reading exists. Off in a lean
    /// player; on in the transcript's ample rung, where measuring
    /// whether the posterior would have moved the argmax is the point.
    pub join_reading: bool,
}

impl MoveBudget {
    /// A budget that affords nothing: every tier past the free
    /// arithmetic refuses and the σ0 fallback answers, named as such.
    pub fn starved(label: &str) -> MoveBudget {
        MoveBudget {
            label: label.to_string(),
            enumeration_fiber_cap: 0,
            mixture_fiber_cap: 0,
            mixture_read_cap: 0,
            regret_acceptance: BigRational::new(BigInt::from(0), BigInt::from(1)),
            join_reading: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Typed refusals.
// ---------------------------------------------------------------------------

/// Every way a tier declines to answer. A refusal is never a value and
/// never a downgrade: it names what was not affordable, or what the
/// store did not hold, and the cascade moves on with the refusal
/// recorded. No variant carries a partial value, so a truncated number
/// cannot be reported even by accident (MB1's `MixtureRefusal`
/// discipline, inherited).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TierRefusal {
    /// (b), (d) — the §49 proof state is a trick-START object (its
    /// constructor asserts an empty partial trick), so at a mid-trick
    /// decision there is no store to consume from or install into. The
    /// exact recursion of (b) still runs; only the store paths refuse.
    /// The boundary is the spike's, not this slice's, and lifting it is
    /// UP1 work.
    ProofStateUnavailable { plays_in_trick: usize },
    /// (b) — a proof state exists but holds no executable witness at all.
    NoExecutableWitness { facts: usize },
    /// (b) — a proof state holds an executable witness whose certified
    /// regret is not zero, so it does not prove this root and (b) will
    /// not consume it. Tier (d) may still accept it under its own
    /// declared acceptance.
    ReceiptNotCertified { regret: BigRational },
    /// (b) — the fiber exceeds the declared enumeration cap.
    EnumerationUnaffordable { fiber: u128, cap: u128 },
    /// (b) — the re-priced value of the extracted policy did not equal
    /// the optimum it was extracted from. Structurally impossible under
    /// §63; typed rather than asserted so the player degrades instead of
    /// dying if it ever fires.
    RepricingDisagreed { extracted: u128, repriced: u128 },
    /// (c) — the seat's carried model belief was retired: an observed
    /// action left the declared type library's support.
    PosteriorFalsified(Box<Falsification>),
    /// (c) — the fiber exceeds the declared mixture cap.
    MixtureUnaffordable { fiber: u128, cap: u128 },
    /// (c) — MB1's own typed refusal, carried verbatim: the measured
    /// spend, the declared ceiling, and the public history it stopped at.
    MixtureRefused { refusal: String },
    /// (c) — the mixture walk returned no recorded choice at this
    /// information state (an argmax table that does not name the state it
    /// was extracted at is not an answer).
    MixtureNoChoice { policy_id: String },
    /// (d) — the store's certified regret exceeds the declared
    /// acceptance.
    RegretAboveAcceptance {
        regret: BigRational,
        acceptance: BigRational,
    },
    /// (d) — no executable bar in the store, so there is no policy to
    /// recommend (§33: at zero executable work nothing is recommended).
    NoRecommendation { facts: usize },
    /// (b), (c), (d) — the canonical root could not be built from the
    /// public state, so no fiber instrument applies. The fallback still
    /// answers; it needs no kernel.
    RootUnavailable { reason: String },
    /// (c) — the seat to move is not the carried line's own viewer, so
    /// this belief is not about this decision.
    LineNotFocal { viewer: usize, to_move: usize },
}

/// An observation that left the declared type library's support: the
/// library predicted a set of actions and the seat played outside it.
/// The posterior is retired here rather than repaired — a widened or
/// re-seeded belief would be a different object presented as the same
/// one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Falsification {
    /// The post-root public history at the moment of the observation.
    pub history: Vec<Domino>,
    /// The acting seat, by index.
    pub seat: usize,
    /// What it played.
    pub observed: Domino,
    /// The merged public branch table the library did support, in tile
    /// order — what the library said the seat could do.
    pub supported: Vec<Domino>,
}

// ---------------------------------------------------------------------------
// Evidence — the tier's own proof, and the only place a tier is named.
// ---------------------------------------------------------------------------

/// A necessary outer profile the store already held, consumed rather
/// than recomputed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumedReceipt {
    /// The witnessing authority (§33's recommended policy).
    pub policy: String,
    /// `B_exec` — the executable floor the store proves.
    pub value: BigRational,
    /// The action's own upper after closure. Equal to `value` exactly
    /// when the store proves the action (which tier (b) requires).
    pub upper: BigRational,
    /// The witnessing fact's id in the store.
    pub fact_id: u128,
    /// How many facts the store held when it was consumed.
    pub facts: usize,
}

/// What a tier actually produced. Each variant carries exactly what its
/// tier can prove and nothing it cannot, and the variant DETERMINES the
/// tier — there is no tier field anywhere, so a decision cannot claim
/// work it did not do. The type has no public constructor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Evidence {
    /// (a) The viewer-objective pmake indicator is settled for every
    /// continuation, or exactly one tile is legal. Free: the arithmetic
    /// of banked points against the contract, plus the rules.
    Decided {
        /// `Some(made)` when [`decided_success`] settled the indicator;
        /// `None` when the tier fired on a forced play instead.
        settled: Option<bool>,
        banked: [u32; 2],
        contract: u32,
        legal_actions: usize,
    },
    /// (b) The exact world-space recursion ran HERE: the optimum as an
    /// exact mass pair over the fiber, and the extracted policy re-priced
    /// to it by the independent fixed-policy evaluator (§63's re-pricing
    /// law — the number is a receipt, not a restatement).
    Enumerated {
        optimum_mass: u128,
        repriced_mass: u128,
        fiber_mass: u128,
        policy_id: String,
        policy_states: usize,
    },
    /// (b) The store already proved this root at zero certified regret.
    /// No walk ran; the spend is zero and the fact is named.
    Consumed(Box<ConsumedReceipt>),
    /// (c) MB1's exact model-space response under the carried posterior:
    /// `Q(ν)` as the exact pair `weighted_mass / weighted_total`, the
    /// extracted mixture argmax policy, and the LEDGER's measured spend.
    Mixture {
        weighted_mass: u128,
        weighted_total: u128,
        policy_id: String,
        live_profiles: usize,
        /// Field consultations this walk spent, measured at the dispatch.
        reads: u64,
    },
    /// (d) The §33 recommendation block off the store's existing facts.
    CertifiedRegret {
        policy: String,
        pmake_lower: BigRational,
        global_upper: BigRational,
        certified_regret: BigRational,
        facts: usize,
    },
    /// (e) The declared field's own choice. Nothing is claimed but the
    /// field's name — which is the honest content of this tier.
    Field { field_id: String },
}

impl Evidence {
    /// The tier this evidence IS. Derived, never stored (the house rule:
    /// derived views, never a second authority).
    pub fn tier(&self) -> Tier {
        match self {
            Evidence::Decided { .. } => Tier::DecidedArithmetic,
            Evidence::Enumerated { .. } | Evidence::Consumed(_) => Tier::EndgameExact,
            Evidence::Mixture { .. } => Tier::MiddlegameMixture,
            Evidence::CertifiedRegret { .. } => Tier::CertifiedRegret,
            Evidence::Field { .. } => Tier::FieldFallback,
        }
    }

    /// The recursion this tier stands in.
    pub fn recursion(&self) -> Recursion {
        match self {
            Evidence::Decided { .. }
            | Evidence::Enumerated { .. }
            | Evidence::Consumed(_)
            | Evidence::CertifiedRegret { .. } => Recursion::BackwardWorld,
            Evidence::Mixture { .. } => Recursion::BackwardModel,
            Evidence::Field { .. } => Recursion::Forward,
        }
    }

    /// The exact value this tier claims for the action it chose, where it
    /// claims one. Tier (a) claims a settled indicator rather than a
    /// value, and tier (e) claims nothing at all — both answer `None`,
    /// which is the honest content of those tiers.
    pub fn value(&self) -> Option<BigRational> {
        match self {
            Evidence::Enumerated {
                repriced_mass,
                fiber_mass,
                ..
            } => Some(BigRational::new(
                BigInt::from(*repriced_mass),
                BigInt::from(*fiber_mass),
            )),
            Evidence::Consumed(c) => Some(c.value.clone()),
            Evidence::Mixture {
                weighted_mass,
                weighted_total,
                ..
            } => Some(BigRational::new(
                BigInt::from(*weighted_mass),
                BigInt::from(*weighted_total),
            )),
            Evidence::CertifiedRegret { pmake_lower, .. } => Some(pmake_lower.clone()),
            Evidence::Decided { .. } | Evidence::Field { .. } => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Spend, the posterior note, the frame.
// ---------------------------------------------------------------------------

/// Field consultations spent at one move, by the tier that spent them.
/// One unit throughout: a consultation of a modeled mind at an
/// information state, counted at the dispatch itself. Every number is a
/// MEASUREMENT — never a ceiling, never a ceiling rounded down.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Spend {
    /// Consultations the world-space recursions of tier (b) made,
    /// measured by the counting decorator around the declared field.
    pub enumeration_reads: u64,
    /// Consultations the model-space walk of tier (c) made, measured by
    /// MB1's own `ReadLedger` on the carried lineage. Includes a join
    /// reading taken at a state tier (b) answered.
    pub mixture_reads: u64,
    /// Consultations the σ0 fallback made (one per fallback answer).
    pub field_reads: u64,
}

impl Spend {
    pub fn total(&self) -> u64 {
        self.enumeration_reads
            .saturating_add(self.mixture_reads)
            .saturating_add(self.field_reads)
    }
}

/// The two answers side by side at a state where both exact recursions
/// were affordable. MB1's finding — values move before argmaxes do — is
/// why both are recorded and neither stands in for the other.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinReading {
    /// `Q(ν)` under the carried posterior, exact.
    pub mixture_value: BigRational,
    /// The exact world-space optimum against the declared fixed field.
    pub fixed_field_value: BigRational,
    /// The two values differ.
    pub value_moved: bool,
    /// The model-space argmax at this information state.
    pub mixture_action: Domino,
    /// The world-space argmax at the same information state.
    pub fixed_field_action: Domino,
    /// The two argmaxes differ — the posterior would have changed the
    /// decision.
    pub argmax_flipped: bool,
}

/// What the carried posterior was at this decision, and whether the
/// answering tier read it. Every field is a derived view of the ONE
/// carried [`ModelBelief`]; nothing here is a second authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PosteriorNote {
    /// A live model belief is carried for this seat.
    pub carried: bool,
    /// Live profiles — profiles the posterior has not zeroed.
    pub live_profiles: usize,
    /// Observations of other seats folded into the line so far.
    pub observations: usize,
    /// This seat's own plays folded into the line so far.
    pub focal_plays: usize,
    /// The answering tier read the posterior.
    pub consulted: bool,
    /// The line was retired: an observed action left the library.
    pub falsified: Option<Falsification>,
    /// Both exact answers, where both were affordable.
    pub join: Option<JoinReading>,
}

/// Where the decision was taken. Public state only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionFrame {
    /// `root_identity` of the canonical root built from this state, when
    /// one could be built.
    pub root_id: Option<u64>,
    pub seat: usize,
    /// Tricks completed before this one, plus one.
    pub trick: usize,
    /// Plays already in the current trick — zero at a trick start.
    pub ply: usize,
    /// The fiber mass at this decision, when a kernel was available.
    pub fiber_mass: Option<u128>,
    pub legal_actions: usize,
    /// The declared budget's label, verbatim.
    pub budget: String,
}

// ---------------------------------------------------------------------------
// Provenance and Decision.
// ---------------------------------------------------------------------------

/// The append-only record of one decision's cascade. Private members: it
/// is constructed only by [`UnifiedPlayer::decide`], so a decision that
/// claims a tier necessarily carries that tier's own evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provenance {
    evidence: Evidence,
    authority: String,
    spend: Spend,
    refusals: Vec<TierRefusal>,
    posterior: PosteriorNote,
    frame: DecisionFrame,
}

impl Provenance {
    /// The tier that answered — a derived view of the evidence.
    pub fn tier(&self) -> Tier {
        self.evidence.tier()
    }

    /// Which of the two recursions this answer came out of.
    pub fn recursion(&self) -> Recursion {
        self.evidence.recursion()
    }

    pub fn evidence(&self) -> &Evidence {
        &self.evidence
    }

    /// The instrument that produced the answer, by name.
    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn spend(&self) -> Spend {
        self.spend
    }

    /// Every typed refusal the cascade fell through, in the order it fell
    /// through them.
    pub fn refusals(&self) -> &[TierRefusal] {
        &self.refusals
    }

    pub fn posterior(&self) -> &PosteriorNote {
        &self.posterior
    }

    pub fn frame(&self) -> &DecisionFrame {
        &self.frame
    }
}

/// One decision: the action, and how it was reached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    action: Domino,
    provenance: Provenance,
}

impl Decision {
    pub fn action(&self) -> Domino {
        self.action
    }

    pub fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

// ---------------------------------------------------------------------------
// The measured field.
// ---------------------------------------------------------------------------

/// The declared field, wrapped so its consultations can be counted. It IS
/// the declared field — `id` forwards to the inner policy, because the
/// belief machinery asserts that one field identity governs a belief's
/// conditionings (§43) and because a measurement decorator that renamed
/// the thing it measures would be measuring something else.
struct CountingField<'a> {
    inner: &'a dyn SlicePolicy,
    reads: Cell<u64>,
}

impl<'a> CountingField<'a> {
    fn new(inner: &'a dyn SlicePolicy) -> CountingField<'a> {
        CountingField {
            inner,
            reads: Cell::new(0),
        }
    }

    fn reads(&self) -> u64 {
        self.reads.get()
    }
}

impl SlicePolicy for CountingField<'_> {
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        self.reads.set(self.reads.get().saturating_add(1));
        self.inner.choose(decl, hand, legal, record)
    }
}

// ---------------------------------------------------------------------------
// The store.
// ---------------------------------------------------------------------------

/// The §49 proof states the player may consume, keyed by root identity.
/// The player installs into it what its own exact walks establish, and a
/// caller may seed it with what other instruments established earlier —
/// which is how a God-tight receipt from U0's census reaches a decision
/// as something CONSUMED rather than recomputed.
#[derive(Clone, Debug, Default)]
pub struct ReceiptStore {
    states: BTreeMap<u64, ProofState>,
}

impl ReceiptStore {
    pub fn new() -> ReceiptStore {
        ReceiptStore {
            states: BTreeMap::new(),
        }
    }

    /// Seed one root's proof state. Replaces any state under the same
    /// root identity — the caller owns what it seeds.
    pub fn seed(&mut self, state: ProofState) {
        self.states.insert(state.identity.root_id, state);
    }

    pub fn get(&self, root_id: u64) -> Option<&ProofState> {
        self.states.get(&root_id)
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// Root identities held, in order.
    pub fn roots(&self) -> Vec<u64> {
        self.states.keys().copied().collect()
    }
}

// ---------------------------------------------------------------------------
// The carried line.
// ---------------------------------------------------------------------------

/// One seat's carried model-belief lineage: the belief constructed at
/// that seat's first decision, advanced by
/// [`ModelBelief::focal_play`] when the seat acts and
/// [`ModelBelief::observe`] when another does. The belief is the only
/// authority here; `observations` and `focal_plays` are counters of what
/// was done to it, not a second record of the line.
pub struct SeatLine {
    viewer: Seat,
    root_id: u64,
    model: Option<ModelBelief>,
    falsified: Option<Falsification>,
    observations: usize,
    focal_plays: usize,
}

impl SeatLine {
    pub fn viewer(&self) -> Seat {
        self.viewer
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    /// The carried belief, absent once the line is retired.
    pub fn model(&self) -> Option<&ModelBelief> {
        self.model.as_ref()
    }

    pub fn falsified(&self) -> Option<&Falsification> {
        self.falsified.as_ref()
    }

    pub fn observations(&self) -> usize {
        self.observations
    }

    pub fn focal_plays(&self) -> usize {
        self.focal_plays
    }

    fn note(&self) -> PosteriorNote {
        PosteriorNote {
            carried: self.model.is_some(),
            live_profiles: self.model.as_ref().map_or(0, |m| m.profiles().len()),
            observations: self.observations,
            focal_plays: self.focal_plays,
            consulted: false,
            falsified: self.falsified.clone(),
            join: None,
        }
    }
}

// ---------------------------------------------------------------------------
// The declared semantics identity.
// ---------------------------------------------------------------------------

/// The §51 identity of a fixed-field root under this player's declared
/// semantics — the one construction site, shared by the player and by
/// anything that seeds the store, so "the store already proves this root"
/// is a statement about the same object on both sides.
///
/// `field_id` is the declared field's own content address
/// (`SlicePolicy::id`), so a fact authored under a DIFFERENT field is
/// rejected `IdentityMismatch` by machinery that already exists. The
/// utility coordinate follows the viewer's parity against the declaring
/// team, exactly as the salvation census types it.
pub fn fixed_field_identity(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
    SemanticsIdentity {
        root_id: root_identity(root, position),
        rules_id: "texas42-v1".to_string(),
        field_id: field.id().to_string(),
        utility_id: if declaring {
            "pmake-v1".to_string()
        } else {
            "pmake-setting-v1".to_string()
        },
        contract: position.bid,
        belief_id: "uniform-root".to_string(),
        policy_class_id: "information-consistent-full".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

// ---------------------------------------------------------------------------
// The player.
// ---------------------------------------------------------------------------

/// The declared type library: one behavior type per alternative with its
/// integer prior weight, applied identically to every hidden seat (the
/// independent per-seat prior of MB0/MB1's registered mixture).
pub struct TypeLibrary {
    entries: Vec<(Rc<BehaviorType>, u128)>,
}

impl TypeLibrary {
    /// A library from declared (type, positive weight) pairs. An empty
    /// library is a player that carries no model belief at all — which is
    /// lawful and simply means tier (c) always refuses.
    pub fn new(entries: Vec<(Rc<BehaviorType>, u128)>) -> TypeLibrary {
        for (_, w) in &entries {
            assert!(*w > 0, "a prior type weight is positive (rules invariant)");
        }
        TypeLibrary { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn priors(&self, root: &CanonicalRoot) -> Vec<SeatTypePrior> {
        root.kernel()
            .hidden()
            .iter()
            .map(|slot| SeatTypePrior {
                seat: slot.seat,
                types: self
                    .entries
                    .iter()
                    .map(|(t, w)| (Rc::clone(t), *w))
                    .collect(),
            })
            .collect()
    }
}

/// The unified walt player. One decision function; every instrument the
/// counted-belief program built, consulted in the order the mathematics
/// says they become affordable; provenance always.
pub struct UnifiedPlayer<'a> {
    oracle: &'a dyn ExactCoverOracle,
    field: &'a dyn SlicePolicy,
    library: TypeLibrary,
    store: ReceiptStore,
    lines: BTreeMap<usize, SeatLine>,
}

impl<'a> UnifiedPlayer<'a> {
    /// Construct the player over a declared oracle, a declared field
    /// (the σ0 authority every world-space instrument evaluates against
    /// AND the total fallback), and a declared type library for the model
    /// posterior.
    pub fn new(
        oracle: &'a dyn ExactCoverOracle,
        field: &'a dyn SlicePolicy,
        library: TypeLibrary,
    ) -> UnifiedPlayer<'a> {
        UnifiedPlayer {
            oracle,
            field,
            library,
            store: ReceiptStore::new(),
            lines: BTreeMap::new(),
        }
    }

    /// Seed the store with proof states established elsewhere — U0's
    /// God-tight receipts, a previous session's facts. Consumed, never
    /// recomputed.
    pub fn seed_store(&mut self, store: ReceiptStore) {
        self.store = store;
    }

    pub fn store(&self) -> &ReceiptStore {
        &self.store
    }

    /// One seat's carried line, once it has one.
    pub fn line(&self, seat: Seat) -> Option<&SeatLine> {
        self.lines.get(&seat.index())
    }

    /// Every carried line's viewer seat, in seat order.
    pub fn lines(&self) -> Vec<Seat> {
        self.lines.values().map(|l| l.viewer).collect::<Vec<Seat>>()
    }

    // -- the cascade ------------------------------------------------------

    /// Decide one action. `state` carries the four arguments of the
    /// brief's signature in this crate's own vocabulary: the SEAT is the
    /// derived view `leader.plus(trick_plays.len())`, the HISTORY is the
    /// public record (`prior_played`, `trick_plays`, `banked`, `voids`),
    /// and the HAND is `viewer_hand`. The BUDGET is passed per move so
    /// one player can be walked under a declared ladder and every
    /// decision names the budget it was taken under.
    ///
    /// Total: some tier always answers. The action is always legal.
    pub fn decide(&mut self, state: &DrivenState<'_>, budget: &MoveBudget) -> Decision {
        let seat = state.leader.plus(state.trick_plays.len());
        let led = state
            .trick_plays
            .first()
            .map(|d| state.decl.led_context(*d));
        let legal = legal_plays(state.decl, state.viewer_hand, led);
        assert!(
            !legal.is_empty(),
            "a seat to move holds a legal tile (rules invariant)"
        );
        let mut refusals: Vec<TierRefusal> = Vec::new();
        let mut spend = Spend::default();

        // The public position, buildable with no kernel at all — which is
        // what keeps the fallback total even where the kernel is not.
        let position = RootPosition {
            decl: state.decl,
            bid: state.bid,
            declaring_team: state.declaring_team,
            leader: state.leader,
            banked: state.banked,
            trick_plays: state.trick_plays.to_vec(),
            prior_played: state.prior_played,
            voids: state.voids,
        };
        let canonical = match driven_root(state) {
            Ok(pair) => Some(pair),
            Err(e) => {
                refusals.push(TierRefusal::RootUnavailable {
                    reason: e.to_string(),
                });
                None
            }
        };
        let fiber = canonical.as_ref().map(|(root, pos)| {
            self.oracle
                .mass(&FactorBelief::uniform_root(root, pos, self.field))
        });
        let mut frame = DecisionFrame {
            root_id: canonical
                .as_ref()
                .map(|(root, pos)| root_identity(root, pos)),
            seat: seat.index(),
            trick: state.prior_played.len() / 4 + 1,
            ply: state.trick_plays.len(),
            fiber_mass: fiber,
            legal_actions: legal.len(),
            budget: budget.label.clone(),
        };

        // Open this seat's line at its first decision, so the posterior
        // is carried from the point the seat actually starts reasoning.
        if let Some((root, pos)) = canonical.as_ref() {
            self.open_line(seat, root, pos);
        }

        // ---- (a) terminal/decided arithmetic, free.
        let settled = decided_success(&position, seat, state.banked, false);
        if settled.is_some() || legal.len() == 1 {
            let action = match settled {
                // A settled indicator values every continuation alike, so
                // the declared tie rule answers: the lowest legal tile.
                Some(_) => first_of(legal),
                None => first_of(legal),
            };
            let evidence = Evidence::Decided {
                settled,
                banked: state.banked,
                contract: state.bid,
                legal_actions: legal.len(),
            };
            return self.finish(
                action,
                evidence,
                if settled.is_some() {
                    "adaptive::decided_success"
                } else {
                    "rules::legal_plays:forced"
                },
                spend,
                refusals,
                seat,
                frame,
                None,
            );
        }

        // ---- (b) endgame exact: consume the store, else enumerate.
        if let (Some((root, pos)), Some(z)) = (canonical.as_ref(), fiber) {
            // (b1) The store already proves this root.
            if let Some((action, receipt)) = self.consume_receipt(root, pos, legal, &mut refusals) {
                let evidence = Evidence::Consumed(Box::new(receipt));
                return self.finish(
                    action,
                    evidence,
                    "proof_state::recommend:certified",
                    spend,
                    refusals,
                    seat,
                    frame,
                    None,
                );
            }
            // (b2) The exact world-space recursion, where the fiber
            // affords it.
            if z <= budget.enumeration_fiber_cap {
                let counted = CountingField::new(self.field);
                let belief = FactorBelief::uniform_root(root, pos, &counted);
                let mut estats = ResponseStats::default();
                let (optimum, policy) = extract_success_policy(
                    self.oracle,
                    &belief,
                    &ExtractionSource::FullLegal,
                    &counted,
                    &mut estats,
                );
                let mut pstats = RecursionStats::default();
                let repriced =
                    viewer_success_mass(self.oracle, &belief, &policy, &counted, &mut pstats);
                spend.enumeration_reads = counted.reads();
                if repriced != optimum {
                    refusals.push(TierRefusal::RepricingDisagreed {
                        extracted: optimum,
                        repriced,
                    });
                } else {
                    let action = match policy.choice_at(&[]) {
                        Some(a) if legal.contains(a) => Some(a),
                        _ => None,
                    };
                    match action {
                        Some(action) => {
                            let value = BigRational::new(BigInt::from(repriced), BigInt::from(z));
                            let fixed_field_answer = Some((action, value));
                            let evidence = Evidence::Enumerated {
                                optimum_mass: optimum,
                                repriced_mass: repriced,
                                fiber_mass: z,
                                policy_id: policy.id().to_string(),
                                policy_states: policy.states(),
                            };
                            // Deposit what the walk established, so the
                            // next visit consumes instead of repeating.
                            self.install_exact(root, pos, legal, action, repriced, z, &policy);
                            // The join reading, where the budget declares
                            // it: price the model side too and record
                            // both answers rather than either alone.
                            let join = if budget.join_reading {
                                self.join_reading(
                                    seat,
                                    legal,
                                    budget,
                                    z,
                                    &fixed_field_answer,
                                    &mut spend,
                                    &mut refusals,
                                )
                            } else {
                                None
                            };
                            return self.finish(
                                action,
                                evidence,
                                "factor_belief::extract_success_policy",
                                spend,
                                refusals,
                                seat,
                                frame,
                                join,
                            );
                        }
                        None => refusals.push(TierRefusal::MixtureNoChoice {
                            policy_id: policy.id().to_string(),
                        }),
                    }
                }
            } else {
                refusals.push(TierRefusal::EnumerationUnaffordable {
                    fiber: z,
                    cap: budget.enumeration_fiber_cap,
                });
            }
        }

        // ---- (c) middlegame budgeted: the exact model-space response
        // under the carried posterior.
        if let Some((action, evidence)) =
            self.mixture_answer(seat, legal, budget, fiber, &mut spend, &mut refusals)
        {
            return self.finish(
                action,
                evidence,
                "model_belief::mixture_response_budgeted",
                spend,
                refusals,
                seat,
                frame,
                None,
            );
        }

        // ---- (d) the §33 recommendation off whatever the store holds.
        if let Some((root, pos)) = canonical.as_ref() {
            if let Some((action, evidence)) =
                self.recommend_within(root, pos, legal, budget, &mut refusals)
            {
                return self.finish(
                    action,
                    evidence,
                    "proof_state::recommend",
                    spend,
                    refusals,
                    seat,
                    frame,
                    None,
                );
            }
        }

        // ---- (e) the σ0 fallback. Total: the declared field always
        // answers, and the answer is named as the fallback it is.
        let record = PublicRecord {
            leader: state.leader,
            trick_plays: state.trick_plays,
            banked: state.banked,
            root: &position,
            history: &[],
        };
        let counted = CountingField::new(self.field);
        let action = counted.choose(state.decl, state.viewer_hand, legal, &record);
        spend.field_reads = counted.reads();
        frame.legal_actions = legal.len();
        let evidence = Evidence::Field {
            field_id: self.field.id().to_string(),
        };
        self.finish(
            action,
            evidence,
            "field::SlicePolicy::choose:fallback",
            spend,
            refusals,
            seat,
            frame,
            None,
        )
    }

    /// Advance every carried line past one play. The driver calls this
    /// after every ply, whoever made it: the line whose viewer played
    /// advances by [`ModelBelief::focal_play`], every other line by
    /// [`ModelBelief::observe`]. A line whose library does not support
    /// the observed action is RETIRED with a typed
    /// [`Falsification`] — never widened, never re-seeded.
    pub fn observe_play(&mut self, state: &DrivenState<'_>, tile: Domino) {
        let actor = state.leader.plus(state.trick_plays.len());
        for line in self.lines.values_mut() {
            let Some(model) = line.model.take() else {
                continue;
            };
            if model.seat_to_move() == line.viewer {
                if line.viewer == actor {
                    line.model = Some(model.focal_play(tile));
                    line.focal_plays += 1;
                } else {
                    // The carried line disagrees with the driver about
                    // whose turn it is; a belief that cannot be advanced
                    // truthfully is retired rather than forced.
                    line.falsified = Some(Falsification {
                        history: model.history().to_vec(),
                        seat: actor.index(),
                        observed: tile,
                        supported: Vec::new(),
                    });
                }
                continue;
            }
            let supported = model.branch_masses(self.oracle);
            if supported.iter().any(|(t, _)| *t == tile) {
                line.model = Some(model.observe(self.oracle, tile));
                line.observations += 1;
            } else {
                line.falsified = Some(Falsification {
                    history: model.history().to_vec(),
                    seat: actor.index(),
                    observed: tile,
                    supported: supported.into_iter().map(|(t, _)| t).collect(),
                });
            }
        }
    }

    // -- the tiers, one function each --------------------------------------

    fn open_line(&mut self, seat: Seat, root: &CanonicalRoot, position: &RootPosition) {
        if self.library.is_empty() || self.lines.contains_key(&seat.index()) {
            return;
        }
        let model = ModelBelief::from_independent_prior(root, position, &self.library.priors(root));
        self.lines.insert(
            seat.index(),
            SeatLine {
                viewer: seat,
                root_id: root_identity(root, position),
                model: Some(model),
                falsified: None,
                observations: 0,
                focal_plays: 0,
            },
        );
    }

    /// (b1) — consume a root the store already proves at zero certified
    /// regret. No walk runs; the spend is zero.
    fn consume_receipt(
        &self,
        root: &CanonicalRoot,
        position: &RootPosition,
        legal: DominoSet,
        refusals: &mut Vec<TierRefusal>,
    ) -> Option<(Domino, ConsumedReceipt)> {
        if !position.trick_plays.is_empty() {
            refusals.push(TierRefusal::ProofStateUnavailable {
                plays_in_trick: position.trick_plays.len(),
            });
            return None;
        }
        let state = self.store.get(root_identity(root, position))?;
        let report = state.closure();
        let Some(rec) = state.recommend() else {
            refusals.push(TierRefusal::NoExecutableWitness {
                facts: state.facts().len(),
            });
            return None;
        };
        if !rec
            .certified_regret
            .eq(&BigRational::new(BigInt::from(0), BigInt::from(1)))
        {
            refusals.push(TierRefusal::ReceiptNotCertified {
                regret: rec.certified_regret.clone(),
            });
            return None;
        }
        if !legal.contains(rec.action) {
            refusals.push(TierRefusal::ReceiptNotCertified {
                regret: rec.certified_regret.clone(),
            });
            return None;
        }
        let upper = match report.views.iter().find(|v| v.action == rec.action) {
            Some(v) => v.upper.clone(),
            None => rec.global_upper.clone(),
        };
        let fact_id = match state
            .facts()
            .iter()
            .find(|f| matches!(&f.fact, Fact::Profile(p) if p.action == rec.action))
        {
            Some(f) => f.id,
            // A bound-fact witness carries no profile; the recommendation
            // names its authority, and the store's own id is what the
            // consumer checks against.
            None => 0,
        };
        Some((
            rec.action,
            ConsumedReceipt {
                policy: rec.policy.clone(),
                value: rec.pmake_lower.clone(),
                upper,
                fact_id,
                facts: state.facts().len(),
            },
        ))
    }

    /// Deposit what an exact walk established: the optimum is an
    /// EXECUTABLE lower for the chosen action (a materialized policy
    /// attains it and was re-priced to it) and a deterministic upper for
    /// EVERY legal action, because the optimum of the state is the max
    /// over actions and therefore bounds each of them.
    #[allow(clippy::too_many_arguments)]
    fn install_exact(
        &mut self,
        root: &CanonicalRoot,
        position: &RootPosition,
        legal: DominoSet,
        action: Domino,
        mass: u128,
        fiber: u128,
        policy: &dyn SlicePolicy,
    ) {
        if !position.trick_plays.is_empty() || fiber == 0 {
            return;
        }
        let identity = fixed_field_identity(root, position, self.field);
        let root_id = identity.root_id;
        let value = BigRational::new(BigInt::from(mass), BigInt::from(fiber));
        let state = self
            .store
            .states
            .entry(root_id)
            .or_insert_with(|| ProofState::open(root, position, identity.clone()));
        if state.identity != identity {
            return;
        }
        let authority = format!("unified-up0:extraction:{}", policy.id());
        let _: Result<u128, Reject> = state.install(
            &identity,
            Fact::Bound(BoundFact::lower(
                action,
                value.clone(),
                &authority,
                true,
                ProofTag::Deterministic,
            )),
        );
        for a in legal.iter() {
            let _: Result<u128, Reject> = state.install(
                &identity,
                Fact::Bound(BoundFact::upper(
                    a,
                    value.clone(),
                    "unified-up0:state-optimum",
                    ProofTag::Deterministic,
                )),
            );
        }
    }

    /// (c) — the exact model-space response under the carried posterior.
    fn mixture_answer(
        &mut self,
        seat: Seat,
        legal: DominoSet,
        budget: &MoveBudget,
        fiber: Option<u128>,
        spend: &mut Spend,
        refusals: &mut Vec<TierRefusal>,
    ) -> Option<(Domino, Evidence)> {
        let (mass, total, policy_id, live, reads, action) =
            self.mixture_walk(seat, legal, budget, fiber, refusals)?;
        spend.mixture_reads = spend.mixture_reads.saturating_add(reads);
        Some((
            action,
            Evidence::Mixture {
                weighted_mass: mass,
                weighted_total: total,
                policy_id,
                live_profiles: live,
                reads,
            },
        ))
    }

    /// The shared body of tier (c) and of the join reading: one budgeted
    /// mixture response off the carried line, with every refusal typed.
    #[allow(clippy::type_complexity)]
    fn mixture_walk(
        &mut self,
        seat: Seat,
        legal: DominoSet,
        budget: &MoveBudget,
        fiber: Option<u128>,
        refusals: &mut Vec<TierRefusal>,
    ) -> Option<(u128, u128, String, usize, u64, Domino)> {
        let line = self.lines.get(&seat.index())?;
        if let Some(f) = &line.falsified {
            refusals.push(TierRefusal::PosteriorFalsified(Box::new(f.clone())));
            return None;
        }
        let model = line.model.as_ref()?;
        if model.seat_to_move() != line.viewer {
            refusals.push(TierRefusal::LineNotFocal {
                viewer: line.viewer.index(),
                to_move: model.seat_to_move().index(),
            });
            return None;
        }
        if let Some(z) = fiber {
            if z > budget.mixture_fiber_cap {
                refusals.push(TierRefusal::MixtureUnaffordable {
                    fiber: z,
                    cap: budget.mixture_fiber_cap,
                });
                return None;
            }
        }
        let baseline = model.ledger().total();
        let mut stats = MixtureStats::default();
        let response = match model.mixture_response_budgeted(
            self.oracle,
            Some(budget.mixture_read_cap),
            &mut stats,
        ) {
            Ok(r) => r,
            Err(e) => {
                refusals.push(TierRefusal::MixtureRefused {
                    refusal: refusal_text(&e),
                });
                return None;
            }
        };
        let reads = model.ledger().total().saturating_sub(baseline);
        let history = model.history().to_vec();
        let Some(action) = response.policy.choice_at(&history) else {
            refusals.push(TierRefusal::MixtureNoChoice {
                policy_id: response.policy.id().to_string(),
            });
            return None;
        };
        if !legal.contains(action) {
            refusals.push(TierRefusal::MixtureNoChoice {
                policy_id: response.policy.id().to_string(),
            });
            return None;
        }
        Some((
            response.outcome.weighted_mass,
            response.outcome.weighted_total,
            response.policy.id().to_string(),
            model.profiles().len(),
            reads,
            action,
        ))
    }

    /// The join: price the model side at a state the world side already
    /// answered, and record both answers. The decision is NOT changed —
    /// the cascade's order is the declared one — but the reading says
    /// whether the posterior would have moved the value, the argmax, or
    /// neither.
    #[allow(clippy::too_many_arguments)]
    fn join_reading(
        &mut self,
        seat: Seat,
        legal: DominoSet,
        budget: &MoveBudget,
        fiber: u128,
        fixed: &Option<(Domino, BigRational)>,
        spend: &mut Spend,
        refusals: &mut Vec<TierRefusal>,
    ) -> Option<JoinReading> {
        let (fixed_action, fixed_value) = fixed.as_ref()?;
        let (mass, total, _, _, reads, action) =
            self.mixture_walk(seat, legal, budget, Some(fiber), refusals)?;
        spend.mixture_reads = spend.mixture_reads.saturating_add(reads);
        if total == 0 {
            return None;
        }
        let mixture_value = BigRational::new(BigInt::from(mass), BigInt::from(total));
        Some(JoinReading {
            value_moved: mixture_value != *fixed_value,
            argmax_flipped: action != *fixed_action,
            mixture_value,
            fixed_field_value: fixed_value.clone(),
            mixture_action: action,
            fixed_field_action: *fixed_action,
        })
    }

    /// (d) — the §33 recommendation off the store's existing facts,
    /// accepted only within the declared certified-regret ceiling.
    fn recommend_within(
        &self,
        root: &CanonicalRoot,
        position: &RootPosition,
        legal: DominoSet,
        budget: &MoveBudget,
        refusals: &mut Vec<TierRefusal>,
    ) -> Option<(Domino, Evidence)> {
        if !position.trick_plays.is_empty() {
            return None;
        }
        let state = self.store.get(root_identity(root, position))?;
        let Some(rec) = state.recommend() else {
            refusals.push(TierRefusal::NoRecommendation {
                facts: state.facts().len(),
            });
            return None;
        };
        if rec.certified_regret > budget.regret_acceptance {
            refusals.push(TierRefusal::RegretAboveAcceptance {
                regret: rec.certified_regret.clone(),
                acceptance: budget.regret_acceptance.clone(),
            });
            return None;
        }
        if !legal.contains(rec.action) {
            return None;
        }
        Some((
            rec.action,
            Evidence::CertifiedRegret {
                policy: rec.policy.clone(),
                pmake_lower: rec.pmake_lower.clone(),
                global_upper: rec.global_upper.clone(),
                certified_regret: rec.certified_regret.clone(),
                facts: state.facts().len(),
            },
        ))
    }

    /// Assemble the decision. The single construction site of
    /// [`Provenance`], which is why a tier cannot be claimed without its
    /// evidence.
    #[allow(clippy::too_many_arguments)]
    fn finish(
        &self,
        action: Domino,
        evidence: Evidence,
        authority: &str,
        spend: Spend,
        refusals: Vec<TierRefusal>,
        seat: Seat,
        frame: DecisionFrame,
        join: Option<JoinReading>,
    ) -> Decision {
        let mut posterior = match self.lines.get(&seat.index()) {
            Some(line) => line.note(),
            None => PosteriorNote {
                carried: false,
                live_profiles: 0,
                observations: 0,
                focal_plays: 0,
                consulted: false,
                falsified: None,
                join: None,
            },
        };
        posterior.consulted = matches!(evidence, Evidence::Mixture { .. }) || join.is_some();
        posterior.join = join;
        Decision {
            action,
            provenance: Provenance {
                evidence,
                authority: authority.to_string(),
                spend,
                refusals,
                posterior,
                frame,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Small shared helpers.
// ---------------------------------------------------------------------------

/// The lowest-index tile of a nonempty legal set — the stack's declared
/// tie rule, and the answer every value-blind tier gives.
fn first_of(legal: DominoSet) -> Domino {
    let mut chosen = Domino::ALL[0];
    let mut found = false;
    for d in legal.iter() {
        if !found {
            chosen = d;
            found = true;
        }
    }
    assert!(found, "a seat to move holds a legal tile (rules invariant)");
    chosen
}

/// MB1's typed refusal, rendered for the record. The refusal's own
/// `Display` carries the measured spend, the declared ceiling and the
/// history it stopped at, so nothing is restated here.
fn refusal_text(refusal: &MixtureRefusal) -> String {
    format!("{refusal}")
}
