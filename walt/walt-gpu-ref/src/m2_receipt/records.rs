use super::wire::{
    put_digest, put_u16, put_u32, put_u64, require_eq, require_zero, validate_path, validate_text,
    ArithmeticRunKind, Arm, ArtifactTag, CodecError, Digest, DirectStatus, InvocationKind,
    KernelId, Reader, Result, SectionTag, SourceKind, ToolId, POISON_WORD,
};

pub const DIRECTORY_ENTRY_BYTES: usize = 64;
pub const ARTIFACT_IDENTITY_BYTES: usize = 48;
pub const TOOL_RECORD_BYTES: usize = 48;
pub const PIPELINE_RECORD_BYTES: usize = 24;
pub const TABLE_RECORD_BYTES: usize = 56;
pub const ARITHMETIC_RUN_RECORD_BYTES: usize = 288;
pub const TASK_KEY_BYTES: usize = 64;
pub const INPUT_HASH_CHAIN_RECORD_BYTES: usize = 72;
pub const CONTEXT_TASK_RECORD_BYTES: usize = 384;
pub const REDUCED_BINDING_RECORD_BYTES: usize = 160;
pub const PHYSICAL_BINDING_RECORD_BYTES: usize = 160;
pub const IDENTITY_STREAM_RECORD_BYTES: usize = 40;
pub const PROTECTED_CHAIN_RECORD_BYTES: usize = 48;
pub const RESPONSE_AGGREGATE_RECORD_BYTES: usize = 32;
pub const TASK_FRAMED_RECORD_PREFIX_BYTES: usize = 16;

pub const COMPLETION_CLASS_COMPLETED: u32 = 1;
pub const NATIVE_STATUS_COMPLETED: u32 = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionDirectoryEntry {
    pub tag: SectionTag,
    pub offset: u64,
    pub length: u64,
    pub record_count: u64,
    pub digest: Digest,
}

impl SectionDirectoryEntry {
    pub fn encode(&self) -> [u8; DIRECTORY_ENTRY_BYTES] {
        let mut out = Vec::with_capacity(DIRECTORY_ENTRY_BYTES);
        put_u16(&mut out, self.tag.into());
        put_u16(&mut out, 1);
        put_u32(&mut out, 0);
        put_u64(&mut out, self.offset);
        put_u64(&mut out, self.length);
        put_u64(&mut out, self.record_count);
        put_digest(&mut out, &self.digest);
        out.try_into().expect("directory entry width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let tag = SectionTag::try_from(reader.u16()?)?;
        require_eq(&reader.u16()?, &1, "directory version")?;
        require_eq(&reader.u32()?, &0, "directory flags")?;
        Ok(Self {
            tag,
            offset: reader.u64()?,
            length: reader.u64()?,
            record_count: reader.u64()?,
            digest: reader.digest()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIdentity {
    pub tag: ArtifactTag,
    pub byte_length: u64,
    pub digest: Digest,
}

impl ArtifactIdentity {
    pub fn encode(&self) -> [u8; ARTIFACT_IDENTITY_BYTES] {
        let mut out = Vec::with_capacity(ARTIFACT_IDENTITY_BYTES);
        put_u32(&mut out, self.tag.into());
        put_u32(&mut out, 1);
        put_u64(&mut out, self.byte_length);
        put_digest(&mut out, &self.digest);
        out.try_into().expect("artifact identity width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let tag = ArtifactTag::try_from(reader.u32()?)?;
        require_eq(&reader.u32()?, &1, "artifact hash kind")?;
        Ok(Self {
            tag,
            byte_length: reader.u64()?,
            digest: reader.digest()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageRecord {
    pub name: String,
    pub version: String,
    pub checksum: Digest,
    pub default_feature: bool,
    pub activated_features: Vec<String>,
}

impl PackageRecord {
    pub fn encode(&self, out: &mut Vec<u8>) -> Result<()> {
        self.validate()?;
        super::wire::put_text(out, &self.name)?;
        super::wire::put_text(out, &self.version)?;
        put_digest(out, &self.checksum);
        put_u32(out, u32::from(self.default_feature));
        put_u32(
            out,
            u32::try_from(self.activated_features.len())
                .map_err(|_| CodecError::LengthOverflow("package feature count"))?,
        );
        for feature in &self.activated_features {
            super::wire::put_text(out, feature)?;
        }
        Ok(())
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let name = reader.text()?;
        let version = reader.text()?;
        let checksum = reader.digest()?;
        let default_feature = match reader.u32()? {
            0 => false,
            1 => true,
            _ => return Err(CodecError::Invalid("package default-feature")),
        };
        let count = usize::try_from(reader.u32()?)
            .map_err(|_| CodecError::LengthOverflow("feature count usize"))?;
        if count > reader.remaining() / 4 {
            return Err(CodecError::Invalid("package feature count"));
        }
        let mut activated_features = Vec::with_capacity(count);
        for _ in 0..count {
            activated_features.push(reader.text()?);
        }
        let value = Self {
            name,
            version,
            checksum,
            default_feature,
            activated_features,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        validate_text(&self.name)?;
        validate_text(&self.version)?;
        if self.name.is_empty() || self.version.is_empty() {
            return Err(CodecError::Invalid("package name/version"));
        }
        let mut previous: Option<&str> = None;
        for feature in &self.activated_features {
            validate_text(feature)?;
            if feature.is_empty() || previous.is_some_and(|value| value >= feature.as_str()) {
                return Err(CodecError::Invalid("sorted unique package features"));
            }
            previous = Some(feature);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolRecord {
    pub id: ToolId,
    pub executable_bytes: u64,
    pub digest: Digest,
}

impl ToolRecord {
    pub fn encode(&self) -> [u8; TOOL_RECORD_BYTES] {
        let mut out = Vec::with_capacity(TOOL_RECORD_BYTES);
        put_u32(&mut out, self.id.into());
        put_u32(&mut out, 0);
        put_u64(&mut out, self.executable_bytes);
        put_digest(&mut out, &self.digest);
        out.try_into().expect("tool record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let id = ToolId::try_from(reader.u32()?)?;
        require_eq(&reader.u32()?, &0, "tool reserved")?;
        Ok(Self {
            id,
            executable_bytes: reader.u64()?,
            digest: reader.digest()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceRecord {
    pub kind: SourceKind,
    pub byte_length: u64,
    pub digest: Digest,
    pub path: String,
}

impl SourceRecord {
    pub fn encode(&self, out: &mut Vec<u8>) -> Result<()> {
        validate_path(&self.path)?;
        put_u32(out, self.kind.into());
        put_u32(out, 0);
        put_u64(out, self.byte_length);
        put_digest(out, &self.digest);
        super::wire::put_text(out, &self.path)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let kind = SourceKind::try_from(reader.u32()?)?;
        require_eq(&reader.u32()?, &0, "source reserved")?;
        let value = Self {
            kind,
            byte_length: reader.u64()?,
            digest: reader.digest()?,
            path: reader.text()?,
        };
        validate_path(&value.path)?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationRecord {
    pub kind: InvocationKind,
    pub source_index: u32,
    pub arguments: Vec<String>,
}

impl InvocationRecord {
    pub fn encode(&self, out: &mut Vec<u8>) -> Result<()> {
        put_u32(out, self.kind.into());
        put_u32(out, self.source_index);
        put_u32(
            out,
            u32::try_from(self.arguments.len())
                .map_err(|_| CodecError::LengthOverflow("invocation argument count"))?,
        );
        put_u32(out, 0);
        for argument in &self.arguments {
            super::wire::put_text(out, argument)?;
        }
        Ok(())
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let kind = InvocationKind::try_from(reader.u32()?)?;
        let source_index = reader.u32()?;
        let count = usize::try_from(reader.u32()?)
            .map_err(|_| CodecError::LengthOverflow("argument count usize"))?;
        require_eq(&reader.u32()?, &0, "invocation reserved")?;
        if count > reader.remaining() / 4 {
            return Err(CodecError::Invalid("invocation argument count"));
        }
        let mut arguments = Vec::with_capacity(count);
        for _ in 0..count {
            arguments.push(reader.text()?);
        }
        Ok(Self {
            kind,
            source_index,
            arguments,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineRecord {
    pub kernel: KernelId,
    pub execution_width: u32,
    pub maximum_threads: u32,
    pub static_group_memory: u64,
}

impl PipelineRecord {
    pub fn encode(&self) -> [u8; PIPELINE_RECORD_BYTES] {
        let mut out = Vec::with_capacity(PIPELINE_RECORD_BYTES);
        put_u32(&mut out, self.kernel.into());
        put_u32(&mut out, self.execution_width);
        put_u32(&mut out, self.maximum_threads);
        put_u32(&mut out, 0);
        put_u64(&mut out, self.static_group_memory);
        out.try_into().expect("pipeline record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let kernel = KernelId::try_from(reader.u32()?)?;
        let execution_width = reader.u32()?;
        let maximum_threads = reader.u32()?;
        require_eq(&reader.u32()?, &0, "pipeline reserved")?;
        Ok(Self {
            kernel,
            execution_width,
            maximum_threads,
            static_group_memory: reader.u64()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableRecord {
    pub tag: u32,
    pub format_version: u32,
    pub rows: u32,
    pub columns: u32,
    pub byte_length: u64,
    pub digest: Digest,
}

impl TableRecord {
    pub fn encode(&self) -> [u8; TABLE_RECORD_BYTES] {
        let mut out = Vec::with_capacity(TABLE_RECORD_BYTES);
        put_u32(&mut out, self.tag);
        put_u32(&mut out, self.format_version);
        put_u32(&mut out, self.rows);
        put_u32(&mut out, self.columns);
        put_u64(&mut out, self.byte_length);
        put_digest(&mut out, &self.digest);
        out.try_into().expect("table record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        Ok(Self {
            tag: reader.u32()?,
            format_version: reader.u32()?,
            rows: reader.u32()?,
            columns: reader.u32()?,
            byte_length: reader.u64()?,
            digest: reader.digest()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArithmeticRunRecord {
    pub kind: ArithmeticRunKind,
    pub case_count: u32,
    pub accepted_count: u32,
    pub input_payload_bytes: u64,
    pub output_payload_bytes: u64,
    pub allocated_input_bytes: u64,
    pub allocated_output_bytes: u64,
    pub success_count: u32,
    pub checked_undefined_count: u32,
    pub hard_count: u32,
    pub input_pre_digest: Digest,
    pub input_post_digest: Digest,
    pub cpu_output_digest: Digest,
    pub gpu_output_digest: Digest,
    pub guard_pre_digest: Digest,
    pub guard_post_digest: Digest,
}

impl ArithmeticRunRecord {
    pub fn encode(&self) -> [u8; ARITHMETIC_RUN_RECORD_BYTES] {
        let mut out = Vec::with_capacity(ARITHMETIC_RUN_RECORD_BYTES);
        put_u32(&mut out, self.kind.into());
        put_u32(&mut out, 1);
        put_u32(&mut out, self.case_count);
        put_u32(&mut out, self.accepted_count);
        put_u32(&mut out, 80);
        put_u32(&mut out, 64);
        put_u32(&mut out, 2);
        put_u32(&mut out, POISON_WORD);
        put_u64(&mut out, self.input_payload_bytes);
        put_u64(&mut out, self.output_payload_bytes);
        put_u64(&mut out, self.allocated_input_bytes);
        put_u64(&mut out, self.allocated_output_bytes);
        put_u32(&mut out, COMPLETION_CLASS_COMPLETED);
        put_u32(&mut out, NATIVE_STATUS_COMPLETED);
        put_u32(&mut out, self.success_count);
        put_u32(&mut out, self.checked_undefined_count);
        put_u32(&mut out, self.hard_count);
        put_u32(&mut out, 0x3f);
        put_digest(&mut out, &self.input_pre_digest);
        put_digest(&mut out, &self.input_post_digest);
        put_digest(&mut out, &self.cpu_output_digest);
        put_digest(&mut out, &self.gpu_output_digest);
        put_digest(&mut out, &self.guard_pre_digest);
        put_digest(&mut out, &self.guard_post_digest);
        out.extend_from_slice(&[0; 8]);
        out.try_into().expect("arithmetic run record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let kind = ArithmeticRunKind::try_from(reader.u32()?)?;
        require_eq(&reader.u32()?, &1, "arithmetic profile")?;
        let case_count = reader.u32()?;
        let accepted_count = reader.u32()?;
        require_eq(&reader.u32()?, &80, "arithmetic input record bytes")?;
        require_eq(&reader.u32()?, &64, "arithmetic output record bytes")?;
        require_eq(&reader.u32()?, &2, "arithmetic guard count")?;
        require_eq(&reader.u32()?, &POISON_WORD, "arithmetic poison")?;
        let value = Self {
            kind,
            case_count,
            accepted_count,
            input_payload_bytes: reader.u64()?,
            output_payload_bytes: reader.u64()?,
            allocated_input_bytes: reader.u64()?,
            allocated_output_bytes: reader.u64()?,
            success_count: {
                require_eq(
                    &reader.u32()?,
                    &COMPLETION_CLASS_COMPLETED,
                    "arithmetic completion class",
                )?;
                require_eq(
                    &reader.u32()?,
                    &NATIVE_STATUS_COMPLETED,
                    "arithmetic native status",
                )?;
                reader.u32()?
            },
            checked_undefined_count: reader.u32()?,
            hard_count: reader.u32()?,
            input_pre_digest: {
                require_eq(&reader.u32()?, &0x3f, "arithmetic validation flags")?;
                reader.digest()?
            },
            input_post_digest: reader.digest()?,
            cpu_output_digest: reader.digest()?,
            gpu_output_digest: reader.digest()?,
            guard_pre_digest: reader.digest()?,
            guard_post_digest: reader.digest()?,
        };
        require_zero(reader.bytes(8)?, "arithmetic reserved")?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(
            &self.input_payload_bytes,
            &(u64::from(self.case_count) * 80),
            "arithmetic input payload length",
        )?;
        require_eq(
            &self.output_payload_bytes,
            &(u64::from(self.case_count) * 64),
            "arithmetic output payload length",
        )?;
        if self.allocated_input_bytes < self.input_payload_bytes
            || self.allocated_output_bytes < self.output_payload_bytes + 128
        {
            return Err(CodecError::Invalid("arithmetic allocated length"));
        }
        require_eq(
            &self.input_pre_digest,
            &self.input_post_digest,
            "arithmetic immutable input digest",
        )?;
        require_eq(
            &self.cpu_output_digest,
            &self.gpu_output_digest,
            "arithmetic output parity digest",
        )?;
        require_eq(
            &self.guard_pre_digest,
            &self.guard_post_digest,
            "arithmetic guard digest",
        )?;
        match self.kind {
            ArithmeticRunKind::Official => {
                require_eq(&self.case_count, &16_384, "official arithmetic cases")?;
                require_eq(
                    &self.accepted_count,
                    &16_384,
                    "official arithmetic accepted",
                )?;
                require_eq(&self.hard_count, &0, "official arithmetic hard count")?;
                let classified = self
                    .success_count
                    .checked_add(self.checked_undefined_count)
                    .ok_or(CodecError::LengthOverflow(
                        "official arithmetic status census",
                    ))?;
                require_eq(&classified, &16_384, "official arithmetic status census")?;
            }
            ArithmeticRunKind::Negative => {
                require_eq(&self.case_count, &13, "negative arithmetic cases")?;
                require_eq(&self.accepted_count, &0, "negative arithmetic accepted")?;
                require_eq(&self.success_count, &0, "negative arithmetic success")?;
                require_eq(
                    &self.checked_undefined_count,
                    &0,
                    "negative arithmetic undefined",
                )?;
                require_eq(&self.hard_count, &13, "negative arithmetic hard count")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskKey {
    pub task_ordinal: u32,
    pub arm: Arm,
    pub arm_ordinal: u32,
    pub declaration: u32,
    pub led: u32,
    pub grade: u32,
    pub pool_mask: u32,
    pub matching_mask: u32,
    pub pool_count: u32,
    pub response_triple_count: u32,
    pub candidate_slot_count: u32,
    pub generator_a: u32,
    pub generator_b: u32,
    pub generator_c: u32,
}

impl TaskKey {
    pub fn encode(&self) -> [u8; TASK_KEY_BYTES] {
        let mut out = Vec::with_capacity(TASK_KEY_BYTES);
        for word in self.words() {
            put_u32(&mut out, word);
        }
        out.try_into().expect("task key width")
    }

    pub fn words(&self) -> [u32; 16] {
        [
            1,
            self.task_ordinal,
            self.arm.into(),
            self.arm_ordinal,
            self.declaration,
            self.led,
            self.grade,
            self.pool_mask,
            self.matching_mask,
            self.pool_count,
            self.response_triple_count,
            self.candidate_slot_count,
            self.generator_a,
            self.generator_b,
            self.generator_c,
            0,
        ]
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        require_eq(&reader.u32()?, &1, "task key version")?;
        let value = Self {
            task_ordinal: reader.u32()?,
            arm: Arm::try_from(reader.u32()?)?,
            arm_ordinal: reader.u32()?,
            declaration: reader.u32()?,
            led: reader.u32()?,
            grade: reader.u32()?,
            pool_mask: reader.u32()?,
            matching_mask: reader.u32()?,
            pool_count: reader.u32()?,
            response_triple_count: reader.u32()?,
            candidate_slot_count: reader.u32()?,
            generator_a: reader.u32()?,
            generator_b: reader.u32()?,
            generator_c: reader.u32()?,
        };
        require_eq(&reader.u32()?, &0, "task key reserved")?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        if self.declaration > 8 || self.led > 7 || !(1..=7).contains(&self.grade) {
            return Err(CodecError::Invalid("task key declaration/led/grade"));
        }
        if self.pool_mask >> 28 != 0
            || self.matching_mask >> 28 != 0
            || self.matching_mask & !self.pool_mask != 0
            || self.matching_mask.count_ones() > 6
        {
            return Err(CodecError::Invalid("task key masks"));
        }
        require_eq(
            &self.pool_count,
            &self.pool_mask.count_ones(),
            "task pool popcount",
        )?;
        require_eq(
            &self.pool_count,
            &(3 * self.grade),
            "task pool grade cardinality",
        )?;
        let response_count = self
            .pool_count
            .checked_mul(self.pool_count.saturating_sub(1))
            .and_then(|value| value.checked_mul(self.pool_count.saturating_sub(2)))
            .ok_or(CodecError::Invalid("task response count overflow"))?;
        require_eq(
            &self.response_triple_count,
            &response_count,
            "task response count",
        )?;
        require_eq(
            &self.candidate_slot_count,
            &response_count
                .checked_mul(10)
                .ok_or(CodecError::Invalid("task slot count overflow"))?,
            "task slot count",
        )?;
        if self.candidate_slot_count > 79_800 {
            return Err(CodecError::Invalid("task slot cap"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputHashChainRecord {
    pub task_ordinal: u32,
    pub pre_digest: Digest,
    pub post_digest: Digest,
}

impl InputHashChainRecord {
    pub fn encode(&self) -> [u8; INPUT_HASH_CHAIN_RECORD_BYTES] {
        let mut out = Vec::with_capacity(INPUT_HASH_CHAIN_RECORD_BYTES);
        put_u32(&mut out, self.task_ordinal);
        put_u32(&mut out, 0);
        put_digest(&mut out, &self.pre_digest);
        put_digest(&mut out, &self.post_digest);
        out.try_into().expect("input hash chain width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        require_eq(
            &value.pre_digest,
            &value.post_digest,
            "input hash chain mutation",
        )?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let task_ordinal = reader.u32()?;
        require_eq(&reader.u32()?, &0, "input hash chain reserved")?;
        Ok(Self {
            task_ordinal,
            pre_digest: reader.digest()?,
            post_digest: reader.digest()?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextTaskRecord {
    pub key: TaskKey,
    pub direct_status: DirectStatus,
    pub direct_world_count: u64,
    pub accepted_cells: u32,
    pub in_range_slot_bytes: u64,
    pub canonical_payload_bytes: u64,
    pub total_scaled_mass: [u32; 8],
    pub cpu_slot_digest: Digest,
    pub gpu_slot_digest: Digest,
    pub cpu_payload_digest: Digest,
    pub gpu_payload_digest: Digest,
    pub cpu_aggregate_digest: Digest,
    pub gpu_aggregate_digest: Digest,
    pub tail_guard_digest: Digest,
}

impl ContextTaskRecord {
    pub fn encode(&self) -> [u8; CONTEXT_TASK_RECORD_BYTES] {
        let mut out = Vec::with_capacity(CONTEXT_TASK_RECORD_BYTES);
        out.extend_from_slice(&self.key.encode());
        put_u32(&mut out, 1);
        put_u32(&mut out, self.direct_status.into());
        put_u64(&mut out, self.direct_world_count);
        put_u64(&mut out, 100_000);
        put_u32(&mut out, self.accepted_cells);
        put_u32(&mut out, 11_730);
        put_u64(&mut out, self.in_range_slot_bytes);
        put_u64(&mut out, self.canonical_payload_bytes);
        for limb in self.total_scaled_mass {
            put_u32(&mut out, limb);
        }
        put_u32(&mut out, COMPLETION_CLASS_COMPLETED);
        put_u32(&mut out, NATIVE_STATUS_COMPLETED);
        put_u32(&mut out, 0x3ff);
        put_u32(&mut out, 0);
        put_digest(&mut out, &self.cpu_slot_digest);
        put_digest(&mut out, &self.gpu_slot_digest);
        put_digest(&mut out, &self.cpu_payload_digest);
        put_digest(&mut out, &self.gpu_payload_digest);
        put_digest(&mut out, &self.cpu_aggregate_digest);
        put_digest(&mut out, &self.gpu_aggregate_digest);
        put_digest(&mut out, &self.tail_guard_digest);
        out.try_into().expect("context task record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        let key = TaskKey::read_from(reader)?;
        require_eq(&reader.u32()?, &1, "context task status")?;
        let direct_status = DirectStatus::try_from(reader.u32()?)?;
        let direct_world_count = reader.u64()?;
        require_eq(&reader.u64()?, &100_000, "context direct cap")?;
        let accepted_cells = reader.u32()?;
        require_eq(&reader.u32()?, &11_730, "context cell cap")?;
        let in_range_slot_bytes = reader.u64()?;
        let canonical_payload_bytes = reader.u64()?;
        let mut total_scaled_mass = [0; 8];
        for limb in &mut total_scaled_mass {
            *limb = reader.u32()?;
        }
        require_eq(
            &reader.u32()?,
            &COMPLETION_CLASS_COMPLETED,
            "context completion class",
        )?;
        require_eq(
            &reader.u32()?,
            &NATIVE_STATUS_COMPLETED,
            "context native status",
        )?;
        require_eq(&reader.u32()?, &0x3ff, "context validation flags")?;
        require_eq(&reader.u32()?, &0, "context reserved")?;
        Ok(Self {
            key,
            direct_status,
            direct_world_count,
            accepted_cells,
            in_range_slot_bytes,
            canonical_payload_bytes,
            total_scaled_mass,
            cpu_slot_digest: reader.digest()?,
            gpu_slot_digest: reader.digest()?,
            cpu_payload_digest: reader.digest()?,
            gpu_payload_digest: reader.digest()?,
            cpu_aggregate_digest: reader.digest()?,
            gpu_aggregate_digest: reader.digest()?,
            tail_guard_digest: reader.digest()?,
        })
    }

    pub fn validate(&self) -> Result<()> {
        self.key.validate()?;
        if self.accepted_cells > 11_730 {
            return Err(CodecError::Invalid("context accepted cell cap"));
        }
        require_eq(
            &self.in_range_slot_bytes,
            &(u64::from(self.key.candidate_slot_count) * 64),
            "context slot byte length",
        )?;
        require_eq(
            &self.canonical_payload_bytes,
            &(50 + u64::from(self.accepted_cells) * 26),
            "context payload byte length",
        )?;
        require_eq(
            &self.cpu_slot_digest,
            &self.gpu_slot_digest,
            "context raw parity",
        )?;
        require_eq(
            &self.cpu_payload_digest,
            &self.gpu_payload_digest,
            "context payload parity",
        )?;
        require_eq(
            &self.cpu_aggregate_digest,
            &self.gpu_aggregate_digest,
            "context aggregate parity",
        )?;
        match self.direct_status {
            DirectStatus::Parity if self.direct_world_count > 100_000 => {
                Err(CodecError::Invalid("direct parity over cap"))
            }
            DirectStatus::DeclaredStop if self.direct_world_count <= 100_000 => {
                Err(CodecError::Invalid("declared stop at/below cap"))
            }
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReducedBindingRecord {
    pub binding_ordinal: u32,
    pub task_ordinal: u32,
    pub arm: Arm,
    pub arm_ordinal: u32,
    pub root_key: [u8; 37],
    pub selected_action: u8,
    pub derived_context: u8,
    pub grade: u8,
    pub matching_count: u8,
    pub reduced_pool_mask: u32,
    pub payload_bytes: u64,
    pub payload_digest: Digest,
    pub semantic_identity: Digest,
}

impl ReducedBindingRecord {
    pub fn encode(&self) -> [u8; REDUCED_BINDING_RECORD_BYTES] {
        let mut out = Vec::with_capacity(REDUCED_BINDING_RECORD_BYTES);
        put_u32(&mut out, 1);
        put_u32(&mut out, self.binding_ordinal);
        put_u32(&mut out, self.task_ordinal);
        put_u32(&mut out, self.arm.into());
        put_u32(&mut out, self.arm_ordinal);
        put_u32(&mut out, 0);
        out.extend_from_slice(&self.root_key);
        out.push(self.selected_action);
        out.push(self.derived_context);
        out.push(self.grade);
        out.push(self.matching_count);
        out.extend_from_slice(&[0; 3]);
        put_u32(&mut out, self.reduced_pool_mask);
        put_u64(&mut out, self.payload_bytes);
        put_digest(&mut out, &self.payload_digest);
        put_digest(&mut out, &self.semantic_identity);
        out.extend_from_slice(&[0; 16]);
        out.try_into().expect("reduced binding width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        value.validate_shape()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        require_eq(&reader.u32()?, &1, "reduced binding version")?;
        let binding_ordinal = reader.u32()?;
        let task_ordinal = reader.u32()?;
        let arm = Arm::try_from(reader.u32()?)?;
        let arm_ordinal = reader.u32()?;
        require_eq(&reader.u32()?, &0, "reduced endpoint")?;
        let root_key = reader.array()?;
        let selected_action = reader.u8()?;
        let derived_context = reader.u8()?;
        let grade = reader.u8()?;
        let matching_count = reader.u8()?;
        require_zero(reader.bytes(3)?, "reduced byte reserved")?;
        let value = Self {
            binding_ordinal,
            task_ordinal,
            arm,
            arm_ordinal,
            root_key,
            selected_action,
            derived_context,
            grade,
            matching_count,
            reduced_pool_mask: reader.u32()?,
            payload_bytes: reader.u64()?,
            payload_digest: reader.digest()?,
            semantic_identity: reader.digest()?,
        };
        require_zero(reader.bytes(16)?, "reduced tail reserved")?;
        Ok(value)
    }

    pub fn validate_shape(&self) -> Result<()> {
        validate_root_key(&self.root_key)?;
        if !matches!(self.arm, Arm::Reduced | Arm::GradeMatching)
            || self.grade == 0
            || self.grade > 6
            || self.derived_context > 7
            || self.selected_action > 27
            || self.reduced_pool_mask >> 28 != 0
        {
            return Err(CodecError::Invalid("reduced binding shape"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalBindingRecord {
    pub binding_ordinal: u32,
    pub task_ordinal: u32,
    pub arm: Arm,
    pub arm_ordinal: u32,
    pub endpoint: u32,
    pub root_key: [u8; 37],
    pub selected_action: u8,
    pub derived_context: u8,
    pub context_pool_mask: u32,
    pub payload_bytes: u64,
    pub payload_digest: Digest,
    pub semantic_identity: Digest,
}

impl PhysicalBindingRecord {
    pub fn encode(&self) -> [u8; PHYSICAL_BINDING_RECORD_BYTES] {
        let mut out = Vec::with_capacity(PHYSICAL_BINDING_RECORD_BYTES);
        put_u32(&mut out, 1);
        put_u32(&mut out, self.binding_ordinal);
        put_u32(&mut out, self.task_ordinal);
        put_u32(&mut out, self.arm.into());
        put_u32(&mut out, self.arm_ordinal);
        put_u32(&mut out, self.endpoint);
        out.extend_from_slice(&self.root_key);
        out.push(self.selected_action);
        out.push(self.derived_context);
        out.push(7);
        out.extend_from_slice(&[0; 4]);
        put_u32(&mut out, self.context_pool_mask);
        put_u64(&mut out, self.payload_bytes);
        put_digest(&mut out, &self.payload_digest);
        put_digest(&mut out, &self.semantic_identity);
        out.extend_from_slice(&[0; 16]);
        out.try_into().expect("physical binding width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self::read_from(&mut reader)?;
        reader.finish()?;
        value.validate_shape()?;
        Ok(value)
    }

    pub(crate) fn read_from(reader: &mut Reader<'_>) -> Result<Self> {
        require_eq(&reader.u32()?, &1, "physical binding version")?;
        let binding_ordinal = reader.u32()?;
        let task_ordinal = reader.u32()?;
        let arm = Arm::try_from(reader.u32()?)?;
        let arm_ordinal = reader.u32()?;
        let endpoint = reader.u32()?;
        let root_key = reader.array()?;
        let selected_action = reader.u8()?;
        let derived_context = reader.u8()?;
        require_eq(&reader.u8()?, &7, "physical grade")?;
        require_zero(reader.bytes(4)?, "physical byte reserved")?;
        let value = Self {
            binding_ordinal,
            task_ordinal,
            arm,
            arm_ordinal,
            endpoint,
            root_key,
            selected_action,
            derived_context,
            context_pool_mask: reader.u32()?,
            payload_bytes: reader.u64()?,
            payload_digest: reader.digest()?,
            semantic_identity: reader.digest()?,
        };
        require_zero(reader.bytes(16)?, "physical tail reserved")?;
        Ok(value)
    }

    pub fn validate_shape(&self) -> Result<()> {
        let endpoint_ok = match self.arm {
            Arm::GradeMatching => self.endpoint == 0,
            Arm::SameContextPair => self.endpoint <= 1,
            Arm::Reduced => false,
        };
        validate_root_key(&self.root_key)?;
        if !endpoint_ok
            || self.derived_context > 7
            || self.selected_action > 27
            || self.context_pool_mask >> 28 != 0
        {
            return Err(CodecError::Invalid("physical binding shape"));
        }
        Ok(())
    }
}

/// Validates the complete 37-byte `W42RTK01` normal form without depending on
/// the higher-level opening-root implementation.
pub fn validate_root_key(root: &[u8; 37]) -> Result<()> {
    require_eq(&root[..8], &b"W42RTK01"[..], "root key magic")?;
    require_eq(
        &u16::from_le_bytes([root[8], root[9]]),
        &1,
        "root key version",
    )?;
    if root[10] > 8 || root[11] > 3 || root[12..=14].iter().any(|seat| *seat != root[11]) {
        return Err(CodecError::Invalid("root declaration/role seats"));
    }
    let hand = u32::from_le_bytes(root[15..19].try_into().expect("four hand bytes"));
    if hand >> 28 != 0 || hand.count_ones() != 7 || root[19] != 7 {
        return Err(CodecError::Invalid("root hand normal form"));
    }
    let expected_loss = match root[20] {
        1 if (30..=41).contains(&root[21]) => 42 - root[21],
        2 if root[21] == 0 => 0,
        _ => return Err(CodecError::Invalid("root contract")),
    };
    require_eq(&root[22], &expected_loss, "root loss budget")?;
    for (offset, expected, label) in [
        (23, 1, "root evidence profile"),
        (25, 1, "root prior profile"),
        (27, 1, "root field profile"),
        (29, 2, "root utility profile"),
        (31, 1, "root horizon profile"),
    ] {
        require_eq(
            &u16::from_le_bytes([root[offset], root[offset + 1]]),
            &expected,
            label,
        )?;
    }
    require_eq(&root[33], &1, "root empty-public marker")?;
    require_zero(&root[34..37], "root public/reserved bytes")?;
    Ok(())
}

/// Returns the focal-hand mask persisted by a validated `W42RTK01` key.
pub fn root_hand_mask(root: &[u8; 37]) -> Result<u32> {
    validate_root_key(root)?;
    Ok(u32::from_le_bytes(
        root[15..19].try_into().expect("four root hand bytes"),
    ))
}

/// Validates the binding facts that cross a root, task key, and selected lead.
///
/// Declaration codes are `0..=6` for pip trump, `7` for doubles trump, and
/// `8` for no-trump. Context `7` is called; all other contexts are natural.
pub fn validate_root_action(
    root: &[u8; 37],
    declaration: u32,
    selected_action: u8,
    derived_context: u8,
) -> Result<()> {
    let hand = root_hand_mask(root)?;
    require_eq(&u32::from(root[10]), &declaration, "root/task declaration")?;
    if selected_action >= 28 || hand & (1u32 << selected_action) == 0 {
        return Err(CodecError::Invalid("selected action not in root hand"));
    }
    require_eq(
        &domino_led_context(root[10], selected_action)?,
        &derived_context,
        "selected action derived context",
    )
}

/// Returns the lowest-index legal root lead in the requested context.
pub fn least_root_action_for_context(root: &[u8; 37], context: u8) -> Result<u8> {
    let hand = root_hand_mask(root)?;
    if context > 7 {
        return Err(CodecError::Invalid("root action context"));
    }
    for action in 0..28u8 {
        if hand & (1u32 << action) != 0 && domino_led_context(root[10], action)? == context {
            return Ok(action);
        }
    }
    Err(CodecError::Invalid("root has no action in context"))
}

fn domino_led_context(declaration: u8, action: u8) -> Result<u8> {
    if declaration > 8 || action >= 28 {
        return Err(CodecError::Invalid("declaration/action code"));
    }
    let mut high = 0u8;
    while action > high * (high + 1) / 2 + high {
        high += 1;
    }
    let low = action - high * (high + 1) / 2;
    let called = match declaration {
        0..=6 => high == declaration || low == declaration,
        7 => high == low,
        8 => false,
        _ => unreachable!("declaration checked above"),
    };
    Ok(if called { 7 } else { high })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityStreamRecord {
    pub binding_ordinal: u32,
    pub identity: Digest,
}

impl IdentityStreamRecord {
    pub fn encode(&self) -> [u8; IDENTITY_STREAM_RECORD_BYTES] {
        let mut out = Vec::with_capacity(IDENTITY_STREAM_RECORD_BYTES);
        put_u32(&mut out, self.binding_ordinal);
        put_u32(&mut out, 0);
        put_digest(&mut out, &self.identity);
        out.try_into().expect("identity stream record width")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtectedChainRecord {
    pub domain: u32,
    pub ordinal: u32,
    pub first_protected_record: u32,
    pub protected_count: u32,
    pub digest: Digest,
}

impl ProtectedChainRecord {
    pub fn encode(&self) -> [u8; PROTECTED_CHAIN_RECORD_BYTES] {
        let mut out = Vec::with_capacity(PROTECTED_CHAIN_RECORD_BYTES);
        put_u32(&mut out, self.domain);
        put_u32(&mut out, self.ordinal);
        put_u32(&mut out, self.first_protected_record);
        put_u32(&mut out, self.protected_count);
        put_digest(&mut out, &self.digest);
        out.try_into().expect("protected chain record width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self {
            domain: reader.u32()?,
            ordinal: reader.u32()?,
            first_protected_record: reader.u32()?,
            protected_count: reader.u32()?,
            digest: reader.digest()?,
        };
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        match self.domain {
            1 if self.ordinal == 0
                && self.first_protected_record == 16_384
                && self.protected_count == 2 =>
            {
                Ok(())
            }
            2 if self.ordinal == 0
                && self.first_protected_record == 13
                && self.protected_count == 2 =>
            {
                Ok(())
            }
            3 if self.ordinal < 13
                && self.first_protected_record == 0
                && self.protected_count == 12 =>
            {
                Ok(())
            }
            4 if self.ordinal < 614
                && self.first_protected_record <= 79_800
                && self.protected_count == 79_802 - self.first_protected_record =>
            {
                Ok(())
            }
            _ => Err(CodecError::Invalid("protected chain record")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseAggregateRecord {
    pub response: [u32; 3],
    pub support: u64,
    pub mass: u64,
}

impl ResponseAggregateRecord {
    pub fn encode(&self) -> [u8; RESPONSE_AGGREGATE_RECORD_BYTES] {
        let mut out = Vec::with_capacity(RESPONSE_AGGREGATE_RECORD_BYTES);
        for value in self.response {
            put_u32(&mut out, value);
        }
        put_u32(&mut out, 0);
        put_u64(&mut out, self.support);
        put_u64(&mut out, self.mass);
        out.try_into().expect("response aggregate width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let value = Self {
            response: [reader.u32()?, reader.u32()?, reader.u32()?],
            support: {
                require_eq(&reader.u32()?, &0, "response aggregate reserved")?;
                reader.u64()?
            },
            mass: reader.u64()?,
        };
        reader.finish()?;
        if value.response[0] == value.response[1]
            || value.response[0] == value.response[2]
            || value.response[1] == value.response[2]
            || value.response.iter().any(|tile| *tile >= 28)
            || value.support == 0
            || value.mass == 0
        {
            return Err(CodecError::Invalid("response aggregate"));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskFramedRecord {
    pub task_ordinal: u32,
    pub payload: Vec<u8>,
}

impl TaskFramedRecord {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(TASK_FRAMED_RECORD_PREFIX_BYTES + self.payload.len());
        put_u32(&mut out, self.task_ordinal);
        put_u32(&mut out, 0);
        put_u64(
            &mut out,
            u64::try_from(self.payload.len())
                .map_err(|_| CodecError::LengthOverflow("task-framed payload u64"))?,
        );
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let task_ordinal = reader.u32()?;
        require_eq(&reader.u32()?, &0, "task-framed reserved")?;
        let len = usize::try_from(reader.u64()?)
            .map_err(|_| CodecError::LengthOverflow("task-framed payload usize"))?;
        let payload = reader.bytes(len)?.to_vec();
        reader.finish()?;
        Ok(Self {
            task_ordinal,
            payload,
        })
    }
}
