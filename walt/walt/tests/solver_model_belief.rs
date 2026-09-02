//! Gates for the MB0 model-belief exact vertical slice [L2 thread]:
//! the field model as a persistent hidden coordinate over the existing
//! counted-belief machinery (`solver::model_belief`).
//!
//! Mathematical source: `walt/math/model_belief_base_player_v0.1.md`
//! §§5–20, §§74–76, as repaired by the intake companion (the §8
//! erratum and the corrected rung table: F₀ = σ0 = `FieldKind::Level0`,
//! F₁ = `FieldKind::Level1`), adopted by rulings MB-A1..A8
//! (`walt/CENSUS-RULINGS.md`); brief `walt/briefs/BRIEF-MB0.md`.
//!
//! DECLARED TEST EPOCH: the registered behavior types are F₀ = σ0 =
//! `Level0 { n0 = 2 }` (the C1 epoch declared field) and F₁ =
//! `Level1 { n_outer = 2, n0 = 2 }` (the declared level-1 test epoch),
//! both `TieRule::LowestTileIndex`, persistence per hand; the prior is
//! ν = (1/2, 1/2) per hidden seat, independent (integer weights 1).
//! Frozen `verify_player` receipt roots: the six enumerable roots of
//! the Slice D epoch (hands 12/10/5/4 at trick 6, hands 8/3 at trick
//! 5). The G3 persistence specimen uses SYNTHETIC declared carrier
//! types (lowest-first / highest-first fixed preferences) on a real
//! receipt root with one seat's hand pinned by a declared table — the
//! brief's "minimal synthetic carrier" option, labeled as such.
//!
//! The gates:
//! - G1 exact (ω,θ) enumeration parity on three receipt roots —
//!   masses, branch masses, posteriors, fixed-policy values, the exact
//!   mixture response, and the argmax policy's per-profile re-pricing.
//! - G2 point-mass parity both ways (MB-I5): ν = δ_{F₀} and ν = δ_{F₁}
//!   reproduce the existing fixed-field authority's value AND selected
//!   action on every tested root.
//! - G3 posterior closure (Theorem 12.1): only the acting seat's
//!   factor changes; merged branch masses conserve (MB-I6); the §9
//!   ½-vs-¼ persistent-vs-resampled separation on the specimen.
//! - G4 merge-before-max (MB-I4) and hidden-type unreadability
//!   (MB-I1): one focal consultation per information state for the
//!   whole bundle, witnessed by a counting policy.
//! - G5 exact linearity of V_ν(ρ) in ν on a swept rational grid
//!   (MB-O5), plus response-vector consistency against δ bundles.
//! - G6 the separated upper (Theorem 18.1): Q(ν) ≤ U^sep per root
//!   action everywhere; equality iff one common policy is pointwise
//!   optimal on support (Theorem 19.1), with a zero specimen and a
//!   strictly positive specimen.
//! - G7 behavior-type identity: every behavior-affecting parameter
//!   change produces a new `BehaviorTypeId`; equal construction
//!   produces equal id.

mod common;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;

use common::receipt;
use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{
    response_success_mass, viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldSpec};
use walt::solver::model_belief::{
    BehaviorType, MixtureStats, ModelBelief, PersistenceScope, SeatTypePrior,
};
use walt::solver::policy::{DecisionMode, TieRule};

/// The three enumeration-parity roots (G1) and the six tested roots
/// (G2/G6): (hand, trick, fiber).
const ENUM_ROOTS: [(usize, usize, u128); 3] = [(12, 6, 6), (10, 6, 19), (5, 6, 27)];
const TESTED_ROOTS: [(usize, usize, u128); 6] = [
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

/// F₀ = σ0, the C1 epoch declared field.
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

/// F₁, the declared level-1 test epoch.
fn level1_spec() -> FieldSpec {
    FieldSpec {
        kind: FieldKind::Level1 {
            n_outer: 2,
            n0: 2,
        },
        construction: "level1-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    }
}

/// The two registered rungs as persistent behavior types (brief item 1).
fn registered_types() -> (Arc<BehaviorType>, Arc<BehaviorType>) {
    (
        Arc::new(BehaviorType::from_field(
            level0_spec(),
            PersistenceScope::PerHand,
        )),
        Arc::new(BehaviorType::from_field(
            level1_spec(),
            PersistenceScope::PerHand,
        )),
    )
}

/// The ν = (1/2, 1/2)-per-seat independent prior over {F₀, F₁} (integer
/// weights 1; denominator 8 over three hidden seats).
fn half_half_model(
    root: &CanonicalRoot,
    position: &RootPosition,
    f0: &Arc<BehaviorType>,
    f1: &Arc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Arc::clone(f0), 1), (Arc::clone(f1), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

/// A δ point-mass prior: one type on every hidden seat.
fn delta_model(
    root: &CanonicalRoot,
    position: &RootPosition,
    behavior: &Arc<BehaviorType>,
) -> ModelBelief {
    let priors: Vec<SeatTypePrior> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| SeatTypePrior {
            seat: slot.seat,
            types: vec![(Arc::clone(behavior), 1)],
        })
        .collect();
    ModelBelief::from_independent_prior(root, position, &priors)
}

/// The test's own public-state replay — the same trick arithmetic as
/// the module walkers, built by hand so the checker is independent of
/// the machinery it gates.
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

/// One policy's choice at the current public state, from a root hand.
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

/// The (ω,θ) pair enumeration walk — the G1 oracle. Walks the shared
/// public state carrying the surviving (profile index, world) pairs;
/// hidden nodes partition pairs by each pair's OWN chosen tile (the
/// profile's field on the world's hand — the augmented semantics,
/// enumerated); focal nodes either follow the fixed policy or maximize
/// the weighted pair count over legal actions, merged across profiles
/// (one action per information state — the same lawfulness as the
/// bundle walk, from the raw side). Returns the weighted success count.
#[allow(clippy::too_many_arguments)]
fn enum_walk(
    model: &ModelBelief,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total: usize,
    pairs: &[(usize, World)],
    exec: &Pub,
    fixed: Option<&dyn SlicePolicy>,
) -> u128 {
    let at_terminal = exec.history.len() == total;
    if let Some(u) = decided_success(position, viewer, exec.banked, at_terminal) {
        return if u {
            pairs.iter().fold(0u128, |acc, (p, _)| {
                acc + model.profiles()[*p].weight()
            })
        } else {
            0
        };
    }
    let seat = exec.seat();
    if seat == viewer {
        let remaining = viewer_hand.difference(exec.played_by[viewer.index()]);
        let led = exec.plays.first().map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, remaining, led);
        assert!(!legal.is_empty(), "a seat to move holds a legal tile");
        let descend = |tile: Domino| {
            let mut child = exec.clone();
            child.play(position, tile);
            enum_walk(
                model,
                position,
                viewer,
                viewer_hand,
                total,
                pairs,
                &child,
                fixed,
            )
        };
        match fixed {
            Some(policy) => {
                let record = exec.record(position);
                let tile = policy.choose(position.decl, remaining, legal, &record);
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
            let tile = choice_at(position, exec, world.hand(seat), field);
            match groups.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push((*p, world.clone())),
                None => groups.push((tile, vec![(*p, world.clone())])),
            }
        }
        let mut mass: u128 = 0;
        for (tile, group) in groups {
            let mut child = exec.clone();
            child.play(position, tile);
            mass += enum_walk(
                model,
                position,
                viewer,
                viewer_hand,
                total,
                &group,
                &child,
                fixed,
            );
        }
        mass
    }
}

/// All (profile index, world) pairs of a model at its root.
fn all_pairs(model: &ModelBelief, root: &CanonicalRoot) -> Vec<(usize, World)> {
    let worlds: Vec<World> = root.worlds().collect();
    (0..model.profiles().len())
        .flat_map(|p| worlds.iter().map(move |w| (p, w.clone())))
        .collect()
}

/// Gate 1 — exact (ω,θ) enumeration parity on three receipt roots:
/// augmented masses, first-hidden-state branch masses and posteriors,
/// fixed-policy mixture values (per profile and weighted), the exact
/// mixture response, and the extracted argmax policy's per-profile
/// re-pricing through the existing fixed-policy recursion.
#[test]
fn enumeration_parity_over_augmented_pairs() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        let viewer_hand = root.kernel().viewer_hand();
        let total = viewer_hand.len()
            + root
                .kernel()
                .hidden()
                .iter()
                .map(|h| h.capacity)
                .sum::<usize>();
        let model = half_half_model(&root, &position, &f0, &f1);
        assert_eq!(model.profiles().len(), 8, "2^3 profiles at the root");
        assert_eq!(model.prior_denominator(), 8);
        // Augmented mass: Σ_θ w·Z_θ = 8 × the physical fiber.
        assert_eq!(model.weighted_total(&oracle), 8 * fiber);
        // Fixed-policy mixture value against the pair enumeration.
        let mut stats = MixtureStats::default();
        let outcome = model.mixture_policy_mass(&oracle, &focal, &mut stats);
        let pairs = all_pairs(&model, &root);
        assert_eq!(
            pairs.len(),
            8 * usize::try_from(fiber).expect("fits"),
            "the augmented latent space is Ω × Θ"
        );
        let enum_fixed = enum_walk(
            &model,
            &position,
            viewer,
            viewer_hand,
            total,
            &pairs,
            &Pub::start(&position),
            Some(&focal),
        );
        assert_eq!(
            outcome.weighted_mass, enum_fixed,
            "V_ν(ρ) equals the (ω,θ) enumeration (hand {hand_id} trick {trick_no})"
        );
        // Per-profile fixed-policy masses against per-profile pair sets.
        for (p, entry) in model.profiles().iter().enumerate() {
            let sub: Vec<(usize, World)> = pairs.iter().filter(|(q, _)| *q == p).cloned().collect();
            let m = enum_walk(
                &model,
                &position,
                viewer,
                viewer_hand,
                total,
                &sub,
                &Pub::start(&position),
                Some(&focal),
            );
            assert_eq!(
                outcome.per_profile_mass[p], m,
                "the §16 response coordinate equals its enumeration (profile {})",
                entry.label()
            );
        }
        // The exact mixture response against the merged-max enumeration.
        let mut rstats = MixtureStats::default();
        let response = model.mixture_response(&oracle, &mut rstats);
        let enum_best = enum_walk(
            &model,
            &position,
            viewer,
            viewer_hand,
            total,
            &pairs,
            &Pub::start(&position),
            None,
        );
        assert_eq!(
            response.outcome.weighted_mass, enum_best,
            "Q(ν) equals the (ω,θ) enumeration (hand {hand_id} trick {trick_no})"
        );
        // The argmax policy re-prices to its own per-profile masses
        // through the EXISTING fixed-policy recursion — one realizable
        // policy, never an envelope.
        for (p, entry) in model.profiles().iter().enumerate() {
            let mut vstats = RecursionStats::default();
            let repriced = viewer_success_mass(
                &oracle,
                entry.belief(),
                &response.policy,
                entry.field().as_ref(),
                &mut vstats,
            );
            assert_eq!(
                repriced, response.outcome.per_profile_mass[p],
                "the extracted mixture policy re-prices unchanged (profile {})",
                entry.label()
            );
        }
        // Branch masses and posteriors at the first hidden state: play
        // the frozen focal's root action (decidedness-independent — the
        // h12-t6 root is decided, so its argmax DAG is rightly empty),
        // then compare the merged branch table and every branch
        // posterior against the pair partition.
        let root_action = choice_at(&position, &Pub::start(&position), viewer_hand, &focal);
        let after = model.focal_play(root_action);
        let mut exec = Pub::start(&position);
        exec.play(&position, root_action);
        assert_ne!(after.seat_to_move(), viewer, "one focal play per trick here");
        let branches = after.branch_masses(&oracle);
        let mut partition: Vec<(Domino, Vec<(usize, World)>)> = Vec::new();
        for (p, world) in &pairs {
            let field: &dyn SlicePolicy = model.profiles()[*p].field().as_ref();
            let seat = exec.seat();
            let tile = choice_at(&position, &exec, world.hand(seat), field);
            match partition.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push((*p, world.clone())),
                None => partition.push((tile, vec![(*p, world.clone())])),
            }
        }
        partition.sort_by_key(|(t, _)| t.index());
        assert_eq!(
            branches,
            partition
                .iter()
                .map(|(t, g)| {
                    let mass = g.iter().fold(0u128, |acc, (p, _)| {
                        acc + model.profiles()[*p].weight()
                    });
                    (*t, mass)
                })
                .collect::<Vec<_>>(),
            "merged branch masses equal the (ω,θ) partition by public action"
        );
        for (tile, group) in &partition {
            let observed = after.observe(&oracle, *tile);
            let masses = observed.posterior_profile_masses(&oracle);
            for (p, entry) in model.profiles().iter().enumerate() {
                let survivors = group.iter().filter(|(q, _)| *q == p).count() as u128;
                let label = entry.label();
                let posterior = masses
                    .iter()
                    .find(|(l, _)| *l == label)
                    .map_or(0, |(_, m)| *m);
                assert_eq!(
                    posterior,
                    survivors * entry.weight(),
                    "posterior profile mass equals its surviving pair count \
                     (tile {tile:?}, profile {label})"
                );
            }
        }
    }
}

/// The raw fixed-field authority at one root: per legal root action the
/// exact best-response mass under the SINGLE field (the existing
/// [`response_success_mass`] machinery on a plain uniform-root belief),
/// plus the selected action under the declared lowest-tile-index rule.
fn raw_authority(
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &dyn SlicePolicy,
) -> (Vec<(Domino, u128)>, Domino) {
    let oracle = SupportOracle;
    let belief = FactorBelief::uniform_root(root, position, field);
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    let mut per_action: Vec<(Domino, u128)> = Vec::new();
    let mut best: Option<(Domino, u128)> = None;
    for tile in legal.iter() {
        let mut stats = ResponseStats::default();
        let q = response_success_mass(&oracle, &belief.focal_play(tile), field, &mut stats);
        per_action.push((tile, q));
        let better = match best {
            None => true,
            Some((_, incumbent)) => q > incumbent,
        };
        if better {
            best = Some((tile, q));
        }
    }
    let (chosen, _) = best.expect("a legal set holds an action");
    (per_action, chosen)
}

/// Gate 2 — point-mass parity both ways (MB-I5, MB-O2, the intake's §8
/// erratum made mechanical): ν = δ_{F₀} reproduces the σ0 fixed-field
/// authority's per-action values AND selected action on every tested
/// root; same for ν = δ_{F₁} against the Level1 authority.
#[test]
fn point_mass_parity_reproduces_both_fixed_field_authorities() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    for behavior in [&f0, &f1] {
        for (hand_id, trick_no, _) in TESTED_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let (per_action, chosen) =
                raw_authority(&root, &position, behavior.mind().as_ref());
            let model = delta_model(&root, &position, behavior);
            assert_eq!(model.profiles().len(), 1, "a δ prior is one profile");
            for (tile, q_raw) in &per_action {
                let mut stats = MixtureStats::default();
                let response = model
                    .focal_play(*tile)
                    .mixture_response(&oracle, &mut stats);
                assert_eq!(
                    response.outcome.weighted_mass, *q_raw,
                    "Q_a(δ) equals the fixed-field authority \
                     (hand {hand_id} trick {trick_no}, {}, action {tile:?})",
                    behavior.construction()
                );
            }
            let mut stats = MixtureStats::default();
            let response = model.mixture_response(&oracle, &mut stats);
            let q_root = per_action.iter().map(|(_, q)| *q).max().expect("actions");
            assert_eq!(
                response.outcome.weighted_mass, q_root,
                "the root response equals the authority's max"
            );
            assert_eq!(
                response.policy.choice_at(&[]),
                Some(chosen),
                "the δ selected action equals the fixed-field authority's \
                 (hand {hand_id} trick {trick_no}, {})",
                behavior.construction()
            );
        }
    }
}

/// Gate 3 — posterior closure and persistence. Part one (Theorem 12.1
/// mechanical): observing a hidden action changes ONLY the acting
/// seat's hand-type factor; the merged branch masses conserve exactly
/// (MB-I6 — asserted inside `branch_masses`, re-checked here). Part
/// two (§9, MB-I2, MB-O12): the ½-vs-¼ persistent-vs-resampled
/// separation on a SYNTHETIC-CARRIER specimen — two declared
/// deterministic carrier types (lowest-first / highest-first) at prior
/// (1/2, 1/2) on one hidden seat whose hand is pinned by a declared
/// table, on a real receipt root. Along the observed line the seat's
/// first action halves the augmented mass (probability exactly 1/2)
/// and resolves the type; its second action, at a state where the two
/// carriers still disagree, moves NO mass (conditional probability 1),
/// so the two-action sequence has probability 1/2 — a per-action
/// resampling semantics would put 1/4 on it. 2·joint = root-mass holds;
/// 4·joint = root-mass fails.
#[test]
fn posterior_closure_and_the_half_vs_quarter_persistence_specimen() {
    let r = receipt();
    let oracle = SupportOracle;
    let (f0, f1) = registered_types();
    // Part one on a real root under the registered types.
    let (root, position) = root_at(&r, 10, 6);
    let model = half_half_model(&root, &position, &f0, &f1);
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let mut exec = Pub::start(&position);
    let root_tile = choice_at(&position, &exec, root.kernel().viewer_hand(), &focal);
    let after = model.focal_play(root_tile);
    exec.play(&position, root_tile);
    let acting = after.seat_to_move();
    assert_ne!(acting, root.kernel().viewer());
    let branches = after.branch_masses(&oracle);
    let branch_sum: u128 = branches.iter().map(|(_, m)| *m).sum();
    assert_eq!(
        branch_sum,
        after.weighted_total(&oracle),
        "merged branch masses conserve the augmented mass (MB-I6)"
    );
    let before_factors: Vec<_> = root
        .kernel()
        .hidden()
        .iter()
        .map(|slot| after.hand_type_factor(slot.seat))
        .collect();
    let heaviest = branches
        .iter()
        .max_by_key(|(t, m)| (*m, usize::MAX - t.index()))
        .expect("a hidden seat has a branch")
        .0;
    let observed = after.observe(&oracle, heaviest);
    for (slot, before) in root.kernel().hidden().iter().zip(before_factors.iter()) {
        let after_factor = observed.hand_type_factor(slot.seat);
        if slot.seat == acting {
            continue;
        }
        // A non-acting seat's per-type factors are unchanged by the
        // observation (Theorem 12.1) — up to profile drops, which this
        // observation may cause only through the ACTING seat's zeros.
        for (id, factor) in &after_factor.slices {
            let matching = before
                .slices
                .iter()
                .find(|(t, _)| t == id)
                .expect("a surviving type existed before");
            assert_eq!(
                matching.1, *factor,
                "only the acting seat's factor changes (Theorem 12.1)"
            );
        }
    }
    // Part two: the synthetic-carrier specimen, searched over pinned
    // hands and the two trick-5 roots; the first satisfying line wins.
    let low: Arc<BehaviorType> = Arc::new(BehaviorType::declared(
        "carrier:lowest-first-v1",
        "none",
        TieRule::LowestTileIndex,
        PersistenceScope::PerHand,
        Arc::new(FixedPreference::lowest_first("mind:lowest-first")),
    ));
    let high: Arc<BehaviorType> = Arc::new(BehaviorType::declared(
        "carrier:highest-first-v1",
        "none",
        TieRule::FirstInPreference,
        PersistenceScope::PerHand,
        Arc::new(FixedPreference::highest_first("mind:highest-first")),
    ));
    let mut specimen_found = false;
    'roots: for (hand_id, trick_no) in [(8usize, 5usize), (3, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        let viewer_hand = root.kernel().viewer_hand();
        // The watched seat: the first hidden seat to act after the
        // viewer's root play.
        let mut probe_exec = Pub::start(&position);
        let probe_tile = choice_at(&position, &probe_exec, viewer_hand, &focal);
        probe_exec.play(&position, probe_tile);
        let watched = probe_exec.seat();
        assert_ne!(watched, viewer);
        let slot = root
            .kernel()
            .hidden()
            .iter()
            .position(|h| h.seat == watched)
            .expect("a hidden seat has a slot");
        let support = root.kernel().allowed(slot);
        let capacity = root.kernel().hidden()[slot].capacity;
        let priors: Vec<SeatTypePrior> = root
            .kernel()
            .hidden()
            .iter()
            .map(|s| SeatTypePrior {
                seat: s.seat,
                types: if s.seat == watched {
                    vec![(Arc::clone(&low), 1), (Arc::clone(&high), 1)]
                } else {
                    vec![(Arc::clone(&low), 1)]
                },
            })
            .collect();
        // Candidate pinned hands: the seat's lawful root hands.
        for hand in hands_of(support, capacity) {
            let base = ModelBelief::from_independent_prior(&root, &position, &priors)
                .with_seat_table(watched, vec![(hand, 1)]);
            if run_persistence_line(
                &base, &oracle, &root, &position, viewer, viewer_hand, &focal, watched, hand,
            ) {
                specimen_found = true;
                break 'roots;
            }
        }
    }
    assert!(
        specimen_found,
        "a ½-vs-¼ specimen exists on the trick-5 receipt roots"
    );
}

/// Every `k`-subset of `allowed`, lexicographic over ascending tile
/// index (the factor support order).
fn hands_of(allowed: DominoSet, k: usize) -> Vec<DominoSet> {
    let tiles: Vec<Domino> = allowed.iter().collect();
    let n = tiles.len();
    if k > n {
        return Vec::new();
    }
    let mut idx: Vec<usize> = (0..k).collect();
    let mut out = Vec::new();
    loop {
        out.push(idx.iter().map(|&i| tiles[i]).collect());
        let mut advanced = false;
        let mut i = k;
        while i > 0 {
            i -= 1;
            if idx[i] + k - i < n {
                idx[i] += 1;
                for j in (i + 1)..k {
                    idx[j] = idx[j - 1] + 1;
                }
                advanced = true;
                break;
            }
        }
        if !advanced {
            return out;
        }
    }
}

/// One candidate persistence line (see G3 part two). Returns true when
/// the pinned hand realizes the full specimen: the watched seat's first
/// action has ≥ 2 legal tiles with the carriers disagreeing (the ½
/// split, asserted), and its second action reaches a state with ≥ 2
/// legal tiles where they STILL disagree (the probability-1
/// continuation, asserted). Lines where either state is forced return
/// false and the search continues.
#[allow(clippy::too_many_arguments)]
fn run_persistence_line(
    base: &ModelBelief,
    oracle: &dyn ExactCoverOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    focal: &dyn SlicePolicy,
    watched: Seat,
    pinned: DominoSet,
) -> bool {
    let root_mass = base.weighted_total(oracle);
    let total = viewer_hand.len()
        + root
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>();
    let root_tile = choice_at(position, &Pub::start(position), viewer_hand, focal);
    let mut model = base.focal_play(root_tile);
    let mut exec = Pub::start(position);
    exec.play(position, root_tile);
    let mut watched_actions = 0usize;
    loop {
        let at_terminal = exec.history.len() == total;
        if decided_success(position, viewer, exec.banked, at_terminal).is_some() {
            return false;
        }
        let seat = exec.seat();
        if seat == viewer {
            let tile = choice_at(position, &exec, viewer_hand, focal);
            model = model.focal_play(tile);
            exec.play(position, tile);
            continue;
        }
        if seat == watched {
            let remaining = pinned.difference(exec.played_by[watched.index()]);
            let led = exec.plays.first().map(|d| position.decl.led_context(*d));
            let legal = legal_plays(position.decl, remaining, led);
            let a_low = legal.iter().next().expect("a legal tile");
            let a_high = legal.iter().last().expect("a legal tile");
            if watched_actions == 0 {
                if legal.len() < 2 {
                    return false;
                }
                assert_ne!(a_low, a_high, "two legal tiles give distinct extremes");
                // The ½ split: exactly two branches of equal mass.
                let branches = model.branch_masses(oracle);
                assert_eq!(branches.len(), 2, "a pinned hand branches by type only");
                assert_eq!(branches[0].1, branches[1].1, "equal prior weights");
                assert_eq!(
                    branches.iter().map(|(_, m)| *m).sum::<u128>(),
                    model.weighted_total(oracle)
                );
                let before = model.weighted_total(oracle);
                model = model.observe(oracle, a_low);
                let after = model.weighted_total(oracle);
                assert_eq!(2 * after, before, "the first observation is the ½");
                assert_eq!(
                    model.profiles().len(),
                    1,
                    "the observation resolves the watched seat's type"
                );
                assert_eq!(
                    model.seat_type_marginals(oracle)
                        .iter()
                        .find(|(s, _)| *s == watched)
                        .expect("the watched seat has a marginal")
                        .1
                        .len(),
                    1,
                    "one surviving type on the watched seat"
                );
                exec.play(position, a_low);
                watched_actions = 1;
                continue;
            }
            // The second watched action.
            if legal.len() < 2 {
                return false;
            }
            assert_ne!(a_low, a_high, "the carriers still disagree here");
            let branches = model.branch_masses(oracle);
            assert_eq!(
                branches,
                vec![(a_low, model.weighted_total(oracle))],
                "the persistent type puts ALL mass on its own action — \
                 conditional probability 1, not the resampled 1/2"
            );
            let before = model.weighted_total(oracle);
            model = model.observe(oracle, a_low);
            let joint = model.weighted_total(oracle);
            assert_eq!(joint, before, "the second observation moves no mass");
            // The §9 arithmetic: the two-action sequence carries mass
            // root/2 (persistent), not root/4 (resampled).
            assert_eq!(2 * joint, root_mass, "P(sequence) = 1/2 under persistence");
            assert_ne!(4 * joint, root_mass, "P(sequence) ≠ 1/4 — the resampled foil");
            return true;
        }
        // Another hidden seat: observe the heaviest merged branch.
        let branches = model.branch_masses(oracle);
        let tile = branches
            .iter()
            .max_by_key(|(t, m)| (*m, usize::MAX - t.index()))
            .expect("a hidden seat has a branch")
            .0;
        model = model.observe(oracle, tile);
        exec.play(position, tile);
    }
}

/// A focal policy that counts its consultations per information state
/// (public history) — the MB-I1 instrument.
struct CountingPolicy {
    inner: FixedPreference,
    calls: RefCell<BTreeMap<Vec<u8>, u64>>,
}

impl CountingPolicy {
    fn new() -> CountingPolicy {
        CountingPolicy {
            inner: FixedPreference::lowest_first("focal:counting-inner"),
            calls: RefCell::new(BTreeMap::new()),
        }
    }
}

impl SlicePolicy for CountingPolicy {
    fn id(&self) -> &str {
        "focal:counting"
    }

    fn choose(
        &self,
        decl: walt::rules::Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        let key: Vec<u8> = record
            .history
            .iter()
            .map(|d| u8::try_from(d.index()).expect("a tile index fits u8"))
            .collect();
        *self.calls.borrow_mut().entry(key).or_insert(0) += 1;
        self.inner.choose(decl, hand, legal, record)
    }
}

/// Gate 4 — merge-before-max (MB-I4, MB-O16) and hidden-type
/// unreadability (MB-I1). The API makes a type-keyed focal policy
/// unconstructible: [`SlicePolicy`] carries no type coordinate, and the
/// bundle walk consults the focal policy exactly ONCE per information
/// state for the whole bundle — so even a stateful policy cannot answer
/// differently per hidden type. The counting instrument witnesses the
/// one-consultation law with all eight profiles live. On the branching
/// side, types choosing the same public action stay in one branch: the
/// merged branch table is strictly smaller than the per-profile sum.
#[test]
fn merge_before_max_and_no_hidden_type_policy_key() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let (root, position) = root_at(&r, 10, 6);
    let model = half_half_model(&root, &position, &f0, &f1);
    let counting = CountingPolicy::new();
    let mut stats = MixtureStats::default();
    let outcome = model.mixture_policy_mass(&oracle, &counting, &mut stats);
    assert!(outcome.weighted_total > 0);
    let calls = counting.calls.borrow();
    assert!(!calls.is_empty(), "the walk reached a focal state");
    assert!(
        calls.values().all(|&c| c == 1),
        "one focal consultation per information state for the WHOLE bundle \
         (MB-I1): the policy is never evaluated per profile"
    );
    assert_eq!(
        calls.len() as u64,
        stats.focal_nodes,
        "every focal bundle node is one consultation"
    );
    // Merge-before-max census at the first hidden state: profiles
    // agreeing on a public action share its branch.
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let mut exec = Pub::start(&position);
    let tile = choice_at(&position, &exec, root.kernel().viewer_hand(), &focal);
    let after = model.focal_play(tile);
    exec.play(&position, tile);
    let (typed, merged) = after.typed_branch_census(&oracle);
    assert!(
        typed > merged,
        "types choosing the same public action stay in one branch \
         (typed rows {typed} > merged public branches {merged})"
    );
    assert_eq!(
        after.branch_masses(&oracle).len(),
        merged,
        "the branch table is the merged census"
    );
}

/// V_ν(ρ) as an exact rational from one machinery run.
fn value_of(
    model: &ModelBelief,
    oracle: &dyn ExactCoverOracle,
    focal: &dyn SlicePolicy,
) -> BigRational {
    let mut stats = MixtureStats::default();
    let outcome = model.mixture_policy_mass(oracle, focal, &mut stats);
    BigRational::new(
        BigInt::from(outcome.weighted_mass),
        BigInt::from(outcome.weighted_total),
    )
}

/// Gate 5 — exact linearity of the fixed-policy value in the model
/// belief (MB-O5, §16), on a swept rational grid of profile-level
/// mixtures between two priors (the profile simplex, not just the
/// independent product — §10's interface honesty), plus response-vector
/// consistency: the bundle's per-profile coordinates equal independent
/// δ-bundle evaluations.
#[test]
fn fixed_policy_value_is_linear_in_the_model_belief() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    for (hand_id, trick_no) in [(12usize, 6usize), (10, 6)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let base = half_half_model(&root, &position, &f0, &f1);
        // Two base priors over the SAME profile list: uniform (all 1)
        // and a deliberately asymmetric profile-level prior.
        let profile_types: Vec<Vec<Arc<BehaviorType>>> = base
            .profiles()
            .iter()
            .map(|e| e.types().to_vec())
            .collect();
        let w0: Vec<u128> = vec![1; profile_types.len()];
        let w1: Vec<u128> = (0..profile_types.len())
            .map(|i| 1 + (i as u128) * (i as u128) % 7)
            .collect();
        let d0: u128 = w0.iter().sum();
        let d1: u128 = w1.iter().sum();
        let model0 = ModelBelief::from_profile_prior(
            &root,
            &position,
            profile_types
                .iter()
                .cloned()
                .zip(w0.iter().copied())
                .collect(),
        );
        let model1 = ModelBelief::from_profile_prior(
            &root,
            &position,
            profile_types
                .iter()
                .cloned()
                .zip(w1.iter().copied())
                .collect(),
        );
        let v0 = value_of(&model0, &oracle, &focal);
        let v1 = value_of(&model1, &oracle, &focal);
        // The swept grid: λ = p/q over ν_mix = λ·ν0 + (1−λ)·ν1,
        // realized exactly by integer weights p·w0·d1 + (q−p)·w1·d0.
        for (p, q) in [(1u128, 2u128), (1, 3), (2, 5), (5, 7), (3, 4)] {
            let weights: Vec<u128> = w0
                .iter()
                .zip(w1.iter())
                .map(|(a, b)| p * a * d1 + (q - p) * b * d0)
                .collect();
            let mixed = ModelBelief::from_profile_prior(
                &root,
                &position,
                profile_types
                    .iter()
                    .cloned()
                    .zip(weights.iter().copied())
                    .collect(),
            );
            let v_mix = value_of(&mixed, &oracle, &focal);
            let lambda = BigRational::new(BigInt::from(p), BigInt::from(q));
            let one = BigRational::new(BigInt::from(1u8), BigInt::from(1u8));
            let expected = lambda.clone() * v0.clone() + (one - lambda) * v1.clone();
            assert_eq!(
                v_mix, expected,
                "V_ν(ρ) is exactly linear in ν (hand {hand_id} trick {trick_no}, λ = {p}/{q})"
            );
        }
        // Response-vector consistency: each per-profile coordinate of
        // the bundle equals an independent single-profile evaluation.
        let mut stats = MixtureStats::default();
        let outcome = model0.mixture_policy_mass(&oracle, &focal, &mut stats);
        for (i, entry) in model0.profiles().iter().enumerate() {
            let single = ModelBelief::from_profile_prior(
                &root,
                &position,
                vec![(entry.types().to_vec(), 1)],
            );
            let mut sstats = MixtureStats::default();
            let s = single.mixture_policy_mass(&oracle, &focal, &mut sstats);
            assert_eq!(
                s.per_profile_mass[0], outcome.per_profile_mass[i],
                "a response coordinate is prior-independent (profile {})",
                entry.label()
            );
        }
    }
}

/// Gate 6 — the type-revealed separated upper (Theorem 18.1, §19,
/// MB-O7, MB-O8): per root action on every tested root, Q_a(ν) ≤
/// U^sep_a under the (1/2, 1/2) prior; equality holds exactly when the
/// extracted mixture-argmax policy is pointwise optimal on the support
/// (Theorem 19.1 — the reverse direction checked mechanically at every
/// zero, the forward direction witnessed at every strict gap by the
/// profile the common policy sacrifices). At least one zero and at
/// least one strictly positive fusion price must appear across the
/// corpus (registered types first; the synthetic carrier mixture is
/// the declared fallback specimen and is always checked too).
#[test]
fn separated_upper_bounds_the_mixture_response_and_zero_iff_common_optimizer() {
    let r = receipt();
    let (f0, f1) = registered_types();
    let oracle = SupportOracle;
    let low: Arc<BehaviorType> = Arc::new(BehaviorType::declared(
        "carrier:lowest-first-v1",
        "none",
        TieRule::LowestTileIndex,
        PersistenceScope::PerHand,
        Arc::new(FixedPreference::lowest_first("mind:lowest-first")),
    ));
    let high: Arc<BehaviorType> = Arc::new(BehaviorType::declared(
        "carrier:highest-first-v1",
        "none",
        TieRule::FirstInPreference,
        PersistenceScope::PerHand,
        Arc::new(FixedPreference::highest_first("mind:highest-first")),
    ));
    let mut zeros = 0usize;
    let mut positives = 0usize;
    for (registered, fa, fb) in [(true, &f0, &f1), (false, &low, &high)] {
        for (hand_id, trick_no, _) in TESTED_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let model = half_half_model(&root, &position, fa, fb);
            let led = position
                .trick_plays
                .first()
                .map(|d| position.decl.led_context(*d));
            let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
            for tile in legal.iter() {
                let at_action = model.focal_play(tile);
                let mut stats = MixtureStats::default();
                let response = at_action.mixture_response(&oracle, &mut stats);
                let sep = at_action.separated_upper(&oracle);
                assert_eq!(
                    response.outcome.weighted_total, sep.weighted_total,
                    "one augmented mass under both evaluations"
                );
                assert!(
                    response.outcome.weighted_mass <= sep.weighted_mass,
                    "Q_a(ν) ≤ U^sep_a (Theorem 18.1) — hand {hand_id} trick \
                     {trick_no} action {tile:?} registered {registered}"
                );
                let zero = response.outcome.weighted_mass == sep.weighted_mass;
                // Theorem 19.1, both directions against the extracted
                // common policy: at zero, it attains every point-mass
                // optimum on support; at a strict gap, some profile is
                // sacrificed.
                let pointwise_optimal = response
                    .outcome
                    .per_profile_mass
                    .iter()
                    .zip(sep.per_profile_mass.iter())
                    .all(|(v, q)| v == q);
                assert_eq!(
                    zero, pointwise_optimal,
                    "Φ = 0 iff one common policy is pointwise optimal on \
                     support (hand {hand_id} trick {trick_no} action {tile:?})"
                );
                if zero {
                    zeros += 1;
                } else {
                    positives += 1;
                }
            }
        }
    }
    assert!(zeros > 0, "the corpus provides a zero fusion price");
    assert!(
        positives > 0,
        "the corpus provides a strictly positive fusion price"
    );
}

/// Scratch timing probe (never part of the gate set — ignored).
#[test]
#[ignore]
fn scratch_level1_timing() {
    let r = receipt();
    let (_, f1) = registered_types();
    let oracle = SupportOracle;
    for (hand_id, trick_no) in [(5usize, 6usize), (8, 5)] {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let focal = FixedPreference::lowest_first("focal:lowest-first");
        let model = delta_model(&root, &position, &f1);
        let start = std::time::Instant::now();
        let mut stats = MixtureStats::default();
        let outcome = model.mixture_policy_mass(&oracle, &focal, &mut stats);
        println!(
            "h{hand_id}-t{trick_no} delta-F1 fixed walk: {} us, mass {}/{}, stats {:?}",
            start.elapsed().as_micros(),
            outcome.weighted_mass,
            outcome.weighted_total,
            stats
        );
    }
}

/// Gate 7 — behavior-type identity (§51, MB-I3): equal construction
/// gives equal id; changing ANY behavior-affecting coordinate —
/// construction label, parent field (level, inner configuration, seed
/// schedule), tie rule, or the declared-vs-field construction path —
/// produces a new id. (Persistence has one declared scope in this
/// slice; its tag is in the id preimage, and a future scope arrives
/// with its machinery.)
#[test]
fn behavior_type_identity_tracks_every_coordinate() {
    let a = BehaviorType::from_field(level0_spec(), PersistenceScope::PerHand);
    let b = BehaviorType::from_field(level0_spec(), PersistenceScope::PerHand);
    assert_eq!(a.id(), b.id(), "equal construction, equal id");
    let mut n0_changed = level0_spec();
    n0_changed.kind = FieldKind::Level0 { n0: 3 };
    assert_ne!(
        a.id(),
        BehaviorType::from_field(n0_changed, PersistenceScope::PerHand).id(),
        "an inner-configuration change is a new identity"
    );
    let mut construction_changed = level0_spec();
    construction_changed.construction = "level0-modeled-mind-v2".to_string();
    assert_ne!(
        a.id(),
        BehaviorType::from_field(construction_changed, PersistenceScope::PerHand).id(),
        "a construction change is a new identity"
    );
    let mut seeded = level0_spec();
    seeded.seed_schedule = vec![7];
    assert_ne!(
        a.id(),
        BehaviorType::from_field(seeded, PersistenceScope::PerHand).id(),
        "a seed-schedule change is a new identity"
    );
    assert_ne!(
        a.id(),
        BehaviorType::from_field(level1_spec(), PersistenceScope::PerHand).id(),
        "the two registered rungs are distinct identities"
    );
    let mind: Arc<dyn SlicePolicy> =
        Arc::new(FixedPreference::lowest_first("mind:lowest-first"));
    let declared_a = BehaviorType::declared(
        "carrier:lowest-first-v1",
        "none",
        TieRule::LowestTileIndex,
        PersistenceScope::PerHand,
        Arc::clone(&mind),
    );
    let declared_b = BehaviorType::declared(
        "carrier:lowest-first-v1",
        "none",
        TieRule::LowestTileIndex,
        PersistenceScope::PerHand,
        Arc::clone(&mind),
    );
    assert_eq!(declared_a.id(), declared_b.id());
    assert_ne!(
        declared_a.id(),
        BehaviorType::declared(
            "carrier:lowest-first-v1",
            "none",
            TieRule::FirstInPreference,
            PersistenceScope::PerHand,
            Arc::clone(&mind),
        )
        .id(),
        "a tie-rule change is a new identity"
    );
    assert_ne!(
        declared_a.id(),
        BehaviorType::declared(
            "carrier:lowest-first-v1",
            "field:other",
            TieRule::LowestTileIndex,
            PersistenceScope::PerHand,
            Arc::clone(&mind),
        )
        .id(),
        "a parent-field change is a new identity"
    );
    assert_ne!(
        a.id(),
        BehaviorType::declared(
            a.construction(),
            "none",
            TieRule::LowestTileIndex,
            PersistenceScope::PerHand,
            Arc::clone(&mind),
        )
        .id(),
        "a field-registered rung and a declared carrier never alias"
    );
}
