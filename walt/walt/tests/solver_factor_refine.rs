//! Gates for the counted-belief Slice G [L2 thread]: the §50 integrated
//! refinement controller. Gate 1 is the C→G capstone: the §36
//! EscalateExact endpoint — the full-action-set factorized recursion —
//! agrees extensionally with the bundled exact authority
//! (`exposure::exact_root_value`) at every gated root and action, and
//! the containment chain holds (fixed-policy mass ≤ grammar mass ≤
//! response mass). Gate 2 is the §37 soundness invariant over a full
//! controller run: lowers only rise, uppers only fall, the bar is
//! monotone, exclusions are permanent, every exact bound in the final
//! record reproduces under independent recomputation, and the result
//! typing is faithful to the surviving set. Gate 3 is §34 steering:
//! the consequence-census item is refused as presently useless at every
//! bar, items on excluded actions never run, refused items charge
//! nothing, and a controller run is a pure function of its inputs
//! (bytewise-identical records across repeated runs). Gate 4 is §36
//! step 12 honesty: budget starvation returns the surviving set with
//! the NAMED fallback rule and never a settled claim, the work charged
//! never exceeds the budget, and the δ ledger fits the declared scope
//! (zero when the sampled tier is off).
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! Part VIII (§32–37) and §50 (Slice G), adopted by rulings CBS-A6 and
//! CBS-A9 (`walt/CENSUS-RULINGS.md`); design register
//! `walt/FACTOR-BELIEF.md`.
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind (the
//! Slice A–F declared field). Frozen `verify_player` receipt roots: the
//! ten gated roots of the Slice F epoch. Sampled tier, where a gate
//! turns it on: prefix 16, δ = 1/20 per endpoint (the Slice A
//! declaration), upper epoch 0, evaluation epoch 1.

mod common;
#[path = "common/fixture.rs"]
mod fixture;

use std::collections::HashMap;
use std::sync::LazyLock;

use common::receipt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::exposure::exact_root_value;
use walt::solver::factor_belief::{
    grammar_success_mass, response_success_mass, viewer_success_mass, ExactCoverOracle,
    FactorBelief, RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{CountPreservation, PolicyGrammar};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::refine::{
    refine_root, LowerBound, ProofClass, RefineConfig, RefineOutcome, RefineResult, RefusalReason,
    TraceEvent, UpperBound, WorkItem,
};

/// The gated roots of the Slice F epoch.
const GATED_ROOTS: [(usize, usize); 10] = [
    (12, 6),
    (10, 6),
    (5, 6),
    (4, 6),
    (8, 5),
    (3, 5),
    (3, 4),
    (4, 4),
    (8, 4),
    (12, 4),
];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn level0_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(num_bigint::BigInt::from(n), num_bigint::BigInt::from(d))
}

/// The exact-only ample configuration: sampled tier off, budget far
/// above every forecast in the gated corpus.
fn ample_exact() -> RefineConfig {
    RefineConfig {
        budget: u64::MAX / 2,
        prefix: 0,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    }
}

/// The two-tier ample configuration: the Slice A sampled declaration on
/// top of the exact ladder.
fn ample_two_tier() -> RefineConfig {
    RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    }
}

// ---------------------------------------------------------------------------
// The recompute-once fixture (BRIEF-CI1): the expensive oracle values
// every gate reads, computed once per test-binary process — the three
// exact masses of the independent recursions per (gated root, action)
// and the controller record per (ample configuration, gated root). A
// derived view of the declared epoch, immutable after construction.
// ---------------------------------------------------------------------------

/// Per legal action of one gated root: `(action, fixed lowest-first
/// mass, grammar mass, response mass)`.
type RootMasses = Vec<(Domino, u128, u128, u128)>;

struct Fixture {
    masses: HashMap<(usize, usize), RootMasses>,
    /// Keyed by the configuration's `prefix` — the one field that differs
    /// between the two ample configurations; the stored configuration is
    /// re-asserted on every read.
    outcomes: HashMap<(u64, usize, usize), (RefineConfig, RefineOutcome)>,
}

enum Job {
    Masses(usize, usize),
    Outcome(RefineConfig, usize, usize),
}

enum Value {
    Masses(RootMasses),
    Outcome(Box<RefineOutcome>),
}

fn masses_at(r: &Receipt, hand_id: usize, trick_no: usize) -> RootMasses {
    let oracle = SupportOracle;
    let (root, position) = root_at(r, hand_id, trick_no);
    let field = FieldModel::new(level0_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let low = walt::solver::adaptive::FixedPreference::lowest_first("focal:lowest-first");
    let high = walt::solver::adaptive::FixedPreference::highest_first("focal:highest-first");
    let count = CountPreservation::new();
    let grammar = PolicyGrammar::new(vec![&low, &high, &count]);
    let mut out = Vec::new();
    for action in root_actions(&root, &position).iter() {
        let child = belief.focal_play(action);
        let mut vs = RecursionStats::default();
        let fixed = viewer_success_mass(&oracle, &child, &low, &field, &mut vs);
        let mut gs = ResponseStats::default();
        let gmass = grammar_success_mass(&oracle, &child, &grammar, &field, &mut gs);
        let mut rs = ResponseStats::default();
        let response = response_success_mass(&oracle, &child, &field, &mut rs);
        out.push((action, fixed, gmass, response));
    }
    out
}

fn build_fixture() -> Fixture {
    let r = receipt();
    // Heaviest first: the two-tier controller runs, then the exact-only
    // runs, then the per-action recursions.
    let mut jobs: Vec<Job> = Vec::new();
    for cfg in [ample_two_tier(), ample_exact()] {
        for (hand_id, trick_no) in GATED_ROOTS {
            jobs.push(Job::Outcome(cfg.clone(), hand_id, trick_no));
        }
    }
    for (hand_id, trick_no) in GATED_ROOTS {
        jobs.push(Job::Masses(hand_id, trick_no));
    }
    let values = fixture::compute_all(&jobs, |job| match job {
        Job::Masses(hand_id, trick_no) => Value::Masses(masses_at(&r, *hand_id, *trick_no)),
        Job::Outcome(cfg, hand_id, trick_no) => {
            let (root, position) = root_at(&r, *hand_id, *trick_no);
            let spec = level0_spec();
            Value::Outcome(Box::new(refine_root(
                &root,
                &position,
                &spec,
                &SupportOracle,
                cfg,
            )))
        }
    });
    let mut fixture = Fixture {
        masses: HashMap::new(),
        outcomes: HashMap::new(),
    };
    for (job, value) in jobs.into_iter().zip(values) {
        match (job, value) {
            (Job::Masses(hand_id, trick_no), Value::Masses(m)) => {
                fixture.masses.insert((hand_id, trick_no), m);
            }
            (Job::Outcome(cfg, hand_id, trick_no), Value::Outcome(o)) => {
                fixture
                    .outcomes
                    .insert((cfg.prefix, hand_id, trick_no), (cfg, *o));
            }
            _ => unreachable!("a job's value is of the job's kind"),
        }
    }
    fixture
}

static FIXTURE: LazyLock<Fixture> = LazyLock::new(build_fixture);

/// `(fixed lowest-first, grammar, response)` exact masses at one gated
/// root action, from the independent recursions.
fn masses(hand_id: usize, trick_no: usize, action: Domino) -> (u128, u128, u128) {
    FIXTURE
        .masses
        .get(&(hand_id, trick_no))
        .unwrap_or_else(|| panic!("the fixture holds no masses at h{hand_id}-t{trick_no}"))
        .iter()
        .find(|(a, _, _, _)| *a == action)
        .map(|(_, fixed, gmass, response)| (*fixed, *gmass, *response))
        .unwrap_or_else(|| panic!("the fixture holds no masses at h{hand_id}-t{trick_no} {action}"))
}

/// The controller record at one ample configuration and gated root.
fn refined(cfg: &RefineConfig, hand_id: usize, trick_no: usize) -> &'static RefineOutcome {
    let (stored, outcome) = FIXTURE
        .outcomes
        .get(&(cfg.prefix, hand_id, trick_no))
        .unwrap_or_else(|| {
            panic!(
                "the fixture holds no controller run at h{hand_id}-t{trick_no} prefix={}",
                cfg.prefix
            )
        });
    assert_eq!(
        format!("{stored:?}"),
        format!("{cfg:?}"),
        "the fixture's run is under this configuration"
    );
    outcome
}

// ---------------------------------------------------------------------------
// Gate 1 — the C→G capstone: escalation parity with the bundled exact
// authority, and the containment chain.
// ---------------------------------------------------------------------------

#[test]
fn escalation_matches_the_bundled_exact_authority() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no) in GATED_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field_factor = FieldModel::new(level0_spec());
        let field_bundled = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field_factor);
        let z = oracle.mass(&belief);

        for action in root_actions(&root, &position).iter() {
            let child = belief.focal_play(action);
            assert_eq!(oracle.mass(&child), z, "a focal play changes no factor");

            let (fixed, gmass, response) = masses(hand_id, trick_no, action);

            // Containment: a policy is a one-policy grammar, a grammar
            // is a restriction of the full action set.
            assert!(
                fixed <= gmass && gmass <= response,
                "containment fixed ≤ grammar ≤ response (h{hand_id}-t{trick_no} {action})"
            );

            // Parity with the bundled exact authority.
            let exact = exact_root_value(&root, &position, action, &field_bundled);
            let factorized = BigRational::new(
                num_bigint::BigInt::from(response),
                num_bigint::BigInt::from(z),
            );
            assert_eq!(
                factorized,
                exact.value(),
                "the factorized full-action-set response equals the bundled \
                 exact authority (h{hand_id}-t{trick_no} {action})"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 2 — the §37 soundness invariant over full controller runs.
// ---------------------------------------------------------------------------

#[test]
fn the_controller_narrows_monotonically_and_soundly() {
    let r = receipt();
    let oracle = SupportOracle;
    for cfg in [ample_exact(), ample_two_tier()] {
        for (hand_id, trick_no) in GATED_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let outcome = refined(&cfg, hand_id, trick_no);
            let label = format!("h{hand_id}-t{trick_no} prefix={}", cfg.prefix);

            // The trace walk: monotone bounds per Ran event, monotone
            // bar, shrinking survivor sets, permanent exclusions.
            let mut last_bar = q(0, 1);
            let mut last_survivors: Option<DominoSet> = None;
            let mut excluded_seen = DominoSet::EMPTY;
            for event in &outcome.trace {
                match event {
                    TraceEvent::Ran(e) => {
                        assert!(e.lower_after >= e.lower_before, "lowers rise ({label})");
                        assert!(e.upper_after <= e.upper_before, "uppers fall ({label})");
                        assert!(e.bar_after >= last_bar, "the bar is monotone ({label})");
                        last_bar = e.bar_after.clone();
                        if let Some(prev) = &last_survivors {
                            assert!(
                                e.survivors_after.is_subset_of(*prev),
                                "survivor sets shrink ({label})"
                            );
                        }
                        last_survivors = Some(e.survivors_after);
                        assert!(
                            e.survivors_after.intersection(excluded_seen).is_empty(),
                            "an excluded action never returns ({label})"
                        );
                    }
                    TraceEvent::Excluded { action, .. } => {
                        excluded_seen.insert(*action);
                    }
                    TraceEvent::Refused { .. } => {}
                }
            }

            // The final record: exclusions and survivors partition the
            // legal set faithfully around the bar.
            let legal = root_actions(&root, &position);
            assert_eq!(
                outcome.survivors.union(outcome.excluded),
                legal,
                "survivors and exclusions partition the legal set ({label})"
            );
            assert!(
                outcome.survivors.intersection(outcome.excluded).is_empty(),
                "no action is both ({label})"
            );
            for interval in &outcome.intervals {
                assert!(
                    interval.lower_value() <= interval.upper_value(),
                    "an interval stays an interval ({label})"
                );
                if outcome.excluded.contains(interval.action) {
                    assert!(
                        interval.upper_value() < outcome.bar,
                        "an excluded action sits below the bar ({label})"
                    );
                } else {
                    assert!(
                        interval.upper_value() >= outcome.bar,
                        "a survivor covers the bar ({label})"
                    );
                }
            }

            // Independent recomputation of every exact bound.
            let field_check = FieldModel::new(level0_spec());
            let belief = FactorBelief::uniform_root(&root, &position, &field_check);
            let z = oracle.mass(&belief);
            for interval in &outcome.intervals {
                assert_eq!(interval.z, z, "the shared root mass ({label})");
                let (fixed, gmass, response) = masses(hand_id, trick_no, interval.action);
                match &interval.lower {
                    LowerBound::ExactPolicy { mass, .. } => {
                        let m = fixed;
                        assert_eq!(*mass, m, "the fixed-policy mass reproduces ({label})");
                    }
                    LowerBound::ExactGrammar { mass, .. } => {
                        let m = gmass;
                        assert_eq!(*mass, m, "the grammar mass reproduces ({label})");
                    }
                    LowerBound::ExactResponse { mass } => {
                        let m = response;
                        assert_eq!(*mass, m, "the response mass reproduces ({label})");
                    }
                    LowerBound::Vacuous | LowerBound::Sampled(_) => {}
                }
                if let UpperBound::ExactResponse { mass } = &interval.upper {
                    let m = response;
                    assert_eq!(*mass, m, "the point upper reproduces ({label})");
                }
            }

            // Result typing is faithful.
            match &outcome.result {
                RefineResult::Settled { action, proof } => {
                    assert_eq!(outcome.survivors.len(), 1, "settled means one ({label})");
                    assert!(outcome.survivors.contains(*action), "{label}");
                    if cfg.prefix == 0 {
                        assert_eq!(*proof, ProofClass::Exact, "no δ, exact proof ({label})");
                    }
                }
                RefineResult::Equivalent {
                    actions,
                    value,
                    proof,
                } => {
                    assert_eq!(*actions, outcome.survivors, "{label}");
                    assert!(actions.len() >= 2, "{label}");
                    for interval in &outcome.intervals {
                        if actions.contains(interval.action) {
                            assert!(
                                interval.deterministic_point(),
                                "equivalence is deterministic points ({label})"
                            );
                            assert_eq!(interval.lower_value(), *value, "{label}");
                        }
                    }
                    if cfg.prefix == 0 {
                        assert_eq!(*proof, ProofClass::Exact, "no δ, exact proof ({label})");
                    }
                }
                RefineResult::Unresolved { .. } => {
                    panic!("an ample budget settles every gated root ({label})");
                }
            }
            assert!(outcome.work_spent <= outcome.budget, "{label}");
            if cfg.prefix == 0 {
                assert!(
                    outcome.risk_spent.numer().bits() == 0,
                    "no δ spent ({label})"
                );
                assert!(!outcome.delta_decisive, "no δ decisive ({label})");
            } else {
                assert!(outcome.risk_spent <= cfg.scope_budget, "{label}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Gate 3 — §34 steering refusals and determinism.
// ---------------------------------------------------------------------------

#[test]
fn steering_refuses_presently_useless_work_deterministically() {
    let r = receipt();
    let oracle = SupportOracle;
    let cfg = ample_exact();
    for (hand_id, trick_no) in GATED_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let spec = level0_spec();
        let outcome = refined(&cfg, hand_id, trick_no);
        let label = format!("h{hand_id}-t{trick_no}");

        // The consequence census is refused as presently useless at
        // every bar (its best case narrows field branch intervals,
        // never a root bound) — and it never runs.
        let census_refused = outcome.trace.iter().any(|e| {
            matches!(
                e,
                TraceEvent::Refused {
                    item: WorkItem::ConsequenceCensus,
                    reason: RefusalReason::PresentlyUseless,
                }
            )
        });
        let multi_action = root_actions(&root, &position).len() >= 2;
        if multi_action {
            assert!(census_refused, "the census is refused (§34) ({label})");
        }
        for event in &outcome.trace {
            if let TraceEvent::Ran(e) = event {
                assert_ne!(
                    e.item,
                    WorkItem::ConsequenceCensus,
                    "the census never runs inside the loop ({label})"
                );
            }
        }

        // No item runs on an excluded action, and refused items charge
        // nothing: the run costs sum exactly to the work spent.
        let mut excluded_seen = DominoSet::EMPTY;
        let mut charged: u64 = 0;
        for event in &outcome.trace {
            match event {
                TraceEvent::Excluded { action, .. } => {
                    excluded_seen.insert(*action);
                }
                TraceEvent::Ran(e) => {
                    if let Some(a) = match e.item {
                        WorkItem::SampledLower(a)
                        | WorkItem::SampledUpper(a)
                        | WorkItem::ExactFixed(a)
                        | WorkItem::ExactGrammar(a)
                        | WorkItem::EscalateExact(a) => Some(a),
                        WorkItem::ConsequenceCensus => None,
                    } {
                        assert!(
                            !excluded_seen.contains(a),
                            "no work runs on an excluded action ({label})"
                        );
                    }
                    charged += e.cost;
                }
                TraceEvent::Refused { .. } => {}
            }
        }
        assert_eq!(
            charged, outcome.work_spent,
            "refusals charge nothing ({label})"
        );

        // Determinism: a run is a pure function of its inputs — one fresh
        // run against the fixture's.
        let again = refine_root(&root, &position, &spec, &oracle, &cfg);
        assert_eq!(
            format!("{:?}", outcome),
            format!("{:?}", again),
            "bytewise-identical records across repeated runs ({label})"
        );
    }
}

// ---------------------------------------------------------------------------
// Gate 4 — §36 step 12 honesty under starvation.
// ---------------------------------------------------------------------------

#[test]
fn budget_exhaustion_returns_the_honest_surviving_set() {
    let r = receipt();
    let oracle = SupportOracle;
    let starved_cfg = RefineConfig {
        budget: 1,
        prefix: 0,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    };
    let mut starved_somewhere = false;
    for (hand_id, trick_no) in GATED_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let spec = level0_spec();
        let outcome = refine_root(&root, &position, &spec, &oracle, &starved_cfg);
        let label = format!("h{hand_id}-t{trick_no}");
        let legal = root_actions(&root, &position);

        if legal.len() == 1 {
            // One legal action settles with no work at any budget.
            assert!(
                matches!(outcome.result, RefineResult::Settled { .. }),
                "{label}"
            );
            continue;
        }
        starved_somewhere = true;
        match &outcome.result {
            RefineResult::Unresolved {
                survivors,
                fallback,
                rule,
            } => {
                // The honest surviving set: nothing ran, nothing pruned.
                assert_eq!(*survivors, legal, "no work, no exclusion ({label})");
                assert!(survivors.contains(*fallback), "{label}");
                assert_eq!(
                    *fallback,
                    survivors.iter().next().expect("a survivor"),
                    "the named rule is lowest-tile-among-survivors ({label})"
                );
                assert_eq!(*rule, "lowest-tile-among-survivors", "{label}");
            }
            other => panic!("budget 1 starves a multi-action root ({label}): {other:?}"),
        }
        assert_eq!(outcome.work_spent, 0, "nothing charged ({label})");
        assert!(
            outcome.risk_spent.numer().bits() == 0,
            "no δ spent ({label})"
        );
        assert!(
            outcome.trace.iter().any(|e| matches!(
                e,
                TraceEvent::Refused {
                    reason: RefusalReason::ExceedsBudget,
                    ..
                }
            )),
            "the starvation is recorded (§32) ({label})"
        );
        assert!(
            !outcome
                .trace
                .iter()
                .any(|e| matches!(e, TraceEvent::Ran(_))),
            "budget 1 runs nothing in the gated corpus ({label})"
        );
    }
    assert!(starved_somewhere, "the gated corpus exercises starvation");

    // The ledger fence: a scope budget of exactly one endpoint δ admits
    // at most one sampled item, and the refusal is recorded.
    let (root, position) = root_at(&r, 3, 5);
    let spec = level0_spec();
    let tight = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 20),
    };
    let outcome = refine_root(&root, &position, &spec, &oracle, &tight);
    let sampled_runs = outcome
        .trace
        .iter()
        .filter(|e| {
            matches!(
                e,
                TraceEvent::Ran(r) if matches!(
                    r.item,
                    WorkItem::SampledLower(_) | WorkItem::SampledUpper(_)
                )
            )
        })
        .count();
    assert!(sampled_runs <= 1, "one endpoint δ fits the scope once");
    assert!(outcome.risk_spent <= tight.scope_budget, "the ledger holds");
}
