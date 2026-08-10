//! Cross-validation against the v0.4 §14.2 trick-6 experiment on receipt
//! hand 0.
//!
//! Every number pinned here is exploratory tier: a regression pin transcribed
//! from the reported experiment record (v0.4 §14.2, restated in PLAN.md's
//! ground-truth bridges), NOT an axiom and NOT a promoted status. A failure
//! means walt and the report disagree and the discrepancy protocol applies.

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::{Decl, Domino, DominoSet, Pip, Seat, Team};
use walt_geom::{q, Line};
use walt_kernel::Kernel;
use walt_strat::{fixed_field_root_lines, pi_census, Direction};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn d(s: &str) -> Domino {
    s.parse().expect("a domino literal")
}

/// The §14.2 kernel: receipt hand 0, start of trick 6.
fn trick6_kernel() -> Kernel {
    let r = receipt();
    Kernel::from_receipt_trick(&r.hands[0], 6).expect("a valid kernel")
}

/// The §14.2 direction: future trick differential plus lambda times the
/// capture differential of 4:1, for the focal partnership T1 = {S1, S3}.
fn direction() -> Direction {
    Direction::trick_diff_plus_tile(d("4-1"))
}

/// The domain rows of §14.2, checked before anything is solved on them.
#[test]
fn the_domain_is_the_reported_one() {
    let k = trick6_kernel();
    assert_eq!(k.decl(), Decl::PipTrump(Pip::new(3).expect("a pip")));
    assert_eq!(k.viewer(), Seat::S1);
    assert_eq!(
        k.viewer_hand(),
        [d("0-0"), d("2-1")].into_iter().collect::<DominoSet>()
    );
    assert!(k.pool().contains(d("4-1")), "the valued tile is unseen");
    assert_eq!(k.count(), 90, "exact hidden fiber size");
    for h in k.hidden() {
        assert_eq!(h.capacity, 2, "six unseen tiles distributed 2,2,2");
    }
}

/// §14.2 fixed-field hidden solve: `Q^H(0:0) = 2/3 + (1/5) lambda` and
/// `Q^H(2:1) = -2/3 - (1/3) lambda`, crossing at -5/2, so 0:0 is optimal on
/// the whole nonnegative ray.
#[test]
fn fixed_field_lines_match_the_report() {
    let k = trick6_kernel();
    let lines = fixed_field_root_lines(&k, Team::T1, &direction())
        .expect("no post-root focal choice exists at horizon 2");
    assert_eq!(
        lines,
        vec![
            (d("0-0"), Line::new(q(2, 3), q(1, 5))),
            (d("2-1"), Line::new(q(-2, 3), q(-1, 3))),
        ]
    );
    let cross = lines[0].1.crossing(&lines[1].1).expect("distinct slopes");
    assert_eq!(cross, q(-5, 2));
}

/// §14.2 worldwise perfect-information response census: 90 worlds, every
/// world/root curve one affine line, eight parametric root-Q classes of sizes
/// (26, 22, 16, 12, 8, 2, 2, 2), four baseline value classes at lambda = 0,
/// and an optimal-action correspondence refining 2 -> 3.
#[test]
fn pi_census_matches_the_report() {
    let k = trick6_kernel();
    let census = pi_census(&k, Team::T1, &direction());

    assert_eq!(census.worlds, 90);
    assert_eq!(census.actions, vec![d("0-0"), d("2-1")]);
    assert_eq!(census.curves, 180);
    assert_eq!(
        census.multisegment_curves, 0,
        "every world/root response was one affine line"
    );

    assert_eq!(census.parametric.len(), 8, "parametric root-Q classes");
    assert_eq!(census.parametric_sizes(), vec![26, 22, 16, 12, 8, 2, 2, 2]);
    assert_eq!(census.baseline.len(), 4, "value vectors at lambda = 0");
    assert_eq!(census.action.len(), 3, "optimal-action correspondences");
    assert_eq!(
        census.baseline_action.len(),
        2,
        "optimal-action sets at lambda = 0"
    );

    // The reported boundary tie: in eight worlds the two roots tie at
    // lambda = 0 and 2:1 (action index 1) is strictly optimal for every
    // lambda > 0 -- a resolution at 0+, not an interior crossing.
    let tie_resolved: usize = census
        .action
        .iter()
        .filter(|(c, _)| c.baseline() == 0b11 && c.after[0] == 0b10)
        .map(|(_, n)| n)
        .sum();
    assert_eq!(tie_resolved, 8, "the §14.2 boundary-tie worlds");

    // The full three-class split -- 0:0 strictly optimal in 62 worlds, the
    // 8 above, both roots tied on the whole ray in 20 -- is walt's own
    // computed refinement. §14.2 states only the 8; 62 and 20 are frozen here
    // as walt-tier regression pins consistent with (not sourced from) the
    // report.
    let sizes: Vec<(u32, u32, usize)> = census
        .action
        .iter()
        .map(|(c, n)| (c.baseline(), c.after[0], *n))
        .collect();
    assert_eq!(
        sizes,
        vec![(0b01, 0b01, 62), (0b11, 0b10, 8), (0b11, 0b11, 20)]
    );
}

/// The census classes must partition the fiber, and each key refines the
/// next: parametric classes refine baseline classes (4 -> 8) and action
/// classes (2 -> 3 at the correspondence level).
#[test]
fn census_partitions_are_consistent() {
    let k = trick6_kernel();
    let census = pi_census(&k, Team::T1, &direction());
    for map_total in [
        census.parametric.values().sum::<usize>(),
        census.baseline.values().sum::<usize>(),
        census.action.values().sum::<usize>(),
        census.baseline_action.values().sum::<usize>(),
    ] {
        assert_eq!(map_total, census.worlds);
    }
    assert!(census.parametric.len() >= census.baseline.len());
    assert!(census.action.len() >= census.baseline_action.len());
    // The true deal of the receipt sits in one of the classes by
    // construction; the fiber machinery already pins that membership.
}
