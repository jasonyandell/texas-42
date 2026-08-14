//! End-to-end validation of the S4 harness on the §14.2 design kernel
//! (receipt hand 0, trick 6): the observation derivation, the chassis
//! invariant, the tree-vs-partition cross-check against S3 machinery, the
//! §12.1 checker's factorization shape, and the marked static passenger.
//!
//! Every number here is exploratory tier: walt-tier computed pins, never
//! axioms (TRUST-01).

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::{Context, Domino, Pip, Team};
use walt_kernel::{Kernel, World};
use walt_strat::{Direction, InfoPartition, ScalarPi, ScalarValuation};

use walt_skeleton::{
    check_lumpability, check_soundness, class_ids, exp3a_registry, exp3a_sound_search, fold_record,
    Atom, AtomDescriptor, ControlSkeleton, Exp3aAtom, Exp3aContext, Exp3aDescriptor, KernelTree,
    LumpabilityFailure, StaticWrap, UpdateKind,
};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn d(s: &str) -> Domino {
    s.parse().expect("a domino literal")
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
        let mut budget: u64 = 1_000_000_000;
        let mut cap_hit = false;
        let partition = InfoPartition::build(&k, a, &mut budget, usize::MAX, &mut cap_hit)
            .expect("non-binding");
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

/// The §14.2 valued tile and the exp3A vocabulary parameters, derived by
/// walt's `Exp3aContext` rule, land exactly on the probe's design-kernel
/// constants: valued 4-1, decisive tile 2-1, decisive context suit 2, boss
/// 2-2, floor 2-0, and a 22-observable registry.
#[test]
fn exp3a_context_instantiates_the_design_kernel_parameters() {
    let k = trick6_kernel();
    let ctx = Exp3aContext::new(&k, d("4-1"));
    assert_eq!(ctx.decisive, d("2-1"));
    assert_eq!(ctx.context, Context::Natural(Pip::new(2).expect("a pip")));
    assert_eq!(
        ctx.context_pool,
        [d("2-0"), d("2-2"), d("4-2"), d("5-2")]
            .into_iter()
            .collect::<walt_core::DominoSet>()
    );
    assert_eq!(ctx.boss, Some(d("2-2")));
    assert_eq!(ctx.floor, Some(d("2-0")));
    assert_eq!(exp3a_registry(&k).len(), 22);
}

/// The parametric (§14.2) 8-class labels, in `Kernel::worlds` order.
fn parametric_labels(kernel: &Kernel) -> Vec<Vec<walt_geom::Envelope>> {
    let dir = Direction::trick_diff_plus_tile(d("4-1"));
    kernel
        .worlds()
        .map(|w| {
            walt_strat::pi_root_values(
                kernel.decl(),
                w.hands(),
                kernel.viewer(),
                kernel.viewer().team(),
                &dir,
            )
            .into_iter()
            .map(|(_, e)| e)
            .collect()
        })
        .collect()
}

/// UNBLOCKED (S4.5; walt/DISCREPANCIES.md "exp3A descriptor pin"): the
/// §14.4 design descriptor D = {comp41, s3max2, team(2:0), team(4:2)},
/// ported from the preserved probe (`walt/probes/exp3a/lambda_probe_v3.py`
/// Part 1), reproduces 90 worlds -> 33 cells -> 8 responses through walt's
/// own §12.1 checker, and stays sound for the 3-class action target with
/// the same 33 cells. The descriptor is marked static -- §14.4 is a static
/// compression result.
#[test]
fn exp3a_design_descriptor_reproduces_90_33_8() {
    let k = trick6_kernel();
    let ctx = Exp3aContext::new(&k, d("4-1"));
    let design = Exp3aDescriptor::new(
        ctx,
        vec![
            Exp3aAtom::Comp,
            Exp3aAtom::FocalMax,
            Exp3aAtom::Team(d("2-0")),
            Exp3aAtom::Team(d("4-2")),
        ],
    );
    assert_eq!(design.kind(), UpdateKind::StaticPassenger);

    let labels = parametric_labels(&k);
    let mut i = 0usize;
    let r8 = check_soundness(&k, &design, |_| {
        let l = labels[i].clone();
        i += 1;
        l
    });
    assert!(r8.is_sound(), "the design descriptor factors the R8 target");
    assert_eq!(
        (r8.worlds, r8.cells, r8.responses),
        (90, 33, 8),
        "90 worlds -> 33 cells -> 8 responses (v0.4 §14.4)"
    );

    let corr: Vec<walt_geom::ArgmaxCorrespondence> = labels
        .iter()
        .map(|es| walt_geom::argmax_correspondence(es))
        .collect();
    let mut i = 0usize;
    let r3 = check_soundness(&k, &design, |_| {
        let l = corr[i].clone();
        i += 1;
        l
    });
    assert!(r3.is_sound());
    assert_eq!((r3.worlds, r3.cells, r3.responses), (90, 33, 3));
}

/// The probe's full Part 1 search record reproduces through walt's own
/// machinery: for BOTH targets (R8 parametric, R3 action correspondence),
/// the minimal sound size is 4 with exactly eight solutions -- the four
/// {comp | comp-rank} x {holder | team} shapes -- at 69/53/53/33 cells.
/// Solution sets and cell counts pinned exactly against the preserved
/// probe output (`v3_output_postfix.txt`).
#[test]
fn exp3a_search_reproduces_the_probe_record() {
    let k = trick6_kernel();
    let ctx = Exp3aContext::new(&k, d("4-1"));
    let labels = parametric_labels(&k);
    let (r8_ids, r8_count) = class_ids(&labels);
    assert_eq!(r8_count, 8);
    let corr: Vec<walt_geom::ArgmaxCorrespondence> = labels
        .iter()
        .map(|es| walt_geom::argmax_correspondence(es))
        .collect();
    let (r3_ids, r3_count) = class_ids(&corr);
    assert_eq!(r3_count, 3);

    let expected: Vec<(Vec<Exp3aAtom>, usize)> = {
        let mut out = Vec::new();
        for comp in [Exp3aAtom::Comp, Exp3aAtom::CompRank] {
            for (pair, cells) in [
                (
                    (Exp3aAtom::Holder(d("2-0")), Exp3aAtom::Holder(d("4-2"))),
                    69,
                ),
                ((Exp3aAtom::Holder(d("2-0")), Exp3aAtom::Team(d("4-2"))), 53),
                ((Exp3aAtom::Holder(d("4-2")), Exp3aAtom::Team(d("2-0"))), 53),
                ((Exp3aAtom::Team(d("2-0")), Exp3aAtom::Team(d("4-2"))), 33),
            ] {
                let mut sol = vec![pair.0, pair.1, comp, Exp3aAtom::FocalMax];
                sol.sort();
                out.push((sol, cells));
            }
        }
        out.sort();
        out
    };

    for (tag, ids) in [("R8", &r8_ids), ("R3", &r3_ids)] {
        let search = exp3a_sound_search(&k, &ctx, 4, ids);
        let minimal = search.minimal.expect("a sound descriptor exists");
        assert_eq!(minimal.size, 4, "{tag}: smallest sound size");
        let mut got: Vec<(Vec<Exp3aAtom>, usize)> = minimal
            .solutions
            .iter()
            .zip(&minimal.cells)
            .map(|(sol, cells)| {
                let mut sol = sol.clone();
                sol.sort();
                (sol, *cells)
            })
            .collect();
        got.sort();
        assert_eq!(got.len(), 8, "{tag}: eight minimal solutions (§14.4)");
        assert_eq!(got, expected, "{tag}: the probe's solution record");
    }
}
