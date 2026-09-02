//! Gates for the God-gap census [L2 thread] — slice U0 of the
//! salvation-complex program (`solver::godgap`): the §8 three-part
//! failure decomposition made mechanical, the four §48 result types,
//! and the §38 fusion horizon as a measured object.
//!
//! G1 re-derives the salvation parent's §9 fourteen-coordinate table
//! from the COMMITTED doom record — the per-world truth column of
//! `probes/factor_belief/doomreport_run1.txt` is parsed out of the
//! file and every figure recomputed, with `d_info = 0` asserted by
//! exact rational equality `Q = 1 − doomed/Z` on all fourteen (SC-A2:
//! the table stops being a quoted number and becomes a checked one).
//! The truth-vs-census divergences are asserted in the same gate, so
//! none can drift unnoticed — and the mechanical count came back
//! THREE (h4-t6 0-0: 56 against truth 60; h8-t5 0-0: 17 against 21;
//! h8-t5 5-3: 0 against 1) where the intake companion names two. The
//! §9 table itself is unaffected (it cites the truth column
//! throughout); the companion's count of divergence POINTS is one
//! short, recorded in `walt/DISCREPANCIES.md`. G2 holds the SC-A4 result
//! typing: zero certified doom with no exact `Q` is `UnknownGodGap`,
//! positive certified doom with no exact `Q` is `GodUpper`, and
//! `PositiveGodGap` only ever exists with its exact witness. G3
//! re-prices every extracted God-tight policy through the independent
//! fixed-policy evaluator and checks that the equality receipt binds
//! root, field, contract, and belief — then installs it and reads the
//! equality back off the proof-state closure. G4 holds the §34/§35
//! refusal discipline: an unaffordable coordinate comes back typed,
//! never dropped and never upgraded into a claim. G5 is §47's
//! preservation gate: the census CONSUMES `solver::doom` and never
//! modifies it — same uppers, same purity, both producers coexisting
//! in one store.
//!
//! Mathematical source: `walt/math/salvation_complex_v0.1.md` §4–§9,
//! §36–§40, §47–§48, §55; the governing intake companion
//! `walt/math/salvation_complex_v0.1_intake.md`; rulings SC-A1..A8.
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frozen `verify_player` receipt; the six
//! enumerable t5/t6 roots of the §9 table plus h8-t4 (fiber 1200) as
//! the positive-gap specimen; doom spec ample (n = 10_000_000,
//! c = 1_000_000, level 3, full descent, critical set empty).

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::DominoSet;
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition, SlicePolicy};
use walt::solver::doom::{doom_census, doom_enumeration, DoomCensusProducer, DoomSpec};
use walt::solver::factor_belief::{
    extract_success_policy, response_success_mass, viewer_success_mass, ExactCoverOracle,
    ExtractionSource, FactorBelief, RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::godgap::{
    coordinate_facts, earliest_fusion_free_trick, fusion_horizon, legal_actions, DoomSource,
    GodGapCoordinate, GodGapProducer, GodGapResult, GodGapSpec, GodGapWalk, Refusal,
};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{BoundSide, Fact, ProofState, ProofTag, SemanticsIdentity};

/// The committed doom record this suite re-derives (SC-A2). Parsed,
/// never quoted: every number below the `PART 1` header is read out of
/// the file and recomputed by the census.
const DOOM_RECORD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../probes/factor_belief/doomreport_run1.txt"
));

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

fn ample_doom() -> DoomSpec {
    DoomSpec {
        node_budget: 10_000_000,
        walk_cap: 1_000_000,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    }
}

/// The census spec used by every gate that wants the exact side: caps
/// above the enumerable corpus's largest fiber, doom ample.
fn ample_spec() -> GodGapSpec {
    GodGapSpec {
        exact_fiber_cap: 40_000,
        profile_fiber_cap: 12_000,
        doom: ample_doom(),
    }
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn zero() -> BigRational {
    BigRational::from_integer(BigInt::from(0))
}

/// One parsed row of the committed record's PART 1: the root label,
/// the action, the fiber, what the CLASS census certified, and the
/// per-world TRUTH.
#[derive(Debug, PartialEq, Eq)]
struct RecordRow {
    root: String,
    action: String,
    fiber: u128,
    census_doomed: u128,
    truth: u128,
}

/// Parse the committed doom record's PART 1 block. Deliberately
/// literal: a format drift in the probe output must fail this parse
/// loudly rather than silently produce an empty table.
fn parse_doom_record() -> Vec<RecordRow> {
    let mut rows = Vec::new();
    let mut current: Option<(String, u128)> = None;
    let mut in_part1 = false;
    for line in DOOM_RECORD.lines() {
        if line.contains("#### PART 1") {
            in_part1 = true;
            continue;
        }
        if line.contains("#### PART 2") {
            break;
        }
        if !in_part1 {
            continue;
        }
        if let Some(rest) = line.strip_suffix("):") {
            // `h12-t6 (fiber 6):`
            let (label, fiber) = rest.split_once(" (fiber ").expect("a root header");
            current = Some((
                label.to_string(),
                fiber.parse().expect("a fiber mass parses"),
            ));
            continue;
        }
        let Some(body) = line.strip_prefix("  ") else {
            continue;
        };
        let Some((action, rest)) = body.split_once(": census doomed ") else {
            continue;
        };
        let (root, fiber) = current
            .clone()
            .expect("a coordinate follows its root header");
        let (census_doomed, rest) = rest.split_once(" of ").expect("a census mass");
        let rest = rest
            .split_once("| per-world truth ")
            .expect("a truth column")
            .1;
        let truth = rest.split_once(' ').expect("a truth mass").0;
        rows.push(RecordRow {
            root,
            action: action.to_string(),
            fiber,
            census_doomed: census_doomed.parse().expect("a census mass parses"),
            truth: truth.parse().expect("a truth mass parses"),
        });
    }
    rows
}

/// Gate 1 (SC-A2, binding) — the §9 fourteen-coordinate table,
/// re-derived mechanically. For every coordinate the committed record
/// names: the census's God upper comes from the per-world truth and
/// EQUALS the committed truth column; the exact information-consistent
/// optimum `Q` satisfies `Q = 1 − doomed/Z` by exact rational
/// equality, so `d_info = 0`; and every coordinate is therefore
/// `GodTightPolicy`. The class census's own (weaker) harvest is
/// re-derived in the same pass, which pins the intake companion's two
/// recorded divergences.
#[test]
fn the_section_nine_table_is_re_derived_from_the_committed_record() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let rows = parse_doom_record();
    assert_eq!(
        rows.len(),
        14,
        "the §9 table is fourteen action coordinates (2+2+2+2+3+3)"
    );
    let spec = ample_spec();
    let mut checked = 0usize;
    let mut divergences: Vec<(String, String, u128, u128)> = Vec::new();
    for (hand_id, trick_no) in [(12usize, 6usize), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)] {
        let label = format!("h{hand_id}-t{trick_no}");
        let (root, position) = root_at(&r, hand_id, trick_no);
        let walk = GodGapWalk {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            spec: &spec,
        };
        let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
        let census = walk.census(&mut progress);
        let expected: Vec<&RecordRow> = rows.iter().filter(|row| row.root == label).collect();
        assert_eq!(
            census.len(),
            expected.len(),
            "{label}: the census walks exactly the coordinates the record names"
        );
        for (coordinate, row) in census.iter().zip(expected) {
            assert_eq!(
                format!("{}", coordinate.context.root_action),
                row.action,
                "{label}: the census walks the record's actions in the record's order"
            );
            assert_eq!(coordinate.fiber_mass, row.fiber, "{label} {}", row.action);
            assert!(
                matches!(coordinate.upper.source, DoomSource::PerWorldTruth { .. }),
                "{label} {}: an enumerable coordinate's upper is the per-world truth",
                row.action
            );
            assert_eq!(
                coordinate.upper.doomed_mass, row.truth,
                "{label} {}: the re-derived doom truth is the committed one",
                row.action
            );
            // The heart of the gate: d_info = 0 by exact rational
            // equality, so Q = 1 − doomed/Z on the nose.
            let q_expected =
                BigRational::new(BigInt::from(row.fiber - row.truth), BigInt::from(row.fiber));
            assert_eq!(
                coordinate.decomposition.d_info,
                Some(zero()),
                "{label} {}: the §9 table's d_info = 0",
                row.action
            );
            assert_eq!(
                coordinate.upper.value, q_expected,
                "{label} {}: U^God = 1 − doomed/Z",
                row.action
            );
            let tight = coordinate.god_tight().unwrap_or_else(|| {
                panic!(
                    "{label} {}: a d_info = 0 coordinate is God-tight — \
                     got {}",
                    row.action,
                    coordinate.result.label()
                )
            });
            assert_eq!(tight.value, q_expected, "{label} {}", row.action);
            assert_eq!(tight.god_upper, q_expected, "{label} {}", row.action);
            assert_eq!(
                coordinate.decomposition.d_policy,
                Some(zero()),
                "{label} {}: the extracted incumbent IS the optimum",
                row.action
            );
            // The class census's weaker harvest, re-derived beside the
            // truth — the intake companion's divergence points.
            let class = doom_census(
                &oracle,
                &root,
                &position,
                &field,
                coordinate.context.root_action,
                &ample_doom(),
            );
            assert_eq!(
                class.doomed_mass, row.census_doomed,
                "{label} {}: the class census's committed harvest",
                row.action
            );
            assert!(
                class.doomed_mass <= row.truth,
                "{label} {}: a certified harvest never exceeds the truth",
                row.action
            );
            if class.doomed_mass != row.truth {
                divergences.push((
                    label.clone(),
                    row.action.clone(),
                    class.doomed_mass,
                    row.truth,
                ));
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 14, "all fourteen coordinates re-derived");
    // THE MECHANICAL COUNT IS THREE, NOT TWO. The intake companion
    // names two truth-vs-census divergence points (h4-t6 0-0 and
    // h8-t5 5-3); the committed record carries a third, h8-t5 0-0,
    // where the class census certified 17 of the 21 truly doomed
    // worlds (809‰ recovery, printed in the record's own recovery
    // column). Nothing in the §9 table moves — it cites the TRUTH
    // column at every coordinate, and every d_info = 0 above is
    // unaffected — but the companion's count of divergence points is
    // one short, and this gate is where that stops being invisible
    // (recorded in `walt/DISCREPANCIES.md`).
    assert_eq!(
        divergences,
        vec![
            ("h4-t6".to_string(), "0-0".to_string(), 56, 60),
            ("h8-t5".to_string(), "0-0".to_string(), 17, 21),
            ("h8-t5".to_string(), "5-3".to_string(), 0, 1),
        ],
        "the three truth-vs-census divergences the committed record actually carries"
    );
}

/// Gate 2 (SC-A4) — the result typing. Three claims, one per honest
/// type: with the exact side capped out, a zero-doom coordinate is
/// `UnknownGodGap` (NEVER `PositiveGodGap` — a census that found no
/// counterexamples has not found a gap); a certified-doom coordinate
/// is `GodUpper`, its upper real and its gap unmeasured; and where the
/// exact side runs, a `PositiveGodGap` carries an exact `Q` witness
/// that an independent call to the response recursion reproduces.
#[test]
fn the_four_result_types_are_typed_by_what_was_actually_established() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let capped = GodGapSpec {
        exact_fiber_cap: 0,
        profile_fiber_cap: 0,
        doom: ample_doom(),
    };

    // (a) h3-t5: the class census certifies nothing here, so with no
    // exact Q the honest verdict is UnknownGodGap.
    let (root, position) = root_at(&r, 3, 5);
    let walk = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &capped,
    };
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    for coordinate in walk.census(&mut progress) {
        assert_eq!(coordinate.upper.doomed_mass, 0, "h3-t5 certifies no doom");
        assert!(coordinate.upper.vacuous(), "the upper is the vacuous 1");
        assert_eq!(
            coordinate.result.label(),
            "UnknownGodGap",
            "zero certified doom with no exact Q is never a gap (SC-A4)"
        );
        assert_eq!(coordinate.decomposition.d_info, None, "an absent number");
        assert_eq!(coordinate.decomposition.d_policy, None, "an absent number");
        assert!(
            coordinate.upper.fact().is_none(),
            "a vacuous upper installs nothing"
        );
    }

    // (b) h12-t6: the whole fiber is doomed, so even with no exact Q
    // the census carries a real deterministic upper.
    let (root, position) = root_at(&r, 12, 6);
    let walk = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &capped,
    };
    for coordinate in walk.census(&mut progress) {
        assert_eq!(coordinate.upper.doomed_mass, coordinate.fiber_mass);
        assert_eq!(
            coordinate.result.label(),
            "GodUpper",
            "certified doom with no exact Q is a standing upper, gap unknown"
        );
        assert_eq!(coordinate.upper.value, zero());
        assert_eq!(coordinate.decomposition.d_info, None);
        assert!(
            coordinate.upper.fact().is_some(),
            "a nonvacuous upper is installable"
        );
    }

    // (c) h8-t4 2-1: the positive-gap specimen. The witness is
    // mandatory and independently reproducible.
    let (root, position) = root_at(&r, 8, 4);
    let spec = ample_spec();
    let walk = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &spec,
    };
    let action = legal_actions(&root, &position)[0];
    let coordinate = walk.god_gap(action, &mut progress);
    let GodGapResult::PositiveGodGap(gap) = &coordinate.result else {
        panic!(
            "h8-t4 {action} is the declared positive-gap specimen, got {}",
            coordinate.result.label()
        );
    };
    assert!(gap.gap > zero(), "a positive gap is positive");
    assert_eq!(
        gap.gap,
        &coordinate.upper.value - &gap.q,
        "Φ = U^God − Q, exactly"
    );
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut stats = ResponseStats::default();
    let independent =
        response_success_mass(&oracle, &belief.focal_play(action), &field, &mut stats);
    assert_eq!(
        gap.q_mass, independent,
        "the exact witness is the response recursion's own value"
    );
    assert_eq!(
        coordinate.decomposition.d_info,
        Some(gap.gap.clone()),
        "the decomposition's second term IS the God gap"
    );
    assert!(
        coordinate.god_tight().is_none(),
        "a positive gap is not God-tight"
    );
}

/// Gate 3 (§36) — the God-tight receipts. Every extracted God-tight
/// policy is re-priced OUTSIDE the census, through the independent
/// fixed-policy evaluator, and must land exactly on the God upper; the
/// receipt binds root, field, contract, and belief; and installing the
/// coordinate's facts makes the closure itself show the executable
/// lower meeting the deterministic upper at that action.
#[test]
fn god_tight_policies_re_price_to_the_god_upper_under_a_bound_receipt() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let spec = ample_spec();
    let mut receipts = 0usize;
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let identity = identity_of(&root, &position);
        let walk = GodGapWalk {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            spec: &spec,
        };
        let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let z = oracle.mass(&belief);
        for coordinate in walk.census(&mut progress) {
            let Some(tight) = coordinate.god_tight() else {
                continue;
            };
            let action = coordinate.context.root_action;
            // The independent re-pricing: extract again outside the
            // census and evaluate the frozen policy with the
            // fixed-policy walk (a different recursion from the max
            // the extraction came out of).
            let child = belief.focal_play(action);
            let mut estats = ResponseStats::default();
            let (mass, policy) = extract_success_policy(
                &oracle,
                &child,
                &ExtractionSource::FullLegal,
                &field,
                &mut estats,
            );
            assert_eq!(
                policy.id(),
                tight.policy_id,
                "h{hand_id}-t{trick_no} {action}: the receipt names the extracted policy"
            );
            let mut pstats = RecursionStats::default();
            let repriced = viewer_success_mass(&oracle, &child, &policy, &field, &mut pstats);
            assert_eq!(repriced, mass, "extraction and re-pricing agree");
            let value = BigRational::new(BigInt::from(repriced), BigInt::from(z));
            assert_eq!(
                value, tight.god_upper,
                "h{hand_id}-t{trick_no} {action}: the executable lower MEETS the doom upper"
            );
            // The receipt's identity binding (§36's third requirement).
            assert_eq!(tight.context.root_id, root_identity(&root, &position));
            assert_eq!(tight.context.field_id, SlicePolicy::id(&field));
            assert_eq!(tight.context.contract, position.bid);
            assert_eq!(tight.context.root_action, action);
            assert_eq!(tight.equality_receipt.belief_id, "uniform-root");
            assert_eq!(tight.equality_receipt.utility_id, identity.utility_id);
            assert_eq!(tight.equality_receipt.fiber_mass, z);
            assert_eq!(tight.equality_receipt.repriced_mass, repriced);
            assert_eq!(
                tight.equality_receipt.doomed_mass + repriced,
                z,
                "God-tightness is exactly: every saveable world saved"
            );
            // Persistence: the facts install and the closure shows the
            // equality without any further work.
            if coordinate.upper.vacuous() {
                // Nothing to install on the upper side (Q = 1 here);
                // the profile still enters as the executable lower.
                assert_eq!(tight.god_upper, BigRational::from_integer(BigInt::from(1)));
            }
            let mut state = ProofState::open(&root, &position, identity.clone());
            for fact in coordinate_facts(&coordinate) {
                state.install(&identity, fact).expect("a God fact lands");
            }
            let closure = state.closure();
            let view = closure
                .views
                .iter()
                .find(|v| v.action == action)
                .expect("the action has a view");
            assert_eq!(
                view.lower, tight.value,
                "h{hand_id}-t{trick_no} {action}: the closure's executable lower"
            );
            if !coordinate.upper.vacuous() {
                assert_eq!(
                    view.upper, tight.god_upper,
                    "h{hand_id}-t{trick_no} {action}: the closure's deterministic upper"
                );
                assert_eq!(view.lower, view.upper, "lower meets upper — God-tightness");
            }
            assert!(
                !view.lower_sampled && !view.upper_sampled,
                "nothing sampled"
            );
            receipts += 1;
        }
    }
    assert_eq!(
        receipts, 7,
        "h5-t6 (2) + h4-t6 (2) + h8-t5 (3) are all God-tight in the §9 table"
    );
}

/// Gate 4 (§34/§35) — refusals are typed and honest. Under a cap that
/// forbids every exact instrument, the census still returns one
/// coordinate per legal action, each carrying the typed reasons it
/// could not go further, with no term of the decomposition invented
/// and no result type upgraded. Raising the cap on the same root
/// removes exactly those refusals — the refusal is a function of the
/// declared budget, not of chance.
#[test]
fn census_refusals_are_typed_and_nothing_is_silently_dropped() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let (root, position) = root_at(&r, 3, 5);
    let actions = legal_actions(&root, &position);
    let starved = GodGapSpec {
        exact_fiber_cap: 0,
        profile_fiber_cap: 0,
        doom: DoomSpec {
            node_budget: 40,
            walk_cap: 20,
            max_level: 3,
            critical: DominoSet::EMPTY,
            descend_top: None,
        },
    };
    let walk = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &starved,
    };
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    let census = walk.census(&mut progress);
    assert_eq!(
        census.len(),
        actions.len(),
        "one coordinate per legal action — nothing is dropped"
    );
    let mut census_refused_somewhere = false;
    for (coordinate, action) in census.iter().zip(&actions) {
        assert_eq!(coordinate.context.root_action, *action);
        assert!(
            !coordinate.refusals.is_empty(),
            "{action}: a starved coordinate states why it stopped"
        );
        assert!(
            coordinate
                .refusals
                .contains(&Refusal::ExactValueUnaffordable {
                    fiber: coordinate.fiber_mass,
                    cap: 0
                }),
            "{action}: the exact value is refused by the declared cap, by name"
        );
        assert!(
            coordinate
                .refusals
                .contains(&Refusal::DoomTruthUnaffordable {
                    fiber: coordinate.fiber_mass,
                    cap: 0
                }),
            "{action}: the per-world truth is refused by the declared cap, by name"
        );
        assert_eq!(coordinate.decomposition.d_info, None);
        assert_eq!(coordinate.decomposition.d_policy, None);
        assert!(
            matches!(
                coordinate.result,
                GodGapResult::GodUpper | GodGapResult::UnknownGodGap
            ),
            "{action}: a refused coordinate never claims a gap"
        );
        census_refused_somewhere |= coordinate
            .refusals
            .iter()
            .any(|f| matches!(f, Refusal::CensusLeftMassRefused { .. }));
        assert!(
            matches!(coordinate.upper.source, DoomSource::CertifiedCensus { .. }),
            "{action}: the class census stands in when the truth is unaffordable"
        );
    }
    assert!(
        census_refused_somewhere,
        "a 40-node doom budget on a 200-world fiber leaves mass unwalked, and says so"
    );
    // The same root, affordable: the exact refusals are gone and the
    // coordinates carry their numbers.
    let ample = ample_spec();
    let walk = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &ample,
    };
    for coordinate in walk.census(&mut progress) {
        assert!(
            !coordinate
                .refusals
                .iter()
                .any(|f| matches!(f, Refusal::ExactValueUnaffordable { .. })),
            "an affordable coordinate refuses nothing on the exact side"
        );
        assert!(coordinate.decomposition.d_info.is_some());
        assert!(coordinate.decomposition.d_policy.is_some());
    }
}

/// Gate 5 (§47 / SC-A3) — doom is preserved, consumed and never
/// modified. The census's God upper is exactly `solver::doom`'s own
/// number when called directly; running the census leaves doom's
/// instruments returning exactly what they returned before (they are
/// pure functions of their declared inputs); and both producers
/// install into ONE store side by side, the God-gap producer adding
/// the God-tight executable lower that the doom producer, being an
/// upper-only instrument, never had.
#[test]
fn the_census_consumes_doom_and_never_modifies_it() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let (root, position) = root_at(&r, 5, 6);
    let identity = identity_of(&root, &position);
    let spec = ample_spec();
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    for action in legal_actions(&root, &position) {
        let before = doom_enumeration(
            &oracle,
            &root,
            &position,
            &field,
            action,
            &ample_doom(),
            &mut progress,
        );
        let walk = GodGapWalk {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            spec: &spec,
        };
        let coordinate = walk.god_gap(action, &mut progress);
        let after = doom_enumeration(
            &oracle,
            &root,
            &position,
            &field,
            action,
            &ample_doom(),
            &mut progress,
        );
        assert_eq!(
            before, after,
            "doom's instrument is unchanged by the census"
        );
        assert_eq!(
            coordinate.upper.doomed_mass, before.doomed,
            "the God upper's doomed mass IS doom's"
        );
        assert_eq!(
            coordinate.upper.value, before.upper,
            "the God upper IS doom's deterministic upper"
        );
    }
    // Both producers, one store. The doom producer's uppers and the
    // God-gap producer's facts coexist; the God-tight lower is what
    // the census adds.
    let mut state = ProofState::open(&root, &position, identity.clone());
    let doom_producer = DoomCensusProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: ample_doom(),
    };
    let doom_results = state.run_producer(&doom_producer);
    assert!(
        doom_results.iter().all(Result::is_ok),
        "the doom producer still installs"
    );
    let doom_facts = state.facts().len();
    assert!(doom_facts > 0, "h5-t6 has certified doom to install");
    let god_producer = GodGapProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: spec.clone(),
    };
    let god_results = state.run_producer(&god_producer);
    assert!(
        god_results.iter().all(Result::is_ok),
        "the God-gap producer installs beside it"
    );
    assert!(
        state.facts().len() > doom_facts,
        "the census adds facts and removes none"
    );
    for sf in state.facts() {
        if let Fact::Bound(b) = &sf.fact {
            assert_eq!(b.proof, ProofTag::Deterministic, "nothing sampled exists");
            assert_eq!(b.side, BoundSide::Upper, "both producers speak in uppers");
        }
    }
    let closure = state.closure();
    assert!(
        closure.exec.is_some(),
        "the God-tight profile gives the store an executable bar the doom census had none of"
    );
    assert_eq!(
        closure.certified_regret,
        zero(),
        "at a God-tight root the executable bar meets the deterministic upper: Γ = 0"
    );
    let again = state.run_producer(&god_producer);
    assert!(again.is_empty(), "an identical fact is proposed once");
    let bytes = state.serialize();
    let parsed = ProofState::parse(&bytes, &root, &position).expect("a lawful state");
    assert_eq!(parsed.serialize(), bytes, "the §67.4 byte round trip");
}

/// Gate 6 (§38) — the fusion-horizon table is an honest stratification:
/// it counts every coordinate handed to it, names every exception,
/// separates DEGENERATE God-tightness (whole-fiber doom, where every
/// policy is God-tight and the equality says nothing) from the
/// substantive kind, and reports the earliest fusion-free depth only
/// when every deeper stratum is fusion-free too. Run over the census's
/// own t4 and t6 specimens, where the answer differs by depth: h8-t4
/// carries four measured information-consistency prices, h12-t4 is
/// God-tight in the degenerate way only, and h5-t6 is substantively
/// fusion-free.
#[test]
fn the_fusion_horizon_table_counts_every_coordinate_and_names_its_exceptions() {
    let oracle = SupportOracle;
    let field = FieldModel::new(level0_spec());
    let r = receipt();
    let spec = ample_spec();
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    let mut entries: Vec<(usize, String, GodGapCoordinate)> = Vec::new();
    for (hand_id, trick_no) in [(8usize, 4usize), (12, 4), (5, 6)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let walk = GodGapWalk {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
            spec: &spec,
        };
        for coordinate in walk.census(&mut progress) {
            entries.push((trick_no, format!("h{hand_id}-t{trick_no}"), coordinate));
        }
    }
    let strata = fusion_horizon(&entries);
    assert_eq!(strata.len(), 2, "two depths tested");
    assert_eq!(strata[0].trick, 4, "increasing depth order");
    assert_eq!(strata[1].trick, 6);
    let tested: usize = strata.iter().map(|s| s.tested).sum();
    assert_eq!(tested, entries.len(), "every coordinate is counted once");
    for s in &strata {
        assert_eq!(
            s.tested,
            s.god_tight + s.positive_gap + s.god_upper_only + s.unknown,
            "the stratum's tallies partition its coordinates"
        );
        assert_eq!(
            s.exceptions.len(),
            s.tested - s.god_tight,
            "every non-God-tight coordinate is named"
        );
    }
    assert!(
        !strata[0].fusion_free(),
        "t4 carries a real information-consistency price on this corpus"
    );
    assert_eq!(strata[0].positive_gap, 4, "all four h8-t4 actions");
    assert!(strata[0].max_gap.is_some(), "a measured Φ");
    assert_eq!(
        strata[0].god_tight, 4,
        "h12-t4's four actions close at the God upper"
    );
    assert_eq!(
        strata[0].god_tight_vacuous, 4,
        "and every one of them does so on whole-fiber doom — nothing was saveable,          so every policy is God-tight there and the equality carries no information"
    );
    assert!(
        !strata[0].substantively_fusion_free(),
        "a degenerate God-tightness never makes a stratum fusion-free"
    );
    assert!(strata[1].fusion_free(), "t6 is fusion-free here");
    assert_eq!(strata[1].god_tight_vacuous, 0, "h5-t6 has worlds to save");
    assert!(
        strata[1].substantively_fusion_free(),
        "t6's God-tightness is the substantive kind"
    );
    // Some(6), not the ledger's trick 5: this gate's two-stratum sub-corpus
    // (h8-t4, h12-t4, h5-t6) contains no t5 root, so the earliest fusion-free
    // depth IT can see is t6. The full 37-coordinate probe corpus includes the
    // t5 stratum and reads trick 5. Both are corpus-relative measurements.
    assert_eq!(
        earliest_fusion_free_trick(&strata),
        Some(6),
        "the horizon stops at the first exception walking back from the deepest stratum"
    );
}
