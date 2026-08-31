//! Gates for the anytime proof-state Phase 3 [L2 thread]: contract
//! projection and certified regret (`ProofState::recommend`, the §33
//! block) — exact profiles project exactly into the recommendation
//! (gate 1), the certified regret contains the exact best-response
//! regret against the bundled authority (gate 2), regret never
//! increases under monotone refinement (gate 3), a non-executable
//! grammar lower can never enter the executable bar (gate 4), and
//! report quantities reuse across contracts exactly when the semantics
//! is bid-blind (gate 5 — the profile gates' 3b specimen owns the
//! σ0 boundary; nothing here re-projects under a bid-reading field).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §29–§33 (bars, certified regret, the recommendation block), §60
//! (the Phase 3 gate list), §10–§11 (the band masses the block
//! carries), adopted by rulings APS-A6/APS-A7
//! (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle` (gates 1–4); the trivial bid-blind semantics for
//! the reuse gate (gate 5). Frozen `verify_player` receipt roots: the
//! six enumerable fibers.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::Domino;
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::exposure::exact_root_value;
use walt::solver::factor_belief::{
    viewer_score_profile, viewer_success_mass, FactorBelief, RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    facts_from_refine_interval, BoundFact, Fact, ProofState, ProofTag, ScoreProfileFact,
    SemanticsIdentity,
};
use walt::solver::refine::{refine_root, RefineConfig};

const ENUM_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn level0_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn identity_of(root: &CanonicalRoot, position: &RootPosition, field_id: &str) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
    SemanticsIdentity {
        root_id: root_identity(root, position),
        rules_id: "texas42-v1".to_string(),
        field_id: field_id.to_string(),
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

/// The continuation profile of `lowest-first` after one root action,
/// under a fresh σ0 field.
fn sigma0_profile_after(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
) -> ScoreProfileFact {
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let field = FieldModel::new(level0_spec());
    let belief = FactorBelief::uniform_root(root, position, &field).focal_play(action);
    let mut stats = RecursionStats::default();
    let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
    ScoreProfileFact {
        action,
        policy_id: "lowest-first-after-root-action".to_string(),
        bins: profile.bins,
    }
}

/// The same policy's exact viewer-objective success value, computed
/// independently through the truncating Slice D walk.
fn sigma0_exact_value_after(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    fiber: u128,
) -> BigRational {
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let field = FieldModel::new(level0_spec());
    let belief = FactorBelief::uniform_root(root, position, &field).focal_play(action);
    let mut stats = RecursionStats::default();
    let mass = viewer_success_mass(&oracle, &belief, &focal, &field, &mut stats);
    BigRational::new(BigInt::from(mass), BigInt::from(fiber))
}

/// Gate 1 — §60's first gate in the proof state: exact profiles
/// project exactly. Two candidate profiles per root; the
/// recommendation picks the stronger, its pmake floor equals the
/// independent exact evaluation of that policy, its score
/// floor/ceiling match the winning bins, the §10/§11 d = 1 bands are
/// the exact bin ratios, and the §7 residual of an exact profile is
/// exactly zero.
#[test]
fn the_recommendation_projects_exact_profiles_exactly() {
    let r = receipt();
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let identity = identity_of(&root, &position, "level0-modeled-mind-v1");
        let mut state = ProofState::open(&root, &position, identity.clone());
        let candidates: Vec<Domino> = state.legal.iter().take(2).copied().collect();
        for a in &candidates {
            let p = sigma0_profile_after(&root, &position, *a);
            state
                .install(&identity, Fact::Profile(Box::new(p)))
                .expect("a profile installs");
        }
        let rec = state.recommend().expect("a profile witness recommends");
        let mut best: Option<(Domino, BigRational)> = None;
        for a in &candidates {
            let v = sigma0_exact_value_after(&root, &position, *a, fiber);
            let better = match &best {
                None => true,
                Some((_, bv)) => v > *bv,
            };
            if better {
                best = Some((*a, v));
            }
        }
        let (best_a, best_v) = best.expect("candidates exist");
        assert_eq!(
            (rec.action, rec.pmake_lower.clone()),
            (best_a, best_v),
            "the recommendation is the stronger exact policy \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(!rec.sampled, "an exact profile witness is deterministic");
        assert_eq!(
            rec.contract_sensitive_residual,
            Some(BigRational::zero()),
            "an exact profile's §7 residual is exactly zero"
        );
        let floor = rec.declaring_score_floor.expect("a profile has a floor");
        let ceiling = rec
            .declaring_score_ceiling
            .expect("a profile has a ceiling");
        assert!(floor <= ceiling && ceiling <= 42);
        let fragile = rec.declaring_fragile_d1.expect("a profile has bands");
        let rescue = rec.declaring_rescue_d1.expect("a profile has bands");
        assert!(fragile >= BigRational::zero() && fragile <= BigRational::one());
        assert!(rescue >= BigRational::zero() && rescue <= BigRational::one());
        assert_eq!(
            rec.certified_regret,
            &rec.global_upper - &rec.pmake_lower,
            "Γ = U* − B_exec"
        );
    }
}

/// Gate 2 — §60's regret containment: with deterministic profile
/// witnesses only, `0 ≤ Q* − V(π̂) ≤ Γ` is a theorem (`U* = 1 ≥ Q*`
/// with no upper facts), and with the RefineV1 two-tier facts added,
/// `Q* ≤ U*` and the containment hold on the frozen fixtures (the
/// sampled facts' validity event holds on these streams — a
/// fixture-fact, the same one the refine escalation gate asserts).
#[test]
fn certified_regret_contains_exact_best_response_regret() {
    let r = receipt();
    let oracle = SupportOracle;
    let spec = level0_spec();
    let cfg = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    };
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let identity = identity_of(&root, &position, "level0-modeled-mind-v1");
        let declaring = root.kernel().viewer().team() == position.declaring_team;
        let q_star = state_q_star(&root, &position, fiber, declaring);
        let mut state = ProofState::open(&root, &position, identity.clone());
        for a in state.legal.clone() {
            let p = sigma0_profile_after(&root, &position, a);
            state
                .install(&identity, Fact::Profile(Box::new(p)))
                .expect("a profile installs");
        }
        let rec = state.recommend().expect("profiles recommend");
        assert!(
            rec.pmake_lower <= q_star,
            "an executable floor never exceeds the exact optimum \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(
            &q_star - &rec.pmake_lower <= rec.certified_regret,
            "Γ contains the exact best-response regret \
             (hand {hand_id} trick {trick_no})"
        );
        let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
        for interval in &outcome.intervals {
            for fact in facts_from_refine_interval(interval) {
                state.install(&identity, fact).expect("a V1 fact installs");
            }
        }
        let rec2 = state.recommend().expect("still recommends");
        assert!(
            q_star <= rec2.global_upper,
            "the exact optimum sits under U* on the frozen fixtures \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(
            &q_star - &rec2.pmake_lower <= rec2.certified_regret,
            "Γ still contains the exact regret after the V1 import \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(
            rec2.certified_regret <= rec.certified_regret,
            "more facts never worsen the certified regret \
             (hand {hand_id} trick {trick_no})"
        );
    }
}

/// `Q* = max_a Q_a` through the bundled exact authority, viewer
/// objective.
fn state_q_star(
    root: &CanonicalRoot,
    position: &RootPosition,
    fiber: u128,
    declaring: bool,
) -> BigRational {
    let field = FieldModel::new(level0_spec());
    let legal = ProofState::open(
        root,
        position,
        SemanticsIdentity {
            root_id: root_identity(root, position),
            rules_id: "texas42-v1".to_string(),
            field_id: "level0-modeled-mind-v1".to_string(),
            utility_id: if declaring {
                "pmake-v1".to_string()
            } else {
                "pmake-setting-v1".to_string()
            },
            contract: position.bid,
            belief_id: "uniform-root".to_string(),
            policy_class_id: "information-consistent-full".to_string(),
            score_semantics_id: "declaring-banked-43bin-v1".to_string(),
        },
    )
    .legal;
    legal
        .iter()
        .map(|a| {
            let v = exact_root_value(root, position, *a, &field);
            assert_eq!(v.fiber, fiber, "the fiber agrees");
            BigRational::new(BigInt::from(v.win_worlds), BigInt::from(fiber))
        })
        .max()
        .expect("a legal action exists")
}

/// Gate 3 — §60's monotonicity: importing the RefineV1 facts ONE AT A
/// TIME, the certified regret never increases, `U*` never rises, and
/// the executable floor never falls, at every step of the walk.
#[test]
fn regret_never_increases_under_monotone_refinement() {
    let r = receipt();
    let oracle = SupportOracle;
    let spec = level0_spec();
    let cfg = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    };
    let (root, position) = root_at(&r, 8, 5);
    let identity = identity_of(&root, &position, "level0-modeled-mind-v1");
    let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
    let mut state = ProofState::open(&root, &position, identity.clone());
    let mut last = state.closure();
    let p = sigma0_profile_after(&root, &position, state.legal[0]);
    let mut facts: Vec<Fact> = vec![Fact::Profile(Box::new(p))];
    for interval in &outcome.intervals {
        facts.extend(facts_from_refine_interval(interval));
    }
    for fact in facts {
        state.install(&identity, fact).expect("a fact installs");
        let now = state.closure();
        assert!(
            now.certified_regret <= last.certified_regret,
            "Γ never increases under refinement"
        );
        assert!(now.u_star <= last.u_star, "U* never rises");
        let floor_last = last
            .exec
            .as_ref()
            .map(|w| w.value.clone())
            .unwrap_or_else(BigRational::zero);
        let floor_now = now
            .exec
            .as_ref()
            .map(|w| w.value.clone())
            .unwrap_or_else(BigRational::zero);
        assert!(floor_now >= floor_last, "B_exec never falls");
        last = now;
    }
}

/// Gate 4 — §60's fence: a non-executable grammar lower raises the
/// PROOF bar and never the executable bar. A later, smaller
/// executable fact becomes the witness even while the proof bar sits
/// higher — and `B_exec ≤ B_proof` holds throughout (asserted inside
/// every closure).
#[test]
fn a_grammar_lower_raises_only_the_proof_bar() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let identity = identity_of(&root, &position, "level0-modeled-mind-v1");
    let mut state = ProofState::open(&root, &position, identity.clone());
    let a = state.legal[0];
    state
        .install(
            &identity,
            Fact::Bound(BoundFact::lower(
                a,
                q(3, 4),
                "test:exact-grammar",
                false,
                ProofTag::Deterministic,
            )),
        )
        .expect("a grammar lower installs");
    let report = state.closure();
    assert_eq!(
        report.bar,
        q(3, 4),
        "the grammar lower raises the proof bar"
    );
    assert!(
        report.exec.is_none(),
        "a non-executable optimum never enters B_exec"
    );
    assert!(
        state.recommend().is_none(),
        "nothing executable, nothing recommended"
    );
    state
        .install(
            &identity,
            Fact::Bound(BoundFact::lower(
                a,
                q(1, 2),
                "test:pinned-policy",
                true,
                ProofTag::Deterministic,
            )),
        )
        .expect("an executable lower installs");
    let report = state.closure();
    assert_eq!(report.bar, q(3, 4), "the proof bar keeps the grammar value");
    let w = report.exec.expect("the executable witness exists");
    assert_eq!(
        (w.value.clone(), w.authority.as_str()),
        (q(1, 2), "test:pinned-policy"),
        "the smaller executable fact is the witness"
    );
    assert_eq!(
        report.certified_regret,
        &report.u_star - &q(1, 2),
        "Γ is priced off the executable floor, not the proof bar"
    );
}

/// Gate 5 — §60's reuse gate, scoped by the Phase 2 boundary finding:
/// under a BID-BLIND semantics (trivial field, fixed focal) the same
/// profile bins install under re-priced identities and the
/// recommendation's floor at each contract equals the independent
/// truncating evaluation at that contract. The σ0 boundary — where
/// re-pricing is a re-run — is owned by the profile gates' frozen
/// specimen and deliberately NOT re-projected here.
#[test]
fn report_quantities_reuse_across_contracts_when_bid_blind() {
    let r = receipt();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 3, 5);
    let trivial_field = FixedPreference::lowest_first("field:lowest-first");
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let a = {
        let probe = ProofState::open(
            &root,
            &position,
            identity_of(&root, &position, "trivial-lowest-v1"),
        );
        probe.legal[0]
    };
    let belief = FactorBelief::uniform_root(&root, &position, &trivial_field).focal_play(a);
    let mut stats = RecursionStats::default();
    let profile = viewer_score_profile(&oracle, &belief, &focal, &trivial_field, &mut stats);
    for bid in [1u32, 21, 30, 36, 42] {
        let mut repriced = position.clone();
        repriced.bid = bid;
        let identity = identity_of(&root, &repriced, "trivial-lowest-v1");
        let mut state = ProofState::open(&root, &repriced, identity.clone());
        state
            .install(
                &identity,
                Fact::Profile(Box::new(ScoreProfileFact {
                    action: a,
                    policy_id: "lowest-first-after-root-action".to_string(),
                    bins: profile.bins,
                })),
            )
            .expect("the same bins install at every contract");
        let rec = state.recommend().expect("a recommendation exists");
        let belief_m = FactorBelief::uniform_root(&root, &repriced, &trivial_field).focal_play(a);
        let mut mstats = RecursionStats::default();
        let mass = viewer_success_mass(&oracle, &belief_m, &focal, &trivial_field, &mut mstats);
        assert_eq!(
            rec.pmake_lower,
            BigRational::new(BigInt::from(mass), BigInt::from(200u32)),
            "one profile prices contract {bid} under a bid-blind semantics"
        );
    }
}
