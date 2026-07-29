//! Exploratory probe: opponent-model counterfactuals for a single trick
//! contest. Re-scores the current trick over the exact fiber under two
//! responder policies — σ's minimal beater (the certified field model) and
//! a max-trump "shut-out" line (the human instinct) — quantifying how much
//! of a trick-odds claim is combinatorics and how much is the opponent
//! model in the denominator.
//!
//! First subject: P4-stream deal 1, trick 4 (the nickel decision).
//! Frozen findings, asserted exactly: partner wins the trick in
//! 5,970/8,400 worlds under σ but exactly 4,200/8,400 (50.0%) under
//! max-play; the led 4-0 wins in 0 worlds under either; single-seat trump
//! voids are 450/8,400 each and a double void is impossible.

use rob_core::{
    algebra_for, apply_auction_action, begin_contracted_play, begin_deal_attempt, close_auction,
    derive_rule_cells, domino_id, initial_contracted_mechanical, update_support,
    CloseAuctionOutcome, DeclarationAlgebra, Domino, DominoId, DominoSet, MatchState, Pip, Play,
    RulesConfig, Seat,
};
use rob_verify::corpus::{deterministic_deal, DetRng};

const DEAL_SEED: u64 = 0x0d41_0000;

fn tile(name: &str) -> DominoId {
    let (h, l) = name.split_once('-').expect("high-low");
    domino_id(Domino::new(
        Pip::new(h.parse().unwrap()).unwrap(),
        Pip::new(l.parse().unwrap()).unwrap(),
    ))
}

/// The 13 public plays before the decision (extracted from the rob trace,
/// deal index 1; identical to `nickel_probe`).
const PREFIX: [(usize, &str); 13] = [
    (2, "2-2"),
    (3, "6-2"),
    (0, "2-0"),
    (1, "5-2"),
    (2, "3-3"),
    (3, "6-4"),
    (0, "3-1"),
    (1, "3-2"),
    (3, "1-1"),
    (0, "2-1"),
    (1, "6-1"),
    (2, "1-0"),
    (3, "4-0"),
];

/// Integer-encoded contest key of `d` under the led context of `led`
/// (strictly monotone in the algebra's `TrickKey` order; 0 for sloughs) —
/// derived from the implemented algebra, never from pip arithmetic.
fn contest_key(algebra: &DeclarationAlgebra, led: DominoId, d: DominoId) -> u32 {
    use rob_core::{Rank, TrickKey};
    match algebra.trick_key(d, algebra.led_suit(led)) {
        TrickKey::Slough => 0,
        TrickKey::Ranked { tier, rank } => {
            let rank_enc = match rank {
                Rank::PipSum(n) => (n as u32) << 2,
                Rank::DoublePip(p) => ((p.value() as u32) << 2) | 1,
                Rank::Top => (255u32 << 2) | 2,
            };
            ((tier as u32) << 16) | (rank_enc + 1)
        }
    }
}

#[test]
fn sigma_counterfactual_trick_contest() {
    // Reconstruct S0's decision state (as in nickel_probe).
    let config = RulesConfig::new(7, 7).expect("valid config");
    let deal_index = 1u64;
    let shaker = Seat::ALL[(deal_index % 4) as usize];
    let mut rng = DetRng::new(DEAL_SEED + deal_index);
    let deal = deterministic_deal(&mut rng);
    let (match_state, _) = MatchState::start(config, shaker);
    let (mut attempt, m_auction, _, _) =
        begin_deal_attempt(&match_state, deal, deal_index).expect("NEED_DEAL");
    for action in rob_player::placeholder_auction_script() {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m_auction, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!(),
    };
    let bidder = pending.win().bidder();
    let declaration = rob_player::placeholder_declaration(deal.hand(bidder));
    let (objective, _, _) = begin_contracted_play(pending, declaration, &m_auction, config)
        .expect("AUCTION precondition");
    let contract = *objective.state().contract();
    let viewer = Seat::ALL[0];
    let mut state =
        initial_contracted_mechanical(viewer, *deal.hand(viewer), contract).expect("seven tiles");
    for (actor, name) in PREFIX {
        state = update_support(
            &state,
            Play {
                actor: Seat::ALL[actor],
                domino: tile(name),
            },
        )
        .expect("legal observation");
    }
    let algebra = algebra_for(declaration);
    let cells = derive_rule_cells(&state);
    let worlds = cells.fiber_worlds();
    assert_eq!(worlds.len(), 8_400, "the decision's exact fiber");

    // The current trick: S3 led 4-0; the viewer's four options are all
    // sloughs, so the contest is entirely between the hidden responders
    // S1 (hidden index 0) then S2 (hidden index 1), in that order.
    let led = tile("4-0");
    let led_key = contest_key(&algebra, led, led);
    for own in [tile("3-0"), tile("5-0"), tile("6-3"), tile("6-6")] {
        assert_eq!(
            contest_key(&algebra, led, own),
            0,
            "every viewer option is a slough: the discard cannot move the contest"
        );
    }
    let hidden = state.hidden_seats();
    assert_eq!(hidden[0], Seat::ALL[1], "responder order S1 then S2");
    assert_eq!(hidden[1], Seat::ALL[2]);

    let beaters = |hand: &DominoSet| -> Vec<u32> {
        let mut keys: Vec<u32> = hand
            .iter()
            .map(|d| contest_key(&algebra, led, d))
            .filter(|&k| k > led_key)
            .collect();
        keys.sort_unstable();
        keys
    };

    let mut sigma_partner = 0u64; // S2 (viewer's partner) takes the trick under σ
    let mut sigma_overtrump = 0u64; // ... specifically by beating S1's minimal beater
    let mut maxplay_partner = 0u64; // S2 takes it if S1 shuts out with its max
    let mut led_holds = 0u64; // the led 4-0 wins (nobody contests)
    let mut s1_void = 0u64;
    let mut s2_void = 0u64;
    for world in &worlds {
        let b1 = beaters(&world.hidden_hands[0]);
        let b2 = beaters(&world.hidden_hands[1]);
        if b1.is_empty() {
            s1_void += 1;
        }
        if b2.is_empty() {
            s2_void += 1;
        }
        // σ: S1 plays its MINIMAL beater; S2 overtrumps it if it can.
        match (b1.first(), b2.last()) {
            (Some(&p1), Some(&m2)) if m2 > p1 => {
                sigma_partner += 1;
                sigma_overtrump += 1;
            }
            (Some(_), _) => {}
            (None, Some(_)) => sigma_partner += 1,
            (None, None) => led_holds += 1,
        }
        // Counterfactual: S1 shuts out with its MAXIMAL beater.
        match (b1.last(), b2.last()) {
            (Some(&p1), Some(&m2)) if m2 > p1 => maxplay_partner += 1,
            (Some(_), _) => {}
            (None, Some(_)) => maxplay_partner += 1,
            (None, None) => {}
        }
    }

    // Frozen findings for this subject (deterministic facts of the frozen
    // position; a mismatch means the machinery changed under us).
    assert_eq!(sigma_partner, 5_970, "partner takes the trick under sigma");
    assert_eq!(
        sigma_overtrump, 5_520,
        "mostly by overtrumping the minimal beater"
    );
    assert_eq!(
        maxplay_partner, 4_200,
        "exactly 50.0% under the shut-out line"
    );
    assert_eq!(led_holds, 0, "the led 4-0 never wins the trick");
    assert_eq!(s1_void, 450, "S1 trump-void worlds");
    assert_eq!(s2_void, 450, "S2 trump-void worlds");
    println!(
        "sigma: partner {sigma_partner}/8400 (overtrump {sigma_overtrump}); max-play: partner {maxplay_partner}/8400; led holds {led_holds}; voids S1 {s1_void} S2 {s2_void}"
    );
}
