//! `solver::calibrate` — §22 step 8: the V5 flip-repair gate and the
//! per-fixed-pair E0 calibration arithmetic (§19 V5/V6).
//!
//! EXPLORATORY tier. Implements the verbatim parent
//! `walt/math/calculated_evidence_v0.1.md` §19 V5 (the historical 40/160
//! flip), §19 V6 (fixed-pair cost calibration — the per-fixed-pair E0
//! calibration of §22 step 8, per-pair and never pooled), §4.2 (empirical
//! coordinates), §7 (the information rate and the leading-order forecast),
//! and §8.4 (the exact forecast dynamic program under a declared
//! predictive law); adjudicated CE-A1..A8 (`walt/CENSUS-RULINGS.md`).
//! On the E0 name: the parent's only E0 use is §22 step 8's "corrected
//! per-fixed-pair E0 calibration"; "E0" is the standing ID of the tilt
//! audit (SP-A4), and the correction the parent makes to it is §19 V6's —
//! per fixed pair, exact coordinates where an exact fiber exists, forecasts
//! compared against observed anytime-valid settlement, never a pooled
//! pseudo-pair and never a fixed-n rule.
//!
//! Everything here is exact rational or exact integer arithmetic. The two
//! transcendental quantities the parent's forecasts mention — `ln T` and
//! the Bernoulli divergence `D_{1/2}(τ)` — are handled as exact rational
//! INTERVAL BOUNDS from their series with explicit rational tail bounds,
//! never as floating-point approximations (the §8.5 refinement vector's
//! omitted-fields note is hereby discharged by bounds, not by floats).
//! Forecasts are forecasts: nothing in this module is a settlement rule.

use std::collections::HashMap;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use crate::kernel::Kernel;
use crate::rules::receipt::ReceiptHand;
use crate::rules::rules::{legal_plays, Trick};
use crate::rules::{Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat, Team};
use crate::solver::adaptive::{
    driven_root, replay_viewer_success, CanonicalRoot, DrivenState, PublicRecord, RootPosition,
    SlicePolicy,
};
use crate::solver::controller::SetResult;
use crate::solver::evidence;
use crate::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, InnerSchedule, Level0Field, TieRule,
};
use crate::solver::{mix, SplitMix64};

// ---------------------------------------------------------------------------
// Exact per-pair coordinates over a complete fiber (§19 V6; §4.2 made exact).
// ---------------------------------------------------------------------------

/// Enumerate the complete fiber once and replay every candidate on every
/// world: `outcomes[k][w]` is candidate `k`'s terminal pmake indicator on
/// enumeration world `w`. One pass serves every pair of the set — the
/// per-PAIR coordinates derived from it are still per-pair objects (V6);
/// only the enumeration work is shared.
pub fn exact_set_outcomes(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[&dyn SlicePolicy],
    field: &dyn SlicePolicy,
) -> Vec<Vec<bool>> {
    assert!(!candidates.is_empty(), "an outcome table names a candidate");
    let fiber = usize::try_from(root.count()).expect("an enumerable fiber fits in usize");
    let viewer = root.kernel().viewer();
    let mut outcomes: Vec<Vec<bool>> = vec![Vec::with_capacity(fiber); candidates.len()];
    let mut visited = 0usize;
    for world in root.worlds() {
        for (k, candidate) in candidates.iter().enumerate() {
            outcomes[k].push(replay_viewer_success(
                position, viewer, &world, *candidate, field,
            ));
        }
        visited += 1;
    }
    assert_eq!(
        visited, fiber,
        "enumeration visits the whole fiber exactly once"
    );
    outcomes
}

/// The EXACT coordinates of one fixed pair over a complete fiber: pivotal
/// counts, pivotal mass `q`, gap `g`, tilt `τ`, and fixed-pair hardness
/// `H = 1/(qτ²) − 1` where defined. These are the true parameters of the
/// §11.2 sampling law (uniform with replacement over the fiber), so the
/// evidence process's settlement behavior for this pair is exactly the
/// Bernoulli walk these coordinates parameterize.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairCoordinates {
    /// Fiber size.
    pub n: u128,
    /// Worlds where `i` succeeds and `j` fails.
    pub a: u128,
    /// Worlds where `j` succeeds and `i` fails.
    pub b: u128,
    /// Nonpivotal worlds `n - a - b`.
    pub n0: u128,
    /// `q = (a+b)/n`, exact.
    pub q: BigRational,
    /// `g = (a-b)/n`, exact.
    pub g: BigRational,
    /// `τ = (a-b)/(a+b)`; `None` exactly when `q = 0` (parent §2.4 Case A:
    /// `q = 0` forces `g = 0`, equal value is a theorem there).
    pub tau: Option<BigRational>,
    /// `H = 1/(qτ²) − 1`; `None` when `q = 0` or `τ = 0` (regime 4:
    /// strict directional evidence has zero asymptotic growth).
    pub hardness: Option<BigRational>,
}

/// Fold two candidates' exact outcome vectors into the pair's exact
/// coordinates.
pub fn pair_coordinates(u_i: &[bool], u_j: &[bool]) -> PairCoordinates {
    assert_eq!(u_i.len(), u_j.len(), "one fiber, one enumeration order");
    assert!(!u_i.is_empty(), "a pair is measured over a nonempty fiber");
    let mut a = 0u128;
    let mut b = 0u128;
    for (x, y) in u_i.iter().zip(u_j) {
        match (x, y) {
            (true, false) => a += 1,
            (false, true) => b += 1,
            _ => {}
        }
    }
    let n = u_i.len() as u128;
    let big = |v: u128| BigInt::from(v);
    let signed = BigInt::from(a) - BigInt::from(b);
    let q = BigRational::new(big(a + b), big(n));
    let g = BigRational::new(signed.clone(), big(n));
    let tau = (a + b > 0).then(|| BigRational::new(signed, big(a + b)));
    let hardness = tau.as_ref().and_then(|t| {
        (!t.is_zero() && !q.is_zero()).then(|| (&q * t * t).recip() - BigRational::one())
    });
    PairCoordinates {
        n,
        a,
        b,
        n0: n - a - b,
        q,
        g,
        tau,
        hardness,
    }
}

// ---------------------------------------------------------------------------
// Exact rational interval bounds for the two transcendental forecast
// ingredients (§7). Series with explicit rational tail bounds — no floats.
// ---------------------------------------------------------------------------

/// Exact rational bounds `(lo, hi)` with `lo ≤ D_{1/2}(τ) ≤ hi`, from the
/// §7.1 series `D_{1/2}(τ) = Σ_{k≥1} τ^{2k}/((2k)(2k−1))` (every term
/// nonnegative, so a partial sum is a lower bound). Tail bound: with
/// `t = τ² < 1` the terms decay at least geometrically in `t`, so
/// `tail ≤ t^{K+1}/((2K+2)(2K+1)(1−t))`; at `|τ| = 1` the tail telescopes
/// to below `1/(2K+1)`.
pub fn d_half_bounds(tau: &BigRational, terms: u32) -> (BigRational, BigRational) {
    let t = tau * tau;
    assert!(t <= BigRational::one(), "a tilt lies in [-1, 1]");
    assert!(terms >= 1, "a series bound takes at least one term");
    if t.is_zero() {
        return (BigRational::zero(), BigRational::zero());
    }
    let mut power = BigRational::one();
    let mut sum = BigRational::zero();
    for k in 1..=u64::from(terms) {
        power *= &t;
        sum += &power / BigRational::from_integer(BigInt::from(2 * k * (2 * k - 1)));
    }
    let k = u64::from(terms);
    let tail = if t == BigRational::one() {
        BigRational::new(BigInt::one(), BigInt::from(2 * k + 1))
    } else {
        &power * &t
            / BigRational::from_integer(BigInt::from((2 * k + 2) * (2 * k + 1)))
            / (BigRational::one() - &t)
    };
    let hi = &sum + tail;
    (sum, hi)
}

/// Exact rational bounds `(lo, hi)` with `lo ≤ ln x ≤ hi` for rational
/// `x ≥ 1`, from the atanh series `ln x = 2 Σ_{k≥0} z^{2k+1}/(2k+1)` with
/// `z = (x−1)/(x+1) ∈ [0,1)` (every term nonnegative). Tail bound:
/// `Σ_{k>K} z^{2k+1}/(2k+1) ≤ z^{2K+3}/((2K+3)(1−z²))`.
pub fn ln_bounds(x: &BigRational, terms: u32) -> (BigRational, BigRational) {
    assert!(*x >= BigRational::one(), "these bounds serve ln on [1, ∞)");
    assert!(terms >= 1, "a series bound takes at least one term");
    let z = (x - BigRational::one()) / (x + BigRational::one());
    if z.is_zero() {
        return (BigRational::zero(), BigRational::zero());
    }
    let z2 = &z * &z;
    let mut power = z.clone(); // z^{2k+1}
    let mut sum = z.clone();
    for k in 1..=u64::from(terms - 1) {
        power *= &z2;
        sum += &power / BigRational::from_integer(BigInt::from(2 * k + 1));
    }
    let k = u64::from(terms - 1);
    let tail = &power * &z2
        / BigRational::from_integer(BigInt::from(2 * k + 3))
        / (BigRational::one() - &z2);
    let two = BigRational::from_integer(BigInt::from(2));
    (&two * &sum, two * (sum + tail))
}

/// §7 — exact rational bounds on the raw-world pivotal information rate
/// `𝓘 = q·D_{1/2}(τ)`. `None` exactly in regime 4 (`q = 0` or `τ = 0`),
/// where strict directional evidence has zero asymptotic growth.
pub fn information_rate_bounds(
    q: &BigRational,
    tau: &BigRational,
    terms: u32,
) -> Option<(BigRational, BigRational)> {
    if q.is_zero() || tau.is_zero() {
        return None;
    }
    let (lo, hi) = d_half_bounds(tau, terms);
    Some((q * lo, q * hi))
}

/// §7 — the leading-order raw-world forecast to reach threshold `T`,
/// as an exact rational interval `[ln T / (q·D_hi), ln T / (q·D_lo)]`
/// containing `ln T / (q·D_{1/2}(τ))`. A forecast, never a stopping rule.
/// `None` in regime 4.
pub fn leading_order_forecast_bounds(
    threshold: &BigRational,
    q: &BigRational,
    tau: &BigRational,
    terms: u32,
) -> Option<(BigRational, BigRational)> {
    assert!(
        threshold > &BigRational::one(),
        "an evidence threshold exceeds one"
    );
    let (rate_lo, rate_hi) = information_rate_bounds(q, tau, terms)?;
    let (ln_lo, ln_hi) = ln_bounds(threshold, terms);
    Some((ln_lo / rate_hi, ln_hi / rate_lo))
}

// ---------------------------------------------------------------------------
// §8.4 — the exact forecast dynamic program under a declared predictive law.
// ---------------------------------------------------------------------------

/// The declared rational predictive law `(p̃+, p̃−, p̃0)` of §8.4, with
/// `p̃0 = 1 − p̃+ − p̃−` derived. On an exact fiber the exact law is
/// `p̃+ = a/n`, `p̃− = b/n` (which is `q(1±τ)/2` algebraically); from
/// observed counts it is the labeled estimate the parent describes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredictiveLaw {
    pub p_plus: BigRational,
    pub p_minus: BigRational,
}

impl PredictiveLaw {
    pub fn new(p_plus: BigRational, p_minus: BigRational) -> PredictiveLaw {
        assert!(!p_plus.is_negative(), "a probability is nonnegative");
        assert!(!p_minus.is_negative(), "a probability is nonnegative");
        assert!(
            &p_plus + &p_minus <= BigRational::one(),
            "a predictive law's parts sum to at most one"
        );
        PredictiveLaw { p_plus, p_minus }
    }

    /// The law from pivotal counts over `n` worlds — exact on a complete
    /// fiber, a labeled estimate on a sampled prefix.
    pub fn from_counts(a: u128, b: u128, n: u128) -> PredictiveLaw {
        assert!(n > 0, "a predictive law needs observations");
        assert!(a + b <= n, "pivotal counts fit the world count");
        PredictiveLaw::new(
            BigRational::new(BigInt::from(a), BigInt::from(n)),
            BigRational::new(BigInt::from(b), BigInt::from(n)),
        )
    }

    pub fn p_zero(&self) -> BigRational {
        BigRational::one() - &self.p_plus - &self.p_minus
    }
}

/// The settlement boundary in pivotal-count space, memoized: `a_min(b)` is
/// the least `a` with `E+_{a,b} ≥ T`, nondecreasing in `b` (appending an
/// adverse pivot multiplies the §4 integrand by `(1−t) ≤ 1`). A state
/// `(a,b)` is settled when either direction has crossed:
/// `a ≥ a_min(b)` or `b ≥ a_min(a)`.
struct SettleBoundary {
    threshold: BigRational,
    a_min: Vec<u64>,
}

impl SettleBoundary {
    fn new(threshold: BigRational) -> SettleBoundary {
        assert!(
            threshold > BigRational::one(),
            "an evidence threshold exceeds one"
        );
        SettleBoundary {
            threshold,
            a_min: Vec::new(),
        }
    }

    fn a_min(&mut self, b: u64) -> u64 {
        while self.a_min.len() <= b as usize {
            let next_b = self.a_min.len() as u64;
            let mut a = self.a_min.last().copied().unwrap_or(0);
            while !evidence::crossed(&evidence::pivotal_evidence(a, next_b), &self.threshold) {
                a += 1;
            }
            self.a_min.push(a);
        }
        self.a_min[b as usize]
    }

    fn settled(&mut self, a: u64, b: u64) -> bool {
        a >= self.a_min(b) || b >= self.a_min(a)
    }
}

/// The §8.4 forecast's answer: the smallest horizon `h` with
/// `F_h(start) ≥ γ` if one exists within `h_max`, and the exact
/// `F(start)` at the horizon where the walk stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DpForecast {
    pub crossing: Option<u64>,
    pub f_at_end: BigRational,
    pub h_max: u64,
}

/// §8.4 — the exact forecast dynamic program: `F_h(a,b)` is the
/// probability, under the declared predictive law, of reaching either
/// evidence threshold within at most `h` additional raw worlds from
/// pivotal counts `(a,b)`. The parent's boxed recursion
/// `F_h = p̃+·F_{h−1}(a+1,b) + p̃−·F_{h−1}(a,b+1) + p̃0·F_{h−1}(a,b)` is
/// computed here by its adjoint — pushing the state distribution forward
/// one raw world at a time and absorbing mass that enters a settled state
/// (the same linear recursion read from the other end; the values agree
/// term by term). All arithmetic is exact: integer numerators over the
/// common denominator `den^h`, with exact mass conservation asserted at
/// every layer.
///
/// The answer is a forecast under the DECLARED law (parent: "exact
/// conditional on the declared predictive law"), never a settlement rule;
/// `h_max` is this computation's own resource cap, and exceeding it means
/// `crossing = None`, honestly.
pub fn dp_settlement_forecast(
    law: &PredictiveLaw,
    threshold: &BigRational,
    start: (u64, u64),
    gamma: &BigRational,
    h_max: u64,
) -> DpForecast {
    assert!(
        gamma > &BigRational::zero() && gamma <= &BigRational::one(),
        "a forecast confidence lies in (0, 1]"
    );
    let mut boundary = SettleBoundary::new(threshold.clone());
    if boundary.settled(start.0, start.1) {
        return DpForecast {
            crossing: Some(0),
            f_at_end: BigRational::one(),
            h_max,
        };
    }
    // Common integer denominator: den = denom(p+)·denom(p−). p̃0's
    // numerator over den is integral by construction.
    let den = law.p_plus.denom() * law.p_minus.denom();
    let p = law.p_plus.numer() * law.p_minus.denom();
    let m = law.p_minus.numer() * law.p_plus.denom();
    let z = &den - &p - &m;
    assert!(
        !z.is_negative(),
        "a predictive law's parts sum to at most one"
    );
    let mut mass: HashMap<(u64, u64), BigInt> = HashMap::new();
    mass.insert(start, BigInt::one());
    let mut absorbed = BigInt::zero();
    let mut den_pow = BigInt::one();
    for h in 1..=h_max {
        den_pow *= &den;
        let mut next: HashMap<(u64, u64), BigInt> = HashMap::with_capacity(mass.len() + 2);
        let mut newly = BigInt::zero();
        for ((a, b), x) in &mass {
            if !z.is_zero() {
                *next.entry((*a, *b)).or_insert_with(BigInt::zero) += &z * x;
            }
            if !p.is_zero() {
                let v = &p * x;
                if boundary.settled(a + 1, *b) {
                    newly += v;
                } else {
                    *next.entry((a + 1, *b)).or_insert_with(BigInt::zero) += v;
                }
            }
            if !m.is_zero() {
                let v = &m * x;
                if boundary.settled(*a, b + 1) {
                    newly += v;
                } else {
                    *next.entry((*a, b + 1)).or_insert_with(BigInt::zero) += v;
                }
            }
        }
        absorbed = absorbed * &den + newly;
        let live: BigInt = next.values().sum();
        assert_eq!(
            &absorbed + live,
            den_pow,
            "probability mass is conserved exactly at every layer"
        );
        mass = next;
        if &absorbed * gamma.denom() >= gamma.numer() * &den_pow {
            return DpForecast {
                crossing: Some(h),
                f_at_end: BigRational::new(absorbed, den_pow),
                h_max,
            };
        }
    }
    DpForecast {
        crossing: None,
        f_at_end: BigRational::new(absorbed, den_pow),
        h_max,
    }
}

// ---------------------------------------------------------------------------
// The V5 cap ladder (§19 V5): settled means settled, never a
// cap-dependent flip.
// ---------------------------------------------------------------------------

/// The lawful summary of one root's cap ladder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapLadderVerdict {
    /// Some cap escalated to the exact endpoint; every cap that produced
    /// an exact answer produced the SAME exact answer.
    ExactStable { winner: Option<usize> },
    /// Some cap δ-settled; every settled cap agrees on winner and
    /// settlement index, and no larger cap un-settles it.
    SettledStable {
        winner: usize,
        settled_at: u64,
        first_settled_cap: u64,
    },
    /// Every settled cap returned ε-equivalence with identical survivors.
    EquivalentStable {
        survivors: Vec<usize>,
        settled_at: u64,
    },
    /// Every cap returned honest `Unresolved` — the near-tie stays a
    /// near-tie, visibly, at every budget.
    HonestOpen { survivors_at_largest: Vec<usize> },
}

/// §19 V5 — assert the flip-repair law over one root's cap ladder: the
/// SAME epoch and stream evaluated at increasing resource caps must
/// produce a monotone story (unresolved may settle later; settled stays
/// settled identically; exact stays exact identically). It must never
/// call two caps "settled" with different answers — the 40/160 failure
/// mode. Panics on any violation; returns the lawful summary.
pub fn assert_cap_ladder(results: &[(u64, &SetResult)]) -> CapLadderVerdict {
    assert!(results.len() >= 2, "a ladder compares at least two caps");
    for pair in results.windows(2) {
        assert!(pair[0].0 < pair[1].0, "caps ascend strictly");
    }
    #[derive(PartialEq, Eq, Clone)]
    enum Settled {
        Delta {
            winner: usize,
            settled_at: u64,
        },
        Exact {
            winner: Option<usize>,
            wins: Vec<u128>,
        },
        Equivalent {
            survivors: Vec<usize>,
            settled_at: u64,
        },
    }
    let mut first: Option<(u64, Settled)> = None;
    let mut last_open: Option<Vec<usize>> = None;
    for (cap, result) in results {
        let this = match result {
            SetResult::DeltaSettled {
                winner, settled_at, ..
            } => Some(Settled::Delta {
                winner: *winner,
                settled_at: *settled_at,
            }),
            SetResult::ExactFrozenSet { winner, wins, .. } => Some(Settled::Exact {
                winner: *winner,
                wins: wins.clone(),
            }),
            SetResult::EpsilonEquivalent {
                survivors,
                settled_at,
                ..
            } => Some(Settled::Equivalent {
                survivors: survivors.clone(),
                settled_at: *settled_at,
            }),
            SetResult::Unresolved { survivors, .. } => {
                last_open = Some(survivors.clone());
                None
            }
        };
        match (&first, this) {
            (None, Some(settled)) => first = Some((*cap, settled)),
            (Some((_, prior)), Some(settled)) => assert!(
                *prior == settled,
                "V5: two caps of one stream settled differently — the 40/160 flip"
            ),
            (Some(_), None) => {
                panic!("V5: a settled stream un-settled under extension (cap {cap})")
            }
            (None, None) => {}
        }
    }
    match first {
        Some((cap, Settled::Delta { winner, settled_at })) => CapLadderVerdict::SettledStable {
            winner,
            settled_at,
            first_settled_cap: cap,
        },
        Some((_, Settled::Exact { winner, .. })) => CapLadderVerdict::ExactStable { winner },
        Some((
            _,
            Settled::Equivalent {
                survivors,
                settled_at,
            },
        )) => CapLadderVerdict::EquivalentStable {
            survivors,
            settled_at,
        },
        None => CapLadderVerdict::HonestOpen {
            survivors_at_largest: last_open.expect("at least one result"),
        },
    }
}

// ---------------------------------------------------------------------------
// The flip fixtures (§19 V5): the step-7 shadow run's four exact-route
// disagreements, as replay fixtures (CE-A5: fixed data may persist as
// replay fixtures and historical coordinates — these are both).
// ---------------------------------------------------------------------------

/// One recorded decision where the live 200/8 player's choice differed
/// from the exact frozen-set answer, from
/// `walt/probes/shadow/{receipt,driven}.jsonl` (2026-08-24 run at the
/// committed defaults). The deal and line prefix reproduce the position;
/// the recorded epoch hash pins the reconstruction to the shadow record
/// byte-for-byte (same root, same candidates, same δ — same stream).
pub struct FlipFixture {
    pub mode: &'static str,
    pub hand: usize,
    /// Focal decision ordinal within the hand (the §6 decision event d).
    pub d: u64,
    pub trick: usize,
    pub ply: usize,
    pub decl_id: usize,
    pub bid: u32,
    pub declaring_team: usize,
    pub bidder: usize,
    /// Seat-indexed seven-tile deals, `(hi, lo)` pips.
    pub deal: [&'static [(u8, u8)]; 4],
    /// `(seat, (hi, lo))` plays before the decision, in play order.
    pub prefix: &'static [(usize, (u8, u8))],
    pub fiber: u128,
    pub m: usize,
    /// Legal tiles at the decision, ascending tile index — the shadow
    /// run's candidate order.
    pub legal: &'static [(u8, u8)],
    pub live_tile: (u8, u8),
    pub exact_winner: (u8, u8),
    /// Exact win counts per legal tile, candidate order.
    pub exact_wins: &'static [u128],
    /// The shadow record's epoch hash (hex) for this decision.
    pub epoch: &'static str,
}

/// The four flip specimens of the 2026-08-24 shadow run.
pub const FLIP_FIXTURES: [FlipFixture; 4] = [
    // receipt hand 4, d=3: trick 5 ply 3, fiber 60 — live 1-0 vs exact 2-1.
    FlipFixture {
        mode: "receipt",
        hand: 4,
        d: 3,
        trick: 5,
        ply: 3,
        decl_id: 1,
        bid: 30,
        declaring_team: 1,
        bidder: 1,
        deal: [
            &[(2, 0), (3, 1), (4, 2), (5, 2), (5, 3), (5, 5), (6, 3)],
            &[(1, 0), (2, 1), (3, 0), (4, 0), (5, 1), (6, 5), (6, 6)],
            &[(0, 0), (1, 1), (3, 2), (4, 1), (4, 4), (5, 0), (6, 4)],
            &[(2, 2), (3, 3), (4, 3), (5, 4), (6, 0), (6, 1), (6, 2)],
        ],
        prefix: &[
            (1, (6, 5)),
            (2, (6, 4)),
            (3, (6, 0)),
            (0, (6, 3)),
            (1, (6, 6)),
            (2, (1, 1)),
            (3, (6, 2)),
            (0, (3, 1)),
            (2, (4, 4)),
            (3, (4, 3)),
            (0, (4, 2)),
            (1, (4, 0)),
            (2, (0, 0)),
            (3, (5, 4)),
            (0, (2, 0)),
            (1, (3, 0)),
            (2, (3, 2)),
            (3, (3, 3)),
            (0, (5, 3)),
        ],
        fiber: 60,
        m: 3,
        legal: &[(1, 0), (2, 1), (5, 1)],
        live_tile: (1, 0),
        exact_winner: (2, 1),
        exact_wins: &[41, 45, 42],
        epoch: "9e836d8fa3ec94a2197cf5f63764a51512b610b4e7040ab204ec51643aa3ee4d",
    },
    // receipt hand 7, d=5: trick 5 ply 1, fiber 28 — live 6-3 vs exact 6-2.
    FlipFixture {
        mode: "receipt",
        hand: 7,
        d: 5,
        trick: 5,
        ply: 1,
        decl_id: 5,
        bid: 30,
        declaring_team: 0,
        bidder: 0,
        deal: [
            &[(1, 0), (4, 4), (5, 1), (5, 3), (5, 5), (6, 2), (6, 3)],
            &[(2, 0), (2, 2), (4, 0), (4, 2), (6, 4), (6, 5), (6, 6)],
            &[(2, 1), (3, 0), (3, 2), (3, 3), (5, 0), (5, 2), (6, 0)],
            &[(0, 0), (1, 1), (3, 1), (4, 1), (4, 3), (5, 4), (6, 1)],
        ],
        prefix: &[
            (0, (5, 5)),
            (1, (6, 5)),
            (2, (5, 0)),
            (3, (5, 4)),
            (0, (5, 3)),
            (1, (2, 0)),
            (2, (5, 2)),
            (3, (0, 0)),
            (0, (4, 4)),
            (1, (4, 0)),
            (2, (3, 0)),
            (3, (4, 3)),
            (0, (1, 0)),
            (1, (2, 2)),
            (2, (2, 1)),
            (3, (3, 1)),
            (3, (1, 1)),
        ],
        fiber: 28,
        m: 3,
        legal: &[(5, 1), (6, 2), (6, 3)],
        live_tile: (6, 3),
        exact_winner: (6, 2),
        exact_wins: &[15, 20, 17],
        epoch: "51ad3fa86500cf7da19be6157b848479635bc2057902852fdf7559ec5d3edaef",
    },
    // receipt hand 11, d=4: trick 4 ply 1, fiber 1750 — live 4-2 vs exact 3-0.
    FlipFixture {
        mode: "receipt",
        hand: 11,
        d: 4,
        trick: 4,
        ply: 1,
        decl_id: 4,
        bid: 30,
        declaring_team: 0,
        bidder: 0,
        deal: [
            &[(1, 0), (1, 1), (2, 2), (3, 0), (4, 2), (5, 4), (6, 4)],
            &[(0, 0), (2, 0), (2, 1), (3, 1), (5, 0), (5, 1), (6, 2)],
            &[(3, 2), (4, 1), (4, 3), (5, 2), (6, 0), (6, 1), (6, 3)],
            &[(3, 3), (4, 0), (4, 4), (5, 3), (5, 5), (6, 5), (6, 6)],
        ],
        prefix: &[
            (0, (1, 1)),
            (1, (2, 1)),
            (2, (6, 1)),
            (3, (3, 3)),
            (0, (5, 4)),
            (1, (0, 0)),
            (2, (4, 1)),
            (3, (4, 0)),
            (0, (2, 2)),
            (1, (2, 0)),
            (2, (3, 2)),
            (3, (4, 4)),
            (3, (5, 3)),
        ],
        fiber: 1750,
        m: 4,
        legal: &[(1, 0), (3, 0), (4, 2), (6, 4)],
        live_tile: (4, 2),
        exact_winner: (3, 0),
        exact_wins: &[1081, 1448, 1415, 1138],
        epoch: "07769c6ac99e3d908f2e94b98efe62b0863dbe0691a38efd6cf9d4d16d400d2a",
    },
    // driven hand 14, d=4: trick 4 ply 3, fiber 50 — live 3-1 vs exact 2-1.
    FlipFixture {
        mode: "driven",
        hand: 14,
        d: 4,
        trick: 4,
        ply: 3,
        decl_id: 5,
        bid: 30,
        declaring_team: 1,
        bidder: 1,
        deal: [
            &[(2, 0), (2, 2), (4, 1), (4, 2), (6, 1), (6, 2), (6, 3)],
            &[(1, 1), (2, 1), (3, 1), (3, 3), (5, 2), (5, 4), (5, 5)],
            &[(0, 0), (4, 0), (5, 1), (5, 3), (6, 0), (6, 4), (6, 6)],
            &[(1, 0), (3, 0), (3, 2), (4, 3), (4, 4), (5, 0), (6, 5)],
        ],
        prefix: &[
            (1, (5, 5)),
            (2, (5, 1)),
            (3, (5, 0)),
            (0, (6, 3)),
            (1, (3, 3)),
            (2, (0, 0)),
            (3, (3, 0)),
            (0, (6, 2)),
            (1, (1, 1)),
            (2, (5, 3)),
            (3, (1, 0)),
            (0, (4, 1)),
            (2, (4, 0)),
            (3, (4, 4)),
            (0, (4, 2)),
        ],
        fiber: 50,
        m: 4,
        legal: &[(2, 1), (3, 1), (5, 2), (5, 4)],
        live_tile: (3, 1),
        exact_winner: (2, 1),
        exact_wins: &[34, 33, 28, 16],
        epoch: "51644a08797d478ef9cbc770878fb2b7f1dd6b8b17116d788b91bff39379bd3f",
    },
];

fn decl_from_id(id: usize) -> Decl {
    match id {
        0..=6 => Decl::PipTrump(Pip::new(id as u8).expect("pip <= 6")),
        7 => Decl::DoublesTrump,
        9 => Decl::NoTrump,
        other => panic!("unknown arena decl id {other}"),
    }
}

fn team_from_index(i: usize) -> Team {
    match i {
        0 => Team::T0,
        1 => Team::T1,
        other => panic!("unknown team index {other}"),
    }
}

fn tile(t: (u8, u8)) -> Domino {
    Domino::new(
        Pip::new(t.0).expect("pip <= 6"),
        Pip::new(t.1).expect("pip <= 6"),
    )
}

/// The reconstructed root of one flip fixture, plus its legal tiles in
/// candidate order.
pub struct FlipRoot {
    pub root: CanonicalRoot,
    pub position: RootPosition,
    pub legal_tiles: Vec<Domino>,
    pub focal: Seat,
}

/// Reconstruct a flip fixture's decision point by replaying its recorded
/// line prefix through the rules machinery (legality, holdings, trick
/// winners, voids, and banked totals all recomputed and asserted — the
/// fixture line is data, the rules are the authority). The result is
/// asserted against every recorded coordinate: seat to move, legal set,
/// and fiber count.
pub fn reconstruct_flip(f: &FlipFixture) -> FlipRoot {
    let decl = decl_from_id(f.decl_id);
    let declaring_team = team_from_index(f.declaring_team);
    let bidder = Seat::from_index(f.bidder).expect("a seat index");
    assert_eq!(bidder.team(), declaring_team, "the bidder's team declares");
    let mut hands: [DominoSet; 4] = core::array::from_fn(|s| {
        let mut set = DominoSet::EMPTY;
        for t in f.deal[s] {
            assert!(set.insert(tile(*t)), "a deal lists a tile once");
        }
        set
    });
    let mut prior_played = DominoSet::EMPTY;
    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut leader = bidder;
    let mut banked = [0u32; 2];
    let mut voids = [ContextSet::EMPTY; 4];
    for (seat_index, t) in f.prefix {
        let seat = leader.plus(trick_plays.len());
        assert_eq!(seat.index(), *seat_index, "the recorded seat is to move");
        let d = tile(*t);
        let led: Option<Context> = trick_plays.first().map(|x| decl.led_context(*x));
        let legal = legal_plays(decl, hands[seat.index()], led);
        assert!(legal.contains(d), "a recorded play is legal");
        if let Some(led) = led {
            if !decl.follows(d, led) {
                voids[seat.index()].insert(led);
            }
        }
        assert!(hands[seat.index()].remove(d), "a recorded play is held");
        trick_plays.push(d);
        if trick_plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
            let trick = Trick::new(leader, doms).expect("four distinct tiles");
            let winner = trick.winner(decl);
            banked[winner.team().index()] += trick.points();
            for x in doms {
                prior_played.insert(x);
            }
            leader = winner;
            trick_plays.clear();
        }
    }
    let focal = leader.plus(trick_plays.len());
    assert_eq!(
        focal, bidder,
        "every shadow flip decision belongs to the focal bidder seat"
    );
    let led: Option<Context> = trick_plays.first().map(|x| decl.led_context(*x));
    let legal = legal_plays(decl, hands[focal.index()], led);
    let legal_tiles: Vec<Domino> = legal.iter().collect();
    let expected: Vec<Domino> = f.legal.iter().map(|t| tile(*t)).collect();
    assert_eq!(legal_tiles, expected, "the recorded legal set reproduces");
    assert_eq!(legal_tiles.len(), f.m, "the recorded m reproduces");
    let state = DrivenState {
        decl,
        bid: f.bid,
        declaring_team,
        viewer_hand: hands[focal.index()],
        leader,
        trick_plays: &trick_plays,
        banked,
        prior_played,
        voids,
    };
    let (root, position) = driven_root(&state).expect("a recorded decision has a lawful kernel");
    assert_eq!(root.count(), f.fiber, "the recorded fiber count reproduces");
    FlipRoot {
        root,
        position,
        legal_tiles,
        focal,
    }
}

/// The shadow run's freeze tuple for one candidate, verbatim (identical
/// `solver_source` string, declared 8/2 inner schedule) — required so the
/// reconstructed candidates' `PolicyId`s, and therefore the §5.3 epoch and
/// its streams, byte-match the shadow record.
pub fn shadow_tuple(position: &RootPosition, pinned: Domino) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-level1-continuation-v1 (solver::level1_evaluate; \
                        saturation-tie refinement 4x per round capped at 16x)"
            .to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![8, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned },
    }
}

/// The shadow run's declared risk scopes for one decision: the run is the
/// hand at δ_run = 1/100, and this is decision event `d` (§6).
pub fn shadow_scopes(f: &FlipFixture) -> (String, String) {
    (
        format!("run:shadow-{}-h{}", f.mode, f.hand),
        format!("decision:shadow-{}-h{}-d{}", f.mode, f.hand, f.d),
    )
}

// ---------------------------------------------------------------------------
// The count-timing reconstruction (§19 V5's named episode, by shape).
// ---------------------------------------------------------------------------

/// Frozen seed for the count-timing family (a distinct stream constant).
pub const COUNT_TIMING_SEED: u64 = 0x40F1_1B24_2026_0823;

/// One reconstructed count-timing root: the SHAPE of the 2026-08-23
/// plunge review's trick-1 near-tie (`wiki/walt-seat-play.md`;
/// `walt/LEVEL2-PROBE.md` specimen 2) — bid 30 on sixes, the bidder's 6-6
/// lead winning trick 1, the partner holding 6-2 and 6-4 (ten count) and
/// no other six, so the legal set is exactly {6-2, 6-4}: slough the count
/// now versus hold the count-trump.
///
/// The LITERAL plunge position is not reconstructible from this
/// repository — its game seeds live plunge-side (ruling L2-A6 cards that
/// as [[gran-anchor-reconstruction]]) — so this is a deterministic family
/// of positions with the specimen's shape, honestly labeled.
pub struct CountTimingSpec {
    pub g: u64,
    pub deal: [DominoSet; 4],
    /// S2's driven reply to the 6-6 lead (level-0 field on its true hand).
    pub s2_tile: Domino,
    pub trick_plays: Vec<Domino>,
    pub voids: [ContextSet; 4],
}

impl CountTimingSpec {
    /// Deterministic family member `g`: deal the fixed tiles (S1 the 6-6,
    /// S3 the 6-2 and 6-4), fill S3 from the shuffled non-six pool so its
    /// legal set stays {6-2, 6-4}, shuffle the remainder over S1/S2/S4,
    /// and drive the two pre-focal plays (S1 pinned to the specimen's 6-6
    /// lead; S2 by the level-0 field at the declared drive n0).
    pub fn new(g: u64, n0_drive: usize) -> CountTimingSpec {
        let decl = Decl::PipTrump(Pip::new(6).expect("pip 6"));
        let six_six = tile((6, 6));
        let six_four = tile((6, 4));
        let six_two = tile((6, 2));
        let mut rng = SplitMix64(COUNT_TIMING_SEED ^ mix(g));
        let mut non_six: Vec<Domino> = Vec::new();
        let mut other_six: Vec<Domino> = Vec::new();
        for i in 0..DominoSet::FULL.len() {
            let d = Domino::from_index(i).expect("tile < 28");
            if d == six_six || d == six_four || d == six_two {
                continue;
            }
            if d.hi().value() == 6 {
                other_six.push(d);
            } else {
                non_six.push(d);
            }
        }
        assert_eq!(non_six.len(), 21, "twenty-one non-six tiles remain");
        assert_eq!(other_six.len(), 4, "four other sixes remain");
        shuffle(&mut non_six, &mut rng);
        let mut deal = [DominoSet::EMPTY; 4];
        assert!(deal[1].insert(six_six));
        assert!(deal[3].insert(six_two));
        assert!(deal[3].insert(six_four));
        for d in non_six.drain(..5) {
            assert!(deal[3].insert(d));
        }
        let mut rest: Vec<Domino> = non_six;
        rest.extend(other_six);
        shuffle(&mut rest, &mut rng);
        for d in rest.drain(..6) {
            assert!(deal[1].insert(d));
        }
        for d in rest.drain(..7) {
            assert!(deal[2].insert(d));
        }
        for d in rest.drain(..7) {
            assert!(deal[0].insert(d));
        }
        assert!(rest.is_empty(), "the deal covers all 28 tiles");
        for hand in &deal {
            assert_eq!(hand.len(), 7, "seven tiles per seat");
        }
        // Drive S1's 6-6 lead (the specimen's recorded play) and S2's
        // level-0 reply on its true hand.
        let led = decl.led_context(six_six);
        let mut s2_hand = deal[2];
        let legal_s2 = legal_plays(decl, s2_hand, Some(led));
        let position_for_s2 = RootPosition {
            decl,
            bid: 30,
            declaring_team: Team::T1,
            leader: Seat::S1,
            banked: [0, 0],
            trick_plays: vec![six_six],
            prior_played: DominoSet::EMPTY,
            voids: [ContextSet::EMPTY; 4],
        };
        let field = Level0Field::new(n0_drive);
        let trick_plays_s2 = [six_six];
        let record = PublicRecord {
            leader: Seat::S1,
            trick_plays: &trick_plays_s2,
            banked: [0, 0],
            root: &position_for_s2,
            history: &[],
        };
        let s2_tile = field.choose(decl, s2_hand, legal_s2, &record);
        assert!(s2_hand.remove(s2_tile), "S2 holds its reply");
        let mut voids = [ContextSet::EMPTY; 4];
        if !decl.follows(s2_tile, led) {
            voids[Seat::S2.index()].insert(led);
        }
        CountTimingSpec {
            g,
            deal,
            s2_tile,
            trick_plays: vec![six_six, s2_tile],
            voids,
        }
    }

    /// The focal root: S3 to act at trick 1 ply 2 with legal exactly
    /// {6-2, 6-4}.
    pub fn root(&self) -> FlipRoot {
        let decl = Decl::PipTrump(Pip::new(6).expect("pip 6"));
        let state = DrivenState {
            decl,
            bid: 30,
            declaring_team: Team::T1,
            viewer_hand: self.deal[3],
            leader: Seat::S1,
            trick_plays: &self.trick_plays,
            banked: [0, 0],
            prior_played: DominoSet::EMPTY,
            voids: self.voids,
        };
        let (root, position) = driven_root(&state).expect("a lawful trick-1 kernel");
        let led = decl.led_context(self.trick_plays[0]);
        let legal = legal_plays(decl, self.deal[3], Some(led));
        let legal_tiles: Vec<Domino> = legal.iter().collect();
        assert_eq!(
            legal_tiles,
            vec![tile((6, 2)), tile((6, 4))],
            "the count-timing decision is exactly 6-2 versus 6-4"
        );
        FlipRoot {
            root,
            position,
            legal_tiles,
            focal: Seat::S3,
        }
    }
}

fn shuffle(tiles: &mut [Domino], rng: &mut SplitMix64) {
    for i in (1..tiles.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        tiles.swap(i, j);
    }
}

/// A count-timing candidate tuple: the shadow tuple shape with the
/// family's own library label (these candidates never claim to be shadow
/// records — different provenance, different content address).
pub fn count_timing_tuple(position: &RootPosition, pinned: Domino) -> FreezeTuple {
    FreezeTuple {
        policy_library: "v5-count-timing-library-v1".to_string(),
        ..shadow_tuple(position, pinned)
    }
}

// ---------------------------------------------------------------------------
// Receipt-root convenience shared by tests and bins.
// ---------------------------------------------------------------------------

/// The canonical objects at the start of `trick_no` of a receipt hand
/// (the trick leader is the viewer).
pub fn receipt_root(hand: &ReceiptHand, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a lawful receipt kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a lawful position");
    (CanonicalRoot::new(kernel), position)
}
