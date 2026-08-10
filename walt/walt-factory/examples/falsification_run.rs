//! The S5c-m1 falsification run: the lesson machinery pointed at the
//! domain where it can actually fail (PLAN.md, adjudicated re-scope).
//!
//! Domain: tricks 3-6, every decision with fiber <= 40,000, exhaustively
//! precomputed — larger trick-3 fibers are EXCLUDED, never sampled (the
//! declared choice; the excluded count is in every receipt). Seeds: every
//! dominated-chosen decision of the walker corpus at an extended
//! exhaustive threshold (100,000 — trick-3 fibers to 90,090 become
//! exhaustive-grade; above it, recorded 64-draw samples as at the CI
//! config), the win form where an origin action is worldwise-optimal
//! (origin scan exhaustive at <= 100,000), and the h0 t6 chassis §12.6
//! failure. Every lesson's basin is measured with the S5c-m1 language
//! (order cells with relaxation ladders, cut refinement) and its
//! milestone-1 rent is measured on the same domain.
//!
//! Writes `results/falsification_2026-08-10_r2.txt`: the headline block, one
//! pin+rent line pair per lesson, then every full receipt. Heavy compute:
//! run only when the corpus walk binary is not holding the machine.

use std::path::Path;

use walt_core::Seat;
use walt_factory::{
    generalize_lumpability, generalize_regret, generalize_win, lesson_pin_line, load_receipt,
    measure_rent, render_lesson, walk_seat, BasinDomain, DescriptorFamily, DomainSpec, Lesson,
    LessonVerdict, RentReport, WalkerConfig,
};

fn main() {
    let threads: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    let receipt = load_receipt();

    let t0 = std::time::Instant::now();
    let domain = BasinDomain::build(&receipt, DomainSpec::tricks_3_to_6(), threads);
    eprintln!(
        "domain: {} decisions ({} excluded by the fiber cap), {} worlds ({:?})",
        domain.decisions.len(),
        domain.excluded_decisions,
        domain.worlds_total,
        t0.elapsed()
    );

    // The extended seed harvest: exhaustive to 100,000 worlds, recorded
    // samples above, whole tricks 3-7 transcripts.
    let config = WalkerConfig {
        enumeration_threshold: 100_000,
        threads,
        ..WalkerConfig::ci()
    };

    let mut lessons: Vec<Lesson> = Vec::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let t = std::time::Instant::now();
            let walk = walk_seat(hand, seat, &config);
            for record in &walk.decisions {
                if !record.chosen_dominated {
                    continue;
                }
                let tg = std::time::Instant::now();
                let lesson = generalize_regret(hand, seat, record, &domain)
                    .expect("a dominated record seeds a refutation lesson");
                eprintln!(
                    "refutation h{} {} t{} p{} generalized in {:?} (walk {:?})",
                    hand.id,
                    seat,
                    record.trick_no,
                    record.ply,
                    tg.elapsed(),
                    t.elapsed()
                );
                lessons.push(lesson);
                if let Some(win) = generalize_win(hand, seat, record, &domain, 100_000) {
                    eprintln!(
                        "win        h{} {} t{} p{} generalized",
                        hand.id, seat, record.trick_no, record.ply
                    );
                    lessons.push(win);
                }
            }
        }
    }
    let checker = generalize_lumpability(&receipt.hands[0], 6, DescriptorFamily::Chassis, &domain)
        .expect("the chassis fails §12.6 on h0 t6 (the S4 pin)");
    lessons.push(checker);

    // The falsification headline: is the atom vocabulary doing lesson
    // work now — surviving cells, introductions, basins past frame caps?
    let mut with_atoms = 0usize;
    let mut with_introduced = 0usize;
    let mut refutation_basins: Vec<usize> = Vec::new();
    let mut win_basins: Vec<usize> = Vec::new();
    let mut pins = String::new();
    for lesson in &lessons {
        if !lesson.selecting_atom_cells().is_empty() {
            with_atoms += 1;
        }
        if !lesson.introduced().is_empty() {
            with_introduced += 1;
        }
        match lesson.verdict {
            LessonVerdict::Refutation { .. } => {
                refutation_basins.push(lesson.basin.decisions_matched)
            }
            LessonVerdict::Win { .. } => win_basins.push(lesson.basin.decisions_matched),
            LessonVerdict::NotLumpable { .. } => {}
        }
        let rent = measure_rent(lesson, &domain);
        pins.push_str(&lesson_pin_line(lesson));
        pins.push('\n');
        pins.push_str(&format!("  rent: {rent}\n"));
        if let RentReport::Refutation { strict_applied, .. } = rent {
            let _ = strict_applied;
        }
    }
    refutation_basins.sort_unstable();
    win_basins.sort_unstable();

    let mut full = String::new();
    for lesson in &lessons {
        full.push_str(&render_lesson(lesson));
        full.push_str("----\n");
    }

    let headline = format!(
        "walt S5c-m1 falsification run — exploratory tier\n\
         domain: {} — {} decisions ({} in-range excluded by the fiber cap), {} worlds\n\
         seeds: walker corpus, exhaustive <= {} (recorded {}-draw samples above), tricks {}-7; \
         win form at origin-scan threshold 100000; h0 t6 chassis §12.6\n\
         lessons: {} — {} with selecting atom cells (re-pinned pairs excluded), {} with introduced (cut-refinement) cells\n\
         refutation basins (matched decisions, sorted): {:?}\n\
         win basins (matched decisions, sorted): {:?}\n",
        domain.spec,
        domain.decisions.len(),
        domain.excluded_decisions,
        domain.worlds_total,
        config.enumeration_threshold,
        config.sample_draws,
        config.min_trick,
        lessons.len(),
        with_atoms,
        with_introduced,
        refutation_basins,
        win_basins,
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let results = root.join("results");
    std::fs::create_dir_all(&results).expect("results dir");
    let out = format!("{headline}\n{pins}\n{full}");
    std::fs::write(results.join("falsification_2026-08-10_r2.txt"), &out).expect("write results");

    print!("{headline}");
    println!("---");
    print!("{pins}");
}
