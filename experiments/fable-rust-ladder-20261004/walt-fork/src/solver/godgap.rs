//! The God-gap census (slice U0) — where the information-consistency
//! price actually appears.
//!
//! THE DECOMPOSITION. Fix a root, a declared deterministic field σ, a
//! contract `c`, and one root action `a`. Let `Z` be the fiber mass
//! after `a`, let `D` be the worlds in which NO viewer continuation —
//! even a world-aware one — attains the viewer objective against σ,
//! and let `Q` be the exact information-consistent optimum. The
//! salvation parent's §8 identity splits one executable policy's
//! total failure into three terms that answer three different
//! questions:
//!
//! ```text
//! 1 − V(ρ) = d_phys + d_info + d_policy(ρ)
//!   d_phys      = β(D)         = |D| / Z         physical doom
//!   d_info      = U^God − Q                      information price
//!   d_policy(ρ) = Q − V(ρ)                       policy gap
//!   U^God       = 1 − β(D)     = (Z − |D|) / Z
//! ```
//!
//! `d_phys` is what the doom census already measures. This module adds
//! the other two — and the point of the exercise is that a zero doom
//! census says NOTHING about them (§8: "a zero doom census affects
//! only the first term; it does not distinguish the second and the
//! third").
//!
//! GOD-TIGHTNESS. A coordinate is God-tight when `Q = U^God`
//! (`d_info = 0`): by Theorem 7.1 one information-consistent policy
//! saves every individually saveable world, so blindness costs
//! nothing there. The §36 machine receipt for it is an EQUALITY of two
//! independently produced numbers — a deterministic doom upper and an
//! executable policy's lower — at exact identity of root, field,
//! contract, and belief. No unrestricted best-response solve is
//! required by the receipt itself; this module happens to compute `Q`
//! exactly where it is affordable, which is strictly more than §36
//! asks for and is what lets the census also report POSITIVE gaps.
//!
//! One reading caveat travels with every God-tight receipt, and the
//! census keeps it typed rather than in prose. Where the WHOLE fiber
//! is physically doomed, `U^God = Q = 0` and Theorem 7.1's common
//! intersection is an intersection over an empty index set: every
//! lawful policy is God-tight, and the equality carries no information
//! about the price of blindness. Those coordinates are real receipts
//! but degenerate evidence, so [`GodTightPolicy::nothing_saveable`]
//! flags them and the §38 stratification counts them apart
//! ([`FusionStratum::substantively_fusion_free`]) — a depth that looks
//! fusion-free only because everything in it is lost is not evidence
//! for the fusion-free-suffix hypothesis.
//!
//! THE FOUR RESULT TYPES (§48, exactly these, and the SC-A4 rule).
//!
//! ```text
//! GodTightPolicy   exact Q = U^God, with an extracted policy re-priced to it
//! PositiveGodGap   exact Q < U^God — the gap is a measured number
//! GodUpper         a NONVACUOUS deterministic upper, no exact Q: gap unknown
//! UnknownGodGap    no exact Q and no certified doom: nothing but the trivial
//! ```
//!
//! The last two are the honest half. SC-A4 is binding: zero certified
//! doom with no exact `Q` is [`GodGapResult::UnknownGodGap`] and is
//! NEVER [`GodGapResult::PositiveGodGap`] — a census that has found no
//! counterexamples has not thereby found a gap. The split between
//! `GodUpper` and `UnknownGodGap` is exactly whether the doom side
//! produced anything: positive certified doom leaves a real upper
//! standing with an unknown gap beneath it; zero certified doom leaves
//! the vacuous upper 1 and no claim at all.
//!
//! WHAT THIS MODULE CONSUMES AND WHAT IT NEVER TOUCHES. The doom side
//! is `solver::doom`, used as-is and unmodified (§47 / SC-A3: the doom
//! census is PRESERVED as the deterministic singleton-conflict
//! producer and the exact God-upper ground truth on enumerable roots).
//! On an enumerable coordinate the God upper comes from
//! [`doom::doom_enumeration`] — the per-world truth, which is the
//! TIGHTEST doom mass any sound doom reasoning can certify — and
//! elsewhere from [`doom::doom_census`]'s certified harvest. The exact
//! `Q` is [`factor_belief::response_success_mass`]; the executable
//! policy is [`factor_belief::extract_success_policy`], re-priced by
//! the independent fixed-policy evaluator
//! [`factor_belief::viewer_success_mass`] (a different recursion — a
//! frozen focal policy, not a max — so the equality receipt is a
//! genuine cross-check and not a tautology). Persistence is the §49
//! proof state: the God upper installs as a deterministic upper and a
//! God-tight policy installs as its score-profile fact, so the
//! closure's own view of that action shows lower meeting upper.
//! Nothing here touches `refine.rs` (freeze 58), `doom.rs`, or
//! `model_belief.rs`.
//!
//! AFFORDABILITY AND REFUSALS. Both exact instruments are full
//! recursions over the fiber, so a coordinate above the declared
//! `exact_fiber_cap` is REFUSED — typed, counted, and carried in the
//! result (§34/§35's discipline as shipped in `frontier`/`opening`).
//! A refused coordinate is never silently dropped and never quietly
//! downgraded into a claim: it comes back as `GodUpper` or
//! `UnknownGodGap` with its reasons attached.
//!
//! THE FUSION HORIZON IS AN EMPIRICAL OBJECT. Stratifying the census
//! by trick depth gives the §38 fusion horizon — the earliest depth
//! beyond which every tested coordinate is God-tight on the declared
//! corpus. SC-A4 is binding here too: this is a measurement on a named
//! corpus, never a theorem, and the census reports its exceptions.
//!
//! Mathematical sources: `walt/math/salvation_complex_v0.1.md` §4–§9
//! (salvation sets, Theorem 7.1, the three-part decomposition, the §9
//! table), §36–§40 (God-tight nodes, the fusion horizon, the
//! cut-oriented experiment), §47–§48 (the immediate ruling and this
//! slice's spec), §55 (the shared data model these types follow);
//! `walt/math/salvation_complex_v0.1_intake.md` (the governing
//! companion); rulings SC-A1..A8 in `walt/CENSUS-RULINGS.md`.
//!
//! MODULE GRAPH. New-core beside `solver::proof_state`, in the
//! `doom`/`laydown`/`covers`/`extraction` pattern: it imports its
//! siblings and is imported by nothing but the crate root, so the
//! group stays deletable together (§67.10).

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{legal_plays, Domino, DominoSet};
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::doom::{doom_census, doom_enumeration, DoomSpec};
use crate::solver::factor_belief::{
    extract_success_policy, response_success_mass, viewer_score_profile, viewer_success_mass,
    ExactCoverOracle, ExtractionSource, FactorBelief, RecursionStats, ResponseStats,
};
use crate::solver::proof_state::{
    BoundFact, Fact, ProofProducer, ProofState, ProofTag, ScoreProfileFact,
};

// ---------------------------------------------------------------------------
// The §55 shared data model.
// ---------------------------------------------------------------------------

/// The identity every salvation object is relative to (§55): one root,
/// one declared field, one contract, one root action. Two God objects
/// are comparable exactly when their contexts are equal — the L2-thread
/// discipline, where changing the field changes the complex.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SalvationContext {
    pub root_id: u64,
    pub field_id: String,
    pub contract: u32,
    pub root_action: Domino,
}

/// Where a God upper's doomed mass came from. The two sources are not
/// interchangeable: the enumeration is the per-world TRUTH (the
/// tightest sound doom mass at that coordinate), the census is a
/// certified — possibly partial — harvest that never exceeds it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DoomSource {
    /// [`doom::doom_enumeration`](crate::solver::doom::doom_enumeration):
    /// every world of the fiber checked for a world-aware make.
    PerWorldTruth { nodes: u64 },
    /// [`doom::doom_census`](crate::solver::doom::doom_census): the
    /// class walk's certified mass under the declared budget.
    CertifiedCensus {
        nodes: u64,
        refused_mass: u128,
        whole_fiber: bool,
    },
    /// Neither instrument was affordable: the vacuous upper 1 with no
    /// doom reasoning behind it at all.
    Unattempted,
}

/// A deterministic God upper `U^God = (Z − |D|)/Z` (§55's `GodUpper`,
/// with the doom proof named by [`DoomSource`] rather than carried
/// inline — the proof object lives in `solver::doom`, which this
/// module consumes and never re-implements).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GodUpper {
    pub context: SalvationContext,
    pub doomed_mass: u128,
    pub fiber_mass: u128,
    pub source: DoomSource,
    /// `(Z − doomed)/Z`, exact.
    pub value: BigRational,
}

impl GodUpper {
    /// `d_phys = β(D)` — the physical-doom term of the §8
    /// decomposition.
    pub fn d_phys(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.doomed_mass),
            BigInt::from(self.fiber_mass),
        )
    }

    /// The upper is VACUOUS when no doom was certified: it says
    /// nothing beyond `Q ≤ 1`.
    pub fn vacuous(&self) -> bool {
        self.doomed_mass == 0
    }

    /// The upper as a deterministic proof-state fact. `None` for a
    /// vacuous upper — an honest empty hand, never an installed 1
    /// (the same discipline as
    /// [`doom::census_fact`](crate::solver::doom::census_fact)).
    pub fn fact(&self) -> Option<Fact> {
        if self.vacuous() {
            return None;
        }
        Some(Fact::Bound(BoundFact::upper(
            self.context.root_action,
            self.value.clone(),
            &self.authority(),
            ProofTag::Deterministic,
        )))
    }

    /// The authority string: the instrument and the field identity
    /// travel with the bound.
    pub fn authority(&self) -> String {
        let instrument = match self.source {
            DoomSource::PerWorldTruth { .. } => "truth",
            DoomSource::CertifiedCensus { .. } => "census",
            DoomSource::Unattempted => "none",
        };
        format!("god-upper-v1:{instrument}:{}", self.context.field_id)
    }
}

/// The §36 equality receipt: the four things a God-tightness claim
/// must bind, plus both sides of the equality as exact integer masses
/// over the same fiber. The receipt is EVIDENCE (the house rule), and
/// its two masses were produced by two different recursions — the
/// argmax extraction's own optimum and the frozen-policy evaluator's
/// re-pricing — over the same declared belief.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EqualityReceipt {
    /// The declared belief identity (`uniform-root` in this epoch).
    pub belief_id: String,
    /// The declared objective identity (`pmake-v1` /
    /// `pmake-setting-v1`), fixed by the viewer's parity at this root.
    pub utility_id: String,
    /// `M*` — the extraction's own optimum mass.
    pub extracted_mass: u128,
    /// `M(ρ)` — the independent fixed-policy re-pricing of the
    /// extracted policy. Equal to `extracted_mass` by the §63
    /// re-pricing law, asserted at construction.
    pub repriced_mass: u128,
    /// `|D|` — the doomed mass behind the God upper.
    pub doomed_mass: u128,
    /// `Z` — the fiber mass both sides are taken over.
    pub fiber_mass: u128,
    /// Focal states recorded in the extracted policy's DAG.
    pub policy_states: usize,
}

/// A God-tight policy at one coordinate (§55): the executable policy
/// whose exact lawful value EQUALS the God upper, with the receipt
/// binding the equality and the score profile that persists it into a
/// proof state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GodTightPolicy {
    pub context: SalvationContext,
    /// The extracted policy's content-addressed id.
    pub policy_id: String,
    /// The policy's exact value — equal to `god_upper`.
    pub value: BigRational,
    pub god_upper: BigRational,
    pub equality_receipt: EqualityReceipt,
    /// The persistable profile fact (present when the profile was
    /// affordable): installing it makes the closure's own executable
    /// lower meet the God upper.
    pub profile: Option<Box<ScoreProfileFact>>,
}

impl GodTightPolicy {
    /// God-tightness with NOTHING to save: every world of the fiber is
    /// physically doomed, so the God upper is 0, the exact optimum is
    /// 0, and Theorem 7.1's common intersection is the intersection
    /// over an empty index set — every lawful policy is God-tight
    /// here, and the equality carries no information about blindness.
    /// A census that counted these beside the substantive receipts
    /// would overstate its own finding, so they are labelled and
    /// tallied apart (§38's horizon table).
    pub fn nothing_saveable(&self) -> bool {
        self.equality_receipt.doomed_mass == self.equality_receipt.fiber_mass
    }
}

/// A measured information-consistency price: the exact `Q` sits
/// STRICTLY below the God upper. The witness is mandatory — this
/// variant cannot be constructed without a `Q` witness present;
/// that the witness is exact is gated by G2's independent
/// reproduction through the response recursion (SC-A4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositiveGodGap {
    /// `M*` — the exact optimum mass witnessing `Q`.
    pub q_mass: u128,
    pub q: BigRational,
    /// `Φ = U^God − Q`, strictly positive.
    pub gap: BigRational,
    /// The extracted incumbent and its independent re-pricing, when
    /// extraction was affordable.
    pub incumbent: Option<Incumbent>,
}

/// One executable incumbent policy at a coordinate: its id, its exact
/// re-priced mass, and the resulting `d_policy = Q − V(ρ)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Incumbent {
    pub policy_id: String,
    pub value_mass: u128,
    pub value: BigRational,
    pub d_policy: BigRational,
}

/// Why an instrument was not run at this coordinate (§34/§35 refusal
/// doctrine, in the shape `frontier`/`opening` already ship). Every
/// refusal is typed and travels in the result — a coordinate is never
/// silently dropped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The fiber exceeds the declared exact cap: the full recursion
    /// for `Q` was not attempted.
    ExactValueUnaffordable { fiber: u128, cap: u128 },
    /// The fiber exceeds the declared exact cap: the per-world doom
    /// truth was not attempted (the class census stands in).
    DoomTruthUnaffordable { fiber: u128, cap: u128 },
    /// The class census left mass unwalked under its node budget, so
    /// its harvest is a declared partial one.
    CensusLeftMassRefused { mass: u128 },
    /// Extraction (and therefore `d_policy`) was not attempted at this
    /// coordinate.
    ExtractionUnaffordable { fiber: u128, cap: u128 },
    /// The score profile — the persistable form of a God-tight policy
    /// — was not attempted.
    ProfileUnaffordable { fiber: u128, cap: u128 },
}

/// The §48 result of one coordinate: exactly four types, and the
/// SC-A4 rule is structural — `PositiveGodGap` cannot exist without
/// its exact `Q` witness, and a zero-doom coordinate with no `Q` can
/// only land in `UnknownGodGap`.
///
/// The God upper itself lives on the [`GodGapCoordinate`], which every
/// result type carries — an upper is established at every coordinate,
/// even where it is the vacuous 1, and the result type says what was
/// established BEYOND it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GodGapResult {
    /// `Q = U^God`, receipted.
    GodTightPolicy(Box<GodTightPolicy>),
    /// `Q < U^God`, measured — the exact witness is inside.
    PositiveGodGap(Box<PositiveGodGap>),
    /// A NONVACUOUS deterministic upper stands (positive certified
    /// doom), but no exact `Q` was affordable: the gap beneath it is
    /// unmeasured. The coordinate's refusals say why.
    GodUpper,
    /// Neither an exact `Q` nor any certified doom — the upper is the
    /// vacuous 1 and nothing is claimed (SC-A4's honest floor).
    UnknownGodGap,
}

impl GodGapResult {
    /// The short type name, for census tables.
    pub fn label(&self) -> &'static str {
        match self {
            GodGapResult::GodTightPolicy(_) => "GodTightPolicy",
            GodGapResult::PositiveGodGap(_) => "PositiveGodGap",
            GodGapResult::GodUpper => "GodUpper",
            GodGapResult::UnknownGodGap => "UnknownGodGap",
        }
    }
}

/// The §8 three-part decomposition as recorded per coordinate. The
/// second and third terms are `None` exactly when the instrument that
/// produces them was refused — an absent number is never a zero.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decomposition {
    pub d_phys: BigRational,
    pub d_info: Option<BigRational>,
    pub d_policy: Option<BigRational>,
}

/// What one coordinate's census cost, in the instruments' own units.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GodGapCost {
    /// Per-world doom search nodes (enumeration), or class-walk nodes
    /// (census).
    pub doom_nodes: u64,
    /// Focal nodes of the exact response recursion.
    pub response_focal: u64,
    pub response_hidden: u64,
    /// Focal nodes of the extraction walk.
    pub extraction_focal: u64,
    /// Nodes of the independent fixed-policy re-pricing.
    pub repricing_nodes: u64,
}

/// One census coordinate: its God upper, its typed result, its
/// decomposition, its bill, and every refusal it accumulated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GodGapCoordinate {
    pub context: SalvationContext,
    pub fiber_mass: u128,
    /// The deterministic God upper — established at every coordinate,
    /// vacuous where no doom was certified.
    pub upper: GodUpper,
    pub result: GodGapResult,
    pub decomposition: Decomposition,
    pub refusals: Vec<Refusal>,
    pub cost: GodGapCost,
}

impl GodGapCoordinate {
    /// The extracted God-tight policy, when this coordinate has one.
    pub fn god_tight(&self) -> Option<&GodTightPolicy> {
        match &self.result {
            GodGapResult::GodTightPolicy(p) => Some(p),
            _ => None,
        }
    }
}

/// The declared census budget: one affordability cap over the exact
/// instruments, plus the doom spec handed to the consumed producer.
/// Deterministic — two censuses with equal spec and inputs are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GodGapSpec {
    /// Fiber mass at or below which the exact recursions (`Q`, the
    /// per-world doom truth, extraction) are attempted.
    pub exact_fiber_cap: u128,
    /// Fiber mass at or below which the 43-bin score profile — the
    /// persistable form of a God-tight policy — is built. Separate
    /// because the profile has no decided cutoff and therefore costs
    /// materially more than the value recursions (§18's caveat).
    pub profile_fiber_cap: u128,
    /// The spec passed through to `solver::doom`, unchanged.
    pub doom: DoomSpec,
}

// ---------------------------------------------------------------------------
// The census.
// ---------------------------------------------------------------------------

/// The declared objective identity at this root — the viewer's parity
/// against the declaring team, in the proof state's own vocabulary.
fn utility_id(root: &CanonicalRoot, position: &RootPosition) -> &'static str {
    if root.kernel().viewer().team() == position.declaring_team {
        "pmake-v1"
    } else {
        "pmake-setting-v1"
    }
}

/// The legal root actions in tile order — the census walk's domain.
pub fn legal_actions(root: &CanonicalRoot, position: &RootPosition) -> Vec<Domino> {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    let mut out: Vec<Domino> = (0..DominoSet::FULL.len())
        .filter_map(Domino::from_index)
        .filter(|t| legal.contains(*t))
        .collect();
    out.sort_by_key(|t| t.index());
    out
}

/// One root's census context: the evaluation authorities and the
/// declared budget, constant across every coordinate of the root.
pub struct GodGapWalk<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
    pub spec: &'a GodGapSpec,
}

impl GodGapWalk<'_> {
    /// The God upper at one coordinate: the per-world truth where the
    /// fiber is enumerable, the class census's certified harvest
    /// otherwise. Both instruments come from `solver::doom` unmodified.
    fn god_upper(
        &self,
        context: &SalvationContext,
        fiber: u128,
        refusals: &mut Vec<Refusal>,
        cost: &mut GodGapCost,
        progress: &mut dyn FnMut(u64, u64, u128, u64),
    ) -> GodUpper {
        let action = context.root_action;
        if fiber <= self.spec.exact_fiber_cap {
            let e = doom_enumeration(
                self.oracle,
                self.root,
                self.position,
                self.field,
                action,
                &self.spec.doom,
                progress,
            );
            assert_eq!(e.fiber, fiber, "the enumeration covers the census fiber");
            cost.doom_nodes = e.nodes;
            return GodUpper {
                context: context.clone(),
                doomed_mass: e.doomed,
                fiber_mass: e.fiber,
                source: DoomSource::PerWorldTruth { nodes: e.nodes },
                value: e.upper,
            };
        }
        refusals.push(Refusal::DoomTruthUnaffordable {
            fiber,
            cap: self.spec.exact_fiber_cap,
        });
        let census = doom_census(
            self.oracle,
            self.root,
            self.position,
            self.field,
            action,
            &self.spec.doom,
        );
        assert_eq!(census.fiber, fiber, "the census fiber is the coordinate's");
        cost.doom_nodes = census.nodes;
        if census.refused_mass > 0 {
            refusals.push(Refusal::CensusLeftMassRefused {
                mass: census.refused_mass,
            });
        }
        GodUpper {
            context: context.clone(),
            doomed_mass: census.doomed_mass,
            fiber_mass: census.fiber,
            source: DoomSource::CertifiedCensus {
                nodes: census.nodes,
                refused_mass: census.refused_mass,
                whole_fiber: census.whole_fiber,
            },
            value: census.upper,
        }
    }
}

impl GodGapWalk<'_> {
    /// One coordinate of the §40 census: establish the God upper, the
    /// exact `Q` where affordable, the extracted incumbent where
    /// affordable, and the typed §48 result they determine.
    ///
    /// `progress` is forwarded to the per-world enumeration verbatim (a
    /// reporting hook only, no effect on the result).
    pub fn god_gap(
        &self,
        action: Domino,
        progress: &mut dyn FnMut(u64, u64, u128, u64),
    ) -> GodGapCoordinate {
        let oracle = self.oracle;
        let root = self.root;
        let position = self.position;
        let field = self.field;
        let spec = self.spec;
        let context = SalvationContext {
            root_id: root_identity(root, position),
            field_id: field.id().to_string(),
            contract: position.bid,
            root_action: action,
        };
        let belief = FactorBelief::uniform_root(root, position, field);
        let fiber = oracle.mass(&belief);
        assert!(fiber > 0, "a census coordinate has positive belief mass");
        let mut refusals: Vec<Refusal> = Vec::new();
        let mut cost = GodGapCost::default();
        let upper = self.god_upper(&context, fiber, &mut refusals, &mut cost, progress);
        let d_phys = upper.d_phys();

        // The exact Q — the one number that turns an upper into a gap.
        if fiber > spec.exact_fiber_cap {
            refusals.push(Refusal::ExactValueUnaffordable {
                fiber,
                cap: spec.exact_fiber_cap,
            });
            refusals.push(Refusal::ExtractionUnaffordable {
                fiber,
                cap: spec.exact_fiber_cap,
            });
            // SC-A4: no exact Q, so nothing may be called a gap. The two
            // honest types split on whether the doom side produced a
            // nonvacuous upper at all.
            let result = if upper.vacuous() {
                GodGapResult::UnknownGodGap
            } else {
                GodGapResult::GodUpper
            };
            return GodGapCoordinate {
                context,
                fiber_mass: fiber,
                upper,
                result,
                decomposition: Decomposition {
                    d_phys,
                    d_info: None,
                    d_policy: None,
                },
                refusals,
                cost,
            };
        }

        let child = belief.focal_play(action);
        let mut rstats = ResponseStats::default();
        let q_mass = response_success_mass(oracle, &child, field, &mut rstats);
        cost.response_focal = rstats.focal_nodes;
        cost.response_hidden = rstats.hidden_nodes;
        let q = BigRational::new(BigInt::from(q_mass), BigInt::from(fiber));
        assert!(
            q <= upper.value,
            "a doom upper never sits below the exact information-consistent optimum: \
         Q = {q_mass}/{fiber} against U^God = {}",
            upper.value
        );
        let d_info = &upper.value - &q;

        // The executable incumbent: the argmax DAG, re-priced by the
        // independent fixed-policy evaluator (§63's re-pricing law is what
        // makes the number a receipt rather than a restatement).
        let mut estats = ResponseStats::default();
        let (extracted_mass, policy) = extract_success_policy(
            oracle,
            &child,
            &ExtractionSource::FullLegal,
            field,
            &mut estats,
        );
        cost.extraction_focal = estats.focal_nodes;
        assert_eq!(
            extracted_mass, q_mass,
            "the §63 extraction attains the exact optimum it extracted from"
        );
        let mut pstats = RecursionStats::default();
        let repriced = viewer_success_mass(oracle, &child, &policy, field, &mut pstats);
        cost.repricing_nodes = pstats.focal_nodes + pstats.hidden_nodes;
        assert_eq!(
            repriced, extracted_mass,
            "the §63 re-pricing gate: the extracted policy re-prices to its extraction mass"
        );
        let value = BigRational::new(BigInt::from(repriced), BigInt::from(fiber));
        let d_policy = &q - &value;
        let incumbent = Incumbent {
            policy_id: policy.id().to_string(),
            value_mass: repriced,
            value,
            d_policy: d_policy.clone(),
        };
        let decomposition = Decomposition {
            d_phys,
            d_info: Some(d_info.clone()),
            d_policy: Some(d_policy),
        };

        if d_info == BigRational::from_integer(BigInt::from(0)) {
            // God-tight: the executable lower MEETS the deterministic
            // doom upper. Persist the profile where affordable.
            let profile = if fiber <= spec.profile_fiber_cap {
                let mut ps = RecursionStats::default();
                let sp = viewer_score_profile(oracle, &child, &policy, field, &mut ps);
                let z = sp.total();
                assert_eq!(z, fiber, "a score profile conserves the fiber mass");
                let tail = sp.tail(position.bid);
                let projected = match utility_id(root, position) {
                    "pmake-v1" => tail,
                    _ => z - tail,
                };
                assert_eq!(
                    projected, repriced,
                    "the profile projects to the policy's re-priced mass"
                );
                Some(Box::new(ScoreProfileFact {
                    action,
                    policy_id: policy.id().to_string(),
                    bins: sp.bins,
                }))
            } else {
                refusals.push(Refusal::ProfileUnaffordable {
                    fiber,
                    cap: spec.profile_fiber_cap,
                });
                None
            };
            let tight = GodTightPolicy {
                context: context.clone(),
                policy_id: incumbent.policy_id.clone(),
                value: incumbent.value.clone(),
                god_upper: upper.value.clone(),
                equality_receipt: EqualityReceipt {
                    belief_id: "uniform-root".to_string(),
                    utility_id: utility_id(root, position).to_string(),
                    extracted_mass,
                    repriced_mass: repriced,
                    doomed_mass: upper.doomed_mass,
                    fiber_mass: fiber,
                    policy_states: policy.states(),
                },
                profile,
            };
            return GodGapCoordinate {
                context,
                fiber_mass: fiber,
                upper,
                result: GodGapResult::GodTightPolicy(Box::new(tight)),
                decomposition,
                refusals,
                cost,
            };
        }

        GodGapCoordinate {
            context,
            fiber_mass: fiber,
            upper,
            result: GodGapResult::PositiveGodGap(Box::new(PositiveGodGap {
                q_mass,
                q,
                gap: d_info,
                incumbent: Some(incumbent),
            })),
            decomposition,
            refusals,
            cost,
        }
    }

    /// The census walk over one root: every legal root action, in tile
    /// order, each coordinate typed and decomposed. Nothing is dropped
    /// — the returned vector has one entry per legal action, refusals
    /// included.
    pub fn census(&self, progress: &mut dyn FnMut(u64, u64, u128, u64)) -> Vec<GodGapCoordinate> {
        legal_actions(self.root, self.position)
            .into_iter()
            .map(|action| self.god_gap(action, progress))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// The producer.
// ---------------------------------------------------------------------------

/// The facts one coordinate persists: the deterministic God upper (when
/// nonvacuous) and, for a God-tight coordinate, the profile of the
/// policy that meets it. Installing both makes the equality visible in
/// the closure itself — the action's lower meets its upper.
pub fn coordinate_facts(coordinate: &GodGapCoordinate) -> Vec<Fact> {
    let mut facts = Vec::new();
    if let Some(fact) = coordinate.upper.fact() {
        facts.push(fact);
    }
    if let Some(tight) = coordinate.god_tight() {
        if let Some(profile) = &tight.profile {
            facts.push(Fact::Profile(profile.clone()));
        }
    }
    facts
}

/// The §49 God-gap producer: one census per legal root action, its
/// deterministic upper and its God-tight profile installed through the
/// open registry. Idempotent against the append-only store — a fact
/// already present is proposed once, never duplicated.
pub struct GodGapProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
    pub spec: GodGapSpec,
}

impl ProofProducer for GodGapProducer<'_> {
    fn name(&self) -> &str {
        "god-gap-v1"
    }

    fn produce(&self, state: &ProofState) -> Vec<Fact> {
        assert_eq!(
            state.identity.root_id,
            root_identity(self.root, self.position),
            "the producer's context is the state's root"
        );
        assert_eq!(
            state.identity.contract, self.position.bid,
            "the producer's contract is the state's"
        );
        let mut out = Vec::new();
        let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
        let walk = GodGapWalk {
            oracle: self.oracle,
            root: self.root,
            position: self.position,
            field: self.field,
            spec: &self.spec,
        };
        for action in &state.legal {
            let coordinate = walk.god_gap(*action, &mut progress);
            for fact in coordinate_facts(&coordinate) {
                if state.facts().iter().any(|sf| sf.fact == fact) {
                    continue;
                }
                out.push(fact);
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// The fusion horizon (§38) — an empirical object over a declared corpus.
// ---------------------------------------------------------------------------

/// One trick depth's stratum of the census (§38): how many coordinates
/// were tested, how many were God-tight, and which were not. A
/// stratum is FUSION-FREE when every tested coordinate is God-tight —
/// on the declared corpus, and never as a theorem (SC-A4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FusionStratum {
    /// Remaining tricks at the root (the census's depth coordinate).
    pub trick: usize,
    pub tested: usize,
    pub god_tight: usize,
    /// Of `god_tight`, the DEGENERATE ones: whole-fiber doom, where
    /// every policy is God-tight and the equality says nothing
    /// ([`GodTightPolicy::nothing_saveable`]).
    pub god_tight_vacuous: usize,
    pub positive_gap: usize,
    pub god_upper_only: usize,
    pub unknown: usize,
    /// The coordinates that are not God-tight, labelled
    /// `<root>:<action>:<result type>` in census order — the
    /// exceptions the horizon claim must state.
    pub exceptions: Vec<String>,
    /// The largest measured `Φ` in this stratum, when any coordinate
    /// measured one.
    pub max_gap: Option<BigRational>,
}

impl FusionStratum {
    /// Every tested coordinate is God-tight — §38's condition,
    /// verbatim.
    pub fn fusion_free(&self) -> bool {
        self.tested > 0 && self.god_tight == self.tested
    }

    /// Fusion-free with at least one SUBSTANTIVE God-tight coordinate
    /// (something was actually saveable there). A stratum that is
    /// fusion-free only because everything in it is doomed carries no
    /// evidence about the information price, and the horizon reading
    /// says so.
    pub fn substantively_fusion_free(&self) -> bool {
        self.fusion_free() && self.god_tight > self.god_tight_vacuous
    }
}

/// Stratify a census by trick depth. Entries are
/// `(trick, root_label, coordinate)`; the table comes back in
/// increasing trick order (earliest depth first), which is the order
/// the §38 horizon is read in.
pub fn fusion_horizon(entries: &[(usize, String, GodGapCoordinate)]) -> Vec<FusionStratum> {
    let mut tricks: Vec<usize> = entries.iter().map(|(t, _, _)| *t).collect();
    tricks.sort_unstable();
    tricks.dedup();
    tricks
        .into_iter()
        .map(|trick| {
            let mut stratum = FusionStratum {
                trick,
                tested: 0,
                god_tight: 0,
                god_tight_vacuous: 0,
                positive_gap: 0,
                god_upper_only: 0,
                unknown: 0,
                exceptions: Vec::new(),
                max_gap: None,
            };
            for (t, label, coordinate) in entries.iter().filter(|(t, _, _)| *t == trick) {
                assert_eq!(*t, trick, "the stratum filter selects its own trick");
                stratum.tested += 1;
                match &coordinate.result {
                    GodGapResult::GodTightPolicy(t) => {
                        stratum.god_tight += 1;
                        if t.nothing_saveable() {
                            stratum.god_tight_vacuous += 1;
                        }
                    }
                    GodGapResult::PositiveGodGap(p) => {
                        stratum.positive_gap += 1;
                        stratum.max_gap = Some(match stratum.max_gap.take() {
                            Some(m) if m >= p.gap => m,
                            _ => p.gap.clone(),
                        });
                    }
                    GodGapResult::GodUpper => stratum.god_upper_only += 1,
                    GodGapResult::UnknownGodGap => stratum.unknown += 1,
                }
                if coordinate.god_tight().is_none() {
                    stratum.exceptions.push(format!(
                        "{label}:{}:{}",
                        coordinate.context.root_action,
                        coordinate.result.label()
                    ));
                }
            }
            stratum
        })
        .collect()
}

/// The §38 horizon itself: the earliest tested depth from which every
/// deeper tested stratum (this one included) is fusion-free. `None`
/// when no such depth exists on the declared corpus. This is §38's
/// condition verbatim, degenerate coordinates included — read it
/// beside each stratum's `god_tight_vacuous` count, since a stratum of
/// nothing-but-doom satisfies the condition while evidencing nothing. Depth increases
/// with the trick number — trick 6 is the latest, so the walk runs
/// from the deepest stratum backwards and stops at the first
/// exception.
pub fn earliest_fusion_free_trick(strata: &[FusionStratum]) -> Option<usize> {
    let mut ordered: Vec<&FusionStratum> = strata.iter().collect();
    ordered.sort_by_key(|s| core::cmp::Reverse(s.trick));
    let mut earliest = None;
    for stratum in ordered {
        if !stratum.fusion_free() {
            break;
        }
        earliest = Some(stratum.trick);
    }
    earliest
}
