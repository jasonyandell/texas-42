//! EXPLORATORY CONSEQUENCE-CEGAR INSTRUMENT (counted-belief Slice F;
//! `walt/math/counted_belief_sandwich_v0.1.md` §27–31/§49, rulings
//! CBS-A6/CBS-A9) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: per-root hand-class
//! censuses at the field-classification bottleneck — how much posterior
//! mass the bare §49 feature vocabulary concentrates in action-exact
//! classes, how many witnessed refinements reach the action-exact
//! endpoint, which tiles the witnesses force into the critical set, and
//! what interval the residual leaves on the branch masses at every
//! stage. §49's measurement discipline holds: residual class mass and
//! root-interval impact, never classifier accuracy — and the instrument
//! pays one field classification per support hand (the same bill as
//! `branch_masses`), so nothing here is a faster classifier. The
//! compression it measures is REPRESENTATIONAL: how few classes would
//! need verifying if an action-exact class verifier existed (§29's
//! vocabulary names that interface; building one is a later
//! construction).
//!
//! DECLARED EPOCH: deterministic fields only — the trivial lowest-first
//! preference and the σ0 Level0 { n0 = 2 } modeled mind. Frozen
//! `verify_player` receipt roots: the six enumerable trick-5/6 fibers,
//! the trick-4 roots of hands 3/4/8/12, and the opening root h0-t1
//! (116,280 acting hands over a 399,072,960-world fiber — §22's
//! standing-in ratio, never enumerated here).
//!
//! Modes:
//!   `factorcegar report <out.txt>` — the Slice F probe
//!
//! No floats anywhere; wall time is integer microseconds; shares are
//! integer permille.

use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::factor_belief::{
    refine_to_action_exact, CegarOutcome, ExactCoverOracle, FactorBelief, SupportOracle,
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

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn lowest_focal(root: &CanonicalRoot, position: &RootPosition) -> Domino {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
    legal.iter().next().expect("a legal focal tile")
}

fn micros(from: Instant) -> u128 {
    from.elapsed().as_micros()
}

fn permille(part: u128, whole: u128) -> u128 {
    part.checked_mul(1000).expect("an exact mass fits u128") / whole
}

fn tiles_of(set: DominoSet) -> String {
    let names: Vec<String> = set.iter().map(|d| d.to_string()).collect();
    if names.is_empty() {
        "-".to_string()
    } else {
        names.join(" ")
    }
}

/// The per-stage table plus the endpoint summary for one refinement
/// record.
fn stage_table(outcome: &CegarOutcome, z: u128) -> String {
    let mut out = String::new();
    out.push_str("    stage |crit| classes exact  exact-mass(permille)  max-width(permille)\n");
    for (i, stage) in outcome.stages.iter().enumerate() {
        let max_width = stage
            .branch_intervals
            .iter()
            .map(|(_, l, u)| u - l)
            .max()
            .unwrap_or(0);
        out.push_str(&format!(
            "    {:>5} {:>6} {:>7} {:>5} {:>12} ({:>4}) {:>11} ({:>4})\n",
            i,
            stage.critical.len(),
            stage.classes,
            stage.exact_classes,
            stage.exact_mass,
            permille(stage.exact_mass, z),
            max_width,
            permille(max_width, z),
        ));
    }
    let last = outcome.stages.last().expect("a final stage");
    out.push_str(&format!(
        "    endpoint: hands {}, classes {}, refinements {}, critical [{}]\n",
        outcome.hands,
        last.classes,
        outcome.witnesses.len(),
        tiles_of(last.critical),
    ));
    out
}

/// One root under one field: warm the field's action cache with the
/// exact contraction (the classification bill, timed), then run the
/// refinement loop on pure cache hits (the partition machinery, timed).
fn probe_root(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    field: &dyn SlicePolicy,
    out: &mut String,
) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, field).focal_play(focal);
    let oracle = SupportOracle;
    let z = oracle.mass(&belief);
    let t_classify = Instant::now();
    let exact = oracle.branch_masses(&belief, field);
    let classify_us = micros(t_classify);
    let t_refine = Instant::now();
    let outcome = refine_to_action_exact(&oracle, &belief, field);
    let refine_us = micros(t_refine);
    assert_eq!(
        outcome.branch_masses, exact,
        "the refined endpoint IS the exact contraction"
    );
    out.push_str(&format!(
        "  h{hand_id}-t{trick_no} focal {focal}: Z {z}, branch tiles {}, \
         classify+contract {classify_us} us, refine-loop {refine_us} us (warm cache)\n",
        exact.len(),
    ));
    out.push_str(&stage_table(&outcome, z));
    out.push('\n');
}

/// The Slice F report.
fn report(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let roots: [(usize, usize); 10] = [
        (12, 6),
        (10, 6),
        (5, 6),
        (4, 6),
        (8, 5),
        (3, 5),
        (3, 4),
        (4, 4),
        (8, 4),
        (12, 4),
    ];
    let mut out = String::new();
    out.push_str(
        "FACTOR-CEGAR PROBE (exploratory; cited by nothing above it)\n\
         Slice F — §49 consequence CEGAR at the field-classification\n\
         bottleneck: hand classes under the §28 feature map, §30\n\
         witness-pair refinement to the action-exact endpoint.\n\
         Stage tables: exact-mass = posterior mass in action-exact\n\
         classes; max-width = widest per-branch interval U_t - L_t\n\
         (the root-interval impact of the residual). All integers;\n\
         permille of Z.\n\n",
    );

    // Section A — the receipt roots under the σ0 modeled mind.
    out.push_str(
        "SECTION A — receipt roots, sigma0 Level0 { n0 = 2 } field\n\
         (classification warmed by the exact contraction first; the\n\
         refine-loop time is pure partition machinery)\n\n",
    );
    for (hand_id, trick_no) in roots {
        let field = FieldModel::new(field_spec());
        probe_root(&receipt, hand_id, trick_no, &field, &mut out);
    }

    // Section B — the same roots under the trivial preference field.
    out.push_str("SECTION B — the same roots, trivial lowest-first preference field\n\n");
    for (hand_id, trick_no) in roots {
        let field = FixedPreference::lowest_first("field:lowest-first");
        probe_root(&receipt, hand_id, trick_no, &field, &mut out);
    }

    // Section C — the opening root at scale (§22's ratio).
    out.push_str(
        "SECTION C — the opening root (h0-t1), sigma0: 116,280 acting\n\
         hands standing in for 399,072,960 worlds (never enumerated).\n\
         The classification bill is paid once by the exact contraction;\n\
         the refinement loop then partitions on pure cache hits.\n\n",
    );
    {
        let field = FieldModel::new(field_spec());
        let (root, position) = root_at(&receipt, 0, 1);
        let focal = lowest_focal(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
        let oracle = SupportOracle;
        let z = oracle.mass(&belief);
        assert_eq!(z, 399_072_960, "the opening fiber");
        let t_classify = Instant::now();
        let exact = oracle.branch_masses(&belief, &field);
        let classify_us = micros(t_classify);
        let states = field.cache_len();
        let t_refine = Instant::now();
        let outcome = refine_to_action_exact(&oracle, &belief, &field);
        let refine_us = micros(t_refine);
        assert_eq!(
            outcome.branch_masses, exact,
            "the refined endpoint IS the exact contraction"
        );
        assert_eq!(outcome.hands, 116_280, "the opening acting-hand count");
        let last = outcome.stages.last().expect("a final stage");
        out.push_str(&format!(
            "  h0-t1 focal {focal}: Z {z}, acting hands {}, sigma0 states {states},\n\
             \x20 branch tiles {}, classification+contraction {classify_us} us,\n\
             \x20 refine-loop {refine_us} us (warm cache, {} stages)\n",
            outcome.hands,
            exact.len(),
            outcome.stages.len(),
        ));
        out.push_str(&stage_table(&outcome, z));
        out.push_str(&format!(
            "\n  the endpoint: {} classes for {} hands — zero-residual\n\
             \x20 action-exactness under the SAMPLED sigma0 mind costs full\n\
             \x20 fragmentation (the S51 falsifier for the tail of the mass:\n\
             \x20 the feature set approaches all hidden tiles).\n",
            last.classes, outcome.hands,
        ));
        out.push_str(
            "  operating points (first stage at or above each exact-mass\n\
             \x20 share — S49's residual-interval discipline, not endpoint\n\
             \x20 chasing):\n",
        );
        for target in [500u128, 800, 900, 950] {
            if let Some(stage) = outcome
                .stages
                .iter()
                .find(|s| permille(s.exact_mass, z) >= target)
            {
                let max_width = stage
                    .branch_intervals
                    .iter()
                    .map(|(_, l, u)| u - l)
                    .max()
                    .unwrap_or(0);
                out.push_str(&format!(
                    "    >={target} permille exact: {} classes ({} hands/class), \
                     residual {} permille, max branch width {} permille\n",
                    stage.classes,
                    outcome.hands / stage.classes,
                    permille(stage.residual_mass, z),
                    permille(max_width, z),
                ));
            }
        }
        out.push_str(&format!(
            "  worlds never materialized: {z} (the contraction and every\n\
             \x20 partition walked hands, never deals).\n",
        ));
    }

    std::fs::write(out_path, &out).expect("the probe record writes");
    print!("{out}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("report") if args.len() == 3 => report(&args[2]),
        _ => {
            eprintln!("usage: factorcegar report <out.txt>");
            std::process::exit(2);
        }
    }
}
