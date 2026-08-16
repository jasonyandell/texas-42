//! History replay: re-derive a receipt hand from the rules alone.
//!
//! Nothing in the receipt is trusted except the tiles and who played them.
//! Actor order, follow legality, trick winners, trick points, cumulative
//! totals, and the made/set verdict are all recomputed and compared.

use core::fmt;

use crate::domino::Domino;
use crate::receipt::{Receipt, ReceiptHand};
use crate::rules::{legal_plays, Trick};
use crate::seat::Seat;
use crate::set::{ContextSet, DominoSet};

/// Total points in a complete hand: seven trick points plus the 35 count
/// (v0.4 §1.4).
pub const HAND_TOTAL_POINTS: u32 = 42;
pub const TRICKS_PER_HAND: usize = 7;
pub const HAND_SIZE: usize = 7;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayError {
    pub hand: usize,
    pub trick: Option<usize>,
    pub message: String,
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hand {}", self.hand)?;
        if let Some(t) = self.trick {
            write!(f, " trick {t}")?;
        }
        write!(f, ": {}", self.message)
    }
}

impl std::error::Error for ReplayError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandReplay {
    pub deal: [DominoSet; Seat::COUNT],
    pub trick_winners: Vec<Seat>,
    pub trick_points: Vec<u32>,
    pub team_points: [u32; 2],
    pub team_tricks: [u32; 2],
}

struct Ctx {
    hand: usize,
    trick: Option<usize>,
}

impl Ctx {
    fn err<T>(&self, message: impl Into<String>) -> Result<T, ReplayError> {
        Err(ReplayError {
            hand: self.hand,
            trick: self.trick,
            message: message.into(),
        })
    }

    fn check(&self, cond: bool, message: impl Into<String>) -> Result<(), ReplayError> {
        if cond {
            Ok(())
        } else {
            self.err(message)
        }
    }
}

/// The complete deal implied by the hand: every tile each seat played.
pub fn deal(hand: &ReceiptHand) -> Result<[DominoSet; Seat::COUNT], ReplayError> {
    let mut ctx = Ctx {
        hand: hand.id,
        trick: None,
    };
    let mut out = [DominoSet::EMPTY; Seat::COUNT];
    for trick in &hand.tricks {
        ctx.trick = Some(trick.number);
        for (seat, domino) in trick.plays {
            if !out[seat.index()].insert(domino) {
                return ctx.err(format!("{seat} plays {domino} twice"));
            }
        }
    }
    ctx.trick = None;
    let mut seen = DominoSet::EMPTY;
    for (i, h) in out.iter().enumerate() {
        ctx.check(
            h.len() == HAND_SIZE,
            format!("seat S{i} plays {} tiles, expected {HAND_SIZE}", h.len()),
        )?;
        ctx.check(seen.is_disjoint(*h), format!("seat S{i} shares a tile"))?;
        seen = seen.union(*h);
    }
    ctx.check(
        seen == DominoSet::FULL,
        format!("the deal covers {} of {} tiles", seen.len(), Domino::COUNT),
    )?;
    Ok(out)
}

/// Remaining hands and the leader immediately before trick `trick_no`
/// (1-based). Derived by replaying the recorded plays, not stored.
pub fn state_before_trick(
    hand: &ReceiptHand,
    trick_no: usize,
) -> Result<([DominoSet; Seat::COUNT], Seat), ReplayError> {
    let mut hands = deal(hand)?;
    let mut leader = hand.bidder;
    for trick in hand.tricks.iter().take(trick_no.saturating_sub(1)) {
        for (seat, domino) in trick.plays {
            hands[seat.index()].remove(domino);
        }
        leader = trick.winner;
    }
    Ok((hands, leader))
}

/// Void constraints observable from tricks `1..trick_no`: a seat that failed
/// to follow the led effective context holds no tile of it (absorption
/// respected, since membership is effective incidence).
pub fn voids_before_trick(hand: &ReceiptHand, trick_no: usize) -> [ContextSet; Seat::COUNT] {
    let mut out = [ContextSet::EMPTY; Seat::COUNT];
    for trick in hand.tricks.iter().take(trick_no.saturating_sub(1)) {
        let (leader, lead_tile) = trick.plays[0];
        let led = hand.decl.led_context(lead_tile);
        for (seat, domino) in trick.plays {
            if seat != leader && !hand.decl.follows(domino, led) {
                out[seat.index()].insert(led);
            }
        }
    }
    out
}

pub fn replay_hand(hand: &ReceiptHand) -> Result<HandReplay, ReplayError> {
    let deal = deal(hand)?;
    let mut ctx = Ctx {
        hand: hand.id,
        trick: None,
    };
    ctx.check(
        hand.tricks.len() == TRICKS_PER_HAND,
        format!("{} tricks recorded", hand.tricks.len()),
    )?;

    let mut hands = deal;
    let mut leader = hand.bidder;
    let mut team_points = [0u32; 2];
    let mut team_tricks = [0u32; 2];
    let mut trick_winners = Vec::with_capacity(TRICKS_PER_HAND);
    let mut trick_points = Vec::with_capacity(TRICKS_PER_HAND);

    for (i, rec) in hand.tricks.iter().enumerate() {
        ctx.trick = Some(rec.number);
        ctx.check(rec.number == i + 1, "tricks out of order")?;
        ctx.check(
            rec.plays[0].0 == leader,
            format!("{} leads, expected {leader}", rec.plays[0].0),
        )?;

        let mut dominoes = [Domino::ALL[0]; 4];
        let mut led = None;
        for (k, (seat, domino)) in rec.plays.into_iter().enumerate() {
            ctx.check(
                seat == leader.plus(k),
                format!("actor {k} is {seat}, expected {}", leader.plus(k)),
            )?;
            let held = &mut hands[seat.index()];
            ctx.check(
                held.contains(domino),
                format!("{seat} does not hold {domino}"),
            )?;
            let legal = legal_plays(hand.decl, *held, led);
            ctx.check(
                legal.contains(domino),
                format!("{seat} plays {domino}, not in the legal set {legal:?}"),
            )?;
            held.remove(domino);
            dominoes[k] = domino;
            if k == 0 {
                led = Some(hand.decl.led_context(domino));
            }
        }

        let trick = match Trick::new(leader, dominoes) {
            Ok(t) => t,
            Err(e) => return ctx.err(format!("{} played twice in one trick", e.0)),
        };
        let winner = trick.winner(hand.decl);
        let points = trick.points();
        ctx.check(
            winner == rec.winner,
            format!("derived winner {winner}, receipt says {}", rec.winner),
        )?;
        ctx.check(
            points == rec.points,
            format!("derived {points} points, receipt says {}", rec.points),
        )?;

        team_points[winner.team().index()] += points;
        team_tricks[winner.team().index()] += 1;
        trick_winners.push(winner);
        trick_points.push(points);
        leader = winner;
    }

    ctx.trick = None;
    for (i, h) in hands.iter().enumerate() {
        ctx.check(h.is_empty(), format!("seat S{i} holds {h:?} at the end"))?;
    }
    let total = team_points[0] + team_points[1];
    ctx.check(
        total == HAND_TOTAL_POINTS,
        format!("hand totals {total}, expected {HAND_TOTAL_POINTS}"),
    )?;
    let declared = team_points[hand.declaring_team.index()];
    ctx.check(
        declared == hand.declaring_points,
        format!(
            "declaring {} took {declared}, receipt says {}",
            hand.declaring_team, hand.declaring_points
        ),
    )?;
    let made = declared >= hand.bid_points;
    ctx.check(
        made == hand.made,
        format!(
            "derived {}, receipt says {}",
            verdict(made),
            verdict(hand.made)
        ),
    )?;
    let winner_of_marks = if made {
        hand.declaring_team
    } else {
        hand.declaring_team.other()
    };
    ctx.check(
        hand.marks_awarded.0 == winner_of_marks,
        format!(
            "marks awarded to {}, derived {winner_of_marks}",
            hand.marks_awarded.0
        ),
    )?;

    Ok(HandReplay {
        deal,
        trick_winners,
        trick_points,
        team_points,
        team_tricks,
    })
}

fn verdict(made: bool) -> &'static str {
    if made {
        "made"
    } else {
        "set"
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptReplay {
    pub hands: Vec<HandReplay>,
    pub final_marks: [u32; 2],
}

/// Replays every hand and re-derives the running mark totals.
pub fn replay_receipt(receipt: &Receipt) -> Result<ReceiptReplay, ReplayError> {
    let mut marks = [0u32; 2];
    let mut hands = Vec::with_capacity(receipt.hands.len());
    for hand in &receipt.hands {
        hands.push(replay_hand(hand)?);
        let (team, n) = hand.marks_awarded;
        marks[team.index()] += n;
        let ctx = Ctx {
            hand: hand.id,
            trick: None,
        };
        ctx.check(
            marks == hand.marks_after,
            format!(
                "derived marks T0 {} - {} T1, receipt says T0 {} - {} T1",
                marks[0], marks[1], hand.marks_after[0], hand.marks_after[1]
            ),
        )?;
    }
    let ctx = Ctx {
        hand: receipt.hands.len(),
        trick: None,
    };
    ctx.check(
        marks == receipt.final_marks,
        "match result disagrees with the per-hand mark awards",
    )?;
    ctx.check(
        receipt.hands_played == receipt.hands.len(),
        format!(
            "match result claims {} hands, {} parsed",
            receipt.hands_played,
            receipt.hands.len()
        ),
    )?;
    Ok(ReceiptReplay {
        hands,
        final_marks: marks,
    })
}
