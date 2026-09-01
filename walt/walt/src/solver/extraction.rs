//! The §63 extraction producer (anytime proof-state Phase 6) — the
//! first shipped [`ProofProducer`]: per legal root action, extract the
//! exact best-response continuation policy
//! ([`factor_belief::extract_success_policy`] over the full legal set),
//! re-price it through the fixed-policy score evaluator
//! ([`factor_belief::viewer_score_profile`]), and install the profile
//! under the extracted policy's content id. A profile fact IS an
//! executable witness (its policy is materialized by construction), so
//! this producer is how the executable bar rises to meet the proof bar
//! and the certified regret `Γ = U* − B_exec` collapses at
//! exactly-settled roots (§30's chain closing; the Phase 3 probe's
//! h3-t4 finding is the motivating specimen).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §30 (the argmax-extraction bridge into `B_exec`), §63 (the Phase 6
//! gate list), under rulings APS-A9 (phases in-crate, RefineV1 frozen)
//! and APS-A4 (the profile installed is ONE realizable policy's — the
//! §20 fence; the policy id is a content address of the choice table,
//! never of an envelope).
//!
//! MODULE GRAPH. This module is new-core beside `solver::proof_state`
//! (§47's greenfield boundary): it imports the proof-state registry
//! and the factor-belief recursions and is imported by NOTHING except
//! the crate root — the pair stays deletable without touching RefineV1
//! or the recursions (§67.10).
//!
//! WHAT THIS PRODUCER IS NOT. No scheduler and no budget: it extracts
//! every root action unconditionally, which is the AMPLE configuration
//! — §33 work-item selection (extract only where the gap pays) belongs
//! to Phase 1's frontier. No upper bounds: extraction certifies what a
//! materialized policy attains; uppers on the unknown best response
//! stay with RefineV1's facts and later §36 producers. No grammar
//! variant here: the producer's action source is the full legal set
//! (the §36 EscalateExact domain); grammar-source extraction exists in
//! the library and is gated, and a grammar-restricted PRODUCER would
//! need its own class identity on the facts it installs.

use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::factor_belief::{
    extract_success_policy, viewer_score_profile, ExactCoverOracle, ExtractionSource, FactorBelief,
    RecursionStats, ResponseStats,
};
use crate::solver::proof_state::{Fact, ProofProducer, ProofState, ScoreProfileFact};

/// The §63 argmax-extraction producer over one root. Holds its own
/// evaluation context (oracle, root, position, field) and asserts at
/// produce time that the proof state's identity names the same root
/// and contract — a producer never installs across identities (§51;
/// the install fence would reject it anyway, but the mismatch here is
/// a caller bug worth failing loudly).
pub struct ExtractionProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
}

/// The §63 extraction of ONE root action's continuation, packaged as
/// the installable profile fact: extract the exact best-response DAG
/// after `action`, re-price it through the fixed-policy evaluator,
/// hold the re-pricing receipt inline (a completion that carried
/// objective weight would fail here), and return the profile under
/// the extracted policy's content id. Shared by the ample producer
/// below and the Phase 1 frontier's targeted `ExtractArgmax` item —
/// one §63 code path.
pub fn extraction_fact_for_action(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    contract: u32,
    utility_id: &str,
    action: crate::rules::Domino,
) -> Fact {
    let belief = FactorBelief::uniform_root(root, position, field).focal_play(action);
    let mut stats = ResponseStats::default();
    let (mass, policy) = extract_success_policy(
        oracle,
        &belief,
        &ExtractionSource::FullLegal,
        field,
        &mut stats,
    );
    let mut ps = RecursionStats::default();
    let profile = viewer_score_profile(oracle, &belief, &policy, field, &mut ps);
    let z = profile.total();
    let tail = profile.tail(contract);
    let projected = match utility_id {
        "pmake-v1" => tail,
        "pmake-setting-v1" => z - tail,
        other => panic!("an unknown utility identity: {other}"),
    };
    assert_eq!(
        projected, mass,
        "the §63 re-pricing gate: the extracted policy re-prices to its extraction mass"
    );
    Fact::Profile(Box::new(ScoreProfileFact {
        action,
        policy_id: policy.id().to_string(),
        bins: profile.bins,
    }))
}

impl ProofProducer for ExtractionProducer<'_> {
    fn name(&self) -> &str {
        "argmax-extraction-v1"
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
            .map(|a| {
                extraction_fact_for_action(
                    self.oracle,
                    self.root,
                    self.position,
                    self.field,
                    state.identity.contract,
                    &state.identity.utility_id,
                    *a,
                )
            })
            .collect()
    }
}
