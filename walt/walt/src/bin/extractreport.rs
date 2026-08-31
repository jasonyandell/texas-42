//! EXPLORATORY EXTRACTION-REPORT INSTRUMENT (anytime proof-state
//! Phase 6; `walt/math/anytime_proof_state_score_v0.1.md` §30, §63,
//! ruling APS-A9) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: the §30 bridge on real
//! roots — per root, the proof state is fed the RefineV1 two-tier
//! facts plus σ0 lowest-first baseline profiles, the certified regret
//! is read, and then the §63 extraction producer runs and the regret
//! is read again; the report prints the executable bar's rise, the
//! extracted policy DAG sizes, the recommendation block after
//! extraction, and the §63 residual verdicts (closure / escape /
//! empty class) for the singleton, two-source, and three-source
//! grammars. Never a play-strength claim; every number is a bound on
//! the declared semantics, not on 42.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; RefineV1 at the two-tier ample configuration
//! (prefix 16, δ = 1/20 per endpoint, scope 1/2); extraction = the
//! full legal set per root action, tie rule lowest tile index. Frozen
//! `verify_player` receipt roots: the six enumerable fibers plus
//! h3-t4; residual verdicts on the enumerable six for three grammars
//! plus h3-t4 for the two-source grammar.
//!
//! Modes:
//!   `extractreport report <out.txt>` — the Phase 6 probe
//!
//! No floats anywhere; wall time is integer microseconds;
//! probabilities print as integer permille of exact rationals.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::Domino;
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::extraction::ExtractionProducer;
use walt::solver::factor_belief::{
    extract_success_policy, grammar_success_mass, residual_split, viewer_score_profile,
    ExactCoverOracle, ExtractionSource, FactorBelief, RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{CountPreservation, PolicyGrammar};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    facts_from_refine_interval, Fact, ProofState, ScoreProfileFact, SemanticsIdentity,
};
use walt::solver::refine::{refine_root, RefineConfig};

const ROOTS: [(usize, usize); 7] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5), (3, 4)];
const RESIDUAL_ENUM: [(usize, usize); 6] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)];

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

fn legal_roots(root: &CanonicalRoot, position: &RootPosition) -> Vec<Domino> {
    walt::rules::legal_plays(position.decl, root.kernel().viewer_hand(), None)
        .iter()
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: extractreport report <out.txt>");
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
        "extraction-report instrument (anytime proof-state Phase 6, §30/§63/APS-A9)\n\
         field=Level0{{n0=2}} oracle=SupportOracle; facts = RefineV1 two-tier ample\n\
         (prefix=16 delta=1/20 scope=1/2) + lowest-first baseline profiles, then the\n\
         argmax-extraction-v1 producer (full legal set, tie rule = lowest tile index).\n\
         Γ = U* − B_exec; permille floors of exact rationals; wall integer μs.\n\
         EXPLORATORY — below every evidentiary tier, quotable only via gate receipts."
    )
    .expect("write");

    let total_start = Instant::now();
    for (hand_id, trick_no) in ROOTS {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let z = oracle.mass(&belief);
        writeln!(
            out,
            "\nroot h{hand_id}-t{trick_no}: fiber={z} bid={} parity={}",
            position.bid, identity.utility_id
        )
        .expect("write");

        let mut state = ProofState::open(&root, &position, identity.clone());
        let refine_start = Instant::now();
        let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
        for interval in &outcome.intervals {
            for fact in facts_from_refine_interval(interval) {
                state
                    .install(&identity, fact)
                    .expect("a refine fact installs");
            }
        }
        let refine_us = refine_start.elapsed().as_micros();
        let low = FixedPreference::lowest_first("focal:lowest-first");
        for a in legal_roots(&root, &position) {
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
        let before = state.closure();
        let bexec_before = before
            .exec
            .as_ref()
            .map(|w| permille(&w.value).to_string())
            .unwrap_or_else(|| "none".to_string());
        writeln!(
            out,
            "  before: bar={}‰ B_exec={}‰ U*={}‰ Γ={}‰ (refine {refine_us} μs)",
            permille(&before.bar),
            bexec_before,
            permille(&before.u_star),
            permille(&before.certified_regret)
        )
        .expect("write");

        // The §63 extraction, reported per action, then installed
        // through the producer (one walk each; the report walk and the
        // producer walk are the same recursion — the doubling is the
        // probe's price, not the producer's).
        for a in legal_roots(&root, &position) {
            let child = belief.focal_play(a);
            let mut es = ResponseStats::default();
            let ex_start = Instant::now();
            let (mass, policy) = extract_success_policy(
                &oracle,
                &child,
                &ExtractionSource::FullLegal,
                &field,
                &mut es,
            );
            let ex_us = ex_start.elapsed().as_micros();
            let value = BigRational::new(BigInt::from(mass), BigInt::from(z));
            writeln!(
                out,
                "  extract a={a}: Q_a={}‰ dag_states={} focal={} hidden={} wall={ex_us} μs",
                permille(&value),
                policy.states(),
                es.focal_nodes,
                es.hidden_nodes
            )
            .expect("write");
        }
        let producer = ExtractionProducer {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let prod_start = Instant::now();
        for res in state.run_producer(&producer) {
            res.expect("an extraction fact installs");
        }
        let prod_us = prod_start.elapsed().as_micros();
        let after = state.closure();
        let bexec_after = after
            .exec
            .as_ref()
            .map(|w| permille(&w.value).to_string())
            .unwrap_or_else(|| "none".to_string());
        writeln!(
            out,
            "  after:  bar={}‰ B_exec={}‰ U*={}‰ Γ={}‰ (producer {prod_us} μs)",
            permille(&after.bar),
            bexec_after,
            permille(&after.u_star),
            permille(&after.certified_regret)
        )
        .expect("write");
        if let Some(rec) = state.recommend() {
            writeln!(
                out,
                "  recommend: a={} policy={} floor={}‰ upper={}‰ Γ={}‰ score=[{},{}] fragile_d1={}‰ rescue_d1={}‰",
                rec.action,
                rec.policy,
                permille(&rec.pmake_lower),
                permille(&rec.global_upper),
                permille(&rec.certified_regret),
                rec.declaring_score_floor
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                rec.declaring_score_ceiling
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                rec.declaring_fragile_d1
                    .as_ref()
                    .map(|v| permille(v).to_string())
                    .unwrap_or_else(|| "-".to_string()),
                rec.declaring_rescue_d1
                    .as_ref()
                    .map(|v| permille(v).to_string())
                    .unwrap_or_else(|| "-".to_string()),
            )
            .expect("write");
        }
    }

    // The §63 residual verdicts: closure (D ≤ M^G forces M* = M^G),
    // escape (M* = D > M^G), or an empty deviating class.
    writeln!(
        out,
        "\nresidual verdicts (per root action child; grammars: one=[low],\n\
         two=[low;high], three=[low;high;count-preservation]):"
    )
    .expect("write");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let one = PolicyGrammar::new(vec![&low]);
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let grammars: [(&str, &PolicyGrammar<'_>); 3] =
        [("one", &one), ("two", &two), ("three", &three)];
    for (hand_id, trick_no) in RESIDUAL_ENUM {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        for (label, grammar) in grammars {
            for a in legal_roots(&root, &position) {
                residual_line(
                    &mut out, &oracle, &belief, grammar, &field, hand_id, trick_no, label, a,
                );
            }
        }
    }
    // h3-t4: the two-source grammar where legal sets can exceed two.
    let (root, position) = root_at(&receipt, 3, 4);
    let field = FieldModel::new(level0_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    for a in legal_roots(&root, &position) {
        residual_line(&mut out, &oracle, &belief, &two, &field, 3, 4, "two", a);
    }

    let total_us = total_start.elapsed().as_micros();
    writeln!(out, "\ntotal wall {total_us} μs; no root was dropped").expect("write");
}

#[allow(clippy::too_many_arguments)]
fn residual_line(
    out: &mut std::fs::File,
    oracle: &SupportOracle,
    belief: &FactorBelief,
    grammar: &PolicyGrammar<'_>,
    field: &FieldModel,
    hand_id: usize,
    trick_no: usize,
    label: &str,
    a: Domino,
) {
    let child = belief.focal_play(a);
    let mut ss = ResponseStats::default();
    let (m_star, dev) = residual_split(oracle, &child, grammar, field, &mut ss);
    let mut gs = ResponseStats::default();
    let gram = grammar_success_mass(oracle, &child, grammar, field, &mut gs);
    let z = oracle.mass(&child);
    let verdict = match dev {
        None => "empty-class",
        Some(d) if d <= gram => "closure",
        Some(_) => "escape",
    };
    let dev_s = dev
        .map(|d| d.to_string())
        .unwrap_or_else(|| "-".to_string());
    writeln!(
        out,
        "  h{hand_id}-t{trick_no} g={label} a={a}: z={z} m*={m_star} gram={gram} dev={dev_s} -> {verdict}"
    )
    .expect("write");
}
