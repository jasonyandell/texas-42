//! The situation census run: the first (d, Theta) candidate under §12.6A
//! equivariant controlled lumpability, measured on the 13 trick-six receipt
//! kernels pooled into one ambient frame.
//!
//! Design: `walt/CENSUS.md`, as amended by the binding fork rulings in
//! `walt/CENSUS-RULINGS.md` (F1 scope, F2 A1-A4 invariants, F3 transports by
//! canonicalization, F4 probability model, F5 primitive steps with the bank as
//! emission, F6 baseline and reporting, F7 failure protocol). The physics
//! lives in `walt_skeleton::equivariant`; this runner enumerates, counts, and
//! writes the results file.
//!
//! Scope: pip-trump only. Exploratory tier. Deterministic: nothing sampled,
//! nothing capped, every class with two or more members checked.
//!
//! Writes `results/census_2026-08-10.txt`.

use std::fmt::Write as _;
use std::path::Path;

use walt_core::receipt::{locate_verify_player, parse_file, Receipt};
use walt_skeleton::equivariant::{
    build_carrier, check_ecl, trick_six_kernels, CandidateSpec, Census, EclVerdict,
};

/// The pinned trick-six fiber sizes of the receipt corpus, asserted so the
/// census cannot silently drift off its declared domain.
const FIBERS: [u128; 13] = [90, 90, 36, 36, 90, 27, 90, 90, 7, 30, 19, 36, 6];

fn receipt() -> Receipt {
    let path = locate_verify_player().expect("rob/receipts/verify_player.txt above the workspace");
    parse_file(&path).expect("the receipt parses")
}

/// An exact fixed-point rendering of `a / b` in thousandths; no division of
/// anything but integers happens anywhere in this run.
fn ratio(a: usize, b: usize) -> String {
    if b == 0 {
        return "n/a".to_string();
    }
    let scaled = (a * 1000) / b;
    format!("{}.{:03}", scaled / 1000, scaled % 1000)
}

fn main() {
    let r = receipt();
    let kernels = trick_six_kernels(&r);
    assert_eq!(kernels.len(), 13, "the trick-six receipt corpus");
    for (i, (hand, kernel)) in kernels.iter().enumerate() {
        assert_eq!(*hand, i);
        assert_eq!(kernel.count(), FIBERS[i], "the pinned h{i} fiber size");
    }
    let roots_expected: u128 = FIBERS.iter().sum();
    if std::env::args().nth(1).as_deref() == Some("r2") {
        run_r2(&kernels, roots_expected);
        return;
    }

    let t0 = std::time::Instant::now();
    let carrier = build_carrier(&kernels);
    eprintln!(
        "carrier: {} situations ({} roots) in {:?}",
        carrier.len(),
        carrier.roots(),
        t0.elapsed()
    );
    assert_eq!(
        carrier.roots() as u128,
        roots_expected,
        "one root per world"
    );

    let t1 = std::time::Instant::now();
    let census = Census::build(carrier, CandidateSpec::FINEST);
    eprintln!(
        "classes: {} equivariant / {} identity in {:?}",
        census.class_members.len(),
        census.identity_members.len(),
        t1.elapsed()
    );

    let t2 = std::time::Instant::now();
    let verdict = check_ecl(&census);
    eprintln!("ecl: {} in {:?}", verdict.verdict(), t2.elapsed());

    let mut out = String::new();
    write_header(&mut out);
    write_carrier(&mut out, &census);
    write_classes(&mut out, &census, &verdict);
    write_strata(&mut out, &census, &verdict);
    write_merges(&mut out, &census);
    write_baseline(&mut out, &census);
    write_verdict(&mut out, &verdict);

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(root.join("results/census_2026-08-10.txt"), out).expect("write results");
    eprintln!("wrote results/census_2026-08-10.txt");
}

/// Round 2: the declared candidate coarsenings, each measured end to end on
/// the same carrier, with the finest candidate reproduced as the reference
/// row. A coarsening is a NEW DECLARED CANDIDATE (v0.4 §12.9), never a
/// mutation of r1's descriptor and never an in-run fix to a failure (F7).
fn run_r2(kernels: &[(usize, walt_kernel::Kernel)], roots_expected: u128) {
    let mut out = String::new();
    out.push_str(
        "walt situation census r2 — declared candidate coarsenings under §12.6A — exploratory tier\n",
    );
    write_provenance(&mut out, "census_run r2");
    out.push_str(
        "candidates declared (each measured end to end; a coarsening is a new declared candidate \
         under v0.4 §12.9, never a mutation of r1's descriptor and never an in-run repair of a \
         failure — F7 NO-RESCUE). c1 reproduces r1's finest candidate as the reference row; its \
         counts are asserted in-run against r1's published numbers:\n",
    );
    for spec in CandidateSpec::ALL {
        let _ = writeln!(out, "  {}", spec.render());
    }
    out.push('\n');

    let mut summary: Vec<(CandidateSpec, EclVerdict, [usize; 6])> = Vec::new();
    for spec in CandidateSpec::ALL {
        let carrier = build_carrier(kernels);
        assert_eq!(
            carrier.roots() as u128,
            roots_expected,
            "one root per world"
        );
        let census = Census::build(carrier, spec);
        let verdict = check_ecl(&census);
        eprintln!(
            "{}: {} classes, {} root classes, ECL {}",
            spec.name,
            census.class_members.len(),
            census.root_classes(),
            verdict.verdict()
        );
        if spec == CandidateSpec::FINEST {
            assert_eq!(census.carrier.len(), 15253, "r1's carrier, reproduced");
            assert_eq!(census.class_members.len(), 11949, "r1's class count");
            assert_eq!(census.root_classes(), 647, "r1's root class count");
            assert!(verdict.passed(), "r1's ECL verdict");
        }
        let cross = census.cross_kernel_classes();
        let root_merges = (0..census.class_members.len())
            .filter(|c| {
                census.class_members[*c].len() > 1
                    && census.class_members[*c]
                        .iter()
                        .any(|i| census.carrier.is_root[*i])
            })
            .count();
        let cross_roots = cross
            .iter()
            .filter(|c| {
                census.class_members[**c]
                    .iter()
                    .any(|i| census.carrier.is_root[*i])
            })
            .count();
        let t7_lead = ply_rows(&census)
            .into_iter()
            .find(|row| row.0 == 4)
            .map_or(0, |row| row.2);

        let _ = writeln!(out, "candidate {}", spec.render());
        let _ = writeln!(
            out,
            "  pooled full carrier: {} classes over {} situations   [ECL {}]",
            census.class_members.len(),
            census.carrier.len(),
            verdict.verdict()
        );
        let _ = writeln!(
            out,
            "  pooled roots only:   {} classes over {} roots   [ECL {}]",
            census.root_classes(),
            census.carrier.roots(),
            verdict.verdict()
        );
        let _ = writeln!(
            out,
            "  singleton classes (ECL vacuous there): {} of {}; largest class {} situations",
            census.singleton_classes(),
            census.class_members.len(),
            census.class_members.iter().map(Vec::len).max().unwrap_or(0)
        );
        let _ = writeln!(
            out,
            "  cross-kernel merges: {} classes ({} of them containing a root situation)",
            cross.len(),
            cross_roots
        );
        let _ = writeln!(
            out,
            "  root merges (classes holding two or more situations, at least one a root): {root_merges}"
        );
        let _ = writeln!(
            out,
            "  identity control: {} classes, dividend {}",
            census.identity_members.len(),
            ratio(census.identity_members.len(), census.class_members.len())
        );
        let _ = writeln!(
            out,
            "  ECL {}: {} classes checked, {} pairs under condition 1 and {} under condition 2, {} counterexamples",
            verdict.verdict(),
            verdict.classes_checked,
            verdict.cond1_checks,
            verdict.cond2_checks,
            verdict.failures.len()
        );
        write_strata(&mut out, &census, &verdict);
        if !verdict.passed() {
            out.push_str(
                "  counterexamples (recorded verbatim; the dropped distinction was load-bearing \
                 after all — a finding, not a bug list):\n",
            );
            for f in &verdict.failures {
                let _ = writeln!(out, "    class#{} key {}", f.class, f.class_key);
                let _ = writeln!(out, "      condition: {}", f.condition);
                let _ = writeln!(out, "      divergence: {}", f.detail);
                let _ = writeln!(out, "      representative: {}", f.representative);
                let _ = writeln!(out, "      member:         {}", f.member);
            }
            out.push('\n');
        }
        summary.push((
            spec,
            verdict,
            [
                census.class_members.len(),
                census.root_classes(),
                census.singleton_classes(),
                cross.len(),
                root_merges,
                t7_lead,
            ],
        ));
    }

    out.push_str(
        "summary (every count exploratory tier and quotable only with its ECL verdict, F6)\n\
         candidate                              full   roots  singleton  cross-kernel  root-merges  t7-lead  ECL\n",
    );
    for (spec, verdict, n) in &summary {
        let _ = writeln!(
            out,
            "  {:<36} {:>6} {:>6} {:>10} {:>13} {:>12} {:>8}  {}",
            spec.name,
            n[0],
            n[1],
            n[2],
            n[3],
            n[4],
            n[5],
            verdict.verdict()
        );
    }
    out.push_str(
        "\n  full = classes over the whole 15253-situation carrier; roots = classes over the 647 \
         kernel roots; t7-lead = classes at ply 4, the trick-7 lead stratum (every seat holds one \
         tile, play forced) — the target alphabet for a backward walk.\n",
    );

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(root.join("results/census_2026-08-10_r2.txt"), out).expect("write results");
    eprintln!("wrote results/census_2026-08-10_r2.txt");
}

/// The shared provenance block; `regenerate` names the run that wrote the
/// file.
fn write_provenance(out: &mut String, regenerate: &str) {
    let _ = write!(
        out,
        "scope: pip-trump only — all 13 receipt trick-six kernels are pip-trump (v0.4 §14.7); \
         doubles-trump and no-trump are excluded, since pooling them would implicitly claim \
         cross-declaration-type transfer, which v0.4 §17.5 does not claim (F1 amendment)\n\
         corpus: rob/receipts/verify_player.txt, hands 0-12, trick 6, viewer = that trick's \
         leader = the focal seat; fibers 90 90 36 36 90 27 90 90 7 30 19 36 6 (asserted in-run)\n\
         model: §12.6A per walt/CENSUS.md with the walt-math rulings of walt/CENSUS-RULINGS.md — \
         carrier = world-level latent situations closed under primitive steps (F1, F5), bank is \
         emission not state (F5 amendment), transports by canonicalization (F3), fixed \
         uniform-legal field with exact rational mass 1/|L| per hidden play and Dirac focal \
         actions (F4)\n\
         determinism: no sampling, no caps, no seeds; the carrier is candidate-independent and \
         every class with two or more members is checked under every candidate\n\
         provenance: SINGLE-IMPLEMENTATION — one Rust implementation (walt-skeleton's equivariant \
         module), exploratory tier, below every project evidentiary tier\n\
         regenerate: cargo run --release -p walt-factory --example {regenerate}\n\n"
    );
}

fn write_header(out: &mut String) {
    out.push_str(
        "walt situation census — the first (d, Theta) candidate under §12.6A — exploratory tier\n\
         scope: pip-trump only — all 13 receipt trick-six kernels are pip-trump (v0.4 §14.7); \
         doubles-trump and no-trump are excluded, since pooling them would implicitly claim \
         cross-declaration-type transfer, which v0.4 §17.5 does not claim (F1 amendment). \
         Cross-kernel merges below are within-pip-trump by corpus construction.\n\
         corpus: rob/receipts/verify_player.txt, hands 0-12, trick 6, viewer = that trick's \
         leader = the focal seat; fibers 90 90 36 36 90 27 90 90 7 30 19 36 6 (asserted in-run)\n\
         model: §12.6A instantiated per walt/CENSUS.md with the walt-math rulings of \
         walt/CENSUS-RULINGS.md — carrier = world-level latent situations closed under primitive \
         steps (F1, F5), bank is emission not state (F5 amendment), descriptor = the finest \
         structural relabeling quotient (F2 A1-A4), transports by canonicalization (F3), fixed \
         uniform-legal field with exact rational mass 1/|L| per hidden play and Dirac focal \
         actions (F4)\n\
         determinism: no sampling, no caps, no seeds — the domain is exhaustively checkable and \
         every class with two or more members is checked; canonicalization is a brute-force \
         minimum over the lawful labelings\n\
         provenance: SINGLE-IMPLEMENTATION — every number below is computed by one Rust \
         implementation (walt-skeleton's equivariant module) and is exploratory tier, below every \
         project evidentiary tier; a class count is quotable only alongside its ECL verdict (F6)\n\
         regenerate: cargo run --release -p walt-factory --example census_run\n\n",
    );
}

fn write_carrier(out: &mut String, census: &Census) {
    let c = &census.carrier;
    out.push_str("carrier (F1/F5: every situation reachable from a kernel root under primitive steps, pooled and deduplicated)\n");
    let _ = writeln!(
        out,
        "  pooled: {} situations = {} roots + {} trick-boundary (non-root) + {} mid-trick",
        c.len(),
        c.roots(),
        c.boundaries() - c.roots(),
        c.mid_trick()
    );
    out.push_str("  per receipt hand (situations reachable from that kernel; a situation reachable from two hands counts in both):\n");
    for (slot, hand) in c.hands.iter().enumerate() {
        let roots = (0..c.len())
            .filter(|i| c.is_root[*i] && (c.provenance[*i] & (1u32 << slot)) != 0)
            .count();
        let _ = writeln!(
            out,
            "    h{hand}: {} situations, {roots} roots",
            census.states_of_slot(slot)
        );
    }
    out.push('\n');
}

fn write_classes(out: &mut String, census: &Census, verdict: &EclVerdict) {
    out.push_str("equivariant classes (the descriptor d of F2; every count below is paired with the ECL verdict at the foot of this file)\n");
    let _ = writeln!(
        out,
        "  pooled full carrier: {} classes over {} situations   [ECL {}]",
        census.class_members.len(),
        census.carrier.len(),
        verdict.verdict()
    );
    let _ = writeln!(
        out,
        "  pooled roots only:   {} classes over {} roots   [ECL {}]",
        census.root_classes(),
        census.carrier.roots(),
        verdict.verdict()
    );
    let _ = writeln!(
        out,
        "  singleton classes (ECL vacuous there): {} of {} — {} classes carry two or more \
         situations and are the checked coverage",
        census.singleton_classes(),
        census.class_members.len(),
        census.class_members.len() - census.singleton_classes()
    );
    let largest = census.class_members.iter().map(Vec::len).max().unwrap_or(0);
    let _ = writeln!(out, "  largest class: {largest} situations");
    out.push_str("  per receipt hand (classes among that kernel's situations):\n");
    for (slot, hand) in census.carrier.hands.iter().enumerate() {
        let _ = writeln!(
            out,
            "    h{hand}: {} classes full-carrier, {} classes root-only",
            census.classes_of_slot(slot, false),
            census.classes_of_slot(slot, true)
        );
    }
    out.push_str(
        "  by structure size (matched tiles = live tiles ∪ unresolved-trick tiles, A1): \
         situations / classes / non-singleton classes\n",
    );
    for n in (1..=8).rev() {
        let rows: Vec<usize> = (0..census.carrier.len())
            .filter(|i| census.carrier.states[*i].matched(census.spec).len() == n)
            .collect();
        if rows.is_empty() {
            continue;
        }
        let classes: std::collections::BTreeSet<usize> =
            rows.iter().map(|i| census.class_of[*i]).collect();
        let merged = classes
            .iter()
            .filter(|c| census.class_members[**c].len() > 1)
            .count();
        let _ = writeln!(
            out,
            "    {n} tiles: {} situations, {} classes, {merged} of them non-singleton",
            rows.len(),
            classes.len()
        );
    }
    out.push('\n');
}

/// The ply of a situation: how many primitive steps separate it from a kernel
/// root, derived as `8 - |live|` (two tricks of four plays). Ply is reporting
/// metadata only — it is bookkeeping outside the situation and outside the
/// descriptor, exactly as the banked increment is (F5 amendment). Nothing in
/// the carrier, the canonical form or the (ECL) check reads it.
fn ply(sit: &walt_skeleton::equivariant::Situation) -> usize {
    8 - sit.live().len()
}

fn ply_label(ply: usize) -> &'static str {
    match ply {
        0 => "trick-6 lead (kernel roots)",
        1..=3 => "trick-6 mid-trick",
        4 => "trick-7 boundary (every seat holds one tile; play is forced)",
        _ => "trick-7 mid-trick",
    }
}

/// One row per occupied ply: `(ply, situations, classes, non-singleton
/// classes, identity classes)`.
fn ply_rows(census: &Census) -> Vec<(usize, usize, usize, usize, usize)> {
    let mut out = Vec::new();
    for p in 0..8 {
        let rows: Vec<usize> = (0..census.carrier.len())
            .filter(|i| ply(&census.carrier.states[*i]) == p)
            .collect();
        if rows.is_empty() {
            continue;
        }
        let classes: std::collections::BTreeSet<usize> =
            rows.iter().map(|i| census.class_of[*i]).collect();
        let merged = classes
            .iter()
            .filter(|c| census.class_members[**c].len() > 1)
            .count();
        let identity: std::collections::BTreeSet<usize> =
            rows.iter().map(|i| census.identity_of[*i]).collect();
        out.push((p, rows.len(), classes.len(), merged, identity.len()));
    }
    out
}

fn write_strata(out: &mut String, census: &Census, verdict: &EclVerdict) {
    out.push_str(
        "class counts stratified by ply (design addendum; ply is bookkeeping outside the state and \
         outside the descriptor, F5's bank-out-of-state ruling applied the same way — the canonical \
         form never reads it). The canonical form encodes structure size and table depth, so the \
         strata partition the classes exactly and these rows sum to the pooled totals.\n",
    );
    let mut situations = 0usize;
    let mut classes_total = 0usize;
    for (p, n, classes, merged, identity) in ply_rows(census) {
        situations += n;
        classes_total += classes;
        let _ = writeln!(
            out,
            "  ply {p} ({}): {n} situations, {classes} classes ({merged} non-singleton), {identity} identity classes, dividend {}   [ECL {}]",
            ply_label(p),
            ratio(identity, classes),
            verdict.verdict()
        );
    }
    let _ = writeln!(
        out,
        "  strata total: {situations} situations, {classes_total} classes (pooled totals: {} and {})",
        census.carrier.len(),
        census.class_members.len()
    );
    assert_eq!(
        situations,
        census.carrier.len(),
        "the strata cover the carrier"
    );
    assert_eq!(
        classes_total,
        census.class_members.len(),
        "the strata partition the classes"
    );
    out.push('\n');
}

fn write_merges(out: &mut String, census: &Census) {
    let cross = census.cross_kernel_classes();
    out.push_str("cross-kernel merges (classes drawing situations from two or more receipt hands — the beyond-the-particular-game signal)\n");
    let root_cross = cross
        .iter()
        .filter(|c| {
            census.class_members[**c]
                .iter()
                .any(|i| census.carrier.is_root[*i])
        })
        .count();
    let _ = writeln!(
        out,
        "  {} classes of {} merge across hands ({} of them contain a root situation)",
        cross.len(),
        census.class_members.len(),
        root_cross
    );
    let mut spread = [0usize; 14];
    for c in &cross {
        spread[census.class_provenance(*c).count_ones() as usize] += 1;
    }
    let widths: Vec<String> = (2..=13)
        .filter(|n| spread[*n] > 0)
        .map(|n| format!("{n} hands: {}", spread[n]))
        .collect();
    if !widths.is_empty() {
        let _ = writeln!(out, "  spread — {}", widths.join(", "));
    }
    for c in cross.iter().take(3) {
        let _ = writeln!(
            out,
            "  example class#{c} (key {}):",
            &census.class_keys[*c]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()[..24.min(2 * census.class_keys[*c].len())]
        );
        let mut shown: u32 = 0;
        for i in &census.class_members[*c] {
            let p = census.carrier.provenance[*i];
            if (p & !shown) == 0 {
                continue;
            }
            shown |= p;
            let hands: Vec<String> = census
                .carrier
                .hands
                .iter()
                .enumerate()
                .filter(|(slot, _)| (p & (1u32 << slot)) != 0)
                .map(|(_, h)| format!("h{h}"))
                .collect();
            let _ = writeln!(
                out,
                "    [{}] {}",
                hands.join(","),
                census.carrier.states[*i].render()
            );
        }
    }
    out.push('\n');
}

fn write_baseline(out: &mut String, census: &Census) {
    out.push_str("identity-interface baseline (F6: the same invariant list read with absolute tiles, absolute seats and absolute contexts — v0.5's closing corollary recovers §12.6 exactly)\n");
    let _ = writeln!(
        out,
        "  pooled full carrier: {} identity classes over {} situations",
        census.identity_members.len(),
        census.carrier.len()
    );
    let _ = writeln!(
        out,
        "  pooled roots only:   {} identity classes over {} roots",
        census.root_identity_classes(),
        census.carrier.roots()
    );
    let _ = writeln!(
        out,
        "  cross-kernel identity merges: {} (the control is expected to merge next to nothing across hands)",
        census.cross_kernel_identity_classes().len()
    );
    let _ = writeln!(
        out,
        "  equivariance dividend (identity classes / equivariant classes): {} full-carrier, {} roots-only",
        ratio(census.identity_members.len(), census.class_members.len()),
        ratio(census.root_identity_classes(), census.root_classes())
    );
    out.push('\n');
}

fn write_verdict(out: &mut String, verdict: &EclVerdict) {
    out.push_str("ECL verdict (F5: exhaustive primitive-step check — condition 1 legality under Theta^A, condition 2 the joint law of (count-free increment, transported observation, successor class); exact rationals, no tolerance)\n");
    let _ = writeln!(
        out,
        "  {}: {} classes total, {} singleton (vacuous), {} classes checked, {} representative-member pairs checked under condition 1 and {} under condition 2",
        verdict.verdict(),
        verdict.classes,
        verdict.singleton_classes,
        verdict.classes_checked,
        verdict.cond1_checks,
        verdict.cond2_checks
    );
    if verdict.passed() {
        out.push_str("  no counterexample: on this domain the finest structural relabeling quotient is equivariantly strongly controlled-lumpable.\n");
        return;
    }
    let _ = writeln!(
        out,
        "  {} counterexample pairs recorded (F7 NO-RESCUE: recorded, never patched in-run; the descriptor was not adjusted). Every pair below is a class whose members disagree.",
        verdict.failures.len()
    );
    let mut by_condition: std::collections::BTreeMap<&str, usize> =
        std::collections::BTreeMap::new();
    for f in &verdict.failures {
        *by_condition.entry(f.condition.as_str()).or_insert(0) += 1;
    }
    for (cond, n) in &by_condition {
        let _ = writeln!(out, "    {cond}: {n} pairs");
    }
    out.push_str("\n  counterexamples (class key, divergent statistic with exact values, both concrete witnesses):\n");
    for f in &verdict.failures {
        let _ = writeln!(out, "    class#{} key {}", f.class, f.class_key);
        let _ = writeln!(out, "      condition: {}", f.condition);
        let _ = writeln!(out, "      divergence: {}", f.detail);
        let _ = writeln!(out, "      representative: {}", f.representative);
        let _ = writeln!(out, "      member:         {}", f.member);
    }
}
