//! `solver::controller` — the §16.4 decision controller: the fixed-candidate
//! adaptive evaluator generalized to `m` frozen candidates with safe
//! elimination (§22 step 5 of the calculated-evidence build program).
//!
//! EXPLORATORY tier. Implements the verbatim parent
//! `walt/math/calculated_evidence_v0.1.md` §5 (decision-level risk ledger,
//! all-pairs allocation, safe elimination, epoch mutation), §6 (run-level
//! allocation), §9.2/§9.3 (practical equivalence), §8.5 (the per-decision
//! refinement vector), §11.3/§11.4 (exact escalation and sampled-work
//! reuse), and §17 (counter-based common random worlds); adjudicated at
//! rulings CE-A1..A8 (`walt/CENSUS-RULINGS.md`), obligations O21/O22/O24/O26.
//!
//! Design commitments, in the parent's order:
//!
//! - **All-pairs risk allocation (§5).** The deliberately conservative
//!   first implementation: every ordered pair gets an evidence process
//!   under `T_edge = m(m-1)/δ_edges` from `evidence::edge_threshold`. The
//!   §5.2 one-at-a-time allocation is explicitly LATER and is not built.
//! - **Epoch identity (§5.3).** The evaluation epoch is a content address
//!   of (root identity, the sorted candidate `PolicyId`s, the declared
//!   δ_dec, the sampling declaration). Mutating the candidate set changes
//!   the epoch, the epoch changes every stream seed, so old evidence is
//!   unreachable from a new epoch by construction — nothing is
//!   reinterpreted.
//! - **Common random worlds (§17.2).** Every LIVE candidate evaluates
//!   world `i` (cache readout after first materialization) before any pair
//!   evidence updates at index `i`. Eliminated candidates stop consuming
//!   future worlds; previously accumulated pair counts stay aligned by
//!   world ID and are frozen, never rewritten.
//! - **Safe elimination (§5.1).** A candidate is removed when any live
//!   candidate has a settled directed edge into it. When one candidate
//!   survives it is the true maximizer of the fixed set on the
//!   no-false-edge event. Several equal-valued survivors may persist
//!   forever — that is honest; no winner is ever forced.
//! - **Resource caps (§1.5, CE-A3/A5).** A world cap is a resource limit
//!   producing `Unresolved` with §8.5 refinement vectors persisted — never
//!   a settlement rule.
//! - **Exact escalation (§11.3, O24).** The switch rule compares the exact
//!   remaining enumeration cost `N_rem·c_enum` against the exact
//!   `h±_min`-based LOWER bound on remaining sampling cost — an integer
//!   comparison, no logarithms. A wrong cost forecast affects performance
//!   only, never the result (gated by forced-switch parity).
//!
//! Result kinds are the parent-§1 ladder at set level: `ExactFrozenSet`,
//! `DeltaSettled`, `EpsilonEquivalent`, `Unresolved`, mechanically distinct
//! with the type tag as the serialization prefix (CE-A3). `ExactFiberRoot`
//! (the root endpoint, not a candidate-set statement) and
//! `HeuristicFallback` (a consumer's explicitly named choice AFTER
//! `Unresolved`, never the controller's) are out of this module's
//! production set by construction; the six-way ladder itself lives in
//! `solver::adaptive::ResultKind`.

use std::collections::HashMap;
use std::fmt;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed};

use crate::kernel::World;
use crate::solver::adaptive::{
    replay_viewer_success, root_identity, CanonicalRoot, RootPosition, SlicePolicy, StreamIdentity,
    SAMPLER_ID,
};
use crate::solver::evidence::{self, BoundedMeanMixture, MeanNull, ScopedDelta};
use crate::solver::policy::{content_digest, FrozenPolicy, PolicyId};

// ---------------------------------------------------------------------------
// The candidate set and its epoch identity (§5.3).
// ---------------------------------------------------------------------------

/// A fixed set of `m ≥ 2` frozen candidates with distinct [`PolicyId`]s.
/// The set is immutable for the whole evidence epoch: any mutation is a new
/// `CandidateSet`, a new epoch, and fresh worlds (§5.3).
pub struct CandidateSet<'a> {
    members: Vec<&'a FrozenPolicy>,
}

impl<'a> CandidateSet<'a> {
    /// Validates `m ≥ 2` and pairwise-distinct `PolicyId`s.
    pub fn new(members: Vec<&'a FrozenPolicy>) -> CandidateSet<'a> {
        assert!(
            members.len() >= 2,
            "a decision compares at least two candidates"
        );
        for (x, one) in members.iter().enumerate() {
            for other in &members[x + 1..] {
                assert!(
                    one.policy_id() != other.policy_id(),
                    "candidate PolicyIds are distinct"
                );
            }
        }
        CandidateSet { members }
    }

    pub fn m(&self) -> usize {
        self.members.len()
    }

    pub fn get(&self, k: usize) -> &'a FrozenPolicy {
        self.members[k]
    }

    /// PolicyIds in candidate-index order.
    pub fn ids(&self) -> Vec<PolicyId> {
        self.members.iter().map(|p| p.policy_id()).collect()
    }

    /// PolicyIds sorted bytewise — the epoch folds these, so the epoch is
    /// invariant under candidate input order.
    pub fn sorted_ids(&self) -> Vec<PolicyId> {
        let mut ids = self.ids();
        ids.sort_by(|x, y| x.bytes().cmp(y.bytes()));
        ids
    }
}

/// §5.3 — the evaluation epoch's content address (SHA-256 over a canonical
/// serialization). Old evidence lives under the old epoch's stream seeds;
/// a new epoch derives disjoint streams, so old pair counts are
/// unreachable from it by construction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EpochId([u8; 32]);

impl EpochId {
    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The first eight bytes as a big-endian `u64`: the `epoch` folded
    /// into every world-stream seed derivation (§17.1).
    pub fn stream_epoch(&self) -> u64 {
        u64::from_be_bytes([
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
        ])
    }
}

impl fmt::Display for EpochId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Length-prefixed, tag-separated field append — the same aliasing
/// discipline as `solver::policy`'s canonical serialization.
fn epoch_field(bytes: &mut Vec<u8>, tag: u8, payload: &[u8]) {
    bytes.push(tag);
    bytes.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    bytes.extend_from_slice(payload);
}

/// The epoch identity of §5.3: folded from the root identity, the SORTED
/// candidate `PolicyId`s, the declared δ_dec (scope and exact value), and
/// the sampling declaration (sampler identity, with-replacement). Adding,
/// removing, or reidentifying any candidate changes the address; candidate
/// input order does not.
pub fn epoch_identity(
    root_id: u64,
    candidates: &CandidateSet<'_>,
    delta_dec: &ScopedDelta,
) -> EpochId {
    let mut bytes: Vec<u8> = Vec::new();
    epoch_field(&mut bytes, 0x00, b"walt-epoch-v1");
    epoch_field(&mut bytes, 0x01, &root_id.to_be_bytes());
    let mut ids: Vec<u8> = Vec::new();
    for id in candidates.sorted_ids() {
        ids.extend_from_slice(id.bytes());
    }
    epoch_field(&mut bytes, 0x02, &ids);
    epoch_field(&mut bytes, 0x03, delta_dec.scope().as_bytes());
    // δ ∈ (0,1) strictly (ScopedDelta's contract) and BigRational keeps
    // denominators positive, so both magnitudes carry the full value.
    epoch_field(&mut bytes, 0x04, &delta_dec.delta().numer().to_bytes_be().1);
    epoch_field(&mut bytes, 0x05, &delta_dec.delta().denom().to_bytes_be().1);
    epoch_field(&mut bytes, 0x06, SAMPLER_ID.as_bytes());
    epoch_field(&mut bytes, 0x07, &[1u8]); // sampling with replacement (§11.2)
    EpochId(content_digest(&bytes))
}

// ---------------------------------------------------------------------------
// The risk plan and the decision ledger (§5, §6, §9.2/§9.3; O21).
// ---------------------------------------------------------------------------

/// Which practical-equivalence engine a pair runs (§9.2/§9.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquivalenceRoute {
    /// §9.2 — CE-T4 at `c = -ε` and CE-T5 at `c = +ε` on the bounded-mean
    /// difference `X = Y ∈ {-1,0,+1} ⊂ [-1,1]`; two one-sided tests per
    /// unordered pair, each separately allocated.
    BoundedMean,
    /// §9.3 — the conservative pivotal-mass route: `|g| ≤ q`, so settling
    /// `q < ε` by the lower-threshold Bernoulli process on the pivotal
    /// indicator `P = |Y|` suffices; one test per unordered pair. It
    /// cannot recognize a large-`q` balanced tie — that is its declared
    /// conservatism, not a defect.
    PivotalMass,
}

impl EquivalenceRoute {
    fn label(self) -> &'static str {
        match self {
            EquivalenceRoute::BoundedMean => "bounded-mean",
            EquivalenceRoute::PivotalMass => "pivotal-mass",
        }
    }

    /// One-sided tests allocated per unordered pair.
    fn tests_per_pair(self) -> u64 {
        match self {
            EquivalenceRoute::BoundedMean => 2,
            EquivalenceRoute::PivotalMass => 1,
        }
    }
}

/// §9.2 — the optional per-run practical-equivalence configuration: a
/// rational tolerance ε, a risk sub-budget δ_eq charged to the decision
/// ledger, and the route.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquivalencePlan {
    pub epsilon: BigRational,
    pub delta: BigRational,
    pub route: EquivalenceRoute,
}

/// The decision's complete declared risk allocation. Constructors enforce
/// the O21 arithmetic up front: everything allocated within the decision
/// sums to at most the decision budget, and a decision budget claimed
/// under a run ledger is at most its §6 allocation `δ_run/(d(d+1))`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RiskPlan {
    decision: ScopedDelta,
    run: Option<(ScopedDelta, u64)>,
    edges: BigRational,
    equivalence: Option<EquivalencePlan>,
}

impl RiskPlan {
    /// Strict mode (§9.1): the whole decision budget backs the directed
    /// edges; a true tie may return `Unresolved`, honestly.
    pub fn strict(decision: ScopedDelta) -> RiskPlan {
        let edges = decision.delta().clone();
        RiskPlan {
            decision,
            run: None,
            edges,
            equivalence: None,
        }
    }

    /// Practical-equivalence mode (§9.2): `edges + equivalence.delta ≤`
    /// the decision budget, checked exactly.
    pub fn with_equivalence(
        decision: ScopedDelta,
        edges: BigRational,
        equivalence: EquivalencePlan,
    ) -> RiskPlan {
        assert!(edges.is_positive(), "the edge sub-budget is positive");
        assert!(
            equivalence.delta.is_positive(),
            "the equivalence sub-budget is positive"
        );
        assert!(
            equivalence.epsilon.is_positive() && equivalence.epsilon < BigRational::one(),
            "a utility tolerance on Y in [-1,1] lies strictly inside (0,1)"
        );
        assert!(
            &edges + &equivalence.delta <= *decision.delta(),
            "allocations within a decision sum to at most the decision budget"
        );
        RiskPlan {
            decision,
            run: None,
            edges,
            equivalence: Some(equivalence),
        }
    }

    /// Record run-level provenance (§6): this is decision event `d` of the
    /// named run, and the decision budget is at most `δ_run/(d(d+1))`.
    pub fn under_run(mut self, run: ScopedDelta, decision_ordinal: u64) -> RiskPlan {
        assert!(
            *self.decision.delta() <= evidence::decision_delta(decision_ordinal, run.delta()),
            "a decision budget is at most its run allocation delta_run/(d(d+1))"
        );
        self.run = Some((run, decision_ordinal));
        self
    }

    pub fn decision(&self) -> &ScopedDelta {
        &self.decision
    }

    pub fn run(&self) -> Option<&(ScopedDelta, u64)> {
        self.run.as_ref()
    }

    pub fn edges(&self) -> &BigRational {
        &self.edges
    }

    pub fn equivalence(&self) -> Option<&EquivalencePlan> {
        self.equivalence.as_ref()
    }
}

/// O21 — the serialized allocation carried by every produced result: run
/// provenance, decision budget, epoch, per-edge α, optional equivalence
/// tests. `allocated_total()` is the exact rational sum of everything
/// allocated; the ledger-completeness gate asserts it never exceeds the
/// declared scope budget.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionLedger {
    pub plan: RiskPlan,
    pub m: u64,
    /// Per ordered pair: `α = δ_edges / (m(m-1))`.
    pub edge_alpha: BigRational,
    /// `T_edge = m(m-1)/δ_edges` (§5).
    pub edge_threshold: BigRational,
    /// Per one-sided equivalence test, when the mode is configured.
    pub eq_alpha: Option<BigRational>,
    /// Equivalence tests allocated up front (all pairs; §5's conservative
    /// style).
    pub eq_tests: u64,
    pub epoch: EpochId,
    pub stream: StreamIdentity,
}

impl DecisionLedger {
    /// The exact rational sum of every allocated risk: `m(m-1)` directed
    /// edges at `edge_alpha` plus `eq_tests` at `eq_alpha`.
    pub fn allocated_total(&self) -> BigRational {
        let edges = BigRational::from_integer(BigInt::from(self.m * (self.m - 1)));
        let mut total = edges * &self.edge_alpha;
        if let Some(alpha) = &self.eq_alpha {
            total += BigRational::from_integer(BigInt::from(self.eq_tests)) * alpha;
        }
        total
    }

    /// The declared decision-scope budget the allocation lives inside.
    pub fn scope_budget(&self) -> &BigRational {
        self.plan.decision.delta()
    }
}

impl fmt::Display for DecisionLedger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ledger{{{}", self.plan.decision)?;
        if let Some((run, d)) = &self.plan.run {
            write!(f, ";run={run};decision_event={d}")?;
        }
        write!(
            f,
            ";m={};edge_alpha={};T_edge={}",
            self.m, self.edge_alpha, self.edge_threshold
        )?;
        if let (Some(alpha), Some(eq)) = (&self.eq_alpha, self.plan.equivalence()) {
            write!(
                f,
                ";eq_route={};eq_epsilon={};eq_alpha={alpha};eq_tests={}",
                eq.route.label(),
                eq.epsilon,
                self.eq_tests
            )?;
        }
        write!(f, ";epoch={};{}}}", self.epoch, self.stream)
    }
}

// ---------------------------------------------------------------------------
// Escalation configuration (§11.3).
// ---------------------------------------------------------------------------

/// §11.3 — declared cost weights for the exact switch rule, in abstract
/// integer work units (e.g. policy replays per world). The comparison is
/// `N_rem · cost_enumerate ≤ H_lb · cost_sample`, where `H_lb` is the
/// smallest `min(h+_min, h-_min)` over live pairs — an exact lower bound
/// on further pivotal work, so crossing it means enumeration is no more
/// work than the BEST sampling case under the declared weights. Wrong
/// weights cost performance only, never correctness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EscalationConfig {
    pub cost_sample: u64,
    pub cost_enumerate: u64,
    /// Stream-index cadence of the switch check. A declared constant of
    /// the evaluation — deliberately NOT the batch size, so throughput
    /// batching cannot change the result (§17.3, V8).
    pub check_every: u64,
}

// ---------------------------------------------------------------------------
// The evaluation request.
// ---------------------------------------------------------------------------

/// One candidate-set evaluation request at one root.
pub struct SetSpec<'a> {
    pub root: &'a CanonicalRoot,
    pub position: &'a RootPosition,
    pub candidates: &'a CandidateSet<'a>,
    /// The declared deterministic field model for non-focal seats.
    pub field: &'a dyn SlicePolicy,
    pub plan: RiskPlan,
    /// Resource cap in raw worlds — a resource limit producing
    /// `Unresolved`, never a settlement rule (§1.5, CE-A3/A5).
    pub world_cap: u64,
    /// Throughput batch size (§17.3): work grouping only; gated by V8.
    pub batch: u64,
    /// Optional automatic exact escalation (§11.3).
    pub escalation: Option<EscalationConfig>,
}

// ---------------------------------------------------------------------------
// Bookkeeping records.
// ---------------------------------------------------------------------------

/// A settled directed edge `from → to` (§5): the evidence for `from` over
/// `to` crossed `T_edge` at stream index `at` with pivotal counts `(a, b)`
/// (wins for `from`, wins for `to`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettledEdge {
    pub from: usize,
    pub to: usize,
    pub at: u64,
    pub a: u64,
    pub b: u64,
}

/// §5.1 — candidate `candidate` removed at stream index `at` because live
/// candidate `by` held a settled edge into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Elimination {
    pub candidate: usize,
    pub by: usize,
    pub at: u64,
}

/// Final pivotal counts of one unordered pair `(i, j)`: `a` wins for `i`,
/// `b` wins for `j`, over `n` common worlds folded while both were live.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairCounts {
    pub i: usize,
    pub j: usize,
    pub a: u64,
    pub b: u64,
    pub n: u64,
}

/// §8.5 — the per-pair refinement vector persisted with `Unresolved`.
///
/// Exact fields (evidence-path arithmetic, `solver::evidence`): `n`, `a`,
/// `b`, `n0`, `e_plus`, `e_minus`, `threshold`, `r_debt_plus`,
/// `r_debt_minus`, `h_plus_min`, `h_minus_min`, and `c_exact`
/// (`N_rem · cost_enumerate` with exact `N_rem`). Estimate fields are
/// exact rationals LABELED estimates: `q_hat`, `tau_hat`, `g_hat`, the
/// crude §8.3 raw-world conversions `n_hat_plus/minus = h±_min/q̂`, and
/// `c_sample_forecast`.
///
/// OMITTED FIELDS: the information rate `𝓘̂ = q̂·D_{1/2}(τ̂)` (§7) and the
/// §8.4 forecast `n̂_forecast` at a confidence level both involve
/// logarithms — transcendental arithmetic this codebase does not
/// approximate with floats. They are omitted rather than falsified; the
/// §8.4 exact-rational forecast dynamic program under a declared
/// predictive law is the future route for both. The forecast COMPARISON
/// the controller actually makes (§8.1/§8.2: `R_debt` and `h±_min`
/// against `N_rem`) needs no logarithms and is exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairRefinement {
    pub i: usize,
    pub j: usize,
    pub policy_i: PolicyId,
    pub policy_j: PolicyId,
    pub n: u64,
    pub a: u64,
    pub b: u64,
    /// Nonpivotal outcomes `n - a - b` (not an inner-mind sample size).
    pub n0: u64,
    /// Estimate: empirical pivotal rate `(a+b)/n`; `None` when `n = 0`.
    pub q_hat: Option<BigRational>,
    /// Estimate: empirical tilt `(a-b)/(a+b)`; `None` when `a+b = 0`.
    pub tau_hat: Option<BigRational>,
    /// Estimate: empirical gap `(a-b)/n`; `None` when `n = 0`.
    pub g_hat: Option<BigRational>,
    pub e_plus: BigRational,
    pub e_minus: BigRational,
    pub threshold: BigRational,
    /// §8.1 exact debt `T/E+`.
    pub r_debt_plus: BigRational,
    /// §8.1 exact debt `T/E-`.
    pub r_debt_minus: BigRational,
    /// §8.2 exact best-case additional favorable pivots, `i` over `j`.
    pub h_plus_min: u64,
    /// §8.2 mirror, `j` over `i`.
    pub h_minus_min: u64,
    /// §8.3 estimate `h+_min/q̂`; `None` when `q̂` is absent or zero.
    pub n_hat_plus: Option<BigRational>,
    /// §8.3 estimate `h-_min/q̂`; `None` when `q̂` is absent or zero.
    pub n_hat_minus: Option<BigRational>,
    /// §11.3 exact remaining enumeration cost `N_rem · cost_enumerate`
    /// under the declared weights.
    pub c_exact: BigRational,
    /// §11.3 forecast `min(n̂+, n̂-) · cost_sample`; `None` when no raw
    /// conversion is available. A forecast, labeled as such.
    pub c_sample_forecast: Option<BigRational>,
}

// ---------------------------------------------------------------------------
// Result kinds (parent §1 at set level; CE-A3).
// ---------------------------------------------------------------------------

/// The controller's producible result kinds — the parent-§1 ladder
/// restricted to what a candidate-set evaluation can honestly claim.
/// Serialization preserves the type tag as its prefix (CE-A3), and
/// `DeltaSettled` carries the full risk-ledger serialization (§17.4, O21).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetResult {
    /// §1.2 — every candidate evaluated over the complete fiber; best of
    /// the fixed set selected exactly. `winner` is `None` exactly on an
    /// exact tie among the maxima (honest output). Exact results spend no
    /// sampling risk (§6.1); the ledger is carried so the allocation
    /// remains reconstructable, its entry closable unspent.
    ExactFrozenSet {
        /// Exact win counts, candidate-index order.
        wins: Vec<u128>,
        fiber: u128,
        winner: Option<usize>,
        policy_ids: Vec<PolicyId>,
        ledger: DecisionLedger,
    },
    /// §1.3 — elimination left one survivor: best of the fixed set except
    /// on an event of probability at most the declared scope budget under
    /// the sampling law. Probabilistic, not exact.
    DeltaSettled {
        winner: usize,
        winner_id: PolicyId,
        /// Stream index at which the last rival was eliminated.
        settled_at: u64,
        ledger: DecisionLedger,
    },
    /// §1.4 — every surviving pair settled `|g| < ε` at its declared
    /// risk; all survivors are within ε of one another pairwise.
    EpsilonEquivalent {
        survivors: Vec<usize>,
        epsilon: BigRational,
        settled_at: u64,
        ledger: DecisionLedger,
    },
    /// §1.5 — the resource cap arrived first. A successful honest output;
    /// refinement vectors say where the next unit of compute should go.
    Unresolved {
        survivors: Vec<usize>,
        consumed: u64,
        refinements: Vec<PairRefinement>,
        ledger: DecisionLedger,
    },
}

impl SetResult {
    /// The mechanical type tag, always the serialization's prefix.
    pub fn tag(&self) -> &'static str {
        match self {
            SetResult::ExactFrozenSet { .. } => "ExactFrozenSet",
            SetResult::DeltaSettled { .. } => "DeltaSettled",
            SetResult::EpsilonEquivalent { .. } => "EpsilonEquivalent",
            SetResult::Unresolved { .. } => "Unresolved",
        }
    }

    pub fn ledger(&self) -> &DecisionLedger {
        match self {
            SetResult::ExactFrozenSet { ledger, .. }
            | SetResult::DeltaSettled { ledger, .. }
            | SetResult::EpsilonEquivalent { ledger, .. }
            | SetResult::Unresolved { ledger, .. } => ledger,
        }
    }
}

impl fmt::Display for SetResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetResult::ExactFrozenSet {
                wins,
                fiber,
                winner,
                policy_ids,
                ledger,
            } => {
                write!(f, "ExactFrozenSet{{wins=[")?;
                for (k, w) in wins.iter().enumerate() {
                    if k > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}:{w}", policy_ids[k])?;
                }
                let winner = match winner {
                    Some(k) => policy_ids[*k].to_string(),
                    None => "exact-tie".to_string(),
                };
                write!(f, "];fiber={fiber};winner={winner};{ledger}}}")
            }
            SetResult::DeltaSettled {
                winner_id,
                settled_at,
                ledger,
                ..
            } => write!(
                f,
                "DeltaSettled{{winner={winner_id};settled_at={settled_at};{ledger}}}"
            ),
            SetResult::EpsilonEquivalent {
                survivors,
                epsilon,
                settled_at,
                ledger,
            } => write!(
                f,
                "EpsilonEquivalent{{survivors={survivors:?};epsilon={epsilon};\
                 settled_at={settled_at};{ledger}}}"
            ),
            SetResult::Unresolved {
                survivors,
                consumed,
                refinements,
                ledger,
            } => write!(
                f,
                "Unresolved{{survivors={survivors:?};consumed={consumed};\
                 open_pairs={};{ledger}}}",
                refinements.len()
            ),
        }
    }
}

/// §11.4 bookkeeping when the exact endpoint ran: switch index and how
/// many enumerated worlds reused a sampled cache entry versus were
/// evaluated fresh.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EscalationReport {
    pub switched_at: u64,
    pub reused_worlds: u64,
    pub fresh_worlds: u64,
}

/// The controller's answer plus its audit bookkeeping. Every record is a
/// derived view of the evidence stream — the result is a function of the
/// stream, never a second authority.
pub struct SetEvaluation {
    pub result: SetResult,
    pub edges: Vec<SettledEdge>,
    pub eliminations: Vec<Elimination>,
    /// Final counts of every pair (frozen at elimination; aligned by
    /// world ID while live).
    pub pair_counts: Vec<PairCounts>,
    /// Raw worlds consumed from the stream.
    pub consumed: u64,
    /// Present exactly when the exact endpoint ran.
    pub escalation: Option<EscalationReport>,
}

// ---------------------------------------------------------------------------
// Internal state.
// ---------------------------------------------------------------------------

enum EqProcesses {
    /// §9.2 — `above` rejects `H0: g ≤ -ε` (CE-T4 at `c = -ε`); `below`
    /// rejects `H0: g ≥ +ε` (CE-T5 at `c = +ε`). Both crossings settle
    /// `-ε < g < ε`.
    /// Boxed so the stateless variant does not carry the mixtures' size.
    BoundedMean {
        above: Box<BoundedMeanMixture>,
        below: Box<BoundedMeanMixture>,
    },
    /// §9.3 — derived from the pair counts; no per-observation state.
    PivotalMass,
}

struct PairState {
    i: usize,
    j: usize,
    a: u64,
    b: u64,
    n: u64,
    settled: Option<u64>,
    eq: Option<EqProcesses>,
    eq_settled_at: Option<u64>,
}

/// The λ mixture for one §9.2 gate: components at `λ_max/2, λ_max/4,
/// λ_max/8` with weights `1/2, 1/4, 1/4`, where `λ_max = 1/(1-ε)` is the
/// CE-T4/T5 bound for range `[-1,1]` at threshold `∓ε`. Staying strictly
/// inside the bound keeps every factor strictly positive, so one adverse
/// observation can never zero the process forever.
fn equivalence_gate(
    null: MeanNull,
    threshold: BigRational,
    epsilon: &BigRational,
) -> BoundedMeanMixture {
    let lambda_max = (BigRational::one() - epsilon).recip();
    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
    let quarter = BigRational::new(BigInt::from(1), BigInt::from(4));
    let eighth = BigRational::new(BigInt::from(1), BigInt::from(8));
    let mixture = [
        (half.clone(), &lambda_max * &half),
        (quarter.clone(), &lambda_max * &quarter),
        (quarter, &lambda_max * eighth),
    ];
    BoundedMeanMixture::new(
        null,
        -BigRational::one(),
        BigRational::one(),
        threshold,
        &mixture,
    )
    .expect("the declared equivalence mixture is lawful")
}

struct Controller<'a, 'b> {
    spec: &'b SetSpec<'a>,
    m: usize,
    root_id: u64,
    epoch: u64,
    threshold: BigRational,
    eq_threshold: Option<BigRational>,
    ledger: DecisionLedger,
    live: Vec<bool>,
    pairs: Vec<PairState>,
    /// §11.4 — canonical world identity → per-candidate terminal outcomes.
    /// `None` slots belong to candidates eliminated before the world's
    /// first materialization; the exact endpoint fills them fresh.
    cache: HashMap<World, Vec<Option<bool>>>,
    edges: Vec<SettledEdge>,
    eliminations: Vec<Elimination>,
}

impl<'a, 'b> Controller<'a, 'b> {
    fn new(spec: &'b SetSpec<'a>) -> Controller<'a, 'b> {
        assert!(spec.batch >= 1, "a batch holds at least one world");
        if let Some(config) = &spec.escalation {
            assert!(
                config.check_every >= 1,
                "the switch-check cadence is at least one stream index"
            );
        }
        let m = spec.candidates.m();
        let root_id = root_identity(spec.root, spec.position);
        let epoch_id = epoch_identity(root_id, spec.candidates, spec.plan.decision());
        let epoch = epoch_id.stream_epoch();
        let stream = StreamIdentity {
            sampler: SAMPLER_ID,
            root_id,
            epoch,
            with_replacement: true,
            fiber: spec.root.count(),
        };
        let m_u64 = m as u64;
        let threshold = evidence::edge_threshold(m_u64, spec.plan.edges());
        let edge_alpha =
            spec.plan.edges() / BigRational::from_integer(BigInt::from(m_u64 * (m_u64 - 1)));
        let pairs_total = m_u64 * (m_u64 - 1) / 2;
        let (eq_alpha, eq_tests, eq_threshold) = match spec.plan.equivalence() {
            None => (None, 0, None),
            Some(eq) => {
                let tests = pairs_total * eq.route.tests_per_pair();
                let alpha = &eq.delta / BigRational::from_integer(BigInt::from(tests));
                let t = alpha.clone().recip();
                (Some(alpha), tests, Some(t))
            }
        };
        let ledger = DecisionLedger {
            plan: spec.plan.clone(),
            m: m_u64,
            edge_alpha,
            edge_threshold: threshold.clone(),
            eq_alpha,
            eq_tests,
            epoch: epoch_id,
            stream,
        };
        let mut pairs = Vec::new();
        for i in 0..m {
            for j in (i + 1)..m {
                let eq = spec.plan.equivalence().map(|plan| match plan.route {
                    EquivalenceRoute::PivotalMass => EqProcesses::PivotalMass,
                    EquivalenceRoute::BoundedMean => EqProcesses::BoundedMean {
                        above: Box::new(equivalence_gate(
                            MeanNull::AtMost,
                            -plan.epsilon.clone(),
                            &plan.epsilon,
                        )),
                        below: Box::new(equivalence_gate(
                            MeanNull::AtLeast,
                            plan.epsilon.clone(),
                            &plan.epsilon,
                        )),
                    },
                });
                pairs.push(PairState {
                    i,
                    j,
                    a: 0,
                    b: 0,
                    n: 0,
                    settled: None,
                    eq,
                    eq_settled_at: None,
                });
            }
        }
        Controller {
            spec,
            m,
            root_id,
            epoch,
            threshold,
            eq_threshold,
            ledger,
            live: vec![true; m],
            pairs,
            cache: HashMap::new(),
            edges: Vec::new(),
            eliminations: Vec::new(),
        }
    }

    fn live_indices(&self) -> Vec<usize> {
        (0..self.m).filter(|&k| self.live[k]).collect()
    }

    fn pair_counts(&self) -> Vec<PairCounts> {
        self.pairs
            .iter()
            .map(|p| PairCounts {
                i: p.i,
                j: p.j,
                a: p.a,
                b: p.b,
                n: p.n,
            })
            .collect()
    }

    /// Fold stream world `index`: §17.2 order — every live candidate
    /// evaluates the world first (cache readout after first
    /// materialization), then every live pair's evidence updates, then
    /// settled edges eliminate.
    fn fold_world(&mut self, index: u64) {
        let viewer = self.spec.root.kernel().viewer();
        let world = self.spec.root.world_at(self.root_id, self.epoch, index);
        let entry = self
            .cache
            .entry(world)
            .or_insert_with(|| vec![None; self.m]);
        for (k, slot) in entry.iter_mut().enumerate() {
            if self.live[k] && slot.is_none() {
                *slot = Some(replay_viewer_success(
                    self.spec.position,
                    viewer,
                    &world,
                    self.spec.candidates.get(k),
                    self.spec.field,
                ));
            }
        }
        let outcomes = entry.clone();
        for p in &mut self.pairs {
            if !(self.live[p.i] && self.live[p.j]) {
                continue;
            }
            let u_i = outcomes[p.i].expect("a live candidate evaluated this world");
            let u_j = outcomes[p.j].expect("a live candidate evaluated this world");
            let y: i8 = match (u_i, u_j) {
                (true, false) => 1,
                (false, true) => -1,
                _ => 0,
            };
            p.n += 1;
            if y > 0 {
                p.a += 1;
            } else if y < 0 {
                p.b += 1;
            }
            // A nonpivotal world leaves the directed evidence unchanged
            // and cannot newly cross (§4).
            if y != 0 && p.settled.is_none() {
                let forward =
                    evidence::crossed(&evidence::pivotal_evidence(p.a, p.b), &self.threshold);
                let backward =
                    evidence::crossed(&evidence::pivotal_evidence(p.b, p.a), &self.threshold);
                assert!(
                    !(forward && backward),
                    "both directions of one edge cannot be settled at once"
                );
                if forward {
                    p.settled = Some(index);
                    self.edges.push(SettledEdge {
                        from: p.i,
                        to: p.j,
                        at: index,
                        a: p.a,
                        b: p.b,
                    });
                } else if backward {
                    p.settled = Some(index);
                    self.edges.push(SettledEdge {
                        from: p.j,
                        to: p.i,
                        at: index,
                        a: p.a,
                        b: p.b,
                    });
                }
            }
            if let (Some(eq), Some(t_eq)) = (&mut p.eq, &self.eq_threshold) {
                if p.eq_settled_at.is_none() {
                    let settled = match eq {
                        EqProcesses::BoundedMean { above, below } => {
                            let x = BigRational::from_integer(BigInt::from(y));
                            above.observe(&x);
                            below.observe(&x);
                            evidence::crossed(&above.evidence(), t_eq)
                                && evidence::crossed(&below.evidence(), t_eq)
                        }
                        EqProcesses::PivotalMass => {
                            let epsilon = &self
                                .spec
                                .plan
                                .equivalence()
                                .expect("an equivalence process implies a plan")
                                .epsilon;
                            let e_lower = evidence::lower_threshold_evidence(
                                p.a + p.b,
                                p.n - p.a - p.b,
                                epsilon,
                            );
                            evidence::crossed(&e_lower, t_eq)
                        }
                    };
                    if settled {
                        p.eq_settled_at = Some(index);
                    }
                }
            }
        }
        // §5.1 safe elimination, to a fixed point: remove any candidate a
        // LIVE candidate holds a settled edge into. The last survivor can
        // never be removed (its remover would have to be live).
        loop {
            let mut removed = false;
            'scan: for j in 0..self.m {
                if !self.live[j] {
                    continue;
                }
                for e in &self.edges {
                    if e.to == j && self.live[e.from] {
                        self.live[j] = false;
                        self.eliminations.push(Elimination {
                            candidate: j,
                            by: e.from,
                            at: index,
                        });
                        removed = true;
                        break 'scan;
                    }
                }
            }
            if !removed {
                break;
            }
        }
    }

    /// §11.3 — the exact switch comparison under the declared weights.
    fn should_escalate(&self, config: &EscalationConfig) -> bool {
        let n_rem = self.spec.root.count() - self.cache.len() as u128;
        let c_exact = BigInt::from(n_rem) * BigInt::from(config.cost_enumerate);
        let h_lb = self
            .pairs
            .iter()
            .filter(|p| self.live[p.i] && self.live[p.j] && p.settled.is_none())
            .map(|p| {
                evidence::h_plus_min(p.a, p.b, &self.threshold).min(evidence::h_minus_min(
                    p.a,
                    p.b,
                    &self.threshold,
                ))
            })
            .min();
        let Some(h_lb) = h_lb else {
            // No open live pair: settlement is imminent; enumeration
            // cannot be justified by remaining sampling work.
            return false;
        };
        let c_sample_lb = BigInt::from(h_lb) * BigInt::from(config.cost_sample);
        c_exact <= c_sample_lb
    }

    /// §11.5/§11.4 — the exact frozen-set endpoint: enumerate the complete
    /// fiber, reuse every cached sampled outcome, evaluate the remainder
    /// fresh (eliminated candidates included — the endpoint names every
    /// candidate). Every physical world counts exactly once (O24).
    fn escalate(self, switched_at: u64) -> SetEvaluation {
        let viewer = self.spec.root.kernel().viewer();
        let mut wins = vec![0u128; self.m];
        let mut reused_worlds = 0u64;
        let mut fresh_worlds = 0u64;
        let mut visited = 0u128;
        for world in self.spec.root.worlds() {
            let cached = self.cache.get(&world);
            if cached.is_some() {
                reused_worlds += 1;
            } else {
                fresh_worlds += 1;
            }
            for (k, win) in wins.iter_mut().enumerate() {
                let u = match cached.and_then(|entry| entry[k]) {
                    Some(u) => u,
                    None => replay_viewer_success(
                        self.spec.position,
                        viewer,
                        &world,
                        self.spec.candidates.get(k),
                        self.spec.field,
                    ),
                };
                if u {
                    *win += 1;
                }
            }
            visited += 1;
        }
        assert_eq!(
            visited,
            self.spec.root.count(),
            "enumeration visits the whole fiber exactly once"
        );
        let best = wins.iter().max().expect("m >= 2 candidates");
        let mut at_max = wins.iter().enumerate().filter(|(_, w)| *w == best);
        let winner = match (at_max.next(), at_max.next()) {
            (Some((k, _)), None) => Some(k),
            _ => None, // several best members: an exact tie, honest output
        };
        SetEvaluation {
            result: SetResult::ExactFrozenSet {
                wins,
                fiber: visited,
                winner,
                policy_ids: self.spec.candidates.ids(),
                ledger: self.ledger.clone(),
            },
            pair_counts: self.pair_counts(),
            edges: self.edges,
            eliminations: self.eliminations,
            consumed: switched_at,
            escalation: Some(EscalationReport {
                switched_at,
                reused_worlds,
                fresh_worlds,
            }),
        }
    }

    /// §8.5 — refinement vectors for every open live pair.
    fn refinements(&self) -> Vec<PairRefinement> {
        let ids = self.spec.candidates.ids();
        let (cost_sample, cost_enumerate) = match &self.spec.escalation {
            Some(config) => (config.cost_sample, config.cost_enumerate),
            None => (1, 1),
        };
        let n_rem = self.spec.root.count() - self.cache.len() as u128;
        self.pairs
            .iter()
            .filter(|p| self.live[p.i] && self.live[p.j] && p.settled.is_none())
            .map(|p| {
                let ratio = |num: i128, den: u64| {
                    (den > 0).then(|| BigRational::new(BigInt::from(num), BigInt::from(den)))
                };
                let pivots = p.a + p.b;
                let signed = i128::from(p.a) - i128::from(p.b);
                let q_hat = ratio(i128::from(pivots), p.n);
                let tau_hat = ratio(signed, pivots);
                let g_hat = ratio(signed, p.n);
                let e_plus = evidence::pivotal_evidence(p.a, p.b);
                let e_minus = evidence::pivotal_evidence(p.b, p.a);
                let h_plus = evidence::h_plus_min(p.a, p.b, &self.threshold);
                let h_minus = evidence::h_minus_min(p.a, p.b, &self.threshold);
                let raw = |h: u64| {
                    q_hat.as_ref().and_then(|q| {
                        q.is_positive()
                            .then(|| BigRational::from_integer(BigInt::from(h)) / q)
                    })
                };
                let n_hat_plus = raw(h_plus);
                let n_hat_minus = raw(h_minus);
                let c_sample_forecast = match (&n_hat_plus, &n_hat_minus) {
                    (Some(x), Some(y)) => Some(x.min(y).clone()),
                    (Some(x), None) => Some(x.clone()),
                    (None, Some(y)) => Some(y.clone()),
                    (None, None) => None,
                }
                .map(|n_hat| n_hat * BigRational::from_integer(BigInt::from(cost_sample)));
                PairRefinement {
                    i: p.i,
                    j: p.j,
                    policy_i: ids[p.i],
                    policy_j: ids[p.j],
                    n: p.n,
                    a: p.a,
                    b: p.b,
                    n0: p.n - p.a - p.b,
                    q_hat,
                    tau_hat,
                    g_hat,
                    r_debt_plus: evidence::evidence_debt(&self.threshold, &e_plus),
                    r_debt_minus: evidence::evidence_debt(&self.threshold, &e_minus),
                    e_plus,
                    e_minus,
                    threshold: self.threshold.clone(),
                    h_plus_min: h_plus,
                    h_minus_min: h_minus,
                    n_hat_plus,
                    n_hat_minus,
                    c_exact: BigRational::from_integer(
                        BigInt::from(n_rem) * BigInt::from(cost_enumerate),
                    ),
                    c_sample_forecast,
                }
            })
            .collect()
    }

    fn finish(self, result: SetResult, consumed: u64) -> SetEvaluation {
        SetEvaluation {
            pair_counts: self.pair_counts(),
            edges: self.edges,
            eliminations: self.eliminations,
            consumed,
            escalation: None,
            result,
        }
    }
}

// ---------------------------------------------------------------------------
// The public evaluators.
// ---------------------------------------------------------------------------

/// Evaluate the fixed candidate set adaptively: common indexed worlds,
/// all-pairs directed evidence, safe elimination, optional practical
/// equivalence, optional automatic exact escalation, and the world cap as
/// a resource limit producing `Unresolved`.
pub fn evaluate_set(spec: &SetSpec<'_>) -> SetEvaluation {
    let mut controller = Controller::new(spec);
    let mut index = 0u64;
    while index < spec.world_cap {
        let hi = (index + spec.batch).min(spec.world_cap);
        // Work is grouped in batches for throughput (§17.3), but evidence
        // folds strictly in stream order and every check runs at its
        // stream index, so batch size cannot change any outcome (V8).
        for i in index..hi {
            controller.fold_world(i);
            let live = controller.live_indices();
            if live.len() == 1 {
                let winner = live[0];
                let winner_id = spec.candidates.get(winner).policy_id();
                let ledger = controller.ledger.clone();
                return controller.finish(
                    SetResult::DeltaSettled {
                        winner,
                        winner_id,
                        settled_at: i,
                        ledger,
                    },
                    i + 1,
                );
            }
            if let Some(eq) = spec.plan.equivalence() {
                let all_equivalent = controller
                    .pairs
                    .iter()
                    .filter(|p| controller.live[p.i] && controller.live[p.j])
                    .all(|p| p.eq_settled_at.is_some());
                if all_equivalent {
                    let ledger = controller.ledger.clone();
                    let epsilon = eq.epsilon.clone();
                    return controller.finish(
                        SetResult::EpsilonEquivalent {
                            survivors: live,
                            epsilon,
                            settled_at: i,
                            ledger,
                        },
                        i + 1,
                    );
                }
            }
            if let Some(config) = &spec.escalation {
                if (i + 1) % config.check_every == 0 && controller.should_escalate(config) {
                    return controller.escalate(i + 1);
                }
            }
        }
        index = hi;
    }
    // The cap arrived first: a successful honest output (§1.5) with the
    // §8.5 refinement vectors persisted.
    let survivors = controller.live_indices();
    let refinements = controller.refinements();
    let ledger = controller.ledger.clone();
    let consumed = spec.world_cap;
    controller.finish(
        SetResult::Unresolved {
            survivors,
            consumed,
            refinements,
            ledger,
        },
        consumed,
    )
}

/// Force the exact switch at stream index `switch_at`: consume exactly
/// that sampled prefix (evidence, eliminations, and the outcome cache all
/// update normally; probabilistic termination is suppressed), then run the
/// exact endpoint reusing the cache. Whatever the switch index, the exact
/// result must equal a cold full enumeration — gate V9 / O24.
pub fn evaluate_set_with_switch(spec: &SetSpec<'_>, switch_at: u64) -> SetEvaluation {
    let mut controller = Controller::new(spec);
    for i in 0..switch_at {
        controller.fold_world(i);
    }
    controller.escalate(switch_at)
}

/// The cold exact frozen-set endpoint: full enumeration, no sampled
/// prefix. Exact results spend no sampling risk (§6.1).
pub fn exact_frozen_set(spec: &SetSpec<'_>) -> SetEvaluation {
    Controller::new(spec).escalate(0)
}
