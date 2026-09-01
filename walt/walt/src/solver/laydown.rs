//! The §16/§17/§64 laydown producer (anytime proof-state Phase 7):
//! the typed laydown hierarchy — four DISTINCT universal results that
//! must never be blurred into each other or inferred from sampled
//! `pmake = 1` (§64's law: no sampled route constructs any laydown
//! type; every fact this module installs is `ProofTag::Deterministic`,
//! gated). The quantifiers, from weakest to strongest:
//!
//! - `PolicyCertainMake(π, σ)` (§16.1): `∀ω: S ≥ c` for ONE
//!   materialized policy against the DECLARED field — exactly
//!   `viewer_success_mass(π) = Z`, the existing exact recursion.
//! - `AdversarialPolicyMake(π)` (§16.2): `∀ω ∀σ_legal` — the fixed
//!   policy survives every compatible world and every legal defense
//!   ([`factor_belief::universal_viewer_success`], `Fixed`).
//! - `ForcedMake` (§16.3): `∃π ∀ω ∀σ` — the focal side can force the
//!   objective (`Exists`; the walk's per-history witness IS an
//!   information-consistent policy, so the existential is honest).
//! - `Laydown` (§16.4): `∀ω ∀π_legal ∀σ_legal` — every legal
//!   continuation succeeds; the hand could be exposed (`All`).
//!
//! All four are VIEWER-OBJECTIVE (the parity travels through
//! `decided_success`): a setting viewer's `Laydown` is the dual —
//! every legal continuation sets — the same universal shape. §17's
//! zero-cost closure is the walk's own first line: a root whose §5
//! arithmetic already decides the objective classifies with NO walk
//! (the "already-made" case, gated with a zero-node census).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §15–§17 (loss from perfection, the typed hierarchy, zero-cost or
//! structural closure), §64 (the Phase 7 gate list), under ruling
//! APS-A9 (phases in-crate, RefineV1 frozen).
//!
//! MODULE GRAPH. New-core beside `solver::proof_state` (the
//! `extraction`/`residual`/`covers` pattern): imported by nothing but
//! the crate root, deletable with its siblings (§67.10).
//!
//! WHAT THIS PRODUCER IS NOT. No structural shortcuts: the universal
//! properties are proved by the actual walk from the actual rules and
//! state, never from a phrase like "seven trumps" (§64) — which also
//! bounds its domain: the walk is exponential in remaining plays, so
//! it is an ENDGAME instrument (affordable at the fixture depths the
//! gates and probe declare), and an opening-depth laydown certificate
//! needs a future structural producer (§17's second route), not a
//! bigger budget here. One declared policy per run for the two
//! π-relative tiers.

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use crate::solver::factor_belief::{
    universal_viewer_success, viewer_success_mass, ExactCoverOracle, FactorBelief, FocalQuantifier,
    RecursionStats, ResponseStats,
};
use crate::solver::proof_state::{BoundFact, Fact, ProofProducer, ProofState, ProofTag};
use num_rational::BigRational;
use num_traits::One;

/// The §16 census of one root state under one declared policy: the
/// four typed results, each with its own quantifier, plus the
/// existential witness where one exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LaydownCensus {
    /// §16.4 `∀ω ∀π ∀σ` — every legal continuation succeeds.
    pub laydown: bool,
    /// §16.3 `∃π ∀ω ∀σ`.
    pub forced_make: bool,
    /// The forcing root action (the walk's root OR-witness), when
    /// `forced_make` — the lowest tile index that forces.
    pub forced_witness: Option<Domino>,
    /// §16.2 `∀ω ∀σ` for the declared policy.
    pub adversarial_policy_make: bool,
    /// §16.1 `∀ω` for the declared policy against the DECLARED field.
    pub policy_certain_make: bool,
    /// Walk census: nodes the three universal walks visited in total
    /// (zero exactly on the §17 zero-cost path — gated).
    pub universal_nodes: u64,
}

/// Classify one root state (§16): the three universal walks plus the
/// exact fixed-policy mass. Deterministic throughout; nothing sampled
/// exists on any path (§64).
pub fn classify_root(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    policy: &dyn SlicePolicy,
) -> LaydownCensus {
    let belief = FactorBelief::uniform_root(root, position, field);
    let mut us = ResponseStats::default();
    let laydown = universal_viewer_success(&belief, &FocalQuantifier::All, &mut us);
    let forced_witness = first_forcing_action(&belief, &mut us);
    let forced_make = forced_witness.is_some()
        || universal_viewer_success(&belief, &FocalQuantifier::Exists, &mut us);
    let adversarial_policy_make =
        universal_viewer_success(&belief, &FocalQuantifier::Fixed(policy), &mut us);
    let mut vs = RecursionStats::default();
    let mass = viewer_success_mass(oracle, &belief, policy, field, &mut vs);
    let policy_certain_make = mass == oracle.mass(&belief);
    // The hierarchy is a theorem; hold it as an internal fence too.
    assert!(!laydown || adversarial_policy_make, "§16.4 implies §16.2");
    assert!(!laydown || forced_make, "§16.4 implies §16.3");
    assert!(
        !adversarial_policy_make || forced_make,
        "§16.2 implies §16.3"
    );
    assert!(
        !adversarial_policy_make || policy_certain_make,
        "§16.2 implies §16.1 (the declared field plays legally)"
    );
    LaydownCensus {
        laydown,
        forced_make,
        forced_witness,
        adversarial_policy_make,
        policy_certain_make,
        universal_nodes: us.focal_nodes + us.hidden_nodes + us.decided_early + us.decided_terminal,
    }
}

/// The lowest root action that forces the objective (`∃π` with this
/// root move): the root layer of the `Exists` walk, made explicit so
/// the witness is nameable. `None` when no action forces — or when
/// the root is not the viewer's move or is already decided, where a
/// root-action witness is not the right shape (the plain `Exists`
/// walk covers those).
fn first_forcing_action(belief: &FactorBelief, stats: &mut ResponseStats) -> Option<Domino> {
    use crate::rules::{legal_plays, DominoSet};
    let viewer = belief.kernel().viewer();
    if belief.seat_to_move() != viewer {
        return None;
    }
    let remaining = belief.kernel().viewer_hand();
    let legal = legal_plays(belief.position().decl, remaining, None);
    let mut tiles: Vec<Domino> = (0..DominoSet::FULL.len())
        .filter_map(Domino::from_index)
        .filter(|d| legal.contains(*d))
        .collect();
    tiles.sort_by_key(|d| d.index());
    tiles
        .into_iter()
        .find(|t| universal_viewer_success(&belief.focal_play(*t), &FocalQuantifier::Exists, stats))
}

/// The §16/§17 laydown producer over one root: classify, then install
/// what the hierarchy certifies as deterministic lower facts. A
/// `Laydown` puts an EXECUTABLE lower 1 on every legal action (any
/// materialized policy is a witness — the declared one travels in the
/// authority); `AdversarialPolicyMake`/`PolicyCertainMake` put an
/// executable lower 1 on the declared policy's root choice (strongest
/// applicable authority only); a bare `ForcedMake` puts a
/// NON-executable lower 1 on its witness action (§30: existence is
/// proof-bar until an argmax policy is materialized). Closure then
/// settles at once — §17's "pmake closes immediately".
pub struct LaydownProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
    /// The declared policy for the π-relative tiers, and the Laydown
    /// executable witness.
    pub policy: &'a dyn SlicePolicy,
}

impl ProofProducer for LaydownProducer<'_> {
    fn name(&self) -> &str {
        "laydown-v1"
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
        let census = classify_root(
            self.oracle,
            self.root,
            self.position,
            self.field,
            self.policy,
        );
        let one = BigRational::one();
        let mut facts = Vec::new();
        if census.laydown {
            for a in &state.legal {
                facts.push(Fact::Bound(BoundFact::lower(
                    *a,
                    one.clone(),
                    &format!("laydown-v1:laydown:{}", self.policy.id()),
                    true,
                    ProofTag::Deterministic,
                )));
            }
            return facts;
        }
        if census.adversarial_policy_make || census.policy_certain_make {
            let record_choice = {
                use crate::rules::legal_plays;
                use crate::solver::adaptive::PublicRecord;
                let remaining = self.root.kernel().viewer_hand();
                let legal = legal_plays(self.position.decl, remaining, None);
                let record = PublicRecord {
                    leader: self.position.leader,
                    trick_plays: &[],
                    banked: self.position.banked,
                    root: self.position,
                    history: &[],
                };
                let tile = self
                    .policy
                    .choose(self.position.decl, remaining, legal, &record);
                assert!(legal.contains(tile), "a policy chooses a legal tile");
                tile
            };
            let tier = if census.adversarial_policy_make {
                "adversarial-policy"
            } else {
                "policy-certain"
            };
            facts.push(Fact::Bound(BoundFact::lower(
                record_choice,
                one.clone(),
                &format!("laydown-v1:{tier}:{}", self.policy.id()),
                true,
                ProofTag::Deterministic,
            )));
        } else if let Some(witness) = census.forced_witness {
            facts.push(Fact::Bound(BoundFact::lower(
                witness,
                one,
                "laydown-v1:forced-make",
                false,
                ProofTag::Deterministic,
            )));
        }
        facts
    }
}
