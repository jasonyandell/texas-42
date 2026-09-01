//! Gates for the anytime proof-state Phase 4 [L2 thread] — the §61
//! score-aware residual Bellman: every intermediate F stage gives a
//! valid root interval, the intervals NEST across the staircase, and
//! the endpoint equals the exact factorized response (gate 1); §23's
//! merge-before-focal-max is law and the deliberate cellwise-max
//! counterexample is REJECTED — the fused per-class sum strictly
//! exceeds the exact optimum somewhere, so it can never be a bound
//! (gate 2); the staged fixed-policy tail envelope brackets and
//! collapses to the exact profile (gate 3); an installed
//! [`TailEnvelopeFact`] closes to an executable lower and puts the
//! first NONZERO `contract_sensitive_residual` on the recommendation,
//! and survives the serialization round trip (gate 4); and the §41
//! census law — the frontier's `ResidualInterval` item has nonzero
//! root potential exactly where the closure can consume an interval,
//! the §42 law holds on its hand-executions, and at the declared flat
//! `3Z` forecast the exact item dominates it in the loop, honestly
//! recorded (gate 5).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §61, §22, §23, §5, §9, §41–§42, §54, under rulings APS-A8/APS-A9
//! (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frozen `verify_player` receipt roots (the six
//! enumerable fibers); staircase stages walked from 0 to the
//! action-exact endpoint (cap 28 — one fresh critical tile per
//! refinement bounds the loop).

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::{
    declaring_score_range, response_success_mass, staged_fused_class_sum, staged_policy_envelope,
    staged_response_interval, viewer_score_profile, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::frontier::{Frontier, SolveGoal, WorkItem};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{Fact, ProofState, SemanticsIdentity, TailEnvelopeFact};
use walt::solver::residual::ResidualIntervalProducer;

const ENUM_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

/// One fresh critical tile per refinement bounds every staircase.
const STAGE_CAP: usize = 28;

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

/// Gate 1 — §61's staircase: at every root action, every stage `s`
/// from 0 to the action-exact endpoint yields a valid interval
/// (ordered, mass-conserving) that BRACKETS the independently
/// recomputed exact §36 response; consecutive stages nest on both
/// sides (§9); and the endpoint collapses to the exact value with
/// zero residual. Non-vacuity: some stage-0 envelope is real
/// (positive residual) and some staircase takes more than one step.
#[test]
fn staircase_intervals_bracket_nest_and_reach_the_exact_endpoint() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut residual_seen = false;
    let mut multi_stage_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let state = ProofState::open(&root, &position, identity_of(&root, &position));
        for a in &state.legal {
            let child = belief.focal_play(*a);
            let z = oracle.mass(&child);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            let mut prev: Option<(u128, u128)> = None;
            for s in 0..=STAGE_CAP {
                let mut ss = ResponseStats::default();
                let i = staged_response_interval(&oracle, &child, &field, s, &mut ss);
                assert!(i.lower <= i.upper && i.upper <= z, "an ordered interval");
                assert_eq!(
                    i.exact_mass + i.residual_mass,
                    z,
                    "the stage census partitions the branch mass"
                );
                assert_eq!(
                    i.upper - i.lower,
                    i.residual_mass,
                    "the §22 width IS the unresolved mass at an undecided node"
                );
                assert!(
                    i.lower <= exact && exact <= i.upper,
                    "every F stage gives a root interval around the exact response"
                );
                if let Some((pl, pu)) = prev {
                    assert!(pl <= i.lower && i.upper <= pu, "the staircase nests (§9)");
                }
                if s == 0 && i.residual_mass > 0 {
                    residual_seen = true;
                }
                if i.residual_mass == 0 {
                    assert_eq!(i.lower, exact, "the endpoint is the exact response");
                    assert_eq!(i.upper, exact, "the endpoint interval is a point");
                    if s > 1 {
                        multi_stage_seen = true;
                    }
                    break;
                }
                assert!(s < STAGE_CAP, "the staircase terminates within 28 tiles");
                prev = Some((i.lower, i.upper));
            }
        }
    }
    assert!(residual_seen, "some stage-0 envelope carries real mass");
    assert!(multi_stage_seen, "some staircase takes more than one step");
}

/// Gate 2 — §23's law and the deliberate counterexample: the focal
/// player observes the public ACTION, so the true
/// information-consistent optimum of the exact-mass domain is the
/// merged per-action sum `Σ_t M*(E_t)` — and the fused per-class sum
/// (recursing each action-uniform class UNMERGED, letting the
/// continuation react to hidden class identity) STRICTLY exceeds it
/// somewhere: the fusion number claims mass no lawful policy attains
/// on that domain, which is exactly why the cellwise max is rejected
/// from every bound. (Comparing against the FULL posterior's optimum
/// would hide this — the residual's slack absorbs the inflation; the
/// same-domain comparison is the theorem's.)
#[test]
fn merged_branches_are_law_and_the_cellwise_max_counterexample_is_rejected() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut inflation_seen = false;
    // The six enumerable roots PLUS h3-t4: at t5/t6 the focal choice
    // is structurally trivial deep in the tree (the Phase 6
    // saturation finding), so classes never fuse — the counterexample
    // needs the structured trick-4 root, the same specimen every
    // phase's interesting behavior has lived on.
    let roots: Vec<(usize, usize)> = ENUM_ROOTS
        .iter()
        .map(|(h, t, _)| (*h, *t))
        .chain([(3usize, 4usize)])
        .collect();
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let state = ProofState::open(&root, &position, identity_of(&root, &position));
        for a in &state.legal {
            let child = belief.focal_play(*a);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            for s in 0..=4usize {
                let mut ss = ResponseStats::default();
                let lawful = staged_response_interval(&oracle, &child, &field, s, &mut ss);
                assert!(lawful.lower <= exact, "the lawful lower is sound");
                let mut fs = ResponseStats::default();
                let fused = staged_fused_class_sum(&oracle, &child, &field, s, &mut fs);
                assert!(
                    fused >= lawful.lower,
                    "fusion only inflates: reacting to hidden class identity never loses mass"
                );
                if fused > lawful.lower {
                    // The rejection, live: the fused number exceeds
                    // the true optimum of its own domain — no lawful
                    // policy attains it.
                    inflation_seen = true;
                }
            }
        }
    }
    assert!(
        inflation_seen,
        "the counterexample is live: the fused sum strictly exceeds the lawful optimum somewhere"
    );
}

/// Gate 3 — the §61/§54 staged fixed-policy tail envelope: at every
/// stage the tails are well-formed, bracket the exact profile's tails
/// threshold by threshold, nest across the staircase, and collapse to
/// the exact tails at the endpoint.
#[test]
fn policy_envelopes_bracket_nest_and_collapse_to_the_exact_profile() {
    let r = receipt();
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let mut straddle_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let state = ProofState::open(&root, &position, identity_of(&root, &position));
        for a in &state.legal {
            let child = belief.focal_play(*a);
            let z = oracle.mass(&child);
            let mut ps = RecursionStats::default();
            let exact = viewer_score_profile(&oracle, &child, &low, &field, &mut ps);
            assert_eq!(exact.total(), z, "profile conservation");
            let mut prev: Option<([u128; 43], [u128; 43])> = None;
            for s in 0..=STAGE_CAP {
                let mut es = RecursionStats::default();
                let e = staged_policy_envelope(&oracle, &child, &low, &field, s, &mut es);
                assert_eq!(e.lower_tail[0], z, "the whole mass at threshold 0");
                assert_eq!(e.upper_tail[0], z, "the whole mass at threshold 0");
                for k in 0..43 {
                    let t = exact.tail(k as u32);
                    assert!(
                        e.lower_tail[k] <= t && t <= e.upper_tail[k],
                        "the envelope brackets the exact tail at every threshold"
                    );
                    if k > 0 {
                        assert!(
                            e.lower_tail[k] <= e.lower_tail[k - 1]
                                && e.upper_tail[k] <= e.upper_tail[k - 1],
                            "tails are monotone"
                        );
                    }
                    if let Some((pl, pu)) = &prev {
                        assert!(
                            pl[k] <= e.lower_tail[k] && e.upper_tail[k] <= pu[k],
                            "the envelope staircase nests (§9)"
                        );
                    }
                }
                if e.lower_tail[position.bid as usize] < e.upper_tail[position.bid as usize] {
                    straddle_seen = true;
                }
                if e.residual_mass == 0 {
                    for k in 0..43 {
                        assert_eq!(e.lower_tail[k], exact.tail(k as u32), "endpoint collapse");
                        assert_eq!(e.upper_tail[k], exact.tail(k as u32), "endpoint collapse");
                    }
                    break;
                }
                assert!(s < STAGE_CAP, "the staircase terminates within 28 tiles");
                prev = Some((e.lower_tail, e.upper_tail));
            }
        }
    }
    assert!(
        straddle_seen,
        "some envelope straddles its contract — the §7 residual is real"
    );
}

/// Gate 4 — the envelope FACT: installing a staged envelope closes to
/// a deterministic EXECUTABLE lower equal to the guaranteed-side
/// parity projection, the recommendation carries the §7
/// contract-sensitive residual (nonzero somewhere — the Phase 3
/// placeholder is retired), and the fact survives the serialization
/// round trip bytewise.
#[test]
fn envelope_facts_close_to_executable_lowers_with_real_residual() {
    let r = receipt();
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let mut nonzero_residual_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        for a in state.legal.clone() {
            let child = belief.focal_play(a);
            let z = oracle.mass(&child);
            let mut es = RecursionStats::default();
            let e = staged_policy_envelope(&oracle, &child, &low, &field, 1, &mut es);
            let c = position.bid as usize;
            let guaranteed = match identity.utility_id.as_str() {
                "pmake-v1" => e.lower_tail[c],
                _ => z - e.upper_tail[c],
            };
            let straddle = e.upper_tail[c] - e.lower_tail[c];
            state
                .install(
                    &identity,
                    Fact::Envelope(Box::new(TailEnvelopeFact {
                        action: a,
                        policy_id: "staged-lowest-first-s1".to_string(),
                        lower_tail: e.lower_tail,
                        upper_tail: e.upper_tail,
                    })),
                )
                .expect("a well-formed envelope installs");
            let report = state.closure();
            let view = report
                .views
                .iter()
                .find(|v| v.action == a)
                .expect("a legal action has a view");
            let expect = BigRational::new(BigInt::from(guaranteed), BigInt::from(z));
            assert_eq!(view.lower, expect, "the guaranteed side is the lower");
            let w = report.exec.expect("an envelope witness is executable");
            if w.action == a {
                assert!(
                    w.authority.starts_with("envelope:"),
                    "the witness names its envelope"
                );
                let rec = state.recommend().expect("an executable witness recommends");
                let residual = rec
                    .contract_sensitive_residual
                    .expect("an envelope witness reports its straddle");
                assert_eq!(
                    residual,
                    BigRational::new(BigInt::from(straddle), BigInt::from(z)),
                    "the recommendation's residual is the §7 straddle"
                );
                if residual > BigRational::zero() {
                    nonzero_residual_seen = true;
                }
            }
        }
        let text = state.serialize();
        let resumed = ProofState::parse(&text, &root, &position).expect("a valid state resumes");
        assert_eq!(resumed.serialize(), text, "the round trip is bytewise");
    }
    assert!(
        nonzero_residual_seen,
        "some recommendation carries a NONZERO contract-sensitive residual"
    );
}

/// Gate 5 — the §41 census law and honest dominance: the ample §61
/// producer's intervals bracket the exact values through the closure;
/// the frontier's `ResidualInterval` item has positive potential
/// EXACTLY where an open interval lets closure consume one (both
/// sides shown); hand-executing it obeys the §42 law; and in the
/// ample loop at the declared flat `3Z` forecast the exact item
/// dominates it — no residual item is ever bought, recorded honestly.
#[test]
fn census_potential_is_nonzero_exactly_where_closure_consumes() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut open_seen = false;
    let mut closed_seen = false;
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
        let goals = [
            SolveGoal::SelectAction,
            SolveGoal::RecommendEpsilonPolicy {
                epsilon: BigRational::zero(),
            },
            SolveGoal::StrengthenToExact,
        ];
        // Top state: every interval is open, so the census has
        // positive potential wherever the goal formula can bite. For
        // the ε-goal the top state is the §41 STALL — a lone residual
        // upper provably cannot move `U*` while every other upper is
        // vacuous — so its potential is rightly ZERO there: the
        // census law is per-goal, not a blanket yes.
        let top = ProofState::open(&root, &position, identity.clone());
        for goal in &goals {
            for a in &top.legal {
                let item = WorkItem::ResidualInterval { action: *a };
                let p = frontier.item_potential(&item, goal, &top);
                if matches!(goal, SolveGoal::RecommendEpsilonPolicy { .. }) && top.legal.len() > 1 {
                    assert_eq!(
                        p,
                        BigRational::zero(),
                        "the §41 stall holds for the census item too"
                    );
                } else {
                    assert!(
                        p > BigRational::zero(),
                        "an open interval is consumable — the census has root potential"
                    );
                    open_seen = true;
                }
                // The §42 law on a hand-execution of the refusable
                // item: realized reduction within the declared bound.
                let mut probe = top.clone();
                let before = goal.debt(&probe, &probe.closure());
                frontier.execute_item(&item, &mut probe);
                let after = goal.debt(&probe, &probe.closure());
                assert!(after <= before, "debts are monotone");
                assert!(&before - &after <= p, "the §42 law on the census item");
            }
        }
        // Ample strengthen run: every surviving interval a point —
        // and the residual item's potential collapses to zero with
        // nothing left to consume. The schedule itself never buys a
        // residual item: at the declared flat 3Z the exact item has
        // the same bound at the same cost and comes first in order
        // (the honest dominance the module doc records).
        let mut state = ProofState::open(&root, &position, identity);
        let out = frontier.advance(&mut state, &SolveGoal::StrengthenToExact, u128::MAX / 4);
        assert!(out.met, "an ample budget strengthens every fixture root");
        assert!(
            !out.executed
                .iter()
                .any(|e| matches!(e.item, WorkItem::ResidualInterval { .. })),
            "at a flat 3Z forecast the exact item dominates the staged interval"
        );
        let report = state.closure();
        for goal in &goals {
            for v in report.views.iter().filter(|v| !v.excluded) {
                let item = WorkItem::ResidualInterval { action: v.action };
                let p = frontier.item_potential(&item, goal, &state);
                assert_eq!(
                    p,
                    BigRational::zero(),
                    "a point interval leaves the census nothing to consume"
                );
                closed_seen = true;
            }
        }
        // The ample producer, for completeness: its intervals install
        // and bracket the exact values already in the state.
        let producer = ResidualIntervalProducer {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let results = state.run_producer(&producer);
        assert!(
            results.iter().all(|r| r.is_ok()),
            "every residual interval installs"
        );
        let after = state.closure();
        for (v_before, v_after) in report.views.iter().zip(after.views.iter()) {
            assert_eq!(
                v_before.lower, v_after.lower,
                "a bracketing interval never moves an exact point"
            );
            assert_eq!(
                v_before.upper, v_after.upper,
                "a bracketing interval never moves an exact point"
            );
        }
    }
    assert!(open_seen && closed_seen, "both census directions are live");
}

/// The §62 range walk underpins Phase 5 but is Phase 4's neighbor in
/// the recursion family — pin its two elementary laws here: the
/// fixed-policy range sits inside the free-focal range, and both
/// ranges contain every score of the policy's exact profile.
#[test]
fn score_ranges_nest_and_contain_the_profile_support() {
    let r = receipt();
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let state = ProofState::open(&root, &position, identity_of(&root, &position));
        for a in &state.legal {
            let child = belief.focal_play(*a);
            let mut s1 = ResponseStats::default();
            let (dev_min, dev_max) = declaring_score_range(&oracle, &child, None, &field, &mut s1);
            let mut s2 = ResponseStats::default();
            let (pol_min, pol_max) =
                declaring_score_range(&oracle, &child, Some(&low), &field, &mut s2);
            assert!(
                dev_min <= pol_min && pol_max <= dev_max,
                "one policy's range sits inside the free-focal range"
            );
            let mut ps = RecursionStats::default();
            let profile = viewer_score_profile(&oracle, &child, &low, &field, &mut ps);
            let floor = profile.bins.iter().position(|m| *m > 0).expect("mass") as u32;
            let ceiling = profile.bins.iter().rposition(|m| *m > 0).expect("mass") as u32;
            assert_eq!(
                (pol_min, pol_max),
                (floor, ceiling),
                "the fixed-policy range IS the profile's support range"
            );
        }
    }
}
