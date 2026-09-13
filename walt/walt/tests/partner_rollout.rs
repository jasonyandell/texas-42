use std::time::Duration;
use walt::policy_search::{partner_rollout as r, request};
use walt::solver::Deadline;

const MISSED: &str = "decl 3\nbid 30\nbidder 0\nseat 2\nhand 0 2 3 15 17 21 25\nplays 0 7 1 24 2 3 3 6 1 5 2 17 3 23 0 12 1 4 2 21 3 22 0 8 0 9 1 19 2 25 3 18 0 13 1 16\nseed 420600\n";

#[test]
fn parser_retains_the_live_request_seed_and_rejects_hidden_fields() {
    let (_, seed) = request::from_text_with_seed(MISSED).unwrap();
    assert_eq!(seed, 420600);
    assert!(request::from_text_with_seed(&format!("{MISSED}hands 0 1 2\n")).is_err());
    assert!(request::from_text_with_seed(&MISSED.replace("420600", "-1")).is_err());
}

#[test]
fn completed_l1_census_matches_saved_deployed_values_and_reuses_information() {
    let (f, seed) = request::from_text_with_seed(MISSED).unwrap();
    let result = r::review(
        &f,
        seed,
        2,
        Deadline::after(Duration::from_secs(14)),
        400,
        true,
    )
    .unwrap();
    assert_eq!(result.coverage, "census");
    assert_eq!(result.samples, 210);
    assert_eq!(result.legal, vec![0, 2, 15]);
    assert_eq!(result.values[1..], [129, 151]);
    assert_eq!(result.choice, 15);
    assert_eq!(result.paired[2], [28, 6, 123, 53]);
    assert_eq!(result.traces.len(), result.samples * result.legal.len());
    assert!(result.cache_hits > 0);
    for (i, pair) in result.paired.iter().enumerate() {
        assert_eq!(pair.iter().sum::<usize>(), 210);
        assert_eq!(pair[0] + pair[2], result.values[i]);
        assert_eq!(pair[1] + pair[2], result.values[1]);
    }
    let mut policy = r::L1::new(
        f.exercise.position.decl,
        0,
        seed,
        Deadline::after(Duration::from_secs(14)),
    );
    let mut recomputed = 0;
    for d in result
        .decisions
        .iter()
        .filter(|d| d.history.len() % 4 != 0)
        .take(24)
    {
        assert_eq!(
            policy.choose(d.seat, d.original, &d.history).unwrap(),
            d.choice
        );
        assert_eq!(
            policy.choose(d.seat, d.original, &d.history).unwrap(),
            d.choice
        );
        recomputed += 1;
    }
    assert_eq!(policy.hits, recomputed);
}

#[test]
fn expired_budget_keeps_baseline_without_partial_world_evidence() {
    let (f, seed) = request::from_text_with_seed(MISSED).unwrap();
    // Already offered count still enters the check. The exhausted deadline
    // prevents any evidence, rather than being interpreted as a loss.
    let result = r::review(&f, seed, 15, Deadline::after(Duration::ZERO), 64, true).unwrap();
    assert_eq!(result.offers, [15]);
    assert_eq!(result.status, "unresolved");
    assert_eq!(result.stop, "deadline");
    assert_eq!(result.choice, 15);
    assert_eq!(result.samples, 0);
    assert!(result.traces.is_empty());
    assert!(result.values.iter().all(|&v| v == 0));
    assert!(result.paired.iter().all(|v| v == &[0; 4]));
}

#[test]
fn a_fixed_prefix_has_complete_paired_worlds_and_does_not_claim_a_census() {
    let (f, seed) = request::from_text_with_seed(MISSED).unwrap();
    let result = r::review(
        &f,
        seed,
        2,
        Deadline::after(Duration::from_secs(14)),
        3,
        true,
    )
    .unwrap();
    assert_eq!(result.samples, 3);
    assert_eq!(result.coverage, "budgeted-sample-prefix");
    assert_eq!(result.status, "unresolved");
    assert_eq!(result.choice, 2);
    assert_eq!(result.traces.len(), 9);
    for vector in result.traces.chunks(3) {
        assert!(vector.iter().all(|t| t.hands == vector[0].hands));
        assert_eq!(
            vector.iter().map(|t| t.action).collect::<Vec<_>>(),
            result.legal
        );
    }
}
