//! EXPLORATORY FACTOR-RESPONSE INSTRUMENT (counted-belief Slice E;
//! `walt/math/counted_belief_sandwich_v0.1.md` §11–12/§23/§48, rulings
//! CBS-A4/CBS-A6/CBS-A9) — sits below every evidentiary tier and is
//! cited by nothing above it. Instrument output only: per-root grammar
//! optima `Q^G` by the §48 factorized recursion, checked where
//! affordable against the Slice B enumeration split (gram/free/dev and
//! the §12 verdict), each grammar source's fixed-policy value beside
//! them (the dominance picture), wall time per route, and the
//! recursion's node census. Never a play-strength claim — a grammar
//! optimum under a modeled field is an evaluation subject, not a
//! recommendation. The §48 fence holds: nothing here maximizes over the
//! full action set; `free` comes only from the Slice B enumeration
//! split, as the bound `Q^G` is a lower witness OF.
//!
//! DECLARED EPOCH: deterministic fields only — the trivial lowest-first
//! preference and the σ0 Level0 { n0 = 2 } modeled mind. Grammar
//! sources: lowest-first, highest-first, and the `CountPreservation`
//! safety policy. Frozen `verify_player` receipt roots: the six
//! enumerable trick-5/6 fibers, plus trick-4 roots as the depth/scale
//! coordinate. The opening root is NOT attempted (the Slice D boundary,
//! unchanged).
//!
//! Modes:
//!   `factorresponse report <out.txt>` — the Slice E probe
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact integer pairs with an integer permille.

use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{
    grammar_success_mass, viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats,
    ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::grammar::{exact_grammar_split, CountPreservation, PolicyGrammar};
use walt::solver::policy::{DecisionMode, TieRule};

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

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> Option<(CanonicalRoot, RootPosition)> {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).ok()?;
    let position = RootPosition::from_receipt_trick(hand, trick_no).ok()?;
    Some((CanonicalRoot::new(kernel), position))
}

fn micros(from: Instant) -> u128 {
    from.elapsed().as_micros()
}

fn permille(mass: u128, z: u128) -> u128 {
    mass.checked_mul(1000).expect("an exact mass fits u128") / z
}

/// The grammar's action set at the root information state.
fn root_grammar_actions(
    root: &CanonicalRoot,
    position: &RootPosition,
    grammar: &PolicyGrammar<'_>,
) -> DominoSet {
    let hand = root.kernel().viewer_hand();
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, hand, led);
    let history: Vec<Domino> = Vec::new();
    let record = PublicRecord {
        leader: position.leader,
        trick_plays: &position.trick_plays,
        banked: position.banked,
        root: position,
        history: &history,
    };
    grammar.actions(position.decl, hand, legal, &record)
}

fn stats_line(stats: &ResponseStats) -> String {
    format!(
        "nodes focal {} (actions {}) hidden {} decided {}+{}, conditionings {}",
        stats.focal_nodes,
        stats.focal_actions,
        stats.hidden_nodes,
        stats.decided_early,
        stats.decided_terminal,
        stats.conditionings,
    )
}

/// One root under σ0: per grammar root action, the factorized `Q^G_a`
/// against the Slice B enumeration split (parity asserted), with the §12
/// verdict and both routes' wall time.
fn sigma0_actions(
    out: &mut String,
    label: &str,
    root: &CanonicalRoot,
    position: &RootPosition,
    grammar: &PolicyGrammar<'_>,
) {
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(root, position, &field);
    let z = oracle.mass(&belief);
    for action in root_grammar_actions(root, position, grammar).iter() {
        let mut stats = ResponseStats::default();
        let t0 = Instant::now();
        let mass = grammar_success_mass(
            &oracle,
            &belief.focal_play(action),
            grammar,
            &field,
            &mut stats,
        );
        let recursion_us = micros(t0);
        let t1 = Instant::now();
        let split = exact_grammar_split(root, position, action, &field, grammar);
        let split_us = micros(t1);
        let gram = split
            .grammar_count()
            .expect("a grammar root action has a grammar side");
        assert_eq!(
            mass,
            u128::from(gram),
            "the factorized Q^G_a equals the Slice B grammar optimum ({label} {action})"
        );
        out.push_str(&format!(
            "{label} a={action}: Z {z}, Q^G_a {mass} (permille {}), free {}, dev {}, \
             verdict {}, recursion {recursion_us} us, split {split_us} us PARITY, {}\n",
            permille(mass, z),
            split.free_count(),
            split
                .deviation_count()
                .map_or_else(|| "-".to_string(), |v| v.to_string()),
            split.verdict(),
            stats_line(&stats),
        ));
    }
}

/// One root under one field: the root grammar optimum and every source's
/// fixed-policy value (dominance asserted), one line each.
fn dominance_rows(
    out: &mut String,
    label: &str,
    root: &CanonicalRoot,
    position: &RootPosition,
    grammar: &PolicyGrammar<'_>,
    sources: &[(&str, &dyn SlicePolicy)],
    field: &dyn SlicePolicy,
) {
    let oracle = SupportOracle;
    let belief = FactorBelief::uniform_root(root, position, field);
    let z = oracle.mass(&belief);
    let mut stats = ResponseStats::default();
    let t0 = Instant::now();
    let q_g = grammar_success_mass(&oracle, &belief, grammar, field, &mut stats);
    let us = micros(t0);
    out.push_str(&format!(
        "{label}: Z {z}, Q^G {q_g} (permille {}), {us} us, {}\n",
        permille(q_g, z),
        stats_line(&stats),
    ));
    for (name, source) in sources {
        let mut r_stats = RecursionStats::default();
        let v = viewer_success_mass(&oracle, &belief, *source, field, &mut r_stats);
        assert!(v <= q_g, "a grammar source is dominated ({label} {name})");
        out.push_str(&format!(
            "  source {name}: M {v} (permille {}), gap {}\n",
            permille(v, z),
            q_g - v,
        ));
    }
}

fn report(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let mut out = String::new();
    out.push_str(
        "EXPLORATORY factor-response probe (Slice E, \
         counted_belief_sandwich_v0.1.md §11-12/§23/§48)\n\
         Below every evidentiary tier; cited by nothing above it.\n\
         Q^G = the exact grammar optimum as a success mass; the §12 value \
         is the exact pair Q^G/Z. free/dev come from the Slice B \
         enumeration split only (the §48 fence: the recursion never \
         maximizes over the full action set).\n\n",
    );

    let roots: [(usize, usize); 6] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)];
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");
    let safety = CountPreservation::new();
    let two = PolicyGrammar::new(vec![&low, &high]);
    let three = PolicyGrammar::new(vec![&low, &high, &safety]);
    let sources_two: [(&str, &dyn SlicePolicy); 2] = [("low", &low), ("high", &high)];
    let sources_three: [(&str, &dyn SlicePolicy); 3] =
        [("low", &low), ("high", &high), ("safety", &safety)];

    // Section A — σ0, per grammar root action, against the Slice B split.
    out.push_str(&format!(
        "SECTION A — receipt roots, level-0 field (n0 = 2), per grammar \
         root action; grammar {}\n\n",
        two.id()
    ));
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&receipt, hand_id, trick_no).expect("a frozen root");
        let label = format!("h{hand_id}-t{trick_no}");
        sigma0_actions(&mut out, &label, &root, &position, &two);
    }
    out.push_str(&format!("\n  same, grammar {}\n\n", three.id()));
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&receipt, hand_id, trick_no).expect("a frozen root");
        let label = format!("h{hand_id}-t{trick_no}");
        sigma0_actions(&mut out, &label, &root, &position, &three);
    }

    // Section B — the same roots under the trivial field: the root
    // grammar optimum and the dominance picture (no Slice B split here —
    // the enumeration split runs under FieldModel fields only).
    out.push_str(&format!(
        "\nSECTION B — receipt roots, trivial field (lowest-first); \
         grammar {}\n\n",
        three.id()
    ));
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&receipt, hand_id, trick_no).expect("a frozen root");
        let label = format!("h{hand_id}-t{trick_no}");
        dominance_rows(
            &mut out,
            &label,
            &root,
            &position,
            &three,
            &sources_three,
            &trivial,
        );
    }

    // Section C — trick-4 roots (16 post-root plies): the grammar
    // recursion at the Slice D depth coordinate. Trivial field on all
    // four; σ0 on hand 4 (the Slice D probe's deep σ0 row, now with the
    // max).
    out.push_str(&format!(
        "\nSECTION C — trick-4 roots (16 post-root plies); grammar {}\n\n",
        two.id()
    ));
    for hand_id in [3, 4, 8, 12] {
        let Some((root, position)) = root_at(&receipt, hand_id, 4) else {
            out.push_str(&format!("h{hand_id}-t4: no trick-start root (skipped)\n"));
            continue;
        };
        let label = format!("h{hand_id}-t4");
        dominance_rows(
            &mut out,
            &label,
            &root,
            &position,
            &two,
            &sources_two,
            &trivial,
        );
    }
    let (root, position) = root_at(&receipt, 4, 4).expect("a frozen root");
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let oracle = SupportOracle;
    let z = oracle.mass(&belief);
    let mut stats = ResponseStats::default();
    let t0 = Instant::now();
    let q_g = grammar_success_mass(&oracle, &belief, &two, &field, &mut stats);
    let us = micros(t0);
    out.push_str(&format!(
        "h4-t4 sigma0: Z {z}, Q^G {q_g} (permille {}), {us} us, {}\n\
        \x20 sigma0 states materialized: {}\n",
        permille(q_g, z),
        stats_line(&stats),
        field.cache_len(),
    ));

    std::fs::write(out_path, &out).expect("the probe record writes");
    print!("{out}");
    println!("wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("report") if args.len() == 3 => report(&args[2]),
        _ => {
            eprintln!("usage: factorresponse report <out.txt>");
            std::process::exit(2);
        }
    }
}
