use core::fmt;

use walt_core::{Context, Decl, Domino, DominoSet, Pip, Seat};
use walt_gpu_spec::{
    sha256, FieldProfileId, PriorProfileId, SemanticTables, UtilityProfileId, FIELD_SCALE,
    TABLE_FORMAT_VERSION,
};

use crate::{
    direct_preflight, project_closed_form, DirectPreflightV1, OpeningContext, OpeningContractV1,
    OpeningError, OpeningProjection, OpeningRootV1, ReducedOpeningCarrierV1,
    ReducedOpeningCoordinateV1, M1_DIRECT_WORLD_CAP_V1,
};

pub const OPENING_RECEIPT_MAGIC: [u8; 8] = *b"W42M1R01";
pub const OPENING_RECEIPT_VERSION: u16 = 1;
pub const OPENING_RECEIPT_HEADER_BYTES: usize = 50;
pub const OPENING_RECEIPT_CELL_BYTES: usize = 26;

/// Fully bound M1 run-envelope magic.  The enclosed projector payload retains
/// its independent `W42M1R01` v1 format unchanged.
pub const OPENING_ENVELOPE_MAGIC: [u8; 8] = *b"W42M1E01";
pub const OPENING_ENVELOPE_VERSION: u16 = 1;
pub const OPENING_ENVELOPE_HEADER_BYTES: usize = 272;

/// A distinct persisted outcome for the grade-5 direct-parity arm.  This is
/// never an opening projector payload and carries no partial output.
pub const M1_GRADE5_STOP_MAGIC: [u8; 8] = *b"W42M1S01";
pub const M1_GRADE5_STOP_VERSION: u16 = 1;
pub const M1_GRADE5_STOP_HEADER_BYTES: usize = 312;
pub const M1_GRADE5_STOP_WORLD_COUNT_V1: u64 = 756_756;
pub const M1_GRADE5_STOP_REASON_DIRECT_WORLD_CAP_V1: u8 = 1;

pub const OPENING_ROOT_KEY_MAGIC: [u8; 8] = *b"W42RTK01";
pub const OPENING_ROOT_KEY_VERSION: u16 = 1;
pub const OPENING_ROOT_KEY_BYTES: usize = 37;

pub const M1_OPENING_PROJECTOR_TASK_ID_V1: u16 = 1;
pub const M1_OPENING_DIRECT_PARITY_TASK_ID_V1: u16 = 2;
pub const M1_INFO_NET_STATUS_NOT_APPLICABLE_V1: u8 = 1;
pub const M1_INFO_NET_VERSION_NOT_APPLICABLE_V1: u16 = 0;
pub const M1_EMPTY_PUBLIC_MARKER_V1: u8 = 1;

pub const IGNORE_AUCTION_EVIDENCE_PROFILE_ID_V1: u16 = 1;
pub const OPENING_HORIZON_PROFILE_ID_V1: u16 = 1;

const SEMANTIC_IDENTITY_MAGIC: [u8; 8] = *b"W42M1I01";
const SEMANTIC_IDENTITY_VERSION: u16 = 1;

/// Complete, byte-frozen identity of the M0/M1 assumptions in force.
///
/// The envelope carries both these bytes and their SHA-256.  Changing any
/// clause is a new freeze-set descriptor and therefore a new artifact key.
pub const GT1_FREEZE_SET_DESCRIPTOR_V1: &[u8] = b"GT1-FREEZE-SET-V1|authority=GPU-NATIVE-TRICK1-v0.3+GT1-A1..GT1-A9+freeze55|standing=CENSUS-through-GT1-A9|rulings=DS-A1,DS-A28,PG-A8,PG-A13,N4-A16(iv),T1-A12,GT1-A1..A9|freezes=7,23,26,47,55|reserved=39,40|excluded=freeze44,M2+|rules=walt-core+independent-prose-bridge-v1|enumeration=freeze-7/23|root=OpeningRootV1;focal=bidder=leader=actor;hand=7;public=empty;contract=PointBid30..41-or-Mark;loss-budget=derived|tasks=M1OpeningResponseProjectorV1,M1OpeningDirectParityDeclaredStopV1|evidence=IgnoreAuctionEvidenceV1|prior=UniformCompatibleOpeningDealsV1|field=UniformRandomLegalV1|utility=DeclaringTeamMakesV1|horizon=OpeningStraightHand21FieldActionsV1|measure=OpeningResponseScaledMassV1|arithmetic=U256MassV1:8xLE-u32|field-scale=420|response-exponent=3|full-horizon-exponent=21|table=SemanticTablesCanonicalV2|carrier=ReducedOpeningCarrierV1:grades2..5|direct-world-cap=100000|grade5-worlds=756756|cell-cap=11730|info-net=NOT_APPLICABLE:v0";

/// SHA-256 of `GT1_FREEZE_SET_DESCRIPTOR_V1`.
pub const GT1_FREEZE_SET_SHA256_V1: [u8; 32] = [
    0x9b, 0x18, 0x10, 0x92, 0x04, 0x5b, 0x00, 0x38, 0x93, 0xca, 0xe7, 0xc0, 0x9c, 0xc7, 0xb7, 0xc8,
    0xb5, 0x7f, 0x75, 0xc3, 0xc5, 0xc4, 0xcf, 0x70, 0x43, 0xb8, 0xd4, 0x28, 0xdf, 0x73, 0x8e, 0xfa,
];

/// SHA-256 of the preserved 82,740-byte received v0.2 guide.
pub const GPU_NATIVE_TRICK1_GUIDE_V02_SHA256: [u8; 32] = [
    0xee, 0x2e, 0x78, 0xda, 0x20, 0xeb, 0x7d, 0x08, 0x7f, 0xb1, 0x21, 0xf4, 0x67, 0xa5, 0x6b, 0xaf,
    0xc0, 0x17, 0x9a, 0x45, 0xfb, 0x69, 0x2c, 0xa0, 0xb9, 0x38, 0xf4, 0xc4, 0x21, 0x0b, 0x6a, 0x44,
];

const ENVELOPE_TASK_OFFSET: usize = 20;
const ENVELOPE_ROOT_KEY_VERSION_OFFSET: usize = 22;
const ENVELOPE_INFO_NET_STATUS_OFFSET: usize = 24;
const ENVELOPE_RESERVED_OFFSET: usize = 25;
const ENVELOPE_INFO_NET_VERSION_OFFSET: usize = 26;
const ENVELOPE_TABLE_FORMAT_OFFSET: usize = 28;
const ENVELOPE_ACTION_OFFSET: usize = 30;
const ENVELOPE_CONTEXT_OFFSET: usize = 31;
pub const OPENING_ENVELOPE_BUILD_IDENTITY_OFFSET: usize = 32;
const ENVELOPE_GUIDE_SHA_OFFSET: usize = 64;
const ENVELOPE_FREEZE_SHA_OFFSET: usize = 96;
const ENVELOPE_TABLE_SHA_OFFSET: usize = 128;
pub const OPENING_ENVELOPE_ROOT_SHA256_OFFSET: usize = 160;
const ENVELOPE_PAYLOAD_SHA_OFFSET: usize = 192;
pub const OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET: usize = 224;
const ENVELOPE_FREEZE_LENGTH_OFFSET: usize = 256;
const ENVELOPE_ROOT_LENGTH_OFFSET: usize = 260;
const ENVELOPE_TABLE_LENGTH_OFFSET: usize = 264;
const ENVELOPE_PAYLOAD_LENGTH_OFFSET: usize = 268;

const STOP_TASK_OFFSET: usize = 20;
const STOP_ROOT_KEY_VERSION_OFFSET: usize = 22;
const STOP_INFO_NET_STATUS_OFFSET: usize = 24;
const STOP_RESERVED_OFFSET: usize = 25;
const STOP_INFO_NET_VERSION_OFFSET: usize = 26;
const STOP_TABLE_FORMAT_OFFSET: usize = 28;
const STOP_REASON_OFFSET: usize = 30;
const STOP_DECL_OFFSET: usize = 31;
pub const M1_GRADE5_STOP_BUILD_IDENTITY_OFFSET: usize = 32;
const STOP_GUIDE_SHA_OFFSET: usize = 64;
const STOP_FREEZE_SHA_OFFSET: usize = 96;
const STOP_TABLE_SHA_OFFSET: usize = 128;
const STOP_ROOT_SHA_OFFSET: usize = 160;
const STOP_COORDINATE_SHA_OFFSET: usize = 192;
pub const M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET: usize = 224;
const STOP_GRADE_OFFSET: usize = 256;
const STOP_CONTEXT_OFFSET: usize = 257;
const STOP_MATCHING_COUNT_OFFSET: usize = 258;
const STOP_COORDINATE_RESERVED_OFFSET: usize = 259;
const STOP_POOL_OFFSET: usize = 260;
const STOP_WORLD_COUNT_OFFSET: usize = 264;
const STOP_CAP_OFFSET: usize = 272;
const STOP_EMITTED_WORLDS_OFFSET: usize = 280;
const STOP_EMITTED_CELLS_OFFSET: usize = 288;
const STOP_PAYLOAD_LENGTH_OFFSET: usize = 292;
const STOP_FREEZE_LENGTH_OFFSET: usize = 296;
const STOP_ROOT_LENGTH_OFFSET: usize = 300;
const STOP_TABLE_LENGTH_OFFSET: usize = 304;
const STOP_FINAL_RESERVED_OFFSET: usize = 308;

/// Integration-supplied build identity for one producing binary/toolchain.
///
/// Its interpretation belongs to integration (for example a hash of the
/// executable build manifest), but an omitted all-zero identity is never a
/// valid persisted-run key.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuildIdentityV1([u8; 32]);

impl BuildIdentityV1 {
    pub fn new(bytes: [u8; 32]) -> Result<BuildIdentityV1, OpeningEnvelopeError> {
        if digest_is_zero(bytes) {
            Err(OpeningEnvelopeError::ZeroBuildIdentity)
        } else {
            Ok(BuildIdentityV1(bytes))
        }
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// Validated identities recovered from a complete persisted M1 envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifiedOpeningEnvelopeV1 {
    build_identity: BuildIdentityV1,
    root: OpeningRootV1,
    selected_action: Domino,
    projection_context: Context,
    semantic_identity_sha256: [u8; 32],
    semantic_table_sha256: [u8; 32],
    projector_payload_sha256: [u8; 32],
    projector_payload_len: u32,
}

impl VerifiedOpeningEnvelopeV1 {
    pub const fn build_identity(self) -> BuildIdentityV1 {
        self.build_identity
    }

    pub const fn root(self) -> OpeningRootV1 {
        self.root
    }

    pub const fn selected_action(self) -> Domino {
        self.selected_action
    }

    pub const fn projection_context(self) -> Context {
        self.projection_context
    }

    pub const fn semantic_identity_sha256(self) -> [u8; 32] {
        self.semantic_identity_sha256
    }

    pub const fn semantic_table_sha256(self) -> [u8; 32] {
        self.semantic_table_sha256
    }

    pub const fn projector_payload_sha256(self) -> [u8; 32] {
        self.projector_payload_sha256
    }

    pub const fn projector_payload_len(self) -> u32 {
        self.projector_payload_len
    }
}

/// Validated identity and exact zero-emission accounting recovered from one
/// persisted grade-5 direct-parity declared stop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifiedM1Grade5DeclaredStopV1 {
    build_identity: BuildIdentityV1,
    root: OpeningRootV1,
    led: Context,
    matching_count: u8,
    pool: DominoSet,
    semantic_identity_sha256: [u8; 32],
}

impl VerifiedM1Grade5DeclaredStopV1 {
    pub const fn build_identity(self) -> BuildIdentityV1 {
        self.build_identity
    }

    pub const fn root(self) -> OpeningRootV1 {
        self.root
    }

    pub const fn grade(self) -> u8 {
        5
    }

    pub const fn led(self) -> Context {
        self.led
    }

    pub const fn matching_count(self) -> u8 {
        self.matching_count
    }

    pub const fn pool(self) -> DominoSet {
        self.pool
    }

    pub const fn world_count(self) -> u64 {
        M1_GRADE5_STOP_WORLD_COUNT_V1
    }

    pub const fn cap(self) -> u64 {
        M1_DIRECT_WORLD_CAP_V1
    }

    pub const fn emitted_worlds(self) -> u64 {
        0
    }

    pub const fn emitted_cells(self) -> u32 {
        0
    }

    pub const fn payload_len(self) -> u32 {
        0
    }

    pub const fn semantic_identity_sha256(self) -> [u8; 32] {
        self.semantic_identity_sha256
    }
}

/// Host-side failure while producing or validating the fully bound envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpeningEnvelopeError {
    Projection(OpeningError),
    ZeroBuildIdentity,
    BuildIdentityMismatch,
    FrozenIdentityMismatch(&'static str),
    RootProjectionMismatch,
    IllegalRootAction {
        action: Domino,
    },
    ActionContextMismatch {
        action: Domino,
        action_context: Context,
        projection_context: Context,
    },
    LengthOverflow(&'static str),
    Truncated(&'static str),
    UnknownIdentity(&'static str),
    NonCanonical(&'static str),
    DigestMismatch(&'static str),
    CoordinateNotInCarrier,
    NotGrade5DeclaredStop,
    PartialStopOutput,
}

impl fmt::Display for OpeningEnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpeningEnvelopeError::Projection(error) => write!(f, "opening projection: {error}"),
            OpeningEnvelopeError::ZeroBuildIdentity => {
                f.write_str("M1 envelope requires a nonzero build identity")
            }
            OpeningEnvelopeError::BuildIdentityMismatch => {
                f.write_str("persisted M1 artifact belongs to a different build identity")
            }
            OpeningEnvelopeError::FrozenIdentityMismatch(identity) => {
                write!(f, "compiled frozen identity mismatch: {identity}")
            }
            OpeningEnvelopeError::RootProjectionMismatch => {
                f.write_str("projection context is not derived from the supplied opening root")
            }
            OpeningEnvelopeError::IllegalRootAction { action } => {
                write!(f, "selected root action {action} is not legal for the opening root")
            }
            OpeningEnvelopeError::ActionContextMismatch {
                action,
                action_context,
                projection_context,
            } => write!(
                f,
                "selected root action {action} leads {action_context}, not projection context {projection_context}"
            ),
            OpeningEnvelopeError::LengthOverflow(field) => {
                write!(f, "M1 envelope length overflow in {field}")
            }
            OpeningEnvelopeError::Truncated(field) => {
                write!(f, "M1 envelope is truncated at {field}")
            }
            OpeningEnvelopeError::UnknownIdentity(field) => {
                write!(f, "M1 envelope carries an unknown identity in {field}")
            }
            OpeningEnvelopeError::NonCanonical(field) => {
                write!(f, "M1 envelope is noncanonical in {field}")
            }
            OpeningEnvelopeError::DigestMismatch(field) => {
                write!(f, "M1 envelope digest mismatch for {field}")
            }
            OpeningEnvelopeError::CoordinateNotInCarrier => {
                f.write_str("grade-5 stop coordinate is not in the supplied root carrier")
            }
            OpeningEnvelopeError::NotGrade5DeclaredStop => {
                f.write_str("direct-parity outcome is not the exact grade-5 declared stop")
            }
            OpeningEnvelopeError::PartialStopOutput => {
                f.write_str("grade-5 declared stop contains forbidden partial output")
            }
        }
    }
}

impl std::error::Error for OpeningEnvelopeError {}

impl From<OpeningError> for OpeningEnvelopeError {
    fn from(error: OpeningError) -> OpeningEnvelopeError {
        OpeningEnvelopeError::Projection(error)
    }
}

impl OpeningProjection {
    /// Renders the fixed little-endian M1 projector payload v1.
    ///
    /// The header is magic, version, declaration, led context, grade, one
    /// zero reserved byte, field scale, pool bits, physical-world count,
    /// direct-world cap, cell count, and total scaled mass.
    ///
    /// Cell records follow the projection's response-then-e ordering and hold
    /// three response indices, three matching counts, support, per-world
    /// coefficient, and scaled mass.
    ///
    /// **NON-PERSISTABLE:** these bytes deliberately omit root, profile,
    /// semantic-table, freeze-set, action, and build identities.  They exist
    /// only as the unchanged inner payload of a validated bound envelope and
    /// for in-memory parity.  Persist `canonical_run_envelope_bytes` instead.
    pub(crate) fn canonical_projector_payload_bytes_v1(&self) -> Result<Vec<u8>, OpeningError> {
        let cell_count = u32::try_from(self.cells().len())
            .map_err(|_| OpeningError::ArithmeticOverflow("receipt cell count"))?;
        let total_mass = self.total_scaled_mass()?.value();
        let world_count = self.context().physical_world_count()?;
        let payload_bytes = self
            .cells()
            .len()
            .checked_mul(OPENING_RECEIPT_CELL_BYTES)
            .ok_or(OpeningError::ArithmeticOverflow("receipt payload length"))?;
        let byte_len = OPENING_RECEIPT_HEADER_BYTES
            .checked_add(payload_bytes)
            .ok_or(OpeningError::ArithmeticOverflow("receipt byte length"))?;
        let mut bytes = Vec::with_capacity(byte_len);

        bytes.extend_from_slice(&OPENING_RECEIPT_MAGIC);
        push_u16(&mut bytes, OPENING_RECEIPT_VERSION);
        bytes.push(decl_code(self.context().decl()));
        bytes.push(
            u8::try_from(self.context().led().index())
                .map_err(|_| OpeningError::ArithmeticOverflow("receipt led context"))?,
        );
        bytes.push(self.context().grade());
        bytes.push(0);
        push_u32(&mut bytes, FIELD_SCALE);
        push_u32(&mut bytes, self.context().pool().bits());
        push_u64(&mut bytes, world_count);
        push_u64(&mut bytes, M1_DIRECT_WORLD_CAP_V1);
        push_u32(&mut bytes, cell_count);
        push_u64(&mut bytes, total_mass);

        for cell in self.cells() {
            for tile in cell.key().response() {
                bytes.push(
                    u8::try_from(tile.index())
                        .map_err(|_| OpeningError::ArithmeticOverflow("receipt response index"))?,
                );
            }
            bytes.extend_from_slice(&cell.key().matching_counts());
            push_u32(&mut bytes, cell.support().value());
            push_u64(&mut bytes, cell.per_world_coefficient().value());
            push_u64(&mut bytes, cell.scaled_mass()?.value());
        }

        if bytes.len() != byte_len {
            return Err(OpeningError::ArithmeticOverflow("receipt rendered length"));
        }
        Ok(bytes)
    }

    /// Renders a fully bound M1 run envelope around the unchanged projector
    /// payload v1.
    ///
    /// The fixed 272-byte header is, in order: envelope magic/version/header
    /// and total lengths; task, root-key, and explicitly-not-applicable
    /// information-net metadata; semantic-table format; selected action and
    /// projection context; build identity; guide, freeze-set, table, root-key,
    /// and payload SHA-256 values; then four section lengths.  Variable
    /// sections follow as freeze descriptor, canonical root key, complete
    /// semantic-table blob, and the complete projector payload v1.
    pub fn canonical_run_envelope_bytes(
        &self,
        root: OpeningRootV1,
        selected_action: Domino,
        build_identity: BuildIdentityV1,
    ) -> Result<Vec<u8>, OpeningEnvelopeError> {
        validate_frozen_identities()?;
        validate_projection_binding(self, root, selected_action)?;

        let root_key = canonical_root_key_bytes(root)?;
        let semantic_tables = SemanticTables::from_walt_core().canonical_bytes();
        let projector_payload = self.canonical_projector_payload_bytes_v1()?;

        let freeze_length = u32::try_from(GT1_FREEZE_SET_DESCRIPTOR_V1.len())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("freeze descriptor"))?;
        let root_length = u32::try_from(root_key.len())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("root key"))?;
        let table_length = u32::try_from(semantic_tables.len())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("semantic table"))?;
        let payload_length = u32::try_from(projector_payload.len())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("projector payload"))?;
        let total_length = [
            GT1_FREEZE_SET_DESCRIPTOR_V1.len(),
            root_key.len(),
            semantic_tables.len(),
            projector_payload.len(),
        ]
        .into_iter()
        .try_fold(OPENING_ENVELOPE_HEADER_BYTES, |total, length| {
            total
                .checked_add(length)
                .ok_or(OpeningEnvelopeError::LengthOverflow("total envelope"))
        })?;
        let total_length_u64 = u64::try_from(total_length)
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("total envelope"))?;
        let selected_action_index = u8::try_from(selected_action.index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("selected action index"))?;
        let context_index = u8::try_from(self.context().led().index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("projection context index"))?;

        let table_sha256 = sha256(&semantic_tables);
        let root_sha256 = sha256(&root_key);
        let payload_sha256 = sha256(&projector_payload);
        let semantic_identity_sha256 = semantic_identity_sha256(
            M1_OPENING_PROJECTOR_TASK_ID_V1,
            build_identity,
            selected_action_index,
            context_index,
            &root_key,
            &semantic_tables,
            &payload_sha256,
            &[],
        )?;
        let mut bytes = Vec::with_capacity(total_length);
        bytes.extend_from_slice(&OPENING_ENVELOPE_MAGIC);
        push_u16(&mut bytes, OPENING_ENVELOPE_VERSION);
        push_u16(
            &mut bytes,
            u16::try_from(OPENING_ENVELOPE_HEADER_BYTES)
                .map_err(|_| OpeningEnvelopeError::LengthOverflow("envelope header"))?,
        );
        push_u64(&mut bytes, total_length_u64);
        push_u16(&mut bytes, M1_OPENING_PROJECTOR_TASK_ID_V1);
        push_u16(&mut bytes, OPENING_ROOT_KEY_VERSION);
        bytes.push(M1_INFO_NET_STATUS_NOT_APPLICABLE_V1);
        bytes.push(0);
        push_u16(&mut bytes, M1_INFO_NET_VERSION_NOT_APPLICABLE_V1);
        push_u16(&mut bytes, TABLE_FORMAT_VERSION);
        bytes.push(selected_action_index);
        bytes.push(context_index);
        bytes.extend_from_slice(&build_identity.bytes());
        bytes.extend_from_slice(&GPU_NATIVE_TRICK1_GUIDE_V02_SHA256);
        bytes.extend_from_slice(&GT1_FREEZE_SET_SHA256_V1);
        bytes.extend_from_slice(&table_sha256);
        bytes.extend_from_slice(&root_sha256);
        bytes.extend_from_slice(&payload_sha256);
        bytes.extend_from_slice(&semantic_identity_sha256);
        push_u32(&mut bytes, freeze_length);
        push_u32(&mut bytes, root_length);
        push_u32(&mut bytes, table_length);
        push_u32(&mut bytes, payload_length);
        if bytes.len() != OPENING_ENVELOPE_HEADER_BYTES {
            return Err(OpeningEnvelopeError::LengthOverflow(
                "rendered envelope header",
            ));
        }
        bytes.extend_from_slice(GT1_FREEZE_SET_DESCRIPTOR_V1);
        bytes.extend_from_slice(&root_key);
        bytes.extend_from_slice(&semantic_tables);
        bytes.extend_from_slice(&projector_payload);
        if bytes.len() != total_length {
            return Err(OpeningEnvelopeError::LengthOverflow(
                "rendered total envelope",
            ));
        }

        let verified = validate_opening_run_envelope_v1(&bytes, build_identity)?;
        if verified.root() != root
            || verified.selected_action() != selected_action
            || verified.build_identity() != build_identity
        {
            return Err(OpeningEnvelopeError::NonCanonical(
                "fresh-envelope self-validation",
            ));
        }
        Ok(bytes)
    }
}

/// Validates a complete projector envelope for the currently running build.
/// A structurally sound artifact from another nonzero build is rejected.
pub fn validate_opening_run_envelope_v1(
    bytes: &[u8],
    expected_build_identity: BuildIdentityV1,
) -> Result<VerifiedOpeningEnvelopeV1, OpeningEnvelopeError> {
    validate_frozen_identities()?;
    require_header(bytes, OPENING_ENVELOPE_HEADER_BYTES, "envelope header")?;
    require_bytes(bytes, 0, &OPENING_ENVELOPE_MAGIC, "envelope magic")?;
    require_u16(bytes, 8, OPENING_ENVELOPE_VERSION, "envelope version")?;
    require_u16(
        bytes,
        10,
        usize_to_u16(OPENING_ENVELOPE_HEADER_BYTES, "envelope header")?,
        "envelope header length",
    )?;
    let declared_total = usize::try_from(read_u64(bytes, 12, "envelope total length")?)
        .map_err(|_| OpeningEnvelopeError::LengthOverflow("envelope total length"))?;
    if declared_total != bytes.len() {
        return Err(OpeningEnvelopeError::NonCanonical("envelope total length"));
    }
    require_u16(
        bytes,
        ENVELOPE_TASK_OFFSET,
        M1_OPENING_PROJECTOR_TASK_ID_V1,
        "envelope task",
    )?;
    validate_shared_header_identities(
        bytes,
        ENVELOPE_ROOT_KEY_VERSION_OFFSET,
        ENVELOPE_INFO_NET_STATUS_OFFSET,
        ENVELOPE_RESERVED_OFFSET,
        ENVELOPE_INFO_NET_VERSION_OFFSET,
        ENVELOPE_TABLE_FORMAT_OFFSET,
    )?;

    let build_identity = BuildIdentityV1::new(read_array::<32>(
        bytes,
        OPENING_ENVELOPE_BUILD_IDENTITY_OFFSET,
        "build identity",
    )?)?;
    if build_identity != expected_build_identity {
        return Err(OpeningEnvelopeError::BuildIdentityMismatch);
    }
    require_bytes(
        bytes,
        ENVELOPE_GUIDE_SHA_OFFSET,
        &GPU_NATIVE_TRICK1_GUIDE_V02_SHA256,
        "received guide SHA-256",
    )?;
    require_bytes(
        bytes,
        ENVELOPE_FREEZE_SHA_OFFSET,
        &GT1_FREEZE_SET_SHA256_V1,
        "freeze-set SHA-256",
    )?;

    let lengths = [
        read_u32(bytes, ENVELOPE_FREEZE_LENGTH_OFFSET, "freeze length")? as usize,
        read_u32(bytes, ENVELOPE_ROOT_LENGTH_OFFSET, "root length")? as usize,
        read_u32(bytes, ENVELOPE_TABLE_LENGTH_OFFSET, "table length")? as usize,
        read_u32(bytes, ENVELOPE_PAYLOAD_LENGTH_OFFSET, "payload length")? as usize,
    ];
    let sections = exact_sections(bytes, OPENING_ENVELOPE_HEADER_BYTES, lengths)?;
    let freeze = sections[0];
    let root_key = sections[1];
    let semantic_tables = sections[2];
    let projector_payload = sections[3];
    if freeze != GT1_FREEZE_SET_DESCRIPTOR_V1 {
        return Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "freeze descriptor",
        ));
    }
    if sha256(freeze) != GT1_FREEZE_SET_SHA256_V1 {
        return Err(OpeningEnvelopeError::DigestMismatch("freeze descriptor"));
    }

    let canonical_tables = SemanticTables::from_walt_core().canonical_bytes();
    if semantic_tables != canonical_tables {
        return Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "semantic table bytes",
        ));
    }
    let table_sha256 = sha256(semantic_tables);
    require_bytes(
        bytes,
        ENVELOPE_TABLE_SHA_OFFSET,
        &table_sha256,
        "semantic table SHA-256",
    )?;

    let root = decode_root_key(root_key)?;
    let root_sha256 = sha256(root_key);
    require_bytes(
        bytes,
        OPENING_ENVELOPE_ROOT_SHA256_OFFSET,
        &root_sha256,
        "root SHA-256",
    )?;

    let selected_action = Domino::from_index(usize::from(read_u8(
        bytes,
        ENVELOPE_ACTION_OFFSET,
        "selected action",
    )?))
    .ok_or(OpeningEnvelopeError::UnknownIdentity("selected action"))?;
    let projection_context = Context::from_index(usize::from(read_u8(
        bytes,
        ENVELOPE_CONTEXT_OFFSET,
        "projection context",
    )?))
    .ok_or(OpeningEnvelopeError::UnknownIdentity("projection context"))?;

    let payload_sha256 = sha256(projector_payload);
    require_bytes(
        bytes,
        ENVELOPE_PAYLOAD_SHA_OFFSET,
        &payload_sha256,
        "projector payload SHA-256",
    )?;
    let expected_semantic_identity = semantic_identity_sha256(
        M1_OPENING_PROJECTOR_TASK_ID_V1,
        build_identity,
        u8::try_from(selected_action.index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("selected action index"))?,
        u8::try_from(projection_context.index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("projection context index"))?,
        root_key,
        semantic_tables,
        &payload_sha256,
        &[],
    )?;
    require_bytes(
        bytes,
        OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET,
        &expected_semantic_identity,
        "combined semantic identity SHA-256",
    )?;

    let expected_context = root.opening_context(projection_context)?;
    let expected_projection = project_closed_form(expected_context)?;
    validate_projection_binding(&expected_projection, root, selected_action)?;
    let expected_payload = expected_projection.canonical_projector_payload_bytes_v1()?;
    if projector_payload != expected_payload {
        return Err(OpeningEnvelopeError::NonCanonical(
            "projector payload does not replay from root",
        ));
    }

    Ok(VerifiedOpeningEnvelopeV1 {
        build_identity,
        root,
        selected_action,
        projection_context,
        semantic_identity_sha256: expected_semantic_identity,
        semantic_table_sha256: table_sha256,
        projector_payload_sha256: payload_sha256,
        projector_payload_len: u32::try_from(projector_payload.len())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("projector payload"))?,
    })
}

/// Produces the only persistable M1 outcome for a grade-5 direct-parity arm.
/// The exact kernel is preflighted, no world is iterated, and the record fixes
/// all emitted counts and payload length to zero.
pub fn canonical_m1_grade5_declared_stop_bytes_v1(
    root: OpeningRootV1,
    coordinate: ReducedOpeningCoordinateV1,
    build_identity: BuildIdentityV1,
) -> Result<Vec<u8>, OpeningEnvelopeError> {
    validate_frozen_identities()?;
    validate_grade5_coordinate(root, coordinate)?;

    let context = coordinate.opening_context()?;
    let preflight = direct_preflight(context)?;
    if preflight
        != (DirectPreflightV1::DeclaredStop {
            world_count: M1_GRADE5_STOP_WORLD_COUNT_V1,
            cap: M1_DIRECT_WORLD_CAP_V1,
        })
    {
        return Err(OpeningEnvelopeError::NotGrade5DeclaredStop);
    }

    let root_key = canonical_root_key_bytes(root)?;
    let semantic_tables = SemanticTables::from_walt_core().canonical_bytes();
    let coordinate_bytes = canonical_grade5_coordinate_bytes(context, coordinate.matching_count())?;
    let coordinate_sha256 = sha256(&coordinate_bytes);
    let empty_payload_sha256 = sha256(&[]);
    let stop_binding = canonical_stop_binding_bytes(context, coordinate.matching_count())?;
    let semantic_identity_sha256 = semantic_identity_sha256(
        M1_OPENING_DIRECT_PARITY_TASK_ID_V1,
        build_identity,
        u8::MAX,
        u8::try_from(context.led().index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("stop context index"))?,
        &root_key,
        &semantic_tables,
        &empty_payload_sha256,
        &stop_binding,
    )?;

    let total_length = [
        GT1_FREEZE_SET_DESCRIPTOR_V1.len(),
        root_key.len(),
        semantic_tables.len(),
    ]
    .into_iter()
    .try_fold(M1_GRADE5_STOP_HEADER_BYTES, |total, length| {
        total
            .checked_add(length)
            .ok_or(OpeningEnvelopeError::LengthOverflow("grade-5 stop"))
    })?;
    let mut bytes = Vec::with_capacity(total_length);
    bytes.extend_from_slice(&M1_GRADE5_STOP_MAGIC);
    push_u16(&mut bytes, M1_GRADE5_STOP_VERSION);
    push_u16(
        &mut bytes,
        usize_to_u16(M1_GRADE5_STOP_HEADER_BYTES, "grade-5 stop header")?,
    );
    push_u64(
        &mut bytes,
        u64::try_from(total_length)
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("grade-5 stop total"))?,
    );
    push_u16(&mut bytes, M1_OPENING_DIRECT_PARITY_TASK_ID_V1);
    push_u16(&mut bytes, OPENING_ROOT_KEY_VERSION);
    bytes.push(M1_INFO_NET_STATUS_NOT_APPLICABLE_V1);
    bytes.push(0);
    push_u16(&mut bytes, M1_INFO_NET_VERSION_NOT_APPLICABLE_V1);
    push_u16(&mut bytes, TABLE_FORMAT_VERSION);
    bytes.push(M1_GRADE5_STOP_REASON_DIRECT_WORLD_CAP_V1);
    bytes.push(decl_code(context.decl()));
    bytes.extend_from_slice(&build_identity.bytes());
    bytes.extend_from_slice(&GPU_NATIVE_TRICK1_GUIDE_V02_SHA256);
    bytes.extend_from_slice(&GT1_FREEZE_SET_SHA256_V1);
    bytes.extend_from_slice(&sha256(&semantic_tables));
    bytes.extend_from_slice(&sha256(&root_key));
    bytes.extend_from_slice(&coordinate_sha256);
    bytes.extend_from_slice(&semantic_identity_sha256);
    bytes.push(context.grade());
    bytes.push(
        u8::try_from(context.led().index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("stop context index"))?,
    );
    bytes.push(coordinate.matching_count());
    bytes.push(0);
    push_u32(&mut bytes, context.pool().bits());
    push_u64(&mut bytes, M1_GRADE5_STOP_WORLD_COUNT_V1);
    push_u64(&mut bytes, M1_DIRECT_WORLD_CAP_V1);
    push_u64(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(
        &mut bytes,
        usize_to_u32(GT1_FREEZE_SET_DESCRIPTOR_V1.len(), "freeze descriptor")?,
    );
    push_u32(&mut bytes, usize_to_u32(root_key.len(), "root key")?);
    push_u32(
        &mut bytes,
        usize_to_u32(semantic_tables.len(), "semantic tables")?,
    );
    push_u32(&mut bytes, 0);
    if bytes.len() != M1_GRADE5_STOP_HEADER_BYTES {
        return Err(OpeningEnvelopeError::LengthOverflow(
            "rendered grade-5 stop header",
        ));
    }
    bytes.extend_from_slice(GT1_FREEZE_SET_DESCRIPTOR_V1);
    bytes.extend_from_slice(&root_key);
    bytes.extend_from_slice(&semantic_tables);
    if bytes.len() != total_length {
        return Err(OpeningEnvelopeError::LengthOverflow(
            "rendered grade-5 stop total",
        ));
    }
    let verified = validate_m1_grade5_declared_stop_v1(&bytes, build_identity)?;
    if verified.root() != root
        || verified.led() != coordinate.led()
        || verified.matching_count() != coordinate.matching_count()
        || verified.pool() != coordinate.pool()
    {
        return Err(OpeningEnvelopeError::NonCanonical(
            "fresh grade-5 stop self-validation",
        ));
    }
    Ok(bytes)
}

/// Validates a grade-5 declared stop for the currently running build.
pub fn validate_m1_grade5_declared_stop_v1(
    bytes: &[u8],
    expected_build_identity: BuildIdentityV1,
) -> Result<VerifiedM1Grade5DeclaredStopV1, OpeningEnvelopeError> {
    validate_frozen_identities()?;
    require_header(bytes, M1_GRADE5_STOP_HEADER_BYTES, "grade-5 stop header")?;
    require_bytes(bytes, 0, &M1_GRADE5_STOP_MAGIC, "grade-5 stop magic")?;
    require_u16(bytes, 8, M1_GRADE5_STOP_VERSION, "grade-5 stop version")?;
    require_u16(
        bytes,
        10,
        usize_to_u16(M1_GRADE5_STOP_HEADER_BYTES, "grade-5 stop header")?,
        "grade-5 stop header length",
    )?;
    let declared_total = usize::try_from(read_u64(bytes, 12, "grade-5 stop total length")?)
        .map_err(|_| OpeningEnvelopeError::LengthOverflow("grade-5 stop total length"))?;
    if declared_total != bytes.len() {
        return Err(OpeningEnvelopeError::NonCanonical(
            "grade-5 stop total length",
        ));
    }
    require_u16(
        bytes,
        STOP_TASK_OFFSET,
        M1_OPENING_DIRECT_PARITY_TASK_ID_V1,
        "grade-5 stop task",
    )?;
    validate_shared_header_identities(
        bytes,
        STOP_ROOT_KEY_VERSION_OFFSET,
        STOP_INFO_NET_STATUS_OFFSET,
        STOP_RESERVED_OFFSET,
        STOP_INFO_NET_VERSION_OFFSET,
        STOP_TABLE_FORMAT_OFFSET,
    )?;
    require_u8(
        bytes,
        STOP_REASON_OFFSET,
        M1_GRADE5_STOP_REASON_DIRECT_WORLD_CAP_V1,
        "grade-5 stop reason",
    )?;

    let build_identity = BuildIdentityV1::new(read_array::<32>(
        bytes,
        M1_GRADE5_STOP_BUILD_IDENTITY_OFFSET,
        "grade-5 stop build identity",
    )?)?;
    if build_identity != expected_build_identity {
        return Err(OpeningEnvelopeError::BuildIdentityMismatch);
    }
    require_bytes(
        bytes,
        STOP_GUIDE_SHA_OFFSET,
        &GPU_NATIVE_TRICK1_GUIDE_V02_SHA256,
        "received guide SHA-256",
    )?;
    require_bytes(
        bytes,
        STOP_FREEZE_SHA_OFFSET,
        &GT1_FREEZE_SET_SHA256_V1,
        "freeze-set SHA-256",
    )?;
    require_u8(bytes, STOP_GRADE_OFFSET, 5, "grade-5 stop grade")?;
    require_u8(
        bytes,
        STOP_COORDINATE_RESERVED_OFFSET,
        0,
        "grade-5 coordinate reserved byte",
    )?;
    require_u64(
        bytes,
        STOP_WORLD_COUNT_OFFSET,
        M1_GRADE5_STOP_WORLD_COUNT_V1,
        "grade-5 world count",
    )?;
    require_u64(
        bytes,
        STOP_CAP_OFFSET,
        M1_DIRECT_WORLD_CAP_V1,
        "grade-5 direct cap",
    )?;
    if read_u64(bytes, STOP_EMITTED_WORLDS_OFFSET, "emitted worlds")? != 0
        || read_u32(bytes, STOP_EMITTED_CELLS_OFFSET, "emitted cells")? != 0
        || read_u32(bytes, STOP_PAYLOAD_LENGTH_OFFSET, "stop payload length")? != 0
    {
        return Err(OpeningEnvelopeError::PartialStopOutput);
    }
    require_u32(
        bytes,
        STOP_FINAL_RESERVED_OFFSET,
        0,
        "grade-5 stop final reserved field",
    )?;

    let lengths = [
        read_u32(bytes, STOP_FREEZE_LENGTH_OFFSET, "stop freeze length")? as usize,
        read_u32(bytes, STOP_ROOT_LENGTH_OFFSET, "stop root length")? as usize,
        read_u32(bytes, STOP_TABLE_LENGTH_OFFSET, "stop table length")? as usize,
    ];
    let sections = exact_sections(bytes, M1_GRADE5_STOP_HEADER_BYTES, lengths)?;
    let freeze = sections[0];
    let root_key = sections[1];
    let semantic_tables = sections[2];
    if freeze != GT1_FREEZE_SET_DESCRIPTOR_V1 {
        return Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "freeze descriptor",
        ));
    }
    if sha256(freeze) != GT1_FREEZE_SET_SHA256_V1 {
        return Err(OpeningEnvelopeError::DigestMismatch("freeze descriptor"));
    }
    let canonical_tables = SemanticTables::from_walt_core().canonical_bytes();
    if semantic_tables != canonical_tables {
        return Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "semantic table bytes",
        ));
    }
    let table_sha256 = sha256(semantic_tables);
    require_bytes(
        bytes,
        STOP_TABLE_SHA_OFFSET,
        &table_sha256,
        "semantic table SHA-256",
    )?;
    let root = decode_root_key(root_key)?;
    require_bytes(
        bytes,
        STOP_ROOT_SHA_OFFSET,
        &sha256(root_key),
        "root SHA-256",
    )?;
    let decl = decode_decl(read_u8(bytes, STOP_DECL_OFFSET, "stop declaration")?)?;
    if decl != root.decl() {
        return Err(OpeningEnvelopeError::RootProjectionMismatch);
    }
    let led = Context::from_index(usize::from(read_u8(
        bytes,
        STOP_CONTEXT_OFFSET,
        "stop context",
    )?))
    .ok_or(OpeningEnvelopeError::UnknownIdentity("stop context"))?;
    let matching_count = read_u8(bytes, STOP_MATCHING_COUNT_OFFSET, "matching count")?;
    let pool = DominoSet::from_bits(read_u32(bytes, STOP_POOL_OFFSET, "stop pool")?)
        .ok_or(OpeningEnvelopeError::UnknownIdentity("stop pool bits"))?;
    let context = OpeningContext::try_reduced(decl, led, pool, 5)?;
    if context.matching_pool().len() != usize::from(matching_count) {
        return Err(OpeningEnvelopeError::NonCanonical("grade-5 matching count"));
    }
    validate_grade5_coordinate_fields(root, context, matching_count)?;
    if direct_preflight(context)?
        != (DirectPreflightV1::DeclaredStop {
            world_count: M1_GRADE5_STOP_WORLD_COUNT_V1,
            cap: M1_DIRECT_WORLD_CAP_V1,
        })
    {
        return Err(OpeningEnvelopeError::NotGrade5DeclaredStop);
    }

    let coordinate_bytes = canonical_grade5_coordinate_bytes(context, matching_count)?;
    require_bytes(
        bytes,
        STOP_COORDINATE_SHA_OFFSET,
        &sha256(&coordinate_bytes),
        "grade-5 coordinate SHA-256",
    )?;
    let stop_binding = canonical_stop_binding_bytes(context, matching_count)?;
    let semantic_identity_sha256 = semantic_identity_sha256(
        M1_OPENING_DIRECT_PARITY_TASK_ID_V1,
        build_identity,
        u8::MAX,
        u8::try_from(led.index())
            .map_err(|_| OpeningEnvelopeError::LengthOverflow("stop context index"))?,
        root_key,
        semantic_tables,
        &sha256(&[]),
        &stop_binding,
    )?;
    require_bytes(
        bytes,
        M1_GRADE5_STOP_SEMANTIC_IDENTITY_SHA256_OFFSET,
        &semantic_identity_sha256,
        "combined semantic identity SHA-256",
    )?;

    Ok(VerifiedM1Grade5DeclaredStopV1 {
        build_identity,
        root,
        led,
        matching_count,
        pool,
        semantic_identity_sha256,
    })
}

fn validate_frozen_identities() -> Result<(), OpeningEnvelopeError> {
    if sha256(GT1_FREEZE_SET_DESCRIPTOR_V1) != GT1_FREEZE_SET_SHA256_V1 {
        return Err(OpeningEnvelopeError::FrozenIdentityMismatch(
            "compiled freeze-set descriptor SHA-256",
        ));
    }
    Ok(())
}

fn validate_projection_binding(
    projection: &OpeningProjection,
    root: OpeningRootV1,
    selected_action: Domino,
) -> Result<(), OpeningEnvelopeError> {
    let projection_context = projection.context();
    if projection_context.grade() != 7
        || projection_context != root.opening_context(projection_context.led())?
    {
        return Err(OpeningEnvelopeError::RootProjectionMismatch);
    }
    if !root.legal_leads().contains(selected_action) {
        return Err(OpeningEnvelopeError::IllegalRootAction {
            action: selected_action,
        });
    }
    let action_context = root.decl().led_context(selected_action);
    if action_context != projection_context.led() {
        return Err(OpeningEnvelopeError::ActionContextMismatch {
            action: selected_action,
            action_context,
            projection_context: projection_context.led(),
        });
    }
    Ok(())
}

/// Returns the established canonical 37-byte `W42RTK01` key for a checked
/// opening root.
///
/// This is a read-only encoder over [`OpeningRootV1`].  It deliberately exposes
/// neither a decoder nor any alternate root constructor, and it delegates to
/// the same private encoder used by the M1 receipt authority.
pub fn canonical_opening_root_key_bytes_v1(
    root: OpeningRootV1,
) -> Result<[u8; OPENING_ROOT_KEY_BYTES], OpeningEnvelopeError> {
    canonical_root_key_bytes(root)?
        .try_into()
        .map_err(|_| OpeningEnvelopeError::LengthOverflow("canonical root key array"))
}

fn canonical_root_key_bytes(root: OpeningRootV1) -> Result<Vec<u8>, OpeningEnvelopeError> {
    let mut bytes = Vec::with_capacity(OPENING_ROOT_KEY_BYTES);
    bytes.extend_from_slice(&OPENING_ROOT_KEY_MAGIC);
    push_u16(&mut bytes, OPENING_ROOT_KEY_VERSION);
    bytes.push(decl_code(root.decl()));
    bytes.push(usize_to_u8(root.focal().index(), "focal seat")?);
    bytes.push(usize_to_u8(root.bidder().index(), "bidder seat")?);
    bytes.push(usize_to_u8(root.leader().index(), "leader seat")?);
    bytes.push(usize_to_u8(root.actor().index(), "actor seat")?);
    push_u32(&mut bytes, root.focal_hand().bits());
    bytes.push(usize_to_u8(root.focal_hand().len(), "focal hand size")?);
    match root.contract() {
        OpeningContractV1::PointBid(bid) => {
            bytes.push(1);
            bytes.push(bid.value());
        }
        OpeningContractV1::Mark => {
            bytes.push(2);
            bytes.push(0);
        }
    }
    bytes.push(root.loss_budget());
    push_u16(&mut bytes, IGNORE_AUCTION_EVIDENCE_PROFILE_ID_V1);
    push_u16(&mut bytes, PriorProfileId::UNIFORM_OPENING_V1.raw());
    push_u16(&mut bytes, FieldProfileId::UNIFORM_RANDOM_LEGAL_V1.raw());
    push_u16(&mut bytes, UtilityProfileId::DECLARING_TEAM_MAKES_V1.raw());
    push_u16(&mut bytes, OPENING_HORIZON_PROFILE_ID_V1);
    bytes.push(M1_EMPTY_PUBLIC_MARKER_V1);
    bytes.push(usize_to_u8(root.public_play_count(), "public play count")?);
    bytes.push(usize_to_u8(
        root.current_trick_len(),
        "current trick length",
    )?);
    bytes.push(0);
    if bytes.len() != OPENING_ROOT_KEY_BYTES {
        return Err(OpeningEnvelopeError::LengthOverflow("canonical root key"));
    }
    Ok(bytes)
}

fn decode_root_key(bytes: &[u8]) -> Result<OpeningRootV1, OpeningEnvelopeError> {
    if bytes.len() != OPENING_ROOT_KEY_BYTES {
        return Err(OpeningEnvelopeError::NonCanonical("root key length"));
    }
    require_bytes(bytes, 0, &OPENING_ROOT_KEY_MAGIC, "root key magic")?;
    require_u16(bytes, 8, OPENING_ROOT_KEY_VERSION, "root key version")?;
    let decl = decode_decl(read_u8(bytes, 10, "root declaration")?)?;
    let focal = decode_seat(read_u8(bytes, 11, "root focal")?, "root focal")?;
    for (offset, field) in [(12, "root bidder"), (13, "root leader"), (14, "root actor")] {
        if decode_seat(read_u8(bytes, offset, field)?, field)? != focal {
            return Err(OpeningEnvelopeError::NonCanonical(field));
        }
    }
    let hand = DominoSet::from_bits(read_u32(bytes, 15, "root hand")?)
        .ok_or(OpeningEnvelopeError::UnknownIdentity("root hand bits"))?;
    require_u8(bytes, 19, 7, "root hand size")?;
    if hand.len() != 7 {
        return Err(OpeningEnvelopeError::NonCanonical("root hand size"));
    }
    let contract = match read_u8(bytes, 20, "root contract tag")? {
        1 => OpeningContractV1::point_bid(read_u8(bytes, 21, "root point bid")?)?,
        2 => {
            require_u8(bytes, 21, 0, "mark contract value")?;
            OpeningContractV1::Mark
        }
        _ => return Err(OpeningEnvelopeError::UnknownIdentity("root contract tag")),
    };
    let root = OpeningRootV1::new(decl, focal, hand, contract)?;
    require_u8(bytes, 22, root.loss_budget(), "derived loss budget")?;
    require_u16(
        bytes,
        23,
        IGNORE_AUCTION_EVIDENCE_PROFILE_ID_V1,
        "evidence profile",
    )?;
    require_u16(
        bytes,
        25,
        PriorProfileId::UNIFORM_OPENING_V1.raw(),
        "prior profile",
    )?;
    require_u16(
        bytes,
        27,
        FieldProfileId::UNIFORM_RANDOM_LEGAL_V1.raw(),
        "field profile",
    )?;
    require_u16(
        bytes,
        29,
        UtilityProfileId::DECLARING_TEAM_MAKES_V1.raw(),
        "utility profile",
    )?;
    require_u16(bytes, 31, OPENING_HORIZON_PROFILE_ID_V1, "horizon profile")?;
    require_u8(bytes, 33, M1_EMPTY_PUBLIC_MARKER_V1, "empty-public marker")?;
    require_u8(bytes, 34, 0, "public play count")?;
    require_u8(bytes, 35, 0, "current trick length")?;
    require_u8(bytes, 36, 0, "root reserved byte")?;
    if canonical_root_key_bytes(root)? != bytes {
        return Err(OpeningEnvelopeError::NonCanonical("root key encoding"));
    }
    Ok(root)
}

fn validate_grade5_coordinate(
    root: OpeningRootV1,
    coordinate: ReducedOpeningCoordinateV1,
) -> Result<(), OpeningEnvelopeError> {
    if coordinate.grade() != 5 {
        return Err(OpeningEnvelopeError::NotGrade5DeclaredStop);
    }
    let context = coordinate.opening_context()?;
    if context.decl() != root.decl()
        || context.matching_pool().len() != usize::from(coordinate.matching_count())
    {
        return Err(OpeningEnvelopeError::CoordinateNotInCarrier);
    }
    validate_grade5_coordinate_fields(root, context, coordinate.matching_count())
}

fn validate_grade5_coordinate_fields(
    root: OpeningRootV1,
    context: OpeningContext,
    matching_count: u8,
) -> Result<(), OpeningEnvelopeError> {
    let carrier = ReducedOpeningCarrierV1::from_root(root)?;
    let represented = carrier.coordinates().iter().any(|coordinate| {
        coordinate.grade() == 5
            && coordinate.led() == context.led()
            && coordinate.matching_count() == matching_count
            && coordinate.pool() == context.pool()
    });
    if !represented {
        return Err(OpeningEnvelopeError::CoordinateNotInCarrier);
    }
    Ok(())
}

fn canonical_grade5_coordinate_bytes(
    context: OpeningContext,
    matching_count: u8,
) -> Result<Vec<u8>, OpeningEnvelopeError> {
    let mut bytes = Vec::with_capacity(18);
    bytes.extend_from_slice(b"W42CO101");
    push_u16(&mut bytes, 1);
    bytes.push(decl_code(context.decl()));
    bytes.push(context.grade());
    bytes.push(usize_to_u8(context.led().index(), "coordinate context")?);
    bytes.push(matching_count);
    push_u32(&mut bytes, context.pool().bits());
    Ok(bytes)
}

fn canonical_stop_binding_bytes(
    context: OpeningContext,
    matching_count: u8,
) -> Result<Vec<u8>, OpeningEnvelopeError> {
    let mut bytes = canonical_grade5_coordinate_bytes(context, matching_count)?;
    bytes.push(M1_GRADE5_STOP_REASON_DIRECT_WORLD_CAP_V1);
    push_u64(&mut bytes, M1_GRADE5_STOP_WORLD_COUNT_V1);
    push_u64(&mut bytes, M1_DIRECT_WORLD_CAP_V1);
    push_u64(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 0);
    Ok(bytes)
}

#[allow(clippy::too_many_arguments)]
fn semantic_identity_sha256(
    task: u16,
    build_identity: BuildIdentityV1,
    action: u8,
    context: u8,
    root_key: &[u8],
    semantic_tables: &[u8],
    payload_sha256: &[u8; 32],
    task_binding: &[u8],
) -> Result<[u8; 32], OpeningEnvelopeError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&SEMANTIC_IDENTITY_MAGIC);
    push_u16(&mut bytes, SEMANTIC_IDENTITY_VERSION);
    push_u16(&mut bytes, task);
    push_u16(&mut bytes, OPENING_ROOT_KEY_VERSION);
    bytes.push(M1_INFO_NET_STATUS_NOT_APPLICABLE_V1);
    bytes.push(0);
    push_u16(&mut bytes, M1_INFO_NET_VERSION_NOT_APPLICABLE_V1);
    push_u16(&mut bytes, TABLE_FORMAT_VERSION);
    bytes.push(action);
    bytes.push(context);
    bytes.extend_from_slice(&build_identity.bytes());
    bytes.extend_from_slice(&GPU_NATIVE_TRICK1_GUIDE_V02_SHA256);
    bytes.extend_from_slice(&GT1_FREEZE_SET_SHA256_V1);
    bytes.extend_from_slice(&sha256(semantic_tables));
    bytes.extend_from_slice(&sha256(root_key));
    bytes.extend_from_slice(payload_sha256);
    push_u32(
        &mut bytes,
        usize_to_u32(GT1_FREEZE_SET_DESCRIPTOR_V1.len(), "freeze descriptor")?,
    );
    push_u32(&mut bytes, usize_to_u32(root_key.len(), "root key")?);
    push_u32(
        &mut bytes,
        usize_to_u32(semantic_tables.len(), "semantic tables")?,
    );
    push_u32(
        &mut bytes,
        usize_to_u32(task_binding.len(), "task binding")?,
    );
    bytes.extend_from_slice(GT1_FREEZE_SET_DESCRIPTOR_V1);
    bytes.extend_from_slice(root_key);
    bytes.extend_from_slice(semantic_tables);
    bytes.extend_from_slice(task_binding);
    Ok(sha256(&bytes))
}

fn validate_shared_header_identities(
    bytes: &[u8],
    root_version_offset: usize,
    info_status_offset: usize,
    reserved_offset: usize,
    info_version_offset: usize,
    table_format_offset: usize,
) -> Result<(), OpeningEnvelopeError> {
    require_u16(
        bytes,
        root_version_offset,
        OPENING_ROOT_KEY_VERSION,
        "root key version",
    )?;
    require_u8(
        bytes,
        info_status_offset,
        M1_INFO_NET_STATUS_NOT_APPLICABLE_V1,
        "information-net status",
    )?;
    require_u8(bytes, reserved_offset, 0, "reserved header byte")?;
    require_u16(
        bytes,
        info_version_offset,
        M1_INFO_NET_VERSION_NOT_APPLICABLE_V1,
        "information-net version",
    )?;
    require_u16(
        bytes,
        table_format_offset,
        TABLE_FORMAT_VERSION,
        "semantic table format",
    )?;
    Ok(())
}

fn decode_decl(raw: u8) -> Result<Decl, OpeningEnvelopeError> {
    match raw {
        0..=6 => Ok(Decl::PipTrump(
            Pip::new(raw).ok_or(OpeningEnvelopeError::UnknownIdentity("declaration"))?,
        )),
        7 => Ok(Decl::DoublesTrump),
        8 => Ok(Decl::NoTrump),
        _ => Err(OpeningEnvelopeError::UnknownIdentity("declaration")),
    }
}

fn decode_seat(raw: u8, field: &'static str) -> Result<Seat, OpeningEnvelopeError> {
    Seat::from_index(usize::from(raw)).ok_or(OpeningEnvelopeError::UnknownIdentity(field))
}

fn digest_is_zero(bytes: [u8; 32]) -> bool {
    bytes.iter().all(|byte| *byte == 0)
}

fn require_header(
    bytes: &[u8],
    header_len: usize,
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    if bytes.len() < header_len {
        Err(OpeningEnvelopeError::Truncated(field))
    } else {
        Ok(())
    }
}

fn exact_sections<const N: usize>(
    bytes: &[u8],
    header_len: usize,
    lengths: [usize; N],
) -> Result<Vec<&[u8]>, OpeningEnvelopeError> {
    let mut offset = header_len;
    let mut sections = Vec::with_capacity(N);
    for length in lengths {
        let end = offset
            .checked_add(length)
            .ok_or(OpeningEnvelopeError::LengthOverflow("artifact sections"))?;
        let section = bytes
            .get(offset..end)
            .ok_or(OpeningEnvelopeError::Truncated("artifact sections"))?;
        sections.push(section);
        offset = end;
    }
    if offset != bytes.len() {
        return Err(OpeningEnvelopeError::NonCanonical(
            "artifact section lengths",
        ));
    }
    Ok(sections)
}

fn require_bytes(
    bytes: &[u8],
    offset: usize,
    expected: &[u8],
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    let end = offset
        .checked_add(expected.len())
        .ok_or(OpeningEnvelopeError::LengthOverflow(field))?;
    let actual = bytes
        .get(offset..end)
        .ok_or(OpeningEnvelopeError::Truncated(field))?;
    if actual != expected {
        Err(OpeningEnvelopeError::UnknownIdentity(field))
    } else {
        Ok(())
    }
}

fn read_array<const N: usize>(
    bytes: &[u8],
    offset: usize,
    field: &'static str,
) -> Result<[u8; N], OpeningEnvelopeError> {
    let end = offset
        .checked_add(N)
        .ok_or(OpeningEnvelopeError::LengthOverflow(field))?;
    bytes
        .get(offset..end)
        .ok_or(OpeningEnvelopeError::Truncated(field))?
        .try_into()
        .map_err(|_| OpeningEnvelopeError::Truncated(field))
}

fn read_u8(bytes: &[u8], offset: usize, field: &'static str) -> Result<u8, OpeningEnvelopeError> {
    bytes
        .get(offset)
        .copied()
        .ok_or(OpeningEnvelopeError::Truncated(field))
}

fn read_u16(bytes: &[u8], offset: usize, field: &'static str) -> Result<u16, OpeningEnvelopeError> {
    Ok(u16::from_le_bytes(read_array(bytes, offset, field)?))
}

fn read_u32(bytes: &[u8], offset: usize, field: &'static str) -> Result<u32, OpeningEnvelopeError> {
    Ok(u32::from_le_bytes(read_array(bytes, offset, field)?))
}

fn read_u64(bytes: &[u8], offset: usize, field: &'static str) -> Result<u64, OpeningEnvelopeError> {
    Ok(u64::from_le_bytes(read_array(bytes, offset, field)?))
}

fn require_u8(
    bytes: &[u8],
    offset: usize,
    expected: u8,
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    if read_u8(bytes, offset, field)? == expected {
        Ok(())
    } else {
        Err(OpeningEnvelopeError::UnknownIdentity(field))
    }
}

fn require_u16(
    bytes: &[u8],
    offset: usize,
    expected: u16,
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    if read_u16(bytes, offset, field)? == expected {
        Ok(())
    } else {
        Err(OpeningEnvelopeError::UnknownIdentity(field))
    }
}

fn require_u32(
    bytes: &[u8],
    offset: usize,
    expected: u32,
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    if read_u32(bytes, offset, field)? == expected {
        Ok(())
    } else {
        Err(OpeningEnvelopeError::UnknownIdentity(field))
    }
}

fn require_u64(
    bytes: &[u8],
    offset: usize,
    expected: u64,
    field: &'static str,
) -> Result<(), OpeningEnvelopeError> {
    if read_u64(bytes, offset, field)? == expected {
        Ok(())
    } else {
        Err(OpeningEnvelopeError::UnknownIdentity(field))
    }
}

fn usize_to_u8(value: usize, field: &'static str) -> Result<u8, OpeningEnvelopeError> {
    u8::try_from(value).map_err(|_| OpeningEnvelopeError::LengthOverflow(field))
}

fn usize_to_u16(value: usize, field: &'static str) -> Result<u16, OpeningEnvelopeError> {
    u16::try_from(value).map_err(|_| OpeningEnvelopeError::LengthOverflow(field))
}

fn usize_to_u32(value: usize, field: &'static str) -> Result<u32, OpeningEnvelopeError> {
    u32::try_from(value).map_err(|_| OpeningEnvelopeError::LengthOverflow(field))
}

fn decl_code(decl: Decl) -> u8 {
    match decl {
        Decl::PipTrump(pip) => pip.value(),
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 8,
    }
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fnv1a64(bytes: &[u8]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    fn reduced_payload_fixture() -> OpeningContext {
        let decl = Decl::NoTrump;
        let led = Context::Natural(Pip::new(6).expect("six is a pip"));
        let matching = decl.effective_incidence(led);
        let nonmatching = DominoSet::FULL.difference(matching);
        let pool: DominoSet = matching
            .iter()
            .take(3)
            .chain(nonmatching.iter().take(9))
            .collect();
        OpeningContext::try_reduced(decl, led, pool, 4).expect("grade-four payload fixture")
    }

    fn fixture_root() -> OpeningRootV1 {
        let hand: DominoSet = ["6-0", "6-1", "6-2", "6-3", "6-4", "6-5", "5-5"]
            .into_iter()
            .map(|name| name.parse::<Domino>().expect("explicit fixture tile"))
            .collect();
        OpeningRootV1::new(
            Decl::NoTrump,
            Seat::S0,
            hand,
            OpeningContractV1::point_bid(30).expect("point contract"),
        )
        .expect("fixture root")
    }

    #[test]
    fn public_root_key_encoder_is_the_existing_canonical_authority() {
        let root = fixture_root();
        let public = canonical_opening_root_key_bytes_v1(root).expect("public root key");
        let private = canonical_root_key_bytes(root).expect("private root key");
        assert_eq!(public.as_slice(), private);
        assert_eq!(&public[..8], &OPENING_ROOT_KEY_MAGIC);
        assert_eq!(public.len(), OPENING_ROOT_KEY_BYTES);
    }

    #[test]
    fn nonpersistable_projector_payload_v1_is_byte_stable_inside_the_crate() {
        let context = reduced_payload_fixture();
        let first_projection = project_closed_form(context).expect("first projection");
        let second_projection = project_closed_form(context).expect("second projection");
        let first = first_projection
            .canonical_projector_payload_bytes_v1()
            .expect("first payload render");
        let second = first_projection
            .canonical_projector_payload_bytes_v1()
            .expect("second payload render");
        let independent = second_projection
            .canonical_projector_payload_bytes_v1()
            .expect("independent payload render");

        assert_eq!(first_projection.cells().len(), 978);
        assert_eq!(context.physical_world_count(), Ok(34_650));
        assert_eq!(
            first_projection
                .total_scaled_mass()
                .expect("payload fixture mass")
                .value(),
            2_567_149_200_000
        );
        assert_eq!(first, second);
        assert_eq!(first, independent);
        assert_eq!(&first[..8], &OPENING_RECEIPT_MAGIC);
        assert_eq!(
            u16::from_le_bytes([first[8], first[9]]),
            OPENING_RECEIPT_VERSION
        );
        assert_eq!(
            u64::from_le_bytes(first[30..38].try_into().expect("cap field width")),
            M1_DIRECT_WORLD_CAP_V1
        );
        assert_eq!(
            u32::from_le_bytes(first[38..42].try_into().expect("cell-count field width")),
            978
        );
        assert_eq!(
            u64::from_le_bytes(first[42..50].try_into().expect("mass field width")),
            2_567_149_200_000
        );
        assert_eq!(
            first.len(),
            OPENING_RECEIPT_HEADER_BYTES
                + OPENING_RECEIPT_CELL_BYTES * first_projection.cells().len()
        );
        assert_eq!(fnv1a64(&first), 2_608_877_348_409_033_899);
    }

    #[test]
    fn self_consistently_rehashed_malformed_payload_still_fails_replay(
    ) -> Result<(), OpeningEnvelopeError> {
        let root = fixture_root();
        let action = "6-0".parse::<Domino>().expect("fixture action");
        let context = root.decl().led_context(action);
        let projection = project_closed_form(root.opening_context(context).expect("root context"))
            .expect("projection");
        let build = BuildIdentityV1::new([0xa5; 32]).expect("build identity");
        let mut bytes = projection
            .canonical_run_envelope_bytes(root, action, build)
            .expect("envelope");

        let freeze_len = read_u32(&bytes, ENVELOPE_FREEZE_LENGTH_OFFSET, "freeze")? as usize;
        let root_len = read_u32(&bytes, ENVELOPE_ROOT_LENGTH_OFFSET, "root")? as usize;
        let table_len = read_u32(&bytes, ENVELOPE_TABLE_LENGTH_OFFSET, "table")? as usize;
        let root_start = OPENING_ENVELOPE_HEADER_BYTES + freeze_len;
        let table_start = root_start + root_len;
        let payload_start = table_start + table_len;
        *bytes.last_mut().expect("payload byte") ^= 1;
        let payload_sha256 = sha256(&bytes[payload_start..]);
        bytes[ENVELOPE_PAYLOAD_SHA_OFFSET..ENVELOPE_PAYLOAD_SHA_OFFSET + 32]
            .copy_from_slice(&payload_sha256);
        let semantic_identity = semantic_identity_sha256(
            M1_OPENING_PROJECTOR_TASK_ID_V1,
            build,
            u8::try_from(action.index()).expect("action index"),
            u8::try_from(context.index()).expect("context index"),
            &bytes[root_start..table_start],
            &bytes[table_start..payload_start],
            &payload_sha256,
            &[],
        )?;
        bytes[OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET
            ..OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET + 32]
            .copy_from_slice(&semantic_identity);

        assert_eq!(
            validate_opening_run_envelope_v1(&bytes, build),
            Err(OpeningEnvelopeError::NonCanonical(
                "projector payload does not replay from root"
            ))
        );
        Ok(())
    }

    #[test]
    fn self_consistently_rehashed_action_context_mismatch_fails_closed(
    ) -> Result<(), OpeningEnvelopeError> {
        let root = fixture_root();
        let original_action = "6-0".parse::<Domino>().expect("fixture action");
        let mismatched_action = "5-5".parse::<Domino>().expect("mismatched action");
        let context = root.decl().led_context(original_action);
        let projection = project_closed_form(root.opening_context(context).expect("root context"))
            .expect("projection");
        let build = BuildIdentityV1::new([0xa5; 32]).expect("build identity");
        let mut bytes = projection
            .canonical_run_envelope_bytes(root, original_action, build)
            .expect("envelope");

        bytes[ENVELOPE_ACTION_OFFSET] =
            u8::try_from(mismatched_action.index()).expect("action index");
        let freeze_len = read_u32(&bytes, ENVELOPE_FREEZE_LENGTH_OFFSET, "freeze")? as usize;
        let root_len = read_u32(&bytes, ENVELOPE_ROOT_LENGTH_OFFSET, "root")? as usize;
        let table_len = read_u32(&bytes, ENVELOPE_TABLE_LENGTH_OFFSET, "table")? as usize;
        let root_start = OPENING_ENVELOPE_HEADER_BYTES + freeze_len;
        let table_start = root_start + root_len;
        let payload_start = table_start + table_len;
        let semantic_identity = semantic_identity_sha256(
            M1_OPENING_PROJECTOR_TASK_ID_V1,
            build,
            u8::try_from(mismatched_action.index()).expect("action index"),
            u8::try_from(context.index()).expect("context index"),
            &bytes[root_start..table_start],
            &bytes[table_start..payload_start],
            &sha256(&bytes[payload_start..]),
            &[],
        )?;
        bytes[OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET
            ..OPENING_ENVELOPE_SEMANTIC_IDENTITY_SHA256_OFFSET + 32]
            .copy_from_slice(&semantic_identity);

        assert!(matches!(
            validate_opening_run_envelope_v1(&bytes, build),
            Err(OpeningEnvelopeError::ActionContextMismatch { action, .. })
                if action == mismatched_action
        ));
        Ok(())
    }
}
