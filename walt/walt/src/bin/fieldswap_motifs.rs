//! EXPLORATORY FIRST-SPLIT MOTIF INSTRUMENT — slice 4c [L2 thread] —
//! sits below every evidentiary tier and is cited by nothing above it.
//! Instrument output only: the six-motif first-split morphology
//! classification of the exact-fiber CORRECTION traces of the three
//! declared roots, the raw suffix enrichment specimens, the exact
//! per-motif decomposition (m_k⁺, m_k⁻, r_k, c_k, τ_k) with its
//! identities asserted, and the §3.6 residual report. Never a
//! play-strength claim; never an exposure claim.
//!
//! Mathematical source: Part 3 (§§3.1–3.9) of the x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
//! adopted by rulings TRIPLE-A6/A7 (`walt/CENSUS-RULINGS.md`). BINDING
//! (TRIPLE-A6): these traces exist only for u0 ≠ u1 worlds, so every
//! number here partitions CORRECTION MASS, never field exposure. The
//! safe phrasing for every mass: "Among exact correction worlds for this
//! root, field pair, and frozen policy, the first mechanical split had
//! motif k on mass m_k."
//!
//! DECLARED (σ0, σ1) EPOCH PAIR — the slice-3 cancel probe's, unchanged:
//! σ0 = Level0 { n0 = 8 }, σ1 = Level1 { n_outer = 4, n0 = 2 }, frozen
//! focal candidates at declared schedule [8, 2]. Roots: the three
//! declared cancel-probe roots — receipt-h7-t5 (fiber 1680),
//! receipt-h8-t4 (fiber 1200), receipt-h4-t6 (fiber 90).
//!
//! Trace records are RAW ONLY (TRIPLE-A7): world, split, terminals, the
//! two post-split suffixes, and the root semantics hash — no motif tag is
//! persisted on any trace record. Classification output rides separate
//! record kinds. Raw trace specimens are capped per (root, action) by the
//! declared `specimen_cap`; the full set is recomputed by rerunning this
//! binary.
//!
//! Mode: `fieldswap_motifs run <out.jsonl> [knobs]`. Knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!   specimen_cap
//!
//! No floats anywhere; wall time is integer microseconds.

use std::collections::BTreeMap;
use std::io::Write as _;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::exposure::{frozen_policy_exposure, WorldDomain};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::CancellationLadder;
use walt::solver::motif::{
    classify_trace, enrich_field_split_traces, EnrichedFieldSplitTrace, ExactMotifDecomposition,
    MotifClassification, RootFrame, RootFrameRegistry, SplitMotif,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

// ---------------------------------------------------------------------------
// Configuration — the declared epoch pair (slice-3 cancel probe, unchanged).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    n0_field0: u64,
    n_outer_field1: u64,
    n0_field1: u64,
    n_outer_frozen: u64,
    n0_frozen: u64,
    specimen_cap: usize,
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
// Roots — the three declared cancel-probe roots.
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

fn suffix_json(plays: &[(Seat, Domino)]) -> String {
    let parts: Vec<String> = plays
        .iter()
        .map(|(s, d)| format!("[{},{}]", s.index(), tile_json(*d)))
        .collect();
    format!("[{}]", parts.join(","))
}

fn history_json(plays: &[Domino]) -> String {
    let parts: Vec<String> = plays.iter().map(|d| tile_json(*d)).collect();
    format!("[{}]", parts.join(","))
}

fn signature_json(s: &walt::solver::motif::SplitSignature) -> String {
    let shape: Vec<String> = s.shape.iter().map(u8::to_string).collect();
    format!(
        "{{\"led\":\"{}\",\"control\":{},\"count\":{},\"trump\":{},\
         \"shape\":[{}],\"strength\":\"({:?},{})\"}}",
        s.led,
        s.control.index(),
        s.count,
        s.trump,
        shape.join(","),
        s.strength.tier,
        s.strength.rank.value(),
    )
}

fn micros(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_micros()).expect("a run fits in u64 microseconds")
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
    let mut registry = RootFrameRegistry::new();
    let frame = RootFrame::of(&root, &position);
    let semantics = registry.register(root_id, frame);
    let mut records: Vec<String> = Vec::new();
    records.push(format!(
        "{{\"kind\":\"root\",\"root\":\"{}\",\"decl\":\"{}\",\"bid\":{},\
         \"declaring_team\":{},\"viewer\":{},\"viewer_hand\":{},\"fiber\":\"{}\",\
         \"root_id\":\"{:#018x}\",\"root_semantics_hash\":\"{:#018x}\",\
         \"rule_version\":\"{}\",\"legal\":{},\
         \"epoch_pair\":{{\"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}}}},\
         \"frozen_schedule\":[{},{}],\"specimen_cap\":{}}}",
        name,
        position.decl,
        position.bid,
        position.declaring_team.index(),
        root.kernel().viewer().index(),
        set_json(root.kernel().viewer_hand()),
        root.count(),
        root_id,
        semantics,
        walt::solver::motif::RULE_VERSION,
        set_json(legal),
        field0.field_id(),
        cfg.n0_field0,
        field1.field_id(),
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.specimen_cap,
    ));
    for action in &actions {
        let rho = FrozenPolicy::new(focal_tuple(&position, cfg, *action));
        let started = Instant::now();
        let exposure = frozen_policy_exposure(
            &root,
            &position,
            &rho,
            &field0,
            &field1,
            WorldDomain::ExactFiber,
        );
        let ladder = CancellationLadder::from_exposure(&exposure);
        let enriched = enrich_field_split_traces(
            &root, &position, *action, &rho, &field0, &field1, &exposure,
        );
        let classified: Vec<(EnrichedFieldSplitTrace, MotifClassification)> = enriched
            .into_iter()
            .map(|e| {
                let c = classify_trace(&e, &registry);
                (e, c)
            })
            .collect();
        let decomposition = ExactMotifDecomposition::from_classified(&ladder, &classified);
        let micros_action = micros(started);
        // Raw trace specimens (TRIPLE-A7: raw fields only, no motif tag),
        // capped by the declared specimen_cap.
        for (enriched, _) in classified.iter().take(cfg.specimen_cap) {
            let t = &enriched.trace;
            records.push(format!(
                "{{\"kind\":\"trace\",\"root\":\"{}\",\"action\":{},\"policy\":\"{}\",\
                 \"specimen\":true,\"world\":[{},{},{},{}],\
                 \"split\":{{\"seat\":{},\"trick\":{},\"ply\":{},\"tile0\":{},\
                 \"tile1\":{},\"hand\":{},\"history\":{}}},\
                 \"u0\":{},\"u1\":{},\
                 \"branch0_suffix\":{},\"branch1_suffix\":{},\
                 \"root_semantics_hash\":\"{:#018x}\"}}",
                name,
                tile_json(*action),
                t.policy,
                t.world[0],
                t.world[1],
                t.world[2],
                t.world[3],
                t.split.seat.index(),
                t.split.trick,
                t.split.ply,
                tile_json(t.split.tile0),
                tile_json(t.split.tile1),
                set_json(t.split.hand),
                history_json(&t.split.history),
                t.u0,
                t.u1,
                suffix_json(&enriched.branch0_suffix),
                suffix_json(&enriched.branch1_suffix),
                enriched.root_semantics_hash,
            ));
        }
        // The per-motif exact decomposition, identities already asserted
        // at construction; flags, actor relations, and terminal signs are
        // co-emitted counts over the same classified set.
        let motifs: Vec<String> = SplitMotif::ALL
            .iter()
            .map(|m| {
                format!(
                    "{{\"motif\":\"{}\",\"plus\":{},\"minus\":{},\
                     \"r_k\":\"{}\",\"c_k\":\"{}\",\"tilt\":{}}}",
                    m.tag(),
                    decomposition.plus[m.index()],
                    decomposition.minus[m.index()],
                    decomposition.r_k(*m),
                    decomposition.c_k(*m),
                    decomposition
                        .tilt(*m)
                        .map_or("null".to_string(), |t| format!("\"{t}\"")),
                )
            })
            .collect();
        let mut flag_counts = [0u64; 6];
        let mut partner = 0u64;
        let mut opponent = 0u64;
        for (_, c) in &classified {
            if let Some(flags) = &c.flags {
                for (i, set) in flags.ordered().iter().enumerate() {
                    if *set {
                        flag_counts[i] += 1;
                    }
                }
            }
            match c.split_actor_relation {
                Some(walt::solver::motif::SplitActorRelation::Partner) => partner += 1,
                Some(walt::solver::motif::SplitActorRelation::Opponent) => opponent += 1,
                None => {}
            }
        }
        records.push(format!(
            "{{\"kind\":\"motif_histogram\",\"root\":\"{}\",\"action\":{},\
             \"policy\":\"{}\",\"domain\":\"exact-fiber\",\"partition\":\"correction-mass\",\
             \"worlds\":{},\"c_plus\":{},\"c_minus\":{},\"correction_worlds\":{},\
             \"motifs\":[{}],\
             \"flag_counts\":{{\"diff_context\":{},\"diff_control\":{},\
             \"diff_count\":{},\"diff_trump\":{},\"diff_suit_shape\":{},\
             \"diff_strength\":{}}},\
             \"split_actor\":{{\"partner\":{},\"opponent\":{}}},\
             \"terminal_sign\":{{\"favors_field1\":{},\"favors_field0\":{}}},\
             \"residual_fraction\":{},\"micros\":{}}}",
            name,
            tile_json(*action),
            ladder.policy,
            ladder.worlds,
            ladder.c_plus,
            ladder.c_minus,
            ladder.outcome_changed,
            motifs.join(","),
            flag_counts[0],
            flag_counts[1],
            flag_counts[2],
            flag_counts[3],
            flag_counts[4],
            flag_counts[5],
            partner,
            opponent,
            ladder.c_plus,
            ladder.c_minus,
            decomposition
                .residual_fraction()
                .map_or("null".to_string(), |f| format!("\"{f}\"")),
            micros_action,
        ));
        // §3.6 — the residual report: raw signature pairs inside Other,
        // with counts (empty when nothing landed in the residual).
        let mut residual_pairs: BTreeMap<String, u64> = BTreeMap::new();
        for (_, c) in &classified {
            if c.motif != SplitMotif::Other {
                continue;
            }
            let key = match (&c.signatures, &c.residual) {
                (Some((s0, s1)), _) => format!(
                    "{{\"sigma0\":{},\"sigma1\":{}}}",
                    signature_json(s0),
                    signature_json(s1)
                ),
                (None, Some(reason)) => format!("{{\"reason\":\"{reason}\"}}"),
                (None, None) => unreachable!("Other carries a reason"),
            };
            *residual_pairs.entry(key).or_insert(0) += 1;
        }
        let pairs: Vec<String> = residual_pairs
            .iter()
            .map(|(k, n)| format!("{{\"pair\":{k},\"count\":{n}}}"))
            .collect();
        records.push(format!(
            "{{\"kind\":\"residual\",\"root\":\"{}\",\"action\":{},\
             \"other_worlds\":{},\"pairs\":[{}]}}",
            name,
            tile_json(*action),
            decomposition.correction_worlds(SplitMotif::Other),
            pairs.join(","),
        ));
        eprintln!(
            "fieldswap_motifs: {name} {} corrections={} other={} ({}us)",
            tile_json(*action),
            ladder.outcome_changed,
            decomposition.correction_worlds(SplitMotif::Other),
            micros_action,
        );
    }
    records
}

fn run(out_path: &str, cfg: Config) {
    let r = receipt();
    eprintln!(
        "fieldswap_motifs: {} roots; declared epoch pair field0 n0={}, field1 {}x{}, \
         frozen {}x{}, specimen cap {}",
        ROOTS.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.specimen_cap,
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
    eprintln!("fieldswap_motifs: wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("run");
    match mode {
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "fieldswap_motifs.jsonl".to_string());
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
                specimen_cap: usize::try_from(knob(8)).expect("fits"),
            };
            run(&out_path, cfg);
        }
        other => panic!("unknown mode {other:?}; expected run"),
    }
}
