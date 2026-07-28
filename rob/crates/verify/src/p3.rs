//! Player-track stage P3 verification harness: the exact W0 solver and its
//! four correctness gates (BRIEF_PLAYER_01 §8 table P3).
//!
//! Gate 1 (known-world degeneracy): with the fiber pinned to one world the
//! solver must equal an independent perfect-information best-response
//! recursion against σ — same value, same canonical action, full window.
//! Gate 2 (tiny-depth exactness): at ≤ 2 tricks remaining every pure plan
//! is enumerated literally and the solver's value must equal the maximum.
//! Gate 3 (no dominated plan): the solver's plan is not pointwise-dominated
//! by any enumerated plan. Gate 4 (bundle conservation, INV-P5): child
//! bundles partition every node; root equals the capacity-DP count.
//! Plus: double-solve determinism (INV-P3) and the two-engine agreement
//! cross-check (`r_sol_engines`).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use num_bigint::BigUint;
use num_traits::Zero;
use rob_core::{
    algebra_for, derive_rule_cells, unrank_world, DeclarationAlgebra, DominoId, DominoSet,
    MechanicalState, RemainderWorld, Seat,
};
use rob_player::player::UtilityLens;
use rob_player::rollout::RolloutPosition;
use rob_player::{greedy_sigma, solve, solve_pinned, PlanChild, PlanNode};

use crate::p2::{boundary_position, BOUNDARIES};
use crate::receipt::{fmt_commas, Receipt};

const LENS: UtilityLens = UtilityLens::Points;

/// All 756 positions as (boundary, hand index), canonical order.
pub fn all_positions() -> Vec<(usize, u64)> {
    (0..BOUNDARIES)
        .flat_map(|b| (0..108).map(move |i| (b, i)))
        .collect()
}

/// Deterministic parallel map over positions: work-stealing by atomic
/// cursor, results collected in input order (std threads only — no new
/// dependencies).
fn par_map<T: Send>(items: &[(usize, u64)], f: impl Fn(usize, u64) -> T + Sync) -> Vec<T> {
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(items.len().max(1));
    let cursor = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<T>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let i = cursor.fetch_add(1, Ordering::Relaxed);
                if i >= items.len() {
                    break;
                }
                let value = f(items[i].0, items[i].1);
                results.lock().expect("no poisoned workers")[i] = Some(value);
            });
        }
    });
    results
        .into_inner()
        .expect("no poisoned workers")
        .into_iter()
        .map(|v| v.expect("every slot filled"))
        .collect()
}

/// The rank-zero fiber world of a position (CELL-25/26 route — works at
/// any fiber size without enumeration).
fn first_world(state: &MechanicalState) -> RemainderWorld {
    let cells = derive_rule_cells(state);
    let (abstract_cells, pool) = cells.to_abstract();
    let world = unrank_world(&abstract_cells, &BigUint::zero());
    RemainderWorld {
        hidden_hands: core::array::from_fn(|i| {
            DominoSet::from_ids(world[i].iter().map(|&t| pool[t]))
        }),
    }
}

/// Build the complete-information rollout position of `state` + `world`.
fn rollout_position(state: &MechanicalState, world: &RemainderWorld) -> RolloutPosition {
    let mut hands = [DominoSet::empty(); 4];
    hands[state.viewer().index()] = *state.own_remaining_hand();
    for (i, &seat) in state.hidden_seats().iter().enumerate() {
        hands[seat.index()] = world.hidden_hands[i];
    }
    RolloutPosition {
        hands,
        leader: state.leader(),
        trick: state.current_trick().to_vec(),
        points: state.hand_points(),
    }
}

fn points_diff(points: [u32; 2], viewer_team: usize) -> i64 {
    points[viewer_team] as i64 - points[1 - viewer_team] as i64
}

/// Independent perfect-information best response against σ: viewer nodes
/// maximize (ties to the earliest tile in canonical order), every other
/// seat plays `greedy_sigma`, played to settlement. Shares the algebra and
/// σ's spec with the solver but none of its machinery (no tables, no
/// compact worlds, no plan types).
fn reference_value(
    algebra: &DeclarationAlgebra,
    pos: &RolloutPosition,
    viewer: Seat,
    viewer_team: usize,
) -> i64 {
    if pos.hands.iter().all(DominoSet::is_empty) {
        assert!(pos.trick.is_empty());
        return points_diff(pos.points, viewer_team);
    }
    let actor = pos.leader.offset(pos.trick.len() as u8);
    if actor == viewer {
        let mut best: Option<i64> = None;
        for action in pos.legal(algebra) {
            let mut next = pos.clone();
            next.apply(algebra, action);
            let value = reference_value(algebra, &next, viewer, viewer_team);
            if best.is_none() || value > best.expect("some") {
                best = Some(value);
            }
        }
        best.expect("nonempty legal set")
    } else {
        let choice = greedy_sigma(algebra, &pos.hands[actor.index()], &pos.trick);
        let mut next = pos.clone();
        next.apply(algebra, choice);
        reference_value(algebra, &next, viewer, viewer_team)
    }
}

/// The reference's canonical root action: earliest legal tile achieving
/// the maximum (identical tie-break to the solver's INV-P3 rule).
fn reference_best(
    algebra: &DeclarationAlgebra,
    pos: &RolloutPosition,
    viewer: Seat,
    viewer_team: usize,
) -> (i64, DominoId) {
    let mut best: Option<(i64, DominoId)> = None;
    for action in pos.legal(algebra) {
        let mut next = pos.clone();
        next.apply(algebra, action);
        let value = reference_value(algebra, &next, viewer, viewer_team);
        if best.is_none() || value > best.expect("some").0 {
            best = Some((value, action));
        }
    }
    best.expect("nonempty legal set")
}

/// One position's full-solve evidence (gate 4 + determinism, one worker).
struct SolveEvidence {
    nodes_checked: u64,
    value_total: i64,
    truncated: bool,
    window: usize,
}

fn solve_evidence(boundary: usize, index: u64) -> SolveEvidence {
    let state = boundary_position(index, boundary);
    let plan = solve(&state, LENS).expect("Points lens always solves");
    let again = solve(&state, LENS).expect("Points lens always solves");
    assert_eq!(plan, again, "double-solve determinism (INV-P3)");
    let nodes_checked = plan.assert_conservation();
    SolveEvidence {
        nodes_checked,
        value_total: plan.root.value_total,
        truncated: plan.truncated,
        window: plan.window,
    }
}

/// Gates 4 + determinism over a position set: returns (solves, total nodes
/// checked, truncated plans, per-depth value totals).
pub fn conservation_determinism_check(
    positions: &[(usize, u64)],
) -> (u64, u64, u64, [i64; BOUNDARIES]) {
    let evidence = par_map(positions, solve_evidence);
    let mut nodes = 0u64;
    let mut truncated = 0u64;
    let mut values = [0i64; BOUNDARIES];
    for ((boundary, _), e) in positions.iter().zip(&evidence) {
        nodes += e.nodes_checked;
        truncated += u64::from(e.truncated);
        values[*boundary] += e.value_total;
        if *boundary >= 2 {
            assert_eq!(e.window, 7 - boundary, "full depth for t >= 2");
        }
    }
    (positions.len() as u64, nodes, truncated, values)
}

/// Gate 1 over a position set: pinned solver ≡ independent perfect-info
/// best response — value, canonical action, full window, every leaf
/// settled.
pub fn known_world_check(positions: &[(usize, u64)]) -> u64 {
    let agreements = par_map(positions, |boundary, index| {
        let state = boundary_position(index, boundary);
        let world = first_world(&state);
        let plan = solve_pinned(&state, &world, LENS).expect("Points lens always solves");
        assert_eq!(plan.fiber_count, 1);
        assert_eq!(
            plan.window,
            state.own_remaining_hand().len(),
            "a pinned fiber affords full depth"
        );
        let algebra = algebra_for(state.contract().declaration());
        let viewer = state.viewer();
        let viewer_team = viewer.team().index();
        let pos = rollout_position(&state, &world);
        let (value, action) = reference_best(&algebra, &pos, viewer, viewer_team);
        assert_eq!(plan.root.value_total, value, "known-world value agreement");
        assert_eq!(plan.root.action, action, "known-world action agreement");
        1u64
    });
    agreements.iter().sum()
}

/// Advance one complete-information world under σ until the next viewer
/// decision or settlement; returns the observation and whether it settled.
fn advance_world(
    algebra: &DeclarationAlgebra,
    pos: &mut RolloutPosition,
    viewer: Seat,
) -> (Vec<(u8, u8)>, bool) {
    let mut obs = Vec::new();
    loop {
        if pos.hands.iter().all(DominoSet::is_empty) {
            assert!(pos.trick.is_empty());
            return (obs, true);
        }
        let actor = pos.leader.offset(pos.trick.len() as u8);
        if actor == viewer {
            return (obs, false);
        }
        let choice = greedy_sigma(algebra, &pos.hands[actor.index()], &pos.trick);
        pos.apply(algebra, choice);
        obs.push((actor.index() as u8, choice.index() as u8));
    }
}

/// Literal pure-plan enumeration (gate 2): every assignment of one action
/// per reachable viewer information set, with exact per-world outcomes.
fn all_plans(
    algebra: &DeclarationAlgebra,
    bundle: &BTreeMap<usize, RolloutPosition>,
    viewer: Seat,
    viewer_team: usize,
) -> Vec<(DominoId, BTreeMap<usize, i64>)> {
    let representative = bundle.values().next().expect("nonempty bundle");
    let legal = representative.legal(algebra);
    let mut plans = Vec::new();
    for action in legal {
        // Advance every world; split into settled outcomes and decision
        // groups by observation.
        let mut settled: BTreeMap<usize, i64> = BTreeMap::new();
        let mut groups: BTreeMap<Vec<(u8, u8)>, BTreeMap<usize, RolloutPosition>> = BTreeMap::new();
        for (&w, pos) in bundle {
            let mut next = pos.clone();
            next.apply(algebra, action);
            let (obs, done) = advance_world(algebra, &mut next, viewer);
            if done {
                settled.insert(w, points_diff(next.points, viewer_team));
            } else {
                groups.entry(obs).or_default().insert(w, next);
            }
        }
        // Sub-plan choices per decision group; cartesian product across
        // groups.
        let group_plans: Vec<Vec<BTreeMap<usize, i64>>> = groups
            .values()
            .map(|g| {
                all_plans(algebra, g, viewer, viewer_team)
                    .into_iter()
                    .map(|(_, outcomes)| outcomes)
                    .collect()
            })
            .collect();
        let mut combos: Vec<BTreeMap<usize, i64>> = vec![settled];
        for choices in &group_plans {
            let mut expanded = Vec::new();
            for combo in &combos {
                for choice in choices {
                    let mut merged = combo.clone();
                    merged.extend(choice.iter().map(|(&w, &v)| (w, v)));
                    expanded.push(merged);
                }
            }
            combos = expanded;
        }
        for outcomes in combos {
            plans.push((action, outcomes));
        }
    }
    plans
}

/// Replay the solver's materialized plan against one complete-information
/// world; the sum over the fiber must equal the plan's root total.
fn replay_plan(
    algebra: &DeclarationAlgebra,
    node: &PlanNode,
    mut pos: RolloutPosition,
    viewer: Seat,
    viewer_team: usize,
) -> i64 {
    pos.apply(algebra, node.action);
    let (obs, done) = advance_world(algebra, &mut pos, viewer);
    if done {
        return points_diff(pos.points, viewer_team);
    }
    match node
        .children
        .get(&obs)
        .expect("every realizable observation is a plan key")
    {
        PlanChild::Node(next) => replay_plan(algebra, next, pos, viewer, viewer_team),
        PlanChild::Leaf(_) => unreachable!("full-depth tiny plans settle"),
    }
}

/// Gates 2 + 3 on the tiny positions (boundaries 5 and 6): returns
/// (positions, plans enumerated).
pub fn brute_force_check() -> (u64, u64) {
    let positions: Vec<(usize, u64)> = all_positions()
        .into_iter()
        .filter(|&(b, _)| b >= 5)
        .collect();
    let results = par_map(&positions, |boundary, index| {
        let state = boundary_position(index, boundary);
        let algebra = algebra_for(state.contract().declaration());
        let viewer = state.viewer();
        let viewer_team = viewer.team().index();
        let plan = solve(&state, LENS).expect("Points lens always solves");
        assert_eq!(plan.window, 7 - boundary, "tiny positions solve full depth");

        let worlds = derive_rule_cells(&state).fiber_worlds();
        let bundle: BTreeMap<usize, RolloutPosition> = worlds
            .iter()
            .enumerate()
            .map(|(w, world)| (w, rollout_position(&state, world)))
            .collect();
        let plans = all_plans(&algebra, &bundle, viewer, viewer_team);
        assert!(!plans.is_empty());
        for (_, outcomes) in &plans {
            assert_eq!(outcomes.len(), worlds.len(), "every plan covers the fiber");
        }
        let best = plans
            .iter()
            .map(|(_, o)| o.values().sum::<i64>())
            .max()
            .expect("nonempty plan space");
        assert_eq!(
            plan.root.value_total, best,
            "solver value equals the literal pure-plan maximum (gate 2)"
        );
        let argmax_roots: Vec<DominoId> = plans
            .iter()
            .filter(|(_, o)| o.values().sum::<i64>() == best)
            .map(|&(a, _)| a)
            .collect();
        assert!(
            argmax_roots.contains(&plan.root.action),
            "solver action lies in the argmax set (gate 2)"
        );

        // Gate 3: the solver's realized per-world outcomes are not
        // pointwise-dominated by any enumerated plan.
        let solver_outcomes: Vec<i64> = worlds
            .iter()
            .map(|world| {
                replay_plan(
                    &algebra,
                    &plan.root,
                    rollout_position(&state, world),
                    viewer,
                    viewer_team,
                )
            })
            .collect();
        assert_eq!(
            solver_outcomes.iter().sum::<i64>(),
            plan.root.value_total,
            "plan replay reproduces the solver total"
        );
        for (_, outcomes) in &plans {
            let dominates = !worlds.is_empty()
                && (0..worlds.len()).all(|w| outcomes[&w] >= solver_outcomes[w])
                && (0..worlds.len()).any(|w| outcomes[&w] > solver_outcomes[w]);
            assert!(
                !dominates,
                "no plan pointwise-dominates the solver's (gate 3)"
            );
        }
        plans.len() as u64
    });
    (positions.len() as u64, results.iter().sum())
}

/// The two-engine cross-check on every position where the window-1 fiber
/// is enumerable (boundaries ≥ 2): the counting and streaming engines must
/// produce identical plans — values, actions, bundles, everything.
pub fn engines_check(positions: &[(usize, u64)]) -> u64 {
    let agreements = par_map(positions, |boundary, index| {
        assert!(
            boundary >= 2,
            "streaming window-1 needs an enumerable fiber"
        );
        let state = boundary_position(index, boundary);
        let counting = rob_player::solver::gate::counting_h1(&state, LENS);
        let streaming = rob_player::solver::gate::streaming_h1(&state, LENS);
        assert_eq!(counting, streaming, "engine agreement (r_sol_engines)");
        1u64
    });
    agreements.iter().sum()
}

fn fmt_signed(v: i64) -> String {
    if v < 0 {
        format!("-{}", fmt_commas(v.unsigned_abs() as u128))
    } else {
        fmt_commas(v as u128)
    }
}

/// Build the canonical P3 receipt (BRIEF_PLAYER_01 §8–§9). Panics on any
/// gate failure.
pub fn receipt() -> String {
    let mut r = Receipt::new("player-p3");
    let all = all_positions();

    let known = known_world_check(&all);
    r.line("r_sol_known_world", &format!("{known} agreements"));

    let (tiny, plans) = brute_force_check();
    r.line(
        "r_sol_brute_force",
        &format!(
            "{tiny} positions; {} pure plans enumerated",
            fmt_commas(plans as u128)
        ),
    );
    r.line("r_sol_undominated", &format!("{tiny} positions"));

    let (solves, nodes, truncated, values) = conservation_determinism_check(&all);
    r.line(
        "r_sol_conservation",
        &format!(
            "{solves} solves; {} nodes checked; {truncated} truncated",
            fmt_commas(nodes as u128)
        ),
    );
    let per_depth: Vec<String> = values.iter().map(|&v| fmt_signed(v)).collect();
    r.line(
        "r_sol_deterministic",
        &format!(
            "{solves} double-solves byte-equal; values {}",
            per_depth.join(" / ")
        ),
    );

    let engine_positions: Vec<(usize, u64)> =
        all.iter().copied().filter(|&(b, _)| b >= 2).collect();
    let engines = engines_check(&engine_positions);
    r.line("r_sol_engines", &format!("{engines} agreements"));
    r.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Positions cheap enough for `cargo test` (the receipt binary runs all
    /// 756; boundary-2 full-depth solves are seconds each and dominate).
    fn fast_positions() -> Vec<(usize, u64)> {
        all_positions()
            .into_iter()
            .filter(|&(b, _)| b != 2)
            .collect()
    }

    #[test]
    fn r_sol_known_world() {
        assert_eq!(known_world_check(&all_positions()), 756);
    }

    #[test]
    fn r_sol_brute_force_and_undominated() {
        assert_eq!(brute_force_check().0, 216);
    }

    #[test]
    fn r_sol_conservation_deterministic() {
        let (solves, nodes, _, _) = conservation_determinism_check(&fast_positions());
        assert_eq!(solves, 648);
        assert!(nodes > 0);
    }

    #[test]
    fn r_sol_engines() {
        let positions: Vec<(usize, u64)> = all_positions()
            .into_iter()
            .filter(|&(b, _)| b >= 3)
            .collect();
        assert_eq!(engines_check(&positions), 432);
    }
}
