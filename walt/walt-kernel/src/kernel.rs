//! The viewer kernel `C = (U; (P_s, k_s))` of v0.4 §2.1.

use core::fmt;

use walt_core::receipt::ReceiptHand;
use walt_core::replay::{state_before_trick, voids_before_trick, ReplayError};
use walt_core::{ContextSet, Decl, DominoSet, Seat};

/// Four seats, one viewer.
pub const HIDDEN_SEATS: usize = 3;

/// The largest hand a seat can hold, and so the largest capacity.
pub const MAX_CAPACITY: usize = 7;

/// One hidden seat: its remaining capacity and the contexts the public
/// history has shown it void in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hidden {
    pub seat: Seat,
    pub capacity: usize,
    pub voids: ContextSet,
}

/// A complete assignment of the live tiles to the four seats. Constructed only
/// by the fiber machinery or by `Kernel::world`, so a `World` always refers to
/// some capacity-cell system; membership in a particular fiber is
/// `Kernel::contains`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct World {
    hands: [DominoSet; Seat::COUNT],
}

impl World {
    pub const fn hands(&self) -> [DominoSet; Seat::COUNT] {
        self.hands
    }

    pub const fn hand(&self, seat: Seat) -> DominoSet {
        self.hands[seat.index()]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KernelError {
    ViewerIsHidden(Seat),
    RepeatedHiddenSeat(Seat),
    PoolMeetsViewerHand(DominoSet),
    CapacityOverMax(usize),
    /// The capacities must exactly consume the hidden pool.
    CapacitySum {
        pool: usize,
        capacity: usize,
    },
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::ViewerIsHidden(s) => write!(f, "{s} is both viewer and hidden"),
            KernelError::RepeatedHiddenSeat(s) => write!(f, "{s} appears twice among the hidden"),
            KernelError::PoolMeetsViewerHand(d) => {
                write!(f, "the hidden pool overlaps the viewer's hand at {d:?}")
            }
            KernelError::CapacityOverMax(k) => write!(f, "capacity {k} exceeds {MAX_CAPACITY}"),
            KernelError::CapacitySum { pool, capacity } => write!(
                f,
                "capacities total {capacity} but the hidden pool holds {pool}"
            ),
        }
    }
}

impl std::error::Error for KernelError {}

/// A viewer's exact rule support at one decision point.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Kernel {
    decl: Decl,
    viewer: Seat,
    viewer_hand: DominoSet,
    pool: DominoSet,
    hidden: [Hidden; HIDDEN_SEATS],
}

impl Kernel {
    pub fn new(
        decl: Decl,
        viewer: Seat,
        viewer_hand: DominoSet,
        pool: DominoSet,
        hidden: [Hidden; HIDDEN_SEATS],
    ) -> Result<Kernel, KernelError> {
        if !pool.is_disjoint(viewer_hand) {
            return Err(KernelError::PoolMeetsViewerHand(
                pool.intersection(viewer_hand),
            ));
        }
        let mut capacity = 0;
        for (i, h) in hidden.iter().enumerate() {
            if h.seat == viewer {
                return Err(KernelError::ViewerIsHidden(h.seat));
            }
            if hidden.iter().take(i).any(|g| g.seat == h.seat) {
                return Err(KernelError::RepeatedHiddenSeat(h.seat));
            }
            if h.capacity > MAX_CAPACITY {
                return Err(KernelError::CapacityOverMax(h.capacity));
            }
            capacity += h.capacity;
        }
        if capacity != pool.len() {
            return Err(KernelError::CapacitySum {
                pool: pool.len(),
                capacity,
            });
        }
        Ok(Kernel {
            decl,
            viewer,
            viewer_hand,
            pool,
            hidden,
        })
    }

    /// The kernel a seat holds at the start of `trick_no` of a receipt hand,
    /// with the viewer taken to be that trick's leader. This is the bridge to
    /// the probe corpus: same viewer, same voids, same pool.
    pub fn from_receipt_trick(hand: &ReceiptHand, trick_no: usize) -> Result<Kernel, ReplayError> {
        let (hands, leader) = state_before_trick(hand, trick_no)?;
        let voids = voids_before_trick(hand, trick_no);
        let mut hidden = [Hidden {
            seat: leader,
            capacity: 0,
            voids: ContextSet::EMPTY,
        }; HIDDEN_SEATS];
        let mut pool = DominoSet::EMPTY;
        for (slot, seat) in hidden.iter_mut().zip((1..=3).map(|k| leader.plus(k))) {
            *slot = Hidden {
                seat,
                capacity: hands[seat.index()].len(),
                voids: voids[seat.index()],
            };
            pool = pool.union(hands[seat.index()]);
        }
        Kernel::new(hand.decl, leader, hands[leader.index()], pool, hidden).map_err(|e| {
            ReplayError {
                hand: hand.id,
                trick: Some(trick_no),
                message: e.to_string(),
            }
        })
    }

    pub const fn decl(&self) -> Decl {
        self.decl
    }

    pub const fn viewer(&self) -> Seat {
        self.viewer
    }

    pub const fn viewer_hand(&self) -> DominoSet {
        self.viewer_hand
    }

    /// The hidden live pool `U`.
    pub const fn pool(&self) -> DominoSet {
        self.pool
    }

    /// The live set `L(K)`: the viewer's remaining tiles and the hidden pool.
    pub const fn live(&self) -> DominoSet {
        self.viewer_hand.union(self.pool)
    }

    /// Absolute live-set mastery (v0.4 §13.1): the live tiles no live tile
    /// can beat when led. A relation to the current live algebra, not an
    /// intrinsic pip magnitude.
    pub fn masters(&self) -> DominoSet {
        let live = self.live();
        live.iter()
            .filter(|d| self.decl.threat(*d).intersection(live).is_empty())
            .collect()
    }

    pub const fn hidden(&self) -> &[Hidden; HIDDEN_SEATS] {
        &self.hidden
    }

    /// `P_s`: the pool tiles that remain locally possible at hidden slot `i`
    /// after the observed voids. A seat shown void in context `q` holds no
    /// tile of the effective incidence of `q` -- absorption is already inside
    /// `effective_incidence`, so a called tile is excluded by a called-suit
    /// void and by nothing else.
    pub fn allowed(&self, i: usize) -> DominoSet {
        let mut out = self.pool;
        for q in self.hidden[i].voids.iter() {
            out = out.difference(self.decl.effective_incidence(q));
        }
        out
    }

    /// The unconstrained fiber size: the multinomial that ignores every void.
    pub fn unconstrained_count(&self) -> u128 {
        let mut out: u128 = 1;
        let mut left = self.pool.len();
        for h in &self.hidden {
            out *= crate::fiber::binomial(left, h.capacity);
            left -= h.capacity;
        }
        out
    }

    /// Assemble a world from an assignment to the hidden slots. No validity
    /// claim is made here; use `contains`.
    pub fn world(&self, hidden_hands: [DominoSet; HIDDEN_SEATS]) -> World {
        let mut hands = [DominoSet::EMPTY; Seat::COUNT];
        hands[self.viewer.index()] = self.viewer_hand;
        for (h, cell) in self.hidden.iter().zip(hidden_hands) {
            hands[h.seat.index()] = cell;
        }
        World { hands }
    }

    /// Exact membership in `Phi(C)`: capacities met, cells disjoint, the pool
    /// exactly partitioned, every void respected, viewer hand untouched.
    pub fn contains(&self, world: &World) -> bool {
        if world.hand(self.viewer) != self.viewer_hand {
            return false;
        }
        let mut seen = DominoSet::EMPTY;
        for (i, h) in self.hidden.iter().enumerate() {
            let cell = world.hand(h.seat);
            if cell.len() != h.capacity || !cell.is_subset_of(self.allowed(i)) {
                return false;
            }
            if !seen.is_disjoint(cell) {
                return false;
            }
            seen = seen.union(cell);
        }
        seen == self.pool
    }
}
