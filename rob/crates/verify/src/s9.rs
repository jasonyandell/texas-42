//! Stage S9 verification harness: transport-aware census plumbing
//! (BRIEF_SLICE_02 §9, table S9).
//!
//! Exchange tier: `x-` rows draw on x:004 (CONFIRMED 2026-07-27). These
//! finite receipts are conformance evidence for `f_{t,u}(R_t) = R_u`, not
//! a proof of it.

use rob_core::support::transport_reach::{
    reachable_census_class, transport_normal_form, transport_play, transport_set,
};
use rob_core::{
    apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt, close_auction,
    contract_from_auction, derive_rule_cells, legal_plays, pip_trump_transport, AuctionAction,
    BidValue, CloseAuctionOutcome, Contract, DealWorld, Declaration, DominoId, DominoSet,
    MatchState, Play, PointAmount, RulesConfig, Seat, SymbolicTraceCertificate,
    TotalSupportNormalForm, UnscoredMechanicsClass, GAME_DECLARATIONS, PIPS,
};

use crate::receipt::{fmt_commas, Receipt};
use crate::s3::{mechanical_trajectory, s3_corpus_hand};

/// `r_tra_class_quotient` (rec ALG-22/23; x:004): the census quotient is
/// constant on the seven pip trumps and yields exactly 3 classes, agreeing
/// with S1's `unscored_mechanics_class` partition.
pub fn class_quotient_check() -> u64 {
    let mut classes = std::collections::BTreeSet::new();
    for &declaration in &GAME_DECLARATIONS {
        let class = reachable_census_class(declaration);
        assert_eq!(
            class,
            rob_core::unscored_mechanics_class(declaration),
            "census quotient agrees with the S1 partition"
        );
        match declaration {
            Declaration::PipTrump(_) => {
                assert_eq!(class, UnscoredMechanicsClass::PipTrumpClass)
            }
            Declaration::DoublesTrump => {
                assert_eq!(class, UnscoredMechanicsClass::DoublesTrumpClass)
            }
            Declaration::NoTrump => assert_eq!(class, UnscoredMechanicsClass::NoTrumpClass),
        }
        classes.insert(format!("{class:?}"));
    }
    classes.len() as u64
}

/// Build a certified contract with the given bidder and declaration.
fn contract_with(bidder: Seat, declaration: Declaration) -> Contract {
    let config = RulesConfig::new(7, 7).expect("valid config");
    let shaker = bidder.offset(3);
    let (m0, _) = MatchState::start(config, shaker);
    let ids: Vec<DominoId> = rob_core::all_ids().collect();
    let hands: [DominoSet; 4] =
        core::array::from_fn(|s| DominoSet::from_ids(ids[s * 7..(s + 1) * 7].iter().copied()));
    let deal = DealWorld::new(hands).expect("valid deal");
    let (mut attempt, m1, _, _) = begin_deal_attempt(&m0, deal, 0).expect("begin");
    let p30 = AuctionAction::Bid(BidValue::Point(PointAmount::new(30).expect("30")));
    for action in [
        p30,
        AuctionAction::Pass,
        AuctionAction::Pass,
        AuctionAction::Pass,
    ] {
        let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
        attempt = next;
    }
    let pending = match close_auction(attempt, &m1, config).expect("closes") {
        CloseAuctionOutcome::Pending(p) => p,
        CloseAuctionOutcome::AllPass(_) => unreachable!(),
    };
    contract_from_auction(pending.win(), declaration, config).expect("certified")
}

/// `x-r_tra_corpus_commutation` (x:004): for all 49 ordered transports of
/// the per-trump S3 hands — the transported prefix is legal on the S2
/// machine, every transported transition is accepted by the S6 symbolic
/// validator, and at every depth `f(N_t(prefix)) = N_u(f(prefix))`.
/// Returns (legal hands, accepted transitions, NF equalities).
pub fn corpus_commutation_check() -> (u64, u64, u64) {
    let viewer = Seat::ALL[0];
    let config = RulesConfig::new(7, 7).expect("valid config");
    let mut legal_hands = 0u64;
    let mut accepted_transitions = 0u64;
    let mut nf_equalities = 0u64;
    #[allow(clippy::needless_range_loop)] // t and u index PIPS as trump labels
    for t in 0..7usize {
        // Source-side NF trajectories for the 12 hands of pip trump t.
        for hand_index in 0..12u64 {
            let index = t as u64 * 12 + hand_index;
            let hand = s3_corpus_hand(index);
            assert_eq!(hand.declaration, Declaration::PipTrump(PIPS[t]));
            let trajectory = mechanical_trajectory(&hand, viewer);
            let source_nfs: Vec<(TotalSupportNormalForm, Vec<DominoId>)> = (0..=28)
                .map(|k| {
                    let cells = derive_rule_cells(&trajectory[k]);
                    let (abstract_cells, order) = cells.to_abstract();
                    (
                        rob_core::compile_total_support(&abstract_cells, None),
                        order,
                    )
                })
                .collect();
            for u in 0..7usize {
                let transport = pip_trump_transport(PIPS[t], PIPS[u]);
                let target_declaration = Declaration::PipTrump(PIPS[u]);
                // Transported deal and trace.
                let mapped_hands: [DominoSet; 4] = core::array::from_fn(|s| {
                    transport_set(&transport, hand.deal.hand(Seat::ALL[s]))
                });
                let mapped_deal = DealWorld::new(mapped_hands).expect("bijective image");
                let mapped_trace: Vec<Play> = hand
                    .steps
                    .iter()
                    .map(|s| {
                        transport_play(
                            &transport,
                            Play {
                                actor: s.actor,
                                domino: s.domino,
                            },
                        )
                    })
                    .collect();

                // (1) Legality on the S2 machine.
                let shaker = Seat::ALL[(index % 4) as usize];
                let (m0, _) = MatchState::start(config, shaker);
                let (mut attempt, m1, _, _) =
                    begin_deal_attempt(&m0, mapped_deal, 0).expect("begin");
                // Reuse the hand's original auction actions (they carry
                // no tiles; the bidder is unchanged by transport).
                let script: Vec<AuctionAction> = hand
                    .events
                    .iter()
                    .filter_map(|e| match e {
                        rob_core::BasePublicEvent::Bid { action, .. } => Some(*action),
                        _ => None,
                    })
                    .collect();
                for action in script {
                    let (next, _) = apply_auction_action(&attempt, action, config).expect("legal");
                    attempt = next;
                }
                let pending = match close_auction(attempt, &m1, config).expect("closes") {
                    CloseAuctionOutcome::Pending(p) => p,
                    CloseAuctionOutcome::AllPass(_) => unreachable!("scripts always bid"),
                };
                let (mut objective, _, _) =
                    begin_contracted_play(pending, target_declaration, &m1, config)
                        .expect("begin play");
                for play in &mapped_trace {
                    assert!(
                        legal_plays(&objective).contains(play.domino),
                        "transported play is legal (x:004)"
                    );
                    let (next, _, _) = apply_play(&objective, play.domino).expect("legal");
                    objective = next;
                }
                legal_hands += 1;

                // (2) Symbolic acceptance + (3) NF commutation per depth.
                let bidder = hand.bidder;
                let certificate = SymbolicTraceCertificate {
                    viewer,
                    viewer_initial_hand: transport_set(&transport, hand.deal.hand(viewer)),
                    contract: contract_with(bidder, target_declaration),
                    trace: mapped_trace,
                    claimed_pool: DominoSet::empty(),
                    claimed_final: {
                        let empty = rob_core::AbstractCells::new(
                            0,
                            core::array::from_fn(|_| Vec::new()),
                            [0, 0, 0],
                        )
                        .expect("empty system");
                        rob_core::compile_total_support(&empty, None)
                    },
                };
                // Depth 0 commutation.
                let (expected0, order0) =
                    transport_normal_form(&transport, &source_nfs[0].0, &source_nfs[0].1);
                {
                    let pool = DominoSet::from_ids(order0.iter().copied());
                    let tiles: Vec<DominoId> = pool.iter().collect();
                    assert_eq!(tiles, order0, "canonical order after transport");
                    let unrestricted = rob_core::AbstractCells::new(
                        tiles.len(),
                        core::array::from_fn(|_| vec![true; tiles.len()]),
                        [7; 3],
                    )
                    .expect("unrestricted");
                    let target_initial = rob_core::compile_total_support(&unrestricted, None);
                    assert_eq!(target_initial, expected0, "depth-0 NF commutation");
                    nf_equalities += 1;
                }
                let mut depth = 1usize;
                let accepted = rob_core::validate_symbolic_trace_with(
                    &certificate,
                    |play_index, support, order| {
                        let (expected, expected_order) = transport_normal_form(
                            &transport,
                            &source_nfs[play_index + 1].0,
                            &source_nfs[play_index + 1].1,
                        );
                        assert_eq!(order, expected_order, "pool image agrees");
                        assert_eq!(
                            support, &expected,
                            "f(N_t(prefix)) = N_u(f(prefix)) at depth {depth}"
                        );
                        depth += 1;
                        accepted_transitions += 1;
                    },
                );
                assert!(accepted.is_ok(), "transported trace accepted (x:004)");
                nf_equalities += 28;
            }
        }
    }
    (legal_hands, accepted_transitions, nf_equalities)
}

/// Build the canonical S9 receipt (§9.1 tier labeling).
pub fn receipt() -> String {
    let mut r = Receipt::new("S9");
    r.line("# exchange", "004 (CONFIRMED 2026-07-27)");
    r.line("r_tra_class_quotient", &class_quotient_check().to_string());
    let (hands, transitions, equalities) = corpus_commutation_check();
    r.line(
        "x-r_tra_corpus_commutation",
        &format!(
            "{} legal transported hands; {} accepted transitions; {} NF equalities",
            hands,
            fmt_commas(transitions as u128),
            fmt_commas(equalities as u128)
        ),
    );
    r.line(
        "r_tra_unscored_only",
        "unscored surface only; no scored transport API (compile_fail doctest)",
    );
    r.finish()
}
