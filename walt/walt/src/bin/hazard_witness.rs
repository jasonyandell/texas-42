//! EXPLORATORY HAZARD-WITNESS INSTRUMENT — slice 4b [L2 thread; dominance
//! objective-level] — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: the One-Round
//! Trump-Extraction producer run over the cancel-corpus pairwise
//! comparisons, reporting accepts (each with its witness VERIFIED by the
//! single authority and cross-checked against exact enumeration) and
//! declines (with the failed-hypothesis histogram). Mostly declines is
//! the expected, correct outcome for a deliberately narrow first
//! producer; nothing here widens a hypothesis to force an accept, and
//! never a play-strength claim.
//!
//! Mathematical source: Part 2 (§§2.1–2.8, proof ledger P5) of the x:024
//! response (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
//! adopted by rulings TRIPLE-A4/A5 (`walt/CENSUS-RULINGS.md`), under
//! PANEL-A7's dominance vocabulary.
//!
//! DECLARED (σ0, σ1) EPOCH PAIR — the slice-3 cancel probe's, unchanged:
//! σ0 = Level0 { n0 = 8 }, σ1 = Level1 { n_outer = 4, n0 = 2 }, frozen
//! focal candidates at declared schedule [8, 2]. Roots: the three
//! declared cancel-probe roots — receipt-h7-t5 (fiber 1680),
//! receipt-h8-t4 (fiber 1200), receipt-h4-t6 (fiber 90).
//!
//! Mode: `hazard_witness run <out.jsonl> [knobs]`. Knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!
//! No floats anywhere; wall time is integer microseconds.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::exact_pairwise_masses;
use walt::solver::hazard::{
    dominance_from_witnessed_hazard_zero, exhibit_benefit_world, one_round_trump_extraction,
    verify_hazard_witness,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

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

fn pinned(position: &RootPosition, cfg: Config, tile: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
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
        action_rule: ActionRule::PinnedThenLevel1 { pinned: tile },
    })
}

/// (hand, trick) — the three declared cancel-probe roots.
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

fn tile_name(d: Domino) -> String {
    format!("{}-{}", d.hi().value(), d.lo().value())
}

fn micros(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_micros()).expect("a run fits in u64 microseconds")
}

fn run_root(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    cfg: Config,
    histogram: &mut BTreeMap<&'static str, u64>,
    accepts: &mut u64,
    pairs: &mut u64,
) -> Vec<String> {
    let name = format!("receipt-h{hand_id}-t{trick_no}");
    let (root, position) = root_at(r, hand_id, trick_no);
    let root_id = root_identity(&root, &position);
    let actions: Vec<Domino> = legal_root_actions(&root, &position).iter().collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|t| pinned(&position, cfg, *t)).collect();
    let mut records = Vec::new();
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"viewer\":{},\"actions\":{},\
         \"epoch_pair\":{{\"field0\":\"level0 n0={}\",\"field1\":\"level1 n_outer={} n0={}\"}},\
         \"frozen_schedule\":[{},{}]}}",
        name,
        position.decl,
        position.bid,
        root.count(),
        root_id,
        root.kernel().viewer().index(),
        actions.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
    ));
    for (field_label, spec) in [("field0", field0_spec(cfg)), ("field1", field1_spec(cfg))] {
        let field = FieldModel::new(spec);
        for (i, rho_a) in policies.iter().enumerate() {
            for (j, rho_b) in policies.iter().enumerate() {
                if i == j {
                    continue;
                }
                *pairs += 1;
                let started = Instant::now();
                match one_round_trump_extraction(&root, &position, rho_a, rho_b, &field) {
                    Ok(witness) => {
                        // The single authority checks every emitted
                        // witness; an emitted witness failing to verify
                        // would be a producer bug and stops the probe.
                        let hazard =
                            verify_hazard_witness(&root, &position, rho_a, rho_b, &field, &witness)
                                .expect("an emitted witness verifies");
                        // Instrument-tier cross-check: exact enumeration
                        // on the same fiber must agree H = 0.
                        let masses = exact_pairwise_masses(&root, &position, rho_a, rho_b, &field);
                        assert_eq!(
                            masses.hazard_worlds(),
                            0,
                            "a verified witness never contradicts the exact route"
                        );
                        let benefit = exhibit_benefit_world(&root, &position, rho_a, rho_b, &field);
                        let dominated = benefit.as_ref().map(|b| {
                            dominance_from_witnessed_hazard_zero(&hazard, b)
                                .tag()
                                .to_string()
                        });
                        *accepts += 1;
                        records.push(format!(
                            "{{\"kind\":\"pair\",\"root\":\"{}\",\"field\":\"{}\",\
                             \"a\":\"pin-{}\",\"b\":\"pin-{}\",\"result\":\"accept\",\
                             \"witness_hash\":\"{:#018x}\",\"exact_H\":\"{}\",\"exact_B\":\"{}\",\
                             \"benefit_world\":{},\"valid_bound_kind\":{},\
                             \"exact_kind\":\"{}\",\"micros\":{}}}",
                            name,
                            field_label,
                            tile_name(actions[i]),
                            tile_name(actions[j]),
                            hazard.witness_hash(),
                            masses.h(),
                            masses.b(),
                            benefit.is_some(),
                            dominated.map_or("null".to_string(), |k| format!("\"{k}\"")),
                            masses.dominance_kind().tag(),
                            micros(started),
                        ));
                    }
                    Err(decline) => {
                        *histogram.entry(decline.tag()).or_insert(0) += 1;
                        records.push(format!(
                            "{{\"kind\":\"pair\",\"root\":\"{}\",\"field\":\"{}\",\
                             \"a\":\"pin-{}\",\"b\":\"pin-{}\",\"result\":\"decline\",\
                             \"failed\":\"{}\",\"micros\":{}}}",
                            name,
                            field_label,
                            tile_name(actions[i]),
                            tile_name(actions[j]),
                            decline.tag(),
                            micros(started),
                        ));
                    }
                }
            }
        }
    }
    records
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() >= 3 && args[1] == "run",
        "usage: hazard_witness run <out.jsonl> [n0_field0 n_outer_field1 n0_field1 \
         n_outer_frozen n0_frozen]"
    );
    let knob = |k: usize, default: u64| -> u64 {
        args.get(2 + k)
            .map_or(default, |s| s.parse().expect("an integer knob"))
    };
    let cfg = Config {
        n0_field0: knob(1, 8),
        n_outer_field1: knob(2, 4),
        n0_field1: knob(3, 2),
        n_outer_frozen: knob(4, 8),
        n0_frozen: knob(5, 2),
    };
    let started = Instant::now();
    let r = receipt();
    let mut histogram: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut accepts = 0u64;
    let mut pairs = 0u64;
    let mut records: Vec<String> = Vec::new();
    for (hand_id, trick_no) in ROOTS {
        eprintln!("== {}", format_args!("receipt-h{hand_id}-t{trick_no}"));
        records.extend(run_root(
            &r,
            hand_id,
            trick_no,
            cfg,
            &mut histogram,
            &mut accepts,
            &mut pairs,
        ));
    }
    let hist_json: Vec<String> = histogram
        .iter()
        .map(|(tag, n)| format!("\"{tag}\":{n}"))
        .collect();
    records.push(format!(
        "{{\"kind\":\"summary\",\"pairs\":{},\"accepts\":{},\"declines\":{},\
         \"decline_histogram\":{{{}}},\"micros\":{}}}",
        pairs,
        accepts,
        pairs - accepts,
        hist_json.join(","),
        micros(started),
    ));
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    for record in &records {
        writeln!(out, "{record}").expect("the output file writes");
    }
    eprintln!(
        "hazard_witness: {} pairs, {} accepts, {} declines ({} records) in {} us",
        pairs,
        accepts,
        pairs - accepts,
        records.len(),
        micros(started),
    );
}
