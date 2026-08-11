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
use walt_skeleton::equivariant::{build_carrier, check_ecl, trick_six_kernels, Census, EclVerdict};

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
    let census = Census::build(carrier);
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
    write_merges(&mut out, &census);
    write_baseline(&mut out, &census);
    write_verdict(&mut out, &verdict);

    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(root.join("results")).expect("results dir");
    std::fs::write(root.join("results/census_2026-08-10.txt"), out).expect("write results");
    eprintln!("wrote results/census_2026-08-10.txt");
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
            .filter(|i| census.carrier.states[*i].matched().len() == n)
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
