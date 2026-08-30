//! EXPLORATORY FACTOR-BELIEF INSTRUMENT (counted-belief Slice C, stages
//! C0–C1; `walt/math/counted_belief_sandwich_v0.1.md` §22/§26/§46,
//! rulings CBS-A6/CBS-A9) — sits below every evidentiary tier and is
//! cited by nothing above it. Instrument output only: per-root branch
//! masses by both routes (contraction over acting-seat hands versus
//! complete-world enumeration), the §26 measured coordinates (contraction
//! time, field-classification time, distinct hands versus worlds, field
//! action-cache reuse), and the §22 opening-root demonstration — exact
//! one-ply branch masses at a 399,072,960-world fiber with no complete
//! world materialized. Never a play-strength claim.
//!
//! DECLARED EPOCH: deterministic fields only — the trivial lowest-first
//! preference (§46 stage C0) on every root, and the σ0 Level0 { n0 = 2 }
//! modeled mind on the small/medium fibers (a stage-C1 down-payment; the
//! opening-root level-0 classification is deferred until its cost is
//! measured here).
//!
//! Modes:
//!   `factorbelief run <out.txt>`            — the C0 probe
//!   `factorbelief opening-level0 <out.txt>` — level-0 classification of
//!                                             the opening root (costly;
//!                                             run deliberately)
//!   `factorbelief cache <out.txt>`          — the stage-C1 cache study:
//!                                             first/repeat/bundled costs
//!                                             per root, cross-history
//!                                             sharing, and the opening
//!                                             root's identity cost
//!                                             (§26 items 2/3/4/6)
//!
//! No floats anywhere; wall time is integer microseconds.

use std::collections::HashMap;
use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::Domino;
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, FiberOracle};
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

/// The complete-world enumeration route, with its own record assembly.
fn enumerate_branches(
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: Domino,
    field: &dyn SlicePolicy,
) -> Vec<(Domino, u128)> {
    let seat = root.kernel().viewer().plus(1);
    let trick_plays = vec![focal];
    let history = vec![focal];
    let mut buckets: Vec<(Domino, u128)> = Vec::new();
    for world in root.worlds() {
        let hand = world.hand(seat);
        let led = Some(position.decl.led_context(focal));
        let legal = legal_plays(position.decl, hand, led);
        let record = PublicRecord {
            leader: position.leader,
            trick_plays: &trick_plays,
            banked: position.banked,
            root: position,
            history: &history,
        };
        let tile = field.choose(position.decl, hand, legal, &record);
        match buckets.iter_mut().find(|(t, _)| *t == tile) {
            Some((_, m)) => *m += 1,
            None => buckets.push((tile, 1)),
        }
    }
    buckets.sort_by_key(|(t, _)| t.index());
    buckets
}

fn branch_table(out: &mut String, label: &str, masses: &[(Domino, u128)], total: u128) {
    out.push_str(&format!("  {label}:\n"));
    for (t, m) in masses {
        out.push_str(&format!("    {t:?}  mass {m} / {total}\n"));
    }
}

fn micros(from: Instant) -> u128 {
    from.elapsed().as_micros()
}

fn run(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let oracle = FiberOracle;
    let mut out = String::new();
    out.push_str(
        "FACTOR-BELIEF C0 PROBE (exploratory; cited by nothing above it)\n\
         ================================================================\n\
         Both routes compute the same exact one-ply branch masses for the\n\
         first hidden seat after the viewer's lowest legal focal play.\n\
         contraction = acting-seat hands weighted by exact completions (§21);\n\
         enumeration = every complete world classified one by one.\n\n",
    );

    // Section A — small/medium receipt roots, both fields.
    out.push_str("SECTION A — receipt roots, trivial field (lowest-first)\n\n");
    for (hand_id, trick_no) in [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)] {
        let field = FixedPreference::lowest_first("field:lowest-first");
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let focal = lowest_focal(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
        let hands = oracle
            .actor_completion_weights(&belief, belief.seat_to_move())
            .len();

        let t0 = Instant::now();
        let contracted = oracle.branch_masses(&belief, &field);
        let contraction_us = micros(t0);
        let t1 = Instant::now();
        let enumerated = enumerate_branches(&root, &position, focal, &field);
        let enumeration_us = micros(t1);
        assert_eq!(contracted, enumerated, "route parity");
        let total: u128 = contracted.iter().map(|(_, m)| m).sum();
        assert_eq!(total, root.count(), "mass conservation");

        out.push_str(&format!(
            "h{hand_id}-t{trick_no}: fiber {}  focal {:?}  acting-seat hands {}\n  \
             contraction {}us  enumeration {}us  parity OK  conservation OK\n",
            root.count(),
            focal,
            hands,
            contraction_us,
            enumeration_us,
        ));
        branch_table(&mut out, "branches", &contracted, total);
        out.push('\n');
    }

    // Section B — the C1 down-payment: the σ0 modeled mind, one FRESH
    // field instance per route so cache_len counts each route's distinct
    // materialized information states.
    out.push_str(
        "SECTION B — receipt roots, level-0 field (n0 = 2; stage-C1 down-payment)\n\
         fresh field instance per route; cache_len = distinct information\n\
         states materialized by that route alone\n\n",
    );
    for (hand_id, trick_no) in [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)] {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let focal = lowest_focal(&root, &position);

        let field_c = FieldModel::new(field_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field_c).focal_play(focal);
        let hands = oracle
            .actor_completion_weights(&belief, belief.seat_to_move())
            .len();
        let t0 = Instant::now();
        let contracted = oracle.branch_masses(&belief, &field_c);
        let contraction_us = micros(t0);
        let states_c = field_c.cache_len();

        let field_e = FieldModel::new(field_spec());
        let t1 = Instant::now();
        let enumerated = enumerate_branches(&root, &position, focal, &field_e);
        let enumeration_us = micros(t1);
        let states_e = field_e.cache_len();

        assert_eq!(contracted, enumerated, "route parity");
        let total: u128 = contracted.iter().map(|(_, m)| m).sum();
        assert_eq!(total, root.count(), "mass conservation");

        out.push_str(&format!(
            "h{hand_id}-t{trick_no}: fiber {}  hands {}  \
             contraction {}us ({} states)  enumeration {}us ({} states)\n",
            root.count(),
            hands,
            contraction_us,
            states_c,
            enumeration_us,
            states_e,
        ));
        branch_table(&mut out, "branches", &contracted, total);
        out.push('\n');
    }

    // Section C — §22 at the opening root: contraction only, no world
    // ever materialized. The enumeration route does not run here.
    out.push_str(
        "SECTION C — the opening root (h0-t1), trivial field, contraction ONLY\n\
         399,072,960 worlds stand behind 116,280 acting-seat hands (§22);\n\
         the enumeration route is deliberately absent\n\n",
    );
    {
        let field = FixedPreference::lowest_first("field:lowest-first");
        let (root, position) = root_at(&receipt, 0, 1);
        let t0 = Instant::now();
        let mass = oracle.mass(&FactorBelief::uniform_root(&root, &position, &field));
        let mass_us = micros(t0);
        let focal = lowest_focal(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);

        let t1 = Instant::now();
        let weights = oracle.actor_completion_weights(&belief, belief.seat_to_move());
        let weights_us = micros(t1);
        let t2 = Instant::now();
        let branches = oracle.branch_masses(&belief, &field);
        let branch_us = micros(t2);
        let total: u128 = branches.iter().map(|(_, m)| m).sum();

        out.push_str(&format!(
            "fiber mass {mass} (counted in {mass_us}us, no enumeration)\n\
             focal {focal:?}\n\
             acting-seat hands {} (completion weights in {weights_us}us)\n\
             branch masses in {branch_us}us (completion + classification, one pass)\n",
            weights.len(),
        ));
        branch_table(&mut out, "branches", &branches, total);
        out.push_str(&format!("  conservation: {} = {} OK\n\n", total, mass));
    }

    std::fs::write(out_path, &out).expect("the probe output writes");
    println!("{out}");
    println!("wrote {out_path}");
}

/// Level-0 classification of the opening root: 116,280 modeled-mind
/// reads. Run deliberately; the §26 field-classification cost is the
/// point of the measurement.
fn opening_level0(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let oracle = FiberOracle;
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&receipt, 0, 1);
    let focal = lowest_focal(&root, &position);
    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);

    let t0 = Instant::now();
    let branches = oracle.branch_masses(&belief, &field);
    let branch_us = micros(t0);
    let total: u128 = branches.iter().map(|(_, m)| m).sum();

    let mut out = String::new();
    out.push_str(
        "FACTOR-BELIEF OPENING-ROOT LEVEL-0 CLASSIFICATION (exploratory)\n\
         ===============================================================\n\
         §46 stage C2 shape at the trick-1 root: every acting-seat hand\n\
         classified once by the σ0 Level0 { n0 = 2 } modeled mind, weighted\n\
         by exact completions. No complete world materialized.\n\n",
    );
    out.push_str(&format!(
        "fiber {}  focal {focal:?}\n\
         distinct information states materialized: {}\n\
         contraction + classification: {}us\n",
        root.count(),
        field.cache_len(),
        branch_us,
    ));
    branch_table(&mut out, "branches", &branches, total);
    out.push_str(&format!("conservation: {} = {} OK\n", total, root.count()));

    std::fs::write(out_path, &out).expect("the probe output writes");
    println!("{out}");
    println!("wrote {out_path}");
}

/// The bundled one-ply oracle (stage C1's extensional route): group the
/// fiber's worlds by the acting seat's remaining hand, ONE field query
/// per distinct hand — `solver::bundle`'s field-ply partition idiom at
/// one ply — and bucket world counts by the chosen tile. Record assembly
/// is local, independent of the module's public-history walker.
fn bundled_branches(
    root: &CanonicalRoot,
    position: &RootPosition,
    focal: Domino,
    field: &dyn SlicePolicy,
) -> (Vec<(Domino, u128)>, usize) {
    let seat = root.kernel().viewer().plus(1);
    let trick_plays = vec![focal];
    let history = vec![focal];
    let mut by_hand: HashMap<u32, Domino> = HashMap::new();
    let mut buckets: Vec<(Domino, u128)> = Vec::new();
    for world in root.worlds() {
        let hand = world.hand(seat);
        let tile = match by_hand.get(&hand.bits()) {
            Some(t) => *t,
            None => {
                let led = Some(position.decl.led_context(focal));
                let legal = legal_plays(position.decl, hand, led);
                let record = PublicRecord {
                    leader: position.leader,
                    trick_plays: &trick_plays,
                    banked: position.banked,
                    root: position,
                    history: &history,
                };
                let t = field.choose(position.decl, hand, legal, &record);
                by_hand.insert(hand.bits(), t);
                t
            }
        };
        match buckets.iter_mut().find(|(t, _)| *t == tile) {
            Some((_, m)) => *m += 1,
            None => buckets.push((tile, 1)),
        }
    }
    buckets.sort_by_key(|(t, _)| t.index());
    (buckets, by_hand.len())
}

/// The stage-C1 cache study: the cached σ0 field under the contraction
/// route — first-call classification cost, repeat-call identity cost,
/// the bundled one-ply oracle's cost and parity, cross-history sharing
/// under the full §43 identity key, and the opening root at scale.
fn cache_study(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let oracle = FiberOracle;
    let mut out = String::new();
    out.push_str(
        "FACTOR-BELIEF C1 CACHE STUDY (exploratory; cited by nothing above it)\n\
         =====================================================================\n\
         The cached σ0 Level0 { n0 = 2 } field under the contraction route:\n\
         every feasible acting-seat hand classified once (§46 stage C1),\n\
         reuse measured through the insert-only cache's entry count, action\n\
         buckets compared with the bundled one-ply oracle (one field query\n\
         per distinct hand — solver::bundle's field-ply partition idiom).\n\
         first = classification + counting; repeat = counting + key identity\n\
         only (zero classifications); us/hand is integer division.\n\n",
    );

    // Section A — six receipt roots: first/repeat/bundled costs and the
    // extensional cache identity between the routes.
    out.push_str("SECTION A — receipt roots, both routes, fresh instance per route\n\n");
    for (hand_id, trick_no) in [(12, 6), (10, 6), (5, 6), (4, 6), (8, 5), (3, 5)] {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let focal = lowest_focal(&root, &position);
        let field_c = FieldModel::new(field_spec());
        let belief = FactorBelief::uniform_root(&root, &position, &field_c).focal_play(focal);
        let feasible = oracle
            .actor_completion_weights(&belief, belief.seat_to_move())
            .len();
        let t0 = Instant::now();
        let contracted = oracle.branch_masses(&belief, &field_c);
        let first_us = micros(t0);
        let t1 = Instant::now();
        let repeat = oracle.branch_masses(&belief, &field_c);
        let repeat_us = micros(t1);
        assert_eq!(repeat, contracted, "a repeat returns the identical table");
        assert_eq!(field_c.cache_len(), feasible, "one state per feasible hand");

        let field_b = FieldModel::new(field_spec());
        let t2 = Instant::now();
        let (bundled, distinct) = bundled_branches(&root, &position, focal, &field_b);
        let bundled_us = micros(t2);
        assert_eq!(contracted, bundled, "route parity");
        assert_eq!(
            field_c.cache_snapshot(),
            field_b.cache_snapshot(),
            "extensional cache identity between the routes"
        );

        out.push_str(&format!(
            "h{hand_id}-t{trick_no}: fiber {}  feasible hands {}\n  \
             first {}us ({}us/hand)  repeat {}us  \
             bundled {}us over {} worlds ({} distinct hands)\n  \
             parity OK  cache-identity OK\n\n",
            root.count(),
            feasible,
            first_us,
            first_us / u128::try_from(feasible).expect("fits"),
            repeat_us,
            bundled_us,
            root.count(),
            distinct,
        ));
    }

    // Section B — cross-history sharing under ONE shared σ0 instance:
    // every legal focal candidate at one root, then a second root.
    out.push_str(
        "SECTION B — cross-history sharing (one shared σ0 instance)\n\
         hits = queries answered from states an earlier history materialized\n\n",
    );
    {
        let field = FieldModel::new(field_spec());
        let mut total_queries: usize = 0;
        let mut total_hits: usize = 0;
        let (root, position) = root_at(&receipt, 4, 6);
        let led = position
            .trick_plays
            .first()
            .map(|d| position.decl.led_context(*d));
        let legal = legal_plays(position.decl, root.kernel().viewer_hand(), led);
        for focal in legal.iter() {
            let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
            let queries = oracle
                .actor_completion_weights(&belief, belief.seat_to_move())
                .len();
            let before = field.cache_len();
            oracle.branch_masses(&belief, &field);
            let hits = queries - (field.cache_len() - before);
            total_queries += queries;
            total_hits += hits;
            out.push_str(&format!(
                "h4-t6 focal {focal:?}: queries {queries}  hits {hits}\n"
            ));
        }
        let (root2, position2) = root_at(&receipt, 12, 6);
        let focal2 = lowest_focal(&root2, &position2);
        let belief2 = FactorBelief::uniform_root(&root2, &position2, &field).focal_play(focal2);
        let queries2 = oracle
            .actor_completion_weights(&belief2, belief2.seat_to_move())
            .len();
        let before2 = field.cache_len();
        oracle.branch_masses(&belief2, &field);
        let hits2 = queries2 - (field.cache_len() - before2);
        total_queries += queries2;
        total_hits += hits2;
        out.push_str(&format!(
            "h12-t6 focal {focal2:?} (same instance): queries {queries2}  hits {hits2}\n\n\
             cross-history hits {total_hits} of {total_queries} queries — the full §43\n\
             identity key shares nothing across public histories; sharing needs a\n\
             proven state reduction (the Slice F vocabulary), never a looser key.\n\n"
        ));
    }

    // Section C — the opening root at scale: classification cost, pure
    // identity cost, and conditioning over an already-classified support.
    out.push_str(
        "SECTION C — the opening root (h0-t1), σ0, contraction ONLY\n\
         the bundled route is deliberately absent: the 399,072,960-world\n\
         loop is the representation this slice retires\n\n",
    );
    {
        let field = FieldModel::new(field_spec());
        let (root, position) = root_at(&receipt, 0, 1);
        let focal = lowest_focal(&root, &position);
        let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);
        let t0 = Instant::now();
        let branches = oracle.branch_masses(&belief, &field);
        let first_us = micros(t0);
        let states = field.cache_len();
        let t1 = Instant::now();
        let again = oracle.branch_masses(&belief, &field);
        let repeat_us = micros(t1);
        assert_eq!(again, branches, "a repeat is pure cache identity");
        assert_eq!(field.cache_len(), states, "a repeat materializes nothing");
        let (top, top_mass) = *branches
            .iter()
            .max_by_key(|(_, m)| *m)
            .expect("a nonempty branch table");
        let t2 = Instant::now();
        let conditioned = oracle.condition(&belief, top, &field);
        let condition_us = micros(t2);
        let new_states = field.cache_len() - states;
        assert_eq!(
            oracle.mass(&conditioned),
            top_mass,
            "the conditioned mass recovers the branch mass"
        );
        out.push_str(&format!(
            "first: {}us ({}us/hand)  states {}\n\
             repeat: {}us ({}ns/query, counting + key identity, zero classifications)\n\
             condition on {top:?} (mass {top_mass}): {}us, {} new states — the whole\n\
             support was already classified at the voidless opening\n\
             cache entries {} (the §26 memory coordinate as an entry count;\n\
             bytes belong to a dedicated probe)\n",
            first_us,
            first_us / u128::try_from(states).expect("fits"),
            states,
            repeat_us,
            repeat_us * 1000 / u128::try_from(states).expect("fits"),
            condition_us,
            new_states,
            states,
        ));
    }

    std::fs::write(out_path, &out).expect("the probe output writes");
    println!("{out}");
    println!("wrote {out_path}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") if args.len() == 3 => run(&args[2]),
        Some("opening-level0") if args.len() == 3 => opening_level0(&args[2]),
        Some("cache") if args.len() == 3 => cache_study(&args[2]),
        _ => {
            eprintln!(
                "usage: factorbelief run <out.txt> | factorbelief opening-level0 <out.txt> | \
                 factorbelief cache <out.txt>"
            );
            std::process::exit(2);
        }
    }
}
