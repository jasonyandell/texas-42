//! CI tests for the §12.6A situation census (`walt/CENSUS.md`, rulings in
//! `walt/CENSUS-RULINGS.md`): the canonical form's invariances, its
//! separations, the identity-interface control, one full small-kernel
//! pipeline, and a negative control showing the (ECL) checker can fail.
//! Everything here is exploratory tier and deterministic; the full 13-kernel
//! run lives in `walt-factory`'s `census_run` example, not here.

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_core::{Decl, Domino, DominoSet, Pip, Seat};
use walt_kernel::Kernel;
use walt_skeleton::equivariant::{
    build_carrier, build_r3, canonicalize, check_ecl, check_ecl_r3, class_dag, closure_carrier,
    identity_key, r1_refines_r3, suffix_library, yard_shape, yard_tree, CandidateSpec, Census, Law,
    Situation,
};

/// r1's descriptor: every test below that does not name a candidate is a
/// statement about the finest one.
const FINEST: CandidateSpec = CandidateSpec::FINEST;

fn tiles(names: &[&str]) -> DominoSet {
    names
        .iter()
        .map(|n| n.parse::<Domino>().expect("a domino"))
        .collect()
}

fn pip(v: u8) -> Decl {
    Decl::PipTrump(Pip::new(v).expect("a pip"))
}

/// A trick-seven situation: four seats holding one tile each, focal leads.
fn lead_situation(decl: Decl, focal: Seat, hands: [&[&str]; 4]) -> Situation {
    Situation {
        decl,
        focal,
        leader: focal,
        hands: [
            tiles(hands[0]),
            tiles(hands[1]),
            tiles(hands[2]),
            tiles(hands[3]),
        ],
        table: Vec::new(),
    }
}

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn kernel_at(r: &Receipt, hand: usize) -> Kernel {
    Kernel::from_receipt_trick(&r.hands[hand], 6).expect("a valid trick-six kernel")
}

#[test]
fn the_canonical_form_is_invariant_under_a_seat_rotation() {
    let a = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-5"]]);
    let b = lead_situation(pip(3), Seat::S1, [&["6-5"], &["3-0"], &["3-1"], &["3-2"]]);
    assert_eq!(canonicalize(&a, FINEST).key, canonicalize(&b, FINEST).key);
    assert_ne!(identity_key(&a, FINEST), identity_key(&b, FINEST));
}

#[test]
fn the_canonical_form_is_invariant_under_a_trump_and_tile_relabeling() {
    // Same live structure through different denotations: trump moves from the
    // threes to the fours, so every matched tile and the called context are
    // relabeled, while holders, follow membership, the led-context map and
    // every trick-key comparison are preserved.
    let a = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-5"]]);
    let b = lead_situation(pip(4), Seat::S0, [&["4-0"], &["4-1"], &["4-2"], &["6-5"]]);
    assert_eq!(canonicalize(&a, FINEST).key, canonicalize(&b, FINEST).key);
    assert_ne!(
        identity_key(&a, FINEST),
        identity_key(&b, FINEST),
        "the identity interface never sees the relabeling"
    );
}

#[test]
fn a_situation_with_different_dynamics_lands_in_a_different_class() {
    let a = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-5"]]);
    // 6-6 is a double, tops its natural context, and changes both the double
    // flag and the trick-key comparisons: a different structure.
    let b = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-6"]]);
    assert_ne!(canonicalize(&a, FINEST).key, canonicalize(&b, FINEST).key);
    // Different holders are different structure too: the rotation is forced by
    // actor alignment, so it cannot absorb a permutation of the hands.
    let c = lead_situation(pip(3), Seat::S0, [&["3-2"], &["3-1"], &["3-0"], &["6-5"]]);
    assert_ne!(canonicalize(&a, FINEST).key, canonicalize(&c, FINEST).key);
}

#[test]
fn the_table_play_order_is_part_of_the_structure() {
    // Same two tiles down in the same trick, opposite order: the current
    // winner sits at a different table position, so the situations differ.
    let base = |table: &[&str]| Situation {
        decl: pip(3),
        focal: Seat::S0,
        leader: Seat::S0,
        hands: [
            DominoSet::EMPTY,
            DominoSet::EMPTY,
            tiles(&["2-1"]),
            tiles(&["5-0"]),
        ],
        table: table
            .iter()
            .map(|n| n.parse::<Domino>().expect("a domino"))
            .collect(),
    };
    let a = base(&["6-5", "6-4"]);
    let b = base(&["6-4", "6-5"]);
    assert_eq!(a.current_winner(), Some(Seat::S0));
    assert_eq!(b.current_winner(), Some(Seat::S1));
    assert_ne!(canonicalize(&a, FINEST).key, canonicalize(&b, FINEST).key);
}

#[test]
fn canonicalization_is_deterministic_on_a_repeated_call() {
    let a = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-5"]]);
    let first = canonicalize(&a, FINEST);
    let second = canonicalize(&a, FINEST);
    assert_eq!(first.key, second.key);
    assert_eq!(first.tile_id, second.tile_id);
}

#[test]
fn every_root_reaches_hand_end_in_exactly_eight_primitive_steps() {
    // The stepping model against walt-core's rules: two tricks of four plays,
    // every hand empty at the boundary, terminal exactly at the eighth play.
    let r = receipt();
    let kernel = kernel_at(&r, 12);
    for world in kernel.worlds() {
        let mut frontier = vec![(Situation::root(&kernel, world.hands()), 0usize)];
        while let Some((sit, depth)) = frontier.pop() {
            assert!(depth < 8, "a hand is eight primitive steps long");
            let legal = sit.legal();
            assert!(!legal.is_empty(), "a seat holding tiles has a legal play");
            for tile in legal.iter() {
                let (increment, next) = sit.step(tile);
                assert!(increment <= 1, "the count-free increment is 0 or one e*");
                match next {
                    Some(next) => frontier.push((next, depth + 1)),
                    None => assert_eq!(depth, 7, "hand end is the eighth play"),
                }
            }
        }
    }
}

/// The whole situation turned by `by` seats — a genuine symmetry of the
/// oriented game (rotations only; reflection is forbidden, A4).
fn rotate(sit: &Situation, by: usize) -> Situation {
    let mut hands = [DominoSet::EMPTY; Seat::COUNT];
    for s in Seat::ALL {
        hands[s.plus(by).index()] = sit.hands[s.index()];
    }
    Situation {
        decl: sit.decl,
        focal: sit.focal.plus(by),
        leader: sit.leader.plus(by),
        hands,
        table: sit.table.clone(),
    }
}

#[test]
fn every_real_situation_keeps_its_class_under_a_seat_rotation() {
    // The invariance swept over real carrier situations of every structure
    // size, not just hand-built ones: if the canonical form leaked an absolute
    // seat anywhere, a turned situation would land in a different class.
    let r = receipt();
    let census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    assert_eq!(census.carrier.len(), 254);
    for sit in &census.carrier.states {
        let key = canonicalize(sit, FINEST).key;
        for by in 1..Seat::COUNT {
            assert_eq!(
                key,
                canonicalize(&rotate(sit, by), FINEST).key,
                "{}",
                sit.render()
            );
        }
    }
}

#[test]
fn the_h12_kernel_runs_the_whole_pipeline_and_its_ecl_verdict_is_pinned() {
    let r = receipt();
    let kernel = kernel_at(&r, 12);
    assert_eq!(kernel.count(), 6, "the pinned h12 fiber size");
    let census = Census::build(build_carrier(&[(12, kernel)]), FINEST);
    // Pinned computed values -- exploratory tier, never axioms (TRUST-01).
    assert_eq!(census.carrier.roots(), 6, "one root per fiber world");
    assert_eq!(census.carrier.len(), 254);
    assert_eq!(census.class_members.len(), 144);
    assert_eq!(census.identity_members.len(), 254);
    let verdict = check_ecl(&census);
    assert_eq!(verdict.verdict(), "PASS");
    assert_eq!(verdict.classes_checked, 66);
    assert_eq!(verdict.cond1_checks, 110);
    assert_eq!(verdict.cond2_checks, 110);
    assert!(verdict.failures.is_empty());
}

#[test]
fn every_declared_candidate_coarsens_the_finest_one_and_still_passes_ecl() {
    // The r2 toggles: each coarser candidate must be a genuine coarsening —
    // situations the finest descriptor identifies stay identified — and each
    // is checked on its own, so a dropped distinction that turns out to be
    // load-bearing shows up as its own (ECL) failure, never as a repair.
    let r = receipt();
    let finest = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    for spec in CandidateSpec::ALL {
        let census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), spec);
        assert_eq!(census.carrier.len(), finest.carrier.len(), "{}", spec.name);
        assert!(
            census.class_members.len() <= finest.class_members.len(),
            "{} must not split what the finest candidate merged",
            spec.name
        );
        for members in &finest.class_members {
            let first = census.class_of[members[0]];
            for m in &members[1..] {
                assert_eq!(
                    first, census.class_of[*m],
                    "{} split a finest-candidate class",
                    spec.name
                );
            }
        }
        assert!(check_ecl(&census).passed(), "{} ECL", spec.name);
    }
}

#[test]
fn dropping_the_beaten_table_tiles_changes_nothing_at_a_lead() {
    // c3 touches the unresolved trick only, so at a lead (no tile down) it is
    // the finest candidate exactly -- which is why no candidate merges roots.
    let a = lead_situation(pip(3), Seat::S0, [&["3-0"], &["3-1"], &["3-2"], &["6-5"]]);
    assert_eq!(
        canonicalize(&a, FINEST).key,
        canonicalize(&a, CandidateSpec::NO_BEATEN_TILES).key
    );
}

#[test]
fn the_identity_control_is_injective_inside_one_kernel() {
    let r = receipt();
    let census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    assert_eq!(
        census.identity_members.len(),
        census.carrier.len(),
        "identity interfaces separate every distinct situation of one kernel"
    );
}

#[test]
fn the_identity_control_does_not_merge_two_pooled_kernels() {
    let r = receipt();
    let census = Census::build(
        build_carrier(&[(0, kernel_at(&r, 0)), (11, kernel_at(&r, 11))]),
        FINEST,
    );
    assert!(
        census.cross_kernel_identity_classes().is_empty(),
        "the §12.6 control merges nothing across receipt hands"
    );
    assert!(
        !census.cross_kernel_classes().is_empty(),
        "the equivariant reading does merge across receipt hands"
    );
}

#[test]
fn every_class_agrees_on_the_actor_type_and_the_legal_count() {
    // An implementation cross-check independent of (ECL): the descriptor
    // determines whether the focal seat is to act and how many legal moves
    // there are, so no class may mix the two node types.
    let r = receipt();
    let census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    for members in &census.class_members {
        let shape = |i: &usize| match &census.laws[*i] {
            Law::Focal(m) => (true, m.len()),
            Law::Field(m) => (false, m.len()),
        };
        let first = shape(&members[0]);
        for m in &members[1..] {
            assert_eq!(first, shape(m), "one class, one actor type and arity");
        }
    }
}

#[test]
fn the_ecl_checker_fails_on_a_deliberately_widened_class() {
    // The negative control: a PASS is only evidence if the checker can fail.
    // Two classes with different step laws are merged by hand -- no descriptor
    // is being proposed, the widened labeling is only a probe of the checker.
    let r = receipt();
    let mut census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    assert!(check_ecl(&census).passed(), "the census itself passes");
    let victim = (0..census.class_members.len())
        .find(|c| {
            let a = census.class_members[*c][0];
            (0..census.class_members.len())
                .any(|d| d != *c && census.laws[census.class_members[d][0]] != census.laws[a])
        })
        .expect("two classes with different laws");
    let donor = (0..census.class_members.len())
        .find(|d| {
            *d != victim
                && census.laws[census.class_members[*d][0]]
                    != census.laws[census.class_members[victim][0]]
        })
        .expect("a donor class");
    let moved = std::mem::take(&mut census.class_members[donor]);
    for i in &moved {
        census.class_of[*i] = victim;
    }
    census.class_members[victim].extend(moved);
    let verdict = check_ecl(&census);
    assert!(
        !verdict.passed(),
        "widening a class past its dynamics must be caught"
    );
    assert!(!verdict.failures[0].representative.is_empty());
    assert!(!verdict.failures[0].detail.is_empty());
}

// ---------------------------------------------------------------------------
// r3 -- the retrograde coarsest quotient (CENSUS-RULINGS.md section r3).
// ---------------------------------------------------------------------------

#[test]
fn the_r3_backward_pass_is_deterministic() {
    // Both freezes at once (Q5.3): the content-addressed encoding and the
    // canonical move order must reproduce bit for bit across runs.
    let r = receipt();
    let first = build_r3(&build_carrier(&[(12, kernel_at(&r, 12))]));
    let second = build_r3(&build_carrier(&[(12, kernel_at(&r, 12))]));
    assert_eq!(first.class_of, second.class_of);
    assert_eq!(first.class_hash, second.class_hash);
    assert_eq!(first.moves, second.moves);
    assert_eq!(first.terminal, second.terminal);
}

#[test]
fn r1_refines_r3_on_h12() {
    // Mandatory Q5.1, as a test: r1 is a lawful (d, Theta) under Q3's typing,
    // so every r1 class must land inside exactly one r3 class. A violation is
    // a bug or a math error -- never something to patch.
    let r = receipt();
    let carrier = build_carrier(&[(12, kernel_at(&r, 12))]);
    let census = Census::build(build_carrier(&[(12, kernel_at(&r, 12))]), FINEST);
    let r3 = build_r3(&carrier);
    let violations = r1_refines_r3(&census, &r3);
    assert!(
        violations.is_empty(),
        "r1 must refine r3, but {} pairs disagree: {:?}",
        violations.len(),
        violations.first()
    );
}

#[test]
fn the_independent_ecl_recheck_of_r3_is_pinned_on_h12() {
    // Mandatory Q5.2: "by construction" is not a receipt. The re-check rebuilds
    // every law from the rules and matches moves by position through the
    // canonical order.
    let r = receipt();
    let carrier = build_carrier(&[(12, kernel_at(&r, 12))]);
    let r3 = build_r3(&carrier);
    // Pinned computed values -- exploratory tier, never axioms (TRUST-01).
    assert_eq!(r3.class_members.len(), 39);
    let verdict = check_ecl_r3(&carrier, &r3);
    assert_eq!(verdict.verdict(), "PASS");
    assert_eq!(verdict.classes_checked, 33);
    assert_eq!(verdict.cond1_checks, 215);
    assert_eq!(verdict.cond2_checks, 215);
    assert!(verdict.failures.is_empty());
}

#[test]
fn r3_merges_two_structurally_different_forced_situations() {
    // The equivariance gain, hand built: at a trick-7 lead every seat holds one
    // tile and the whole hand is forced, so the future cone is just the
    // sequence of (actor offset, classification, increment). These two states
    // admit no full-structure tile bijection -- one leader's tile is a double
    // and the other's is not, which r1 separates -- yet every primitive step
    // emits the same statistics, so r3 merges them.
    let a = lead_situation(pip(3), Seat::S0, [&["6-5"], &["6-4"], &["6-2"], &["6-1"]]);
    let b = lead_situation(pip(5), Seat::S0, [&["4-4"], &["4-3"], &["4-2"], &["4-1"]]);
    assert_ne!(
        canonicalize(&a, FINEST).key,
        canonicalize(&b, FINEST).key,
        "r1 separates them: the double flag differs"
    );
    let carrier = closure_carrier(&[a.clone(), b.clone()]);
    let r3 = build_r3(&carrier);
    let ia = carrier.lookup(&a).expect("seed a is in the carrier");
    let ib = carrier.lookup(&b).expect("seed b is in the carrier");
    assert_eq!(
        r3.class_of[ia], r3.class_of[ib],
        "r3 merges them: same offsets, same classifications, same increment"
    );
    assert!(check_ecl_r3(&carrier, &r3).passed());
}

#[test]
fn the_t5_rung_runs_the_whole_pipeline_on_the_smallest_kernel() {
    // The trick-five climb, on h8 (the smallest trick-five fiber): grades 12
    // down to 0, both mandatory receipts, pinned. Same construction as the
    // trick-six rung -- the r3 ruling binds unchanged at this rung.
    let r = receipt();
    let kernel = Kernel::from_receipt_trick(&r.hands[8], 5).expect("the h8 trick-five kernel");
    assert_eq!(kernel.count(), 92, "the pinned h8 trick-five fiber size");
    let carrier = build_carrier(&[(8, kernel.clone())]);
    // Pinned computed values -- exploratory tier, never axioms (TRUST-01).
    assert_eq!(carrier.roots(), 92);
    assert_eq!(carrier.len(), 25796);
    let r3 = build_r3(&carrier);
    assert_eq!(r3.class_members.len(), 1939);
    let verdict = check_ecl_r3(&carrier, &r3);
    assert_eq!(verdict.verdict(), "PASS");
    assert!(verdict.failures.is_empty());
    // Q5.1 at this rung too: r1's finest candidate must still refine r3.
    let census = Census::build(build_carrier(&[(8, kernel_t5(&r, 8))]), FINEST);
    assert!(r1_refines_r3(&census, &r3).is_empty());
}

#[test]
fn the_live_sub_dag_of_the_smallest_kernel_is_pinned() {
    // The pruning probe: the seat facing one actual kernel searches only the
    // classes reachable from its fiber's root classes. Support pruning only --
    // no belief weight is applied, and belief can only concentrate this
    // further, never widen it.
    let r = receipt();
    let kernels: Vec<(usize, Kernel)> = (0..13).map(|h| (h, kernel_at(&r, h))).collect();
    let carrier = build_carrier(&kernels);
    let r3 = build_r3(&carrier);
    let dag = class_dag(&r3);
    assert_eq!(r3.class_members.len(), 1459, "the pooled trick-six rung");
    // h12, the smallest trick-six fiber.
    let slot = 12;
    let roots: Vec<usize> = (0..carrier.len())
        .filter(|i| carrier.is_root[*i] && (carrier.provenance[*i] & (1u32 << slot)) != 0)
        .collect();
    assert_eq!(roots.len(), 6, "the pinned h12 fiber size");
    let root_classes: Vec<usize> = {
        let set: std::collections::BTreeSet<usize> =
            roots.iter().map(|i| r3.class_of[*i]).collect();
        set.into_iter().collect()
    };
    // Pinned computed values -- exploratory tier, never axioms (TRUST-01).
    assert_eq!(root_classes.len(), 3, "six worlds, three root classes");
    let live = dag.reachable(&root_classes);
    assert_eq!(live.len(), 39);
    assert_eq!(dag.edges(&live), (42, 3));
    assert!(
        live.len() < r3.class_members.len(),
        "pruning to one kernel must not reach the whole rung"
    );
}

#[test]
fn the_yard_routine_reproduces_r3s_partition_at_every_level() {
    // Y2 test 1 (P1, the refactoring): ONE shared grade-free routine -- it
    // takes no grade and no level argument -- recomputes level-j classes from
    // level-(j-1) classes, and its partition must be r3's exactly. A mismatch
    // is a bug or a hidden grade dependence, never a result.
    let r = receipt();
    let carrier = build_carrier(&[(12, kernel_at(&r, 12))]);
    let r3 = build_r3(&carrier);
    let handoff = |sit: &Situation| -> u64 {
        r3.class_of[carrier.lookup(sit).expect("closed under steps")] as u64
    };
    for level in 1..=2 {
        let states: Vec<usize> = (0..carrier.len())
            .filter(|i| walt_skeleton::equivariant::grade(&carrier.states[*i]) == 4 * level)
            .collect();
        assert!(!states.is_empty(), "level {level} is present");
        let mut yard: std::collections::BTreeMap<Vec<u8>, Vec<usize>> =
            std::collections::BTreeMap::new();
        for i in &states {
            yard.entry(yard_tree(&carrier.states[*i], &handoff).encode())
                .or_default()
                .push(*i);
        }
        let mut classes: std::collections::BTreeMap<usize, Vec<usize>> =
            std::collections::BTreeMap::new();
        for i in &states {
            classes.entry(r3.class_of[*i]).or_default().push(*i);
        }
        let mut a: Vec<Vec<usize>> = yard.into_values().collect();
        let mut b: Vec<Vec<usize>> = classes.into_values().collect();
        a.sort();
        b.sort();
        assert_eq!(a, b, "the yard partition is r3's at level {level}");
    }
    // The shape of a level-1 tree: every seat holds one tile, so the tree is a
    // single forced path with the terminal as its only leaf.
    let level_one = (0..carrier.len())
        .find(|i| walt_skeleton::equivariant::grade(&carrier.states[*i]) == 4)
        .expect("a level-1 boundary situation");
    let shape = yard_shape(&yard_tree(&carrier.states[level_one], &handoff))
        .expect("within the declared canonicalization ceiling");
    assert!(!shape.is_empty());
}

/// The class trees of one level of a single-kernel carrier.
fn level_trees(
    carrier: &walt_skeleton::equivariant::Carrier,
    r3: &walt_skeleton::equivariant::R3,
    level: usize,
) -> Vec<walt_skeleton::equivariant::YardNode> {
    let handoff = |sit: &Situation| -> u64 {
        r3.class_of[carrier.lookup(sit).expect("closed under steps")] as u64
    };
    let mut representative: std::collections::BTreeMap<usize, usize> =
        std::collections::BTreeMap::new();
    for i in 0..carrier.len() {
        if walt_skeleton::equivariant::grade(&carrier.states[i]) == 4 * level {
            representative.entry(r3.class_of[i]).or_insert(i);
        }
    }
    representative
        .values()
        .map(|i| yard_tree(&carrier.states[*i], &handoff))
        .collect()
}

#[test]
fn the_suffix_library_is_deterministic_and_its_overlap_is_pinned() {
    // Shape notion v2, on the smallest kernel. Instrument tier: these are
    // library sizes, never class counts, and neither variant satisfies (ECL).
    let r = receipt();
    let carrier = build_carrier(&[(12, kernel_at(&r, 12))]);
    let r3 = build_r3(&carrier);
    let one = level_trees(&carrier, &r3, 1);
    let two = level_trees(&carrier, &r3, 2);

    let first = suffix_library(&one, 3).expect("within the declared ceiling");
    let second = suffix_library(&one, 3).expect("within the declared ceiling");
    for d in 0..3 {
        assert_eq!(first.strict[d], second.strict[d], "strict depth {}", d + 1);
        assert_eq!(first.open[d], second.open[d], "open depth {}", d + 1);
    }

    let upper = suffix_library(&two, 3).expect("within the declared ceiling");
    // Pinned computed values -- exploratory/instrument tier, never axioms.
    assert_eq!(first.strict[0].len(), 11);
    assert_eq!(first.open[0].len(), 11);
    assert_eq!(upper.strict[0].len(), 12);
    assert_eq!(upper.open[0].len(), 12);
    let shared = first.open[0].intersection(&upper.open[0]).count();
    assert_eq!(shared, 6, "the pinned depth-1 v2-open cross-level overlap");
    // The open variant can only merge what strict keeps apart, never the
    // reverse: it deduplicates options at unconstrained nodes and changes
    // nothing else.
    for d in 0..3 {
        assert!(first.open[d].len() <= first.strict[d].len());
        assert!(upper.open[d].len() <= upper.strict[d].len());
    }
}

#[test]
fn the_complete_level_one_alphabet_is_sixty_four() {
    // THE NUMBER, pinned. A level-one tree is a forced chain, so its class is
    // exactly (actor offset from focal, the three followers' classifications,
    // the count-free increment): 4 x 2 x 2 x 2 x 2 = 64 combinations, and the
    // complete enumeration realizes every one of them. This test walks one
    // pip declaration rather than all seven -- the full run is in the example
    // -- because a single declaration already realizes the whole alphabet.
    let handoff = |_: &Situation| -> u64 { walt_skeleton::equivariant::YARD_TERMINAL };
    let tiles = Domino::ALL;
    let decl = pip(0);
    let mut classes: std::collections::BTreeSet<Vec<u8>> = std::collections::BTreeSet::new();
    for a in 0..tiles.len() {
        for b in 0..tiles.len() {
            for c in 0..tiles.len() {
                for d in 0..tiles.len() {
                    if a == b || a == c || a == d || b == c || b == d || c == d {
                        continue;
                    }
                    let hands = [
                        DominoSet::single(tiles[a]),
                        DominoSet::single(tiles[b]),
                        DominoSet::single(tiles[c]),
                        DominoSet::single(tiles[d]),
                    ];
                    for leader in Seat::ALL {
                        for focal in Seat::ALL {
                            let sit = Situation {
                                decl,
                                focal,
                                leader,
                                hands,
                                table: Vec::new(),
                            };
                            classes.insert(yard_tree(&sit, &handoff).encode());
                        }
                    }
                }
            }
        }
    }
    assert_eq!(classes.len(), 64, "the complete level-one alphabet");
}

fn kernel_t5(r: &Receipt, hand: usize) -> Kernel {
    Kernel::from_receipt_trick(&r.hands[hand], 5).expect("a valid trick-five kernel")
}

#[test]
fn r3_separates_forced_situations_that_pay_the_other_partnership() {
    // The control for the merge above: move the winning tile to the seat
    // across the table and the count-free increment differs, so r3 must keep
    // the two apart.
    let a = lead_situation(pip(3), Seat::S0, [&["6-5"], &["6-4"], &["6-2"], &["6-1"]]);
    let c = lead_situation(pip(3), Seat::S0, [&["6-1"], &["6-5"], &["6-4"], &["6-2"]]);
    let carrier = closure_carrier(&[a.clone(), c.clone()]);
    let r3 = build_r3(&carrier);
    let ia = carrier.lookup(&a).expect("seed a");
    let ic = carrier.lookup(&c).expect("seed c");
    assert_ne!(r3.class_of[ia], r3.class_of[ic]);
}
