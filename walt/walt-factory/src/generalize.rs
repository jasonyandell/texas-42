//! The generalizer (S5b): from one conflict to a lesson, by greedy
//! constraint-dropping with witness-terminated widening.
//!
//! 1UIP culture, not minimal-core extraction: the initial implicant is the
//! origin's full vocabulary description (identity, public frame, and every
//! atom cell constant across the origin fiber); the generalizer then walks
//! a **declared drop order**, tentatively removing one cell at a time and
//! re-verifying the verdict exhaustively over the whole basin domain. A
//! verified widening keeps the cell dropped; the first counterexample ends
//! that widening, restores the cell, and names it load-bearing — the
//! witness is recorded whole. Two fixed orders are tried (the forward
//! order and its reverse — the restart policy, cheap and deterministic)
//! and the lesson with the larger basin is kept (ties to forward).
//!
//! Drop orders, declared: forward = transcript identity (`Hand`, `Seat`)
//! first, then the atom cells in vocabulary order, then the public frame
//! (`Decl`, `Ply`, `Role`, `Horizon`) last; reverse is its mirror. The
//! forward order widens identity early (the in-domain origin's natural
//! cut); the reverse order lifts the frame first, which is what lets an
//! out-of-domain origin (a trick-3/4 conflict against a trick-5/6 domain)
//! reach the domain at all — until its `Horizon` cell drops, nothing
//! matches and every drop is vacuously verified. Vacuous verification is
//! allowed and honest: the basin report says what actually matched.
//!
//! Verification is per the verdict's quantifier (`lesson` module doc):
//! pair verdicts world-by-world over precomputed exact PI values, the
//! checker verdict decision-by-decision through the exhaustive §12.6
//! checker. Nothing is sampled; no seeds exist here.

use std::collections::BTreeMap;

use walt_core::receipt::ReceiptHand;
use walt_core::Seat;
use walt_kernel::{Kernel, ReceiptDecision};
use walt_skeleton::{check_lumpability, AtomDescriptor, Exp3aContext, KernelTree};
use walt_strat::{OperatorLabel, ScalarPi, ScalarValuation};

use crate::basin::{eval_atom, valued_tile, vocabulary, BasinDomain, DomainDecision};
use crate::lesson::{
    ActionSelector, AtomValue, BasinReport, Constraint, DescriptorFamily, DropOutcome, DropStep,
    Implicant, Lesson, LessonGrade, LessonOrigin, LessonVerdict, MatchedDecision, Role,
    WideningWitness,
};
use crate::walker::DecisionRecord;

/// Basin statistics accumulated by one successful verification pass.
struct Stats {
    decisions_matched: usize,
    worlds_matched: usize,
    strict_worlds: usize,
    matched: Vec<MatchedDecision>,
}

impl Stats {
    fn empty() -> Stats {
        Stats {
            decisions_matched: 0,
            worlds_matched: 0,
            strict_worlds: 0,
            matched: Vec::new(),
        }
    }
}

fn role_of(rh: &ReceiptHand, seat: Seat) -> Role {
    if rh.declaring_team == seat.team() {
        Role::Declaring
    } else {
        Role::Defending
    }
}

/// Do the decision-sort cells hold at this domain decision?
fn decision_cells_hold(d: &DomainDecision, cells: &[Constraint]) -> bool {
    cells.iter().all(|c| match c {
        Constraint::Hand(h) => *h == d.hand,
        Constraint::Seat(s) => *s == d.seat,
        Constraint::Decl(x) => *x == d.kernel.decl(),
        Constraint::Role(r) => *r == d.role,
        Constraint::Horizon(n) => *n == d.horizon,
        Constraint::Ply(p) => *p == d.ply,
        Constraint::Atom(..) => true,
    })
}

/// Does world `widx` of this decision satisfy every atom cell?
fn world_matches(d: &DomainDecision, cells: &[Constraint], widx: usize) -> bool {
    cells.iter().all(|c| match c {
        Constraint::Atom(atom, v) => d.column(*atom).is_some_and(|col| col[widx] == Some(*v)),
        _ => true,
    })
}

/// Do the atom cells hold at *every* world of this decision (the
/// fiber-valid reading used by decision-graded verdicts)?
fn atom_cells_fiber_valid(d: &DomainDecision, cells: &[Constraint]) -> bool {
    (0..d.worlds.len()).all(|w| world_matches(d, cells, w))
}

/// One exhaustive verification of `verdict` under `cells` over the whole
/// domain. `Ok` carries the basin statistics of everything that matched;
/// `Err` carries the first counterexample in domain order.
fn verify(
    domain: &BasinDomain,
    cells: &[Constraint],
    verdict: &LessonVerdict,
    trees: &mut BTreeMap<usize, KernelTree>,
) -> Result<Stats, WideningWitness> {
    let mut stats = Stats::empty();
    for (di, d) in domain.decisions.iter().enumerate() {
        if !decision_cells_hold(d, cells) {
            continue;
        }
        match verdict {
            LessonVerdict::Refutation { .. } | LessonVerdict::Win { .. } => {
                let (worse, better) = match verdict {
                    LessonVerdict::Refutation { worse, better } => (Some(*worse), *better),
                    LessonVerdict::Win { action } => (None, *action),
                    LessonVerdict::NotLumpable { .. } => unreachable!("outer arm"),
                };
                let Some(b_tile) = better.resolve(&d.actions, d.decisive) else {
                    continue;
                };
                let bi = d.actions.iter().position(|a| *a == b_tile).expect("legal");
                // `None` marks the win form; a refutation needs the worse
                // selector resolved to a *distinct* legal action.
                let wi = match worse {
                    None => None,
                    Some(sel) => match sel.resolve(&d.actions, d.decisive) {
                        Some(t) if t != b_tile => {
                            Some(d.actions.iter().position(|a| *a == t).expect("legal"))
                        }
                        _ => continue,
                    },
                };
                let mut wm = 0usize;
                let mut strict = 0usize;
                for (widx, world) in d.worlds.iter().enumerate() {
                    if !world_matches(d, cells, widx) {
                        continue;
                    }
                    wm += 1;
                    let row = &d.values[widx];
                    let ok = match wi {
                        None => row[bi] == *row.iter().max().expect("actions"),
                        Some(wi) => row[bi] >= row[wi],
                    };
                    if !ok {
                        return Err(WideningWitness::World {
                            hand: d.hand,
                            seat: d.seat,
                            trick_no: d.trick_no,
                            ply: d.ply,
                            world: *world,
                            values: d.actions.iter().copied().zip(row.iter().copied()).collect(),
                        });
                    }
                    if let Some(wi) = wi {
                        if row[bi] > row[wi] {
                            strict += 1;
                        }
                    }
                }
                if wm > 0 {
                    stats.decisions_matched += 1;
                    stats.worlds_matched += wm;
                    stats.strict_worlds += strict;
                    stats.matched.push(MatchedDecision {
                        hand: d.hand,
                        seat: d.seat,
                        trick_no: d.trick_no,
                        ply: d.ply,
                        worlds_matched: wm,
                        worlds_total: d.worlds.len(),
                        strict_worlds: strict,
                    });
                }
            }
            LessonVerdict::NotLumpable { descriptor } => {
                // The §12.6 carrier starts at a viewer-lead trick boundary,
                // and tree cost explodes below horizon 2 — the checker
                // verdict's declared applicability.
                if d.ply != 0 || d.horizon > 2 {
                    continue;
                }
                if !atom_cells_fiber_valid(d, cells) {
                    continue;
                }
                let tree = trees.entry(di).or_insert_with(|| {
                    KernelTree::build(&d.kernel, ScalarValuation::trick_plus_count())
                });
                let skeleton = match descriptor {
                    DescriptorFamily::Chassis => AtomDescriptor::new(&d.kernel, true, Vec::new()),
                };
                let report = check_lumpability(&d.kernel, tree, &skeleton);
                if report.failure.is_none() {
                    return Err(WideningWitness::Lumpable {
                        hand: d.hand,
                        seat: d.seat,
                        trick_no: d.trick_no,
                        ply: d.ply,
                        nodes: report.nodes,
                        classes: report.classes,
                    });
                }
                stats.decisions_matched += 1;
                stats.worlds_matched += d.worlds.len();
                stats.matched.push(MatchedDecision {
                    hand: d.hand,
                    seat: d.seat,
                    trick_no: d.trick_no,
                    ply: d.ply,
                    worlds_matched: d.worlds.len(),
                    worlds_total: d.worlds.len(),
                    strict_worlds: 0,
                });
            }
        }
    }
    Ok(stats)
}

/// The atom cells constant across a kernel's whole fiber (defined at every
/// world with one value) — the latent part of an origin's description.
fn constant_atom_cells(kernel: &Kernel, ctx: &Exp3aContext) -> Vec<Constraint> {
    let atoms = vocabulary(kernel);
    let mut baseline: Vec<Option<AtomValue>> = vec![None; atoms.len()];
    let mut dead = vec![false; atoms.len()];
    let mut first = true;
    for world in kernel.worlds() {
        for (i, atom) in atoms.iter().enumerate() {
            if dead[i] {
                continue;
            }
            let v = eval_atom(*atom, kernel, ctx, &world);
            if first {
                match v {
                    Some(x) => baseline[i] = Some(x),
                    None => dead[i] = true,
                }
            } else if baseline[i] != v {
                dead[i] = true;
            }
        }
        first = false;
    }
    atoms
        .into_iter()
        .zip(baseline)
        .zip(dead)
        .filter_map(|((atom, v), dead)| match (v, dead) {
            (Some(v), false) => Some(Constraint::Atom(atom, v)),
            _ => None,
        })
        .collect()
}

/// The origin's full implicant: identity, public frame, constant atoms.
fn initial_implicant(rh: &ReceiptHand, seat: Seat, decision: &ReceiptDecision) -> Implicant {
    let kernel = &decision.kernel;
    let ctx = Exp3aContext::new(kernel, valued_tile(kernel));
    let mut cells = vec![
        Constraint::Hand(rh.id),
        Constraint::Seat(seat),
        Constraint::Decl(kernel.decl()),
        Constraint::Role(role_of(rh, seat)),
        Constraint::Horizon(kernel.viewer_hand().len()),
        Constraint::Ply(decision.ply),
    ];
    cells.extend(constant_atom_cells(kernel, &ctx));
    Implicant { cells }
}

/// The declared forward drop order over the initial cells (module doc).
fn drop_order(cells: &[Constraint]) -> Vec<usize> {
    let mut order = Vec::with_capacity(cells.len());
    let stages: [&dyn Fn(&Constraint) -> bool; 6] = [
        &|c| matches!(c, Constraint::Hand(_) | Constraint::Seat(_)),
        &|c| matches!(c, Constraint::Atom(..)),
        &|c| matches!(c, Constraint::Decl(_)),
        &|c| matches!(c, Constraint::Ply(_)),
        &|c| matches!(c, Constraint::Role(_)),
        &|c| matches!(c, Constraint::Horizon(_)),
    ];
    for stage in stages {
        order.extend(
            cells
                .iter()
                .enumerate()
                .filter(|(_, c)| stage(c))
                .map(|(i, _)| i),
        );
    }
    debug_assert_eq!(order.len(), cells.len(), "every cell is staged once");
    order
}

/// One greedy pass in one order. Returns the surviving-cell mask and the
/// full trace.
fn greedy_pass(
    cells: &[Constraint],
    order: &[usize],
    domain: &BasinDomain,
    verdict: &LessonVerdict,
    trees: &mut BTreeMap<usize, KernelTree>,
) -> (Vec<bool>, Vec<DropStep>) {
    let mut live = vec![true; cells.len()];
    let mut trace = Vec::with_capacity(order.len());
    for &i in order {
        live[i] = false;
        let implicant: Vec<Constraint> = cells
            .iter()
            .zip(&live)
            .filter(|(_, l)| **l)
            .map(|(c, _)| *c)
            .collect();
        match verify(domain, &implicant, verdict, trees) {
            Ok(_) => trace.push(DropStep {
                cell: cells[i],
                outcome: DropOutcome::Dropped,
            }),
            Err(w) => {
                live[i] = true;
                trace.push(DropStep {
                    cell: cells[i],
                    outcome: DropOutcome::LoadBearing(w),
                });
            }
        }
    }
    (live, trace)
}

/// The full generalization: initial verification, two greedy passes, the
/// larger basin kept.
fn run(
    origin: LessonOrigin,
    verdict: LessonVerdict,
    grade: LessonGrade,
    initial: Implicant,
    domain: &BasinDomain,
) -> Lesson {
    let mut trees = BTreeMap::new();
    if let Err(w) = verify(domain, &initial.cells, &verdict, &mut trees) {
        panic!("the origin implicant must verify on the domain, got witness {w:?}");
    }
    let forward = drop_order(&initial.cells);
    let mut reverse = forward.clone();
    reverse.reverse();

    let mut best: Option<(Vec<bool>, Vec<DropStep>, Stats)> = None;
    for order in [&forward, &reverse] {
        let (live, trace) = greedy_pass(&initial.cells, order, domain, &verdict, &mut trees);
        let final_cells: Vec<Constraint> = initial
            .cells
            .iter()
            .zip(&live)
            .filter(|(_, l)| **l)
            .map(|(c, _)| *c)
            .collect();
        let stats = verify(domain, &final_cells, &verdict, &mut trees)
            .expect("the accepted implicant re-verifies");
        let better = match &best {
            None => true,
            Some((_, _, b)) => {
                (stats.decisions_matched, stats.worlds_matched)
                    > (b.decisions_matched, b.worlds_matched)
            }
        };
        if better {
            best = Some((live, trace, stats));
        }
    }
    let (live, trace, stats) = best.expect("two passes ran");
    let implicant = Implicant {
        cells: initial
            .cells
            .iter()
            .zip(&live)
            .filter(|(_, l)| **l)
            .map(|(c, _)| *c)
            .collect(),
    };
    Lesson {
        origin,
        verdict,
        grade,
        initial,
        trace,
        implicant,
        basin: BasinReport {
            domain: domain.spec,
            decisions_total: domain.decisions.len(),
            worlds_total: domain.worlds_total,
            decisions_matched: stats.decisions_matched,
            worlds_matched: stats.worlds_matched,
            strict_worlds: stats.strict_worlds,
            matched: stats.matched,
        },
    }
}

/// Generalizes a walker dominance conflict into a refutation lesson: the
/// worse selector names the transcript's chosen action, the better
/// selector the dominating alternative with the highest expectation (ties
/// to the lowest tile index). `None` when the record's chosen action is
/// not worldwise-dominated on its evaluated world set.
pub fn generalize_regret(
    rh: &ReceiptHand,
    seat: Seat,
    record: &DecisionRecord,
    domain: &BasinDomain,
) -> Option<Lesson> {
    if !record.chosen_dominated {
        return None;
    }
    let conflict = record
        .conflict(rh.id, seat)
        .expect("a dominated choice is regretted");
    let mut best: Option<usize> = None;
    for (i, j) in &record.dominance {
        if *j != record.chosen_index {
            continue;
        }
        best = Some(match best {
            None => *i,
            Some(b) if record.values[*i] > record.values[b] => *i,
            Some(b) => b,
        });
    }
    let better_tile = record.actions[best.expect("a dominated choice has a dominator")];

    let decision =
        ReceiptDecision::at(rh, record.trick_no, seat).expect("the walked decision reconstructs");
    let ctx = Exp3aContext::new(&decision.kernel, valued_tile(&decision.kernel));
    let verdict = LessonVerdict::Refutation {
        worse: ActionSelector::fit(record.chosen, &record.actions, ctx.decisive),
        better: ActionSelector::fit(better_tile, &record.actions, ctx.decisive),
    };
    let initial = initial_implicant(rh, seat, &decision);
    Some(run(
        LessonOrigin::Regret(conflict),
        verdict,
        LessonGrade::Worldwise {
            operator: OperatorLabel::Pi,
        },
        initial,
        domain,
    ))
}

/// Generalizes the win form from the same conflict decision: an action
/// that attains the world optimum in *every* fiber world of the origin
/// (the intersection of the per-world argmax sets, smallest tile if
/// several). `None` when the record is not a conflict, the fiber exceeds
/// `enumeration_threshold` (the origin scan is exhaustive or absent,
/// never sampled), or no action is worldwise-optimal there.
pub fn generalize_win(
    rh: &ReceiptHand,
    seat: Seat,
    record: &DecisionRecord,
    domain: &BasinDomain,
    enumeration_threshold: u128,
) -> Option<Lesson> {
    let conflict = record.conflict(rh.id, seat)?;
    let decision =
        ReceiptDecision::at(rh, record.trick_no, seat).expect("the walked decision reconstructs");
    let kernel = &decision.kernel;
    if kernel.count() > enumeration_threshold {
        return None;
    }
    let mut pi = ScalarPi::new(
        kernel.decl(),
        kernel.viewer().team(),
        ScalarValuation::trick_plus_count(),
    );
    let mut mask = u32::MAX;
    for world in kernel.worlds() {
        let solved = pi.action_values(world.hands(), decision.leader, &decision.prefix);
        debug_assert!(solved
            .iter()
            .map(|(d, _)| *d)
            .eq(record.actions.iter().copied()));
        let values: Vec<i64> = solved.into_iter().map(|(_, v)| v).collect();
        let best = *values.iter().max().expect("actions");
        let mut m = 0u32;
        for (i, v) in values.iter().enumerate() {
            if *v == best {
                m |= 1 << i;
            }
        }
        mask &= m;
        if mask == 0 {
            return None;
        }
    }
    let action = record.actions[mask.trailing_zeros() as usize];
    let ctx = Exp3aContext::new(kernel, valued_tile(kernel));
    let verdict = LessonVerdict::Win {
        action: ActionSelector::fit(action, &record.actions, ctx.decisive),
    };
    let initial = initial_implicant(rh, seat, &decision);
    Some(run(
        LessonOrigin::Regret(conflict),
        verdict,
        LessonGrade::Worldwise {
            operator: OperatorLabel::Pi,
        },
        initial,
        domain,
    ))
}

/// Generalizes a §12.6 checker conflict: the named descriptor family
/// fails strong controlled lumpability at the viewer-lead trick-`trick_no`
/// kernel of the hand, and the lesson asks over which class of lead
/// decisions that failure persists. `None` when the family is lumpable at
/// the origin (no conflict to generalize).
pub fn generalize_lumpability(
    rh: &ReceiptHand,
    trick_no: usize,
    descriptor: DescriptorFamily,
    domain: &BasinDomain,
) -> Option<Lesson> {
    let kernel = Kernel::from_receipt_trick(rh, trick_no).expect("a valid receipt kernel");
    let seat = kernel.viewer();
    let tree = KernelTree::build(&kernel, ScalarValuation::trick_plus_count());
    let skeleton = match descriptor {
        DescriptorFamily::Chassis => AtomDescriptor::new(&kernel, true, Vec::new()),
    };
    let failure = check_lumpability(&kernel, &tree, &skeleton).failure?;

    let decision = ReceiptDecision::at(rh, trick_no, seat).expect("the lead decision reconstructs");
    assert_eq!(decision.ply, 0, "the trick leader is the viewer");
    let initial = initial_implicant(rh, seat, &decision);
    Some(run(
        LessonOrigin::Lumpability {
            hand: rh.id,
            trick_no,
            descriptor,
            failure,
        },
        LessonVerdict::NotLumpable { descriptor },
        LessonGrade::Checker,
        initial,
        domain,
    ))
}
