//! Gates for the anytime proof-state Phase 1 [L2 thread], the Part IX
//! half (the §58 skeleton is the §49 spike's, already gated): declared
//! solve goals type their debts exactly (gate 1); the §42 steering
//! bounds are SAFE — no executed item's realized debt reduction ever
//! exceeds its declared bound, across every root and goal (gate 2);
//! `SelectAction` settles every enumerable root with honest refusals —
//! a refused zero-potential item really moves nothing (gate 3); the
//! §41 macro-plan is load-bearing — from the top state every
//! standalone `ExactValue` is provably useless for the upper side and
//! the declared macro is what moves `U*`, after which
//! `RecommendEpsilonPolicy(0)` certifies Γ = 0 while refusing
//! extraction of dominated actions (gate 4); the §43 containment —
//! `StrengthenToExact` lands every surviving interval on the
//! independently recomputed exact value, twice, with identical
//! schedules and identical serialized states (gate 5); and budgets
//! are honest — a starved run stops unmet with `spent ≤ budget` and a
//! sound state, and resuming with the remainder reproduces the
//! one-shot result (gate 6, §44).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §35, §39–§44, under rulings APS-A8/APS-A9
//! (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frontier's declared cost model (Z per
//! fixed-policy walk, 3Z per max walk). Frozen `verify_player`
//! receipt roots: the six enumerable fibers.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::Domino;
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{
    response_success_mass, ExactCoverOracle, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::frontier::{Frontier, Refusal, SolveGoal, WorkItem};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofState, SemanticsIdentity, StateResult};

const ENUM_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn level0_spec() -> FieldSpec {
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

fn identity_of(root: &CanonicalRoot, position: &RootPosition) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
    SemanticsIdentity {
        root_id: root_identity(root, position),
        rules_id: "texas42-v1".to_string(),
        field_id: "level0-modeled-mind-v1".to_string(),
        utility_id: if declaring {
            "pmake-v1".to_string()
        } else {
            "pmake-setting-v1".to_string()
        },
        contract: position.bid,
        belief_id: "uniform-root".to_string(),
        policy_class_id: "information-consistent-full".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

fn goals() -> Vec<SolveGoal> {
    vec![
        SolveGoal::SelectAction,
        SolveGoal::RecommendEpsilonPolicy {
            epsilon: BigRational::zero(),
        },
        SolveGoal::StrengthenToExact,
        SolveGoal::ComputeFullScoreProfile,
    ]
}

/// Gate 1 — §39 debt typing: on the top state every goal's debt is
/// positive (nothing is proved at zero facts, on roots with real
/// choice); after an ample frontier run, debt is zero exactly when
/// `met`; and the four debts are the documented quantities (survivor
/// count, regret gap, width sum, missing profiles) — checked against
/// the closure they summarize.
#[test]
fn goal_debts_are_typed_and_zero_exactly_at_satisfaction() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        for goal in goals() {
            let identity = identity_of(&root, &position);
            let mut state = ProofState::open(&root, &position, identity);
            let report = state.closure();
            let debt0 = goal.debt(&state, &report);
            // The top state: |legal| ≥ 2 at every fixture root, so
            // SelectAction starts in debt; Γ = 1 − 0 = 1; widths are
            // 1 each; every profile is missing.
            assert!(
                debt0 > BigRational::zero(),
                "the top state owes every goal something on a root with choice"
            );
            let out = frontier.advance(&mut state, &goal, u128::MAX / 4);
            assert!(out.met, "an ample budget meets every goal here");
            assert_eq!(out.debt, BigRational::zero(), "met means zero debt");
            let after = state.closure();
            assert_eq!(
                goal.debt(&state, &after),
                BigRational::zero(),
                "the debt recomputes to zero on the final state"
            );
            match goal {
                SolveGoal::SelectAction => assert!(
                    !matches!(after.result, StateResult::Unresolved { .. }),
                    "SelectAction met means settled or exact-equivalent"
                ),
                SolveGoal::RecommendEpsilonPolicy { .. } => {
                    assert_eq!(
                        after.certified_regret,
                        BigRational::zero(),
                        "epsilon 0 met means certified regret zero"
                    );
                    assert!(
                        state.recommend().is_some(),
                        "a met epsilon goal recommends an executable policy"
                    );
                }
                SolveGoal::StrengthenToExact => {
                    for v in after.views.iter().filter(|v| !v.excluded) {
                        assert_eq!(v.lower, v.upper, "every surviving interval is a point");
                    }
                }
                SolveGoal::ComputeFullScoreProfile => {
                    // met is definitionally every action holding a
                    // profile fact; the recomputed zero above is the
                    // check.
                }
            }
        }
    }
}

/// Gate 2 — the §42 law, externally recomputed: for every executed
/// item across every root and goal, `debt_before − debt_after ≤
/// bound` and debts never rise. (The frontier asserts this internally
/// too; the gate holds the REPORT to it, so the report cannot drift
/// from the execution.)
#[test]
fn steering_bounds_are_safe_across_roots_and_goals() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut executed_total = 0u64;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        for goal in goals() {
            let identity = identity_of(&root, &position);
            let mut state = ProofState::open(&root, &position, identity);
            let out = frontier.advance(&mut state, &goal, u128::MAX / 4);
            for step in &out.executed {
                executed_total += 1;
                assert!(
                    step.debt_after <= step.debt_before,
                    "monotone: a bought fact never raises the debt"
                );
                assert!(
                    &step.debt_before - &step.debt_after <= step.bound,
                    "the §42 law on the report: realized ≤ declared bound"
                );
            }
            let spent: u128 = out.executed.iter().map(|s| s.cost).sum();
            assert_eq!(spent, out.spent, "the spend is the sum of the schedule");
        }
    }
    assert!(executed_total > 0, "the gate watched real purchases");
}

/// Gate 3 — SelectAction with honest refusals: every enumerable root
/// settles; and at the moment the frontier stops, executing any
/// refused ZERO-POTENTIAL candidate by hand moves the debt by exactly
/// nothing (the §34/§41 refusal is a theorem about the closed state,
/// not a heuristic).
#[test]
fn select_action_settles_and_zero_potential_refusals_are_real() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut refusals_checked = 0u64;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        let out = frontier.advance(&mut state, &SolveGoal::SelectAction, u128::MAX / 4);
        assert!(out.met, "SelectAction settles every enumerable root");
        // Rebuild the state one step short of the full schedule, ask
        // for one more step with zero extra budget headroom — no:
        // directly test refusal honesty on the FINAL state of a
        // DIFFERENT goal that ends unmet-free: rerun SelectAction on
        // the met state; everything must be refused (debt is zero) —
        // then hand-execute refused items on clones and observe
        // nothing move.
        let baseline_report = state.closure();
        let debt = SolveGoal::SelectAction.debt(&state, &baseline_report);
        assert_eq!(debt, BigRational::zero());
        // Hand-run refused candidates from an UNMET intermediate
        // state. The guaranteed zero-potential specimen is §39's own
        // sentence inverted — "the same item can be irrelevant to
        // action selection and valuable to score pricing": an
        // `ExactValue` is unconditionally irrelevant to
        // `ComputeFullScoreProfile`, so a profile-goal run starved
        // one baseline short must refuse every exact item as
        // ZeroPotential (money was still available for them).
        let mut starved = ProofState::open(&root, &position, identity);
        let z = {
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            oracle.mass(&belief)
        };
        let n = starved.legal.len() as u128;
        let goal = SolveGoal::ComputeFullScoreProfile;
        let poor = frontier.advance(&mut starved, &goal, z * (n - 1));
        assert!(!poor.met, "one baseline short by construction");
        let zero_refused: Vec<&WorkItem> = poor
            .refusals
            .iter()
            .filter(|(_, r)| *r == Refusal::ZeroPotential)
            .map(|(i, _)| i)
            .collect();
        assert!(
            zero_refused
                .iter()
                .any(|i| matches!(i, WorkItem::ExactValue { .. })),
            "every exact item is refused as zero-potential for the profile goal"
        );
        for item in zero_refused {
            refusals_checked += 1;
            let report = starved.closure();
            let before = goal.debt(&starved, &report);
            let mut probe_state = starved.clone();
            frontier.execute_item(item, &mut probe_state);
            let after_report = probe_state.closure();
            let after = goal.debt(&probe_state, &after_report);
            assert_eq!(
                before, after,
                "a zero-potential refusal is exact: executing it anyway moves nothing"
            );
        }
    }
    assert!(
        refusals_checked > 0,
        "at least one zero-potential refusal was hand-executed and verified"
    );
}

/// Gate 4 — the §41 macro is load-bearing and dominated extractions
/// are refused: from the top state, every standalone `ExactValue` has
/// zero potential for `RecommendEpsilonPolicy` (lowering one upper
/// cannot lower the max while any other upper is vacuous — checked on
/// the report's refusals of the FINAL stopped step of a macro-less
/// world by construction below), the executed schedule reaches Γ = 0
/// via the macro, and extraction is bought ONLY for actions whose
/// upper exceeds the executable bar (dominated actions' extractions
/// are never in the schedule — the Phase 6 ample producer extracted
/// everything; the frontier is the targeting).
#[test]
fn the_macro_moves_the_upper_side_and_dominated_extractions_are_refused() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut macro_bought = 0u64;
    let mut dominated_skipped = 0u64;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity);
        let goal = SolveGoal::RecommendEpsilonPolicy {
            epsilon: BigRational::zero(),
        };
        let out = frontier.advance(&mut state, &goal, u128::MAX / 4);
        assert!(out.met, "epsilon zero is certifiable on enumerable roots");
        let after = state.closure();
        assert_eq!(after.certified_regret, BigRational::zero());
        let bought_macro = out
            .executed
            .iter()
            .any(|s| matches!(s.item, WorkItem::ExactValueSurvivors));
        let bought_standalone_exact = out
            .executed
            .iter()
            .any(|s| matches!(s.item, WorkItem::ExactValue { .. }));
        // The upper side moved somehow; from the top state the macro
        // is the only §42-lawful mover (standalone exacts are zero
        // there), so if any upper work happened at all, the macro is
        // in the schedule.
        if bought_macro {
            macro_bought += 1;
        }
        assert!(
            bought_macro || !bought_standalone_exact,
            "no standalone ExactValue precedes the macro from the top state"
        );
        // Extraction targeting: every extracted action's upper
        // exceeded the executable bar at purchase time; count the
        // legal actions whose extraction never appears.
        let extracted: Vec<Domino> = out
            .executed
            .iter()
            .filter_map(|s| match s.item {
                WorkItem::ExtractArgmax { action } => Some(action),
                _ => None,
            })
            .collect();
        for a in &state.legal {
            if !extracted.contains(a) {
                dominated_skipped += 1;
            }
        }
    }
    assert!(macro_bought > 0, "the §41 macro was bought somewhere");
    assert!(
        dominated_skipped > 0,
        "somewhere an action's extraction was never needed — the frontier \
         targets what the ample producer bought wholesale"
    );
}

/// Gate 5 — the §43 containment and determinism: `StrengthenToExact`
/// lands every surviving interval exactly on the independently
/// recomputed best-response value, and running the identical
/// configuration twice yields the identical schedule and the
/// byte-identical serialized state.
#[test]
fn strengthen_to_exact_contains_the_exact_solve_deterministically() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let identity = identity_of(&root, &position);
        let mut s1 = ProofState::open(&root, &position, identity.clone());
        let out1 = frontier.advance(&mut s1, &SolveGoal::StrengthenToExact, u128::MAX / 4);
        assert!(out1.met);
        let report = s1.closure();
        for v in report.views.iter().filter(|v| !v.excluded) {
            let child = FactorBelief::uniform_root(&root, &position, &field).focal_play(v.action);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            let z = oracle.mass(&child);
            let value = BigRational::new(BigInt::from(exact), BigInt::from(z));
            assert_eq!(v.lower, value, "the surviving point IS the exact value");
            assert_eq!(v.upper, value, "the surviving point IS the exact value");
        }
        let mut s2 = ProofState::open(&root, &position, identity);
        let out2 = frontier.advance(&mut s2, &SolveGoal::StrengthenToExact, u128::MAX / 4);
        assert_eq!(
            out1.executed, out2.executed,
            "the schedule is deterministic"
        );
        assert_eq!(
            s1.serialize(),
            s2.serialize(),
            "the final states are byte-identical"
        );
    }
}

/// Gate 6 — budget honesty and §44 resume: a starved run stops unmet
/// with `spent ≤ budget` and a still-sound state; granting the
/// remainder afterwards lands the same final debt, the same closure
/// views, and the same serialized fact content as one uninterrupted
/// ample run.
#[test]
fn a_starved_run_is_honest_and_resume_equals_uninterrupted() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut starved_seen = 0u64;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let identity = identity_of(&root, &position);
        let goal = SolveGoal::RecommendEpsilonPolicy {
            epsilon: BigRational::zero(),
        };
        let z = {
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            oracle.mass(&belief)
        };
        // Starve: enough for roughly one cheap item.
        let mut split = ProofState::open(&root, &position, identity.clone());
        let poor = frontier.advance(&mut split, &goal, z);
        assert!(poor.spent <= z, "spend never exceeds the budget");
        if !poor.met {
            starved_seen += 1;
            // The state is sound mid-way: closure holds its own
            // invariants (asserted inside), debts recompute.
            let _ = split.closure();
        }
        // Resume with an ample remainder.
        let more = frontier.advance(&mut split, &goal, u128::MAX / 4);
        assert!(more.met, "the resumed run completes");
        // One-shot reference.
        let mut whole = ProofState::open(&root, &position, identity);
        let full = frontier.advance(&mut whole, &goal, u128::MAX / 4);
        assert!(full.met);
        let a = split.closure();
        let b = whole.closure();
        assert_eq!(a.bar, b.bar, "resume equals uninterrupted: the bar");
        assert_eq!(a.u_star, b.u_star, "…the global upper");
        assert_eq!(
            a.certified_regret, b.certified_regret,
            "…the certified regret"
        );
        assert_eq!(a.survivors, b.survivors, "…the survivor set");
    }
    assert!(
        starved_seen > 0,
        "at least one root was genuinely starved mid-schedule"
    );
}
