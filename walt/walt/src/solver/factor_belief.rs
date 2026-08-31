//! `solver::factor_belief` — the counted-belief Slices C through F: the
//! seat-factored belief, the exact-cover contraction interface, backend
//! zero (the shipped `FiberDp` wrapped for 0/1 factors — stage C0), the
//! general support contraction ([`SupportOracle`] — Slice D), the §23
//! factorized fixed-policy recursion ([`viewer_success_mass`]), the
//! §48 factorized grammar best response ([`grammar_success_mass`] —
//! Slice E), and the §49 consequence-CEGAR hand-class instrument
//! ([`refine_to_action_exact`] — Slice F).
//!
//! EXPLORATORY tier. Mathematical source:
//! `walt/math/counted_belief_sandwich_v0.1.md` Parts V–VII (§18–26, §43,
//! §46 stage C0, §47 Slice D, §48 Slice E, §49 Slice F with §27–31, §23),
//! adopted by rulings CBS-A6 and CBS-A9 (`walt/CENSUS-RULINGS.md`);
//! design register `walt/FACTOR-BELIEF.md`; intake companion
//! `walt/math/counted_belief_sandwich_v0.1_intake.md`.
//!
//! The objects (parent §18–21): a **hand factor** is one hidden seat's
//! exact nonnegative integer weight table over its possible ROOT hands
//! `φ_{s,h}: C(U, k_s) → ℕ`; a **factor belief** joins the seat factors
//! under the disjoint-cover constraint, together with the public history
//! that produced them. Factors always range over root hands — a played
//! tile advances the public history, never re-bases the factor domain.
//! The uniform lawful fiber is the special case where every factor is the
//! 0/1 legality predicate `hand ⊆ kernel.allowed(slot)`.
//!
//! Theorem 20.1 (posterior closure, adjudicated CBS-A6) is the load-bearing
//! statement: under a seat-local field, conditioning on an observed hidden
//! action multiplies ONLY the acting seat's factor by its action
//! likelihood, and the posterior stays a product of seat factors coupled
//! only by disjoint cover. Void inference is subsumed — a hand whose legal
//! set forces a different tile gets likelihood zero, so no separate void
//! bookkeeping exists here. Theorem 23.1's focal case is [`FactorBelief::
//! focal_play`]: a focal action changes the public history and NO factor.
//!
//! The one-ply contraction target (§21, boxed):
//!
//! ```text
//! Z_ht = Σ_{A ∈ C(U, k_s)} φ_{s,h}(A) · K_s(t | A, h) · C_{-s,h}(U \ A)
//! ```
//!
//! with `Pr(t | h) = Z_ht / Z_h` — a loop over the acting seat's possible
//! hands weighted by exact compatible completions, never a loop over
//! complete deals. At a trick-1 root that is 116,280 hands standing in for
//! 399,072,960 worlds (§22).
//!
//! DECLARED C0 DOMAIN of backend zero. [`FiberOracle`] serves exactly the
//! uniform-root special case and its one-ply conditionings: every factor
//! is either the kernel's own 0/1 legality predicate or ONE explicit
//! table produced by [`ExactCoverOracle::condition`]. A contraction
//! across two or more table factors is REFUSED by backend zero, not
//! approximated — it is the declared domain of the Slice D backend
//! [`SupportOracle`] (§47), which contracts any mix of uniform and table
//! factors by walking explicit supports (§25.2's acting-hand loop
//! generalized to conditioned completions, §25.4's sparse-support shape)
//! and is gated by extensional parity with backend zero on the C0 domain
//! and with surviving-world enumeration beyond it. Fields are
//! deterministic [`SlicePolicy`] reads (the trait's documented contract);
//! a stochastic field needs an explicit tape factor (CBS-A6's boundary
//! obligation) and has no entry point in this module.
//!
//! SLICE D (§47, §23): [`viewer_success_mass`] evaluates ONE frozen focal
//! policy under the declared field by the §23 factorized Bellman
//! recursion, cleared of denominators. The recursion computes the
//! viewer-objective success MASS `M(B)` — the exact weight of worlds
//! whose terminal pmake indicator is true under (focal, field) — so the
//! §23 value is the exact pair `V(B) = M(B)/Z(B)` and no rational ever
//! appears:
//!
//! ```text
//! decided:  M(B) = u·Z(B)          (u the decided indicator, §23 terminal)
//! focal:    M(B) = M(B·ρ(I))       (§23 focal, one action — no max)
//! hidden:   M(B) = Σ_t M(B·t)      (§23 field node × Z, by conservation)
//! ```
//!
//! Exactness is Theorem 23.1 restricted to the singleton policy class
//! {ρ}: branch ratios are exact conditional probabilities (Theorem 20.1),
//! and mass conservation `Z = Σ_t Z_t` (asserted at every contraction)
//! clears every denominator. The decided cutoff is
//! [`decided_success`] — monotone in banked points, so truncation never
//! changes the indicator (the same law the bundled walker relies on).
//!
//! SLICE E (§48, §12): [`grammar_success_mass`] is the same recursion
//! with the focal case's single frozen action replaced by a MAX over the
//! grammar's actions at the focal information state:
//!
//! ```text
//! grammar focal:  M^G(B) = max_{t ∈ G(I)} M^G(B·t)
//! ```
//!
//! The max is lawful on the cleared side because every focal child shares
//! `Z(B)` — a focal play changes no factor (Theorem 23.1, focal case) —
//! so the max of masses IS the max of values. `M^G(B)` equals
//! `max_{ρ ∈ Π^G} M_ρ(B)` (the §12 grammar optimum `Q^G` as a mass) by
//! the cylinder-partition argument the Slice B walk asserts nodewise
//! (Theorem 9.1): the viewer's information states are in bijection with
//! the recursion's focal nodes — the belief IS a function of the public
//! history, and the focal hand is constant across the represented worlds
//! — so per-node choices compose into one lawful deterministic policy,
//! and the branch-wise optimum distributes through the hidden sum
//! because different branches' continuations are chosen at disjoint
//! information states. No strategy fusion is possible by construction:
//! [`PolicyGrammar::actions`] sees only public data and the focal hand
//! (§11's non-theorem stays a fence). §48's second sentence is a fence
//! too and it is kept: NOTHING here maximizes over the full action set —
//! the exact lower witness `Q^G` is the deliverable, and "only after
//! this lands should the full action set be enabled" means a later
//! slice, on its own gates. The recursion returns the value, not an
//! argmax — extracting an optimal grammar policy needs a declared tie
//! order and is not a Slice E claim.
//!
//! SLICE F (§49, §27–31): [`refine_to_action_exact`] instruments hand
//! classes at the field-classification bottleneck — the per-hand
//! `field.choose` loop inside every contraction, measured at 99% of the
//! opening-root bill (stage C2). A [`ClassSignature`] is §28's feature
//! map `κ` on the acting seat's REMAINING hands with §49's starting
//! vocabulary (critical tile membership, trump count and highest trump,
//! led-suit count, count-tile possession, current-winner and ruff
//! possibility), parameterized by the §31 critical tile set. The CEGAR
//! loop is §30 verbatim: partition the support, aggregate action-uniform
//! classes exactly, and for the largest-mass non-uniform class produce a
//! WITNESS PAIR — two same-class hands with different field actions —
//! whose lowest differing tile enters the critical set (every refinement
//! carries its witness, §49's requirement). Theorem 30.1 is the safety
//! law the gates assert: per-branch mass intervals `[L_t, U_t]` (exact
//! classes below, exact plus action-bounded residual above) NEST as the
//! critical set grows, residual class mass falls monotonically to zero,
//! and the fully refined endpoint reproduces the exact contraction —
//! completeness because two distinct hands in one class always differ in
//! a tile outside the critical set. WHAT THIS SLICE IS NOT: the
//! instrument pays one classification per support hand — the same bill
//! as `branch_masses` — to measure what an action-exact class verifier
//! WOULD make aggregatable; §49 measures residual class mass and root
//! interval impact, never classifier accuracy, and nothing here is a
//! faster classifier. A verifier that certifies action-exactness
//! without per-hand classification is a later construction (§29's
//! verifier vocabulary is its interface).
//!
//! Deviations from the `walt/FACTOR-BELIEF.md` trait sketch, under L2-A3's
//! naming latitude, recorded here and in the register:
//! - `branch_masses`/`condition` take no seat argument — the acting seat
//!   is the derived view `seat_to_move()`, and passing it would store one
//!   authority twice;
//! - masses are `u128` with checked arithmetic (the kernel's own counting
//!   width; the trick-1 partition function is < 2^64), not `BigUint`;
//! - `count_cell` stays deferred: Slice F's hand class turned out to be
//!   a ONE-seat predicate — `marginal` counts it exactly — while Part
//!   IV's multi-seat structural cells still have no consumer; the slice
//!   that first needs a cross-seat cell mass gives them their type.
//!
//! SCORE PROFILE (anytime proof-state §18, Phase 2, adopted APS-A2):
//! [`viewer_score_profile`] is the Slice D recursion carrying the full
//! 43-bin exact score object instead of one tail sum — bin `s` holds the
//! integer world mass whose complete continuation banks exactly `s`
//! points for the DECLARING team:
//!
//! ```text
//! terminal:  H(B; s) = Z(B)·1{s = banked_decl}   (§18 terminal case)
//! focal:     H(B; ·) = H(B·ρ(I); ·)              (§18 focal case)
//! hidden:    H(B; s) = Σ_t H(B·t; s)             (§18 branch addition)
//! ```
//!
//! The profile is viewer-independent — the viewer parity enters only at
//! projection (`tail(bid)` for the declaring viewer, `Z − tail(bid)` for
//! the setting viewer), and the profile never reads the bid, so ONE
//! profile answers every contract threshold (§44's reuse, mechanical).
//! The price of the whole curve is the decided cutoff: a monotone
//! decided state knows the indicator, not the final score, so this
//! recursion walks past [`decided_success`]'s early exit to true
//! terminals (§18's own caveat) — the gates assert `decided_early == 0`
//! and the probe reports the cost against [`viewer_success_mass`]
//! honestly. Mass conservation `Σ_s H(s) = Z` and the tail projection
//! `Σ_{s≥bid} H(s) = M` are gate families, and no threshold-wise
//! envelope of profiles is ever built here — a profile is the record of
//! ONE policy (the §20 fence, binding at APS-A4).
//!
//! No belief cache exists at C0. When one arrives, its key must be the
//! FULL §43 identity list carried by [`FactorBelief`] — a hit under an
//! omitted coordinate is the PiKey defect reborn (CBS-A6).

use std::collections::BTreeMap;

use crate::kernel::fiber::binomial;
use crate::kernel::{FiberDp, Kernel, HIDDEN_SEATS};
use crate::rules::{legal_plays, Context, Decl, Domino, DominoSet, Seat, Trick};
use crate::solver::adaptive::{
    decided_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::grammar::PolicyGrammar;

// ---------------------------------------------------------------------------
// Hand factors (parent §18–19).
// ---------------------------------------------------------------------------

/// One hidden seat's weight table representation. Weights are exact
/// nonnegative integers; a hand absent from a table carries weight zero.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FactorWeights {
    /// The 0/1 legality predicate: weight 1 on every capacity-sized subset
    /// of `allowed`, weight 0 elsewhere. The uniform-root special case.
    UniformLawful { allowed: DominoSet },
    /// An explicit table over root hands, in the deterministic enumeration
    /// order of the support it was refined from. Zero-weight entries are
    /// never stored.
    Table(Vec<(DominoSet, u128)>),
}

/// One hidden seat's factor: identity (seat), root capacity, and the
/// weight table (parent §43: representation and weights are both identity
/// components).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HandFactor {
    seat: Seat,
    capacity: usize,
    weights: FactorWeights,
}

impl HandFactor {
    pub fn seat(&self) -> Seat {
        self.seat
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn weights(&self) -> &FactorWeights {
        &self.weights
    }

    /// The factor's support with weights: every root hand of positive
    /// weight, in deterministic order (lexicographic combinations over
    /// ascending tile index for the uniform case; table order otherwise).
    pub fn support(&self) -> Vec<(DominoSet, u128)> {
        match &self.weights {
            FactorWeights::UniformLawful { allowed } => hands_of(*allowed, self.capacity)
                .into_iter()
                .map(|h| (h, 1u128))
                .collect(),
            FactorWeights::Table(entries) => entries.clone(),
        }
    }
}

/// Every `k`-subset of `allowed`, as lexicographic combinations over the
/// ascending tile order. Empty when `k` exceeds the set (an empty fiber,
/// not an error).
fn hands_of(allowed: DominoSet, k: usize) -> Vec<DominoSet> {
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

// ---------------------------------------------------------------------------
// The factor belief (parent §43's binding identity list).
// ---------------------------------------------------------------------------

/// A seat-factored belief: the root physical fiber (kernel), the public
/// state sufficient for every field read (root position plus post-root
/// history), the field identity, and every seat factor. This is the §43
/// identity list made a struct; equality is componentwise and total. The
/// stochastic-tape coordinate is structurally absent: C0 fields are
/// deterministic `SlicePolicy` reads.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FactorBelief {
    /// The evidence-stream root identity (kernel fiber ⊕ public position)
    /// of the ROOT this belief refines — unchanged by conditioning.
    root_id: u64,
    kernel: Kernel,
    position: RootPosition,
    /// Every tile played after the root, in play order.
    history: Vec<Domino>,
    /// The field identity every conditioning in `history` was computed
    /// under. Ops assert the presented field matches.
    field_id: String,
    factors: [HandFactor; HIDDEN_SEATS],
}

impl FactorBelief {
    /// The uniform root belief: every factor is the kernel's own 0/1
    /// legality predicate. The exact-cover mass of this belief is
    /// `|Φ(C)|` by construction.
    pub fn uniform_root(
        root: &CanonicalRoot,
        position: &RootPosition,
        field: &dyn SlicePolicy,
    ) -> FactorBelief {
        let kernel = root.kernel().clone();
        assert_eq!(
            kernel.viewer(),
            position.leader.plus(position.trick_plays.len()),
            "the kernel viewer is the seat to move at the root"
        );
        let factors: [HandFactor; HIDDEN_SEATS] = core::array::from_fn(|i| HandFactor {
            seat: kernel.hidden()[i].seat,
            capacity: kernel.hidden()[i].capacity,
            weights: FactorWeights::UniformLawful {
                allowed: kernel.allowed(i),
            },
        });
        FactorBelief {
            root_id: root_identity(root, position),
            kernel,
            position: position.clone(),
            history: Vec::new(),
            field_id: field.id().to_string(),
            factors,
        }
    }

    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn position(&self) -> &RootPosition {
        &self.position
    }

    pub fn history(&self) -> &[Domino] {
        &self.history
    }

    pub fn field_id(&self) -> &str {
        &self.field_id
    }

    pub fn factors(&self) -> &[HandFactor; HIDDEN_SEATS] {
        &self.factors
    }

    /// The hidden slot index of `seat`, or a panic if the seat is the
    /// viewer or unknown — hidden-slot membership is a derived view of the
    /// kernel, never a stored field.
    fn slot_of(&self, seat: Seat) -> usize {
        assert_ne!(seat, self.kernel.viewer(), "the viewer has no factor");
        self.kernel
            .hidden()
            .iter()
            .position(|h| h.seat == seat)
            .expect("a hidden seat has a slot")
    }

    /// The seat to move at this belief's public state — a derived view of
    /// (root position, history).
    pub fn seat_to_move(&self) -> Seat {
        self.cursor().seat()
    }

    /// Replay the post-root public history over the root frame. Public
    /// data only; the same trick arithmetic as the replay walkers.
    fn cursor(&self) -> PublicCursor {
        let mut cursor = PublicCursor {
            leader: self.position.leader,
            plays: self.position.trick_plays.clone(),
            banked: self.position.banked,
            played_by: [DominoSet::EMPTY; Seat::COUNT],
        };
        for d in &self.history {
            cursor.play(&self.position, *d);
        }
        cursor
    }

    /// Theorem 23.1, focal case: the viewer plays `action`. The public
    /// history advances; NO factor changes. The action must be legal for
    /// the viewer's remaining hand at this state.
    pub fn focal_play(&self, action: Domino) -> FactorBelief {
        let cursor = self.cursor();
        let viewer = self.kernel.viewer();
        assert_eq!(cursor.seat(), viewer, "a focal play is the viewer's");
        let remaining = self
            .kernel
            .viewer_hand()
            .difference(cursor.played_by[viewer.index()]);
        let led = cursor
            .plays
            .first()
            .map(|d| self.position.decl.led_context(*d));
        let legal = legal_plays(self.position.decl, remaining, led);
        assert!(legal.contains(action), "a focal play is legal");
        let mut out = self.clone();
        out.history.push(action);
        out
    }
}

/// The walked public state: the same evolution as the replay walkers'
/// `Exec`, restricted to public data. `played_by` attributes each
/// POST-ROOT play to its seat (root factors already exclude pre-root
/// plays).
struct PublicCursor {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played_by: [DominoSet; Seat::COUNT],
}

impl PublicCursor {
    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.played_by[seat.index()].insert(tile),
            "a tile is played once"
        );
        self.plays.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn record<'a>(&'a self, position: &'a RootPosition, history: &'a [Domino]) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history,
        }
    }
}

// ---------------------------------------------------------------------------
// The exact-cover contraction interface (parent §24).
// ---------------------------------------------------------------------------

/// One abstract counting authority (parent §24). Every returned mass is an
/// exact integer derived from one canonical factor state; a backend may be
/// swapped only under exact extensional parity gates (CBS-O13 shape).
pub trait ExactCoverOracle {
    /// `Z_h` — the exact-cover partition function of the factor state.
    fn mass(&self, belief: &FactorBelief) -> u128;

    /// Per root hand `A` of `seat`: `φ_{s,h}(A) · C_{-s,h}(U \ A)`, the
    /// exact compatible-completion weight (parent §21). Nonzero entries
    /// only, in the factor's deterministic support order.
    fn actor_completion_weights(&self, belief: &FactorBelief, seat: Seat)
        -> Vec<(DominoSet, u128)>;

    /// `{t ↦ Z_ht}` for the hidden seat to move under `field` — the
    /// one-ply branch-mass target (parent §21, boxed). Sorted by tile
    /// index, nonzero masses only. The mass-conservation gate
    /// `Z_h = Σ_t Z_ht` (§46) is asserted inside every call.
    fn branch_masses(&self, belief: &FactorBelief, field: &dyn SlicePolicy) -> Vec<(Domino, u128)>;

    /// The posterior update: the hidden seat to move is observed playing
    /// `action`. ONLY that seat's factor is multiplied by the action
    /// likelihood (Theorem 20.1 — closure is the theorem); the history
    /// advances.
    fn condition(
        &self,
        belief: &FactorBelief,
        action: Domino,
        field: &dyn SlicePolicy,
    ) -> FactorBelief;

    /// Exact marginal mass of a root-hand predicate for one seat:
    /// `Σ_{A: pred(A)} φ_{s,h}(A) · C_{-s,h}(U \ A)`.
    fn marginal(
        &self,
        belief: &FactorBelief,
        seat: Seat,
        predicate: &dyn Fn(DominoSet) -> bool,
    ) -> u128;
}

// ---------------------------------------------------------------------------
// Backend zero: the shipped FiberDp, wrapped (§25.1, CBS-A6).
// ---------------------------------------------------------------------------

/// Backend zero. The all-uniform mass IS the shipped tile-pattern
/// capacity DP; a one-table mass contracts the table against exact
/// two-seat completions. No new counting mathematics (CBS-A6).
pub struct FiberOracle;

impl FiberOracle {
    /// Enforce the declared C0 domain and return the table slot if any:
    /// every factor is the kernel's own legality predicate, except at most
    /// one explicit table.
    fn c0_domain(&self, belief: &FactorBelief) -> Option<usize> {
        let mut table: Option<usize> = None;
        for (i, f) in belief.factors.iter().enumerate() {
            assert_eq!(
                f.seat,
                belief.kernel.hidden()[i].seat,
                "slot order is the kernel's"
            );
            assert_eq!(
                f.capacity,
                belief.kernel.hidden()[i].capacity,
                "factors range over root hands"
            );
            match &f.weights {
                FactorWeights::UniformLawful { allowed } => assert_eq!(
                    *allowed,
                    belief.kernel.allowed(i),
                    "a C0 uniform factor is the kernel's own legality predicate"
                ),
                FactorWeights::Table(_) => {
                    assert!(
                        table.is_none(),
                        "the stage C0 backend contracts at most one conditioned factor; \
                         the recursive contraction is Slice D (§47)"
                    );
                    table = Some(i);
                }
            }
        }
        table
    }

    /// The completion weight of the two slots other than `slot` over
    /// `pool` (the root pool minus the proposed hand), each restricted to
    /// its own legality predicate.
    fn pair_completions(&self, belief: &FactorBelief, slot: usize, pool: DominoSet) -> u128 {
        let others: Vec<usize> = (0..HIDDEN_SEATS).filter(|i| *i != slot).collect();
        let (j, k) = (others[0], others[1]);
        pair_count(
            pool,
            belief.kernel.allowed(j),
            belief.factors[j].capacity,
            belief.kernel.allowed(k),
            belief.factors[k].capacity,
        )
    }
}

/// The per-hand walk shared by classification and conditioning, on both
/// backends: the acting seat's remaining hand, its legal set, and the
/// field's chosen tile at this belief's public state. Field purity holds
/// by construction — the record is public data and `remaining` is the
/// seat's own hand.
fn field_action(
    belief: &FactorBelief,
    cursor: &PublicCursor,
    root_hand: DominoSet,
    field: &dyn SlicePolicy,
) -> Domino {
    let seat = cursor.seat();
    let played = cursor.played_by[seat.index()];
    assert!(
        played.is_subset_of(root_hand),
        "a factor's support contains the tiles its seat has played"
    );
    let remaining = root_hand.difference(played);
    let led = cursor
        .plays
        .first()
        .map(|d| belief.position.decl.led_context(*d));
    let legal = legal_plays(belief.position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let record = cursor.record(&belief.position, &belief.history);
    let tile = field.choose(belief.position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a field chooses a legal tile");
    tile
}

/// The one-ply branch-mass route (§21, boxed), shared by both backends:
/// bucket the acting hidden seat's completion weights by the field's
/// chosen tile, and assert the §46 conservation gate against the
/// backend's own mass.
fn branch_masses_via(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
) -> Vec<(Domino, u128)> {
    assert_eq!(
        field.id(),
        belief.field_id,
        "one field identity governs a belief's conditionings (§43)"
    );
    let cursor = belief.cursor();
    let seat = cursor.seat();
    assert_ne!(
        seat,
        belief.kernel.viewer(),
        "branch masses are a hidden seat's"
    );
    let mut masses: Vec<(Domino, u128)> = Vec::new();
    for (hand, weight) in oracle.actor_completion_weights(belief, seat) {
        let tile = field_action(belief, &cursor, hand, field);
        match masses.iter_mut().find(|(t, _)| *t == tile) {
            Some((_, m)) => *m = m.checked_add(weight).expect("an exact mass fits u128"),
            None => masses.push((tile, weight)),
        }
    }
    masses.sort_by_key(|(t, _)| t.index());
    // The §46 gate, at every contraction: the branch masses partition
    // the belief mass exactly.
    let total: u128 = masses.iter().fold(0u128, |acc, (_, m)| {
        acc.checked_add(*m).expect("an exact mass fits u128")
    });
    assert_eq!(
        total,
        oracle.mass(belief),
        "mass conservation: Z_h = Σ_t Z_ht"
    );
    masses
}

/// The Theorem 20.1 posterior route, shared by both backends: multiply
/// ONLY the acting seat's factor by the (deterministic) action likelihood
/// — the 0/1 indicator that the field plays `action` from this root hand
/// at this public state — and advance the history.
///
/// The support walk is restricted to hands CONSISTENT WITH THE PUBLIC
/// RECORD: the seat's own post-root plays are in the hand, and no other
/// seat's post-root play is. A hand failing either test has completion
/// weight zero in EVERY contraction — a conditioned seat's table hands
/// each contain that seat's own plays (the field chose them from the
/// hand), so a candidate holding another seat's played tile overlaps
/// every cover — and its action likelihood at this state is not even
/// defined (its information state contradicts the record), so it is
/// DROPPED, never classified. Zero-weight entries are never stored, so
/// this loses nothing. At one conditioning after a focal play the filter
/// is a no-op — the acting seat has played nothing and the only post-root
/// play is the viewer's, outside the pool — which is why stage C1's
/// conditioning-support law (the whole support classifies, once) is
/// unchanged; beyond one ply the filter is what makes the recursion's
/// deep conditionings lawful.
fn condition_via(belief: &FactorBelief, action: Domino, field: &dyn SlicePolicy) -> FactorBelief {
    assert_eq!(
        field.id(),
        belief.field_id,
        "one field identity governs a belief's conditionings (§43)"
    );
    let cursor = belief.cursor();
    let seat = cursor.seat();
    assert_ne!(
        seat,
        belief.kernel.viewer(),
        "conditioning is on a hidden action"
    );
    let slot = belief.slot_of(seat);
    let own = cursor.played_by[seat.index()];
    let others = (0..Seat::COUNT)
        .filter(|i| *i != seat.index())
        .fold(DominoSet::EMPTY, |acc, i| acc.union(cursor.played_by[i]));
    let kept: Vec<(DominoSet, u128)> = belief.factors[slot]
        .support()
        .into_iter()
        .filter(|(hand, _)| own.is_subset_of(*hand) && hand.is_disjoint(others))
        .filter(|(hand, _)| field_action(belief, &cursor, *hand, field) == action)
        .collect();
    assert!(
        !kept.is_empty(),
        "an observed action has positive mass under the belief"
    );
    let mut out = belief.clone();
    out.factors[slot].weights = FactorWeights::Table(kept);
    out.history.push(action);
    out
}

/// The exact marginal route, shared by both backends: filter the acting
/// seat's completion weights by the predicate and sum.
fn marginal_via(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    seat: Seat,
    predicate: &dyn Fn(DominoSet) -> bool,
) -> u128 {
    oracle
        .actor_completion_weights(belief, seat)
        .into_iter()
        .filter(|(hand, _)| predicate(*hand))
        .fold(0u128, |acc, (_, w)| {
            acc.checked_add(w).expect("an exact mass fits u128")
        })
}

/// Exact count of ways to split `pool` between two slots with 0/1 legality
/// sets `a0`/`a1` and capacities `c0`/`c1`, covering `pool` exactly. A
/// tile allowed at one slot only is forced there; the free choice is the
/// both-allowed group, so the count is one binomial.
fn pair_count(pool: DominoSet, a0: DominoSet, c0: usize, a1: DominoSet, c1: usize) -> u128 {
    let a0 = a0.intersection(pool);
    let a1 = a1.intersection(pool);
    if !pool.difference(a0.union(a1)).is_empty() {
        return 0;
    }
    let both = a0.intersection(a1);
    let only0 = a0.difference(both);
    let only1 = a1.difference(both);
    if only0.len() > c0 || only1.len() > c1 {
        return 0;
    }
    let to0 = c0 - only0.len();
    let to1 = c1 - only1.len();
    if both.len() != to0 + to1 {
        return 0;
    }
    binomial(both.len(), to0)
}

impl ExactCoverOracle for FiberOracle {
    fn mass(&self, belief: &FactorBelief) -> u128 {
        match self.c0_domain(belief) {
            // The uniform-root special case IS the shipped counting DP.
            None => FiberDp::new(&belief.kernel).count(),
            Some(slot) => {
                let pool = belief.kernel.pool();
                let mut total: u128 = 0;
                for (hand, weight) in belief.factors[slot].support() {
                    let completions = self.pair_completions(belief, slot, pool.difference(hand));
                    total = total
                        .checked_add(
                            weight
                                .checked_mul(completions)
                                .expect("an exact mass fits u128"),
                        )
                        .expect("an exact mass fits u128");
                }
                total
            }
        }
    }

    fn actor_completion_weights(
        &self,
        belief: &FactorBelief,
        seat: Seat,
    ) -> Vec<(DominoSet, u128)> {
        let table = self.c0_domain(belief);
        let slot = belief.slot_of(seat);
        assert!(
            table.is_none() || table == Some(slot),
            "a C0 completion query never spans a conditioned factor; \
             the recursive contraction is Slice D (§47)"
        );
        let pool = belief.kernel.pool();
        let mut out = Vec::new();
        for (hand, weight) in belief.factors[slot].support() {
            let completions = self.pair_completions(belief, slot, pool.difference(hand));
            let w = weight
                .checked_mul(completions)
                .expect("an exact mass fits u128");
            if w != 0 {
                out.push((hand, w));
            }
        }
        out
    }

    fn branch_masses(&self, belief: &FactorBelief, field: &dyn SlicePolicy) -> Vec<(Domino, u128)> {
        branch_masses_via(self, belief, field)
    }

    fn condition(
        &self,
        belief: &FactorBelief,
        action: Domino,
        field: &dyn SlicePolicy,
    ) -> FactorBelief {
        condition_via(belief, action, field)
    }

    fn marginal(
        &self,
        belief: &FactorBelief,
        seat: Seat,
        predicate: &dyn Fn(DominoSet) -> bool,
    ) -> u128 {
        marginal_via(self, belief, seat, predicate)
    }
}

// ---------------------------------------------------------------------------
// The §23 factorized fixed-policy recursion (Slice D, §47).
// ---------------------------------------------------------------------------

/// Exact integer counters of one recursion: how the walk's nodes divided
/// between the three §23 cases, and how many conditionings (posterior
/// updates) the hidden branches performed.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RecursionStats {
    /// Nodes settled by the decided cutoff before the terminal depth.
    pub decided_early: u64,
    /// Nodes settled at the terminal depth itself.
    pub decided_terminal: u64,
    /// Focal (§23 focal-case) nodes walked.
    pub focal_nodes: u64,
    /// Hidden (§23 field-case) nodes walked.
    pub hidden_nodes: u64,
    /// Posterior updates performed — one per hidden branch taken.
    pub conditionings: u64,
}

/// The §47 evaluation: one frozen focal policy under the belief's declared
/// deterministic field, by the §23 recursion cleared of denominators (see
/// the module doc). Returns the viewer-objective success mass `M(B)`; the
/// §23 value is the exact pair `M(B) / oracle.mass(belief)`. The oracle
/// must serve every belief the recursion reaches — beyond one conditioning
/// that is [`SupportOracle`]'s domain, not backend zero's.
pub fn viewer_success_mass(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
    stats: &mut RecursionStats,
) -> u128 {
    let cursor = belief.cursor();
    let viewer = belief.kernel.viewer();
    // Post-root plays to terminal: the viewer's hand plus every hidden
    // capacity (the same derived view the bundled walker uses).
    let total = belief.kernel.viewer_hand().len()
        + belief
            .kernel
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let at_terminal = belief.history.len() == total;
    if let Some(u) = decided_success(&belief.position, viewer, cursor.banked, at_terminal) {
        // §23 terminal/decided case: the indicator is a constant of every
        // continuation of every represented world (decidedness is
        // monotone in banked points), so M = u·Z.
        if at_terminal {
            stats.decided_terminal += 1;
        } else {
            stats.decided_early += 1;
        }
        return if u { oracle.mass(belief) } else { 0 };
    }
    assert!(
        belief.history.len() < total,
        "the 42-point pool exhausts at terminal, so an undecided state has plays left"
    );
    if cursor.seat() == viewer {
        // §23 focal case: every represented world shares this public
        // history and the constant focal hand, hence ONE information
        // state — the frozen policy's single choice serves them all, and
        // no factor changes (Theorem 23.1, focal case).
        stats.focal_nodes += 1;
        let remaining = belief
            .kernel
            .viewer_hand()
            .difference(cursor.played_by[viewer.index()]);
        let led = cursor
            .plays
            .first()
            .map(|d| belief.position.decl.led_context(*d));
        let legal = legal_plays(belief.position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = cursor.record(&belief.position, &belief.history);
        let tile = focal.choose(belief.position.decl, remaining, legal, &record);
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        viewer_success_mass(oracle, &belief.focal_play(tile), focal, field, stats)
    } else {
        // §23 field case, cleared of denominators: the branch masses
        // partition Z exactly (asserted inside the contraction), so
        // M = Σ_t M(B·t) with each child conditioned by Theorem 20.1.
        stats.hidden_nodes += 1;
        let mut mass: u128 = 0;
        for (tile, _) in oracle.branch_masses(belief, field) {
            stats.conditionings += 1;
            let child = oracle.condition(belief, tile, field);
            let m = viewer_success_mass(oracle, &child, focal, field, stats);
            mass = mass.checked_add(m).expect("an exact mass fits u128");
        }
        mass
    }
}

// ---------------------------------------------------------------------------
// The anytime proof-state §18 fixed-policy score profile (Phase 2).
// ---------------------------------------------------------------------------

/// The exact unnormalized score profile `H_ρ(B; ·)` of one frozen focal
/// policy under the declared field (anytime proof-state parent §2/§18):
/// bin `s` holds the exact integer world mass whose complete continuation
/// banks exactly `s` points for the DECLARING team. Viewer-independent —
/// the viewer parity enters only at projection ([`ScoreProfile::tail`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreProfile {
    /// Exact world mass per declaring-team final score, indices `0..=42`.
    pub bins: [u128; 43],
}

impl ScoreProfile {
    /// The all-zero profile (the additive identity of branch addition).
    pub fn zero() -> ScoreProfile {
        ScoreProfile { bins: [0; 43] }
    }

    /// Total represented mass `Σ_s H(s)` — equals `Z(B)` by conservation
    /// (a gate, not an assumption).
    pub fn total(&self) -> u128 {
        self.bins
            .iter()
            .try_fold(0u128, |a, b| a.checked_add(*b))
            .expect("an exact mass fits u128")
    }

    /// The tail mass `T(k) = Σ_{s ≥ k} H(s)` — the declaring team's
    /// success mass at contract `k`; the setting side's is `Z − T(k)`.
    /// `k > 42` gives 0 and `k = 0` gives the total, both meaningful.
    pub fn tail(&self, k: u32) -> u128 {
        self.bins
            .iter()
            .enumerate()
            .filter(|(s, _)| *s as u32 >= k)
            .try_fold(0u128, |a, (_, b)| a.checked_add(*b))
            .expect("an exact mass fits u128")
    }

    /// The exact point-mass sum `Σ_s s·H(s)` — the §3 tail-sum identity's
    /// left side (`= Σ_{k=1}^{42} T(k)`, a gate), so the expected score
    /// is the exact pair `point_mass_sum / total`.
    pub fn point_mass_sum(&self) -> u128 {
        self.bins
            .iter()
            .enumerate()
            .try_fold(0u128, |a, (s, b)| {
                a.checked_add(b.checked_mul(s as u128).expect("fits"))
            })
            .expect("an exact point-mass sum fits u128")
    }

    fn add_assign(&mut self, other: &ScoreProfile) {
        for (a, b) in self.bins.iter_mut().zip(other.bins.iter()) {
            *a = a.checked_add(*b).expect("an exact mass fits u128");
        }
    }
}

/// The §18 evaluation: the exact 43-bin score profile of this belief
/// state under ONE frozen focal policy and the declared field. Same walk
/// as [`viewer_success_mass`] with two deliberate differences: the
/// terminal case bins the declaring team's banked total instead of
/// testing it against the bid, and there is NO early decided cutoff — a
/// monotone decided state knows the make indicator, not the final score,
/// so the recursion continues to true terminals (§18's caveat; the
/// module doc's SCORE PROFILE section). `stats.decided_early` therefore
/// stays 0 (gated) and `stats.decided_terminal` counts terminal leaves.
pub fn viewer_score_profile(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
    stats: &mut RecursionStats,
) -> ScoreProfile {
    let cursor = belief.cursor();
    let viewer = belief.kernel.viewer();
    let total = belief.kernel.viewer_hand().len()
        + belief
            .kernel
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    if belief.history.len() == total {
        // §18 terminal case: every play is public, so the declaring
        // team's banked total is a constant of the represented worlds
        // and the whole pool is banked (asserted).
        stats.decided_terminal += 1;
        let banked = cursor.banked;
        assert_eq!(
            banked[0] + banked[1],
            42,
            "the 42-point pool is fully banked at terminal"
        );
        let score = banked[belief.position.declaring_team.index()];
        let mut profile = ScoreProfile::zero();
        profile.bins[score as usize] = oracle.mass(belief);
        return profile;
    }
    if cursor.seat() == viewer {
        // §18 focal case: one information state, one frozen choice, no
        // factor changes (Theorem 23.1, focal case — as in Slice D).
        stats.focal_nodes += 1;
        let remaining = belief
            .kernel
            .viewer_hand()
            .difference(cursor.played_by[viewer.index()]);
        let led = cursor
            .plays
            .first()
            .map(|d| belief.position.decl.led_context(*d));
        let legal = legal_plays(belief.position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = cursor.record(&belief.position, &belief.history);
        let tile = focal.choose(belief.position.decl, remaining, legal, &record);
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        viewer_score_profile(oracle, &belief.focal_play(tile), focal, field, stats)
    } else {
        // §18 branch addition: the branch masses partition Z exactly, so
        // profiles add binwise — mass conservation follows binwise too.
        stats.hidden_nodes += 1;
        let mut profile = ScoreProfile::zero();
        for (tile, _) in oracle.branch_masses(belief, field) {
            stats.conditionings += 1;
            let child = oracle.condition(belief, tile, field);
            let h = viewer_score_profile(oracle, &child, focal, field, stats);
            profile.add_assign(&h);
        }
        profile
    }
}

// ---------------------------------------------------------------------------
// The §48 factorized grammar best response (Slice E).
// ---------------------------------------------------------------------------

/// Exact integer counters of one grammar recursion — the [`RecursionStats`]
/// shape plus how much room the max explored.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ResponseStats {
    /// Nodes settled by the decided cutoff before the terminal depth.
    pub decided_early: u64,
    /// Nodes settled at the terminal depth itself.
    pub decided_terminal: u64,
    /// Focal (grammar-max) nodes walked.
    pub focal_nodes: u64,
    /// `Σ |G(I)|` over the walked focal nodes — the branches the max
    /// actually explored (the recursion's grammar-branching census).
    pub focal_actions: u64,
    /// Hidden (§23 field-case) nodes walked.
    pub hidden_nodes: u64,
    /// Posterior updates performed — one per hidden branch taken.
    pub conditionings: u64,
}

/// The §48 evaluation: the exact grammar optimum `Q^G` of this belief
/// state as a success MASS — `max_{ρ ∈ Π^G} M_ρ(B)`, with the §12 value
/// the exact pair `M^G(B) / oracle.mass(belief)`. Same recursion as
/// [`viewer_success_mass`] except the focal case maximizes over `G(I)`
/// (see the module doc for why the max is lawful and why nodewise max
/// equals the policy-class max). Maximization is over GRAMMAR actions
/// only — the §48 fence. The full action set's one entry point is
/// [`response_success_mass`], the §36 EscalateExact endpoint Slice G
/// enabled after this slice landed (§48's own sequencing).
///
/// Per-root-action values need no separate producer: for a grammar root
/// action `a`, `Q^G_a` is this function on [`FactorBelief::focal_play`]
/// `(a)`, and the root call is their max (asserted by the Slice E gates
/// against the Slice B enumeration split).
pub fn grammar_success_mass(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    grammar: &PolicyGrammar<'_>,
    field: &dyn SlicePolicy,
    stats: &mut ResponseStats,
) -> u128 {
    let cursor = belief.cursor();
    let viewer = belief.kernel.viewer();
    let total = belief.kernel.viewer_hand().len()
        + belief
            .kernel
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let at_terminal = belief.history.len() == total;
    if let Some(u) = decided_success(&belief.position, viewer, cursor.banked, at_terminal) {
        // Decided: the indicator is a constant of every continuation of
        // every represented world, so every policy class attains u·Z —
        // the same decided quotient the Slice B walk truncates under.
        if at_terminal {
            stats.decided_terminal += 1;
        } else {
            stats.decided_early += 1;
        }
        return if u { oracle.mass(belief) } else { 0 };
    }
    assert!(
        belief.history.len() < total,
        "the 42-point pool exhausts at terminal, so an undecided state has plays left"
    );
    if cursor.seat() == viewer {
        // §48 focal case: max over the grammar's actions at this ONE
        // information state. Every child shares Z(B) — a focal play
        // changes no factor — so the max of masses is the max of values.
        stats.focal_nodes += 1;
        let remaining = belief
            .kernel
            .viewer_hand()
            .difference(cursor.played_by[viewer.index()]);
        let led = cursor
            .plays
            .first()
            .map(|d| belief.position.decl.led_context(*d));
        let legal = legal_plays(belief.position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = cursor.record(&belief.position, &belief.history);
        let actions = grammar.actions(belief.position.decl, remaining, legal, &record);
        let mut best: Option<u128> = None;
        for tile in actions.iter() {
            stats.focal_actions += 1;
            let m = grammar_success_mass(oracle, &belief.focal_play(tile), grammar, field, stats);
            best = Some(best.map_or(m, |b| b.max(m)));
        }
        best.expect("a grammar holds an action at every focal state (§11)")
    } else {
        // §23 field case, unchanged from the fixed-policy recursion: the
        // hidden seat is not the optimizer, and the branch-wise optimum
        // distributes through the sum because different branches'
        // continuations are chosen at disjoint information states.
        stats.hidden_nodes += 1;
        let mut mass: u128 = 0;
        for (tile, _) in oracle.branch_masses(belief, field) {
            stats.conditionings += 1;
            let child = oracle.condition(belief, tile, field);
            let m = grammar_success_mass(oracle, &child, grammar, field, stats);
            mass = mass.checked_add(m).expect("an exact mass fits u128");
        }
        mass
    }
}

// ---------------------------------------------------------------------------
// The §36 EscalateExact endpoint (Slice G): the full-action-set response.
// ---------------------------------------------------------------------------

/// The exact unrestricted best-response success mass — the §48 recursion
/// with the focal max over the FULL legal action set. `M*(B) / Z(B)` is
/// the exact viewer-objective value `Q(B)`; on a root child
/// [`FactorBelief::focal_play`]`(a)` it is the exact `Q_a`. This is the
/// §36 EscalateExact endpoint and the §38 enumeration-fallback authority
/// of the Slice G controller: it collapses a root-action interval to a
/// point, at full-recursion cost. §48 sequenced this deliberately —
/// grammar first (Slice E), then the full action set (Slice G); the
/// Slice G gates hold this recursion to extensional parity with the
/// bundled exact authority (`exposure::exact_root_value`) at every gated
/// root, the C→G cross-representation capstone.
///
/// The max is lawful for the same reason as the grammar max: every
/// represented world shares the focal information state, so the nodewise
/// max over legal actions equals the max over all lawful full policies
/// (§12/§48 with `G(I)` = the legal set — the grammar argument never
/// used properness of the restriction).
pub fn response_success_mass(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
    stats: &mut ResponseStats,
) -> u128 {
    let cursor = belief.cursor();
    let viewer = belief.kernel.viewer();
    let total = belief.kernel.viewer_hand().len()
        + belief
            .kernel
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let at_terminal = belief.history.len() == total;
    if let Some(u) = decided_success(&belief.position, viewer, cursor.banked, at_terminal) {
        // Decided: constant indicator over every continuation of every
        // represented world — every policy class attains u·Z.
        if at_terminal {
            stats.decided_terminal += 1;
        } else {
            stats.decided_early += 1;
        }
        return if u { oracle.mass(belief) } else { 0 };
    }
    assert!(
        belief.history.len() < total,
        "the 42-point pool exhausts at terminal, so an undecided state has plays left"
    );
    if cursor.seat() == viewer {
        // Focal case: max over EVERY legal action at this one
        // information state. All children share Z(B), so the max of
        // masses is the max of values.
        stats.focal_nodes += 1;
        let remaining = belief
            .kernel
            .viewer_hand()
            .difference(cursor.played_by[viewer.index()]);
        let led = cursor
            .plays
            .first()
            .map(|d| belief.position.decl.led_context(*d));
        let legal = legal_plays(belief.position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let mut best: Option<u128> = None;
        for tile in legal.iter() {
            stats.focal_actions += 1;
            let m = response_success_mass(oracle, &belief.focal_play(tile), field, stats);
            best = Some(best.map_or(m, |b| b.max(m)));
        }
        best.expect("a legal set holds an action")
    } else {
        // §23 field case, unchanged: the hidden seat is not the
        // optimizer; the branch-wise optimum distributes through the sum
        // because different branches' continuations are chosen at
        // disjoint information states.
        stats.hidden_nodes += 1;
        let mut mass: u128 = 0;
        for (tile, _) in oracle.branch_masses(belief, field) {
            stats.conditionings += 1;
            let child = oracle.condition(belief, tile, field);
            let m = response_success_mass(oracle, &child, field, stats);
            mass = mass.checked_add(m).expect("an exact mass fits u128");
        }
        mass
    }
}

// ---------------------------------------------------------------------------
// The Slice D backend: the general support contraction (§47, §25.2/§25.4).
// ---------------------------------------------------------------------------

/// The Slice D backend. Contracts any mix of uniform and table factors by
/// walking explicit supports: the acting seat's loop is §25.2 unchanged,
/// and the completion count behind each hand generalizes `pair_count`'s
/// one-binomial special case to a walk of the conditioned support
/// (§25.4's sparse shape). On the C0 domain it is extensionally equal to
/// backend zero (gated); beyond it, it is gated against surviving-world
/// enumeration. No new counting mathematics — every count is a sum of
/// products of factor weights over exact covers.
pub struct SupportOracle;

impl SupportOracle {
    /// The exact completion count of the two slots other than `slot` over
    /// `pool`: `C_{-s,h}(pool) = Σ_{B} w_j(B) · w_k(pool \ B)`. When both
    /// remaining factors are uniform this IS `pair_count` (the C0 parity
    /// anchor); when a table is present, its explicit support is walked
    /// and the complement is weighed in the other factor.
    fn completions(&self, belief: &FactorBelief, slot: usize, pool: DominoSet) -> u128 {
        let others: Vec<usize> = (0..HIDDEN_SEATS).filter(|i| *i != slot).collect();
        let (j, k) = (others[0], others[1]);
        let table_j = matches!(belief.factors[j].weights, FactorWeights::Table(_));
        let table_k = matches!(belief.factors[k].weights, FactorWeights::Table(_));
        match (table_j, table_k) {
            (false, false) => pair_count(
                pool,
                belief.kernel.allowed(j),
                belief.factors[j].capacity,
                belief.kernel.allowed(k),
                belief.factors[k].capacity,
            ),
            // Walk one explicit support; the complement is weighed in the
            // other factor. Iterating the table (never the uniform side)
            // keeps the walk proportional to the SPARSE support.
            (true, _) => self.split_sum(belief, j, k, pool),
            (false, true) => self.split_sum(belief, k, j, pool),
        }
    }

    /// `Σ_{(B, w) ∈ support(walked), B ⊆ pool} w · weight_of(other, pool \ B)`.
    fn split_sum(
        &self,
        belief: &FactorBelief,
        walked: usize,
        other: usize,
        pool: DominoSet,
    ) -> u128 {
        let mut total: u128 = 0;
        for (hand, weight) in belief.factors[walked].support() {
            if !hand.is_subset_of(pool) {
                continue;
            }
            let rest = pool.difference(hand);
            let w = weight
                .checked_mul(self.weight_of(belief, other, rest))
                .expect("an exact mass fits u128");
            total = total.checked_add(w).expect("an exact mass fits u128");
        }
        total
    }

    /// One factor's exact weight on one specific root hand: the 0/1
    /// capacity-and-legality indicator for a uniform factor, the stored
    /// entry (zero when absent) for a table.
    fn weight_of(&self, belief: &FactorBelief, slot: usize, hand: DominoSet) -> u128 {
        match &belief.factors[slot].weights {
            FactorWeights::UniformLawful { allowed } => u128::from(
                hand.len() == belief.factors[slot].capacity && hand.is_subset_of(*allowed),
            ),
            FactorWeights::Table(entries) => entries
                .iter()
                .find(|(h, _)| *h == hand)
                .map(|(_, w)| *w)
                .unwrap_or(0),
        }
    }
}

impl ExactCoverOracle for SupportOracle {
    fn mass(&self, belief: &FactorBelief) -> u128 {
        let all_uniform = belief
            .factors
            .iter()
            .all(|f| matches!(f.weights, FactorWeights::UniformLawful { .. }));
        if all_uniform {
            // The uniform-root special case IS the shipped counting DP —
            // the same fast path as backend zero.
            return FiberDp::new(&belief.kernel).count();
        }
        let slot = belief
            .factors
            .iter()
            .position(|f| matches!(f.weights, FactorWeights::Table(_)))
            .expect("a non-uniform belief holds a table");
        let pool = belief.kernel.pool();
        let mut total: u128 = 0;
        for (hand, weight) in belief.factors[slot].support() {
            let completions = self.completions(belief, slot, pool.difference(hand));
            total = total
                .checked_add(
                    weight
                        .checked_mul(completions)
                        .expect("an exact mass fits u128"),
                )
                .expect("an exact mass fits u128");
        }
        total
    }

    fn actor_completion_weights(
        &self,
        belief: &FactorBelief,
        seat: Seat,
    ) -> Vec<(DominoSet, u128)> {
        let slot = belief.slot_of(seat);
        let pool = belief.kernel.pool();
        let mut out = Vec::new();
        for (hand, weight) in belief.factors[slot].support() {
            let completions = self.completions(belief, slot, pool.difference(hand));
            let w = weight
                .checked_mul(completions)
                .expect("an exact mass fits u128");
            if w != 0 {
                out.push((hand, w));
            }
        }
        out
    }

    fn branch_masses(&self, belief: &FactorBelief, field: &dyn SlicePolicy) -> Vec<(Domino, u128)> {
        branch_masses_via(self, belief, field)
    }

    fn condition(
        &self,
        belief: &FactorBelief,
        action: Domino,
        field: &dyn SlicePolicy,
    ) -> FactorBelief {
        condition_via(belief, action, field)
    }

    fn marginal(
        &self,
        belief: &FactorBelief,
        seat: Seat,
        predicate: &dyn Fn(DominoSet) -> bool,
    ) -> u128 {
        marginal_via(self, belief, seat, predicate)
    }
}

// ---------------------------------------------------------------------------
// The §49 consequence CEGAR (Slice F, §27–31).
// ---------------------------------------------------------------------------

/// §28's feature map `κ` evaluated on one remaining hand at one public
/// state, with §49's starting vocabulary plus the §31 critical-set
/// coordinate. Two hands share a class exactly when their signatures are
/// equal; the derived `Ord` gives the partition a deterministic order.
/// The names do not make a class exact (§28) — exactness is decided by
/// the verifier loop below, never by the vocabulary.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ClassSignature {
    /// `remaining ∩ critical`, as the set's bit representation — the §31
    /// coordinate that CEGAR grows. Empty until a witness demands a tile.
    critical_bits: u32,
    /// Remaining called (trump) tiles.
    trump_count: u8,
    /// Highest declaration-relative rank among remaining called tiles.
    highest_trump: Option<u8>,
    /// Remaining tiles that follow the led effective context; 0 leading.
    led_count: u8,
    /// Count pips held (the sum of `count()` labels over the remaining
    /// hand — count-tile possession as one exact coordinate).
    count_pips: u8,
    /// Some legal tile strictly beats the current trick's best key
    /// (current-winner possibility; false when leading — no winner yet).
    can_beat: bool,
    /// Void in the led natural context while holding a called tile (ruff
    /// possibility; false when leading or when the called suit was led).
    can_ruff: bool,
}

/// Evaluate `κ` on one remaining hand. Public data plus the seat's own
/// hand only — the same purity boundary as a field read: `trick_plays`
/// are the current trick's plays in actor order (empty when the seat
/// leads).
pub fn class_signature(
    decl: Decl,
    remaining: DominoSet,
    trick_plays: &[Domino],
    critical: DominoSet,
) -> ClassSignature {
    let led = trick_plays.first().map(|d| decl.led_context(*d));
    let legal = legal_plays(decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let trump = remaining.intersection(decl.called_set());
    let (led_count, can_beat, can_ruff) = match led {
        None => (0u8, false, false),
        Some(q) => {
            let followers = remaining.intersection(decl.effective_incidence(q));
            let best = trick_plays
                .iter()
                .map(|d| decl.trick_key(*d, q))
                .max()
                .expect("a led trick has a play");
            let beat = legal.iter().any(|t| decl.trick_key(t, q) > best);
            let ruff =
                followers.is_empty() && !trump.is_empty() && matches!(q, Context::Natural(_));
            (
                u8::try_from(followers.len()).expect("a hand holds at most 7 tiles"),
                beat,
                ruff,
            )
        }
    };
    ClassSignature {
        critical_bits: remaining.intersection(critical).bits(),
        trump_count: u8::try_from(trump.len()).expect("a hand holds at most 7 tiles"),
        highest_trump: trump.iter().map(|d| decl.rank(d).value()).max(),
        led_count,
        count_pips: u8::try_from(remaining.iter().map(|d| d.count()).sum::<u32>())
            .expect("count pips total 35"),
        can_beat,
        can_ruff,
    }
}

/// One §30 refinement, witness attached (§49: every refinement carries
/// one): a class that failed action-uniformity, two of its hands proving
/// the failure, and the discriminator tile — the lowest-index tile the
/// two hands disagree on — that enters the §31 critical set. Hands here
/// are REMAINING hands (the domain `κ` classifies).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RefinementWitness {
    /// The signature of the class that split, under the critical set of
    /// its stage.
    pub signature: ClassSignature,
    pub left_hand: DominoSet,
    pub left_action: Domino,
    pub right_hand: DominoSet,
    pub right_action: Domino,
    /// The tile that separates the pair. Same-class hands agree on every
    /// critical tile, so a differing tile is provably fresh.
    pub discriminator: Domino,
}

/// One abstraction stage's census — §49's measurements at one critical
/// set. `branch_intervals` is the root-interval impact: per branch tile,
/// the exact lower mass `L_t` (action-uniform classes choosing `t`) and
/// the action-bounded upper `U_t` (plus every unresolved class whose
/// observed action set contains `t`). Theorem 30.1 is the gates'
/// monotonicity of exactly these fields across stages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbstractionStage {
    /// The §31 critical tile set this stage partitioned under.
    pub critical: DominoSet,
    /// Classes with positive support mass.
    pub classes: u64,
    /// Classes whose field action is uniform over their support.
    pub exact_classes: u64,
    /// Mass in action-uniform classes.
    pub exact_mass: u128,
    /// Mass in classes still awaiting a refinement — §49's residual
    /// class mass. `exact_mass + residual_mass = Z` at every stage.
    pub residual_mass: u128,
    /// Per branch tile `(t, L_t, U_t)`, sorted by tile index.
    pub branch_intervals: Vec<(Domino, u128, u128)>,
}

/// The full refinement record: every stage from the bare §49 vocabulary
/// (stage 0, empty critical set) to the action-exact endpoint, the
/// witness chain that drove it, and the endpoint's per-branch masses
/// (equal to the exact contraction — gated).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CegarOutcome {
    /// Support hands classified (the instrument's ground-truth bill —
    /// identical to one `branch_masses` pass).
    pub hands: u64,
    pub stages: Vec<AbstractionStage>,
    /// One witness per refinement: `witnesses.len() + 1 == stages.len()`.
    pub witnesses: Vec<RefinementWitness>,
    /// The endpoint's exact per-tile masses.
    pub branch_masses: Vec<(Domino, u128)>,
}

/// The §30 counterexample-guided refinement loop at one hidden node, run
/// to the action-exact endpoint. Ground truth is one field
/// classification per support hand (the same walk, and the same bill, as
/// [`ExactCoverOracle::branch_masses`]); the loop then partitions by
/// [`class_signature`], aggregates uniform classes, and splits the
/// largest-mass non-uniform class by a witnessed discriminator until the
/// residual is zero. Terminates in at most 28 refinements: every
/// refinement adds a fresh tile to the critical set, and under the full
/// tile set classes are singleton remaining hands (§30's completeness).
///
/// This is an INSTRUMENT (§49): it measures how much posterior mass a
/// small exact vocabulary concentrates in action-exact classes and what
/// interval the residual leaves on the branch masses — never a cheaper
/// classifier. Deterministic throughout: support order, `BTreeMap`
/// partition order, strict-improvement class selection, lowest-index
/// discriminator.
pub fn refine_to_action_exact(
    oracle: &dyn ExactCoverOracle,
    belief: &FactorBelief,
    field: &dyn SlicePolicy,
) -> CegarOutcome {
    assert_eq!(
        field.id(),
        belief.field_id,
        "one field identity governs a belief's classifications (§43)"
    );
    let cursor = belief.cursor();
    let seat = cursor.seat();
    assert_ne!(
        seat,
        belief.kernel.viewer(),
        "hand classes are a hidden seat's (§28)"
    );
    let decl = belief.position.decl;
    let played = cursor.played_by[seat.index()];
    let trick_plays = cursor.plays.clone();
    // Ground truth, once per support hand. `κ` classifies remaining
    // hands: the played prefix is constant across the support, so the
    // remaining hand determines the action given the public record.
    let hands: Vec<(DominoSet, u128, Domino)> = oracle
        .actor_completion_weights(belief, seat)
        .into_iter()
        .map(|(hand, weight)| {
            let action = field_action(belief, &cursor, hand, field);
            (hand.difference(played), weight, action)
        })
        .collect();
    assert!(!hands.is_empty(), "a hidden seat to move has support");
    let z = oracle.mass(belief);

    struct ClassAcc {
        mass: u128,
        members: Vec<usize>,
        actions: DominoSet,
    }

    let mut critical = DominoSet::EMPTY;
    let mut stages: Vec<AbstractionStage> = Vec::new();
    let mut witnesses: Vec<RefinementWitness> = Vec::new();
    loop {
        let mut classes: BTreeMap<ClassSignature, ClassAcc> = BTreeMap::new();
        for (i, (remaining, weight, action)) in hands.iter().enumerate() {
            let sig = class_signature(decl, *remaining, &trick_plays, critical);
            let acc = classes.entry(sig).or_insert(ClassAcc {
                mass: 0,
                members: Vec::new(),
                actions: DominoSet::EMPTY,
            });
            acc.mass = acc
                .mass
                .checked_add(*weight)
                .expect("an exact mass fits u128");
            acc.members.push(i);
            acc.actions.insert(*action);
        }
        // The stage census (§49's measurements at this critical set).
        let mut exact_classes: u64 = 0;
        let mut exact_mass: u128 = 0;
        let mut residual_mass: u128 = 0;
        let mut tiles = DominoSet::EMPTY;
        for acc in classes.values() {
            tiles = tiles.union(acc.actions);
            if acc.actions.len() == 1 {
                exact_classes += 1;
                exact_mass = exact_mass
                    .checked_add(acc.mass)
                    .expect("an exact mass fits u128");
            } else {
                residual_mass = residual_mass
                    .checked_add(acc.mass)
                    .expect("an exact mass fits u128");
            }
        }
        assert_eq!(
            exact_mass
                .checked_add(residual_mass)
                .expect("an exact mass fits u128"),
            z,
            "the classes partition the belief mass"
        );
        let branch_intervals: Vec<(Domino, u128, u128)> = tiles
            .iter()
            .map(|t| {
                let mut lower: u128 = 0;
                let mut upper: u128 = 0;
                for acc in classes.values() {
                    if !acc.actions.contains(t) {
                        continue;
                    }
                    upper = upper
                        .checked_add(acc.mass)
                        .expect("an exact mass fits u128");
                    if acc.actions.len() == 1 {
                        lower = lower
                            .checked_add(acc.mass)
                            .expect("an exact mass fits u128");
                    }
                }
                (t, lower, upper)
            })
            .collect();
        stages.push(AbstractionStage {
            critical,
            classes: u64::try_from(classes.len()).expect("a class count fits u64"),
            exact_classes,
            exact_mass,
            residual_mass,
            branch_intervals,
        });
        // §30 steps 3–5 on the largest-mass unresolved class (§33 step
        // 10's decision-impact order, mass as the exact proxy; ties fall
        // to the smallest signature by strict improvement over the
        // BTreeMap's deterministic order).
        let split = classes
            .iter()
            .filter(|(_, acc)| acc.actions.len() > 1)
            .fold(
                None::<(&ClassSignature, &ClassAcc)>,
                |best, (sig, acc)| match best {
                    Some((_, b)) if b.mass >= acc.mass => best,
                    _ => Some((sig, acc)),
                },
            );
        let Some((sig, acc)) = split else {
            break;
        };
        let first = acc.members[0];
        let (left_hand, _, left_action) = hands[first];
        let differing = acc
            .members
            .iter()
            .copied()
            .find(|&j| hands[j].2 != left_action)
            .expect("a non-uniform class holds a differing pair");
        let (right_hand, _, right_action) = hands[differing];
        let disagreement = left_hand
            .difference(right_hand)
            .union(right_hand.difference(left_hand));
        let discriminator = disagreement
            .iter()
            .next()
            .expect("distinct remaining hands differ in a tile");
        // Same class ⇒ equal critical_bits ⇒ agreement on every critical
        // tile, so the discriminator is provably fresh and the loop
        // terminates within the 28-tile alphabet.
        assert!(
            !critical.contains(discriminator),
            "a witnessed discriminator is outside the critical set"
        );
        witnesses.push(RefinementWitness {
            signature: *sig,
            left_hand,
            left_action,
            right_hand,
            right_action,
            discriminator,
        });
        critical.insert(discriminator);
    }
    let last = stages.last().expect("the loop records at least one stage");
    assert_eq!(last.residual_mass, 0, "the endpoint is action-exact");
    let branch_masses: Vec<(Domino, u128)> = last
        .branch_intervals
        .iter()
        .map(|(t, lower, upper)| {
            assert_eq!(lower, upper, "an exact endpoint has point intervals");
            (*t, *lower)
        })
        .collect();
    let total: u128 = branch_masses.iter().fold(0u128, |a, (_, m)| {
        a.checked_add(*m).expect("an exact mass fits u128")
    });
    assert_eq!(total, z, "mass conservation: Z_h = Σ_t Z_ht");
    CegarOutcome {
        hands: u64::try_from(hands.len()).expect("a hand count fits u64"),
        stages,
        witnesses,
        branch_masses,
    }
}
