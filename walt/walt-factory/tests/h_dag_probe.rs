//! Offline diagnostics for the four r1-capped big-fiber decisions —
//! `#[ignore]`d: these are declared manual runs (minutes to hours), never
//! CI. Exploratory tier; printed numbers are reporting material for the
//! S5c-m3 record, quotable only through a declared results artifact.
//!
//! `probe_uncapped_dag_on_the_r1_capped_fibers` measures each decision's
//! TRUE memoized-DAG size (budget u64::MAX): exact values, dag steps,
//! exact tree-equivalent steps, cache statistics. This is the fact base
//! for any bigger-declared-budget decision and for the walt-math
//! amendment-4 feasibility estimate of an uncapped tree cross-validation.
//!
//! `crosscheck_tree_uncapped` is that cross-validation (amendment 4(i)):
//! the unmemoized tree walk, uncapped, on ALL FOUR fibers (probe-measured
//! tree-equiv 4.2e9..6.5e10 steps — ~1.5 h total at ~2e7 particle-steps
//! per second single-threaded), asserted byte-identical to the memoized
//! values, and written as the declared offline receipt
//! `results/h_tree_crossval_2026-08-10.txt`.

use walt_core::{DominoSet, Seat};
use walt_factory::{load_receipt, DomainDecision};
use walt_kernel::ReceiptDecision;
use walt_strat::{ScalarHidden, ScalarValuation};

/// The four decisions the m2 run capped at tree-v0 10^8
/// (`results/label_transfer_2026-08-10.txt`), ascending fiber size.
const CAPPED: &[(usize, Seat, usize)] = &[
    (1, Seat::S3, 3),
    (11, Seat::S1, 3),
    (1, Seat::S2, 4),
    (5, Seat::S1, 3),
];

fn solve_dag(hand: usize, seat: Seat, trick_no: usize, budget: u64) {
    let receipt = load_receipt();
    let d = DomainDecision::at(&receipt, hand, seat, trick_no);
    let rd = ReceiptDecision::at(&receipt.hands[hand], trick_no, seat)
        .expect("the decision reconstructs");
    let solver = ScalarHidden::new(
        d.kernel.decl(),
        d.kernel.viewer(),
        d.kernel.viewer().team(),
        ScalarValuation::trick_plus_count(),
    );
    let worlds: Vec<[DominoSet; Seat::COUNT]> = d.worlds.iter().map(|w| w.hands()).collect();
    let t = std::time::Instant::now();
    let mut b = budget;
    let (values, stats) = solver.action_values_dag(&worlds, rd.leader, &rd.prefix, &mut b);
    println!(
        "h{hand} {seat} t{trick_no} fiber {}: {:?} — steps={} tree-equiv={} entries={} hits={} key-particles={} (~{} MB keys)",
        worlds.len(),
        t.elapsed(),
        stats.steps,
        stats.tree_steps,
        stats.entries,
        stats.hits,
        stats.key_particles,
        stats.key_particles * 32 / 1_048_576,
    );
    match values {
        None => println!("  CAPPED at budget {budget}"),
        Some(vs) => {
            for (a, v) in vs {
                println!("  Q^H({a})={v}");
            }
        }
    }
}

#[test]
#[ignore = "offline diagnostic: the true dag-v1 size of the four r1-capped fibers"]
fn probe_uncapped_dag_on_the_r1_capped_fibers() {
    for &(hand, seat, trick_no) in CAPPED {
        solve_dag(hand, seat, trick_no, u64::MAX);
    }
}

#[test]
#[ignore = "offline cross-validation receipt (walt-math amendment 4(i)): uncapped tree walk vs dag on all four lifted fibers, ~1.5 h"]
fn crosscheck_tree_uncapped() {
    // The two r3-FAILED decisions first (their cross-validation is
    // mandatory before any quote — a memoization bug would masquerade as
    // label fragility), then the two that held. The receipt is rewritten
    // after every decision so an interrupted run still leaves a valid
    // partial receipt on disk.
    const ORDERED: &[(usize, Seat, usize)] = &[
        (1, Seat::S2, 4),
        (11, Seat::S1, 3),
        (5, Seat::S1, 3),
        (1, Seat::S3, 3),
    ];
    let receipt = load_receipt();
    let mut body = String::new();
    for &(hand, seat, trick_no) in ORDERED {
        let d = DomainDecision::at(&receipt, hand, seat, trick_no);
        let rd = ReceiptDecision::at(&receipt.hands[hand], trick_no, seat)
            .expect("the decision reconstructs");
        let solver = ScalarHidden::new(
            d.kernel.decl(),
            d.kernel.viewer(),
            d.kernel.viewer().team(),
            ScalarValuation::trick_plus_count(),
        );
        let worlds: Vec<[DominoSet; Seat::COUNT]> = d.worlds.iter().map(|w| w.hands()).collect();
        let mut b = u64::MAX;
        let (dag, stats) = solver.action_values_dag(&worlds, rd.leader, &rd.prefix, &mut b);
        let dag = dag.expect("uncapped");
        println!(
            "h{hand} {seat} t{trick_no}: dag done (steps={} tree-equiv={}) — starting the uncapped tree walk",
            stats.steps, stats.tree_steps
        );
        let t = std::time::Instant::now();
        let mut b = u64::MAX;
        let tree = solver
            .action_values(&worlds, rd.leader, &rd.prefix, &mut b)
            .expect("uncapped");
        let tree_charged = u64::MAX - b;
        let identical = tree == dag;
        assert_eq!(
            stats.tree_steps,
            u128::from(tree_charged),
            "h{hand} {seat} t{trick_no}: tree-equivalent accounting exact"
        );
        body.push_str(&format!(
            "h{hand} {seat} t{trick_no} p{} fiber {}: tree walk {} particle-steps in {:?}; dag walk {} particle-steps\n",
            rd.ply,
            worlds.len(),
            tree_charged,
            t.elapsed(),
            stats.steps,
        ));
        for ((a_t, v_t), (a_d, v_d)) in tree.iter().zip(dag.iter()) {
            body.push_str(&format!(
                "  Q^H({a_t})={v_t} (tree) vs Q^H({a_d})={v_d} (dag) -> {}\n",
                if a_t == a_d && v_t == v_d {
                    "IDENTICAL"
                } else {
                    "MISMATCH"
                }
            ));
        }
        body.push_str(&format!(
            "  verdict: {}\n",
            if identical {
                "byte-identical action-value vectors"
            } else {
                "MISMATCH — do not trust either implementation until diagnosed"
            }
        ));
        println!(
            "h{hand} {seat} t{trick_no}: {}",
            if identical { "IDENTICAL" } else { "MISMATCH" }
        );
        write_receipt(&body);
        assert!(identical, "h{hand} {seat} t{trick_no}: values must match");
    }
    print!("{body}");
}

/// Rewrites the receipt with everything cross-validated so far.
fn write_receipt(body: &str) {
    let header = "walt S5c-m3 tree cross-validation receipt — exploratory tier\n\
        scope: the four r2-capped decisions (r3-failed decisions first), values computed independently by BOTH implementations — the unmemoized tree walk (the m2-pinned reference, uncapped: budget u64::MAX, semantics=tree-v0) and the dag-v1 memoized walk (uncapped) — and compared exactly\n\
        purpose: walt-math S5c-m3 amendment 4(i) — the r3 supplement's values lose their SINGLE-IMPLEMENTATION marking for exactly the decisions listed below\n\
        regenerate: cargo test --release -p walt-factory --test h_dag_probe -- --ignored crosscheck_tree_uncapped\n";
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::write(
        root.join("results/h_tree_crossval_2026-08-10.txt"),
        format!("{header}\n{body}"),
    )
    .expect("write receipt");
}
