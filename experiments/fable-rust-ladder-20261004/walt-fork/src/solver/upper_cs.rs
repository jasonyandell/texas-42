//! `solver::upper_cs` — the δ-valid admissible-upper E3 producer: the
//! max-preserving upper confidence sequence over the empirical-optimum
//! split-reach count, with its directional variants (slice 4a).
//!
//! EXPLORATORY tier. **[L2 thread]**, consuming the CE evidence engine
//! through the sanctioned one-directional crossing (L2 consumes CE
//! machinery; CE never consumes L2). Implements Part 1 of the x:024
//! response (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`
//! §§1.1–1.10, proof ledger P1–P4; intake companion
//! `walt/math/response_deferred_producers_triple_v0.1_intake.md`), adopted
//! by rulings **TRIPLE-A2/A3** (`walt/CENSUS-RULINGS.md`, "The
//! deferred-producers adjudication (2026-08-25)"). Nothing here promotes
//! the response's tier: the mathematics is implemented and cited, and the
//! gates are release tests, not receipts.
//!
//! The construction (Theorem M1 + Corollary M2, §§1.3–1.4):
//!
//! - To upper-bound `R_a = sup_ρ Pr(D_ρ = 1)` it suffices that ONE fixed
//!   true maximizer — belief-selected, never sample-selected — has a valid
//!   anytime upper confidence sequence. The branchwise maximum of upper
//!   endpoints therefore covers `R_a` at the SAME δ: **no Bonferroni
//!   split, no |Π_a| factor** (branch-mixture e-processes are RETIRED for
//!   upper bounds on maxima — wrong orientation, §1.1).
//! - Endpoint monotonicity in the success count (§1.2) collapses
//!   `max_ρ U_{δ,N}(S_{ρ,n}, n)` to `U_{δ,N}(S*_n, n)` — one integer.
//!   The shipped [`sampled_split_reach`] count IS `S*_n` (verified at
//!   intake: one shared information-consistent policy across the sample,
//!   the E4 walk's same-action-per-public-node invariant).
//! - The reported bound is the §1.4 nested prefix minimum
//!   `R̂^E3_{a,n} = min_{t ≤ n} U_{δ,N}(S*_t, t)`: nonincreasing, with
//!   coverage `Pr(∃n: R_a > R̂^E3_{a,n}) ≤ δ` preserved (P3).
//! - Pathwise `R̂^E3 ≤ R̂^E2-sample` against the fused baseline under the
//!   same engine and risk (P4, the max-versus-sum-of-max inequality).
//!
//! Standing hypotheses, declared here because they are the claim's
//! preconditions (§1.3, intake "hypotheses that must stay declared"):
//! a FIXED policy class for the evidence epoch (no data-dependent mutation
//! of policy identities inside an epoch — the class id travels on every
//! result); the declared world stream is i.i.d. from the fixed belief at
//! the epoch (the kernel's exactly-uniform with-replacement sampler); and
//! the grid `G_N = {0, 1/N, …, 1}` uses the KNOWN fiber size `N` from the
//! exact fiber counter, so every true policy mean lies on the grid.
//!
//! Directional variants (§1.5; TRIPLE-A3): the same construction on the
//! indicators `X⁺ = 1{u1 = 1, u0 = 0}` and `X⁻ = 1{u1 = 0, u0 = 1}`, as
//! SEPARATE solves with SEPARATE risk entries; the maximizing policies may
//! differ, and the coupled branches run to decided terminals (the
//! PANEL-A8 cost note, unchanged). The directional count is the fused
//! directional optimum of [`sampled_directional_count`], whose cross-group
//! focal fusion is the safe direction only: it pathwise DOMINATES the
//! count of every single information-consistent ρ ∈ Π_a — in particular
//! the fixed true directional maximizer's — so the P1–P3 chain applies
//! verbatim through endpoint monotonicity and the bound stays δ-valid,
//! merely conservative. It is never called the one-policy empirical
//! optimum itself, and its method label says so.
//!
//! §1.8 — where the risk payment remains: the no-|Π_a| result is inside
//! ONE scalar maximum. Risks across DISTINCT screen inputs (per action,
//! per direction, baselines) still sum against the screen budget:
//! [`assert_screen_risk_allocation`] enforces exactly that arithmetic over
//! the existing [`ScopedDelta`] declared-allocation discipline (a δ never
//! travels without its scope; no new ledger).
//!
//! §1.9 — typing: [`SplitReachUpper`] is the admissible-upper sibling of
//! the ESTIMATE-tier [`crate::solver::exposure::SplitReachSampled`]
//! (which stays, unchanged in meaning, and still cannot enter a screen).
//! Only the symmetric direction converts — via [`SplitReachUpper::screen_upper`]
//! — into a rung-E3 [`RootActionExposureUpper`], the screen's one entry
//! type; the directional pair enters the directional screen through
//! [`directional_screen_upper`]. An exact E4 result replaces a sampled E3
//! upper with δ = 0; a sampled E3 upper may exceed the exact E4 value and
//! at its declared failure probability may under-cover, and it is NEVER
//! described as tighter than exact E4 because a realized number happens to
//! be smaller.

use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

use crate::rules::Domino;
use crate::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use crate::solver::evidence::{crossed, lower_threshold_evidence, ScopedDelta};
use crate::solver::exposure::{
    sampled_directional_count, sampled_split_reach, DirectionalObjective, ExposureRung,
    RootActionDirectionalUpper, RootActionExposureUpper,
};
use crate::solver::field::{FieldId, FieldModel};

/// The one policy class of this producer family: all deterministic
/// information-consistent continuations after the fixed root action (the
/// class the E4 walk's per-public-node choice ranges over). Fixed for the
/// evidence epoch — the §1.3 hypothesis — and carried on every result.
pub const POLICY_CLASS_INFO_CONSISTENT: &str = "info-consistent-continuations-v1";

/// §1.2 — the exact one-mean upper confidence endpoint `U_{δ,N}(s, n)`:
/// the largest `c` on the grid `G_N = {0, 1/N, …, 1}` whose lower-tail
/// evidence `E⁻_{s,n−s}(c)` has not crossed `1/δ`, with the endpoint
/// conventions `c = 0` is never rejected and `c = 1` remains possible
/// exactly when `s = n`. Direct inversion of the existing CE-T2 anytime
/// test ([`lower_threshold_evidence`], consumed verbatim) at the true grid
/// point, so `Pr_p(∃n: p > U_{δ,N}(S_n, n)) ≤ δ` for every `p ∈ G_N` (P1).
///
/// `E⁻_{s,f}(c) = E>_{f,s}(1−c)` is nondecreasing in `c`: every term of
/// the §3.1 finite sum carries `R^i` with `R = c/(1−c)` nondecreasing in
/// `c` on `(0,1)`, so the survival set `{c ∈ G_N : E⁻ < 1/δ}` is
/// downward-closed and the maximum survivor is found by bisection.
pub fn grid_upper_endpoint(s: u64, n: u64, grid: u128, delta: &BigRational) -> BigRational {
    assert!(n >= 1, "an endpoint needs at least one observation");
    assert!(s <= n, "successes never exceed observations");
    assert!(grid >= 1, "the grid G_N needs N >= 1");
    assert!(
        delta > &BigRational::zero() && delta < &BigRational::one(),
        "a risk budget lies strictly inside (0,1)"
    );
    if s == n {
        // §1.2 endpoint convention: with zero observed failures the point
        // c = 1 remains possible, and it is the grid maximum.
        return BigRational::one();
    }
    let threshold = BigRational::one() / delta;
    let survives = |k: u128| -> bool {
        let c = BigRational::new(BigInt::from(k), BigInt::from(grid));
        !crossed(&lower_threshold_evidence(s, n - s, &c), &threshold)
    };
    // Bisection invariant: c = lo/N survives (c = 0 never rejected),
    // c = hi/N is rejected (c = 1 is rejected here because s < n).
    let mut lo: u128 = 0;
    let mut hi: u128 = grid;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if survives(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    BigRational::new(BigInt::from(lo), BigInt::from(grid))
}

/// §1.4 — the nested prefix bounds over one declared count path: entry
/// `t − 1` is `min_{t' ≤ t} U_{δ,N}(counts[t'−1], t')`. The running
/// minimum makes the reported bound nonincreasing without changing its
/// coverage (P3 step 4: an intersection over prefixes of anytime-valid
/// events).
pub fn nested_prefix_uppers(counts: &[u64], grid: u128, delta: &BigRational) -> Vec<BigRational> {
    assert!(
        !counts.is_empty(),
        "a declared prefix holds at least one world"
    );
    let mut out: Vec<BigRational> = Vec::with_capacity(counts.len());
    for (i, &s) in counts.iter().enumerate() {
        let t = u64::try_from(i + 1).expect("a declared prefix fits u64");
        let u = grid_upper_endpoint(s, t, grid, delta);
        let running = match out.last() {
            Some(previous) if previous < &u => previous.clone(),
            _ => u,
        };
        out.push(running);
    }
    out
}

/// Which supremum a [`SplitReachUpper`] bounds: the symmetric split-reach
/// mass `R_a`, or one of the §1.5 directional correction masses `R_a^±`.
/// Directions are distinct claims with distinct risk entries (TRIPLE-A3);
/// only the symmetric direction may enter the symmetric §7.4 screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E3Direction {
    /// `R_a = sup_ρ Pr(D_ρ = 1)` — the split-reach exposure mass.
    Symmetric,
    /// `R_a^+ = sup_ρ Pr(u1 = 1, u0 = 0)` — the field upgrade turns fail
    /// into make.
    Plus,
    /// `R_a^- = sup_ρ Pr(u1 = 0, u0 = 1)`.
    Minus,
}

impl E3Direction {
    pub fn label(self) -> &'static str {
        match self {
            E3Direction::Symmetric => "symmetric",
            E3Direction::Plus => "plus",
            E3Direction::Minus => "minus",
        }
    }
}

impl fmt::Display for E3Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// §1.9 — the δ-valid admissible-upper E3 result: the typed sibling of the
/// ESTIMATE-tier [`crate::solver::exposure::SplitReachSampled`], carrying
/// (action, direction, upper, δ, stream epoch, prefix length, policy-class
/// id, method). The stored authority is the declared prefix-count path;
/// the reported upper and the per-prefix bounds are DERIVED views
/// (recomputed on demand, never a second stored authority).
///
/// This type — and only its `Symmetric` direction — converts into the
/// §7.4 screen entry [`RootActionExposureUpper`] at rung E3 via
/// [`Self::screen_upper`]. The estimate sibling still has no such route.
/// A sampled E3 upper may exceed the exact E4 value; it is never described
/// as tighter than exact E4 on realized numbers (§1.9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitReachUpper {
    /// The fixed root action `a`.
    pub action: Domino,
    /// The bounded supremum's direction.
    pub direction: E3Direction,
    /// `FieldId(σ0)`.
    pub field0: FieldId,
    /// `FieldId(σ1)`.
    pub field1: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The stream epoch of the declared prefix.
    pub epoch: u64,
    /// `N = |Φ(I)|` from the exact fiber counter — grid validity: every
    /// true policy mean lies on `G_N` (§1.2 hypothesis).
    pub grid: u128,
    /// The fixed policy class of the evidence epoch (§1.3 hypothesis).
    pub policy_class: &'static str,
    delta: ScopedDelta,
    counts: Vec<u64>,
}

impl SplitReachUpper {
    /// The typed carrier over a declared prefix-count path. The counts'
    /// validity — that entry `t − 1` pathwise dominates the count of
    /// every fixed ρ ∈ Π_a on the stream prefix `0..t` — is the
    /// producer's obligation ([`e3_split_reach_upper`],
    /// [`e3_directional_upper`]); the constructor asserts the shape every
    /// lawful count path has: `s_t ≤ t`, nondecreasing, unit steps.
    #[allow(clippy::too_many_arguments)]
    pub fn from_prefix_counts(
        action: Domino,
        direction: E3Direction,
        field0: FieldId,
        field1: FieldId,
        root_id: u64,
        epoch: u64,
        grid: u128,
        policy_class: &'static str,
        delta: ScopedDelta,
        counts: Vec<u64>,
    ) -> SplitReachUpper {
        assert!(grid >= 1, "the grid G_N needs N >= 1");
        assert!(
            !counts.is_empty(),
            "a declared prefix holds at least one world"
        );
        for (i, &s) in counts.iter().enumerate() {
            let t = u64::try_from(i + 1).expect("a declared prefix fits u64");
            assert!(s <= t, "a prefix count never exceeds its prefix length");
            if i > 0 {
                assert!(
                    counts[i - 1] <= s && s <= counts[i - 1] + 1,
                    "a prefix-optimum path is nondecreasing with unit steps"
                );
            }
        }
        SplitReachUpper {
            action,
            direction,
            field0,
            field1,
            root_id,
            epoch,
            grid,
            policy_class,
            delta,
            counts,
        }
    }

    /// The declared prefix length `n`.
    pub fn prefix(&self) -> u64 {
        u64::try_from(self.counts.len()).expect("a declared prefix fits u64")
    }

    /// The stored count path: entry `t − 1` is the declared optimum over
    /// the stream prefix `0..t`.
    pub fn counts(&self) -> &[u64] {
        &self.counts
    }

    /// The declared risk entry — a δ that never travels without its scope.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }

    /// §1.9 — the method label. The symmetric count is the one-policy
    /// empirical optimum (Corollary M2); the directional count is the
    /// fused directional optimum, a pathwise dominator of the empirical
    /// optimum (safe direction only), and its label keeps the distinction.
    pub fn method(&self) -> &'static str {
        match self.direction {
            E3Direction::Symmetric => "empirical-optimum-upper-cs",
            E3Direction::Plus | E3Direction::Minus => "fused-directional-optimum-upper-cs",
        }
    }

    /// The §1.4 nested per-prefix bounds — a derived view of the counts.
    pub fn prefix_uppers(&self) -> Vec<BigRational> {
        nested_prefix_uppers(&self.counts, self.grid, self.delta.delta())
    }

    /// The reported bound `R̂^E3` at the declared prefix: the last nested
    /// prefix bound, `min_{t ≤ n} U_{δ,N}(S*_t, t)`.
    pub fn upper(&self) -> BigRational {
        self.prefix_uppers()
            .pop()
            .expect("a declared prefix holds at least one world")
    }

    /// The §7.4 screen admission: the symmetric bound as a rung-E3
    /// [`RootActionExposureUpper`] — the accessor the screen's interval
    /// arithmetic consumes, which the ESTIMATE sibling deliberately lacks.
    /// A directional bound is NOT an upper bound on `R_a` (it bounds
    /// `R_a^±`, which sits below `R_a`), so only the symmetric direction
    /// has this route; the directional pair enters the directional screen
    /// through [`directional_screen_upper`].
    pub fn screen_upper(&self) -> RootActionExposureUpper {
        assert_eq!(
            self.direction,
            E3Direction::Symmetric,
            "only the symmetric E3 bound is an upper bound on R_a; the \
             directional pair feeds the directional screen instead"
        );
        RootActionExposureUpper::from_rung(ExposureRung::E3, self.upper())
    }
}

impl fmt::Display for SplitReachUpper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SplitReachUpper{{rung=E3;direction={};upper={};action={};field0={};field1={};\
             root={:#018x};epoch={};prefix={};grid={};policy_class={};method={};{}}}",
            self.direction,
            self.upper(),
            self.action,
            self.field0,
            self.field1,
            self.root_id,
            self.epoch,
            self.prefix(),
            self.grid,
            self.policy_class,
            self.method(),
            self.delta
        )
    }
}

/// Produce the symmetric δ-valid admissible-upper E3 bound for one root
/// action over the declared stream prefix `0..prefix` at `epoch`
/// (TRIPLE-A2): per prefix length `t`, the empirical-optimum count `S*_t`
/// from [`sampled_split_reach`] (the E4 walk verbatim on the declared
/// multiset — one shared information-consistent policy across the
/// sample), inverted through [`grid_upper_endpoint`] on `G_N` with
/// `N = |Φ(I)|` from the exact fiber counter, prefix minimum taken.
///
/// The empirical maximizer may change with `t`; that causes no selection
/// defect (Theorem M1 compares against one fixed true maximizer, §1.4).
#[allow(clippy::too_many_arguments)]
pub fn e3_split_reach_upper(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field0: &FieldModel,
    field1: &FieldModel,
    epoch: u64,
    prefix: u64,
    delta: ScopedDelta,
) -> SplitReachUpper {
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let counts: Vec<u64> = (1..=prefix)
        .map(|t| {
            sampled_split_reach(root, position, action, field0, field1, epoch, t).frontier_worlds
        })
        .collect();
    SplitReachUpper::from_prefix_counts(
        action,
        E3Direction::Symmetric,
        field0.field_id(),
        field1.field_id(),
        root_identity(root, position),
        epoch,
        root.count(),
        POLICY_CLASS_INFO_CONSISTENT,
        delta,
        counts,
    )
}

/// Produce one directional δ-valid E3 bound (§1.5; TRIPLE-A3): the same
/// construction on the directional indicator, as a SEPARATE solve with its
/// own risk entry. The per-prefix count is the fused directional optimum
/// of [`sampled_directional_count`] (coupled branches run to decided
/// terminals — the PANEL-A8 cost note applies unchanged), which pathwise
/// dominates the count of every single ρ ∈ Π_a, so the inverted bound is
/// δ-valid for `R_a^±`, conservatively.
#[allow(clippy::too_many_arguments)]
pub fn e3_directional_upper(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    direction: E3Direction,
    field0: &FieldModel,
    field1: &FieldModel,
    epoch: u64,
    prefix: u64,
    delta: ScopedDelta,
) -> SplitReachUpper {
    let objective = match direction {
        E3Direction::Plus => DirectionalObjective::Plus,
        E3Direction::Minus => DirectionalObjective::Minus,
        E3Direction::Symmetric => {
            panic!("the symmetric producer is e3_split_reach_upper")
        }
    };
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let counts: Vec<u64> = (1..=prefix)
        .map(|t| {
            sampled_directional_count(root, position, action, objective, field0, field1, epoch, t)
        })
        .collect();
    SplitReachUpper::from_prefix_counts(
        action,
        direction,
        field0.field_id(),
        field1.field_id(),
        root_identity(root, position),
        epoch,
        root.count(),
        POLICY_CLASS_INFO_CONSISTENT,
        delta,
        counts,
    )
}

/// The directional screen admission (§1.5; TRIPLE-A3): one `Plus` and one
/// `Minus` result of the SAME claim identity — action, field pair, root,
/// epoch, grid, policy class — with SEPARATE risk entries (distinct
/// scopes asserted: separate solves, separate ledger entries), paired into
/// the [`RootActionDirectionalUpper`] the directional screen consumes.
pub fn directional_screen_upper(
    plus: &SplitReachUpper,
    minus: &SplitReachUpper,
) -> RootActionDirectionalUpper {
    assert_eq!(plus.direction, E3Direction::Plus, "a Plus bound first");
    assert_eq!(minus.direction, E3Direction::Minus, "a Minus bound second");
    assert_eq!(plus.action, minus.action, "one root action");
    assert_eq!(plus.field0, minus.field0, "one σ0");
    assert_eq!(plus.field1, minus.field1, "one σ1");
    assert_eq!(plus.root_id, minus.root_id, "one root");
    assert_eq!(plus.epoch, minus.epoch, "one stream epoch");
    assert_eq!(plus.grid, minus.grid, "one grid");
    assert_eq!(plus.policy_class, minus.policy_class, "one policy class");
    assert_ne!(
        plus.delta().scope(),
        minus.delta().scope(),
        "directional solves are separate risk entries (TRIPLE-A3)"
    );
    RootActionDirectionalUpper::from_bounds(plus.upper(), minus.upper())
}

/// §1.8 / TRIPLE-A2 — the screen-budget arithmetic: the theorem removes
/// the policy-class penalty INSIDE each scalar maximum, and nothing else.
/// Risks across distinct screen inputs (per action, per direction,
/// baselines) still sum against the screen budget. This asserts exactly
/// that over the existing [`ScopedDelta`] declared-allocation discipline
/// — distinct inputs carry distinct scopes, and the exact rational sum is
/// at most the budget — and returns the sum. No new ledger.
pub fn assert_screen_risk_allocation(
    budget: &ScopedDelta,
    entries: &[&ScopedDelta],
) -> BigRational {
    let mut total = BigRational::zero();
    for (i, entry) in entries.iter().enumerate() {
        for other in &entries[i + 1..] {
            assert_ne!(
                entry.scope(),
                other.scope(),
                "distinct screen inputs carry distinct risk scopes"
            );
        }
        total += entry.delta();
    }
    assert!(
        &total <= budget.delta(),
        "screen-input risks sum to at most the screen budget ({budget})"
    );
    total
}
