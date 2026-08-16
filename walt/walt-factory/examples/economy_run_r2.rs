//! The S5c-m3c re-priced economy run (r2): the same 16-lesson working set
//! priced at (H, fixed-uniform-legal) under dag-v1 budget semantics at
//! the r3-declared budget — the m3 exit artifact, the economy priced in
//! the seat's actual currency at the validated solver.
//!
//! Differences from the committed r1 (economy_2026-08-10.txt, tree-v0 at
//! 10^8): the H measurement runs on the memoized DAG at 10^9
//! particle-steps (semantics=dag-v1 — a semantics-and-budget declaration
//! change, never the same statistic improving), so the four r1
//! provisionally-held lessons now PRICE; per the r3 supplement, three of
//! them carry H FAILURES that price as MEASURED ZEROS with the
//! verdict-failed reason, and measured zeros count toward
//! measured-consecutive. The crossval receipt
//! (results/h_tree_crossval_2026-08-10.txt) backs the four big-fiber
//! decisions' values with the unmemoized tree walk — CONTEXT ONLY: it is
//! not a registered H checker, so no clearance is appended and every
//! at-collection stamp stays SINGLE-IMPLEMENTATION (the Python checker
//! remains the only clearance path). Certificates and the restart/merge
//! demonstration stand as committed in r1 and are not re-run here.
//!
//! Writes `results/economy_2026-08-10_r2.txt` (r1 left in place).

use std::path::Path;

use walt_core::Seat;
use walt_factory::{
    collect_epoch_at, generalize_lumpability, generalize_regret, generalize_win, lesson_applies,
    load_receipt, render_measurement, render_record, BasinDomain, BudgetSemantics,
    DescriptorFamily, DomainSpec, HCheckerRegistry, InsertOutcome, Lesson, LessonDb, WalkerConfig,
    WatchIndex, DELETION_EPOCHS_N, EPOCH_UNIT, H_DAG_BUDGET_PARTICLE_STEPS, H_DAG_CACHE_CONFIG,
    H_DAG_SEMANTICS, LEDGER_VERSION, VOCAB_REGISTRY_VERSION,
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

/// The four decisions the tree cross-validation receipt covers — context
/// annotation only, never a clearance source.
const CROSSVAL: [(usize, Seat, usize, usize); 4] = [
    (1, Seat::S2, 4, 0),
    (1, Seat::S3, 3, 1),
    (5, Seat::S1, 3, 3),
    (11, Seat::S1, 3, 3),
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

    // 1. The 16-lesson inventory from its seeds (identical to r1).
    let config = WalkerConfig {
        enumeration_threshold: 100_000,
        threads,
        ..WalkerConfig::ci()
    };
    let mut lessons: Vec<Lesson> = Vec::new();
    for (hand, seat, trick_no, expect_win) in SEEDS {
        let rh = &receipt.hands[hand];
        let decision = ReceiptDecision::at(rh, trick_no, seat).expect("seed reconstructs");
        let record = walt_factory::walk_decision(rh, &decision, &config);
        assert!(record.chosen_dominated, "every seed is a dominated choice");
        lessons.push(generalize_regret(rh, seat, &record, &domain).expect("a refutation lesson"));
        let win = generalize_win(rh, seat, &record, &domain, 100_000);
        assert_eq!(win.is_some(), expect_win, "the m1 win-form inventory");
        if let Some(win) = win {
            lessons.push(win);
        }
        eprintln!("seed h{hand} {seat} t{trick_no}: done");
    }
    lessons.push(
        generalize_lumpability(&receipt.hands[0], 6, DescriptorFamily::Chassis, &domain)
            .expect("the chassis fails §12.6 on h0 t6"),
    );
    assert_eq!(lessons.len(), 16, "the declared 16-lesson inventory");

    let mut db = LessonDb::new();
    for lesson in lessons {
        assert!(matches!(db.insert(lesson), InsertOutcome::New { .. }));
    }
    let index = WatchIndex::build(&db);

    // 2. The index cross-check (same contract as r1).
    let (mut total_pairs, mut candidate_pairs, mut applier_pairs) = (0usize, 0usize, 0usize);
    for d in &domain.decisions {
        let candidates = index.candidates(d);
        let brute: Vec<usize> = db
            .working()
            .filter(|(_, l)| lesson_applies(l, d).is_some())
            .map(|(e, _)| e)
            .collect();
        for e in &brute {
            assert!(candidates.contains(e), "candidate-completeness");
        }
        total_pairs += db.working_len();
        candidate_pairs += candidates.len();
        applier_pairs += brute.len();
    }

    // 3. Two dag-v1 rent-collection epochs; deletion rule after each.
    let registry = HCheckerRegistry::none_registered();
    let mut ledger = walt_factory::Ledger::new();
    for number in 1..=DELETION_EPOCHS_N {
        assert!(index.is_current(&db));
        let t = std::time::Instant::now();
        let epoch = collect_epoch_at(
            number,
            &db,
            &index,
            &domain,
            &receipt,
            H_DAG_BUDGET_PARTICLE_STEPS,
            BudgetSemantics::DagV1,
        );
        eprintln!("epoch {number} collected in {:?}", t.elapsed());
        ledger.push_epoch(epoch);
        ledger.evaluate_deletions(&mut db, &registry);
    }

    // 4. The results file.
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line("walt S5c-m3c re-priced economy run (r2) — exploratory tier".to_string());
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
        "  relation to r1 (economy_2026-08-10.txt, committed): same 16-lesson inventory, same \
         index, same deletion rule; the H measurement moves from tree-v0 at 100000000 to dag-v1 \
         at 1000000000 particle-steps — a SEMANTICS-AND-BUDGET declaration change (never the \
         same statistic improving); certificates and the restart/merge demonstration stand as \
         committed in r1"
            .to_string(),
    );
    line(format!("  epoch unit: {EPOCH_UNIT}"));
    line(format!(
        "  deletion rule (measured-consecutive): zero rent in the last N = {DELETION_EPOCHS_N} \
         MEASURED epochs, no intervening measured nonzero; capped epochs are cited as gaps; a \
         verdict that FAILS at the H label prices as a MEASURED ZERO with its reason — it counts \
         toward measured-consecutive"
    ));
    line(
        "  pricing label: (H, fixed-uniform-legal), root weighting uniform-over-fiber; \
         diagnostic label: (C, minimax-omniscient) — recorded, NEVER summed with H; the checker \
         lesson is priced in its own applied-count ledger at its own label"
            .to_string(),
    );
    line(format!(
        "  H envelope of every H row: {H_DAG_BUDGET_PARTICLE_STEPS} particle-steps per decision; \
         {H_DAG_SEMANTICS}; cache: {H_DAG_CACHE_CONFIG}"
    ));
    line(
        "  crossval context: the four big-fiber decisions' values (h1 S2 t4 p0, h1 S3 t3 p1, h5 \
         S1 t3 p3, h11 S1 t3 p3) are crossval-covered by the unmemoized tree walk \
         (results/h_tree_crossval_2026-08-10.txt) — CONTEXT ONLY: the receipt is not a \
         registered H checker, no clearance is appended, and every at-collection stamp below \
         stays SINGLE-IMPLEMENTATION; the Python checker remains the only clearance path"
            .to_string(),
    );
    line(format!(
        "  ledger version: {LEDGER_VERSION}; vocabulary registry version: \
         {VOCAB_REGISTRY_VERSION}; H-checker registry: {} registered",
        registry.registered().len()
    ));
    line(String::new());
    line("watched-feature index (candidate-completeness cross-check, exhaustive):".to_string());
    line(format!(
        "  {} decisions x {} lessons = {} pairs; candidates {} ; gate-applied {} ; \
         index-excluded {} (every exclusion provably gate-refused: VERIFIED)",
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
            "epoch {} (unit: {EPOCH_UNIT}; H at dag-v1, {} particle-steps); dual ledger:",
            epoch.number, epoch.budget_per_decision
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
            // Crossval context, where the lesson's H rows touch the
            // covered decisions (annotation only; stamps stand).
            if let Some(detail) = epoch.h_details.get(&r.entry) {
                let covered: Vec<String> = detail
                    .rows
                    .iter()
                    .filter(|row| {
                        CROSSVAL.iter().any(|(h, s, t, p)| {
                            row.hand == *h && row.seat == *s && row.trick_no == *t && row.ply == *p
                        })
                    })
                    .map(|row| format!("h{} {} t{} p{}", row.hand, row.seat, row.trick_no, row.ply))
                    .collect();
                if !covered.is_empty() {
                    line(format!(
                        "    context: H values at {} are crossval-covered (see \
                         results/h_tree_crossval_2026-08-10.txt); stamp unchanged — clearance \
                         is per ledger row via a registered checker only",
                        covered.join(", ")
                    ));
                }
            }
            line(format!(
                "    applied {} decisions, sole-applier at {}",
                r.applied_decisions, r.sole_applier_decisions
            ));
        }
        line(format!(
            "  overlap: {} decisions with >=2 applying lessons",
            epoch.overlap.len()
        ));
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

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let results = root.join("results");
    std::fs::create_dir_all(&results).expect("results dir");
    std::fs::write(results.join("economy_2026-08-10_r2.txt"), &out).expect("write results");
    print!("{out}");
}
