//! Exhaustive small-case audit of the envelope `_combine` merge.
//!
//! The reference semantics is scalar: an envelope built as max/min/sums of
//! lines must evaluate, at every probe point, to exactly the max/min/sum of
//! the line evaluations. The probe battery hits every breakpoint (the owned
//! endpoint), points just left and right of it, midpoints, the origin, and a
//! point beyond the last breakpoint -- the places v0.4 §14.1's disclosed
//! interval-endpoint bug lived.

use walt::geom::{q, qi, Envelope, Line, Q};

/// The small rational coefficient grid. 5 x 5 = 25 lines.
fn grid() -> Vec<Q> {
    vec![qi(-2), qi(-1), qi(0), q(1, 2), qi(2)]
}

fn lines() -> Vec<Line> {
    let mut out = Vec::new();
    for a in grid() {
        for b in grid() {
            out.push(Line::new(a, b));
        }
    }
    out
}

/// Probe points for one envelope: 0, every breakpoint, its shifted
/// neighborhoods (clamped to the ray), midpoints of adjacent breakpoints, and
/// a point past the last one.
fn probes(e: &Envelope) -> Vec<Q> {
    let eps = q(1, 7);
    let mut out = vec![qi(0)];
    let bps: Vec<Q> = e.breakpoints().collect();
    for &t in &bps {
        out.push(t);
        out.push(t + eps);
        if t - eps >= qi(0) {
            out.push(t - eps);
        }
    }
    for w in bps.windows(2) {
        out.push((w[0] + w[1]) * q(1, 2));
    }
    out.push(bps.last().copied().unwrap_or(qi(0)) + qi(3));
    out
}

fn check_against_lines(e: &Envelope, family: &[Line], upper: bool) {
    e.assert_invariants();
    for x in probes(e) {
        let want = family
            .iter()
            .map(|l| l.eval(x))
            .reduce(|p, v| if upper { p.max(v) } else { p.min(v) })
            .expect("nonempty family");
        assert_eq!(
            e.eval(x),
            want,
            "envelope disagrees with the {} of {family:?} at {x}",
            if upper { "max" } else { "min" },
        );
    }
}

#[test]
fn every_pair_of_lines_merges_exactly() {
    let lines = lines();
    for l1 in &lines {
        for l2 in &lines {
            let (a, b) = (Envelope::line(*l1), Envelope::line(*l2));
            let family = [*l1, *l2];
            check_against_lines(&a.max_with(&b), &family, true);
            check_against_lines(&a.min_with(&b), &family, false);
            // Commutativity: canonical forms are unique, so equality is
            // function equality.
            assert_eq!(a.max_with(&b), b.max_with(&a));
            assert_eq!(a.min_with(&b), b.min_with(&a));
        }
    }
}

#[test]
fn every_triple_of_lines_merges_exactly_and_associatively() {
    let lines = lines();
    for l1 in &lines {
        for l2 in &lines {
            for l3 in &lines {
                let family = [*l1, *l2, *l3];
                let es: Vec<Envelope> = family.iter().map(|l| Envelope::line(*l)).collect();
                let left = es[0].max_with(&es[1]).max_with(&es[2]);
                let right = es[0].max_with(&es[1].max_with(&es[2]));
                assert_eq!(left, right, "max is associative on {family:?}");
                check_against_lines(&left, &family, true);
                let left = es[0].min_with(&es[1]).min_with(&es[2]);
                let right = es[0].min_with(&es[1].min_with(&es[2]));
                assert_eq!(left, right, "min is associative on {family:?}");
                check_against_lines(&left, &family, false);
            }
        }
    }
}

/// Merges of already-piecewise envelopes: max of pairwise minima, min of
/// pairwise maxima, and sums, over a subsampled family so the case count
/// stays honest but bounded.
#[test]
fn merges_of_piecewise_envelopes_stay_exact() {
    let lines = lines();
    // Every 3rd line keeps 9 of the 25: 81 pairs of pairs.
    let sub: Vec<Line> = lines.iter().copied().step_by(3).collect();
    for l1 in &sub {
        for l2 in &sub {
            for l3 in &sub {
                for l4 in &sub {
                    let lower = Envelope::line(*l1).min_with(&Envelope::line(*l2));
                    let upper = Envelope::line(*l3).max_with(&Envelope::line(*l4));
                    let family = [*l1, *l2, *l3, *l4];

                    let e = lower.max_with(&upper);
                    e.assert_invariants();
                    for x in probes(&e) {
                        let want = (l1.eval(x).min(l2.eval(x))).max(l3.eval(x).max(l4.eval(x)));
                        assert_eq!(e.eval(x), want, "max(min, max) at {x} over {family:?}");
                    }

                    let e = upper.min_with(&lower);
                    e.assert_invariants();
                    for x in probes(&e) {
                        let want = (l3.eval(x).max(l4.eval(x))).min(l1.eval(x).min(l2.eval(x)));
                        assert_eq!(e.eval(x), want, "min(max, min) at {x} over {family:?}");
                    }

                    let e = lower.add(&upper);
                    e.assert_invariants();
                    for x in probes(&e) {
                        let want = l1.eval(x).min(l2.eval(x)) + l3.eval(x).max(l4.eval(x));
                        assert_eq!(e.eval(x), want, "min + max at {x} over {family:?}");
                    }
                }
            }
        }
    }
}

/// Endpoint ownership, stated directly: every breakpoint is owned by exactly
/// one piece (the one that starts there), the pieces tile the ray with no
/// gaps or overlaps, and the function is continuous across each boundary.
#[test]
fn endpoint_ownership_holds_on_every_generated_envelope() {
    let lines = lines();
    let mut multisegment = 0usize;
    for l1 in &lines {
        for l2 in &lines {
            for e in [
                Envelope::line(*l1).max_with(&Envelope::line(*l2)),
                Envelope::line(*l1).min_with(&Envelope::line(*l2)),
            ] {
                e.assert_invariants();
                if e.pieces().len() > 1 {
                    multisegment += 1;
                }
                for (i, p) in e.pieces().iter().enumerate() {
                    // The piece that starts at lo owns lo...
                    assert_eq!(e.owner_index(p.lo), i);
                    // ...and its value there is the envelope's value.
                    assert_eq!(e.eval(p.lo), p.line.eval(p.lo));
                }
                for w in e.pieces().windows(2) {
                    // Just left of a boundary the left piece still owns.
                    let inside = (w[0].lo + w[1].lo) * q(1, 2);
                    assert_eq!(e.pieces()[e.owner_index(inside)], w[0]);
                    // Left and right pieces agree at the boundary: no jump
                    // hides in the ownership convention.
                    assert_eq!(w[0].line.eval(w[1].lo), w[1].line.eval(w[1].lo));
                }
            }
        }
    }
    // The grid must actually exercise multisegment envelopes.
    assert!(multisegment > 0, "the grid produced only affine merges");
}
