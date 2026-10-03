//! Fixed-bundle SUM/MAX response with lawful lower and C+ upper witnesses.
//! Completed root transactions retain max lower/min upper for the SAME target.
use crate::compiled::Actor;
use crate::mechanics::{tiles, Budget, Field, Problem, PublicState};
use crate::policy::{check_revision, field_action, price, priorities, replay_from, Policy};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Future focal decisions AFTER the forced root action.
    pub max_horizon: usize,
    pub priority_plans: usize,
    /// Exact single-scenario world-and-tape-revealed search at the frontier.
    /// If false, unresolved frontier upper is simply its full arrival mass.
    pub scenario_upper: bool,
    pub stop_when_certified: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_horizon: 7,
            priority_plans: 2,
            scenario_upper: true,
            stop_when_certified: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBounds {
    pub action: u8,
    pub lower: u64,
    pub upper: u64,
    pub policy: Policy,
    /// False explicitly denotes an unpriced executable reserve and zero floor.
    pub priced: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub actions: Vec<ActionBounds>,
    pub chosen: u8,
    pub incumbent: Policy,
    pub incumbent_value: Option<u64>,
    pub regret_bound: u64,
    /// Certifies the least-tile optimal ROOT ACTION, not its continuation value.
    pub canonical_certified: bool,
    /// Certifies an optimal ROOT ACTION; policy optimality requires zero regret.
    pub optimal_certified: bool,
    pub exact_vector: bool,
    /// Highest horizon for which all root transactions completed. Zero may
    /// also mean no horizon transaction completed; inspect completed_rounds.
    pub completed_horizon: usize,
    pub completed_rounds: usize,
    pub interrupted: bool,
    /// Controller/replay ticks and field-query ticks consumed by this solve.
    /// A field may do nested exact work internally; those nodes are NOT counted
    /// here. Its wall deadline still aborts incomplete modeled-policy queries.
    pub work_used: u64,
    pub field_revision: String,
    pub mass: u64,
}

impl Report {
    fn refresh(&mut self) {
        let best = self
            .actions
            .iter()
            .max_by(|a, b| {
                a.lower
                    .cmp(&b.lower)
                    .then_with(|| a.priced.cmp(&b.priced))
                    .then_with(|| b.action.cmp(&a.action))
            })
            .unwrap();
        self.chosen = best.action;
        self.incumbent = best.policy.clone();
        self.incumbent_value = best.priced.then_some(best.lower);
        self.regret_bound = self.actions.iter().map(|a| a.upper).max().unwrap() - best.lower;
        self.optimal_certified = self
            .actions
            .iter()
            .all(|a| a.action == best.action || a.upper <= best.lower);
        self.canonical_certified = self.actions.iter().all(|a| {
            a.action == best.action
                || if a.action < best.action {
                    a.upper < best.lower
                } else {
                    a.upper <= best.lower
                }
        });
        self.exact_vector = self.actions.iter().all(|a| a.lower == a.upper);
    }
}

struct Node {
    lower: u64,
    upper: u64,
    decisions: BTreeMap<Vec<u8>, u8>,
}

fn merge(target: &mut BTreeMap<Vec<u8>, u8>, source: BTreeMap<Vec<u8>, u8>) -> Result<(), String> {
    for (history, action) in source {
        if target
            .insert(history, action)
            .is_some_and(|old| old != action)
        {
            return Err("conflicting choices at one complete public history".into());
        }
    }
    Ok(())
}

struct Search<'a, F: Field + ?Sized> {
    problem: &'a Problem,
    field: &'a mut F,
    budget: &'a mut Budget,
    scenario_upper: bool,
}

impl<F: Field + ?Sized> Search<'_, F> {
    fn mass(&self, alive: &[usize]) -> u64 {
        alive
            .iter()
            .map(|&id| self.problem.scenarios[id].weight)
            .sum()
    }

    /// A single scenario's focal player sees the entire deal AND tape. This is
    /// only an upper evaluator and its chosen actions never enter a policy.
    fn revealed(&mut self, state: &PublicState, id: usize) -> Result<Option<u64>, String> {
        if self.budget.tick().is_none() {
            return Ok(None);
        }
        if let Some(payoff) = state.payoff(self.problem.bid, self.problem.viewer) {
            return Ok(Some(payoff));
        }
        if state.actor() == self.problem.viewer {
            let legal = tiles(state.legal(self.problem.decl, self.problem.hand));
            if legal.is_empty() {
                return Err("nonterminal revealed state has no legal action".into());
            }
            for action in legal {
                let Some(value) = self.revealed(&state.after(self.problem.decl, action), id)?
                else {
                    return Ok(None);
                };
                if value == 1 {
                    return Ok(Some(1));
                }
            }
            Ok(Some(0))
        } else {
            let Some(action) = field_action(
                self.problem,
                state,
                &self.problem.scenarios[id],
                self.field,
                self.budget,
            )?
            else {
                return Ok(None);
            };
            self.revealed(&state.after(self.problem.decl, action), id)
        }
    }

    fn frontier(
        &mut self,
        state: &PublicState,
        alive: &[usize],
        tail: &Policy,
    ) -> Result<Option<Node>, String> {
        let mut node = Node {
            lower: 0,
            upper: self.mass(alive),
            decisions: BTreeMap::new(),
        };
        for &id in alive {
            let scenario = &self.problem.scenarios[id];
            let Some(replay) =
                replay_from(self.problem, state, scenario, tail, self.field, self.budget)?
            else {
                return Ok(None);
            };
            node.lower += scenario.weight * replay.payoff;
            merge(
                &mut node.decisions,
                replay
                    .focal_decisions
                    .into_iter()
                    .map(|d| (d.history, d.action))
                    .collect(),
            )?;
        }
        if self.scenario_upper && node.lower != node.upper {
            let mut upper = 0;
            for &id in alive {
                let Some(value) = self.revealed(state, id)? else {
                    return Ok(None);
                };
                upper += self.problem.scenarios[id].weight * value;
            }
            node.upper = upper;
        }
        Ok(Some(node))
    }

    fn node(
        &mut self,
        state: &PublicState,
        alive: &[usize],
        horizon: usize,
        tail: &Policy,
    ) -> Result<Option<Node>, String> {
        if self.budget.tick().is_none() {
            return Ok(None);
        }
        if let Some(payoff) = state.payoff(self.problem.bid, self.problem.viewer) {
            let value = self.mass(alive) * payoff;
            return Ok(Some(Node {
                lower: value,
                upper: value,
                decisions: BTreeMap::new(),
            }));
        }
        if state.actor() == self.problem.viewer {
            if horizon == 0 {
                return self.frontier(state, alive, tail);
            }
            let legal = tiles(state.legal(self.problem.decl, self.problem.hand));
            if legal.is_empty() {
                return Err("nonterminal focal state has no legal action".into());
            }
            let mut best: Option<Node> = None;
            let mut upper = 0;
            // MAX copies the same arrival mass into each complete legal action.
            for action in legal {
                let Some(mut child) = self.node(
                    &state.after(self.problem.decl, action),
                    alive,
                    horizon - 1,
                    tail,
                )?
                else {
                    return Ok(None);
                };
                upper = upper.max(child.upper);
                if best.as_ref().is_none_or(|old| child.lower > old.lower) {
                    child.decisions.insert(state.history.clone(), action);
                    best = Some(child);
                }
            }
            let mut best = best.unwrap();
            best.upper = upper;
            Ok(Some(best))
        } else {
            let mut buckets: BTreeMap<u8, Vec<usize>> = BTreeMap::new();
            // Classify EVERY original ID before any descendant can optimize.
            // Equal deals/tapes remain separate weighted samples in one bucket.
            for &id in alive {
                let Some(action) = field_action(
                    self.problem,
                    state,
                    &self.problem.scenarios[id],
                    self.field,
                    self.budget,
                )?
                else {
                    return Ok(None);
                };
                buckets.entry(action).or_default().push(id);
            }
            let mut sum = Node {
                lower: 0,
                upper: 0,
                decisions: BTreeMap::new(),
            };
            for (action, ids) in buckets {
                let Some(child) =
                    self.node(&state.after(self.problem.decl, action), &ids, horizon, tail)?
                else {
                    return Ok(None);
                };
                sum.lower += child.lower;
                sum.upper += child.upper;
                merge(&mut sum.decisions, child.decisions)?;
            }
            Ok(Some(sum))
        }
    }
}

/// Solve one immutable response target. None-valued operations are aborted
/// transactions: previous completed lower policies and upper facts survive.
/// Err refuses a malformed problem, unlawful field, or changed field revision.
/// A backend returns only complete policy prices, in the supplied row order.
/// None cancels the entire batch; earlier batches remain usable incumbents.
pub trait PolicyEvaluator {
    fn prices(&mut self, problem: &Problem, policies: &[Policy], field: &mut dyn Field,
        budget: &mut Budget) -> Result<Option<Vec<u64>>, String>;
}

pub struct CpuEvaluator;
impl PolicyEvaluator for CpuEvaluator {
    fn prices(&mut self, problem: &Problem, policies: &[Policy], field: &mut dyn Field,
        budget: &mut Budget) -> Result<Option<Vec<u64>>, String> {
        let mut values = Vec::with_capacity(policies.len());
        for policy in policies {
            let Some(value) = price(problem, policy, field, budget)? else { return Ok(None); };
            values.push(value);
        }
        Ok(Some(values))
    }
}

pub fn solve(problem: &Problem, field: &mut dyn Field, budget: &mut Budget,
    config: &Config) -> Result<Report, String> {
    solve_with_evaluator(problem, field, budget, config, None)
}

/// Batched pricing is used for complete rounds of lawful initial policies.
/// CPU SUM/MAX refinement then improves the same fixed-bundle target.
pub fn solve_with_evaluator(problem: &Problem, field: &mut dyn Field,
    budget: &mut Budget, config: &Config, evaluator: Option<&mut dyn PolicyEvaluator>,
) -> Result<Report, String> {
    solve_with_tail_evaluator(problem, field, budget, config, None, evaluator)
}

/// Solve with an optional total compiled focal tail. A present tail gets one
/// forced policy row per root action; the legacy priority pool remains intact
/// when the tail is absent.
pub fn solve_with_tail_evaluator(
    problem: &Problem,
    field: &mut dyn Field,
    budget: &mut Budget,
    config: &Config,
    compiled_tail: Option<&Actor>,
    mut evaluator: Option<&mut dyn PolicyEvaluator>,
) -> Result<Report, String> {
    problem.validate()?;
    check_revision(problem, field)?;
    if let Some(actor) = compiled_tail {
        actor.validate()?;
    }
    let started = budget.used;
    let legal = tiles(problem.root.legal(problem.decl, problem.hand));
    if legal.is_empty() {
        return Err("root has no legal actions".into());
    }
    let actions: Vec<_> = legal
        .iter()
        .map(|&action| {
            let policy = if let Some(actor) = compiled_tail {
                Policy::forced_with_tail(
                    problem,
                    action,
                    tiles(problem.hand & !(1 << action)),
                    actor.clone(),
                )?
            } else {
                Policy::forced(problem, action, tiles(problem.hand))
            };
            Ok(ActionBounds {
                action,
                lower: 0,
                upper: problem.mass(),
                priced: false,
                policy,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut report = Report {
        chosen: legal[0],
        incumbent: actions[0].policy.clone(),
        incumbent_value: None,
        regret_bound: problem.mass(),
        canonical_certified: false,
        optimal_certified: false,
        exact_vector: false,
        actions,
        completed_horizon: 0,
        completed_rounds: 0,
        interrupted: false,
        work_used: 0,
        field_revision: problem.field_revision.clone(),
        mass: problem.mass(),
    };
    'work: {
        // Round-robin roots: establish one priced witness per action before
        // spending a second plan on any action. Each row is still atomic.
        let pools: Vec<_> = report
            .actions
            .iter()
            .map(|bound| {
                if compiled_tail.is_some() {
                    vec![tiles(problem.hand & !(1 << bound.action))]
                } else {
                    priorities(problem.hand, bound.action, config.priority_plans)
                }
            })
            .collect();
        let rounds = pools.iter().map(Vec::len).max().unwrap_or(0);
        for rank in 0..rounds {
            let rows: Vec<_> = report.actions.iter().zip(&pools).enumerate()
                .filter_map(|(index, (bound, pool))| {
                    pool.get(rank).map(|priority| {
                        let policy = if let Some(actor) = compiled_tail {
                            Policy::forced_with_tail(
                                problem,
                                bound.action,
                                priority.clone(),
                                actor.clone(),
                            )
                        } else {
                            Ok(Policy::forced(problem, bound.action, priority.clone()))
                        };
                        policy.map(|policy| (index, policy))
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            if let Some(backend) = evaluator.as_deref_mut() {
                let policies: Vec<_> = rows.iter().map(|(_, policy)| policy.clone()).collect();
                let Some(values) = backend.prices(problem, &policies, field, budget)? else {
                    report.interrupted = true;
                    break 'work;
                };
                check_revision(problem, field)?;
                if values.len() != rows.len() || values.iter().any(|&v| v > problem.mass()) {
                    return Err("invalid completed policy batch".into());
                }
                for ((index, policy), value) in rows.into_iter().zip(values) {
                    let bound = &mut report.actions[index];
                    if !bound.priced || value > bound.lower {
                        bound.lower = value; bound.policy = policy; bound.priced = true;
                    }
                }
            } else {
                // Scalar cancellation preserves each previous complete row.
                for (index, policy) in rows {
                    let Some(value) = price(problem, &policy, field, budget)? else {
                        report.interrupted = true;
                        break 'work;
                    };
                    let bound = &mut report.actions[index];
                    if !bound.priced || value > bound.lower {
                        bound.lower = value; bound.policy = policy; bound.priced = true;
                    }
                }
            }
            report.refresh();
            if config.stop_when_certified && report.canonical_certified {
                break 'work;
            }
        }
        report.refresh();
        if config.stop_when_certified && report.canonical_certified {
            break 'work;
        }
        let alive: Vec<_> = (0..problem.scenarios.len()).collect();
        let max_horizon = config
            .max_horizon
            .min(problem.hand.count_ones().saturating_sub(1) as usize);
        for horizon in 0..=max_horizon {
            for bound in &mut report.actions {
                if bound.priced && bound.lower == bound.upper {
                    continue;
                }
                let mut search = Search {
                    problem,
                    field,
                    budget,
                    scenario_upper: config.scenario_upper,
                };
                let Some(mut node) = search.node(
                    &problem.root.after(problem.decl, bound.action),
                    &alive,
                    horizon,
                    &bound.policy,
                )?
                else {
                    report.interrupted = true;
                    break 'work;
                };
                check_revision(problem, field)?;
                node.decisions
                    .insert(problem.root.history.clone(), bound.action);
                if node.lower > node.upper || node.lower > bound.upper || node.upper < bound.lower {
                    return Err("inconsistent bound witnesses for a frozen response target".into());
                }
                if !bound.priced || node.lower > bound.lower {
                    bound.lower = node.lower;
                    bound.policy = bound.policy.materialized(node.decisions);
                    bound.priced = true;
                }
                bound.upper = bound.upper.min(node.upper);
            }
            report.completed_horizon = horizon;
            report.completed_rounds += 1;
            report.refresh();
            if report.exact_vector || config.stop_when_certified && report.canonical_certified {
                break;
            }
        }
    }
    check_revision(problem, field)?;
    report.work_used = budget.used - started;
    report.refresh();
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_exactly_priced_zero_witness_precedes_an_unpriced_zero_reserve() {
        // Lower-endpoint ties do not establish a true root-value tie.
        let reserve = Policy::priority(0, vec![1, 2]);
        let priced = Policy::priority(0, vec![2, 1]);
        let mut report = Report {
            actions: vec![
                ActionBounds {
                    action: 1,
                    lower: 0,
                    upper: 8,
                    policy: reserve.clone(),
                    priced: false,
                },
                ActionBounds {
                    action: 2,
                    lower: 0,
                    upper: 8,
                    policy: priced.clone(),
                    priced: true,
                },
            ],
            chosen: 1,
            incumbent: reserve,
            incumbent_value: None,
            regret_bound: 8,
            canonical_certified: false,
            optimal_certified: false,
            exact_vector: false,
            completed_horizon: 0,
            completed_rounds: 0,
            interrupted: true,
            work_used: 0,
            field_revision: "test-field".into(),
            mass: 8,
        };
        report.refresh();
        assert_eq!(report.chosen, 2);
        assert_eq!(report.incumbent, priced);
        assert_eq!(report.incumbent_value, Some(0));
        assert!(!report.canonical_certified);
        // Once both values are known, the genuine tie uses the canonical tile.
        report.actions[0].priced = true;
        for action in &mut report.actions {
            action.upper = 0;
        }
        report.refresh();
        assert_eq!(report.chosen, 1);
        assert!(report.canonical_certified && report.exact_vector);
        assert_eq!(report.regret_bound, 0);
    }
}
