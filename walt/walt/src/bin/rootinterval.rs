//! EXPLORATORY ROOT-INTERVAL INSTRUMENT (counted-belief Slice A, §44
//! probe; `walt/math/counted_belief_sandwich_v0.1.md`, rulings
//! CBS-A1..A9) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: per-root, per-action exact
//! `Q_a` beside the δ-valid root interval, the interval and survivor-set
//! evolution by declared prefix, worlds-to-singleton where it occurs,
//! and the two shortfall attributions the §44 probe demands — upper
//! looseness (`U_a − Q_a`, optimization-overfit cost) and lower
//! shortfall (`Q_a − L_a`, witness-policy quality). Never a
//! play-strength claim.
//!
//! DECLARED EPOCH: one field σ = Level0 { n0 = 2 }; pinned level-1
//! continuations at declared schedule [2, 2] as lower witnesses
//! (provenance FIXED — declared a priori, no on-stream selection); upper
//! stream epoch 0, evaluation stream epoch 1; δ = 1/20 per endpoint,
//! per action. Exact authority: `exact_root_value` per action.
//!
//! Roots: the affordable exact-root receipt fixtures by trick and fiber
//! size — h4-t6 (90), h10-t6 (19), h5-t6 (27), h12-t6 (6), h8-t5 (92),
//! h3-t5 (200).
//!
//! Mode: `rootinterval run <out.txt> [prefix]` (default prefix 16).
//!
//! No floats anywhere; wall time is integer microseconds.

use std::io::Write as _;
use std::time::Instant;

use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::rules::legal_plays;
use walt::rules::{Domino, DominoSet};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::evidence::ScopedDelta;
use walt::solver::exposure::exact_root_value;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{
    ActionRule, DecisionMode, FreezeTuple, FrozenPolicy, InnerSchedule, TieRule,
};
use walt::solver::root_interval::{
    decide, frozen_policy_lower, pmake_empirical_max_upper, PolicyProvenance, RootActionInterval,
};

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

fn legal_root_actions(root: &CanonicalRoot, position: &RootPosition) -> DominoSet {
    let led = position
        .trick_plays
        .first()
        .map(|d| position.decl.led_context(*d));
    legal_plays(position.decl, root.kernel().viewer_hand(), led)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "run" {
        eprintln!("usage: rootinterval run <out.txt> [prefix]");
        std::process::exit(2);
    }
    let prefix: u64 = args
        .get(3)
        .map(|s| s.parse().expect("an integer prefix"))
        .unwrap_or(16);
    assert!(prefix >= 1, "a declared prefix holds at least one world");
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");

    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");

    writeln!(
        out,
        "root-interval instrument (counted-belief Slice A, CBS-A1..A9)\n\
         field=Level0{{n0=2}} witness=PinnedThenLevel1[2,2] provenance=fixed\n\
         upper-epoch=0 eval-epoch=1 delta=1/20-per-endpoint prefix={prefix}\n"
    )
    .expect("write");

    for (hand_id, trick_no) in [(4usize, 6usize), (10, 6), (5, 6), (12, 6), (8, 5), (3, 5)] {
        let hand = &receipt.hands[hand_id];
        assert_eq!(hand.id, hand_id);
        let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
        let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
        let root = CanonicalRoot::new(kernel);
        let field = FieldModel::new(field_spec());
        let legal = legal_root_actions(&root, &position);
        let fiber = root.count();
        writeln!(
            out,
            "== root h{hand_id}-t{trick_no} fiber={fiber} legal={}",
            legal.len()
        )
        .expect("write");

        // The exact authority per action.
        let exact_start = Instant::now();
        let exact: Vec<(Domino, BigRational)> = legal
            .iter()
            .map(|a| (a, exact_root_value(&root, &position, a, &field).value()))
            .collect();
        let exact_us = exact_start.elapsed().as_micros();
        let q_max = exact.iter().map(|(_, v)| v.clone()).max().expect("actions");

        // The intervals at the declared prefix.
        let upper_start = Instant::now();
        let uppers: Vec<_> = legal
            .iter()
            .map(|a| {
                pmake_empirical_max_upper(
                    &root,
                    &position,
                    a,
                    &field,
                    0,
                    prefix,
                    ScopedDelta::new(format!("ri-{hand_id}-{trick_no}/{a}/upper"), q(1, 20)),
                )
            })
            .collect();
        let upper_us = upper_start.elapsed().as_micros();
        let lower_start = Instant::now();
        let lowers: Vec<_> = legal
            .iter()
            .map(|a| {
                let policy = pinned(&position, a);
                frozen_policy_lower(
                    &root,
                    &position,
                    &policy,
                    &field,
                    PolicyProvenance::Fixed,
                    1,
                    prefix,
                    ScopedDelta::new(format!("ri-{hand_id}-{trick_no}/{a}/lower"), q(1, 20)),
                )
            })
            .collect();
        let lower_us = lower_start.elapsed().as_micros();
        let intervals: Vec<RootActionInterval> = lowers
            .into_iter()
            .zip(uppers)
            .map(|(l, u)| RootActionInterval::new(l, u))
            .collect();

        // Per-action rows: exact beside interval, with both attributions.
        for interval in &intervals {
            let q_a = exact
                .iter()
                .find(|(a, _)| *a == interval.action())
                .map(|(_, v)| v.clone())
                .expect("an exact value");
            let l = interval.lower_value();
            let u = interval.upper_value();
            let covered = l <= q_a && q_a <= u;
            writeln!(
                out,
                "action={} exact={} lower={} upper={} shortfall={} excess={} covered={} optimal={}",
                interval.action(),
                q_a,
                l,
                u,
                &q_a - &l,
                &u - &q_a,
                covered,
                q_a == q_max
            )
            .expect("write");
        }

        // Survivor-set evolution by prefix, from the derived per-prefix
        // bound paths (no recomputation).
        let upper_paths: Vec<Vec<BigRational>> =
            intervals.iter().map(|i| i.upper.prefix_uppers()).collect();
        let lower_paths: Vec<Vec<BigRational>> =
            intervals.iter().map(|i| i.lower.prefix_lowers()).collect();
        let mut to_singleton: Option<u64> = None;
        let mut evolution = String::new();
        for t in 0..usize::try_from(prefix).expect("fits") {
            let bar = lower_paths
                .iter()
                .map(|p| p[t].clone())
                .max()
                .expect("actions");
            let surviving = upper_paths.iter().filter(|p| p[t] >= bar).count();
            evolution.push_str(&format!("{surviving} "));
            if surviving == 1 && to_singleton.is_none() {
                to_singleton = Some(u64::try_from(t + 1).expect("fits"));
            }
        }
        writeln!(out, "survivors-by-prefix: {}", evolution.trim_end()).expect("write");
        match to_singleton {
            Some(t) => writeln!(out, "worlds-to-singleton: {t}").expect("write"),
            None => writeln!(out, "worlds-to-singleton: not-reached").expect("write"),
        }
        let decision = decide(&intervals, legal);
        writeln!(
            out,
            "decision: {decision}\nwall-us: exact={exact_us} upper-walks={upper_us} \
             lower-replays={lower_us}\n"
        )
        .expect("write");
    }
    writeln!(out, "done").expect("write");
}
