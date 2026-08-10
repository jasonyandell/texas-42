//! §16.11 certificate emission (S5c-m3): every lesson emits a
//! self-describing certificate an independent implementation can check.
//!
//! The schema (`walt/walt-factory/docs/certificate-schema.md`, schema-v1)
//! has ELEVEN record kinds — the §16.11 list, specialized to scalar
//! lessons. Records inapplicable to a scalar lesson (rational affine
//! segments, breakpoint/continuity witnesses, information-price segments)
//! are emitted PRESENT-AND-EMPTY, marked NOT-APPLICABLE with a reason —
//! absence is indistinguishable from omission, so nothing is omitted.
//!
//! **Coverage honesty**: every record carries a checker-coverage
//! annotation; records no independent implementation can verify today are
//! marked UNCHECKED-EXTERNALLY, and the certificate claims only its
//! checked subset. Today's coverage: fiber enumeration/counts and kernel
//! reconstruction (the preserved probe fiber machinery + receipt replay),
//! (C) value distributions (the preserved exp5 scalar PI solver), and
//! atom truth vectors where the exp3a port covers the vocabulary. H value
//! rows have NO independent checker: they are emitted
//! UNCHECKED-EXTERNALLY — registration alone clears nothing, and ledger
//! clearance is per rent row, never per certificate — while the ledger
//! separately blocks H-rent deletions (the sequencing law lives in
//! `ledger`, in types, not in prose here).
//!
//! **Value rows in multiset form** (walt-math ADJUDICATED, confirmed with
//! provisos): record 5's (C) content per matched decision is the exact
//! distribution of the value pair the verdict reads — (v_better, v_worse)
//! for refutations, (v_action, v_optimum) for wins — as `pair -> world
//! count`. The per-world pairing is inside each element, so the multiset
//! determines the worldwise verdict exactly. Provisos implemented here:
//! the comparison protocol is declared verbatim in the record, the
//! coverage annotation states that WORLD-ALIGNMENT IS UNCHECKED (an
//! alignment bug that agrees on the aggregate passes multiset comparison
//! — masks implementation disagreement, never a false claim), and record
//! 9 stays PER-WORLD (truth vectors in canonical world order) — that is
//! the dependency letting a checker reconstruct which worlds are in the
//! multiset.
//!
//! Deterministic text; exact integers and rationals; no timestamps.

use std::collections::BTreeMap;

use crate::basin::BasinDomain;
use crate::db::{verdict_kind, ContentKey};
use crate::generalize::{cell_holds_at, lesson_applies, INTRO_BUDGET};
use crate::ledger::{
    HCheckerRegistry, HLessonDetail, HRowOutcome, H_BUDGET_SEMANTICS, H_CACHE_CONFIG,
};
use crate::lesson::{
    Constraint, Lesson, LessonAtom, LessonOrigin, LessonVerdict, NumericAtom, StepOutcome,
    TraceStep,
};
use crate::lesson_report::render_witness;

/// The schema version every certificate self-identifies with.
pub const SCHEMA_VERSION: &str = "schema-v1";

/// The eleven §16.11 record kinds, in emission order. The count is a CI
/// assertion (`tests`).
pub const RECORD_KINDS: [&str; 11] = [
    "kernel-reconstruction",
    "world-enumeration-count",
    "field-and-belief",
    "policy-witness",
    "terminal-feature-law-scalar",
    "rational-affine-segments",
    "breakpoint-continuity-witnesses",
    "response-class-labels",
    "descriptor-truth-vectors",
    "cell-purity-counterexample-witnesses",
    "information-price-segments",
];

/// Deterministic certificate filename from the content key (Fork 8: the
/// filename is a function of projected content, never of origin or epoch).
pub fn certificate_filename(lesson: &Lesson) -> String {
    format!(
        "cert_{}_{:016x}.txt",
        verdict_kind(lesson),
        ContentKey::of(lesson).hash()
    )
}

fn origin_tuple(lesson: &Lesson) -> String {
    match &lesson.origin {
        LessonOrigin::Regret(c) => {
            format!(
                "h{} {} t{} p{} (regret conflict)",
                c.hand, c.seat, c.trick_no, c.ply
            )
        }
        LessonOrigin::Lumpability {
            hand,
            trick_no,
            descriptor,
            ..
        } => format!("h{hand} t{trick_no} (§12.6 {descriptor} failure, viewer = trick leader)"),
    }
}

/// Is this final-implicant cell independently checkable today? The exp3a
/// port covers the control shapes and the holder/team coordinates; the
/// beater-count columns and their totals are walt-native.
fn cell_coverage(cell: &Constraint) -> &'static str {
    match cell {
        Constraint::Atom(LessonAtom::Ctl(_), _)
        | Constraint::Atom(LessonAtom::Holder(_), _)
        | Constraint::Atom(LessonAtom::Team(_), _) => {
            "CHECKED (exp3a port: walt/probes/exp3a/lambda_probe_v3.py Part 1)"
        }
        Constraint::Atom(LessonAtom::Beaters(_), _) => {
            "UNCHECKED-EXTERNALLY (walt-native beater vectors)"
        }
        Constraint::NumericGe(NumericAtom::OppBeaters, _)
        | Constraint::NumericLe(NumericAtom::OppBeaters, _) => "CHECKED (exp3a port: opp-beaters)",
        Constraint::NumericGe(NumericAtom::BeatersTotal(_), _)
        | Constraint::NumericLe(NumericAtom::BeatersTotal(_), _) => {
            "UNCHECKED-EXTERNALLY (walt-native beater totals)"
        }
        _ => "CHECKED (decision-level public fact: receipt replay)",
    }
}

/// Emits the full certificate text for one lesson. `h_detail` carries the
/// per-decision H measurement where one was taken (`None` for the checker
/// lesson); `registry` drives the H coverage annotation honestly.
pub fn emit_certificate(
    lesson: &Lesson,
    domain: &BasinDomain,
    h_detail: Option<&HLessonDetail>,
    registry: &HCheckerRegistry,
) -> String {
    let key = ContentKey::of(lesson);
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };

    line(format!(
        "walt lesson certificate — {SCHEMA_VERSION} — exploratory tier (never an axiom; TRUST-01)"
    ));
    line(format!("content-key hash: {:016x}", key.hash()));
    line("canonical content (the identity projection — origin/trace/epoch excluded):".to_string());
    for l in key.canonical.lines() {
        line(format!("  | {l}"));
    }
    line(String::new());

    // Matched-decision handles used by several records.
    let matched: Vec<(usize, &crate::basin::DomainDecision, Vec<usize>)> = domain
        .decisions
        .iter()
        .enumerate()
        .filter_map(|(di, d)| lesson_applies(lesson, d).map(|idx| (di, d, idx)))
        .collect();
    assert_eq!(
        matched.len(),
        lesson.basin.decisions_matched,
        "gate-applied decisions reproduce the basin"
    );
    assert_eq!(
        matched.iter().map(|(_, _, idx)| idx.len()).sum::<usize>(),
        lesson.basin.worlds_matched,
        "gate-matched worlds reproduce the basin"
    );

    // [1/11] kernel reconstruction.
    line(format!(
        "[1/11] {} — coverage: CHECKED (receipt replay + preserved fiber machinery, walt/probes/exp5)",
        RECORD_KINDS[0]
    ));
    line(
        "  receipt: rob/receipts/verify_player.txt (read-only ground truth; pip-trump-only caveat)"
            .to_string(),
    );
    line(format!("  origin decision: {}", origin_tuple(lesson)));
    line(
        "  basin kernels (reconstruct each by replaying the named hand to the start of the named \
         trick; the viewer's prefix is that trick's plays before the viewer):"
            .to_string(),
    );
    for (_, d, _) in &matched {
        line(format!(
            "    h{} {} t{} p{} decl {} viewer-hand {} pool-size {}",
            d.hand,
            d.seat,
            d.trick_no,
            d.ply,
            d.kernel.decl(),
            d.horizon,
            d.kernel.pool().len()
        ));
    }
    if matched.is_empty() {
        line("    (empty basin: no matched decisions)".to_string());
    }
    line(String::new());

    // [2/11] world enumeration / count.
    line(format!(
        "[2/11] {} — coverage: CHECKED (preserved exact fiber enumeration/counting, walt/probes/exp5)",
        RECORD_KINDS[1]
    ));
    line(format!(
        "  domain: {} — {} decisions / {} worlds, {} in-range decisions EXCLUDED by the fiber cap \
         (exclusion, never sampling)",
        lesson.basin.domain,
        lesson.basin.domain_decisions,
        lesson.basin.domain_worlds,
        lesson.basin.domain_excluded
    ));
    if lesson.basin.domain_excluded > 0 {
        line(
            "  control-bias annotation (S5c-m2, travels with every capped domain): the exclusion \
             frontier is control-biased — fiber size anti-correlates with focal control (exp5 \
             covariate), so the excluded set skews low-control"
                .to_string(),
        );
    }
    for (_, d, idx) in &matched {
        line(format!(
            "    h{} {} t{} p{}: fiber {} matched-worlds {}",
            d.hand,
            d.seat,
            d.trick_no,
            d.ply,
            d.worlds.len(),
            idx.len()
        ));
    }
    line(String::new());

    // [3/11] field and belief identifier. Certificate H rows are freshly
    // emitted figures — clearance is per LEDGER row (walt-math amendment)
    // and never applies to them, and registration alone clears nothing:
    // they are UNCHECKED-EXTERNALLY until an independent H checker
    // re-derives them.
    line(format!(
        "[3/11] {} — coverage: CHECKED (declaration well-formedness only; labels are declared, not computed)",
        RECORD_KINDS[2]
    ));
    line(format!("  lesson grade: {}", lesson.grade));
    line(format!(
        "  H-checker registry at emission: {} registered{}",
        registry.registered().len(),
        if registry.registered().is_empty() {
            String::new()
        } else {
            format!(
                " ({})",
                registry
                    .registered()
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        }
    ));
    line(
        "  (C) rows: operator (C, minimax-omniscient); root weighting uniform-over-fiber"
            .to_string(),
    );
    match h_detail {
        Some(detail) => {
            line(format!(
                "  (H) rows: operator (H, fixed-uniform-legal); root weighting uniform-over-fiber; \
                 budget {} particle-steps per decision; budget semantics: {}; cache: {} — the \
                 measurability envelope is part of the claim",
                detail.budget_per_decision, H_BUDGET_SEMANTICS, H_CACHE_CONFIG
            ));
        }
        None => line(
            "  (H) rows: none — checker lesson, not re-measured (§12.6 already lives at the fixed \
             field)"
                .to_string(),
        ),
    }
    line(String::new());

    // [4/11] policy witness: selector resolution per applied decision.
    line(format!(
        "[4/11] {} — coverage: UNCHECKED-EXTERNALLY (selector resolution has no registered independent implementation)",
        RECORD_KINDS[3]
    ));
    for (_, d, _) in &matched {
        let resolved = match &lesson.verdict {
            LessonVerdict::Refutation { worse, better } => format!(
                "worse {} -> {} | better {} -> {}",
                worse,
                worse
                    .resolve(&d.actions, d.decisive)
                    .expect("gate-applied decisions resolve"),
                better,
                better
                    .resolve(&d.actions, d.decisive)
                    .expect("gate-applied decisions resolve"),
            ),
            LessonVerdict::Win { action } => format!(
                "action {} -> {}",
                action,
                action
                    .resolve(&d.actions, d.decisive)
                    .expect("gate-applied decisions resolve")
            ),
            LessonVerdict::NotLumpable { descriptor } => {
                format!("descriptor family {descriptor} (no action selector)")
            }
        };
        line(format!(
            "    h{} {} t{} p{} decisive {}: {}",
            d.hand, d.seat, d.trick_no, d.ply, d.decisive, resolved
        ));
    }
    if matched.is_empty() {
        line("    (empty basin)".to_string());
    }
    line(String::new());

    // [5/11] terminal feature law, scalar specialization.
    line(format!(
        "[5/11] {} — coverage: (C) distributions CHECKED (preserved exp5 scalar PI solver) with \
         the caveat that WORLD-ALIGNMENT IS UNCHECKED (a bug assigning pairs to wrong worlds \
         while agreeing on the aggregate passes multiset comparison — masks implementation \
         disagreement, never a false claim; record 9's per-world truth vectors carry matched-set \
         membership); H rows UNCHECKED-EXTERNALLY (single-implementation: no independent \
         re-derivation of these figures)",
        RECORD_KINDS[4]
    ));
    line(
        "  comparison protocol (declared): checker aggregates its own per-world rows to a \
         multiset; multiset equality; asserts no pair has v_better < v_worse (win form: asserts \
         every pair has v_action = v_optimum)"
            .to_string(),
    );
    line(
        "  valuation: q_points (each trick worth 1 + count points of its four tiles, focal minus \
         opponents), future-increment mode (§8.5)"
            .to_string(),
    );
    match &lesson.verdict {
        LessonVerdict::NotLumpable { .. } => {
            line(
                "  checker lesson: no scalar value rows — the verdict reads the §12.6 carrier \
                 (kernel/legal-set agreement), reproducible from record 1's kernels and the \
                 descriptor family"
                    .to_string(),
            );
        }
        LessonVerdict::Refutation { worse, better } => {
            line(
                "  (C) content per matched decision: exact distribution of (v_better, v_worse) \
                 over matched worlds, `pair -> count` — the per-world pairing is inside each \
                 element, so the multiset determines the worldwise verdict"
                    .to_string(),
            );
            for (_, d, idx) in &matched {
                let bi = better
                    .resolve(&d.actions, d.decisive)
                    .and_then(|t| d.actions.iter().position(|a| *a == t))
                    .expect("gate-applied");
                let wi = worse
                    .resolve(&d.actions, d.decisive)
                    .and_then(|t| d.actions.iter().position(|a| *a == t))
                    .expect("gate-applied");
                let mut dist: BTreeMap<(i64, i64), usize> = BTreeMap::new();
                for &w in idx {
                    *dist.entry((d.values[w][bi], d.values[w][wi])).or_default() += 1;
                }
                let total: usize = dist.values().sum();
                assert_eq!(total, idx.len(), "distribution covers every matched world");
                let pairs: Vec<String> = dist
                    .iter()
                    .map(|((b, w), n)| format!("({b},{w})x{n}"))
                    .collect();
                line(format!(
                    "    h{} {} t{} p{} [{} worlds]: {}",
                    d.hand,
                    d.seat,
                    d.trick_no,
                    d.ply,
                    total,
                    pairs.join(" ")
                ));
            }
        }
        LessonVerdict::Win { action } => {
            line(
                "  (C) content per matched decision: exact distribution of (v_action, v_optimum) \
                 over matched worlds, `pair -> count` (v_action = v_optimum everywhere in a \
                 verified win basin — the distribution shows it)"
                    .to_string(),
            );
            for (_, d, idx) in &matched {
                let ai = action
                    .resolve(&d.actions, d.decisive)
                    .and_then(|t| d.actions.iter().position(|a| *a == t))
                    .expect("gate-applied");
                let mut dist: BTreeMap<(i64, i64), usize> = BTreeMap::new();
                for &w in idx {
                    let best = *d.values[w].iter().max().expect("actions");
                    *dist.entry((d.values[w][ai], best)).or_default() += 1;
                }
                let total: usize = dist.values().sum();
                assert_eq!(total, idx.len(), "distribution covers every matched world");
                let pairs: Vec<String> = dist
                    .iter()
                    .map(|((a, b), n)| format!("({a},{b})x{n}"))
                    .collect();
                line(format!(
                    "    h{} {} t{} p{} [{} worlds]: {}",
                    d.hand,
                    d.seat,
                    d.trick_no,
                    d.ply,
                    total,
                    pairs.join(" ")
                ));
            }
        }
    }
    if let Some(detail) = h_detail {
        line(
            "  (H) rows per fiber-valid applied decision (Q^H per legal action, exact rationals; \
             envelope in record 3) — UNCHECKED-EXTERNALLY:"
                .to_string(),
        );
        for row in &detail.rows {
            match &row.outcome {
                HRowOutcome::Solved { values } => {
                    let vals: Vec<String> =
                        values.iter().map(|(a, v)| format!("{a}={v}")).collect();
                    line(format!(
                        "    h{} {} t{} p{} fiber {}: {}",
                        row.hand,
                        row.seat,
                        row.trick_no,
                        row.ply,
                        row.fiber,
                        vals.join(" ")
                    ));
                }
                HRowOutcome::NotFiberValid => line(format!(
                    "    h{} {} t{} p{} fiber {}: NOT-FIBER-VALID (the per-decision H claim does \
                     not extend here)",
                    row.hand, row.seat, row.trick_no, row.ply, row.fiber
                )),
                HRowOutcome::Capped => line(format!(
                    "    h{} {} t{} p{} fiber {}: H-CAPPED at {} particle-steps — UNMEASURED, \
                     never zero",
                    row.hand,
                    row.seat,
                    row.trick_no,
                    row.ply,
                    row.fiber,
                    detail.budget_per_decision
                )),
            }
        }
        if detail.rows.is_empty() {
            line("    (no applied decisions)".to_string());
        }
    }
    line(String::new());

    // [6/11], [7/11], [11/11]: present-and-empty for scalar lessons.
    line(format!(
        "[6/11] {} — NOT-APPLICABLE (scalar lesson: one fixed integer valuation, no λ-parametric \
         envelope; no affine segments exist to record)",
        RECORD_KINDS[5]
    ));
    line(String::new());
    line(format!(
        "[7/11] {} — NOT-APPLICABLE (scalar lesson: no envelope, hence no breakpoints or \
         continuity witnesses)",
        RECORD_KINDS[6]
    ));
    line(String::new());

    // [8/11] response-class labels: the verdict with its quantifier shape.
    line(format!(
        "[8/11] {} — coverage: CHECKED (declaration well-formedness only)",
        RECORD_KINDS[7]
    ));
    line(format!("  verdict: {}", lesson.verdict));
    let quantifier = match &lesson.verdict {
        LessonVerdict::Refutation { .. } => {
            "per matching (decision, world) at (C); at H the shape changes BY NECESSITY to ONE \
             inequality Q^H(better) >= Q^H(worse) per matching decision, atom cells read \
             fiber-valid"
        }
        LessonVerdict::Win { .. } => {
            "per matching (decision, world) at (C) — per-world sufficiency ONLY, never a \
             seat-facing guarantee (§7.6); at H: Q^H(action) = max per matching decision, atom \
             cells read fiber-valid"
        }
        LessonVerdict::NotLumpable { .. } => {
            "per matching decision (ply 0, horizon <= 2), atom cells read fiber-valid"
        }
    };
    line(format!("  quantifier shape: {quantifier}"));
    line(String::new());

    // [9/11] descriptor truth vectors — PER WORLD, never compressed (the
    // adjudicated dependency of record 5's multiset form: matched-set
    // membership is what lets a checker reconstruct which worlds are in
    // the multiset). Vectors are '0'/'1' strings over the CANONICAL world
    // order (schema §canonical-world-order: worlds sorted lexicographically
    // by their hidden hands as ascending tile-index sequences, in hidden-
    // slot order), so an independent enumerator aligns by sorting, not by
    // trusting walt's internal order.
    line(format!(
        "[9/11] {} — coverage: per cell (below); PER-WORLD truth vectors ('0'/'1') in canonical \
         world order per matched decision; the `matched` vector is the conjunction — its \
         popcount equals the matched-world count",
        RECORD_KINDS[8]
    ));
    if lesson.implicant.cells.is_empty() {
        line(
            "    (empty implicant: every eligible in-domain decision matches wholesale — the \
             matched vector is all-ones over each fiber)"
                .to_string(),
        );
    }
    // Canonical permutation per matched decision (world -> sort key =
    // hidden hands as ascending tile-index lists, hidden-slot order).
    let canonical_order = |d: &crate::basin::DomainDecision| -> Vec<usize> {
        let mut keyed: Vec<(Vec<u8>, usize)> = d
            .worlds
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let mut key = Vec::new();
                for h in d.kernel.hidden() {
                    let mut tiles: Vec<u8> =
                        w.hand(h.seat).iter().map(|t| t.index() as u8).collect();
                    tiles.sort_unstable();
                    key.extend(tiles);
                    key.push(u8::MAX); // hand separator keeps keys prefix-free
                }
                (key, i)
            })
            .collect();
        keyed.sort();
        keyed.into_iter().map(|(_, i)| i).collect()
    };
    for (_, d, idx) in &matched {
        let order = canonical_order(d);
        line(format!(
            "  decision h{} {} t{} p{} (fiber {}, canonical world order):",
            d.hand,
            d.seat,
            d.trick_no,
            d.ply,
            d.worlds.len()
        ));
        for cell in &lesson.implicant.cells {
            let bits: String = order
                .iter()
                .map(|&w| if cell_holds_at(d, cell, w) { '1' } else { '0' })
                .collect();
            let satisfied = bits.bytes().filter(|&b| b == b'1').count();
            line(format!(
                "    cell {} [{}] satisfied {}/{}: {}",
                cell,
                cell_coverage(cell),
                satisfied,
                d.worlds.len(),
                bits
            ));
        }
        let in_matched: Vec<bool> = {
            let mut v = vec![false; d.worlds.len()];
            for &w in idx {
                v[w] = true;
            }
            v
        };
        let bits: String = order
            .iter()
            .map(|&w| if in_matched[w] { '1' } else { '0' })
            .collect();
        let popcount = bits.bytes().filter(|&b| b == b'1').count();
        assert_eq!(
            popcount,
            idx.len(),
            "the matched vector's popcount equals the matched-world count"
        );
        line(format!(
            "    matched (all cells) {popcount}/{}: {}",
            d.worlds.len(),
            bits
        ));
    }
    line(String::new());

    // [10/11] cell-purity / counterexample witnesses: the widening trace.
    line(format!(
        "[10/11] {} — coverage: (C) witness value rows CHECKED (exp5 scalar PI solver); witness \
         worlds are complete deals (reconstruction data)",
        RECORD_KINDS[9]
    ));
    line(format!(
        "  initial implicant ({} cells): {}",
        lesson.initial.cells.len(),
        lesson.initial.render()
    ));
    for step in &lesson.trace {
        match step {
            TraceStep::Drop { cell, outcome } => match outcome {
                StepOutcome::Dropped => line(format!("  drop {cell} -> dropped")),
                StepOutcome::Survives(w) => {
                    line(format!("  drop {cell} -> SURVIVES; {}", render_witness(w)))
                }
                StepOutcome::BoundHeld { held, witness } => line(format!(
                    "  relax {cell} -> BOUND HELD at {held}; {}",
                    render_witness(witness)
                )),
            },
            TraceStep::Introduce { cell, witness } => line(format!(
                "  introduce {cell} (cut refinement, distinct from drops) excluding {}",
                render_witness(witness)
            )),
        }
    }
    line(format!(
        "  intro budget: {}/{} spent",
        lesson.introduced().len(),
        INTRO_BUDGET
    ));
    line(String::new());

    line(format!(
        "[11/11] {} — NOT-APPLICABLE (scalar lesson: no support-function price decomposition is \
         part of this claim)",
        RECORD_KINDS[10]
    ));
    line(String::new());

    // The honest closing summary: only the checked subset is claimed.
    let mut checked = vec!["kernel-reconstruction", "world-enumeration-count"];
    let mut unchecked = vec!["policy-witness"];
    checked.push("field-and-belief (well-formedness)");
    checked.push("response-class-labels (well-formedness)");
    if matches!(lesson.verdict, LessonVerdict::NotLumpable { .. }) {
        unchecked.push(
            "terminal-feature-law-scalar (checker carrier: no independent §12.6 implementation)",
        );
    } else {
        checked.push(
            "terminal-feature-law-scalar, (C) distributions (world-alignment unchecked — \
             multiset comparison)",
        );
        if h_detail.is_some() {
            unchecked.push("terminal-feature-law-scalar, H rows");
        }
    }
    checked.push("descriptor-truth-vectors (exp3a-covered cells)");
    unchecked.push("descriptor-truth-vectors (walt-native beater cells)");
    checked.push("cell-purity-counterexample-witnesses ((C) value rows)");
    line(format!(
        "coverage summary — this certificate claims only its checked subset. CHECKED: [{}]. \
         UNCHECKED-EXTERNALLY: [{}]. NOT-APPLICABLE: [rational-affine-segments, \
         breakpoint-continuity-witnesses, information-price-segments].",
        checked.join("; "),
        unchecked.join("; ")
    ));
    out
}
