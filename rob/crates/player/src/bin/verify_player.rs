//! Player self-play receipt binary: one deterministic full match
//! (byte-diffed by CI against `rob/receipts/verify_player.txt`).
//!
//! The transcript is normative for regression purposes (determinism of the
//! whole stack); the trailing statistics block is explicitly non-normative.

use rob_core::{RulesConfig, Seat};
use rob_player::{self_play_match, MonteCarloPlayer, UtilityLens};

fn main() {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let player = MonteCarloPlayer {
        worlds_per_decision: 12,
        lens: UtilityLens::Points,
        seed: 7,
    };
    let transcript = self_play_match(config, &player, 42);
    println!("rob player self-play verification: PASS");
    println!(
        "player: fixed-field MC best response; worlds/decision 12; lens Points; player seed 7; match seed 42; first shaker S{}",
        Seat::ALL[0].index()
    );
    for line in &transcript.lines {
        println!("{line}");
    }
    println!("non-normative statistics (printout only, no verified claim):");
    for (declaration, (hands, points, made)) in &transcript.stats.per_declaration {
        println!("  {declaration}: hands {hands}; declaring points total {points}; made {made}");
    }
}
