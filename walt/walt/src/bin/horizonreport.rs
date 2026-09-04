//! EXPLORATORY IN-SOLVE HORIZON CENSUS INSTRUMENT (`solver::horizon`,
//! slice U0b) — sits below every evidentiary tier and is cited by
//! nothing above it. Instrument output only: per (root, contract, cut
//! depth), every frontier belief node's mass, doomed mass, exact `Q`,
//! God upper and information price; the tally of God-tight versus
//! positive-gap nodes; the mass-weighted frontier price; and the root
//! consequence — the exact value versus what a §39 fusion cut at that
//! depth would compute, and whether the cut would change the root play.
//! Never a play-strength claim, and never a theorem: the horizon is an
//! empirical object on a declared corpus (SC-A4).
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle`; the frozen `verify_player` receipt; the four trick-4
//! gated roots of the Slice F epoch cut at 4 and 8 plays (the trick-5
//! and trick-6 frontiers), each under the receipt contract AND a
//! declared contract sweep; the two trick-5 roots cut at 4; and the
//! smallest trick-3 receipt root cut at 4 (its trick-4 frontier) under
//! a declared node fiber cap.
//!
//! Modes:
//!   `horizonreport scout <hand> <trick> <cut> [contract] [node-cap]`
//!   `horizonreport report <out.txt>`
//!
//! No floats anywhere; wall time is integer microseconds; values are
//! exact rationals, printed alongside integer permille.

use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, RootPosition};
use walt::solver::factor_belief::SupportOracle;
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::horizon::{
    horizon_census, with_contract, HorizonCensus, HorizonSpec, NodeVerdict,
};
use walt::solver::policy::{DecisionMode, TieRule};

/// The four trick-4 gated roots, the two trick-5 roots, and the smallest
/// trick-3 receipt root. `(hand, trick)`.
const T4_CORPUS: [(usize, usize); 4] = [(3, 4), (4, 4), (8, 4), (12, 4)];
const T5_CORPUS: [(usize, usize); 2] = [(8, 5), (3, 5)];
const T3_ROOT: (usize, usize) = (8, 3);

/// The declared contract sweep beside each root's receipt contract.
const CONTRACT_SWEEP: [u32; 5] = [30, 33, 36, 39, 42];

/// Frontier node fiber cap for the enumerable corpus (admits every
/// trick-5/trick-6 frontier node under any trick-4 root: the largest
/// trick-4 receipt fiber is 34,650 and a frontier node holds a slice of
/// it).
const REPORT_NODE_CAP: u128 = 40_000;
/// The trick-3 root's trick-4 frontier: nodes above this are refused and
/// counted, never priced by a partial walk.
const T3_NODE_CAP: u128 = 12_000;

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

fn exact(v: &BigRational) -> String {
    format!("{v} ({}‰)", permille(v))
}

fn opt_exact(v: Option<BigRational>) -> String {
    match v {
        Some(x) => exact(&x),
        None => "REFUSED".to_string(),
    }
}

fn opt_tile(t: Option<walt::rules::Domino>) -> String {
    match t {
        Some(d) => format!("{d}"),
        None => "-".to_string(),
    }
}

fn ratio(m: u128, z: u128) -> BigRational {
    BigRational::new(BigInt::from(m), BigInt::from(z))
}

/// One census, printed: the summary block, the root readings, then the
/// positive-gap and refused nodes in full (God-tight nodes are counted,
/// not listed — they are the bulk).
fn print_census(out: &mut String, label: &str, c: &HorizonCensus, wall_us: u128) {
    let (tight, vacuous, positive) = c.tally();
    let (dec_n, dec_mass) = c.decided_before_cut();
    let _ = writeln!(
        out,
        "== {label} contract {} cut {} | Z={} | frontier {} priced ({} God-tight substantive, \
         {} vacuous, {} positive gap), {} refused; {} decided before the cut carrying {} mass \
         | reads {} | wall {}us",
        c.contract,
        c.spec.cut_plays,
        c.root_fiber,
        c.frontier_nodes(),
        tight,
        vacuous,
        positive,
        c.refused(),
        dec_n,
        dec_mass,
        c.field_reads,
        wall_us
    );
    let _ = writeln!(
        out,
        "   frontier price: max Φ = {} | mass-weighted Φ = {}",
        opt_exact(c.max_phi()),
        opt_exact(c.weighted_phi())
    );
    let _ = writeln!(
        out,
        "   ROOT: exact {} (check {}) argmax {} | under a cut at this depth {} argmax {} | \
         over-pricing {} | cut flips root play: {}",
        exact(&ratio(c.root_exact_mass, c.root_fiber)),
        c.root_check_mass,
        opt_tile(c.exact_argmax),
        opt_exact(c.root_cut_mass.map(|m| ratio(m, c.root_fiber))),
        opt_tile(c.cut_argmax),
        opt_exact(c.root_over_pricing()),
        match c.cut_flips_root() {
            Some(true) => "YES",
            Some(false) => "no",
            None => "REFUSED",
        }
    );
    for a in &c.actions {
        let _ = writeln!(
            out,
            "     action {}: exact {} | cut {}",
            a.action,
            exact(&ratio(a.exact_mass, c.root_fiber)),
            opt_exact(a.cut_mass.map(|m| ratio(m, c.root_fiber)))
        );
    }
    for n in &c.nodes {
        match &n.verdict {
            NodeVerdict::Priced(p) if !p.god_tight() => {
                let names: Vec<String> = n.history.iter().map(|d| format!("{d}")).collect();
                let _ = writeln!(
                    out,
                    "     GAP [{}] to-move s{}{} Z={} doomed={} Q={} U^God={} Φ={}",
                    names.join(" "),
                    n.seat_to_move,
                    if n.viewer_to_move { " (viewer)" } else { "" },
                    n.mass,
                    p.doomed,
                    p.q_mass,
                    exact(&p.upper),
                    exact(&p.phi)
                );
            }
            NodeVerdict::Refused { fiber, cap } => {
                let names: Vec<String> = n.history.iter().map(|d| format!("{d}")).collect();
                let _ = writeln!(
                    out,
                    "     REFUSED [{}] Z={fiber} above cap {cap}",
                    names.join(" ")
                );
            }
            _ => {}
        }
    }
}

fn run(
    r: &Receipt,
    hand_id: usize,
    trick_no: usize,
    contract: Option<u32>,
    cut: usize,
    cap: u128,
) -> (String, HorizonCensus, u128) {
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = match contract {
        Some(c) => with_contract(&position, c),
        None => position,
    };
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let spec = HorizonSpec {
        cut_plays: cut,
        node_fiber_cap: cap,
    };
    let t0 = Instant::now();
    let census = horizon_census(&oracle, &root, &position, &field, &spec);
    let wall = t0.elapsed().as_micros();
    (format!("h{hand_id}-t{trick_no}"), census, wall)
}

/// The cross-census table: one row per census.
struct RowSummary {
    label: String,
    contract: u32,
    cut: usize,
    fiber: u128,
    frontier: usize,
    tight: usize,
    vacuous: usize,
    positive: usize,
    refused: usize,
    max_phi: Option<BigRational>,
    weighted_phi: Option<BigRational>,
    over_pricing: Option<BigRational>,
    flips: Option<bool>,
    wall_us: u128,
}

fn summarize(label: &str, c: &HorizonCensus, wall_us: u128) -> RowSummary {
    let (tight, vacuous, positive) = c.tally();
    RowSummary {
        label: label.to_string(),
        contract: c.contract,
        cut: c.spec.cut_plays,
        fiber: c.root_fiber,
        frontier: c.frontier_nodes(),
        tight,
        vacuous,
        positive,
        refused: c.refused(),
        max_phi: c.max_phi(),
        weighted_phi: c.weighted_phi(),
        over_pricing: c.root_over_pricing(),
        flips: c.cut_flips_root(),
        wall_us,
    }
}

fn print_table(out: &mut String, rows: &[RowSummary]) {
    let _ = writeln!(
        out,
        "\n#### THE HORIZON TABLE — one row per (root, contract, cut) ####\n"
    );
    let _ = writeln!(
        out,
        " root    | bid | cut |     Z     | frontier | tight | vac | gap | ref | max Φ ‰ | wtd Φ ‰ | root over-pricing ‰ | flips | wall"
    );
    let _ = writeln!(
        out,
        "---------+-----+-----+-----------+----------+-------+-----+-----+-----+---------+---------+---------------------+-------+------"
    );
    for r in rows {
        let _ = writeln!(
            out,
            " {:<7} | {:>3} | {:>3} | {:>9} | {:>8} | {:>5} | {:>3} | {:>3} | {:>3} | {:>7} | {:>7} | {:>19} | {:<5} | {}us",
            r.label,
            r.contract,
            r.cut,
            r.fiber,
            r.frontier,
            r.tight,
            r.vacuous,
            r.positive,
            r.refused,
            r.max_phi.as_ref().map_or("-".to_string(), |v| permille(v).to_string()),
            r.weighted_phi
                .as_ref()
                .map_or("-".to_string(), |v| permille(v).to_string()),
            r.over_pricing
                .as_ref()
                .map_or("REFUSED".to_string(), |v| format!("{v} ({}‰)", permille(v))),
            match r.flips {
                Some(true) => "YES",
                Some(false) => "no",
                None => "-",
            },
            r.wall_us
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let r = parse_file(&locate_verify_player().expect("the receipt above the workspace"))
        .expect("the receipt parses");
    match mode {
        "scout" => {
            let hand_id: usize = args[2].parse().expect("a hand id");
            let trick_no: usize = args[3].parse().expect("a trick number");
            let cut: usize = args[4].parse().expect("a cut depth");
            let contract: Option<u32> = args.get(5).map(|s| s.parse().expect("a contract"));
            let cap: u128 = args
                .get(6)
                .map_or(REPORT_NODE_CAP, |s| s.parse().expect("a node cap"));
            let (label, census, wall) = run(&r, hand_id, trick_no, contract, cut, cap);
            let mut out = String::new();
            print_census(&mut out, &label, &census, wall);
            print!("{out}");
        }
        "report" => {
            let path = args.get(2).expect("an output path").clone();
            let mut out = String::new();
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            let _ = writeln!(
                out,
                "IN-SOLVE HORIZON CENSUS (slice U0b) — EXPLORATORY\n\
                 \n\
                 The §38/§40 God-gap census run at every belief node the exact recursion \
                 reaches at a declared depth below a root, and the root consequence of a §39 \
                 fusion cut at that depth. Never a theorem (SC-A4); an empirical object on the \
                 declared corpus.\n\
                 \n\
                 declared field: level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle\n\
                 corpus: the four trick-4 gated roots cut at 4 and at 8 plays under the receipt \
                 contract and the sweep {CONTRACT_SWEEP:?}; the two trick-5 roots cut at 4; the \
                 smallest trick-3 receipt root cut at 4 under node cap {T3_NODE_CAP}\n\
                 node fiber cap on the enumerable corpus: {REPORT_NODE_CAP}\n\
                 tie rule for both argmaxes: lowest tile index\n\
                 wall is the only approximate number here\n"
            );
            flush(&out);
            let mut rows: Vec<RowSummary> = Vec::new();
            for (hand_id, trick_no) in T4_CORPUS {
                let receipt_bid = r.hands[hand_id].bid_points;
                let mut contracts: Vec<u32> = vec![receipt_bid];
                for c in CONTRACT_SWEEP {
                    if !contracts.contains(&c) {
                        contracts.push(c);
                    }
                }
                for cut in [4usize, 8usize] {
                    for c in &contracts {
                        eprintln!("  h{hand_id}-t{trick_no} contract {c} cut {cut} ...");
                        let (label, census, wall) =
                            run(&r, hand_id, trick_no, Some(*c), cut, REPORT_NODE_CAP);
                        print_census(&mut out, &label, &census, wall);
                        rows.push(summarize(&label, &census, wall));
                        flush(&out);
                    }
                }
            }
            for (hand_id, trick_no) in T5_CORPUS {
                eprintln!("  h{hand_id}-t{trick_no} receipt contract cut 4 ...");
                let (label, census, wall) = run(&r, hand_id, trick_no, None, 4, REPORT_NODE_CAP);
                print_census(&mut out, &label, &census, wall);
                rows.push(summarize(&label, &census, wall));
                flush(&out);
            }
            {
                let (hand_id, trick_no) = T3_ROOT;
                eprintln!(
                    "  h{hand_id}-t{trick_no} receipt contract cut 4 (node cap {T3_NODE_CAP}) ..."
                );
                let (label, census, wall) = run(&r, hand_id, trick_no, None, 4, T3_NODE_CAP);
                print_census(&mut out, &label, &census, wall);
                rows.push(summarize(&label, &census, wall));
                flush(&out);
            }
            print_table(&mut out, &rows);
            let _ = writeln!(
                out,
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts. \
                 The horizon is a measurement on the declared corpus, never a theorem."
            );
            flush(&out);
            println!("{out}");
        }
        _ => {
            eprintln!(
                "usage: horizonreport scout <hand> <trick> <cut> [contract] [node-cap] | \
                 horizonreport report <out.txt>"
            );
            std::process::exit(2);
        }
    }
}
