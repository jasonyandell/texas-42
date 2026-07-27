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
