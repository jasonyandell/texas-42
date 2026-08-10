//! The S5c-m2 label-transfer run: reproduce the m1 lesson inventory (same
//! domain, same seeds, deterministic), then re-measure every lesson's
//! basin at (H, fixed-uniform-legal) — the seat-facing operator pair.
//! §12.4 makes this the falsification-grade question for the inventory:
//! the outcome defines the currency of any lesson-database economy.
//!
//! Writes `results/label_transfer_2026-08-10.txt`: the survival headline,
//! then per lesson its (renamed-vocabulary) pin line and the full H
//! table with exact rational values. Per-decision particle-step budget is
//! a CLI arg (default 100,000,000); over-budget decisions are H-capped —
//! excluded, never sampled.

use std::path::Path;

use walt_core::Seat;
use walt_factory::{
    generalize_lumpability, generalize_regret, generalize_win, lesson_pin_line, load_receipt,
    remeasure_at_h, render_h_report, BasinDomain, DescriptorFamily, DomainSpec, Lesson,
    LessonVerdict, WalkerConfig,
};

fn main() {
    let threads: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    let budget: u64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000_000);
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

    let mut body = String::new();
    let mut survive = 0usize;
    let mut fail = 0usize;
    let mut unmeasured = 0usize;
    let mut refut_survive = 0usize;
    let mut refut_total = 0usize;
    for lesson in &lessons {
        body.push_str(&lesson_pin_line(lesson));
        body.push('\n');
        let t = std::time::Instant::now();
        match remeasure_at_h(lesson, &domain, &receipt, budget) {
            None => body.push_str(
                "  H: not re-measured (checker verdict lives at the fixed field already)\n",
            ),
            Some(report) => {
                body.push_str(&render_h_report(&report));
                body.push_str(&format!("  (H measurement {:?})\n", t.elapsed()));
                let is_refut = matches!(lesson.verdict, LessonVerdict::Refutation { .. });
                if is_refut {
                    refut_total += 1;
                }
                match report.survives() {
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
        body.push('\n');
    }

    let headline = format!(
        "walt S5c-m2 label-transfer run — exploratory tier\n\
         inventory: the m1 lessons reproduced on {} ({} decisions, {} excluded — exclusion frontier is control-biased: fiber size anti-correlates with focal control, exp5 covariate), re-measured at (H, fixed-uniform-legal), root weighting uniform-over-fiber, per-decision budget {} particle-steps\n\
         survival: {survive} survive, {fail} fail, {unmeasured} unmeasured of {} value lessons; refutations {refut_survive}/{refut_total} survive\n",
        domain.spec,
        domain.decisions.len(),
        domain.excluded_decisions,
        budget,
        lessons.len() - 1,
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(
        root.join("results/label_transfer_2026-08-10.txt"),
        format!("{headline}\n{body}"),
    )
    .expect("write results");
    print!("{headline}");
    println!("---");
    print!("{body}");
}
