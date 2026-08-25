//! EXPLORATORY FIELD-SWAP CANCELLATION INSTRUMENT — slice 3 [L2 thread]
//! — sits below every evidentiary tier and is cited by nothing above it.
//! Instrument output only: fixed-policy cancellation ladders (d, r, c⁺,
//! c⁻, c), pairwise (B, H, q, g) masses with dominance labels, pair
//! lifts Λ, directional rungs R⁺/R⁻/R^outcome beside E4, directional and
//! symmetric screens at the frozen and exact-root tiers, Stage-4
//! survivor-only σ1 work, sampled E3 estimates, and first-split
//! aggregates. Never a play-strength claim.
//!
//! Mathematical source: Part VI of the x:019–023 panel response
//! (`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`
//! §§31–42), adopted by rulings PANEL-A7/A8 (`walt/CENSUS-RULINGS.md`).
//! The interpretation rule (§42) is binding on every record here:
//! cancellation may justify a value statement under one declared
//! objective, belief, and model — never pathwise safety, structural
//! irrelevance, dominance, or reweighting stability.
//!
//! DECLARED (σ0, σ1) EPOCH PAIR — the slice-2 probe epoch's, unchanged:
//! σ0 = Level0 { n0 = 8 }, σ1 = Level1 { n_outer = 4, n0 = 2 }, frozen
//! focal candidates at declared schedule [8, 2]. Declared ε for the
//! EpsilonEquivalent label: 1/20 (a probe-epoch declaration, carried on
//! the root record). Roots: the three exact parity roots — receipt-h7-t5
//! (fiber 1680), receipt-h8-t4 (fiber 1200), receipt-h4-t6 (fiber 90).
//!
//! KNOWN CHECK VALUE: at default knobs the committed h8-t4 frozen-pair
//! lift Λ(pin-5-5, pin-3-3) = 31/1200 (corrected 2026-08-24 from a
//! 41/1200 mis-addition by the x:019–023 response §32) is asserted, not
//! just recorded.
//!
//! Mode: `fieldswap_cancel run <out.jsonl> [knobs]`. Knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!   stream_worlds
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::exposure::{
    directional_reach, exact_root_value, exact_split_reach, frozen_policy_exposure,
    sampled_split_reach, WorldDomain,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    correction_pivotal_evidence, exact_frozen_action_values, exact_pairwise_masses,
    fixed_policy_cancellation_kind, pair_lift, survivor_stage4, ActionBound,
    ActionDirectionalUpper, ActionExposureUpper, AdmissibleScreen, BaselineTier,
    CancellationLadder, DirectionalScreen, ExactFrozenBaseline, SplitAggregate,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

// ---------------------------------------------------------------------------
// Configuration — the declared epoch pair (slice-2 probe epoch, unchanged).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    n0_field0: u64,
    n_outer_field1: u64,
    n0_field1: u64,
    n_outer_frozen: u64,
    n0_frozen: u64,
    stream_worlds: u64,
}

impl Config {
    fn is_default(&self) -> bool {
        self.n0_field0 == 8
            && self.n_outer_field1 == 4
            && self.n0_field1 == 2
            && self.n_outer_frozen == 8
            && self.n0_frozen == 2
    }
}

fn field0_spec(cfg: Config) -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: cfg.n0_field0 },
        construction: "level0-modeled-mind-v1 (Solver::modeled_choice; \
                       frozen INNER_SEED belief worlds)"
            .to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn field1_spec(cfg: Config) -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 {
            n_outer: cfg.n_outer_field1,
            n0: cfg.n0_field1,
        },
        construction: "level1-modeled-mind-v1 (solver::level1_evaluate; \
                       saturation-tie refinement 4x per round capped at 16x; \
                       per-state FIELD_DOMAIN seed)"
            .to_string(),
        practical_equivalence: None,
        fallback: "none (no wall-clock cutoff)".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn focal_tuple(position: &RootPosition, cfg: Config, pinned: Domino) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-level1-continuation-v1 (solver::level1_evaluate; \
                        saturation-tie refinement 4x per round capped at 16x)"
            .to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![cfg.n_outer_frozen, cfg.n0_frozen]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    }
}

/// The declared ε for the EpsilonEquivalent label — a probe-epoch
/// declaration, carried on every root record.
fn declared_epsilon() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(20))
}

// ---------------------------------------------------------------------------
// Roots — the three exact parity roots.
// ---------------------------------------------------------------------------

/// (hand, trick).
const ROOTS: [(usize, usize); 3] = [(7, 5), (8, 4), (4, 6)];

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

// ---------------------------------------------------------------------------
// JSON helpers (hand-rolled, like the sibling probes — no serde).
// ---------------------------------------------------------------------------

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn set_json(s: DominoSet) -> String {
    let tiles: Vec<String> = s.iter().map(tile_json).collect();
    format!("[{}]", tiles.join(","))
}

fn micros(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_micros()).expect("a run fits in u64 microseconds")
}

fn values_json(baseline: &ExactFrozenBaseline) -> String {
    let parts: Vec<String> = baseline
        .actions
        .iter()
        .zip(&baseline.values)
        .map(|(a, v)| format!("{{\"action\":{},\"value\":\"{}\"}}", tile_json(*a), v))
        .collect();
    format!("[{}]", parts.join(","))
}

// ---------------------------------------------------------------------------
// The per-root run.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn run_root(r: &Receipt, hand_id: usize, trick_no: usize, cfg: Config) -> Vec<String> {
    let name = format!("receipt-h{hand_id}-t{trick_no}");
    let (root, position) = root_at(r, hand_id, trick_no);
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    let root_id = root_identity(&root, &position);
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let eps = declared_epsilon();
    let mut records: Vec<String> = Vec::new();
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\
         \"declaring_team\":{},\"viewer\":{},\"viewer_hand\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"legal\":{},\
         \"epoch_pair\":{{\"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}}}},\
         \"frozen_schedule\":[{},{}],\"declared_epsilon\":\"{}\",\
         \"stream_worlds\":{}}}",
        name,
        position.decl,
        position.bid,
        position.declaring_team.index(),
        root.kernel().viewer().index(),
        set_json(root.kernel().viewer_hand()),
        root.count(),
        root_id,
        set_json(legal),
        field0.field_id(),
        cfg.n0_field0,
        field1.field_id(),
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        eps,
        cfg.stream_worlds,
    ));
    // The frozen candidate family: one pinned candidate per legal action,
    // frozen once, reused across every pass (acceptance item 3).
    let policies: Vec<FrozenPolicy> = actions
        .iter()
        .map(|a| FrozenPolicy::new(focal_tuple(&position, cfg, *a)))
        .collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    // Stage 1 — exact frozen-set baselines under both fields (the σ1 pass
    // doubles as the sandwich/parity oracle for the records below).
    let started = Instant::now();
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field0, "cancel-sigma0");
    let baseline1 =
        exact_frozen_action_values(&root, &position, &candidates, &field1, "cancel-sigma1");
    let micros_baselines = micros(started);
    for (label, baseline) in [("sigma0", &baseline0), ("sigma1", &baseline1)] {
        records.push(format!(
            "{{\"kind\":\"baseline\",\"root\":\"{}\",\"field\":\"{}\",\
             \"tier\":\"exact-frozen-set\",\"fiber\":\"{}\",\"values\":{}}}",
            name,
            label,
            baseline.fiber,
            values_json(baseline),
        ));
    }
    // Per-action: cancellation ladders, split aggregates, directional
    // rungs beside E4, sandwich, sampled E3.
    let started = Instant::now();
    let mut ladders: Vec<CancellationLadder> = Vec::new();
    let mut e4_bounds: Vec<ActionExposureUpper> = Vec::new();
    let mut directional_bounds: Vec<ActionDirectionalUpper> = Vec::new();
    for (action, rho) in &candidates {
        let exposure = frozen_policy_exposure(
            &root,
            &position,
            rho,
            &field0,
            &field1,
            WorldDomain::ExactFiber,
        );
        let ladder = CancellationLadder::from_exposure(&exposure);
        let kind = fixed_policy_cancellation_kind(&ladder, Some(&eps));
        records.push(format!(
            "{{\"kind\":\"ladder\",\"root\":\"{}\",\"action\":{},\"policy\":\"{}\",\
             \"domain\":\"exact-fiber\",\"worlds\":{},\"exposed\":{},\
             \"outcome_changed\":{},\"c_plus\":{},\"c_minus\":{},\
             \"d\":\"{}\",\"r\":\"{}\",\"c\":\"{}\",\"label\":\"{}\",\
             \"pivotal_evidence\":\"{}\"}}",
            name,
            tile_json(*action),
            rho.policy_id(),
            ladder.worlds,
            ladder.exposed,
            ladder.outcome_changed,
            ladder.c_plus,
            ladder.c_minus,
            ladder.d(),
            ladder.r(),
            ladder.c(),
            kind,
            correction_pivotal_evidence(&ladder),
        ));
        let aggregate = SplitAggregate::from_exposure(&exposure);
        let by_trick: Vec<String> = aggregate
            .by_trick
            .iter()
            .map(|(t, n)| format!("{{\"trick\":{t},\"splits\":{n}}}"))
            .collect();
        records.push(format!(
            "{{\"kind\":\"split_aggregate\",\"root\":\"{}\",\"action\":{},\
             \"exposed\":{},\"plus\":{},\"minus\":{},\
             \"by_seat\":[{},{},{},{}],\"by_trick\":[{}],\
             \"conditional_outcome_difference\":{}}}",
            name,
            tile_json(*action),
            aggregate.exposed,
            aggregate.plus,
            aggregate.minus,
            aggregate.by_seat[0],
            aggregate.by_seat[1],
            aggregate.by_seat[2],
            aggregate.by_seat[3],
            by_trick.join(","),
            aggregate
                .conditional_outcome_difference()
                .map_or("null".to_string(), |d| format!("\"{d}\"")),
        ));
        ladders.push(ladder);
        // The rung ladder beside E4: exact split reach and the coupled
        // directional solve on the same walk.
        let started_dir = Instant::now();
        let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
        let micros_e4 = micros(started_dir);
        let started_dir = Instant::now();
        let dir = directional_reach(&root, &position, *action, &field0, &field1);
        let micros_dir = micros(started_dir);
        // The shared walk agrees with the independent E4 producer exactly.
        assert_eq!(
            dir.exposure_worlds, solve.frontier_worlds,
            "the directional walk's split-reach optimum IS rung E4's"
        );
        records.push(format!(
            "{{\"kind\":\"directional\",\"root\":\"{}\",\"action\":{},\
             \"fiber\":\"{}\",\"plus_worlds\":{},\"minus_worlds\":{},\
             \"outcome_worlds\":{},\"exposure_worlds\":{},\
             \"plus\":\"{}\",\"minus\":\"{}\",\"outcome\":\"{}\",\
             \"e4_r\":\"{}\",\"micros_e4\":{},\"micros_directional\":{}}}",
            name,
            tile_json(*action),
            dir.fiber,
            dir.plus_worlds,
            dir.minus_worlds,
            dir.outcome_worlds,
            dir.exposure_worlds,
            dir.plus_upper(),
            dir.minus_upper(),
            dir.outcome_upper(),
            solve.r(),
            micros_e4,
            micros_dir,
        ));
        // The frozen-tier sandwich record: V0 − (R⁻)^U ≤ V1 ≤ V0 + (R⁺)^U,
        // asserted, not just printed.
        let v0 = baseline0.value(*action);
        let v1 = baseline1.value(*action);
        let low = v0 - dir.minus_upper();
        let high = v0 + dir.plus_upper();
        assert!(
            low <= *v1 && *v1 <= high,
            "the directional sandwich holds at {action} on {name}"
        );
        records.push(format!(
            "{{\"kind\":\"sandwich\",\"root\":\"{}\",\"tier\":\"exact-frozen-set\",\
             \"action\":{},\"v0\":\"{}\",\"v1\":\"{}\",\"low\":\"{}\",\"high\":\"{}\"}}",
            name,
            tile_json(*action),
            v0,
            v1,
            low,
            high,
        ));
        e4_bounds.push(ActionExposureUpper {
            action: *action,
            bound: solve.e4_upper(),
        });
        directional_bounds.push(ActionDirectionalUpper {
            action: *action,
            bound: dir.directional_upper(),
        });
        // The sampled E3 sibling — an estimate record, mechanically
        // labeled as such; it feeds nothing.
        let sampled = sampled_split_reach(
            &root,
            &position,
            *action,
            &field0,
            &field1,
            0,
            cfg.stream_worlds,
        );
        records.push(format!(
            "{{\"kind\":\"e3_sampled\",\"root\":\"{}\",\"action\":{},\
             \"tier\":\"estimate\",\"domain\":\"{}\",\"frontier_worlds\":{},\
             \"estimate\":\"{}\"}}",
            name,
            tile_json(*action),
            sampled.domain(),
            sampled.frontier_worlds,
            sampled.estimate(),
        ));
    }
    let micros_actions = micros(started);
    // Pairwise (B, H, q, g) masses for every ordered pair under each
    // field, with the dominance label; pair lifts Λ per ordered pair.
    let started = Instant::now();
    for (label, field) in [("sigma0", &field0), ("sigma1", &field1)] {
        for (a, rho_a) in &candidates {
            for (b, rho_b) in &candidates {
                if a == b {
                    continue;
                }
                let masses = exact_pairwise_masses(&root, &position, *rho_a, *rho_b, field);
                records.push(format!(
                    "{{\"kind\":\"pairwise\",\"root\":\"{}\",\"field\":\"{}\",\
                     \"a\":{},\"b\":{},\"benefit\":{},\"hazard\":{},\
                     \"both_make\":{},\"both_fail\":{},\
                     \"B\":\"{}\",\"H\":\"{}\",\"q\":\"{}\",\"g\":\"{}\",\
                     \"label\":\"{}\"}}",
                    name,
                    label,
                    tile_json(*a),
                    tile_json(*b),
                    masses.benefit_worlds(),
                    masses.hazard_worlds(),
                    masses.both_make_worlds(),
                    masses.both_fail_worlds(),
                    masses.b(),
                    masses.h(),
                    masses.q(),
                    masses.g(),
                    masses.dominance_kind(),
                ));
            }
        }
    }
    for (i, (a, _)) in candidates.iter().enumerate() {
        for (j, (b, _)) in candidates.iter().enumerate() {
            if i == j {
                continue;
            }
            let lift = pair_lift(&ladders[i], &ladders[j]);
            records.push(format!(
                "{{\"kind\":\"pair_lift\",\"root\":\"{}\",\"a\":{},\"b\":{},\
                 \"lambda\":\"{}\",\"bound\":\"{}\"}}",
                name,
                tile_json(*a),
                tile_json(*b),
                lift.lambda,
                lift.bound,
            ));
            // The committed check value (README correction trail): at the
            // default knobs, Λ(pin-5-5, pin-3-3) on receipt-h8-t4 is
            // 31/1200 — asserted, since the components are exact counts.
            if cfg.is_default()
                && hand_id == 8
                && trick_no == 4
                && tile_json(*a) == "[5,5]"
                && tile_json(*b) == "[3,3]"
            {
                assert_eq!(
                    lift.lambda,
                    BigRational::new(BigInt::from(31), BigInt::from(1200)),
                    "the corrected h8-t4 frozen-pair lift is 31/1200"
                );
                eprintln!("fieldswap_cancel: {name} Λ(5-5, 3-3) = 31/1200 CHECK PASSED");
            }
        }
    }
    let micros_pairwise = micros(started);
    // Screens at the frozen tier: symmetric (E4 bounds) and directional;
    // the directional admissible set is asserted a subset.
    let symmetric = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &baseline0.point_bounds(),
        &e4_bounds,
        field0.field_id(),
        field1.field_id(),
        root_id,
    );
    let directional = DirectionalScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &baseline0.point_bounds(),
        &directional_bounds,
        field0.field_id(),
        field1.field_id(),
        root_id,
    );
    let admitted_sym = symmetric.admissible();
    let admitted_dir = directional.admissible();
    for action in &admitted_dir {
        assert!(
            admitted_sym.contains(action),
            "the directional screen only ever prunes MORE"
        );
    }
    // Exclusion soundness replayed with exact σ1 numbers on every screen.
    let best1 = actions
        .iter()
        .map(|a| baseline1.value(*a).clone())
        .max()
        .expect("actions");
    for (screen_label, admitted) in [("e4", &admitted_sym), ("directional", &admitted_dir)] {
        for action in &actions {
            if !admitted.contains(action) {
                assert!(
                    *baseline1.value(*action) < best1,
                    "{screen_label}: excluded {action} must be strictly σ1-nonoptimal"
                );
            }
        }
    }
    for (label, kind, bar, admitted, serialized) in [
        (
            "e4",
            symmetric.kind(),
            symmetric.bar(),
            &admitted_sym,
            symmetric.to_string(),
        ),
        (
            "directional",
            directional.kind(),
            directional.bar(),
            &admitted_dir,
            directional.to_string(),
        ),
    ] {
        records.push(format!(
            "{{\"kind\":\"screen\",\"root\":\"{}\",\"bounds\":\"{}\",\
             \"tier\":\"exact-frozen-set\",\"result\":\"{}\",\"bar\":\"{}\",\
             \"legal_count\":{},\"admitted_count\":{},\"admissible\":[{}],\
             \"serialized\":\"{}\"}}",
            name,
            label,
            kind,
            bar,
            actions.len(),
            admitted.len(),
            admitted
                .iter()
                .map(|a| tile_json(*a))
                .collect::<Vec<_>>()
                .join(","),
            serialized,
        ));
    }
    // §36 — the directional winner-stability table, with the implication
    // asserted against the exact σ1 values wherever the premise holds.
    let stability: Vec<String> = directional
        .slack_table()
        .iter()
        .map(|s| {
            let stable = s.slack > BigRational::new(BigInt::from(0), BigInt::from(1));
            if stable {
                assert!(
                    baseline1.value(s.a) > baseline1.value(s.b),
                    "§36: a positive directional slack survives the swap"
                );
            }
            format!(
                "{{\"a\":{},\"b\":{},\"slack\":\"{}\",\"winner_stable\":{}}}",
                tile_json(s.a),
                tile_json(s.b),
                s.slack,
                stable
            )
        })
        .collect();
    records.push(format!(
        "{{\"kind\":\"winner_stability\",\"root\":\"{}\",\"tier\":\"exact-frozen-set\",\
         \"pairs\":[{}]}}",
        name,
        stability.join(","),
    ));
    // Stage 4 — σ1 work confined to the symmetric screen's survivors.
    let started = Instant::now();
    let stage4 = survivor_stage4(
        &root,
        &position,
        &symmetric,
        &baseline0,
        &candidates,
        &field1,
        "cancel-stage4",
    );
    let micros_stage4 = micros(started);
    records.push(format!(
        "{{\"kind\":\"stage4\",\"root\":\"{}\",\"survivors\":[{}],\
         \"sigma1_work\":\"{}\",\"settled0\":{},\"selected1\":{},\
         \"decision_changed\":{},\"result\":\"{}\",\"micros\":{}}}",
        name,
        stage4
            .survivors
            .iter()
            .map(|a| tile_json(*a))
            .collect::<Vec<_>>()
            .join(","),
        match &stage4.values1 {
            None => "none (singleton admissible set)".to_string(),
            Some(v) => format!("survivor-only, {} actions", v.actions.len()),
        },
        tile_json(stage4.settled0),
        tile_json(stage4.selected1),
        stage4.decision_changed(),
        stage4.kind,
        micros_stage4,
    ));
    // The ExactRoot tier: exact optimized Q under each field, the
    // exact-root sandwich, and the directional screen at that tier.
    let started = Instant::now();
    let mut root_bounds0: Vec<ActionBound> = Vec::new();
    for (k, action) in actions.iter().enumerate() {
        let q0 = exact_root_value(&root, &position, *action, &field0);
        let q1 = exact_root_value(&root, &position, *action, &field1);
        let dir = &directional_bounds[k];
        let low = q0.value() - dir.bound.screenable_minus();
        let high = q0.value() + dir.bound.screenable_plus();
        assert!(
            low <= q1.value() && q1.value() <= high,
            "the exact-root sandwich holds at {action} on {name}"
        );
        // The optimizer dominates its own frozen candidate under each
        // field (both are information-consistent continuations).
        assert!(q0.value() >= *baseline0.value(*action));
        assert!(q1.value() >= *baseline1.value(*action));
        records.push(format!(
            "{{\"kind\":\"exact_root\",\"root\":\"{}\",\"action\":{},\
             \"q0\":\"{}\",\"q1\":\"{}\",\"q0_wins\":{},\"q1_wins\":{},\
             \"sandwich_low\":\"{}\",\"sandwich_high\":\"{}\"}}",
            name,
            tile_json(*action),
            q0.value(),
            q1.value(),
            q0.win_worlds,
            q1.win_worlds,
            low,
            high,
        ));
        root_bounds0.push(ActionBound {
            action: *action,
            lower: q0.value(),
            upper: q0.value(),
        });
    }
    let micros_exact_root = micros(started);
    let root_screen = DirectionalScreen::compute(
        legal,
        BaselineTier::ExactRoot,
        &root_bounds0,
        &directional_bounds,
        field0.field_id(),
        field1.field_id(),
        root_id,
    );
    let admitted_root = root_screen.admissible();
    records.push(format!(
        "{{\"kind\":\"screen\",\"root\":\"{}\",\"bounds\":\"directional\",\
         \"tier\":\"exact-root\",\"result\":\"{}\",\"bar\":\"{}\",\
         \"legal_count\":{},\"admitted_count\":{},\"admissible\":[{}],\
         \"serialized\":\"{}\"}}",
        name,
        root_screen.kind(),
        root_screen.bar(),
        actions.len(),
        admitted_root.len(),
        admitted_root
            .iter()
            .map(|a| tile_json(*a))
            .collect::<Vec<_>>()
            .join(","),
        root_screen,
    ));
    // §12.1 — the cost note, plain integer microseconds.
    records.push(format!(
        "{{\"kind\":\"cost\",\"root\":\"{}\",\"micros_baselines\":{},\
         \"micros_ladders_rungs_e3\":{},\"micros_pairwise\":{},\
         \"micros_stage4\":{},\"micros_exact_root\":{}}}",
        name, micros_baselines, micros_actions, micros_pairwise, micros_stage4, micros_exact_root,
    ));
    eprintln!(
        "fieldswap_cancel: {name} done: legal={} admissible(e4)={} admissible(dir)={} \
         admissible(exact-root)={} (baselines {}us, ladders+rungs {}us, pairwise {}us, \
         exact-root {}us)",
        actions.len(),
        admitted_sym.len(),
        admitted_dir.len(),
        admitted_root.len(),
        micros_baselines,
        micros_actions,
        micros_pairwise,
        micros_exact_root,
    );
    records
}

fn run(out_path: &str, cfg: Config) {
    let r = receipt();
    eprintln!(
        "fieldswap_cancel: {} roots; declared epoch pair field0 n0={}, field1 {}x{}, \
         frozen {}x{}, stream {}",
        ROOTS.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.stream_worlds,
    );
    #[cfg(feature = "parallel")]
    let per_root: Vec<Vec<String>> = ROOTS
        .par_iter()
        .map(|(hand_id, trick_no)| run_root(&r, *hand_id, *trick_no, cfg))
        .collect();
    #[cfg(not(feature = "parallel"))]
    let per_root: Vec<Vec<String>> = ROOTS
        .iter()
        .map(|(hand_id, trick_no)| run_root(&r, *hand_id, *trick_no, cfg))
        .collect();
    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    for records in per_root {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("fieldswap_cancel: wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("run");
    match mode {
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "fieldswap_cancel.jsonl".to_string());
            let mut knobs = args.iter().skip(3);
            let mut knob = |default: u64| -> u64 {
                knobs
                    .next()
                    .map(|v| v.parse().expect("an integer knob"))
                    .unwrap_or(default)
            };
            let cfg = Config {
                n0_field0: knob(8),
                n_outer_field1: knob(4),
                n0_field1: knob(2),
                n_outer_frozen: knob(8),
                n0_frozen: knob(2),
                stream_worlds: knob(64),
            };
            run(&out_path, cfg);
        }
        other => panic!("unknown mode {other:?}; expected run"),
    }
}
