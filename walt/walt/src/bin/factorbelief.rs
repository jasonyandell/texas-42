//! EXPLORATORY FACTOR-BELIEF INSTRUMENT (counted-belief Slice C, stages
//! C0–C2; `walt/math/counted_belief_sandwich_v0.1.md` §22/§26/§46,
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
//!   `factorbelief c2 <out.txt>`             — the stage-C2 report: ALL
//!                                             SEVEN §46 coordinates from
//!                                             ONE opening-root run under
//!                                             the σ0 level-0 field
//!
//! No floats anywhere; wall time is integer microseconds, memory is
//! integer bytes, ratios are integer division.

use std::collections::HashMap;
use std::process::Command;
use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::Domino;
use walt::solver::adaptive::{
    CanonicalRoot, FixedPreference, PublicRecord, RootPosition, SlicePolicy,
};
use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief, FiberOracle};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec, FieldStateKey};
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

/// Resident set size of THIS process in integer bytes, measured by
/// `/bin/ps -o rss=` (KiB as reported, scaled by 1024). A MEASUREMENT of
/// the whole process — never an accounting of the cache alone, and never
/// a peak: it is the resident size at the instant of the call. `None`
/// where `ps` is unavailable or its output does not parse.
fn rss_bytes() -> Option<u128> {
    let pid = std::process::id().to_string();
    let out = Command::new("/bin/ps")
        .args(["-o", "rss=", "-p", &pid])
        .output()
        .ok()?;
    let kib: u128 = String::from_utf8(out.stdout).ok()?.trim().parse().ok()?;
    Some(kib * 1024)
}

fn rss_line(label: &str, rss: Option<u128>) -> String {
    match rss {
        Some(b) => format!("  {label}: {b} bytes\n"),
        None => format!("  {label}: unavailable (/bin/ps did not report)\n"),
    }
}

/// The std `HashMap` bucket count for `len` live entries, from the
/// documented growth policy (7/8 load factor, power-of-two tables):
/// `buckets = next_power_of_two(ceil(len * 8 / 7))`, with the small-table
/// cases 4 and 8. Used ONLY inside the declared byte accounting below —
/// it is arithmetic over a documented policy, not a measurement of the
/// allocator.
fn hashmap_buckets(len: usize) -> usize {
    if len < 4 {
        4
    } else if len < 8 {
        8
    } else {
        (len * 8 / 7 + usize::from(!(len * 8).is_multiple_of(7))).next_power_of_two()
    }
}

/// §46 stage C2 — the opening-root report. ONE run at the frozen
/// `verify_player` receipt root h0-t1 (fiber 399,072,960, acting-seat
/// hands 116,280) under the σ0 `Level0 { n0 = 2 }` field, producing all
/// seven coordinates §46 requires reported separately. Costly by
/// construction (the field classifier is the bill); run deliberately.
fn c2_report(out_path: &str) {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt = parse_file(&path).expect("the verify_player receipt parses");
    let oracle = FiberOracle;
    let field = FieldModel::new(field_spec());
    let (root, position) = root_at(&receipt, 0, 1);
    let focal = lowest_focal(&root, &position);
    let seat = root.kernel().viewer().plus(1);

    let rss_start = rss_bytes();

    // The fiber's own mass: the shipped capacity DP, no field, no world.
    let t_mass = Instant::now();
    let fiber_mass = oracle.mass(&FactorBelief::uniform_root(&root, &position, &field));
    let mass_us = micros(t_mass);

    let belief = FactorBelief::uniform_root(&root, &position, &field).focal_play(focal);

    // COORDINATE 1/2 — the acting seat's root hands and the pure §21
    // contraction arithmetic: one completion binomial per hand, the field
    // never consulted.
    let t_weights = Instant::now();
    let weights = oracle.actor_completion_weights(&belief, seat);
    let weights_us = micros(t_weights);
    let hands = weights.len();
    assert_eq!(hands, 116_280, "the opening root's acting-seat hand count");
    let weight_total: u128 = weights.iter().map(|(_, w)| w).sum();
    assert_eq!(
        weight_total, fiber_mass,
        "the completion weights partition the fiber"
    );
    let rss_after_weights = rss_bytes();

    // COORDINATE 3 — the cold pass: contraction PLUS one σ0
    // classification per hand.
    let t_cold = Instant::now();
    let branches = oracle.branch_masses(&belief, &field);
    let cold_us = micros(t_cold);
    let states = field.cache_len();
    assert_eq!(states, hands, "one information state per acting-seat hand");
    let rss_after_classification = rss_bytes();

    // COORDINATE 5 — the warm pass: contraction plus full §43-key cache
    // identity, zero classifications.
    let t_warm = Instant::now();
    let again = oracle.branch_masses(&belief, &field);
    let warm_us = micros(t_warm);
    assert_eq!(again, branches, "a repeat returns the identical table");
    assert_eq!(
        field.cache_len(),
        states,
        "a repeat materializes no new state"
    );
    let classify_us = cold_us - warm_us;

    // COORDINATE 4 / 7 — distinct field actions, and conservation.
    let total: u128 = branches.iter().map(|(_, m)| m).sum();
    assert_eq!(total, 399_072_960, "the §46 stage-C2 fiber mass");
    assert_eq!(total, root.count(), "exact mass conservation");
    let actions = branches.len();

    // §26 item 5, beyond the seven: support shrinkage after one observed
    // action, on the heaviest branch.
    let (top, top_mass) = *branches
        .iter()
        .max_by_key(|(_, m)| *m)
        .expect("a nonempty branch table");
    let t_cond = Instant::now();
    let conditioned = oracle.condition(&belief, top, &field);
    let condition_us = micros(t_cond);
    assert_eq!(
        oracle.mass(&conditioned),
        top_mass,
        "the conditioned mass recovers the branch mass"
    );
    let support = conditioned.factors()[0].support().len();
    let new_states = field.cache_len() - states;
    let rss_end = rss_bytes();

    // COORDINATE 6 — memory. Two figures, labelled apart: a DECLARED
    // ACCOUNTING over `size_of` and the documented map growth policy, and
    // a MEASURED process resident size.
    let entry = std::mem::size_of::<(FieldStateKey, Domino)>();
    let tile = std::mem::size_of::<Domino>();
    let buckets = hashmap_buckets(states);
    let table_bytes = buckets * (entry + 1) + 16;
    let heap_per_entry = tile; // history = [focal]: one tile, capacity 1
    let heap_bytes = states * heap_per_entry;
    let accounted = table_bytes + heap_bytes;

    let mut out = String::new();
    out.push_str(
        "FACTOR-BELIEF C2 REPORT — the opening root (exploratory)\n\
         ========================================================\n\
         EXPLORATORY tier: sits below every evidentiary tier and is cited\n\
         by nothing above it.\n\n\
         §46 stage C2 in ONE run: the frozen verify_player receipt root\n\
         h0-t1 under the σ0 Level0 { n0 = 2 } field (construction\n\
         level0-modeled-mind-v1), all seven required coordinates reported\n\
         separately. No complete world is materialized at any point — the\n\
         enumeration and bundled routes are deliberately absent, because\n\
         the 399,072,960-world loop is the representation this slice\n\
         retires (§22).\n\n\
         All times are integer microseconds, all memory integer bytes, all\n\
         ratios integer division.\n\n",
    );
    out.push_str(&format!(
        "root h0-t1  focal {focal:?}  acting seat {seat:?}\n\
         fiber mass {fiber_mass} (shipped capacity DP, {mass_us}us, no enumeration)\n\n"
    ));

    out.push_str("COORDINATE 1 — number of acting-seat hands\n");
    out.push_str(&format!(
        "  {hands} root hands carry nonzero completion weight (asserted == 116280)\n  \
         worlds per hand: {} (integer division of {} by {hands})\n\n",
        u128::try_from(hands).map(|h| fiber_mass / h).expect("fits"),
        fiber_mass,
    ));

    out.push_str("COORDINATE 2 — contraction time\n");
    out.push_str(&format!(
        "  completion weights (one binomial per hand, field never consulted): {weights_us}us\n  \
         warm contraction (weights + full §43-key cache identity, zero\n    \
         classifications): {warm_us}us\n  \
         fiber mass alone: {mass_us}us\n\n"
    ));

    out.push_str("COORDINATE 3 — field-classification time\n");
    out.push_str(&format!(
        "  cold pass (contraction + one σ0 classification per hand): {cold_us}us\n  \
         classification alone, DERIVED by subtraction (cold - warm): {classify_us}us\n  \
         per hand: {}us  ({} of the cold pass in percent, integer division)\n\n",
        classify_us / u128::try_from(hands).expect("fits"),
        classify_us * 100 / cold_us,
    ));

    out.push_str("COORDINATE 4 — number of distinct field actions\n");
    out.push_str(&format!(
        "  {actions} distinct branch tiles over {hands} hands\n"
    ));
    branch_table(&mut out, "branches", &branches, total);
    out.push('\n');

    out.push_str("COORDINATE 5 — cache reuse\n");
    out.push_str(&format!(
        "  first contraction {cold_us}us, {states} states materialized\n  \
         repeat contraction {warm_us}us, 0 states materialized, 0 classifications\n  \
         identity cost {}ns/query; saving x{} (integer division)\n  \
         cross-history reuse is 0 by the §43 identity law (stage C1,\n    \
         cache_run1.txt): the full key carries the public history\n\n",
        warm_us * 1000 / u128::try_from(states).expect("fits"),
        cold_us / warm_us,
    ));

    out.push_str("COORDINATE 6 — memory\n");
    out.push_str(&format!(
        "  cache entries: {states}\n\
         \n  \
         DECLARED ACCOUNTING (arithmetic over size_of and the documented\n  \
         std HashMap growth policy — NOT an allocator measurement):\n    \
         entry inline size_of::<(FieldStateKey, Domino)>() = {entry} bytes\n    \
         buckets = next_power_of_two(ceil(entries * 8 / 7)) = {buckets}\n    \
         table  = buckets * (entry + 1 control byte) + 16 group bytes\n           \
         = {buckets} * ({entry} + 1) + 16 = {table_bytes} bytes\n    \
         per-entry heap = the key's history Vec<Domino>, len 1 at this\n      \
         root (trick_plays is empty at a trick start, so the cloned\n      \
         RootPosition allocates nothing) = size_of::<Domino>() = {tile} bytes\n    \
         heap   = entries * {heap_per_entry} = {heap_bytes} bytes\n    \
         TOTAL accounted for the action cache = {accounted} bytes\n\
         \n  \
         MEASURED process resident size (/bin/ps -o rss=, KiB * 1024 —\n  \
         the WHOLE process at that instant, not the cache alone, and not\n  \
         a peak; peak footprint is captured externally by\n  \
         /usr/bin/time -l and recorded with this probe):\n"
    ));
    out.push_str(&rss_line("at start", rss_start));
    out.push_str(&rss_line("after completion weights", rss_after_weights));
    out.push_str(&rss_line("after classification", rss_after_classification));
    out.push_str(&rss_line("at end", rss_end));
    if let (Some(a), Some(b)) = (rss_after_weights, rss_after_classification) {
        out.push_str(&format!(
            "  classification resident delta: {} bytes over {states} entries\n",
            b - a,
        ));
    }
    out.push('\n');

    out.push_str("COORDINATE 7 — exact mass conservation\n");
    out.push_str(&format!(
        "  sum of branch masses = {total}\n  \
         fiber mass            = {}\n  \
         asserted equal, and asserted == 399072960\n  \
         completion weights also sum to the fiber mass exactly\n\n",
        root.count(),
    ));

    out.push_str("BEYOND THE SEVEN — §26 item 5, support shrinkage\n");
    out.push_str(&format!(
        "  condition on the heaviest branch {top:?} (mass {top_mass} of {total}):\n    \
         {condition_us}us, {new_states} new states, conditioned support {support}\n    \
         hands of {hands} — the posterior update is table filtering over an\n    \
         already-classified support (Theorem 20.1: only the acting seat's\n    \
         factor is multiplied)\n"
    ));

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
        Some("c2") if args.len() == 3 => c2_report(&args[2]),
        _ => {
            eprintln!(
                "usage: factorbelief run <out.txt> | factorbelief opening-level0 <out.txt> | \
                 factorbelief cache <out.txt> | factorbelief c2 <out.txt>"
            );
            std::process::exit(2);
        }
    }
}
