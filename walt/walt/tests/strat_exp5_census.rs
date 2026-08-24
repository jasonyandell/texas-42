//! Cross-validation against the exp5 census probe suite
//! (`walt/probes/exp5`, exploratory tier, landed at commit b3cb523).
//!
//! Every number pinned here is an exploratory-tier regression pin transcribed
//! from `exp5_results.md` / `exp5_records.jsonl`, NOT an axiom and NOT a
//! promoted status (TRUST-01). The probes are the independent second
//! implementation; a failure means walt and the probe disagree and the
//! discrepancy protocol applies.
//!
//! Definitions extracted from the suite (exp5_core.Solver, exp5_census):
//! `q_trick` labels a world by its exact PI root value vector with each trick
//! worth +-1 (focal team minus opponents); `q_points` uses the real scoring
//! differential, each trick worth +-(1 + count points of its four tiles);
//! `act_*` labels by the set of optimal root actions. The focal seat is the
//! trick leader; roots are its tiles, ascending.
//!
//! The h1t3/h3t3 headline pins are censuses of a SAMPLED world set: 10,000
//! exactly-uniform draws with recorded seeds (42042013 / 42042033). The
//! distinct worlds of those exact draws (9,920 / 9,933 -- both match the
//! recorded `n_distinct_worlds_solved`) were regenerated with the probe's own
//! sampler and frozen under `tests/data/`; duplicates cannot add classes, so
//! the census over the distinct set is the census over the draws.

use std::collections::BTreeSet;
use std::path::PathBuf;

use walt::geom::qi;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{Domino, DominoSet, Seat};
use walt::strat::{
    pi_census, pi_root_values, scalar_fiber_census, Direction, ScalarCensus, ScalarPi,
    ScalarValuation,
};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn kernel_at(r: &Receipt, hand_id: usize, trick_no: usize) -> Kernel {
    Kernel::from_receipt_trick(&r.hands[hand_id], trick_no).expect("a valid kernel")
}

/// exp5_core.highest_count_unseen: highest count-point unseen tile, ties by
/// pip sum then tile name -- (hi, lo) lexicographic, which is Domino order.
fn highest_count_unseen(kernel: &Kernel) -> Domino {
    kernel
        .pool()
        .iter()
        .max_by_key(|d| (d.count(), d.pip_sum(), d.index()))
        .expect("a nonempty pool")
}

/// The exhaustive per-kernel table of exp5_results.md, horizons 2 and 3:
/// (hand, trick, fiber, q_trick, act_trick, q_points, act_points).
const EXHAUSTIVE_PINS: &[(usize, usize, usize, usize, usize, usize, usize)] = &[
    (0, 6, 90, 4, 2, 8, 3),
    (1, 6, 90, 4, 2, 4, 2),
    (2, 6, 36, 2, 1, 3, 1),
    (3, 6, 36, 1, 1, 1, 1),
    (4, 6, 90, 3, 2, 6, 3),
    (5, 6, 27, 5, 3, 7, 3),
    (6, 6, 90, 1, 1, 1, 1),
    (7, 6, 90, 2, 1, 2, 1),
    (8, 6, 7, 2, 1, 2, 2),
    (9, 6, 30, 4, 3, 5, 3),
    (10, 6, 19, 4, 2, 7, 3),
    (11, 6, 36, 4, 2, 8, 3),
    (12, 6, 6, 1, 1, 1, 1),
    (0, 5, 1680, 2, 1, 5, 3),
    (1, 5, 1680, 2, 1, 2, 1),
    (2, 5, 1680, 2, 1, 7, 4),
    (3, 5, 200, 1, 1, 1, 1),
    (4, 5, 1680, 6, 3, 15, 3),
    (5, 5, 560, 20, 7, 94, 7),
    (6, 5, 1680, 15, 5, 46, 5),
    (7, 5, 1680, 18, 7, 18, 7),
    (8, 5, 92, 2, 1, 3, 2),
    (9, 5, 1680, 11, 4, 38, 6),
    (10, 5, 700, 23, 7, 50, 7),
    (11, 5, 1120, 6, 2, 34, 7),
    (12, 5, 1680, 3, 2, 6, 2),
];

/// The exhaustive H=2/H=3 rows of the report reproduce exactly, for both
/// valuations and both label kinds, over all 13 hands.
#[test]
fn exhaustive_trick5_and_trick6_censuses_match_the_probe() {
    let r = receipt();
    for &(hand, trick, fiber, q_trick, act_trick, q_points, act_points) in EXHAUSTIVE_PINS {
        let kernel = kernel_at(&r, hand, trick);
        assert_eq!(kernel.count() as usize, fiber, "h{hand}t{trick} fiber");
        let ct = scalar_fiber_census(&kernel, ScalarValuation::trick_only());
        let cp = scalar_fiber_census(&kernel, ScalarValuation::trick_plus_count());
        assert_eq!(ct.worlds, fiber);
        for (label, got, want) in [
            ("q_trick", ct.value_classes.len(), q_trick),
            ("act_trick", ct.action_classes.len(), act_trick),
            ("q_points", cp.value_classes.len(), q_points),
            ("act_points", cp.action_classes.len(), act_points),
        ] {
            assert_eq!(got, want, "h{hand}t{trick} {label} classes");
        }
    }
}

/// The trick-6 `q_param` row of the report: the parametric census (trick
/// differential + lambda times the capture sign of the highest-count unseen
/// tile) through walt's symbolic PI operator. `act_param` is deliberately not
/// pinned: the probe canonicalizes correspondences by segment identity, walt
/// by argmax value with at-point events -- different statistics.
#[test]
fn trick6_parametric_censuses_match_the_probe() {
    // (hand, q_param classes) from the H=2 table.
    const Q_PARAM: &[(usize, usize)] = &[
        (0, 8),
        (1, 4),
        (2, 3),
        (3, 1),
        (4, 6),
        (5, 6),
        (6, 1),
        (7, 4),
        (8, 2),
        (9, 5),
        (10, 7),
        (11, 8),
        (12, 1),
    ];
    let r = receipt();
    for &(hand, want) in Q_PARAM {
        let kernel = kernel_at(&r, hand, 6);
        let dir = Direction::trick_diff_plus_tile(highest_count_unseen(&kernel));
        let census = pi_census(&kernel, kernel.viewer().team(), &dir);
        assert_eq!(census.parametric.len(), want, "h{hand}t6 q_param classes");
    }
}

/// The scalar solver agrees with the S2 symbolic operator wherever both run:
/// every world of every trick-6 fiber, both valuations, root by root.
#[test]
fn scalar_solver_agrees_with_the_symbolic_operator_at_trick6() {
    let r = receipt();
    let directions = [
        (ScalarValuation::trick_only(), Direction::trick_diff()),
        (ScalarValuation::trick_plus_count(), {
            let mut base = walt::geom::FeatureVec::unit_trick();
            for d in Domino::ALL {
                base.tiles[d.index()] = qi(i128::from(d.count()));
            }
            Direction {
                base,
                delta: walt::geom::FeatureVec::zero(),
            }
        }),
    ];
    let mut worlds_checked = 0usize;
    for hand in 0..13 {
        let kernel = kernel_at(&r, hand, 6);
        for (val, dir) in &directions {
            let mut pi = ScalarPi::new(kernel.decl(), kernel.viewer().team(), val.clone());
            for world in kernel.worlds() {
                let scalar = pi.root_values(world.hands(), kernel.viewer());
                let symbolic = pi_root_values(
                    kernel.decl(),
                    world.hands(),
                    kernel.viewer(),
                    kernel.viewer().team(),
                    dir,
                );
                assert_eq!(scalar.len(), symbolic.len());
                for ((d1, v), (d2, env)) in scalar.iter().zip(symbolic.iter()) {
                    assert_eq!(d1, d2);
                    assert_eq!(qi(i128::from(*v)), env.eval(qi(0)), "h{hand}t6 {d1}");
                }
                worlds_checked += 1;
            }
        }
    }
    assert_eq!(worlds_checked, 2 * 647);
}

/// One frozen world sample of the probe run: parses `tests/data/`, checks the
/// header against the kernel, membership of every world in the fiber, and
/// returns the worlds as four-seat deals.
fn fixture_worlds(kernel: &Kernel, name: &str, distinct: usize) -> Vec<[DominoSet; Seat::COUNT]> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(name);
    let text = std::fs::read_to_string(&path).expect("the frozen sample exists");
    let mut lines = text.lines();
    let header = lines.next().expect("a header line");
    assert!(header.contains("seats=0,1,3") && header.contains("focal=2"));
    assert!(header.contains(&format!("distinct={distinct}")));
    assert_eq!(kernel.viewer(), Seat::S2);

    let seats = [Seat::S0, Seat::S1, Seat::S3];
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(distinct);
    for line in lines {
        let mut hands = [DominoSet::EMPTY; Seat::COUNT];
        hands[Seat::S2.index()] = kernel.viewer_hand();
        for (seat, word) in seats.iter().zip(line.split_whitespace()) {
            let bits = u32::from_str_radix(word, 16).expect("a hex mask");
            hands[seat.index()] = DominoSet::from_bits(bits).expect("a 28-bit mask");
        }
        assert!(seen.insert(hands), "worlds in the fixture are distinct");
        // Membership in the void-constrained fiber, via the kernel's own
        // exact test (slot order is the kernel's, seat order is the file's).
        let cells = core::array::from_fn(|i| hands[kernel.hidden()[i].seat.index()]);
        assert!(kernel.contains(&kernel.world(cells)), "world in the fiber");
        out.push(hands);
    }
    assert_eq!(out.len(), distinct, "every recorded distinct world present");
    out
}

fn sampled_census(
    kernel: &Kernel,
    worlds: &[[DominoSet; Seat::COUNT]],
    val: ScalarValuation,
) -> ScalarCensus {
    walt::strat::scalar_census(kernel, val, worlds.iter().copied())
}

/// The true root vector of the receipt's actual deal, for the membership pin.
fn true_world_vector(r: &Receipt, kernel: &Kernel, hand: usize, val: ScalarValuation) -> Vec<i64> {
    let (true_hands, _) =
        walt::rules::replay::state_before_trick(&r.hands[hand], 3).expect("a valid hand");
    let mut pi = ScalarPi::new(kernel.decl(), kernel.viewer().team(), val);
    pi.root_values(true_hands, kernel.viewer())
        .into_iter()
        .map(|(_, v)| v)
        .collect()
}

/// The headline exp5 control-hypothesis pins, unblocked: h1t3 (P6, high
/// control) and h3t3 (P0, no control), identical fiber 756,756, censused on
/// the probe's own recorded 10,000-draw samples. h1t3: 10 `q_points` classes;
/// h3t3: 5,345. Sampled lower bounds of the true fiber censuses -- pinned
/// here as censuses OF THOSE SAMPLES, which is what the probe measured. The
/// probe also recorded `true_world_class_found: true` for both: the receipt's
/// actual deal falls in a class the sample already discovered.
#[test]
fn sampled_h1t3_and_h3t3_censuses_match_the_probe() {
    let r = receipt();
    // (hand, fixture, distinct, q_points, act_points, q_trick, act_trick)
    // from exp5_records.jsonl.
    for (hand, name, distinct, qp, ap, qt, at) in [
        (1, "exp5_sample_h1t3.txt", 9920, 10, 8, 2, 1),
        (3, "exp5_sample_h3t3.txt", 9933, 5345, 31, 1007, 31),
    ] {
        let kernel = kernel_at(&r, hand, 3);
        assert_eq!(kernel.count(), 756_756);
        let worlds = fixture_worlds(&kernel, name, distinct);

        let cp = sampled_census(&kernel, &worlds, ScalarValuation::trick_plus_count());
        assert_eq!(cp.value_classes.len(), qp, "h{hand}t3 q_points classes");
        assert_eq!(cp.action_classes.len(), ap, "h{hand}t3 act_points classes");
        let truth = true_world_vector(&r, &kernel, hand, ScalarValuation::trick_plus_count());
        assert!(
            cp.value_classes.contains_key(&truth),
            "h{hand}t3 true world's class was sampled"
        );

        let ct = sampled_census(&kernel, &worlds, ScalarValuation::trick_only());
        assert_eq!(ct.value_classes.len(), qt, "h{hand}t3 q_trick classes");
        assert_eq!(ct.action_classes.len(), at, "h{hand}t3 act_trick classes");

        if hand == 1 {
            // The recorded h1t3 class representative [15,15,15,15,15].
            assert!(cp.value_classes.contains_key(&vec![15i64; 5]));
        }
    }
}
