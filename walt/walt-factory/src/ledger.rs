//! The rent ledger (S5c-m3): purpose-specific rent in a dual ledger,
//! per-epoch records, deletion, readmission, restart-with-retention.
//!
//! **Dual-ledger, H-primary, never summed** (walt-math Fork-3 ruling):
//! the economy's pricing currency is seat-facing (H, fixed-uniform-legal)
//! rent; (C, minimax-omniscient) rent is a recorded diagnostic ledger.
//! The two are never added, averaged, or compared as one scalar — every
//! row carries its full operator-pair label and the two measurements are
//! separate fields with separate types. **Unmeasured is not zero**: a
//! budget-capped H measurement is `Unmeasured`, never `Measured(0)`;
//! deletion triggers fire only on MEASURED zeros at the pricing label.
//! Vocabulary: H-measured lessons are *priced*; H-capped lessons are
//! *provisionally held* (they live on their diagnostic (C) ledger,
//! label-provisional); the checker lesson is *not re-measured* (§12.6
//! already lives at the fixed field) and is priced in its own
//! applied-count ledger at its own label. No (C)-basis deletion path
//! exists in ledger-v1; per the adjudication, any future one must cite
//! its labels verbatim in the deletion record.
//!
//! **Deletion is an economy action, never an evidence action** (Fork 5):
//! only working-set membership changes; the archive keeps everything. The
//! epoch unit and the deletion threshold `N` are declared constants
//! restated in every ledger header; the trigger property is
//! MEASURED-CONSECUTIVE (adjudicated vocabulary — never bare
//! "consecutive"): **zero rent in the last `N` MEASURED epochs, no
//! intervening measured nonzero**. Only evidence moves the count, in
//! either direction — a capped epoch neither advances nor resets it, so
//! a capped-every-other-epoch lesson is not shielded forever. No decay
//! factors exist. Every deletion, block, and readmission emits a record
//! citing the lesson's canonical content key, the pricing label, the
//! evidence pattern INCLUDING gaps ("measured-zero e1, e3; e2 capped at
//! <budget, semantics>"), and the ledger version — deletions are
//! reproducible from the ledger the way verdicts are from traces.
//!
//! **The H-checker sequencing law** (Fork 6): certificates emit H records
//! marked UNCHECKED-EXTERNALLY today, and the FIRST deletion driven by H
//! rent is mechanically blocked until an independent H checker is
//! registered. The block is enforced by type: executing an H-priced
//! deletion requires an `HCheckerToken`, and the only constructor is
//! `HCheckerRegistry::token`, which returns one exactly when a checker is
//! registered. An empty registry can only produce `DeletionBlocked`
//! records. The walt-math amendment (adjudicated, per-row form) composes
//! with the block: every H rent row carries an at-collection coverage
//! stamp (`HValueCoverage`, append-only historical record), lesson-level
//! coverage is display only, and CLEARANCE IS PER ROW — an independent
//! checker re-derives a cited row's figures and a `ClearanceRecord` is
//! appended beside the row. The deletion clause requires every cited row
//! to have been collected under cleared coverage or to carry an appended
//! clearance; no deletion decision may cite a single-implementation rent
//! figure.
//!
//! **Rent attribution under overlap** (Fork 7): every price is STANDALONE
//! rent per lesson — deterministic and order-free; marginal/leave-one-out
//! rent is schedule-dependent and forbidden as a primary price. Overlap is
//! RECORDED instead: per decision the set of applying lessons, per lesson
//! its sole-applier decision count. Standalone rents are never summed into
//! a "total DB value" — overlap makes that number double-count (§5.4:
//! a union is not a mixture).

use std::collections::BTreeMap;

use walt_core::receipt::Receipt;
use walt_core::{Domino, Seat};
use walt_geom::{qi, Q};
use walt_kernel::ReceiptDecision;
use walt_strat::{ScalarHidden, ScalarValuation};

use crate::basin::BasinDomain;
use crate::db::{ContentKey, LessonDb};
use crate::index::{appliers, WatchIndex};
use crate::label_transfer::BudgetSemantics;
use crate::lesson::{FieldLabel, FocalInfoLabel, Lesson, LessonVerdict, OperatorPair, RentReport};
use walt_strat::MemoStats;

/// The ledger format version, cited in every economy record.
pub const LEDGER_VERSION: &str = "ledger-v1 (S5c-m3)";

/// Deletion threshold: the measured-consecutive requirement — zero rent
/// in the last `N` MEASURED epochs, no intervening measured nonzero.
/// Rationale (declared, restated in headers): at this corpus scale one
/// zero epoch cannot distinguish an idle lesson from a dead one — the
/// same deterministic pass must confirm it once before the economy acts;
/// capped epochs are cited as gaps, never as evidence.
pub const DELETION_EPOCHS_N: usize = 2;

/// The declared epoch unit: one full rent-collection pass over the
/// declared corpus domain.
pub const EPOCH_UNIT: &str = "one full rent-collection pass over the declared corpus domain";

/// Per-decision H budget and its semantics — the measurability envelope,
/// part of every H claim (S5c-m2 values, unchanged). tree-v0.
pub const H_BUDGET_PARTICLE_STEPS: u64 = 100_000_000;
pub const H_BUDGET_SEMANTICS: &str = "particle-steps-v1 (S5c-m2): one budget unit per (particle, \
     node) visit; an over-budget solve returns nothing — exclusion, never sampling";
pub const H_CACHE_CONFIG: &str = "none (S5c-m2 recursive solver, no memoization)";

/// The dag-v1 envelope (S5c-m3c, the declared raised budget of the r3
/// supplement): one budget unit per (particle, node) visit actually
/// computed on the memoized DAG — a pooled-state cache hit costs zero by
/// unit definition. A lesson capped at tree-v0 and measured at dag-v1 is
/// a SEMANTICS CHANGE, never the same statistic improving.
pub const H_DAG_BUDGET_PARTICLE_STEPS: u64 = 1_000_000_000;
pub const H_DAG_SEMANTICS: &str = "dag-v1 (S5c-m3): one budget unit per (particle, node) visit \
     computed on the memoized DAG; cache hits cost zero by unit definition; an over-budget solve \
     returns nothing — exclusion, never sampling";
pub const H_DAG_CACHE_CONFIG: &str = "pooled-state boundary memoization, fresh per measurement \
     call of one solver frame, unbounded, freed between calls (per-call scope — incidental \
     warmth cannot exist)";

/// The cache-configuration string of one budget semantics.
pub fn cache_config(semantics: BudgetSemantics) -> &'static str {
    match semantics {
        BudgetSemantics::TreeV0 => H_CACHE_CONFIG,
        BudgetSemantics::DagV1 => H_DAG_CACHE_CONFIG,
    }
}

/// The full budget-semantics description of one semantics.
pub fn semantics_description(semantics: BudgetSemantics) -> &'static str {
    match semantics {
        BudgetSemantics::TreeV0 => H_BUDGET_SEMANTICS,
        BudgetSemantics::DagV1 => H_DAG_SEMANTICS,
    }
}

/// The pricing label: seat-facing H against the §7.4 fixed uniform-legal
/// field, root weighting uniform-over-fiber.
pub fn pricing_label() -> OperatorPair {
    OperatorPair {
        focal_info: FocalInfoLabel::H,
        field: FieldLabel::FixedUniformLegal,
    }
}

/// The diagnostic label: the walker's (C, minimax-omniscient) scalar.
pub fn diagnostic_label() -> OperatorPair {
    OperatorPair {
        focal_info: FocalInfoLabel::C,
        field: FieldLabel::MinimaxOmniscient,
    }
}

// ---------------------------------------------------------------------------
// The H-checker registry (the sequencing-law block).

/// A registered independent H checker (a non-Rust implementation that can
/// re-verify H value rows). None exists today; a Python checker is a
/// planned separate work unit.
#[derive(Clone, Debug)]
pub struct HCheckerDesc {
    pub name: String,
}

/// Possession of a token proves a checker is registered — the only
/// constructor is `HCheckerRegistry::token`.
pub struct HCheckerToken {
    _priv: (),
}

/// Checker coverage of the H figures behind one rent row, stamped at
/// collection time (walt-math amendment, per-row form): the at-collection
/// stamp is a HISTORICAL RECORD — append-only, never rewritten — so a
/// future ledger reader sees which figures were single-implementation
/// when recorded. Lesson-level coverage is display only; **clearance is
/// PER ROW**: an independent checker re-derives a cited row's figures and
/// a `ClearanceRecord` is APPENDED beside the row (the stamp stands). The
/// deletion clause requires every cited row to either have been collected
/// under cleared coverage or carry an appended clearance record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HValueCoverage {
    /// The figures exist in one implementation only (today: the Rust
    /// solvers — the budgeted tree walk and the memoized DAG path are one
    /// implementation lineage, not independent checks). **No deletion
    /// decision may cite a single-implementation rent figure** unless a
    /// clearance record has been appended for that row. Applied
    /// conservatively: even a zero-applied measured zero carries the
    /// marking, since the applicability gate is also single-implementation
    /// (declared choice; the checker obligation for any cited row is
    /// applied-decision-SET equality over the row's declared domain —
    /// the empty set for zero-applied rows).
    SingleImplementation,
    /// Collected while the row's figures were already independently
    /// re-derived (no current path stamps this; it exists for a future
    /// continuously-checked regime and for the deletion clause's
    /// "collected under cleared coverage" arm).
    IndependentlyReverified,
}

impl core::fmt::Display for HValueCoverage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HValueCoverage::SingleImplementation => f.write_str(
                "SINGLE-IMPLEMENTATION (Rust only; no independent re-derivation of these \
                 figures — a deletion citing this row needs an appended clearance record)",
            ),
            HValueCoverage::IndependentlyReverified => {
                f.write_str("INDEPENDENTLY-REVERIFIED at collection")
            }
        }
    }
}

/// A per-row clearance (walt-math amendment): a registered independent
/// checker re-derived the figures of one rent row, identified by (epoch,
/// content-key hash). Appended beside the row — the at-collection stamp
/// is never rewritten. Only constructible through
/// `Ledger::append_clearance`, which demands an `HCheckerToken`.
#[derive(Clone, Debug)]
pub struct ClearanceRecord {
    pub epoch: usize,
    pub key_hash: u64,
    pub checker: String,
}

#[derive(Default)]
pub struct HCheckerRegistry {
    registered: Vec<HCheckerDesc>,
}

impl HCheckerRegistry {
    /// Today's honest state: nothing registered.
    pub fn none_registered() -> HCheckerRegistry {
        HCheckerRegistry::default()
    }

    pub fn register(&mut self, desc: HCheckerDesc) {
        self.registered.push(desc);
    }

    pub fn registered(&self) -> &[HCheckerDesc] {
        &self.registered
    }

    /// `Some` exactly when an independent H checker is registered — the
    /// capability an H-priced deletion (and any clearance append) must
    /// present.
    pub fn token(&self) -> Option<HCheckerToken> {
        (!self.registered.is_empty()).then_some(HCheckerToken { _priv: () })
    }
}

// ---------------------------------------------------------------------------
// H measurement detail (shared by rent rows and certificate emission).

/// One applied decision's H outcome, with the full per-action value row
/// where solved (certificates read these; rent reads the projections).
#[derive(Clone, Debug)]
pub enum HRowOutcome {
    /// Exact Q^H per legal action, ascending tile order.
    Solved { values: Vec<(Domino, Q)> },
    /// The lesson's atom cells are not valid over this decision's whole
    /// fiber: the H-shaped claim (per-decision quantifier) does not extend
    /// here; contributes no H rent, recorded.
    NotFiberValid,
    /// Budget exhausted: excluded from the H measurement, never sampled.
    Capped,
}

#[derive(Clone, Debug)]
pub struct HRow {
    pub hand: usize,
    pub seat: Seat,
    pub trick_no: usize,
    pub ply: usize,
    pub fiber: usize,
    pub outcome: HRowOutcome,
    /// dag-v1 cache statistics of this row's measurement call, where the
    /// solver reports them (`None` under tree semantics) — tree-equiv
    /// provenance for results files, never verdict input.
    pub memo: Option<MemoStats>,
}

/// The full H measurement of one lesson's gate-applied decision set. The
/// measurability envelope — budget value AND budget semantics — is part
/// of the claim and travels with the detail.
#[derive(Clone, Debug)]
pub struct HLessonDetail {
    pub rows: Vec<HRow>,
    pub budget_per_decision: u64,
    pub semantics: BudgetSemantics,
}

/// Measures one value lesson at (H, fixed-uniform-legal) over every
/// domain decision the gate accepts, under tree-v0 budget semantics.
/// `None` for the checker verdict — not a value comparison, not
/// re-measured.
pub fn measure_h_detail(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
) -> Option<HLessonDetail> {
    measure_h_detail_at(
        lesson,
        domain,
        receipt,
        budget_per_decision,
        BudgetSemantics::TreeV0,
    )
}

/// The same measurement under dag-v1 budget semantics (the memoized DAG;
/// rows carry the solver's cache statistics as tree-equiv provenance).
pub fn measure_h_detail_dag(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
) -> Option<HLessonDetail> {
    measure_h_detail_at(
        lesson,
        domain,
        receipt,
        budget_per_decision,
        BudgetSemantics::DagV1,
    )
}

fn measure_h_detail_at(
    lesson: &Lesson,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
    semantics: BudgetSemantics,
) -> Option<HLessonDetail> {
    if matches!(lesson.verdict, LessonVerdict::NotLumpable { .. }) {
        return None;
    }
    let mut rows = Vec::new();
    for d in &domain.decisions {
        let Some(matched) = crate::generalize::lesson_applies(lesson, d) else {
            continue;
        };
        let mut row = HRow {
            hand: d.hand,
            seat: d.seat,
            trick_no: d.trick_no,
            ply: d.ply,
            fiber: d.worlds.len(),
            outcome: HRowOutcome::Capped,
            memo: None,
        };
        if matched.len() != d.worlds.len() {
            row.outcome = HRowOutcome::NotFiberValid;
            rows.push(row);
            continue;
        }
        let rd = ReceiptDecision::at(&receipt.hands[d.hand], d.trick_no, d.seat)
            .expect("a domain decision reconstructs");
        let solver = ScalarHidden::new(
            d.kernel.decl(),
            d.kernel.viewer(),
            d.kernel.viewer().team(),
            ScalarValuation::trick_plus_count(),
        );
        let hands: Vec<[walt_core::DominoSet; Seat::COUNT]> =
            d.worlds.iter().map(|w| w.hands()).collect();
        let mut budget = budget_per_decision;
        match semantics {
            BudgetSemantics::TreeV0 => {
                if let Some(values) =
                    solver.action_values(&hands, rd.leader, &rd.prefix, &mut budget)
                {
                    row.outcome = HRowOutcome::Solved { values };
                }
            }
            BudgetSemantics::DagV1 => {
                let (values, stats) =
                    solver.action_values_dag(&hands, rd.leader, &rd.prefix, &mut budget);
                row.memo = Some(stats);
                if let Some(values) = values {
                    row.outcome = HRowOutcome::Solved { values };
                }
            }
        }
        rows.push(row);
    }
    Some(HLessonDetail {
        rows,
        budget_per_decision,
        semantics,
    })
}

// ---------------------------------------------------------------------------
// Rent measurements (the pricing side).

/// Purpose-specific H rent — the pricing content of a measured epoch.
/// At H the quantifier is per decision (one inequality per matching
/// decision), so counts are decision counts and `improvement` is the sum
/// of per-decision `Q^H(better) - Q^H(worse)` over strict decisions —
/// never paid in ties (T-coverage), per the purpose split.
#[derive(Clone, Debug)]
pub enum HRent {
    Refutation {
        applied: usize,
        strict: usize,
        tied: usize,
        /// Decisions where the H inequality FAILS — a label-transfer
        /// conflict, reported loudly, paying nothing.
        failed: usize,
        improvement: Q,
    },
    Win {
        applied: usize,
        held: usize,
        failed: usize,
        /// Sum of (legal actions - 1) over held decisions: the pruning the
        /// lesson buys at the information-state level.
        actions_pruned: usize,
        /// Fiber sizes summed over held decisions — labeled context.
        worlds_covered: usize,
    },
}

/// One lesson's pricing measurement for one epoch.
#[derive(Clone, Debug)]
pub enum RentMeasurement {
    /// Complete H measurement at the pricing label: the lesson is PRICED.
    Measured(HRent),
    /// At least one applied decision's H solve was budget-capped: the H
    /// rent is UNMEASURED — never zero — and the lesson is PROVISIONALLY
    /// HELD on its diagnostic (C) ledger, label-provisional. The
    /// measurability envelope the cap occurred under travels on the row.
    Unmeasured {
        fiber_valid_applied: usize,
        capped_decisions: usize,
        not_fiber_valid: usize,
        budget: u64,
        semantics_id: &'static str,
    },
    /// The checker lesson: not re-measured at H (§12.6 already lives at
    /// the fixed field); priced in its own applied-count ledger at its own
    /// label.
    CheckerOwnLedger { applied: usize },
}

impl RentMeasurement {
    /// `Some(true)` exactly for a MEASURED zero at the pricing ledger —
    /// the only evidence a deletion trigger may consume. `None` when
    /// unmeasured (capped): never counts toward deletion. A verdict that
    /// FAILS at the H label collects zero rent at the failed decisions —
    /// that is a measured zero (it counts toward measured-consecutive),
    /// never an unmeasured one; `zero_reason` says why.
    pub fn measured_zero(&self) -> Option<bool> {
        match self {
            RentMeasurement::Measured(HRent::Refutation { improvement, .. }) => {
                Some(*improvement == qi(0))
            }
            RentMeasurement::Measured(HRent::Win { actions_pruned, .. }) => {
                Some(*actions_pruned == 0)
            }
            RentMeasurement::CheckerOwnLedger { applied } => Some(*applied == 0),
            RentMeasurement::Unmeasured { .. } => None,
        }
    }

    /// WHY a measured zero is zero — the ledger row's own explanation,
    /// distinguishing verdict-failed-at-label from applied-but-no-
    /// improvement from not-applied. `None` unless `measured_zero()` is
    /// `Some(true)`. A derived view of the stored counts, never a second
    /// authority.
    pub fn zero_reason(&self) -> Option<String> {
        if self.measured_zero() != Some(true) {
            return None;
        }
        Some(match self {
            RentMeasurement::Measured(HRent::Refutation {
                applied,
                tied,
                failed,
                ..
            }) => {
                if *applied == 0 {
                    "not applied: no domain decision matched".to_string()
                } else if *failed > 0 {
                    format!(
                        "verdict FAILED at the H label at {failed} of {applied} applied \
                         decisions (label-transfer conflict); the rest tied ({tied}) — zero \
                         rent, measured"
                    )
                } else {
                    format!(
                        "applied but no improvement: all {tied} applied decisions tied at H — \
                         refutation rent is never paid in T-coverage"
                    )
                }
            }
            RentMeasurement::Measured(HRent::Win {
                applied, failed, ..
            }) => {
                if *applied == 0 {
                    "not applied: no domain decision matched".to_string()
                } else if *failed > 0 {
                    format!(
                        "verdict FAILED at the H label at {failed} of {applied} applied \
                         decisions (the selector action does not attain the H max) — zero rent, \
                         measured"
                    )
                } else {
                    "applied but no pruning: every held decision offers a single legal action"
                        .to_string()
                }
            }
            RentMeasurement::CheckerOwnLedger { .. } => {
                "not applied: no eligible lead kernel matched".to_string()
            }
            RentMeasurement::Unmeasured { .. } => unreachable!("filtered above"),
        })
    }
}

fn value_of(values: &[(Domino, Q)], tile: Domino) -> Q {
    values
        .iter()
        .find(|(a, _)| *a == tile)
        .expect("selector actions are legal")
        .1
}

// ---------------------------------------------------------------------------
// Epoch collection.

/// One lesson's row in one epoch's dual ledger. `pricing` and
/// `diagnostic` are separate typed fields — no sum, no average, no shared
/// scalar exists anywhere in this module.
pub struct LessonEpochRecord {
    pub entry: usize,
    pub key_hash: u64,
    /// Human display only (origin summary); identity is the content key.
    pub name: String,
    pub pricing_label: String,
    pub pricing: RentMeasurement,
    pub diagnostic_label: String,
    /// Milestone-1 purpose-specific rent at (C, minimax-omniscient) — the
    /// recorded diagnostic ledger.
    pub diagnostic: RentReport,
    /// Gate-level applied decisions (overlap basis, label-free
    /// applicability).
    pub applied_decisions: usize,
    /// Decisions where this lesson was the ONLY applier (Fork 7).
    pub sole_applier_decisions: usize,
    /// The at-collection coverage stamp of this row's H figures — a
    /// historical record, append-only, never rewritten (walt-math
    /// amendment). `None` only for the checker lesson's own-ledger row,
    /// whose figures are not H values. Lesson-level coverage is display
    /// only; the deletion clause reads this stamp OR an appended per-row
    /// `ClearanceRecord`, never anything lesson-level.
    pub h_value_coverage: Option<HValueCoverage>,
}

/// One epoch: the declared unit is `EPOCH_UNIT`. The measurability
/// envelope (budget value + budget semantics) is one declaration per
/// epoch, restated here and on every unmeasured row.
pub struct EpochLedger {
    pub number: usize,
    pub records: Vec<LessonEpochRecord>,
    /// Decisions with two or more applying lessons: (decision tag, the
    /// applying entries' key hashes) — the recorded overlap.
    pub overlap: Vec<(String, Vec<u64>)>,
    /// Per-entry H detail (kept for certificate emission; evidence, not a
    /// second rent authority).
    pub h_details: BTreeMap<usize, HLessonDetail>,
    pub budget_per_decision: u64,
    pub semantics: BudgetSemantics,
}

/// Runs one full rent-collection pass (= one epoch) over the domain,
/// under the declared budget semantics (tree-v0 or dag-v1).
/// Deterministic: iteration is domain order and working-set admission
/// order; every quantity is an exact solve or count.
pub fn collect_epoch_at(
    number: usize,
    db: &LessonDb,
    index: &WatchIndex,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
    semantics: BudgetSemantics,
) -> EpochLedger {
    assert!(
        index.is_current(db),
        "a stale index must be rebuilt before rent collection"
    );
    // Overlap: per decision, the applying working-set lessons through the
    // index + full gate.
    let mut applied: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sole: BTreeMap<usize, usize> = BTreeMap::new();
    let mut overlap = Vec::new();
    for d in &domain.decisions {
        let hits = appliers(index, db, d);
        for (entry, _) in &hits {
            *applied.entry(*entry).or_default() += 1;
        }
        if hits.len() == 1 {
            *sole.entry(hits[0].0).or_default() += 1;
        }
        if hits.len() >= 2 {
            overlap.push((
                format!("h{} {} t{} p{}", d.hand, d.seat, d.trick_no, d.ply),
                hits.iter()
                    .map(|(e, _)| ContentKey::of(db.representative(*e)).hash())
                    .collect(),
            ));
        }
    }

    let mut records = Vec::new();
    let mut h_details = BTreeMap::new();
    for (entry, lesson) in db.working() {
        let key_hash = ContentKey::of(lesson).hash();
        let diagnostic = crate::generalize::measure_rent(lesson, domain);
        let mut h_value_coverage = None;
        let pricing =
            match measure_h_detail_at(lesson, domain, receipt, budget_per_decision, semantics) {
                None => RentMeasurement::CheckerOwnLedger {
                    applied: applied.get(&entry).copied().unwrap_or(0),
                },
                Some(detail) => {
                    let rent = h_rent(lesson, &detail, domain);
                    h_details.insert(entry, detail);
                    // Today's honest stamp: every H figure is produced by the
                    // one Rust implementation lineage (the tree and DAG paths
                    // are one lineage, not independent checks). No current
                    // path stamps IndependentlyReverified; clearance is per
                    // row, appended later beside the row, never rewriting
                    // this.
                    h_value_coverage = Some(HValueCoverage::SingleImplementation);
                    rent
                }
            };
        records.push(LessonEpochRecord {
            entry,
            key_hash,
            name: display_name(lesson),
            pricing_label: match &pricing {
                RentMeasurement::CheckerOwnLedger { .. } => {
                    "checker applied-count ledger (§12.6, fixed-uniform-legal field, q_points)"
                        .to_string()
                }
                _ => format!("{}, uniform-over-fiber", pricing_label()),
            },
            pricing,
            diagnostic_label: format!("{}, uniform-over-fiber", diagnostic_label()),
            diagnostic,
            applied_decisions: applied.get(&entry).copied().unwrap_or(0),
            sole_applier_decisions: sole.get(&entry).copied().unwrap_or(0),
            h_value_coverage,
        });
    }
    EpochLedger {
        number,
        records,
        overlap,
        h_details,
        budget_per_decision,
        semantics,
    }
}

/// `collect_epoch_at` under tree-v0 semantics — the m3-B entry point,
/// kept for callers and pins that predate the dag path.
pub fn collect_epoch(
    number: usize,
    db: &LessonDb,
    index: &WatchIndex,
    domain: &BasinDomain,
    receipt: &Receipt,
    budget_per_decision: u64,
) -> EpochLedger {
    collect_epoch_at(
        number,
        db,
        index,
        domain,
        receipt,
        budget_per_decision,
        BudgetSemantics::TreeV0,
    )
}

/// Prices one value lesson from its H detail (the checker lesson takes
/// the own-ledger path in `collect_epoch`). Selector resolution runs
/// against the domain decision each row names — the row's legal set plus
/// that decision's decisive tile.
pub fn h_rent(lesson: &Lesson, detail: &HLessonDetail, domain: &BasinDomain) -> RentMeasurement {
    let capped = detail
        .rows
        .iter()
        .filter(|r| matches!(r.outcome, HRowOutcome::Capped))
        .count();
    let nfv = detail
        .rows
        .iter()
        .filter(|r| matches!(r.outcome, HRowOutcome::NotFiberValid))
        .count();
    let solved: Vec<&HRow> = detail
        .rows
        .iter()
        .filter(|r| matches!(r.outcome, HRowOutcome::Solved { .. }))
        .collect();
    if capped > 0 {
        return RentMeasurement::Unmeasured {
            fiber_valid_applied: solved.len() + capped,
            capped_decisions: capped,
            not_fiber_valid: nfv,
            budget: detail.budget_per_decision,
            semantics_id: detail.semantics.identifier(),
        };
    }
    let decision_of = |row: &HRow| {
        domain
            .decisions
            .iter()
            .find(|d| {
                d.hand == row.hand
                    && d.seat == row.seat
                    && d.trick_no == row.trick_no
                    && d.ply == row.ply
            })
            .expect("H rows name domain decisions")
    };
    match &lesson.verdict {
        LessonVerdict::Refutation { worse, better } => {
            let mut strict = 0;
            let mut tied = 0;
            let mut failed = 0;
            let mut improvement = qi(0);
            for row in &solved {
                let HRowOutcome::Solved { values } = &row.outcome else {
                    unreachable!("filtered to solved")
                };
                let d = decision_of(row);
                let b = better
                    .resolve(&d.actions, d.decisive)
                    .expect("gate-applied");
                let w = worse.resolve(&d.actions, d.decisive).expect("gate-applied");
                let (qb, qw) = (value_of(values, b), value_of(values, w));
                if qb > qw {
                    strict += 1;
                    improvement += qb - qw;
                } else if qb == qw {
                    tied += 1;
                } else {
                    failed += 1;
                }
            }
            RentMeasurement::Measured(HRent::Refutation {
                applied: solved.len(),
                strict,
                tied,
                failed,
                improvement,
            })
        }
        LessonVerdict::Win { action } => {
            let mut held = 0;
            let mut failed = 0;
            let mut actions_pruned = 0;
            let mut worlds_covered = 0;
            for row in &solved {
                let HRowOutcome::Solved { values } = &row.outcome else {
                    unreachable!("filtered to solved")
                };
                let d = decision_of(row);
                let a = action
                    .resolve(&d.actions, d.decisive)
                    .expect("gate-applied");
                let qa = value_of(values, a);
                let best = values.iter().map(|(_, v)| *v).max().expect("actions");
                if qa == best {
                    held += 1;
                    actions_pruned += values.len() - 1;
                    worlds_covered += row.fiber;
                } else {
                    failed += 1;
                }
            }
            RentMeasurement::Measured(HRent::Win {
                applied: solved.len(),
                held,
                failed,
                actions_pruned,
                worlds_covered,
            })
        }
        LessonVerdict::NotLumpable { .. } => unreachable!("checker lessons have no H detail"),
    }
}

/// Human display name (origin summary + verdict kind) — display only,
/// never identity.
pub fn display_name(lesson: &Lesson) -> String {
    use crate::lesson::LessonOrigin;
    let origin = match &lesson.origin {
        LessonOrigin::Regret(c) => {
            format!("regret h{} {} t{} p{}", c.hand, c.seat, c.trick_no, c.ply)
        }
        LessonOrigin::Lumpability {
            hand,
            trick_no,
            descriptor,
            ..
        } => format!("lumpability h{hand} t{trick_no} {descriptor}"),
    };
    format!("{} [{}]", origin, crate::db::verdict_kind(lesson))
}

// ---------------------------------------------------------------------------
// Lifetime records, deletion, readmission, restart.

/// The decayless per-lesson lifetime record — cumulative counters only,
/// no decay factors anywhere.
#[derive(Clone, Debug, Default)]
pub struct LifetimeRecord {
    pub epochs_seen: usize,
    pub epochs_measured: usize,
    pub epochs_capped: usize,
    /// The MEASURED-CONSECUTIVE zero count (adjudicated vocabulary —
    /// never bare "consecutive"): zero rent in the last N MEASURED
    /// epochs, no intervening measured nonzero. Only evidence moves it,
    /// in either direction — a capped epoch neither advances nor resets
    /// it, so a capped-every-other-epoch lesson is not shielded forever.
    /// Reset on readmission (rent collection resumes fresh).
    pub measured_consecutive_zeros: usize,
    pub cumulative_applied: usize,
}

/// Economy actions, proof-logged (Fork 5c): reproducible from the ledger.
#[derive(Clone, Debug)]
pub enum EconomyRecord {
    DeletionTriggered {
        key_hash: u64,
        canonical: String,
        pricing_label: String,
        /// The measured-zero epochs the trigger relies on.
        measured_zero_epochs: Vec<usize>,
        /// The full evidence-pattern citation, gaps included, e.g.
        /// "measured-zero e1, e3; e2 capped at <budget, semantics>" —
        /// the ledger shows the pattern, not just the count.
        citation: String,
        ledger_version: &'static str,
    },
    /// A block fired (sequencing law or the per-row clearance clause):
    /// the deletion was NOT executed.
    DeletionBlocked {
        key_hash: u64,
        reason: String,
    },
    DeletionExecuted {
        key_hash: u64,
        canonical: String,
        pricing_label: String,
        measured_zero_epochs: Vec<usize>,
        citation: String,
        ledger_version: &'static str,
        tie_break: String,
    },
    Readmission {
        key_hash: u64,
        canonical: String,
        at_epoch: usize,
        ledger_version: &'static str,
    },
    Restart {
        after_epoch: usize,
        note: String,
    },
    Merge {
        key_hash: u64,
        derivations: usize,
    },
}

/// The running ledger: epochs, lifetimes (keyed by content-key hash so
/// they survive deletion), per-row clearance records, and the action log.
#[derive(Default)]
pub struct Ledger {
    pub epochs: Vec<EpochLedger>,
    pub lifetimes: BTreeMap<u64, LifetimeRecord>,
    /// Per-row clearances, appended beside the rows they reference —
    /// at-collection stamps are never rewritten.
    pub clearances: Vec<ClearanceRecord>,
    pub actions: Vec<EconomyRecord>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger::default()
    }

    /// Folds one collected epoch into the lifetimes and stores it.
    pub fn push_epoch(&mut self, epoch: EpochLedger) {
        for r in &epoch.records {
            let life = self.lifetimes.entry(r.key_hash).or_default();
            life.epochs_seen += 1;
            life.cumulative_applied += r.applied_decisions;
            match r.pricing.measured_zero() {
                Some(zero) => {
                    life.epochs_measured += 1;
                    if zero {
                        life.measured_consecutive_zeros += 1;
                    } else {
                        life.measured_consecutive_zeros = 0;
                    }
                }
                None => life.epochs_capped += 1,
            }
        }
        self.epochs.push(epoch);
    }

    /// Appends a per-row clearance: a registered independent checker
    /// re-derived the figures of the (epoch, lesson) rent row. Requires
    /// the token (only a non-empty registry can mint one); the row must
    /// exist and be H-priced. The at-collection stamp is untouched.
    pub fn append_clearance(
        &mut self,
        epoch: usize,
        key_hash: u64,
        checker: String,
        _token: &HCheckerToken,
    ) {
        let row_exists = self
            .epochs
            .iter()
            .filter(|e| e.number == epoch)
            .flat_map(|e| e.records.iter())
            .any(|r| r.key_hash == key_hash && r.h_value_coverage.is_some());
        assert!(
            row_exists,
            "a clearance references an existing H-priced rent row"
        );
        self.clearances.push(ClearanceRecord {
            epoch,
            key_hash,
            checker,
        });
    }

    /// Is the (epoch, lesson) rent row deletion-citable? Either it was
    /// collected under cleared coverage (the at-collection stamp) or a
    /// clearance record has been appended beside it.
    fn row_cleared(&self, epoch: usize, key_hash: u64) -> bool {
        let stamped = self
            .epochs
            .iter()
            .filter(|e| e.number == epoch)
            .flat_map(|e| e.records.iter())
            .any(|r| {
                r.key_hash == key_hash
                    && r.h_value_coverage == Some(HValueCoverage::IndependentlyReverified)
            });
        stamped
            || self
                .clearances
                .iter()
                .any(|c| c.epoch == epoch && c.key_hash == key_hash)
    }

    /// The evidence pattern behind one lesson's trigger: walking back
    /// from the latest epoch, the epochs of the last
    /// `DELETION_EPOCHS_N` MEASURED zeros (ascending) plus any capped
    /// epochs interleaved among them, rendered as the citation string —
    /// "measured-zero e1, e3; e2 capped at <budget, semantics>".
    fn evidence_pattern(&self, key_hash: u64) -> (Vec<usize>, String) {
        let mut zeros: Vec<usize> = Vec::new();
        let mut capped: Vec<(usize, u64, &'static str)> = Vec::new();
        for epoch in self.epochs.iter().rev() {
            let Some(row) = epoch.records.iter().find(|r| r.key_hash == key_hash) else {
                continue;
            };
            match row.pricing.measured_zero() {
                Some(true) => zeros.push(epoch.number),
                Some(false) => break,
                None => {
                    if let RentMeasurement::Unmeasured {
                        budget,
                        semantics_id,
                        ..
                    } = &row.pricing
                    {
                        capped.push((epoch.number, *budget, semantics_id));
                    }
                }
            }
            if zeros.len() == DELETION_EPOCHS_N {
                break;
            }
        }
        zeros.reverse();
        // Capped epochs OUTSIDE the cited zero span are not part of the
        // pattern; keep those interleaved with (after the first cited
        // zero) only. Each cites its own measurability envelope.
        let first_zero = zeros.first().copied().unwrap_or(usize::MAX);
        let mut capped: Vec<(usize, u64, &'static str)> = capped
            .into_iter()
            .filter(|&(e, ..)| e > first_zero)
            .collect();
        capped.sort_unstable();
        let zero_list = zeros
            .iter()
            .map(|e| format!("e{e}"))
            .collect::<Vec<_>>()
            .join(", ");
        let citation = if capped.is_empty() {
            format!("measured-zero {zero_list}")
        } else {
            let capped_list = capped
                .iter()
                .map(|(e, budget, id)| {
                    format!("e{e} capped at {budget} particle-steps (semantics={id})")
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("measured-zero {zero_list}; {capped_list}")
        };
        (zeros, citation)
    }

    /// Evaluates the declared deletion rule after the latest epoch and
    /// executes what it may: a trigger fires on zero rent in the last
    /// `DELETION_EPOCHS_N` MEASURED epochs with no intervening measured
    /// nonzero (measured-consecutive; capped epochs are cited as gaps in
    /// the evidence pattern). An H-priced deletion executes only when (a)
    /// an `HCheckerToken` exists (sequencing law) and (b) EVERY cited
    /// measured-zero row either was collected under cleared coverage or
    /// carries an appended per-row `ClearanceRecord` — lesson-level
    /// coverage is display only and is never read here. Checker-own-ledger
    /// deletions need no H token (their pricing label is not H).
    /// Candidates are processed in the declared tie-break order (smallest
    /// implicant, then oldest certificate).
    pub fn evaluate_deletions(&mut self, db: &mut LessonDb, registry: &HCheckerRegistry) {
        let Some(last) = self.epochs.last() else {
            return;
        };
        let mut candidates: Vec<(usize, u64, bool)> = Vec::new();
        for r in &last.records {
            let life = &self.lifetimes[&r.key_hash];
            if life.measured_consecutive_zeros >= DELETION_EPOCHS_N {
                let h_priced = !matches!(r.pricing, RentMeasurement::CheckerOwnLedger { .. });
                candidates.push((r.entry, r.key_hash, h_priced));
            }
        }
        candidates.sort_by(|a, b| db.tie_break(a.0, b.0));
        let tie_break_note =
            "tie-break: smallest implicant (cell count), then oldest certificate (archive order)";
        for (entry, key_hash, h_priced) in candidates {
            let canonical = ContentKey::of(db.representative(entry)).canonical.clone();
            let (measured_zero_epochs, citation) = self.evidence_pattern(key_hash);
            assert_eq!(
                measured_zero_epochs.len(),
                DELETION_EPOCHS_N,
                "a trigger cites exactly N measured zeros"
            );
            let label = if h_priced {
                format!("{}, uniform-over-fiber", pricing_label())
            } else {
                "checker applied-count ledger (§12.6, fixed-uniform-legal field, q_points)"
                    .to_string()
            };
            self.actions.push(EconomyRecord::DeletionTriggered {
                key_hash,
                canonical: canonical.clone(),
                pricing_label: label.clone(),
                measured_zero_epochs: measured_zero_epochs.clone(),
                citation: citation.clone(),
                ledger_version: LEDGER_VERSION,
            });
            if h_priced && registry.token().is_none() {
                self.actions.push(EconomyRecord::DeletionBlocked {
                    key_hash,
                    reason: "no independent H checker is registered (sequencing law: the first \
                             deletion driven by H rent is mechanically blocked until one is)"
                        .to_string(),
                });
                continue;
            }
            // The per-row clearance clause (walt-math amendment): every
            // cited row must be deletion-citable — collected under
            // cleared coverage, or carrying an appended clearance record.
            if h_priced {
                let uncleared: Vec<usize> = measured_zero_epochs
                    .iter()
                    .copied()
                    .filter(|&e| !self.row_cleared(e, key_hash))
                    .collect();
                if !uncleared.is_empty() {
                    let list = uncleared
                        .iter()
                        .map(|e| format!("e{e}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.actions.push(EconomyRecord::DeletionBlocked {
                        key_hash,
                        reason: format!(
                            "cited rows {list} are SINGLE-IMPLEMENTATION with no appended \
                             clearance record (no deletion decision may cite a rent figure that \
                             exists in one implementation only; a registered checker must \
                             re-derive those rows' figures and append per-row clearances)"
                        ),
                    });
                    continue;
                }
            }
            let removed = db.remove_from_working(entry);
            assert!(removed, "a triggered entry is in the working set");
            self.actions.push(EconomyRecord::DeletionExecuted {
                key_hash,
                canonical,
                pricing_label: label,
                measured_zero_epochs,
                citation,
                ledger_version: LEDGER_VERSION,
                tie_break: tie_break_note.to_string(),
            });
        }
    }

    /// Records a readmission (the DB insert already re-added membership);
    /// the streak resets — rent collection resumes fresh, no re-proving.
    pub fn record_readmission(&mut self, key_hash: u64, canonical: String, at_epoch: usize) {
        if let Some(life) = self.lifetimes.get_mut(&key_hash) {
            life.measured_consecutive_zeros = 0;
        }
        self.actions.push(EconomyRecord::Readmission {
            key_hash,
            canonical,
            at_epoch,
            ledger_version: LEDGER_VERSION,
        });
    }

    /// Restart-with-retention (Fork 5d): the DB (working set + archive)
    /// and this ledger survive; search state does not. The watched-feature
    /// index is caller-owned search state — drop and rebuild it; memo
    /// tables (solver caches, kernel trees) are run-scoped by construction
    /// in this codebase and cannot survive. This records the restart.
    pub fn record_restart(&mut self, note: String) {
        let after_epoch = self.epochs.last().map(|e| e.number).unwrap_or(0);
        self.actions
            .push(EconomyRecord::Restart { after_epoch, note });
    }

    pub fn record_merge(&mut self, key_hash: u64, derivations: usize) {
        self.actions.push(EconomyRecord::Merge {
            key_hash,
            derivations,
        });
    }
}

// ---------------------------------------------------------------------------
// Rendering (deterministic; the results-file voice).

/// Renders one rent measurement with its mandated vocabulary. A measured
/// zero states its own reason (verdict-failed-at-label vs applied-but-no-
/// improvement vs not-applied).
pub fn render_measurement(m: &RentMeasurement) -> String {
    let reason = match m.zero_reason() {
        Some(r) => format!(" — measured zero ({r})"),
        None => String::new(),
    };
    match m {
        RentMeasurement::Measured(HRent::Refutation {
            applied,
            strict,
            tied,
            failed,
            improvement,
        }) => {
            let failed = if *failed > 0 {
                format!(" H-FAILED at {failed} decisions (label-transfer conflict)")
            } else {
                String::new()
            };
            format!(
                "PRICED: applied {applied} strict {strict} tied {tied} improvement {improvement}{failed}{reason}"
            )
        }
        RentMeasurement::Measured(HRent::Win {
            applied,
            held,
            failed,
            actions_pruned,
            worlds_covered,
        }) => {
            let failed = if *failed > 0 {
                format!(" H-FAILED at {failed} decisions (label-transfer conflict)")
            } else {
                String::new()
            };
            format!(
                "PRICED: applied {applied} held {held} actions-pruned {actions_pruned} \
                 (worlds-covered {worlds_covered}, context){failed}{reason}"
            )
        }
        RentMeasurement::Unmeasured {
            fiber_valid_applied,
            capped_decisions,
            not_fiber_valid,
            budget,
            semantics_id,
        } => format!(
            "PROVISIONALLY HELD: (H, fixed-uniform-legal) rent UNMEASURED — {capped_decisions} of \
             {fiber_valid_applied} fiber-valid applied decisions capped at {budget} \
             particle-steps (semantics={semantics_id}); not-fiber-valid {not_fiber_valid}; lives \
             on the (C, minimax-omniscient) diagnostic ledger, label-provisional; never counts \
             toward deletion"
        ),
        RentMeasurement::CheckerOwnLedger { applied } => format!(
            "NOT RE-MEASURED at H (§12.6 lives at the fixed field); own ledger: applied \
             {applied}{reason}"
        ),
    }
}

/// Renders one economy record.
pub fn render_record(r: &EconomyRecord) -> String {
    match r {
        EconomyRecord::DeletionTriggered {
            key_hash,
            pricing_label,
            citation,
            ledger_version,
            ..
        } => format!(
            "DELETION TRIGGERED key {key_hash:016x} on {pricing_label} rent: {citation} \
             ({ledger_version})"
        ),
        EconomyRecord::DeletionBlocked { key_hash, reason } => {
            format!("DELETION BLOCKED key {key_hash:016x}: {reason}")
        }
        EconomyRecord::DeletionExecuted {
            key_hash,
            pricing_label,
            citation,
            ledger_version,
            tie_break,
            ..
        } => format!(
            "DELETION EXECUTED key {key_hash:016x} on {pricing_label} rent: {citation} \
             ({ledger_version}; {tie_break}); archive untouched"
        ),
        EconomyRecord::Readmission {
            key_hash,
            at_epoch,
            ledger_version,
            ..
        } => format!(
            "READMISSION key {key_hash:016x} at epoch {at_epoch}: verdict never lapsed, rent \
             collection resumes without re-proving ({ledger_version})"
        ),
        EconomyRecord::Restart { after_epoch, note } => {
            format!("RESTART after epoch {after_epoch}: {note}")
        }
        EconomyRecord::Merge {
            key_hash,
            derivations,
        } => format!(
            "MERGE key {key_hash:016x}: re-derivation folded in — one working-set entry, \
             {derivations} archived derivations"
        ),
    }
}
