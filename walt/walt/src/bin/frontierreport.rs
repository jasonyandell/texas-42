//! EXPLORATORY FRONTIER-REPORT INSTRUMENT (anytime proof-state
//! Phase 1, the Part IX half; `walt/math/anytime_proof_state_score_v0.1.md`
//! §35, §39–§44, rulings APS-A8/APS-A9) — sits below every evidentiary
//! tier and is cited by nothing above it. Instrument output only: the
//! first anytime SCHEDULES on real roots — per root × goal, the
//! frontier starts from the top state (zero facts) and buys work items
//! under the declared cost model (Z per fixed-policy walk, 3Z per max
//! walk), printing every purchase (item, forecast cost, §42 bound,
//! debt trajectory), the §34/§41 refusal census at the stop, the exact
//! spend, and the §33 block where one exists. Never a play-strength
//! claim; costs are declared forecasts, not measurements — wall time
//! prints beside them for the honest comparison.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; goals = SelectAction, RecommendEpsilonPolicy(0),
//! StrengthenToExact. Frozen `verify_player` receipt roots: the six
//! enumerable fibers plus h3-t4.
//!
//! Modes:
//!   `frontierreport report <out.txt>` — the Phase 1 probe
//!
//! No floats anywhere; wall time is integer microseconds;
//! probabilities print as integer permille of exact rationals.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, SupportOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::frontier::{Frontier, Refusal, SolveGoal, WorkItem};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofState, SemanticsIdentity};

const ROOTS: [(usize, usize); 7] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5), (3, 4)];

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
        root_id: root_identity(root, position),
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

fn permille(v: &BigRational) -> BigInt {
    (v * BigRational::from_integer(BigInt::from(1000)))
        .floor()
        .numer()
        .clone()
}

fn item_name(item: &WorkItem) -> String {
    match item {
        WorkItem::BaselineProfile { action } => format!("baseline({action})"),
        WorkItem::ExactValue { action } => format!("exact({action})"),
        WorkItem::ExtractArgmax { action } => format!("extract({action})"),
        WorkItem::ExactValueSurvivors => "exact-survivors[§41 macro]".to_string(),
    }
}

fn goal_name(goal: &SolveGoal) -> &'static str {
    match goal {
        SolveGoal::SelectAction => "SelectAction",
        SolveGoal::RecommendEpsilonPolicy { .. } => "RecommendEpsilonPolicy(0)",
        SolveGoal::StrengthenToExact => "StrengthenToExact",
        SolveGoal::ComputeFullScoreProfile => "ComputeFullScoreProfile",
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: frontierreport report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    let oracle = SupportOracle;

    writeln!(
        out,
        "frontier-report instrument (anytime proof-state Phase 1, §35/§39–§44/APS-A8/A9)\n\
         field=Level0{{n0=2}} oracle=SupportOracle; top state (zero facts) per root × goal;\n\
         declared cost model: Z per fixed-policy walk, 3Z per max walk (forecasts, not\n\
         measurements — wall μs prints beside). Γ = U* − B_exec; permille floors.\n\
         EXPLORATORY — below every evidentiary tier, quotable only via gate receipts."
    )
    .expect("write");

    let goals = [
        SolveGoal::SelectAction,
        SolveGoal::RecommendEpsilonPolicy {
            epsilon: BigRational::zero(),
        },
        SolveGoal::StrengthenToExact,
    ];
    let total_start = Instant::now();
    for (hand_id, trick_no) in ROOTS {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let field = FieldModel::new(level0_spec());
        let z = {
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            oracle.mass(&belief)
        };
        writeln!(
            out,
            "\nroot h{hand_id}-t{trick_no}: fiber={z} bid={} parity={}",
            position.bid, identity.utility_id
        )
        .expect("write");
        let frontier = Frontier {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        for goal in &goals {
            let mut state = ProofState::open(&root, &position, identity.clone());
            let start = Instant::now();
            let report = frontier.advance(&mut state, goal, u128::MAX / 4);
            let wall = start.elapsed().as_micros();
            writeln!(
                out,
                "  goal {}: met={} spent={} ({}Z) wall={wall} μs",
                goal_name(goal),
                report.met,
                report.spent,
                report.spent / z
            )
            .expect("write");
            for step in &report.executed {
                writeln!(
                    out,
                    "    buy {}: cost={} bound={}‰ debt {}‰ -> {}‰",
                    item_name(&step.item),
                    step.cost,
                    permille(&step.bound),
                    permille(&step.debt_before),
                    permille(&step.debt_after)
                )
                .expect("write");
            }
            if !report.refusals.is_empty() {
                let zero = report
                    .refusals
                    .iter()
                    .filter(|(_, r)| *r == Refusal::ZeroPotential)
                    .count();
                let present = report
                    .refusals
                    .iter()
                    .filter(|(_, r)| *r == Refusal::AlreadyPresent)
                    .count();
                let poor = report
                    .refusals
                    .iter()
                    .filter(|(_, r)| *r == Refusal::Unaffordable)
                    .count();
                writeln!(
                    out,
                    "    refusals at stop: zero-potential={zero} already-present={present} unaffordable={poor}"
                )
                .expect("write");
            }
            if let Some(rec) = state.recommend() {
                writeln!(
                    out,
                    "    recommend: a={} policy={} floor={}‰ Γ={}‰",
                    rec.action,
                    rec.policy,
                    permille(&rec.pmake_lower),
                    permille(&rec.certified_regret)
                )
                .expect("write");
            }
        }
    }
    let total_us = total_start.elapsed().as_micros();
    writeln!(out, "\ntotal wall {total_us} μs; no root was dropped").expect("write");
}
