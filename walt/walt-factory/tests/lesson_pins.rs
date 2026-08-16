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
//! the full seed table lives in the latest `results/lesson_basins_*` revision.

use std::sync::OnceLock;

use walt_core::receipt::Receipt;
use walt_core::Seat;
use walt_factory::{
    cell_holds_at, generalize_lumpability, generalize_regret, generalize_win, lesson_applies,
    lesson_pin_line, load_receipt, measure_rent, remeasure_at_h, render_h_report, render_lesson,
    walk_decision, BasinDomain, DescriptorFamily, DomainDecision, DomainSpec, Grade, Lesson,
    LessonOrigin, RentReport, StepOutcome, TraceStep, WalkerConfig, WideningWitness,
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
    check_invariants_on(lesson, domain())
}

/// Same invariants against an explicit domain (for lessons generalized on
/// the m1 subset domain).
fn check_invariants_on(lesson: &Lesson, dom: &BasinDomain) {
    let b = &lesson.basin;
    assert_eq!(b.decisions_matched, b.matched.len());
    assert_eq!(
        b.worlds_matched,
        b.matched.iter().map(|m| m.worlds_matched).sum::<usize>()
    );
    // The dominance primitive (the world-count triple): per-decision
    // triples sum to the basin total, `lt` is 0 in every verified basin
    // (an `lt` world would have been the aborting witness), and a matched
    // decision's triple covers exactly its matched worlds.
    match b.dominance {
        Some(t) => {
            assert_eq!(t.lt, 0);
            assert_eq!(t.worlds(), b.worlds_matched);
            let mut gt = 0;
            let mut eq = 0;
            for m in &b.matched {
                let mt = m.dominance.expect("pair verdicts carry triples");
                assert_eq!(mt.lt, 0);
                assert_eq!(mt.worlds(), m.worlds_matched);
                gt += mt.gt;
                eq += mt.eq;
            }
            assert_eq!((gt, eq), (t.gt, t.eq));
        }
        None => assert!(b.matched.iter().all(|m| m.dominance.is_none())),
    }
    // Carrier-relative rate bases (§11.1): matched <= eligible <= domain.
    assert!(b.decisions_matched <= b.decisions_eligible);
    assert!(b.decisions_eligible <= b.domain_decisions);
    assert!(b.worlds_matched <= b.worlds_eligible);
    assert!(b.worlds_eligible <= b.domain_worlds);
    // The purpose split (§9.6 is purpose-relative): the refutation
    // subbasin is exactly the strict-somewhere matched decisions, the
    // safe-substitution subbasin is every matched decision.
    if let Some((refutation, safe)) = lesson.subbasins() {
        assert_eq!(safe, b.decisions_matched);
        assert_eq!(
            refutation,
            b.matched
                .iter()
                .filter(|m| m.dominance.is_some_and(|t| t.gt > 0))
                .count()
        );
        assert!(refutation <= safe);
    }
    for cell in &lesson.implicant.cells {
        let is_bound = matches!(
            cell,
            walt_factory::Constraint::HorizonGe(_)
                | walt_factory::Constraint::HorizonLe(_)
                | walt_factory::Constraint::NumericGe(..)
                | walt_factory::Constraint::NumericLe(..)
        );
        assert!(
            lesson.initial.cells.contains(cell) || lesson.introduced().contains(cell) || is_bound,
            "a final cell is initial, introduced, or a relaxed bound"
        );
    }
    let restored = lesson
        .trace
        .iter()
        .filter(|s| {
            matches!(
                s,
                TraceStep::Drop {
                    outcome: StepOutcome::Survives(_) | StepOutcome::BoundHeld { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!(restored, lesson.surviving().len());
    let drops = lesson
        .trace
        .iter()
        .filter(|s| matches!(s, TraceStep::Drop { .. }))
        .count();
    assert_eq!(
        drops,
        lesson.initial.cells.len(),
        "every initial cell is tried exactly once in the kept pass"
    );
    let introduced = lesson
        .trace
        .iter()
        .filter(|s| matches!(s, TraceStep::Introduce { .. }))
        .count();
    assert_eq!(introduced, lesson.introduced().len());
    // Witness exclusion (m2 adjudication, the §12.9 steps-5/6 twin): every
    // introduced cell must EXCLUDE the widening witness that provoked it —
    // a budget-spending no-op cell is unconstructible.
    for step in &lesson.trace {
        if let TraceStep::Introduce {
            cell,
            witness:
                WideningWitness::World {
                    decision_index,
                    world_index,
                    ..
                },
        } = step
        {
            assert!(
                !cell_holds_at(&dom.decisions[*decision_index], cell, *world_index),
                "an introduced cell excludes its witness"
            );
        }
    }
    // A terminating witness for a pair verdict carries a full value row.
    for step in &lesson.trace {
        let witness = match step {
            TraceStep::Drop {
                outcome: StepOutcome::Survives(w) | StepOutcome::BoundHeld { witness: w, .. },
                ..
            } => Some(w),
            TraceStep::Introduce { witness, .. } => Some(witness),
            _ => None,
        };
        if let Some(WideningWitness::World { values, .. }) = witness {
            assert!(!values.is_empty());
        }
    }
    // The frame-compatible denominator sits between the basin and the
    // eligible carrier.
    assert!(b.decisions_matched <= b.frame_compatible);
    assert!(b.frame_compatible <= b.decisions_eligible);
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
        "regret h0 S1 t5 p0: verdict [refutation: value(decisive) >= value(max-count) at every matching (decision, world)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [beaters-total(2-0)>=4 & beaters-total(1-1)<=0] surviving [none] introduced [beaters-total(2-0)>=4, beaters-total(1-1)<=0] re-pinned [none] intro-budget 2/4 basin 1/32 eligible decisions 1680/14387 eligible worlds frame-rate 1/32 (carrier: selector-resolvable decisions x their fiber worlds; domain context 104/23790, 0 excluded) triple (488,1192,0) class W (weak, strict somewhere) split refutation 1 safe-substitution 1"
    );
    let win = win.expect("2-1 is worldwise-optimal at the origin");
    check_invariants(&win);
    check_origin_in_basin(&win);
    assert_eq!(
        lesson_pin_line(&win),
        "regret h0 S1 t5 p0: verdict [win: decisive suffices per-world (attains the world optimum) at every matching (decision, world); never a seat-facing guarantee (§7.6)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [beaters-total(2-0)=4 & beaters-total(1-1)<=0] surviving [none] introduced [beaters-total(2-0)<=4, beaters-total(2-0)>=4, beaters-total(1-1)<=0] re-pinned [beaters-total(2-0)=4] intro-budget 3/4 basin 1/86 eligible decisions 1680/22101 eligible worlds frame-rate 1/86 (carrier: selector-resolvable decisions x their fiber worlds; domain context 104/23790, 0 excluded) triple (0,1680,0)"
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
        "regret h1 S0 t5 p2: verdict [refutation: value(tile(5-2)) >= value(decisive) at every matching (decision, world)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [beaters-total(3-1)<=1] surviving [none] introduced [beaters-total(3-1)<=1] re-pinned [none] intro-budget 1/4 basin 2/9 eligible decisions 222/1715 eligible worlds frame-rate 2/9 (carrier: selector-resolvable decisions x their fiber worlds; domain context 104/23790, 0 excluded) triple (73,149,0) class W (weak, strict somewhere) split refutation 2 safe-substitution 2"
    );
    let win = win.expect("5-2 is worldwise-optimal at the origin");
    check_invariants(&win);
    assert_eq!(
        lesson_pin_line(&win),
        "regret h1 S0 t5 p2: verdict [win: tile(5-2) suffices per-world (attains the world optimum) at every matching (decision, world); never a seat-facing guarantee (§7.6)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [ply=2] surviving [ply=2] introduced [none] re-pinned [none] intro-budget 0/4 basin 5/11 eligible decisions 651/2135 eligible worlds frame-rate 5/5 (carrier: selector-resolvable decisions x their fiber worlds; domain context 104/23790, 0 excluded) triple (0,651,0)"
    );
}

#[test]
fn h4_s3_t5_refutation_lesson_matches_the_pin() {
    let (refutation, _) = lessons_at(4, Seat::S3, 5);
    check_invariants(&refutation);
    check_origin_in_basin(&refutation);
    assert_eq!(
        lesson_pin_line(&refutation),
        "regret h4 S3 t5 p2: verdict [refutation: value(max-count) >= value(min-count) at every matching (decision, world)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [beaters-total(1-1)<=0 & beaters-total(5-5)<=3 & beaters-total(5-5)>=1] surviving [none] introduced [beaters-total(1-1)<=0, beaters-total(5-5)<=3, beaters-total(5-5)>=1] re-pinned [none] intro-budget 3/4 basin 4/77 eligible decisions 1926/21144 eligible worlds frame-rate 4/77 (carrier: selector-resolvable decisions x their fiber worlds; domain context 104/23790, 0 excluded) triple (1,1925,0) class W (weak, strict somewhere) split refutation 1 safe-substitution 4"
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
        "lumpability h0 t6 chassis: verdict [not-lumpable: chassis fails §12.6 at every matching decision] grade [checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)] final [(empty)] surviving [none] introduced [none] re-pinned [none] intro-budget 0/4 basin 13/13 eligible decisions 647/647 eligible worlds frame-rate 13/13 (carrier: lead-kernel trees (ply 0, horizon <= 2); domain context 104/23790, 0 excluded)"
    );
}

/// The S5c-m1 subset domain: tricks 3-6 at fiber cap 3,000 — CI-scale,
/// with the void-constrained trick-3/4 decisions in and the big fibers
/// excluded (never sampled).
fn m1_domain() -> &'static BasinDomain {
    static DOMAIN: OnceLock<BasinDomain> = OnceLock::new();
    DOMAIN.get_or_init(|| {
        BasinDomain::build(
            receipt(),
            DomainSpec {
                min_trick: 3,
                max_trick: 6,
                max_fiber: 3_000,
            },
            2,
        )
    })
}

/// Milestone-1 rent coherence: refutation rent is measured through the
/// gated application path, so applied == the basin, strict-applied == the
/// refutation subbasin, and improvement is positive exactly when strict
/// content exists (never paid in T-coverage).
fn check_refutation_rent(lesson: &Lesson) -> String {
    let rent = measure_rent(lesson, m1_domain());
    let RentReport::Refutation {
        applied,
        strict_applied,
        improvement,
    } = &rent
    else {
        panic!("a refutation lesson pays refutation rent");
    };
    assert_eq!(*applied, lesson.basin.decisions_matched);
    let (refutation, _) = lesson.subbasins().expect("a refutation lesson");
    assert_eq!(*strict_applied, refutation);
    assert_eq!(*improvement > walt_geom::qi(0), *strict_applied > 0);
    rent.to_string()
}

/// The falsification mechanics at CI scale (S5c-m1): two trick-4 origins
/// that were OUT of the S5b domain — and basin-0 there — now sit inside
/// the tricks-3-6 subset domain, where order cells can relax and cut
/// refinement can introduce atom cells. Their pinned outcomes are the
/// CI-affordable slice of the falsification measurement; the full run is
/// `results/falsification_2026-08-10.txt` (the falsification_run
/// example).
#[test]
fn t4_seeds_on_the_m1_subset_domain_match_the_pins() {
    let config = WalkerConfig {
        threads: 2,
        ..WalkerConfig::ci()
    };
    let mut lines = Vec::new();
    for (hand, seat, trick_no) in [(2usize, Seat::S2, 4usize), (3, Seat::S3, 4)] {
        let rh = &receipt().hands[hand];
        let decision = ReceiptDecision::at(rh, trick_no, seat).expect("decision reconstructs");
        let record = walk_decision(rh, &decision, &config);
        assert!(record.chosen_dominated, "the pinned seed is dominated");
        let lesson =
            generalize_regret(rh, seat, &record, m1_domain()).expect("a refutation lesson");
        check_invariants_on(&lesson, m1_domain());
        let rent = check_refutation_rent(&lesson);
        lines.push(format!("{}\n  rent: {}", lesson_pin_line(&lesson), rent));
    }
    let got = lines.join("\n");
    assert_eq!(
        got, M1_PINS,
        "regenerate via the falsification_run example at this domain"
    );
}

/// Pinned by running the same seeds at the same declared subset domain.
const M1_PINS: &str = "regret h2 S2 t4 p3: verdict [refutation: value(tile(6-6)) >= value(tile(6-3)) at every matching (decision, world)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [(empty)] surviving [none] introduced [none] re-pinned [none] intro-budget 0/4 basin 1/1 eligible decisions 1120/1120 eligible worlds frame-rate 1/1 (carrier: selector-resolvable decisions x their fiber worlds; domain context 133/58941, 75 excluded) triple (91,1029,0) class W (weak, strict somewhere) split refutation 1 safe-substitution 1\n  rent: refutation rent: applied 1 strict-applied 1 improvement 65/112\nregret h3 S3 t4 p1: verdict [refutation: value(decisive) >= value(min-count) at every matching (decision, world)] grade [worldwise at (C, minimax-omniscient); weighting-free] final [beaters-total(3-0)<=2 & beaters-total(3-1)=4] surviving [none] introduced [beaters-total(3-0)<=2, beaters-total(3-1)<=4, beaters-total(3-1)>=4] re-pinned [beaters-total(3-1)=4] intro-budget 3/4 basin 1/69 eligible decisions 2450/42299 eligible worlds frame-rate 1/69 (carrier: selector-resolvable decisions x their fiber worlds; domain context 133/58941, 75 excluded) triple (191,2259,0) class W (weak, strict somewhere) split refutation 1 safe-substitution 1\n  rent: refutation rent: applied 1 strict-applied 1 improvement 1243/1225";

/// The (H, fixed-uniform-legal) re-measurement end-to-end at CI cost
/// (S5c-m2): label transfer is §12.4-fragile by default, so what actually
/// transferred is pinned with exact rationals. Two affordable lessons:
/// the h0 S1 t5 refutation (one 1,680-world fiber at horizon 3) and the
/// h1 S0 t5 win (five small decisions across four hands). Both measured
/// on the S5b tricks-5-6 domain; the full-inventory table lives in
/// `results/label_transfer_2026-08-10.txt`.
#[test]
fn h_remeasurement_of_affordable_lessons_matches_the_pins() {
    let (refutation, win) = lessons_at(0, Seat::S1, 5);
    let report = remeasure_at_h(&refutation, domain(), receipt(), 100_000_000)
        .expect("a value lesson re-measures");
    assert_eq!(report.survives(), Some(true));
    assert_eq!(
        render_h_report(&report),
        "  h0 S1 t5 p0 fiber 1680: Q^H(2-1)=80/7 Q^H(3-2)=202/21 -> HOLDS\n  H summary: measured 1 held 1 failed 0 not-fiber-valid 0 capped 0 -> SURVIVES at (H, fixed-uniform-legal) on the measured subdomain\n"
    );
    drop(win);

    let (_, win) = lessons_at(1, Seat::S0, 5);
    let win = win.expect("5-2 is worldwise-optimal at the origin");
    let report =
        remeasure_at_h(&win, domain(), receipt(), 100_000_000).expect("a value lesson re-measures");
    assert_eq!(report.survives(), Some(true));
    assert_eq!(
        render_h_report(&report),
        "  h1 S0 t5 p2 fiber 210: Q^H(5-2)=562/315 best=562/315 -> HOLDS\n  h1 S0 t6 p2 fiber 12: Q^H(5-2)=13/12 best=13/12 -> HOLDS\n  h6 S1 t5 p2 fiber 210: Q^H(5-2)=-335/21 best=-335/21 -> HOLDS\n  h7 S2 t5 p2 fiber 210: Q^H(5-2)=1607/140 best=1607/140 -> HOLDS\n  h9 S3 t6 p2 fiber 9: Q^H(5-2)=-3 best=-3 -> HOLDS\n  H summary: measured 5 held 5 failed 0 not-fiber-valid 0 capped 0 -> SURVIVES at (H, fixed-uniform-legal) on the measured subdomain\n"
    );
}

/// The application gate (S5b.1, TRUST-01 shape: a verdict's scope is its
/// verified domain). The h2 S2 t4 p3 conflict generalizes to an EMPTY
/// implicant on the tricks-5-6 domain — its widening was vacuous, nothing
/// in the domain has 6-6 and 6-3 jointly legal. An empty implicant must
/// never syntactically match beyond its verified domain: applying the
/// lesson at its own trick-4 origin decision — where both tiles ARE legal
/// and distinct, so nothing but the domain gate can refuse — must return
/// `None` without re-verification. The in-domain positive control shows
/// the same entry point saying yes where the verified scope covers it.
#[test]
fn application_is_gated_by_the_verified_domain_spec() {
    let receipt = receipt();
    let rh = &receipt.hands[2];
    let decision = ReceiptDecision::at(rh, 4, Seat::S2).expect("decision reconstructs");
    let config = WalkerConfig {
        threads: 2,
        ..WalkerConfig::ci()
    };
    let record = walk_decision(rh, &decision, &config);
    let lesson = generalize_regret(rh, Seat::S2, &record, domain()).expect("a refutation lesson");
    assert!(
        lesson.implicant.cells.is_empty(),
        "the pinned empty-implicant case"
    );
    assert_eq!(lesson.basin.decisions_matched, 0);

    // The out-of-domain decision: the lesson's own origin, precomputed
    // through the same machinery the domain uses.
    let t4 = DomainDecision::at(receipt, 2, Seat::S2, 4);
    assert_eq!(t4.trick_no, 4);
    let worse: walt_core::Domino = "6-3".parse().expect("a tile");
    let better: walt_core::Domino = "6-6".parse().expect("a tile");
    assert!(
        t4.actions.contains(&worse) && t4.actions.contains(&better),
        "both selector tiles are legal at the origin — only the domain gate can refuse"
    );
    assert!(
        lesson_applies(&lesson, &t4).is_none(),
        "an empty implicant never matches beyond its verified domain"
    );

    // Positive control: the h0 S1 t5 refutation lesson applies at its own
    // in-domain origin decision, matching the whole fiber.
    let rh0 = &receipt.hands[0];
    let d0 = ReceiptDecision::at(rh0, 5, Seat::S1).expect("decision reconstructs");
    let record0 = walk_decision(rh0, &d0, &config);
    let lesson0 =
        generalize_regret(rh0, Seat::S1, &record0, domain()).expect("a refutation lesson");
    let origin0 = domain()
        .decisions
        .iter()
        .find(|d| d.hand == 0 && d.seat == Seat::S1 && d.trick_no == 5)
        .expect("the origin is a domain decision");
    let matched = lesson_applies(&lesson0, origin0).expect("in-domain application applies");
    assert_eq!(matched.len(), 1680, "the whole origin fiber matches");
}
