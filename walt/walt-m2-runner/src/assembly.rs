//! Canonical receipt assembly from move-only accepted Metal evidence.

use core::fmt;

use walt_gpu_ref::m2_receipt::{
    canonical_binding_sections, protected_chain_digest, stream_digest, ArithmeticRunKind,
    ArithmeticRunRecord, ArithmeticSection, Arm, AuthoritySection, CarrierSection, CodecError,
    ContextTaskRecord, ContextTasksSection, DeviceSection, Digest, DirectStatus, GlobalSection,
    InputHashChainRecord, PhysicalBindingRecord, PhysicalBindingsSection, ProtectedChainRecord,
    ReceiptSections, ReducedBindingRecord, ReducedBindingsSection, StreamPurpose, SuccessReceipt,
    TableRecord, TablesAndAbiSection, TaskKey, ToolchainSection, ARITHMETIC_CAPACITY,
    FREEZE56_DESCRIPTOR_SHA256, PROJECTOR_CAPACITY, ZERO_DIGEST,
};
use walt_gpu_ref::{
    canonical_opening_root_key_bytes_v1, DirectPreflightV1, M2BridgeError, M2GlobalParityDigestsV1,
    M2OpeningParityCarrierV1, OpeningChooseTableV1, PhysicalActionBindingV1,
    ReducedCarrierProfileV1, ReducedEvidenceBindingV1, M2_CONTEXT_TASK_COUNT_V1,
    M2_PHYSICAL_BINDING_COUNT_V1, M2_REDUCED_BINDING_COUNT_V1,
};
use walt_gpu_spec::{sha256, SemanticTables, TABLE_FORMAT_VERSION};
use walt_metal::{
    AcceptedMetalArithmeticNegativeV1, AcceptedMetalArithmeticV1, AcceptedMetalOpeningNegativeV1,
    AcceptedMetalOpeningTaskV1, ArithmeticRunIntegrity,
};

/// Every observation needed to render one complete success receipt.
///
/// The accepted Metal tokens are move-only.  Callers cannot replace them with
/// digests or portable byte-parity results.
pub(crate) struct OfficialEvidenceV1 {
    pub build_identity: Digest,
    pub authority: AuthoritySection,
    pub toolchain: ToolchainSection,
    pub device: DeviceSection,
    pub choose: OpeningChooseTableV1,
    pub arithmetic: AcceptedMetalArithmeticV1,
    pub arithmetic_negative: AcceptedMetalArithmeticNegativeV1,
    pub opening_negatives: Vec<AcceptedMetalOpeningNegativeV1>,
    pub openings: Vec<AcceptedMetalOpeningTaskV1>,
    pub global_parity: M2GlobalParityDigestsV1,
}

#[derive(Debug)]
pub enum AssemblyError {
    Codec(CodecError),
    Portable(M2BridgeError),
    Invariant(&'static str),
    Length(&'static str),
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(error) => write!(formatter, "receipt codec: {error}"),
            Self::Portable(error) => write!(formatter, "portable evidence: {error}"),
            Self::Invariant(label) => write!(formatter, "M2 evidence invariant: {label}"),
            Self::Length(label) => write!(formatter, "M2 length conversion: {label}"),
        }
    }
}

impl std::error::Error for AssemblyError {}

impl From<CodecError> for AssemblyError {
    fn from(error: CodecError) -> Self {
        Self::Codec(error)
    }
}

impl From<M2BridgeError> for AssemblyError {
    fn from(error: M2BridgeError) -> Self {
        Self::Portable(error)
    }
}

/// Consume accepted evidence and render one self-validating canonical receipt.
///
/// `binding_progress` receives exactly `0..1117`; `section_progress` receives
/// exactly `0..9`, after the corresponding closed record/section validates.
pub(crate) fn assemble_success_receipt<F, G>(
    evidence: OfficialEvidenceV1,
    mut binding_progress: F,
    mut section_progress: G,
) -> Result<Vec<u8>, AssemblyError>
where
    F: FnMut(u32) -> Result<(), AssemblyError>,
    G: FnMut(u32) -> Result<(), AssemblyError>,
{
    if evidence.build_identity == ZERO_DIGEST {
        return Err(AssemblyError::Invariant("zero M2 build identity"));
    }
    if evidence.opening_negatives.len() != 13 {
        return Err(AssemblyError::Invariant("opening negative census"));
    }
    if evidence.openings.len() != M2_CONTEXT_TASK_COUNT_V1 {
        return Err(AssemblyError::Invariant("accepted opening census"));
    }
    validate_arithmetic_integrity(evidence.arithmetic.integrity(), true)?;
    validate_arithmetic_integrity(evidence.arithmetic_negative.integrity(), false)?;

    let carrier = M2OpeningParityCarrierV1::canonical()?;
    let mut task_records = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1);
    let mut task_chain = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1 * 72);
    let mut choose_chain = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1 * 72);
    let mut reduced_records = Vec::with_capacity(M2_REDUCED_BINDING_COUNT_V1);
    let mut physical_records = Vec::with_capacity(M2_PHYSICAL_BINDING_COUNT_V1);

    for (ordinal, (accepted, task)) in evidence.openings.iter().zip(carrier.tasks()).enumerate() {
        let ordinal_u32 =
            u32::try_from(ordinal).map_err(|_| AssemblyError::Length("task ordinal"))?;
        let integrity = accepted.integrity();
        let bound = accepted.bound_task();
        let checked = bound.checked_payload();
        if integrity.task_ordinal != ordinal_u32
            || checked.task_ordinal() != ordinal_u32
            || task.ordinal() != ordinal_u32
            || integrity.response_count != checked.response_count()
            || integrity.response_count != task.response_count()
            || integrity.candidate_slot_count != checked.candidate_slot_count()
            || integrity.candidate_slot_count != task.candidate_slot_count()
            || integrity.valid_cell_count != checked.cell_count()
            || checked.context() != task.context()
            || bound.reduced_bindings().len() != task.reduced_binding_count()
            || bound.physical_bindings().len() != task.physical_binding_count()
        {
            return Err(AssemblyError::Invariant("accepted task metadata join"));
        }
        validate_opening_integrity(integrity, checked)?;

        let key = TaskKey::decode(&task.task_key().to_le_bytes())?;
        let direct = task.direct_preflight();
        let direct_status = match direct {
            DirectPreflightV1::Admitted { .. } => DirectStatus::Parity,
            DirectPreflightV1::DeclaredStop { .. } => DirectStatus::DeclaredStop,
        };
        let total_mass = checked.total_mass();
        let mut mass_limbs = [0u32; 8];
        mass_limbs[0] = total_mass as u32;
        mass_limbs[1] = (total_mass >> 32) as u32;
        let task_record = ContextTaskRecord {
            key,
            direct_status,
            direct_world_count: direct.world_count(),
            accepted_cells: checked.cell_count(),
            in_range_slot_bytes: u64::from(checked.candidate_slot_count()) * 64,
            canonical_payload_bytes: checked.payload_length(),
            total_scaled_mass: mass_limbs,
            cpu_slot_digest: checked.cpu_raw_sha256(),
            gpu_slot_digest: checked.gpu_raw_sha256(),
            cpu_payload_digest: checked.cpu_payload_sha256(),
            gpu_payload_digest: checked.gpu_payload_sha256(),
            cpu_aggregate_digest: checked.cpu_aggregate_sha256(),
            gpu_aggregate_digest: checked.gpu_aggregate_sha256(),
            tail_guard_digest: integrity.protected_post_digest,
        };
        task_record.validate()?;
        task_records.push(task_record);

        task_chain.extend_from_slice(
            &InputHashChainRecord {
                task_ordinal: ordinal_u32,
                pre_digest: integrity.task_pre_digest,
                post_digest: integrity.task_post_digest,
            }
            .encode(),
        );
        choose_chain.extend_from_slice(
            &InputHashChainRecord {
                task_ordinal: ordinal_u32,
                pre_digest: integrity.choose_pre_digest,
                post_digest: integrity.choose_post_digest,
            }
            .encode(),
        );

        for binding in bound.reduced_bindings() {
            let record = reduced_binding_record(binding)?;
            record.validate_shape()?;
            let ordinal = u32::try_from(reduced_records.len())
                .map_err(|_| AssemblyError::Length("reduced ordinal"))?;
            if record.binding_ordinal != ordinal {
                return Err(AssemblyError::Invariant("reduced binding order"));
            }
            reduced_records.push(record);
        }
        for binding in bound.physical_bindings() {
            let record = physical_binding_record(binding)?;
            record.validate_shape()?;
            let ordinal = u32::try_from(physical_records.len())
                .map_err(|_| AssemblyError::Length("physical binding ordinal"))?;
            if record.binding_ordinal != ordinal {
                return Err(AssemblyError::Invariant("physical binding order"));
            }
            physical_records.push(record);
        }
    }

    if reduced_records.len() != M2_REDUCED_BINDING_COUNT_V1
        || physical_records.len() != M2_PHYSICAL_BINDING_COUNT_V1
    {
        return Err(AssemblyError::Invariant("binding census"));
    }
    let semantic_table_bytes = SemanticTables::from_walt_core().canonical_bytes();
    let semantic_table = TableRecord {
        tag: 1,
        format_version: u32::from(TABLE_FORMAT_VERSION),
        rows: 0,
        columns: 0,
        byte_length: u64::try_from(semantic_table_bytes.len())
            .map_err(|_| AssemblyError::Length("semantic table bytes"))?,
        digest: sha256(&semantic_table_bytes),
    };
    let choose_bytes = words_to_le_bytes(evidence.choose.words());
    let choose_table = TableRecord {
        tag: 2,
        format_version: 1,
        rows: 22,
        columns: 22,
        byte_length: u64::try_from(choose_bytes.len())
            .map_err(|_| AssemblyError::Length("choose table bytes"))?,
        digest: sha256(&choose_bytes),
    };

    let arithmetic_official =
        arithmetic_record(ArithmeticRunKind::Official, evidence.arithmetic.integrity());
    let arithmetic_negative = arithmetic_record(
        ArithmeticRunKind::Negative,
        evidence.arithmetic_negative.integrity(),
    );

    let context_tasks = ContextTasksSection {
        records: task_records,
    };
    let (canonical_reduced, canonical_physical) = canonical_binding_sections(&context_tasks)?;
    if canonical_reduced.records != reduced_records
        || canonical_physical.records != physical_records
    {
        return Err(AssemblyError::Invariant(
            "accepted binding tokens differ from canonical carrier",
        ));
    }
    for index in 0..M2_REDUCED_BINDING_COUNT_V1 {
        binding_progress(
            u32::try_from(index).map_err(|_| AssemblyError::Length("reduced progress"))?,
        )?;
    }
    for index in 0..M2_PHYSICAL_BINDING_COUNT_V1 {
        let unit = M2_REDUCED_BINDING_COUNT_V1
            .checked_add(index)
            .ok_or(AssemblyError::Length("physical progress"))?;
        binding_progress(u32::try_from(unit).map_err(|_| AssemblyError::Length("physical unit"))?)?;
    }
    let accepted_payload_bytes = context_tasks.accepted_payload_bytes()?;
    let protected = protected_records(
        evidence.arithmetic.integrity(),
        evidence.arithmetic_negative.integrity(),
        &evidence.opening_negatives,
        &evidence.openings,
    )?;
    let protected_digest = protected_chain_digest(&protected)?;

    let mut receipt = SuccessReceipt {
        build_identity: evidence.build_identity,
        freeze56_descriptor_digest: FREEZE56_DESCRIPTOR_SHA256,
        sections: ReceiptSections {
            authority: evidence.authority,
            toolchain: evidence.toolchain,
            device: evidence.device,
            tables_and_abi: TablesAndAbiSection {
                semantic_table,
                choose_table,
            },
            arithmetic: ArithmeticSection {
                official: arithmetic_official,
                negative: arithmetic_negative,
            },
            carrier: CarrierSection {
                accepted_payload_bytes,
                task_key_stream_digest: carrier.task_key_stream_sha256(),
                task_input_hash_chain_digest: stream_digest(
                    StreamPurpose::TaskInputHashChain,
                    614,
                    &task_chain,
                )?,
                choose_input_hash_chain_digest: stream_digest(
                    StreamPurpose::ChooseInputHashChain,
                    614,
                    &choose_chain,
                )?,
            },
            context_tasks,
            reduced_bindings: ReducedBindingsSection {
                records: reduced_records,
            },
            physical_bindings: PhysicalBindingsSection {
                records: physical_records,
            },
            global: GlobalSection {
                digests: [
                    evidence.global_parity.cpu_raw_sha256(),
                    evidence.global_parity.gpu_raw_sha256(),
                    evidence.global_parity.cpu_payload_sha256(),
                    evidence.global_parity.gpu_payload_sha256(),
                    evidence.global_parity.cpu_aggregate_sha256(),
                    evidence.global_parity.gpu_aggregate_sha256(),
                    protected_digest,
                    ZERO_DIGEST,
                    ZERO_DIGEST,
                    ZERO_DIGEST,
                ],
            },
        },
    };
    receipt.canonicalize()?;
    receipt.validate()?;

    // Validate each exact section before publishing its progress unit.
    receipt.sections.authority.encode()?;
    section_progress(0)?;
    receipt.sections.toolchain.encode()?;
    section_progress(1)?;
    receipt.sections.device.encode()?;
    section_progress(2)?;
    receipt.sections.tables_and_abi.encode()?;
    section_progress(3)?;
    receipt.sections.arithmetic.encode()?;
    section_progress(4)?;
    receipt.sections.carrier.encode()?;
    section_progress(5)?;
    receipt.sections.context_tasks.encode()?;
    section_progress(6)?;
    receipt.sections.reduced_bindings.encode()?;
    section_progress(7)?;
    receipt.sections.physical_bindings.encode()?;
    section_progress(8)?;
    receipt.sections.global.encode()?;
    section_progress(9)?;

    let bytes = receipt.encode()?;
    let decoded = SuccessReceipt::decode(&bytes)?;
    if decoded != receipt {
        return Err(AssemblyError::Invariant("receipt encode/decode identity"));
    }
    Ok(bytes)
}

fn arithmetic_record(
    kind: ArithmeticRunKind,
    value: &ArithmeticRunIntegrity,
) -> ArithmeticRunRecord {
    ArithmeticRunRecord {
        kind,
        case_count: value.case_count,
        accepted_count: value.accepted_count,
        input_payload_bytes: u64::from(value.case_count) * 80,
        output_payload_bytes: u64::from(value.case_count) * 64,
        allocated_input_bytes: value.allocated_input_bytes,
        allocated_output_bytes: value.allocated_output_bytes,
        success_count: value.success_count,
        checked_undefined_count: value.checked_undefined_count,
        hard_count: value.hard_count,
        input_pre_digest: value.input_pre_digest,
        input_post_digest: value.input_post_digest,
        cpu_output_digest: value.cpu_output_digest,
        gpu_output_digest: value.gpu_output_digest,
        guard_pre_digest: value.guard_pre_digest,
        guard_post_digest: value.guard_post_digest,
    }
}

fn reduced_binding_record(
    binding: &ReducedEvidenceBindingV1,
) -> Result<ReducedBindingRecord, AssemblyError> {
    let arm = match binding.profile() {
        ReducedCarrierProfileV1::ReducedArm => Arm::Reduced,
        ReducedCarrierProfileV1::GradeMatching => Arm::GradeMatching,
    };
    Ok(ReducedBindingRecord {
        binding_ordinal: binding.binding_ordinal(),
        task_ordinal: binding.task_ordinal(),
        arm,
        arm_ordinal: binding.arm_ordinal(),
        root_key: canonical_opening_root_key_bytes_v1(binding.root())
            .map_err(|_| AssemblyError::Invariant("reduced root key"))?,
        selected_action: u8::try_from(binding.selected_action().index())
            .map_err(|_| AssemblyError::Length("reduced selected action"))?,
        derived_context: u8::try_from(binding.context().led().index())
            .map_err(|_| AssemblyError::Length("reduced context"))?,
        grade: binding.context().grade(),
        matching_count: binding.matching_count(),
        reduced_pool_mask: binding.context().pool().bits(),
        payload_bytes: binding.payload_length(),
        payload_digest: binding.payload_sha256(),
        semantic_identity: ZERO_DIGEST,
    })
}

fn physical_binding_record(
    binding: &PhysicalActionBindingV1,
) -> Result<PhysicalBindingRecord, AssemblyError> {
    let arm = match binding.arm().code() {
        2 => Arm::GradeMatching,
        3 => Arm::SameContextPair,
        _ => return Err(AssemblyError::Invariant("physical binding arm")),
    };
    Ok(PhysicalBindingRecord {
        binding_ordinal: binding.binding_ordinal(),
        task_ordinal: binding.task_ordinal(),
        arm,
        arm_ordinal: binding.arm_ordinal(),
        endpoint: binding.endpoint(),
        root_key: canonical_opening_root_key_bytes_v1(binding.root())
            .map_err(|_| AssemblyError::Invariant("physical root key"))?,
        selected_action: u8::try_from(binding.selected_action().index())
            .map_err(|_| AssemblyError::Length("physical selected action"))?,
        derived_context: u8::try_from(binding.context().led().index())
            .map_err(|_| AssemblyError::Length("physical context"))?,
        context_pool_mask: binding.context().pool().bits(),
        payload_bytes: binding.payload_length(),
        payload_digest: binding.payload_sha256(),
        semantic_identity: ZERO_DIGEST,
    })
}

fn protected_records(
    official: &ArithmeticRunIntegrity,
    arithmetic_negative: &ArithmeticRunIntegrity,
    opening_negatives: &[AcceptedMetalOpeningNegativeV1],
    openings: &[AcceptedMetalOpeningTaskV1],
) -> Result<Vec<ProtectedChainRecord>, AssemblyError> {
    validate_arithmetic_integrity(official, true)?;
    validate_arithmetic_integrity(arithmetic_negative, false)?;
    let mut records = Vec::with_capacity(629);
    records.push(ProtectedChainRecord {
        domain: 1,
        ordinal: 0,
        first_protected_record: 16_384,
        protected_count: 2,
        digest: official.guard_post_digest,
    });
    records.push(ProtectedChainRecord {
        domain: 2,
        ordinal: 0,
        first_protected_record: 13,
        protected_count: 2,
        digest: arithmetic_negative.guard_post_digest,
    });
    for (ordinal, accepted_negative) in opening_negatives.iter().enumerate() {
        let negative = accepted_negative.integrity();
        let ordinal_u32 = u32::try_from(ordinal)
            .map_err(|_| AssemblyError::Length("opening negative ordinal"))?;
        if negative.ordinal != ordinal_u32 {
            return Err(AssemblyError::Invariant("opening negative order"));
        }
        if negative.task_pre_digest != negative.task_post_digest
            || negative.choose_pre_digest != negative.choose_post_digest
        {
            return Err(AssemblyError::Invariant(
                "opening negative immutable input digest",
            ));
        }
        records.push(ProtectedChainRecord {
            domain: 3,
            ordinal: ordinal_u32,
            first_protected_record: 0,
            protected_count: 12,
            digest: negative.protected_post_digest,
        });
    }
    for (ordinal, accepted) in openings.iter().enumerate() {
        let ordinal_u32 =
            u32::try_from(ordinal).map_err(|_| AssemblyError::Length("protected task ordinal"))?;
        let integrity = accepted.integrity();
        if integrity.task_ordinal != ordinal_u32 {
            return Err(AssemblyError::Invariant("protected task order"));
        }
        records.push(ProtectedChainRecord {
            domain: 4,
            ordinal: ordinal_u32,
            first_protected_record: integrity.candidate_slot_count,
            protected_count: 79_802u32
                .checked_sub(integrity.candidate_slot_count)
                .ok_or(AssemblyError::Invariant("protected task range"))?,
            digest: integrity.protected_post_digest,
        });
    }
    if records.len() != 629 {
        return Err(AssemblyError::Invariant("protected chain census"));
    }
    Ok(records)
}

fn validate_arithmetic_integrity(
    value: &ArithmeticRunIntegrity,
    official: bool,
) -> Result<(), AssemblyError> {
    if value.input_pre_digest != value.input_post_digest
        || value.cpu_output_digest != value.gpu_output_digest
        || value.guard_pre_digest != value.guard_post_digest
    {
        return Err(AssemblyError::Invariant(
            "arithmetic immutable, parity, or guard digest",
        ));
    }
    let allocated = value
        .allocated_input_bytes
        .checked_add(value.allocated_output_bytes)
        .ok_or(AssemblyError::Length("arithmetic allocated high-water"))?;
    if official && allocated < ARITHMETIC_CAPACITY {
        return Err(AssemblyError::Invariant(
            "official arithmetic allocation high-water",
        ));
    }
    Ok(())
}

fn validate_opening_integrity(
    integrity: &walt_metal::OpeningRunIntegrity,
    checked: &walt_gpu_ref::CheckedM2ProjectionPayloadV1,
) -> Result<(), AssemblyError> {
    if integrity.task_pre_digest != integrity.task_post_digest
        || integrity.choose_pre_digest != integrity.choose_post_digest
        || integrity.protected_pre_digest != integrity.protected_post_digest
        || integrity.defensive_cpu_slot_digest != checked.cpu_raw_sha256()
        || integrity.gpu_slot_digest != checked.gpu_raw_sha256()
    {
        return Err(AssemblyError::Invariant(
            "opening immutable, protected, or raw digest join",
        ));
    }
    let allocated = integrity
        .allocated_task_bytes
        .checked_add(integrity.allocated_choose_bytes)
        .and_then(|value| value.checked_add(integrity.allocated_output_bytes))
        .ok_or(AssemblyError::Length("opening allocated high-water"))?;
    if allocated < PROJECTOR_CAPACITY {
        return Err(AssemblyError::Invariant("opening allocation high-water"));
    }
    Ok(())
}

fn words_to_le_bytes(words: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for word in words {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes
}
