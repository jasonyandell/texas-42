//! EXPLORATORY ORDERING BENCH (`walt/probes/ordering/`) — sits below
//! every evidentiary tier and is cited by nothing above it. Wall-clock
//! numbers are machine-local instrument readings, never receipts.
//!
//! Benches the `solve_viewer` visit order (E-A15: the ORDER of evaluation
//! is lawful to change; the legal SET is not). Declared workload:
//!   1. The three cheap receipt roots the targeted gates use — hand 4
//!      trick 6 (fiber 90), hand 8 trick 5 (fiber 92), hand 10 trick 6
//!      (fiber 19) — each through the cold exact frozen-set endpoint
//!      under the declared cheap pair (σ0 = Level0 n0=2, σ1 = Level1
//!      n_outer=2 n0=2), exactly the `solver_targeted` fixtures.
//!   2. `level1_evaluate` at two synthetic declared-seed roots (the
//!      `solver_viewer_fiber` deal construction).
//!   3. The same synthetic roots driven through a bench-owned `Solver`,
//!      where the break counters are readable (the high-level endpoints
//!      build their `Shared`s internally, so their counters are not
//!      reachable from here — the direct items carry the counter signal).
//!   4. `hard` argument only: hand 8 trick 4 (fiber 1200), σ0.
//!
//! Prints integer wall-micros per item and the children-solved / legal
//! counters as exact integers ("a/b"). No floats anywhere.

use std::sync::Arc;
use std::time::{Duration, Instant};

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{Decl, Domino, Seat, Team};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::exact_frozen_action_values;
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::targeted::legal_root_actions;
use walt::solver::{
    level1_evaluate, mask_bits, mix, sample_belief, Deadline, Field, Key, MoveOrdering, Shared,
    Solver, SplitMix64,
};

/// The `solver_viewer_fiber` declared deal seed.
const SEED: u64 = 0x9E37_79B9;
const N_OUTER: usize = 8;
const N0: usize = 2;
const BID: u8 = 30;

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
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

fn exact_item(r: &Receipt, hand_id: usize, trick_no: usize, field: &FieldModel, tag: &str) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|a| pinned(&position, *a)).collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let t = Instant::now();
    let baseline = exact_frozen_action_values(
        &root,
        &position,
        &candidates,
        field,
        &format!("ordering-bench-{tag}"),
    );
    let micros = t.elapsed().as_micros();
    let values: Vec<String> = baseline
        .actions
        .iter()
        .zip(&baseline.values)
        .map(|(a, v)| format!("{a}={v}"))
        .collect();
    println!(
        "item={tag} fiber={} micros={micros} values=[{}]",
        baseline.fiber,
        values.join(", "),
    );
}

/// The `solver_viewer_fiber` deal construction at the declared seed.
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

fn level1_item(hand_no: u64) {
    let hands = deal(hand_no);
    let dcl = Decl::NoTrump;
    let seat = Seat::from_index(1).expect("seat 1");
    let hand = hands[1];
    let key = root_key();
    let mut rng = SplitMix64(SEED ^ mix(0x11 ^ hand_no));
    let t = Instant::now();
    let opts = level1_evaluate(
        dcl, BID, seat, hand, hand, &key, [7; 4], [0; 4], 0, 7, N_OUTER, N0, 86_400, &mut rng,
    )
    .expect("no deadline in the bench");
    let micros = t.elapsed().as_micros();
    println!(
        "item=level1-deal{hand_no} options={} micros={micros}",
        opts.len()
    );
}

/// Both visit-order arms on one fixture (a fresh `Shared` per arm so the
/// folded totals stay per-arm): the in-binary A/B the `MoveOrdering`
/// selector exists for.
fn direct_item(hand_no: u64, ordering: MoveOrdering, tag: &str) {
    let hands = deal(hand_no);
    let dcl = Decl::NoTrump;
    let seat = Seat::from_index(1).expect("seat 1");
    let hand = hands[1];
    let key = root_key();
    let mut rng = SplitMix64(SEED ^ mix(0x22 ^ hand_no));
    let worlds = sample_belief(seat.index(), hand, 0, [7; 4], [0; 4], N_OUTER, &mut rng);
    let deadline = Deadline::after(Duration::from_secs(86_400));
    let sh = Arc::new(Shared::new(dcl, BID, vec![N0], 0, 7, deadline));
    let maximize = seat.team() == Team::T1;
    let solver = Solver::new(
        Arc::clone(&sh),
        seat,
        hand,
        maximize,
        worlds,
        Vec::new(),
        Field::Level(0),
    )
    .with_ordering(ordering);
    let t = Instant::now();
    let mut values: Vec<String> = Vec::new();
    for tile_idx in mask_bits(hand) {
        let tile = Domino::from_index(usize::from(tile_idx)).expect("tile < 28");
        let child = solver.child_after_play(&key, tile, 0);
        let v = solver.solve(&child).expect("no deadline in the bench");
        values.push(format!("{tile}={v}"));
    }
    let micros = t.elapsed().as_micros();
    solver.flush_nodes();
    let (host_children, host_legal) = solver.viewer_break_counters();
    let (all_children, all_legal) = sh.viewer_break_totals();
    println!(
        "item=direct-deal{hand_no}-{tag} micros={micros} \
         host_children/legal={host_children}/{host_legal} \
         shared_children/legal={all_children}/{all_legal} values=[{}]",
        values.join(", "),
    );
}

fn main() {
    let hard = std::env::args().any(|a| a == "hard");
    println!("ordering_bench v1 (EXPLORATORY; integer micros; counters exact)");
    let r = receipt();
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    for (hand_id, trick_no) in [(4usize, 6usize), (8, 5), (10, 6)] {
        exact_item(
            &r,
            hand_id,
            trick_no,
            &field0,
            &format!("exact-sigma0-h{hand_id}-t{trick_no}"),
        );
        exact_item(
            &r,
            hand_id,
            trick_no,
            &field1,
            &format!("exact-sigma1-h{hand_id}-t{trick_no}"),
        );
    }
    for hand_no in [1u64, 2] {
        level1_item(hand_no);
        direct_item(hand_no, MoveOrdering::TileIndex, "tileindex");
        direct_item(hand_no, MoveOrdering::CaptureFirst, "capturefirst");
    }
    if hard {
        exact_item(&r, 8, 4, &field0, "exact-sigma0-h8-t4-hard");
    }
}
