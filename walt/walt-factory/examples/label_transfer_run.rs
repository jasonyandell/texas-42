//! The S5c-m3 label-transfer re-measurement (r2): reproduce the m1 lesson
//! inventory (same domain, same seeds, deterministic — identical pin lines
//! to r1), then re-measure every lesson's basin at (H, fixed-uniform-legal)
//! under `dag-v1` budget semantics — the pooled-state memoized solver, one
//! fresh cache per decision's measurement call. The m2 run
//! (`results/label_transfer_2026-08-10.txt`, tree-v0 semantics) is left
//! committed; this writes `results/label_transfer_2026-08-10_r2.txt`.
//!
//! What changed is the budget UNIT, not the statistic: a lesson capped at
//! tree-v0 and measured here is a semantics change, recorded as such.
//! Previously-measured rows must be byte-identical in value (CI-asserted
//! solver-side in `tests/h_value_transparency.rs`).
//!
//! Per-decision particle-step budget is a CLI arg (default 100,000,000);
//! over-budget decisions stay H-capped — excluded, never sampled.
//!
//! Mode `r3` (third CLI arg; budget default 1,000,000,000) is the
//! raised-declared-budget supplement: ONLY the lessons whose basins
//! contain the four r2-capped decisions, re-measured sequentially at the
//! raised budget, written to `results/label_transfer_2026-08-10_r3.txt`.
//! r3 differs from r2 by declared budget only — same solver, same
//! dag-v1 semantics; raising the declared budget is the walt-math-lawful
//! response to capped fibers (never key coarsening).

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use walt_core::Seat;
use walt_factory::label_transfer::remeasure_at_h_dag;
use walt_factory::{
    generalize_lumpability, generalize_regret, generalize_win, lesson_pin_line, load_receipt,
    render_h_report, BasinDomain, DescriptorFamily, DomainSpec, Lesson, LessonVerdict,
    WalkerConfig,
};

/// One lesson's measured block and its survival contribution.
struct MeasuredLesson {
    block: String,
    /// `None` for the checker lesson; else (is-refutation, survives()).
    value: Option<(bool, Option<bool>)>,
}

/// The four decisions the r1/r2 runs capped (hand, seat, trick_no) — the
/// r3 supplement's whole scope.
const R2_CAPPED: &[(usize, Seat, usize)] = &[
    (1, Seat::S2, 4),
    (1, Seat::S3, 3),
    (5, Seat::S1, 3),
    (11, Seat::S1, 3),
];

/// The r3 supplement: only the lessons whose basins touch the r2-capped
/// decisions, re-measured sequentially (one decision at a time, cache
/// freed between — the per-measurement-call scope) at the raised
/// declared budget.
fn run_r3(
    lessons: &[Lesson],
    domain: &walt_factory::BasinDomain,
    receipt: &walt_core::receipt::Receipt,
    budget: u64,
) {
    let affected: Vec<&Lesson> = lessons
        .iter()
        .filter(|l| {
            !matches!(l.verdict, LessonVerdict::NotLumpable { .. })
                && l.basin.matched.iter().any(|m| {
                    R2_CAPPED
                        .iter()
                        .any(|&(h, s, t)| m.hand == h && m.seat == s && m.trick_no == t)
                })
        })
        .collect();

    let mut body = String::new();
    let mut survive = 0usize;
    let mut fail = 0usize;
    let mut unmeasured = 0usize;
    let mut capped_rows = 0usize;
    for lesson in &affected {
        body.push_str(&lesson_pin_line(lesson));
        body.push('\n');
        let t = std::time::Instant::now();
        let report = remeasure_at_h_dag(lesson, domain, receipt, budget).expect("a value lesson");
        body.push_str(&render_h_report(&report));
        body.push_str(&format!("  (H measurement {:?})\n", t.elapsed()));
        body.push('\n');
        let (_, _, _, _, capped) = report.counts();
        capped_rows += capped;
        match report.survives() {
            Some(true) => survive += 1,
            Some(false) => fail += 1,
            None => unmeasured += 1,
        }
    }

    let headline = format!(
        "walt S5c-m3 label-transfer raised-budget supplement (r3) — exploratory tier\n\
         scope: ONLY the lessons whose basins contain the four r2-capped decisions (h1 S2 t4 p0, h1 S3 t3 p1, h5 S1 t3 p3, h11 S1 t3 p3); every other lesson's record stands in r2 (label_transfer_2026-08-10_r2.txt) and is not re-measured here\n\
         relation to r2: r3 differs from r2 by DECLARED BUDGET ONLY — same solver, same dag-v1 semantics, same domain and lesson inventory; raising the declared budget is the lawful response to capped fibers (walt-math Fork 1: bigger declared budget or restructured computation, never key coarsening)\n\
         solver: walt-strat ScalarHidden + dag-v1 pooled-state boundary memoization (S5c-m3), unmemoized tree walk retained as the transparency reference (tests/h_value_transparency.rs)\n\
         budget: {budget} particle-steps per decision, semantics=dag-v1 — one unit per (particle, node) visit computed on the memoized DAG; cache hits cost zero by unit definition; capped is EXCLUSION, never sampling; measured sequentially, one decision at a time, cache freed between (per-measurement-call scope)\n\
         cache: fresh per measurement call of one solver frame (frame carried by table scope), unbounded, no trimming; key = (canonical weighted world-multiset: particles sorted by packed hands, weights gcd-normalized unit-fraction denominators) x (leader) at trick boundaries with the ruled key's prefix component empty\n\
         semantics note: these decisions were CAPPED at tree-v0 10^8 (r1) and at dag-v1 10^8 (r2); measured here at dag-v1 {budget} — a SEMANTICS-AND-BUDGET declaration change from r1 and a budget change from r2, never the same statistic improving; each measured row carries tree-equiv=, the exact tree-v0 cost of the identical computation\n\
         provenance: SINGLE-IMPLEMENTATION — every value below is computed by the memoized dag-v1 implementation only, until an independent re-verification (planned: uncapped tree-walk cross-validation receipt, results/h_tree_crossval_2026-08-10.txt) covers it; nothing here is promoted above exploratory tier\n\
         supplement outcome: {survive} survive, {fail} fail, {unmeasured} unmeasured of {} re-measured lessons; {capped_rows} decision rows still capped\n",
        affected.len(),
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(
        root.join("results/label_transfer_2026-08-10_r3.txt"),
        format!("{headline}\n{body}"),
    )
    .expect("write results");
    print!("{headline}");
    println!("---");
    print!("{body}");
}

fn main() {
    let threads: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    let r3 = std::env::args().nth(3).is_some_and(|s| s == "r3");
    let budget: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(if r3 { 1_000_000_000 } else { 100_000_000 });
    let receipt = load_receipt();
    let domain = BasinDomain::build(&receipt, DomainSpec::tricks_3_to_6(), threads);
    let config = WalkerConfig {
        enumeration_threshold: 100_000,
        threads,
        ..WalkerConfig::ci()
    };

    let mut lessons: Vec<Lesson> = Vec::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let walk = walt_factory::walk_seat(hand, seat, &config);
            for record in &walk.decisions {
                if !record.chosen_dominated {
                    continue;
                }
                lessons.push(
                    generalize_regret(hand, seat, record, &domain).expect("a refutation lesson"),
                );
                if let Some(win) = generalize_win(hand, seat, record, &domain, 100_000) {
                    lessons.push(win);
                }
            }
        }
    }
    lessons.push(
        generalize_lumpability(&receipt.hands[0], 6, DescriptorFamily::Chassis, &domain)
            .expect("the chassis fails §12.6 on h0 t6"),
    );

    if r3 {
        run_r3(&lessons, &domain, &receipt, budget);
        return;
    }

    // Each measurement call is single-threaded and cache-isolated by
    // construction, so lessons re-measure concurrently with a
    // schedule-independent result: slot `i` depends on lesson `i` only.
    // The pool is kept small — the big-fiber calls carry the caches.
    let next = AtomicUsize::new(0);
    let slots: Mutex<Vec<Option<MeasuredLesson>>> =
        Mutex::new((0..lessons.len()).map(|_| None).collect());
    let workers = threads.clamp(1, 4);
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(lesson) = lessons.get(i) else {
                    break;
                };
                let mut block = String::new();
                block.push_str(&lesson_pin_line(lesson));
                block.push('\n');
                let t = std::time::Instant::now();
                let value = match remeasure_at_h_dag(lesson, &domain, &receipt, budget) {
                    None => {
                        block.push_str(
                            "  H: not re-measured (checker verdict lives at the fixed field already)\n",
                        );
                        None
                    }
                    Some(report) => {
                        block.push_str(&render_h_report(&report));
                        block.push_str(&format!("  (H measurement {:?})\n", t.elapsed()));
                        let is_refut = matches!(lesson.verdict, LessonVerdict::Refutation { .. });
                        Some((is_refut, report.survives()))
                    }
                };
                block.push('\n');
                slots.lock().expect("no poisoned slots")[i] = Some(MeasuredLesson { block, value });
            });
        }
    });

    let mut body = String::new();
    let mut survive = 0usize;
    let mut fail = 0usize;
    let mut unmeasured = 0usize;
    let mut refut_survive = 0usize;
    let mut refut_total = 0usize;
    for slot in slots.into_inner().expect("no poisoned slots") {
        let m = slot.expect("every lesson measured");
        body.push_str(&m.block);
        if let Some((is_refut, survives)) = m.value {
            if is_refut {
                refut_total += 1;
            }
            match survives {
                Some(true) => {
                    survive += 1;
                    if is_refut {
                        refut_survive += 1;
                    }
                }
                Some(false) => fail += 1,
                None => unmeasured += 1,
            }
        }
    }

    let headline = format!(
        "walt S5c-m3 label-transfer re-measurement (r2) — exploratory tier\n\
         solver: walt-strat ScalarHidden + dag-v1 pooled-state boundary memoization (S5c-m3); the unmemoized tree walk is retained as the value-transparency reference, CI-asserted byte-identical on every m2-measured decision (tests/h_value_transparency.rs)\n\
         budget: {budget} particle-steps per decision, semantics=dag-v1 — one unit per (particle, node) visit computed on the memoized DAG; a cache hit costs zero by unit definition, deterministically (measurability is a function of declared inputs, never run history; capped is EXCLUSION, never sampling)\n\
         cache: fresh per measurement call of one solver frame (the frame is carried by table scope), unbounded, no trimming; key = (canonical weighted world-multiset: particles sorted by packed hands, weights gcd-normalized unit-fraction denominators) x (leader), stored at trick boundaries with the ruled key's prefix component empty — sound (full key, never a coarsening); (boundary state, observed prefix) -> mid-trick state is surjective determination, not bijection, so convergent mid-trick states are foregone hits by design (walt-math, S5c-m3)\n\
         semantics note: r1 (label_transfer_2026-08-10.txt) measured at tree-v0; a row capped there and measured here is a SEMANTICS CHANGE (tree-v0 -> dag-v1), never the same statistic improving; previously-measured rows are byte-identical in value; each measured dag row carries tree-equiv=, the exact tree-v0 cost of the identical computation\n\
         provenance: any row measured here that r1 capped is computed by the memoized implementation only — marked SINGLE-IMPLEMENTATION until an independent checker re-verifies one\n\
         inventory: the m1 lessons reproduced on {} ({} decisions, {} excluded — exclusion frontier is control-biased: fiber size anti-correlates with focal control, exp5 covariate), re-measured at (H, fixed-uniform-legal), root weighting uniform-over-fiber\n\
         survival: {survive} survive, {fail} fail, {unmeasured} unmeasured of {} value lessons; refutations {refut_survive}/{refut_total} survive\n",
        domain.spec,
        domain.decisions.len(),
        domain.excluded_decisions,
        lessons.len() - 1,
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(
        root.join("results/label_transfer_2026-08-10_r2.txt"),
        format!("{headline}\n{body}"),
    )
    .expect("write results");
    print!("{headline}");
    println!("---");
    print!("{body}");
}
