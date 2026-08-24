//! Continuous piecewise-linear envelopes on the ray `lambda in [0, inf)`.
//!
//! Endpoint ownership is the type invariant, not a convention the operations
//! must each remember: piece `i` owns the half-open interval
//! `[lo_i, lo_{i+1})` and the last piece owns `[lo_last, inf)`, so every
//! endpoint is owned by exactly one piece, with no overlaps and no gaps.
//! v0.4 §14.1 discloses a piecewise-linear interval-endpoint bug found by the
//! first multisegment domain; this representation makes that bug
//! unrepresentable, and `assert_invariants` makes the rest of the contract
//! (strictly increasing starts, continuity at every breakpoint, no redundant
//! pieces) checkable in every test.

use crate::geom::line::Line;
use crate::geom::rat::{qi, Q};

/// One affine piece and the start of the half-open interval it owns.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Piece {
    pub lo: Q,
    pub line: Line,
}

/// A continuous piecewise-linear function on `[0, inf)` in canonical form:
/// first piece starts at 0, starts strictly increase, adjacent pieces carry
/// distinct lines, and the function is continuous at every breakpoint.
/// Equality of envelopes is equality of functions, because the canonical form
/// is unique.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Envelope {
    pieces: Vec<Piece>,
}

/// Whether `combine` keeps the upper or the lower of the two functions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sense {
    Max,
    Min,
}

impl Sense {
    /// The line that wins immediately to the right of a point where the two
    /// values are equal: slopes decide.
    fn right_winner(self, la: Line, lb: Line) -> Line {
        let a_wins = match self {
            Sense::Max => la.b > lb.b,
            Sense::Min => la.b < lb.b,
        };
        if a_wins {
            la
        } else {
            lb
        }
    }

    fn value_winner(self, la: Line, lb: Line, at: Q) -> Line {
        let (va, vb) = (la.eval(at), lb.eval(at));
        if va == vb {
            return self.right_winner(la, lb);
        }
        let a_wins = match self {
            Sense::Max => va > vb,
            Sense::Min => va < vb,
        };
        if a_wins {
            la
        } else {
            lb
        }
    }
}

/// Appends a piece, merging it into the previous one when the line repeats.
fn push(pieces: &mut Vec<Piece>, lo: Q, line: Line) {
    if let Some(last) = pieces.last() {
        if last.line == line {
            return;
        }
        assert!(last.lo < lo, "piece starts must strictly increase");
        assert_eq!(
            last.line.eval(lo),
            line.eval(lo),
            "envelope discontinuity at {lo}"
        );
    } else {
        assert_eq!(lo, qi(0), "the first piece must own 0");
    }
    pieces.push(Piece { lo, line });
}

impl Envelope {
    /// The envelope that is one line everywhere.
    pub fn line(line: Line) -> Envelope {
        Envelope {
            pieces: vec![Piece { lo: qi(0), line }],
        }
    }

    pub fn zero() -> Envelope {
        Envelope::line(Line::zero())
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    /// The interior breakpoints: every `lo` after the first.
    pub fn breakpoints(&self) -> impl Iterator<Item = Q> + '_ {
        self.pieces.iter().skip(1).map(|p| p.lo)
    }

    pub fn is_affine(&self) -> bool {
        self.pieces.len() == 1
    }

    /// The index of the unique piece owning `x`. Panics off the ray.
    pub fn owner_index(&self, x: Q) -> usize {
        assert!(x >= qi(0), "envelopes live on [0, inf)");
        self.pieces.partition_point(|p| p.lo <= x) - 1
    }

    pub fn eval(&self, x: Q) -> Q {
        self.pieces[self.owner_index(x)].line.eval(x)
    }

    /// Every canonical-form invariant, spelled out for the tests. The
    /// operations maintain these by construction (`push` asserts as it goes);
    /// this is the independent audit.
    pub fn assert_invariants(&self) {
        assert!(
            !self.pieces.is_empty(),
            "an envelope has at least one piece"
        );
        assert_eq!(self.pieces[0].lo, qi(0), "the first piece owns 0");
        for w in self.pieces.windows(2) {
            let (prev, next) = (w[0], w[1]);
            assert!(prev.lo < next.lo, "starts strictly increase");
            assert_ne!(prev.line, next.line, "adjacent pieces carry one line");
            assert_eq!(
                prev.line.eval(next.lo),
                next.line.eval(next.lo),
                "continuous at every breakpoint"
            );
        }
    }

    /// The merged, deduplicated piece starts of two envelopes.
    fn cuts(&self, other: &Envelope) -> Vec<Q> {
        let mut cuts: Vec<Q> = self
            .pieces
            .iter()
            .chain(other.pieces.iter())
            .map(|p| p.lo)
            .collect();
        cuts.sort();
        cuts.dedup();
        cuts
    }

    /// Pointwise sum.
    pub fn add(&self, other: &Envelope) -> Envelope {
        let mut pieces = Vec::new();
        for s in self.cuts(other) {
            let la = self.pieces[self.owner_index(s)].line;
            let lb = other.pieces[other.owner_index(s)].line;
            push(&mut pieces, s, la.add(&lb));
        }
        Envelope { pieces }
    }

    pub fn add_line(&self, line: &Line) -> Envelope {
        self.add(&Envelope::line(*line))
    }

    /// Pointwise difference `self - other`.
    pub fn sub(&self, other: &Envelope) -> Envelope {
        self.add(&other.scale(qi(-1)))
    }

    /// Pointwise sum of a finite family, reduced pairwise: a left-to-right
    /// running sum carries the union of every breakpoint seen so far into
    /// each partial, so summing n envelopes costs n times the union size;
    /// balancing keeps intermediate piece counts near the summand sizes.
    pub fn sum_of<I: IntoIterator<Item = Envelope>>(terms: I) -> Envelope {
        let mut layer: Vec<Envelope> = terms.into_iter().collect();
        if layer.is_empty() {
            return Envelope::zero();
        }
        while layer.len() > 1 {
            let mut next = Vec::with_capacity(layer.len().div_ceil(2));
            let mut it = layer.into_iter();
            while let Some(a) = it.next() {
                match it.next() {
                    Some(b) => next.push(a.add(&b)),
                    None => next.push(a),
                }
            }
            layer = next;
        }
        layer.pop().expect("nonempty by the early return")
    }

    /// Whether the function is nonnegative on the whole ray: every piece is
    /// nonnegative where it starts (continuity carries the check across each
    /// breakpoint) and the final piece does not descend forever.
    pub fn is_nonnegative(&self) -> bool {
        self.pieces.iter().all(|p| p.line.eval(p.lo) >= qi(0))
            && self.pieces.last().expect("an envelope is nonempty").line.b >= qi(0)
    }

    /// Pointwise scale. A zero scale collapses to the zero envelope; a
    /// negative scale is still a valid continuous PWL function.
    pub fn scale(&self, c: Q) -> Envelope {
        let mut pieces = Vec::new();
        for p in &self.pieces {
            push(&mut pieces, p.lo, p.line.scale(c));
        }
        Envelope { pieces }
    }

    /// The `_combine` merge: on each interval of the common refinement both
    /// functions are single lines, so the winner changes only at a strict
    /// interior crossing, which becomes a new breakpoint.
    fn combine(&self, other: &Envelope, sense: Sense) -> Envelope {
        let cuts = self.cuts(other);
        let mut pieces = Vec::new();
        for (i, &s) in cuts.iter().enumerate() {
            let end = cuts.get(i + 1).copied();
            let la = self.pieces[self.owner_index(s)].line;
            let lb = other.pieces[other.owner_index(s)].line;
            if la == lb {
                push(&mut pieces, s, la);
                continue;
            }
            match la.crossing(&lb) {
                Some(x) if s < x && end.is_none_or(|e| x < e) => {
                    // The values differ at `s` (a crossing at `s` itself would
                    // fail `s < x`), so the winner on `[s, x)` is by value and
                    // the other line takes over at `x`.
                    let first = sense.value_winner(la, lb, s);
                    let second = if first == la { lb } else { la };
                    push(&mut pieces, s, first);
                    push(&mut pieces, x, second);
                }
                _ => push(&mut pieces, s, sense.value_winner(la, lb, s)),
            }
        }
        Envelope { pieces }
    }

    /// Pointwise maximum of two envelopes. Named `max_with` so `Ord::max`
    /// (which compares whole envelopes as values) can never shadow it.
    pub fn max_with(&self, other: &Envelope) -> Envelope {
        self.combine(other, Sense::Max)
    }

    /// Pointwise minimum of two envelopes.
    pub fn min_with(&self, other: &Envelope) -> Envelope {
        self.combine(other, Sense::Min)
    }

    /// Pointwise maximum of a nonempty family.
    pub fn max_of<I: IntoIterator<Item = Envelope>>(family: I) -> Envelope {
        family
            .into_iter()
            .reduce(|acc, e| acc.max_with(&e))
            .expect("a maximum needs at least one envelope")
    }

    /// Pointwise minimum of a nonempty family.
    pub fn min_of<I: IntoIterator<Item = Envelope>>(family: I) -> Envelope {
        family
            .into_iter()
            .reduce(|acc, e| acc.min_with(&e))
            .expect("a minimum needs at least one envelope")
    }

    /// The fixed linear-expectation operator of v0.4 §9.9: `sum p_i E_i`.
    /// The weights are not required to sum to one; a chance node passes its
    /// probabilities, an accumulator may pass plain coefficients.
    pub fn affine_combination<'a, I>(terms: I) -> Envelope
    where
        I: IntoIterator<Item = (Q, &'a Envelope)>,
    {
        let mut acc = Envelope::zero();
        for (p, e) in terms {
            acc = acc.add(&e.scale(p));
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::rat::q;

    #[test]
    fn a_single_line_owns_the_whole_ray() {
        let e = Envelope::line(Line::new(q(1, 3), qi(-2)));
        e.assert_invariants();
        assert!(e.is_affine());
        assert_eq!(e.owner_index(qi(0)), 0);
        assert_eq!(e.owner_index(qi(1000)), 0);
        assert_eq!(e.eval(q(1, 2)), q(1, 3) - qi(1));
    }

    #[test]
    fn max_of_two_crossing_lines_breaks_at_the_crossing() {
        let rising = Envelope::line(Line::new(qi(0), qi(1)));
        let flat = Envelope::line(Line::new(qi(1), qi(0)));
        let e = rising.max_with(&flat);
        e.assert_invariants();
        assert_eq!(e.pieces().len(), 2);
        assert_eq!(e.pieces()[0].line, Line::new(qi(1), qi(0)));
        assert_eq!(
            e.pieces()[1],
            Piece {
                lo: qi(1),
                line: Line::new(qi(0), qi(1)),
            }
        );
        // The breakpoint is owned by the rising piece alone.
        assert_eq!(e.owner_index(qi(1)), 1);
        assert_eq!(e.eval(qi(1)), qi(1));
        let m = rising.min_with(&flat);
        m.assert_invariants();
        assert_eq!(m.pieces()[0].line, Line::new(qi(0), qi(1)));
        assert_eq!(m.pieces()[1].lo, qi(1));
    }

    #[test]
    fn a_crossing_left_of_the_ray_never_breaks() {
        // These lines cross at -5/2 (the §14.2 pair); on the ray one wins.
        let top = Envelope::line(Line::new(q(2, 3), q(1, 5)));
        let bottom = Envelope::line(Line::new(q(-2, 3), q(-1, 3)));
        assert_eq!(
            Line::new(q(2, 3), q(1, 5)).crossing(&Line::new(q(-2, 3), q(-1, 3))),
            Some(q(-5, 2))
        );
        let e = top.max_with(&bottom);
        assert!(e.is_affine());
        assert_eq!(e, top);
        assert_eq!(bottom.min_with(&top), bottom);
    }

    #[test]
    fn a_tie_at_zero_is_broken_by_slope() {
        let rising = Envelope::line(Line::new(qi(0), qi(1)));
        let falling = Envelope::line(Line::new(qi(0), qi(-1)));
        assert_eq!(rising.max_with(&falling), rising);
        assert_eq!(rising.min_with(&falling), falling);
    }

    #[test]
    fn add_cancels_to_the_zero_envelope() {
        let rising = Envelope::line(Line::new(qi(0), qi(1)));
        let flat = Envelope::line(Line::new(qi(1), qi(0)));
        let e = rising.max_with(&flat);
        let sum = e.add(&e.scale(qi(-1)));
        sum.assert_invariants();
        assert_eq!(sum, Envelope::zero());
    }

    #[test]
    fn affine_combination_of_lines_is_the_weighted_line() {
        let a = Envelope::line(Line::new(qi(1), qi(0)));
        let b = Envelope::line(Line::new(qi(0), qi(1)));
        let e = Envelope::affine_combination([(q(1, 3), &a), (q(2, 3), &b)]);
        assert_eq!(e, Envelope::line(Line::new(q(1, 3), q(2, 3))));
    }

    #[test]
    #[should_panic(expected = "[0, inf)")]
    fn evaluation_off_the_ray_is_refused() {
        Envelope::zero().eval(qi(-1));
    }

    #[test]
    fn sub_of_itself_is_zero_and_sub_undoes_add() {
        let hinge = Envelope::line(Line::new(qi(0), qi(0)))
            .max_with(&Envelope::line(Line::new(qi(-1), qi(1))));
        assert_eq!(hinge.sub(&hinge), Envelope::zero());
        let line = Envelope::line(Line::new(q(1, 3), q(-2, 7)));
        assert_eq!(hinge.add(&line).sub(&line), hinge);
    }

    #[test]
    fn sum_of_matches_sequential_addition() {
        let a = Envelope::line(Line::new(qi(1), qi(0)));
        let b = Envelope::line(Line::new(qi(0), qi(1))).max_with(&a);
        let c = Envelope::line(Line::new(q(1, 2), q(1, 3)));
        let family = [a.clone(), b.clone(), c.clone(), b.clone()];
        let sequential = a.add(&b).add(&c).add(&b);
        assert_eq!(Envelope::sum_of(family), sequential);
        assert_eq!(Envelope::sum_of([]), Envelope::zero());
        assert_eq!(Envelope::sum_of([c.clone()]), c);
    }

    #[test]
    fn nonnegativity_checks_starts_and_the_tail() {
        let hinge = Envelope::line(Line::new(qi(0), qi(0)))
            .max_with(&Envelope::line(Line::new(qi(-1), qi(1))));
        assert!(hinge.is_nonnegative());
        assert!(Envelope::zero().is_nonnegative());
        // Positive at 0, but the tail descends forever.
        assert!(!Envelope::line(Line::new(qi(5), qi(-1))).is_nonnegative());
        // Nonnegative tail, negative start.
        assert!(!Envelope::line(Line::new(qi(-1), qi(1))).is_nonnegative());
    }
}
