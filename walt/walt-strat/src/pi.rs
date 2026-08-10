//! Worldwise perfect-information symbolic parametric backward induction
//! (v0.4 §9.9): all four hands are known, focal-team decision nodes take the
//! finite pointwise maximum, opposing nodes the minimum, and leaves are
//! affine in the valuation parameter, so every root action value is a
//! continuous piecewise-linear envelope. Per §10.8 this is the PI operator --
//! a different operator from the fixed-field revealed treatments (S3).
//!
//! The semantic state of the recursion is (remaining hands, leader, partial
//! trick); trick increments are added functionally as tricks resolve, so no
//! derived tally is ever stored beside the state.

use walt_core::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::Envelope;

use crate::direction::Direction;

struct Ctx<'a> {
    decl: Decl,
    focal: Team,
    dir: &'a Direction,
}

/// The parametric value of the continuation from a mid-trick node, for the
/// team-of-the-actor operator convention above.
fn value(
    ctx: &Ctx,
    hands: [DominoSet; Seat::COUNT],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
) -> Envelope {
    if k == 4 {
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(ctx.decl);
        let inc = ctx.dir.trick_line(ctx.focal, winner, tiles);
        if hands.iter().all(|h| h.is_empty()) {
            return Envelope::line(inc);
        }
        return value(ctx, hands, winner, [Domino::ALL[0]; 4], 0).add_line(&inc);
    }
    let seat = leader.plus(k);
    let led = (k > 0).then(|| ctx.decl.led_context(tiles[0]));
    let legal = legal_plays(ctx.decl, hands[seat.index()], led);
    let children = legal.iter().map(|d| {
        let mut hands = hands;
        hands[seat.index()].remove(d);
        let mut tiles = tiles;
        tiles[k] = d;
        value(ctx, hands, leader, tiles, k + 1)
    });
    if seat.team() == ctx.focal {
        Envelope::max_of(children)
    } else {
        Envelope::min_of(children)
    }
}

/// The parametric root action values `Q(a; lambda)` for one complete world:
/// for each tile the leader may play, the minimax value of the entire
/// continuation after it, in ascending domino order.
pub fn pi_root_values(
    decl: Decl,
    hands: [DominoSet; Seat::COUNT],
    leader: Seat,
    focal: Team,
    dir: &Direction,
) -> Vec<(Domino, Envelope)> {
    let ctx = Ctx { decl, focal, dir };
    let mut out = Vec::new();
    for d in hands[leader.index()].iter() {
        let mut hands = hands;
        hands[leader.index()].remove(d);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[0] = d;
        out.push((d, value(&ctx, hands, leader, tiles, 1)));
    }
    out
}
