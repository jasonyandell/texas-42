//! Player-track stage P4 verification harness: rob at the table and the
//! mirrored paired match (BRIEF_PLAYER_01 §8 table P4).
//!
//! rob plays every decision of his team's two seats from trick 1 by
//! rolling re-solve; the baseline (the demoted §11.4 Monte Carlo player)
//! plays the other team. Deals are mirrored: each deterministic deal is
//! played twice with the teams swapped, so deal luck cancels in the net
//! margin. The frozen margin is a measurement, never a target (§10.3).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use rob_core::{
    apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt, close_auction,
    complete_hand, derive_rule_cells, initial_contracted_mechanical, update_support,
    CloseAuctionOutcome, DominoSet, MatchState, Play, RemainderWorld, RulesConfig, Seat,
    DOMINO_COUNT,
};
use rob_player::player::UtilityLens;
use rob_player::window::{window_depth_with, WINDOW_BUDGET};
use rob_player::{placeholder_auction_script, placeholder_declaration, MonteCarloPlayer, Rob};

use crate::corpus::{deterministic_deal, DetRng};
use crate::receipt::{fmt_commas, Receipt};

/// Paired-match size (BRIEF_PLAYER_01 §8 P4): 100 deterministic deals,
/// each played under both seatings.
pub const DEALS: u64 = 100;

const DEAL_SEED: u64 = 0x0d41_0000;

/// The baseline opponent, receipt configuration (worlds 12, Points,
/// seed 7 — the slice-01 receipt player, untouched per §10.6).
fn baseline() -> MonteCarloPlayer {
    MonteCarloPlayer {
        worlds_per_decision: 12,
        lens: UtilityLens::Points,
        seed: 7,
    }
}

/// One hand's evidence.
struct HandEvidence {
    /// rob-team points minus baseline-team points.
    margin: i64,
    /// rob decisions taken (rolling re-solve, every one from trick 1).
    rob_decisions: u64,
    /// Window-formula agreements (INV-P6).
    window_checks: u64,
    /// True-world fiber memberships (CELL-05 losslessness in live play).
    fiber_checks: u64,
}

/// Play one deal with rob on `rob_team` (0 or 1), the baseline on the
/// other. `budget` overrides the window budget for the ablation runs;
/// `None` is the normative constant.
fn play_hand(deal_index: u64, rob_team: usize, budget: Option<u128>) -> HandEvidence {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = Seat::ALL[(deal_index % 4) as usize];
    let mut rng = DetRng::new(DEAL_SEED + deal_index);
    let deal = deterministic_deal(&mut rng);
    let rob = Rob::new();
    let base = baseline();

    let (match_state, _) = MatchState::start(config, shaker);
    let (mut attempt, m_auction, _, _) =
        begin_deal_attempt(&match_state, deal, deal_index).expect("NEED_DEAL");
    for action in placeholder_auction_script() {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m_auction, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!("the placeholder always bids"),
    };
    let bidder = pending.win().bidder();
    let declaration = placeholder_declaration(deal.hand(bidder));
    let (mut objective, m_play, _) =
        begin_contracted_play(pending, declaration, &m_auction, config)
            .expect("AUCTION precondition");

    let contract = *objective.state().contract();
    let mut viewers: [rob_core::MechanicalState; 4] = core::array::from_fn(|s| {
        initial_contracted_mechanical(Seat::ALL[s], *deal.hand(Seat::ALL[s]), contract)
            .expect("seven-tile hands")
    });
    let mut played = [DominoSet::empty(); 4];

    let mut evidence = HandEvidence {
        margin: 0,
        rob_decisions: 0,
        window_checks: 0,
        fiber_checks: 0,
    };
    for play_index in 0..DOMINO_COUNT as u64 {
        let actor = objective.state().current_actor().expect("play phase");
        let domino = if actor.team().index() == rob_team {
            let viewer_state = &viewers[actor.index()];
            let plan = match budget {
                None => rob.decide(viewer_state),
                Some(b) => rob_player::solver::gate::solve_with_budget(viewer_state, rob.lens, b)
                    .expect("the Points lens solves at every window"),
            };
            // INV-P6: the solved window is exactly the formula's output.
            let hand_size = viewer_state.own_remaining_hand().len();
            let effective = budget.unwrap_or(WINDOW_BUDGET);
            assert_eq!(
                plan.window,
                window_depth_with(plan.fiber_count, hand_size, effective).min(hand_size),
                "rolling window matches the normative formula"
            );
            evidence.window_checks += 1;
            // CELL-05 live: the true remainder world lies in the derived
            // fiber at every rob decision.
            let cells = derive_rule_cells(viewer_state);
            let hidden = viewer_state.hidden_seats();
            let world = RemainderWorld {
                hidden_hands: core::array::from_fn(|i| {
                    deal.hand(hidden[i]).difference(&played[hidden[i].index()])
                }),
            };
            assert!(
                cells.fiber_contains(&world),
                "the dealt world survives in the fiber (losslessness live)"
            );
            evidence.fiber_checks += 1;
            evidence.rob_decisions += 1;
            plan.root.action
        } else {
            let nonce = deal_index * 64 + play_index;
            base.decide(&viewers[actor.index()], nonce).chosen
        };
        let (next, _, _) = apply_play(&objective, domino).expect("legal play");
        objective = next;
        played[actor.index()].insert(domino);
        for viewer in &mut viewers {
            *viewer = update_support(viewer, Play { actor, domino }).expect("support update");
        }
    }
    let (result, _) = complete_hand(&objective, &m_play).expect("HAND_COMPLETE settles");
    evidence.margin =
        result.final_points[rob_team] as i64 - result.final_points[1 - rob_team] as i64;
    evidence
}

/// The mirrored paired match under one budget: returns (seating-A margin,
/// seating-B margin, net, rob decisions, fiber checks). Seating A puts rob
/// on team 0; every deal is played under both seatings.
pub fn paired_match(budget: Option<u128>) -> (i64, i64, i64, u64, u64) {
    let deals: Vec<u64> = (0..DEALS).collect();
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let cursor = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<(HandEvidence, HandEvidence)>>> =
        Mutex::new((0..deals.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = cursor.fetch_add(1, Ordering::Relaxed);
                if i >= deals.len() {
                    break;
                }
                let a = play_hand(deals[i], 0, budget);
                let b = play_hand(deals[i], 1, budget);
                results.lock().expect("no poisoned workers")[i] = Some((a, b));
            });
        }
    });
    let mut margin_a = 0i64;
    let mut margin_b = 0i64;
    let mut decisions = 0u64;
    let mut fibers = 0u64;
    for slot in results.into_inner().expect("no poisoned workers") {
        let (a, b) = slot.expect("every deal played");
        assert_eq!(a.rob_decisions, a.window_checks);
        assert_eq!(a.rob_decisions, a.fiber_checks);
        margin_a += a.margin;
        margin_b += b.margin;
        decisions += a.rob_decisions + b.rob_decisions;
        fibers += a.fiber_checks + b.fiber_checks;
    }
    (margin_a, margin_b, margin_a + margin_b, decisions, fibers)
}

fn fmt_signed(v: i64) -> String {
    if v < 0 {
        format!("-{}", fmt_commas(v.unsigned_abs() as u128))
    } else {
        fmt_commas(v as u128)
    }
}

/// Build the canonical P4 receipt (BRIEF_PLAYER_01 §8–§9). Panics on any
/// check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("player-p4");
    let (a, b, net, decisions, fibers) = paired_match(None);
    r.line(
        "r_mat_rolling",
        &format!(
            "{} rob decisions; {} window agreements; {} live fiber memberships",
            fmt_commas(decisions as u128),
            fmt_commas(decisions as u128),
            fmt_commas(fibers as u128)
        ),
    );
    r.line(
        "r_mat_paired",
        &format!(
            "200 hands; seating margins {} / {}; net {}",
            fmt_signed(a),
            fmt_signed(b),
            fmt_signed(net)
        ),
    );
    let (_, _, net_half, _, _) = paired_match(Some(WINDOW_BUDGET / 2));
    let (_, _, net_double, _, _) = paired_match(Some(WINDOW_BUDGET * 2));
    r.line(
        "r_mat_window_ablation",
        &format!(
            "net B/2 {} ; net 2B {}",
            fmt_signed(net_half),
            fmt_signed(net_double)
        ),
    );
    r.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-deal smoke of the full paired loop (the receipt binary runs
    /// the whole match; a full run is tens of minutes).
    #[test]
    fn r_mat_smoke() {
        for deal in 0..2 {
            let a = play_hand(deal, 0, None);
            let b = play_hand(deal, 1, None);
            assert_eq!(a.rob_decisions, 14);
            assert_eq!(b.rob_decisions, 14);
            assert_eq!(a.rob_decisions, a.fiber_checks);
        }
    }
}
