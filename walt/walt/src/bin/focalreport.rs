//! EXPLORATORY FOCAL-HORIZON INSTRUMENT (`solver::focal_horizon`, slice
//! FH1) — sits below every evidentiary tier and is cited by nothing
//! above it. Instrument output only: per (root, contract, tail, k), every
//! root action's focal-horizon interval `[L_{a,k}, U_{a,k}]`, width and
//! survivor mark; the root verdict; the materialized `π_k`'s id; the
//! executable lower, the global upper and the certified regret `Γ_k`;
//! the independent focal depth; and the spend line with the ply
//! distribution of tail consultations. Never a play-strength claim.
//!
//! DECLARED EPOCH: the σ0 Level0 { n0 = 2 } modeled mind under
//! `SupportOracle` as the field; the frozen `verify_player` receipt; the
//! four trick-4 gated roots at the receipt contract for k ∈ {0, 1, 2}
//! under the σ0 tail (σ0 driving the viewer seat, FH-A4's primary tail);
//! `FixedPreference::lowest_first` as the gate-only second tail.
//!
//! Modes:
//!   `focalreport scout <hand> <trick> <k> [contract] [node-cap] [tail] [exact]`
//!   `focalreport scout-corpus <out.txt>`
//!   `focalreport ladder <hand> <trick> <contract|receipt> [nomemo] [cap=N] <k:ceiling>...`
//!   `focalreport ladder-record <out.txt>`
//!   `focalreport report <out.txt> [h<hand>-t<trick> ...]`
//!
//! `tail` is `sigma0` (default) or `lowest`; a trailing `exact` also
//! prices every `Q_a` by `response_success_mass` and prints the
//! per-action split `(U − Q) + (Q − L)` (walt-math W3). The
//! report-of-record mode is slice FH3's; the binary stays open for it.
//!
//! `ladder` (slice FH2, `solver::focal_ladder`) runs ONE ladder through
//! a schedule of `k:ceiling` steps (ceiling = field + tail reads for that
//! pass; `k:inf` for no ceiling) with the suffix memo on (`nomemo` turns
//! it off — the memory and read comparison; `cap=N` sets the node fiber
//! cap, default 40,000), printing per
//! step the outcome, reads, residual frontier, fact-store movement,
//! suffix hits and the derived root view. `ladder-record` is the record
//! of record over h8-t4 and h3-t4 at the receipt contract with the
//! pinned schedules stated in its header.
//!
//! `report` (slice FH3) is THE REPORT OF RECORD (the parent's §38): for
//! every (root, contract) of the corpus — the four trick-4 gated roots ×
//! contracts {receipt, 33, 36, 39, 42}, the six trick-5/6 roots ×
//! {receipt, 36} — and the FH8 anchors (h8-t3 at the receipt contract;
//! h8-t4 at 36/39 and h4-t4 across contracts are corpus rows), at every
//! horizon k ∈ {0, 1, 2, 3}: the direct engine per k (reads WITHOUT
//! reuse, FH1-comparable) and ONE ladder per coordinate walked k = 0, 1,
//! 2, 3 with the suffix memo on (reads WITH reuse); per action `L`, `U`,
//! `U − L`, `Δ^L`, `Δ^U`, the survivor mark, `Q_a` (a fresh
//! `response_success_mass` where affordable; the record's values, cited,
//! at h8-t3) and the split; the verdict, `π_k`, `L_exec`, `U*`, `Γ_k`,
//! the lower policy's root action and its changes by horizon; the
//! independent focal depth; the spend; the ply cut's argmax under
//! Proposition FH-cut on viewer-lead roots. The §41 laws (containment,
//! nesting, ladder ≡ direct in value, `Settled ⇒ exact argmax`) are
//! asserted per coordinate; a failure stops the record naming the
//! coordinate. Filters restrict the corpus for scouting; the record is
//! the unfiltered run. Corpus coordinates run on a worker pool (walls
//! are contended); the h8-t3 anchor runs alone after them.
//!
//! No floats anywhere; wall time is integer microseconds and the one
//! approximate number.

use std::fmt::Write as _;
use std::io::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::rules::Domino;
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::factor_belief::{
    response_success_mass, ExactCoverOracle, FactorBelief, ResponseStats, SupportOracle,
};
use walt::solver::field::{FieldKind, FieldModel, FieldSpec};
use walt::solver::focal_horizon::{
    focal_depth, focal_horizon, FocalHorizonResult, FocalRefusal, FocalSpec, FocalVerdict,
};
use walt::solver::focal_ladder::{
    FocalLadder, LadderContext, LadderView, Outcome, ResidualCause, SuffixMemo, WorkBudget,
};
use walt::solver::horizon::with_contract;
use walt::solver::policy::{DecisionMode, TieRule};

const T4_CORPUS: [(usize, usize); 4] = [(3, 4), (4, 4), (8, 4), (12, 4)];
const CORPUS_HORIZONS: [usize; 3] = [0, 1, 2];
/// Admits every frontier node under a trick-4 root (the largest trick-4
/// receipt fiber is 34,650).
const NODE_CAP: u128 = 40_000;

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

fn ratio(m: u128, z: u128) -> BigRational {
    BigRational::new(BigInt::from(m), BigInt::from(z))
}

enum Tail {
    Sigma0,
    Lowest,
}

impl Tail {
    fn parse(s: &str) -> Tail {
        match s {
            "sigma0" => Tail::Sigma0,
            "lowest" => Tail::Lowest,
            other => panic!("unknown tail {other}: sigma0 | lowest"),
        }
    }
}

struct Run {
    label: String,
    result: Result<FocalHorizonResult, FocalRefusal>,
    /// `h_f` after each root action, in tile order, and at the root.
    depth_after: Vec<(walt::rules::Domino, usize)>,
    depth_root: usize,
    /// `Q_a` per action when priced.
    exact_q: Option<Vec<(walt::rules::Domino, u128)>>,
    wall_us: u128,
}

/// One scout coordinate.
struct Coordinate {
    hand_id: usize,
    trick_no: usize,
    contract: Option<u32>,
    k: usize,
    cap: u128,
    tail: Tail,
    price_exact: bool,
}

fn run(r: &Receipt, c: &Coordinate) -> Run {
    let Coordinate {
        hand_id,
        trick_no,
        contract,
        k,
        cap,
        ref tail,
        price_exact,
    } = *c;
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = match contract {
        Some(c) => with_contract(&position, c),
        None => position,
    };
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let lowest = FixedPreference::lowest_first("focal:lowest-first");
    let tail_policy: &dyn SlicePolicy = match tail {
        Tail::Sigma0 => &field,
        Tail::Lowest => &lowest,
    };
    let spec = FocalSpec {
        horizon: k,
        node_fiber_cap: cap,
    };
    let t0 = Instant::now();
    let result = focal_horizon(&oracle, &root, &position, tail_policy, &field, &spec);
    let wall_us = t0.elapsed().as_micros();
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let mut depth_after = Vec::new();
    let mut exact_q = if price_exact { Some(Vec::new()) } else { None };
    for a in walt::solver::godgap::legal_actions(&root, &position) {
        let child = belief.focal_play(a);
        depth_after.push((a, focal_depth(&oracle, &child, &field)));
        if let Some(q) = exact_q.as_mut() {
            let mut rs = ResponseStats::default();
            q.push((a, response_success_mass(&oracle, &child, &field, &mut rs)));
        }
    }
    let depth_root = focal_depth(&oracle, &belief, &field);
    Run {
        label: format!("h{hand_id}-t{trick_no}"),
        result,
        depth_after,
        depth_root,
        exact_q,
        wall_us,
    }
}

fn print_run(out: &mut String, run: &Run, tail_name: &str) {
    match &run.result {
        Err(FocalRefusal::UpperUnaffordable {
            history,
            fiber,
            cap,
        }) => {
            let names: Vec<String> = history.iter().map(|d| format!("{d}")).collect();
            let _ = writeln!(
                out,
                "== {} tail {tail_name} | REFUSED: frontier node [{}] fiber {fiber} above cap {cap} \
                 | wall {}us",
                run.label,
                names.join(" "),
                run.wall_us
            );
        }
        Ok(res) => {
            let z = res.root_mass();
            let _ = writeln!(
                out,
                "== {} contract {} tail {tail_name} k={} | Z={} | wall {}us",
                run.label, res.identity.contract, res.identity.horizon, z, run.wall_us
            );
            for a in &res.actions {
                let mark = if res.survivors.contains(&a.action) {
                    "S"
                } else {
                    "-"
                };
                let q = run
                    .exact_q
                    .as_ref()
                    .and_then(|v| v.iter().find(|(t, _)| *t == a.action).map(|(_, m)| *m));
                let split = match q {
                    Some(qm) => format!(
                        " | Q {} | fusion price U−Q {} | policy gap Q−L {}",
                        exact(&ratio(qm, z)),
                        exact(&ratio(a.upper_mass - qm, z)),
                        exact(&ratio(qm - a.lower_mass, z))
                    ),
                    None => String::new(),
                };
                let _ = writeln!(
                    out,
                    "     action {} [{mark}]: L {} | U {} | width {}{split}",
                    a.action,
                    exact(&a.lower),
                    exact(&a.upper),
                    exact(&a.width())
                );
            }
            let verdict = match &res.verdict {
                FocalVerdict::Settled { action } => format!("SETTLED {action}"),
                FocalVerdict::Equivalent {
                    actions,
                    value_mass,
                } => {
                    let names: Vec<String> = actions.iter().map(|d| format!("{d}")).collect();
                    format!(
                        "EQUIVALENT {{{}}} at {}",
                        names.join(" "),
                        exact(&ratio(*value_mass, z))
                    )
                }
                FocalVerdict::Unresolved { survivors } => {
                    let names: Vec<String> = survivors.iter().map(|d| format!("{d}")).collect();
                    format!("UNRESOLVED survivors {{{}}}", names.join(" "))
                }
            };
            let survivors: Vec<String> = res.survivors.iter().map(|d| format!("{d}")).collect();
            let _ = writeln!(
                out,
                "   bar B_k {} | survivors {{{}}} | verdict {verdict}",
                exact(&res.bar()),
                survivors.join(" ")
            );
            let _ = writeln!(
                out,
                "   π_k {} ({} states, plays {}) | L_exec {} | U* {} | Γ_k {}",
                res.policy.id(),
                res.policy.states(),
                res.policy_action(),
                exact(&ratio(res.executable_lower_mass, z)),
                exact(&res.global_upper()),
                exact(&res.certified_regret)
            );
            let depths: Vec<String> = run
                .depth_after
                .iter()
                .map(|(a, d)| format!("{a}:{d}"))
                .collect();
            let _ = writeln!(
                out,
                "   focal depth h_f: root {} | after action {}",
                run.depth_root,
                depths.join(" ")
            );
            let s = &res.spend;
            let plies: Vec<String> = s
                .tail_plies
                .iter()
                .map(|(d, n)| format!("ply{d}:{n}"))
                .collect();
            let _ = writeln!(
                out,
                "   spend: field reads {} | tail reads {} | conditionings {} | focal {} | hidden {} \
                 | decided early {} terminal {} | tail evaluations lower {} upper {} (forced {}) \
                 | worlds {} | line-walk nodes {} | frontier plies [{}]",
                s.field_reads,
                s.tail_reads,
                s.conditionings,
                s.focal_nodes,
                s.hidden_nodes,
                s.decided_early,
                s.decided_terminal,
                s.lower_tail_evaluations,
                s.upper_tail_evaluations,
                s.forced_tail_evaluations,
                s.worlds_enumerated,
                s.line_walk_nodes,
                plies.join(" ")
            );
        }
    }
}

struct Row {
    label: String,
    contract: u32,
    k: usize,
    z: u128,
    survivors: String,
    verdict: String,
    lower: BigRational,
    upper: BigRational,
    regret: BigRational,
    reads: u64,
    tail_evals: u64,
    wall_us: u128,
}

fn summarize(run: &Run) -> Option<Row> {
    let res = run.result.as_ref().ok()?;
    let survivors: Vec<String> = res.survivors.iter().map(|d| format!("{d}")).collect();
    let verdict = match &res.verdict {
        FocalVerdict::Settled { action } => format!("SETTLED {action}"),
        FocalVerdict::Equivalent { actions, .. } => {
            let names: Vec<String> = actions.iter().map(|d| format!("{d}")).collect();
            format!("EQUIV {{{}}}", names.join(" "))
        }
        FocalVerdict::Unresolved { .. } => "UNRESOLVED".to_string(),
    };
    Some(Row {
        label: run.label.clone(),
        contract: res.identity.contract,
        k: res.identity.horizon,
        z: res.root_mass(),
        survivors: survivors.join(" "),
        verdict,
        lower: res.bar(),
        upper: res.global_upper(),
        regret: res.certified_regret.clone(),
        reads: res.spend.field_reads + res.spend.tail_reads,
        tail_evals: res.spend.tail_consultations(),
        wall_us: run.wall_us,
    })
}

fn print_table(out: &mut String, rows: &[Row]) {
    let _ = writeln!(
        out,
        "\n#### THE FOCAL-HORIZON TABLE — one row per (root, contract, k), σ0 tail ####\n"
    );
    let _ = writeln!(
        out,
        " root    | bid | k |    Z    | survivors   | verdict          | B_k ‰ | U* ‰ | Γ_k ‰ | reads     | tail evals | wall"
    );
    let _ = writeln!(
        out,
        "---------+-----+---+---------+-------------+------------------+-------+------+-------+-----------+------------+------"
    );
    for r in rows {
        let _ = writeln!(
            out,
            " {:<7} | {:>3} | {} | {:>7} | {:<11} | {:<16} | {:>5} | {:>4} | {:>5} | {:>9} | {:>10} | {}us",
            r.label,
            r.contract,
            r.k,
            r.z,
            r.survivors,
            r.verdict,
            permille(&r.lower),
            permille(&r.upper),
            permille(&r.regret),
            r.reads,
            r.tail_evals,
            r.wall_us
        );
    }
}

// ---------------------------------------------------------------------------
// The ladder (slice FH2).
// ---------------------------------------------------------------------------

/// One schedule step: the horizon and the read ceiling for that pass.
#[derive(Clone, Copy)]
struct Step {
    k: usize,
    ceiling: u64,
}

fn parse_step(s: &str) -> Step {
    let (k, c) = s
        .split_once(':')
        .unwrap_or_else(|| panic!("a schedule step is k:ceiling, got {s}"));
    let k: usize = k.parse().expect("a horizon");
    let ceiling: u64 = if c == "inf" {
        u64::MAX
    } else {
        c.parse().expect("a read ceiling")
    };
    Step { k, ceiling }
}

fn step_name(step: &Step) -> String {
    if step.ceiling == u64::MAX {
        format!("{}:inf", step.k)
    } else {
        format!("{}:{}", step.k, step.ceiling)
    }
}

fn verdict_name(view: &LadderView) -> String {
    let z = view.root_mass;
    match &view.verdict {
        FocalVerdict::Settled { action } => format!("SETTLED {action}"),
        FocalVerdict::Equivalent {
            actions,
            value_mass,
        } => {
            let names: Vec<String> = actions.iter().map(|d| format!("{d}")).collect();
            format!(
                "EQUIVALENT {{{}}} at {}",
                names.join(" "),
                exact(&ratio(*value_mass, z))
            )
        }
        FocalVerdict::Unresolved { survivors } => {
            let names: Vec<String> = survivors.iter().map(|d| format!("{d}")).collect();
            format!("UNRESOLVED survivors {{{}}}", names.join(" "))
        }
    }
}

fn print_view(out: &mut String, view: &LadderView) {
    let z = view.root_mass;
    for a in &view.actions {
        let mark = if view.survivors.contains(&a.action) {
            "S"
        } else {
            "-"
        };
        let lower = match a.lower_horizon {
            Some(k) => format!("L {} @k{k}", exact(&a.lower())),
            None => "L 0 (placeholder, tail)".to_string(),
        };
        let upper = match (a.upper_mass, a.upper_horizon) {
            (Some(u), Some(k)) => format!(
                "U {} @k{k} | width {}",
                exact(&ratio(u, z)),
                exact(&ratio(u - a.lower_mass, z))
            ),
            _ => "U absent (no fact) | no interval".to_string(),
        };
        let _ = writeln!(out, "     action {} [{mark}]: {lower} | {upper}", a.action);
    }
    let survivors: Vec<String> = view.survivors.iter().map(|d| format!("{d}")).collect();
    let _ = writeln!(
        out,
        "   bar {} (plays {}) | survivors {{{}}} | verdict {}",
        exact(&view.bar()),
        view.bar_action,
        survivors.join(" "),
        verdict_name(view)
    );
    let u_star = view
        .global_upper()
        .map_or("absent".to_string(), |u| exact(&u));
    let gamma = view
        .certified_regret
        .as_ref()
        .map_or("absent".to_string(), exact);
    let horizon = view.horizon.map_or("-".to_string(), |h| format!("{h}"));
    let _ = writeln!(
        out,
        "   π {} ({} states) | L_exec {} | U* {u_star} | Γ {gamma} | established horizon {horizon}",
        view.policy.id(),
        view.policy.states(),
        exact(&ratio(view.executable_lower_mass, z))
    );
}

/// One ladder coordinate: the root, the contract, the cap and the memo switch.
struct LadderRun {
    hand_id: usize,
    trick_no: usize,
    contract: Option<u32>,
    cap: u128,
    use_memo: bool,
}

/// One ladder through a schedule at one root; returns the per-step table
/// rows for the summary.
fn run_ladder(out: &mut String, r: &Receipt, run: &LadderRun, schedule: &[Step]) -> Vec<LadderRow> {
    let LadderRun {
        hand_id,
        trick_no,
        contract,
        cap,
        use_memo,
    } = *run;
    let (root, position) = root_at(r, hand_id, trick_no);
    let position = match contract {
        Some(c) => with_contract(&position, c),
        None => position,
    };
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let ctx = LadderContext {
        oracle: &oracle,
        root: &root,
        position: &position,
        lower_tail: &field,
        field: &field,
    };
    let mut ladder = FocalLadder::open(&ctx);
    let mut memo = SuffixMemo::new();
    let label = format!("h{hand_id}-t{trick_no}");
    let _ = writeln!(
        out,
        "== {label} contract {} tail sigma0 | Z={} | legal {} | schedule [{}] | node cap {cap} | suffix memo {}",
        position.bid,
        ladder.root_mass(),
        ladder.legal().len(),
        schedule.iter().map(step_name).collect::<Vec<_>>().join(" "),
        if use_memo { "on" } else { "off" }
    );
    let mut rows = Vec::new();
    for (i, step) in schedule.iter().enumerate() {
        eprintln!("  {label} step {} ({}) ...", i + 1, step_name(step));
        let t0 = Instant::now();
        let outcome = ladder.advance(
            &ctx,
            step.k,
            &WorkBudget {
                read_ceiling: step.ceiling,
                node_fiber_cap: cap,
            },
            if use_memo { Some(&mut memo) } else { None },
        );
        let wall_us = t0.elapsed().as_micros();
        let report = outcome.report();
        let outcome_name = match &outcome {
            Outcome::Completed { .. } => "COMPLETED".to_string(),
            Outcome::Interrupted {
                residual_frontier,
                stopping_node,
                unaffordable,
                ..
            } => {
                let stop = stopping_node.as_ref().map_or("none".to_string(), |h| {
                    let names: Vec<String> = h.iter().map(|d| format!("{d}")).collect();
                    format!("[{}]", names.join(" "))
                });
                let mass: u128 = residual_frontier.iter().map(|n| n.mass).sum();
                let mut stopped = 0usize;
                let mut enclosing = 0usize;
                let mut unvisited = 0usize;
                let mut refused = 0usize;
                let mut retained = 0usize;
                for n in residual_frontier {
                    match n.cause {
                        ResidualCause::Stopped => stopped += 1,
                        ResidualCause::Enclosing => enclosing += 1,
                        ResidualCause::Unvisited => unvisited += 1,
                        ResidualCause::Unaffordable { .. } => refused += 1,
                    }
                    if n.retained.is_some() {
                        retained += 1;
                    }
                }
                let root_children: Vec<String> = residual_frontier
                    .iter()
                    .filter(|n| n.history.len() == 1)
                    .map(|n| format!("{}", n.history[0]))
                    .collect();
                format!(
                    "INTERRUPTED stop {stop} | residual frontier {} nodes (mass {mass}; stopped {stopped}, \
                     enclosing {enclosing}, unvisited {unvisited}, unaffordable {refused}; {retained} with \
                     retained facts) | unfinished root children {{{}}} | cap refusals {}",
                    residual_frontier.len(),
                    root_children.join(" "),
                    unaffordable.len()
                )
            }
        };
        let _ = writeln!(
            out,
            "-- step {} k={} ceiling {} | {} | reads {} (field {} + tail {}) | wall {wall_us}us",
            i + 1,
            step.k,
            if step.ceiling == u64::MAX {
                "inf".to_string()
            } else {
                format!("{}", step.ceiling)
            },
            outcome_name,
            report.reads_spent,
            report.spend.field_reads,
            report.spend.tail_reads
        );
        let completed: Vec<String> = report
            .children_completed
            .iter()
            .map(|d| format!("{d}"))
            .collect();
        let _ = writeln!(
            out,
            "   facts: {} stored ({} collapsed) | this pass new {} | revisited {} (tightened {}) | root children completed {{{}}}",
            ladder.facts().len(),
            ladder.collapsed_count(),
            report.facts_new,
            report.facts_revisited,
            report.facts_tightened,
            completed.join(" ")
        );
        let first_hit = memo.first_hit.as_ref().map_or("none yet".to_string(), |h| {
            let names: Vec<String> = h.iter().map(|d| format!("{d}")).collect();
            format!("[{}]", names.join(" "))
        });
        let _ = writeln!(
            out,
            "   suffix memo: hits {} / lookups {} this pass | receipts held {} | first hit ever {first_hit} | spend: focal {} hidden {} tail evaluations {} (forced {})",
            report.suffix_hits,
            report.suffix_lookups,
            memo.receipts,
            report.spend.focal_nodes,
            report.spend.hidden_nodes,
            report.spend.tail_consultations(),
            report.spend.forced_tail_evaluations
        );
        print_view(out, &report.view);
        rows.push(LadderRow {
            label: label.clone(),
            step: step_name(step),
            outcome: if outcome.is_completed() {
                "completed".to_string()
            } else {
                "interrupted".to_string()
            },
            survivors: report
                .view
                .survivors
                .iter()
                .map(|d| format!("{d}"))
                .collect::<Vec<_>>()
                .join(" "),
            verdict: match &report.view.verdict {
                FocalVerdict::Settled { action } => format!("SETTLED {action}"),
                FocalVerdict::Equivalent { .. } => "EQUIV".to_string(),
                FocalVerdict::Unresolved { .. } => "UNRESOLVED".to_string(),
            },
            bar: report.view.bar(),
            regret: report.view.certified_regret.clone(),
            reads: report.reads_spent,
            hits: report.suffix_hits,
            facts: ladder.facts().len(),
            wall_us,
        });
    }
    rows
}

struct LadderRow {
    label: String,
    step: String,
    outcome: String,
    survivors: String,
    verdict: String,
    bar: BigRational,
    regret: Option<BigRational>,
    reads: u64,
    hits: u64,
    facts: usize,
    wall_us: u128,
}

fn print_ladder_table(out: &mut String, rows: &[LadderRow]) {
    let _ = writeln!(
        out,
        "\n#### THE LADDER TABLE — one row per schedule step, σ0 tail, suffix memo on ####\n"
    );
    let _ = writeln!(
        out,
        " root    | step       | outcome     | survivors   | verdict      | bar ‰ | Γ ‰  | reads     | hits   | facts  | wall"
    );
    let _ = writeln!(
        out,
        "---------+------------+-------------+-------------+--------------+-------+------+-----------+--------+--------+------"
    );
    for r in rows {
        let _ = writeln!(
            out,
            " {:<7} | {:<10} | {:<11} | {:<11} | {:<12} | {:>5} | {:>4} | {:>9} | {:>6} | {:>6} | {}us",
            r.label,
            r.step,
            r.outcome,
            r.survivors,
            r.verdict,
            permille(&r.bar),
            r.regret
                .as_ref()
                .map_or("-".to_string(), |g| format!("{}", permille(g))),
            r.reads,
            r.hits,
            r.facts,
            r.wall_us
        );
    }
}

/// The record's pinned schedules (stated in its header).
const LADDER_RECORD: [(usize, usize, &str); 2] = [
    (8, 4, "0:150000 0:inf 1:250000 1:inf 2:inf"),
    (3, 4, "0:800000 0:inf 1:1200000 1:inf 2:inf"),
];

// ---------------------------------------------------------------------------
// The report of record (slice FH3, the parent's §38).
// ---------------------------------------------------------------------------

/// The trick-4 corpus contracts: the receipt's and four fixed bids.
const REPORT_T4_CONTRACTS: [Option<u32>; 5] = [None, Some(33), Some(36), Some(39), Some(42)];
/// The trick-5/6 corpus (FH1's T56) and its contracts.
const REPORT_T56: [(usize, usize); 6] = [(8, 5), (3, 5), (12, 6), (10, 6), (5, 6), (4, 6)];
const REPORT_T56_CONTRACTS: [Option<u32>; 2] = [None, Some(36)];
/// The deepest corpus horizon: `h_f = 3` at trick 4, one beyond FH-last's
/// collapse; constant beyond `h_f` at trick 5/6.
const REPORT_KMAX: usize = 3;
/// Corpus coordinates run on this many workers; the h8-t3 anchor runs
/// alone after them.
const REPORT_WORKERS: usize = 6;
/// Anchor (i): run the ladder's k = 3 (the exact solve, FH-last) only
/// when the k = 2 pass came in under this wall (BRIEF-FH3: ~10 min).
const H8T3_K3_WALL_GATE_US: u128 = 600_000_000;
/// Anchor (i): the direct engine runs at k ≤ this (measured: k = 0 is
/// affordable; each further direct horizon re-walks without reuse).
const H8T3_DIRECT_KMAX: usize = 1;

/// Anchor (i)'s exact per-action values, CITED from the record
/// (`horizon_run1.txt`, h8-t3 contract 30 cut 4: the 14-minute exact
/// solve, `Q* = 28859/29988` argmax 1-1) — never recomputed here. Masses
/// over `Z = 59,976`.
const RECORD_H8T3_Z: u128 = 59_976;
const RECORD_H8T3_Q: [(&str, u128); 5] = [
    ("1-1", 57_718),
    ("2-1", 55_706),
    ("3-1", 53_405),
    ("3-3", 57_280),
    ("5-5", 55_316),
];

/// One coordinate of the record.
struct ReportSpec {
    hand_id: usize,
    trick_no: usize,
    contract: Option<u32>,
    /// Direct-engine horizons `0..=direct_kmax`; `None` = not run here.
    direct_kmax: Option<usize>,
    /// Ladder horizons `0..=ladder_kmax`, memo on, sequential.
    ladder_kmax: usize,
    /// Run the ladder one horizon further when the last pass's wall is
    /// under this (the affordability decision is printed either way).
    ladder_extend_gate_us: Option<u128>,
    /// `Q_a` by a fresh `response_success_mass` (else the record's, cited).
    fresh_exact: bool,
    /// `h_f` after each action by the independent walk (else FH-A6's law).
    depth_walk: bool,
    anchor: Option<&'static str>,
}

impl ReportSpec {
    fn label(&self) -> String {
        format!("h{}-t{}", self.hand_id, self.trick_no)
    }
}

/// One finished coordinate: its printed section and its table rows.
type Finished = (String, Vec<ReportRow>);

/// One (root, contract, k) line of the record's table.
#[derive(Clone)]
struct ReportRow {
    label: String,
    contract: u32,
    k: usize,
    z: u128,
    survivors: String,
    verdict: String,
    plays: String,
    bar: BigRational,
    upper: Option<BigRational>,
    regret: Option<BigRational>,
    reads_direct: Option<u64>,
    reads_ladder: u64,
    hits: u64,
    facts: usize,
    wall_direct_us: Option<u128>,
    wall_ladder_us: u128,
}

/// One horizon's uniform view for the law checks: the direct result
/// where it ran, else the ladder's derived view.
struct KView {
    k: usize,
    actions: Vec<(Domino, u128, Option<u128>)>,
    bar_mass: u128,
    bar_action: Domino,
    verdict: FocalVerdict,
    survivors: Vec<Domino>,
    regret: Option<BigRational>,
}

fn kview_direct(res: &FocalHorizonResult) -> KView {
    KView {
        k: res.identity.horizon,
        actions: res
            .actions
            .iter()
            .map(|a| (a.action, a.lower_mass, Some(a.upper_mass)))
            .collect(),
        bar_mass: res.bar_mass,
        bar_action: res.policy_action(),
        verdict: res.verdict.clone(),
        survivors: res.survivors.clone(),
        regret: Some(res.certified_regret.clone()),
    }
}

fn kview_ladder(k: usize, view: &LadderView) -> KView {
    KView {
        k,
        actions: view
            .actions
            .iter()
            .map(|a| (a.action, a.lower_mass, a.upper_mass))
            .collect(),
        bar_mass: view.bar_mass,
        bar_action: view.bar_action,
        verdict: view.verdict.clone(),
        survivors: view.survivors.clone(),
        regret: view.certified_regret.clone(),
    }
}

fn names(ds: &[Domino]) -> String {
    ds.iter()
        .map(|d| format!("{d}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn verdict_short(v: &FocalVerdict) -> String {
    match v {
        FocalVerdict::Settled { action } => format!("SETTLED {action}"),
        FocalVerdict::Equivalent { actions, .. } => format!("EQUIV {{{}}}", names(actions)),
        FocalVerdict::Unresolved { .. } => "UNRESOLVED".to_string(),
    }
}

/// The lowest-tile argmax of a per-action mass (the ply cut's argmax
/// under Proposition FH-cut when the mass is `U_{a,m−1}`).
fn lowest_tile_argmax(values: &[(Domino, u128)]) -> Domino {
    let mut best: Option<(u128, Domino)> = None;
    for (a, m) in values {
        let take = match best {
            None => true,
            Some((b, _)) => *m > b,
        };
        if take {
            best = Some((*m, *a));
        }
    }
    best.expect("a legal set holds an action").1
}

/// The §41 laws at one coordinate, over every horizon that ran. A
/// failure stops the record with the coordinate named.
fn check_laws(coord: &str, views: &[KView], q: &[(Domino, u128)], q_source: &str) {
    let qmax = q.iter().map(|(_, m)| *m).max().expect("a root action");
    let exact_set: Vec<Domino> = q
        .iter()
        .filter(|(_, m)| *m == qmax)
        .map(|(a, _)| *a)
        .collect();
    for v in views {
        for (a, l, u) in &v.actions {
            let qa = q
                .iter()
                .find(|(t, _)| t == a)
                .map(|(_, m)| *m)
                .unwrap_or_else(|| panic!("{coord}: Q_a is known at every root action"));
            assert!(
                *l <= qa,
                "§41(2) at {coord} k={}: L_{a} = {l} > Q_a = {qa} ({q_source})",
                v.k
            );
            if let Some(u) = u {
                assert!(
                    qa <= *u,
                    "§41(2) at {coord} k={}: Q_a = {qa} > U_{a} = {u} ({q_source})",
                    v.k
                );
            }
        }
        match &v.verdict {
            FocalVerdict::Settled { action } => assert!(
                exact_set == vec![*action],
                "Settled ⇒ exact argmax FAILED at {coord} k={}: Settled {action} but the exact \
                 maximizers are {{{}}} ({q_source})",
                v.k,
                names(&exact_set)
            ),
            FocalVerdict::Equivalent {
                actions,
                value_mass,
            } => {
                assert!(
                    *actions == exact_set && *value_mass == qmax,
                    "Equivalent ⇒ exact tie set FAILED at {coord} k={}: {{{}}} at {value_mass} vs \
                     {{{}}} at {qmax} ({q_source})",
                    v.k,
                    names(actions),
                    names(&exact_set)
                );
            }
            FocalVerdict::Unresolved { survivors } => {
                for a in &exact_set {
                    assert!(
                        survivors.contains(a),
                        "§41 containment FAILED at {coord} k={}: exact maximizer {a} is not a \
                         survivor ({q_source})",
                        v.k
                    );
                }
            }
        }
    }
    for w in views.windows(2) {
        let (p, n) = (&w[0], &w[1]);
        for ((a, lp, up), (b, ln, un)) in p.actions.iter().zip(&n.actions) {
            assert_eq!(a, b, "{coord}: the same root actions at every horizon");
            assert!(
                ln >= lp,
                "§41(3) at {coord}: L_{a} fell {lp} → {ln} from k={} to k={}",
                p.k,
                n.k
            );
            if let (Some(up), Some(un)) = (up, un) {
                assert!(
                    un <= up,
                    "§41(4) at {coord}: U_{a} rose {up} → {un} from k={} to k={}",
                    p.k,
                    n.k
                );
            }
        }
        assert!(n.bar_mass >= p.bar_mass, "{coord}: the bar never falls");
        for a in &n.survivors {
            assert!(
                p.survivors.contains(a),
                "{coord}: survivors only shrink (Theorem 6) — {a} appeared at k={}",
                n.k
            );
        }
        if let (Some(gp), Some(gn)) = (&p.regret, &n.regret) {
            assert!(gn <= gp, "{coord}: Γ never rises (P8(ii))");
        }
    }
}

/// The direct result and the ladder's derived view at one horizon agree
/// on every value (the policy id may differ on a lower-side tie, FH2).
fn check_ladder_parity(coord: &str, k: usize, res: &FocalHorizonResult, view: &LadderView) {
    for (d, l) in res.actions.iter().zip(&view.actions) {
        assert_eq!(d.action, l.action, "{coord} k={k}: action order");
        assert_eq!(
            (d.lower_mass, Some(d.upper_mass)),
            (l.lower_mass, l.upper_mass),
            "ladder ≡ direct FAILED at {coord} k={k} action {}: direct [{}, {}] vs ladder [{}, {:?}]",
            d.action,
            d.lower_mass,
            d.upper_mass,
            l.lower_mass,
            l.upper_mass
        );
    }
    assert_eq!(res.bar_mass, view.bar_mass, "{coord} k={k}: bar");
    assert_eq!(res.verdict, view.verdict, "{coord} k={k}: verdict");
    assert_eq!(res.survivors, view.survivors, "{coord} k={k}: survivors");
    assert_eq!(
        Some(res.global_upper_mass),
        view.global_upper_mass,
        "{coord} k={k}: U*"
    );
    assert_eq!(
        Some(&res.certified_regret),
        view.certified_regret.as_ref(),
        "{coord} k={k}: Γ"
    );
}

struct DirectRun {
    k: usize,
    result: Result<FocalHorizonResult, FocalRefusal>,
    wall_us: u128,
}

struct LadderPass {
    k: usize,
    outcome: Outcome,
    wall_us: u128,
    facts: usize,
    collapsed: usize,
    receipts: u64,
    first_hit: Option<Vec<Domino>>,
}

/// One coordinate of the record: everything §38 lists, printed, plus its
/// table rows.
fn report_coordinate(r: &Receipt, spec: &ReportSpec) -> Finished {
    let mut out = String::new();
    let label = spec.label();
    let (root, position) = root_at(r, spec.hand_id, spec.trick_no);
    let position = match spec.contract {
        Some(c) => with_contract(&position, c),
        None => position,
    };
    let coord = format!("{label} contract {}", position.bid);
    let oracle = SupportOracle;
    let field = FieldModel::new(field_spec());
    let belief = FactorBelief::uniform_root(&root, &position, &field);
    let z = oracle.mass(&belief);
    let actions = walt::solver::godgap::legal_actions(&root, &position);
    let viewer_leads = position.trick_plays.is_empty();
    let _ = writeln!(
        out,
        "== {coord} tail sigma0 | Z={z} | legal {{{}}} | viewer {} | node cap {NODE_CAP}{}",
        names(&actions),
        if viewer_leads { "leads" } else { "follows" },
        spec.anchor
            .map_or(String::new(), |a| format!(" | FH8 ANCHOR {a}"))
    );

    // Q_a: fresh, or the record's (cited). EVERY run below gets its own
    // `FieldModel` instance so no wall is warmed by another run's σ0 cache
    // (reads are exact regardless; wall is the approximate column): one for
    // the exact pricing, one per direct horizon, ONE for the whole ladder
    // (its passes share it — that warmth is the ladder's own reuse story,
    // as in FH2's record), one for the depth walks.
    let t0 = Instant::now();
    let (q, q_source): (Vec<(Domino, u128)>, String) = if spec.fresh_exact {
        let q_field = FieldModel::new(field_spec());
        let q_belief = FactorBelief::uniform_root(&root, &position, &q_field);
        let mut q = Vec::new();
        for a in &actions {
            let mut rs = ResponseStats::default();
            q.push((
                *a,
                response_success_mass(&oracle, &q_belief.focal_play(*a), &q_field, &mut rs),
            ));
        }
        (q, "fresh response_success_mass".to_string())
    } else {
        assert!(
            (spec.hand_id, spec.trick_no, position.bid) == (8, 3, 30) && z == RECORD_H8T3_Z,
            "the record's Q_a is cited at h8-t3 contract 30 only"
        );
        let q: Vec<(Domino, u128)> = RECORD_H8T3_Q
            .iter()
            .map(|(name, m)| (name.parse::<Domino>().expect("a tile name"), *m))
            .collect();
        assert_eq!(
            q.iter().map(|(a, _)| *a).collect::<Vec<_>>(),
            actions,
            "the record's actions are this root's legal set"
        );
        (
            q,
            "the RECORD (horizon_run1.txt h8-t3 cut 4, the 14-min exact solve; NOT recomputed)"
                .to_string(),
        )
    };
    let q_wall_us = t0.elapsed().as_micros();
    let qmax = q.iter().map(|(_, m)| *m).max().expect("a root action");
    let exact_set: Vec<Domino> = q
        .iter()
        .filter(|(_, m)| *m == qmax)
        .map(|(a, _)| *a)
        .collect();

    // The direct engine per k (reads without reuse).
    let mut direct: Vec<DirectRun> = Vec::new();
    if let Some(kmax) = spec.direct_kmax {
        for k in 0..=kmax {
            eprintln!("  {coord} direct k={k} ...");
            let spec_k = FocalSpec {
                horizon: k,
                node_fiber_cap: NODE_CAP,
            };
            let run_field = FieldModel::new(field_spec());
            let t0 = Instant::now();
            let result = focal_horizon(&oracle, &root, &position, &run_field, &run_field, &spec_k);
            direct.push(DirectRun {
                k,
                result,
                wall_us: t0.elapsed().as_micros(),
            });
        }
    }

    // One ladder, memo on, k = 0, 1, 2, ... (reads with reuse), on its
    // own field instance: cold at k = 0, warm after.
    let ladder_field = FieldModel::new(field_spec());
    let ctx = LadderContext {
        oracle: &oracle,
        root: &root,
        position: &position,
        lower_tail: &ladder_field,
        field: &ladder_field,
    };
    let mut ladder = FocalLadder::open(&ctx);
    let mut memo = SuffixMemo::new();
    let mut passes: Vec<LadderPass> = Vec::new();
    let mut extension_note = String::new();
    let mut k = 0usize;
    let mut kmax = spec.ladder_kmax;
    // The affordability extension fires at most ONCE (one horizon beyond
    // the declared kmax), never as a ladder that climbs while cheap.
    let mut extended = false;
    loop {
        if k > kmax {
            break;
        }
        eprintln!("  {coord} ladder k={k} ...");
        let t0 = Instant::now();
        let outcome = ladder.advance(
            &ctx,
            k,
            &WorkBudget {
                read_ceiling: u64::MAX,
                node_fiber_cap: NODE_CAP,
            },
            Some(&mut memo),
        );
        let wall_us = t0.elapsed().as_micros();
        passes.push(LadderPass {
            k,
            outcome,
            wall_us,
            facts: ladder.facts().len(),
            collapsed: ladder.collapsed_count(),
            receipts: memo.receipts,
            first_hit: memo.first_hit.clone(),
        });
        if k == kmax && !extended {
            if let Some(gate) = spec.ladder_extend_gate_us {
                extended = true;
                if wall_us < gate {
                    let _ = write!(
                        extension_note,
                        "k = {k} pass wall {wall_us}us < gate {gate}us → k = {} RUN; ",
                        k + 1
                    );
                    kmax += 1;
                } else {
                    let _ = write!(
                        extension_note,
                        "k = {k} pass wall {wall_us}us ≥ gate {gate}us → k = {} NOT run; ",
                        k + 1
                    );
                }
            }
        }
        k += 1;
    }

    // Uniform views per horizon for the laws; parity where both ran.
    let mut views: Vec<KView> = Vec::new();
    for p in &passes {
        let view = &p.outcome.report().view;
        match direct.iter().find(|d| d.k == p.k) {
            Some(DirectRun {
                result: Ok(res), ..
            }) => {
                check_ladder_parity(&coord, p.k, res, view);
                views.push(kview_direct(res));
            }
            _ => views.push(kview_ladder(p.k, view)),
        }
    }
    check_laws(&coord, &views, &q, &q_source);

    // Print per horizon.
    let mut rows = Vec::new();
    for (i, p) in passes.iter().enumerate() {
        let report = p.outcome.report();
        let view = &report.view;
        let d = direct.iter().find(|d| d.k == p.k);
        let direct_line = match d {
            Some(DirectRun {
                result: Ok(res),
                wall_us,
                ..
            }) => format!(
                "direct: reads {} (field {} + tail {}) | conditionings {} | focal {} hidden {} | \
                 tail evaluations {} (forced {}) | wall {wall_us}us",
                res.spend.field_reads + res.spend.tail_reads,
                res.spend.field_reads,
                res.spend.tail_reads,
                res.spend.conditionings,
                res.spend.focal_nodes,
                res.spend.hidden_nodes,
                res.spend.tail_consultations(),
                res.spend.forced_tail_evaluations,
            ),
            Some(DirectRun {
                result:
                    Err(FocalRefusal::UpperUnaffordable {
                        history,
                        fiber,
                        cap,
                    }),
                wall_us,
                ..
            }) => format!(
                "direct: REFUSED whole root at [{}] fiber {fiber} > cap {cap} | wall {wall_us}us",
                names(history)
            ),
            None => "direct: not run at this horizon (ladder only)".to_string(),
        };
        let (outcome_name, refused_count, refused_mass) = match &p.outcome {
            Outcome::Completed { .. } => ("COMPLETED".to_string(), 0usize, 0u128),
            Outcome::Interrupted {
                unaffordable,
                residual_frontier,
                ..
            } => (
                format!(
                    "INTERRUPTED (cap refusals {}, residual frontier {} nodes)",
                    unaffordable.len(),
                    residual_frontier.len()
                ),
                unaffordable.len(),
                unaffordable.iter().map(|(_, f)| *f).sum(),
            ),
        };
        let _ = writeln!(
            out,
            "-- k={} | {direct_line}\n   ladder pass: {outcome_name} | reads {} (field {} + tail {}) | \
             conditionings {} | suffix hits {} / lookups {} | receipts held {} | facts {} ({} collapsed; \
             new {} revisited {} tightened {}) | refused frontier {refused_count} nodes mass {refused_mass} | \
             wall {}us",
            p.k,
            report.reads_spent,
            report.spend.field_reads,
            report.spend.tail_reads,
            report.spend.conditionings,
            report.suffix_hits,
            report.suffix_lookups,
            p.receipts,
            p.facts,
            p.collapsed,
            report.facts_new,
            report.facts_revisited,
            report.facts_tightened,
            p.wall_us
        );
        let next = passes.get(i + 1).map(|n| &n.outcome.report().view);
        for a in &view.actions {
            let mark = if view.survivors.contains(&a.action) {
                "S"
            } else {
                "-"
            };
            let qa = q
                .iter()
                .find(|(t, _)| *t == a.action)
                .map(|(_, m)| *m)
                .expect("Q_a known");
            let upper = match a.upper_mass {
                Some(u) => format!(
                    "U {} | width {} | U−Q {}",
                    exact(&ratio(u, z)),
                    exact(&ratio(u - a.lower_mass, z)),
                    exact(&ratio(u - qa, z))
                ),
                None => "U absent (retained: no fact) | no interval".to_string(),
            };
            let deltas = match next.and_then(|n| n.action(a.action)) {
                Some(n) => {
                    let dl = exact(&ratio(n.lower_mass - a.lower_mass, z));
                    let du = match (a.upper_mass, n.upper_mass) {
                        (Some(u), Some(un)) => exact(&ratio(u - un, z)),
                        _ => "-".to_string(),
                    };
                    format!("Δ^L→k{} {dl} | Δ^U→k{} {du}", p.k + 1, p.k + 1)
                }
                None => "Δ^L - | Δ^U -".to_string(),
            };
            let _ = writeln!(
                out,
                "     action {} [{mark}]: L {} | {upper} | Q {} | Q−L {} | {deltas}",
                a.action,
                exact(&a.lower()),
                exact(&ratio(qa, z)),
                exact(&ratio(qa - a.lower_mass, z))
            );
        }
        let pi_line = match d {
            Some(DirectRun {
                result: Ok(res), ..
            }) if res.policy.id() != view.policy.id() => format!(
                "π_k {} ({} states; direct) | ladder π {} ({} states) — ids differ on a \
                 lower-side tie (prior wins, FH2), values equal",
                res.policy.id(),
                res.policy.states(),
                view.policy.id(),
                view.policy.states()
            ),
            _ => format!("π_k {} ({} states)", view.policy.id(), view.policy.states()),
        };
        let u_star = view
            .global_upper()
            .map_or("absent".to_string(), |u| exact(&u));
        let gamma = view
            .certified_regret
            .as_ref()
            .map_or("absent".to_string(), exact);
        let _ = writeln!(
            out,
            "   bar B_k {} (π_k plays {}) | survivors {{{}}} | verdict {}\n   {pi_line} | L_exec {} | U* {u_star} | Γ_k {gamma}",
            exact(&view.bar()),
            view.bar_action,
            names(&view.survivors),
            verdict_name(view),
            exact(&ratio(view.executable_lower_mass, z))
        );
        rows.push(ReportRow {
            label: label.clone(),
            contract: position.bid,
            k: p.k,
            z,
            survivors: names(&view.survivors),
            verdict: verdict_short(&view.verdict),
            plays: format!("{}", view.bar_action),
            bar: view.bar(),
            upper: view.global_upper(),
            regret: view.certified_regret.clone(),
            reads_direct: match d {
                Some(DirectRun {
                    result: Ok(res), ..
                }) => Some(res.spend.field_reads + res.spend.tail_reads),
                _ => None,
            },
            reads_ladder: report.reads_spent,
            hits: report.suffix_hits,
            facts: p.facts,
            wall_direct_us: d.map(|d| d.wall_us),
            wall_ladder_us: p.wall_us,
        });
    }

    // The exact column, the changes by horizon, the ply cut, the depth.
    let _ = writeln!(
        out,
        "   exact: Q_a by {q_source} | Q* {} | exact argmax {{{}}} | wall {q_wall_us}us",
        exact(&ratio(qmax, z)),
        names(&exact_set)
    );
    let plays: Vec<String> = views
        .iter()
        .map(|v| format!("k{}:{}", v.k, v.bar_action))
        .collect();
    let changes: Vec<String> = views
        .windows(2)
        .filter(|w| w[0].bar_action != w[1].bar_action)
        .map(|w| {
            format!(
                "k{}→k{} {}→{}",
                w[0].k, w[1].k, w[0].bar_action, w[1].bar_action
            )
        })
        .collect();
    let first_settled = views
        .iter()
        .find(|v| matches!(v.verdict, FocalVerdict::Settled { .. }))
        .map_or("none".to_string(), |v| format!("k={}", v.k));
    let first_equiv = views
        .iter()
        .find(|v| matches!(v.verdict, FocalVerdict::Equivalent { .. }))
        .map_or("none".to_string(), |v| format!("k={}", v.k));
    let gammas: Vec<String> = views
        .iter()
        .map(|v| {
            format!(
                "k{}:{}",
                v.k,
                v.regret
                    .as_ref()
                    .map_or("absent".to_string(), |g| format!("{}‰", permille(g)))
            )
        })
        .collect();
    let survivors_by_k: Vec<String> = views
        .iter()
        .map(|v| format!("k{}:{{{}}}", v.k, names(&v.survivors)))
        .collect();
    let _ = writeln!(
        out,
        "   by horizon: π_k plays [{}] | changes [{}] | survivors [{}] | Γ [{}] | first Settled {first_settled} | first Equivalent {first_equiv}",
        plays.join(" "),
        if changes.is_empty() {
            "none".to_string()
        } else {
            changes.join("; ")
        },
        survivors_by_k.join(" "),
        gammas.join(" ")
    );
    // The record's ply cuts (cut 4 and cut 8) at the viewer-lead trick-3/4
    // roots: Proposition FH-cut identifies them with U_{a,0} and U_{a,1}.
    if viewer_leads && spec.trick_no <= 4 {
        let mut cuts = Vec::new();
        for (m, cut) in [(0usize, 4usize), (1, 8)] {
            if let Some(v) = views.iter().find(|v| v.k == m) {
                if v.actions.iter().all(|(_, _, u)| u.is_some()) {
                    let masses: Vec<(Domino, u128)> = v
                        .actions
                        .iter()
                        .map(|(a, _, u)| (*a, u.expect("present")))
                        .collect();
                    let cut_argmax = lowest_tile_argmax(&masses);
                    let wrong = !exact_set.contains(&cut_argmax);
                    let certified_wrong = views.iter().any(|w| match &w.verdict {
                        FocalVerdict::Settled { action } => *action == cut_argmax && wrong,
                        _ => false,
                    });
                    cuts.push(format!(
                        "cut-{cut} (= argmax U_{{a,{m}}}, FH-cut) plays {cut_argmax}{} | ladder ever certifies it: {}",
                        if wrong {
                            " — FLIPS (not an exact maximizer)"
                        } else {
                            " — agrees with exact"
                        },
                        if certified_wrong { "YES — §41 FAILURE" } else { "no" }
                    ));
                }
            }
        }
        let _ = writeln!(out, "   ply cut: {}", cuts.join(" || "));
    }
    let depth_line = if spec.depth_walk {
        let depth_field = FieldModel::new(field_spec());
        let depth_belief = FactorBelief::uniform_root(&root, &position, &depth_field);
        let t0 = Instant::now();
        let depths: Vec<String> = actions
            .iter()
            .map(|a| {
                format!(
                    "{a}:{}",
                    focal_depth(&oracle, &depth_belief.focal_play(*a), &depth_field)
                )
            })
            .collect();
        format!(
            "h_f after action [{}] by the independent walk | wall {}us",
            depths.join(" "),
            t0.elapsed().as_micros()
        )
    } else {
        format!(
            "h_f after action = 7 − T = {} by FH-A6's law at an undecided viewer-lead root (NOT walked here: the walk is an exact-size pass)",
            7 - spec.trick_no
        )
    };
    let completed_k = passes.iter().map(|p| p.k).max().unwrap_or(0);
    let first_hit = passes
        .last()
        .and_then(|p| p.first_hit.as_ref())
        .map_or("none".to_string(), |h| format!("[{}]", names(h)));
    let _ = writeln!(
        out,
        "   completed focal depth k={completed_k} | {depth_line} | first suffix hit {first_hit}{}",
        if extension_note.is_empty() {
            String::new()
        } else {
            format!(" | affordability: {extension_note}")
        }
    );
    let _ = writeln!(
        out,
        "   §41 checks at this coordinate: containment L ≤ Q ≤ U ✓ | nesting (no lower fell, no upper rose) ✓ | survivors shrink ✓ | Γ never rose ✓ | ladder ≡ direct in value at every shared k ✓ | verdict vs exact argmax ✓\n"
    );
    (out, rows)
}

fn print_report_table(out: &mut String, rows: &[ReportRow]) {
    let _ = writeln!(
        out,
        "\n#### THE FOCAL-HORIZON TABLE OF RECORD — one row per (root, contract, k), σ0 tail ####\n"
    );
    let _ = writeln!(
        out,
        " root    | bid | k |    Z    | plays | survivors           | verdict                 | B_k ‰ | U* ‰ | Γ_k ‰ | reads direct | reads ladder | hits    | facts   | wall direct | wall ladder"
    );
    let _ = writeln!(
        out,
        "---------+-----+---+---------+-------+---------------------+-------------------------+-------+------+-------+--------------+--------------+---------+---------+-------------+------------"
    );
    for r in rows {
        let _ = writeln!(
            out,
            " {:<7} | {:>3} | {} | {:>7} | {:<5} | {:<19} | {:<23} | {:>5} | {:>4} | {:>5} | {:>12} | {:>12} | {:>7} | {:>7} | {:>9} | {}us",
            r.label,
            r.contract,
            r.k,
            r.z,
            r.plays,
            r.survivors,
            r.verdict,
            permille(&r.bar),
            r.upper.as_ref().map_or("-".to_string(), |u| format!("{}", permille(u))),
            r.regret
                .as_ref()
                .map_or("-".to_string(), |g| format!("{}", permille(g))),
            r.reads_direct
                .map_or("-".to_string(), |n| format!("{n}")),
            r.reads_ladder,
            r.hits,
            r.facts,
            r.wall_direct_us
                .map_or("-".to_string(), |w| format!("{w}us")),
            r.wall_ladder_us
        );
    }
}

/// The corpus of the record, then the h8-t3 anchor alone.
fn report_specs() -> (Vec<ReportSpec>, ReportSpec) {
    let mut specs = Vec::new();
    for (hand_id, trick_no) in T4_CORPUS {
        for contract in REPORT_T4_CONTRACTS {
            let anchor = match (hand_id, contract) {
                (8, Some(36)) | (8, Some(39)) => Some("(ii) h8-t4 trick-6 cut flip"),
                (4, _) => Some("(iii) h4-t4 contract-sensitive trick-5 specimen"),
                _ => None,
            };
            specs.push(ReportSpec {
                hand_id,
                trick_no,
                contract,
                direct_kmax: Some(REPORT_KMAX),
                ladder_kmax: REPORT_KMAX,
                ladder_extend_gate_us: None,
                fresh_exact: true,
                depth_walk: true,
                anchor,
            });
        }
    }
    for (hand_id, trick_no) in REPORT_T56 {
        for contract in REPORT_T56_CONTRACTS {
            specs.push(ReportSpec {
                hand_id,
                trick_no,
                contract,
                direct_kmax: Some(REPORT_KMAX),
                ladder_kmax: REPORT_KMAX,
                ladder_extend_gate_us: None,
                fresh_exact: true,
                depth_walk: true,
                anchor: None,
            });
        }
    }
    let anchor = ReportSpec {
        hand_id: 8,
        trick_no: 3,
        contract: None,
        direct_kmax: Some(H8T3_DIRECT_KMAX),
        ladder_kmax: 2,
        ladder_extend_gate_us: Some(H8T3_K3_WALL_GATE_US),
        fresh_exact: false,
        depth_walk: false,
        anchor: Some("(i) h8-t3 the exact trick-3 root"),
    };
    (specs, anchor)
}

fn report_of_record(r: &Receipt, path: &str, filters: &[String]) {
    let keep = |s: &ReportSpec| filters.is_empty() || filters.iter().any(|f| *f == s.label());
    let (specs, anchor) = report_specs();
    let run_anchor = keep(&anchor);
    // Dedupe a contract equal to the receipt's own.
    let mut seen: Vec<(usize, usize, u32)> = Vec::new();
    let specs: Vec<ReportSpec> = specs
        .into_iter()
        .filter(|s| {
            let (_, position) = root_at(r, s.hand_id, s.trick_no);
            let bid = s.contract.unwrap_or(position.bid);
            let key = (s.hand_id, s.trick_no, bid);
            if seen.contains(&key) {
                false
            } else {
                seen.push(key);
                true
            }
        })
        .filter(keep)
        .collect();
    let header = format!(
        "FOCAL-HORIZON HIERARCHY — THE REPORT OF RECORD (slice FH3) — EXPLORATORY\n\
         \n\
         The parent's §38 measurements at every (root, contract) of the corpus and the FH8 anchors, \
         horizons k ∈ {{0, 1, 2, 3}}: the direct engine per k (solver::focal_horizon, reads WITHOUT \
         reuse, FH1-comparable) and ONE ladder per coordinate (solver::focal_ladder) walked k = 0, 1, 2, 3 \
         with the suffix memo on (reads WITH reuse), uncapped ceiling, node fiber cap {NODE_CAP}. \
         Per action: L_{{a,k}}, U_{{a,k}}, U − L, Δ^L_{{a,k}} = L_{{a,k+1}} − L_{{a,k}}, \
         Δ^U_{{a,k}} = U_{{a,k}} − U_{{a,k+1}}, the survivor mark, Q_a (a fresh response_success_mass \
         where affordable; the record's values, cited, at h8-t3) and the split (U − Q) + (Q − L). \
         Per horizon: bar, survivors, verdict, π_k id (both ids where the ladder's differs on a tie), \
         L_exec, U*, Γ_k, the lower policy's root action; then the changes by horizon, the exact column, \
         the ply cut's argmax under Proposition FH-cut on viewer-lead roots (cut-4 = argmax U_{{a,0}}, \
         cut-8 = argmax U_{{a,1}}) and whether the ladder ever certifies it, the completed focal depth \
         and h_f after each action, the first suffix hit. The §41 laws are asserted at every coordinate; \
         a failure stops the record naming the coordinate. Never a theorem; never a play-strength claim.\n\
         \n\
         corpus: T4 {T4_CORPUS:?} × contracts {REPORT_T4_CONTRACTS:?} (None = the receipt's); \
         T56 {REPORT_T56:?} × {REPORT_T56_CONTRACTS:?}; anchor (i) h8-t3 receipt contract alone at the end\n\
         declared field: level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle\n\
         lower tail: σ0 driving the viewer seat (FH-A4); upper tail: the God line walk (FH-God)\n\
         tie rule for π_k: lowest tile index; the ladder's prior lower wins a tie (FH2)\n\
         forced focal nodes consume a unit of horizon (FH-A6); FH-last: trick-T roots collapse at k = 6 − T\n\
         h8-t3: direct engine at k ≤ {H8T3_DIRECT_KMAX}; ladder k = 0, 1, 2 and k = 3 only if the k = 2 pass \
         is under {H8T3_K3_WALL_GATE_US}us; Q_a cited from the record, never recomputed\n\
         corpus coordinates run on {REPORT_WORKERS} workers (walls are CONTENDED); the anchor runs alone\n\
         σ0 cache: every direct run and the exact pricing use a FRESH field instance (cold walls); \
         the ladder's passes share one instance (cold at k = 0, warm after — its own reuse)\n\
         filters: {}\n\
         wall is the only approximate number here\n",
        if filters.is_empty() {
            "none (the record)".to_string()
        } else {
            filters.join(" ")
        }
    );
    let slots: Mutex<Vec<Option<Finished>>> = Mutex::new((0..specs.len()).map(|_| None).collect());
    let flush = |slots: &[Option<Finished>], tail: Option<&str>| {
        let mut text = header.clone();
        for (spec, slot) in specs.iter().zip(slots) {
            match slot {
                Some((s, _)) => text.push_str(s),
                None => {
                    let _ = writeln!(
                        text,
                        "== {} contract {:?} | PENDING\n",
                        spec.label(),
                        spec.contract
                    );
                }
            }
        }
        if let Some(t) = tail {
            text.push_str(t);
        }
        let mut f = std::fs::File::create(path).expect("the output file opens");
        f.write_all(text.as_bytes())
            .expect("the output file writes");
        text
    };
    flush(&slots.lock().expect("the slots lock"), None);
    let next = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..REPORT_WORKERS.min(specs.len().max(1)) {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= specs.len() {
                    break;
                }
                let done = report_coordinate(r, &specs[i]);
                let mut guard = slots.lock().expect("the slots lock");
                guard[i] = Some(done);
                flush(&guard, None);
            });
        }
    });
    let mut tail = String::new();
    let mut rows: Vec<ReportRow> = Vec::new();
    {
        let guard = slots.lock().expect("the slots lock");
        for slot in guard.iter() {
            rows.extend(
                slot.as_ref()
                    .expect("every coordinate completed")
                    .1
                    .iter()
                    .cloned(),
            );
        }
    }
    if run_anchor {
        let _ = writeln!(
            tail,
            "#### ANCHOR (i) — h8-t3, run alone after the corpus ####\n"
        );
        flush(&slots.lock().expect("the slots lock"), Some(&tail));
        let (s, anchor_rows) = report_coordinate(r, &anchor);
        tail.push_str(&s);
        rows.extend(anchor_rows);
    }
    print_report_table(&mut tail, &rows);
    let anchor_labels = [
        ("h8-t3", 30u32),
        ("h8-t4", 36),
        ("h8-t4", 39),
        ("h4-t4", 30),
        ("h4-t4", 33),
        ("h4-t4", 36),
        ("h4-t4", 39),
        ("h4-t4", 42),
    ];
    let _ = writeln!(
        tail,
        "\n#### THE FH8 ANCHORS — verdict, Γ_k and reads by k (direct / ladder with reuse) ####\n"
    );
    for (label, bid) in anchor_labels {
        let mine: Vec<&ReportRow> = rows
            .iter()
            .filter(|r| r.label == label && r.contract == bid)
            .collect();
        if mine.is_empty() {
            continue;
        }
        let cells: Vec<String> = mine
            .iter()
            .map(|r| {
                format!(
                    "k{}: {} Γ {} reads {}/{}",
                    r.k,
                    r.verdict,
                    r.regret
                        .as_ref()
                        .map_or("-".to_string(), |g| format!("{}‰", permille(g))),
                    r.reads_direct.map_or("-".to_string(), |n| format!("{n}")),
                    r.reads_ladder
                )
            })
            .collect();
        let first = mine
            .iter()
            .find(|r| r.verdict.starts_with("SETTLED") || r.verdict.starts_with("EQUIV"))
            .map_or("none".to_string(), |r| format!("k={} ({})", r.k, r.verdict));
        let _ = writeln!(
            tail,
            " {label} bid {bid}: {} | smallest k that settles or ties exactly: {first}",
            cells.join(" | ")
        );
    }
    let _ = writeln!(
        tail,
        "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts."
    );
    let text = flush(&slots.lock().expect("the slots lock"), Some(&tail));
    println!("{text}");
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
            let k: usize = args[4].parse().expect("a horizon");
            let contract: Option<u32> = args.get(5).map(|s| s.parse().expect("a contract"));
            let cap: u128 = args
                .get(6)
                .map_or(NODE_CAP, |s| s.parse().expect("a node cap"));
            let tail_name = args.get(7).map_or("sigma0", String::as_str);
            let tail = Tail::parse(tail_name);
            let price_exact = args.get(8).is_some_and(|s| s == "exact");
            let run = run(
                &r,
                &Coordinate {
                    hand_id,
                    trick_no,
                    contract,
                    k,
                    cap,
                    tail,
                    price_exact,
                },
            );
            let mut out = String::new();
            print_run(&mut out, &run, tail_name);
            print!("{out}");
        }
        "scout-corpus" => {
            let path = args.get(2).expect("an output path").clone();
            let mut out = String::new();
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            let _ = writeln!(
                out,
                "FOCAL-HORIZON HIERARCHY SCOUT (slice FH1) — EXPLORATORY\n\
                 \n\
                 The parent's §28 engine at the four trick-4 gated roots, receipt contract, \
                 horizons k ∈ {CORPUS_HORIZONS:?}, σ0 driving the viewer seat as the lower tail \
                 (FH-A4) and the world-revealed God continuation as the upper tail (FH-God). \
                 Every Q_a priced by response_success_mass so the per-action width splits into \
                 the fusion price (U − Q) and the policy gap (Q − L). Never a theorem; never a \
                 play-strength claim.\n\
                 \n\
                 declared field: level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle\n\
                 node fiber cap: {NODE_CAP}\n\
                 tie rule for π_k: lowest tile index\n\
                 forced focal nodes consume a unit of horizon (FH-A6)\n\
                 wall is the only approximate number here\n"
            );
            flush(&out);
            let mut rows: Vec<Row> = Vec::new();
            for (hand_id, trick_no) in T4_CORPUS {
                for k in CORPUS_HORIZONS {
                    eprintln!("  h{hand_id}-t{trick_no} receipt contract k={k} ...");
                    let run = run(
                        &r,
                        &Coordinate {
                            hand_id,
                            trick_no,
                            contract: None,
                            k,
                            cap: NODE_CAP,
                            tail: Tail::Sigma0,
                            price_exact: true,
                        },
                    );
                    print_run(&mut out, &run, "sigma0");
                    if let Some(row) = summarize(&run) {
                        rows.push(row);
                    }
                    flush(&out);
                }
            }
            print_table(&mut out, &rows);
            let _ = writeln!(
                out,
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts."
            );
            flush(&out);
            println!("{out}");
        }
        "ladder" => {
            let hand_id: usize = args[2].parse().expect("a hand id");
            let trick_no: usize = args[3].parse().expect("a trick number");
            let contract: Option<u32> = match args[4].as_str() {
                "receipt" => None,
                c => Some(c.parse().expect("a contract or 'receipt'")),
            };
            let use_memo = !args[5..].iter().any(|s| s == "nomemo");
            let cap: u128 = args[5..]
                .iter()
                .find_map(|s| s.strip_prefix("cap="))
                .map_or(NODE_CAP, |c| c.parse().expect("a node fiber cap"));
            let schedule: Vec<Step> = args[5..]
                .iter()
                .filter(|s| *s != "nomemo" && !s.starts_with("cap="))
                .map(|s| parse_step(s))
                .collect();
            assert!(!schedule.is_empty(), "a schedule holds a step");
            let mut out = String::new();
            let rows = run_ladder(
                &mut out,
                &r,
                &LadderRun {
                    hand_id,
                    trick_no,
                    contract,
                    cap,
                    use_memo,
                },
                &schedule,
            );
            print_ladder_table(&mut out, &rows);
            print!("{out}");
        }
        "ladder-record" => {
            let path = args.get(2).expect("an output path").clone();
            let mut out = String::new();
            let flush = |s: &str| {
                let mut f = std::fs::File::create(&path).expect("the output file opens");
                f.write_all(s.as_bytes()).expect("the output file writes");
            };
            let _ = writeln!(
                out,
                "FOCAL-HORIZON LADDER RECORD (slice FH2) — EXPLORATORY\n\
                 \n\
                 One focal ladder per root (solver::focal_ladder) through a PINNED schedule of \
                 k:ceiling passes — ceiling = field + tail reads for that pass, inf = no ceiling — \
                 with the suffix memo on. Facts are installed by intersection at completed nodes \
                 only (Proposition FH-int); an interrupted pass retains every prior fact; a resume \
                 is the same horizon again with more budget. Every root view below is DERIVED from \
                 the fact store. Never a theorem; never a play-strength claim.\n\
                 \n\
                 declared field: level0-modeled-mind-v1 (Level0 n0=2) under SupportOracle\n\
                 lower tail: σ0 driving the viewer seat (FH-A4); upper tail: the God line walk (FH-God)\n\
                 node fiber cap: {NODE_CAP}\n\
                 tie rule for π: lowest tile index; a prior lower's policy survives a tie\n\
                 schedules: h8-t4 [{}]; h3-t4 [{}]\n\
                 wall is the only approximate number here\n",
                LADDER_RECORD[0].2,
                LADDER_RECORD[1].2
            );
            flush(&out);
            let mut rows = Vec::new();
            for (hand_id, trick_no, schedule) in LADDER_RECORD {
                let steps: Vec<Step> = schedule.split(' ').map(parse_step).collect();
                rows.extend(run_ladder(
                    &mut out,
                    &r,
                    &LadderRun {
                        hand_id,
                        trick_no,
                        contract: None,
                        cap: NODE_CAP,
                        use_memo: true,
                    },
                    &steps,
                ));
                flush(&out);
            }
            print_ladder_table(&mut out, &rows);
            let _ = writeln!(
                out,
                "\nEXPLORATORY — below every evidentiary tier; quotable only via gate receipts."
            );
            flush(&out);
            println!("{out}");
        }
        "report" => {
            let path = args.get(2).expect("an output path").clone();
            let filters: Vec<String> = args[3..].to_vec();
            report_of_record(&r, &path, &filters);
        }
        _ => {
            eprintln!(
                "usage: focalreport scout <hand> <trick> <k> [contract] [node-cap] [sigma0|lowest] [exact] | \
                 focalreport scout-corpus <out.txt> | \
                 focalreport ladder <hand> <trick> <contract|receipt> <k:ceiling>... | \
                 focalreport ladder-record <out.txt> | \
                 focalreport report <out.txt> [h<hand>-t<trick> ...]"
            );
            std::process::exit(2);
        }
    }
}
