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
pub mod conflict;
pub mod corpus;
pub mod generalize;
pub mod lesson;
pub mod lesson_report;
pub mod report;
pub mod walker;

pub use basin::{valued_tile, vocabulary, BasinDomain, DomainDecision, DomainSpec};
pub use conflict::{Conflict, Grade, RegretConflict};
pub use corpus::{load_receipt, walk_corpus};
pub use generalize::{generalize_lumpability, generalize_regret, generalize_win, lesson_applies};
pub use lesson::{
    ActionSelector, AtomValue, BasinReport, CarrierLabel, Constraint, DescriptorFamily,
    DominanceClass, DominanceTriple, DropOutcome, DropStep, FieldLabel, FocalInfoLabel, Implicant,
    Lesson, LessonAtom, LessonGrade, LessonOrigin, LessonVerdict, MatchedDecision, OperatorPair,
    Role, WideningWitness,
};
pub use lesson_report::{lesson_pin_line, render_lesson};
pub use report::{corpus_pin_line, render_walk};
pub use walker::{
    walk_decision, walk_seat, DecisionRecord, EvidenceBasis, HandSeatWalk, LostVerdict,
    WalkerConfig,
};
