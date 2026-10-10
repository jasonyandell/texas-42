//! `solver::grammar` — the two-policy grammar and the residual split: the
//! counted-belief Slice B (parent §45).
//!
//! EXPLORATORY tier. **[L2 consuming CE machinery]** through the
//! sanctioned one-directional crossing. Implements Part III (§9, §11,
//! §12) of the verbatim parent
//! `walt/math/counted_belief_sandwich_v0.1.md` (intake companion beside
//! it; adjudicated `walt/CENSUS-RULINGS.md` CBS-A4: grammars and the §12
//! decomposition ADOPTED; the §11 non-theorem is the O34 strategy-fusion
//! fence and stays load-bearing — two disjoint seed policies provide two
//! lower witnesses and a grammar, NEVER coverage of omitted policies;
//! coverage is only ever a residual bound).
//!
//! The objects. A [`PolicyGrammar`] is §11's induced map
//! `G(I) = {ρ_1(I), …, ρ_k(I)} ⊆ A(I)` over deterministic
//! information-consistent source policies: it may legally combine the
//! sources by information state, and it cannot choose by hidden world —
//! type-enforced, because [`PolicyGrammar::actions`] sees only the focal
//! hand and the [`PublicRecord`]. One walk
//! ([`exact_grammar_split`] / [`sampled_grammar_split`]) computes, for a
//! fixed root action over a declared world set, the value triple of the
//! §12 decomposition:
//!
//! - `free` — the unrestricted optimum over the deterministic
//!   information-consistent continuations (the
//!   [`exact_root_value`](crate::solver::exposure::exact_root_value) /
//!   `sampled_root_optimum` class, one action per public-history node, no
//!   strategy fusion);
//! - `gram` — the optimum restricted to `ρ(I) ∈ G(I)` at every reached
//!   focal state (`Q^G_a` of §12);
//! - `dev`  — the optimum over continuations that take at least one
//!   off-grammar action at a reached, still-undecided focal state (the
//!   §12 residual, over continuations that DEVIATE WHERE IT COULD
//!   MATTER — see the quotient note below).
//!
//! Theorem 9.1's cylinder partition makes `free = max(gram, dev)` a
//! nodewise identity, and the walk asserts it at EVERY node, not only at
//! the root: at a focal state the child cylinders partition by the chosen
//! action (grammar children keep the constraint, off-grammar children
//! release it); at a field state the bundle splits by the field's
//! per-world response and a deviating continuation needs its deviation in
//! exactly one branch (deviating in more is dominated, since the
//! constrained optimum never exceeds the free one).
//!
//! **The quotient note (decided truncation).** Continuation classes here
//! are over decided-truncated behavior: the walk stops where
//! [`decided_success`] settles the pmake indicator for every
//! continuation, so an action taken after the outcome is decided cannot
//! distinguish continuations. "Off-grammar" therefore means off-grammar
//! at a state where the outcome is still open; a continuation that
//! deviates only after decision is value-equivalent to an in-grammar one
//! and counts on the grammar side. This is the quotient under which the
//! §12 exclusion is meaningful: `gram > dev` says every optimal
//! continuation plays in-grammar at every state that could still change
//! the outcome.
//!
//! **The §8 identity, made mechanical
//! ([`residual_empirical_max_upper`]).** On a declared stream prefix the
//! only admissible empirical-max count for the residual class is the FULL
//! class optimum: a continuation may deviate at an information state the
//! sample never reaches, realizing the unrestricted optimum inside
//! `Π_a^{¬G}`, so any smaller count — in particular the on-sample
//! deviation optimum `dev` — is a lower approximation and is refused by
//! Corollary 5.2's one-way rule. Partitioning by itself tightens nothing
//! (parent §8); the sampled residual upper coincides with the Slice A
//! upper, and the producer here documents and asserts that rather than
//! pretending otherwise. Genuine residual tightening comes only from the
//! exact side: `dev` over the complete fiber is an exact residual value,
//! and `gram > dev` is the §12 boxed exclusion realized exactly.
//!
//! First-deviation witnesses ([`first_deviation`]) follow §12's lazy
//! route (CBS-A4: first-deviation cylinders may be discovered lazily)
//! under the declared canonical order: depth-first from the root,
//! ascending tile index at focal states, field-group discovery order at
//! field states. The witness names the first off-grammar step of one
//! optimal deviating line — the §10 split-order signal and §45's "first
//! off-grammar information states in exact counterexamples."
//!
//! Deviations from the parent sketch, recorded honestly: (a) §45 names
//! "current level-2 sketch or waking continuation" as a grammar source —
//! neither is `SlicePolicy`-shaped today, so the shipped sources are the
//! pinned level-1 continuation (`solver::policy::FrozenPolicy`), the σ0
//! level-0 modeled mind (`solver::field::FieldModel`, itself a
//! `SlicePolicy`), and this module's [`CountPreservation`] safety policy;
//! the waking/level-2 source joins when one exists as a deterministic
//! information-consistent policy. (b) The grammar solve on the sample is
//! exact ON ITS DECLARED PREFIX and bounds nothing about the fiber —
//! stated per the sampled-zero discipline, same as every sampled count.
//!
//! Nothing in this module touches the live default player (CBS-A9,
//! CE-A7/§20.16).

use std::fmt;

use crate::kernel::World;
use crate::rules::rules::legal_plays;
use crate::rules::{Decl, Domino, DominoSet, Seat};
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::evidence::ScopedDelta;
use crate::solver::exposure::{PublicExec, WorldDomain};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::root_interval::RootActionUpper;
use crate::solver::upper_cs::POLICY_CLASS_INFO_CONSISTENT;

// ---------------------------------------------------------------------------
// The policy grammar (§11).
// ---------------------------------------------------------------------------

/// §11 — the grammar induced by a small set of deterministic
/// information-consistent source policies:
/// `G(I) = {ρ_1(I), …, ρ_k(I)}`, nonempty at every information state
/// because every source always chooses. The grammar may combine sources
/// by information state; it cannot choose by hidden world — the only
/// query surface is (declaration, focal hand, legal set, public record).
pub struct PolicyGrammar<'a> {
    sources: Vec<&'a dyn SlicePolicy>,
    label: String,
}

impl<'a> PolicyGrammar<'a> {
    /// Build the induced grammar. At least one source is required —
    /// `G(I)` must be nonempty at every information state (§11).
    pub fn new(sources: Vec<&'a dyn SlicePolicy>) -> PolicyGrammar<'a> {
        assert!(
            !sources.is_empty(),
            "a grammar needs at least one source policy: G(I) is nonempty"
        );
        let names: Vec<&str> = sources.iter().map(|s| s.id()).collect();
        let label = format!("grammar:[{}]-v1", names.join(";"));
        PolicyGrammar { sources, label }
    }

    /// The grammar identity: the ordered source identities. Changing any
    /// source — or their order — is a different grammar (§43 discipline:
    /// the residual class is a function of the grammar, so the identity
    /// travels with every result).
    pub fn id(&self) -> &str {
        &self.label
    }

    /// `G(I)` at one focal information state: every source's choice.
    /// Nonempty and a subset of `legal` by construction (each source
    /// chooses a legal action; asserted).
    pub fn actions(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> DominoSet {
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let mut out = DominoSet::EMPTY;
        for source in &self.sources {
            let tile = source.choose(decl, hand, legal, record);
            assert!(
                legal.contains(tile),
                "a grammar source chooses a legal action"
            );
            out.insert(tile);
        }
        out
    }
}

// ---------------------------------------------------------------------------
// The value triple carrier (§12).
// ---------------------------------------------------------------------------

/// The §12 verdict of one root action's grammar split, under the decided
/// quotient (module docs): what the exact comparison of `gram` and `dev`
/// establishes about optimal continuations of this action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrammarVerdict {
    /// The root action itself is off-grammar: every continuation of this
    /// action lies in the residual, and `dev = free`.
    RootOffGrammar,
    /// `gram > dev` (or no deviating continuation exists at all): every
    /// optimal continuation of this action plays in-grammar at every
    /// still-undecided state — the §12 boxed exclusion, realized exactly
    /// when the domain is the complete fiber.
    Closes,
    /// `gram = dev`: an optimal in-grammar continuation exists, and so
    /// does an equally good deviating one — the grammar attains the
    /// optimum without excluding the residual.
    Ties,
    /// `gram < dev`: the optimum requires leaving the grammar; a
    /// first-deviation witness names where ([`first_deviation`]).
    Counterexample,
}

impl fmt::Display for GrammarVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GrammarVerdict::RootOffGrammar => "root-off-grammar",
            GrammarVerdict::Closes => "closes",
            GrammarVerdict::Ties => "ties",
            GrammarVerdict::Counterexample => "counterexample",
        };
        write!(f, "{s}")
    }
}

/// One root action's §12 value triple over a declared world set: the
/// unrestricted optimum `free`, the grammar-restricted optimum `gram`
/// (`Q^G_a`; absent when the root action itself is off-grammar — the
/// grammar class of this action is empty), and the residual optimum `dev`
/// (absent when no deviating continuation exists — the grammar saturates
/// every reached, still-undecided choice). Theorem 9.1's identity
/// `free = max(gram, dev)` is asserted at construction and at every node
/// of the walk that produced it.
///
/// Like every result of this stack, the counts are exact over the
/// DECLARED domain: a [`WorldDomain::StreamPrefix`] triple is exact on
/// its enumerated worlds and bounds nothing about the fiber by itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSplitValue {
    /// The fixed root action `a`.
    pub action: Domino,
    /// The grammar identity the split is relative to.
    pub grammar: String,
    /// The one declared field the walk ran under.
    pub field: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The declared world set.
    pub domain: WorldDomain,
    /// Whether the root action lies in `G` at the root information state.
    pub root_in_grammar: bool,
    free: u64,
    gram: Option<u64>,
    dev: Option<u64>,
}

impl GrammarSplitValue {
    #[allow(clippy::too_many_arguments)]
    fn new(
        action: Domino,
        grammar: String,
        field: FieldId,
        root_id: u64,
        domain: WorldDomain,
        root_in_grammar: bool,
        free: u64,
        gram: Option<u64>,
        dev: Option<u64>,
    ) -> GrammarSplitValue {
        assert_eq!(
            root_in_grammar,
            gram.is_some(),
            "the grammar side exists exactly when the root action is in-grammar"
        );
        if !root_in_grammar {
            assert_eq!(
                dev,
                Some(free),
                "an off-grammar root action puts every continuation in the residual"
            );
        }
        assert_eq!(
            free,
            gram.unwrap_or(0).max(dev.unwrap_or(0)),
            "Theorem 9.1: the cylinder partition gives free = max(gram, dev)"
        );
        GrammarSplitValue {
            action,
            grammar,
            field,
            root_id,
            domain,
            root_in_grammar,
            free,
            gram,
            dev,
        }
    }

    /// The unrestricted optimum count over the declared domain.
    pub fn free_count(&self) -> u64 {
        self.free
    }

    /// `Q^G_a` as a count — `None` when the root action is off-grammar.
    pub fn grammar_count(&self) -> Option<u64> {
        self.gram
    }

    /// The residual optimum count — `None` when no deviating
    /// continuation exists.
    pub fn deviation_count(&self) -> Option<u64> {
        self.dev
    }

    /// The §12 comparison under the decided quotient.
    pub fn verdict(&self) -> GrammarVerdict {
        match (self.gram, self.dev) {
            (None, _) => GrammarVerdict::RootOffGrammar,
            (Some(_), None) => GrammarVerdict::Closes,
            (Some(g), Some(d)) => {
                if g > d {
                    GrammarVerdict::Closes
                } else if g == d {
                    GrammarVerdict::Ties
                } else {
                    GrammarVerdict::Counterexample
                }
            }
        }
    }
}

impl fmt::Display for GrammarSplitValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GrammarSplitValue{{action={};verdict={};free={};gram={};dev={};root={:#018x};\
             field={};domain={};grammar={}}}",
            self.action,
            self.verdict(),
            self.free,
            self.gram.map_or_else(|| "-".to_string(), |v| v.to_string()),
            self.dev.map_or_else(|| "-".to_string(), |v| v.to_string()),
            self.root_id,
            self.field,
            self.domain,
            self.grammar
        )
    }
}

/// §12's lazy first-deviation witness: the first off-grammar step of one
/// optimal deviating line, under the declared canonical order (depth-first
/// from the root, ascending tile index at focal states, field-group
/// discovery order at field states). `history` is the post-root public
/// play sequence at the deviation state (the root action, when taken, is
/// its first entry); `depth = history.len()`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstDeviation {
    /// Post-root plies made before the deviation state.
    pub depth: usize,
    /// The post-root public history identifying the deviation state.
    pub history: Vec<Domino>,
    /// `G(I)` at the deviation state.
    pub state_grammar: DominoSet,
    /// `A(I)` at the deviation state.
    pub legal: DominoSet,
    /// The off-grammar action the optimal deviating line takes there.
    pub deviation: Domino,
}

impl fmt::Display for FirstDeviation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hist: Vec<String> = self.history.iter().map(|d| d.to_string()).collect();
        write!(
            f,
            "FirstDeviation{{depth={};deviation={};grammar_size={};legal_size={};history=[{}]}}",
            self.depth,
            self.deviation,
            self.state_grammar.len(),
            self.legal.len(),
            hist.join(",")
        )
    }
}

// ---------------------------------------------------------------------------
// The walk.
// ---------------------------------------------------------------------------

/// The nodewise §12 triple: `free`/`gram` are total (the grammar holds an
/// action at every focal state), `dev` is `None` where no deviating
/// continuation exists at or below the node.
#[derive(Clone, Copy)]
struct Triple {
    free: u64,
    gram: u64,
    dev: Option<u64>,
}

impl Triple {
    fn check(self) -> Triple {
        assert_eq!(
            self.free,
            self.gram.max(self.dev.unwrap_or(0)),
            "Theorem 9.1 holds nodewise: free = max(gram, dev)"
        );
        self
    }
}

/// The single-field grammar walk over a declared world set: the
/// [`exact_root_value`](crate::solver::exposure::exact_root_value)
/// tree — one action per public-history node at
/// focal states, the declared field's per-world partition at field
/// states, decided-outcome payoff — carrying the grammar constraint
/// alongside the free optimum.
struct GrammarWalk<'a> {
    position: &'a RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    worlds: &'a [World],
    field: &'a FieldModel,
    grammar: &'a PolicyGrammar<'a>,
    /// Post-root plays to terminal.
    total: usize,
}

impl GrammarWalk<'_> {
    fn decided(&self, exec: &PublicExec) -> Option<bool> {
        decided_success(
            self.position,
            self.viewer,
            exec.banked,
            exec.history.len() == self.total,
        )
    }

    /// The focal frame at one viewer node: remaining hand, legal set.
    fn focal_frame(&self, exec: &PublicExec) -> (DominoSet, DominoSet) {
        let led = exec
            .plays
            .first()
            .map(|d| self.position.decl.led_context(*d));
        let hand = self.viewer_hand.difference(exec.played_since());
        let legal = legal_plays(self.position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        (hand, legal)
    }

    /// Group the live worlds at one field node by the declared field's
    /// choice (the exact-root-value partition verbatim).
    fn partition(&self, exec: &PublicExec, idxs: &[u32]) -> Vec<(Domino, Vec<u32>)> {
        let seat = exec.seat();
        let led = exec
            .plays
            .first()
            .map(|d| self.position.decl.led_context(*d));
        let played = exec.played_since();
        let record = exec.record(self.position);
        let mut groups: Vec<(Domino, Vec<u32>)> = Vec::new();
        for &i in idxs {
            let hand = self.worlds[usize::try_from(i).expect("fits")]
                .hand(seat)
                .difference(played);
            let legal = legal_plays(self.position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let tile = self.field.choose(self.position.decl, hand, legal, &record);
            match groups.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push(i),
                None => groups.push((tile, vec![i])),
            }
        }
        groups
    }

    /// The §12 triple at one node (see the module docs for the recursion
    /// and the nodewise Theorem 9.1 assertion).
    fn split_count(&self, exec: &PublicExec, idxs: &[u32]) -> Triple {
        assert!(!idxs.is_empty(), "a walk node carries at least one world");
        if let Some(u) = self.decided(exec) {
            let wins = if u {
                u64::try_from(idxs.len()).expect("fits")
            } else {
                0
            };
            // Decided: every continuation class attains the same payoff,
            // and no still-undecided deviation can occur below.
            return Triple {
                free: wins,
                gram: wins,
                dev: None,
            };
        }
        assert!(
            exec.history.len() < self.total,
            "the 42-point pool exhausts at terminal, so an undecided state has plays left"
        );
        if exec.seat() == self.viewer {
            let (hand, legal) = self.focal_frame(exec);
            let record = exec.record(self.position);
            let in_grammar = self
                .grammar
                .actions(self.position.decl, hand, legal, &record);
            let mut free = 0u64;
            let mut gram: Option<u64> = None;
            let mut dev: Option<u64> = None;
            for tile in legal.iter() {
                let mut child = exec.clone();
                child.play(self.position, tile);
                let t = self.split_count(&child, idxs);
                free = free.max(t.free);
                if in_grammar.contains(tile) {
                    gram = Some(gram.unwrap_or(0).max(t.gram));
                    if let Some(d) = t.dev {
                        dev = Some(dev.unwrap_or(0).max(d));
                    }
                } else {
                    // Deviating HERE releases the constraint below.
                    dev = Some(dev.unwrap_or(0).max(t.free));
                }
            }
            Triple {
                free,
                gram: gram.expect("a grammar holds an action at every focal state"),
                dev,
            }
            .check()
        } else {
            let groups = self.partition(exec, idxs);
            let mut free = 0u64;
            let mut gram = 0u64;
            // The smallest cost of placing the one deviation in a branch:
            // min over branches of (free − dev), tracked as a deficit so
            // every quantity stays in u64.
            let mut deficit: Option<u64> = None;
            for (tile, group) in groups {
                let mut child = exec.clone();
                child.play(self.position, tile);
                let t = self.split_count(&child, &group);
                free += t.free;
                gram += t.gram;
                if let Some(d) = t.dev {
                    let branch_deficit = t.free - d;
                    deficit = Some(deficit.map_or(branch_deficit, |b| b.min(branch_deficit)));
                }
            }
            Triple {
                free,
                gram,
                dev: deficit.map(|d| free - d),
            }
            .check()
        }
    }

    /// Trace one optimal deviating line to its first off-grammar step
    /// (declared canonical order: ascending tile index at focal states,
    /// field-group discovery order at field states). Precondition: a
    /// deviating continuation exists at or below this node.
    fn trace(&self, exec: &PublicExec, idxs: &[u32]) -> FirstDeviation {
        let here = self.split_count(exec, idxs);
        let target = here
            .dev
            .expect("the trace follows a node with a deviating continuation");
        assert!(
            self.decided(exec).is_none(),
            "no deviating continuation exists below a decided node"
        );
        if exec.seat() == self.viewer {
            let (hand, legal) = self.focal_frame(exec);
            let record = exec.record(self.position);
            let in_grammar = self
                .grammar
                .actions(self.position.decl, hand, legal, &record);
            // Deviate here if an off-grammar child attains the target.
            for tile in legal.iter() {
                if in_grammar.contains(tile) {
                    continue;
                }
                let mut child = exec.clone();
                child.play(self.position, tile);
                if self.split_count(&child, idxs).free == target {
                    return FirstDeviation {
                        depth: exec.history.len(),
                        history: exec.history.clone(),
                        state_grammar: in_grammar,
                        legal,
                        deviation: tile,
                    };
                }
            }
            // Otherwise stay in-grammar and push the deviation deeper.
            for tile in legal.iter() {
                if !in_grammar.contains(tile) {
                    continue;
                }
                let mut child = exec.clone();
                child.play(self.position, tile);
                if self.split_count(&child, idxs).dev == Some(target) {
                    return self.trace(&child, idxs);
                }
            }
            unreachable!("the deviation optimum is attained by one of its children");
        } else {
            let groups = self.partition(exec, idxs);
            // Recompute the free sum, then find the first branch whose
            // constrained value closes the gap to the target.
            let mut parts: Vec<(PublicExec, Vec<u32>, Triple)> = Vec::new();
            let mut free = 0u64;
            for (tile, group) in groups {
                let mut child = exec.clone();
                child.play(self.position, tile);
                let t = self.split_count(&child, &group);
                free += t.free;
                parts.push((child, group, t));
            }
            for (child, group, t) in parts {
                if let Some(d) = t.dev {
                    if free - (t.free - d) == target {
                        return self.trace(&child, &group);
                    }
                }
            }
            unreachable!("the deviation optimum is attained in one of its branches");
        }
    }
}

// ---------------------------------------------------------------------------
// Producers.
// ---------------------------------------------------------------------------

/// The shared entry: root-state grammar membership, the fixed root
/// action, then the walk.
fn split_over(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
    worlds: &[World],
    domain: WorldDomain,
) -> GrammarSplitValue {
    let kernel = root.kernel();
    let walk = GrammarWalk {
        position,
        viewer: kernel.viewer(),
        viewer_hand: kernel.viewer_hand(),
        worlds,
        field,
        grammar,
        total: kernel.viewer_hand().len()
            + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>(),
    };
    let exec0 = PublicExec::start(position);
    assert_eq!(
        exec0.seat(),
        walk.viewer,
        "the root decision is the viewer's"
    );
    let led = exec0.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, walk.viewer_hand, led);
    assert!(legal.contains(action), "a root action is legal at the root");
    let record = exec0.record(position);
    let root_grammar = grammar.actions(position.decl, walk.viewer_hand, legal, &record);
    let root_in_grammar = root_grammar.contains(action);
    let mut exec = exec0;
    exec.play(position, action);
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let t = walk.split_count(&exec, &idxs);
    let (gram, dev) = if root_in_grammar {
        (Some(t.gram), t.dev)
    } else {
        // The deviation already happened at the root: every continuation
        // of this action is residual, unconstrained below.
        (None, Some(t.free))
    };
    GrammarSplitValue::new(
        action,
        grammar.id().to_string(),
        field.field_id(),
        root_identity(root, position),
        domain,
        root_in_grammar,
        t.free,
        gram,
        dev,
    )
}

/// The §12 triple of one root action over the COMPLETE enumerated fiber:
/// exact `Q_a`, exact `Q^G_a`, exact residual value — the exact side of
/// §45, where `gram > dev` realizes the boxed §12 exclusion.
pub fn exact_grammar_split(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
) -> GrammarSplitValue {
    let worlds: Vec<World> = root.worlds().collect();
    assert!(
        u128::try_from(worlds.len()).expect("fits") == root.count(),
        "the exact split enumerates the complete fiber"
    );
    split_over(
        root,
        position,
        action,
        field,
        grammar,
        &worlds,
        WorldDomain::ExactFiber,
    )
}

/// The §12 triple over the indexed evidence-stream prefix `0..worlds` at
/// `epoch` — §45's "solve the restricted grammar exactly on the sample."
/// Exact on the declared multiset; bounds nothing about the fiber by
/// itself.
#[allow(clippy::too_many_arguments)]
pub fn sampled_grammar_split(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
    epoch: u64,
    worlds: u64,
) -> GrammarSplitValue {
    assert!(worlds >= 1, "a declared prefix holds at least one world");
    let root_id = root_identity(root, position);
    let sample: Vec<World> = (0..worlds)
        .map(|i| root.world_at(root_id, epoch, i))
        .collect();
    split_over(
        root,
        position,
        action,
        field,
        grammar,
        &sample,
        WorldDomain::StreamPrefix { epoch, worlds },
    )
}

/// The lazy first-deviation witness of one root action over the complete
/// fiber: `None` when no deviating continuation exists; the root action
/// itself (empty post-root history) when it is off-grammar; otherwise the
/// first off-grammar step of one optimal deviating line under the
/// declared canonical order.
pub fn first_deviation(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
) -> Option<FirstDeviation> {
    let worlds: Vec<World> = root.worlds().collect();
    let kernel = root.kernel();
    let walk = GrammarWalk {
        position,
        viewer: kernel.viewer(),
        viewer_hand: kernel.viewer_hand(),
        worlds: &worlds,
        field,
        grammar,
        total: kernel.viewer_hand().len()
            + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>(),
    };
    let exec0 = PublicExec::start(position);
    assert_eq!(
        exec0.seat(),
        walk.viewer,
        "the root decision is the viewer's"
    );
    let led = exec0.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, walk.viewer_hand, led);
    assert!(legal.contains(action), "a root action is legal at the root");
    let record = exec0.record(position);
    let root_grammar = grammar.actions(position.decl, walk.viewer_hand, legal, &record);
    if !root_grammar.contains(action) {
        return Some(FirstDeviation {
            depth: 0,
            history: Vec::new(),
            state_grammar: root_grammar,
            legal,
            deviation: action,
        });
    }
    let mut exec = exec0;
    exec.play(position, action);
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let t = walk.split_count(&exec, &idxs);
    t.dev.map(|_| walk.trace(&exec, &idxs))
}

// ---------------------------------------------------------------------------
// The grammar census (§45's action-set measurement).
// ---------------------------------------------------------------------------

/// Aggregates over every reachable, still-undecided focal state of one
/// root action's walk tree (post-root; the shared root state is not
/// counted): how much room the grammar leaves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GrammarCensus {
    /// Reachable undecided focal states.
    pub focal_states: u64,
    /// `Σ |G(I)|` over those states.
    pub grammar_action_total: u64,
    /// `Σ |A(I)|` over those states.
    pub legal_action_total: u64,
    /// States where `G(I) = A(I)` — no deviation is possible there.
    pub saturated_states: u64,
}

impl fmt::Display for GrammarCensus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GrammarCensus{{focal_states={};grammar_actions={};legal_actions={};saturated={}}}",
            self.focal_states,
            self.grammar_action_total,
            self.legal_action_total,
            self.saturated_states
        )
    }
}

fn census_walk(walk: &GrammarWalk<'_>, exec: &PublicExec, idxs: &[u32], out: &mut GrammarCensus) {
    if idxs.is_empty() || walk.decided(exec).is_some() {
        return;
    }
    if exec.seat() == walk.viewer {
        let (hand, legal) = walk.focal_frame(exec);
        let record = exec.record(walk.position);
        let in_grammar = walk
            .grammar
            .actions(walk.position.decl, hand, legal, &record);
        out.focal_states += 1;
        out.grammar_action_total += u64::try_from(in_grammar.len()).expect("fits");
        out.legal_action_total += u64::try_from(legal.len()).expect("fits");
        if in_grammar == legal {
            out.saturated_states += 1;
        }
        for tile in legal.iter() {
            let mut child = exec.clone();
            child.play(walk.position, tile);
            census_walk(walk, &child, idxs, out);
        }
    } else {
        for (tile, group) in walk.partition(exec, idxs) {
            let mut child = exec.clone();
            child.play(walk.position, tile);
            census_walk(walk, &child, &group, out);
        }
    }
}

/// Count the grammar's room over the complete fiber tree of one root
/// action (§45: "grammar action-set size per information state").
pub fn grammar_census(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
) -> GrammarCensus {
    let worlds: Vec<World> = root.worlds().collect();
    let kernel = root.kernel();
    let walk = GrammarWalk {
        position,
        viewer: kernel.viewer(),
        viewer_hand: kernel.viewer_hand(),
        worlds: &worlds,
        field,
        grammar,
        total: kernel.viewer_hand().len()
            + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>(),
    };
    let mut exec = PublicExec::start(position);
    assert_eq!(
        exec.seat(),
        walk.viewer,
        "the root decision is the viewer's"
    );
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, walk.viewer_hand, led);
    assert!(legal.contains(action), "a root action is legal at the root");
    exec.play(position, action);
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let mut out = GrammarCensus::default();
    census_walk(&walk, &exec, &idxs, &mut out);
    out
}

// ---------------------------------------------------------------------------
// The residual empirical-max upper (§45, via §8 and Corollary 5.2).
// ---------------------------------------------------------------------------

/// The δ-valid empirical-max upper for the residual region `Π_a^{¬G}`,
/// over the declared stream prefix — and the §8 identity made mechanical:
/// the declared count path IS the full-class optimum path, because a
/// continuation deviating at an information state the sample never
/// reaches realizes the unrestricted optimum inside the residual, so any
/// smaller count (the on-sample deviation optimum in particular) is a
/// lower approximation and inadmissible by Corollary 5.2's one-way rule.
/// The result therefore coincides, count for count, with Slice A's
/// [`pmake_empirical_max_upper`] — partitioning by itself tightens
/// nothing (parent §8), and CBS-A4's "coverage is only ever a residual
/// bound" has its sampled half here: the sampled route can bound the
/// residual, but never below the full class. Genuine tightening is the
/// exact side's job ([`exact_grammar_split`]'s `dev`).
///
/// The walk asserts `dev ≤ free` on every prefix (the constrained
/// optimum never exceeds the free one) while producing the path.
///
/// [`pmake_empirical_max_upper`]: crate::solver::root_interval::pmake_empirical_max_upper
#[allow(clippy::too_many_arguments)]
pub fn residual_empirical_max_upper(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    grammar: &PolicyGrammar<'_>,
    epoch: u64,
    prefix: u64,
    delta: ScopedDelta,
) -> RootActionUpper {
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let counts: Vec<u64> = (1..=prefix)
        .map(|t| {
            let s = sampled_grammar_split(root, position, action, field, grammar, epoch, t);
            if let Some(d) = s.deviation_count() {
                assert!(
                    d <= s.free_count(),
                    "the on-sample deviation optimum never exceeds the full optimum"
                );
            }
            s.free_count()
        })
        .collect();
    RootActionUpper::from_prefix_counts(
        action,
        field.field_id(),
        root_identity(root, position),
        epoch,
        root.count(),
        POLICY_CLASS_INFO_CONSISTENT,
        delta,
        counts,
    )
}

// ---------------------------------------------------------------------------
// The safety source (§45's third policy).
// ---------------------------------------------------------------------------

/// §45's "one safety/trump-control or count-preservation policy": a
/// deterministic information-consistent heuristic, declared by its label
/// — a grammar SOURCE whose quality is never load-bearing (it widens
/// `G(I)`; correctness lives in the walk). The rule, in priority order at
/// each state: leading with two or more called tiles, draw with the
/// strongest one (trump control); otherwise lead the strongest non-count
/// tile (count preservation). Following with the partner currently
/// winning, feed the heaviest count; otherwise win as cheaply as possible
/// spending no count if any winning play exists; otherwise discard the
/// lightest tile. Every comparison breaks ties toward the lowest tile
/// index, so the choice is a total deterministic function of the
/// information state.
pub struct CountPreservation {
    label: String,
}

impl CountPreservation {
    pub fn new() -> CountPreservation {
        CountPreservation {
            label: "safety:count-preservation-v1".to_string(),
        }
    }
}

impl Default for CountPreservation {
    fn default() -> CountPreservation {
        CountPreservation::new()
    }
}

/// The first tile of `set` (ascending index order) strictly maximizing
/// `better` — deterministic by the fixed iteration order.
fn select(set: DominoSet, better: impl Fn(Domino, Domino) -> bool) -> Domino {
    let mut best: Option<Domino> = None;
    for d in set.iter() {
        match best {
            Some(b) if !better(d, b) => {}
            _ => best = Some(d),
        }
    }
    best.expect("a nonempty selection set")
}

impl SlicePolicy for CountPreservation {
    fn id(&self) -> &str {
        &self.label
    }

    fn choose(
        &self,
        decl: Decl,
        _hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        if legal.len() == 1 {
            return legal.iter().next().expect("one legal tile");
        }
        let plays = record.trick_plays;
        if plays.is_empty() {
            // Leading. Trump control first: with two or more called tiles,
            // draw with the strongest.
            let called: DominoSet = legal.iter().filter(|d| decl.is_called(*d)).collect();
            if called.len() >= 2 {
                return select(called, |a, b| {
                    decl.trick_key(a, decl.led_context(a)) > decl.trick_key(b, decl.led_context(b))
                });
            }
            // Count preservation: lead the strongest non-count tile; with
            // only count tiles, the lightest of them.
            let non_count: DominoSet = legal.iter().filter(|d| d.count() == 0).collect();
            if non_count.is_empty() {
                return select(legal, |a, b| a.count() < b.count());
            }
            return select(non_count, |a, b| {
                decl.trick_key(a, decl.led_context(a)) > decl.trick_key(b, decl.led_context(b))
            });
        }
        // Following: find the current winner of the partial trick.
        let led = decl.led_context(plays[0]);
        let mut win_k = 0usize;
        for k in 1..plays.len() {
            if decl.trick_key(plays[k], led) > decl.trick_key(plays[win_k], led) {
                win_k = k;
            }
        }
        let me = record.leader.plus(plays.len());
        let partner_winning = record.leader.plus(win_k) == me.plus(2);
        if partner_winning {
            // Feed the heaviest count; among equals, the weakest tile.
            return select(legal, |a, b| {
                a.count() > b.count()
                    || (a.count() == b.count() && decl.trick_key(a, led) < decl.trick_key(b, led))
            });
        }
        let winning = decl.beats(led, plays[win_k]).intersection(legal);
        if !winning.is_empty() {
            // Win as cheaply as possible, spending no count if we can.
            return select(winning, |a, b| {
                a.count() < b.count()
                    || (a.count() == b.count() && decl.trick_key(a, led) < decl.trick_key(b, led))
            });
        }
        // Cannot win: discard the lightest tile, preserving count.
        select(legal, |a, b| {
            a.count() < b.count()
                || (a.count() == b.count() && decl.trick_key(a, led) < decl.trick_key(b, led))
        })
    }
}
