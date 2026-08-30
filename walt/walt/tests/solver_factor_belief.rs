//! Gates for the counted-belief Slice C, stages C0 and C1 [L2 thread]:
//! the factor belief, the exact-cover contraction interface, and backend
//! zero — mass parity with the shipped counting DP AND complete-world
//! enumeration, branch-mass parity with world-by-world field
//! classification, the Theorem 20.1 conditioning route, mass conservation
//! at every contraction, and the declared C0 domain refusals (gates 1–6);
//! then the stage-C1 cache laws of the cached σ0 field (gates 7–10): σ0
//! branch parity with the bundled one-ply oracle on every receipt fiber
//! with full extensional cache identity between the routes, classification
//! once per information state, zero sharing across public histories under
//! the full §43 identity key, and the opening root's 116,280 hands
//! classified exactly once.
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! Parts V–VI (§18–26, §43, §46 stages C0–C1), adopted by rulings CBS-A6
//! and CBS-A9 (`walt/CENSUS-RULINGS.md`); design register
//! `walt/FACTOR-BELIEF.md`.
//!
//! DECLARED TEST EPOCH: deterministic fields only — the trivial
//! `FixedPreference` fields of §46 stage C0 and the σ0 Level0 { n0 = 2 }
//! modeled mind (stage C1's declared cached field). Frozen `verify_player`
//! receipt roots: hands 4/5/10/12 at trick 6 (fibers 90/27/19/6), hands
//! 3/8 at trick 5 (fibers 200/92), and hand 0 at trick 1 (fiber
//! 399,072,960 — contracted, never enumerated).

mod common;

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, FactorWeights, FiberOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// The lowest legal tile of the viewer at a trick-start root.
fn lowest_focal(root: &CanonicalRoot, position: &RootPosition) -> Domino {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    legal.iter().next().expect("a legal focal tile")
}

/// The complete-world enumeration oracle for one-ply branch masses: replay
/// the focal tile over EVERY world of the fiber, classify the next hidden
/// seat's field action world by world, and bucket. The record is built by
/// hand here — independently of the module's public-history walker.
fn enumerate_branches(
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: Domino,
    field: &dyn walt::solver::adaptive::SlicePolicy,
) -> Vec<(Domino, u128)> {
    let viewer = root.kernel().viewer();
    let seat = viewer.plus(1);
    let trick_plays = vec![focal];
    let history = vec![focal];
    let mut buckets: Vec<(Domino, u128)> = Vec::new();
    for world in root.worlds() {
        let hand = world.hand(seat);
        let led = Some(position.decl.led_context(focal));
        let legal = legal_plays(position.decl, hand, led);
        let record = PublicRecord {
            leader: position.leader,
            trick_plays: &trick_plays,
            banked: position.banked,
            root: position,
            history: &history,
        };
        let tile = field.choose(position.decl, hand, legal, &record);
        match buckets.iter_mut().find(|(t, _)| *t == tile) {
            Some((_, m)) => *m += 1,
            None => buckets.push((tile, 1)),
        }
    }
    buckets.sort_by_key(|(t, _)| t.index());
    buckets
}

/// The σ0 field of the C1 down-payment gate.
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

/// Gate 1 — the uniform root mass agrees three ways on every receipt
/// root: backend zero (the shipped counting DP), the canonical root's own
/// count, and complete-world enumeration. A focal play (Theorem 23.1,
/// focal case) changes NO factor and no mass.
#[test]
fn uniform_mass_three_way_parity_and_focal_invariance() {
    let r = receipt();
    let field = FixedPreference::lowest_first("field:lowest-first");
    let oracle = FiberOracle;
    for (hand_id, trick_no, fiber) in [
        (12, 6, 6u128),
        (10, 6, 19),
        (5, 6, 27),
        (4, 6, 90),
        (8, 5, 92),
        (3, 5, 200),
    ] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        assert_eq!(oracle.mass(&belief), fiber);
        assert_eq!(root.count(), fiber);
        assert_eq!(root.worlds().count() as u128, fiber);

        let after = belief.focal_play(lowest_focal(&root, &position));
        assert_eq!(
            after.factors(),
            belief.factors(),
            "a focal play changes no factor"
        );
        assert_eq!(oracle.mass(&after), fiber, "a focal play changes no mass");
        assert_eq!(after.history(), &[lowest_focal(&root, &position)]);
    }
}

/// Gate 2 — one-ply branch masses under the trivial fields equal
/// world-by-world enumeration exactly, on small and medium fibers, for
/// two distinct preference orders (the same hands classify to different
/// branch tables). Mass conservation is asserted inside every
/// `branch_masses` call; the sum is re-checked against the fiber here.
#[test]
fn branch_masses_match_complete_world_enumeration() {
    let r = receipt();
    let oracle = FiberOracle;
    for (hand_id, trick_no) in [(4, 6), (10, 6), (8, 5), (3, 5)] {
        for lowest in [true, false] {
            let field = if lowest {
                FixedPreference::lowest_first("field:lowest-first")
            } else {
                FixedPreference::highest_first("field:highest-first")
            };
            let (root, position) = root_at(&r, hand_id, trick_no);
            let focal = lowest_focal(&root, &position);
            let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
            let contracted = oracle.branch_masses(&belief, &field);
            let enumerated = enumerate_branches(&root, &position, focal, &field);
            assert_eq!(contracted, enumerated, "hand {hand_id} trick {trick_no}");
            let total: u128 = contracted.iter().map(|(_, m)| m).sum();
            assert_eq!(total, root.count());
        }
    }
}

/// Gate 2b — the stage-C1 down-payment: the same branch-mass parity under
/// the σ0 Level0 modeled mind on the two smallest fibers. Both routes
/// share one field instance, so the insert-only action cache serves both;
/// the contraction route classifies DISTINCT hands, never worlds.
#[test]
fn level0_field_branch_parity() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = level0_field();
    for (hand_id, trick_no) in [(12, 6), (10, 6)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let focal = lowest_focal(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
        let contracted = oracle.branch_masses(&belief, &field);
        let enumerated = enumerate_branches(&root, &position, focal, &field);
        assert_eq!(contracted, enumerated, "hand {hand_id} trick {trick_no}");
    }
}

/// Gate 3 — Theorem 20.1's conditioning route: conditioning on each
/// observed branch touches ONLY the acting seat's factor, every kept hand
/// holds the observed tile, and the conditioned mass recovers that
/// branch's mass exactly (the one-table contraction re-derives what the
/// classification summed).
#[test]
fn condition_recovers_each_branch_mass() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = FixedPreference::lowest_first("field:lowest-first");
    let (root, position) = root_at(&r, 4, 6);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
    let branches = oracle.branch_masses(&belief, &field);
    assert!(!branches.is_empty());
    let acting = belief.seat_to_move();
    for (tile, mass) in &branches {
        let conditioned = oracle.condition(&belief, *tile, &field);
        assert_eq!(oracle.mass(&conditioned), *mass);
        assert_eq!(conditioned.history(), &[focal, *tile]);
        for (i, factor) in conditioned.factors().iter().enumerate() {
            if factor.seat() == acting {
                match factor.weights() {
                    FactorWeights::Table(entries) => {
                        assert!(!entries.is_empty());
                        for (hand, weight) in entries {
                            assert!(hand.contains(*tile), "a kept hand holds the observed tile");
                            assert_eq!(*weight, 1, "a conditioned uniform weight stays 0/1");
                        }
                    }
                    FactorWeights::UniformLawful { .. } => {
                        panic!("the acting factor was conditioned")
                    }
                }
            } else {
                assert_eq!(
                    factor,
                    &belief.factors()[i],
                    "conditioning touches only the acting seat's factor"
                );
            }
        }
    }
}

/// Gate 4 — the conditioned marginal equals complete-world enumeration:
/// for every pool tile, the posterior mass of "the acting seat holds it"
/// matches a world-by-world count over the observed branch.
#[test]
fn conditioned_marginal_matches_enumeration() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = FixedPreference::lowest_first("field:lowest-first");
    let (root, position) = root_at(&r, 10, 6);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
    let branches = oracle.branch_masses(&belief, &field);
    let (observed, _) = *branches
        .iter()
        .max_by_key(|(_, m)| *m)
        .expect("a nonempty branch table");
    let conditioned = oracle.condition(&belief, observed, &field);
    let acting = belief.seat_to_move();

    // Enumeration route: worlds where the field's classified action is the
    // observed tile, bucketed by whether the acting hand holds each probe.
    let viewer = root.kernel().viewer();
    assert_eq!(acting, viewer.plus(1));
    let trick_plays = vec![focal];
    let history = vec![focal];
    for probe in root.kernel().pool().iter() {
        let mut count: u128 = 0;
        for world in root.worlds() {
            let hand = world.hand(acting);
            let led = Some(position.decl.led_context(focal));
            let legal = legal_plays(position.decl, hand, led);
            let record = PublicRecord {
                leader: position.leader,
                trick_plays: &trick_plays,
                banked: position.banked,
                root: &position,
                history: &history,
            };
            if field.choose(position.decl, hand, legal, &record) == observed && hand.contains(probe)
            {
                count += 1;
            }
        }
        let marginal = oracle.marginal(&conditioned, acting, &|hand| hand.contains(probe));
        assert_eq!(marginal, count, "probe {probe:?}");
    }
}

/// Gate 5 — the declared C0 domain and the §43 identity gates refuse, by
/// panic, everything outside them: a mismatched field identity, branch
/// masses at a focal node, a focal play at a hidden node, an illegal
/// focal play, and any contraction across two conditioned factors (the
/// Slice D boundary).
#[test]
fn c0_domain_and_identity_refusals() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = FixedPreference::lowest_first("field:lowest-first");
    let other = FixedPreference::highest_first("field:highest-first");
    let (root, position) = root_at(&r, 4, 6);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let after = belief.focal_play(focal);

    // One field identity governs a belief's conditionings.
    assert!(catch_unwind(AssertUnwindSafe(|| oracle.branch_masses(&after, &other))).is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let (t, _) = oracle.branch_masses(&after, &field)[0];
        oracle.condition(&after, t, &other)
    }))
    .is_err());

    // Branch masses are a hidden seat's; a focal play is the viewer's.
    assert!(catch_unwind(AssertUnwindSafe(|| oracle.branch_masses(&belief, &field))).is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| after.focal_play(focal))).is_err());

    // An illegal focal play is refused at the root.
    let unheld = root
        .kernel()
        .pool()
        .iter()
        .next()
        .expect("a pool tile the viewer cannot hold");
    assert!(catch_unwind(AssertUnwindSafe(|| belief.focal_play(unheld))).is_err());

    // Two conditioned factors put mass outside the C0 domain (Slice D),
    // and a completion query never spans a conditioned factor.
    let (t1, _) = oracle.branch_masses(&after, &field)[0];
    let once = oracle.condition(&after, t1, &field);
    assert!(
        catch_unwind(AssertUnwindSafe(|| oracle.branch_masses(&once, &field))).is_err(),
        "the next actor's completion query spans the conditioned factor"
    );
    let next = once.seat_to_move();
    let t2 = {
        // Find the second actor's field action under ANY world consistent
        // with the belief: enumerate its uniform support and take the
        // first classified action.
        let slot = once
            .factors()
            .iter()
            .position(|f| f.seat() == next)
            .expect("a hidden actor");
        let hand = match once.factors()[slot].weights() {
            FactorWeights::UniformLawful { allowed } => {
                let tiles: Vec<Domino> = allowed.iter().collect();
                let k = once.factors()[slot].capacity();
                tiles[..k].iter().copied().collect::<DominoSet>()
            }
            FactorWeights::Table(_) => panic!("the second actor is unconditioned"),
        };
        let led = Some(position.decl.led_context(focal));
        let legal = legal_plays(position.decl, hand, led);
        legal.iter().next().expect("a legal tile")
    };
    let twice_result = catch_unwind(AssertUnwindSafe(|| {
        let twice = oracle.condition(&once, t2, &field);
        oracle.mass(&twice)
    }));
    assert!(
        twice_result.is_err(),
        "a two-table contraction is refused (Slice D boundary)"
    );
}

/// Gate 6 — §22 at the opening root, contracted and never enumerated: at
/// hand 0 trick 1 the fiber holds 399,072,960 worlds; backend zero counts
/// them through the shipped DP, the acting seat's support is exactly
/// 116,280 root hands each with exactly 3,432 compatible completions, and
/// the one-ply branch masses under the trivial field partition the full
/// mass — with no complete world ever materialized on the contraction
/// route.
#[test]
fn opening_root_contraction_without_worlds() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = FixedPreference::lowest_first("field:lowest-first");
    let (root, position) = root_at(&r, 0, 1);
    assert_eq!(root.count(), 399_072_960);
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    assert_eq!(oracle.mass(&belief), 399_072_960);

    let after = belief.focal_play(lowest_focal(&root, &position));
    let weights = oracle.actor_completion_weights(&after, after.seat_to_move());
    assert_eq!(weights.len(), 116_280, "C(21,7) acting-seat root hands");
    assert!(
        weights.iter().all(|(_, w)| *w == 3_432),
        "C(14,7) completions per hand at the voidless opening"
    );

    let branches = oracle.branch_masses(&after, &field);
    assert!(
        branches.len() >= 2,
        "the opening branch table is nontrivial"
    );
    let total: u128 = branches.iter().map(|(_, m)| m).sum();
    assert_eq!(total, 399_072_960, "Z_h = Σ_t Z_ht at the opening root");
}

// ---------------------------------------------------------------------------
// Stage C1 — the cached σ0 field (§46 stage C1, §26 items 2/4/6).
// ---------------------------------------------------------------------------

/// The bundled one-ply oracle (stage C1's extensional route): iterate the
/// fiber's worlds, group members by the acting seat's remaining hand,
/// query the field ONCE per distinct hand — `solver::bundle`'s field-ply
/// partition idiom realized at one ply — and bucket world counts by the
/// chosen tile. The record is built by hand here, independently of the
/// module's public-history walker. Returns the buckets and the number of
/// distinct hands materialized.
fn bundled_branches(
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: Domino,
    field: &dyn SlicePolicy,
) -> (Vec<(Domino, u128)>, usize) {
    let seat = root.kernel().viewer().plus(1);
    let trick_plays = vec![focal];
    let history = vec![focal];
    let mut by_hand: HashMap<u32, Domino> = HashMap::new();
    let mut buckets: Vec<(Domino, u128)> = Vec::new();
    for world in root.worlds() {
        let hand = world.hand(seat);
        let tile = match by_hand.get(&hand.bits()) {
            Some(t) => *t,
            None => {
                let led = Some(position.decl.led_context(focal));
                let legal = legal_plays(position.decl, hand, led);
                let record = PublicRecord {
                    leader: position.leader,
                    trick_plays: &trick_plays,
                    banked: position.banked,
                    root: position,
                    history: &history,
                };
                let t = field.choose(position.decl, hand, legal, &record);
                by_hand.insert(hand.bits(), t);
                t
            }
        };
        match buckets.iter_mut().find(|(t, _)| *t == tile) {
            Some((_, m)) => *m += 1,
            None => buckets.push((tile, 1)),
        }
    }
    buckets.sort_by_key(|(t, _)| t.index());
    (buckets, by_hand.len())
}

/// Gate 7 (stage C1) — σ0 branch-mass parity with the bundled one-ply
/// oracle on ALL six receipt fibers, one fresh field instance per route,
/// plus the extensional cache identity: both routes materialize the SAME
/// information states (exactly the feasible root hands — a root hand is
/// realizable in some world iff its completion count is positive) with
/// the SAME cached action at every one. Map equality over the two caches
/// is the whole claim at once.
#[test]
fn level0_branch_parity_with_the_bundled_one_ply_oracle() {
    let r = receipt();
    let oracle = FiberOracle;
    for (hand_id, trick_no) in [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let focal = lowest_focal(&root, &position);
        let field_c = level0_field();
        let belief = FactorBelief::uniform_root(&root, &position, &field_c).focal_play(focal);
        let contracted = oracle.branch_masses(&belief, &field_c);
        let field_b = level0_field();
        let (bundled, distinct) = bundled_branches(&root, &position, focal, &field_b);
        assert_eq!(contracted, bundled, "h{hand_id}-t{trick_no}: route parity");
        assert_eq!(
            field_c.cache_len(),
            distinct,
            "h{hand_id}-t{trick_no}: the contraction route classifies exactly the realizable hands"
        );
        assert_eq!(
            field_c.cache_snapshot(),
            field_b.cache_snapshot(),
            "h{hand_id}-t{trick_no}: both routes materialize identical states with identical actions"
        );
    }
}

/// Gate 8 (stage C1) — classification is once per information state:
/// counting alone reads no field, the first contraction materializes one
/// state per feasible hand, a repeated contraction materializes nothing
/// and returns the identical table, and conditioning — which re-reads the
/// WHOLE support — adds exactly the zero-completion support hands, once
/// across every observed branch.
#[test]
fn classification_is_once_per_information_state() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = level0_field();
    let (root, position) = root_at(&r, 4, 6);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
    let seat = belief.seat_to_move();
    let support_len = belief
        .factors()
        .iter()
        .find(|f| f.seat() == seat)
        .expect("a hidden actor")
        .support()
        .len();
    let feasible = oracle.actor_completion_weights(&belief, seat).len();
    assert_eq!(field.cache_len(), 0, "counting alone reads no field");
    let first = oracle.branch_masses(&belief, &field);
    assert_eq!(field.cache_len(), feasible, "one state per feasible hand");
    let second = oracle.branch_masses(&belief, &field);
    assert_eq!(second, first, "a repeat returns the identical table");
    assert_eq!(
        field.cache_len(),
        feasible,
        "a repeat classifies nothing new"
    );
    for (tile, mass) in &first {
        let conditioned = oracle.condition(&belief, *tile, &field);
        assert_eq!(oracle.mass(&conditioned), *mass);
    }
    assert_eq!(
        field.cache_len(),
        support_len,
        "conditioning adds exactly the zero-completion support hands, once"
    );
}

/// Gate 9 (stage C1) — the §43 identity law of the cache: the full
/// information-state key shares NOTHING across focal candidates or roots.
/// Every contraction under a new public history materializes its full
/// query count — zero hits — because the key carries the history. Sharing
/// across histories would need a proven state reduction (the Slice F
/// vocabulary), never a looser key: a hit under an omitted coordinate is
/// the PiKey defect reborn (CBS-A6).
#[test]
fn the_full_identity_key_shares_nothing_across_candidates_or_roots() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = level0_field();
    let (root, position) = root_at(&r, 4, 6);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    assert!(
        legal.len() >= 2,
        "the cross-candidate claim needs two candidates"
    );
    for focal in legal.iter() {
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
        let queries = oracle
            .actor_completion_weights(&belief, belief.seat_to_move())
            .len();
        let before = field.cache_len();
        oracle.branch_masses(&belief, &field);
        assert_eq!(
            field.cache_len(),
            before + queries,
            "zero cross-candidate sharing under the full identity key"
        );
    }
    let (root2, position2) = root_at(&r, 12, 6);
    let focal2 = lowest_focal(&root2, &position2);
    let belief2 = FactorBelief::uniform_root(&root2, &position2, &field).focal_play(focal2);
    let queries2 = oracle
        .actor_completion_weights(&belief2, belief2.seat_to_move())
        .len();
    let before2 = field.cache_len();
    oracle.branch_masses(&belief2, &field);
    assert_eq!(
        field.cache_len(),
        before2 + queries2,
        "zero cross-root sharing under the full identity key"
    );
}

/// Gate 10 (stage C1) — §46's charter sentence at the scale that names
/// it: at the opening root every one of the 116,280 feasible acting-seat
/// hands is classified by the σ0 mind EXACTLY ONCE, the branch masses
/// conserve the 399,072,960-world mass, and a repeated contraction is
/// pure cache identity — the identical table, zero new states. The
/// bundled route is deliberately absent here: the 399-million-world loop
/// is the representation this slice retires.
#[test]
fn opening_root_level0_classification_is_once_per_hand() {
    let r = receipt();
    let oracle = FiberOracle;
    let field = level0_field();
    let (root, position) = root_at(&r, 0, 1);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
    let branches = oracle.branch_masses(&belief, &field);
    assert_eq!(
        field.cache_len(),
        116_280,
        "C(21,7) hands, each classified once"
    );
    let again = oracle.branch_masses(&belief, &field);
    assert_eq!(again, branches, "a repeat is pure cache identity");
    assert_eq!(field.cache_len(), 116_280, "a repeat materializes nothing");
    let total: u128 = branches.iter().map(|(_, m)| m).sum();
    assert_eq!(total, 399_072_960, "Z_h = Σ_t Z_ht under the σ0 mind");
}
