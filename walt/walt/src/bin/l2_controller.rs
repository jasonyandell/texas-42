//! EXPLORATORY TARGETED FIELD-1 CONTROLLER INSTRUMENT — the assembled
//! per-root pipeline of `solver::targeted` (parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §8 Stages 1–5;
//! rulings L2-A1..A7, PANEL-A7/A8, TRIPLE-A2) run over the predeclared
//! corpus. Sits below every evidentiary tier and is cited by nothing
//! above it. Instrument output only: per-root classification, rung-typed
//! exposure bounds, schedule-controlled rung spend, survivor-only σ1
//! work and what it changed, typed refusals. Never a play-strength
//! claim; nothing here touches the live player or any default.
//!
//! DECLARED (σ0, σ1) EPOCH PAIR — the fieldswap-screen/cancel probe
//! epoch's, unchanged: σ0 = Level0 { n0 = 8 }, σ1 = Level1 { n_outer =
//! 4, n0 = 2 }, one frozen focal candidate per legal root action at
//! declared schedule [8, 2] (`ActionRule::PinnedThenLevel1`), one freeze
//! construction for the whole probe epoch. Both FieldIds ride every
//! record; a different schedule is a different experiment epoch.
//!
//! PREDECLARED CORPUS (existing roots only; O14 — declared before any
//! number is read):
//! - the three slice-2/3 screen roots: receipt-h7-t5 (fiber 1680),
//!   receipt-h8-t4 (fiber 1200), receipt-h4-t6 (fiber 90) — exact route;
//! - the four step-9 flip-fixture roots (`calibrate::FLIP_FIXTURES`,
//!   reconstructed decisions) — exact route;
//! - the six count-timing shape-family members
//!   (`calibrate::CountTimingSpec`, g = 0..5 at drive n0 = 8, fiber
//!   46,558,512) — the sampled route (the honest degradation path).
//!
//! Declared risks (sampled roots only): screen budget δ = 1/50 per root;
//! per-action one-sided baseline endpoints at 1/800 each; per-action
//! symmetric E3 at 1/400. Prefixes: baseline 128 worlds, E3 24 worlds.
//! Caps and prefixes are resource limits, never settlement rules.
//!
//! Mode: `l2_controller run <out.jsonl> [knobs]`. Knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!   exact_cap baseline_prefix e3_prefix ct_members
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::Domino;
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::calibrate::{reconstruct_flip, CountTimingSpec, FLIP_FIXTURES};
use walt::solver::evidence::ScopedDelta;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::targeted::{
    legal_root_actions, targeted_root, RungBudget, StageFourOutcome, TargetedConfig, TargetedRisk,
    TargetedRootReport,
};

// ---------------------------------------------------------------------------
// Configuration — the declared epoch pair and resource limits.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    n0_field0: u64,
    n_outer_field1: u64,
    n0_field1: u64,
    n_outer_frozen: u64,
    n0_frozen: u64,
    exact_cap: u128,
    baseline_prefix: u64,
    e3_prefix: u64,
    ct_members: u64,
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

fn targeted_config(name: &str, cfg: Config) -> TargetedConfig {
    TargetedConfig {
        budget: RungBudget {
            exact_fiber_cap: cfg.exact_cap,
            baseline_prefix: cfg.baseline_prefix,
            e3_prefix: cfg.e3_prefix,
            directional: true,
        },
        risk: Some(TargetedRisk {
            screen_budget: ScopedDelta::new(
                format!("l2c:{name}:screen"),
                BigRational::new(BigInt::from(1), BigInt::from(50)),
            ),
            per_baseline_side: BigRational::new(BigInt::from(1), BigInt::from(800)),
            per_e3: BigRational::new(BigInt::from(1), BigInt::from(400)),
        }),
        epsilon: Some(BigRational::new(BigInt::from(1), BigInt::from(20))),
        epoch: 0,
        scope: format!("l2c:{name}"),
    }
}

// ---------------------------------------------------------------------------
// Roots — the predeclared corpus.
// ---------------------------------------------------------------------------

const RECEIPT_ROOTS: [(usize, usize); 3] = [(7, 5), (8, 4), (4, 6)];

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn receipt_root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

// ---------------------------------------------------------------------------
// JSON helpers (hand-rolled, like the sibling probes — no serde).
// ---------------------------------------------------------------------------

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn opt_tile_json(d: Option<Domino>) -> String {
    d.map_or("null".to_string(), tile_json)
}

// ---------------------------------------------------------------------------
// The per-root run: one targeted_root call, serialized faithfully.
// ---------------------------------------------------------------------------

fn run_root(name: &str, root: &CanonicalRoot, position: &RootPosition, cfg: Config) -> Vec<String> {
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    let legal = legal_root_actions(root, position);
    let actions: Vec<Domino> = legal.iter().collect();
    let policies: Vec<FrozenPolicy> = actions
        .iter()
        .map(|a| FrozenPolicy::new(focal_tuple(position, cfg, *a)))
        .collect();
    let candidates: Vec<(Domino, &FrozenPolicy)> =
        actions.iter().copied().zip(policies.iter()).collect();
    let config = targeted_config(name, cfg);
    let report = targeted_root(root, position, &candidates, &field0, &field1, &config);
    serialize_report(name, root, position, cfg, &report)
}

#[allow(clippy::too_many_lines)]
fn serialize_report(
    name: &str,
    root: &CanonicalRoot,
    position: &RootPosition,
    cfg: Config,
    report: &TargetedRootReport,
) -> Vec<String> {
    let mut records: Vec<String> = Vec::new();
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\
         \"declaring_team\":{},\"viewer\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"legal_count\":{},\
         \"epoch_pair\":{{\"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}}}},\
         \"frozen_schedule\":[{},{}],\
         \"budget\":{{\"exact_cap\":\"{}\",\"baseline_prefix\":{},\"e3_prefix\":{},\
         \"directional\":true}},\
         \"declared_risk\":{{\"screen_budget\":\"1/50\",\"per_baseline_side\":\"1/800\",\
         \"per_e3\":\"1/400\"}},\"tier\":\"{}\"}}",
        name,
        position.decl,
        position.bid,
        position.declaring_team.index(),
        root.kernel().viewer().index(),
        report.fiber,
        report.root_id,
        report.rows.len(),
        report.field0,
        cfg.n0_field0,
        report.field1,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.exact_cap,
        cfg.baseline_prefix,
        cfg.e3_prefix,
        report.tier.label(),
    ));
    for row in &report.rows {
        records.push(format!(
            "{{\"kind\":\"row\",\"root\":\"{}\",\"action\":{},\
             \"lower0\":\"{}\",\"upper0\":\"{}\",\
             \"exposure\":{{\"rung\":\"{}\",\"upper\":\"{}\"}},\
             \"steering_lower\":{},\"admitted\":{}}}",
            name,
            tile_json(row.action),
            row.lower0,
            row.upper0,
            row.exposure.rung(),
            row.exposure.screenable_upper(),
            row.steering
                .as_ref()
                .map_or("null".to_string(), |s| format!(
                    "{{\"producer\":\"{}\",\"value\":\"{}\"}}",
                    s.producer,
                    s.value_for_steering()
                )),
            row.admitted,
        ));
    }
    if let Some(screen) = &report.screen {
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
            "{{\"kind\":\"screen\",\"root\":\"{}\",\"tier\":\"{}\",\
             \"result\":\"{}\",\"stop\":\"{}\",\"bar\":\"{}\",\
             \"legal_count\":{},\"admitted_count\":{},\"admissible\":[{}],\
             \"slack\":[{}]}}",
            name,
            report.tier.label(),
            report.kind,
            report.stop.tag(),
            screen.bar(),
            report.rows.len(),
            admissible.len(),
            admissible
                .iter()
                .map(|a| tile_json(*a))
                .collect::<Vec<_>>()
                .join(","),
            slack.join(","),
        ));
    }
    records.push(format!(
        "{{\"kind\":\"directional\",\"root\":\"{}\",\"phase\":\"{}\",\
         \"admissible\":{}}}",
        name,
        report.directional_phase.tag(),
        report.directional.as_ref().map_or("null".to_string(), |d| {
            format!(
                "[{}]",
                d.admissible()
                    .iter()
                    .map(|a| tile_json(*a))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }),
    ));
    let spend: Vec<String> = report
        .spend
        .iter()
        .map(|p| {
            format!(
                "{{\"phase\":\"{}\",\"micros\":{},\"items\":{}}}",
                p.phase, p.micros, p.items
            )
        })
        .collect();
    records.push(format!(
        "{{\"kind\":\"spend\",\"root\":\"{}\",\"phases\":[{}]}}",
        name,
        spend.join(","),
    ));
    match &report.stage4 {
        StageFourOutcome::ExactSurvivors {
            evaluation,
            ladders,
        } => {
            let values1 = evaluation.values1.as_ref().map_or("null".to_string(), |v| {
                let parts: Vec<String> = v
                    .actions
                    .iter()
                    .zip(&v.values)
                    .map(|(a, val)| format!("{{\"action\":{},\"value\":\"{val}\"}}", tile_json(*a)))
                    .collect();
                format!("[{}]", parts.join(","))
            });
            let ladder_rows: Vec<String> = ladders
                .iter()
                .map(|l| {
                    format!(
                        "{{\"action\":{},\"d\":\"{}\",\"r\":\"{}\",\"c\":\"{}\",\
                         \"c_plus\":{},\"c_minus\":{},\"label\":\"{}\"}}",
                        tile_json(l.action),
                        l.ladder.d(),
                        l.ladder.r(),
                        l.ladder.c(),
                        l.ladder.c_plus,
                        l.ladder.c_minus,
                        l.label,
                    )
                })
                .collect();
            records.push(format!(
                "{{\"kind\":\"stage4\",\"root\":\"{}\",\"route\":\"exact\",\
                 \"survivors\":[{}],\"settled0\":{},\"selected1\":{},\
                 \"decision_changed\":{},\"result\":\"{}\",\
                 \"sigma1_values\":{},\"ladders\":[{}]}}",
                name,
                evaluation
                    .survivors
                    .iter()
                    .map(|a| tile_json(*a))
                    .collect::<Vec<_>>()
                    .join(","),
                tile_json(evaluation.settled0),
                tile_json(evaluation.selected1),
                evaluation.decision_changed(),
                report.kind,
                values1,
                ladder_rows.join(","),
            ));
        }
        StageFourOutcome::DeltaSingleton { selected } => {
            records.push(format!(
                "{{\"kind\":\"stage4\",\"root\":\"{}\",\"route\":\"delta-singleton\",\
                 \"selected\":{},\"result\":\"{}\"}}",
                name,
                tile_json(*selected),
                report.kind,
            ));
        }
        StageFourOutcome::DeltaSurvivors {
            sigma1,
            settled0,
            selected1,
        } => {
            let intervals: Vec<String> = sigma1
                .actions
                .iter()
                .enumerate()
                .map(|(k, a)| {
                    format!(
                        "{{\"action\":{},\"wins\":{},\"lower\":\"{}\",\"upper\":\"{}\"}}",
                        tile_json(*a),
                        sigma1.win_counts[k],
                        sigma1.lower(*a),
                        sigma1.upper(*a),
                    )
                })
                .collect();
            records.push(format!(
                "{{\"kind\":\"stage4\",\"root\":\"{}\",\"route\":\"delta-survivors\",\
                 \"worlds\":{},\"sigma1_intervals\":[{}],\"settled0\":{},\
                 \"selected1\":{},\"open\":{},\"result\":\"{}\"}}",
                name,
                sigma1.worlds,
                intervals.join(","),
                opt_tile_json(*settled0),
                opt_tile_json(*selected1),
                selected1.is_none(),
                report.kind,
            ));
        }
        StageFourOutcome::NotRun(refusal) => {
            records.push(format!(
                "{{\"kind\":\"stage4\",\"root\":\"{}\",\"route\":\"not-run\",\
                 \"refusal\":\"{}\",\"result\":\"{}\"}}",
                name, refusal, report.kind,
            ));
        }
    }
    for refusal in &report.refusals {
        records.push(format!(
            "{{\"kind\":\"refusal\",\"root\":\"{}\",\"stage\":\"{}\",\
             \"reason\":\"{}\"}}",
            name, refusal.stage, refusal.reason,
        ));
    }
    records.push(format!(
        "{{\"kind\":\"risk\",\"root\":\"{}\",\"spent\":{},\"budget\":\"1/50\"}}",
        name,
        report
            .risk_spent
            .as_ref()
            .map_or("null".to_string(), |r| format!("\"{r}\"")),
    ));
    eprintln!(
        "l2_controller: {name} done: tier={} kind={} stop={} survivors={}/{}",
        report.tier.label(),
        report.kind,
        report.stop.tag(),
        report.survivors().len(),
        report.rows.len(),
    );
    records
}

// ---------------------------------------------------------------------------
// The run: the whole predeclared corpus, roots in parallel.
// ---------------------------------------------------------------------------

enum CorpusRoot {
    Receipt(usize, usize),
    Flip(usize),
    CountTiming(u64),
}

fn corpus(cfg: Config) -> Vec<CorpusRoot> {
    let mut roots: Vec<CorpusRoot> = RECEIPT_ROOTS
        .iter()
        .map(|(h, t)| CorpusRoot::Receipt(*h, *t))
        .collect();
    roots.extend((0..FLIP_FIXTURES.len()).map(CorpusRoot::Flip));
    roots.extend((0..cfg.ct_members).map(CorpusRoot::CountTiming));
    roots
}

fn run_corpus_root(r: &Receipt, entry: &CorpusRoot, cfg: Config) -> Vec<String> {
    match entry {
        CorpusRoot::Receipt(hand_id, trick_no) => {
            let name = format!("receipt-h{hand_id}-t{trick_no}");
            let (root, position) = receipt_root_at(r, *hand_id, *trick_no);
            run_root(&name, &root, &position, cfg)
        }
        CorpusRoot::Flip(k) => {
            let fixture = &FLIP_FIXTURES[*k];
            let name = format!("flip-h{}-d{}", fixture.hand, fixture.d);
            let flip = reconstruct_flip(fixture);
            run_root(&name, &flip.root, &flip.position, cfg)
        }
        CorpusRoot::CountTiming(g) => {
            let name = format!("count-timing-g{g}");
            let spec = CountTimingSpec::new(*g, 8);
            let flip = spec.root();
            run_root(&name, &flip.root, &flip.position, cfg)
        }
    }
}

fn run(out_path: &str, cfg: Config) {
    let r = receipt();
    let roots = corpus(cfg);
    eprintln!(
        "l2_controller: {} roots; epoch pair field0 n0={}, field1 {}x{}, frozen {}x{}; \
         exact cap {}, baseline prefix {}, e3 prefix {}",
        roots.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.exact_cap,
        cfg.baseline_prefix,
        cfg.e3_prefix,
    );
    #[cfg(feature = "parallel")]
    let per_root: Vec<Vec<String>> = roots
        .par_iter()
        .map(|entry| run_corpus_root(&r, entry, cfg))
        .collect();
    #[cfg(not(feature = "parallel"))]
    let per_root: Vec<Vec<String>> = roots
        .iter()
        .map(|entry| run_corpus_root(&r, entry, cfg))
        .collect();
    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    for records in per_root {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("l2_controller: wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("run");
    match mode {
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "l2_controller.jsonl".to_string());
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
                exact_cap: u128::from(knob(4096)),
                baseline_prefix: knob(128),
                e3_prefix: knob(24),
                ct_members: knob(6),
            };
            run(&out_path, cfg);
        }
        other => panic!("unknown mode {other:?}; expected run"),
    }
}
