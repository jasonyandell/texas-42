//! The S5b lesson machinery end-to-end at CI cost: three in-domain seed
//! conflicts (the trick-5 worldwise-dominated chosen actions of the walker
//! corpus, regenerated through the walker library at the CI config), the
//! win form where the origin admits one, and the checker-species seed (the
//! h0 t6 chassis §12.6 failure) — each generalized against the declared
//! tricks-5-6 exhaustive domain and pinned by its summary line, plus the
//! byte-frozen designated receipt.
//!
//! Every pin is a walt-tier regression pin of a computed verdict at a
//! declared config — exploratory tier, never an axiom. Regenerate via
//! `cargo run --release --example lesson_run`; never hand-edit. The
//! pinned basin sizes ARE the S5b falsification measurement at CI scope;
//! the full seed table lives in `results/lesson_basins_2026-08-10.txt`.

use std::sync::OnceLock;

use walt_core::receipt::Receipt;
use walt_core::Seat;
use walt_factory::{
    generalize_lumpability, generalize_regret, generalize_win, lesson_pin_line, load_receipt,
    render_lesson, walk_decision, BasinDomain, DescriptorFamily, DomainSpec, DropOutcome, Grade,
    Lesson, LessonOrigin, WalkerConfig, WideningWitness,
};
use walt_kernel::ReceiptDecision;

fn receipt() -> &'static Receipt {
    static RECEIPT: OnceLock<Receipt> = OnceLock::new();
    RECEIPT.get_or_init(load_receipt)
}

/// The domain is built once for the whole test binary (2 workers — the
/// build is exact and schedule-independent).
fn domain() -> &'static BasinDomain {
    static DOMAIN: OnceLock<BasinDomain> = OnceLock::new();
    DOMAIN.get_or_init(|| BasinDomain::build(receipt(), DomainSpec::tricks_5_to_6(), 2))
}

/// Walks one decision at the CI config and generalizes its dominance
/// conflict (and win form, where one exists).
fn lessons_at(hand: usize, seat: Seat, trick_no: usize) -> (Lesson, Option<Lesson>) {
    let rh = &receipt().hands[hand];
    let decision = ReceiptDecision::at(rh, trick_no, seat).expect("decision reconstructs");
    let config = WalkerConfig {
        threads: 2,
        ..WalkerConfig::ci()
    };
    let record = walk_decision(rh, &decision, &config);
    assert!(
        record.chosen_dominated,
        "the pinned seed is a dominated choice"
    );
    let refutation = generalize_regret(rh, seat, &record, domain()).expect("a refutation lesson");
    let win = generalize_win(rh, seat, &record, domain(), 40_000);
    (refutation, win)
}

/// Structural invariants every lesson must satisfy, checked before any
/// pin: basin totals are consistent, matched decisions exist in the
/// domain, load-bearing cells are exactly the restored trace steps, and
/// the final implicant is a subset of the initial one.
fn check_invariants(lesson: &Lesson) {
    let b = &lesson.basin;
    assert_eq!(b.decisions_matched, b.matched.len());
    assert_eq!(
        b.worlds_matched,
        b.matched.iter().map(|m| m.worlds_matched).sum::<usize>()
    );
    assert_eq!(
        b.strict_worlds,
        b.matched.iter().map(|m| m.strict_worlds).sum::<usize>()
    );
    assert!(b.decisions_matched <= b.decisions_total);
    assert!(b.worlds_matched <= b.worlds_total);
    for cell in &lesson.implicant.cells {
        assert!(
            lesson.initial.cells.contains(cell),
            "the final implicant only keeps initial cells"
        );
    }
    let restored = lesson
        .trace
        .iter()
        .filter(|s| matches!(s.outcome, DropOutcome::LoadBearing(_)))
        .count();
    assert_eq!(restored, lesson.load_bearing().len());
    assert_eq!(
        lesson.trace.len(),
        lesson.initial.cells.len(),
        "every cell is tried exactly once in the kept pass"
    );
    // A load-bearing witness for a pair verdict carries a full value row.
    for step in &lesson.trace {
        if let DropOutcome::LoadBearing(WideningWitness::World { values, .. }) = &step.outcome {
            assert!(!values.is_empty());
        }
    }
}

/// An in-domain origin's own decision must sit in the basin with its whole
/// fiber matched.
fn check_origin_in_basin(lesson: &Lesson) {
    let LessonOrigin::Regret(c) = &lesson.origin else {
        panic!("a regret origin");
    };
    let m = lesson
        .basin
        .matched
        .iter()
        .find(|m| {
            m.hand == c.hand && m.seat == c.seat && m.trick_no == c.trick_no && m.ply == c.ply
        })
        .expect("the in-domain origin is in its own basin");
    assert_eq!(m.worlds_matched, m.worlds_total, "the whole origin fiber");
    assert_eq!(m.worlds_total as u128, c.fiber);
}

#[test]
fn h0_s1_t5_refutation_and_win_lessons_match_the_pins() {
    let (refutation, win) = lessons_at(0, Seat::S1, 5);
    let LessonOrigin::Regret(c) = &refutation.origin else {
        panic!("a regret origin");
    };
    assert!(matches!(c.grade, Grade::WorldwiseDominance { .. }));
    check_invariants(&refutation);
    check_origin_in_basin(&refutation);
    assert_eq!(
        lesson_pin_line(&refutation),
        "regret h0 S1 t5 p0: verdict [refutation: value(decisive) >= value(max-count) at every matching (decision, world)] grade [worldwise (PI; weighting-free)] final [decl=P3 & ply=0] load-bearing [decl=P3, ply=0] basin 1/104 decisions 1680/23790 worlds strict 488"
    );
    let win = win.expect("2-1 is worldwise-optimal at the origin");
    check_invariants(&win);
    check_origin_in_basin(&win);
    assert_eq!(
        lesson_pin_line(&win),
        "regret h0 S1 t5 p0: verdict [win: decisive attains the world optimum at every matching (decision, world)] grade [worldwise (PI; weighting-free)] final [decl=P3 & horizon=3 & ply=0] load-bearing [decl=P3, ply=0, horizon=3] basin 1/104 decisions 1680/23790 worlds strict 0"
    );

    // The designated byte-frozen receipt (regenerate via lesson_run).
    assert_eq!(
        render_lesson(&refutation),
        include_str!("data/lesson_h0_S1_t5.txt"),
        "regenerate via the lesson_run example"
    );
}

#[test]
fn h1_s0_t5_lessons_match_the_pins() {
    let (refutation, win) = lessons_at(1, Seat::S0, 5);
    check_invariants(&refutation);
    check_origin_in_basin(&refutation);
    assert_eq!(
        lesson_pin_line(&refutation),
        "regret h1 S0 t5 p2: verdict [refutation: value(tile(5-2)) >= value(decisive) at every matching (decision, world)] grade [worldwise (PI; weighting-free)] final [ply=2] load-bearing [ply=2] basin 3/104 decisions 231/23790 worlds strict 76"
    );
    let win = win.expect("5-2 is worldwise-optimal at the origin");
    check_invariants(&win);
    assert_eq!(
        lesson_pin_line(&win),
        "regret h1 S0 t5 p2: verdict [win: tile(5-2) attains the world optimum at every matching (decision, world)] grade [worldwise (PI; weighting-free)] final [ply=2] load-bearing [ply=2] basin 5/104 decisions 651/23790 worlds strict 0"
    );
}

#[test]
fn h4_s3_t5_refutation_lesson_matches_the_pin() {
    let (refutation, _) = lessons_at(4, Seat::S3, 5);
    check_invariants(&refutation);
    check_origin_in_basin(&refutation);
    assert_eq!(
        lesson_pin_line(&refutation),
        "regret h4 S3 t5 p2: verdict [refutation: value(max-count) >= value(min-count) at every matching (decision, world)] grade [worldwise (PI; weighting-free)] final [decl=P1 & horizon=3] load-bearing [decl=P1, horizon=3] basin 2/104 decisions 1890/23790 worlds strict 1"
    );
}

#[test]
fn chassis_lumpability_lesson_generalizes_to_all_13_lead_kernels() {
    let lesson =
        generalize_lumpability(&receipt().hands[0], 6, DescriptorFamily::Chassis, domain())
            .expect("the chassis fails §12.6 on h0 t6 (the S4 pin)");
    check_invariants(&lesson);
    assert!(matches!(
        lesson.origin,
        LessonOrigin::Lumpability {
            hand: 0,
            trick_no: 6,
            ..
        }
    ));
    // The checker species generalizes maximally: every cell drops, and the
    // basin is every eligible (ply-0, horizon-2) lead decision — 13 of 13,
    // 647 worlds (the whole trick-6 kernel corpus).
    assert_eq!(
        lesson_pin_line(&lesson),
        "lumpability h0 t6 chassis: verdict [not-lumpable: chassis fails §12.6 at every matching decision] grade [checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)] final [(empty)] load-bearing [none] basin 13/104 decisions 647/23790 worlds strict 0"
    );
}
