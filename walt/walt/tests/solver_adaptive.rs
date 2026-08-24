//! V-gates for `solver::adaptive` — the vertical-slice acceptance of the
//! calculated-evidence program (parent `walt/math/calculated_evidence_v0.1.md`
//! §19: V8 mini, V9 mini, V4 mini; §22 steps 2/5/6 in miniature; rulings
//! CE-A3/A5/A7).
//!
//! The small root is hand 4, trick 6 of the frozen `verify_player` receipt:
//! fiber 90, and the slice's two frozen preference policies split it
//! 78/34 — nonzero pivotal mass and a decisive exact gap, found by exact
//! enumeration and pinned below.

mod common;

use std::collections::HashMap;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::solver::adaptive::{
    evaluate_pair, evaluate_pair_with_switch, exact_frozen_pair, world_id, CanonicalRoot,
    FixedPreference, PairSpec, ResultKind, RootPosition,
};
use walt::solver::evidence::ScopedDelta;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The slice's small root: hand 4, trick 6 (fiber 90).
const SMALL_HAND: usize = 4;
const SMALL_TRICK: usize = 6;

fn small_root(r: &Receipt) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[SMALL_HAND];
    assert_eq!(hand.id, SMALL_HAND);
    let kernel = Kernel::from_receipt_trick(hand, SMALL_TRICK).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, SMALL_TRICK).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn policies() -> (FixedPreference, FixedPreference, FixedPreference) {
    (
        FixedPreference::highest_first("slice:highest-first"),
        FixedPreference::lowest_first("slice:lowest-first"),
        FixedPreference::lowest_first("slice:field-lowest-first"),
    )
}

// ---------------------------------------------------------------------------
// §22 step 2 — the canonical kernel adapter.
// ---------------------------------------------------------------------------

/// One canonical object serves exact count, exactly-uniform samples,
/// canonical world identity, and enumeration — and world i is a pure
/// function of (root identity, epoch, i).
#[test]
fn the_canonical_root_serves_count_sampling_identity_and_enumeration_from_one_object() {
    let r = receipt();
    let (root, position) = small_root(&r);
    assert_eq!(root.count(), 90, "the pinned small-root fiber size");
    // Enumeration and the counting DP describe the same set, through the
    // same object.
    let worlds: Vec<_> = root.worlds().collect();
    assert_eq!(worlds.len() as u128, root.count());
    let distinct: std::collections::HashSet<[u32; 4]> = worlds.iter().map(world_id).collect();
    assert_eq!(
        distinct.len(),
        worlds.len(),
        "world identity separates the fiber"
    );
    // Sampled worlds are members of the same fiber, and the counter-based
    // stream is a pure function of (root identity, epoch, index).
    let root_id = walt::solver::adaptive::root_identity(&root, &position);
    let mut seen_distinct = std::collections::HashSet::new();
    for i in 0..64u64 {
        let w = root.world_at(root_id, 7, i);
        assert!(
            root.kernel().contains(&w),
            "sampled world {i} is in the fiber"
        );
        assert_eq!(
            w,
            root.world_at(root_id, 7, i),
            "world {i} is a pure function"
        );
        seen_distinct.insert(world_id(&w));
    }
    assert!(
        seen_distinct.len() > 1,
        "a with-replacement stream on a 90-world fiber visits many worlds"
    );
    // A different epoch is a different stream.
    let stream_a: Vec<_> = (0..16u64)
        .map(|i| world_id(&root.world_at(root_id, 7, i)))
        .collect();
    let stream_b: Vec<_> = (0..16u64)
        .map(|i| world_id(&root.world_at(root_id, 8, i)))
        .collect();
    assert_ne!(stream_a, stream_b, "epochs separate evidence streams");
}

// ---------------------------------------------------------------------------
// §22 step 6 (minimum) — the exact endpoint on the small root, pinned.
// ---------------------------------------------------------------------------

/// The exact `ExactFrozenSet` on the small root, by cold full enumeration:
/// 78 of 90 worlds for highest-first, 34 of 90 for lowest-first.
#[test]
fn the_small_root_exact_frozen_set_is_pinned() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (a, b, field) = policies();
    let cold = exact_frozen_pair(&root, &position, &a, &b, &field, &HashMap::new());
    assert_eq!(cold.reused, 0, "a cold run reuses nothing");
    assert_eq!(cold.fresh, 90);
    match &cold.result {
        ResultKind::ExactFrozenSet {
            policy_a,
            policy_b,
            wins_a,
            wins_b,
            fiber,
            winner,
            ..
        } => {
            assert_eq!(policy_a, "slice:highest-first");
            assert_eq!(policy_b, "slice:lowest-first");
            assert_eq!(*wins_a, 78);
            assert_eq!(*wins_b, 34);
            assert_eq!(*fiber, 90);
            assert_eq!(winner.as_deref(), Some("slice:highest-first"));
        }
        other => panic!("expected ExactFrozenSet, got {other}"),
    }
}

// ---------------------------------------------------------------------------
// V8 (mini) — batching does not change worlds, counts, crossing, or result.
// ---------------------------------------------------------------------------

#[test]
fn v8_mini_batch_size_does_not_change_worlds_counts_crossing_or_result() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (a, b, field) = policies();
    let spec_with_batch = |batch: u64| PairSpec {
        root: &root,
        position: &position,
        policy_a: &a,
        policy_b: &b,
        field: &field,
        delta: ScopedDelta::new("decision:v8-mini", q(1, 100)),
        epoch: 3,
        world_cap: 4096,
        batch,
    };
    let e1 = evaluate_pair(&spec_with_batch(1));
    let e7 = evaluate_pair(&spec_with_batch(7));
    let e64 = evaluate_pair(&spec_with_batch(64));
    // Identical world IDs and identical pair counts at each stream index …
    assert_eq!(e1.trace, e7.trace);
    assert_eq!(e1.trace, e64.trace);
    // … identical first crossing and identical result.
    assert_eq!(e1.result, e7.result);
    assert_eq!(e1.result, e64.result);
    // The gate exercises a real crossing, and the settlement index is the
    // trace's last record under every batch size.
    match &e1.result {
        ResultKind::DeltaSettled {
            winner, settled_at, ..
        } => {
            assert_eq!(winner, "slice:highest-first");
            assert_eq!(
                e1.trace.last().expect("a nonempty trace").index,
                *settled_at
            );
        }
        other => panic!("expected DeltaSettled on this decisive root, got {other}"),
    }
}

// ---------------------------------------------------------------------------
// V9 (mini) — exact-switch parity with cold enumeration.
// ---------------------------------------------------------------------------

/// Forcing the exact switch at several different stream indices yields the
/// identical exact result as a cold full enumeration, and sampled
/// multiplicities collapse to distinct worlds — never double-counted
/// (§11.4; obligation O24's gate).
#[test]
fn v9_mini_exact_switch_parity_matches_cold_enumeration_without_double_counting() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (a, b, field) = policies();
    let cold = exact_frozen_pair(&root, &position, &a, &b, &field, &HashMap::new());
    let spec = PairSpec {
        root: &root,
        position: &position,
        policy_a: &a,
        policy_b: &b,
        field: &field,
        delta: ScopedDelta::new("decision:v9-mini", q(1, 100)),
        epoch: 5,
        world_cap: 4096,
        batch: 16,
    };
    for switch_at in [0u64, 1, 17, 90, 350] {
        let escalated = evaluate_pair_with_switch(&spec, switch_at);
        assert_eq!(
            escalated.result, cold.result,
            "exact switch at stream index {switch_at} equals cold enumeration"
        );
        // Every fiber world is counted exactly once: reused + fresh = 90,
        // and reuse never exceeds the distinct prefix (multiplicities from
        // with-replacement sampling collapse in the cache).
        assert_eq!(
            u128::from(escalated.reused) + u128::from(escalated.fresh),
            root.count()
        );
        assert!(u128::from(escalated.reused) <= u128::from(switch_at).min(root.count()));
    }
    // A 350-draw prefix on a 90-world fiber has certainly repeated worlds;
    // the cache still holds at most 90 entries by construction (asserted
    // above via reused ≤ fiber). Nothing further to spend here: the parity
    // assertions carry the gate.
}

// ---------------------------------------------------------------------------
// V4 (mini) — REGRESSION EVIDENCE, not the correctness claim.
// ---------------------------------------------------------------------------

/// Parent §19 V4: "The theorem, not the empirical frequency, carries
/// correctness. The experiment catches implementation defects." Many
/// adaptive streams against the exact `ExactFrozenSet` winner at
/// δ_dec = 1/100: the observed disagreement frequency must be compatible
/// with δ. 60 streams at δ = 1/100 expect at most 0.6 false settlements;
/// four or more would be wildly incompatible.
#[test]
fn v4_mini_adaptive_streams_disagree_with_the_exact_winner_at_a_rate_compatible_with_delta() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (a, b, field) = policies();
    let cold = exact_frozen_pair(&root, &position, &a, &b, &field, &HashMap::new());
    let exact_winner = match &cold.result {
        ResultKind::ExactFrozenSet { winner, .. } => {
            winner.clone().expect("the pinned root is not a tie")
        }
        other => panic!("expected ExactFrozenSet, got {other}"),
    };
    let mut settled = 0u32;
    let mut wrong = 0u32;
    let mut unresolved = 0u32;
    for epoch in 0..60u64 {
        let evaluation = evaluate_pair(&PairSpec {
            root: &root,
            position: &position,
            policy_a: &a,
            policy_b: &b,
            field: &field,
            delta: ScopedDelta::new("decision:v4-mini", q(1, 100)),
            epoch,
            world_cap: 4096,
            batch: 32,
        });
        match evaluation.result {
            ResultKind::DeltaSettled { winner, .. } => {
                settled += 1;
                if winner != exact_winner {
                    wrong += 1;
                }
            }
            ResultKind::Unresolved { .. } => unresolved += 1,
            other => panic!("the slice evaluator produced {other}"),
        }
    }
    assert_eq!(settled + unresolved, 60);
    assert!(
        settled >= 30,
        "most streams settle on this decisive root (settled {settled} of 60)"
    );
    assert!(
        wrong <= 3,
        "false settlements incompatible with delta = 1/100: {wrong} of {settled}"
    );
}

// ---------------------------------------------------------------------------
// CE-A3 — the six-way ladder is mechanically distinct and serialized with
// its type preserved.
// ---------------------------------------------------------------------------

#[test]
fn result_kinds_serialize_with_the_type_tag_preserved() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (a, b, field) = policies();
    // The three producible kinds, produced.
    let exact = exact_frozen_pair(&root, &position, &a, &b, &field, &HashMap::new()).result;
    let settled = evaluate_pair(&PairSpec {
        root: &root,
        position: &position,
        policy_a: &a,
        policy_b: &b,
        field: &field,
        delta: ScopedDelta::new("decision:serialize", q(1, 100)),
        epoch: 11,
        world_cap: 4096,
        batch: 16,
    })
    .result;
    // A cap of zero worlds is the degenerate resource limit: Unresolved at
    // once, never a settlement (CE-A3/A5).
    let unresolved = evaluate_pair(&PairSpec {
        root: &root,
        position: &position,
        policy_a: &a,
        policy_b: &b,
        field: &field,
        delta: ScopedDelta::new("decision:serialize", q(1, 100)),
        epoch: 11,
        world_cap: 0,
        batch: 16,
    })
    .result;
    // The three type-only kinds of this slice, constructed as values.
    let ladder = [
        ResultKind::ExactFiberRoot { fiber: 90 },
        exact,
        settled,
        ResultKind::EpsilonEquivalent {
            epsilon: q(1, 64),
            delta: ScopedDelta::new("decision:serialize", q(1, 100)),
        },
        unresolved,
        ResultKind::HeuristicFallback {
            fallback: "legacy-level1".to_string(),
        },
    ];
    let tags = [
        "ExactFiberRoot",
        "ExactFrozenSet",
        "DeltaSettled",
        "EpsilonEquivalent",
        "Unresolved",
        "HeuristicFallback",
    ];
    for (kind, tag) in ladder.iter().zip(tags) {
        assert_eq!(kind.tag(), tag);
        let serialized = kind.to_string();
        assert!(
            serialized.starts_with(&format!("{tag}{{")),
            "serialization preserves the type: {serialized}"
        );
    }
    // Probabilistic kinds carry their scoped δ and their stream identity.
    for kind in [&ladder[2], &ladder[4]] {
        let serialized = kind.to_string();
        assert!(serialized.contains("delta[decision:serialize]=1/100"));
        assert!(serialized.contains("with_replacement=true"));
        assert!(serialized.contains("fiber=90"));
    }
}
