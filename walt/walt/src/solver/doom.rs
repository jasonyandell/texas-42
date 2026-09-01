//! The doom census — counterexample mass as a deterministic upper
//! bound (the structural producer §70 asks for, and the ∀-fail dual of
//! the §16 laydown hierarchy).
//!
//! THE BOUND. `pmake` at a root action is `max_π M_π / Z` over
//! information-consistent viewer policies, where `M_π` counts the
//! worlds in which `π` achieves the viewer objective against the
//! declared field σ0. Let `D` be any set of worlds in which EVERY
//! viewer continuation fails against σ0 — even a world-aware one. Any
//! information-consistent policy induces, in each world of `D`, one
//! concrete play sequence, which fails there; so `M_π ≤ Z − |D|` for
//! every `π`, and `pmake(action) ≤ (Z − |D|) / Z`.
//!
//! Failing worlds are found by COUNTING, never by witnessing: a
//! sampled counterexample subtracts one world's weight, a certified
//! class subtracts its exact §22 mass. The census constructs classes,
//! certifies them wholesale, and sizes them with the exact-cover
//! oracle.
//!
//! THE CERTIFIER. [`universal_viewer_failure`] is the quantifier dual
//! of the §16 universal walk: at focal nodes EVERY legal viewer tile
//! must fail (`∀π` — one surviving escape kills the class); at hidden
//! nodes the acting seat's record-consistent support is PARTITIONED by
//! the declared σ0's deterministic choice (this is `pmake`'s own field
//! semantics — a `∀σ`-fail dual would certify nearly nothing, since
//! fields that donate count let the viewer make), and every branch
//! must fail with the posterior restricted to the hands that chose its
//! tile. Decided cutoffs close both ends: a state whose §5 arithmetic
//! already decides the objective returns at once, so genuinely doomed
//! classes certify at the depth where the field's banked count crosses
//! the contract's complement — typically tricks two to three — not at
//! the horizon.
//!
//! SOUNDNESS OF THE RELAXATION. The walk keeps PER-SEAT consistency,
//! not the joint exact cover (the same relaxation, in the same
//! direction, as the §16 walk): branches that are per-seat consistent
//! but jointly impossible are still walked, so the walk ranges over a
//! SUPERSET of the true world set. Certifying failure over a superset
//! certifies it over the truth — a phantom world can only block a
//! certification, never manufacture one. In a true world of the class,
//! every hidden node's branch containing that world's hand follows
//! exactly σ0's tile and keeps the world in its posterior, and the
//! focal `∀` covers every play sequence a policy could induce; walked
//! failure everywhere therefore certifies per-world failure. The MASS,
//! by contrast, is never relaxed: a certified class is counted by the
//! oracle on the exact factor state, so the subtracted `|D|` is exact.
//!
//! THE DESCENT. Classes come from the §28/§49 vocabulary, not from
//! guessed predicates: level 1 partitions the first-responding hidden
//! seat's root hands by [`class_signature`] at the post-action record
//! (trump count, highest trump, follower count, count pips, can-beat,
//! can-ruff — the structural coordinates of a punishing hand); classes
//! the walk cannot certify descend, splitting the next hidden seat in
//! acting order, to a declared maximum level. Leaves partition the
//! fiber, so certified leaf masses add — one deterministic upper per
//! action carries their sum. A level-0 pre-walk on the unrestricted
//! belief catches the whole-fiber dooms (the §17-dual zero-cost path:
//! an already-set root certifies in one decided read). Everything is
//! budgeted in walk nodes and every refusal is counted; the census
//! reports `doomed + survived + refused (+ empty) = Z` as a checked
//! partition law, in the §46 discipline.
//!
//! WHAT THIS PRODUCER IS NOT. Nothing sampled exists on any path — no
//! stream seeds candidate classes, no δ is spent, and every installed
//! fact is `ProofTag::Deterministic` (§64's law, applied to uppers).
//! It certifies failure against the DECLARED field only: the facts'
//! authority carries the field identity, and no adversarial or
//! universal-field claim is ever made. It never touches `refine.rs`
//! (RefineV1 stays frozen) and installs through the open §49 producer
//! registry only.
//!
//! Mathematical sources: `walt/math/anytime_proof_state_score_v0.1.md`
//! §16–§17 (the quantifier hierarchy this dualizes), §22 (interval
//! recursion over unresolved classes — the counting discipline),
//! §28–§31/§49 (the class vocabulary and CEGAR pedigree), §70 (the
//! falsifier this answers: covers vacuous at rich roots → richer
//! structural producers). Direction: Jason, 2026-09-01 — "construct
//! and then size the number of beating arrangements".
//!
//! MODULE GRAPH. New-core beside `solver::proof_state` (the
//! `laydown`/`covers`/`extraction` pattern): imported by nothing but
//! the crate root, deletable with its siblings (§67.10).

use std::collections::BTreeMap;

use crate::rules::{legal_plays, Domino, DominoSet, Seat, Trick};
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::factor_belief::{
    class_signature, ClassSignature, ExactCoverOracle, FactorBelief,
};
use crate::solver::proof_state::{BoundFact, Fact, ProofProducer, ProofState, ProofTag};
use num_bigint::BigInt;
use num_rational::BigRational;

/// The declared census budget and vocabulary parameters. Everything
/// deterministic: two censuses with equal spec and inputs are equal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoomSpec {
    /// Total walk-node budget for one action's census.
    pub node_budget: u64,
    /// Per-walk node cap — one class never eats the whole budget.
    pub walk_cap: u64,
    /// Maximum descent level: how many hidden seats (in acting order
    /// after the root action) the partition may restrict, `1..=3`.
    pub max_level: usize,
    /// The §31 critical tile set fed to the signature vocabulary.
    pub critical: DominoSet,
    /// `None` — the FULL census: every class at every level is walked
    /// (or refused by the node budget), masses are computed for all,
    /// and the §46 partition law is asserted per split. `Some(k)` —
    /// the PRIORITY census for rich roots: each level walks only its
    /// top `k` classes under the declared punish order (opponents of
    /// the viewer nastiest-first: can-ruff, can-beat, trump count,
    /// count pips; the viewer's partner weakest-first: the same
    /// coordinates reversed), the rest are counted skipped with no
    /// mass computed; certified doom is then a declared PARTIAL
    /// harvest — sound, never complete, coverage stated by the
    /// census's own ledger.
    pub descend_top: Option<usize>,
}

/// One certified-doomed leaf: the signature path that names it (one
/// `(seat, signature)` per restricted level), its exact mass, and the
/// nodes its certifying walk spent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoomedLeaf {
    pub path: Vec<(Seat, ClassSignature)>,
    pub mass: u128,
    pub nodes: u64,
}

/// One action's doom census. The partition law holds by construction
/// and is asserted: `doomed + survived + refused + unpartitioned = Z`
/// (`unpartitioned` is the walked-but-uncertified interior mass that
/// never reached a leaf disposition because the budget died before its
/// children were enumerated — zero under an ample budget).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoomCensus {
    pub action: Domino,
    /// `Z` — the root fiber mass.
    pub fiber: u128,
    /// Certified counterexample mass (the sum over `doomed_leaves`).
    pub doomed_mass: u128,
    /// Mass in max-level leaves where the walk found an escape (a
    /// possibly-phantom viewer make path — never counted as doom).
    pub survived_mass: u128,
    /// Mass in classes never walked to a verdict (budget).
    pub refused_mass: u128,
    /// Walks run (interior and leaf).
    pub classes_walked: u64,
    pub classes_doomed: u64,
    pub classes_survived: u64,
    pub classes_refused: u64,
    /// Signature classes with zero exact mass under the parent
    /// restriction (represent no world; skipped, never walked).
    pub classes_empty: u64,
    /// Classes below the declared priority cut (`descend_top`) —
    /// never walked, no mass computed. Zero in a full census.
    pub classes_skipped: u64,
    /// Walk nodes spent across the census.
    pub nodes: u64,
    /// The level-0 pre-walk certified the WHOLE fiber (the §17-dual
    /// zero-cost path when it closes in one decided read).
    pub whole_fiber: bool,
    pub doomed_leaves: Vec<DoomedLeaf>,
    /// The deterministic upper this census certifies:
    /// `(Z − doomed_mass) / Z`.
    pub upper: BigRational,
}

// ---------------------------------------------------------------------------
// The certifying walk.
// ---------------------------------------------------------------------------

/// Immutable walk frame: everything constant across one walk.
struct WalkFrame<'a> {
    position: &'a RootPosition,
    viewer: Seat,
    viewer_root_hand: DominoSet,
    hidden_seats: [Seat; 3],
    /// Post-root plays at terminal (viewer remaining + hidden capacities).
    total_plays: usize,
    field: &'a dyn SlicePolicy,
}

/// One node's public state: the same evolution as the replay walkers,
/// restricted to public data. `played_by` attributes POST-ROOT plays.
#[derive(Clone)]
struct NodeState {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played_by: [DominoSet; Seat::COUNT],
    history: Vec<Domino>,
}

impl NodeState {
    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.leader.plus(self.plays.len());
        assert!(
            self.played_by[seat.index()].insert(tile),
            "a tile is played once"
        );
        self.plays.push(tile);
        self.history.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }
}

/// The §16-dual universal-failure walk over one class: `Some(true)`
/// exactly when — in EVERY represented world, for EVERY legal viewer
/// continuation — the viewer objective FAILS against the declared
/// deterministic field; `Some(false)` when some walked branch reaches
/// a make (possibly a phantom world — an escape is never a
/// certification of anything); `None` when the node budget dies first.
/// Sound per the module argument: the per-seat relaxation only ever
/// blocks certification.
fn universal_viewer_failure(
    frame: &WalkFrame<'_>,
    state: &NodeState,
    tables: [&[(DominoSet, u128)]; 3],
    budget: &mut u64,
) -> Option<bool> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let at_terminal = state.history.len() == frame.total_plays;
    if let Some(u) = decided_success(frame.position, frame.viewer, state.banked, at_terminal) {
        return Some(!u);
    }
    let seat = state.leader.plus(state.plays.len());
    let decl = frame.position.decl;
    let led = state.plays.first().map(|d| decl.led_context(*d));
    if seat == frame.viewer {
        // Focal: every legal escape must fail.
        let remaining = frame
            .viewer_root_hand
            .difference(state.played_by[frame.viewer.index()]);
        let legal = legal_plays(decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        for tile in legal.iter() {
            let mut child = state.clone();
            child.play(frame.position, tile);
            match universal_viewer_failure(frame, &child, tables, budget) {
                Some(true) => {}
                other => return other,
            }
        }
        Some(true)
    } else {
        // Hidden: partition the record-consistent support by the
        // declared field's deterministic choice; every branch must
        // fail with the posterior restricted to the hands that chose
        // its tile.
        let slot = frame
            .hidden_seats
            .iter()
            .position(|s| *s == seat)
            .expect("a non-viewer seat is hidden");
        let own = state.played_by[seat.index()];
        let others = (0..Seat::COUNT)
            .filter(|i| *i != seat.index())
            .fold(DominoSet::EMPTY, |acc, i| acc.union(state.played_by[i]));
        let record = PublicRecord {
            leader: state.leader,
            trick_plays: &state.plays,
            banked: state.banked,
            root: frame.position,
            history: &state.history,
        };
        let mut branches: Vec<(Domino, Vec<(DominoSet, u128)>)> = Vec::new();
        for (hand, weight) in tables[slot] {
            if !own.is_subset_of(*hand) || !hand.is_disjoint(others) {
                continue;
            }
            let remaining = hand.difference(own);
            let legal = legal_plays(decl, remaining, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            let tile = frame.field.choose(decl, remaining, legal, &record);
            assert!(legal.contains(tile), "a field chooses a legal tile");
            match branches.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, kept)) => kept.push((*hand, *weight)),
                None => branches.push((tile, vec![(*hand, *weight)])),
            }
        }
        if branches.is_empty() {
            // No record-consistent hand: the branch represents no
            // world — vacuously doomed (the §16 walk's same case).
            // This vacuity is also what disciplines the per-seat
            // relaxation from below: a phantom line must keep every
            // seat's kept hand disjoint from every tile the OTHER
            // seats actually play, so phantoms strand on physical
            // tile conservation as the pool drains — often before
            // any phantom make can be reached (the loose-boss gate
            // certifies exactly because of this squeeze).
            return Some(true);
        }
        branches.sort_by_key(|(t, _)| t.index());
        for (tile, kept) in &branches {
            let mut child = state.clone();
            child.play(frame.position, *tile);
            let mut child_tables = tables;
            child_tables[slot] = kept;
            match universal_viewer_failure(frame, &child, child_tables, budget) {
                Some(true) => {}
                other => return other,
            }
        }
        Some(true)
    }
}

// ---------------------------------------------------------------------------
// Exact class mass.
// ---------------------------------------------------------------------------

/// The exact mass of a census class: the oracle counts the restricted
/// belief for at most two restricted factors; with all three
/// restricted, the third hand is DETERMINED (hidden hands partition
/// the pool), so the mass is a two-table sum with a membership lookup
/// — `Σ_{A,B disjoint} w_A · w_B · w_C(pool \ A \ B)`.
fn class_mass(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    hidden_seats: &[Seat; 3],
    restriction: &[Option<Vec<(DominoSet, u128)>>; 3],
) -> u128 {
    let restricted: Vec<usize> = (0..3).filter(|i| restriction[*i].is_some()).collect();
    if restricted.len() < 3 {
        let mut b = belief.clone();
        for i in &restricted {
            b = b.with_factor_table(
                hidden_seats[*i],
                restriction[*i].clone().expect("restricted"),
            );
        }
        return oracle.mass(&b);
    }
    let pool = belief.kernel().pool();
    // Iterate the two smallest tables; look the determined complement
    // up in the largest.
    let mut order = [0usize, 1, 2];
    order.sort_by_key(|i| {
        restriction[*i]
            .as_ref()
            .map(|t| t.len())
            .expect("restricted")
    });
    let (a, b, c) = (order[0], order[1], order[2]);
    let ta = restriction[a].as_ref().expect("restricted");
    let tb = restriction[b].as_ref().expect("restricted");
    let tc = restriction[c].as_ref().expect("restricted");
    let mut lookup: Vec<(u32, u128)> = tc.iter().map(|(h, w)| (h.bits(), *w)).collect();
    lookup.sort_by_key(|(bits, _)| *bits);
    let mut total: u128 = 0;
    for (ha, wa) in ta {
        let rest = pool.difference(*ha);
        for (hb, wb) in tb {
            if !hb.is_subset_of(rest) {
                continue;
            }
            let hc = rest.difference(*hb);
            if let Ok(k) = lookup.binary_search_by_key(&hc.bits(), |(bits, _)| *bits) {
                let w = wa
                    .checked_mul(*wb)
                    .and_then(|x| x.checked_mul(lookup[k].1))
                    .expect("an exact mass fits u128");
                total = total.checked_add(w).expect("an exact mass fits u128");
            }
        }
    }
    total
}

// ---------------------------------------------------------------------------
// The census descent.
// ---------------------------------------------------------------------------

struct Descent<'a> {
    oracle: &'a dyn ExactCoverOracle,
    belief: &'a FactorBelief,
    frame: &'a WalkFrame<'a>,
    root_state: &'a NodeState,
    supports: &'a [Vec<(DominoSet, u128)>; 3],
    /// Hidden slots in acting order after the root action.
    order: [usize; 3],
    /// Post-action trick plays — the declared classification record.
    class_plays: Vec<Domino>,
    spec: &'a DoomSpec,
    budget: u64,
}

impl Descent<'_> {
    fn tables<'b>(
        &'b self,
        restriction: &'b [Option<Vec<(DominoSet, u128)>>; 3],
    ) -> [&'b [(DominoSet, u128)]; 3] {
        core::array::from_fn(|i| {
            restriction[i]
                .as_deref()
                .unwrap_or(self.supports[i].as_slice())
        })
    }

    /// The declared punish-priority key of one class: larger walks
    /// first. Opponents of the viewer rank nastiest-first (can-ruff,
    /// can-beat, trump count, highest trump, count pips); the viewer's
    /// partner ranks weakest-first (the same coordinates reversed).
    /// Support size breaks ties toward mass; the signature order is
    /// the final deterministic tie-break.
    fn punish_key(&self, seat: Seat, sig: &ClassSignature, support: usize) -> [u64; 6] {
        let viewer_team = self.frame.viewer.team() == seat.team();
        let ruff = u64::from(sig.can_ruff);
        let beat = u64::from(sig.can_beat);
        let trumps = u64::from(sig.trump_count);
        let high = u64::from(sig.highest_trump.map_or(0, |r| r + 1));
        let pips = u64::from(sig.count_pips);
        let n = u64::try_from(support).expect("a support fits u64");
        if viewer_team {
            // The partner: weakest first.
            [1 - ruff, 1 - beat, 7 - trumps, 13 - high, 35 - pips, n]
        } else {
            [ruff, beat, trumps, high, pips, n]
        }
    }

    /// Partition one level, walk each class, descend on escapes. In a
    /// full census children partition the parent exactly — asserted
    /// (§46). In a priority census only the top `descend_top` classes
    /// are walked; the rest are counted skipped, unweighed.
    fn refine_level(
        &mut self,
        level: usize,
        restriction: &mut [Option<Vec<(DominoSet, u128)>>; 3],
        parent_mass: u128,
        path: &mut Vec<(Seat, ClassSignature)>,
        census: &mut DoomCensus,
    ) {
        let slot = self.order[level - 1];
        let seat = self.frame.hidden_seats[slot];
        let decl = self.frame.position.decl;
        let mut classes: BTreeMap<ClassSignature, Vec<(DominoSet, u128)>> = BTreeMap::new();
        for (hand, weight) in &self.supports[slot] {
            let sig = class_signature(decl, *hand, &self.class_plays, self.spec.critical);
            classes.entry(sig).or_default().push((*hand, *weight));
        }
        let mut ordered: Vec<(ClassSignature, Vec<(DominoSet, u128)>)> =
            classes.into_iter().collect();
        match self.spec.descend_top {
            // Full census: largest support first (the biggest
            // certifiable chunks buy the most upper per node).
            None => ordered.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(&b.0))),
            // Priority census: the declared punish order.
            Some(_) => ordered.sort_by(|a, b| {
                self.punish_key(seat, &b.0, b.1.len())
                    .cmp(&self.punish_key(seat, &a.0, a.1.len()))
                    .then(a.0.cmp(&b.0))
            }),
        }
        let mut children_mass: u128 = 0;
        let mut feasible_taken: usize = 0;
        for (sig, hands) in ordered {
            if let Some(k) = self.spec.descend_top {
                // The cut counts FEASIBLE classes only: empties pass
                // through the ranking unharvested (two nasty seats
                // often chase the same physical tiles, and a cut that
                // spent its k on empty intersections would harvest
                // nothing — the h0-t1 scout's finding).
                if feasible_taken >= k {
                    census.classes_skipped += 1;
                    continue;
                }
            }
            restriction[slot] = Some(hands);
            let mass = class_mass(
                self.oracle,
                self.belief,
                &self.frame.hidden_seats,
                restriction,
            );
            children_mass = children_mass
                .checked_add(mass)
                .expect("an exact mass fits u128");
            if mass == 0 {
                census.classes_empty += 1;
                continue;
            }
            feasible_taken += 1;
            if self.budget == 0 {
                census.classes_refused += 1;
                census.refused_mass += mass;
                continue;
            }
            let mut allot = self.spec.walk_cap.min(self.budget);
            let before = allot;
            let verdict = universal_viewer_failure(
                self.frame,
                self.root_state,
                self.tables(restriction),
                &mut allot,
            );
            let used = before - allot;
            self.budget -= used;
            census.nodes += used;
            census.classes_walked += 1;
            match verdict {
                Some(true) => {
                    census.classes_doomed += 1;
                    census.doomed_mass += mass;
                    let mut leaf_path = path.clone();
                    leaf_path.push((seat, sig));
                    census.doomed_leaves.push(DoomedLeaf {
                        path: leaf_path,
                        mass,
                        nodes: used,
                    });
                }
                Some(false) if level < self.spec.max_level => {
                    path.push((seat, sig));
                    self.refine_level(level + 1, restriction, mass, path, census);
                    path.pop();
                }
                Some(false) => {
                    census.classes_survived += 1;
                    census.survived_mass += mass;
                }
                None => {
                    census.classes_refused += 1;
                    census.refused_mass += mass;
                }
            }
        }
        restriction[slot] = None;
        if self.spec.descend_top.is_none() {
            assert_eq!(
                children_mass, parent_mass,
                "a signature partition is a partition (§46 discipline)"
            );
        } else {
            assert!(
                children_mass <= parent_mass,
                "walked children never exceed the parent (§46 discipline)"
            );
        }
    }
}

/// One action's doom census at one root: build the post-action state,
/// pre-walk the unrestricted belief (level 0), then descend the
/// signature partition under the declared budget. Deterministic
/// throughout.
pub fn doom_census(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    action: Domino,
    spec: &DoomSpec,
) -> DoomCensus {
    assert!(
        (1..=3).contains(&spec.max_level),
        "the descent restricts one to three hidden seats"
    );
    let belief = FactorBelief::uniform_root(root, position, field);
    let z = oracle.mass(&belief);
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let hidden_seats: [Seat; 3] = core::array::from_fn(|i| kernel.hidden()[i].seat);
    let frame = WalkFrame {
        position,
        viewer,
        viewer_root_hand: kernel.viewer_hand(),
        hidden_seats,
        total_plays: kernel.viewer_hand().len()
            + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>(),
        field,
    };
    let mut state = NodeState {
        leader: position.leader,
        plays: position.trick_plays.clone(),
        banked: position.banked,
        played_by: [DominoSet::EMPTY; Seat::COUNT],
        history: Vec::new(),
    };
    assert_eq!(
        state.leader.plus(state.plays.len()),
        viewer,
        "a census action is the viewer's"
    );
    state.play(position, action);
    let supports: [Vec<(DominoSet, u128)>; 3] =
        core::array::from_fn(|i| belief.factors()[i].support());
    // Hidden slots in acting order after the root seat.
    let mut order = [0usize, 1, 2];
    order.sort_by_key(|i| (hidden_seats[*i].index() + Seat::COUNT - viewer.index()) % Seat::COUNT);
    let mut census = DoomCensus {
        action,
        fiber: z,
        doomed_mass: 0,
        survived_mass: 0,
        refused_mass: 0,
        classes_walked: 0,
        classes_doomed: 0,
        classes_survived: 0,
        classes_refused: 0,
        classes_empty: 0,
        classes_skipped: 0,
        nodes: 0,
        whole_fiber: false,
        doomed_leaves: Vec::new(),
        upper: BigRational::from_integer(BigInt::from(1)),
    };
    let mut budget = spec.node_budget;
    // Level 0: the unrestricted pre-walk (the §17-dual zero-cost path).
    {
        let mut allot = spec.walk_cap.min(budget);
        let before = allot;
        let tables: [&[(DominoSet, u128)]; 3] = core::array::from_fn(|i| supports[i].as_slice());
        let verdict = universal_viewer_failure(&frame, &state, tables, &mut allot);
        let used = before - allot;
        budget -= used;
        census.nodes += used;
        census.classes_walked += 1;
        if verdict == Some(true) {
            census.classes_doomed += 1;
            census.doomed_mass = z;
            census.whole_fiber = true;
            census.doomed_leaves.push(DoomedLeaf {
                path: Vec::new(),
                mass: z,
                nodes: used,
            });
        }
    }
    if !census.whole_fiber {
        let class_plays = state.plays.clone();
        let mut descent = Descent {
            oracle,
            belief: &belief,
            frame: &frame,
            root_state: &state,
            supports: &supports,
            order,
            class_plays,
            spec,
            budget,
        };
        let mut restriction: [Option<Vec<(DominoSet, u128)>>; 3] = [None, None, None];
        let mut path = Vec::new();
        descent.refine_level(1, &mut restriction, z, &mut path, &mut census);
    }
    let accounted = census
        .doomed_mass
        .checked_add(census.survived_mass)
        .and_then(|m| m.checked_add(census.refused_mass))
        .expect("an exact mass fits u128");
    assert!(
        accounted <= census.fiber,
        "leaf masses never exceed the fiber"
    );
    census.upper = BigRational::new(
        BigInt::from(census.fiber - census.doomed_mass),
        BigInt::from(census.fiber),
    );
    census
}

// ---------------------------------------------------------------------------
// The per-world enumeration (the census's counted ground truth).
// ---------------------------------------------------------------------------

/// One action's exact per-world doom enumeration. Where the class walk
/// certifies wholesale over a relaxation, this counts the truth one
/// world at a time: a world is doomed exactly when NO viewer line —
/// even a world-aware one — reaches the objective against the declared
/// deterministic field. The doomed count is therefore the LARGEST doom
/// mass any sound doom reasoning can ever certify, and
/// `(Z − doomed)/Z` the tightest upper this bound family admits; the
/// class census aspires to reach this number cheaply, and the gap
/// between them prices the structural work remaining. Cost is linear
/// in the fiber with a small per-world search — affordable where the
/// class walk's relaxation is not, and a bill the report states
/// honestly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoomEnumeration {
    pub action: Domino,
    /// Worlds enumerated — asserted equal to the oracle's fiber mass.
    pub fiber: u128,
    /// Worlds in which no viewer continuation makes.
    pub doomed: u128,
    /// Search nodes across every world's line walk.
    pub nodes: u64,
    /// `(fiber − doomed)/fiber` — deterministic, δ-free.
    pub upper: BigRational,
    /// The doom mass grouped by the FIRST RESPONDER's root-hand
    /// signature at the post-action record — the structural profile
    /// the class census's vocabulary would have to resolve. Entries
    /// `(signature, doomed, total)` in signature order.
    pub by_first_responder: Vec<(ClassSignature, u128, u128)>,
}

struct LineCtx<'a> {
    frame: &'a WalkFrame<'a>,
    history: Vec<Domino>,
    nodes: u64,
}

/// Small copyable line state for one world's search.
#[derive(Clone, Copy)]
struct LineState {
    leader: Seat,
    plays: [Domino; 4],
    play_count: usize,
    banked: [u32; 2],
    remaining: [DominoSet; 4],
    played: usize,
}

fn line_apply(position: &RootPosition, mut st: LineState, tile: Domino) -> LineState {
    let seat = st.leader.plus(st.play_count);
    assert!(
        st.remaining[seat.index()].remove(tile),
        "a played tile leaves its hand"
    );
    st.plays[st.play_count] = tile;
    st.play_count += 1;
    st.played += 1;
    if st.play_count == 4 {
        let trick = Trick::new(st.leader, st.plays).expect("four distinct tiles");
        let winner = trick.winner(position.decl);
        st.banked[winner.team().index()] += trick.points();
        st.leader = winner;
        st.play_count = 0;
    }
    st
}

/// Can ANY viewer line make from here, with every other seat playing
/// the declared field on its known hand? Viewer tiles are tried in
/// descending index (a declared order — strong tiles first finds make
/// lines early; the answer is order-independent).
fn line_can_make(ctx: &mut LineCtx<'_>, position: &RootPosition, st: LineState) -> bool {
    ctx.nodes += 1;
    let at_terminal = st.played == ctx.frame.total_plays;
    if let Some(u) = decided_success(position, ctx.frame.viewer, st.banked, at_terminal) {
        return u;
    }
    let seat = st.leader.plus(st.play_count);
    let remaining = st.remaining[seat.index()];
    let led = if st.play_count == 0 {
        None
    } else {
        Some(position.decl.led_context(st.plays[0]))
    };
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    if seat == ctx.frame.viewer {
        let mut tiles: Vec<Domino> = legal.iter().collect();
        tiles.sort_by_key(|t| core::cmp::Reverse(t.index()));
        for tile in tiles {
            let child = line_apply(position, st, tile);
            ctx.history.push(tile);
            let made = line_can_make(ctx, position, child);
            ctx.history.pop();
            if made {
                return true;
            }
        }
        false
    } else {
        let tile = if legal.len() == 1 {
            legal.iter().next().expect("one legal tile")
        } else {
            let record = PublicRecord {
                leader: st.leader,
                trick_plays: &st.plays[..st.play_count],
                banked: st.banked,
                root: position,
                history: &ctx.history,
            };
            let chosen = ctx
                .frame
                .field
                .choose(position.decl, remaining, legal, &record);
            assert!(legal.contains(chosen), "a field chooses a legal tile");
            chosen
        };
        let child = line_apply(position, st, tile);
        ctx.history.push(tile);
        let made = line_can_make(ctx, position, child);
        ctx.history.pop();
        made
    }
}

/// Enumerate every world of the root fiber after `action` and count
/// the doomed exactly. `progress` is called every 1024 outer hands
/// with (outer done, outer total, doomed so far, nodes so far) — a
/// reporting hook only, no effect on the result.
pub fn doom_enumeration(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
    action: Domino,
    spec: &DoomSpec,
    progress: &mut dyn FnMut(u64, u64, u128, u64),
) -> DoomEnumeration {
    let belief = FactorBelief::uniform_root(root, position, field);
    let z = oracle.mass(&belief);
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let hidden_seats: [Seat; 3] = core::array::from_fn(|i| kernel.hidden()[i].seat);
    let frame = WalkFrame {
        position,
        viewer,
        viewer_root_hand: kernel.viewer_hand(),
        hidden_seats,
        total_plays: kernel.viewer_hand().len()
            + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>(),
        field,
    };
    // The root line state BEFORE the action (played counts pre-root
    // trick plays as 0 — `played` measures post-root plays plus the
    // action itself via total_plays + 1 at terminal).
    let mut base = LineState {
        leader: position.leader,
        plays: [Domino::from_index(0).expect("a tile"); 4],
        play_count: 0,
        banked: position.banked,
        remaining: [DominoSet::EMPTY; Seat::COUNT],
        played: 0,
    };
    for (i, d) in position.trick_plays.iter().enumerate() {
        base.plays[i] = *d;
        base.play_count += 1;
    }
    base.remaining[viewer.index()] = kernel.viewer_hand();
    assert_eq!(
        base.leader.plus(base.play_count),
        viewer,
        "an enumeration action is the viewer's"
    );
    // Descent order: hidden slots by acting distance (as the census).
    let mut order = [0usize, 1, 2];
    order.sort_by_key(|i| (hidden_seats[*i].index() + Seat::COUNT - viewer.index()) % Seat::COUNT);
    let (outer, mid, last) = (order[0], order[1], order[2]);
    let outer_hands = hands_of_allowed(kernel.allowed(outer), kernel.hidden()[outer].capacity);
    let class_plays: Vec<Domino> = {
        // The post-action record the aggregation signatures read.
        let st = line_apply(position, base, action);
        st.plays[..st.play_count].to_vec()
    };
    let mut ctx = LineCtx {
        frame: &frame,
        history: vec![action],
        nodes: 0,
    };
    let mut doomed: u128 = 0;
    let mut worlds: u128 = 0;
    let mut by_sig: BTreeMap<ClassSignature, (u128, u128)> = BTreeMap::new();
    let outer_total = u64::try_from(outer_hands.len()).expect("a support fits u64");
    for (done, outer_hand) in outer_hands.iter().enumerate() {
        let sig = class_signature(position.decl, *outer_hand, &class_plays, spec.critical);
        let entry = by_sig.entry(sig).or_insert((0, 0));
        let rest = kernel.pool().difference(*outer_hand);
        let mid_allowed = kernel.allowed(mid).intersection(rest);
        for mid_hand in hands_of_allowed(mid_allowed, kernel.hidden()[mid].capacity) {
            let last_hand = rest.difference(mid_hand);
            if last_hand.len() != kernel.hidden()[last].capacity
                || !last_hand.is_subset_of(kernel.allowed(last))
            {
                continue;
            }
            worlds += 1;
            entry.1 += 1;
            let mut st = base;
            st.remaining[hidden_seats[outer].index()] = *outer_hand;
            st.remaining[hidden_seats[mid].index()] = mid_hand;
            st.remaining[hidden_seats[last].index()] = last_hand;
            let st = line_apply(position, st, action);
            if !line_can_make(&mut ctx, position, st) {
                doomed += 1;
                entry.0 += 1;
            }
        }
        if (done + 1) % 1024 == 0 {
            progress(
                u64::try_from(done + 1).expect("fits"),
                outer_total,
                doomed,
                ctx.nodes,
            );
        }
    }
    assert_eq!(worlds, z, "the enumeration covers the fiber exactly");
    DoomEnumeration {
        action,
        fiber: z,
        doomed,
        nodes: ctx.nodes,
        upper: BigRational::new(BigInt::from(z - doomed), BigInt::from(z)),
        by_first_responder: by_sig
            .into_iter()
            .map(|(sig, (d, t))| (sig, d, t))
            .collect(),
    }
}

/// Every `k`-subset of `allowed` in ascending-tile lexicographic
/// order (the factor-support order).
fn hands_of_allowed(allowed: DominoSet, k: usize) -> Vec<DominoSet> {
    let tiles: Vec<Domino> = allowed.iter().collect();
    let n = tiles.len();
    if k > n {
        return Vec::new();
    }
    let mut idx: Vec<usize> = (0..k).collect();
    let mut out = Vec::new();
    loop {
        out.push(idx.iter().map(|&i| tiles[i]).collect());
        let mut advanced = false;
        let mut i = k;
        while i > 0 {
            i -= 1;
            if idx[i] + k - i < n {
                idx[i] += 1;
                for j in (i + 1)..k {
                    idx[j] = idx[j - 1] + 1;
                }
                advanced = true;
                break;
            }
        }
        if !advanced {
            return out;
        }
    }
}

/// The enumeration's fact: a deterministic, δ-free upper
/// `(Z − doomed)/Z` under the `doom-enum-v1` authority (no budget
/// coordinates — the enumeration is exact or absent).
pub fn enumeration_fact(enumeration: &DoomEnumeration, field_id: &str) -> Option<Fact> {
    if enumeration.doomed == 0 {
        return None;
    }
    Some(Fact::Bound(BoundFact::upper(
        enumeration.action,
        enumeration.upper.clone(),
        &format!("doom-enum-v1:{field_id}"),
        ProofTag::Deterministic,
    )))
}

// ---------------------------------------------------------------------------
// The producer.
// ---------------------------------------------------------------------------

/// The authority string of one census's fact: the field identity and
/// the full declared spec travel with the bound.
pub fn census_authority(field_id: &str, spec: &DoomSpec) -> String {
    format!(
        "doom-census-v1:{field_id}:n{}:c{}:l{}:k{}",
        spec.node_budget,
        spec.walk_cap,
        spec.max_level,
        spec.critical.bits(),
    )
}

/// The fact one census certifies: a deterministic upper
/// `(Z − M_doom)/Z` on its action, or nothing when no doom was
/// certified (an honest empty hand — never a vacuous upper 1).
pub fn census_fact(census: &DoomCensus, field_id: &str, spec: &DoomSpec) -> Option<Fact> {
    if census.doomed_mass == 0 {
        return None;
    }
    Some(Fact::Bound(BoundFact::upper(
        census.action,
        census.upper.clone(),
        &census_authority(field_id, spec),
        ProofTag::Deterministic,
    )))
}

/// The §49 doom-census producer: one census per legal root action,
/// one deterministic upper per certified census. Idempotent against
/// the append-only store — a fact already present is proposed once,
/// never duplicated.
pub struct DoomCensusProducer<'a> {
    pub oracle: &'a dyn ExactCoverOracle,
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub field: &'a dyn SlicePolicy,
    pub spec: DoomSpec,
}

impl ProofProducer for DoomCensusProducer<'_> {
    fn name(&self) -> &str {
        "doom-census-v1"
    }

    fn produce(&self, state: &ProofState) -> Vec<Fact> {
        assert_eq!(
            state.identity.root_id,
            root_identity(self.root, self.position),
            "the producer's context is the state's root"
        );
        assert_eq!(
            state.identity.contract, self.position.bid,
            "the producer's contract is the state's"
        );
        let mut facts = Vec::new();
        for action in &state.legal {
            let census = doom_census(
                self.oracle,
                self.root,
                self.position,
                self.field,
                *action,
                &self.spec,
            );
            if let Some(fact) = census_fact(&census, self.field.id(), &self.spec) {
                if state.facts().iter().any(|sf| sf.fact == fact) {
                    continue;
                }
                facts.push(fact);
            }
        }
        facts
    }
}
