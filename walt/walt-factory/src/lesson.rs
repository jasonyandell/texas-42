//! The Lesson type of the S5 spine (S5b): a generalized conflict.
//!
//! A lesson is an **implicant over the atom vocabularies** — a conjunction
//! of typed cells — paired with a **graded, labeled verdict**, its conflict
//! of origin, the widening trace that produced it, and its measured basin
//! (v0.4 §12's compression frame: the implicant is the small Scheme-shaped
//! query, the verdict its answer relation — the typing discipline of §3–§4,
//! not the full machinery). Every piece is data on the type: a lesson
//! quotable without its grade and labels is unconstructible, the same
//! discipline as the walker's `Grade`.
//!
//! **Two-sorted implicants.** Decision cells (`Hand`, `Seat`, `Decl`,
//! `Role`, `Horizon`, `Ply`) are public facts of a decision point. Atom
//! cells (`Atom(a, v)`) are latent facts from the union vocabulary — the
//! S4 holder registry (holder/team/beater-counts per pool tile) and the
//! ported exp3A control shapes — instantiated kernel-generically through
//! the `Exp3aContext` derivation (S4.5's recorded generalization rule), so
//! the same cell is evaluable at any decision whose pool still carries the
//! referenced content. An atom cell is *partial*: where its precondition
//! fails (tile not hidden, companion undefined) it is unsatisfied, never
//! defaulted.
//!
//! **Quantifier placement is part of the verdict type.** Pair verdicts
//! (`Refutation`, `Win`) quantify per matching (decision, world): atom
//! cells select worlds inside each decision's fiber. The checker verdict
//! (`NotLumpable`) quantifies per matching decision: atom cells must hold
//! at *every* fiber world (fiber-valid) to count as a fact of the
//! decision. The basin counts what was actually verified, under exactly
//! that quantifier.
//!
//! Everything here is exploratory tier: lessons are computed evidence
//! about a declared finite domain, never axioms and never promoted
//! statuses (TRUST-01). The receipt rendering (`lesson_report`) records
//! enough for an independent implementation to re-verify (§16.11 spirit).

use core::fmt;

use walt_core::{Decl, Domino, Seat};
use walt_kernel::{World, HIDDEN_SEATS};
use walt_skeleton::{CompositeState, Exp3aAtom, LumpabilityFailure, StaticValue};
use walt_strat::WeightingLabel;

use crate::basin::DomainSpec;
use crate::conflict::RegretConflict;

/// The focal-information coordinate of an evaluation operator (v0.4
/// §10.3): what the focal side is allowed to condition on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FocalInfoLabel {
    /// World revealed before the root.
    F,
    /// Common root action, continuation re-optimized per world.
    C,
    /// Actual hidden information throughout.
    H,
}

/// The field coordinate of an evaluation operator (§10.8: perfect
/// information and fixed-field revelation are different operators, so the
/// field is a separate label, never implied).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldLabel {
    /// The field plays the omniscient adversarial best response.
    MinimaxOmniscient,
    /// The fixed uniform-over-legal chance law (§7.4).
    FixedUniformLegal,
}

/// An operator label is the *pair* (focal information, field) from the
/// §10.3 x §10.8 product grid — never a single rung. §12.4 makes basins
/// label-relative: a basin measured at one pair can shatter or merge at
/// another, so every lesson stores its pair and no verdict is quotable
/// without it (walt-math design amendment, 2026-08-10).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OperatorPair {
    pub focal_info: FocalInfoLabel,
    pub field: FieldLabel,
}

impl OperatorPair {
    /// The S5a walker's per-world scalar statistic: root action held
    /// common, continuation re-optimized per world, omniscient adversarial
    /// field — (C, MinimaxOmniscient) in the grid. The S5a walker enum
    /// spells this "PI"; an S5a part-2 amendment aligns the walker.
    pub fn walker_scalar() -> OperatorPair {
        OperatorPair {
            focal_info: FocalInfoLabel::C,
            field: FieldLabel::MinimaxOmniscient,
        }
    }
}

impl fmt::Display for OperatorPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = match self.focal_info {
            FocalInfoLabel::F => "F",
            FocalInfoLabel::C => "C",
            FocalInfoLabel::H => "H",
        };
        let field = match self.field {
            FieldLabel::MinimaxOmniscient => "minimax-omniscient",
            FieldLabel::FixedUniformLegal => "fixed-uniform-legal",
        };
        write!(f, "({info}, {field})")
    }
}

/// The dominance primitive for one comparison over a set of worlds: the
/// world-count triple (#{better > worse}, #{better = worse},
/// #{better < worse}). The named classes are *derived views*, never
/// stored: T (tied everywhere), W (weak dominance, strict somewhere), S
/// (strict everywhere), I (incomparable). §9.6 settles that pruning
/// requires strictness somewhere, so a conflict fires on W and T is never
/// a conflict; ties are never collapsed early (the S3.5 act_param
/// precedent).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DominanceTriple {
    pub gt: usize,
    pub eq: usize,
    pub lt: usize,
}

/// The derived dominance classification of a triple.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DominanceClass {
    /// `gt = lt = 0`: an interchangeability statement at the label, not a
    /// refutation — and §10.9's caveat applies: payoff equivalence at the
    /// label does not make the actions interchangeable for the seat (they
    /// can induce different continuation-information structure).
    TiedEverywhere,
    /// `gt > 0, lt = 0, eq > 0`: weak dominance, strict somewhere — the
    /// class a conflict fires on.
    WeakStrictSomewhere,
    /// `lt = eq = 0, gt > 0`.
    StrictEverywhere,
    /// `gt > 0, lt > 0` (or `lt > 0` at all): never present in a verified
    /// basin — verification aborts with the witnessing world instead.
    Incomparable,
}

impl DominanceTriple {
    pub fn class(&self) -> DominanceClass {
        if self.lt > 0 {
            DominanceClass::Incomparable
        } else if self.gt == 0 {
            DominanceClass::TiedEverywhere
        } else if self.eq == 0 {
            DominanceClass::StrictEverywhere
        } else {
            DominanceClass::WeakStrictSomewhere
        }
    }

    pub fn worlds(&self) -> usize {
        self.gt + self.eq + self.lt
    }
}

impl fmt::Display for DominanceTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.gt, self.eq, self.lt)
    }
}

impl fmt::Display for DominanceClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DominanceClass::TiedEverywhere => f.write_str("T (tied everywhere)"),
            DominanceClass::WeakStrictSomewhere => f.write_str("W (weak, strict somewhere)"),
            DominanceClass::StrictEverywhere => f.write_str("S (strict everywhere)"),
            DominanceClass::Incomparable => f.write_str("I (incomparable)"),
        }
    }
}

/// The viewer team's role in the hand — the one public fact of a decision
/// that needs the hand context beyond the kernel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Declaring,
    Defending,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Declaring => f.write_str("declaring"),
            Role::Defending => f.write_str("defending"),
        }
    }
}

/// A kernel-generic action selector: how a lesson names an action at
/// decisions other than its origin. Selectors resolve against the legal
/// action list and the kernel's derived decisive tile; an unresolved
/// selector (or two selectors resolving to the same tile) makes the
/// decision inapplicable — never a default.
///
/// `MaxCount`/`MinCount` order by `(count, pip_sum, index)` — a global
/// tile order that is walt-tier vocabulary (count points are globally
/// meaningful for the q_points valuation; rank is not comparable across
/// contexts, §1.3, so no rank-based selector is offered).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionSelector {
    /// The kernel's decisive tile (the `Exp3aContext` derivation), when
    /// legal.
    Decisive,
    /// The legal action maximal by `(count, pip_sum, index)`.
    MaxCount,
    /// The legal action minimal by `(count, pip_sum, index)`.
    MinCount,
    /// A concrete tile, when legal — the non-generic fallback.
    Tile(Domino),
}

impl ActionSelector {
    /// Resolves on a legal action list (ascending) given the kernel's
    /// decisive tile. Deterministic; `None` where the selector's tile is
    /// not legal.
    pub fn resolve(self, legal: &[Domino], decisive: Domino) -> Option<Domino> {
        match self {
            ActionSelector::Decisive => legal.contains(&decisive).then_some(decisive),
            ActionSelector::MaxCount => legal
                .iter()
                .copied()
                .max_by_key(|d| (d.count(), d.pip_sum(), d.index())),
            ActionSelector::MinCount => legal
                .iter()
                .copied()
                .min_by_key(|d| (d.count(), d.pip_sum(), d.index())),
            ActionSelector::Tile(d) => legal.contains(&d).then_some(d),
        }
    }

    /// The most generic selector that picks exactly `tile` at a decision:
    /// the first of `Decisive`, `MaxCount`, `MinCount` that resolves to
    /// it, else the concrete `Tile`. The seeding rule, declared here.
    pub fn fit(tile: Domino, legal: &[Domino], decisive: Domino) -> ActionSelector {
        [
            ActionSelector::Decisive,
            ActionSelector::MaxCount,
            ActionSelector::MinCount,
        ]
        .into_iter()
        .find(|s| s.resolve(legal, decisive) == Some(tile))
        .unwrap_or(ActionSelector::Tile(tile))
    }
}

impl fmt::Display for ActionSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionSelector::Decisive => f.write_str("decisive"),
            ActionSelector::MaxCount => f.write_str("max-count"),
            ActionSelector::MinCount => f.write_str("min-count"),
            ActionSelector::Tile(d) => write!(f, "tile({d})"),
        }
    }
}

/// One atom of the union vocabulary. `Ctl` wraps only the ten exp3A
/// control shapes — the holder/team coordinates come through the native
/// variants, so no atom has two names (the vocabulary builder is the one
/// construction site).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LessonAtom {
    /// Which hidden slot (viewer-relative) holds the tile.
    Holder(Domino),
    /// Whether the focal (viewer's) team holds the tile.
    Team(Domino),
    /// Unshown beaters of the tile per hidden slot (its `THREAT` set).
    Beaters(Domino),
    /// One of the ten exp3A control shapes, under the kernel's derived
    /// parameters (valued tile, decisive context).
    Ctl(Exp3aAtom),
}

impl fmt::Display for LessonAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LessonAtom::Holder(d) => write!(f, "holder({d})"),
            LessonAtom::Team(d) => write!(f, "team({d})"),
            LessonAtom::Beaters(d) => write!(f, "beaters({d})"),
            LessonAtom::Ctl(a) => write!(f, "{a}"),
        }
    }
}

/// An atom's value in a cell: the exp3A typed relational values, or the
/// beater-count vector.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AtomValue {
    Static(StaticValue),
    Counts([u8; HIDDEN_SEATS]),
}

impl fmt::Display for AtomValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomValue::Static(StaticValue::Slot(i)) => write!(f, "slot{i}"),
            AtomValue::Static(StaticValue::Flag(b)) => write!(f, "{b}"),
            AtomValue::Static(StaticValue::Tile(d)) => write!(f, "{d}"),
            AtomValue::Static(StaticValue::Rank(Some(r))) => write!(f, "r{r}"),
            AtomValue::Static(StaticValue::Rank(None)) => f.write_str("outside"),
            AtomValue::Static(StaticValue::Count(c)) => write!(f, "{c}"),
            AtomValue::Counts([a, b, c]) => write!(f, "[{a},{b},{c}]"),
        }
    }
}

/// A numeric-valued atom the order-cell language can bound (S5c-m1: the
/// adjudicated shapes — registered derived predicates with declared
/// semantics per §3.3; more numerics join only by registration here).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NumericAtom {
    /// Total unshown beaters of the tile across the three hidden slots
    /// (the componentwise `Beaters` vector summed). Undefined when the
    /// tile is not in the hidden pool.
    BeatersTotal(Domino),
    /// The exp3A `opp-beaters` count under the kernel's derived context.
    OppBeaters,
}

impl fmt::Display for NumericAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericAtom::BeatersTotal(d) => write!(f, "beaters-total({d})"),
            NumericAtom::OppBeaters => f.write_str("opp-beaters"),
        }
    }
}

/// One implicant cell: an equality cell, or a one-sided order cell over a
/// registered numeric (S5c-m1). A numeric fact enters the initial
/// implicant as the *pair* of bounds (`>= n` and `<= n`, together the
/// equality), so the generalizer can relax each side stepwise instead of
/// only keeping or deleting it — the observed S5b failure mode, where
/// four zero-basin lessons died on a horizon pin equality could not bend.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Constraint {
    /// The receipt hand — pure transcript identity.
    Hand(usize),
    /// The viewing seat — pure transcript identity.
    Seat(Seat),
    Decl(Decl),
    Role(Role),
    /// The viewer's position in the trick: 0 led, 1..=3 followed.
    Ply(usize),
    /// Tiles remaining in the viewer's hand, bounded below / above
    /// (kernel-generic; `8 - trick`).
    HorizonGe(usize),
    HorizonLe(usize),
    /// A latent atom equality cell, partial: unsatisfied where undefined.
    Atom(LessonAtom, AtomValue),
    /// Order cells over a registered numeric atom, partial like equality
    /// cells: an undefined numeric satisfies no bound.
    NumericGe(NumericAtom, u8),
    NumericLe(NumericAtom, u8),
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Hand(h) => write!(f, "hand={h}"),
            Constraint::Seat(s) => write!(f, "seat={s}"),
            Constraint::Decl(d) => write!(f, "decl={d}"),
            Constraint::Role(r) => write!(f, "role={r}"),
            Constraint::Ply(p) => write!(f, "ply={p}"),
            Constraint::HorizonGe(n) => write!(f, "horizon>={n}"),
            Constraint::HorizonLe(n) => write!(f, "horizon<={n}"),
            Constraint::Atom(a, v) => write!(f, "{a}={v}"),
            Constraint::NumericGe(a, k) => write!(f, "{a}>={k}"),
            Constraint::NumericLe(a, k) => write!(f, "{a}<={k}"),
        }
    }
}

/// A conjunction of cells. Cell order is the vocabulary order of the
/// origin (identity, frame, atoms); the drop order is the generalizer's,
/// declared there.
#[derive(Clone, Debug)]
pub struct Implicant {
    pub cells: Vec<Constraint>,
}

impl Implicant {
    /// Deterministic rendering. An equality-in-disguise bound pair
    /// (`>= k` and `<= k` on the same numeric) prints once as the derived
    /// equality `atom=k`, never as an interval (m2 adjudication) — the
    /// pair itself stays two cells in the data.
    pub fn render(&self) -> String {
        if self.cells.is_empty() {
            return "(empty)".to_string();
        }
        let mut parts: Vec<String> = Vec::new();
        let mut skip = vec![false; self.cells.len()];
        for (i, c) in self.cells.iter().enumerate() {
            if skip[i] {
                continue;
            }
            let partner = |target: &Constraint| {
                self.cells
                    .iter()
                    .enumerate()
                    .position(|(j, x)| j != i && !skip[j] && x == target)
            };
            match c {
                Constraint::NumericGe(a, k) => {
                    if let Some(j) = partner(&Constraint::NumericLe(*a, *k)) {
                        skip[j] = true;
                        parts.push(format!("{a}={k}"));
                        continue;
                    }
                    parts.push(c.to_string());
                }
                Constraint::NumericLe(a, k) => {
                    if let Some(j) = partner(&Constraint::NumericGe(*a, *k)) {
                        skip[j] = true;
                        parts.push(format!("{a}={k}"));
                        continue;
                    }
                    parts.push(c.to_string());
                }
                _ => parts.push(c.to_string()),
            }
        }
        parts.join(" & ")
    }
}

/// The kernel-generic descriptor families a checker lesson can name — each
/// constructible identically at any kernel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DescriptorFamily {
    /// The public chassis alone (hand, leader, prefix; no latent atoms).
    Chassis,
}

impl fmt::Display for DescriptorFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DescriptorFamily::Chassis => f.write_str("chassis"),
        }
    }
}

/// What the lesson asserts across its class.
#[derive(Clone, Copy, Debug)]
pub enum LessonVerdict {
    /// At every matching (decision, world): the `better` selector's exact
    /// PI continuation value is at least the `worse` selector's (weak
    /// dominance per world; the strict count is reported in the basin, not
    /// required — the refutation form of the spine).
    Refutation {
        worse: ActionSelector,
        better: ActionSelector,
    },
    /// At every matching (decision, world): the selector's action attains
    /// the world's optimal value under the lesson's operator pair — the
    /// win/sufficiency form, the QBF-cube analog. **Per-world sufficiency
    /// only, never a guarantee**: §7.6's fusion gap means no single
    /// information-consistent strategy need achieve the per-world values,
    /// so a worldwise win never yields a "guaranteed"/"safe" lesson at any
    /// seat-facing label (walt-math design amendment, 2026-08-10; the one
    /// exported corollary runs the other way — worldwise *loss* verdicts
    /// negate guarantees).
    Win { action: ActionSelector },
    /// At every matching decision (ply 0, horizon <= 2): the descriptor
    /// family fails §12.6 strong controlled lumpability — the checker
    /// species of conflict, generalized.
    NotLumpable { descriptor: DescriptorFamily },
}

impl fmt::Display for LessonVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LessonVerdict::Refutation { worse, better } => write!(
                f,
                "refutation: value({better}) >= value({worse}) at every matching (decision, world)"
            ),
            LessonVerdict::Win { action } => write!(
                f,
                "win: {action} suffices per-world (attains the world optimum) at every matching (decision, world); never a seat-facing guarantee (§7.6)"
            ),
            LessonVerdict::NotLumpable { descriptor } => write!(
                f,
                "not-lumpable: {descriptor} fails §12.6 at every matching decision"
            ),
        }
    }
}

/// The lesson's own grade: how its verdict was verified on the domain,
/// with the declared knobs as data. The ladder is the spine's (worldwise,
/// then exact-expectation, then sampled) and a lesson never quotes above
/// the rung it was verified at. Tonight's domains are exhaustively enumerated,
/// so only `Worldwise` and `Checker` are constructed; the lower rungs are
/// typed now so the ladder is fixed before sampled domains exist.
#[derive(Clone, Copy, Debug)]
pub enum LessonGrade {
    /// Verified in every world of every matching decision's exhaustively
    /// enumerated fiber. Weighting-free, never operator-free — the
    /// operator is the (focal information, field) pair.
    Worldwise { operator: OperatorPair },
    /// Verified as an exact expectation under a declared weighting.
    ExactExpectation {
        operator: OperatorPair,
        weighting: WeightingLabel,
    },
    /// Verified only on a recorded sample. Evidence, always marked.
    Sampled {
        operator: OperatorPair,
        weighting: WeightingLabel,
        seed: u64,
        draws: u32,
    },
    /// Verified per decision by the exhaustive §12.6 lumpability checker
    /// under the fixed uniform-legal field at the q_points valuation.
    Checker,
}

impl fmt::Display for LessonGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LessonGrade::Worldwise { operator } => {
                write!(f, "worldwise at {operator}; weighting-free")
            }
            LessonGrade::ExactExpectation {
                operator,
                weighting,
            } => write!(f, "exact-expectation at {operator}, {weighting}"),
            LessonGrade::Sampled {
                operator,
                weighting,
                seed,
                draws,
            } => write!(
                f,
                "sampled at {operator}, {weighting}, seed {seed:#018x}, draws {draws}"
            ),
            LessonGrade::Checker => f.write_str(
                "checker (§12.6 exhaustive lumpability, uniform-legal field, q_points valuation)",
            ),
        }
    }
}

/// The conflict a lesson generalizes — recorded whole, with its own grade
/// and labels (a lesson verified worldwise on its domain does not upgrade
/// a sampled origin; the two records travel together).
#[derive(Clone, Debug)]
pub enum LessonOrigin {
    /// A walker regret conflict (S5a).
    Regret(RegretConflict),
    /// An S4 checker failure: the named descriptor family failed §12.6 on
    /// the named receipt kernel, with the typed witness.
    Lumpability {
        hand: usize,
        trick_no: usize,
        descriptor: DescriptorFamily,
        failure: LumpabilityFailure<CompositeState>,
    },
}

/// The counterexample that ended one widening attempt and named its
/// dropped cell load-bearing.
#[derive(Clone, Debug)]
pub enum WideningWitness {
    /// A matching (decision, world) where a pair verdict fails, with the
    /// full action-value row (reconstruction data: the world is the
    /// complete deal). The indices locate the pair in the domain's
    /// enumeration order (reconstructible; the cut-refinement candidate
    /// scan reads them, receipts do not print them).
    World {
        hand: usize,
        seat: Seat,
        trick_no: usize,
        ply: usize,
        world: World,
        values: Vec<(Domino, i64)>,
        decision_index: usize,
        world_index: usize,
    },
    /// A matching decision where the descriptor family IS lumpable — the
    /// checker verdict's counterexample shape.
    Lumpable {
        hand: usize,
        seat: Seat,
        trick_no: usize,
        ply: usize,
        nodes: usize,
        classes: usize,
    },
}

/// One step of the widening trace (S5c-m1 vocabulary: Drop covers both
/// equality cells and bound-relaxation ladders; Introduce is cut
/// refinement proper).
#[derive(Clone, Debug)]
pub enum TraceStep {
    /// One initial cell's widening attempt: for an equality cell, a
    /// single drop; for a bound cell, a stepwise relaxation ladder whose
    /// terminal state is the outcome.
    Drop {
        cell: Constraint,
        outcome: StepOutcome,
    },
    /// Cut refinement: a widening failed at the witness, and this
    /// world-selecting cell — constant across the then-verified matched
    /// set, different or undefined at the witness — was introduced to
    /// separate them, after which the widening re-verified.
    Introduce {
        cell: Constraint,
        witness: WideningWitness,
    },
}

#[derive(Clone, Debug)]
pub enum StepOutcome {
    /// The cell is gone: the drop (or the relaxation to vacuity)
    /// re-verified over the whole domain.
    Dropped,
    /// An equality cell whose drop was refuted; restored — it SURVIVES,
    /// named by the witness (m2 vocabulary).
    Survives(WideningWitness),
    /// A bound cell relaxed as far as verification allowed: it stands at
    /// `held` (possibly its original value), and the witness refuted the
    /// next relaxation — the load-bearing bound, named with its value.
    BoundHeld {
        held: Constraint,
        witness: WideningWitness,
    },
}

/// One matched decision of the basin, with its matched-world counts.
#[derive(Clone, Debug)]
pub struct MatchedDecision {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    pub ply: usize,
    /// Worlds of this decision's fiber matching the implicant (pair
    /// verdicts; equals `worlds_total` for the checker verdict's
    /// fiber-valid matching).
    pub worlds_matched: usize,
    /// The decision's full fiber size.
    pub worlds_total: usize,
    /// The dominance primitive over this decision's matched worlds:
    /// better-vs-worse for refutation, action-vs-world-optimum for win
    /// (`gt` is then structurally 0). `None` for the checker verdict —
    /// there is no per-world comparison. `lt = 0` in every verified basin
    /// (verification aborts with the witnessing world instead).
    pub dominance: Option<DominanceTriple>,
}

/// The carrier a basin's measure lives on (§11.1: measures are
/// carrier-relative — a coverage rate is only meaningful against the
/// decisions *eligible* for the verdict, never against the whole domain,
/// which may appear as labeled context but never as a rate base).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CarrierLabel {
    /// Domain decisions where the verdict's selectors resolve to legal
    /// (and, for refutation, distinct) actions, times their fiber worlds.
    SelectorPairs,
    /// Viewer-lead trick-boundary kernels at horizon <= 2 — the decisions
    /// whose §12.6 carrier trees the checker verdict is defined on.
    LeadKernelTrees,
}

impl fmt::Display for CarrierLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CarrierLabel::SelectorPairs => {
                f.write_str("selector-resolvable decisions x their fiber worlds")
            }
            CarrierLabel::LeadKernelTrees => f.write_str("lead-kernel trees (ply 0, horizon <= 2)"),
        }
    }
}

/// The measured basin: every domain decision matching the final implicant,
/// verified under the verdict's quantifier. Matched and verified coincide
/// by construction — the generalizer only accepts verified widenings —
/// and the report records both totals and the per-decision detail. A
/// basin is only meaningful at its label (§12.4): it is quotable solely
/// through its `Lesson`, whose `grade` stamps the operator pair — the
/// same measurement at another pair can shatter or merge. Its rate base
/// is the verdict's own carrier (§11.1): `decisions_eligible` /
/// `worlds_eligible` count the carrier, and the full-domain totals are
/// labeled context only.
#[derive(Clone, Debug)]
pub struct BasinReport {
    pub domain: DomainSpec,
    pub domain_decisions: usize,
    pub domain_worlds: usize,
    /// In-trick-range decision points excluded by the domain's fiber cap
    /// (never sampled — outside the verified scope entirely).
    pub domain_excluded: usize,
    pub carrier: CarrierLabel,
    /// Domain decisions eligible for this verdict on its own carrier,
    /// independent of the implicant.
    pub decisions_eligible: usize,
    /// Those decisions' fiber worlds.
    pub worlds_eligible: usize,
    pub decisions_matched: usize,
    pub worlds_matched: usize,
    /// Eligible decisions whose *decision-sort* cells (identity, frame,
    /// horizon bounds) hold under the final implicant — the denominator
    /// of the basin/frame-compatible rate: how much selection work the
    /// world-sort (atom) cells do beyond the frame.
    pub frame_compatible: usize,
    /// The dominance primitive summed over every matched world; `None`
    /// for the checker verdict.
    pub dominance: Option<DominanceTriple>,
    pub matched: Vec<MatchedDecision>,
}

/// Milestone-1 rent (S5c-m1): coarse purpose-specific ranking statistics
/// measured through the gated application entry point — a lesson's
/// usefulness on the corpus, not yet the S5c database economy (no
/// watched-feature indexing, no deletion). Purpose-specificity is typed:
/// refutation rent is never paid in T-coverage — only strict-somewhere
/// applications count, and the improvement is the exact mean
/// matched-world gain (better minus worse, uniform over matched worlds)
/// summed over those decisions.
#[derive(Clone, Debug)]
pub enum RentReport {
    Refutation {
        applied: usize,
        strict_applied: usize,
        improvement: walt_geom::Q,
    },
    Win {
        applied: usize,
        worlds_covered: usize,
        actions_pruned: usize,
    },
    Checker {
        applied: usize,
    },
}

impl fmt::Display for RentReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RentReport::Refutation {
                applied,
                strict_applied,
                improvement,
            } => write!(
                f,
                "refutation rent: applied {applied} strict-applied {strict_applied} improvement {improvement}"
            ),
            RentReport::Win {
                applied,
                worlds_covered,
                actions_pruned,
            } => write!(
                f,
                "win rent: applied {applied} worlds-covered {worlds_covered} actions-pruned {actions_pruned}"
            ),
            RentReport::Checker { applied } => write!(f, "checker rent: applied {applied}"),
        }
    }
}

/// A lesson: origin, verdict, grade, implicants (initial and final), the
/// widening trace, and the measured basin. Constructed only by the
/// generalizer, so every field is the record of an actual verification.
#[derive(Clone, Debug)]
pub struct Lesson {
    pub origin: LessonOrigin,
    pub verdict: LessonVerdict,
    pub grade: LessonGrade,
    pub initial: Implicant,
    pub trace: Vec<TraceStep>,
    pub implicant: Implicant,
    pub basin: BasinReport,
}

impl Lesson {
    /// The surviving cells: initial cells that survived their drop
    /// attempt — restored equality cells and held bounds (at their held
    /// values), in trace order. A derived view of the trace, never stored
    /// twice. (m2 vocabulary: "surviving" beside "introduced"; together
    /// they are the selecting cells.)
    pub fn surviving(&self) -> Vec<Constraint> {
        self.trace
            .iter()
            .filter_map(|s| match s {
                TraceStep::Drop {
                    cell,
                    outcome: StepOutcome::Survives(_),
                } => Some(*cell),
                TraceStep::Drop {
                    outcome: StepOutcome::BoundHeld { held, .. },
                    ..
                } => Some(*held),
                _ => None,
            })
            .collect()
    }

    /// The cut-refinement cells introduced during the kept pass, in trace
    /// order. A derived view of the trace.
    pub fn introduced(&self) -> Vec<Constraint> {
        self.trace
            .iter()
            .filter_map(|s| match s {
                TraceStep::Introduce { cell, .. } => Some(*cell),
                _ => None,
            })
            .collect()
    }

    /// The selecting cells of the final implicant: surviving union
    /// introduced (the two ways a cell ends up doing work).
    pub fn selecting_cells(&self) -> Vec<Constraint> {
        let mut out = self.surviving();
        out.extend(self.introduced());
        out
    }

    /// Equality-in-disguise bound pairs in the final implicant: numerics
    /// pinned by both `>= k` and `<= k` at the same value. Flagged
    /// `re-pinned` and EXCLUDED from any "atoms select" count (m2
    /// adjudication): a re-pinned pair re-describes its origin at
    /// equality tightness and carries no relaxation content.
    pub fn re_pinned(&self) -> Vec<(NumericAtom, u8)> {
        let mut out = Vec::new();
        for c in &self.implicant.cells {
            if let Constraint::NumericGe(a, k) = c {
                if self
                    .implicant
                    .cells
                    .contains(&Constraint::NumericLe(*a, *k))
                    && !out.contains(&(*a, *k))
                {
                    out.push((*a, *k));
                }
            }
        }
        out
    }

    /// Atom-sort cells of the final implicant that genuinely select —
    /// members of re-pinned pairs excluded. The falsification question's
    /// numerator shape: is the latent vocabulary doing selection work?
    pub fn selecting_atom_cells(&self) -> Vec<Constraint> {
        let re_pinned = self.re_pinned();
        self.implicant
            .cells
            .iter()
            .filter(|c| match c {
                Constraint::Atom(..) => true,
                Constraint::NumericGe(a, k) | Constraint::NumericLe(a, k) => {
                    !re_pinned.contains(&(*a, *k))
                }
                _ => false,
            })
            .copied()
            .collect()
    }

    /// The basin's derived dominance class, for refutation lessons with a
    /// nonempty basin. A T outcome means the lesson's domain-restricted
    /// content is an interchangeability statement, not a refutation
    /// (§9.6), with §10.9's seat-facing caveat.
    pub fn dominance_class(&self) -> Option<DominanceClass> {
        if !matches!(self.verdict, LessonVerdict::Refutation { .. }) {
            return None;
        }
        let t = self.basin.dominance?;
        (t.worlds() > 0).then(|| t.class())
    }

    /// The purpose-split subbasins of a refutation lesson (§9.6 is
    /// purpose-relative), derived per matched decision from its triple:
    /// `(refutation, safe_substitution)` decision counts. The refutation
    /// subbasin holds the decisions strict somewhere (the pruning-grade
    /// content); the safe-substitution subbasin holds every matched
    /// decision — weak dominance is verified there, so substituting the
    /// better selector for the worse loses nothing in any matched world,
    /// and T-coverage counts. S5c rent is purpose-specific: a refutation
    /// lesson may not pay rent in T-coverage.
    pub fn subbasins(&self) -> Option<(usize, usize)> {
        if !matches!(self.verdict, LessonVerdict::Refutation { .. }) {
            return None;
        }
        let strict = self
            .basin
            .matched
            .iter()
            .filter(|m| m.dominance.is_some_and(|t| t.gt > 0))
            .count();
        Some((strict, self.basin.decisions_matched))
    }
}
