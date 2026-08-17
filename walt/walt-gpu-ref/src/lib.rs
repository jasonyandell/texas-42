//! Portable scalar reference for the M1 opening-response projector.
//!
//! The closed-form projector and the direct physical enumerator intentionally
//! take different implementation paths.  Their common boundary is the
//! canonical `(response triple, matching-count vector)` key.

#![forbid(unsafe_code)]

mod carrier;
mod context;
mod projection;
mod receipt;
mod root;

pub use carrier::{
    ReducedOpeningCarrierV1, ReducedOpeningCoordinateV1, MAX_REDUCED_GRADE_V1,
    MAX_REDUCED_MATCHING_COUNT_V1, MIN_REDUCED_GRADE_V1,
};
pub use context::{
    OpeningContext, OpeningError, M1_DIRECT_WORLD_CAP_V1, MAX_GRADE, MAX_OPENING_MATCHING_COUNT_V1,
};
pub use projection::{
    direct_preflight, project_closed_form, project_direct, DirectPreflightV1, OpeningCell,
    OpeningCellKey, OpeningProjection, OpeningResponseAggregate, ResponseRole,
    MAX_OPENING_CELLS_V1, OPENING_DEAL_COUNT,
};
pub use receipt::{
    canonical_m1_grade5_declared_stop_bytes_v1, validate_m1_grade5_declared_stop_v1,
    validate_opening_run_envelope_v1, BuildIdentityV1, OpeningEnvelopeError,
    VerifiedM1Grade5DeclaredStopV1, VerifiedOpeningEnvelopeV1, GPU_NATIVE_TRICK1_GUIDE_V02_SHA256,
    GT1_FREEZE_SET_DESCRIPTOR_V1, GT1_FREEZE_SET_SHA256_V1, M1_GRADE5_STOP_BUILD_IDENTITY_OFFSET,
    M1_GRADE5_STOP_HEADER_BYTES, M1_GRADE5_STOP_MAGIC, M1_GRADE5_STOP_REASON_DIRECT_WORLD_CAP_V1,
    M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET, M1_GRADE5_STOP_VERSION,
    M1_GRADE5_STOP_WORLD_COUNT_V1, M1_INFO_NET_STATUS_NOT_APPLICABLE_V1,
    M1_INFO_NET_VERSION_NOT_APPLICABLE_V1, M1_OPENING_DIRECT_PARITY_TASK_ID_V1,
    M1_OPENING_PROJECTOR_TASK_ID_V1, OPENING_ENVELOPE_BUILD_IDENTITY_OFFSET,
    OPENING_ENVELOPE_HEADER_BYTES, OPENING_ENVELOPE_MAGIC, OPENING_ENVELOPE_ROOT_SHA256_OFFSET,
    OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET, OPENING_ENVELOPE_VERSION,
    OPENING_RECEIPT_CELL_BYTES, OPENING_RECEIPT_HEADER_BYTES, OPENING_RECEIPT_MAGIC,
    OPENING_RECEIPT_VERSION, OPENING_ROOT_KEY_BYTES, OPENING_ROOT_KEY_MAGIC,
    OPENING_ROOT_KEY_VERSION,
};
pub use root::{
    DeclaringTeamMakesV1, IgnoreAuctionEvidenceV1, OpeningContractV1, OpeningModelProfileV1,
    OpeningRootV1, OpeningStraightHand21FieldActionsV1, PointBidV1,
    UniformCompatibleOpeningDealsV1, UniformRandomLegalV1, MAX_POINT_BID_V1, MIN_POINT_BID_V1,
    OPENING_MODEL_PROFILE_V1,
};
pub use walt_gpu_spec::{OpeningLikelihoodCoeff, ScaledOpeningMass, SupportCount};
