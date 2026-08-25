//! Gates for slice 4b [L2 thread; dominance objective-level]: the
//! Hazard-Exclusion Invariant verifier (the single dominance-bound
//! authority), the δ = 0 `StructuralHazardZero` result, the exact benefit
//! exhibit and the PANEL-A7 valid-bound route into `Dominated`, and the
//! One-Round Trump-Extraction witness producer with its typed refusal
//! path.
//!
//! Mathematical source: Part 2 (§§2.1–2.8) and proof ledger P5 of the
//! x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
//! adopted by rulings TRIPLE-A4/A5 (`walt/CENSUS-RULINGS.md`, "The
//! deferred-producers adjudication (2026-08-25)"), under PANEL-A7's
//! dominance vocabulary. Exploratory tier throughout; these are release
//! tests, never receipts.
//!
//! Fixture family: synthetic two- and three-trick endgames built through
//! the live-root bridge (`driven_root`) with void-engineered fibers, so
//! every worked number is exact; plus the fieldswap_cancel parity root
//! h4-t6 with the declared cheap test pair σ0 = Level0{2},
//! σ1 = Level1{2,2}, frozen [2,2] pins — the register of
//! `solver_fieldswap_cancel.rs`, unchanged.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Context, ContextSet, Decl, Domino, DominoSet, Pip, Seat};
use walt::solver::adaptive::{
    driven_root, CanonicalRoot, DrivenState, FixedPreference, RootPosition, SlicePolicy,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{exact_pairwise_masses, CancellationKind};
use walt::solver::hazard::{
    dominance_from_witnessed_hazard_zero, exhibit_benefit_world, one_round_trump_extraction,
    verify_hazard_witness, BranchClaim, ExtractionDecline, WitnessRejection,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

fn tile(a: u8, b: u8) -> Domino {
    Domino::new(Pip::new(a).expect("a pip"), Pip::new(b).expect("a pip"))
}

fn set(tiles: &[Domino]) -> DominoSet {
    let mut out = DominoSet::EMPTY;
    for t in tiles {
        assert!(out.insert(*t), "a fixture hand lists a tile once");
    }
    out
}

fn voids(contexts: &[Context]) -> ContextSet {
    let mut out = ContextSet::EMPTY;
    for q in contexts {
        assert!(out.insert(*q), "a fixture void lists a context once");
    }
    out
}

fn nat(p: u8) -> Context {
    Context::Natural(Pip::new(p).expect("a pip"))
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The declared cheap test field σ0 (the fieldswap_cancel register's).
fn field0_spec() -> FieldSpec {
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

/// σ1 of the declared cheap pair.
fn field1_spec() -> FieldSpec {
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

/// The response §2.6 worked two-trick specimen, realized as an exact 42
/// endgame with a void-engineered TWO-WORLD fiber. Trump is sixes; the
/// viewer (S0, declaring, bid 35 with 28 banked) holds the top trump 6-6
/// and the vulnerable double 1-1; the six hidden tiles are pinned by
/// observed voids so that exactly two deals remain:
///
/// - w0 (the threat world): S3 holds {6-0, 4-0} — one hostile low trump,
///   void in ones, the ruff available;
/// - w1 (the benign world): S3 holds {4-0, 2-0}, the low trump sits with
///   the partner S2 — no hostile trump anywhere.
///
/// Policy a leads 6-6 then 1-1; policy b leads 1-1 then 6-6 — the §2.6
/// H/D exchange verbatim.
fn worked_two_trick_specimen(bid: u32, banked: [u32; 2]) -> (CanonicalRoot, RootPosition) {
    let viewer_hand = set(&[tile(6, 6), tile(1, 1)]);
    let pool = set(&[
        tile(6, 0),
        tile(4, 0),
        tile(2, 0),
        tile(3, 2),
        tile(5, 1),
        tile(5, 3),
    ]);
    let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
    let state = DrivenState {
        decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
        bid,
        declaring_team: Seat::S0.team(),
        viewer_hand,
        leader: Seat::S0,
        trick_plays: &[],
        banked,
        prior_played,
        voids: [
            ContextSet::EMPTY,
            // S1 is pinned to {5-1, 5-3}: shown void in the called suit,
            // twos, and fours.
            voids(&[Context::Called, nat(2), nat(4)]),
            // S2 (the partner) can hold only {6-0, 2-0, 3-2}: void in
            // fours and fives.
            voids(&[nat(4), nat(5)]),
            // S3 can hold only {6-0, 4-0, 2-0}: void in threes and fives.
            voids(&[nat(3), nat(5)]),
        ],
    };
    let (root, position) = driven_root(&state).expect("a lawful fixture root");
    assert_eq!(root.count(), 2, "the void engineering pins two worlds");
    (root, position)
}

/// Policy a of the specimens: highest tile index first — leads the top
/// trump, then cashes the vulnerable tile.
fn spec_policy_a() -> FixedPreference {
    FixedPreference::highest_first("spec-extract-then-cash")
}

/// Policy b: lowest tile index first — exposes the vulnerable tile before
/// extraction.
fn spec_policy_b() -> FixedPreference {
    FixedPreference::lowest_first("spec-expose-then-scramble")
}

// ---------------------------------------------------------------------------
// §2.6 — the worked two-trick specimen: accept, verify, B = 1/2, H = 0,
// and the valid-bound route into Dominated.
// ---------------------------------------------------------------------------

#[test]
fn the_worked_two_trick_specimen_accepts_and_dominates_by_the_valid_bound_route() {
    let (root, position) = worked_two_trick_specimen(35, [28, 7]);
    let field = FieldModel::new(field0_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();
    let witness = one_round_trump_extraction(&root, &position, &a, &b, &field)
        .expect("the §2.6 specimen satisfies all eight hypotheses");
    assert_eq!(witness.lead_a, tile(6, 6));
    assert_eq!(witness.lead_b, tile(1, 1));
    assert_eq!(witness.cells.len(), 6, "the §2.5 two-cell chain pair");
    let hazard = verify_hazard_witness(&root, &position, &a, &b, &field, &witness)
        .expect("the single authority accepts the emitted witness");
    assert_eq!(hazard.hazard_upper(), BigRational::zero());
    assert_eq!(hazard.delta(), BigRational::zero());
    assert_eq!(hazard.witness_hash(), witness.digest());
    let shown = hazard.to_string();
    assert!(shown.starts_with("StructuralHazardZero{"));
    assert!(shown.contains("hazard_upper=0;delta=0"));
    // The witness never contradicts the exact route: full enumeration on
    // the same fiber confirms H = 0 — and the response's worked numbers,
    // B = 1/2 with H = 0, exactly.
    let masses = exact_pairwise_masses(&root, &position, &a, &b, &field);
    assert_eq!(masses.hazard_worlds(), 0, "H = 0 by exact enumeration");
    assert_eq!(masses.h(), BigRational::zero());
    assert_eq!(masses.b(), q(1, 2), "the §2.6 benefit mass, exactly");
    // The valid-bound route: verified H = 0 witness + one exhibited
    // benefit world = Dominated (PANEL-A7's second admission, produced
    // for the first time).
    let benefit = exhibit_benefit_world(&root, &position, &a, &b, &field)
        .expect("the threat world is a benefit world");
    assert_eq!(
        dominance_from_witnessed_hazard_zero(&hazard, &benefit),
        CancellationKind::Dominated
    );
    // The exact-enumeration route agrees, untouched beside it.
    assert_eq!(masses.dominance_kind(), CancellationKind::Dominated);
}

// ---------------------------------------------------------------------------
// §2.7 — the three-trick two-round extraction specimen: dominance real,
// the one-round producer declines at hypothesis 3.
// ---------------------------------------------------------------------------

/// The §2.7 non-coverage specimen: three tricks, the viewer holds the two
/// highest trumps and the vulnerable double; the free six-tile split
/// includes worlds where one hostile hand holds BOTH low trumps, so a
/// one-round extraction cannot cover the fiber — while two rounds
/// (policy a's actual line) still sweep every world.
fn two_round_extraction_specimen() -> (CanonicalRoot, RootPosition) {
    let viewer_hand = set(&[tile(6, 6), tile(6, 5), tile(1, 1)]);
    let pool = set(&[
        tile(6, 0),
        tile(6, 1),
        tile(3, 2),
        tile(5, 1),
        tile(5, 3),
        tile(4, 3),
        tile(4, 0),
        tile(2, 0),
        tile(5, 4),
    ]);
    let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
    let state = DrivenState {
        decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
        bid: 35,
        declaring_team: Seat::S0.team(),
        viewer_hand,
        leader: Seat::S0,
        trick_plays: &[],
        banked: [27, 7],
        prior_played,
        voids: [
            ContextSet::EMPTY,
            // S1 is pinned to {4-0, 2-0, 5-4}; the other six tiles split
            // freely between S2 and S3 (twenty worlds, including the
            // both-trumps-in-one-hostile-hand threat worlds).
            voids(&[Context::Called, nat(1), nat(3)]),
            ContextSet::EMPTY,
            ContextSet::EMPTY,
        ],
    };
    let (root, position) = driven_root(&state).expect("a lawful fixture root");
    assert_eq!(root.count(), 20, "one seat pinned, twenty free splits");
    (root, position)
}

#[test]
fn the_three_trick_specimen_declines_while_exact_enumeration_proves_dominance() {
    let (root, position) = two_round_extraction_specimen();
    let field = FieldModel::new(field0_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();
    // The producer declines — and names hypothesis 3, exactly as §2.7
    // states ("a hostile hand may contain more than one trump").
    assert_eq!(
        one_round_trump_extraction(&root, &position, &a, &b, &field),
        Err(ExtractionDecline::HostileTrumpsExceedOneRound)
    );
    // Dominance is real: the exact-enumeration route proves it on the
    // same fiber. Honest non-coverage, not a missed hazard.
    let masses = exact_pairwise_masses(&root, &position, &a, &b, &field);
    assert_eq!(masses.hazard_worlds(), 0, "H = 0: two rounds sweep");
    assert!(masses.benefit_worlds() > 0, "B > 0: the exposure is real");
    assert_eq!(masses.dominance_kind(), CancellationKind::Dominated);
    // A hand-built witness for this root is outside the v1 language, and
    // the verifier says so rather than guessing.
    let mut witness = one_round_trump_extraction(
        &worked_two_trick_specimen(35, [28, 7]).0,
        &worked_two_trick_specimen(35, [28, 7]).1,
        &a,
        &b,
        &field,
    )
    .expect("the two-trick specimen accepts");
    witness.root_id = walt::solver::adaptive::root_identity(&root, &position);
    assert!(matches!(
        verify_hazard_witness(&root, &position, &a, &b, &field, &witness),
        Err(WitnessRejection::LanguageShape { .. })
    ));
}

// ---------------------------------------------------------------------------
// The refusal path, hypothesis by hypothesis (TRIPLE-A5: refusal is part
// of the producer's correctness).
// ---------------------------------------------------------------------------

#[test]
fn each_constructible_hypothesis_failure_declines_with_its_name() {
    let field = FieldModel::new(field0_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();

    // Frame precondition: a mid-trick root is not a focal lead.
    {
        let viewer_hand = set(&[tile(6, 6), tile(1, 1)]);
        let pool = set(&[tile(6, 0), tile(2, 0), tile(3, 2), tile(5, 1), tile(5, 3)]);
        let led = [tile(4, 0)];
        let prior_played = DominoSet::FULL
            .difference(viewer_hand)
            .difference(pool)
            .difference(set(&led));
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 35,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S3,
            trick_plays: &led,
            banked: [28, 7],
            prior_played,
            voids: [ContextSet::EMPTY; 4],
        };
        let (root, position) = driven_root(&state).expect("a lawful mid-trick root");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::ShapeNotFocalLead)
        );
    }

    // Hypothesis 1: the a lead is not the highest remaining trump (here:
    // not a trump at all — the lowest-first policy leads 1-1).
    {
        let (root, position) = worked_two_trick_specimen(35, [28, 7]);
        assert_eq!(
            one_round_trump_extraction(&root, &position, &b, &b, &field),
            Err(ExtractionDecline::LeadNotHighestTrump)
        );
    }

    // Hypothesis 2: the b lead is not a vulnerable nontrump (here: it is
    // the called 6-6).
    {
        let (root, position) = worked_two_trick_specimen(35, [28, 7]);
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &a, &field),
            Err(ExtractionDecline::LeadNotVulnerableNontrump)
        );
    }

    // Hypothesis 3 on a TWO-trick root: an unconstrained fiber with two
    // low trumps loose puts both in one hostile hand somewhere.
    {
        let viewer_hand = set(&[tile(6, 6), tile(1, 1)]);
        let pool = set(&[
            tile(6, 0),
            tile(6, 1),
            tile(2, 0),
            tile(3, 2),
            tile(5, 1),
            tile(5, 3),
        ]);
        let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 35,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S0,
            trick_plays: &[],
            banked: [28, 7],
            prior_played,
            voids: [ContextSet::EMPTY; 4],
        };
        let (root, position) = driven_root(&state).expect("a lawful fixture root");
        assert_eq!(root.count(), 90, "the unconstrained two-trick fiber");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::HostileTrumpsExceedOneRound)
        );
    }

    // Hypothesis 4: a hostile-holdable nontrump suit beater of the
    // vulnerable lead (5-5 over the led 5-4).
    {
        let viewer_hand = set(&[tile(6, 6), tile(5, 4)]);
        let pool = set(&[
            tile(6, 0),
            tile(5, 5),
            tile(5, 1),
            tile(4, 0),
            tile(2, 0),
            tile(3, 2),
        ]);
        let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 30,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S0,
            trick_plays: &[],
            banked: [20, 5],
            prior_played,
            voids: [ContextSet::EMPTY; 4],
        };
        let (root, position) = driven_root(&state).expect("a lawful fixture root");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::HostileSuitBeater)
        );
    }

    // The v1 language boundary: a three-trick root that passes hypotheses
    // 1–4 (single loose trump, no suit beater of the top-of-suit 1-1) is
    // declined for its shape, not for a hypothesis.
    {
        let viewer_hand = set(&[tile(6, 6), tile(6, 5), tile(1, 1)]);
        let pool = set(&[
            tile(6, 0),
            tile(3, 2),
            tile(5, 1),
            tile(5, 3),
            tile(4, 3),
            tile(2, 1),
            tile(4, 0),
            tile(2, 0),
            tile(5, 4),
        ]);
        let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 35,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S0,
            trick_plays: &[],
            banked: [27, 7],
            prior_played,
            voids: [
                ContextSet::EMPTY,
                voids(&[Context::Called, nat(1), nat(3)]),
                ContextSet::EMPTY,
                ContextSet::EMPTY,
            ],
        };
        let (root, position) = driven_root(&state).expect("a lawful fixture root");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::TwoTrickShapeRequired)
        );
    }

    // Hypothesis 5's vacuous premise: both hostile seats shown void in
    // the called suit — the loose trump is always the partner's, no ruff
    // threat exists, and the pattern honestly refuses.
    {
        let viewer_hand = set(&[tile(6, 6), tile(1, 1)]);
        let pool = set(&[
            tile(6, 0),
            tile(4, 0),
            tile(2, 0),
            tile(3, 2),
            tile(5, 1),
            tile(5, 3),
        ]);
        let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 35,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S0,
            trick_plays: &[],
            banked: [28, 7],
            prior_played,
            voids: [
                ContextSet::EMPTY,
                voids(&[Context::Called]),
                ContextSet::EMPTY,
                voids(&[Context::Called]),
            ],
        };
        let (root, position) = driven_root(&state).expect("a lawful fixture root");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::NoHostileThreatWorld)
        );
    }

    // The companion subtlety: in every threat world the holder also holds
    // the led suit, so follow-suit obligations block the proposed ruff.
    {
        let viewer_hand = set(&[tile(6, 6), tile(1, 1)]);
        let pool = set(&[
            tile(6, 0),
            tile(5, 1),
            tile(4, 0),
            tile(2, 0),
            tile(5, 3),
            tile(3, 2),
        ]);
        let prior_played = DominoSet::FULL.difference(viewer_hand).difference(pool);
        let state = DrivenState {
            decl: Decl::PipTrump(Pip::new(6).expect("a pip")),
            bid: 35,
            declaring_team: Seat::S0.team(),
            viewer_hand,
            leader: Seat::S0,
            trick_plays: &[],
            banked: [28, 7],
            prior_played,
            voids: [
                ContextSet::EMPTY,
                ContextSet::EMPTY,
                ContextSet::EMPTY,
                // S3 is pinned to {6-0, 5-1}: the only possible hostile
                // trump holder always holds a one as well.
                voids(&[nat(2), nat(3), nat(4)]),
            ],
        };
        let (root, position) = driven_root(&state).expect("a lawful fixture root");
        assert_eq!(root.count(), 6, "S3 pinned, six free splits");
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::FollowSuitBlocksRuff)
        );
    }

    // Hypothesis 7: with a point of slack already banked (29 of 35), the
    // lost vulnerable trick is no longer conservatively fatal
    // (29 + 7 − 1 ≥ 35), and the producer refuses the pattern even though
    // the exchange shape — and the field's ruff — are identical.
    {
        let (root, position) = worked_two_trick_specimen(35, [29, 6]);
        assert_eq!(
            one_round_trump_extraction(&root, &position, &a, &b, &field),
            Err(ExtractionDecline::LostVulnerableTrickNotFatal)
        );
    }

    // Hypotheses 6 and 8 have no cheap fixtures BY THE RULES: a called
    // holder's legal answer to a called lead is always called
    // (follow-suit), so ExtractionIncomplete is unreachable without rules
    // drift; and with hypotheses 1–6 holding, the no-hostile-trump
    // residual sweep cannot fail (nothing outranks the verified top trump,
    // and hypothesis 4 removed every hostile suit beater). Both checks
    // stay in the producer as guards, honestly unreachable today.
}

// ---------------------------------------------------------------------------
// The verifier's own refusals: incomplete cover, broken successor
// obligation, unsafe terminal, identity mismatch.
// ---------------------------------------------------------------------------

#[test]
fn the_verifier_rejects_tampered_witnesses_for_the_stated_reason() {
    let (root, position) = worked_two_trick_specimen(35, [28, 7]);
    let field = FieldModel::new(field0_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();
    let witness =
        one_round_trump_extraction(&root, &position, &a, &b, &field).expect("the specimen accepts");

    // An incomplete initial cover: dropping the OneHostileTrump cell
    // leaves the threat world uncovered — named with the world id.
    let mut incomplete = witness.clone();
    incomplete.initial_cover = vec![0];
    assert!(matches!(
        verify_hazard_witness(&root, &position, &a, &b, &field, &incomplete),
        Err(WitnessRejection::InitialCoverIncomplete { .. })
    ));

    // A broken successor obligation: the REVERSED witness (b over a)
    // claims the vulnerable lead sweeps; the closure sweep finds the
    // completion that takes the trick away. The verifier refuses to prove
    // H(b|a) = 0 — which is genuinely nonzero.
    let mut reversed = witness.clone();
    reversed.policy_a = b.id().to_string();
    reversed.policy_b = a.id().to_string();
    reversed.lead_a = witness.lead_b;
    reversed.lead_b = witness.lead_a;
    assert!(matches!(
        verify_hazard_witness(&root, &position, &b, &a, &field, &reversed),
        Err(WitnessRejection::SuccessorObligationBroken { .. })
    ));

    // An unsafe terminal cell: weakening a terminal claim to
    // Unconstrained severs the u_b ≤ u_a implication.
    let mut unsafe_terminal = witness.clone();
    unsafe_terminal.cells[2].claim_a = BranchClaim::Unconstrained;
    assert!(matches!(
        verify_hazard_witness(&root, &position, &a, &b, &field, &unsafe_terminal),
        Err(WitnessRejection::TerminalUnsafe { cell: 2 })
    ));

    // Identity mismatches: another field, or swapped policies, are not
    // the named objects.
    let field1 = FieldModel::new(field1_spec());
    assert!(matches!(
        verify_hazard_witness(&root, &position, &a, &b, &field1, &witness),
        Err(WitnessRejection::IdentityMismatch { .. })
    ));
    assert!(matches!(
        verify_hazard_witness(&root, &position, &b, &a, &field, &witness),
        Err(WitnessRejection::IdentityMismatch { .. })
    ));

    // Every rejection serializes with its mechanical tag.
    let shown = WitnessRejection::TerminalUnsafe { cell: 2 }.to_string();
    assert!(shown.starts_with("WitnessRejection{terminal-unsafe"));
}

// ---------------------------------------------------------------------------
// §2.8 — no cross-field composition: the dominance wiring refuses mixed
// identities outright.
// ---------------------------------------------------------------------------

#[test]
fn no_dominance_composes_across_fields() {
    let (root, position) = worked_two_trick_specimen(35, [28, 7]);
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();
    let witness = one_round_trump_extraction(&root, &position, &a, &b, &field0)
        .expect("the specimen accepts under σ0");
    let hazard = verify_hazard_witness(&root, &position, &a, &b, &field0, &witness)
        .expect("the σ0 witness verifies");
    // Benefit evidence under a DIFFERENT field can never combine with the
    // σ0 witness (response §2.8): the wiring panics rather than compose.
    let cross = exhibit_benefit_world(&root, &position, &a, &b, &field1)
        .expect("σ1 also exposes a benefit world here");
    assert!(catch_unwind(AssertUnwindSafe(|| {
        dominance_from_witnessed_hazard_zero(&hazard, &cross)
    }))
    .is_err());
    // Same-field evidence composes.
    let same = exhibit_benefit_world(&root, &position, &a, &b, &field0).expect("a benefit world");
    assert_eq!(
        dominance_from_witnessed_hazard_zero(&hazard, &same),
        CancellationKind::Dominated
    );
}

// ---------------------------------------------------------------------------
// Soundness cross-check and honest accept/decline on the frozen receipt
// corpus root (the standing exact Dominated specimen's register).
// ---------------------------------------------------------------------------

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn pinned(position: &RootPosition, tile: Domino) -> FrozenPolicy {
    FrozenPolicy::new(FreezeTuple {
        solver_source: "walt-level1-continuation-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "level0".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::Declared(vec![2, 2]),
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        practical_equivalence: None,
        policy_library: "level1-continuation-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
        action_rule: ActionRule::PinnedThenLevel1 { pinned: tile },
    })
}

#[test]
fn producer_accepts_never_contradict_exact_enumeration_on_the_corpus_root() {
    let r = receipt();
    // h4-t6: the two-trick parity root carrying the standing exact
    // Dominated specimen (pin-1-1 over pin-0-0 under both fields).
    let (root, position) = root_at(&r, 4, 6);
    assert_eq!(root.count(), 90);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let actions: Vec<Domino> = legal_plays(position.decl, root.kernel().viewer_hand(), led)
        .iter()
        .collect();
    let policies: Vec<FrozenPolicy> = actions.iter().map(|t| pinned(&position, *t)).collect();
    let mut accepts = 0usize;
    let mut declines = 0usize;
    for spec in [field0_spec(), field1_spec()] {
        let field = FieldModel::new(spec);
        for (i, rho_a) in policies.iter().enumerate() {
            for (j, rho_b) in policies.iter().enumerate() {
                if i == j {
                    continue;
                }
                match one_round_trump_extraction(&root, &position, rho_a, rho_b, &field) {
                    Ok(witness) => {
                        accepts += 1;
                        // The single authority accepts what the producer
                        // emits, and the witness never contradicts the
                        // exact route on the same fiber.
                        let hazard =
                            verify_hazard_witness(&root, &position, rho_a, rho_b, &field, &witness)
                                .expect("an emitted witness verifies");
                        let masses = exact_pairwise_masses(&root, &position, rho_a, rho_b, &field);
                        assert_eq!(
                            masses.hazard_worlds(),
                            0,
                            "a verified H = 0 witness agrees with exact enumeration"
                        );
                        if let Some(benefit) =
                            exhibit_benefit_world(&root, &position, rho_a, rho_b, &field)
                        {
                            assert_eq!(
                                dominance_from_witnessed_hazard_zero(&hazard, &benefit),
                                CancellationKind::Dominated
                            );
                            assert_eq!(masses.dominance_kind(), CancellationKind::Dominated);
                        }
                    }
                    Err(decline) => {
                        declines += 1;
                        // A decline is a refusal to certify, never a
                        // hazard claim: the exact route remains free to
                        // prove dominance (the §2.7 shape) or not.
                        let _ = decline.tag();
                    }
                }
            }
        }
    }
    // Honest recording: a deliberately narrow first producer mostly
    // declines; the counts are reported by the probe, and this gate pins
    // only the census.
    assert_eq!(accepts + declines, 4, "two ordered pairs under two fields");
}

// ---------------------------------------------------------------------------
// Type-level locks, restated at run time (the compile-time halves are the
// module's compile_fail doctests).
// ---------------------------------------------------------------------------

#[test]
fn the_delta_zero_type_is_reachable_only_through_the_verifier() {
    // The only constructor of StructuralHazardZero in the crate's public
    // surface is verify_hazard_witness's Ok — this function's signature is
    // the run-time restatement (the compile_fail doctests on
    // `solver::hazard` are the compile-time demonstration that neither a
    // struct literal nor a sampled object can reach it).
    fn only_the_verifier_mints(
        outcome: Result<
            walt::solver::hazard::StructuralHazardZero,
            walt::solver::hazard::WitnessRejection,
        >,
    ) -> Option<walt::solver::hazard::StructuralHazardZero> {
        outcome.ok()
    }
    let (root, position) = worked_two_trick_specimen(35, [28, 7]);
    let field = FieldModel::new(field0_spec());
    let a = spec_policy_a();
    let b = spec_policy_b();
    let witness =
        one_round_trump_extraction(&root, &position, &a, &b, &field).expect("the specimen accepts");
    let minted = only_the_verifier_mints(verify_hazard_witness(
        &root, &position, &a, &b, &field, &witness,
    ));
    assert!(minted.is_some());
    // Every decline tag is mechanically distinct (histogram-safe).
    for (i, x) in ExtractionDecline::ALL.iter().enumerate() {
        for y in &ExtractionDecline::ALL[i + 1..] {
            assert_ne!(x.tag(), y.tag());
        }
    }
}
