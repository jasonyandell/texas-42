//! Exploratory probe (wiki/idea-hierarchical-fibers, rung 1 — deep
//! counting; NOT receipt rows unless promoted by amendment): the
//! σ-response-class recursion carried across trick boundaries
//! (`gate::counting_deep`), cross-checked plan-for-plan against the
//! certified streaming engine, then pointed at the two questions the idea
//! page leaves open — how the class count grows with window depth, and
//! whether exact H = 2 at the trick-one 399,072,960-world fiber is
//! minutes-scale.
//!
//! The admissibility discipline (idea page §4) is enforced in miniature
//! here: the pooled (intensional) engine ships nothing on an argument —
//! every cross-check test asserts *whole-plan equality* against the
//! extensional engine, the same two-engine pattern as the `r_sol_engines`
//! receipt, extended to windows ≥ 2.
//!
//! Findings, frozen 2026-07-28 (release build, single thread, Jason's
//! machine; exploratory — quoted in wiki/idea-hierarchical-fibers §6):
//!
//! - Every cross-check green on first run: 756 window-1 plans ≡ the
//!   receipted counting engine; 324 full-depth + 216 boundary-3 + 6
//!   boundary-2 plans ≡ streaming.
//! - Exact H = 2 at the trick-one 399,072,960-world fiber: 7,107 / 8,381 /
//!   9,982 ms at positions 0/1/2 — seconds, not the idea page's estimated
//!   minutes. Leaf classes 909,256 / 1,176,243 / 1,156,370 (≈ 350×
//!   intensional compression); DP calls 3.6–5.1 M; 72–74 % of σ-class
//!   extensions zero-pruned. The H = 2 opening confirms the H = 1 opening
//!   at all three positions; the rejected-plan ranking reshuffles
//!   (position 0's runner-up changes).
//! - Exact H = 3 at position 0: 243,351 ms (~4 min) — 49,595,988 leaf
//!   classes, 213,650,059 DP calls, 73 % zero-pruned, opening confirmed
//!   again. Per depth step the class tree grows ≈ 55×, so intensional
//!   compression against the fixed 399 M-world fiber erodes from ≈ 350×
//!   (H = 2) to ≈ 8× (H = 3): the trick-one crossover sits near H ≈ 4.
//! - Class growth (aggregated leaf classes | fiber total): boundary 4,
//!   108 positions, H = 1/2/3: 17,889 / 250,024 / 458,479 | 87,099;
//!   boundary 3, 12 positions, H = 1/2/3/4: 5,894 / 265,808 / 2,301,626 /
//!   3,815,194 | 256,690. Where fibers are small the class tree outgrows
//!   the fiber — intensional counting pays exactly where streaming is
//!   priced out, and nowhere else.
//! - DAG dedup (folded-trick presentation keys, nodes → distinct):
//!   boundary 4 H = 2/3: 17,082 → 17,082 / 101,923 → 92,081; boundary 3
//!   H = 2/3/4: 52,800 → 52,800 / 871,295 → 833,202 / 4,293,496 →
//!   3,112,899. Within-solve merging is weak (1.00–1.38×): distinct
//!   observation paths leave distinct played-tile multisets. PLAY-12's
//!   fold congruence held on every collision (value equality asserted
//!   in-engine). Cross-solve reuse is unmeasured.

use rob_core::MechanicalState;
use rob_player::player::UtilityLens;
use rob_player::solver::gate::{counting_deep, counting_deep_dedup, counting_h1, streaming_h};
use rob_player::solver::DeepStats;
use rob_player::OpeningValue;
use rob_verify::p2::boundary_position;

fn deep(state: &MechanicalState, window: usize) -> (rob_player::Plan, DeepStats) {
    counting_deep(state, UtilityLens::Points, window, true, None)
}

/// Full-depth tail (boundaries 4..6, fibers ≤ 1,680): the deep counting
/// engine's plan equals the streaming engine's plan exactly — values,
/// actions, observation keys, bundle counts, leaf kinds — on all 324
/// positions, at full window depth.
#[test]
fn deep_matches_streaming_tail() {
    for boundary in 4..=6usize {
        let window = 7 - boundary;
        for index in 0..108u64 {
            let state = boundary_position(index, boundary);
            let (deep_plan, _) = deep(&state, window);
            let streamed = streaming_h(&state, UtilityLens::Points, window);
            assert!(!deep_plan.truncated && !streamed.truncated);
            assert_eq!(
                deep_plan, streamed,
                "boundary {boundary} index {index}: intensional ≡ extensional"
            );
            deep_plan.assert_conservation();
        }
    }
}

/// Boundary 3 (fibers ≤ 34,650): plan equality at window 2 and at full
/// depth (window 4) on all 108 positions.
#[test]
fn deep_matches_streaming_boundary3() {
    for index in 0..108u64 {
        let state = boundary_position(index, 3);
        for window in [2usize, 4] {
            let (deep_plan, _) = deep(&state, window);
            let streamed = streaming_h(&state, UtilityLens::Points, window);
            assert_eq!(
                deep_plan, streamed,
                "boundary 3 index {index} window {window}"
            );
            deep_plan.assert_conservation();
        }
    }
}

/// Boundary 2 (fibers ≤ 756,756 — the trick-3 wall the idea page aims
/// at): plan equality at window 2 on a deterministic sample.
#[test]
fn deep_matches_streaming_boundary2_sample() {
    for index in 0..6u64 {
        let state = boundary_position(index, 2);
        let (deep_plan, _) = deep(&state, 2);
        let streamed = streaming_h(&state, UtilityLens::Points, 2);
        assert_eq!(deep_plan, streamed, "boundary 2 index {index} window 2");
        deep_plan.assert_conservation();
    }
}

/// Window 1 degeneracy: the deep engine at H = 1 must reproduce the
/// certified window-1 counting engine exactly — including at boundary 0,
/// where the fiber is the full 399,072,960 worlds and neither engine
/// enumerates one. This is the σ-class logic (lead and response cases)
/// checked against the receipted engine on all 756 positions.
#[test]
fn deep_h1_matches_counting_h1() {
    for boundary in 0..7usize {
        for index in 0..108u64 {
            let state = boundary_position(index, boundary);
            let (deep_plan, _) = deep(&state, 1);
            let h1 = counting_h1(&state, UtilityLens::Points);
            assert_eq!(deep_plan, h1, "boundary {boundary} index {index} window 1");
        }
    }
}

/// The class-growth measurement (the open question rung 1 answers): how
/// does the number of counted leaf classes grow with window depth, against
/// a fixed fiber? Prints one line per (boundary, window) aggregated over
/// positions; run with `--nocapture`.
#[test]
fn class_growth_by_depth() {
    println!("boundary window positions fiber_total leaf_classes dp_calls zero_pruned");
    for (boundary, indices, windows) in [
        (4usize, 0..108u64, vec![1usize, 2, 3]),
        (3usize, 0..12u64, vec![1usize, 2, 3, 4]),
    ] {
        for window in windows {
            let mut fiber_total = 0u64;
            let mut agg = DeepStats::default();
            let mut positions = 0u64;
            for index in indices.clone() {
                let state = boundary_position(index, boundary);
                let (plan, stats) = counting_deep(&state, UtilityLens::Points, window, false, None);
                fiber_total += plan.fiber_count;
                agg.dp_calls += stats.dp_calls;
                agg.leaf_classes += stats.leaf_classes;
                agg.zero_pruned += stats.zero_pruned;
                positions += 1;
            }
            println!(
                "{boundary} {window} {positions} {fiber_total} {} {} {}",
                agg.leaf_classes, agg.dp_calls, agg.zero_pruned
            );
        }
    }
}

/// The DAG-dedup measurement (idea page §7's surviving quotient): how
/// many *distinct* residual games do the deep engine's decision nodes
/// actually face? Every key collision asserts value equality inside the
/// engine, so the run is itself evidence that kernel-equal nodes are
/// mergeable. Prints nodes / distinct per (boundary, window); the ratio
/// is a lower bound on the DAG compression (presentation-level keys, no
/// normal-form reduction). Run with `--nocapture`.
#[test]
fn dag_dedup_by_depth() {
    println!("boundary window positions nodes distinct");
    for (boundary, indices, windows) in [
        (4usize, 0..24u64, vec![2usize, 3]),
        (3usize, 0..12u64, vec![2usize, 3, 4]),
    ] {
        for window in windows {
            let (mut nodes, mut distinct, mut positions) = (0u64, 0u64, 0u64);
            for index in indices.clone() {
                let state = boundary_position(index, boundary);
                let (stats, d) = counting_deep_dedup(&state, UtilityLens::Points, window);
                nodes += stats.decision_nodes;
                distinct += d;
                positions += 1;
            }
            println!("{boundary} {window} {positions} {nodes} {distinct}");
        }
    }
}

/// Depth-growth stretch at the trick-one fiber: one exact window-3 solve
/// (position 0), for the H = 2 → H = 3 class-growth multiplier at the
/// largest pool. Heavier than `trick_one_depth_two`; run manually:
/// `cargo test --release --test hierarchical_fiber_probe depth_three -- --ignored --nocapture`
#[test]
#[ignore]
fn trick_one_depth_three() {
    let state = boundary_position(0, 0);
    let started = std::time::Instant::now();
    let (h3_plan, stats) = counting_deep(&state, UtilityLens::Points, 3, false, None);
    let millis = started.elapsed().as_millis();
    println!(
        "position 0: H=3 action {:?} value {} | {} ms | leaf_classes {} dp_calls {} zero_pruned {} decision_nodes {}",
        h3_plan.root.action,
        h3_plan.root.value_total,
        millis,
        stats.leaf_classes,
        stats.dp_calls,
        stats.zero_pruned,
        stats.decision_nodes
    );
}

/// The headline measurement: exact window-2 solves at the trick-one
/// 399,072,960-world fiber — the solve no other engine in the repo can
/// perform (streaming would need ~2.8×10⁹ world-segments against the
/// 2²⁸ budget). Reports, per position: the H = 1 and H = 2 chosen
/// openings and per-opening plan values, class counts, DP calls, and
/// wall millis. Heavy; run manually:
/// `cargo test --release --test hierarchical_fiber_probe trick_one -- --ignored --nocapture`
#[test]
#[ignore]
fn trick_one_depth_two() {
    for index in 0..3u64 {
        let state = boundary_position(index, 0);
        let mut h1_openings: Vec<OpeningValue> = Vec::new();
        let (h1_plan, _) = counting_deep(
            &state,
            UtilityLens::Points,
            1,
            false,
            Some(&mut h1_openings),
        );
        let started = std::time::Instant::now();
        let mut h2_openings: Vec<OpeningValue> = Vec::new();
        let (h2_plan, stats) = counting_deep(
            &state,
            UtilityLens::Points,
            2,
            false,
            Some(&mut h2_openings),
        );
        let millis = started.elapsed().as_millis();
        assert_eq!(h2_plan.fiber_count, 399_072_960);
        println!(
            "position {index}: fiber {} | H=1 action {:?} value {} | H=2 action {:?} value {} | {} ms",
            h2_plan.fiber_count,
            h1_plan.root.action,
            h1_plan.root.value_total,
            h2_plan.root.action,
            h2_plan.root.value_total,
            millis
        );
        println!(
            "  stats: leaf_classes {} dp_calls {} zero_pruned {} decision_nodes {}",
            stats.leaf_classes, stats.dp_calls, stats.zero_pruned, stats.decision_nodes
        );
        for (h1, h2) in h1_openings.iter().zip(h2_openings.iter()) {
            assert_eq!(h1.action, h2.action);
            println!(
                "  opening {:?}: H=1 value {} | H=2 value {}",
                h1.action, h1.value_total, h2.value_total
            );
        }
    }
}
