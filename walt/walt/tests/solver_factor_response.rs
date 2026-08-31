//! Gates for the counted-belief Slice E [L2 thread]: the §48 factorized
//! grammar best response ([`grammar_success_mass`]) — the §23 recursion
//! with the focal case's frozen action replaced by a max over the
//! grammar's actions. Gate 1 is the headline parity: per grammar root
//! action, the factorized `Q^G_a` equals the Slice B enumeration split's
//! grammar optimum, and the root call is their max. Gate 2 collapses a
//! singleton grammar to the Slice D fixed-policy recursion. Gate 3 is
//! dominance and non-vacuity: every source policy's value is bounded by
//! the grammar optimum, the grammar optimum by the free optimum, and the
//! constraint BINDS somewhere (the parity gates distinguish `gram` from
//! `free`). Gate 4 is the every-node checker: mass equals the
//! surviving-world count at every node, branch masses equal the world
//! partition at every hidden node, and the enumerated grammar-max value
//! equals the factorized one, with the walk crossing the two-table
//! boundary.
//!
//! Mathematical source: `walt/math/counted_belief_sandwich_v0.1.md`
//! §11–12 (grammars and the decomposition), §23 (the factorized Bellman
//! recursion), §48 (Slice E), adopted by rulings CBS-A4, CBS-A6 and
//! CBS-A9 (`walt/CENSUS-RULINGS.md`); design register
//! `walt/FACTOR-BELIEF.md`.
//!
//! DECLARED TEST EPOCH: deterministic fields only — the trivial
//! `FixedPreference` field and the σ0 Level0 { n0 = 2 } modeled mind
//! (the Slice B/C/D declared fields); grammar sources
//! `FixedPreference::lowest_first` / `highest_first` and the
//! `CountPreservation` safety policy. Frozen `verify_player` receipt
//! roots: hands 4/5/10/12 at trick 6 (fibers 90/27/19/6), hands 3/8 at
//! trick 5 (fibers 200/92). The §48 fence holds throughout: nothing here
//! maximizes over the full action set.

mod common;

use common::receipt;
use walt::kernel::{Kernel, World};
use walt::rules::receipt::Receipt;
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet, Seat, Trick};
use walt::solver::adaptive::{
    decided_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{
    grammar_success_mass, viewer_success_mass, ExactCoverOracle, FactorBelief, FactorWeights,
    RecursionStats, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{exact_grammar_split, CountPreservation, PolicyGrammar};
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

/// The σ0 field of the declared epoch, fresh per call.
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

/// The test's own public-state replay — the Slice D checker's `Pub`,
/// rebuilt by hand so this gate file is independent of the machinery it
/// gates.
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

/// The focal frame at the current public state: remaining hand, legal set.
fn frame_at(position: &RootPosition, exec: &Pub, root_hand: DominoSet) -> (DominoSet, DominoSet) {
    let remaining = root_hand.difference(exec.played);
    let led = exec.plays.first().map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, remaining, led);
    assert!(!legal.is_empty(), "a seat to move holds a legal tile");
    (remaining, legal)
}

/// One policy's choice at the current public state, from a root hand.
fn choice_at(
    position: &RootPosition,
    exec: &Pub,
    root_hand: DominoSet,
    policy: &dyn SlicePolicy,
) -> Domino {
    let (remaining, legal) = frame_at(position, exec, root_hand);
    let record = exec.record(position);
    let tile = policy.choose(position.decl, remaining, legal, &record);
    assert!(legal.contains(tile), "a policy chooses a legal tile");
    tile
}

/// The grammar's action set at the current public state, for the viewer.
fn grammar_at(
    position: &RootPosition,
    exec: &Pub,
    viewer_hand: DominoSet,
    grammar: &PolicyGrammar<'_>,
) -> DominoSet {
    let (remaining, legal) = frame_at(position, exec, viewer_hand);
    let record = exec.record(position);
    grammar.actions(position.decl, remaining, legal, &record)
}

fn tables_of(belief: &FactorBelief) -> usize {
    belief
        .factors()
        .iter()
        .filter(|f| matches!(f.weights(), FactorWeights::Table(_)))
        .count()
}

fn total_plies(root: &CanonicalRoot) -> usize {
    root.kernel().viewer_hand().len()
        + root
            .kernel()
            .hidden()
            .iter()
            .map(|h| h.capacity)
            .sum::<usize>()
}

/// Gate 1 — the headline §48 parity, under the σ0 field: for every
/// grammar root action `a`, the factorized `Q^G_a`
/// (`grammar_success_mass` after `focal_play(a)`) equals the Slice B
/// enumeration split's grammar optimum; the root recursion equals the max
/// over the grammar root actions; and every `Q^G_a` is bounded by the
/// split's free optimum. Two grammars, so saturation at small legal sets
/// cannot make the gate vacuous by accident.
#[test]
fn grammar_root_values_match_the_slice_b_split_under_the_level0_field() {
    let r = receipt();
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let general = SupportOracle;
    for grammar in [&two, &three] {
        for (hand_id, trick_no, fiber) in ENUM_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            // ONE σ0 instance serves the split and the recursion: field
            // purity (O29) makes the shared cache an optimization, never
            // an input.
            let field = level0_field();
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            assert_eq!(general.mass(&belief), fiber);
            let root_actions = grammar_at(
                &position,
                &Pub::start(&position),
                root.kernel().viewer_hand(),
                grammar,
            );
            let mut best: Option<u128> = None;
            for action in root_actions.iter() {
                let split = exact_grammar_split(&root, &position, action, &field, grammar);
                let gram = split
                    .grammar_count()
                    .expect("a grammar root action has a grammar side");
                let mut stats = ResponseStats::default();
                let mass = grammar_success_mass(
                    &general,
                    &belief.focal_play(action),
                    grammar,
                    &field,
                    &mut stats,
                );
                assert_eq!(
                    mass,
                    u128::from(gram),
                    "the factorized Q^G_a equals the Slice B grammar optimum \
                     (hand {hand_id} trick {trick_no}, action {action})"
                );
                assert!(
                    mass <= u128::from(split.free_count()),
                    "the grammar optimum is bounded by the free optimum"
                );
                best = Some(best.map_or(mass, |b: u128| b.max(mass)));
            }
            let mut stats = ResponseStats::default();
            let at_root = grammar_success_mass(&general, &belief, grammar, &field, &mut stats);
            let cursor = Pub::start(&position);
            if decided_success(&position, root.kernel().viewer(), cursor.banked, false).is_some() {
                // Decided at the root: the cutoff answers before the max,
                // and every per-action value agrees with it.
                assert_eq!(Some(at_root), best, "a decided root's actions all agree");
            } else {
                assert_eq!(
                    Some(at_root),
                    best,
                    "the root recursion is the max over the grammar root actions \
                     (hand {hand_id} trick {trick_no})"
                );
                assert!(
                    stats.focal_actions >= stats.focal_nodes,
                    "a grammar holds an action at every focal state"
                );
            }
        }
    }
}

/// Gate 2 — a singleton grammar collapses the max: `grammar_success_mass`
/// under `G = {ρ}` equals the Slice D fixed-policy recursion under `ρ`,
/// on every enumerable root, under both declared fields.
#[test]
fn a_singleton_grammar_reduces_to_the_fixed_policy_recursion() {
    let r = receipt();
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let general = SupportOracle;
    for source in [&low, &high] {
        let singleton = PolicyGrammar::new(vec![source]);
        for (hand_id, trick_no, _) in ENUM_ROOTS {
            let (root, position) = root_at(&r, hand_id, trick_no);
            let sigma0 = level0_field();
            let fields: [&dyn SlicePolicy; 2] = [&trivial, &sigma0];
            for field in fields {
                let belief = FactorBelief::uniform_root(&root, &position, field);
                let mut r_stats = RecursionStats::default();
                let fixed = viewer_success_mass(&general, &belief, source, field, &mut r_stats);
                let mut g_stats = ResponseStats::default();
                let grammar =
                    grammar_success_mass(&general, &belief, &singleton, field, &mut g_stats);
                assert_eq!(
                    fixed,
                    grammar,
                    "a singleton grammar is the fixed-policy recursion \
                     (hand {hand_id} trick {trick_no}, field {})",
                    field.id()
                );
                assert_eq!(
                    g_stats.focal_actions, g_stats.focal_nodes,
                    "a singleton grammar explores exactly one action per focal node"
                );
            }
        }
    }
}

/// Gate 3 — dominance and non-vacuity: every grammar source's fixed-policy
/// value is bounded by the grammar optimum on every root under both
/// fields (the §12 lower-witness direction), and under σ0 the grammar
/// constraint BINDS on at least one root — `Q^G` falls strictly below the
/// free optimum, so the gate-1 parity is measuring the constrained
/// quantity, not the free one.
#[test]
fn grammar_sources_are_dominated_and_the_constraint_binds_somewhere() {
    let r = receipt();
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let grammar = PolicyGrammar::new(vec![&low, &high]);
    let general = SupportOracle;
    let mut binds = false;
    for (hand_id, trick_no, fiber) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let sigma0 = level0_field();
        let fields: [&dyn SlicePolicy; 2] = [&trivial, &sigma0];
        for field in fields {
            let belief = FactorBelief::uniform_root(&root, &position, field);
            let mut g_stats = ResponseStats::default();
            let q_g = grammar_success_mass(&general, &belief, &grammar, field, &mut g_stats);
            assert!(q_g <= fiber, "a success mass is bounded by the fiber");
            for source in [&low, &high] {
                let mut r_stats = RecursionStats::default();
                let v = viewer_success_mass(&general, &belief, source, field, &mut r_stats);
                assert!(
                    v <= q_g,
                    "a grammar source is dominated by the grammar optimum \
                     (hand {hand_id} trick {trick_no}, field {})",
                    field.id()
                );
            }
        }
        // The binding check runs under σ0, where the Slice B split serves
        // as the free-optimum authority. An observed fact of these tiny
        // endgame fibers: the TWO-source grammar ties the free optimum on
        // every enumerable root (the constraint never binds — small legal
        // sets, and where G(I) ⊊ A(I) the deviations don't pay), so the
        // binding witness comes from the singleton grammars, which bind
        // exactly where the two sources' values differ.
        let singles = [
            PolicyGrammar::new(vec![&low]),
            PolicyGrammar::new(vec![&high]),
        ];
        for g in [&grammar].into_iter().chain(singles.iter()) {
            let field = level0_field();
            let belief = FactorBelief::uniform_root(&root, &position, &field);
            let root_actions = grammar_at(
                &position,
                &Pub::start(&position),
                root.kernel().viewer_hand(),
                g,
            );
            for action in root_actions.iter() {
                let split = exact_grammar_split(&root, &position, action, &field, g);
                let mut stats = ResponseStats::default();
                let mass = grammar_success_mass(
                    &general,
                    &belief.focal_play(action),
                    g,
                    &field,
                    &mut stats,
                );
                assert_eq!(
                    mass,
                    u128::from(
                        split
                            .grammar_count()
                            .expect("a grammar root action has a grammar side")
                    ),
                    "the factorized Q^G_a equals the Slice B grammar optimum"
                );
                if mass < u128::from(split.free_count()) {
                    binds = true;
                }
            }
        }
    }
    assert!(
        binds,
        "the grammar constraint binds on at least one enumerable root: \
         somewhere Q^G_a < Q_a, so gram and free are distinguished"
    );
}

/// The every-node checker of gate 4: walk the grammar recursion tree
/// carrying the surviving complete-world list. At EVERY node the
/// backend's mass equals the surviving-world count; at every hidden node
/// the branch masses equal the world partition by the field's chosen
/// tile; at every focal node the value is the max over the grammar's
/// actions of the children (the enumerated §48 structure). Returns the
/// enumerated grammar-max mass and tracks the deepest table count.
#[allow(clippy::too_many_arguments)]
fn check_gram_node(
    oracle: &SupportOracle,
    position: &RootPosition,
    viewer: Seat,
    viewer_hand: DominoSet,
    total: usize,
    belief: &FactorBelief,
    worlds: Vec<World>,
    exec: &Pub,
    grammar: &PolicyGrammar<'_>,
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
        let actions = grammar_at(position, exec, viewer_hand, grammar);
        let mut best: Option<u128> = None;
        for tile in actions.iter() {
            let child = belief.focal_play(tile);
            let mut child_exec = exec.clone();
            child_exec.play(position, tile);
            let m = check_gram_node(
                oracle,
                position,
                viewer,
                viewer_hand,
                total,
                &child,
                worlds.clone(),
                &child_exec,
                grammar,
                field,
                max_tables,
            );
            best = Some(best.map_or(m, |b| b.max(m)));
        }
        best.expect("a grammar holds an action at every focal state")
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
            mass += check_gram_node(
                oracle,
                position,
                viewer,
                viewer_hand,
                total,
                &child,
                group,
                &child_exec,
                grammar,
                field,
                max_tables,
            );
        }
        mass
    }
}

/// Gate 4 — mass, branch, and grammar-max parity at EVERY node of the
/// recursion tree, against complete-world enumeration, under both the
/// trivial field and σ0; the checker's total equals the recursion's, and
/// the walk crosses the two-table boundary on every multi-trick root.
#[test]
fn every_node_mass_and_grammar_max_parity_with_world_enumeration() {
    let r = receipt();
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let grammar = PolicyGrammar::new(vec![&low, &high]);
    let general = SupportOracle;
    for (hand_id, trick_no, _) in ENUM_ROOTS {
        let (root, position) = root_at(&r, hand_id, trick_no);
        let viewer = root.kernel().viewer();
        let viewer_hand = root.kernel().viewer_hand();
        let total = total_plies(&root);
        let sigma0 = level0_field();
        let fields: [&dyn SlicePolicy; 2] = [&trivial, &sigma0];
        for field in fields {
            let belief = FactorBelief::uniform_root(&root, &position, field);
            let worlds: Vec<World> = root.worlds().collect();
            let mut max_tables = 0usize;
            let checked = check_gram_node(
                &general,
                &position,
                viewer,
                viewer_hand,
                total,
                &belief,
                worlds,
                &Pub::start(&position),
                &grammar,
                field,
                &mut max_tables,
            );
            let mut stats = ResponseStats::default();
            let recursed = grammar_success_mass(&general, &belief, &grammar, field, &mut stats);
            assert_eq!(
                checked,
                recursed,
                "the enumerated and factorized grammar optima agree \
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
