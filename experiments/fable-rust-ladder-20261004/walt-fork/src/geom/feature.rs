//! The 29-dimensional additive feature space (v0.4 §8.1--§8.2) and finite
//! feature sets with their support functions (§9.1--§9.2, §9.4).
//!
//! One shape serves both sides of the pairing: a `FeatureVec` is a point
//! (trick count and per-tile capture coordinates) or a valuation coefficient
//! vector `(b, w)`; `dot` is the evaluation `b*t + sum w(d) x_d`. The action
//! polytope of §9.1 is carried finite-first (§16.1) as its generating set of
//! expected-feature points -- a support function has the same maximum over a
//! finite set and its convex hull, so the hull is never materialized.

use crate::rules::Domino;

use crate::geom::envelope::Envelope;
use crate::geom::line::Line;
use crate::geom::rat::{qi, Q};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FeatureVec {
    /// The trick coordinate `t`, or the trick coefficient `b`.
    pub tricks: Q,
    /// The capture coordinate `x_d` per physical domino, or the tile
    /// valuation `w(d)`, indexed by `Domino::index`.
    pub tiles: [Q; Domino::COUNT],
}

impl FeatureVec {
    pub fn zero() -> FeatureVec {
        FeatureVec {
            tricks: qi(0),
            tiles: [qi(0); Domino::COUNT],
        }
    }

    /// The pure trick direction: `b = 1`, `w = 0`.
    pub fn unit_trick() -> FeatureVec {
        FeatureVec {
            tricks: qi(1),
            ..FeatureVec::zero()
        }
    }

    /// The one-tile direction `e_d` of §9.4.
    pub fn unit_tile(d: Domino) -> FeatureVec {
        let mut out = FeatureVec::zero();
        out.tiles[d.index()] = qi(1);
        out
    }

    /// One step along the additive gauge line of §8.3:
    /// `(b, w) ~ (b - 4c, w + c*1)` on every legal terminal feature.
    pub fn gauge_shift(c: Q) -> FeatureVec {
        FeatureVec {
            tricks: qi(-4) * c,
            tiles: [c; Domino::COUNT],
        }
    }

    pub fn dot(&self, other: &FeatureVec) -> Q {
        let mut out = self.tricks * other.tricks;
        for (a, b) in self.tiles.iter().zip(other.tiles.iter()) {
            out += *a * *b;
        }
        out
    }

    pub fn add(&self, other: &FeatureVec) -> FeatureVec {
        let mut out = self.clone();
        out.tricks += other.tricks;
        for (a, b) in out.tiles.iter_mut().zip(other.tiles.iter()) {
            *a += *b;
        }
        out
    }

    pub fn scale(&self, c: Q) -> FeatureVec {
        let mut out = self.clone();
        out.tricks *= c;
        for a in out.tiles.iter_mut() {
            *a *= c;
        }
        out
    }

    /// The universal legal identity of §8.1: `sum_d x_d = 4t`. Holds for the
    /// capture feature of any consistently measured continuation; it is what
    /// makes the gauge line invisible to legal features.
    pub fn satisfies_capture_identity(&self) -> bool {
        self.tiles.iter().copied().sum::<Q>() == qi(4) * self.tricks
    }
}

/// A finite set of feature points: the generating set of a policy feature
/// polytope (§9.1), kept as points per the finite-first strategy (§16.1).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct FeatureSet {
    points: Vec<FeatureVec>,
}

impl FeatureSet {
    pub fn new() -> FeatureSet {
        FeatureSet::default()
    }

    pub fn push(&mut self, x: FeatureVec) {
        self.points.push(x);
    }

    pub fn points(&self) -> &[FeatureVec] {
        &self.points
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// The support function `h(v) = max_x <v, x>` (§9.2). Panics on the empty
    /// set, whose support function is nowhere finite.
    pub fn support(&self, v: &FeatureVec) -> Q {
        self.points
            .iter()
            .map(|x| v.dot(x))
            .max()
            .expect("the empty set has no support function")
    }

    /// The one-parameter support envelope along the ray `v0 + lambda*u`
    /// (§9.4): each point contributes the line `<v0,x> + <u,x>*lambda`, and
    /// the envelope is their pointwise maximum.
    pub fn support_envelope(&self, v0: &FeatureVec, u: &FeatureVec) -> Envelope {
        Envelope::max_of(
            self.points
                .iter()
                .map(|x| Envelope::line(Line::new(v0.dot(x), u.dot(x)))),
        )
    }

    /// The generators attaining the support in direction `v`: where the
    /// exposed face `F(v)` of §10.6 meets the generating set, finite-first.
    pub fn exposed(&self, v: &FeatureVec) -> Vec<&FeatureVec> {
        let top = self.support(v);
        self.points.iter().filter(|x| v.dot(x) == top).collect()
    }
}

/// The §10.6 zero-information criterion, finite-first. Precondition (not
/// checkable from generators alone): `conv(sub)` is included in `conv(sup)`,
/// as the policy-gluing inclusion §10.2 guarantees for information-polytope
/// pairs. Then the supports agree at `v` iff some generator of `sub` lies on
/// `sup`'s exposed face `F(v)` -- an exposed optimal feature of the larger
/// class is already implementable in the smaller one.
pub fn zero_information_at(sub: &FeatureSet, sup: &FeatureSet, v: &FeatureVec) -> bool {
    let top = sup.support(v);
    sub.points().iter().any(|x| v.dot(x) == top)
}

impl FromIterator<FeatureVec> for FeatureSet {
    fn from_iter<I: IntoIterator<Item = FeatureVec>>(iter: I) -> FeatureSet {
        FeatureSet {
            points: iter.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::rat::q;

    /// A legal-shaped terminal feature: `t` tricks and exactly `4t` captured
    /// tiles, the first `4t` dominoes for definiteness.
    fn legal_feature(t: usize) -> FeatureVec {
        let mut out = FeatureVec::zero();
        out.tricks = qi(t as i128);
        for i in 0..4 * t {
            out.tiles[i] = qi(1);
        }
        out
    }

    #[test]
    fn the_capture_identity_recognizes_legal_features() {
        for t in 0..=7 {
            assert!(legal_feature(t).satisfies_capture_identity());
        }
        let mut broken = legal_feature(2);
        broken.tiles[27] = qi(1);
        assert!(!broken.satisfies_capture_identity());
    }

    #[test]
    fn the_gauge_line_is_invisible_on_legal_features() {
        let v = FeatureVec::unit_trick().add(&FeatureVec::unit_tile(Domino::ALL[5]).scale(qi(3)));
        let shifted = v.add(&FeatureVec::gauge_shift(q(7, 3)));
        for t in 0..=7 {
            let x = legal_feature(t);
            assert_eq!(v.dot(&x), shifted.dot(&x));
        }
        // And it is visible on an illegal one.
        let mut broken = legal_feature(1);
        broken.tricks = qi(2);
        assert_ne!(v.dot(&broken), shifted.dot(&broken));
    }

    #[test]
    fn support_is_the_max_over_points_and_matches_the_envelope() {
        let d = Domino::ALL[9];
        let set: FeatureSet = (0..=3).map(legal_feature).collect();
        let v0 = FeatureVec::unit_trick();
        let u = FeatureVec::unit_tile(d);
        let env = set.support_envelope(&v0, &u);
        env.assert_invariants();
        for lambda in [qi(0), q(1, 2), qi(1), qi(10)] {
            let v = v0.add(&u.scale(lambda));
            assert_eq!(set.support(&v), env.eval(lambda));
        }
    }

    #[test]
    fn zero_information_iff_an_exposed_generator_is_implementable() {
        // sup generates the segment hull of t=0 and t=3 features; sub holds
        // only the t=0 endpoint, so conv(sub) is a face of conv(sup).
        let low = legal_feature(0);
        let high = legal_feature(3);
        let sub: FeatureSet = [low.clone()].into_iter().collect();
        let sup: FeatureSet = [low.clone(), high.clone()].into_iter().collect();

        // Directions rewarding tricks expose `high`, which sub cannot reach.
        let up = FeatureVec::unit_trick();
        assert!(!zero_information_at(&sub, &sup, &up));
        assert_ne!(sub.support(&up), sup.support(&up));
        assert_eq!(sup.exposed(&up), vec![&high]);

        // The opposite direction exposes `low`, which sub implements, and the
        // criterion agrees with exact support equality.
        let down = up.scale(qi(-1));
        assert!(zero_information_at(&sub, &sup, &down));
        assert_eq!(sub.support(&down), sup.support(&down));

        // A direction blind to the difference (a capture coordinate neither
        // point holds) exposes both generators; supports agree at 0.
        let blind = FeatureVec::unit_tile(Domino::ALL[27]);
        assert!(zero_information_at(&sub, &sup, &blind));
        assert_eq!(sup.exposed(&blind).len(), 2);
    }

    #[test]
    fn a_dominated_point_never_moves_the_support() {
        // §9.6 shape: y dominates x on the nonnegative cone.
        let y = legal_feature(3);
        let x = legal_feature(1);
        let with: FeatureSet = [y.clone(), x].into_iter().collect();
        let without: FeatureSet = [y].into_iter().collect();
        let v0 = FeatureVec::unit_trick();
        for i in 0..Domino::COUNT {
            let u = FeatureVec::unit_tile(Domino::ALL[i]);
            assert_eq!(
                with.support_envelope(&v0, &u),
                without.support_envelope(&v0, &u)
            );
        }
    }
}
