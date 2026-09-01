//! Gates for the doom census [L2 thread] — counterexample mass as a
//! deterministic upper (`solver::doom`, the §70 structural producer,
//! ∀-fail dual of the §16 hierarchy): an already-set root dooms the
//! WHOLE fiber on the level-0 pre-walk in one decided read (gate 1);
//! on the loose-boss fixture the census meets the exact recursion
//! from above EXACTLY — certified doom mass 1120/1680, upper equal to
//! `response_success_mass / Z` on every action, with certification
//! landing at levels 1 and 2: the per-seat phantom escape is defeated
//! not by a deeper split but by physical tile conservation stranding
//! every phantom hand as the pool drains (gate 2); on enumerable
//! receipt roots every census
//! upper sits at or above the exact best-response value and the leaf
//! masses partition the fiber exactly (gate 3); a starved budget
//! refuses honestly and deterministically (gate 4); and the producer
//! installs `ProofTag::Deterministic` uppers the closure consumes,
//! idempotently, surviving the §67.4 byte round trip (gate 5).
//!
//! Mathematical source: `walt/walt/src/solver/doom.rs` module
//! argument; `walt/math/anytime_proof_state_score_v0.1.md` §16–§17,
//! §28–§31, §46, §49, §70.
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; synthetic fixtures at three remaining tricks;
//! receipt roots = the t5 enumerable fibers (3,5) and (8,5); ample
//! spec n=10_000_000 c=1_000_000 l=3, starved spec n=40 c=20;
//! critical set empty throughout.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{Hidden, Kernel};
use walt::rules::receipt::Receipt;
use walt::rules::{ContextSet, Decl, Domino, DominoSet, Pip, Seat};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::doom::{doom_census, doom_enumeration, DoomCensusProducer, DoomSpec};
use walt::solver::factor_belief::{
    response_success_mass, ExactCoverOracle, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{BoundSide, Fact, ProofState, ProofTag, SemanticsIdentity};

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

fn ample() -> DoomSpec {
    DoomSpec {
        node_budget: 10_000_000,
        walk_cap: 1_000_000,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    }
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// A synthetic three-trick root (the solver_laydown fixture shape):
/// the viewer leads with `viewer_hand`, the 9-tile `pool` covers
/// three hidden seats of capacity 3, banked forced by the §5 identity
/// with the declaring side holding `decl_banked`.
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

/// The loose-boss fixture: the viewer's 6-5/6-4/6-3 chain at the
/// all-or-nothing bid 33 with the 6-6 loose in the pool. Exactly the
/// worlds placing 6-6 with an OPPONENT (seats 1 and 3; seat 2 is the
/// partner) are doomed — the boss wins whichever trick it enters, and
/// one trick point unmakes 33.
fn loose_boss() -> (CanonicalRoot, RootPosition) {
    synthetic_root(
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
    )
}

fn legal_actions(root: &CanonicalRoot, position: &RootPosition) -> Vec<Domino> {
    let legal = walt::rules::legal_plays(position.decl, root.kernel().viewer_hand(), None);
    let mut out: Vec<Domino> = (0..DominoSet::FULL.len())
        .filter_map(Domino::from_index)
        .filter(|t| legal.contains(*t))
        .collect();
    out.sort_by_key(|t| t.index());
    out
}

/// Gate 1 — the §17-dual zero-cost path: a root the opponents have
/// already set (banked past the contract's complement) certifies the
/// WHOLE fiber doomed on the level-0 pre-walk, in one decided read,
/// with upper exactly 0 on every action.
#[test]
fn an_already_set_root_dooms_the_whole_fiber_at_zero_cost() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    // Bid 33 with only 10 of the banked 29 declaring: opponents hold
    // 19 > 42 − 33, so the objective is decided against the viewer
    // before a tile moves.
    let (root, position) = synthetic_root(
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
        33,
        10,
    );
    let z = oracle.mass(&FactorBelief::uniform_root(&root, &position, &field));
    for action in legal_actions(&root, &position) {
        let census = doom_census(&oracle, &root, &position, &field, action, &ample());
        assert!(census.whole_fiber, "the level-0 pre-walk certifies");
        assert_eq!(census.doomed_mass, z, "the whole fiber is doomed");
        assert_eq!(census.nodes, 1, "one decided read, zero tree");
        assert_eq!(census.classes_walked, 1, "no partition was needed");
        assert_eq!(
            census.upper,
            BigRational::from_integer(BigInt::from(0)),
            "the deterministic upper closes pmake at 0"
        );
        assert_eq!(census.doomed_leaves.len(), 1);
        assert!(census.doomed_leaves[0].path.is_empty(), "a level-0 leaf");
    }
}

/// Gate 2 — the census meets the exact recursion from above EXACTLY
/// on the loose-boss fixture: certified doom mass 1120 of 1680 (6-6
/// with either opponent), upper equal to the exact best-response
/// value 560/1680 on every action. The certification pattern is the
/// interesting part: boss-with-first-responder classes certify at
/// level 1; boss-with-third-seat worlds LOOK like they need level 3
/// (the per-seat relaxation admits a phantom in which no seat holds
/// the boss and the viewer makes), yet they certify at level 2 —
/// physical tile conservation strands every phantom: with only five
/// non-trumps in the pool, seats 1 and 2 play all five before seat
/// 3's last turn, so every phantom seat-3 hand collides with a played
/// tile and dies vacuous before any phantom make is reached. The
/// squeeze, not a deeper split, is what closes the walk. Zero-mass
/// joint classes are skipped as empty, never walked.
#[test]
fn the_census_meets_the_exact_recursion_on_the_loose_boss() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let (root, position) = loose_boss();
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let z = oracle.mass(&belief);
    assert_eq!(z, 1680, "C(9,3) · C(6,3) worlds");
    for action in legal_actions(&root, &position) {
        let census = doom_census(&oracle, &root, &position, &field, action, &ample());
        let mut stats = ResponseStats::default();
        let exact = response_success_mass(&oracle, &belief.focal_play(action), &field, &mut stats);
        assert_eq!(
            exact, 560,
            "the exact make worlds place 6-6 with the partner"
        );
        assert_eq!(
            census.doomed_mass, 1120,
            "the census certifies every boss-with-an-opponent world"
        );
        assert_eq!(
            census.upper,
            BigRational::new(BigInt::from(560), BigInt::from(1680)),
            "the deterministic upper EQUALS the exact best-response value"
        );
        assert_eq!(census.refused_mass, 0, "an ample budget refuses nothing");
        assert_eq!(
            census.doomed_mass + census.survived_mass,
            z,
            "leaves partition the fiber"
        );
        assert!(
            census.classes_empty > 0,
            "joint boss-twice classes are empty"
        );
        let levels: Vec<usize> = census.doomed_leaves.iter().map(|l| l.path.len()).collect();
        assert!(
            levels.contains(&1),
            "boss-with-first-responder certifies at level 1"
        );
        assert!(
            levels.contains(&2),
            "boss-with-seat-3 certifies at level 2 — the conservation squeeze \
             strands the no-boss phantoms, no seat-3 pin needed"
        );
        assert!(
            !levels.contains(&3),
            "nothing here needs the full three-seat restriction"
        );
    }
}

/// Gate 3 — soundness against the exact recursion on enumerable
/// receipt roots: for every legal action, the census upper sits at or
/// above the exact best-response value (`Z − doomed ≥ M*`), and the
/// leaf masses partition the fiber exactly under an ample budget.
#[test]
fn doom_uppers_are_sound_on_the_enumerable_receipt_roots() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    for (hand_id, trick_no) in [(3usize, 5usize), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let z = oracle.mass(&belief);
        for action in legal_actions(&root, &position) {
            let census = doom_census(&oracle, &root, &position, &field, action, &ample());
            assert_eq!(census.fiber, z);
            let mut stats = ResponseStats::default();
            let exact =
                response_success_mass(&oracle, &belief.focal_play(action), &field, &mut stats);
            assert!(
                z - census.doomed_mass >= exact,
                "h{hand_id}-t{trick_no} {action}: certified doom {} would push the upper \
                 below the exact value {exact} of {z}",
                census.doomed_mass
            );
            assert_eq!(census.classes_refused, 0, "an ample budget refuses nothing");
            if !census.whole_fiber {
                assert_eq!(
                    census.doomed_mass + census.survived_mass,
                    z,
                    "leaves partition the fiber"
                );
            }
        }
    }
}

/// Gate 4 — a starved budget refuses honestly: some classes are
/// refused with their mass counted, certified mass never exceeds the
/// ample census's, and the whole census is deterministic (two runs
/// are equal in every field).
#[test]
fn a_starved_budget_refuses_honestly_and_deterministically() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let (root, position) = root_at(&r, 3, 5);
    let starved = DoomSpec {
        node_budget: 40,
        walk_cap: 20,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    };
    let mut refused_somewhere = false;
    for action in legal_actions(&root, &position) {
        let full = doom_census(&oracle, &root, &position, &field, action, &ample());
        if full.whole_fiber {
            continue;
        }
        let a = doom_census(&oracle, &root, &position, &field, action, &starved);
        let b = doom_census(&oracle, &root, &position, &field, action, &starved);
        assert_eq!(a, b, "a census is a pure function of its declared inputs");
        assert!(
            a.doomed_mass <= full.doomed_mass,
            "starvation never certifies more than plenty"
        );
        assert!(a.nodes <= starved.node_budget, "the budget binds");
        refused_somewhere |= a.classes_refused > 0;
    }
    assert!(
        refused_somewhere,
        "a 40-node budget on a 200-world fiber leaves refusals to count"
    );
}

/// Gate 5 — the producer in the §49 registry: deterministic uppers
/// install once, the closure consumes them (global upper drops to the
/// exact value on the loose-boss fixture), a second produce proposes
/// nothing new, and the state survives the §67.4 byte round trip.
#[test]
fn the_producer_installs_uppers_the_closure_consumes() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let (root, position) = loose_boss();
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity.clone());
    let producer = DoomCensusProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: ample(),
    };
    let results = state.run_producer(&producer);
    assert_eq!(results.len(), 3, "one upper per legal action");
    assert!(results.iter().all(|r| r.is_ok()), "every install lands");
    for sf in state.facts() {
        let Fact::Bound(b) = &sf.fact else {
            panic!("the producer proposes bound facts only");
        };
        assert_eq!(b.side, BoundSide::Upper);
        assert_eq!(b.proof, ProofTag::Deterministic, "nothing sampled exists");
        assert!(!b.executable, "uppers are never executable");
        assert!(
            b.authority.starts_with("doom-census-v1:field:level0:"),
            "the field identity (content-digest label) travels with the bound"
        );
        assert_eq!(
            b.value,
            BigRational::new(BigInt::from(560), BigInt::from(1680)),
            "the certified upper is the exact value"
        );
    }
    let closure = state.closure();
    assert_eq!(
        closure.u_star,
        BigRational::new(BigInt::from(560), BigInt::from(1680)),
        "the closure's global upper is the doom census's"
    );
    assert!(!closure.delta_decisive, "no sampled fact exists to lean on");
    let again = state.run_producer(&producer);
    assert!(again.is_empty(), "an identical fact is proposed once");
    let bytes = state.serialize();
    let parsed = ProofState::parse(&bytes, &root, &position).expect("a lawful state");
    assert_eq!(parsed.serialize(), bytes, "the §67.4 byte round trip");
}

/// Gate 6 — the enumeration meets the census where the census is
/// exact: on the loose-boss fixture the per-world count is the same
/// 1120 of 1680 the class walk certifies (world-aware information
/// buys nothing there — make is decided by where the boss sits), and
/// the built-in coverage assert has already checked the enumeration
/// visits the fiber exactly once.
#[test]
fn the_enumeration_meets_the_census_on_the_loose_boss() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let (root, position) = loose_boss();
    for action in legal_actions(&root, &position) {
        let census = doom_census(&oracle, &root, &position, &field, action, &ample());
        let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
        let e = doom_enumeration(
            &oracle,
            &root,
            &position,
            &field,
            action,
            &ample(),
            &mut progress,
        );
        assert_eq!(e.fiber, 1680);
        assert_eq!(e.doomed, 1120, "the per-world truth is the census's 1120");
        assert_eq!(e.upper, census.upper, "the two instruments agree exactly");
        let by_sig_total: u128 = e.by_first_responder.iter().map(|(_, _, t)| t).sum();
        let by_sig_doomed: u128 = e.by_first_responder.iter().map(|(_, d, _)| d).sum();
        assert_eq!(
            by_sig_total, e.fiber,
            "the signature profile covers the fiber"
        );
        assert_eq!(
            by_sig_doomed, e.doomed,
            "the signature profile covers the doom"
        );
    }
}

/// Gate 7 — on enumerable receipt roots the enumeration DOMINATES the
/// class census (every class-certified world is enumeration-doomed)
/// and stays sound against the exact recursion
/// (`Z − doomed ≥ response_success_mass` — world-aware doom can never
/// push the upper below the information-consistent optimum).
#[test]
fn the_enumeration_dominates_the_census_and_stays_sound() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    for (hand_id, trick_no) in [(3usize, 5usize), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let z = oracle.mass(&belief);
        for action in legal_actions(&root, &position) {
            let census = doom_census(&oracle, &root, &position, &field, action, &ample());
            let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
            let e = doom_enumeration(
                &oracle,
                &root,
                &position,
                &field,
                action,
                &ample(),
                &mut progress,
            );
            assert_eq!(e.fiber, z);
            assert!(
                e.doomed >= census.doomed_mass,
                "h{hand_id}-t{trick_no} {action}: the census certified {} but the \
                 per-world truth is only {}",
                census.doomed_mass,
                e.doomed
            );
            let mut stats = ResponseStats::default();
            let exact =
                response_success_mass(&oracle, &belief.focal_play(action), &field, &mut stats);
            assert!(
                z - e.doomed >= exact,
                "the enumeration upper never undercuts the exact value"
            );
        }
    }
}

/// Gate 8 — the priority census is a declared partial harvest: under
/// `descend_top`, certified doom never exceeds the full census's, the
/// skip ledger is populated, and the harvest is deterministic.
#[test]
fn the_priority_census_is_a_sound_partial_harvest() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let (root, position) = root_at(&r, 3, 5);
    let topk = DoomSpec {
        descend_top: Some(2),
        ..ample()
    };
    for action in legal_actions(&root, &position) {
        let full = doom_census(&oracle, &root, &position, &field, action, &ample());
        let a = doom_census(&oracle, &root, &position, &field, action, &topk);
        let b = doom_census(&oracle, &root, &position, &field, action, &topk);
        assert_eq!(a, b, "a priority census is a pure function of its inputs");
        assert!(
            a.doomed_mass <= full.doomed_mass,
            "a partial harvest never exceeds the full census"
        );
        if !a.whole_fiber && full.classes_walked > 3 {
            assert!(a.classes_skipped > 0, "the skip ledger is populated");
        }
    }
}
