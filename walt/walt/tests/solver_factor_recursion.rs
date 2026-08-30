//! Gates for the counted-belief Slice D [L2 thread]: the general support
//! contraction ([`SupportOracle`]) and the §23 factorized fixed-policy
//! recursion ([`viewer_success_mass`]) — extensional parity with backend
//! zero on the C0 domain (gate 1), surviving-world mass parity beyond it
//! with backend zero's refusal at the boundary (gate 2), the §47 value
//! parity with the bundled walk under the trivial field and under the σ0
//! level-0 field (gates 3–4), and the every-node checker: mass equals the
//! surviving-world count and branch masses equal the world partition at
//! EVERY node of the recursion tree, with the walk crossing the two-table
//! boundary (gate 5).
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! §23 (the factorized Bellman recursion, Theorem 23.1), §25.2/§25.4
//! (the support-contraction backend shape), §47 (Slice D), adopted by
//! rulings CBS-A6 and CBS-A9 (`walt/CENSUS-RULINGS.md`); design register
//! `walt/FACTOR-BELIEF.md`.
//!
//! DECLARED TEST EPOCH: deterministic fields only — the trivial
//! `FixedPreference` fields and the σ0 Level0 { n0 = 2 } modeled mind
//! (stage C1's declared cached field). Frozen `verify_player` receipt
//! roots: hands 4/5/10/12 at trick 6 (fibers 90/27/19/6), hands 3/8 at
//! trick 5 (fibers 200/92), and hand 0 at trick 1 (fiber 399,072,960 —
//! contracted in gate 1, never enumerated; the recursion at the opening
//! root is future work, not a Slice D claim).

mod common;

use std::panic::{catch_unwind, AssertUnwindSafe};

use common::receipt;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::bundle::bundled_set_outcomes;
use walt::solver::factor_belief::{
    viewer_success_mass, ExactCoverOracle, FactorBelief, FactorWeights, FiberOracle,
    RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};

/// The six enumerable frozen receipt roots: (hand, trick, fiber).
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

/// The σ0 field of the C1 epoch, fresh per call.
fn level0_field() -> FieldModel {
    FieldModel::new(FieldSpec {
        kind: FieldKind::Level0 { n0: 2 },
        construction: "level0-modeled-mind-v1".to_string(),
        practical_equivalence: None,
        fallback: "none".to_string(),
        seed_schedule: vec![],
        tie_rule: TieRule::LowestTileIndex,
        policy_library: "field-library-v1".to_string(),
        mode: DecisionMode::Heuristic,
    })
}

/// The test's own public-state replay: the same trick arithmetic as the
/// module's private walker and the bundled walk, built by hand here so
/// the checker is independent of the machinery it gates. `played` is the
/// union of all post-root plays — subtracting it from any one seat's root
/// hand removes exactly that seat's played tiles (hands are disjoint).
#[derive(Clone)]
struct Pub {
    leader: Seat,
    plays: Vec<Domino>,
    banked: [u32; 2],
    played: DominoSet,
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
            played: DominoSet::EMPTY,
            history: Vec::new(),
        }
    }

    fn seat(&self) -> Seat {
        self.leader.plus(self.plays.len())
    }

    fn play(&mut self, position: &RootPosition, tile: Domino) {
        assert!(self.played.insert(tile), "a tile is played once");
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
    let remaining = root_hand.difference(exec.played);
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    let record = exec.record(position);
    let tile = policy.choose(position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal tile");
    tile
}

fn tables_of(belief: &FactorBelief) -> usize {
    belief
        .factors()
        .iter()
        .filter(|f| matches!(f.weights(), FactorWeights::Table(_)))
        .count()
}

/// Gate 1 — on the C0 domain (all-uniform, and one conditioned table) the
/// Slice D backend is extensionally equal to backend zero: mass,
/// completion weights, branch masses, and the conditioned posterior all
/// agree on every enumerable root, and mass plus branch masses agree at
/// the opening root through the contraction route.
#[test]
fn support_backend_matches_backend_zero_across_the_c0_domain() {
    let r = receipt();
    let field = FixedPreference::lowest_first("field:lowest-first");
    let zero = FiberOracle;
    let general = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let uniform = FactorBelief::uniform_root(&root, &position, &field);
        let focal = choice_at(
            &position,
            &Pub::start(&position),
            root.kernel().viewer_hand(),
            &field,
        );
        let belief = uniform.focal_play(focal);
        assert_eq!(general.mass(&belief), fiber);
        assert_eq!(zero.mass(&belief), general.mass(&belief));
        let seat = belief.seat_to_move();
        assert_eq!(
            zero.actor_completion_weights(&belief, seat),
            general.actor_completion_weights(&belief, seat)
        );
        let branches = general.branch_masses(&belief, &field);
        assert_eq!(zero.branch_masses(&belief, &field), branches);
        // One conditioning — still the C0 domain; the posteriors and
        // their masses agree between the backends.
        let heaviest = branches
            .iter()
            .max_by_key(|(_, m)| *m)
            .expect("a hidden seat has a branch")
            .0;
        let cond_zero = zero.condition(&belief, heaviest, &field);
        let cond_general = general.condition(&belief, heaviest, &field);
        assert_eq!(cond_zero, cond_general);
        assert_eq!(zero.mass(&cond_zero), general.mass(&cond_general));
    }
    // The opening root, contracted and never enumerated: the all-uniform
    // fast path is the shipped DP on both backends, and the one-ply
    // branch tables agree entry by entry.
    let (root, position) = root_at(&r, 0, 1);
    let uniform = FactorBelief::uniform_root(&root, &position, &field);
    let focal = choice_at(
        &position,
        &Pub::start(&position),
        root.kernel().viewer_hand(),
        &field,
    );
    let belief = uniform.focal_play(focal);
    assert_eq!(general.mass(&belief), 399_072_960);
    assert_eq!(
        zero.branch_masses(&belief, &field),
        general.branch_masses(&belief, &field)
    );
}

/// Gate 2 — beyond the C0 domain: walk the heaviest-branch line under the
/// trivial field, conditioning every acting hidden seat. At every hidden
/// step the support backend's mass equals the surviving-world count; the
/// walk crosses the two-table boundary, where backend zero REFUSES the
/// contraction (the declared C0 domain) and the support backend keeps
/// counting exactly.
#[test]
fn beyond_one_table_mass_matches_surviving_worlds_and_backend_zero_refuses() {
    let r = receipt();
    let field = FixedPreference::lowest_first("field:lowest-first");
    let general = SupportOracle;
    for (hand_id, trick_no) in [(4, 6), (8, 5), (3, 5)] {
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
        let mut belief = FactorBelief::uniform_root(&root, &position, &field);
        let mut worlds: Vec<World> = root.worlds().collect();
        let mut exec = Pub::start(&position);
        let mut crossed: Option<FactorBelief> = None;
        loop {
            let at_terminal = exec.history.len() == total;
            if decided_success(&position, viewer, exec.banked, at_terminal).is_some() {
                break;
            }
            let seat = exec.seat();
            if seat == viewer {
                let tile = choice_at(&position, &exec, viewer_hand, &field);
                belief = belief.focal_play(tile);
                exec.play(&position, tile);
            } else {
                let branches = general.branch_masses(&belief, &field);
                let tile = branches
                    .iter()
                    .max_by_key(|(_, m)| *m)
                    .expect("a hidden seat has a branch")
                    .0;
                worlds.retain(|w| choice_at(&position, &exec, w.hand(seat), &field) == tile);
                belief = general.condition(&belief, tile, &field);
                exec.play(&position, tile);
                assert_eq!(
                    general.mass(&belief),
                    u128::try_from(worlds.len()).expect("fits"),
                    "a conditioned mass is its surviving-world count"
                );
            }
            if tables_of(&belief) >= 2 && crossed.is_none() {
                crossed = Some(belief.clone());
            }
        }
        let crossed = crossed.expect("the heaviest-branch line conditions two hidden seats");
        let refused = catch_unwind(AssertUnwindSafe(|| FiberOracle.mass(&crossed)));
        assert!(
            refused.is_err(),
            "backend zero refuses a two-table contraction (the declared C0 domain)"
        );
    }
}

/// Gate 3 — the §47 value gate, trivial field: the recursion's success
/// mass equals the bundled walk's wins on every enumerable root, for two
/// frozen focal policies, with Z the fiber count.
#[test]
fn frozen_policy_values_match_the_bundled_walk_under_the_trivial_field() {
    let r = receipt();
    let field = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let general = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        for focal in [&low, &high] {
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            assert_eq!(general.mass(&belief), fiber);
            let mut stats = RecursionStats::default();
            let mass = viewer_success_mass(&general, &belief, focal, &field, &mut stats);
            let bundled = bundled_set_outcomes(&root, &position, &[focal], &field);
            assert_eq!(
                mass,
                bundled.wins(0),
                "the factorized recursion equals the bundled walk (hand {hand_id} trick {trick_no})"
            );
            assert!(mass <= fiber, "a success mass is bounded by the fiber");
            assert!(
                stats.decided_early + stats.decided_terminal >= 1,
                "a finite recursion reaches a decided state"
            );
            assert!(
                stats.conditionings >= stats.hidden_nodes,
                "every hidden node takes at least one branch"
            );
        }
    }
}

/// Gate 4 — the §47 "then move to the current level-0 field" step: the
/// same value parity under the σ0 modeled mind, fresh field instances per
/// route (per-state determinism makes them agree — the C1 law).
#[test]
fn frozen_policy_values_match_the_bundled_walk_under_the_level0_field() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let general = SupportOracle;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let field_r = level0_field();
        let belief = FactorBelief::uniform_root(&root, &position, &field_r);
        let mut stats = RecursionStats::default();
        let mass = viewer_success_mass(&general, &belief, &focal, &field_r, &mut stats);
        let field_b = level0_field();
        let bundled = bundled_set_outcomes(&root, &position, &[&focal], &field_b);
        assert_eq!(
            mass,
            bundled.wins(0),
            "the factorized recursion equals the bundled walk under σ0 \
             (hand {hand_id} trick {trick_no})"
        );
        assert!(mass <= fiber, "a success mass is bounded by the fiber");
    }
}

/// The every-node checker of gate 5: walk the recursion tree carrying the
/// surviving complete-world list. At EVERY node the backend's mass equals
/// the surviving-world count; at every hidden node the branch masses
/// equal the world partition by the field's chosen tile; decided leaves
/// contribute their whole surviving set. Returns the enumerated success
/// mass and tracks the deepest table count reached.
#[allow(clippy::too_many_arguments)]
fn check_node(
    oracle: &SupportOracle,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total: usize,
    belief: &FactorBelief,
    worlds: Vec<World>,
    exec: &Pub,
    focal: &dyn SlicePolicy,
    field: &dyn SlicePolicy,
    max_tables: &mut usize,
) -> u128 {
    assert_eq!(
        oracle.mass(belief),
        u128::try_from(worlds.len()).expect("fits"),
        "every node's mass is its surviving-world count"
    );
    *max_tables = (*max_tables).max(tables_of(belief));
    let at_terminal = exec.history.len() == total;
    if let Some(u) = decided_success(position, viewer, exec.banked, at_terminal) {
        return if u {
            u128::try_from(worlds.len()).expect("fits")
        } else {
            0
        };
    }
    let seat = exec.seat();
    if seat == viewer {
        let tile = choice_at(position, exec, viewer_hand, focal);
        let child = belief.focal_play(tile);
        let mut child_exec = exec.clone();
        child_exec.play(position, tile);
        check_node(
            oracle,
            position,
            viewer,
            viewer_hand,
            total,
            &child,
            worlds,
            &child_exec,
            focal,
            field,
            max_tables,
        )
    } else {
        let branches = oracle.branch_masses(belief, field);
        let mut groups: Vec<(Domino, Vec<World>)> = Vec::new();
        for world in worlds {
            let tile = choice_at(position, exec, world.hand(seat), field);
            match groups.iter_mut().find(|(t, _)| *t == tile) {
                Some((_, group)) => group.push(world),
                None => groups.push((tile, vec![world])),
            }
        }
        groups.sort_by_key(|(t, _)| t.index());
        assert_eq!(
            branches,
            groups
                .iter()
                .map(|(t, g)| (*t, u128::try_from(g.len()).expect("fits")))
                .collect::<Vec<_>>(),
            "branch masses equal the world partition at every hidden node"
        );
        let mut mass: u128 = 0;
        for (tile, group) in groups {
            let child = oracle.condition(belief, tile, field);
            let mut child_exec = exec.clone();
            child_exec.play(position, tile);
            mass += check_node(
                oracle,
                position,
                viewer,
                viewer_hand,
                total,
                &child,
                group,
                &child_exec,
                focal,
                field,
                max_tables,
            );
        }
        mass
    }
}

/// Gate 5 — mass and branch parity at EVERY node of the recursion tree,
/// against complete-world enumeration, under both the trivial field and
/// σ0; the checker's total equals the recursion's, and the walk crosses
/// the two-table boundary on every multi-trick root.
#[test]
fn every_node_mass_and_branch_parity_with_world_enumeration() {
    let r = receipt();
    let focal = FixedPreference::lowest_first("focal:lowest-first");
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let general = SupportOracle;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
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
        // σ0 uses ONE instance across checker and recursion: field purity
        // (O29) makes the shared cache an optimization, never an input.
        let sigma0 = level0_field();
        let fields: [&dyn SlicePolicy; 2] = [&trivial, &sigma0];
        for field in fields {
            let belief = FactorBelief::uniform_root(&root, &position, field);
            let worlds: Vec<World> = root.worlds().collect();
            let mut max_tables = 0usize;
            let checked = check_node(
                &general,
                &position,
                viewer,
                viewer_hand,
                total,
                &belief,
                worlds,
                &Pub::start(&position),
                &focal,
                field,
                &mut max_tables,
            );
            let mut stats = RecursionStats::default();
            let recursed = viewer_success_mass(&general, &belief, &focal, field, &mut stats);
            assert_eq!(
                checked,
                recursed,
                "the enumerated and factorized success masses agree \
                 (hand {hand_id} trick {trick_no}, field {})",
                field.id()
            );
            if trick_no <= 5 {
                assert!(
                    max_tables >= 2,
                    "a multi-trick walk conditions at least two hidden seats"
                );
            }
        }
    }
}
