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

use walt_core::{Domino, Team};
use walt_geom::{q, qi, Envelope};
use walt_kernel::{Kernel, World};

use crate::direction::Direction;
use crate::info::{root_tiles, walk, Particle, WalkCtx};

/// The revealed continuation values of one world: for each root action, the
/// viewer's optimal fixed-field continuation value with the world known --
/// the support data of `P_{omega,a}` along the direction ray. The walk is the
/// same observation-tree solve as H, on a one-world bag.
pub fn revealed_world_root_values(
    kernel: &Kernel,
    world: &World,
    focal: Team,
    dir: &Direction,
) -> Vec<(Domino, Envelope)> {
    let ctx = WalkCtx::new(kernel, focal, dir);
    kernel
        .viewer_hand()
        .iter()
        .map(|a| {
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
            );
            (a, env)
        })
        .collect()
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
}

/// Solves every world once and aggregates C and F under the uniform fiber
/// belief.
pub fn revealed_summary(kernel: &Kernel, focal: Team, dir: &Direction) -> RevealedSummary {
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    let mut per_root: Vec<Vec<Envelope>> = vec![Vec::new(); actions.len()];
    let mut f_terms: Vec<Envelope> = Vec::new();
    let mut worlds: u128 = 0;
    for world in kernel.worlds() {
        let solved = revealed_world_root_values(kernel, &world, focal, dir);
        f_terms.push(Envelope::max_of(solved.iter().map(|(_, e)| e.clone())));
        for (i, (slot, (a, e))) in per_root.iter_mut().zip(solved).enumerate() {
            debug_assert_eq!(a, actions[i], "root actions are the viewer's hand");
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
    RevealedSummary { q_c, v_f }
}
