//! EXPLORATORY FIELD-SWAP SMOKE INSTRUMENT (§21 step 5 of the targeted
//! level-2 build program, `walt/math/targeted_level2_field_stability_v0.1.md`;
//! rulings L2-A1..A7) — sits below every evidentiary tier and is cited by
//! nothing above it. Detector work only: fixed-policy exposure and
//! correction on reconstructable roots, never a root-action bound, never a
//! play-strength claim.
//!
//! The Gran anchor seeds are not yet reconstructed
//! (`kanban/backlog/gran-anchor-reconstruction.md`), so the smoke runs on
//! roots reconstructable from the step-7 shadow run instead: the driven
//! scenario's trick-1 root (trump fives, P30 by T1, S1 the bidder holding
//! the receipt-hand-8 tiles — S1 holds 5-5, the ten-count trump) and
//! receipt-hand roots where the bidder leads a small fiber holding a
//! countable trump it could reveal or retain. For each root, two focal
//! policies are frozen (`ActionRule::PinnedThenLevel1` with contrasting
//! pins: reveal the counter now versus retain it) and their coupled
//! (σ0, σ1) exposure is computed over a declared world set.
//!
//! Modes:
//!   `fieldswap scan` — list receipt roots where the bidder leads (the
//!       pinned root information state is the bidder's), with fiber sizes
//!       and the counters the bidder holds.
//!   `fieldswap run <out.jsonl> [knobs]` — the smoke. Knobs (positional):
//!       n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!       stream_worlds
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::replay::state_before_trick;
use walt::rules::rules::legal_plays;
use walt::rules::{Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::solver::adaptive::{
    driven_root, root_identity, CanonicalRoot, DrivenState, RootPosition,
};
use walt::solver::exposure::{frozen_policy_exposure, FrozenPolicyExposure, WorldDomain, WorldRow};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

// ---------------------------------------------------------------------------
// Configuration.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    /// σ0's declared belief-world count (the shadow drive field's n0).
    n0_field0: u64,
    /// σ1's declared level-1 inner schedule.
    n_outer_field1: u64,
    n0_field1: u64,
    /// The frozen focal policies' declared schedule (identity fields of
    /// every candidate's FreezeTuple, CE-A5).
    n_outer_frozen: u64,
    n0_frozen: u64,
    /// Declared stream-prefix size for roots whose fiber is too large to
    /// enumerate.
    stream_worlds: u64,
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

// ---------------------------------------------------------------------------
// Roots.
// ---------------------------------------------------------------------------

/// The playout/shadow scenario's frozen S1 hand (receipt hand 8's S1
/// tiles) — the driven trick-1 root's viewer hand.
fn s1_scenario_hand() -> DominoSet {
    let p = |v: u8| Pip::new(v).expect("pip");
    [
        Domino::new(p(5), p(2)),
        Domino::new(p(5), p(4)),
        Domino::new(p(1), p(1)),
        Domino::new(p(2), p(1)),
        Domino::new(p(3), p(1)),
        Domino::new(p(3), p(3)),
        Domino::new(p(5), p(5)),
    ]
    .into_iter()
    .collect()
}

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// One smoke root: a name, the canonical objects, the declared world
/// domain, and the two contrasting pins.
struct SmokeRoot {
    name: String,
    root: CanonicalRoot,
    position: RootPosition,
    domain: WorldDomain,
    /// (label, pinned tile) — reveal-shaped first, retain-shaped second.
    pins: [(String, Domino); 2],
}

/// The driven scenario's trick-1 root: S1 (the bidder) to lead, empty
/// record, fiber C(21,7)·C(14,7). Reveal pin = 5-5 (the ten-count trump);
/// retain pin = 3-3 (an off-suit double, no count).
fn driven_trick1_root(cfg: Config) -> SmokeRoot {
    let p = |v: u8| Pip::new(v).expect("pip");
    let decl = Decl::PipTrump(p(5));
    let hand = s1_scenario_hand();
    let state = DrivenState {
        decl,
        bid: 30,
        declaring_team: Team::T1,
        viewer_hand: hand,
        leader: Seat::S1,
        trick_plays: &[],
        banked: [0, 0],
        prior_played: DominoSet::EMPTY,
        voids: [walt::rules::ContextSet::EMPTY; 4],
    };
    let (root, position) = driven_root(&state).expect("the scenario root has a lawful kernel");
    SmokeRoot {
        name: "driven-h0-t1".to_string(),
        root,
        position,
        domain: WorldDomain::StreamPrefix {
            epoch: 0,
            worlds: cfg.stream_worlds,
        },
        pins: [
            ("reveal-5-5".to_string(), Domino::new(p(5), p(5))),
            ("retain-3-3".to_string(), Domino::new(p(3), p(3))),
        ],
    }
}

/// A receipt root at a trick start where the bidder leads: the pinned
/// root information state is the bidder's own decision.
fn receipt_root(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    reveal: (u8, u8),
    retain: (u8, u8),
) -> SmokeRoot {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let (hands, leader) = state_before_trick(hand, trick_no).expect("a valid trick boundary");
    assert_eq!(leader, hand.bidder, "the bidder leads this root");
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    let p = |v: u8| Pip::new(v).expect("pip");
    let tile = |t: (u8, u8)| Domino::new(p(t.0), p(t.1));
    let bidder_hand = hands[hand.bidder.index()];
    for t in [tile(reveal), tile(retain)] {
        assert!(bidder_hand.contains(t), "a pinned tile is held at the root");
    }
    SmokeRoot {
        name: format!("receipt-h{hand_id}-t{trick_no}"),
        root: CanonicalRoot::new(kernel),
        position,
        domain: WorldDomain::ExactFiber,
        pins: [
            (format!("reveal-{}-{}", reveal.0, reveal.1), tile(reveal)),
            (format!("retain-{}-{}", retain.0, retain.1), tile(retain)),
        ],
    }
}

// ---------------------------------------------------------------------------
// Scan mode.
// ---------------------------------------------------------------------------

fn scan() {
    let r = receipt();
    println!("hand trick decl bid team bidder fiber  bidder-hand (count points; * = trump)");
    for hand in &r.hands {
        for trick_no in 2..=6usize {
            let Ok((hands, leader)) = state_before_trick(hand, trick_no) else {
                continue;
            };
            if leader != hand.bidder {
                continue;
            }
            let Ok(kernel) = Kernel::from_receipt_trick(hand, trick_no) else {
                continue;
            };
            let root = CanonicalRoot::new(kernel);
            let tiles: Vec<String> = hands[hand.bidder.index()]
                .iter()
                .map(|d| {
                    let trump = match hand.decl {
                        Decl::PipTrump(p) => d.has(p),
                        Decl::DoublesTrump => d.is_double(),
                        Decl::NoTrump => false,
                    };
                    format!(
                        "{}-{}{}{}",
                        d.hi().value(),
                        d.lo().value(),
                        if d.count() > 0 {
                            format!("({})", d.count())
                        } else {
                            String::new()
                        },
                        if trump { "*" } else { "" }
                    )
                })
                .collect();
            println!(
                "{:4} {:5} {:4} {:3} {:4} {:6} {:6}  {}",
                hand.id,
                trick_no,
                hand.decl.to_string(),
                hand.bid_points,
                hand.declaring_team.index(),
                hand.bidder.index(),
                root.count(),
                tiles.join(" ")
            );
        }
    }
}

// ---------------------------------------------------------------------------
// JSON helpers (hand-rolled, like shadow.rs — no serde in the workspace).
// ---------------------------------------------------------------------------

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn tiles_json(ds: &[Domino]) -> String {
    let parts: Vec<String> = ds.iter().map(|d| tile_json(*d)).collect();
    format!("[{}]", parts.join(","))
}

fn set_json(s: DominoSet) -> String {
    let tiles: Vec<Domino> = s.iter().collect();
    tiles_json(&tiles)
}

fn micros(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_micros()).expect("a run fits in u64 microseconds")
}

fn world_row_json(
    root_name: &str,
    pin_label: &str,
    policy: &FrozenPolicy,
    row: &WorldRow,
) -> String {
    let split_json = row.split.as_ref().map_or("null".to_string(), |s| {
        format!(
            "{{\"seat\":{},\"trick\":{},\"ply\":{},\"tile0\":{},\"tile1\":{},\
             \"hand\":{},\"history\":{}}}",
            s.seat.index(),
            s.trick,
            s.ply,
            tile_json(s.tile0),
            tile_json(s.tile1),
            set_json(s.hand),
            tiles_json(&s.history),
        )
    });
    format!(
        "{{\"kind\":\"world\",\"root\":\"{}\",\"policy\":\"{}\",\"policy_id\":\"{}\",\
         \"index\":{},\"world\":[{},{},{},{}],\"d\":{},\"u0\":{},\"u1\":{},\"split\":{}}}",
        root_name,
        pin_label,
        policy.policy_id(),
        row.index,
        row.world[0],
        row.world[1],
        row.world[2],
        row.world[3],
        u8::from(row.split.is_some()),
        row.u0,
        row.u1,
        split_json,
    )
}

fn exposure_json(
    root_name: &str,
    pin_label: &str,
    pinned: Domino,
    exposure: &FrozenPolicyExposure,
    wall_us: u64,
) -> String {
    format!(
        "{{\"kind\":\"policy\",\"tier\":\"FrozenPolicyExposure\",\"root\":\"{}\",\
         \"policy\":\"{}\",\"pinned\":{},\"policy_id\":\"{}\",\"field0\":\"{}\",\
         \"field1\":\"{}\",\"root_id\":\"{:#018x}\",\"domain\":\"{}\",\"worlds\":{},\
         \"exposed\":{},\"c_plus\":{},\"c_minus\":{},\"d_hat\":\"{}\",\"c_hat\":\"{}\",\
         \"bound_ok\":true,\"micros\":{}}}",
        root_name,
        pin_label,
        tile_json(pinned),
        exposure.policy,
        exposure.field0,
        exposure.field1,
        exposure.root_id,
        exposure.domain,
        exposure.worlds,
        exposure.exposed,
        exposure.corrections_plus,
        exposure.corrections_minus,
        exposure.d_hat(),
        exposure.c_hat(),
        wall_us,
    )
}

// ---------------------------------------------------------------------------
// Run mode.
// ---------------------------------------------------------------------------

fn run_root(smoke: &SmokeRoot, cfg: Config) -> Vec<String> {
    let mut records: Vec<String> = Vec::new();
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\
         \"declaring_team\":{},\"viewer\":{},\"viewer_hand\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"domain\":\"{}\",\
         \"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}},\
         \"frozen_schedule\":[{},{}]}}",
        smoke.name,
        smoke.position.decl,
        smoke.position.bid,
        smoke.position.declaring_team.index(),
        smoke.root.kernel().viewer().index(),
        set_json(smoke.root.kernel().viewer_hand()),
        smoke.root.count(),
        root_identity(&smoke.root, &smoke.position),
        smoke.domain,
        field0.field_id(),
        cfg.n0_field0,
        field1.field_id(),
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
    ));
    for (pin_label, pinned) in &smoke.pins {
        let viewer_hand = smoke.root.kernel().viewer_hand();
        assert!(
            viewer_hand.contains(*pinned),
            "a pinned tile is held at the root"
        );
        let led = smoke
            .position
            .trick_plays
            .first()
            .map(|d| smoke.position.decl.led_context(*d));
        assert!(
            legal_plays(smoke.position.decl, viewer_hand, led).contains(*pinned),
            "a pinned tile is legal at the root"
        );
        let focal = FrozenPolicy::new(focal_tuple(&smoke.position, cfg, *pinned));
        let started = Instant::now();
        let exposure = frozen_policy_exposure(
            &smoke.root,
            &smoke.position,
            &focal,
            &field0,
            &field1,
            smoke.domain.clone(),
        );
        let wall_us = micros(started);
        for row in &exposure.rows {
            records.push(world_row_json(&smoke.name, pin_label, &focal, row));
        }
        records.push(exposure_json(
            &smoke.name,
            pin_label,
            *pinned,
            &exposure,
            wall_us,
        ));
        eprintln!(
            "fieldswap: {} {} done: worlds={} exposed={} c=+{}/-{} ({} us)",
            smoke.name,
            pin_label,
            exposure.worlds,
            exposure.exposed,
            exposure.corrections_plus,
            exposure.corrections_minus,
            wall_us
        );
    }
    records
}

fn run(out_path: &str, cfg: Config) {
    let r = receipt();
    let roots: Vec<SmokeRoot> = vec![
        driven_trick1_root(cfg),
        // Chosen by `fieldswap scan` (see probes/fieldswap/README.md):
        // bidder leads, small exact fiber, a countable trump in hand.
        receipt_root(
            &r,
            RECEIPT_ROOT_A.0,
            RECEIPT_ROOT_A.1,
            RECEIPT_ROOT_A.2,
            RECEIPT_ROOT_A.3,
        ),
        receipt_root(
            &r,
            RECEIPT_ROOT_B.0,
            RECEIPT_ROOT_B.1,
            RECEIPT_ROOT_B.2,
            RECEIPT_ROOT_B.3,
        ),
    ];
    eprintln!(
        "fieldswap: {} roots; field0 n0={}, field1 {}x{}, frozen {}x{}, stream_worlds={}",
        roots.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.stream_worlds,
    );
    #[cfg(feature = "parallel")]
    let per_root: Vec<Vec<String>> = roots.par_iter().map(|s| run_root(s, cfg)).collect();
    #[cfg(not(feature = "parallel"))]
    let per_root: Vec<Vec<String>> = roots.iter().map(|s| run_root(s, cfg)).collect();
    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    for records in per_root {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("fieldswap: wrote {out_path}");
}

/// (hand, trick, reveal pin, retain pin) — chosen from `fieldswap scan`:
/// bidder leads, small exact fiber, the ten-count trump 5-5 in hand.
///
/// Root A: receipt hand 8, trick 4 (P5 by T1, S1 the bidder — the same
/// tiles the driven scenario reuses), fiber 1200: reveal 5-5 vs retain 3-3.
const RECEIPT_ROOT_A: (usize, usize, (u8, u8), (u8, u8)) = (8, 4, (5, 5), (3, 3));
/// Root B: receipt hand 7, trick 5 (P5 by T0, S0 the bidder), fiber 1680:
/// reveal 5-5 vs retain 1-0.
const RECEIPT_ROOT_B: (usize, usize, (u8, u8), (u8, u8)) = (7, 5, (5, 5), (1, 0));

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("scan");
    match mode {
        "scan" => scan(),
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "fieldswap.jsonl".to_string());
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
        other => panic!("unknown mode {other:?}; expected scan|run"),
    }
}
