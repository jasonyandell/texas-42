//! Scalar (H, fixed-uniform-legal) action values at arbitrary decision
//! points (S5c-m2): exact `Q^H` per legal viewer action under the actual
//! hidden-information treatment — every viewer choice below the root is
//! made once per pooled information state against the whole particle set —
//! with the §7.4 fixed uniform-legal field and the declared
//! uniform-over-fiber root weighting. The scalar sibling of `hidden`
//! (symbolic, root-only); mid-trick roots included, exactly like the
//! walker's decision points.
//!
//! Exactness carrier: a particle's weight along a branch is always a unit
//! fraction `1/d` — the product of per-world `1/|legal|` field shares —
//! so weights are stored as bare `u128` denominators and rational
//! arithmetic (walt-geom `Q`, i128 ratios) happens only at trick
//! resolutions and comparisons. Viewer maximization compares unnormalized
//! weighted sums: the particle set is common across the actions of one
//! pooled information state, so the argmax over conditional expectations
//! equals the argmax over the sums. Field-node mass conservation
//! (children sum to the parent) holds by construction — each particle
//! splits into exactly its `|legal|` children at share `1/|legal|` — and
//! is spot-asserted in debug builds.
//!
//! Budgeted: the observation-tree walk costs one budget unit per
//! (particle, node) visit; a call that would exceed its budget returns
//! `None` — the same exclusion honesty as the basin domain's fiber cap,
//! never a silent sample.
//!
//! Everything exploratory tier: exact computed evidence at a declared
//! label, never a promoted status.

use walt_core::{legal_plays, Decl, Domino, DominoSet, Seat, Team, Trick};
use walt_geom::{q, qi, Q};

use crate::scalar::ScalarValuation;

/// One particle: a full deal's remaining hands plus the unit-fraction
/// weight denominator accumulated from field shares.
type Particle = ([DominoSet; Seat::COUNT], u128);

/// The scalar hidden-treatment solver for one (declaration, viewer,
/// focal, valuation) frame.
pub struct ScalarHidden {
    decl: Decl,
    viewer: Seat,
    focal: Team,
    val: ScalarValuation,
}

impl ScalarHidden {
    pub fn new(decl: Decl, viewer: Seat, focal: Team, val: ScalarValuation) -> ScalarHidden {
        ScalarHidden {
            decl,
            viewer,
            focal,
            val,
        }
    }

    /// Exact `Q^H` (future increment, focal-minus-opponent) for every
    /// legal viewer action at the decision, ascending — the viewer to act
    /// is `leader.plus(prefix.len())` and must be this solver's viewer.
    /// `worlds` is the exhaustively enumerated fiber (each world a full
    /// four-seat deal, the viewer's hand identical across them); the root
    /// weighting is uniform over it. `None` when `budget` particle-steps
    /// would be exceeded.
    pub fn action_values(
        &self,
        worlds: &[[DominoSet; Seat::COUNT]],
        leader: Seat,
        prefix: &[Domino],
        budget: &mut u64,
    ) -> Option<Vec<(Domino, Q)>> {
        assert!(!worlds.is_empty(), "a nonempty fiber");
        let k = prefix.len();
        assert!(k < 4, "a decision point sits inside a trick");
        assert_eq!(leader.plus(k), self.viewer, "the root is the viewer's");
        let hand = worlds[0][self.viewer.index()];
        debug_assert!(
            worlds.iter().all(|w| w[self.viewer.index()] == hand),
            "the viewer's hand is common across the fiber"
        );
        let led = (k > 0).then(|| self.decl.led_context(prefix[0]));
        let legal = legal_plays(self.decl, hand, led);
        let mut tiles = [Domino::ALL[0]; 4];
        tiles[..k].copy_from_slice(prefix);
        let n = i128::try_from(worlds.len()).expect("fiber sizes fit i128");
        let mut out = Vec::new();
        for a in legal.iter() {
            let parts: Vec<Particle> = worlds
                .iter()
                .map(|w| {
                    let mut w = *w;
                    w[self.viewer.index()].remove(a);
                    (w, 1u128)
                })
                .collect();
            let mut tiles = tiles;
            tiles[k] = a;
            let v = self.node(&parts, leader, tiles, k + 1, budget)?;
            out.push((a, v * q(1, n)));
        }
        Some(out)
    }

    /// Unnormalized value of one observation node: the sum over particles
    /// of `weight x` (focal future increment given the world), under the
    /// common pooled-information viewer policy below.
    fn node(
        &self,
        parts: &[Particle],
        leader: Seat,
        tiles: [Domino; 4],
        k: usize,
        budget: &mut u64,
    ) -> Option<Q> {
        let cost = parts.len() as u64;
        if *budget < cost {
            return None;
        }
        *budget -= cost;

        if k == 4 {
            let trick = Trick::new(leader, tiles).expect("distinct tiles by construction");
            let winner = trick.winner(self.decl);
            let mut inc = self.val.trick;
            for d in tiles {
                inc += self.val.tiles[d.index()];
            }
            if winner.team() != self.focal {
                inc = -inc;
            }
            let mass: Q = parts.iter().map(|(_, den)| unit(*den)).sum();
            let banked = qi(i128::from(inc)) * mass;
            if parts[0].0.iter().all(|h| h.is_empty()) {
                return Some(banked);
            }
            let rest = self.node(parts, winner, [Domino::ALL[0]; 4], 0, budget)?;
            return Some(banked + rest);
        }

        let seat = leader.plus(k);
        let led = (k > 0).then(|| self.decl.led_context(tiles[0]));

        if seat == self.viewer {
            // A pooled information state: the hand is common, one choice
            // for the whole particle set, maximized exactly.
            let hand = parts[0].0[seat.index()];
            debug_assert!(parts.iter().all(|(w, _)| w[seat.index()] == hand));
            let legal = legal_plays(self.decl, hand, led);
            let mut best: Option<Q> = None;
            for a in legal.iter() {
                let child: Vec<Particle> = parts
                    .iter()
                    .map(|(w, den)| {
                        let mut w = *w;
                        w[seat.index()].remove(a);
                        (w, *den)
                    })
                    .collect();
                let mut tiles = tiles;
                tiles[k] = a;
                let v = self.node(&child, leader, tiles, k + 1, budget)?;
                best = Some(match best {
                    None => v,
                    Some(b) if v > b => v,
                    Some(b) => b,
                });
            }
            return best;
        }

        // A field seat: §7.4 chance. Each particle splits over its own
        // legal set at share 1/|legal|; branching on the observed tile.
        let mut union = DominoSet::EMPTY;
        for (w, _) in parts {
            union = union.union(legal_plays(self.decl, w[seat.index()], led));
        }
        let mut acc = qi(0);
        for m in union.iter() {
            let mut child: Vec<Particle> = Vec::new();
            for (w, den) in parts {
                let lg = legal_plays(self.decl, w[seat.index()], led);
                if lg.contains(m) {
                    let mut w = *w;
                    w[seat.index()].remove(m);
                    child.push((w, den * lg.len() as u128));
                }
            }
            if child.is_empty() {
                continue;
            }
            let mut tiles = tiles;
            tiles[k] = m;
            acc += self.node(&child, leader, tiles, k + 1, budget)?;
        }
        Some(acc)
    }
}

fn unit(den: u128) -> Q {
    q(
        1,
        i128::try_from(den).expect("field-share denominators fit i128"),
    )
}
