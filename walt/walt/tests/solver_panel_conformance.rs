//! Gates for the panel-response conformance audits (PANEL-A3/A5/A6,
//! `walt/CENSUS-RULINGS.md`, "The panel-response adjudication (2026-08-24)";
//! audit note `walt/audits/panel_response_conformance.md`).
//!
//! EXPLORATORY tier. These are mechanical assertions backing an audit of
//! shipped code against adopted rulings — session evidence about this tree,
//! promoting nothing. Two audit threads share the file:
//!
//! - **CE thread** (Claim-D repair + W7–W11): `solver::controller` /
//!   `solver::adaptive` preallocate all edge risk before any world is
//!   observed, read liveness per index (the O26 ambiguity's canonical
//!   side), resolve same-index crossings deterministically, and leave a
//!   complete pause state whose continuation is replay-consistent.
//! - **L2 thread** (τ coupling): `solver::exposure::coupled_replay`'s
//!   first-split fork is exactly the PANEL-A6 stopping time τ — equal
//!   public histories, non-focal actor, σ0(J) ≠ σ1(J), FIRST such t on
//!   the common prefix.
//!
//! The small root is the controller gates': hand 4, trick 6 of the frozen
//! `verify_player` receipt (fiber 90).

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::One;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{
    root_identity, CanonicalRoot, FixedPreference, RootPosition, SlicePolicy,
};
use walt::solver::controller::{
    epoch_identity, evaluate_set, CandidateSet, RiskPlan, SetEvaluation, SetResult, SetSpec,
};
use walt::solver::evidence::{self, ScopedDelta};
use walt::solver::exposure::coupled_replay;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

// ---------------------------------------------------------------------------
// Roots and candidates (the controller-gate fixtures of
// tests/solver_controller.rs, restated locally).
// ---------------------------------------------------------------------------

const SMALL_ROOT: (usize, usize, u128) = (4, 6, 90);

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

fn small_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [descending(), ascending(), stride(3, 1), stride(11, 7)]
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

/// The live set as a derived view of the elimination record.
fn live_of(evaluation: &SetEvaluation, m: usize) -> Vec<usize> {
    (0..m)
        .filter(|k| !evaluation.eliminations.iter().any(|e| e.candidate == *k))
        .collect()
}

// ---------------------------------------------------------------------------
// PANEL-A3 (Claim-D repair): edge risk is fully preallocated before any
// world is observed, and a mutated candidate set is a new epoch with a
// disjoint world stream — old evidence unreachable by construction.
// ---------------------------------------------------------------------------

/// The shipped controller's sound form is PREALLOCATION (§5 all-pairs):
/// every directed edge's α is fixed by `Controller::new` before world 0 is
/// folded, the strict plan's allocation sums exactly to the decision
/// budget, and no later code path can widen it (the pair vector never
/// grows; §5.2 one-at-a-time opening is not built).
#[test]
fn panel_a3_edge_risk_is_fully_preallocated_and_sums_to_the_declared_budget() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().collect());
    let f = field();
    let spec = strict_spec(&root, &position, &candidates, &f, "decision:a3", 32);
    let evaluation = evaluate_set(&spec);
    let ledger = evaluation.result.ledger();
    // All m(m-1) directed edges are allocated up front; under the strict
    // plan the preallocation exhausts the decision budget exactly — there
    // is no unallocated remainder a retrospective opening could claim.
    assert_eq!(ledger.m, 4);
    assert_eq!(
        &ledger.edge_alpha * BigRational::from_integer(BigInt::from(12)),
        q(1, 50),
        "twelve directed edges at edge_alpha preallocate the whole budget"
    );
    assert_eq!(ledger.allocated_total(), *ledger.scope_budget());
    // T_edge is the §5 calculated threshold of the preallocation.
    assert_eq!(
        ledger.edge_threshold,
        evidence::edge_threshold(4, &q(1, 50))
    );
}

/// §5.3: mutating the candidate set changes the epoch, and the epoch is
/// folded into every world-stream seed — so evidence gathered under the
/// old set is unreachable from the new one (nothing is reinterpreted,
/// which is what forecloses retrospective assignment across set changes).
#[test]
fn panel_a3_a_mutated_candidate_set_is_a_new_epoch_with_a_disjoint_stream() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let four = CandidateSet::new(pool.iter().collect());
    let three = CandidateSet::new(pool.iter().take(3).collect());
    let delta = ScopedDelta::new("decision:a3-epoch", q(1, 50));
    let root_id = root_identity(&root, &position);
    let epoch_four = epoch_identity(root_id, &four, &delta);
    let epoch_three = epoch_identity(root_id, &three, &delta);
    assert_ne!(epoch_four, epoch_three, "a set mutation is a new epoch");
    let diverged = (0..32).any(|i| {
        root.world_at(root_id, epoch_four.stream_epoch(), i)
            != root.world_at(root_id, epoch_three.stream_epoch(), i)
    });
    assert!(
        diverged,
        "the two epochs derive disjoint world streams, so old pair counts \
         are unreachable from the new epoch by construction"
    );
}

// ---------------------------------------------------------------------------
// W8/W9 (PANEL-A5): the O26 batch-boundary ambiguity fixture, ported from
// the panel-response verifier as the standing divergence witness. The
// naive batch-start-liveness reading diverges from canonical per-index
// liveness; the shipped `fold_world` reads liveness per index inside the
// fold, so the naive reading has no code path.
// ---------------------------------------------------------------------------

/// One pair's directed counts under a 3-candidate outcome table.
struct FixturePair {
    i: usize,
    j: usize,
    a: u64,
    b: u64,
    settled: Option<(usize, usize, u64)>,
}

fn fixture_pairs() -> Vec<FixturePair> {
    let mut pairs = Vec::new();
    for i in 0..3 {
        for j in (i + 1)..3 {
            pairs.push(FixturePair {
                i,
                j,
                a: 0,
                b: 0,
                settled: None,
            });
        }
    }
    pairs
}

fn update_pair(p: &mut FixturePair, row: &[u8; 3], at: u64, threshold: &BigRational) {
    let y = i8::try_from(row[p.i]).expect("0/1") - i8::try_from(row[p.j]).expect("0/1");
    if y > 0 {
        p.a += 1;
    } else if y < 0 {
        p.b += 1;
    }
    if y != 0 && p.settled.is_none() {
        if evidence::crossed(&evidence::pivotal_evidence(p.a, p.b), threshold) {
            p.settled = Some((p.i, p.j, at));
        } else if evidence::crossed(&evidence::pivotal_evidence(p.b, p.a), threshold) {
            p.settled = Some((p.j, p.i, at));
        }
    }
}

/// §19 canonical semantics, in the shipped `fold_world` order: per index,
/// every unordered pair with both endpoints live updates, then the
/// live-remover elimination runs to a fixed point.
fn canonical_fold(
    table: &[[u8; 3]],
    threshold: &BigRational,
) -> (Vec<usize>, Vec<(usize, usize, u64)>) {
    let mut live = [true; 3];
    let mut pairs = fixture_pairs();
    let mut edges: Vec<(usize, usize, u64)> = Vec::new();
    for (n, row) in table.iter().enumerate() {
        for p in &mut pairs {
            if !(live[p.i] && live[p.j]) {
                continue;
            }
            let before = p.settled;
            update_pair(p, row, n as u64, threshold);
            if before.is_none() {
                if let Some(edge) = p.settled {
                    edges.push(edge);
                }
            }
        }
        loop {
            let mut removed = false;
            'scan: for j in 0..3 {
                if !live[j] {
                    continue;
                }
                for e in &edges {
                    if e.1 == j && live[e.0] {
                        live[j] = false;
                        removed = true;
                        break 'scan;
                    }
                }
            }
            if !removed {
                break;
            }
        }
    }
    ((0..3).filter(|&k| live[k]).collect(), edges)
}

/// The plausible-but-unsound W5 reading the O26 fixture witnesses
/// against: liveness frozen at batch start for the whole batch, every
/// ORDERED direction's first crossing tracked, edges applied only at the
/// batch boundary.
fn naive_batch_start_fold(
    table: &[[u8; 3]],
    threshold: &BigRational,
) -> (Vec<usize>, Vec<(usize, usize, u64)>) {
    let ordered: [(usize, usize); 6] = [(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)];
    let mut counts = [[0u64; 3]; 3];
    let mut first: Vec<(usize, usize, u64)> = Vec::new();
    for (n, row) in table.iter().enumerate() {
        for &(i, j) in &ordered {
            if row[i] == 1 && row[j] == 0 {
                counts[i][j] += 1;
            }
        }
        for &(i, j) in &ordered {
            if !first.iter().any(|e| (e.0, e.1) == (i, j))
                && evidence::crossed(
                    &evidence::pivotal_evidence(counts[i][j], counts[j][i]),
                    threshold,
                )
            {
                first.push((i, j, n as u64));
            }
        }
    }
    let mut live = [true; 3];
    for e in &first {
        live[e.1] = false;
    }
    ((0..3).filter(|&k| live[k]).collect(), first)
}

/// The O26 ambiguity fixture (panel-response §18, verifier
/// `verify_walt_panel_response_v0_1.py`): five worlds, three candidates,
/// threshold 3/2. Canonically, candidate 1 dies at index 0; under the
/// batch-start reading its evidence keeps accumulating and crosses at
/// index 4 with E⁺₄,₁ = 19/10 — a canonically dead candidate settling
/// edges. The two readings' live sets diverge.
#[test]
fn w8_the_o26_ambiguity_fixture_diverges_between_canonical_and_batch_start_liveness() {
    let table: [[u8; 3]; 5] = [[1, 0, 1], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0]];
    let threshold = q(3, 2);
    // The fixture's two pinned evidence values.
    assert_eq!(evidence::pivotal_evidence(1, 0), q(3, 2));
    assert_eq!(evidence::pivotal_evidence(4, 1), q(19, 10));
    let (canonical_live, canonical_edges) = canonical_fold(&table, &threshold);
    assert_eq!(canonical_live, vec![0, 2]);
    assert_eq!(canonical_edges, vec![(0, 1, 0), (2, 1, 0)]);
    let (naive_live, naive_edges) = naive_batch_start_fold(&table, &threshold);
    // The dead candidate's evidence crosses at index 4 under the naive
    // reading — in both directions it participates in late edges.
    assert!(naive_edges.contains(&(1, 0, 4)));
    assert!(naive_edges.contains(&(1, 2, 4)));
    // The divergence witness: the live sets differ (here the naive
    // reading even empties the table).
    assert_ne!(canonical_live, naive_live);
    assert!(naive_live.is_empty());
}

/// W9 for the shipped controller path: batching is loop chunking only —
/// `fold_world` runs per index in stream order with liveness read at fold
/// time, so no speculative outcome can enter evidence ahead of its index.
/// The mechanical form is the existing V8 gate
/// (tests/solver_controller.rs) plus this determinism rerun: two
/// evaluations of one spec agree on every semantic artifact.
#[test]
fn w10_same_index_resolution_and_the_whole_record_are_deterministic_across_reruns() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().collect());
    let f = field();
    let run = || {
        let spec = strict_spec(&root, &position, &candidates, &f, "decision:w10", 64);
        evaluate_set(&spec)
    };
    let first = run();
    let second = run();
    assert_eq!(first.result, second.result);
    assert_eq!(first.edges, second.edges);
    assert_eq!(first.eliminations, second.eliminations);
    assert_eq!(first.pair_counts, second.pair_counts);
    assert_eq!(first.consumed, second.consumed);
}

/// W10's typed-inconsistency branch is vacuous in the shipped controller,
/// and this lemma is half of why: a directed crossing requires a STRICT
/// count majority in its direction, because `E⁺(a,b) ≤ 1` whenever
/// `a ≤ b` while every edge threshold `m(m-1)/δ` exceeds one. The other
/// half is structural (audit note §1c): pairs among simultaneously live
/// candidates fold the SAME common worlds, so directed count imbalances
/// around any candidate cycle telescope to zero pathwise — they cannot
/// all be strictly positive, so same-index settled-edge cycles cannot
/// form.
#[test]
fn w10_crossing_requires_strict_count_majority_so_settled_edge_cycles_cannot_form() {
    for a in 0u64..=60 {
        for b in a..=60 {
            assert!(
                evidence::pivotal_evidence(a, b) <= BigRational::one(),
                "E+({a},{b}) stays at or below one without a strict majority"
            );
        }
    }
    for m in 2u64..=6 {
        for delta in [q(99, 100), q(1, 2), q(1, 50)] {
            assert!(
                evidence::edge_threshold(m, &delta) > BigRational::one(),
                "every edge threshold exceeds one"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// W11 (PANEL-A5): the pause state is complete, and its continuation is
// replay-consistent under the counter-based stream (§17.1: world `i` is a
// pure function of (root_id, epoch, i), so resuming = re-evaluating at a
// higher cap, with an identical prefix by construction).
// ---------------------------------------------------------------------------

#[test]
fn w11_the_unresolved_pause_state_is_complete_and_resume_consistent() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let candidates = CandidateSet::new(pool.iter().collect());
    let f = field();
    // Cap 8 cannot settle any edge: the best case E+(8,0) = 511/9 sits
    // far below T_edge = 12·50 = 600, so the pause artifact is fully
    // populated (all pairs open, all candidates live).
    assert!(evidence::pivotal_evidence(8, 0) < evidence::edge_threshold(4, &q(1, 50)));
    let spec_a = strict_spec(&root, &position, &candidates, &f, "decision:w11", 8);
    let paused = evaluate_set(&spec_a);
    let SetResult::Unresolved {
        survivors,
        consumed,
        refinements,
        ledger,
    } = &paused.result
    else {
        panic!("cap 8 pauses unresolved, got {}", paused.result);
    };
    // The W11 fields, on the pause artifact itself: next canonical index,
    // live set, pair counts, first crossings, policy IDs, risk ledger,
    // epoch.
    assert_eq!(*consumed, 8, "the next canonical index is the cap");
    assert_eq!(*survivors, vec![0, 1, 2, 3], "the live set is complete");
    assert_eq!(paused.pair_counts.len(), 6, "every pair's counts persist");
    for p in &paused.pair_counts {
        assert_eq!(p.n, 8, "every pair folded the whole prefix");
    }
    assert!(paused.edges.is_empty(), "no first crossing below the cap");
    assert_eq!(refinements.len(), 6, "every open pair carries a vector");
    let ids = candidates.ids();
    for refinement in refinements {
        assert_eq!(refinement.policy_i, ids[refinement.i]);
        assert_eq!(refinement.policy_j, ids[refinement.j]);
        assert_eq!(refinement.threshold, ledger.edge_threshold);
    }
    assert_eq!(ledger.stream.epoch, ledger.epoch.stream_epoch());
    // Resume by replay: a higher-cap evaluation of the same spec is the
    // paused run's continuation — same worlds by the counter-based
    // stream, so its record extends the pause without rewriting it.
    let spec_b = strict_spec(&root, &position, &candidates, &f, "decision:w11", 24);
    let resumed = evaluate_set(&spec_b);
    assert!(
        resumed.edges.iter().all(|e| e.at >= 8),
        "no crossing is retro-dated into the paused prefix"
    );
    assert!(resumed.eliminations.iter().all(|e| e.at >= 8));
    let live_after = live_of(&resumed, 4);
    assert!(
        live_after.iter().all(|k| survivors.contains(k)),
        "the continuation's live set only narrows the paused one"
    );
    for (before, after) in paused.pair_counts.iter().zip(&resumed.pair_counts) {
        assert_eq!((before.i, before.j), (after.i, after.j));
        assert!(after.n >= before.n && after.a >= before.a && after.b >= before.b);
    }
}

// ---------------------------------------------------------------------------
// PANEL-A6 (L2 thread): the exposure walk's first-split fork is exactly
// the stopping time τ.
// ---------------------------------------------------------------------------

const FIELD_SMALL_HAND: usize = 4;
const FIELD_SMALL_TRICK: usize = 6;

fn field_small_root(r: &Receipt) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[FIELD_SMALL_HAND];
    assert_eq!(hand.id, FIELD_SMALL_HAND);
    let kernel = Kernel::from_receipt_trick(hand, FIELD_SMALL_TRICK).expect("a valid kernel");
    let position =
        RootPosition::from_receipt_trick(hand, FIELD_SMALL_TRICK).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn field0_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn field1_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn exposure_focal(position: &RootPosition, pinned: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
        solver_source: "walt-level1-continuation-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![2, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    })
}

/// τ's event is σ0(J) ≠ σ1(J) at a non-focal J on the common prefix — so
/// under σ0 = σ1 the event is empty and τ = ∞ on every world. A fork on
/// any world here would mean the walk's split event is WIDER than τ.
#[test]
fn panel_a6_identical_fields_never_split_so_the_event_is_not_wider_than_tau() {
    let r = receipt();
    let (root, position) = field_small_root(&r);
    let viewer = root.kernel().viewer();
    let sigma0 = FieldModel::new(field0_spec());
    let sigma0_again = FieldModel::new(field0_spec());
    let pinned = root.kernel().viewer_hand().iter().next().expect("a tile");
    let rho = exposure_focal(&position, pinned);
    for world in root.worlds() {
        let outcome = coupled_replay(&position, viewer, &world, &rho, &sigma0, &sigma0_again);
        assert!(
            !outcome.exposed(),
            "identical fields admit no σ0(J) ≠ σ1(J) state, so τ = ∞"
        );
        assert_eq!(outcome.u0, outcome.u1);
    }
}

/// τ is a symmetric function of the unordered field pair on the common
/// prefix: swapping σ0 and σ1 must yield the same stopping state (seat,
/// trick, ply, hand, common history) with the chosen tiles and terminals
/// swapped. A wider or narrower fork (e.g. one keyed to a particular
/// field's branch after divergence) would break this symmetry. The split
/// seat is asserted non-focal — the third τ conjunct.
#[test]
fn panel_a6_the_first_split_is_the_symmetric_non_focal_stopping_time() {
    let r = receipt();
    let (root, position) = field_small_root(&r);
    let viewer = root.kernel().viewer();
    let sigma0 = FieldModel::new(field0_spec());
    let sigma1 = FieldModel::new(field1_spec());
    let pinned = root.kernel().viewer_hand().iter().next().expect("a tile");
    let rho = exposure_focal(&position, pinned);
    let mut splits = 0u32;
    for world in root.worlds() {
        let forward = coupled_replay(&position, viewer, &world, &rho, &sigma0, &sigma1);
        let swapped = coupled_replay(&position, viewer, &world, &rho, &sigma1, &sigma0);
        assert_eq!(forward.exposed(), swapped.exposed(), "D is symmetric");
        assert_eq!(forward.u0, swapped.u1);
        assert_eq!(forward.u1, swapped.u0);
        match (&forward.split, &swapped.split) {
            (None, None) => {}
            (Some(f), Some(s)) => {
                splits += 1;
                assert_ne!(f.seat, viewer, "τ requires a non-focal actor");
                assert_ne!(f.tile0, f.tile1, "τ requires σ0(J) ≠ σ1(J)");
                assert_eq!((f.seat, f.trick, f.ply), (s.seat, s.trick, s.ply));
                assert_eq!(f.hand, s.hand);
                assert_eq!(f.history, s.history, "one common prefix, either order");
                assert_eq!(f.tile0, s.tile1);
                assert_eq!(f.tile1, s.tile0);
            }
            _ => unreachable!("exposure symmetry already asserted"),
        }
    }
    // Pinned so the gate is non-vacuous on this root (regression
    // evidence, exploratory tier): the σ0/σ1 pair does split here.
    assert!(splits > 0, "the fixture exercises the τ event");
}
