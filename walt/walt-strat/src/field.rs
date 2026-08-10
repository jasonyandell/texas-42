//! The fixed uniform-random legal field of v0.4 §14.2: every non-focal seat
//! plays uniformly at random over its legal set, and the focal expectation is
//! taken under the uniform belief on the kernel fiber.
//!
//! S2 deliberately covers only the degenerate slice where the focal seat
//! never has a choice after the root -- true of every horizon-2 kernel, where
//! the viewer's one remaining tile is forced. With no post-root focal
//! decision, hidden and revealed optimization coincide vacuously and each
//! world/root value is one exact affine line. The general hidden fixed-field
//! solve (optimization over focal information states) is the S3 H operator;
//! this function refuses, rather than misprices, any kernel that needs it.

use walt_core::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::{q, Line};
use walt_kernel::Kernel;

use crate::direction::Direction;

/// A focal decision node below the root: the input needs the S3 H operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PostRootChoice {
    pub seat: Seat,
    pub options: DominoSet,
}

struct Ctx<'a> {
    decl: Decl,
    viewer: Seat,
    focal: Team,
    dir: &'a Direction,
}

/// The expected affine value of the continuation from a mid-trick node, under
/// the uniform-random legal field, in one fixed world.
fn expected(
    ctx: &Ctx,
    hands: [DominoSet; Seat::COUNT],
    leader: Seat,
    tiles: [Domino; 4],
    k: usize,
) -> Result<Line, PostRootChoice> {
    if k == 4 {
        let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
        let winner = trick.winner(ctx.decl);
        let inc = ctx.dir.trick_line(ctx.focal, winner, tiles);
        if hands.iter().all(|h| h.is_empty()) {
            return Ok(inc);
        }
        return Ok(expected(ctx, hands, winner, [Domino::ALL[0]; 4], 0)?.add(&inc));
    }
    let seat = leader.plus(k);
    let led = (k > 0).then(|| ctx.decl.led_context(tiles[0]));
    let legal = legal_plays(ctx.decl, hands[seat.index()], led);
    if seat == ctx.viewer && legal.len() != 1 {
        return Err(PostRootChoice {
            seat,
            options: legal,
        });
    }
    let mut sum = Line::zero();
    for d in legal.iter() {
        let mut hands = hands;
        hands[seat.index()].remove(d);
        let mut tiles = tiles;
        tiles[k] = d;
        sum = sum.add(&expected(ctx, hands, leader, tiles, k + 1)?);
    }
    Ok(sum.scale(q(1, legal.len() as i128)))
}

/// `Q^H(a; lambda)` for every root action of the viewer: the exact affine
/// action values under uniform belief on the fiber and the uniform-random
/// legal field, in ascending domino order.
pub fn fixed_field_root_lines(
    kernel: &Kernel,
    focal: Team,
    dir: &Direction,
) -> Result<Vec<(Domino, Line)>, PostRootChoice> {
    let ctx = Ctx {
        decl: kernel.decl(),
        viewer: kernel.viewer(),
        focal,
        dir,
    };
    let mut out = Vec::new();
    for a in kernel.viewer_hand().iter() {
        let mut sum = Line::zero();
        let mut worlds: i128 = 0;
        for world in kernel.worlds() {
            let mut hands = world.hands();
            hands[ctx.viewer.index()].remove(a);
            let mut tiles = [Domino::ALL[0]; 4];
            tiles[0] = a;
            sum = sum.add(&expected(&ctx, hands, ctx.viewer, tiles, 1)?);
            worlds += 1;
        }
        assert_eq!(worlds as u128, kernel.count(), "enumeration count drift");
        out.push((a, sum.scale(q(1, worlds))));
    }
    Ok(out)
}
