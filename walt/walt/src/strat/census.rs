//! The worldwise perfect-information response census of v0.4 §14.2: solve
//! every world of a kernel fiber with the PI operator and partition the fiber
//! three ways -- by the full parametric root-Q vector, by its lambda = 0
//! restriction, and by the optimal-action correspondence.
//!
//! Every class map is a derived view of the fiber and the direction; nothing
//! here is an authority.

use std::collections::BTreeMap;

use crate::geom::{argmax_correspondence, ArgmaxCorrespondence, Envelope, Q};
use crate::kernel::Kernel;
use crate::rules::{Domino, Team};

use crate::strat::direction::Direction;
use crate::strat::pi::pi_root_values;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PiCensus {
    /// Worlds solved: the fiber size.
    pub worlds: usize,
    /// The root actions, ascending; identical in every world of the fiber
    /// because they are the viewer's own hand.
    pub actions: Vec<Domino>,
    /// Worlds by full parametric root-Q vector, in action order.
    pub parametric: BTreeMap<Vec<Envelope>, usize>,
    /// Worlds by the lambda = 0 root-value vector.
    pub baseline: BTreeMap<Vec<Q>, usize>,
    /// Worlds by optimal-action correspondence on the whole ray.
    pub action: BTreeMap<ArgmaxCorrespondence, usize>,
    /// Worlds by the lambda = 0 optimal-action set (bitmask in action order).
    pub baseline_action: BTreeMap<u32, usize>,
    /// World/root curves seen, and how many were not a single line.
    pub curves: usize,
    pub multisegment_curves: usize,
}

impl PiCensus {
    /// Class sizes, largest first, for pinning against reported censuses.
    pub fn parametric_sizes(&self) -> Vec<usize> {
        let mut out: Vec<usize> = self.parametric.values().copied().collect();
        out.sort_unstable_by(|a, b| b.cmp(a));
        out
    }
}

/// Solves the whole fiber worldwise under perfect information.
pub fn pi_census(kernel: &Kernel, focal: Team, dir: &Direction) -> PiCensus {
    let actions: Vec<Domino> = kernel.viewer_hand().iter().collect();
    let mut census = PiCensus {
        worlds: 0,
        actions: actions.clone(),
        parametric: BTreeMap::new(),
        baseline: BTreeMap::new(),
        action: BTreeMap::new(),
        baseline_action: BTreeMap::new(),
        curves: 0,
        multisegment_curves: 0,
    };
    for world in kernel.worlds() {
        let solved = pi_root_values(kernel.decl(), world.hands(), kernel.viewer(), focal, dir);
        let (roots, envelopes): (Vec<Domino>, Vec<Envelope>) = solved.into_iter().unzip();
        assert_eq!(roots, actions, "the root actions are the viewer's hand");

        census.worlds += 1;
        census.curves += envelopes.len();
        census.multisegment_curves += envelopes.iter().filter(|e| !e.is_affine()).count();

        let baseline: Vec<Q> = envelopes
            .iter()
            .map(|e| e.eval(crate::geom::qi(0)))
            .collect();
        let correspondence = argmax_correspondence(&envelopes);
        *census
            .baseline_action
            .entry(correspondence.baseline())
            .or_insert(0) += 1;
        *census.action.entry(correspondence).or_insert(0) += 1;
        *census.baseline.entry(baseline).or_insert(0) += 1;
        *census.parametric.entry(envelopes).or_insert(0) += 1;
    }
    census
}
