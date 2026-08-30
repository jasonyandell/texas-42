//! EXPLORATORY FACTOR-RECURSION INSTRUMENT (counted-belief Slice D;
//! `walt/math/counted_belief_sandwich_v0.1.md` §23/§25/§47, rulings
//! CBS-A6/CBS-A9) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: per-root frozen-policy
//! success masses by both routes (the §23 factorized recursion over the
//! support backend versus the bundled complete-world walk), exact value
//! pairs `M/Z`, wall time per route, and the recursion's node census
//! (focal/hidden/decided, conditionings). Never a play-strength claim —
//! a frozen focal policy under a modeled field is an evaluation subject,
//! not a recommendation.
//!
//! DECLARED EPOCH: deterministic fields only — the trivial lowest-first
//! preference and the σ0 Level0 { n0 = 2 } modeled mind (stage C1's
//! declared cached field). Frozen `verify_player` receipt roots: the six
//! enumerable trick-5/6 fibers of the Slice C probes, plus trick-4 roots
//! of the same hands as a depth/scale coordinate (16 post-root plies).
//! The opening root is NOT attempted here: a trick-1 recursion's
//! conditioned completions walk 116,280-hand tables per node, and that
//! cost is a coordinate for a later slice, not a Slice D claim.
//!
//! Modes:
//!   `factorrecursion report <out.txt>` — the Slice D probe: both routes
//!                                        on every root, both fields,
//!                                        parity asserted inline
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact integer pairs with an integer permille.

use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::bundle::bundled_set_outcomes;
use walt::solver::factor_belief::{
    viewer_success_mass, ExactCoverOracle, FactorBelief, RecursionStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
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

/// Both routes at one root under one (focal, field) pair: the factorized
/// recursion first, then the bundled walk, parity asserted; one report
/// line. Field instances are the caller's — fresh per route where the
/// route independence matters.
#[allow(clippy::too_many_arguments)]
fn both_routes(
    out: &mut String,
    label: &str,
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: &dyn SlicePolicy,
    field_recursion: &dyn SlicePolicy,
    field_bundled: &dyn SlicePolicy,
) {
    let oracle = SupportOracle;
    let belief = FactorBelief::uniform_root(root, position, field_recursion);
    let z = oracle.mass(&belief);
    assert_eq!(z, root.count(), "the uniform root mass is the fiber count");
    let mut stats = RecursionStats::default();
    let t0 = Instant::now();
    let mass = viewer_success_mass(&oracle, &belief, focal, field_recursion, &mut stats);
    let recursion_us = micros(t0);
    let t1 = Instant::now();
    let bundled = bundled_set_outcomes(root, position, &[focal], field_bundled);
    let bundled_us = micros(t1);
    assert_eq!(
        mass,
        bundled.wins(0),
        "the factorized recursion equals the bundled walk ({label})"
    );
    let permille = mass.checked_mul(1000).expect("an exact mass fits u128") / z;
    out.push_str(&format!(
        "{label}: Z {z}, M {mass} (permille {permille}), recursion {recursion_us} us, \
         bundled {bundled_us} us PARITY, nodes focal {} hidden {} decided {}+{}, \
         conditionings {}\n",
        stats.focal_nodes,
        stats.hidden_nodes,
        stats.decided_early,
        stats.decided_terminal,
        stats.conditionings,
    ));
}

fn report(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let mut out = String::new();
    out.push_str(
        "EXPLORATORY factor-recursion probe (Slice D, \
         counted_belief_sandwich_v0.1.md §23/§25/§47)\n\
         Below every evidentiary tier; cited by nothing above it.\n\
         M = viewer-objective success mass under (focal, field); the §23 \
         value is the exact pair M/Z.\n\n",
    );

    let roots: [(usize, usize); 6] = [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)];
    let trivial = FixedPreference::lowest_first("field:lowest-first");
    let low = FixedPreference::lowest_first("focal:lowest-first");
    let high = FixedPreference::highest_first("focal:highest-first");

    // Section A — the six enumerable Slice C roots, trivial field, two
    // frozen focal policies.
    out.push_str("SECTION A — receipt roots, trivial field (lowest-first)\n\n");
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&receipt, hand_id, trick_no).expect("a frozen root");
        for (name, focal) in [("low", &low as &dyn SlicePolicy), ("high", &high)] {
            let label = format!("h{hand_id}-t{trick_no} focal {name}");
            both_routes(
                &mut out, &label, &root, &position, focal, &trivial, &trivial,
            );
        }
    }

    // Section B — the same roots under the σ0 modeled mind, fresh field
    // instances per route (the C1 determinism law makes them agree); the
    // recursion instance's cache census is the reuse coordinate.
    out.push_str("\nSECTION B — receipt roots, level-0 field (n0 = 2)\n\n");
    for (hand_id, trick_no) in roots {
        let (root, position) = root_at(&receipt, hand_id, trick_no).expect("a frozen root");
        let field_r = FieldModel::new(field_spec());
        let field_b = FieldModel::new(field_spec());
        let label = format!("h{hand_id}-t{trick_no} focal low");
        both_routes(&mut out, &label, &root, &position, &low, &field_r, &field_b);
        out.push_str(&format!(
            "  sigma0 states materialized: recursion route {}, bundled route {}\n",
            field_r.cache_len(),
            field_b.cache_len(),
        ));
    }

    // Section C — trick-4 roots of the same hands: 16 post-root plies,
    // the deepest recursion this probe attempts. Fibers are still
    // enumerable (16 remaining tiles), so parity is asserted on every
    // row, both fields.
    out.push_str(
        "\nSECTION C — trick-4 roots (16 post-root plies; the depth/scale \
         coordinate)\n\n",
    );
    for hand_id in [3, 4, 8, 12] {
        let Some((root, position)) = root_at(&receipt, hand_id, 4) else {
            out.push_str(&format!("h{hand_id}-t4: no trick-start root (skipped)\n"));
            continue;
        };
        let label = format!("h{hand_id}-t4 focal low");
        both_routes(&mut out, &label, &root, &position, &low, &trivial, &trivial);
        let field_r = FieldModel::new(field_spec());
        let field_b = FieldModel::new(field_spec());
        let label = format!("h{hand_id}-t4 focal low sigma0");
        both_routes(&mut out, &label, &root, &position, &low, &field_r, &field_b);
        out.push_str(&format!(
            "  sigma0 states materialized: recursion route {}, bundled route {}\n",
            field_r.cache_len(),
            field_b.cache_len(),
        ));
    }

    std::fs::write(out_path, &out).expect("the probe record writes");
    print!("{out}");
    println!("wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("report") if args.len() == 3 => report(&args[2]),
        _ => {
            eprintln!("usage: factorrecursion report <out.txt>");
            std::process::exit(2);
        }
    }
}
