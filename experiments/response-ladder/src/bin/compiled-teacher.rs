//! Exact finite-bundle teachers with explicitly distinct sampling targets.
//!
//! The worker accepts one public-information request per JSON line.  It
//! The default retains compatible-root-reset-v1. Optional historical H0
//! and compatible-H0-seed diagnostics use the native H0 seed and fixed n=8.
//! Historical inner worlds are deliberately Voidless, while observations
//! remain lawful. They are evaluated only by the frozen native Dice Solver.
//! A deadline at any point produces a refusal; no incomplete vector is
//! serialized as a lesson.

use num_rational::BigRational;
use num_traits::{One, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};
use walt::rules::Decl;
use walt::solver::{self, decl_of, Shared, Solver, FULL_MASK};
use walt_response_ladder::compiled;
#[cfg(test)]
use walt_response_ladder::compiled::Actor;
use walt_response_ladder::compiled_family::Family;
use walt_response_ladder::compiled_field::CompiledField;
use walt_response_ladder::core::{self, Config};
use walt_response_ladder::mechanics::{tiles, Budget, Field, Problem, PublicState, Scenario};
use walt_response_ladder::model;

const SCHEMA: &str = "compiled-teacher-v1";
const TARGET: &str = "compatible-root-reset-v1";
const FIELD_REVISION: &str = "historical-dice-v1";
const DEFAULT_N: usize = 8;
const WORK_LIMIT: u64 = u64::MAX / 2;

enum TeacherMode {
    T0,
    HistoricalH0,
    CompatibleH0Seed,
    T1 {
        actor: Family,
        field_revision: String,
    },
}

impl TeacherMode {
    fn target(&self) -> String {
        match self {
            Self::HistoricalH0 => "historical-voidless-h0-v1",
            Self::CompatibleH0Seed => "compatible-root-h0-seed-v1",
            _ => TARGET,
        }.into()
    }

    fn revision(&self) -> String {
        match self {
            Self::T0 => "t0-dice-exact-v1".into(),
            Self::HistoricalH0 => "h0-native-dice-exact-v1".into(),
            Self::CompatibleH0Seed => "t0-h0-seed-dice-exact-v1".into(),
            Self::T1 { .. } => "t1-br-compiled-c0-v1".into(),
        }
    }

    fn mode_name(&self) -> &'static str {
        match self {
            Self::T0 => "compatible-t0",
            Self::HistoricalH0 => "historical-h0",
            Self::CompatibleH0Seed => "compatible-h0-seed",
            Self::T1 { .. } => "compatible-t1",
        }
    }

    fn prior(&self) -> &'static str {
        if matches!(self, Self::HistoricalH0) { "historical-voidless" }
        else { "public-void-compatible-shuffle-reject" }
    }

    fn native_h0_seed(&self) -> bool {
        matches!(self, Self::HistoricalH0 | Self::CompatibleH0Seed)
    }

    fn seed_rule(&self) -> &'static str {
        if self.native_h0_seed() { "native-h0-seat-current-hand-record-v1" }
        else { "request-seed-v1" }
    }

    fn field_revision(&self) -> &str {
        match self {
            Self::T0 | Self::HistoricalH0 | Self::CompatibleH0Seed => FIELD_REVISION,
            Self::T1 { field_revision, .. } => field_revision,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    decl: usize,
    bid: u8,
    bidder: u8,
    seat: u8,
    hand: Vec<u8>,
    original_hand: Vec<u8>,
    history: Vec<[u8; 2]>,
    seed: u64,
    budget_ms: f64,
    #[serde(default = "default_n")]
    n: usize,
}

fn default_n() -> usize {
    DEFAULT_N
}

#[derive(Clone, Debug, Serialize)]
struct NormalizedState {
    decl: usize,
    bid: u8,
    viewer: u8,
    hand: u32,
    public: PublicState,
}

#[derive(Clone, Debug, Serialize)]
struct Bundle {
    identity: String,
    target: String,
    revision: String,
    field_revision: String,
    mode: &'static str,
    prior: &'static str,
    seed_rule: &'static str,
    /// Effective sampling seed; request.seed remains separately in the receipt.
    seed: u64,
    n: usize,
    mass: u64,
    /// Original ordered scenario columns, including duplicate deals and
    /// their tapes.  Masks use the normalized internal seat order.
    scenarios: Vec<Scenario>,
}

#[derive(Clone, Debug, Serialize)]
struct Cost {
    action: u8,
    numerator: u64,
    denominator: u64,
}

#[derive(Clone, Debug, Serialize)]
struct Output {
    schema: &'static str,
    status: &'static str,
    request: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_state: Option<NormalizedState>,
    target: String,
    revision: String,
    field_revision: String,
    mode: &'static str,
    prior: &'static str,
    seed_rule: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle: Option<Bundle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_counts: Option<Vec<(u8, u64)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    costs: Option<Vec<Cost>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maxcount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    denominator: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mass: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canonical_action: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    clause_actions: Option<Vec<Option<u8>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_schema: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    objective: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    elapsed_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    budget_used: Option<u64>,
}

fn base(request: Value, start: Instant, status: &'static str, mode: &TeacherMode) -> Output {
    Output {
        schema: SCHEMA,
        status,
        request,
        normalized_state: None,
        target: mode.target(),
        revision: mode.revision(),
        field_revision: mode.field_revision().into(),
        mode: mode.mode_name(),
        prior: mode.prior(),
        seed_rule: mode.seed_rule(),
        bundle: None,
        action: None,
        action_counts: None,
        costs: None,
        maxcount: None,
        denominator: None,
        mass: None,
        canonical_action: None,
        clause_actions: None,
        feature_schema: None,
        objective: None,
        reason: None,
        elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
        budget_used: None,
    }
}

fn mask(ts: &[u8]) -> Result<u32, String> {
    let mut result = 0u32;
    for &tile in ts {
        if tile >= 28 || result & (1u32 << tile) != 0 {
            return Err("bad or repeated tile".into());
        }
        result |= 1u32 << tile;
    }
    Ok(result)
}

fn declaration(id: usize) -> Result<Decl, String> {
    if id <= 7 || id == 9 {
        Ok(decl_of(id))
    } else {
        Err("declaration must be 0..7 or 9".into())
    }
}

fn normalized(
    decl_id: usize,
    bid: u8,
    viewer: u8,
    hand: u32,
    public: &PublicState,
) -> NormalizedState {
    NormalizedState {
        decl: decl_id,
        bid,
        viewer,
        hand,
        public: public.clone(),
    }
}

fn bundle_identity(problem: &Problem, seed: u64) -> String {
    // FNV-1a is deliberately kept local so the identity has no dependency on
    // a non-authoritative digest implementation.  The full scenarios remain
    // in the receipt, so this is an index as well as a compact audit handle.
    let mut hash = 0xcbf29ce484222325u64;
    let mut add = |value: u64| {
        for byte in value.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    };
    add(seed);
    let decl_id = match problem.decl {
        Decl::PipTrump(pip) => u64::from(pip.value()),
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 9,
    };
    add(decl_id);
    add(u64::from(problem.bid));
    add(u64::from(problem.viewer));
    add(u64::from(problem.hand));
    add(u64::from(problem.root.played));
    add(u64::from(problem.root.leader));
    for &tile in &problem.root.history {
        add(u64::from(tile));
    }
    for scenario in &problem.scenarios {
        for &hand in &scenario.hands {
            add(u64::from(hand));
        }
        add(scenario.tape);
        add(scenario.weight);
    }
    format!("ordered-scenarios-v1-{hash:016x}")
}

fn bundle(problem: &Problem, seed: u64, mode: &TeacherMode) -> Bundle {
    let identity = match mode {
        TeacherMode::T0 => bundle_identity(problem, seed),
        TeacherMode::T1 { .. } | TeacherMode::HistoricalH0 | TeacherMode::CompatibleH0Seed => {
            format!("{}-{}", mode.revision(), bundle_identity(problem, seed))
        },
    };
    Bundle {
        identity,
        target: mode.target(),
        revision: mode.revision(),
        field_revision: mode.field_revision().into(),
        mode: mode.mode_name(),
        prior: mode.prior(),
        seed_rule: mode.seed_rule(),
        seed,
        n: problem.scenarios.len(),
        mass: problem.mass(),
        scenarios: problem.scenarios.clone(),
    }
}

/// Exact native H0 stream: no request seed, original hand, banked score, or
/// public void coordinate enters this seed. Those scores still affect utility.
fn h0_seed(viewer: u8, hand: u32, public: &PublicState) -> u64 {
    solver::INNER_SEED ^ solver::mix(u64::from(viewer))
        ^ solver::mix(u64::from(hand)) ^ solver::record_hash(&public.key())
}

#[allow(clippy::too_many_arguments)]
fn sample_teacher_problem(mode: &TeacherMode, decl: Decl, bid: u8, viewer: u8,
    hand: u32, public: &PublicState, n: usize, seed: u64, budget: &mut Budget,
) -> Result<Option<Problem>, String> {
    if !matches!(mode, TeacherMode::HistoricalH0) {
        return model::sample_problem(decl, bid, viewer, hand, public, n, seed,
            mode.field_revision().into(), budget);
    }
    let (_, _, sizes) = model::frame(public)?;
    let mut rng = solver::SplitMix64(seed);
    let Some(worlds) = solver::InnerBelief::Voidless.sample(
        decl, walt::rules::Seat::from_index(viewer as usize).ok_or("invalid seat")?,
        hand, &public.key(), sizes, n, &mut rng, budget.deadline,
    ) else { return Ok(None); };
    // Keep original IDs and all duplicate worlds. Tapes are drawn only after
    // every in-place shuffle, exactly as native pi(0).
    let scenarios = worlds.into_iter().map(|hands| Scenario {
        hands, tape: rng.next_u64(), weight: 1,
    }).collect();
    if budget.deadline.passed() { return Ok(None); }
    let problem = Problem { decl, bid, viewer, hand, root: public.clone(), scenarios,
        field_revision: mode.field_revision().into() };
    // This validator checks mechanical capacities/conservation, not historical
    // void compatibility. Counterfactual inner worlds are not compiled inputs.
    problem.validate()?;
    Ok(Some(problem))
}

fn own_count(value: &BigRational, mass: u64, maximize: bool) -> Result<u64, String> {
    let scaled = value.clone() * BigRational::from_integer(mass.into());
    if !scaled.denom().is_one() {
        return Err("solver returned a utility whose denominator is not bundle mass".into());
    }
    let count = scaled
        .to_integer()
        .to_u64()
        .ok_or_else(|| "solver returned an unrepresentable utility count".to_string())?;
    if maximize {
        Ok(count)
    } else {
        mass.checked_sub(count)
            .ok_or_else(|| "utility count exceeds bundle mass".into())
    }
}

fn refusal(
    mut output: Output,
    reason: impl Into<String>,
    start: Instant,
    budget: &Budget,
) -> Output {
    output.status = "refused";
    output.reason = Some(reason.into());
    output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    output.budget_used = Some(budget.used);
    output
}

#[cfg(test)]
fn answer(request: Value, req: Request) -> Output {
    answer_with_mode(request, req, &TeacherMode::T0)
}

#[cfg(test)]
fn answer_with_mode(request: Value, req: Request, mode: &TeacherMode) -> Output {
    answer_with_schema(request, req, mode, compiled::SCHEMA)
}

fn answer_with_schema(request: Value, req: Request, mode: &TeacherMode, feature_schema: &'static str) -> Output {
    let start = Instant::now();
    let mut output = base(request, start, "error", mode);
    if feature_schema == compiled::SCHEMA_V2 {
        output.schema = "compiled-teacher-v2";
        output.feature_schema = Some(compiled::SCHEMA_V2);
    }

    if mode.native_h0_seed() && req.n != DEFAULT_N {
        output.reason = Some("native-H0 diagnostic modes require fixed n=8".into());
        return output;
    }
    if req.bid != 30 {
        output.reason = Some("compiled teacher accepts bid 30 only".into());
        return output;
    }
    if req.bidder >= 4
        || req.seat >= 4
        || req.n == 0
        || req.n > 4096
        || !req.budget_ms.is_finite()
        || !(0.0..=86_400_000.0).contains(&req.budget_ms)
    {
        output.reason = Some("invalid bidder, seat, n, or budget_ms".into());
        return output;
    }
    let decl = match declaration(req.decl) {
        Ok(decl) => decl,
        Err(reason) => {
            output.reason = Some(reason);
            return output;
        }
    };
    let hand = match mask(&req.hand) {
        Ok(mask) => mask,
        Err(reason) => {
            output.reason = Some(reason);
            return output;
        }
    };
    let original_hand = match mask(&req.original_hand) {
        Ok(mask) => mask,
        Err(reason) => {
            output.reason = Some(reason);
            return output;
        }
    };

    // Match response-player's arena normalization exactly: even bidders are
    // rotated by one seat so the declaring team is internal seats 1 and 3.
    let rotation = u8::from(req.bidder % 2 == 0);
    let viewer = (req.seat + rotation) % 4;
    let mut public = PublicState::opening((req.bidder + rotation) % 4);
    for [actor, tile] in &req.history {
        if *actor >= 4
            || (*actor + rotation) % 4 != public.actor()
            || *tile >= 28
            || public.played & (1u32 << tile) != 0
        {
            output.reason = Some("invalid public history".into());
            return output;
        }
        let internal_actor = (*actor + rotation) % 4;
        if mode.native_h0_seed() && public.voids[internal_actor as usize] & (1u32 << tile) != 0 {
            output.reason = Some("public history contradicts an earlier failure to follow".into());
            return output;
        }
        let in_original = original_hand & (1u32 << tile) != 0;
        if in_original != (internal_actor == viewer) {
            output.reason = Some("public history disagrees with original_hand".into());
            return output;
        }
        if internal_actor == viewer
            && public.legal(decl, original_hand & !public.played) & (1u32 << tile) == 0
        {
            output.reason = Some("viewer history contains an illegal follow or play".into());
            return output;
        }
        public = public.after(decl, *tile);
    }

    let state = normalized(req.decl, req.bid, viewer, hand, &public);
    output.normalized_state = Some(state);
    if public.actor() != viewer {
        output.reason = Some("public history does not end at the requested viewer".into());
        return output;
    }
    if original_hand & !FULL_MASK != 0
        || original_hand.count_ones() != 7
        || original_hand & !public.played != hand
    {
        output.reason = Some("invalid original_hand/current hand relationship".into());
        return output;
    }
    let (_, _, sizes) = match model::frame(&public) {
        Ok(frame) => frame,
        Err(reason) => {
            output.reason = Some(reason);
            return output;
        }
    };
    if hand == 0
        || hand & public.played != 0
        || hand.count_ones() as usize != sizes[viewer as usize]
    {
        output.reason = Some("hand disagrees with public frame".into());
        return output;
    }
    if mode.native_h0_seed() {
        // Validate the actual observation, independently of the intentionally
        // relaxed hidden completions used by the historical inner mind.
        if let Err(error) = solver::belief_frame_feasibility(
            viewer as usize, hand, public.played, sizes, public.voids,
        ) {
            output.reason = Some(format!("infeasible public observation: {error:?}"));
            return output;
        }
        if let Err(error) = compiled::clause_actions_for_schema(feature_schema, decl, viewer, hand, &public) {
            output.reason = Some(error);
            return output;
        }
    }
    let legal = tiles(public.legal(decl, hand));
    if legal.is_empty() {
        output.reason = Some("no legal play".into());
        return output;
    }
    if legal.len() == 1 {
        output.status = "forced";
        output.action = Some(legal[0]);
        output.reason = Some("only legal action".into());
        output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        return output;
    }
    if public.payoff(req.bid, viewer).is_some() {
        output.status = "settled";
        output.reason = Some("contract already settled".into());
        output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        return output;
    }

    let duration = Duration::from_secs_f64(req.budget_ms / 1000.0);
    let mut budget = Budget::new(WORK_LIMIT, duration);
    let effective_seed = if mode.native_h0_seed() { h0_seed(viewer, hand, &public) } else { req.seed };
    let problem = match sample_teacher_problem(
        mode, decl, req.bid, viewer, hand, &public, req.n, effective_seed, &mut budget,
    ) {
        Ok(Some(problem)) => problem,
        Ok(None) => return refusal(output, "deadline_or_budget_during_sampling", start, &budget),
        Err(reason) => {
            output.reason = Some(reason);
            output.budget_used = Some(budget.used);
            output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            return output;
        }
    };
    output.bundle = Some(bundle(&problem, effective_seed, mode));

    let mass = problem.mass();
    let maximize = viewer % 2 == 1;
    let counts: Result<Vec<(u8, u64)>, String> =
        match mode {
            TeacherMode::T0 | TeacherMode::HistoricalH0 | TeacherMode::CompatibleH0Seed => {
                let (boundary, size, _) = match model::frame(&public) {
                    Ok(frame) => frame,
                    Err(reason) => return refusal(output, reason, start, &budget),
                };
                let shared = std::sync::Arc::new(Shared::new(
                    decl,
                    req.bid,
                    vec![req.n],
                    boundary,
                    size,
                    budget.deadline,
                ));
                let solver = Solver::new(
                    shared,
                    walt::rules::Seat::from_index(viewer as usize).expect("validated seat"),
                    hand,
                    maximize,
                    problem
                        .scenarios
                        .iter()
                        .map(|scenario| scenario.hands)
                        .collect(),
                    problem
                        .scenarios
                        .iter()
                        .map(|scenario| scenario.tape)
                        .collect(),
                    solver::Field::Dice,
                );
                let values = match solver.action_values(&public.key(), &legal) {
                    Some(values) if !budget.deadline.passed() => values,
                    _ => {
                        return refusal(
                            output,
                            "deadline_or_budget_during_exact_solve",
                            start,
                            &budget,
                        )
                    }
                };
                values
                    .iter()
                    .map(|(action, value)| Ok((*action, own_count(value, mass, maximize)?)))
                    .collect()
            }
            TeacherMode::T1 { actor, .. } => {
                let mut field = match CompiledField::all_family(actor) {
                    Ok(field) => field,
                    Err(reason) => {
                        output.reason = Some(reason);
                        output.budget_used = Some(budget.used);
                        output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                        return output;
                    }
                };
                let config = Config {
                    max_horizon: 7,
                    priority_plans: 1,
                    scenario_upper: true,
                    stop_when_certified: false,
                };
                let report = match core::solve(&problem, &mut field, &mut budget, &config) {
                    Ok(report) => report,
                    Err(reason) => {
                        output.reason = Some(reason);
                        output.budget_used = Some(budget.used);
                        output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                        return output;
                    }
                };
                if budget.deadline.passed() || report.interrupted || !report.exact_vector {
                    return refusal(
                        output,
                        "deadline_or_incomplete_compiled_field_solve",
                        start,
                        &budget,
                    );
                }
                if report.mass != mass
                    || report.actions.len() != legal.len()
                    || report.actions.iter().zip(&legal).any(|(bound, &action)| {
                        bound.action != action || bound.lower != bound.upper
                    })
                {
                    return refusal(output, "compiled_field_vector_not_complete", start, &budget);
                }
                Ok(report
                    .actions
                    .iter()
                    .map(|bound| (bound.action, bound.lower))
                    .collect())
            }
        };
    let counts = match counts {
        Ok(counts) => counts,
        Err(reason) => return refusal(output, reason, start, &budget),
    };
    let maxcount = counts.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let costs = counts
        .iter()
        .map(|(action, count)| Cost {
            action: *action,
            numerator: maxcount - *count,
            denominator: mass,
        })
        .collect::<Vec<_>>();
    let canonical_action = counts
        .iter()
        .max_by(|(action_a, count_a), (action_b, count_b)| {
            count_a.cmp(count_b).then_with(|| action_b.cmp(action_a))
        })
        .map(|(action, _)| *action)
        .expect("legal action");
    let clause_actions =
        match compiled::clause_actions_for_schema(feature_schema, decl, viewer, hand, &public) {
            Ok(actions) => actions,
            Err(reason) => return refusal(output, reason, start, &budget),
        };
    output.status = "complete";
    output.action = Some(canonical_action);
    output.action_counts = Some(counts.clone());
    output.costs = Some(costs);
    output.maxcount = Some(maxcount);
    output.denominator = Some(mass);
    output.mass = Some(mass);
    output.canonical_action = Some(canonical_action);
    output.clause_actions = Some(clause_actions);
    output.objective = Some(json!({
        "team": if maximize { "declaring" } else { "defending" },
        "own_utility": counts,
        "denominator": mass,
    }));
    output.budget_used = Some(budget.used);
    output.elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    output
}

fn process_line_with_schema(line: &str, mode: &TeacherMode, feature_schema: &'static str) -> Value {
    let schema = if feature_schema == compiled::SCHEMA_V2 { "compiled-teacher-v2" } else { SCHEMA };
    let raw: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(reason) => {
            return json!({"schema": schema, "feature_schema": feature_schema, "status": "error", "error": reason.to_string()})
        }
    };
    let req: Request = match serde_json::from_value(raw.clone()) {
        Ok(req) => req,
        Err(reason) => {
            return json!({"schema": schema, "feature_schema": feature_schema, "status": "error", "request": raw, "error": reason.to_string()})
        }
    };
    serde_json::to_value(answer_with_schema(raw, req, mode, feature_schema)).expect("teacher response serializes")
}

fn load_c0(path: &str) -> Result<TeacherMode, String> {
    let contents = std::fs::read_to_string(path).map_err(|error| format!("read C0: {error}"))?;
    let actor: Family =
        serde_json::from_str(&contents).map_err(|error| format!("parse C0: {error}"))?;
    actor.validate()?;
    let field = CompiledField::all_family(&actor)?;
    Ok(TeacherMode::T1 {
        actor,
        field_revision: field.revision().to_owned(),
    })
}

fn worker(mode: TeacherMode, feature_schema: &'static str) -> Result<(), String> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut ready = json!({
        "ready": true,
        "schema": if feature_schema == compiled::SCHEMA_V2 { "compiled-teacher-v2" } else { SCHEMA },
        "feature_schema": feature_schema,
        "target": mode.target(),
        "revision": mode.revision(),
        "mode": mode.mode_name(),
        "prior": mode.prior(),
        "seed_rule": mode.seed_rule(),
        "fixed_n": if mode.native_h0_seed() { Some(DEFAULT_N) } else { None },
        "field_revision": mode.field_revision(),
        "default_n": DEFAULT_N,
        "bundle": "ordered scenarios and Dice tapes retained",
    });
    if let TeacherMode::T1 { actor, .. } = &mode {
        ready["c0"] = serde_json::to_value(actor).map_err(|error| error.to_string())?;
    }
    writeln!(out, "{ready}").map_err(|error| error.to_string())?;
    out.flush().map_err(|error| error.to_string())?;
    for line in io::stdin().lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        let value = process_line_with_schema(&line, &mode, feature_schema);
        writeln!(out, "{value}").map_err(|error| error.to_string())?;
        out.flush().map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn usage() {
    println!("usage: compiled-teacher worker [--c0 PATH | --mode historical-h0 | --mode compatible-h0-seed] [--grammar v1|v2]");
    println!("       compiled-teacher --help");
    println!("default worker teaches exact T0 against historical Dice");
    println!("--c0 PATH teaches exact T1 best response against an immutable compiled C0");
    println!("--mode historical-h0 teaches native fixed-n8 Voidless H0; compatible-h0-seed changes only its prior");
}

fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    match args.next().as_deref() {
        Some("worker") => {
            let mut c0_path = None;
            let mut diagnostic_mode = None;
            let mut feature_schema = compiled::SCHEMA;
            while let Some(arg) = args.next() {
                if arg == "--c0" {
                    c0_path = Some(args.next().ok_or_else(|| "missing --c0 path".to_owned()));
                } else if arg == "--mode" {
                    if diagnostic_mode.is_some() {
                        eprintln!("--mode may be supplied once"); std::process::exit(2);
                    }
                    diagnostic_mode = Some(match args.next().as_deref() {
                        Some("historical-h0") => TeacherMode::HistoricalH0,
                        Some("compatible-h0-seed") => TeacherMode::CompatibleH0Seed,
                        _ => { eprintln!("--mode requires historical-h0 or compatible-h0-seed"); std::process::exit(2); }
                    });
                } else if arg == "--grammar" {
                    feature_schema = match args.next().as_deref() {
                        Some("v1") => compiled::SCHEMA,
                        Some("v2") => compiled::SCHEMA_V2,
                        _ => { eprintln!("--grammar requires v1 or v2"); std::process::exit(2); }
                    };
                } else {
                    eprintln!("unknown worker option: {arg}");
                    usage();
                    std::process::exit(2);
                }
            }
            let mode = match (c0_path, diagnostic_mode) {
                (Some(_), Some(_)) => Err("--c0 and --mode are mutually exclusive".into()),
                (None, Some(mode)) => Ok(mode),
                (None, None) => Ok(TeacherMode::T0),
                (Some(Ok(path)), None) => load_c0(&path),
                (Some(Err(error)), None) => Err(error),
            };
            if let Err(error) = mode.and_then(|mode| worker(mode, feature_schema)) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        Some("--help") | Some("-h") | None => usage(),
        Some(other) => {
            eprintln!("unknown command: {other}");
            usage();
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use walt::rules::Seat;
    use walt::solver::{self, Deadline, Shared, SplitMix64};

    #[test]
    fn malformed_requests_preserve_selected_feature_schema() {
        for line in ["not json", "{}"] {
            let value = process_line_with_schema(line, &TeacherMode::T0, compiled::SCHEMA_V2);
            assert_eq!(value["schema"], "compiled-teacher-v2");
            assert_eq!(value["feature_schema"], compiled::SCHEMA_V2);
            assert_eq!(value["status"], "error");
        }
    }

    fn request_value(
        hand: Vec<u8>,
        original_hand: Vec<u8>,
        history: Vec<[u8; 2]>,
        budget_ms: f64,
    ) -> (Value, Request) {
        let value = json!({
            "decl": 9,
            "bid": 30,
            "bidder": 1,
            "seat": 1,
            "hand": hand,
            "original_hand": original_hand,
            "history": history,
            "seed": 9182,
            "budget_ms": budget_ms,
            "n": 1,
        });
        let req = serde_json::from_value(value.clone()).unwrap();
        (value, req)
    }

    fn c0_mode() -> TeacherMode {
        let actor = Actor::new("scheme-relational-actor-v1", vec![5, 8, 2]).unwrap();
        let field = CompiledField::all(&actor).unwrap();
        TeacherMode::T1 {
            actor: Family::Single(actor),
            field_revision: field.revision().to_owned(),
        }
    }

    fn defender_problem(seed: u64) -> Option<Problem> {
        let mut rng = SplitMix64(seed);
        let mut deck: Vec<u8> = (0..28).collect();
        for i in (1..deck.len()).rev() {
            deck.swap(i, rng.below((i + 1) as u64) as usize);
        }
        let mut hands = [0u32; 4];
        for seat in 0..4 {
            hands[seat] = deck[seat * 7..seat * 7 + 7]
                .iter()
                .fold(0, |mask, &tile| mask | (1 << tile));
        }
        let mut remaining = hands;
        let mut public = PublicState::opening(1);
        for _ in 0..20 {
            let actor = public.actor();
            let hand = remaining[actor as usize] & !public.played;
            let legal = tiles(public.legal(Decl::NoTrump, hand));
            let tile = legal[rng.below(legal.len() as u64) as usize];
            remaining[actor as usize] &= !(1 << tile);
            public = public.after(Decl::NoTrump, tile);
        }
        let viewer = public.actor();
        if viewer % 2 != 0 || (remaining[viewer as usize] & !public.played).count_ones() != 2 {
            return None;
        }
        let bid = public.banked_t1 + 1;
        if bid > 42 || public.payoff(bid, viewer).is_some() {
            return None;
        }
        Some(Problem {
            decl: Decl::NoTrump,
            bid,
            viewer,
            hand: remaining[viewer as usize] & !public.played,
            root: public,
            scenarios: vec![Scenario {
                hands: remaining,
                tape: rng.next_u64(),
                weight: 1,
            }],
            field_revision: FIELD_REVISION.into(),
        })
    }

    #[test]
    fn defender_counts_are_the_complement_of_minimizing_solver_values() {
        let (problem, values) = (1..512)
            .filter_map(defender_problem)
            .find_map(|problem| {
                let (boundary, size, _) = model::frame(&problem.root).unwrap();
                let shared = Arc::new(Shared::new(
                    problem.decl,
                    problem.bid,
                    vec![1],
                    boundary,
                    size,
                    Deadline::after(Duration::from_secs(5)),
                ));
                let solver = Solver::new(
                    shared,
                    Seat::from_index(problem.viewer as usize).unwrap(),
                    problem.hand,
                    false,
                    problem.scenarios.iter().map(|s| s.hands).collect(),
                    problem.scenarios.iter().map(|s| s.tape).collect(),
                    solver::Field::Dice,
                );
                let legal = tiles(problem.root.legal(problem.decl, problem.hand));
                let values = solver.action_values(&problem.root.key(), &legal)?;
                (values.windows(2).any(|pair| pair[0].1 != pair[1].1)).then_some((problem, values))
            })
            .expect("lawful defender fixture with distinct action values");
        let mut field = walt_response_ladder::mechanics::DiceField;
        let report = walt_response_ladder::core::solve(
            &problem,
            &mut field,
            &mut Budget::new(20_000_000, Duration::from_secs(5)),
            &walt_response_ladder::core::Config {
                max_horizon: 7,
                priority_plans: 2,
                scenario_upper: true,
                stop_when_certified: false,
            },
        )
        .unwrap();
        assert!(report.exact_vector);
        let native_counts: Vec<_> = values
            .iter()
            .map(|(action, value)| (*action, own_count(value, 1, false).unwrap()))
            .collect();
        for bound in &report.actions {
            let expected = native_counts
                .iter()
                .find(|(action, _)| *action == bound.action)
                .unwrap()
                .1;
            assert_eq!((bound.lower, bound.upper), (expected, expected));
        }
        assert_eq!(
            solver::best_of(&values, false),
            native_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .unwrap()
                .0
        );
    }

    #[test]
    fn malformed_history_actor_and_original_hand_are_rejected() {
        let (raw, req) = request_value((0..7).collect(), (0..7).collect(), vec![[0, 0]], 10.0);
        let actor = answer(raw, req);
        assert_eq!(actor.status, "error");
        assert!(actor.reason.unwrap().contains("public history"));

        let (raw, req) = request_value((0..7).collect(), (1..8).collect(), Vec::new(), 10.0);
        let original = answer(raw, req);
        assert_eq!(original.status, "error");
        assert!(original.reason.unwrap().contains("original_hand"));
    }

    #[test]
    fn viewer_renege_in_history_is_rejected() {
        let hand = (1u32 << 7) - 1;
        let (lead, illegal) = (7..28)
            .find_map(|lead| {
                let public = PublicState::opening(0).after(Decl::NoTrump, lead);
                let legal = public.legal(Decl::NoTrump, hand);
                (legal != hand).then(|| (lead, (hand & !legal).trailing_zeros() as u8))
            })
            .expect("a lead that requires a proper subset of viewer hand");
        let raw = json!({"decl":9, "bid":30, "bidder":0, "seat":1,
            "hand":tiles(hand & !(1 << illegal)), "original_hand":tiles(hand),
            "history":[[0,lead],[1,illegal]], "seed":9182, "budget_ms":10.0, "n":1});
        let req: Request = serde_json::from_value(raw.clone()).unwrap();
        let result = answer(raw.clone(), req.clone());
        assert_eq!(result.status, "error");
        assert!(result.reason.unwrap().contains("illegal follow"));
    }

    #[test]
    fn zero_budget_is_an_explicit_refusal() {
        let (raw, req) = request_value((0..7).collect(), (0..7).collect(), Vec::new(), 0.0);
        let result = answer(raw, req);
        assert_eq!(result.status, "refused");
        assert!(result.reason.unwrap().contains("deadline"));
        assert!(result.action_counts.is_none());
        assert!(result.costs.is_none());
    }

    #[test]
    fn small_lawful_endgame_can_return_a_complete_teacher_vector() {
        let mut rng = SplitMix64(77019);
        let mut found = None;
        for _ in 0..128 {
            let mut deck: Vec<u8> = (0..28).collect();
            for i in (1..deck.len()).rev() {
                deck.swap(i, rng.below((i + 1) as u64) as usize);
            }
            let mut original = [0u32; 4];
            for seat in 0..4 {
                original[seat] = deck[seat * 7..seat * 7 + 7]
                    .iter()
                    .fold(0, |mask, &tile| mask | (1 << tile));
            }
            let mut remaining = original;
            let mut public = PublicState::opening(1);
            let mut history = Vec::new();
            for _ in 0..20 {
                let actor = public.actor();
                let hand = remaining[actor as usize] & !public.played;
                let legal = tiles(public.legal(Decl::NoTrump, hand));
                let tile = legal[rng.below(legal.len() as u64) as usize];
                remaining[actor as usize] &= !(1 << tile);
                history.push([actor, tile]);
                public = public.after(Decl::NoTrump, tile);
            }
            let viewer = public.actor() as usize;
            let hand = remaining[viewer] & !public.played;
            if viewer == 1
                && public.payoff(30, viewer as u8).is_none()
                && tiles(public.legal(Decl::NoTrump, hand)).len() > 1
            {
                found = Some((tiles(hand), tiles(original[viewer]), history));
                break;
            }
        }
        let (hand, original, history) = found.expect("a small unsettled endgame fixture");
        let (raw, req) = request_value(hand, original, history, 5_000.0);
        let result = answer(raw.clone(), req.clone());
        assert_eq!(result.status, "complete", "{:?}", result.reason);
        assert_eq!(result.mass, Some(1));
        assert_eq!(
            result.action_counts.as_ref().unwrap().len(),
            result.costs.as_ref().unwrap().len()
        );
        assert!(result
            .costs
            .as_ref()
            .unwrap()
            .iter()
            .all(|cost| cost.denominator == 1));

        let v2 = answer_with_schema(raw.clone(), req.clone(), &TeacherMode::T0, compiled::SCHEMA_V2);
        assert_eq!(v2.status, "complete");
        assert_eq!(v2.schema, "compiled-teacher-v2");
        assert_eq!(v2.feature_schema, Some(compiled::SCHEMA_V2));
        assert_eq!(v2.action_counts, result.action_counts);
        assert_eq!(v2.clause_actions.as_ref().unwrap().len(), 16);
        assert_eq!(&v2.clause_actions.as_ref().unwrap()[..14], result.clause_actions.as_ref().unwrap());

        let t1 = answer_with_mode(raw.clone(), req.clone(), &c0_mode());
        assert_eq!(t1.status, "complete", "{:?}", t1.reason);
        assert_eq!(t1.revision, "t1-br-compiled-c0-v1");
        assert!(t1
            .bundle
            .as_ref()
            .unwrap()
            .field_revision
            .starts_with("compiled-field-v1/"));
    }

    #[test]
    fn t1_extracted_policy_replay_matches_full_cost() {
        let mut problem = (1..512)
            .filter_map(defender_problem)
            .next()
            .expect("lawful small endgame fixture");
        let actor = Actor::new("scheme-relational-actor-v1", vec![5, 8, 2]).unwrap();
        problem.field_revision = CompiledField::all(&actor).unwrap().revision().to_owned();
        let mut field = CompiledField::all(&actor).unwrap();
        let report = core::solve(
            &problem,
            &mut field,
            &mut Budget::new(20_000_000, Duration::from_secs(5)),
            &Config {
                max_horizon: 7,
                priority_plans: 1,
                scenario_upper: true,
                stop_when_certified: false,
            },
        )
        .unwrap();
        assert!(report.exact_vector);
        for bound in &report.actions {
            let mut replay_field = CompiledField::all(&actor).unwrap();
            let value = walt_response_ladder::policy::price(
                &problem,
                &bound.policy,
                &mut replay_field,
                &mut Budget::new(20_000_000, Duration::from_secs(5)),
            )
            .unwrap()
            .unwrap();
            assert_eq!(value, bound.lower, "action {}", bound.action);
        }
    }

    #[test]
    fn t1_zero_budget_refuses_without_cost_labels() {
        let (raw, req) = request_value((0..7).collect(), (0..7).collect(), Vec::new(), 0.0);
        let result = answer_with_mode(raw, req, &c0_mode());
        assert_eq!(result.status, "refused");
        assert!(result.action_counts.is_none());
        assert!(result.costs.is_none());
        assert_eq!(result.revision, "t1-br-compiled-c0-v1");
    }
    /// Deterministic test fixture search only; teaching never resamples requests
    /// by outcome. Every prefix is played legally from a complete physical deal.
    fn diagnostic_request(partial: usize, role: u8, completed_plays: usize,
                          require_no_voids: bool) -> (Value, Request) {
        for seed in 1..4096 {
            let mut rng = SplitMix64(seed);
            let mut deck: Vec<u8> = (0..28).collect();
            for i in (1..deck.len()).rev() {
                deck.swap(i, rng.below((i + 1) as u64) as usize);
            }
            let originals: [u32; 4] = std::array::from_fn(|seat| {
                deck[seat * 7..seat * 7 + 7].iter().fold(0, |h, &t| h | (1 << t))
            });
            let mut public = PublicState::opening(1);
            let mut history = Vec::new();
            for _ in 0..completed_plays + partial {
                let actor = public.actor();
                let hand = originals[actor as usize] & !public.played;
                let legal = tiles(public.legal(Decl::NoTrump, hand));
                let tile = legal[rng.below(legal.len() as u64) as usize];
                history.push([actor, tile]);
                public = public.after(Decl::NoTrump, tile);
            }
            let viewer = public.actor();
            let hand = originals[viewer as usize] & !public.played;
            if viewer % 2 != role || public.payoff(30, viewer).is_some()
                || public.legal(Decl::NoTrump, hand).count_ones() < 2
                || (require_no_voids && public.voids != [0; 4]) { continue; }
            let raw = json!({"decl":9,"bid":30,"bidder":1,"seat":viewer,
                "hand":tiles(hand),"original_hand":tiles(originals[viewer as usize]),
                "history":history,"seed":991827,"budget_ms":5000.0,"n":8});
            return (raw.clone(), serde_json::from_value(raw).unwrap());
        }
        panic!("no lawful diagnostic fixture: partial={partial}, role={role}");
    }

    fn request_public(req: &Request) -> PublicState {
        let mut public = PublicState::opening(req.bidder);
        for &[actor, tile] in &req.history {
            assert_eq!(actor, public.actor());
            public = public.after(Decl::NoTrump, tile);
        }
        public
    }

    fn direct_values(problem: &Problem) -> Vec<(u8, BigRational)> {
        let (boundary, size, _) = model::frame(&problem.root).unwrap();
        let shared = Arc::new(Shared::new(problem.decl, problem.bid, vec![8],
            boundary, size, Deadline::after(Duration::from_secs(10))));
        Solver::new(shared, Seat::from_index(problem.viewer as usize).unwrap(),
            problem.hand, problem.viewer % 2 == 1,
            problem.scenarios.iter().map(|s| s.hands).collect(),
            problem.scenarios.iter().map(|s| s.tape).collect(), solver::Field::Dice)
            .action_values(&problem.root.key(), &tiles(problem.root.legal(problem.decl, problem.hand)))
            .expect("complete exact diagnostic vector")
    }

    #[test]
    fn no_void_native_seed_matches_compatible_worlds_tapes_and_vectors_in_partial_frames() {
        for partial in 0..4 {
            for role in 0..2 {
                // One completed trick ensures partially depleted hands; seats
                // already acting in the partial trick have one less tile again.
                let (_, req) = diagnostic_request(partial, role, 4, true);
                let public = request_public(&req);
                assert_eq!(public.plays.len(), partial);
                assert_eq!(public.voids, [0; 4]);
                let hand = mask(&req.hand).unwrap();
                let seed = h0_seed(req.seat, hand, &public);
                let sample = |mode: &TeacherMode| sample_teacher_problem(mode,
                    Decl::NoTrump, 30, req.seat, hand, &public, 8, seed,
                    &mut Budget::new(WORK_LIMIT, Duration::from_secs(10)))
                    .unwrap().unwrap();
                let historical = sample(&TeacherMode::HistoricalH0);
                let compatible = sample(&TeacherMode::CompatibleH0Seed);
                assert!(historical.root.key() == compatible.root.key());
                assert_eq!(model::frame(&historical.root), model::frame(&compatible.root));
                assert_eq!(serde_json::to_value(&historical.scenarios).unwrap(),
                           serde_json::to_value(&compatible.scenarios).unwrap());
                assert_eq!(direct_values(&historical), direct_values(&compatible));
            }
        }
    }

    #[test]
    fn historical_h0_teacher_matches_native_choice_for_both_roles_and_partial_hands() {
        for partial in 0..4 {
            for role in 0..2 {
                let (raw, req) = diagnostic_request(partial, role, 20, false);
                let result = answer_with_schema(raw, req.clone(), &TeacherMode::HistoricalH0, compiled::SCHEMA_V2);
                assert_eq!(result.status, "complete", "{:?}", result.reason);
                assert_eq!(result.target, "historical-voidless-h0-v1");
                assert_eq!(result.revision, "h0-native-dice-exact-v1");
                assert_eq!(result.clause_actions.as_ref().unwrap().len(), 16);
                assert_eq!(result.mass, Some(8));
                let public = request_public(&req);
                let hand = mask(&req.hand).unwrap();
                let (boundary, size, _) = model::frame(&public).unwrap();
                assert_eq!(hand.count_ones(), 2);
                let sh = Arc::new(Shared::new(Decl::NoTrump, 30, vec![8], boundary, size,
                    Deadline::after(Duration::from_secs(10))));
                let seat = Seat::from_index(req.seat as usize).unwrap();
                let solver = Solver::new(sh, seat, hand, role == 1, vec![], vec![], solver::Field::Dice);
                assert_eq!(result.canonical_action,
                    solver.modeled_choice(0, &public.key(), seat, hand, public.legal(Decl::NoTrump, hand)));
                let bundle = result.bundle.unwrap();
                assert_eq!(bundle.seed, h0_seed(req.seat, hand, &public));
                assert_eq!(bundle.prior, "historical-voidless");
                let problem = Problem { decl: Decl::NoTrump, bid: 30, viewer: req.seat,
                    hand, root: public, scenarios: bundle.scenarios, field_revision: FIELD_REVISION.into() };
                let native = direct_values(&problem);
                assert_eq!(result.action_counts.unwrap(), native.iter().map(|(a,v)|
                    (*a, own_count(v, 8, role == 1).unwrap())).collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn diagnostic_modes_have_distinct_identity_fixed_n_and_seed_authority() {
        let (raw, req) = diagnostic_request(1, 0, 20, false);
        let first = answer_with_schema(raw.clone(), req.clone(), &TeacherMode::HistoricalH0, compiled::SCHEMA_V2);
        let mut changed = req.clone(); changed.seed ^= u64::MAX;
        let mut changed_raw = raw.clone(); changed_raw["seed"] = json!(changed.seed);
        let second = answer_with_schema(changed_raw, changed, &TeacherMode::HistoricalH0, compiled::SCHEMA_V2);
        assert_eq!(first.status, "complete"); assert_eq!(second.status, "complete");
        assert_eq!(first.action_counts, second.action_counts);
        assert_eq!(first.bundle.unwrap().identity, second.bundle.unwrap().identity);
        let compatible = answer_with_schema(raw.clone(), req.clone(), &TeacherMode::CompatibleH0Seed, compiled::SCHEMA_V2);
        assert_eq!(compatible.target, "compatible-root-h0-seed-v1");
        assert_eq!(compatible.revision, "t0-h0-seed-dice-exact-v1");
        let existing = answer_with_schema(raw.clone(), req.clone(), &TeacherMode::T0, compiled::SCHEMA_V2);
        assert_eq!(existing.status, "complete");
        assert_eq!(existing.target, TARGET);
        assert_eq!(existing.revision, "t0-dice-exact-v1");
        assert_eq!(existing.bundle.unwrap().seed, req.seed);
        for mode in [TeacherMode::HistoricalH0, TeacherMode::CompatibleH0Seed] {
            let mut bad = req.clone(); bad.n = 32;
            let value = answer_with_schema(raw.clone(), bad, &mode, compiled::SCHEMA_V2);
            assert_eq!(value.status, "error"); assert!(value.action_counts.is_none());
            let mut zero = req.clone(); zero.budget_ms = 0.0;
            let value = answer_with_schema(raw.clone(), zero, &mode, compiled::SCHEMA_V2);
            assert_eq!(value.status, "refused"); assert!(value.costs.is_none());
        }
    }

    #[test]
    fn historical_mode_retains_strict_observation_checks() {
        let (raw, req) = diagnostic_request(2, 1, 20, false);
        for mode in [TeacherMode::HistoricalH0, TeacherMode::CompatibleH0Seed] {
            let mut bad = req.clone(); bad.history[0][0] = (bad.history[0][0] + 1) % 4;
            assert_eq!(answer_with_schema(raw.clone(), bad, &mode, compiled::SCHEMA_V2).status, "error");
            let mut bad = req.clone(); bad.hand.pop();
            assert_eq!(answer_with_schema(raw.clone(), bad, &mode, compiled::SCHEMA_V2).status, "error");
            let mut bad = req.clone(); bad.seat = (bad.seat + 1) % 4;
            assert_eq!(answer_with_schema(raw.clone(), bad, &mode, compiled::SCHEMA_V2).status, "error");
            let hand = (1u32 << 7) - 1;
            let (lead, illegal) = (7..28).find_map(|lead| {
                let public = PublicState::opening(0).after(Decl::NoTrump, lead);
                let legal = public.legal(Decl::NoTrump, hand);
                (legal != hand).then(|| (lead, (hand & !legal).trailing_zeros() as u8))
            }).unwrap();
            let invalid = json!({"decl":9,"bid":30,"bidder":0,"seat":1,
                "hand":tiles(hand & !(1 << illegal)),"original_hand":tiles(hand),
                "history":[[0,lead],[1,illegal]],"seed":1,"budget_ms":1000.0,"n":8});
            let result = answer_with_schema(invalid.clone(), serde_json::from_value(invalid).unwrap(),
                &mode, compiled::SCHEMA_V2);
            assert_eq!(result.status, "error");
            assert!(result.reason.unwrap().contains("illegal follow"));
        }
    }

}
