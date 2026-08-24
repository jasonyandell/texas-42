//! Cross-validation against v0.4 §14.5 (Experiment 3B) and §14.6
//! (Experiment 4A) on the trick-5 kernel of receipt hand 0.
//!
//! Every number pinned here is exploratory tier: a regression pin transcribed
//! from the reported experiment record, NOT an axiom and NOT a promoted
//! status. A failure means walt and the report disagree and the discrepancy
//! protocol applies. Values marked "walt-tier" below are walt's own computed
//! refinements, frozen as pins consistent with (not sourced from) the record.

use walt::geom::{argmax_correspondence, q, qi, Envelope, Q};
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::strat::{
    hidden_root_values, information_prices, policy_value, zero_information, Direction,
    InfoPartition, InfoPrices, Policy,
};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn d(s: &str) -> Domino {
    s.parse().expect("a domino literal")
}

/// The §14.5 kernel: receipt hand 0, start of trick 5, S1 on lead.
/// A non-binding freeze-44 budget for these test kernels.
const BUDGET: u64 = 10_000_000_000;

fn trick5_kernel() -> Kernel {
    let r = receipt();
    Kernel::from_receipt_trick(&r.hands[0], 5).expect("a valid kernel")
}

/// The four unseen tiles without a 2-pip: the free directions of §14.5.
const FREE: [&str; 4] = ["1-1", "4-1", "4-4", "5-1"];

/// The other eight live tiles: the control directions of §14.6.
const CONTROL: [&str; 8] = ["0-0", "2-0", "2-1", "2-2", "3-2", "4-2", "5-2", "6-2"];

fn dir_for(tile: &str) -> Direction {
    Direction::trick_diff_plus_tile(d(tile))
}

/// The pieces of an envelope as (start, value-at-0-of-the-line, slope).
fn pieces(e: &Envelope) -> Vec<(Q, Q, Q)> {
    e.pieces()
        .iter()
        .map(|p| (p.lo, p.line.a, p.line.b))
        .collect()
}

/// The domain rows of §14.5, checked before anything is solved on them.
#[test]
fn the_domain_is_the_reported_one() {
    let k = trick5_kernel();
    assert_eq!(k.decl(), Decl::PipTrump(Pip::new(3).expect("a pip")));
    assert_eq!(k.viewer(), Seat::S1);
    assert_eq!(
        k.viewer_hand(),
        [d("0-0"), d("2-1"), d("3-2")]
            .into_iter()
            .collect::<DominoSet>(),
        "S1 holds {{0:0, 2:1, 3:2}}: three root actions"
    );
    assert_eq!(k.pool().len(), 9, "nine unseen tiles");
    assert_eq!(k.count(), 1680, "exact hidden fiber size");
    for h in k.hidden() {
        assert_eq!(h.capacity, 3, "nine unseen tiles distributed 3,3,3");
    }
    assert_eq!(
        k.live().len(),
        12,
        "twelve live tiles = twelve valuation directions"
    );
    // "3:2 is the last live trump": absolute live-set mastery (§13.1) holds
    // for it and nothing else.
    assert_eq!(k.masters(), DominoSet::single(d("3-2")));
    // The free/control split is exactly "unseen without a 2-pip".
    let two = Pip::new(2).expect("a pip");
    for t in FREE {
        assert!(k.pool().contains(d(t)) && !d(t).has(two), "{t} is free");
    }
    for t in CONTROL {
        assert!(
            k.live().contains(d(t)) && (d(t).has(two) || k.viewer_hand().contains(d(t))),
            "{t} is a control tile: a 2-pip unseen tile or a viewer tile"
        );
    }
}

/// §14.5 hidden fixed-field solve, identical in all four free directions:
/// `Q^H(0:0)` breaks at 1/5 and 4, `Q^H(2:1)` and `Q^H(3:2)` are affine, and
/// the globally optimal root switches at `lambda* = 7/19`.
#[test]
fn hidden_treatment_reproduces_the_exp3b_record() {
    let k = trick5_kernel();
    for tile in FREE {
        let (solved, _residuals) =
            hidden_root_values(&k, Team::T1, &dir_for(tile), BUDGET).expect("non-binding");
        let actions: Vec<Domino> = solved.iter().map(|(a, _)| *a).collect();
        assert_eq!(actions, vec![d("0-0"), d("2-1"), d("3-2")]);

        assert_eq!(
            pieces(&solved[0].1),
            vec![
                (qi(0), q(37, 21), q(22, 35)),
                (q(1, 5), q(26, 15), q(27, 35)),
                (qi(4), q(176, 105), q(11, 14)),
            ],
            "Q^H(0:0) under {tile}"
        );
        assert_eq!(
            pieces(&solved[1].1),
            vec![(qi(0), q(5, 3), q(20, 21))],
            "Q^H(2:1) under {tile}"
        );
        assert_eq!(
            pieces(&solved[2].1),
            vec![(qi(0), q(37, 21), q(4, 7))],
            "Q^H(3:2) under {tile}"
        );

        // The root switch: 0:0 and 3:2 tie at lambda = 0 (both 37/21, slope
        // resolves to 0:0), 0:0 rules until 7/19, then 2:1 forever.
        let envs: Vec<Envelope> = solved.into_iter().map(|(_, e)| e).collect();
        let c = argmax_correspondence(&envs);
        assert_eq!(c.points, vec![qi(0), q(7, 19)]);
        assert_eq!(c.at_point, vec![0b101, 0b011]);
        assert_eq!(c.after, vec![0b001, 0b010]);
    }
}

/// §14.5: "the eight other directions were affine under H" -- every root
/// curve of every control direction is one line.
#[test]
fn hidden_control_directions_are_affine() {
    let k = trick5_kernel();
    for tile in CONTROL {
        let (solved, _residuals) =
            hidden_root_values(&k, Team::T1, &dir_for(tile), BUDGET).expect("non-binding");
        for (a, e) in solved {
            assert!(e.is_affine(), "Q^H({a:?}) under control direction {tile}");
        }
    }
}

/// §14.6 continuation revelation on the free directions: the hidden
/// three-segment 0:0 becomes the reported nine-segment curve with prices
/// {1/4, 1/3, 1/2, 2/3, 1, 3/2, 2, 3} (the hidden breakpoints 1/5, 7/19, 4
/// are not C breakpoints), `Q^C(2:1)` coincides with `Q^H(2:1)`, the
/// common-root optimum switches at 177/131, the exact prices at lambda = 0
/// are 19/105 + 4051/45360 = 12259/45360, and the root-revealed envelope has
/// 42--53 segments.
#[test]
fn continuation_revelation_reproduces_the_exp4a_record() {
    let k = trick5_kernel();
    for tile in FREE {
        let mut rb = BUDGET;
        let mut stop = None;
        let p = information_prices(&k, Team::T1, &dir_for(tile), BUDGET, &mut rb, &mut stop)
            .expect("non-binding");
        exp4a_free_direction_checks(&p, tile);
    }
}

fn exp4a_free_direction_checks(p: &InfoPrices, tile: &str) {
    assert_eq!(
        pieces(&p.q_c[0].1),
        vec![
            (qi(0), q(68, 35), q(151, 210)),
            (q(1, 4), q(233, 120), q(76, 105)),
            (q(1, 3), q(163, 84), q(611, 840)),
            (q(1, 2), q(543, 280), q(613, 840)),
            (q(2, 3), q(1621, 840), q(125, 168)),
            (qi(1), q(1577, 840), q(223, 280)),
            (q(3, 2), q(3127, 1680), q(113, 140)),
            (qi(2), q(3097, 1680), q(457, 560)),
            (qi(3), q(307, 168), q(23, 28)),
        ],
        "Q^C(0:0) under {tile}"
    );
    assert_eq!(p.q_c[1].1, p.q_h[1].1, "Q^C(2:1) = Q^H(2:1) under {tile}");

    // The common-root switch: 0:0 alone from 0, 2:1 from 177/131 on. The
    // hidden events at 0 (the 3:2 tie) and 7/19 are gone.
    let envs: Vec<Envelope> = p.q_c.iter().map(|(_, e)| e.clone()).collect();
    let c = argmax_correspondence(&envs);
    assert_eq!(c.points, vec![qi(0), q(177, 131)]);
    assert_eq!(c.at_point, vec![0b001, 0b011]);
    assert_eq!(c.after, vec![0b001, 0b010]);

    // §14.6 exact information prices at lambda = 0.
    assert_eq!(p.g_cont.eval(qi(0)), q(19, 105));
    assert_eq!(p.g_root.eval(qi(0)), q(4051, 45360));
    assert_eq!(p.g_total.eval(qi(0)), q(12259, 45360));

    // §14.6: G^cont for root 2:1 vanishes identically on the whole ray.
    assert_eq!(p.g_cont_by_root[1].0, d("2-1"));
    assert!(zero_information(&p.g_cont_by_root[1].1));

    // §14.6: the fully root-revealed envelope had 42--53 segments in the
    // four free directions. The exact per-direction counts are walt-tier
    // pins consistent with that range.
    let expected = match tile {
        "1-1" => 51,
        "4-1" => 51,
        "4-4" => 42,
        "5-1" => 53,
        _ => unreachable!("free tiles only"),
    };
    assert_eq!(p.v_f.pieces().len(), expected, "V^F segments under {tile}");
    assert!((42..=53).contains(&p.v_f.pieces().len()));
}

/// §14.6 control directions under revelation: seven of eight become
/// multisegment under C and F; only the viewer's own last trump 3:2 stays
/// affine in all three treatments. And the lambda = 0 prices and the
/// identically-zero G^cont(2:1) are direction-independent facts, holding in
/// all twelve directions.
#[test]
fn control_directions_under_revelation_match_the_record() {
    let k = trick5_kernel();
    let mut multisegment = 0;
    for tile in CONTROL {
        let mut rb = BUDGET;
        let mut stop = None;
        let p = information_prices(&k, Team::T1, &dir_for(tile), BUDGET, &mut rb, &mut stop)
            .expect("non-binding");

        // Direction-independent at lambda = 0 (the tile term vanishes).
        assert_eq!(p.g_cont.eval(qi(0)), q(19, 105));
        assert_eq!(p.g_root.eval(qi(0)), q(4051, 45360));
        assert!(zero_information(&p.g_cont_by_root[1].1), "under {tile}");

        let c_affine = p.q_c.iter().all(|(_, e)| e.is_affine());
        let f_affine = p.v_f.is_affine();
        if tile == "3-2" {
            assert!(
                c_affine && f_affine,
                "the last trump stays affine in all three treatments"
            );
        } else {
            assert!(
                !c_affine && !f_affine,
                "control direction {tile} becomes multisegment under C and F"
            );
            multisegment += 1;
        }
    }
    assert_eq!(multisegment, 7, "seven of eight");
}

/// §14.5's future focal information-state counts, reproduced as the states
/// with a genuine choice: the record's 168 / 7848 / 504 after roots 0:0 /
/// 2:1 / 3:2 count the reachable states where the viewer has two or more
/// legal actions. The full reachable state totals (forced states included)
/// are walt-tier pins.
#[test]
fn information_state_counts_match_the_record() {
    let k = trick5_kernel();
    let reported: [(&str, usize, usize); 3] = [
        ("0-0", 168, 60360),
        ("2-1", 7848, 69600),
        ("3-2", 504, 164088),
    ];
    for (root, choices, total) in reported {
        let mut pb = BUDGET;
        let mut cap_hit = false;
        let p = InfoPartition::build(&k, d(root), &mut pb, usize::MAX, &mut cap_hit)
            .expect("non-binding");
        assert_eq!(p.choice_states(), choices, "§14.5 count after {root}");
        assert_eq!(p.len(), total, "walt-tier total after {root}");
        // The partition is a derived view of the fiber: every state pools at
        // least one world and never more than the fiber.
        for id in p.ids() {
            let n = p.pooled_nodes(id);
            assert!(n >= 1 && n as u128 <= k.count());
            assert!(!p.legal(id).is_empty());
        }
    }
}

/// §14.6's mechanism behind `G^cont(2:1) == 0`: after leading 2:1 the viewer
/// keeps the last trump and the blank double, and playing the trump next wins
/// both remaining tricks in every world. That continuation is an
/// information-consistent policy, so its (single, affine) value must equal
/// `Q^H(2:1)` -- revelation has nothing to add. The evaluator is the
/// no-maximization code path, so this also cross-checks the H solve.
#[test]
fn a_trump_first_policy_attains_the_hidden_optimum_after_2_1() {
    let k = trick5_kernel();
    let dir = dir_for("4-1");
    let mut pb = BUDGET;
    let mut cap_hit = false;
    let partition =
        InfoPartition::build(&k, d("2-1"), &mut pb, usize::MAX, &mut cap_hit).expect("non-binding");
    let trump = d("3-2");

    let trump_first = Policy::build(&partition, |_, legal| {
        if legal.contains(trump) {
            trump
        } else {
            legal.iter().next().expect("nonempty")
        }
    });
    let mut lb = BUDGET;
    let line =
        policy_value(&k, Team::T1, &dir, &partition, &trump_first, &mut lb).expect("non-binding");
    assert_eq!((line.a, line.b), (q(5, 3), q(20, 21)), "equals Q^H(2:1)");

    // Any other information-consistent policy is dominated pointwise; the
    // lowest-tile policy (blank before trump) is strictly worse. Its exact
    // line is a walt-tier pin.
    let blank_first = Policy::build(&partition, |_, legal| {
        legal.iter().next().expect("nonempty")
    });
    let mut wb = BUDGET;
    let worse =
        policy_value(&k, Team::T1, &dir, &partition, &blank_first, &mut wb).expect("non-binding");
    assert_eq!((worse.a, worse.b), (q(1, 3), q(67, 360)));

    let (hrv, _residuals) = hidden_root_values(&k, Team::T1, &dir, BUDGET).expect("non-binding");
    let q_h_2_1 = &hrv[1].1;
    for x in [qi(0), q(7, 19), qi(1), qi(5)] {
        assert_eq!(line.eval(x), q_h_2_1.eval(x));
        assert!(worse.eval(x) < q_h_2_1.eval(x));
    }
}
