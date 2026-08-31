//! EXPLORATORY SCORE-PROFILE INSTRUMENT (anytime proof-state Phase 2;
//! `walt/math/anytime_proof_state_score_v0.1.md` §2–§4, §10–§11, §18,
//! ruling APS-A2) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: per-root exact 43-bin
//! score profiles of ONE frozen focal policy under the declared field —
//! the full bid-threshold curve from one run, the exact expected score
//! by the §3 tail-sum identity, the §10/§11 rescue and fragile-make
//! band masses at the root's own contract, and the honest price of the
//! whole curve (the profile walk forgoes the decided cutoff; its wall
//! and node counts are printed beside the truncating success-mass
//! walk's). Never a play-strength claim; a profile is the record of one
//! policy — no envelope across policies is built here (APS-A4).
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind as the field
//! under `SupportOracle`; two frozen focals — lowest-first and the σ0
//! mind itself playing the viewer's hand. Frozen `verify_player`
//! receipt roots: the ten gated Slice F/G roots.
//!
//! Modes:
//!   `factorprofile report <out.txt>` — the Phase 2 probe
//!
//! No floats anywhere; wall time is integer microseconds; probabilities
//! print as integer permille of exact rationals.

use std::io::Write as _;
use std::time::Instant;

use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::factor_belief::{
    viewer_score_profile, viewer_success_mass, FactorBelief, RecursionStats, ScoreProfile,
    SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::policy::{DecisionMode, TieRule};

const GATED_ROOTS: [(usize, usize); 10] = [
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

fn root_at(r: &Receipt, hand_id: usize, trick_no: usize) -> (CanonicalRoot, RootPosition) {
    let hand = &r.hands[hand_id];
    assert_eq!(hand.id, hand_id);
    let kernel = Kernel::from_receipt_trick(hand, trick_no).expect("a valid kernel");
    let position = RootPosition::from_receipt_trick(hand, trick_no).expect("a valid position");
    (CanonicalRoot::new(kernel), position)
}

fn micros(t: Instant) -> u128 {
    t.elapsed().as_micros()
}

/// Integer permille of the exact pair `num/den`, floored.
fn permille(num: u128, den: u128) -> u128 {
    assert!(den > 0, "a fiber is nonempty");
    num.checked_mul(1000).expect("fits") / den
}

/// The declaring-side make mass at threshold `k`, flipped to the
/// viewer's objective when the viewer is setting.
fn viewer_mass_at(profile: &ScoreProfile, declaring_viewer: bool, k: u32) -> u128 {
    let tail = profile.tail(k);
    if declaring_viewer {
        tail
    } else {
        profile.total() - tail
    }
}

#[allow(clippy::too_many_arguments)]
fn run_one(
    out: &mut std::fs::File,
    root: &CanonicalRoot,
    position: &RootPosition,
    hand_id: usize,
    trick_no: usize,
    focal_name: &str,
    focal: &dyn SlicePolicy,
) {
    let oracle = SupportOracle;
    let declaring_viewer = root.kernel().viewer().team() == position.declaring_team;

    let field_p = level0_field();
    let belief_p = FactorBelief::uniform_root(root, position, &field_p);
    let t0 = Instant::now();
    let mut pstats = RecursionStats::default();
    let profile = viewer_score_profile(&oracle, &belief_p, focal, &field_p, &mut pstats);
    let profile_us = micros(t0);

    let field_m = level0_field();
    let belief_m = FactorBelief::uniform_root(root, position, &field_m);
    let t1 = Instant::now();
    let mut mstats = RecursionStats::default();
    let mass = viewer_success_mass(&oracle, &belief_m, focal, &field_m, &mut mstats);
    let mass_us = micros(t1);

    let z = profile.total();
    assert_eq!(
        viewer_mass_at(&profile, declaring_viewer, position.bid),
        mass,
        "the tail projection equals the success mass (gate family 1, live here too)"
    );

    writeln!(
        out,
        "-- h{hand_id}-t{trick_no} focal={focal_name} viewer={} bid={} Z={z}",
        if declaring_viewer {
            "declaring"
        } else {
            "setting"
        },
        position.bid,
    )
    .expect("write");
    let bins: Vec<String> = profile
        .bins
        .iter()
        .enumerate()
        .filter(|(_, m)| **m > 0)
        .map(|(s, m)| format!("{s}:{m}"))
        .collect();
    writeln!(out, "   bins {}", bins.join(" ")).expect("write");
    let curve: Vec<String> = (1..=42u32)
        .map(|k| format!("{}", permille(profile.tail(k), z)))
        .collect();
    writeln!(out, "   tail-permille k=1..42 {}", curve.join(" ")).expect("write");
    writeln!(
        out,
        "   pmake(bid)={mass}/{z} ({}‰ viewer objective)  expected_score_milli={}",
        permille(mass, z),
        permille(profile.point_mass_sum(), z),
    )
    .expect("write");
    let c = position.bid;
    let rescue: Vec<String> = [1u32, 5, 10]
        .iter()
        .map(|d| {
            let band = profile.tail(c.saturating_sub(*d)) - profile.tail(c);
            format!("d={d}:{}‰", permille(band, z))
        })
        .collect();
    let fragile: Vec<String> = [1u32, 5, 10]
        .iter()
        .map(|d| {
            let band = profile.tail(c) - profile.tail(c + d);
            format!("d={d}:{}‰", permille(band, z))
        })
        .collect();
    writeln!(
        out,
        "   rescue-band(§10) {}   fragile-make(§11) {}",
        rescue.join(" "),
        fragile.join(" "),
    )
    .expect("write");
    writeln!(
        out,
        "   cost profile {profile_us} µs ({} focal / {} hidden / {} terminals) vs \
         success-mass {mass_us} µs ({} focal / {} hidden / {} early+{} terminal decided)",
        pstats.focal_nodes,
        pstats.hidden_nodes,
        pstats.decided_terminal,
        mstats.focal_nodes,
        mstats.hidden_nodes,
        mstats.decided_early,
        mstats.decided_terminal,
    )
    .expect("write");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args[1] != "report" {
        eprintln!("usage: factorprofile report <out.txt>");
        std::process::exit(2);
    }
    let mut out = std::fs::File::create(&args[2]).expect("the output file opens");
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    let receipt: Receipt = parse_file(&path).expect("the receipt parses");

    writeln!(
        out,
        "score-profile instrument (anytime proof-state Phase 2, §18/APS-A2)\n\
         field=Level0{{n0=2}} oracle=SupportOracle focals=[lowest-first, sigma0-as-focal]\n\
         one exact 43-bin profile per (root, focal): the full bid-threshold curve from one\n\
         run; the profile walk forgoes the decided cutoff (§18's caveat) and its price is\n\
         printed beside the truncating walk's. every mass integer, every ratio exact.\n"
    )
    .expect("write");

    for (hand_id, trick_no) in GATED_ROOTS {
        let (root, position) = root_at(&receipt, hand_id, trick_no);
        let low = FixedPreference::lowest_first("focal:lowest-first");
        run_one(
            &mut out,
            &root,
            &position,
            hand_id,
            trick_no,
            "lowest-first",
            &low,
        );
        let sigma0 = level0_field();
        run_one(
            &mut out,
            &root,
            &position,
            hand_id,
            trick_no,
            "sigma0-as-focal",
            &sigma0,
        );
    }
    writeln!(
        out,
        "\nno root was dropped: all ten gated roots ran to completion under both focals."
    )
    .expect("write");
}
