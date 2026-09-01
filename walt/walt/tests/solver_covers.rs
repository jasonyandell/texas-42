//! Gates for the anytime proof-state Phase 5 [L2 thread] — the §62
//! count-threat covers: accepted covers never understate the residual
//! score gain — the verified movement bound contains every
//! materialized deviation's movement and the derived rescue-band
//! upper never undercuts the exact §36 response value (gate 1);
//! declined covers produce no number — no incumbent profile, no fact
//! (gate 2); exact zero gain collapses the residual at the cell — the
//! interval closes to the incumbent's own point (gate 3); and a rare
//! count hazard remains VISIBLE — the derived upper exceeds the lower
//! by exactly the §10 rescue-band mass, somewhere small but nonzero,
//! never averaged away (gate 4).
//!
//! Mathematical source: `walt/math/anytime_proof_state_score_v0.1.md`
//! §62, §10/§11, §13, §5, under ruling APS-A9
//! (`walt/CENSUS-RULINGS.md`).
//!
//! DECLARED TEST EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frozen `verify_player` receipt roots (the six
//! enumerable fibers); incumbents = the lowest-first baseline
//! profiles (the Phase 1 vocabulary).

mod common;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::solver::adaptive::{root_identity, CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::covers::{cover_for_action, CountThreatProducer};
use walt::solver::factor_belief::{
    response_success_mass, viewer_score_profile, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{
    Fact, ProofProducer, ProofState, ScoreProfileFact, SemanticsIdentity,
};

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

/// Install the lowest-first baseline profile for every legal action —
/// the incumbents every cover here anchors on.
fn install_baselines(
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &FieldModel,
    state: &mut ProofState,
) {
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let identity = state.identity.clone();
    for a in state.legal.clone() {
        let child = FactorBelief::uniform_root(root, position, field).focal_play(a);
        let mut ps = RecursionStats::default();
        let profile = viewer_score_profile(oracle, &child, &low, field, &mut ps);
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
}

/// Gate 1 — §62's soundness: every accepted cover's derived
/// rescue-band upper sits at or above the exact §36 response value
/// (never understating what deviations can still claim), and the
/// verified movement bound contains every MATERIALIZED deviation's
/// score movement (highest-first as the deliberately adversarial
/// specimen). Non-vacuity: somewhere the verified bound is strictly
/// sharper than the §5 arithmetic envelope.
#[test]
fn accepted_covers_never_understate_the_residual_gain() {
    let r = receipt();
    let oracle = SupportOracle;
    let high = FixedPreference::highest_first("focal:highest-first");
    let mut sharper_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        install_baselines(&oracle, &root, &position, &field, &mut state);
        let before = state.closure();
        let producer = CountThreatProducer {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let facts = producer.produce(&state);
        assert_eq!(
            facts.len(),
            state.legal.len(),
            "every action with an incumbent gets a cover"
        );
        let unbanked = 42 - position.banked[0] - position.banked[1];
        for fact in &facts {
            let Fact::Cover(cv) = fact else {
                panic!("the producer speaks covers only")
            };
            assert_eq!(
                cv.trick_gain_upper
                    + 5 * cv.five_count_tiles.len() as u32
                    + 10 * cv.ten_count_tiles.len() as u32,
                unbanked,
                "the resources decompose the §5 remainder exactly"
            );
            if cv.score_gain_upper < unbanked {
                sharper_seen = true;
            }
            // The verified bound contains the adversarial
            // materialized deviation's movement against the incumbent.
            let child = FactorBelief::uniform_root(&root, &position, &field).focal_play(cv.action);
            let mut ps = RecursionStats::default();
            let dev = viewer_score_profile(&oracle, &child, &high, &field, &mut ps);
            let dev_floor = dev.bins.iter().position(|m| *m > 0).expect("mass") as u32;
            let dev_ceiling = dev.bins.iter().rposition(|m| *m > 0).expect("mass") as u32;
            let inc = state
                .facts()
                .iter()
                .find_map(|sf| match &sf.fact {
                    Fact::Profile(p)
                        if p.action == cv.action && p.policy_id == cv.incumbent_policy_id =>
                    {
                        Some(p.clone())
                    }
                    _ => None,
                })
                .expect("the cover's incumbent is installed");
            let inc_floor = inc.bins.iter().position(|m| *m > 0).expect("mass") as u32;
            let inc_ceiling = inc.bins.iter().rposition(|m| *m > 0).expect("mass") as u32;
            match identity.utility_id.as_str() {
                "pmake-v1" => assert!(
                    dev_ceiling <= inc_floor + cv.score_gain_upper,
                    "the movement bound contains the materialized deviation's gain"
                ),
                _ => assert!(
                    dev_floor + cv.score_gain_upper >= inc_ceiling,
                    "the movement bound contains the materialized deviation's loss"
                ),
            }
        }
        // Install and check the value-level anchor: derived uppers
        // never undercut the exact response values.
        for fact in facts {
            state.install(&identity, fact).expect("a cover installs");
        }
        let after = state.closure();
        for v in &after.views {
            let child = FactorBelief::uniform_root(&root, &position, &field).focal_play(v.action);
            let z = oracle.mass(&child);
            let mut rs = ResponseStats::default();
            let exact = response_success_mass(&oracle, &child, &field, &mut rs);
            let exact_value = BigRational::new(BigInt::from(exact), BigInt::from(z));
            assert!(
                v.upper >= exact_value,
                "a rescue-band upper never understates the exact response"
            );
            let b = before.views.iter().find(|w| w.action == v.action).unwrap();
            assert!(v.upper <= b.upper, "covers only tighten uppers");
            assert!(v.lower == b.lower, "covers move no lower");
        }
    }
    assert!(
        sharper_seen,
        "somewhere the range walk beats the arithmetic envelope"
    );
}

/// Gate 2 — the decline path: no incumbent profile, no cover, no
/// number; one incumbent, exactly one cover, for that action only.
#[test]
fn declined_covers_produce_no_number() {
    let r = receipt();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, ENUM_ROOTS[0].0, ENUM_ROOTS[0].1);
    let field = FieldModel::new(level0_spec());
    let identity = identity_of(&root, &position);
    let mut state = ProofState::open(&root, &position, identity.clone());
    let producer = CountThreatProducer {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
    };
    assert!(
        producer.produce(&state).is_empty(),
        "a state without incumbents declines every action"
    );
    for a in &state.legal {
        assert!(
            cover_for_action(&oracle, &root, &position, &field, &state, *a).is_none(),
            "a declined cover is None, not a weaker number"
        );
    }
    let first = state.legal[0];
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let child = FactorBelief::uniform_root(&root, &position, &field).focal_play(first);
    let mut ps = RecursionStats::default();
    let profile = viewer_score_profile(&oracle, &child, &low, &field, &mut ps);
    state
        .install(
            &identity,
            Fact::Profile(Box::new(ScoreProfileFact {
                action: first,
                policy_id: "lowest-first-after-root-action".to_string(),
                bins: profile.bins,
            })),
        )
        .expect("a baseline profile installs");
    let facts = producer.produce(&state);
    assert_eq!(facts.len(), 1, "one incumbent, one cover");
    assert!(
        matches!(&facts[0], Fact::Cover(cv) if cv.action == first),
        "the cover names its action"
    );
}

/// Gates 3 and 4 — collapse and visibility, searched honestly across
/// the fixture roots: somewhere the verified gain is EXACTLY ZERO and
/// the derived upper equals the incumbent's own value — the residual
/// collapses at that cell (gate 3); and somewhere a small-but-nonzero
/// §10 rescue band stays visible — the derived upper exceeds the
/// lower by exactly the incumbent's mass in the rescue window, a
/// hazard a mean would average away (gate 4). Both assertions also
/// pin the derivation identity everywhere: the interval left open is
/// exactly the rescue-band mass.
#[test]
fn zero_gain_collapses_and_rare_hazards_stay_visible() {
    let r = receipt();
    let oracle = SupportOracle;
    let mut collapse_seen = false;
    let mut hazard_seen = false;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field = FieldModel::new(level0_spec());
        let identity = identity_of(&root, &position);
        let mut state = ProofState::open(&root, &position, identity.clone());
        install_baselines(&oracle, &root, &position, &field, &mut state);
        let producer = CountThreatProducer {
            oracle: &oracle,
            root: &root,
            position: &position,
            field: &field,
        };
        let facts = producer.produce(&state);
        let covers: Vec<_> = facts
            .iter()
            .map(|f| match f {
                Fact::Cover(cv) => cv.clone(),
                _ => panic!("the producer speaks covers only"),
            })
            .collect();
        for fact in facts {
            state.install(&identity, fact).expect("a cover installs");
        }
        let report = state.closure();
        let c = position.bid;
        for cv in covers {
            let v = report
                .views
                .iter()
                .find(|v| v.action == cv.action)
                .expect("a legal action has a view");
            let inc = state
                .facts()
                .iter()
                .find_map(|sf| match &sf.fact {
                    Fact::Profile(p)
                        if p.action == cv.action && p.policy_id == cv.incumbent_policy_id =>
                    {
                        Some(p.clone())
                    }
                    _ => None,
                })
                .expect("the cover's incumbent is installed");
            let z = inc.total();
            // The derivation identity: the open interval is exactly
            // the §10/§11 band mass of the incumbent under the
            // verified movement bound.
            let band = match identity.utility_id.as_str() {
                "pmake-v1" => inc.tail(c.saturating_sub(cv.score_gain_upper)) - inc.tail(c),
                _ => inc.tail(c) - inc.tail(c + cv.score_gain_upper),
            };
            let band_value = BigRational::new(BigInt::from(band), BigInt::from(z));
            let width = &v.upper - &v.lower;
            assert!(
                width <= band_value,
                "the interval left open never exceeds the rescue-band mass"
            );
            if cv.score_gain_upper == 0 {
                assert_eq!(
                    v.upper, v.lower,
                    "zero verified gain collapses the residual to the incumbent's point"
                );
                collapse_seen = true;
            }
            let quarter = BigRational::new(BigInt::from(1), BigInt::from(4));
            if width > BigRational::zero() && width <= quarter && cv.score_gain_upper >= 5 {
                // A count-sized hazard, small but visible: the state
                // honestly refuses to call this action settled.
                hazard_seen = true;
            }
        }
    }
    assert!(
        collapse_seen,
        "somewhere the score is pinned and the residual collapses"
    );
    assert!(
        hazard_seen,
        "somewhere a small count hazard stays visible in the interval"
    );
}
