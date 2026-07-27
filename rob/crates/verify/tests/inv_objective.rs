//! Named invariant enforcement tests for the objective machine (BRIEF §5:
//! INV-2 PROJECTED-EQUALITY, INV-3 PROOF-IRRELEVANT-REACHABILITY, INV-8
//! ONE-SOURCE-OF-TRUTH).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rob_core::{
    algebra_for, apply_auction_action, begin_contracted_play, begin_deal_attempt, close_auction,
    AuctionAction, BasePublicEvent, BidValue, CloseAuctionOutcome, Declaration, MatchState, Play,
    PointAmount, RulesConfig, Seat,
};
use rob_verify::corpus::{deterministic_deal, DetRng, PlayedHand};
use rob_verify::s2::corpus_hand;

fn hash_of<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

/// INV-2 PROJECTED-EQUALITY: two certified states identical in semantic
/// fields but differing in witness/audit data are equal with the same hash.
#[test]
fn inv_projected_equality() {
    let hand = corpus_hand(3);
    let with_audit = hand.states[10].clone();
    assert!(
        with_audit.audit().is_some(),
        "lifecycle retains an audit witness"
    );
    let erased = with_audit.clone().with_erased_audit();
    assert!(erased.audit().is_none());
    assert_eq!(with_audit, erased, "audit data is excluded from equality");
    assert_eq!(hash_of(&with_audit), hash_of(&erased), "and from hashing");
}

/// INV-3 PROOF-IRRELEVANT-REACHABILITY: no stored reachability flag — two
/// states certified through *different* legal auction histories with equal
/// projections are the same semantic state (D1; Exec §25).
#[test]
fn inv_no_reachability_field() {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = Seat::ALL[0];
    let mut rng = DetRng::new(42);
    let deal = deterministic_deal(&mut rng);

    let p = |n: u8| AuctionAction::Bid(BidValue::Point(PointAmount::new(n).expect("valid")));
    // Both histories end with seat shaker+3 winning at P(35), via different
    // intermediate bids.
    let history_a = [p(30), AuctionAction::Pass, p(35), AuctionAction::Pass];
    let history_b = [AuctionAction::Pass, p(31), p(35), AuctionAction::Pass];

    let build = |script: [AuctionAction; 4]| {
        let (m0, _) = MatchState::start(config, shaker);
        let (mut attempt, m1, _, _) = begin_deal_attempt(&m0, deal, 0).expect("begin");
        for action in script {
            let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
            attempt = next;
        }
        let pending = match close_auction(attempt, &m1, config).expect("closes") {
            CloseAuctionOutcome::Pending(pending) => pending,
            CloseAuctionOutcome::AllPass(_) => panic!("bids were made"),
        };
        let (state, _, _) =
            begin_contracted_play(pending, Declaration::NoTrump, &m1, config).expect("begin play");
        state
    };

    let state_a = build(history_a);
    let state_b = build(history_b);
    assert_eq!(
        state_a.state().contract().bidder(),
        shaker.offset(3),
        "both histories certify the same bidder"
    );
    assert_eq!(
        state_a, state_b,
        "equal projections are the same semantic state"
    );
    assert_eq!(hash_of(&state_a), hash_of(&state_b));
}

/// INV-8 ONE-SOURCE-OF-TRUTH: every derived public fact (auction winner,
/// trick winners, trick points, running score, settlement) recomputes from
/// the base event stream alone and agrees with the machine outputs
/// (R-INFO-02A).
#[test]
fn inv_event_replay() {
    for index in [0u64, 1, 2, 17, 99] {
        replay_and_compare(&corpus_hand(index));
    }
}

fn replay_and_compare(hand: &PlayedHand) {
    // Parse the base stream.
    let mut bids: Vec<(Seat, AuctionAction)> = Vec::new();
    let mut declaration = None;
    let mut plays: Vec<Play> = Vec::new();
    for event in &hand.events {
        match *event {
            BasePublicEvent::Bid { actor, action, .. } => bids.push((actor, action)),
            BasePublicEvent::Declaration {
                actor,
                declaration: d,
                ..
            } => declaration = Some((actor, d)),
            BasePublicEvent::Play { actor, domino, .. } => plays.push(Play { actor, domino }),
            _ => {}
        }
    }
    let (decl_actor, declaration) = declaration.expect("declaration event present");
    assert_eq!(declaration, hand.declaration);

    // Derived auction result: the last nonpass bidder wins (R-AUC-10).
    let (bidder, winning_bid) = bids
        .iter()
        .rev()
        .find_map(|&(seat, action)| match action {
            AuctionAction::Bid(v) => Some((seat, v)),
            AuctionAction::Pass => None,
        })
        .expect("a winner exists");
    assert_eq!(bidder, decl_actor);
    assert_eq!(bidder, hand.bidder);

    // Derived trick facts: replay the plays from events alone.
    let algebra = algebra_for(declaration);
    let mut leader = bidder;
    let mut points = [0u32; 2];
    let mut winners = Vec::new();
    assert_eq!(plays.len(), 28);
    for trick in plays.chunks(4) {
        assert_eq!(trick[0].actor, leader, "leader derived from prior winner");
        let result = algebra.resolve_trick(trick).expect("legal trick resolves");
        points[result.winner.team().index()] += result.points as u32;
        winners.push(result.winner);
        leader = result.winner;
    }
    assert_eq!(points[0] + points[1], 42);

    // Compare with the machine's outputs.
    let machine_winners: Vec<Seat> = hand
        .steps
        .iter()
        .filter_map(|s| s.trick_result.map(|r| r.winner))
        .collect();
    assert_eq!(winners, machine_winners, "trick winners replay identically");
    assert_eq!(points, hand.result.final_points, "running score replays");

    // Derived settlement (R-SETTLE-01..04).
    let threshold = match winning_bid {
        BidValue::Point(n) => n.value() as u32,
        BidValue::Mark(_) => 42,
    };
    let declaring = bidder.team();
    let made = points[declaring.index()] >= threshold;
    assert_eq!(made, hand.result.award.made, "settlement replays");
    let team = if made {
        declaring
    } else {
        declaring.opponent()
    };
    assert_eq!(team, hand.result.award.team);
}
