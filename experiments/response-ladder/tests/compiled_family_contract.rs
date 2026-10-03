use walt::kernel::{Hidden, Kernel};
use walt::rules::rules::legal_plays;
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Seat, Team};
use walt::scheme::{Budget, Frame, PolicyInput, Registry};

use walt_response_ladder::compiled::{Actor, SCHEMA, SCHEMA_V2};
use walt_response_ladder::compiled_family::Family;
use walt_response_ladder::compiled_player::{self, Config, Request};
use walt_response_ladder::mechanics::{domino, PublicState};

fn mask(tiles: impl IntoIterator<Item = u8>) -> u32 {
    tiles.into_iter().map(|tile| 1u32 << tile).sum()
}

fn with_family_input<R>(
    decl: Decl,
    viewer: u8,
    original: &[u32; 4],
    public: &PublicState,
    history: &[(Seat, Domino)],
    f: impl for<'a> FnOnce(&'a PolicyInput<'a>) -> R,
) -> R {
    let viewer_seat = Seat::from_index(viewer as usize).unwrap();
    let viewer_hand = DominoSet::from_bits(original[viewer as usize] & !public.played).unwrap();
    let mut hidden = Vec::new();
    let mut pool = DominoSet::EMPTY;
    for seat in Seat::ALL {
        if seat == viewer_seat {
            continue;
        }
        let remaining = DominoSet::from_bits(original[seat.index()] & !public.played).unwrap();
        let voids: ContextSet = Context::ALL
            .into_iter()
            .filter(|q| {
                let incidence = decl.effective_incidence(*q).bits();
                incidence != 0 && public.voids[seat.index()] & incidence == incidence
            })
            .collect();
        hidden.push(Hidden {
            seat,
            capacity: remaining.len(),
            voids,
        });
        pool = pool.union(remaining);
    }
    let kernel = Kernel::new(
        decl,
        viewer_seat,
        viewer_hand,
        pool,
        hidden.try_into().unwrap(),
    )
    .unwrap();
    let frame = Frame::new(
        kernel,
        Seat::from_index(public.leader as usize).unwrap(),
        public.plays.iter().copied().map(domino).collect(),
        DominoSet::from_bits(public.played).unwrap(),
    )
    .unwrap();
    // The family guard uses the normalized viewer's fixed seat team. The
    // declaring-team field is orthogonal to that role guard.
    let input = PolicyInput::new(
        &frame,
        history,
        [public.banked_t0 as u32, public.banked_t1 as u32],
        30,
        Team::T1,
    )
    .unwrap();
    f(&input)
}

#[test]
fn family_scheme_executes_the_selected_role_across_all_prefixes() {
    let declaring = Actor::new(SCHEMA, vec![12, 1, 8]).unwrap();
    let defending = Actor::new(SCHEMA_V2, vec![14, 15, 2]).unwrap();
    let family = Family::Roles {
        declaring,
        defending,
    };
    let program = family.policy_program_named("family-contract").unwrap();
    let compiled = program.compile(&Registry::standard()).unwrap();
    let original = [mask(0..7), mask(7..14), mask(14..21), mask(21..28)];

    for decl in Decl::ALL {
        for opening_leader in 0..4u8 {
            let mut remaining = original;
            let mut public = PublicState::opening(opening_leader);
            let mut history = Vec::new();
            while history.len() < Domino::COUNT {
                let viewer = public.actor();
                let hand = remaining[viewer as usize] & !public.played;
                let expected = family
                    .actor(viewer)
                    .unwrap()
                    .choose(decl, viewer, hand, &public)
                    .unwrap();
                let actual =
                    with_family_input(decl, viewer, &original, &public, &history, |input| {
                        let mut budget = Budget::new(100_000);
                        let mut controller = compiled.initialize(input, &mut budget).unwrap();
                        compiled
                            .choose_traced(&mut controller, input, &mut budget)
                            .unwrap()
                            .action
                            .index() as u8
                    });
                assert_eq!(
                    actual, expected.action,
                    "{decl:?} leader {opening_leader} viewer {viewer} history {:?}",
                    public.history
                );

                let legal = legal_plays(
                    decl,
                    DominoSet::from_bits(hand).unwrap(),
                    public
                        .plays
                        .first()
                        .map(|tile| decl.led_context(domino(*tile))),
                );
                let tile = legal.iter().next().unwrap().index() as u8;
                remaining[viewer as usize] &= !(1u32 << tile);
                history.push((Seat::from_index(viewer as usize).unwrap(), domino(tile)));
                public = public.after(decl, tile);
            }
        }
    }
}

fn opening_request(bidder: u8, seat: u8) -> Request {
    Request {
        decl: 6,
        bid: 30,
        bidder,
        seat,
        hand: vec![0, 6, 9, 15, 18, 24, 27],
        original_hand: vec![0, 6, 9, 15, 18, 24, 27],
        history: if bidder == seat { vec![] } else { vec![[bidder, 1]] },
        seed: 5538,
        budget_ms: 0.0,
        config: Config {
            outer: 2,
            ..Default::default()
        },
    }
}

#[test]
fn decide_family_reserve_uses_role_for_both_original_bidder_parities() {
    let declaring = Actor::new(SCHEMA, vec![0]).unwrap();
    let defending = Actor::new(SCHEMA_V2, vec![14]).unwrap();
    let c0 = Family::Single(Actor::empty());
    let c1 = Family::Roles {
        declaring: declaring.clone(),
        defending: defending.clone(),
    };
    for (bidder, seat, expected_viewer) in [(1, 1, 1u8), (0, 1, 2u8)] {
        let request = opening_request(bidder, seat);
        let (decl, viewer, hand, _, public) = compiled_player::normalize(&request).unwrap();
        assert_eq!(viewer, expected_viewer);
        let expected = c1
            .actor(viewer)
            .unwrap()
            .choose(decl, viewer, hand, &public)
            .unwrap()
            .action;
        let answer = compiled_player::decide_family(&request, &c0, &c1, None).unwrap();
        assert!(answer.fallback);
        assert_eq!(answer.tile, expected);
        assert!(answer.report.is_none());
    }
}
