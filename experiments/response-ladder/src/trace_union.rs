//! Exact sampled response reconstructed from a complete union of public traces.
//!
//! Original column bits, never lane multiplicity, carry weighted arrival mass.
//! At focal nodes all legal children carry the same mask; field children form
//! a complete disjoint partition. Only public contract settlement is terminal.
//!
//! Producer boundary: traces must come from the named immutable deterministic
//! field. This fold checks mechanics, per-column determinism and tree coverage;
//! it cannot authenticate a consistently altered legal field action without
//! evaluating that field again. `TraceBundle` pins the full ordered problem,
//! and independent policy replay is the end-to-end field-fidelity check.
use crate::mechanics::{tiles, Budget, Problem, PublicState};
use crate::policy::{Decision, Policy};
use std::collections::BTreeMap;
use walt::rules::Decl;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    pub payoff: u64,
    /// Every physical tile after the fixed root, ending at first public payoff.
    pub continuation: Vec<u8>,
}
impl From<crate::rollout::LaneTrace> for Trace {
    fn from(value: crate::rollout::LaneTrace) -> Self {
        Self {
            payoff: u64::from(value.payoff),
            continuation: value.plays,
        }
    }
}
#[cfg(feature = "gpu")]
impl From<crate::gpu_epochs::EpochTrace> for Trace {
    fn from(value: crate::gpu_epochs::EpochTrace) -> Self {
        Self {
            payoff: value.payoff,
            continuation: value.continuation,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Identity {
    decl: Decl,
    bid: u8,
    viewer: u8,
    hand: u32,
    root: PublicState,
    columns: Vec<([u32; 4], u64, u64)>,
    field_revision: String,
}
impl Identity {
    fn of(p: &Problem) -> Self {
        Self {
            decl: p.decl,
            bid: p.bid,
            viewer: p.viewer,
            hand: p.hand,
            root: p.root.clone(),
            columns: p
                .scenarios
                .iter()
                .map(|s| (s.hands, s.tape, s.weight))
                .collect(),
            field_revision: p.field_revision.clone(),
        }
    }
}

/// Rows may repeat traces. Every row must contain every ORIGINAL column in
/// original order. Completeness is verified from the union, not row count:
/// a partial permutation family is rejected whenever any tree coverage is
/// missing; a smaller family that covers the entire same tree is sufficient.
#[derive(Clone, Debug)]
pub struct TraceBundle {
    identity: Identity,
    pub rows: Vec<Vec<Trace>>,
}
impl TraceBundle {
    /// Call at the producer boundary using the same frozen problem as rollout.
    pub fn new(problem: &Problem, rows: Vec<Vec<Trace>>) -> Self {
        Self {
            identity: Identity::of(problem),
            rows,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExactResponse {
    /// Every legal root action, sorted by tile. Values are unnormalized integer
    /// focal-team success masses on the fixed original bundle.
    pub action_values: Vec<(u8, u64)>,
    pub chosen: u8,
    pub policy: Policy,
    pub value: u64,
    pub mass: u64,
    pub field_revision: String,
    pub unique_nodes: usize,
    pub lanes: usize,
}

const NONE: u32 = u32::MAX;
const NONTERMINAL: u8 = 2;

// No per-node history allocation, hidden-hand copy or 28-slot child array.
#[derive(Clone, Copy)]
struct Node {
    mask: u64,
    value: u64,
    first_child: u32,
    sibling: u32,
    best_child: u32,
    tile: u8,
    terminal: u8,
}
impl Node {
    fn new(tile: u8, sibling: u32) -> Self {
        Self {
            mask: 0,
            value: 0,
            first_child: NONE,
            sibling,
            best_child: NONE,
            tile,
            terminal: NONTERMINAL,
        }
    }
}

fn child(arena: &mut Vec<Node>, parent: u32, tile: u8) -> Result<u32, String> {
    let first = arena[parent as usize].first_child;
    let mut id = first;
    while id != NONE {
        let n = arena[id as usize];
        if n.tile == tile {
            return Ok(id);
        }
        id = n.sibling;
    }
    let id = u32::try_from(arena.len()).map_err(|_| "trace trie exceeds u32 arena")?;
    if id == NONE {
        return Err("trace trie exhausted node indices".into());
    }
    arena
        .try_reserve(1)
        .map_err(|_| "trace trie allocation refused")?;
    arena.push(Node::new(tile, first));
    arena[parent as usize].first_child = id;
    Ok(id)
}

fn mass(p: &Problem, mut mask: u64) -> u64 {
    // Problem::validate checked the full positive u64 sum; subsets cannot overflow.
    let mut value = 0;
    while mask != 0 {
        let id = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        value += p.scenarios[id].weight;
    }
    value
}

/// Validate each COMPLETE face before evaluating any of its descendants.
/// The recursive stack is at most 29 public states, independent of lane count.
fn reduce(
    p: &Problem,
    arena: &mut [Node],
    id: u32,
    state: &PublicState,
    budget: &mut Budget,
) -> Result<Option<u64>, String> {
    if budget.tick().is_none() {
        return Ok(None);
    }
    let n = arena[id as usize];
    if let Some(payoff) = state.payoff(p.bid, p.viewer) {
        if n.first_child != NONE || u64::from(n.terminal) != payoff {
            return Err("trace does not stop with the first public payoff".into());
        }
        let value = mass(p, n.mask) * payoff;
        arena[id as usize].value = value;
        return Ok(Some(value));
    }
    if n.terminal != NONTERMINAL || n.first_child == NONE {
        return Err("trace stops before public payoff / incomplete continuation".into());
    }
    let focal = state.actor() == p.viewer;
    let legal = if focal {
        state.legal(p.decl, p.hand)
    } else {
        0
    };
    let mut observed = 0u32;
    let mut partition = 0u64;
    let mut at = n.first_child;
    while at != NONE {
        if budget.tick().is_none() {
            return Ok(None);
        }
        let c = arena[at as usize];
        if c.mask == 0 || c.mask & !n.mask != 0 || observed & (1 << c.tile) != 0 {
            return Err("invalid or repeated trace child".into());
        }
        observed |= 1 << c.tile;
        if focal {
            if c.mask != n.mask || legal & (1 << c.tile) == 0 {
                return Err("focal child lacks full common mask or is illegal".into());
            }
        } else {
            if partition & c.mask != 0 {
                return Err("one original column has two field actions at one history".into());
            }
            partition |= c.mask;
            let mut ids = c.mask;
            while ids != 0 {
                if budget.tick().is_none() {
                    return Ok(None);
                }
                let sid = ids.trailing_zeros() as usize;
                ids &= ids - 1;
                if state.legal(p.decl, p.scenarios[sid].hands[state.actor() as usize])
                    & (1 << c.tile)
                    == 0
                {
                    return Err("trace contains illegal modeled-seat action".into());
                }
            }
        }
        at = c.sibling;
    }
    if focal && observed != legal {
        return Err("incomplete legal focal action face".into());
    }
    if !focal && partition != n.mask {
        return Err("incomplete modeled observation partition".into());
    }
    let mut value = 0;
    let mut best = NONE;
    at = n.first_child;
    while at != NONE {
        let c = arena[at as usize];
        let next = state.after(p.decl, c.tile);
        let Some(v) = reduce(p, arena, at, &next, budget)? else {
            return Ok(None);
        };
        if focal {
            if best == NONE || v > value || (v == value && c.tile < arena[best as usize].tile) {
                value = v;
                best = at;
            }
        } else {
            value += v; // Disjoint original masks bound this by parent mass.
        }
        at = c.sibling;
    }
    if value > mass(p, n.mask) {
        return Err("trace value exceeds original arrival mass".into());
    }
    arena[id as usize].value = value;
    arena[id as usize].best_child = best;
    Ok(Some(value))
}

fn extract(
    p: &Problem,
    arena: &[Node],
    id: u32,
    state: &PublicState,
    choices: &mut BTreeMap<Vec<u8>, u8>,
    budget: &mut Budget,
) -> Result<Option<()>, String> {
    if budget.tick().is_none() {
        return Ok(None);
    }
    let n = arena[id as usize];
    if n.first_child == NONE {
        return Ok(Some(()));
    }
    if state.actor() == p.viewer {
        if n.best_child == NONE {
            return Err("missing folded focal decision".into());
        }
        let c = arena[n.best_child as usize];
        if choices.insert(state.history.clone(), c.tile).is_some() {
            return Err("duplicate complete-history policy decision".into());
        }
        extract(
            p,
            arena,
            n.best_child,
            &state.after(p.decl, c.tile),
            choices,
            budget,
        )
    } else {
        let mut at = n.first_child;
        while at != NONE {
            let c = arena[at as usize];
            if extract(p, arena, at, &state.after(p.decl, c.tile), choices, budget)?.is_none() {
                return Ok(None);
            }
            at = c.sibling;
        }
        Ok(Some(()))
    }
}

/// Publish a complete exact vector and total canonical policy, or no result.
/// `Ok(None)` is cancellation; malformed/identity-mismatched/incomplete traces
/// are errors. No incomplete vector, zero-filled lane, or partial policy escapes.
pub fn fold(
    problem: &Problem,
    bundle: &TraceBundle,
    budget: &mut Budget,
) -> Result<Option<ExactResponse>, String> {
    if budget.tick().is_none() {
        return Ok(None);
    }
    problem.validate()?;
    let count = problem.scenarios.len();
    if count > 64 {
        return Err("trace union supports at most 64 original columns".into());
    }
    if bundle.identity != Identity::of(problem) {
        return Err("trace bundle problem/field identity mismatch".into());
    }
    if bundle.rows.is_empty() {
        return Err("empty trace family".into());
    }
    let all = if count == 64 {
        u64::MAX
    } else {
        (1u64 << count) - 1
    };
    let mut arena = vec![Node::new(0, NONE)];
    let mut lanes = 0usize;
    let max_plays = 28 - problem.root.played.count_ones() as usize;
    for row in &bundle.rows {
        if row.len() != count {
            return Err("trace row lacks original columns".into());
        }
        for (sid, trace) in row.iter().enumerate() {
            if budget.tick().is_none() {
                return Ok(None);
            }
            if trace.payoff > 1 || trace.continuation.len() > max_plays {
                return Err("invalid trace payoff or physical length".into());
            }
            let bit = 1u64 << sid;
            let mut id = 0u32;
            let mut played = problem.root.played;
            arena[0].mask |= bit;
            for &tile in &trace.continuation {
                if budget.tick().is_none() {
                    return Ok(None);
                }
                if tile >= 28 || played & (1 << tile) != 0 {
                    return Err("trace contains invalid or already played tile".into());
                }
                played |= 1 << tile;
                id = child(&mut arena, id, tile)?;
                arena[id as usize].mask |= bit;
            }
            let n = &mut arena[id as usize];
            if n.terminal != NONTERMINAL && u64::from(n.terminal) != trace.payoff {
                return Err("conflicting payoff at one public history".into());
            }
            n.terminal = trace.payoff as u8;
            lanes = lanes.checked_add(1).ok_or("trace lane count overflow")?;
        }
    }
    if arena[0].mask != all {
        return Err("root is missing original columns".into());
    }
    let Some(value) = reduce(problem, &mut arena, 0, &problem.root, budget)? else {
        return Ok(None);
    };
    let mut action_values = Vec::new();
    if problem.root.payoff(problem.bid, problem.viewer).is_some() {
        action_values.extend(
            tiles(problem.root.legal(problem.decl, problem.hand))
                .into_iter()
                .map(|a| (a, value)),
        );
    } else {
        let mut at = arena[0].first_child;
        while at != NONE {
            let n = arena[at as usize];
            action_values.push((n.tile, n.value));
            at = n.sibling;
        }
        action_values.sort_unstable_by_key(|&(tile, _)| tile);
    }
    let &(mut chosen, mut best) = action_values.first().ok_or("root has no legal action")?;
    for &(tile, v) in &action_values[1..] {
        if v > best {
            chosen = tile;
            best = v;
        }
    }
    let mut choices = BTreeMap::new();
    if extract(problem, &arena, 0, &problem.root, &mut choices, budget)?.is_none() {
        return Ok(None);
    }
    let policy = Policy {
        viewer: problem.viewer,
        priority: tiles(problem.hand),
        decisions: choices
            .into_iter()
            .map(|(history, action)| Decision { history, action })
            .collect(),
        compiled_tail: None,
    };
    if budget.exhausted() {
        return Ok(None);
    }
    Ok(Some(ExactResponse {
        action_values,
        chosen,
        policy,
        value: best,
        mass: problem.mass(),
        field_revision: problem.field_revision.clone(),
        unique_nodes: arena.len(),
        lanes,
    }))
}
