//! The S5b measured run: seed conflicts from the walker corpus at the CI
//! config, generalize each into lessons, and measure every basin on the
//! declared exhaustive domain (tricks 5-6, all hands, all seats).
//!
//! Writes:
//!   - `results/lesson_basins_2026-08-10.txt`: one pin line per lesson,
//!     then every full lesson receipt (deterministic rendering);
//!   - `tests/data/lesson_h0_S1_t5.txt`: the designated byte-frozen lesson
//!     receipt (the h0 S1 t5 dominance conflict's refutation lesson).
//!
//! Everything exploratory tier: walt-tier evidence at declared configs,
//! never axioms. Regenerate via this example; never hand-edit outputs.

use std::path::Path;

use walt_core::Seat;
use walt_factory::{
    generalize_lumpability, generalize_regret, generalize_win, lesson_pin_line, load_receipt,
    render_lesson, walk_seat, BasinDomain, DescriptorFamily, DomainSpec, Lesson, WalkerConfig,
};

fn main() {
    let receipt = load_receipt();
    let t0 = std::time::Instant::now();
    let domain = BasinDomain::build(&receipt, DomainSpec::tricks_5_to_6(), 4);
    eprintln!(
        "domain: {} decisions, {} worlds ({:?})",
        domain.decisions.len(),
        domain.worlds_total,
        t0.elapsed()
    );

    let config = WalkerConfig {
        threads: 4,
        ..WalkerConfig::ci()
    };
    let mut lessons: Vec<Lesson> = Vec::new();
    for hand in &receipt.hands {
        for seat in Seat::ALL {
            let walk = walk_seat(hand, seat, &config);
            for record in &walk.decisions {
                if !record.chosen_dominated {
                    continue;
                }
                let t = std::time::Instant::now();
                let lesson = generalize_regret(hand, seat, record, &domain)
                    .expect("a dominated record seeds a refutation lesson");
                eprintln!(
                    "refutation h{} {} t{} p{} generalized in {:?}",
                    hand.id,
                    seat,
                    record.trick_no,
                    record.ply,
                    t.elapsed()
                );
                lessons.push(lesson);
                if let Some(win) = generalize_win(hand, seat, record, &domain, 40_000) {
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

    let mut pins = String::new();
    let mut full = String::new();
    for lesson in &lessons {
        pins.push_str(&lesson_pin_line(lesson));
        pins.push('\n');
        full.push_str(&render_lesson(lesson));
        full.push_str("----\n");
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let results = root.join("results");
    std::fs::create_dir_all(&results).expect("results dir");
    let out = format!(
        "walt S5b lesson run — exploratory tier\n\
         domain: {} — {} decisions, {} worlds\n\
         seeds: walker corpus at the CI config (threshold {}, draws {}, seed {:#018x}, tricks {}-7), \
         all dominated-chosen decisions, win form where an action is worldwise-optimal at the origin \
         (fiber <= 40000), plus the h0 t6 chassis §12.6 failure\n\n{}\n{}",
        domain.spec,
        domain.decisions.len(),
        domain.worlds_total,
        config.enumeration_threshold,
        config.sample_draws,
        config.seed,
        config.min_trick,
        pins,
        full
    );
    std::fs::write(results.join("lesson_basins_2026-08-10.txt"), &out).expect("write results");

    // The designated byte-frozen fixture: the first lesson is h0 S1 t5's
    // refutation (hand-major, seat-major, trick-major harvest order).
    let designated = render_lesson(&lessons[0]);
    let data = root.join("tests/data");
    std::fs::create_dir_all(&data).expect("tests/data");
    std::fs::write(data.join("lesson_h0_S1_t5.txt"), &designated).expect("write fixture");

    println!("{pins}");
    println!("lessons: {}", lessons.len());
}
