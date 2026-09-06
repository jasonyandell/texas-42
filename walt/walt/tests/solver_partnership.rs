//! Focused gates for the bounded partnership evaluator. Exploratory tier.

mod common;

use std::sync::Arc;
use std::time::Duration;

use common::receipt;
use walt::rules::replay::state_before_trick;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, Seat, Team};
use walt::solver::adaptive::RootPosition;
use walt::solver::partnership::{self, Config, FieldProfile, RefusalReason};
use walt::solver::policy::{continuation_frame, t1_frame_bid};
use walt::solver::{
    mask_bits, mask_of, sample_belief, Deadline, Field, Key, Shared, Solver, SplitMix64,
};

const SEED: u64 = 0xD1CE_5041_5254_4E52;

struct Fixture {
    dcl: walt::rules::Decl,
    bid: u8,
    seat: Seat,
    hand: u32,
    legal: u32,
    key: Key,
    sizes: [usize; 4],
    voids: [u32; 4],
    trick_start_played: u32,
    boundary_hand_size: usize,
    world: [u32; 4],
}

fn fixture() -> Fixture {
    let r = receipt();
    let hand = &r.hands[4];
    let trick_no = 6;
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("valid root");
    let frame = continuation_frame(position.decl, &position, &[]);
    let (hands, _) = state_before_trick(hand, trick_no).expect("valid replay");
    let seat = frame.seat;
    let own = mask_of(hands[seat.index()]);
    let legal = mask_of(legal_plays(position.decl, hands[seat.index()], None));
    assert!(legal.count_ones() > 1, "fixture has a real choice");
    let sizes = frame.sizes();
    Fixture {
        dcl: position.decl,
        bid: t1_frame_bid(position.bid, position.declaring_team),
        seat,
        hand: own,
        legal,
        key: frame.key,
        sizes,
        voids: frame.voids,
        trick_start_played: frame.trick_start_played,
        boundary_hand_size: frame.boundary_hand_size,
        world: core::array::from_fn(|i| mask_of(hands[i])),
    }
}

fn config(profile: FieldProfile) -> Config {
    Config {
        profile,
        n_outer: 2,
        n1: 2,
        n0: 2,
        seed: SEED,
        deadline: Deadline::after(Duration::from_secs(30)),
    }
}

fn direct_values(
    f: &Fixture,
    worlds: Vec<[u32; 4]>,
    field: Field,
) -> Vec<(u8, num_rational::BigRational)> {
    let shared = Arc::new(Shared::new(
        f.dcl,
        f.bid,
        vec![2, 2],
        f.trick_start_played,
        f.boundary_hand_size,
        Deadline::after(Duration::from_secs(30)),
    ));
    let solver = Solver::new(
        shared,
        f.seat,
        f.hand,
        f.seat.team() == Team::T1,
        worlds,
        Vec::new(),
        field,
    );
    let values = mask_bits(f.legal)
        .into_iter()
        .map(|tile_index| {
            let tile = Domino::from_index(usize::from(tile_index)).expect("tile < 28");
            let child = solver.child_after_play(&f.key, tile, 0);
            (tile_index, solver.solve(&child).expect("ample deadline"))
        })
        .collect();
    solver.flush_nodes();
    values
}

#[test]
fn seat_profiles_upgrade_exactly_the_declared_seats() {
    for focal in Seat::ALL {
        assert_eq!(FieldProfile::Baseline.seat_levels(focal), [0; 4]);
        let partner = FieldProfile::PartnerOnly.seat_levels(focal);
        for seat in Seat::ALL {
            let expected = usize::from(seat == focal.plus(2));
            assert_eq!(
                partner[seat.index()],
                expected,
                "focal {focal}, seat {seat}"
            );
        }
        let all = FieldProfile::AllLevel1.seat_levels(focal);
        for seat in Seat::ALL {
            assert_eq!(all[seat.index()], usize::from(seat != focal));
        }
    }
}

#[test]
fn seat_levels_all_zero_is_exactly_the_uniform_level_zero_field() {
    let f = fixture();
    let mut rng = SplitMix64(SEED);
    let worlds = sample_belief(
        f.seat.index(),
        f.hand,
        f.key.played,
        f.sizes,
        f.voids,
        2,
        &mut rng,
    )
    .expect("receipt frame is feasible");
    let uniform = direct_values(&f, worlds.clone(), Field::Level(0));
    let per_seat = direct_values(&f, worlds, Field::SeatLevels([0; 4]));
    assert_eq!(per_seat, uniform);
}

#[test]
fn seat_levels_all_one_is_exactly_the_uniform_level_one_field() {
    let f = fixture();
    let mut rng = SplitMix64(SEED);
    let worlds = sample_belief(
        f.seat.index(),
        f.hand,
        f.key.played,
        f.sizes,
        f.voids,
        2,
        &mut rng,
    )
    .expect("receipt frame is feasible");
    let uniform = direct_values(&f, worlds.clone(), Field::Level(1));
    let per_seat = direct_values(
        &f,
        worlds,
        Field::SeatLevels(FieldProfile::AllLevel1.seat_levels(f.seat)),
    );
    assert_eq!(per_seat, uniform);
}

#[test]
fn modeled_level_one_is_independent_of_outer_completions_and_cache_order() {
    let f = fixture();
    let acting = f.seat;
    let mut swapped = f.world;
    let a = acting.plus(1).index();
    let b = acting.plus(2).index();
    swapped.swap(a, b);
    let worlds_a = vec![f.world, swapped];
    let worlds_b = vec![swapped, f.world];
    let cold_choice = |worlds| {
        let shared = Arc::new(Shared::new(
            f.dcl,
            f.bid,
            vec![2, 2],
            f.trick_start_played,
            f.boundary_hand_size,
            Deadline::after(Duration::from_secs(30)),
        ));
        let solver = Solver::new(
            shared,
            acting.plus(1),
            f.world[acting.plus(1).index()],
            false,
            worlds,
            Vec::new(),
            Field::Level(0),
        );
        solver
            .modeled_choice(1, &f.key, acting, f.hand, f.legal)
            .expect("ample deadline")
    };
    let choice_a = cold_choice(worlds_a.clone());
    let choice_b = cold_choice(worlds_b.clone());
    assert_eq!(
        choice_a, choice_b,
        "cold caches cannot observe outer world identity or order"
    );

    let shared = Arc::new(Shared::new(
        f.dcl,
        f.bid,
        vec![2, 2],
        f.trick_start_played,
        f.boundary_hand_size,
        Deadline::after(Duration::from_secs(30)),
    ));
    let first = Solver::new(
        Arc::clone(&shared),
        acting.plus(1),
        f.world[acting.plus(1).index()],
        false,
        worlds_a,
        Vec::new(),
        Field::Level(0),
    );
    let second = Solver::new(
        Arc::clone(&shared),
        acting.plus(1),
        f.world[acting.plus(1).index()],
        false,
        worlds_b,
        Vec::new(),
        Field::Level(0),
    );
    let shared_choice = first
        .modeled_choice(1, &f.key, acting, f.hand, f.legal)
        .expect("ample deadline");
    let after_first = shared.pi_cache_len();
    let repeat_choice = second
        .modeled_choice(1, &f.key, acting, f.hand, f.legal)
        .expect("cache hit is total");
    assert_eq!(shared_choice, repeat_choice);
    assert_eq!(
        shared.pi_cache_len(),
        after_first,
        "revisiting one information state inserts nothing, including nested entries"
    );

    let mut banked_key = f.key.clone();
    banked_key.banked_t1 = banked_key.banked_t1.saturating_add(1);
    let _ = second
        .modeled_choice(1, &banked_key, acting, f.hand, f.legal)
        .expect("banked-score variant terminates");
    let after_banked = shared.pi_cache_len();
    assert!(after_banked > after_first, "banked score is part of PiKey");

    let give = f.hand.trailing_zeros();
    let take = f.world[acting.plus(1).index()].trailing_zeros();
    let changed_hand = (f.hand & !(1u32 << give)) | (1u32 << take);
    let changed_legal = mask_of(legal_plays(f.dcl, walt::solver::set_of(changed_hand), None));
    let _ = second
        .modeled_choice(1, &f.key, acting, changed_hand, changed_legal)
        .expect("own-hand variant terminates");
    assert!(
        shared.pi_cache_len() > after_banked,
        "the acting seat's own hand is part of PiKey"
    );
}

#[test]
fn baseline_evaluator_uses_the_exact_common_world_stream_and_level_zero_field() {
    let f = fixture();
    let report = partnership::evaluate(
        f.dcl,
        f.bid,
        f.seat,
        f.hand,
        f.legal,
        &f.key,
        f.sizes,
        f.voids,
        f.trick_start_played,
        f.boundary_hand_size,
        &config(FieldProfile::Baseline),
    )
    .expect("small late-game solve fits the deadline");
    let mut rng = SplitMix64(SEED);
    let worlds = sample_belief(
        f.seat.index(),
        f.hand,
        f.key.played,
        f.sizes,
        f.voids,
        2,
        &mut rng,
    )
    .expect("receipt frame is feasible");
    let direct = direct_values(&f, worlds, Field::Level(0));
    let reported: Vec<_> = report
        .actions
        .iter()
        .map(|action| (action.tile.index() as u8, action.value.clone()))
        .collect();
    assert_eq!(reported, direct);
}

#[test]
fn bounded_evaluator_is_deterministic_and_reports_every_legal_action() {
    let f = fixture();
    let run = || {
        partnership::evaluate(
            f.dcl,
            f.bid,
            f.seat,
            f.hand,
            f.legal,
            &f.key,
            f.sizes,
            f.voids,
            f.trick_start_played,
            f.boundary_hand_size,
            &config(FieldProfile::PartnerOnly),
        )
        .expect("small late-game solve fits the deadline")
    };
    let a = run();
    let b = run();
    assert_eq!(a.actions, b.actions);
    assert_eq!(a.best(), b.best());
    assert_eq!(a.actions.len(), f.legal.count_ones() as usize);
    assert_eq!(a.stats.outer_worlds, 2);
    assert_eq!(a.stats.pi_calls_by_level.len(), 2);
    assert_eq!(
        a.stats.inner_worlds_by_level[0],
        a.stats.pi_calls_by_level[0] * 2
    );
    assert_eq!(
        a.stats.inner_worlds_by_level[1],
        a.stats.pi_calls_by_level[1] * 2
    );
}

#[test]
fn zero_deadline_aborts_before_sampling_or_action_comparison() {
    let f = fixture();
    let cfg = Config {
        deadline: Deadline::after(Duration::ZERO),
        ..config(FieldProfile::PartnerOnly)
    };
    let refusal = partnership::evaluate(
        f.dcl,
        f.bid,
        f.seat,
        f.hand,
        f.legal,
        &f.key,
        f.sizes,
        f.voids,
        f.trick_start_played,
        f.boundary_hand_size,
        &cfg,
    )
    .expect_err("a zero deadline cannot begin work");
    assert_eq!(refusal.reason, RefusalReason::Deadline);
    assert_eq!(refusal.stats.outer_worlds, 0);
    assert_eq!(refusal.stats.nodes, 0);
}
