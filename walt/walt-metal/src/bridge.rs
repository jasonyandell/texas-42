use core::mem::{align_of, size_of};
use core::ptr;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{MTLBuffer, MTLCommandBuffer, MTLCommandBufferStatus, MTLComputeCommandEncoder};

use crate::{CommandState, MetalError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BufferRole {
    ArithmeticInput,
    ArithmeticOutput,
    OpeningTask,
    OpeningChoose,
    OpeningOutput,
}

/// An owned shared buffer whose complete logical word range has been checked.
///
/// This is private to the crate: callers can neither fabricate one nor obtain a
/// pointer into its storage.
pub(crate) struct ValidatedBuffer {
    raw: Retained<ProtocolObject<dyn MTLBuffer>>,
    logical_words: usize,
    role: BufferRole,
    purpose: &'static str,
}

impl ValidatedBuffer {
    pub(crate) fn logical_bytes(&self) -> Result<usize, MetalError> {
        logical_bytes(self.logical_words, self.purpose)
    }

    pub(crate) fn reported_bytes(&self) -> usize {
        self.raw.length()
    }
}

/// A retained command plus the retained buffers encoded into that command.
pub(crate) struct BoundCommand {
    raw: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
    bound: Vec<Retained<ProtocolObject<dyn MTLBuffer>>>,
}

impl BoundCommand {
    pub(crate) fn new(raw: Retained<ProtocolObject<dyn MTLCommandBuffer>>) -> Self {
        Self {
            raw,
            bound: Vec::new(),
        }
    }

    pub(crate) fn raw(&self) -> &ProtocolObject<dyn MTLCommandBuffer> {
        &self.raw
    }
}

/// Proof that the retained command reached Completed with no NSError.
///
/// It owns the command and every registered binding, so completed-buffer reads
/// cannot outlive the ownership and synchronization provenance they rely on.
pub(crate) struct CompletionProof {
    command: BoundCommand,
}

fn logical_bytes(words: usize, purpose: &'static str) -> Result<usize, MetalError> {
    words
        .checked_mul(size_of::<u32>())
        .ok_or(MetalError::LengthOverflow(purpose))
}

fn validate_storage(
    raw: &ProtocolObject<dyn MTLBuffer>,
    words: usize,
    purpose: &'static str,
) -> Result<(), MetalError> {
    let required = logical_bytes(words, purpose)?;
    let reported = raw.length();
    if reported < required {
        return Err(MetalError::BufferTooShort {
            purpose,
            required,
            reported,
        });
    }
    // `contents` returns NonNull; checking its address also records the frozen
    // four-byte host-word alignment precondition before any pointer operation.
    if !raw
        .contents()
        .as_ptr()
        .addr()
        .is_multiple_of(align_of::<u32>())
    {
        return Err(MetalError::BufferMisaligned(purpose));
    }
    Ok(())
}

/// Unsafe class 1/3: copy validated host words into owned shared storage.
pub(crate) fn copy_host_words(
    raw: Retained<ProtocolObject<dyn MTLBuffer>>,
    logical_words: usize,
    words: &[u32],
    role: BufferRole,
    purpose: &'static str,
) -> Result<ValidatedBuffer, MetalError> {
    if words.len() != logical_words {
        return Err(MetalError::WrongRecordCount {
            purpose,
            expected: logical_words,
            actual: words.len(),
        });
    }
    copy_words_into(&raw, logical_words, words, purpose)?;
    Ok(ValidatedBuffer {
        raw,
        logical_words,
        role,
        purpose,
    })
}

/// Reinitialize a reusable buffer while the private sequential runtime proves
/// that no command is in flight. This is the same host-word copy operation
/// class as initial creation; no pointer or storage escapes.
pub(crate) fn overwrite_idle_words(
    buffer: &ValidatedBuffer,
    words: &[u32],
) -> Result<(), MetalError> {
    if words.len() != buffer.logical_words {
        return Err(MetalError::WrongRecordCount {
            purpose: buffer.purpose,
            expected: buffer.logical_words,
            actual: words.len(),
        });
    }
    copy_words_into(&buffer.raw, buffer.logical_words, words, buffer.purpose)
}

fn copy_words_into(
    raw: &ProtocolObject<dyn MTLBuffer>,
    logical_words: usize,
    words: &[u32],
    purpose: &'static str,
) -> Result<(), MetalError> {
    validate_storage(raw, logical_words, purpose)?;
    // SAFETY: `validate_storage` proves a non-null, u32-aligned destination of
    // at least `logical_words * 4` bytes. `words` has exactly that many u32s;
    // its immutable allocation cannot overlap the separately owned MTLBuffer.
    unsafe {
        ptr::copy_nonoverlapping(
            words.as_ptr(),
            raw.contents().cast::<u32>().as_ptr(),
            logical_words,
        );
    }
    Ok(())
}

/// Unsafe class 2/3: bind a validated buffer at the frozen zero offset.
pub(crate) fn bind_zero(
    command: &mut BoundCommand,
    encoder: &ProtocolObject<dyn MTLComputeCommandEncoder>,
    buffer: &ValidatedBuffer,
    index: usize,
    expected_role: BufferRole,
    expected_words: usize,
) -> Result<(), MetalError> {
    if index > 2 {
        return Err(MetalError::InvalidBinding {
            index,
            reason: "only frozen indices 0 through 2 are permitted",
        });
    }
    validate_binding_contract(
        buffer.role,
        buffer.logical_words,
        expected_role,
        expected_words,
        index,
    )?;
    validate_storage(&buffer.raw, buffer.logical_words, buffer.purpose)?;
    // SAFETY: The wrapper fixes offset zero and an in-registry index, and the
    // buffer's complete logical range, alignment, and shared contents have just
    // been checked. `BoundCommand` retains the buffer through completion.
    unsafe {
        encoder.setBuffer_offset_atIndex(Some(&buffer.raw), 0, index);
    }
    command.bound.push(buffer.raw.clone());
    Ok(())
}

fn validate_binding_contract(
    actual_role: BufferRole,
    actual_words: usize,
    expected_role: BufferRole,
    expected_words: usize,
    index: usize,
) -> Result<(), MetalError> {
    if actual_role != expected_role {
        return Err(MetalError::InvalidBinding {
            index,
            reason: "buffer role does not match frozen kernel binding",
        });
    }
    if actual_words != expected_words {
        return Err(MetalError::InvalidBinding {
            index,
            reason: "buffer length does not match frozen kernel binding",
        });
    }
    Ok(())
}

pub(crate) fn prove_completed(command: BoundCommand) -> Result<CompletionProof, MetalError> {
    let state = command.raw.status();
    if state != MTLCommandBufferStatus::Completed {
        return Err(MetalError::UnexpectedCommandState(command_state(state)));
    }
    if let Some(error) = command.raw.error() {
        return Err(MetalError::CommandError(error.to_string()));
    }
    Ok(CompletionProof { command })
}

/// Unsafe class 3/3: copy completed shared storage into owned Rust words.
pub(crate) fn copy_completed_words(
    proof: &CompletionProof,
    buffer: &ValidatedBuffer,
) -> Result<Vec<u32>, MetalError> {
    if proof.command.raw.status() != MTLCommandBufferStatus::Completed {
        return Err(MetalError::UnexpectedCommandState(command_state(
            proof.command.raw.status(),
        )));
    }
    if let Some(error) = proof.command.raw.error() {
        return Err(MetalError::CommandError(error.to_string()));
    }
    let owned_by_command = proof
        .command
        .bound
        .iter()
        .any(|candidate| ptr::eq::<ProtocolObject<dyn MTLBuffer>>(&**candidate, &*buffer.raw));
    if !owned_by_command {
        return Err(MetalError::InvalidBinding {
            index: usize::MAX,
            reason: "completion proof does not own this buffer",
        });
    }
    validate_storage(&buffer.raw, buffer.logical_words, buffer.purpose)?;
    let mut words = vec![0u32; buffer.logical_words];
    // SAFETY: Completion plus absence of command error proves GPU writes are
    // coherent. Ownership is tied to this proof, and `validate_storage` proves
    // a non-null, aligned source covering the exactly sized owned destination.
    unsafe {
        ptr::copy_nonoverlapping(
            buffer.raw.contents().cast::<u32>().as_ptr(),
            words.as_mut_ptr(),
            buffer.logical_words,
        );
    }
    Ok(words)
}

pub(crate) fn command_state(status: MTLCommandBufferStatus) -> CommandState {
    match status {
        MTLCommandBufferStatus::NotEnqueued => CommandState::NotEnqueued,
        MTLCommandBufferStatus::Enqueued => CommandState::Enqueued,
        MTLCommandBufferStatus::Committed => CommandState::Committed,
        MTLCommandBufferStatus::Scheduled => CommandState::Scheduled,
        MTLCommandBufferStatus::Completed => CommandState::Completed,
        MTLCommandBufferStatus::Error => CommandState::Error,
        other => CommandState::Unknown(other.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_binding_contract_rejects_wrong_role_and_length() {
        validate_binding_contract(BufferRole::OpeningTask, 8, BufferRole::OpeningTask, 8, 0)
            .expect("exact typed binding");
        assert!(matches!(
            validate_binding_contract(BufferRole::OpeningChoose, 8, BufferRole::OpeningTask, 8, 0,),
            Err(MetalError::InvalidBinding { index: 0, .. })
        ));
        assert!(matches!(
            validate_binding_contract(BufferRole::OpeningTask, 9, BufferRole::OpeningTask, 8, 0,),
            Err(MetalError::InvalidBinding { index: 0, .. })
        ));
    }
}
