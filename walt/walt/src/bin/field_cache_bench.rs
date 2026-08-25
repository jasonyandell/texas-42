//! EXPLORATORY FIELD-CACHE BENCH (`walt/probes/field_cache/`) — sits
//! below every evidentiary tier and is cited by nothing above it.
//! Wall-clock numbers are machine-local instrument readings, never
//! receipts.
//!
//! Benches the two surgical levers in the acting hot path on the SAME
//! per-world replay workload, three ways:
//!
//!   (a) `bare-full`   — bare `Level0Field`, full-terminal replay (the
//!       pre-lever act baseline, reconstructed bench-side);
//!   (b) `cached-full` — the cached `FieldModel` (act's declared σ0
//!       spec), full-terminal replay (lever 1 alone);
//!   (c) `cached-cut`  — the cached `FieldModel` through the library
//!       `replay_viewer_success`, whose decided cutoff ships (levers
//!       1 + 2 — the shipped configuration).
//!
//! Every arm replays the identical (world × candidate) grid with FRESH
//! policy instances (no cross-arm cache warmth) and the arms' wins
//! vectors are asserted identical — the value-identity the gates in
//! `tests/solver_field_cache.rs` establish, shown live here.
//!
//! Declared workload: the two `solver_controller` receipt roots (hand 4
//! trick 6, fiber 90; hand 11 trick 5, fiber 1120) over their COMPLETE
//! fibers, and the convicted regime — hand 0 trick 1 (fiber 399,072,960)
//! over the first 128 worlds of a declared evidence stream (the
//! interactive `world_cap`), candidates = one act-shaped frozen level-1
//! continuation per legal root action at the interactive declared
//! schedule [8, 2].
//!
//! Prints integer wall-micros per arm; ratios are printed as "a/b",
//! never computed. No floats anywhere.

use std::time::Instant;

use walt::kernel::{Kernel, World};
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Domino, Seat};
use walt::solver::act::{act_field_spec, continuation_tuple};
use walt::solver::adaptive::{
    replay_viewer_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::controller::{epoch_identity, CandidateSet};
use walt::solver::evidence::ScopedDelta;
use walt::solver::field::FieldModel;
use walt::solver::policy::{FrozenPolicy, Level0Field};
use walt::solver::targeted::legal_root_actions;

use num_bigint::BigInt;
use num_rational::BigRational;

/// The interactive act schedule (`ActConfig::interactive`): declared
/// outer/inner counts of every frozen level-1 continuation candidate,
/// and the declared σ0 field count.
const N_OUTER_FROZEN: u64 = 8;
const N0_FROZEN: u64 = 2;
/// The interactive world cap — the sampled-route prefix length.
const SAMPLED_WORLDS: u64 = 128;

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

/// One act-shaped candidate pool: a frozen level-1 continuation pinned to
/// each legal root action, exactly `solver::act`'s candidate identity.
fn pool(position: &RootPosition, actions: &[Domino]) -> Vec<FrozenPolicy> {
    actions
        .iter()
        .map(|t| {
            FrozenPolicy::new(continuation_tuple(
                position.decl,
                position.bid,
                position.declaring_team,
                N_OUTER_FROZEN,
                N0_FROZEN,
                *t,
            ))
        })
        .collect()
}

/// The pre-lever full-terminal replay, reconstructed bench-side (the
/// library's `replay_viewer_success` now carries the decided cutoff).
fn replay_full(
    position: &RootPosition,
    viewer: Seat,
    world: &World,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
) -> bool {
    let mut hands = world.hands();
    let mut leader = position.leader;
    let mut plays = position.trick_plays.clone();
    let mut banked = position.banked;
    let mut history: Vec<Domino> = Vec::new();
    while hands.iter().any(|h| !h.is_empty()) {
        let seat = leader.plus(plays.len());
        let led = plays.first().map(|d| position.decl.led_context(*d));
        let hand = hands[seat.index()];
        let legal = legal_plays(position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = PublicRecord {
            leader,
            trick_plays: &plays,
            banked,
            root: position,
            history: &history,
        };
        let policy = if seat == viewer { focal } else { field };
        let tile = policy.choose(position.decl, hand, legal, &record);
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        assert!(hands[seat.index()].remove(tile), "the chosen tile is held");
        plays.push(tile);
        history.push(tile);
        if plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| plays[i]);
            let trick = Trick::new(leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            banked[winner.team().index()] += trick.points();
            leader = winner;
            plays.clear();
        }
    }
    let made = banked[position.declaring_team.index()] >= position.bid;
    if viewer.team() == position.declaring_team {
        made
    } else {
        !made
    }
}

enum Replay {
    Full,
    Cutoff,
}

/// One arm: fresh candidates, the given field, the given replay variant,
/// over the identical world grid. Returns (micros, wins).
fn arm(
    position: &RootPosition,
    viewer: Seat,
    worlds: &[World],
    actions: &[Domino],
    field: &dyn SlicePolicy,
    replay: &Replay,
) -> (u128, Vec<u64>) {
    let candidates = pool(position, actions);
    let mut wins = vec![0u64; candidates.len()];
    let t = Instant::now();
    for world in worlds {
        for (k, candidate) in candidates.iter().enumerate() {
            let u = match replay {
                Replay::Full => replay_full(position, viewer, world, candidate, field),
                Replay::Cutoff => replay_viewer_success(position, viewer, world, candidate, field),
            };
            if u {
                wins[k] += 1;
            }
        }
    }
    (t.elapsed().as_micros(), wins)
}

fn wins_str(wins: &[u64]) -> String {
    let parts: Vec<String> = wins.iter().map(|w| w.to_string()).collect();
    parts.join(",")
}

fn bench_item(tag: &str, root: &CanonicalRoot, position: &RootPosition, worlds: &[World]) {
    let viewer = root.kernel().viewer();
    let actions: Vec<Domino> = legal_root_actions(root, position).iter().collect();
    let bare = Level0Field::new(usize::try_from(N0_FROZEN).expect("fits"));
    let (a_micros, a_wins) = arm(position, viewer, worlds, &actions, &bare, &Replay::Full);
    let cached_b = FieldModel::new(act_field_spec(N0_FROZEN));
    let (b_micros, b_wins) = arm(position, viewer, worlds, &actions, &cached_b, &Replay::Full);
    let cached_c = FieldModel::new(act_field_spec(N0_FROZEN));
    let (c_micros, c_wins) = arm(
        position,
        viewer,
        worlds,
        &actions,
        &cached_c,
        &Replay::Cutoff,
    );
    assert_eq!(a_wins, b_wins, "{tag}: cached field diverged from bare");
    assert_eq!(a_wins, c_wins, "{tag}: decided cutoff diverged from full");
    println!(
        "item={tag} fiber={} worlds={} candidates={} wins=[{}]",
        root.count(),
        worlds.len(),
        actions.len(),
        wins_str(&a_wins),
    );
    println!("  arm=bare-full   micros={a_micros}");
    println!("  arm=cached-full micros={b_micros}");
    println!("  arm=cached-cut  micros={c_micros}");
    println!("  ratio bare-full/cached-cut = {a_micros}/{c_micros}");
}

fn main() {
    println!("field_cache_bench v1 (EXPLORATORY; integer micros; single-shot)");
    let r = receipt();

    // The two solver_controller roots, complete fibers.
    for (hand_id, trick_no, tag) in [(4usize, 6usize, "exact-h4-t6"), (11, 5, "exact-h11-t5")] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let worlds: Vec<World> = root.worlds().collect();
        bench_item(tag, &root, &position, &worlds);
    }

    // The convicted regime: a trick-1 receipt root, sampled route at the
    // interactive cap. The worlds are the declared evidence stream's
    // first 128, exactly as the controller would draw them (the epoch
    // never folds the field, so every arm sees the same worlds).
    let (root, position) = root_at(&r, 0, 1);
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let candidates_owned = pool(&position, &actions);
    let candidates = CandidateSet::new(candidates_owned.iter().collect());
    let delta = ScopedDelta::new(
        "decision:field-cache-bench-t1",
        BigRational::new(BigInt::from(1), BigInt::from(200)),
    );
    let root_id = root_identity(&root, &position);
    let epoch = epoch_identity(root_id, &candidates, &delta).stream_epoch();
    let worlds: Vec<World> = (0..SAMPLED_WORLDS)
        .map(|i| root.world_at(root_id, epoch, i))
        .collect();
    bench_item("sampled-h0-t1", &root, &position, &worlds);
}
