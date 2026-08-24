//! Independent prose-rules bridge required by T1-A12 and GT1-A6.
//!
//! `rob_verify::prose_resolver` deliberately implements the published rules
//! without Rob's declaration algebra.  This test compares that independent
//! resolver with Walt for every declaration, every leader, every possible
//! lead, and every three-subset of the remaining dominoes.  Stable domino and
//! seat indices are the only shared encoding.

use rob_core::{
    Declaration as RobDeclaration, DominoId as RobDominoId, Pip as RobPip, Seat as RobSeat,
};
use rob_verify::prose_resolver::prose_resolve;
use walt::rules::{legal_plays, Context, Decl, Domino, DominoSet, Seat, Trick};

fn prose_is_trump(decl: Decl, domino: Domino) -> bool {
    match decl {
        Decl::PipTrump(pip) => domino.has(pip),
        Decl::DoublesTrump => domino.is_double(),
        Decl::NoTrump => false,
    }
}

fn prose_led_context(decl: Decl, lead: Domino) -> Context {
    if prose_is_trump(decl, lead) {
        Context::Called
    } else {
        Context::Natural(lead.hi())
    }
}

fn prose_follows(decl: Decl, led: Context, domino: Domino) -> bool {
    match led {
        Context::Called => prose_is_trump(decl, domino),
        Context::Natural(pip) => !prose_is_trump(decl, domino) && domino.has(pip),
    }
}

fn prose_legal_plays(decl: Decl, hand: DominoSet, led: Context) -> DominoSet {
    let followers: DominoSet = hand
        .iter()
        .filter(|domino| prose_follows(decl, led, *domino))
        .collect();
    if followers.is_empty() {
        hand
    } else {
        followers
    }
}

fn rob_declaration(decl: Decl) -> RobDeclaration {
    match decl {
        Decl::PipTrump(pip) => RobDeclaration::PipTrump(
            RobPip::new(pip.value()).expect("Walt pips are in the Rob pip universe"),
        ),
        Decl::DoublesTrump => RobDeclaration::DoublesTrump,
        Decl::NoTrump => RobDeclaration::NoTrump,
    }
}

fn rob_seat_index(index: usize) -> RobSeat {
    RobSeat::new(u8::try_from(index).expect("seat index fits in u8"))
        .expect("Walt seats are in the Rob seat universe")
}

#[test]
fn m0_prose_follow_and_led_context_agree_exhaustively() {
    let mut led_cases = 0usize;
    let mut follow_cases = 0usize;

    for decl in Decl::ALL {
        for lead in Domino::ALL {
            assert_eq!(
                decl.led_context(lead),
                prose_led_context(decl, lead),
                "led-context drift for {decl:?}, {lead}"
            );
            led_cases += 1;
        }
        for led in Context::ALL {
            let expected_mask: DominoSet = Domino::ALL
                .into_iter()
                .filter(|domino| prose_follows(decl, led, *domino))
                .collect();
            assert_eq!(decl.effective_incidence(led), expected_mask);

            // Singleton and two-tile hands exhaust both branches of the
            // compelled-follow rule for every pair of physical identities.
            for first in 0..Domino::COUNT {
                let singleton = DominoSet::single(Domino::ALL[first]);
                assert_eq!(
                    legal_plays(decl, singleton, Some(led)),
                    prose_legal_plays(decl, singleton, led)
                );
                follow_cases += 1;
                for second in first + 1..Domino::COUNT {
                    let hand = singleton.union(DominoSet::single(Domino::ALL[second]));
                    assert_eq!(
                        legal_plays(decl, hand, Some(led)),
                        prose_legal_plays(decl, hand, led),
                        "legal-set drift for {decl:?}, {led}, {hand:?}"
                    );
                    follow_cases += 1;
                }
            }
        }
    }

    assert_eq!(led_cases, 252);
    let hands_per_context = Domino::COUNT + Domino::COUNT * (Domino::COUNT - 1) / 2;
    assert_eq!(
        follow_cases,
        Decl::COUNT * Context::COUNT * hands_per_context
    );
    assert_eq!(follow_cases, 29_232);
}

#[test]
fn m0_prose_resolver_agrees_on_every_actor_attributed_four_tile_trick() {
    let mut comparisons = 0usize;

    for decl in Decl::ALL {
        let rob_decl = rob_declaration(decl);
        for lead_index in 0..Domino::COUNT {
            let remaining: Vec<Domino> = Domino::ALL
                .into_iter()
                .filter(|domino| domino.index() != lead_index)
                .collect();
            for first in 0..remaining.len() - 2 {
                for second in first + 1..remaining.len() - 1 {
                    for third in second + 1..remaining.len() {
                        let dominoes = [
                            Domino::ALL[lead_index],
                            remaining[first],
                            remaining[second],
                            remaining[third],
                        ];
                        for leader in Seat::ALL {
                            let trick = Trick::new(leader, dominoes)
                                .expect("the lead and remaining subset are distinct");
                            let rob_plays = core::array::from_fn(|offset| {
                                let actor_index = (leader.index() + offset) % Seat::COUNT;
                                let domino_id = RobDominoId::from_index(dominoes[offset].index())
                                    .expect("Walt and Rob share the triangular domino universe");
                                (rob_seat_index(actor_index), domino_id)
                            });
                            let (rob_winner, rob_points) = prose_resolve(rob_decl, &rob_plays);

                            assert_eq!(
                                trick.winner(decl).index(),
                                rob_winner.index(),
                                "winner drift for {decl:?}, leader {leader}, {dominoes:?}"
                            );
                            assert_eq!(
                                trick.points(),
                                u32::from(rob_points),
                                "point drift for {decl:?}, leader {leader}, {dominoes:?}"
                            );
                            comparisons += 1;
                        }
                    }
                }
            }
        }
    }

    assert_eq!(comparisons, 2_948_400);
}
