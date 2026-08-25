//! EXPLORATORY LEVEL-2 DETECTION-LAYER INSTRUMENT (§22 step 9; L2-A5) —
//! sits below every evidentiary tier and is cited by nothing above it.
//!
//! The wake-up probe: per predeclared root position, paired detection
//! evidence for the frozen candidate pairs under both fields — σ0 (the
//! modeled level-0 field) and σ1 (field seats evaluated by the level-1
//! machinery at the declared freeze; that freeze is part of this probe's
//! policy identity, SP-A8). Output contract per pair (LEVEL2-PROBE.md as
//! amended by CE-A6/L2-A5): q̂/τ̂/ĝ under both fields, paired; the §14.6
//! paired-Z evidence columns; per-field 𝓘 interval bounds with the
//! §14.5 verdict; the three wake-up types kept distinct; exact-zero vs
//! practical-zero kept distinct in the record types. Estimates are never
//! receipts; open results are successful outputs; refusals are typed.
//!
//! PREDECLARED CORPUS (single-look discipline, O14 — declared here and in
//! `walt/probes/step9/README.md` before any number was read):
//!
//! 1. The four step-7 shadow flip fixtures (`calibrate::FLIP_FIXTURES`) —
//!    the 18 E0-calibration fixed pairs of step 8 (`probes/step8/e0.jsonl`),
//!    run on the EXACT route (complete-fiber coupled enumeration).
//! 2. The count-timing six-member shape family
//!    (`calibrate::CountTimingSpec`, g = 0..5, drive n0 = 8 — step 8's
//!    exact construction), one pair each ({6-2, 6-4}), run on the SAMPLED
//!    route (the literal plunge position stays blocked on
//!    [[gran-anchor-reconstruction]]; the ignored test
//!    `v5_literal_count_timing_position_reconstructs` is the marker).
//!
//! DECLARED (σ0, σ1) EPOCH PAIR: σ0 = Level0 { n0 = 2 } — the SAME
//! evaluation field as step 8's E0 calibration, so step 8's per-pair
//! baselines are literally this probe's Stage-1 σ0 evidence layer
//! (L2-A6); σ1 = Level1 { n_outer = 4, n0 = 2 } — the standing fieldswap
//! probe-epoch σ1. Frozen candidates at declared schedule [8, 2] (the
//! step-7/step-8 shadow tuple, verbatim). A changed σ0 is a NEW
//! experiment: every record names both FieldIds.
//!
//! DECLARED RISKS AND CONSTANTS: ε_q = 1/20; δ_decision = 1/200 per field
//! scope (m = 2 edge threshold 400); δ_value = 1/100 (split over the two
//! one-sided engines, threshold 200 each); δ_response = 1/100 (threshold
//! 100); δ_practical-zero = 1/100 (threshold 100); betting mixture
//! (1/4, 1/8), (1/4, 1/4), (1/4, 1/2), (1/4, 3/4); sampled world cap 256
//! (a resource limit, never a settlement rule); minimum honest sampled
//! budget 64; exact enumeration budget 4096 worlds.
//!
//! Usage: `wakeup run <out.jsonl> [knobs]` — knobs (positional):
//!   n0_field0 n_outer_field1 n0_field1 n_outer_frozen n0_frozen
//!   world_cap min_worlds exact_budget
//!
//! Records are byte-deterministic — no wall-clock fields (timing goes to
//! stderr only). No floats anywhere.

use std::io::Write as _;
use std::time::Instant;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::rules::Domino;
use walt::solver::adaptive::root_identity;
use walt::solver::calibrate::{
    count_timing_tuple, reconstruct_flip, shadow_tuple, CountTimingSpec, FlipRoot, FLIP_FIXTURES,
};
use walt::solver::evidence::ScopedDelta;
use walt::solver::exposure::{frozen_policy_exposure, WorldDomain};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{CancellationLadder, SplitAggregate};
use walt::solver::mix;
use walt::solver::policy::{DecisionMode, FrozenPolicy, InnerSchedule, TieRule};
use walt::solver::wakeup::{
    exact_paired_detection, refuse_exact_if_oversized, refuse_sampled_if_underfunded,
    sampled_paired_detection, DecisionWakeUp, DetectionRefusal, DetectionRiskPlan,
    ExactPairedDetection, InformationComparison, ResponseWakeUp, SampledDecisionKind,
    SampledDetectionSpec, SampledPairedDetection, ValueWakeUp,
};

/// Frozen instrument seed for the sampled epochs (a declared constant,
/// distinct from every other bin's).
const WAKEUP_SEED: u64 = 0x57A9_EB01_2026_0825;

// ---------------------------------------------------------------------------
// Configuration — the declared probe epoch pair and budgets.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Config {
    n0_field0: u64,
    n_outer_field1: u64,
    n0_field1: u64,
    n_outer_frozen: u64,
    n0_frozen: u64,
    world_cap: u64,
    min_worlds: u64,
    exact_budget: u128,
}

impl Config {
    /// The committed-run defaults: only under these is the σ0 leg
    /// asserted against the step-8/shadow exact win counts.
    fn is_default(&self) -> bool {
        self.n0_field0 == 2
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

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn eps_q() -> BigRational {
    q(1, 20)
}

fn mixture() -> Vec<(BigRational, BigRational)> {
    vec![
        (q(1, 4), q(1, 8)),
        (q(1, 4), q(1, 4)),
        (q(1, 4), q(1, 2)),
        (q(1, 4), q(3, 4)),
    ]
}

fn risk_plan(root_name: &str) -> DetectionRiskPlan {
    DetectionRiskPlan {
        eps_q: eps_q(),
        delta_decision: ScopedDelta::new(format!("pair-decision:{root_name}"), q(1, 200)),
        delta_value: ScopedDelta::new(format!("value-direction:{root_name}"), q(1, 100)),
        delta_response: ScopedDelta::new(format!("response:{root_name}"), q(1, 100)),
        delta_practical_zero: ScopedDelta::new(format!("practical-zero-q0:{root_name}"), q(1, 100)),
        mixture: mixture(),
    }
}

// ---------------------------------------------------------------------------
// JSON helpers (hand-rolled, like the sibling probes — no serde).
// ---------------------------------------------------------------------------

fn tile_json(d: Domino) -> String {
    format!("[{},{}]", d.hi().value(), d.lo().value())
}

fn rational_json(v: &BigRational) -> String {
    format!("\"{}/{}\"", v.numer(), v.denom())
}

fn opt_rational_json(v: Option<&BigRational>) -> String {
    v.map_or("null".to_string(), rational_json)
}

fn interval_json(v: Option<&(BigRational, BigRational)>) -> String {
    v.map_or("null".to_string(), |(lo, hi)| {
        format!("[{},{}]", rational_json(lo), rational_json(hi))
    })
}

fn z_json(z: &[u64; 5]) -> String {
    format!("[{},{},{},{},{}]", z[0], z[1], z[2], z[3], z[4])
}

fn information_json(i: &InformationComparison) -> String {
    format!(
        "{{\"rate0\":{},\"rate1\":{},\"verdict\":\"{}\"}}",
        interval_json(i.rate0.as_ref()),
        interval_json(i.rate1.as_ref()),
        i.verdict.tag(),
    )
}

fn ladder_json(l: &CancellationLadder) -> String {
    format!(
        "{{\"worlds\":{},\"exposed\":{},\"outcome_changed\":{},\"c_plus\":{},\
         \"c_minus\":{},\"d\":{},\"r\":{},\"c\":{}}}",
        l.worlds,
        l.exposed,
        l.outcome_changed,
        l.c_plus,
        l.c_minus,
        rational_json(&l.d()),
        rational_json(&l.r()),
        rational_json(&l.c()),
    )
}

fn splits_json(s: &SplitAggregate) -> String {
    let by_trick: Vec<String> = s
        .by_trick
        .iter()
        .map(|(t, n)| format!("{{\"trick\":{t},\"splits\":{n}}}"))
        .collect();
    format!(
        "{{\"exposed\":{},\"plus\":{},\"minus\":{},\"by_seat\":[{},{},{},{}],\
         \"by_trick\":[{}],\"conditional_outcome_difference\":{}}}",
        s.exposed,
        s.plus,
        s.minus,
        s.by_seat[0],
        s.by_seat[1],
        s.by_seat[2],
        s.by_seat[3],
        by_trick.join(","),
        s.conditional_outcome_difference()
            .map_or("null".to_string(), |d| format!("\"{d}\"")),
    )
}

fn response_json(r: &ResponseWakeUp) -> String {
    match r {
        ResponseWakeUp::Exact {
            dq,
            eps_q,
            positive,
            exceeds_eps,
        } => format!(
            "{{\"tag\":\"exact\",\"dq\":{},\"eps_q\":{},\"positive\":{positive},\
             \"exceeds_eps\":{exceeds_eps}}}",
            rational_json(dq),
            rational_json(eps_q),
        ),
        ResponseWakeUp::SampledEstablished(e) => format!(
            "{{\"tag\":\"sampled-established\",\"eps_q\":{},\"delta\":\"{}\",\
             \"settled_at\":{},\"evidence\":{}}}",
            rational_json(e.eps_q()),
            e.delta(),
            e.settled_at(),
            rational_json(e.evidence()),
        ),
        ResponseWakeUp::SampledOpen {
            eps_q,
            consumed,
            evidence,
        } => format!(
            "{{\"tag\":\"sampled-open\",\"eps_q\":{},\"consumed\":{consumed},\
             \"evidence\":{}}}",
            rational_json(eps_q),
            rational_json(evidence),
        ),
    }
}

fn value_json(v: &ValueWakeUp) -> String {
    match v {
        ValueWakeUp::Exact { gap_change, wake } => format!(
            "{{\"tag\":\"exact\",\"gap_change\":{},\"wake\":{wake}}}",
            rational_json(gap_change),
        ),
        ValueWakeUp::SampledSettled(s) => format!(
            "{{\"tag\":\"sampled-settled\",\"direction\":\"{}\",\"settled_at\":{},\
             \"evidence\":{}}}",
            s.direction().tag(),
            s.settled_at(),
            rational_json(s.evidence()),
        ),
        ValueWakeUp::SampledOpen {
            consumed,
            evidence_up,
            evidence_down,
            z_mean_hat,
        } => format!(
            "{{\"tag\":\"sampled-open\",\"consumed\":{consumed},\"evidence_up\":{},\
             \"evidence_down\":{},\"z_mean_hat\":{}}}",
            rational_json(evidence_up),
            rational_json(evidence_down),
            rational_json(z_mean_hat),
        ),
    }
}

fn decision_json(d: &DecisionWakeUp) -> String {
    match d {
        DecisionWakeUp::Exact {
            winner0,
            winner1,
            changed,
        } => format!(
            "{{\"tag\":\"exact\",\"winner0\":\"{}\",\"winner1\":\"{}\",\"changed\":{changed}}}",
            winner0.tag(),
            winner1.tag(),
        ),
        DecisionWakeUp::Sampled(kind) => {
            let legs = match kind {
                SampledDecisionKind::Changed { settle0, settle1 }
                | SampledDecisionKind::SameWinner { settle0, settle1 } => format!(
                    "\"settle0\":{},\"settle1\":{}",
                    settle_json(settle0),
                    settle_json(settle1)
                ),
                SampledDecisionKind::NewlySettled { settle1 } => {
                    format!("\"settle0\":null,\"settle1\":{}", settle_json(settle1))
                }
                SampledDecisionKind::NewlyOpen { settle0 } => {
                    format!("\"settle0\":{},\"settle1\":null", settle_json(settle0))
                }
                SampledDecisionKind::BothOpen => "\"settle0\":null,\"settle1\":null".to_string(),
            };
            format!("{{\"tag\":\"sampled\",\"kind\":\"{}\",{legs}}}", kind.tag())
        }
    }
}

fn settle_json(s: &walt::solver::wakeup::PairDecisionSettled) -> String {
    format!(
        "{{\"winner\":\"{}\",\"settled_at\":{},\"evidence\":{}}}",
        s.winner().tag(),
        s.settled_at(),
        rational_json(s.evidence()),
    )
}

fn refusal_json(name: &str, r: &DetectionRefusal) -> String {
    format!(
        "{{\"kind\":\"refusal\",\"root\":\"{name}\",\"root_id\":\"{:#018x}\",\"field0\":\"{}\",\
         \"field1\":\"{}\",\"route\":\"{}\",\"reason\":\"{}\"}}",
        r.root_id, r.field0, r.field1, r.route, r.reason,
    )
}

// ---------------------------------------------------------------------------
// The exact route: the four flip fixtures, all pairs (step 8's 18).
// ---------------------------------------------------------------------------

fn exact_pair_json(name: &str, d: &ExactPairedDetection) -> String {
    format!(
        "{{\"kind\":\"pair\",\"route\":\"exact\",\"root\":\"{name}\",\
         \"root_id\":\"{:#018x}\",\"field0\":\"{}\",\"field1\":\"{}\",\
         \"tile_a\":{},\"tile_b\":{},\"policy_a\":\"{}\",\"policy_b\":\"{}\",\
         \"fiber\":{},\
         \"coords0\":{{\"a\":\"{}\",\"b\":\"{}\",\"q\":{},\"g\":{},\"tau\":{}}},\
         \"coords1\":{{\"a\":\"{}\",\"b\":\"{}\",\"q\":{},\"g\":{},\"tau\":{}}},\
         \"z_counts\":{},\
         \"response\":{},\"value\":{},\"decision\":{},\"information\":{},\
         \"ladder_a\":{},\"ladder_b\":{},\"splits_a\":{},\"splits_b\":{}}}",
        d.root_id,
        d.field0,
        d.field1,
        tile_json(d.tile_a),
        tile_json(d.tile_b),
        d.policy_a,
        d.policy_b,
        d.fiber,
        d.coords0.a,
        d.coords0.b,
        rational_json(&d.coords0.q),
        rational_json(&d.coords0.g),
        opt_rational_json(d.coords0.tau.as_ref()),
        d.coords1.a,
        d.coords1.b,
        rational_json(&d.coords1.q),
        rational_json(&d.coords1.g),
        opt_rational_json(d.coords1.tau.as_ref()),
        z_json(&d.z_counts),
        response_json(&d.response),
        value_json(&d.value),
        decision_json(&d.decision),
        information_json(&d.information),
        ladder_json(&d.ladder_a),
        ladder_json(&d.ladder_b),
        splits_json(&d.splits_a),
        splits_json(&d.splits_b),
    )
}

fn run_flip_fixture(fixture: usize, cfg: Config) -> Vec<String> {
    let f = &FLIP_FIXTURES[fixture];
    let name = format!("step9-{}-h{}-d{}", f.mode, f.hand, f.d);
    let started = Instant::now();
    let flip = reconstruct_flip(f);
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    let root_id = root_identity(&flip.root, &flip.position);
    let mut records: Vec<String> = Vec::new();
    records.push(root_record(
        &name, &flip, &field0, &field1, root_id, cfg, "exact",
    ));
    if let Err(refusal) = refuse_exact_if_oversized(
        root_id,
        field0.field_id(),
        field1.field_id(),
        flip.root.count(),
        cfg.exact_budget,
    ) {
        records.push(refusal_json(&name, &refusal));
        eprintln!("wakeup: {name} REFUSED (exact budget)");
        return records;
    }
    let candidates: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| {
            let mut tuple = shadow_tuple(&flip.position, *t);
            tuple.inner_schedule = InnerSchedule::Declared(vec![cfg.n_outer_frozen, cfg.n0_frozen]);
            FrozenPolicy::new(tuple)
        })
        .collect();
    // One complete-fiber coupled exposure per candidate, shared across the
    // candidate's pairs.
    let exposures: Vec<_> = candidates
        .iter()
        .map(|rho| {
            frozen_policy_exposure(
                &flip.root,
                &flip.position,
                rho,
                &field0,
                &field1,
                WorldDomain::ExactFiber,
            )
        })
        .collect();
    // Step-8 consumption check (L2-A6): at the committed defaults the σ0
    // leg's per-candidate exact win totals must reproduce the shadow
    // run's recorded exact wins — the same numbers step 8's e0
    // calibration was built on.
    if cfg.is_default() {
        for (k, exposure) in exposures.iter().enumerate() {
            let wins0 = exposure.rows.iter().filter(|r| r.u0).count() as u128;
            assert_eq!(
                wins0, f.exact_wins[k],
                "{name}: the σ0 leg reproduces the recorded exact wins"
            );
        }
        eprintln!("wakeup: {name} σ0 exact-wins CHECK PASSED");
    }
    let e = eps_q();
    for i in 0..f.m {
        for j in (i + 1)..f.m {
            let detection = exact_paired_detection(
                &exposures[i],
                &exposures[j],
                flip.legal_tiles[i],
                flip.legal_tiles[j],
                &e,
            );
            records.push(exact_pair_json(&name, &detection));
        }
    }
    eprintln!(
        "wakeup: {name} done (fiber {}, m {}, {}us)",
        flip.root.count(),
        f.m,
        started.elapsed().as_micros(),
    );
    records
}

// ---------------------------------------------------------------------------
// The sampled route: the count-timing six-member shape family.
// ---------------------------------------------------------------------------

fn sampled_pair_json(name: &str, d: &SampledPairedDetection) -> String {
    format!(
        "{{\"kind\":\"pair\",\"route\":\"sampled\",\"root\":\"{name}\",\
         \"root_id\":\"{:#018x}\",\"field0\":\"{}\",\"field1\":\"{}\",\
         \"epoch\":\"{:016x}\",\
         \"tile_a\":{},\"tile_b\":{},\"policy_a\":\"{}\",\"policy_b\":\"{}\",\
         \"consumed\":{},\"world_cap\":{},\"stopped_early\":{},\
         \"coords0\":{{\"a\":{},\"b\":{},\"worlds\":{},\"q_hat\":{},\"g_hat\":{},\
         \"tau_hat\":{}}},\
         \"coords1\":{{\"a\":{},\"b\":{},\"worlds\":{},\"q_hat\":{},\"g_hat\":{},\
         \"tau_hat\":{}}},\
         \"z_counts\":{},\
         \"q0_practical\":{},\"q0_practical_evidence\":{},\
         \"response\":{},\"value\":{},\"decision\":{},\"information\":{},\
         \"splits_a\":{},\"splits_b\":{}}}",
        d.root_id,
        d.field0,
        d.field1,
        d.epoch,
        tile_json(d.tile_a),
        tile_json(d.tile_b),
        d.policy_a,
        d.policy_b,
        d.consumed,
        d.world_cap,
        d.stopped_early,
        d.coords0.a,
        d.coords0.b,
        d.coords0.worlds,
        rational_json(&d.coords0.q_hat()),
        rational_json(&d.coords0.g_hat()),
        opt_rational_json(d.coords0.tau_hat().as_ref()),
        d.coords1.a,
        d.coords1.b,
        d.coords1.worlds,
        rational_json(&d.coords1.q_hat()),
        rational_json(&d.coords1.g_hat()),
        opt_rational_json(d.coords1.tau_hat().as_ref()),
        z_json(&d.z_counts),
        d.q0_practical.as_ref().map_or("null".to_string(), |p| {
            format!(
                "{{\"eps_q\":{},\"delta\":\"{}\",\"settled_at\":{},\"evidence\":{}}}",
                rational_json(p.eps_q()),
                p.delta(),
                p.settled_at(),
                rational_json(p.evidence()),
            )
        }),
        rational_json(&d.q0_practical_evidence),
        response_json(&d.response),
        value_json(&d.value),
        decision_json(&d.decision),
        information_json(&d.information),
        splits_json(&d.splits_a),
        splits_json(&d.splits_b),
    )
}

fn run_count_timing(g: u64, cfg: Config) -> Vec<String> {
    let name = format!("step9-ct-g{g}");
    let started = Instant::now();
    // Step 8's exact construction: family member g at drive n0 = 8.
    let spec = CountTimingSpec::new(g, 8);
    let flip: FlipRoot = spec.root();
    let field0 = FieldModel::new(field0_spec(cfg));
    let field1 = FieldModel::new(field1_spec(cfg));
    let root_id = root_identity(&flip.root, &flip.position);
    let mut records: Vec<String> = Vec::new();
    records.push(root_record(
        &name, &flip, &field0, &field1, root_id, cfg, "sampled",
    ));
    if let Err(refusal) = refuse_sampled_if_underfunded(
        root_id,
        field0.field_id(),
        field1.field_id(),
        cfg.world_cap,
        cfg.min_worlds,
    ) {
        records.push(refusal_json(&name, &refusal));
        eprintln!("wakeup: {name} REFUSED (sampled budget)");
        return records;
    }
    let candidates: Vec<FrozenPolicy> = flip
        .legal_tiles
        .iter()
        .map(|t| {
            let mut tuple = count_timing_tuple(&flip.position, *t);
            tuple.inner_schedule = InnerSchedule::Declared(vec![cfg.n_outer_frozen, cfg.n0_frozen]);
            FrozenPolicy::new(tuple)
        })
        .collect();
    assert_eq!(candidates.len(), 2, "the count-timing decision is one pair");
    let plan = risk_plan(&name);
    let epoch = mix(WAKEUP_SEED ^ mix(0x100 + g));
    let detection = sampled_paired_detection(&SampledDetectionSpec {
        root: &flip.root,
        position: &flip.position,
        tile_a: flip.legal_tiles[0],
        tile_b: flip.legal_tiles[1],
        policy_a: &candidates[0],
        policy_b: &candidates[1],
        field0: &field0,
        field1: &field1,
        epoch,
        world_cap: cfg.world_cap,
        plan: &plan,
    });
    records.push(sampled_pair_json(&name, &detection));
    eprintln!(
        "wakeup: {name} done (consumed {}, response {}, value {}, decision {}, {}us)",
        detection.consumed,
        detection.response.tag(),
        detection.value.tag(),
        detection.decision.tag(),
        started.elapsed().as_micros(),
    );
    records
}

// ---------------------------------------------------------------------------
// Shared root record.
// ---------------------------------------------------------------------------

fn root_record(
    name: &str,
    flip: &FlipRoot,
    field0: &FieldModel,
    field1: &FieldModel,
    root_id: u64,
    cfg: Config,
    route: &str,
) -> String {
    let legal: Vec<String> = flip.legal_tiles.iter().map(|t| tile_json(*t)).collect();
    format!(
        "{{\"kind\":\"root\",\"root\":\"{name}\",\"route\":\"{route}\",\
         \"root_id\":\"{:#018x}\",\"decl\":\"{}\",\"bid\":{},\"declaring_team\":{},\
         \"viewer\":{},\"fiber\":\"{}\",\"legal\":[{}],\
         \"epoch_pair\":{{\"field0\":{{\"id\":\"{}\",\"kind\":\"level0\",\"n0\":{}}},\
         \"field1\":{{\"id\":\"{}\",\"kind\":\"level1\",\"n_outer\":{},\"n0\":{}}}}},\
         \"frozen_schedule\":[{},{}],\"eps_q\":\"1/20\",\
         \"deltas\":{{\"decision\":\"1/200 per field scope\",\"value\":\"1/100 split\",\
         \"response\":\"1/100\",\"practical_zero\":\"1/100\"}},\
         \"mixture\":\"(1/4,1/8),(1/4,1/4),(1/4,1/2),(1/4,3/4)\",\
         \"world_cap\":{},\"min_worlds\":{},\"exact_budget\":\"{}\"}}",
        root_id,
        flip.position.decl,
        flip.position.bid,
        flip.position.declaring_team.index(),
        flip.focal.index(),
        flip.root.count(),
        legal.join(","),
        field0.field_id(),
        cfg.n0_field0,
        field1.field_id(),
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.world_cap,
        cfg.min_worlds,
        cfg.exact_budget,
    )
}

// ---------------------------------------------------------------------------
// Driver.
// ---------------------------------------------------------------------------

enum Unit {
    Flip(usize),
    CountTiming(u64),
}

fn run(out_path: &str, cfg: Config) {
    let units: Vec<Unit> = (0..FLIP_FIXTURES.len())
        .map(Unit::Flip)
        .chain((0..6).map(Unit::CountTiming))
        .collect();
    eprintln!(
        "wakeup: {} units; field0 n0={}, field1 {}x{}, frozen {}x{}, cap {}, min {}",
        units.len(),
        cfg.n0_field0,
        cfg.n_outer_field1,
        cfg.n0_field1,
        cfg.n_outer_frozen,
        cfg.n0_frozen,
        cfg.world_cap,
        cfg.min_worlds,
    );
    let run_unit = |unit: &Unit| -> Vec<String> {
        match unit {
            Unit::Flip(fixture) => run_flip_fixture(*fixture, cfg),
            Unit::CountTiming(g) => run_count_timing(*g, cfg),
        }
    };
    #[cfg(feature = "parallel")]
    let per_unit: Vec<Vec<String>> = units.par_iter().map(run_unit).collect();
    #[cfg(not(feature = "parallel"))]
    let per_unit: Vec<Vec<String>> = units.iter().map(run_unit).collect();
    let mut out = std::fs::File::create(out_path).expect("the output file opens");
    for records in per_unit {
        for record in records {
            writeln!(out, "{record}").expect("the output file writes");
        }
    }
    eprintln!("wakeup: wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("run");
    match mode {
        "run" => {
            let out_path = args
                .get(2)
                .cloned()
                .unwrap_or_else(|| "wakeup.jsonl".to_string());
            let mut knobs = args.iter().skip(3);
            let mut knob = |default: u64| -> u64 {
                knobs
                    .next()
                    .map(|v| v.parse().expect("an integer knob"))
                    .unwrap_or(default)
            };
            let cfg = Config {
                n0_field0: knob(2),
                n_outer_field1: knob(4),
                n0_field1: knob(2),
                n_outer_frozen: knob(8),
                n0_frozen: knob(2),
                world_cap: knob(256),
                min_worlds: knob(64),
                exact_budget: u128::from(knob(4096)),
            };
            run(&out_path, cfg);
        }
        other => panic!("unknown mode {other:?}; expected run"),
    }
}
