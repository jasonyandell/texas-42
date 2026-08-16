//! The S5c-m3 economy machinery end-to-end at CI cost: projected-content
//! identity and merge, the watched-feature index's candidate-completeness
//! contract (exhaustive cross-check against the brute-force gate over
//! receipt-corpus domains), dual-ledger rent with "unmeasured is not
//! zero", the deletion rule with the H-checker sequencing block, and
//! §16.11 certificate shape (eleven records, NOT-APPLICABLE emitted
//! present-and-empty, H rows UNCHECKED-EXTERNALLY).
//!
//! Every pin is a walt-tier regression pin at a declared config —
//! exploratory tier, never an axiom. The full-inventory run is the
//! `economy_run` example (`results/economy_2026-08-10.txt`).

use std::sync::OnceLock;

use walt_core::receipt::Receipt;
use walt_core::Seat;
use walt_factory::{
    certificate_filename, collect_epoch, emit_certificate, generalize_lumpability,
    generalize_regret, generalize_win, h_rent, lesson_applies, load_receipt, measure_h_detail,
    walk_decision, BasinDomain, ContentKey, DescriptorFamily, DomainSpec, HCheckerDesc,
    HCheckerRegistry, HRent, InsertOutcome, Ledger, Lesson, LessonDb, RentMeasurement,
    WalkerConfig, WatchIndex, DELETION_EPOCHS_N, RECORD_KINDS,
};
use walt_geom::q;
use walt_kernel::ReceiptDecision;

fn receipt() -> &'static Receipt {
    static RECEIPT: OnceLock<Receipt> = OnceLock::new();
    RECEIPT.get_or_init(load_receipt)
}

/// The S5b tricks-5-6 exhaustive domain (2 workers; exact and
/// schedule-independent).
fn domain() -> &'static BasinDomain {
    static DOMAIN: OnceLock<BasinDomain> = OnceLock::new();
    DOMAIN.get_or_init(|| BasinDomain::build(receipt(), DomainSpec::tricks_5_to_6(), 2))
}

/// The S5c-m1 CI-scale subset domain: tricks 3-6 at fiber cap 3,000
/// (exclusion, never sampling).
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

fn refutation_at(hand: usize, seat: Seat, trick_no: usize, dom: &BasinDomain) -> Lesson {
    let rh = &receipt().hands[hand];
    let decision = ReceiptDecision::at(rh, trick_no, seat).expect("decision reconstructs");
    let config = WalkerConfig {
        threads: 2,
        ..WalkerConfig::ci()
    };
    let record = walk_decision(rh, &decision, &config);
    assert!(record.chosen_dominated, "the seed is a dominated choice");
    generalize_regret(rh, seat, &record, dom).expect("a refutation lesson")
}

fn win_at(hand: usize, seat: Seat, trick_no: usize, dom: &BasinDomain) -> Lesson {
    let rh = &receipt().hands[hand];
    let decision = ReceiptDecision::at(rh, trick_no, seat).expect("decision reconstructs");
    let config = WalkerConfig {
        threads: 2,
        ..WalkerConfig::ci()
    };
    let record = walk_decision(rh, &decision, &config);
    generalize_win(rh, seat, &record, dom, 40_000).expect("the origin admits a win form")
}

/// A small mixed working set: three t5 value lessons and the checker
/// lesson on the tricks-5-6 domain, one t4 lesson on the m1 subset
/// domain, and the empty-implicant basin-0 lesson (the zero-rent case).
fn small_db() -> LessonDb {
    let mut db = LessonDb::new();
    for lesson in [
        refutation_at(0, Seat::S1, 5, domain()),
        win_at(0, Seat::S1, 5, domain()),
        win_at(1, Seat::S0, 5, domain()),
        refutation_at(2, Seat::S2, 4, domain()), // the empty-implicant, basin-0 lesson
        refutation_at(3, Seat::S3, 4, m1_domain()),
        generalize_lumpability(&receipt().hands[0], 6, DescriptorFamily::Chassis, domain())
            .expect("the chassis fails §12.6 on h0 t6"),
    ] {
        let outcome = db.insert(lesson);
        assert!(matches!(outcome, InsertOutcome::New { .. }));
    }
    db
}

/// Fork 8: identity is projected content — a re-derivation yields the
/// SAME key and MERGES (one working-set entry, both derivations
/// archived); distinct content yields distinct keys.
#[test]
fn content_key_identity_and_merge() {
    let a = refutation_at(0, Seat::S1, 5, domain());
    let b = refutation_at(0, Seat::S1, 5, domain());
    let key_a = ContentKey::of(&a);
    assert_eq!(key_a, ContentKey::of(&b), "re-derivations project equally");
    assert_eq!(
        certificate_filename(&a),
        certificate_filename(&b),
        "filenames are functions of the content key"
    );
    let win = win_at(0, Seat::S1, 5, domain());
    assert_ne!(
        key_a,
        ContentKey::of(&win),
        "distinct verdicts, distinct keys"
    );

    let mut db = LessonDb::new();
    assert!(matches!(db.insert(a), InsertOutcome::New { entry: 0 }));
    let outcome = db.insert(b);
    let InsertOutcome::Merged { entry, derivations } = outcome else {
        panic!("a re-derivation must merge, got {outcome:?}");
    };
    assert_eq!((entry, derivations), (0, 2));
    assert_eq!(db.working_len(), 1, "one working-set entry");
    assert_eq!(db.archive_len(), 1, "one archived content");
    assert_eq!(db.entry(0).derivations.len(), 2, "both derivations kept");
}

/// Fork 4: the candidate-completeness contract, cross-checked
/// exhaustively against the brute-force gate over BOTH receipt-corpus
/// test domains — a missed applier is a completeness bug and fails here.
/// The index must also do real work (exclude something), and every
/// exclusion is proven sound by the superset assertion itself.
#[test]
fn index_candidates_are_complete_over_the_receipt_corpus_domains() {
    let db = small_db();
    let index = WatchIndex::build(&db);
    assert_eq!(index.len(), db.working_len());
    let mut candidate_pairs = 0usize;
    let mut applier_pairs = 0usize;
    let mut total_pairs = 0usize;
    for dom in [domain(), m1_domain()] {
        for d in &dom.decisions {
            let candidates = index.candidates(d);
            let brute: Vec<usize> = db
                .working()
                .filter(|(_, l)| lesson_applies(l, d).is_some())
                .map(|(e, _)| e)
                .collect();
            for e in &brute {
                assert!(
                    candidates.contains(e),
                    "completeness: applier {e} missing from candidates at h{} {} t{} p{}",
                    d.hand,
                    d.seat,
                    d.trick_no,
                    d.ply
                );
            }
            total_pairs += db.working_len();
            candidate_pairs += candidates.len();
            applier_pairs += brute.len();
        }
    }
    assert!(applier_pairs <= candidate_pairs);
    assert!(
        candidate_pairs < total_pairs,
        "the index excludes some (decision, lesson) pairs — it does real work"
    );

    // Invalidation (Fork 4c): current now; stale after a working-set
    // change; current again after rebuild.
    assert!(index.is_current(&db));
    let mut db = db;
    db.insert(win_at(1, Seat::S0, 5, m1_domain()));
    assert!(!index.is_current(&db), "membership change invalidates");
    let index = WatchIndex::build(&db);
    assert!(index.is_current(&db));
}

/// Fork 3: unmeasured is not zero. A budget-capped H measurement prices
/// as UNMEASURED (provisionally held), never as a measured zero; the
/// full-budget measurement of the same lesson prices exactly.
#[test]
fn unmeasured_is_never_zero_and_the_h_price_is_exact() {
    let lesson = refutation_at(0, Seat::S1, 5, domain());

    // A deliberately starved budget: the 1,680-world fiber caps.
    let starved =
        measure_h_detail(&lesson, domain(), receipt(), 1_000).expect("a value lesson re-measures");
    let rent = h_rent(&lesson, &starved, domain());
    let RentMeasurement::Unmeasured {
        capped_decisions, ..
    } = &rent
    else {
        panic!("a capped measurement must be UNMEASURED, got {rent:?}");
    };
    assert!(*capped_decisions >= 1);
    assert_eq!(
        rent.measured_zero(),
        None,
        "unmeasured never counts as a zero (or any) measurement"
    );

    // The full budget: the S5c-m2 pinned values, Q^H(2-1)=80/7 over
    // Q^H(3-2)=202/21, improvement exactly 38/21.
    let full = measure_h_detail(&lesson, domain(), receipt(), 100_000_000)
        .expect("a value lesson re-measures");
    let rent = h_rent(&lesson, &full, domain());
    let RentMeasurement::Measured(HRent::Refutation {
        applied,
        strict,
        tied,
        failed,
        improvement,
    }) = &rent
    else {
        panic!("a full-budget measurement prices, got {rent:?}");
    };
    assert_eq!(
        (*applied, *strict, *tied, *failed),
        (1, 1, 0, 0),
        "one strict basin decision"
    );
    assert_eq!(
        *improvement,
        q(38, 21),
        "Q^H(2-1) - Q^H(3-2) = 80/7 - 202/21"
    );
    assert_eq!(rent.measured_zero(), Some(false));
}

/// Fork 5 + the Fork 6 sequencing law: the empty-basin lesson collects
/// N consecutive MEASURED zero H-rent epochs, the deletion triggers, an
/// empty registry mechanically BLOCKS it, a registered checker lets it
/// execute (archive untouched), and a re-derivation READMITS — rent
/// collection resumes without re-proving.
#[test]
fn deletion_is_blocked_without_an_h_checker_and_readmission_resumes() {
    let mut db = LessonDb::new();
    // The zero-rent lesson (empty implicant, basin 0, applies nowhere)
    // and a positive-rent control that must never trigger.
    let zero = refutation_at(2, Seat::S2, 4, domain());
    assert!(zero.implicant.cells.is_empty() && zero.basin.decisions_matched == 0);
    let zero_key = ContentKey::of(&zero);
    db.insert(zero);
    db.insert(refutation_at(0, Seat::S1, 5, domain()));

    let mut index = WatchIndex::build(&db);
    let mut ledger = Ledger::new();
    let registry = HCheckerRegistry::none_registered();
    for number in 1..=DELETION_EPOCHS_N {
        let epoch = collect_epoch(number, &db, &index, domain(), receipt(), 100_000_000);
        ledger.push_epoch(epoch);
        ledger.evaluate_deletions(&mut db, &registry);
    }
    let life = &ledger.lifetimes[&zero_key.hash()];
    assert_eq!(life.measured_consecutive_zeros, DELETION_EPOCHS_N);
    // A measured zero states its own reason: this one is not-applied
    // (empty basin), never verdict-failed and never unmeasured.
    let zero_row = ledger.epochs[0]
        .records
        .iter()
        .find(|r| r.key_hash == zero_key.hash())
        .expect("the zero lesson has a row");
    assert_eq!(zero_row.pricing.measured_zero(), Some(true));
    assert!(
        zero_row
            .pricing
            .zero_reason()
            .expect("a measured zero carries its reason")
            .contains("not applied"),
        "the row says WHY it is zero"
    );
    // The walt-math amendment: every H rent row is stamped with the H
    // label's checker coverage at collection time — single-implementation
    // today, for every value lesson.
    for epoch in &ledger.epochs {
        for r in &epoch.records {
            assert_eq!(
                r.h_value_coverage,
                Some(walt_factory::HValueCoverage::SingleImplementation),
                "value-lesson H rows carry the single-implementation marking"
            );
        }
    }
    let rendered: Vec<String> = ledger
        .actions
        .iter()
        .map(walt_factory::render_record)
        .collect();
    assert!(
        rendered
            .iter()
            .any(|r| r.starts_with(&format!("DELETION TRIGGERED key {:016x}", zero_key.hash()))),
        "the rule fires on N measured zeros: {rendered:?}"
    );
    assert!(
        rendered
            .iter()
            .any(|r| r.starts_with(&format!("DELETION BLOCKED key {:016x}", zero_key.hash()))),
        "an empty registry blocks execution: {rendered:?}"
    );
    assert!(
        !rendered.iter().any(|r| r.starts_with("DELETION EXECUTED")),
        "nothing executes without a registered H checker"
    );
    assert_eq!(db.working_len(), 2, "the working set is untouched");
    // The positive-rent control never accumulates a streak.
    let control_key = ContentKey::of(db.representative(1));
    assert_eq!(
        ledger.lifetimes[&control_key.hash()].measured_consecutive_zeros,
        0
    );

    // Register a checker (a test double exercising the MECHANICS only —
    // no independent H checker exists; nothing here is a claim that one
    // does). Registration ALONE is not enough (the per-row clearance
    // clause): the trigger stays blocked until the cited ROWS carry
    // appended clearance records.
    let mut registry = registry;
    registry.register(HCheckerDesc {
        name: "test-double (mechanics test only; not an independent H checker)".to_string(),
    });
    ledger.evaluate_deletions(&mut db, &registry);
    assert!(
        ledger
            .actions
            .iter()
            .map(walt_factory::render_record)
            .any(|r| r.contains("SINGLE-IMPLEMENTATION with no appended clearance record")),
        "with a checker registered but the cited rows uncleared, the \
         per-row clearance clause blocks"
    );
    assert_eq!(db.working_len(), 2, "still nothing executed");

    // Per-ROW clearance: the checker re-derives each cited row's figures
    // and clearances are appended beside them (the at-collection stamps
    // stand — append-only). The standing trigger now executes.
    let token = registry
        .token()
        .expect("a registered checker mints a token");
    for epoch in 1..=DELETION_EPOCHS_N {
        ledger.append_clearance(
            epoch,
            zero_key.hash(),
            "test-double (mechanics test only)".to_string(),
            &token,
        );
    }
    // The stamps were not rewritten: still single-implementation.
    for epoch in &ledger.epochs {
        for r in epoch
            .records
            .iter()
            .filter(|r| r.key_hash == zero_key.hash())
        {
            assert_eq!(
                r.h_value_coverage,
                Some(walt_factory::HValueCoverage::SingleImplementation),
                "clearance appends beside the row; the historical stamp stands"
            );
        }
    }
    ledger.evaluate_deletions(&mut db, &registry);
    assert!(ledger
        .actions
        .iter()
        .map(walt_factory::render_record)
        .any(|r| r.starts_with(&format!("DELETION EXECUTED key {:016x}", zero_key.hash()))),);
    assert_eq!(db.working_len(), 1, "the entry left the working set");
    assert_eq!(
        db.archive_len(),
        2,
        "the archive is untouched (evidence is monotone)"
    );
    assert!(!index.is_current(&db), "deletion invalidates the index");
    index = WatchIndex::build(&db);
    assert!(index.is_current(&db));

    // Readmission via re-derivation: same content key, working set
    // membership restored, no new archive entry, streak reset.
    let rederived = refutation_at(2, Seat::S2, 4, domain());
    let outcome = db.insert(rederived);
    let InsertOutcome::Readmitted { derivations, .. } = outcome else {
        panic!("re-deriving deleted content must readmit, got {outcome:?}");
    };
    assert_eq!(derivations, 2);
    assert_eq!((db.working_len(), db.archive_len()), (2, 2));
    ledger.record_readmission(
        zero_key.hash(),
        zero_key.canonical.clone(),
        DELETION_EPOCHS_N,
    );
    assert_eq!(
        ledger.lifetimes[&zero_key.hash()].measured_consecutive_zeros,
        0
    );
}

/// Fork 6 shape assertions: eleven records exactly, in order;
/// NOT-APPLICABLE records present-and-empty with reasons; H rows
/// UNCHECKED-EXTERNALLY under an empty registry; the internal count
/// assertions (distributions cover every matched world) run during
/// emission. The declared record-kind count is itself a CI assertion.
#[test]
fn certificates_carry_eleven_annotated_records() {
    assert_eq!(
        RECORD_KINDS.len(),
        11,
        "the §16.11 schema has eleven record kinds"
    );
    let registry = HCheckerRegistry::none_registered();

    let refutation = refutation_at(0, Seat::S1, 5, domain());
    let detail = measure_h_detail(&refutation, domain(), receipt(), 100_000_000)
        .expect("a value lesson re-measures");
    let text = emit_certificate(&refutation, domain(), Some(&detail), &registry);
    let headers: Vec<&str> = text.lines().filter(|l| l.starts_with('[')).collect();
    assert_eq!(headers.len(), 11, "eleven records, always present");
    for (i, kind) in RECORD_KINDS.iter().enumerate() {
        assert!(
            headers[i].starts_with(&format!("[{}/11] {}", i + 1, kind)),
            "record {} is {kind} in order; got {}",
            i + 1,
            headers[i]
        );
    }
    for na in [5usize, 6, 10] {
        assert!(
            headers[na].contains("NOT-APPLICABLE"),
            "scalar-inapplicable records are present-and-empty with a reason: {}",
            headers[na]
        );
    }
    assert!(
        text.contains(
            "H rows UNCHECKED-EXTERNALLY (single-implementation: no independent re-derivation of \
             these figures)"
        ),
        "H rows carry the single-implementation annotation"
    );
    assert!(
        text.contains(
            "comparison protocol (declared): checker aggregates its own per-world rows to a \
             multiset; multiset equality; asserts no pair has v_better < v_worse"
        ),
        "record 5 declares the comparison protocol verbatim"
    );
    assert!(
        text.contains("WORLD-ALIGNMENT IS UNCHECKED"),
        "record 5's coverage states the alignment caveat"
    );
    assert!(
        text.contains("matched (all cells)"),
        "record 9 emits per-world truth vectors with the matched vector"
    );
    assert!(text.contains("Q^H"), "H value rows are emitted");
    assert!(
        text.contains("coverage summary"),
        "the checked subset is restated"
    );
    assert!(
        text.contains(&format!(
            "content-key hash: {:016x}",
            ContentKey::of(&refutation).hash()
        )),
        "the header cites the content key"
    );

    // The checker lesson: no H rows, record 5 names the §12.6 carrier.
    let checker =
        generalize_lumpability(&receipt().hands[0], 6, DescriptorFamily::Chassis, domain())
            .expect("the chassis fails §12.6 on h0 t6");
    let text = emit_certificate(&checker, domain(), None, &registry);
    assert_eq!(text.lines().filter(|l| l.starts_with('[')).count(), 11);
    assert!(text.contains("checker lesson, not re-measured"));
    assert!(text.contains("no scalar value rows"));
}

/// The dag-v1 wiring (S5c-m3c) at CI cost: on a small-fiber lesson the
/// dag path prices to the SAME exact figures as tree-v0 (identical exact
/// values — only the budget unit changed), every row carries the
/// solver's memo stats as tree-equiv provenance, and a starved dag
/// budget prices as UNMEASURED carrying the dag envelope on the row —
/// never a measured zero. The full re-priced run is the economy_run_r2
/// example (`results/economy_2026-08-10_r2.txt`).
#[test]
fn dag_rent_matches_tree_rent_on_small_fibers() {
    use walt_factory::{measure_h_detail_dag, BudgetSemantics};

    let lesson = refutation_at(0, Seat::S1, 5, domain());
    let dag = measure_h_detail_dag(&lesson, domain(), receipt(), 100_000_000)
        .expect("a value lesson re-measures");
    assert!(matches!(dag.semantics, BudgetSemantics::DagV1));
    assert!(
        dag.rows.iter().all(|r| r.memo.is_some()),
        "dag rows carry memo stats (tree-equiv provenance)"
    );
    let rent = h_rent(&lesson, &dag, domain());
    let RentMeasurement::Measured(HRent::Refutation {
        applied,
        strict,
        improvement,
        ..
    }) = &rent
    else {
        panic!("the dag path prices the small lesson, got {rent:?}");
    };
    assert_eq!((*applied, *strict), (1, 1));
    assert_eq!(
        *improvement,
        q(38, 21),
        "identical exact price under either budget semantics"
    );

    let starved = measure_h_detail_dag(&lesson, domain(), receipt(), 1_000)
        .expect("a value lesson re-measures");
    let starved_rent = h_rent(&lesson, &starved, domain());
    let RentMeasurement::Unmeasured {
        budget,
        semantics_id,
        capped_decisions,
        ..
    } = &starved_rent
    else {
        panic!("a starved dag budget is UNMEASURED, got {starved_rent:?}");
    };
    assert_eq!((*budget, *semantics_id), (1_000, "dag-v1"));
    assert!(*capped_decisions >= 1);
    assert!(
        walt_factory::render_measurement(&starved_rent).contains("semantics=dag-v1"),
        "the unmeasured row cites its own envelope"
    );
    assert_eq!(starved_rent.measured_zero(), None);
}
