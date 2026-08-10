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
//! Everything exploratory tier; exact rationals; deterministic.

use walt_core::receipt::Receipt;
use walt_core::{Domino, Seat};
use walt_geom::Q;
use walt_kernel::ReceiptDecision;
use walt_strat::{ScalarHidden, ScalarValuation};

use crate::basin::BasinDomain;
use crate::generalize::lesson_applies;
use crate::lesson::{Lesson, LessonVerdict};

/// One matched decision's H re-measurement.
#[derive(Clone, Debug)]
pub struct HDecision {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    pub ply: usize,
    pub fiber: usize,
    pub outcome: HOutcome,
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

/// Re-measures one lesson's matched decisions at (H, fixed-uniform-legal).
/// `None` for the checker verdict (not a value comparison).
pub fn remeasure_at_h(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
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
        let Some(values) = solver.action_values(&hands, rd.leader, &rd.prefix, &mut budget) else {
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
    })
}

/// Deterministic rendering: one line per matched decision plus the
/// survival summary.
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
        out.push('\n');
    }
    let (measured, held, failed, nfv, capped) = report.counts();
    let survival = match report.survives() {
        Some(true) => "SURVIVES at (H, fixed-uniform-legal) on the measured subdomain",
        Some(false) => "FAILS at (H, fixed-uniform-legal)",
        None => "UNMEASURED (empty basin, all inapplicable, or all capped)",
    };
    out.push_str(&format!(
        "  H summary: measured {measured} held {held} failed {failed} not-fiber-valid {nfv} capped {capped} -> {survival}\n"
    ));
    out
}
