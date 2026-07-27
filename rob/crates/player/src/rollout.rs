//! Deterministic hand completion under a fixed common policy.
//!
//! Consumes the rules engine (`DeclarationAlgebra::follows`, `led_suit`,
//! `resolve_trick`) — no second copy of the algebra. Legality is the
//! Exec §11 `legalPlays` formula spelled through the engine's follow
//! relation: follow-if-possible on effective suits, else anything.

use rob_core::{DeclarationAlgebra, DominoId, DominoSet, Play, Seat};

use crate::policy::{RolloutPolicy, RolloutView};

/// A complete-information rollout position: all four hands plus the public
/// trick residue. Constructed per simulated world by the evaluator; the
/// policy never sees this struct.
#[derive(Clone, Debug)]
pub struct RolloutPosition {
    /// Remaining hands by seat (complete information inside one sampled
    /// world).
    pub hands: [DominoSet; 4],
    /// Current trick leader.
    pub leader: Seat,
    /// Current partial trick.
    pub trick: Vec<Play>,
    /// Banked points by partnership.
    pub points: [u32; 2],
}

impl RolloutPosition {
    fn remaining(&self) -> usize {
        self.hands.iter().map(DominoSet::len).sum()
    }

    fn actor(&self) -> Seat {
        self.leader.offset(self.trick.len() as u8)
    }

    /// The exact legal set for the current actor (Exec §11 via the engine's
    /// follow relation).
    pub fn legal(&self, algebra: &DeclarationAlgebra) -> Vec<DominoId> {
        let hand = &self.hands[self.actor().index()];
        if self.trick.is_empty() {
            return hand.iter().collect();
        }
        let q = algebra.led_suit(self.trick[0].domino);
        let followers: Vec<DominoId> = hand.iter().filter(|&d| algebra.follows(d, q)).collect();
        if followers.is_empty() {
            hand.iter().collect()
        } else {
            followers
        }
    }

    /// Apply one play; resolves the trick on the fourth play via the
    /// engine's `resolve_trick`.
    pub fn apply(&mut self, algebra: &DeclarationAlgebra, domino: DominoId) {
        let actor = self.actor();
        assert!(self.hands[actor.index()].contains(domino), "play from hand");
        self.hands[actor.index()].remove(domino);
        self.trick.push(Play { actor, domino });
        if self.trick.len() == 4 {
            let result = algebra
                .resolve_trick(&self.trick)
                .expect("four distinct clockwise plays resolve");
            self.points[result.winner.team().index()] += result.points as u32;
            self.trick.clear();
            self.leader = result.winner;
        }
    }
}

/// Finish a hand under one fixed common policy for every seat, returning
/// final points by partnership. Every rollout must conserve exactly 42
/// points (R-SCORE-04) — asserted here, and again by the named invariant
/// test `inv_rollout_conservation`.
pub fn finish_hand(
    algebra: &DeclarationAlgebra,
    mut position: RolloutPosition,
    policy: &mut impl RolloutPolicy,
) -> [u32; 2] {
    let declaration = algebra.declaration();
    while position.remaining() > 0 {
        let legal = position.legal(algebra);
        let view = RolloutView {
            declaration,
            leader: position.leader,
            trick: &position.trick,
            points: position.points,
            actor: position.actor(),
        };
        let index = policy.choose(&view, &legal);
        position.apply(algebra, legal[index]);
    }
    assert!(position.trick.is_empty(), "all tricks resolve");
    assert_eq!(
        position.points[0] + position.points[1],
        42,
        "every rollout sums to exactly 42 points"
    );
    position.points
}
