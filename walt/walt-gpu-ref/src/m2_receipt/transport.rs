use super::receipt::{
    SuccessReceipt, FREEZE56_DESCRIPTOR_SHA256, PARENT_COMMIT_SHA1, SUCCESS_MAGIC,
};
use super::wire::{
    put_digest, put_i32, put_u16, put_u32, put_u64, require_eq, require_zero, sha256, CodecError,
    Digest, FailureCode, FailurePhase, FrameKind, Reader, Result, TerminalCode, ZERO_DIGEST,
};

pub const FAILURE_MAGIC: [u8; 8] = *b"W42M2F01";
pub const FAILURE_BYTES: usize = 256;
pub const SMOKE_MAGIC: [u8; 8] = *b"W42M2SM1";
pub const SMOKE_BYTES: usize = 32;
pub const FRAME_FIXED_PAYLOAD_BYTES: usize = 16;
pub const FRAME_HEADER_BYTES: usize = 20;
pub const MAX_FRAME_PAYLOAD_BYTES: u32 = 0x7fff_ffff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureReceipt {
    pub phase: FailurePhase,
    pub code: FailureCode,
    pub task_ordinal: u32,
    pub subordinal: u32,
    pub child_exit: i32,
    pub native_status: u32,
    pub observed_mismatch: u32,
    pub build_identity: Digest,
    pub freeze56_digest: Digest,
    pub child_failure_frame_digest: Digest,
}

impl FailureReceipt {
    pub fn encode(&self) -> [u8; FAILURE_BYTES] {
        let mut out = Vec::with_capacity(FAILURE_BYTES);
        out.extend_from_slice(&FAILURE_MAGIC);
        put_u16(&mut out, 1);
        put_u16(&mut out, 256);
        put_u32(&mut out, 2);
        put_u64(&mut out, 256);
        put_u32(&mut out, self.phase.into());
        put_u32(&mut out, self.code.into());
        put_u32(&mut out, self.task_ordinal);
        put_u32(&mut out, self.subordinal);
        put_i32(&mut out, self.child_exit);
        put_u32(&mut out, self.native_status);
        for _ in 0..4 {
            put_u32(&mut out, 0);
        }
        put_u64(&mut out, 0);
        put_u32(&mut out, self.observed_mismatch);
        put_u32(&mut out, 0);
        put_digest(&mut out, &self.build_identity);
        put_digest(&mut out, &self.freeze56_digest);
        put_digest(&mut out, &self.child_failure_frame_digest);
        out.extend_from_slice(&PARENT_COMMIT_SHA1);
        out.extend_from_slice(&[0; 60]);
        out.try_into().expect("failure receipt width")
    }

    pub fn encode_child_zeroed(&self) -> Result<[u8; FAILURE_BYTES]> {
        require_eq(
            &self.child_failure_frame_digest,
            &ZERO_DIGEST,
            "child-zeroed failure digest",
        )?;
        Ok(self.encode())
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &FAILURE_BYTES, "failure receipt length")?;
        let mut reader = Reader::new(bytes);
        require_eq(&reader.array::<8>()?, &FAILURE_MAGIC, "failure magic")?;
        require_eq(&reader.u16()?, &1, "failure version")?;
        require_eq(&reader.u16()?, &256, "failure fixed bytes")?;
        require_eq(&reader.u32()?, &2, "failure outcome")?;
        require_eq(&reader.u64()?, &256, "failure total bytes")?;
        let phase = FailurePhase::try_from(reader.u32()?)?;
        let code = FailureCode::try_from(reader.u32()?)?;
        let task_ordinal = reader.u32()?;
        let subordinal = reader.u32()?;
        let child_exit = reader.i32()?;
        let native_status = reader.u32()?;
        for _ in 0..4 {
            require_eq(&reader.u32()?, &0, "failure accepted count")?;
        }
        require_eq(&reader.u64()?, &0, "failure accepted payload")?;
        let observed_mismatch = reader.u32()?;
        require_eq(&reader.u32()?, &0, "failure partial result")?;
        let value = Self {
            phase,
            code,
            task_ordinal,
            subordinal,
            child_exit,
            native_status,
            observed_mismatch,
            build_identity: reader.digest()?,
            freeze56_digest: reader.digest()?,
            child_failure_frame_digest: reader.digest()?,
        };
        require_eq(
            &reader.array::<20>()?,
            &PARENT_COMMIT_SHA1,
            "failure parent commit",
        )?;
        require_zero(reader.bytes(60)?, "failure reserved")?;
        reader.finish()?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<()> {
        require_eq(
            &self.freeze56_digest,
            &FREEZE56_DESCRIPTOR_SHA256,
            "failure freeze56 digest",
        )?;
        if self.task_ordinal != u32::MAX && self.task_ordinal >= 614 {
            return Err(CodecError::Invalid("failure task ordinal"));
        }
        if self.child_exit == 124 {
            let active_timeout_phase = matches!(
                self.phase,
                FailurePhase::Gate0
                    | FailurePhase::ArithmeticNegative
                    | FailurePhase::ArithmeticCorpus
                    | FailurePhase::OpeningNegative
                    | FailurePhase::ProjectorTask
            );
            if !active_timeout_phase || self.code != FailureCode::Timeout {
                return Err(CodecError::Invalid("exit-124 timeout mapping"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmokeReport;

impl SmokeReport {
    pub fn encode(self) -> [u8; SMOKE_BYTES] {
        let mut out = Vec::with_capacity(SMOKE_BYTES);
        out.extend_from_slice(&SMOKE_MAGIC);
        put_u32(&mut out, 1);
        put_u32(&mut out, 2);
        put_u32(&mut out, 2);
        put_u32(&mut out, 0);
        put_u64(&mut out, 0);
        out.try_into().expect("smoke report width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        require_eq(&bytes.len(), &SMOKE_BYTES, "smoke report length")?;
        let mut reader = Reader::new(bytes);
        require_eq(&reader.array::<8>()?, &SMOKE_MAGIC, "smoke magic")?;
        require_eq(&reader.u32()?, &1, "smoke version")?;
        require_eq(&reader.u32()?, &2, "smoke command count")?;
        require_eq(&reader.u32()?, &2, "smoke completed count")?;
        require_eq(&reader.u32()?, &0, "smoke accepted count")?;
        require_eq(&reader.u64()?, &0, "smoke payload bytes")?;
        reader.finish()?;
        Ok(Self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireFrame {
    pub kind: FrameKind,
    pub phase_or_command_ordinal: u32,
    pub unit_or_terminal_code: u32,
    pub detail: Vec<u8>,
}

impl WireFrame {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate_detail()?;
        let detail_len = u32::try_from(self.detail.len())
            .map_err(|_| CodecError::LengthOverflow("frame detail u32"))?;
        let payload_len = detail_len
            .checked_add(FRAME_FIXED_PAYLOAD_BYTES as u32)
            .ok_or(CodecError::LengthOverflow("frame payload u32"))?;
        if payload_len > MAX_FRAME_PAYLOAD_BYTES {
            return Err(CodecError::Invalid("frame payload cap"));
        }
        let mut out = Vec::with_capacity(FRAME_HEADER_BYTES + self.detail.len());
        put_u32(&mut out, payload_len);
        put_u16(&mut out, 1);
        put_u16(&mut out, self.kind.into());
        put_u32(&mut out, self.phase_or_command_ordinal);
        put_u32(&mut out, self.unit_or_terminal_code);
        put_u32(&mut out, detail_len);
        out.extend_from_slice(&self.detail);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = Reader::new(bytes);
        let payload_len = reader.u32()?;
        if payload_len > MAX_FRAME_PAYLOAD_BYTES {
            return Err(CodecError::Invalid("frame payload cap"));
        }
        require_eq(
            &checked_frame_total(payload_len)?,
            &bytes.len(),
            "complete frame length",
        )?;
        require_eq(&reader.u16()?, &1, "frame protocol version")?;
        let kind = FrameKind::try_from(reader.u16()?)?;
        let phase_or_command_ordinal = reader.u32()?;
        let unit_or_terminal_code = reader.u32()?;
        let detail_len = usize::try_from(reader.u32()?)
            .map_err(|_| CodecError::LengthOverflow("frame detail usize"))?;
        let encoded_detail_len = u32::try_from(detail_len)
            .map_err(|_| CodecError::LengthOverflow("frame detail u32"))?;
        let expected_payload_len = encoded_detail_len
            .checked_add(FRAME_FIXED_PAYLOAD_BYTES as u32)
            .ok_or(CodecError::LengthOverflow("frame payload/detail relation"))?;
        require_eq(
            &payload_len,
            &expected_payload_len,
            "frame payload/detail relation",
        )?;
        let detail = reader.bytes(detail_len)?.to_vec();
        reader.finish()?;
        let value = Self {
            kind,
            phase_or_command_ordinal,
            unit_or_terminal_code,
            detail,
        };
        value.validate_detail()?;
        Ok(value)
    }

    pub fn validate_detail(&self) -> Result<()> {
        match self.kind {
            FrameKind::Preparing | FrameKind::Finalizing => {
                if !self.detail.is_empty() {
                    return Err(CodecError::Invalid("nonempty progress detail"));
                }
            }
            FrameKind::Committed => {
                if !self.detail.is_empty() || self.unit_or_terminal_code != 0 {
                    return Err(CodecError::Invalid("committed frame detail/unit"));
                }
            }
            FrameKind::Terminal => {
                if !self.detail.is_empty() {
                    return Err(CodecError::Invalid("nonempty terminal detail"));
                }
                TerminalCode::try_from(self.unit_or_terminal_code)?;
            }
            FrameKind::Success => {
                require_eq(
                    &(self.phase_or_command_ordinal, self.unit_or_terminal_code),
                    &(0, 0),
                    "success frame coordinates",
                )?;
                if self.detail.starts_with(&SUCCESS_MAGIC) {
                    SuccessReceipt::decode(&self.detail)?;
                } else if self.detail.starts_with(&SMOKE_MAGIC) {
                    SmokeReport::decode(&self.detail)?;
                } else {
                    return Err(CodecError::Invalid("success frame detail magic"));
                }
            }
            FrameKind::Failure => {
                let failure = FailureReceipt::decode(&self.detail)?;
                require_eq(
                    &failure.child_failure_frame_digest,
                    &ZERO_DIGEST,
                    "child FAILURE frame digest must be zero",
                )?;
            }
        }
        Ok(())
    }
}

/// Validates a child's single zeroed FAILURE frame and renders the terminal
/// parent-side receipt.  The returned receipt hashes the child frame but is not
/// itself embedded in another frame, so the construction is acyclic.
pub fn rerender_parent_failure(
    child_frame_bytes: &[u8],
    child_exit: i32,
) -> Result<FailureReceipt> {
    let frame = WireFrame::decode(child_frame_bytes)?;
    require_eq(
        &frame.kind,
        &FrameKind::Failure,
        "parent failure frame kind",
    )?;
    let mut failure = FailureReceipt::decode(&frame.detail)?;
    require_eq(
        &failure.child_failure_frame_digest,
        &ZERO_DIGEST,
        "child failure digest zero",
    )?;
    require_eq(
        &failure.child_exit,
        &i32::MIN,
        "child failure unavailable exit",
    )?;
    if failure.code == FailureCode::Timeout {
        return Err(CodecError::Invalid("direct child timeout failure"));
    }
    failure.child_exit = child_exit;
    failure.child_failure_frame_digest = sha256(child_frame_bytes);
    failure.validate()?;
    Ok(failure)
}

fn checked_frame_total(payload_len: u32) -> Result<usize> {
    usize::try_from(payload_len)
        .map_err(|_| CodecError::LengthOverflow("frame payload usize"))?
        .checked_add(4)
        .ok_or(CodecError::LengthOverflow("complete frame usize"))
}
