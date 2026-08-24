//! Receipt decision points: the viewer kernel at an *arbitrary* decision of
//! a transcript (v0.4 §2.1 — the fiber from the actor-attributed legal
//! public prefix), mid-trick included.
//!
//! `Kernel::from_receipt_trick` covers only trick-start points with the
//! viewer leading. A seat's transcript has seven decision points per hand,
//! most of them mid-trick: tiles of the current partial trick have left the
//! pool but not yet resolved into a trick, and a follow failure *inside* the
//! partial trick is already an observable void. This module folds both into
//! the §2.1 capacity-cell system.

use crate::rules::receipt::ReceiptHand;
use crate::rules::replay::{state_before_trick, voids_before_trick, ReplayError, TRICKS_PER_HAND};
use crate::rules::{Domino, DominoSet, Seat};

use crate::kernel::kernel::{Hidden, Kernel, World, HIDDEN_SEATS};

/// One decision point of one seat along a receipt hand: the seat is about to
/// play its tile of trick `trick_no` (1-based) with `prefix` already on the
/// table. Everything here is derived from the receipt by replay — nothing is
/// stored authority.
#[derive(Clone, Debug)]
pub struct ReceiptDecision {
    pub kernel: Kernel,
    pub trick_no: usize,
    /// The viewer's position in the trick: 0 leads, 1..=3 follow.
    pub ply: usize,
    pub leader: Seat,
    /// The current trick's plays before the viewer, in play order; the actor
    /// of `prefix[i]` is `leader.plus(i)`.
    pub prefix: Vec<Domino>,
    /// The tile the receipt records the viewer actually playing here.
    pub chosen: Domino,
}

/// The replayed concrete position at one decision point: remaining hands
/// (current-trick prefix already removed), the trick leader, and the
/// viewer's ply. Shared by the kernel constructor and the true-world view.
fn position_at(
    hand: &ReceiptHand,
    trick_no: usize,
    viewer: Seat,
) -> Result<([DominoSet; Seat::COUNT], Seat, usize), ReplayError> {
    let err = |message: String| ReplayError {
        hand: hand.id,
        trick: Some(trick_no),
        message,
    };
    let (mut hands, leader) = state_before_trick(hand, trick_no)?;
    let trick = hand
        .tricks
        .get(trick_no - 1)
        .ok_or_else(|| err(format!("no trick {trick_no} in the receipt")))?;
    if trick.plays[0].0 != leader {
        return Err(err(format!(
            "{} leads the recorded trick, replay derives {leader}",
            trick.plays[0].0
        )));
    }
    let ply = (0..4)
        .find(|k| leader.plus(*k) == viewer)
        .expect("every seat plays in every trick");
    for (k, (seat, tile)) in trick.plays.iter().take(ply).enumerate() {
        if *seat != leader.plus(k) {
            return Err(err(format!(
                "actor {k} is {seat}, expected {}",
                leader.plus(k)
            )));
        }
        if !hands[seat.index()].remove(*tile) {
            return Err(err(format!("{seat} does not hold {tile}")));
        }
    }
    Ok((hands, leader, ply))
}

impl ReceiptDecision {
    /// The decision point of `viewer` in trick `trick_no` of `hand`. The
    /// kernel's void constraints come from the completed tricks *and* from
    /// follow failures inside the current partial trick; the pool is the
    /// live hidden tiles minus the partial trick's plays.
    pub fn at(
        hand: &ReceiptHand,
        trick_no: usize,
        viewer: Seat,
    ) -> Result<ReceiptDecision, ReplayError> {
        let (hands, leader, ply) = position_at(hand, trick_no, viewer)?;
        let trick = &hand.tricks[trick_no - 1];
        let mut voids = voids_before_trick(hand, trick_no);
        if ply > 0 {
            let led = hand.decl.led_context(trick.plays[0].1);
            for (seat, tile) in trick.plays.iter().take(ply).skip(1) {
                if !hand.decl.follows(*tile, led) {
                    voids[seat.index()].insert(led);
                }
            }
        }
        let mut hidden = [Hidden {
            seat: viewer,
            capacity: 0,
            voids: crate::rules::ContextSet::EMPTY,
        }; HIDDEN_SEATS];
        let mut pool = DominoSet::EMPTY;
        for (slot, seat) in hidden.iter_mut().zip((1..=3).map(|k| viewer.plus(k))) {
            *slot = Hidden {
                seat,
                capacity: hands[seat.index()].len(),
                voids: voids[seat.index()],
            };
            pool = pool.union(hands[seat.index()]);
        }
        let kernel =
            Kernel::new(hand.decl, viewer, hands[viewer.index()], pool, hidden).map_err(|e| {
                ReplayError {
                    hand: hand.id,
                    trick: Some(trick_no),
                    message: e.to_string(),
                }
            })?;
        Ok(ReceiptDecision {
            kernel,
            trick_no,
            ply,
            leader,
            prefix: trick.plays.iter().take(ply).map(|(_, d)| *d).collect(),
            chosen: trick.plays[ply].1,
        })
    }

    /// Every decision point of `viewer` along the hand, in trick order.
    pub fn all_for_seat(
        hand: &ReceiptHand,
        viewer: Seat,
    ) -> Result<Vec<ReceiptDecision>, ReplayError> {
        (1..=TRICKS_PER_HAND)
            .map(|t| ReceiptDecision::at(hand, t, viewer))
            .collect()
    }

    /// The receipt's actual hidden remainders at this decision, as a `World`
    /// of this kernel. A God's-eye validator view derived by replay — never
    /// available to the seat.
    pub fn true_world(&self, hand: &ReceiptHand) -> Result<World, ReplayError> {
        let (hands, _, _) = position_at(hand, self.trick_no, self.kernel.viewer())?;
        let cells: [DominoSet; HIDDEN_SEATS] =
            core::array::from_fn(|i| hands[self.kernel.hidden()[i].seat.index()]);
        Ok(self.kernel.world(cells))
    }
}
