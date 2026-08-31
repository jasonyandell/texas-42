//! Gates for the anytime proof-state Phase 6 [L2 thread]: §63 argmax
//! extraction and residual policy bounds — the extracted policy
//! re-prices to the extraction optimum through the UNCHANGED
//! fixed-policy evaluator (gate 1), its serialized profile is one
//! realizable profile (gate 2), grammar plus residual exactly covers
//! the full policy class via the §12 identity `M* = max(M^G, D)`
//! (gate 3), a residual at or below the grammar lower proves
//! unrestricted closure `M* = M^G` (gate 4), the §20 fence: a
//! threshold-wise profile envelope exists, differs from every
//! contributing policy's own tails, and is NOT what extraction
//! serializes (gate 5), and the §30 bridge: the extraction producer
//! raises the executable bar to meet the proof bar and the certified
//! regret collapses to zero at an exactly-settled root (gate 6).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §30 (proof bar versus executable bar — the argmax-extraction
//! bridge), §63 (the Phase 6 gate list), §12/§48 (grammar optimum and
//! residual), §20 (the envelope fence, binding at APS-A4), under
//! ruling APS-A9 (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; grammars = the Slice E two-source (lowest-first,
//! highest-first) and three-source (+ CountPreservation) fences;
//! extraction tie rule = lowest tile index. Frozen `verify_player`
//! receipt roots: the six enumerable fibers.

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::Domino;
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::extraction::ExtractionProducer;
use walt::solver::factor_belief::{
    extract_success_policy, grammar_success_mass, residual_split, response_success_mass,
    viewer_score_profile, viewer_success_mass, ExactCoverOracle, ExtractionSource, FactorBelief,
    RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{CountPreservation, PolicyGrammar};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    facts_from_refine_interval, BoundFact, Fact, ProofState, ProofTag, ScoreProfileFact,
    SemanticsIdentity,
};
use walt::solver::refine::{refine_root, RefineConfig};

const ENUM_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
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

/// The legal root actions of one root, ascending tile index.
fn legal_roots(root: &CanonicalRoot, position: &RootPosition) -> Vec<Domino> {
    let legal = walt::rules::legal_plays(position.decl, root.kernel().viewer_hand(), None);
    legal.iter().collect()
}

/// Gate 1 — §63 first gate: the extracted policy's value equals the
/// extraction optimum, recomputed through the UNCHANGED fixed-policy
/// evaluator. Both sources: every grammar optimum and the full-legal
/// best response, at every enumerable root and every legal root
/// action's child. The equality is also the receipt that the declared
/// off-DAG completion never carries objective weight.
#[test]
fn an_extracted_policy_reprices_to_the_extraction_optimum() {
    let r = receipt();
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let oracle = SupportOracle;
    let sources = [
        ExtractionSource::FullLegal,
        ExtractionSource::Grammar(&two),
        ExtractionSource::Grammar(&three),
    ];
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        assert_eq!(oracle.mass(&belief), fiber);
        for source in &sources {
            for a in legal_roots(&root, &position) {
                let child = belief.focal_play(a);
                let mut es = ResponseStats::default();
                let (mass, policy) =
                    extract_success_policy(&oracle, &child, source, &field, &mut es);
                // The independent optimum the extraction claims.
                let mut os = ResponseStats::default();
                let optimum = match source {
                    ExtractionSource::FullLegal => {
                        response_success_mass(&oracle, &child, &field, &mut os)
                    }
                    ExtractionSource::Grammar(g) => {
                        grammar_success_mass(&oracle, &child, g, &field, &mut os)
                    }
                };
                assert_eq!(
                    mass, optimum,
                    "extraction returns the recursion's own optimum"
                );
                // The §63 re-pricing: the unchanged evaluator, the
                // extracted DAG as an ordinary frozen policy.
                let mut rs = RecursionStats::default();
                let repriced = viewer_success_mass(&oracle, &child, &policy, &field, &mut rs);
                assert_eq!(repriced, mass, "the argmax DAG re-prices to the optimum");
            }
        }
    }
}

/// Gate 2 — §63 second gate: the extracted policy's profile is ONE
/// realizable profile — it conserves the fiber mass binwise and its
/// contract tail projects to exactly the extraction optimum. Full
/// legal source at every enumerable root's best action child.
#[test]
fn an_extracted_profile_is_one_realizable_profile() {
    let r = receipt();
    let oracle = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let declaring = root.kernel().viewer().team() == position.declaring_team;
        for a in legal_roots(&root, &position) {
            let child = belief.focal_play(a);
            let mut es = ResponseStats::default();
            let (mass, policy) = extract_success_policy(
                &oracle,
                &child,
                &ExtractionSource::FullLegal,
                &field,
                &mut es,
            );
            let mut ps = RecursionStats::default();
            let profile = viewer_score_profile(&oracle, &child, &policy, &field, &mut ps);
            assert_eq!(profile.total(), fiber, "binwise conservation: Σ_s H(s) = Z");
            let tail = profile.tail(position.bid);
            let projected = if declaring { tail } else { fiber - tail };
            assert_eq!(
                projected, mass,
                "the profile's contract projection is the extraction optimum"
            );
        }
    }
}

/// Gate 3 — §63 third gate: grammar plus residual exactly covers the
/// full policy class. The §12 cover identity `M* = max(M^G, D)` holds
/// at every enumerable root child, both grammars — with `D = None`
/// (empty deviating class) forcing `M* = M^G` outright. The residual
/// walk's own `M*` must also agree with the independent Slice G
/// recursion.
#[test]
fn grammar_plus_residual_covers_the_full_policy_class() {
    let r = receipt();
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let oracle = SupportOracle;
    for grammar in [&two, &three] {
        for (hand_id, trick_no, _) in ENUM_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let field = FieldModel::new(level0_spec());
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            for a in legal_roots(&root, &position) {
                let child = belief.focal_play(a);
                let mut ss = ResponseStats::default();
                let (m_star, dev) = residual_split(&oracle, &child, grammar, &field, &mut ss);
                let mut gs = ResponseStats::default();
                let gram = grammar_success_mass(&oracle, &child, grammar, &field, &mut gs);
                let mut rs = ResponseStats::default();
                let full = response_success_mass(&oracle, &child, &field, &mut rs);
                assert_eq!(m_star, full, "the residual walk's M* is Slice G's");
                match dev {
                    Some(d) => assert_eq!(
                        m_star,
                        gram.max(d),
                        "the §12 cover identity: M* = max(M^G, D)"
                    ),
                    None => assert_eq!(
                        m_star, gram,
                        "an empty deviating class leaves only the grammar class"
                    ),
                }
            }
        }
    }
}

/// Gate 4 — §63 fourth gate, both directions: a residual at or below
/// the grammar lower proves unrestricted closure, `D ≤ M^G ⟹
/// M* = M^G`, checked wherever the premise holds; and where the
/// premise FAILS, the residual detects the escape exactly — `M* = D >
/// M^G` (the §12 boxed exclusion's other face). Grammars: singleton
/// (lowest-first — room to deviate everywhere), two-source, and
/// three-source. Non-vacuity is asserted for BOTH directions, plus
/// the structural finding this gate discovered: at these t5/t6 roots
/// every multi-source grammar's deviating class is EMPTY (`None`) —
/// post-root focal states hold ≤ 2 tiles, so two sources saturate
/// every legal set. Slice E's "ties free everywhere at t5/t6" is not
/// a value coincidence but class saturation, and a strict-loss
/// specimen (`Some(D) < M^G`) is structurally absent at these roots:
/// the singleton grammar leaves room but σ0-fielded lowest-first is
/// weak enough that some deviation always ties or wins.
#[test]
fn a_residual_below_the_grammar_lower_proves_unrestricted_closure() {
    let r = receipt();
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let one = PolicyGrammar::new(vec![&low]);
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let oracle = SupportOracle;
    let mut closures = 0u64;
    let mut empty_class = 0u64;
    let mut escapes = 0u64;
    for grammar in [&one, &two, &three] {
        for (hand_id, trick_no, _) in ENUM_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let field = FieldModel::new(level0_spec());
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            for a in legal_roots(&root, &position) {
                let child = belief.focal_play(a);
                let mut ss = ResponseStats::default();
                let (m_star, dev) = residual_split(&oracle, &child, grammar, &field, &mut ss);
                let mut gs = ResponseStats::default();
                let gram = grammar_success_mass(&oracle, &child, grammar, &field, &mut gs);
                match dev {
                    None => {
                        empty_class += 1;
                        closures += 1;
                        assert_eq!(
                            m_star, gram,
                            "an empty deviating class leaves only the grammar class"
                        );
                    }
                    Some(d) if d <= gram => {
                        closures += 1;
                        assert_eq!(
                            m_star, gram,
                            "closure: no deviating policy beats the grammar, so the \
                             unrestricted optimum IS the grammar optimum"
                        );
                    }
                    Some(d) => {
                        escapes += 1;
                        assert_eq!(
                            m_star, d,
                            "escape: the residual attains the unrestricted optimum \
                             the grammar misses"
                        );
                        assert!(d > gram, "the escape premise");
                    }
                }
            }
        }
    }
    assert!(closures > 0, "the closure premise holds somewhere");
    assert!(
        empty_class > 0,
        "multi-source saturation at t5/t6: the deviating class is empty somewhere"
    );
    assert!(
        escapes > 0,
        "the singleton grammar leaks somewhere and the residual proves it"
    );
}

/// Gate 5 — §63 fifth gate, the §20 fence: a threshold-wise profile
/// envelope over two materialized policies exists, strictly exceeds
/// each contributor's own tails on opposite thresholds somewhere (so
/// it is not either policy's record), and what extraction serializes
/// is NEVER that envelope — the extracted policy's profile equals its
/// own fixed-policy evaluation bin for bin.
#[test]
fn no_threshold_envelope_is_serialized_as_the_extracted_policy() {
    let r = receipt();
    let oracle = SupportOracle;
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let mut specimen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        for a in legal_roots(&root, &position) {
            let child = belief.focal_play(a);
            let mut es = ResponseStats::default();
            let (_, policy) = extract_success_policy(
                &oracle,
                &child,
                &ExtractionSource::FullLegal,
                &field,
                &mut es,
            );
            let mut p1 = RecursionStats::default();
            let ext = viewer_score_profile(&oracle, &child, &policy, &field, &mut p1);
            let mut p2 = RecursionStats::default();
            let base = viewer_score_profile(&oracle, &child, &low, &field, &mut p2);
            // The extracted policy's serialized record is its OWN
            // evaluation — always, at every root.
            let mut p3 = RecursionStats::default();
            let again = viewer_score_profile(&oracle, &child, &policy, &field, &mut p3);
            assert_eq!(ext.bins, again.bins, "one policy, one profile, bin for bin");
            // The envelope specimen: some pair of thresholds where the
            // binwise-max tail takes from DIFFERENT contributors —
            // an object no single policy of the two realizes.
            let ext_beats = (0..=42u32).any(|k| ext.tail(k) > base.tail(k));
            let base_beats = (0..=42u32).any(|k| base.tail(k) > ext.tail(k));
            if ext_beats && base_beats {
                specimen = true;
                // And the envelope is NOT the extracted record: it
                // strictly exceeds the extracted tails somewhere.
                let k = (0..=42u32)
                    .find(|k| base.tail(*k) > ext.tail(*k))
                    .expect("the specimen has one");
                assert!(
                    base.tail(k).max(ext.tail(k)) > ext.tail(k),
                    "the envelope strictly exceeds the serialized record at k"
                );
            }
        }
    }
    assert!(
        specimen,
        "an envelope specimen exists among the enumerable roots: tails cross, \
         so the threshold-wise max is no single policy's record"
    );
}

/// Gate 6 — the §30 bridge, end to end, in two stages. Stage one: at
/// an enumerable root with RefineV1 facts and a σ0 executable
/// baseline, the extraction producer raises the executable bar to
/// meet the PROOF bar exactly — `B_exec = B_proof` at every root
/// (§30's chain closing from below). The certified regret does NOT
/// necessarily reach zero yet, and the gate found why: RefineV1
/// settles on cross-action dominance (the winner's lower beats every
/// rival's upper), so the WINNER'S own upper can stay vacuous at 1
/// and `U*` keeps it — at h4-t6 the after-extraction regret is
/// honestly 2/15 with the action certain. Stage two: installing the
/// §36 exact upper facts (`Q_a` is an upper as well as a lower)
/// prices the upper side, `U* = Q* = B_exec`, and the certified
/// regret collapses to EXACTLY zero, with the recommendation's policy
/// the extracted content id wherever the baseline had left a gap.
/// Both the pre-extraction gap and the post-extraction loose-upper
/// residue are asserted non-vacuous.
#[test]
fn the_extraction_producer_collapses_certified_regret_at_settled_roots() {
    let r = receipt();
    let oracle = SupportOracle;
    let spec = level0_spec();
    let cfg = RefineConfig {
        budget: u64::MAX / 2,
        prefix: 0,
        delta: q(1, 20),
        scope_budget: q(1, 2),
    };
    let mut gap_seen = false;
    let mut loose_upper_seen = false;
    let mut improved_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        // The proof side: RefineV1's exact ladder settles the root.
        let outcome = refine_root(&root, &position, &spec, &oracle, &cfg);
        for interval in &outcome.intervals {
            for fact in facts_from_refine_interval(interval) {
                state
                    .install(&identity, fact)
                    .expect("a refine fact installs");
            }
        }
        // An executable baseline: σ0 lowest-first continuation
        // profiles for every legal action.
        let field = FieldModel::new(level0_spec());
        let low = FixedPreference::lowest_first("focal:lowest-first");
        let belief = FactorBelief::uniform_root(&root, &position, &field);
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
        assert!(
            before.certified_regret >= BigRational::zero(),
            "regret is a nonnegative width"
        );
        let had_gap = before.certified_regret > BigRational::zero();
        if had_gap {
            gap_seen = true;
        }
        // The §30 bridge.
        let producer = ExtractionProducer {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        for outcome in state.run_producer(&producer) {
            outcome.expect("an extraction fact installs");
        }
        let after = state.closure();
        // Stage one: the §30 bridge from below — the executable bar
        // meets the proof bar.
        let w = after.exec.as_ref().expect("extraction installed a witness");
        assert_eq!(
            w.value, after.bar,
            "the §30 bridge: B_exec = B_proof after extraction"
        );
        assert!(
            after.certified_regret <= before.certified_regret,
            "the §31 monotone law across the producer run"
        );
        if after.certified_regret > BigRational::zero() {
            loose_upper_seen = true;
        }
        // Stage two: price the upper side — the §36 exact upper facts
        // (each action's exact best-response value bounds it from
        // above; test-local authority, the way a §36 producer would).
        for a in legal_roots(&root, &position) {
            let child = belief.focal_play(a);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            let z = oracle.mass(&child);
            let value = BigRational::new(BigInt::from(exact), BigInt::from(z));
            state
                .install(
                    &identity,
                    Fact::Bound(BoundFact::upper(
                        a,
                        value,
                        "response-exact-upper-v1",
                        ProofTag::Deterministic,
                    )),
                )
                .expect("an exact upper installs");
        }
        let priced = state.closure();
        assert_eq!(
            priced.certified_regret,
            BigRational::zero(),
            "with the upper side priced, extraction closes the gap exactly: \
             B_exec = U* = Q*"
        );
        let rec = state.recommend().expect("an executable witness exists");
        // The witness rule keeps the FIRST holder on ties, so the
        // extraction id shows exactly where the executable bar rose
        // STRICTLY — a before-gap alone is not enough (a vacuous
        // winner-upper makes the regret positive even when the σ0
        // baseline is already optimal, and the baseline then keeps
        // the witness on the tie).
        let baseline = before
            .exec
            .as_ref()
            .expect("the σ0 profiles install a witness")
            .value
            .clone();
        if baseline < w.value {
            improved_seen = true;
            assert!(
                rec.policy.starts_with("profile:argmax-full-legal-"),
                "the recommendation's policy is the extracted content id"
            );
        }
        assert_eq!(
            rec.certified_regret,
            BigRational::zero(),
            "the recommendation carries the collapsed regret"
        );
    }
    assert!(
        gap_seen,
        "somewhere the baseline left a strictly positive regret, so the \
         collapse is not vacuous"
    );
    assert!(
        loose_upper_seen,
        "somewhere a settled root kept the winner's vacuous upper (h4-t6), \
         so stage two is not vacuous either"
    );
    assert!(
        improved_seen,
        "somewhere extraction strictly beat the σ0 baseline and took the witness"
    );
}
