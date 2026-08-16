//! The S5c-m3 economy run: the lesson database as a working set, priced
//! in seat-facing H rent — watched-feature indexing, dual-ledger rent
//! epochs, the deletion rule with the H-checker sequencing block,
//! restart-with-retention with an identity merge, and §16.11 certificate
//! emission for the whole inventory.
//!
//! Seeds are the S5c-m1 inventory (results/falsification_2026-08-10_r2):
//! the ten dominated walker decisions at exhaustive threshold 100,000,
//! the win form at the five origins admitting one, and the h0 t6 chassis
//! §12.6 failure — re-walked and re-asserted here, never trusted from the
//! file. Domain: the declared S5c-m1 tricks-3-6 fiber-capped corpus
//! domain. The four big-fiber lessons cap at the current H solver's
//! budget and stay PROVISIONALLY HELD (a concurrent work unit lifts the
//! caps; this run does not wait for it).
//!
//! Writes `results/economy_2026-08-10.txt` (full provenance header) and
//! `results/certificates_2026-08-10/` (one file per lesson, deterministic
//! filenames from content keys). Heavy compute: the seed walks and the
//! capped H attempts dominate.

use std::path::Path;

use walt_core::Seat;
use walt_factory::{
    certificate_filename, collect_epoch, emit_certificate, generalize_lumpability,
    generalize_regret, generalize_win, lesson_applies, load_receipt, render_measurement,
    render_record, walk_decision, BasinDomain, ContentKey, DescriptorFamily, DomainSpec,
    HCheckerRegistry, InsertOutcome, Lesson, LessonDb, WalkerConfig, WatchIndex, DELETION_EPOCHS_N,
    EPOCH_UNIT, H_BUDGET_PARTICLE_STEPS, H_BUDGET_SEMANTICS, H_CACHE_CONFIG, LEDGER_VERSION,
    VOCAB_REGISTRY_VERSION,
};
use walt_kernel::ReceiptDecision;

/// The m1 seed inventory: (hand, seat, trick, win form expected).
const SEEDS: [(usize, Seat, usize, bool); 10] = [
    (0, Seat::S1, 5, true),
    (1, Seat::S0, 5, true),
    (1, Seat::S2, 3, false),
    (1, Seat::S2, 4, true),
    (2, Seat::S2, 3, false),
    (2, Seat::S2, 4, false),
    (3, Seat::S3, 4, false),
    (4, Seat::S0, 4, false),
    (4, Seat::S3, 5, true),
    (11, Seat::S1, 3, true),
];

fn main() {
    let threads: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    let receipt = load_receipt();

    let t0 = std::time::Instant::now();
    let domain = BasinDomain::build(&receipt, DomainSpec::tricks_3_to_6(), threads);
    eprintln!(
        "domain: {} decisions ({} excluded), {} worlds ({:?})",
        domain.decisions.len(),
        domain.excluded_decisions,
        domain.worlds_total,
        t0.elapsed()
    );

    // 1. Rebuild the 16-lesson inventory from its seeds.
    let config = WalkerConfig {
        enumeration_threshold: 100_000,
        threads,
        ..WalkerConfig::ci()
    };
    let mut lessons: Vec<Lesson> = Vec::new();
    for (hand, seat, trick_no, expect_win) in SEEDS {
        let rh = &receipt.hands[hand];
        let decision = ReceiptDecision::at(rh, trick_no, seat).expect("seed reconstructs");
        let t = std::time::Instant::now();
        let record = walk_decision(rh, &decision, &config);
        assert!(record.chosen_dominated, "every seed is a dominated choice");
        lessons.push(generalize_regret(rh, seat, &record, &domain).expect("a refutation lesson"));
        let win = generalize_win(rh, seat, &record, &domain, 100_000);
        assert_eq!(
            win.is_some(),
            expect_win,
            "the win-form inventory matches the m1 record at h{hand} {seat} t{trick_no}"
        );
        if let Some(win) = win {
            lessons.push(win);
        }
        eprintln!("seed h{hand} {seat} t{trick_no}: done in {:?}", t.elapsed());
    }
    lessons.push(
        generalize_lumpability(&receipt.hands[0], 6, DescriptorFamily::Chassis, &domain)
            .expect("the chassis fails §12.6 on h0 t6"),
    );
    assert_eq!(lessons.len(), 16, "the declared 16-lesson inventory");

    // 2. The database: 16 inserts, all new content.
    let mut db = LessonDb::new();
    for lesson in lessons {
        let outcome = db.insert(lesson);
        assert!(
            matches!(outcome, InsertOutcome::New { .. }),
            "the 16 lessons are pairwise distinct content"
        );
    }
    assert_eq!(db.working_len(), 16);

    // 3. The watched-feature index + the completeness cross-check.
    let mut index = WatchIndex::build(&db);
    let (mut total_pairs, mut candidate_pairs, mut applier_pairs) = (0usize, 0usize, 0usize);
    for d in &domain.decisions {
        let candidates = index.candidates(d);
        let brute: Vec<usize> = db
            .working()
            .filter(|(_, l)| lesson_applies(l, d).is_some())
            .map(|(e, _)| e)
            .collect();
        for e in &brute {
            assert!(
                candidates.contains(e),
                "candidate-completeness: the index never excludes an applier"
            );
        }
        total_pairs += db.working_len();
        candidate_pairs += candidates.len();
        applier_pairs += brute.len();
    }
    eprintln!("index cross-check done");

    // 4. Two rent-collection epochs at both labels, deletion rule after
    // each; today's registry is honestly empty.
    let registry = HCheckerRegistry::none_registered();
    let mut ledger = walt_factory::Ledger::new();
    for number in 1..=DELETION_EPOCHS_N {
        assert!(index.is_current(&db));
        let t = std::time::Instant::now();
        let epoch = collect_epoch(
            number,
            &db,
            &index,
            &domain,
            &receipt,
            H_BUDGET_PARTICLE_STEPS,
        );
        eprintln!("epoch {number} collected in {:?}", t.elapsed());
        ledger.push_epoch(epoch);
        ledger.evaluate_deletions(&mut db, &registry);
    }
    let epoch_one_details: std::collections::BTreeMap<usize, walt_factory::HLessonDetail> = ledger
        .epochs[0]
        .h_details
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    // 5. Certificates for the whole working set, epoch-1 H details.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cert_dir = root.join("results").join("certificates_2026-08-10");
    std::fs::create_dir_all(&cert_dir).expect("certificate dir");
    let mut manifest: Vec<String> = Vec::new();
    for (entry, _) in db.working() {
        // The quotable derivation: max archived grade (grade is not
        // identity; today every entry has one derivation).
        let lesson = db.quotable(entry);
        let text = emit_certificate(lesson, &domain, epoch_one_details.get(&entry), &registry);
        let name = certificate_filename(lesson);
        std::fs::write(cert_dir.join(&name), &text).expect("write certificate");
        manifest.push(format!(
            "  {} <- {}",
            name,
            walt_factory::display_name(lesson)
        ));
    }
    eprintln!("certificates emitted");

    // 6. Restart-with-retention + the identity merge on re-derivation.
    ledger.record_restart(
        "search state discarded (the watched-feature index; memo tables are run-scoped by \
         construction and cannot survive), lesson DB + archive + ledger retained"
            .to_string(),
    );
    drop(index);
    index = WatchIndex::build(&db); // rebuilt derived view, post-restart
    let (working_before, archive_before) = (db.working_len(), db.archive_len());
    let rh0 = &receipt.hands[0];
    let d0 = ReceiptDecision::at(rh0, 5, Seat::S1).expect("decision reconstructs");
    let record0 = walk_decision(rh0, &d0, &config);
    let rederived =
        generalize_regret(rh0, Seat::S1, &record0, &domain).expect("a refutation lesson");
    let rederived_key = ContentKey::of(&rederived);
    let outcome = db.insert(rederived);
    let InsertOutcome::Merged { entry, derivations } = outcome else {
        panic!("a re-derivation of existing content must MERGE, got {outcome:?}");
    };
    assert_eq!(derivations, 2, "both derivations archived");
    assert_eq!(db.working_len(), working_before, "one working-set entry");
    assert_eq!(db.archive_len(), archive_before, "no new archive entry");
    ledger.record_merge(rederived_key.hash(), derivations);
    assert!(
        index.is_current(&db),
        "a pure merge leaves the index snapshot intact"
    );
    let merged_entry = entry;

    // 7. The results file.
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line("walt S5c-m3 economy run — exploratory tier".to_string());
    line("provenance:".to_string());
    line(format!(
        "  domain (the declared corpus domain): {} — {} decisions / {} worlds, {} in-range \
         decisions EXCLUDED by the fiber cap (exclusion, never sampling; control-bias annotation: \
         the excluded set skews low-control — fiber size anti-correlates with focal control, exp5 \
         covariate)",
        domain.spec,
        domain.decisions.len(),
        domain.worlds_total,
        domain.excluded_decisions
    ));
    line(
        "  seeds: the S5c-m1 inventory (falsification_2026-08-10_r2) re-walked and re-asserted: \
         10 dominated decisions at exhaustive threshold 100000, win form at the 5 origins \
         admitting one, h0 t6 chassis §12.6 — 16 lessons"
            .to_string(),
    );
    line(format!("  epoch unit: {EPOCH_UNIT}"));
    line(format!(
        "  deletion rule (measured-consecutive): zero rent in the last N = {DELETION_EPOCHS_N} \
         MEASURED epochs, no intervening measured nonzero (rationale: one zero epoch cannot \
         distinguish idle from dead at this corpus scale — the deterministic pass must confirm \
         once before the economy acts); capped epochs are cited as gaps in the evidence \
         pattern, never as evidence; deletion records cite the pattern verbatim"
    ));
    line(
        "  pricing label: (H, fixed-uniform-legal), root weighting uniform-over-fiber — the \
         seat-facing currency; diagnostic label: (C, minimax-omniscient) — recorded, NEVER summed \
         with H; the checker lesson is priced in its own applied-count ledger at its own label"
            .to_string(),
    );
    line(format!(
        "  H budget semantics of every H row: {H_BUDGET_PARTICLE_STEPS} particle-steps per \
         decision; {H_BUDGET_SEMANTICS}; cache: {H_CACHE_CONFIG}"
    ));
    line(
        "  tie-break (deletion among mutually redundant lessons): smallest implicant (cell \
         count), then oldest certificate (archive insertion order)"
            .to_string(),
    );
    line(format!(
        "  ledger version: {LEDGER_VERSION}; vocabulary registry version: {VOCAB_REGISTRY_VERSION}"
    ));
    line(format!(
        "  H-checker registry: {} registered — every H record is UNCHECKED-EXTERNALLY and the \
         first H-rent deletion is mechanically blocked (sequencing law)",
        registry.registered().len()
    ));
    line(
        "  H-value coverage marking (walt-math amendment): every H rent row carries the H \
         label's checker coverage for its lesson — SINGLE-IMPLEMENTATION until an independent \
         checker re-verifies that lesson's values (all rows below; the Rust tree and DAG solvers \
         are one implementation lineage, not independent checks) — and no deletion decision may \
         cite a single-implementation figure"
            .to_string(),
    );
    line(String::new());
    line("watched-feature index (candidate-completeness cross-check, exhaustive):".to_string());
    line(format!(
        "  {} decisions x {} lessons = {} pairs; candidates {} ; gate-applied {} ; \
         index-excluded {} (every exclusion provably gate-refused: VERIFIED — candidates were a \
         superset of appliers at every decision)",
        domain.decisions.len(),
        db.working_len(),
        total_pairs,
        candidate_pairs,
        applier_pairs,
        total_pairs - candidate_pairs
    ));
    line(String::new());

    for epoch in &ledger.epochs {
        line(format!(
            "epoch {} (unit: {EPOCH_UNIT}); dual ledger, one lesson per block:",
            epoch.number
        ));
        for r in &epoch.records {
            line(format!("  {} key {:016x}", r.name, r.key_hash));
            line(format!(
                "    pricing   [{}]: {}",
                r.pricing_label,
                render_measurement(&r.pricing)
            ));
            line(format!(
                "    diagnostic [{}]: {}",
                r.diagnostic_label, r.diagnostic
            ));
            if let Some(coverage) = &r.h_value_coverage {
                line(format!("    H-value coverage: {coverage}"));
            }
            line(format!(
                "    applied {} decisions, sole-applier at {}",
                r.applied_decisions, r.sole_applier_decisions
            ));
        }
        line(format!(
            "  overlap: {} decisions with >=2 applying lessons:",
            epoch.overlap.len()
        ));
        for (tag, keys) in &epoch.overlap {
            let keys: Vec<String> = keys.iter().map(|k| format!("{k:016x}")).collect();
            line(format!("    {}: [{}]", tag, keys.join(", ")));
        }
        line(String::new());
    }

    line("lifetime records (decayless, cumulative):".to_string());
    for (key, life) in &ledger.lifetimes {
        line(format!(
            "  key {key:016x}: epochs seen {} measured {} capped {} measured-consecutive-zeros \
             {} cumulative-applied {}",
            life.epochs_seen,
            life.epochs_measured,
            life.epochs_capped,
            life.measured_consecutive_zeros,
            life.cumulative_applied
        ));
    }
    line(String::new());

    line("economy actions (proof-logged; reproducible from the epochs above):".to_string());
    if ledger.actions.is_empty() {
        line("  none".to_string());
    }
    for action in &ledger.actions {
        line(format!("  {}", render_record(action)));
    }
    line(String::new());

    line(format!(
        "restart-with-retention: DB retained (working {} / archive {}), index rebuilt as a \
         derived view; the re-derived h0 S1 t5 refutation MERGED into archive entry {} \
         ({} derivations, one working-set entry — identity is projected content, evidence \
         accumulates)",
        db.working_len(),
        db.archive_len(),
        merged_entry,
        db.entry(merged_entry).derivations.len()
    ));
    line(String::new());

    line(format!(
        "certificates: {} emitted to certificates_2026-08-10/ (schema-v1; one file per lesson, \
         filenames from content keys):",
        manifest.len()
    ));
    for m in &manifest {
        line(m.clone());
    }

    let results = root.join("results");
    std::fs::create_dir_all(&results).expect("results dir");
    std::fs::write(results.join("economy_2026-08-10.txt"), &out).expect("write results");
    print!("{out}");
}
