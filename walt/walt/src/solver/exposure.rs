//! `solver::exposure` — the coupled pre-split replay, fixed-policy
//! exposure (§21 step 4), and the exposure rung producers (§21 steps 6–7,
//! exact route).
//!
//! EXPLORATORY tier. Implements parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §3.1 (coupled
//! execution), L2-T1 (first-disagreement localization, O30), §3.2 (the
//! fixed-policy correction bound), §6.1/§6.3 (exposure result tiers), the
//! first-split trace fields of §10 the first slice needs, and the §7
//! exposure rungs E0 ([`ClairvoyantReach::e0_upper`]), E1
//! ([`StructuralSplitCover`], [`rung_e1`]), E2
//! ([`ClairvoyantReach::e2_upper`]), and the EXACT split-reach route
//! ([`exact_split_reach`], rung-labeled E4 per §7.5) — under rulings
//! L2-A1..A7 (`walt/CENSUS-RULINGS.md`) and obligations O31/O34 of
//! `walt/SCENARIO-PLAYER.md` §10.
//!
//! Exposure-tier typing is BINDING (L2-A4, O31): [`FrozenPolicyExposure`]
//! is a fixed-policy observation and supports only fixed-policy statements
//! — `|V_1(ρ) − V_0(ρ)| ≤ d_ρ`. It is mechanically distinct from
//! [`RootActionExposureUpper`], the only tier that may ever feed the
//! L2-T2..T4 screen; a sampled fixed-policy exposure is NEVER an upper
//! bound on omitted continuations (acceptance item 8), and this module
//! offers no conversion between the tiers. Every root-action bound names
//! its derivation rung ([`ExposureRung`]).
//!
//! The one load-bearing lock of the rung producers (§7.4, O34): a sampled
//! LOWER witness to `R_a` is never an upper bound. Every producer in this
//! module is exhaustive over its declared domain — the reach walks
//! enumerate the complete fiber and every focal branching, the covers
//! count the complete fiber — so each bound is a proved over-approximation
//! of `sup_ρ Pr(D_ρ = 1)`, never an observation of some policies'
//! exposure. The sampled/adaptive E3 producer is deliberately NOT built in
//! this slice; the exact route ([`exact_split_reach`]) is the only
//! split-reach solve, and its rung label (E4) keeps it mechanically
//! distinguishable from any future sampled variant.

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::kernel::{Kernel, World};
use crate::rules::rules::{legal_plays, Trick};
use crate::rules::{Domino, DominoSet, Seat};
use crate::solver::adaptive::{
    root_identity, world_id, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::policy::{FrozenPolicy, PolicyId};

// ---------------------------------------------------------------------------
// Exposure result tiers (parent §6; L2-A4).
// ---------------------------------------------------------------------------

/// The derivation rung of a root-action exposure upper bound (parent §7).
/// No rung may be silently promoted to a stronger one; every
/// [`RootActionExposureUpper`] names its rung. This slice's producers:
/// E0/E2 from the shared pre-split reach walk ([`clairvoyant_reach`]), E1
/// from counted structural covers ([`rung_e1`]), E4 from the exact
/// split-reach solve ([`exact_split_reach`]). The sampled/adaptive E3
/// producer is a later slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExposureRung {
    /// §7.1 — exact field equality on the dependency-closed reachable
    /// domain.
    E0,
    /// §7.2 — structural split cover (counted boundary).
    E1,
    /// §7.3 — clairvoyant split-reach cover (safe-direction strategy
    /// fusion; an upper bound, never a playable policy).
    E2,
    /// §7.4 — information-consistent split-reach upper bound.
    E3,
    /// §7.5 — exact split-reach value (dependency closure).
    E4,
}

impl fmt::Display for ExposureRung {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExposureRung::E0 => write!(f, "E0"),
            ExposureRung::E1 => write!(f, "E1"),
            ExposureRung::E2 => write!(f, "E2"),
            ExposureRung::E3 => write!(f, "E3"),
            ExposureRung::E4 => write!(f, "E4"),
        }
    }
}

/// Parent §6.3 — a valid upper bound on `R_a = sup_ρ Pr(D_ρ = 1)` over
/// ALL information-consistent continuations in `Π_a`. The ONLY exposure
/// tier the L2-T2..T4 screen may consume (L2-A4, O31), and it always
/// names its derivation rung. This slice builds no producer — the type
/// exists now so the tier separation is mechanical before any screen
/// exists (acceptance item 7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootActionExposureUpper {
    rung: ExposureRung,
    upper: BigRational,
}

impl RootActionExposureUpper {
    /// A rung-named upper bound in `[0, 1]`. Callers are the E0–E4 rung
    /// implementations of later slices; a sampled fixed-policy exposure is
    /// never a lawful input here (acceptance item 8).
    pub fn from_rung(rung: ExposureRung, upper: BigRational) -> RootActionExposureUpper {
        assert!(
            upper >= BigRational::from_integer(BigInt::from(0))
                && upper <= BigRational::from_integer(BigInt::from(1)),
            "an exposure probability bound lies in [0, 1]"
        );
        RootActionExposureUpper { rung, upper }
    }

    pub fn rung(&self) -> ExposureRung {
        self.rung
    }

    /// The screen's one entry point: the L2-T2..T4 interval arithmetic
    /// consumes bounds only through this accessor, which exists on no
    /// other exposure type (L2-A4).
    pub fn screenable_upper(&self) -> &BigRational {
        &self.upper
    }
}

impl fmt::Display for RootActionExposureUpper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RootActionExposureUpper{{rung={};upper={}}}",
            self.rung, self.upper
        )
    }
}

// ---------------------------------------------------------------------------
// The coupled pre-split replay (parent §3.1; O30).
// ---------------------------------------------------------------------------

/// The first field split of one coupled execution (the §10 trace fields
/// this slice needs): where the two field models first chose different
/// tiles, at which seat and trick/ply, under which information state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstSplit {
    /// The acting non-focal seat.
    pub seat: Seat,
    /// 1-based trick number of the split play (counting the whole hand).
    pub trick: usize,
    /// 0-based ply within that trick.
    pub ply: usize,
    /// σ0's chosen tile.
    pub tile0: Domino,
    /// σ1's chosen tile.
    pub tile1: Domino,
    /// The acting seat's private hand at the split.
    pub hand: DominoSet,
    /// The common public record since the root at the split (with the
    /// root frame, the complete modeled information-state key).
    pub history: Vec<Domino>,
}

/// One coupled execution's outcome: the terminal make indicators under
/// each field and the first split if one occurred. `D_ρ(ω) = 1` exactly
/// when `split` is present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoupledOutcome {
    pub split: Option<FirstSplit>,
    /// Terminal make indicator under (ρ, σ0).
    pub u0: bool,
    /// Terminal make indicator under (ρ, σ1).
    pub u1: bool,
}

impl CoupledOutcome {
    /// The field-exposure event `D_ρ(ω)`.
    pub fn exposed(&self) -> bool {
        self.split.is_some()
    }

    /// The policy correction variable `C_ρ(ω) = u1 − u0 ∈ {−1, 0, +1}`.
    pub fn correction(&self) -> i8 {
        i8::from(self.u1) - i8::from(self.u0)
    }
}

/// One execution's state, evolved from the root by legal plays. All public
/// fields are derived from (root position, world, plays so far).
struct Exec {
    hands: [DominoSet; 4],
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    history: Vec<Domino>,
}

impl Exec {
    fn start(position: &RootPosition, world: &World) -> Exec {
        Exec {
            hands: world.hands(),
            leader: position.leader,
            plays: position.trick_plays.clone(),
            banked: position.banked,
            history: Vec::new(),
        }
    }

    fn done(&self) -> bool {
        self.hands.iter().all(|h| h.is_empty())
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }

    /// Apply one legal play (the `replay_viewer_success` step, verbatim
    /// semantics).
    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.hands[seat.index()].remove(tile),
            "the chosen tile is held"
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

    /// The viewer-objective terminal make indicator (make for the
    /// declaring side, set for the other).
    fn success(&self, position: &RootPosition, viewer: Seat) -> bool {
        assert!(self.done() && self.plays.is_empty(), "a terminal state");
        let made = self.banked[position.declaring_team.index()] >= position.bid;
        if viewer.team() == position.declaring_team {
            made
        } else {
            !made
        }
    }

    /// The public view compared across the coupled pair before the first
    /// split (acceptance item 5).
    fn public_view(&self) -> (Seat, &[Domino], [u32; 2], &[Domino]) {
        (self.leader, &self.plays, self.banked, &self.history)
    }
}

/// 1-based trick and 0-based ply of the NEXT play, from the root frame and
/// the post-root history length — derived views, never stored.
fn trick_and_ply(position: &RootPosition, history_len: usize) -> (usize, usize) {
    let total = position.prior_played.len() + position.trick_plays.len() + history_len;
    (total / 4 + 1, total % 4)
}

/// Drive one execution to terminal: the viewer plays `focal`, every other
/// seat plays `field`.
fn run_to_terminal(
    exec: &mut Exec,
    position: &RootPosition,
    viewer: Seat,
    focal: &dyn SlicePolicy,
    field: &FieldModel,
) {
    while !exec.done() {
        let seat = exec.seat();
        let led = exec.plays.first().map(|d| position.decl.led_context(*d));
        let hand = exec.hands[seat.index()];
        let legal = legal_plays(position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let record = exec.record(position);
        let tile = if seat == viewer {
            focal.choose(position.decl, hand, legal, &record)
        } else {
            field.choose(position.decl, hand, legal, &record)
        };
        assert!(legal.contains(tile), "a policy chooses a legal tile");
        exec.play(position, tile);
    }
}

/// Parent §3.1 — the coupled execution: for one fixed (ρ, ω), run
/// execution 0 under (ρ, σ0) and execution 1 under (ρ, σ1) from the same
/// root, asserting public-history equality before the first field split
/// (acceptance item 5) and focal information consistency at every focal
/// step. At the first non-focal information state where the fields choose
/// different tiles, the split is recorded and both branches run to
/// terminal under their own field. Returns (D, u0, u1) with the first
/// split.
///
/// L2-T1 (O30) is asserted, not assumed: when no split occurs, the two
/// terminals are checked equal, so no correction can ever be attributed to
/// an unsplit world.
pub fn coupled_replay(
    position: &RootPosition,
    viewer: Seat,
    world: &World,
    focal: &dyn SlicePolicy,
    field0: &FieldModel,
    field1: &FieldModel,
) -> CoupledOutcome {
    let mut e0 = Exec::start(position, world);
    let mut e1 = Exec::start(position, world);
    while !e0.done() {
        // Acceptance item 5: before the first field split, the two public
        // histories are equal — asserted at every step, not assumed.
        assert_eq!(
            e0.public_view(),
            e1.public_view(),
            "coupled public histories agree before the first field split"
        );
        let seat = e0.seat();
        let led = e0.plays.first().map(|d| position.decl.led_context(*d));
        let hand = e0.hands[seat.index()];
        assert_eq!(hand, e1.hands[seat.index()], "coupled hands agree");
        let legal = legal_plays(position.decl, hand, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        if seat == viewer {
            let a0 = focal.choose(position.decl, hand, legal, &e0.record(position));
            let a1 = focal.choose(position.decl, hand, legal, &e1.record(position));
            assert_eq!(
                a0, a1,
                "an information-consistent focal policy chooses the same action \
                 on equal public histories (parent §3.1)"
            );
            e0.play(position, a0);
            e1.play(position, a1);
        } else {
            let t0 = field0.choose(position.decl, hand, legal, &e0.record(position));
            let t1 = field1.choose(position.decl, hand, legal, &e1.record(position));
            if t0 == t1 {
                e0.play(position, t0);
                e1.play(position, t1);
            } else {
                // The first state in the disagreement frontier F_{0,1}.
                let (trick, ply) = trick_and_ply(position, e0.history.len());
                let split = FirstSplit {
                    seat,
                    trick,
                    ply,
                    tile0: t0,
                    tile1: t1,
                    hand,
                    history: e0.history.clone(),
                };
                e0.play(position, t0);
                e1.play(position, t1);
                run_to_terminal(&mut e0, position, viewer, focal, field0);
                run_to_terminal(&mut e1, position, viewer, focal, field1);
                let u0 = e0.success(position, viewer);
                let u1 = e1.success(position, viewer);
                return CoupledOutcome {
                    split: Some(split),
                    u0,
                    u1,
                };
            }
        }
    }
    let u0 = e0.success(position, viewer);
    let u1 = e1.success(position, viewer);
    // L2-T1, asserted (O30): an unsplit world has equal terminals.
    assert_eq!(
        u0, u1,
        "L2-T1: without a field split the coupled terminals are equal"
    );
    CoupledOutcome {
        split: None,
        u0,
        u1,
    }
}

// ---------------------------------------------------------------------------
// FrozenPolicyExposure over a declared world set (parent §6.1).
// ---------------------------------------------------------------------------

/// The declared world set an exposure result ranges over. The domain is
/// part of the result — an exact-fiber exposure and a streamed-prefix
/// exposure are different claims.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldDomain {
    /// Exact lazy enumeration of the complete fiber: `d_ρ` is the exact
    /// frontier-reaching fraction under the uniform fiber measure.
    ExactFiber,
    /// The indexed evidence-stream prefix `0..worlds` at `epoch` (the
    /// kernel's exactly-uniform with-replacement sampler): `d̂_ρ` is exact
    /// over the enumerated stream worlds, an estimate of `d_ρ`.
    StreamPrefix { epoch: u64, worlds: u64 },
}

impl fmt::Display for WorldDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorldDomain::ExactFiber => write!(f, "exact-fiber"),
            WorldDomain::StreamPrefix { epoch, worlds } => {
                write!(f, "stream-prefix{{epoch={epoch};worlds={worlds}}}")
            }
        }
    }
}

/// One world's coupled outcome inside an exposure result: the audit rows
/// the aggregates are a function of (derived views, never a second
/// authority).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldRow {
    /// Enumeration or stream index.
    pub index: u64,
    /// Canonical world identity (four hand bitmasks, seat-indexed).
    pub world: [u32; 4],
    pub u0: bool,
    pub u1: bool,
    pub split: Option<FirstSplit>,
}

/// Parent §6.1 — the fixed-policy exposure of ONE named frozen policy ρ
/// over a declared world set: the exposure tally `d̂_ρ`, the complete
/// signed correction tallies `C_ρ` (never sign frequency alone, acceptance
/// item 14), and the per-world first-split rows.
///
/// TIER CONTRACT (L2-A4, O31): this type supports fixed-policy statements
/// only — `|V_1(ρ) − V_0(ρ)| ≤ d_ρ` on the declared domain. It does not
/// account for omitted continuations, offers no
/// `screenable_upper` accessor, and cannot be converted into a
/// [`RootActionExposureUpper`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrozenPolicyExposure {
    /// The one named frozen focal policy.
    pub policy: PolicyId,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The declared world set.
    pub domain: WorldDomain,
    /// Worlds evaluated (`|domain|`).
    pub worlds: u64,
    /// Worlds whose coupled execution reached the frontier (`D_ρ = 1`).
    pub exposed: u64,
    /// Worlds with `C_ρ = +1` (the field upgrade turned fail into make).
    pub corrections_plus: u64,
    /// Worlds with `C_ρ = −1`.
    pub corrections_minus: u64,
    /// The per-world audit rows the tallies are computed from.
    pub rows: Vec<WorldRow>,
}

impl FrozenPolicyExposure {
    /// The exact exposure fraction `d̂_ρ` over the declared world set.
    pub fn d_hat(&self) -> BigRational {
        BigRational::new(BigInt::from(self.exposed), BigInt::from(self.worlds))
    }

    /// The exact mean correction `ĉ_ρ` over the declared world set (a
    /// complete signed mean, parent §3.2).
    pub fn c_hat(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.corrections_plus) - BigInt::from(self.corrections_minus),
            BigInt::from(self.worlds),
        )
    }

    /// The exact mean absolute correction `E[|C_ρ|]`.
    pub fn c_abs_hat(&self) -> BigRational {
        BigRational::new(
            BigInt::from(self.corrections_plus) + BigInt::from(self.corrections_minus),
            BigInt::from(self.worlds),
        )
    }
}

impl fmt::Display for FrozenPolicyExposure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FrozenPolicyExposure{{policy={};field0={};field1={};root={:#018x};domain={};\
             worlds={};exposed={};c_plus={};c_minus={};d_hat={};c_hat={}}}",
            self.policy,
            self.field0,
            self.field1,
            self.root_id,
            self.domain,
            self.worlds,
            self.exposed,
            self.corrections_plus,
            self.corrections_minus,
            self.d_hat(),
            self.c_hat()
        )
    }
}

/// Compute the fixed-policy exposure of one frozen focal policy over a
/// declared world set: coupled replay on every world, the L2-T1 pointwise
/// bound `|u1 − u0| ≤ D` asserted on each (O30), and the §3.2 aggregate
/// bound `|ĉ_ρ| ≤ E[|C_ρ|] ≤ d̂_ρ` asserted on the result.
pub fn frozen_policy_exposure(
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: &FrozenPolicy,
    field0: &FieldModel,
    field1: &FieldModel,
    domain: WorldDomain,
) -> FrozenPolicyExposure {
    let viewer = root.kernel().viewer();
    let root_id = root_identity(root, position);
    let indexed_worlds: Vec<(u64, World)> = match &domain {
        WorldDomain::ExactFiber => root
            .worlds()
            .enumerate()
            .map(|(i, w)| (u64::try_from(i).expect("an enumerable fiber fits u64"), w))
            .collect(),
        WorldDomain::StreamPrefix { epoch, worlds } => (0..*worlds)
            .map(|i| (i, root.world_at(root_id, *epoch, i)))
            .collect(),
    };
    if let WorldDomain::ExactFiber = domain {
        assert_eq!(
            u128::try_from(indexed_worlds.len()).expect("fits"),
            root.count(),
            "exact enumeration visits the whole fiber exactly once"
        );
    }
    let mut rows: Vec<WorldRow> = Vec::with_capacity(indexed_worlds.len());
    let mut exposed = 0u64;
    let mut corrections_plus = 0u64;
    let mut corrections_minus = 0u64;
    for (index, world) in &indexed_worlds {
        let outcome = coupled_replay(position, viewer, world, focal, field0, field1);
        // L2-T1 pointwise on every world (O30): |u1 − u0| ≤ D.
        assert!(
            outcome.correction() == 0 || outcome.exposed(),
            "L2-T1: a nonzero correction occurs only on an exposed world"
        );
        if outcome.exposed() {
            exposed += 1;
        }
        match outcome.correction() {
            1 => corrections_plus += 1,
            -1 => corrections_minus += 1,
            _ => {}
        }
        rows.push(WorldRow {
            index: *index,
            world: world_id(world),
            u0: outcome.u0,
            u1: outcome.u1,
            split: outcome.split,
        });
    }
    let result = FrozenPolicyExposure {
        policy: focal.policy_id(),
        field0: field0.field_id(),
        field1: field1.field_id(),
        root_id,
        domain,
        worlds: u64::try_from(rows.len()).expect("a declared world set fits u64"),
        exposed,
        corrections_plus,
        corrections_minus,
        rows,
    };
    // §3.2: |ĉ_ρ| ≤ E[|C_ρ|] ≤ d̂_ρ, exactly.
    let c_hat_abs = {
        let c = result.c_hat();
        if c < BigRational::from_integer(BigInt::from(0)) {
            -c
        } else {
            c
        }
    };
    assert!(
        c_hat_abs <= result.c_abs_hat() && result.c_abs_hat() <= result.d_hat(),
        "the fixed-policy correction bound |c| <= E[|C|] <= d holds exactly"
    );
    result
}

// ---------------------------------------------------------------------------
// The shared pre-split reach walk — the rung E0/E2/E4 skeleton (parent
// §7.1, §7.3, §7.5).
// ---------------------------------------------------------------------------

/// The public half of one execution: leader, current trick, banked points,
/// and the post-root history — shared by every fiber world that has
/// produced the same public record. Per-world remaining hands are derived
/// views of (world, played history), never stored, so one node of the reach
/// walk serves a whole set of worlds at once.
#[derive(Clone)]
struct PublicExec {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    history: Vec<Domino>,
}

impl PublicExec {
    fn start(position: &RootPosition) -> PublicExec {
        PublicExec {
            leader: position.leader,
            plays: position.trick_plays.clone(),
            banked: position.banked,
            history: Vec::new(),
        }
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }

    /// Every tile played since the root — a seat's remaining hand is its
    /// root hand minus this set.
    fn played_since(&self) -> DominoSet {
        self.history.iter().copied().collect()
    }

    /// Apply one legal play (the same trick arithmetic as the coupled
    /// replay's `Exec::play`, minus the per-world hands).
    fn play(&mut self, position: &RootPosition, tile: Domino) {
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

/// The immutable context of one pre-split reach walk after a fixed root
/// action: the walk branches FREELY at focal states (a superset of every
/// information-consistent continuation — the safe direction, §7.3) and
/// follows the COMMON field action at agreeing non-focal states; a world
/// leaves the walk at its first field disagreement.
struct ReachWalk<'a> {
    position: &'a RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    worlds: &'a [World],
    field0: &'a FieldModel,
    field1: &'a FieldModel,
    /// Post-root plays to terminal: the viewer's hand size plus the hidden
    /// capacities.
    total: usize,
}

impl ReachWalk<'_> {
    fn setup<'a>(
        root: &'a CanonicalRoot,
        position: &'a RootPosition,
        worlds: &'a [World],
        field0: &'a FieldModel,
        field1: &'a FieldModel,
    ) -> ReachWalk<'a> {
        let kernel = root.kernel();
        let total =
            kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
        assert!(
            u128::try_from(worlds.len()).expect("fits") == root.count(),
            "the reach walk enumerates the complete fiber"
        );
        assert!(
            u32::try_from(worlds.len()).is_ok(),
            "an enumerable reach-walk fiber fits u32 indices"
        );
        ReachWalk {
            position,
            viewer: kernel.viewer(),
            viewer_hand: kernel.viewer_hand(),
            worlds,
            field0,
            field1,
            total,
        }
    }

    /// The exec after the fixed root action: the seat to move at the root
    /// must be the viewer, and the action must be legal there.
    fn root_exec(&self, action: Domino) -> PublicExec {
        let mut exec = PublicExec::start(self.position);
        assert_eq!(
            exec.seat(),
            self.viewer,
            "the root decision is the viewer's"
        );
        let led = exec
            .plays
            .first()
            .map(|d| self.position.decl.led_context(*d));
        let legal = legal_plays(self.position.decl, self.viewer_hand, led);
        assert!(legal.contains(action), "a root action is legal at the root");
        exec.play(self.position, action);
        exec
    }

    /// Group the still-live worlds at one non-focal node: each world's
    /// acting hand is queried against BOTH fields; a disagreeing world is
    /// reported through `split`, an agreeing world joins the group of its
    /// common action.
    fn partition(
        &self,
        exec: &PublicExec,
        idxs: &[u32],
        mut split: impl FnMut(u32),
    ) -> Vec<(Domino, Vec<u32>)> {
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
            let (t0, t1) = (
                self.field0.choose(self.position.decl, hand, legal, &record),
                self.field1.choose(self.position.decl, hand, legal, &record),
            );
            if t0 == t1 {
                match groups.iter_mut().find(|(tile, _)| *tile == t0) {
                    Some((_, group)) => group.push(i),
                    None => groups.push((t0, vec![i])),
                }
            } else {
                // A state of the disagreement frontier F_{0,1}: the world
                // leaves the pre-split walk here.
                split(i);
            }
        }
        groups
    }

    /// Union mode (rungs E0/E2): mark every world for which SOME legal
    /// focal continuation reaches the frontier. Free focal branching is a
    /// superset of every information-consistent ρ ∈ Π_a, so an unmarked
    /// world satisfies D_ρ(ω) = 0 for every ρ — and zero marks anywhere
    /// proves σ0 = σ1 on the whole dependency-closed reachable domain.
    fn mark(&self, exec: &PublicExec, idxs: &[u32], reached: &mut [bool]) {
        let live: Vec<u32> = idxs
            .iter()
            .copied()
            .filter(|&i| !reached[usize::try_from(i).expect("fits")])
            .collect();
        if live.is_empty() || exec.history.len() == self.total {
            return;
        }
        if exec.seat() == self.viewer {
            let led = exec
                .plays
                .first()
                .map(|d| self.position.decl.led_context(*d));
            let hand = self.viewer_hand.difference(exec.played_since());
            let legal = legal_plays(self.position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            for tile in legal.iter() {
                let mut child = exec.clone();
                child.play(self.position, tile);
                self.mark(&child, &live, reached);
            }
        } else {
            let groups = self.partition(exec, &live, |i| {
                reached[usize::try_from(i).expect("fits")] = true;
            });
            for (tile, group) in groups {
                let mut child = exec.clone();
                child.play(self.position, tile);
                self.mark(&child, &group, reached);
            }
        }
    }

    /// Max mode (rung E4): the §7.4 split-reach objective solved exactly.
    /// Every node of this tree is one distinct public history; worlds
    /// compatible with the same public history present the SAME focal
    /// information state, so choosing one action per node — and taking the
    /// per-node maximum — ranges over exactly the deterministic
    /// information-consistent continuations in Π_a, with no strategy
    /// fusion (O34): one action serves every compatible world at once.
    fn max_count(&self, exec: &PublicExec, idxs: &[u32]) -> u64 {
        if idxs.is_empty() || exec.history.len() == self.total {
            return 0;
        }
        if exec.seat() == self.viewer {
            let led = exec
                .plays
                .first()
                .map(|d| self.position.decl.led_context(*d));
            let hand = self.viewer_hand.difference(exec.played_since());
            let legal = legal_plays(self.position.decl, hand, led);
            assert!(!legal.is_empty(), "a seat to move holds a legal tile");
            legal
                .iter()
                .map(|tile| {
                    let mut child = exec.clone();
                    child.play(self.position, tile);
                    self.max_count(&child, idxs)
                })
                .max()
                .expect("a nonempty legal set")
        } else {
            let mut split = 0u64;
            let groups = self.partition(exec, idxs, |_| split += 1);
            split
                + groups
                    .into_iter()
                    .map(|(tile, group)| {
                        let mut child = exec.clone();
                        child.play(self.position, tile);
                        self.max_count(&child, &group)
                    })
                    .sum::<u64>()
        }
    }
}

// ---------------------------------------------------------------------------
// Rungs E0 and E2 — exact field equality and the clairvoyant split-reach
// cover (parent §7.1, §7.3).
// ---------------------------------------------------------------------------

/// The clairvoyant split-reach cover of one root action over the complete
/// enumerated fiber (§7.3): per world, whether ANY legal focal continuation
/// (full knowledge of that world — deliberate safe-direction strategy
/// fusion) can reach the disagreement frontier under the shared pre-split
/// field. An upper bound on exposure, never a playable policy or a lower
/// witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClairvoyantReach {
    /// The fixed root action `a`.
    pub action: Domino,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// Exact `|Φ(I)|`.
    pub fiber: u128,
    /// Worlds where some focal continuation reaches the frontier.
    pub reach_worlds: u64,
}

impl ClairvoyantReach {
    /// The exact cover mass `Pr(P_a^PI = 1)` under the uniform fiber
    /// measure.
    pub fn mass(&self) -> BigRational {
        BigRational::new(BigInt::from(self.reach_worlds), BigInt::from(self.fiber))
    }

    /// §7.3 — the rung-E2 bound: `R_a ≤ Pr(P_a^PI = 1)`.
    pub fn e2_upper(&self) -> RootActionExposureUpper {
        RootActionExposureUpper::from_rung(ExposureRung::E2, self.mass())
    }

    /// §7.1 — rung E0 fires exactly when the walk found NO reachable
    /// disagreement: the two fields chose the same action at every
    /// non-focal information state of the dependency-closed reachable
    /// domain (free focal branching over-approximates every ρ ∈ Π_a), so
    /// `R_a = 0` exactly. `None` when a disagreement is reachable — E0
    /// makes no claim then.
    pub fn e0_upper(&self) -> Option<RootActionExposureUpper> {
        (self.reach_worlds == 0).then(|| {
            RootActionExposureUpper::from_rung(
                ExposureRung::E0,
                BigRational::from_integer(BigInt::from(0)),
            )
        })
    }
}

/// Run the pre-split reach walk for one root action over the complete
/// fiber (rungs E0/E2). Exhaustive by construction: every fiber world,
/// every focal branching, every agreeing common continuation.
pub fn clairvoyant_reach(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field0: &FieldModel,
    field1: &FieldModel,
) -> ClairvoyantReach {
    let worlds: Vec<World> = root.worlds().collect();
    let walk = ReachWalk::setup(root, position, &worlds, field0, field1);
    let exec = walk.root_exec(action);
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let mut reached = vec![false; worlds.len()];
    walk.mark(&exec, &idxs, &mut reached);
    ClairvoyantReach {
        action,
        field0: field0.field_id(),
        field1: field1.field_id(),
        root_id: root_identity(root, position),
        fiber: root.count(),
        reach_worlds: u64::try_from(reached.iter().filter(|r| **r).count()).expect("fits"),
    }
}

// ---------------------------------------------------------------------------
// Rung E1 — structural split covers (parent §7.2).
// ---------------------------------------------------------------------------

/// §7.2 — a structural world predicate `P_a(ω)` carrying the containment
/// obligation `D_ρ(ω) = 1 ⇒ P_a(ω) = 1` for EVERY information-consistent
/// ρ ∈ Π_a. Implementations state their soundness argument in their docs;
/// looseness costs pruning power, never correctness.
pub trait StructuralSplitCover {
    fn id(&self) -> &'static str;
    fn covers(&self, kernel: &Kernel, world: &World) -> bool;
}

/// `P ≡ 1`: always sound, never prunes — the §8.1 degenerate bound the
/// screen lawfully starts from.
pub struct TrivialSplitCover;

impl StructuralSplitCover for TrivialSplitCover {
    fn id(&self) -> &'static str {
        "trivial-cover-v1"
    }

    fn covers(&self, _kernel: &Kernel, _world: &World) -> bool {
        true
    }
}

/// `P ≡ 0` exactly when every hidden seat's remaining capacity is at most
/// one tile. Soundness: a non-focal seat holding at most one tile is
/// forced at every future decision, a deterministic field must choose the
/// forced tile, so both fields agree at every reachable non-focal state
/// and no continuation of any ρ ∈ Π_a can reach `F_{0,1}` — `D_ρ(ω) = 0`
/// for every ρ and every ω. A kernel-shape predicate, constant across
/// worlds.
pub struct ForcedNonFocalCover;

impl StructuralSplitCover for ForcedNonFocalCover {
    fn id(&self) -> &'static str {
        "forced-non-focal-cover-v1"
    }

    fn covers(&self, kernel: &Kernel, _world: &World) -> bool {
        !kernel.hidden().iter().all(|h| h.capacity <= 1)
    }
}

/// §7.2 — the counted-boundary route: the exact fiber mass of a
/// structural split cover, as a rung-E1 bound. Valid for every root
/// action whose cover the predicate is (the two covers above are
/// action-independent, so their mass bounds every legal `a`).
pub fn rung_e1(root: &CanonicalRoot, cover: &dyn StructuralSplitCover) -> RootActionExposureUpper {
    let kernel = root.kernel();
    let mut covered = 0u64;
    let mut visited = 0u128;
    for world in root.worlds() {
        if cover.covers(kernel, &world) {
            covered += 1;
        }
        visited += 1;
    }
    assert_eq!(
        visited,
        root.count(),
        "a counted boundary enumerates the whole fiber exactly once"
    );
    RootActionExposureUpper::from_rung(
        ExposureRung::E1,
        BigRational::new(BigInt::from(covered), BigInt::from(visited)),
    )
}

// ---------------------------------------------------------------------------
// The exact split-reach solve (parent §7.4 objective, §7.5 exact route —
// rung E4).
// ---------------------------------------------------------------------------

/// The §7.4 split-reach control objective solved EXACTLY over the complete
/// fiber (§7.5): the optimal value of the Boolean hit-frontier payoff over
/// the pre-split single-field game, ranging over all deterministic
/// information-consistent continuations in Π_a. The exact optimal value IS
/// `R_a`. Mechanically distinct from any sampled E3 variant (none exists
/// in this slice) by its rung label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitReachExact {
    /// The fixed root action `a`.
    pub action: Domino,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// Exact `|Φ(I)|`.
    pub fiber: u128,
    /// `max_ρ #{ω : D_ρ(ω) = 1}` — the exact optimum of the split-reach
    /// objective.
    pub frontier_worlds: u64,
}

impl SplitReachExact {
    /// The exact `R_a = sup_ρ Pr(D_ρ = 1)` under the uniform fiber
    /// measure.
    pub fn r(&self) -> BigRational {
        BigRational::new(BigInt::from(self.frontier_worlds), BigInt::from(self.fiber))
    }

    /// §7.5 — the exact value as a rung-E4 screening bound (an exact value
    /// is in particular a valid upper bound).
    pub fn e4_upper(&self) -> RootActionExposureUpper {
        RootActionExposureUpper::from_rung(ExposureRung::E4, self.r())
    }
}

/// Solve the split-reach objective exactly for one root action (rung E4).
///
/// The walk tree branches exactly on public histories; every fiber world
/// compatible with a node's public history presents the same focal
/// information state there, so the per-node action choice (maximized
/// per node) ranges over exactly the deterministic information-consistent
/// continuations — no strategy fusion (O34). Worlds proved incapable of
/// reaching the frontier by the clairvoyant walk are dropped first: their
/// `D_ρ(ω)` is 0 under every ρ, so the optimum is unchanged and the tree
/// shrinks.
pub fn exact_split_reach(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field0: &FieldModel,
    field1: &FieldModel,
) -> SplitReachExact {
    let worlds: Vec<World> = root.worlds().collect();
    let walk = ReachWalk::setup(root, position, &worlds, field0, field1);
    let exec = walk.root_exec(action);
    let idxs: Vec<u32> = (0..u32::try_from(worlds.len()).expect("fits")).collect();
    let mut reached = vec![false; worlds.len()];
    walk.mark(&exec, &idxs, &mut reached);
    let capable: Vec<u32> = idxs
        .iter()
        .copied()
        .filter(|&i| reached[usize::try_from(i).expect("fits")])
        .collect();
    let frontier_worlds = walk.max_count(&exec, &capable);
    assert!(
        frontier_worlds <= u64::try_from(capable.len()).expect("fits"),
        "the exact optimum is at most the clairvoyant cover count"
    );
    SplitReachExact {
        action,
        field0: field0.field_id(),
        field1: field1.field_id(),
        root_id: root_identity(root, position),
        fiber: root.count(),
        frontier_worlds,
    }
}
