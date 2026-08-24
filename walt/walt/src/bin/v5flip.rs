//! EXPLORATORY V5 GATE INSTRUMENT (§22 step 8; parent §19 V5) — sits
//! below every evidentiary tier and is cited by nothing above it.
//!
//! The historical flip repair: re-run flip-shaped roots under the §16.4
//! adaptive controller on ONE epoch and ONE common indexed stream at an
//! ascending ladder of resource caps, and demonstrate mechanically that
//! the outcome is monotone — unresolved may settle later, settled stays
//! settled identically, exact stays exact identically — never two caps
//! "settled" with different answers (the 40/160 failure mode). Where the
//! fiber permits, the exact frozen-set endpoint is recorded beside the
//! adaptive ladder.
//!
//! Two specimen families:
//!   1. The step-7 shadow run's four exact-route disagreements
//!      (`solver::calibrate::FLIP_FIXTURES`), reconstructed by rules
//!      replay and pinned to the shadow records by their epoch hashes.
//!   2. The count-timing SHAPE family (`CountTimingSpec`): the 2026-08-23
//!      plunge review's trick-1 6-2 vs 6-4 near-tie, reconstructed by
//!      shape (the literal position's seeds live plunge-side; L2-A6).
//!
//! Usage: `v5flip <out.jsonl> [count_timing_n]`
//! Caps are the declared ladder 40/160/640. No floats anywhere.

use std::io::Write as _;

use num_bigint::BigInt;
use num_rational::BigRational;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use walt::rules::Domino;
use walt::solver::adaptive::root_identity;
use walt::solver::calibrate::{
    assert_cap_ladder, count_timing_tuple, reconstruct_flip, shadow_scopes, shadow_tuple,
    CapLadderVerdict, CountTimingSpec, FlipRoot, FLIP_FIXTURES,
};
use walt::solver::controller::{
    epoch_identity, evaluate_set, exact_frozen_set, CandidateSet, EscalationConfig, RiskPlan,
    SetEvaluation, SetResult, SetSpec,
};
use walt::solver::evidence::{decision_delta, ScopedDelta};
use walt::solver::policy::FrozenPolicy;

/// The declared cap ladder: the historical coordinates 40 and 160 (kept
/// as replay fixtures per CE-A5 — resource limits here, never settlement
/// rules) plus one further extension.
const CAPS: [u64; 3] = [40, 160, 640];

fn delta_run() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(100))
}

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

/// One cap step's projection of a `SetResult` for the JSON record.
#[derive(Default)]
struct CapView {
    winner: Option<usize>,
    survivors: Option<Vec<usize>>,
    settled_at: Option<u64>,
    wins: Option<Vec<u128>>,
}

fn cap_json(cap: u64, evaluation: &SetEvaluation, legal: &[Domino]) -> String {
    let CapView {
        winner,
        survivors,
        settled_at,
        wins,
    } = match &evaluation.result {
        SetResult::ExactFrozenSet { winner, wins, .. } => CapView {
            winner: *winner,
            wins: Some(wins.clone()),
            ..CapView::default()
        },
        SetResult::DeltaSettled {
            winner, settled_at, ..
        } => CapView {
            winner: Some(*winner),
            settled_at: Some(*settled_at),
            ..CapView::default()
        },
        SetResult::EpsilonEquivalent {
            survivors,
            settled_at,
            ..
        } => CapView {
            survivors: Some(survivors.clone()),
            settled_at: Some(*settled_at),
            ..CapView::default()
        },
        SetResult::Unresolved { survivors, .. } => CapView {
            survivors: Some(survivors.clone()),
            ..CapView::default()
        },
    };
    let pairs: Vec<String> = evaluation
        .pair_counts
        .iter()
        .map(|p| {
            format!(
                "{{\"i\":{},\"j\":{},\"a\":{},\"b\":{},\"n\":{}}}",
                p.i, p.j, p.a, p.b, p.n
            )
        })
        .collect();
    format!(
        "{{\"cap\":{cap},\"tag\":\"{}\",\"winner\":{},\"winner_tile\":{},\
         \"survivors\":{},\"settled_at\":{},\"wins\":{},\"consumed\":{},\
         \"escalated_at\":{},\"pair_counts\":[{}]}}",
        evaluation.result.tag(),
        winner.map_or("null".to_string(), |k| k.to_string()),
        winner.map_or("null".to_string(), |k| tile_json(legal[k])),
        survivors.map_or("null".to_string(), |s| format!(
            "[{}]",
            s.iter().map(usize::to_string).collect::<Vec<_>>().join(",")
        )),
        settled_at.map_or("null".to_string(), |x| x.to_string()),
        wins.map_or("null".to_string(), |w| format!(
            "[{}]",
            w.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(",")
        )),
        evaluation.consumed,
        evaluation
            .escalation
            .as_ref()
            .map_or("null".to_string(), |e| e.switched_at.to_string()),
        pairs.join(","),
    )
}

/// Run the cap ladder for one root over one shared candidate set (shared
/// so materialized actions are computed once — sharing changes cost only,
/// never results, which are functions of the stream).
fn ladder(
    flip: &FlipRoot,
    candidates_owned: &[FrozenPolicy],
    run_scope: &str,
    dec_scope: &str,
    d: u64,
) -> (Vec<(u64, SetEvaluation)>, CapLadderVerdict, String) {
    let candidates = CandidateSet::new(candidates_owned.iter().collect());
    let dec_delta = decision_delta(d, &delta_run());
    let field = walt::solver::policy::Level0Field::new(2);
    let mut evaluations: Vec<(u64, SetEvaluation)> = Vec::new();
    for cap in CAPS {
        let plan = RiskPlan::strict(ScopedDelta::new(dec_scope.to_string(), dec_delta.clone()))
            .under_run(ScopedDelta::new(run_scope.to_string(), delta_run()), d);
        let spec = SetSpec {
            root: &flip.root,
            position: &flip.position,
            candidates: &candidates,
            field: &field,
            plan,
            world_cap: cap,
            batch: 8,
            escalation: Some(EscalationConfig {
                cost_sample: 1,
                cost_enumerate: 1,
                check_every: 8,
            }),
        };
        evaluations.push((cap, evaluate_set(&spec)));
    }
    let results: Vec<(u64, &SetResult)> = evaluations
        .iter()
        .map(|(cap, e)| (*cap, &e.result))
        .collect();
    let verdict = assert_cap_ladder(&results);
    let plan = RiskPlan::strict(ScopedDelta::new(dec_scope.to_string(), dec_delta))
        .under_run(ScopedDelta::new(run_scope.to_string(), delta_run()), d);
    let root_id = root_identity(&flip.root, &flip.position);
    let epoch = epoch_identity(root_id, &candidates, plan.decision());
    (evaluations, verdict, epoch.to_string())
}

fn flip_record(index: usize) -> String {
    let f = &FLIP_FIXTURES[index];
    let flip = reconstruct_flip(f);
    let candidates_owned: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| FrozenPolicy::new(shadow_tuple(&flip.position, *t)))
        .collect();
    let (run_scope, dec_scope) = shadow_scopes(f);
    let (evaluations, verdict, epoch) =
        ladder(&flip, &candidates_owned, &run_scope, &dec_scope, f.d);
    assert_eq!(
        epoch, f.epoch,
        "the reconstruction reproduces the shadow record's epoch exactly"
    );
    // The exact frozen-set reference beside the adaptive ladder (§19 V5:
    // "switch to exact enumeration and return the exact target"). Must
    // reproduce the shadow record's exact wins.
    let candidates = CandidateSet::new(candidates_owned.iter().collect());
    let plan = RiskPlan::strict(ScopedDelta::new(
        dec_scope.clone(),
        decision_delta(f.d, &delta_run()),
    ))
    .under_run(ScopedDelta::new(run_scope.clone(), delta_run()), f.d);
    let field = walt::solver::policy::Level0Field::new(2);
    let spec = SetSpec {
        root: &flip.root,
        position: &flip.position,
        candidates: &candidates,
        field: &field,
        plan,
        world_cap: 0,
        batch: 8,
        escalation: None,
    };
    let exact = exact_frozen_set(&spec);
    let SetResult::ExactFrozenSet { wins, winner, .. } = &exact.result else {
        panic!("the exact endpoint returns ExactFrozenSet");
    };
    assert_eq!(
        wins.as_slice(),
        f.exact_wins,
        "the exact wins reproduce the shadow record"
    );
    let ladder_json: Vec<String> = evaluations
        .iter()
        .map(|(cap, e)| cap_json(*cap, e, &flip.legal_tiles))
        .collect();
    format!(
        "{{\"kind\":\"flip\",\"source\":\"shadow-exact-disagreement\",\
         \"mode\":\"{}\",\"hand\":{},\"d\":{},\"trick\":{},\"ply\":{},\
         \"fiber\":\"{}\",\"m\":{},\"legal\":[{}],\"live_tile\":{},\
         \"exact\":{{\"wins\":[{}],\"winner_tile\":{}}},\"epoch\":\"{}\",\
         \"ladder\":[{}],\"verdict\":\"{:?}\"}}",
        f.mode,
        f.hand,
        f.d,
        f.trick,
        f.ply,
        f.fiber,
        f.m,
        flip.legal_tiles
            .iter()
            .map(|t| tile_json(*t))
            .collect::<Vec<_>>()
            .join(","),
        tile_json(Domino::new(
            walt::rules::Pip::new(f.live_tile.0).expect("pip"),
            walt::rules::Pip::new(f.live_tile.1).expect("pip"),
        )),
        wins.iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<_>>()
            .join(","),
        winner.map_or("exact-tie".to_string(), |k| tile_json(flip.legal_tiles[k])),
        epoch,
        ladder_json.join(","),
        verdict,
    )
}

fn count_timing_record(g: u64) -> String {
    let spec = CountTimingSpec::new(g, 8);
    let flip = spec.root();
    let candidates_owned: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| FrozenPolicy::new(count_timing_tuple(&flip.position, *t)))
        .collect();
    let run_scope = format!("run:v5-count-timing-g{g}");
    let dec_scope = format!("decision:v5-count-timing-g{g}-d1");
    let (evaluations, verdict, epoch) = ladder(&flip, &candidates_owned, &run_scope, &dec_scope, 1);
    let ladder_json: Vec<String> = evaluations
        .iter()
        .map(|(cap, e)| cap_json(*cap, e, &flip.legal_tiles))
        .collect();
    let deal_json: Vec<String> = spec
        .deal
        .iter()
        .map(|hand| {
            format!(
                "[{}]",
                hand.iter().map(tile_json).collect::<Vec<_>>().join(",")
            )
        })
        .collect();
    format!(
        "{{\"kind\":\"count-timing\",\"source\":\"plunge-2026-08-23-shape\",\
         \"g\":{g},\"s2_tile\":{},\"fiber\":\"{}\",\"m\":{},\"legal\":[{}],\
         \"deal\":[{}],\"epoch\":\"{}\",\"ladder\":[{}],\"verdict\":\"{:?}\"}}",
        tile_json(spec.s2_tile),
        flip.root.count(),
        flip.legal_tiles.len(),
        flip.legal_tiles
            .iter()
            .map(|t| tile_json(*t))
            .collect::<Vec<_>>()
            .join(","),
        deal_json.join(","),
        epoch,
        ladder_json.join(","),
        verdict,
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "v5.jsonl".to_string());
    let count_timing_n: u64 = args
        .get(2)
        .map(|v| v.parse().expect("a count of count-timing roots"))
        .unwrap_or(6);
    enum Unit {
        Flip(usize),
        CountTiming(u64),
    }
    let units: Vec<Unit> = (0..FLIP_FIXTURES.len())
        .map(Unit::Flip)
        .chain((0..count_timing_n).map(Unit::CountTiming))
        .collect();
    let run = |unit: &Unit| -> String {
        match unit {
            Unit::Flip(i) => {
                let record = flip_record(*i);
                eprintln!(
                    "v5flip: fixture {} ({} hand {}) done",
                    i, FLIP_FIXTURES[*i].mode, FLIP_FIXTURES[*i].hand
                );
                record
            }
            Unit::CountTiming(g) => {
                let record = count_timing_record(*g);
                eprintln!("v5flip: count-timing g={g} done");
                record
            }
        }
    };
    #[cfg(feature = "parallel")]
    let records: Vec<String> = units.par_iter().map(run).collect();
    #[cfg(not(feature = "parallel"))]
    let records: Vec<String> = units.iter().map(run).collect();
    let mut out = std::fs::File::create(&out_path).expect("the output file opens");
    for record in records {
        writeln!(out, "{record}").expect("the output file writes");
    }
    eprintln!("v5flip: wrote {out_path}");
}
