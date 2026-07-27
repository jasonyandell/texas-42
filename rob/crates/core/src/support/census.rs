//! Exact censuses: ternary signatures and their S₃ quotient, the 81-bit
//! standalone support census, reachable capacity profiles, and the no-void
//! floor families.
//!
//! Implements Math §7.12.1 (CELL-21/22/23), §7.12.5 (CELL-27),
//! §7.13.1 (REACH-04), and the REACH-12 floor-family count. All counts are
//! computed from the displayed formulas, never hard-coded (INV-5's numbers
//! live only in the verification assertions).

use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::support::normal_form::ternary_signature_valid;

/// One seat-labeled six-integer ternary signature
/// `(r_0, n_0, r_1, n_1, r_2, n_2)` (Math §7.12.1; CELL-21).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TernarySignature {
    /// Residual capacities, each in `1..=7`.
    pub r: [usize; 3],
    /// Excluded-seat category sizes.
    pub n_excluded: [usize; 3],
}

impl TernarySignature {
    /// The ambiguous-pool size `n = Σ r_s`.
    pub fn pool_size(&self) -> usize {
        self.r.iter().sum()
    }

    /// The derived unrestricted-category size `n_★` (never stored as an
    /// independent key field).
    pub fn unrestricted(&self) -> usize {
        self.pool_size() - self.n_excluded.iter().sum::<usize>()
    }

    /// The canonical S₃-orbit key: the sorted multiset of `(r_s, n_s)`
    /// pairs (CELL-22).
    pub fn orbit_key(&self) -> [(usize, usize); 3] {
        let mut pairs = [
            (self.r[0], self.n_excluded[0]),
            (self.r[1], self.n_excluded[1]),
            (self.r[2], self.n_excluded[2]),
        ];
        pairs.sort_unstable();
        pairs
    }
}

/// Enumerate every valid seat-labeled ternary signature under native bounds
/// (Math §7.12.1: 136,514 of them — the count is asserted by the receipts,
/// not assumed here).
pub fn all_ternary_signatures() -> Vec<TernarySignature> {
    let mut out = Vec::new();
    for r0 in 1..=7usize {
        for r1 in 1..=7usize {
            for r2 in 1..=7usize {
                let n = r0 + r1 + r2;
                for n0 in 0..=(n - r0 - 1) {
                    for n1 in 0..=(n - r1 - 1) {
                        for n2 in 0..=(n - r2 - 1) {
                            if n0 + n1 + n2 <= n
                                && ternary_signature_valid([r0, r1, r2], [n0, n1, n2])
                            {
                                out.push(TernarySignature {
                                    r: [r0, r1, r2],
                                    n_excluded: [n0, n1, n2],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

/// A feasible category-allocation split `(x_0, x_1, x_2)` (Math §7.12.1):
/// `x_0` = W₀ tiles at seat 1, `x_1` = W₁ tiles at seat 0, `x_2` = W₂ tiles
/// at seat 0; the unrestricted-row allocations `y_s` are then forced.
pub fn feasible_splits(signature: &TernarySignature) -> Vec<[usize; 3]> {
    let [r0, r1, r2] = signature.r;
    let [n0, n1, n2] = signature.n_excluded;
    let mut out = Vec::new();
    for x0 in 0..=n0 {
        for x1 in 0..=n1 {
            for x2 in 0..=n2 {
                let y0 = (r0).checked_sub(x1 + x2);
                let y1 = (r1).checked_sub(x0 + (n2 - x2));
                let y2 = (r2).checked_sub((n0 - x0) + (n1 - x1));
                if y0.is_some() && y1.is_some() && y2.is_some() {
                    out.push([x0, x1, x2]);
                }
            }
        }
    }
    out
}

/// The full 4×3 category-allocation matrix of one split: rows are the
/// unrestricted category then `W_0, W_1, W_2`; columns are seats.
pub fn split_matrix(signature: &TernarySignature, split: [usize; 3]) -> [[usize; 3]; 4] {
    let [r0, r1, r2] = signature.r;
    let [n0, n1, n2] = signature.n_excluded;
    let [x0, x1, x2] = split;
    let y0 = r0 - x1 - x2;
    let y1 = r1 - x0 - (n2 - x2);
    let y2 = r2 - (n0 - x0) - (n1 - x1);
    [
        [y0, y1, y2],
        [0, x0, n0 - x0],
        [x1, 0, n1 - x1],
        [x2, n2 - x2, 0],
    ]
}

/// The six seat permutations of S₃.
pub const SEAT_PERMUTATIONS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

/// Apply a simultaneous seat/category relabeling to a signature (CELL-22):
/// seat `s` is renamed `p[s]`.
pub fn relabel_signature(signature: &TernarySignature, p: [usize; 3]) -> TernarySignature {
    let mut r = [0; 3];
    let mut n_excluded = [0; 3];
    for s in 0..3 {
        r[p[s]] = signature.r[s];
        n_excluded[p[s]] = signature.n_excluded[s];
    }
    TernarySignature { r, n_excluded }
}

/// Apply a simultaneous seat/category relabeling to an allocation matrix:
/// `M'[★][p(t)] = M[★][t]`, `M'[W_{p(s)}][p(t)] = M[W_s][t]`.
pub fn relabel_matrix(matrix: &[[usize; 3]; 4], p: [usize; 3]) -> [[usize; 3]; 4] {
    let mut out = [[0usize; 3]; 4];
    for t in 0..3 {
        out[0][p[t]] = matrix[0][t];
        for s in 0..3 {
            out[1 + p[s]][p[t]] = matrix[1 + s][t];
        }
    }
    out
}

/// Exact factorial table `0..=28` (BigUint).
pub fn factorials() -> Vec<BigUint> {
    let mut out = vec![BigUint::one()];
    for i in 1..=28u32 {
        let next = out.last().expect("nonempty") * BigUint::from(i);
        out.push(next);
    }
    out
}

/// `F(R; b)` of Math §7.12.5: assignments of `R` labeled dominoes to three
/// bounded certain-holder categories and one outside-pool category.
pub fn census_f(r: usize, bounds: [usize; 3], fact: &[BigUint]) -> BigUint {
    let mut total = BigUint::zero();
    for c0 in 0..=bounds[0].min(r) {
        for c1 in 0..=bounds[1].min(r - c0) {
            for c2 in 0..=bounds[2].min(r - c0 - c1) {
                let rest = r - c0 - c1 - c2;
                total += &fact[r] / (&fact[c0] * &fact[c1] * &fact[c2] * &fact[rest]);
            }
        }
    }
    total
}

/// The four-branch native standalone support census (Math §7.12.5;
/// CELL-27), computed from the displayed formulas.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SupportCensus {
    /// The single extensional empty state.
    pub empty: BigUint,
    /// Determinate-branch states.
    pub determinate: BigUint,
    /// Binary-branch states.
    pub binary: BigUint,
    /// Ternary-branch states.
    pub ternary: BigUint,
}

impl SupportCensus {
    /// The census total.
    pub fn total(&self) -> BigUint {
        &self.empty + &self.determinate + &self.binary + &self.ternary
    }

    /// The exact fixed-width bit requirement `⌈log₂ total⌉`.
    pub fn fixed_width_bits(&self) -> u64 {
        self.total().bits()
    }
}

/// Compute the census (Math §7.12.5). `binomial(28, n)` and the multinomial
/// prefactors come from the factorial table; the ternary branch sums over
/// every valid six-integer signature.
pub fn support_census() -> SupportCensus {
    let fact = factorials();
    let empty = BigUint::one();
    let determinate = census_f(28, [7, 7, 7], &fact);

    let mut binary = BigUint::zero();
    for inactive in 0..3usize {
        let active: Vec<usize> = (0..3).filter(|&s| s != inactive).collect();
        for ra in 1..=7usize {
            for rb in 1..=7usize {
                let mut r = [0usize; 3];
                r[active[0]] = ra;
                r[active[1]] = rb;
                let n = ra + rb;
                let choose = &fact[28] / (&fact[n] * &fact[28 - n]);
                binary += choose * census_f(28 - n, [7 - r[0], 7 - r[1], 7 - r[2]], &fact);
            }
        }
    }

    let mut ternary = BigUint::zero();
    for signature in all_ternary_signatures() {
        let [r0, r1, r2] = signature.r;
        let [n0, n1, n2] = signature.n_excluded;
        let n = signature.pool_size();
        let n_star = signature.unrestricted();
        let prefactor =
            &fact[28] / (&fact[28 - n] * &fact[n0] * &fact[n1] * &fact[n2] * &fact[n_star]);
        ternary += prefactor * census_f(28 - n, [7 - r0, 7 - r1, 7 - r2], &fact);
    }

    SupportCensus {
        empty,
        determinate,
        binary,
        ternary,
    }
}

/// Reachable hidden-capacity profiles (REACH-04; Math §7.13.1): exactly the
/// labeled triples with `max − min ≤ 1`, enumerated from the full 8³ cube.
pub fn reachable_capacity_profiles() -> Vec<[usize; 3]> {
    let mut out = Vec::new();
    for k0 in 0..=7usize {
        for k1 in 0..=7usize {
            for k2 in 0..=7usize {
                let max = k0.max(k1).max(k2);
                let min = k0.min(k1).min(k2);
                if max - min <= 1 {
                    out.push([k0, k1, k2]);
                }
            }
        }
    }
    out
}

/// The universally reachable no-void floor-family count (REACH-12):
/// `C(28,21) + 3·C(28,20) + 3·C(28,19) + C(28,18)`.
pub fn floor_family_count() -> BigUint {
    let fact = factorials();
    let binomial = |n: usize, k: usize| &fact[n] / (&fact[k] * &fact[n - k]);
    binomial(28, 21)
        + BigUint::from(3u32) * binomial(28, 20)
        + BigUint::from(3u32) * binomial(28, 19)
        + binomial(28, 18)
}
