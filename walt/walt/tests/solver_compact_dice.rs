#![cfg(all(feature = "compact-dice", feature = "parallel"))]

use std::sync::{atomic::Ordering, Arc};
use std::time::Duration;
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Decl, Domino, Seat, Team};
use walt::solver::{
    self, Deadline, Field, InnerBelief, Key, Shared, Solver, SplitMix64, INNER_SEED,
};

struct Fixture {
    decl: Decl,
    key: Key,
    viewer: Seat,
    hand: u32,
    worlds: Vec<[u32; 4]>,
    seeds: Vec<u64>,
    tiles: Vec<u8>,
    tricks: usize,
    bid: u8,
    boundary_played: u32,
}

// Construct an actual legal public prefix, then draw each viewer belief world
// through the production sampler. The two solvers receive identical worlds and
// independent Dice tapes for each sample ID, including duplicate deals.
fn fixture(decl: Decl, tricks: usize, partial: usize, n: usize, seed: u64) -> Fixture {
    let mut rng = SplitMix64(seed);
    for _ in 0..10_000 {
        let mut deck: Vec<usize> = (0..28).collect();
        for i in (1..28).rev() {
            let j = rng.below((i + 1) as u64) as usize;
            deck.swap(i, j);
        }
        let mut hands = [0u32; 4];
        for seat in 0..4 {
            for &t in &deck[seat * 7..seat * 7 + 7] {
                hands[seat] |= 1 << t;
            }
        }
        let mut leader = rng.below(4) as usize;
        let mut played = 0u32;
        let mut scores = [0u8; 2];
        for _ in 0..7 - tricks {
            let mut trick_tiles = [Domino::ALL[0]; 4];
            let mut led = None;
            for (i, slot) in trick_tiles.iter_mut().enumerate() {
                let seat = (leader + i) % 4;
                let legal = solver::mask_bits(solver::mask_of(legal_plays(
                    decl,
                    solver::set_of(hands[seat]),
                    led,
                )));
                let t = usize::from(legal[rng.below(legal.len() as u64) as usize]);
                *slot = Domino::ALL[t];
                if i == 0 {
                    led = Some(decl.led_context(*slot));
                }
                hands[seat] &= !(1 << t);
                played |= 1 << t;
            }
            let trick = Trick::new(Seat::from_index(leader).unwrap(), trick_tiles).unwrap();
            leader = trick.winner(decl).index();
            scores[leader & 1] += trick.points() as u8;
        }
        let boundary_played = played;
        let mut plays = Vec::new();
        let mut led = None;
        for i in 0..partial {
            let seat = (leader + i) % 4;
            let legal = solver::mask_bits(solver::mask_of(legal_plays(
                decl,
                solver::set_of(hands[seat]),
                led,
            )));
            let t = legal[rng.below(legal.len() as u64) as usize];
            let tile = Domino::ALL[t as usize];
            if i == 0 {
                led = Some(decl.led_context(tile));
            }
            hands[seat] &= !(1 << t);
            played |= 1 << t;
            plays.push(t);
        }
        // Keep the root undecided so the recurrence, not only score cutoffs,
        // is tested. A legal 30..42 bid always uses the same score threshold.
        let bid = (30..=42).find(|&b| scores[1] < b && scores[0] <= 42 - b);
        let Some(bid) = bid else {
            continue;
        };
        let viewer = Seat::from_index((leader + partial) % 4).unwrap();
        let hand = hands[viewer.index()];
        let key = Key {
            voids: None,
            played,
            leader: leader as u8,
            plays,
            banked_t1: scores[1],
            banked_t0: scores[0],
            alive: 0,
        };
        let mut sizes = [tricks; 4];
        for i in 0..partial {
            sizes[(leader + i) % 4] -= 1;
        }
        let mut inner = SplitMix64(
            INNER_SEED
                ^ solver::mix(viewer.index() as u64)
                ^ solver::mix(u64::from(hand))
                ^ solver::record_hash(&key),
        );
        let mut worlds = InnerBelief::Voidless
            .sample(
                decl,
                viewer,
                hand,
                &key,
                sizes,
                n,
                &mut inner,
                Deadline::after(Duration::from_secs(5)),
            )
            .unwrap();
        if n >= 2 {
            worlds[1] = worlds[0];
        }
        let seeds = (0..n).map(|_| inner.next_u64()).collect();
        let mut tiles = solver::mask_bits(solver::mask_of(legal_plays(
            decl,
            solver::set_of(hand),
            led,
        )));
        tiles.reverse(); // action_values must preserve a caller's comparison order.
        return Fixture {
            decl,
            key,
            viewer,
            hand,
            worlds,
            seeds,
            tiles,
            tricks,
            bid,
            boundary_played,
        };
    }
    panic!("could not find an undecided legal prefix");
}

fn evaluate_with(
    f: &Fixture,
    key: &Key,
    belief: InnerBelief,
    reference: bool,
    tiles: &[u8],
    deadline: Deadline,
) -> Option<solver::selection::Values> {
    let shared = Arc::new(
        Shared::new(
            f.decl,
            f.bid,
            vec![f.worlds.len()],
            f.boundary_played,
            f.tricks,
            deadline,
        )
        .with_inner_belief(belief),
    );
    let make_solver = || {
        Solver::new(
            Arc::clone(&shared),
            f.viewer,
            f.hand,
            f.viewer.team() == Team::T1,
            f.worlds.clone(),
            f.seeds.clone(),
            Field::Dice,
        )
    };
    if reference {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
            .install(|| make_solver().parallel().action_values(key, tiles))
    } else {
        make_solver().action_values(key, tiles)
    }
}

fn evaluate(
    f: &Fixture,
    reference: bool,
    tiles: &[u8],
    deadline: Deadline,
) -> Option<solver::selection::Values> {
    evaluate_with(f, &f.key, InnerBelief::Voidless, reference, tiles, deadline)
}

#[test]
fn matches_original_dice_for_partial_tricks_and_sample_id_multiplicity() {
    let mut cases = 0;
    for (di, decl) in [Decl::STRAIGHT[0], Decl::STRAIGHT[7], Decl::STRAIGHT[8]]
        .into_iter()
        .enumerate()
    {
        for partial in 0..4 {
            for n in [1, 2, 8] {
                let f = fixture(
                    decl,
                    4,
                    partial,
                    n,
                    0x9090_4177 + (di * 100 + partial * 10 + n) as u64,
                );
                let a = evaluate(
                    &f,
                    false,
                    &f.tiles,
                    Deadline::after(Duration::from_secs(10)),
                );
                let b = evaluate(&f, true, &f.tiles, Deadline::after(Duration::from_secs(10)));
                assert_eq!(a, b, "declaration {di}, partial {partial}, samples {n}");
                if f.tiles.len() > 1 {
                    let subset = &f.tiles[1..];
                    let a = evaluate(&f, false, subset, Deadline::after(Duration::from_secs(10)));
                    let b = evaluate(&f, true, subset, Deadline::after(Duration::from_secs(10)));
                    assert_eq!(a, b, "candidate subset");
                }
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 36);
}

#[test]
fn expired_deadline_refuses_the_whole_vector() {
    let f = fixture(Decl::STRAIGHT[8], 4, 2, 8, 0xfeed_beef);
    assert!(evaluate(&f, false, &f.tiles, Deadline::after(Duration::ZERO)).is_none());
}

#[test]
fn counted_void_record_and_external_cancellation_keep_dice_values_sound() {
    let f = fixture(Decl::STRAIGHT[7], 4, 2, 8, 0xface_600d);
    let mut key = f.key.clone();
    key.voids = Some([0; 4]);
    let compact = evaluate_with(
        &f,
        &key,
        InnerBelief::VoidsCounted,
        false,
        &f.tiles,
        Deadline::after(Duration::from_secs(10)),
    );
    let reference = evaluate_with(
        &f,
        &key,
        InnerBelief::VoidsCounted,
        true,
        &f.tiles,
        Deadline::after(Duration::from_secs(10)),
    );
    assert_eq!(compact, reference);

    let shared = Arc::new(Shared::new(
        f.decl,
        f.bid,
        vec![8],
        f.boundary_played,
        f.tricks,
        Deadline::after(Duration::from_secs(10)),
    ));
    let solver = Solver::new(
        Arc::clone(&shared),
        f.viewer,
        f.hand,
        f.viewer.team() == Team::T1,
        f.worlds,
        f.seeds,
        Field::Dice,
    );
    shared.dead.store(true, Ordering::Relaxed);
    assert!(solver.action_values(&f.key, &f.tiles).is_none());
    assert_eq!(shared.compact_dice_calls.load(Ordering::Relaxed), 1);
}

#[cfg(feature = "bounded-choice")]
fn modeled_reference(f: &Fixture, n: usize, pool: &rayon::ThreadPool) -> solver::selection::Values {
    let mut sizes = [f.tricks; 4];
    for i in 0..f.key.plays.len() {
        sizes[(usize::from(f.key.leader) + i) % 4] -= 1;
    }
    let mut rng = SplitMix64(
        INNER_SEED
            ^ solver::mix(f.viewer.index() as u64)
            ^ solver::mix(u64::from(f.hand))
            ^ solver::record_hash(&f.key),
    );
    let worlds = InnerBelief::Voidless
        .sample(
            f.decl,
            f.viewer,
            f.hand,
            &f.key,
            sizes,
            n,
            &mut rng,
            Deadline::after(Duration::from_secs(10)),
        )
        .unwrap();
    let seeds = (0..n).map(|_| rng.next_u64()).collect();
    let shared = Arc::new(Shared::new(
        f.decl,
        f.bid,
        vec![n],
        f.boundary_played,
        f.tricks,
        Deadline::after(Duration::from_secs(10)),
    ));
    let mut tiles = f.tiles.clone();
    tiles.sort_unstable();
    pool.install(|| {
        Solver::new(
            shared,
            f.viewer,
            f.hand,
            f.viewer.team() == Team::T1,
            worlds,
            seeds,
            Field::Dice,
        )
        .parallel()
        .action_values(&f.key, &tiles)
        .unwrap()
    })
}

#[cfg(feature = "bounded-choice")]
fn modeled_bounded(f: &Fixture, n: usize, deadline: Deadline) -> (Option<u8>, Arc<Shared>) {
    let shared = Arc::new(Shared::new(
        f.decl,
        f.bid,
        vec![n],
        f.boundary_played,
        f.tricks,
        deadline,
    ));
    let solver = Solver::new(
        Arc::clone(&shared),
        f.viewer,
        f.hand,
        f.viewer.team() == Team::T1,
        Vec::new(),
        Vec::new(),
        Field::Level(0),
    );
    let legal_mask = f.tiles.iter().fold(0u32, |mask, &t| mask | (1 << t));
    let choice = solver.modeled_choice(0, &f.key, f.viewer, f.hand, legal_mask);
    (choice, shared)
}

#[test]
#[cfg(feature = "bounded-choice")]
fn bounded_l0_choice_matches_complete_fixed_comparison() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    let mut ties = 0;
    let mut probes = 0;
    let mut rejections = 0;
    let mut objectives = [0; 2];
    let mut cases = 0;
    for (di, decl) in [Decl::STRAIGHT[0], Decl::STRAIGHT[7], Decl::STRAIGHT[8]]
        .into_iter()
        .enumerate()
    {
        for partial in 0..4 {
            for n in [1, 2, 8] {
                for tricks in [3, 5] {
                    let seed = 0xb0ad_005e + (di * 1000 + partial * 100 + n * 10 + tricks) as u64;
                    let f = fixture(decl, tricks, partial, n, seed);
                    let values = modeled_reference(&f, n, &pool);
                    let expected = solver::best_of(&values, f.viewer.team() == Team::T1);
                    let (actual, shared) =
                        modeled_bounded(&f, n, Deadline::after(Duration::from_secs(10)));
                    assert_eq!(
                        actual,
                        Some(expected),
                        "declaration {di}, partial {partial}, samples {n}, tricks {tricks}"
                    );
                    let bypassed = cfg!(feature = "bypass-l0-cache")
                        && (cfg!(feature = "bypass-l0-cache-all") || tricks <= 2);
                    assert_eq!(shared.pi_cache_len(), usize::from(!bypassed));
                    assert_eq!(shared.compact_dice_calls.load(Ordering::Relaxed), 1);
                    probes += shared.bounded_choice_probes.load(Ordering::Relaxed);
                    rejections += shared.bounded_choice_rejections.load(Ordering::Relaxed);
                    let best = &values.iter().find(|(t, _)| *t == expected).unwrap().1;
                    ties += usize::from(values.iter().filter(|(_, v)| v == best).count() > 1);
                    objectives[usize::from(f.viewer.team() == Team::T1)] += 1;
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 72);
    assert!(ties > 0, "the corpus exercises the ascending tie rule");
    assert!(probes > 0, "the corpus exercises count windows");
    assert!(rejections > 0, "the corpus certifies losing candidates");
    assert!(
        objectives.iter().all(|&n| n > 0),
        "both viewer objectives occur"
    );
    // Opening and last-trick cases exercise the same bounded entry point.
    for tricks in [1, 7] {
        let f = fixture(Decl::STRAIGHT[8], tricks, 1, 8, 0x7770_0000 + tricks as u64);
        let values = modeled_reference(&f, 8, &pool);
        let expected = solver::best_of(&values, f.viewer.team() == Team::T1);
        assert_eq!(
            modeled_bounded(&f, 8, Deadline::after(Duration::from_secs(10))).0,
            Some(expected)
        );
    }
}

#[test]
#[cfg(feature = "bounded-choice")]
fn aborted_bounded_choice_never_enters_policy_cache() {
    let f = fixture(Decl::STRAIGHT[0], 5, 2, 8, 0xc001_cafe);
    let (choice, shared) = modeled_bounded(&f, 8, Deadline::after(Duration::ZERO));
    assert_eq!(choice, None);
    assert_eq!(shared.pi_cache_len(), 0);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    let expected = solver::best_of(
        &modeled_reference(&f, 8, &pool),
        f.viewer.team() == Team::T1,
    );
    assert_eq!(
        modeled_bounded(&f, 8, Deadline::after(Duration::from_secs(10))).0,
        Some(expected)
    );
}
