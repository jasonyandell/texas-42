//! Inspector-trace tests: determinism, structural sanity, and masking
//! honesty (each seat view contains exactly what that seat knows).

use std::collections::BTreeSet;

use rob_core::RulesConfig;
use rob_player::trace::{decision_count, trace_match};
use rob_player::{MonteCarloPlayer, UtilityLens};

fn small_trace() -> rob_player::trace::TraceDocument {
    let config = RulesConfig::new(2, 2).expect("valid config");
    let player = MonteCarloPlayer {
        worlds_per_decision: 3,
        lens: UtilityLens::Points,
        seed: 5,
    };
    trace_match(config, &player, 17)
}

/// The trace is deterministic in its seeds, byte for byte.
#[test]
fn trace_deterministic() {
    assert_eq!(small_trace().to_json(), small_trace().to_json());
}

/// Structural and honesty checks over every emitted record.
#[test]
fn trace_structure_and_masking() {
    let doc = small_trace();
    assert!(!doc.hands.is_empty());
    assert_eq!(decision_count(&doc), 28 * doc.hands.len());
    for hand in &doc.hands {
        assert_eq!(hand.decisions.len(), 28);
        assert!(hand.result.is_some(), "every hand settles");
        let deal: [BTreeSet<&String>; 4] = core::array::from_fn(|s| hand.deal[s].iter().collect());
        for decision in &hand.decisions {
            // Chosen action is legal; argmax honesty on the emitted totals.
            assert!(decision.legal.contains(&decision.chosen));
            assert_eq!(decision.forced, decision.worlds == 0);
            if !decision.forced {
                assert_eq!(decision.avgs.len(), decision.legal.len());
                let max = decision.totals.iter().max().expect("nonempty");
                let chosen_index = decision
                    .legal
                    .iter()
                    .position(|t| *t == decision.chosen)
                    .expect("chosen is legal");
                assert_eq!(decision.totals[chosen_index], *max, "chosen is an argmax");
            }
            #[allow(clippy::needless_range_loop)] // s indexes deal/played/views in parallel
            for s in 0..4 {
                let view = &decision.views[s];
                // A seat's own hand is exactly its dealt hand minus its
                // publicly played tiles — nothing more leaks into a view.
                let played: BTreeSet<&String> = decision.public.played[s].iter().collect();
                let own: BTreeSet<&String> = view.own.iter().collect();
                let expected: BTreeSet<&String> = deal[s].difference(&played).copied().collect();
                assert_eq!(own, expected, "view own hand is masked exactly");
                // Marginal rows cover the pool exactly.
                assert_eq!(view.marginals.len(), view.pool.len());
                // Pool from any seat excludes that seat's own tiles.
                assert!(own.iter().all(|t| !view.pool.contains(*t)));
            }
        }
    }
}

/// The serialized document is well-formed JSON (brace/quote balance
/// scanner; full parsing is exercised by the viewer's own loader).
#[test]
fn trace_json_balanced() {
    let json = small_trace().to_json();
    let mut depth = 0i64;
    let mut in_string = false;
    let mut escaped = false;
    for ch in json.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => depth -= 1,
            _ => {}
        }
        assert!(depth >= 0);
    }
    assert_eq!(depth, 0, "balanced braces and brackets");
    assert!(!in_string, "closed strings");
    assert!(json.starts_with("{\"format\":\"rob-inspector-trace\",\"version\":1"));
}
