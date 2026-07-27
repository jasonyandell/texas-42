//! rob-core: the exact Texas 42 engine (slice 01).
//!
//! An executable mathematical specification with proof receipts, built as the
//! reconciled merge of the two ingest packages — rec's mathematics under
//! v0.7's type discipline (rob/BRIEF.md §1). Pure, no I/O, exact arithmetic
//! only (INV-4).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod algebra;
pub mod declaration;
pub mod domino;
pub mod objective;
pub mod pip;
pub mod rules;
pub mod seat;
pub mod support;

pub use algebra::order::{Rank, TrickKey};
pub use algebra::suits::{LedSuit, LedSuitSet};
pub use algebra::transport::{
    pip_trump_transport, unscored_mechanics_class, PipTrumpTransport, ScoredMechanics,
    UnscoredMechanics, UnscoredMechanicsClass,
};
pub use algebra::trick::{Play, TrickError, TrickResult};
pub use algebra::{algebra_for, DeclarationAlgebra};
pub use declaration::{Declaration, GAME_DECLARATIONS};
pub use domino::{
    all_ids, domino_from_id, domino_id, natural_incidence, Domino, DominoId, DominoSet, DOMINOES,
    DOMINO_COUNT,
};
pub use objective::auction::{
    enumerate_terminal_histories, AuctionResult, AuctionState, AuctionWin, ObjectiveDealAttempt,
};
pub use objective::contract::{contract_from_auction, settle, Contract, HandAward};
pub use objective::deal::{hidden_assignment_count, ordered_deal_count, DealWorld};
pub use objective::events::{BasePublicEvent, PrivateDealObservation};
pub use objective::play::{
    apply_auction_action, apply_play, begin_contracted_play, begin_deal_attempt, close_auction,
    complete_hand, legal_plays, CloseAuctionOutcome, ContractedPlayState, HandResult, MatchPhase,
    MatchState, PendingDeclaration, PlayPhase, ReachableContractedPlayState,
    UncertifiedContractedPlayStructure,
};
pub use objective::ObjectiveError;
pub use pip::{Pip, PipPermutation, PIPS};
pub use rules::{AuctionAction, BidValue, MarkAmount, PointAmount, RulesConfig};
pub use seat::{Seat, Team};
pub use support::cells::{
    derive_auction_cells, derive_rule_cells, initial_contracted_mechanical, sample_native_world,
    update_support, AbstractCells, AbstractWorld, MechanicalCompiledView, MechanicalState,
    RemainderWorld, RuleDerivedCellSystem, HIDDEN_SEATS,
};
pub use support::census::{
    all_ternary_signatures, floor_family_count, reachable_capacity_profiles, support_census,
    SupportCensus, TernarySignature,
};
pub use support::count::{
    assignment_count, count_deletion_recurrence, count_generating_function, count_occupancy_dp,
    OccupancyDpStats,
};
pub use support::dynamics::{
    ambiguity_rank, certified_matching_minor_update, game_observation, matching_minor_update,
    DeletionRecord, ObservationKind, TypedHiddenObservation,
};
pub use support::normal_form::{
    compile_exact_support, compile_total_support, feasible_world, marginal_by_scc,
    native_compile_exact_support, native_validate_support, rank_world, ternary_signature_valid,
    unrank_world, Ambiguity, FeasibleSupportNormalForm, NativeSupportNormalForm,
    TotalSupportNormalForm,
};
pub use support::outer::{
    lead_witness_b_convolution, lead_witness_b_inclusion_exclusion, profile_certificate_count,
    schedule_admissible, schedule_census_a, schedule_census_t, OuterCheckReport,
    ReachabilityOuterNecessaryProfile,
};
pub use support::reduce::{marginal_allowed, marginal_by_projection, reduce};
pub use support::sampler::{sample_uniform_world, world_probability, ExactRationalChoiceSource};
pub use support::symbolic::{
    validate_symbolic_trace, validate_symbolic_trace_with, AcceptedSymbolicSupport,
    SymbolicRejection, SymbolicTraceCertificate,
};
pub use support::SupportError;
