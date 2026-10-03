//! Experimental compatible-root response to frozen compiled lower rungs.
use crate::{compiled::Actor, compiled_family::Family, compiled_field::CompiledField, core, mechanics::*, model};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use walt::{clock::Instant, solver};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub outer: usize,
    pub plans: usize,
    pub horizon: usize,
    pub work: u64,
    #[serde(default)]
    pub compiled_tail: bool,
}
impl Default for Config {
    fn default() -> Self { Self { outer: 40, plans: 1, horizon: 7, work: 2_000_000, compiled_tail: false } }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub decl: usize, pub bid: u8, pub bidder: u8, pub seat: u8,
    pub hand: Vec<u8>, pub original_hand: Vec<u8>, pub history: Vec<[u8; 2]>,
    pub seed: u64, pub budget_ms: f64,
    #[serde(default)] pub config: Config,
}

#[derive(Serialize)]
pub struct Decision {
    pub tile: u8,
    pub elapsed_ms: f64,
    pub fallback: bool,
    pub reason: String,
    pub target: &'static str,
    pub report: Option<core::Report>,
}

fn mask(tiles: &[u8]) -> Result<u32, String> {
    let mut hand = 0u32;
    for &tile in tiles {
        if tile >= 28 || hand & (1 << tile) != 0 { return Err("invalid or duplicate hand tile".into()); }
        hand |= 1 << tile;
    }
    Ok(hand)
}

/// Normalize arena seats and verify every visible own-hand legality constraint.
pub fn normalize(req: &Request) -> Result<(walt::rules::Decl, u8, u32, u32, PublicState), String> {
    if ![0,1,2,3,4,5,6,7,9].contains(&req.decl) || req.bid != 30 || req.bidder >= 4 || req.seat >= 4 {
        return Err("compiled campaign requires a standard declaration, bid 30, and seats 0..3".into());
    }
    if !req.budget_ms.is_finite() || !(0.0..=86_400_000.0).contains(&req.budget_ms)
        || req.config.outer == 0 || req.config.outer > 4096 || req.config.plans > 5040 || req.config.horizon > 7 {
        return Err("invalid compiled player budget or configuration".into());
    }
    let decl = solver::decl_of(req.decl);
    let original = mask(&req.original_hand)?;
    let hand = mask(&req.hand)?;
    if original.count_ones() != 7 { return Err("original own hand must have seven tiles".into()); }
    let rotation = u8::from(req.bidder % 2 == 0);
    let viewer = (req.seat + rotation) % 4;
    let mut public = PublicState::opening((req.bidder + rotation) % 4);
    for &[actor, tile] in &req.history {
        if actor >= 4 || tile >= 28 || (actor + rotation) % 4 != public.actor()
            || public.played & (1 << tile) != 0 { return Err("invalid public history".into()); }
        let owns = original & (1 << tile) != 0;
        if owns != (public.actor() == viewer) { return Err("history contradicts own hand".into()); }
        if owns && public.legal(decl, original & !public.played) & (1 << tile) == 0 {
            return Err("own hand reneged in public history".into());
        }
        public = public.after(decl, tile);
    }
    let (_, _, sizes) = model::frame(&public)?;
    if public.actor() != viewer || original & !public.played != hand || hand == 0
        || hand.count_ones() as usize != sizes[viewer as usize] {
        return Err("current actor/hand disagrees with history".into());
    }
    Ok((decl, viewer, hand, original, public))
}

pub fn decide(req: &Request, c0: &Actor, c1: &Actor,
    evaluator: Option<&mut dyn core::PolicyEvaluator>) -> Result<Decision, String> {
    decide_family(req, &Family::Single(c0.clone()), &Family::Single(c1.clone()), evaluator)
}

/// Decide with role-aware C0/C1 families after request-seat normalization.
pub fn decide_family(req: &Request, c0: &Family, c1: &Family,
    evaluator: Option<&mut dyn core::PolicyEvaluator>) -> Result<Decision, String> {
    let start = Instant::now();
    let (decl, viewer, hand, original, public) = normalize(req)?;
    let allowance = Duration::from_secs_f64(req.budget_ms / 1000.0).saturating_sub(start.elapsed());
    let mut budget = Budget::new(req.config.work, allowance);
    c0.validate()?;
    c1.validate()?;
    let legal = public.legal(decl, hand);
    let reserve_actor = c1.actor(viewer)?;
    let reserve = reserve_actor.choose(decl, viewer, hand, &public)?.action;
    let base = |tile, fallback, reason: &str, report| Decision {
        tile, fallback, reason: reason.into(), report,
        target: "compatible-root-reset-v1/compiled-partner",
        elapsed_ms: start.elapsed().as_secs_f64() * 1000.0,
    };
    if legal.count_ones() == 1 { return Ok(base(reserve, false, "forced", None)); }
    if public.payoff(req.bid, viewer).is_some() { return Ok(base(reserve, false, "contract-settled", None)); }
    let mut field = CompiledField::partner_family(viewer, c0, c1)?;
    let stream = req.seed ^ solver::mix(u64::from(original)) ^ solver::record_hash(&public.key());
    let Some(problem) = model::sample_problem(decl, req.bid, viewer, hand, &public,
        req.config.outer, stream, field.revision().into(), &mut budget)? else {
            return Ok(base(reserve, true, "sample-interrupted-compiled-reserve", None));
        };
    let cfg = core::Config { max_horizon: req.config.horizon, priority_plans: req.config.plans,
        scenario_upper: true, stop_when_certified: true };
    let compiled_tail = req.config.compiled_tail.then_some(reserve_actor);
    let mut report = core::solve_with_tail_evaluator(
        &problem,
        &mut field,
        &mut budget,
        &cfg,
        compiled_tail,
        evaluator,
    )?;
    let fallback = report.incumbent_value.is_none();
    if fallback {
        let bound = report.actions.iter().find(|bound| bound.action == reserve).unwrap();
        report.chosen = reserve;
        report.incumbent = bound.policy.clone();
    }
    Ok(base(report.chosen, fallback,
        if fallback { "unpriced-compiled-reserve" } else { "bounded-compiled-response" }, Some(report)))
}
