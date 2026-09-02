//! EXPLORATORY GOD-GAP CENSUS INSTRUMENT (`solver::godgap`, slice U0)
//! — sits below every evidentiary tier and is cited by nothing above
//! it. Instrument output only: per root-action coordinate, the fiber
//! mass, the doomed mass, the God upper `U^God`, the exact `Q` where
//! affordable, the §8 three-part decomposition
//! `(d_phys, d_info, d_policy)`, the §48 result type, the extracted
//! God-tight policy id where equality landed, and the wall bill. Then
//! the §38 fusion-horizon table by trick depth, and a plain-language
//! two-regime summary. Never a play-strength claim, and never a
//! theorem: the horizon is an empirical object on a declared corpus
//! (SC-A4).
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frozen `verify_player` receipt; the corpus is
//! the ten gated roots of the Slice F epoch (the six enumerable
//! trick-5/trick-6 roots plus the four trick-4 roots) and the h0-t1
//! opening root; caps and doom spec in the REPORT constants below.
//!
//! Modes:
//!   `godgapreport scout <hand> <trick> [exact-cap] [profile-cap]`
//!       — one root, caller-declared caps, timing scout
//!   `godgapreport report <out.txt>` — the declared full run
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille.

use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::DominoSet;
use walt::solver::adaptive::{CanonicalRoot, RootPosition, SlicePolicy};
use walt::solver::doom::DoomSpec;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::godgap::{
    earliest_fusion_free_trick, fusion_horizon, GodGapCoordinate, GodGapResult, GodGapSpec,
    GodGapWalk,
};
use walt::solver::policy::{DecisionMode, TieRule};

/// The declared report caps. The exact cap admits every gated root of
/// the Slice F epoch (largest fiber 34,650) and excludes the opening
/// root (399,072,960). The profile cap is lower: the 43-bin profile
/// has no decided cutoff (§18), so it is bought only where it is
/// cheap.
const REPORT_EXACT_FIBER_CAP: u128 = 40_000;
const REPORT_PROFILE_FIBER_CAP: u128 = 12_000;
/// The doom spec used on the ENUMERABLE corpus (full census, ample).
const REPORT_DOOM_NODE_BUDGET: u64 = 10_000_000;
const REPORT_DOOM_WALK_CAP: u64 = 1_000_000;
/// The opening root's doom spec: the priority census, at a declared
/// FRACTION of `doomreport`'s committed budget. The committed run
/// (`probes/factor_belief/doomreport_run1.txt`, 500k nodes per action)
/// certified zero doom on every opening action; this cheaper pass is
/// re-run here only so the census's own opening verdict rests on its
/// own instrument, never on a quoted number.
const OPENING_DOOM_NODE_BUDGET: u64 = 50_000;
const OPENING_DOOM_WALK_CAP: u64 = 20_000;
const OPENING_DESCEND_TOP: usize = 8;

/// The corpus: `(hand_id, trick_no)`, the ten gated roots of the
/// Slice F epoch in increasing depth order.
const CORPUS: [(usize, usize); 10] = [
    (3, 4),
    (4, 4),
    (8, 4),
    (12, 4),
    (8, 5),
    (3, 5),
    (12, 6),
    (10, 6),
    (5, 6),
    (4, 6),
];

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

fn permille(v: &BigRational) -> i128 {
    let scaled = (v * BigRational::from_integer(BigInt::from(1000))).floor();
    i128::try_from(scaled.to_integer()).expect("a permille of a probability fits i128")
}

/// An exact rational, printed as `n/d` with its floored permille.
fn exact(v: &BigRational) -> String {
    format!("{v} ({}‰)", permille(v))
}

fn opt_exact(v: Option<&BigRational>) -> String {
    match v {
        Some(x) => exact(x),
        None => "REFUSED".to_string(),
    }
}

fn print_coordinate(out: &mut String, c: &GodGapCoordinate, wall_us: u128) {
    let doomed = c.upper.doomed_mass;
    out.push_str(&format!(
        "  {}: Z={} doomed={} U^God={} | {}\n",
        c.context.root_action,
        c.fiber_mass,
        doomed,
        exact(&c.upper.value),
        c.result.label()
    ));
    let q = match &c.result {
        GodGapResult::GodTightPolicy(t) => Some(t.value.clone()),
        GodGapResult::PositiveGodGap(p) => Some(p.q.clone()),
        GodGapResult::GodUpper | GodGapResult::UnknownGodGap => None,
    };
    out.push_str(&format!(
        "      Q={} | d_phys={} d_info={} d_policy={}\n",
        match &q {
            Some(v) => exact(v),
            None => "UNKNOWN".to_string(),
        },
        exact(&c.decomposition.d_phys),
        opt_exact(c.decomposition.d_info.as_ref()),
        opt_exact(c.decomposition.d_policy.as_ref()),
    ));
    if let Some(tight) = c.god_tight() {
        let r = &tight.equality_receipt;
        out.push_str(&format!(
            "      GOD-TIGHT receipt: policy={} states={} | L={}/{} = U^God with doomed={} \
             | root={} field={} contract={} belief={} utility={} | profile={}\n",
            tight.policy_id,
            r.policy_states,
            r.repriced_mass,
            r.fiber_mass,
            r.doomed_mass,
            tight.context.root_id,
            tight.context.field_id,
            tight.context.contract,
            r.belief_id,
            r.utility_id,
            if tight.profile.is_some() {
                "persisted"
            } else {
                "refused (cap)"
            },
        ));
        if tight.nothing_saveable() {
            out.push_str(
                "      NOTE: whole-fiber doom — every policy is God-tight here and the \
                 equality carries no information about blindness (counted apart)\n",
            );
        }
    }
    if !c.refusals.is_empty() {
        out.push_str(&format!("      refusals: {:?}\n", c.refusals));
    }
    out.push_str(&format!(
        "      cost: doom_nodes={} response_focal={} response_hidden={} extraction_focal={} \
         repricing_nodes={} | wall={}us\n",
        c.cost.doom_nodes,
        c.cost.response_focal,
        c.cost.response_hidden,
        c.cost.extraction_focal,
        c.cost.repricing_nodes,
        wall_us
    ));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let r = parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses");
    let field = FieldModel::new(field_spec());
    let oracle = walt::solver::factor_belief::SupportOracle;
    let full_doom = DoomSpec {
        node_budget: REPORT_DOOM_NODE_BUDGET,
        walk_cap: REPORT_DOOM_WALK_CAP,
        max_level: 3,
        critical: DominoSet::EMPTY,
        descend_top: None,
    };
    match mode {
        "scout" => {
            let hand_id: usize = args[2].parse().expect("a hand id");
            let trick_no: usize = args[3].parse().expect("a trick number");
            let exact_cap: u128 = args
                .get(4)
                .map_or(REPORT_EXACT_FIBER_CAP, |a| a.parse().expect("a cap"));
            let profile_cap: u128 = args
                .get(5)
                .map_or(REPORT_PROFILE_FIBER_CAP, |a| a.parse().expect("a cap"));
            let (root, position) = root_at(&r, hand_id, trick_no);
            let spec = GodGapSpec {
                exact_fiber_cap: exact_cap,
                profile_fiber_cap: profile_cap,
                doom: full_doom,
            };
            let walk = GodGapWalk {
                oracle: &oracle,
                root: &root,
                position: &position,
                field: &field,
                spec: &spec,
            };
            let mut out = String::new();
            out.push_str(&format!(
                "h{hand_id}-t{trick_no} bid={} caps exact={exact_cap} profile={profile_cap}\n",
                position.bid
            ));
            for action in walt::solver::godgap::legal_actions(&root, &position) {
                let mut progress = |done: u64, total: u64, doomed: u128, nodes: u64| {
                    eprintln!("  enum {done}/{total} outer hands: doomed {doomed} nodes {nodes}");
                };
                let t0 = Instant::now();
                let c = walk.god_gap(action, &mut progress);
                print_coordinate(&mut out, &c, t0.elapsed().as_micros());
            }
            print!("{out}");
        }
        "report" => {
            let path = args.get(2).expect("an output path").clone();
            let mut out = String::new();
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            out.push_str(&format!(
                "GOD-GAP CENSUS REPORT (slice U0) — field {}\n\
                 declared caps: exact_fiber_cap={REPORT_EXACT_FIBER_CAP} \
                 profile_fiber_cap={REPORT_PROFILE_FIBER_CAP}\n\
                 corpus doom spec: node_budget={REPORT_DOOM_NODE_BUDGET} \
                 walk_cap={REPORT_DOOM_WALK_CAP} max_level=3 top-k=full critical=empty\n\
                 opening doom spec: node_budget={OPENING_DOOM_NODE_BUDGET} \
                 walk_cap={OPENING_DOOM_WALK_CAP} max_level=3 top-k={OPENING_DESCEND_TOP}\n\
                 corpus: the ten gated roots of the Slice F epoch, plus the h0-t1 opening root\n",
                SlicePolicy::id(&field),
            ));
            flush(&out);

            // -------- Part 1: the coordinate census.
            out.push_str("\n#### PART 1 — the census, coordinate by coordinate ####\n");
            let spec = GodGapSpec {
                exact_fiber_cap: REPORT_EXACT_FIBER_CAP,
                profile_fiber_cap: REPORT_PROFILE_FIBER_CAP,
                doom: full_doom.clone(),
            };
            let mut entries: Vec<(usize, String, GodGapCoordinate)> = Vec::new();
            for (hand_id, trick_no) in CORPUS {
                let (root, position) = root_at(&r, hand_id, trick_no);
                let label = format!("h{hand_id}-t{trick_no}");
                let walk = GodGapWalk {
                    oracle: &oracle,
                    root: &root,
                    position: &position,
                    field: &field,
                    spec: &spec,
                };
                let z = oracle_mass(&oracle, &root, &position, &field);
                out.push_str(&format!(
                    "\n{label} (fiber {z}, bid {}, viewer {}):\n",
                    position.bid,
                    if root.kernel().viewer().team() == position.declaring_team {
                        "declaring (pmake-v1)"
                    } else {
                        "setting (pmake-setting-v1)"
                    }
                ));
                for action in walt::solver::godgap::legal_actions(&root, &position) {
                    let mut progress = |done: u64, total: u64, doomed: u128, nodes: u64| {
                        eprintln!(
                            "  {label} {action}: enum {done}/{total} outer, \
                             doomed {doomed}, nodes {nodes}"
                        );
                    };
                    let t0 = Instant::now();
                    let c = walk.god_gap(action, &mut progress);
                    print_coordinate(&mut out, &c, t0.elapsed().as_micros());
                    entries.push((trick_no, label.clone(), c));
                    flush(&out);
                }
            }

            // -------- Part 2: the opening root, on the census's own
            // instrument (the exact side refuses; the doom side runs a
            // declared cheap priority census).
            out.push_str("\n#### PART 2 — the h0-t1 opening root ####\n");
            flush(&out);
            {
                let (root, position) = root_at(&r, 0, 1);
                let opening_spec = GodGapSpec {
                    exact_fiber_cap: 0,
                    profile_fiber_cap: 0,
                    doom: DoomSpec {
                        node_budget: OPENING_DOOM_NODE_BUDGET,
                        walk_cap: OPENING_DOOM_WALK_CAP,
                        max_level: 3,
                        critical: DominoSet::EMPTY,
                        descend_top: Some(OPENING_DESCEND_TOP),
                    },
                };
                let walk = GodGapWalk {
                    oracle: &oracle,
                    root: &root,
                    position: &position,
                    field: &field,
                    spec: &opening_spec,
                };
                let z = oracle_mass(&oracle, &root, &position, &field);
                out.push_str(&format!(
                    "\nh0-t1 (fiber {z}, bid {}, viewer declaring):\n",
                    position.bid
                ));
                for action in walt::solver::godgap::legal_actions(&root, &position) {
                    let mut progress = |_: u64, _: u64, _: u128, _: u64| {};
                    let t0 = Instant::now();
                    let c = walk.god_gap(action, &mut progress);
                    print_coordinate(&mut out, &c, t0.elapsed().as_micros());
                    entries.push((1, "h0-t1".to_string(), c));
                    flush(&out);
                }
            }

            // -------- Part 3: the §38 fusion-horizon table.
            out.push_str(
                "\n#### PART 3 — the fusion-horizon table (§38, EMPIRICAL) ####\n\
                 §48 asks for stratification by trick, grade, contract, trump structure, \
                 count state and field level. On THIS corpus the last four are constant — \
                 one declared field (above), contract 30 at every root, the receipt's own \
                 trump per root, and field level 0 — so trick depth is the only \
                 stratification coordinate that varies, with the fiber mass (the grade) \
                 printed per coordinate in Part 1. A corpus that varies the others is \
                 future work, and this table makes no claim beyond the roots it names.\n",
            );
            let strata = fusion_horizon(&entries);
            out.push_str(
                "\n trick | tested | God-tight (vacuous) | pos gap | GodUpper | Unknown \
                 | max Φ\n\
                 -------+--------+---------------------+---------+----------+---------\
                 +-------\n",
            );
            for s in &strata {
                out.push_str(&format!(
                    "   t{}  |   {:>3}  |     {:>3} ({:>3})        |   {:>3}   |   {:>3}    \
                     |   {:>3}   | {}\n",
                    s.trick,
                    s.tested,
                    s.god_tight,
                    s.god_tight_vacuous,
                    s.positive_gap,
                    s.god_upper_only,
                    s.unknown,
                    match &s.max_gap {
                        Some(g) => exact(g),
                        None => "-".to_string(),
                    }
                ));
            }
            for s in &strata {
                if !s.exceptions.is_empty() {
                    out.push_str(&format!(
                        "  t{} exceptions ({}): {}\n",
                        s.trick,
                        s.exceptions.len(),
                        s.exceptions.join(", ")
                    ));
                }
            }
            match earliest_fusion_free_trick(&strata) {
                Some(t) => {
                    out.push_str(&format!(
                        "\n  earliest fusion-free depth on THIS corpus: trick {t} \
                         (every tested coordinate at t{t} and later is God-tight)\n"
                    ));
                    let substantive = strata
                        .iter()
                        .filter(|s| s.trick >= t && s.substantively_fusion_free())
                        .count();
                    out.push_str(&format!(
                        "  of those strata, {substantive} are SUBSTANTIVELY fusion-free — \
                         they hold God-tight coordinates with something left to save; a \
                         stratum that is fusion-free only because everything in it is \
                         doomed is no evidence about the information price\n"
                    ));
                }
                None => out.push_str("\n  no fusion-free depth on this corpus\n"),
            }
            let tight: Vec<&GodGapCoordinate> = entries
                .iter()
                .map(|(_, _, c)| c)
                .filter(|c| c.god_tight().is_some())
                .collect();
            let persisted = tight
                .iter()
                .filter(|c| {
                    c.god_tight()
                        .expect("a God-tight coordinate")
                        .profile
                        .is_some()
                })
                .count();
            out.push_str(&format!(
                "  God-tight policies extracted: {} of {} coordinates ({persisted} with a \
                 persisted score profile)\n",
                tight.len(),
                entries.len()
            ));
            flush(&out);

            // -------- Part 4: the two-regime summary, in words.
            out.push_str("\n#### PART 4 — the two regimes, in plain language ####\n\n");
            out.push_str(&two_regime_summary(&entries, &strata));
            out.push_str(
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts. \
                 The fusion horizon is a MEASUREMENT on the declared corpus above, never a \
                 theorem (SC-A4).\n",
            );
            flush(&out);
            println!("{out}");
        }
        _ => {
            eprintln!(
                "usage: godgapreport scout <hand> <trick> [exact-cap] [profile-cap] \
                 | godgapreport report <out.txt>"
            );
            std::process::exit(2);
        }
    }
}

fn oracle_mass(
    oracle: &walt::solver::factor_belief::SupportOracle,
    root: &CanonicalRoot,
    position: &RootPosition,
    field: &FieldModel,
) -> u128 {
    use walt::solver::factor_belief::{ExactCoverOracle, FactorBelief};
    oracle.mass(&FactorBelief::uniform_root(root, position, field))
}

/// The findings paragraph the report closes with — assembled from the
/// census's own numbers, never from a quoted result.
fn two_regime_summary(
    entries: &[(usize, String, GodGapCoordinate)],
    strata: &[walt::solver::godgap::FusionStratum],
) -> String {
    let mut s = String::new();
    let late: Vec<&(usize, String, GodGapCoordinate)> =
        entries.iter().filter(|(t, _, _)| *t >= 5).collect();
    let late_tight = late
        .iter()
        .filter(|(_, _, c)| c.god_tight().is_some())
        .count();
    let late_vacuous = late
        .iter()
        .filter(|(_, _, c)| c.god_tight().is_some_and(|t| t.nothing_saveable()))
        .count();
    let t4: Vec<&(usize, String, GodGapCoordinate)> =
        entries.iter().filter(|(t, _, _)| *t == 4).collect();
    let t4_tight = t4
        .iter()
        .filter(|(_, _, c)| c.god_tight().is_some())
        .count();
    let t4_vacuous = t4
        .iter()
        .filter(|(_, _, c)| c.god_tight().is_some_and(|t| t.nothing_saveable()))
        .count();
    let opening: Vec<&(usize, String, GodGapCoordinate)> =
        entries.iter().filter(|(t, _, _)| *t == 1).collect();
    s.push_str(&format!(
        "LATE REGIME (t5/t6, {} coordinates): {late_tight} God-tight, {} of them \
         substantively (the other {late_vacuous} are whole-fiber doom). Where the census \
         can see the whole fiber, the deterministic doom upper and the exact \
         information-consistent optimum are the SAME NUMBER — blindness costs nothing, and \
         the executable policy that attains it is extracted and re-priced.\n\n",
        late.len(),
        late_tight - late_vacuous
    ));
    s.push_str(&format!(
        "MIDDLE REGIME (t4, {} coordinates): {t4_tight} God-tight, of which {t4_vacuous} \
         are whole-fiber doom — every policy is God-tight where nothing is saveable, so \
         those carry no evidence either way. This is where the information-consistency \
         price first appears on this corpus: the God upper stays high while the exact Q \
         sits below it, so the failure that remains is NOT physical doom, and no amount \
         of further counterexample counting will touch it.\n\n",
        t4.len()
    ));
    s.push_str(&format!(
        "OPENING (h0-t1, {} coordinates): the exact side is unaffordable and the doom side \
         certifies nothing, so every coordinate is UnknownGodGap — the SC-A4 floor. A zero \
         doom census is not a gap measurement; it leaves the vacuous upper 1 standing and \
         says nothing about d_info or d_policy.\n\n",
        opening.len()
    ));
    let horizon = earliest_fusion_free_trick(strata);
    s.push_str(&match horizon {
        Some(t) => format!(
            "READ: on this corpus the two recursions trade dominance between trick {} and \
             trick {t}. Looking back from the endgame, the salvation complex is a single \
             face — one policy saves everything saveable. Looking forward from the opening, \
             neither instrument reaches. The census measures where the changeover sits; it \
             does not explain it.\n",
            t - 1
        ),
        None => "READ: no depth on this corpus is fusion-free.\n".to_string(),
    });
    s
}
