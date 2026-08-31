//! EXPLORATORY RECOMMENDATION-REPORT INSTRUMENT (anytime proof-state
//! Phase 3; `walt/math/anytime_proof_state_score_v0.1.md` §29–§33,
//! §60, rulings APS-A6/APS-A7) — sits below every evidentiary tier and
//! is cited by nothing above it. Instrument output only: the first §33
//! recommendation blocks on real roots — per root, the proof state is
//! fed the RefineV1 two-tier facts plus exact continuation profiles
//! for candidate root actions, and the report prints the recommended
//! executable policy, its certified pmake floor, the global
//! best-response upper, the certified regret Γ = U* − B_exec, the
//! declaring score floor/ceiling and d = 1 bands, the proof class,
//! and the risk scopes. Never a play-strength claim; the certified
//! regret is a bound on the declared semantics, not on 42.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; RefineV1 at the two-tier ample configuration
//! (prefix 16, δ = 1/20 per endpoint, scope 1/2); candidate profiles =
//! lowest-first continuations after up to three legal root actions.
//! Frozen `verify_player` receipt roots: the six enumerable fibers
//! plus h3-t4.
//!
//! Modes:
//!   `proofreport report <out.txt>` — the Phase 3 probe
//!
//! No floats anywhere; wall time is integer microseconds;
//! probabilities print as integer permille of exact rationals.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::factor_belief::{
    viewer_score_profile, FactorBelief, RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    facts_from_refine_interval, Fact, ProofState, ScoreProfileFact, SemanticsIdentity,
};
use walt::solver::refine::{refine_root, RefineConfig};

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

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

fn permille(v: &BigRational) -> BigInt {
    (v * BigRational::from_integer(BigInt::from(1000)))
        .floor()
        .numer()
        .clone()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: proofreport report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    let oracle = SupportOracle;
    let spec = level0_spec();
    let cfg = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 16,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    };

    writeln!(
        out,
        "recommendation-report instrument (anytime proof-state Phase 3, §33/APS-A7)\n\
         field=Level0{{n0=2}} oracle=SupportOracle; facts = RefineV1 two-tier ample\n\
         (prefix=16 delta=1/20 scope=1/2) + lowest-first continuation profiles for up\n\
         to three legal root actions. Γ = U* − B_exec is the certified pmake regret of\n\
         the recommended EXECUTABLE policy under the declared semantics.\n"
    )
    .expect("write");

    for (hand_id, trick_no) in ROOTS {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let t0 = Instant::now();
        let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
        let mut state = ProofState::open(&root, &position, identity.clone());
        for interval in &outcome.intervals {
            for fact in facts_from_refine_interval(interval) {
                state.install(&identity, fact).expect("a V1 fact installs");
            }
        }
        let candidates: Vec<_> = state.legal.iter().take(3).copied().collect();
        for a in &candidates {
            let focal = FixedPreference::lowest_first("focal:lowest-first");
            let field = FieldModel::new(level0_spec());
            let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(*a);
            let mut stats = RecursionStats::default();
            let profile = viewer_score_profile(&oracle, &belief, &focal, &field, &mut stats);
            state
                .install(
                    &identity,
                    Fact::Profile(Box::new(ScoreProfileFact {
                        action: *a,
                        policy_id: "lowest-first-after-root-action".to_string(),
                        bins: profile.bins,
                    })),
                )
                .expect("a profile installs");
        }
        let report = state.closure();
        let rec = state.recommend().expect("a recommendation exists");
        let wall = t0.elapsed().as_micros();
        let survivors: Vec<String> = report.survivors.iter().map(|d| format!("{d}")).collect();
        writeln!(
            out,
            "-- h{hand_id}-t{trick_no} bid={} viewer={} survivors=[{}] wall={wall} µs",
            position.bid,
            if identity.utility_id == "pmake-v1" {
                "declaring"
            } else {
                "setting"
            },
            survivors.join(" "),
        )
        .expect("write");
        writeln!(
            out,
            "   recommended action={} policy={}",
            rec.action, rec.policy
        )
        .expect("write");
        writeln!(
            out,
            "   pmake floor {}‰  global upper {}‰  CERTIFIED REGRET {}‰  proof={}",
            permille(&rec.pmake_lower),
            permille(&rec.global_upper),
            permille(&rec.certified_regret),
            if rec.sampled {
                "delta-qualified"
            } else {
                "deterministic"
            },
        )
        .expect("write");
        let bands = match (
            &rec.declaring_score_floor,
            &rec.declaring_score_ceiling,
            &rec.declaring_fragile_d1,
            &rec.declaring_rescue_d1,
        ) {
            (Some(f), Some(c), Some(fr), Some(re)) => format!(
                "score floor {f} ceiling {c}; declaring fragile(d=1) {}‰ rescue(d=1) {}‰",
                permille(fr),
                permille(re),
            ),
            _ => "score coordinates: none (bound-fact witness)".to_string(),
        };
        writeln!(out, "   {bands}").expect("write");
        writeln!(
            out,
            "   risk scopes: {}",
            if rec.risk_scopes.is_empty() {
                "none".to_string()
            } else {
                format!("{} sampled scopes present", rec.risk_scopes.len())
            },
        )
        .expect("write");
    }
    writeln!(
        out,
        "\nno root was dropped: all seven roots ran to completion."
    )
    .expect("write");
}
