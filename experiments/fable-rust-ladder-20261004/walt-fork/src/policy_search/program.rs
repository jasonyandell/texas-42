//! Interchange between sampled table policies and executable Scheme policies.

use std::collections::BTreeMap;

use crate::kernel::World;
use crate::rules::{legal_plays, Domino, Seat};
use crate::scheme::{
    step_frame, Budget, CompiledPolicy, DecisionProvenance, ExactRule, Fallback, ObservedPlay,
    PlayClass, PolicyControllerState, PolicyInput, PolicyKey, PolicyProgram,
};
use crate::solver::adaptive::SlicePolicy;

use super::{Fixture, State, TablePolicy};

const POLICY_WORK: u64 = 100_000_000;

fn observation(frame: &crate::scheme::Frame, actor: Seat, tile: Domino) -> ObservedPlay {
    let class = match frame.led_context() {
        None => PlayClass::Lead,
        Some(q) if frame.kernel().decl().follows(tile, q) => PlayClass::Follow(q),
        Some(q) => PlayClass::Slough(q),
    };
    ObservedPlay { actor, tile, class }
}

fn root_input<'a>(fixture: &'a Fixture) -> Result<PolicyInput<'a>, String> {
    PolicyInput::new(
        &fixture.exercise.frame,
        &fixture.history,
        fixture.exercise.position.banked,
        fixture.exercise.position.bid as u8,
        fixture.exercise.position.declaring_team,
    )
    .map_err(|e| e.to_string())
}

/// Export every focal table entry as an exact, viewer-information key.
/// Histories are replayed through the typed Scheme frame transform; no concrete
/// hidden assignment is consulted during export.
pub fn export(fixture: &Fixture, table: &TablePolicy, name: &str) -> Result<PolicyProgram, String> {
    let mut by_key = BTreeMap::new();
    for (suffix, action) in &table.choices {
        let mut frame = fixture.exercise.frame.clone();
        let mut state = State::from_root(&fixture.exercise.position);
        let mut history = fixture.history.clone();
        let mut budget = Budget::new(POLICY_WORK);
        for index in suffix {
            let tile = Domino::from_index(*index as usize)
                .ok_or_else(|| format!("table history contains invalid domino index {index}"))?;
            let actor = state.actor();
            frame = step_frame(&frame, observation(&frame, actor, tile), &mut budget)
                .map_err(|e| format!("table history is not a public support path: {e}"))?;
            history.push((actor, tile));
            state = state.step(fixture.exercise.position.decl, tile);
        }
        if state.actor() != fixture.exercise.root.kernel().viewer() {
            return Err("table entry is not a focal decision history".into());
        }
        let input = PolicyInput::new(
            &frame,
            &history,
            state.banked,
            fixture.exercise.position.bid as u8,
            fixture.exercise.position.declaring_team,
        )
        .map_err(|e| e.to_string())?;
        let key = PolicyKey::from_input(&input);
        if let Some(prior) = by_key.insert(key, *action) {
            if prior != *action {
                return Err(
                    "two table histories map to one information key with different actions".into(),
                );
            }
        }
    }
    let program = PolicyProgram {
        name: name.to_owned(),
        initial_mode: "fresh".into(),
        bindings: vec![],
        exact_rules: by_key
            .into_iter()
            .map(|(key, action)| ExactRule { key, action })
            .collect(),
        rules: vec![],
        fallback: Fallback::LowestLegal,
    };
    let reparsed: PolicyProgram = program
        .to_string()
        .parse()
        .map_err(|e| format!("exported policy failed its text roundtrip: {e}"))?;
    if reparsed != program {
        return Err("exported policy changed during text roundtrip".into());
    }
    Ok(program)
}

/// Layer a frozen exact-table export over a stateless relational program.
///
/// The exact source must contain only exact rules. The relational source must
/// contain no exact rules or rigid bindings, and every rule must remain in its
/// initial mode. These restrictions make the composition an exact-key layer
/// followed by one shared relational fallback without inventing controller
/// initialization or transition semantics.
pub fn combine_exact_table_with_relational(
    exact: &PolicyProgram,
    relational: &PolicyProgram,
    name: &str,
) -> Result<PolicyProgram, String> {
    if !exact.bindings.is_empty() || !exact.rules.is_empty() {
        return Err("exact-table source must contain only exact rules".into());
    }
    if !relational.exact_rules.is_empty() {
        return Err("relational source must not contain exact rules".into());
    }
    if !relational.bindings.is_empty() {
        return Err("stateless relational source must not contain rigid bindings".into());
    }
    if relational.rules.iter().any(|rule| {
        rule.in_mode != relational.initial_mode || rule.next_mode != relational.initial_mode
    }) {
        return Err("stateless relational rules must stay in the initial mode".into());
    }
    Ok(PolicyProgram {
        name: name.to_owned(),
        initial_mode: relational.initial_mode.clone(),
        bindings: vec![],
        exact_rules: exact.exact_rules.clone(),
        rules: relational.rules.clone(),
        fallback: relational.fallback,
    })
}

/// One focal decision in an independent full-program replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramDecisionTrace {
    /// Number of earlier focal decisions in this replay; the first is zero.
    pub focal_decision_depth: usize,
    pub action: Domino,
    pub provenance: DecisionProvenance,
    pub contract_resolved: Option<bool>,
    pub policy_work_used: u64,
    pub controller_before: PolicyControllerState,
    pub controller_after: PolicyControllerState,
}

/// Auditable output of a complete independent serialized-program replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramReplayTrace {
    pub made: bool,
    pub plays: Vec<(Seat, Domino)>,
    pub initialization_work: u64,
    pub total_policy_work: u64,
    pub focal_decisions: Vec<ProgramDecisionTrace>,
}

/// Independent full-rules replay through a compiled serialized policy.
/// The controller sees its remaining hand and actor-attributed public record;
/// the concrete world is used only to supply each acting seat's physical hand.
pub fn replay_program(
    fixture: &Fixture,
    field: &dyn SlicePolicy,
    world: &World,
    compiled: &CompiledPolicy,
) -> Result<(bool, Vec<(Seat, Domino)>), String> {
    let replay = replay_program_traced(fixture, field, world, compiled)?;
    Ok((replay.made, replay.plays))
}

/// Independent full-rules replay with policy provenance and work accounting.
pub fn replay_program_traced(
    fixture: &Fixture,
    field: &dyn SlicePolicy,
    world: &World,
    compiled: &CompiledPolicy,
) -> Result<ProgramReplayTrace, String> {
    if !fixture.exercise.root.kernel().contains(world) {
        return Err("program replay outside support".into());
    }
    let initial = root_input(fixture)?;
    let mut budget = Budget::new(POLICY_WORK);
    let before_initialization = budget.spent();
    let mut controller = compiled
        .initialize(&initial, &mut budget)
        .map_err(|e| e.to_string())?;
    let initialization_work = budget.spent() - before_initialization;
    let mut total_policy_work = initialization_work;
    let mut frame = fixture.exercise.frame.clone();
    let mut state = State::from_root(&fixture.exercise.position);
    let mut public_history = fixture.history.clone();
    let mut trace = Vec::new();
    let mut focal_decisions = Vec::new();
    while state.played.len() < 28 {
        let actor = state.actor();
        let hand = world.hand(actor).difference(state.played);
        let legal = legal_plays(
            fixture.exercise.position.decl,
            hand,
            state
                .prefix
                .first()
                .map(|d| fixture.exercise.position.decl.led_context(*d)),
        );
        if legal.is_empty() {
            return Err("program replay reached an invalid empty legal set".into());
        }
        let tile = if actor == fixture.exercise.root.kernel().viewer() {
            let input = PolicyInput::new(
                &frame,
                &public_history,
                state.banked,
                fixture.exercise.position.bid as u8,
                fixture.exercise.position.declaring_team,
            )
            .map_err(|e| e.to_string())?;
            let controller_before = controller.state();
            let work_before = budget.spent();
            let decision = compiled
                .choose_traced(&mut controller, &input, &mut budget)
                .map_err(|e| e.to_string())?;
            let policy_work_used = budget.spent() - work_before;
            total_policy_work += policy_work_used;
            let action = decision.action;
            focal_decisions.push(ProgramDecisionTrace {
                focal_decision_depth: focal_decisions.len(),
                action,
                provenance: decision.provenance,
                contract_resolved: decision.contract_resolved,
                policy_work_used,
                controller_before,
                controller_after: controller.state(),
            });
            action
        } else {
            field.choose(
                fixture.exercise.position.decl,
                hand,
                legal,
                &state.record(&fixture.exercise.position),
            )
        };
        if !legal.contains(tile) {
            return Err("program replay selected an illegal action".into());
        }
        frame = step_frame(&frame, observation(&frame, actor, tile), &mut budget)
            .map_err(|e| e.to_string())?;
        public_history.push((actor, tile));
        trace.push((actor, tile));
        state = state.step(fixture.exercise.position.decl, tile);
    }
    if state.banked.iter().sum::<u32>() != 42 {
        return Err("program replay score conservation".into());
    }
    Ok(ProgramReplayTrace {
        made: state.banked[fixture.exercise.position.declaring_team.index()]
            >= fixture.exercise.position.bid,
        plays: trace,
        initialization_work,
        total_policy_work,
        focal_decisions,
    })
}
