//! End-to-end validation of the S4 harness on the §14.2 design kernel
//! (receipt hand 0, trick 6): the observation derivation, the chassis
//! invariant, the tree-vs-partition cross-check against S3 machinery, the
//! §12.1 checker's factorization shape, and the marked static passenger.
//!
//! Every number here is exploratory tier: walt-tier computed pins, never
//! axioms (TRUST-01).

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::{Domino, Team};
use walt_kernel::{Kernel, World};
use walt_strat::{InfoPartition, ScalarPi, ScalarValuation};

use walt_skeleton::{
    check_lumpability, check_soundness, fold_record, Atom, AtomDescriptor, ControlSkeleton,
    KernelTree, LumpabilityFailure, StaticWrap, UpdateKind,
};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// The §14.2 design kernel: receipt hand 0, start of trick 6.
fn trick6_kernel() -> Kernel {
    let r = receipt();
    Kernel::from_receipt_trick(&r.hands[0], 6).expect("a valid kernel")
}

/// Exact `q_points` root value vectors for the whole fiber, in
/// `Kernel::worlds` order (the exp5 labeling, walt-strat's scalar PI).
fn q_points_labels(kernel: &Kernel) -> Vec<Vec<i64>> {
    let mut pi = ScalarPi::new(
        kernel.decl(),
        kernel.viewer().team(),
        ScalarValuation::trick_plus_count(),
    );
    kernel
        .worlds()
        .map(|w| {
            pi.root_values(w.hands(), kernel.viewer())
                .into_iter()
                .map(|(_, v)| v)
                .collect()
        })
        .collect()
}

/// The tree's chassis discipline: at every reachable viewer-decision node
/// of every fiber world, the chassis fold of the observation record carries
/// exactly the node's legal set (§10.1 validity from seat-honest state).
#[test]
fn chassis_fold_carries_the_legal_set_at_every_node() {
    let k = trick6_kernel();
    let tree = KernelTree::build(&k, ScalarValuation::trick_plus_count());
    let chassis = AtomDescriptor::new(&k, true, vec![]);
    let worlds: Vec<World> = k.worlds().collect();
    for node in &tree.nodes {
        let state = fold_record(&chassis, &k, &worlds[node.world], &node.record);
        let c = state.chassis.expect("the chassis is on");
        assert_eq!(c.legal(k.decl()), node.legal, "at record {:?}", node.record);
    }
}

/// The lumpability carrier agrees with S3's information machinery: for each
/// root action, the tree's future nodes group by record into exactly the
/// canonical perfect-recall information states, pooling exactly as many
/// worlds as `InfoPartition` reports.
#[test]
fn the_tree_refines_the_s3_information_partition_worldwise() {
    let k = trick6_kernel();
    let tree = KernelTree::build(&k, ScalarValuation::trick_plus_count());
    let mut future = 0usize;
    for a in k.viewer_hand().iter() {
        let partition = InfoPartition::build(&k, a);
        let mut by_record: std::collections::BTreeMap<&[Domino], usize> =
            std::collections::BTreeMap::new();
        for node in &tree.nodes {
            if node.record.first() == Some(&a) {
                *by_record.entry(&node.record).or_insert(0) += 1;
            }
        }
        assert_eq!(by_record.len(), partition.len(), "distinct records");
        for (record, worlds) in by_record {
            let id = partition.id(record).expect("the record is a state");
            assert_eq!(partition.pooled_nodes(id), worlds, "pooled worlds");
            future += worlds;
        }
    }
    // Every node is a root (one per world) or a future focal node.
    assert_eq!(tree.nodes.len(), 90 + future);
}

/// §12.1 on the identity end: the full holder map IS the world at the root,
/// so it factors every target with zero compression -- 90 -> 90 -> 8 for
/// the `q_points` target (8 classes, the exp5 h0t6 row).
#[test]
fn full_holder_descriptor_factors_as_the_identity() {
    let k = trick6_kernel();
    let holders: Vec<Atom> = k.pool().iter().map(Atom::HolderOf).collect();
    let descriptor = AtomDescriptor::new(&k, false, holders);
    let labels = q_points_labels(&k);
    let mut i = 0usize;
    let report = check_soundness(&k, &descriptor, |_| {
        let l = labels[i].clone();
        i += 1;
        l
    });
    assert!(report.is_sound());
    assert_eq!(
        (report.worlds, report.cells, report.responses),
        (90, 90, 8),
        "90 worlds -> 90 cells -> 8 responses"
    );
}

/// §12.1 on the failing end: the coarsest descriptor (no atoms at all)
/// merges everything, and the checker emits the §12.9 witness pair.
#[test]
fn trivial_descriptor_fails_soundness_with_a_witness_pair() {
    let k = trick6_kernel();
    let descriptor = AtomDescriptor::new(&k, false, vec![]);
    let labels = q_points_labels(&k);
    let mut i = 0usize;
    let report = check_soundness(&k, &descriptor, |_| {
        let l = labels[i].clone();
        i += 1;
        l
    });
    assert!(!report.is_sound());
    assert_eq!((report.worlds, report.cells, report.responses), (90, 1, 8));
    let ce = report.counterexample.expect("a witness pair");
    assert_ne!(ce.label_a, ce.label_b, "separated by the target");
}

/// The day-one passenger: a §14.4-style static descriptor riding the
/// harness as a degenerate-update transducer, marked static. Its root
/// factorization is checkable end-to-end, and its lumpability verdict is
/// structurally forced: the frozen state merges each world's root with
/// that same world's future nodes, where the legal sets already disagree
/// (condition 1), so a static passenger is never lumpable on any kernel
/// with a future focal decision.
#[test]
fn static_passenger_is_marked_and_fails_lumpability_on_legal_sets() {
    let k = trick6_kernel();
    let teams: Vec<Atom> = k.pool().iter().map(Atom::TeamOf).collect();
    let passenger = StaticWrap(AtomDescriptor::new(&k, false, teams));
    assert_eq!(passenger.kind(), UpdateKind::StaticPassenger);
    assert!(passenger.name().starts_with("static["));

    // End-to-end factorization run (the passenger exists to prove the
    // harness, not to win). One hidden slot is focal here, so a team
    // pattern is that slot's two-tile hand: C(6,2) = 15 cells on this
    // unconstrained fiber; the q_points target does not factor through
    // them.
    let labels = q_points_labels(&k);
    let mut i = 0usize;
    let sound = check_soundness(&k, &passenger, |_| {
        let l = labels[i].clone();
        i += 1;
        l
    });
    assert_eq!((sound.worlds, sound.cells, sound.responses), (90, 15, 8));
    assert!(!sound.is_sound());

    let tree = KernelTree::build(&k, ScalarValuation::trick_plus_count());
    let lump = check_lumpability(&k, &tree, &passenger);
    assert_eq!(lump.kind, UpdateKind::StaticPassenger);
    assert!(!lump.is_lumpable());
    assert!(
        matches!(lump.failure, Some(LumpabilityFailure::LegalSets { .. })),
        "a frozen state merges a root with its own future"
    );
}

/// The focal side of every labeling above is the viewer's team, as in the
/// probe suite; the design kernel's viewer is S1 on team T1.
#[test]
fn the_design_kernel_focal_side_is_the_viewer_team() {
    let k = trick6_kernel();
    assert_eq!(k.viewer().team(), Team::T1);
}

/// BLOCKED (walt/DISCREPANCIES.md, "exp3A descriptor pin"): v0.4 §14.4
/// reports the four-atom static descriptor {comp41, s3max2, team(2:0),
/// team(4:2)} producing 33 purpose-sound cells for the eight-class root-Q
/// target (90 -> 33 -> 8), but the spec defines neither the 22-observable
/// vocabulary nor those atoms' semantics ("comp41" and "s3max2" appear
/// nowhere else; §12.3 gives only shape names), and no exp3A probe source
/// survives -- walt/probes/ holds exp5 only. Reproducing 33 would require
/// inventing semantics, which the ambiguity protocol forbids.
#[test]
#[ignore = "blocked: §14.4 atom semantics undefined and no exp3A probe source survives; see walt/DISCREPANCIES.md 'exp3A descriptor pin'"]
fn exp3a_static_descriptor_pin() {
    panic!("unblock by supplying exp3A atom semantics, then reproduce 90 -> 33 -> 8");
}
