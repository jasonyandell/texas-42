//! The full corpus walk: all 13 receipt hands x 4 seats, whole transcripts
//! (trick 1 on), exhaustive through trick-3-scale fibers, recorded-seed
//! samples above. Run in release; the CI tests pin only the fast subset,
//! this binary's output is summarized selectively.

use std::time::Instant;

use walt_factory::{load_receipt, walk_seat, EvidenceBasis, WalkerConfig};

fn main() {
    let config = WalkerConfig::full();
    println!(
        "walk_corpus: threshold {} draws {} seed {:#018x} min_trick {}",
        config.enumeration_threshold, config.sample_draws, config.seed, config.min_trick
    );
    let receipt = load_receipt();
    let start = Instant::now();
    let mut conflict_total = 0usize;
    for hand in &receipt.hands {
        for seat in walt_core::Seat::ALL {
            let t0 = Instant::now();
            let walk = walk_seat(hand, seat, &config);
            let sampled = walk
                .decisions
                .iter()
                .filter(|d| matches!(d.basis, EvidenceBasis::Sampled { .. }))
                .count();
            let conflicts = walk.conflicts();
            conflict_total += conflicts.len();
            let role = if hand.declaring_team == seat.team() {
                "declaring"
            } else {
                "defending"
            };
            let lost = match &walk.lost_from {
                None => "-".to_string(),
                Some(v) => format!(
                    "lost(t{} p{}, {}, worldwise)",
                    v.trick_no, v.ply, v.operator
                ),
            };
            println!(
                "hand {:2} seat {} ({:9}): total_regret {:>8} zero {}/{} dominated {} sampled_decisions {} verdict {} [{} ms]",
                walk.hand,
                walk.seat,
                role,
                walk.total_regret.to_string(),
                walk.zero_regret_decisions,
                walk.decisions.len(),
                walk.dominated_choices,
                sampled,
                lost,
                t0.elapsed().as_millis()
            );
            for c in &conflicts {
                println!(
                    "    conflict t{} p{}: chosen {} regret {} better {:?} grade {:?} fiber {}",
                    c.trick_no, c.ply, c.chosen, c.regret, c.better, c.grade, c.fiber
                );
            }
        }
    }
    println!(
        "walk_corpus: {} conflicts total, {} ms",
        conflict_total,
        start.elapsed().as_millis()
    );
}
