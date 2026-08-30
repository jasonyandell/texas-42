//! `solver::root_interval` — root intervals and survivor sets: the
//! counted-belief Slice A (parent §56 acceptance contract).
//!
//! EXPLORATORY tier. **[L2 consuming CE machinery]** through the
//! sanctioned one-directional crossing. Implements Part I (§1–3) and the
//! sampled Part II route (§5–7) of the verbatim parent
//! `walt/math/counted_belief_sandwich_v0.1.md` (intake companion beside
//! it; adjudicated `walt/CENSUS-RULINGS.md` CBS-A1..A9). Naming per
//! CBS-A3: **root interval** and **survivor set** — never "sandwich."
//!
//! The objects: for each legal root action `a`, a lower and an upper
//! bound on the fixed-field best-response value
//! `Q_a = max_{ρ ∈ Π_a} Pr_β(u_ρ = 1)` (the pmake objective), and the
//! survivor arithmetic of parent Theorem 2.1: with bar `B = max_a L_a`,
//! every optimal action lies in `{a : U_a ≥ B}` on the event that every
//! input interval is valid, so exclusion is safe and a singleton
//! survivor is the optimum at the inputs' joint risk.
//!
//! The upper is the CBS-A2 route — Theorem 5.1, which is x:024 M1/M2
//! over the pmake objective: the empirical best-response count `S*_{a,t}`
//! ([`crate::solver::exposure::sampled_root_optimum`] — one
//! information-consistent policy shared across the declared prefix, no
//! strategy fusion) inverted through the CE one-mean engine
//! ([`grid_upper_endpoint`]) on the grid `G_N` with `N = |Φ(I)|`, prefix
//! minimum taken. No |Π_a| factor enters the risk.
//!
//! The lower is the §6 route: ONE frozen lawful policy
//! ([`crate::solver::policy::FrozenPolicy`]), replayed on an evaluation
//! stream ([`replay_viewer_success`]), inverted through the mirror
//! endpoint ([`grid_lower_endpoint`]), prefix maximum taken. The §6 lock
//! is mechanical: a policy whose provenance is
//! [`PolicyProvenance::Discovered`] on epoch `e` REFUSES evaluation on
//! epoch `e` — a same-stream selected argmax is not a lower witness.
//! (A [`PolicyProvenance::Fixed`] policy — declared a priori, e.g. the
//! pinned level-1 continuation, whose internal discovery stream is
//! seed-schedule-separated by the freeze tuple itself, §12.4 — carries
//! no such constraint.)
//!
//! Result typing (parent §2, extending the CE six-way ladder): a
//! singleton survivor is a [`RootDecision::DeltaRootWinner`]; a proper
//! subset is a [`RootDecision::DeltaRootSet`]; the full legal set — or
//! the empty set, which certifies a realized coverage failure among the
//! δ inputs and is never promoted to anything — is a
//! [`RootDecision::UnresolvedRootSet`]. A [`HeuristicFallback`] may
//! choose within a surviving set and is NEVER serialized as a settled
//! winner (CBS-A3).
//!
//! Nothing in this module touches the live default player (CBS-A9,
//! CE-A7/§20.16).

use std::fmt;

use num_rational::BigRational;
use num_traits::One;

use crate::rules::rules::legal_plays;
use crate::rules::{Domino, DominoSet};
use crate::solver::adaptive::{
    replay_viewer_success, root_identity, CanonicalRoot, PublicRecord, RootPosition, SlicePolicy,
};
use crate::solver::evidence::ScopedDelta;
use crate::solver::exposure::sampled_root_optimum;
use crate::solver::field::{FieldId, FieldModel};
use crate::solver::policy::{FrozenPolicy, PolicyId};
use crate::solver::upper_cs::{
    assert_screen_risk_allocation, grid_upper_endpoint, nested_prefix_uppers,
    POLICY_CLASS_INFO_CONSISTENT,
};

// ---------------------------------------------------------------------------
// The mirror endpoint (§6): the exact one-mean lower confidence endpoint.
// ---------------------------------------------------------------------------

/// The exact one-mean lower confidence endpoint `L_{δ,N}(s, n)`, by the
/// complement construction the parent's verifier uses:
/// `L_{δ,N}(s, n) = 1 − U_{δ,N}(n − s, n)`. Complementing the Bernoulli
/// swaps successes with failures and maps `p ↦ 1 − p`, under which the
/// grid `G_N` maps to itself — so P1's coverage mirrors exactly:
/// `Pr_p(∃n: p < L_{δ,N}(S_n, n)) ≤ δ` for every `p ∈ G_N`.
pub fn grid_lower_endpoint(s: u64, n: u64, grid: u128, delta: &BigRational) -> BigRational {
    assert!(s <= n, "successes never exceed observations");
    BigRational::one() - grid_upper_endpoint(n - s, n, grid, delta)
}

/// The §6 nested prefix lower bounds over one success-count path: entry
/// `t − 1` is `max_{t' ≤ t} L_{δ,N}(counts[t'−1], t')`. The running
/// maximum makes the reported witness nondecreasing without changing its
/// coverage (the mirror of the §1.4 intersection argument).
pub fn nested_prefix_lowers(counts: &[u64], grid: u128, delta: &BigRational) -> Vec<BigRational> {
    assert!(
        !counts.is_empty(),
        "a declared prefix holds at least one world"
    );
    let mut out: Vec<BigRational> = Vec::with_capacity(counts.len());
    for (i, &s) in counts.iter().enumerate() {
        let t = u64::try_from(i + 1).expect("a declared prefix fits u64");
        let l = grid_lower_endpoint(s, t, grid, delta);
        let running = match out.last() {
            Some(previous) if previous > &l => previous.clone(),
            _ => l,
        };
        out.push(running);
    }
    out
}

// ---------------------------------------------------------------------------
// The upper carrier — parent §42's DeltaEmpiricalMax over the pmake
// objective (CBS-A2).
// ---------------------------------------------------------------------------

/// The δ-valid empirical-max upper bound on `Q_a`: the typed carrier over
/// a declared prefix path of pmake empirical-optimum counts. The stored
/// authority is the count path; the reported bound and the per-prefix
/// bounds are derived views. The counts' validity — that entry `t − 1`
/// equals (or pathwise dominates, Corollary 5.2) `S*_{a,t}` — is the
/// producer's obligation ([`pmake_empirical_max_upper`]); the constructor
/// asserts the shape every lawful count path has.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootActionUpper {
    /// The fixed root action `a`.
    pub action: Domino,
    /// The one field the optimization runs under.
    pub field: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The stream epoch of the declared prefix.
    pub epoch: u64,
    /// `N = |Φ(I)|` from the exact fiber counter — grid validity.
    pub grid: u128,
    /// The fixed policy class of the evidence epoch.
    pub policy_class: &'static str,
    delta: ScopedDelta,
    counts: Vec<u64>,
}

impl RootActionUpper {
    #[allow(clippy::too_many_arguments)]
    pub fn from_prefix_counts(
        action: Domino,
        field: FieldId,
        root_id: u64,
        epoch: u64,
        grid: u128,
        policy_class: &'static str,
        delta: ScopedDelta,
        counts: Vec<u64>,
    ) -> RootActionUpper {
        assert!(grid >= 1, "the grid G_N needs N >= 1");
        assert_count_path(&counts);
        RootActionUpper {
            action,
            field,
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

    /// The stored count path: entry `t − 1` is the declared pmake optimum
    /// over the stream prefix `0..t`.
    pub fn counts(&self) -> &[u64] {
        &self.counts
    }

    /// The declared risk entry — a δ that never travels without its scope.
    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }

    /// The §1.4-shape nested per-prefix bounds — a derived view.
    pub fn prefix_uppers(&self) -> Vec<BigRational> {
        nested_prefix_uppers(&self.counts, self.grid, self.delta.delta())
    }

    /// The reported bound at the declared prefix:
    /// `min_{t ≤ n} U_{δ,N}(S*_t, t)`.
    pub fn upper(&self) -> BigRational {
        self.prefix_uppers()
            .pop()
            .expect("a declared prefix holds at least one world")
    }

    pub fn method(&self) -> &'static str {
        "delta-empirical-max"
    }
}

impl fmt::Display for RootActionUpper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RootActionUpper{{method={};upper={};action={};field={};root={:#018x};epoch={};\
             prefix={};grid={};policy_class={};{}}}",
            self.method(),
            self.upper(),
            self.action,
            self.field,
            self.root_id,
            self.epoch,
            self.prefix(),
            self.grid,
            self.policy_class,
            self.delta
        )
    }
}

// ---------------------------------------------------------------------------
// The lower carrier — parent §42's DeltaPolicy (§6).
// ---------------------------------------------------------------------------

/// How the lower-witness policy came to be the one evaluated — the §6
/// discovery/evaluation separation, carried on the result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyProvenance {
    /// Declared a priori: no on-stream selection happened. (A frozen
    /// tuple's INTERNAL discovery stream is separated by the tuple's own
    /// seed-schedule identity, §12.4 — that is not on-stream selection.)
    Fixed,
    /// Selected among candidates by realized counts on the named
    /// discovery epoch ([`discover_lower_policy`]). Evaluation on that
    /// same epoch is refused: a same-stream selected argmax is not a
    /// lower witness (§6).
    Discovered {
        /// The discovery stream epoch the selection read.
        epoch: u64,
    },
}

impl fmt::Display for PolicyProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolicyProvenance::Fixed => write!(f, "fixed"),
            PolicyProvenance::Discovered { epoch } => write!(f, "discovered@{epoch}"),
        }
    }
}

/// The δ-valid frozen-policy lower witness on `Q_a`: one lawful
/// information-consistent policy's realized success path on an evaluation
/// stream, inverted through the mirror endpoint. Valid for `V(ρ) ≤ Q_a`
/// at the declared risk — a witness, never a bound on omitted policies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootActionLower {
    /// The root action the witness policy plays at the root.
    pub action: Domino,
    /// The frozen identity of the witness policy.
    pub policy: PolicyId,
    /// The declared field the replay ran under.
    pub field: FieldId,
    /// The evidence-stream root identity of the position.
    pub root_id: u64,
    /// The EVALUATION stream epoch.
    pub epoch: u64,
    /// `N = |Φ(I)|` — the witness's true mean lies on `G_N`.
    pub grid: u128,
    /// The §6 separation record.
    pub provenance: PolicyProvenance,
    delta: ScopedDelta,
    successes: Vec<u64>,
}

impl RootActionLower {
    /// The typed carrier over a declared cumulative-success path. The §6
    /// lock is enforced here: a policy discovered on epoch `e` refuses an
    /// evaluation path declared at epoch `e`.
    #[allow(clippy::too_many_arguments)]
    pub fn from_prefix_successes(
        action: Domino,
        policy: PolicyId,
        field: FieldId,
        root_id: u64,
        epoch: u64,
        grid: u128,
        provenance: PolicyProvenance,
        delta: ScopedDelta,
        successes: Vec<u64>,
    ) -> RootActionLower {
        assert!(grid >= 1, "the grid G_N needs N >= 1");
        assert_count_path(&successes);
        if let PolicyProvenance::Discovered {
            epoch: discovery_epoch,
        } = provenance
        {
            assert_ne!(
                discovery_epoch, epoch,
                "a same-stream selected argmax is not a lower witness (§6): \
                 the discovery epoch and the evaluation epoch must differ"
            );
        }
        RootActionLower {
            action,
            policy,
            field,
            root_id,
            epoch,
            grid,
            provenance,
            delta,
            successes,
        }
    }

    /// The declared evaluation prefix length `n`.
    pub fn prefix(&self) -> u64 {
        u64::try_from(self.successes.len()).expect("a declared prefix fits u64")
    }

    /// The stored success path: entry `t − 1` is the policy's realized
    /// success count on the evaluation prefix `0..t`.
    pub fn successes(&self) -> &[u64] {
        &self.successes
    }

    pub fn delta(&self) -> &ScopedDelta {
        &self.delta
    }

    /// The §6 nested per-prefix witnesses — a derived view.
    pub fn prefix_lowers(&self) -> Vec<BigRational> {
        nested_prefix_lowers(&self.successes, self.grid, self.delta.delta())
    }

    /// The reported witness at the declared prefix:
    /// `max_{t ≤ n} L_{δ,N}(T_t, t)`.
    pub fn lower(&self) -> BigRational {
        self.prefix_lowers()
            .pop()
            .expect("a declared prefix holds at least one world")
    }

    pub fn method(&self) -> &'static str {
        "delta-frozen-policy"
    }
}

impl fmt::Display for RootActionLower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RootActionLower{{method={};lower={};action={};policy={};field={};root={:#018x};\
             epoch={};prefix={};grid={};provenance={};{}}}",
            self.method(),
            self.lower(),
            self.action,
            self.policy,
            self.field,
            self.root_id,
            self.epoch,
            self.prefix(),
            self.grid,
            self.provenance,
            self.delta
        )
    }
}

/// The shape every lawful cumulative count path has: `s_t ≤ t`,
/// nondecreasing, unit steps. (This rejects malformed paths; it cannot —
/// and does not claim to — verify the producer's semantic obligation.)
fn assert_count_path(counts: &[u64]) {
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
                "a prefix count path is nondecreasing with unit steps"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The interval and the survivor arithmetic (parent §2, Theorem 2.1).
// ---------------------------------------------------------------------------

/// One root action's interval: a lower witness and an upper bound of the
/// SAME action, field, root, and grid, under DISTINCT risk scopes. The
/// two realized values may cross — that is the declared δ event showing
/// itself, and the type reports it honestly rather than forbidding it.
#[derive(Clone, Debug)]
pub struct RootActionInterval {
    pub lower: RootActionLower,
    pub upper: RootActionUpper,
}

impl RootActionInterval {
    pub fn new(lower: RootActionLower, upper: RootActionUpper) -> RootActionInterval {
        assert_eq!(lower.action, upper.action, "one root action per interval");
        assert_eq!(lower.field, upper.field, "one field per interval");
        assert_eq!(lower.root_id, upper.root_id, "one root per interval");
        assert_eq!(lower.grid, upper.grid, "one grid per interval");
        assert_ne!(
            lower.delta().scope(),
            upper.delta().scope(),
            "lower and upper claims carry distinct risk entries"
        );
        RootActionInterval { lower, upper }
    }

    pub fn action(&self) -> Domino {
        self.upper.action
    }

    pub fn lower_value(&self) -> BigRational {
        self.lower.lower()
    }

    pub fn upper_value(&self) -> BigRational {
        self.upper.upper()
    }
}

impl fmt::Display for RootActionInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RootActionInterval{{action={};lower={};upper={}}}",
            self.action(),
            self.lower_value(),
            self.upper_value()
        )
    }
}

/// The survivor arithmetic of parent Theorem 2.1 over one interval set.
#[derive(Clone, Debug)]
pub struct RootSurvivors {
    /// The bar `B = max_a L_a`.
    pub bar: BigRational,
    /// `{a : U_a ≥ B}` — every optimal action lies here on the validity
    /// event. May be EMPTY: that certifies a realized coverage failure
    /// among the δ inputs (some interval's own bounds crossed the bar
    /// arithmetic), and it is typed as unresolved, never as a winner.
    pub survivors: DominoSet,
}

/// Compute the bar and the survivor set (Theorem 2.1). Every interval
/// must name the same root and field; actions must be distinct.
pub fn survivors(intervals: &[RootActionInterval]) -> RootSurvivors {
    assert!(!intervals.is_empty(), "a survivor set needs an interval");
    let first = &intervals[0];
    let mut seen = DominoSet::EMPTY;
    for interval in intervals {
        assert_eq!(
            interval.upper.root_id, first.upper.root_id,
            "one root per survivor computation"
        );
        assert_eq!(
            interval.upper.field, first.upper.field,
            "one field per survivor computation"
        );
        assert!(
            seen.insert(interval.action()),
            "one interval per root action"
        );
    }
    let bar = intervals
        .iter()
        .map(RootActionInterval::lower_value)
        .max()
        .expect("a nonempty interval set");
    let mut out = DominoSet::EMPTY;
    for interval in intervals {
        if interval.upper_value() >= bar {
            out.insert(interval.action());
        }
    }
    RootSurvivors {
        bar,
        survivors: out,
    }
}

/// The typed root decision (parent §2's result ladder, the δ rungs this
/// slice produces; exact rungs arrive with the exact backend).
#[derive(Clone, Debug)]
pub enum RootDecision {
    /// A singleton survivor: the exact optimum on the joint validity
    /// event of every input interval.
    DeltaRootWinner { action: Domino, bar: BigRational },
    /// A proper nonempty subset survived: every excluded action is
    /// safely excluded on the validity event.
    DeltaRootSet {
        survivors: DominoSet,
        bar: BigRational,
    },
    /// Nothing was excluded (the full legal set survived), or the
    /// survivor set came back EMPTY (a realized coverage failure among
    /// the δ inputs). Caps and budget exhaustion land here — never on a
    /// forced winner.
    UnresolvedRootSet {
        survivors: DominoSet,
        bar: BigRational,
    },
}

impl RootDecision {
    /// The surviving actions this decision carries.
    pub fn surviving(&self) -> DominoSet {
        match self {
            RootDecision::DeltaRootWinner { action, .. } => {
                let mut set = DominoSet::EMPTY;
                set.insert(*action);
                set
            }
            RootDecision::DeltaRootSet { survivors, .. }
            | RootDecision::UnresolvedRootSet { survivors, .. } => *survivors,
        }
    }
}

impl fmt::Display for RootDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootDecision::DeltaRootWinner { action, bar } => {
                write!(f, "DeltaRootWinner{{action={action};bar={bar}}}")
            }
            RootDecision::DeltaRootSet { survivors, bar } => {
                write!(f, "DeltaRootSet{{survivors={};bar={bar}}}", survivors.len())
            }
            RootDecision::UnresolvedRootSet { survivors, bar } => {
                write!(
                    f,
                    "UnresolvedRootSet{{survivors={};bar={bar}}}",
                    survivors.len()
                )
            }
        }
    }
}

/// Type the decision over a complete interval cover of the legal root
/// actions. Coverage is asserted: an action without an interval could
/// never be excluded, so a partial cover types nothing.
pub fn decide(intervals: &[RootActionInterval], legal: DominoSet) -> RootDecision {
    let mut covered = DominoSet::EMPTY;
    for interval in intervals {
        assert!(
            legal.contains(interval.action()),
            "every interval names a legal root action"
        );
        assert!(covered.insert(interval.action()), "one interval per action");
    }
    assert_eq!(
        covered, legal,
        "the decision needs one interval per legal root action"
    );
    let s = survivors(intervals);
    let count = s.survivors.len();
    if count == 1 {
        RootDecision::DeltaRootWinner {
            action: s.survivors.iter().next().expect("a singleton"),
            bar: s.bar,
        }
    } else if count == 0 || s.survivors == legal {
        RootDecision::UnresolvedRootSet {
            survivors: s.survivors,
            bar: s.bar,
        }
    } else {
        RootDecision::DeltaRootSet {
            survivors: s.survivors,
            bar: s.bar,
        }
    }
}

/// A named heuristic choice WITHIN a surviving set — the parent §2
/// fallback rung. It is a play selection, never a settled result, and no
/// route exists from this type into [`RootDecision`]. When the survivor
/// set is empty (a realized coverage failure), the fallback falls back to
/// the full legal set, honestly labeled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeuristicFallback {
    pub method: &'static str,
    pub choice: Domino,
}

impl HeuristicFallback {
    /// The lowest-tile-index chooser (the stack's ascending convention).
    pub fn lowest_tile(decision: &RootDecision, legal: DominoSet) -> HeuristicFallback {
        let surviving = decision.surviving();
        let pool = if surviving.is_empty() {
            legal
        } else {
            surviving
        };
        HeuristicFallback {
            method: "fallback-lowest-tile-index",
            choice: pool.iter().next().expect("a nonempty legal set"),
        }
    }
}

impl fmt::Display for HeuristicFallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HeuristicFallback{{method={};choice={}}}",
            self.method, self.choice
        )
    }
}

// ---------------------------------------------------------------------------
// Producers.
// ---------------------------------------------------------------------------

/// Produce the δ-valid empirical-max upper for one root action over the
/// declared stream prefix `0..prefix` at `epoch` (CBS-A2): per prefix
/// length `t`, the pmake empirical-optimum count `S*_{a,t}` from
/// [`sampled_root_optimum`] (the value walk on the declared multiset —
/// one shared information-consistent policy across the sample), inverted
/// through [`grid_upper_endpoint`] on `G_N` with `N = |Φ(I)|`, prefix
/// minimum taken. The empirical maximizer may change with `t`; Theorem
/// 5.1 compares against one fixed true maximizer, so no selection defect
/// arises.
pub fn pmake_empirical_max_upper(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
    field: &FieldModel,
    epoch: u64,
    prefix: u64,
    delta: ScopedDelta,
) -> RootActionUpper {
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let counts: Vec<u64> = (1..=prefix)
        .map(|t| sampled_root_optimum(root, position, action, field, epoch, t))
        .collect();
    RootActionUpper::from_prefix_counts(
        action,
        field.field_id(),
        root_identity(root, position),
        epoch,
        root.count(),
        POLICY_CLASS_INFO_CONSISTENT,
        delta,
        counts,
    )
}

/// The root action a deterministic focal policy plays at this root's one
/// focal information state (empty post-root history).
pub fn policy_root_action(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy: &dyn SlicePolicy,
) -> Domino {
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: position,
        history: &[],
    };
    let hand = root.kernel().viewer_hand();
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, hand, led);
    let tile = policy.choose(position.decl, hand, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal root action");
    tile
}

/// Evaluate one frozen policy on the evaluation stream and produce the
/// δ-valid lower witness (§6). The witness's root action is the policy's
/// own root choice; the replay runs every non-focal seat under the
/// declared field. The §6 separation is enforced by the carrier: a
/// [`PolicyProvenance::Discovered`] policy refuses its discovery epoch.
#[allow(clippy::too_many_arguments)]
pub fn frozen_policy_lower(
    root: &CanonicalRoot,
    position: &RootPosition,
    policy: &FrozenPolicy,
    field: &FieldModel,
    provenance: PolicyProvenance,
    epoch: u64,
    prefix: u64,
    delta: ScopedDelta,
) -> RootActionLower {
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let action = policy_root_action(root, position, policy);
    let root_id = root_identity(root, position);
    let viewer = root.kernel().viewer();
    let mut successes: Vec<u64> = Vec::with_capacity(usize::try_from(prefix).expect("fits"));
    let mut running = 0u64;
    for i in 0..prefix {
        let world = root.world_at(root_id, epoch, i);
        if replay_viewer_success(position, viewer, &world, policy, field) {
            running += 1;
        }
        successes.push(running);
    }
    RootActionLower::from_prefix_successes(
        action,
        policy.policy_id(),
        field.field_id(),
        root_id,
        epoch,
        root.count(),
        provenance,
        delta,
        successes,
    )
}

/// Select the strongest candidate by realized success count on the
/// DISCOVERY stream prefix `0..prefix` at `epoch` (§6's discovery half).
/// Returns the winning index; ties break toward the earlier candidate.
/// The caller carries `PolicyProvenance::Discovered { epoch }` into the
/// evaluation, where the carrier refuses that same epoch.
pub fn discover_lower_policy(
    root: &CanonicalRoot,
    position: &RootPosition,
    candidates: &[&FrozenPolicy],
    field: &FieldModel,
    epoch: u64,
    prefix: u64,
) -> usize {
    assert!(!candidates.is_empty(), "discovery needs a candidate");
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let root_id = root_identity(root, position);
    let viewer = root.kernel().viewer();
    let mut best = 0usize;
    let mut best_count = 0u64;
    for (index, policy) in candidates.iter().enumerate() {
        let mut count = 0u64;
        for i in 0..prefix {
            let world = root.world_at(root_id, epoch, i);
            if replay_viewer_success(position, viewer, &world, *policy, field) {
                count += 1;
            }
        }
        if index == 0 || count > best_count {
            best = index;
            best_count = count;
        }
    }
    best
}

/// The risk arithmetic over one interval set (§1.8 shape, no new ledger):
/// every interval contributes its lower and upper risk entries; distinct
/// scopes are asserted pairwise and the exact rational sum must fit the
/// declared decision budget. Returns the sum.
pub fn assert_root_risk_allocation(
    budget: &ScopedDelta,
    intervals: &[RootActionInterval],
) -> BigRational {
    let entries: Vec<&ScopedDelta> = intervals
        .iter()
        .flat_map(|interval| [interval.lower.delta(), interval.upper.delta()])
        .collect();
    assert_screen_risk_allocation(budget, &entries)
}
