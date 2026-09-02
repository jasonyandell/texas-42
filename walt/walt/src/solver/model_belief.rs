//! `solver::model_belief` — the MB0 exact finite-type vertical slice: the
//! field model as a persistent hidden coordinate (§74's bounded
//! assignment), over the existing counted-belief machinery.
//!
//! EXPLORATORY tier. Mathematical source:
//! `walt/math/model_belief_base_player_v0.1.md` §§5–20 (types, Ξ = Ω×Θ,
//! Theorem 7.1's augmented-world reduction, §9 persistence, §11 hand-type
//! factors, Theorem 12.1's posterior closure, §13 exact branch masses,
//! §16 response vectors, Theorem 18.1's separated upper, §19's
//! model-fusion price) and §§74–76 (the assignment, the report, the
//! go/no-go), as repaired by the intake companion
//! (`walt/math/model_belief_base_player_v0.1_intake.md` — the §8
//! erratum: `Q_a(δ_{F_k})` equals the ordinary exact best response to
//! rung `F_k`; the corrected rung table: F₀ = σ0 = `FieldKind::Level0`,
//! F₁ = `FieldKind::Level1`), adopted by rulings MB-A1..A8
//! (`walt/CENSUS-RULINGS.md`); brief `walt/briefs/BRIEF-MB0.md`; status
//! ledger `walt/FACTOR-BELIEF.md`.
//!
//! THE REPRESENTATION (Theorem 7.1 made a struct). A [`ModelBelief`] is
//! the augmented belief over Ξ = Ω×Θ stored as its exact type-profile
//! expansion: one existing [`FactorBelief`] per profile θ (the physical
//! belief conditional on θ), an integer prior weight `w(θ)` per profile
//! (the rational prior ν(θ) = w(θ)/Σw — exact integers, cleared of the
//! denominator), and one [`ProfileField`] per profile — the profile's
//! seat-local deterministic field, dispatching the acting hidden seat to
//! its assigned type's mind. Every mass, branch table, posterior, and
//! value is derived through the existing `factor_belief` authorities;
//! nothing here forks the contraction, conditioning, or recursion
//! machinery. The §52 hand-type factor `φ_s(H, θ_s)` is the DERIVED VIEW
//! [`ModelBelief::hand_type_factor`]: profiles agreeing on seat `s`'s
//! type must agree as φ-maps wherever both store a hand — Theorem
//! 12.1's seat-locality, asserted on every derivation (see the
//! positive-support tightening below for why full byte equality is not
//! the right law); it holds because conditioning multiplies only the
//! acting seat's factor by a kernel likelihood that reads only (θ_s,
//! the seat's hand, the public record).
//!
//! PERSISTENCE IS STRUCTURAL (§9, MB-I2, MB-O12). A profile's types are
//! fixed at construction and never change along any lineage —
//! [`ModelBelief::focal_play`] and [`ModelBelief::observe`] carry each
//! entry's type vector unchanged (an entry can only be DROPPED, when an
//! observed action has zero mass under it). No resampling path exists:
//! there is no API that redraws a seat's type mid-hand. The §9 ½-vs-¼
//! separation is gated on a real-root specimen.
//!
//! MERGE BEFORE MAX (§13, §32, MB-I4, MB-O16). Hidden branching is by
//! PUBLIC ACTION only: [`ModelBelief::branch_masses`] aggregates the
//! per-profile branch tables tile by tile, and the mixture walk recurses
//! per tile into the surviving sub-bundle. No hidden-type branch exists
//! in any signature. On the focal side, hidden types are UNREADABLE by
//! construction (MB-I1): the mixture walk consults the focal policy
//! exactly ONCE per focal information state for the entire bundle — a
//! [`SlicePolicy`] is never evaluated per profile, so no policy, however
//! stateful, can key its choice on θ. The mixture response's maximization
//! likewise picks ONE action per public history for the whole bundle,
//! merged before max; the type-revealed relaxation exists only as the
//! separately-typed upper [`ModelBelief::separated_upper`] (Theorem
//! 18.1), never as a policy.
//!
//! THE ARITHMETIC. All masses are exact `u128` with checked arithmetic,
//! as in `factor_belief`; every reported probability is the exact pair
//! (weighted mass, weighted total) with the weighted total
//! `Σ_θ w(θ)·Z_θ(B)`. Fixed-policy value `V_ν(ρ) = Σ_θ w·M_θ / Σ_θ w·Z_θ`
//! is linear in ν by construction (MB-O5, gated on a swept rational
//! grid); the mixture response `Q(ν)` maximizes `Σ_θ w·M_θ` at each
//! focal node (lawful because a focal play changes no factor, so every
//! child shares each `Z_θ` — the §48 argument, lifted to the weighted
//! bundle); the separated upper is `Σ_θ w·q_θ` with `q_θ` the
//! single-profile respond walk — the §8 point-mass demotion, anchored
//! by the gates to the raw fixed-field authority on its terminating
//! domain. `Q(ν) ≤ U^sep` is Theorem 18.1; the difference is the
//! model-fusion price (§19).
//!
//! Deterministic types only in this slice: a stochastic type needs an
//! explicit tape coordinate (§6's `Z`), which is structurally absent
//! here — [`BehaviorType`] holds no tape and its identity says so.
//!
//! THE POSITIVE-SUPPORT TIGHTENING (an MB0 discovery, mechanical in the
//! gates). The σ1 mind materializes its belief by the §4.2
//! shuffle-and-reject sampler (`solver::sample_belief`), whose
//! acceptance region is empty exactly at information states of zero
//! joint completion mass — a candidate hand that is record-consistent
//! for its own seat but jointly uncompletable against the other seats'
//! void structure. The shared conditioning route classifies the acting
//! seat's RAW support, which can contain such zero-mass entries (they
//! are harmless dead weight under σ0; under σ1 their classification
//! never terminates — a live specimen is pinned in the gate file).
//! This module therefore tightens the acting seat's factor to its
//! positive joint support ([`ExactCoverOracle::actor_completion_weights`]
//! — pure counting, no field reads) immediately before every
//! conditioning. Dropping a zero-completion entry changes NO exact
//! mass, branch table, posterior, or value (it contributes zero to
//! every contraction — the same law that lets `FactorWeights` never
//! store zero-weight entries); it changes only the stored table
//! representation, so this module's factor tables are DECLARED to be
//! φ restricted to the positive support at each conditioning time.
//! Consequence for the derived §52 view: two profiles sharing θ_s may
//! trim different zero-mass entries (positivity is a joint property),
//! so [`ModelBelief::hand_type_factor`] asserts Theorem 12.1's
//! mechanical residue — same-type factors agree as φ-maps wherever
//! both store a hand — and returns the union table; every asymmetric
//! entry was dropped by construction only when its completion weight
//! was exactly zero in the trimming profile.
//!
//! ---
//!
//! MB1 EXTENSIONS (brief `walt/briefs/BRIEF-MB1.md`, items 3 and 4;
//! the recursion's own machinery lives beside this module in
//! [`crate::solver::model_recursion`]). Three additions, none of which
//! changes any exact value MB0 computed:
//!
//! 1. TIGHTENING IS NOW THE DEFAULT ON EVERY CLASSIFYING ENTRY POINT
//!    (item 3). MB0 tightened inside [`ModelBelief::observe`] and the
//!    mixture walk but not in [`ModelBelief::branch_masses`] or
//!    [`ModelBelief::typed_branch_census`], which classify the acting
//!    seat's raw support and so could reach the σ1 sampler's empty
//!    acceptance region. Both now tighten first. Exactness-neutral by
//!    the zero-entry law (a dropped entry contributes zero to every
//!    branch and to `Z`), so every merged branch mass is unchanged;
//!    what changes is only [`ModelBelief::typed_branch_census`]'s
//!    typed-row count, which is a representation census and is
//!    declared to count POSITIVE-SUPPORT rows.
//!
//! 2. THE FIELD-READ LEDGER ([`ReadLedger`], item 4). Every
//!    [`ProfileField`] dispatch records the consulted behavior type in
//!    a ledger shared by the whole lineage of one constructed
//!    [`ModelBelief`] — the same object survives
//!    [`ModelBelief::focal_play`], [`ModelBelief::observe`] and the
//!    per-profile walks inside [`ModelBelief::separated_upper`]. The
//!    ledger is APPEND-ONLY and its counts are a measurement of work
//!    actually spent, never a forecast and never the cap.
//!
//! 3. TYPED BUDGET REFUSALS ([`MixtureRefusal`], item 4). The budgeted
//!    entry points ([`ModelBelief::mixture_response_budgeted`],
//!    [`ModelBelief::mixture_policy_mass_budgeted`],
//!    [`ModelBelief::separated_upper_budgeted`]) carry a declared
//!    ceiling on the field reads one walk may spend and return
//!    `Result`. The ceiling is checked at the boundary of every walked
//!    bundle node BEFORE that node spends anything, so a refusal
//!    reports the exact reads spent and the public history it stopped
//!    at — there is no truncated value anywhere, and the unbudgeted
//!    entry points MB0 shipped are the same walk under an absent
//!    ceiling (a refusal is then unconstructible, which is why they
//!    keep returning a value).

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use crate::kernel::HIDDEN_SEATS;
use crate::rules::{legal_plays, Decl, Domino, DominoSet, Seat, Trick};
use crate::solver::adaptive::{
    decided_success, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{ExactCoverOracle, FactorBelief};
use crate::solver::field::{FieldModel, FieldSpec};
use crate::solver::policy::{content_digest, Canon, TieRule};

// ---------------------------------------------------------------------------
// Behavior-type identity (§51, brief item 1).
// ---------------------------------------------------------------------------

/// The declared persistence scope of a behavior type (§9, §51). One
/// variant exists in this slice — the hand-persistent semantics MB-I2
/// binds — and the machinery implements exactly it. A future scope
/// variant lands together with its machinery, never as a bare tag.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PersistenceScope {
    /// The type is fixed once per seat for the whole hand, never
    /// resampled per action.
    PerHand,
}

impl PersistenceScope {
    fn tag(self) -> u8 {
        match self {
            PersistenceScope::PerHand => 0,
        }
    }
}

/// The immutable content address (SHA-256) of a complete behavior-type
/// declaration (§51): construction, parent field identity, tie rule,
/// persistence scope, and the deterministic no-tape marker. Changing any
/// behavior-affecting coordinate changes the identity (a gate).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BehaviorTypeId([u8; 32]);

impl BehaviorTypeId {
    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The first eight bytes as lowercase hex — the short label used in
    /// profile identities and reports. Collisions here would cost label
    /// readability only; every identity comparison uses the full digest.
    pub fn short(&self) -> String {
        let mut s = String::new();
        for b in &self.0[..8] {
            s.push_str(&format!("{b:02x}"));
        }
        s
    }
}

impl fmt::Display for BehaviorTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// One registered persistent behavior type: the §51 identity coordinates
/// plus the materialized deterministic mind. Deterministic fields carry
/// no tape — the absence is structural (no tape field exists) and named
/// in the identity preimage, so a future stochastic type cannot alias a
/// deterministic one.
pub struct BehaviorType {
    id: BehaviorTypeId,
    construction: String,
    parent_field: String,
    tie_rule: TieRule,
    persistence: PersistenceScope,
    mind: Rc<dyn SlicePolicy>,
}

impl BehaviorType {
    /// The canonical serialization the identity hashes: a fresh header
    /// (aliasing no other serialization family), every §51 coordinate
    /// this slice carries, and the deterministic no-tape marker.
    fn canonical_bytes(
        construction: &str,
        parent_field: &str,
        tie_rule: TieRule,
        persistence: PersistenceScope,
    ) -> Vec<u8> {
        let mut canon = Canon::new("walt-behavior-type-v1");
        canon.str_field(0x01, construction);
        canon.str_field(0x02, parent_field);
        canon.tag(0x03);
        canon.u8(match tie_rule {
            TieRule::FirstInPreference => 0,
            TieRule::LowestTileIndex => 1,
        });
        canon.tag(0x04);
        canon.u8(persistence.tag());
        // Deterministic: no tape (§6's Z coordinate is structurally
        // absent; the marker keeps the preimage honest about it).
        canon.tag(0x05);
        canon.u8(0);
        canon.finish()
    }

    /// Register an existing solver field as a persistent behavior type
    /// (brief item 1): the parent field identity is the complete
    /// [`FieldSpec`] content address, so every behavior-affecting field
    /// coordinate (construction, level, inner configuration, seed
    /// schedule, tie rule, fallback, mode) is in this identity too.
    pub fn from_field(spec: FieldSpec, persistence: PersistenceScope) -> BehaviorType {
        let construction = spec.construction.clone();
        let tie_rule = spec.tie_rule;
        let model = FieldModel::new(spec);
        let parent_field = format!("field:{}", model.field_id());
        let id = BehaviorTypeId(content_digest(&BehaviorType::canonical_bytes(
            &construction,
            &parent_field,
            tie_rule,
            persistence,
        )));
        BehaviorType {
            id,
            construction,
            parent_field,
            tie_rule,
            persistence,
            mind: Rc::new(model),
        }
    }

    /// Register a declared deterministic carrier mind (the specimen path
    /// for gates and fixtures — labeled by its construction string,
    /// never presented as a solver rung). The declared coordinates are
    /// identity; the caller vouches the mind is deterministic and
    /// information-consistent (every [`SlicePolicy`] must be).
    pub fn declared(
        construction: &str,
        parent_field: &str,
        tie_rule: TieRule,
        persistence: PersistenceScope,
        mind: Rc<dyn SlicePolicy>,
    ) -> BehaviorType {
        let id = BehaviorTypeId(content_digest(&BehaviorType::canonical_bytes(
            construction,
            parent_field,
            tie_rule,
            persistence,
        )));
        BehaviorType {
            id,
            construction: construction.to_string(),
            parent_field: parent_field.to_string(),
            tie_rule,
            persistence,
            mind,
        }
    }

    pub fn id(&self) -> BehaviorTypeId {
        self.id
    }

    pub fn construction(&self) -> &str {
        &self.construction
    }

    pub fn parent_field(&self) -> &str {
        &self.parent_field
    }

    pub fn tie_rule(&self) -> TieRule {
        self.tie_rule
    }

    pub fn persistence(&self) -> PersistenceScope {
        self.persistence
    }

    /// The materialized mind, for point-mass parity comparisons against
    /// the raw field authority.
    pub fn mind(&self) -> &Rc<dyn SlicePolicy> {
        &self.mind
    }
}

// ---------------------------------------------------------------------------
// The profile field: one θ's seat-local deterministic field.
// ---------------------------------------------------------------------------

/// The exact field-consultation ledger of one model-belief lineage
/// (MB1 item 4). Every [`ProfileField`] dispatch records the behavior
/// type it consulted. APPEND-ONLY: there is no reset and no decrement,
/// so a reported count is always the work a walk actually spent —
/// callers take a baseline before a walk and read the difference.
/// Single-threaded interior mutability, because [`SlicePolicy::choose`]
/// takes `&self` and the ledger must observe the real dispatch.
pub struct ReadLedger {
    per_type: RefCell<Vec<(BehaviorTypeId, u64)>>,
    total: Cell<u64>,
}

impl ReadLedger {
    fn new() -> ReadLedger {
        ReadLedger {
            per_type: RefCell::new(Vec::new()),
            total: Cell::new(0),
        }
    }

    fn record(&self, id: BehaviorTypeId) {
        let mut per_type = self.per_type.borrow_mut();
        match per_type.iter_mut().find(|(t, _)| *t == id) {
            Some((_, n)) => *n += 1,
            None => per_type.push((id, 1)),
        }
        self.total.set(self.total.get() + 1);
    }

    /// Total field consultations recorded on this lineage.
    pub fn total(&self) -> u64 {
        self.total.get()
    }

    /// The per-type census, in first-consulted order.
    pub fn per_type(&self) -> Vec<(BehaviorTypeId, u64)> {
        self.per_type.borrow().clone()
    }

    /// Consultations recorded for one type.
    pub fn reads_of(&self, id: BehaviorTypeId) -> u64 {
        self.per_type
            .borrow()
            .iter()
            .find(|(t, _)| *t == id)
            .map_or(0, |(_, n)| *n)
    }
}

/// Why a budgeted mixture walk stopped without a value (MB1 item 4,
/// §34/§35 shape). The only variant is a declared-ceiling refusal, and
/// it carries the MEASURED reads spent — never the ceiling as a
/// stand-in — together with the public history it stopped at. No
/// variant carries a value: a refused walk has no number, and none can
/// be written into this type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MixtureRefusal {
    /// The declared field-read ceiling was reached before the walk
    /// closed.
    ReadBudget {
        /// Field consultations this walk had spent when it stopped.
        spent: u64,
        /// The declared ceiling that stopped it.
        cap: u64,
        /// The post-root public history of the bundle node that was
        /// about to be expanded.
        at_history: Vec<Domino>,
    },
}

impl fmt::Display for MixtureRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MixtureRefusal::ReadBudget {
                spent,
                cap,
                at_history,
            } => {
                write!(f, "read-budget refusal: spent {spent} of cap {cap} at [")?;
                for (i, d) in at_history.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{d}")?;
                }
                write!(f, "]")
            }
        }
    }
}

/// One walk's declared read ceiling, resolved against the lineage
/// ledger's baseline. An absent cap makes [`WalkBudget::check`] total,
/// which is why the unbudgeted entry points cannot refuse.
struct WalkBudget<'a> {
    ledger: &'a ReadLedger,
    baseline: u64,
    cap: Option<u64>,
}

impl WalkBudget<'_> {
    fn spent(&self) -> u64 {
        self.ledger.total() - self.baseline
    }

    fn check(&self, history: &[Domino]) -> Result<(), MixtureRefusal> {
        match self.cap {
            Some(cap) if self.spent() >= cap => Err(MixtureRefusal::ReadBudget {
                spent: self.spent(),
                cap,
                at_history: history.to_vec(),
            }),
            _ => Ok(()),
        }
    }
}

/// The seat-dispatched field of one type profile θ: the acting hidden
/// seat (a derived view of the public record — leader plus plays this
/// trick) is routed to its assigned type's mind. Seat-locality is
/// inherited: each inner mind reads only (its own hand, the public
/// record), so the profile field is a lawful deterministic
/// [`SlicePolicy`] and Theorem 12.1 applies to every conditioning under
/// it. The viewer has no assignment — a focal consultation panics.
/// Every dispatch is recorded in the lineage's [`ReadLedger`] (MB1
/// item 4): the count is taken at the dispatch itself, so it measures
/// the mind's real consultations rather than the walk's estimate of
/// them.
pub struct ProfileField {
    label: String,
    assignment: [Option<Rc<BehaviorType>>; Seat::COUNT],
    ledger: Rc<ReadLedger>,
}

impl SlicePolicy for ProfileField {
    fn id(&self) -> &str {
        &self.label
    }

    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        let seat = record.leader.plus(record.trick_plays.len());
        let behavior = self.assignment[seat.index()]
            .as_ref()
            .expect("a hidden seat has a type; the viewer never consults the field");
        self.ledger.record(behavior.id());
        behavior.mind.choose(decl, hand, legal, record)
    }
}

// ---------------------------------------------------------------------------
// The model belief (§6, §11): the exact profile expansion of Ξ.
// ---------------------------------------------------------------------------

/// A per-seat type prior for the independent-prior constructor: the
/// hidden seat and its (type, positive integer weight) alternatives. The
/// rational per-seat prior is weight/Σweights — ν = (1/2, 1/2) is two
/// entries of weight 1.
pub struct SeatTypePrior {
    pub seat: Seat,
    pub types: Vec<(Rc<BehaviorType>, u128)>,
}

/// One live profile θ of a [`ModelBelief`]: the per-hidden-slot types
/// (kernel hidden-slot order), the immutable prior weight `w(θ)`, the
/// profile's dispatched field, and the physical [`FactorBelief`]
/// conditional on θ. Constructed only by [`ModelBelief`]; readable
/// everywhere.
pub struct ProfileEntry {
    types: Vec<Rc<BehaviorType>>,
    weight: u128,
    field: Rc<ProfileField>,
    belief: FactorBelief,
}

impl ProfileEntry {
    /// The profile's types in kernel hidden-slot order.
    pub fn types(&self) -> &[Rc<BehaviorType>] {
        &self.types
    }

    /// The immutable prior weight `w(θ)` — never changed by
    /// conditioning (the hand factor absorbs every likelihood; the
    /// posterior type weight is the derived view `w·Z_θ / Σ w·Z`).
    pub fn weight(&self) -> u128 {
        self.weight
    }

    /// The profile's dispatched field.
    pub fn field(&self) -> &Rc<ProfileField> {
        &self.field
    }

    /// The physical belief conditional on this profile.
    pub fn belief(&self) -> &FactorBelief {
        &self.belief
    }

    /// The profile label used in reports: the per-slot type short ids.
    pub fn label(&self) -> String {
        let parts: Vec<String> = self.types.iter().map(|t| t.id().short()).collect();
        parts.join(",")
    }
}

/// The §52 hand-type factor `φ_s(H, θ_s)` as a derived view: one hidden
/// seat's per-type hand tables, extracted from the profile expansion
/// under Theorem 12.1's mechanical residue — profiles agreeing on the
/// seat's type must agree as φ-maps wherever both store a hand (checked
/// on every derivation); each slice is the UNION table over its
/// profiles, since positive-support tightening (module doc) may trim
/// different zero-mass entries in different profiles. Types absent from
/// the live support are absent here (their posterior mass is zero).
pub struct HandTypeFactor {
    pub seat: Seat,
    /// (type id, the type's evolved union hand table), first-live order.
    pub slices: Vec<(BehaviorTypeId, Vec<(DominoSet, u128)>)>,
}

/// The augmented belief over Ξ = Ω×Θ (§6), stored as its exact profile
/// expansion. See the module doc for the representation ruling.
pub struct ModelBelief {
    /// Σ of profile prior weights at construction — the prior
    /// denominator, constant along every lineage (dropped profiles keep
    /// their prior weight in the denominator; their posterior mass is
    /// zero through `Z_θ = 0`, represented by absence).
    prior_denominator: u128,
    entries: Vec<ProfileEntry>,
    /// The lineage's field-consultation ledger (MB1 item 4), shared by
    /// every profile field and by every belief derived from this one.
    ledger: Rc<ReadLedger>,
}

impl ModelBelief {
    /// The general constructor (§10's interface honesty: independence is
    /// a convenience, never asserted as permanent): explicit profiles as
    /// (types in kernel hidden-slot order, positive weight). Distinct
    /// profiles only; at least one.
    pub fn from_profile_prior(
        root: &CanonicalRoot,
        position: &RootPosition,
        profiles: Vec<(Vec<Rc<BehaviorType>>, u128)>,
    ) -> ModelBelief {
        assert!(!profiles.is_empty(), "a model belief holds a profile");
        let hidden = root.kernel().hidden();
        let ledger = Rc::new(ReadLedger::new());
        let mut entries: Vec<ProfileEntry> = Vec::new();
        let mut denominator: u128 = 0;
        for (types, weight) in profiles {
            assert!(weight > 0, "a prior profile weight is positive");
            assert_eq!(
                types.len(),
                HIDDEN_SEATS,
                "a profile assigns one type per hidden seat, in kernel slot order"
            );
            let ids: Vec<BehaviorTypeId> = types.iter().map(|t| t.id()).collect();
            assert!(
                !entries.iter().any(|e| {
                    e.types
                        .iter()
                        .map(|t| t.id())
                        .collect::<Vec<BehaviorTypeId>>()
                        == ids
                }),
                "prior profiles are distinct"
            );
            let mut assignment: [Option<Rc<BehaviorType>>; Seat::COUNT] =
                core::array::from_fn(|_| None);
            for (slot, behavior) in types.iter().enumerate() {
                assert_eq!(
                    behavior.persistence(),
                    PersistenceScope::PerHand,
                    "this slice's machinery implements the hand-persistent scope"
                );
                assignment[hidden[slot].seat.index()] = Some(Rc::clone(behavior));
            }
            let label = format!(
                "profile:{}",
                types
                    .iter()
                    .map(|t| t.id().short())
                    .collect::<Vec<String>>()
                    .join(",")
            );
            let field = Rc::new(ProfileField {
                label,
                assignment,
                ledger: Rc::clone(&ledger),
            });
            let belief = FactorBelief::uniform_root(root, position, field.as_ref());
            denominator = denominator
                .checked_add(weight)
                .expect("an exact weight sum fits u128");
            entries.push(ProfileEntry {
                types,
                weight,
                field,
                belief,
            });
        }
        ModelBelief {
            prior_denominator: denominator,
            entries,
            ledger,
        }
    }

    /// The independent-prior constructor (brief item 2): per-seat type
    /// priors, one per hidden seat, expanded to the product profile
    /// prior — profile weight = the product of its per-seat weights.
    pub fn from_independent_prior(
        root: &CanonicalRoot,
        position: &RootPosition,
        priors: &[SeatTypePrior],
    ) -> ModelBelief {
        let hidden = root.kernel().hidden();
        assert_eq!(
            priors.len(),
            HIDDEN_SEATS,
            "one per-seat prior per hidden seat"
        );
        // Per-seat alternatives in kernel hidden-slot order.
        let per_slot: Vec<&SeatTypePrior> = hidden
            .iter()
            .map(|slot| {
                priors
                    .iter()
                    .find(|p| p.seat == slot.seat)
                    .expect("every hidden seat has a declared type prior")
            })
            .collect();
        for prior in &per_slot {
            assert!(!prior.types.is_empty(), "a seat prior holds a type");
        }
        // The cartesian product, slot-0 outermost, deterministic order.
        let mut profiles: Vec<(Vec<Rc<BehaviorType>>, u128)> = vec![(Vec::new(), 1u128)];
        for prior in &per_slot {
            let mut next = Vec::new();
            for (types, weight) in &profiles {
                for (behavior, w) in &prior.types {
                    let mut extended = types.clone();
                    extended.push(Rc::clone(behavior));
                    next.push((
                        extended,
                        weight.checked_mul(*w).expect("an exact weight fits u128"),
                    ));
                }
            }
            profiles = next;
        }
        ModelBelief::from_profile_prior(root, position, profiles)
    }

    /// The live profiles (positive-posterior-support profiles only —
    /// observation drops a profile exactly when the observed action has
    /// zero mass under it).
    pub fn profiles(&self) -> &[ProfileEntry] {
        &self.entries
    }

    /// The constant prior denominator Σw.
    pub fn prior_denominator(&self) -> u128 {
        self.prior_denominator
    }

    /// The shared post-root public history (identical across profiles —
    /// asserted).
    pub fn history(&self) -> &[Domino] {
        let history = self.entries[0].belief.history();
        for entry in &self.entries[1..] {
            assert_eq!(
                entry.belief.history(),
                history,
                "one public history governs every profile"
            );
        }
        history
    }

    /// The seat to move at the shared public state.
    pub fn seat_to_move(&self) -> Seat {
        self.entries[0].belief.seat_to_move()
    }

    /// The weighted augmented mass `Σ_θ w(θ)·Z_θ(B)` — the exact-cover
    /// mass of the belief over Ξ, cleared of the prior denominator.
    pub fn weighted_total(&self, oracle: &dyn ExactCoverOracle) -> u128 {
        self.entries.iter().fold(0u128, |acc, entry| {
            acc.checked_add(weighted(entry.weight, oracle.mass(&entry.belief)))
                .expect("an exact mass fits u128")
        })
    }

    /// Theorem 23.1, focal case, lifted to the bundle: the viewer plays
    /// `action`; every profile's history advances and NO factor changes.
    pub fn focal_play(&self, action: Domino) -> ModelBelief {
        ModelBelief {
            prior_denominator: self.prior_denominator,
            entries: self
                .entries
                .iter()
                .map(|entry| ProfileEntry {
                    types: entry.types.clone(),
                    weight: entry.weight,
                    field: Rc::clone(&entry.field),
                    belief: entry.belief.focal_play(action),
                })
                .collect(),
            ledger: Rc::clone(&self.ledger),
        }
    }

    /// The lineage's field-consultation ledger (MB1 item 4) — a derived
    /// view of work already spent, shared by every belief descended
    /// from the same constructor call.
    pub fn ledger(&self) -> &Rc<ReadLedger> {
        &self.ledger
    }

    /// The §13 exact branch masses of the acting hidden seat, MERGED BY
    /// PUBLIC ACTION across profiles (§32, MB-I4): `Z_ht = Σ_θ w(θ)·
    /// Z_ht(θ)`, sorted by tile. Conservation `Σ_t Z_ht = Σ_θ w·Z_θ` is
    /// asserted here at the merged level (MB-I6) on top of the
    /// per-profile assertion inside every contraction.
    ///
    /// MB1 item 3: the acting seat's factor is tightened to its
    /// positive joint support before any classification, so this entry
    /// point never asks a σ1 mind about a zero-completion hand. Every
    /// returned mass is unchanged by the tightening (a dropped entry
    /// contributes zero to every branch).
    pub fn branch_masses(&self, oracle: &dyn ExactCoverOracle) -> Vec<(Domino, u128)> {
        let mut merged: Vec<(Domino, u128)> = Vec::new();
        for entry in &self.entries {
            let tight = tighten_acting(oracle, &entry.belief);
            for (tile, mass) in oracle.branch_masses(&tight, entry.field.as_ref()) {
                let m = weighted(entry.weight, mass);
                match merged.iter_mut().find(|(t, _)| *t == tile) {
                    Some((_, acc)) => *acc = acc.checked_add(m).expect("an exact mass fits u128"),
                    None => merged.push((tile, m)),
                }
            }
        }
        merged.sort_by_key(|(t, _)| t.index());
        let total: u128 = merged.iter().fold(0u128, |acc, (_, m)| {
            acc.checked_add(*m).expect("an exact mass fits u128")
        });
        assert_eq!(
            total,
            self.weighted_total(oracle),
            "merged mass conservation: Σ_t Z_ht = Σ_θ w·Z_θ (MB-I6)"
        );
        merged
    }

    /// The §75 aggregation census at the acting hidden state: (the sum
    /// over live profiles of their distinct branch tiles, the count of
    /// merged public branches). The first is what a type-branching
    /// solver would pay; the second is what public-action merging pays.
    /// Both are counted over the POSITIVE-SUPPORT tables (MB1 item 3),
    /// which is the representation every walk actually classifies.
    pub fn typed_branch_census(&self, oracle: &dyn ExactCoverOracle) -> (usize, usize) {
        let mut typed = 0usize;
        let mut merged: Vec<Domino> = Vec::new();
        for entry in &self.entries {
            let tight = tighten_acting(oracle, &entry.belief);
            let branches = oracle.branch_masses(&tight, entry.field.as_ref());
            typed += branches.len();
            for (tile, _) in branches {
                if !merged.contains(&tile) {
                    merged.push(tile);
                }
            }
        }
        (typed, merged.len())
    }

    /// The specimen/census refinement, mirroring
    /// [`FactorBelief::with_factor_table`]: every profile's belief
    /// refined to the same declared sub-table of one seat's hand factor
    /// (a declared narrowing, type-independent by construction — the
    /// physical evidence is shared; posterior updates stay
    /// [`ModelBelief::observe`]'s alone).
    pub fn with_seat_table(&self, seat: Seat, table: Vec<(DominoSet, u128)>) -> ModelBelief {
        ModelBelief {
            prior_denominator: self.prior_denominator,
            entries: self
                .entries
                .iter()
                .map(|entry| ProfileEntry {
                    types: entry.types.clone(),
                    weight: entry.weight,
                    field: Rc::clone(&entry.field),
                    belief: entry.belief.with_factor_table(seat, table.clone()),
                })
                .collect(),
            ledger: Rc::clone(&self.ledger),
        }
    }

    /// Theorem 12.1 lifted to the bundle: the acting hidden seat is
    /// observed playing `action`. Each surviving profile conditions its
    /// OWN belief (multiplying only the acting seat's factor by that
    /// profile's kernel likelihood); a profile under which the action
    /// has zero mass is dropped — that is the posterior zeroing its
    /// type weight, represented by absence. The profile weights never
    /// change (persistence: the type is the same latent object; the
    /// evidence lives in the hand factors).
    pub fn observe(&self, oracle: &dyn ExactCoverOracle, action: Domino) -> ModelBelief {
        self.observe_with_survivors(oracle, action).0
    }

    /// [`ModelBelief::observe`] together with the indices (into
    /// [`ModelBelief::profiles`] BEFORE the observation) of the
    /// profiles that survived it — the alignment a posterior-carrying
    /// recursion needs to scatter per-profile masses back to their
    /// parents (MB1 item 1). The surviving indices are strictly
    /// increasing.
    pub fn observe_with_survivors(
        &self,
        oracle: &dyn ExactCoverOracle,
        action: Domino,
    ) -> (ModelBelief, Vec<usize>) {
        let mut entries: Vec<ProfileEntry> = Vec::new();
        let mut survivors: Vec<usize> = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            let tight = tighten_acting(oracle, &entry.belief);
            let supported = oracle
                .branch_masses(&tight, entry.field.as_ref())
                .iter()
                .any(|(tile, _)| *tile == action);
            if !supported {
                continue;
            }
            survivors.push(i);
            entries.push(ProfileEntry {
                types: entry.types.clone(),
                weight: entry.weight,
                field: Rc::clone(&entry.field),
                belief: oracle.condition(&tight, action, entry.field.as_ref()),
            });
        }
        assert!(
            !entries.is_empty(),
            "an observed action has positive augmented mass"
        );
        (
            ModelBelief {
                prior_denominator: self.prior_denominator,
                entries,
                ledger: Rc::clone(&self.ledger),
            },
            survivors,
        )
    }

    /// The posterior profile masses: per live profile, `w(θ)·Z_θ(B)`.
    /// The posterior probability of θ is the exact pair over
    /// [`ModelBelief::weighted_total`].
    pub fn posterior_profile_masses(&self, oracle: &dyn ExactCoverOracle) -> Vec<(String, u128)> {
        self.entries
            .iter()
            .map(|entry| {
                (
                    entry.label(),
                    weighted(entry.weight, oracle.mass(&entry.belief)),
                )
            })
            .collect()
    }

    /// The per-seat posterior type marginals: for each hidden seat, the
    /// live types with their posterior masses `Σ_{θ: θ_s = t} w·Z_θ`.
    pub fn seat_type_marginals(
        &self,
        oracle: &dyn ExactCoverOracle,
    ) -> Vec<(Seat, Vec<(BehaviorTypeId, u128)>)> {
        let hidden = self.entries[0].belief.kernel().hidden();
        (0..HIDDEN_SEATS)
            .map(|slot| {
                let mut marginal: Vec<(BehaviorTypeId, u128)> = Vec::new();
                for entry in &self.entries {
                    let id = entry.types[slot].id();
                    let m = weighted(entry.weight, oracle.mass(&entry.belief));
                    match marginal.iter_mut().find(|(t, _)| *t == id) {
                        Some((_, acc)) => {
                            *acc = acc.checked_add(m).expect("an exact mass fits u128");
                        }
                        None => marginal.push((id, m)),
                    }
                }
                (hidden[slot].seat, marginal)
            })
            .collect()
    }

    /// The §52 hand-type factor of one hidden seat, derived from the
    /// profile expansion under Theorem 12.1's mechanical residue (see
    /// [`HandTypeFactor`]).
    pub fn hand_type_factor(&self, seat: Seat) -> HandTypeFactor {
        let hidden = self.entries[0].belief.kernel().hidden();
        let slot = hidden
            .iter()
            .position(|h| h.seat == seat)
            .expect("a hidden seat has a slot");
        let mut slices: Vec<(BehaviorTypeId, Vec<(DominoSet, u128)>)> = Vec::new();
        for entry in &self.entries {
            let id = entry.types[slot].id();
            let support = entry.belief.factors()[slot].support();
            match slices.iter_mut().find(|(t, _)| *t == id) {
                Some((_, union)) => {
                    for (hand, weight) in support {
                        match union.iter().find(|(h, _)| *h == hand) {
                            Some((_, existing)) => assert_eq!(
                                *existing, weight,
                                "profiles agreeing on a seat's type agree as φ-maps \
                                 wherever both store a hand (Theorem 12.1, mechanical)"
                            ),
                            None => union.push((hand, weight)),
                        }
                    }
                }
                None => slices.push((id, support)),
            }
        }
        HandTypeFactor { seat, slices }
    }
}

/// `w·m` with checked arithmetic.
fn weighted(weight: u128, mass: u128) -> u128 {
    weight.checked_mul(mass).expect("an exact mass fits u128")
}

/// The positive-support tightening (module doc): the acting seat's
/// factor narrowed to entries with nonzero exact completion weight,
/// derived by pure counting before any classification touches the
/// support. Exactness-neutral by the zero-entry law; load-bearing for
/// σ1 termination. When nothing is dropped the belief is returned
/// unchanged (representation stability in the common case).
fn tighten_acting(oracle: &dyn ExactCoverOracle, belief: &FactorBelief) -> FactorBelief {
    let seat = belief.seat_to_move();
    let positive: Vec<DominoSet> = oracle
        .actor_completion_weights(belief, seat)
        .into_iter()
        .map(|(hand, _)| hand)
        .collect();
    let slot = belief
        .factors()
        .iter()
        .position(|f| f.seat() == seat)
        .expect("a hidden seat has a factor");
    let support = belief.factors()[slot].support();
    let table: Vec<(DominoSet, u128)> = support
        .iter()
        .filter(|(hand, _)| positive.contains(hand))
        .cloned()
        .collect();
    if table.len() == support.len() {
        return belief.clone();
    }
    belief.with_factor_table(seat, table)
}

// ---------------------------------------------------------------------------
// The mixture walk: fixed-policy value and exact mixture response on Ξ.
// ---------------------------------------------------------------------------

/// Exact integer counters of one mixture walk over the bundle.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct MixtureStats {
    /// Bundle nodes settled by the decided cutoff before terminal depth.
    pub decided_early: u64,
    /// Bundle nodes settled at the terminal depth itself.
    pub decided_terminal: u64,
    /// Focal bundle nodes walked (one focal consultation or max each).
    pub focal_nodes: u64,
    /// Actions explored across focal nodes (1 per node in fixed mode).
    pub focal_actions: u64,
    /// Hidden bundle nodes walked.
    pub hidden_nodes: u64,
    /// Posterior updates performed — one per (profile, branch) taken.
    pub conditionings: u64,
}

/// One mixture evaluation: the per-profile response coordinates (§16's
/// model-response vector as exact masses) and their weighted sums. The
/// value is the exact pair `weighted_mass / weighted_total`; the
/// per-profile value against θ is `per_profile_mass[i] /
/// per_profile_total[i]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MixtureOutcome {
    /// Per live profile (aligned with [`ModelBelief::profiles`]): the
    /// success mass `M_θ` of the evaluated/extracted policy against θ.
    pub per_profile_mass: Vec<u128>,
    /// Per live profile: the physical mass `Z_θ` at the evaluated state.
    pub per_profile_total: Vec<u128>,
    /// `Σ_θ w·M_θ`.
    pub weighted_mass: u128,
    /// `Σ_θ w·Z_θ`.
    pub weighted_total: u128,
}

impl MixtureOutcome {
    /// §16/§23 repricing: the same fixed policy's value under DIFFERENT
    /// profile weights, as the exact pair `(Σ w'·M_θ, Σ w'·Z_θ)`. Two
    /// dot products against the stored response vector — no walk. The
    /// weights are integer prior weights in profile order, cleared of
    /// their denominator exactly as [`ProfileEntry::weight`] is; they
    /// need not sum to anything in particular, and a zero weight is a
    /// profile the new belief excludes.
    ///
    /// Why this is exact and not an approximation: the walk's
    /// per-profile masses `M_θ` are computed against θ alone (a fixed
    /// focal policy makes the same public choices for every profile,
    /// so no profile's mass depends on any other profile's weight),
    /// and the mixture value at any state is
    /// `Σ w·M_θ / Σ w·Z_θ` — the posterior-weighted average of the
    /// per-profile values, since the posterior weight of θ is itself
    /// `w·Z_θ / Σ w·Z`.
    pub fn reprice(&self, weights: &[u128]) -> (u128, u128) {
        assert_eq!(
            weights.len(),
            self.per_profile_mass.len(),
            "a repricing weight vector is aligned with the response vector"
        );
        let mass = weights
            .iter()
            .zip(self.per_profile_mass.iter())
            .fold(0u128, |acc, (w, m)| {
                acc.checked_add(weighted(*w, *m))
                    .expect("an exact mass fits u128")
            });
        let total = weights
            .iter()
            .zip(self.per_profile_total.iter())
            .fold(0u128, |acc, (w, z)| {
                acc.checked_add(weighted(*w, *z))
                    .expect("an exact mass fits u128")
            });
        (mass, total)
    }
}

/// The extracted mixture-argmax policy: ONE realizable deterministic
/// policy (never an envelope), keyed by post-root public history, with
/// off-DAG states completed by the declared lowest-legal-tile rule —
/// the same shape as the §63 extraction. Its id is a content address of
/// the choice table. Information-consistent by construction: the key is
/// the public history alone.
pub struct MixturePolicy {
    id: String,
    choices: BTreeMap<Vec<u8>, Domino>,
}

impl MixturePolicy {
    fn new(choices: BTreeMap<Vec<u8>, Domino>) -> MixturePolicy {
        let mut bytes = Vec::new();
        for (k, v) in &choices {
            bytes.extend_from_slice(k);
            bytes.push(0xff);
            bytes.push(u8::try_from(v.index()).expect("a tile index fits u8"));
            bytes.push(0xfe);
        }
        let digest = content_digest(&bytes);
        let mut hex = String::new();
        for b in &digest[..16] {
            hex.push_str(&format!("{b:02x}"));
        }
        MixturePolicy {
            id: format!("mixture-argmax-{hex}"),
            choices,
        }
    }

    /// Recorded focal states — the DAG's size.
    pub fn states(&self) -> usize {
        self.choices.len()
    }

    /// The recorded choice at one post-root history, when on the DAG.
    pub fn choice_at(&self, history: &[Domino]) -> Option<Domino> {
        self.choices.get(&history_key(history)).copied()
    }
}

impl SlicePolicy for MixturePolicy {
    fn id(&self) -> &str {
        &self.id
    }

    fn choose(
        &self,
        _decl: Decl,
        _hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        if let Some(d) = self.choices.get(&history_key(record.history)) {
            assert!(
                legal.contains(*d),
                "a recorded choice was legal at extraction and legality is a \
                 function of the same public state and hand"
            );
            return *d;
        }
        legal
            .iter()
            .next()
            .expect("a seat to move holds a legal tile")
    }
}

/// The mixture response: the extracted argmax policy and its outcome —
/// `weighted_mass` is the exact `Q(ν)` numerator over `weighted_total`.
pub struct MixtureResponse {
    pub outcome: MixtureOutcome,
    pub policy: MixturePolicy,
}

/// The canonical key of one focal information state: the post-root
/// public history as tile indices (the viewer sees every play; its hand
/// is a function of root and history).
fn history_key(history: &[Domino]) -> Vec<u8> {
    history
        .iter()
        .map(|d| u8::try_from(d.index()).expect("a tile index fits u8"))
        .collect()
}

/// The walked public state, one per bundle node — the same trick
/// arithmetic as the replay walkers, restricted to public data.
#[derive(Clone)]
struct PublicWalk {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played_by: [DominoSet; Seat::COUNT],
    history: Vec<Domino>,
}

impl PublicWalk {
    fn start(position: &RootPosition, history: &[Domino]) -> PublicWalk {
        let mut walk = PublicWalk {
            leader: position.leader,
            plays: position.trick_plays.clone(),
            banked: position.banked,
            played_by: [DominoSet::EMPTY; Seat::COUNT],
            history: Vec::new(),
        };
        for d in history {
            walk.play(position, *d);
        }
        walk
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.played_by[seat.index()].insert(tile),
            "a tile is played once"
        );
        self.plays.push(tile);
        self.history.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }
}

/// The focal regime of one mixture walk: one frozen policy consulted
/// once per information state for the whole bundle, or the exact
/// response max — ONE action per information state for the whole
/// bundle, merged before max (§32; the type-revealed relaxation is
/// [`ModelBelief::separated_upper`], never this walk).
enum FocalMode<'a> {
    Fixed(&'a dyn SlicePolicy),
    Respond,
}

/// The bundle recursion (Theorem 7.1 operational): the §23/§48 walk
/// over the profile expansion. Decided and focal cases read the SHARED
/// public state (identical across profiles); hidden cases branch by
/// public action into the surviving sub-bundle. Returns per-entry
/// success masses (unweighted `M_θ`); the caller weights them. In
/// respond mode, `choices` receives the argmax table of the walked
/// region (winner branches only).
///
/// MB1 item 4: the declared read ceiling is checked at the boundary of
/// every walked node, BEFORE the node spends anything, and a refusal
/// propagates out unchanged. Under an absent ceiling
/// [`WalkBudget::check`] is total and the `Err` arm is unreachable —
/// which is what lets MB0's entry points keep returning a bare value.
#[allow(clippy::too_many_arguments)]
fn mixture_walk(
    oracle: &dyn ExactCoverOracle,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total_plays: usize,
    entries: &[ProfileEntry],
    walk: &PublicWalk,
    mode: &FocalMode<'_>,
    stats: &mut MixtureStats,
    choices: &mut BTreeMap<Vec<u8>, Domino>,
    budget: &WalkBudget<'_>,
) -> Result<Vec<u128>, MixtureRefusal> {
    budget.check(&walk.history)?;
    let at_terminal = walk.history.len() == total_plays;
    if let Some(u) = decided_success(position, viewer, walk.banked, at_terminal) {
        if at_terminal {
            stats.decided_terminal += 1;
        } else {
            stats.decided_early += 1;
        }
        return Ok(entries
            .iter()
            .map(|entry| if u { oracle.mass(&entry.belief) } else { 0 })
            .collect());
    }
    assert!(
        walk.history.len() < total_plays,
        "the 42-point pool exhausts at terminal, so an undecided state has plays left"
    );
    if walk.seat() == viewer {
        stats.focal_nodes += 1;
        let remaining = viewer_hand.difference(walk.played_by[viewer.index()]);
        let led = walk.plays.first().map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        match mode {
            FocalMode::Fixed(policy) => {
                // ONE consultation for the whole bundle: the policy
                // cannot key on θ because it is never evaluated per
                // profile (MB-I1).
                stats.focal_actions += 1;
                let record = walk.record(position);
                let tile = policy.choose(position.decl, remaining, legal, &record);
                assert!(legal.contains(tile), "a policy chooses a legal tile");
                descend_focal(
                    oracle,
                    position,
                    viewer,
                    viewer_hand,
                    total_plays,
                    entries,
                    walk,
                    mode,
                    stats,
                    choices,
                    tile,
                    budget,
                )
            }
            FocalMode::Respond => {
                // ONE action per information state for the whole bundle,
                // chosen by the weighted total — merged before max.
                // Ascending tile order with strictly-greater replacement
                // realizes the declared lowest-tile-index tie rule.
                struct Candidate {
                    total: u128,
                    masses: Vec<u128>,
                    choices: BTreeMap<Vec<u8>, Domino>,
                    tile: Domino,
                }
                let mut best: Option<Candidate> = None;
                for tile in legal.iter() {
                    stats.focal_actions += 1;
                    let mut sub_choices = BTreeMap::new();
                    let masses = descend_focal(
                        oracle,
                        position,
                        viewer,
                        viewer_hand,
                        total_plays,
                        entries,
                        walk,
                        mode,
                        stats,
                        &mut sub_choices,
                        tile,
                        budget,
                    )?;
                    let total = entries
                        .iter()
                        .zip(masses.iter())
                        .fold(0u128, |acc, (entry, m)| {
                            acc.checked_add(weighted(entry.weight, *m))
                                .expect("an exact mass fits u128")
                        });
                    let better = match &best {
                        None => true,
                        Some(incumbent) => total > incumbent.total,
                    };
                    if better {
                        best = Some(Candidate {
                            total,
                            masses,
                            choices: sub_choices,
                            tile,
                        });
                    }
                }
                let winner = best.expect("a legal set holds an action");
                choices.extend(winner.choices);
                choices.insert(history_key(&walk.history), winner.tile);
                Ok(winner.masses)
            }
        }
    } else {
        // §13/§32: branch by PUBLIC ACTION into the surviving
        // sub-bundle; sum over branches (the hidden seat is not the
        // optimizer). Per-profile conservation is asserted inside every
        // `branch_masses` call.
        stats.hidden_nodes += 1;
        // Tighten every profile to its positive support before any
        // classification (module doc; exactness-neutral).
        let entries: Vec<ProfileEntry> = entries
            .iter()
            .map(|entry| ProfileEntry {
                types: entry.types.clone(),
                weight: entry.weight,
                field: Rc::clone(&entry.field),
                belief: tighten_acting(oracle, &entry.belief),
            })
            .collect();
        let entries = &entries[..];
        let tables: Vec<Vec<(Domino, u128)>> = entries
            .iter()
            .map(|entry| oracle.branch_masses(&entry.belief, entry.field.as_ref()))
            .collect();
        let mut tiles: Vec<Domino> = Vec::new();
        for table in &tables {
            for (tile, _) in table {
                if !tiles.contains(tile) {
                    tiles.push(*tile);
                }
            }
        }
        tiles.sort_by_key(|t| t.index());
        let mut masses = vec![0u128; entries.len()];
        for tile in tiles {
            let mut sub_entries: Vec<ProfileEntry> = Vec::new();
            let mut sub_index: Vec<usize> = Vec::new();
            for (i, (entry, table)) in entries.iter().zip(tables.iter()).enumerate() {
                if table.iter().any(|(t, _)| t == &tile) {
                    stats.conditionings += 1;
                    sub_entries.push(ProfileEntry {
                        types: entry.types.clone(),
                        weight: entry.weight,
                        field: Rc::clone(&entry.field),
                        belief: oracle.condition(&entry.belief, tile, entry.field.as_ref()),
                    });
                    sub_index.push(i);
                }
            }
            let mut sub_walk = walk.clone();
            sub_walk.play(position, tile);
            let sub_masses = mixture_walk(
                oracle,
                position,
                viewer,
                viewer_hand,
                total_plays,
                &sub_entries,
                &sub_walk,
                mode,
                stats,
                choices,
                budget,
            )?;
            for (i, m) in sub_index.into_iter().zip(sub_masses) {
                masses[i] = masses[i].checked_add(m).expect("an exact mass fits u128");
            }
        }
        Ok(masses)
    }
}

/// The shared focal descent: every profile advances by the same public
/// action (no factor changes), the walk advances once.
#[allow(clippy::too_many_arguments)]
fn descend_focal(
    oracle: &dyn ExactCoverOracle,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total_plays: usize,
    entries: &[ProfileEntry],
    walk: &PublicWalk,
    mode: &FocalMode<'_>,
    stats: &mut MixtureStats,
    choices: &mut BTreeMap<Vec<u8>, Domino>,
    tile: Domino,
    budget: &WalkBudget<'_>,
) -> Result<Vec<u128>, MixtureRefusal> {
    let sub_entries: Vec<ProfileEntry> = entries
        .iter()
        .map(|entry| ProfileEntry {
            types: entry.types.clone(),
            weight: entry.weight,
            field: Rc::clone(&entry.field),
            belief: entry.belief.focal_play(tile),
        })
        .collect();
    let mut sub_walk = walk.clone();
    sub_walk.play(position, tile);
    mixture_walk(
        oracle,
        position,
        viewer,
        viewer_hand,
        total_plays,
        &sub_entries,
        &sub_walk,
        mode,
        stats,
        choices,
        budget,
    )
}

impl ModelBelief {
    fn walk_frame(&self) -> (RootPosition, Seat, DominoSet, usize, PublicWalk) {
        let belief = &self.entries[0].belief;
        let position = belief.position().clone();
        let viewer = belief.kernel().viewer();
        let viewer_hand = belief.kernel().viewer_hand();
        let total_plays = viewer_hand.len()
            + belief
                .kernel()
                .hidden()
                .iter()
                .map(|h| h.capacity)
                .sum::<usize>();
        let history = self.history().to_vec();
        let walk = PublicWalk::start(&position, &history);
        (position, viewer, viewer_hand, total_plays, walk)
    }

    fn outcome_of(&self, oracle: &dyn ExactCoverOracle, masses: Vec<u128>) -> MixtureOutcome {
        let per_profile_total: Vec<u128> = self
            .entries
            .iter()
            .map(|entry| oracle.mass(&entry.belief))
            .collect();
        let weighted_mass =
            self.entries
                .iter()
                .zip(masses.iter())
                .fold(0u128, |acc, (entry, m)| {
                    acc.checked_add(weighted(entry.weight, *m))
                        .expect("an exact mass fits u128")
                });
        let weighted_total =
            self.entries
                .iter()
                .zip(per_profile_total.iter())
                .fold(0u128, |acc, (entry, z)| {
                    acc.checked_add(weighted(entry.weight, *z))
                        .expect("an exact mass fits u128")
                });
        MixtureOutcome {
            per_profile_mass: masses,
            per_profile_total,
            weighted_mass,
            weighted_total,
        }
    }

    /// The fixed-policy mixture evaluation `V_ν(ρ)` (§16): one frozen
    /// focal policy walked over the whole bundle, consulted once per
    /// focal information state. The value is the exact pair
    /// `weighted_mass / weighted_total`; the per-profile masses are the
    /// §16 model-response vector of ρ at this state.
    pub fn mixture_policy_mass(
        &self,
        oracle: &dyn ExactCoverOracle,
        focal: &dyn SlicePolicy,
        stats: &mut MixtureStats,
    ) -> MixtureOutcome {
        self.mixture_policy_mass_budgeted(oracle, focal, None, stats)
            .expect("an absent read ceiling makes a budget refusal unconstructible")
    }

    /// [`ModelBelief::mixture_policy_mass`] under a declared field-read
    /// ceiling (MB1 item 4): `cap` is the number of field
    /// consultations THIS walk may spend, measured against the
    /// lineage ledger's value on entry. `None` is the unbudgeted walk.
    pub fn mixture_policy_mass_budgeted(
        &self,
        oracle: &dyn ExactCoverOracle,
        focal: &dyn SlicePolicy,
        cap: Option<u64>,
        stats: &mut MixtureStats,
    ) -> Result<MixtureOutcome, MixtureRefusal> {
        let (position, viewer, viewer_hand, total_plays, walk) = self.walk_frame();
        let mut choices = BTreeMap::new();
        let budget = WalkBudget {
            ledger: &self.ledger,
            baseline: self.ledger.total(),
            cap,
        };
        let masses = mixture_walk(
            oracle,
            &position,
            viewer,
            viewer_hand,
            total_plays,
            &self.entries,
            &walk,
            &FocalMode::Fixed(focal),
            stats,
            &mut choices,
            &budget,
        )?;
        Ok(self.outcome_of(oracle, masses))
    }

    /// The exact mixture response `Q(ν)` (§16, boxed): the max over
    /// lawful focal policies of the ν-weighted success mass, computed by
    /// the bundle walk with ONE action per information state (merged
    /// before max), plus the extracted argmax policy realizing it.
    pub fn mixture_response(
        &self,
        oracle: &dyn ExactCoverOracle,
        stats: &mut MixtureStats,
    ) -> MixtureResponse {
        self.mixture_response_budgeted(oracle, None, stats)
            .expect("an absent read ceiling makes a budget refusal unconstructible")
    }

    /// [`ModelBelief::mixture_response`] under a declared field-read
    /// ceiling (MB1 item 4). On refusal there is NO value — not a
    /// truncated one, not a partial maximum: the walk's own maximum is
    /// only defined once every branch of an information state has been
    /// priced, so the honest result is the typed refusal alone.
    pub fn mixture_response_budgeted(
        &self,
        oracle: &dyn ExactCoverOracle,
        cap: Option<u64>,
        stats: &mut MixtureStats,
    ) -> Result<MixtureResponse, MixtureRefusal> {
        let (position, viewer, viewer_hand, total_plays, walk) = self.walk_frame();
        let mut choices = BTreeMap::new();
        let budget = WalkBudget {
            ledger: &self.ledger,
            baseline: self.ledger.total(),
            cap,
        };
        let masses = mixture_walk(
            oracle,
            &position,
            viewer,
            viewer_hand,
            total_plays,
            &self.entries,
            &walk,
            &FocalMode::Respond,
            stats,
            &mut choices,
            &budget,
        )?;
        Ok(MixtureResponse {
            outcome: self.outcome_of(oracle, masses),
            policy: MixturePolicy::new(choices),
        })
    }

    /// The type-revealed separated upper `U^sep` (§18): per live
    /// profile, the exact best response `q_θ = Q(δ_θ)` against that
    /// profile alone — the single-entry respond walk, i.e. the mixture
    /// response demoted to a point mass (the §8 ladder-demotion made
    /// operational; the gates anchor it to the raw fixed-field
    /// authority `response_success_mass` on its terminating domain) —
    /// weighted by the prior. The relaxation lets the focal policy
    /// differ per type; Theorem 18.1 makes it an upper on the mixture
    /// response, and the difference is the model-fusion price (§19).
    /// Separately typed from every other upper (MB-I8): this function's
    /// result is only ever a `U^sep`.
    pub fn separated_upper(&self, oracle: &dyn ExactCoverOracle) -> MixtureOutcome {
        self.separated_upper_budgeted(oracle, None)
            .expect("an absent read ceiling makes a budget refusal unconstructible")
    }

    /// [`ModelBelief::separated_upper`] under a declared field-read
    /// ceiling (MB1 item 4). The ceiling covers the WHOLE sequence of
    /// per-profile point-mass walks, because `U^sep` is their weighted
    /// sum and a partial sum is not an upper on anything.
    pub fn separated_upper_budgeted(
        &self,
        oracle: &dyn ExactCoverOracle,
        cap: Option<u64>,
    ) -> Result<MixtureOutcome, MixtureRefusal> {
        let (position, viewer, viewer_hand, total_plays, walk) = self.walk_frame();
        let budget = WalkBudget {
            ledger: &self.ledger,
            baseline: self.ledger.total(),
            cap,
        };
        let mut masses: Vec<u128> = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            let single = [ProfileEntry {
                types: entry.types.clone(),
                weight: 1,
                field: Rc::clone(&entry.field),
                belief: entry.belief.clone(),
            }];
            let mut stats = MixtureStats::default();
            let mut choices = BTreeMap::new();
            let m = mixture_walk(
                oracle,
                &position,
                viewer,
                viewer_hand,
                total_plays,
                &single,
                &walk,
                &FocalMode::Respond,
                &mut stats,
                &mut choices,
                &budget,
            )?;
            masses.push(m[0]);
        }
        Ok(self.outcome_of(oracle, masses))
    }

    /// The per-profile point-mass optima `q_a(θ)` (§18) as raw masses,
    /// aligned with [`ModelBelief::profiles`], together with the ONE
    /// realizable policy each attains (MB1 item 2: these are the
    /// columns a `U^sep`-attaining common policy would have to match
    /// simultaneously, so Theorem 19.1's witness search reads off this
    /// list). The weighted sum of the masses is exactly
    /// [`ModelBelief::separated_upper`]'s `weighted_mass`.
    pub fn point_mass_optima(
        &self,
        oracle: &dyn ExactCoverOracle,
        cap: Option<u64>,
    ) -> Result<Vec<(u128, MixturePolicy)>, MixtureRefusal> {
        let (position, viewer, viewer_hand, total_plays, walk) = self.walk_frame();
        let budget = WalkBudget {
            ledger: &self.ledger,
            baseline: self.ledger.total(),
            cap,
        };
        let mut out: Vec<(u128, MixturePolicy)> = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            let single = [ProfileEntry {
                types: entry.types.clone(),
                weight: 1,
                field: Rc::clone(&entry.field),
                belief: entry.belief.clone(),
            }];
            let mut stats = MixtureStats::default();
            let mut choices = BTreeMap::new();
            let m = mixture_walk(
                oracle,
                &position,
                viewer,
                viewer_hand,
                total_plays,
                &single,
                &walk,
                &FocalMode::Respond,
                &mut stats,
                &mut choices,
                &budget,
            )?;
            out.push((m[0], MixturePolicy::new(choices)));
        }
        Ok(out)
    }
}
