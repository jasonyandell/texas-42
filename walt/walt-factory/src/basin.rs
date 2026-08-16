//! The basin domain: the declared, exhaustively precomputed set of corpus
//! decisions a lesson is verified and measured against (S5b).
//!
//! A domain element is one decision point of one seat in one receipt hand
//! (`ReceiptDecision`, mid-trick included), with its whole fiber
//! enumerated, every world's exact PI action values solved (`ScalarPi`,
//! q_points valuation, focal = the viewer's team, §8.5 future-increment
//! mode — the walker's own semantics), and every vocabulary atom's value
//! tabulated per world. Tonight's honest exhaustive domain is tricks 5–6
//! of all 13 hands and all four seats — the trick-6 kernel corpus and the
//! trick-5 family of the S4/S4.5 machinery, extended to mid-trick decision
//! points; the spec is a parameter so full-hand domains can arrive later
//! without a shape change.
//!
//! Everything precomputed here is a derived view of the receipt and the
//! rules — exact solves and exact enumerations, schedule-independent, no
//! sampling, no seeds.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use walt_core::receipt::Receipt;
use walt_core::{Domino, Seat};
use walt_kernel::{Kernel, ReceiptDecision, World};
use walt_skeleton::{Exp3aAtom, Exp3aContext};
use walt_strat::{ScalarPi, ScalarValuation};

use crate::lesson::{AtomValue, LessonAtom, Role};

/// The ten exp3A control shapes (the §14.4 vocabulary minus its
/// holder/team coordinates, which the union vocabulary carries natively).
pub const CONTROL_SHAPES: [Exp3aAtom; 10] = [
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
];

/// The valued-tile convention shared with the exp5 statistics and the S4
/// runs: the highest count-point unseen tile, ties by pip sum then index.
pub fn valued_tile(kernel: &Kernel) -> Domino {
    kernel
        .pool()
        .iter()
        .max_by_key(|d| (d.count(), d.pip_sum(), d.index()))
        .expect("a nonempty pool")
}

/// The union atom vocabulary of one kernel: holder, team, and
/// beater-count facts per hidden-pool tile, then the ten control shapes.
pub fn vocabulary(kernel: &Kernel) -> Vec<LessonAtom> {
    let mut out: Vec<LessonAtom> = kernel.pool().iter().map(LessonAtom::Holder).collect();
    out.extend(kernel.pool().iter().map(LessonAtom::Team));
    out.extend(kernel.pool().iter().map(LessonAtom::Beaters));
    out.extend(CONTROL_SHAPES.iter().copied().map(LessonAtom::Ctl));
    out
}

/// Partial evaluation of one union-vocabulary atom at one world of one
/// kernel, under the kernel's derived exp3A context. `None` exactly where
/// the atom's precondition fails (tile not hidden here, companion
/// undefined at this capacity).
pub fn eval_atom(
    atom: LessonAtom,
    kernel: &Kernel,
    ctx: &Exp3aContext,
    world: &World,
) -> Option<AtomValue> {
    match atom {
        LessonAtom::Holder(d) => ctx
            .try_eval(Exp3aAtom::Holder(d), world)
            .map(AtomValue::Static),
        LessonAtom::Team(d) => ctx
            .try_eval(Exp3aAtom::Team(d), world)
            .map(AtomValue::Static),
        LessonAtom::Beaters(d) => {
            if !kernel.pool().contains(d) {
                return None;
            }
            let threat = kernel.decl().threat(d);
            Some(AtomValue::Counts(core::array::from_fn(|i| {
                u8::try_from(
                    world
                        .hand(kernel.hidden()[i].seat)
                        .intersection(threat)
                        .len(),
                )
                .expect("a hand holds at most seven tiles")
            })))
        }
        LessonAtom::Ctl(a) => ctx.try_eval(a, world).map(AtomValue::Static),
    }
}

/// The declared domain: which tricks' decision points enter (all hands,
/// all seats, all plies), and the fiber cap. Trick 7 is excluded by
/// default because every seat's play there is forced — no pair verdict
/// can apply and no compression question survives. Decisions whose fiber
/// exceeds `max_fiber` are EXCLUDED, never sampled — the S5c-m1 choice,
/// stated: an exhaustively affordable domain restriction is honest, a
/// sampled membership would silently downgrade every lesson verified on
/// it. The excluded count is part of the built domain's record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DomainSpec {
    pub min_trick: usize,
    pub max_trick: usize,
    pub max_fiber: u128,
}

impl DomainSpec {
    /// The S5b domain (no trick-5/6 fiber exceeds the cap: 0 excluded).
    pub fn tricks_5_to_6() -> DomainSpec {
        DomainSpec {
            min_trick: 5,
            max_trick: 6,
            max_fiber: 40_000,
        }
    }

    /// The S5c-m1 falsification domain: tricks 3-6 where exhaustively
    /// affordable — every trick-4-6 decision qualifies (t4 fibers top out
    /// at 34,650); at trick 3 only void-constrained fibers do.
    pub fn tricks_3_to_6() -> DomainSpec {
        DomainSpec {
            min_trick: 3,
            max_trick: 6,
            max_fiber: 40_000,
        }
    }

    /// Whether a decision at `trick_no` with fiber size `fiber` lies
    /// inside this verified scope. The application gate reads this before
    /// anything else (TRUST-01 shape: a verdict's scope is its verified
    /// domain) — an excluded-fiber decision was never verified, so it is
    /// outside the scope exactly like an out-of-range trick.
    pub fn covers(&self, trick_no: usize, fiber: u128) -> bool {
        (self.min_trick..=self.max_trick).contains(&trick_no) && fiber <= self.max_fiber
    }
}

impl core::fmt::Display for DomainSpec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "receipt corpus, hands 0-12, all seats, tricks {}-{}, fiber <= {}",
            self.min_trick, self.max_trick, self.max_fiber
        )
    }
}

/// One precomputed domain decision. All fields are derived views; the
/// atom columns are eager so the whole structure is immutable and
/// shareable.
pub struct DomainDecision {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    pub ply: usize,
    pub role: Role,
    pub horizon: usize,
    /// The legal actions, ascending (world-independent).
    pub actions: Vec<Domino>,
    /// The kernel's derived decisive tile (selector substrate).
    pub decisive: Domino,
    pub kernel: Kernel,
    pub worlds: Vec<World>,
    /// `values[w][a]`: exact PI future-increment value of action `a` in
    /// world `w` (q_points valuation, focal = viewer's team).
    pub values: Vec<Vec<i64>>,
    /// Per-atom value columns over `worlds`, for the kernel's whole union
    /// vocabulary. An atom outside this kernel's vocabulary has no column
    /// and is undefined at every world here.
    columns: BTreeMap<LessonAtom, Vec<Option<AtomValue>>>,
}

impl DomainDecision {
    /// The atom's value column, or `None` when the atom is not in this
    /// kernel's vocabulary (then it is undefined at every world).
    pub fn column(&self, atom: LessonAtom) -> Option<&[Option<AtomValue>]> {
        self.columns.get(&atom).map(Vec::as_slice)
    }

    /// Precomputes one decision point outside a `BasinDomain` — the entry
    /// the application gate is tested against, and the S5c hook for
    /// applying lessons at decisions of other domains (which still
    /// requires the lesson's verified `DomainSpec` to cover the trick).
    pub fn at(receipt: &Receipt, hand: usize, seat: Seat, trick_no: usize) -> DomainDecision {
        DomainDecision::build(receipt, hand, seat, trick_no)
    }

    fn build(receipt: &Receipt, hand: usize, seat: Seat, trick_no: usize) -> DomainDecision {
        let rh = &receipt.hands[hand];
        let decision = ReceiptDecision::at(rh, trick_no, seat)
            .expect("replay-verified receipts yield every decision point");
        let kernel = decision.kernel.clone();
        let ctx = Exp3aContext::new(&kernel, valued_tile(&kernel));
        let worlds: Vec<World> = kernel.worlds().collect();
        assert_eq!(
            worlds.len() as u128,
            kernel.count(),
            "the domain fiber is exhaustive"
        );

        let mut pi = ScalarPi::new(
            kernel.decl(),
            kernel.viewer().team(),
            ScalarValuation::trick_plus_count(),
        );
        let mut actions: Vec<Domino> = Vec::new();
        let values: Vec<Vec<i64>> = worlds
            .iter()
            .map(|w| {
                let solved = pi.action_values(w.hands(), decision.leader, &decision.prefix);
                if actions.is_empty() {
                    actions = solved.iter().map(|(d, _)| *d).collect();
                } else {
                    debug_assert!(
                        solved.iter().map(|(d, _)| *d).eq(actions.iter().copied()),
                        "the legal set is view-determined"
                    );
                }
                solved.into_iter().map(|(_, v)| v).collect()
            })
            .collect();

        let columns: BTreeMap<LessonAtom, Vec<Option<AtomValue>>> = vocabulary(&kernel)
            .into_iter()
            .map(|atom| {
                let col = worlds
                    .iter()
                    .map(|w| eval_atom(atom, &kernel, &ctx, w))
                    .collect();
                (atom, col)
            })
            .collect();

        DomainDecision {
            hand,
            seat,
            trick_no,
            ply: decision.ply,
            role: if rh.declaring_team == seat.team() {
                Role::Declaring
            } else {
                Role::Defending
            },
            horizon: kernel.viewer_hand().len(),
            actions,
            decisive: ctx.decisive,
            kernel,
            worlds,
            values,
            columns,
        }
    }
}

/// The precomputed domain: decisions in (hand, trick, seat) order.
/// `excluded_decisions` counts the in-trick-range decision points whose
/// fiber exceeded the cap — part of the domain's honest record.
pub struct BasinDomain {
    pub spec: DomainSpec,
    pub decisions: Vec<DomainDecision>,
    pub worlds_total: usize,
    pub excluded_decisions: usize,
}

impl BasinDomain {
    /// Builds the whole domain. `threads` bounds the scoped workers (0
    /// uses one); every precomputed quantity is an exact solve or an exact
    /// enumeration, so the result is schedule-independent. Fiber counts
    /// (the counting DP) decide membership before any enumeration.
    pub fn build(receipt: &Receipt, spec: DomainSpec, threads: usize) -> BasinDomain {
        let mut triples: Vec<(usize, Seat, usize)> = Vec::new();
        let mut excluded_decisions = 0usize;
        for hand in 0..receipt.hands.len() {
            for trick_no in spec.min_trick..=spec.max_trick {
                for seat in Seat::ALL {
                    let d = ReceiptDecision::at(&receipt.hands[hand], trick_no, seat)
                        .expect("replay-verified receipts yield every decision point");
                    if d.kernel.count() <= spec.max_fiber {
                        triples.push((hand, seat, trick_no));
                    } else {
                        excluded_decisions += 1;
                    }
                }
            }
        }
        let slots: Vec<OnceLock<DomainDecision>> =
            (0..triples.len()).map(|_| OnceLock::new()).collect();
        let next = AtomicUsize::new(0);
        let workers = threads.max(1).min(triples.len());
        std::thread::scope(|scope| {
            for _ in 0..workers {
                scope.spawn(|| loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    if i >= triples.len() {
                        break;
                    }
                    let (hand, seat, trick_no) = triples[i];
                    let d = DomainDecision::build(receipt, hand, seat, trick_no);
                    slots[i].set(d).ok().expect("each slot is built once");
                });
            }
        });
        let decisions: Vec<DomainDecision> = slots
            .into_iter()
            .map(|s| s.into_inner().expect("every slot built"))
            .collect();
        let worlds_total = decisions.iter().map(|d| d.worlds.len()).sum();
        BasinDomain {
            spec,
            decisions,
            worlds_total,
            excluded_decisions,
        }
    }
}
