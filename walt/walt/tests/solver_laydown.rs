//! Gates for the anytime proof-state Phase 7 [L2 thread] — the §16
//! typed laydown hierarchy: the four result types are proved from the
//! actual rules and state on STRUCTURAL fixtures — a boss-chain
//! control hand is a true `Laydown`, the "already-made" arithmetic
//! case classifies at ZERO walk nodes (§17's zero-cost closure), and
//! the deliberate near-laydown counterexamples (the boss trump loose
//! in the pool; a possible ruff of a natural boss) break every
//! universal tier (gates 1–2); the hierarchy implications hold and
//! `PolicyCertainMake` coincides exactly with the existing
//! `viewer_success_mass = Z` on the receipt roots (gate 3); and the
//! producer installs only `ProofTag::Deterministic` facts — no
//! sampled route constructs any laydown type (§64's law) — with the
//! closure settling immediately on a certified state (gate 4).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §15–§17, §64, under ruling APS-A9 (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle` for the field-relative tier; declared policy =
//! lowest-first; synthetic fixtures at three remaining tricks (the
//! walk is an endgame instrument — its exponential domain is the
//! declared boundary); receipt roots = the t6 enumerable fibers (the
//! universal walks' affordable receipt domain).

mod common;

use common::receipt;
use num_rational::BigRational;
use num_traits::One;
use walt::kernel::{Hidden, Kernel};
use walt::rules::receipt::Receipt;
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Pip, Seat};
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::{
    viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::laydown::{classify_root, LaydownProducer};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{Fact, ProofState, ProofTag, SemanticsIdentity, StateResult};

fn d(a: usize, b: usize) -> Domino {
    Domino::new(Pip::ALL[a], Pip::ALL[b])
}

fn set(tiles: &[Domino]) -> DominoSet {
    let mut s = DominoSet::EMPTY;
    for t in tiles {
        s.insert(*t);
    }
    s
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

/// A synthetic three-trick root: the viewer leads with `viewer_hand`,
/// the 9-tile `pool` covers three hidden seats of capacity 3, no void
/// knowledge, banked points forced by arithmetic (the §5 identity:
/// banked total = 42 − points still in play). The banked split gives
/// the declaring side `decl_banked`.
fn synthetic_root(
    decl: Decl,
    viewer_hand: &[Domino],
    pool: &[Domino],
    bid: u32,
    decl_banked: u32,
) -> (CanonicalRoot, RootPosition) {
    let viewer = Seat::ALL[0];
    let hand = set(viewer_hand);
    let pool = set(pool);
    assert_eq!(hand.len(), 3, "three remaining tricks");
    assert_eq!(pool.len(), 9, "three hidden seats of capacity three");
    let in_play = hand.union(pool);
    let points_in_play: u32 = 3 + in_play.iter().map(|t| t.count()).sum::<u32>();
    let banked_total = 42 - points_in_play;
    assert!(decl_banked <= banked_total, "a consistent banked split");
    let hidden = [
        Hidden {
            seat: Seat::ALL[1],
            capacity: 3,
            voids: ContextSet::EMPTY,
        },
        Hidden {
            seat: Seat::ALL[2],
            capacity: 3,
            voids: ContextSet::EMPTY,
        },
        Hidden {
            seat: Seat::ALL[3],
            capacity: 3,
            voids: ContextSet::EMPTY,
        },
    ];
    let kernel = Kernel::new(decl, viewer, hand, pool, hidden).expect("a lawful kernel");
    let position = RootPosition {
        decl,
        bid,
        declaring_team: viewer.team(),
        leader: viewer,
        banked: [decl_banked, banked_total - decl_banked],
        trick_plays: vec![],
        prior_played: DominoSet::FULL.difference(in_play),
        voids: [ContextSet::EMPTY; 4],
    };
    (CanonicalRoot::new(kernel), position)
}

/// The boss-chain control: viewer holds the three highest sixes
/// (6-6 double-top, 6-5, 6-4) with every other trump in the pool
/// ranking below 6-4 — every legal viewer play leads an unbeatable
/// trump, so every continuation banks all 13 remaining points.
fn control_fixture(bid: u32) -> (CanonicalRoot, RootPosition) {
    synthetic_root(
        Decl::PipTrump(Pip::ALL[6]),
        &[d(6, 6), d(6, 5), d(6, 4)],
        &[
            d(6, 3),
            d(6, 2),
            d(6, 1),
            d(6, 0),
            d(5, 4),
            d(5, 3),
            d(5, 2),
            d(5, 1),
            d(4, 3),
        ],
        bid,
        20,
    )
}

/// Gate 1 — the control fixture is a TRUE `Laydown` at the
/// all-or-nothing contract (bid = banked + everything in play): every
/// tier true, witness named, and the census walked real nodes (this
/// is proved from the rules, not the phrase "seven trumps" — §64).
#[test]
fn the_boss_chain_control_is_a_laydown() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let (root, position) = control_fixture(33);
    let census = classify_root(&oracle, &root, &position, &field, &low);
    assert!(census.laydown, "every legal continuation makes");
    assert!(census.forced_make, "§16.4 implies §16.3");
    assert!(census.adversarial_policy_make, "§16.4 implies §16.2");
    assert!(census.policy_certain_make, "§16.2 implies §16.1");
    assert_eq!(
        census.forced_witness,
        Some(d(6, 4)),
        "the lowest-index forcing action is named"
    );
    assert!(
        census.universal_nodes > 0,
        "the certificate came from a real walk, not arithmetic"
    );
}

/// Gate 2 — §17's zero-cost closure and the §64 counterexamples: an
/// already-made root classifies with ZERO walk nodes; one loose boss
/// trump breaks every universal tier at the all-or-nothing contract;
/// and a possible ruff of a natural boss chain breaks them too.
#[test]
fn zero_cost_arithmetic_and_the_near_laydown_counterexamples() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let low = FixedPreference::lowest_first("focal:lowest-first");

    // Already made: bid at or below the declaring side's bank.
    let (root, position) = control_fixture(20);
    let census = classify_root(&oracle, &root, &position, &field, &low);
    assert!(
        census.laydown && census.forced_make && census.policy_certain_make,
        "an already-made root is every tier at once"
    );
    assert_eq!(
        census.universal_nodes, 3,
        "three §17 zero-cost classifications — one decided root read per walk, zero tree"
    );

    // The boss loose in the pool: some world holds 6-6 over the
    // viewer's 6-5, and at bid 33 a single lost point fails.
    let (root, position) = synthetic_root(
        Decl::PipTrump(Pip::ALL[6]),
        &[d(6, 5), d(6, 4), d(6, 3)],
        &[
            d(6, 6),
            d(6, 2),
            d(6, 1),
            d(6, 0),
            d(5, 4),
            d(5, 3),
            d(5, 2),
            d(5, 1),
            d(4, 3),
        ],
        33,
        20,
    );
    let census = classify_root(&oracle, &root, &position, &field, &low);
    assert!(
        !census.laydown && !census.forced_make && !census.adversarial_policy_make,
        "one loose boss breaks every universal tier"
    );

    // The ruff: natural five-suit bosses under a six-trump
    // declaration — a world where an opponent is void in fives and
    // holds the 6-0 trump ruffs the 5-5 lead.
    let (root, position) = synthetic_root(
        Decl::PipTrump(Pip::ALL[6]),
        &[d(5, 5), d(5, 4), d(5, 3)],
        &[
            d(6, 0),
            d(5, 2),
            d(5, 1),
            d(4, 3),
            d(4, 2),
            d(3, 1),
            d(2, 1),
            d(2, 0),
            d(1, 0),
        ],
        33,
        20,
    );
    let census = classify_root(&oracle, &root, &position, &field, &low);
    assert!(
        !census.laydown && !census.forced_make,
        "one possible ruff breaks universality"
    );
}

/// Gate 3 — the hierarchy on receipt roots: the implication chain
/// holds (asserted inside `classify_root` and re-checked here), and
/// `PolicyCertainMake` coincides EXACTLY with the existing
/// `viewer_success_mass = Z` — the §16.1 tier is the recursion the
/// crate already trusted.
#[test]
fn the_hierarchy_holds_on_receipt_roots() {
    let r = receipt();
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let mut certain_seen = false;
    let mut uncertain_seen = false;
    for (hand_id, trick_no) in [(12usize, 6usize), (10, 6), (5, 6), (4, 6)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let census = classify_root(&oracle, &root, &position, &field, &low);
        assert!(!census.laydown || census.adversarial_policy_make);
        assert!(!census.adversarial_policy_make || census.forced_make);
        assert!(!census.adversarial_policy_make || census.policy_certain_make);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let mut vs = RecursionStats::default();
        let mass = viewer_success_mass(&oracle, &belief, &low, &field, &mut vs);
        assert_eq!(
            census.policy_certain_make,
            mass == oracle.mass(&belief),
            "§16.1 IS the exact fixed-policy recursion at Z"
        );
        if census.policy_certain_make {
            certain_seen = true;
        } else {
            uncertain_seen = true;
        }
    }
    assert!(
        certain_seen && uncertain_seen,
        "both census directions are live on the receipt roots"
    );
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// Gate 4 — §64's law and §17's immediate closure: every fact the
/// producer installs is `ProofTag::Deterministic` (no sampled route
/// to any laydown type), the control state closes SETTLED-or-tied at
/// bar 1 with an executable laydown witness, and a counterexample
/// state yields no laydown-tier fact at all.
#[test]
fn the_producer_is_deterministic_and_closes_immediately() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let (root, position) = control_fixture(33);
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity);
    let producer = LaydownProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        policy: &low,
    };
    let results = state.run_producer(&producer);
    assert_eq!(results.len(), state.legal.len(), "one fact per action");
    assert!(results.iter().all(|r| r.is_ok()));
    for sf in state.facts() {
        let Fact::Bound(b) = &sf.fact else {
            panic!("the laydown producer speaks bounds only")
        };
        assert_eq!(b.proof, ProofTag::Deterministic, "no sampled route (§64)");
        assert!(b.authority.starts_with("laydown-v1:laydown:"));
        assert!(
            b.executable,
            "a laydown's witness is any materialized policy"
        );
        assert_eq!(b.value, BigRational::one());
    }
    let report = state.closure();
    assert_eq!(
        report.bar,
        BigRational::one(),
        "pmake closes immediately (§17)"
    );
    assert_eq!(report.certified_regret, BigRational::from_integer(0.into()));
    assert!(matches!(
        report.result,
        StateResult::Settled { .. } | StateResult::Equivalent { .. }
    ));

    // The loose-boss counterexample: no universal tier, no laydown
    // fact — the producer may still certify nothing at all.
    let (root, position) = synthetic_root(
        Decl::PipTrump(Pip::ALL[6]),
        &[d(6, 5), d(6, 4), d(6, 3)],
        &[
            d(6, 6),
            d(6, 2),
            d(6, 1),
            d(6, 0),
            d(5, 4),
            d(5, 3),
            d(5, 2),
            d(5, 1),
            d(4, 3),
        ],
        33,
        20,
    );
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity);
    let producer = LaydownProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        policy: &low,
    };
    let results = state.run_producer(&producer);
    for r in &results {
        assert!(r.is_ok());
    }
    assert!(
        state
            .facts()
            .iter()
            .all(|sf| !matches!(&sf.fact, Fact::Bound(b) if b.authority.starts_with("laydown-v1:laydown"))),
        "no laydown fact without the universal property"
    );
}
