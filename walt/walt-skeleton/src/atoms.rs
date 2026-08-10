//! A small descriptor vocabulary in the §14.4 style -- holder facts and
//! bounded local-control relations -- built as genuine transducers.
//!
//! v0.4 §14.4 names its winning exp3A atoms (comp41, s3max2, team(2:0),
//! team(4:2)) but defines neither the 22-observable vocabulary nor those
//! atoms' semantics, and no exp3A probe source survives
//! (walt/DISCREPANCIES.md, "exp3A descriptor pin"). The atoms HERE are
//! walt's own, fully specified below, in the same language family: per-tile
//! holder facts and a beater-control relation shaped like §12.5's "beater
//! and overtake chains". They are not a reproduction of exp3A.
//!
//! Every atom is closed-updatable from observed plays alone, because tile
//! possession is static until the tile is shown: the root evaluation reads
//! the latent world once, and each observed play only ever *removes*
//! latent content (a shown tile's holder question dies; a shown beater
//! leaves its slot's count). The public chassis (viewer hand, leader,
//! current trick prefix) is likewise a fold of the record.

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
