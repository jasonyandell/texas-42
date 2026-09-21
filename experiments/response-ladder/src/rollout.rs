//! Lawful fixed-priority policies, evaluated independently on original scenarios.
//!
//! Rows are complete policies; columns retain sample identity and tape, even
//! when two scenarios have identical hands. No weighting or maximization is
//! performed here. A restricted row pool is a lower-witness family only.
use crate::mechanics::{Budget, DiceField, Field, FieldQuery, Problem, PublicState, tiles};
use std::time::Duration;
use walt::rules::{Context, Decl, Domino};
use walt::solver::{mix, SplitMix64};

pub type PayoffMatrix = Vec<Vec<u32>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LaneTrace {
    pub payoff: u32,
    /// Chronological continuation after the root, through contract settlement.
    pub plays: Vec<u8>,
}

pub(crate) fn validate(problem: &Problem, plans: &[Vec<u8>]) -> Result<(), String> {
    problem.validate()?;
    let root = &problem.root;
    if root.leader >= 4 || root.plays.len() > 3 || problem.hand.count_ones() > 7
        || root.banked_t1 > 42 || root.banked_t0 > 42
        || u16::from(root.banked_t1) + u16::from(root.banked_t0) > 42
    { return Err("invalid rollout root".into()); }
    let mut partial = 0;
    for &tile in &root.plays {
        if tile >= 28 || partial & (1 << tile) != 0 || root.played & (1 << tile) == 0 {
            return Err("invalid partial trick".into());
        }
        partial |= 1 << tile;
    }
    let legal = root.legal(problem.decl, problem.hand);
    for plan in plans {
        let mut mask = 0u32;
        for &tile in plan {
            if tile >= 28 || mask & (1 << tile) != 0 { return Err("repeated or invalid plan tile".into()); }
            mask |= 1 << tile;
        }
        if mask != problem.hand || plan.first().is_none_or(|&t| legal & (1 << t) == 0) {
            return Err("plan must permute the full own hand and start with a legal root".into());
        }
    }
    Ok(())
}

/// Complete permutation family, optionally limited to supplied legal roots.
/// Limiting roots retains completeness within each included root action.
pub fn priority_plans(problem: &Problem, roots: Option<&[u8]>) -> Result<Vec<Vec<u8>>, String> {
    validate(problem, &[])?;
    let legal = problem.root.legal(problem.decl, problem.hand);
    let all = tiles(legal);
    let roots = roots.unwrap_or(&all);
    let mut used = 0u32;
    let mut out = Vec::new();
    fn append(prefix: &mut Vec<u8>, remaining: u32, out: &mut Vec<Vec<u8>>) {
        if remaining == 0 { out.push(prefix.clone()); return; }
        for tile in tiles(remaining) {
            prefix.push(tile);
            append(prefix, remaining & !(1 << tile), out);
            prefix.pop();
        }
    }
    for &root in roots {
        if root >= 28 || legal & (1 << root) == 0 || used & (1 << root) != 0 {
            return Err("candidate roots must be distinct legal tiles".into());
        }
        used |= 1 << root;
        append(&mut vec![root], problem.hand & !(1 << root), &mut out);
    }
    validate(problem, &out)?;
    Ok(out)
}

/// Canonical scalar reference. Cancellation returns an error, never an
/// incomplete matrix that could be mistaken for zero-payoff lanes.
pub fn evaluate<F: Field>(problem: &Problem, plans: &[Vec<u8>], field: &mut F,
    budget: &mut Budget) -> Result<PayoffMatrix, String>
{
    Ok(evaluate_traces(problem, plans, field, budget)?.into_iter()
        .map(|row| row.into_iter().map(|lane| lane.payoff).collect()).collect())
}

pub fn evaluate_traces<F: Field>(problem: &Problem, plans: &[Vec<u8>], field: &mut F,
    budget: &mut Budget) -> Result<Vec<Vec<LaneTrace>>, String>
{
    validate(problem, plans)?;
    if field.revision() != problem.field_revision { return Err("field revision mismatch".into()); }
    let mut matrix = Vec::with_capacity(plans.len());
    for plan in plans {
        let mut row = Vec::with_capacity(problem.scenarios.len());
        for scenario in &problem.scenarios {
            let mut public = problem.root.clone();
            let mut plays = Vec::new();
            loop {
                if let Some(payoff) = public.payoff(problem.bid, problem.viewer) {
                    row.push(LaneTrace { payoff: payoff as u32, plays });
                    break;
                }
                budget.tick().ok_or("rollout budget exhausted")?;
                if plays.len() >= 28 { return Err("unsettled contract after 28 plays".into()); }
                let seat = public.actor();
                let hand = scenario.hands[seat as usize] & !public.played;
                let legal = public.legal(problem.decl, hand);
                if field.revision() != problem.field_revision { return Err("field revision changed during rollout".into()); }
                let tile = if seat == problem.viewer {
                    plan.iter().copied().find(|&t| legal & (1 << t) != 0)
                } else {
                    field.choose(FieldQuery { decl: problem.decl, bid: problem.bid, seat,
                        hand, public: &public, tape: scenario.tape }, budget)
                }.ok_or("rollout field cancelled or no legal tile")?;
                if field.revision() != problem.field_revision { return Err("field revision changed during rollout".into()); }
                if tile >= 28 || legal & (1 << tile) == 0 { return Err("field returned illegal tile".into()); }
                public = public.after(problem.decl, tile);
                plays.push(tile);
            }
        }
        matrix.push(row);
    }
    Ok(matrix)
}

pub fn evaluate_dice(problem: &Problem, plans: &[Vec<u8>]) -> Result<PayoffMatrix, String> {
    evaluate(problem, plans, &mut DiceField,
        &mut Budget::new(u64::MAX, Duration::from_secs(3600)))
}

// Finite tables are derived directly from the frozen rule algebra. They are
// shared by WGSL and the allocation-free scalar performance control; the
// PublicState implementation above remains the independent trace oracle.
pub(crate) fn rules_table() -> Vec<u32> {
    let mut out = Vec::with_capacity(2368);
    for decl in Decl::ALL {
        out.extend(Domino::ALL.map(|d| decl.led_context(d).index() as u32));
        out.extend(Context::ALL.map(|q| decl.effective_incidence(q).bits()));
        for q in Context::ALL {
            out.extend(Domino::ALL.map(|d| {
                let key = decl.trick_key(d, q);
                (key.tier as u32) * 16 + u32::from(key.rank.value())
            }));
        }
    }
    out.extend(Domino::ALL.map(Domino::count));
    assert_eq!(out.len(), 2368);
    out
}

#[derive(Clone, Copy)]
struct SmallState { played: u32, plays: u32, len: u32, leader: u32, t1: u32, t0: u32 }
impl From<&PublicState> for SmallState {
    fn from(p: &PublicState) -> Self {
        Self { played: p.played, plays: p.plays.iter().enumerate()
            .fold(0, |bits,(i,&t)| bits | u32::from(t) << (5*i)),
            len: p.plays.len() as u32, leader: p.leader.into(),
            t1: p.banked_t1.into(), t0: p.banked_t0.into() }
    }
}

/// Allocation-free per-lane Dice control using the same finite rule tables as
/// WGSL. Initialization, packet construction, and result allocation remain in
/// this call so benchmark comparisons do not omit CPU preparation.
pub fn evaluate_dice_fast(problem: &Problem, plans: &[Vec<u8>]) -> Result<PayoffMatrix, String> {
    validate(problem, plans)?;
    if problem.field_revision != "historical-dice-v1" { return Err("Dice field revision required".into()); }
    let table = rules_table();
    let base = Decl::ALL.iter().position(|&d| d == problem.decl).unwrap() * 260;
    let root = SmallState::from(&problem.root);
    let mut matrix = Vec::with_capacity(plans.len());
    for plan in plans {
        let mut row = Vec::with_capacity(problem.scenarios.len());
        for scenario in &problem.scenarios {
            let mut s = root;
            for step in 0..=28 {
                if s.t1 >= u32::from(problem.bid) || s.t0 > 42-u32::from(problem.bid) {
                    row.push(u32::from((s.t1 >= u32::from(problem.bid)) == (problem.viewer % 2 == 1)));
                    break;
                }
                if step == 28 { return Err("unsettled contract after 28 plays".into()); }
                let seat = (s.leader+s.len) & 3;
                let hand = scenario.hands[seat as usize] & !s.played;
                let follows = if s.len == 0 { 0 } else {
                    hand & table[base+28+table[base+(s.plays & 31) as usize] as usize]
                };
                let mut legal = if follows == 0 { hand } else { follows };
                if legal == 0 { return Err("no legal tile".into()); }
                let tile = if seat == u32::from(problem.viewer) {
                    u32::from(*plan.iter().find(|&&t| legal & (1 << t) != 0).unwrap())
                } else {
                    if legal.count_ones() > 1 {
                        let mut hash = mix(u64::from(s.played));
                        hash = mix(hash ^ (u64::from(s.leader) << 32));
                        for i in 0..s.len { hash = mix(hash ^ u64::from(0x100 | ((s.plays >> (5*i)) & 31))); }
                        let skip = SplitMix64(scenario.tape ^ hash).below(u64::from(legal.count_ones()));
                        for _ in 0..skip { legal &= legal-1; }
                    }
                    legal.trailing_zeros()
                };
                s.played |= 1 << tile;
                s.plays |= tile << (5*s.len);
                s.len += 1;
                if s.len == 4 {
                    let q = table[base+(s.plays & 31) as usize] as usize;
                    let mut best = 0;
                    let mut winner = 0;
                    let mut points = 1;
                    for i in 0..4 {
                        let t = ((s.plays >> (5*i)) & 31) as usize;
                        let strength = table[base+36+q*28+t];
                        if i == 0 || strength > best { best = strength; winner = i; }
                        points += table[2340+t];
                    }
                    s.leader = (s.leader+winner) & 3;
                    if s.leader & 1 == 1 { s.t1 += points; } else { s.t0 += points; }
                    s.plays = 0;
                    s.len = 0;
                }
            }
        }
        matrix.push(row);
    }
    Ok(matrix)
}
