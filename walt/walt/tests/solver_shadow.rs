//! Gates for §22 step 7's library pieces: the live-root bridge
//! (`solver::adaptive::driven_root`) and the frozen level-1 continuation
//! policies (`solver::policy::ActionRule::PinnedThenLevel1`, the §12.4
//! sampled-discovery case), plus the `Level0Field` adapter. Parent:
//! `walt/math/calculated_evidence_v0.1.md` §12, §18, §22 step 7; rulings
//! CE-A3/A5/A7; obligations O13/O22/O23/O24.
//!
//! Roots come from the frozen `verify_player` receipt (small horizons so
//! the sampled materializations stay cheap). Everything here is regression
//! evidence at exploratory tier; nothing is promoted.

mod common;

use common::{receipt, true_world};
use walt::kernel::{Kernel, ReceiptDecision};
use walt::rules::receipt::Receipt;
use walt::rules::replay::{state_before_trick, voids_before_trick};
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Domino, DominoSet, Team};
use walt::solver::adaptive::{
    driven_root, evaluate_pair, replay_viewer_success, CanonicalRoot, DrivenState, PairSpec,
    PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::controller::{
    evaluate_set_with_switch, exact_frozen_set, CandidateSet, RiskPlan, SetResult, SetSpec,
};
use walt::solver::evidence::ScopedDelta;
use walt::solver::policy::{
    continuation_frame, t1_frame_bid, ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InfoKey,
    InnerSchedule, Level0Field, TieRule, NO_DEADLINE_SECS,
};
use walt::solver::{best_of, level1_evaluate, mask_of, SplitMix64};

use num_bigint::BigInt;
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn root_at(r: &Receipt, hand_no: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_no];
    assert_eq!(hand.id, hand_no);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// A `PinnedThenLevel1` freeze tuple for one root, with a small declared
/// schedule so test materializations stay cheap.
fn continuation_tuple(position: &RootPosition, pinned: Domino) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-level1-continuation-v1-test".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![4, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    }
}

// ---------------------------------------------------------------------------
// The live-root bridge (§22 step 7 item 2; O23 — one canonical root).
// ---------------------------------------------------------------------------

/// At every trick start of every receipt hand, the bridge fed with the
/// replayed public state constructs EXACTLY the objects the receipt
/// constructors build: the same kernel, the same position, the same count.
#[test]
fn driven_root_matches_the_receipt_constructors_at_every_trick_start() {
    let r = receipt();
    for hand in &r.hands {
        for trick_no in 2..=7usize {
            let expected_kernel = Kernel::from_receipt_trick(hand, trick_no).expect("kernel");
            let expected_position =
                RootPosition::from_receipt_trick(hand, trick_no).expect("position");
            let (hands, leader) = state_before_trick(hand, trick_no).expect("state");
            let state = DrivenState {
                decl: hand.decl,
                bid: hand.bid_points,
                declaring_team: hand.declaring_team,
                viewer_hand: hands[leader.index()],
                leader,
                trick_plays: &[],
                banked: expected_position.banked,
                prior_played: expected_position.prior_played,
                voids: voids_before_trick(hand, trick_no),
            };
            let (root, position) = driven_root(&state).expect("a lawful driven root");
            assert_eq!(
                root.kernel(),
                &expected_kernel,
                "hand {} trick {trick_no}",
                hand.id
            );
            assert_eq!(position, expected_position);
            assert_eq!(
                root.count(),
                CanonicalRoot::new(expected_kernel).count(),
                "one exact count from one kernel"
            );
        }
    }
}

/// Mid-trick: for every ply of a voidful hand's tricks, the bridge agrees
/// with `ReceiptDecision::at` (the receipt's mid-trick constructor) and
/// the true world stays a member of the fiber.
#[test]
fn driven_root_matches_receipt_decisions_mid_trick() {
    let r = receipt();
    for hand_no in [4usize, 8, 11] {
        let hand = &r.hands[hand_no];
        for trick_no in 4..=6usize {
            let trick = &hand.tricks[trick_no - 1];
            let position = RootPosition::from_receipt_trick(hand, trick_no).expect("position");
            let (hands_before, leader) = state_before_trick(hand, trick_no).expect("state");
            for ply in 1..4usize {
                let viewer = leader.plus(ply);
                let decision = ReceiptDecision::at(hand, trick_no, viewer).expect("decision");
                assert_eq!(decision.ply, ply);
                // Voids incl. failures inside the partial trick, exactly
                // as decision.rs derives them.
                let mut voids = voids_before_trick(hand, trick_no);
                let led = hand.decl.led_context(trick.plays[0].1);
                for (seat, tile) in trick.plays.iter().take(ply).skip(1) {
                    if !hand.decl.follows(*tile, led) {
                        voids[seat.index()].insert(led);
                    }
                }
                let prefix: Vec<Domino> = trick.plays.iter().take(ply).map(|(_, d)| *d).collect();
                let state = DrivenState {
                    decl: hand.decl,
                    bid: hand.bid_points,
                    declaring_team: hand.declaring_team,
                    viewer_hand: hands_before[viewer.index()],
                    leader,
                    trick_plays: &prefix,
                    banked: position.banked,
                    prior_played: position.prior_played,
                    voids,
                };
                let (root, mid_position) = driven_root(&state).expect("a lawful driven root");
                assert_eq!(
                    root.kernel(),
                    &decision.kernel,
                    "hand {hand_no} trick {trick_no} ply {ply}"
                );
                assert_eq!(mid_position.trick_plays, prefix);
                let world = decision.true_world(hand).expect("true world");
                assert!(root.kernel().contains(&world), "the true deal is lawful");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PinnedThenLevel1 — identity, the pin, and hidden-world invariance
// (§12.3/§12.4/O22).
// ---------------------------------------------------------------------------

/// Distinct pins are distinct PolicyIds; the root information state
/// materializes the pin itself; and across all 90 hidden worlds of the
/// fiber the root state occupies ONE memo entry — the evaluation world
/// never reaches the key.
#[test]
fn pinned_policies_differ_by_pin_and_materialize_the_pin_at_the_root() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let viewer = root.kernel().viewer();
    let hand = root.kernel().viewer_hand();
    let legal = legal_plays(position.decl, hand, None);
    assert!(legal.len() >= 2, "the root offers a real choice");
    let tiles: Vec<Domino> = legal.iter().collect();
    let a = FrozenPolicy::new(continuation_tuple(&position, tiles[0]));
    let b = FrozenPolicy::new(continuation_tuple(&position, tiles[1]));
    assert_ne!(a.policy_id(), b.policy_id(), "the pin is identity");
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &[],
    };
    for world in root.worlds() {
        assert_eq!(world.hand(viewer), hand);
        assert_eq!(a.choose(position.decl, hand, legal, &record), tiles[0]);
        assert_eq!(b.choose(position.decl, hand, legal, &record), tiles[1]);
    }
    assert_eq!(a.cache_len(), 1, "ninety worlds, one root state, one entry");
    assert_eq!(b.cache_len(), 1);
}

/// §12.4 — the evaluation stream cannot influence materialized actions:
/// two instances of the SAME freeze tuple, replayed under two different
/// evidence epochs (disjoint world streams), agree on every information
/// state they both visited.
#[test]
fn materialized_actions_are_invariant_to_the_evaluation_stream() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let pinned = legal_plays(position.decl, root.kernel().viewer_hand(), None)
        .iter()
        .next()
        .expect("a legal tile");
    let one = FrozenPolicy::new(continuation_tuple(&position, pinned));
    let two = FrozenPolicy::new(continuation_tuple(&position, pinned));
    assert_eq!(one.policy_id(), two.policy_id());
    let field = Level0Field::new(2);
    let viewer = root.kernel().viewer();
    let root_id = walt::solver::adaptive::root_identity(&root, &position);
    // Instance one replays evidence-epoch 1's worlds; instance two
    // replays evidence-epoch 2's (a disjoint stream).
    for i in 0..6u64 {
        let w1 = root.world_at(root_id, 1, i);
        let w2 = root.world_at(root_id, 2, i);
        replay_viewer_success(&position, viewer, &w1, &one, &field);
        replay_viewer_success(&position, viewer, &w2, &two, &field);
    }
    let snap_one = one.cache_snapshot();
    let snap_two = two.cache_snapshot();
    let mut shared = 0usize;
    for (key, action) in &snap_one {
        if let Some(other) = snap_two.get(key) {
            assert_eq!(action, other, "one information state, one action");
            shared += 1;
        }
    }
    assert!(shared >= 1, "the root state at least is shared");
}

/// The continuation parity gate: at a post-root information state, the
/// materialized action equals a DIRECT run of the existing level-1
/// machinery on the discovery-derived stream — the §22 step 7 seam,
/// reproduced from public API alone.
#[test]
fn continuation_materialization_equals_direct_level1_on_the_discovery_stream() {
    let r = receipt();
    // Walk the TRUE world of a mid-hand root forward under (policy,
    // field) until the viewer faces a multi-option decision with
    // non-empty history; whether one occurs depends on the deal, so
    // several roots are tried and the first that produces one is used.
    let mut found = None;
    for (hand_no, trick_no) in [(4usize, 5usize), (4, 4), (0, 5), (11, 5), (0, 4)] {
        let (root, position) = root_at(&r, hand_no, trick_no);
        let hand = &r.hands[hand_no];
        let viewer = root.kernel().viewer();
        let decl = position.decl;
        let pinned = legal_plays(decl, root.kernel().viewer_hand(), None)
            .iter()
            .next()
            .expect("a legal tile");
        let policy = FrozenPolicy::new(continuation_tuple(&position, pinned));
        let field = Level0Field::new(2);
        let world = true_world(root.kernel(), hand, trick_no);
        let mut hands = world.hands();
        let mut leader = position.leader;
        let mut plays: Vec<Domino> = Vec::new();
        let mut banked = position.banked;
        let mut history: Vec<Domino> = Vec::new();
        let mut target: Option<(DominoSet, DominoSet, Vec<Domino>)> = None;
        while hands.iter().any(|h| !h.is_empty()) {
            let seat = leader.plus(plays.len());
            let led = plays.first().map(|d| decl.led_context(*d));
            let seat_hand = hands[seat.index()];
            let legal = legal_plays(decl, seat_hand, led);
            if seat == viewer && !history.is_empty() && legal.len() > 1 && target.is_none() {
                target = Some((seat_hand, legal, history.clone()));
            }
            let record = PublicRecord {
                leader,
                trick_plays: &plays,
                banked,
                root: &position,
                history: &history,
            };
            let tile = if seat == viewer {
                policy.choose(decl, seat_hand, legal, &record)
            } else {
                field.choose(decl, seat_hand, legal, &record)
            };
            assert!(hands[seat.index()].remove(tile));
            plays.push(tile);
            history.push(tile);
            if plays.len() == 4 {
                let doms: [Domino; 4] = core::array::from_fn(|i| plays[i]);
                let trick = Trick::new(leader, doms).expect("distinct tiles");
                let winner = trick.winner(decl);
                banked[winner.team().index()] += trick.points();
                leader = winner;
                plays.clear();
            }
        }
        if let Some(t) = target {
            found = Some((policy, position, viewer, t));
            break;
        }
    }
    let (policy, position, viewer, (seat_hand, legal, at_history)) =
        found.expect("some walked root reaches a multi-option viewer state");
    let decl = position.decl;
    // Reproduce the materialization from public API: the frame is a
    // derived view of the record, the stream is the policy's discovery
    // derivation, the machinery is the live player's level1_evaluate.
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &at_history,
    };
    let key = InfoKey::from_public(policy.policy_id(), seat_hand, &record);
    let frame = continuation_frame(decl, &position, &at_history);
    let mut rng = SplitMix64(policy.discovery_rng(&key, 0).next_u64());
    let opts = level1_evaluate(
        decl,
        t1_frame_bid(position.bid, position.declaring_team),
        frame.seat,
        mask_of(seat_hand),
        mask_of(legal),
        &frame.key,
        frame.sizes(),
        frame.voids,
        frame.trick_start_played,
        frame.boundary_hand_size,
        4,
        2,
        NO_DEADLINE_SECS,
        &mut rng,
    )
    .expect("no wall-clock cutoff");
    let expected = Domino::from_index(usize::from(best_of(&opts, frame.seat.team() == Team::T1)))
        .expect("tile < 28");
    let cached = policy
        .cache_snapshot()
        .get(&key)
        .copied()
        .expect("the walk materialized this state");
    assert_eq!(cached, expected, "the seam runs the existing machinery");
    assert_eq!(frame.seat, viewer);
}

// ---------------------------------------------------------------------------
// Level0Field — deterministic, legal, and the library's one authority.
// ---------------------------------------------------------------------------

#[test]
fn level0_field_is_deterministic_legal_and_named_by_its_declared_n0() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let field = Level0Field::new(2);
    assert_eq!(field.id(), "field:level0-n2-v1");
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &[],
    };
    let hand = root.kernel().viewer_hand();
    let legal = legal_plays(position.decl, hand, None);
    let once = field.choose(position.decl, hand, legal, &record);
    let twice = field.choose(position.decl, hand, legal, &record);
    assert_eq!(once, twice, "a modeled mind is pure in its state");
    assert!(legal.contains(once));
}

// ---------------------------------------------------------------------------
// The controller over pinned continuation candidates: V9/O24 parity and
// the honest ladder.
// ---------------------------------------------------------------------------

/// Cold exact enumeration, forced switches at arbitrary indices, and the
/// adaptive path all agree on the fiber-90 root with a full pinned
/// candidate set — the sampled-discovery policies compose with the
/// escalation bookkeeping unchanged.
#[test]
fn pinned_candidates_forced_switch_parity_on_the_small_root() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), None);
    let pool: Vec<FrozenPolicy> = legal
        .iter()
        .map(|t| FrozenPolicy::new(continuation_tuple(&position, t)))
        .collect();
    let candidates = CandidateSet::new(pool.iter().collect());
    let field = Level0Field::new(2);
    let spec = SetSpec {
        root: &root,
        position: &position,
        candidates: &candidates,
        field: &field,
        plan: RiskPlan::strict(ScopedDelta::new("decision:shadow-v9", q(1, 50))),
        world_cap: 512,
        batch: 8,
        escalation: None,
    };
    let cold = exact_frozen_set(&spec);
    let SetResult::ExactFrozenSet { wins, fiber, .. } = &cold.result else {
        panic!("cold enumeration is exact");
    };
    assert_eq!(*fiber, 90);
    assert_eq!(wins.len(), legal.len());
    for switch_at in [0u64, 7, 33] {
        let escalated = evaluate_set_with_switch(&spec, switch_at);
        assert_eq!(
            escalated.result, cold.result,
            "forced switch at {switch_at} equals cold enumeration"
        );
        let report = escalated.escalation.expect("the exact endpoint ran");
        assert_eq!(
            u128::from(report.reused_worlds) + u128::from(report.fresh_worlds),
            root.count()
        );
    }
}

/// The pair evaluator accepts pinned continuation policies through the
/// same seam (a smoke gate that the evidence path and the sampled
/// discovery never touch each other's streams: the run completes with the
/// §1 ladder's honest kinds only).
#[test]
fn evaluate_pair_runs_pinned_continuations_to_an_honest_result() {
    let r = receipt();
    let (root, position) = root_at(&r, 4, 6);
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), None);
    let tiles: Vec<Domino> = legal.iter().collect();
    let a = FrozenPolicy::new(continuation_tuple(&position, tiles[0]));
    let b = FrozenPolicy::new(continuation_tuple(&position, tiles[1]));
    let field = Level0Field::new(2);
    let evaluation = evaluate_pair(&PairSpec {
        root: &root,
        position: &position,
        policy_a: &a,
        policy_b: &b,
        field: &field,
        delta: ScopedDelta::new("decision:shadow-pair", q(1, 100)),
        epoch: 1,
        world_cap: 96,
        batch: 8,
    });
    let tag = evaluation.result.tag();
    assert!(
        tag == "DeltaSettled" || tag == "Unresolved",
        "the pair path stays on the honest ladder, got {tag}"
    );
}

// ---------------------------------------------------------------------------
// Freeze-time validation.
// ---------------------------------------------------------------------------

#[test]
#[should_panic(expected = "declares its inner schedule")]
fn a_pinned_continuation_without_a_declared_schedule_is_rejected() {
    let r = receipt();
    let (_, position) = root_at(&r, 4, 6);
    let pinned = Domino::from_index(0).expect("tile");
    let tuple = FreezeTuple {
        inner_schedule: InnerSchedule::None,
        ..continuation_tuple(&position, pinned)
    };
    FrozenPolicy::new(tuple);
}

#[test]
#[should_panic(expected = "lowest tile index")]
fn a_pinned_continuation_must_name_the_tie_rule_its_algorithm_applies() {
    let r = receipt();
    let (_, position) = root_at(&r, 4, 6);
    let pinned = Domino::from_index(0).expect("tile");
    let tuple = FreezeTuple {
        tie_rule: TieRule::FirstInPreference,
        ..continuation_tuple(&position, pinned)
    };
    FrozenPolicy::new(tuple);
}
