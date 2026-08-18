//! Portable production semantics for the freeze-57 M3 perfect-recall net.
//!
//! This crate owns the production key domains, exact carried mass, typed net,
//! backward recurrence, fixed-policy repricer, and terminal evidence folds.
//! It deliberately contains no oracle recursion, Metal binding, persistence,
//! receipt construction, floating-point arithmetic, or unsafe code.

#![forbid(unsafe_code)]

mod counter;
mod evidence;
mod key;
mod mass;
mod net;
mod objective;
mod recurrence;

pub use counter::{
    CounterError, EdgeCounter, EmissionCounter, HighWaterKind, HighWaterRecord,
    ReductionLevelZeroCounter, VisitCounter, EDGE_CAP, EMISSION_CAP,
    REDUCTION_LEVEL_ZERO_CAP, VISIT_CAP,
};
pub use evidence::{
    fold_selected_m3a, fold_selected_m3b, M3aEvidence, M3bEvidence, TerminalEvidenceError,
};
pub use key::{
    AdmittedCKey, AdmittedHKey, CKey, HKey, KeyAdmissionError, KeyParseError, M3NetKey,
    PersistedKey, PublicPlay, ReplayFacts, ReplayVerifier, TreatmentKeyKind, C_CODEC,
    C_KEY_MAX_BYTES, H_CODEC, H_KEY_MAX_BYTES, MAX_RECORD_PAIRS, MIN_RECORD_PAIRS,
    SUPPORT_WORLD_COUNT,
};
pub use mass::{
    ExponentMass, MassError, SignedMagnitude, FIELD_EXPONENT_MAX, FIELD_SCALE,
    ROOT_SUPPORT_MASS, TERMINAL_SCALE,
};
pub use net::{
    ActionRow, Arrival, ArrivalTarget, CanonicalStateOrdinal, Edge, EdgeTarget, FocalEpoch,
    InfoStateId, LegalFace, Net, NetBuildError, NetBuilder, ParentRef, PhysicalAction,
    RootSentinel, State,
};
pub use objective::{
    FutureTrickDifferential, M3aTerminal, M3bTerminal, Objective, P30Make,
};
pub use recurrence::{
    solve_exact, ArgmaxFace, FixedPolicy, PolicyRow, RepriceError, RepriceTable, SolveError,
    SolvedNet,
};

