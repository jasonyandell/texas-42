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

use std::cell::Cell;
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
///
/// Budgeted (freeze 44): one walk-step per (particle, node) visit -- the
/// charge is `bag.len()`, taken at entry before any child call, the same
/// rule as `hidden_scalar`'s particle-step. On exhaustion the walk returns
/// `None` immediately and NO PARTIAL FOLD of any kind is retained: a
/// partially folded envelope is neither an upper nor a lower bound on the
/// true one ((C2) of the errata; PG-A13's asymmetry is the precedent). The
/// stop point is a function of (kernel, budget) alone.
#[allow(clippy::too_many_arguments)]
pub(crate) fn walk(
    ctx: &WalkCtx,
    bag: &[Particle],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
    obs: &mut Vec<Domino>,
    expand: &mut impl FnMut(&[Domino], DominoSet, usize) -> DominoSet,
    budget: &Cell<u64>,
) -> Option<Envelope> {
    let cost = u64::try_from(bag.len()).expect("bag fits u64");
    if budget.get() < cost {
        return None;
    }
    budget.set(budget.get() - cost);
    if k == 4 {
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(ctx.decl);
        let mass: Q = bag.iter().map(|p| p.weight).sum();
        let inc = ctx.dir.trick_line(ctx.focal, winner, tiles).scale(mass);
        if bag[0].hands.iter().all(|h| h.is_empty()) {
            return Some(Envelope::line(inc));
        }
        return Some(
            walk(
                ctx,
                bag,
                winner,
                [Domino::ALL[0]; 4],
                0,
                obs,
                expand,
                budget,
            )?
            .add_line(&inc),
        );
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
        let mut children = Vec::new();
        for d in chosen.iter() {
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
            let out = walk(ctx, &bag, leader, tiles, k + 1, obs, expand, budget);
            obs.pop();
            children.push(out?);
        }
        return Some(Envelope::max_of(children));
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
        let out = walk(ctx, &child, leader, tiles, k + 1, obs, expand, budget);
        obs.pop();
        sum = sum.add(&out?);
    }
    Some(sum)
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
    /// so the bare trick differential is used internally. Budgeted (freeze
    /// 44): `None` on exhaustion, nothing partial retained. The state cap
    /// (freeze 44(e), P_max) is checked at each insertion; on exceedance the
    /// walk is stopped promptly by poisoning the budget, `cap_hit` is set,
    /// and `None` is returned -- PG-A13 governs: no partial partition, no
    /// bound.
    pub fn build(
        kernel: &Kernel,
        root: Domino,
        budget: &mut u64,
        state_cap: usize,
        cap_hit: &mut bool,
    ) -> Option<InfoPartition> {
        let dir = Direction::trick_diff();
        let ctx = WalkCtx::new(kernel, kernel.viewer().team(), &dir);
        let bag = root_bag(kernel, root);
        let mut index: BTreeMap<Vec<Domino>, InfoStateId> = BTreeMap::new();
        let mut legal_by_id: Vec<DominoSet> = Vec::new();
        let mut nodes_by_id: Vec<usize> = Vec::new();
        let mut obs = vec![root];
        let cell = Cell::new(*budget);
        let mut hit = false;
        // N4-A13(i): the residual at the moment the cap fires is recorded
        // BEFORE the poison, so the caller's charge arithmetic stays true.
        let mut cap_residual: Option<u64> = None;
        let out = walk(
            &ctx,
            &bag,
            ctx.viewer,
            root_tiles(root),
            1,
            &mut obs,
            &mut |record, legal, nodes| {
                if legal_by_id.len() >= state_cap {
                    if !hit {
                        hit = true;
                        cap_residual = Some(cell.get());
                        cell.set(0);
                    }
                    return legal;
                }
                let id = InfoStateId(u32::try_from(legal_by_id.len()).expect("state count"));
                let prev = index.insert(record.to_vec(), id);
                assert!(prev.is_none(), "each observation record is reached once");
                legal_by_id.push(legal);
                nodes_by_id.push(nodes);
                legal
            },
            &cell,
        );
        *budget = cap_residual.unwrap_or_else(|| cell.get());
        *cap_hit = hit;
        if hit {
            return None;
        }
        out?;
        Some(InfoPartition {
            root,
            index,
            legal: legal_by_id,
            nodes: nodes_by_id,
        })
    }

    /// The N4-A15(iii) count-only pass: the same traversal as `build`
    /// (Lemma N(a)) with a callback that only counts and folds the streaming
    /// set digest — the EXACT partition state count at O(1) memory, so the
    /// cap becomes an admission threshold on a measured count instead of a
    /// truncation. Honesty clause, printed by callers: this pass loses the
    /// `prev.is_none()` dedup check; uniqueness rests on the by-construction
    /// argument that a tree walk's node label is its play prefix, so the
    /// focal callback fires exactly once per observation record. The digest
    /// is the N4-A5 streaming set digest: per record, the 128-bit FNV-1a of
    /// the record's canonical byte encoding (domino indices in play order),
    /// folded by wrapping addition mod 2^128 (commutative, exact).
    pub fn count_digest(kernel: &Kernel, root: Domino, budget: &mut u64) -> Option<(u64, u128)> {
        let dir = Direction::trick_diff();
        let ctx = WalkCtx::new(kernel, kernel.viewer().team(), &dir);
        let bag = root_bag(kernel, root);
        let mut count: u64 = 0;
        let mut digest: u128 = 0;
        let mut obs = vec![root];
        let cell = Cell::new(*budget);
        let out = walk(
            &ctx,
            &bag,
            ctx.viewer,
            root_tiles(root),
            1,
            &mut obs,
            &mut |record, legal, _| {
                count += 1;
                digest = digest.wrapping_add(fnv128_record(record));
                legal
            },
            &cell,
        );
        *budget = cell.get();
        out?;
        Some((count, digest))
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

/// The SEP-A13 max-freedom receipt of one `policy_value_receipt` walk: the
/// no-maximization property counted, not inspected. Focal callback
/// invocations, singleton expansions, and distinct partition states reached
/// are tallied separately; a caller asserts all three equal. A fixed policy
/// prunes counterfactual focal branches, so the reached set is the
/// policy-consistent subtree of the partition, not the whole partition.
#[derive(Clone, Copy, Debug)]
pub struct MaxFreeReceipt {
    /// Focal information states the walk evaluated (callback invocations).
    pub focal_states: u64,
    /// Expansions returned as singletons (asserted at every invocation).
    pub singleton_expansions: u64,
    /// Distinct partition states among the visited records: equality with
    /// `focal_states` receipts that no state was visited twice and every
    /// visited record is a partition state.
    pub distinct_states: u64,
}

/// The exact expected value of one deterministic information-consistent
/// policy under the uniform fiber belief and the uniform-random legal field:
/// a single affine line (§9.4 -- one policy contributes one line), plus the
/// SEP-A13 counted receipt. This is the no-maximization code path; the H
/// operator must dominate it pointwise and touch it wherever the policy is
/// optimal. At every focal state the expansion is asserted a singleton at
/// the callback itself -- the structural form of DS-A14/DS-A27's obligation.
pub fn policy_value_receipt(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    partition: &InfoPartition,
    policy: &Policy,
    budget: &mut u64,
) -> Option<(Line, MaxFreeReceipt)> {
    assert_eq!(
        partition.root(),
        policy.root(),
        "a policy is evaluated on its own root's partition"
    );
    let ctx = WalkCtx::new(kernel, focal, dir);
    let bag = root_bag(kernel, policy.root());
    let mut obs = vec![policy.root()];
    let mut focal_states: u64 = 0;
    let mut singleton_expansions: u64 = 0;
    let mut seen: std::collections::BTreeSet<InfoStateId> = std::collections::BTreeSet::new();
    let cell = Cell::new(*budget);
    let env = walk(
        &ctx,
        &bag,
        ctx.viewer,
        root_tiles(policy.root()),
        1,
        &mut obs,
        &mut |record, _legal, _| {
            focal_states += 1;
            let id = partition
                .id(record)
                .expect("the partition covers every reachable state");
            seen.insert(id);
            let chosen = DominoSet::single(policy.action(id));
            assert_eq!(chosen.len(), 1, "the L path expands singletons only");
            singleton_expansions += 1;
            chosen
        },
        &cell,
    );
    *budget = cell.get();
    let env = env?;
    let n = i128::try_from(kernel.count()).expect("fiber sizes fit i128");
    let env = env.scale(q(1, n));
    // Cheap invariant only: vacuous as a receipt at a zero-slope direction
    // (SEP-A13); the counted receipt above is the reported object.
    assert!(env.is_affine(), "one deterministic policy induces one line");
    Some((
        env.pieces()[0].line,
        MaxFreeReceipt {
            focal_states,
            singleton_expansions,
            distinct_states: u64::try_from(seen.len()).expect("state count"),
        },
    ))
}

/// `policy_value_receipt` without the receipt, for callers that only need
/// the line.
pub fn policy_value(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    partition: &InfoPartition,
    policy: &Policy,
    budget: &mut u64,
) -> Option<Line> {
    policy_value_receipt(kernel, focal, dir, partition, policy, budget).map(|(l, _)| l)
}

/// 128-bit FNV-1a of a record's canonical byte encoding: domino indices in
/// play order, one byte each (N4-A5's streaming set-digest primitive).
/// Exact integer arithmetic; wrapping multiplication mod 2^128.
pub fn fnv128_record(record: &[Domino]) -> u128 {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013B;
    let mut h = OFFSET;
    for d in record {
        h ^= d.index() as u128;
        h = h.wrapping_mul(PRIME);
    }
    h
}

/// The receipt of one record-keyed fixed-policy walk (the N4-A15 fallback
/// form): the counted max-freedom halves of SEP-A13/SEP-A19. The
/// distinct-states set is NOT held (that is the point of the fallback);
/// uniqueness of visited records rests on the by-construction tree-walk
/// argument of N4-A15(iii), which callers print.
#[derive(Clone, Copy, Debug)]
pub struct RecordWalkReceipt {
    pub focal_states: u64,
    pub singleton_expansions: u64,
}

/// The exact expected value of one deterministic information-consistent
/// policy given as a TOTAL record-keyed choice map -- the N4-A5/N4-A15
/// fallback pricing path: no `InfoPartition` and no id map are resident.
/// The structural max-freedom obligation is discharged identically to
/// `policy_value_receipt`: the focal expansion is a singleton at every
/// callback, asserted per call. A record absent from the map is
/// stop-and-report (the seed was not total on the reached set).
pub fn policy_value_by_record(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
    root: Domino,
    choices: &BTreeMap<Vec<Domino>, Domino>,
    budget: &mut u64,
) -> Option<(Line, RecordWalkReceipt)> {
    let ctx = WalkCtx::new(kernel, focal, dir);
    let bag = root_bag(kernel, root);
    let mut obs = vec![root];
    let mut focal_states: u64 = 0;
    let mut singleton_expansions: u64 = 0;
    let cell = Cell::new(*budget);
    let env = walk(
        &ctx,
        &bag,
        ctx.viewer,
        root_tiles(root),
        1,
        &mut obs,
        &mut |record, _legal, _| {
            focal_states += 1;
            let chosen = DominoSet::single(
                *choices
                    .get(record)
                    .expect("stop-and-report: the seed map does not cover a reached record"),
            );
            assert_eq!(chosen.len(), 1, "the L path expands singletons only");
            singleton_expansions += 1;
            chosen
        },
        &cell,
    );
    *budget = cell.get();
    let env = env?;
    let n = i128::try_from(kernel.count()).expect("fiber sizes fit i128");
    let env = env.scale(q(1, n));
    assert!(env.is_affine(), "one deterministic policy induces one line");
    Some((
        env.pieces()[0].line,
        RecordWalkReceipt {
            focal_states,
            singleton_expansions,
        },
    ))
}
