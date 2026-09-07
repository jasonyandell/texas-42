//! Sound finite-support information-price teacher for policy construction.
//!
//! The instrument owns one root-relative prior and materializes the complete
//! public tree induced by one immutable nonfocal field.  A node's information
//! state is its complete post-root public play history together with the
//! multiset of surviving original-world ids.  Duplicate input worlds therefore
//! retain their multiplicity.
//!
//! Prices are examiner-only.  Their fixed, versioned feature basis may inspect
//! a complete world, while every coefficient is shared across roots, histories,
//! and actions.  Centers are recomputed exactly at every focal information
//! state.  A queried root action is imposed before the relaxation begins, so a
//! root-only centered charge cannot manufacture an improvement.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::gym::ExerciseRoot;
use crate::kernel::World;
use crate::rules::{legal_plays, Domino, Seat};
use crate::solver::adaptive::SlicePolicy;

use super::State;

pub const EVENT_BASIS_ID: &str = "scheme-information-events-v1";
pub const EVENT_COUNT: usize = 4;

/// The actual Scheme `Fix` for one event/action pair.  The native evaluator
/// below is a specialization of these four queries; focused tests compare the
/// two semantics on concrete worlds.  Query identity additionally incorporates
/// the standard registry's predicate versions.
pub fn event_scheme_source(event: usize, action: Domino) -> Result<String, String> {
    let source = match event {
        0 => "(fix (roles (chair me) (chair ally) (domino held)) (out) \
                    (case (viewer me) (partner me ally) (holds ally held) (count held 5)))"
            .to_owned(),
        1 => "(fix (roles (chair me) (chair ally) (domino held)) (out) \
                    (case (viewer me) (partner me ally) (holds ally held) (count held 10)))"
            .to_owned(),
        2 => format!(
            "(fix (roles (chair me) (chair ally) (domino held) (context q)) (out) \
             (case (viewer me) (partner me ally) (holds ally held) \
             (leads-context {action} q) (in held q)))"
        ),
        3 => format!(
            "(fix (roles (chair me) (chair ally) (domino held) (context q)) (out) \
             (case (viewer me) (partner me ally) (holds ally held) \
             (leads-context {action} q) (in held q) (count held 5)) \
             (case (viewer me) (partner me ally) (holds ally held) \
             (leads-context {action} q) (in held q) (count held 10)))"
        ),
        _ => return Err(format!("unknown price event {event}")),
    };
    Ok(source)
}

pub type NodeId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriorAuthority {
    /// Exact only for the explicitly supplied finite multiset.
    FiniteEmpirical { samples: usize },
    /// The supplied prior was checked against the root's complete uniform
    /// enumeration, including order and multiplicity.
    ExactRootFiber { worlds: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriceProgram {
    basis_id: String,
    coefficients: [i64; EVENT_COUNT],
}

impl PriceProgram {
    pub fn new(coefficients: [i64; EVENT_COUNT]) -> Self {
        Self {
            basis_id: EVENT_BASIS_ID.to_owned(),
            coefficients,
        }
    }

    pub fn zero() -> Self {
        Self::new([0; EVENT_COUNT])
    }

    pub fn basis_id(&self) -> &str {
        &self.basis_id
    }

    pub fn coefficients(&self) -> &[i64; EVENT_COUNT] {
        &self.coefficients
    }

    /// A finite discovery library.  Selecting a member is deliberately left
    /// to discovery data; a held-out oracle consumes one already-frozen member.
    pub fn ternary_library(work: &mut PriceWork) -> Result<Vec<Self>, String> {
        let mut out = Vec::with_capacity(3usize.pow(EVENT_COUNT as u32));
        for a in -1..=1 {
            for b in -1..=1 {
                for c in -1..=1 {
                    for d in -1..=1 {
                        work.spend(WorkKind::CoefficientCandidate)?;
                        out.push(Self::new([a, b, c, d]));
                    }
                }
            }
        }
        Ok(out)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorkKind {
    Node,
    Field,
    Event,
    Moment,
    Inner,
    Policy,
    Authority,
    CoefficientCandidate,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PriceWork {
    pub limit: u64,
    pub nodes: u64,
    pub field_calls: u64,
    pub event_evaluations: u64,
    pub moment_terms: u64,
    pub inner_branches: u64,
    pub policy_calls: u64,
    pub authority_checks: u64,
    pub coefficient_candidates: u64,
}

impl PriceWork {
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            ..Self::default()
        }
    }

    pub fn spent(&self) -> u64 {
        self.nodes
            + self.field_calls
            + self.event_evaluations
            + self.moment_terms
            + self.inner_branches
            + self.policy_calls
            + self.authority_checks
            + self.coefficient_candidates
    }

    fn spend(&mut self, kind: WorkKind) -> Result<(), String> {
        if self.spent() >= self.limit {
            return Err("information-price work budget exhausted".into());
        }
        match kind {
            WorkKind::Node => self.nodes += 1,
            WorkKind::Field => self.field_calls += 1,
            WorkKind::Event => self.event_evaluations += 1,
            WorkKind::Moment => self.moment_terms += 1,
            WorkKind::Inner => self.inner_branches += 1,
            WorkKind::Policy => self.policy_calls += 1,
            WorkKind::Authority => self.authority_checks += 1,
            WorkKind::CoefficientCandidate => self.coefficient_candidates += 1,
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct OracleAction {
    pub tile: Domino,
    pub child: NodeId,
    /// Exact best lawful continuation mass after fixing this action.
    pub exact_makes: usize,
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    Terminal { success: bool },
    Hidden { edges: Vec<(Domino, NodeId)> },
    Focal { actions: Vec<OracleAction> },
}

#[derive(Clone, Debug)]
pub struct OracleNode {
    pub id: NodeId,
    pub state: State,
    /// Ids index the oracle's original world vector.  Repeated worlds have
    /// distinct ids and are intentionally not deduplicated.
    pub active_ids: Vec<usize>,
    pub exact_makes: usize,
    pub kind: NodeKind,
}

#[derive(Clone, Debug)]
pub struct CenterTable {
    scope: Arc<()>,
    basis_id: String,
    field_id: String,
    root_identity: u64,
    prior_digest: u64,
    coefficients: [i64; EVENT_COUNT],
    values: BTreeMap<(NodeId, usize), [BigRational; EVENT_COUNT]>,
}

impl CenterTable {
    pub fn center(&self, node: NodeId, action: Domino, event: usize) -> Option<&BigRational> {
        self.values
            .get(&(node, action.index()))
            .and_then(|centers| centers.get(event))
    }
}

#[derive(Clone, Debug)]
pub struct ActionBounds {
    pub tile: Domino,
    pub child: NodeId,
    pub exact_lawful_q: BigRational,
    pub lawful_program_q: Option<BigRational>,
    pub perfect_information_upper: BigRational,
    pub priced_upper: Option<BigRational>,
}

impl ActionBounds {
    /// The certified cost upper from this action's supplied lawful lower and
    /// the largest available action upper at the same information state.
    pub fn cost_upper(actions: &[Self], action: Domino) -> Option<BigRational> {
        let lower = actions
            .iter()
            .find(|row| row.tile == action)?
            .lawful_program_q
            .clone()?;
        let best_upper = actions
            .iter()
            .map(|row| {
                row.priced_upper
                    .clone()
                    .unwrap_or_else(|| row.perfect_information_upper.clone())
            })
            .max()?;
        Some((best_upper - lower).clamp(BigRational::zero(), BigRational::one()))
    }
}

/// Complete finite information tree under one immutable root, prior, and field.
pub struct FiniteOracle {
    scope: Arc<()>,
    field_id: String,
    root_identity: u64,
    position: crate::solver::adaptive::RootPosition,
    viewer: Seat,
    worlds: Vec<World>,
    prior_digest: u64,
    authority: PriorAuthority,
    nodes: Vec<OracleNode>,
    by_history: BTreeMap<Vec<u8>, NodeId>,
}

impl FiniteOracle {
    pub fn from_worlds(
        root: &ExerciseRoot,
        field: &dyn SlicePolicy,
        worlds: Vec<World>,
        work: &mut PriceWork,
    ) -> Result<Self, String> {
        Self::build(root, field, worlds, false, work)
    }

    pub fn exact_root_fiber(
        root: &ExerciseRoot,
        field: &dyn SlicePolicy,
        max_worlds: usize,
        work: &mut PriceWork,
    ) -> Result<Self, String> {
        let count = usize::try_from(root.root.count()).map_err(|e| e.to_string())?;
        if count > max_worlds {
            return Err(format!("root fiber cap: {count} > {max_worlds}"));
        }
        let worlds = root.root.worlds().collect::<Vec<_>>();
        Self::build(root, field, worlds, true, work)
    }

    fn build(
        root: &ExerciseRoot,
        field: &dyn SlicePolicy,
        worlds: Vec<World>,
        exact_claim: bool,
        work: &mut PriceWork,
    ) -> Result<Self, String> {
        if worlds.is_empty() {
            return Err("empty finite prior".into());
        }
        if root.root.kernel().viewer().team() != root.position.declaring_team {
            return Err("finite teacher currently requires a declaring-team viewer".into());
        }
        for world in &worlds {
            work.spend(WorkKind::Authority)?;
            if !root.root.kernel().contains(world) {
                return Err("finite prior contains a world outside root support".into());
            }
        }
        let authority = if exact_claim {
            if usize::try_from(root.root.count()).map_err(|e| e.to_string())? != worlds.len() {
                return Err("exact-root-fiber authority has the wrong world count".into());
            }
            for (expected, supplied) in root.root.worlds().zip(&worlds) {
                work.spend(WorkKind::Authority)?;
                if &expected != supplied {
                    return Err(
                        "exact-root-fiber authority requires the complete ordered root enumeration"
                            .into(),
                    );
                }
            }
            PriorAuthority::ExactRootFiber {
                worlds: worlds.len(),
            }
        } else {
            PriorAuthority::FiniteEmpirical {
                samples: worlds.len(),
            }
        };
        let prior_digest = prior_digest(&worlds);
        let mut oracle = Self {
            scope: Arc::new(()),
            field_id: field.id().to_owned(),
            root_identity: root.position.identity(),
            position: root.position.clone(),
            viewer: root.root.kernel().viewer(),
            worlds,
            prior_digest,
            authority,
            nodes: Vec::new(),
            by_history: BTreeMap::new(),
        };
        let ids = (0..oracle.worlds.len()).collect::<Vec<_>>();
        oracle.build_node(State::from_root(&root.position), ids, field, work)?;
        Ok(oracle)
    }

    pub fn authority(&self) -> PriorAuthority {
        self.authority
    }

    pub fn field_id(&self) -> &str {
        &self.field_id
    }

    pub fn worlds(&self) -> &[World] {
        &self.worlds
    }

    pub fn nodes(&self) -> &[OracleNode] {
        &self.nodes
    }

    pub fn node(&self, id: NodeId) -> Option<&OracleNode> {
        self.nodes.get(id)
    }

    pub fn node_for_history(&self, history: &[Domino]) -> Option<&OracleNode> {
        let key = history.iter().map(|d| d.index() as u8).collect::<Vec<_>>();
        self.by_history.get(&key).and_then(|id| self.node(*id))
    }

    pub fn decision(&self, history: &[Domino]) -> Result<&OracleNode, String> {
        let node = self
            .node_for_history(history)
            .ok_or_else(|| "history is absent from this root-scoped finite tree".to_owned())?;
        if !matches!(node.kind, NodeKind::Focal { .. }) {
            return Err("history does not end at a focal decision".into());
        }
        Ok(node)
    }

    /// Compute exact full-information-state centers for one frozen shared
    /// coefficient program.  No caller-supplied or coarsened centers enter.
    pub fn centers(
        &self,
        program: &PriceProgram,
        work: &mut PriceWork,
    ) -> Result<CenterTable, String> {
        self.check_program(program)?;
        let mut values = BTreeMap::new();
        for node in &self.nodes {
            let NodeKind::Focal { actions } = &node.kind else {
                continue;
            };
            for action in actions {
                let mut sums: [BigInt; EVENT_COUNT] = std::array::from_fn(|_| BigInt::zero());
                for &id in &node.active_ids {
                    let features = self.features(node, action.tile, id, work)?;
                    for (sum, feature) in sums.iter_mut().zip(features) {
                        work.spend(WorkKind::Moment)?;
                        *sum += feature;
                    }
                }
                let den = BigInt::from(node.active_ids.len());
                values.insert(
                    (node.id, action.tile.index()),
                    std::array::from_fn(|j| BigRational::new(sums[j].clone(), den.clone())),
                );
            }
        }
        Ok(CenterTable {
            scope: Arc::clone(&self.scope),
            basis_id: EVENT_BASIS_ID.to_owned(),
            field_id: self.field_id.clone(),
            root_identity: self.root_identity,
            prior_digest: self.prior_digest,
            coefficients: program.coefficients,
            values,
        })
    }

    /// Recompute the defining conditional-zero equations.  This is an exact
    /// verifier, not a sampled tolerance check.
    pub fn audit_centers(
        &self,
        program: &PriceProgram,
        centers: &CenterTable,
        work: &mut PriceWork,
    ) -> Result<(), String> {
        self.check_centers(program, centers)?;
        for node in &self.nodes {
            let NodeKind::Focal { actions } = &node.kind else {
                continue;
            };
            for action in actions {
                let center = centers
                    .values
                    .get(&(node.id, action.tile.index()))
                    .ok_or_else(|| "missing full-information-state center".to_owned())?;
                let mut residuals: [BigRational; EVENT_COUNT] =
                    std::array::from_fn(|_| BigRational::zero());
                for &world_id in &node.active_ids {
                    let features = self.features(node, action.tile, world_id, work)?;
                    for j in 0..EVENT_COUNT {
                        work.spend(WorkKind::Moment)?;
                        residuals[j] +=
                            BigRational::from_integer(features[j].clone()) - center[j].clone();
                    }
                }
                if residuals.iter().any(|residual| !residual.is_zero()) {
                    return Err("center table failed an exact conditional-zero equation".into());
                }
            }
        }
        Ok(())
    }

    /// All legal root-action values at an actual tree information state.
    /// Exact Q is retained as a truth audit.  The two relaxed uppers fix the
    /// queried action before entering their recursions.
    pub fn action_values_for_history(
        &self,
        history: &[Domino],
        priced: Option<(&PriceProgram, &CenterTable)>,
        lawful: Option<&dyn SlicePolicy>,
        work: &mut PriceWork,
    ) -> Result<Vec<ActionBounds>, String> {
        let node = self.decision(history)?;
        let NodeKind::Focal { actions } = &node.kind else {
            unreachable!()
        };
        if let Some((program, centers)) = priced {
            self.check_centers(program, centers)?;
        }
        let den = BigInt::from(node.active_ids.len());
        let mut out = Vec::with_capacity(actions.len());
        for action in actions {
            let exact = BigRational::new(BigInt::from(action.exact_makes), den.clone());
            let perfect = self.relaxed_action_upper(node, action, None, work)?;
            let price = match priced {
                Some((program, centers)) => {
                    let value =
                        self.relaxed_action_upper(node, action, Some((program, centers)), work)?;
                    if value < exact {
                        return Err("priced result failed exact lawful-Q upper audit".into());
                    }
                    Some(value.min(perfect.clone()).min(BigRational::one()))
                }
                None => None,
            };
            let lawful_program_q = lawful
                .map(|policy| self.lawful_action_value(node, action, policy, work))
                .transpose()?;
            out.push(ActionBounds {
                tile: action.tile,
                child: action.child,
                exact_lawful_q: exact,
                lawful_program_q,
                perfect_information_upper: perfect,
                priced_upper: price,
            });
        }
        Ok(out)
    }

    fn build_node(
        &mut self,
        state: State,
        active_ids: Vec<usize>,
        field: &dyn SlicePolicy,
        work: &mut PriceWork,
    ) -> Result<NodeId, String> {
        work.spend(WorkKind::Node)?;
        if active_ids.is_empty() {
            return Err("internal empty posterior".into());
        }
        let key = history_key(&state.history);
        if self.by_history.contains_key(&key) {
            return Err("public history appeared twice in a tree".into());
        }
        let id = self.nodes.len();
        self.by_history.insert(key, id);
        self.nodes.push(OracleNode {
            id,
            state: state.clone(),
            active_ids: active_ids.clone(),
            exact_makes: 0,
            kind: NodeKind::Terminal { success: false },
        });

        let (kind, exact_makes) = if let Some(success) = state.success(&self.position) {
            (
                NodeKind::Terminal { success },
                if success { active_ids.len() } else { 0 },
            )
        } else {
            let actor = state.actor();
            let led = state
                .prefix
                .first()
                .map(|d| self.position.decl.led_context(*d));
            if actor == self.viewer {
                let hand = self.worlds[active_ids[0]]
                    .hand(actor)
                    .difference(state.played);
                if active_ids.iter().any(|&world_id| {
                    self.worlds[world_id].hand(actor).difference(state.played) != hand
                }) {
                    return Err("focal information state has inconsistent own hands".into());
                }
                let legal = legal_plays(self.position.decl, hand, led);
                if legal.is_empty() {
                    return Err("focal decision has no legal action".into());
                }
                let mut actions = Vec::new();
                for tile in legal {
                    let child = self.build_node(
                        state.step(self.position.decl, tile),
                        active_ids.clone(),
                        field,
                        work,
                    )?;
                    actions.push(OracleAction {
                        tile,
                        child,
                        exact_makes: self.nodes[child].exact_makes,
                    });
                }
                let exact = actions.iter().map(|a| a.exact_makes).max().unwrap();
                (NodeKind::Focal { actions }, exact)
            } else {
                let mut branches: BTreeMap<Domino, Vec<usize>> = BTreeMap::new();
                let record = state.record(&self.position);
                for &world_id in &active_ids {
                    let hand = self.worlds[world_id].hand(actor).difference(state.played);
                    let legal = legal_plays(self.position.decl, hand, led);
                    if legal.is_empty() {
                        return Err("hidden field decision has no legal action".into());
                    }
                    work.spend(WorkKind::Field)?;
                    let tile = field.choose(self.position.decl, hand, legal, &record);
                    if !legal.contains(tile) {
                        return Err("immutable field selected an illegal action".into());
                    }
                    branches.entry(tile).or_default().push(world_id);
                }
                let mut edges = Vec::with_capacity(branches.len());
                let mut exact = 0;
                for (tile, ids) in branches {
                    let child =
                        self.build_node(state.step(self.position.decl, tile), ids, field, work)?;
                    exact += self.nodes[child].exact_makes;
                    edges.push((tile, child));
                }
                (NodeKind::Hidden { edges }, exact)
            }
        };
        self.nodes[id].kind = kind;
        self.nodes[id].exact_makes = exact_makes;
        Ok(id)
    }

    fn features(
        &self,
        node: &OracleNode,
        action: Domino,
        world_id: usize,
        work: &mut PriceWork,
    ) -> Result<[BigInt; EVENT_COUNT], String> {
        let world = self
            .worlds
            .get(world_id)
            .ok_or_else(|| "invalid world id".to_owned())?;
        let partner = self.viewer.plus(2);
        let hand = world.hand(partner).difference(node.state.played);
        let mut has_five = false;
        let mut has_ten = false;
        let context = self.position.decl.led_context(action);
        let incidence = self.position.decl.effective_incidence(context);
        let mut coholds_context = false;
        let mut count_in_context = false;
        for tile in hand {
            has_five |= tile.count() == 5;
            has_ten |= tile.count() == 10;
            coholds_context |= incidence.contains(tile);
            count_in_context |= tile.count() > 0 && incidence.contains(tile);
        }
        let flags = [has_five, has_ten, coholds_context, count_in_context];
        for _ in 0..EVENT_COUNT {
            work.spend(WorkKind::Event)?;
        }
        Ok(flags.map(|flag| BigInt::from(u8::from(flag))))
    }

    /// Native values of the versioned Scheme event basis, exposed for exact
    /// parity checks and examiner diagnostics.
    pub fn event_values(
        &self,
        node_id: NodeId,
        action: Domino,
        world_id: usize,
        work: &mut PriceWork,
    ) -> Result<[bool; EVENT_COUNT], String> {
        let node = self
            .node(node_id)
            .ok_or_else(|| "invalid oracle node id".to_owned())?;
        let values = self.features(node, action, world_id, work)?;
        Ok(values.map(|value| !value.is_zero()))
    }

    fn check_program(&self, program: &PriceProgram) -> Result<(), String> {
        if program.basis_id != EVENT_BASIS_ID {
            return Err("price program uses the wrong event-basis version".into());
        }
        Ok(())
    }

    fn check_centers(&self, program: &PriceProgram, centers: &CenterTable) -> Result<(), String> {
        self.check_program(program)?;
        if !Arc::ptr_eq(&centers.scope, &self.scope)
            || centers.basis_id != EVENT_BASIS_ID
            || centers.field_id != self.field_id
            || centers.root_identity != self.root_identity
            || centers.prior_digest != self.prior_digest
            || centers.coefficients != program.coefficients
        {
            return Err(
                "center table has the wrong root, field, prior, basis, or coefficients".into(),
            );
        }
        Ok(())
    }

    fn price(
        &self,
        node: &OracleNode,
        action: Domino,
        world_id: usize,
        program: &PriceProgram,
        centers: &CenterTable,
        work: &mut PriceWork,
    ) -> Result<BigRational, String> {
        let center = centers
            .values
            .get(&(node.id, action.index()))
            .ok_or_else(|| "missing full-information-state center".to_owned())?;
        let features = self.features(node, action, world_id, work)?;
        let mut result = BigRational::zero();
        for j in 0..EVENT_COUNT {
            work.spend(WorkKind::Moment)?;
            result += BigRational::from_integer(BigInt::from(program.coefficients[j]))
                * (BigRational::from_integer(features[j].clone()) - center[j].clone());
        }
        Ok(result)
    }

    fn relaxed_action_upper(
        &self,
        node: &OracleNode,
        action: &OracleAction,
        priced: Option<(&PriceProgram, &CenterTable)>,
        work: &mut PriceWork,
    ) -> Result<BigRational, String> {
        let mut memo = HashMap::new();
        let mut total = BigRational::zero();
        // The action has already been imposed.  Deliberately do not subtract a
        // price at `node`; only future focal decisions are priced.
        for &world_id in &node.active_ids {
            total += self.relaxed_world(action.child, world_id, priced, &mut memo, work)?;
        }
        Ok(total / BigInt::from(node.active_ids.len()))
    }

    fn relaxed_world(
        &self,
        node_id: NodeId,
        world_id: usize,
        priced: Option<(&PriceProgram, &CenterTable)>,
        memo: &mut HashMap<(NodeId, usize), BigRational>,
        work: &mut PriceWork,
    ) -> Result<BigRational, String> {
        if let Some(value) = memo.get(&(node_id, world_id)) {
            return Ok(value.clone());
        }
        let node = self
            .node(node_id)
            .ok_or_else(|| "invalid oracle node id".to_owned())?;
        if !node.active_ids.contains(&world_id) {
            return Err("world id is not active at the requested node".into());
        }
        work.spend(WorkKind::Inner)?;
        let value = match &node.kind {
            NodeKind::Terminal { success } => BigRational::from_integer(BigInt::from(*success)),
            NodeKind::Hidden { edges } => {
                let child = edges
                    .iter()
                    .map(|(_, child)| *child)
                    .find(|child| self.nodes[*child].active_ids.contains(&world_id))
                    .ok_or_else(|| "hidden branch lost an active world".to_owned())?;
                self.relaxed_world(child, world_id, priced, memo, work)?
            }
            NodeKind::Focal { actions } => {
                let mut best: Option<BigRational> = None;
                for action in actions {
                    work.spend(WorkKind::Inner)?;
                    let mut candidate =
                        self.relaxed_world(action.child, world_id, priced, memo, work)?;
                    if let Some((program, centers)) = priced {
                        candidate -=
                            self.price(node, action.tile, world_id, program, centers, work)?;
                    }
                    if best.as_ref().is_none_or(|value| candidate > *value) {
                        best = Some(candidate);
                    }
                }
                best.ok_or_else(|| "focal relaxation has no action".to_owned())?
            }
        };
        memo.insert((node_id, world_id), value.clone());
        Ok(value)
    }

    fn lawful_action_value(
        &self,
        node: &OracleNode,
        action: &OracleAction,
        policy: &dyn SlicePolicy,
        work: &mut PriceWork,
    ) -> Result<BigRational, String> {
        let makes = self.lawful_node(action.child, policy, work)?;
        Ok(BigRational::new(
            BigInt::from(makes),
            BigInt::from(node.active_ids.len()),
        ))
    }

    fn lawful_node(
        &self,
        node_id: NodeId,
        policy: &dyn SlicePolicy,
        work: &mut PriceWork,
    ) -> Result<usize, String> {
        let node = &self.nodes[node_id];
        match &node.kind {
            NodeKind::Terminal { success } => Ok(if *success { node.active_ids.len() } else { 0 }),
            NodeKind::Hidden { edges } => edges
                .iter()
                .map(|(_, child)| self.lawful_node(*child, policy, work))
                .try_fold(0usize, |sum, value| value.map(|v| sum + v)),
            NodeKind::Focal { actions } => {
                let hand = self.worlds[node.active_ids[0]]
                    .hand(self.viewer)
                    .difference(node.state.played);
                let led = node
                    .state
                    .prefix
                    .first()
                    .map(|d| self.position.decl.led_context(*d));
                let legal = legal_plays(self.position.decl, hand, led);
                work.spend(WorkKind::Policy)?;
                let tile = policy.choose(
                    self.position.decl,
                    hand,
                    legal,
                    &node.state.record(&self.position),
                );
                if !legal.contains(tile) {
                    return Err("supplied lawful program selected an illegal action".into());
                }
                let child = actions
                    .iter()
                    .find(|action| action.tile == tile)
                    .map(|action| action.child)
                    .ok_or_else(|| "policy action is absent from complete focal node".to_owned())?;
                self.lawful_node(child, policy, work)
            }
        }
    }
}

fn history_key(history: &[Domino]) -> Vec<u8> {
    history.iter().map(|d| d.index() as u8).collect()
}

fn prior_digest(worlds: &[World]) -> u64 {
    let mut digest = 0x5052_494f_525f_5631u64;
    for (id, world) in worlds.iter().enumerate() {
        digest = crate::solver::mix(digest ^ id as u64);
        for hand in world.hands() {
            digest = crate::solver::mix(digest ^ u64::from(hand.bits()));
        }
    }
    digest
}
