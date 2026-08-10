//! Decision nodes, the canonical information partition, and
//! information-consistent policies (v0.4 §10.1, §7.2), plus the shared
//! observation-tree traversal every fixed-field treatment (§10.3) is built on.
//!
//! A decision node is a concrete (world, history) pair over the fiber
//! (§10.1); the focal viewer cannot see the world, so nodes sharing one
//! observation record pool into one information state. Pooling by the full
//! record is the canonical perfect-recall partition -- the finest information
//! structure the viewer can actually have. §10.1 validity (pooled nodes share
//! the acting player and the legal action labels) holds by construction:
//! the viewer's legal set is a function of its own remaining hand and the led
//! context, both part of the observation record. Coarser gluings of §10.1
//! (and the §10.2 inclusion they induce) are not implemented here; nothing in
//! S3 consumes them, and a non-perfect-recall gluing breaks the backward
//! induction this module's solvers rely on, so they must arrive with their
//! own solver, not as a parameter to this one.
//!
//! Information honesty by type: a `Policy` maps `InfoStateId`s to actions.
//! An id names an observation record and nothing else -- hidden hands are not
//! reachable from it, so a policy that peeks at worlds is unconstructible
//! (§7.2: a policy may branch on observations received, never on the hidden
//! world).

use std::collections::BTreeMap;

use walt_core::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::{q, qi, Envelope, Line, Q};
use walt_kernel::Kernel;

use crate::direction::Direction;

/// One decision node in flight: a concrete world's remaining hands and the
/// exact probability the field has assigned its history so far. Constructed
/// only inside this crate; seat-facing types never see one.
#[derive(Clone, Debug)]
pub(crate) struct Particle {
    pub hands: [DominoSet; Seat::COUNT],
    pub weight: Q,
}

pub(crate) struct WalkCtx<'a> {
    pub decl: Decl,
    pub viewer: Seat,
    pub focal: Team,
    pub dir: &'a Direction,
}

impl<'a> WalkCtx<'a> {
    pub fn new(kernel: &Kernel, focal: Team, dir: &'a Direction) -> WalkCtx<'a> {
        WalkCtx {
            decl: kernel.decl(),
            viewer: kernel.viewer(),
            focal,
            dir,
        }
    }
}

/// The uniform-belief root bag after the viewer's root action: every fiber
/// world with weight one (callers normalize by the fiber size).
pub(crate) fn root_bag(kernel: &Kernel, root: Domino) -> Vec<Particle> {
    assert!(
        kernel.viewer_hand().contains(root),
        "a root action is a viewer tile"
    );
    let mut bag = Vec::new();
    for w in kernel.worlds() {
        let mut hands = w.hands();
        hands[kernel.viewer().index()].remove(root);
        bag.push(Particle {
            hands,
            weight: qi(1),
        });
    }
    assert_eq!(bag.len() as u128, kernel.count(), "enumeration count drift");
    bag
}

pub(crate) fn root_tiles(root: Domino) -> [Domino; 4] {
    let mut tiles = [Domino::ALL[0]; 4];
    tiles[0] = root;
    tiles
}

/// The observation-tree traversal shared by the H and revealed treatments:
/// field seats move as chance -- uniform over their world-relative legal set,
/// the observed tile filtering the bag and charging each surviving particle
/// `1/|legal|` -- the viewer moves at pooled information states, and a
/// resolved trick contributes its observable increment scaled by the bag
/// mass. Returns the unnormalized weighted expectation `sum_p w_p E_p`.
///
/// Viewer choices are delegated to `expand`: at each future focal information
/// state it receives the observation record (all plays since the kernel
/// decision point, root first), the legal set, and the pooled node count, and
/// returns the nonempty legal subset to consider; the walk folds the chosen
/// children with the pointwise maximum. Expanding the full legal set solves
/// the treatment; expanding a singleton evaluates a policy. Each observation
/// record is visited at most once per walk, so the traversal *is* the
/// canonical information-state tree and pooled maximization decomposes state
/// by state (perfect recall; §7.5's finite policy maximum is attained).
pub(crate) fn walk(
    ctx: &WalkCtx,
    bag: &[Particle],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
    obs: &mut Vec<Domino>,
    expand: &mut impl FnMut(&[Domino], DominoSet, usize) -> DominoSet,
) -> Envelope {
    if k == 4 {
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(ctx.decl);
        let mass: Q = bag.iter().map(|p| p.weight).sum();
        let inc = ctx.dir.trick_line(ctx.focal, winner, tiles).scale(mass);
        if bag[0].hands.iter().all(|h| h.is_empty()) {
            return Envelope::line(inc);
        }
        return walk(ctx, bag, winner, [Domino::ALL[0]; 4], 0, obs, expand).add_line(&inc);
    }
    let seat = leader.plus(k);
    let led = (k > 0).then(|| ctx.decl.led_context(tiles[0]));
    if seat == ctx.viewer {
        let hand = bag[0].hands[seat.index()];
        debug_assert!(
            bag.iter().all(|p| p.hands[seat.index()] == hand),
            "the viewer's hand is observable, hence constant on the bag"
        );
        let legal = legal_plays(ctx.decl, hand, led);
        let chosen = expand(obs, legal, bag.len());
        assert!(
            !chosen.is_empty() && chosen.is_subset_of(legal),
            "an information-consistent choice is a nonempty legal subset"
        );
        return Envelope::max_of(chosen.iter().map(|d| {
            let bag: Vec<Particle> = bag
                .iter()
                .map(|p| {
                    let mut p = p.clone();
                    p.hands[seat.index()].remove(d);
                    p
                })
                .collect();
            let mut tiles = tiles;
            tiles[k] = d;
            obs.push(d);
            let out = walk(ctx, &bag, leader, tiles, k + 1, obs, expand);
            obs.pop();
            out
        }));
    }
    // A field seat: chance over its world-relative legal set.
    let legals: Vec<DominoSet> = bag
        .iter()
        .map(|p| legal_plays(ctx.decl, p.hands[seat.index()], led))
        .collect();
    let mut union = DominoSet::EMPTY;
    for l in &legals {
        union = union.union(*l);
    }
    let mut sum = Envelope::zero();
    for d in union.iter() {
        let child: Vec<Particle> = bag
            .iter()
            .zip(&legals)
            .filter(|(_, legal)| legal.contains(d))
            .map(|(p, legal)| {
                let mut p = p.clone();
                p.hands[seat.index()].remove(d);
                p.weight *= q(1, legal.len() as i128);
                p
            })
            .collect();
        let mut tiles = tiles;
        tiles[k] = d;
        obs.push(d);
        sum = sum.add(&walk(ctx, &child, leader, tiles, k + 1, obs, expand));
        obs.pop();
    }
    sum
}

/// An opaque name for one future focal information state of one
/// `InfoPartition`. Carries no world content by construction.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InfoStateId(u32);

/// The canonical perfect-recall information partition of the future focal
/// decision nodes after one root action (§10.1; §10.9's reachable
/// information-state set `I^m_{B,a}`). A derived view of the kernel: nothing
/// here is an authority over the fiber.
pub struct InfoPartition {
    root: Domino,
    index: BTreeMap<Vec<Domino>, InfoStateId>,
    legal: Vec<DominoSet>,
    /// Decision nodes pooled into each state: the worlds consistent with its
    /// observation record.
    nodes: Vec<usize>,
}

impl InfoPartition {
    /// Enumerates the reachable future focal information states after `root`
    /// by one full traversal. The direction is irrelevant to the tree shape,
    /// so the bare trick differential is used internally.
    pub fn build(kernel: &Kernel, root: Domino) -> InfoPartition {
        let dir = Direction::trick_diff();
        let ctx = WalkCtx::new(kernel, kernel.viewer().team(), &dir);
        let bag = root_bag(kernel, root);
        let mut index: BTreeMap<Vec<Domino>, InfoStateId> = BTreeMap::new();
        let mut legal_by_id: Vec<DominoSet> = Vec::new();
        let mut nodes_by_id: Vec<usize> = Vec::new();
        let mut obs = vec![root];
        walk(
            &ctx,
            &bag,
            ctx.viewer,
            root_tiles(root),
            1,
            &mut obs,
            &mut |record, legal, nodes| {
                let id = InfoStateId(u32::try_from(legal_by_id.len()).expect("state count"));
                let prev = index.insert(record.to_vec(), id);
                assert!(prev.is_none(), "each observation record is reached once");
                legal_by_id.push(legal);
                nodes_by_id.push(nodes);
                legal
            },
        );
        InfoPartition {
            root,
            index,
            legal: legal_by_id,
            nodes: nodes_by_id,
        }
    }

    pub const fn root(&self) -> Domino {
        self.root
    }

    /// `E_B(a)` of §10.9: the number of reachable future focal information
    /// states. An exact computational observable, not an information value.
    pub fn len(&self) -> usize {
        self.legal.len()
    }

    pub fn is_empty(&self) -> bool {
        self.legal.is_empty()
    }

    /// The states at which the viewer has a genuine choice.
    pub fn choice_states(&self) -> usize {
        self.legal.iter().filter(|l| l.len() > 1).count()
    }

    pub fn ids(&self) -> impl Iterator<Item = InfoStateId> + '_ {
        (0..u32::try_from(self.legal.len()).expect("state count")).map(InfoStateId)
    }

    /// The id of the state with this observation record, if reachable.
    pub fn id(&self, record: &[Domino]) -> Option<InfoStateId> {
        self.index.get(record).copied()
    }

    /// The legal action labels, shared by every pooled node (§10.1 validity).
    pub fn legal(&self, id: InfoStateId) -> DominoSet {
        self.legal[id.0 as usize]
    }

    /// How many decision nodes (fiber worlds) the state pools.
    pub fn pooled_nodes(&self, id: InfoStateId) -> usize {
        self.nodes[id.0 as usize]
    }
}

/// A deterministic information-consistent policy for one root action: one
/// legal choice per information state (§7.2). The domain is `InfoStateId`,
/// so branching on hidden worlds is unconstructible by type.
pub struct Policy {
    root: Domino,
    choices: Vec<Domino>,
}

impl Policy {
    /// Builds a policy by choosing at every state of the partition; the
    /// chooser sees only the id and the legal labels.
    pub fn build(
        partition: &InfoPartition,
        mut choose: impl FnMut(InfoStateId, DominoSet) -> Domino,
    ) -> Policy {
        let choices = partition
            .ids()
            .map(|id| {
                let legal = partition.legal(id);
                let d = choose(id, legal);
                assert!(legal.contains(d), "a policy picks a legal action");
                d
            })
            .collect();
        Policy {
            root: partition.root(),
            choices,
        }
    }

    pub const fn root(&self) -> Domino {
        self.root
    }

    pub fn action(&self, id: InfoStateId) -> Domino {
        self.choices[id.0 as usize]
    }
}

/// The exact expected value of one deterministic information-consistent
/// policy under the uniform fiber belief and the uniform-random legal field:
/// a single affine line (§9.4 -- one policy contributes one line). This is
/// the no-maximization code path; the H operator must dominate it pointwise
/// and touch it wherever the policy is optimal.
pub fn policy_value(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    partition: &InfoPartition,
    policy: &Policy,
) -> Line {
    assert_eq!(
        partition.root(),
        policy.root(),
        "a policy is evaluated on its own root's partition"
    );
    let ctx = WalkCtx::new(kernel, focal, dir);
    let bag = root_bag(kernel, policy.root());
    let mut obs = vec![policy.root()];
    let env = walk(
        &ctx,
        &bag,
        ctx.viewer,
        root_tiles(policy.root()),
        1,
        &mut obs,
        &mut |record, _legal, _| {
            let id = partition
                .id(record)
                .expect("the partition covers every reachable state");
            DominoSet::single(policy.action(id))
        },
    );
    let n = i128::try_from(kernel.count()).expect("fiber sizes fit i128");
    let env = env.scale(q(1, n));
    assert!(env.is_affine(), "one deterministic policy induces one line");
    env.pieces()[0].line
}
