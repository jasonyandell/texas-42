#![cfg(feature = "hand-cache")]

mod common;

use std::{sync::Arc, time::Duration};
use walt::rules::{replay::state_before_trick, rules::legal_plays, Domino, Seat};
use walt::solver::{
    adaptive::RootPosition,
    mask_of,
    policy::{continuation_frame, t1_frame_bid},
    selection::Rule,
    set_of, Deadline, Field, InnerBelief, Key, Shared, Solver,
};

fn host(sh: Arc<Shared>) -> Solver {
    Solver::new(sh, Seat::ALL[0], 0, false, vec![], vec![], Field::Level(0))
}

#[test]
fn equivalent_boundaries_reuse_identical_policies_at_every_partial_trick() {
    let receipt = common::receipt();
    let mut seen_seats = 0u8;
    for hand in &receipt.hands {
        let position = RootPosition::from_receipt_trick(hand, 6).unwrap();
        let (world, _) = state_before_trick(hand, 6).unwrap();
        let history: Vec<Domino> = hand.tricks[5].plays.iter().map(|(_, d)| *d).collect();
        for partial in 0..4 {
            let frame = continuation_frame(hand.decl, &position, &history[..partial]);
            let seat = frame.seat;
            seen_seats |= 1 << seat.index();
            let own = mask_of(world[seat.index()]) & !frame.key.played;
            let led = frame
                .key
                .plays
                .first()
                .map(|&t| hand.decl.led_context(Domino::ALL[t as usize]));
            let legal = mask_of(legal_plays(hand.decl, set_of(own), led));
            let bid = t1_frame_bid(position.bid, position.declaring_team);
            for belief in [InnerBelief::Voidless, InnerBelief::VoidsCounted] {
                for rule in [Rule::Fixed, Rule::Refine, Rule::RaceRefine] {
                    let make = |boundary, size| {
                        Shared::new(
                            hand.decl,
                            bid,
                            vec![2, 2],
                            boundary,
                            size,
                            Deadline::after(Duration::from_secs(30)),
                        )
                        .with_inner_belief(belief)
                        .with_modeled_selection(rule)
                    };
                    let key = Key {
                        voids: belief.root_voids(frame.voids),
                        ..frame.key.clone()
                    };
                    let original = Arc::new(make(0, 7));
                    let source = host(Arc::clone(&original));
                    let expected: Vec<_> = (0..=1)
                        .map(|k| source.modeled_choice(k, &key, seat, own, legal).unwrap())
                        .collect();
                    drop(source);
                    let mut original = Arc::try_unwrap(original).ok().unwrap();
                    let entries = original.pi_cache_len();
                    assert!(entries > 0);
                    let mut warm = make(frame.trick_start_played, frame.boundary_hand_size);
                    assert_eq!(warm.take_policy_cache_from(&mut original), entries);
                    assert_eq!(original.pi_cache_len(), 0);
                    let warm = Arc::new(warm);
                    let cached = host(Arc::clone(&warm));
                    let cold = host(Arc::new(make(
                        frame.trick_start_played,
                        frame.boundary_hand_size,
                    )));
                    for k in 0..=1 {
                        assert_eq!(
                            cached.modeled_choice(k, &key, seat, own, legal),
                            Some(expected[k])
                        );
                        assert_eq!(cold.modeled_choice(k, &key, seat, own, legal), Some(expected[k]),
                            "hand {}, partial {partial}, belief {belief:?}, rule {rule:?}, level {k}", hand.id);
                    }
                    assert_eq!(
                        warm.pi_calls_by_level()[1],
                        0,
                        "L1 reuses the transferred cache"
                    );
                    let bypassed =
                        cfg!(feature = "bypass-l0-cache") && belief == InnerBelief::Voidless;
                    assert_eq!(
                        warm.pi_calls_by_level()[0],
                        u64::from(bypassed),
                        "L0 either hits the transferred cache or deliberately recomputes"
                    );
                }
            }
        }
    }
    assert_eq!(seen_seats, 15);
}

#[test]
fn transfer_rejects_incompatible_or_malformed_contexts() {
    let r = common::receipt();
    let hand = &r.hands[4];
    let p = RootPosition::from_receipt_trick(hand, 6).unwrap();
    let frame = continuation_frame(hand.decl, &p, &[]);
    let (world, _) = state_before_trick(hand, 6).unwrap();
    let own = mask_of(world[frame.seat.index()]);
    let legal = mask_of(legal_plays(hand.decl, set_of(own), None));
    let bid = t1_frame_bid(p.bid, p.declaring_team);
    let make = || {
        Shared::new(
            hand.decl,
            bid,
            vec![2, 2],
            0,
            7,
            Deadline::after(Duration::from_secs(30)),
        )
    };
    let original = Arc::new(make());
    host(Arc::clone(&original))
        .modeled_choice(1, &frame.key, frame.seat, own, legal)
        .unwrap();
    let mut original = Arc::try_unwrap(original).ok().unwrap();
    let entries = original.pi_cache_len();
    assert!(
        entries > 0,
        "context refusal is checked with a populated L1 cache"
    );
    for variant in 0..8 {
        let mut other = make();
        match variant {
            0 => other.bid = if bid == 30 { 31 } else { 30 },
            1 => other.n_inner = vec![3, 2],
            2 => other.boundary_hand_size = 6,
            3 => other.boundary_played = 1, // unfinished boundary
            4 => other.boundary_played = 0xf000_0000, // outside the deck
            5 => other = other.with_inner_belief(InnerBelief::VoidsCounted),
            6 => other = other.with_modeled_selection(Rule::Refine),
            7 => {
                other.dcl = *walt::rules::Decl::ALL
                    .iter()
                    .find(|&&d| d != other.dcl)
                    .unwrap()
            }
            _ => unreachable!(),
        }
        assert_eq!(
            other.take_policy_cache_from(&mut original),
            0,
            "variant {variant}"
        );
        assert_eq!(original.pi_cache_len(), entries);
    }
}
