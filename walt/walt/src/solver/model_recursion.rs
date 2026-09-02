//! `solver::model_recursion` — MB1: the model-belief recursion joins
//! the solver. A SIBLING of [`crate::solver::model_belief`] (MB0), which
//! it consumes and never forks: every mass, posterior and value below is
//! produced by MB0's bundle walk, and MB0's own module holds the walk,
//! the ledger and the typed refusals (MB1 items 3 and 4 landed there
//! because that is where the walk lives).
//!
//! EXPLORATORY tier. Mathematical source:
//! `walt/math/model_belief_base_player_v0.1.md` §§16–23 (response
//! vectors, the convex envelope, Theorem 18.1's separated upper, §19's
//! fusion price, §21's column-and-cut form, §22's gap diagnosis, §23's
//! reuse-by-dot-product) and §§29–33 (typed unresolved fields, field
//! cylinders, §32's merge-before-max, §33's disagreement frontier),
//! under rulings MB-A1..A8 (`walt/CENSUS-RULINGS.md`); brief
//! `walt/briefs/BRIEF-MB1.md`; status ledger `walt/FACTOR-BELIEF.md`.
//! Gates: `walt/walt/tests/solver_model_belief_recursion.rs`.
//!
//! WHAT "JOINS THE SOLVER" MEANS HERE. Three couplings, in the order
//! they matter:
//!
//! 1. THE POSTERIOR IS CARRIED, NOT RECOMPUTED ([`PosteriorTrace`]).
//!    Descending a public line maintains one [`ModelBelief`] whose
//!    profiles condition themselves as the line is walked (Theorem
//!    12.1 at every observed action, merge-before-max at every hidden
//!    node). Every reported posterior is a DERIVED VIEW of that one
//!    object — [`ModelBelief::seat_type_marginals`] and
//!    [`ModelBelief::posterior_profile_masses`] — so there is no second
//!    authority to drift from. The trace records what was walked; it
//!    stores no belief of its own.
//!
//! 2. THE VALUES BECOME FACTS ([`ModelBeliefProducer`]). The §49 proof
//!    state gets the mixture's exact `Q_a` as a matched
//!    executable-lower/deterministic-upper pair and `U^sep_a` as the
//!    Theorem 18.1 upper — under a proof-state identity whose
//!    `field_id` is the MIXTURE's, never any component field's (item 7).
//!
//! 3. THE BELIEF IS REPRICED, NOT RE-WALKED ([`ResponseEnvelope`]).
//!    §16 makes fixed-policy value linear in ν and §21 makes the exact
//!    response the upper envelope of finitely many such linear forms.
//!    An envelope of response vectors therefore answers `Q(ν)` for a
//!    whole sweep of model beliefs by dot products, walking once per
//!    FACET rather than once per belief.
//!
//! THE FIELD-IDENTITY FENCE (item 7, U0's flag; `walt/briefs/U0-REPORT.md`
//! "For MB1 / the model-belief program"). God-tightness and every
//! doom-derived bound is field-SPECIFIC under SC-A7's strictest class:
//! each is an equality against a doom upper computed under ONE declared
//! σ0. A model belief is not that field. Its profile fields are
//! `profile:<type ids>` objects, and the mixture over them is a third
//! thing again, so a fact established under σ0 has, a priori, no meaning
//! inside the model-space recursion.
//!
//! The fence is structural where it can be. [`CoupledFact`] holds
//! private members and has no public constructor: the ONLY way to
//! obtain one is [`couple_fixed_field_fact`], which either returns a
//! coupling that names its own justification or a typed
//! [`CouplingRefusal`]. A consumer that wants a fixed-field fact inside
//! the model-space recursion must therefore take a [`CoupledFact`], and
//! a bare [`Fact`] simply does not type-check there. Where structure
//! cannot reach — the §49 store, which accepts any [`Fact`] under a
//! matching identity — the fence is the identity itself: a model-belief
//! proof state's `field_id` is [`mixture_field_id`], so a σ0-authored
//! fact is rejected `IdentityMismatch` by machinery that already
//! existed. Both halves are gated, and they are gated even though MB1
//! transports nothing: the gate is what keeps the FIRST transport
//! honest, which is the whole point of building it before there is one.
//!
//! The one coupling this slice can actually discharge is the degenerate
//! one, and it is not free. A point-mass model belief δ_θ is
//! extensionally the fixed field θ dispatches to, but "extensionally"
//! is a claim about values and not an identity of objects, so
//! [`FieldCoupling::PointMassParity`] carries a re-run WITNESS: both
//! authorities priced at the same state, both exact pairs recorded in
//! the coupling. Where the fixed authority cannot run — the σ1 mind's
//! refusal set, pinned by MB0's G2 and named by the σ1-repair slice —
//! the coupling refuses, which is the correct answer and not a gap.
//!
//! THE ν-INVARIANCE COROLLARY (§19, and the reason MB1's earlier-root
//! probe looks where it does). Theorem 19.1 says `Φ_a(ν) = 0` iff one
//! lawful policy attains `q_a(θ)` for every θ in the support of ν. Take
//! ν with FULL support and `Φ_a(ν) = 0`: the witness ρ* attains `q_a`
//! at every θ whatsoever, so for ANY other belief ν′,
//! `Q_a(ν′) ≥ ⟨ν′, v_{ρ*}⟩ = Σ ν′(θ) q_a(θ) = U^sep_a(ν′)`, and
//! Theorem 18.1 gives the reverse. Hence `Φ_a(ν′) = 0` for every ν′.
//! A zero fusion price measured at one full-support belief is therefore
//! a zero fusion price at EVERY belief over the same types, and no
//! amount of re-weighting can find a strict specimen at a coordinate
//! that already censused zero. [`CommonOptimizer`] is that witness made
//! explicit, and the consequence for the search is sharp: strict fusion
//! prices are found by moving to new ROOTS, never by moving ν.
//!
//! VACUITY IS TYPED APART, as U0 typed degenerate God-tightness apart
//! (`walt/briefs/U0-REPORT.md`). Where `U^sep_a` is 0 or the whole
//! fiber mass, every lawful policy attains every point-mass optimum for
//! the arithmetically trivial reason that there is nothing to choose
//! between, and `Φ_a = 0` carries no information about the price of
//! model blindness. [`ActionCoordinate::substantive`] is false there,
//! and [`RootModelCensus::substantive_zero_prices`] counts only the
//! coordinates where the zero says something.

use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::factor_belief::ExactCoverOracle;
use crate::solver::model_belief::{
    BehaviorTypeId, MixturePolicy, MixtureRefusal, MixtureStats, ModelBelief,
};
use crate::solver::policy::{content_digest, Canon};
use crate::solver::proof_state::{
    BoundFact, Fact, ProofProducer, ProofState, ProofTag, SemanticsIdentity,
};

// ---------------------------------------------------------------------------
// Field identity and the transport fence (item 7).
// ---------------------------------------------------------------------------

/// The field identity of an object in the model-belief program (item 7,
/// SC-A7's typing). Exactly two shapes exist, and the distinction is the
/// fence: a fact carrying [`ModelFieldId::Fixed`] was established
/// against ONE declared field, and a model-space consumer is a
/// [`ModelFieldId::Mixture`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelFieldId {
    /// One declared field's identity string — what
    /// [`crate::solver::factor_belief::FactorBelief::field_id`] and
    /// [`SemanticsIdentity::field_id`] carry today.
    Fixed(String),
    /// The model mixture over a declared profile set: the content
    /// address of the ordered (profile label, prior weight) list
    /// together with the prior denominator. A point-mass mixture is
    /// still a Mixture — degeneracy is a fact about the weights, never
    /// a change of kind, so nothing can slip through by being small.
    Mixture {
        id: String,
        profiles: Vec<(String, u128)>,
        prior_denominator: u128,
    },
}

impl ModelFieldId {
    /// The identity string, in the form [`SemanticsIdentity`] stores.
    pub fn as_str(&self) -> &str {
        match self {
            ModelFieldId::Fixed(s) => s,
            ModelFieldId::Mixture { id, .. } => id,
        }
    }

    /// How many profiles the mixture spans; one for a fixed field.
    pub fn profile_count(&self) -> usize {
        match self {
            ModelFieldId::Fixed(_) => 1,
            ModelFieldId::Mixture { profiles, .. } => profiles.len(),
        }
    }
}

impl fmt::Display for ModelFieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The mixture field identity of a model belief: a content address over
/// the ordered (profile label, prior weight) list and the prior
/// denominator. Deliberately NOT equal to any component field's
/// identity, and deliberately not equal for two mixtures that differ
/// only in weights — a reweighted mixture is a different field for
/// transport purposes, because the values established under it are
/// different values.
pub fn mixture_field_id(model: &ModelBelief) -> ModelFieldId {
    let profiles: Vec<(String, u128)> = model
        .profiles()
        .iter()
        .map(|e| (e.field().id().to_string(), e.weight()))
        .collect();
    let mut canon = Canon::new("walt-model-mixture-field-v1");
    canon.len(profiles.len());
    for (label, weight) in &profiles {
        canon.str_field(0x01, label);
        canon.tag(0x02);
        canon.u64s_field(0x03, &split_u128(*weight));
    }
    canon.tag(0x04);
    canon.u64s_field(0x05, &split_u128(model.prior_denominator()));
    let digest = content_digest(&canon.finish());
    let mut hex = String::new();
    for b in &digest[..16] {
        hex.push_str(&format!("{b:02x}"));
    }
    ModelFieldId::Mixture {
        id: format!("model-mixture:{hex}"),
        profiles,
        prior_denominator: model.prior_denominator(),
    }
}

/// A `u128` as two big-endian `u64` halves, so [`Canon`]'s integer
/// writers cover the whole range without a new primitive.
fn split_u128(v: u128) -> [u64; 2] {
    [
        u64::try_from(v >> 64).expect("the high half of a u128 is a u64"),
        u64::try_from(v & u128::from(u64::MAX)).expect("the low half of a u128 is a u64"),
    ]
}

/// The exact pair `(mass, total)` two authorities agreed on when a
/// point-mass coupling was discharged. Both sides are recorded, not one
/// side and an assertion, so a reader can see what was compared.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointMassWitness {
    /// The value the fixed-field authority produced.
    pub fixed_side: (u128, u128),
    /// The value the point-mass model belief produced.
    pub model_side: (u128, u128),
    /// The behavior type every hidden seat carries in the point mass.
    pub behavior: BehaviorTypeId,
}

/// Why a fact established under one field identity may be read under
/// another (item 7). Every variant names its own justification; there is
/// no `Assumed` variant, and adding one would be a mathematical claim
/// rather than a code change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldCoupling {
    /// The two identities are the same string: nothing crosses.
    Identical,
    /// The target is a point-mass model belief over a single behavior
    /// type whose parent field is the source, AND the two authorities
    /// were re-run at the coupling state and agreed exactly.
    PointMassParity(PointMassWitness),
}

/// Why a coupling was refused. A refusal is the honest answer, not a
/// failure: it says the boundary was not crossed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CouplingRefusal {
    /// The target model belief spans more than one live profile. A
    /// fixed-field fact has no meaning over a nondegenerate mixture
    /// without an explicit coupling proof, and this slice constructs
    /// none.
    MixtureTarget {
        source: String,
        target: String,
        live_profiles: usize,
    },
    /// The target IS a point mass, but not over the source field: the
    /// declared parent field of its behavior type is a different
    /// object.
    ParentFieldMismatch {
        source: String,
        target_parent: String,
    },
    /// The target point mass does not assign the SAME type to every
    /// hidden seat, so there is no single fixed field it collapses to.
    MixedSeatTypes { seats: usize },
    /// Both authorities ran and disagreed. The coupling cannot be
    /// discharged and the disagreement is reported exactly.
    ParityDisagreement {
        fixed_side: (u128, u128),
        model_side: (u128, u128),
    },
    /// The point-mass parity witness could not be produced because the
    /// model side refused its own read budget. Nothing is claimed.
    WitnessRefused(MixtureRefusal),
}

impl fmt::Display for CouplingRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CouplingRefusal::MixtureTarget {
                source,
                target,
                live_profiles,
            } => write!(
                f,
                "field coupling refused: {source} is fixed-field, {target} spans \
                 {live_profiles} live profiles"
            ),
            CouplingRefusal::ParentFieldMismatch {
                source,
                target_parent,
            } => write!(
                f,
                "field coupling refused: source {source} is not the target type's \
                 parent field {target_parent}"
            ),
            CouplingRefusal::MixedSeatTypes { seats } => write!(
                f,
                "field coupling refused: the point mass assigns {seats} distinct \
                 types across hidden seats"
            ),
            CouplingRefusal::ParityDisagreement {
                fixed_side,
                model_side,
            } => write!(
                f,
                "field coupling refused: authorities disagree — fixed {}/{} vs model {}/{}",
                fixed_side.0, fixed_side.1, model_side.0, model_side.1
            ),
            CouplingRefusal::WitnessRefused(r) => {
                write!(f, "field coupling refused: witness unavailable — {r}")
            }
        }
    }
}

/// A fact that has crossed a field-identity boundary under a named
/// coupling (item 7). PRIVATE MEMBERS AND NO PUBLIC CONSTRUCTOR: the
/// only way to build one is [`couple_fixed_field_fact`], so a
/// model-space consumer that takes a `CoupledFact` cannot be handed a
/// bare fixed-field [`Fact`] at all. That is the fence made
/// unconstructible rather than merely checked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoupledFact {
    fact: Fact,
    source: ModelFieldId,
    target: ModelFieldId,
    coupling: FieldCoupling,
}

impl CoupledFact {
    /// The transported fact.
    pub fn fact(&self) -> &Fact {
        &self.fact
    }

    /// The identity the fact was established under.
    pub fn source(&self) -> &ModelFieldId {
        &self.source
    }

    /// The identity it may now be read under.
    pub fn target(&self) -> &ModelFieldId {
        &self.target
    }

    /// The justification that let it cross.
    pub fn coupling(&self) -> &FieldCoupling {
        &self.coupling
    }
}

/// The one gate through the field-identity fence (item 7). `fact` was
/// established under the fixed field `source`; `target` is the model
/// belief that wants to read it; `witness` re-prices both authorities at
/// the coupling state and is the only evidence accepted.
///
/// The parity witness is supplied by the caller rather than computed
/// here because the fixed-field side is a different recursion with its
/// own cost and its own refusal modes (the σ1 mind's blocked set, named
/// by the σ1-repair slice): forcing the caller to produce it keeps the
/// price of a coupling visible at the call site, where the budget lives.
pub fn couple_fixed_field_fact(
    fact: Fact,
    source: &ModelFieldId,
    target: &ModelBelief,
    witness: Option<PointMassWitness>,
) -> Result<CoupledFact, CouplingRefusal> {
    let target_id = mixture_field_id(target);
    if source.as_str() == target_id.as_str() {
        return Ok(CoupledFact {
            fact,
            source: source.clone(),
            target: target_id,
            coupling: FieldCoupling::Identical,
        });
    }
    let live = target.profiles();
    if live.len() != 1 {
        return Err(CouplingRefusal::MixtureTarget {
            source: source.as_str().to_string(),
            target: target_id.as_str().to_string(),
            live_profiles: live.len(),
        });
    }
    let types = live[0].types();
    let head = types[0].id();
    if types.iter().any(|t| t.id() != head) {
        return Err(CouplingRefusal::MixedSeatTypes { seats: types.len() });
    }
    let parent = types[0].parent_field().to_string();
    if parent != source.as_str() {
        return Err(CouplingRefusal::ParentFieldMismatch {
            source: source.as_str().to_string(),
            target_parent: parent,
        });
    }
    let witness = witness.ok_or(CouplingRefusal::MixtureTarget {
        source: source.as_str().to_string(),
        target: target_id.as_str().to_string(),
        live_profiles: 1,
    })?;
    if witness.behavior != head {
        return Err(CouplingRefusal::ParentFieldMismatch {
            source: source.as_str().to_string(),
            target_parent: parent,
        });
    }
    if !equal_ratio(witness.fixed_side, witness.model_side) {
        return Err(CouplingRefusal::ParityDisagreement {
            fixed_side: witness.fixed_side,
            model_side: witness.model_side,
        });
    }
    Ok(CoupledFact {
        fact,
        source: source.clone(),
        target: target_id,
        coupling: FieldCoupling::PointMassParity(witness),
    })
}

/// Exact equality of two `(mass, total)` pairs as rationals, by cross
/// multiplication — no division, no floats.
fn equal_ratio(a: (u128, u128), b: (u128, u128)) -> bool {
    let (an, ad) = a;
    let (bn, bd) = b;
    match (an.checked_mul(bd), bn.checked_mul(ad)) {
        (Some(l), Some(r)) => l == r,
        _ => BigInt::from(an) * BigInt::from(bd) == BigInt::from(bn) * BigInt::from(ad),
    }
}

// ---------------------------------------------------------------------------
// The posterior-carrying line (item 1).
// ---------------------------------------------------------------------------

/// One hidden step of a walked public line: what the acting seat could
/// have played, what it did play, and the model posterior AFTER the
/// observation. Every field is a derived view of the single
/// [`ModelBelief`] the walk carries.
pub struct PosteriorStep {
    /// The post-root public history BEFORE this step.
    pub history: Vec<Domino>,
    /// The hidden seat that acted, as a seat label.
    pub seat: String,
    /// The merged public branch table `Z_ht` at the step (§32).
    pub branches: Vec<(Domino, u128)>,
    /// `(typed rows, merged public branches)` — the aggregation census.
    pub census: (usize, usize),
    /// The action observed.
    pub observed: Domino,
    /// Profiles alive before the step, and after it: an observation
    /// that kills a profile is the posterior zeroing that type.
    pub live_before: usize,
    pub live_after: usize,
    /// Per hidden seat, the posterior type marginals after the step,
    /// over the post-step weighted total.
    pub marginals: Vec<(String, Vec<(BehaviorTypeId, u128)>)>,
    /// The post-step weighted total `Σ w·Z_θ`.
    pub weighted_total: u128,
}

/// A walked public line with its posterior evolution (item 1). The
/// trace holds no belief: it is the record of what one carried
/// [`ModelBelief`] did.
pub struct PosteriorTrace {
    pub steps: Vec<PosteriorStep>,
    /// Live profiles at the end of the line.
    pub final_live: usize,
    /// True when the line ran out of hidden steps before `plies` were
    /// taken (a decided or exhausted state), rather than being cut by
    /// the requested length.
    pub exhausted: bool,
}

impl PosteriorTrace {
    /// The number of hidden observations recorded.
    pub fn depth(&self) -> usize {
        self.steps.len()
    }

    /// True when some step strictly reduced the live profile count —
    /// the posterior actually excluding a type profile rather than
    /// merely re-weighting it.
    pub fn eliminated_a_profile(&self) -> bool {
        self.steps.iter().any(|s| s.live_after < s.live_before)
    }
}

/// Descend at most `plies` hidden observations from `model`, taking the
/// HEAVIEST merged public branch at each hidden state and the focal
/// policy's own choice at each focal state, recording the posterior
/// after every observation (item 1). Deterministic: ties on branch mass
/// go to the lower tile index.
///
/// The walk is the solver's, not a second one: hidden steps go through
/// [`ModelBelief::observe_with_survivors`] (Theorem 12.1 per profile,
/// positive-support tightened) and focal steps through
/// [`ModelBelief::focal_play`], which is exactly what the bundle
/// recursion does at those two node kinds.
pub fn trace_heaviest_line(
    oracle: &dyn ExactCoverOracle,
    model: &ModelBelief,
    focal: &dyn SlicePolicy,
    plies: usize,
) -> (ModelBelief, PosteriorTrace) {
    let mut live = model.same_state();
    let mut steps: Vec<PosteriorStep> = Vec::new();
    let mut exhausted = false;
    while steps.len() < plies {
        let Some(next) = advance_one(oracle, &live, focal) else {
            exhausted = true;
            break;
        };
        match next {
            Advance::Focal(child) => live = child,
            Advance::Hidden(step, child) => {
                steps.push(step);
                live = child;
            }
        }
    }
    let final_live = live.profiles().len();
    (
        live,
        PosteriorTrace {
            steps,
            final_live,
            exhausted,
        },
    )
}

enum Advance {
    Focal(ModelBelief),
    Hidden(PosteriorStep, ModelBelief),
}

/// One node of the traced line, or `None` at a state with no hidden
/// seat left to observe (every tile played, or the viewer holding a
/// state the walk cannot advance past without a focal choice it has no
/// legal action for — which cannot happen at a live state).
fn advance_one(
    oracle: &dyn ExactCoverOracle,
    model: &ModelBelief,
    focal: &dyn SlicePolicy,
) -> Option<Advance> {
    let viewer = model.profiles()[0].belief().kernel().viewer();
    if model.history().len() >= model.total_plays() {
        return None;
    }
    if model.seat_to_move() == viewer {
        let tile = model.focal_choice(focal);
        return Some(Advance::Focal(model.focal_play(tile)));
    }
    let branches = model.branch_masses(oracle);
    let (observed, _) = branches
        .iter()
        .copied()
        .reduce(|a, b| if b.1 > a.1 { b } else { a })
        .expect("a hidden seat to move has a branch");
    let census = model.typed_branch_census(oracle);
    let history = model.history().to_vec();
    let seat = format!("{}", model.seat_to_move());
    let live_before = model.profiles().len();
    let (child, _survivors) = model.observe_with_survivors(oracle, observed);
    let marginals = child
        .seat_type_marginals(oracle)
        .into_iter()
        .map(|(s, m)| (format!("{s}"), m))
        .collect();
    let step = PosteriorStep {
        history,
        seat,
        branches,
        census,
        observed,
        live_before,
        live_after: child.profiles().len(),
        marginals,
        weighted_total: child.weighted_total(oracle),
    };
    Some(Advance::Hidden(step, child))
}

// ---------------------------------------------------------------------------
// The response envelope (item 2, §21/§23).
// ---------------------------------------------------------------------------

/// One column of the §21 policy library: a realizable focal policy and
/// its §16 model-response vector at one state, as exact masses.
pub struct ResponseColumn {
    /// The policy's identity — a content address of its choice table.
    pub policy_id: String,
    /// `M_θ` per live profile, aligned with [`ModelBelief::profiles`].
    pub response: Vec<u128>,
    /// `Z_θ` per live profile. Shared by every column at one state (a
    /// focal choice changes no factor), and asserted equal on insert.
    pub totals: Vec<u128>,
}

/// The §21 column-and-cut lower `L^R(ν) = max_ρ ⟨ν, v_ρ⟩` as a reusable
/// object (item 2, §23's reuse-by-dot-product). Repricing the whole
/// library under a new model belief is `|R|` dot products; walking is
/// not involved.
///
/// The comparison is over MASSES rather than values, which is exact and
/// division-free: at one state every column shares the same `Z_θ`, so
/// every candidate's value has the same denominator `Σ w·Z_θ` and the
/// argmax of the values is the argmax of `Σ w·M_θ`.
pub struct ResponseEnvelope {
    columns: Vec<ResponseColumn>,
    totals: Vec<u128>,
}

/// The envelope's answer at one model belief: which column won and what
/// it is worth, as the exact pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnvelopeReading {
    pub policy_id: String,
    pub value: (u128, u128),
}

impl ResponseEnvelope {
    /// An empty library over a state whose per-profile totals are
    /// `totals`.
    pub fn new(totals: Vec<u128>) -> ResponseEnvelope {
        ResponseEnvelope {
            columns: Vec::new(),
            totals,
        }
    }

    /// Add a column, unless a policy of the same identity is already
    /// present. Returns whether the library grew — the facet counter.
    pub fn insert(&mut self, column: ResponseColumn) -> bool {
        assert_eq!(
            column.totals, self.totals,
            "every column of one envelope is priced at the same state, so the \
             per-profile totals are shared (a focal choice changes no factor)"
        );
        if self.columns.iter().any(|c| c.policy_id == column.policy_id) {
            return false;
        }
        self.columns.push(column);
        true
    }

    pub fn columns(&self) -> &[ResponseColumn] {
        &self.columns
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// `L^R(ν)` and its attaining column, by dot product. Ties go to the
    /// first inserted column, which makes the reading a deterministic
    /// function of insertion order alone.
    pub fn read(&self, weights: &[u128]) -> Option<EnvelopeReading> {
        assert_eq!(
            weights.len(),
            self.totals.len(),
            "a repricing weight vector is aligned with the envelope's state"
        );
        let denominator = dot(weights, &self.totals);
        let mut best: Option<(&ResponseColumn, u128)> = None;
        for column in &self.columns {
            let mass = dot(weights, &column.response);
            let better = match &best {
                None => true,
                Some((_, incumbent)) => mass > *incumbent,
            };
            if better {
                best = Some((column, mass));
            }
        }
        best.map(|(column, mass)| EnvelopeReading {
            policy_id: column.policy_id.clone(),
            value: (mass, denominator),
        })
    }
}

/// `Σ w_i · x_i` with checked arithmetic.
fn dot(weights: &[u128], xs: &[u128]) -> u128 {
    weights.iter().zip(xs.iter()).fold(0u128, |acc, (w, x)| {
        acc.checked_add(w.checked_mul(*x).expect("an exact mass fits u128"))
            .expect("an exact mass fits u128")
    })
}

/// One point of a swept ν grid: the belief's integer weights, what the
/// envelope read, and whether reading it required a new walk.
pub struct SweepPoint {
    pub weights: Vec<u128>,
    pub reading: EnvelopeReading,
    /// True when the envelope was strictly below the exact response at
    /// this belief and a walk had to supply the missing column — a
    /// FACET of the §21 upper envelope.
    pub new_facet: bool,
}

/// Sweep a grid of model beliefs against one state, maintaining the
/// §21 envelope (item 2, §23). `walk` prices the EXACT response under a
/// given weight vector and returns `(exact mass, the argmax column)`.
///
/// This entry point walks at every grid point on purpose: it is the
/// AUDITED sweep, and what it audits is the envelope's prediction.
/// Before each walk it records what the library would have answered, and
/// afterwards it asserts §21 (the library never exceeds the exact
/// response) and pins the saving that a cheap sweep would have realized
/// — [`SweepPoint::new_facet`] is true exactly at the points where the
/// library was strictly below and a walk was genuinely required. The
/// count of `new_facet` points is therefore the number of walks
/// [`ResponseEnvelope::read`] alone would have needed, measured rather
/// than asserted, and every reading returned here is exact.
pub fn sweep_envelope<F>(
    envelope: &mut ResponseEnvelope,
    grid: &[Vec<u128>],
    mut walk: F,
) -> Result<Vec<SweepPoint>, MixtureRefusal>
where
    F: FnMut(&[u128]) -> Result<(u128, ResponseColumn), MixtureRefusal>,
{
    let mut out: Vec<SweepPoint> = Vec::new();
    for weights in grid {
        let predicted = envelope.read(weights);
        let (exact_mass, column) = walk(weights)?;
        if let Some(p) = &predicted {
            assert!(
                p.value.0 <= exact_mass,
                "§21: the column-and-cut lower never exceeds the exact response"
            );
        }
        let new_facet = predicted.as_ref().is_none_or(|p| p.value.0 < exact_mass);
        envelope.insert(column);
        let reading = envelope
            .read(weights)
            .expect("an envelope with a column reads");
        assert_eq!(
            reading.value.0, exact_mass,
            "the envelope reads the exact response once its facet is present"
        );
        out.push(SweepPoint {
            weights: weights.clone(),
            reading,
            new_facet,
        });
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// The fusion price at one root action, with its ν-invariance witness.
// ---------------------------------------------------------------------------

/// The §19 witness that a coordinate's fusion price is zero at EVERY
/// model belief over the same types: one lawful policy attaining the
/// point-mass optimum `q_a(θ)` for every θ in the (full) support.
/// Constructed only where the equality was checked per profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommonOptimizer {
    pub policy_id: String,
    /// `V_θ(ρ*)` per profile — equal to `q_a(θ)` at every entry, which
    /// is the property that makes the corollary apply.
    pub per_profile_mass: Vec<u128>,
}

/// One root-action coordinate of the model census (item 5). The four
/// shapes are the honest ones: a priced coordinate, or a refusal naming
/// which side ran out of budget.
pub enum ActionCoordinate {
    /// Both sides priced.
    Priced(Box<PricedCoordinate>),
    /// The exact mixture response refused its declared read ceiling.
    ResponseRefused {
        action: Domino,
        refusal: MixtureRefusal,
    },
    /// `Q_a` was priced but the point-mass sequence behind `U^sep_a`
    /// refused. `Φ_a` is NOT reported: an upper that did not finish is
    /// not an upper.
    SeparatedRefused {
        action: Domino,
        q: (u128, u128),
        refusal: MixtureRefusal,
    },
}

/// A fully priced root-action coordinate.
pub struct PricedCoordinate {
    pub action: Domino,
    /// The exact mixture response `Q_a(ν)`.
    pub q: (u128, u128),
    /// Theorem 18.1's `U^sep_a(ν)`.
    pub usep: (u128, u128),
    /// `Φ_a = U^sep_a − Q_a` as an exact pair over the shared
    /// denominator. Nonnegative by Theorem 18.1, asserted.
    pub phi: (u128, u128),
    /// The per-profile point-mass optima `q_a(θ)`.
    pub point_mass_optima: Vec<u128>,
    /// The mixture argmax policy's identity.
    pub argmax_policy: String,
    /// The §19 witness, present exactly when `Φ_a = 0` — and then the
    /// coordinate's zero holds at EVERY model belief over these types
    /// (the ν-invariance corollary in the module doc).
    pub common_optimizer: Option<CommonOptimizer>,
    /// False where `U^sep_a` is 0 or the whole fiber mass: there the
    /// zero price is arithmetic, not evidence about model blindness
    /// (U0's vacuity discipline, carried across).
    pub substantive: bool,
    /// Field reads this coordinate spent, measured.
    pub reads: u64,
}

impl ActionCoordinate {
    pub fn action(&self) -> Domino {
        match self {
            ActionCoordinate::Priced(p) => p.action,
            ActionCoordinate::ResponseRefused { action, .. } => *action,
            ActionCoordinate::SeparatedRefused { action, .. } => *action,
        }
    }

    pub fn priced(&self) -> Option<&PricedCoordinate> {
        match self {
            ActionCoordinate::Priced(p) => Some(p),
            _ => None,
        }
    }
}

/// The declared per-coordinate read ceilings of one census (item 4).
/// Both are counted in field consultations, the unit the ledger
/// measures; `None` declines to cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CensusBudget {
    /// Ceiling on the exact mixture response of one root action.
    pub response_cap: Option<u64>,
    /// Ceiling on the whole point-mass sequence behind one root
    /// action's `U^sep`.
    pub separated_cap: Option<u64>,
}

/// The model census of one root (item 5): every legal root action
/// priced or refused, under one declared model belief and one declared
/// budget.
pub struct RootModelCensus {
    pub root_label: String,
    pub field: ModelFieldId,
    /// `Z` at the root, the physical fiber mass.
    pub fiber: u128,
    /// `Σ_θ w·Z_θ` at the root.
    pub augmented: u128,
    pub coordinates: Vec<ActionCoordinate>,
    /// Total field reads the census spent, measured.
    pub reads: u64,
    /// Per behavior type, the reads it received.
    pub reads_by_type: Vec<(BehaviorTypeId, u64)>,
}

impl RootModelCensus {
    /// Coordinates with a strictly positive fusion price.
    pub fn strict_prices(&self) -> Vec<&PricedCoordinate> {
        self.coordinates
            .iter()
            .filter_map(|c| c.priced())
            .filter(|p| p.phi.0 > 0)
            .collect()
    }

    /// Zero-price coordinates whose zero carries information (U0's
    /// vacuity discipline).
    pub fn substantive_zero_prices(&self) -> usize {
        self.coordinates
            .iter()
            .filter_map(|c| c.priced())
            .filter(|p| p.phi.0 == 0 && p.substantive)
            .count()
    }

    /// Zero-price coordinates where nothing was at stake.
    pub fn vacuous_zero_prices(&self) -> usize {
        self.coordinates
            .iter()
            .filter_map(|c| c.priced())
            .filter(|p| p.phi.0 == 0 && !p.substantive)
            .count()
    }

    pub fn refusals(&self) -> usize {
        self.coordinates
            .iter()
            .filter(|c| c.priced().is_none())
            .count()
    }
}

/// Price every legal root action of `model` under the declared budget
/// (item 5). One `ModelBelief` is constructed by the caller; this
/// function walks its focal children.
pub fn model_census(
    oracle: &dyn ExactCoverOracle,
    root_label: &str,
    model: &ModelBelief,
    budget: CensusBudget,
) -> RootModelCensus {
    let field = mixture_field_id(model);
    let entry = &model.profiles()[0];
    let belief = entry.belief();
    let fiber = oracle.mass(belief);
    let augmented = model.weighted_total(oracle);
    assert!(
        model.history().is_empty(),
        "a root census runs at the root, before any observation"
    );
    let legal = model
        .legal_focal_actions()
        .expect("a census root has the viewer to move");
    assert!(!legal.is_empty(), "a root holds a legal action");
    let weights: Vec<u128> = model.profiles().iter().map(|e| e.weight()).collect();
    let mut coordinates: Vec<ActionCoordinate> = Vec::new();
    for action in legal.iter() {
        let before = model.ledger().total();
        let at_action = model.focal_play(action);
        let mut stats = MixtureStats::default();
        let response =
            match at_action.mixture_response_budgeted(oracle, budget.response_cap, &mut stats) {
                Ok(r) => r,
                Err(refusal) => {
                    coordinates.push(ActionCoordinate::ResponseRefused { action, refusal });
                    continue;
                }
            };
        let q = (
            response.outcome.weighted_mass,
            response.outcome.weighted_total,
        );
        let optima = match at_action.point_mass_optima(oracle, budget.separated_cap) {
            Ok(o) => o,
            Err(refusal) => {
                coordinates.push(ActionCoordinate::SeparatedRefused { action, q, refusal });
                continue;
            }
        };
        let point_mass_optima: Vec<u128> = optima.iter().map(|(m, _)| *m).collect();
        let usep_mass = dot(&weights, &point_mass_optima);
        assert!(
            usep_mass >= q.0,
            "Theorem 18.1: Q_a(ν) ≤ U^sep_a(ν) at every coordinate"
        );
        let phi = (usep_mass - q.0, q.1);
        // §19: when the price is zero at this FULL-support belief, the
        // argmax policy is the common optimizer — checked per profile
        // rather than inferred, because the check is one repricing.
        let common_optimizer = if phi.0 == 0 {
            let per_profile_mass = response.outcome.per_profile_mass.clone();
            assert_eq!(
                per_profile_mass, point_mass_optima,
                "§19 forward direction: a zero price at full support forces the \
                 mixture argmax to attain every point-mass optimum"
            );
            Some(CommonOptimizer {
                policy_id: response.policy.id().to_string(),
                per_profile_mass,
            })
        } else {
            None
        };
        let substantive = usep_mass > 0 && usep_mass < q.1;
        coordinates.push(ActionCoordinate::Priced(Box::new(PricedCoordinate {
            action,
            q,
            usep: (usep_mass, q.1),
            phi,
            point_mass_optima,
            argmax_policy: response.policy.id().to_string(),
            common_optimizer,
            substantive,
            reads: model.ledger().total() - before,
        })));
    }
    RootModelCensus {
        root_label: root_label.to_string(),
        field,
        fiber,
        augmented,
        coordinates,
        reads: model.ledger().total(),
        reads_by_type: model.ledger().per_type(),
    }
}

// ---------------------------------------------------------------------------
// The §49 producer (item 1: the values become facts).
// ---------------------------------------------------------------------------

/// The proof-state identity of a model-belief root (item 1 and item 7).
/// `field_id` is the MIXTURE's, never a component field's, and
/// `belief_id` names the model belief rather than the physical one —
/// which is what makes a σ0-authored fact `IdentityMismatch` here
/// without any new machinery.
pub fn mixture_identity(
    root: &CanonicalRoot,
    position: &RootPosition,
    model: &ModelBelief,
) -> SemanticsIdentity {
    SemanticsIdentity {
        root_id: root_identity(root, position),
        rules_id: "texas42-straight-v1".to_string(),
        field_id: mixture_field_id(model).as_str().to_string(),
        utility_id: "pmake-v1".to_string(),
        contract: position.bid,
        belief_id: "model-belief-uniform-root".to_string(),
        policy_class_id: "full-legal".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

/// The §49 producer for one root's model census (item 1). Per priced
/// coordinate it proposes three facts:
///
/// - `Q_a` as an EXECUTABLE lower, witnessed by the extracted mixture
///   argmax policy (a realizable deterministic policy, never an
///   envelope);
/// - `Q_a` as a deterministic upper, because the mixture response IS
///   the maximum over lawful policies at this belief — the two together
///   collapse the action's interval to a point;
/// - `U^sep_a` as a deterministic upper (Theorem 18.1), separately
///   typed and separately authored, so the Φ record survives in the
///   store even where it is zero.
///
/// Refused coordinates propose NOTHING. A refusal is not a bound.
pub struct ModelBeliefProducer {
    census: RootModelCensus,
    authority: String,
}

impl ModelBeliefProducer {
    pub fn new(census: RootModelCensus) -> ModelBeliefProducer {
        ModelBeliefProducer {
            authority: format!("mb1-model-belief:{}", census.field.as_str()),
            census,
        }
    }

    pub fn census(&self) -> &RootModelCensus {
        &self.census
    }
}

impl ProofProducer for ModelBeliefProducer {
    fn name(&self) -> &str {
        "mb1-model-belief-v1"
    }

    fn produce(&self, _state: &ProofState) -> Vec<Fact> {
        let mut facts = Vec::new();
        for coordinate in &self.census.coordinates {
            let Some(p) = coordinate.priced() else {
                continue;
            };
            let q = ratio(p.q);
            facts.push(Fact::Bound(BoundFact::lower(
                p.action,
                q.clone(),
                &format!("{}:mixture-argmax", self.authority),
                true,
                ProofTag::Deterministic,
            )));
            facts.push(Fact::Bound(BoundFact::upper(
                p.action,
                q,
                &format!("{}:mixture-response", self.authority),
                ProofTag::Deterministic,
            )));
            facts.push(Fact::Bound(BoundFact::upper(
                p.action,
                ratio(p.usep),
                &format!("{}:separated-upper", self.authority),
                ProofTag::Deterministic,
            )));
        }
        facts
    }
}

/// An exact `(mass, total)` pair as a rational in `[0, 1]`.
fn ratio(pair: (u128, u128)) -> BigRational {
    assert!(pair.1 > 0, "a value's denominator is the positive total");
    BigRational::new(BigInt::from(pair.0), BigInt::from(pair.1))
}

/// A response column for the extracted policy of one mixture response —
/// the shape [`sweep_envelope`] consumes.
pub fn column_of(policy: &MixturePolicy, response: Vec<u128>, totals: Vec<u128>) -> ResponseColumn {
    ResponseColumn {
        policy_id: policy.id().to_string(),
        response,
        totals,
    }
}

/// The response vector of one FIXED policy at a state, as the exact
/// per-profile masses (§16) — the library column a repricing sweep
/// stores.
pub fn response_vector(
    oracle: &dyn ExactCoverOracle,
    model: &ModelBelief,
    policy: &dyn SlicePolicy,
    cap: Option<u64>,
) -> Result<(Vec<u128>, Vec<u128>), MixtureRefusal> {
    let mut stats = MixtureStats::default();
    let outcome = model.mixture_policy_mass_budgeted(oracle, policy, cap, &mut stats)?;
    Ok((outcome.per_profile_mass, outcome.per_profile_total))
}

/// A rational-grid sweep of per-seat model beliefs: `steps + 1` points
/// from all-weight-on-the-first-type to all-weight-on-the-second, over
/// a two-type independent prior expanded to `profiles` product
/// profiles. Integer weights throughout — the grid is exact.
pub fn two_type_grid(seat_count: usize, steps: u128) -> Vec<Vec<u128>> {
    assert!(steps > 0, "a swept grid holds at least two points");
    let mut grid: Vec<Vec<u128>> = Vec::new();
    for k in 0..=steps {
        // Per-seat weights (k, steps − k); the profile weight is the
        // product over seats of the seat's weight for its assigned
        // type, in the same slot-0-outermost order
        // `from_independent_prior` expands.
        let per_seat = [steps - k, k];
        let profiles = 1usize << seat_count;
        let mut weights: Vec<u128> = Vec::with_capacity(profiles);
        for index in 0..profiles {
            let mut w: u128 = 1;
            for slot in 0..seat_count {
                // Slot 0 outermost: the highest bit is slot 0.
                let bit = (index >> (seat_count - 1 - slot)) & 1;
                w = w.checked_mul(per_seat[bit]).expect("an exact weight fits");
            }
            weights.push(w);
        }
        grid.push(weights);
    }
    grid
}

/// The per-profile weight vector of a model belief, in profile order.
pub fn weights_of(model: &ModelBelief) -> Vec<u128> {
    model.profiles().iter().map(|e| e.weight()).collect()
}

/// The declared behavior-type multiset of a model belief, for report
/// headers: every distinct type id in first-profile order.
pub fn declared_types(model: &ModelBelief) -> Vec<BehaviorTypeId> {
    let mut out: Vec<BehaviorTypeId> = Vec::new();
    for entry in model.profiles() {
        for behavior in entry.types() {
            if !out.contains(&behavior.id()) {
                out.push(behavior.id());
            }
        }
    }
    out
}

/// Ordered profile labels, for report headers and identity dumps.
pub fn profile_labels(model: &ModelBelief) -> Vec<String> {
    model.profiles().iter().map(|e| e.label()).collect()
}

/// A map from behavior-type id to its short label, for reports.
pub fn short_labels(ids: &[BehaviorTypeId]) -> BTreeMap<String, String> {
    ids.iter()
        .map(|id| (id.to_string(), id.short()))
        .collect::<BTreeMap<String, String>>()
}

/// Keep the `Rc` import meaningful for downstream callers constructing
/// shared minds; re-exported so gates and probes name one path.
pub type SharedMind = Rc<dyn SlicePolicy>;
