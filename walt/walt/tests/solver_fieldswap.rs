//! Gates for `solver::field` and `solver::exposure` — the field-swap
//! vertical slice of parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` (§21 steps 3–4;
//! rulings L2-A1..A7; obligations O29/O30/O31 of
//! `walt/SCENARIO-PLAYER.md` §10).
//!
//! The small root is the policy slice's: hand 4, trick 6 of the frozen
//! `verify_player` receipt — fiber 90, horizon 2 — cheap enough to
//! enumerate exactly under a level-1 σ1 field at a declared [2,2] inner
//! schedule.

mod common;

use std::cell::Cell;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::{Decl, Domino, DominoSet, Seat};
use walt::solver::adaptive::{
    replay_viewer_success, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy, STREAM_DOMAIN,
};
use walt::solver::exposure::{
    coupled_replay, frozen_policy_exposure, ExposureRung, FrozenPolicyExposure,
    RootActionExposureUpper, WorldDomain,
};
use walt::solver::field::{
    field_actions, FieldKind, FieldModel, FieldSpec, FieldStateKey, FIELD_DOMAIN,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule, DISCOVERY_DOMAIN,
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

/// A frozen pinned-then-level-1 focal policy for the small root, pinned to
/// the given tile of the viewer's hand (the viewer leads, so every held
/// tile is legal at the root information state).
fn focal(position: &RootPosition, pinned: Domino) -> FrozenPolicy {
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

fn viewer_tiles(root: &CanonicalRoot) -> Vec<Domino> {
    root.kernel().viewer_hand().iter().collect()
}

// ---------------------------------------------------------------------------
// Coupled-replay self-consistency (parent §3.1): the coupled execution's
// two terminals equal two direct single-field runs on the same world.
// ---------------------------------------------------------------------------

#[test]
fn coupled_replay_matches_two_direct_runs_on_the_small_fiber() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let viewer = root.kernel().viewer();
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = focal(&position, viewer_tiles(&root)[0]);
    let mut splits = 0u32;
    for world in root.worlds() {
        let outcome = coupled_replay(&position, viewer, &world, &rho, &field0, &field1);
        let direct0 = replay_viewer_success(&position, viewer, &world, &rho, &field0);
        let direct1 = replay_viewer_success(&position, viewer, &world, &rho, &field1);
        assert_eq!(outcome.u0, direct0, "coupled u0 equals the direct σ0 run");
        assert_eq!(outcome.u1, direct1, "coupled u1 equals the direct σ1 run");
        if outcome.exposed() {
            splits += 1;
        }
        // L2-T1 pointwise: |u1 − u0| ≤ D.
        assert!(outcome.correction() == 0 || outcome.exposed());
    }
    // Honest either way: the fields may or may not split on this root —
    // the gate is consistency, not a split count.
    let _ = splits;
}

// ---------------------------------------------------------------------------
// The pre-split assertions fire (acceptance item 5 / parent §3.1): an
// information-inconsistent focal policy is caught, not absorbed.
// ---------------------------------------------------------------------------

/// A deliberately unlawful focal policy: interior state makes successive
/// calls disagree, so the two coupled executions receive different focal
/// actions on equal public histories.
struct Alternator {
    calls: Cell<u64>,
}

impl SlicePolicy for Alternator {
    fn id(&self) -> &str {
        "test:alternator"
    }

    fn choose(
        &self,
        _decl: Decl,
        _hand: DominoSet,
        legal: DominoSet,
        _record: &PublicRecord<'_>,
    ) -> Domino {
        let call = self.calls.get();
        self.calls.set(call + 1);
        let tiles: Vec<Domino> = legal.iter().collect();
        if call.is_multiple_of(2) {
            tiles[0]
        } else {
            tiles[tiles.len() - 1]
        }
    }
}

#[test]
#[should_panic(expected = "information-consistent")]
fn the_coupled_replay_rejects_an_information_inconsistent_focal_policy() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let viewer = root.kernel().viewer();
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let cheat = Alternator {
        calls: Cell::new(0),
    };
    let world = root.worlds().next().expect("a nonempty fiber");
    // The viewer leads the small root with two legal tiles, so the
    // alternator's first two calls disagree and the assertion fires.
    let _ = coupled_replay(&position, viewer, &world, &cheat, &field0, &field1);
}

// ---------------------------------------------------------------------------
// FrozenPolicyExposure: L2-T1 pointwise on every world, the §3.2 bound,
// and tally/row agreement (O30).
// ---------------------------------------------------------------------------

#[test]
fn exposure_over_the_exact_fiber_satisfies_l2_t1_pointwise_and_the_correction_bound() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = focal(&position, viewer_tiles(&root)[0]);
    let exposure = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    assert_eq!(u128::from(exposure.worlds), root.count());
    assert_eq!(exposure.rows.len() as u64, exposure.worlds);
    let mut exposed = 0u64;
    let mut plus = 0u64;
    let mut minus = 0u64;
    for row in &exposure.rows {
        // L2-T1 pointwise: a correction without a split is impossible.
        assert!(row.u0 == row.u1 || row.split.is_some());
        if row.split.is_some() {
            exposed += 1;
        }
        match (row.u1, row.u0) {
            (true, false) => plus += 1,
            (false, true) => minus += 1,
            _ => {}
        }
    }
    // The aggregates are a function of the rows, not a second authority.
    assert_eq!(exposure.exposed, exposed);
    assert_eq!(exposure.corrections_plus, plus);
    assert_eq!(exposure.corrections_minus, minus);
    // §3.2: |c| ≤ E[|C|] ≤ d, exactly.
    let c = exposure.c_hat();
    let c_abs = if c < q(0, 1) { -c } else { c };
    assert!(c_abs <= exposure.c_abs_hat());
    assert!(exposure.c_abs_hat() <= exposure.d_hat());
    // The result names both fields, the policy, and its domain.
    assert_eq!(exposure.policy, rho.policy_id());
    assert_eq!(exposure.field0, field0.field_id());
    assert_eq!(exposure.field1, field1.field_id());
    assert_eq!(exposure.domain, WorldDomain::ExactFiber);
}

// ---------------------------------------------------------------------------
// Field purity (O29): the hidden-world adversarial route. Two worlds that
// agree on one seat's hand but differ elsewhere cannot produce different
// field actions for that seat — the action is a function of the declared
// information state alone, and the choose signature admits no world.
// ---------------------------------------------------------------------------

#[test]
fn field_actions_are_pure_functions_of_the_information_state_for_both_models() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let viewer = root.kernel().viewer();
    // Find a hidden seat and two worlds agreeing on that seat's hand but
    // differing on another hidden seat's hand.
    let worlds: Vec<World> = root.worlds().collect();
    let mut adversarial: Option<(Seat, World, World)> = None;
    'outer: for i in 0..worlds.len() {
        for j in (i + 1)..worlds.len() {
            for hidden in root.kernel().hidden() {
                let s = hidden.seat;
                if worlds[i].hand(s) == worlds[j].hand(s) && worlds[i] != worlds[j] {
                    adversarial = Some((s, worlds[i], worlds[j]));
                    break 'outer;
                }
            }
        }
    }
    let (seat, w1, w2) = adversarial.expect("fiber 90 holds an agreeing pair");
    assert_ne!(seat, viewer);
    let hand = w1.hand(seat);
    assert_eq!(hand, w2.hand(seat));
    // Drive the record one ply past the root so `seat` could be the actor
    // in a real replay; here the query itself is the point — its inputs
    // are (hand, public record) and nothing else. Two FRESH field pairs,
    // queried under the two different hidden completions, must agree.
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &[],
    };
    let legal = hand;
    for spec in [field0_spec(), field1_spec()] {
        let fresh_a = FieldModel::new(spec.clone());
        let fresh_b = FieldModel::new(spec.clone());
        // "Under world w1" and "under world w2": the worlds appear nowhere
        // in the call — that is the O29 type-level guarantee — so the
        // actions must be identical.
        let (a1, b1) = (
            fresh_a.choose(position.decl, hand, legal, &record),
            fresh_b.choose(position.decl, hand, legal, &record),
        );
        assert_eq!(a1, b1, "fresh instances agree on the same state");
        // Determinism across repeated queries (cache hit path).
        assert_eq!(fresh_a.choose(position.decl, hand, legal, &record), a1);
    }
    // Whole-replay purity: one shared field pair across every fiber world.
    // Any action that depended on hidden data would trip the insert-only
    // cache assertion when a revisited state recomputed differently.
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = focal(&position, viewer_tiles(&root)[0]);
    for world in &worlds {
        let _ = coupled_replay(&position, viewer, world, &rho, &field0, &field1);
    }
    assert!(field0.cache_len() > 0);
    assert!(field1.cache_len() > 0);
}

// ---------------------------------------------------------------------------
// FieldId (parent §8 Stage 0; acceptance item 2): immutable content
// address, every identity component distinct, no serialization aliasing.
// ---------------------------------------------------------------------------

#[test]
fn field_id_is_a_stable_content_address_and_every_component_is_identity() {
    let base = field1_spec();
    assert_eq!(base.field_id(), field1_spec().field_id());
    assert_eq!(base.canonical_bytes(), field1_spec().canonical_bytes());
    assert_eq!(base.field_id().to_string().len(), 64, "a full SHA-256 hex");
    let variants: Vec<FieldSpec> = vec![
        FieldSpec {
            kind: FieldKind::Level1 { n_outer: 3, n0: 2 },
            ..base.clone()
        },
        FieldSpec {
            kind: FieldKind::Level1 { n_outer: 2, n0: 3 },
            ..base.clone()
        },
        FieldSpec {
            kind: FieldKind::Level0 { n0: 2 },
            ..base.clone()
        },
        FieldSpec {
            construction: "level1-modeled-mind-v2".to_string(),
            ..base.clone()
        },
        FieldSpec {
            practical_equivalence: Some(q(1, 100)),
            ..base.clone()
        },
        FieldSpec {
            fallback: "other".to_string(),
            ..base.clone()
        },
        FieldSpec {
            seed_schedule: vec![7],
            ..base.clone()
        },
        FieldSpec {
            tie_rule: TieRule::FirstInPreference,
            ..base.clone()
        },
        FieldSpec {
            policy_library: "field-library-v2".to_string(),
            ..base.clone()
        },
        FieldSpec {
            mode: DecisionMode::Exact,
            ..base.clone()
        },
    ];
    let mut ids = vec![base.field_id()];
    for v in &variants {
        let id = v.field_id();
        assert!(
            !ids.contains(&id),
            "every changed identity component is a new FieldId"
        );
        ids.push(id);
    }
}

#[test]
fn field_serialization_aliases_no_other_serialization_family() {
    let r = receipt();
    let (_, position) = small_root(&r);
    let spec = field1_spec();
    let tuple = focal(
        &position,
        position
            .trick_plays
            .first()
            .copied()
            .unwrap_or(Domino::from_index(0).expect("tile")),
    );
    // The two families open with different fresh headers, so no spec's
    // bytes can equal any freeze tuple's bytes.
    assert_ne!(spec.canonical_bytes(), tuple.tuple().canonical_bytes());
    assert!(spec
        .canonical_bytes()
        .windows(b"walt-field-model-v1".len())
        .any(|w| w == b"walt-field-model-v1"));
    assert!(tuple
        .tuple()
        .canonical_bytes()
        .windows(b"walt-freeze-tuple-v1".len())
        .any(|w| w == b"walt-freeze-tuple-v1"));
}

#[test]
fn the_three_stream_domain_tags_are_pairwise_distinct() {
    assert_ne!(FIELD_DOMAIN, DISCOVERY_DOMAIN);
    assert_ne!(FIELD_DOMAIN, STREAM_DOMAIN);
    assert_ne!(DISCOVERY_DOMAIN, STREAM_DOMAIN);
}

// ---------------------------------------------------------------------------
// Exposure-tier typing (L2-A4, O31, acceptance items 7–9): the tiers are
// mechanically distinct types with mechanically distinct serializations,
// and only RootActionExposureUpper offers the screen's entry point.
// ---------------------------------------------------------------------------

/// The API shape of the future L2-T2..T4 screen: its input type is
/// `RootActionExposureUpper` and nothing else. This function existing (and
/// no analogue existing for `FrozenPolicyExposure`, which has no
/// `screenable_upper`) is the type-level separation this slice fixes.
fn screen_gate(bound: &RootActionExposureUpper) -> &BigRational {
    bound.screenable_upper()
}

#[test]
fn exposure_tiers_are_mechanically_distinct_types_with_distinct_serializations() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = focal(&position, viewer_tiles(&root)[0]);
    let fixed: FrozenPolicyExposure = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &field1,
        WorldDomain::StreamPrefix {
            epoch: 0,
            worlds: 4,
        },
    );
    // The fixed-policy tier serializes under its own mechanical tag and
    // never under the screening tier's.
    let serialized = fixed.to_string();
    assert!(serialized.starts_with("FrozenPolicyExposure{"));
    assert!(!serialized.contains("RootActionExposureUpper"));
    assert!(
        !serialized.contains("rung="),
        "no rung: not a screening bound"
    );
    // The screening tier always names its derivation rung.
    let upper = RootActionExposureUpper::from_rung(ExposureRung::E2, q(1, 4));
    let serialized = upper.to_string();
    assert!(serialized.starts_with("RootActionExposureUpper{rung=E2;"));
    assert_eq!(screen_gate(&upper), &q(1, 4));
    assert_eq!(upper.rung(), ExposureRung::E2);
    // A degenerate bound of 1 is lawful (the naive survivor set); a bound
    // outside [0,1] is not a probability and is rejected.
    let _ = RootActionExposureUpper::from_rung(ExposureRung::E1, q(1, 1));
}

#[test]
#[should_panic(expected = "[0, 1]")]
fn a_root_action_exposure_upper_outside_the_unit_interval_is_rejected() {
    let _ = RootActionExposureUpper::from_rung(ExposureRung::E1, q(3, 2));
}

// ---------------------------------------------------------------------------
// Insert-only field cache semantics (O29's deterministic-replay route).
// ---------------------------------------------------------------------------

#[test]
fn the_field_action_cache_is_insert_only_across_further_replay() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = focal(&position, viewer_tiles(&root)[0]);
    // Materialize a prefix of the stream, snapshot, then enumerate the
    // whole fiber: every earlier entry must survive unchanged.
    let _ = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &field1,
        WorldDomain::StreamPrefix {
            epoch: 0,
            worlds: 5,
        },
    );
    let snap0 = field0.cache_snapshot();
    let snap1 = field1.cache_snapshot();
    assert!(!snap0.is_empty() || !snap1.is_empty());
    let _ = frozen_policy_exposure(
        &root,
        &position,
        &rho,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    let after0 = field0.cache_snapshot();
    let after1 = field1.cache_snapshot();
    for (key, action) in &snap0 {
        assert_eq!(after0.get(key), Some(action), "σ0 entries never change");
    }
    for (key, action) in &snap1 {
        assert_eq!(after1.get(key), Some(action), "σ1 entries never change");
    }
    assert!(after0.len() >= snap0.len());
    assert!(after1.len() >= snap1.len());
}

// ---------------------------------------------------------------------------
// Field-to-field comparison is the frontier membership test (parent §3).
// ---------------------------------------------------------------------------

#[test]
fn field_actions_reports_both_models_choices_at_one_state() {
    let r = receipt();
    let (root, position) = small_root(&r);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: &position,
        history: &[],
    };
    // Query at the root state with the viewer's own hand (the state is a
    // lawful information state for whoever holds these tiles).
    let hand = root.kernel().viewer_hand();
    let (a0, a1) = field_actions(position.decl, hand, hand, &record, &field0, &field1);
    assert_eq!(a0, field0.choose(position.decl, hand, hand, &record));
    assert_eq!(a1, field1.choose(position.decl, hand, hand, &record));
    // The key type rejects an inconsistent hand (a tile already played).
    let played = position
        .prior_played
        .iter()
        .next()
        .expect("trick 6 has prior plays");
    let mut bad = hand;
    assert!(bad.insert(played));
    let result = std::panic::catch_unwind(|| FieldStateKey::from_public(bad, &record));
    assert!(result.is_err(), "an inconsistent hand is rejected");
}
