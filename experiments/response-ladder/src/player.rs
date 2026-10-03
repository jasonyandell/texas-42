//! Playable native/portable controller. Sampling only sees the acting hand.
use crate::{core::{self, Config, Report}, mechanics::*, model::{self, FrozenModel}};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use walt::{clock::Instant, rules::Seat, solver::{self, Shared, Solver}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub field: String,
    pub outer: usize,
    pub n0: usize,
    pub n1: usize,
    pub plans: usize,
    pub horizon: usize,
    pub work: u64,
}
impl Default for PlayerConfig {
    fn default() -> Self { Self { field: "partner".into(), outer: 40, n0: 8,
        n1: 2, plans: 8, horizon: 7, work: 2_000_000 } }
}

#[derive(Serialize)]
pub struct Decision {
    pub tile: u8,
    pub elapsed_ms: f64,
    pub mode: String,
    pub fallback: bool,
    pub reason: String,
    pub report: Option<Report>,
    pub exact_counts: Option<Vec<(u8, u64)>>,
    pub reserve: Option<Box<Decision>>,
}

fn levels(config: &PlayerConfig, viewer: u8) -> Result<[usize; 4], String> {
    let mut levels = [0; 4];
    match config.field.as_str() {
        "dice" | "l0" => {},
        "partner" => { levels[(viewer as usize + 2) % 4] = 1; },
        "all-l1" => { levels = [1; 4]; levels[viewer as usize] = 0; },
        _ => return Err("field must be dice, l0, partner or all-l1".into()),
    }
    Ok(levels)
}

/// This interface cannot receive actual opponent hands. `original_hand` is
/// used only to preserve the historical outer seed, and must belong to viewer.
#[allow(clippy::too_many_arguments)]
pub fn decide(decl: walt::rules::Decl, bid: u8, viewer: u8, hand: u32,
    original_hand: u32, public: &PublicState, seed: u64, duration: Duration,
    exact: bool, config: &PlayerConfig) -> Result<Decision, String> {
    decide_with_evaluator(decl, bid, viewer, hand, original_hand, public, seed,
        duration, exact, config, None)
}

#[allow(clippy::too_many_arguments)]
pub fn decide_with_evaluator(decl: walt::rules::Decl, bid: u8, viewer: u8, hand: u32,
    original_hand: u32, public: &PublicState, seed: u64, duration: Duration,
    exact: bool, config: &PlayerConfig, evaluator: Option<&mut dyn core::PolicyEvaluator>,
) -> Result<Decision, String> {
    let start = Instant::now();
    let mut budget = Budget::new(config.work, duration);
    if config.outer == 0 || config.n0 == 0 || config.n1 == 0 {
        return Err("sample counts must be positive".into());
    }
    if viewer >= 4 || public.actor() != viewer || !(1..=42).contains(&bid) {
        return Err("invalid root actor/contract".into());
    }
    let field_levels = levels(config, viewer)?;
    if original_hand & !solver::FULL_MASK != 0 || original_hand.count_ones() != 7
        || original_hand & !public.played != hand {
        return Err("invalid original viewer hand".into());
    }
    let (_, _, sizes) = model::frame(public)?;
    if hand.count_ones() as usize != sizes[viewer as usize] || hand & public.played != 0 {
        return Err("hand disagrees with public frame".into());
    }
    let legal = public.legal(decl, hand);
    if legal == 0 { return Err("no legal play".into()); }
    let reserve = legal.trailing_zeros() as u8;
    let base = |reason: &str, fallback: bool| Decision { tile: reserve,
        elapsed_ms: start.elapsed().as_secs_f64()*1000., mode: if exact { "exact-v34" } else { "anytime" }.into(),
        fallback, reason: reason.into(), report: None, exact_counts: None, reserve: None };
    if legal.count_ones() == 1 { return Ok(base("forced", false)); }
    if public.payoff(bid, viewer).is_some() { return Ok(base("contract-settled", false)); }
    let mut reserve_evaluation = None;
    let mut reserve_tile = reserve;
    if evaluator.is_some() && !exact {
        let reserve_config = PlayerConfig { field:"l0".into(),outer:8,n0:2,n1:2,
            ..config.clone() };
        let value = decide(decl,bid,viewer,hand,original_hand,public,seed,
            duration.div_f64(4.0).min(Duration::from_millis(50)),true,&reserve_config)?;
        reserve_tile = value.tile;
        reserve_evaluation = Some(Box::new(value));
    }
    let identity = if config.field == "dice" { "historical-dice-v1".into() }
        else { model::revision(field_levels, config.n0, config.n1) };
    let stream = seed ^ solver::mix(original_hand as u64) ^ solver::record_hash(&public.key());
    let problem = match model::sample_problem(decl, bid, viewer, hand, public,
        config.outer, stream, identity, &mut budget)? {
        Some(p) => p, None => {
            let mut answer=base("sample-interrupted",true);answer.tile=reserve_tile;
            answer.reserve=reserve_evaluation;return Ok(answer);
        },
    };
    if exact {
        let (boundary, size, _) = model::frame(public)?;
        let sh = Arc::new(Shared::new(decl, bid, vec![config.n0, config.n1],
            boundary, size, budget.deadline));
        let field = if config.field == "dice" { solver::Field::Dice }
            else { solver::Field::SeatLevels(field_levels) };
        let solver = Solver::new(sh, Seat::from_index(viewer as usize).unwrap(), hand,
            viewer % 2 == 1, problem.scenarios.iter().map(|s|s.hands).collect(),
            problem.scenarios.iter().map(|s|s.tape).collect(), field);
        let Some(values) = solver.action_values(&public.key(), &tiles(legal)) else {
            return Ok(base("exact-comparison-interrupted", true));
        };
        use num_traits::ToPrimitive;
        let counts = values.iter().map(|(a,v)| {
            let t1 = (v * num_rational::BigRational::from_integer((config.outer as u64).into()))
                .to_integer().to_u64().expect("bounded success count");
            (*a, if viewer % 2 == 1 { t1 } else { config.outer as u64 - t1 })
        }).collect();
        let chosen = solver::best_of(&values, viewer % 2 == 1);
        return Ok(Decision { tile: chosen, exact_counts: Some(counts),
            elapsed_ms: start.elapsed().as_secs_f64()*1000., ..base("exact", false) });
    }
    let cfg = Config { max_horizon: config.horizon, priority_plans: config.plans,
        scenario_upper: true, stop_when_certified: true };
    let mut report = if config.field == "dice" {
        core::solve_with_evaluator(&problem, &mut DiceField, &mut budget, &cfg, evaluator)?
    } else {
        let mut field = FrozenModel::new(&problem, field_levels, config.n0, config.n1, budget.deadline)?;
        core::solve_with_evaluator(&problem, &mut field, &mut budget, &cfg, evaluator)?
    };
    let fallback = report.incumbent_value.is_none();
    if fallback {
        // Reserve belongs to a different sampled target: retain only a trivial
        // floor on this bundle, never transplant its L1 value into these bounds.
        let bound=report.actions.iter().find(|a|a.action==reserve_tile).unwrap();
        report.chosen=reserve_tile;report.incumbent=bound.policy.clone();
    }
    let mut answer = base(if fallback {
        if reserve_evaluation.as_ref().is_some_and(|r| !r.fallback) { "completed-l1-reserve" }
        else { "unpriced-lawful-reserve" }
    } else { "bounded-response" }, fallback);
    answer.tile = report.chosen;
    answer.report = Some(report);
    answer.reserve = reserve_evaluation;
    answer.elapsed_ms = start.elapsed().as_secs_f64()*1000.;
    Ok(answer)
}
