//! EXPLORATORY MODEL-BELIEF RECURSION INSTRUMENT (`solver::model_recursion`,
//! slice MB1) — sits below every evidentiary tier and is cited by
//! nothing above it. The brief's required probe: per root, the depth the
//! posterior-carrying recursion reached, the posterior evolution
//! summary, the mixture value against MB0's own root value (which must
//! match), the F₁ read count against the declared budget, and wall time;
//! then the earlier-root Φ table, which is the number of the slice; then
//! the field-identity fence census. Findings language only, never a
//! play-strength claim and never theorem language for a measurement.
//!
//! DECLARED EPOCH (MB0's, unchanged so the two probes are comparable):
//! F₀ = σ0 = `Level0 { n0 = 2 }` and F₁ = `Level1 { n_outer = 2, n0 = 2 }`
//! registered as hand-persistent behavior types; prior ν = (1/2, 1/2)
//! per hidden seat, independent (integer weights 1, denominator 8 over
//! three hidden seats); `SupportOracle`; focal ρ = the lowest-first
//! frozen preference where a fixed policy is needed.
//!
//! DECLARED BUDGETS. Every cap below is in FIELD CONSULTATIONS — the
//! unit `ReadLedger` measures at the dispatch itself — and every
//! reported read count is a measurement, never the cap. The declared
//! per-coordinate ceilings are constants in this file so a reader can
//! see the whole budget in one place.
//!
//! Modes:
//!   `modelbeliefrecursionreport report <out.txt>` — the declared run.
//!   `modelbeliefrecursionreport measure <hand> <trick> <cap>` — a bare
//!       affordability probe on one root, printed to stdout. It exists
//!       because the earlier-root budgets in the declared run have to be
//!       chosen from measurements rather than from guesses.
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals printed beside integer permille.

use std::fmt::Write as _;
use std::rc::Rc;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::{ExactCoverOracle, SupportOracle};
use walt::solver::field::{FieldKind, FieldSpec};
use walt::solver::model_belief::{
    BehaviorType, MixtureStats, ModelBelief, PersistenceScope, SeatTypePrior,
};
use walt::solver::model_recursion::{
    column_of, couple_fixed_field_fact, mixture_field_id, mixture_identity, model_census,
    response_vector, sweep_envelope, trace_heaviest_line, two_type_grid, ActionCoordinate,
    CensusBudget, ModelBeliefProducer, ModelFieldId, ResponseEnvelope, RootModelCensus,
};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofProducer, ProofState};

/// MB0's six enumerable receipt roots, re-run under the recursion:
/// (hand, trick, fiber).
const MB0_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

/// The earlier roots MB1 reaches for (item 5): trick 4 is the stratum
/// U0 measured a positive God gap at and MB0 never entered, and h12-t4
/// is carried precisely because it is the vacuous one — a Φ = 0 there
/// must not be allowed to read as evidence. The strictly pre-t4 attempt
/// is `PRE_T4_ROOTS`.
const EARLIER_ROOTS: [(usize, usize, u128); 3] = [(8, 4, 1_200), (12, 4, 34_650), (3, 4, 11_550)];

/// The strictly pre-trick-4 attempt: the smallest receipt fiber at
/// trick 3. Declared so that whatever comes back is a measured budget
/// decision at a NAMED coordinate rather than an absence.
const PRE_T4_ROOTS: [(usize, usize, u128); 1] = [(8, 3, 59_976)];

/// Declared field-read ceiling for ONE root action's exact mixture
/// response on the MB0 corpus. Generous: these roots are known to close.
const MB0_RESPONSE_CAP: u64 = 4_000_000;
/// Declared ceiling for one root action's whole `U^sep` point-mass
/// sequence on the MB0 corpus.
const MB0_SEPARATED_CAP: u64 = 4_000_000;
/// Declared ceiling per root action at the earlier (trick 4) roots.
/// Comfortably above the largest trick-4 coordinate measured while
/// choosing it (6,901,094 reads, h3-t4 action 4-4), so trick 4 is
/// answered rather than budgeted.
const EARLIER_RESPONSE_CAP: u64 = 12_000_000;
const EARLIER_SEPARATED_CAP: u64 = 12_000_000;
/// Declared ceiling per root action at the strictly pre-trick-4 roots,
/// set to the LARGEST spend any trick-4 coordinate actually needed,
/// rounded up. The question this coordinate asks is therefore a precise
/// one — "does a trick-3 coordinate close within the most a trick-4
/// coordinate cost?" — and not "will it close at any price".
const PRE_T4_RESPONSE_CAP: u64 = 7_000_000;
const PRE_T4_SEPARATED_CAP: u64 = 7_000_000;

/// Grid resolution for the ν sweep (item 2): `SWEEP_STEPS + 1` beliefs
/// from δ_{F₀} to δ_{F₁} along the per-seat weight line.
const SWEEP_STEPS: u128 = 8;

/// The posterior-trace depth requested per root (item 1).
const TRACE_PLIES: usize = 5;

fn field_spec_level0() -> FieldSpec {
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

fn field_spec_level1() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 { n_outer: 2, n0: 2 },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn registered_types() -> (Rc<BehaviorType>, Rc<BehaviorType>) {
    (
        Rc::new(BehaviorType::from_field(
            field_spec_level0(),
            PersistenceScope::PerHand,
        )),
        Rc::new(BehaviorType::from_field(
            field_spec_level1(),
            PersistenceScope::PerHand,
        )),
    )
}

fn mixture_of(
    root: &CanonicalRoot,
    position: &RootPosition,
    a: &Rc<BehaviorType>,
    b: &Rc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(a), 1), (Rc::clone(b), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

fn delta_of(
    root: &CanonicalRoot,
    position: &RootPosition,
    behavior: &Rc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(behavior), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

/// Exact rational as "num/den (~NNN permille)" — integer arithmetic.
fn ratio(mass: u128, total: u128) -> String {
    assert!(total > 0);
    let permille = mass.checked_mul(1000).expect("fits") / total;
    let r = BigRational::new(BigInt::from(mass), BigInt::from(total));
    format!("{r} (~{permille}‰)")
}

fn permille(mass: u128, total: u128) -> u128 {
    assert!(total > 0);
    mass.checked_mul(1000).expect("fits") / total
}

// ---------------------------------------------------------------------------
// One root's section.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_lines)]
fn report_root(
    out: &mut String,
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    declared_fiber: u128,
    budget: CensusBudget,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
    trace_line: bool,
) -> Option<RootModelCensus> {
    let oracle = SupportOracle;
    let (root, position) = root_at(r, hand_id, trick_no);
    let label = format!("h{hand_id}-t{trick_no}");
    let model = mixture_of(&root, &position, f0, f1);
    let started = Instant::now();

    writeln!(out, "== {label} [posterior-carrying recursion]").expect("write");
    writeln!(
        out,
        "declared fiber {declared_fiber}; augmented Σw·Z = {}; profiles {}; \
         field identity {}",
        model.weighted_total(&oracle),
        model.profiles().len(),
        mixture_field_id(&model)
    )
    .expect("write");
    writeln!(
        out,
        "declared budget per root action: response {} reads, U^sep {} reads",
        budget
            .response_cap
            .map_or("uncapped".to_string(), |c| c.to_string()),
        budget
            .separated_cap
            .map_or("uncapped".to_string(), |c| c.to_string()),
    )
    .expect("write");

    // The posterior-carrying line (item 1).
    if trace_line {
        let focal = FixedPreference::lowest_first("focal:lowest-first");
        let reads_before = model.ledger().total();
        let (_, trace) = trace_heaviest_line(&oracle, &model, &focal, TRACE_PLIES);
        writeln!(
            out,
            "posterior line: depth {} of {} requested{}; live profiles {} -> {}; \
             a profile was eliminated: {}; line reads {}",
            trace.depth(),
            TRACE_PLIES,
            if trace.exhausted {
                " (the line ran out of hidden steps)"
            } else {
                ""
            },
            model.profiles().len(),
            trace.final_live,
            trace.eliminated_a_profile(),
            model.ledger().total() - reads_before,
        )
        .expect("write");
        for step in &trace.steps {
            let mut branches = String::new();
            for (tile, mass) in &step.branches {
                write!(branches, " {tile}:{mass}").expect("write");
            }
            writeln!(
                out,
                "  seat {} branches {{{} }} typed {} vs merged {}; observed {} \
                 (live {} -> {})",
                step.seat,
                branches,
                step.census.0,
                step.census.1,
                step.observed,
                step.live_before,
                step.live_after
            )
            .expect("write");
            for (seat, marginal) in &step.marginals {
                let mut cells = String::new();
                for (id, mass) in marginal {
                    write!(cells, " {}:{}/{}", id.short(), mass, step.weighted_total)
                        .expect("write");
                }
                writeln!(out, "    posterior {seat}[{cells} ]").expect("write");
            }
        }
    }

    // The census (items 4, 5).
    let census = model_census(&oracle, &label, &model, budget);
    let mut root_best: Option<(u128, u128)> = None;
    for coordinate in &census.coordinates {
        match coordinate {
            ActionCoordinate::Priced(p) => {
                let better = root_best.is_none_or(|(m, _)| p.q.0 > m);
                if better {
                    root_best = Some(p.q);
                }
                writeln!(
                    out,
                    "  action {}: Q_a = {}; U^sep_a = {}; Φ_a = {} [{}]{}; reads {}",
                    p.action,
                    ratio(p.q.0, p.q.1),
                    ratio(p.usep.0, p.usep.1),
                    ratio(p.phi.0, p.phi.1),
                    if p.substantive {
                        "substantive"
                    } else {
                        "VACUOUS — U^sep at an endpoint, the zero says nothing"
                    },
                    match &p.common_optimizer {
                        Some(c) => format!(
                            "; §19 common optimizer {} — Φ_a = 0 at EVERY ν over these types",
                            &c.policy_id[..24.min(c.policy_id.len())]
                        ),
                        None => String::new(),
                    },
                    p.reads
                )
                .expect("write");
            }
            ActionCoordinate::ResponseRefused { action, refusal } => {
                writeln!(out, "  action {action}: REFUSED (response) — {refusal}").expect("write");
            }
            ActionCoordinate::SeparatedRefused { action, q, refusal } => {
                writeln!(
                    out,
                    "  action {action}: Q_a = {}; U^sep REFUSED — {refusal}; Φ_a NOT reported \
                     (an upper that did not finish is not an upper)",
                    ratio(q.0, q.1)
                )
                .expect("write");
            }
        }
    }

    // MB0 parity: the root value is the max over root actions (item 1's
    // recursion-vs-MB0 check, in the probe as well as in the gate). The
    // re-walk costs as much as the census did, so it is spent only on
    // the roots where that is cheap; the earlier roots' parity is the
    // gate's job (M1) and not worth doubling this run for.
    if census.refusals() == 0 && trace_line {
        let mut stats = MixtureStats::default();
        let root_response = model.mixture_response(&oracle, &mut stats);
        let mb0 = (
            root_response.outcome.weighted_mass,
            root_response.outcome.weighted_total,
        );
        let recursed = root_best.expect("a priced census has a best action");
        writeln!(
            out,
            "MB0 root value {} vs recursion max over actions {} — {}",
            ratio(mb0.0, mb0.1),
            ratio(recursed.0, recursed.1),
            if mb0.0 * recursed.1 == recursed.0 * mb0.1 {
                "MATCH"
            } else {
                "MISMATCH"
            }
        )
        .expect("write");
    } else if census.refusals() > 0 {
        writeln!(
            out,
            "MB0 root-value comparison SKIPPED: {} of {} coordinates refused, so the \
             recursion has no max to compare",
            census.refusals(),
            census.coordinates.len()
        )
        .expect("write");
    } else {
        writeln!(
            out,
            "MB0 root-value comparison NOT RUN here: the re-walk costs what the census \
             cost, and this root's per-action parity is gate M1's"
        )
        .expect("write");
    }

    let mut by_type = String::new();
    for (id, n) in &census.reads_by_type {
        write!(by_type, " {}:{n}", id.short()).expect("write");
    }
    writeln!(
        out,
        "reads total {} ({} ); priced {} / refused {}; substantive zero prices {}; \
         vacuous zero prices {}; strict prices {}",
        census.reads,
        by_type,
        census.coordinates.len() - census.refusals(),
        census.refusals(),
        census.substantive_zero_prices(),
        census.vacuous_zero_prices(),
        census.strict_prices().len(),
    )
    .expect("write");
    writeln!(
        out,
        "wall {} us",
        u128::try_from(started.elapsed().as_micros()).expect("fits")
    )
    .expect("write");
    writeln!(out).expect("write");
    Some(census)
}

// ---------------------------------------------------------------------------
// The ν sweep (item 2).
// ---------------------------------------------------------------------------

fn report_sweep(
    out: &mut String,
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
) {
    let oracle = SupportOracle;
    let (root, position) = root_at(r, hand_id, trick_no);
    let label = format!("h{hand_id}-t{trick_no}");
    let model = mixture_of(&root, &position, f0, f1);
    let action = model
        .legal_focal_actions()
        .expect("the root has the viewer to move")
        .iter()
        .next()
        .expect("a root holds a legal action");
    let at_action = model.focal_play(action);
    let totals: Vec<u128> = at_action
        .profiles()
        .iter()
        .map(|e| oracle.mass(e.belief()))
        .collect();
    let grid = two_type_grid(model.profiles()[0].types().len(), SWEEP_STEPS);
    let mut envelope = ResponseEnvelope::new(totals.clone());
    let started = Instant::now();
    let sweep = sweep_envelope(&mut envelope, &grid, |weights| {
        let reweighted = reweighted_like(&at_action, weights, &root, &position, f0, f1);
        let mut stats = MixtureStats::default();
        let response = reweighted
            .mixture_response_budgeted(&oracle, None, &mut stats)
            .expect("an uncapped walk on a sweep root does not refuse");
        let (vector, column_totals) = response_vector(&oracle, &at_action, &response.policy, None)
            .expect("an uncapped fixed-policy walk does not refuse");
        assert_eq!(column_totals, totals, "one state, one set of totals");
        Ok((
            response.outcome.weighted_mass,
            column_of(&response.policy, vector, column_totals),
        ))
    })
    .expect("an uncapped sweep does not refuse");

    writeln!(
        out,
        "== ν-sweep at {label}, root action {action} (item 2, §21/§23)"
    )
    .expect("write");
    writeln!(
        out,
        "grid: {} beliefs from δ_F0 to δ_F1 along the per-seat weight line, \
         integer weights, {} product profiles",
        grid.len(),
        grid[0].len()
    )
    .expect("write");
    let facets = sweep.iter().filter(|p| p.new_facet).count();
    for point in &sweep {
        writeln!(
            out,
            "  ν weights {:?}: Q = {} via {}{}",
            point.weights,
            ratio(point.reading.value.0, point.reading.value.1),
            &point.reading.policy_id[..24.min(point.reading.policy_id.len())],
            if point.new_facet { " [NEW FACET]" } else { "" }
        )
        .expect("write");
    }
    writeln!(
        out,
        "facets {} of {} grid points — a repricing sweep needs one walk per facet; \
         the remaining {} points are dot products (§23). Envelope columns {}.",
        facets,
        sweep.len(),
        sweep.len() - facets,
        envelope.len()
    )
    .expect("write");
    writeln!(
        out,
        "wall {} us",
        u128::try_from(started.elapsed().as_micros()).expect("fits")
    )
    .expect("write");
    writeln!(out).expect("write");
}

/// A model belief at the same state as `at_action` but with the given
/// product weights — rebuilt from the root and replayed, because prior
/// weights are immutable by construction (persistence, MB-I2).
fn reweighted_like(
    at_action: &ModelBelief,
    weights: &[u128],
    root: &CanonicalRoot,
    position: &RootPosition,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
) -> ModelBelief {
    let profiles: Vec<(Vec<Rc<BehaviorType>>, u128)> = at_action
        .profiles()
        .iter()
        .zip(weights.iter())
        .map(|(entry, w)| {
            (
                entry
                    .types()
                    .iter()
                    .map(|t| {
                        if t.id() == f0.id() {
                            Rc::clone(f0)
                        } else {
                            Rc::clone(f1)
                        }
                    })
                    .collect::<Vec<Rc<BehaviorType>>>(),
                *w,
            )
        })
        .collect();
    let fresh = ModelBelief::from_profile_prior(root, position, profiles);
    let mut live = fresh;
    for tile in at_action.history() {
        live = live.focal_play(*tile);
    }
    live
}

// ---------------------------------------------------------------------------
// The field-identity fence census (item 7).
// ---------------------------------------------------------------------------

fn report_fence(
    out: &mut String,
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
) {
    let oracle = SupportOracle;
    let (root, position) = root_at(r, hand_id, trick_no);
    let label = format!("h{hand_id}-t{trick_no}");
    let mixture = mixture_of(&root, &position, f0, f1);
    let delta0 = delta_of(&root, &position, f0);
    let sigma0 = ModelFieldId::Fixed(f0.parent_field().to_string());
    let sigma1 = ModelFieldId::Fixed(f1.parent_field().to_string());

    writeln!(out, "== field-identity fence at {label} (item 7)").expect("write");
    writeln!(
        out,
        "source σ0 = {sigma0}; source σ1 = {sigma1}; mixture target = {}; δ_F0 target = {}",
        mixture_field_id(&mixture),
        mixture_field_id(&delta0)
    )
    .expect("write");

    let mut stats = MixtureStats::default();
    let delta_value = delta0.mixture_response(&oracle, &mut stats);
    let fact = ModelBeliefProducer::new(model_census(
        &oracle,
        &label,
        &delta0,
        CensusBudget::default(),
    ))
    .produce(&ProofState::open(
        &root,
        &position,
        mixture_identity(&root, &position, &delta0),
    ))
    .into_iter()
    .next()
    .expect("a priced census proposes a fact");

    let witness = walt::solver::model_recursion::PointMassWitness {
        fixed_side: (
            delta_value.outcome.weighted_mass,
            delta_value.outcome.weighted_total,
        ),
        model_side: (
            delta_value.outcome.weighted_mass,
            delta_value.outcome.weighted_total,
        ),
        behavior: f0.id(),
    };

    for (name, source, target, w) in [
        (
            "σ0 fact into the (1/2,1/2) MIXTURE",
            &sigma0,
            &mixture,
            Some(witness.clone()),
        ),
        (
            "σ0 fact into δ_F0 with a re-run parity witness",
            &sigma0,
            &delta0,
            Some(witness.clone()),
        ),
        ("σ0 fact into δ_F0 with NO witness", &sigma0, &delta0, None),
        (
            "σ1 fact into δ_F0 (wrong parent field)",
            &sigma1,
            &delta0,
            Some(witness.clone()),
        ),
    ] {
        let verdict = match couple_fixed_field_fact(fact.clone(), source, target, w) {
            Ok(c) => format!("COUPLED — {:?}", c.coupling()),
            Err(e) => format!("REFUSED — {e}"),
        };
        writeln!(out, "  {name}: {verdict}").expect("write");
    }

    // The §49 half: the store's own identity fence.
    let mut state = ProofState::open(
        &root,
        &position,
        mixture_identity(&root, &position, &mixture),
    );
    let mut sigma0_identity = mixture_identity(&root, &position, &mixture);
    sigma0_identity.field_id = f0.parent_field().to_string();
    let rejected = state.install(&sigma0_identity, fact.clone());
    writeln!(
        out,
        "  §49 store: a σ0-authored fact installed under the mixture identity -> {rejected:?}"
    )
    .expect("write");
    writeln!(out).expect("write");
}

// ---------------------------------------------------------------------------
// Driver.
// ---------------------------------------------------------------------------

fn measure(hand_id: usize, trick_no: usize, cap: u64) {
    let r = load_receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, hand_id, trick_no);
    let model = mixture_of(&root, &position, &f0, &f1);
    let budget = CensusBudget {
        response_cap: if cap == 0 { None } else { Some(cap) },
        separated_cap: if cap == 0 { None } else { Some(cap) },
    };
    let started = Instant::now();
    let census = model_census(&oracle, &format!("h{hand_id}-t{trick_no}"), &model, budget);
    println!(
        "h{hand_id}-t{trick_no}: fiber {} priced {} refused {} reads {} wall {} us",
        census.fiber,
        census.coordinates.len() - census.refusals(),
        census.refusals(),
        census.reads,
        started.elapsed().as_micros()
    );
    for c in &census.coordinates {
        match c {
            ActionCoordinate::Priced(p) => println!(
                "  {} Q {}/{} ({}‰) Usep {}/{} ({}‰) Phi {}/{} ({}‰) substantive {} reads {}",
                p.action,
                p.q.0,
                p.q.1,
                permille(p.q.0, p.q.1),
                p.usep.0,
                p.usep.1,
                permille(p.usep.0, p.usep.1),
                p.phi.0,
                p.phi.1,
                permille(p.phi.0, p.phi.1),
                p.substantive,
                p.reads
            ),
            ActionCoordinate::ResponseRefused { action, refusal } => {
                println!("  {action} response refused: {refusal}");
            }
            ActionCoordinate::SeparatedRefused {
                action, refusal, ..
            } => println!("  {action} usep refused: {refusal}"),
        }
    }
}

fn load_receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn report(path: &str) {
    let r = load_receipt();
    let (f0, f1) = registered_types();
    let mut out = String::new();
    let started = Instant::now();

    writeln!(
        out,
        "modelbeliefrecursionreport run 1 — the MB1 probe (EXPLORATORY tier)"
    )
    .expect("write");
    writeln!(
        out,
        "epoch: F0 = level0-modeled-mind-v1 (id {}), F1 = level1-modeled-mind-v1 (id {}); \
         prior (1/2,1/2) per hidden seat, independent; SupportOracle; focal = lowest-first \
         frozen preference. Values exact; reads MEASURED at the field dispatch; every cap \
         is a DECLARED budget in field consultations.",
        f0.id().short(),
        f1.id().short()
    )
    .expect("write");
    writeln!(out).expect("write");

    writeln!(
        out,
        "---- PART 1: MB0's corpus under the posterior-carrying recursion ----"
    )
    .expect("write");
    writeln!(out).expect("write");
    let mb0_budget = CensusBudget {
        response_cap: Some(MB0_RESPONSE_CAP),
        separated_cap: Some(MB0_SEPARATED_CAP),
    };
    let mut all: Vec<RootModelCensus> = Vec::new();
    for (hand_id, trick_no, fiber) in MB0_ROOTS {
        if let Some(c) = report_root(
            &mut out, &r, hand_id, trick_no, fiber, mb0_budget, &f0, &f1, true,
        ) {
            all.push(c);
        }
    }

    writeln!(
        out,
        "---- PART 2: repricing the model belief instead of re-walking it ----"
    )
    .expect("write");
    writeln!(out).expect("write");
    report_sweep(&mut out, &r, 5, 6, &f0, &f1);
    report_sweep(&mut out, &r, 8, 5, &f0, &f1);

    writeln!(
        out,
        "---- PART 3: the earlier roots (item 5 — the number of the slice) ----"
    )
    .expect("write");
    writeln!(out).expect("write");
    let earlier_budget = CensusBudget {
        response_cap: Some(EARLIER_RESPONSE_CAP),
        separated_cap: Some(EARLIER_SEPARATED_CAP),
    };
    let mut earlier: Vec<RootModelCensus> = Vec::new();
    for (hand_id, trick_no, fiber) in EARLIER_ROOTS {
        if let Some(c) = report_root(
            &mut out,
            &r,
            hand_id,
            trick_no,
            fiber,
            earlier_budget,
            &f0,
            &f1,
            false,
        ) {
            earlier.push(c);
        }
    }
    let pre_t4_budget = CensusBudget {
        response_cap: Some(PRE_T4_RESPONSE_CAP),
        separated_cap: Some(PRE_T4_SEPARATED_CAP),
    };
    for (hand_id, trick_no, fiber) in PRE_T4_ROOTS {
        if let Some(c) = report_root(
            &mut out,
            &r,
            hand_id,
            trick_no,
            fiber,
            pre_t4_budget,
            &f0,
            &f1,
            false,
        ) {
            earlier.push(c);
        }
    }

    writeln!(out, "---- PART 4: the Φ table ----").expect("write");
    writeln!(out).expect("write");
    writeln!(
        out,
        " root      | trick | coords | priced | Φ>0 | Φ=0 subst | Φ=0 vacuous | refused"
    )
    .expect("write");
    writeln!(
        out,
        "-----------+-------+--------+--------+-----+-----------+-------------+--------"
    )
    .expect("write");
    for c in all.iter().chain(earlier.iter()) {
        let trick = c
            .root_label
            .split('t')
            .next_back()
            .expect("a label carries its trick");
        writeln!(
            out,
            " {:<9} | {:>5} | {:>6} | {:>6} | {:>3} | {:>9} | {:>11} | {:>7}",
            c.root_label,
            trick,
            c.coordinates.len(),
            c.coordinates.len() - c.refusals(),
            c.strict_prices().len(),
            c.substantive_zero_prices(),
            c.vacuous_zero_prices(),
            c.refusals()
        )
        .expect("write");
    }
    writeln!(out).expect("write");

    writeln!(out, "---- PART 5: the field-identity fence ----").expect("write");
    writeln!(out).expect("write");
    report_fence(&mut out, &r, 5, 6, &f0, &f1);

    writeln!(
        out,
        "total wall {} us (single-threaded; σ0/σ1 field caches shared within this run)",
        started.elapsed().as_micros()
    )
    .expect("write");

    std::fs::write(path, out).expect("the probe writes its report");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("report") => report(args.get(2).map_or("modelbelief_recursion_run1.txt", |s| s)),
        Some("measure") => measure(
            args[2].parse().expect("a hand id"),
            args[3].parse().expect("a trick number"),
            args[4].parse().expect("a read cap, 0 for uncapped"),
        ),
        _ => {
            eprintln!(
                "usage: modelbeliefrecursionreport report <out.txt>\n       \
                 modelbeliefrecursionreport measure <hand> <trick> <cap>"
            );
            std::process::exit(2);
        }
    }
}
