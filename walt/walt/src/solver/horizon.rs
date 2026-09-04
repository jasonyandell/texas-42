//! The in-solve horizon census (slice U0b) — the §38/§40 God-gap census
//! of `walt/math/salvation_complex_v0.1.md`, run at EVERY belief node the
//! exact recursion reaches at a declared depth below a root, rather than
//! at root-action coordinates only.
//!
//! WHY. U0 measured the fusion horizon at fourteen trick-5/trick-6
//! receipt coordinates and twelve trick-4 ones: every t5/t6 coordinate
//! God-tight, every substantive t4 coordinate carrying a positive
//! information price. That is fourteen points. A trick-4 solve visits
//! hundreds to thousands of trick-5 belief nodes — the same recursion,
//! the same fixed field, conditioned posteriors instead of uniform roots
//! — and the §39 fusion-cut substitution needs God-tightness at THOSE
//! nodes, not at receipt roots. This census asks the question where the
//! substitution would actually be applied.
//!
//! WHAT IS COMPUTED. Descend from the root exactly as
//! [`response_success_mass`] does (focal nodes branch over every legal
//! action, hidden nodes over every branch tile with the posterior
//! conditioned) until the post-root history reaches the declared cut
//! depth. Each frontier belief `B` is priced three ways: its exact
//! information-consistent optimum `Q(B)` by the §48 recursion; its
//! world-revealed upper `U^God(B)` by a per-world make check over the
//! worlds `B` represents (the SAME line walk the doom census uses,
//! reached through `pub(crate)` visibility rather than a copy — the
//! posterior under a deterministic field is uniform on its surviving
//! worlds, so the count is the mass); and the gap `Φ(B) = U^God − Q`.
//! Nodes decided by the §5 arithmetic before the cut are recorded with
//! their mass and not descended.
//!
//! THE ROOT CONSEQUENCE. The same descent re-prices the root twice: once
//! with the exact frontier values (which must reproduce
//! [`response_success_mass`] at the root — a gate), and once with the
//! God uppers substituted at the frontier — what a §39 fusion cut at
//! this depth would compute. The two root values, and the two root
//! argmaxes under the declared lowest-tile tie rule, are the practical
//! reading: the exact over-pricing a cut at this depth would introduce
//! at THIS root, and whether it would change the play.
//!
//! WHAT THIS IS NOT. Not a theorem (SC-A4 forbids theorem language for
//! the horizon), not a producer, and not a substitution: it measures
//! whether the substitution would be exact, and by how much it would
//! not. Contract variation is the caller's: a census is per (root,
//! contract), and a bid-reading field makes different contracts
//! different fields (§31's boundary).
//!
//! EXPLORATORY tier — below every evidentiary tier, cited by nothing
//! above it. New-core beside `godgap.rs`; imported by nothing but the
//! crate root.

use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::rules::{legal_plays, Domino, DominoSet, Seat};
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, RootPosition, SlicePolicy,
};
use crate::solver::doom::{line_can_make, LineCtx, LineState, WalkFrame};
use crate::solver::factor_belief::{
    response_success_mass, ExactCoverOracle, FactorBelief, ResponseStats,
};

/// The declared census parameters. Deterministic: two censuses with
/// equal spec and inputs are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonSpec {
    /// Post-root plays at which a belief node is a frontier node. `4`
    /// below a trick-start root is the next trick's start.
    pub cut_plays: usize,
    /// Frontier nodes whose mass exceeds this are REFUSED, typed, and
    /// counted; never priced by a partial walk.
    pub node_fiber_cap: u128,
}

/// What one frontier node established.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PricedNode {
    /// Worlds in which even a world-aware viewer cannot make.
    pub doomed: u128,
    /// The exact information-consistent optimum, as a mass over the
    /// node's fiber.
    pub q_mass: u128,
    /// `(Z − doomed) / Z`.
    pub upper: BigRational,
    /// `q_mass / Z`.
    pub q: BigRational,
    /// `upper − q`, the information price at this node.
    pub phi: BigRational,
    /// `Z − doomed == 0`: every world doomed, so the equality carries no
    /// information (U0's vacuity discipline).
    pub nothing_saveable: bool,
    /// The §5 arithmetic decides the node at the cut itself.
    pub decided_at_cut: Option<bool>,
    pub response_focal: u64,
    pub response_hidden: u64,
    pub doom_nodes: u64,
}

impl PricedNode {
    /// God-tight: the exact optimum meets the world-revealed upper.
    pub fn god_tight(&self) -> bool {
        self.phi == BigRational::from_integer(BigInt::from(0))
    }

    /// God-tight with something at stake.
    pub fn substantively_god_tight(&self) -> bool {
        self.god_tight() && !self.nothing_saveable
    }
}

/// The typed verdict of one node the descent stopped at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeVerdict {
    /// Decided by the §5 arithmetic before reaching the cut: the mass is
    /// recorded and the node is not descended (every continuation values
    /// alike).
    DecidedBeforeCut { depth: usize, made: bool },
    /// A frontier node above the declared fiber cap. No value is
    /// reported for it; the root's cut value is then absent too.
    Refused { fiber: u128, cap: u128 },
    /// A frontier node priced exactly.
    Priced(Box<PricedNode>),
}

/// One node the descent stopped at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrontierNode {
    /// The post-root public history, in play order.
    pub history: Vec<Domino>,
    pub seat_to_move: usize,
    pub viewer_to_move: bool,
    /// `Z(B)` — the node's exact posterior mass.
    pub mass: u128,
    pub verdict: NodeVerdict,
}

/// One root action's two readings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootActionReading {
    pub action: Domino,
    /// The exact `Q_a` mass, re-descended through the frontier.
    pub exact_mass: u128,
    /// The mass a fusion cut at the declared depth would compute for
    /// this action — God uppers at the frontier. `None` when any
    /// frontier node below the action was refused.
    pub cut_mass: Option<u128>,
}

/// The census of one root at one contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonCensus {
    pub root_id: u64,
    pub field_id: String,
    pub contract: u32,
    pub root_fiber: u128,
    pub spec: HorizonSpec,
    pub nodes: Vec<FrontierNode>,
    pub actions: Vec<RootActionReading>,
    /// `max_a exact_mass` — the root's exact optimum through the frontier.
    pub root_exact_mass: u128,
    /// [`response_success_mass`] at the root, computed independently of
    /// the descent. Equal to `root_exact_mass` by construction; recorded
    /// so the equality is a checked number.
    pub root_check_mass: u128,
    /// `max_a cut_mass` — what a fusion cut at this depth would compute.
    pub root_cut_mass: Option<u128>,
    /// The root argmax under the lowest-tile tie rule, exact and under
    /// the cut. `None` when there is no legal root action (never at a
    /// lawful root) or, for the cut, when any action was refused.
    pub exact_argmax: Option<Domino>,
    pub cut_argmax: Option<Domino>,
    /// Field consultations the whole census made, by the counting
    /// decorator around the declared field.
    pub field_reads: u64,
}

impl HorizonCensus {
    pub fn frontier_nodes(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| !matches!(n.verdict, NodeVerdict::DecidedBeforeCut { .. }))
            .count()
    }

    pub fn decided_before_cut(&self) -> (usize, u128) {
        let mut count = 0usize;
        let mut mass = 0u128;
        for n in &self.nodes {
            if matches!(n.verdict, NodeVerdict::DecidedBeforeCut { .. }) {
                count += 1;
                mass = mass.checked_add(n.mass).expect("an exact mass fits u128");
            }
        }
        (count, mass)
    }

    pub fn refused(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| matches!(n.verdict, NodeVerdict::Refused { .. }))
            .count()
    }

    pub fn priced(&self) -> Vec<(&FrontierNode, &PricedNode)> {
        self.nodes
            .iter()
            .filter_map(|n| match &n.verdict {
                NodeVerdict::Priced(p) => Some((n, p.as_ref())),
                _ => None,
            })
            .collect()
    }

    /// `(substantive God-tight, vacuous God-tight, positive gap)` over
    /// the priced frontier nodes.
    pub fn tally(&self) -> (usize, usize, usize) {
        let mut tight = 0usize;
        let mut vacuous = 0usize;
        let mut positive = 0usize;
        for (_, p) in self.priced() {
            if !p.god_tight() {
                positive += 1;
            } else if p.nothing_saveable {
                vacuous += 1;
            } else {
                tight += 1;
            }
        }
        (tight, vacuous, positive)
    }

    /// The largest information price on the frontier.
    pub fn max_phi(&self) -> Option<BigRational> {
        self.priced().into_iter().map(|(_, p)| p.phi.clone()).max()
    }

    /// The mass-weighted mean information price over the priced frontier:
    /// `Σ_B (Z_B − doomed_B − Q_B) / Σ_B Z_B` — the fraction of frontier
    /// mass a God-upper substitution would over-count, summed over nodes.
    pub fn weighted_phi(&self) -> Option<BigRational> {
        let mut num: u128 = 0;
        let mut den: u128 = 0;
        for (n, p) in self.priced() {
            num = num
                .checked_add(n.mass - p.doomed - p.q_mass)
                .expect("an exact mass fits u128");
            den = den.checked_add(n.mass).expect("an exact mass fits u128");
        }
        if den == 0 {
            return None;
        }
        Some(BigRational::new(BigInt::from(num), BigInt::from(den)))
    }

    /// `(root_cut − root_exact) / Z` — the exact over-pricing a fusion cut
    /// at this depth introduces at the root. `None` when refused.
    pub fn root_over_pricing(&self) -> Option<BigRational> {
        let cut = self.root_cut_mass?;
        Some(BigRational::new(
            BigInt::from(cut - self.root_exact_mass),
            BigInt::from(self.root_fiber),
        ))
    }

    /// Whether the cut would change the root play under the declared tie
    /// rule. `None` when the cut is refused.
    pub fn cut_flips_root(&self) -> Option<bool> {
        Some(self.cut_argmax? != self.exact_argmax?)
    }
}

/// A root position with a different contract; everything else equal.
/// The census is per (root, contract), and under a bid-reading field the
/// two are different fields (§31's boundary) — recorded, never blurred.
pub fn with_contract(position: &RootPosition, bid: u32) -> RootPosition {
    let mut out = position.clone();
    out.bid = bid;
    out
}

// ---------------------------------------------------------------------------
// The census.
// ---------------------------------------------------------------------------

struct Descent<'a> {
    oracle: &'a dyn ExactCoverOracle,
    field: &'a dyn SlicePolicy,
    spec: &'a HorizonSpec,
    total_plays: usize,
    nodes: Vec<FrontierNode>,
}

impl Descent<'_> {
    /// Descend one belief; return `(exact mass, cut mass)` of the
    /// subtree, the cut mass `None` where a refusal occurred below.
    fn walk(&mut self, belief: &FactorBelief) -> (u128, Option<u128>) {
        let viewer = belief.kernel().viewer();
        let state = belief.public_state();
        let depth = belief.history().len();
        let at_terminal = depth == self.total_plays;
        let z = self.oracle.mass(belief);
        if depth >= self.spec.cut_plays {
            let seat = state.leader.plus(state.plays.len());
            if z > self.spec.node_fiber_cap {
                self.nodes.push(FrontierNode {
                    history: belief.history().to_vec(),
                    seat_to_move: seat.index(),
                    viewer_to_move: seat == viewer,
                    mass: z,
                    verdict: NodeVerdict::Refused {
                        fiber: z,
                        cap: self.spec.node_fiber_cap,
                    },
                });
                let mut rs = ResponseStats::default();
                // The exact side is still computed so the root's exact
                // re-descent stays complete; only the cut side is absent.
                let q = response_success_mass(self.oracle, belief, self.field, &mut rs);
                return (q, None);
            }
            let priced = price_node(self.oracle, belief, self.field, z);
            let cut = z - priced.doomed;
            let q = priced.q_mass;
            self.nodes.push(FrontierNode {
                history: belief.history().to_vec(),
                seat_to_move: seat.index(),
                viewer_to_move: seat == viewer,
                mass: z,
                verdict: NodeVerdict::Priced(Box::new(priced)),
            });
            return (q, Some(cut));
        }
        if let Some(made) = decided_success(belief.position(), viewer, state.banked, at_terminal) {
            self.nodes.push(FrontierNode {
                history: belief.history().to_vec(),
                seat_to_move: state.leader.plus(state.plays.len()).index(),
                viewer_to_move: state.leader.plus(state.plays.len()) == viewer,
                mass: z,
                verdict: NodeVerdict::DecidedBeforeCut { depth, made },
            });
            let v = if made { z } else { 0 };
            return (v, Some(v));
        }
        assert!(
            depth < self.total_plays,
            "an undecided state has plays left (rules invariant)"
        );
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
            let mut best_exact: Option<u128> = None;
            let mut best_cut: Option<Option<u128>> = None;
            for tile in legal.iter() {
                let (e, c) = self.walk(&belief.focal_play(tile));
                best_exact = Some(best_exact.map_or(e, |b| b.max(e)));
                best_cut = Some(match (best_cut, c) {
                    (None, c) => c,
                    (Some(None), _) | (Some(_), None) => None,
                    (Some(Some(b)), Some(c)) => Some(b.max(c)),
                });
            }
            (
                best_exact.expect("a legal set holds an action"),
                best_cut.expect("a legal set holds an action"),
            )
        } else {
            let mut exact: u128 = 0;
            let mut cut: Option<u128> = Some(0);
            for (tile, _) in self.oracle.branch_masses(belief, self.field) {
                let child = self.oracle.condition(belief, tile, self.field);
                let (e, c) = self.walk(&child);
                exact = exact.checked_add(e).expect("an exact mass fits u128");
                cut = match (cut, c) {
                    (Some(a), Some(b)) => Some(a.checked_add(b).expect("an exact mass fits u128")),
                    _ => None,
                };
            }
            (exact, cut)
        }
    }
}

/// Price one frontier belief: exact `Q`, per-world doom over the worlds
/// it represents, and the gap.
fn price_node(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
    z: u128,
) -> PricedNode {
    let mut rs = ResponseStats::default();
    let q_mass = response_success_mass(oracle, belief, field, &mut rs);
    let (doomed, worlds, doom_nodes) = doom_over_belief(belief, field);
    assert_eq!(
        worlds, z,
        "the per-world enumeration covers the node's posterior mass exactly"
    );
    assert!(
        q_mass <= z - doomed,
        "a doom upper never sits below the exact information-consistent optimum"
    );
    let upper = BigRational::new(BigInt::from(z - doomed), BigInt::from(z));
    let q = BigRational::new(BigInt::from(q_mass), BigInt::from(z));
    let phi = &upper - &q;
    let viewer = belief.kernel().viewer();
    let state = belief.public_state();
    let total = belief.kernel().viewer_hand().len()
        + belief
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let decided_at_cut = decided_success(
        belief.position(),
        viewer,
        state.banked,
        belief.history().len() == total,
    );
    PricedNode {
        doomed,
        q_mass,
        upper,
        q,
        phi,
        nothing_saveable: z == doomed,
        decided_at_cut,
        response_focal: rs.focal_nodes,
        response_hidden: rs.hidden_nodes,
        doom_nodes,
    }
}

/// Enumerate the worlds a belief represents and count the doomed —
/// `(doomed, worlds, line-walk nodes)`. The posterior under a
/// deterministic field is uniform on its surviving worlds (every
/// likelihood is 0 or 1), so the count IS the mass; asserted per world.
fn doom_over_belief(belief: &FactorBelief, field: &dyn SlicePolicy) -> (u128, u128, u64) {
    let kernel = belief.kernel();
    let viewer = kernel.viewer();
    let position = belief.position();
    let state = belief.public_state();
    let hidden_seats: [Seat; 3] = core::array::from_fn(|i| kernel.hidden()[i].seat);
    let total_plays =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let frame = WalkFrame {
        position,
        viewer,
        viewer_root_hand: kernel.viewer_hand(),
        hidden_seats,
        total_plays,
        field,
    };
    let mut base = LineState {
        leader: state.leader,
        plays: [Domino::from_index(0).expect("a tile"); 4],
        play_count: 0,
        banked: state.banked,
        remaining: [DominoSet::EMPTY; Seat::COUNT],
        played: belief.history().len(),
    };
    for (i, d) in state.plays.iter().enumerate() {
        base.plays[i] = *d;
        base.play_count += 1;
    }
    base.remaining[viewer.index()] = kernel
        .viewer_hand()
        .difference(state.played_by[viewer.index()]);
    let supports: [Vec<(DominoSet, u128)>; 3] =
        core::array::from_fn(|i| belief.factors()[i].support());
    let last_table: BTreeMap<u32, u128> = supports[2].iter().map(|(h, w)| (h.bits(), *w)).collect();
    let pool = kernel.pool();
    let mut ctx = LineCtx {
        frame: &frame,
        history: belief.history().to_vec(),
        nodes: 0,
    };
    let mut doomed: u128 = 0;
    let mut worlds: u128 = 0;
    for (h0, w0) in &supports[0] {
        let rest = pool.difference(*h0);
        for (h1, w1) in &supports[1] {
            if !h1.is_subset_of(rest) {
                continue;
            }
            let h2 = rest.difference(*h1);
            let Some(w2) = last_table.get(&h2.bits()) else {
                continue;
            };
            let weight = w0
                .checked_mul(*w1)
                .and_then(|w| w.checked_mul(*w2))
                .expect("an exact weight fits u128");
            assert_eq!(
                weight, 1,
                "under a deterministic field every surviving world has unit weight"
            );
            worlds += 1;
            let mut st = base;
            st.remaining[hidden_seats[0].index()] =
                h0.difference(state.played_by[hidden_seats[0].index()]);
            st.remaining[hidden_seats[1].index()] =
                h1.difference(state.played_by[hidden_seats[1].index()]);
            st.remaining[hidden_seats[2].index()] =
                h2.difference(state.played_by[hidden_seats[2].index()]);
            if !line_can_make(&mut ctx, position, st) {
                doomed += 1;
            }
        }
    }
    (doomed, worlds, ctx.nodes)
}

/// The counting decorator around the declared field, so the census
/// reports its consultations as a measurement.
struct CountingField<'a> {
    inner: &'a dyn SlicePolicy,
    reads: std::cell::Cell<u64>,
}

impl SlicePolicy for CountingField<'_> {
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn choose(
        &self,
        decl: crate::rules::Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &crate::solver::adaptive::PublicRecord<'_>,
    ) -> Domino {
        self.reads.set(self.reads.get().saturating_add(1));
        self.inner.choose(decl, hand, legal, record)
    }
}

/// The census of one root at the contract its position declares.
pub fn horizon_census(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    spec: &HorizonSpec,
) -> HorizonCensus {
    let counted = CountingField {
        inner: field,
        reads: std::cell::Cell::new(0),
    };
    let belief = FactorBelief::uniform_root(root, position, &counted);
    let z = oracle.mass(&belief);
    assert!(z > 0, "a census root has positive belief mass");
    let kernel = root.kernel();
    let total_plays =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let mut descent = Descent {
        oracle,
        field: &counted,
        spec,
        total_plays,
        nodes: Vec::new(),
    };
    // Root actions in tile order; the root is focal by construction.
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, kernel.viewer_hand(), led);
    let mut tiles: Vec<Domino> = legal.iter().collect();
    tiles.sort_by_key(|t| t.index());
    let mut actions: Vec<RootActionReading> = Vec::new();
    for a in tiles {
        let (e, c) = descent.walk(&belief.focal_play(a));
        actions.push(RootActionReading {
            action: a,
            exact_mass: e,
            cut_mass: c,
        });
    }
    let root_exact_mass = actions.iter().map(|r| r.exact_mass).max().unwrap_or(0);
    let exact_argmax = actions
        .iter()
        .find(|r| r.exact_mass == root_exact_mass)
        .map(|r| r.action);
    let root_cut_mass = if actions.iter().any(|r| r.cut_mass.is_none()) {
        None
    } else {
        actions.iter().filter_map(|r| r.cut_mass).max()
    };
    let cut_argmax = root_cut_mass.and_then(|m| {
        actions
            .iter()
            .find(|r| r.cut_mass == Some(m))
            .map(|r| r.action)
    });
    let mut rs = ResponseStats::default();
    let root_check_mass = response_success_mass(oracle, &belief, &counted, &mut rs);
    assert_eq!(
        root_check_mass, root_exact_mass,
        "the frontier re-descent reproduces the root's exact optimum"
    );
    HorizonCensus {
        root_id: root_identity(root, position),
        field_id: field.id().to_string(),
        contract: position.bid,
        root_fiber: z,
        spec: spec.clone(),
        nodes: descent.nodes,
        actions,
        root_exact_mass,
        root_check_mass,
        root_cut_mass,
        exact_argmax,
        cut_argmax,
        field_reads: counted.reads.get(),
    }
}
