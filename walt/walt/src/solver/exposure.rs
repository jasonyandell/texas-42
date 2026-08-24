//! `solver::exposure` — the coupled pre-split replay and fixed-policy
//! exposure (§21 step 4).
//!
//! EXPLORATORY tier. Implements parent
//! `walt/math/targeted_level2_field_stability_v0.1.md` §3.1 (coupled
//! execution), L2-T1 (first-disagreement localization, O30), §3.2 (the
//! fixed-policy correction bound), §6.1/§6.3 (exposure result tiers), and
//! the first-split trace fields of §10 this slice needs, under rulings
//! L2-A1..A7 (`walt/CENSUS-RULINGS.md`).
//!
//! Exposure-tier typing is BINDING (L2-A4, O31): [`FrozenPolicyExposure`]
//! is a fixed-policy observation and supports only fixed-policy statements
//! — `|V_1(ρ) − V_0(ρ)| ≤ d_ρ`. It is mechanically distinct from
//! [`RootActionExposureUpper`], the only tier that may ever feed the
//! L2-T2..T4 screen; a sampled fixed-policy exposure is NEVER an upper
//! bound on omitted continuations (acceptance item 8), and this module
//! offers no conversion between the tiers. Every root-action bound names
//! its derivation rung ([`ExposureRung`]); the rung producers (E0–E4) are
//! later slices — this slice fixes the typing so nothing narrower can be
//! built against.

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;

use crate::kernel::World;
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
/// [`RootActionExposureUpper`] names its rung. Producers land in later
/// slices (§21 steps 6–7).
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
