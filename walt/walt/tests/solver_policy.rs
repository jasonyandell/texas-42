//! Gates for `solver::policy` — the frozen policy authority of parent
//! `walt/math/calculated_evidence_v0.1.md` §12 (§22 step 4; rulings
//! CE-A3/A5/A7; obligations O13/O22/O27 of `walt/SCENARIO-PLAYER.md`).
//!
//! The small root is the slice's: hand 4, trick 6 of the frozen
//! `verify_player` receipt — fiber 90, exact split 78/34 for the
//! highest-first/lowest-first preference pair over a lowest-first field.
//! The frozen wrapping of the same preferences must reproduce that exact
//! endpoint (§13.1 replay parity).

mod common;

use std::collections::HashSet;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{legal_plays, Decl, Domino, DominoSet};
use walt::solver::adaptive::{
    self, evaluate_pair, exact_frozen_pair, replay_viewer_success, world_id, CanonicalRoot,
    FixedPreference, PairSpec, PublicRecord, ResultKind, RootPosition, SlicePolicy,
};
use walt::solver::evidence::ScopedDelta;
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InfoKey, InnerSchedule, TieRule,
    DISCOVERY_DOMAIN,
};

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

fn ascending() -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index(i).expect("index < 28"))
        .collect()
}

fn descending() -> Vec<Domino> {
    ascending().into_iter().rev().collect()
}

/// The base freeze tuple for the small root, parameterized by its
/// preference order.
fn freeze(position: &RootPosition, order: Vec<Domino>) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-solver-step4-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "fixed-preference".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::None,
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::FirstInPreference,
        practical_equivalence: None,
        policy_library: "preference-library-v1".to_string(),
        mode: DecisionMode::Exact,
        action_rule: ActionRule::Preference(order),
    }
}

fn frozen_trio(position: &RootPosition) -> (FrozenPolicy, FrozenPolicy, FrozenPolicy) {
    (
        FrozenPolicy::new(freeze(position, descending())),
        FrozenPolicy::new(freeze(position, ascending())),
        // The field is its own frozen policy; it differs from the focal
        // lowest-first policy in an identity-only field (the library
        // name), so their PolicyIds differ while their actions coincide.
        FrozenPolicy::new(FreezeTuple {
            policy_library: "preference-library-v1-field".to_string(),
            ..freeze(position, ascending())
        }),
    )
}

fn root_record(position: &RootPosition) -> PublicRecord<'_> {
    PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: position,
        history: &[],
    }
}

// ---------------------------------------------------------------------------
// §12.1 / §12.5 — PolicyId is a content address; any tuple change is a
// new identity.
// ---------------------------------------------------------------------------

/// Equal tuples freeze to equal PolicyIds; the id names the policy in its
/// SlicePolicy identity string (O22: evidence observations name immutable
/// PolicyIds).
#[test]
fn equal_tuples_freeze_to_equal_policy_ids_and_the_label_names_the_id() {
    let r = receipt();
    let (_, position) = small_root(&r);
    let tuple = freeze(&position, descending());
    let once = FrozenPolicy::new(tuple.clone());
    let twice = FrozenPolicy::new(tuple.clone());
    assert_eq!(once.policy_id(), twice.policy_id());
    assert_eq!(tuple.canonical_bytes(), tuple.clone().canonical_bytes());
    assert_eq!(once.id(), format!("frozen:{}", once.policy_id()));
    assert_eq!(once.policy_id().to_string().len(), 64, "a full SHA-256 hex");
}

/// §12.5 — changing ANY field of the freeze tuple produces a different
/// PolicyId: fourteen single-field mutants and the base are fifteen
/// distinct identities.
#[test]
fn every_freeze_tuple_field_change_produces_a_new_policy_id() {
    let r = receipt();
    let (_, position) = small_root(&r);
    let base = freeze(&position, descending());
    let other_decl = Decl::ALL
        .into_iter()
        .find(|d| *d != base.decl)
        .expect("nine declarations");
    let mutants = [
        FreezeTuple {
            solver_source: "walt-solver-step4-v2".to_string(),
            ..base.clone()
        },
        FreezeTuple {
            decl: other_decl,
            ..base.clone()
        },
        FreezeTuple {
            bid: base.bid + 1,
            ..base.clone()
        },
        FreezeTuple {
            declaring_team: base.declaring_team.other(),
            ..base.clone()
        },
        FreezeTuple {
            field_model: "level-k".to_string(),
            ..base.clone()
        },
        FreezeTuple {
            field_level: 1,
            ..base.clone()
        },
        FreezeTuple {
            inner_schedule: InnerSchedule::Declared(vec![32, 16]),
            ..base.clone()
        },
        FreezeTuple {
            discovery_stream: "policy-discovery-splitmix64-counter-v2".to_string(),
            ..base.clone()
        },
        FreezeTuple {
            discovery_seed_schedule: vec![1],
            ..base.clone()
        },
        FreezeTuple {
            tie_rule: TieRule::LowestTileIndex,
            ..base.clone()
        },
        FreezeTuple {
            practical_equivalence: Some(q(1, 1000)),
            ..base.clone()
        },
        FreezeTuple {
            policy_library: "preference-library-v2".to_string(),
            ..base.clone()
        },
        FreezeTuple {
            mode: DecisionMode::Heuristic,
            ..base.clone()
        },
        FreezeTuple {
            action_rule: ActionRule::Preference(ascending()),
            ..base.clone()
        },
    ];
    let mut ids = HashSet::new();
    ids.insert(base.policy_id());
    for mutant in &mutants {
        assert!(
            ids.insert(mutant.policy_id()),
            "a tuple field changed without changing the PolicyId: {mutant:?}"
        );
    }
    assert_eq!(ids.len(), 15);
}

// ---------------------------------------------------------------------------
// §12.4 / O13 — discovery derives from information state + tuple only,
// domain-tagged apart from the evidence stream.
// ---------------------------------------------------------------------------

/// The discovery domain tag differs from the evidence stream's, and the
/// discovery stream is a pure function of (tuple, information state,
/// counter) — separated across counters and across PolicyIds.
#[test]
fn discovery_streams_are_domain_separated_and_derive_from_the_information_state_alone() {
    assert_ne!(
        DISCOVERY_DOMAIN,
        adaptive::STREAM_DOMAIN,
        "discovery and evidence seed derivations never share a domain tag"
    );
    let r = receipt();
    let (root, position) = small_root(&r);
    let (frozen_a, frozen_b, _) = frozen_trio(&position);
    let hand = root.kernel().viewer_hand();
    let record = root_record(&position);
    let key_a = InfoKey::from_public(frozen_a.policy_id(), hand, &record);
    // A pure function: the same (key, counter) always yields the same
    // stream.
    let draws = |policy: &FrozenPolicy, key: &InfoKey, counter: u64| {
        let mut rng = policy.discovery_rng(key, counter);
        [rng.next_u64(), rng.next_u64(), rng.next_u64()]
    };
    assert_eq!(draws(&frozen_a, &key_a, 0), draws(&frozen_a, &key_a, 0));
    assert_ne!(draws(&frozen_a, &key_a, 0), draws(&frozen_a, &key_a, 1));
    let key_b = InfoKey::from_public(frozen_b.policy_id(), hand, &record);
    assert_ne!(
        draws(&frozen_a, &key_a, 0),
        draws(&frozen_b, &key_b, 0),
        "different PolicyIds discover on different streams"
    );
}

/// §12.4 adversarial gate: two evaluation worlds with different hidden
/// hands but the identical focal information state MUST materialize the
/// identical action. At the root every fiber world presents the same
/// focal information state, so all 90 must agree — and they occupy ONE
/// memo entry, proving the hidden hands never reached the key.
#[test]
fn hidden_worlds_with_identical_focal_information_materialize_the_identical_action() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (frozen_a, _, _) = frozen_trio(&position);
    let viewer = root.kernel().viewer();
    let record = root_record(&position);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let worlds: Vec<_> = root.worlds().collect();
    assert_eq!(worlds.len(), 90);
    // The fiber's worlds genuinely differ in their hidden hands.
    let distinct: HashSet<[u32; 4]> = worlds.iter().map(world_id).collect();
    assert_eq!(distinct.len(), 90);
    let mut actions = HashSet::new();
    for world in &worlds {
        let hand = world.hand(viewer);
        assert_eq!(hand, root.kernel().viewer_hand());
        let legal = legal_plays(position.decl, hand, led);
        actions.insert(frozen_a.choose(position.decl, hand, legal, &record));
    }
    assert_eq!(
        actions.len(),
        1,
        "one focal information state, one materialized action"
    );
    assert_eq!(
        frozen_a.cache_len(),
        1,
        "ninety hidden worlds, one information state, one memo entry"
    );
}

// ---------------------------------------------------------------------------
// §12.2 / §12.5 / O22 — lazy materialization over an immutable cache:
// extension is legal, changing a defined action is impossible.
// ---------------------------------------------------------------------------

/// A cache miss extends the representation; a revisit reads the identical
/// action back. Earlier snapshots survive later replay verbatim, and a
/// full re-replay adds nothing.
#[test]
fn the_cache_extends_on_new_states_and_never_changes_a_defined_action() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (frozen_a, frozen_b, field) = frozen_trio(&position);
    let viewer = root.kernel().viewer();
    // Materialize along one world's replay only.
    let root_id = adaptive::root_identity(&root, &position);
    let world = root.world_at(root_id, 1, 0);
    let first = replay_viewer_success(&position, viewer, &world, &frozen_a, &field);
    let after_one_world = frozen_a.cache_snapshot();
    assert!(!after_one_world.is_empty());
    // Full exact enumeration EXTENDS the memo table without changing any
    // defined action (§12.5: extension legal, mutation impossible).
    let exact = exact_frozen_pair(
        &root,
        &position,
        &frozen_a,
        &frozen_b,
        &field,
        &std::collections::HashMap::new(),
    );
    let after_fiber = frozen_a.cache_snapshot();
    assert!(after_fiber.len() >= after_one_world.len());
    for (key, action) in &after_one_world {
        assert_eq!(
            after_fiber.get(key),
            Some(action),
            "an earlier cache entry survived later replay unchanged"
        );
    }
    // Replaying the same world again is pure cache readout: identical
    // outcome, not one new entry anywhere.
    let again = replay_viewer_success(&position, viewer, &world, &frozen_a, &field);
    assert_eq!(first, again);
    assert_eq!(frozen_a.cache_snapshot(), after_fiber);
    let _ = exact;
}

// ---------------------------------------------------------------------------
// §13.1 replay parity — the frozen wrapping reproduces the slice's exact
// endpoint, and replay after materialization is cheap.
// ---------------------------------------------------------------------------

/// The pinned exact split of the slice (78/34 over fiber 90) reproduces
/// under FrozenPolicy wrappings of the same preference orders, and a
/// second full replay is answered entirely from the memo tables.
#[test]
fn replay_parity_the_frozen_pair_reproduces_the_pinned_78_34_split_and_replays_from_cache() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (frozen_a, frozen_b, field) = frozen_trio(&position);
    let cold = exact_frozen_pair(
        &root,
        &position,
        &frozen_a,
        &frozen_b,
        &field,
        &std::collections::HashMap::new(),
    );
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
            assert_eq!(policy_a, frozen_a.id());
            assert_eq!(policy_b, frozen_b.id());
            assert_eq!(*wins_a, 78, "the slice's pinned split reproduces");
            assert_eq!(*wins_b, 34, "the slice's pinned split reproduces");
            assert_eq!(*fiber, 90);
            assert_eq!(winner.as_deref(), Some(frozen_a.id()));
        }
        other => panic!("expected ExactFrozenSet, got {other}"),
    }
    // §13.1's cure: the second full replay is pure cache readout — the
    // identical result with not one new materialization.
    let sizes = (
        frozen_a.cache_len(),
        frozen_b.cache_len(),
        field.cache_len(),
    );
    let warm = exact_frozen_pair(
        &root,
        &position,
        &frozen_a,
        &frozen_b,
        &field,
        &std::collections::HashMap::new(),
    );
    assert_eq!(warm.result, cold.result);
    assert_eq!(
        (
            frozen_a.cache_len(),
            frozen_b.cache_len(),
            field.cache_len(),
        ),
        sizes,
        "replay after materialization adds no cache entries"
    );
    // And the frozen wrapping agrees with the slice's declared-constant
    // policies exactly, world for world.
    let slice_a = FixedPreference::highest_first("slice:highest-first");
    let slice_b = FixedPreference::lowest_first("slice:lowest-first");
    let slice_field = FixedPreference::lowest_first("slice:field-lowest-first");
    let slice_cold = exact_frozen_pair(
        &root,
        &position,
        &slice_a,
        &slice_b,
        &slice_field,
        &std::collections::HashMap::new(),
    );
    match (&cold.result, &slice_cold.result) {
        (
            ResultKind::ExactFrozenSet { wins_a, wins_b, .. },
            ResultKind::ExactFrozenSet {
                wins_a: slice_wins_a,
                wins_b: slice_wins_b,
                ..
            },
        ) => {
            assert_eq!(wins_a, slice_wins_a);
            assert_eq!(wins_b, slice_wins_b);
        }
        _ => panic!("both endpoints are ExactFrozenSet"),
    }
}

/// The adaptive evaluator accepts frozen policies through the same seam:
/// on the common stream its per-index trace is identical to the slice
/// policies' trace (same worlds, same signed outcomes, same crossing),
/// and the winner is named by PolicyId.
#[test]
fn evaluate_pair_accepts_frozen_policies_and_walks_the_identical_stream() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let (frozen_a, frozen_b, field) = frozen_trio(&position);
    let slice_a = FixedPreference::highest_first("slice:highest-first");
    let slice_b = FixedPreference::lowest_first("slice:lowest-first");
    let slice_field = FixedPreference::lowest_first("slice:field-lowest-first");
    let frozen_run = evaluate_pair(&PairSpec {
        root: &root,
        position: &position,
        policy_a: &frozen_a,
        policy_b: &frozen_b,
        field: &field,
        delta: ScopedDelta::new("decision:step4-seam", q(1, 100)),
        epoch: 3,
        world_cap: 4096,
        batch: 16,
    });
    let slice_run = evaluate_pair(&PairSpec {
        root: &root,
        position: &position,
        policy_a: &slice_a,
        policy_b: &slice_b,
        field: &slice_field,
        delta: ScopedDelta::new("decision:step4-seam", q(1, 100)),
        epoch: 3,
        world_cap: 4096,
        batch: 16,
    });
    assert_eq!(
        frozen_run.trace, slice_run.trace,
        "the frozen wrapping replays the identical stream outcomes"
    );
    match (&frozen_run.result, &slice_run.result) {
        (
            ResultKind::DeltaSettled {
                winner,
                settled_at,
                a,
                b,
                ..
            },
            ResultKind::DeltaSettled {
                winner: slice_winner,
                settled_at: slice_settled_at,
                a: slice_a_count,
                b: slice_b_count,
                ..
            },
        ) => {
            assert_eq!(winner, frozen_a.id(), "the winner is named by PolicyId");
            assert_eq!(slice_winner, "slice:highest-first");
            assert_eq!(settled_at, slice_settled_at);
            assert_eq!(a, slice_a_count);
            assert_eq!(b, slice_b_count);
        }
        (f, s) => panic!("expected DeltaSettled twice, got {f} and {s}"),
    }
}
