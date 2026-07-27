//! Stage S2 verification harness: objective hand machine receipts
//! (BRIEF §8, table S2).

use num_bigint::BigUint;

use rob_core::{
    algebra_for, enumerate_terminal_histories, hidden_assignment_count, ordered_deal_count,
    AuctionAction, BasePublicEvent, BidValue, CloseAuctionOutcome, Declaration, DominoId,
    DominoSet, MatchPhase, MatchState, ObjectiveError, RulesConfig, Seat, GAME_DECLARATIONS,
};

use crate::corpus::{
    deterministic_auction_script, deterministic_deal, play_hand, sweep_hand, DetRng, PlayedHand,
};
use crate::receipt::{fmt_commas, Receipt};

/// `r_obj_deals` (Math §6): the ordered-deal domain size `28!/(7!)^4`,
/// computed exactly as a `BigUint` from the deal definition.
pub fn deals_check() -> BigUint {
    let count = ordered_deal_count();
    assert_eq!(count.to_string(), "472518347558400");
    count
}

/// `r_obj_hidden` (Math §7): conditional hidden assignments for one viewer,
/// `21!/(7!)^3`.
pub fn hidden_check() -> BigUint {
    let count = hidden_assignment_count();
    assert_eq!(count.to_string(), "399072960");
    count
}

/// `r_obj_auction_census` (R-AUC-12): exhaustive auction tree per mark cap
/// 1..7 — terminal-history counts, reached mark maxima, and identity of the
/// cap-5/6/7 trees.
pub fn auction_census_check() -> ([usize; 7], [u32; 7]) {
    let mut counts = [0usize; 7];
    let mut maxima = [0u32; 7];
    let mut trees: Vec<Vec<Vec<AuctionAction>>> = Vec::new();
    for cap in 1..=7u32 {
        let config = RulesConfig::new(cap, 7).expect("valid config");
        let mut histories = enumerate_terminal_histories(config);
        histories.sort_by_key(|h| history_sort_key(h));
        let max_mark = histories
            .iter()
            .flatten()
            .filter_map(|a| match a {
                AuctionAction::Bid(BidValue::Mark(m)) => Some(m.value()),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        counts[(cap - 1) as usize] = histories.len();
        maxima[(cap - 1) as usize] = max_mark;
        trees.push(histories);
    }
    assert_eq!(counts, [2380, 3060, 3196, 3213, 3214, 3214, 3214]);
    assert_eq!(maxima, [1, 2, 3, 4, 5, 5, 5]);
    // Reachable ceiling min(m_max, 5): caps 5, 6, 7 induce identical trees.
    assert_eq!(trees[4], trees[5], "caps 5 and 6 must coincide");
    assert_eq!(trees[5], trees[6], "caps 6 and 7 must coincide");
    (counts, maxima)
}

fn history_sort_key(history: &[AuctionAction]) -> Vec<(u8, u32)> {
    history
        .iter()
        .map(|a| match a {
            AuctionAction::Pass => (0u8, 0u32),
            AuctionAction::Bid(BidValue::Point(n)) => (1, n.value() as u32),
            AuctionAction::Bid(BidValue::Mark(m)) => (2, m.value()),
        })
        .collect()
}

/// `r_obj_lifecycle` (Exec §10.1; INV-3/8): certified constructors enforce
/// their pre/postconditions, all-pass advances the shaker without marks,
/// events are explicit return data, and phases are unconstructible out of
/// order.
pub fn lifecycle_check() {
    let config = RulesConfig::new(2, 7).expect("valid config");
    let shaker = Seat::ALL[0];
    let mut rng = DetRng::new(7);
    let deal = deterministic_deal(&mut rng);

    // Match start: NEED_DEAL, marks (0,0), MATCH_STARTED returned as data.
    let (m0, start_event) = MatchState::start(config, shaker);
    assert_eq!(m0.phase(), MatchPhase::NeedDeal);
    assert_eq!(m0.marks(), [0, 0]);
    assert!(matches!(start_event, BasePublicEvent::MatchStarted { .. }));

    // begin_deal_attempt: AUCTION phase, marks/shaker/target unchanged,
    // DEAL_STARTED plus four private observations returned as data.
    let (attempt, m1, deal_event, observations) =
        rob_core::begin_deal_attempt(&m0, deal, 0).expect("NEED_DEAL precondition holds");
    assert_eq!(m1.phase(), MatchPhase::Auction);
    assert_eq!(
        (m1.marks(), m1.shaker(), m1.target()),
        (m0.marks(), m0.shaker(), m0.target())
    );
    assert!(matches!(deal_event, BasePublicEvent::DealStarted { .. }));
    for (i, obs) in observations.iter().enumerate() {
        assert_eq!(obs.hand, *deal.hand(Seat::ALL[i]));
    }
    // Phase misuse is rejected: a second begin on the AUCTION state fails.
    assert_eq!(
        rob_core::begin_deal_attempt(&m1, deal, 1).unwrap_err(),
        ObjectiveError::PhaseMismatch
    );

    // Auction actions validate legality: P(31) then a lower P(30) is
    // rejected; the actor sequence is forced.
    let p30 = AuctionAction::Bid(BidValue::Point(rob_core::PointAmount::new(30).expect("30")));
    let p31 = AuctionAction::Bid(BidValue::Point(rob_core::PointAmount::new(31).expect("31")));
    let (attempt2, bid_event) =
        rob_core::apply_auction_action(&attempt, p31, config).expect("legal opening bid");
    assert!(matches!(bid_event, BasePublicEvent::Bid { actor, .. } if actor == shaker.next()));
    assert_eq!(
        rob_core::apply_auction_action(&attempt2, p30, config).unwrap_err(),
        ObjectiveError::InvalidAuctionAction
    );

    // All-pass: shaker advances clockwise, no marks, NEED_DEAL again.
    let mut all_pass = attempt.clone();
    for _ in 0..4 {
        let (next, _) = rob_core::apply_auction_action(&all_pass, AuctionAction::Pass, config)
            .expect("pass is legal at every incomplete node");
        all_pass = next;
    }
    match rob_core::close_auction(all_pass, &m1, config).expect("complete auction closes") {
        CloseAuctionOutcome::AllPass(next) => {
            assert_eq!(next.shaker(), shaker.next());
            assert_eq!(next.marks(), [0, 0]);
            assert_eq!(next.phase(), MatchPhase::NeedDeal);
        }
        CloseAuctionOutcome::Pending(_) => panic!("four passes are ALL_PASS"),
    }

    // Contracted play: leader is the bidder, hands are the deal, points are
    // zero, phases move to PLAY; premature settlement is rejected.
    let mut auction = attempt2;
    for _ in 0..3 {
        let (next, _) = rob_core::apply_auction_action(&auction, AuctionAction::Pass, config)
            .expect("pass legal");
        auction = next;
    }
    let pending = match rob_core::close_auction(auction, &m1, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => panic!("a bid was made"),
    };
    // Out-of-order construction is rejected: NEED_DEAL match state.
    assert!(matches!(
        rob_core::begin_contracted_play(pending.clone(), Declaration::NoTrump, &m0, config),
        Err(ObjectiveError::PhaseMismatch)
    ));
    let (play_state, m2, decl_event) =
        rob_core::begin_contracted_play(pending, Declaration::NoTrump, &m1, config)
            .expect("AUCTION precondition holds");
    assert_eq!(m2.phase(), MatchPhase::Play);
    assert!(matches!(decl_event, BasePublicEvent::Declaration { .. }));
    let s = play_state.state();
    assert_eq!(s.leader(), s.contract().bidder());
    assert_eq!(s.hand_points(), [0, 0]);
    assert!(s.current_trick().is_empty());
    for &seat in &Seat::ALL {
        assert_eq!(s.remaining_hand(seat), deal.hand(seat));
    }
    assert_eq!(
        rob_core::complete_hand(&play_state, &m2).unwrap_err(),
        ObjectiveError::PhaseMismatch
    );
}

/// Run one deterministic corpus hand for a given index.
pub fn corpus_hand(index: u64) -> PlayedHand {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = Seat::ALL[(index % 4) as usize];
    let mut rng = DetRng::new(0x5eed_0000_0000_0000 ^ index);
    let deal = deterministic_deal(&mut rng);
    let script = deterministic_auction_script(&mut rng, config, shaker);
    let declaration = GAME_DECLARATIONS[(index % 9) as usize];
    play_hand(config, shaker, deal, &script, declaration, |_, legal| {
        rng.below(legal.len())
    })
    .expect("deterministic corpus hand plays to completion")
}

/// Assert the legal-play laws on one played hand (R-FOLLOW-01/02, R-LEAD-01,
/// R-PLAY-02): legal set is follow-if-possible on effective suits else
/// anything; the bidder leads trick one; the trick winner leads next.
pub fn check_legal_play_laws(hand: &PlayedHand) {
    let algebra = algebra_for(hand.declaration);
    assert_eq!(hand.steps[0].actor, hand.bidder, "bidder leads trick one");
    let mut expected_leader = hand.bidder;
    for step in &hand.steps {
        // Independent re-derivation of the exact legal set.
        let expected: Vec<DominoId> = if step.trick_before.is_empty() {
            assert_eq!(step.actor, expected_leader, "trick winner leads next");
            step.hand_before.iter().collect()
        } else {
            let q = algebra.led_suit(step.trick_before[0].domino);
            let followers =
                DominoSet::from_ids(step.hand_before.iter().filter(|&d| algebra.follows(d, q)));
            if followers.is_empty() {
                step.hand_before.iter().collect()
            } else {
                followers.iter().collect()
            }
        };
        assert_eq!(step.legal, expected, "legal plays are follow-if-possible");
        assert!(step.legal.contains(&step.domino));
        if let Some(result) = step.trick_result {
            expected_leader = result.winner;
        }
    }
}

/// `r_obj_legal_play`: the legal-play laws on the deterministic corpus.
/// Returns the number of hands checked.
pub fn legal_play_check() -> u64 {
    let hands = 200;
    for index in 0..hands {
        check_legal_play_laws(&corpus_hand(index));
    }
    hands
}

/// Assert conservation on one played hand (R-SETTLE-02A; R-SCORE-04):
/// seven tricks, exactly 42 points, and `P_D = 42` iff the declaring side
/// took all seven tricks.
pub fn check_conservation(hand: &PlayedHand) {
    let results: Vec<_> = hand.steps.iter().filter_map(|s| s.trick_result).collect();
    assert_eq!(results.len(), 7, "seven tricks per hand");
    let total: u32 = results.iter().map(|r| r.points as u32).sum();
    assert_eq!(total, 42, "hand points sum to exactly 42");
    let declaring = hand.bidder.team();
    let final_points = hand.result.final_points;
    assert_eq!(final_points[0] + final_points[1], 42);
    let swept = results.iter().all(|r| r.winner.team() == declaring);
    assert_eq!(
        final_points[declaring.index()] == 42,
        swept,
        "P_D = 42 iff the declaring side won all seven tricks"
    );
}

/// `r_obj_conservation`: conservation and the sweep equivalence on the
/// corpus plus the deterministic sweep witness. Returns hands checked.
pub fn conservation_check() -> u64 {
    let hands = 200;
    for index in 0..hands {
        check_conservation(&corpus_hand(index));
    }
    // Positive witness: the sweep hand makes P_D = 42 by taking all seven.
    let config = RulesConfig::new(7, 7).expect("valid config");
    let sweep = sweep_hand(config).expect("sweep hand plays to completion");
    check_conservation(&sweep);
    let declaring = sweep.bidder.team();
    assert_eq!(sweep.result.final_points[declaring.index()], 42);
    assert!(sweep.result.award.made);
    hands + 1
}

/// Build the canonical S2 receipt (BRIEF §9). Panics on any check failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("S2");
    r.line("r_obj_deals", &fmt_commas(472_518_347_558_400u128));
    deals_check();
    r.line("r_obj_hidden", &fmt_commas(399_072_960u128));
    hidden_check();
    let (counts, maxima) = auction_census_check();
    r.line(
        "r_obj_auction_census",
        &format!(
            "({}, {}, {}, {}, {}, {}, {}); maxima ({}, {}, {}, {}, {}, {}, {}); caps 5..7 identical",
            counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6],
            maxima[0], maxima[1], maxima[2], maxima[3], maxima[4], maxima[5], maxima[6]
        ),
    );
    lifecycle_check();
    r.line(
        "r_obj_lifecycle",
        "certified constructors enforce Exec 10.1; all-pass advances shaker without marks; events are return data",
    );
    let legal_hands = legal_play_check();
    r.line(
        "r_obj_legal_play",
        &format!(
            "{legal_hands} hands; follow-if-possible; bidder leads trick 1; winner leads next"
        ),
    );
    let conservation_hands = conservation_check();
    r.line(
        "r_obj_conservation",
        &format!("{conservation_hands} hands; 42 points each; P_D=42 iff seven-trick sweep"),
    );
    r.finish()
}
