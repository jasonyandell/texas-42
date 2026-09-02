//! Gates for MB1 — the model-belief recursion joins the solver [L2
//! thread]: `solver::model_recursion`, plus the MB0-module extensions
//! it rests on (the read ledger, the typed budget refusals, the default
//! positive-support tightening, and the §16/§23 repricing identity).
//!
//! Mathematical source: `walt/math/model_belief_base_player_v0.1.md`
//! §§16–23 and §§29–33, under rulings MB-A1..A8
//! (`walt/CENSUS-RULINGS.md`); brief `walt/briefs/BRIEF-MB1.md`;
//! U0's field-specificity flag (`walt/briefs/U0-REPORT.md`, SC-A7).
//!
//! DECLARED TEST EPOCH (MB0's, unchanged): F₀ = σ0 = `Level0 { n0 = 2 }`,
//! F₁ = `Level1 { n_outer = 2, n0 = 2 }`, both `TieRule::LowestTileIndex`,
//! persistence per hand; the prior is ν = (1/2, 1/2) per hidden seat,
//! independent (integer weights 1, denominator 8 over three hidden
//! seats); `SupportOracle`; frozen `verify_player` receipt roots.
//!
//! The gates:
//! - M1 recursion-versus-enumeration parity on the full MB0 corpus:
//!   augmented masses, per-root-action `Q_a`, `U^sep_a`, the selected
//!   action, and the carried posterior after an observation, each
//!   against an INDEPENDENT (ω,θ) pair enumeration written here.
//! - M2 the §16/§23 repricing identity and the §21 envelope: a stored
//!   response vector reprices any ν by dot product, and an envelope of
//!   such vectors reproduces the exact response at every point of a
//!   swept rational grid, walking once per facet.
//! - M3 point-mass collapse INSIDE the recursion: δ endpoints reproduce
//!   fixed-field authorities at depth, after real observations, not only
//!   at roots.
//! - M4 budget refusals typed and honest: refusal is a function of the
//!   declared ceiling and of nothing else, a refused coordinate carries
//!   no value, and a refused coordinate proposes no fact.
//! - M5 the instruments MB1 consumes are unperturbed: item 3's
//!   tightening is exactness-neutral on a specimen where it actually
//!   drops entries, and the doom census is bit-identical either side of
//!   a model census (§47/SC-A3, checked from the MB1 side).
//! - M6 the earlier-root finding, pinned: the strict fusion price at
//!   trick 4 with its exact value, the vacuity discipline that keeps it
//!   honest, and the §19 ν-invariance corollary that says MB0's zeros
//!   could never have been moved by re-weighting.
//! - M7 the field-identity fence: unconstructible-by-API on the
//!   coupling side, identity-rejected on the §49 store side.

mod common;

use std::rc::Rc;

use common::receipt;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::doom::{doom_enumeration, DoomSpec};
use walt::solver::factor_belief::{
    response_success_mass, ExactCoverOracle, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::model_belief::{
    BehaviorType, MixtureRefusal, MixtureStats, ModelBelief, PersistenceScope, SeatTypePrior,
};
use walt::solver::model_recursion::{
    column_of, couple_fixed_field_fact, mixture_field_id, mixture_identity, model_census,
    response_vector, sweep_envelope, trace_heaviest_line, two_type_grid, ActionCoordinate,
    CensusBudget, CouplingRefusal, FieldCoupling, ModelBeliefProducer, ModelFieldId,
    PointMassWitness, ResponseEnvelope,
};
use walt::solver::policy::{DecisionMode, TieRule};
use walt::solver::proof_state::{ProofProducer, ProofState, Reject};

/// MB0's six roots: (hand, trick, fiber). M1 runs the whole corpus.
const MB0_ROOTS: [(usize, usize, u128); 6] = [
    (12, 6, 6),
    (10, 6, 19),
    (5, 6, 27),
    (4, 6, 90),
    (8, 5, 92),
    (3, 5, 200),
];

/// The trick-4 specimen M6 pins: the smallest receipt fiber one stratum
/// earlier than anything MB0 entered.
const T4_SPECIMEN: (usize, usize, u128) = (8, 4, 1_200);

/// The trick-4 root where the whole fiber is already decided — the
/// vacuity control, free to walk (U0's h12-t4, the degenerate one).
const T4_VACUOUS: (usize, usize) = (12, 4);

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

fn level1_spec() -> FieldSpec {
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
            level0_spec(),
            PersistenceScope::PerHand,
        )),
        Rc::new(BehaviorType::from_field(
            level1_spec(),
            PersistenceScope::PerHand,
        )),
    )
}

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn half_half(
    root: &CanonicalRoot,
    position: &RootPosition,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(f0), 1), (Rc::clone(f1), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

fn delta(
    root: &CanonicalRoot,
    position: &RootPosition,
    behavior: &Rc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Rc::clone(behavior), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

// ---------------------------------------------------------------------------
// The independent (ω,θ) pair enumerator — written here so M1's authority
// is not the machinery it gates.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Pub {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played_by: [DominoSet; Seat::COUNT],
    history: Vec<Domino>,
}

impl Pub {
    fn start(position: &RootPosition) -> Pub {
        assert!(
            position.trick_plays.is_empty(),
            "the frozen fixtures are trick-start roots"
        );
        Pub {
            leader: position.leader,
            plays: Vec::new(),
            banked: position.banked,
            played_by: [DominoSet::EMPTY; Seat::COUNT],
            history: Vec::new(),
        }
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        let seat = self.seat();
        assert!(
            self.played_by[seat.index()].insert(tile),
            "a tile is played once"
        );
        self.plays.push(tile);
        self.history.push(tile);
        if self.plays.len() == 4 {
            let doms: [Domino; 4] = core::array::from_fn(|i| self.plays[i]);
            let trick = Trick::new(self.leader, doms).expect("four distinct tiles");
            let winner = trick.winner(position.decl);
            self.banked[winner.team().index()] += trick.points();
            self.leader = winner;
            self.plays.clear();
        }
    }

    fn record<'a>(&'a self, position: &'a RootPosition) -> PublicRecord<'a> {
        PublicRecord {
            leader: self.leader,
            trick_plays: &self.plays,
            banked: self.banked,
            root: position,
            history: &self.history,
        }
    }
}

fn choice_at(
    position: &RootPosition,
    exec: &Pub,
    root_hand: DominoSet,
    policy: &dyn SlicePolicy,
) -> Domino {
    let remaining = root_hand.difference(exec.played_by[exec.seat().index()]);
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let record = exec.record(position);
    let tile = policy.choose(position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal tile");
    tile
}

/// The enumeration frame of one root: everything the pair walk needs.
struct Frame {
    position: RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total: usize,
}

fn frame_of(root: &CanonicalRoot, position: &RootPosition) -> Frame {
    Frame {
        position: position.clone(),
        viewer: root.kernel().viewer(),
        viewer_hand: root.kernel().viewer_hand(),
        total: root.kernel().viewer_hand().len()
            + root
                .kernel()
                .hidden()
                .iter()
                .map(|h| h.capacity)
                .sum::<usize>(),
    }
}

/// The (ω,θ) pair walk. Hidden nodes partition surviving pairs by each
/// pair's OWN chosen tile (the profile's field applied to the world's
/// hand — the augmented semantics, enumerated); focal nodes take one
/// action for the whole surviving set (merged before max) or follow a
/// fixed policy. Returns the weighted success count.
fn enum_walk(
    model: &ModelBelief,
    frame: &Frame,
    pairs: &[(usize, World)],
    exec: &Pub,
    fixed: Option<&dyn SlicePolicy>,
) -> u128 {
    let at_terminal = exec.history.len() == frame.total;
    if let Some(u) = decided_success(&frame.position, frame.viewer, exec.banked, at_terminal) {
        return if u {
            pairs
                .iter()
                .fold(0u128, |acc, (p, _)| acc + model.profiles()[*p].weight())
        } else {
            0
        };
    }
    let seat = exec.seat();
    if seat == frame.viewer {
        let remaining = frame
            .viewer_hand
            .difference(exec.played_by[frame.viewer.index()]);
        let led = exec
            .plays
            .first()
            .map(|d| frame.position.decl.led_context(*d));
        let legal = legal_plays(frame.position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let descend = |tile: Domino| {
            let mut child = exec.clone();
            child.play(&frame.position, tile);
            enum_walk(model, frame, pairs, &child, fixed)
        };
        match fixed {
            Some(policy) => {
                let record = exec.record(&frame.position);
                let tile = policy.choose(frame.position.decl, remaining, legal, &record);
                assert!(legal.contains(tile), "a policy chooses a legal tile");
                descend(tile)
            }
            None => {
                let mut best: Option<u128> = None;
                for tile in legal.iter() {
                    let m = descend(tile);
                    best = Some(best.map_or(m, |b| b.max(m)));
                }
                best.expect("a legal set holds an action")
            }
        }
    } else {
        let mut groups: Vec<(Domino, Vec<(usize, World)>)> = Vec::new();
        for (p, world) in pairs {
            let field: &dyn SlicePolicy = model.profiles()[*p].field().as_ref();
            let tile = choice_at(&frame.position, exec, world.hand(seat), field);
            match groups.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push((*p, *world)),
                None => groups.push((tile, vec![(*p, *world)])),
            }
        }
        let mut mass: u128 = 0;
        for (tile, group) in groups {
            let mut child = exec.clone();
            child.play(&frame.position, tile);
            mass += enum_walk(model, frame, &group, &child, fixed);
        }
        mass
    }
}

fn all_pairs(model: &ModelBelief, root: &CanonicalRoot) -> Vec<(usize, World)> {
    let worlds: Vec<World> = root.worlds().collect();
    (0..model.profiles().len())
        .flat_map(|p| worlds.iter().map(move |w| (p, *w)))
        .collect()
}

/// The pairs surviving one observed public action at a hidden node.
fn survivors(
    model: &ModelBelief,
    frame: &Frame,
    pairs: &[(usize, World)],
    exec: &Pub,
    observed: Domino,
) -> Vec<(usize, World)> {
    let seat = exec.seat();
    pairs
        .iter()
        .filter(|(p, world)| {
            let field: &dyn SlicePolicy = model.profiles()[*p].field().as_ref();
            choice_at(&frame.position, exec, world.hand(seat), field) == observed
        })
        .copied()
        .collect()
}

// ---------------------------------------------------------------------------
// M1
// ---------------------------------------------------------------------------

/// Gate M1 — the posterior-carrying recursion against the (ω,θ) pair
/// enumeration, on the FULL MB0 corpus. Four quantities per root:
/// the augmented mass, every root action's exact `Q_a`, every root
/// action's `U^sep_a` (as the weighted sum of per-profile point-mass
/// optima, each enumerated on that profile's own pair slice), and the
/// selected action. Then the CARRIED posterior: after the heaviest
/// observation, the per-seat type marginals the recursion maintains
/// equal the weighted counts of the pairs the enumeration keeps.
#[test]
fn m1_recursion_matches_the_pair_enumeration_on_the_whole_corpus() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let mut checked_actions = 0usize;
    let mut checked_roots = 0usize;
    for (hand_id, trick_no, fiber) in MB0_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let frame = frame_of(&root, &position);
        let model = half_half(&root, &position, &f0, &f1);
        let pairs = all_pairs(&model, &root);
        assert_eq!(
            model.weighted_total(&oracle),
            8 * fiber,
            "Σ_θ w·Z_θ is the prior denominator times the fiber (h{hand_id}-t{trick_no})"
        );
        assert_eq!(
            pairs.len(),
            8 * usize::try_from(fiber).expect("fits"),
            "the augmented latent space is Ω × Θ"
        );

        let census = model_census(
            &oracle,
            &format!("h{hand_id}-t{trick_no}"),
            &model,
            CensusBudget::default(),
        );
        assert_eq!(census.refusals(), 0, "an uncapped census does not refuse");
        assert_eq!(census.fiber, fiber, "the census reports the declared fiber");

        let mut best: Option<(Domino, u128)> = None;
        for coordinate in &census.coordinates {
            let p = coordinate.priced().expect("uncapped");
            let mut exec = Pub::start(&position);
            exec.play(&position, p.action);
            // Q_a against the enumeration.
            let q = enum_walk(&model, &frame, &pairs, &exec, None);
            assert_eq!(
                p.q.0, q,
                "Q_a matches the (ω,θ) enumeration (h{hand_id}-t{trick_no} action {})",
                p.action
            );
            assert_eq!(p.q.1, model.weighted_total(&oracle), "shared denominator");
            // U^sep_a: per profile, that profile's own point-mass optimum.
            let mut usep = 0u128;
            for (index, entry) in model.profiles().iter().enumerate() {
                let slice: Vec<(usize, World)> =
                    pairs.iter().filter(|(p, _)| *p == index).copied().collect();
                let q_theta = enum_walk(&model, &frame, &slice, &exec, None);
                // The slice is already prior-weighted by `enum_walk`, so
                // divide back out to compare with the raw optimum.
                assert_eq!(entry.weight(), 1, "the declared prior has unit weights");
                assert_eq!(
                    p.point_mass_optima[index], q_theta,
                    "q_a(θ) matches the single-profile enumeration \
                     (h{hand_id}-t{trick_no} action {} profile {index})",
                    p.action
                );
                usep += q_theta;
            }
            assert_eq!(p.usep.0, usep, "U^sep_a is the weighted sum of the optima");
            assert!(p.usep.0 >= p.q.0, "Theorem 18.1");
            assert_eq!(p.phi.0, p.usep.0 - p.q.0, "Φ_a = U^sep_a − Q_a exactly");
            if best.is_none_or(|(_, m)| p.q.0 > m) {
                best = Some((p.action, p.q.0));
            }
            checked_actions += 1;
        }

        // The selected action: the recursion's root argmax under the
        // declared lowest-tile-index tie rule.
        let mut stats = MixtureStats::default();
        let response = model.mixture_response(&oracle, &mut stats);
        let (best_action, best_mass) = best.expect("a root holds an action");
        assert_eq!(
            response.outcome.weighted_mass, best_mass,
            "the root response is the max over root actions"
        );
        match response.policy.choice_at(&[]) {
            Some(chosen) => assert_eq!(
                chosen, best_action,
                "the extracted policy's root choice is the argmax action \
                 (h{hand_id}-t{trick_no})"
            ),
            None => {
                // A root the decided cutoff settles before any focal
                // node: there is no choice to record, and every action
                // carries the same decided value. Asserting a choice
                // here would be asserting a fabricated one.
                for coordinate in &census.coordinates {
                    let p = coordinate.priced().expect("uncapped");
                    assert_eq!(
                        p.q.0, best_mass,
                        "a root-decided fiber values every action alike \
                         (h{hand_id}-t{trick_no})"
                    );
                }
            }
        }

        // The CARRIED posterior at the first hidden state of the
        // heaviest line.
        let focal = FixedPreference::lowest_first("focal:lowest-first");
        let (_, trace) = trace_heaviest_line(&oracle, &model, &focal, 1);
        if let Some(step) = trace.steps.first() {
            let mut exec = Pub::start(&position);
            let mut live = pairs.clone();
            for tile in &step.history {
                if exec.seat() == frame.viewer {
                    exec.play(&position, *tile);
                } else {
                    live = survivors(&model, &frame, &live, &exec, *tile);
                    exec.play(&position, *tile);
                }
            }
            // Branch masses at this hidden state.
            let seat = exec.seat();
            let mut enumerated: Vec<(Domino, u128)> = Vec::new();
            for (p, world) in &live {
                let field: &dyn SlicePolicy = model.profiles()[*p].field().as_ref();
                let tile = choice_at(&position, &exec, world.hand(seat), field);
                let w = model.profiles()[*p].weight();
                match enumerated.iter_mut().find(|(t, _)| *t == tile) {
                    Some((_, acc)) => *acc += w,
                    None => enumerated.push((tile, w)),
                }
            }
            enumerated.sort_by_key(|(t, _)| t.index());
            assert_eq!(
                step.branches, enumerated,
                "the merged branch table is the enumeration's own partition \
                 (h{hand_id}-t{trick_no})"
            );
            // Posterior type marginals after the observation.
            let kept = survivors(&model, &frame, &live, &exec, step.observed);
            let hidden = root.kernel().hidden();
            for (slot, (seat_label, marginal)) in step.marginals.iter().enumerate() {
                assert_eq!(*seat_label, format!("{}", hidden[slot].seat));
                for (type_id, mass) in marginal {
                    let counted = kept
                        .iter()
                        .filter(|(p, _)| model.profiles()[*p].types()[slot].id() == *type_id)
                        .fold(0u128, |acc, (p, _)| acc + model.profiles()[*p].weight());
                    assert_eq!(
                        *mass, counted,
                        "the carried posterior marginal is the surviving pairs' \
                         weighted count (h{hand_id}-t{trick_no} slot {slot})"
                    );
                }
            }
            assert_eq!(
                step.weighted_total,
                kept.iter()
                    .fold(0u128, |acc, (p, _)| acc + model.profiles()[*p].weight()),
                "the post-observation weighted total is the surviving pair count"
            );
        }
        checked_roots += 1;
    }
    assert_eq!(checked_roots, 6, "the whole MB0 corpus");
    assert_eq!(
        checked_actions, 14,
        "MB0's fourteen root-action coordinates"
    );
}

// ---------------------------------------------------------------------------
// M2
// ---------------------------------------------------------------------------

/// Gate M2 — §16/§23 repricing and the §21 envelope.
///
/// Part one: a stored response vector reprices any ν by dot product.
/// A fixed policy's per-profile masses are walked ONCE; then for every
/// point of a swept rational grid the dot-product value is compared
/// against a full walk under a model belief actually built with those
/// weights. Exact, both sides, no division.
///
/// Part two: an envelope of response vectors reproduces the EXACT
/// mixture response at every grid point. `sweep_envelope` walks at
/// every point on purpose and audits the envelope's prediction against
/// the walk — §21 (never above) at every point, exactness once the
/// facet is present — so the facet count it reports is the number of
/// walks a cheap sweep would have needed, measured rather than claimed.
#[test]
fn m2_response_vectors_reprice_by_dot_product() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let mut checked = 0usize;
    let mut total_facets = 0usize;
    let mut total_points = 0usize;
    for (hand_id, trick_no, _) in [(5usize, 6usize, 0u128), (4, 6, 0), (8, 5, 0)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let model = half_half(&root, &position, &f0, &f1);
        let action = model
            .legal_focal_actions()
            .expect("the root has the viewer to move")
            .iter()
            .next()
            .expect("a root holds a legal action");
        let at_action = model.focal_play(action);
        let totals: Vec<u128> = at_action
            .profiles()
            .iter()
            .map(|e| oracle.mass(e.belief()))
            .collect();
        let grid = two_type_grid(3, 4);
        assert_eq!(grid.len(), 3, "the endpoints are excluded by construction");
        for weights in &grid {
            assert!(
                weights.iter().all(|w| *w > 0),
                "every swept belief has full support, so no comparison silently \
                 changes the belief's support"
            );
        }

        // Part one: fixed-policy repricing.
        let mut stats = MixtureStats::default();
        let outcome = at_action.mixture_policy_mass(&oracle, &focal, &mut stats);
        for weights in &grid {
            let repriced = outcome.reprice(weights);
            let rebuilt = rebuild_with_weights(&root, &position, &f0, &f1, weights, &at_action);
            let mut s = MixtureStats::default();
            let walked = rebuilt.mixture_policy_mass(&oracle, &focal, &mut s);
            assert_eq!(
                repriced,
                (walked.weighted_mass, walked.weighted_total),
                "§16: a stored response vector reprices ν exactly \
                 (h{hand_id}-t{trick_no}, weights {weights:?})"
            );
            checked += 1;
        }

        // Part two: the §21 envelope over the same grid.
        let mut envelope = ResponseEnvelope::new(totals.clone());
        let sweep = sweep_envelope(&mut envelope, &grid, |weights| {
            let rebuilt = rebuild_with_weights(&root, &position, &f0, &f1, weights, &at_action);
            let mut s = MixtureStats::default();
            let response = rebuilt
                .mixture_response_budgeted(&oracle, None, &mut s)
                .expect("an uncapped walk does not refuse");
            let (vector, column_totals) =
                response_vector(&oracle, &at_action, &response.policy, None)
                    .expect("an uncapped fixed-policy walk does not refuse");
            assert_eq!(column_totals, totals, "one state, one set of totals");
            Ok((
                response.outcome.weighted_mass,
                column_of(&response.policy, vector, column_totals),
            ))
        })
        .expect("an uncapped sweep does not refuse");
        assert_eq!(sweep.len(), grid.len(), "every grid point is answered");
        for point in &sweep {
            let reading = envelope
                .read(&point.weights)
                .expect("a populated envelope reads");
            assert_eq!(
                reading.value, point.reading.value,
                "the completed envelope reproduces every swept point by dot \
                 product alone (h{hand_id}-t{trick_no})"
            );
        }
        let facets = sweep.iter().filter(|p| p.new_facet).count();
        assert!(facets >= 1, "a nonempty sweep discovers at least one facet");
        assert!(
            facets <= sweep.len(),
            "a facet count never exceeds the grid"
        );
        assert_eq!(
            envelope.len(),
            envelope
                .columns()
                .iter()
                .map(|c| c.policy_id.clone())
                .collect::<std::collections::BTreeSet<String>>()
                .len(),
            "an envelope holds one column per policy identity"
        );
        total_facets += facets;
        total_points += sweep.len();
    }
    assert_eq!(
        checked, 9,
        "three interior grid points on each of three roots"
    );
    assert!(
        total_facets < total_points,
        "repricing saves walks somewhere on the corpus ({total_facets} facets \
         over {total_points} points)"
    );

    // Part three (item 6): the general constructor accepts NON-PRODUCT
    // priors, and MB1 neither builds correlation machinery nor breaks
    // the interface that would carry it. A maximally correlated prior —
    // heavy on "every seat is F₀" and "every seat is F₁", light on the
    // mixed profiles — is priced exactly and reprices exactly, and it
    // is genuinely non-product: a product prior necessarily satisfies
    // w(000)·w(110) = w(100)·w(010), and this one does not.
    let (root, position) = root_at(&r, 5, 6);
    let template = half_half(&root, &position, &f0, &f1);
    let correlated: Vec<u128> = (0..8)
        .map(|i| if i == 0 || i == 7 { 5u128 } else { 1u128 })
        .collect();
    assert_ne!(
        correlated[0] * correlated[6],
        correlated[4] * correlated[2],
        "the declared prior is NOT a product prior: it fails the product \
         identity w(000)·w(110) = w(100)·w(010)"
    );
    let model = ModelBelief::from_profile_prior(
        &root,
        &position,
        template
            .profiles()
            .iter()
            .zip(correlated.iter())
            .map(|(e, w)| (e.types().to_vec(), *w))
            .collect(),
    );
    assert_eq!(
        model.prior_denominator(),
        correlated.iter().sum::<u128>(),
        "the denominator is the declared weights' own sum"
    );
    let census = model_census(&oracle, "h5-t6", &model, CensusBudget::default());
    assert_eq!(census.refusals(), 0, "a non-product prior prices exactly");
    for coordinate in &census.coordinates {
        let p = coordinate.priced().expect("uncapped");
        assert!(
            p.usep.0 >= p.q.0,
            "Theorem 18.1 holds under correlation too"
        );
        assert_eq!(p.phi.0, p.usep.0 - p.q.0);
        // And the response vector still reprices by dot product.
        let action_state = model.focal_play(p.action);
        let mut s = MixtureStats::default();
        let outcome = action_state.mixture_policy_mass(&oracle, &focal, &mut s);
        assert_eq!(
            outcome.reprice(&correlated),
            (outcome.weighted_mass, outcome.weighted_total),
            "repricing under the belief's own weights returns its own value"
        );
        // And across the product/non-product boundary: the correlated
        // belief's stored response vector reprices to the INDEPENDENT
        // belief's own walked value, because a response vector is a
        // property of the policy and the state, not of the prior.
        let uniform_state = template.focal_play(p.action);
        let mut s = MixtureStats::default();
        let uniform_walk = uniform_state.mixture_policy_mass(&oracle, &focal, &mut s);
        assert_eq!(
            outcome.reprice(&[1u128; 8]),
            (uniform_walk.weighted_mass, uniform_walk.weighted_total),
            "a response vector priced under a correlated prior reprices to the \
             independent prior's walked value: §16's linearity does not know \
             which prior produced the vector"
        );
    }
}

/// The same state as `at_action`, rebuilt from the root under declared
/// product weights — prior weights are immutable by construction
/// (persistence, MB-I2), so a reweighting is a new belief replayed down
/// the same public line, never a mutation.
fn rebuild_with_weights(
    root: &CanonicalRoot,
    position: &RootPosition,
    f0: &Rc<BehaviorType>,
    f1: &Rc<BehaviorType>,
    weights: &[u128],
    at_action: &ModelBelief,
) -> ModelBelief {
    let profiles: Vec<(Vec<Rc<BehaviorType>>, u128)> = at_action
        .profiles()
        .iter()
        .zip(weights.iter())
        .map(|(entry, w)| {
            (
                entry
                    .types()
                    .iter()
                    .map(|t| {
                        if t.id() == f0.id() {
                            Rc::clone(f0)
                        } else {
                            Rc::clone(f1)
                        }
                    })
                    .collect::<Vec<Rc<BehaviorType>>>(),
                *w,
            )
        })
        .collect();
    let mut live = ModelBelief::from_profile_prior(root, position, profiles);
    for tile in at_action.history() {
        live = live.focal_play(*tile);
    }
    live
}

// ---------------------------------------------------------------------------
// M3
// ---------------------------------------------------------------------------

/// Gate M3 — point-mass collapse INSIDE the recursion. MB0 gated δ
/// parity at roots; the recursion's claim is stronger, so the check is
/// made after real observations have moved the posterior.
///
/// Part one, the fixed-field authority at depth: a δ_{F₀} model belief
/// is descended along the heaviest line, and its exact response at that
/// depth is compared against `response_success_mass` run on an
/// INDEPENDENTLY conditioned `FactorBelief` — one built from the root
/// and walked down the same public history under the RAW σ0
/// `FieldModel`, never through the profile field. A different object
/// and a different recursion.
///
/// Part two, both endpoints at depth against the enumeration: δ_{F₀}
/// and δ_{F₁} each reproduce the (ω,θ) pair enumeration restricted to
/// their own type and to the worlds consistent with the observed
/// history. This is the endpoint the raw σ1 authority provably cannot
/// price on this root (MB0's G2 refusal set), so the enumeration is the
/// authority there — which is exactly the scope MB0 declared and MB1
/// inherits unchanged.
#[test]
fn m3_point_mass_endpoints_collapse_at_depth() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let mut checked = 0usize;
    for (hand_id, trick_no) in [(5usize, 6usize), (4, 6), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let frame = frame_of(&root, &position);
        for behavior in [&f0, &f1] {
            let model = delta(&root, &position, behavior);
            let (deep, trace) = trace_heaviest_line(&oracle, &model, &focal, 2);
            assert!(
                trace.depth() >= 1,
                "the traced line reaches a hidden state (h{hand_id}-t{trick_no})"
            );
            assert_eq!(
                deep.profiles().len(),
                1,
                "a point mass stays a point mass — persistence, never resampling"
            );
            let mut stats = MixtureStats::default();
            let deep_value = deep.mixture_response(&oracle, &mut stats);

            // Part two: the enumeration at the same depth.
            let pairs = all_pairs(&model, &root);
            let mut exec = Pub::start(&position);
            let mut live = pairs.clone();
            for tile in deep.history() {
                if exec.seat() == frame.viewer {
                    exec.play(&position, *tile);
                } else {
                    live = survivors(&model, &frame, &live, &exec, *tile);
                    exec.play(&position, *tile);
                }
            }
            let enumerated = enum_walk(&model, &frame, &live, &exec, None);
            assert_eq!(
                deep_value.outcome.weighted_mass,
                enumerated,
                "the δ endpoint reproduces the (ω,θ) enumeration at depth \
                 (h{hand_id}-t{trick_no}, type {})",
                behavior.id().short()
            );
            assert_eq!(
                deep_value.outcome.weighted_total,
                live.iter()
                    .fold(0u128, |acc, (p, _)| acc + model.profiles()[*p].weight()),
                "the augmented mass at depth is the surviving pair count"
            );

            // Part one: the RAW σ0 authority at depth, independently
            // conditioned. Only F₀ — the raw σ1 authority's refusal set
            // is MB0's G2 and is not reopened here.
            if behavior.id() == f0.id() {
                let raw = FieldModel::new(level0_spec());
                let mut belief = FactorBelief::uniform_root(&root, &position, &raw);
                let mut walk = Pub::start(&position);
                for tile in deep.history() {
                    if walk.seat() == frame.viewer {
                        belief = belief.focal_play(*tile);
                    } else {
                        belief = oracle.condition(&belief, *tile, &raw);
                    }
                    walk.play(&position, *tile);
                }
                let mut rstats = ResponseStats::default();
                let raw_value = response_success_mass(&oracle, &belief, &raw, &mut rstats);
                assert_eq!(
                    raw_value, deep_value.outcome.weighted_mass,
                    "δ_F₀ at depth reproduces the raw σ0 authority \
                     (h{hand_id}-t{trick_no})"
                );
                assert_eq!(
                    oracle.mass(&belief),
                    deep_value.outcome.weighted_total,
                    "the two authorities agree on the conditioned mass too"
                );
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 6, "both endpoints on three roots");
}

// ---------------------------------------------------------------------------
// M4
// ---------------------------------------------------------------------------

/// Gate M4 — budget refusals typed and honest, with no silent
/// truncation anywhere.
///
/// (a) A zero ceiling refuses at the root history itself, having spent
///     nothing: the check is at the node boundary, before the node
///     spends.
/// (b) A small ceiling refuses with a MEASURED spend and the public
///     history it stopped at — the refusal type has no value field, so
///     a truncated number cannot be reported even by accident.
/// (c) The refusal is a function of the DECLARED ceiling and nothing
///     else: the same root under an ample ceiling returns exactly the
///     uncapped value, and the successful walk's measured spend is
///     strictly under its ceiling.
/// (d) A refused census coordinate carries no `Φ` and the producer
///     proposes NO fact for it — a refusal is not a bound.
#[test]
fn m4_read_budget_refusals_are_typed_and_honest() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 8, 5);
    let model = half_half(&root, &position, &f0, &f1);
    let action = model
        .legal_focal_actions()
        .expect("viewer to move")
        .iter()
        .next()
        .expect("a legal action");
    let at_action = model.focal_play(action);

    // (a) a zero ceiling refuses having spent nothing.
    let mut stats = MixtureStats::default();
    let zero = at_action.mixture_response_budgeted(&oracle, Some(0), &mut stats);
    match zero {
        Err(MixtureRefusal::ReadBudget {
            spent,
            cap,
            at_history,
        }) => {
            assert_eq!(spent, 0, "the ceiling is checked before the node spends");
            assert_eq!(cap, 0);
            assert_eq!(
                at_history,
                at_action.history().to_vec(),
                "the refusal names the node it stopped at"
            );
        }
        other => panic!("a zero ceiling refuses: {other:?}", other = other.is_ok()),
    }

    // (b) a small ceiling refuses with a measured spend.
    let baseline = model.ledger().total();
    let mut stats = MixtureStats::default();
    let small = at_action.mixture_response_budgeted(&oracle, Some(64), &mut stats);
    match small {
        Err(MixtureRefusal::ReadBudget { spent, cap, .. }) => {
            assert_eq!(cap, 64);
            assert!(spent >= cap, "a refusal reports at least its ceiling");
            assert_eq!(
                spent,
                model.ledger().total() - baseline,
                "the reported spend is the ledger's own measurement"
            );
        }
        other => panic!("a 64-read ceiling refuses here: {}", other.is_ok()),
    }

    // (c) refusal is a function of the declared ceiling alone.
    let before = model.ledger().total();
    let mut stats = MixtureStats::default();
    let ample = at_action
        .mixture_response_budgeted(&oracle, Some(4_000_000), &mut stats)
        .expect("an ample ceiling does not refuse at this root");
    let spent = model.ledger().total() - before;
    assert!(
        spent < 4_000_000,
        "a successful walk's measured spend is under its ceiling ({spent})"
    );
    let mut stats = MixtureStats::default();
    let uncapped = at_action.mixture_response(&oracle, &mut stats);
    assert_eq!(
        ample.outcome, uncapped.outcome,
        "a ceiling that is not reached changes no value"
    );

    // (d) a refused coordinate carries no Φ and proposes no fact.
    let census = model_census(
        &oracle,
        "h8-t5",
        &model,
        CensusBudget {
            response_cap: Some(64),
            separated_cap: Some(64),
        },
    );
    assert_eq!(
        census.refusals(),
        census.coordinates.len(),
        "every coordinate refuses under a 64-read ceiling, and none is dropped"
    );
    for coordinate in &census.coordinates {
        assert!(
            coordinate.priced().is_none(),
            "a refused coordinate holds no priced value"
        );
        assert!(
            matches!(coordinate, ActionCoordinate::ResponseRefused { .. }),
            "the response side is what ran out first"
        );
    }
    let identity = mixture_identity(&root, &position, &model);
    let mut state = ProofState::open(&root, &position, identity);
    let results = state.run_producer(&ModelBeliefProducer::new(census));
    assert!(
        results.is_empty(),
        "a wholly refused census proposes no fact at all"
    );
    assert!(state.facts().is_empty(), "and installs none");

    // The separated side refuses on its own account too: an ample
    // response ceiling with a starved U^sep ceiling yields a coordinate
    // with Q priced and NO Φ.
    let starved = model_census(
        &oracle,
        "h8-t5",
        &model,
        CensusBudget {
            response_cap: Some(4_000_000),
            separated_cap: Some(1),
        },
    );
    assert!(
        starved
            .coordinates
            .iter()
            .all(|c| matches!(c, ActionCoordinate::SeparatedRefused { .. })),
        "a starved U^sep ceiling refuses on the upper side, with Q already priced"
    );
    assert_eq!(
        starved.strict_prices().len(),
        0,
        "no Φ is reported where the upper did not finish"
    );
}

// ---------------------------------------------------------------------------
// M5
// ---------------------------------------------------------------------------

/// Gate M5 — the instruments MB1 consumes are unperturbed.
///
/// Part one (item 3): making positive-support tightening the default on
/// the classifying entry points is EXACTNESS-NEUTRAL. On a specimen
/// where tightening actually drops entries, the merged branch masses and
/// the augmented mass are unchanged, and the typed-row census is the one
/// that moves — which is why the census is declared to count
/// positive-support rows rather than raw ones.
///
/// Part two (§47/SC-A3, from the MB1 side): the doom census is a pure
/// function of its declared inputs. `doom_enumeration` before a model
/// census and after it are bit-identical — the model-belief machinery
/// consumes doom's instrument and cannot perturb it, which is the
/// property U0's G5 asserts from the other direction.
#[test]
fn m5_the_consumed_instruments_are_unperturbed() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;

    // Part one: tightening drops entries somewhere on the corpus, and
    // changes no exact number when it does.
    let (root, position) = root_at(&r, 5, 6);
    let model = delta(&root, &position, &f0);
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let (deep, _) = trace_heaviest_line(&oracle, &model, &focal, 2);
    let entry = &deep.profiles()[0];
    let seat = deep.seat_to_move();
    let raw_support = entry
        .belief()
        .factors()
        .iter()
        .find(|f| f.seat() == seat)
        .expect("a hidden seat has a factor")
        .support();
    let positive = oracle.actor_completion_weights(entry.belief(), seat);
    let tightened = deep.branch_masses(&oracle);
    let conserved: u128 = tightened.iter().map(|(_, m)| *m).sum();
    assert_eq!(
        conserved,
        deep.weighted_total(&oracle),
        "the tightened branch table conserves the augmented mass exactly"
    );
    assert!(
        positive.len() <= raw_support.len(),
        "tightening never adds support"
    );
    // Every dropped entry had exactly zero completion weight — the
    // zero-entry law, which is what makes the drop exactness-neutral.
    for (hand, _) in &raw_support {
        if !positive.iter().any(|(h, _)| h == hand) {
            let weight = oracle.marginal(entry.belief(), seat, &|a| a == *hand);
            assert_eq!(
                weight, 0,
                "an entry tightening drops has zero exact completion weight"
            );
        }
    }

    // Part two: doom is untouched by a model census.
    let (droot, dposition) = root_at(&r, 5, 6);
    let action = legal_plays(dposition.decl, droot.kernel().viewer_hand(), None)
        .iter()
        .next()
        .expect("a legal root action");
    let raw0 = FieldModel::new(level0_spec());
    let doom_spec = DoomSpec {
        node_budget: 10_000_000,
        walk_cap: 1_000_000,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    };
    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
    let before = doom_enumeration(
        &oracle,
        &droot,
        &dposition,
        &raw0,
        action,
        &doom_spec,
        &mut progress,
    );
    let mixture = half_half(&droot, &dposition, &f0, &f1);
    let census = model_census(&oracle, "h5-t6", &mixture, CensusBudget::default());
    assert_eq!(census.refusals(), 0);
    let after = doom_enumeration(
        &oracle,
        &droot,
        &dposition,
        &raw0,
        action,
        &doom_spec,
        &mut progress,
    );
    assert_eq!(
        before, after,
        "§47/SC-A3: the doom instrument is a pure function of its declared \
         inputs and the model census does not touch it"
    );
}

// ---------------------------------------------------------------------------
// M6
// ---------------------------------------------------------------------------

/// Gate M6 — the earlier-root finding, pinned.
///
/// Part one, THE SPECIMEN. At h8-t4 — the smallest receipt fiber a full
/// stratum earlier than anything MB0 entered — the registered F₀/F₁
/// mixture has a STRICTLY POSITIVE model-fusion price at root action
/// `3-1`, pinned here as an exact rational. This is the finding MB0's
/// criterion 4 reported as absent on its own corpus, and it is present
/// one trick earlier. The gate re-derives it rather than citing the
/// probe. Only this one coordinate is re-derived, because each costs
/// about a million field consultations; the remaining three of the same
/// root are in the probe and carry the same sign.
///
/// Part two, THE VACUITY CONTROL. At h12-t4 the whole fiber is decided
/// at the root, so every coordinate's `Φ` is zero for the arithmetic
/// reason that nothing is at stake. The census types all four apart as
/// non-substantive, which is what keeps part one's finding from being
/// diluted by four free zeros — U0's degenerate-God-tightness
/// discipline, carried across intact.
///
/// Part three, THE ν-INVARIANCE COROLLARY (§19). Where the price IS
/// zero at the declared full-support ν, the census carries the common
/// optimizer, and that witness makes the zero hold at EVERY belief over
/// the same types: the swept grid reproduces `U^sep` exactly at every
/// point by repricing the single witness policy. So MB0's zeros were
/// never movable by re-weighting, and a strict specimen had to come
/// from a new root — which is where this one came from.
#[test]
fn m6_the_earlier_root_finding_is_pinned() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;

    // Part two first — it is free, and it fixes the reading of part one.
    let (vroot, vposition) = root_at(&r, T4_VACUOUS.0, T4_VACUOUS.1);
    let vmodel = half_half(&vroot, &vposition, &f0, &f1);
    let vcensus = model_census(&oracle, "h12-t4", &vmodel, CensusBudget::default());
    assert_eq!(vcensus.refusals(), 0, "a decided root costs nothing");
    assert_eq!(vcensus.reads, 0, "a root-decided fiber consults no field");
    assert_eq!(
        vcensus.strict_prices().len(),
        0,
        "nothing is at stake, so no price is strict"
    );
    assert_eq!(
        vcensus.substantive_zero_prices(),
        0,
        "and none of the zeros is evidence"
    );
    assert_eq!(
        vcensus.vacuous_zero_prices(),
        vcensus.coordinates.len(),
        "every h12-t4 coordinate is a VACUOUS zero — U^sep sits at an \
         endpoint and the equality carries no information"
    );

    // Part one: the strict specimen.
    let (root, position) = root_at(&r, T4_SPECIMEN.0, T4_SPECIMEN.1);
    let model = half_half(&root, &position, &f0, &f1);
    assert_eq!(
        oracle.mass(model.profiles()[0].belief()),
        T4_SPECIMEN.2,
        "the declared trick-4 fiber"
    );
    let specimen_action = tile_named("3-1");
    let at_action = model.focal_play(specimen_action);
    let mut stats = MixtureStats::default();
    let response = at_action
        .mixture_response_budgeted(&oracle, None, &mut stats)
        .expect("the specimen coordinate closes uncapped");
    let optima = at_action
        .point_mass_optima(&oracle, None)
        .expect("the specimen's point-mass sequence closes uncapped");
    let weights: Vec<u128> = model.profiles().iter().map(|e| e.weight()).collect();
    let usep = weights
        .iter()
        .zip(optima.iter())
        .fold(0u128, |acc, (w, (m, _))| acc + w * m);
    let q = response.outcome.weighted_mass;
    let total = response.outcome.weighted_total;
    assert!(
        usep > q,
        "THE MB1 FINDING: the model-fusion price at h8-t4 action 3-1 is \
         STRICTLY POSITIVE — U^sep {usep}/{total} against Q {q}/{total}"
    );
    assert_eq!(
        (q, usep, total),
        M6_SPECIMEN,
        "the pinned exact triple (Q, U^sep, denominator) at h8-t4 action 3-1"
    );
    assert!(
        usep > 0 && usep < total,
        "the specimen is SUBSTANTIVE — U^sep is at neither endpoint, so the \
         strict price is evidence and not arithmetic"
    );
    // §19's biconditional at a strict coordinate: no single policy
    // attains every point-mass optimum.
    assert_ne!(
        response.outcome.per_profile_mass,
        optima.iter().map(|(m, _)| *m).collect::<Vec<u128>>(),
        "Theorem 19.1: a strictly positive price means no ONE lawful policy \
         attains q_a(θ) at every type in the support"
    );

    // Part three: the ν-invariance corollary on a zero coordinate.
    let (zroot, zposition) = root_at(&r, 5, 6);
    let zmodel = half_half(&zroot, &zposition, &f0, &f1);
    let zcensus = model_census(&oracle, "h5-t6", &zmodel, CensusBudget::default());
    let zero = zcensus
        .coordinates
        .iter()
        .filter_map(ActionCoordinate::priced)
        .find(|p| p.phi.0 == 0 && p.substantive)
        .expect("h5-t6 carries a substantive zero-price coordinate");
    let witness = zero
        .common_optimizer
        .as_ref()
        .expect("§19: a zero price at full support carries its common optimizer");
    assert_eq!(
        witness.per_profile_mass, zero.point_mass_optima,
        "the witness attains every point-mass optimum simultaneously"
    );
    for weights in two_type_grid(3, 4) {
        let repriced = weights
            .iter()
            .zip(witness.per_profile_mass.iter())
            .fold(0u128, |acc, (w, m)| acc + w * m);
        let usep_at_nu = weights
            .iter()
            .zip(zero.point_mass_optima.iter())
            .fold(0u128, |acc, (w, m)| acc + w * m);
        assert_eq!(
            repriced, usep_at_nu,
            "§19 corollary: the witness attains U^sep at EVERY ν, so Φ = 0 \
             everywhere over these types and no re-weighting could have found \
             a strict specimen here (weights {weights:?})"
        );
    }
}

/// The pinned exact triple `(Q, U^sep, denominator)` of the MB1
/// specimen — h8-t4, root action 3-1, registered F₀/F₁ mixture at
/// ν = (1/2,1/2) per hidden seat, three hidden seats, eight profiles
/// over the fiber 1,200 (denominator 8 × 1,200 = 9,600). The fusion
/// price is `38/9600`, about 4‰: not large, and strictly positive,
/// which is the whole point. Regenerated by re-running the gate;
/// never hand-edited.
const M6_SPECIMEN: (u128, u128, u128) = (8_323, 8_361, 9_600);

// ---------------------------------------------------------------------------
// M7
// ---------------------------------------------------------------------------

/// Gate M7 — the field-identity fence (item 7, U0's SC-A7 flag).
///
/// Part one, the coupling side, unconstructible-by-API: `CoupledFact`
/// has private members and no public constructor, so the only way to
/// read a fixed-field fact inside the model-space recursion is
/// `couple_fixed_field_fact`. Four attempts are made and each lands on
/// its own typed answer — refused into a nondegenerate MIXTURE, refused
/// into a point mass over the wrong parent field, refused without a
/// re-run witness, refused with a DISAGREEING witness, and coupled only
/// where a real parity witness exists.
///
/// Part two, the store side, where structure cannot reach: the §49
/// store accepts any well-typed `Fact` under a matching identity, so
/// the fence there IS the identity. A model-belief proof state's
/// `field_id` is the mixture's content address, so a σ0-authored fact
/// is rejected `IdentityMismatch` by machinery that already existed —
/// and a REWEIGHTED mixture is a different identity again, because the
/// values established under it are different values.
#[test]
fn m7_the_field_identity_fence_holds_on_both_sides() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 5, 6);
    let mixture = half_half(&root, &position, &f0, &f1);
    let delta0 = delta(&root, &position, &f0);
    let sigma0 = ModelFieldId::Fixed(f0.parent_field().to_string());
    let sigma1 = ModelFieldId::Fixed(f1.parent_field().to_string());

    // The mixture identity is nobody else's.
    let mix_id = mixture_field_id(&mixture);
    let delta_id = mixture_field_id(&delta0);
    assert_ne!(mix_id.as_str(), sigma0.as_str());
    assert_ne!(mix_id.as_str(), sigma1.as_str());
    assert_ne!(mix_id.as_str(), delta_id.as_str());
    assert_eq!(mix_id.profile_count(), 8);
    assert_eq!(delta_id.profile_count(), 1);
    // A reweighted mixture is a different field.
    let reweighted = ModelBelief::from_profile_prior(
        &root,
        &position,
        mixture
            .profiles()
            .iter()
            .enumerate()
            .map(|(i, e)| (e.types().to_vec(), if i == 0 { 3u128 } else { e.weight() }))
            .collect(),
    );
    assert_ne!(
        mixture_field_id(&reweighted).as_str(),
        mix_id.as_str(),
        "a reweighted mixture is a different field identity: the values \
         established under it are different values"
    );

    // A real fact to transport, and a real parity witness for it.
    let census = model_census(&oracle, "h5-t6", &delta0, CensusBudget::default());
    let identity = mixture_identity(&root, &position, &delta0);
    let facts = ModelBeliefProducer::new(census).produce(&ProofState::open(
        &root,
        &position,
        identity.clone(),
    ));
    let fact = facts
        .first()
        .expect("a priced census proposes a fact")
        .clone();
    let mut stats = MixtureStats::default();
    let model_side = {
        let v = delta0.mixture_response(&oracle, &mut stats);
        (v.outcome.weighted_mass, v.outcome.weighted_total)
    };
    let fixed_side = {
        let raw = FieldModel::new(level0_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &raw);
        let mut rstats = ResponseStats::default();
        (
            response_success_mass(&oracle, &belief, &raw, &mut rstats),
            oracle.mass(&belief),
        )
    };
    assert_eq!(
        fixed_side, model_side,
        "the point-mass parity witness is a REAL re-run of both authorities"
    );
    let witness = PointMassWitness {
        fixed_side,
        model_side,
        behavior: f0.id(),
    };

    // (a) into a nondegenerate mixture: refused, naming the profiles.
    match couple_fixed_field_fact(fact.clone(), &sigma0, &mixture, Some(witness.clone())) {
        Err(CouplingRefusal::MixtureTarget { live_profiles, .. }) => {
            assert_eq!(live_profiles, 8);
        }
        other => panic!("a σ0 fact does not enter a mixture: {}", describe(&other)),
    }
    // (b) into a point mass over the wrong parent field: refused.
    match couple_fixed_field_fact(fact.clone(), &sigma1, &delta0, Some(witness.clone())) {
        Err(CouplingRefusal::ParentFieldMismatch { .. }) => {}
        other => panic!("σ1 is not δ_F₀'s parent: {}", describe(&other)),
    }
    // (c) without a witness: refused. Extensional equality is a claim,
    //     and an unwitnessed claim does not cross.
    match couple_fixed_field_fact(fact.clone(), &sigma0, &delta0, None) {
        Err(CouplingRefusal::MixtureTarget { .. }) => {}
        other => panic!("no witness, no coupling: {}", describe(&other)),
    }
    // (d) with a disagreeing witness: refused, reporting both sides.
    let bad = PointMassWitness {
        fixed_side: (fixed_side.0 + 1, fixed_side.1),
        model_side,
        behavior: f0.id(),
    };
    match couple_fixed_field_fact(fact.clone(), &sigma0, &delta0, Some(bad)) {
        Err(CouplingRefusal::ParityDisagreement {
            fixed_side: fs,
            model_side: ms,
        }) => {
            assert_eq!(fs, (fixed_side.0 + 1, fixed_side.1));
            assert_eq!(ms, model_side);
        }
        other => panic!(
            "a disagreeing witness does not couple: {}",
            describe(&other)
        ),
    }
    // (e) with the real witness: coupled, carrying its justification.
    let coupled = couple_fixed_field_fact(fact.clone(), &sigma0, &delta0, Some(witness.clone()))
        .expect("a real parity witness discharges the degenerate coupling");
    assert!(matches!(
        coupled.coupling(),
        FieldCoupling::PointMassParity(_)
    ));
    assert_eq!(coupled.source().as_str(), sigma0.as_str());
    assert_eq!(coupled.target().as_str(), delta_id.as_str());
    assert_eq!(coupled.fact(), &fact);

    // Part two: the §49 store's own identity fence.
    let mut state = ProofState::open(
        &root,
        &position,
        mixture_identity(&root, &position, &mixture),
    );
    let mut sigma0_identity = mixture_identity(&root, &position, &mixture);
    sigma0_identity.field_id = f0.parent_field().to_string();
    assert_eq!(
        state.install(&sigma0_identity, fact.clone()),
        Err(Reject::IdentityMismatch),
        "a σ0-authored fact is rejected by the model-belief store's identity"
    );
    assert!(
        state.facts().is_empty(),
        "a rejection changes nothing but the trace"
    );
    // And the mixture's own producer installs.
    let mixture_census = model_census(&oracle, "h5-t6", &mixture, CensusBudget::default());
    let installed = state.run_producer(&ModelBeliefProducer::new(mixture_census));
    assert!(
        installed.iter().all(Result::is_ok),
        "the mixture's own facts install under the mixture identity"
    );
    assert!(
        !state.facts().is_empty(),
        "the model-belief values are now facts in the §49 store"
    );
    let view = state.closure();
    assert!(
        !view.views.is_empty(),
        "the store's closure reads the model-belief values back"
    );
}

/// A coupling outcome as text, for panic messages only.
fn describe(
    outcome: &Result<walt::solver::model_recursion::CoupledFact, CouplingRefusal>,
) -> String {
    match outcome {
        Ok(_) => "COUPLED".to_string(),
        Err(e) => format!("{e}"),
    }
}

/// A tile named by its label, for the pinned specimen action.
fn tile_named(label: &str) -> Domino {
    (0..DominoSet::FULL.len())
        .filter_map(Domino::from_index)
        .find(|d| format!("{d}") == label)
        .expect("a named tile exists")
}
