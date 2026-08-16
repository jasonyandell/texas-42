//! The optimal-action correspondence of a finite family of envelopes: which
//! indices attain the pointwise maximum, as a function of lambda on the ray.
//!
//! v0.4 §14.2 census vocabulary: the correspondence can gain members at an
//! isolated point (a boundary tie resolved at `0+` is the reported example),
//! so the canonical form records the argmax set exactly at each event point
//! and on the open interval after it.

use std::collections::BTreeSet;

use crate::envelope::Envelope;
use crate::rat::{q, qi, Q};

/// Canonical form of the argmax correspondence. Sets are bitmasks over the
/// family index (so the family is limited to 32 envelopes; a seat holds at
/// most 7 tiles). Two correspondences are equal as functions iff they are
/// equal as values.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArgmaxCorrespondence {
    /// Event points, strictly increasing, starting at 0.
    pub points: Vec<Q>,
    /// The argmax set exactly at `points[i]`.
    pub at_point: Vec<u32>,
    /// The argmax set on the open interval `(points[i], points[i+1])`, or
    /// `(points[last], inf)` for the final entry.
    pub after: Vec<u32>,
}

impl ArgmaxCorrespondence {
    /// The argmax set at lambda = 0: the baseline optimal actions.
    pub fn baseline(&self) -> u32 {
        self.at_point[0]
    }
}

fn argmax_mask(envelopes: &[Envelope], upper: &Envelope, x: Q) -> u32 {
    let top = upper.eval(x);
    let mut mask = 0u32;
    for (i, e) in envelopes.iter().enumerate() {
        if e.eval(x) == top {
            mask |= 1u32 << i;
        }
    }
    mask
}

/// The candidate event points: every piece start of every envelope and of the
/// upper envelope, plus every strict-interior crossing between an envelope's
/// line and the upper envelope's line on their common refinement. Between
/// consecutive candidates the sign of `upper - e_i` cannot change for any `i`,
/// so the argmax set is constant on each open gap.
fn candidates(envelopes: &[Envelope], upper: &Envelope) -> Vec<Q> {
    let mut out: BTreeSet<Q> = BTreeSet::new();
    out.insert(qi(0));
    for p in upper.pieces() {
        out.insert(p.lo);
    }
    for e in envelopes {
        for p in e.pieces() {
            out.insert(p.lo);
        }
        let mut cuts: Vec<Q> = upper
            .pieces()
            .iter()
            .chain(e.pieces().iter())
            .map(|p| p.lo)
            .collect();
        cuts.sort();
        cuts.dedup();
        for (i, &s) in cuts.iter().enumerate() {
            let end = cuts.get(i + 1).copied();
            let lu = upper.pieces()[upper.owner_index(s)].line;
            let le = e.pieces()[e.owner_index(s)].line;
            if let Some(x) = lu.crossing(&le) {
                if s < x && end.is_none_or(|t| x < t) {
                    out.insert(x);
                }
            }
        }
    }
    out.into_iter().collect()
}

/// The optimal-action correspondence of a nonempty family (at most 32).
pub fn argmax_correspondence(envelopes: &[Envelope]) -> ArgmaxCorrespondence {
    assert!(
        !envelopes.is_empty() && envelopes.len() <= 32,
        "a nonempty family of at most 32 envelopes"
    );
    let upper = Envelope::max_of(envelopes.iter().cloned());
    let points = candidates(envelopes, &upper);

    let mut at_point: Vec<u32> = Vec::with_capacity(points.len());
    let mut after: Vec<u32> = Vec::with_capacity(points.len());
    for (i, &p) in points.iter().enumerate() {
        at_point.push(argmax_mask(envelopes, &upper, p));
        let probe = match points.get(i + 1) {
            Some(&next) => (p + next) * q(1, 2),
            None => p + qi(1),
        };
        after.push(argmax_mask(envelopes, &upper, probe));
    }

    // Canonicalize: an interior candidate where nothing changes is not an
    // event. The origin always stays.
    let mut c = ArgmaxCorrespondence {
        points: vec![points[0]],
        at_point: vec![at_point[0]],
        after: vec![after[0]],
    };
    for i in 1..points.len() {
        let prev_after = *c.after.last().expect("nonempty by construction");
        if at_point[i] == prev_after && after[i] == prev_after {
            continue;
        }
        c.points.push(points[i]);
        c.at_point.push(at_point[i]);
        c.after.push(after[i]);
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;

    fn line(a: Q, b: Q) -> Envelope {
        Envelope::line(Line::new(a, b))
    }

    #[test]
    fn one_envelope_is_always_optimal() {
        let c = argmax_correspondence(&[line(qi(2), q(1, 3))]);
        assert_eq!(c.points, vec![qi(0)]);
        assert_eq!(c.at_point, vec![0b1]);
        assert_eq!(c.after, vec![0b1]);
    }

    #[test]
    fn a_strict_crossing_is_a_three_phase_event() {
        // Flat wins, ties at 1, rising wins.
        let c = argmax_correspondence(&[line(qi(1), qi(0)), line(qi(0), qi(1))]);
        assert_eq!(c.points, vec![qi(0), qi(1)]);
        assert_eq!(c.at_point, vec![0b01, 0b11]);
        assert_eq!(c.after, vec![0b01, 0b10]);
        assert_eq!(c.baseline(), 0b01);
    }

    #[test]
    fn a_boundary_tie_resolves_at_zero_plus() {
        // Tied at 0; index 1 strictly optimal for every lambda > 0. This is
        // the §14.2 tie shape, and it needs the at-point/after split.
        let c = argmax_correspondence(&[line(qi(0), qi(0)), line(qi(0), qi(1))]);
        assert_eq!(c.points, vec![qi(0)]);
        assert_eq!(c.at_point, vec![0b11]);
        assert_eq!(c.after, vec![0b10]);
    }

    #[test]
    fn an_isolated_interior_tangency_is_kept_as_an_event() {
        // The hinge max(0, lambda-1) and the flat 0 coincide on [0, 1]; the
        // hinge wins alone after 1. The event at 1 separates tie from win.
        let hinge = line(qi(0), qi(0)).max_with(&line(qi(-1), qi(1)));
        let c = argmax_correspondence(&[hinge, line(qi(0), qi(0))]);
        assert_eq!(c.points, vec![qi(0), qi(1)]);
        assert_eq!(c.at_point, vec![0b11, 0b11]);
        assert_eq!(c.after, vec![0b11, 0b01]);
    }

    #[test]
    fn equal_envelopes_collapse_to_one_event() {
        let e = line(q(1, 7), q(-2, 5));
        let c = argmax_correspondence(&[e.clone(), e]);
        assert_eq!(c.points, vec![qi(0)]);
        assert_eq!(c.at_point, vec![0b11]);
        assert_eq!(c.after, vec![0b11]);
    }
}
