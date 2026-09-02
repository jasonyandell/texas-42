//! Slice UP0 — the unified walt player. Six gates.
//!
//! UP1 totality and the no-swallowed-refusal discipline: every legal
//!     state of every walked root gets a Decision with well-formed
//!     provenance, under three budget rungs; plus the source gate that
//!     the module holds no `unwrap`/`panic!`/`unreachable!`/`todo!` and
//!     that every `expect` in it is annotated as a rules invariant.
//! UP2 endgame consistency: where U0's God-tight receipts are seeded
//!     into the store, the decision CONSUMES them — zero field
//!     consultations — and re-prices to the God upper; and the consumed
//!     instruments are byte-identical either side of the decision.
//! UP3 the join: the carried posterior is a derived view of (root,
//!     public line) and nothing else, checked against an independent
//!     replay and against MB1's own `trace_heaviest_line`; the value-move
//!     specimen and the argmax-flip specimen are pinned.
//! UP4 budget honesty: a starved budget falls through with every refusal
//!     typed and the σ0 fallback named; the same state under the same
//!     budget yields an identical Decision.
//! UP5 the consumed instruments are unperturbed, and the counting
//!     decorator around the declared field is value-neutral.
//! UP6 provenance soundness: every claimed tier is independently
//!     verifiable, and provenance has no public constructor.
//!
//! EXPLORATORY tier throughout. Nothing here is a play-strength claim.

use std::rc::Rc;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::replay::state_before_trick;
use walt::rules::{legal_plays, Context, ContextSet, Domino, DominoSet, Trick};
use walt::solver::adaptive::{
    decided_success, driven_root, root_identity, CanonicalRoot, DrivenState, RootPosition,
    SlicePolicy,
};
use walt::solver::doom::DoomSpec;
use walt::solver::factor_belief::{
    extract_success_policy, response_success_mass, viewer_success_mass, ExactCoverOracle,
    ExtractionSource, FactorBelief, RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::godgap::{coordinate_facts, GodGapCoordinate, GodGapSpec, GodGapWalk};
use walt::solver::model_belief::{
    BehaviorType, BehaviorTypeId, MixtureStats, ModelBelief, PersistenceScope, SeatTypePrior,
};
use walt::solver::model_recursion::trace_heaviest_line;
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::ProofState;
use walt::solver::unified::{
    fixed_field_identity, Decision, Evidence, MoveBudget, ReceiptStore, Recursion, Tier,
    TypeLibrary, UnifiedPlayer,
};

// ---------------------------------------------------------------------------
// The declared epoch, shared with the probe.
// ---------------------------------------------------------------------------

fn field_spec_level0() -> FieldSpec {
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

fn field_spec_level1() -> FieldSpec {
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

fn registered_types() -> (Rc<BehaviorType>, Rc<BehaviorType>) {
    (
        Rc::new(BehaviorType::from_field(
            field_spec_level0(),
            PersistenceScope::PerHand,
        )),
        Rc::new(BehaviorType::from_field(
            field_spec_level1(),
            PersistenceScope::PerHand,
        )),
    )
}

fn library() -> TypeLibrary {
    let (f0, f1) = registered_types();
    TypeLibrary::new(vec![(f0, 1), (f1, 1)])
}

fn mixture_at(root: &CanonicalRoot, position: &RootPosition) -> ModelBelief {
    let (f0, f1) = registered_types();
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(&f0), 1), (Rc::clone(&f1), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

fn receipt() -> Receipt {
    parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses")
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn ratio(mass: u128, total: u128) -> BigRational {
    BigRational::new(BigInt::from(mass), BigInt::from(total))
}

// The three declared budget rungs the gates use.

fn budget(label: &str, enum_cap: u128, mix_cap: u128, reads: u64, join: bool) -> MoveBudget {
    MoveBudget {
        label: label.to_string(),
        enumeration_fiber_cap: enum_cap,
        mixture_fiber_cap: mix_cap,
        mixture_read_cap: reads,
        regret_acceptance: BigRational::new(BigInt::from(1), BigInt::from(4)),
        join_reading: join,
    }
}

fn lean() -> MoveBudget {
    budget("lean", 32, 0, 0, false)
}

fn ample() -> MoveBudget {
    budget("ample", 40_000, 256, 4_000_000, true)
}

/// The two structural caps swapped: the world-space tier refuses on the
/// band between them and the MODEL-space tier is the one that answers.
/// The cascade's declared order means tier (c) can only ever answer where
/// tier (b) refused first, so this is the rung that exercises it.
fn model_rung() -> MoveBudget {
    budget("model", 8, 256, 4_000_000, false)
}

// ---------------------------------------------------------------------------
// The walk driver — the gates' own, independent of the probe binary.
// ---------------------------------------------------------------------------

/// One recorded ply of a walked hand.
struct Play {
    trick: usize,
    ply: usize,
    seat: usize,
    /// The full public state at the decision, replayable.
    state: OwnedState,
    decision: Decision,
}

/// An owned copy of the public state at one decision, so a gate can
/// rebuild the `DrivenState` and re-ask the same question.
#[derive(Clone)]
struct OwnedState {
    bid: u32,
    declaring_team: walt::rules::Team,
    viewer_hand: DominoSet,
    leader: walt::rules::Seat,
    trick_plays: Vec<Domino>,
    banked: [u32; 2],
    prior_played: DominoSet,
    voids: [ContextSet; 4],
}

impl OwnedState {
    fn driven<'a>(&'a self, decl: walt::rules::Decl) -> DrivenState<'a> {
        DrivenState {
            decl,
            bid: self.bid,
            declaring_team: self.declaring_team,
            viewer_hand: self.viewer_hand,
            leader: self.leader,
            trick_plays: &self.trick_plays,
            banked: self.banked,
            prior_played: self.prior_played,
            voids: self.voids,
        }
    }
}

/// Every derived view of one seat's carried line, read off the carried
/// [`ModelBelief`] before the player is dropped. Nothing here is stored
/// by the player: each field is recomputed from the one carried object.
struct LineView {
    seat: usize,
    root_id: u64,
    history: Vec<Domino>,
    live_profiles: usize,
    weighted_total: u128,
    marginals: Vec<(walt::rules::Seat, Vec<(BehaviorTypeId, u128)>)>,
    posterior_masses: Vec<(String, u128)>,
    falsified: bool,
    observations: usize,
    focal_plays: usize,
}

/// Walk one receipt root under the registered type library.
fn walk(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    b: &MoveBudget,
    store: ReceiptStore,
    max_plies: usize,
) -> (Vec<Play>, Vec<LineView>) {
    walk_with(r, hand_id, trick_no, b, store, max_plies, library())
}

/// Walk one receipt root to terminal (or for at most `max_plies`
/// decisions) with the unified player choosing every seat's action.
#[allow(clippy::too_many_arguments)]
fn walk_with(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    b: &MoveBudget,
    store: ReceiptStore,
    max_plies: usize,
    lib: TypeLibrary,
) -> (Vec<Play>, Vec<LineView>) {
    let hand = &r.hands[hand_id];
    let (start_hands, start_leader) =
        state_before_trick(hand, trick_no).expect("a valid receipt trick");
    let decl = hand.decl;
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    let mut player = UnifiedPlayer::new(&oracle, &field, lib);
    player.seed_store(store);

    let mut hands = start_hands;
    let mut leader = start_leader;
    let mut banked = [0u32; 2];
    let mut prior_played = DominoSet::EMPTY;
    let mut voids = [ContextSet::EMPTY; 4];
    for t in hand.tricks.iter().take(trick_no - 1) {
        let doms: [Domino; 4] = core::array::from_fn(|i| t.plays[i].1);
        let trick = Trick::new(t.plays[0].0, doms).expect("four distinct tiles");
        banked[trick.winner(decl).team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        let led = decl.led_context(doms[0]);
        for (k, d) in doms.iter().enumerate() {
            if !decl.follows(*d, led) {
                voids[t.plays[0].0.plus(k).index()].insert(led);
            }
        }
    }

    let mut trick_plays: Vec<Domino> = Vec::new();
    let mut plays: Vec<Play> = Vec::new();
    'outer: for trick_index in trick_no..=7usize {
        for _ in 0..4usize {
            if plays.len() >= max_plies {
                break 'outer;
            }
            let seat = leader.plus(trick_plays.len());
            let owned = OwnedState {
                bid: hand.bid_points,
                declaring_team: hand.declaring_team,
                viewer_hand: hands[seat.index()],
                leader,
                trick_plays: trick_plays.clone(),
                banked,
                prior_played,
                voids,
            };
            let state = owned.driven(decl);
            let decision = player.decide(&state, b);
            let led: Option<Context> = trick_plays.first().map(|t| decl.led_context(*t));
            let legal = legal_plays(decl, hands[seat.index()], led);
            assert!(
                legal.contains(decision.action()),
                "UP1: the unified player chooses a legal tile at every state"
            );
            player.observe_play(&state, decision.action());
            if let Some(led) = led {
                if !decl.follows(decision.action(), led) {
                    voids[seat.index()].insert(led);
                }
            }
            assert!(
                hands[seat.index()].remove(decision.action()),
                "the chosen tile is held"
            );
            trick_plays.push(decision.action());
            plays.push(Play {
                trick: trick_index,
                ply: trick_plays.len() - 1,
                seat: seat.index(),
                state: owned,
                decision,
            });
        }
        let doms: [Domino; 4] = core::array::from_fn(|i| trick_plays[i]);
        let trick = Trick::new(leader, doms).expect("four distinct tiles");
        banked[trick.winner(decl).team().index()] += trick.points();
        for d in doms {
            prior_played.insert(d);
        }
        leader = trick.winner(decl);
        trick_plays.clear();
    }
    let mut views: Vec<LineView> = Vec::new();
    for seat in player.lines() {
        let Some(line) = player.line(seat) else {
            continue;
        };
        let (history, live_profiles, weighted_total, marginals, posterior_masses) =
            match line.model() {
                Some(m) => (
                    m.history().to_vec(),
                    m.profiles().len(),
                    m.weighted_total(&oracle),
                    m.seat_type_marginals(&oracle),
                    m.posterior_profile_masses(&oracle),
                ),
                None => (Vec::new(), 0, 0, Vec::new(), Vec::new()),
            };
        views.push(LineView {
            seat: seat.index(),
            root_id: line.root_id(),
            history,
            live_profiles,
            weighted_total,
            marginals,
            posterior_masses,
            falsified: line.falsified().is_some(),
            observations: line.observations(),
            focal_plays: line.focal_plays(),
        });
    }
    (plays, views)
}

/// MB0's six enumerable roots plus the smallest trick-4 root.
const CORPUS: [(usize, usize); 7] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5), (8, 4)];

// ---------------------------------------------------------------------------
// UP1 — totality, and the source discipline.
// ---------------------------------------------------------------------------

#[test]
fn up1_every_legal_state_gets_a_well_formed_decision() {
    let r = receipt();
    let rungs = [MoveBudget::starved("starved"), lean(), model_rung()];
    let mut decisions = 0usize;
    let mut tiers_seen: Vec<Tier> = Vec::new();
    for b in &rungs {
        for (hand_id, trick_no) in CORPUS {
            let (plays, _) = walk(&r, hand_id, trick_no, b, ReceiptStore::new(), usize::MAX);
            let expected = (8 - trick_no) * 4;
            assert_eq!(
                plays.len(),
                expected,
                "UP1: the cascade terminates at every state of h{hand_id}-t{trick_no}, \
                 so a full hand is walked"
            );
            for p in &plays {
                decisions += 1;
                let prov = p.decision.provenance();
                // Well-formed provenance: the tier is the evidence's own,
                // the frame names the state, and the budget is recorded.
                assert_eq!(
                    prov.tier(),
                    prov.evidence().tier(),
                    "UP1: the tier is a derived view of the evidence"
                );
                assert_eq!(prov.frame().seat, p.seat, "UP1: the frame names the seat");
                assert_eq!(
                    prov.frame().trick,
                    p.trick,
                    "UP1: the frame names the trick"
                );
                assert_eq!(prov.frame().ply, p.ply, "UP1: the frame names the ply");
                assert_eq!(
                    prov.frame().budget,
                    b.label,
                    "UP1: the frame names the budget the decision was taken under"
                );
                assert!(
                    prov.frame().legal_actions >= 1,
                    "UP1: a decided state holds a legal action"
                );
                assert!(
                    !prov.authority().is_empty(),
                    "UP1: every decision names its authority"
                );
                // The spend is a measurement: a tier that spent nothing
                // reports nothing, and a fallback reports exactly its one
                // consultation.
                if matches!(prov.evidence(), Evidence::Field { .. }) {
                    assert_eq!(
                        prov.spend().field_reads,
                        1,
                        "UP1: the fallback spends exactly one field consultation"
                    );
                }
                if matches!(prov.evidence(), Evidence::Decided { .. }) {
                    assert_eq!(
                        prov.spend().total(),
                        0,
                        "UP1: the decided-arithmetic tier is free"
                    );
                }
                if !tiers_seen.contains(&prov.tier()) {
                    tiers_seen.push(prov.tier());
                }
            }
        }
    }
    assert!(
        decisions >= 3 * 7 * 4,
        "UP1: the sweep covers a substantial number of states, got {decisions}"
    );
    // The sweep must actually exercise more than one tier, or totality is
    // being demonstrated by a single always-answering path.
    assert!(
        tiers_seen.len() >= 3,
        "UP1: the sweep reaches at least three tiers, saw {tiers_seen:?}"
    );
    assert!(
        tiers_seen.contains(&Tier::FieldFallback),
        "UP1: the starved rung reaches the total fallback"
    );
    assert!(
        tiers_seen.contains(&Tier::DecidedArithmetic),
        "UP1: the free arithmetic tier fires"
    );
    assert!(
        tiers_seen.contains(&Tier::EndgameExact),
        "UP1: the exact tier fires where the fiber affords it"
    );
    assert!(
        tiers_seen.contains(&Tier::MiddlegameMixture),
        "UP1: the model-space tier fires on the rung that affords it and not the \
         world-space one"
    );
}

#[test]
fn up1_no_swallowed_instrument_refusal_in_the_source() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/solver/unified.rs"),
    )
    .expect("the module source is readable");
    for (n, line) in source.lines().enumerate() {
        let code = match line.find("//") {
            Some(i) => &line[..i],
            None => line,
        };
        for token in ["unwrap", "panic!", "unreachable!", "todo!"] {
            assert!(
                !code.contains(token),
                "UP1: solver/unified.rs holds no `{token}` (line {}): {line}",
                n + 1
            );
        }
        if code.contains("expect(") {
            assert!(
                line.contains("rules invariant"),
                "UP1: every `expect` in solver/unified.rs is annotated as a rules \
                 invariant, never applied to an instrument result (line {}): {line}",
                n + 1
            );
        }
    }
}

// ---------------------------------------------------------------------------
// UP2 — the endgame receipts are CONSUMED, and the instruments are pure.
// ---------------------------------------------------------------------------

/// U0's census on one root, rendered so two runs can be compared byte for
/// byte.
fn census_of(r: &Receipt, hand_id: usize, trick_no: usize) -> (Vec<GodGapCoordinate>, String) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    let spec = GodGapSpec {
        exact_fiber_cap: 40_000,
        profile_fiber_cap: 12_000,
        doom: DoomSpec {
            node_budget: 10_000_000,
            walk_cap: 1_000_000,
            max_level: 3,
            critical: DominoSet::EMPTY,
            descend_top: None,
        },
    };
    let w = GodGapWalk {
        oracle: &oracle,
        root: &root,
        position: &position,
        field: &field,
        spec: &spec,
    };
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    let coordinates = w.census(&mut progress);
    let rendered = format!("{coordinates:?}");
    (coordinates, rendered)
}

/// The store U0's census fills for one root, under UP0's own declared
/// identity — the one construction site both sides use.
fn seeded_store(r: &Receipt, hand_id: usize, trick_no: usize) -> (ReceiptStore, Vec<String>) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let field = FieldModel::new(field_spec_level0());
    let identity = fixed_field_identity(&root, &position, &field);
    let mut state = ProofState::open(&root, &position, identity.clone());
    let (coordinates, _) = census_of(r, hand_id, trick_no);
    let mut labels = Vec::new();
    for c in &coordinates {
        labels.push(c.result.label().to_string());
        for fact in coordinate_facts(c) {
            let outcome = state.install(&identity, fact);
            assert!(
                outcome.is_ok(),
                "UP2: a God-gap fact installs under the player's own identity: {outcome:?}"
            );
        }
    }
    let mut store = ReceiptStore::new();
    store.seed(state);
    (store, labels)
}

#[test]
fn up2_a_seeded_god_tight_root_is_consumed_not_recomputed() {
    let r = receipt();
    // h10-t6 is deliberately NOT here: its contract is already settled at
    // the root, so the free arithmetic tier answers before the store is
    // ever consulted — which is the cascade working, and is separately
    // pinned below.
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6)] {
        let (store, labels) = seeded_store(&r, hand_id, trick_no);
        assert!(
            labels.iter().all(|l| l == "GodTightPolicy"),
            "UP2: the seeded root is God-tight at every action, got {labels:?}"
        );
        // The consumed instruments, before and after the decision.
        let (_, before) = census_of(&r, hand_id, trick_no);
        let (plays, _) = walk(&r, hand_id, trick_no, &lean(), store, 1);
        let (_, after) = census_of(&r, hand_id, trick_no);
        assert_eq!(
            before, after,
            "UP2: the God-gap and doom instruments are byte-identical either side of a \
             unified decision (h{hand_id}-t{trick_no})"
        );

        let decision = &plays[0].decision;
        let prov = decision.provenance();
        assert_eq!(
            prov.tier(),
            Tier::EndgameExact,
            "UP2: a proved root answers at the endgame tier (h{hand_id}-t{trick_no})"
        );
        let Evidence::Consumed(receipt_view) = prov.evidence() else {
            panic!(
                "UP2: a seeded proved root is CONSUMED, not re-enumerated: {:?}",
                prov.evidence()
            );
        };
        assert_eq!(
            prov.spend().total(),
            0,
            "UP2: consuming a receipt spends no field consultation"
        );
        assert_eq!(
            receipt_view.value, receipt_view.upper,
            "UP2: the consumed decision re-prices to the God upper — its executable \
             floor meets the action's own upper (h{hand_id}-t{trick_no})"
        );

        // The value is U0's own number for that action, recomputed here
        // from the census rather than quoted.
        let (coordinates, _) = census_of(&r, hand_id, trick_no);
        let matching = coordinates
            .iter()
            .find(|c| c.context.root_action == decision.action())
            .expect("the consumed action is a census coordinate");
        assert_eq!(
            receipt_view.value, matching.upper.value,
            "UP2: the consumed value IS the God upper of the chosen action"
        );
        assert_eq!(
            prov.recursion(),
            Recursion::BackwardWorld,
            "UP2: a consumed receipt stands in the backward world-space recursion"
        );
    }
}

#[test]
fn up2_a_root_decided_by_arithmetic_never_reaches_the_store() {
    let r = receipt();
    // h10-t6: the declaring side has already banked its bid, so the pmake
    // indicator is settled for every continuation. The cascade's free
    // tier answers and the seeded God-tight receipts stay unread — the
    // ordering "deepest certainty first" doing exactly what it says.
    let (store, labels) = seeded_store(&r, 10, 6);
    assert!(labels.iter().all(|l| l == "GodTightPolicy"));
    let (plays, _) = walk(&r, 10, 6, &lean(), store, 1);
    let prov = plays[0].decision.provenance();
    assert_eq!(prov.tier(), Tier::DecidedArithmetic);
    assert_eq!(prov.spend().total(), 0);
    let Evidence::Decided { settled, .. } = prov.evidence() else {
        panic!("UP2: a settled root answers with the arithmetic");
    };
    assert_eq!(
        *settled,
        Some(true),
        "UP2: and the settled indicator is the viewer's own objective"
    );
}

#[test]
fn up2_an_unseeded_root_enumerates_and_deposits() {
    let r = receipt();
    // Same root, empty store: the decision must ENUMERATE (a different
    // evidence variant at the same tier), spend real consultations, and
    // leave the store holding what it established.
    let (plays, _) = walk(&r, 5, 6, &lean(), ReceiptStore::new(), 1);
    let prov = plays[0].decision.provenance();
    assert_eq!(prov.tier(), Tier::EndgameExact);
    let Evidence::Enumerated {
        optimum_mass,
        repriced_mass,
        fiber_mass,
        ..
    } = prov.evidence()
    else {
        panic!(
            "UP2: an unseeded affordable root enumerates: {:?}",
            prov.evidence()
        );
    };
    assert_eq!(
        optimum_mass, repriced_mass,
        "UP2: the §63 re-pricing law holds inside the player"
    );
    assert!(
        prov.spend().enumeration_reads > 0,
        "UP2: an enumeration spends field consultations and reports the measurement"
    );
    assert_eq!(*fiber_mass, 27, "UP2: h5-t6's fiber is 27");
}

// ---------------------------------------------------------------------------
// UP3 — the join.
// ---------------------------------------------------------------------------

#[test]
fn up3_the_carried_posterior_is_a_derived_view_of_the_line() {
    let r = receipt();
    let (hand_id, trick_no) = (8usize, 5usize);
    let oracle = SupportOracle;
    let (plays, lines) = walk(&r, hand_id, trick_no, &ample(), ReceiptStore::new(), 8);
    assert!(!lines.is_empty(), "UP3: the walk opened carried lines");

    // For every seat that carried a line, rebuild that seat's belief from
    // its root and the public line ALONE and demand the carried object
    // agree in every derived view. The carried posterior is therefore a
    // function of (root, public line) and of nothing the player stored
    // beside it.
    let field = FieldModel::new(field_spec_level0());
    let decl = r.hands[hand_id].decl;
    let mut checked = 0usize;
    for view in &lines {
        // The seat's own root: the state at its FIRST decision.
        let first = plays
            .iter()
            .position(|p| p.seat == view.seat)
            .expect("a carried line's seat decided at least once");
        let driven = plays[first].state.driven(decl);
        let (root, position) = driven_root(&driven).expect("a lawful driven root");
        assert_eq!(
            view.root_id,
            root_identity(&root, &position),
            "UP3: the carried line is rooted where the seat first decided"
        );
        let mut replay = mixture_at(&root, &position);
        let mut observations = 0usize;
        let mut focal_plays = 0usize;
        let mut falsified = false;
        for p in plays.iter().skip(first) {
            let tile = p.decision.action();
            if p.seat == view.seat {
                replay = replay.focal_play(tile);
                focal_plays += 1;
            } else {
                let supported = replay.branch_masses(&oracle);
                if !supported.iter().any(|(t, _)| *t == tile) {
                    // The library was falsified on this line; the carried
                    // object retires at exactly this point and so does
                    // the independent replay.
                    falsified = true;
                    break;
                }
                replay = replay.observe(&oracle, tile);
                observations += 1;
            }
        }
        assert_eq!(
            falsified, view.falsified,
            "UP3: the two agree on whether the library was falsified (seat {})",
            view.seat
        );
        if falsified {
            checked += 1;
            continue;
        }
        assert_eq!(
            (observations, focal_plays),
            (view.observations, view.focal_plays),
            "UP3: the two agree on how the line was advanced (seat {})",
            view.seat
        );
        assert_eq!(
            replay.history(),
            view.history.as_slice(),
            "UP3: same public history (seat {})",
            view.seat
        );
        assert_eq!(
            replay.profiles().len(),
            view.live_profiles,
            "UP3: same live profiles (seat {})",
            view.seat
        );
        assert_eq!(
            replay.weighted_total(&oracle),
            view.weighted_total,
            "UP3: same augmented mass (seat {})",
            view.seat
        );
        assert_eq!(
            replay.seat_type_marginals(&oracle),
            view.marginals,
            "UP3: same per-seat posterior type marginals (seat {})",
            view.seat
        );
        assert_eq!(
            replay.posterior_profile_masses(&oracle),
            view.posterior_masses,
            "UP3: same per-profile posterior masses (seat {})",
            view.seat
        );
        checked += 1;
    }
    assert!(
        checked >= 2,
        "UP3: at least two seats' lines were checked, got {checked}"
    );
    // The posterior must actually have MOVED somewhere on this line, or
    // the derived-view law is being demonstrated against a constant.
    assert!(
        lines
            .iter()
            .any(|v| v.observations > 0 && v.weighted_total > 0),
        "UP3: at least one carried line folded in a real observation"
    );

    // The same law against MB1's OWN instrument: `trace_heaviest_line`
    // descends by focal choice and heaviest merged branch, and a manual
    // replay of the actions it reports must land on the same posterior.
    let (root, position) = root_at(&r, hand_id, trick_no);
    let model = mixture_at(&root, &position);
    let (traced, trace) = trace_heaviest_line(&oracle, &model, &field, 4);
    assert!(
        trace.depth() > 0,
        "UP3: the traced line reached a hidden step"
    );
    let mut manual = mixture_at(&root, &position);
    let mut steps = trace.steps.iter();
    loop {
        if manual.history().len() >= manual.total_plays() {
            break;
        }
        if manual.seat_to_move() == root.kernel().viewer() {
            let tile = manual.focal_choice(&field);
            manual = manual.focal_play(tile);
            continue;
        }
        let Some(step) = steps.next() else { break };
        assert_eq!(
            manual.history(),
            step.history.as_slice(),
            "UP3: the manual replay tracks the trace's own history"
        );
        manual = manual.observe(&oracle, step.observed);
    }
    assert_eq!(
        manual.history(),
        traced.history(),
        "UP3: replaying the trace's public line reaches the trace's own state"
    );
    assert_eq!(
        manual.profiles().len(),
        traced.profiles().len(),
        "UP3: and the same live profiles"
    );
    assert_eq!(
        manual.weighted_total(&oracle),
        traced.weighted_total(&oracle),
        "UP3: and the same augmented mass — the posterior is a derived view of the line"
    );
    let a = manual.seat_type_marginals(&oracle);
    let b = traced.seat_type_marginals(&oracle);
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        assert_eq!(x.0, y.0, "UP3: the same hidden seats");
        assert_eq!(x.1, y.1, "UP3: the same per-seat type marginals");
    }
}

#[test]
fn up3_the_value_move_specimen_is_pinned() {
    let r = receipt();
    // h3-t5: the posterior moves the VALUE without moving the argmax —
    // MB1's "values move before argmaxes do", now on a played line.
    let (plays, _) = walk(&r, 3, 5, &ample(), ReceiptStore::new(), 2);
    let second = &plays[1];
    let join = second
        .decision
        .provenance()
        .posterior()
        .join
        .as_ref()
        .expect("UP3: the ample rung takes a join reading here");
    assert_eq!(second.seat, 1, "UP3: the specimen is seat 1's decision");
    assert_eq!(
        join.fixed_field_value,
        BigRational::new(BigInt::from(4), BigInt::from(5)),
        "UP3: the fixed-field exact optimum at the specimen is 4/5"
    );
    assert_eq!(
        join.mixture_value,
        BigRational::new(BigInt::from(29), BigInt::from(40)),
        "UP3: Q(nu) under the carried posterior at the specimen is 29/40"
    );
    assert!(join.value_moved, "UP3: the value moved");
    assert!(
        !join.argmax_flipped,
        "UP3: and the argmax did not — the value moves first"
    );
    assert_eq!(join.mixture_action, join.fixed_field_action);
}

#[test]
fn up3_the_argmax_flip_specimen_is_pinned() {
    let r = receipt();
    // h8-t4: the one state on this corpus where the carried posterior
    // would have chosen differently. The cascade's declared order plays
    // the fixed-field answer; the reading records that the model-space
    // recursion disagreed, and by how much.
    let (plays, _) = walk(&r, 8, 4, &ample(), ReceiptStore::new(), 3);
    let third = &plays[2];
    assert_eq!(third.seat, 3, "UP3: the flip specimen is seat 3's decision");
    assert_eq!(third.trick, 4);
    assert_eq!(third.ply, 2);
    let prov = third.decision.provenance();
    let join = prov
        .posterior()
        .join
        .as_ref()
        .expect("UP3: the ample rung takes a join reading here");
    assert!(
        join.argmax_flipped,
        "UP3: the argmax flipped at the specimen"
    );
    assert_eq!(
        join.fixed_field_value,
        BigRational::new(BigInt::from(173), BigInt::from(216)),
        "UP3: the fixed-field exact optimum is 173/216"
    );
    assert_eq!(
        join.mixture_value,
        BigRational::new(BigInt::from(617), BigInt::from(864)),
        "UP3: Q(nu) under the carried posterior is 617/864"
    );
    assert!(
        join.mixture_value < join.fixed_field_value,
        "UP3: the two are values against different opponent models, and here the \
         model-space one is the lower number"
    );
    assert_eq!(
        third.decision.action(),
        join.fixed_field_action,
        "UP3: the declared cascade plays tier (b)'s answer; the flip is RECORDED, not \
         acted on. Whether it should be is a sequencing question this slice does not \
         settle."
    );
    assert_ne!(join.mixture_action, join.fixed_field_action);
}

// ---------------------------------------------------------------------------
// UP4 — budget honesty.
// ---------------------------------------------------------------------------

#[test]
fn up4_a_starved_budget_falls_through_with_every_refusal_typed() {
    let r = receipt();
    let starved = MoveBudget::starved("starved");
    let (plays, _) = walk(&r, 3, 5, &starved, ReceiptStore::new(), usize::MAX);
    let mut fell_through = 0usize;
    for p in &plays {
        let prov = p.decision.provenance();
        if prov.tier() == Tier::DecidedArithmetic {
            continue;
        }
        fell_through += 1;
        assert_eq!(
            prov.tier(),
            Tier::FieldFallback,
            "UP4: under a starved budget every undecided state reaches the fallback"
        );
        assert_eq!(
            prov.recursion(),
            Recursion::Forward,
            "UP4: and the fallback is the FORWARD recursion, named as such"
        );
        let Evidence::Field { field_id } = prov.evidence() else {
            panic!("UP4: the fallback carries the field's own name");
        };
        assert!(
            field_id.starts_with("field:level0:"),
            "UP4: the named field is the declared one, got {field_id}"
        );
        assert!(
            !prov.refusals().is_empty(),
            "UP4: reaching the fallback means at least one tier refused, and it is typed"
        );
        let text = format!("{:?}", prov.refusals());
        assert!(
            text.contains("EnumerationUnaffordable"),
            "UP4: the exact tier's refusal names the fiber and the cap: {text}"
        );
    }
    assert!(
        fell_through > 0,
        "UP4: the starved walk contains undecided states"
    );
}

#[test]
fn up4_the_same_state_under_the_same_budget_yields_the_same_decision() {
    let r = receipt();
    for b in [lean(), ample()] {
        let (a, _) = walk(&r, 8, 5, &b, ReceiptStore::new(), 4);
        let (c, _) = walk(&r, 8, 5, &b, ReceiptStore::new(), 4);
        assert_eq!(a.len(), c.len());
        for (x, y) in a.iter().zip(c.iter()) {
            assert_eq!(
                x.decision, y.decision,
                "UP4: a decision is a function of the state and the declared budget — \
                 identical provenance, identical spend, identical refusals"
            );
        }
    }
}

#[test]
fn up4_an_empty_library_carries_nothing_and_changes_no_world_space_answer() {
    let r = receipt();
    // The join is not free. Carrying a model belief costs a support
    // classification under every live profile at EVERY ply, whether or
    // not a tier reads it — on the lean rung of the committed transcript
    // that carry is 99% of the wall, spent on a posterior nothing
    // consulted. The lever is declared, not hidden: an EMPTY type
    // library opens no line at all, and the world-space tiers, which
    // never read the posterior, answer exactly as before.
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6), (8, 5)] {
        let (carried, lines) = walk(&r, hand_id, trick_no, &lean(), ReceiptStore::new(), 6);
        let (bare, bare_lines) = walk_with(
            &r,
            hand_id,
            trick_no,
            &lean(),
            ReceiptStore::new(),
            6,
            TypeLibrary::new(vec![]),
        );
        assert!(
            !lines.is_empty(),
            "UP4: the registered library carries lines at h{hand_id}-t{trick_no}"
        );
        assert!(
            bare_lines.is_empty(),
            "UP4: an empty library carries none — no line, no carry"
        );
        assert_eq!(carried.len(), bare.len());
        for (x, y) in carried.iter().zip(bare.iter()) {
            assert_eq!(
                x.decision.action(),
                y.decision.action(),
                "UP4: the world-space tiers do not read the posterior, so dropping it \
                 changes no action they produced (h{hand_id}-t{trick_no})"
            );
            assert_eq!(
                x.decision.provenance().tier(),
                y.decision.provenance().tier(),
                "UP4: nor which tier answered"
            );
            assert_eq!(
                x.decision.provenance().evidence().value(),
                y.decision.provenance().evidence().value(),
                "UP4: nor the exact value it claimed"
            );
            assert!(
                !y.decision.provenance().posterior().carried,
                "UP4: and the provenance says plainly that nothing was carried"
            );
        }
    }
}

#[test]
fn up4_an_ample_budget_changes_no_value_a_lean_one_produced() {
    let r = receipt();
    // Refusal is a function of the declared budget and of nothing else
    // (U0's G4 shape): where BOTH rungs afford the exact tier, they must
    // report the same exact value.
    let (lean_plays, _) = walk(&r, 5, 6, &lean(), ReceiptStore::new(), 1);
    let (ample_plays, _) = walk(&r, 5, 6, &ample(), ReceiptStore::new(), 1);
    let lv = lean_plays[0].decision.provenance().evidence().value();
    let av = ample_plays[0].decision.provenance().evidence().value();
    assert_eq!(
        lv, av,
        "UP4: a bigger budget buys more tiers, never a different exact value"
    );
    assert_eq!(
        lean_plays[0].decision.action(),
        ample_plays[0].decision.action()
    );
}

// ---------------------------------------------------------------------------
// UP5 — the consumed instruments are unperturbed; the decorator is
// value-neutral.
// ---------------------------------------------------------------------------

#[test]
fn up5_the_consumed_instruments_are_unperturbed() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());

    // MB1's model recursion, before and after a unified decision.
    let (root, position) = root_at(&r, 8, 5);
    let before_model = mixture_at(&root, &position);
    let mut s1 = MixtureStats::default();
    let before = before_model.mixture_response(&oracle, &mut s1);
    let before_pair = (
        before.outcome.weighted_mass,
        before.outcome.weighted_total,
        before.policy.id().to_string(),
    );

    let _ = walk(&r, 8, 5, &ample(), ReceiptStore::new(), 3);

    let after_model = mixture_at(&root, &position);
    let mut s2 = MixtureStats::default();
    let after = after_model.mixture_response(&oracle, &mut s2);
    assert_eq!(
        before_pair,
        (
            after.outcome.weighted_mass,
            after.outcome.weighted_total,
            after.policy.id().to_string()
        ),
        "UP5: MB1's mixture response is unperturbed by a unified decision"
    );

    // The raw σ0 authority, likewise.
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut r1 = ResponseStats::default();
    let m1 = response_success_mass(&oracle, &belief, &field, &mut r1);
    let _ = walk(&r, 8, 5, &lean(), ReceiptStore::new(), 2);
    let mut r2 = ResponseStats::default();
    let m2 = response_success_mass(&oracle, &belief, &field, &mut r2);
    assert_eq!(m1, m2, "UP5: the σ0 exact authority is unperturbed");
    assert_eq!(r1, r2, "UP5: including its node census");
}

#[test]
fn up5_measuring_the_field_does_not_change_what_it_answers() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    // The player's tier (b) runs under a counting decorator around the
    // declared field. The decorator forwards `id`, so it IS the same
    // field — which is exactly what has to be checked, because a
    // decorator that renamed the thing it measures would be measuring
    // something else.
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let belief = FactorBelief::uniform_root(&root, &position, &field);
        let z = oracle.mass(&belief);
        let mut es = ResponseStats::default();
        let (bare_mass, bare_policy) = extract_success_policy(
            &oracle,
            &belief,
            &ExtractionSource::FullLegal,
            &field,
            &mut es,
        );
        let mut ps = RecursionStats::default();
        let bare_repriced = viewer_success_mass(&oracle, &belief, &bare_policy, &field, &mut ps);
        let bare_action = bare_policy.choice_at(&[]);

        let (plays, _) = walk(&r, hand_id, trick_no, &lean_for(z), ReceiptStore::new(), 1);
        let prov = plays[0].decision.provenance();
        if let Evidence::Enumerated {
            optimum_mass,
            repriced_mass,
            ..
        } = prov.evidence()
        {
            assert_eq!(
                *optimum_mass, bare_mass,
                "UP5: the measured field yields the bare field's optimum \
                 (h{hand_id}-t{trick_no})"
            );
            assert_eq!(*repriced_mass, bare_repriced);
            assert_eq!(
                Some(plays[0].decision.action()),
                bare_action,
                "UP5: and the bare field's argmax action"
            );
        }
    }
}

/// A rung whose enumeration cap admits exactly the given fiber.
fn lean_for(fiber: u128) -> MoveBudget {
    budget("lean-fitted", fiber, 0, 0, false)
}

// ---------------------------------------------------------------------------
// UP6 — provenance soundness.
// ---------------------------------------------------------------------------

#[test]
fn up6_every_claimed_tier_is_independently_verifiable() {
    let r = receipt();
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec_level0());
    let mut verified = [0usize; 5];
    for (b, roots, cap) in [
        (lean(), &CORPUS[..], usize::MAX),
        (ample(), &CORPUS[4..6], 4usize),
        (model_rung(), &CORPUS[2..6], 4usize),
    ] {
        for (hand_id, trick_no) in roots.iter().copied() {
            let decl = r.hands[hand_id].decl;
            let (plays, _) = walk(&r, hand_id, trick_no, &b, ReceiptStore::new(), cap);
            for p in &plays {
                let prov = p.decision.provenance();
                let driven = p.state.driven(decl);
                match prov.evidence() {
                    Evidence::Decided {
                        settled,
                        banked,
                        contract,
                        legal_actions,
                    } => {
                        verified[0] += 1;
                        let position = RootPosition {
                            decl,
                            bid: p.state.bid,
                            declaring_team: p.state.declaring_team,
                            leader: p.state.leader,
                            banked: *banked,
                            trick_plays: p.state.trick_plays.clone(),
                            prior_played: p.state.prior_played,
                            voids: p.state.voids,
                        };
                        let seat = p.state.leader.plus(p.state.trick_plays.len());
                        let independent = decided_success(&position, seat, *banked, false);
                        assert_eq!(
                            *settled, independent,
                            "UP6: a decided claim reproduces the arithmetic"
                        );
                        assert_eq!(*contract, p.state.bid);
                        assert!(
                            settled.is_some() || *legal_actions == 1,
                            "UP6: tier (a) fires only on a settled indicator or a forced play"
                        );
                    }
                    Evidence::Enumerated {
                        optimum_mass,
                        repriced_mass,
                        fiber_mass,
                        ..
                    } => {
                        verified[1] += 1;
                        let (root, position) = driven_root(&driven).expect("a lawful driven root");
                        let belief = FactorBelief::uniform_root(&root, &position, &field);
                        assert_eq!(
                            oracle.mass(&belief),
                            *fiber_mass,
                            "UP6: the claimed fiber is the state's own"
                        );
                        let mut es = ResponseStats::default();
                        let (mass, policy) = extract_success_policy(
                            &oracle,
                            &belief,
                            &ExtractionSource::FullLegal,
                            &field,
                            &mut es,
                        );
                        assert_eq!(
                            mass, *optimum_mass,
                            "UP6: a tier (b) claim carries the enumeration's exact value"
                        );
                        assert_eq!(mass, *repriced_mass);
                        assert_eq!(
                            policy.choice_at(&[]),
                            Some(p.decision.action()),
                            "UP6: and the enumeration's own argmax action"
                        );
                        // The spend is a measurement, so it is checked
                        // against what the independent re-run actually
                        // had to consult: a walk with no hidden node to
                        // classify honestly spends nothing.
                        if es.hidden_nodes > 0 {
                            assert!(
                                prov.spend().enumeration_reads > 0,
                                "UP6: a walk with hidden nodes spent consultations"
                            );
                        } else {
                            assert_eq!(
                                prov.spend().enumeration_reads,
                                0,
                                "UP6: a walk with no hidden node to classify spends nothing"
                            );
                        }
                    }
                    Evidence::Consumed(c) => {
                        verified[1] += 1;
                        assert_eq!(
                            prov.spend().total(),
                            0,
                            "UP6: a consumed claim spends nothing"
                        );
                        assert!(!c.policy.is_empty());
                    }
                    Evidence::Mixture {
                        weighted_mass,
                        weighted_total,
                        reads,
                        ..
                    } => {
                        verified[2] += 1;
                        assert!(
                            *reads > 0,
                            "UP6: a tier (c) claim carries the ledger's measured spend"
                        );
                        assert_eq!(
                            prov.spend().mixture_reads,
                            *reads,
                            "UP6: and the spend agrees with the evidence"
                        );
                        assert!(*weighted_total > 0);
                        assert!(weighted_mass <= weighted_total);
                        assert_eq!(prov.recursion(), Recursion::BackwardModel);
                    }
                    Evidence::CertifiedRegret {
                        certified_regret, ..
                    } => {
                        verified[3] += 1;
                        assert!(
                            *certified_regret <= b.regret_acceptance,
                            "UP6: a tier (d) claim is within its declared acceptance"
                        );
                    }
                    Evidence::Field { field_id } => {
                        verified[4] += 1;
                        assert_eq!(field_id, SlicePolicy::id(&field));
                        assert_eq!(prov.spend().field_reads, 1);
                    }
                }
            }
        }
    }
    assert!(
        verified[0] > 0 && verified[1] > 0 && verified[2] > 0 && verified[4] > 0,
        "UP6: the sweep verified the free tier, both endgame-exact evidences, the \
         model-space tier and the fallback, got {verified:?}"
    );
}

#[test]
fn up6_the_model_tier_answers_where_the_world_tier_refused() {
    let r = receipt();
    let oracle = SupportOracle;
    // On the model rung the world-space tier refuses structurally and the
    // model-space tier answers. The claim it carries is re-derived here
    // from an independently constructed belief at the same root.
    let (plays, _) = walk(&r, 5, 6, &model_rung(), ReceiptStore::new(), 1);
    let prov = plays[0].decision.provenance();
    assert_eq!(prov.tier(), Tier::MiddlegameMixture);
    assert_eq!(prov.recursion(), Recursion::BackwardModel);
    assert_eq!(
        prov.recursion().direction(),
        "backward",
        "UP6: the model-space tier is a BACKWARD recursion — over Xi = Omega x Theta"
    );
    let text = format!("{:?}", prov.refusals());
    assert!(
        text.contains("EnumerationUnaffordable"),
        "UP6: and it was entered only after the world-space tier typed its refusal: {text}"
    );
    let Evidence::Mixture {
        weighted_mass,
        weighted_total,
        reads,
        live_profiles,
        ..
    } = prov.evidence()
    else {
        panic!("UP6: the model rung answers with mixture evidence");
    };
    assert!(*reads > 0, "UP6: the ledger measured a real spend");
    assert_eq!(
        *live_profiles, 8,
        "UP6: eight profiles at an unobserved root"
    );
    let (root, position) = root_at(&r, 5, 6);
    let model = mixture_at(&root, &position);
    let mut stats = MixtureStats::default();
    let independent = model.mixture_response(&oracle, &mut stats);
    assert_eq!(
        (*weighted_mass, *weighted_total),
        (
            independent.outcome.weighted_mass,
            independent.outcome.weighted_total
        ),
        "UP6: a tier (c) claim carries the exact Q(nu) an independent walk reproduces"
    );
    assert_eq!(
        independent.policy.choice_at(&[]),
        Some(plays[0].decision.action()),
        "UP6: and the mixture argmax action"
    );
}

#[test]
fn up6_provenance_has_no_public_constructor() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/solver/unified.rs"),
    )
    .expect("the module source is readable");
    // The record of a decision must be unforgeable: no public field on
    // `Provenance` or `Decision`, and no public function anywhere in the
    // crate that returns either from parts.
    for name in ["pub struct Provenance {", "pub struct Decision {"] {
        let start = source
            .find(name)
            .unwrap_or_else(|| panic!("UP6: {name} is declared"));
        let rest = &source[start + name.len()..];
        let end = rest.find("\n}").expect("UP6: the struct block closes");
        let body = &rest[..end];
        assert!(
            !body.contains("pub "),
            "UP6: {name} holds no public field — a decision's record cannot be \
             assembled from outside the module. Body was:{body}"
        );
    }
    // `Provenance` is assembled in exactly one place. An `Evidence` value
    // is a public sum type — a reader must be able to match on it, so its
    // variants are constructible like any enum's — but a fabricated one
    // has nowhere to go, because the single assembly site is private and
    // is reached only by the cascade.
    let assembly = source.matches("provenance: Provenance {").count();
    assert_eq!(
        assembly, 1,
        "UP6: a Provenance is assembled in exactly one place in the module"
    );
    let sites = source.matches("fn finish(").count();
    assert_eq!(
        sites, 1,
        "UP6: and that place is the single private `finish`"
    );
    assert!(
        !source.contains("pub fn finish("),
        "UP6: the assembly site is private"
    );
}

#[test]
fn up6_the_tier_is_a_function_of_the_evidence_alone() {
    let r = receipt();
    // Exhaustive over the variants the corpus produces: the tier and the
    // recursion are derived, so two decisions carrying equal evidence
    // necessarily agree on both.
    let (plays, _) = walk(&r, 8, 5, &ample(), ReceiptStore::new(), 6);
    for p in &plays {
        let prov = p.decision.provenance();
        let e = prov.evidence().clone();
        assert_eq!(prov.tier(), e.tier());
        assert_eq!(prov.recursion(), e.recursion());
        match e.tier() {
            Tier::DecidedArithmetic | Tier::FieldFallback => {
                assert!(
                    e.value().is_none(),
                    "UP6: the free tier and the fallback claim no value"
                );
            }
            _ => {
                let v = e.value().expect("UP6: a value-claiming tier carries one");
                assert!(v >= ratio(0, 1) && v <= ratio(1, 1));
            }
        }
    }
}
