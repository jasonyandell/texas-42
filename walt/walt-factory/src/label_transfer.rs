//! (H, fixed-uniform-legal) re-measurement of a lesson's basin (S5c-m2).
//!
//! Label transfer is the weakest load-bearing assumption in the inventory
//! (§12.4: basins are label-relative — a basin measured at one operator
//! pair can shatter or merge at another), so BEFORE any database economy
//! the existing lessons are re-measured at the seat-facing label. Both
//! coordinates move: (C, minimax-omniscient) -> (H, fixed-uniform-legal)
//! — pooled-information focal optimization against the §7.4 fixed
//! uniform-legal chance law — while the root weighting stays the declared
//! uniform-over-fiber, so what is tested is exactly the inventory's
//! transfer to the pair the seat actually faces.
//!
//! At H the verdict quantifier changes shape BY NECESSITY: H values live
//! on pooled information states, so a re-measured refutation is ONE
//! inequality `Q^H(better) >= Q^H(worse)` per matching decision, and a
//! re-measured win is `Q^H(action) = max_a Q^H(a)` per matching decision;
//! atom cells are read fiber-valid (the checker verdict's quantifier
//! placement, which the lesson types already carry). A matched decision
//! whose atom cells are not fiber-valid is H-inapplicable — the lesson's
//! H-shaped claim does not extend there; one over the particle-step
//! budget is H-capped — excluded, never sampled. The checker verdict is
//! not re-measured: §12.6 already lives at the fixed field and is not a
//! value comparison.
//!
//! S5c-m3 adds the `dag-v1` budget semantics (`remeasure_at_h_dag`): the
//! same measurement over walt-strat's memoized DAG, so the m2
//! budget-capped big fibers become measurable at the seat-facing label.
//! Whether a lesson is measured or capped stays a deterministic function
//! of declared inputs — the cache is scoped to one measurement call, and
//! the unit (not the work) is what changed. Rows and results headers
//! carry the semantics identifier; a lesson capped at tree-v0 and
//! measured at dag-v1 is a semantics change, never the same statistic
//! improving.
//!
//! Everything exploratory tier; exact rationals; deterministic.

use walt_core::receipt::Receipt;
use walt_core::{Domino, Seat};
use walt_geom::Q;
use walt_kernel::ReceiptDecision;
use walt_strat::{MemoStats, ScalarHidden, ScalarValuation};

use crate::basin::BasinDomain;
use crate::generalize::lesson_applies;
use crate::lesson::{Lesson, LessonVerdict};

/// The declared particle-step budget unit (S5c-m3, walt-math ruling):
/// measurability is a deterministic function of (solver version, budget
/// semantics, budget value, root inputs) — never of run history. A lesson
/// capped under one semantics and measured under another is a SEMANTICS
/// CHANGE, recorded as such, never the same statistic improving.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BudgetSemantics {
    /// One unit per (particle, node) visit of the observation tree — the
    /// S5c-m2 unit.
    TreeV0,
    /// One unit per (particle, node) visit actually computed on the
    /// memoized DAG; a pooled-state cache hit costs zero by unit
    /// definition (`hidden_scalar` module doc). Cache scope is one
    /// measurement call — incidental warmth cannot exist.
    DagV1,
}

impl BudgetSemantics {
    pub fn identifier(self) -> &'static str {
        match self {
            BudgetSemantics::TreeV0 => "tree-v0",
            BudgetSemantics::DagV1 => "dag-v1",
        }
    }
}

/// One matched decision's H re-measurement.
#[derive(Clone, Debug)]
pub struct HDecision {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    pub ply: usize,
    pub fiber: usize,
    pub outcome: HOutcome,
    /// `dag-v1` cache statistics for this decision's measurement call
    /// (`None` under tree semantics) — provenance, never verdict input.
    pub memo: Option<MemoStats>,
}

#[derive(Clone, Debug)]
pub enum HOutcome {
    /// The refutation inequality at this decision, with both exact values.
    Refutation {
        worse: (Domino, Q),
        better: (Domino, Q),
        holds: bool,
    },
    /// The win equality at this decision: the selector action's value and
    /// the decision's optimum.
    Win {
        action: (Domino, Q),
        best: Q,
        holds: bool,
    },
    /// Atom cells not fiber-valid here: the H-shaped claim does not apply
    /// at this decision (its (C)-basin membership rested on a strict
    /// sub-fiber).
    NotFiberValid,
    /// Particle-step budget exceeded: excluded from the H measurement,
    /// never sampled.
    Capped,
}

/// The re-measurement of one lesson's whole basin.
#[derive(Clone, Debug)]
pub struct HReport {
    pub decisions: Vec<HDecision>,
    pub budget_per_decision: u64,
    pub semantics: BudgetSemantics,
}

impl HReport {
    /// (measured, held, failed, not-fiber-valid, capped).
    pub fn counts(&self) -> (usize, usize, usize, usize, usize) {
        let mut measured = 0;
        let mut held = 0;
        let mut failed = 0;
        let mut nfv = 0;
        let mut capped = 0;
        for d in &self.decisions {
            match &d.outcome {
                HOutcome::Refutation { holds, .. } | HOutcome::Win { holds, .. } => {
                    measured += 1;
                    if *holds {
                        held += 1;
                    } else {
                        failed += 1;
                    }
                }
                HOutcome::NotFiberValid => nfv += 1,
                HOutcome::Capped => capped += 1,
            }
        }
        (measured, held, failed, nfv, capped)
    }

    /// `Some(true)` when every measured decision holds (and at least one
    /// was measured); `Some(false)` when any measured decision fails;
    /// `None` when nothing was measurable.
    pub fn survives(&self) -> Option<bool> {
        let (measured, _, failed, _, _) = self.counts();
        if measured == 0 {
            return None;
        }
        Some(failed == 0)
    }
}

/// Re-measures one lesson's matched decisions at (H, fixed-uniform-legal)
/// under the S5c-m2 tree-v0 budget semantics (the unmemoized walk — the
/// value-transparency reference). `None` for the checker verdict (not a
/// value comparison).
pub fn remeasure_at_h(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
) -> Option<HReport> {
    remeasure(
        lesson,
        domain,
        receipt,
        budget_per_decision,
        BudgetSemantics::TreeV0,
    )
}

/// The same re-measurement under `dag-v1` budget semantics: the memoized
/// solver, one fresh cache per decision's measurement call. Values are
/// byte-identical to tree-v0 where both measure (CI-asserted); only
/// measurability can differ, and that difference is a semantics change.
pub fn remeasure_at_h_dag(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
) -> Option<HReport> {
    remeasure(
        lesson,
        domain,
        receipt,
        budget_per_decision,
        BudgetSemantics::DagV1,
    )
}

fn remeasure(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
    semantics: BudgetSemantics,
) -> Option<HReport> {
    if matches!(lesson.verdict, LessonVerdict::NotLumpable { .. }) {
        return None;
    }
    let mut decisions = Vec::new();
    for m in &lesson.basin.matched {
        let d = domain
            .decisions
            .iter()
            .find(|d| {
                d.hand == m.hand && d.seat == m.seat && d.trick_no == m.trick_no && d.ply == m.ply
            })
            .expect("a matched decision is a domain decision");
        let mut entry = HDecision {
            hand: m.hand,
            seat: m.seat,
            trick_no: m.trick_no,
            ply: m.ply,
            fiber: d.worlds.len(),
            outcome: HOutcome::Capped,
            memo: None,
        };
        // Fiber-validity through the gated application path: the H
        // quantifier applies the implicant per decision, so every fiber
        // world must satisfy the atom cells.
        let matched = lesson_applies(lesson, d).expect("a basin member applies");
        if matched.len() != d.worlds.len() {
            entry.outcome = HOutcome::NotFiberValid;
            decisions.push(entry);
            continue;
        }
        let rd = ReceiptDecision::at(&receipt.hands[m.hand], m.trick_no, m.seat)
            .expect("the decision reconstructs");
        let solver = ScalarHidden::new(
            d.kernel.decl(),
            d.kernel.viewer(),
            d.kernel.viewer().team(),
            ScalarValuation::trick_plus_count(),
        );
        let hands: Vec<[walt_core::DominoSet; Seat::COUNT]> =
            d.worlds.iter().map(|w| w.hands()).collect();
        let mut budget = budget_per_decision;
        let values = match semantics {
            BudgetSemantics::TreeV0 => {
                solver.action_values(&hands, rd.leader, &rd.prefix, &mut budget)
            }
            BudgetSemantics::DagV1 => {
                let (values, stats) =
                    solver.action_values_dag(&hands, rd.leader, &rd.prefix, &mut budget);
                entry.memo = Some(stats);
                values
            }
        };
        let Some(values) = values else {
            decisions.push(entry);
            continue;
        };
        let value_of = |tile: Domino| -> Q {
            values
                .iter()
                .find(|(a, _)| *a == tile)
                .expect("selector actions are legal")
                .1
        };
        entry.outcome = match &lesson.verdict {
            LessonVerdict::Refutation { worse, better } => {
                let w_tile = worse.resolve(&d.actions, d.decisive).expect("eligible");
                let b_tile = better.resolve(&d.actions, d.decisive).expect("eligible");
                let (qw, qb) = (value_of(w_tile), value_of(b_tile));
                HOutcome::Refutation {
                    worse: (w_tile, qw),
                    better: (b_tile, qb),
                    holds: qb >= qw,
                }
            }
            LessonVerdict::Win { action } => {
                let a_tile = action.resolve(&d.actions, d.decisive).expect("eligible");
                let qa = value_of(a_tile);
                let best = values.iter().map(|(_, v)| *v).max().expect("actions");
                HOutcome::Win {
                    action: (a_tile, qa),
                    best,
                    holds: qa == best,
                }
            }
            LessonVerdict::NotLumpable { .. } => unreachable!("filtered above"),
        };
        decisions.push(entry);
    }
    Some(HReport {
        decisions,
        budget_per_decision,
        semantics,
    })
}

/// Deterministic rendering: one line per matched decision plus the
/// survival summary. Tree-v0 rows render exactly as in S5c-m2 (they are
/// pinned); dag-v1 rows each carry the budget-semantics identifier and
/// the decision's cache statistics.
pub fn render_h_report(report: &HReport) -> String {
    let mut out = String::new();
    for d in &report.decisions {
        let line = match &d.outcome {
            HOutcome::Refutation {
                worse,
                better,
                holds,
            } => format!(
                "  h{} {} t{} p{} fiber {}: Q^H({})={} Q^H({})={} -> {}",
                d.hand,
                d.seat,
                d.trick_no,
                d.ply,
                d.fiber,
                better.0,
                better.1,
                worse.0,
                worse.1,
                if *holds { "HOLDS" } else { "FAILS" }
            ),
            HOutcome::Win {
                action,
                best,
                holds,
            } => format!(
                "  h{} {} t{} p{} fiber {}: Q^H({})={} best={} -> {}",
                d.hand,
                d.seat,
                d.trick_no,
                d.ply,
                d.fiber,
                action.0,
                action.1,
                best,
                if *holds { "HOLDS" } else { "FAILS" }
            ),
            HOutcome::NotFiberValid => format!(
                "  h{} {} t{} p{} fiber {}: NOT-FIBER-VALID (H claim inapplicable)",
                d.hand, d.seat, d.trick_no, d.ply, d.fiber
            ),
            HOutcome::Capped => format!(
                "  h{} {} t{} p{} fiber {}: CAPPED at budget {}",
                d.hand, d.seat, d.trick_no, d.ply, d.fiber, report.budget_per_decision
            ),
        };
        out.push_str(&line);
        if report.semantics == BudgetSemantics::DagV1 {
            match &d.memo {
                Some(s) => {
                    // On a capped call the tree-equivalent count only
                    // covers completed root actions, so render the best
                    // available lower bound (a dag walk never charges
                    // more than the tree walk it replaces).
                    let (bound, tree) = if matches!(d.outcome, HOutcome::Capped) {
                        (">=", s.tree_steps.max(u128::from(s.steps)))
                    } else {
                        ("=", s.tree_steps)
                    };
                    out.push_str(&format!(
                        " [semantics=dag-v1 steps={} tree-equiv{bound}{tree} entries={} hits={} key-particles={}]",
                        s.steps, s.entries, s.hits, s.key_particles
                    ));
                }
                None => out.push_str(" [semantics=dag-v1]"),
            }
        }
        out.push('\n');
    }
    let (measured, held, failed, nfv, capped) = report.counts();
    let survival = match report.survives() {
        Some(true) => "SURVIVES at (H, fixed-uniform-legal) on the measured subdomain",
        Some(false) => "FAILS at (H, fixed-uniform-legal)",
        None => "UNMEASURED (empty basin, all inapplicable, or all capped)",
    };
    let tag = match report.semantics {
        BudgetSemantics::TreeV0 => "",
        BudgetSemantics::DagV1 => " [semantics=dag-v1]",
    };
    out.push_str(&format!(
        "  H summary: measured {measured} held {held} failed {failed} not-fiber-valid {nfv} capped {capped} -> {survival}{tag}\n"
    ));
    out
}
