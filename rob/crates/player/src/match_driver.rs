//! Deterministic self-play through the certified S2 lifecycle.
//!
//! Each seat runs the same Monte Carlo player against its own
//! viewer-perspective mechanical state (partners share utility but never
//! information — TEAM-01). The transcript is deterministic in the seed and
//! is the byte-diffable self-play receipt.

use std::collections::BTreeMap;

use rob_core::{
    all_ids, apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt,
    close_auction, complete_hand, domino_from_id, initial_contracted_mechanical, update_support,
    CloseAuctionOutcome, DealWorld, Declaration, DominoId, DominoSet, MatchPhase, MatchState, Play,
    RulesConfig, Seat, DOMINO_COUNT,
};

use crate::bidding::{placeholder_auction_script, placeholder_declaration};
use crate::player::MonteCarloPlayer;
use crate::rng::SplitMix64;

/// Non-normative self-play statistics: useful printout, not a verified
/// claim (labeled as such in the receipt).
#[derive(Clone, Debug, Default)]
pub struct SelfPlayStats {
    /// Per-declaration: (hands, total declaring points, contracts made).
    pub per_declaration: BTreeMap<String, (u32, u64, u32)>,
    /// Total contracted hands played.
    pub hands: u32,
}

/// A finished deterministic self-play match.
#[derive(Clone, Debug)]
pub struct MatchTranscript {
    /// The transcript lines (deterministic, byte-diffable).
    pub lines: Vec<String>,
    /// Non-normative summary statistics.
    pub stats: SelfPlayStats,
    /// Final marks by partnership.
    pub final_marks: [u32; 2],
}

fn fmt_domino(id: DominoId) -> String {
    let d = domino_from_id(id);
    format!("{}-{}", d.high().value(), d.low().value())
}

fn fmt_declaration(declaration: Declaration) -> String {
    match declaration {
        Declaration::PipTrump(p) => format!("P{}", p.value()),
        Declaration::DoublesTrump => "DT".to_string(),
        Declaration::NoTrump => "NT".to_string(),
    }
}

/// A deterministic Fisher–Yates deal from the driver's own tape.
fn deterministic_deal(rng: &mut SplitMix64) -> DealWorld {
    let mut ids: Vec<DominoId> = all_ids().collect();
    for i in (1..DOMINO_COUNT).rev() {
        ids.swap(i, rng.below((i + 1) as u64) as usize);
    }
    let hands: [DominoSet; 4] =
        core::array::from_fn(|s| DominoSet::from_ids(ids[s * 7..(s + 1) * 7].iter().copied()));
    DealWorld::new(hands).expect("slices of a permutation form a valid deal")
}

/// Play one full match to the configured target, all four seats driven by
/// `player`. Deterministic in `seed`.
pub fn self_play_match(
    config: RulesConfig,
    player: &MonteCarloPlayer,
    seed: u64,
) -> MatchTranscript {
    let mut rng = SplitMix64::new(seed);
    let mut lines = Vec::new();
    let mut stats = SelfPlayStats::default();
    let (mut match_state, _) = MatchState::start(config, Seat::ALL[0]);
    let mut attempt_index = 0u64;

    while match_state.phase() != MatchPhase::MatchComplete {
        let deal = deterministic_deal(&mut rng);
        let (mut attempt, m_auction, _, _) =
            begin_deal_attempt(&match_state, deal, attempt_index).expect("NEED_DEAL");
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

        lines.push(format!(
            "hand {attempt_index}: shaker S{}, bidder S{}, bid P(30), declaration {}",
            match_state.shaker().index(),
            bidder.index(),
            fmt_declaration(declaration)
        ));

        // Per-seat viewer states: each seat sees only its own hand plus the
        // public prefix.
        let contract = *objective.state().contract();
        let mut viewers: [rob_core::MechanicalState; 4] = core::array::from_fn(|s| {
            initial_contracted_mechanical(Seat::ALL[s], *deal.hand(Seat::ALL[s]), contract)
                .expect("seven-tile hands")
        });

        let mut trick_line: Vec<String> = Vec::new();
        let mut trick_number = 0u32;
        for play_index in 0..DOMINO_COUNT as u64 {
            let actor = objective.state().current_actor().expect("play phase");
            let nonce = attempt_index * 64 + play_index;
            let report = player.decide(&viewers[actor.index()], nonce);
            let domino = report.chosen;
            let (next, _, trick_result) = apply_play(&objective, domino).expect("legal play");
            objective = next;
            trick_line.push(format!("S{}:{}", actor.index(), fmt_domino(domino)));
            for viewer in &mut viewers {
                *viewer = update_support(viewer, Play { actor, domino }).expect("support update");
            }
            if let Some(result) = trick_result {
                trick_number += 1;
                lines.push(format!(
                    "  trick {trick_number}: {} -> S{} +{}",
                    trick_line.join(" "),
                    result.winner.index(),
                    result.points
                ));
                trick_line.clear();
            }
        }

        let (hand_result, next_match) =
            complete_hand(&objective, &m_play).expect("HAND_COMPLETE settles");
        let declaring = contract.declaring_team();
        let declaring_points = hand_result.final_points[declaring.index()];
        lines.push(format!(
            "  result: declaring T{} {} points -> {} ({} mark to T{}); marks T0 {} - {} T1",
            declaring.index(),
            declaring_points,
            if hand_result.award.made {
                "made"
            } else {
                "set"
            },
            hand_result.award.marks,
            hand_result.award.team.index(),
            next_match.marks()[0],
            next_match.marks()[1],
        ));
        let entry = stats
            .per_declaration
            .entry(fmt_declaration(declaration))
            .or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += declaring_points as u64;
        entry.2 += u32::from(hand_result.award.made);
        stats.hands += 1;

        match_state = next_match;
        attempt_index += 1;
    }
    lines.push(format!(
        "match result: T0 {} - {} T1 after {} hands",
        match_state.marks()[0],
        match_state.marks()[1],
        stats.hands
    ));
    MatchTranscript {
        lines,
        stats,
        final_marks: match_state.marks(),
    }
}
