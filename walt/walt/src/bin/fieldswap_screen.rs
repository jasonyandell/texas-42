//! EXPLORATORY FIELD-SWAP SCREEN INSTRUMENT (§21 steps 6–8 of the
//! targeted level-2 build program,
//! `walt/math/targeted_level2_field_stability_v0.1.md`; rulings
//! L2-A1..A7) — sits below every evidentiary tier and is cited by nothing
//! above it. Instrument output only: per-root all-action exposure bounds
//! with their rungs, the admissible level-2 action set against the legal
//! count, the stability-slack table, and a targeted-versus-naive cost note
//! (§12.1). Never a play-strength claim.
//!
//! DECLARED (σ0, σ1) EPOCH PAIR — slice 1's open question resolved by
//! declaration: this probe epoch declares σ0 = Level0 { n0 = 8 } and
//! σ1 = Level1 { n_outer = 4, n0 = 2 } (the same pair slice 1's smoke
//! declared), frozen focal candidates at declared schedule [8, 2]. The σ0
//! inner schedule n0 is a FieldId identity component; both FieldIds ride
//! every record. A different schedule is a different FieldId and a
//! different experiment epoch.
//!
//! Roots: the three exact parity roots of the slice — receipt-h7-t5
//! (fiber 1680, slice 1's zero-split specimen: does rung E0 fire at the
//! root-action level?), receipt-h8-t4 (fiber 1200, slice 1's split-heavy
//! specimen), and receipt-h4-t6 (fiber 90, the small gate root).
//!
//! Mode: `fieldswap_screen run <out.jsonl> [knobs]`. Knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::exposure::{
    clairvoyant_reach, exact_split_reach, rung_e1, ForcedNonFocalCover, RootActionExposureUpper,
    TrivialSplitCover,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{
    exact_frozen_action_values, ActionExposureUpper, AdmissibleScreen, BaselineTier,
    ExactFrozenBaseline,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

// ---------------------------------------------------------------------------
// Configuration — the declared epoch pair.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    n0_field0: u64,
    n_outer_field1: u64,
    n0_field1: u64,
    n_outer_frozen: u64,
    n0_frozen: u64,
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
// Roots — the three exact parity roots.
// ---------------------------------------------------------------------------

/// (name suffix, hand, trick).
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

fn bound_json(bound: &RootActionExposureUpper) -> String {
    format!(
        "{{\"rung\":\"{}\",\"upper\":\"{}\"}}",
        bound.rung(),
        bound.screenable_upper()
    )
}

// ---------------------------------------------------------------------------
// The per-root run: Stage 1 baseline, Stage 2 rung ladder, Stage 3
// screens, the exact σ1 parity pass, and the §12.1 cost note.
// ---------------------------------------------------------------------------

fn run_root(r: &Receipt, hand_id: usize, trick_no: usize, cfg: Config) -> Vec<String> {
    let name = format!("receipt-h{hand_id}-t{trick_no}");
    let (root, position) = root_at(r, hand_id, trick_no);
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    let root_id = root_identity(&root, &position);
    let legal = legal_root_actions(&root, &position);
    let actions: Vec<Domino> = legal.iter().collect();
    let mut records: Vec<String> = Vec::new();
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\
         \"declaring_team\":{},\"viewer\":{},\"viewer_hand\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"legal\":{},\
         \"epoch_pair\":{{\"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}}}},\
         \"frozen_schedule\":[{},{}]}}",
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
    ));
    // The frozen candidate family: one pinned candidate per legal action,
    // frozen once, reused across σ0, σ1, and the rungs (acceptance item 3).
    let policies: Vec<FrozenPolicy> = actions
        .iter()
        .map(|a| FrozenPolicy::new(focal_tuple(&position, cfg, *a)))
        .collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    // Stage 1 — exact frozen-set baseline under σ0.
    let started = Instant::now();
    let baseline0 =
        exact_frozen_action_values(&root, &position, &candidates, &field0, "screen-sigma0");
    let micros_baseline0 = micros(started);
    records.push(format!(
        "{{\"kind\":\"baseline\",\"root\":\"{}\",\"field\":\"sigma0\",\
         \"tier\":\"exact-frozen-set\",\"fiber\":\"{}\",\"values\":{},\"micros\":{}}}",
        name,
        baseline0.fiber,
        values_json(&baseline0),
        micros_baseline0,
    ));
    // Stage 2 — the rung ladder for every legal action. E1 covers are
    // action-independent; E0/E2 come from one reach walk per action; E4
    // from the exact split-reach solve.
    let started = Instant::now();
    let e1_trivial = rung_e1(&root, &TrivialSplitCover);
    let e1_forced = rung_e1(&root, &ForcedNonFocalCover);
    let micros_e1 = micros(started);
    records.push(format!(
        "{{\"kind\":\"exposure\",\"root\":\"{}\",\"action\":null,\
         \"producer\":\"trivial-cover-v1\",\"bound\":{},\"micros\":{}}}",
        name,
        bound_json(&e1_trivial),
        micros_e1,
    ));
    records.push(format!(
        "{{\"kind\":\"exposure\",\"root\":\"{}\",\"action\":null,\
         \"producer\":\"forced-non-focal-cover-v1\",\"bound\":{},\"micros\":0}}",
        name,
        bound_json(&e1_forced),
    ));
    let mut e2_bounds: Vec<ActionExposureUpper> = Vec::new();
    let mut e4_bounds: Vec<ActionExposureUpper> = Vec::new();
    let mut screen_bounds: Vec<ActionExposureUpper> = Vec::new();
    let mut micros_rungs = 0u64;
    for action in &actions {
        let started = Instant::now();
        let reach = clairvoyant_reach(&root, &position, *action, &field0, &field1);
        let micros_walk = micros(started);
        let e0 = reach.e0_upper();
        let e2 = reach.e2_upper();
        records.push(format!(
            "{{\"kind\":\"exposure\",\"root\":\"{}\",\"action\":{},\
             \"producer\":\"clairvoyant-reach-walk\",\"reach_worlds\":{},\"fiber\":\"{}\",\
             \"e0_fires\":{},\"bound\":{},\"micros\":{}}}",
            name,
            tile_json(*action),
            reach.reach_worlds,
            reach.fiber,
            e0.is_some(),
            bound_json(&e2),
            micros_walk,
        ));
        let started = Instant::now();
        let solve = exact_split_reach(&root, &position, *action, &field0, &field1);
        let micros_solve = micros(started);
        records.push(format!(
            "{{\"kind\":\"exposure\",\"root\":\"{}\",\"action\":{},\
             \"producer\":\"exact-split-reach\",\"frontier_worlds\":{},\"fiber\":\"{}\",\
             \"bound\":{},\"micros\":{}}}",
            name,
            tile_json(*action),
            solve.frontier_worlds,
            solve.fiber,
            bound_json(&solve.e4_upper()),
            micros_solve,
        ));
        micros_rungs += micros_walk + micros_solve;
        // §8 Stage 2 — the cheapest sound bound for the screen: E0 when it
        // fires, else the clairvoyant E2 cover (the pre-exact route).
        screen_bounds.push(ActionExposureUpper {
            action: *action,
            bound: e0.clone().unwrap_or_else(|| e2.clone()),
        });
        e2_bounds.push(ActionExposureUpper {
            action: *action,
            bound: e2,
        });
        e4_bounds.push(ActionExposureUpper {
            action: *action,
            bound: solve.e4_upper(),
        });
    }
    // Stage 3 — the admissible set, at the cheapest-rung bounds and at the
    // exact E4 bounds.
    for (label, bounds) in [("cheapest", &screen_bounds), ("e4", &e4_bounds)] {
        let screen = AdmissibleScreen::compute(
            legal,
            BaselineTier::ExactFrozenSet,
            &baseline0.point_bounds(),
            bounds,
            field0.field_id(),
            field1.field_id(),
            root_id,
        );
        let admissible = screen.admissible();
        let slack: Vec<String> = screen
            .slack_table()
            .iter()
            .map(|s| {
                format!(
                    "{{\"a\":{},\"b\":{},\"slack\":\"{}\"}}",
                    tile_json(s.a),
                    tile_json(s.b),
                    s.slack
                )
            })
            .collect();
        records.push(format!(
            "{{\"kind\":\"screen\",\"root\":\"{}\",\"bounds\":\"{}\",\
             \"tier\":\"exact-frozen-set\",\"result\":\"{}\",\"bar\":\"{}\",\
             \"legal_count\":{},\"admitted_count\":{},\"admissible\":[{}],\
             \"slack\":[{}],\"serialized\":\"{}\"}}",
            name,
            label,
            screen.kind(),
            screen.bar(),
            actions.len(),
            admissible.len(),
            admissible
                .iter()
                .map(|a| tile_json(*a))
                .collect::<Vec<_>>()
                .join(","),
            slack.join(","),
            screen,
        ));
    }
    // The exact σ1 parity pass (O32's route, instrument-grade here; the
    // load-bearing gates are the tests): exact Q^(1) for the same frozen
    // candidates, exclusion audit against the E4 screen.
    let started = Instant::now();
    let baseline1 =
        exact_frozen_action_values(&root, &position, &candidates, &field1, "screen-sigma1");
    let micros_sigma1 = micros(started);
    records.push(format!(
        "{{\"kind\":\"baseline\",\"root\":\"{}\",\"field\":\"sigma1\",\
         \"tier\":\"exact-frozen-set\",\"fiber\":\"{}\",\"values\":{},\"micros\":{}}}",
        name,
        baseline1.fiber,
        values_json(&baseline1),
        micros_sigma1,
    ));
    let screen_e4 = AdmissibleScreen::compute(
        legal,
        BaselineTier::ExactFrozenSet,
        &baseline0.point_bounds(),
        &e4_bounds,
        field0.field_id(),
        field1.field_id(),
        root_id,
    );
    let admissible = screen_e4.admissible();
    let best1 = baseline1.values.iter().max().expect("actions").clone();
    let excluded: Vec<Domino> = actions
        .iter()
        .copied()
        .filter(|a| !admissible.contains(a))
        .collect();
    let every_excluded_nonoptimal = excluded.iter().all(|a| *baseline1.value(*a) < best1);
    let l2t2_ok = actions.iter().zip(e4_bounds.iter()).all(|(a, e)| {
        let c = baseline1.value(*a) - baseline0.value(*a);
        let magnitude = if c < BigRational::from_integer(0.into()) {
            -c
        } else {
            c
        };
        magnitude <= *e.bound.screenable_upper()
    });
    assert!(
        every_excluded_nonoptimal && l2t2_ok,
        "the exact parity audit holds on every probe root"
    );
    records.push(format!(
        "{{\"kind\":\"parity\",\"root\":\"{}\",\"excluded\":[{}],\
         \"every_excluded_sigma1_nonoptimal\":{},\"l2t2_ok\":{}}}",
        name,
        excluded
            .iter()
            .map(|a| tile_json(*a))
            .collect::<Vec<_>>()
            .join(","),
        every_excluded_nonoptimal,
        l2t2_ok,
    ));
    // §12.1 — the cost note, plain integer microseconds. The naive
    // program pays the σ1 pass for every action; the targeted program
    // pays baseline + rungs + the survivor share of σ1 work.
    records.push(format!(
        "{{\"kind\":\"cost\",\"root\":\"{}\",\"micros_baseline_sigma0\":{},\
         \"micros_rungs_total\":{},\"micros_sigma1_all_actions\":{},\
         \"legal_count\":{},\"admitted_cheapest\":{},\"admitted_e4\":{},\
         \"note\":\"naive = sigma1 work for every legal action; targeted = \
         baseline + rungs + sigma1 work confined to the admissible set\"}}",
        name,
        micros_baseline0,
        micros_rungs,
        micros_sigma1,
        actions.len(),
        screen_bounds.len() - excluded.len().min(screen_bounds.len()),
        admissible.len(),
    ));
    eprintln!(
        "fieldswap_screen: {name} done: legal={} admissible_e4={} \
         (baseline0 {}us, rungs {}us, sigma1 {}us)",
        actions.len(),
        admissible.len(),
        micros_baseline0,
        micros_rungs,
        micros_sigma1,
    );
    records
}

fn run(out_path: &str, cfg: Config) {
    let r = receipt();
    eprintln!(
        "fieldswap_screen: {} roots; declared epoch pair field0 n0={}, field1 {}x{}, frozen {}x{}",
        ROOTS.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
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
    eprintln!("fieldswap_screen: wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("run");
    match mode {
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "fieldswap_screen.jsonl".to_string());
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
            };
            run(&out_path, cfg);
        }
        other => panic!("unknown mode {other:?}; expected run"),
    }
}
