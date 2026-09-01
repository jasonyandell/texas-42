//! Gates for the anytime proof-state Phase 8 [L2 thread] — the §65
//! opening-root iterative run: the zero-budget stop is the §25 top
//! state with a deterministic serialization round trip (gate 1); the
//! sampled ladder narrows monotonically with an exact derived risk
//! ledger and an idempotent presence guard (gate 2); resume is
//! semantically identical to uninterrupted refinement — §67.5, panel
//! for panel and byte for byte (gate 3); the opening root's
//! affordability cliff is honest — the frontier pass buys nothing,
//! installs nothing, and never manufactures a winner (§34/§66.14,
//! gate 4); and the same ladder settles enumerable roots exactly under
//! an ample frontier budget, with a consistent §65 panel and the §49
//! census coordinate conserving mass (gate 5).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §25–§31, §34, §39–§43, §48, §65, §66.3/§66.14, §67.4–5, under
//! ruling APS-A9 (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; sampled tier at the Slice A declaration (upper
//! epoch 0, evaluation epoch 1), δ = 1/100 per endpoint, root scope
//! budget 1/2, ladder ε = 1/4. Frozen `verify_player` receipt roots:
//! enumerable fibers for the ladder gates, the opening root h0-t1 for
//! the cliff gate.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::frontier::Refusal;
use walt::solver::opening::{OpeningLadder, OpeningStopSpec, StopVerdict};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{Fact, ProofState, ProofTag, SemanticsIdentity};

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

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn sampled_stop(label: &str, prefix: u64) -> OpeningStopSpec {
    OpeningStopSpec {
        label: label.to_string(),
        sampled_prefix: prefix,
        endpoint_delta: q(1, 100),
        census: false,
        frontier_budget: 0,
    }
}

/// Gate 1 — the §25 top state at the zero-budget stop, on every
/// enumerable root and the opening root: no facts, every legal action
/// alive at `[0, 1]`, bar 0, `U* = 1`, `Γ = 1`, no recommendation, an
/// honest `Unresolved` verdict, width debt exactly `|legal|` — and the
/// §67.4 round trip: serialize, parse, re-serialize bytewise equal.
#[test]
fn zero_budget_stop_is_the_top_state_with_deterministic_serialization() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut roots: Vec<(usize, usize)> = ENUM_ROOTS.iter().map(|(h, t, _)| (*h, *t)).collect();
    roots.push((0, 1));
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let ladder = OpeningLadder {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            scope_budget: q(1, 2),
            epsilon: q(1, 4),
        };
        let mut state = ProofState::open(&root, &position, identity_of(&root, &position));
        let report = ladder.run_stop(&mut state, &sampled_stop("zero", 0));
        assert_eq!(report.facts, 0, "the top state holds no facts");
        assert_eq!(report.survivors.len(), state.legal.len());
        assert!(report.excluded.is_empty());
        assert_eq!(report.proof_bar, BigRational::zero());
        assert_eq!(report.exec_bar, BigRational::zero());
        assert_eq!(report.global_upper, BigRational::one());
        assert_eq!(report.certified_regret, BigRational::one());
        assert_eq!(
            report.width_debt,
            BigRational::from_integer(BigInt::from(state.legal.len() as u64)),
            "every interval is the vacuous [0, 1]"
        );
        assert!(report.recommendation.is_none(), "no executable work yet");
        assert_eq!(report.verdict, StopVerdict::Unresolved);
        assert_eq!(report.risk_spent, BigRational::zero());
        assert_eq!(report.policy_cylinders, 0);
        assert_eq!(report.count_threat_cells, 0);
        let text = state.serialize();
        let resumed = ProofState::parse(&text, &root, &position).expect("the round trip parses");
        assert_eq!(resumed.serialize(), text, "bytewise round trip (§67.4)");
    }
}

/// Gate 2 — the sampled ladder on enumerable roots: across stops the
/// bar never falls, the global upper never rises, certified regret
/// never rises, survivors never grow; every sampled fact carries its
/// full scoped provenance (§66.3); the derived risk ledger is exactly
/// `scopes × δ` and every scope is distinct; the executable pinned
/// witness yields a recommendation; and re-running a stop imports
/// nothing twice (the presence guard).
#[test]
fn sampled_ladder_narrows_monotonically_with_exact_risk_ledger() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, _) in [(3usize, 5usize, 200u128), (8, 5, 92)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let ladder = OpeningLadder {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            scope_budget: q(1, 2),
            epsilon: q(1, 4),
        };
        let mut state = ProofState::open(&root, &position, identity_of(&root, &position));
        let legal = state.legal.len() as u64;
        let mut last_bar = BigRational::zero();
        let mut last_upper = BigRational::one();
        let mut last_regret = BigRational::one();
        let mut last_survivors = state.legal.len();
        let mut stops_run = 0u64;
        for (label, prefix) in [("p16", 16u64), ("p32", 32)] {
            let report = ladder.run_stop(&mut state, &sampled_stop(label, prefix));
            stops_run += 1;
            assert!(report.proof_bar >= last_bar, "the bar is monotone (§37)");
            assert!(report.global_upper <= last_upper, "U* only falls");
            assert!(
                report.certified_regret <= last_regret,
                "Γ is monotone nonincreasing under refinement (§31)"
            );
            assert!(
                report.survivors.len() <= last_survivors,
                "exclusions are permanent (§34)"
            );
            last_bar = report.proof_bar.clone();
            last_upper = report.global_upper.clone();
            last_regret = report.certified_regret.clone();
            last_survivors = report.survivors.len();
            assert_eq!(report.sampled_work, 2 * prefix * legal);
            assert_eq!(
                report.risk_scopes.len() as u64,
                2 * legal * stops_run,
                "two fresh scopes per action per stop"
            );
            assert_eq!(
                report.risk_spent,
                q(1, 100)
                    * BigRational::from_integer(BigInt::from(report.risk_scopes.len() as u64)),
                "the derived ledger is exactly scopes × δ"
            );
            let rec = report
                .recommendation
                .expect("a pinned witness is executable");
            assert!(rec.sampled, "the witness is δ-qualified");
            assert_eq!(rec.certified_regret, report.certified_regret);
            assert_eq!(
                report.policy_cylinders,
                state.legal.len(),
                "one pinned-witness cylinder per action, prefix-independent"
            );
        }
        // §66.3: every sampled fact carries a complete scoped identity.
        for sf in state.facts() {
            let Fact::Bound(b) = &sf.fact else {
                panic!("the sampled ladder installs bound facts only")
            };
            let ProofTag::Sampled { scope, delta } = &b.proof else {
                panic!("every ladder fact is sampled")
            };
            assert!(scope.starts_with("opening-"), "the declared scope family");
            assert_eq!(*delta, q(1, 100));
        }
        // The presence guard: the same stop again imports nothing.
        let facts_before = state.facts().len();
        let again = ladder.run_stop(&mut state, &sampled_stop("p32-again", 32));
        assert_eq!(again.sampled_work, 0, "a resumed stop re-imports nothing");
        assert_eq!(state.facts().len(), facts_before);
    }
}

/// Gate 3 — §67.5: resume is semantically identical to uninterrupted
/// refinement. Serialize mid-ladder, parse into a fresh state, run the
/// remaining stop on both; the §65 panels agree field for field and
/// the final serializations agree byte for byte.
#[test]
fn resume_is_semantically_identical_to_uninterrupted_refinement() {
    let r = receipt();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 3, 5);
    let field = FieldModel::new(level0_spec());
    let ladder = OpeningLadder {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        scope_budget: q(1, 2),
        epsilon: q(1, 4),
    };
    let mut uninterrupted = ProofState::open(&root, &position, identity_of(&root, &position));
    ladder.run_stop(&mut uninterrupted, &sampled_stop("p16", 16));
    let checkpoint = uninterrupted.serialize();
    let mut resumed =
        ProofState::parse(&checkpoint, &root, &position).expect("the checkpoint parses");
    let final_spec = OpeningStopSpec {
        label: "p32+frontier".to_string(),
        sampled_prefix: 32,
        endpoint_delta: q(1, 100),
        census: true,
        frontier_budget: 200 * 3 * 20,
    };
    let a = ladder.run_stop(&mut uninterrupted, &final_spec);
    let b = ladder.run_stop(&mut resumed, &final_spec);
    assert_eq!(a, b, "the resumed panel is the uninterrupted panel");
    assert_eq!(
        uninterrupted.serialize(),
        resumed.serialize(),
        "the resumed store is the uninterrupted store, byte for byte"
    );
}

/// Gate 4 — the opening root's affordability cliff is honest (§34,
/// §40, §66.14): at h0-t1 under a frontier budget of `Z/2` the pass
/// buys nothing, spends nothing, installs nothing — every refusal is
/// zero-potential or unaffordable, at least one is unaffordable (the
/// cliff, not the stall), the survivor set stays complete, and the
/// verdict is the honest `Unresolved` — budget exhaustion never
/// manufactures a winner.
#[test]
fn opening_root_frontier_pass_is_an_honest_cliff() {
    let r = receipt();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 0, 1);
    let field = FieldModel::new(level0_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let z = oracle.mass(&belief);
    assert_eq!(z, 399_072_960, "the opening fiber");
    let ladder = OpeningLadder {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        scope_budget: q(1, 2),
        epsilon: q(1, 4),
    };
    let mut state = ProofState::open(&root, &position, identity_of(&root, &position));
    let report = ladder.run_stop(
        &mut state,
        &OpeningStopSpec {
            label: "cliff".to_string(),
            sampled_prefix: 0,
            endpoint_delta: q(1, 100),
            census: false,
            frontier_budget: z / 2,
        },
    );
    let f = report.frontier.expect("the frontier pass ran");
    assert!(f.executed.is_empty(), "nothing is affordable at the cliff");
    assert_eq!(f.spent, 0);
    assert!(!f.met);
    assert!(!f.refusals.is_empty());
    assert!(
        f.refusals
            .iter()
            .all(|(_, r)| matches!(r, Refusal::ZeroPotential | Refusal::Unaffordable)),
        "every refusal is typed and honest"
    );
    assert!(
        f.refusals
            .iter()
            .any(|(_, r)| matches!(r, Refusal::Unaffordable)),
        "the cliff: positive-potential items exceed the budget"
    );
    assert_eq!(report.facts, 0, "a refused pass installs nothing");
    assert_eq!(report.survivors.len(), state.legal.len());
    assert_eq!(report.verdict, StopVerdict::Unresolved);
    assert_eq!(report.covers_installed, 0, "no incumbent, no cover (§62)");
}

/// Gate 5 — the same ladder settles enumerable roots: under an ample
/// frontier budget the §65 run reaches an exact typed verdict, the
/// panel is internally consistent (`B_exec ≤ B ≤ U*`,
/// `Γ = U* − B_exec`), and the §49 census coordinate conserves mass
/// per action at the declared stage.
#[test]
fn ample_ladder_settles_enumerable_roots_with_consistent_panels() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, z) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let ladder = OpeningLadder {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            scope_budget: q(1, 2),
            epsilon: q(1, 4),
        };
        let mut state = ProofState::open(&root, &position, identity_of(&root, &position));
        let report = ladder.run_stop(
            &mut state,
            &OpeningStopSpec {
                label: "ample".to_string(),
                sampled_prefix: 0,
                endpoint_delta: q(1, 100),
                census: true,
                frontier_budget: z * 3 * 30,
            },
        );
        let f = report.frontier.as_ref().expect("the frontier pass ran");
        assert!(f.met, "an ample budget settles an enumerable root");
        assert!(
            matches!(
                report.verdict,
                StopVerdict::Exact | StopVerdict::EpsilonOptimal
            ),
            "no sampled fact took part — the verdict is deterministic"
        );
        assert!(!report.delta_decisive);
        assert!(report.exec_bar <= report.proof_bar);
        assert!(report.proof_bar <= report.global_upper);
        assert_eq!(
            report.certified_regret,
            &report.global_upper - &report.exec_bar,
            "Γ = U* − B_exec (§31)"
        );
        assert_eq!(report.census.len(), state.legal.len());
        for c in &report.census {
            assert_eq!(
                c.exact_mass + c.residual_mass,
                z,
                "the census conserves the fiber mass at every stage (§49)"
            );
            assert!(c.exact_classes <= c.classes);
        }
        assert_eq!(
            report.count_threat_cells, report.covers_installed,
            "first pass: every stored cover is this stop's"
        );
    }
}
