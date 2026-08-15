//! walt-factory — the factory layer, S5a slice: the regret walker.
//!
//! PLAN.md's crate map places corpus pipelines and the synthesis loops here,
//! importing every other crate. The S5 spine's conflict generator is the
//! factory's first module: given a transcript (a receipt hand) and a focal
//! seat, walk that seat's seven decision points; at each one, build the
//! viewer kernel from the actor-attributed legal public prefix (v0.4 §2.1,
//! mid-trick included), and evaluate every legal action by its
//! fiber-expected perfect-information continuation value (`ScalarPi`, §9.9
//! at one integer valuation).
//!
//! **What the numbers are, and are not** (§7.6, §12.4): the PI-averaged
//! action value is the information-relaxed diagnostic — best response
//! computed separately in each hidden world, then averaged. It is NOT the
//! seat's hidden value `Q^H`; the gap is the strategy-fusion gap, and it is
//! action-specific (the §14.5 record's `G^cont(2-1) ≡ 0` next to
//! `G^cont(0-0) > 0` is the concrete mechanism). Every verdict emitted here
//! therefore carries its operator label and weighting label as *data*
//! (`Grade`); worldwise dominance is weighting-free but never operator-free.
//!
//! Everything is exploratory tier: walker outputs are walt-tier regression
//! pins and typed conflict material for S5b, never promoted statuses.

pub mod basin;
pub mod certificate;
pub mod conflict;
pub mod corpus;
pub mod db;
// The two FC comparison tables are FROZEN TRANSCRIPTIONS of results filed by
// prior runs (SEP-A14(ii), FT-A28(i)), and their shape is part of what was
// checked at transcription time. They stay `const` — compile-time constants
// that no code path can mutate — and the lint that would have them written as
// `static` is answered here, at the wiring, rather than by editing a frozen
// artifact for lint shape.
#[allow(clippy::large_const_arrays)]
pub mod fc_cores;
#[allow(clippy::large_const_arrays)]
pub mod fc_kappa;
pub mod generalize;
pub mod index;
pub mod label_transfer;
pub mod ledger;
pub mod lesson;
pub mod lesson_report;
pub mod report;
pub mod walker;

pub use basin::{valued_tile, vocabulary, BasinDomain, DomainDecision, DomainSpec};
pub use certificate::{certificate_filename, emit_certificate, RECORD_KINDS, SCHEMA_VERSION};
pub use conflict::{Conflict, Grade, RegretConflict};
pub use corpus::{load_receipt, walk_corpus};
pub use db::{
    fnv64, label_projection, verdict_kind, ArchiveEntry, ContentKey, InsertOutcome, LessonDb,
};
pub use fc_cores::{CoreRow, FT_CORES};
pub use fc_kappa::{KappaRow, V11_KAPPA};
pub use generalize::{
    cell_holds_at, generalize_lumpability, generalize_regret, generalize_win, lesson_applies,
    measure_rent, INTRO_BUDGET,
};
pub use index::{appliers, WatchIndex, VOCAB_REGISTRY_VERSION};
pub use label_transfer::{
    remeasure_at_h, render_h_report, BudgetSemantics, HDecision, HOutcome, HReport,
};
pub use ledger::{
    cache_config, collect_epoch, collect_epoch_at, diagnostic_label, display_name, h_rent,
    measure_h_detail, measure_h_detail_dag, pricing_label, render_measurement, render_record,
    semantics_description, ClearanceRecord, EconomyRecord, EpochLedger, HCheckerDesc,
    HCheckerRegistry, HCheckerToken, HLessonDetail, HRent, HRow, HRowOutcome, HValueCoverage,
    Ledger, LessonEpochRecord, LifetimeRecord, RentMeasurement, DELETION_EPOCHS_N, EPOCH_UNIT,
    H_BUDGET_PARTICLE_STEPS, H_BUDGET_SEMANTICS, H_CACHE_CONFIG, H_DAG_BUDGET_PARTICLE_STEPS,
    H_DAG_CACHE_CONFIG, H_DAG_SEMANTICS, LEDGER_VERSION,
};
pub use lesson::{
    ActionSelector, AtomValue, BasinReport, CarrierLabel, Constraint, DescriptorFamily,
    DominanceClass, DominanceTriple, FieldLabel, FocalInfoLabel, Implicant, Lesson, LessonAtom,
    LessonGrade, LessonOrigin, LessonVerdict, MatchedDecision, NumericAtom, OperatorPair,
    RentReport, Role, StepOutcome, TraceStep, WideningWitness,
};
pub use lesson_report::{lesson_pin_line, render_lesson};
pub use report::{corpus_pin_line, render_walk};
pub use walker::{
    walk_decision, walk_seat, DecisionRecord, EvidenceBasis, HandSeatWalk, LostVerdict,
    WalkerConfig,
};
