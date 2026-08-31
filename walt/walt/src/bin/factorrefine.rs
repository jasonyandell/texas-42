//! EXPLORATORY REFINEMENT-CONTROLLER INSTRUMENT (counted-belief Slice G;
//! `walt/math/counted_belief_sandwich_v0.1.md` Part VIII §32–37, §50,
//! rulings CBS-A6/CBS-A9) — sits below every evidentiary tier and is
//! cited by nothing above it. Instrument output only: per-root
//! controller traces — which §33 work item the §35 scheduler ran at
//! each step and what it cost, how the typed intervals narrowed, where
//! the §34 rule refused work, when actions fell below the bar, and the
//! §53 central coordinates (root decision width versus cumulative
//! declared cost). Never a play-strength claim; the controller
//! schedules and accounts, it manufactures no bound (§37.8).
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; sampled tier where a section turns it on at the
//! Slice A declaration (prefix 16, δ = 1/20 per endpoint, upper epoch
//! 0, evaluation epoch 1), root scope budget 4/5. Frozen
//! `verify_player` receipt roots: the ten gated Slice F roots, plus the
//! opening root h0-t1 for the affordability-cliff section.
//!
//! Modes:
//!   `factorrefine report <out.txt>` — the Slice G probe
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::DominoSet;
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::refine::{
    refine_root, ActionInterval, LowerBound, ProofClass, RefineConfig, RefineOutcome, RefineResult,
    RefusalReason, TraceEvent, UpperBound, WorkItem,
};

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

fn field_spec() -> FieldSpec {
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

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn micros(from: Instant) -> u128 {
    from.elapsed().as_micros()
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn permille_q(v: &BigRational) -> u128 {
    let scaled = (v * BigRational::from_integer(BigInt::from(1000))).floor();
    u128::try_from(scaled.to_integer()).expect("a permille of a probability fits u128")
}

fn tiles_of(set: DominoSet) -> String {
    let names: Vec<String> = set.iter().map(|d| d.to_string()).collect();
    if names.is_empty() {
        "-".to_string()
    } else {
        names.join(" ")
    }
}

fn lower_kind(b: &LowerBound) -> &'static str {
    match b {
        LowerBound::Vacuous => "vacuous",
        LowerBound::Sampled(_) => "sampled",
        LowerBound::ExactPolicy { .. } => "exact-fixed",
        LowerBound::ExactGrammar { .. } => "exact-grammar",
        LowerBound::ExactResponse { .. } => "exact-Q",
    }
}

fn upper_kind(b: &UpperBound) -> &'static str {
    match b {
        UpperBound::Vacuous => "vacuous",
        UpperBound::Sampled(_) => "sampled",
        UpperBound::ExactResponse { .. } => "exact-Q",
    }
}

fn result_line(outcome: &RefineOutcome) -> String {
    match &outcome.result {
        RefineResult::Settled { action, proof } => format!(
            "SETTLED {action} ({})",
            match proof {
                ProofClass::Exact => "exact",
                ProofClass::DeltaQualified => "delta-qualified",
            }
        ),
        RefineResult::Equivalent {
            actions,
            value,
            proof,
        } => format!(
            "EQUIVALENT [{}] value {} ({}‰) ({})",
            tiles_of(*actions),
            value,
            permille_q(value),
            match proof {
                ProofClass::Exact => "exact",
                ProofClass::DeltaQualified => "delta-qualified",
            }
        ),
        RefineResult::Unresolved {
            survivors,
            fallback,
            rule,
        } => format!(
            "UNRESOLVED [{}] fallback {fallback} (rule {rule}; never promoted)",
            tiles_of(*survivors)
        ),
    }
}

/// The §53 central coordinates after each run step: decision width
/// (survivor count and total excess-upper permille) against cumulative
/// declared cost.
fn trace_table(outcome: &RefineOutcome) -> String {
    let mut out = String::new();
    out.push_str(
        "    step item                     cost      cum  after[l,u]‰   bar‰ surv  event\n",
    );
    let mut cum: u64 = 0;
    let mut step = 0usize;
    for event in &outcome.trace {
        match event {
            TraceEvent::Ran(e) => {
                step += 1;
                cum += e.cost;
                out.push_str(&format!(
                    "    {:>4} {:<24} {:>6} {:>8}  [{:>4},{:>4}] {:>6} {:>4}  ran\n",
                    step,
                    e.item.to_string(),
                    e.cost,
                    cum,
                    permille_q(&e.lower_after),
                    permille_q(&e.upper_after),
                    permille_q(&e.bar_after),
                    e.survivors_after.len(),
                ));
            }
            TraceEvent::Excluded {
                action,
                bar_holder,
                delta_decisive,
            } => {
                out.push_str(&format!(
                    "         excluded {action} below the bar held by {bar_holder}{}\n",
                    if *delta_decisive {
                        " (delta-decisive)"
                    } else {
                        " (exact)"
                    }
                ));
            }
            TraceEvent::Refused { item, reason } => {
                out.push_str(&format!(
                    "         refused {item}: {}\n",
                    match reason {
                        RefusalReason::ExcludedAction => "excluded action (§34)",
                        RefusalReason::PresentlyUseless => "presently useless (§34)",
                        RefusalReason::ExceedsBudget => "exceeds remaining budget",
                        RefusalReason::ExceedsRiskScope => "exceeds the root risk scope",
                    }
                ));
            }
        }
    }
    out
}

fn interval_lines(intervals: &[ActionInterval]) -> String {
    let mut out = String::new();
    for iv in intervals {
        out.push_str(&format!(
            "    {}: [{}, {}] = [{}‰, {}‰] lower {} upper {}\n",
            iv.action,
            iv.lower_value(),
            iv.upper_value(),
            permille_q(&iv.lower_value()),
            permille_q(&iv.upper_value()),
            lower_kind(&iv.lower),
            upper_kind(&iv.upper),
        ));
    }
    out
}

fn run_root(
    out: &mut std::fs::File,
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    cfg: &RefineConfig,
    full_trace: bool,
) -> RefineOutcome {
    let (root, position) = root_at(r, hand_id, trick_no);
    let spec = field_spec();
    let oracle = SupportOracle;
    let field_probe = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field_probe);
    let z = oracle.mass(&belief);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    let start = Instant::now();
    let outcome = refine_root(&root, &position, &spec, &oracle, cfg);
    let us = micros(start);
    writeln!(
        out,
        "== root h{hand_id}-t{trick_no} fiber={} Z={z} legal={} budget={} prefix={}",
        root.count(),
        legal.len(),
        if cfg.budget > 1 << 60 {
            "ample".to_string()
        } else {
            cfg.budget.to_string()
        },
        cfg.prefix,
    )
    .expect("write");
    if full_trace {
        write!(out, "{}", trace_table(&outcome)).expect("write");
        write!(out, "{}", interval_lines(&outcome.intervals)).expect("write");
    }
    writeln!(
        out,
        "    result {} | work {} | risk {} | bar {}‰ | wall {us} us",
        result_line(&outcome),
        outcome.work_spent,
        outcome.risk_spent,
        permille_q(&outcome.bar),
    )
    .expect("write");
    outcome
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: factorrefine report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");

    writeln!(
        out,
        "refinement-controller instrument (counted-belief Slice G, §50/Part VIII)\n\
         field=Level0{{n0=2}} oracle=SupportOracle grammar=[lowest,highest,count-preservation]\n\
         sampled tier where on: prefix=16 delta=1/20-per-endpoint scope=4/5 epochs upper=0 eval=1\n\
         work units are DECLARED FORECASTS (deterministic), never wall time\n"
    )
    .expect("write");

    // Section A — the exact-only ladder at ample budget: the §36 loop
    // with the sampled tier off. The upper story is escalation itself
    // (§40.1's regime: no cheap structural upper exists on this ladder).
    writeln!(
        out,
        "== SECTION A: exact-only ladder, ample budget (prefix=0) =="
    )
    .expect("write");
    let ample_exact = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 0,
        delta: q(1, 20),
        scope_budget: q(4, 5),
    };
    for (hand_id, trick_no) in GATED_ROOTS {
        run_root(&mut out, &receipt, hand_id, trick_no, &ample_exact, true);
    }

    // Section B — the two-tier ladder: Slice A sampled δ bounds on top
    // of the exact rungs, same roots. The question (§51, integrated
    // solver row): do cheap sampled bounds prune before escalation?
    writeln!(
        out,
        "\n== SECTION B: two-tier ladder, ample budget (prefix=16, delta=1/20) =="
    )
    .expect("write");
    let ample_two = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(4, 5),
    };
    for (hand_id, trick_no) in GATED_ROOTS {
        let outcome = run_root(&mut out, &receipt, hand_id, trick_no, &ample_two, false);
        let mut first_escalation: Option<usize> = None;
        let mut first_exclusion: Option<usize> = None;
        let mut delta_exclusions = 0u64;
        let mut position_in_trace = 0usize;
        for event in &outcome.trace {
            match event {
                TraceEvent::Ran(e) => {
                    position_in_trace += 1;
                    if matches!(e.item, WorkItem::EscalateExact(_)) && first_escalation.is_none() {
                        first_escalation = Some(position_in_trace);
                    }
                }
                TraceEvent::Excluded { delta_decisive, .. } => {
                    if first_exclusion.is_none() {
                        first_exclusion = Some(position_in_trace);
                    }
                    if *delta_decisive {
                        delta_exclusions += 1;
                    }
                }
                TraceEvent::Refused { .. } => {}
            }
        }
        writeln!(
            out,
            "    first exclusion after step {:?}, first escalation at step {:?}, \
             delta-decisive exclusions {}",
            first_exclusion, first_escalation, delta_exclusions,
        )
        .expect("write");
    }

    // Section C — the budget ladder (§53's central graph): decision
    // width against cumulative declared cost, two trick-4 roots.
    writeln!(out, "\n== SECTION C: budget ladder (prefix=0) ==").expect("write");
    for (hand_id, trick_no) in [(3usize, 4usize), (8, 4)] {
        writeln!(out, "  -- h{hand_id}-t{trick_no} --").expect("write");
        writeln!(
            out,
            "    budget    work surv result                              bar‰"
        )
        .expect("write");
        for budget in [1000u64, 3000, 6000, 12000, 48000, u64::MAX / 2] {
            let cfg = RefineConfig {
                budget,
                prefix: 0,
                delta: q(1, 20),
                scope_budget: q(4, 5),
            };
            let (root, position) = root_at(&receipt, hand_id, trick_no);
            let outcome = refine_root(&root, &position, &field_spec(), &SupportOracle, &cfg);
            writeln!(
                out,
                "    {:>9} {:>5} {:>4} {:<38} {:>4}",
                if budget > 1 << 60 {
                    "ample".to_string()
                } else {
                    budget.to_string()
                },
                outcome.work_spent,
                outcome.survivors.len(),
                result_line(&outcome),
                permille_q(&outcome.bar),
            )
            .expect("write");
        }
    }

    // Section D — the opening root across the affordability cliff: the
    // sampled tier fits the budget, every exact recursion is refused by
    // its own declared forecast (the §40.4/§40.5 walls, honestly
    // labeled), and the controller returns the honest surviving set.
    writeln!(
        out,
        "\n== SECTION D: opening root h0-t1, sampled tier only (budget=100000) =="
    )
    .expect("write");
    let cliff = RefineConfig {
        budget: 100_000,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(4, 5),
    };
    let outcome = run_root(&mut out, &receipt, 0, 1, &cliff, true);
    match &outcome.result {
        RefineResult::Unresolved { survivors, .. } => {
            writeln!(
                out,
                "    the exact tier is refused by forecast at this budget (the §40 \
                 contraction/field-classification walls); {} of {} actions survive the \
                 sampled bounds; the surviving set is returned honestly, the fallback \
                 is named and never promoted (§37.9)",
                survivors.len(),
                outcome.intervals.len(),
            )
            .expect("write");
        }
        other => {
            writeln!(out, "    note: the sampled tier alone reached {other:?}").expect("write");
        }
    }
    writeln!(
        out,
        "    worlds in the fiber 399072960; worlds sampled per endpoint <= 16; \
         no complete-world enumeration ran"
    )
    .expect("write");

    println!("factorrefine: report written");
}
