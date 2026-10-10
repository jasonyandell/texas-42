//! One-parameter valuation directions `v0 + lambda*u` (v0.4 §9.4), applied to
//! the focal-differential capture feature: a resolved trick pays its holder
//! team the trick coefficient plus its tiles' coefficients, and the utility
//! is the focal team's total minus the opposing team's.

use crate::geom::{FeatureVec, Line};
use crate::rules::{Domino, Seat, Team};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Direction {
    /// The baseline valuation `v0`.
    pub base: FeatureVec,
    /// The lambda coefficient `u`.
    pub delta: FeatureVec,
}

impl Direction {
    /// The bare future-trick differential: `v0 = e_star`, `u = 0` -- the
    /// lambda-independent utility every §14 experiment shares at lambda = 0.
    pub fn trick_diff() -> Direction {
        Direction {
            base: FeatureVec::unit_trick(),
            delta: FeatureVec::zero(),
        }
    }

    /// The experiment utility `Psi(lambda)` of v0.4 §14.2: future trick
    /// differential plus lambda times the capture differential of one valued
    /// tile.
    pub fn trick_diff_plus_tile(d: Domino) -> Direction {
        Direction {
            base: FeatureVec::unit_trick(),
            delta: FeatureVec::unit_tile(d),
        }
    }

    /// The affine increment one resolved trick contributes to the focal
    /// differential: `sign * <v, (1, tiles)>` with sign +1 when the focal
    /// team captured the trick and -1 otherwise.
    pub fn trick_line(&self, focal: Team, winner: Seat, tiles: [Domino; 4]) -> Line {
        let paired = |v: &FeatureVec| {
            let mut out = v.tricks;
            for d in tiles {
                out += v.tiles[d.index()];
            }
            out
        };
        let line = Line::new(paired(&self.base), paired(&self.delta));
        if winner.team() == focal {
            line
        } else {
            line.scale(crate::geom::qi(-1))
        }
    }
}
