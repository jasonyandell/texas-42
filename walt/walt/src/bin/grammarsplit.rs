//! EXPLORATORY GRAMMAR-SPLIT INSTRUMENT (counted-belief Slice B, §45
//! probe; `walt/math/counted_belief_sandwich_v0.1.md`, ruling CBS-A4) —
//! sits below every evidentiary tier and is cited by nothing above it.
//! Instrument output only: per-root, per-action exact §12 triples
//! (`free`/`gram`/`dev`), verdicts, first-deviation witnesses, the
//! grammar census, root-closure lines (`Q^G` best vs exact best), the
//! sampled-route triples, and the §8 residual-upper identity in the
//! numbers. Never a play-strength claim.
//!
//! DECLARED EPOCH: one field σ = Level0 { n0 = 2 } (the Slice A field).
//! Grammars: **G2** = {lowest-first, highest-first} (the two-preference
//! pair), **G3** = {pinned level-1 continuation at declared schedule
//! [2, 2], the σ0 modeled mind itself as a source, count-preservation
//! safety} — §45's list with the σ0 mind standing in for the
//! level-2/waking source (recorded deviation: neither is
//! `SlicePolicy`-shaped yet). The level-1 source is pinned per action to
//! the action under analysis, so it is in-grammar at every root state.
//! Sampled route: upper stream epoch 0.
//!
//! Roots: the affordable exact-root receipt fixtures by trick and fiber
//! size — h12-t6 (6), h10-t6 (19), h5-t6 (27), h4-t6 (90), h8-t5 (92),
//! h3-t5 (200). The opening root is OUT OF SCOPE for this walk: exact
//! and sampled grammar splits live where the Slice A optimizer lives
//! (the world-tree); the opening-scale grammar solve is §48's factorized
//! route (Slice E), not a bigger tree.
//!
//! Mode: `grammarsplit run <out.txt> [prefix]` (default prefix 64).
//!
//! No floats anywhere; wall time is integer microseconds.

use std::time::Instant;

use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition};
use walt::solver::evidence::ScopedDelta;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{
    exact_grammar_split, first_deviation, grammar_census, residual_empirical_max_upper,
    sampled_grammar_split, CountPreservation, GrammarVerdict, PolicyGrammar,
};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::root_interval::pmake_empirical_max_upper;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(num_bigint::BigInt::from(n), num_bigint::BigInt::from(d))
}

fn field_spec() -> FieldSpec {
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

const FIXTURES: [(usize, usize); 6] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)];

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

fn opt(v: Option<u64>) -> String {
    v.map_or_else(|| "-".to_string(), |x| x.to_string())
}

/// One grammar's exact sweep over one root: per-action triples, verdicts,
/// witnesses, census, and the closure line.
#[allow(clippy::too_many_lines)]
fn exact_section(
    out: &mut String,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &FieldModel,
    grammar_name: &str,
    build: &dyn Fn(Domino) -> Vec<Box<dyn walt::solver::adaptive::SlicePolicy>>,
) {
    let legal = legal_root_actions(root, position);
    let mut best_free: Option<(Domino, u64)> = None;
    let mut best_gram: Option<(Domino, u64)> = None;
    for action in legal.iter() {
        let sources = build(action);
        let refs: Vec<&dyn walt::solver::adaptive::SlicePolicy> =
            sources.iter().map(|b| b.as_ref()).collect();
        let grammar = PolicyGrammar::new(refs);
        let t0 = Instant::now();
        let split = exact_grammar_split(root, position, action, field, &grammar);
        let split_us = t0.elapsed().as_micros();
        let census = grammar_census(root, position, action, field, &grammar);
        let witness = match split.verdict() {
            GrammarVerdict::Counterexample => {
                let w = first_deviation(root, position, action, field, &grammar)
                    .expect("a counterexample carries a witness");
                format!(" witness={w}")
            }
            _ => String::new(),
        };
        out.push_str(&format!(
            "    {grammar_name} action={action} free={} gram={} dev={} verdict={} \
             census[focal={} G_total={} legal_total={} saturated={}] split_us={split_us}{witness}\n",
            split.free_count(),
            opt(split.grammar_count()),
            opt(split.deviation_count()),
            split.verdict(),
            census.focal_states,
            census.grammar_action_total,
            census.legal_action_total,
            census.saturated_states,
        ));
        if best_free.is_none_or(|(_, v)| split.free_count() > v) {
            best_free = Some((action, split.free_count()));
        }
        if let Some(g) = split.grammar_count() {
            if best_gram.is_none_or(|(_, v)| g > v) {
                best_gram = Some((action, g));
            }
        }
    }
    let (fa, fv) = best_free.expect("a legal action exists");
    match best_gram {
        Some((ga, gv)) => {
            let closes = if gv == fv { "YES" } else { "no" };
            out.push_str(&format!(
                "    {grammar_name} root-closure: exact_best={fa}({fv}) grammar_best={ga}({gv}) \
                 grammar_attains_root_optimum={closes}\n"
            ));
        }
        None => out.push_str(&format!(
            "    {grammar_name} root-closure: exact_best={fa}({fv}) grammar_best=- (no \
             in-grammar root action)\n"
        )),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let usage = "usage: grammarsplit run <out.txt> [prefix]";
    assert!(args.len() >= 3, "{usage}");
    assert_eq!(args[1], "run", "{usage}");
    let out_path = &args[2];
    let prefix: u64 = args.get(3).map_or(64, |s| s.parse().expect("a prefix"));
    let receipt_path = locate_verify_player().expect("rob/receipts/verify_player.txt");
    let receipt = parse_file(&receipt_path).expect("a parseable receipt");
    let field = FieldModel::new(field_spec());
    let mut out = String::new();
    out.push_str(
        "GRAMMAR-SPLIT PROBE (counted-belief Slice B, §45; EXPLORATORY, cited by nothing)\n",
    );
    out.push_str(&format!(
        "field={} grammars: G2={{lowest,highest}} G3={{level1[2,2]-pinned-per-action,sigma0-mind,\
         count-preservation}} sampled_prefix={prefix}\n\n",
        field.field_id()
    ));

    out.push_str("SECTION A — exact §12 triples over the complete fiber\n");
    for (hand_id, trick_no) in FIXTURES {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        out.push_str(&format!(
            "  h{hand_id}-t{trick_no} fiber={} viewer={:?}\n",
            root.count(),
            root.kernel().viewer()
        ));
        // G1: the singleton lowest-first grammar — one behavioral policy,
        // the sharpest counterexample generator.
        exact_section(&mut out, &root, &position, &field, "G1", &|_a| {
            vec![Box::new(FixedPreference::lowest_first(
                "preference:lowest-v1",
            ))]
        });
        // G2: the two-preference grammar (action-independent sources).
        exact_section(&mut out, &root, &position, &field, "G2", &|_a| {
            vec![
                Box::new(FixedPreference::lowest_first("preference:lowest-v1")),
                Box::new(FixedPreference::highest_first("preference:highest-v1")),
            ]
        });
        // G3: §45's source list — the level-1 continuation pinned to the
        // action under analysis, the σ0 modeled mind, the safety policy.
        let pos = position.clone();
        exact_section(&mut out, &root, &position, &field, "G3", &move |a| {
            vec![
                Box::new(pinned(&pos, a)),
                Box::new(FieldModel::new(field_spec())),
                Box::new(CountPreservation::new()),
            ]
        });
    }

    out.push_str(
        "\nSECTION B — the sampled route (declared stream prefix; exact on the sample, \
                  bounds nothing about the fiber)\n",
    );
    for (hand_id, trick_no) in [(8usize, 5usize), (3, 5)] {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        out.push_str(&format!(
            "  h{hand_id}-t{trick_no} fiber={} prefix={prefix}\n",
            root.count()
        ));
        let lowest = FixedPreference::lowest_first("preference:lowest-v1");
        let highest = FixedPreference::highest_first("preference:highest-v1");
        let safety = CountPreservation::new();
        let grammar = PolicyGrammar::new(vec![&lowest, &highest, &safety]);
        for action in legal_root_actions(&root, &position).iter() {
            let t0 = Instant::now();
            let split =
                sampled_grammar_split(&root, &position, action, &field, &grammar, 0, prefix);
            let us = t0.elapsed().as_micros();
            out.push_str(&format!(
                "    action={action} free={} gram={} dev={} verdict={} sampled_us={us}\n",
                split.free_count(),
                opt(split.grammar_count()),
                opt(split.deviation_count()),
                split.verdict(),
            ));
        }
        // The §8 identity in the numbers: the residual upper's count path
        // and bound coincide with the full-class upper's.
        let action = legal_root_actions(&root, &position)
            .iter()
            .next()
            .expect("a legal action");
        let residual = residual_empirical_max_upper(
            &root,
            &position,
            action,
            &field,
            &grammar,
            0,
            prefix.min(24),
            ScopedDelta::new(format!("probe/h{hand_id}t{trick_no}/residual"), q(1, 20)),
        );
        let full = pmake_empirical_max_upper(
            &root,
            &position,
            action,
            &field,
            0,
            prefix.min(24),
            ScopedDelta::new(format!("probe/h{hand_id}t{trick_no}/full"), q(1, 20)),
        );
        assert_eq!(residual.counts(), full.counts());
        assert_eq!(residual.upper(), full.upper());
        out.push_str(&format!(
            "    §8 identity at action={action}: residual upper == full-class upper == {} \
             (count paths byte-identical)\n",
            residual.upper()
        ));
    }

    std::fs::write(out_path, &out).expect("probe output written");
    println!("{out}");
}
