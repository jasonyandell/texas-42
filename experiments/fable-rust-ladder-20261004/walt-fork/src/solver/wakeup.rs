//! `solver::wakeup` — the level-2 detection layer (§22 step 9): typed
//! wake-up records for one frozen action pair under a declared (σ0, σ1)
//! field pair.
//!
//! EXPLORATORY tier. Implements `walt/LEVEL2-PROBE.md` as amended — the
//! CE-A6 wake-up split (`walt/math/calculated_evidence_v0.1.md` §14) under
//! the L2-A5 role assignment (`walt/CENSUS-RULINGS.md`): this module is
//! the *detection layer* of the one field-swap program; the targeting
//! layer (exposure bounds, the stability screen, survivor-only field-1
//! optimization) is owned by `solver::field_swap` and the targeted parent,
//! and nothing here restates it.
//!
//! Three distinct wake-up objects, never collapsed (§14; CE-A6):
//!
//! - **Response wake-up** ([`ResponseWakeUp`]) — `q₁ − q₀ > ε_q`: newly
//!   active response structure. NOT by itself a value statement: the
//!   upgraded field may create many disagreements whose signs balance
//!   exactly (§14.4's `q₁ > 0, τ₁ = 0, g₁ = 0`). There is no conversion
//!   from this type to [`ValueWakeUp`], and none may be added:
//!
//! ```compile_fail
//! use walt::solver::wakeup::{ResponseWakeUp, ValueWakeUp};
//!
//! fn response_wake_is_not_a_value_statement(r: ResponseWakeUp) -> ValueWakeUp {
//!     // No such conversion exists in either direction (§14.4): this
//!     // does not compile.
//!     ValueWakeUp::from(r)
//! }
//! ```
//!
//! - **Value wake-up** ([`ValueWakeUp`]) — the signed gap change
//!   `g₁ − g₀`, settled by the §14.6 paired field-correction evidence:
//!   on the same world, `Z = Y⁽¹⁾ − Y⁽⁰⁾ ∈ {−2..2}` with
//!   `E[Z] = g₁ − g₀`, driven through the CE bounded-mean engine on
//!   `X = Z/2` (consumed verbatim from `solver::evidence`, never
//!   reimplemented).
//!
//! - **Decision wake-up** ([`DecisionWakeUp`]) — the pair's selected
//!   action changes, or an open comparison becomes settled.
//!
//! Sampling cost under each field is compared ONLY by the information
//! rate `𝓘_f = q_f · D_{1/2}(τ_f)` ([`InformationComparison`], §14.5) —
//! never by `q̂` alone and never by a plug-in `Ĥ` ordering; no hardness
//! accessor exists here.
//!
//! **Exact-zero discipline (§14.7), type-enforced.** A claim `q = 0`
//! requires full enumeration (the exact route) or a structural proof;
//! sampling supports only `q ≤ ε_q` at declared risk. The sampled
//! coordinate type has no exact-zero reading:
//!
//! ```compile_fail
//! use walt::solver::wakeup::SampledFieldCoordinates;
//!
//! fn a_sample_never_pronounces_exact_zero(s: &SampledFieldCoordinates) -> bool {
//!     // No such method exists on the sampled type (§14.7): this does
//!     // not compile.
//!     s.exactly_zero()
//! }
//! ```
//!
//! A practical-zero statement is a crossing witness, never a convention —
//! [`PracticalZero`] has private fields and no public constructor; the
//! only producer is a [`PracticalZeroProbe`] whose evidence crossed its
//! declared threshold:
//!
//! ```compile_fail
//! use walt::solver::wakeup::PracticalZero;
//!
//! fn practical_zero_needs_a_crossing() -> PracticalZero {
//!     // Private fields, no public constructor: this does not compile.
//!     PracticalZero { settled_at: 0 }
//! }
//! ```
//!
//! And the exact record itself cannot be forged from sampled data — its
//! witness type has no public constructor; the only producer is
//! [`exact_paired_detection`], which asserts complete-fiber domains:
//!
//! ```compile_fail
//! use walt::solver::wakeup::ExactFiberWitness;
//!
//! fn a_sample_cannot_forge_the_enumeration_witness() -> ExactFiberWitness {
//!     ExactFiberWitness { _private: () }
//! }
//! ```
//!
//! Discipline inherited from the standing rulings: one (σ0, σ1) pair per
//! epoch — every record names both [`FieldId`]s, and a σ0 change is a new
//! experiment (LEVEL2-PROBE gates; targeted parent §8 Stage 0); an open
//! result is a successful output (CE §1.5); a resource cap is a resource
//! limit, never a settlement rule (CE-A3/A5); a root whose declared
//! budget cannot honestly buy its evaluation gets a typed refusal
//! ([`DetectionRefusal`]), never a degraded number. Derived views, never
//! stored state: every aggregate here is recomputed from the per-world
//! audit rows or the folded engine states, and cross-checked against the
//! independent `field_swap` producers where both exist.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, world_id, CanonicalRoot, RootPosition};
use crate::solver::calibrate::{information_rate_bounds, pair_coordinates, PairCoordinates};
use crate::solver::evidence::{
    crossed, edge_threshold, pivotal_evidence, BoundedMeanMixture, MeanNull, MixtureError,
    ScopedDelta,
};
use crate::solver::exposure::{coupled_replay, FrozenPolicyExposure, WorldDomain, WorldRow};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::field_swap::{pair_lift, CancellationLadder, SplitAggregate};
use crate::solver::policy::{FrozenPolicy, PolicyId};

/// Series depth for the exact rational tail bounds of every information
/// rate in this module (one declared constant).
pub const INFO_TERMS: u32 = 24;

// ---------------------------------------------------------------------------
// Practical zero (§14.7's sampled half): a crossing witness, not a label.
// ---------------------------------------------------------------------------

/// An ESTABLISHED practical-zero statement: `q < ε_q` at the declared
/// scope risk, witnessed by the [`PracticalZeroProbe`]'s evidence
/// crossing its threshold. Private fields, no public constructor — a
/// sampled zero count can never inhabit this type by convention alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PracticalZero {
    eps_q: BigRational,
    delta: ScopedDelta,
    settled_at: u64,
    evidence: BigRational,
}

impl PracticalZero {
    /// The declared tolerance the statement is relative to.
    pub fn eps_q(&self) -> &BigRational {
        &self.eps_q
    }

    /// The declared scope risk the statement spends.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }

    /// The observation count at the crossing.
    pub fn settled_at(&self) -> u64 {
        self.settled_at
    }

    /// The exact mixture evidence at the crossing.
    pub fn evidence(&self) -> &BigRational {
        &self.evidence
    }
}

/// The practical-zero engine: the CE-T5 bounded-mean process on the
/// pivotal indicator `X ∈ {0, 1} ⊂ [0, 1]` testing `H0: q ≥ ε_q`;
/// crossing `1/δ` (Ville) establishes `q < ε_q` at the declared risk.
/// Absence of a crossing establishes NOTHING — the honest open state.
pub struct PracticalZeroProbe {
    eps_q: BigRational,
    delta: ScopedDelta,
    threshold: BigRational,
    mixture: BoundedMeanMixture,
    witness: Option<PracticalZero>,
}

impl PracticalZeroProbe {
    /// A lawful probe at the declared tolerance, scope risk, and betting
    /// mixture (`(weight, λ)` pairs; the CE-T5 validity contract is
    /// checked at construction).
    pub fn new(
        eps_q: BigRational,
        delta: ScopedDelta,
        mixture: &[(BigRational, BigRational)],
    ) -> Result<PracticalZeroProbe, MixtureError> {
        assert!(
            eps_q > BigRational::zero() && eps_q < BigRational::one(),
            "a declared pivotal-mass tolerance lies strictly inside (0, 1)"
        );
        let threshold = delta.delta().recip();
        let engine = BoundedMeanMixture::new(
            MeanNull::AtLeast,
            BigRational::zero(),
            BigRational::one(),
            eps_q.clone(),
            mixture,
        )?;
        Ok(PracticalZeroProbe {
            eps_q,
            delta,
            threshold,
            mixture: engine,
            witness: None,
        })
    }

    /// Fold one world's pivotal indicator. The FIRST crossing mints the
    /// witness; further observations keep folding (anytime validity is
    /// indifferent to the stopping rule).
    pub fn observe(&mut self, pivotal: bool) {
        let x = if pivotal {
            BigRational::one()
        } else {
            BigRational::zero()
        };
        self.mixture.observe(&x);
        if self.witness.is_none() && crossed(&self.mixture.evidence(), &self.threshold) {
            self.witness = Some(PracticalZero {
                eps_q: self.eps_q.clone(),
                delta: self.delta.clone(),
                settled_at: self.mixture.observations(),
                evidence: self.mixture.evidence(),
            });
        }
    }

    /// The witness, if the evidence has crossed.
    pub fn witness(&self) -> Option<&PracticalZero> {
        self.witness.as_ref()
    }

    /// The current exact mixture evidence.
    pub fn evidence(&self) -> BigRational {
        self.mixture.evidence()
    }

    /// Worlds folded so far.
    pub fn observations(&self) -> u64 {
        self.mixture.observations()
    }
}

// ---------------------------------------------------------------------------
// The value-direction engine (§14.6): paired field-correction evidence.
// ---------------------------------------------------------------------------

/// The settled direction of the field correction `g₁ − g₀`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// `g₁ − g₀ > 0` at the declared risk.
    Positive,
    /// `g₁ − g₀ < 0` at the declared risk.
    Negative,
}

impl Direction {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(self) -> &'static str {
        match self {
            Direction::Positive => "positive",
            Direction::Negative => "negative",
        }
    }
}

/// A settled value-direction statement (private fields: only the
/// [`DirectionProbe`]'s crossing produces one).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectionSettled {
    direction: Direction,
    settled_at: u64,
    evidence: BigRational,
}

impl DirectionSettled {
    /// The settled direction.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// The observation count at the crossing.
    pub fn settled_at(&self) -> u64 {
        self.settled_at
    }

    /// The exact mixture evidence at the crossing.
    pub fn evidence(&self) -> &BigRational {
        &self.evidence
    }
}

/// §14.6 — the paired field-correction engine: on each world,
/// `Z = Y⁽¹⁾ − Y⁽⁰⁾ ∈ {−2..2}` with `E[Z] = g₁ − g₀`; two one-sided
/// CE-T4/T5 processes on `X = Z/2 ∈ [−1, 1]` against the null mean 0,
/// each at threshold `2/δ` (the declared scope risk split across the two
/// directions by a plain union bound). `Z` is never reduced to its sign —
/// difference magnitude matters.
pub struct DirectionProbe {
    delta: ScopedDelta,
    threshold: BigRational,
    up: BoundedMeanMixture,
    down: BoundedMeanMixture,
    settled: Option<DirectionSettled>,
}

impl DirectionProbe {
    /// A lawful probe at the declared scope risk and betting mixture.
    pub fn new(
        delta: ScopedDelta,
        mixture: &[(BigRational, BigRational)],
    ) -> Result<DirectionProbe, MixtureError> {
        let two = BigRational::from_integer(BigInt::from(2));
        let threshold = &two / delta.delta();
        let up = BoundedMeanMixture::new(
            MeanNull::AtMost,
            -BigRational::one(),
            BigRational::one(),
            BigRational::zero(),
            mixture,
        )?;
        let down = BoundedMeanMixture::new(
            MeanNull::AtLeast,
            -BigRational::one(),
            BigRational::one(),
            BigRational::zero(),
            mixture,
        )?;
        Ok(DirectionProbe {
            delta,
            threshold,
            up,
            down,
            settled: None,
        })
    }

    /// Fold one world's paired correction difference `Z ∈ {−2..2}`.
    pub fn observe(&mut self, z: i8) {
        assert!(
            (-2..=2).contains(&z),
            "a paired Boolean-payoff correction difference lies in {{-2..2}}"
        );
        let x = BigRational::new(BigInt::from(z), BigInt::from(2));
        self.up.observe(&x);
        self.down.observe(&x);
        if self.settled.is_none() {
            let (direction, evidence) = if crossed(&self.up.evidence(), &self.threshold) {
                (Direction::Positive, self.up.evidence())
            } else if crossed(&self.down.evidence(), &self.threshold) {
                (Direction::Negative, self.down.evidence())
            } else {
                return;
            };
            self.settled = Some(DirectionSettled {
                direction,
                settled_at: self.up.observations(),
                evidence,
            });
        }
    }

    /// The settled direction, if either side has crossed.
    pub fn settled(&self) -> Option<&DirectionSettled> {
        self.settled.as_ref()
    }

    /// The current exact evidence for `g₁ − g₀ > 0`.
    pub fn evidence_up(&self) -> BigRational {
        self.up.evidence()
    }

    /// The current exact evidence for `g₁ − g₀ < 0`.
    pub fn evidence_down(&self) -> BigRational {
        self.down.evidence()
    }

    /// Worlds folded so far.
    pub fn observations(&self) -> u64 {
        self.up.observations()
    }

    /// The declared scope risk.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }
}

// ---------------------------------------------------------------------------
// The response engine (§14.1): newly active response structure.
// ---------------------------------------------------------------------------

/// An ESTABLISHED response wake-up: `q₁ − q₀ > ε_q` at the declared scope
/// risk (private fields: only the [`ResponseProbe`]'s crossing produces
/// one). A response-geometry statement — deliberately NOT convertible to
/// any value statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseEstablished {
    eps_q: BigRational,
    delta: ScopedDelta,
    settled_at: u64,
    evidence: BigRational,
}

impl ResponseEstablished {
    /// The declared response tolerance.
    pub fn eps_q(&self) -> &BigRational {
        &self.eps_q
    }

    /// The declared scope risk.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }

    /// The observation count at the crossing.
    pub fn settled_at(&self) -> u64 {
        self.settled_at
    }

    /// The exact mixture evidence at the crossing.
    pub fn evidence(&self) -> &BigRational {
        &self.evidence
    }
}

/// §14.1 — the response engine: on each world,
/// `W = 1{pivotal under σ1} − 1{pivotal under σ0} ∈ {−1, 0, 1}` with
/// `E[W] = q₁ − q₀`; the CE-T4 process on `W` against the null
/// `E[W] ≤ ε_q` at threshold `1/δ`. Crossing establishes response
/// wake-up at the declared risk; absence establishes nothing.
pub struct ResponseProbe {
    eps_q: BigRational,
    delta: ScopedDelta,
    threshold: BigRational,
    mixture: BoundedMeanMixture,
    established: Option<ResponseEstablished>,
}

impl ResponseProbe {
    /// A lawful probe at the declared tolerance, scope risk, and betting
    /// mixture.
    pub fn new(
        eps_q: BigRational,
        delta: ScopedDelta,
        mixture: &[(BigRational, BigRational)],
    ) -> Result<ResponseProbe, MixtureError> {
        assert!(
            eps_q > BigRational::zero() && eps_q < BigRational::one(),
            "a declared response tolerance lies strictly inside (0, 1)"
        );
        let threshold = delta.delta().recip();
        let engine = BoundedMeanMixture::new(
            MeanNull::AtMost,
            -BigRational::one(),
            BigRational::one(),
            eps_q.clone(),
            mixture,
        )?;
        Ok(ResponseProbe {
            eps_q,
            delta,
            threshold,
            mixture: engine,
            established: None,
        })
    }

    /// Fold one world's pivotal-indicator pair (σ1 first — the wake-up
    /// direction is `q₁` over `q₀`).
    pub fn observe(&mut self, pivotal1: bool, pivotal0: bool) {
        let w = i8::from(pivotal1) - i8::from(pivotal0);
        let x = BigRational::from_integer(BigInt::from(w));
        self.mixture.observe(&x);
        if self.established.is_none() && crossed(&self.mixture.evidence(), &self.threshold) {
            self.established = Some(ResponseEstablished {
                eps_q: self.eps_q.clone(),
                delta: self.delta.clone(),
                settled_at: self.mixture.observations(),
                evidence: self.mixture.evidence(),
            });
        }
    }

    /// The establishment witness, if the evidence has crossed.
    pub fn established(&self) -> Option<&ResponseEstablished> {
        self.established.as_ref()
    }

    /// The current exact mixture evidence.
    pub fn evidence(&self) -> BigRational {
        self.mixture.evidence()
    }

    /// Worlds folded so far.
    pub fn observations(&self) -> u64 {
        self.mixture.observations()
    }
}

// ---------------------------------------------------------------------------
// The per-field pair-decision engine on the common paired stream.
// ---------------------------------------------------------------------------

/// Which member of the frozen pair a settled decision selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairWinner {
    /// The first-listed candidate.
    A,
    /// The second-listed candidate.
    B,
}

impl PairWinner {
    /// The mechanical type tag.
    pub fn tag(self) -> &'static str {
        match self {
            PairWinner::A => "a",
            PairWinner::B => "b",
        }
    }
}

/// A δ-settled pair decision under ONE field (private fields: only the
/// [`PairDecisionProbe`]'s crossing produces one).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairDecisionSettled {
    winner: PairWinner,
    settled_at: u64,
    evidence: BigRational,
}

impl PairDecisionSettled {
    /// The settled winner.
    pub fn winner(&self) -> PairWinner {
        self.winner
    }

    /// The observation count at the crossing.
    pub fn settled_at(&self) -> u64 {
        self.settled_at
    }

    /// The exact pivotal evidence at the crossing.
    pub fn evidence(&self) -> &BigRational {
        &self.evidence
    }
}

/// One field's pair-decision engine on the common paired stream: the
/// CE-T3 exact pivotal-direction evidence on the running counts `(a, b)`,
/// both directions checked against the m = 2 edge threshold
/// (`solver::evidence::edge_threshold`). δ-settled means settled at the
/// declared scope risk — probabilistic, never exact.
pub struct PairDecisionProbe {
    delta: ScopedDelta,
    threshold: BigRational,
    a: u64,
    b: u64,
    observations: u64,
    settled: Option<PairDecisionSettled>,
}

impl PairDecisionProbe {
    /// A lawful probe at the declared scope risk.
    pub fn new(delta: ScopedDelta) -> PairDecisionProbe {
        let threshold = edge_threshold(2, delta.delta());
        PairDecisionProbe {
            delta,
            threshold,
            a: 0,
            b: 0,
            observations: 0,
            settled: None,
        }
    }

    /// Fold one world's signed pair outcome `Y ∈ {−1, 0, 1}` under this
    /// probe's field.
    pub fn observe(&mut self, y: i8) {
        assert!(
            (-1..=1).contains(&y),
            "a Boolean-payoff pair outcome lies in {{-1, 0, 1}}"
        );
        self.observations += 1;
        match y {
            1 => self.a += 1,
            -1 => self.b += 1,
            _ => {}
        }
        if self.settled.is_none() {
            let (winner, evidence) = if crossed(&pivotal_evidence(self.a, self.b), &self.threshold)
            {
                (PairWinner::A, pivotal_evidence(self.a, self.b))
            } else if crossed(&pivotal_evidence(self.b, self.a), &self.threshold) {
                (PairWinner::B, pivotal_evidence(self.b, self.a))
            } else {
                return;
            };
            self.settled = Some(PairDecisionSettled {
                winner,
                settled_at: self.observations,
                evidence,
            });
        }
    }

    /// The settled decision, if either direction has crossed.
    pub fn settled(&self) -> Option<&PairDecisionSettled> {
        self.settled.as_ref()
    }

    /// The running pivotal counts `(a, b)`.
    pub fn counts(&self) -> (u64, u64) {
        (self.a, self.b)
    }

    /// Worlds folded so far.
    pub fn observations(&self) -> u64 {
        self.observations
    }

    /// The declared scope risk.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }
}

// ---------------------------------------------------------------------------
// Sampled per-field coordinates: estimates, mechanically labeled.
// ---------------------------------------------------------------------------

/// One field's pair coordinates over a declared stream prefix — exact
/// counts over the enumerated sample, ESTIMATES of the fiber masses.
/// Deliberately has NO exact-zero reading (§14.7): a zero pivot count
/// here supports only `q ≤ ε_q` at declared risk, through a
/// [`PracticalZero`] crossing witness, never through this type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampledFieldCoordinates {
    /// Worlds where the first candidate alone succeeds.
    pub a: u64,
    /// Worlds where the second candidate alone succeeds.
    pub b: u64,
    /// Worlds enumerated from the declared stream prefix.
    pub worlds: u64,
}

impl SampledFieldCoordinates {
    /// The sample estimate of the pivotal mass `q`.
    pub fn q_hat(&self) -> BigRational {
        assert!(self.worlds > 0, "an estimate needs observations");
        BigRational::new(BigInt::from(self.a + self.b), BigInt::from(self.worlds))
    }

    /// The sample estimate of the gap `g`.
    pub fn g_hat(&self) -> BigRational {
        assert!(self.worlds > 0, "an estimate needs observations");
        BigRational::new(
            BigInt::from(self.a) - BigInt::from(self.b),
            BigInt::from(self.worlds),
        )
    }

    /// The sample estimate of the tilt `τ`; `None` when no pivot has been
    /// observed (an estimate gap, not a zero claim).
    pub fn tau_hat(&self) -> Option<BigRational> {
        (self.a + self.b > 0).then(|| {
            BigRational::new(
                BigInt::from(self.a) - BigInt::from(self.b),
                BigInt::from(self.a + self.b),
            )
        })
    }

    /// §14.5 — exact rational interval bounds on the information-rate
    /// ESTIMATE `𝓘̂ = q̂·D_{1/2}(τ̂)`; `None` when the estimate is
    /// undefined (no pivots) or degenerate (`τ̂ = 0`).
    pub fn info_rate_hat_bounds(&self, terms: u32) -> Option<(BigRational, BigRational)> {
        let tau = self.tau_hat()?;
        information_rate_bounds(&self.q_hat(), &tau, terms)
    }
}

// ---------------------------------------------------------------------------
// The three wake-up types (§14; CE-A6) — distinct, never collapsed.
// ---------------------------------------------------------------------------

/// §14.1 — response wake-up: a statement about `q₁ − q₀` only. Carries no
/// gap, no value, and no conversion into [`ValueWakeUp`] (the module-doc
/// compile_fail lock).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResponseWakeUp {
    /// Full-enumeration route: `q₁ − q₀` exact.
    Exact {
        /// The exact difference `q₁ − q₀` (may be negative).
        dq: BigRational,
        /// The declared operational tolerance.
        eps_q: BigRational,
        /// `q₁ > q₀` strictly.
        positive: bool,
        /// The §14.1 operational form `q₁ − q₀ > ε_q`.
        exceeds_eps: bool,
    },
    /// Sampled route, established at declared risk.
    SampledEstablished(ResponseEstablished),
    /// Sampled route, not established within the consumed budget — NOT a
    /// statement that response structure is absent.
    SampledOpen {
        /// The declared tolerance the open test was against.
        eps_q: BigRational,
        /// Worlds consumed.
        consumed: u64,
        /// The engine's exact evidence at the cap.
        evidence: BigRational,
    },
}

impl ResponseWakeUp {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            ResponseWakeUp::Exact { .. } => "exact",
            ResponseWakeUp::SampledEstablished(_) => "sampled-established",
            ResponseWakeUp::SampledOpen { .. } => "sampled-open",
        }
    }
}

/// §14.2 — value wake-up: a statement about the signed gap change
/// `g₁ − g₀`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueWakeUp {
    /// Full-enumeration route: `g₁ − g₀` exact.
    Exact {
        /// The exact gap change `g₁ − g₀`.
        gap_change: BigRational,
        /// `g₁ − g₀ ≠ 0` exactly.
        wake: bool,
    },
    /// Sampled route: the §14.6 paired-Z engine settled a direction at
    /// declared risk.
    SampledSettled(DirectionSettled),
    /// Sampled route, direction not settled within the consumed budget —
    /// the honest open state.
    SampledOpen {
        /// Worlds consumed.
        consumed: u64,
        /// Exact evidence for `g₁ − g₀ > 0` at the cap.
        evidence_up: BigRational,
        /// Exact evidence for `g₁ − g₀ < 0` at the cap.
        evidence_down: BigRational,
        /// The sample estimate of `E[Z] = g₁ − g₀`.
        z_mean_hat: BigRational,
    },
}

impl ValueWakeUp {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            ValueWakeUp::Exact { .. } => "exact",
            ValueWakeUp::SampledSettled(_) => "sampled-settled",
            ValueWakeUp::SampledOpen { .. } => "sampled-open",
        }
    }
}

/// The exact frozen-pair selection under one field: the sign of the
/// exact tilt, with an exact tie honestly its own case (`q = 0` is
/// parent §2.4 Case A — equal value is a theorem there, not an estimate).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactPairSelection {
    /// The first-listed candidate wins exactly.
    A,
    /// The second-listed candidate wins exactly.
    B,
    /// Exact tie (`a = b`; includes `q = 0`).
    ExactTie,
}

impl ExactPairSelection {
    /// The mechanical type tag.
    pub fn tag(self) -> &'static str {
        match self {
            ExactPairSelection::A => "a",
            ExactPairSelection::B => "b",
            ExactPairSelection::ExactTie => "exact-tie",
        }
    }
}

/// The sampled decision comparison across the two fields, each leg either
/// δ-settled or honestly open at the consumed budget.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SampledDecisionKind {
    /// Both legs settled, different winners.
    Changed {
        /// σ0's settled decision.
        settle0: PairDecisionSettled,
        /// σ1's settled decision.
        settle1: PairDecisionSettled,
    },
    /// σ0 open at the consumed budget while σ1 settled — the
    /// unresolved-becomes-settled wake of §14.3.
    NewlySettled {
        /// σ1's settled decision.
        settle1: PairDecisionSettled,
    },
    /// σ0 settled while σ1 stayed open — the honest reverse case.
    NewlyOpen {
        /// σ0's settled decision.
        settle0: PairDecisionSettled,
    },
    /// Both legs settled on the same winner.
    SameWinner {
        /// σ0's settled decision.
        settle0: PairDecisionSettled,
        /// σ1's settled decision.
        settle1: PairDecisionSettled,
    },
    /// Neither leg settled within the consumed budget.
    BothOpen,
}

impl SampledDecisionKind {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            SampledDecisionKind::Changed { .. } => "changed",
            SampledDecisionKind::NewlySettled { .. } => "newly-settled",
            SampledDecisionKind::NewlyOpen { .. } => "newly-open",
            SampledDecisionKind::SameWinner { .. } => "same-winner",
            SampledDecisionKind::BothOpen => "both-open",
        }
    }
}

/// §14.3 — decision wake-up: the pair's selected action changes, or an
/// open comparison becomes settled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionWakeUp {
    /// Full-enumeration route: exact selections under both fields.
    Exact {
        /// The exact selection under σ0.
        winner0: ExactPairSelection,
        /// The exact selection under σ1.
        winner1: ExactPairSelection,
        /// The selections differ (a broken exact tie counts).
        changed: bool,
    },
    /// Sampled route: the typed cross-field comparison.
    Sampled(SampledDecisionKind),
}

impl DecisionWakeUp {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            DecisionWakeUp::Exact { .. } => "exact",
            DecisionWakeUp::Sampled(_) => "sampled",
        }
    }
}

// ---------------------------------------------------------------------------
// §14.5 — the information-rate comparison: the ONLY cost coordinate.
// ---------------------------------------------------------------------------

/// The typed verdict of an interval comparison of `𝓘₀` and `𝓘₁`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfoVerdict {
    /// `𝓘₁ > 𝓘₀` (interval-separated, or σ0 exactly in regime 4 with
    /// `𝓘₁ > 0` on the exact route).
    Field1Higher,
    /// `𝓘₀ > 𝓘₁` symmetrically.
    Field0Higher,
    /// The intervals overlap — no ordering claimed.
    Overlapping,
    /// Exact route, both fields in regime 4: both rates exactly zero.
    BothZeroRateExact,
    /// Sampled route: σ0's estimate undefined (no pivots or `τ̂ = 0`) —
    /// an estimate gap, never a zero claim.
    Undefined0,
    /// Sampled route: σ1's estimate undefined.
    Undefined1,
    /// Sampled route: both estimates undefined.
    UndefinedBoth,
}

impl InfoVerdict {
    /// The mechanical type tag.
    pub fn tag(self) -> &'static str {
        match self {
            InfoVerdict::Field1Higher => "field1-higher",
            InfoVerdict::Field0Higher => "field0-higher",
            InfoVerdict::Overlapping => "overlapping",
            InfoVerdict::BothZeroRateExact => "both-zero-rate-exact",
            InfoVerdict::Undefined0 => "undefined-sigma0",
            InfoVerdict::Undefined1 => "undefined-sigma1",
            InfoVerdict::UndefinedBoth => "undefined-both",
        }
    }
}

/// §14.5 — per-field information-rate bounds and their interval verdict.
/// Sampling cost is compared HERE and nowhere else: this type exposes no
/// hardness and no bare-`q` ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InformationComparison {
    /// Exact rational bounds on `𝓘₀` (`None`: regime 4 on the exact
    /// route — a rate of exactly zero; an undefined estimate on the
    /// sampled route).
    pub rate0: Option<(BigRational, BigRational)>,
    /// Exact rational bounds on `𝓘₁`.
    pub rate1: Option<(BigRational, BigRational)>,
    /// The typed verdict.
    pub verdict: InfoVerdict,
}

type RateBounds = Option<(BigRational, BigRational)>;

fn interval_verdict(rate0: &RateBounds, rate1: &RateBounds) -> Option<InfoVerdict> {
    match (rate0, rate1) {
        (Some((lo0, hi0)), Some((lo1, hi1))) => Some(if lo1 > hi0 {
            InfoVerdict::Field1Higher
        } else if lo0 > hi1 {
            InfoVerdict::Field0Higher
        } else {
            InfoVerdict::Overlapping
        }),
        _ => None,
    }
}

/// The exact-route comparison: on a complete fiber, regime 4 (`q = 0` or
/// `τ = 0`) means the rate is EXACTLY zero, so a one-sided `None` orders
/// against a positive interval.
pub fn exact_information_comparison(
    coords0: &PairCoordinates,
    coords1: &PairCoordinates,
    terms: u32,
) -> InformationComparison {
    let rate = |c: &PairCoordinates| {
        c.tau
            .as_ref()
            .and_then(|tau| information_rate_bounds(&c.q, tau, terms))
    };
    let rate0 = rate(coords0);
    let rate1 = rate(coords1);
    let verdict = interval_verdict(&rate0, &rate1).unwrap_or_else(|| match (&rate0, &rate1) {
        (None, None) => InfoVerdict::BothZeroRateExact,
        // An exact regime-4 rate is exactly zero; a defined interval's
        // lower bound is strictly positive (the series' first term is
        // τ²/2 > 0), so the defined side is strictly higher.
        (None, Some(_)) => InfoVerdict::Field1Higher,
        (Some(_), None) => InfoVerdict::Field0Higher,
        (Some(_), Some(_)) => unreachable!("ordered above"),
    });
    InformationComparison {
        rate0,
        rate1,
        verdict,
    }
}

/// The sampled-route comparison: an undefined side is an ESTIMATE gap,
/// typed as such — never read as a zero rate.
pub fn sampled_information_comparison(
    coords0: &SampledFieldCoordinates,
    coords1: &SampledFieldCoordinates,
    terms: u32,
) -> InformationComparison {
    let rate0 = coords0.info_rate_hat_bounds(terms);
    let rate1 = coords1.info_rate_hat_bounds(terms);
    let verdict = interval_verdict(&rate0, &rate1).unwrap_or_else(|| match (&rate0, &rate1) {
        (None, None) => InfoVerdict::UndefinedBoth,
        (None, Some(_)) => InfoVerdict::Undefined0,
        (Some(_), None) => InfoVerdict::Undefined1,
        (Some(_), Some(_)) => unreachable!("ordered above"),
    });
    InformationComparison {
        rate0,
        rate1,
        verdict,
    }
}

// ---------------------------------------------------------------------------
// The paired tally shared by both routes (derived views of aligned rows).
// ---------------------------------------------------------------------------

/// The paired per-world tally of two candidates' coupled rows: per-field
/// pivotal counts and the `Z` histogram. A derived view — recomputed from
/// the audit rows, never stored beside them as a second authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairedTally {
    /// σ0: worlds where the first candidate alone succeeds.
    pub a0: u64,
    /// σ0: worlds where the second candidate alone succeeds.
    pub b0: u64,
    /// σ1: worlds where the first candidate alone succeeds.
    pub a1: u64,
    /// σ1: worlds where the second candidate alone succeeds.
    pub b1: u64,
    /// Histogram of `Z = Y⁽¹⁾ − Y⁽⁰⁾` over `{−2, −1, 0, 1, 2}`,
    /// ascending.
    pub z_counts: [u64; 5],
    /// Worlds tallied.
    pub worlds: u64,
}

impl PairedTally {
    /// Fold two candidates' aligned rows (same worlds, same order —
    /// asserted per row).
    pub fn from_rows(rows_a: &[WorldRow], rows_b: &[WorldRow]) -> PairedTally {
        assert_eq!(rows_a.len(), rows_b.len(), "one world set, one order");
        let mut tally = PairedTally {
            a0: 0,
            b0: 0,
            a1: 0,
            b1: 0,
            z_counts: [0; 5],
            worlds: u64::try_from(rows_a.len()).expect("a world set fits u64"),
        };
        for (ra, rb) in rows_a.iter().zip(rows_b) {
            assert_eq!(ra.world, rb.world, "paired rows share their world");
            match (ra.u0, rb.u0) {
                (true, false) => tally.a0 += 1,
                (false, true) => tally.b0 += 1,
                _ => {}
            }
            match (ra.u1, rb.u1) {
                (true, false) => tally.a1 += 1,
                (false, true) => tally.b1 += 1,
                _ => {}
            }
            let y0 = i8::from(ra.u0) - i8::from(rb.u0);
            let y1 = i8::from(ra.u1) - i8::from(rb.u1);
            let z = y1 - y0;
            tally.z_counts[usize::try_from(z + 2).expect("z + 2 in 0..5")] += 1;
        }
        tally
    }

    /// The exact mean of `Z` over the tallied worlds — `g₁ − g₀` on the
    /// exact route, its sample estimate on the sampled route.
    pub fn z_mean(&self) -> BigRational {
        assert!(self.worlds > 0, "a mean needs worlds");
        let mut numerator = BigInt::zero();
        for (slot, count) in self.z_counts.iter().enumerate() {
            let z = i64::try_from(slot).expect("slot < 5") - 2;
            numerator += BigInt::from(z) * BigInt::from(*count);
        }
        BigRational::new(numerator, BigInt::from(self.worlds))
    }

    /// The paired tally agrees with the online decision probes count for
    /// count (two derivations of the same numbers).
    pub fn matches_probes(
        &self,
        decision0: &PairDecisionProbe,
        decision1: &PairDecisionProbe,
    ) -> bool {
        decision0.counts() == (self.a0, self.b0) && decision1.counts() == (self.a1, self.b1)
    }
}

// ---------------------------------------------------------------------------
// The exact route (§14.7's enumeration half).
// ---------------------------------------------------------------------------

/// The witness that a detection record's coordinates came from complete
/// fiber enumeration. No public constructor: the only producer is
/// [`exact_paired_detection`], which asserts `ExactFiber` domains on both
/// exposures (the module-doc compile_fail lock).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactFiberWitness {
    _private: (),
}

/// One frozen action pair's EXACT paired detection record: coordinates
/// under both fields from full enumeration, the `Z` histogram, the three
/// wake-ups at the exact tier, the §14.5 comparison, and the per-candidate
/// cancellation ladders and first-split aggregates (the mechanism notes).
/// Everything is exact FOR THE FROZEN PAIR under the declared fields —
/// never an optimized-root statement (O18).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactPairedDetection {
    /// The enumeration witness (unforgeable outside this module).
    pub witness: ExactFiberWitness,
    /// `FieldId(σ0)` — every record names both field identities.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The first candidate's pinned root action.
    pub tile_a: Domino,
    /// The second candidate's pinned root action.
    pub tile_b: Domino,
    /// The first candidate's content address.
    pub policy_a: PolicyId,
    /// The second candidate's content address.
    pub policy_b: PolicyId,
    /// Fiber size.
    pub fiber: u64,
    /// Exact pair coordinates under σ0.
    pub coords0: PairCoordinates,
    /// Exact pair coordinates under σ1.
    pub coords1: PairCoordinates,
    /// The exact `Z` histogram (§14.6's columns, enumerated).
    pub z_counts: [u64; 5],
    /// §14.1 at the exact tier.
    pub response: ResponseWakeUp,
    /// §14.2 at the exact tier.
    pub value: ValueWakeUp,
    /// §14.3 at the exact tier.
    pub decision: DecisionWakeUp,
    /// §14.5 — the only cost comparison.
    pub information: InformationComparison,
    /// The first candidate's fixed-policy cancellation ladder.
    pub ladder_a: CancellationLadder,
    /// The second candidate's.
    pub ladder_b: CancellationLadder,
    /// The first candidate's first-split aggregate (mechanism notes).
    pub splits_a: SplitAggregate,
    /// The second candidate's.
    pub splits_b: SplitAggregate,
    /// The declared response tolerance the exact wake-up was read
    /// against.
    pub eps_q: BigRational,
}

fn exact_selection(coords: &PairCoordinates) -> ExactPairSelection {
    use core::cmp::Ordering;
    match coords.a.cmp(&coords.b) {
        Ordering::Greater => ExactPairSelection::A,
        Ordering::Less => ExactPairSelection::B,
        Ordering::Equal => ExactPairSelection::ExactTie,
    }
}

/// The exact-route producer: derive one pair's detection record from the
/// two candidates' complete-fiber exposures (computed once per candidate
/// by `solver::exposure::frozen_policy_exposure` and shared across the
/// candidate's pairs). Cross-checks, asserted not assumed:
///
/// - both exposures cover the SAME complete fiber under the SAME (σ0, σ1);
/// - the `Z`-histogram mean equals `g₁ − g₀` from the per-field
///   coordinates equals the `field_swap::pair_lift` Λ of the two ladders
///   (three independent derivations of one number);
/// - parent §2.4 Case A: `q = 0` forces `g = 0` under each field.
pub fn exact_paired_detection(
    exposure_a: &FrozenPolicyExposure,
    exposure_b: &FrozenPolicyExposure,
    tile_a: Domino,
    tile_b: Domino,
    eps_q: &BigRational,
) -> ExactPairedDetection {
    assert_ne!(
        tile_a, tile_b,
        "a detection pair compares two distinct actions"
    );
    assert_eq!(exposure_a.root_id, exposure_b.root_id, "one root");
    assert_eq!(exposure_a.field0, exposure_b.field0, "one σ0");
    assert_eq!(exposure_a.field1, exposure_b.field1, "one σ1");
    assert_eq!(
        exposure_a.domain,
        WorldDomain::ExactFiber,
        "the exact route requires complete enumeration (§14.7)"
    );
    assert_eq!(
        exposure_b.domain,
        WorldDomain::ExactFiber,
        "the exact route requires complete enumeration (§14.7)"
    );
    let tally = PairedTally::from_rows(&exposure_a.rows, &exposure_b.rows);
    let u = |rows: &[WorldRow], field1: bool| -> Vec<bool> {
        rows.iter()
            .map(|r| if field1 { r.u1 } else { r.u0 })
            .collect()
    };
    let coords0 = pair_coordinates(&u(&exposure_a.rows, false), &u(&exposure_b.rows, false));
    let coords1 = pair_coordinates(&u(&exposure_a.rows, true), &u(&exposure_b.rows, true));
    // The tally and the coordinate fold agree count for count (two
    // derivations of the same numbers).
    assert_eq!(
        (u128::from(tally.a0), u128::from(tally.b0)),
        (coords0.a, coords0.b),
        "the paired tally re-derives the σ0 coordinates"
    );
    assert_eq!(
        (u128::from(tally.a1), u128::from(tally.b1)),
        (coords1.a, coords1.b),
        "the paired tally re-derives the σ1 coordinates"
    );
    // Parent §2.4 Case A, asserted: q = 0 forces g = 0.
    for coords in [&coords0, &coords1] {
        if coords.q.is_zero() {
            assert!(coords.g.is_zero(), "Case A: q = 0 forces g = 0");
        }
    }
    let gap_change = &coords1.g - &coords0.g;
    assert_eq!(
        tally.z_mean(),
        gap_change,
        "§14.6: E[Z] = g₁ − g₀, exactly, over the enumerated fiber"
    );
    let ladder_a = CancellationLadder::from_exposure(exposure_a);
    let ladder_b = CancellationLadder::from_exposure(exposure_b);
    let lift = pair_lift(&ladder_a, &ladder_b);
    assert_eq!(
        lift.lambda, gap_change,
        "the pair lift Λ re-derives the gap change from an independent producer"
    );
    let dq = &coords1.q - &coords0.q;
    let response = ResponseWakeUp::Exact {
        positive: dq > BigRational::zero(),
        exceeds_eps: dq > *eps_q,
        dq,
        eps_q: eps_q.clone(),
    };
    let value = ValueWakeUp::Exact {
        wake: !gap_change.is_zero(),
        gap_change,
    };
    let winner0 = exact_selection(&coords0);
    let winner1 = exact_selection(&coords1);
    let decision = DecisionWakeUp::Exact {
        winner0,
        winner1,
        changed: winner0 != winner1,
    };
    let information = exact_information_comparison(&coords0, &coords1, INFO_TERMS);
    ExactPairedDetection {
        witness: ExactFiberWitness { _private: () },
        field0: exposure_a.field0,
        field1: exposure_a.field1,
        root_id: exposure_a.root_id,
        tile_a,
        tile_b,
        policy_a: exposure_a.policy,
        policy_b: exposure_b.policy,
        fiber: exposure_a.worlds,
        coords0,
        coords1,
        z_counts: tally.z_counts,
        response,
        value,
        decision,
        information,
        ladder_a,
        ladder_b,
        splits_a: SplitAggregate::from_exposure(exposure_a),
        splits_b: SplitAggregate::from_exposure(exposure_b),
        eps_q: eps_q.clone(),
    }
}

// ---------------------------------------------------------------------------
// The sampled route: dig-until-settled on one common paired stream.
// ---------------------------------------------------------------------------

/// The declared risk plan of one sampled detection: every scope travels
/// with its δ (`solver::evidence::ScopedDelta` — a δ without its scope is
/// meaningless), the response tolerance, and the one betting mixture
/// shared by the bounded-mean engines.
pub struct DetectionRiskPlan {
    /// The declared response/practical-zero tolerance `ε_q`.
    pub eps_q: BigRational,
    /// Per-FIELD pair-decision risk (spent once per field, scope suffixed
    /// `:sigma0` / `:sigma1`).
    pub delta_decision: ScopedDelta,
    /// The value-direction risk (split across the two one-sided engines).
    pub delta_value: ScopedDelta,
    /// The response-establishment risk.
    pub delta_response: ScopedDelta,
    /// The σ0 practical-zero risk.
    pub delta_practical_zero: ScopedDelta,
    /// The declared `(weight, λ)` betting mixture.
    pub mixture: Vec<(BigRational, BigRational)>,
}

/// One sampled paired detection's inputs: the frozen pair, the declared
/// (σ0, σ1) models, one epoch (one common indexed stream — both fields
/// and both candidates see the same ordered worlds), a world cap that is
/// a resource limit and never a settlement rule, and the risk plan.
pub struct SampledDetectionSpec<'a> {
    /// The canonical root.
    pub root: &'a CanonicalRoot,
    /// The root position.
    pub position: &'a RootPosition,
    /// The first candidate's pinned root action.
    pub tile_a: Domino,
    /// The second candidate's pinned root action.
    pub tile_b: Domino,
    /// The first frozen candidate.
    pub policy_a: &'a FrozenPolicy,
    /// The second frozen candidate.
    pub policy_b: &'a FrozenPolicy,
    /// σ0 — the modeled level-0 field.
    pub field0: &'a FieldModel,
    /// σ1 — the level-1 field at its declared freeze.
    pub field1: &'a FieldModel,
    /// The common evidence-stream epoch.
    pub epoch: u64,
    /// The resource cap in raw worlds.
    pub world_cap: u64,
    /// The declared risks, tolerance, and mixture.
    pub plan: &'a DetectionRiskPlan,
}

/// One frozen action pair's SAMPLED paired detection record over a
/// declared stream prefix: per-field coordinate estimates, the `Z`
/// histogram, the three wake-ups at the sampled tier, the §14.5
/// comparison of estimates, the σ0 practical-zero state, and the two
/// candidates' streamed exposures (audit rows; ladders and split
/// aggregates are derived views of them). Estimates throughout; open
/// results are successful outputs.
pub struct SampledPairedDetection {
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity.
    pub root_id: u64,
    /// The common stream epoch.
    pub epoch: u64,
    /// The first candidate's pinned root action.
    pub tile_a: Domino,
    /// The second candidate's pinned root action.
    pub tile_b: Domino,
    /// The first candidate's content address.
    pub policy_a: PolicyId,
    /// The second candidate's content address.
    pub policy_b: PolicyId,
    /// Worlds consumed (≤ the declared cap).
    pub consumed: u64,
    /// The declared cap.
    pub world_cap: u64,
    /// Every typed question settled before the cap (the dig-until-settled
    /// loop stopped early).
    pub stopped_early: bool,
    /// σ0 coordinate estimates.
    pub coords0: SampledFieldCoordinates,
    /// σ1 coordinate estimates.
    pub coords1: SampledFieldCoordinates,
    /// The `Z` histogram over the consumed prefix.
    pub z_counts: [u64; 5],
    /// §14.7's sampled half: the σ0 practical-zero witness, if its
    /// engine crossed.
    pub q0_practical: Option<PracticalZero>,
    /// The σ0 practical-zero engine's evidence at the end (recorded
    /// whether or not it crossed).
    pub q0_practical_evidence: BigRational,
    /// §14.1 at the sampled tier.
    pub response: ResponseWakeUp,
    /// §14.2 at the sampled tier.
    pub value: ValueWakeUp,
    /// §14.3 at the sampled tier.
    pub decision: DecisionWakeUp,
    /// §14.5 — estimates compared as intervals.
    pub information: InformationComparison,
    /// The first candidate's streamed exposure (audit rows).
    pub exposure_a: FrozenPolicyExposure,
    /// The second candidate's streamed exposure.
    pub exposure_b: FrozenPolicyExposure,
    /// The first candidate's first-split aggregate (mechanism notes).
    pub splits_a: SplitAggregate,
    /// The second candidate's.
    pub splits_b: SplitAggregate,
}

/// The sampled-route producer: the dig-until-settled outer loop on ONE
/// common indexed stream. Each world is evaluated by coupled replay for
/// BOTH candidates — four terminals per world, pairing every comparison
/// on the same world (§14.6). The loop stops early only when every typed
/// question has crossed its threshold; otherwise it runs to the declared
/// cap and reports each question's state honestly. The engines are
/// deterministic folds of the row sequence: same epoch, same record.
pub fn sampled_paired_detection(spec: &SampledDetectionSpec<'_>) -> SampledPairedDetection {
    assert_ne!(
        spec.tile_a, spec.tile_b,
        "a detection pair compares two distinct actions"
    );
    assert!(
        spec.world_cap >= 1,
        "a declared cap buys at least one world"
    );
    let viewer = spec.root.kernel().viewer();
    let root_id = root_identity(spec.root, spec.position);
    let plan = spec.plan;
    let scoped = |base: &ScopedDelta, suffix: &str| {
        ScopedDelta::new(format!("{}{suffix}", base.scope()), base.delta().clone())
    };
    let mut decision0 = PairDecisionProbe::new(scoped(&plan.delta_decision, ":sigma0"));
    let mut decision1 = PairDecisionProbe::new(scoped(&plan.delta_decision, ":sigma1"));
    let mut direction = DirectionProbe::new(plan.delta_value.clone(), &plan.mixture)
        .expect("a declared value mixture is lawful");
    let mut response = ResponseProbe::new(
        plan.eps_q.clone(),
        plan.delta_response.clone(),
        &plan.mixture,
    )
    .expect("a declared response mixture is lawful");
    let mut practical = PracticalZeroProbe::new(
        plan.eps_q.clone(),
        plan.delta_practical_zero.clone(),
        &plan.mixture,
    )
    .expect("a declared practical-zero mixture is lawful");
    let mut rows_a: Vec<WorldRow> = Vec::new();
    let mut rows_b: Vec<WorldRow> = Vec::new();
    let mut stopped_early = false;
    for index in 0..spec.world_cap {
        let world = spec.root.world_at(root_id, spec.epoch, index);
        let outcome_a = coupled_replay(
            spec.position,
            viewer,
            &world,
            spec.policy_a,
            spec.field0,
            spec.field1,
        );
        let outcome_b = coupled_replay(
            spec.position,
            viewer,
            &world,
            spec.policy_b,
            spec.field0,
            spec.field1,
        );
        let y0 = i8::from(outcome_a.u0) - i8::from(outcome_b.u0);
        let y1 = i8::from(outcome_a.u1) - i8::from(outcome_b.u1);
        decision0.observe(y0);
        decision1.observe(y1);
        direction.observe(y1 - y0);
        response.observe(y1 != 0, y0 != 0);
        practical.observe(y0 != 0);
        rows_a.push(WorldRow {
            index,
            world: world_id(&world),
            u0: outcome_a.u0,
            u1: outcome_a.u1,
            split: outcome_a.split,
        });
        rows_b.push(WorldRow {
            index,
            world: world_id(&world),
            u0: outcome_b.u0,
            u1: outcome_b.u1,
            split: outcome_b.split,
        });
        if decision0.settled().is_some()
            && decision1.settled().is_some()
            && direction.settled().is_some()
            && response.established().is_some()
            && practical.witness().is_some()
        {
            stopped_early = true;
            break;
        }
    }
    let consumed = u64::try_from(rows_a.len()).expect("a consumed prefix fits u64");
    let tally = PairedTally::from_rows(&rows_a, &rows_b);
    assert!(
        tally.matches_probes(&decision0, &decision1),
        "the paired tally re-derives the online decision counts"
    );
    let coords0 = SampledFieldCoordinates {
        a: tally.a0,
        b: tally.b0,
        worlds: consumed,
    };
    let coords1 = SampledFieldCoordinates {
        a: tally.a1,
        b: tally.b1,
        worlds: consumed,
    };
    let domain = WorldDomain::StreamPrefix {
        epoch: spec.epoch,
        worlds: consumed,
    };
    let exposure_of = |policy: &FrozenPolicy, rows: Vec<WorldRow>| {
        let exposed =
            u64::try_from(rows.iter().filter(|r| r.split.is_some()).count()).expect("fits");
        let plus = u64::try_from(rows.iter().filter(|r| r.u1 && !r.u0).count()).expect("fits");
        let minus = u64::try_from(rows.iter().filter(|r| !r.u1 && r.u0).count()).expect("fits");
        FrozenPolicyExposure {
            policy: policy.policy_id(),
            field0: spec.field0.field_id(),
            field1: spec.field1.field_id(),
            root_id,
            domain: domain.clone(),
            worlds: consumed,
            exposed,
            corrections_plus: plus,
            corrections_minus: minus,
            rows,
        }
    };
    let exposure_a = exposure_of(spec.policy_a, rows_a);
    let exposure_b = exposure_of(spec.policy_b, rows_b);
    // The ladder constructors re-run the L2-T1 pointwise bound and the
    // census assertions on the streamed rows; the ladders themselves stay
    // derived views of the carried exposures.
    let _ = CancellationLadder::from_exposure(&exposure_a);
    let _ = CancellationLadder::from_exposure(&exposure_b);
    let response_record = match response.established() {
        Some(established) => ResponseWakeUp::SampledEstablished(established.clone()),
        None => ResponseWakeUp::SampledOpen {
            eps_q: plan.eps_q.clone(),
            consumed,
            evidence: response.evidence(),
        },
    };
    let value_record = match direction.settled() {
        Some(settled) => ValueWakeUp::SampledSettled(settled.clone()),
        None => ValueWakeUp::SampledOpen {
            consumed,
            evidence_up: direction.evidence_up(),
            evidence_down: direction.evidence_down(),
            z_mean_hat: tally.z_mean(),
        },
    };
    let decision_record =
        DecisionWakeUp::Sampled(match (decision0.settled(), decision1.settled()) {
            (Some(s0), Some(s1)) if s0.winner() != s1.winner() => SampledDecisionKind::Changed {
                settle0: s0.clone(),
                settle1: s1.clone(),
            },
            (Some(s0), Some(s1)) => SampledDecisionKind::SameWinner {
                settle0: s0.clone(),
                settle1: s1.clone(),
            },
            (None, Some(s1)) => SampledDecisionKind::NewlySettled {
                settle1: s1.clone(),
            },
            (Some(s0), None) => SampledDecisionKind::NewlyOpen {
                settle0: s0.clone(),
            },
            (None, None) => SampledDecisionKind::BothOpen,
        });
    let information = sampled_information_comparison(&coords0, &coords1, INFO_TERMS);
    let splits_a = SplitAggregate::from_exposure(&exposure_a);
    let splits_b = SplitAggregate::from_exposure(&exposure_b);
    SampledPairedDetection {
        field0: spec.field0.field_id(),
        field1: spec.field1.field_id(),
        root_id,
        epoch: spec.epoch,
        tile_a: spec.tile_a,
        tile_b: spec.tile_b,
        policy_a: spec.policy_a.policy_id(),
        policy_b: spec.policy_b.policy_id(),
        consumed,
        world_cap: spec.world_cap,
        stopped_early,
        coords0,
        coords1,
        z_counts: tally.z_counts,
        q0_practical: practical.witness().cloned(),
        q0_practical_evidence: practical.evidence(),
        response: response_record,
        value: value_record,
        decision: decision_record,
        information,
        exposure_a,
        exposure_b,
        splits_a,
        splits_b,
    }
}

// ---------------------------------------------------------------------------
// Typed refusals: an unaffordable root is a record, not a degraded number.
// ---------------------------------------------------------------------------

/// A typed refusal: the declared budget cannot honestly buy this root's
/// paired evaluation under the declared fields. Refusal paths are part of
/// correctness — the alternative is a degraded number, and there are no
/// degraded numbers here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectionRefusal {
    /// The refused root.
    pub root_id: u64,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The route that was refused (`"exact"` or `"sampled"`).
    pub route: &'static str,
    /// The stated reason, with the declared numbers.
    pub reason: String,
}

/// Refuse a sampled root whose declared cap is below the declared minimum
/// honest budget.
pub fn refuse_sampled_if_underfunded(
    root_id: u64,
    field0: FieldId,
    field1: FieldId,
    world_cap: u64,
    min_worlds: u64,
) -> Result<(), DetectionRefusal> {
    if world_cap < min_worlds {
        return Err(DetectionRefusal {
            root_id,
            field0,
            field1,
            route: "sampled",
            reason: format!(
                "declared world cap {world_cap} is below the declared minimum \
                 honest budget {min_worlds}"
            ),
        });
    }
    Ok(())
}

/// Refuse an exact root whose fiber exceeds the declared enumeration
/// budget (the sampled route or a typed refusal are the honest options; a
/// partial enumeration presented as exact is neither).
pub fn refuse_exact_if_oversized(
    root_id: u64,
    field0: FieldId,
    field1: FieldId,
    fiber: u128,
    enumeration_budget: u128,
) -> Result<(), DetectionRefusal> {
    if fiber > enumeration_budget {
        return Err(DetectionRefusal {
            root_id,
            field0,
            field1,
            route: "exact",
            reason: format!(
                "fiber {fiber} exceeds the declared enumeration budget \
                 {enumeration_budget}"
            ),
        });
    }
    Ok(())
}
