//! `solver::adaptive` — the calculated-evidence decision path, first slice.
//!
//! EXPLORATORY tier. The A.6 minimum coherent vertical slice of the
//! verbatim parent `walt/math/calculated_evidence_v0.1.md`, adopted as the
//! build program's first landing at ruling CE-A7 (`walt/CENSUS-RULINGS.md`):
//! one canonical kernel adapter per live root, a fixed frozen-policy pair
//! on one common indexed world stream, exact pivotal evidence from
//! [`crate::solver::evidence`], `DeltaSettled` versus `Unresolved`, and the
//! exact full-fiber endpoint on a small root.
//!
//! Result kinds are the six-way ladder of parent §1, BINDING for this path
//! per CE-A3: mechanically distinct, serialized with the type preserved. A
//! sample cap is a resource limit producing `Unresolved`, never a
//! settlement rule (CE-A3/A5). The legacy `sample_belief` paths in
//! `solver::mod`, `bin/playout.rs`, and `bin/walt_bridge.rs` are untouched:
//! they stay as-is outside this correctness path (parent §A.3, §11.1).
//!
//! The [`SlicePolicy`] trait is the seam for frozen policies. This module
//! keeps the simple declared-constant policies of the first slice
//! ([`FixedPreference`]); the full FreezeTuple/PolicyId machinery of parent
//! §12 (§22 step 4) lives in [`crate::solver::policy`], whose
//! `FrozenPolicy` implements the same trait and plugs into
//! [`evaluate_pair`] and [`exact_frozen_pair`] unchanged.

use std::collections::HashMap;
use std::fmt;

use num_rational::BigRational;

use crate::kernel::{
    FiberDp, FiberIter, Hidden, Kernel, KernelError, SplitMix64 as KernelRng, World,
};
use crate::rules::receipt::ReceiptHand;
use crate::rules::replay::{state_before_trick, voids_before_trick, ReplayError};
use crate::rules::rules::{legal_plays, Trick};
use crate::rules::{ContextSet, Decl, Domino, DominoSet, Seat, Team};
use crate::solver::evidence::{self, ScopedDelta};
use crate::solver::{arena_decl_id, mix};

/// The sampler this slice declares in every probabilistic result (§17.4):
/// the canonical kernel's count-driven exactly-uniform sampler, drawn with
/// replacement, seeded per world index from (root identity, epoch, index).
pub const SAMPLER_ID: &str = "kernel-fiberdp-splitmix64-counter-v1";

/// Domain-separation tag for the evidence world stream's seed derivation.
/// Public so the §12.4 disjointness gate can assert it differs from the
/// discovery domain tag in [`crate::solver::policy`]: evidence and
/// discovery streams must never share a seed derivation.
pub const STREAM_DOMAIN: u64 = 0xCE00_51CE_0A6E_57A6;

// ---------------------------------------------------------------------------
// Result kinds (parent §1; CE-A3).
// ---------------------------------------------------------------------------

/// The identity of the probability space supporting a `δ` claim (§17.4):
/// sampler, seed provenance (root identity + epoch + counter), replacement
/// semantics, and the exact fiber the stream targets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamIdentity {
    pub sampler: &'static str,
    /// The stream's root identity: a digest of the kernel and the public
    /// root position. World `i` is a pure function of (root_id, epoch, i).
    pub root_id: u64,
    pub epoch: u64,
    pub with_replacement: bool,
    /// Exact `|Φ(C)|` of the targeted fiber.
    pub fiber: u128,
}

impl fmt::Display for StreamIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "stream{{sampler={};root={:#018x};epoch={};with_replacement={};fiber={}}}",
            self.sampler, self.root_id, self.epoch, self.with_replacement, self.fiber
        )
    }
}

/// The six-way result ladder of parent §1 — mechanically distinct, and
/// serialized (via `Display`) with the type preserved (CE-A3). In this
/// slice only `ExactFrozenSet`, `DeltaSettled`, and `Unresolved` are
/// producible; the other three exist as types so no consumer can be built
/// against a smaller ladder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultKind {
    /// §1.1 — complete-fiber solve under the declared field model. Not
    /// producible in this slice (§22 step 6, exact-root half, is later).
    ExactFiberRoot { fiber: u128 },
    /// §1.2 — every named frozen candidate evaluated over the complete
    /// fiber; best of the fixed set selected exactly. `winner` is `None`
    /// exactly on an exact tie (several best members — honest output).
    ExactFrozenSet {
        policy_a: String,
        policy_b: String,
        wins_a: u128,
        wins_b: u128,
        fiber: u128,
        root_id: u64,
        winner: Option<String>,
    },
    /// §1.3 — best of the fixed pair except on an event of probability at
    /// most the scoped δ under the declared sampling law. Probabilistic,
    /// not exact.
    DeltaSettled {
        winner: String,
        /// First-crossing world index in the stream (§17.3).
        settled_at: u64,
        a: u64,
        b: u64,
        threshold: BigRational,
        delta: ScopedDelta,
        stream: StreamIdentity,
    },
    /// §1.4 — all survivors within a declared utility tolerance ε at
    /// declared risk. Not producible in this slice.
    EpsilonEquivalent {
        epsilon: BigRational,
        delta: ScopedDelta,
    },
    /// §1.5 — the resource cap arrived before any stopping condition. A
    /// successful honest output, never silently a tie-break.
    Unresolved {
        consumed: u64,
        a: u64,
        b: u64,
        evidence_plus: BigRational,
        evidence_minus: BigRational,
        threshold: BigRational,
        delta: ScopedDelta,
        stream: StreamIdentity,
    },
    /// §1.6 — an explicitly named fallback chose after `Unresolved`; no
    /// exact or δ-settled claim. Not producible in this slice.
    HeuristicFallback { fallback: String },
}

impl ResultKind {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            ResultKind::ExactFiberRoot { .. } => "ExactFiberRoot",
            ResultKind::ExactFrozenSet { .. } => "ExactFrozenSet",
            ResultKind::DeltaSettled { .. } => "DeltaSettled",
            ResultKind::EpsilonEquivalent { .. } => "EpsilonEquivalent",
            ResultKind::Unresolved { .. } => "Unresolved",
            ResultKind::HeuristicFallback { .. } => "HeuristicFallback",
        }
    }
}

impl fmt::Display for ResultKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResultKind::ExactFiberRoot { fiber } => {
                write!(f, "ExactFiberRoot{{fiber={fiber}}}")
            }
            ResultKind::ExactFrozenSet {
                policy_a,
                policy_b,
                wins_a,
                wins_b,
                fiber,
                root_id,
                winner,
            } => write!(
                f,
                "ExactFrozenSet{{a={policy_a};wins_a={wins_a};b={policy_b};wins_b={wins_b};\
                 fiber={fiber};root={root_id:#018x};winner={}}}",
                winner.as_deref().unwrap_or("exact-tie")
            ),
            ResultKind::DeltaSettled {
                winner,
                settled_at,
                a,
                b,
                threshold,
                delta,
                stream,
            } => write!(
                f,
                "DeltaSettled{{winner={winner};settled_at={settled_at};a={a};b={b};\
                 T={threshold};{delta};{stream}}}"
            ),
            ResultKind::EpsilonEquivalent { epsilon, delta } => {
                write!(f, "EpsilonEquivalent{{epsilon={epsilon};{delta}}}")
            }
            ResultKind::Unresolved {
                consumed,
                a,
                b,
                evidence_plus,
                evidence_minus,
                threshold,
                delta,
                stream,
            } => write!(
                f,
                "Unresolved{{consumed={consumed};a={a};b={b};E_plus={evidence_plus};\
                 E_minus={evidence_minus};T={threshold};{delta};{stream}}}"
            ),
            ResultKind::HeuristicFallback { fallback } => {
                write!(f, "HeuristicFallback{{fallback={fallback}}}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The canonical kernel adapter (§22 step 2; parent §16.1, §11.1).
// ---------------------------------------------------------------------------

/// One canonical kernel per live root decision: THE SAME object serves the
/// exact count `|Φ(C)|`, exactly-uniform sampled worlds (with replacement,
/// §11.2), canonical world identity, and lazy full enumeration. The
/// `FiberDp` is a derived, immutable function of the kernel — a memo, not
/// a second authority.
pub struct CanonicalRoot {
    kernel: Kernel,
    dp: FiberDp,
    kernel_id: u64,
}

impl CanonicalRoot {
    pub fn new(kernel: Kernel) -> CanonicalRoot {
        let dp = FiberDp::new(&kernel);
        let kernel_id = kernel_identity(&kernel);
        CanonicalRoot {
            kernel,
            dp,
            kernel_id,
        }
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    /// Exact `|Φ(C)|` from the shared counting DP.
    pub fn count(&self) -> u128 {
        self.dp.count()
    }

    /// Digest of the kernel alone (fiber identity). The evidence stream's
    /// root identity additionally folds in the public root position; see
    /// [`root_identity`].
    pub fn kernel_id(&self) -> u64 {
        self.kernel_id
    }

    /// Lazy exhaustive enumeration of the same fiber.
    pub fn worlds(&self) -> FiberIter<'_> {
        self.kernel.worlds()
    }

    /// Counter-based world identity (§17.1): world `i` of the stream is a
    /// pure function of (root identity, evaluation epoch, `i`), drawn
    /// exactly uniformly (with replacement) by the kernel's count-driven
    /// sampler. Batch size, thread count, elimination, and resume
    /// boundaries therefore cannot change which world occupies index `i`.
    ///
    /// Panics if the fiber is empty — an empty fiber has no worlds to
    /// stream and no decision to make.
    pub fn world_at(&self, root_id: u64, epoch: u64, index: u64) -> World {
        let mut seed = mix(root_id ^ STREAM_DOMAIN);
        seed = mix(seed ^ epoch);
        seed = mix(seed ^ index);
        let mut rng = KernelRng::new(seed);
        self.kernel
            .sample_with(&self.dp, &mut rng)
            .expect("a streamed root fiber is nonempty")
    }
}

/// Canonical world identity: the four hand bitmasks, seat-indexed. Two
/// worlds are the same physical world exactly when their ids are equal.
pub fn world_id(world: &World) -> [u32; 4] {
    let hands = world.hands();
    core::array::from_fn(|i| hands[i].bits())
}

fn kernel_identity(kernel: &Kernel) -> u64 {
    let mut h = mix(arena_decl_id(kernel.decl()) as u64);
    h = mix(h ^ kernel.viewer().index() as u64);
    h = mix(h ^ u64::from(kernel.viewer_hand().bits()));
    h = mix(h ^ u64::from(kernel.pool().bits()));
    for hidden in kernel.hidden() {
        h = mix(h ^ hidden.seat.index() as u64);
        h = mix(h ^ hidden.capacity as u64);
        let voids = hidden
            .voids
            .iter()
            .fold(0u64, |acc, q| acc | (1u64 << q.index()));
        h = mix(h ^ voids);
    }
    h
}

// ---------------------------------------------------------------------------
// The public root position and terminal pmake replay.
// ---------------------------------------------------------------------------

/// The public frame of one root decision: declaration, contract, whose
/// lead, banked totals so far, the current partial trick, and the pre-root
/// public residue a continuation policy needs (`prior_played`, observed
/// voids). Derived by replay from a receipt or a driven game — never
/// stored authority. `Hash` so the frame can enter an
/// information-consistent action key (`solver::policy`, §12.3).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RootPosition {
    pub decl: Decl,
    /// The contract in points: the declaring team makes at or above it.
    pub bid: u32,
    pub declaring_team: Team,
    pub leader: Seat,
    /// Banked points so far, indexed by `Team::index()`.
    pub banked: [u32; 2],
    /// The current partial trick's tiles in play order (empty at a trick
    /// start). Worlds of the matching kernel already exclude these tiles.
    pub trick_plays: Vec<Domino>,
    /// Every tile played in tricks COMPLETED before the root — the played
    /// mask at the current trick's start. With `trick_plays` this is the
    /// complete played set at the root, a derived view of the public
    /// record (the kernel's live set is its complement; the bridge asserts
    /// their agreement).
    pub prior_played: DominoSet,
    /// Observed void contexts per seat (union of failures to follow up to
    /// and including the current partial trick), seat-indexed. Public
    /// data: derived from the record, mirrored by the kernel's hidden-slot
    /// voids for the non-viewer seats.
    pub voids: [ContextSet; 4],
}

impl RootPosition {
    /// The public position at the start of `trick_no` (1-based) of a
    /// receipt hand — the companion of `Kernel::from_receipt_trick`.
    /// Banked totals, the prior played mask, and the observed voids are
    /// derived by replaying the completed tricks through the rules
    /// machinery, not read from stored fields.
    pub fn from_receipt_trick(
        hand: &ReceiptHand,
        trick_no: usize,
    ) -> Result<RootPosition, ReplayError> {
        assert!(trick_no >= 1, "tricks are numbered from one");
        let (_, leader) = state_before_trick(hand, trick_no)?;
        let mut banked = [0u32; 2];
        let mut prior_played = DominoSet::EMPTY;
        for trick in hand.tricks.iter().take(trick_no - 1) {
            let doms: [Domino; 4] = core::array::from_fn(|i| trick.plays[i].1);
            let t = Trick::new(trick.plays[0].0, doms).map_err(|e| ReplayError {
                hand: hand.id,
                trick: Some(trick.number),
                message: format!("repeated domino {:?}", e.0),
            })?;
            banked[t.winner(hand.decl).team().index()] += t.points();
            for d in doms {
                prior_played.insert(d);
            }
        }
        Ok(RootPosition {
            decl: hand.decl,
            bid: hand.bid_points,
            declaring_team: hand.declaring_team,
            leader,
            banked,
            trick_plays: Vec::new(),
            prior_played,
            voids: voids_before_trick(hand, trick_no),
        })
    }

    /// Digest of the public position, folded into the stream root identity.
    pub fn identity(&self) -> u64 {
        let mut h = mix(arena_decl_id(self.decl) as u64);
        h = mix(h ^ u64::from(self.bid));
        h = mix(h ^ self.declaring_team.index() as u64);
        h = mix(h ^ self.leader.index() as u64);
        h = mix(h ^ u64::from(self.banked[0]));
        h = mix(h ^ u64::from(self.banked[1]));
        for d in &self.trick_plays {
            h = mix(h ^ (0x100 | d.index() as u64));
        }
        h = mix(h ^ u64::from(self.prior_played.bits()));
        for voids in &self.voids {
            let folded = voids.iter().fold(0u64, |acc, q| acc | (1u64 << q.index()));
            h = mix(h ^ (0x200 | folded));
        }
        h
    }
}

/// The complete public state of one driven-game decision point, in the
/// solver's internal seat labels — the input to the live-root bridge. All
/// fields are public data (a seat's own view plus the shared record); the
/// seat to move is the derived view `leader.plus(trick_plays.len())`.
pub struct DrivenState<'a> {
    pub decl: Decl,
    pub bid: u32,
    pub declaring_team: Team,
    /// The seat to move's own remaining hand.
    pub viewer_hand: DominoSet,
    pub leader: Seat,
    /// The current partial trick in play order (may be empty).
    pub trick_plays: &'a [Domino],
    /// Banked points so far, indexed by `Team::index()`.
    pub banked: [u32; 2],
    /// Tiles of tricks completed before the current trick.
    pub prior_played: DominoSet,
    /// Observed voids per seat, INCLUDING failures inside the current
    /// partial trick.
    pub voids: [ContextSet; 4],
}

/// §22 step 7 — the live-root bridge: construct the canonical objects
/// (`CanonicalRoot` + `RootPosition`) for a mid-hand decision of a DRIVEN
/// game, the generalization of the receipt-driven
/// `Kernel::from_receipt_trick` / `RootPosition::from_receipt_trick` pair
/// (and of `ReceiptDecision::at` for mid-trick points). Everything is a
/// derived view of the driven public state; capacities, the pool, and the
/// hidden voids are recomputed here, never stored twice.
pub fn driven_root(state: &DrivenState<'_>) -> Result<(CanonicalRoot, RootPosition), KernelError> {
    assert!(
        state.prior_played.len().is_multiple_of(4),
        "completed tricks are whole"
    );
    assert!(
        state.trick_plays.len() < 4,
        "a partial trick has at most 3 plays"
    );
    let completed = state.prior_played.len() / 4;
    let trick_size = 7 - completed;
    assert_eq!(
        state.viewer_hand.len(),
        trick_size,
        "the seat to move has not yet played in the current trick"
    );
    let viewer = state.leader.plus(state.trick_plays.len());
    let mut in_trick = DominoSet::EMPTY;
    for d in state.trick_plays {
        assert!(in_trick.insert(*d), "a trick plays a tile once");
    }
    assert!(
        in_trick.is_disjoint(state.prior_played),
        "current-trick tiles are not previously played"
    );
    assert!(
        state
            .viewer_hand
            .is_disjoint(state.prior_played.union(in_trick)),
        "a remaining hand is disjoint from the played record"
    );
    let pool = DominoSet::FULL
        .difference(state.prior_played)
        .difference(in_trick)
        .difference(state.viewer_hand);
    let mut hidden = [Hidden {
        seat: viewer,
        capacity: 0,
        voids: ContextSet::EMPTY,
    }; crate::kernel::HIDDEN_SEATS];
    for (slot, seat) in hidden.iter_mut().zip((1..=3).map(|k| viewer.plus(k))) {
        // Seats between the leader and the viewer have already played in
        // the current trick; the others have not.
        let played_this_trick = (0..state.trick_plays.len()).any(|k| state.leader.plus(k) == seat);
        *slot = Hidden {
            seat,
            capacity: trick_size - usize::from(played_this_trick),
            voids: state.voids[seat.index()],
        };
    }
    let kernel = Kernel::new(state.decl, viewer, state.viewer_hand, pool, hidden)?;
    let position = RootPosition {
        decl: state.decl,
        bid: state.bid,
        declaring_team: state.declaring_team,
        leader: state.leader,
        banked: state.banked,
        trick_plays: state.trick_plays.to_vec(),
        prior_played: state.prior_played,
        voids: state.voids,
    };
    Ok((CanonicalRoot::new(kernel), position))
}

/// The evidence stream's root identity (§17.1): kernel fiber plus public
/// position.
pub fn root_identity(root: &CanonicalRoot, position: &RootPosition) -> u64 {
    mix(root.kernel_id() ^ mix(position.identity()))
}

/// What a policy is allowed to see beyond its own hand: the public record
/// only (§12.3). The evaluation world's hidden hands never reach a policy.
///
/// `root` and `history` are the FULL public record since the root — the
/// authority a `solver::policy` information-consistent key stores (§12.3:
/// full record now, proved sufficient reductions later). `leader`,
/// `trick_plays`, and `banked` are derived views of (root, history),
/// carried for simple policies' convenience; they are never a second
/// stored authority.
pub struct PublicRecord<'a> {
    pub leader: Seat,
    pub trick_plays: &'a [Domino],
    pub banked: [u32; 2],
    /// The public root frame this record extends.
    pub root: &'a RootPosition,
    /// Every tile played after the root, in play order (completed tricks
    /// then the current partial trick). The current trick's plays are its
    /// suffix.
    pub history: &'a [Domino],
}

/// A deterministic information-consistent frozen policy. `choose` sees the
/// seat's own remaining hand, its legal set, and the public record —
/// nothing hidden. Identity may be a declared constant (this module's
/// [`FixedPreference`]) or a full content-addressed PolicyId
/// (`solver::policy::FrozenPolicy`, parent §12).
pub trait SlicePolicy {
    fn id(&self) -> &str;
    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino;
}

/// A fixed preference ordering over all 28 tiles: the policy plays its
/// most-preferred legal tile. Trivially deterministic and
/// information-consistent.
pub struct FixedPreference {
    id: &'static str,
    order: Vec<Domino>,
}

impl FixedPreference {
    pub fn new(id: &'static str, order: Vec<Domino>) -> FixedPreference {
        let mut seen = DominoSet::EMPTY;
        for d in &order {
            assert!(seen.insert(*d), "a preference order lists a tile twice");
        }
        assert_eq!(seen, DominoSet::FULL, "a preference order covers all 28");
        FixedPreference { id, order }
    }

    /// Ascending tile index — the stack's lowest-tile convention.
    pub fn lowest_first(id: &'static str) -> FixedPreference {
        FixedPreference::new(
            id,
            (0..DominoSet::FULL.len())
                .map(|i| Domino::from_index(i).expect("index < 28"))
                .collect(),
        )
    }

    /// Descending tile index.
    pub fn highest_first(id: &'static str) -> FixedPreference {
        FixedPreference::new(
            id,
            (0..DominoSet::FULL.len())
                .rev()
                .map(|i| Domino::from_index(i).expect("index < 28"))
                .collect(),
        )
    }
}

impl SlicePolicy for FixedPreference {
    fn id(&self) -> &str {
        self.id
    }

    fn choose(
        &self,
        _decl: Decl,
        _hand: DominoSet,
        legal: DominoSet,
        _record: &PublicRecord<'_>,
    ) -> Domino {
        *self
            .order
            .iter()
            .find(|d| legal.contains(**d))
            .expect("a seat to move holds a legal tile")
    }
}

/// The total banked points of one complete hand: 7 trick points plus the
/// 35 count points. Gated live: every walk asserts a terminal state is
/// decided, which fails if any hand's banked total misses this value.
const TOTAL_POINTS: u32 = 42;

/// Whether the viewer-objective terminal make indicator is already decided
/// at a public state, for EVERY continuation: the declaring side has
/// banked its bid (monotone — points only accumulate), or the unbanked
/// remainder of the 42-point pool cannot reach it. At a terminal state the
/// pool is empty, so the answer is always `Some` there (`at_terminal`
/// asserts it).
pub fn decided_success(
    position: &RootPosition,
    viewer: Seat,
    banked: [u32; 2],
    at_terminal: bool,
) -> Option<bool> {
    let total = banked[0] + banked[1];
    assert!(
        total <= TOTAL_POINTS,
        "banked points never exceed the 42-point pool"
    );
    let declared = banked[position.declaring_team.index()];
    let pool = TOTAL_POINTS - total;
    let made = if declared >= position.bid {
        Some(true)
    } else if declared + pool < position.bid {
        Some(false)
    } else {
        None
    };
    assert!(
        !(at_terminal && made.is_none()),
        "the 42-point pool exhausts at terminal, so a terminal outcome is decided"
    );
    made.map(|m| {
        if viewer.team() == position.declaring_team {
            m
        } else {
            !m
        }
    })
}

/// Replay one world from the root position: the viewer seat plays
/// `focal`, every other seat plays `field` (the declared deterministic
/// field model of this slice). Returns the viewer-objective make
/// indicator: whether the viewer's team achieved its pmake objective
/// (make for the declaring side, set for the other). The replay stops at
/// the first trick boundary where the indicator is decided for every
/// continuation ([`decided_success`] — the indicator is monotone, points
/// only accumulate), which is value-identical to playing out the
/// remaining plies; a terminal state is always decided, so the truncation
/// never changes the returned Boolean.
pub fn replay_viewer_success(
    position: &RootPosition,
    viewer: Seat,
    world: &World,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
) -> bool {
    let mut hands = world.hands();
    let mut leader = position.leader;
    let mut plays = position.trick_plays.clone();
    let mut banked = position.banked;
    let mut history: Vec<Domino> = Vec::new();
    if let Some(decided) = decided_success(position, viewer, banked, false) {
        return decided;
    }
    while hands.iter().any(|h| !h.is_empty()) {
        let seat = leader.plus(plays.len());
        let led = plays.first().map(|d| position.decl.led_context(*d));
        let hand = hands[seat.index()];
        let legal = legal_plays(position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = PublicRecord {
            leader,
            trick_plays: &plays,
            banked,
            root: position,
            history: &history,
        };
        let policy = if seat == viewer { focal } else { field };
        let tile = policy.choose(position.decl, hand, legal, &record);
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        assert!(hands[seat.index()].remove(tile), "the chosen tile is held");
        plays.push(tile);
        history.push(tile);
        if plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| plays[i]);
            let trick = Trick::new(leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            banked[winner.team().index()] += trick.points();
            leader = winner;
            plays.clear();
            if let Some(decided) = decided_success(position, viewer, banked, false) {
                return decided;
            }
        }
    }
    assert!(plays.is_empty(), "a hand ends on a trick boundary");
    let made = banked[position.declaring_team.index()] >= position.bid;
    if viewer.team() == position.declaring_team {
        made
    } else {
        !made
    }
}

// ---------------------------------------------------------------------------
// The fixed-pair adaptive evaluator (§22 step 5, minimum).
// ---------------------------------------------------------------------------

/// One fixed-pair evaluation request. The pair, field, δ scope, epoch, and
/// cap together freeze the evaluation's identity for this slice.
pub struct PairSpec<'a> {
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub policy_a: &'a dyn SlicePolicy,
    pub policy_b: &'a dyn SlicePolicy,
    /// The declared deterministic field model for the non-focal seats.
    pub field: &'a dyn SlicePolicy,
    pub delta: ScopedDelta,
    pub epoch: u64,
    /// Resource cap in raw worlds. Reaching it produces `Unresolved` — a
    /// cap is a resource limit, never a settlement rule (CE-A3/A5).
    pub world_cap: u64,
    /// Throughput batch size (§17.3): affects work grouping only. It must
    /// not change which world is world `i`, the pair counts at any index,
    /// the first crossing, or the result — gate V8.
    pub batch: u64,
}

/// One stream step, for audit and the V8 invariance gate: the world's
/// canonical identity, the signed outcome, and the running pivotal counts
/// after folding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub index: u64,
    pub world_id: [u32; 4],
    pub y: i8,
    pub a: u64,
    pub b: u64,
}

/// The evaluator's answer plus its per-index trace (a derived audit log —
/// the result is a function of the trace, not a second authority).
pub struct PairEvaluation {
    pub result: ResultKind,
    pub trace: Vec<StepRecord>,
}

/// Evaluate a frozen pair on the common indexed world stream until the
/// pivotal evidence crosses the calculated threshold (`DeltaSettled`) or
/// the resource cap arrives (`Unresolved`). The threshold is
/// `T_edge = m(m-1)/δ` with `m = 2` — calculated from the declared risk
/// budget, never tuned (§3.2, §5).
pub fn evaluate_pair(spec: &PairSpec<'_>) -> PairEvaluation {
    assert!(spec.batch >= 1, "a batch holds at least one world");
    let viewer = spec.root.kernel().viewer();
    let root_id = root_identity(spec.root, spec.position);
    let stream = StreamIdentity {
        sampler: SAMPLER_ID,
        root_id,
        epoch: spec.epoch,
        with_replacement: true,
        fiber: spec.root.count(),
    };
    let threshold = evidence::edge_threshold(2, spec.delta.delta());
    let mut a = 0u64;
    let mut b = 0u64;
    let mut trace: Vec<StepRecord> = Vec::new();
    let mut index = 0u64;
    while index < spec.world_cap {
        let hi = (index + spec.batch).min(spec.world_cap);
        // Batch outcomes are computed together for throughput, then folded
        // in stream order: a batch may overshoot the first crossing, but
        // the reported settlement index is the first crossing (§17.3).
        let outcomes: Vec<(u64, [u32; 4], bool, bool)> = (index..hi)
            .map(|i| {
                let world = spec.root.world_at(root_id, spec.epoch, i);
                let u_a =
                    replay_viewer_success(spec.position, viewer, &world, spec.policy_a, spec.field);
                let u_b =
                    replay_viewer_success(spec.position, viewer, &world, spec.policy_b, spec.field);
                (i, world_id(&world), u_a, u_b)
            })
            .collect();
        for (i, wid, u_a, u_b) in outcomes {
            let y: i8 = match (u_a, u_b) {
                (true, false) => 1,
                (false, true) => -1,
                _ => 0,
            };
            if y > 0 {
                a += 1;
            } else if y < 0 {
                b += 1;
            }
            trace.push(StepRecord {
                index: i,
                world_id: wid,
                y,
                a,
                b,
            });
            // A nonpivotal world leaves the evidence unchanged and cannot
            // newly cross (§4: it costs time, it creates no evidence).
            if y != 0 {
                let winner = if evidence::crossed(&evidence::pivotal_evidence(a, b), &threshold) {
                    Some(spec.policy_a.id())
                } else if evidence::crossed(&evidence::pivotal_evidence(b, a), &threshold) {
                    Some(spec.policy_b.id())
                } else {
                    None
                };
                if let Some(winner) = winner {
                    return PairEvaluation {
                        result: ResultKind::DeltaSettled {
                            winner: winner.to_string(),
                            settled_at: i,
                            a,
                            b,
                            threshold,
                            delta: spec.delta.clone(),
                            stream,
                        },
                        trace,
                    };
                }
            }
        }
        index = hi;
    }
    // The cap arrived first: a successful honest output (§1.5).
    PairEvaluation {
        result: ResultKind::Unresolved {
            consumed: spec.world_cap,
            a,
            b,
            evidence_plus: evidence::pivotal_evidence(a, b),
            evidence_minus: evidence::pivotal_evidence(b, a),
            threshold,
            delta: spec.delta.clone(),
            stream,
        },
        trace,
    }
}

// ---------------------------------------------------------------------------
// The exact frozen-set endpoint (§22 step 6, minimum; §11.4/§11.5).
// ---------------------------------------------------------------------------

/// An exact endpoint's answer plus its §11.4 bookkeeping: how many fiber
/// worlds reused a cached sampled outcome, and how many were evaluated
/// fresh during enumeration.
pub struct ExactPairReport {
    pub result: ResultKind,
    pub reused: u64,
    pub fresh: u64,
}

/// Enumerate the complete fiber and replay both frozen policies on every
/// world, reusing cached outcomes for worlds already seen in a sampled
/// stream. The exact sum counts every physical world exactly once,
/// regardless of how often it appeared among the samples (§11.4; O24).
/// Exact results spend no sampling risk (§6.1).
pub fn exact_frozen_pair(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy_a: &dyn SlicePolicy,
    policy_b: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
    cache: &HashMap<World, (bool, bool)>,
) -> ExactPairReport {
    let viewer = root.kernel().viewer();
    let mut wins_a = 0u128;
    let mut wins_b = 0u128;
    let mut reused = 0u64;
    let mut fresh = 0u64;
    let mut visited = 0u128;
    for world in root.worlds() {
        let (u_a, u_b) = match cache.get(&world) {
            Some(&pair) => {
                reused += 1;
                pair
            }
            None => {
                fresh += 1;
                (
                    replay_viewer_success(position, viewer, &world, policy_a, field),
                    replay_viewer_success(position, viewer, &world, policy_b, field),
                )
            }
        };
        if u_a {
            wins_a += 1;
        }
        if u_b {
            wins_b += 1;
        }
        visited += 1;
    }
    assert_eq!(
        visited,
        root.count(),
        "enumeration visits the whole fiber exactly once"
    );
    let winner = match wins_a.cmp(&wins_b) {
        core::cmp::Ordering::Greater => Some(policy_a.id().to_string()),
        core::cmp::Ordering::Less => Some(policy_b.id().to_string()),
        core::cmp::Ordering::Equal => None,
    };
    ExactPairReport {
        result: ResultKind::ExactFrozenSet {
            policy_a: policy_a.id().to_string(),
            policy_b: policy_b.id().to_string(),
            wins_a,
            wins_b,
            fiber: visited,
            root_id: root_identity(root, position),
            winner,
        },
        reused,
        fresh,
    }
}

/// Sample the stream prefix `0..switch_at`, caching each DISTINCT world's
/// pair outcomes once, then escalate to exact enumeration reusing that
/// cache. Whatever the switch index, the exact endpoint must equal a cold
/// full enumeration — gate V9.
pub fn evaluate_pair_with_switch(spec: &PairSpec<'_>, switch_at: u64) -> ExactPairReport {
    let viewer = spec.root.kernel().viewer();
    let root_id = root_identity(spec.root, spec.position);
    let mut cache: HashMap<World, (bool, bool)> = HashMap::new();
    for i in 0..switch_at {
        let world = spec.root.world_at(root_id, spec.epoch, i);
        cache.entry(world).or_insert_with(|| {
            (
                replay_viewer_success(spec.position, viewer, &world, spec.policy_a, spec.field),
                replay_viewer_success(spec.position, viewer, &world, spec.policy_b, spec.field),
            )
        });
    }
    exact_frozen_pair(
        spec.root,
        spec.position,
        spec.policy_a,
        spec.policy_b,
        spec.field,
        &cache,
    )
}
