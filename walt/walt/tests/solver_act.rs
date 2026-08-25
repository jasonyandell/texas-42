//! Gates for `solver::act` — the §16.4 controller as an ACTING player
//! (CE thread; the playable-controller delivery). The action policy is a
//! pure function of the controller result; these tests drive it end to
//! end on frozen receipt roots: forced plays, the pre-routed exact
//! endpoint (winner and honest-tie routes), and the Unresolved→level-1
//! fallback at a deliberately tiny world cap.
//!
//! Everything here is regression evidence at exploratory tier; nothing
//! is promoted, and no strength claim is made or implied.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::rules::receipt::Receipt;
use walt::rules::replay::{state_before_trick, voids_before_trick};
use walt::rules::Domino;
use walt::solver::act::{act, delta_run_default, ActConfig, ActRoute};
use walt::solver::adaptive::{DrivenState, RootPosition};
use walt::solver::best_of;
use walt::solver::controller::SetResult;

/// A cheap test configuration: tiny declared schedules, the shadow
/// preroute cap, and a small fallback.
fn cheap(world_cap: u64, exact_cap: u128) -> ActConfig {
    ActConfig {
        n_outer_frozen: 2,
        n0_frozen: 1,
        world_cap,
        exact_cap,
        fallback_n_outer: 4,
        fallback_n0: 2,
    }
}

/// The driven public state at the start of `trick_no` of a receipt hand,
/// with the trick leader as the seat to move.
fn state_at(r: &Receipt, hand_no: usize, trick_no: usize) -> (DrivenState<'static>, u64) {
    let hand = &r.hands[hand_no];
    assert_eq!(hand.id, hand_no);
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("position");
    let (hands, leader) = state_before_trick(hand, trick_no).expect("state");
    let d = (position.prior_played.len() + 1) as u64;
    (
        DrivenState {
            decl: hand.decl,
            bid: hand.bid_points,
            declaring_team: hand.declaring_team,
            viewer_hand: hands[leader.index()],
            leader,
            trick_plays: &[],
            banked: position.banked,
            prior_played: position.prior_played,
            voids: voids_before_trick(hand, trick_no),
        },
        d,
    )
}

/// Trick 7 leaves one tile per seat: the play is forced, the controller
/// never runs, and the route says so.
#[test]
fn a_forced_play_routes_forced_without_running_the_controller() {
    let r = receipt();
    let (state, d) = state_at(&r, 0, 7);
    assert_eq!(state.viewer_hand.len(), 1, "trick 7 holds one tile");
    let decision = act(
        &state,
        &cheap(4, 2000),
        "run:test-forced",
        d,
        &delta_run_default(),
    );
    assert_eq!(decision.route, ActRoute::Forced);
    assert!(decision.route.settled());
    assert_eq!(decision.controller_route, "forced");
    assert!(decision.evaluation.is_none());
    assert_eq!(decision.legal, vec![decision.tile]);
    assert_eq!(decision.among, vec![decision.tile]);
    assert!(decision.fallback_opts.is_none());
}

/// A small root (hand 0 trick 6, fiber 90 — the pinned exp5 count) takes
/// the exact pre-route: the result is `ExactFrozenSet` over the whole
/// fiber, and the acted route is the winner when unique, the level-1
/// choice among the tied maxima otherwise — never an index break.
#[test]
fn a_small_root_pre_routes_exact_and_the_route_matches_the_result() {
    let r = receipt();
    let (state, d) = state_at(&r, 0, 6);
    let legal_count = state.viewer_hand.len();
    assert_eq!(legal_count, 2, "a trick-6 lead offers the whole hand");
    let decision = act(
        &state,
        &cheap(4, 2000),
        "run:test-exact",
        d,
        &delta_run_default(),
    );
    assert_eq!(decision.controller_route, "preroute");
    let evaluation = decision.evaluation.as_ref().expect("the controller ran");
    let SetResult::ExactFrozenSet {
        wins,
        fiber,
        winner,
        ..
    } = &evaluation.result
    else {
        panic!("the pre-route produces ExactFrozenSet");
    };
    assert_eq!(*fiber, 90, "the exp5-pinned hand-0 trick-6 fiber");
    assert_eq!(wins.len(), legal_count, "one candidate per legal tile");
    assert!(wins.iter().all(|w| *w <= 90), "wins live inside the fiber");
    assert!(decision.legal.contains(&decision.tile));
    match winner {
        Some(k) => {
            assert_eq!(decision.route, ActRoute::ExactWinner);
            assert!(decision.route.settled());
            assert_eq!(decision.tile, decision.legal[*k]);
            assert_eq!(decision.among, vec![decision.tile]);
        }
        None => {
            assert_eq!(decision.route, ActRoute::ExactTieLevel1);
            assert!(!decision.route.settled(), "a tie fallback is never settled");
            let best = wins.iter().max().expect("wins");
            let tied: Vec<Domino> = wins
                .iter()
                .enumerate()
                .filter(|(_, w)| *w == best)
                .map(|(k, _)| decision.legal[k])
                .collect();
            assert_eq!(decision.among, tied, "the fallback chose among the maxima");
            assert!(decision.among.contains(&decision.tile));
        }
    }
}

/// With the pre-route disabled and a world cap of 4, no edge can settle
/// (`T_edge = 2·d(d+1)·100 ≥ 400` while four pivotal worlds carry
/// evidence at most 31/5), so the result is honest `Unresolved` and the
/// tile is the level-1 choice among the δ-survivors — recorded as the
/// fallback route, never as a settlement.
#[test]
fn the_capped_sampled_route_falls_back_to_level1_among_survivors() {
    let r = receipt();
    let (state, _) = state_at(&r, 0, 6);
    let decision = act(
        &state,
        &cheap(4, 0),
        "run:test-capped",
        1,
        &delta_run_default(),
    );
    assert_eq!(decision.controller_route, "sampled");
    assert_eq!(decision.route, ActRoute::UnresolvedLevel1);
    assert!(!decision.route.settled());
    let evaluation = decision.evaluation.as_ref().expect("the controller ran");
    let SetResult::Unresolved {
        survivors,
        consumed,
        ..
    } = &evaluation.result
    else {
        panic!("a cap of 4 under T_edge >= 400 cannot settle");
    };
    assert_eq!(*consumed, 4, "the cap is consumed exactly");
    assert_eq!(survivors.len(), 2, "no elimination is possible at cap 4");
    let survivor_tiles: Vec<Domino> = survivors.iter().map(|&k| decision.legal[k]).collect();
    assert_eq!(decision.among, survivor_tiles);
    assert!(decision.among.contains(&decision.tile));
    let opts = decision.fallback_opts.as_ref().expect("a ranked fallback");
    assert_eq!(
        opts.len(),
        survivors.len(),
        "the fallback ranks the survivors only"
    );
    let viewer = state.leader.plus(state.trick_plays.len());
    let ranked = best_of(opts, viewer.team() == walt::rules::Team::T1);
    assert_eq!(
        decision.tile.index(),
        usize::from(ranked),
        "the tile is the live level-1 ordering's choice"
    );
}

/// The acted decision is a pure function of its inputs: same state, same
/// config, same run scope, same ordinal, same tile and route.
#[test]
fn acting_is_deterministic_per_information_state() {
    let r = receipt();
    let (state, d) = state_at(&r, 0, 6);
    let cfg = cheap(4, 2000);
    let delta = delta_run_default();
    let one = act(&state, &cfg, "run:test-det", d, &delta);
    let two = act(&state, &cfg, "run:test-det", d, &delta);
    assert_eq!(one.tile, two.tile);
    assert_eq!(one.route, two.route);
    assert_eq!(one.among, two.among);
    assert_eq!(one.controller_route, two.controller_route);
}

/// The run-scoped risk allocation is honest arithmetic: the per-decision
/// budgets `δ_run/(d(d+1))` over ANY injective set of ordinals telescope
/// to at most δ_run — the license for the stateless `d = plies + 1`
/// convention the play surfaces use. Checked exactly over all 28 plies.
#[test]
fn the_stateless_decision_ordinals_stay_within_the_run_budget() {
    let delta_run = delta_run_default();
    let mut total = BigRational::new(BigInt::from(0), BigInt::from(1));
    for d in 1..=28u64 {
        total += &delta_run / BigRational::from_integer(BigInt::from(d) * BigInt::from(d + 1));
    }
    assert!(
        total < delta_run,
        "the 28-ply allocation stays under delta_run"
    );
}
