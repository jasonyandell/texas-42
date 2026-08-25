//! Gates for `solver::bundle` — the bundled world evaluator against the
//! enumerated per-world oracle (`solver::calibrate::exact_set_outcomes`),
//! ELEMENT-WISE: `outcomes[k][w]` must agree cell by cell, which is
//! strictly stronger than the wins totals the exact escalation route
//! consumes. Wins totals are additionally asserted against the standing
//! `solver_controller` pins ([78, 34, 34] on the fiber-90 root;
//! [1118, 654, 563, 556] on the fiber-1120 root).
//!
//! Agreements here are REGRESSION EVIDENCE at exploratory tier (parent
//! `walt/math/calculated_evidence_v0.1.md` §19 V4). Nothing is promoted.

mod common;

use common::receipt;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::bundle::{bundled_set_outcomes, bundled_set_outcomes_declared};
use walt::solver::calibrate::exact_set_outcomes;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

// ---------------------------------------------------------------------------
// Roots and candidates (the solver_controller fixtures, restated).
// ---------------------------------------------------------------------------

/// (hand, trick, pinned exact fiber) for the three receipt roots: the
/// controller's two pinned roots plus a voids-constrained root of a
/// different shape (hand 10 trick 5: 700 of 1680 unconstrained).
const SMALL_ROOT: (usize, usize, u128) = (4, 6, 90);
const SECOND_ROOT: (usize, usize, u128) = (11, 5, 1120);
const THIRD_ROOT: (usize, usize, u128) = (10, 5, 700);

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

/// The small root's m=3 candidate prefix of the controller's pool
/// (descending worth 78/90, then two worth 34/90).
fn small_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [descending(), ascending(), stride(3, 1)]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

/// The second root's pinned m=4 pool (1118, 654, 563, 556 of 1120).
fn second_pool(position: &RootPosition) -> Vec<FrozenPolicy> {
    [ascending(), stride(5, 2), stride(13, 0), descending()]
        .into_iter()
        .map(|order| FrozenPolicy::new(freeze(position, order)))
        .collect()
}

fn field() -> FixedPreference {
    FixedPreference::lowest_first("field:lowest-first")
}

fn as_dyn(pool: &[FrozenPolicy]) -> Vec<&dyn SlicePolicy> {
    pool.iter().map(|p| p as &dyn SlicePolicy).collect()
}

/// Element-wise equality against the oracle plus the attribution and
/// wins-consistency checks shared by every gate.
fn assert_element_wise(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[&dyn SlicePolicy],
    field: &dyn SlicePolicy,
) -> Vec<u128> {
    let oracle = exact_set_outcomes(root, position, candidates, field);
    let bundled = bundled_set_outcomes(root, position, candidates, field);
    assert_eq!(
        bundled.outcomes(),
        &oracle[..],
        "bundled outcomes match the enumerated oracle element-wise"
    );
    let fiber = u64::try_from(root.count()).expect("an enumerable fiber");
    let m = u64::try_from(candidates.len()).expect("fits");
    for (k, row) in oracle.iter().enumerate() {
        assert_eq!(
            u128::try_from(row.len()).expect("fits"),
            root.count(),
            "each candidate's vector covers the whole fiber"
        );
        let wins = u128::try_from(row.iter().filter(|u| **u).count()).expect("fits");
        assert_eq!(bundled.wins(k), wins, "wins derive from the attribution");
    }
    assert_eq!(
        bundled.early_settled() + bundled.terminal_settled(),
        m * fiber,
        "every candidate-world cell is attributed exactly once"
    );
    (0..candidates.len()).map(|k| bundled.wins(k)).collect()
}

// ---------------------------------------------------------------------------
// Element-wise equivalence on the three receipt roots.
// ---------------------------------------------------------------------------

#[test]
fn bundled_outcomes_match_the_oracle_element_wise_on_the_fiber_90_root() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let f = field();
    let wins = assert_element_wise(&root, &position, &as_dyn(&pool), &f);
    assert_eq!(wins, vec![78, 34, 34], "the pinned small-root values");
}

#[test]
fn bundled_outcomes_match_the_oracle_element_wise_on_the_fiber_1120_root() {
    let r = receipt();
    let (root, position) = root_at(&r, SECOND_ROOT);
    let pool = second_pool(&position);
    let f = field();
    let wins = assert_element_wise(&root, &position, &as_dyn(&pool), &f);
    assert_eq!(
        wins,
        vec![1118, 654, 563, 556],
        "the pinned second-root values"
    );
}

#[test]
fn bundled_outcomes_match_the_oracle_element_wise_on_a_voids_constrained_root() {
    let r = receipt();
    let (root, position) = root_at(&r, THIRD_ROOT);
    let pool = small_pool(&position);
    let f = field();
    // No standing pin for this root: the oracle itself is the authority,
    // and the element-wise assertion inside carries the gate.
    assert_element_wise(&root, &position, &as_dyn(&pool), &f);
}

// ---------------------------------------------------------------------------
// The realistic field configuration: a cached FieldModel.
// ---------------------------------------------------------------------------

/// One cached level-0 field model (the cheap declared configuration the
/// ordering bench also uses). Shared across both routes: its action cache
/// is insert-only and every entry is a pure function of its key (O29), so
/// warmth cannot change any chosen action.
fn level0_field() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

#[test]
fn bundled_outcomes_match_the_oracle_under_a_cached_field_model() {
    let r = receipt();
    let (root, position) = root_at(&r, SMALL_ROOT);
    let pool = small_pool(&position);
    let f = level0_field();
    assert_element_wise(&root, &position, &as_dyn(&pool), &f);
}

// ---------------------------------------------------------------------------
// Focal purity (the O22 consequence): a world list whose focal hand is
// not the root's viewer hand cannot enter a bundle.
// ---------------------------------------------------------------------------

#[test]
#[should_panic(expected = "the focal hand is constant across the world set")]
fn a_world_list_with_a_different_focal_hand_is_rejected_at_setup() {
    let r = receipt();
    let (small, _) = root_at(&r, SMALL_ROOT);
    let (second, second_position) = root_at(&r, SECOND_ROOT);
    let pool = second_pool(&second_position);
    let candidates = as_dyn(&pool);
    let f = field();
    // Worlds enumerated from the SMALL root carry a different viewer
    // hand than the second root's kernel: the setup assertion fires
    // before any policy is consulted.
    let foreign: Vec<_> = small.worlds().collect();
    bundled_set_outcomes_declared(&second, &second_position, &candidates, &f, &foreign);
}
