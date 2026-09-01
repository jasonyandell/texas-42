//! EXPLORATORY BELLMAN-REPORT INSTRUMENT (anytime proof-state Phases
//! 4 and 5; `walt/math/anytime_proof_state_score_v0.1.md` §61/§62,
//! rulings APS-A8/APS-A9) — sits below every evidentiary tier and is
//! cited by nothing above it. Instrument output only: per root ×
//! action, the §61 F STAIRCASE — every stage's root interval, exact
//! mass, and class census up to the action-exact endpoint, with the
//! exact §36 response beside it — the stage-1 fixed-policy tail
//! envelope's §7 straddle; and the §62 COVER TABLE — the verified
//! movement bound against the §5 arithmetic envelope, the named
//! resources, the derived rescue-band upper against the exact
//! response, and the per-action verdict (collapsed / hazard-visible /
//! open). Never a play-strength claim; wall time is honest context.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; incumbents = lowest-first baselines; staircase
//! stages 0..=28 (one fresh critical tile per refinement). Frozen
//! `verify_player` receipt roots: the six enumerable fibers plus
//! h3-t4.
//!
//! Modes:
//!   `bellmanreport report <out.txt>` — the Phase 4/5 probe
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
use walt::solver::covers::cover_for_action;
use walt::solver::factor_belief::{
    response_success_mass, staged_policy_envelope, staged_response_interval, viewer_score_profile,
    ExactCoverOracle, FactorBelief, RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{Fact, ProofState, ScoreProfileFact, SemanticsIdentity};

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

fn permille(mass: u128, z: u128) -> u128 {
    mass * 1000 / z
}

fn permille_r(v: &BigRational) -> BigInt {
    (v * BigRational::from_integer(BigInt::from(1000)))
        .floor()
        .numer()
        .clone()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: bellmanreport report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");

    writeln!(
        out,
        "bellman-report instrument (anytime proof-state Phases 4/5, §61/§62/APS-A8/A9)\n\
         field=Level0{{n0=2}} oracle=SupportOracle; incumbents = lowest-first baselines.\n\
         Staircase rows: stage s -> [lower,upper]permille exact-mass-permille classes(exact).\n\
         Cover rows: verified gain vs arithmetic envelope, resources, derived upper vs exact.\n"
    )
    .unwrap();

    let total_start = Instant::now();
    for (hand_id, trick_no) in ROOTS {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let identity = identity_of(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let mut state = ProofState::open(&root, &position, identity.clone());
        let root_start = Instant::now();
        writeln!(
            out,
            "== h{hand_id}-t{trick_no} bid={} viewer={} z={} legal={} ==",
            position.bid,
            identity.utility_id,
            oracle.mass(&belief),
            state.legal.len()
        )
        .unwrap();

        // Phase 4: the staircase per action.
        for a in &state.legal {
            let child = belief.focal_play(*a);
            let z = oracle.mass(&child);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            writeln!(
                out,
                " action {a}: exact-response {}permille",
                permille(exact, z)
            )
            .unwrap();
            for s in 0..=28usize {
                let mut ss = ResponseStats::default();
                let i = staged_response_interval(&oracle, &child, &field, s, &mut ss);
                writeln!(
                    out,
                    "   stage {s}: [{},{}]permille exact {}permille classes {}({})",
                    permille(i.lower, z),
                    permille(i.upper, z),
                    permille(i.exact_mass, z),
                    i.classes,
                    i.exact_classes
                )
                .unwrap();
                if i.residual_mass == 0 {
                    break;
                }
            }
            let mut es = RecursionStats::default();
            let e = staged_policy_envelope(&oracle, &child, &low, &field, 1, &mut es);
            let c = position.bid as usize;
            writeln!(
                out,
                "   envelope(s=1, lowest-first): straddle-at-contract {}permille",
                permille(e.upper_tail[c] - e.lower_tail[c], z)
            )
            .unwrap();
        }

        // Phase 5: install baselines, then the cover table.
        for a in state.legal.clone() {
            let child = belief.focal_play(a);
            let mut ps = RecursionStats::default();
            let profile = viewer_score_profile(&oracle, &child, &low, &field, &mut ps);
            state
                .install(
                    &identity,
                    Fact::Profile(Box::new(ScoreProfileFact {
                        action: a,
                        policy_id: "lowest-first-after-root-action".to_string(),
                        bins: profile.bins,
                    })),
                )
                .expect("a baseline profile installs");
        }
        let unbanked = 42 - position.banked[0] - position.banked[1];
        let mut cover_facts = Vec::new();
        for a in state.legal.clone() {
            let fact = cover_for_action(&oracle, &root, &position, &field, &state, a)
                .expect("every action holds an incumbent here");
            let Fact::Cover(cv) = &fact else {
                unreachable!()
            };
            let fives: Vec<String> = cv.five_count_tiles.iter().map(|d| format!("{d}")).collect();
            let tens: Vec<String> = cv.ten_count_tiles.iter().map(|d| format!("{d}")).collect();
            writeln!(
                out,
                " cover {a}: gain {} (envelope {unbanked}) tricks {} fives [{}] tens [{}]",
                cv.score_gain_upper,
                cv.trick_gain_upper,
                fives.join(" "),
                tens.join(" ")
            )
            .unwrap();
            cover_facts.push(fact);
        }
        for fact in cover_facts {
            state.install(&identity, fact).expect("a cover installs");
        }
        let report = state.closure();
        for v in &report.views {
            let child = belief.focal_play(v.action);
            let z = oracle.mass(&child);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            let lower = permille_r(&v.lower);
            let upper = permille_r(&v.upper);
            let verdict = if v.upper == v.lower {
                "COLLAPSED"
            } else {
                "OPEN(hazard visible)"
            };
            writeln!(
                out,
                "   {}: incumbent {lower}permille upper {upper}permille exact {}permille {verdict}",
                v.action,
                permille(exact, z)
            )
            .unwrap();
        }
        writeln!(out, " wall {} us\n", root_start.elapsed().as_micros()).unwrap();
    }
    writeln!(out, "total wall {} us", total_start.elapsed().as_micros()).unwrap();
}
