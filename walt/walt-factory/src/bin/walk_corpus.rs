//! The full corpus walk: all 13 receipt hands x 4 seats, whole transcripts
//! (trick 1 on), exhaustive through trick-3-scale fibers, recorded-seed
//! samples above. Run in release; the CI tests pin only the fast subset,
//! this binary's output is summarized selectively.
//!
//! Resumable: `walk_corpus [start_hand [start_seat_index [max_pairs]]]`
//! walks the hand-major pair order from `(start_hand, start_seat)` on, at
//! most `max_pairs` pairs. Per-decision sample seeds are a fixed function
//! of (base seed, hand, seat, trick) — never of walk order — so a resumed
//! run's output for a pair is identical to the original run's (the
//! per-pair wall-time field aside), and the parts concatenate into one
//! artifact. Flushed line-by-line so a killed run loses no completed pair.

use std::io::Write as _;
use std::time::Instant;

use walt_factory::{load_receipt, walk_seat, EvidenceBasis, WalkerConfig};

fn main() {
    let args: Vec<usize> = std::env::args()
        .skip(1)
        .map(|a| a.parse().expect("numeric arguments"))
        .collect();
    let start_hand = args.first().copied().unwrap_or(0);
    let start_seat = args.get(1).copied().unwrap_or(0);
    let max_pairs = args.get(2).copied().unwrap_or(usize::MAX);

    let config = WalkerConfig::full();
    println!(
        "walk_corpus: threshold {} draws {} seed {:#018x} min_trick {} from h{} s{} max {}",
        config.enumeration_threshold,
        config.sample_draws,
        config.seed,
        config.min_trick,
        start_hand,
        start_seat,
        max_pairs
    );
    let receipt = load_receipt();
    let start = Instant::now();
    let mut conflict_total = 0usize;
    let mut walked = 0usize;
    for hand in &receipt.hands {
        for seat in walt_core::Seat::ALL {
            if (hand.id, seat.index()) < (start_hand, start_seat) || walked >= max_pairs {
                continue;
            }
            walked += 1;
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
            std::io::stdout().flush().expect("stdout");
        }
    }
    println!(
        "walk_corpus: {} pairs walked, {} conflicts total, {} ms",
        walked,
        conflict_total,
        start.elapsed().as_millis()
    );
}
