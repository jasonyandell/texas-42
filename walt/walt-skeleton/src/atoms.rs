//! Descriptor vocabularies: the S4 transducer registry and the ported
//! exp3A static vocabulary (S4.5).
//!
//! The S4 registry (`Atom`) is walt's own -- per-tile holder facts and a
//! beater-control relation shaped like §12.5's "beater and overtake
//! chains" -- built as genuine transducers: every atom is closed-updatable
//! from observed plays alone, because tile possession is static until the
//! tile is shown: the root evaluation reads the latent world once, and
//! each observed play only ever *removes* latent content (a shown tile's
//! holder question dies; a shown beater leaves its slot's count). The
//! public chassis (viewer hand, leader, current trick prefix) is likewise
//! a fold of the record.
//!
//! The exp3A vocabulary (`Exp3aAtom`, below) is the §14.4 registry itself,
//! ported from the preserved probe suite (`walt/probes/exp3a/`, rescued at
//! commit 9357536 after S4 had recorded it lost) -- see its own doc block.

use core::fmt;

use walt_core::{legal_plays, Context, Decl, Domino, DominoSet, Seat, Trick};
use walt_kernel::{Kernel, World, HIDDEN_SEATS};

use crate::obs::ObservedPlay;
use crate::skeleton::{ControlSkeleton, UpdateKind};

/// One registry atom, named by what it tracks. Identity only -- the
/// semantics live in `AtomDescriptor::eval_atom` / `step_atom`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Atom {
    /// Which hidden slot holds the tile (or that it has been shown).
    HolderOf(Domino),
    /// Which team holds the tile, relative to the viewer's (focal) team.
    TeamOf(Domino),
    /// Exact counts, per hidden slot, of still-unshown tiles that beat this
    /// tile when it is led (its `THREAT` set).
    BeaterCounts(Domino),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::HolderOf(d) => write!(f, "holder({d})"),
            Atom::TeamOf(d) => write!(f, "team({d})"),
            Atom::BeaterCounts(d) => write!(f, "beaters({d})"),
        }
    }
}

/// One atom's typed relational state.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AtomState {
    /// `HolderOf`: hidden slot index, or shown.
    Holder(Option<u8>),
    /// `TeamOf`: `true` when the focal (viewer's) team holds it; `None`
    /// once shown.
    Team(Option<bool>),
    /// `BeaterCounts`: unshown beaters per hidden slot.
    Beaters([u8; HIDDEN_SEATS]),
}

/// The public chassis: what the seat must carry to know its own legal set
/// and the causal position -- §12.5's "current led-context strength" and
/// "actual next actor" substrate. A pure fold of the observation record.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ChassisState {
    /// The viewer's remaining hand (public to the viewer).
    pub hand: DominoSet,
    /// The current trick's leader.
    pub leader: Seat,
    /// The current trick's plays so far (empty at a trick boundary).
    pub prefix: Vec<Domino>,
}

impl ChassisState {
    /// The led context the current trick imposes, if any.
    pub fn led(&self, decl: Decl) -> Option<Context> {
        self.prefix.first().map(|d| decl.led_context(*d))
    }

    /// The viewer's legal set at this state -- the chassis carries exactly
    /// what §10.1 validity needs.
    pub fn legal(&self, decl: Decl) -> DominoSet {
        legal_plays(decl, self.hand, self.led(decl))
    }
}

/// The composite state of an `AtomDescriptor`: optional chassis plus one
/// state per atom, in registry order.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CompositeState {
    pub chassis: Option<ChassisState>,
    pub atoms: Vec<AtomState>,
}

/// A conjunction of atoms over one kernel, with or without the public
/// chassis, as a genuine transducer. Constructed from public kernel data
/// only (declaration, viewer, hidden-slot seats); latent reads happen in
/// `init` alone.
#[derive(Clone, Debug)]
pub struct AtomDescriptor {
    decl: Decl,
    viewer: Seat,
    slots: [Seat; HIDDEN_SEATS],
    chassis: bool,
    atoms: Vec<Atom>,
    /// Per-atom `THREAT` set (for `BeaterCounts`), precomputed.
    threats: Vec<DominoSet>,
}

impl AtomDescriptor {
    /// Builds the descriptor. Every atom's tile must be in the hidden pool:
    /// facts about the viewer's own tiles are public and belong to the
    /// chassis, not the latent vocabulary.
    pub fn new(kernel: &Kernel, chassis: bool, atoms: Vec<Atom>) -> AtomDescriptor {
        let threats = atoms
            .iter()
            .map(|a| {
                let tile = match a {
                    Atom::HolderOf(d) | Atom::TeamOf(d) | Atom::BeaterCounts(d) => *d,
                };
                assert!(
                    kernel.pool().contains(tile),
                    "atom tiles live in the hidden pool"
                );
                match a {
                    Atom::BeaterCounts(d) => kernel.decl().threat(*d),
                    _ => DominoSet::EMPTY,
                }
            })
            .collect();
        AtomDescriptor {
            decl: kernel.decl(),
            viewer: kernel.viewer(),
            slots: core::array::from_fn(|i| kernel.hidden()[i].seat),
            chassis,
            atoms,
            threats,
        }
    }

    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    fn slot_of(&self, seat: Seat) -> Option<u8> {
        self.slots
            .iter()
            .position(|s| *s == seat)
            .map(|i| u8::try_from(i).expect("three slots"))
    }

    fn eval_atom(&self, i: usize, kernel: &Kernel, world: &World) -> AtomState {
        match self.atoms[i] {
            Atom::HolderOf(d) => {
                let slot = self
                    .slots
                    .iter()
                    .position(|s| world.hand(*s).contains(d))
                    .expect("a pool tile sits in some hidden slot");
                AtomState::Holder(Some(u8::try_from(slot).expect("three slots")))
            }
            Atom::TeamOf(d) => {
                let seat = self.slots[self
                    .slots
                    .iter()
                    .position(|s| world.hand(*s).contains(d))
                    .expect("a pool tile sits in some hidden slot")];
                AtomState::Team(Some(seat.team() == kernel.viewer().team()))
            }
            Atom::BeaterCounts(_) => AtomState::Beaters(core::array::from_fn(|s| {
                u8::try_from(
                    world
                        .hand(self.slots[s])
                        .intersection(self.threats[i])
                        .len(),
                )
                .expect("a hand holds at most seven tiles")
            })),
        }
    }

    fn step_atom(&self, i: usize, state: &AtomState, obs: ObservedPlay) -> AtomState {
        match (self.atoms[i], state) {
            (Atom::HolderOf(d), AtomState::Holder(h)) => {
                AtomState::Holder(if obs.tile == d { None } else { *h })
            }
            (Atom::TeamOf(d), AtomState::Team(t)) => {
                AtomState::Team(if obs.tile == d { None } else { *t })
            }
            (Atom::BeaterCounts(_), AtomState::Beaters(counts)) => {
                let mut counts = *counts;
                if self.threats[i].contains(obs.tile) {
                    if let Some(slot) = self.slot_of(obs.seat) {
                        counts[slot as usize] -= 1;
                    }
                }
                AtomState::Beaters(counts)
            }
            _ => unreachable!("atom states travel with their atoms"),
        }
    }
}

impl ControlSkeleton for AtomDescriptor {
    type State = CompositeState;

    fn name(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if self.chassis {
            parts.push("chassis".to_string());
        }
        parts.extend(self.atoms.iter().map(|a| a.to_string()));
        if parts.is_empty() {
            parts.push("trivial".to_string());
        }
        parts.join("+")
    }

    fn kind(&self) -> UpdateKind {
        UpdateKind::Transducer
    }

    fn init(&self, kernel: &Kernel, world: &World) -> CompositeState {
        CompositeState {
            chassis: self.chassis.then(|| ChassisState {
                hand: kernel.viewer_hand(),
                leader: kernel.viewer(),
                prefix: Vec::new(),
            }),
            atoms: (0..self.atoms.len())
                .map(|i| self.eval_atom(i, kernel, world))
                .collect(),
        }
    }

    fn step(&self, d: &CompositeState, obs: ObservedPlay) -> CompositeState {
        let chassis = d.chassis.as_ref().map(|c| {
            let mut c = c.clone();
            if obs.seat == self.viewer {
                c.hand.remove(obs.tile);
            }
            c.prefix.push(obs.tile);
            if c.prefix.len() == 4 {
                let tiles: [Domino; 4] = core::array::from_fn(|k| c.prefix[k]);
                c.leader = Trick::new(c.leader, tiles)
                    .expect("a trick holds distinct tiles")
                    .winner(self.decl);
                c.prefix.clear();
            }
            c
        });
        CompositeState {
            chassis,
            atoms: (0..self.atoms.len())
                .map(|i| self.step_atom(i, &d.atoms[i], obs))
                .collect(),
        }
    }
}

/// The Experiment 3A vocabulary, ported from the preserved probe
/// (`walt/probes/exp3a/lambda_probe_v3.py`, `build_atoms`) -- the registry
/// v0.4 §14.4 searched, whose winning four-atom descriptor is
/// `{comp41, s3max2, team(2:0), team(4:2)}`. The probes are validators,
/// never source: the semantics below are reimplemented from the probe's
/// definitions, and equality is at the *partition* level -- walt's `Rank`
/// is order-isomorphic to the probe's ad-hoc suit ranking (double top,
/// mixed by pip sum), and every atom feeds only equality cells and strict
/// order comparisons, so the induced world-partitions are identical even
/// though the encoded values differ.
///
/// The probe fixes the design kernel's parameters: valued tile 4-1, the
/// decisive context suit 2 (the context the viewer's remaining tile 2-1
/// forces at trick 7), partner seat S3. `Exp3aContext` derives them from
/// any trick-6 kernel: the decisive tile is the viewer tile whose led
/// context touches the most hidden-pool tiles (ties to the higher tile) --
/// walt's generalization choice, recorded here; on the design kernel it
/// selects 2-1 and suit 2 exactly. These atoms are a STATIC vocabulary
/// (root labelings; §14.4 is a static compression result), so the
/// descriptor is marked `StaticPassenger` -- closed update laws for
/// control shapes are S5 work.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Exp3aAtom {
    /// `h(t)`: which hidden slot holds the tile.
    Holder(Domino),
    /// `team(t)`: the focal (viewer's) team holds the tile.
    Team(Domino),
    /// `comp41`: the tile sharing the valued tile's holder's hand.
    Comp,
    /// `comp41_in_suit2`: the companion follows the decisive context.
    CompInContext,
    /// `comp41_is_2-0`: the companion is the context's lowest pool tile.
    CompIsFloor,
    /// `comp41_rank2`: the companion's rank in the decisive context.
    CompRank,
    /// `s3max2`: the partner slot's best decisive-context rank.
    FocalMax,
    /// `oppmax2`: the opponent slots' best decisive-context rank.
    OppMax,
    /// `s3_top2`: the partner strictly out-ranks both opponents there.
    FocalTop,
    /// `nbeat_opp`: opponent-held context tiles above the decisive tile.
    OppBeaters,
    /// `t7w_bestkeep`: the focal team wins the forced context trick when
    /// every hidden seat keeps its best context tile.
    BestKeep,
    /// `41_with_22`: the valued tile sits with the context's pool boss.
    WithBoss,
}

impl fmt::Display for Exp3aAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp3aAtom::Holder(d) => write!(f, "holder({d})"),
            Exp3aAtom::Team(d) => write!(f, "team({d})"),
            Exp3aAtom::Comp => f.write_str("comp"),
            Exp3aAtom::CompInContext => f.write_str("comp-in-context"),
            Exp3aAtom::CompIsFloor => f.write_str("comp-is-floor"),
            Exp3aAtom::CompRank => f.write_str("comp-rank"),
            Exp3aAtom::FocalMax => f.write_str("focal-max"),
            Exp3aAtom::OppMax => f.write_str("opp-max"),
            Exp3aAtom::FocalTop => f.write_str("focal-top"),
            Exp3aAtom::OppBeaters => f.write_str("opp-beaters"),
            Exp3aAtom::BestKeep => f.write_str("bestkeep"),
            Exp3aAtom::WithBoss => f.write_str("with-boss"),
        }
    }
}

/// One exp3A atom's value: a typed relational fact about one root world.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StaticValue {
    Slot(u8),
    Flag(bool),
    Tile(Domino),
    /// A rank inside the decisive context, `None` when the tile is outside
    /// it (the probe's `-1`).
    Rank(Option<u8>),
    Count(u8),
}

/// The public parameters the exp3A vocabulary reads: all derived from the
/// kernel alone (no latent content), computed once.
#[derive(Clone, Debug)]
pub struct Exp3aContext {
    decl: Decl,
    viewer: Seat,
    slots: [Seat; HIDDEN_SEATS],
    valued: Domino,
    /// The viewer tile whose led context the vocabulary reasons about.
    pub decisive: Domino,
    /// That tile's led context.
    pub context: Context,
    /// Hidden-pool tiles that follow the context.
    pub context_pool: DominoSet,
    /// The highest- and lowest-ranked of those, if any.
    pub boss: Option<Domino>,
    pub floor: Option<Domino>,
}

impl Exp3aContext {
    /// Derives the vocabulary parameters. The decisive tile is the viewer
    /// tile whose led context contains the most hidden-pool tiles, ties to
    /// the higher tile -- on the design kernel: 2-1, suit 2.
    pub fn new(kernel: &Kernel, valued: Domino) -> Exp3aContext {
        assert!(kernel.pool().contains(valued), "the valued tile is unseen");
        let decl = kernel.decl();
        let decisive = kernel
            .viewer_hand()
            .iter()
            .max_by_key(|u| {
                let q = decl.led_context(*u);
                (
                    kernel
                        .pool()
                        .intersection(decl.effective_incidence(q))
                        .len(),
                    u.index(),
                )
            })
            .expect("the viewer holds a tile");
        let context = decl.led_context(decisive);
        let context_pool = kernel
            .pool()
            .intersection(decl.effective_incidence(context));
        let by_rank = |d: &Domino| decl.rank(*d);
        Exp3aContext {
            decl,
            viewer: kernel.viewer(),
            slots: core::array::from_fn(|i| kernel.hidden()[i].seat),
            valued,
            decisive,
            context,
            context_pool,
            boss: context_pool.iter().max_by_key(by_rank),
            floor: context_pool.iter().min_by_key(by_rank),
        }
    }

    fn holder_slot(&self, world: &World, t: Domino) -> u8 {
        let i = self
            .slots
            .iter()
            .position(|s| world.hand(*s).contains(t))
            .expect("a pool tile sits in some hidden slot");
        u8::try_from(i).expect("three slots")
    }

    /// The companion: the other tile in the valued tile's holder's hand.
    /// Well-defined exactly at capacity 2 (every trick-6 kernel), asserted.
    fn companion(&self, world: &World) -> Domino {
        let seat = self.slots[self.holder_slot(world, self.valued) as usize];
        let mut rest = world.hand(seat);
        rest.remove(self.valued);
        assert_eq!(rest.len(), 1, "the exp3A companion needs capacity 2");
        rest.iter().next().expect("one tile")
    }

    /// The best context rank in one hidden slot's hand, if any.
    fn slot_max(&self, world: &World, slot: usize) -> Option<u8> {
        world
            .hand(self.slots[slot])
            .intersection(self.context_pool)
            .iter()
            .map(|d| self.decl.rank(d).value())
            .max()
    }

    fn is_focal(&self, seat: Seat) -> bool {
        seat.team() == self.viewer.team()
    }

    fn eval(&self, atom: Exp3aAtom, world: &World) -> StaticValue {
        let focal_slot = (0..HIDDEN_SEATS)
            .find(|&i| self.is_focal(self.slots[i]))
            .expect("one hidden slot is the partner");
        let opp_max = (0..HIDDEN_SEATS)
            .filter(|&i| i != focal_slot)
            .filter_map(|i| self.slot_max(world, i))
            .max();
        match atom {
            Exp3aAtom::Holder(t) => StaticValue::Slot(self.holder_slot(world, t)),
            Exp3aAtom::Team(t) => {
                StaticValue::Flag(self.is_focal(self.slots[self.holder_slot(world, t) as usize]))
            }
            Exp3aAtom::Comp => StaticValue::Tile(self.companion(world)),
            Exp3aAtom::CompInContext => {
                StaticValue::Flag(self.context_pool.contains(self.companion(world)))
            }
            Exp3aAtom::CompIsFloor => StaticValue::Flag(self.floor == Some(self.companion(world))),
            Exp3aAtom::CompRank => {
                let c = self.companion(world);
                StaticValue::Rank(
                    self.context_pool
                        .contains(c)
                        .then(|| self.decl.rank(c).value()),
                )
            }
            Exp3aAtom::FocalMax => StaticValue::Rank(self.slot_max(world, focal_slot)),
            Exp3aAtom::OppMax => StaticValue::Rank(opp_max),
            Exp3aAtom::FocalTop => StaticValue::Flag(self.slot_max(world, focal_slot) > opp_max),
            Exp3aAtom::OppBeaters => {
                let threshold = self.decl.rank(self.decisive);
                let count = self
                    .context_pool
                    .iter()
                    .filter(|d| self.decl.rank(*d) > threshold)
                    .filter(|d| !self.is_focal(self.slots[self.holder_slot(world, *d) as usize]))
                    .count();
                StaticValue::Count(u8::try_from(count).expect("a small pool"))
            }
            Exp3aAtom::BestKeep => {
                // The viewer leads the decisive tile; hidden slots in order,
                // strictly better rank takes the trick. Ranks are injective
                // within the context, so no tie is ever compared.
                let mut best_rank = self.decl.rank(self.decisive).value();
                let mut best_focal = true;
                for i in 0..HIDDEN_SEATS {
                    if let Some(r) = self.slot_max(world, i) {
                        if r > best_rank {
                            best_rank = r;
                            best_focal = self.is_focal(self.slots[i]);
                        }
                    }
                }
                StaticValue::Flag(best_focal)
            }
            Exp3aAtom::WithBoss => StaticValue::Flag(match self.boss {
                Some(boss) => self.holder_slot(world, boss) == self.holder_slot(world, self.valued),
                None => false,
            }),
        }
    }
}

/// The full 22-observable registry over one trick-6 kernel: a holder and a
/// team fact per pool tile, plus the ten control shapes.
pub fn exp3a_registry(kernel: &Kernel) -> Vec<Exp3aAtom> {
    let mut out: Vec<Exp3aAtom> = kernel.pool().iter().map(Exp3aAtom::Holder).collect();
    out.extend(kernel.pool().iter().map(Exp3aAtom::Team));
    out.extend([
        Exp3aAtom::Comp,
        Exp3aAtom::CompInContext,
        Exp3aAtom::CompIsFloor,
        Exp3aAtom::CompRank,
        Exp3aAtom::FocalMax,
        Exp3aAtom::OppMax,
        Exp3aAtom::FocalTop,
        Exp3aAtom::OppBeaters,
        Exp3aAtom::BestKeep,
        Exp3aAtom::WithBoss,
    ]);
    out
}

/// A conjunction of exp3A atoms as a marked static descriptor: the state
/// is the root evaluation, frozen (§14.4 is a static compression result;
/// closed update laws for these shapes are S5 work).
#[derive(Clone, Debug)]
pub struct Exp3aDescriptor {
    ctx: Exp3aContext,
    atoms: Vec<Exp3aAtom>,
}

impl Exp3aDescriptor {
    pub fn new(ctx: Exp3aContext, atoms: Vec<Exp3aAtom>) -> Exp3aDescriptor {
        Exp3aDescriptor { ctx, atoms }
    }

    pub fn context(&self) -> &Exp3aContext {
        &self.ctx
    }
}

impl ControlSkeleton for Exp3aDescriptor {
    type State = Vec<StaticValue>;

    fn name(&self) -> String {
        let parts: Vec<String> = self.atoms.iter().map(|a| a.to_string()).collect();
        format!("exp3a[{}]", parts.join("+"))
    }

    fn kind(&self) -> UpdateKind {
        UpdateKind::StaticPassenger
    }

    fn init(&self, _kernel: &Kernel, world: &World) -> Vec<StaticValue> {
        self.atoms
            .iter()
            .map(|a| self.ctx.eval(*a, world))
            .collect()
    }

    fn step(&self, d: &Vec<StaticValue>, _obs: ObservedPlay) -> Vec<StaticValue> {
        d.clone()
    }
}
