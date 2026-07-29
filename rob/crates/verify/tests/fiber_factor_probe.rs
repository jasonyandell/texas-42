//! Exploratory probe (wiki/idea-hierarchical-fibers — feature
//! factorization; NOT receipt rows unless promoted by amendment): does
//! factoring the fiber by WHERE-THE-COUNT-IS constrain the decision?
//!
//! The factor coordinate: the assignment of the live count tiles
//! (5-0, 4-1, 3-2, 6-4, 5-5, those still in the unseen pool) to the three
//! hidden seats — at most 3^5 = 243 cells, each an intensional sub-fiber
//! (conditioning by tile→seat assignment is the force move of the
//! matching-minor calculus, TRANS-08/09). The probe prices the coordinate
//! exactly, in the currency the game pays:
//!
//! - **VOI(coordinate)** = Σ over cells of (cell-best opening's margin sum
//!   − global-best opening's margin sum within the cell): the exact value
//!   of learning the coordinate before committing to an opening.
//! - **VOI(full)** = Σ over worlds of (per-world best opening margin −
//!   global-best opening margin): the perfect-information gap, an upper
//!   bound for every coordinate.
//! - Their ratio is the fraction of decision-relevant uncertainty the
//!   coordinate captures; cells whose best opening disagrees with the
//!   global best are the contested regions a hierarchical engine would
//!   refine (idea page rung 3's refinement signal, made concrete).
//! - Contested cells are then refined by a second coordinate —
//!   WHERE-ARE-THE-BEATERS of the opening tile (pool tiles that beat it
//!   in its own led context) — and the residual VOI is measured again.
//!
//! Self-validation: per opening, the replayed per-world margins must sum
//! to the exact `gate::solve_opening` plan value.
//!
//! Findings, frozen 2026-07-29 (exploratory; quoted in the idea page §8):
//! across 24 boundary-3 positions the coordinate alone captures 0–390‰
//! of VOI(full) (four positions capture exactly 0 — the opening is
//! optimal in every cell), and composing with beaters-of-the-opening on
//! contested cells reaches up to 613‰. At the boundary-2 wall: index 1
//! (fiber 72,072) has only 96‰ of worlds in agreeing cells (8 of 9 cells
//! contested) and the composed coordinate captures 592‰ via 1,587
//! sub-cells; index 2 (fiber 324,324): 496‰ agree, composed 338‰ via
//! 5,623 sub-cells. Index 0's plan exceeds the materialization cap and
//! is skipped.
//!
//! Rung-3 bounds (`bound_cover_*` tests), frozen 2026-07-29: with
//! U_a = Σ per-world clairvoyant-vs-σ margins (first move pinned) and
//! L = exact plan values, bounds alone settle the root decision at 10 of
//! 13 positions (9/12 at boundary 3; the boundary-2 wall position closes
//! with closest rival U 165,504 < V* 174,554 in ≈ 6 s). Clairvoyance
//! premium U − V*: 0–353‰ of |V|, exactly 0 twice, 64‰ at the wall.
//! U_a ≥ V_a asserted at every position (soundness); decided-cell world
//! shares 0–1000‰ at count-location granularity. Quoted in the idea page
//! §9.
//!
//! Run: `cargo test --release --test fiber_factor_probe -- --ignored --nocapture`

use std::collections::BTreeMap;

use rob_core::{
    algebra_for, derive_rule_cells, domino_id, DeclarationAlgebra, Domino, DominoId, DominoSet,
    MechanicalState, Pip, Play, Seat,
};
use rob_player::player::UtilityLens;
use rob_player::solver::gate::solve_opening;
use rob_player::{greedy_sigma, PlanChild, PlanNode};
use rob_verify::p2::boundary_position;

fn tile(name: &str) -> DominoId {
    let (h, l) = name.split_once('-').expect("high-low");
    domino_id(Domino::new(
        Pip::new(h.parse().unwrap()).unwrap(),
        Pip::new(l.parse().unwrap()).unwrap(),
    ))
}

fn count_tiles() -> [DominoId; 5] {
    ["5-0", "4-1", "3-2", "6-4", "5-5"].map(tile)
}

struct Sim {
    hands: [DominoSet; 4],
    leader: Seat,
    trick: Vec<Play>,
    banked: [u32; 2],
}

impl Sim {
    fn actor(&self) -> Seat {
        self.leader.offset(self.trick.len() as u8)
    }

    fn apply(&mut self, algebra: &DeclarationAlgebra, d: DominoId) {
        let actor = self.actor();
        assert!(self.hands[actor.index()].contains(d));
        self.hands[actor.index()].remove(d);
        self.trick.push(Play { actor, domino: d });
        if self.trick.len() == 4 {
            let result = algebra.resolve_trick(&self.trick).expect("resolves");
            self.banked[result.winner.team().index()] += result.points as u32;
            self.leader = result.winner;
            self.trick.clear();
        }
    }
}

/// Replay one materialized plan against σ in one world; final viewer-team
/// margin (the nickel-probe pattern, outcome only).
fn replay(algebra: &DeclarationAlgebra, node: &PlanNode, mut sim: Sim, viewer: Seat) -> i64 {
    let mut current = node;
    let mut obs: Vec<(u8, u8)> = Vec::new();
    let mut pending_action = Some(current.action);
    loop {
        if sim.hands.iter().all(DominoSet::is_empty) {
            return sim.banked[viewer.team().index()] as i64
                - sim.banked[viewer.team().opponent().index()] as i64;
        }
        let actor = sim.actor();
        let d = if actor == viewer {
            if pending_action.is_none() {
                match current.children.get(&obs).expect("realizable obs is a key") {
                    PlanChild::Node(n) => current = n,
                    PlanChild::Leaf(_) => unreachable!("full-depth plans settle"),
                }
                obs.clear();
            }
            pending_action.take().unwrap_or(current.action)
        } else {
            let choice = greedy_sigma(algebra, &sim.hands[actor.index()], &sim.trick);
            obs.push((actor.index() as u8, choice.index() as u8));
            choice
        };
        sim.apply(algebra, d);
        if actor == viewer {
            pending_action = None;
        }
    }
}

/// Integer per-mille of `part` in `whole`.
fn permille(part: i64, whole: i64) -> i64 {
    if whole == 0 {
        0
    } else {
        part * 1000 / whole
    }
}

fn probe_position(boundary: usize, index: u64) {
    let state: MechanicalState = boundary_position(index, boundary);
    let viewer = state.viewer();
    let cells = derive_rule_cells(&state);
    let worlds = cells.fiber_worlds();
    let algebra = algebra_for(state.contract().declaration());

    // The factor coordinate: live count tiles → holder (hidden index 0..2).
    let live_count: Vec<DominoId> = count_tiles()
        .into_iter()
        .filter(|d| cells.unseen_pool().contains(*d))
        .collect();
    let world_key = |w: &rob_core::RemainderWorld| -> Vec<u8> {
        live_count
            .iter()
            .map(|&d| {
                w.hidden_hands
                    .iter()
                    .position(|h| h.contains(d))
                    .expect("a pool tile is in some hidden hand") as u8
            })
            .collect()
    };

    // Every legal opening's exact full plan, then per-world margins
    // (boundary positions lead a fresh trick — the whole hand is legal).
    let legal: Vec<DominoId> = state.own_remaining_hand().iter().collect();
    let mut plans = Vec::new();
    for &opening in &legal {
        let plan = solve_opening(&state, UtilityLens::Points, opening);
        if plan.truncated {
            println!(
                "boundary {boundary} index {index}: opening {opening:?} plan truncated — skipping position"
            );
            return;
        }
        plans.push(plan);
    }
    let mut margins: Vec<Vec<i64>> = Vec::with_capacity(plans.len());
    for plan in &plans {
        let mut per_world = Vec::with_capacity(worlds.len());
        let mut sum = 0i64;
        for world in &worlds {
            let mut hands = [DominoSet::empty(); 4];
            hands[viewer.index()] = *state.own_remaining_hand();
            for (i, &seat) in state.hidden_seats().iter().enumerate() {
                hands[seat.index()] = world.hidden_hands[i];
            }
            let sim = Sim {
                hands,
                leader: state.leader(),
                trick: state.current_trick().to_vec(),
                banked: state.hand_points(),
            };
            let m = replay(&algebra, &plan.root, sim, viewer);
            sum += m;
            per_world.push(m);
        }
        assert_eq!(
            sum, plan.root.value_total,
            "replay reproduces the solver's exact value (self-validation)"
        );
        margins.push(per_world);
    }

    // Global best opening (value, then lowest id — the solver's tie-break).
    let global = (0..plans.len())
        .max_by_key(|&a| {
            (
                plans[a].root.value_total,
                std::cmp::Reverse(legal[a].index()),
            )
        })
        .expect("nonempty legal");

    // Cells of the coordinate, with per-opening sums.
    let mut cell_sums: BTreeMap<Vec<u8>, Vec<i64>> = BTreeMap::new();
    let mut cell_ns: BTreeMap<Vec<u8>, u64> = BTreeMap::new();
    for (w, world) in worlds.iter().enumerate() {
        let key = world_key(world);
        let sums = cell_sums
            .entry(key.clone())
            .or_insert_with(|| vec![0i64; plans.len()]);
        for (a, per_world) in margins.iter().enumerate() {
            sums[a] += per_world[w];
        }
        *cell_ns.entry(key).or_insert(0) += 1;
    }

    // VOI(coordinate) and the contested cells.
    let mut voi_cells = 0i64;
    let mut contested: Vec<Vec<u8>> = Vec::new();
    let mut agree_worlds = 0u64;
    for (key, sums) in &cell_sums {
        let best = (0..sums.len())
            .max_by_key(|&a| (sums[a], std::cmp::Reverse(legal[a].index())))
            .expect("nonempty");
        voi_cells += sums[best] - sums[global];
        if best == global {
            agree_worlds += cell_ns[key];
        } else {
            contested.push(key.clone());
        }
    }

    // VOI(full): per-world best vs the global opening.
    let mut voi_full = 0i64;
    for (w, &global_margin) in margins[global].iter().enumerate() {
        let best = margins.iter().map(|m| m[w]).max().expect("nonempty");
        voi_full += best - global_margin;
    }

    println!(
        "boundary {boundary} index {index}: fiber {} | openings {} | live count tiles {} | cells {} | global opening {:?}",
        worlds.len(),
        legal.len(),
        live_count.len(),
        cell_sums.len(),
        legal[global],
    );
    println!(
        "  agree: {} of {} worlds ({}‰) in cells whose best opening = global | contested cells {}",
        agree_worlds,
        worlds.len(),
        permille(agree_worlds as i64, worlds.len() as i64),
        contested.len(),
    );
    println!(
        "  VOI: coordinate {} | full {} | captured {}‰ (of the perfect-information gap)",
        voi_cells,
        voi_full,
        permille(voi_cells, voi_full),
    );

    // Second coordinate on contested cells only: where are the beaters of
    // the global opening tile (pool tiles beating it in its own led
    // context)?
    if contested.is_empty() {
        return;
    }
    let open_tile = legal[global];
    let led = algebra.led_suit(open_tile);
    let open_key = algebra.trick_key(open_tile, led);
    let beaters: Vec<DominoId> = cells
        .unseen_pool()
        .iter()
        .filter(|&d| algebra.trick_key(d, led) > open_key)
        .collect();
    let beater_key = |w: &rob_core::RemainderWorld| -> Vec<u8> {
        beaters
            .iter()
            .map(|&d| {
                w.hidden_hands
                    .iter()
                    .position(|h| h.contains(d))
                    .expect("a pool tile is in some hidden hand") as u8
            })
            .collect()
    };
    let mut refined_sums: BTreeMap<(Vec<u8>, Vec<u8>), Vec<i64>> = BTreeMap::new();
    for (w, world) in worlds.iter().enumerate() {
        let key = world_key(world);
        if !contested.contains(&key) {
            continue;
        }
        let sums = refined_sums
            .entry((key, beater_key(world)))
            .or_insert_with(|| vec![0i64; plans.len()]);
        for (a, per_world) in margins.iter().enumerate() {
            sums[a] += per_world[w];
        }
    }
    let mut voi_refined = 0i64;
    for sums in refined_sums.values() {
        let best = (0..sums.len())
            .max_by_key(|&a| (sums[a], std::cmp::Reverse(legal[a].index())))
            .expect("nonempty");
        voi_refined += sums[best] - sums[global];
    }
    // Agreeing cells contribute 0 to VOI, so the composed (count-tiles,
    // then beaters on contested cells) coordinate's VOI is the refined
    // sub-cell sum alone; it can only grow toward VOI(full).
    println!(
        "  refine contested by beaters of {:?} ({} beaters): {} sub-cells | composed VOI {} | captured {}‰",
        open_tile,
        beaters.len(),
        refined_sums.len(),
        voi_refined,
        permille(voi_refined, voi_full),
    );
}

/// Boundary 3 (fibers ≤ 34,650, full-depth plans well under the
/// materialization cap): the wide sweep.
#[test]
#[ignore]
fn count_location_boundary3() {
    for index in 0..24u64 {
        probe_position(3, index);
    }
}

/// Boundary 2 — the trick-3 wall itself (fibers ≤ 756,756). Heavier: each
/// position solves every opening's full plan (~10–17 s each).
#[test]
#[ignore]
fn count_location_boundary2() {
    for index in 0..3u64 {
        probe_position(2, index);
    }
}

// ---------------------------------------------------------------------------
// Rung 3 without new theorems: sound bounds that decompose over cells.
//
// Upper bound U_a(S) = Σ_{w∈S} (perfect-information best margin vs σ with
// the first move fixed to `a`) — sound because one plan per info set can
// never beat per-world best play (max of sums ≤ sum of maxes); this is the
// known-world gate's quantity, summed. Lower bound L_a(S) = the concrete
// plan-a's replayed margins summed over S. An opening is *decided* on S
// when L_{a*}(S) ≥ U_b(S) for every rival b — exactly branch-and-bound
// over the info-set tree, idea page §3 rung 3, with refinement along the
// §8 feature coordinates. U_a − V_a (exact) is also the clairvoyance
// premium of opening `a` — the strategy-fusion gap a PIMC-style sampler
// would silently pocket.
// ---------------------------------------------------------------------------

/// Perfect-information best margin against σ, viewer's first move
/// optionally pinned (≤ 5! viewer action sequences at boundary 2 — plain
/// DFS, no memo).
fn clairvoyant_margin(
    algebra: &DeclarationAlgebra,
    sim: &Sim,
    viewer: Seat,
    pending_first: Option<DominoId>,
) -> i64 {
    if sim.hands.iter().all(DominoSet::is_empty) {
        return sim.banked[viewer.team().index()] as i64
            - sim.banked[viewer.team().opponent().index()] as i64;
    }
    let actor = sim.actor();
    if actor == viewer {
        let hand = &sim.hands[viewer.index()];
        let legal: Vec<DominoId> = if sim.trick.is_empty() {
            hand.iter().collect()
        } else {
            let q = algebra.led_suit(sim.trick[0].domino);
            let followers: Vec<DominoId> = hand.iter().filter(|&d| algebra.follows(d, q)).collect();
            if followers.is_empty() {
                hand.iter().collect()
            } else {
                followers
            }
        };
        let choices: Vec<DominoId> = match pending_first {
            Some(f) => {
                assert!(legal.contains(&f), "pinned first move is legal");
                vec![f]
            }
            None => legal,
        };
        choices
            .into_iter()
            .map(|d| {
                let mut next = Sim {
                    hands: sim.hands,
                    leader: sim.leader,
                    trick: sim.trick.clone(),
                    banked: sim.banked,
                };
                next.apply(algebra, d);
                clairvoyant_margin(algebra, &next, viewer, None)
            })
            .max()
            .expect("nonempty legal set")
    } else {
        let choice = greedy_sigma(algebra, &sim.hands[actor.index()], &sim.trick);
        let mut next = Sim {
            hands: sim.hands,
            leader: sim.leader,
            trick: sim.trick.clone(),
            banked: sim.banked,
        };
        next.apply(algebra, choice);
        clairvoyant_margin(algebra, &next, viewer, None)
    }
}

fn probe_bounds(boundary: usize, index: u64) {
    let state: MechanicalState = boundary_position(index, boundary);
    let viewer = state.viewer();
    let cells = derive_rule_cells(&state);
    let worlds = cells.fiber_worlds();
    let algebra = algebra_for(state.contract().declaration());
    let live_count: Vec<DominoId> = count_tiles()
        .into_iter()
        .filter(|d| cells.unseen_pool().contains(*d))
        .collect();
    let world_key = |w: &rob_core::RemainderWorld| -> Vec<u8> {
        live_count
            .iter()
            .map(|&d| {
                w.hidden_hands
                    .iter()
                    .position(|h| h.contains(d))
                    .expect("a pool tile is in some hidden hand") as u8
            })
            .collect()
    };
    let legal: Vec<DominoId> = state.own_remaining_hand().iter().collect();
    let mut plans = Vec::new();
    for &opening in &legal {
        let plan = solve_opening(&state, UtilityLens::Points, opening);
        if plan.truncated {
            println!("boundary {boundary} index {index}: truncated plan — skipping");
            return;
        }
        plans.push(plan);
    }
    let global = (0..plans.len())
        .max_by_key(|&a| {
            (
                plans[a].root.value_total,
                std::cmp::Reverse(legal[a].index()),
            )
        })
        .expect("nonempty legal");

    // Per world × opening: L (plan replay) and U (clairvoyant, pinned).
    let sim_for = |world: &rob_core::RemainderWorld| -> Sim {
        let mut hands = [DominoSet::empty(); 4];
        hands[viewer.index()] = *state.own_remaining_hand();
        for (i, &seat) in state.hidden_seats().iter().enumerate() {
            hands[seat.index()] = world.hidden_hands[i];
        }
        Sim {
            hands,
            leader: state.leader(),
            trick: state.current_trick().to_vec(),
            banked: state.hand_points(),
        }
    };
    let mut upper_total = vec![0i64; plans.len()];
    let mut cell_l: BTreeMap<Vec<u8>, i64> = BTreeMap::new();
    let mut cell_u: BTreeMap<Vec<u8>, Vec<i64>> = BTreeMap::new();
    let mut cell_ns: BTreeMap<Vec<u8>, u64> = BTreeMap::new();
    for world in &worlds {
        let key = world_key(world);
        *cell_ns.entry(key.clone()).or_insert(0) += 1;
        let lm = replay(&algebra, &plans[global].root, sim_for(world), viewer);
        *cell_l.entry(key.clone()).or_insert(0) += lm;
        let us = cell_u.entry(key).or_insert_with(|| vec![0i64; plans.len()]);
        for (a, &opening) in legal.iter().enumerate() {
            let u = clairvoyant_margin(&algebra, &sim_for(world), viewer, Some(opening));
            us[a] += u;
            upper_total[a] += u;
            debug_assert!(u >= lm || a != global);
        }
    }

    // Soundness: U_a ≥ V_a for every opening (per-world max ≥ any plan).
    for a in 0..plans.len() {
        assert!(
            upper_total[a] >= plans[a].root.value_total,
            "clairvoyant sum bounds the exact plan value"
        );
    }

    // Root-level: do bounds alone eliminate every rival of the exact best?
    let rivals_closed = (0..plans.len())
        .filter(|&b| b != global)
        .all(|b| upper_total[b] < plans[global].root.value_total);
    println!(
        "boundary {boundary} index {index}: fiber {} | exact best {:?} value {} | clairvoyance premium {} ({}‰ of |V|) | root decided by bounds: {}",
        worlds.len(),
        legal[global],
        plans[global].root.value_total,
        upper_total[global] - plans[global].root.value_total,
        permille(
            upper_total[global] - plans[global].root.value_total,
            plans[global].root.value_total.abs().max(1)
        ),
        rivals_closed,
    );
    for (a, &opening) in legal.iter().enumerate() {
        if a != global {
            println!(
                "    rival {:?}: U {} vs V* {} → {}",
                opening,
                upper_total[a],
                plans[global].root.value_total,
                if upper_total[a] < plans[global].root.value_total {
                    "eliminated by bound"
                } else {
                    "needs refinement"
                }
            );
        }
    }

    // Cell-level: worlds in cells where the incumbent's L beats every
    // rival's U — those cells never need an exact cell solve.
    let (mut decided_cells, mut decided_worlds) = (0u64, 0u64);
    for (key, l) in &cell_l {
        let us = &cell_u[key];
        let decided = (0..plans.len())
            .filter(|&b| b != global)
            .all(|b| us[b] <= *l);
        if decided {
            decided_cells += 1;
            decided_worlds += cell_ns[key];
        }
    }
    println!(
        "    cells (count-location): {} of {} decided by bounds | {} of {} worlds ({}‰)",
        decided_cells,
        cell_l.len(),
        decided_worlds,
        worlds.len(),
        permille(decided_worlds as i64, worlds.len() as i64),
    );
}

/// Rung-3 bounds at boundary 3: root elimination + decided-cell shares.
/// Run: `cargo test --release --test fiber_factor_probe bound_cover -- --ignored --nocapture`
#[test]
#[ignore]
fn bound_cover_boundary3() {
    for index in 0..12u64 {
        probe_bounds(3, index);
    }
}

/// Rung-3 bounds at the trick-3 wall (heavier: per-world clairvoyant DFS
/// across every opening — index 1's 72,072-world fiber; index 2's 324k
/// would run hours at this DFS cost).
#[test]
#[ignore]
fn bound_cover_boundary2() {
    probe_bounds(2, 1);
}
