//! `solver::factor_belief` — the counted-belief Slice C, stage C0: the
//! seat-factored belief, the exact-cover contraction interface, and
//! backend zero (the shipped `FiberDp` wrapped for 0/1 factors).
//!
//! EXPLORATORY tier. Mathematical source:
//! `walt/math/counted_belief_sandwich_v0.1.md` Parts V–VI (§18–26, §43,
//! §46 stage C0), adopted by rulings CBS-A6 and CBS-A9
//! (`walt/CENSUS-RULINGS.md`); design register `walt/FACTOR-BELIEF.md`;
//! intake companion `walt/math/counted_belief_sandwich_v0.1_intake.md`.
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
//! DECLARED C0 DOMAIN. Backend zero serves exactly the uniform-root
//! special case and its one-ply conditionings: every factor is either the
//! kernel's own 0/1 legality predicate or ONE explicit table produced by
//! [`ExactCoverOracle::condition`]. A contraction across two or more
//! table factors is the recursive machinery of Slice D (§47) and is
//! REFUSED here, not approximated. Fields are deterministic
//! [`SlicePolicy`] reads (the trait's documented contract); a stochastic
//! field needs an explicit tape factor (CBS-A6's boundary obligation) and
//! has no entry point in this module.
//!
//! Deviations from the `walt/FACTOR-BELIEF.md` trait sketch, under L2-A3's
//! naming latitude, recorded here and in the register:
//! - `branch_masses`/`condition` take no seat argument — the acting seat
//!   is the derived view `seat_to_move()`, and passing it would store one
//!   authority twice;
//! - masses are `u128` with checked arithmetic (the kernel's own counting
//!   width; the trick-1 partition function is < 2^64), not `BigUint`;
//! - `count_cell` is deferred to the slice that gives Part IV cell
//!   predicates a concrete type (Slice F); `marginal` covers the one-seat
//!   case exactly.
//!
//! No belief cache exists at C0. When one arrives, its key must be the
//! FULL §43 identity list carried by [`FactorBelief`] — a hit under an
//! omitted coordinate is the PiKey defect reborn (CBS-A6).

use crate::kernel::fiber::binomial;
use crate::kernel::{FiberDp, Kernel, HIDDEN_SEATS};
use crate::rules::{legal_plays, Domino, DominoSet, Seat, Trick};
use crate::solver::adaptive::{
    root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};

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

    /// The per-hand walk shared by classification and conditioning: the
    /// acting seat's remaining hand, its legal set, and the field's chosen
    /// tile at this belief's public state. Field purity holds by
    /// construction — the record is public data and `remaining` is the
    /// seat's own hand.
    fn field_action(
        &self,
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
        for (hand, weight) in self.actor_completion_weights(belief, seat) {
            let tile = self.field_action(belief, &cursor, hand, field);
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
            self.mass(belief),
            "mass conservation: Z_h = Σ_t Z_ht"
        );
        masses
    }

    fn condition(
        &self,
        belief: &FactorBelief,
        action: Domino,
        field: &dyn SlicePolicy,
    ) -> FactorBelief {
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
        // Theorem 20.1: multiply ONLY the acting seat's factor by the
        // (deterministic) action likelihood — the 0/1 indicator that the
        // field plays `action` from this root hand at this public state.
        let kept: Vec<(DominoSet, u128)> = belief.factors[slot]
            .support()
            .into_iter()
            .filter(|(hand, _)| self.field_action(belief, &cursor, *hand, field) == action)
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

    fn marginal(
        &self,
        belief: &FactorBelief,
        seat: Seat,
        predicate: &dyn Fn(DominoSet) -> bool,
    ) -> u128 {
        self.actor_completion_weights(belief, seat)
            .into_iter()
            .filter(|(hand, _)| predicate(*hand))
            .fold(0u128, |acc, (_, w)| {
                acc.checked_add(w).expect("an exact mass fits u128")
            })
    }
}
