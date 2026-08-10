//! H -- the actual hidden-information treatment (v0.4 §10.3): the viewer
//! sees only the original information record, so every choice below the root
//! is made once per pooled information state against the whole fiber, under
//! the fixed uniform-random legal field and the uniform fiber belief.
//!
//! The solve is exact symbolic backward induction on the observation tree
//! (`info::walk` with the full legal set expanded at every focal state).
//! It is exact because only the viewer optimizes and the viewer has perfect
//! recall: the canonical partition is a tree, so the pooled maximum
//! decomposes state by state and §7.5's finite policy maximum is attained --
//! per lambda, hence as the upper PWL envelope on the whole ray.
//!
//! This operator supersedes S2's `fixed_field_root_lines`, which refused any
//! kernel with a post-root focal choice; on such degenerate kernels the two
//! agree (every envelope is one line) and the S2 function is deleted.

use walt_core::{Domino, Team};
use walt_geom::{q, Envelope};
use walt_kernel::Kernel;

use crate::direction::Direction;
use crate::info::{root_bag, root_tiles, walk, WalkCtx};

/// `Q^H(a; lambda)` for every root action of the viewer, in ascending domino
/// order: the exact hidden-treatment action values.
pub fn hidden_root_values(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
) -> Vec<(Domino, Envelope)> {
    let ctx = WalkCtx::new(kernel, focal, dir);
    let n = i128::try_from(kernel.count()).expect("fiber sizes fit i128");
    kernel
        .viewer_hand()
        .iter()
        .map(|a| {
            let bag = root_bag(kernel, a);
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
            (a, env.scale(q(1, n)))
        })
        .collect()
}
