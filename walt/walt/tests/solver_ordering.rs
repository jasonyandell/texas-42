//! Gates for the `solve_viewer` visit order and its break
//! instrumentation (reorder-not-cull; `walt/CENSUS-RULINGS.md` E-A15:
//! the ORDER of evaluation is lawful to change, the legal SET is not).
//! `solve_viewer` is value-only — max/min over children, no action
//! returned — so a visit-order change exposes no tie-break; these gates
//! pin exactly that invariance.
//!
//! The value pins are KNOWN CHECK VALUES recorded from the
//! ascending-tile-order baseline (the commit that added the break
//! counters, before any reordering landed). The visit order is
//! value-invariant, so these pins must never move; if one moves, the
//! change escaped E-A15's fence.

mod common;

use std::sync::Arc;
use std::time::Duration;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{Decl, Domino, Seat, Team};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::exact_frozen_action_values;
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::targeted::legal_root_actions;
use walt::solver::{
    mask_bits, mix, sample_belief, Deadline, Field, Key, MoveOrdering, Shared, Solver, SplitMix64,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn field0_spec() -> FieldSpec {
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

fn field1_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn pinned(position: &RootPosition, tile: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
        solver_source: "walt-level1-continuation-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![2, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned: tile },
    })
}

/// The `solver_viewer_fiber` deal construction at its declared seed.
const SEED: u64 = 0x9E37_79B9;
const N: usize = 8;
const N0: usize = 2;
const BID: u8 = 30;

fn deal(hand_no: u64) -> [u32; 4] {
    let mut rng = SplitMix64(SEED ^ mix(hand_no));
    let mut tiles: Vec<u8> = (0..28).collect();
    for i in (1..tiles.len()).rev() {
        let j = rng.below((i + 1) as u64) as usize;
        tiles.swap(i, j);
    }
    let mask_slice = |sl: &[u8]| sl.iter().fold(0u32, |a, &x| a | (1u32 << x));
    [
        mask_slice(&tiles[0..7]),
        mask_slice(&tiles[7..14]),
        mask_slice(&tiles[14..21]),
        mask_slice(&tiles[21..28]),
    ]
}

fn root_key() -> Key {
    Key {
        played: 0,
        leader: 1,
        plays: Vec::new(),
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    }
}

/// Per-tile values, host (children, legal), shared (children, legal).
type DirectSolve = (Vec<(String, BigRational)>, (u64, u64), (u64, u64));

/// One serial solve of every root child on a deal fixture under the
/// given visit order; returns the per-tile values and the host solver's
/// break counters. NOTE: memo_len/alive_sets are traversal-order-
/// sensitive readings (intern ids are assigned in encounter order) and
/// are deliberately never asserted anywhere in this file.
fn direct_solve(hand_no: u64, ordering: MoveOrdering) -> DirectSolve {
    let hands = deal(hand_no);
    let dcl = Decl::NoTrump;
    let seat = Seat::from_index(1).expect("seat 1");
    let hand = hands[1];
    let key = root_key();
    let mut rng = SplitMix64(SEED ^ mix(0x22 ^ hand_no));
    let worlds = sample_belief(seat.index(), hand, 0, [7; 4], [0; 4], N, &mut rng);
    let deadline = Deadline::after(Duration::from_secs(86_400));
    let sh = Arc::new(Shared::new(dcl, BID, vec![N0], 0, 7, deadline));
    let solver = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        seat.team() == Team::T1,
        worlds,
        Vec::new(),
        Field::Level(0),
    )
    .with_ordering(ordering);
    let mut values = Vec::new();
    for tile_idx in mask_bits(hand) {
        let tile = Domino::from_index(usize::from(tile_idx)).expect("tile < 28");
        let child = solver.child_after_play(&key, tile, 0);
        let v = solver.solve(&child).expect("no deadline in tests");
        values.push((tile.to_string(), v));
    }
    solver.flush_nodes();
    (
        values,
        solver.viewer_break_counters(),
        sh.viewer_break_totals(),
    )
}

/// The three cheap receipt roots of the targeted gates, σ0 and σ1, pinned
/// to their ascending-order baseline values — the primary gate that a
/// visit-order change changed nothing.
#[test]
fn bench_root_exact_values_match_their_frozen_pins() {
    type Pins = &'static [(&'static str, i64, i64)];
    let cases: &[(usize, usize, u128, Pins, Pins)] = &[
        (
            4,
            6,
            90,
            &[("0-0", 1, 3), ("1-1", 13, 15)],
            &[("0-0", 1, 3), ("1-1", 13, 15)],
        ),
        (
            8,
            5,
            92,
            &[("0-0", 71, 92), ("5-0", 16, 23), ("5-3", 91, 92)],
            &[("0-0", 35, 46), ("5-0", 16, 23), ("5-3", 35, 46)],
        ),
        (
            10,
            6,
            19,
            &[("2-2", 1, 1), ("3-3", 1, 1)],
            &[("2-2", 1, 1), ("3-3", 1, 1)],
        ),
    ];
    let r = receipt();
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    for &(hand_id, trick_no, fiber, pins0, pins1) in cases {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber);
        let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
        let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
        let candidates: Vec<(Domino, &FrozenPolicy)> =
            actions.iter().copied().zip(policies.iter()).collect();
        for (field, pins, tag) in [(&field0, pins0, "sigma0"), (&field1, pins1, "sigma1")] {
            let baseline = exact_frozen_action_values(
                &root,
                &position,
                &candidates,
                field,
                &format!("ordering-pins-{tag}-h{hand_id}-t{trick_no}"),
            );
            assert_eq!(baseline.actions.len(), pins.len());
            for ((action, value), &(pin_tile, n, d)) in baseline
                .actions
                .iter()
                .zip(&baseline.values)
                .zip(pins.iter())
            {
                assert_eq!(action.to_string(), pin_tile);
                assert_eq!(
                    *value,
                    q(n, d),
                    "KNOWN CHECK VALUE moved at h{hand_id}-t{trick_no} {tag} {pin_tile}"
                );
            }
        }
    }
}

/// The visit order is a canonical permutation of the legal set — same
/// multiset (a set: no duplicates), nothing culled, nothing invented —
/// and deterministic across invocations, at a lead root and at a
/// mid-trick follow with count on the table.
#[test]
fn viewer_visit_order_is_a_canonical_permutation_of_the_legal_set() {
    use walt::rules::rules::legal_plays;
    use walt::rules::DominoSet;
    use walt::solver::set_of;

    let hands = deal(1);
    let dcl = Decl::PipTrump(walt::rules::Pip::new(6).expect("pip 6"));
    let seat = Seat::from_index(1).expect("seat 1");
    let hand = hands[1];
    let mut rng = SplitMix64(SEED ^ mix(0x33));
    let worlds = sample_belief(seat.index(), hand, 0, [7; 4], [0; 4], N, &mut rng);
    let deadline = Deadline::after(Duration::from_secs(86_400));
    let sh = Arc::new(Shared::new(dcl, BID, vec![N0], 0, 7, deadline));
    let solver = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        seat.team() == Team::T1,
        worlds,
        Vec::new(),
        Field::Level(0),
    );
    // A lead root (led None) and a follow after seat 1's lowest tile led
    // (led Some, one play on the table).
    let lead_key = root_key();
    let led_tile = mask_bits(hand)[0];
    let follow_key = Key {
        played: 1u32 << led_tile,
        leader: 1,
        plays: vec![led_tile],
        banked_t1: 0,
        banked_t0: 0,
        alive: 0,
    };
    let follower_hand = hands[2];
    let cases: &[(&Key, Option<walt::rules::Context>, u32)] = &[
        (&lead_key, None, hand),
        (
            &follow_key,
            Some(dcl.led_context(Domino::from_index(usize::from(led_tile)).expect("led tile"))),
            follower_hand,
        ),
    ];
    for &(key, led, raw_hand) in cases {
        let legal = legal_plays(dcl, set_of(raw_hand & !key.played), led);
        let order = solver.viewer_visit_order(key, led, legal);
        assert_eq!(order.len(), legal.len(), "nothing culled, nothing added");
        let as_set: DominoSet = order.iter().copied().collect();
        assert_eq!(as_set, legal, "the same legal set, permuted");
        let again = solver.viewer_visit_order(key, led, legal);
        assert_eq!(order, again, "deterministic across invocations");
    }
    // The TileIndex arm is exactly the historical ascending order.
    let ascending_solver = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        seat.team() == Team::T1,
        Vec::new(),
        Vec::new(),
        Field::Level(0),
    )
    .with_ordering(MoveOrdering::TileIndex);
    for &(key, led, raw_hand) in cases {
        let legal = legal_plays(dcl, set_of(raw_hand & !key.played), led);
        let order = ascending_solver.viewer_visit_order(key, led, legal);
        let ascending: Vec<Domino> = legal.iter().collect();
        assert_eq!(order, ascending, "TileIndex is the ascending baseline");
    }
}

/// The direct serial solve: values pinned to the ascending-order
/// baseline, and the break counters advance, never exceed the legal
/// total, and strictly undercut it (the Boolean break fires somewhere in
/// this tree), deterministically across two invocations.
#[test]
fn direct_solve_values_and_break_counters_are_sane() {
    let pins: &[(&str, i64, i64)] = &[
        ("1-0", 3, 8),
        ("2-0", 5, 8),
        ("3-1", 3, 8),
        ("3-2", 3, 8),
        ("5-4", 1, 2),
        ("6-0", 1, 2),
        ("6-1", 5, 8),
    ];
    let (values, host, shared) = direct_solve(1, MoveOrdering::CaptureFirst);
    assert_eq!(values.len(), pins.len());
    for ((tile, value), &(pin_tile, n, d)) in values.iter().zip(pins.iter()) {
        assert_eq!(tile, pin_tile);
        assert_eq!(*value, q(n, d), "KNOWN CHECK VALUE moved at {pin_tile}");
    }
    let (children, legal) = host;
    assert!(children > 0, "the counter advances");
    assert!(children <= legal, "children solved never exceed legal");
    assert!(
        children < legal,
        "the Boolean break fires on this root, so the ratio is strictly below 1"
    );
    let (all_children, all_legal) = shared;
    assert!(all_children >= children, "the fold includes the host");
    assert!(all_legal >= legal, "the fold includes the host");
    // Deterministic across invocations: this path is serial (no rayon),
    // so the counters are exact, not just the values.
    let (values2, host2, shared2) = direct_solve(1, MoveOrdering::CaptureFirst);
    assert_eq!(values, values2);
    assert_eq!(host, host2);
    assert_eq!(shared, shared2);
}

/// The equivalence gate — reorder-not-cull as a checked property, not a
/// claim: the same fixtures under both visit orders produce identical
/// exact values on every root child. Counters may differ (that is the
/// point of the knob); values may not.
#[test]
fn both_orderings_agree_on_every_exact_value() {
    for hand_no in [1u64, 2] {
        let (capture, _, _) = direct_solve(hand_no, MoveOrdering::CaptureFirst);
        let (ascending, _, _) = direct_solve(hand_no, MoveOrdering::TileIndex);
        assert_eq!(
            capture, ascending,
            "E-A15: visit order changed a value on deal({hand_no})"
        );
    }
}
