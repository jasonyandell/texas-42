//! Gates for the §49 architecture spike [L2 thread]: the persistent
//! proof state (`solver::proof_state`) — the zero-budget top state is
//! sound and serializes/resumes bytewise (gate 1), imported RefineV1
//! facts reproduce the G controller's survivors, exclusions, bar, and
//! typed result on every enumerable root under both the exact-only and
//! two-tier configurations (gate 2), closure is idempotent and
//! insertion-order-independent (gate 3), identity mismatches and
//! malformed facts are rejected while stored facts round-trip
//! serialization exactly (gate 4), a score-profile fact raises the
//! EXECUTABLE bar through closure with `B_exec ≤ B_proof` (gate 5),
//! and a producer registers from OUTSIDE the module — no enum edited —
//! proving the open registry (gate 6).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §24–§31, §41, §49–§56, adopted by ruling APS-A9
//! (`walt/CENSUS-RULINGS.md`); RefineV1 is freeze 58 and is consumed
//! here strictly as the frozen reference oracle.
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; RefineV1 run at the refine gates' own ample
//! configurations (exact-only prefix 0, two-tier prefix 16, δ = 1/20,
//! scope 1/2). Frozen `verify_player` receipt roots: the six
//! enumerable fibers.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::{
    viewer_score_profile, viewer_success_mass, FactorBelief, RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    facts_from_refine_interval, BoundFact, Fact, ProofProducer, ProofState, ProofTag, Reject,
    ScoreProfileFact, SemanticsIdentity, StateResult,
};
use walt::solver::refine::{refine_root, ProofClass, RefineConfig, RefineResult};

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

fn ample_exact() -> RefineConfig {
    RefineConfig {
        budget: u64::MAX / 2,
        prefix: 0,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    }
}

fn ample_two_tier() -> RefineConfig {
    RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    }
}

/// The declared identity of one spike root: the viewer parity travels
/// in the utility id, everything else is the declared epoch.
fn identity_of(root: &CanonicalRoot, position: &RootPosition) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
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
    }
}

fn set_of(actions: &[Domino]) -> DominoSet {
    let mut s = DominoSet::EMPTY;
    for a in actions {
        s.insert(*a);
    }
    s
}

/// Gate 1 — the §25 top state is sound, and the zero-fact state
/// serializes, resumes, and re-serializes bytewise: every legal action
/// survives at `[0, 1]`, the result is the honest Unresolved, and
/// nothing is manufactured at zero work.
#[test]
fn the_zero_budget_state_is_sound_and_serializes_and_resumes() {
    let r = receipt();
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let state = ProofState::open(&root, &position, identity_of(&root, &position));
        let report = state.closure();
        assert_eq!(
            report.survivors, state.legal,
            "every legal action survives at zero work (hand {hand_id} trick {trick_no})"
        );
        assert!(report.excluded.is_empty(), "nothing excludes at zero work");
        assert_eq!(report.bar, BigRational::zero(), "the zero-work bar");
        assert!(
            report.exec_bar.is_none(),
            "no executable witness at zero work"
        );
        assert!(!report.delta_decisive, "no sampled fact at zero work");
        assert!(
            matches!(report.result, StateResult::Unresolved { .. }),
            "the top state is honest, not a failure"
        );
        for v in &report.views {
            assert_eq!(v.lower, BigRational::zero());
            assert_eq!(v.upper, BigRational::one());
        }
        let text = state.serialize();
        let resumed = ProofState::parse(&text, &root, &position).expect("a clean resume");
        assert_eq!(resumed.closure(), report, "a resumed closure is identical");
        assert_eq!(
            resumed.serialize(),
            text,
            "a resumed state re-serializes bytewise"
        );
    }
}

/// Gate 2 — RefineV1 as the frozen oracle (§48 step 4, freeze 58): on
/// every enumerable root, under the exact-only AND two-tier ample
/// configurations, importing the controller's final interval endpoints
/// as facts and closing reproduces its survivors, exclusions, bar, and
/// typed result. Under prefix 0 both sides agree the outcome is exact.
#[test]
fn imported_refine_v1_facts_reproduce_the_controller_verdicts() {
    let r = receipt();
    let oracle = SupportOracle;
    let spec = level0_spec();
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        for (cfg, label) in [(ample_exact(), "exact"), (ample_two_tier(), "two-tier")] {
            let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
            let mut state = ProofState::open(&root, &position, identity_of(&root, &position));
            let identity = identity_of(&root, &position);
            for interval in &outcome.intervals {
                for fact in facts_from_refine_interval(interval) {
                    state.install(&identity, fact).expect("a V1 fact installs");
                }
            }
            let report = state.closure();
            let tag = format!("hand {hand_id} trick {trick_no} {label}");
            assert_eq!(set_of(&report.survivors), outcome.survivors, "{tag}");
            assert_eq!(set_of(&report.excluded), outcome.excluded, "{tag}");
            assert_eq!(report.bar, outcome.bar, "{tag}");
            match &outcome.result {
                RefineResult::Settled { action, proof } => {
                    assert_eq!(
                        report.result,
                        StateResult::Settled { action: *action },
                        "{tag}"
                    );
                    if matches!(proof, ProofClass::Exact) {
                        assert!(!report.delta_decisive, "{tag}: exact stays exact");
                    }
                }
                RefineResult::Equivalent {
                    actions,
                    value,
                    proof,
                } => {
                    assert_eq!(
                        report.result,
                        StateResult::Equivalent {
                            actions: report.survivors.clone(),
                            value: value.clone(),
                        },
                        "{tag}"
                    );
                    assert_eq!(set_of(&report.survivors), *actions, "{tag}");
                    if matches!(proof, ProofClass::Exact) {
                        assert!(!report.delta_decisive, "{tag}: exact stays exact");
                    }
                }
                RefineResult::Unresolved { survivors, .. } => {
                    assert_eq!(
                        report.result,
                        StateResult::Unresolved {
                            survivors: report.survivors.clone(),
                        },
                        "{tag}"
                    );
                    assert_eq!(set_of(&report.survivors), *survivors, "{tag}");
                }
            }
            if cfg.prefix == 0 {
                assert!(
                    !outcome.delta_decisive && !report.delta_decisive,
                    "{tag}: an all-exact run is δ-free on both sides"
                );
            }
        }
    }
}

/// Gate 3 — closure is a pure derived view: calling it twice yields
/// equal reports, and installing the same facts in reversed order
/// yields the same closure (the store is append-only; the views are
/// functions of the SET of facts).
#[test]
fn closure_is_idempotent_and_insertion_order_independent() {
    let r = receipt();
    let oracle = SupportOracle;
    let spec = level0_spec();
    let (root, position) = root_at(&r, 3, 5);
    let outcome = refine_root(&root, &position, &spec, &oracle, &ample_two_tier());
    let identity = identity_of(&root, &position);
    let mut facts = Vec::new();
    for interval in &outcome.intervals {
        facts.extend(facts_from_refine_interval(interval));
    }
    let mut forward = ProofState::open(&root, &position, identity.clone());
    for f in facts.iter().cloned() {
        forward.install(&identity, f).expect("installs");
    }
    let mut backward = ProofState::open(&root, &position, identity.clone());
    for f in facts.iter().rev().cloned() {
        backward.install(&identity, f).expect("installs");
    }
    let once = forward.closure();
    assert_eq!(once, forward.closure(), "closure is idempotent");
    assert_eq!(once, backward.closure(), "closure ignores insertion order");
}

/// Gate 4 — the fences and the round trip: a fact under a mismatched
/// identity is rejected in ANY coordinate, malformed values are
/// rejected, unknown actions are rejected — none of them changes the
/// state — and a store holding every fact kind round-trips
/// serialization exactly (§56: the fact lines are content-hashed and
/// re-validated on resume).
#[test]
fn fences_reject_and_stored_facts_round_trip() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity.clone());
    let a = state.legal[0];

    let mut wrong = identity.clone();
    wrong.contract += 1;
    let f = Fact::Bound(BoundFact::lower(
        a,
        q(1, 2),
        "test:lower",
        true,
        ProofTag::Deterministic,
    ));
    assert_eq!(
        state.install(&wrong, f.clone()),
        Err(Reject::IdentityMismatch),
        "a contract mismatch rejects"
    );
    let mut wrong_field = identity.clone();
    wrong_field.field_id = "some-other-field".to_string();
    assert_eq!(
        state.install(&wrong_field, f.clone()),
        Err(Reject::IdentityMismatch),
        "a field mismatch rejects"
    );
    assert_eq!(
        state.install(
            &identity,
            Fact::Bound(BoundFact::lower(
                a,
                q(3, 2),
                "test:lower",
                true,
                ProofTag::Deterministic,
            ))
        ),
        Err(Reject::MalformedValue),
        "a bound above one rejects"
    );
    let illegal = (0..28)
        .filter_map(Domino::from_index)
        .find(|d| !state.legal.contains(d))
        .expect("some tile is not legal at this root");
    assert_eq!(
        state.install(
            &identity,
            Fact::Bound(BoundFact::lower(
                illegal,
                q(1, 2),
                "test:lower",
                true,
                ProofTag::Deterministic,
            ))
        ),
        Err(Reject::UnknownAction),
        "an action outside the root rejects"
    );
    assert!(state.facts().is_empty(), "rejections change nothing");

    state.install(&identity, f).expect("a lower installs");
    state
        .install(
            &identity,
            Fact::Bound(BoundFact::upper(
                a,
                q(9, 10),
                "test:upper",
                ProofTag::Sampled {
                    scope: "test-scope/upper".to_string(),
                    delta: q(1, 20),
                },
            )),
        )
        .expect("a sampled upper installs");
    // Consistent with the sampled upper above: most mass under the
    // contract (the closure's §37 assert rejects contradictory toys —
    // it fired on a first draft of this test, which is the point).
    let mut bins = [0u128; 43];
    bins[20] = 5;
    bins[42] = 3;
    state
        .install(
            &identity,
            Fact::Profile(Box::new(ScoreProfileFact {
                action: a,
                policy_id: "test-policy".to_string(),
                bins,
            })),
        )
        .expect("a profile installs");
    let text = state.serialize();
    let resumed = ProofState::parse(&text, &root, &position).expect("a clean resume");
    assert_eq!(resumed.facts(), state.facts(), "facts round-trip exactly");
    assert_eq!(resumed.closure(), state.closure(), "views round-trip");
    assert_eq!(resumed.serialize(), text, "bytewise re-serialization");
}

/// Gate 5 — §41 closure-aware derivation and the §30 executable bar: a
/// score-profile fact is not a root bound, but closure projects its
/// 43 bins to a deterministic EXECUTABLE lower at the identity's
/// contract, equal to the independent success-mass evaluation of the
/// same policy — and `B_exec ≤ B_proof` holds (asserted inside every
/// closure; observed here with a live executable bar).
#[test]
fn a_score_profile_fact_raises_the_executable_bar_through_closure() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in [(4usize, 6usize, 90u128), (8, 5, 92)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        let a = state.legal[0];
        let focal = FixedPreference::lowest_first("focal:lowest-first");
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(a);
        let mut stats = RecursionStats::default();
        let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
        assert_eq!(profile.total(), fiber, "the continuation conserves Z");
        state
            .install(
                &identity,
                Fact::Profile(Box::new(ScoreProfileFact {
                    action: a,
                    policy_id: "lowest-first-after-root-action".to_string(),
                    bins: profile.bins,
                })),
            )
            .expect("the profile installs");
        let report = state.closure();
        let field_m = FieldModel::new(level0_spec());
        let belief_m = FactorBelief::uniform_root(&root, &position, &field_m).focal_play(a);
        let mut mstats = RecursionStats::default();
        let mass = viewer_success_mass(&oracle, &belief_m, &focal, &field_m, &mut mstats);
        let expect = BigRational::new(BigInt::from(mass), BigInt::from(fiber));
        let view = report
            .views
            .iter()
            .find(|v| v.action == a)
            .expect("the action has a view");
        assert_eq!(
            view.lower, expect,
            "the projection is the policy's exact success value \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(!view.lower_sampled, "a profile projection is deterministic");
        assert_eq!(
            report.exec_bar,
            Some((a, expect)),
            "the profile witnesses the executable bar (hand {hand_id} trick {trick_no})"
        );
    }
}

/// A producer defined OUTSIDE `solver::proof_state` — the §49 seventh
/// requirement made literal. It proves the §5 arithmetic score floor:
/// when the declaring team has already banked the contract, EVERY
/// lawful continuation makes (banked points only accumulate), so every
/// root action carries a deterministic executable lower of 1.
struct BankedFloorProducer {
    declaring_banked: u32,
    contract: u32,
    declaring_viewer: bool,
}

impl ProofProducer for BankedFloorProducer {
    fn name(&self) -> &str {
        "test:banked-floor"
    }

    fn produce(&self, state: &ProofState) -> Vec<Fact> {
        if !self.declaring_viewer || self.declaring_banked < self.contract {
            return Vec::new();
        }
        state
            .legal
            .iter()
            .map(|a| {
                Fact::Bound(BoundFact::lower(
                    *a,
                    BigRational::one(),
                    "test:banked-floor",
                    true,
                    ProofTag::Deterministic,
                ))
            })
            .collect()
    }
}

/// Gate 6 — the open registry: the banked-floor producer above lives
/// in THIS test file; registering and running it edits no enum in the
/// module. On a repriced root whose contract is already banked, it
/// closes the state to the exact Equivalent-at-1 tie across every
/// legal action — an §17-flavored structural closure with no
/// best-response solve.
#[test]
fn a_producer_registers_without_editing_the_module() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let declaring_viewer = root.kernel().viewer().team() == position.declaring_team;
    let mut repriced = position.clone();
    repriced.bid = repriced.banked[repriced.declaring_team.index()];
    let mut identity = identity_of(&root, &repriced);
    identity.root_id = root_identity(&root, &repriced);
    identity.contract = repriced.bid;
    let mut state = ProofState::open(&root, &repriced, identity);
    let producer = BankedFloorProducer {
        declaring_banked: repriced.banked[repriced.declaring_team.index()],
        contract: repriced.bid,
        declaring_viewer,
    };
    let results = state.run_producer(&producer);
    if declaring_viewer {
        assert!(!results.is_empty(), "the floor fires on a banked contract");
        assert!(
            results.iter().all(|r| r.is_ok()),
            "every floor fact installs"
        );
        let report = state.closure();
        assert_eq!(
            report.result,
            StateResult::Equivalent {
                actions: state.legal.clone(),
                value: BigRational::one(),
            },
            "an already-banked contract closes to the exact all-action tie at 1"
        );
        assert!(!report.delta_decisive, "a structural floor is exact");
        assert_eq!(
            report.exec_bar.map(|(_, v)| v),
            Some(BigRational::one()),
            "the floor is executable: any lawful policy witnesses it"
        );
    } else {
        assert!(
            results.is_empty(),
            "the producer declines on a setting viewer (it may decline — CBS-A7)"
        );
    }
}
