//! Player-track stage P5 verification harness: the contingency book
//! (BRIEF_PLAYER_01 §8 table P5; INV-P1).
//!
//! `r_book_roundtrip`: every P2 position's solved plan emits canonical
//! JSON, parses back, re-emits byte-identically, and parses to a
//! structurally equal plan. The inspector trace embeds capped book
//! projections (display data only; reviewed by eye per the brief).

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use rob_core::{
    apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt, close_auction,
    complete_hand, initial_contracted_mechanical, update_support, CloseAuctionOutcome, MatchState,
    Play, RulesConfig, Seat, DOMINO_COUNT,
};
use rob_player::book::{
    plan_book_projection, plan_book_projection_with_openings, plan_from_json, plan_to_json,
};
use rob_player::player::UtilityLens;
use rob_player::trace::{
    fmt_action, trump_set, view_of, TraceDecision, TraceDocument, TraceHand, TracePublic,
    TraceResult, TraceView,
};
use rob_player::{placeholder_auction_script, placeholder_declaration, MonteCarloPlayer, Rob};

use crate::corpus::{deterministic_deal, DetRng};
use crate::p2::boundary_position;
use crate::p3::all_positions;
use crate::receipt::{fmt_commas, Receipt};

/// Book display caps for the inspector projection: three decision levels,
/// twenty-four branches per node, exact elision counts beyond.
const BOOK_DEPTH: usize = 3;
const BOOK_BREADTH: usize = 24;

/// Trace size: deals embedded in the rob inspector trace.
pub const TRACE_DEALS: u64 = 4;

const DEAL_SEED: u64 = 0x0d41_0000;

/// `r_book_roundtrip`: solve every P2 position, emit canonical plan JSON,
/// parse, re-emit byte-identically, compare structurally. Returns
/// (plans, total canonical bytes).
pub fn roundtrip_check() -> (u64, u64) {
    let positions = all_positions();
    // Memory-bounded parallelism (see p4::paired_match).
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(10);
    let cursor = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<u64>>> = Mutex::new((0..positions.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = cursor.fetch_add(1, Ordering::Relaxed);
                if i >= positions.len() {
                    break;
                }
                let (boundary, index) = positions[i];
                let state = boundary_position(index, boundary);
                let plan =
                    rob_player::solve(&state, UtilityLens::Points).expect("Points lens solves");
                let json = plan_to_json(&plan);
                let parsed = plan_from_json(&json);
                assert_eq!(parsed, plan, "parsed plan is structurally equal");
                let again = plan_to_json(&parsed);
                assert_eq!(json, again, "re-emission is byte-identical");
                // The capped projection must also emit deterministically.
                let book = plan_book_projection(&plan, BOOK_DEPTH, BOOK_BREADTH);
                assert_eq!(
                    book,
                    plan_book_projection(&parsed, BOOK_DEPTH, BOOK_BREADTH)
                );
                results.lock().expect("no poisoned workers")[i] = Some(json.len() as u64);
            });
        }
    });
    let sizes: Vec<u64> = results
        .into_inner()
        .expect("no poisoned workers")
        .into_iter()
        .map(|v| v.expect("every slot filled"))
        .collect();
    (sizes.len() as u64, sizes.iter().sum())
}

/// Build the rob inspector trace: `TRACE_DEALS` deterministic deals (the
/// P4 deal stream), rob playing team 0 by rolling re-solve, the baseline
/// playing team 1; rob decisions carry the capped contingency-book
/// projection. Returns (document, rob decisions embedded).
pub fn rob_trace_document() -> (TraceDocument, u64) {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let rob = Rob::new();
    let baseline = MonteCarloPlayer {
        worlds_per_decision: 12,
        lens: UtilityLens::Points,
        seed: 7,
    };
    let mut hands = Vec::new();
    let mut plans_embedded = 0u64;
    for deal_index in 0..TRACE_DEALS {
        let shaker = Seat::ALL[(deal_index % 4) as usize];
        let mut rng = DetRng::new(DEAL_SEED + deal_index);
        let deal = deterministic_deal(&mut rng);
        let (match_state, _) = MatchState::start(config, shaker);
        let (mut attempt, m_auction, _, _) =
            begin_deal_attempt(&match_state, deal, deal_index).expect("NEED_DEAL");
        for action in placeholder_auction_script() {
            let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
            attempt = next;
        }
        let auction_actions = attempt.auction.actions().to_vec();
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

        let fmt_domino = rob_player::match_driver::fmt_domino;
        let mut hand = TraceHand {
            index: deal_index,
            shaker: shaker.index() as u8,
            bidder: bidder.index() as u8,
            declaration: rob_player::match_driver::fmt_declaration(declaration),
            trump: trump_set(declaration).iter().map(fmt_domino).collect(),
            auction: auction_actions
                .iter()
                .map(|&(seat, action)| (seat.index() as u8, fmt_action(action)))
                .collect(),
            deal: core::array::from_fn(|s| {
                deal.hand(Seat::ALL[s]).iter().map(fmt_domino).collect()
            }),
            decisions: Vec::with_capacity(28),
            result: None,
        };

        for play_index in 0..DOMINO_COUNT as u64 {
            let actor = objective.state().current_actor().expect("play phase");
            let viewer_state = viewers[actor.index()].clone();
            let (domino, legal, totals, worlds, plan_book) = if actor.team().index() == 0 {
                let (plan, openings) = rob_player::solve_with_openings(&viewer_state, rob.lens)
                    .expect("the Points lens solves at every window");
                plans_embedded += 1;
                let legal: Vec<String> = rob_player::viewer_legal(&viewer_state)
                    .iter()
                    .copied()
                    .map(fmt_domino)
                    .collect();
                let book =
                    plan_book_projection_with_openings(&plan, &openings, BOOK_DEPTH, BOOK_BREADTH);
                (plan.root.action, legal, Vec::new(), 0usize, Some(book))
            } else {
                let nonce = deal_index * 64 + play_index;
                let report = baseline.decide(&viewer_state, nonce);
                (
                    report.chosen,
                    report.legal.iter().copied().map(fmt_domino).collect(),
                    report.totals.clone(),
                    report.worlds,
                    None,
                )
            };
            let reference = &viewers[0];
            let public = TracePublic {
                leader: reference.leader().index() as u8,
                trick: reference
                    .current_trick()
                    .iter()
                    .map(|p| (p.actor.index() as u8, fmt_domino(p.domino)))
                    .collect(),
                points: reference.hand_points(),
                played: core::array::from_fn(|s| {
                    reference
                        .played_by_seat(Seat::ALL[s])
                        .iter()
                        .map(fmt_domino)
                        .collect()
                }),
                voids: core::array::from_fn(|s| {
                    reference
                        .public_voids(Seat::ALL[s])
                        .iter()
                        .map(|q| match q {
                            rob_core::LedSuit::Natural(p) => p.value().to_string(),
                            rob_core::LedSuit::Called => "C".to_string(),
                        })
                        .collect()
                }),
            };
            let views: [TraceView; 4] = core::array::from_fn(|s| view_of(&viewers[s]));
            let truth: [Vec<String>; 4] = core::array::from_fn(|s| {
                objective
                    .state()
                    .remaining_hand(Seat::ALL[s])
                    .iter()
                    .map(fmt_domino)
                    .collect()
            });
            let (next, _, trick_result) = apply_play(&objective, domino).expect("legal play");
            let after = next.state();
            hand.decisions.push(TraceDecision {
                play: play_index,
                trick: (play_index / 4) as u32 + 1,
                pos: (play_index % 4) as u8,
                actor: actor.index() as u8,
                public,
                views,
                truth,
                legal,
                totals,
                worlds,
                avgs: Vec::new(),
                chosen: fmt_domino(domino),
                forced: false,
                plan: plan_book,
                after_leader: after.leader().index() as u8,
                after_points: after.hand_points(),
                trick_complete: trick_result.map(|r| (r.winner.index() as u8, r.points)),
            });
            objective = next;
            for viewer in &mut viewers {
                *viewer = update_support(viewer, Play { actor, domino }).expect("support update");
            }
        }
        let (result, post) = complete_hand(&objective, &m_play).expect("HAND_COMPLETE settles");
        let declaring = contract.declaring_team();
        hand.result = Some(TraceResult {
            declaring_team: declaring.index() as u8,
            declaring_points: result.final_points[declaring.index()],
            made: result.award.made,
            stake: result.award.marks,
            award_team: result.award.team.index() as u8,
            marks_after: post.marks(),
        });
        hands.push(hand);
    }
    let document = TraceDocument {
        config: (config.max_mark_bid(), config.match_target()),
        player: (0, "Points".to_string(), 0),
        match_seed: DEAL_SEED,
        hands,
        final_marks: [0, 0],
    };
    (document, plans_embedded)
}

/// Build the canonical P5 receipt (BRIEF_PLAYER_01 §8–§9). Panics on any
/// check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("player-p5");
    let (plans, bytes) = roundtrip_check();
    r.line(
        "r_book_roundtrip",
        &format!(
            "{plans} plans byte-equal; {} canonical bytes",
            fmt_commas(bytes as u128)
        ),
    );
    let (document, embedded) = rob_trace_document();
    let decisions: usize = document.hands.iter().map(|h| h.decisions.len()).sum();
    assert_eq!(document.hands.len() as u64, TRACE_DEALS);
    assert_eq!(decisions as u64, TRACE_DEALS * 28);
    assert_eq!(embedded, TRACE_DEALS * 14, "rob decides his team's plays");
    r.line(
        "r_book_trace",
        &format!(
            "{} hands; {decisions} decisions; {embedded} plans embedded",
            document.hands.len()
        ),
    );
    r.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_book_roundtrip_sample() {
        for (boundary, index) in [(4usize, 0u64), (6, 5), (0, 3)] {
            let state = boundary_position(index, boundary);
            let plan = rob_player::solve(&state, UtilityLens::Points).expect("solves");
            let json = plan_to_json(&plan);
            let parsed = plan_from_json(&json);
            assert_eq!(parsed, plan);
            assert_eq!(plan_to_json(&parsed), json);
        }
    }
}
