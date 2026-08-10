//! Deterministic text rendering of a walked transcript — walt's first
//! receipt-shaped artifact (exploratory tier, marked in the header). Byte
//! stability is the point: the designated fixture test freezes one walk's
//! full rendering, so every formatting choice here is part of the freeze.

use walt_core::receipt::ReceiptHand;

use crate::walker::{EvidenceBasis, HandSeatWalk, LostVerdict, WalkerConfig};

/// The labeled verdict text shared by every rendering — the operator label
/// is part of the sentence, never dropped.
fn verdict_text(lost: &Option<LostVerdict>) -> String {
    match lost {
        None => "none".to_string(),
        Some(v) => {
            let where_ = if v.at_the_deal && v.ply == 0 {
                "at the deal".to_string()
            } else if v.at_the_deal {
                format!("at the deal given the opening prefix (p{})", v.ply)
            } else {
                format!("from t{} p{}", v.trick_no, v.ply)
            };
            format!("lost {} under {} semantics, worldwise", where_, v.operator)
        }
    }
}

/// One summary line per (hand, seat): the statistics the CI corpus test
/// freezes. Dominance pairs are summed over the walked decisions.
pub fn corpus_pin_line(hand: &ReceiptHand, walk: &HandSeatWalk) -> String {
    let role = if hand.declaring_team == walk.seat.team() {
        "declaring"
    } else {
        "defending"
    };
    let dominance_pairs: usize = walk.decisions.iter().map(|d| d.dominance.len()).sum();
    format!(
        "h{} {} {}: regret {} zero {}/{} dominated {} dominance_pairs {} verdict {}",
        walk.hand,
        walk.seat,
        role,
        walk.total_regret,
        walk.zero_regret_decisions,
        walk.decisions.len(),
        walk.dominated_choices,
        dominance_pairs,
        verdict_text(&walk.lost_from)
    )
}

/// Renders one walk in full. Exact rationals throughout (`Q`'s own display:
/// `n/d`, integers bare); every sampled basis carries its recorded seed.
pub fn render_walk(hand: &ReceiptHand, walk: &HandSeatWalk, config: &WalkerConfig) -> String {
    let mut out = String::new();
    let w = &mut out;
    let line = |w: &mut String, s: String| {
        w.push_str(&s);
        w.push('\n');
    };
    line(w, "walt regret walker (S5a) — exploratory tier".to_string());
    line(
        w,
        format!(
            "config: threshold {} draws {} seed {:#018x} min_trick {}",
            config.enumeration_threshold, config.sample_draws, config.seed, config.min_trick
        ),
    );
    line(
        w,
        format!(
            "hand {} seat {} decl {:?} bid {} declaring {} viewer_team {}",
            walk.hand,
            walk.seat,
            hand.decl,
            hand.bid_points,
            hand.declaring_team,
            walk.seat.team()
        ),
    );
    line(
        w,
        "valuation q_points (each trick +-(1 + count))".to_string(),
    );
    line(
        w,
        "operator PI = the product pair (C, minimax-omniscient) of §10.3 x §10.8".to_string(),
    );
    for d in &walk.decisions {
        let basis = match d.basis {
            EvidenceBasis::Exhaustive { worlds } => format!("exhaustive({worlds})"),
            EvidenceBasis::Sampled { seed, draws } => {
                format!("sampled(seed {seed:#018x}, draws {draws})")
            }
        };
        line(
            w,
            format!(
                "decision t{} p{}: fiber {} basis {} operator {} weighting {} banked {:+}",
                d.trick_no, d.ply, d.fiber, basis, d.operator, d.weighting, d.banked
            ),
        );
        let values: Vec<String> = d
            .actions
            .iter()
            .zip(&d.values)
            .map(|(a, v)| format!("{a}={v}"))
            .collect();
        line(w, format!("  values {}", values.join(" ")));
        let dominance: Vec<String> = d
            .dominance
            .iter()
            .map(|(i, j)| {
                let t = d.triple(*i, *j);
                format!(
                    "{}>{}({}/{}/{})",
                    d.actions[*i], d.actions[*j], t.gt, t.eq, t.lt
                )
            })
            .collect();
        let evaluated = match d.basis {
            EvidenceBasis::Exhaustive { worlds } => worlds,
            EvidenceBasis::Sampled { draws, .. } => u128::from(draws),
        };
        line(
            w,
            format!(
                "  chosen {} regret {} dominance [{}] chosen_dominated {} live_worlds {}/{}",
                d.chosen,
                d.regret,
                dominance.join(" "),
                d.chosen_dominated,
                d.live_worlds,
                evaluated
            ),
        );
    }
    let lost = verdict_text(&walk.lost_from);
    line(
        w,
        format!(
            "summary: total_regret {} zero_regret {}/{} dominated_choices {} verdict {}",
            walk.total_regret,
            walk.zero_regret_decisions,
            walk.decisions.len(),
            walk.dominated_choices,
            lost
        ),
    );
    out
}
