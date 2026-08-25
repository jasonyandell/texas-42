//! Gates for slice 4c [L2 thread]: the six-motif first-split morphology
//! classifier over correction traces, the raw post-split suffix
//! enrichment, root-frame resolution, and the §3.7 aggregate typing.
//!
//! Mathematical source: Part 3 (§§3.1–3.9) and proof ledger P6 of the
//! x:024 response
//! (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`),
//! adopted by rulings TRIPLE-A6/A7 (`walt/CENSUS-RULINGS.md`, "The
//! deferred-producers adjudication (2026-08-25)"). Everything gated here
//! stays exploratory instrument tier.
//!
//! DECLARED TEST EPOCH PAIR (unchanged from slices 2–3): σ0 = Level0
//! { n0 = 2 }, σ1 = Level1 { n_outer = 2, n0 = 2 }; frozen focal
//! candidates at declared schedule [2, 2]. Exact parity roots from the
//! frozen `verify_player` receipt: hand 4 trick 6 (fiber 90), hand 8
//! trick 5 (fiber 92), hand 10 trick 6 (fiber 19). At each root the
//! viewer leads, so every held tile is a legal root action.

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use walt::kernel::Kernel;
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat, Team};
use walt::solver::adaptive::{root_identity, CanonicalRoot, RootPosition};
use walt::solver::exposure::{frozen_policy_exposure, RootActionExposureUpper, WorldDomain};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::field_swap::{ActionExposureUpper, CancellationLadder};
use walt::solver::motif::{
    classify_signature_pair, classify_trace, enrich_field_split_traces, DescriptiveMotifHistogram,
    EnrichedFieldSplitTrace, ExactMotifDecomposition, MotifClassification, ResidualReason,
    RootFrame, RootFrameRegistry, SplitMotif, SplitSignature,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

/// The declared exact parity roots: (hand, trick, fiber).
const PARITY_ROOTS: [(usize, usize, u128); 3] = [(4, 6, 90), (8, 5, 92), (10, 6, 19)];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

/// σ0 of the declared test epoch pair.
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

/// σ1 of the declared test epoch pair.
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

/// One pinned frozen focal candidate for a legal root action, at the
/// declared [2, 2] schedule.
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

fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

/// Enriched-and-classified correction traces of one (root, action) at the
/// declared test pair, with the ladder they decompose.
fn classified_corpus(
    root: &CanonicalRoot,
    position: &RootPosition,
    action: Domino,
) -> (
    CancellationLadder,
    Vec<(EnrichedFieldSplitTrace, MotifClassification)>,
    RootFrameRegistry,
) {
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let rho = pinned(position, action);
    let exposure = frozen_policy_exposure(
        root,
        position,
        &rho,
        &field0,
        &field1,
        WorldDomain::ExactFiber,
    );
    let ladder = CancellationLadder::from_exposure(&exposure);
    let enriched =
        enrich_field_split_traces(root, position, action, &rho, &field0, &field1, &exposure);
    let mut registry = RootFrameRegistry::new();
    registry.register(root_identity(root, position), RootFrame::of(root, position));
    let classified: Vec<(EnrichedFieldSplitTrace, MotifClassification)> = enriched
        .into_iter()
        .map(|e| {
            let c = classify_trace(&e, &registry);
            (e, c)
        })
        .collect();
    (ladder, classified, registry)
}

// ---------------------------------------------------------------------------
// P6 — every correction trace gets exactly one primary label, and the
// §3.7 exact decomposition identities hold on the real corpus.
// ---------------------------------------------------------------------------

#[test]
fn p6_partition_and_exact_decomposition_identities_on_the_parity_roots() {
    let r = receipt();
    let mut total_corrections = 0u64;
    for (hand_id, trick_no, fiber) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        assert_eq!(root.count(), fiber, "the declared parity fiber");
        for action in legal_root_actions(&root, &position).iter() {
            let (ladder, classified, registry) = classified_corpus(&root, &position, action);
            total_corrections += ladder.outcome_changed;
            // P6: every trace maps to exactly one primary label — the
            // label is one enum value, `Other` carries a reason exactly
            // when it is the label, and the flags exist because the frame
            // resolved for every trace here.
            let mut histogram = [0u64; 7];
            for (enriched, classification) in &classified {
                histogram[classification.motif.index()] += 1;
                assert_eq!(
                    classification.motif == SplitMotif::Other,
                    classification.residual.is_some(),
                    "the residual reason travels with Other and only Other"
                );
                let flags = classification
                    .flags
                    .as_ref()
                    .expect("a resolved frame derives the flags");
                match classification.motif {
                    SplitMotif::Other => {
                        assert!(!flags.any(), "Other means no coordinate differs");
                        assert_eq!(
                            classification.residual,
                            Some(ResidualReason::NoCoordinateDiffers)
                        );
                    }
                    motif => {
                        let least = flags
                            .ordered()
                            .iter()
                            .position(|d| *d)
                            .expect("a motif has a differing coordinate");
                        assert_eq!(least, motif.index(), "least differing coordinate");
                    }
                }
                // Determinism: classifying the same trace again yields
                // the identical classification.
                assert_eq!(
                    *classification,
                    classify_trace(enriched, &registry),
                    "classification is deterministic"
                );
                // §3.5: the flags re-derive from the same signature pair
                // — derived views, never a second authority.
                let (s0, s1) = classification.signatures.expect("resolved signatures");
                let (motif2, residual2, flags2) = classify_signature_pair(&s0, &s1);
                assert_eq!(motif2, classification.motif);
                assert_eq!(residual2, classification.residual);
                assert_eq!(flags2, *flags);
            }
            assert_eq!(
                histogram.iter().sum::<u64>(),
                ladder.outcome_changed,
                "P6: the histogram is a census of the correction traces"
            );
            // §3.7 identities, re-asserted externally against the ladder.
            let decomposition = ExactMotifDecomposition::from_classified(&ladder, &classified);
            assert_eq!(decomposition.plus.iter().sum::<u64>(), ladder.c_plus);
            assert_eq!(decomposition.minus.iter().sum::<u64>(), ladder.c_minus);
            let mut net = BigRational::zero();
            let mut r_total = BigRational::zero();
            for motif in SplitMotif::ALL {
                net += decomposition.c_k(motif);
                r_total += decomposition.r_k(motif);
                assert_eq!(
                    decomposition.correction_worlds(motif),
                    decomposition.plus[motif.index()] + decomposition.minus[motif.index()],
                );
                // τ_k exists exactly when r_k > 0, lies in [−1, 1], and
                // r_k · τ_k = c_k exactly.
                match decomposition.tilt(motif) {
                    None => assert_eq!(decomposition.correction_worlds(motif), 0),
                    Some(tilt) => {
                        let one = BigRational::from_integer(BigInt::from(1));
                        assert!(-&one <= tilt && tilt <= one);
                        assert_eq!(decomposition.r_k(motif) * tilt, decomposition.c_k(motif));
                    }
                }
            }
            assert_eq!(net, ladder.c(), "Σ c_k = c holds exactly");
            assert_eq!(
                r_total,
                ladder.r(),
                "the motif masses are a census of the correction mass"
            );
        }
    }
    assert!(
        total_corrections > 0,
        "the parity corpus exercises the classifier on real corrections"
    );
}

// ---------------------------------------------------------------------------
// §3.4–§3.5 — every motif and the no-difference residual are reachable,
// the ordering is first-difference, and the flags retain co-occurrence.
// ---------------------------------------------------------------------------

fn pip(v: u8) -> Pip {
    Pip::new(v).expect("a pip")
}

/// A hand-built signature at the fives declaration; every coordinate is a
/// real rules value.
fn base_signature() -> SplitSignature {
    let decl = Decl::PipTrump(pip(5));
    let three_two = Domino::new(pip(3), pip(2));
    SplitSignature {
        led: Context::Natural(pip(3)),
        control: Team::T0,
        count: 0,
        trump: false,
        shape: [1, 1, 1, 1, 0, 0, 0, 2],
        strength: decl.trick_key(three_two, Context::Natural(pip(3))),
    }
}

#[test]
fn every_motif_and_the_no_difference_residual_are_reachable() {
    let decl = Decl::PipTrump(pip(5));
    let base = base_signature();
    let six_four = Domino::new(pip(6), pip(4));
    // Vary exactly one coordinate at a time, in coordinate order.
    let variants: [(SplitMotif, SplitSignature); 6] = [
        (
            SplitMotif::LeadContextFork,
            SplitSignature {
                led: Context::Called,
                ..base
            },
        ),
        (
            SplitMotif::ImmediateControlFork,
            SplitSignature {
                control: Team::T1,
                ..base
            },
        ),
        (
            SplitMotif::CountCommitmentFork,
            SplitSignature { count: 10, ..base },
        ),
        (
            SplitMotif::TrumpCommitmentFork,
            SplitSignature {
                trump: true,
                ..base
            },
        ),
        (
            SplitMotif::SuitShapeFork,
            SplitSignature {
                shape: [0, 1, 1, 1, 1, 0, 0, 2],
                ..base
            },
        ),
        (
            SplitMotif::StrengthCommitmentFork,
            SplitSignature {
                strength: decl.trick_key(six_four, Context::Natural(pip(6))),
                ..base
            },
        ),
    ];
    for (expected, varied) in &variants {
        let (motif, residual, flags) = classify_signature_pair(&base, varied);
        assert_eq!(motif, *expected, "one varied coordinate names its motif");
        assert_eq!(residual, None);
        assert_eq!(
            flags.ordered().iter().filter(|d| **d).count(),
            1,
            "exactly one flag set"
        );
        assert!(flags.ordered()[expected.index()]);
        // The pair order does not matter for the label.
        let (mirrored, _, _) = classify_signature_pair(varied, &base);
        assert_eq!(mirrored, *expected);
    }
    // Identical signatures: the residual, with its reason — never a
    // nearest readable label.
    let (motif, residual, flags) = classify_signature_pair(&base, &base);
    assert_eq!(motif, SplitMotif::Other);
    assert_eq!(residual, Some(ResidualReason::NoCoordinateDiffers));
    assert!(!flags.any());
    // Co-occurring differences: the PRIMARY is the least index (a
    // taxonomy convention, not a causal ranking — §3.6) and the flags
    // retain every difference.
    let many = SplitSignature {
        control: Team::T1,
        count: 5,
        trump: true,
        ..base
    };
    let (motif, _, flags) = classify_signature_pair(&base, &many);
    assert_eq!(
        motif,
        SplitMotif::ImmediateControlFork,
        "the earliest differing coordinate captures the primary label"
    );
    assert!(flags.diff_control && flags.diff_count && flags.diff_trump);
    assert!(!flags.diff_context && !flags.diff_suit_shape && !flags.diff_strength);
}

// ---------------------------------------------------------------------------
// §3.2 / TRIPLE-A6 — root-frame resolution failure declines; it never
// guesses.
// ---------------------------------------------------------------------------

#[test]
fn missing_root_frame_declines_and_never_guesses() {
    let r = receipt();
    // A root with real corrections at the declared pair.
    let mut sample: Option<EnrichedFieldSplitTrace> = None;
    'roots: for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        for action in legal_root_actions(&root, &position).iter() {
            let (_, classified, _) = classified_corpus(&root, &position, action);
            if let Some((enriched, _)) = classified.into_iter().next() {
                sample = Some(enriched);
                break 'roots;
            }
        }
    }
    let enriched = sample.expect("the parity corpus holds a correction trace");
    // An empty registry: the classifier returns the residual with
    // missing_root_frame and derives NOTHING else.
    let empty = RootFrameRegistry::new();
    let declined = classify_trace(&enriched, &empty);
    assert_eq!(declined.motif, SplitMotif::Other);
    assert_eq!(declined.residual, Some(ResidualReason::MissingRootFrame));
    assert_eq!(declined.flags, None, "no signature is derivable");
    assert_eq!(declined.signatures, None);
    assert_eq!(declined.split_actor_relation, None);
    assert_eq!(declined.terminal_sign, None);
    // A registry holding a DIFFERENT frame for the same root_id (the
    // semantics hash disagrees): still a decline, never a guess.
    let (hand_id, trick_no, _) = PARITY_ROOTS[0];
    let (other_root, other_position) = root_at(&r, hand_id, trick_no);
    let other_frame = RootFrame::of(&other_root, &other_position);
    if other_frame.semantics_hash() != enriched.root_semantics_hash {
        let mut wrong = RootFrameRegistry::new();
        wrong.register(enriched.trace.root_id, other_frame);
        let declined = classify_trace(&enriched, &wrong);
        assert_eq!(declined.residual, Some(ResidualReason::MissingRootFrame));
    }
}

// ---------------------------------------------------------------------------
// §3.9 / TRIPLE-A7 — the enrichment is raw, replayable, and carries no
// motif tag.
// ---------------------------------------------------------------------------

#[test]
fn suffix_enrichment_is_raw_and_replayable() {
    let r = receipt();
    let mut checked = 0u64;
    for (hand_id, trick_no, _) in PARITY_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let frame = RootFrame::of(&root, &position);
        let semantics = frame.semantics_hash();
        for action in legal_root_actions(&root, &position).iter() {
            let (_, classified, _) = classified_corpus(&root, &position, action);
            for (enriched, _) in &classified {
                checked += 1;
                let t = &enriched.trace;
                // The semantics hash is the frame's own, deterministically.
                assert_eq!(enriched.root_semantics_hash, semantics);
                assert_eq!(semantics, frame.semantics_hash());
                // Both suffixes exhaust the remaining tiles after the
                // split play, each tile exactly once, and neither replays
                // its branch's split tile.
                let remaining = Domino::COUNT
                    - position.prior_played.len()
                    - position.trick_plays.len()
                    - t.split.history.len()
                    - 1;
                assert_eq!(enriched.branch0_suffix.len(), remaining);
                assert_eq!(enriched.branch1_suffix.len(), remaining);
                let mut played0 = DominoSet::EMPTY;
                let mut played1 = DominoSet::EMPTY;
                for (_, d) in &enriched.branch0_suffix {
                    assert!(played0.insert(*d), "a tile is played once");
                }
                for (_, d) in &enriched.branch1_suffix {
                    assert!(played1.insert(*d), "a tile is played once");
                }
                assert!(!played0.contains(t.split.tile0));
                assert!(!played1.contains(t.split.tile1));
                // The suffixes make the distinguishing public observation
                // explicit (the item-11 closure): branch b's public line
                // is history · (seat, tile_b) · suffix_b, and the two
                // lines first differ at the recorded split play.
                assert_ne!(t.split.tile0, t.split.tile1);
                // Raw record discipline (TRIPLE-A7): the enriched trace
                // serializes its raw fields and NO motif tag — motif
                // labels are derived views, never persisted trace state.
                let serialized = enriched.to_string();
                assert!(!serialized.contains("Fork"), "no motif tag on the trace");
                assert!(serialized.contains("suffix0="));
                assert!(serialized.contains("suffix1="));
            }
        }
    }
    assert!(checked > 0, "the corpus exercised the enrichment");
}

// ---------------------------------------------------------------------------
// §3.7 — descriptive histograms are typed out of the screen; the exact
// decomposition refuses sampled domains.
// ---------------------------------------------------------------------------

#[test]
fn descriptive_histograms_are_typed_out_of_the_screen() {
    let labels = [
        SplitMotif::LeadContextFork,
        SplitMotif::LeadContextFork,
        SplitMotif::CountCommitmentFork,
        SplitMotif::Other,
    ];
    let domain = WorldDomain::StreamPrefix {
        epoch: 0,
        worlds: 16,
    };
    let position = RootPosition {
        decl: Decl::PipTrump(pip(5)),
        bid: 30,
        declaring_team: Team::T0,
        leader: Seat::S0,
        banked: [0, 0],
        trick_plays: vec![],
        prior_played: DominoSet::EMPTY,
        voids: Default::default(),
    };
    let policy = pinned(&position, Domino::new(pip(5), pip(5)));
    let field0 = FieldModel::new(field0_spec());
    let field1 = FieldModel::new(field1_spec());
    let histogram = DescriptiveMotifHistogram::from_stream(
        policy.policy_id(),
        field0.field_id(),
        field1.field_id(),
        0xABCD,
        &domain,
        &labels,
    );
    assert_eq!(histogram.count(SplitMotif::LeadContextFork), 2);
    assert_eq!(histogram.count(SplitMotif::CountCommitmentFork), 1);
    assert_eq!(histogram.count(SplitMotif::Other), 1);
    assert_eq!(histogram.domain(), domain);
    // The serialization is mechanically labeled descriptive.
    assert!(histogram
        .to_string()
        .starts_with("DescriptiveMotifHistogram{descriptive;"));
    // The screen's one exposure input wraps RootActionExposureUpper and
    // nothing else; a descriptive histogram has no conversion to it. The
    // module's compile_fail doctest is the type-level gate — this
    // function is the only door, and the histogram does not fit it.
    fn only_screenable(action: Domino, bound: RootActionExposureUpper) -> ActionExposureUpper {
        ActionExposureUpper { action, bound }
    }
    let _ = only_screenable;
    // An exact-fiber domain is refused: exact corrections take
    // ExactMotifDecomposition and its identity obligations.
    let refused = catch_unwind(AssertUnwindSafe(|| {
        DescriptiveMotifHistogram::from_stream(
            policy.policy_id(),
            field0.field_id(),
            field1.field_id(),
            0xABCD,
            &WorldDomain::ExactFiber,
            &labels,
        )
    }));
    assert!(
        refused.is_err(),
        "a descriptive histogram never ranges over the exact fiber"
    );
}

// ---------------------------------------------------------------------------
// §3.2 — the registry is exact; frames are immutable records.
// ---------------------------------------------------------------------------

#[test]
fn the_root_frame_registry_resolves_exactly_and_frames_are_immutable() {
    let r = receipt();
    let (root, position) = root_at(&r, 8, 5);
    let frame = RootFrame::of(&root, &position);
    let root_id = root_identity(&root, &position);
    let mut registry = RootFrameRegistry::new();
    let hash = registry.register(root_id, frame.clone());
    assert_eq!(hash, frame.semantics_hash());
    // Exact resolution; a wrong key resolves to nothing (and the
    // classifier's decline path, gated above, is the only consumer of
    // that nothing).
    assert_eq!(registry.resolve(root_id, hash), Some(&frame));
    assert_eq!(registry.resolve(root_id, hash ^ 1), None);
    assert_eq!(registry.resolve(root_id ^ 1, hash), None);
    // Idempotent re-registration of the identical frame is fine.
    let again = registry.register(root_id, frame.clone());
    assert_eq!(again, hash);
    // A semantically different frame keys under its own hash — a
    // different record, never an overwrite of the immutable one.
    let mut mutated = frame.clone();
    mutated.bid += 1;
    let mutated_hash = mutated.semantics_hash();
    assert_ne!(
        mutated_hash, hash,
        "the semantics hash separates the frames"
    );
    let other = registry.register(root_id, mutated.clone());
    assert_eq!(other, mutated_hash);
    assert_eq!(registry.resolve(root_id, hash), Some(&frame));
    assert_eq!(registry.resolve(root_id, mutated_hash), Some(&mutated));
}
