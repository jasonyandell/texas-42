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
//!   `focalreport ladder <hand> <trick> <contract|receipt> [nomemo] <k:ceiling>...`
//!   `focalreport ladder-record <out.txt>`
//!
//! `tail` is `sigma0` (default) or `lowest`; a trailing `exact` also
//! prices every `Q_a` by `response_success_mass` and prints the
//! per-action split `(U − Q) + (Q − L)` (walt-math W3). The
//! report-of-record mode is slice FH3's; the binary stays open for it.
//!
//! `ladder` (slice FH2, `solver::focal_ladder`) runs ONE ladder through
//! a schedule of `k:ceiling` steps (ceiling = field + tail reads for that
//! pass; `k:inf` for no ceiling) with the suffix memo on (`nomemo` turns
//! it off — the memory and read comparison), printing per
//! step the outcome, reads, residual frontier, fact-store movement,
//! suffix hits and the derived root view. `ladder-record` is the record
//! of record over h8-t4 and h3-t4 at the receipt contract with the
//! pinned schedules stated in its header.
//!
//! No floats anywhere; wall time is integer microseconds and the one
//! approximate number.

use std::fmt::Write as _;
use std::io::Write as _;
use std::time::Instant;

use num_bigint::BigInt;
use num_rational::BigRational;
use walt::kernel::Kernel;
use walt::rules::receipt::{locate_verify_player, parse_file, Receipt};
use walt::solver::adaptive::{CanonicalRoot, FixedPreference, RootPosition, SlicePolicy};
use walt::solver::factor_belief::{
    response_success_mass, FactorBelief, ResponseStats, SupportOracle,
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
            let schedule: Vec<Step> = args[5..]
                .iter()
                .filter(|s| *s != "nomemo")
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
                    cap: NODE_CAP,
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
        _ => {
            eprintln!(
                "usage: focalreport scout <hand> <trick> <k> [contract] [node-cap] [sigma0|lowest] [exact] | \
                 focalreport scout-corpus <out.txt> | \
                 focalreport ladder <hand> <trick> <contract|receipt> <k:ceiling>... | \
                 focalreport ladder-record <out.txt>"
            );
            std::process::exit(2);
        }
    }
}
