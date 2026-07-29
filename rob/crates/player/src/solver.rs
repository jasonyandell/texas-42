//! The exact W0 information-set best-response solver
//! (BRIEF_PLAYER_01 §7 solver; INV-P3/P5/P6/P7).
//!
//! `solve` computes rob's whole-hand contingent plan at a viewer decision:
//! exact backward induction over the viewer's information-set tree, every
//! fiber world visited (never sampled — INV-P3), σ ([`crate::sigma`]) played
//! at every hidden seat, integer sum-values compared only between actions at
//! the same node, canonical lowest-identity tie-breaks. The window depth `H`
//! comes from the normative budget formula ([`crate::window`]) — callers
//! cannot pass a depth (INV-P6).
//!
//! Two exact engines, one law:
//!
//! - **Streaming backward induction** — the fiber is enumerated into a
//!   compact buffer and every world advanced in lockstep through the
//!   window, worlds partitioned by observation at each viewer decision.
//!   Used whenever the window's work estimate fits the budget.
//! - **Response-class counting** (window 1 only) — when even a one-trick
//!   sweep of the fiber exceeds the budget (the trick-one fiber is up to
//!   399,072,960 worlds), the value sum is computed *without visiting
//!   worlds*: σ's choice for a hidden seat is a pure function of its hand,
//!   so the fiber partitions into finitely many σ-response classes
//!   ("the seat holds `r` and avoids `E`"), each counted exactly by a
//!   capacity DP (CELL-10 route, machine integers). Identical values by a
//!   different arithmetic route; the engines cross-check in the
//!   `r_sol_engines` receipt.
//!
//! Both engines materialize the same [`Plan`]; child bundles partition
//! every node (INV-P5, asserted here and receipted).

use std::collections::BTreeMap;

use rob_core::{
    algebra_for, derive_rule_cells, domino_from_id, DominoId, MechanicalState, PlayPhase, Rank,
    RemainderWorld, RuleDerivedCellSystem, Seat, TrickKey,
};

use crate::plan::{
    FrontierLeaf, LeafKind, Observation, Plan, PlanChild, PlanLeaf, PlanNode, MATERIALIZATION_CAP,
};
use crate::player::UtilityLens;
use crate::window::{window_depth, window_estimate, WINDOW_BUDGET};

/// Typed solve failure (BRIEF_PLAYER_01 §10.7).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SolveError {
    /// `ContractSuccess` is undefined at a truncated frontier: the lens is
    /// permitted only when the window reaches the end of the hand.
    ContractSuccessTruncated,
}

const SEATS: u8 = 4;
const TILES: usize = 28;
const HIDDEN: usize = 3;

/// Declaration-relative lookup tables, generated at solve time from the
/// implemented S1 algebra (derived immutable tables from implemented
/// definitions — BRIEF.md §5 guardrail; never transcribed).
struct Tables {
    /// `key[led][d]`: integer-encoded `trick_key(d, led_suit(led))`,
    /// strictly monotone in the `TrickKey` order; 0 exactly for sloughs.
    key: [[u32; TILES]; TILES],
    /// `follows[led][d]` — effective-suit membership.
    follows: [[bool; TILES]; TILES],
    /// `self_key[d] = key[d][d]`: a tile led under its own context.
    self_key: [u32; TILES],
    /// Count points per tile.
    points: [u32; TILES],
}

fn encode_key(key: TrickKey) -> u32 {
    match key {
        TrickKey::Slough => 0,
        TrickKey::Ranked { tier, rank } => {
            let rank_enc = match rank {
                Rank::PipSum(n) => (n as u32) << 2,
                Rank::DoublePip(p) => ((p.value() as u32) << 2) | 1,
                Rank::Top => (255u32 << 2) | 2,
            };
            ((tier as u32) << 16) | (rank_enc + 1)
        }
    }
}

impl Tables {
    fn build(declaration: rob_core::Declaration) -> Tables {
        let algebra = algebra_for(declaration);
        let mut key = [[0u32; TILES]; TILES];
        let mut follows = [[false; TILES]; TILES];
        let mut self_key = [0u32; TILES];
        let mut points = [0u32; TILES];
        for led in rob_core::all_ids() {
            let q = algebra.led_suit(led);
            for d in rob_core::all_ids() {
                key[led.index()][d.index()] = encode_key(algebra.trick_key(d, q));
                follows[led.index()][d.index()] = algebra.follows(d, q);
            }
        }
        for d in rob_core::all_ids() {
            self_key[d.index()] = key[d.index()][d.index()];
            points[d.index()] = domino_from_id(d).count_points() as u32;
        }
        Tables {
            key,
            follows,
            self_key,
            points,
        }
    }
}

/// One fiber world in compact form: per hidden seat, up to seven global
/// tile indices, `0xFF`-padded. A solver compute buffer, never a semantic
/// type (the semantic fiber object stays [`RemainderWorld`]).
type CompactWorld = [[u8; 7]; HIDDEN];

/// A fixed-capacity partial trick.
#[derive(Clone, Copy)]
struct TrickBuf {
    arr: [(u8, u8); 4],
    len: u8,
}

impl TrickBuf {
    fn from_slice(plays: &[(u8, u8)]) -> TrickBuf {
        let mut t = TrickBuf {
            arr: [(u8::MAX, u8::MAX); 4],
            len: 0,
        };
        for &p in plays {
            t.push(p.0, p.1);
        }
        t
    }

    fn push(&mut self, seat: u8, tile: u8) {
        self.arr[self.len as usize] = (seat, tile);
        self.len += 1;
    }

    fn plays(&self) -> &[(u8, u8)] {
        &self.arr[..self.len as usize]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}

/// The public decision context shared by every world of a bundle.
#[derive(Clone)]
struct Pos {
    leader: u8,
    trick: TrickBuf,
    /// Banked points by team.
    banked: [u32; 2],
    /// Viewer's remaining hand (global tile indices, canonical order).
    viewer_hand: Vec<u8>,
    /// Tiles played by each hidden seat *within this solve*.
    hidden_played: [[bool; TILES]; HIDDEN],
    /// Completed tricks of the hand (absolute).
    tricks_done: usize,
}

/// Fixed-capacity observation buffer: at most six plays separate two viewer
/// decisions (three completing the current trick, three before the viewer's
/// turn in the next).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ObsBuf {
    arr: [(u8, u8); 6],
    len: u8,
}

impl ObsBuf {
    fn new() -> ObsBuf {
        ObsBuf {
            arr: [(u8::MAX, u8::MAX); 6],
            len: 0,
        }
    }

    fn push(&mut self, seat: u8, tile: u8) {
        self.arr[self.len as usize] = (seat, tile);
        self.len += 1;
    }

    fn plays(&self) -> &[(u8, u8)] {
        &self.arr[..self.len as usize]
    }

    fn to_observation(self) -> Observation {
        self.plays().to_vec()
    }
}

/// What a simulated segment runs into.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SegmentEnd {
    /// The viewer is to act again inside the window.
    Decision,
    /// The window frontier (trick boundary, window exhausted).
    Frontier,
    /// The hand settled inside the window.
    Settled,
}

/// The solve-wide fixed context.
struct Solve {
    tables: Tables,
    viewer: u8,
    viewer_team: usize,
    declaring_team: usize,
    threshold: u32,
    lens: UtilityLens,
    /// Absolute completed-trick count at which the window ends.
    window_end: usize,
    /// Hidden index (clockwise offsets 1..=3 from the viewer) → seat.
    hidden_seat: [u8; HIDDEN],
}

impl Solve {
    fn hidden_index(&self, seat: u8) -> usize {
        self.hidden_seat
            .iter()
            .position(|&s| s == seat)
            .expect("non-viewer seats are hidden")
    }

    fn utility(&self, banked: [u32; 2], settled: bool) -> i64 {
        match self.lens {
            UtilityLens::Points => {
                banked[self.viewer_team] as i64 - banked[1 - self.viewer_team] as i64
            }
            UtilityLens::ContractSuccess => {
                assert!(settled, "ContractSuccess demands a settled hand (§10.7)");
                let made = banked[self.declaring_team] >= self.threshold;
                let favorable = (self.viewer_team == self.declaring_team) == made;
                if favorable {
                    1
                } else {
                    -1
                }
            }
        }
    }

    /// σ on a compact alive-tile list — allocation-free, and must agree
    /// exactly with [`crate::sigma::greedy_sigma`] (unit-tested, and
    /// receipted through the brute-force, known-world, and engine gates).
    fn sigma_compact(&self, alive: &[u8], trick: &TrickBuf) -> u8 {
        debug_assert!(!alive.is_empty());
        let t = &self.tables;
        if trick.is_empty() {
            let mut best = alive[0];
            for &d in &alive[1..] {
                let (bk, dk) = (t.self_key[best as usize], t.self_key[d as usize]);
                if dk > bk || (dk == bk && d < best) {
                    best = d;
                }
            }
            return best;
        }
        let led = trick.plays()[0].1 as usize;
        let best_key = trick
            .plays()
            .iter()
            .map(|&(_, d)| t.key[led][d as usize])
            .max()
            .expect("nonempty trick");
        // One pass: min-(key,id) among beating followers, followers,
        // beating non-followers, and the whole hand.
        let mut min_beat_fol: Option<(u32, u8)> = None;
        let mut min_fol: Option<(u32, u8)> = None;
        let mut min_beat_any: Option<(u32, u8)> = None;
        let mut min_any: Option<(u32, u8)> = None;
        for &d in alive {
            let k = t.key[led][d as usize];
            let entry = (k, d);
            let fold = |slot: &mut Option<(u32, u8)>| {
                if slot.is_none() || entry < slot.unwrap() {
                    *slot = Some(entry);
                }
            };
            fold(&mut min_any);
            if k > best_key {
                fold(&mut min_beat_any);
            }
            if t.follows[led][d as usize] {
                fold(&mut min_fol);
                if k > best_key {
                    fold(&mut min_beat_fol);
                }
            }
        }
        if min_fol.is_some() {
            // Followers are the legal set.
            min_beat_fol.or(min_fol).expect("has follower").1
        } else {
            // Void: whole hand legal.
            min_beat_any.or(min_any).expect("nonempty hand").1
        }
    }

    /// Resolve a full four-play trick: (winner seat, trick points).
    fn resolve(&self, trick: &TrickBuf) -> (u8, u32) {
        debug_assert_eq!(trick.len, 4);
        let led = trick.arr[0].1 as usize;
        let mut winner = trick.arr[0].0;
        let mut best = self.tables.key[led][trick.arr[0].1 as usize];
        let mut points = 1u32;
        for &(s, d) in trick.plays() {
            let k = self.tables.key[led][d as usize];
            if k > best {
                best = k;
                winner = s;
            }
            points += self.tables.points[d as usize];
        }
        (winner, points)
    }

    /// Advance one world from `pos` through viewer action `action` until
    /// the next viewer decision, the frontier, or settlement.
    fn simulate(&self, world: &CompactWorld, pos: &Pos, action: u8) -> (ObsBuf, SegmentEnd) {
        let mut alive: CompactWorld = *world;
        for (i, hand) in alive.iter_mut().enumerate() {
            for slot in hand.iter_mut() {
                if *slot != u8::MAX && pos.hidden_played[i][*slot as usize] {
                    *slot = u8::MAX;
                }
            }
        }
        let mut alive_slices: [[u8; 7]; HIDDEN] = [[u8::MAX; 7]; HIDDEN];
        let mut alive_len = [0usize; HIDDEN];
        for i in 0..HIDDEN {
            for &slot in &alive[i] {
                if slot != u8::MAX {
                    alive_slices[i][alive_len[i]] = slot;
                    alive_len[i] += 1;
                }
            }
        }
        let mut trick = pos.trick;
        trick.push(self.viewer, action);
        let mut obs = ObsBuf::new();
        let mut leader = pos.leader;
        let mut tricks_done = pos.tricks_done;
        loop {
            if trick.len == 4 {
                let (winner, _) = self.resolve(&trick);
                tricks_done += 1;
                if tricks_done == 7 {
                    return (obs, SegmentEnd::Settled);
                }
                if tricks_done == self.window_end {
                    return (obs, SegmentEnd::Frontier);
                }
                leader = winner;
                trick.clear();
            }
            let actor = (leader + trick.len) % SEATS;
            if actor == self.viewer {
                return (obs, SegmentEnd::Decision);
            }
            let h = self.hidden_index(actor);
            let choice = self.sigma_compact(&alive_slices[h][..alive_len[h]], &trick);
            let pos_in = alive_slices[h][..alive_len[h]]
                .iter()
                .position(|&d| d == choice)
                .expect("sigma plays from the alive hand");
            alive_slices[h][pos_in] = alive_slices[h][alive_len[h] - 1];
            alive_len[h] -= 1;
            trick.push(actor, choice);
            obs.push(actor, choice);
        }
    }

    /// Replay an observed segment onto a shared position — the observation
    /// group's common successor context.
    fn replay(&self, pos: &Pos, action: u8, obs: &ObsBuf) -> Pos {
        let mut next = pos.clone();
        next.viewer_hand.retain(|&d| d != action);
        next.trick.push(self.viewer, action);
        for &(seat, tile) in obs.plays() {
            if next.trick.len == 4 {
                let (winner, points) = self.resolve(&next.trick);
                next.banked[(winner % 2) as usize] += points;
                next.tricks_done += 1;
                next.leader = winner;
                next.trick.clear();
            }
            let h = self.hidden_index(seat);
            next.hidden_played[h][tile as usize] = true;
            next.trick.push(seat, tile);
        }
        if next.trick.len == 4 {
            let (winner, points) = self.resolve(&next.trick);
            next.banked[(winner % 2) as usize] += points;
            next.tricks_done += 1;
            next.leader = winner;
            next.trick.clear();
        }
        next
    }

    /// The viewer's exact legal set at `pos` (Exec §11 via the tables).
    fn viewer_legal(&self, pos: &Pos) -> Vec<u8> {
        if pos.trick.is_empty() {
            return pos.viewer_hand.clone();
        }
        let led = pos.trick.plays()[0].1 as usize;
        let followers: Vec<u8> = pos
            .viewer_hand
            .iter()
            .copied()
            .filter(|&d| self.tables.follows[led][d as usize])
            .collect();
        if followers.is_empty() {
            pos.viewer_hand.clone()
        } else {
            followers
        }
    }

    /// Partition a bundle by the observation its segment produces.
    fn group(
        &self,
        worlds: &[CompactWorld],
        bundle: &[u32],
        pos: &Pos,
        action: u8,
    ) -> Vec<(ObsBuf, SegmentEnd, Vec<u32>)> {
        let mut keyed: BTreeMap<ObsBuf, (SegmentEnd, Vec<u32>)> = BTreeMap::new();
        for &w in bundle {
            let (obs, end) = self.simulate(&worlds[w as usize], pos, action);
            let entry = keyed.entry(obs).or_insert_with(|| (end, Vec::new()));
            debug_assert!(entry.0 == end, "equal observations share their end");
            entry.1.push(w);
        }
        keyed
            .into_iter()
            .map(|(obs, (end, members))| (obs, end, members))
            .collect()
    }

    /// Exact solve for `bundle` at `pos` in a single sweep: every action's
    /// subtree is valued (and materialized) once; the loser subtrees are
    /// dropped at node scope, so the chosen tree survives. Ties break to
    /// the lowest action identity (INV-P3).
    fn stream_solve(
        &self,
        worlds: &[CompactWorld],
        bundle: &[u32],
        pos: &Pos,
        mut openings: Option<&mut Vec<(u8, i64)>>,
    ) -> PlanNode {
        let legal = self.viewer_legal(pos);
        debug_assert!(!legal.is_empty());
        let mut best: Option<PlanNode> = None;
        for &action in &legal {
            let mut children: BTreeMap<Observation, PlanChild> = BTreeMap::new();
            let mut value_total = 0i64;
            for (obs, end, members) in self.group(worlds, bundle, pos, action) {
                let next = self.replay(pos, action, &obs);
                let child = match end {
                    SegmentEnd::Decision => {
                        PlanChild::Node(self.stream_solve(worlds, &members, &next, None))
                    }
                    SegmentEnd::Frontier | SegmentEnd::Settled => {
                        let settled = end == SegmentEnd::Settled;
                        if settled {
                            debug_assert_eq!(
                                next.banked[0] + next.banked[1],
                                42,
                                "settled hands conserve 42 points (INV-P6)"
                            );
                        }
                        let value = members.len() as i64 * self.utility(next.banked, settled);
                        PlanChild::Leaf(PlanLeaf {
                            kind: if settled {
                                LeafKind::Settled
                            } else {
                                LeafKind::Frontier(FrontierLeaf::BankedPoints)
                            },
                            world_count: members.len() as u64,
                            value_total: value,
                        })
                    }
                };
                value_total += child.value_total();
                children.insert(obs.to_observation(), child);
            }
            let candidate = PlanNode {
                action: id_of(action),
                world_count: bundle.len() as u64,
                value_total,
                children,
            };
            if let Some(collector) = openings.as_deref_mut() {
                collector.push((action, value_total));
            }
            let better = match &best {
                None => true,
                Some(b) => {
                    candidate.value_total > b.value_total
                        || (candidate.value_total == b.value_total
                            && action < b.action.index() as u8)
                }
            };
            if better {
                best = Some(candidate);
            }
        }
        best.expect("nonempty legal set")
    }
}

/// Depth-prune a plan to the materialization cap (all-or-nothing per node:
/// a pruned node keeps its exact totals and drops all its children, so the
/// INV-P5 walk stays meaningful on every surviving node). Returns whether
/// anything was pruned.
fn prune_to_cap(root: &mut PlanNode, cap: usize) -> bool {
    fn count(node: &PlanNode) -> usize {
        1 + node
            .children
            .values()
            .map(|c| match c {
                PlanChild::Node(n) => count(n),
                PlanChild::Leaf(_) => 1,
            })
            .sum::<usize>()
    }
    fn max_depth(node: &PlanNode) -> usize {
        1 + node
            .children
            .values()
            .map(|c| match c {
                PlanChild::Node(n) => max_depth(n),
                PlanChild::Leaf(_) => 1,
            })
            .max()
            .unwrap_or(0)
    }
    fn drop_below(node: &mut PlanNode, depth: usize) {
        if depth == 0 {
            node.children.clear();
            return;
        }
        for child in node.children.values_mut() {
            if let PlanChild::Node(n) = child {
                drop_below(n, depth - 1);
            }
        }
    }
    if count(root) <= cap {
        return false;
    }
    let mut depth = max_depth(root);
    while depth > 0 && count(root) > cap {
        depth -= 1;
        drop_below(root, depth);
    }
    true
}

fn id_of(index: u8) -> DominoId {
    rob_core::all_ids()
        .nth(index as usize)
        .expect("valid tile index")
}

/// Enumerate the fiber into compact worlds (a solver buffer; the caller
/// asserts the count against the capacity DP).
fn enumerate_compact(cells: &RuleDerivedCellSystem) -> Vec<CompactWorld> {
    let pool: Vec<u8> = cells
        .unseen_pool()
        .iter()
        .map(|d| d.index() as u8)
        .collect();
    let possible: [Vec<bool>; HIDDEN] = core::array::from_fn(|i| {
        pool.iter()
            .map(|&t| cells.possible(i).contains(id_of(t)))
            .collect()
    });
    let capacity: [usize; HIDDEN] = core::array::from_fn(|i| cells.capacity(i));
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        tile: usize,
        pool: &[u8],
        possible: &[Vec<bool>; HIDDEN],
        capacity: &[usize; HIDDEN],
        current: &mut CompactWorld,
        filled: &mut [usize; HIDDEN],
        out: &mut Vec<CompactWorld>,
    ) {
        if tile == pool.len() {
            out.push(*current);
            return;
        }
        for s in 0..HIDDEN {
            if filled[s] < capacity[s] && possible[s][tile] {
                current[s][filled[s]] = pool[tile];
                filled[s] += 1;
                dfs(tile + 1, pool, possible, capacity, current, filled, out);
                filled[s] -= 1;
                current[s][filled[s]] = u8::MAX;
            }
        }
    }
    let mut out = Vec::new();
    let mut current: CompactWorld = [[u8::MAX; 7]; HIDDEN];
    let mut filled = [0usize; HIDDEN];
    dfs(
        0,
        &pool,
        &possible,
        &capacity,
        &mut current,
        &mut filled,
        &mut out,
    );
    out
}

// ---------------------------------------------------------------------------
// Response-class counting engine (window 1).
// ---------------------------------------------------------------------------

/// Constraint-counting context over the unseen pool.
struct CountCtx {
    /// Pool tiles as global indices, canonical order.
    pool: Vec<u8>,
    /// Per hidden seat: allowed pool positions (voids already applied).
    possible: [Vec<bool>; HIDDEN],
    /// Per hidden seat: capacity.
    capacity: [usize; HIDDEN],
}

impl CountCtx {
    fn from_cells(cells: &RuleDerivedCellSystem) -> CountCtx {
        let pool: Vec<u8> = cells
            .unseen_pool()
            .iter()
            .map(|d| d.index() as u8)
            .collect();
        CountCtx {
            possible: core::array::from_fn(|i| {
                cells
                    .unseen_pool()
                    .iter()
                    .map(|d| cells.possible(i).contains(d))
                    .collect()
            }),
            capacity: core::array::from_fn(|i| cells.capacity(i)),
            pool,
        }
    }

    /// Exact count of pool partitions where each constrained seat holds its
    /// required tile and avoids its excluded set — a flat u64 occupancy DP
    /// (CELL-10 route; every native count fits u64).
    fn count(&self, required: &[(usize, usize)], excluded: &[Vec<bool>; HIDDEN]) -> u64 {
        let mut capacity = self.capacity;
        for &(seat, _) in required {
            debug_assert!(capacity[seat] > 0, "required tile fits the capacity");
            capacity[seat] -= 1;
        }
        let dims = [capacity[0] + 1, capacity[1] + 1, capacity[2] + 1];
        let size = dims[0] * dims[1] * dims[2];
        let strides = [dims[1] * dims[2], dims[2], 1];
        let mut layer = vec![0u64; size];
        layer[0] = 1;
        #[allow(clippy::needless_range_loop)] // `t` indexes three parallel per-seat tables
        for t in 0..self.pool.len() {
            if required.iter().any(|&(_, pos)| pos == t) {
                continue;
            }
            let mut next = vec![0u64; size];
            for (i, &coeff) in layer.iter().enumerate() {
                if coeff == 0 {
                    continue;
                }
                let occ = [i / strides[0], (i / strides[1]) % dims[1], i % dims[2]];
                for s in 0..HIDDEN {
                    if occ[s] < capacity[s] && self.possible[s][t] && !excluded[s][t] {
                        next[i + strides[s]] += coeff;
                    }
                }
            }
            layer = next;
        }
        layer[size - 1]
    }
}

impl Solve {
    /// Counting engine (window 1): the exact plan without visiting a single
    /// world. Σ of response-class counts is asserted equal to the fiber
    /// count per action (INV-P5).
    fn counting_node(
        &self,
        ctx: &CountCtx,
        pos: &Pos,
        fiber_count: u64,
        mut openings: Option<&mut Vec<(u8, i64)>>,
    ) -> PlanNode {
        debug_assert_eq!(pos.tricks_done + 1, self.window_end.min(7));
        let legal = self.viewer_legal(pos);
        let mut best: Option<(i64, u8, BTreeMap<Observation, PlanChild>)> = None;
        for &action in &legal {
            let mut trick = pos.trick;
            trick.push(self.viewer, action);
            let mut children: BTreeMap<Observation, PlanChild> = BTreeMap::new();
            let mut value_total = 0i64;
            let mut worlds_total = 0u64;
            let mut required: Vec<(usize, usize)> = Vec::new();
            let mut excluded: [Vec<bool>; HIDDEN] =
                core::array::from_fn(|_| vec![false; ctx.pool.len()]);
            self.enumerate_responses(
                ctx,
                pos,
                &mut trick,
                &mut required,
                &mut excluded,
                &mut |trick, required, excluded| {
                    let n = ctx.count(required, excluded);
                    if n == 0 {
                        return;
                    }
                    let (winner, points) = self.resolve(trick);
                    let mut banked = pos.banked;
                    banked[(winner % 2) as usize] += points;
                    let settled = pos.tricks_done + 1 == 7;
                    let value = n as i64 * self.utility(banked, settled);
                    let obs: Observation = trick.plays()[pos.trick.len as usize + 1..].to_vec();
                    children.insert(
                        obs,
                        PlanChild::Leaf(PlanLeaf {
                            kind: if settled {
                                LeafKind::Settled
                            } else {
                                LeafKind::Frontier(FrontierLeaf::BankedPoints)
                            },
                            world_count: n,
                            value_total: value,
                        }),
                    );
                    value_total += value;
                    worlds_total += n;
                },
            );
            assert_eq!(
                worlds_total, fiber_count,
                "response classes partition the fiber (INV-P5)"
            );
            if let Some(collector) = openings.as_deref_mut() {
                collector.push((action, value_total));
            }
            let better = match &best {
                None => true,
                Some((v, a, _)) => value_total > *v || (value_total == *v && action < *a),
            };
            if better {
                best = Some((value_total, action, children));
            }
        }
        let (value_total, action, children) = best.expect("nonempty legal set");
        PlanNode {
            action: id_of(action),
            world_count: fiber_count,
            value_total,
            children,
        }
    }

    /// Recursively enumerate the σ-response classes of the hidden seats
    /// still to play in the current trick. Each class fixes, per responder,
    /// the σ-chosen tile `r` (required) and the tiles whose presence would
    /// change σ's choice (excluded) — exactly the "σ(X) = r" condition as
    /// hand-membership constraints.
    fn enumerate_responses(
        &self,
        ctx: &CountCtx,
        pos: &Pos,
        trick: &mut TrickBuf,
        required: &mut Vec<(usize, usize)>,
        excluded: &mut [Vec<bool>; HIDDEN],
        emit: &mut impl FnMut(&TrickBuf, &[(usize, usize)], &[Vec<bool>; HIDDEN]),
    ) {
        if trick.len == 4 {
            emit(trick, required, excluded);
            return;
        }
        let actor = (pos.leader + trick.len) % SEATS;
        debug_assert_ne!(actor, self.viewer, "the viewer already played");
        let seat = self.hidden_index(actor);
        let led = trick.plays()[0].1 as usize;
        let best_key = trick
            .plays()
            .iter()
            .map(|&(_, d)| self.tables.key[led][d as usize])
            .max()
            .expect("nonempty trick");
        let t = &self.tables;
        let pool = &ctx.pool;
        let kid = |p: usize| (t.key[led][pool[p] as usize], pool[p]);
        for r in 0..pool.len() {
            if !ctx.possible[seat][r] || required.iter().any(|&(_, p)| p == r) {
                continue;
            }
            let r_tile = pool[r] as usize;
            let r_follows = t.follows[led][r_tile];
            let r_key = t.key[led][r_tile];
            // The exclusion set that makes σ's choice exactly `r`.
            let mut extra: Vec<usize> = Vec::new();
            for p in 0..pool.len() {
                if p == r || excluded[seat][p] {
                    continue;
                }
                let p_tile = pool[p] as usize;
                let p_follows = t.follows[led][p_tile];
                let p_key = t.key[led][p_tile];
                let exclude = if r_follows {
                    if r_key > best_key {
                        // Beating follower: no better beating follower.
                        p_follows && p_key > best_key && kid(p) < kid(r)
                    } else {
                        // Minimal non-beating follower: no beating
                        // follower, no lower follower.
                        p_follows && (p_key > best_key || kid(p) < kid(r))
                    }
                } else if r_key > best_key {
                    // Void beater: void of the context, no better beater.
                    p_follows || (p_key > best_key && kid(p) < kid(r))
                } else {
                    // Void slough: void, no beater, no lower tile.
                    p_follows || p_key > best_key || kid(p) < kid(r)
                };
                if exclude {
                    extra.push(p);
                }
            }
            for &p in &extra {
                excluded[seat][p] = true;
            }
            required.push((seat, r));
            trick.push(actor, pool[r]);
            self.enumerate_responses(ctx, pos, trick, required, excluded, emit);
            trick.len -= 1;
            required.pop();
            for &p in &extra {
                excluded[seat][p] = false;
            }
        }
    }
}

/// Shared solve construction from a mechanical state.
fn solve_context(state: &MechanicalState, lens: UtilityLens, window: usize) -> (Solve, Pos) {
    let viewer = state.viewer();
    let contract = state.contract();
    let hidden = state.hidden_seats();
    debug_assert!(Seat::ALL.iter().all(|s| s.team().index() == s.index() % 2));
    let tricks_done = 7 - state.own_remaining_hand().len();
    let solve = Solve {
        tables: Tables::build(contract.declaration()),
        viewer: viewer.index() as u8,
        viewer_team: viewer.team().index(),
        declaring_team: contract.declaring_team().index(),
        threshold: contract.threshold(),
        lens,
        window_end: tricks_done + window,
        hidden_seat: core::array::from_fn(|i| hidden[i].index() as u8),
    };
    let pos = Pos {
        leader: state.leader().index() as u8,
        trick: TrickBuf::from_slice(
            &state
                .current_trick()
                .iter()
                .map(|p| (p.actor.index() as u8, p.domino.index() as u8))
                .collect::<Vec<_>>(),
        ),
        banked: state.hand_points(),
        viewer_hand: state
            .own_remaining_hand()
            .iter()
            .map(|d| d.index() as u8)
            .collect(),
        hidden_played: [[false; TILES]; HIDDEN],
        tricks_done,
    };
    (solve, pos)
}

fn assert_decision_state(state: &MechanicalState) {
    assert_eq!(
        state.phase(),
        PlayPhase::Play,
        "a solve needs the play phase"
    );
    assert_eq!(
        state.current_actor(),
        Some(state.viewer()),
        "the deciding seat must be the acting seat"
    );
}

fn fiber_count_of(cells: &RuleDerivedCellSystem) -> u64 {
    let (abstract_cells, _) = cells.to_abstract();
    u64::try_from(&rob_core::count_occupancy_dp(&abstract_cells).count)
        .expect("native fiber counts fit u64")
}

fn stream_plan(
    solve: &Solve,
    pos: &Pos,
    worlds: &[CompactWorld],
    viewer: Seat,
    window: usize,
    fiber_count: u64,
    openings: Option<&mut Vec<(u8, i64)>>,
) -> Plan {
    let bundle: Vec<u32> = (0..worlds.len() as u32).collect();
    let mut root = solve.stream_solve(worlds, &bundle, pos, openings);
    let truncated = prune_to_cap(&mut root, MATERIALIZATION_CAP);
    Plan {
        viewer,
        window,
        fiber_count,
        truncated,
        root,
    }
}

/// The exact value of the best plan opening with one action — a plan
/// projection (PLAN-NOT-TILE): this is never a value of a domino, it is
/// the value of the whole contingent plan whose first move is `action`,
/// aggregated over the fiber under the solve lens (INV-P1).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OpeningValue {
    /// The opening action.
    pub action: DominoId,
    /// The best plan's exact integer value total over the fiber.
    pub value_total: i64,
}

/// Solve the viewer's decision at `state` exactly (BRIEF_PLAYER_01 §7):
/// window depth from the normative budget formula (INV-P6), the engine
/// chosen by whether the streaming estimate fits the budget, the `Plan`
/// materialized up to the node cap.
pub fn solve(state: &MechanicalState, lens: UtilityLens) -> Result<Plan, SolveError> {
    solve_inner(state, lens, None)
}

/// [`solve`], additionally reporting the exact best-plan value for every
/// legal opening (the rejected plans' root values — display projections
/// for the contingency book).
pub fn solve_with_openings(
    state: &MechanicalState,
    lens: UtilityLens,
) -> Result<(Plan, Vec<OpeningValue>), SolveError> {
    let mut raw: Vec<(u8, i64)> = Vec::new();
    let plan = solve_inner(state, lens, Some(&mut raw))?;
    let openings = raw
        .into_iter()
        .map(|(action, value_total)| OpeningValue {
            action: id_of(action),
            value_total,
        })
        .collect();
    Ok((plan, openings))
}

fn solve_inner(
    state: &MechanicalState,
    lens: UtilityLens,
    mut openings: Option<&mut Vec<(u8, i64)>>,
) -> Result<Plan, SolveError> {
    assert_decision_state(state);
    let cells = derive_rule_cells(state);
    let fiber_count = fiber_count_of(&cells);
    let hand = state.own_remaining_hand().len();
    let window = window_depth(fiber_count, hand);
    if lens == UtilityLens::ContractSuccess && window < hand {
        return Err(SolveError::ContractSuccessTruncated);
    }
    let (solve, pos) = solve_context(state, lens, window);
    if window == 1 {
        // Window 1 always takes the counting engine: exact by response
        // classes at any fiber size, milliseconds even over the full
        // 399,072,960-world trick-one fiber (and the only exact route when
        // the streaming estimate exceeds the budget).
        let ctx = CountCtx::from_cells(&cells);
        let root = solve.counting_node(&ctx, &pos, fiber_count, openings.as_deref_mut());
        Ok(Plan {
            viewer: state.viewer(),
            window,
            fiber_count,
            truncated: false,
            root,
        })
    } else {
        debug_assert!(window_estimate(fiber_count, hand, window) <= WINDOW_BUDGET);
        let worlds = enumerate_compact(&cells);
        assert_eq!(worlds.len() as u64, fiber_count, "enumeration = DP count");
        Ok(stream_plan(
            &solve,
            &pos,
            &worlds,
            state.viewer(),
            window,
            fiber_count,
            openings,
        ))
    }
}

/// The known-world degeneracy gate surface (BRIEF_PLAYER_01 §8 gate 1):
/// solve with the fiber pinned to a single world. The window formula sees
/// fiber count 1, so the window is full depth and every leaf settles.
pub fn solve_pinned(
    state: &MechanicalState,
    world: &RemainderWorld,
    lens: UtilityLens,
) -> Result<Plan, SolveError> {
    assert_decision_state(state);
    let cells = derive_rule_cells(state);
    assert!(cells.fiber_contains(world), "the pinned world is possible");
    let hand = state.own_remaining_hand().len();
    let window = window_depth(1, hand);
    if lens == UtilityLens::ContractSuccess && window < hand {
        return Err(SolveError::ContractSuccessTruncated);
    }
    let (solve, pos) = solve_context(state, lens, window);
    let mut compact: CompactWorld = [[u8::MAX; 7]; HIDDEN];
    for (i, hand_set) in world.hidden_hands.iter().enumerate() {
        for (slot, d) in hand_set.iter().enumerate() {
            compact[i][slot] = d.index() as u8;
        }
    }
    Ok(stream_plan(
        &solve,
        &pos,
        &[compact],
        state.viewer(),
        window,
        1,
        None,
    ))
}

/// Verification-only engine access (the `r_sol_engines` cross-check and
/// the `r_mat_window_ablation` budget override, BRIEF_PLAYER_01 §8). Not
/// part of the player API: `solve` alone picks engines and windows for
/// play (INV-P6).
#[doc(hidden)]
pub mod gate {
    use super::*;
    use crate::window::window_depth_with;

    /// `solve` under an explicit budget (the B/2 and 2B ablation runs).
    pub fn solve_with_budget(
        state: &MechanicalState,
        lens: UtilityLens,
        budget: u128,
    ) -> Result<Plan, SolveError> {
        assert_decision_state(state);
        let cells = derive_rule_cells(state);
        let fiber_count = fiber_count_of(&cells);
        let hand = state.own_remaining_hand().len();
        let window = window_depth_with(fiber_count, hand, budget);
        if lens == UtilityLens::ContractSuccess && window < hand {
            return Err(SolveError::ContractSuccessTruncated);
        }
        let (solve, pos) = solve_context(state, lens, window);
        if window == 1 {
            let ctx = CountCtx::from_cells(&cells);
            let root = solve.counting_node(&ctx, &pos, fiber_count, None);
            Ok(Plan {
                viewer: state.viewer(),
                window,
                fiber_count,
                truncated: false,
                root,
            })
        } else {
            let worlds = enumerate_compact(&cells);
            assert_eq!(worlds.len() as u64, fiber_count, "enumeration = DP count");
            Ok(stream_plan(
                &solve,
                &pos,
                &worlds,
                state.viewer(),
                window,
                fiber_count,
                None,
            ))
        }
    }

    /// Force the counting engine at window 1.
    pub fn counting_h1(state: &MechanicalState, lens: UtilityLens) -> Plan {
        assert_decision_state(state);
        let cells = derive_rule_cells(state);
        let fiber_count = fiber_count_of(&cells);
        let (solve, pos) = solve_context(state, lens, 1);
        let ctx = CountCtx::from_cells(&cells);
        let root = solve.counting_node(&ctx, &pos, fiber_count, None);
        Plan {
            viewer: state.viewer(),
            window: 1,
            fiber_count,
            truncated: false,
            root,
        }
    }

    /// Force the streaming engine at window 1.
    pub fn streaming_h1(state: &MechanicalState, lens: UtilityLens) -> Plan {
        assert_decision_state(state);
        let cells = derive_rule_cells(state);
        let fiber_count = fiber_count_of(&cells);
        let (solve, pos) = solve_context(state, lens, 1);
        let worlds = enumerate_compact(&cells);
        assert_eq!(worlds.len() as u64, fiber_count, "enumeration = DP count");
        stream_plan(&solve, &pos, &worlds, state.viewer(), 1, fiber_count, None)
    }
}
