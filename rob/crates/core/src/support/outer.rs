//! The necessary outer language: schedule language, lead-witness
//! coefficients, per-profile certificate counts, and the five-check
//! necessary-profile validator.
//!
//! Implements rec Math §7.13.3 (REACH-06/06A), §7.13.4 lead-witness
//! necessity (REACH-07), §7.13.6 (REACH-11), and the shallow-phase
//! follower-supply check established by the exchange-adjudicated x:002
//! (tier-labeled; necessary-only). D3 naming is binding: this is a
//! **necessary outer profile** — it cannot construct reachability, and no
//! conversion from any outer type to a certified or reachable type exists
//! (INV-14):
//!
//! ```compile_fail
//! use rob_core::{ReachabilityOuterNecessaryProfile, ReachableContractedPlayState};
//! fn forbidden(p: ReachabilityOuterNecessaryProfile) -> ReachableContractedPlayState {
//!     p.into() // no such conversion exists (INV-14; D3)
//! }
//! ```

use num_bigint::BigUint;
use num_traits::Zero;

use crate::algebra::algebra_for;
use crate::algebra::suits::{LedSuit, LedSuitSet};
use crate::declaration::Declaration;
use crate::domino::{all_ids, DominoSet};
use crate::support::cells::{AbstractCells, HIDDEN_SEATS};

/// `F(B)`: the largest set of hidden seats that can already have acted as
/// followers in the current partial trick, for low-capacity set `B`
/// (Math §7.13.3 table; seats are hidden indices 0..2 = clockwise offsets
/// h1..h3).
pub fn follower_set(low: &[usize]) -> Vec<usize> {
    match low {
        [] => vec![],
        [0] => vec![0],
        [1] => vec![],
        [2] => vec![],
        [0, 1] => vec![0, 1],
        [0, 2] => vec![0],
        [1, 2] => vec![2],
        [0, 1, 2] => vec![1, 2],
        _ => unreachable!("low sets are sorted subsets of {{0,1,2}}"),
    }
}

/// The schedule-language predicate (Math §7.13.3; REACH-06): a triple of
/// void masks is realizable in the projected turn schedule iff `|Q| ≤ j`,
/// or `|Q| = j + 1` and some used context's membership is a nonempty subset
/// of `F(B)`.
pub fn schedule_admissible(
    capacities: [usize; HIDDEN_SEATS],
    void_masks: &[LedSuitSet; 3],
) -> bool {
    let h = *capacities.iter().max().expect("three seats");
    let j = 7 - h;
    let low: Vec<usize> = if capacities.iter().all(|&k| k == h) {
        Vec::new()
    } else {
        (0..HIDDEN_SEATS)
            .filter(|&i| capacities[i] == h - 1)
            .collect()
    };
    let f = follower_set(&low);
    let mut used: Vec<LedSuit> = Vec::new();
    for q in LedSuit::all() {
        if void_masks.iter().any(|m| m.contains(q)) {
            used.push(q);
        }
    }
    if used.len() <= j {
        return true;
    }
    if used.len() == j + 1 {
        return used.iter().any(|&q| {
            let members: Vec<usize> = (0..3).filter(|&i| void_masks[i].contains(q)).collect();
            !members.is_empty() && members.iter().all(|m| f.contains(m))
        });
    }
    false
}

/// `A_j = Σ_{u≤j} C(7,u) 7^u` (Math §7.13.3): the projected schedule
/// census at a completed-trick boundary.
pub fn schedule_census_a() -> [u64; 8] {
    core::array::from_fn(|j| {
        (0..=j as u64)
            .map(|u| binomial_u64(7, u) * 7u64.pow(u as u32))
            .sum()
    })
}

/// `T_{j,f} = A_j + C(7,j+1)(7^{j+1} − (8−2^f)^{j+1})` (Math §7.13.3),
/// with the out-of-range binomial zero at `j = 7`.
pub fn schedule_census_t(f: u32) -> [u64; 8] {
    let a = schedule_census_a();
    core::array::from_fn(|j| {
        let jp = (j + 1) as u32;
        let extra = if j < 7 {
            binomial_u64(7, j as u64 + 1) * (7u64.pow(jp) - (8 - 2u64.pow(f)).pow(jp))
        } else {
            0
        };
        a[j] + extra
    })
}

fn binomial_u64(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

/// `B_{n,u}` by the inclusion–exclusion route (Math §7.13.6; REACH-07/11):
/// out-of-range binomials are zero. Declaration-independent because the
/// lead-fiber cardinalities are always the multiset `{1..7}`.
pub fn lead_witness_b_inclusion_exclusion() -> Vec<Vec<BigUint>> {
    let sizes: [u64; 7] = [1, 2, 3, 4, 5, 6, 7];
    let mut table = vec![vec![BigUint::zero(); 8]; 22];
    for q_mask in 0u32..128 {
        let q: Vec<usize> = (0..7).filter(|&i| q_mask & (1 << i) != 0).collect();
        let u = q.len();
        for n in 0..=21u64 {
            let mut total: i128 = 0;
            for s_mask in 0u32..(1 << u) {
                let mut sign = 1i128;
                let mut ell = 0u64;
                for (bit, &ctx) in q.iter().enumerate() {
                    if s_mask & (1 << bit) != 0 {
                        sign = -sign;
                        ell += sizes[ctx];
                    }
                }
                if ell <= n && 28 - ell >= n - ell {
                    total += sign * binomial_u64(28 - ell, n - ell) as i128;
                }
            }
            assert!(total >= 0, "B coefficients are nonnegative");
            table[n as usize][u] += BigUint::from(total as u128);
        }
    }
    table
}

/// `B_{n,u}` by the polynomial-convolution route (the independent second
/// route of `r_out_lead_witness`).
pub fn lead_witness_b_convolution() -> Vec<Vec<BigUint>> {
    let sizes: [usize; 7] = [1, 2, 3, 4, 5, 6, 7];
    let mut table = vec![vec![BigUint::zero(); 8]; 22];
    for q_mask in 0u32..128 {
        let q: Vec<usize> = (0..7).filter(|&i| q_mask & (1 << i) != 0).collect();
        let u = q.len();
        // Start from (1+x)^(sum of unused sizes).
        let unused: usize = (0..7)
            .filter(|i| q_mask & (1 << i) == 0)
            .map(|i| sizes[i])
            .sum();
        let mut poly = binomial_poly(unused);
        for &ctx in &q {
            // Multiply by (1+x)^{ℓ} − x^{ℓ}.
            let ell = sizes[ctx];
            let mut factor = binomial_poly(ell);
            factor[ell] -= 1i128;
            poly = convolve(&poly, &factor);
        }
        for n in 0..=21usize {
            if n < poly.len() {
                assert!(poly[n] >= 0);
                table[n][u] += BigUint::from(poly[n] as u128);
            }
        }
    }
    table
}

fn binomial_poly(m: usize) -> Vec<i128> {
    let mut poly = vec![0i128; m + 1];
    for (k, coefficient) in poly.iter_mut().enumerate() {
        *coefficient = binomial_u64(m as u64, k as u64) as i128;
    }
    poly
}

fn convolve(a: &[i128], b: &[i128]) -> Vec<i128> {
    let mut out = vec![0i128; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] += x * y;
        }
    }
    out
}

/// The per-profile declaration-tagged outer-certificate count `C(k)`
/// (Math §7.13.6; REACH-11), from the `B` table: the terminal profile has
/// one canonical empty-void certificate.
pub fn profile_certificate_count(capacities: [usize; 3], b: &[Vec<BigUint>]) -> BigUint {
    let n: usize = capacities.iter().sum();
    if n == 0 {
        return BigUint::from(1u32);
    }
    let h = *capacities.iter().max().expect("three seats");
    let j = 7 - h;
    let low: Vec<usize> = if capacities.iter().all(|&k| k == h) {
        Vec::new()
    } else {
        (0..3).filter(|&i| capacities[i] == h - 1).collect()
    };
    let f = follower_set(&low).len() as u32;
    let mut total = BigUint::zero();
    #[allow(clippy::needless_range_loop)] // u is both exponent and B-table index
    for u in 0..=j {
        total += BigUint::from(7u64.pow(u as u32)) * &b[n][u];
    }
    if f > 0 && j < 7 {
        let factor = 7u64.pow(j as u32 + 1) - (8 - 2u64.pow(f)).pow(j as u32 + 1);
        total += BigUint::from(factor) * &b[n][j + 1];
    }
    total
}

/// The necessary outer profile (D3 naming — never a "certificate"): a
/// candidate external claim about declaration, hidden capacities, hidden
/// pool, and public void masks. Membership in the outer language is
/// necessary for reachability, never sufficient (INV-14; the x:002 witness
/// is the permanent regression).
#[derive(Clone, Debug)]
pub struct ReachabilityOuterNecessaryProfile {
    /// The claimed declaration.
    pub declaration: Declaration,
    /// Claimed hidden capacities (clockwise from the viewer).
    pub capacities: [usize; 3],
    /// Claimed public void masks per hidden seat.
    pub void_masks: [LedSuitSet; 3],
    /// The claimed hidden pool.
    pub pool: DominoSet,
}

/// The five-check report (necessary-only; x:002 for the fifth check).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OuterCheckReport {
    /// Reachable capacity shape: `max − min ≤ 1` (REACH-04).
    pub capacity_shape: bool,
    /// Schedule-language admissibility (REACH-06).
    pub schedule: bool,
    /// Lead witness: every used context has a lead tile outside the pool
    /// (REACH-07).
    pub lead_witness: bool,
    /// Hall feasibility of the decoded cells (CELL-09).
    pub hall: bool,
    /// Shallow-phase follower supply (x:002, exchange tier): in the
    /// `(6,6,6)`, one-used-context, single-void-seat phase, the effective
    /// follow set needs at least two members outside the pool.
    pub follower_supply: bool,
}

impl OuterCheckReport {
    /// Conjunction of all five checks — outer-language membership only.
    pub fn all(&self) -> bool {
        self.capacity_shape
            && self.schedule
            && self.lead_witness
            && self.hall
            && self.follower_supply
    }
}

impl ReachabilityOuterNecessaryProfile {
    /// Decode the raw cells this profile denotes (Exec §18 decoder): pool,
    /// capacities, and void-derived allowed sets. Never promoted to any
    /// certified type.
    pub fn decode_cells(&self) -> (AbstractCells, Vec<crate::domino::DominoId>) {
        let algebra = algebra_for(self.declaration);
        let tiles: Vec<crate::domino::DominoId> = self.pool.iter().collect();
        let possible: [Vec<bool>; 3] = core::array::from_fn(|s| {
            tiles
                .iter()
                .map(|&d| !self.void_masks[s].iter().any(|q| algebra.follows(d, q)))
                .collect()
        });
        (
            AbstractCells::new(tiles.len(), possible, self.capacities)
                .expect("structural conservation is the caller's obligation"),
            tiles,
        )
    }

    /// Run the five necessary checks (INV-14: returns profile membership
    /// only; there is no path from this report to a certified type).
    pub fn check_necessary(&self) -> OuterCheckReport {
        let algebra = algebra_for(self.declaration);
        let capacity_shape = {
            let max = *self.capacities.iter().max().expect("three");
            let min = *self.capacities.iter().min().expect("three");
            max - min <= 1
        };
        let schedule = schedule_admissible(self.capacities, &self.void_masks);
        let used: Vec<LedSuit> = LedSuit::all()
            .into_iter()
            .filter(|&q| self.void_masks.iter().any(|m| m.contains(q)))
            .collect();
        let lead_witness = used
            .iter()
            .all(|&q| algebra.lead_fiber(q).iter().any(|d| !self.pool.contains(d)));
        let hall = self.decode_cells().0.is_feasible();
        let follower_supply = {
            let single_membership = used.len() == 1 && {
                let q = used[0];
                (0..3).filter(|&i| self.void_masks[i].contains(q)).count() == 1
            };
            if self.capacities == [6, 6, 6] && single_membership {
                let q = used[0];
                let outside = all_ids()
                    .filter(|&d| algebra.follows(d, q) && !self.pool.contains(d))
                    .count();
                outside >= 2
            } else {
                // Outside the shallow phase x:002 establishes, the check is
                // vacuous (necessary-only discipline: no unproved pruning).
                true
            }
        };
        OuterCheckReport {
            capacity_shape,
            schedule,
            lead_witness,
            hall,
            follower_supply,
        }
    }
}
