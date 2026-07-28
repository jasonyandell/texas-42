//! The materialized whole-hand contingent plan ρ
//! (BRIEF_PLAYER_01 §7 plan; INV-P1/P2/P5/P7).
//!
//! A plan is rob's decision variable made concrete (PLAN-NOT-TILE): one
//! action per viewer information set within the exact window, keyed by the
//! observation sequence that distinguishes the info sets. The tile rob puts
//! on the table is the root action — page one of the contingency book. No
//! item in this module (or anywhere in the workspace) exposes a scalar
//! "value of a domino"; per-node totals are values *of the plan* at that
//! info set, and any per-tile display number is a projection produced by the
//! trace layer alone (INV-P1).
//!
//! ```compile_fail
//! // INV-P1: no per-domino scalar value API exists anywhere in the player.
//! let id = rob_core::all_ids().next().unwrap();
//! let _ = rob_player::tile_value(id);
//! ```

use std::collections::BTreeMap;

use rob_core::{DominoId, Seat};

/// One observation step: `(seat index, domino index)` for a single play by
/// a non-viewer seat, in the order the viewer sees them.
pub type ObservationStep = (u8, u8);

/// The observation sequence strictly between two consecutive viewer
/// decisions — possibly empty (the viewer wins a trick and immediately
/// leads). Distinct sequences key distinct information sets (CELL-07 made
/// concrete), so the plan's ordered map enforces INV-P2 by construction:
/// one info set, one action.
pub type Observation = Vec<ObservationStep>;

/// The frontier leaf — one typed variant, no parameters, no tunable terms
/// (INV-P7 FRONTIER-LEAF-IS-LAW). A truncated window's frontier pays out
/// the banked team points of the interrupted hand and nothing else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrontierLeaf {
    /// Banked team points at the frontier; the only leaf the law permits.
    BankedPoints,
}

/// How a plan path ends inside the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LeafKind {
    /// The hand settled inside the window: the value is a settled hand's
    /// exact integer outcome (42-point conservation asserted, INV-P6).
    Settled,
    /// The window frontier: the typed non-tunable leaf (INV-P7).
    Frontier(FrontierLeaf),
}

/// A terminal plan entry with its bundle audit data.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanLeaf {
    /// Settled or frontier.
    pub kind: LeafKind,
    /// Exact number of fiber worlds pooled at this leaf (INV-P5 audit).
    pub world_count: u64,
    /// Exact integer utility total over those worlds under the solve lens.
    pub value_total: i64,
}

/// A child of a decision node: another decision, or a leaf.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PlanChild {
    /// The next viewer decision along this observation branch.
    Node(PlanNode),
    /// The branch ends inside the window.
    Leaf(PlanLeaf),
}

impl PlanChild {
    /// The exact world count pooled under this child (INV-P5).
    pub fn world_count(&self) -> u64 {
        match self {
            PlanChild::Node(n) => n.world_count,
            PlanChild::Leaf(l) => l.world_count,
        }
    }

    /// The exact value total under this child.
    pub fn value_total(&self) -> i64 {
        match self {
            PlanChild::Node(n) => n.value_total,
            PlanChild::Leaf(l) => l.value_total,
        }
    }
}

/// One viewer decision in the plan: the chosen action and one child per
/// distinct observation some fiber world actually produces under σ
/// (INV-P2: the ordered map admits exactly one action per info set).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlanNode {
    /// The plan's action at this information set.
    pub action: DominoId,
    /// Exact number of fiber worlds consistent with this info set.
    pub world_count: u64,
    /// Exact integer utility total of the plan over those worlds.
    pub value_total: i64,
    /// One branch per realizable observation. Empty exactly when every
    /// continuation was pooled into leaves elsewhere or materialization was
    /// truncated at this node (`Plan::truncated`).
    pub children: BTreeMap<Observation, PlanChild>,
}

/// A materialized solve result: the whole-hand contingent plan within the
/// exact window (BRIEF_PLAYER_01 §7). `decide` surfaces exactly this type —
/// callers take `.root.action` off the top; there is no scored-tile API
/// (INV-P1).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Plan {
    /// The deciding viewer seat.
    pub viewer: Seat,
    /// The normative window depth `H` this plan was solved to (INV-P6).
    pub window: usize,
    /// The exact fiber count at the root (capacity DP, CELL-10).
    pub fiber_count: u64,
    /// True when materialization was depth-truncated at the node cap
    /// (values remain exact; display data only — BRIEF_PLAYER_01 §8 P5).
    pub truncated: bool,
    /// The root decision.
    pub root: PlanNode,
}

/// The materialization node cap (BRIEF_PLAYER_01 §8 P5): plans are fully
/// materialized up to `2²⁰` nodes; beyond it, deeper children are dropped
/// with `truncated` set.
pub const MATERIALIZATION_CAP: usize = 1 << 20;

impl Plan {
    /// Total materialized node count (decision nodes plus leaves).
    pub fn node_count(&self) -> usize {
        fn walk(node: &PlanNode) -> usize {
            1 + node
                .children
                .values()
                .map(|c| match c {
                    PlanChild::Node(n) => walk(n),
                    PlanChild::Leaf(_) => 1,
                })
                .sum::<usize>()
        }
        walk(&self.root)
    }

    /// INV-P5 audit walk: at every materialized decision node with
    /// children, the child bundles partition the node's bundle — world
    /// counts and value totals both sum exactly. Returns the number of
    /// nodes checked. Panics on any violation.
    pub fn assert_conservation(&self) -> u64 {
        fn walk(node: &PlanNode, truncated: bool) -> u64 {
            let mut checked = 1u64;
            if !node.children.is_empty() {
                let worlds: u64 = node.children.values().map(PlanChild::world_count).sum();
                let values: i64 = node.children.values().map(PlanChild::value_total).sum();
                assert_eq!(worlds, node.world_count, "bundle partition (INV-P5)");
                assert_eq!(values, node.value_total, "value conservation (INV-P5)");
            } else {
                assert!(
                    truncated,
                    "an untruncated decision node pools its worlds in children"
                );
            }
            for child in node.children.values() {
                if let PlanChild::Node(n) = child {
                    checked += walk(n, truncated);
                }
            }
            checked
        }
        assert_eq!(self.root.world_count, self.fiber_count, "root = fiber");
        walk(&self.root, self.truncated)
    }
}
