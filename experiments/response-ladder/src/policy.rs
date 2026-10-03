//! Total lawful policies and independent, transactional witness pricing.
use crate::compiled::Actor;
use crate::mechanics::{tiles, Budget, Field, FieldQuery, Problem, PublicState, Scenario};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use walt::rules::Decl;

/// The fixed root, contract and own hand belong to the response problem.
/// Only complete chronological public history indexes decisions. No scenario,
/// tape, sampled component, or hidden hand appears in this observation key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub history: Vec<u8>,
    pub action: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    pub viewer: u8,
    /// First remaining legal entry wins; least legal tile is the final default.
    /// This frozen default applies even on histories absent from the bundle.
    pub priority: Vec<u8>,
    /// Sorted lexicographically by the COMPLETE history, not a reduced record.
    pub decisions: Vec<Decision>,
    /// Optional total focal tail for histories without an explicit decision.
    /// Missing in legacy JSON means the legacy priority policy is unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compiled_tail: Option<Actor>,
}

impl Policy {
    pub fn priority(viewer: u8, priority: Vec<u8>) -> Self {
        Self {
            viewer,
            priority,
            decisions: Vec::new(),
            compiled_tail: None,
        }
    }
    pub fn forced(problem: &Problem, action: u8, priority: Vec<u8>) -> Self {
        Self {
            viewer: problem.viewer,
            priority,
            decisions: vec![Decision {
                history: problem.root.history.clone(),
                action,
            }],
            compiled_tail: None,
        }
    }
    pub fn forced_with_tail(
        problem: &Problem,
        action: u8,
        priority: Vec<u8>,
        compiled_tail: Actor,
    ) -> Result<Self, String> {
        compiled_tail.validate()?;
        Ok(Self {
            viewer: problem.viewer,
            priority,
            decisions: vec![Decision {
                history: problem.root.history.clone(),
                action,
            }],
            compiled_tail: Some(compiled_tail),
        })
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.viewer >= 4 {
            return Err("policy viewer must be one of seats 0..=3".into());
        }
        if let Some(actor) = &self.compiled_tail {
            actor.validate()?;
        }
        Ok(())
    }
    pub fn choose(&self, decl: Decl, public: &PublicState, hand: u32) -> Option<u8> {
        if public.actor() != self.viewer {
            return None;
        }
        let legal = public.legal(decl, hand);
        if legal == 0 {
            return None;
        }
        let found = self
            .decisions
            .binary_search_by(|d| d.history.cmp(&public.history));
        if let Ok(index) = found {
            let action = self.decisions[index].action;
            if action < 28 && legal & (1 << action) != 0 {
                return Some(action);
            }
        }
        if let Some(actor) = &self.compiled_tail {
            if actor.validate().is_err() {
                return None;
            }
            return actor.action_unchecked(decl, self.viewer, hand & !public.played, public);
        }
        self.priority
            .iter()
            .copied()
            .find(|&tile| tile < 28 && legal & (1 << tile) != 0)
            .or_else(|| Some(legal.trailing_zeros() as u8))
    }
    pub(crate) fn materialized(&self, decisions: BTreeMap<Vec<u8>, u8>) -> Self {
        Self {
            viewer: self.viewer,
            priority: self.priority.clone(),
            decisions: decisions
                .into_iter()
                .map(|(history, action)| Decision { history, action })
                .collect(),
            compiled_tail: self.compiled_tail.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Replay {
    pub payoff: u64,
    pub terminal: PublicState,
    pub focal_decisions: Vec<Decision>,
}

pub(crate) fn check_revision<F: Field + ?Sized>(
    problem: &Problem,
    field: &F,
) -> Result<(), String> {
    if field.revision() == problem.field_revision {
        Ok(())
    } else {
        Err(format!(
            "field revision changed: expected {}, got {}",
            problem.field_revision,
            field.revision()
        ))
    }
}

pub(crate) fn field_action<F: Field + ?Sized>(
    problem: &Problem,
    public: &PublicState,
    scenario: &Scenario,
    field: &mut F,
    budget: &mut Budget,
) -> Result<Option<u8>, String> {
    check_revision(problem, field)?;
    if budget.exhausted() {
        return Ok(None);
    }
    let seat = public.actor();
    let hand = scenario.hands[seat as usize] & !public.played;
    let action = field.choose(
        FieldQuery {
            decl: problem.decl,
            bid: problem.bid,
            seat,
            hand,
            public,
            tape: scenario.tape,
        },
        budget,
    );
    check_revision(problem, field)?;
    if let Some(action) = action {
        if action >= 28 || public.legal(problem.decl, hand) & (1 << action) == 0 {
            return Err("field returned an illegal action".into());
        }
    }
    Ok(action)
}

pub(crate) fn replay_from<F: Field + ?Sized>(
    problem: &Problem,
    public: &PublicState,
    scenario: &Scenario,
    policy: &Policy,
    field: &mut F,
    budget: &mut Budget,
) -> Result<Option<Replay>, String> {
    check_revision(problem, field)?;
    policy.validate()?;
    if policy.viewer != problem.viewer {
        return Err("policy viewer differs from response problem".into());
    }
    let mut state = public.clone();
    let mut focal_decisions = Vec::new();
    loop {
        if budget.tick().is_none() {
            return Ok(None);
        }
        if let Some(payoff) = state.payoff(problem.bid, problem.viewer) {
            return Ok(Some(Replay {
                payoff,
                terminal: state,
                focal_decisions,
            }));
        }
        let action = if state.actor() == problem.viewer {
            let Some(action) = policy.choose(problem.decl, &state, problem.hand) else {
                return Err("nonterminal focal state has no legal action".into());
            };
            focal_decisions.push(Decision {
                history: state.history.clone(),
                action,
            });
            action
        } else {
            let Some(action) = field_action(problem, &state, scenario, field, budget)? else {
                return Ok(None);
            };
            action
        };
        state = state.after(problem.decl, action);
    }
}

/// Replay a policy on an arbitrary valid scenario, including an off-sample
/// scenario with the same fixed root and focal hand. None means interruption;
/// no partial payoff is exposed as a completed witness.
pub fn replay_scenario<F: Field + ?Sized>(
    problem: &Problem,
    scenario: &Scenario,
    policy: &Policy,
    field: &mut F,
    budget: &mut Budget,
) -> Result<Option<Replay>, String> {
    let mut single = problem.clone();
    single.scenarios = vec![scenario.clone()];
    single.validate()?;
    replay_from(problem, &problem.root, scenario, policy, field, budget)
}

/// Independent straight rollout price over ORIGINAL weighted scenario IDs.
/// Returns an exact integer only after the entire bundle has completed.
pub fn price<F: Field + ?Sized>(
    problem: &Problem,
    policy: &Policy,
    field: &mut F,
    budget: &mut Budget,
) -> Result<Option<u64>, String> {
    problem.validate()?;
    check_revision(problem, field)?;
    let mut value = 0u64;
    for scenario in &problem.scenarios {
        let Some(replay) = replay_from(problem, &problem.root, scenario, policy, field, budget)?
        else {
            return Ok(None);
        };
        value += scenario.weight * replay.payoff;
    }
    check_revision(problem, field)?;
    Ok(Some(value))
}

/// A deterministic small pool. Every row is a total lawful priority policy;
/// restricting this pool never licenses a column-maximum upper bound.
pub(crate) fn priorities(hand: u32, action: u8, limit: usize) -> Vec<Vec<u8>> {
    let mut order = tiles(hand & !(1 << action));
    let mut out = Vec::new();
    while out.len() < limit {
        out.push(order.clone());
        let Some(i) = (0..order.len().saturating_sub(1))
            .rev()
            .find(|&i| order[i] < order[i + 1])
        else {
            break;
        };
        let j = (i + 1..order.len())
            .rev()
            .find(|&j| order[j] > order[i])
            .unwrap();
        order.swap(i, j);
        order[i + 1..].reverse();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiled::SCHEMA;
    use crate::mechanics::PublicState;

    #[test]
    fn materialized_policy_preserves_compiled_tail() {
        let tail = Actor::new(SCHEMA, vec![5, 8, 2]).unwrap();
        let policy = Policy {
            viewer: 0,
            priority: vec![1, 2],
            decisions: vec![],
            compiled_tail: Some(tail.clone()),
        };
        let mut decisions = BTreeMap::new();
        decisions.insert(PublicState::opening(0).history, 1);
        let materialized = policy.materialized(decisions);
        assert_eq!(materialized.compiled_tail, Some(tail));
        assert_eq!(materialized.decisions.len(), 1);
    }
}
