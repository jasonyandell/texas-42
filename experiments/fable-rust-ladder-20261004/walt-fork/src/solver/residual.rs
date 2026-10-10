//! The §61 residual-Bellman producer (anytime proof-state Phase 4):
//! per legal root action, run the score-aware residual Bellman at the
//! DECLARED F stage ([`factor_belief::staged_response_interval`] —
//! exact §36 response on the merged exact branches, §5 envelope on the
//! unresolved mass) and install the resulting root interval as a
//! non-executable lower and an upper under the residual-Bellman
//! authority. This is what makes Slice F's consequence census
//! CONSUMABLE (§41's canonical example): before this producer the
//! census had provably zero root effect; now every stage of the
//! staircase is a root interval the closure reads.
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §61 (the Phase 4 gate list), §22 (interval recursion over
//! unresolved field classes), §23 (merge before focal maximization),
//! §5 (the arithmetic envelope), §9 (monotone refinement — the
//! staircase nests), under rulings APS-A9 (phases in-crate, RefineV1
//! frozen) and APS-A8 (closure-aware usefulness).
//!
//! MODULE GRAPH. New-core beside `solver::proof_state` (§47's
//! greenfield boundary), the `solver::extraction` pattern: imports the
//! proof-state registry and the factor-belief recursions, imported by
//! nothing but the crate root and `solver::frontier` (the targeted
//! `ResidualInterval` work item shares one §61 code path) — deletable
//! with its siblings without touching RefineV1 or the recursions
//! (§67.10).
//!
//! WHAT THIS PRODUCER IS NOT. No stage scheduling: it runs the one
//! DECLARED stage below on every legal action — which stage pays at
//! which node is Phase 8's affordability question, and at today's
//! declared costs the exact §36 item dominates it at equal forecast
//! (recorded honestly in the frontier; the staged item's cost
//! advantage arrives with a staged cost model, not by fiat). Its
//! lower is NEVER executable — the §30 split: exact branch optima are
//! proof-bar values until an argmax policy is extracted and re-priced.

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::factor_belief::{
    staged_response_interval, ExactCoverOracle, FactorBelief, ResponseStats,
};
use crate::solver::proof_state::{BoundFact, Fact, ProofProducer, ProofState, ProofTag};

/// The declared F stage the producer and the frontier's targeted item
/// run: enough witness refinements to concentrate most fixture mass in
/// exact classes (the Slice F census's neighborhood), few enough that
/// the envelope stays load-bearing. A declared constant in the §40
/// forecast sense — never measured, tunable only by a later phase.
pub const RESIDUAL_STAGE: usize = 4;

/// The authority every §61 interval fact carries.
pub const RESIDUAL_AUTHORITY: &str = "residual-bellman-v1";

/// The §61 interval of ONE root action at the declared stage, packaged
/// as installable facts: a non-executable lower and an upper on the
/// action's §36 best-response value. Shared by the ample producer
/// below and the frontier's targeted `ResidualInterval` item — one
/// §61 code path.
pub fn residual_interval_facts_for_action(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    action: Domino,
) -> Vec<Fact> {
    let child = FactorBelief::uniform_root(root, position, field).focal_play(action);
    let mut stats = ResponseStats::default();
    let interval = staged_response_interval(oracle, &child, field, RESIDUAL_STAGE, &mut stats);
    let z = oracle.mass(&child);
    assert!(
        interval.lower <= interval.upper && interval.upper <= z,
        "a staged interval is ordered inside the branch mass"
    );
    let rational = |m: u128| BigRational::new(BigInt::from(m), BigInt::from(z));
    vec![
        Fact::Bound(BoundFact::lower(
            action,
            rational(interval.lower),
            RESIDUAL_AUTHORITY,
            false,
            ProofTag::Deterministic,
        )),
        Fact::Bound(BoundFact::upper(
            action,
            rational(interval.upper),
            RESIDUAL_AUTHORITY,
            ProofTag::Deterministic,
        )),
    ]
}

/// The §61 residual-Bellman producer over one root: the declared stage
/// on every legal action, unconditionally (the AMPLE configuration —
/// targeted selection is the frontier's `ResidualInterval` item).
pub struct ResidualIntervalProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
}

impl ProofProducer for ResidualIntervalProducer<'_> {
    fn name(&self) -> &str {
        "residual-bellman-v1"
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
        state
            .legal
            .iter()
            .flat_map(|a| {
                residual_interval_facts_for_action(
                    self.oracle,
                    self.root,
                    self.position,
                    self.field,
                    *a,
                )
            })
            .collect()
    }
}
