//! The exhaustive controlled lumpability checker (v0.4 §12.6).
//!
//! §12.6 fixes the exact dynamic compression target: a latent descriptor
//! `d: X -> Y` is strongly controlled-lumpable when, whenever `d(x) = d(y)`,
//! (1) the legal focal action sets agree, `A(x) = A(y)`, and (2) for every
//! legal action `a`, feature increment `r`, observation `o`, and descriptor
//! class `y'`, the class-aggregated kernel masses agree:
//! `sum_{x': d(x') = y'} K_a(x; r, o, x') = sum_{x': d(x') = y'} K_a(y; r, o, x')`.
//!
//! **Instantiation on a finite kernel** (the spec leaves the carrier to the
//! implementation; this is the honest walt reading, recorded here and in
//! PLAN.md):
//!
//! - the recursion boundary is the viewer's decision points, and `X` is the
//!   set of ALL reachable viewer-decision nodes -- (fiber world,
//!   observation record) pairs -- from every world of the fiber, every
//!   depth pooled into one finite carrier, plus one absorbing terminal
//!   state that is its own class;
//! - `A(x)` is the viewer's legal set at the node (a function of its
//!   remaining hand and the led context);
//! - the field is the fixed uniform-legal field of the §7.4 specialization:
//!   between two viewer decisions the hidden seats play chance moves with
//!   exact mass `prod 1/|legal|`;
//! - `o` is the observed field-play segment between the viewer's action and
//!   its next decision (the viewer's own play is the action, not part of
//!   `o`), and exactly one trick resolves inside each segment, so the
//!   universal feature increment `r` is that trick's signed scalar value
//!   (`ScalarValuation`, focal team = the viewer's);
//! - `d` on a node is the skeleton's root evaluation folded through the
//!   node's record (`fold_record`), so a transducer's classes are exactly
//!   what the seat could maintain.
//!
//! Exhaustive means exhaustive: every fiber world's every reachable node,
//! every legal action, every positive-mass field segment. Each kernel row
//! is asserted to be a probability law (masses sum to one exactly), and the
//! world count is asserted against the counting DP. Nothing is sampled;
//! there are no bounds.

use std::collections::BTreeMap;

use walt_core::{legal_plays, Domino, DominoSet, Seat, Trick};
use walt_geom::{q, qi, Q};
use walt_kernel::{Kernel, World};
use walt_strat::ScalarValuation;

use crate::obs::ObservedPlay;
use crate::skeleton::{fold_record, ControlSkeleton, UpdateKind};

/// One reachable viewer-decision node: one fiber world at one observation
/// record. A derived view of the kernel and the field law -- never an
/// authority.
#[derive(Clone, Debug)]
pub struct Node {
    /// Index of the world in `Kernel::worlds` enumeration order.
    pub world: usize,
    /// walt-strat's observation record at the node (all plays since the
    /// kernel decision point, viewer's included, in play order).
    pub record: Vec<Domino>,
    /// `A(x)`: the viewer's legal action labels.
    pub legal: DominoSet,
}

/// One concrete kernel outcome `K_a(x; r, o, x')` with positive mass.
#[derive(Clone, Debug)]
pub struct Entry {
    /// `r`: the resolved trick's signed increment under the tree's
    /// valuation (focal = the viewer's team).
    pub increment: i64,
    /// `o`: the field-play segment.
    pub obs: Vec<ObservedPlay>,
    /// The exact chance mass `prod 1/|legal|` of the segment.
    pub prob: Q,
    /// `x'`: the successor node, or `None` for the absorbing terminal.
    pub successor: Option<usize>,
}

/// The kernel row for one (node, action).
#[derive(Clone, Debug)]
pub struct Row {
    pub node: usize,
    pub action: Domino,
    pub entries: Vec<Entry>,
}

/// The fully enumerated controlled kernel of one finite walt kernel under
/// the fixed uniform-legal field: nodes, rows, and the valuation that names
/// the increment alphabet. Built once per kernel; every candidate skeleton
/// is then checked against the same recorded structure.
pub struct KernelTree {
    pub valuation: ScalarValuation,
    pub nodes: Vec<Node>,
    pub rows: Vec<Row>,
    rows_of: Vec<Vec<usize>>,
}

impl KernelTree {
    /// Enumerates the whole reachable carrier exhaustively.
    pub fn build(kernel: &Kernel, valuation: ScalarValuation) -> KernelTree {
        let mut tree = KernelTree {
            valuation,
            nodes: Vec::new(),
            rows: Vec::new(),
            rows_of: Vec::new(),
        };
        let mut worlds = 0usize;
        for world in kernel.worlds() {
            let mut b = Builder {
                kernel,
                valuation: &tree.valuation,
                world: worlds,
                nodes: &mut tree.nodes,
                rows: &mut tree.rows,
                rows_of: &mut tree.rows_of,
            };
            let mut record = Vec::new();
            b.node(
                world.hands(),
                kernel.viewer(),
                [Domino::ALL[0]; 4],
                0,
                &mut record,
            );
            worlds += 1;
        }
        assert_eq!(
            worlds as u128,
            kernel.count(),
            "the lumpability carrier covers the whole fiber"
        );
        tree
    }

    /// The rows of one node, in legal-action order.
    pub fn rows_of(&self, node: usize) -> &[usize] {
        &self.rows_of[node]
    }
}

struct Builder<'a> {
    kernel: &'a Kernel,
    valuation: &'a ScalarValuation,
    world: usize,
    nodes: &'a mut Vec<Node>,
    rows: &'a mut Vec<Row>,
    rows_of: &'a mut Vec<Vec<usize>>,
}

impl Builder<'_> {
    /// Registers the viewer-decision node at the current concrete state and
    /// expands every legal action's row. Returns the node index.
    fn node(
        &mut self,
        hands: [DominoSet; Seat::COUNT],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        record: &mut Vec<Domino>,
    ) -> usize {
        let viewer = self.kernel.viewer();
        let decl = self.kernel.decl();
        debug_assert_eq!(leader.plus(k), viewer, "a node is a viewer decision");
        let led = (k > 0).then(|| decl.led_context(tiles[0]));
        let legal = legal_plays(decl, hands[viewer.index()], led);
        assert!(!legal.is_empty(), "a seat holding tiles has a legal play");
        let node = self.nodes.len();
        self.nodes.push(Node {
            world: self.world,
            record: record.clone(),
            legal,
        });
        self.rows_of.push(Vec::new());
        for action in legal.iter() {
            let row = self.rows.len();
            self.rows.push(Row {
                node,
                action,
                entries: Vec::new(),
            });
            self.rows_of[node].push(row);
            let mut hands = hands;
            hands[viewer.index()].remove(action);
            let mut tiles = tiles;
            tiles[k] = action;
            record.push(action);
            let mut obs = Vec::new();
            self.segment(
                row,
                hands,
                leader,
                tiles,
                k + 1,
                record,
                &mut obs,
                qi(1),
                None,
            );
            record.pop();
            let mass: Q = self.rows[row].entries.iter().map(|e| e.prob).sum();
            assert_eq!(mass, qi(1), "a kernel row is a probability law");
        }
        node
    }

    /// Expands the chance segment after a viewer action: field seats play
    /// uniformly over their legal sets until the viewer's next decision or
    /// the terminal, resolving exactly one trick on the way.
    #[allow(clippy::too_many_arguments)]
    fn segment(
        &mut self,
        row: usize,
        hands: [DominoSet; Seat::COUNT],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        record: &mut Vec<Domino>,
        obs: &mut Vec<ObservedPlay>,
        prob: Q,
        increment: Option<i64>,
    ) {
        let viewer = self.kernel.viewer();
        let decl = self.kernel.decl();
        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            let winner = trick.winner(decl);
            let mut inc = self.valuation.trick;
            for d in tiles {
                inc += self.valuation.tiles[d.index()];
            }
            if winner.team() != viewer.team() {
                inc = -inc;
            }
            assert!(increment.is_none(), "one trick resolves per segment");
            if hands.iter().all(|h| h.is_empty()) {
                self.rows[row].entries.push(Entry {
                    increment: inc,
                    obs: obs.clone(),
                    prob,
                    successor: None,
                });
                return;
            }
            self.segment(
                row,
                hands,
                winner,
                [Domino::ALL[0]; 4],
                0,
                record,
                obs,
                prob,
                Some(inc),
            );
            return;
        }
        let seat = leader.plus(k);
        if seat == viewer {
            let increment =
                increment.expect("exactly one trick resolves before the viewer's next decision");
            let successor = self.node(hands, leader, tiles, k, record);
            self.rows[row].entries.push(Entry {
                increment,
                obs: obs.clone(),
                prob,
                successor: Some(successor),
            });
            return;
        }
        let led = (k > 0).then(|| decl.led_context(tiles[0]));
        let legal = legal_plays(decl, hands[seat.index()], led);
        assert!(!legal.is_empty(), "a seat holding tiles has a legal play");
        let share = q(1, legal.len() as i128);
        for tile in legal.iter() {
            let mut hands = hands;
            hands[seat.index()].remove(tile);
            let mut tiles = tiles;
            tiles[k] = tile;
            record.push(tile);
            obs.push(ObservedPlay { seat, tile });
            self.segment(
                row,
                hands,
                leader,
                tiles,
                k + 1,
                record,
                obs,
                prob * share,
                increment,
            );
            obs.pop();
            record.pop();
        }
    }
}

/// Why a descriptor failed §12.6, with the witnessing pair.
#[derive(Clone, Debug)]
pub enum LumpabilityFailure<St> {
    /// Condition 1: two nodes in one class with different legal sets.
    LegalSets {
        class: St,
        node_a: usize,
        node_b: usize,
    },
    /// Condition 2: a (action, increment, observation, successor-class)
    /// event with different aggregated masses at two nodes of one class.
    Kernel {
        class: St,
        node_a: usize,
        node_b: usize,
        action: Domino,
        increment: i64,
        obs: Vec<ObservedPlay>,
        successor: Option<St>,
        mass_a: Q,
        mass_b: Q,
    },
}

/// The exhaustive §12.6 verdict for one skeleton on one kernel tree.
#[derive(Clone, Debug)]
pub struct LumpabilityReport<St> {
    pub skeleton: String,
    pub kind: UpdateKind,
    /// `|X|`, terminal excluded.
    pub nodes: usize,
    /// `|im d|` on the nodes.
    pub classes: usize,
    /// The largest class size -- 1 means the descriptor separates
    /// everything (trivially lumpable).
    pub largest_class: usize,
    pub failure: Option<LumpabilityFailure<St>>,
}

impl<St> LumpabilityReport<St> {
    pub fn is_lumpable(&self) -> bool {
        self.failure.is_none()
    }

    /// How many nodes the quotient actually merged.
    pub fn merged_nodes(&self) -> usize {
        self.nodes - self.classes
    }

    /// Lumpable AND compressing: the reportable success shape. A verdict of
    /// "lumpable but trivial" (classes == nodes) is not a compression.
    pub fn is_nontrivially_lumpable(&self) -> bool {
        self.is_lumpable() && self.merged_nodes() > 0
    }
}

type EventKey<St> = (Domino, i64, Vec<ObservedPlay>, Option<St>);

/// Checks §12.6 strong controlled lumpability of a skeleton's labeling on a
/// fully enumerated kernel tree. The labeling of a node is `fold_record`
/// (root evaluation folded through the record), so what is checked is
/// exactly the state a seat running this transducer would hold there.
pub fn check_lumpability<S: ControlSkeleton>(
    kernel: &Kernel,
    tree: &KernelTree,
    skeleton: &S,
) -> LumpabilityReport<S::State> {
    let worlds: Vec<World> = kernel.worlds().collect();
    let states: Vec<S::State> = tree
        .nodes
        .iter()
        .map(|n| fold_record(skeleton, kernel, &worlds[n.world], &n.record))
        .collect();

    let mut classes: BTreeMap<&S::State, Vec<usize>> = BTreeMap::new();
    for (i, st) in states.iter().enumerate() {
        classes.entry(st).or_default().push(i);
    }

    let mut report = LumpabilityReport {
        skeleton: skeleton.name(),
        kind: skeleton.kind(),
        nodes: tree.nodes.len(),
        classes: classes.len(),
        largest_class: classes.values().map(Vec::len).max().unwrap_or(0),
        failure: None,
    };

    // The aggregated law `(a, r, o, y') -> mass` at one node.
    let events = |node: usize| -> BTreeMap<EventKey<S::State>, Q> {
        let mut out = BTreeMap::new();
        for &row in tree.rows_of(node) {
            let row = &tree.rows[row];
            for e in &row.entries {
                let key = (
                    row.action,
                    e.increment,
                    e.obs.clone(),
                    e.successor.map(|s| states[s].clone()),
                );
                *out.entry(key).or_insert_with(|| qi(0)) += e.prob;
            }
        }
        out
    };

    'classes: for (class, members) in &classes {
        if members.len() < 2 {
            continue;
        }
        let rep = members[0];
        // Condition 1: legal-set agreement.
        for &m in &members[1..] {
            if tree.nodes[m].legal != tree.nodes[rep].legal {
                report.failure = Some(LumpabilityFailure::LegalSets {
                    class: (*class).clone(),
                    node_a: rep,
                    node_b: m,
                });
                break 'classes;
            }
        }
        // Condition 2: class-aggregated kernel agreement.
        let rep_events = events(rep);
        for &m in &members[1..] {
            let m_events = events(m);
            let mut keys: Vec<&EventKey<S::State>> = rep_events.keys().collect();
            keys.extend(m_events.keys());
            for key in keys {
                let a = rep_events.get(key).copied().unwrap_or_else(|| qi(0));
                let b = m_events.get(key).copied().unwrap_or_else(|| qi(0));
                if a != b {
                    let (action, increment, obs, successor) = key.clone();
                    report.failure = Some(LumpabilityFailure::Kernel {
                        class: (*class).clone(),
                        node_a: rep,
                        node_b: m,
                        action,
                        increment,
                        obs,
                        successor,
                        mass_a: a,
                        mass_b: b,
                    });
                    break 'classes;
                }
            }
        }
    }
    report
}
