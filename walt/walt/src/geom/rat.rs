//! Exact rational scalars: i128-backed `Ratio`. PLAN.md's width decision:
//! start here, escalate to BigRational only where denominators force it --
//! nothing in S2 does, and overflow-checks are on in every profile, so an
//! overflow is a loud panic, never a wrong number.

pub type Q = num_rational::Ratio<i128>;

/// `n / d`, normalized. Panics if `d == 0`.
pub fn q(n: i128, d: i128) -> Q {
    Q::new(n, d)
}

/// The integer `n` as a rational.
pub fn qi(n: i128) -> Q {
    Q::from_integer(n)
}
