//! Value transparency of the `dag-v1` memoized H solver (S5c-m3, the
//! mandatory CI invariant): on every decision the S5c-m2 run measured —
//! the sixteen distinct decisions across the ten measured lessons'
//! basins — the memoized solver returns byte-identical exact `Q^H`
//! action values to the unmemoized tree walk, and never charges more
//! particle-steps. The two famous h0 S1 pins (`Q^H(2-1) = 80/7` vs
//! `Q^H(3-2) = 202/21`) are asserted against both paths explicitly.
//!
//! The decisions are reconstructed directly (`DomainDecision::at`), not
//! through lesson generalization — this pins the solver equivalence, at
//! CI cost (the measured basins are small fibers; the capped big fibers
//! are deliberately NOT here, their dag-v1 outcomes live in the
//! label-transfer r2 results file). Exploratory tier throughout.

use walt_core::{Domino, DominoSet, Seat};
use walt_factory::{load_receipt, DomainDecision};
use walt_geom::q;
use walt_kernel::ReceiptDecision;
use walt_strat::{ScalarHidden, ScalarValuation};

/// Every (hand, seat, trick) the m2 label-transfer run measured
/// (`results/label_transfer_2026-08-10.txt`), deduplicated.
const MEASURED: &[(usize, Seat, usize)] = &[
    (0, Seat::S1, 5),
    (1, Seat::S0, 5),
    (1, Seat::S0, 6),
    (1, Seat::S1, 5),
    (1, Seat::S2, 6),
    (2, Seat::S2, 4),
    (3, Seat::S3, 4),
    (4, Seat::S0, 4),
    (4, Seat::S1, 5),
    (4, Seat::S3, 4),
    (4, Seat::S3, 5),
    (4, Seat::S3, 6),
    (6, Seat::S1, 5),
    (7, Seat::S2, 5),
    (7, Seat::S3, 6),
    (9, Seat::S3, 6),
];

const BUDGET: u64 = 100_000_000;

/// Both solver paths on one reconstructed decision: identical value
/// vectors (as exact rationals and as rendered text), dag steps never
/// above tree steps.
fn check_decision(hand: usize, seat: Seat, trick_no: usize) {
    let receipt = load_receipt();
    let d = DomainDecision::at(&receipt, hand, seat, trick_no);
    let rd = ReceiptDecision::at(&receipt.hands[hand], trick_no, seat)
        .expect("the measured decision reconstructs");
    let solver = ScalarHidden::new(
        d.kernel.decl(),
        d.kernel.viewer(),
        d.kernel.viewer().team(),
        ScalarValuation::trick_plus_count(),
    );
    let worlds: Vec<[DominoSet; Seat::COUNT]> = d.worlds.iter().map(|w| w.hands()).collect();

    let mut tree_budget = BUDGET;
    let tree = solver
        .action_values(&worlds, rd.leader, &rd.prefix, &mut tree_budget)
        .expect("every m2-measured decision fits the tree budget");
    let mut dag_budget = BUDGET;
    let (dag, stats) = solver.action_values_dag(&worlds, rd.leader, &rd.prefix, &mut dag_budget);
    let dag = dag.expect("dag-v1 charges no more than tree-v0");

    assert_eq!(
        tree, dag,
        "h{hand} {seat} t{trick_no}: memoized values must be identical"
    );
    let render = |vs: &[(walt_core::Domino, walt_geom::Q)]| -> String {
        vs.iter()
            .map(|(a, v)| format!("Q^H({a})={v}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    assert_eq!(
        render(&tree),
        render(&dag),
        "h{hand} {seat} t{trick_no}: byte-identical rendering"
    );
    let tree_steps = BUDGET - tree_budget;
    assert!(
        stats.steps <= tree_steps,
        "h{hand} {seat} t{trick_no}: dag steps {} exceed tree steps {tree_steps}",
        stats.steps
    );
    assert_eq!(stats.steps, BUDGET - dag_budget, "stats track the charge");
    // The dag walk's tree-equivalent accounting is exact: it must equal
    // what the unmemoized walk actually charged.
    assert_eq!(
        stats.tree_steps,
        u128::from(tree_steps),
        "h{hand} {seat} t{trick_no}: tree-equivalent accounting is exact"
    );

    if (hand, seat, trick_no) == (0, Seat::S1, 5) {
        let pin = |vs: &[(walt_core::Domino, walt_geom::Q)], tile: &str| {
            let tile: Domino = tile.parse().expect("a tile");
            vs.iter().find(|(a, _)| *a == tile).expect("legal").1
        };
        for vs in [&tree, &dag] {
            assert_eq!(pin(vs, "2-1"), q(80, 7), "the pinned h0 S1 Q^H(2-1)");
            assert_eq!(pin(vs, "3-2"), q(202, 21), "the pinned h0 S1 Q^H(3-2)");
        }
    }
}

#[test]
fn dag_v1_values_are_byte_identical_on_all_m2_measured_decisions() {
    // Independent decisions, fixed chunks over scoped threads (the
    // workspace's parallelism pattern); each check is self-contained, so
    // the result is schedule-independent.
    std::thread::scope(|s| {
        for chunk in MEASURED.chunks(4) {
            s.spawn(move || {
                for &(hand, seat, trick_no) in chunk {
                    check_decision(hand, seat, trick_no);
                }
            });
        }
    });
}
