//! Capacity cells: the abstract three-seat schema, the rule-derived native
//! system, mechanical state, and typed support updates.
//!
//! Implements Math §7.1–§7.5 (CELL-05/06/07), §7.14 (TRANS-01..07) and
//! Exec §§15, 17 under INV-1 DERIVED-NOT-STORED: cells and fibers are pure
//! functions of semantic state; no semantic struct stores them.

use crate::algebra::algebra_for;
use crate::algebra::suits::{LedSuit, LedSuitSet};
use crate::algebra::trick::Play;
use crate::domino::{all_ids, DominoId, DominoSet};
use crate::objective::contract::Contract;
use crate::objective::play::{MatchState, PlayPhase};
use crate::seat::Seat;
use crate::support::SupportError;

/// Number of hidden seats from one viewer's perspective (Math §7.1).
pub const HIDDEN_SEATS: usize = 3;

/// An abstract three-seat capacitated cell system over the tile universe
/// `0..universe` (Math §7.1 schema). The exhaustive tiny corpora (TRANS-07,
/// CELL-10..) and the native system's computations both run on this type.
///
/// Structurally a system requires `possible[s] ⊆ universe` and
/// `Σ capacity == universe size` (Exec §15); feasibility is a separate
/// exact property — a well-formed system may denote an empty fiber.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AbstractCells {
    universe: usize,
    possible: [Vec<bool>; 3],
    capacity: [usize; 3],
}

/// One abstract remainder world: three disjoint sorted tile lists exhausting
/// the universe (Math §7.3).
pub type AbstractWorld = [Vec<usize>; 3];

impl AbstractCells {
    /// Validating constructor (Exec §15 structural invariants).
    pub fn new(
        universe: usize,
        possible: [Vec<bool>; 3],
        capacity: [usize; 3],
    ) -> Result<AbstractCells, SupportError> {
        if possible.iter().any(|p| p.len() != universe) {
            return Err(SupportError::InvariantViolation);
        }
        if capacity.iter().sum::<usize>() != universe {
            return Err(SupportError::InvariantViolation);
        }
        Ok(AbstractCells {
            universe,
            possible,
            capacity,
        })
    }

    /// Universe (pool) size.
    pub fn universe(&self) -> usize {
        self.universe
    }

    /// One seat's allowed-set membership vector.
    pub fn possible(&self, seat: usize) -> &[bool] {
        &self.possible[seat]
    }

    /// One seat's capacity.
    pub fn capacity(&self, seat: usize) -> usize {
        self.capacity[seat]
    }

    /// Capacitated Hall feasibility (Math §7.7; CELL-09): the fiber is
    /// nonempty iff every one of the seven nonempty seat subsets `R` has
    /// `|⋃_{s∈R} P_s| ≥ Σ_{s∈R} k_s` (total capacity equals the pool by the
    /// structural invariant).
    pub fn is_feasible(&self) -> bool {
        for mask in 1u8..8 {
            let mut union = vec![false; self.universe];
            let mut demand = 0usize;
            for s in 0..HIDDEN_SEATS {
                if mask & (1 << s) != 0 {
                    demand += self.capacity[s];
                    for (t, &m) in self.possible[s].iter().enumerate() {
                        union[t] = union[t] || m;
                    }
                }
            }
            if union.iter().filter(|&&m| m).count() < demand {
                return false;
            }
        }
        true
    }

    /// Enumerate the exact fiber `Φ(C)` (Math §7.3) — an exact query, never
    /// the definition of the fiber.
    pub fn worlds(&self) -> Vec<AbstractWorld> {
        let mut out = Vec::new();
        let mut current: AbstractWorld = [Vec::new(), Vec::new(), Vec::new()];
        let mut remaining = self.capacity;
        self.enumerate(0, &mut current, &mut remaining, &mut out);
        out
    }

    fn enumerate(
        &self,
        tile: usize,
        current: &mut AbstractWorld,
        remaining: &mut [usize; 3],
        out: &mut Vec<AbstractWorld>,
    ) {
        if tile == self.universe {
            out.push(current.clone());
            return;
        }
        for s in 0..HIDDEN_SEATS {
            if remaining[s] > 0 && self.possible[s][tile] {
                remaining[s] -= 1;
                current[s].push(tile);
                self.enumerate(tile + 1, current, remaining, out);
                current[s].pop();
                remaining[s] += 1;
            }
        }
    }

    /// Remove every allowed edge `(seat, tile)` for which `keep` returns
    /// false, keeping universe and capacities (slice-02 dynamics helper:
    /// the slough deletion step of the matching-minor update, TRANS-09).
    pub fn retain_edges(&mut self, keep: impl Fn(usize, usize) -> bool) {
        for s in 0..HIDDEN_SEATS {
            for tile in 0..self.universe {
                if self.possible[s][tile] && !keep(s, tile) {
                    self.possible[s][tile] = false;
                }
            }
        }
    }

    /// Typed update for a hidden lead or successful follow by seat `s`
    /// playing tile `d` (Math §7.5, §7.14): the tile itself is the witness —
    /// remove `d` from the pool (hence from every allowed set) and lower
    /// `k_s`; no positive follower clause survives (CELL-06).
    pub fn removal_update(&self, seat: usize, tile: usize) -> Result<AbstractCells, SupportError> {
        if tile >= self.universe || !self.possible[seat][tile] || self.capacity[seat] == 0 {
            return Err(SupportError::ImpossibleObservation);
        }
        let mut possible = self.possible.clone();
        for p in &mut possible {
            p.remove(tile);
        }
        let mut capacity = self.capacity;
        capacity[seat] -= 1;
        AbstractCells::new(self.universe - 1, possible, capacity)
    }

    /// Typed update for a hidden failure to follow: seat `s` plays `d` with
    /// `d ∉ F` while context follow set `F` was led (Math §7.5): delete the
    /// whole follow set from `P_s`, remove `d` globally, lower `k_s`.
    pub fn fail_follow_update(
        &self,
        seat: usize,
        tile: usize,
        follow_set: &[bool],
    ) -> Result<AbstractCells, SupportError> {
        if follow_set.len() != self.universe || follow_set[tile] {
            return Err(SupportError::InvariantViolation);
        }
        let mut base = self.clone();
        for (t, &in_follow) in follow_set.iter().enumerate() {
            if in_follow {
                base.possible[seat][t] = false;
            }
        }
        base.removal_update(seat, tile)
    }
}

/// One native remainder world: the three current hidden hands, indexed by
/// clockwise offset from the viewer (Math §6.3 `RemainderWorld`).
///
/// A distinct type from a complete initial [`crate::DealWorld`] (INV-9): no
/// conversion between them exists.
///
/// ```compile_fail
/// use rob_core::{DealWorld, support::cells::RemainderWorld};
/// fn forbidden(deal: DealWorld) -> RemainderWorld { deal.into() }
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RemainderWorld {
    /// Hidden hands by clockwise offset from the viewer (offset 1..=3 maps
    /// to index 0..=2).
    pub hidden_hands: [DominoSet; HIDDEN_SEATS],
}

/// The rule-derived native cell system (Exec §15 `RuleDerivedCellSystem`):
/// a derived view of mechanical state, never a stored field (INV-1; D2).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuleDerivedCellSystem {
    unseen_pool: DominoSet,
    possible: [DominoSet; HIDDEN_SEATS],
    capacity: [usize; HIDDEN_SEATS],
}

impl RuleDerivedCellSystem {
    /// The common unseen pool `U` (Math §7.1).
    pub fn unseen_pool(&self) -> &DominoSet {
        &self.unseen_pool
    }

    /// Locally allowed holder set of one hidden seat (clockwise offset − 1).
    pub fn possible(&self, hidden_index: usize) -> &DominoSet {
        &self.possible[hidden_index]
    }

    /// Exact remaining capacity of one hidden seat.
    pub fn capacity(&self, hidden_index: usize) -> usize {
        self.capacity[hidden_index]
    }

    /// Total locally allowed holder edges `Σ_s |P_s|` (TRANS-12 static
    /// budget: at most `3·21 = 63`).
    pub fn holder_edge_count(&self) -> usize {
        self.possible.iter().map(|p| p.len()).sum()
    }

    /// Convert to the abstract schema plus the tile order mapping abstract
    /// indices back to dominoes (canonical identity order of the pool).
    pub fn to_abstract(&self) -> (AbstractCells, Vec<DominoId>) {
        let tiles: Vec<DominoId> = self.unseen_pool.iter().collect();
        let possible = core::array::from_fn(|s| {
            tiles
                .iter()
                .map(|&d| self.possible[s].contains(d))
                .collect::<Vec<bool>>()
        });
        let cells = AbstractCells::new(tiles.len(), possible, self.capacity)
            .expect("rule-derived cells satisfy the structural invariants");
        (cells, tiles)
    }

    /// Capacitated Hall feasibility (Exec §15 `isFeasible`; CELL-09).
    pub fn is_feasible(&self) -> bool {
        self.to_abstract().0.is_feasible()
    }

    /// Enumerate the exact native fiber `Φ(C)` (Math §7.3) — an exact
    /// query on the intensional fiber.
    pub fn fiber_worlds(&self) -> Vec<RemainderWorld> {
        let (cells, tiles) = self.to_abstract();
        cells
            .worlds()
            .into_iter()
            .map(|world| RemainderWorld {
                hidden_hands: core::array::from_fn(|s| {
                    DominoSet::from_ids(world[s].iter().map(|&t| tiles[t]))
                }),
            })
            .collect()
    }

    /// Exact membership test (Exec §16 `contains`): subsets of allowed sets,
    /// exact capacities, pairwise disjoint, union the pool.
    pub fn fiber_contains(&self, world: &RemainderWorld) -> bool {
        let mut union = DominoSet::empty();
        for s in 0..HIDDEN_SEATS {
            let hand = &world.hidden_hands[s];
            if !hand.is_subset(&self.possible[s]) || hand.len() != self.capacity[s] {
                return false;
            }
            for other in world.hidden_hands.iter().skip(s + 1) {
                if !hand.is_disjoint(other) {
                    return false;
                }
            }
            union = union.union(hand);
        }
        union == self.unseen_pool
    }
}

/// The viewer's mechanical/support state (Exec §15 `MechanicalState`):
/// exactly the semantic fields from which cells, fiber, and normal form are
/// derived. Stores no derived view (INV-1; D2) and no reachability flag
/// (INV-3; D1); equality is structural over these semantic fields only.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MechanicalState {
    viewer: Seat,
    own_remaining_hand: DominoSet,
    contract: Contract,
    leader: Seat,
    current_trick: Vec<Play>,
    hand_points: [u32; 2],
    match_state: Option<MatchState>,
    played_by_seat: [DominoSet; 4],
    public_voids: [LedSuitSet; 4],
    phase: PlayPhase,
}

impl MechanicalState {
    /// Optional retained match residue (Exec §15 `matchState`).
    pub fn match_state(&self) -> Option<&MatchState> {
        self.match_state.as_ref()
    }

    /// The viewing seat.
    pub fn viewer(&self) -> Seat {
        self.viewer
    }

    /// The viewer's known remaining hand.
    pub fn own_remaining_hand(&self) -> &DominoSet {
        &self.own_remaining_hand
    }

    /// The certified contract.
    pub fn contract(&self) -> &Contract {
        &self.contract
    }

    /// The current trick leader.
    pub fn leader(&self) -> Seat {
        self.leader
    }

    /// The current partial trick.
    pub fn current_trick(&self) -> &[Play] {
        &self.current_trick
    }

    /// Banked hand points by partnership.
    pub fn hand_points(&self) -> [u32; 2] {
        self.hand_points
    }

    /// Actor-attributed publicly played set of one seat (includes dominoes
    /// currently in the trick).
    pub fn played_by_seat(&self, seat: Seat) -> &DominoSet {
        &self.played_by_seat[seat.index()]
    }

    /// Public void contexts of one seat (Math §7.1 `V_s`).
    pub fn public_voids(&self, seat: Seat) -> &LedSuitSet {
        &self.public_voids[seat.index()]
    }

    /// The hand phase.
    pub fn phase(&self) -> PlayPhase {
        self.phase
    }

    /// The seat to act (play phase only).
    pub fn current_actor(&self) -> Option<Seat> {
        match self.phase {
            PlayPhase::Play => Some(self.leader.offset(self.current_trick.len() as u8)),
            PlayPhase::HandComplete => None,
        }
    }

    /// The hidden seats in clockwise order from the viewer (Math §7.1).
    pub fn hidden_seats(&self) -> [Seat; HIDDEN_SEATS] {
        [
            self.viewer.offset(1),
            self.viewer.offset(2),
            self.viewer.offset(3),
        ]
    }
}

/// The initial contracted mechanical state for one viewer (Exec §17
/// projection base): own dealt hand, bidder leads, nothing played.
pub fn initial_contracted_mechanical(
    viewer: Seat,
    own_hand: DominoSet,
    contract: Contract,
) -> Result<MechanicalState, SupportError> {
    if own_hand.len() != 7 {
        return Err(SupportError::InvariantViolation);
    }
    Ok(MechanicalState {
        viewer,
        own_remaining_hand: own_hand,
        contract,
        leader: contract.bidder(),
        current_trick: Vec::new(),
        hand_points: [0, 0],
        match_state: None,
        played_by_seat: core::array::from_fn(|_| DominoSet::empty()),
        public_voids: core::array::from_fn(|_| LedSuitSet::empty()),
        phase: PlayPhase::Play,
    })
}

/// The initial unconstrained auction-phase cell system (Exec §15
/// `deriveAuctionCells`; Math §7.4, AUC-07/08): pool is the 21 tiles outside
/// the viewer hand, every allowed set is the pool, every capacity is 7 —
/// straight bids and declarations remove no deal by rule.
pub fn derive_auction_cells(
    viewer_hand: &DominoSet,
) -> Result<RuleDerivedCellSystem, SupportError> {
    if viewer_hand.len() != 7 {
        return Err(SupportError::InvariantViolation);
    }
    let pool = DominoSet::full().difference(viewer_hand);
    Ok(RuleDerivedCellSystem {
        unseen_pool: pool,
        possible: [pool; HIDDEN_SEATS],
        capacity: [7; HIDDEN_SEATS],
    })
}

/// Derive the rule cells from mechanical state (Exec §15 `deriveRuleCells`;
/// Math §7.1) — the semantic source of support, recomputed exactly on every
/// call (INV-1):
/// `U = D ∖ own ∖ ⋃ played`, `P_s = U ∖ ⋃_{q ∈ V_s} σ̂_q`,
/// `k_s = 7 − |played_s|`.
pub fn derive_rule_cells(state: &MechanicalState) -> RuleDerivedCellSystem {
    let algebra = algebra_for(state.contract.declaration());
    let mut pool = state.own_remaining_hand;
    for played in &state.played_by_seat {
        pool = pool.union(played);
    }
    let pool = DominoSet::full().difference(&pool);
    let hidden = state.hidden_seats();
    let possible = core::array::from_fn(|i| {
        let seat = hidden[i];
        let mut allowed = pool;
        for q in state.public_voids[seat.index()].iter() {
            allowed = DominoSet::from_ids(allowed.iter().filter(|&d| !algebra.follows(d, q)));
        }
        allowed
    });
    let capacity = core::array::from_fn(|i| 7 - state.played_by_seat[hidden[i].index()].len());
    RuleDerivedCellSystem {
        unseen_pool: pool,
        possible,
        capacity,
    }
}

/// Typed support update for one observed actor-attributed play (Exec §17
/// `updateHiddenSupport` / `updateViewerSupport`): performs the same public
/// physical transition as the objective game, records public voids from
/// failures to follow, and lets the successor cells be *derived* (the pool
/// removal, capacity decrement, allowed-set deletion, and void exclusion are
/// consequences of `derive_rule_cells`, not separately mutable state).
pub fn update_support(
    state: &MechanicalState,
    play: Play,
) -> Result<MechanicalState, SupportError> {
    if state.phase != PlayPhase::Play {
        return Err(SupportError::PhaseMismatch);
    }
    let actor = state.current_actor().expect("play phase");
    if play.actor != actor {
        return Err(SupportError::ImpossibleObservation);
    }
    let algebra = algebra_for(state.contract.declaration());
    let d = play.domino;

    let led = state
        .current_trick
        .first()
        .map(|p| algebra.led_suit(p.domino));
    let followed = led.is_none_or(|q| algebra.follows(d, q));

    if actor == state.viewer {
        // The play must be legal against the known viewer hand.
        if !state.own_remaining_hand.contains(d) {
            return Err(SupportError::IllegalPlay);
        }
        if let Some(q) = led {
            let has_follower = state
                .own_remaining_hand
                .iter()
                .any(|e| algebra.follows(e, q));
            if !followed && has_follower {
                return Err(SupportError::IllegalPlay);
            }
        }
    } else {
        // Preconditions on the hidden observation (Exec §17).
        let cells = derive_rule_cells(state);
        let hidden_index = state
            .hidden_seats()
            .iter()
            .position(|&s| s == actor)
            .expect("actor is hidden");
        if !cells.unseen_pool().contains(d) || !cells.possible(hidden_index).contains(d) {
            return Err(SupportError::ImpossibleObservation);
        }
    }

    let mut next = state.clone();
    if actor == state.viewer {
        next.own_remaining_hand.remove(d);
    }
    next.current_trick.push(play);
    next.played_by_seat[actor.index()].insert(d);
    if let Some(q) = led {
        if !followed {
            next.public_voids[actor.index()].insert(q);
        }
    }

    if next.current_trick.len() == 4 {
        let result = algebra
            .resolve_trick(&next.current_trick)
            .map_err(|_| SupportError::InvariantViolation)?;
        next.hand_points[result.winner.team().index()] += result.points as u32;
        next.current_trick.clear();
        next.leader = result.winner;
    }

    let post_cells = derive_rule_cells(&next);
    if next.own_remaining_hand.is_empty() && (0..HIDDEN_SEATS).all(|i| post_cells.capacity(i) == 0)
    {
        if !next.current_trick.is_empty() || next.hand_points[0] + next.hand_points[1] != 42 {
            return Err(SupportError::InvariantViolation);
        }
        next.phase = PlayPhase::HandComplete;
    }
    if !post_cells.is_feasible() {
        return Err(SupportError::InfeasibleCellSystem);
    }
    Ok(next)
}

/// A compiled view coupling a mechanical state with an optional cells cache
/// (Exec §15 `MechanicalCompiledView`). The cache lives outside semantic
/// equality; [`MechanicalCompiledView::coherent`] is the INV-1 coherence
/// assertion — a cache mismatch is an invariant violation, never an
/// alternative state.
#[derive(Clone, Debug)]
pub struct MechanicalCompiledView {
    /// The semantic state (the sole equality carrier).
    pub state: MechanicalState,
    /// Optional cached derivation result.
    pub cells_cache: Option<RuleDerivedCellSystem>,
}

impl PartialEq for MechanicalCompiledView {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for MechanicalCompiledView {}

impl MechanicalCompiledView {
    /// The INV-1 coherence invariant: any cached cells equal a fresh
    /// derivation from the semantic state.
    pub fn coherent(&self) -> bool {
        match &self.cells_cache {
            None => true,
            Some(cache) => *cache == derive_rule_cells(&self.state),
        }
    }
}

/// The effective follow set of a context under a declaration, as a
/// `DominoSet` (σ̂_q^δ; used to spell the void deletion in Math §7.1).
pub fn effective_suit_set(declaration: crate::declaration::Declaration, q: LedSuit) -> DominoSet {
    let algebra = algebra_for(declaration);
    DominoSet::from_ids(all_ids().filter(|&d| algebra.follows(d, q)))
}

/// Sample one exactly uniform native remainder world from the fiber of a
/// rule-derived cell system (CELL-10E/F): the abstract count-ratio sampler
/// lifted through the canonical offset ↔ `DominoId` bijection, so callers
/// no longer re-implement the translation (slice-01 ergonomic finding).
pub fn sample_native_world(
    cells: &RuleDerivedCellSystem,
    source: &mut dyn crate::support::sampler::ExactRationalChoiceSource,
) -> RemainderWorld {
    let (abstract_cells, tile_order) = cells.to_abstract();
    let world = crate::support::sampler::sample_uniform_world(&abstract_cells, source);
    RemainderWorld {
        hidden_hands: core::array::from_fn(|s| {
            DominoSet::from_ids(world[s].iter().map(|&t| tile_order[t]))
        }),
    }
}
