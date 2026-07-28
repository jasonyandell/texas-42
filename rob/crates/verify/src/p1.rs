//! Player-track stage P1 verification harness: `GreedySigma`, the fixed
//! deterministic field policy (BRIEF_PLAYER_01 §8 table P1; INV-P4).

use rob_core::{algebra_for, GAME_DECLARATIONS};
use rob_player::greedy_sigma;

use crate::corpus::{deterministic_auction_script, deterministic_deal, DetRng, PlayedHand};
use crate::receipt::{fmt_commas, Receipt};
use crate::s3::s3_corpus_hand;

/// One σ-self-play hand from the S3 corpus deal `index` (same config,
/// shaker, deal, and auction script as [`s3_corpus_hand`]; every one of the
/// 28 plays chosen by `greedy_sigma` instead of the corpus tape). σ's choice
/// is located inside the engine's own legal set — a panic here would mean σ
/// chose an illegal tile, so every completed hand is a legality receipt.
pub fn sigma_hand(index: u64) -> PlayedHand {
    assert!(index < 108);
    let config = rob_core::RulesConfig::new(7, 7).expect("valid config");
    let declaration = GAME_DECLARATIONS[(index / 12) as usize];
    let shaker = rob_core::Seat::ALL[(index % 4) as usize];
    let mut rng = DetRng::new(0x0c0_3000 + index);
    let deal = deterministic_deal(&mut rng);
    let script = deterministic_auction_script(&mut rng, config, shaker);
    let algebra = algebra_for(declaration);
    crate::corpus::play_hand(
        config,
        shaker,
        deal,
        &script,
        declaration,
        |state, legal| {
            let actor = state.state().current_actor().expect("play phase");
            let hand = state.state().remaining_hand(actor);
            let choice = greedy_sigma(&algebra, hand, state.state().current_trick());
            legal
                .iter()
                .position(|&d| d == choice)
                .expect("sigma's choice lies in the engine's legal set")
        },
    )
    .expect("sigma plays every corpus deal to completion")
}

/// Corpus-shape guard: the σ corpus reuses the S3 deals byte-for-byte.
fn assert_same_deal(index: u64, hand: &PlayedHand) {
    let corpus = s3_corpus_hand(index);
    assert_eq!(
        corpus.deal, hand.deal,
        "sigma self-play reuses the S3 corpus deal"
    );
    assert_eq!(corpus.declaration, hand.declaration);
    assert_eq!(corpus.bidder, hand.bidder);
}

/// A deterministic byte trace of one played hand (actor:tile per play).
pub fn hand_trace(hand: &PlayedHand) -> String {
    hand.steps
        .iter()
        .map(|s| format!("S{}:{}", s.actor.index(), s.domino.index()))
        .collect::<Vec<_>>()
        .join(" ")
}

/// `r_sig_selfplay` (R-FOLLOW/R-SETTLE; INV-P4): σ-self-play from each of
/// the 108 S3 corpus deals — every play legal (by construction through the
/// certified lifecycle), exactly 7 resolved tricks and 28 plays per hand,
/// and exactly 42 points conserved per hand. Returns (hands, plays).
pub fn selfplay_check() -> (u64, u64) {
    let mut hands = 0u64;
    let mut plays = 0u64;
    for index in 0..108 {
        let hand = sigma_hand(index);
        assert_same_deal(index, &hand);
        assert_eq!(hand.steps.len(), 28, "28 plays per hand");
        let tricks = hand
            .steps
            .iter()
            .filter(|s| s.trick_result.is_some())
            .count();
        assert_eq!(tricks, 7, "seven resolved tricks per hand");
        assert_eq!(
            hand.result.final_points[0] + hand.result.final_points[1],
            42,
            "42-point conservation (R-SCORE-04)"
        );
        hands += 1;
        plays += hand.steps.len() as u64;
    }
    assert_eq!(hands, 108);
    assert_eq!(plays, 3_024);
    (hands, plays)
}

/// `r_sig_deterministic` (INV-P4): the complete σ-self-play trace set,
/// generated twice, is byte-identical. Returns the trace count.
pub fn deterministic_check() -> u64 {
    for index in 0..108 {
        let first = hand_trace(&sigma_hand(index));
        let second = hand_trace(&sigma_hand(index));
        assert_eq!(first, second, "sigma is deterministic (INV-P4)");
    }
    108
}

/// Build the canonical P1 receipt (BRIEF_PLAYER_01 §8–§9). Panics on any
/// check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("player-p1");
    let (hands, plays) = selfplay_check();
    r.line(
        "r_sig_selfplay",
        &format!(
            "{hands} hands; {} plays; 42x{hands} conserved",
            fmt_commas(plays as u128)
        ),
    );
    let traces = deterministic_check();
    r.line(
        "r_sig_deterministic",
        &format!("{traces} traces byte-equal"),
    );
    r.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn r_sig_selfplay() {
        assert_eq!(selfplay_check(), (108, 3_024));
    }

    #[test]
    fn r_sig_deterministic() {
        assert_eq!(deterministic_check(), 108);
    }

    proptest! {
        /// `r_sig_total` (INV-P4): σ returns a legal tile at every decision
        /// of arbitrary deterministically generated hands — the certified
        /// lifecycle rejects illegal plays, so completion is totality.
        #[test]
        fn r_sig_total(seed in 0u64..1_000_000) {
            let config = rob_core::RulesConfig::new(7, 7).expect("valid config");
            let declaration =
                GAME_DECLARATIONS[(seed % GAME_DECLARATIONS.len() as u64) as usize];
            let shaker = rob_core::Seat::ALL[(seed % 4) as usize];
            let mut rng = DetRng::new(seed ^ 0x5157_0000);
            let deal = deterministic_deal(&mut rng);
            let script = deterministic_auction_script(&mut rng, config, shaker);
            let algebra = algebra_for(declaration);
            let hand = crate::corpus::play_hand(
                config,
                shaker,
                deal,
                &script,
                declaration,
                |state, legal| {
                    let actor = state.state().current_actor().expect("play phase");
                    let own = state.state().remaining_hand(actor);
                    let choice = greedy_sigma(&algebra, own, state.state().current_trick());
                    legal
                        .iter()
                        .position(|&d| d == choice)
                        .expect("sigma's choice lies in the engine's legal set")
                },
            )
            .expect("sigma plays arbitrary deals to completion");
            prop_assert_eq!(
                hand.result.final_points[0] + hand.result.final_points[1],
                42
            );
        }
    }
}
