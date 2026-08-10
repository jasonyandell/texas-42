//! Affine functions `a + b*lambda` of the one valuation parameter (v0.4 §9.4:
//! each policy contributes one line; §9.9: leaves of the symbolic recursion).

use crate::rat::{qi, Q};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Line {
    /// Value at `lambda = 0`.
    pub a: Q,
    /// Slope in `lambda`.
    pub b: Q,
}

impl Line {
    pub fn new(a: Q, b: Q) -> Line {
        Line { a, b }
    }

    pub fn zero() -> Line {
        Line { a: qi(0), b: qi(0) }
    }

    pub fn eval(&self, x: Q) -> Q {
        self.a + self.b * x
    }

    pub fn add(&self, other: &Line) -> Line {
        Line {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }

    pub fn scale(&self, c: Q) -> Line {
        Line {
            a: self.a * c,
            b: self.b * c,
        }
    }

    /// The unique crossing parameter with `other`, when the slopes differ.
    /// Parallel (including equal) lines have none.
    pub fn crossing(&self, other: &Line) -> Option<Q> {
        if self.b == other.b {
            None
        } else {
            Some((other.a - self.a) / (self.b - other.b))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rat::q;

    #[test]
    fn eval_add_scale() {
        let l = Line::new(q(1, 2), q(-1, 3));
        assert_eq!(l.eval(qi(0)), q(1, 2));
        assert_eq!(l.eval(qi(3)), q(-1, 2));
        let m = l.add(&Line::new(q(1, 2), q(1, 3)));
        assert_eq!(m, Line::new(qi(1), qi(0)));
        assert_eq!(l.scale(qi(-6)), Line::new(qi(-3), qi(2)));
    }

    #[test]
    fn crossing_is_where_the_lines_meet() {
        let l = Line::new(qi(0), qi(1));
        let m = Line::new(qi(1), qi(0));
        let x = l.crossing(&m).unwrap();
        assert_eq!(x, qi(1));
        assert_eq!(l.eval(x), m.eval(x));
        assert_eq!(l.crossing(&l), None);
        assert_eq!(l.crossing(&Line::new(qi(5), qi(1))), None);
    }
}
