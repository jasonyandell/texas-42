//! EXPLORATORY OPENING-ROOT LADDER INSTRUMENT (anytime proof-state
//! Phase 8; `walt/math/anytime_proof_state_score_v0.1.md` §65, ruling
//! APS-A9) — sits below every evidentiary tier and is cited by nothing
//! above it. Instrument output only: the §65 report panel at each
//! declared budget stop of one opening-root run — surviving actions,
//! proof and executable bars, global upper, certified regret,
//! width debt, F masses, policy cylinders, count-threat cells, risk,
//! work, wall time, the recommendation, and the typed verdict. Never a
//! play-strength claim.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the opening root h0-t1 of the frozen
//! `verify_player` receipt (fiber 399,072,960); sampled tier at the
//! Slice A declaration (upper epoch 0, evaluation epoch 1), δ = 1/100
//! per endpoint, root scope budget 3/5; ladder ε = 1/4; census at the
//! Phase 4 declared stage; frontier budgets in the §40 Z units. The
//! δ-valid endpoints are functions of the DECLARED δ: this ladder's
//! values differ from RefineV1 Section D's at the same prefix because
//! Section D declared δ = 1/20 — same streams, same successes,
//! tighter risk here, more conservative bounds (stated, not drift).
//!
//! Modes:
//!   `openingreport report <out.txt>` — the Phase 8 probe
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::frontier::{Refusal, WorkItem};
use walt::solver::opening::{OpeningLadder, OpeningStopSpec, StopReport, StopVerdict};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofState, SemanticsIdentity};

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

fn identity_of(root: &CanonicalRoot, position: &RootPosition) -> SemanticsIdentity {
    let declaring = root.kernel().viewer().team() == position.declaring_team;
    SemanticsIdentity {
        root_id: walt::solver::adaptive::root_identity(root, position),
        rules_id: "texas42-v1".to_string(),
        field_id: "level0-modeled-mind-v1".to_string(),
        utility_id: if declaring {
            "pmake-v1".to_string()
        } else {
            "pmake-setting-v1".to_string()
        },
        contract: position.bid,
        belief_id: "uniform-root".to_string(),
        policy_class_id: "information-consistent-full".to_string(),
        score_semantics_id: "declaring-banked-43bin-v1".to_string(),
    }
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn permille(v: &BigRational) -> u128 {
    let scaled = (v * BigRational::from_integer(BigInt::from(1000))).floor();
    u128::try_from(scaled.to_integer()).expect("a permille of a probability fits u128")
}

fn verdict_name(v: StopVerdict) -> &'static str {
    match v {
        StopVerdict::Exact => "EXACT",
        StopVerdict::DeltaQualified => "DELTA-QUALIFIED",
        StopVerdict::EpsilonOptimal => "EPSILON-OPTIMAL",
        StopVerdict::Unresolved => "UNRESOLVED",
    }
}

fn item_name(item: &WorkItem) -> String {
    match item {
        WorkItem::BaselineProfile { action } => format!("baseline-profile({action})"),
        WorkItem::ExactValue { action } => format!("exact-value({action})"),
        WorkItem::ExtractArgmax { action } => format!("extract-argmax({action})"),
        WorkItem::ResidualInterval { action } => format!("residual-interval({action})"),
        WorkItem::ExactValueSurvivors => "exact-value-survivors".to_string(),
    }
}

fn refusal_name(r: &Refusal) -> &'static str {
    match r {
        Refusal::ZeroPotential => "zero potential (§34/§41)",
        Refusal::AlreadyPresent => "already present",
        Refusal::Unaffordable => "exceeds remaining budget",
    }
}

fn print_stop(out: &mut String, report: &StopReport, state: &ProofState, wall_us: u128) {
    out.push_str(&format!(
        "\n== STOP {}: wall={}us sampled_work={} facts={} ==\n",
        report.label, wall_us, report.sampled_work, report.facts
    ));
    let closure = state.closure();
    for v in &closure.views {
        out.push_str(&format!(
            "    {}: [{:4},{:4}]‰ {}{}{}\n",
            v.action,
            permille(&v.lower),
            permille(&v.upper),
            if v.lower_sampled {
                "lower-sampled "
            } else {
                ""
            },
            if v.upper_sampled {
                "upper-sampled "
            } else {
                ""
            },
            if v.excluded { "EXCLUDED" } else { "" }
        ));
    }
    let survivors: Vec<String> = report.survivors.iter().map(|d| d.to_string()).collect();
    out.push_str(&format!(
        "  survivors: {} of {}  [{}]\n",
        report.survivors.len(),
        report.survivors.len() + report.excluded.len(),
        survivors.join(" ")
    ));
    out.push_str(&format!(
        "  proof_bar={}‰ exec_bar={}‰ global_upper={}‰ certified_regret={}‰\n",
        permille(&report.proof_bar),
        permille(&report.exec_bar),
        permille(&report.global_upper),
        permille(&report.certified_regret)
    ));
    out.push_str(&format!(
        "  width_debt={}‰ policy_cylinders={} count_threat_cells={} covers_installed={}\n",
        permille(&report.width_debt),
        report.policy_cylinders,
        report.count_threat_cells,
        report.covers_installed
    ));
    out.push_str(&format!(
        "  risk_spent={} ({} scopes) delta_decisive={}\n",
        report.risk_spent,
        report.risk_scopes.len(),
        report.delta_decisive
    ));
    match &report.contract_sensitive_residual {
        Some(r) => out.push_str(&format!("  contract_sensitive_residual={}‰\n", permille(r))),
        None => out.push_str("  contract_sensitive_residual=none (bound-fact witness)\n"),
    }
    if !report.census.is_empty() {
        out.push_str(&format!(
            "  census (stage {} of the CEGAR record, §49 coordinate):\n",
            report.census[0].stage
        ));
        for c in &report.census {
            let z = c.exact_mass + c.residual_mass;
            out.push_str(&format!(
                "    after {}: classes={} exact_classes={} exact_mass={}‰ residual_mass={}‰ of Z={}\n",
                c.action,
                c.classes,
                c.exact_classes,
                (c.exact_mass * 1000) / z,
                (c.residual_mass * 1000) / z,
                z
            ));
        }
    }
    if let Some(f) = &report.frontier {
        out.push_str(&format!(
            "  frontier: executed={} spent={}Z-units met={} refusals={}\n",
            f.executed.len(),
            f.spent,
            f.met,
            f.refusals.len()
        ));
        for e in &f.executed {
            out.push_str(&format!(
                "    bought {} cost={} debt {}→{}\n",
                item_name(&e.item),
                e.cost,
                e.debt_before,
                e.debt_after
            ));
        }
        for (item, r) in &f.refusals {
            out.push_str(&format!(
                "    refused {}: {}\n",
                item_name(item),
                refusal_name(r)
            ));
        }
    }
    match &report.recommendation {
        Some(r) => out.push_str(&format!(
            "  recommendation: {} via {} pmake_lower={}‰ upper={}‰ regret={}‰ sampled={}\n",
            r.action,
            r.policy,
            permille(&r.pmake_lower),
            permille(&r.global_upper),
            permille(&r.certified_regret),
            r.sampled
        )),
        None => out.push_str("  recommendation: none (no executable witness yet)\n"),
    }
    out.push_str(&format!("  verdict: {}\n", verdict_name(report.verdict)));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 3 && args[1] == "report",
        "usage: openingreport report <out.txt>"
    );
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&receipt, 0, 1);
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let z = oracle.mass(&belief);

    let mut out = String::new();
    out.push_str(
        "ANYTIME PROOF-STATE PHASE 8 — THE §65 OPENING-ROOT ITERATIVE RUN (exploratory)\n",
    );
    out.push_str(
        "==============================================================================\n",
    );
    out.push_str(
        "One opening root, one append-only proof state, declared budget stops,\n\
         the full §65 report panel at every stop. σ0 Level0{n0=2} under\n\
         SupportOracle; δ=1/100 per endpoint against root scope budget 3/5;\n\
         ladder ε=1/4; census at the Phase 4 declared stage; frontier budgets\n\
         in §40 Z units. The declared first target (§65): not a seven-trick\n\
         exact solution — a materially smaller correct survivor set or a\n\
         useful certified-regret recommendation under a playable budget.\n",
    );
    out.push_str(&format!(
        "\nroot h0-t1 fiber Z={} legal={} contract={}\n",
        z,
        belief.kernel().viewer_hand().len(),
        position.bid
    ));

    let ladder = OpeningLadder {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        scope_budget: q(3, 5),
        epsilon: q(1, 4),
    };
    let stops = [
        OpeningStopSpec {
            label: "0 zero-budget (§25 top state)".to_string(),
            sampled_prefix: 0,
            endpoint_delta: q(1, 100),
            census: false,
            frontier_budget: 0,
        },
        OpeningStopSpec {
            label: "1 sampled p=16".to_string(),
            sampled_prefix: 16,
            endpoint_delta: q(1, 100),
            census: false,
            frontier_budget: 0,
        },
        OpeningStopSpec {
            label: "2 sampled p=64".to_string(),
            sampled_prefix: 64,
            endpoint_delta: q(1, 100),
            census: false,
            frontier_budget: 0,
        },
        OpeningStopSpec {
            label: "3 sampled p=256, census, frontier Z/2".to_string(),
            sampled_prefix: 256,
            endpoint_delta: q(1, 100),
            census: true,
            frontier_budget: z / 2,
        },
        OpeningStopSpec {
            label: "4 sampled p=512, frontier Z/2".to_string(),
            sampled_prefix: 512,
            endpoint_delta: q(1, 100),
            census: false,
            frontier_budget: z / 2,
        },
    ];

    let out_path = &args[2];
    let flush = |text: &str| {
        let mut f = std::fs::File::create(out_path).expect("the output file opens");
        f.write_all(text.as_bytes()).expect("the report writes");
    };
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity);
    for spec in &stops {
        let t = Instant::now();
        let report = ladder.run_stop(&mut state, spec);
        let wall = t.elapsed().as_micros();
        print_stop(&mut out, &report, &state, wall);
        flush(&out);
    }
    out.push_str(&format!(
        "\nserialized proof state: {} facts, {} bytes (walt-proof-state-v1)\n",
        state.facts().len(),
        state.serialize().len()
    ));
    flush(&out);
    println!("{out}");
}
