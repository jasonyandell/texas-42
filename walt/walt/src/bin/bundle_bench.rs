//! EXPLORATORY BUNDLE BENCH (`walt/probes/bundle/`) — sits below every
//! evidentiary tier and is cited by nothing above it. Wall-clock numbers
//! are machine-local instrument readings, never receipts.
//!
//! Times the exact route's two evaluators on the same roots, candidates,
//! and field configuration:
//!
//!   - PER-WORLD: the `replay_viewer_success` loop exactly as the
//!     controller's §11.5 escalation endpoint runs it — every fiber
//!     world replayed separately to terminal, per candidate.
//!   - BUNDLED: `solver::bundle::bundled_set_outcomes` — one shared-tree
//!     walk per candidate carrying the whole fiber, partitioned only
//!     where a field observation distinguishes members, with the decided
//!     cutoff settling whole bundles early.
//!
//! Both routes produce `outcomes[k][w]`; the bench asserts their wins
//! totals agree before printing anything (E-A15: same set, same values —
//! only the order of evaluation differs). Declared workload: the
//! `solver_controller` fiber-90 (hand 4 trick 6, m=3) and fiber-1120
//! (hand 11 trick 5, m=4) roots, plus the largest affordable receipt
//! root, hand 11 trick 4 — fiber 23100, m=3 (chosen from the exp5
//! corpus; the next sizes up, 34650/59976, add nothing structural).
//! Field: a cached level-0 `FieldModel` at n0=2 (the cheap declared
//! configuration the ordering bench also uses), a FRESH cold instance
//! per route so neither route reads the other's warmth. Candidates:
//! preference-order `FrozenPolicy`s (the controller fixtures), likewise
//! fresh per route.
//!
//! Prints integer wall-micros and exact integer counters. The per-world
//! route's node figure is `m · fiber · total_plays` exactly (every
//! replay runs every post-root play to terminal); the bundled route's
//! sharing and cutoff statistics come from the primitive itself. No
//! floats anywhere.

use std::cell::Cell;
use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::{Decl, Domino, DominoSet};
use walt::solver::adaptive::{
    replay_viewer_success, CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::bundle::bundled_set_outcomes;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};

/// A bench-owned counting shim: delegates every choice to the wrapped
/// field unchanged and counts the queries — an exact integer statistic of
/// how much field work each route performs. Purely observational (O29:
/// the wrapped choice function is untouched).
struct CountingField<'a> {
    inner: &'a dyn SlicePolicy,
    queries: Cell<u64>,
}

impl<'a> CountingField<'a> {
    fn new(inner: &'a dyn SlicePolicy) -> CountingField<'a> {
        CountingField {
            inner,
            queries: Cell::new(0),
        }
    }
}

impl SlicePolicy for CountingField<'_> {
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn choose(
        &self,
        decl: Decl,
        hand: DominoSet,
        legal: DominoSet,
        record: &PublicRecord<'_>,
    ) -> Domino {
        self.queries.set(self.queries.get() + 1);
        self.inner.choose(decl, hand, legal, record)
    }
}

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

fn root_at(r: &Receipt, hand_no: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_no];
    assert_eq!(hand.id, hand_no);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn ascending() -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index(i).expect("index < 28"))
        .collect()
}

fn descending() -> Vec<Domino> {
    ascending().into_iter().rev().collect()
}

fn stride(mult: usize, offset: usize) -> Vec<Domino> {
    (0..DominoSet::FULL.len())
        .map(|i| Domino::from_index((offset + mult * i) % 28).expect("index < 28"))
        .collect()
}

fn freeze(position: &RootPosition, order: Vec<Domino>) -> FreezeTuple {
    FreezeTuple {
        solver_source: "walt-solver-step5-v1".to_string(),
        decl: position.decl,
        bid: position.bid,
        declaring_team: position.declaring_team,
        field_model: "fixed-preference".to_string(),
        field_level: 0,
        inner_schedule: InnerSchedule::None,
        discovery_stream: "policy-discovery-splitmix64-counter-v1".to_string(),
        discovery_seed_schedule: vec![],
        tie_rule: TieRule::FirstInPreference,
        practical_equivalence: None,
        policy_library: "preference-library-v1".to_string(),
        mode: DecisionMode::Exact,
        action_rule: ActionRule::Preference(order),
    }
}

fn pool(position: &RootPosition, orders: &[Vec<Domino>]) -> Vec<FrozenPolicy> {
    orders
        .iter()
        .map(|order| FrozenPolicy::new(freeze(position, order.clone())))
        .collect()
}

/// A fresh cold level-0 field model at the declared cheap configuration.
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

/// The two declared field configurations: the cached level-0 model (the
/// realistic exact-route field) and the trivial lowest-first preference
/// (isolates the walk-sharing effect from per-query field cost).
enum FieldConfig {
    CachedLevel0,
    LowestFirst,
}

impl FieldConfig {
    fn label(&self) -> &'static str {
        match self {
            FieldConfig::CachedLevel0 => "cached-level0-n2",
            FieldConfig::LowestFirst => "lowest-first",
        }
    }

    fn materialize(&self) -> Box<dyn SlicePolicy> {
        match self {
            FieldConfig::CachedLevel0 => Box::new(level0_field()),
            FieldConfig::LowestFirst => {
                Box::new(FixedPreference::lowest_first("field:lowest-first"))
            }
        }
    }
}

fn bench_root(r: &Receipt, hand_no: usize, trick_no: usize, orders: &[Vec<Domino>]) {
    let (root, position) = root_at(r, hand_no, trick_no);
    let kernel = root.kernel();
    let viewer = kernel.viewer();
    let total =
        kernel.viewer_hand().len() + kernel.hidden().iter().map(|h| h.capacity).sum::<usize>();
    let fiber = root.count();
    let m = orders.len();
    println!("root h{hand_no}-t{trick_no} fiber={fiber} m={m} total_plays={total}");

    for config in [FieldConfig::CachedLevel0, FieldConfig::LowestFirst] {
        // PER-WORLD route (fresh candidates + fresh cold field): the
        // escalation endpoint's loop, verbatim in shape.
        let per_world_pool = pool(&position, orders);
        let per_world_inner = config.materialize();
        let per_world_field = CountingField::new(per_world_inner.as_ref());
        let start = Instant::now();
        let mut wins = vec![0u128; m];
        let mut visited = 0u128;
        for world in root.worlds() {
            for (k, win) in wins.iter_mut().enumerate() {
                if replay_viewer_success(
                    &position,
                    viewer,
                    &world,
                    &per_world_pool[k],
                    &per_world_field,
                ) {
                    *win += 1;
                }
            }
            visited += 1;
        }
        let per_world_us = start.elapsed().as_micros();
        assert_eq!(
            visited, fiber,
            "enumeration visits the whole fiber exactly once"
        );
        let per_world_plays =
            u128::try_from(m).expect("fits") * fiber * u128::try_from(total).expect("fits");
        println!(
            "  [{}] per-world: wall_us={} plays={} (m*fiber*total) field_queries={}",
            config.label(),
            per_world_us,
            per_world_plays,
            per_world_field.queries.get()
        );

        // BUNDLED route (fresh candidates + fresh cold field).
        let bundled_pool = pool(&position, orders);
        let bundled_inner = config.materialize();
        let bundled_field = CountingField::new(bundled_inner.as_ref());
        let candidates: Vec<&dyn SlicePolicy> =
            bundled_pool.iter().map(|p| p as &dyn SlicePolicy).collect();
        let start = Instant::now();
        let bundled = bundled_set_outcomes(&root, &position, &candidates, &bundled_field);
        let bundled_us = start.elapsed().as_micros();
        println!(
            "  [{}] bundled:   wall_us={} nodes={} early_settled={} terminal_settled={} field_queries={}",
            config.label(),
            bundled_us,
            bundled.nodes(),
            bundled.early_settled(),
            bundled.terminal_settled(),
            bundled_field.queries.get()
        );

        let bundled_wins: Vec<u128> = (0..m).map(|k| bundled.wins(k)).collect();
        assert_eq!(
            wins, bundled_wins,
            "the two routes agree on every wins total"
        );
        println!("  [{}] wins agree: {wins:?}", config.label());
    }
}

fn main() {
    println!("== bundle bench (exploratory; integer micros; single-shot walls)");
    let r = receipt();
    let m3: Vec<Vec<Domino>> = vec![descending(), ascending(), stride(3, 1)];
    let m4: Vec<Vec<Domino>> = vec![ascending(), stride(5, 2), stride(13, 0), descending()];
    bench_root(&r, 4, 6, &m3);
    bench_root(&r, 11, 5, &m4);
    bench_root(&r, 11, 4, &m3);
}
