//! C and F -- the revealed treatments of v0.4 §10.3.
//!
//! C (common root, continuation revealed): the root is chosen on the pooled
//! root information state, then the world is revealed before any later focal
//! decision, so the continuation is optimized independently per world and
//! `P_a^C` is the weighted Minkowski sum of the per-world polytopes. F (world
//! revealed before the root): the root may also depend on the world. Both are
//! computed at the support level along the direction ray (finite-first, §9.2):
//! the support of a weighted Minkowski sum is the weighted sum of supports,
//! and the support of `conv` of a union is the pointwise maximum -- so no
//! polytope is ever materialized.
//!
//! §10.8 caution: these keep the *fixed stochastic field* for the other three
//! seats and change only the focal information. Per-world perfect-information
//! minimax (the PI operator, `pi.rs`) changes both the information and the
//! continuation operator; C is the controlled causal comparison for
//! information value, and the two must never be conflated.
//!
//! Budgeted (freeze 44): `revealed_summary` takes ONE budget for the whole
//! call -- all worlds and all root actions, decremented monotonically across
//! the world loop (outer) and the action loop (inner) -- never per-world and
//! never per-action (a per-world budget would let a call exhaust the budget
//! |X| times over). On exhaustion ALL partial state is discarded: no partial
//! `q_c`, no partial `v_f`, no retained per-world envelope. The reason is
//! (C2) of the errata §3.4: a partial fiber sum is a sum over a proper
//! sub-multiset of worlds -- it is not U_a, it is not a bound on U_a in
//! either direction, and it may not be printed as one, scaled into one, or
//! carried forward. The stop returns the action and world index reached,
//! which are counts of the run, never statements about the coordinate.

use crate::geom::{q, qi, Envelope};
use crate::kernel::{Kernel, World};
use crate::rules::{Domino, Team};

use crate::strat::direction::Direction;
use crate::strat::info::{root_tiles, walk, Particle, WalkCtx};

/// Where a budget-exhausted `revealed_summary` stopped: the root action and
/// world index reached in the frozen enumeration order (freeze 44(d)).
#[derive(Clone, Copy, Debug)]
pub struct RevealedStop {
    pub action: Domino,
    pub world_index: u128,
}

/// The revealed continuation values of one world: for each root action, the
/// viewer's optimal fixed-field continuation value with the world known --
/// the support data of `P_{omega,a}` along the direction ray. The walk is the
/// same observation-tree solve as H, on a one-world bag. Budgeted (freeze
/// 44): the budget is shared across this world's per-action walks; `None` on
/// exhaustion with nothing partial.
pub fn revealed_world_root_values(
    kernel: &Kernel,
    world: &World,
    focal: Team,
    dir: &Direction,
    budget: &mut u64,
) -> Option<Vec<(Domino, Envelope)>> {
    let ctx = WalkCtx::new(kernel, focal, dir);
    let mut out = Vec::new();
    let cell = std::cell::Cell::new(*budget);
    for a in kernel.viewer_hand().iter() {
        let mut hands = world.hands();
        hands[ctx.viewer.index()].remove(a);
        let bag = [Particle {
            hands,
            weight: qi(1),
        }];
        let mut obs = vec![a];
        let env = walk(
            &ctx,
            &bag,
            ctx.viewer,
            root_tiles(a),
            1,
            &mut obs,
            &mut |_, legal, _| legal,
            &cell,
        );
        *budget = cell.get();
        out.push((a, env?));
    }
    Some(out)
}

/// Both revealed treatments, aggregated over the fiber in one pass.
pub struct RevealedSummary {
    /// `Q^C(a; lambda)` per root, ascending domino order: the fiber-weighted
    /// sums of per-world revealed values (§10.3's Minkowski formula at the
    /// support level).
    pub q_c: Vec<(Domino, Envelope)>,
    /// `V^F(lambda)`: the world average of the per-world root-revealed
    /// optimum `max_a`.
    pub v_f: Envelope,
    /// Per-action walk-step subtotals, exact integers -- an exact
    /// computational observable of the declared traversal in SEP-A19(b)'s
    /// class: never an information value, a decision width, a cost claim, or
    /// a term in the DS-A2 ladder.
    pub action_steps: Vec<(Domino, u64)>,
}

/// Solves every world once and aggregates C and F under the uniform fiber
/// belief. One whole-call budget (freeze 44(d)); on exhaustion `stop`
/// carries the action and world index reached and the return is `None` with
/// all partial state discarded.
pub fn revealed_summary(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    budget: &mut u64,
    stop: &mut Option<RevealedStop>,
) -> Option<RevealedSummary> {
    let ctx = WalkCtx::new(kernel, focal, dir);
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    let mut per_root: Vec<Vec<Envelope>> = vec![Vec::new(); actions.len()];
    let mut steps: Vec<u64> = vec![0; actions.len()];
    let mut f_terms: Vec<Envelope> = Vec::new();
    let mut worlds: u128 = 0;
    for world in kernel.worlds() {
        let mut solved: Vec<Envelope> = Vec::with_capacity(actions.len());
        for (i, a) in actions.iter().enumerate() {
            let mut hands = world.hands();
            hands[ctx.viewer.index()].remove(*a);
            let bag = [Particle {
                hands,
                weight: qi(1),
            }];
            let mut obs = vec![*a];
            let before = *budget;
            let cell = std::cell::Cell::new(*budget);
            let env = walk(
                &ctx,
                &bag,
                ctx.viewer,
                root_tiles(*a),
                1,
                &mut obs,
                &mut |_, legal, _| legal,
                &cell,
            );
            *budget = cell.get();
            let Some(env) = env else {
                *stop = Some(RevealedStop {
                    action: *a,
                    world_index: worlds,
                });
                return None;
            };
            steps[i] += before - *budget;
            solved.push(env);
        }
        f_terms.push(Envelope::max_of(solved.iter().cloned()));
        for (slot, e) in per_root.iter_mut().zip(solved) {
            slot.push(e);
        }
        worlds += 1;
    }
    assert_eq!(worlds, kernel.count(), "enumeration count drift");
    let n = i128::try_from(worlds).expect("fiber sizes fit i128");
    let q_c = actions
        .iter()
        .zip(per_root)
        .map(|(a, envs)| (*a, Envelope::sum_of(envs).scale(q(1, n))))
        .collect();
    let v_f = Envelope::sum_of(f_terms).scale(q(1, n));
    let action_steps = actions.iter().copied().zip(steps).collect();
    Some(RevealedSummary {
        q_c,
        v_f,
        action_steps,
    })
}
