//! Gates for `solver::controller` — the §16.4 decision controller of parent
//! `walt/math/calculated_evidence_v0.1.md` (§22 step 5 generalized to `m`
//! candidates: §5 all-pairs allocation and safe elimination, §5.3 epoch
//! mutation, §9.2/§9.3 practical equivalence, §8.5 refinement vectors,
//! §11.3/§11.4 exact escalation, §17 common random worlds; rulings
//! CE-A3/A5/A7; obligations O21/O24/O26).
//!
//! Two roots, both sized by the kernel's exact count:
//!
//! - the SMALL root — hand 4, trick 6 of the frozen `verify_player`
//!   receipt, fiber 90. Its two-trick horizon makes every fixed-preference
//!   policy worth exactly 78 or 34 of 90 (probed by exact enumeration and
//!   pinned below), so it carries a unique-best set AND exact-tie sets.
//! - the SECOND root — hand 11, trick 5 of the same receipt, fiber 1120
//!   (a five-trick horizon). Four pinned candidate values 1118, 654, 563,
//!   556 of 1120: all distinct, with a decisively best member.
//!
//! Adaptive-versus-exact agreements here are REGRESSION EVIDENCE at
//! exploratory tier (parent §19 V4: the theorem carries correctness; the
//! experiment catches implementation defects). Nothing is promoted.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Signed;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::controller::{
    epoch_identity, evaluate_set, evaluate_set_with_switch, exact_frozen_set, CandidateSet,
    EquivalencePlan, EquivalenceRoute, EscalationConfig, RiskPlan, SetResult, SetSpec,
};
use walt::solver::evidence::{self, ScopedDelta};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

// ---------------------------------------------------------------------------
// Roots and candidates.
// ---------------------------------------------------------------------------

/// (hand, trick, pinned exact fiber) for the two roots.
const SMALL_ROOT: (usize, usize, u128) = (4, 6, 90);
const SECOND_ROOT: (usize, usize, u128) = (11, 5, 1120);

fn root_at(r: &Receipt, spec: (usize, usize, u128)) -> (CanonicalRoot, RootPosition) {
    let (hand_no, trick_no, fiber) = spec;
    let hand = &r.hands[hand_no];
    assert_eq!(hand.id, hand_no);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    let root = CanonicalRoot::new(kernel);
    assert_eq!(
        root.count(),
        fiber,
        "the kernel's exact count sizes the root"
    );
    (root, position)
}

fn ascending() -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index(i).expect("index < 28"))
        .collect()
}

fn descending() -> Vec<Domino> {
    ascending().into_iter().rev().collect()
}

/// A total preference order by stride: tile `(offset + mult·i) mod 28`,
/// a permutation whenever `gcd(mult, 28) = 1`.
fn stride(mult: usize, offset: usize) -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index((offset + mult * i) % 28).expect("index < 28"))
        .collect()
}

fn freeze(position: &RootPosition, order: Vec<Domino>) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-solver-step5-v1".to_string(),
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

/// The small root's candidate pool: descending (worth 78/90 over the
/// lowest-first field), then three worth 34/90 — a unique best.
fn small_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [descending(), ascending(), stride(3, 1), stride(11, 7)]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

/// The second root's candidate pool: ascending (1118/1120), stride-5
/// (654), stride-13 (563), descending (556) — four distinct exact values.
fn second_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [ascending(), stride(5, 2), stride(13, 0), descending()]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

fn field() -> FixedPreference {
    FixedPreference::lowest_first("field:lowest-first")
}

fn strict_spec<'a>(
    root: &'a CanonicalRoot,
    position: &'a RootPosition,
    candidates: &'a CandidateSet<'a>,
    field: &'a dyn SlicePolicy,
    scope: &str,
    cap: u64,
) -> SetSpec<'a> {
    SetSpec {
        root,
        position,
        candidates,
        field,
        plan: RiskPlan::strict(ScopedDelta::new(scope, q(1, 50))),
        world_cap: cap,
        batch: 16,
        escalation: None,
    }
}

fn exact_winner(evaluation: &SetResult) -> Option<usize> {
    match evaluation {
        SetResult::ExactFrozenSet { winner, .. } => *winner,
        other => panic!("expected ExactFrozenSet, got {other}"),
    }
}

// ---------------------------------------------------------------------------
// The second root is found by the kernel's exact count.
// ---------------------------------------------------------------------------

/// Fiber 1120 ≠ 90, from the same exact counting DP the sampler and the
/// exhaustive endpoint use; the second pool's exact values are pinned
/// (regression evidence, exploratory tier).
#[test]
fn the_second_root_has_a_different_fiber_size_and_pinned_exact_values() {
    let r = receipt();
    let (root, position) = root_at(&r, SECOND_ROOT);
    assert_ne!(root.count(), SMALL_ROOT.2);
    let pool = second_pool(&position);
    let candidates = CandidateSet::new(pool.iter().collect());
    let f = field();
    let cold = exact_frozen_set(&strict_spec(
        &root,
        &position,
        &candidates,
        &f,
        "decision:second-root-pin",
        0,
    ));
    match &cold.result {
        SetResult::ExactFrozenSet {
            wins,
            fiber,
            winner,
            ..
        } => {
            assert_eq!(*fiber, 1120);
            assert_eq!(wins, &[1118u128, 654, 563, 556]);
            assert_eq!(*winner, Some(0), "ascending is the unique exact best");
        }
        other => panic!("expected ExactFrozenSet, got {other}"),
    }
}

// ---------------------------------------------------------------------------
// m=3 / m=4 adaptive-versus-exact winner parity on both roots.
// ---------------------------------------------------------------------------

/// Runs `streams` independently named decisions of the same candidate set
/// and asserts every `DeltaSettled` names the exact winner. Returns
/// (settled, unresolved).
fn settled_streams_match_exact(
    root: &CanonicalRoot,
    position: &RootPosition,
    pool: &[FrozenPolicy],
    m: usize,
    scope_stem: &str,
    streams: u64,
) -> (u32, u32) {
    let candidates = CandidateSet::new(pool.iter().take(m).collect());
    let f = field();
    let cold = exact_frozen_set(&strict_spec(
        root,
        position,
        &candidates,
        &f,
        &format!("{scope_stem}-exact"),
        0,
    ));
    let best = exact_winner(&cold.result).expect("the pool's best is unique");
    let mut settled = 0u32;
    let mut unresolved = 0u32;
    for stream in 0..streams {
        // Each stream is a separately named decision: a distinct declared
        // δ scope is a distinct epoch identity, hence a fresh world
        // stream (§5.3/§17.1).
        let spec = strict_spec(
            root,
            position,
            &candidates,
            &f,
            &format!("{scope_stem}-{stream}"),
            4096,
        );
        match evaluate_set(&spec).result {
            SetResult::DeltaSettled { winner, .. } => {
                settled += 1;
                assert_eq!(
                    winner, best,
                    "a settled stream of {scope_stem} names the exact winner"
                );
            }
            SetResult::Unresolved { .. } => unresolved += 1,
            other => panic!("strict mode without escalation produced {other}"),
        }
    }
    (settled, unresolved)
}

#[test]
fn m3_adaptive_winner_matches_exact_on_every_settled_stream_on_both_roots() {
    let r = receipt();
    let (small, small_pos) = root_at(&r, SMALL_ROOT);
    let (settled, _) = settled_streams_match_exact(
        &small,
        &small_pos,
        &small_pool(&small_pos),
        3,
        "decision:m3-small",
        10,
    );
    assert!(settled >= 5, "the decisive small root settles most streams");
    let (second, second_pos) = root_at(&r, SECOND_ROOT);
    let (settled, _) = settled_streams_match_exact(
        &second,
        &second_pos,
        &second_pool(&second_pos),
        3,
        "decision:m3-second",
        6,
    );
    assert!(
        settled >= 3,
        "the decisive second root settles most streams"
    );
}

#[test]
fn m4_adaptive_winner_matches_exact_on_every_settled_stream_on_both_roots() {
    let r = receipt();
    let (small, small_pos) = root_at(&r, SMALL_ROOT);
    let (settled, _) = settled_streams_match_exact(
        &small,
        &small_pos,
        &small_pool(&small_pos),
        4,
        "decision:m4-small",
        10,
    );
    assert!(settled >= 5, "the decisive small root settles most streams");
    let (second, second_pos) = root_at(&r, SECOND_ROOT);
    let (settled, _) = settled_streams_match_exact(
        &second,
        &second_pos,
        &second_pool(&second_pos),
        4,
        "decision:m4-second",
        6,
    );
    assert!(
        settled >= 3,
        "the decisive second root settles most streams"
    );
}

// ---------------------------------------------------------------------------
// §5.1 — elimination soundness: the exact best is never eliminated.
// ---------------------------------------------------------------------------

#[test]
fn elimination_soundness_the_exact_best_is_never_eliminated() {
    let r = receipt();
    for (root_spec, pool_of, stem) in [
        (
            SMALL_ROOT,
            small_pool as fn(&RootPosition) -> Vec<FrozenPolicy>,
            "decision:elim-small",
        ),
        (SECOND_ROOT, second_pool, "decision:elim-second"),
    ] {
        let (root, position) = root_at(&r, root_spec);
        let pool = pool_of(&position);
        let candidates = CandidateSet::new(pool.iter().collect());
        let f = field();
        let cold = exact_frozen_set(&strict_spec(
            &root,
            &position,
            &candidates,
            &f,
            &format!("{stem}-exact"),
            0,
        ));
        let best = exact_winner(&cold.result).expect("the pool's best is unique");
        for stream in 0..12u64 {
            let spec = strict_spec(
                &root,
                &position,
                &candidates,
                &f,
                &format!("{stem}-{stream}"),
                4096,
            );
            let evaluation = evaluate_set(&spec);
            assert!(
                evaluation.eliminations.iter().all(|e| e.candidate != best),
                "the exact best was eliminated on stream {stream} of {stem}"
            );
            match evaluation.result {
                SetResult::DeltaSettled { winner, .. } => assert_eq!(winner, best),
                SetResult::Unresolved { survivors, .. } => {
                    assert!(survivors.contains(&best));
                }
                other => panic!("strict mode without escalation produced {other}"),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// §9.1/§9.2 — the honest tie: never a fabricated winner.
// ---------------------------------------------------------------------------

/// Two policies with IDENTICAL action behavior whose freeze tuples differ
/// only in the tie rule: distinct PolicyIds, the same actions on every
/// world. The total preference order leaves the tie rule no work, so
/// every terminal outcome pair is equal — the pair is never pivotal.
fn twins(position: &RootPosition) -> (FrozenPolicy, FrozenPolicy) {
    let base = freeze(position, descending());
    let twin = FreezeTuple {
        tie_rule: TieRule::LowestTileIndex,
        ..base.clone()
    };
    let a = FrozenPolicy::new(base);
    let b = FrozenPolicy::new(twin);
    assert_ne!(a.policy_id(), b.policy_id(), "identity differs by tie rule");
    (a, b)
}

#[test]
fn honest_tie_identical_actions_edge_never_settles_and_survives_to_the_cap() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let (a, b) = twins(&position);
    let candidates = CandidateSet::new(vec![&a, &b]);
    let f = field();
    let spec = strict_spec(
        &root,
        &position,
        &candidates,
        &f,
        "decision:honest-tie",
        512,
    );
    let evaluation = evaluate_set(&spec);
    assert!(evaluation.edges.is_empty(), "the twin edge never settles");
    assert!(evaluation.eliminations.is_empty());
    match &evaluation.result {
        SetResult::Unresolved {
            survivors,
            consumed,
            refinements,
            ..
        } => {
            assert_eq!(survivors, &[0, 1], "both twins survive to the cap");
            assert_eq!(*consumed, 512);
            // Identical actions: never pivotal, exactly.
            assert_eq!(refinements.len(), 1);
            assert_eq!((refinements[0].a, refinements[0].b), (0, 0));
            assert_eq!(refinements[0].n0, refinements[0].n);
        }
        other => panic!("a true tie under a cap is Unresolved, got {other}"),
    }
}

#[test]
fn honest_tie_under_epsilon_mode_settles_epsilon_equivalent_on_both_routes() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let (a, b) = twins(&position);
    let candidates = CandidateSet::new(vec![&a, &b]);
    let f = field();
    for (route, scope) in [
        (EquivalenceRoute::BoundedMean, "decision:eq-bounded-mean"),
        (EquivalenceRoute::PivotalMass, "decision:eq-pivotal-mass"),
    ] {
        let spec = SetSpec {
            root: &root,
            position: &position,
            candidates: &candidates,
            field: &f,
            plan: RiskPlan::with_equivalence(
                ScopedDelta::new(scope, q(1, 100)),
                q(1, 200),
                EquivalencePlan {
                    epsilon: q(1, 8),
                    delta: q(1, 200),
                    route,
                },
            ),
            world_cap: 512,
            batch: 16,
            escalation: None,
        };
        let evaluation = evaluate_set(&spec);
        assert!(evaluation.edges.is_empty(), "the twin edge never settles");
        match &evaluation.result {
            SetResult::EpsilonEquivalent {
                survivors, epsilon, ..
            } => {
                assert_eq!(survivors, &[0, 1]);
                assert_eq!(epsilon, &q(1, 8));
            }
            other => panic!("{route:?} on a true tie is EpsilonEquivalent, got {other}"),
        }
    }
}

// ---------------------------------------------------------------------------
// §5.3 — candidate-set mutation starts a new epoch on fresh worlds.
// ---------------------------------------------------------------------------

#[test]
fn epoch_mutation_adding_a_candidate_yields_a_fresh_epoch_and_divergent_streams() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let two = CandidateSet::new(pool.iter().take(2).collect());
    let three = CandidateSet::new(pool.iter().take(3).collect());
    let delta = ScopedDelta::new("decision:epoch-mutation", q(1, 50));
    let root_id = walt::solver::adaptive::root_identity(&root, &position);
    let old = epoch_identity(root_id, &two, &delta);
    let new = epoch_identity(root_id, &three, &delta);
    // Same policies plus one added: a different epoch by content address.
    assert_ne!(old, new, "candidate-set mutation is a new epoch");
    assert_ne!(old.stream_epoch(), new.stream_epoch());
    // A different epoch derives a different world stream: old evidence is
    // aligned to world IDs the new epoch never draws at those indices, so
    // old pair counts are unreachable from the new epoch by construction.
    let old_stream: Vec<_> = (0..16u64)
        .map(|i| walt::solver::adaptive::world_id(&root.world_at(root_id, old.stream_epoch(), i)))
        .collect();
    let new_stream: Vec<_> = (0..16u64)
        .map(|i| walt::solver::adaptive::world_id(&root.world_at(root_id, new.stream_epoch(), i)))
        .collect();
    assert_ne!(old_stream, new_stream, "streams diverge across epochs");
    // A changed δ declaration is likewise a fresh epoch.
    let renamed = ScopedDelta::new("decision:epoch-mutation-2", q(1, 50));
    assert_ne!(
        epoch_identity(root_id, &two, &delta),
        epoch_identity(root_id, &two, &renamed)
    );
}

#[test]
fn epoch_identity_is_invariant_under_candidate_input_order() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let forward = CandidateSet::new(pool.iter().take(3).collect());
    let reversed = CandidateSet::new(pool.iter().take(3).rev().collect());
    let delta = ScopedDelta::new("decision:epoch-order", q(1, 50));
    let root_id = walt::solver::adaptive::root_identity(&root, &position);
    assert_eq!(
        epoch_identity(root_id, &forward, &delta),
        epoch_identity(root_id, &reversed, &delta),
        "the epoch folds SORTED PolicyIds"
    );
}

// ---------------------------------------------------------------------------
// V8 at m=3 — batching changes nothing observable.
// ---------------------------------------------------------------------------

#[test]
fn v8_m3_batch_sizes_do_not_change_elimination_graph_settlement_indices_or_result() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().take(3).collect());
    let f = field();
    let with_batch = |batch: u64, escalation: Option<EscalationConfig>| {
        let spec = SetSpec {
            root: &root,
            position: &position,
            candidates: &candidates,
            field: &f,
            plan: RiskPlan::strict(ScopedDelta::new("decision:v8-m3", q(1, 50))),
            world_cap: 4096,
            batch,
            escalation,
        };
        evaluate_set(&spec)
    };
    for escalation in [
        None,
        // The switch-check cadence is a declared constant, deliberately
        // decoupled from the batch size (§17.3).
        Some(EscalationConfig {
            cost_sample: 3,
            cost_enumerate: 3,
            check_every: 32,
        }),
    ] {
        let e1 = with_batch(1, escalation.clone());
        let e16 = with_batch(16, escalation.clone());
        let e64 = with_batch(64, escalation.clone());
        for other in [&e16, &e64] {
            assert_eq!(e1.edges, other.edges, "identical settled edges");
            assert_eq!(e1.eliminations, other.eliminations);
            assert_eq!(e1.pair_counts, other.pair_counts);
            assert_eq!(e1.consumed, other.consumed);
            assert_eq!(e1.result, other.result);
            assert_eq!(e1.escalation, other.escalation);
        }
        match &e1.result {
            SetResult::DeltaSettled {
                winner, settled_at, ..
            } => {
                assert_eq!(*winner, 0, "descending is the small root's best");
                assert_eq!(
                    e1.eliminations.iter().map(|e| e.at).max(),
                    Some(*settled_at),
                    "settlement is the last elimination's stream index"
                );
            }
            SetResult::ExactFrozenSet { winner, .. } => {
                assert_eq!(*winner, Some(0));
            }
            other => panic!("the decisive m=3 set settles, got {other}"),
        }
    }
}

// ---------------------------------------------------------------------------
// V9 / O24 at m=3 — escalation parity with cold enumeration.
// ---------------------------------------------------------------------------

#[test]
fn escalation_parity_forced_switches_at_arbitrary_indices_equal_cold_enumeration() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().take(3).collect());
    let f = field();
    let spec = strict_spec(&root, &position, &candidates, &f, "decision:v9-m3", 4096);
    let cold = exact_frozen_set(&spec);
    match &cold.result {
        SetResult::ExactFrozenSet { wins, winner, .. } => {
            assert_eq!(wins, &[78u128, 34, 34], "the pinned small-root values");
            assert_eq!(*winner, Some(0));
        }
        other => panic!("expected ExactFrozenSet, got {other}"),
    }
    for switch_at in [0u64, 1, 17, 90, 350] {
        let escalated = evaluate_set_with_switch(&spec, switch_at);
        assert_eq!(
            escalated.result, cold.result,
            "forced switch at stream index {switch_at} equals cold enumeration"
        );
        let report = escalated.escalation.expect("the exact endpoint ran");
        assert_eq!(report.switched_at, switch_at);
        // Every fiber world counts exactly once: reuse plus fresh covers
        // the fiber, and reuse never exceeds the distinct sampled prefix.
        assert_eq!(
            u128::from(report.reused_worlds) + u128::from(report.fresh_worlds),
            root.count()
        );
        assert!(u128::from(report.reused_worlds) <= u128::from(switch_at).min(root.count()));
    }
}

/// §11.3 — a wrong cost forecast switches the ROUTE, never the selection:
/// weights forcing immediate escalation produce the cold exact result;
/// weights forbidding escalation produce the same winner by settlement.
#[test]
fn wrong_cost_forecasts_change_route_but_never_the_selected_winner() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().take(3).collect());
    let f = field();
    let with_escalation = |escalation: Option<EscalationConfig>| {
        let spec = SetSpec {
            root: &root,
            position: &position,
            candidates: &candidates,
            field: &f,
            plan: RiskPlan::strict(ScopedDelta::new("decision:forecast-wrong", q(1, 50))),
            world_cap: 4096,
            batch: 16,
            escalation,
        };
        evaluate_set(&spec)
    };
    let cold = exact_frozen_set(&SetSpec {
        root: &root,
        position: &position,
        candidates: &candidates,
        field: &f,
        plan: RiskPlan::strict(ScopedDelta::new("decision:forecast-wrong", q(1, 50))),
        world_cap: 4096,
        batch: 16,
        escalation: None,
    });
    // "Enumeration is free": escalates at the first check.
    let eager = with_escalation(Some(EscalationConfig {
        cost_sample: 1,
        cost_enumerate: 0,
        check_every: 1,
    }));
    assert_eq!(eager.result, cold.result);
    assert_eq!(
        eager.escalation.expect("escalated").switched_at,
        1,
        "a free-enumeration forecast escalates at the first check"
    );
    // "Sampling is free": never escalates; settles adaptively on the
    // same winner.
    let reluctant = with_escalation(Some(EscalationConfig {
        cost_sample: 0,
        cost_enumerate: 1,
        check_every: 1,
    }));
    let exact_best = exact_winner(&cold.result).expect("unique best");
    match &reluctant.result {
        SetResult::DeltaSettled { winner, .. } => assert_eq!(*winner, exact_best),
        SetResult::ExactFrozenSet { winner, .. } => assert_eq!(*winner, Some(exact_best)),
        other => panic!("a decisive set resolves, got {other}"),
    }
}

// ---------------------------------------------------------------------------
// O21 — ledger completeness from every produced result kind.
// ---------------------------------------------------------------------------

/// Reconstruct the full allocation — run → decision → epoch → edges →
/// optional ε tests — and check the exact rational sum against the scope
/// budget. Also checks the CE-A3 serialization contract.
fn audit_ledger(result: &SetResult, expect_eq_tests: bool) {
    let ledger = result.ledger();
    // Decision → edges: the all-pairs §5 allocation, reconstructed.
    let ordered_pairs = BigRational::from_integer(BigInt::from(ledger.m * (ledger.m - 1)));
    assert_eq!(
        &ledger.edge_alpha * &ordered_pairs,
        *ledger.plan.edges(),
        "per-edge alpha times m(m-1) is the edge sub-budget"
    );
    assert_eq!(
        ledger.edge_threshold,
        evidence::edge_threshold(ledger.m, ledger.plan.edges()),
        "T_edge is calculated, never tuned"
    );
    // Decision → optional equivalence tests.
    match (ledger.plan.equivalence(), &ledger.eq_alpha) {
        (Some(eq), Some(alpha)) => {
            assert!(expect_eq_tests);
            assert_eq!(
                alpha * BigRational::from_integer(BigInt::from(ledger.eq_tests)),
                eq.delta,
                "per-test alpha times test count is the equivalence sub-budget"
            );
        }
        (None, None) => assert!(!expect_eq_tests),
        _ => panic!("equivalence plan and per-test alpha travel together"),
    }
    // The exact rational sum of everything allocated fits the scope.
    let total = ledger.allocated_total();
    assert!(
        &total <= ledger.scope_budget(),
        "allocated risks sum within the declared scope budget"
    );
    assert!(total.is_positive());
    // Run → decision, when run provenance is declared (§6).
    if let Some((run, d)) = ledger.plan.run() {
        assert!(
            *ledger.plan.decision().delta() <= evidence::decision_delta(*d, run.delta()),
            "the decision budget fits its run allocation"
        );
    }
    // CE-A3: the serialization's prefix is the type tag, and the ledger
    // (scope, per-edge alpha, epoch, stream identity) travels with it.
    let serialized = result.to_string();
    assert!(serialized.starts_with(&format!("{}{{", result.tag())));
    assert!(serialized.contains("delta["));
    assert!(serialized.contains("edge_alpha="));
    assert!(serialized.contains("epoch="));
    assert!(serialized.contains("stream{"));
    assert!(serialized.contains("with_replacement=true"));
}

#[test]
fn ledger_completeness_reconstructs_the_full_allocation_within_scope() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().take(3).collect());
    let f = field();
    let run_budget = ScopedDelta::new("run:o21", q(1, 20));
    // DeltaSettled, under run provenance (decision event 3 of the run).
    let settled = evaluate_set(&SetSpec {
        root: &root,
        position: &position,
        candidates: &candidates,
        field: &f,
        plan: RiskPlan::strict(ScopedDelta::new("decision:o21-settled", q(1, 300)))
            .under_run(run_budget.clone(), 3),
        world_cap: 8192,
        batch: 16,
        escalation: None,
    });
    assert_eq!(settled.result.tag(), "DeltaSettled");
    audit_ledger(&settled.result, false);
    // Unresolved at a zero-world cap: the degenerate resource limit is
    // still never a settlement rule (CE-A3/A5).
    let unresolved = evaluate_set(&strict_spec(
        &root,
        &position,
        &candidates,
        &f,
        "decision:o21-unresolved",
        0,
    ));
    assert_eq!(unresolved.result.tag(), "Unresolved");
    audit_ledger(&unresolved.result, false);
    // ExactFrozenSet by escalation: risk allocated, closable unspent.
    let exact = evaluate_set_with_switch(
        &strict_spec(
            &root,
            &position,
            &candidates,
            &f,
            "decision:o21-exact",
            4096,
        ),
        17,
    );
    assert_eq!(exact.result.tag(), "ExactFrozenSet");
    audit_ledger(&exact.result, false);
    // EpsilonEquivalent, with the equivalence sub-budget in the ledger.
    let (a, b) = twins(&position);
    let twin_set = CandidateSet::new(vec![&a, &b]);
    let equivalent = evaluate_set(&SetSpec {
        root: &root,
        position: &position,
        candidates: &twin_set,
        field: &f,
        plan: RiskPlan::with_equivalence(
            ScopedDelta::new("decision:o21-equivalent", q(1, 100)),
            q(1, 200),
            EquivalencePlan {
                epsilon: q(1, 8),
                delta: q(1, 200),
                route: EquivalenceRoute::BoundedMean,
            },
        ),
        world_cap: 512,
        batch: 16,
        escalation: None,
    });
    assert_eq!(equivalent.result.tag(), "EpsilonEquivalent");
    audit_ledger(&equivalent.result, true);
}

// ---------------------------------------------------------------------------
// §8.5 — refinement vectors recompute exactly from evidence arithmetic.
// ---------------------------------------------------------------------------

#[test]
fn unresolved_refinement_vectors_recompute_exactly_from_evidence_arithmetic() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().take(2).collect());
    let f = field();
    let cap = 12u64;
    let spec = strict_spec(
        &root,
        &position,
        &candidates,
        &f,
        "decision:refinement",
        cap,
    );
    let evaluation = evaluate_set(&spec);
    let SetResult::Unresolved { refinements, .. } = &evaluation.result else {
        panic!("a 12-world cap on T_edge = 100 leaves the pair open");
    };
    assert_eq!(refinements.len(), 1);
    let refinement = &refinements[0];
    let (a, b, n) = (refinement.a, refinement.b, refinement.n);
    assert_eq!(n, cap, "both candidates stayed live for the whole stream");
    assert_eq!(refinement.n0, n - a - b);
    assert!(a + b > 0, "the descending/ascending pair is pivotal early");
    // Exact fields, recomputed through solver::evidence.
    let threshold = evidence::edge_threshold(2, spec.plan.edges());
    assert_eq!(refinement.threshold, threshold);
    assert_eq!(refinement.e_plus, evidence::pivotal_evidence(a, b));
    assert_eq!(refinement.e_minus, evidence::pivotal_evidence(b, a));
    assert_eq!(
        refinement.r_debt_plus,
        evidence::evidence_debt(&threshold, &refinement.e_plus)
    );
    assert_eq!(
        refinement.r_debt_minus,
        evidence::evidence_debt(&threshold, &refinement.e_minus)
    );
    assert_eq!(
        refinement.h_plus_min,
        evidence::h_plus_min(a, b, &threshold)
    );
    assert_eq!(
        refinement.h_minus_min,
        evidence::h_minus_min(a, b, &threshold)
    );
    // Estimate fields: exact rationals labeled estimates.
    let pivots = BigInt::from(a + b);
    assert_eq!(
        refinement.q_hat,
        Some(BigRational::new(pivots.clone(), BigInt::from(n)))
    );
    assert_eq!(
        refinement.tau_hat,
        Some(BigRational::new(
            BigInt::from(i128::from(a) - i128::from(b)),
            pivots
        ))
    );
    assert_eq!(
        refinement.g_hat,
        Some(BigRational::new(
            BigInt::from(i128::from(a) - i128::from(b)),
            BigInt::from(n)
        ))
    );
    let q_hat = refinement.q_hat.clone().expect("pivotal mass observed");
    assert_eq!(
        refinement.n_hat_plus,
        Some(BigRational::from_integer(BigInt::from(refinement.h_plus_min)) / &q_hat)
    );
    // §11.3 costs under the default unit weights: N_rem is exact and at
    // most the fiber, at least fiber minus the distinct sampled prefix.
    let fiber = BigRational::from_integer(BigInt::from(root.count()));
    let floor = BigRational::from_integer(BigInt::from(root.count() - u128::from(cap)));
    assert!(refinement.c_exact <= fiber);
    assert!(refinement.c_exact >= floor);
    assert!(refinement.c_sample_forecast.is_some());
}

// ---------------------------------------------------------------------------
// Candidate-set validation.
// ---------------------------------------------------------------------------

#[test]
#[should_panic(expected = "candidate PolicyIds are distinct")]
fn candidate_sets_reject_duplicate_policy_ids() {
    let r = receipt();
    let (_, position) = root_at(&r, SMALL_ROOT);
    let one = FrozenPolicy::new(freeze(&position, descending()));
    let same = FrozenPolicy::new(freeze(&position, descending()));
    CandidateSet::new(vec![&one, &same]);
}
