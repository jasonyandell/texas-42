//! Gates for the two surgical levers in the acting hot path (CE thread):
//!
//! 1. `solver::act` evaluates its SetSpec under a CACHED `FieldModel`
//!    (`FieldKind::Level0`) instead of the bare `Level0Field`. The
//!    `FieldModel` delegates every modeled choice to the one bare-field
//!    authority, so the swap is value-identical by construction; the A/B
//!    gates here run the same controller evaluation under both fields and
//!    assert the complete `SetEvaluation` is identical.
//! 2. `replay_viewer_success` stops at the first trick boundary where the
//!    pmake indicator is decided for every continuation (the indicator is
//!    monotone — points only accumulate). The gate compares the truncated
//!    replay against a test-local FULL replay (the pre-cutoff loop, kept
//!    here as a reference implementation, never library source) on every
//!    world of two complete fibers, and re-derives the pinned wins
//!    vectors [78, 34, 34] and [1118, 654, 563, 556] from both variants.
//!
//! Everything here is regression evidence at exploratory tier; nothing is
//! promoted, and no strength claim is made or implied.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::{legal_plays, Trick};
use walt::rules::{Domino, DominoSet, Seat};
use walt::solver::act::act_field_spec;
use walt::solver::adaptive::{
    decided_success, replay_viewer_success, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::controller::{
    evaluate_set, exact_frozen_set, CandidateSet, RiskPlan, SetEvaluation, SetSpec,
};
use walt::solver::evidence::ScopedDelta;
use walt::solver::field::FieldModel;
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, Level0Field, TieRule,
};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// (hand, trick, pinned exact fiber) — the two solver_controller roots.
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

/// The first three of the `solver_controller` small pool — pinned exact
/// values 78, 34, 34 of 90 under the lowest-first field.
fn small_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [descending(), ascending(), stride(3, 1)]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

/// The `solver_controller` second pool — pinned exact values 1118, 654,
/// 563, 556 of 1120 under the lowest-first field.
fn second_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [ascending(), stride(5, 2), stride(13, 0), descending()]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

fn assert_evaluations_identical(a: &SetEvaluation, b: &SetEvaluation, label: &str) {
    assert_eq!(a.result, b.result, "{label}: result");
    assert_eq!(a.edges, b.edges, "{label}: settled edges");
    assert_eq!(a.eliminations, b.eliminations, "{label}: eliminations");
    assert_eq!(a.pair_counts, b.pair_counts, "{label}: pair counts");
    assert_eq!(a.consumed, b.consumed, "{label}: consumed worlds");
    assert_eq!(a.escalation, b.escalation, "{label}: escalation report");
}

/// The lowest-legal-tile fixed field the `solver_controller` pins run
/// under (a test-local equivalent of its `FixedPreference` fixture).
struct FixedLowest;

impl SlicePolicy for FixedLowest {
    fn id(&self) -> &str {
        "field:lowest-first"
    }

    fn choose(
        &self,
        _decl: walt::rules::Decl,
        _hand: DominoSet,
        legal: DominoSet,
        _record: &PublicRecord<'_>,
    ) -> Domino {
        legal
            .iter()
            .next()
            .expect("a seat to move holds a legal tile")
    }
}

// ---------------------------------------------------------------------------
// Lever 1 — the cached field is choice-identical to the bare field.
// ---------------------------------------------------------------------------

/// The exact endpoint under the bare `Level0Field` and under act's
/// declared cached `FieldModel` produce the identical complete
/// `SetEvaluation` on both declared roots. The cached run also shows the
/// cache genuinely engaged.
#[test]
fn the_cached_field_model_reproduces_the_bare_field_exact_evaluation_on_both_roots() {
    let r = receipt();
    for (spec, pool_of, scope) in [
        (
            SMALL_ROOT,
            small_pool as fn(&RootPosition) -> Vec<FrozenPolicy>,
            "decision:field-cache-exact-small",
        ),
        (
            SECOND_ROOT,
            second_pool,
            "decision:field-cache-exact-second",
        ),
    ] {
        let (root, position) = root_at(&r, spec);
        let pool = pool_of(&position);
        let candidates = CandidateSet::new(pool.iter().collect());
        let bare = Level0Field::new(2);
        let cached = FieldModel::new(act_field_spec(2));
        let with_field = |field: &dyn SlicePolicy| {
            exact_frozen_set(&SetSpec {
                root: &root,
                position: &position,
                candidates: &candidates,
                field,
                plan: RiskPlan::strict(ScopedDelta::new(scope, q(1, 50))),
                world_cap: 0,
                batch: 16,
                escalation: None,
            })
        };
        let a = with_field(&bare);
        let b = with_field(&cached);
        assert_evaluations_identical(&a, &b, scope);
        assert!(
            cached.cache_len() > 0,
            "{scope}: the field action cache materialized states"
        );
    }
}

/// The sampled endpoint (honest cap, fresh evidence stream) is likewise
/// identical under both fields: same worlds (the epoch never folds the
/// field), same replay outcomes, same complete record.
#[test]
fn the_cached_field_model_reproduces_the_bare_field_sampled_evaluation() {
    let r = receipt();
    let (root, position) = root_at(&r, SECOND_ROOT);
    let pool = second_pool(&position);
    let candidates = CandidateSet::new(pool.iter().collect());
    let bare = Level0Field::new(2);
    let cached = FieldModel::new(act_field_spec(2));
    let with_field = |field: &dyn SlicePolicy| {
        evaluate_set(&SetSpec {
            root: &root,
            position: &position,
            candidates: &candidates,
            field,
            plan: RiskPlan::strict(ScopedDelta::new("decision:field-cache-sampled", q(1, 50))),
            world_cap: 64,
            batch: 8,
            escalation: None,
        })
    };
    let a = with_field(&bare);
    let b = with_field(&cached);
    assert_evaluations_identical(&a, &b, "decision:field-cache-sampled");
}

// ---------------------------------------------------------------------------
// Lever 2 — the decided cutoff is value-identical to the full replay.
// ---------------------------------------------------------------------------

/// The pre-cutoff replay loop, kept test-side as the reference: every
/// world plays to full terminal, and the make indicator is read off the
/// terminal banked totals alone.
fn replay_full(
    position: &RootPosition,
    viewer: Seat,
    world: &World,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
) -> bool {
    let mut hands = world.hands();
    let mut leader = position.leader;
    let mut plays = position.trick_plays.clone();
    let mut banked = position.banked;
    let mut history: Vec<Domino> = Vec::new();
    while hands.iter().any(|h| !h.is_empty()) {
        let seat = leader.plus(plays.len());
        let led = plays.first().map(|d| position.decl.led_context(*d));
        let hand = hands[seat.index()];
        let legal = legal_plays(position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = PublicRecord {
            leader,
            trick_plays: &plays,
            banked,
            root: position,
            history: &history,
        };
        let policy = if seat == viewer { focal } else { field };
        let tile = policy.choose(position.decl, hand, legal, &record);
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        assert!(hands[seat.index()].remove(tile), "the chosen tile is held");
        plays.push(tile);
        history.push(tile);
        if plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| plays[i]);
            let trick = Trick::new(leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            banked[winner.team().index()] += trick.points();
            leader = winner;
            plays.clear();
        }
    }
    assert!(plays.is_empty(), "a hand ends on a trick boundary");
    let made = banked[position.declaring_team.index()] >= position.bid;
    if viewer.team() == position.declaring_team {
        made
    } else {
        !made
    }
}

/// Per-world equivalence over both complete fibers: fresh policy
/// instances per variant (so neither warms the other's cache), every
/// world compared, and the wins totals re-derive the pinned vectors of
/// `solver_controller` from BOTH variants.
#[test]
fn the_decided_cutoff_matches_the_full_replay_on_every_world_of_both_fibers() {
    let r = receipt();
    for (spec, pool_of, pins) in [
        (
            SMALL_ROOT,
            small_pool as fn(&RootPosition) -> Vec<FrozenPolicy>,
            vec![78u64, 34, 34],
        ),
        (SECOND_ROOT, second_pool, vec![1118u64, 654, 563, 556]),
    ] {
        let (root, position) = root_at(&r, spec);
        let viewer = root.kernel().viewer();
        // Fresh instances per variant: FrozenPolicy carries an
        // insert-only cache, and the gate must not let one variant
        // answer from states the other materialized.
        let pool_full = pool_of(&position);
        let pool_cut = pool_of(&position);
        let mut wins_full = vec![0u64; pool_full.len()];
        let mut wins_cut = vec![0u64; pool_cut.len()];
        for world in root.worlds() {
            for k in 0..pool_full.len() {
                let full = replay_full(&position, viewer, &world, &pool_full[k], &FixedLowest);
                let cut =
                    replay_viewer_success(&position, viewer, &world, &pool_cut[k], &FixedLowest);
                assert_eq!(
                    full, cut,
                    "hand {} trick {} candidate {k}: cutoff diverges from full replay",
                    spec.0, spec.1
                );
                if full {
                    wins_full[k] += 1;
                }
                if cut {
                    wins_cut[k] += 1;
                }
            }
        }
        assert_eq!(wins_full, pins, "the full replay re-derives the pins");
        assert_eq!(wins_cut, pins, "the cutoff replay re-derives the pins");
    }
}

/// The truncated replay under a MODELED field too: every world of the
/// small fiber under the level-0 field, cutoff versus full, with fresh
/// instances per variant.
#[test]
fn the_decided_cutoff_matches_the_full_replay_under_the_level0_field() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let viewer = root.kernel().viewer();
    let pool_full = small_pool(&position);
    let pool_cut = small_pool(&position);
    let field_full = Level0Field::new(2);
    let field_cut = Level0Field::new(2);
    for world in root.worlds() {
        for k in 0..pool_full.len() {
            let full = replay_full(&position, viewer, &world, &pool_full[k], &field_full);
            let cut = replay_viewer_success(&position, viewer, &world, &pool_cut[k], &field_cut);
            assert_eq!(
                full, cut,
                "candidate {k}: cutoff diverges under the level-0 field"
            );
        }
    }
}

/// Late roots (trick 7) are decided at or near the root: both variants
/// agree there too, and an already-decided root's truncated outcome
/// matches the played-out terminal by monotonicity.
#[test]
fn the_cutoff_agrees_with_the_full_replay_on_late_roots() {
    let r = receipt();
    for hand_no in 0..r.hands.len() {
        let hand = &r.hands[hand_no];
        let Ok(kernel) = Kernel::from_receipt_trick(hand, 7) else {
            continue;
        };
        let position = RootPosition::from_receipt_trick(hand, 7).expect("a valid position");
        let root = CanonicalRoot::new(kernel);
        let viewer = root.kernel().viewer();
        let pool_full = small_pool(&position);
        let pool_cut = small_pool(&position);
        for world in root.worlds() {
            for k in 0..pool_full.len() {
                let full = replay_full(&position, viewer, &world, &pool_full[k], &FixedLowest);
                let cut =
                    replay_viewer_success(&position, viewer, &world, &pool_cut[k], &FixedLowest);
                assert_eq!(full, cut, "hand {hand_no} trick 7 candidate {k}");
            }
        }
        if let Some(decided) = decided_success(&position, viewer, position.banked, false) {
            let world = root.worlds().next().expect("a nonempty fiber");
            assert_eq!(
                decided,
                replay_full(&position, viewer, &world, &pool_full[0], &FixedLowest),
                "hand {hand_no}: a decided root's outcome is every continuation's outcome"
            );
        }
    }
}
