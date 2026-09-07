//! Interchange between sampled table policies and executable Scheme policies.

use std::collections::BTreeMap;

use crate::kernel::World;
use crate::rules::{legal_plays, Domino, Seat};
use crate::scheme::{
    step_frame, Budget, CompiledPolicy, ExactRule, Fallback, ObservedPlay, PlayClass, PolicyInput,
    PolicyKey, PolicyProgram,
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

/// Independent full-rules replay through a compiled serialized policy.
/// The controller sees its remaining hand and actor-attributed public record;
/// the concrete world is used only to supply each acting seat's physical hand.
pub fn replay_program(
    fixture: &Fixture,
    field: &dyn SlicePolicy,
    world: &World,
    compiled: &CompiledPolicy,
) -> Result<(bool, Vec<(Seat, Domino)>), String> {
    if !fixture.exercise.root.kernel().contains(world) {
        return Err("program replay outside support".into());
    }
    let initial = root_input(fixture)?;
    let mut budget = Budget::new(POLICY_WORK);
    let mut controller = compiled
        .initialize(&initial, &mut budget)
        .map_err(|e| e.to_string())?;
    let mut frame = fixture.exercise.frame.clone();
    let mut state = State::from_root(&fixture.exercise.position);
    let mut public_history = fixture.history.clone();
    let mut trace = Vec::new();
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
            compiled
                .choose(&mut controller, &input, &mut budget)
                .map_err(|e| e.to_string())?
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
    Ok((
        state.banked[fixture.exercise.position.declaring_team.index()]
            >= fixture.exercise.position.bid,
        trace,
    ))
}
