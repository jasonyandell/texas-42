//! Sampled, information-consistent policy construction and incremental reuse.
//! Exact only on the supplied finite sample against the declared frozen field.
//! A memo belongs to one immutable root, field and ordered sample stream.
//! Completed unrestricted memo entries remain valid resumable work after a
//! budget refusal, but no partial node or donor grammar is returned. Donor
//! composition equals unrestricted search only when the accumulated donor
//! action pool contains an optimum for the joint sample.
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use crate::gym::{self, ExerciseRoot};
use crate::kernel::World;
use crate::rules::{legal_plays, Decl, Domino, DominoSet, Seat, Trick};
use crate::solver::adaptive::{PublicRecord, RootPosition, SlicePolicy};
use crate::solver::mix;

pub mod request;

pub mod program;

/// Stable content identity for serialized executable experiment policies.
pub fn program_digest(source: &str) -> String {
    crate::solver::policy::content_digest(source.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[derive(Clone, Debug)]
pub struct State {
    pub leader: Seat,
    pub prefix: Vec<Domino>,
    pub banked: [u32; 2],
    pub played: DominoSet,
    pub history: Vec<Domino>,
}
impl State {
    pub fn from_root(root: &RootPosition) -> Self {
        Self {
            leader: root.leader,
            prefix: root.trick_plays.clone(),
            banked: root.banked,
            played: root
                .prior_played
                .union(root.trick_plays.iter().copied().collect()),
            history: Vec::new(),
        }
    }
    pub fn actor(&self) -> Seat {
        self.leader.plus(self.prefix.len())
    }
    pub fn step(&self, decl: Decl, tile: Domino) -> Self {
        assert!(!self.played.contains(tile));
        let mut next = self.clone();
        next.played.insert(tile);
        next.prefix.push(tile);
        next.history.push(tile);
        if next.prefix.len() == 4 {
            let trick = Trick::new(next.leader, next.prefix.clone().try_into().unwrap()).unwrap();
            next.leader = trick.winner(decl);
            next.banked[next.leader.team().index()] += trick.points();
            next.prefix.clear();
        }
        next
    }
    pub fn record<'a>(&'a self, root: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.prefix,
            banked: self.banked,
            root,
            history: &self.history,
        }
    }
    pub fn success(&self, root: &RootPosition) -> Option<bool> {
        let t = root.declaring_team.index();
        if self.banked[t] >= root.bid {
            Some(true)
        } else if self.banked[1 - t] > 42 - root.bid {
            Some(false)
        } else {
            None
        }
    }
}

/// A cheap deterministic, seat-local field for isolating synthesis costs.
/// It is an experimental field, not the existing L0 or a strength benchmark.
pub struct HashField;
impl SlicePolicy for HashField {
    fn id(&self) -> &str {
        "hash-legal-v1"
    }
    fn choose(
        &self,
        _decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        let mut hash = mix(0x504f_4c49_4359 ^ u64::from(hand.bits()) ^ record.root.identity());
        for tile in record.history {
            hash = mix(hash ^ (tile.index() as u64 + 1));
        }
        let mut rng = crate::kernel::SplitMix64::new(hash);
        legal
            .iter()
            .nth(rng.below(legal.len() as u128) as usize)
            .unwrap()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Work {
    pub nodes: u64,
    pub hits: u64,
    pub field_calls: u64,
    pub limit: u64,
}
impl Work {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            ..Self::default()
        }
    }
    fn spend(&mut self) -> Result<(), String> {
        if self.nodes >= self.limit {
            return Err("node budget exhausted".into());
        }
        self.nodes += 1;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub makes: usize,
    /// Only a chosen focal edge or the disjoint observable hidden edges.
    pub edges: Vec<(Domino, Arc<Node>)>,
    pub focal: bool,
}
type MemoKey = (Vec<u8>, Vec<usize>);
pub type ActionPool = BTreeMap<Vec<u8>, DominoSet>;

/// Root-scoped session: owned sample vector prevents cache reuse under new
/// sample meanings. New samples may only be appended. Field/root are private,
/// immutable borrows for this session's entire lifetime. The current payoff
/// maximizes declaring-team makes, so execution refuses a defending viewer.
pub struct Search<'a> {
    root: &'a ExerciseRoot,
    field: &'a dyn SlicePolicy,
    worlds: Vec<World>,
    memo: HashMap<MemoKey, Arc<Node>>,
}
impl<'a> Search<'a> {
    pub fn new(root: &'a ExerciseRoot, field: &'a dyn SlicePolicy) -> Self {
        Self {
            root,
            field,
            worlds: Vec::new(),
            memo: HashMap::new(),
        }
    }
    pub fn append(&mut self, world: World) -> Result<(), String> {
        if !self.root.root.kernel().contains(&world) {
            return Err("sample outside root support".into());
        }
        self.worlds.push(world);
        Ok(())
    }
    pub fn root(&self) -> &ExerciseRoot {
        self.root
    }
    pub fn samples(&self) -> usize {
        self.worlds.len()
    }
    pub fn cache_len(&self) -> usize {
        self.memo.len()
    }
    pub fn clear_cache(&mut self) {
        self.memo.clear();
    }
    pub fn solve(&mut self, work: &mut Work) -> Result<Arc<Node>, String> {
        self.check_controller_scope()?;
        if self.worlds.is_empty() {
            return Err("empty sample".into());
        }
        self.walk(
            &State::from_root(&self.root.position),
            &(0..self.worlds.len()).collect::<Vec<_>>(),
            None,
            &mut None,
            work,
        )
    }
    /// Extract ALL successful singleton action alternatives, not just one
    /// selected optimizer. Disagreements between witnesses are not cuts.
    pub fn donor(
        &mut self,
        index: usize,
        pool: &mut ActionPool,
        work: &mut Work,
    ) -> Result<Arc<Node>, String> {
        self.check_controller_scope()?;
        if index >= self.worlds.len() {
            return Err("invalid donor index".into());
        }
        // Donor traversal must visit nodes even when a memo entry exists.  A
        // refusal must not expose a partially collected grammar.
        let mut staged = ActionPool::new();
        let result = self.walk(
            &State::from_root(&self.root.position),
            &[index],
            None,
            &mut Some(&mut staged),
            work,
        );
        if result.is_ok() {
            for (history, alternatives) in staged {
                let entry = pool.entry(history).or_insert(DominoSet::EMPTY);
                *entry = entry.union(alternatives);
            }
        }
        result
    }
    pub fn compose(&mut self, pool: &ActionPool, work: &mut Work) -> Result<Arc<Node>, String> {
        self.check_controller_scope()?;
        if self.worlds.is_empty() {
            return Err("empty sample".into());
        }
        // A changed grammar is not a valid scope for old exact memo entries.
        self.memo.clear();
        let result = self.walk(
            &State::from_root(&self.root.position),
            &(0..self.worlds.len()).collect::<Vec<_>>(),
            Some(pool),
            &mut None,
            work,
        );
        // Restricted-grammar entries are never valid for solve() or for a
        // composition under a different pool.
        self.memo.clear();
        result
    }
    fn check_controller_scope(&self) -> Result<(), String> {
        if self.root.root.kernel().viewer().team() != self.root.position.declaring_team {
            return Err("policy search maximizes declaring-team makes; viewer must be on the declaring team".into());
        }
        Ok(())
    }
    fn walk(
        &mut self,
        state: &State,
        ids: &[usize],
        pool: Option<&ActionPool>,
        collect: &mut Option<&mut ActionPool>,
        work: &mut Work,
    ) -> Result<Arc<Node>, String> {
        let key = (
            state
                .history
                .iter()
                .map(|d| d.index() as u8)
                .collect::<Vec<_>>(),
            ids.to_vec(),
        );
        if collect.is_none() {
            if let Some(node) = self.memo.get(&key) {
                work.hits += 1;
                return Ok(Arc::clone(node));
            }
        }
        work.spend()?;
        if let Some(success) = state.success(&self.root.position) {
            return Ok(Arc::new(Node {
                makes: if success { ids.len() } else { 0 },
                edges: Vec::new(),
                focal: false,
            }));
        }
        let seat = state.actor();
        let decl = self.root.position.decl;
        let led = state.prefix.first().map(|d| decl.led_context(*d));
        let node = if seat == self.root.root.kernel().viewer() {
            let hand = self
                .root
                .root
                .kernel()
                .viewer_hand()
                .difference(state.played);
            let legal = legal_plays(decl, hand, led);
            if legal.is_empty() {
                return Err("undecided terminal or invalid focal hand".into());
            }
            let offered = pool
                .and_then(|p| p.get(&key.0))
                .map(|p| p.intersection(legal))
                .filter(|p| !p.is_empty())
                .unwrap_or(legal);
            let mut best: Option<(Domino, Arc<Node>)> = None;
            let mut successful = DominoSet::EMPTY;
            for tile in offered.iter() {
                let child = self.walk(&state.step(decl, tile), ids, pool, collect, work)?;
                if child.makes > 0 {
                    successful.insert(tile);
                }
                if best.as_ref().is_none_or(|(_, b)| child.makes > b.makes) {
                    best = Some((tile, child));
                }
            }
            if let Some(p) = collect.as_deref_mut() {
                // Empty alternatives indicate no successful continuation at
                // this node; composition retains the full legal fallback.
                let entry = p.entry(key.0.clone()).or_insert(DominoSet::EMPTY);
                *entry = entry.union(successful);
            }
            let (tile, child) = best.unwrap();
            Arc::new(Node {
                makes: child.makes,
                edges: vec![(tile, child)],
                focal: true,
            })
        } else {
            let mut branches: BTreeMap<Domino, Vec<usize>> = BTreeMap::new();
            let record = state.record(&self.root.position);
            for &id in ids {
                let hand = self.worlds[id].hand(seat).difference(state.played);
                let legal = legal_plays(decl, hand, led);
                if legal.is_empty() {
                    return Err("invalid hidden hand".into());
                }
                let tile = self.field.choose(decl, hand, legal, &record);
                if !legal.contains(tile) {
                    return Err("field chose illegal tile".into());
                }
                work.field_calls += 1;
                branches.entry(tile).or_default().push(id);
            }
            let mut edges = Vec::new();
            let mut makes = 0;
            for (tile, branch) in branches {
                let child = self.walk(&state.step(decl, tile), &branch, pool, collect, work)?;
                makes += child.makes;
                edges.push((tile, child));
            }
            Arc::new(Node {
                makes,
                edges,
                focal: false,
            })
        };
        if collect.is_none() {
            self.memo.insert(key, Arc::clone(&node));
        }
        Ok(node)
    }
}

/// One root-scoped total policy. Missing branches use the lowest legal tile.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TablePolicy {
    pub choices: BTreeMap<Vec<u8>, Domino>,
}
impl TablePolicy {
    pub fn from_node(node: &Node) -> Self {
        fn visit(node: &Node, history: &mut Vec<u8>, out: &mut BTreeMap<Vec<u8>, Domino>) {
            if node.focal {
                out.insert(history.clone(), node.edges[0].0);
            }
            for (tile, child) in &node.edges {
                history.push(tile.index() as u8);
                visit(child, history, out);
                history.pop();
            }
        }
        let mut result = Self::default();
        visit(node, &mut Vec::new(), &mut result.choices);
        result
    }
    pub fn choose(&self, state: &State, legal: DominoSet) -> Domino {
        let key = state
            .history
            .iter()
            .map(|d| d.index() as u8)
            .collect::<Vec<_>>();
        let action = self
            .choices
            .get(&key)
            .copied()
            .unwrap_or_else(|| legal.iter().next().unwrap());
        assert!(
            legal.contains(action),
            "table belongs to its extraction root"
        );
        action
    }
}

/// Independent fixed-policy replay: no search recursion or sample IDs.
pub fn replay(
    root: &ExerciseRoot,
    field: &dyn SlicePolicy,
    world: &World,
    policy: &TablePolicy,
) -> Result<(bool, Vec<(Seat, Domino)>), String> {
    if !root.root.kernel().contains(world) {
        return Err("replay outside support".into());
    }
    let mut state = State::from_root(&root.position);
    let mut trace = Vec::new();
    while state.played.len() < 28 {
        let seat = state.actor();
        let hand = world.hand(seat).difference(state.played);
        let legal = legal_plays(
            root.position.decl,
            hand,
            state
                .prefix
                .first()
                .map(|d| root.position.decl.led_context(*d)),
        );
        let tile = if seat == root.root.kernel().viewer() {
            policy.choose(&state, legal)
        } else {
            field.choose(
                root.position.decl,
                hand,
                legal,
                &state.record(&root.position),
            )
        };
        if !legal.contains(tile) {
            return Err("replay illegal action".into());
        }
        trace.push((seat, tile));
        state = state.step(root.position.decl, tile);
    }
    if state.banked.iter().sum::<u32>() != 42 {
        return Err("replay score conservation".into());
    }
    Ok((
        state.banked[root.position.declaring_team.index()] >= root.position.bid,
        trace,
    ))
}

pub struct Fixture {
    pub exercise: ExerciseRoot,
    pub original: DominoSet,
    pub history: Vec<(Seat, Domino)>,
}
/// Every fixture begins with a real full deal; suffixes come from legal replay.
pub fn fixture(
    seed: u64,
    decl: Decl,
    tiles: usize,
    fixed: Option<DominoSet>,
) -> Result<Fixture, String> {
    if !(1..=7).contains(&tiles) {
        return Err("tiles outside 1..7".into());
    }
    let mut rng = crate::kernel::SplitMix64::new(seed ^ 0x4445_414c);
    let mut deck = DominoSet::FULL.iter().collect::<Vec<_>>();
    rng.shuffle(&mut deck);
    let original = fixed.unwrap_or_else(|| deck[..7].iter().copied().collect());
    if original.len() != 7 {
        return Err("original hand must have 7 tiles".into());
    }
    let mut other = DominoSet::FULL
        .difference(original)
        .iter()
        .collect::<Vec<_>>();
    rng.shuffle(&mut other);
    let mut hands = [DominoSet::EMPTY; 4];
    hands[0] = original;
    for s in 1..4 {
        hands[s] = other[(s - 1) * 7..s * 7].iter().copied().collect();
    }
    let opening = gym::from_request(decl, Seat::S0, Seat::S0, original, &[])?;
    let mut state = State::from_root(&opening.position);
    let mut history = Vec::new();
    while !(state.actor() == Seat::S0 && original.difference(state.played).len() == tiles) {
        let seat = state.actor();
        let hand = hands[seat.index()].difference(state.played);
        let legal = legal_plays(
            decl,
            hand,
            state.prefix.first().map(|d| decl.led_context(*d)),
        );
        let tile = HashField.choose(decl, hand, legal, &state.record(&opening.position));
        history.push((seat, tile));
        state = state.step(decl, tile);
        if state.played.len() == 28 {
            return Err("target coordinate absent".into());
        }
    }
    let exercise = gym::from_request(decl, Seat::S0, Seat::S0, original, &history)?;
    Ok(Fixture {
        exercise,
        original,
        history,
    })
}
