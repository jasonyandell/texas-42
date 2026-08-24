//! Portable M2 corpus, carrier and opening-slot validation.
//!
//! This module deliberately makes no Metal provenance claim.  Raw words can
//! become a `CheckedM2ProjectionPayloadV1` only by exact comparison with the
//! scalar projector, but only `walt-metal` may later join that token with a
//! completed retained command.

use core::cmp::Ordering;
use core::fmt;
use std::sync::OnceLock;

use num_bigint::BigUint;
use walt::rules::{Context, Decl, Domino, DominoSet, Pip, Seat};
use walt::spec::{sha256, SemanticTables, Sha256State, U256Mass, FIELD_SCALE};

use crate::{
    direct_preflight, project_closed_form, DirectPreflightV1, OpeningContext, OpeningContractV1,
    OpeningError, OpeningProjection, OpeningRootV1, ReducedOpeningCarrierV1,
    M1_DIRECT_WORLD_CAP_V1, MAX_OPENING_CELLS_V1, OPENING_RECEIPT_CELL_BYTES,
    OPENING_RECEIPT_HEADER_BYTES, OPENING_RECEIPT_MAGIC, OPENING_RECEIPT_VERSION,
};

pub const M2_CONTEXT_TASK_COUNT_V1: usize = 614;
pub const M2_REDUCED_BINDING_COUNT_V1: usize = 103;
pub const M2_PHYSICAL_BINDING_COUNT_V1: usize = 1_015;
pub const M2_DIRECT_PARITY_COUNT_V1: usize = 73;
pub const M2_DIRECT_STOP_COUNT_V1: usize = 541;

pub const OPENING_TASK_WORDS_V1: usize = 8;
pub const OPENING_SLOT_WORDS_V1: usize = 16;
pub const OPENING_CHOOSE_DIM_V1: usize = 22;
pub const OPENING_CHOOSE_WORDS_V1: usize = OPENING_CHOOSE_DIM_V1 * OPENING_CHOOSE_DIM_V1;
pub const OPENING_CANDIDATE_SLOT_CAP_V1: usize = 79_800;
pub const OPENING_ARENA_SLOT_COUNT_V1: usize = OPENING_CANDIDATE_SLOT_CAP_V1 + 2;
pub const OPENING_NEGATIVE_CONTROL_COUNT_V1: usize = 13;
pub const OPENING_ABI_VERSION_V1: u32 = 1;
pub const OPENING_SLOT_SKIP_V1: u32 = 0;
pub const OPENING_SLOT_VALID_V1: u32 = 1;
pub const OPENING_HARD_BAD_ABI_V1: u32 = 0x8000_0001;
pub const OPENING_HARD_BAD_MASK_V1: u32 = 0x8000_0002;
pub const OPENING_HARD_BAD_COUNT_V1: u32 = 0x8000_0003;
pub const OPENING_HARD_BAD_RESPONSE_ORDINAL_V1: u32 = 0x8000_0004;
pub const OPENING_HARD_TOO_MANY_STRATA_V1: u32 = 0x8000_0005;
pub const OPENING_HARD_CHOOSE_INDEX_V1: u32 = 0x8000_0006;
pub const OPENING_HARD_SUPPORT_OVERFLOW_V1: u32 = 0x8000_0007;
pub const OPENING_HARD_COEFFICIENT_OVERFLOW_V1: u32 = 0x8000_0008;
pub const OPENING_HARD_MASS_OVERFLOW_V1: u32 = 0x8000_0009;
pub const OPENING_MAX_CHOOSE_ENTRY_V1: u32 = 352_716;
pub const OPENING_MAX_CELL_SUPPORT_V1: u32 = 17_153_136;
pub const OPENING_MAX_CELL_COEFFICIENT_V1: u64 = 74_088_000;
pub const OPENING_MAX_CELL_MASS_V1: u64 = 1_270_841_539_968_000;
pub const OPENING_MAX_WHOLE_MASS_V1: u64 = 29_566_517_460_480_000;
pub const M2_POISON_WORD_V1: u32 = 0xA5A5_5A5A;

pub const ARITHMETIC_ABI_VERSION_V1: u32 = 1;
pub const ARITHMETIC_INPUT_WORDS_V1: usize = 20;
pub const ARITHMETIC_OUTPUT_WORDS_V1: usize = 16;
pub const M2_ARITHMETIC_CASE_COUNT_V1: usize = 16_384;
pub const ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1: usize = 13;
pub const ARITHMETIC_OP_CHECKED_ADD_V1: u32 = 1;
pub const ARITHMETIC_OP_CHECKED_SUB_V1: u32 = 2;
pub const ARITHMETIC_OP_CHECKED_MUL_SMALL_V1: u32 = 3;
pub const ARITHMETIC_OP_CHECKED_MUL_POW_420_V1: u32 = 4;
pub const ARITHMETIC_OP_COMPARE_V1: u32 = 5;
pub const ARITHMETIC_SUCCESS_V1: u32 = 1;
pub const ARITHMETIC_CHECKED_UNDEFINED_V1: u32 = 2;
pub const ARITHMETIC_HARD_BAD_ABI_V1: u32 = 0x8000_0001;
pub const ARITHMETIC_HARD_BAD_OPERATION_V1: u32 = 0x8000_0002;
pub const ARITHMETIC_HARD_BAD_OPERAND_V1: u32 = 0x8000_0003;
pub const ARITHMETIC_HARD_BAD_UNUSED_RHS_V1: u32 = 0x8000_0004;
pub const ARITHMETIC_HARD_BAD_EXPONENT_V1: u32 = 0x8000_0005;
pub const ARITHMETIC_HARD_INTERNAL_V1: u32 = 0x8000_0006;

const STREAM_MAGIC_V1: &[u8; 8] = b"W42M2DG1";
const STREAM_VERSION_V1: u32 = 1;
const STREAM_TASK_KEYS_V1: u32 = 1;
const STREAM_ARITHMETIC_INPUT_V1: u32 = 2;
const STREAM_ARITHMETIC_OUTPUT_V1: u32 = 3;
const STREAM_CONTEXT_SLOTS_V1: u32 = 4;
const STREAM_CONTEXT_PAYLOAD_V1: u32 = 5;
const STREAM_CONTEXT_AGGREGATES_V1: u32 = 6;
const STREAM_PROTECTED_RECORDS_V1: u32 = 9;
const STREAM_GLOBAL_SLOTS_V1: u32 = 12;
const STREAM_GLOBAL_PAYLOAD_V1: u32 = 13;
const STREAM_GLOBAL_AGGREGATES_V1: u32 = 14;
const GLOBAL_RECORD_HEADER_BYTES_V1: u64 = 16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum M2BridgeError {
    Opening(OpeningError),
    Binding(M2BindingError),
    Arithmetic(&'static str),
    Carrier(&'static str),
    ChooseOracleMismatch {
        n: u8,
        k: u8,
        extracted: u32,
        oracle: u32,
    },
    ArenaLength {
        expected: usize,
        actual: usize,
    },
    SlotMismatch {
        slot: u32,
        word: u8,
        expected: u32,
        actual: u32,
    },
    ProtectedWordMismatch {
        slot: u32,
        word: u8,
        actual: u32,
    },
    MalformedValidSlot {
        slot: u32,
        field: &'static str,
    },
    FrozenBoundExceeded {
        field: &'static str,
        maximum: u64,
        actual: u64,
    },
    TooManyCells {
        actual: usize,
        cap: usize,
    },
    PayloadMismatch,
    AggregateMismatch,
    TaskMismatch,
    GlobalTaskOrder {
        expected: u32,
        actual: u32,
    },
    GlobalTaskIdentity,
    GlobalAlreadyComplete,
    GlobalIncomplete {
        expected: u32,
        actual: u32,
    },
    GlobalStream(&'static str),
}

impl fmt::Display for M2BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opening(error) => write!(f, "M2 opening reference failed: {error}"),
            Self::Binding(error) => write!(f, "M2 opening binding failed: {error}"),
            Self::Arithmetic(field) => write!(f, "M2 arithmetic corpus failed at {field}"),
            Self::Carrier(field) => write!(f, "M2 carrier failed at {field}"),
            Self::ChooseOracleMismatch {
                n,
                k,
                extracted,
                oracle,
            } => write!(
                f,
                "choose({n},{k}) extracted {extracted}, independent oracle {oracle}"
            ),
            Self::ArenaLength { expected, actual } => {
                write!(f, "M2 arena has {actual} records, expected {expected}")
            }
            Self::SlotMismatch {
                slot,
                word,
                expected,
                actual,
            } => write!(
                f,
                "M2 slot {slot} word {word} is {actual:#010x}, expected {expected:#010x}"
            ),
            Self::ProtectedWordMismatch { slot, word, actual } => write!(
                f,
                "M2 protected slot {slot} word {word} changed to {actual:#010x}"
            ),
            Self::MalformedValidSlot { slot, field } => {
                write!(f, "M2 valid slot {slot} has malformed {field}")
            }
            Self::FrozenBoundExceeded {
                field,
                maximum,
                actual,
            } => write!(f, "M2 {field} {actual} exceeds frozen maximum {maximum}"),
            Self::TooManyCells { actual, cap } => {
                write!(f, "M2 compacted {actual} cells, exceeding cap {cap}")
            }
            Self::PayloadMismatch => f.write_str("M2 compacted payload differs from scalar bytes"),
            Self::AggregateMismatch => {
                f.write_str("M2 response aggregates differ from scalar aggregates")
            }
            Self::TaskMismatch => f.write_str("M2 checked payload is bound to a different task"),
            Self::GlobalTaskOrder { expected, actual } => write!(
                f,
                "M2 global accumulator expected task {expected}, received {actual}"
            ),
            Self::GlobalTaskIdentity => {
                f.write_str("M2 global accumulator received a noncanonical task identity")
            }
            Self::GlobalAlreadyComplete => {
                f.write_str("M2 global accumulator already accepted all tasks")
            }
            Self::GlobalIncomplete { expected, actual } => write!(
                f,
                "M2 global accumulator accepted {actual} tasks, expected {expected}"
            ),
            Self::GlobalStream(field) => {
                write!(f, "M2 global stream invariant failed at {field}")
            }
        }
    }
}

impl std::error::Error for M2BridgeError {}

impl From<OpeningError> for M2BridgeError {
    fn from(value: OpeningError) -> Self {
        Self::Opening(value)
    }
}

impl From<M2BindingError> for M2BridgeError {
    fn from(value: M2BindingError) -> Self {
        Self::Binding(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum M2BindingError {
    TaskMismatch,
    CarrierInvariant(&'static str),
}

impl fmt::Display for M2BindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TaskMismatch => f.write_str("checked M2 payload belongs to another task"),
            Self::CarrierInvariant(field) => write!(f, "M2 binding invariant failed at {field}"),
        }
    }
}

impl std::error::Error for M2BindingError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpeningChooseTableV1 {
    words: [u32; OPENING_CHOOSE_WORDS_V1],
    sha256: [u8; 32],
}

impl OpeningChooseTableV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        let tables = SemanticTables::from_walt_core();
        let mut words = [0u32; OPENING_CHOOSE_WORDS_V1];
        for n in 0..OPENING_CHOOSE_DIM_V1 {
            for k in 0..OPENING_CHOOSE_DIM_V1 {
                let n_u8 = u8::try_from(n).map_err(|_| M2BridgeError::Arithmetic("choose n"))?;
                let k_u8 = u8::try_from(k).map_err(|_| M2BridgeError::Arithmetic("choose k"))?;
                let extracted = tables
                    .choose(n_u8, k_u8)
                    .ok_or(M2BridgeError::Arithmetic("choose extraction"))?;
                let oracle = big_binomial_u32(n_u8, k_u8)?;
                if extracted != oracle {
                    return Err(M2BridgeError::ChooseOracleMismatch {
                        n: n_u8,
                        k: k_u8,
                        extracted,
                        oracle,
                    });
                }
                require_frozen_bound(
                    "choose entry",
                    u64::from(extracted),
                    u64::from(OPENING_MAX_CHOOSE_ENTRY_V1),
                )?;
                words[n * OPENING_CHOOSE_DIM_V1 + k] = extracted;
            }
        }
        let sha256 = sha256(&words_to_le_bytes(&words));
        Ok(Self { words, sha256 })
    }

    pub const fn words(&self) -> &[u32; OPENING_CHOOSE_WORDS_V1] {
        &self.words
    }

    pub const fn digest(&self) -> [u8; 32] {
        self.sha256
    }
}

fn big_binomial_u32(n: u8, k: u8) -> Result<u32, M2BridgeError> {
    if k > n {
        return Ok(0);
    }
    let k = k.min(n - k);
    let mut value = BigUint::from(1u8);
    for step in 0..k {
        value *= BigUint::from(n - step);
        value /= BigUint::from(step + 1);
    }
    big_to_u32(&value).ok_or(M2BridgeError::Arithmetic("choose oracle width"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum M2CarrierArmV1 {
    Reduced,
    GradeMatching,
    SameContextPair,
}

impl M2CarrierArmV1 {
    pub const fn code(self) -> u32 {
        match self {
            Self::Reduced => 1,
            Self::GradeMatching => 2,
            Self::SameContextPair => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReducedCarrierProfileV1 {
    ReducedArm,
    GradeMatching,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct M2TaskKeyV1 {
    words: [u32; 16],
}

impl M2TaskKeyV1 {
    pub const fn words(self) -> [u32; 16] {
        self.words
    }

    pub fn to_le_bytes(self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for (index, word) in self.words.iter().enumerate() {
            bytes[index * 4..index * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
        bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReducedBindingIntentV1 {
    profile: ReducedCarrierProfileV1,
    binding_ordinal: u32,
    task_ordinal: u32,
    arm_ordinal: u32,
    root_selector: u32,
    coordinate_ordinal: u32,
    root: OpeningRootV1,
    selected_action: Domino,
    context: OpeningContext,
    matching_count: u8,
    generator: [u32; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhysicalBindingIntentV1 {
    binding_ordinal: u32,
    task_ordinal: u32,
    arm: M2CarrierArmV1,
    arm_ordinal: u32,
    endpoint: u32,
    root: OpeningRootV1,
    selected_action: Domino,
    context: OpeningContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BindingIntentV1 {
    Reduced(ReducedBindingIntentV1),
    Physical(PhysicalBindingIntentV1),
}

/// Read-only crate-internal projection of the closed carrier's binding
/// intents. Receipt persistence uses this view to regenerate exact roots,
/// actions, coordinates, and contexts without exposing an intent constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CanonicalBindingFactV1 {
    Reduced {
        binding_ordinal: u32,
        task_ordinal: u32,
        arm: M2CarrierArmV1,
        arm_ordinal: u32,
        root: OpeningRootV1,
        selected_action: Domino,
        context: OpeningContext,
        matching_count: u8,
    },
    Physical {
        binding_ordinal: u32,
        task_ordinal: u32,
        arm: M2CarrierArmV1,
        arm_ordinal: u32,
        endpoint: u32,
        root: OpeningRootV1,
        selected_action: Domino,
        context: OpeningContext,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M2OpeningTaskV1 {
    ordinal: u32,
    arm: M2CarrierArmV1,
    arm_ordinal: u32,
    context: OpeningContext,
    task_words: [u32; OPENING_TASK_WORDS_V1],
    task_key: M2TaskKeyV1,
    task_key_sha256: [u8; 32],
    direct: DirectPreflightV1,
    bindings: Box<[BindingIntentV1]>,
}

impl M2OpeningTaskV1 {
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub const fn arm(&self) -> M2CarrierArmV1 {
        self.arm
    }

    pub const fn arm_ordinal(&self) -> u32 {
        self.arm_ordinal
    }

    pub const fn context(&self) -> OpeningContext {
        self.context
    }

    pub const fn task_words(&self) -> &[u32; OPENING_TASK_WORDS_V1] {
        &self.task_words
    }

    pub const fn task_key(&self) -> M2TaskKeyV1 {
        self.task_key
    }

    pub const fn task_key_sha256(&self) -> [u8; 32] {
        self.task_key_sha256
    }

    pub const fn direct_preflight(&self) -> DirectPreflightV1 {
        self.direct
    }

    pub const fn response_count(&self) -> u32 {
        self.task_words[6]
    }

    pub const fn candidate_slot_count(&self) -> u32 {
        self.task_words[7]
    }

    pub fn reduced_binding_count(&self) -> usize {
        self.bindings
            .iter()
            .filter(|intent| matches!(intent, BindingIntentV1::Reduced(_)))
            .count()
    }

    pub fn physical_binding_count(&self) -> usize {
        self.bindings
            .iter()
            .filter(|intent| matches!(intent, BindingIntentV1::Physical(_)))
            .count()
    }

    pub(crate) fn canonical_binding_facts_v1(
        &self,
    ) -> impl Iterator<Item = CanonicalBindingFactV1> + '_ {
        self.bindings.iter().map(move |intent| match intent {
            BindingIntentV1::Reduced(intent) => CanonicalBindingFactV1::Reduced {
                binding_ordinal: intent.binding_ordinal,
                task_ordinal: intent.task_ordinal,
                arm: self.arm,
                arm_ordinal: intent.arm_ordinal,
                root: intent.root,
                selected_action: intent.selected_action,
                context: intent.context,
                matching_count: intent.matching_count,
            },
            BindingIntentV1::Physical(intent) => CanonicalBindingFactV1::Physical {
                binding_ordinal: intent.binding_ordinal,
                task_ordinal: intent.task_ordinal,
                arm: intent.arm,
                arm_ordinal: intent.arm_ordinal,
                endpoint: intent.endpoint,
                root: intent.root,
                selected_action: intent.selected_action,
                context: intent.context,
            },
        })
    }

    pub fn render_expected_slot_words_v1(
        &self,
    ) -> Result<Box<[[u32; OPENING_SLOT_WORDS_V1]]>, M2BridgeError> {
        let projection = project_closed_form(self.context)?;
        render_expected_slots(self, &projection)
    }

    pub fn validate_slot_words_v1(
        &self,
        arena: &[[u32; OPENING_SLOT_WORDS_V1]],
    ) -> Result<CheckedM2ProjectionPayloadV1, M2BridgeError> {
        validate_slot_arena(self, arena)
    }

    pub fn bind_checked_payload_v1(
        &self,
        checked: CheckedM2ProjectionPayloadV1,
    ) -> Result<M2BoundOpeningTaskV1, M2BindingError> {
        if checked.task_ordinal != self.ordinal || checked.task_key_sha256 != self.task_key_sha256 {
            return Err(M2BindingError::TaskMismatch);
        }
        let mut reduced = Vec::new();
        let mut physical = Vec::new();
        for intent in &self.bindings {
            match intent {
                BindingIntentV1::Reduced(intent) => {
                    validate_reduced_intent(self, intent)?;
                    reduced.push(ReducedEvidenceBindingV1 {
                        profile: intent.profile,
                        binding_ordinal: intent.binding_ordinal,
                        task_ordinal: intent.task_ordinal,
                        arm_ordinal: intent.arm_ordinal,
                        root_selector: intent.root_selector,
                        coordinate_ordinal: intent.coordinate_ordinal,
                        root: intent.root,
                        selected_action: intent.selected_action,
                        context: intent.context,
                        matching_count: intent.matching_count,
                        generator: intent.generator,
                        payload_length: checked.payload_length,
                        payload_sha256: checked.gpu_payload_sha256,
                    });
                }
                BindingIntentV1::Physical(intent) => {
                    validate_physical_intent(self, intent)?;
                    physical.push(PhysicalActionBindingV1 {
                        binding_ordinal: intent.binding_ordinal,
                        task_ordinal: intent.task_ordinal,
                        arm: intent.arm,
                        arm_ordinal: intent.arm_ordinal,
                        endpoint: intent.endpoint,
                        root: intent.root,
                        selected_action: intent.selected_action,
                        context: intent.context,
                        payload_length: checked.payload_length,
                        payload_sha256: checked.gpu_payload_sha256,
                    });
                }
            }
        }
        Ok(M2BoundOpeningTaskV1 {
            checked,
            reduced: reduced.into_boxed_slice(),
            physical: physical.into_boxed_slice(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M2OpeningParityCarrierV1 {
    tasks: Box<[M2OpeningTaskV1]>,
    task_key_stream_sha256: [u8; 32],
}

impl M2OpeningParityCarrierV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        generate_carrier()
    }

    pub fn tasks(&self) -> &[M2OpeningTaskV1] {
        &self.tasks
    }

    pub const fn task_key_stream_sha256(&self) -> [u8; 32] {
        self.task_key_stream_sha256
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GlobalTaskPayloadLengthsV1 {
    raw: u64,
    payload: u64,
    aggregates: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GlobalPayloadLengthsV1 {
    raw: u64,
    payload: u64,
    aggregates: u64,
}

type CachedGlobalPayloadLengthsV1 =
    Result<(GlobalPayloadLengthsV1, Box<[GlobalTaskPayloadLengthsV1]>), M2BridgeError>;

static GLOBAL_PAYLOAD_LENGTHS_V1: OnceLock<CachedGlobalPayloadLengthsV1> = OnceLock::new();

/// Canonical scalar facts for one official context receipt record.
///
/// These are deliberately receipt-shaped values, not evidence that Metal ran.
/// A receipt verifier can use them to close the canonical half of each exact
/// CPU/GPU equality without reconstructing or retaining the 79,802-slot arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct M2CanonicalContextReceiptFactsV1 {
    pub(crate) task_ordinal: u32,
    pub(crate) accepted_cells: u32,
    pub(crate) total_scaled_mass: U256Mass,
    pub(crate) canonical_payload_bytes: u64,
    pub(crate) raw_sha256: [u8; 32],
    pub(crate) payload_sha256: [u8; 32],
    pub(crate) aggregate_sha256: [u8; 32],
    pub(crate) tail_guard_sha256: [u8; 32],
}

/// Canonical purpose-12/13/14 digests over all 614 task-framed records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct M2CanonicalGlobalReceiptFactsV1 {
    pub(crate) raw_sha256: [u8; 32],
    pub(crate) payload_sha256: [u8; 32],
    pub(crate) aggregate_sha256: [u8; 32],
}

/// Canonical opening facts needed to adjudicate the context and global receipt
/// fields.  The contexts are in frozen task-ordinal order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct M2CanonicalOpeningReceiptFactsV1 {
    pub(crate) contexts: Box<[M2CanonicalContextReceiptFactsV1]>,
    pub(crate) global: M2CanonicalGlobalReceiptFactsV1,
}

/// Canonical facts for one official or negative arithmetic run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct M2CanonicalArithmeticRunReceiptFactsV1 {
    pub(crate) case_count: u32,
    pub(crate) accepted_count: u32,
    pub(crate) input_payload_bytes: u64,
    pub(crate) output_payload_bytes: u64,
    pub(crate) success_count: u32,
    pub(crate) checked_undefined_count: u32,
    pub(crate) hard_count: u32,
    pub(crate) input_sha256: [u8; 32],
    pub(crate) output_sha256: [u8; 32],
    pub(crate) guard_sha256: [u8; 32],
}

/// Canonical arithmetic receipt facts, official corpus first and the thirteen
/// malformed negative controls second.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct M2CanonicalArithmeticReceiptFactsV1 {
    pub(crate) official: M2CanonicalArithmeticRunReceiptFactsV1,
    pub(crate) negative: M2CanonicalArithmeticRunReceiptFactsV1,
}

type CachedCanonicalOpeningReceiptFactsV1 = Result<M2CanonicalOpeningReceiptFactsV1, M2BridgeError>;
type CachedCanonicalArithmeticReceiptFactsV1 =
    Result<M2CanonicalArithmeticReceiptFactsV1, M2BridgeError>;

static CANONICAL_OPENING_RECEIPT_FACTS_V1: OnceLock<CachedCanonicalOpeningReceiptFactsV1> =
    OnceLock::new();
static CANONICAL_ARITHMETIC_RECEIPT_FACTS_V1: OnceLock<CachedCanonicalArithmeticReceiptFactsV1> =
    OnceLock::new();

struct FixedStreamHasherV1 {
    state: Sha256State,
    expected_records: u64,
    observed_records: u64,
    expected_payload_bytes: u64,
    observed_payload_bytes: u64,
}

impl FixedStreamHasherV1 {
    fn new(purpose: u32, expected_records: u64, expected_payload_bytes: u64) -> Self {
        let mut state = Sha256State::new();
        state.update(STREAM_MAGIC_V1);
        state.update(&purpose.to_le_bytes());
        state.update(&STREAM_VERSION_V1.to_le_bytes());
        state.update(&expected_records.to_le_bytes());
        state.update(&expected_payload_bytes.to_le_bytes());
        Self {
            state,
            expected_records,
            observed_records: 0,
            expected_payload_bytes,
            observed_payload_bytes: 0,
        }
    }

    fn validate_record(&self, payload_len: u64) -> Result<(), M2BridgeError> {
        let next_records = self
            .observed_records
            .checked_add(1)
            .ok_or(M2BridgeError::GlobalStream("record count overflow"))?;
        let framed_len = GLOBAL_RECORD_HEADER_BYTES_V1
            .checked_add(payload_len)
            .ok_or(M2BridgeError::GlobalStream("framed record length"))?;
        let next_bytes = self
            .observed_payload_bytes
            .checked_add(framed_len)
            .ok_or(M2BridgeError::GlobalStream("payload byte count overflow"))?;
        if next_records > self.expected_records || next_bytes > self.expected_payload_bytes {
            return Err(M2BridgeError::GlobalStream("precomputed stream extent"));
        }
        Ok(())
    }

    fn append_validated_record(&mut self, task_ordinal: u32, payload_len: u64, payload: &[u8]) {
        debug_assert_eq!(usize::try_from(payload_len), Ok(payload.len()));
        self.state.update(&task_ordinal.to_le_bytes());
        self.state.update(&0u32.to_le_bytes());
        self.state.update(&payload_len.to_le_bytes());
        self.state.update(payload);
        self.observed_records += 1;
        self.observed_payload_bytes += GLOBAL_RECORD_HEADER_BYTES_V1 + payload_len;
    }

    fn finish(self) -> Result<[u8; 32], M2BridgeError> {
        if self.observed_records != self.expected_records {
            return Err(M2BridgeError::GlobalStream("final record count"));
        }
        if self.observed_payload_bytes != self.expected_payload_bytes {
            return Err(M2BridgeError::GlobalStream("final payload byte count"));
        }
        Ok(self.state.finish())
    }
}

/// Six exact purpose-12/13/14 global parity digests.
///
/// Values can only be produced by completing the opaque accumulator over all
/// 614 canonical tasks.  No stream bytes or caller-supplied digests enter this
/// type.
#[derive(Debug, PartialEq, Eq)]
pub struct M2GlobalParityDigestsV1 {
    cpu_raw_sha256: [u8; 32],
    gpu_raw_sha256: [u8; 32],
    cpu_payload_sha256: [u8; 32],
    gpu_payload_sha256: [u8; 32],
    cpu_aggregate_sha256: [u8; 32],
    gpu_aggregate_sha256: [u8; 32],
}

impl M2GlobalParityDigestsV1 {
    pub const fn cpu_raw_sha256(&self) -> [u8; 32] {
        self.cpu_raw_sha256
    }

    pub const fn gpu_raw_sha256(&self) -> [u8; 32] {
        self.gpu_raw_sha256
    }

    pub const fn cpu_payload_sha256(&self) -> [u8; 32] {
        self.cpu_payload_sha256
    }

    pub const fn gpu_payload_sha256(&self) -> [u8; 32] {
        self.gpu_payload_sha256
    }

    pub const fn cpu_aggregate_sha256(&self) -> [u8; 32] {
        self.cpu_aggregate_sha256
    }

    pub const fn gpu_aggregate_sha256(&self) -> [u8; 32] {
        self.gpu_aggregate_sha256
    }
}

/// Opaque strict-order accumulator for the six frozen global parity streams.
///
/// The accumulator owns its canonical carrier, precomputes all three exact
/// framed stream extents before accepting a result, and retains only SHA-256
/// state—not raw, compact payload or aggregate streams.  Every accepted arena
/// passes the same full slot validator used by [`M2OpeningTaskV1`].
pub struct M2GlobalParityAccumulatorV1 {
    carrier: M2OpeningParityCarrierV1,
    task_lengths: Box<[GlobalTaskPayloadLengthsV1]>,
    next_task_ordinal: u32,
    cpu_raw: FixedStreamHasherV1,
    gpu_raw: FixedStreamHasherV1,
    cpu_payload: FixedStreamHasherV1,
    gpu_payload: FixedStreamHasherV1,
    cpu_aggregates: FixedStreamHasherV1,
    gpu_aggregates: FixedStreamHasherV1,
}

impl M2GlobalParityAccumulatorV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        let carrier = M2OpeningParityCarrierV1::canonical()?;
        let (global_lengths, task_lengths) = cached_global_payload_lengths(&carrier)?;
        let record_count = u64::try_from(M2_CONTEXT_TASK_COUNT_V1)
            .map_err(|_| M2BridgeError::GlobalStream("canonical record count"))?;
        Ok(Self {
            carrier,
            task_lengths,
            next_task_ordinal: 0,
            cpu_raw: FixedStreamHasherV1::new(
                STREAM_GLOBAL_SLOTS_V1,
                record_count,
                global_lengths.raw,
            ),
            gpu_raw: FixedStreamHasherV1::new(
                STREAM_GLOBAL_SLOTS_V1,
                record_count,
                global_lengths.raw,
            ),
            cpu_payload: FixedStreamHasherV1::new(
                STREAM_GLOBAL_PAYLOAD_V1,
                record_count,
                global_lengths.payload,
            ),
            gpu_payload: FixedStreamHasherV1::new(
                STREAM_GLOBAL_PAYLOAD_V1,
                record_count,
                global_lengths.payload,
            ),
            cpu_aggregates: FixedStreamHasherV1::new(
                STREAM_GLOBAL_AGGREGATES_V1,
                record_count,
                global_lengths.aggregates,
            ),
            gpu_aggregates: FixedStreamHasherV1::new(
                STREAM_GLOBAL_AGGREGATES_V1,
                record_count,
                global_lengths.aggregates,
            ),
        })
    }

    pub const fn accepted_task_count(&self) -> u32 {
        self.next_task_ordinal
    }

    pub fn next_task(&self) -> Option<&M2OpeningTaskV1> {
        self.carrier.tasks.get(self.next_task_ordinal as usize)
    }

    /// Validates and advances the next canonical task without accepting a
    /// caller-supplied task identity.  A runner may copy [`next_task`] words,
    /// release that borrow for dispatch, then submit only the completed arena.
    ///
    /// [`next_task`]: Self::next_task
    pub fn accept_next_task_slot_words_v1(
        &mut self,
        arena: &[[u32; OPENING_SLOT_WORDS_V1]],
    ) -> Result<M2BoundOpeningTaskV1, M2BridgeError> {
        let task = self
            .next_task()
            .cloned()
            .ok_or(M2BridgeError::GlobalAlreadyComplete)?;
        self.accept_task_slot_words_v1(&task, arena)
    }

    /// Identity-checking variant retained for adversarial order and carrier
    /// tests.  Production integration can use
    /// [`accept_next_task_slot_words_v1`] to avoid a caller-supplied identity.
    ///
    /// [`accept_next_task_slot_words_v1`]: Self::accept_next_task_slot_words_v1
    pub fn accept_task_slot_words_v1(
        &mut self,
        task: &M2OpeningTaskV1,
        arena: &[[u32; OPENING_SLOT_WORDS_V1]],
    ) -> Result<M2BoundOpeningTaskV1, M2BridgeError> {
        let expected_task = self
            .carrier
            .tasks
            .get(self.next_task_ordinal as usize)
            .ok_or(M2BridgeError::GlobalAlreadyComplete)?;
        if task.ordinal != self.next_task_ordinal {
            return Err(M2BridgeError::GlobalTaskOrder {
                expected: self.next_task_ordinal,
                actual: task.ordinal,
            });
        }
        if task != expected_task {
            return Err(M2BridgeError::GlobalTaskIdentity);
        }
        let validated = validate_slot_arena_detailed(expected_task, arena)?;
        let task_index = usize::try_from(self.next_task_ordinal)
            .map_err(|_| M2BridgeError::GlobalStream("task index"))?;
        let expected_lengths = *self
            .task_lengths
            .get(task_index)
            .ok_or(M2BridgeError::GlobalStream("task length index"))?;
        let actual_lengths = GlobalTaskPayloadLengthsV1 {
            raw: u64::try_from(validated.gpu_raw_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("raw task length"))?,
            payload: u64::try_from(validated.gpu_payload_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("payload task length"))?,
            aggregates: u64::try_from(validated.gpu_aggregate_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("aggregate task length"))?,
        };
        if actual_lengths != expected_lengths
            || validated.cpu_raw_bytes.len() != validated.gpu_raw_bytes.len()
            || validated.cpu_payload_bytes.len() != validated.gpu_payload_bytes.len()
            || validated.cpu_aggregate_bytes.len() != validated.gpu_aggregate_bytes.len()
        {
            return Err(M2BridgeError::GlobalStream("per-task payload extent"));
        }

        self.cpu_raw.validate_record(expected_lengths.raw)?;
        self.gpu_raw.validate_record(expected_lengths.raw)?;
        self.cpu_payload.validate_record(expected_lengths.payload)?;
        self.gpu_payload.validate_record(expected_lengths.payload)?;
        self.cpu_aggregates
            .validate_record(expected_lengths.aggregates)?;
        self.gpu_aggregates
            .validate_record(expected_lengths.aggregates)?;

        let bound = expected_task.bind_checked_payload_v1(validated.checked)?;
        let ordinal = self.next_task_ordinal;
        self.cpu_raw.append_validated_record(
            ordinal,
            expected_lengths.raw,
            &validated.cpu_raw_bytes,
        );
        self.gpu_raw.append_validated_record(
            ordinal,
            expected_lengths.raw,
            &validated.gpu_raw_bytes,
        );
        self.cpu_payload.append_validated_record(
            ordinal,
            expected_lengths.payload,
            &validated.cpu_payload_bytes,
        );
        self.gpu_payload.append_validated_record(
            ordinal,
            expected_lengths.payload,
            &validated.gpu_payload_bytes,
        );
        self.cpu_aggregates.append_validated_record(
            ordinal,
            expected_lengths.aggregates,
            &validated.cpu_aggregate_bytes,
        );
        self.gpu_aggregates.append_validated_record(
            ordinal,
            expected_lengths.aggregates,
            &validated.gpu_aggregate_bytes,
        );
        self.next_task_ordinal = self
            .next_task_ordinal
            .checked_add(1)
            .ok_or(M2BridgeError::GlobalStream("next task ordinal"))?;
        Ok(bound)
    }

    pub fn finish(self) -> Result<M2GlobalParityDigestsV1, M2BridgeError> {
        let expected = u32::try_from(M2_CONTEXT_TASK_COUNT_V1)
            .map_err(|_| M2BridgeError::GlobalStream("finish task count"))?;
        if self.next_task_ordinal != expected {
            return Err(M2BridgeError::GlobalIncomplete {
                expected,
                actual: self.next_task_ordinal,
            });
        }
        let digests = M2GlobalParityDigestsV1 {
            cpu_raw_sha256: self.cpu_raw.finish()?,
            gpu_raw_sha256: self.gpu_raw.finish()?,
            cpu_payload_sha256: self.cpu_payload.finish()?,
            gpu_payload_sha256: self.gpu_payload.finish()?,
            cpu_aggregate_sha256: self.cpu_aggregates.finish()?,
            gpu_aggregate_sha256: self.gpu_aggregates.finish()?,
        };
        if digests.cpu_raw_sha256 != digests.gpu_raw_sha256
            || digests.cpu_payload_sha256 != digests.gpu_payload_sha256
            || digests.cpu_aggregate_sha256 != digests.gpu_aggregate_sha256
        {
            return Err(M2BridgeError::GlobalStream("final CPU/GPU parity"));
        }
        Ok(digests)
    }
}

fn precompute_global_payload_lengths(
    carrier: &M2OpeningParityCarrierV1,
) -> Result<(GlobalPayloadLengthsV1, Box<[GlobalTaskPayloadLengthsV1]>), M2BridgeError> {
    if carrier.tasks.len() != M2_CONTEXT_TASK_COUNT_V1 {
        return Err(M2BridgeError::GlobalStream("carrier task count"));
    }
    let mut global = GlobalPayloadLengthsV1 {
        raw: 0,
        payload: 0,
        aggregates: 0,
    };
    let mut tasks = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1);
    for task in &carrier.tasks {
        let projection = project_closed_form(task.context)?;
        let raw = u64::from(task.candidate_slot_count())
            .checked_mul((OPENING_SLOT_WORDS_V1 * 4) as u64)
            .ok_or(M2BridgeError::GlobalStream("precomputed raw length"))?;
        let payload = u64::try_from(
            OPENING_RECEIPT_HEADER_BYTES
                .checked_add(
                    projection
                        .cells()
                        .len()
                        .checked_mul(OPENING_RECEIPT_CELL_BYTES)
                        .ok_or(M2BridgeError::GlobalStream("precomputed M1 cell bytes"))?,
                )
                .ok_or(M2BridgeError::GlobalStream("precomputed M1 payload bytes"))?,
        )
        .map_err(|_| M2BridgeError::GlobalStream("precomputed payload u64"))?;
        let aggregates = u64::try_from(projection.response_aggregates()?.len())
            .map_err(|_| M2BridgeError::GlobalStream("precomputed aggregate count"))?
            .checked_mul(32)
            .ok_or(M2BridgeError::GlobalStream("precomputed aggregate bytes"))?;
        let lengths = GlobalTaskPayloadLengthsV1 {
            raw,
            payload,
            aggregates,
        };
        global.raw = global
            .raw
            .checked_add(GLOBAL_RECORD_HEADER_BYTES_V1 + raw)
            .ok_or(M2BridgeError::GlobalStream("global raw payload length"))?;
        global.payload = global
            .payload
            .checked_add(GLOBAL_RECORD_HEADER_BYTES_V1 + payload)
            .ok_or(M2BridgeError::GlobalStream("global compact payload length"))?;
        global.aggregates = global
            .aggregates
            .checked_add(GLOBAL_RECORD_HEADER_BYTES_V1 + aggregates)
            .ok_or(M2BridgeError::GlobalStream(
                "global aggregate payload length",
            ))?;
        tasks.push(lengths);
    }
    Ok((global, tasks.into_boxed_slice()))
}

fn cached_global_payload_lengths(
    carrier: &M2OpeningParityCarrierV1,
) -> Result<(GlobalPayloadLengthsV1, Box<[GlobalTaskPayloadLengthsV1]>), M2BridgeError> {
    GLOBAL_PAYLOAD_LENGTHS_V1
        .get_or_init(|| precompute_global_payload_lengths(carrier))
        .clone()
}

/// Returns the complete canonical scalar facts needed by the official context
/// records and their three global parity streams.
///
/// The expensive projection/render/hash pass is performed at most once per
/// process.  Construction streams one task at a time into the existing fixed-
/// extent purpose-12/13/14 hashers; it never builds a global concatenation or a
/// full protected-tail arena.
pub(crate) fn canonical_opening_receipt_facts_v1(
) -> Result<M2CanonicalOpeningReceiptFactsV1, M2BridgeError> {
    CANONICAL_OPENING_RECEIPT_FACTS_V1
        .get_or_init(compute_canonical_opening_receipt_facts_v1)
        .clone()
}

fn compute_canonical_opening_receipt_facts_v1(
) -> Result<M2CanonicalOpeningReceiptFactsV1, M2BridgeError> {
    let carrier = M2OpeningParityCarrierV1::canonical()?;
    let (global_lengths, task_lengths) = cached_global_payload_lengths(&carrier)?;
    if task_lengths.len() != M2_CONTEXT_TASK_COUNT_V1 {
        return Err(M2BridgeError::GlobalStream(
            "canonical receipt task length count",
        ));
    }
    let record_count = u64::try_from(M2_CONTEXT_TASK_COUNT_V1)
        .map_err(|_| M2BridgeError::GlobalStream("canonical receipt record count"))?;
    let mut global_raw =
        FixedStreamHasherV1::new(STREAM_GLOBAL_SLOTS_V1, record_count, global_lengths.raw);
    let mut global_payload = FixedStreamHasherV1::new(
        STREAM_GLOBAL_PAYLOAD_V1,
        record_count,
        global_lengths.payload,
    );
    let mut global_aggregates = FixedStreamHasherV1::new(
        STREAM_GLOBAL_AGGREGATES_V1,
        record_count,
        global_lengths.aggregates,
    );
    let poison_record = slots_to_le_bytes(&[[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]]);
    let mut tail_digest_cache: Vec<(u32, [u8; 32])> = Vec::new();
    let mut contexts = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1);

    for (task_index, task) in carrier.tasks().iter().enumerate() {
        let expected_ordinal = u32::try_from(task_index)
            .map_err(|_| M2BridgeError::GlobalStream("canonical receipt task ordinal"))?;
        if task.ordinal != expected_ordinal {
            return Err(M2BridgeError::GlobalStream("canonical receipt task order"));
        }

        let projection = project_closed_form(task.context)?;
        let slots = render_expected_slots(task, &projection)?;
        let raw_bytes = slots_to_le_bytes(&slots);
        let payload_bytes = projection.canonical_projector_payload_bytes_v1()?;
        let scalar_aggregates: Vec<CompactedAggregateV1> = projection
            .response_aggregates()?
            .into_iter()
            .map(|aggregate| CompactedAggregateV1 {
                response: aggregate.response().map(|tile| tile.index() as u32),
                support: aggregate.support(),
                mass: aggregate.scaled_mass().value(),
            })
            .collect();
        let aggregate_bytes = encode_aggregate_records(&scalar_aggregates);

        let actual_lengths = GlobalTaskPayloadLengthsV1 {
            raw: u64::try_from(raw_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("canonical raw task length"))?,
            payload: u64::try_from(payload_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("canonical payload task length"))?,
            aggregates: u64::try_from(aggregate_bytes.len())
                .map_err(|_| M2BridgeError::GlobalStream("canonical aggregate task length"))?,
        };
        let expected_lengths = *task_lengths
            .get(task_index)
            .ok_or(M2BridgeError::GlobalStream(
                "canonical receipt task length index",
            ))?;
        if actual_lengths != expected_lengths {
            return Err(M2BridgeError::GlobalStream(
                "canonical receipt per-task extent",
            ));
        }

        global_raw.validate_record(actual_lengths.raw)?;
        global_payload.validate_record(actual_lengths.payload)?;
        global_aggregates.validate_record(actual_lengths.aggregates)?;
        global_raw.append_validated_record(task.ordinal, actual_lengths.raw, &raw_bytes);
        global_payload.append_validated_record(
            task.ordinal,
            actual_lengths.payload,
            &payload_bytes,
        );
        global_aggregates.append_validated_record(
            task.ordinal,
            actual_lengths.aggregates,
            &aggregate_bytes,
        );

        let raw_record_count = u64::from(task.candidate_slot_count());
        let aggregate_record_count = u64::try_from(scalar_aggregates.len())
            .map_err(|_| M2BridgeError::Carrier("canonical aggregate record count"))?;
        let candidate_slot_count = task.candidate_slot_count();
        let tail_guard_sha256 = match tail_digest_cache
            .iter()
            .find(|(candidate_count, _)| *candidate_count == candidate_slot_count)
        {
            Some((_, digest)) => *digest,
            None => {
                let candidate_count = usize::try_from(candidate_slot_count)
                    .map_err(|_| M2BridgeError::Carrier("canonical candidate slot usize"))?;
                let protected_count = OPENING_ARENA_SLOT_COUNT_V1
                    .checked_sub(candidate_count)
                    .ok_or(M2BridgeError::Carrier("canonical protected record count"))?;
                let digest = repeated_record_stream_digest(
                    STREAM_PROTECTED_RECORDS_V1,
                    u64::try_from(protected_count).map_err(|_| {
                        M2BridgeError::Carrier("canonical protected record count u64")
                    })?,
                    &poison_record,
                )?;
                tail_digest_cache.push((candidate_slot_count, digest));
                digest
            }
        };
        let accepted_cells = u32::try_from(projection.cells().len())
            .map_err(|_| M2BridgeError::Carrier("canonical accepted cell count"))?;
        let total_scaled_mass = U256Mass::from_u64(projection.total_scaled_mass()?.value());
        contexts.push(M2CanonicalContextReceiptFactsV1 {
            task_ordinal: task.ordinal,
            accepted_cells,
            total_scaled_mass,
            canonical_payload_bytes: actual_lengths.payload,
            raw_sha256: stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_record_count, &raw_bytes)?,
            payload_sha256: stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &payload_bytes)?,
            aggregate_sha256: stream_digest(
                STREAM_CONTEXT_AGGREGATES_V1,
                aggregate_record_count,
                &aggregate_bytes,
            )?,
            tail_guard_sha256,
        });
    }

    Ok(M2CanonicalOpeningReceiptFactsV1 {
        contexts: contexts.into_boxed_slice(),
        global: M2CanonicalGlobalReceiptFactsV1 {
            raw_sha256: global_raw.finish()?,
            payload_sha256: global_payload.finish()?,
            aggregate_sha256: global_aggregates.finish()?,
        },
    })
}

/// Returns exact purpose-2/3/9 digests and status censuses for both canonical
/// arithmetic runs.  Like the opening facts, the result is computed once and
/// thereafter cloned from a small cache.
pub(crate) fn canonical_arithmetic_receipt_facts_v1(
) -> Result<M2CanonicalArithmeticReceiptFactsV1, M2BridgeError> {
    CANONICAL_ARITHMETIC_RECEIPT_FACTS_V1
        .get_or_init(compute_canonical_arithmetic_receipt_facts_v1)
        .clone()
}

fn compute_canonical_arithmetic_receipt_facts_v1(
) -> Result<M2CanonicalArithmeticReceiptFactsV1, M2BridgeError> {
    let corpus = M2ArithmeticCorpusV1::canonical()?;
    let official =
        canonical_arithmetic_run_receipt_facts_v1(corpus.inputs(), corpus.expected_outputs())?;

    let controls = ArithmeticNegativeControlsV1::canonical()?;
    let negative_inputs: Vec<[u32; ARITHMETIC_INPUT_WORDS_V1]> = controls
        .controls()
        .iter()
        .map(|control| *control.input())
        .collect();
    let negative_outputs: Vec<[u32; ARITHMETIC_OUTPUT_WORDS_V1]> = controls
        .controls()
        .iter()
        .map(|control| *control.expected_output())
        .collect();
    let negative = canonical_arithmetic_run_receipt_facts_v1(&negative_inputs, &negative_outputs)?;

    Ok(M2CanonicalArithmeticReceiptFactsV1 { official, negative })
}

fn canonical_arithmetic_run_receipt_facts_v1(
    inputs: &[[u32; ARITHMETIC_INPUT_WORDS_V1]],
    outputs: &[[u32; ARITHMETIC_OUTPUT_WORDS_V1]],
) -> Result<M2CanonicalArithmeticRunReceiptFactsV1, M2BridgeError> {
    if inputs.len() != outputs.len() {
        return Err(M2BridgeError::Arithmetic(
            "canonical receipt input/output count",
        ));
    }
    let case_count = u32::try_from(inputs.len())
        .map_err(|_| M2BridgeError::Arithmetic("canonical receipt case count"))?;
    let input_bytes = fixed_word_records_to_le_bytes(inputs)?;
    let output_bytes = fixed_word_records_to_le_bytes(outputs)?;
    let mut success_count = 0u32;
    let mut checked_undefined_count = 0u32;
    let mut hard_count = 0u32;
    for output in outputs {
        let counter = match output[0] {
            ARITHMETIC_SUCCESS_V1 => &mut success_count,
            ARITHMETIC_CHECKED_UNDEFINED_V1 => &mut checked_undefined_count,
            status if status & 0x8000_0000 != 0 => &mut hard_count,
            _ => return Err(M2BridgeError::Arithmetic("canonical receipt output status")),
        };
        *counter = counter
            .checked_add(1)
            .ok_or(M2BridgeError::Arithmetic("canonical receipt status census"))?;
    }
    let classified_count = success_count
        .checked_add(checked_undefined_count)
        .and_then(|value| value.checked_add(hard_count))
        .ok_or(M2BridgeError::Arithmetic(
            "canonical receipt classified count",
        ))?;
    if classified_count != case_count {
        return Err(M2BridgeError::Arithmetic(
            "canonical receipt complete census",
        ));
    }
    let accepted_count =
        success_count
            .checked_add(checked_undefined_count)
            .ok_or(M2BridgeError::Arithmetic(
                "canonical receipt accepted count",
            ))?;
    let poison_record = slots_to_le_bytes(&[[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]]);
    Ok(M2CanonicalArithmeticRunReceiptFactsV1 {
        case_count,
        accepted_count,
        input_payload_bytes: u64::try_from(input_bytes.len())
            .map_err(|_| M2BridgeError::Arithmetic("canonical input payload bytes"))?,
        output_payload_bytes: u64::try_from(output_bytes.len())
            .map_err(|_| M2BridgeError::Arithmetic("canonical output payload bytes"))?,
        success_count,
        checked_undefined_count,
        hard_count,
        input_sha256: stream_digest(
            STREAM_ARITHMETIC_INPUT_V1,
            u64::from(case_count),
            &input_bytes,
        )?,
        output_sha256: stream_digest(
            STREAM_ARITHMETIC_OUTPUT_V1,
            u64::from(case_count),
            &output_bytes,
        )?,
        guard_sha256: repeated_record_stream_digest(
            STREAM_PROTECTED_RECORDS_V1,
            2,
            &poison_record,
        )?,
    })
}

fn fixed_word_records_to_le_bytes<const WORDS: usize>(
    records: &[[u32; WORDS]],
) -> Result<Vec<u8>, M2BridgeError> {
    let capacity = records
        .len()
        .checked_mul(WORDS)
        .and_then(|words| words.checked_mul(core::mem::size_of::<u32>()))
        .ok_or(M2BridgeError::Arithmetic(
            "canonical receipt record byte length",
        ))?;
    let mut bytes = Vec::with_capacity(capacity);
    for record in records {
        for word in record {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
    }
    Ok(bytes)
}

fn repeated_record_stream_digest(
    purpose: u32,
    record_count: u64,
    record: &[u8],
) -> Result<[u8; 32], M2BridgeError> {
    let record_bytes = u64::try_from(record.len())
        .map_err(|_| M2BridgeError::Carrier("protected record byte length"))?;
    let payload_bytes = record_count
        .checked_mul(record_bytes)
        .ok_or(M2BridgeError::Carrier("protected stream byte length"))?;
    let mut state = Sha256State::new();
    state.update(STREAM_MAGIC_V1);
    state.update(&purpose.to_le_bytes());
    state.update(&STREAM_VERSION_V1.to_le_bytes());
    state.update(&record_count.to_le_bytes());
    state.update(&payload_bytes.to_le_bytes());
    for _ in 0..record_count {
        state.update(record);
    }
    Ok(state.finish())
}

#[derive(Debug, PartialEq, Eq)]
pub struct CheckedM2ProjectionPayloadV1 {
    task_ordinal: u32,
    task_key_sha256: [u8; 32],
    context: OpeningContext,
    response_count: u32,
    candidate_slot_count: u32,
    cell_count: u32,
    total_mass: u64,
    cpu_raw_sha256: [u8; 32],
    gpu_raw_sha256: [u8; 32],
    cpu_payload_sha256: [u8; 32],
    gpu_payload_sha256: [u8; 32],
    payload_length: u64,
    cpu_aggregate_sha256: [u8; 32],
    gpu_aggregate_sha256: [u8; 32],
    canonical_payload: Box<[u8]>,
}

impl CheckedM2ProjectionPayloadV1 {
    pub const fn task_ordinal(&self) -> u32 {
        self.task_ordinal
    }

    pub const fn context(&self) -> OpeningContext {
        self.context
    }

    pub const fn response_count(&self) -> u32 {
        self.response_count
    }

    pub const fn candidate_slot_count(&self) -> u32 {
        self.candidate_slot_count
    }

    pub const fn cell_count(&self) -> u32 {
        self.cell_count
    }

    pub const fn total_mass(&self) -> u64 {
        self.total_mass
    }

    pub const fn cpu_raw_sha256(&self) -> [u8; 32] {
        self.cpu_raw_sha256
    }

    pub const fn gpu_raw_sha256(&self) -> [u8; 32] {
        self.gpu_raw_sha256
    }

    pub const fn cpu_payload_sha256(&self) -> [u8; 32] {
        self.cpu_payload_sha256
    }

    pub const fn gpu_payload_sha256(&self) -> [u8; 32] {
        self.gpu_payload_sha256
    }

    pub const fn payload_length(&self) -> u64 {
        self.payload_length
    }

    pub const fn cpu_aggregate_sha256(&self) -> [u8; 32] {
        self.cpu_aggregate_sha256
    }

    pub const fn gpu_aggregate_sha256(&self) -> [u8; 32] {
        self.gpu_aggregate_sha256
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct M2BoundOpeningTaskV1 {
    checked: CheckedM2ProjectionPayloadV1,
    reduced: Box<[ReducedEvidenceBindingV1]>,
    physical: Box<[PhysicalActionBindingV1]>,
}

impl M2BoundOpeningTaskV1 {
    pub fn checked_payload(&self) -> &CheckedM2ProjectionPayloadV1 {
        &self.checked
    }

    pub fn reduced_bindings(&self) -> &[ReducedEvidenceBindingV1] {
        &self.reduced
    }

    pub fn physical_bindings(&self) -> &[PhysicalActionBindingV1] {
        &self.physical
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReducedEvidenceBindingV1 {
    profile: ReducedCarrierProfileV1,
    binding_ordinal: u32,
    task_ordinal: u32,
    arm_ordinal: u32,
    root_selector: u32,
    coordinate_ordinal: u32,
    root: OpeningRootV1,
    selected_action: Domino,
    context: OpeningContext,
    matching_count: u8,
    generator: [u32; 3],
    payload_length: u64,
    payload_sha256: [u8; 32],
}

impl ReducedEvidenceBindingV1 {
    pub const fn profile(&self) -> ReducedCarrierProfileV1 {
        self.profile
    }
    pub const fn binding_ordinal(&self) -> u32 {
        self.binding_ordinal
    }
    pub const fn task_ordinal(&self) -> u32 {
        self.task_ordinal
    }
    pub const fn arm_ordinal(&self) -> u32 {
        self.arm_ordinal
    }
    pub const fn root_selector(&self) -> u32 {
        self.root_selector
    }
    pub const fn coordinate_ordinal(&self) -> u32 {
        self.coordinate_ordinal
    }
    pub const fn root(&self) -> OpeningRootV1 {
        self.root
    }
    pub const fn selected_action(&self) -> Domino {
        self.selected_action
    }
    pub const fn context(&self) -> OpeningContext {
        self.context
    }
    pub const fn matching_count(&self) -> u8 {
        self.matching_count
    }
    pub const fn generator(&self) -> [u32; 3] {
        self.generator
    }
    pub const fn payload_length(&self) -> u64 {
        self.payload_length
    }
    pub const fn payload_sha256(&self) -> [u8; 32] {
        self.payload_sha256
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalActionBindingV1 {
    binding_ordinal: u32,
    task_ordinal: u32,
    arm: M2CarrierArmV1,
    arm_ordinal: u32,
    endpoint: u32,
    root: OpeningRootV1,
    selected_action: Domino,
    context: OpeningContext,
    payload_length: u64,
    payload_sha256: [u8; 32],
}

impl PhysicalActionBindingV1 {
    pub const fn binding_ordinal(&self) -> u32 {
        self.binding_ordinal
    }
    pub const fn task_ordinal(&self) -> u32 {
        self.task_ordinal
    }
    pub const fn arm(&self) -> M2CarrierArmV1 {
        self.arm
    }
    pub const fn arm_ordinal(&self) -> u32 {
        self.arm_ordinal
    }
    pub const fn endpoint(&self) -> u32 {
        self.endpoint
    }
    pub const fn root(&self) -> OpeningRootV1 {
        self.root
    }
    pub const fn selected_action(&self) -> Domino {
        self.selected_action
    }
    pub const fn context(&self) -> OpeningContext {
        self.context
    }
    pub const fn payload_length(&self) -> u64 {
        self.payload_length
    }
    pub const fn payload_sha256(&self) -> [u8; 32] {
        self.payload_sha256
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M2ArithmeticCorpusV1 {
    inputs: Box<[[u32; ARITHMETIC_INPUT_WORDS_V1]]>,
    expected_outputs: Box<[[u32; ARITHMETIC_OUTPUT_WORDS_V1]]>,
}

impl M2ArithmeticCorpusV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        let edges = arithmetic_edges()?;
        if edges.len() != 32 {
            return Err(M2BridgeError::Arithmetic("edge count"));
        }
        let mut inputs = Vec::with_capacity(M2_ARITHMETIC_CASE_COUNT_V1);

        for operation in [
            ARITHMETIC_OP_CHECKED_ADD_V1,
            ARITHMETIC_OP_CHECKED_SUB_V1,
            ARITHMETIC_OP_COMPARE_V1,
        ] {
            for lhs in &edges {
                for rhs in &edges {
                    push_arithmetic_input(&mut inputs, operation, 0, *lhs, *rhs)?;
                }
            }
        }

        const SMALL_FACTORS: [u32; 16] = [
            0,
            1,
            2,
            3,
            6,
            7,
            10,
            42,
            60,
            84,
            105,
            140,
            210,
            419,
            420,
            u32::MAX,
        ];
        for lhs in &edges {
            for factor in SMALL_FACTORS {
                push_arithmetic_input(
                    &mut inputs,
                    ARITHMETIC_OP_CHECKED_MUL_SMALL_V1,
                    factor,
                    *lhs,
                    [0; 8],
                )?;
            }
        }
        for lhs in &edges {
            for exponent in 0..=21 {
                push_arithmetic_input(
                    &mut inputs,
                    ARITHMETIC_OP_CHECKED_MUL_POW_420_V1,
                    exponent,
                    *lhs,
                    [0; 8],
                )?;
            }
        }
        if inputs.len() != 4_288 {
            return Err(M2BridgeError::Arithmetic("edge prefix count"));
        }

        let mut splitmix = SplitMix64V1::new(0x4d32_5f55_3235_3656);
        for tail_ordinal in 0..12_096usize {
            let lhs = splitmix_limbs(&mut splitmix);
            let provisional_rhs = splitmix_limbs(&mut splitmix);
            let selector = splitmix.next();
            let selector_low = selector as u32;
            let operation = match tail_ordinal % 5 {
                0 => ARITHMETIC_OP_CHECKED_ADD_V1,
                1 => ARITHMETIC_OP_CHECKED_SUB_V1,
                2 => ARITHMETIC_OP_CHECKED_MUL_SMALL_V1,
                3 => ARITHMETIC_OP_CHECKED_MUL_POW_420_V1,
                4 => ARITHMETIC_OP_COMPARE_V1,
                _ => unreachable!(),
            };
            let (operand, rhs) = match operation {
                ARITHMETIC_OP_CHECKED_MUL_SMALL_V1 => (selector_low, [0; 8]),
                ARITHMETIC_OP_CHECKED_MUL_POW_420_V1 => (selector_low % 22, [0; 8]),
                _ => (0, provisional_rhs),
            };
            push_arithmetic_input(&mut inputs, operation, operand, lhs, rhs)?;
        }
        if inputs.len() != M2_ARITHMETIC_CASE_COUNT_V1 {
            return Err(M2BridgeError::Arithmetic("complete corpus count"));
        }

        let expected_outputs = inputs
            .iter()
            .map(arithmetic_oracle_output)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            inputs: inputs.into_boxed_slice(),
            expected_outputs: expected_outputs.into_boxed_slice(),
        })
    }

    pub fn inputs(&self) -> &[[u32; ARITHMETIC_INPUT_WORDS_V1]] {
        &self.inputs
    }

    pub fn expected_outputs(&self) -> &[[u32; ARITHMETIC_OUTPUT_WORDS_V1]] {
        &self.expected_outputs
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticNegativeControlV1 {
    ordinal: u32,
    input: [u32; ARITHMETIC_INPUT_WORDS_V1],
    expected_output: [u32; ARITHMETIC_OUTPUT_WORDS_V1],
}

impl ArithmeticNegativeControlV1 {
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub const fn input(&self) -> &[u32; ARITHMETIC_INPUT_WORDS_V1] {
        &self.input
    }
    pub const fn expected_output(&self) -> &[u32; ARITHMETIC_OUTPUT_WORDS_V1] {
        &self.expected_output
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticNegativeControlsV1 {
    controls: [ArithmeticNegativeControlV1; ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1],
}

impl ArithmeticNegativeControlsV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        let mut inputs = Vec::with_capacity(ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1);
        for ordinal in 0..ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1 {
            let case_id = u32::try_from(ordinal)
                .map_err(|_| M2BridgeError::Arithmetic("negative case id"))?;
            let mut words = [0u32; ARITHMETIC_INPUT_WORDS_V1];
            words[0] = ARITHMETIC_ABI_VERSION_V1;
            words[1] = case_id;
            words[2] = ARITHMETIC_OP_CHECKED_ADD_V1;
            match ordinal {
                0 => words[0] = 0,
                1 => words[0] = 2,
                2 => words[2] = 0,
                3 => words[2] = 6,
                4 => words[3] = 1,
                5 => {
                    words[2] = ARITHMETIC_OP_CHECKED_SUB_V1;
                    words[3] = 1;
                }
                6 => {
                    words[2] = ARITHMETIC_OP_COMPARE_V1;
                    words[3] = 1;
                }
                7 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_SMALL_V1;
                    words[12] = 1;
                }
                8 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_SMALL_V1;
                    words[19] = 1;
                }
                9 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_POW_420_V1;
                    words[12] = 1;
                }
                10 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_POW_420_V1;
                    words[19] = 1;
                }
                11 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_POW_420_V1;
                    words[3] = 22;
                }
                12 => {
                    words[2] = ARITHMETIC_OP_CHECKED_MUL_POW_420_V1;
                    words[3] = u32::MAX;
                }
                _ => unreachable!(),
            }
            inputs.push(words);
        }
        let mut controls = Vec::with_capacity(ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1);
        for input in inputs {
            let ordinal = input[1];
            let status = malformed_arithmetic_status(&input)
                .ok_or(M2BridgeError::Arithmetic("negative control remained valid"))?;
            controls.push(ArithmeticNegativeControlV1 {
                ordinal,
                input,
                expected_output: arithmetic_hard_output(status, ordinal, input[2]),
            });
        }
        let controls: [ArithmeticNegativeControlV1; ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1] =
            controls
                .try_into()
                .map_err(|_| M2BridgeError::Arithmetic("negative control count"))?;
        Ok(Self { controls })
    }

    pub const fn controls(
        &self,
    ) -> &[ArithmeticNegativeControlV1; ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1] {
        &self.controls
    }
}

fn push_arithmetic_input(
    inputs: &mut Vec<[u32; ARITHMETIC_INPUT_WORDS_V1]>,
    operation: u32,
    operand: u32,
    lhs: [u32; 8],
    rhs: [u32; 8],
) -> Result<(), M2BridgeError> {
    let case_id =
        u32::try_from(inputs.len()).map_err(|_| M2BridgeError::Arithmetic("arithmetic case id"))?;
    let mut words = [0u32; ARITHMETIC_INPUT_WORDS_V1];
    words[0] = ARITHMETIC_ABI_VERSION_V1;
    words[1] = case_id;
    words[2] = operation;
    words[3] = operand;
    words[4..12].copy_from_slice(&lhs);
    words[12..20].copy_from_slice(&rhs);
    inputs.push(words);
    Ok(())
}

fn arithmetic_edges() -> Result<Vec<[u32; 8]>, M2BridgeError> {
    let mut values = Vec::with_capacity(32);
    for value in [BigUint::from(0u8), BigUint::from(1u8), BigUint::from(2u8)] {
        values.push(big_to_limbs(&value)?);
    }
    for exponent in [32u32, 64, 96, 128, 160, 192, 224] {
        values.push(big_to_limbs(
            &((BigUint::from(1u8) << exponent) - BigUint::from(1u8)),
        )?);
        values.push(big_to_limbs(&(BigUint::from(1u8) << exponent))?);
    }
    let max = (BigUint::from(1u8) << 256u32) - BigUint::from(1u8);
    values.push(big_to_limbs(&max)?);
    values.push(big_to_limbs(&(max.clone() - BigUint::from(1u8)))?);
    values.push([0xaaaa_aaaa; 8]);
    values.push([0x5555_5555; 8]);
    values.push([0, 1, 2, 3, 4, 5, 6, 7]);
    values.push([7, 6, 5, 4, 3, 2, 1, 0]);
    let n0 = BigUint::from(399_072_960u64);
    let scale = BigUint::from(420u32).pow(21);
    values.push(big_to_limbs(&n0)?);
    values.push(big_to_limbs(&BigUint::from(420u32))?);
    values.push(big_to_limbs(&scale)?);
    values.push(big_to_limbs(&(n0.clone() * scale.clone()))?);
    values.push(big_to_limbs(&(BigUint::from(42u8) * n0 * scale))?);
    values.push(big_to_limbs(&(BigUint::from(1u8) << 255u32))?);
    values.push(big_to_limbs(&(BigUint::from(1u8) << 254u32))?);
    values.push([
        0xffff_ffff,
        0,
        0xffff_ffff,
        0,
        0xffff_ffff,
        0,
        0xffff_ffff,
        0,
    ]);
    values.push([
        0,
        0xffff_ffff,
        0,
        0xffff_ffff,
        0,
        0xffff_ffff,
        0,
        0xffff_ffff,
    ]);
    Ok(values)
}

struct SplitMix64V1 {
    state: u64,
}

impl SplitMix64V1 {
    const fn new(state: u64) -> Self {
        Self { state }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

fn splitmix_limbs(generator: &mut SplitMix64V1) -> [u32; 8] {
    let mut limbs = [0u32; 8];
    for pair in 0..4 {
        let value = generator.next();
        limbs[pair * 2] = value as u32;
        limbs[pair * 2 + 1] = (value >> 32) as u32;
    }
    limbs
}

fn arithmetic_oracle_output(
    input: &[u32; ARITHMETIC_INPUT_WORDS_V1],
) -> Result<[u32; ARITHMETIC_OUTPUT_WORDS_V1], M2BridgeError> {
    if let Some(status) = malformed_arithmetic_status(input) {
        return Ok(arithmetic_hard_output(status, input[1], input[2]));
    }
    let lhs = limbs_to_big(
        input[4..12]
            .try_into()
            .map_err(|_| M2BridgeError::Arithmetic("lhs slice"))?,
    );
    let rhs = limbs_to_big(
        input[12..20]
            .try_into()
            .map_err(|_| M2BridgeError::Arithmetic("rhs slice"))?,
    );
    let limit = BigUint::from(1u8) << 256u32;
    match input[2] {
        ARITHMETIC_OP_CHECKED_ADD_V1 => {
            let result = lhs + rhs;
            if result >= limit {
                Ok(arithmetic_undefined_output(input[1], input[2]))
            } else {
                arithmetic_success_output(input[1], input[2], big_to_limbs(&result)?)
            }
        }
        ARITHMETIC_OP_CHECKED_SUB_V1 => {
            if lhs < rhs {
                Ok(arithmetic_undefined_output(input[1], input[2]))
            } else {
                arithmetic_success_output(input[1], input[2], big_to_limbs(&(lhs - rhs))?)
            }
        }
        ARITHMETIC_OP_CHECKED_MUL_SMALL_V1 => {
            let result = lhs * BigUint::from(input[3]);
            if result >= limit {
                Ok(arithmetic_undefined_output(input[1], input[2]))
            } else {
                arithmetic_success_output(input[1], input[2], big_to_limbs(&result)?)
            }
        }
        ARITHMETIC_OP_CHECKED_MUL_POW_420_V1 => {
            let result = lhs * BigUint::from(420u32).pow(input[3]);
            if result >= limit {
                Ok(arithmetic_undefined_output(input[1], input[2]))
            } else {
                arithmetic_success_output(input[1], input[2], big_to_limbs(&result)?)
            }
        }
        ARITHMETIC_OP_COMPARE_V1 => {
            let ordering = match lhs.cmp(&rhs) {
                Ordering::Less => 1,
                Ordering::Equal => 2,
                Ordering::Greater => 3,
            };
            let mut output = [0u32; ARITHMETIC_OUTPUT_WORDS_V1];
            output[0] = ARITHMETIC_SUCCESS_V1;
            output[1] = input[1];
            output[2] = input[2];
            output[3] = 1;
            output[4] = ordering;
            Ok(output)
        }
        _ => Err(M2BridgeError::Arithmetic("oracle operation")),
    }
}

fn malformed_arithmetic_status(input: &[u32; ARITHMETIC_INPUT_WORDS_V1]) -> Option<u32> {
    if input[0] != ARITHMETIC_ABI_VERSION_V1 {
        return Some(ARITHMETIC_HARD_BAD_ABI_V1);
    }
    if !(ARITHMETIC_OP_CHECKED_ADD_V1..=ARITHMETIC_OP_COMPARE_V1).contains(&input[2]) {
        return Some(ARITHMETIC_HARD_BAD_OPERATION_V1);
    }
    match input[2] {
        ARITHMETIC_OP_CHECKED_ADD_V1 | ARITHMETIC_OP_CHECKED_SUB_V1 | ARITHMETIC_OP_COMPARE_V1
            if input[3] != 0 =>
        {
            Some(ARITHMETIC_HARD_BAD_OPERAND_V1)
        }
        ARITHMETIC_OP_CHECKED_MUL_SMALL_V1 | ARITHMETIC_OP_CHECKED_MUL_POW_420_V1
            if input[12..20].iter().any(|word| *word != 0) =>
        {
            Some(ARITHMETIC_HARD_BAD_UNUSED_RHS_V1)
        }
        ARITHMETIC_OP_CHECKED_MUL_POW_420_V1 if input[3] > 21 => {
            Some(ARITHMETIC_HARD_BAD_EXPONENT_V1)
        }
        _ => None,
    }
}

fn arithmetic_success_output(
    case_id: u32,
    operation: u32,
    result: [u32; 8],
) -> Result<[u32; ARITHMETIC_OUTPUT_WORDS_V1], M2BridgeError> {
    let mut output = [0u32; ARITHMETIC_OUTPUT_WORDS_V1];
    output[0] = ARITHMETIC_SUCCESS_V1;
    output[1] = case_id;
    output[2] = operation;
    output[3] = 1;
    output[5..13].copy_from_slice(&result);
    Ok(output)
}

fn arithmetic_undefined_output(case_id: u32, operation: u32) -> [u32; ARITHMETIC_OUTPUT_WORDS_V1] {
    let mut output = [0u32; ARITHMETIC_OUTPUT_WORDS_V1];
    output[0] = ARITHMETIC_CHECKED_UNDEFINED_V1;
    output[1] = case_id;
    output[2] = operation;
    output
}

fn arithmetic_hard_output(
    status: u32,
    case_id: u32,
    operation: u32,
) -> [u32; ARITHMETIC_OUTPUT_WORDS_V1] {
    let mut output = [0u32; ARITHMETIC_OUTPUT_WORDS_V1];
    output[0] = status;
    output[1] = case_id;
    output[2] = operation;
    output
}

fn limbs_to_big(limbs: [u32; 8]) -> BigUint {
    let mut bytes = [0u8; 32];
    for (index, limb) in limbs.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&limb.to_le_bytes());
    }
    BigUint::from_bytes_le(&bytes)
}

fn big_to_limbs(value: &BigUint) -> Result<[u32; 8], M2BridgeError> {
    if value.bits() > 256 {
        return Err(M2BridgeError::Arithmetic("U256 oracle width"));
    }
    let bytes = value.to_bytes_le();
    let mut fixed = [0u8; 32];
    fixed[..bytes.len()].copy_from_slice(&bytes);
    Ok(U256Mass::from_le_bytes(fixed).limbs_le())
}

fn big_to_u32(value: &BigUint) -> Option<u32> {
    let digits = value.to_u32_digits();
    match digits.as_slice() {
        [] => Some(0),
        [value] => Some(*value),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum BindingTemplateV1 {
    Reduced {
        binding_ordinal: u32,
        profile: ReducedCarrierProfileV1,
        root_selector: u32,
        coordinate_ordinal: u32,
        root: OpeningRootV1,
        selected_action: Domino,
        matching_count: u8,
    },
    Physical {
        binding_ordinal: u32,
        endpoint: u32,
        root: OpeningRootV1,
        selected_action: Domino,
    },
}

fn generate_carrier() -> Result<M2OpeningParityCarrierV1, M2BridgeError> {
    let mut tasks = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1);
    generate_reduced_arm(&mut tasks)?;
    generate_grade_matching_arm(&mut tasks)?;
    generate_same_context_arm(&mut tasks)?;

    if tasks.len() != M2_CONTEXT_TASK_COUNT_V1 {
        return Err(M2BridgeError::Carrier("task count"));
    }
    let reduced_count: usize = tasks
        .iter()
        .map(M2OpeningTaskV1::reduced_binding_count)
        .sum();
    let physical_count: usize = tasks
        .iter()
        .map(M2OpeningTaskV1::physical_binding_count)
        .sum();
    let parity_count = tasks
        .iter()
        .filter(|task| matches!(task.direct, DirectPreflightV1::Admitted { .. }))
        .count();
    let stop_count = tasks
        .iter()
        .filter(|task| matches!(task.direct, DirectPreflightV1::DeclaredStop { .. }))
        .count();
    if reduced_count != M2_REDUCED_BINDING_COUNT_V1 {
        return Err(M2BridgeError::Carrier("reduced binding count"));
    }
    if physical_count != M2_PHYSICAL_BINDING_COUNT_V1 {
        return Err(M2BridgeError::Carrier("physical binding count"));
    }
    if parity_count != M2_DIRECT_PARITY_COUNT_V1 || stop_count != M2_DIRECT_STOP_COUNT_V1 {
        return Err(M2BridgeError::Carrier("direct outcome census"));
    }
    for (ordinal, task) in tasks.iter().enumerate() {
        if task.ordinal != u32::try_from(ordinal).map_err(|_| M2BridgeError::Carrier("ordinal"))? {
            return Err(M2BridgeError::Carrier("global task order"));
        }
    }

    let mut key_bytes = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1 * 64);
    for task in &tasks {
        key_bytes.extend_from_slice(&task.task_key.to_le_bytes());
    }
    let task_key_stream_sha256 = stream_digest(
        STREAM_TASK_KEYS_V1,
        u64::try_from(tasks.len()).map_err(|_| M2BridgeError::Carrier("task key count"))?,
        &key_bytes,
    )?;
    Ok(M2OpeningParityCarrierV1 {
        tasks: tasks.into_boxed_slice(),
        task_key_stream_sha256,
    })
}

fn generate_reduced_arm(tasks: &mut Vec<M2OpeningTaskV1>) -> Result<(), M2BridgeError> {
    let roots = canonical_reduced_roots()?;
    let mut arm_ordinal = 0u32;
    for (root_selector, root) in roots.into_iter().enumerate() {
        let carrier = ReducedOpeningCarrierV1::from_root(root)?;
        for (coordinate_ordinal, coordinate) in carrier.coordinates().iter().copied().enumerate() {
            let context = coordinate.opening_context()?;
            let selected_action = least_context_action(root, coordinate.led())?;
            let template = BindingTemplateV1::Reduced {
                binding_ordinal: u32::try_from(tasks.len())
                    .map_err(|_| M2BridgeError::Carrier("Reduced binding ordinal"))?,
                profile: ReducedCarrierProfileV1::ReducedArm,
                root_selector: u32::try_from(root_selector)
                    .map_err(|_| M2BridgeError::Carrier("root selector"))?,
                coordinate_ordinal: u32::try_from(coordinate_ordinal)
                    .map_err(|_| M2BridgeError::Carrier("coordinate ordinal"))?,
                root,
                selected_action,
                matching_count: coordinate.matching_count(),
            };
            let generator = [
                u32::try_from(root_selector)
                    .map_err(|_| M2BridgeError::Carrier("root selector"))?,
                u32::try_from(coordinate_ordinal)
                    .map_err(|_| M2BridgeError::Carrier("coordinate ordinal"))?,
                0,
            ];
            tasks.push(make_task(
                tasks.len(),
                M2CarrierArmV1::Reduced,
                arm_ordinal,
                context,
                generator,
                vec![template],
            )?);
            arm_ordinal = arm_ordinal
                .checked_add(1)
                .ok_or(M2BridgeError::Carrier("Reduced arm ordinal"))?;
        }
    }
    if arm_ordinal != 64 {
        return Err(M2BridgeError::Carrier("Reduced arm count"));
    }
    Ok(())
}

fn generate_grade_matching_arm(tasks: &mut Vec<M2OpeningTaskV1>) -> Result<(), M2BridgeError> {
    let mut arm_ordinal = 0u32;
    for grade in 1..=7u8 {
        for matching_count in 0..=6u8.min(grade * 3) {
            let context = grade_matching_context(grade, matching_count)?;
            let root = grade_matching_fixture_root(matching_count)?;
            let selected_action = least_context_action(root, natural_six())?;
            let template = if grade == 7 {
                BindingTemplateV1::Physical {
                    binding_ordinal: u32::from(matching_count),
                    endpoint: 0,
                    root,
                    selected_action,
                }
            } else {
                BindingTemplateV1::Reduced {
                    binding_ordinal: u32::try_from(tasks.len())
                        .map_err(|_| M2BridgeError::Carrier("GradeMatching binding ordinal"))?,
                    profile: ReducedCarrierProfileV1::GradeMatching,
                    root_selector: u32::from(matching_count),
                    coordinate_ordinal: arm_ordinal,
                    root,
                    selected_action,
                    matching_count,
                }
            };
            tasks.push(make_task(
                tasks.len(),
                M2CarrierArmV1::GradeMatching,
                arm_ordinal,
                context,
                [u32::from(grade), u32::from(matching_count), 0],
                vec![template],
            )?);
            arm_ordinal = arm_ordinal
                .checked_add(1)
                .ok_or(M2BridgeError::Carrier("GradeMatching arm ordinal"))?;
        }
    }
    if arm_ordinal != 46 {
        return Err(M2BridgeError::Carrier("GradeMatching arm count"));
    }
    Ok(())
}

fn generate_same_context_arm(tasks: &mut Vec<M2OpeningTaskV1>) -> Result<(), M2BridgeError> {
    let mut arm_ordinal = 0u32;
    for decl in Decl::ALL {
        let mut declaration_pairs = 0u32;
        for first_index in 0..Domino::COUNT {
            let first = Domino::ALL[first_index];
            for second_index in first_index + 1..Domino::COUNT {
                let second = Domino::ALL[second_index];
                let led = decl.led_context(first);
                if decl.led_context(second) != led {
                    continue;
                }
                let root = same_context_fixture_root(decl, first, second)?;
                let context = root.opening_context(led)?;
                let templates = vec![
                    BindingTemplateV1::Physical {
                        binding_ordinal: 7 + 2 * arm_ordinal,
                        endpoint: 0,
                        root,
                        selected_action: first,
                    },
                    BindingTemplateV1::Physical {
                        binding_ordinal: 7 + 2 * arm_ordinal + 1,
                        endpoint: 1,
                        root,
                        selected_action: second,
                    },
                ];
                tasks.push(make_task(
                    tasks.len(),
                    M2CarrierArmV1::SameContextPair,
                    arm_ordinal,
                    context,
                    [
                        decl_code_u32(decl),
                        first.index() as u32,
                        second.index() as u32,
                    ],
                    templates,
                )?);
                arm_ordinal = arm_ordinal
                    .checked_add(1)
                    .ok_or(M2BridgeError::Carrier("SameContext arm ordinal"))?;
                declaration_pairs = declaration_pairs
                    .checked_add(1)
                    .ok_or(M2BridgeError::Carrier("declaration pair count"))?;
            }
        }
        if declaration_pairs != 56 {
            return Err(M2BridgeError::Carrier("SameContext declaration pair count"));
        }
    }
    if arm_ordinal != 504 {
        return Err(M2BridgeError::Carrier("SameContext arm count"));
    }
    Ok(())
}

fn make_task(
    ordinal: usize,
    arm: M2CarrierArmV1,
    arm_ordinal: u32,
    context: OpeningContext,
    generator: [u32; 3],
    templates: Vec<BindingTemplateV1>,
) -> Result<M2OpeningTaskV1, M2BridgeError> {
    let ordinal = u32::try_from(ordinal).map_err(|_| M2BridgeError::Carrier("task ordinal"))?;
    let grade = u32::from(context.grade());
    let pool_count =
        u32::try_from(context.pool().len()).map_err(|_| M2BridgeError::Carrier("pool count"))?;
    let response_count = pool_count
        .checked_mul(
            pool_count
                .checked_sub(1)
                .ok_or(M2BridgeError::Carrier("response n-1"))?,
        )
        .and_then(|value| value.checked_mul(pool_count.checked_sub(2)?))
        .ok_or(M2BridgeError::Carrier("response count"))?;
    let candidate_count = response_count
        .checked_mul(10)
        .ok_or(M2BridgeError::Carrier("candidate count"))?;
    if candidate_count > OPENING_CANDIDATE_SLOT_CAP_V1 as u32 {
        return Err(M2BridgeError::Carrier("candidate cap"));
    }
    let matching_mask = context.matching_pool().bits();
    let task_words = [
        OPENING_ABI_VERSION_V1,
        ordinal,
        grade,
        context.pool().bits(),
        matching_mask,
        pool_count,
        response_count,
        candidate_count,
    ];
    validate_task_words(context, ordinal, &task_words)?;
    let task_key = M2TaskKeyV1 {
        words: [
            1,
            ordinal,
            arm.code(),
            arm_ordinal,
            decl_code_u32(context.decl()),
            context.led().index() as u32,
            grade,
            context.pool().bits(),
            matching_mask,
            pool_count,
            response_count,
            candidate_count,
            generator[0],
            generator[1],
            generator[2],
            0,
        ],
    };
    let task_key_sha256 = sha256(&task_key.to_le_bytes());
    let direct = direct_preflight(context)?;
    let mut bindings = Vec::with_capacity(templates.len());
    for template in templates {
        let binding = match template {
            BindingTemplateV1::Reduced {
                binding_ordinal,
                profile,
                root_selector,
                coordinate_ordinal,
                root,
                selected_action,
                matching_count,
            } => BindingIntentV1::Reduced(ReducedBindingIntentV1 {
                binding_ordinal,
                profile,
                task_ordinal: ordinal,
                arm_ordinal,
                root_selector,
                coordinate_ordinal,
                root,
                selected_action,
                context,
                matching_count,
                generator,
            }),
            BindingTemplateV1::Physical {
                binding_ordinal,
                endpoint,
                root,
                selected_action,
            } => BindingIntentV1::Physical(PhysicalBindingIntentV1 {
                binding_ordinal,
                task_ordinal: ordinal,
                arm,
                arm_ordinal,
                endpoint,
                root,
                selected_action,
                context,
            }),
        };
        bindings.push(binding);
    }
    Ok(M2OpeningTaskV1 {
        ordinal,
        arm,
        arm_ordinal,
        context,
        task_words,
        task_key,
        task_key_sha256,
        direct,
        bindings: bindings.into_boxed_slice(),
    })
}

fn canonical_reduced_roots() -> Result<[OpeningRootV1; 2], M2BridgeError> {
    let hand: DominoSet = [20usize, 21, 22, 23, 24, 25, 26]
        .into_iter()
        .map(|index| Domino::from_index(index).ok_or(M2BridgeError::Carrier("root hand tile")))
        .collect::<Result<DominoSet, _>>()?;
    let pip_six = Pip::new(6).ok_or(M2BridgeError::Carrier("pip six"))?;
    Ok([
        OpeningRootV1::new(
            Decl::NoTrump,
            Seat::S0,
            hand,
            OpeningContractV1::point_bid(30)?,
        )?,
        OpeningRootV1::new(
            Decl::PipTrump(pip_six),
            Seat::S2,
            hand,
            OpeningContractV1::Mark,
        )?,
    ])
}

fn natural_six() -> Context {
    Context::Natural(Pip::ALL[6])
}

fn grade_matching_context(grade: u8, matching_count: u8) -> Result<OpeningContext, M2BridgeError> {
    let decl = Decl::NoTrump;
    let led = natural_six();
    let matching = decl.effective_incidence(led);
    let nonmatching = DominoSet::FULL.difference(matching);
    let target = usize::from(grade)
        .checked_mul(3)
        .ok_or(M2BridgeError::Carrier("grade matching target"))?;
    let selected_matching = usize::from(matching_count);
    let selected_nonmatching = target
        .checked_sub(selected_matching)
        .ok_or(M2BridgeError::Carrier("grade matching subtraction"))?;
    if selected_matching > matching.len() || selected_nonmatching > nonmatching.len() {
        return Err(M2BridgeError::Carrier("grade matching availability"));
    }
    let pool: DominoSet = matching
        .iter()
        .take(selected_matching)
        .chain(nonmatching.iter().take(selected_nonmatching))
        .collect();
    Ok(OpeningContext::try_reduced(decl, led, pool, grade)?)
}

fn grade_matching_fixture_root(matching_count: u8) -> Result<OpeningRootV1, M2BridgeError> {
    let fixture_context = grade_matching_context(7, matching_count)?;
    Ok(OpeningRootV1::new(
        Decl::NoTrump,
        Seat::S0,
        DominoSet::FULL.difference(fixture_context.pool()),
        OpeningContractV1::point_bid(30)?,
    )?)
}

fn same_context_fixture_root(
    decl: Decl,
    first: Domino,
    second: Domino,
) -> Result<OpeningRootV1, M2BridgeError> {
    if first.index() >= second.index() || decl.led_context(first) != decl.led_context(second) {
        return Err(M2BridgeError::Carrier("SameContext pair geometry"));
    }
    let hand: DominoSet = [first, second]
        .into_iter()
        .chain(
            Domino::ALL
                .into_iter()
                .filter(|tile| *tile != first && *tile != second)
                .take(5),
        )
        .collect();
    Ok(OpeningRootV1::new(
        decl,
        Seat::S0,
        hand,
        OpeningContractV1::point_bid(30)?,
    )?)
}

fn least_context_action(root: OpeningRootV1, led: Context) -> Result<Domino, M2BridgeError> {
    root.legal_leads()
        .iter()
        .find(|action| root.decl().led_context(*action) == led)
        .ok_or(M2BridgeError::Carrier("least context action"))
}

fn validate_task_words(
    context: OpeningContext,
    ordinal: u32,
    words: &[u32; OPENING_TASK_WORDS_V1],
) -> Result<(), M2BridgeError> {
    if words[0] != OPENING_ABI_VERSION_V1 || words[1] != ordinal {
        return Err(M2BridgeError::Carrier("task ABI/ordinal"));
    }
    if !(1..=7).contains(&words[2]) {
        return Err(M2BridgeError::Carrier("task grade"));
    }
    if words[3] & !DominoSet::FULL.bits() != 0 || words[4] & !DominoSet::FULL.bits() != 0 {
        return Err(M2BridgeError::Carrier("task mask high bits"));
    }
    if words[4] & !words[3] != 0 {
        return Err(M2BridgeError::Carrier("task matching subset"));
    }
    if words[3].count_ones() != words[5] || words[5] != 3 * words[2] || words[4].count_ones() > 6 {
        return Err(M2BridgeError::Carrier("task count relation"));
    }
    let expected_responses = words[5] * (words[5] - 1) * (words[5] - 2);
    if words[6] != expected_responses
        || words[7] != expected_responses * 10
        || words[7] > OPENING_CANDIDATE_SLOT_CAP_V1 as u32
    {
        return Err(M2BridgeError::Carrier("task response/slot relation"));
    }
    if words[2] != u32::from(context.grade())
        || words[3] != context.pool().bits()
        || words[4] != context.matching_pool().bits()
    {
        return Err(M2BridgeError::Carrier("task context round trip"));
    }
    Ok(())
}

fn validate_reduced_intent(
    task: &M2OpeningTaskV1,
    intent: &ReducedBindingIntentV1,
) -> Result<(), M2BindingError> {
    if intent.task_ordinal != task.ordinal
        || intent.context != task.context
        || intent.arm_ordinal != task.arm_ordinal
        || intent.binding_ordinal != task.ordinal
    {
        return Err(M2BindingError::TaskMismatch);
    }
    if task.context.grade() == 7
        || !task.context.pool().is_subset_of(intent.root.hidden_pool())
        || intent.root.decl() != task.context.decl()
        || !intent.root.legal_leads().contains(intent.selected_action)
        || intent.root.decl().led_context(intent.selected_action) != task.context.led()
    {
        return Err(M2BindingError::CarrierInvariant(
            "reduced root/action/context",
        ));
    }
    let least = intent
        .root
        .legal_leads()
        .iter()
        .find(|action| intent.root.decl().led_context(*action) == task.context.led())
        .ok_or(M2BindingError::CarrierInvariant("reduced least action"))?;
    if least != intent.selected_action {
        return Err(M2BindingError::CarrierInvariant("reduced action order"));
    }
    match intent.profile {
        ReducedCarrierProfileV1::ReducedArm => {
            if task.arm != M2CarrierArmV1::Reduced
                || intent.root_selector > 1
                || intent.generator != [intent.root_selector, intent.coordinate_ordinal, 0]
            {
                return Err(M2BindingError::CarrierInvariant("Reduced binding identity"));
            }
            let roots = canonical_reduced_roots()
                .map_err(|_| M2BindingError::CarrierInvariant("Reduced canonical roots"))?;
            let root_index = usize::try_from(intent.root_selector)
                .map_err(|_| M2BindingError::CarrierInvariant("Reduced root selector"))?;
            let expected_root = *roots
                .get(root_index)
                .ok_or(M2BindingError::CarrierInvariant("Reduced root selector"))?;
            if intent.root != expected_root {
                return Err(M2BindingError::CarrierInvariant("Reduced canonical root"));
            }
            let carrier = ReducedOpeningCarrierV1::from_root(intent.root)
                .map_err(|_| M2BindingError::CarrierInvariant("Reduced carrier"))?;
            let coordinate = carrier
                .coordinates()
                .get(intent.coordinate_ordinal as usize)
                .ok_or(M2BindingError::CarrierInvariant(
                    "Reduced coordinate ordinal",
                ))?;
            if coordinate
                .opening_context()
                .map_err(|_| M2BindingError::CarrierInvariant("Reduced coordinate context"))?
                != task.context
                || coordinate.matching_count() != intent.matching_count
            {
                return Err(M2BindingError::CarrierInvariant("Reduced coordinate"));
            }
            let arm_offset = if root_index == 0 {
                0
            } else {
                u32::try_from(
                    ReducedOpeningCarrierV1::from_root(roots[0])
                        .map_err(|_| {
                            M2BindingError::CarrierInvariant("Reduced first-root carrier")
                        })?
                        .coordinates()
                        .len(),
                )
                .map_err(|_| M2BindingError::CarrierInvariant("Reduced arm offset"))?
            };
            if task.arm_ordinal
                != arm_offset
                    .checked_add(intent.coordinate_ordinal)
                    .ok_or(M2BindingError::CarrierInvariant("Reduced arm ordinal"))?
            {
                return Err(M2BindingError::CarrierInvariant("Reduced arm ordinal"));
            }
        }
        ReducedCarrierProfileV1::GradeMatching => {
            if task.arm != M2CarrierArmV1::GradeMatching || task.context.grade() >= 7 {
                return Err(M2BindingError::CarrierInvariant(
                    "GradeMatching binding identity",
                ));
            }
            let regenerated =
                grade_matching_context(task.context.grade(), intent.matching_count)
                    .map_err(|_| M2BindingError::CarrierInvariant("GradeMatching coordinate"))?;
            let fixture = grade_matching_context(7, intent.matching_count)
                .map_err(|_| M2BindingError::CarrierInvariant("GradeMatching fixture"))?;
            let expected_root = grade_matching_fixture_root(intent.matching_count)
                .map_err(|_| M2BindingError::CarrierInvariant("GradeMatching fixture root"))?;
            let prior_tasks = if task.context.grade() == 1 {
                0
            } else {
                4 + u32::from(task.context.grade() - 2) * 7
            };
            if regenerated != task.context
                || intent.root.hidden_pool() != fixture.pool()
                || intent.root != expected_root
                || intent.root_selector != u32::from(intent.matching_count)
                || intent.coordinate_ordinal != task.arm_ordinal
                || task.arm_ordinal != prior_tasks + u32::from(intent.matching_count)
                || intent.generator
                    != [
                        u32::from(task.context.grade()),
                        u32::from(intent.matching_count),
                        0,
                    ]
            {
                return Err(M2BindingError::CarrierInvariant("GradeMatching binding"));
            }
        }
    }
    Ok(())
}

fn validate_physical_intent(
    task: &M2OpeningTaskV1,
    intent: &PhysicalBindingIntentV1,
) -> Result<(), M2BindingError> {
    if intent.task_ordinal != task.ordinal
        || intent.context != task.context
        || intent.arm != task.arm
        || intent.arm_ordinal != task.arm_ordinal
    {
        return Err(M2BindingError::TaskMismatch);
    }
    let expected_binding_ordinal = match task.arm {
        M2CarrierArmV1::GradeMatching if task.context.grade() == 7 && intent.endpoint == 0 => task
            .arm_ordinal
            .checked_sub(39)
            .ok_or(M2BindingError::CarrierInvariant(
                "GradeMatching physical ordinal",
            ))?,
        M2CarrierArmV1::SameContextPair if intent.endpoint <= 1 => 7u32
            .checked_add(task.arm_ordinal.checked_mul(2).ok_or(
                M2BindingError::CarrierInvariant("SameContext physical ordinal"),
            )?)
            .and_then(|value| value.checked_add(intent.endpoint))
            .ok_or(M2BindingError::CarrierInvariant(
                "SameContext physical ordinal",
            ))?,
        _ => {
            return Err(M2BindingError::CarrierInvariant(
                "physical arm and endpoint",
            ));
        }
    };
    if intent.binding_ordinal != expected_binding_ordinal {
        return Err(M2BindingError::CarrierInvariant("physical binding ordinal"));
    }
    match task.arm {
        M2CarrierArmV1::GradeMatching => {
            let matching_count =
                u8::try_from(task.context.matching_pool().len()).map_err(|_| {
                    M2BindingError::CarrierInvariant("GradeMatching physical matching count")
                })?;
            let expected_root = grade_matching_fixture_root(matching_count).map_err(|_| {
                M2BindingError::CarrierInvariant("GradeMatching physical fixture root")
            })?;
            if intent.root != expected_root
                || task.task_key.words[12..15] != [7, u32::from(matching_count), 0]
            {
                return Err(M2BindingError::CarrierInvariant(
                    "GradeMatching physical fixture",
                ));
            }
        }
        M2CarrierArmV1::SameContextPair => {
            let decl = task.context.decl();
            let first = Domino::from_index(task.task_key.words[13] as usize)
                .ok_or(M2BindingError::CarrierInvariant("SameContext first action"))?;
            let second = Domino::from_index(task.task_key.words[14] as usize).ok_or(
                M2BindingError::CarrierInvariant("SameContext second action"),
            )?;
            let expected_root = same_context_fixture_root(decl, first, second)
                .map_err(|_| M2BindingError::CarrierInvariant("SameContext fixture root"))?;
            let expected_action = if intent.endpoint == 0 { first } else { second };
            if task.task_key.words[12] != decl_code_u32(decl)
                || intent.root != expected_root
                || intent.selected_action != expected_action
            {
                return Err(M2BindingError::CarrierInvariant("SameContext fixture"));
            }
        }
        M2CarrierArmV1::Reduced => {
            return Err(M2BindingError::CarrierInvariant("physical Reduced arm"));
        }
    }
    if task.context.grade() != 7
        || intent.root.decl() != task.context.decl()
        || !intent.root.legal_leads().contains(intent.selected_action)
        || intent.root.decl().led_context(intent.selected_action) != task.context.led()
        || intent
            .root
            .opening_context(task.context.led())
            .map_err(|_| M2BindingError::CarrierInvariant("physical context"))?
            != task.context
    {
        return Err(M2BindingError::CarrierInvariant(
            "physical root/action/context",
        ));
    }
    Ok(())
}

fn decl_code_u32(decl: Decl) -> u32 {
    match decl {
        Decl::PipTrump(pip) => u32::from(pip.value()),
        Decl::DoublesTrump => 7,
        Decl::NoTrump => 8,
    }
}

const OPENING_NEGATIVE_ARENA_SLOT_COUNT_V1: usize = 12;

/// One closed malformed opening descriptor and its complete expected arena.
///
/// Construction is intentionally private: these descriptors bypass the
/// production host preflight and exist only for the frozen negative gate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M2OpeningNegativeControlV1 {
    ordinal: u32,
    task_words: [u32; OPENING_TASK_WORDS_V1],
    expected_status: u32,
    expected_slots: [[u32; OPENING_SLOT_WORDS_V1]; OPENING_NEGATIVE_ARENA_SLOT_COUNT_V1],
}

impl M2OpeningNegativeControlV1 {
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub const fn task_words(&self) -> &[u32; OPENING_TASK_WORDS_V1] {
        &self.task_words
    }

    pub const fn expected_status(&self) -> u32 {
        self.expected_status
    }

    pub const fn expected_slots(
        &self,
    ) -> &[[u32; OPENING_SLOT_WORDS_V1]; OPENING_NEGATIVE_ARENA_SLOT_COUNT_V1] {
        &self.expected_slots
    }
}

/// The exact thirteen frozen opening-descriptor negative controls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M2OpeningNegativeControlsV1 {
    controls: [M2OpeningNegativeControlV1; OPENING_NEGATIVE_CONTROL_COUNT_V1],
}

impl M2OpeningNegativeControlsV1 {
    pub fn canonical() -> Result<Self, M2BridgeError> {
        let mut controls = Vec::with_capacity(OPENING_NEGATIVE_CONTROL_COUNT_V1);
        for ordinal in 0..OPENING_NEGATIVE_CONTROL_COUNT_V1 {
            let task_ordinal = u32::try_from(ordinal)
                .map_err(|_| M2BridgeError::Carrier("opening negative ordinal"))?;
            let mut task_words = [
                OPENING_ABI_VERSION_V1,
                task_ordinal,
                1,
                0x0000_0007,
                0,
                3,
                6,
                60,
            ];
            match ordinal {
                0 => task_words[0] = 0,
                1 => task_words[0] = 2,
                2 => task_words[3] |= 0x1000_0000,
                3 => task_words[4] = 0x1000_0000,
                4 => task_words[4] = 0x0000_0008,
                5 => task_words[2] = 0,
                6 => task_words[2] = 8,
                7 => task_words[5] = 4,
                8 => task_words[3] = 0x0000_000f,
                9 => task_words[2] = 2,
                10 => {
                    task_words[2] = 3;
                    task_words[3] = 0x0000_01ff;
                    task_words[4] = 0x0000_007f;
                    task_words[5] = 9;
                    task_words[6] = 504;
                    task_words[7] = 5_040;
                }
                11 => task_words[6] = 7,
                12 => task_words[7] = 61,
                _ => unreachable!(),
            }
            let expected_status = malformed_opening_status(&task_words)
                .ok_or(M2BridgeError::Carrier("opening negative remained valid"))?;
            let mut expected_slots =
                [[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]; OPENING_NEGATIVE_ARENA_SLOT_COUNT_V1];
            for (slot_ordinal, slot) in expected_slots[..10].iter_mut().enumerate() {
                *slot = canonical_opening_status_slot(
                    expected_status,
                    task_ordinal,
                    u32::try_from(slot_ordinal)
                        .map_err(|_| M2BridgeError::Carrier("opening negative slot ordinal"))?,
                );
            }
            controls.push(M2OpeningNegativeControlV1 {
                ordinal: task_ordinal,
                task_words,
                expected_status,
                expected_slots,
            });
        }
        let controls = controls
            .try_into()
            .map_err(|_| M2BridgeError::Carrier("opening negative control count"))?;
        Ok(Self { controls })
    }

    pub const fn controls(
        &self,
    ) -> &[M2OpeningNegativeControlV1; OPENING_NEGATIVE_CONTROL_COUNT_V1] {
        &self.controls
    }
}

fn malformed_opening_status(words: &[u32; OPENING_TASK_WORDS_V1]) -> Option<u32> {
    if words[0] != OPENING_ABI_VERSION_V1 {
        return Some(OPENING_HARD_BAD_ABI_V1);
    }
    if words[3] & !DominoSet::FULL.bits() != 0 || words[4] & !DominoSet::FULL.bits() != 0 {
        return Some(OPENING_HARD_BAD_MASK_V1);
    }
    if words[4] & !words[3] != 0 {
        return Some(OPENING_HARD_BAD_MASK_V1);
    }
    if !(1..=7).contains(&words[2]) {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    }
    if words[3].count_ones() != words[5] || words[5] != 3 * words[2] {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    }
    if words[4].count_ones() > 6 {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    }
    let Some(response_count) = words[5]
        .checked_mul(words[5].saturating_sub(1))
        .and_then(|value| value.checked_mul(words[5].saturating_sub(2)))
    else {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    };
    if words[6] != response_count {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    }
    if words[7] != response_count.saturating_mul(10)
        || words[7] > OPENING_CANDIDATE_SLOT_CAP_V1 as u32
    {
        return Some(OPENING_HARD_BAD_COUNT_V1);
    }
    None
}

fn canonical_opening_status_slot(
    status: u32,
    task_ordinal: u32,
    slot_ordinal: u32,
) -> [u32; OPENING_SLOT_WORDS_V1] {
    let mut slot = [0u32; OPENING_SLOT_WORDS_V1];
    slot[0] = status;
    slot[1] = task_ordinal;
    slot[2] = slot_ordinal;
    slot
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompactedCellV1 {
    response: [u32; 3],
    matching_counts: [u32; 3],
    support: u32,
    coefficient: u64,
    mass: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompactedAggregateV1 {
    response: [u32; 3],
    support: u64,
    mass: u64,
}

fn require_frozen_bound(
    field: &'static str,
    actual: u64,
    maximum: u64,
) -> Result<(), M2BridgeError> {
    if actual > maximum {
        return Err(M2BridgeError::FrozenBoundExceeded {
            field,
            maximum,
            actual,
        });
    }
    Ok(())
}

fn validate_opening_cell_bounds(
    support: u32,
    coefficient: u64,
    mass: u64,
) -> Result<(), M2BridgeError> {
    require_frozen_bound(
        "opening cell support",
        u64::from(support),
        u64::from(OPENING_MAX_CELL_SUPPORT_V1),
    )?;
    require_frozen_bound(
        "opening cell coefficient",
        coefficient,
        OPENING_MAX_CELL_COEFFICIENT_V1,
    )?;
    require_frozen_bound("opening cell mass", mass, OPENING_MAX_CELL_MASS_V1)
}

fn validate_opening_whole_mass_bound(total_mass: u64) -> Result<(), M2BridgeError> {
    require_frozen_bound("opening whole mass", total_mass, OPENING_MAX_WHOLE_MASS_V1)
}

fn render_expected_slots(
    task: &M2OpeningTaskV1,
    projection: &OpeningProjection,
) -> Result<Box<[[u32; OPENING_SLOT_WORDS_V1]]>, M2BridgeError> {
    if projection.context() != task.context {
        return Err(M2BridgeError::TaskMismatch);
    }
    validate_task_words(task.context, task.ordinal, &task.task_words)?;
    validate_opening_whole_mass_bound(projection.total_scaled_mass()?.value())?;
    let candidate_count = usize::try_from(task.candidate_slot_count())
        .map_err(|_| M2BridgeError::Carrier("candidate slot usize"))?;
    let response_count = usize::try_from(task.response_count())
        .map_err(|_| M2BridgeError::Carrier("response count usize"))?;
    let mut slots = vec![[0u32; OPENING_SLOT_WORDS_V1]; candidate_count];
    let mut cell_cursor = 0usize;

    for q in 0..response_count {
        let response = unrank_response(task.context.pool(), q)?;
        let start = q
            .checked_mul(10)
            .ok_or(M2BridgeError::Carrier("rendered slot ordinal"))?;
        for local in 0..10usize {
            let slot_ordinal = start
                .checked_add(local)
                .ok_or(M2BridgeError::Carrier("rendered local slot ordinal"))?;
            slots[slot_ordinal] = canonical_opening_status_slot(
                OPENING_SLOT_SKIP_V1,
                task.ordinal,
                u32::try_from(slot_ordinal)
                    .map_err(|_| M2BridgeError::Carrier("rendered slot u32"))?,
            );
        }

        let group_start = cell_cursor;
        while let Some(cell) = projection.cells().get(cell_cursor) {
            if cell.key().response() != response {
                break;
            }
            let local = cell_cursor - group_start;
            if local >= 10 {
                return Err(M2BridgeError::MalformedValidSlot {
                    slot: u32::try_from(start)
                        .map_err(|_| M2BridgeError::Carrier("too-many-strata slot"))?,
                    field: "scalar response strata count",
                });
            }
            let slot_ordinal = start
                .checked_add(local)
                .ok_or(M2BridgeError::Carrier("rendered valid slot ordinal"))?;
            slots[slot_ordinal] = render_scalar_cell_slot(task.ordinal, slot_ordinal, *cell)?;
            cell_cursor += 1;
        }
    }
    if cell_cursor != projection.cells().len() {
        return Err(M2BridgeError::Carrier(
            "raw renderer did not consume scalar cells",
        ));
    }
    Ok(slots.into_boxed_slice())
}

fn render_scalar_cell_slot(
    task_ordinal: u32,
    slot_ordinal: usize,
    cell: crate::OpeningCell,
) -> Result<[u32; OPENING_SLOT_WORDS_V1], M2BridgeError> {
    let mut slot = canonical_opening_status_slot(
        OPENING_SLOT_VALID_V1,
        task_ordinal,
        u32::try_from(slot_ordinal).map_err(|_| M2BridgeError::Carrier("valid slot u32"))?,
    );
    for (word, tile) in slot[3..6].iter_mut().zip(cell.key().response()) {
        *word = u32::try_from(tile.index())
            .map_err(|_| M2BridgeError::Carrier("response physical index"))?;
    }
    for (word, count) in slot[6..9].iter_mut().zip(cell.key().matching_counts()) {
        *word = u32::from(count);
    }
    let support = cell.support().value();
    let coefficient = cell.per_world_coefficient().value();
    let mass = cell.scaled_mass()?.value();
    validate_opening_cell_bounds(support, coefficient, mass)?;
    slot[9] = support;
    slot[10] = coefficient as u32;
    slot[11] = (coefficient >> 32) as u32;
    slot[12] = mass as u32;
    slot[13] = (mass >> 32) as u32;
    Ok(slot)
}

fn unrank_response(pool: DominoSet, q: usize) -> Result<[Domino; 3], M2BridgeError> {
    let tiles: Vec<Domino> = pool.iter().collect();
    let n = tiles.len();
    if n < 3 {
        return Err(M2BridgeError::Carrier("response pool cardinality"));
    }
    let response_count = n
        .checked_mul(n - 1)
        .and_then(|value| value.checked_mul(n - 2))
        .ok_or(M2BridgeError::Carrier("response unrank count"))?;
    if q >= response_count {
        return Err(M2BridgeError::Carrier("response ordinal range"));
    }
    let block = (n - 1) * (n - 2);
    let i = q / block;
    let remainder = q % block;
    let j_rank = remainder / (n - 2);
    let k_rank = remainder % (n - 2);
    let j = if j_rank >= i { j_rank + 1 } else { j_rank };
    let lower = i.min(j);
    let upper = i.max(j);
    let mut k = k_rank;
    if k >= lower {
        k += 1;
    }
    if k >= upper {
        k += 1;
    }
    Ok([tiles[i], tiles[j], tiles[k]])
}

struct ValidatedSlotArenaV1 {
    checked: CheckedM2ProjectionPayloadV1,
    cpu_raw_bytes: Vec<u8>,
    gpu_raw_bytes: Vec<u8>,
    cpu_payload_bytes: Vec<u8>,
    gpu_payload_bytes: Vec<u8>,
    cpu_aggregate_bytes: Vec<u8>,
    gpu_aggregate_bytes: Vec<u8>,
}

fn validate_slot_arena(
    task: &M2OpeningTaskV1,
    arena: &[[u32; OPENING_SLOT_WORDS_V1]],
) -> Result<CheckedM2ProjectionPayloadV1, M2BridgeError> {
    Ok(validate_slot_arena_detailed(task, arena)?.checked)
}

fn validate_slot_arena_detailed(
    task: &M2OpeningTaskV1,
    arena: &[[u32; OPENING_SLOT_WORDS_V1]],
) -> Result<ValidatedSlotArenaV1, M2BridgeError> {
    if arena.len() != OPENING_ARENA_SLOT_COUNT_V1 {
        return Err(M2BridgeError::ArenaLength {
            expected: OPENING_ARENA_SLOT_COUNT_V1,
            actual: arena.len(),
        });
    }
    validate_task_words(task.context, task.ordinal, &task.task_words)?;
    let projection = project_closed_form(task.context)?;
    let expected_slots = render_expected_slots(task, &projection)?;
    let candidate_count = expected_slots.len();

    // Validate GPU-produced semantic magnitudes independently before the
    // byte-for-byte scalar comparison below.  Equality is not a substitute for
    // enforcing the separately proved frozen bounds at the acceptance edge.
    let cells = compact_valid_cells(&arena[..candidate_count])?;
    if cells.len() > MAX_OPENING_CELLS_V1 {
        return Err(M2BridgeError::TooManyCells {
            actual: cells.len(),
            cap: MAX_OPENING_CELLS_V1,
        });
    }
    validate_compacted_cells(&cells)?;
    let total_mass = cells.iter().try_fold(0u64, |total, cell| {
        total
            .checked_add(cell.mass)
            .ok_or(M2BridgeError::MalformedValidSlot {
                slot: task.ordinal,
                field: "total mass overflow",
            })
    })?;
    validate_opening_whole_mass_bound(total_mass)?;

    for (slot_ordinal, (actual, expected)) in arena[..candidate_count]
        .iter()
        .zip(expected_slots.iter())
        .enumerate()
    {
        for (word_ordinal, (actual_word, expected_word)) in
            actual.iter().zip(expected.iter()).enumerate()
        {
            if actual_word != expected_word {
                return Err(M2BridgeError::SlotMismatch {
                    slot: u32::try_from(slot_ordinal)
                        .map_err(|_| M2BridgeError::Carrier("mismatch slot ordinal"))?,
                    word: u8::try_from(word_ordinal)
                        .map_err(|_| M2BridgeError::Carrier("mismatch word ordinal"))?,
                    expected: *expected_word,
                    actual: *actual_word,
                });
            }
        }
    }
    for (slot_ordinal, slot) in arena[candidate_count..].iter().enumerate() {
        let absolute_slot = candidate_count
            .checked_add(slot_ordinal)
            .ok_or(M2BridgeError::Carrier("protected slot ordinal"))?;
        for (word_ordinal, word) in slot.iter().enumerate() {
            if *word != M2_POISON_WORD_V1 {
                return Err(M2BridgeError::ProtectedWordMismatch {
                    slot: u32::try_from(absolute_slot)
                        .map_err(|_| M2BridgeError::Carrier("protected slot u32"))?,
                    word: u8::try_from(word_ordinal)
                        .map_err(|_| M2BridgeError::Carrier("protected word u8"))?,
                    actual: *word,
                });
            }
        }
    }

    let payload = encode_m1_payload_from_cells(task.context, &cells, total_mass)?;
    let scalar_payload = projection.canonical_projector_payload_bytes_v1()?;
    if payload != scalar_payload {
        return Err(M2BridgeError::PayloadMismatch);
    }

    let aggregates = compact_response_aggregates(&cells)?;
    let scalar_aggregates = validate_aggregates_against_scalar(&aggregates, &projection)?;
    let gpu_aggregate_bytes = encode_aggregate_records(&aggregates);
    let cpu_aggregate_bytes = encode_aggregate_records(&scalar_aggregates);
    let gpu_raw_bytes = slots_to_le_bytes(&arena[..candidate_count]);
    let cpu_raw_bytes = slots_to_le_bytes(&expected_slots);
    let raw_record_count = u64::try_from(candidate_count)
        .map_err(|_| M2BridgeError::Carrier("raw stream record count"))?;
    let cpu_raw_sha256 = stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_record_count, &cpu_raw_bytes)?;
    let gpu_raw_sha256 = stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_record_count, &gpu_raw_bytes)?;
    let cpu_payload_sha256 = stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &scalar_payload)?;
    let gpu_payload_sha256 = stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &payload)?;
    let cpu_aggregate_sha256 = stream_digest(
        STREAM_CONTEXT_AGGREGATES_V1,
        u64::try_from(scalar_aggregates.len())
            .map_err(|_| M2BridgeError::Carrier("CPU aggregate record count"))?,
        &cpu_aggregate_bytes,
    )?;
    let gpu_aggregate_sha256 = stream_digest(
        STREAM_CONTEXT_AGGREGATES_V1,
        u64::try_from(aggregates.len())
            .map_err(|_| M2BridgeError::Carrier("GPU aggregate record count"))?,
        &gpu_aggregate_bytes,
    )?;
    if cpu_raw_sha256 != gpu_raw_sha256
        || cpu_payload_sha256 != gpu_payload_sha256
        || cpu_aggregate_sha256 != gpu_aggregate_sha256
    {
        return Err(M2BridgeError::GlobalStream(
            "per-task CPU/GPU digest parity",
        ));
    }
    let payload_length =
        u64::try_from(payload.len()).map_err(|_| M2BridgeError::Carrier("payload length"))?;
    let cell_count =
        u32::try_from(cells.len()).map_err(|_| M2BridgeError::Carrier("cell count"))?;

    let checked = CheckedM2ProjectionPayloadV1 {
        task_ordinal: task.ordinal,
        task_key_sha256: task.task_key_sha256,
        context: task.context,
        response_count: task.response_count(),
        candidate_slot_count: task.candidate_slot_count(),
        cell_count,
        total_mass,
        cpu_raw_sha256,
        gpu_raw_sha256,
        cpu_payload_sha256,
        gpu_payload_sha256,
        payload_length,
        cpu_aggregate_sha256,
        gpu_aggregate_sha256,
        canonical_payload: payload.clone().into_boxed_slice(),
    };
    Ok(ValidatedSlotArenaV1 {
        checked,
        cpu_raw_bytes,
        gpu_raw_bytes,
        cpu_payload_bytes: scalar_payload,
        gpu_payload_bytes: payload,
        cpu_aggregate_bytes,
        gpu_aggregate_bytes,
    })
}

fn compact_valid_cells(
    slots: &[[u32; OPENING_SLOT_WORDS_V1]],
) -> Result<Vec<CompactedCellV1>, M2BridgeError> {
    let mut cells = Vec::new();
    for (slot_ordinal, slot) in slots.iter().enumerate() {
        match slot[0] {
            OPENING_SLOT_SKIP_V1 => {}
            OPENING_SLOT_VALID_V1 => cells.push(CompactedCellV1 {
                response: [slot[3], slot[4], slot[5]],
                matching_counts: [slot[6], slot[7], slot[8]],
                support: slot[9],
                coefficient: u64::from(slot[10]) | (u64::from(slot[11]) << 32),
                mass: u64::from(slot[12]) | (u64::from(slot[13]) << 32),
            }),
            _ => {
                return Err(M2BridgeError::MalformedValidSlot {
                    slot: u32::try_from(slot_ordinal)
                        .map_err(|_| M2BridgeError::Carrier("unknown-status slot"))?,
                    field: "status",
                });
            }
        }
    }
    Ok(cells)
}

fn validate_compacted_cells(cells: &[CompactedCellV1]) -> Result<(), M2BridgeError> {
    let mut prior_key: Option<[u32; 6]> = None;
    for (ordinal, cell) in cells.iter().enumerate() {
        let slot =
            u32::try_from(ordinal).map_err(|_| M2BridgeError::Carrier("compacted cell ordinal"))?;
        let key = [
            cell.response[0],
            cell.response[1],
            cell.response[2],
            cell.matching_counts[0],
            cell.matching_counts[1],
            cell.matching_counts[2],
        ];
        if prior_key.is_some_and(|prior| prior >= key) {
            return Err(M2BridgeError::MalformedValidSlot {
                slot,
                field: "strict cell key order",
            });
        }
        if cell
            .response
            .iter()
            .any(|index| *index >= Domino::COUNT as u32)
            || cell.response[0] == cell.response[1]
            || cell.response[0] == cell.response[2]
            || cell.response[1] == cell.response[2]
        {
            return Err(M2BridgeError::MalformedValidSlot {
                slot,
                field: "response",
            });
        }
        if cell.support == 0 {
            return Err(M2BridgeError::MalformedValidSlot {
                slot,
                field: "zero support",
            });
        }
        validate_opening_cell_bounds(cell.support, cell.coefficient, cell.mass)?;
        let expected_mass = u64::from(cell.support)
            .checked_mul(cell.coefficient)
            .ok_or(M2BridgeError::MalformedValidSlot {
                slot,
                field: "support coefficient overflow",
            })?;
        if cell.mass != expected_mass {
            return Err(M2BridgeError::MalformedValidSlot {
                slot,
                field: "support coefficient mass",
            });
        }
        prior_key = Some(key);
    }
    Ok(())
}

fn encode_m1_payload_from_cells(
    context: OpeningContext,
    cells: &[CompactedCellV1],
    total_mass: u64,
) -> Result<Vec<u8>, M2BridgeError> {
    let cell_bytes = cells
        .len()
        .checked_mul(OPENING_RECEIPT_CELL_BYTES)
        .ok_or(M2BridgeError::Carrier("M1 cell bytes"))?;
    let capacity = OPENING_RECEIPT_HEADER_BYTES
        .checked_add(cell_bytes)
        .ok_or(M2BridgeError::Carrier("M1 payload capacity"))?;
    let mut bytes = Vec::with_capacity(capacity);
    bytes.extend_from_slice(&OPENING_RECEIPT_MAGIC);
    push_u16(&mut bytes, OPENING_RECEIPT_VERSION);
    bytes.push(
        u8::try_from(decl_code_u32(context.decl()))
            .map_err(|_| M2BridgeError::Carrier("M1 declaration code"))?,
    );
    bytes.push(
        u8::try_from(context.led().index())
            .map_err(|_| M2BridgeError::Carrier("M1 led context"))?,
    );
    bytes.push(context.grade());
    bytes.push(0);
    push_u32(&mut bytes, FIELD_SCALE);
    push_u32(&mut bytes, context.pool().bits());
    push_u64(&mut bytes, context.physical_world_count()?);
    push_u64(&mut bytes, M1_DIRECT_WORLD_CAP_V1);
    push_u32(
        &mut bytes,
        u32::try_from(cells.len()).map_err(|_| M2BridgeError::Carrier("M1 cell count"))?,
    );
    push_u64(&mut bytes, total_mass);
    for cell in cells {
        for response in cell.response {
            bytes.push(
                u8::try_from(response).map_err(|_| M2BridgeError::Carrier("M1 response index"))?,
            );
        }
        for count in cell.matching_counts {
            bytes.push(
                u8::try_from(count).map_err(|_| M2BridgeError::Carrier("M1 matching count"))?,
            );
        }
        push_u32(&mut bytes, cell.support);
        push_u64(&mut bytes, cell.coefficient);
        push_u64(&mut bytes, cell.mass);
    }
    if bytes.len() != capacity {
        return Err(M2BridgeError::Carrier("M1 payload rendered length"));
    }
    Ok(bytes)
}

fn compact_response_aggregates(
    cells: &[CompactedCellV1],
) -> Result<Vec<CompactedAggregateV1>, M2BridgeError> {
    let mut aggregates: Vec<CompactedAggregateV1> = Vec::new();
    for cell in cells {
        match aggregates.last_mut() {
            Some(aggregate) if aggregate.response == cell.response => {
                aggregate.support = aggregate
                    .support
                    .checked_add(u64::from(cell.support))
                    .ok_or(M2BridgeError::MalformedValidSlot {
                        slot: 0,
                        field: "response support overflow",
                    })?;
                aggregate.mass = aggregate.mass.checked_add(cell.mass).ok_or(
                    M2BridgeError::MalformedValidSlot {
                        slot: 0,
                        field: "response mass overflow",
                    },
                )?;
            }
            _ => aggregates.push(CompactedAggregateV1 {
                response: cell.response,
                support: u64::from(cell.support),
                mass: cell.mass,
            }),
        }
    }
    Ok(aggregates)
}

fn validate_aggregates_against_scalar(
    actual: &[CompactedAggregateV1],
    projection: &OpeningProjection,
) -> Result<Vec<CompactedAggregateV1>, M2BridgeError> {
    let expected: Vec<CompactedAggregateV1> = projection
        .response_aggregates()?
        .into_iter()
        .map(|aggregate| CompactedAggregateV1 {
            response: aggregate.response().map(|tile| tile.index() as u32),
            support: aggregate.support(),
            mass: aggregate.scaled_mass().value(),
        })
        .collect();
    if actual.len() != expected.len() {
        return Err(M2BridgeError::AggregateMismatch);
    }
    for (actual, expected) in actual.iter().zip(&expected) {
        if actual != expected {
            return Err(M2BridgeError::AggregateMismatch);
        }
    }
    Ok(expected)
}

fn encode_aggregate_records(aggregates: &[CompactedAggregateV1]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(aggregates.len() * 32);
    for aggregate in aggregates {
        for response in aggregate.response {
            push_u32(&mut bytes, response);
        }
        push_u32(&mut bytes, 0);
        push_u64(&mut bytes, aggregate.support);
        push_u64(&mut bytes, aggregate.mass);
    }
    bytes
}

fn slots_to_le_bytes(slots: &[[u32; OPENING_SLOT_WORDS_V1]]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(slots.len() * OPENING_SLOT_WORDS_V1 * 4);
    for slot in slots {
        for word in slot {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
    }
    bytes
}

fn words_to_le_bytes(words: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}

fn stream_digest(
    purpose: u32,
    record_count: u64,
    payload: &[u8],
) -> Result<[u8; 32], M2BridgeError> {
    let payload_len = u64::try_from(payload.len())
        .map_err(|_| M2BridgeError::Carrier("stream payload length"))?;
    let mut state = Sha256State::new();
    state.update(STREAM_MAGIC_V1);
    state.update(&purpose.to_le_bytes());
    state.update(&STREAM_VERSION_V1.to_le_bytes());
    state.update(&record_count.to_le_bytes());
    state.update(&payload_len.to_le_bytes());
    state.update(payload);
    Ok(state.finish())
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

    #[test]
    fn choose_table_is_closed_and_biguint_checked() {
        let table = OpeningChooseTableV1::canonical().expect("canonical choose table");
        assert_eq!(table.words().len(), 22 * 22);
        assert_eq!(table.words()[21 * 22 + 10], 352_716);
        assert_eq!(table.words()[20 * 22 + 21], 0);
        assert_ne!(table.digest(), [0; 32]);
    }

    #[test]
    fn frozen_projector_magnitude_bounds_accept_boundary_and_reject_successor() {
        validate_opening_cell_bounds(
            OPENING_MAX_CELL_SUPPORT_V1,
            OPENING_MAX_CELL_COEFFICIENT_V1,
            OPENING_MAX_CELL_MASS_V1,
        )
        .expect("all exact cell maxima are admitted");
        validate_opening_whole_mass_bound(OPENING_MAX_WHOLE_MASS_V1)
            .expect("exact whole-mass maximum is admitted");
        require_frozen_bound(
            "choose entry",
            u64::from(OPENING_MAX_CHOOSE_ENTRY_V1),
            u64::from(OPENING_MAX_CHOOSE_ENTRY_V1),
        )
        .expect("exact choose maximum is admitted");

        for (field, result) in [
            (
                "opening cell support",
                validate_opening_cell_bounds(
                    OPENING_MAX_CELL_SUPPORT_V1 + 1,
                    OPENING_MAX_CELL_COEFFICIENT_V1,
                    OPENING_MAX_CELL_MASS_V1,
                ),
            ),
            (
                "opening cell coefficient",
                validate_opening_cell_bounds(
                    OPENING_MAX_CELL_SUPPORT_V1,
                    OPENING_MAX_CELL_COEFFICIENT_V1 + 1,
                    OPENING_MAX_CELL_MASS_V1,
                ),
            ),
            (
                "opening cell mass",
                validate_opening_cell_bounds(
                    OPENING_MAX_CELL_SUPPORT_V1,
                    OPENING_MAX_CELL_COEFFICIENT_V1,
                    OPENING_MAX_CELL_MASS_V1 + 1,
                ),
            ),
            (
                "opening whole mass",
                validate_opening_whole_mass_bound(OPENING_MAX_WHOLE_MASS_V1 + 1),
            ),
            (
                "choose entry",
                require_frozen_bound(
                    "choose entry",
                    u64::from(OPENING_MAX_CHOOSE_ENTRY_V1) + 1,
                    u64::from(OPENING_MAX_CHOOSE_ENTRY_V1),
                ),
            ),
        ] {
            assert!(matches!(
                result,
                Err(M2BridgeError::FrozenBoundExceeded {
                    field: actual,
                    ..
                }) if actual == field
            ));
        }
    }

    #[test]
    fn arithmetic_corpus_and_negative_controls_are_exactly_sized() {
        let corpus = M2ArithmeticCorpusV1::canonical().expect("canonical arithmetic corpus");
        assert_eq!(corpus.inputs().len(), M2_ARITHMETIC_CASE_COUNT_V1);
        assert_eq!(corpus.expected_outputs().len(), M2_ARITHMETIC_CASE_COUNT_V1);
        for (case_id, input) in corpus.inputs().iter().enumerate() {
            assert_eq!(input[1], case_id as u32);
            assert_eq!(input[0], ARITHMETIC_ABI_VERSION_V1);
        }

        let controls = ArithmeticNegativeControlsV1::canonical().expect("arithmetic controls");
        assert_eq!(
            controls.controls().len(),
            ARITHMETIC_NEGATIVE_CONTROL_COUNT_V1
        );
        for control in controls.controls() {
            assert_eq!(control.expected_output()[0] & 0x8000_0000, 0x8000_0000);
            assert_eq!(control.expected_output()[1], control.ordinal());
        }
    }

    #[test]
    fn canonical_arithmetic_receipt_facts_close_digests_and_censuses() {
        let facts =
            canonical_arithmetic_receipt_facts_v1().expect("canonical arithmetic receipt facts");
        assert_eq!(facts.official.case_count, 16_384);
        assert_eq!(facts.official.accepted_count, 16_384);
        assert_eq!(facts.official.input_payload_bytes, 16_384 * 80);
        assert_eq!(facts.official.output_payload_bytes, 16_384 * 64);
        assert_eq!(facts.official.success_count, 8_141);
        assert_eq!(facts.official.checked_undefined_count, 8_243);
        assert_eq!(facts.official.hard_count, 0);
        assert_eq!(
            facts
                .official
                .success_count
                .checked_add(facts.official.checked_undefined_count),
            Some(16_384)
        );
        assert_eq!(facts.negative.case_count, 13);
        assert_eq!(facts.negative.accepted_count, 0);
        assert_eq!(facts.negative.input_payload_bytes, 13 * 80);
        assert_eq!(facts.negative.output_payload_bytes, 13 * 64);
        assert_eq!(facts.negative.success_count, 0);
        assert_eq!(facts.negative.checked_undefined_count, 0);
        assert_eq!(facts.negative.hard_count, 13);
        assert_eq!(facts.official.guard_sha256, facts.negative.guard_sha256);
        assert_ne!(facts.official.input_sha256, facts.negative.input_sha256);
        assert_ne!(facts.official.output_sha256, facts.negative.output_sha256);

        let corpus = M2ArithmeticCorpusV1::canonical().expect("comparison arithmetic corpus");
        let official_inputs =
            fixed_word_records_to_le_bytes(corpus.inputs()).expect("official input bytes");
        let official_outputs = fixed_word_records_to_le_bytes(corpus.expected_outputs())
            .expect("official output bytes");
        assert_eq!(
            facts.official.input_sha256,
            stream_digest(STREAM_ARITHMETIC_INPUT_V1, 16_384, &official_inputs)
                .expect("official input digest")
        );
        assert_eq!(
            facts.official.output_sha256,
            stream_digest(STREAM_ARITHMETIC_OUTPUT_V1, 16_384, &official_outputs)
                .expect("official output digest")
        );
    }

    #[test]
    fn repeated_protected_record_hasher_matches_one_shot_stream() {
        let record = slots_to_le_bytes(&[[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]]);
        let mut three_records = Vec::with_capacity(record.len() * 3);
        for _ in 0..3 {
            three_records.extend_from_slice(&record);
        }
        assert_eq!(
            repeated_record_stream_digest(STREAM_PROTECTED_RECORDS_V1, 3, &record)
                .expect("streamed protected digest"),
            stream_digest(STREAM_PROTECTED_RECORDS_V1, 3, &three_records)
                .expect("one-shot protected digest")
        );
    }

    #[test]
    fn opening_negative_controls_have_ten_hard_slots_and_two_guards() {
        let controls = M2OpeningNegativeControlsV1::canonical().expect("opening controls");
        assert_eq!(controls.controls().len(), OPENING_NEGATIVE_CONTROL_COUNT_V1);
        for control in controls.controls() {
            for (slot_ordinal, slot) in control.expected_slots()[..10].iter().enumerate() {
                assert_eq!(slot[0], control.expected_status());
                assert_eq!(slot[1], control.ordinal());
                assert_eq!(slot[2], slot_ordinal as u32);
                assert!(slot[3..].iter().all(|word| *word == 0));
            }
            assert!(control.expected_slots()[10..]
                .iter()
                .flatten()
                .all(|word| *word == M2_POISON_WORD_V1));
        }
    }

    #[test]
    fn carrier_census_and_ordinals_are_frozen() {
        let carrier = M2OpeningParityCarrierV1::canonical().expect("canonical M2 carrier");
        assert_eq!(carrier.tasks().len(), M2_CONTEXT_TASK_COUNT_V1);
        assert_eq!(carrier.tasks()[0].arm(), M2CarrierArmV1::Reduced);
        assert_eq!(carrier.tasks()[64].arm(), M2CarrierArmV1::GradeMatching);
        assert_eq!(carrier.tasks()[110].arm(), M2CarrierArmV1::SameContextPair);
        assert_eq!(carrier.tasks()[613].ordinal(), 613);
        assert_ne!(carrier.task_key_stream_sha256(), [0; 32]);
        assert_eq!(
            carrier
                .tasks()
                .iter()
                .map(M2OpeningTaskV1::reduced_binding_count)
                .sum::<usize>(),
            M2_REDUCED_BINDING_COUNT_V1
        );
        assert_eq!(
            carrier
                .tasks()
                .iter()
                .map(M2OpeningTaskV1::physical_binding_count)
                .sum::<usize>(),
            M2_PHYSICAL_BINDING_COUNT_V1
        );
        let reduced_ordinals: Vec<u32> = carrier
            .tasks()
            .iter()
            .flat_map(|task| task.bindings.iter())
            .filter_map(|intent| match intent {
                BindingIntentV1::Reduced(intent) => Some(intent.binding_ordinal),
                BindingIntentV1::Physical(_) => None,
            })
            .collect();
        let physical_ordinals: Vec<u32> = carrier
            .tasks()
            .iter()
            .flat_map(|task| task.bindings.iter())
            .filter_map(|intent| match intent {
                BindingIntentV1::Reduced(_) => None,
                BindingIntentV1::Physical(intent) => Some(intent.binding_ordinal),
            })
            .collect();
        assert!(reduced_ordinals
            .iter()
            .enumerate()
            .all(|(ordinal, actual)| *actual == ordinal as u32));
        assert!(physical_ordinals
            .iter()
            .enumerate()
            .all(|(ordinal, actual)| *actual == ordinal as u32));
        for task in carrier.tasks() {
            for intent in &task.bindings {
                match intent {
                    BindingIntentV1::Reduced(intent) => {
                        validate_reduced_intent(task, intent).expect("valid reduced intent");
                    }
                    BindingIntentV1::Physical(intent) => {
                        validate_physical_intent(task, intent).expect("valid physical intent");
                    }
                }
            }
        }
    }

    #[test]
    fn global_accumulator_rejects_out_of_order_and_incomplete_runs() {
        assert!(matches!(
            M2GlobalParityAccumulatorV1::canonical()
                .expect("empty global accumulator")
                .finish(),
            Err(M2BridgeError::GlobalIncomplete {
                expected: 614,
                actual: 0
            })
        ));

        let carrier = M2OpeningParityCarrierV1::canonical().expect("comparison carrier");
        let mut accumulator = M2GlobalParityAccumulatorV1::canonical().expect("global accumulator");
        assert!(matches!(
            accumulator.accept_task_slot_words_v1(&carrier.tasks()[1], &[]),
            Err(M2BridgeError::GlobalTaskOrder {
                expected: 0,
                actual: 1
            })
        ));
        assert_eq!(accumulator.accepted_task_count(), 0);
        assert_eq!(
            accumulator.next_task().map(M2OpeningTaskV1::ordinal),
            Some(0)
        );
    }

    #[test]
    fn fixed_stream_hasher_matches_independent_one_shot_framing() {
        let records: [(u32, &[u8]); 2] = [(0, b"abc"), (1, b"defgh")];
        let mut framed = Vec::new();
        for (ordinal, payload) in records {
            push_u32(&mut framed, ordinal);
            push_u32(&mut framed, 0);
            push_u64(&mut framed, payload.len() as u64);
            framed.extend_from_slice(payload);
        }
        let mut streaming =
            FixedStreamHasherV1::new(STREAM_GLOBAL_PAYLOAD_V1, 2, framed.len() as u64);
        for (ordinal, payload) in records {
            let payload_len = payload.len() as u64;
            streaming
                .validate_record(payload_len)
                .expect("validated test record");
            streaming.append_validated_record(ordinal, payload_len, payload);
        }
        assert_eq!(
            streaming.finish().expect("complete streaming digest"),
            stream_digest(STREAM_GLOBAL_PAYLOAD_V1, 2, &framed).expect("one-shot framed digest")
        );
    }

    #[test]
    fn canonical_opening_receipt_facts_close_one_context_exactly() {
        let facts = canonical_opening_receipt_facts_v1().expect("canonical opening facts");
        assert_eq!(facts.contexts.len(), M2_CONTEXT_TASK_COUNT_V1);
        assert_eq!(
            canonical_opening_receipt_facts_v1().expect("cached canonical opening facts"),
            facts
        );

        let carrier = M2OpeningParityCarrierV1::canonical().expect("canonical carrier");
        let task = &carrier.tasks()[64];
        let rendered = task
            .render_expected_slot_words_v1()
            .expect("rendered slots");
        let mut arena =
            vec![[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]; OPENING_ARENA_SLOT_COUNT_V1];
        arena[..rendered.len()].copy_from_slice(&rendered);
        let checked = task
            .validate_slot_words_v1(&arena)
            .expect("checked canonical context");
        let fact = facts.contexts[64];
        assert_eq!(fact.task_ordinal, 64);
        assert_eq!(fact.accepted_cells, checked.cell_count());
        assert_eq!(
            fact.total_scaled_mass,
            U256Mass::from_u64(checked.total_mass())
        );
        assert_eq!(fact.canonical_payload_bytes, checked.payload_length());
        assert_eq!(fact.raw_sha256, checked.cpu_raw_sha256());
        assert_eq!(fact.payload_sha256, checked.cpu_payload_sha256());
        assert_eq!(fact.aggregate_sha256, checked.cpu_aggregate_sha256());
        let tail_bytes = slots_to_le_bytes(&arena[rendered.len()..]);
        assert_eq!(
            fact.tail_guard_sha256,
            stream_digest(
                STREAM_PROTECTED_RECORDS_V1,
                (OPENING_ARENA_SLOT_COUNT_V1 - rendered.len()) as u64,
                &tail_bytes,
            )
            .expect("independent context tail digest")
        );
    }

    #[test]
    fn global_accumulator_finishes_all_six_equal_parity_streams() {
        let mut accumulator = M2GlobalParityAccumulatorV1::canonical().expect("global accumulator");
        let mut reduced_bindings = 0usize;
        let mut physical_bindings = 0usize;
        let mut arena =
            vec![[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]; OPENING_ARENA_SLOT_COUNT_V1];

        while let Some(task) = accumulator.next_task().cloned() {
            arena.fill([M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]);
            let expected = task
                .render_expected_slot_words_v1()
                .expect("expected slots");
            arena[..expected.len()].copy_from_slice(&expected);
            let bound = accumulator
                .accept_next_task_slot_words_v1(&arena)
                .expect("accepted global task");
            reduced_bindings += bound.reduced_bindings().len();
            physical_bindings += bound.physical_bindings().len();
        }

        assert_eq!(accumulator.accepted_task_count(), 614);
        assert_eq!(reduced_bindings, M2_REDUCED_BINDING_COUNT_V1);
        assert_eq!(physical_bindings, M2_PHYSICAL_BINDING_COUNT_V1);
        let digests = accumulator.finish().expect("complete global digests");
        assert_eq!(digests.cpu_raw_sha256(), digests.gpu_raw_sha256());
        assert_eq!(digests.cpu_payload_sha256(), digests.gpu_payload_sha256());
        assert_eq!(
            digests.cpu_aggregate_sha256(),
            digests.gpu_aggregate_sha256()
        );
        assert_ne!(digests.cpu_raw_sha256(), [0; 32]);
        assert_ne!(digests.cpu_payload_sha256(), [0; 32]);
        assert_ne!(digests.cpu_aggregate_sha256(), [0; 32]);

        let facts = canonical_opening_receipt_facts_v1().expect("canonical opening receipt facts");
        assert_eq!(facts.contexts.len(), M2_CONTEXT_TASK_COUNT_V1);
        assert!(facts
            .contexts
            .iter()
            .enumerate()
            .all(|(ordinal, fact)| fact.task_ordinal == ordinal as u32));
        assert_eq!(
            facts.contexts.iter().map(|fact| fact.accepted_cells).max(),
            Some(11_730)
        );
        assert!(facts.contexts.iter().all(|fact| {
            fact.canonical_payload_bytes == 50 + u64::from(fact.accepted_cells) * 26
                && fact.total_scaled_mass != U256Mass::ZERO
                && fact.raw_sha256 != [0; 32]
                && fact.payload_sha256 != [0; 32]
                && fact.aggregate_sha256 != [0; 32]
                && fact.tail_guard_sha256 != [0; 32]
        }));
        assert_eq!(facts.global.raw_sha256, digests.cpu_raw_sha256());
        assert_eq!(facts.global.payload_sha256, digests.cpu_payload_sha256());
        assert_eq!(
            facts.global.aggregate_sha256,
            digests.cpu_aggregate_sha256()
        );
    }

    #[test]
    fn raw_slots_validate_to_unchanged_m1_payload_and_bind_once() {
        let carrier = M2OpeningParityCarrierV1::canonical().expect("canonical M2 carrier");
        let task = &carrier.tasks()[64];
        assert_eq!(task.context().grade(), 1);
        assert_eq!(task.candidate_slot_count(), 60);
        let rendered = task.render_expected_slot_words_v1().expect("render slots");
        let mut arena =
            vec![[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]; OPENING_ARENA_SLOT_COUNT_V1];
        arena[..rendered.len()].copy_from_slice(&rendered);
        let ValidatedSlotArenaV1 {
            checked,
            cpu_raw_bytes,
            gpu_raw_bytes,
            cpu_payload_bytes,
            gpu_payload_bytes,
            cpu_aggregate_bytes,
            gpu_aggregate_bytes,
        } = validate_slot_arena_detailed(task, &arena).expect("validate detailed slots");
        assert_eq!(checked.context(), task.context());
        assert_eq!(checked.candidate_slot_count(), 60);
        assert_ne!(cpu_raw_bytes.as_ptr(), gpu_raw_bytes.as_ptr());
        assert_ne!(cpu_payload_bytes.as_ptr(), gpu_payload_bytes.as_ptr());
        assert_ne!(cpu_aggregate_bytes.as_ptr(), gpu_aggregate_bytes.as_ptr());
        let raw_count = u64::from(task.candidate_slot_count());
        let aggregate_count =
            u64::try_from(cpu_aggregate_bytes.len() / 32).expect("aggregate record count");
        assert_eq!(
            checked.cpu_raw_sha256(),
            stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_count, &cpu_raw_bytes)
                .expect("CPU raw digest")
        );
        assert_eq!(
            checked.gpu_raw_sha256(),
            stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_count, &gpu_raw_bytes)
                .expect("GPU raw digest")
        );
        assert_eq!(
            checked.cpu_payload_sha256(),
            stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &cpu_payload_bytes)
                .expect("CPU payload digest")
        );
        assert_eq!(
            checked.gpu_payload_sha256(),
            stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &gpu_payload_bytes)
                .expect("GPU payload digest")
        );
        assert_eq!(
            checked.cpu_aggregate_sha256(),
            stream_digest(
                STREAM_CONTEXT_AGGREGATES_V1,
                aggregate_count,
                &cpu_aggregate_bytes,
            )
            .expect("CPU aggregate digest")
        );
        assert_eq!(
            checked.gpu_aggregate_sha256(),
            stream_digest(
                STREAM_CONTEXT_AGGREGATES_V1,
                aggregate_count,
                &gpu_aggregate_bytes,
            )
            .expect("GPU aggregate digest")
        );
        assert_eq!(checked.cpu_raw_sha256(), checked.gpu_raw_sha256());
        assert_eq!(checked.cpu_payload_sha256(), checked.gpu_payload_sha256());
        assert_eq!(
            checked.cpu_aggregate_sha256(),
            checked.gpu_aggregate_sha256()
        );

        let mut mutated_cpu_raw = cpu_raw_bytes.clone();
        mutated_cpu_raw[0] ^= 1;
        assert_ne!(
            stream_digest(STREAM_CONTEXT_SLOTS_V1, raw_count, &mutated_cpu_raw)
                .expect("mutated CPU raw digest"),
            checked.gpu_raw_sha256()
        );
        let mut mutated_cpu_payload = cpu_payload_bytes.clone();
        mutated_cpu_payload[0] ^= 1;
        assert_ne!(
            stream_digest(STREAM_CONTEXT_PAYLOAD_V1, 1, &mutated_cpu_payload)
                .expect("mutated CPU payload digest"),
            checked.gpu_payload_sha256()
        );
        let mut mutated_cpu_aggregate = cpu_aggregate_bytes.clone();
        mutated_cpu_aggregate[0] ^= 1;
        assert_ne!(
            stream_digest(
                STREAM_CONTEXT_AGGREGATES_V1,
                aggregate_count,
                &mutated_cpu_aggregate,
            )
            .expect("mutated CPU aggregate digest"),
            checked.gpu_aggregate_sha256()
        );
        assert_eq!(
            checked.canonical_payload.as_ref(),
            project_closed_form(task.context())
                .expect("scalar projection")
                .canonical_projector_payload_bytes_v1()
                .expect("scalar payload")
        );
        let bound = task
            .bind_checked_payload_v1(checked)
            .expect("bind checked payload");
        assert_eq!(bound.reduced_bindings().len(), 1);
        assert!(bound.physical_bindings().is_empty());
    }

    #[test]
    fn protected_tail_mutation_is_rejected() {
        let carrier = M2OpeningParityCarrierV1::canonical().expect("canonical M2 carrier");
        let task = &carrier.tasks()[64];
        let rendered = task.render_expected_slot_words_v1().expect("render slots");
        let mut arena =
            vec![[M2_POISON_WORD_V1; OPENING_SLOT_WORDS_V1]; OPENING_ARENA_SLOT_COUNT_V1];
        arena[..rendered.len()].copy_from_slice(&rendered);
        arena[rendered.len()][7] = 0;
        assert!(matches!(
            task.validate_slot_words_v1(&arena),
            Err(M2BridgeError::ProtectedWordMismatch { .. })
        ));
    }
}
