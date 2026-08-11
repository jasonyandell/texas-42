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
    build_carrier, canonicalize, check_ecl, identity_key, CandidateSpec, Census, Law, Situation,
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
