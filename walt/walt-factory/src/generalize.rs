//! The generalizer (S5b, extended S5c-m1): from one conflict to a lesson,
//! by greedy widening with witness-terminated verification.
//!
//! 1UIP culture, not minimal-core extraction: the initial implicant is the
//! origin's full vocabulary description (identity, public frame with
//! horizon as a bound pair, every atom cell constant across the origin
//! fiber — numerics as bound pairs); the generalizer then walks a
//! **declared drop order**. An equality cell attempts a single drop; a
//! bound cell walks its RELAXATION LADDER, one step weaker per verified
//! widening, until vacuity (then it is dropped) or a counterexample (then
//! it HOLDS at its last verified value — the load-bearing bound, named
//! with its value). Every widening is re-verified exhaustively over the
//! whole basin domain, and the first counterexample is recorded whole.
//!
//! **Cut refinement** (S5c-m1): when a widening fails at a witness, the
//! generalizer may INTRODUCE a world-selecting cell from the origin's
//! registered vocabularies — one that is constant (equality) or bounded
//! (numeric interval) across the pairs verified under the pre-widening
//! implicant and excludes the witness — then re-verify; up to
//! `INTRO_BUDGET` introductions per pass, first candidate in vocabulary
//! order, everything rolled back if the widening still fails. Introduced
//! cells are traced as `Introduce` steps, distinct from drops.
//!
//! Two fixed orders are tried (forward = identity, atoms, frame; and its
//! reverse — the restart policy, cheap and deterministic) and the lesson
//! with the larger basin is kept (ties to forward). Vacuous verification
//! is allowed and honest: the basin report says what actually matched.
//!
//! Verification is per the verdict's quantifier (`lesson` module doc):
//! pair verdicts world-by-world over precomputed exact PI values, the
//! checker verdict decision-by-decision through the exhaustive §12.6
//! checker. Nothing is sampled; no seeds exist here.

use std::collections::BTreeMap;

use walt_core::receipt::ReceiptHand;
use walt_core::Seat;
use walt_kernel::{Kernel, ReceiptDecision};
use walt_skeleton::{check_lumpability, AtomDescriptor, Exp3aAtom, Exp3aContext, KernelTree};
use walt_strat::{ScalarPi, ScalarValuation};

use crate::basin::{eval_atom, valued_tile, vocabulary, BasinDomain, DomainDecision};
use walt_geom::{q, qi};
use walt_skeleton::StaticValue;

use crate::lesson::{
    ActionSelector, AtomValue, BasinReport, CarrierLabel, Constraint, DescriptorFamily,
    DominanceTriple, Implicant, Lesson, LessonAtom, LessonGrade, LessonOrigin, LessonVerdict,
    MatchedDecision, NumericAtom, OperatorPair, RentReport, Role, StepOutcome, TraceStep,
    WideningWitness,
};
use crate::walker::DecisionRecord;

/// Basin statistics accumulated by one successful verification pass.
struct Stats {
    decisions_matched: usize,
    worlds_matched: usize,
    /// Filled by `run` after the final implicant is fixed.
    frame_compatible: usize,
    /// Summed dominance triples of the matched worlds; `None` for the
    /// checker verdict (no per-world comparison).
    dominance: Option<DominanceTriple>,
    matched: Vec<MatchedDecision>,
}

impl Stats {
    fn empty(pair_verdict: bool) -> Stats {
        Stats {
            decisions_matched: 0,
            worlds_matched: 0,
            frame_compatible: 0,
            dominance: pair_verdict.then_some(DominanceTriple {
                gt: 0,
                eq: 0,
                lt: 0,
            }),
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

/// Is this cell decision-sort (public frame/identity) as opposed to
/// world-sort (latent atoms)?
fn is_decision_cell(c: &Constraint) -> bool {
    !matches!(
        c,
        Constraint::Atom(..) | Constraint::NumericGe(..) | Constraint::NumericLe(..)
    )
}

/// Do the decision-sort cells hold at this domain decision?
fn decision_cells_hold(d: &DomainDecision, cells: &[Constraint]) -> bool {
    cells.iter().all(|c| match c {
        Constraint::Hand(h) => *h == d.hand,
        Constraint::Seat(s) => *s == d.seat,
        Constraint::Decl(x) => *x == d.kernel.decl(),
        Constraint::Role(r) => *r == d.role,
        Constraint::HorizonGe(n) => d.horizon >= *n,
        Constraint::HorizonLe(n) => d.horizon <= *n,
        Constraint::Ply(p) => *p == d.ply,
        Constraint::Atom(..) | Constraint::NumericGe(..) | Constraint::NumericLe(..) => true,
    })
}

/// The registered numeric's value at one world of one decision, through
/// the precomputed columns. `None` where the underlying atom's
/// precondition fails — a bound over an undefined numeric is unsatisfied,
/// the same partial semantics as equality cells.
fn numeric_value(d: &DomainDecision, atom: NumericAtom, widx: usize) -> Option<u8> {
    match atom {
        NumericAtom::BeatersTotal(t) => match d.column(LessonAtom::Beaters(t))?[widx]? {
            AtomValue::Counts(v) => Some(v.iter().sum()),
            AtomValue::Static(_) => unreachable!("beaters columns hold count vectors"),
        },
        NumericAtom::OppBeaters => match d.column(LessonAtom::Ctl(Exp3aAtom::OppBeaters))?[widx]? {
            AtomValue::Static(StaticValue::Count(c)) => Some(c),
            _ => unreachable!("opp-beaters is a count"),
        },
    }
}

/// Does world `widx` of this decision satisfy every atom cell?
fn world_matches(d: &DomainDecision, cells: &[Constraint], widx: usize) -> bool {
    cells.iter().all(|c| match c {
        Constraint::Atom(atom, v) => d.column(*atom).is_some_and(|col| col[widx] == Some(*v)),
        Constraint::NumericGe(a, k) => numeric_value(d, *a, widx).is_some_and(|v| v >= *k),
        Constraint::NumericLe(a, k) => numeric_value(d, *a, widx).is_some_and(|v| v <= *k),
        _ => true,
    })
}

/// Do the atom cells hold at *every* world of this decision (the
/// fiber-valid reading used by decision-graded verdicts)?
fn atom_cells_fiber_valid(d: &DomainDecision, cells: &[Constraint]) -> bool {
    (0..d.worlds.len()).all(|w| world_matches(d, cells, w))
}

/// Is this decision on the verdict's own carrier (§11.1)? Eligibility is
/// implicant-independent: for pair verdicts the selectors must resolve to
/// legal (and, for refutation, distinct) actions; for the checker verdict
/// the decision must be a viewer-lead trick boundary at horizon <= 2 (the
/// §12.6 carrier requirement).
fn eligible_for(d: &DomainDecision, verdict: &LessonVerdict) -> bool {
    match verdict {
        LessonVerdict::Refutation { worse, better } => matches!(
            (
                worse.resolve(&d.actions, d.decisive),
                better.resolve(&d.actions, d.decisive),
            ),
            (Some(w), Some(b)) if w != b
        ),
        LessonVerdict::Win { action } => action.resolve(&d.actions, d.decisive).is_some(),
        LessonVerdict::NotLumpable { .. } => d.ply == 0 && d.horizon <= 2,
    }
}

/// Applies a lesson at one precomputed decision — the application entry
/// point (and S5c's pruning hook). TRUST-01 shape: **a verdict's scope is
/// its verified domain**, so the lesson's stored `DomainSpec` gates
/// application before the implicant is even read — outside the verified
/// domain the answer is `None` no matter how weak the implicant is (an
/// empty implicant never matches beyond its verified domain; widening the
/// scope means re-verifying on the wider domain, never syntactic match).
/// Inside the domain, `Some` carries the matched world indices of the
/// decision's fiber (every index for the checker verdict, whose claim is
/// per decision).
pub fn lesson_applies(lesson: &Lesson, d: &DomainDecision) -> Option<Vec<usize>> {
    if !lesson
        .basin
        .domain
        .covers(d.trick_no, d.worlds.len() as u128)
    {
        return None;
    }
    if !eligible_for(d, &lesson.verdict) {
        return None;
    }
    let cells = &lesson.implicant.cells;
    if !decision_cells_hold(d, cells) {
        return None;
    }
    match &lesson.verdict {
        LessonVerdict::Refutation { .. } | LessonVerdict::Win { .. } => {
            let idx: Vec<usize> = (0..d.worlds.len())
                .filter(|&w| world_matches(d, cells, w))
                .collect();
            (!idx.is_empty()).then_some(idx)
        }
        LessonVerdict::NotLumpable { .. } => {
            atom_cells_fiber_valid(d, cells).then(|| (0..d.worlds.len()).collect())
        }
    }
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
    let mut stats = Stats::empty(!matches!(verdict, LessonVerdict::NotLumpable { .. }));
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
                // The dominance primitive over this decision's matched
                // worlds. `lt` stays 0 in any verified basin: a `lt`
                // world is the witness that aborts the widening.
                let mut triple = DominanceTriple {
                    gt: 0,
                    eq: 0,
                    lt: 0,
                };
                for (widx, world) in d.worlds.iter().enumerate() {
                    if !world_matches(d, cells, widx) {
                        continue;
                    }
                    let row = &d.values[widx];
                    let (b, w) = match wi {
                        None => (row[bi], *row.iter().max().expect("actions")),
                        Some(wi) => (row[bi], row[wi]),
                    };
                    if b < w {
                        return Err(WideningWitness::World {
                            hand: d.hand,
                            seat: d.seat,
                            trick_no: d.trick_no,
                            ply: d.ply,
                            world: *world,
                            values: d.actions.iter().copied().zip(row.iter().copied()).collect(),
                            decision_index: di,
                            world_index: widx,
                        });
                    }
                    if b > w {
                        triple.gt += 1;
                    } else {
                        triple.eq += 1;
                    }
                }
                let wm = triple.worlds();
                if wm > 0 {
                    stats.decisions_matched += 1;
                    stats.worlds_matched += wm;
                    let total = stats.dominance.as_mut().expect("pair verdict");
                    total.gt += triple.gt;
                    total.eq += triple.eq;
                    total.lt += triple.lt;
                    stats.matched.push(MatchedDecision {
                        hand: d.hand,
                        seat: d.seat,
                        trick_no: d.trick_no,
                        ply: d.ply,
                        worlds_matched: wm,
                        worlds_total: d.worlds.len(),
                        dominance: Some(triple),
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
                    dominance: None,
                });
            }
        }
    }
    Ok(stats)
}

/// The atom cells constant across a kernel's whole fiber (defined at
/// every world with one value) — the latent part of an origin's
/// description. Equality cells for the non-numeric vocabulary; the
/// registered numerics (beater totals, opp-beaters) enter as bound PAIRS
/// at their constant value so the generalizer can relax them stepwise
/// (S5c-m1; the per-slot beater vector remains column substrate, not
/// implicant language — a declared narrowing, see PLAN).
fn constant_atom_cells(kernel: &Kernel, ctx: &Exp3aContext) -> Vec<Constraint> {
    let atoms: Vec<LessonAtom> = vocabulary(kernel)
        .into_iter()
        .filter(|a| {
            !matches!(
                a,
                LessonAtom::Beaters(_) | LessonAtom::Ctl(Exp3aAtom::OppBeaters)
            )
        })
        .collect();
    let mut numerics: Vec<NumericAtom> = kernel
        .pool()
        .iter()
        .map(NumericAtom::BeatersTotal)
        .collect();
    numerics.push(NumericAtom::OppBeaters);

    let mut baseline: Vec<Option<AtomValue>> = vec![None; atoms.len()];
    let mut dead = vec![false; atoms.len()];
    let mut num_baseline: Vec<Option<u8>> = vec![None; numerics.len()];
    let mut num_dead = vec![false; numerics.len()];
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
        for (i, num) in numerics.iter().enumerate() {
            if num_dead[i] {
                continue;
            }
            let v = match num {
                NumericAtom::BeatersTotal(t) => {
                    eval_atom(LessonAtom::Beaters(*t), kernel, ctx, &world).map(|v| match v {
                        AtomValue::Counts(c) => c.iter().sum(),
                        AtomValue::Static(_) => unreachable!("beaters are count vectors"),
                    })
                }
                NumericAtom::OppBeaters => {
                    eval_atom(LessonAtom::Ctl(Exp3aAtom::OppBeaters), kernel, ctx, &world).map(
                        |v| match v {
                            AtomValue::Static(StaticValue::Count(c)) => c,
                            _ => unreachable!("opp-beaters is a count"),
                        },
                    )
                }
            };
            if first {
                match v {
                    Some(x) => num_baseline[i] = Some(x),
                    None => num_dead[i] = true,
                }
            } else if num_baseline[i] != v {
                num_dead[i] = true;
            }
        }
        first = false;
    }
    let mut cells: Vec<Constraint> = atoms
        .into_iter()
        .zip(baseline)
        .zip(dead)
        .filter_map(|((atom, v), dead)| match (v, dead) {
            (Some(v), false) => Some(Constraint::Atom(atom, v)),
            _ => None,
        })
        .collect();
    for ((num, v), dead) in numerics.into_iter().zip(num_baseline).zip(num_dead) {
        if let (Some(v), false) = (v, dead) {
            cells.push(Constraint::NumericGe(num, v));
            cells.push(Constraint::NumericLe(num, v));
        }
    }
    cells
}

/// The origin's full implicant: identity, public frame (horizon as its
/// bound pair), constant atoms.
fn initial_implicant(rh: &ReceiptHand, seat: Seat, decision: &ReceiptDecision) -> Implicant {
    let kernel = &decision.kernel;
    let ctx = Exp3aContext::new(kernel, valued_tile(kernel));
    let h = kernel.viewer_hand().len();
    let mut cells = vec![
        Constraint::Hand(rh.id),
        Constraint::Seat(seat),
        Constraint::Decl(kernel.decl()),
        Constraint::Role(role_of(rh, seat)),
        Constraint::HorizonGe(h),
        Constraint::HorizonLe(h),
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
        &|c| {
            matches!(
                c,
                Constraint::Atom(..) | Constraint::NumericGe(..) | Constraint::NumericLe(..)
            )
        },
        &|c| matches!(c, Constraint::Decl(_)),
        &|c| matches!(c, Constraint::Ply(_)),
        &|c| matches!(c, Constraint::Role(_)),
        &|c| matches!(c, Constraint::HorizonGe(_) | Constraint::HorizonLe(_)),
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

/// One relaxation step of a bound cell, or `None` when the next step is
/// vacuous everywhere (`horizon` bounds at their extremes) — the bound
/// then attempts a full drop. A numeric bound at its extreme still
/// asserts DEFINEDNESS of the numeric (partial semantics), so its ladder
/// ends at the extreme value and the last step is likewise a full drop.
fn relax(cell: Constraint) -> Option<Constraint> {
    match cell {
        Constraint::HorizonGe(n) if n > 2 => Some(Constraint::HorizonGe(n - 1)),
        Constraint::HorizonLe(n) if n < 7 => Some(Constraint::HorizonLe(n + 1)),
        Constraint::NumericGe(a, k) if k > 0 => Some(Constraint::NumericGe(a, k - 1)),
        Constraint::NumericLe(a, k) if k < 21 => Some(Constraint::NumericLe(a, k + 1)),
        _ => None,
    }
}

fn is_bound(cell: &Constraint) -> bool {
    matches!(
        cell,
        Constraint::HorizonGe(_)
            | Constraint::HorizonLe(_)
            | Constraint::NumericGe(..)
            | Constraint::NumericLe(..)
    )
}

/// The (decision, world) pairs matching an implicant on the verdict's
/// carrier — the preservation target for cut refinement.
fn matched_pairs(
    domain: &BasinDomain,
    cells: &[Constraint],
    verdict: &LessonVerdict,
) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (di, d) in domain.decisions.iter().enumerate() {
        if !eligible_for(d, verdict) || !decision_cells_hold(d, cells) {
            continue;
        }
        if matches!(verdict, LessonVerdict::NotLumpable { .. }) {
            if atom_cells_fiber_valid(d, cells) {
                out.extend((0..d.worlds.len()).map(|w| (di, w)));
            }
            continue;
        }
        out.extend(
            (0..d.worlds.len())
                .filter(|&w| world_matches(d, cells, w))
                .map(|w| (di, w)),
        );
    }
    out
}

/// Cut refinement: the first cell of the lesson's atom language that is
/// constant (equality) or bounded (numeric) across the preserved matched
/// pairs and excludes the witness — `None` when no registered cell
/// separates them. Candidate order is the origin vocabulary's, so the
/// choice is deterministic.
fn separating_cell(
    domain: &BasinDomain,
    preserved: &[(usize, usize)],
    witness: &WideningWitness,
    candidate_atoms: &[LessonAtom],
    candidate_numerics: &[NumericAtom],
) -> Option<Constraint> {
    let WideningWitness::World {
        decision_index,
        world_index,
        ..
    } = witness
    else {
        return None;
    };
    let (wdi, wwi) = (*decision_index, *world_index);
    if preserved.is_empty() {
        return None;
    }
    for atom in candidate_atoms {
        let Some(first_col) = domain.decisions[preserved[0].0].column(*atom) else {
            continue;
        };
        let Some(v) = first_col[preserved[0].1] else {
            continue;
        };
        let constant = preserved.iter().all(|&(di, wi)| {
            domain.decisions[di]
                .column(*atom)
                .is_some_and(|col| col[wi] == Some(v))
        });
        if !constant {
            continue;
        }
        let at_witness = domain.decisions[wdi].column(*atom).and_then(|col| col[wwi]);
        if at_witness != Some(v) {
            return Some(Constraint::Atom(*atom, v));
        }
    }
    for num in candidate_numerics {
        let values: Option<Vec<u8>> = preserved
            .iter()
            .map(|&(di, wi)| numeric_value(&domain.decisions[di], *num, wi))
            .collect();
        let Some(values) = values else { continue };
        let (lo, hi) = (
            *values.iter().min().expect("nonempty"),
            *values.iter().max().expect("nonempty"),
        );
        let at_witness = numeric_value(&domain.decisions[wdi], *num, wwi);
        match at_witness {
            Some(w) if w > hi => return Some(Constraint::NumericLe(*num, hi)),
            Some(w) if w < lo => return Some(Constraint::NumericGe(*num, lo)),
            None => return Some(Constraint::NumericLe(*num, hi)),
            _ => {}
        }
    }
    None
}

/// One pass's working state: per-initial-cell current form (possibly
/// relaxed, `None` = dropped) plus the introduced cells.
struct PassState {
    current: Vec<Option<Constraint>>,
    introduced: Vec<Constraint>,
    intro_budget: usize,
    trace: Vec<TraceStep>,
}

impl PassState {
    fn implicant(&self) -> Vec<Constraint> {
        let mut out: Vec<Constraint> = self.current.iter().filter_map(|c| *c).collect();
        out.extend(self.introduced.iter().copied());
        out
    }
}

/// Attempts one widening (cell `i` moved to `to`), with cut refinement on
/// failure: introduced cells must preserve the pre-widening matched set
/// and exclude the witness, and each introduction costs one unit of the
/// pass's budget. On failure everything this attempt added is rolled
/// back. Returns whether the widening ended up verified.
#[allow(clippy::too_many_arguments)]
fn attempt_widening(
    state: &mut PassState,
    i: usize,
    to: Option<Constraint>,
    domain: &BasinDomain,
    verdict: &LessonVerdict,
    trees: &mut BTreeMap<usize, KernelTree>,
    candidate_atoms: &[LessonAtom],
    candidate_numerics: &[NumericAtom],
) -> Result<(), WideningWitness> {
    let prev = state.current[i];
    let preserved = matched_pairs(domain, &state.implicant(), verdict);
    state.current[i] = to;
    let mut added = 0usize;
    let mut first_witness: Option<WideningWitness> = None;
    loop {
        match verify(domain, &state.implicant(), verdict, trees) {
            Ok(_) => return Ok(()),
            Err(w) => {
                if first_witness.is_none() {
                    first_witness = Some(w.clone());
                }
                if state.intro_budget == 0 {
                    break;
                }
                let Some(cell) =
                    separating_cell(domain, &preserved, &w, candidate_atoms, candidate_numerics)
                else {
                    break;
                };
                state.introduced.push(cell);
                state.intro_budget -= 1;
                added += 1;
                state.trace.push(TraceStep::Introduce { cell, witness: w });
            }
        }
    }
    // Roll back: the widening failed even with refinement.
    for _ in 0..added {
        state.introduced.pop();
        state.trace.pop();
        state.intro_budget += 1;
    }
    state.current[i] = prev;
    Err(first_witness.expect("the loop only exits after a failure"))
}

/// The per-pass cut-refinement budget (declared; 1UIP culture — a few
/// good cells cheaply, never a saturation search).
const INTRO_BUDGET: usize = 4;

/// One greedy pass in one order: equality cells attempt a drop, bound
/// cells walk their relaxation ladder; both may recruit cut refinement.
#[allow(clippy::too_many_arguments)]
fn greedy_pass(
    cells: &[Constraint],
    order: &[usize],
    domain: &BasinDomain,
    verdict: &LessonVerdict,
    trees: &mut BTreeMap<usize, KernelTree>,
    candidate_atoms: &[LessonAtom],
    candidate_numerics: &[NumericAtom],
) -> PassState {
    let mut state = PassState {
        current: cells.iter().map(|c| Some(*c)).collect(),
        introduced: Vec::new(),
        intro_budget: INTRO_BUDGET,
        trace: Vec::new(),
    };
    for &i in order {
        let cell = cells[i];
        if !is_bound(&cell) {
            let outcome = match attempt_widening(
                &mut state,
                i,
                None,
                domain,
                verdict,
                trees,
                candidate_atoms,
                candidate_numerics,
            ) {
                Ok(()) => StepOutcome::Dropped,
                Err(w) => StepOutcome::LoadBearing(w),
            };
            state.trace.push(TraceStep::Drop { cell, outcome });
            continue;
        }
        // Bound cell: relax stepwise, then attempt the full drop.
        let outcome = loop {
            let held = state.current[i].expect("bound cells relax in place");
            let next = relax(held);
            match attempt_widening(
                &mut state,
                i,
                next,
                domain,
                verdict,
                trees,
                candidate_atoms,
                candidate_numerics,
            ) {
                Ok(()) => {
                    if next.is_none() {
                        break StepOutcome::Dropped;
                    }
                }
                Err(w) => break StepOutcome::BoundHeld { held, witness: w },
            }
        };
        state.trace.push(TraceStep::Drop { cell, outcome });
    }
    state
}

/// The full generalization: initial verification, two greedy passes, the
/// larger basin kept.
fn run(
    origin: LessonOrigin,
    verdict: LessonVerdict,
    grade: LessonGrade,
    initial: Implicant,
    origin_kernel: &Kernel,
    domain: &BasinDomain,
) -> Lesson {
    let mut trees = BTreeMap::new();
    if let Err(w) = verify(domain, &initial.cells, &verdict, &mut trees) {
        panic!("the origin implicant must verify on the domain, got witness {w:?}");
    }
    // The cut-refinement candidate language: the origin kernel's own
    // vocabulary (equality shapes minus the numerics' equality forms) and
    // the registered numerics.
    let candidate_atoms: Vec<LessonAtom> = vocabulary(origin_kernel)
        .into_iter()
        .filter(|a| {
            !matches!(
                a,
                LessonAtom::Beaters(_) | LessonAtom::Ctl(Exp3aAtom::OppBeaters)
            )
        })
        .collect();
    let mut candidate_numerics: Vec<NumericAtom> = origin_kernel
        .pool()
        .iter()
        .map(NumericAtom::BeatersTotal)
        .collect();
    candidate_numerics.push(NumericAtom::OppBeaters);

    let forward = drop_order(&initial.cells);
    let mut reverse = forward.clone();
    reverse.reverse();

    let mut best: Option<(PassState, Stats)> = None;
    for order in [&forward, &reverse] {
        let state = greedy_pass(
            &initial.cells,
            order,
            domain,
            &verdict,
            &mut trees,
            &candidate_atoms,
            &candidate_numerics,
        );
        let stats = verify(domain, &state.implicant(), &verdict, &mut trees)
            .expect("the accepted implicant re-verifies");
        let better = match &best {
            None => true,
            Some((_, b)) => {
                (stats.decisions_matched, stats.worlds_matched)
                    > (b.decisions_matched, b.worlds_matched)
            }
        };
        if better {
            best = Some((state, stats));
        }
    }
    let (state, mut stats) = best.expect("two passes ran");
    let implicant = Implicant {
        cells: state.implicant(),
    };
    let trace = state.trace;
    // The basin/frame-compatible denominator: eligible decisions whose
    // decision-sort cells hold under the final implicant.
    let decision_cells: Vec<Constraint> = implicant
        .cells
        .iter()
        .filter(|c| is_decision_cell(c))
        .copied()
        .collect();
    stats.frame_compatible = domain
        .decisions
        .iter()
        .filter(|d| eligible_for(d, &verdict) && decision_cells_hold(d, &decision_cells))
        .count();
    // The verdict's own carrier (§11.1): eligibility is
    // implicant-independent, so the rate base is fixed per verdict.
    let eligible: Vec<&DomainDecision> = domain
        .decisions
        .iter()
        .filter(|d| eligible_for(d, &verdict))
        .collect();
    let carrier = match verdict {
        LessonVerdict::NotLumpable { .. } => CarrierLabel::LeadKernelTrees,
        _ => CarrierLabel::SelectorPairs,
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
            domain_decisions: domain.decisions.len(),
            domain_worlds: domain.worlds_total,
            domain_excluded: domain.excluded_decisions,
            carrier,
            decisions_eligible: eligible.len(),
            worlds_eligible: eligible.iter().map(|d| d.worlds.len()).sum(),
            decisions_matched: stats.decisions_matched,
            worlds_matched: stats.worlds_matched,
            frame_compatible: stats.frame_compatible,
            dominance: stats.dominance,
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
            operator: OperatorPair::walker_scalar(),
        },
        initial,
        &decision.kernel,
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
            operator: OperatorPair::walker_scalar(),
        },
        initial,
        kernel,
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
        &kernel,
        domain,
    ))
}

/// Milestone-1 rent: the lesson's coarse purpose-specific usefulness on a
/// domain, measured through the gated application entry point (so an
/// out-of-scope domain pays nothing by construction). See `RentReport`
/// for the purpose discipline.
pub fn measure_rent(lesson: &Lesson, domain: &BasinDomain) -> RentReport {
    match &lesson.verdict {
        LessonVerdict::Refutation { worse, better } => {
            let mut applied = 0usize;
            let mut strict_applied = 0usize;
            let mut improvement = qi(0);
            for d in &domain.decisions {
                let Some(idx) = lesson_applies(lesson, d) else {
                    continue;
                };
                applied += 1;
                let b_tile = better.resolve(&d.actions, d.decisive).expect("applied");
                let w_tile = worse.resolve(&d.actions, d.decisive).expect("applied");
                let bi = d.actions.iter().position(|a| *a == b_tile).expect("legal");
                let wi = d.actions.iter().position(|a| *a == w_tile).expect("legal");
                let gain: i128 = idx
                    .iter()
                    .map(|&w| i128::from(d.values[w][bi]) - i128::from(d.values[w][wi]))
                    .sum();
                if gain > 0 {
                    strict_applied += 1;
                    improvement += q(gain, idx.len() as i128);
                }
            }
            RentReport::Refutation {
                applied,
                strict_applied,
                improvement,
            }
        }
        LessonVerdict::Win { .. } => {
            let mut applied = 0usize;
            let mut worlds_covered = 0usize;
            let mut actions_pruned = 0usize;
            for d in &domain.decisions {
                let Some(idx) = lesson_applies(lesson, d) else {
                    continue;
                };
                applied += 1;
                worlds_covered += idx.len();
                actions_pruned += idx.len() * (d.actions.len() - 1);
            }
            RentReport::Win {
                applied,
                worlds_covered,
                actions_pruned,
            }
        }
        LessonVerdict::NotLumpable { .. } => RentReport::Checker {
            applied: domain
                .decisions
                .iter()
                .filter(|d| lesson_applies(lesson, d).is_some())
                .count(),
        },
    }
}
