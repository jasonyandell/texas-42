//! The focal-horizon hierarchy, slice FH1 — the parent's §28 generic
//! fixed-field engine (`walt/math/focal_horizon_sandwich_v0.1.md`, cited
//! by title only; the construction is the FOCAL-HORIZON HIERARCHY, FH-A2)
//! as narrowed by its companion and the rulings FH-A1..A11 in
//! `walt/CENSUS-RULINGS.md`.
//!
//! THE OBJECT (§7–§8, exact-mass form §22). For a belief `B`, a horizon
//! `k`, a lawful lower tail `π` driving the viewer seat, and the God
//! upper tail, the focal-horizon interval `[L_k(B), U_k(B)]` in mass form
//! `M^L_k = Z·L_k`, `M^U_k = Z·U_k`, by structural recursion:
//!
//! - decided (the §5 arithmetic through [`decided_success`] — the SAME
//!   predicate everywhere, FH-A6): both `u·Z`;
//! - focal, `k = 0`: `M^L_0 = viewer_success_mass(π)` (the Slice D
//!   fixed-policy recursion, reused) and `M^U_0 = Z − doomed(B)` (the
//!   undoomed-world count through the doom census's OWN line walk,
//!   [`doom_over_belief`] — Proposition FH-God: this IS `G`);
//! - focal, `k ≥ 1`: `max_a` over the FULL legal set of the children at
//!   `k − 1` (every child shares `Z`; every legal action, always — §41,
//!   FH-int), argmax recorded under `TieRule::LowestTileIndex` as the
//!   materialized policy `π_k`'s choice; a FORCED focal node consumes a
//!   unit like any other (FH-A6, binding);
//! - hidden (modeled seat): `Σ_t` over every positive-mass branch of the
//!   conditioned child at the SAME `k` — public observations consume no
//!   horizon (§6).
//!
//! THE ROOT (§16): per legal root action `a`, `[L_{a,k}, U_{a,k}]` on
//! `B_0 a` where `k` counts ADDITIONAL focal layers after the root action;
//! bar `B_k = max_a L_{a,k}`; survivor set `S_k = {a : U_{a,k} ≥ B_k}`;
//! verdict `Settled{b}` iff `L_{b,k} > max_{a≠b} U_{a,k}` (§18),
//! `Equivalent` iff every survivor is collapsed (Proposition FH-tie),
//! else `Unresolved`. `π_k` is a total policy: the argmax table keyed by
//! post-root history for the first `k` layers and THE TAIL `π` below and
//! off-DAG (FH-A7). `L_exec = V(π_k) = B_k`, `U*_k = max_a U_{a,k}`,
//! `Γ_k = U*_k − L_exec` (§19).
//!
//! AFFORDABLE-OR-REFUSE (FH-A11, binding for FH1). A frontier node whose
//! God enumeration is unaffordable (`Z > node_fiber_cap`) refuses the
//! WHOLE root with the boundary named; nothing partial is returned and
//! the trivial upper is never installed as a fact (FH-A3). Budgeted runs
//! with retained intervals are slice FH2, gated on Proposition FH-int.
//!
//! IDENTITY (FH-A4): the σ0 tail reads the bid, so a lower is per (root,
//! contract, field, tail id, k) and is never projected across contracts;
//! the result carries all five.
//!
//! COMPOSE, NEVER COPY: [`viewer_success_mass`], [`decided_success`],
//! [`legal_plays`], the oracle's `branch_masses`/`condition`, and the doom
//! line walk are the authorities; this module holds no second
//! implementation of any of them. Exact integers throughout; the rational
//! appears only at the report. EXPLORATORY tier — below every evidentiary
//! tier, cited by nothing above it.

use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{legal_plays, Decl, Domino, DominoSet};
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{
    viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
};
use crate::solver::horizon::{doom_over_belief, CountingField};
use crate::solver::policy::content_digest;

/// The declared engine parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocalSpec {
    /// `k` — additional focal layers after the root action.
    pub horizon: usize,
    /// A frontier node whose fiber exceeds this refuses the whole root.
    pub node_fiber_cap: u128,
}

/// The five-coordinate identity of one result (FH-A4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocalIdentity {
    pub root_id: u64,
    pub field_id: String,
    pub contract: u32,
    pub tail_id: String,
    pub horizon: usize,
}

/// One root action's focal-horizon interval, in mass form and as exact
/// rationals over the shared root mass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionInterval {
    pub action: Domino,
    /// `Z·L_{a,k}`.
    pub lower_mass: u128,
    /// `Z·U_{a,k}`.
    pub upper_mass: u128,
    /// `Z` — shared by every root action (a focal play changes no factor).
    pub root_mass: u128,
    pub lower: BigRational,
    pub upper: BigRational,
}

impl ActionInterval {
    /// `U_{a,k} − L_{a,k}`.
    pub fn width(&self) -> BigRational {
        &self.upper - &self.lower
    }

    /// Collapsed at this horizon: `L_{a,k} = U_{a,k}` (then `= Q_a`).
    pub fn collapsed(&self) -> bool {
        self.lower_mass == self.upper_mass
    }
}

/// The root verdict (§18, Proposition FH-tie).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FocalVerdict {
    /// `L_{b,k} > max_{a≠b} U_{a,k}`: `b` is the unique exact optimal
    /// root action.
    Settled { action: Domino },
    /// Every survivor is collapsed: the exact optimal set is
    /// `{a ∈ S_k : Q_a = B_k}`, listed here, and `Q* = B_k`.
    Equivalent {
        actions: Vec<Domino>,
        value_mass: u128,
    },
    /// Some survivor is not collapsed; the survivor set is a superset of
    /// the exact optimal set.
    Unresolved { survivors: Vec<Domino> },
}

/// The materialized lower policy `π_k`'s choice table: the argmax under
/// `TieRule::LowestTileIndex` at the root (key: the empty history) and at
/// every focal node of the first `k` post-root layers on the DAG
/// reachable under those choices, keyed by post-root history. Below the
/// `k`-th layer and off the DAG the policy IS the tail (FH-A7) — which is
/// why this table is not itself a `SlicePolicy`: bind it to its tail with
/// [`FocalChoices::with_tail`] to obtain the total policy. The id is a
/// content address over the table and the tail id
/// (`focal-k<k>-<tail id>-<hex>`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocalChoices {
    id: String,
    horizon: usize,
    tail_id: String,
    choices: BTreeMap<Vec<u8>, Domino>,
}

impl FocalChoices {
    fn new(horizon: usize, tail_id: &str, choices: BTreeMap<Vec<u8>, Domino>) -> FocalChoices {
        let mut bytes = Vec::new();
        for (k, v) in &choices {
            bytes.extend_from_slice(k);
            bytes.push(0xff);
            bytes.push(u8::try_from(v.index()).expect("a tile index fits u8"));
            bytes.push(0xfe);
        }
        bytes.push(0xfd);
        bytes.extend_from_slice(tail_id.as_bytes());
        let digest = content_digest(&bytes);
        let mut hex = String::new();
        for b in &digest[..16] {
            hex.push_str(&format!("{b:02x}"));
        }
        FocalChoices {
            id: format!("focal-k{horizon}-{tail_id}-{hex}"),
            horizon,
            tail_id: tail_id.to_string(),
            choices,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn horizon(&self) -> usize {
        self.horizon
    }

    pub fn tail_id(&self) -> &str {
        &self.tail_id
    }

    /// Recorded focal states — the DAG's size (the root included).
    pub fn states(&self) -> usize {
        self.choices.len()
    }

    /// The recorded choice at one post-root history, when on the DAG.
    pub fn choice_at(&self, history: &[Domino]) -> Option<Domino> {
        self.choices.get(&history_key(history)).copied()
    }

    /// The total policy `π_k`: this table on the DAG, `tail` everywhere
    /// else. The tail must be the one the table was materialized over.
    pub fn with_tail<'a>(&'a self, tail: &'a dyn SlicePolicy) -> FocalPolicy<'a> {
        assert_eq!(
            tail.id(),
            self.tail_id,
            "π_k is completed by the tail it was materialized over (FH-A7)"
        );
        FocalPolicy {
            choices: self,
            tail,
        }
    }
}

/// `π_k` as a total, deterministic, information-consistent
/// [`SlicePolicy`]: the recorded argmax on the DAG, the tail below the
/// `k`-th layer and off the DAG.
pub struct FocalPolicy<'a> {
    choices: &'a FocalChoices,
    tail: &'a dyn SlicePolicy,
}

impl SlicePolicy for FocalPolicy<'_> {
    fn id(&self) -> &str {
        &self.choices.id
    }

    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        if let Some(d) = self.choices.choices.get(&history_key(record.history)) {
            assert!(
                legal.contains(*d),
                "a recorded choice was legal at materialization and legality is a \
                 function of the same public state and hand"
            );
            return *d;
        }
        self.tail.choose(decl, hand, legal, record)
    }
}

fn history_key(history: &[Domino]) -> Vec<u8> {
    history
        .iter()
        .map(|d| u8::try_from(d.index()).expect("a tile index fits u8"))
        .collect()
}

/// Exact integer counters of one run.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FocalSpend {
    /// Consultations of the declared field, in its FIELD role.
    pub field_reads: u64,
    /// Consultations of the lower tail, in its FOCAL role (under the σ0
    /// tail these are σ0 reads too, counted apart from the field's).
    pub tail_reads: u64,
    /// Posterior updates performed by the hierarchy's own hidden nodes.
    pub conditionings: u64,
    /// Focal nodes of the first `k` layers (the max nodes).
    pub focal_nodes: u64,
    /// Hidden nodes walked by the hierarchy itself.
    pub hidden_nodes: u64,
    pub decided_early: u64,
    pub decided_terminal: u64,
    /// Lower-tail evaluations: `k = 0` undecided focal nodes.
    pub lower_tail_evaluations: u64,
    /// Upper-tail evaluations: the same nodes, priced by God enumeration.
    pub upper_tail_evaluations: u64,
    /// Of those frontier nodes, the ones with a singleton legal set — a
    /// forced node consults the tails trivially (Proposition FH-last).
    pub forced_tail_evaluations: u64,
    /// Worlds enumerated across every upper-tail evaluation.
    pub worlds_enumerated: u128,
    /// Line-walk nodes across every upper-tail evaluation.
    pub line_walk_nodes: u64,
    /// The `k`-th focal frontier by post-root depth: tail consultations
    /// per ply (FH-A11's ply distribution).
    pub tail_plies: BTreeMap<usize, u64>,
    /// The lower tail's own recursion counters, summed over evaluations.
    pub lower_tail_stats: RecursionStats,
}

impl FocalSpend {
    /// Lower plus upper tail evaluations.
    pub fn tail_consultations(&self) -> u64 {
        self.lower_tail_evaluations + self.upper_tail_evaluations
    }
}

/// The completed hierarchy at one root, one horizon.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocalHorizonResult {
    pub identity: FocalIdentity,
    pub spec: FocalSpec,
    /// Root actions in tile order.
    pub actions: Vec<ActionInterval>,
    /// `Z·B_k = max_a lower_mass`.
    pub bar_mass: u128,
    /// `S_k` in tile order.
    pub survivors: Vec<Domino>,
    pub verdict: FocalVerdict,
    /// `π_k`'s choice table; bind with the tail for the total policy.
    pub policy: FocalChoices,
    /// `Z·L_exec = Z·V(π_k)` — the bar, by construction (gate FH5 checks
    /// it through the independent evaluator).
    pub executable_lower_mass: u128,
    /// `Z·U*_k = max_a upper_mass`.
    pub global_upper_mass: u128,
    /// `Γ_k = U*_k − L_exec`, exact.
    pub certified_regret: BigRational,
    pub spend: FocalSpend,
}

impl FocalHorizonResult {
    pub fn root_mass(&self) -> u128 {
        self.actions.first().map_or(0, |a| a.root_mass)
    }

    pub fn bar(&self) -> BigRational {
        ratio(self.bar_mass, self.root_mass())
    }

    pub fn global_upper(&self) -> BigRational {
        ratio(self.global_upper_mass, self.root_mass())
    }

    /// The root action `π_k` plays — the bar's argmax under the lowest
    /// tile rule.
    pub fn policy_action(&self) -> Domino {
        self.policy
            .choice_at(&[])
            .expect("π_k records the root choice")
    }

    pub fn interval(&self, action: Domino) -> Option<&ActionInterval> {
        self.actions.iter().find(|a| a.action == action)
    }
}

/// The typed whole-root refusal (FH-A11): no intervals, no verdict, no
/// regret travel with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FocalRefusal {
    /// A `k`-th-frontier focal node's God enumeration exceeds the cap.
    UpperUnaffordable {
        /// The post-root public history of the refusing node.
        history: Vec<Domino>,
        fiber: u128,
        cap: u128,
    },
}

fn ratio(m: u128, z: u128) -> BigRational {
    BigRational::new(BigInt::from(m), BigInt::from(z))
}

// ---------------------------------------------------------------------------
// The recursion.
// ---------------------------------------------------------------------------

/// One subtree's exact-mass interval and the choice table of `π_k`
/// through it.
struct Node {
    lower: u128,
    upper: u128,
    choices: BTreeMap<Vec<u8>, Domino>,
}

struct Engine<'a> {
    oracle: &'a dyn ExactCoverOracle,
    /// The declared field, counted.
    field: &'a CountingField<'a>,
    /// The lower tail, counted.
    tail: &'a CountingField<'a>,
    cap: u128,
    total_plays: usize,
    spend: FocalSpend,
}

impl Engine<'_> {
    fn walk(&mut self, belief: &FactorBelief, k: usize) -> Result<Node, FocalRefusal> {
        let viewer = belief.kernel().viewer();
        let state = belief.public_state();
        let depth = belief.history().len();
        let at_terminal = depth == self.total_plays;
        if let Some(u) = decided_success(belief.position(), viewer, state.banked, at_terminal) {
            if at_terminal {
                self.spend.decided_terminal += 1;
            } else {
                self.spend.decided_early += 1;
            }
            let z = self.oracle.mass(belief);
            let v = if u { z } else { 0 };
            return Ok(Node {
                lower: v,
                upper: v,
                choices: BTreeMap::new(),
            });
        }
        assert!(
            depth < self.total_plays,
            "an undecided state has plays left (rules invariant)"
        );
        let seat = state.leader.plus(state.plays.len());
        if seat != viewer {
            // Hidden: Σ over every positive-mass branch at the SAME k.
            self.spend.hidden_nodes += 1;
            let mut lower: u128 = 0;
            let mut upper: u128 = 0;
            let mut choices: BTreeMap<Vec<u8>, Domino> = BTreeMap::new();
            for (tile, _) in self.oracle.branch_masses(belief, self.field) {
                self.spend.conditionings += 1;
                let child = self.oracle.condition(belief, tile, self.field);
                let node = self.walk(&child, k)?;
                lower = lower
                    .checked_add(node.lower)
                    .expect("an exact mass fits u128");
                upper = upper
                    .checked_add(node.upper)
                    .expect("an exact mass fits u128");
                for (key, v) in node.choices {
                    let prior = choices.insert(key, v);
                    assert!(prior.is_none(), "hidden branches extend disjoint histories");
                }
            }
            return Ok(Node {
                lower,
                upper,
                choices,
            });
        }
        let remaining = belief
            .kernel()
            .viewer_hand()
            .difference(state.played_by[viewer.index()]);
        let led = state
            .plays
            .first()
            .map(|d| belief.position().decl.led_context(*d));
        let legal = legal_plays(belief.position().decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        if k == 0 {
            // The k-th focal frontier: both tails, or a whole-root refusal.
            let z = self.oracle.mass(belief);
            if z > self.cap {
                return Err(FocalRefusal::UpperUnaffordable {
                    history: belief.history().to_vec(),
                    fiber: z,
                    cap: self.cap,
                });
            }
            self.spend.lower_tail_evaluations += 1;
            self.spend.upper_tail_evaluations += 1;
            if legal.len() == 1 {
                self.spend.forced_tail_evaluations += 1;
            }
            *self.spend.tail_plies.entry(depth).or_insert(0) += 1;
            let mut rs = RecursionStats::default();
            let lower = viewer_success_mass(self.oracle, belief, self.tail, self.field, &mut rs);
            self.spend.lower_tail_stats.decided_early += rs.decided_early;
            self.spend.lower_tail_stats.decided_terminal += rs.decided_terminal;
            self.spend.lower_tail_stats.focal_nodes += rs.focal_nodes;
            self.spend.lower_tail_stats.hidden_nodes += rs.hidden_nodes;
            self.spend.lower_tail_stats.conditionings += rs.conditionings;
            let (doomed, worlds, nodes) = doom_over_belief(belief, self.field);
            assert_eq!(
                worlds, z,
                "the per-world enumeration covers the node's posterior mass exactly"
            );
            self.spend.worlds_enumerated = self
                .spend
                .worlds_enumerated
                .checked_add(worlds)
                .expect("an exact count fits u128");
            self.spend.line_walk_nodes = self.spend.line_walk_nodes.saturating_add(nodes);
            let upper = z - doomed;
            assert!(
                lower <= upper,
                "a lawful tail never exceeds the world-revealed upper (Lemma 0.3, FH-God)"
            );
            return Ok(Node {
                lower,
                upper,
                choices: BTreeMap::new(),
            });
        }
        // Focal with horizon left: max over EVERY legal action at k − 1;
        // argmax by strictly-greater replacement over ascending tile
        // iteration (TieRule::LowestTileIndex); only the argmax child's
        // table survives (the DAG reachable under π_k).
        self.spend.focal_nodes += 1;
        let mut best: Option<(u128, Domino, BTreeMap<Vec<u8>, Domino>)> = None;
        let mut upper: Option<u128> = None;
        for tile in legal.iter() {
            let node = self.walk(&belief.focal_play(tile), k - 1)?;
            upper = Some(upper.map_or(node.upper, |u| u.max(node.upper)));
            let take = match &best {
                None => true,
                Some((b, _, _)) => node.lower > *b,
            };
            if take {
                best = Some((node.lower, tile, node.choices));
            }
        }
        let (lower, tile, mut choices) = best.expect("a legal set holds an action");
        let prior = choices.insert(history_key(belief.history()), tile);
        assert!(prior.is_none(), "a history names one focal node");
        Ok(Node {
            lower,
            upper: upper.expect("a legal set holds an action"),
            choices,
        })
    }
}

/// The parent's §28 engine at a uniform root: the action-indexed
/// focal-horizon intervals at horizon `spec.horizon`, the survivor set,
/// the verdict, the materialized `π_k`, and the certified regret — or one
/// typed whole-root refusal.
pub fn focal_horizon(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    lower_tail: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
    spec: &FocalSpec,
) -> Result<FocalHorizonResult, FocalRefusal> {
    let counted_field = CountingField {
        inner: field,
        reads: std::cell::Cell::new(0),
    };
    let counted_tail = CountingField {
        inner: lower_tail,
        reads: std::cell::Cell::new(0),
    };
    let belief = FactorBelief::uniform_root(root, position, &counted_field);
    let z = oracle.mass(&belief);
    assert!(z > 0, "a root has positive belief mass");
    let kernel = root.kernel();
    let total_plays =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let mut engine = Engine {
        oracle,
        field: &counted_field,
        tail: &counted_tail,
        cap: spec.node_fiber_cap,
        total_plays,
        spend: FocalSpend::default(),
    };
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, kernel.viewer_hand(), led);
    assert!(!legal.is_empty(), "a root's viewer holds a legal tile");
    let mut actions: Vec<ActionInterval> = Vec::new();
    let mut choices: BTreeMap<Vec<u8>, Domino> = BTreeMap::new();
    // Ascending tile order — the tie rule's iteration order.
    for a in legal.iter() {
        let node = engine.walk(&belief.focal_play(a), spec.horizon)?;
        for (key, v) in node.choices {
            let prior = choices.insert(key, v);
            assert!(prior.is_none(), "root actions extend disjoint histories");
        }
        actions.push(ActionInterval {
            action: a,
            lower_mass: node.lower,
            upper_mass: node.upper,
            root_mass: z,
            lower: ratio(node.lower, z),
            upper: ratio(node.upper, z),
        });
    }
    let bar_mass = actions
        .iter()
        .map(|a| a.lower_mass)
        .max()
        .expect("a root action exists");
    let bar_action = actions
        .iter()
        .find(|a| a.lower_mass == bar_mass)
        .expect("the bar is attained")
        .action;
    let prior = choices.insert(history_key(&[]), bar_action);
    assert!(prior.is_none(), "the root is the empty history");
    let global_upper_mass = actions
        .iter()
        .map(|a| a.upper_mass)
        .max()
        .expect("a root action exists");
    let survivors: Vec<Domino> = actions
        .iter()
        .filter(|a| a.upper_mass >= bar_mass)
        .map(|a| a.action)
        .collect();
    let settled = actions.iter().find(|b| {
        actions
            .iter()
            .filter(|a| a.action != b.action)
            .all(|a| b.lower_mass > a.upper_mass)
    });
    let verdict = match settled {
        Some(b) => FocalVerdict::Settled { action: b.action },
        None => {
            let all_collapsed = actions
                .iter()
                .filter(|a| a.upper_mass >= bar_mass)
                .all(ActionInterval::collapsed);
            if all_collapsed {
                FocalVerdict::Equivalent {
                    actions: actions
                        .iter()
                        .filter(|a| a.upper_mass >= bar_mass && a.lower_mass == bar_mass)
                        .map(|a| a.action)
                        .collect(),
                    value_mass: bar_mass,
                }
            } else {
                FocalVerdict::Unresolved {
                    survivors: survivors.clone(),
                }
            }
        }
    };
    let mut spend = engine.spend;
    spend.field_reads = counted_field.reads.get();
    spend.tail_reads = counted_tail.reads.get();
    Ok(FocalHorizonResult {
        identity: FocalIdentity {
            root_id: root_identity(root, position),
            field_id: field.id().to_string(),
            contract: position.bid,
            tail_id: lower_tail.id().to_string(),
            horizon: spec.horizon,
        },
        spec: spec.clone(),
        actions,
        bar_mass,
        survivors,
        verdict,
        policy: FocalChoices::new(spec.horizon, lower_tail.id(), choices),
        executable_lower_mass: bar_mass,
        global_upper_mass,
        certified_regret: ratio(global_upper_mass - bar_mass, z),
        spend,
    })
}

// ---------------------------------------------------------------------------
// The focal depth (§6), an INDEPENDENT walk.
// ---------------------------------------------------------------------------

/// `h_f(B)` by the §6 definition, walked independently of the engine's
/// recursion: decided (the same [`decided_success`] cutoff, FH-A6) → 0;
/// focal → `1 + max_a h_f(Ba)` over every legal action, forced nodes
/// counting; hidden → `max_t h_f(B_t)` over every positive-mass branch.
pub fn focal_depth(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
) -> usize {
    let kernel = belief.kernel();
    let total_plays =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    depth_walk(oracle, belief, field, total_plays)
}

fn depth_walk(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
    total_plays: usize,
) -> usize {
    let viewer = belief.kernel().viewer();
    let state = belief.public_state();
    let depth = belief.history().len();
    if decided_success(
        belief.position(),
        viewer,
        state.banked,
        depth == total_plays,
    )
    .is_some()
    {
        return 0;
    }
    assert!(depth < total_plays, "an undecided state has plays left");
    let seat = state.leader.plus(state.plays.len());
    if seat == viewer {
        let remaining = belief
            .kernel()
            .viewer_hand()
            .difference(state.played_by[viewer.index()]);
        let led = state
            .plays
            .first()
            .map(|d| belief.position().decl.led_context(*d));
        let legal = legal_plays(belief.position().decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let mut best = 0usize;
        for tile in legal.iter() {
            best = best.max(depth_walk(
                oracle,
                &belief.focal_play(tile),
                field,
                total_plays,
            ));
        }
        1 + best
    } else {
        let mut best = 0usize;
        for (tile, _) in oracle.branch_masses(belief, field) {
            let child = oracle.condition(belief, tile, field);
            best = best.max(depth_walk(oracle, &child, field, total_plays));
        }
        best
    }
}
