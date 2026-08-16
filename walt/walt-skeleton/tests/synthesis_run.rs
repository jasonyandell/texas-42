//! The S4 synthesis run: registry-subset soundness search (§12.9 at S4
//! scope) and the exhaustive §12.6 lumpability grading, over the encoded
//! trick-6 kernel corpus (all 13 receipt hands), plus one trick-5
//! soundness search on the §14.5 kernel.
//!
//! Scope, stated loudly (PLAN.md session log has the same words):
//!
//! - The candidate language is the S4 atom registry (a team fact and a
//!   holder fact per pool tile, plus the valued tile's beater counts); the
//!   soundness search is exhaustive over subsets of size <= 4 at trick 6
//!   and <= 3 at trick 5. Nothing outside that space is claimed.
//! - Lumpability is checked EXHAUSTIVELY (every fiber world, every
//!   reachable viewer-decision node, every legal action, every
//!   positive-mass field segment) on the trick-6 kernels only; trick-5
//!   lumpability is out of S4 budget and simply not run -- not sampled.
//! - Targets are the existing exact machinery's labelings: `q_points`
//!   root-value classes and optimal-root-action classes (walt-strat scalar
//!   PI, the exp5 statistics) and the parametric root-Q envelope classes
//!   (walt-strat symbolic PI, the §14.2 statistic), valued tile = highest
//!   count-point unseen tile (the exp5 convention).
//!
//! Every pinned line below is a walt-tier regression pin of a computed
//! verdict -- exploratory tier, never an axiom (TRUST-01). The run is
//! deterministic: no sampling anywhere, so no seeds to record.

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::Domino;
use walt_kernel::Kernel;
use walt_strat::{pi_root_values, Direction, ScalarCensus, ScalarPi, ScalarValuation};

use walt_skeleton::{
    check_lumpability, class_ids, registry, sound_search, Atom, AtomDescriptor, KernelTree,
    LumpabilityFailure, LumpabilityReport, SoundSearch,
};

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn kernel_at(r: &Receipt, hand: usize, trick: usize) -> Kernel {
    Kernel::from_receipt_trick(&r.hands[hand], trick).expect("a valid kernel")
}

/// exp5's valued-tile convention: highest count-point unseen tile, ties by
/// pip sum then tile order.
fn highest_count_unseen(kernel: &Kernel) -> Domino {
    kernel
        .pool()
        .iter()
        .max_by_key(|d| (d.count(), d.pip_sum(), d.index()))
        .expect("a nonempty pool")
}

/// `q_points` root-value vectors in `Kernel::worlds` order.
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

/// Optimal-root-action masks from the `q_points` vectors.
fn action_labels(values: &[Vec<i64>]) -> Vec<u32> {
    values
        .iter()
        .map(|v| ScalarCensus::argmax_mask(v))
        .collect()
}

/// Parametric root-Q envelope vectors (§14.2 statistic) under the valued
/// tile's direction, in `Kernel::worlds` order.
fn parametric_labels(kernel: &Kernel, valued: Domino) -> Vec<Vec<walt_geom::Envelope>> {
    let dir = Direction::trick_diff_plus_tile(valued);
    kernel
        .worlds()
        .map(|w| {
            pi_root_values(
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

fn sound_line<A: std::fmt::Display>(
    hand: usize,
    fiber: usize,
    tag: &str,
    s: &SoundSearch<A>,
) -> String {
    match &s.minimal {
        Some(m) => {
            let first = m
                .solutions
                .first()
                .expect("nonempty at minimal size")
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join("+");
            let first = if first.is_empty() { "{}" } else { &first };
            format!(
                "h{hand} fiber={fiber} {tag}: min-size={} solutions={} first={first} cells={}",
                m.size,
                m.solutions.len(),
                m.cells[0],
            )
        }
        None => format!(
            "h{hand} fiber={fiber} {tag}: UNSOUND at every size <= {} (vocabulary ceiling)",
            s.max_size
        ),
    }
}

fn lump_line(
    hand: usize,
    tag: &str,
    r: &LumpabilityReport<walt_skeleton::CompositeState>,
) -> String {
    let verdict = match &r.failure {
        None => {
            if r.merged_nodes() > 0 {
                "LUMPABLE nontrivial"
            } else {
                "lumpable trivial"
            }
        }
        Some(LumpabilityFailure::LegalSets { .. }) => "fail:legal-sets",
        Some(LumpabilityFailure::Kernel { .. }) => "fail:kernel",
    };
    format!(
        "h{hand} nodes={} {tag}: {verdict} classes={} merged={} largest={}",
        r.nodes,
        r.classes,
        r.merged_nodes(),
        r.largest_class
    )
}

/// The pinned soundness table: three targets per trick-6 kernel.
const SOUND_PINS: &[&str] = &[
    "h0 fiber=90 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
    "h0 fiber=90 action: UNSOUND at every size <= 4 (vocabulary ceiling)",
    "h0 fiber=90 parametric: UNSOUND at every size <= 4 (vocabulary ceiling)",
    "h1 fiber=90 q_points: min-size=4 solutions=2 first=team(5-3)+holder(1-0)+holder(4-0)+holder(5-4) cells=42",
    "h1 fiber=90 action: min-size=4 solutions=2 first=team(5-3)+holder(1-0)+holder(4-0)+holder(5-4) cells=42",
    "h1 fiber=90 parametric: min-size=4 solutions=2 first=team(5-3)+holder(1-0)+holder(4-0)+holder(5-4) cells=42",
    "h2 fiber=36 q_points: min-size=2 solutions=1 first=holder(2-2)+holder(4-1) cells=9",
    "h2 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
    "h2 fiber=36 parametric: min-size=2 solutions=1 first=holder(2-2)+holder(4-1) cells=9",
    "h3 fiber=36 q_points: min-size=0 solutions=1 first={} cells=1",
    "h3 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
    "h3 fiber=36 parametric: min-size=0 solutions=1 first={} cells=1",
    "h4 fiber=90 q_points: min-size=4 solutions=5 first=team(2-1)+holder(2-0)+holder(5-5)+beaters(5-5) cells=47",
    "h4 fiber=90 action: min-size=4 solutions=10 first=team(2-1)+team(5-5)+holder(2-0)+beaters(5-5) cells=36",
    "h4 fiber=90 parametric: min-size=4 solutions=5 first=team(2-1)+holder(2-0)+holder(5-5)+beaters(5-5) cells=47",
    "h5 fiber=27 q_points: min-size=4 solutions=12 first=team(4-4)+team(5-1)+team(5-5)+holder(1-1) cells=16",
    "h5 fiber=27 action: min-size=3 solutions=4 first=team(4-4)+team(5-1)+holder(1-1) cells=11",
    "h5 fiber=27 parametric: min-size=4 solutions=12 first=team(4-4)+team(5-1)+team(5-5)+holder(1-1) cells=16",
    "h6 fiber=90 q_points: min-size=0 solutions=1 first={} cells=1",
    "h6 fiber=90 action: min-size=0 solutions=1 first={} cells=1",
    "h6 fiber=90 parametric: min-size=0 solutions=1 first={} cells=1",
    "h7 fiber=90 q_points: min-size=1 solutions=2 first=team(1-1) cells=2",
    "h7 fiber=90 action: min-size=0 solutions=1 first={} cells=1",
    "h7 fiber=90 parametric: min-size=3 solutions=1 first=holder(1-1)+holder(3-1)+holder(3-3) cells=24",
    "h8 fiber=7 q_points: min-size=1 solutions=2 first=team(5-5) cells=2",
    "h8 fiber=7 action: min-size=1 solutions=2 first=team(5-5) cells=2",
    "h8 fiber=7 parametric: min-size=1 solutions=2 first=team(5-5) cells=2",
    "h9 fiber=30 q_points: min-size=4 solutions=16 first=team(1-1)+holder(3-0)+holder(3-2)+holder(5-2) cells=27",
    "h9 fiber=30 action: min-size=4 solutions=16 first=team(1-1)+holder(3-0)+holder(3-2)+holder(5-2) cells=27",
    "h9 fiber=30 parametric: min-size=4 solutions=16 first=team(1-1)+holder(3-0)+holder(3-2)+holder(5-2) cells=27",
    "h10 fiber=19 q_points: min-size=3 solutions=4 first=team(6-3)+holder(2-0)+holder(3-2) cells=13",
    "h10 fiber=19 action: min-size=3 solutions=4 first=team(6-3)+holder(2-0)+holder(3-2) cells=13",
    "h10 fiber=19 parametric: min-size=3 solutions=4 first=team(6-3)+holder(2-0)+holder(3-2) cells=13",
    "h11 fiber=36 q_points: UNSOUND at every size <= 4 (vocabulary ceiling)",
    "h11 fiber=36 action: min-size=4 solutions=4 first=team(3-1)+team(6-6)+holder(5-5)+holder(6-3) cells=24",
    "h11 fiber=36 parametric: UNSOUND at every size <= 4 (vocabulary ceiling)",
    "h12 fiber=6 q_points: min-size=0 solutions=1 first={} cells=1",
    "h12 fiber=6 action: min-size=0 solutions=1 first={} cells=1",
    "h12 fiber=6 parametric: min-size=0 solutions=1 first={} cells=1",
];

/// The pinned lumpability table: the candidate list per trick-6 kernel
/// (the min-sound candidate exists only where the q_points search found
/// a sound subset, so h0 and h11 carry six rows, the rest seven).
const LUMP_PINS: &[&str] = &[
    "h0 nodes=738 chassis: fail:kernel classes=110 merged=628 largest=360",
    "h0 nodes=738 chassis+team(v): fail:kernel classes=138 merged=600 largest=144",
    "h0 nodes=738 chassis+team-all: fail:kernel classes=261 merged=477 largest=12",
    "h0 nodes=738 chassis+holder-all: LUMPABLE nontrivial classes=366 merged=372 largest=6",
    "h0 nodes=738 chassis+beaters(v): fail:kernel classes=157 merged=581 largest=72",
    "h0 nodes=738 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=366 merged=372 largest=6",
    "h1 nodes=666 chassis: fail:kernel classes=60 merged=606 largest=288",
    "h1 nodes=666 chassis+team(v): fail:kernel classes=64 merged=602 largest=120",
    "h1 nodes=666 chassis+team-all: fail:kernel classes=189 merged=477 largest=12",
    "h1 nodes=666 chassis+holder-all: LUMPABLE nontrivial classes=330 merged=336 largest=6",
    "h1 nodes=666 chassis+beaters(v): fail:kernel classes=60 merged=606 largest=288",
    "h1 nodes=666 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=330 merged=336 largest=6",
    "h1 nodes=666 chassis+min-sound(q_points): fail:kernel classes=220 merged=446 largest=16",
    "h2 nodes=468 chassis: fail:kernel classes=49 merged=419 largest=288",
    "h2 nodes=468 chassis+team(v): fail:kernel classes=64 merged=404 largest=144",
    "h2 nodes=468 chassis+team-all: fail:kernel classes=142 merged=326 largest=12",
    "h2 nodes=468 chassis+holder-all: LUMPABLE nontrivial classes=212 merged=256 largest=6",
    "h2 nodes=468 chassis+beaters(v): fail:kernel classes=65 merged=403 largest=144",
    "h2 nodes=468 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=212 merged=256 largest=6",
    "h2 nodes=468 chassis+min-sound(q_points): fail:kernel classes=81 merged=387 largest=56",
    "h3 nodes=432 chassis: fail:kernel classes=3 merged=429 largest=288",
    "h3 nodes=432 chassis+team(v): fail:kernel classes=8 merged=424 largest=144",
    "h3 nodes=432 chassis+team-all: fail:kernel classes=113 merged=319 largest=8",
    "h3 nodes=432 chassis+holder-all: LUMPABLE nontrivial classes=170 merged=262 largest=6",
    "h3 nodes=432 chassis+beaters(v): fail:kernel classes=10 merged=422 largest=144",
    "h3 nodes=432 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=170 merged=262 largest=6",
    "h3 nodes=432 chassis+min-sound(q_points): fail:kernel classes=3 merged=429 largest=288",
    "h4 nodes=738 chassis: fail:kernel classes=68 merged=670 largest=288",
    "h4 nodes=738 chassis+team(v): fail:kernel classes=91 merged=647 largest=120",
    "h4 nodes=738 chassis+team-all: fail:kernel classes=198 merged=540 largest=12",
    "h4 nodes=738 chassis+holder-all: LUMPABLE nontrivial classes=330 merged=408 largest=6",
    "h4 nodes=738 chassis+beaters(v): fail:kernel classes=90 merged=648 largest=144",
    "h4 nodes=738 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=330 merged=408 largest=6",
    "h4 nodes=738 chassis+min-sound(q_points): fail:kernel classes=222 merged=516 largest=24",
    "h5 nodes=219 chassis: fail:kernel classes=39 merged=180 largest=36",
    "h5 nodes=219 chassis+team(v): fail:kernel classes=43 merged=176 largest=24",
    "h5 nodes=219 chassis+team-all: fail:kernel classes=146 merged=73 largest=3",
    "h5 nodes=219 chassis+holder-all: LUMPABLE nontrivial classes=161 merged=58 largest=2",
    "h5 nodes=219 chassis+beaters(v): fail:kernel classes=39 merged=180 largest=36",
    "h5 nodes=219 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=161 merged=58 largest=2",
    "h5 nodes=219 chassis+min-sound(q_points): fail:kernel classes=113 merged=106 largest=4",
    "h6 nodes=594 chassis: fail:kernel classes=3 merged=591 largest=252",
    "h6 nodes=594 chassis+team(v): fail:kernel classes=8 merged=586 largest=180",
    "h6 nodes=594 chassis+team-all: fail:kernel classes=75 merged=519 largest=12",
    "h6 nodes=594 chassis+holder-all: LUMPABLE nontrivial classes=210 merged=384 largest=6",
    "h6 nodes=594 chassis+beaters(v): fail:kernel classes=3 merged=591 largest=252",
    "h6 nodes=594 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=210 merged=384 largest=6",
    "h6 nodes=594 chassis+min-sound(q_points): fail:kernel classes=3 merged=591 largest=252",
    "h7 nodes=1098 chassis: fail:kernel classes=76 merged=1022 largest=720",
    "h7 nodes=1098 chassis+team(v): fail:kernel classes=101 merged=997 largest=360",
    "h7 nodes=1098 chassis+team-all: fail:kernel classes=219 merged=879 largest=12",
    "h7 nodes=1098 chassis+holder-all: LUMPABLE nontrivial classes=354 merged=744 largest=6",
    "h7 nodes=1098 chassis+beaters(v): fail:kernel classes=76 merged=1022 largest=720",
    "h7 nodes=1098 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=354 merged=744 largest=6",
    "h7 nodes=1098 chassis+min-sound(q_points): fail:kernel classes=79 merged=1019 largest=360",
    "h8 nodes=63 chassis: fail:kernel classes=21 merged=42 largest=7",
    "h8 nodes=63 chassis+team(v): fail:kernel classes=29 merged=34 largest=7",
    "h8 nodes=63 chassis+team-all: fail:kernel classes=47 merged=16 largest=3",
    "h8 nodes=63 chassis+holder-all: LUMPABLE nontrivial classes=51 merged=12 largest=2",
    "h8 nodes=63 chassis+beaters(v): fail:kernel classes=27 merged=36 largest=4",
    "h8 nodes=63 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=51 merged=12 largest=2",
    "h8 nodes=63 chassis+min-sound(q_points): fail:kernel classes=22 merged=41 largest=6",
    "h9 nodes=354 chassis: fail:kernel classes=107 merged=247 largest=120",
    "h9 nodes=354 chassis+team(v): fail:kernel classes=112 merged=242 largest=72",
    "h9 nodes=354 chassis+team-all: fail:kernel classes=144 merged=210 largest=6",
    "h9 nodes=354 chassis+holder-all: LUMPABLE nontrivial classes=164 merged=190 largest=6",
    "h9 nodes=354 chassis+beaters(v): fail:kernel classes=107 merged=247 largest=120",
    "h9 nodes=354 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=164 merged=190 largest=6",
    "h9 nodes=354 chassis+min-sound(q_points): fail:kernel classes=161 merged=193 largest=6",
    "h10 nodes=151 chassis: fail:kernel classes=30 merged=121 largest=20",
    "h10 nodes=151 chassis+team(v): fail:kernel classes=37 merged=114 largest=20",
    "h10 nodes=151 chassis+team-all: fail:kernel classes=84 merged=67 largest=6",
    "h10 nodes=151 chassis+holder-all: LUMPABLE nontrivial classes=97 merged=54 largest=4",
    "h10 nodes=151 chassis+beaters(v): fail:kernel classes=34 merged=117 largest=20",
    "h10 nodes=151 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=97 merged=54 largest=4",
    "h10 nodes=151 chassis+min-sound(q_points): fail:kernel classes=71 merged=80 largest=8",
    "h11 nodes=288 chassis: fail:kernel classes=50 merged=238 largest=144",
    "h11 nodes=288 chassis+team(v): fail:kernel classes=63 merged=225 largest=60",
    "h11 nodes=288 chassis+team-all: fail:kernel classes=123 merged=165 largest=8",
    "h11 nodes=288 chassis+holder-all: LUMPABLE nontrivial classes=162 merged=126 largest=4",
    "h11 nodes=288 chassis+beaters(v): fail:kernel classes=50 merged=238 largest=144",
    "h11 nodes=288 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=162 merged=126 largest=4",
    "h12 nodes=78 chassis: fail:kernel classes=10 merged=68 largest=48",
    "h12 nodes=78 chassis+team(v): fail:kernel classes=15 merged=63 largest=24",
    "h12 nodes=78 chassis+team-all: fail:kernel classes=33 merged=45 largest=6",
    "h12 nodes=78 chassis+holder-all: LUMPABLE nontrivial classes=50 merged=28 largest=2",
    "h12 nodes=78 chassis+beaters(v): fail:kernel classes=17 merged=61 largest=24",
    "h12 nodes=78 chassis+holder-all+beaters(v): LUMPABLE nontrivial classes=50 merged=28 largest=2",
    "h12 nodes=78 chassis+min-sound(q_points): fail:kernel classes=10 merged=68 largest=48",
];

/// The full S4 run at trick 6, both axes, all 13 kernels.
#[test]
fn synthesis_run_over_the_trick6_corpus() {
    let r = receipt();
    let mut sound_lines = Vec::new();
    let mut lump_lines = Vec::new();
    for hand in 0..13 {
        let kernel = kernel_at(&r, hand, 6);
        let fiber = usize::try_from(kernel.count()).expect("small fibers");
        let valued = highest_count_unseen(&kernel);
        let reg = registry(&kernel, valued);
        assert_eq!(reg.len(), 2 * kernel.pool().len() + 1, "registry size");

        // Exact targets, densified.
        let qp = q_points_labels(&kernel);
        let (qp_ids, _) = class_ids(&qp);
        let act = action_labels(&qp);
        let (act_ids, _) = class_ids(&act);
        let par = parametric_labels(&kernel, valued);
        let (par_ids, _) = class_ids(&par);

        // Static axis: minimum-size purity search, ceiling 4.
        let qp_search = sound_search(&kernel, &reg, 4, &qp_ids);
        let act_search = sound_search(&kernel, &reg, 4, &act_ids);
        let par_search = sound_search(&kernel, &reg, 4, &par_ids);
        sound_lines.push(sound_line(hand, fiber, "q_points", &qp_search));
        sound_lines.push(sound_line(hand, fiber, "action", &act_search));
        sound_lines.push(sound_line(hand, fiber, "parametric", &par_search));

        // Dynamic axis: exhaustive §12.6 grading of the candidate list.
        let tree = KernelTree::build(&kernel, ScalarValuation::trick_plus_count());
        let teams: Vec<Atom> = kernel.pool().iter().map(Atom::TeamOf).collect();
        let holders: Vec<Atom> = kernel.pool().iter().map(Atom::HolderOf).collect();
        let mut candidates: Vec<(String, Vec<Atom>)> = vec![
            ("chassis".into(), vec![]),
            ("chassis+team(v)".into(), vec![Atom::TeamOf(valued)]),
            ("chassis+team-all".into(), teams),
            ("chassis+holder-all".into(), holders.clone()),
            (
                "chassis+beaters(v)".into(),
                vec![Atom::BeaterCounts(valued)],
            ),
            ("chassis+holder-all+beaters(v)".into(), {
                let mut a = holders;
                a.push(Atom::BeaterCounts(valued));
                a
            }),
        ];
        if let Some(m) = &qp_search.minimal {
            candidates.push((
                "chassis+min-sound(q_points)".into(),
                m.solutions.first().expect("nonempty").clone(),
            ));
        }
        for (tag, atoms) in candidates {
            let skeleton = AtomDescriptor::new(&kernel, true, atoms);
            let report = check_lumpability(&kernel, &tree, &skeleton);
            lump_lines.push(lump_line(hand, &tag, &report));
        }
    }
    let pins: Vec<String> = SOUND_PINS.iter().map(|s| s.to_string()).collect();
    assert_eq!(sound_lines, pins, "soundness table drifted");
    let pins: Vec<String> = LUMP_PINS.iter().map(|s| s.to_string()).collect();
    assert_eq!(lump_lines, pins, "lumpability table drifted");
}

/// The pinned exp3A-registry soundness table (S4.5): the ported
/// 22-observable control vocabulary, searched at the probe's own ceiling
/// (subsets <= 4), against the same three targets. The vocabulary
/// parameters on the twelve non-design kernels come from walt's
/// generalization rule (`Exp3aContext`: decisive tile = the viewer tile
/// whose led context touches the most pool tiles) -- walt-tier, NOT probe
/// numbers, except the h0 rows, which the probe record backs.
const EXP3A_PINS: &[&str] = &[
    "h0 fiber=90 q_points: min-size=4 solutions=8 first=holder(2-0)+holder(4-2)+comp+focal-max cells=69",
    "h0 fiber=90 action: min-size=4 solutions=8 first=holder(2-0)+holder(4-2)+comp+focal-max cells=69",
    "h0 fiber=90 parametric: min-size=4 solutions=8 first=holder(2-0)+holder(4-2)+comp+focal-max cells=69",
    "h1 fiber=90 q_points: min-size=3 solutions=14 first=holder(5-3)+holder(5-4)+comp cells=27",
    "h1 fiber=90 action: min-size=3 solutions=18 first=holder(4-0)+holder(5-3)+comp cells=39",
    "h1 fiber=90 parametric: min-size=3 solutions=14 first=holder(5-3)+holder(5-4)+comp cells=27",
    "h2 fiber=36 q_points: min-size=2 solutions=8 first=holder(2-2)+holder(4-1) cells=9",
    "h2 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
    "h2 fiber=36 parametric: min-size=2 solutions=8 first=holder(2-2)+holder(4-1) cells=9",
    "h3 fiber=36 q_points: min-size=0 solutions=1 first={} cells=1",
    "h3 fiber=36 action: min-size=0 solutions=1 first={} cells=1",
    "h3 fiber=36 parametric: min-size=0 solutions=1 first={} cells=1",
    "h4 fiber=90 q_points: min-size=4 solutions=2 first=holder(2-0)+holder(2-1)+holder(5-1)+holder(5-5) cells=54",
    "h4 fiber=90 action: min-size=4 solutions=3 first=holder(2-0)+holder(2-1)+holder(5-1)+holder(5-5) cells=54",
    "h4 fiber=90 parametric: min-size=4 solutions=2 first=holder(2-0)+holder(2-1)+holder(5-1)+holder(5-5) cells=54",
    "h5 fiber=27 q_points: min-size=3 solutions=4 first=holder(1-1)+holder(4-4)+comp cells=23",
    "h5 fiber=27 action: min-size=3 solutions=33 first=holder(1-1)+holder(4-4)+holder(5-1) cells=16",
    "h5 fiber=27 parametric: min-size=3 solutions=4 first=holder(1-1)+holder(4-4)+comp cells=23",
    "h6 fiber=90 q_points: min-size=0 solutions=1 first={} cells=1",
    "h6 fiber=90 action: min-size=0 solutions=1 first={} cells=1",
    "h6 fiber=90 parametric: min-size=0 solutions=1 first={} cells=1",
    "h7 fiber=90 q_points: min-size=1 solutions=6 first=holder(1-1) cells=3",
    "h7 fiber=90 action: min-size=0 solutions=1 first={} cells=1",
    "h7 fiber=90 parametric: min-size=3 solutions=13 first=holder(1-1)+holder(3-1)+holder(3-3) cells=24",
    "h8 fiber=7 q_points: min-size=1 solutions=7 first=holder(5-5) cells=2",
    "h8 fiber=7 action: min-size=1 solutions=7 first=holder(5-5) cells=2",
    "h8 fiber=7 parametric: min-size=1 solutions=7 first=holder(5-5) cells=2",
    "h9 fiber=30 q_points: min-size=4 solutions=73 first=holder(1-1)+holder(2-2)+holder(3-0)+holder(3-2) cells=30",
    "h9 fiber=30 action: min-size=4 solutions=73 first=holder(1-1)+holder(2-2)+holder(3-0)+holder(3-2) cells=30",
    "h9 fiber=30 parametric: min-size=4 solutions=73 first=holder(1-1)+holder(2-2)+holder(3-0)+holder(3-2) cells=30",
    "h10 fiber=19 q_points: min-size=3 solutions=8 first=holder(2-0)+holder(3-2)+holder(6-3) cells=13",
    "h10 fiber=19 action: min-size=3 solutions=8 first=holder(2-0)+holder(3-2)+holder(6-3) cells=13",
    "h10 fiber=19 parametric: min-size=3 solutions=8 first=holder(2-0)+holder(3-2)+holder(6-3) cells=13",
    "h11 fiber=36 q_points: min-size=4 solutions=28 first=holder(6-0)+holder(6-2)+holder(6-3)+comp cells=32",
    "h11 fiber=36 action: min-size=4 solutions=156 first=holder(3-1)+holder(5-5)+holder(6-3)+holder(6-6) cells=30",
    "h11 fiber=36 parametric: min-size=4 solutions=28 first=holder(6-0)+holder(6-2)+holder(6-3)+comp cells=32",
    "h12 fiber=6 q_points: min-size=0 solutions=1 first={} cells=1",
    "h12 fiber=6 action: min-size=0 solutions=1 first={} cells=1",
    "h12 fiber=6 parametric: min-size=0 solutions=1 first={} cells=1",
];

/// The ported exp3A vocabulary across the corpus: does the control-shaped
/// registry find sound small subsets where S4's holder-shaped registry hit
/// its ceiling? (S4: h0 UNSOUND at <= 4 on all three targets; h11 on two.)
#[test]
fn exp3a_registry_search_over_the_trick6_corpus() {
    let r = receipt();
    let mut lines = Vec::new();
    for hand in 0..13 {
        let kernel = kernel_at(&r, hand, 6);
        let fiber = usize::try_from(kernel.count()).expect("small fibers");
        let valued = highest_count_unseen(&kernel);
        let ctx = walt_skeleton::Exp3aContext::new(&kernel, valued);

        let qp = q_points_labels(&kernel);
        let (qp_ids, _) = class_ids(&qp);
        let act = action_labels(&qp);
        let (act_ids, _) = class_ids(&act);
        let par = parametric_labels(&kernel, valued);
        let (par_ids, _) = class_ids(&par);

        for (tag, ids) in [
            ("q_points", &qp_ids),
            ("action", &act_ids),
            ("parametric", &par_ids),
        ] {
            let search = walt_skeleton::exp3a_sound_search(&kernel, &ctx, 4, ids);
            lines.push(sound_line(hand, fiber, tag, &search));
        }
    }
    let pins: Vec<String> = EXP3A_PINS.iter().map(|s| s.to_string()).collect();
    assert_eq!(lines, pins, "exp3A soundness table drifted");
}

/// The pinned trick-5 soundness lines (§14.5 kernel, fiber 1680).
const TRICK5_PINS: &[&str] = &[
    "h0 fiber=1680 q_points: UNSOUND at every size <= 3 (vocabulary ceiling)",
    "h0 fiber=1680 action: UNSOUND at every size <= 3 (vocabulary ceiling)",
];

/// The static-axis search scales to the §14.5 horizon-3 kernel: registry
/// over the nine-tile pool (19 atoms), ceiling 3, scalar targets only (the
/// parametric labeling is out of S4 budget at this horizon -- not run, not
/// sampled).
#[test]
fn static_soundness_search_at_trick5_h0() {
    let r = receipt();
    let kernel = kernel_at(&r, 0, 5);
    assert_eq!(kernel.count(), 1680);
    let valued = highest_count_unseen(&kernel);
    let reg = registry(&kernel, valued);
    assert_eq!(reg.len(), 19);

    let qp = q_points_labels(&kernel);
    let (qp_ids, _) = class_ids(&qp);
    let act = action_labels(&qp);
    let (act_ids, _) = class_ids(&act);

    let lines = vec![
        sound_line(
            0,
            1680,
            "q_points",
            &sound_search(&kernel, &reg, 3, &qp_ids),
        ),
        sound_line(0, 1680, "action", &sound_search(&kernel, &reg, 3, &act_ids)),
    ];
    let pins: Vec<String> = TRICK5_PINS.iter().map(|s| s.to_string()).collect();
    assert_eq!(lines, pins, "trick-5 soundness table drifted");
}
