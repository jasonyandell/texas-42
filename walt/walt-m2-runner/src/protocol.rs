//! Closed parent-side frame validation and child supervision for freeze 56.
//!
//! The child owns Metal.  This module owns the other half of the contract: it
//! accepts only the complete smoke or official frame language, applies the two
//! monotonic watchdogs, and never returns child success until the exact stream
//! has ended and the child has been reaped successfully.

use std::fmt;
use std::io::{self, Read};
use std::process::{Child, ChildStdout, ExitStatus};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender};
use std::thread;
use std::time::{Duration, Instant};

use walt_gpu_ref::m2_receipt::{
    rerender_parent_failure, CodecError, Digest, FailureCode, FailurePhase, FailureReceipt,
    FrameKind, SmokeReport, SuccessReceipt, TerminalCode, WireFrame, FAILURE_BYTES,
    FRAME_FIXED_PAYLOAD_BYTES, FRAME_HEADER_BYTES, FREEZE56_DESCRIPTOR_SHA256,
    MAX_FRAME_PAYLOAD_BYTES, SMOKE_BYTES, SUCCESS_HEADER_BYTES, ZERO_DIGEST,
};

pub const MAXIMUM_POLL_INTERVAL: Duration = Duration::from_millis(10);
pub const PARENT_COMMAND_DEADLINE: Duration = Duration::from_millis(125_000);
pub const CPU_LIVENESS_DEADLINE: Duration = Duration::from_millis(600_000);

const UNAVAILABLE_ORDINAL: u32 = u32::MAX;
const UNAVAILABLE_EXIT: i32 = i32::MIN;
const UNAVAILABLE_STATUS: u32 = u32::MAX;
const NATIVE_NOT_ENQUEUED: u32 = 0;
const NATIVE_ENQUEUED: u32 = 1;
const NATIVE_SCHEDULED: u32 = 3;
const NATIVE_ERROR: u32 = 5;
const SMOKE_COMMAND_COUNT: u32 = 2;
const OFFICIAL_COMMAND_COUNT: u32 = 630;
const OFFICIAL_PREPARING_COUNT: u32 = 1_232;
const OFFICIAL_BINDING_COUNT: u32 = 1_118;
const OFFICIAL_SECTION_COUNT: u32 = 10;
const OFFICIAL_STREAM_QUEUE_CAPACITY: usize = 4_252;
const MAX_OFFICIAL_SUCCESS_DETAIL_BYTES: u32 = 1_048_576;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunProfile {
    Smoke,
    Official,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SupervisorDeadlines {
    pub poll_interval: Duration,
    pub command: Duration,
    pub cpu_liveness: Duration,
}

impl SupervisorDeadlines {
    pub const fn production() -> Self {
        Self {
            poll_interval: MAXIMUM_POLL_INTERVAL,
            command: PARENT_COMMAND_DEADLINE,
            cpu_liveness: CPU_LIVENESS_DEADLINE,
        }
    }

    fn validate(self) -> Result<(), SupervisorError> {
        if self.poll_interval.is_zero() || self.poll_interval > MAXIMUM_POLL_INTERVAL {
            return Err(SupervisorError::InvalidDeadlines(
                "poll interval must be in 1ns..=10ms",
            ));
        }
        if self.command.is_zero() {
            return Err(SupervisorError::InvalidDeadlines(
                "command deadline must be nonzero",
            ));
        }
        if self.cpu_liveness.is_zero() {
            return Err(SupervisorError::InvalidDeadlines(
                "CPU liveness deadline must be nonzero",
            ));
        }
        Ok(())
    }
}

impl Default for SupervisorDeadlines {
    fn default() -> Self {
        Self::production()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SupervisedSuccess {
    Smoke(SmokeReport),
    Official(Box<SuccessReceipt>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SupervisedOutcome {
    Success(SupervisedSuccess),
    Failure(FailureReceipt),
}

#[derive(Debug)]
pub enum SupervisorError {
    InvalidDeadlines(&'static str),
    MissingPipedStdout,
    ChildIo {
        operation: &'static str,
        source: io::Error,
    },
    ReaderThreadPanicked,
    FailureCodec(CodecError),
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeadlines(reason) => {
                write!(formatter, "invalid M2 supervisor deadlines: {reason}")
            }
            Self::MissingPipedStdout => {
                formatter.write_str("M2 child stdout was not configured as piped")
            }
            Self::ChildIo { operation, source } => {
                write!(formatter, "M2 child {operation} failed: {source}")
            }
            Self::ReaderThreadPanicked => formatter.write_str("M2 frame reader thread panicked"),
            Self::FailureCodec(source) => {
                write!(
                    formatter,
                    "M2 failure receipt codec rejected supervisor output: {source}"
                )
            }
        }
    }
}

impl std::error::Error for SupervisorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChildIo { source, .. } => Some(source),
            Self::FailureCodec(source) => Some(source),
            _ => None,
        }
    }
}

/// Supervise one already-spawned child with the frozen production deadlines.
///
/// The child is consumed because every return path owns the responsibility to
/// kill when necessary and reap exactly once.  Its stdout must be piped.
pub fn supervise_child(
    child: Child,
    profile: RunProfile,
    build_identity: Digest,
) -> Result<SupervisedOutcome, SupervisorError> {
    supervise_child_with_deadlines(
        child,
        profile,
        build_identity,
        SupervisorDeadlines::production(),
    )
}

/// The same closed supervisor with injectable deadlines for deterministic
/// synthetic tests.  Production callers should use [`supervise_child`].
pub fn supervise_child_with_deadlines(
    mut child: Child,
    profile: RunProfile,
    build_identity: Digest,
    deadlines: SupervisorDeadlines,
) -> Result<SupervisedOutcome, SupervisorError> {
    if let Err(error) = deadlines.validate() {
        kill_and_reap(&mut child)?;
        return Err(error);
    }
    let Some(stdout) = child.stdout.take() else {
        kill_and_reap(&mut child)?;
        return Err(SupervisorError::MissingPipedStdout);
    };

    // This holds the complete largest valid stream plus EOF.  Consequently a
    // descheduled supervisor cannot prevent the reader from timestamping a
    // timely COMMITTED/TERMINAL pair.  Invalid unbounded streams still meet
    // backpressure once this closed census is exceeded.
    let (sender, receiver) = mpsc::sync_channel(OFFICIAL_STREAM_QUEUE_CAPACITY);
    let reader = thread::spawn(move || read_frames(stdout, sender));
    let loop_result = supervise_loop(&mut child, &receiver, profile, build_identity, deadlines);
    drop(receiver);

    match loop_result {
        LoopResult::Complete { status, candidate } => {
            let result = finish_complete(status, candidate, build_identity);
            reader
                .join()
                .map_err(|_| SupervisorError::ReaderThreadPanicked)
                .and(result)
        }
        LoopResult::Abort(specification) => kill_and_reap(&mut child).and_then(|status| {
            render_parent_failure(specification, status, build_identity)
                .map(SupervisedOutcome::Failure)
        }),
    }
}

fn finish_complete(
    status: ExitStatus,
    candidate: Option<Candidate>,
    build_identity: Digest,
) -> Result<SupervisedOutcome, SupervisorError> {
    match candidate {
        Some(Candidate::Success(success)) if status.success() => {
            Ok(SupervisedOutcome::Success(success))
        }
        Some(Candidate::Success(_)) => {
            let specification = exit_failure_specification(status, None);
            render_parent_failure(specification, status, build_identity)
                .map(SupervisedOutcome::Failure)
        }
        Some(Candidate::ChildFailure(frame_bytes)) => {
            let child_exit = exit_code(status);
            match rerender_parent_failure(&frame_bytes, child_exit) {
                Ok(receipt) if receipt.build_identity == build_identity => {
                    Ok(SupervisedOutcome::Failure(receipt))
                }
                Ok(_) | Err(_) => {
                    render_parent_failure(FailureSpecification::protocol(), status, build_identity)
                        .map(SupervisedOutcome::Failure)
                }
            }
        }
        Some(Candidate::TerminalTimeout(specification)) if status.code() == Some(124) => {
            render_parent_failure(specification, status, build_identity)
                .map(SupervisedOutcome::Failure)
        }
        Some(Candidate::TerminalTimeout(_)) => {
            render_parent_failure(FailureSpecification::protocol(), status, build_identity)
                .map(SupervisedOutcome::Failure)
        }
        None => {
            let specification = exit_failure_specification(status, None);
            render_parent_failure(specification, status, build_identity)
                .map(SupervisedOutcome::Failure)
        }
    }
}

fn supervise_loop(
    child: &mut Child,
    receiver: &Receiver<ReaderEvent>,
    profile: RunProfile,
    build_identity: Digest,
    deadlines: SupervisorDeadlines,
) -> LoopResult {
    let mut machine = ProtocolMachine::new(profile);
    let mut last_cpu_progress = Instant::now();
    let mut committed: Option<(u32, Instant)> = None;
    let mut candidate = None;
    let mut eof = false;
    let mut exit_status = None;

    loop {
        if exit_status.is_none() {
            match child.try_wait() {
                Ok(status) => exit_status = status,
                Err(_) => return LoopResult::Abort(FailureSpecification::protocol()),
            }
        }
        if eof {
            if let Some(status) = exit_status {
                if candidate.is_none() && status.code() == Some(124) {
                    let specification = committed
                        .map_or_else(FailureSpecification::protocol, |(command, _)| {
                            FailureSpecification::command_timeout(profile, command)
                        });
                    return LoopResult::Abort(specification);
                }
                return LoopResult::Complete { status, candidate };
            }
        }

        let now = Instant::now();
        let observed_exit = exit_status.as_ref().and_then(ExitStatus::code);
        let (deadline, timeout_specification) = if let Some((command, started)) = committed {
            let specification = if exit_status.is_some() && observed_exit != Some(124) {
                // Once the child has exited, only the contract's dedicated 124
                // exit can still corroborate an active command timeout.  A
                // signalled or differently exited child whose pipe is held by
                // a descendant is a protocol failure, not a timeout that may
                // borrow the active command's coordinates.
                FailureSpecification::protocol()
            } else {
                FailureSpecification::command_timeout(profile, command)
            };
            (started + deadlines.command, specification)
        } else if candidate.is_some() || exit_status.is_some() {
            // EOF is part of the accepted stream.  If the child has already
            // exited without an active committed command, a delayed EOF (for
            // example, an inherited stdout holder) must never be retyped as a
            // timeout in the machine's next phase.  Wait for the ordinary CPU
            // deadline so queued frames can drain, then fail as protocol.
            (
                last_cpu_progress + deadlines.cpu_liveness,
                FailureSpecification::protocol(),
            )
        } else {
            (
                last_cpu_progress + deadlines.cpu_liveness,
                FailureSpecification::cpu_timeout(machine.failure_phase()),
            )
        };
        let deadline_expired = now >= deadline;
        let wait = if deadline_expired {
            // A frame whose reader receipt timestamp met the deadline remains
            // timely even if this thread was descheduled.  One bounded poll
            // lets an already-read frame cross the synchronous channel; its
            // carried timestamp, never dequeue time, decides admissibility.
            deadlines.poll_interval
        } else {
            deadlines.poll_interval.min(deadline.duration_since(now))
        };

        match receiver.recv_timeout(wait) {
            Ok(ReaderEvent::Frame {
                bytes: frame_bytes,
                received_at,
            }) => {
                if received_at > deadline {
                    return LoopResult::Abort(timeout_specification);
                }
                if candidate.is_some() {
                    return LoopResult::Abort(FailureSpecification::protocol());
                }
                let frame = match WireFrame::decode(&frame_bytes) {
                    Ok(frame) => frame,
                    Err(_) => return LoopResult::Abort(FailureSpecification::protocol()),
                };
                match machine.accept(&frame, build_identity) {
                    Ok(Accepted::CpuProgress) => last_cpu_progress = received_at,
                    Ok(Accepted::CommandCommitted(command)) => {
                        committed = Some((command, received_at));
                    }
                    Ok(Accepted::CommandCompleted(command)) => {
                        if committed.map(|(active, _)| active) != Some(command) {
                            return LoopResult::Abort(FailureSpecification::protocol());
                        }
                        committed = None;
                        last_cpu_progress = received_at;
                    }
                    Ok(Accepted::CommandFailed { command, terminal }) => {
                        committed = None;
                        last_cpu_progress = received_at;
                        if terminal == TerminalCode::Timeout {
                            candidate = Some(Candidate::TerminalTimeout(
                                FailureSpecification::terminal(profile, command, terminal),
                            ));
                        }
                    }
                    Ok(Accepted::Success(success)) => {
                        committed = None;
                        last_cpu_progress = received_at;
                        candidate = Some(Candidate::Success(success));
                    }
                    Ok(Accepted::ChildFailure) => {
                        committed = None;
                        last_cpu_progress = received_at;
                        candidate = Some(Candidate::ChildFailure(frame_bytes));
                    }
                    Err(_) => return LoopResult::Abort(FailureSpecification::protocol()),
                }
            }
            Ok(ReaderEvent::Eof { received_at }) => {
                if received_at > deadline {
                    return LoopResult::Abort(timeout_specification);
                }
                eof = true;
            }
            Ok(ReaderEvent::Fault {
                fault: _fault,
                received_at,
            }) => {
                if received_at > deadline {
                    return LoopResult::Abort(timeout_specification);
                }
                return LoopResult::Abort(FailureSpecification::protocol());
            }
            Err(RecvTimeoutError::Timeout) => {
                if deadline_expired {
                    return LoopResult::Abort(timeout_specification);
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                if eof {
                    if deadline_expired {
                        return LoopResult::Abort(timeout_specification);
                    }
                    thread::sleep(wait);
                    continue;
                }
                return LoopResult::Abort(FailureSpecification::protocol());
            }
        }
    }
}

#[derive(Debug)]
enum LoopResult {
    Complete {
        status: ExitStatus,
        candidate: Option<Candidate>,
    },
    Abort(FailureSpecification),
}

#[derive(Debug)]
enum Candidate {
    Success(SupervisedSuccess),
    ChildFailure(Vec<u8>),
    TerminalTimeout(FailureSpecification),
}

#[derive(Clone, Copy, Debug)]
struct FailureSpecification {
    phase: FailurePhase,
    code: FailureCode,
    task_ordinal: u32,
    subordinal: u32,
    native_status: u32,
    observed_mismatch: u32,
    admits_child_exit_124: bool,
}

impl FailureSpecification {
    const fn protocol() -> Self {
        Self {
            phase: FailurePhase::ChildProtocol,
            code: FailureCode::ChildProtocolFailure,
            task_ordinal: UNAVAILABLE_ORDINAL,
            subordinal: UNAVAILABLE_ORDINAL,
            native_status: UNAVAILABLE_STATUS,
            observed_mismatch: 1,
            admits_child_exit_124: false,
        }
    }

    const fn cpu_timeout(phase: FailurePhase) -> Self {
        Self {
            phase,
            code: FailureCode::Timeout,
            task_ordinal: UNAVAILABLE_ORDINAL,
            subordinal: UNAVAILABLE_ORDINAL,
            native_status: UNAVAILABLE_STATUS,
            observed_mismatch: 0,
            admits_child_exit_124: false,
        }
    }

    fn command_timeout(profile: RunProfile, command: u32) -> Self {
        let (phase, task_ordinal, subordinal) = command_failure_coordinates(profile, command);
        Self {
            phase,
            code: FailureCode::Timeout,
            task_ordinal,
            subordinal,
            native_status: UNAVAILABLE_STATUS,
            observed_mismatch: 0,
            admits_child_exit_124: true,
        }
    }

    fn terminal(profile: RunProfile, command: u32, terminal: TerminalCode) -> Self {
        let mut value = Self::command_timeout(profile, command);
        value.native_status = match terminal {
            TerminalCode::Error => NATIVE_ERROR,
            TerminalCode::NotEnqueued => NATIVE_NOT_ENQUEUED,
            TerminalCode::NotCommitted => NATIVE_ENQUEUED,
            TerminalCode::Scheduled => NATIVE_SCHEDULED,
            TerminalCode::Timeout | TerminalCode::Unknown | TerminalCode::Completed => {
                UNAVAILABLE_STATUS
            }
        };
        value.code = match terminal {
            TerminalCode::Timeout => FailureCode::Timeout,
            TerminalCode::Error => FailureCode::CommandError,
            TerminalCode::NotEnqueued
            | TerminalCode::NotCommitted
            | TerminalCode::Scheduled
            | TerminalCode::Unknown => FailureCode::CommandStateFailure,
            TerminalCode::Completed => FailureCode::InternalFailure,
        };
        value.admits_child_exit_124 = terminal == TerminalCode::Timeout;
        value
    }
}

fn command_failure_coordinates(profile: RunProfile, command: u32) -> (FailurePhase, u32, u32) {
    match (profile, command) {
        (_, 0) => (
            FailurePhase::Gate0,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        ),
        (RunProfile::Smoke, 1) => (FailurePhase::ProjectorTask, 109, UNAVAILABLE_ORDINAL),
        (RunProfile::Official, 1) => (
            FailurePhase::ArithmeticNegative,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        ),
        (RunProfile::Official, 2) => (
            FailurePhase::ArithmeticCorpus,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        ),
        (RunProfile::Official, 3..=15) => (
            FailurePhase::OpeningNegative,
            command - 3,
            UNAVAILABLE_ORDINAL,
        ),
        (RunProfile::Official, 16..=629) => (
            FailurePhase::ProjectorTask,
            command - 16,
            UNAVAILABLE_ORDINAL,
        ),
        _ => (
            FailurePhase::ChildProtocol,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        ),
    }
}

fn exit_failure_specification(
    status: ExitStatus,
    active: Option<(RunProfile, u32)>,
) -> FailureSpecification {
    if status.code() == Some(124) {
        if let Some((profile, command)) = active {
            return FailureSpecification::command_timeout(profile, command);
        }
    }
    FailureSpecification::protocol()
}

fn render_parent_failure(
    specification: FailureSpecification,
    status: ExitStatus,
    build_identity: Digest,
) -> Result<FailureReceipt, SupervisorError> {
    let observed_exit = exit_code(status);
    // Exit 124 corroborates only a timeout specification that originated at
    // an actually committed command (or its exact TIMEOUT terminal).  Phase
    // and code alone are insufficient: a CPU-liveness timeout in an
    // active-looking next phase could otherwise borrow command coordinates if
    // the child exits between the last status poll and reap.
    let child_exit_124_rejected = observed_exit == 124 && !specification.admits_child_exit_124;
    let specification = if child_exit_124_rejected {
        FailureSpecification::protocol()
    } else {
        specification
    };
    let child_exit = if child_exit_124_rejected {
        UNAVAILABLE_EXIT
    } else {
        observed_exit
    };
    let receipt = FailureReceipt {
        phase: specification.phase,
        code: specification.code,
        task_ordinal: specification.task_ordinal,
        subordinal: specification.subordinal,
        child_exit,
        native_status: specification.native_status,
        observed_mismatch: specification.observed_mismatch,
        build_identity,
        freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
        child_failure_frame_digest: ZERO_DIGEST,
    };
    FailureReceipt::decode(&receipt.encode()).map_err(SupervisorError::FailureCodec)
}

fn exit_code(status: ExitStatus) -> i32 {
    status.code().unwrap_or(UNAVAILABLE_EXIT)
}

fn kill_and_reap(child: &mut Child) -> Result<ExitStatus, SupervisorError> {
    match child.try_wait() {
        Ok(Some(status)) => return Ok(status),
        Ok(None) => {}
        Err(source) => {
            return Err(SupervisorError::ChildIo {
                operation: "status poll",
                source,
            });
        }
    }
    if let Err(source) = child.kill() {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) | Err(_) => {
                return Err(SupervisorError::ChildIo {
                    operation: "kill",
                    source,
                });
            }
        }
    }
    child.wait().map_err(|source| SupervisorError::ChildIo {
        operation: "reap",
        source,
    })
}

#[derive(Debug)]
enum ReaderEvent {
    Frame {
        bytes: Vec<u8>,
        received_at: Instant,
    },
    Eof {
        received_at: Instant,
    },
    Fault {
        fault: ReaderFault,
        received_at: Instant,
    },
}

#[derive(Debug)]
enum ReaderFault {
    Io,
    TruncatedPrefix,
    TruncatedPayload,
    MalformedHeader,
    Oversized,
    LengthOverflow,
    Allocation,
}

fn read_frames(mut stdout: ChildStdout, sender: SyncSender<ReaderEvent>) {
    loop {
        let mut prefix = [0u8; 4];
        match read_exact_or_eof(&mut stdout, &mut prefix) {
            Ok(ReadBoundary::CleanEof) => {
                let _ = sender.send(ReaderEvent::Eof {
                    received_at: Instant::now(),
                });
                return;
            }
            Ok(ReadBoundary::Complete) => {}
            Err(ReadFailure::Io) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::Io,
                    received_at: Instant::now(),
                });
                return;
            }
            Err(ReadFailure::Truncated) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::TruncatedPrefix,
                    received_at: Instant::now(),
                });
                return;
            }
        }
        let payload_len = u32::from_le_bytes(prefix);
        if payload_len > MAX_FRAME_PAYLOAD_BYTES || payload_len < FRAME_FIXED_PAYLOAD_BYTES as u32 {
            let _ = sender.send(ReaderEvent::Fault {
                fault: ReaderFault::Oversized,
                received_at: Instant::now(),
            });
            return;
        }
        let mut fixed_payload = [0u8; FRAME_FIXED_PAYLOAD_BYTES];
        match read_exact_or_eof(&mut stdout, &mut fixed_payload) {
            Ok(ReadBoundary::Complete) => {}
            Ok(ReadBoundary::CleanEof) | Err(ReadFailure::Truncated) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::TruncatedPayload,
                    received_at: Instant::now(),
                });
                return;
            }
            Err(ReadFailure::Io) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::Io,
                    received_at: Instant::now(),
                });
                return;
            }
        }
        let version = u16::from_le_bytes(fixed_payload[0..2].try_into().expect("fixed version"));
        let kind_value = u16::from_le_bytes(fixed_payload[2..4].try_into().expect("fixed kind"));
        let detail_len = u32::from_le_bytes(
            fixed_payload[12..16]
                .try_into()
                .expect("fixed detail length"),
        );
        let kind = match FrameKind::try_from(kind_value) {
            Ok(kind) => kind,
            Err(_) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::MalformedHeader,
                    received_at: Instant::now(),
                });
                return;
            }
        };
        let detail_extent_valid = match kind {
            FrameKind::Preparing
            | FrameKind::Committed
            | FrameKind::Terminal
            | FrameKind::Finalizing => detail_len == 0,
            FrameKind::Failure => detail_len == FAILURE_BYTES as u32,
            FrameKind::Success => {
                detail_len == SMOKE_BYTES as u32
                    || (detail_len >= SUCCESS_HEADER_BYTES as u32
                        && detail_len <= MAX_OFFICIAL_SUCCESS_DETAIL_BYTES)
            }
        };
        if version != 1
            || detail_len.checked_add(FRAME_FIXED_PAYLOAD_BYTES as u32) != Some(payload_len)
            || !detail_extent_valid
        {
            let _ = sender.send(ReaderEvent::Fault {
                fault: ReaderFault::MalformedHeader,
                received_at: Instant::now(),
            });
            return;
        }
        let Ok(detail_len) = usize::try_from(detail_len) else {
            let _ = sender.send(ReaderEvent::Fault {
                fault: ReaderFault::LengthOverflow,
                received_at: Instant::now(),
            });
            return;
        };
        let Some(frame_len) = FRAME_HEADER_BYTES.checked_add(detail_len) else {
            let _ = sender.send(ReaderEvent::Fault {
                fault: ReaderFault::LengthOverflow,
                received_at: Instant::now(),
            });
            return;
        };
        let mut frame = Vec::new();
        if frame.try_reserve_exact(frame_len).is_err() {
            let _ = sender.send(ReaderEvent::Fault {
                fault: ReaderFault::Allocation,
                received_at: Instant::now(),
            });
            return;
        }
        frame.resize(frame_len, 0);
        frame[..4].copy_from_slice(&prefix);
        frame[4..FRAME_HEADER_BYTES].copy_from_slice(&fixed_payload);
        match read_exact_or_eof(&mut stdout, &mut frame[FRAME_HEADER_BYTES..]) {
            Ok(ReadBoundary::Complete) => {}
            Ok(ReadBoundary::CleanEof) | Err(ReadFailure::Truncated) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::TruncatedPayload,
                    received_at: Instant::now(),
                });
                return;
            }
            Err(ReadFailure::Io) => {
                let _ = sender.send(ReaderEvent::Fault {
                    fault: ReaderFault::Io,
                    received_at: Instant::now(),
                });
                return;
            }
        }
        let received_at = Instant::now();
        if sender
            .send(ReaderEvent::Frame {
                bytes: frame,
                received_at,
            })
            .is_err()
        {
            return;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReadBoundary {
    Complete,
    CleanEof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReadFailure {
    Io,
    Truncated,
}

fn read_exact_or_eof(
    reader: &mut impl Read,
    bytes: &mut [u8],
) -> Result<ReadBoundary, ReadFailure> {
    let mut offset = 0;
    while offset < bytes.len() {
        match reader.read(&mut bytes[offset..]) {
            Ok(0) if offset == 0 => return Ok(ReadBoundary::CleanEof),
            Ok(0) => return Err(ReadFailure::Truncated),
            Ok(count) => offset += count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(ReadFailure::Io),
        }
    }
    Ok(ReadBoundary::Complete)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandStep {
    Committed,
    Terminal,
    PostCommand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MachineState {
    Preparing(u32),
    Commands {
        command: u32,
        step: CommandStep,
    },
    CommandFailure {
        command: u32,
        terminal: TerminalCode,
    },
    FinalBindings(u32),
    FinalSections(u32),
    ExpectSuccess,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProtocolMachine {
    profile: RunProfile,
    state: MachineState,
}

#[derive(Debug)]
enum Accepted {
    CpuProgress,
    CommandCommitted(u32),
    CommandCompleted(u32),
    CommandFailed {
        command: u32,
        terminal: TerminalCode,
    },
    Success(SupervisedSuccess),
    ChildFailure,
}

#[derive(Debug)]
struct ProtocolViolation;

impl ProtocolMachine {
    const fn new(profile: RunProfile) -> Self {
        Self {
            profile,
            state: MachineState::Preparing(0),
        }
    }

    fn accept(
        &mut self,
        frame: &WireFrame,
        build_identity: Digest,
    ) -> Result<Accepted, ProtocolViolation> {
        if self.state == MachineState::Done {
            return Err(ProtocolViolation);
        }
        if frame.kind == FrameKind::Failure {
            if matches!(
                self.state,
                MachineState::Commands {
                    step: CommandStep::Terminal,
                    ..
                }
            ) || frame.phase_or_command_ordinal != 0
                || frame.unit_or_terminal_code != 0
            {
                return Err(ProtocolViolation);
            }
            let failure = FailureReceipt::decode(&frame.detail).map_err(|_| ProtocolViolation)?;
            if failure.child_failure_frame_digest != ZERO_DIGEST
                || failure.build_identity != build_identity
                || failure.child_exit != UNAVAILABLE_EXIT
                || failure.code == FailureCode::Timeout
            {
                return Err(ProtocolViolation);
            }
            match self.state {
                MachineState::CommandFailure { command, terminal } => {
                    validate_command_failure(self.profile, command, terminal, &failure)?;
                }
                MachineState::Commands {
                    command,
                    step: CommandStep::Committed | CommandStep::PostCommand,
                } => validate_active_command_failure(self.profile, command, &failure)?,
                _ => {}
            }
            self.state = MachineState::Done;
            return Ok(Accepted::ChildFailure);
        }

        match self.state {
            MachineState::Preparing(next) => {
                let expected = expected_preparing(self.profile, next).ok_or(ProtocolViolation)?;
                require_frame(frame, FrameKind::Preparing, expected.0, expected.1)?;
                let preparing_count = preparing_count(self.profile);
                self.state = if next + 1 == preparing_count {
                    MachineState::Commands {
                        command: 0,
                        step: CommandStep::Committed,
                    }
                } else {
                    MachineState::Preparing(next + 1)
                };
                Ok(Accepted::CpuProgress)
            }
            MachineState::Commands { command, step } => match step {
                CommandStep::Committed => {
                    require_frame(frame, FrameKind::Committed, command, 0)?;
                    self.state = MachineState::Commands {
                        command,
                        step: CommandStep::Terminal,
                    };
                    Ok(Accepted::CommandCommitted(command))
                }
                CommandStep::Terminal => {
                    if frame.kind != FrameKind::Terminal
                        || frame.phase_or_command_ordinal != command
                    {
                        return Err(ProtocolViolation);
                    }
                    let terminal = TerminalCode::try_from(frame.unit_or_terminal_code)
                        .map_err(|_| ProtocolViolation)?;
                    if terminal != TerminalCode::Completed {
                        self.state = if terminal == TerminalCode::Timeout {
                            MachineState::Done
                        } else {
                            MachineState::CommandFailure { command, terminal }
                        };
                        return Ok(Accepted::CommandFailed { command, terminal });
                    }
                    self.state = MachineState::Commands {
                        command,
                        step: CommandStep::PostCommand,
                    };
                    Ok(Accepted::CommandCompleted(command))
                }
                CommandStep::PostCommand => {
                    require_frame(frame, FrameKind::Preparing, 9, command)?;
                    let next = command + 1;
                    self.state = if next < command_count(self.profile) {
                        MachineState::Commands {
                            command: next,
                            step: CommandStep::Committed,
                        }
                    } else if self.profile == RunProfile::Official {
                        MachineState::FinalBindings(0)
                    } else {
                        MachineState::ExpectSuccess
                    };
                    Ok(Accepted::CpuProgress)
                }
            },
            MachineState::FinalBindings(unit) => {
                require_frame(frame, FrameKind::Finalizing, 7, unit)?;
                self.state = if unit + 1 == OFFICIAL_BINDING_COUNT {
                    MachineState::FinalSections(0)
                } else {
                    MachineState::FinalBindings(unit + 1)
                };
                Ok(Accepted::CpuProgress)
            }
            MachineState::FinalSections(unit) => {
                require_frame(frame, FrameKind::Finalizing, 8, unit)?;
                self.state = if unit + 1 == OFFICIAL_SECTION_COUNT {
                    MachineState::ExpectSuccess
                } else {
                    MachineState::FinalSections(unit + 1)
                };
                Ok(Accepted::CpuProgress)
            }
            MachineState::ExpectSuccess => {
                if frame.kind != FrameKind::Success {
                    return Err(ProtocolViolation);
                }
                let success = match self.profile {
                    RunProfile::Smoke => SupervisedSuccess::Smoke(
                        SmokeReport::decode(&frame.detail).map_err(|_| ProtocolViolation)?,
                    ),
                    RunProfile::Official => {
                        let receipt =
                            SuccessReceipt::decode(&frame.detail).map_err(|_| ProtocolViolation)?;
                        if receipt.build_identity != build_identity {
                            return Err(ProtocolViolation);
                        }
                        SupervisedSuccess::Official(Box::new(receipt))
                    }
                };
                self.state = MachineState::Done;
                Ok(Accepted::Success(success))
            }
            MachineState::CommandFailure { .. } => Err(ProtocolViolation),
            MachineState::Done => Err(ProtocolViolation),
        }
    }

    fn failure_phase(self) -> FailurePhase {
        match self.state {
            MachineState::Preparing(next) => preparing_failure_phase(self.profile, next),
            MachineState::Commands { command, .. } => {
                command_failure_coordinates(self.profile, command).0
            }
            MachineState::CommandFailure { command, .. } => {
                command_failure_coordinates(self.profile, command).0
            }
            MachineState::FinalBindings(_) => FailurePhase::Bindings,
            MachineState::FinalSections(_) | MachineState::ExpectSuccess => {
                FailurePhase::ReceiptRender
            }
            MachineState::Done => FailurePhase::ChildProtocol,
        }
    }
}

fn validate_active_command_failure(
    profile: RunProfile,
    command: u32,
    failure: &FailureReceipt,
) -> Result<(), ProtocolViolation> {
    let (phase, task_ordinal, subordinal) = command_failure_coordinates(profile, command);
    if failure.phase == phase
        && failure.task_ordinal == task_ordinal
        && failure.subordinal == subordinal
    {
        Ok(())
    } else {
        Err(ProtocolViolation)
    }
}

fn validate_command_failure(
    profile: RunProfile,
    command: u32,
    terminal: TerminalCode,
    failure: &FailureReceipt,
) -> Result<(), ProtocolViolation> {
    let (phase, task_ordinal, subordinal) = command_failure_coordinates(profile, command);
    if failure.phase != phase
        || failure.task_ordinal != task_ordinal
        || failure.subordinal != subordinal
        || failure.child_exit != UNAVAILABLE_EXIT
        || failure.observed_mismatch != 0
    {
        return Err(ProtocolViolation);
    }
    let compatible = match terminal {
        TerminalCode::Error => {
            failure.native_status == NATIVE_ERROR
                && matches!(
                    failure.code,
                    FailureCode::CommandError | FailureCode::CommandStateFailure
                )
        }
        TerminalCode::NotEnqueued => {
            failure.native_status == NATIVE_NOT_ENQUEUED
                && failure.code == FailureCode::CommandStateFailure
        }
        TerminalCode::NotCommitted => {
            failure.native_status == NATIVE_ENQUEUED
                && failure.code == FailureCode::CommandStateFailure
        }
        TerminalCode::Scheduled => {
            failure.native_status == NATIVE_SCHEDULED
                && failure.code == FailureCode::CommandStateFailure
        }
        TerminalCode::Unknown => failure.code == FailureCode::CommandStateFailure,
        TerminalCode::Completed | TerminalCode::Timeout => false,
    };
    if compatible {
        Ok(())
    } else {
        Err(ProtocolViolation)
    }
}

fn require_frame(
    frame: &WireFrame,
    kind: FrameKind,
    phase_or_command: u32,
    unit: u32,
) -> Result<(), ProtocolViolation> {
    if frame.kind == kind
        && frame.phase_or_command_ordinal == phase_or_command
        && frame.unit_or_terminal_code == unit
    {
        Ok(())
    } else {
        Err(ProtocolViolation)
    }
}

const fn preparing_count(profile: RunProfile) -> u32 {
    match profile {
        RunProfile::Smoke => 4,
        RunProfile::Official => OFFICIAL_PREPARING_COUNT,
    }
}

const fn command_count(profile: RunProfile) -> u32 {
    match profile {
        RunProfile::Smoke => SMOKE_COMMAND_COUNT,
        RunProfile::Official => OFFICIAL_COMMAND_COUNT,
    }
}

const fn expected_preparing(profile: RunProfile, next: u32) -> Option<(u32, u32)> {
    match profile {
        RunProfile::Smoke => match next {
            0 => Some((2, 0)),
            1 => Some((3, 0)),
            2 => Some((5, 0)),
            3 => Some((6, 0)),
            _ => None,
        },
        RunProfile::Official => match next {
            0 => Some((1, 0)),
            1 => Some((2, 0)),
            2 => Some((3, 0)),
            3..=616 => Some((4, next - 3)),
            617..=1_230 => Some((5, next - 617)),
            1_231 => Some((6, 0)),
            _ => None,
        },
    }
}

const fn preparing_failure_phase(profile: RunProfile, next: u32) -> FailurePhase {
    match expected_preparing(profile, next) {
        Some((1, _)) => FailurePhase::Historical,
        Some((2, _)) => FailurePhase::MetalToolchain,
        Some((3, _)) => FailurePhase::Tables,
        Some((4, _)) | Some((5, _)) => FailurePhase::CarrierPreflight,
        Some((6, _)) => FailurePhase::Gate0,
        _ => FailurePhase::ChildProtocol,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, OpenOptions};
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ORDINAL: AtomicU64 = AtomicU64::new(0);
    const TEST_BUILD: Digest = [0x42; 32];

    struct TempBytes {
        path: PathBuf,
        device: u64,
        inode: u64,
    }

    impl TempBytes {
        fn new(bytes: &[u8]) -> Self {
            Self::new_with_first_path(bytes, None)
        }

        fn new_with_first_path(bytes: &[u8], mut first_path: Option<PathBuf>) -> Self {
            loop {
                let path = first_path.take().unwrap_or_else(|| {
                    let ordinal = TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed);
                    std::env::temp_dir().join(format!(
                        "walt-m2-protocol-{}-{ordinal}.bin",
                        std::process::id()
                    ))
                });
                let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
                    Ok(file) => file,
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("create synthetic M2 child stream: {error}"),
                };
                file.write_all(bytes)
                    .expect("write synthetic M2 child stream");
                file.sync_all().expect("sync synthetic M2 child stream");
                let metadata = file.metadata().expect("inspect synthetic M2 child stream");
                assert!(metadata.file_type().is_file());
                return Self {
                    path,
                    device: metadata.dev(),
                    inode: metadata.ino(),
                };
            }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempBytes {
        fn drop(&mut self) {
            let Ok(metadata) = fs::symlink_metadata(&self.path) else {
                return;
            };
            if metadata.file_type().is_file()
                && !metadata.file_type().is_symlink()
                && metadata.dev() == self.device
                && metadata.ino() == self.inode
            {
                let _ = fs::remove_file(&self.path);
            }
        }
    }

    #[test]
    fn synthetic_stream_creation_never_follows_or_removes_an_occupied_symlink() {
        use std::os::unix::fs::symlink;

        let ordinal = TEMP_ORDINAL.fetch_add(1, Ordering::Relaxed);
        let target = std::env::temp_dir().join(format!(
            "walt-m2-protocol-symlink-target-{}-{ordinal}.bin",
            std::process::id()
        ));
        let occupied = std::env::temp_dir().join(format!(
            "walt-m2-protocol-symlink-{}-{ordinal}.bin",
            std::process::id()
        ));
        fs::write(&target, b"do not overwrite").expect("write symlink target");
        symlink(&target, &occupied).expect("create occupied synthetic-stream path");

        let stream = TempBytes::new_with_first_path(b"new stream", Some(occupied.clone()));
        assert_ne!(stream.path(), occupied);
        assert_eq!(
            fs::read(&target).expect("read symlink target"),
            b"do not overwrite"
        );
        assert!(fs::symlink_metadata(&occupied)
            .expect("inspect occupied path")
            .file_type()
            .is_symlink());

        fs::remove_file(&occupied).expect("remove occupied symlink");
        fs::remove_file(&target).expect("remove symlink target");
    }

    fn frame(kind: FrameKind, phase_or_command: u32, unit: u32) -> WireFrame {
        WireFrame {
            kind,
            phase_or_command_ordinal: phase_or_command,
            unit_or_terminal_code: unit,
            detail: Vec::new(),
        }
    }

    fn encoded_frame(kind: FrameKind, phase_or_command: u32, unit: u32) -> Vec<u8> {
        frame(kind, phase_or_command, unit)
            .encode()
            .expect("encode synthetic frame")
    }

    fn smoke_prefix_through_commit(command: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        for phase in [2, 3, 5, 6] {
            bytes.extend_from_slice(&encoded_frame(FrameKind::Preparing, phase, 0));
        }
        for completed in 0..command {
            bytes.extend_from_slice(&encoded_frame(FrameKind::Committed, completed, 0));
            bytes.extend_from_slice(&encoded_frame(
                FrameKind::Terminal,
                completed,
                u32::from(TerminalCode::Completed),
            ));
            bytes.extend_from_slice(&encoded_frame(FrameKind::Preparing, 9, completed));
        }
        bytes.extend_from_slice(&encoded_frame(FrameKind::Committed, command, 0));
        bytes
    }

    fn smoke_success_stream() -> Vec<u8> {
        let mut bytes = smoke_prefix_through_commit(0);
        bytes.extend_from_slice(&encoded_frame(
            FrameKind::Terminal,
            0,
            u32::from(TerminalCode::Completed),
        ));
        bytes.extend_from_slice(&encoded_frame(FrameKind::Preparing, 9, 0));
        bytes.extend_from_slice(&encoded_frame(FrameKind::Committed, 1, 0));
        bytes.extend_from_slice(&encoded_frame(
            FrameKind::Terminal,
            1,
            u32::from(TerminalCode::Completed),
        ));
        bytes.extend_from_slice(&encoded_frame(FrameKind::Preparing, 9, 1));
        bytes.extend_from_slice(
            &WireFrame {
                kind: FrameKind::Success,
                phase_or_command_ordinal: 0,
                unit_or_terminal_code: 0,
                detail: SmokeReport.encode().to_vec(),
            }
            .encode()
            .expect("encode smoke success"),
        );
        bytes
    }

    fn send_reader_frame(
        sender: &std::sync::mpsc::Sender<ReaderEvent>,
        bytes: Vec<u8>,
        received_at: Instant,
    ) {
        sender
            .send(ReaderEvent::Frame { bytes, received_at })
            .expect("queue synthetic reader frame");
    }

    fn queue_smoke_through_commit(
        sender: &std::sync::mpsc::Sender<ReaderEvent>,
        received_at: Instant,
    ) {
        for phase in [2, 3, 5, 6] {
            send_reader_frame(
                sender,
                encoded_frame(FrameKind::Preparing, phase, 0),
                received_at,
            );
        }
        send_reader_frame(
            sender,
            encoded_frame(FrameKind::Committed, 0, 0),
            received_at,
        );
    }

    fn queue_smoke_through_post_command(
        sender: &std::sync::mpsc::Sender<ReaderEvent>,
        received_at: Instant,
    ) {
        queue_smoke_through_commit(sender, received_at);
        send_reader_frame(
            sender,
            encoded_frame(FrameKind::Terminal, 0, u32::from(TerminalCode::Completed)),
            received_at,
        );
        send_reader_frame(
            sender,
            encoded_frame(FrameKind::Preparing, 9, 0),
            received_at,
        );
    }

    fn child_failure_frame(
        phase: FailurePhase,
        code: FailureCode,
        task_ordinal: u32,
        subordinal: u32,
    ) -> Vec<u8> {
        child_failure_frame_with_identity_and_coordinates(
            phase,
            code,
            task_ordinal,
            subordinal,
            TEST_BUILD,
            0,
            0,
        )
    }

    fn command_failure_frame(
        phase: FailurePhase,
        code: FailureCode,
        task_ordinal: u32,
        native_status: u32,
    ) -> Vec<u8> {
        let failure = FailureReceipt {
            phase,
            code,
            task_ordinal,
            subordinal: UNAVAILABLE_ORDINAL,
            child_exit: UNAVAILABLE_EXIT,
            native_status,
            observed_mismatch: 0,
            build_identity: TEST_BUILD,
            freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
            child_failure_frame_digest: ZERO_DIGEST,
        };
        WireFrame {
            kind: FrameKind::Failure,
            phase_or_command_ordinal: 0,
            unit_or_terminal_code: 0,
            detail: failure.encode_child_zeroed().unwrap().to_vec(),
        }
        .encode()
        .unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn child_failure_frame_with_identity_and_coordinates(
        phase: FailurePhase,
        code: FailureCode,
        task_ordinal: u32,
        subordinal: u32,
        build_identity: Digest,
        outer_phase: u32,
        outer_unit: u32,
    ) -> Vec<u8> {
        let failure = FailureReceipt {
            phase,
            code,
            task_ordinal,
            subordinal,
            child_exit: UNAVAILABLE_EXIT,
            native_status: UNAVAILABLE_STATUS,
            observed_mismatch: 0,
            build_identity,
            freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
            child_failure_frame_digest: ZERO_DIGEST,
        };
        WireFrame {
            kind: FrameKind::Failure,
            phase_or_command_ordinal: outer_phase,
            unit_or_terminal_code: outer_unit,
            detail: failure
                .encode_child_zeroed()
                .expect("encode child failure")
                .to_vec(),
        }
        .encode()
        .expect("encode child failure frame")
    }

    fn spawn_cat(file: &TempBytes) -> Child {
        Command::new("/bin/cat")
            .arg(file.path())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic cat child")
    }

    fn spawn_cat_then_sleep(file: &TempBytes) -> Child {
        Command::new("/bin/sh")
            .args(["-c", "/bin/cat \"$1\"; exec /bin/sleep 5", "m2-test"])
            .arg(file.path())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic sleeping child")
    }

    fn spawn_cat_close_stdout_then_sleep(file: &TempBytes) -> Child {
        Command::new("/bin/sh")
            .args([
                "-c",
                "/bin/cat \"$1\"; exec 1>&-; exec /bin/sleep 5",
                "m2-test",
            ])
            .arg(file.path())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic child that closes stdout")
    }

    fn spawn_cat_then_exit(file: &TempBytes, code: u8) -> Child {
        Command::new("/bin/sh")
            .args(["-c", "/bin/cat \"$1\"; exit \"$2\"", "m2-test"])
            .arg(file.path())
            .arg(code.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic exiting child")
    }

    fn spawn_exit(code: u8) -> Child {
        Command::new("/bin/sh")
            .args(["-c", "exit \"$1\"", "m2-test"])
            .arg(code.to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic empty exiting child")
    }

    fn spawn_silent_sleep() -> Child {
        Command::new("/bin/sleep")
            .arg("5")
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic silent child")
    }

    fn spawn_exited_child_with_stdout_holder() -> Child {
        Command::new("/bin/sh")
            .args(["-c", "/bin/sleep 5 & exit 1", "m2-test"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn exited child with inherited stdout holder")
    }

    fn spawn_true() -> Child {
        Command::new("/usr/bin/true")
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn synthetic successful child")
    }

    fn short_deadlines() -> SupervisorDeadlines {
        SupervisorDeadlines {
            poll_interval: Duration::from_millis(1),
            command: Duration::from_millis(80),
            cpu_liveness: Duration::from_millis(300),
        }
    }

    fn semantic_deadlines() -> SupervisorDeadlines {
        // These tests adjudicate complete bytes and exit status rather than a
        // watchdog boundary.  Give the live reader/process integration ample
        // scheduling room so host contention cannot manufacture a different
        // semantic result.  Timeout tests continue to use short_deadlines().
        SupervisorDeadlines {
            poll_interval: Duration::from_millis(1),
            command: Duration::from_secs(5),
            cpu_liveness: Duration::from_secs(5),
        }
    }

    fn expect_failure(outcome: SupervisedOutcome) -> FailureReceipt {
        match outcome {
            SupervisedOutcome::Failure(failure) => failure,
            SupervisedOutcome::Success(success) => panic!("unexpected success: {success:?}"),
        }
    }

    #[test]
    fn smoke_machine_accepts_only_the_exact_closed_sequence() {
        let mut machine = ProtocolMachine::new(RunProfile::Smoke);
        for phase in [2, 3, 5, 6] {
            assert!(matches!(
                machine.accept(&frame(FrameKind::Preparing, phase, 0), TEST_BUILD),
                Ok(Accepted::CpuProgress)
            ));
        }
        for command in 0..2 {
            assert!(matches!(
                machine.accept(&frame(FrameKind::Committed, command, 0), TEST_BUILD),
                Ok(Accepted::CommandCommitted(observed)) if observed == command
            ));
            assert!(matches!(
                machine.accept(
                    &frame(
                        FrameKind::Terminal,
                        command,
                        u32::from(TerminalCode::Completed),
                    ),
                    TEST_BUILD,
                ),
                Ok(Accepted::CommandCompleted(observed)) if observed == command
            ));
            assert!(matches!(
                machine.accept(&frame(FrameKind::Preparing, 9, command), TEST_BUILD),
                Ok(Accepted::CpuProgress)
            ));
        }
        let success = WireFrame {
            kind: FrameKind::Success,
            phase_or_command_ordinal: 0,
            unit_or_terminal_code: 0,
            detail: SmokeReport.encode().to_vec(),
        };
        assert!(matches!(
            machine.accept(&success, TEST_BUILD),
            Ok(Accepted::Success(SupervisedSuccess::Smoke(_)))
        ));
        assert!(machine.accept(&success, TEST_BUILD).is_err());

        let mut reordered = ProtocolMachine::new(RunProfile::Smoke);
        assert!(reordered
            .accept(&frame(FrameKind::Preparing, 3, 0), TEST_BUILD)
            .is_err());
    }

    #[test]
    fn official_machine_reaches_success_gate_only_after_every_frozen_ordinal() {
        let mut machine = ProtocolMachine::new(RunProfile::Official);
        for next in 0..OFFICIAL_PREPARING_COUNT {
            let (phase, unit) = expected_preparing(RunProfile::Official, next).unwrap();
            assert!(machine
                .accept(&frame(FrameKind::Preparing, phase, unit), TEST_BUILD)
                .is_ok());
        }
        for command in 0..OFFICIAL_COMMAND_COUNT {
            machine
                .accept(&frame(FrameKind::Committed, command, 0), TEST_BUILD)
                .unwrap();
            machine
                .accept(
                    &frame(
                        FrameKind::Terminal,
                        command,
                        u32::from(TerminalCode::Completed),
                    ),
                    TEST_BUILD,
                )
                .unwrap();
            machine
                .accept(&frame(FrameKind::Preparing, 9, command), TEST_BUILD)
                .unwrap();
        }
        for unit in 0..OFFICIAL_BINDING_COUNT {
            machine
                .accept(&frame(FrameKind::Finalizing, 7, unit), TEST_BUILD)
                .unwrap();
        }
        for unit in 0..OFFICIAL_SECTION_COUNT {
            machine
                .accept(&frame(FrameKind::Finalizing, 8, unit), TEST_BUILD)
                .unwrap();
        }
        assert_eq!(machine.state, MachineState::ExpectSuccess);

        let smoke_success = WireFrame {
            kind: FrameKind::Success,
            phase_or_command_ordinal: 0,
            unit_or_terminal_code: 0,
            detail: SmokeReport.encode().to_vec(),
        };
        assert!(machine.accept(&smoke_success, TEST_BUILD).is_err());
    }

    #[test]
    fn smoke_supervisor_returns_success_only_after_clean_eof_and_zero_exit() {
        let file = TempBytes::new(&smoke_success_stream());
        let outcome = supervise_child_with_deadlines(
            spawn_cat(&file),
            RunProfile::Smoke,
            TEST_BUILD,
            semantic_deadlines(),
        )
        .unwrap();
        assert_eq!(
            outcome,
            SupervisedOutcome::Success(SupervisedSuccess::Smoke(SmokeReport))
        );
    }

    #[test]
    fn trailing_or_multiple_success_never_returns_partial_success() {
        for suffix in [vec![0xa5], smoke_success_stream()] {
            let mut bytes = smoke_success_stream();
            bytes.extend_from_slice(&suffix);
            let file = TempBytes::new(&bytes);
            let failure = expect_failure(
                supervise_child_with_deadlines(
                    spawn_cat(&file),
                    RunProfile::Smoke,
                    TEST_BUILD,
                    semantic_deadlines(),
                )
                .unwrap(),
            );
            assert_eq!(failure.phase, FailurePhase::ChildProtocol);
            assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
            assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
        }
    }

    #[test]
    fn complete_success_bytes_from_a_child_that_does_not_exit_are_not_success() {
        let file = TempBytes::new(&smoke_success_stream());
        let started = Instant::now();
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_sleep(&file),
                RunProfile::Smoke,
                TEST_BUILD,
                short_deadlines(),
            )
            .unwrap(),
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
    }

    #[test]
    fn closed_stdout_from_a_live_child_still_reaches_the_watchdog() {
        let file = TempBytes::new(&smoke_success_stream());
        let started = Instant::now();
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_close_stdout_then_sleep(&file),
                RunProfile::Smoke,
                TEST_BUILD,
                short_deadlines(),
            )
            .unwrap(),
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
    }

    #[test]
    fn malformed_and_truncated_frames_are_protocol_failure() {
        let mut contradictory_huge_detail = Vec::with_capacity(FRAME_HEADER_BYTES);
        contradictory_huge_detail
            .extend_from_slice(&(FRAME_FIXED_PAYLOAD_BYTES as u32).to_le_bytes());
        contradictory_huge_detail.extend_from_slice(&1u16.to_le_bytes());
        contradictory_huge_detail.extend_from_slice(&u16::from(FrameKind::Success).to_le_bytes());
        contradictory_huge_detail.extend_from_slice(&0u32.to_le_bytes());
        contradictory_huge_detail.extend_from_slice(&0u32.to_le_bytes());
        contradictory_huge_detail.extend_from_slice(&u32::MAX.to_le_bytes());
        for bytes in [
            vec![0x10, 0, 0],
            vec![0x20, 0, 0, 0, 1, 0, 1],
            vec![0xff, 0xff, 0xff, 0xff],
            contradictory_huge_detail,
        ] {
            let file = TempBytes::new(&bytes);
            let failure = expect_failure(
                supervise_child_with_deadlines(
                    spawn_cat(&file),
                    RunProfile::Smoke,
                    TEST_BUILD,
                    semantic_deadlines(),
                )
                .unwrap(),
            );
            assert_eq!(failure.phase, FailurePhase::ChildProtocol);
            assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        }
    }

    #[test]
    fn committed_deadline_is_short_in_tests_and_maps_the_active_command() {
        let file = TempBytes::new(&smoke_prefix_through_commit(0));
        let started = Instant::now();
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_sleep(&file),
                RunProfile::Smoke,
                TEST_BUILD,
                short_deadlines(),
            )
            .unwrap(),
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(failure.phase, FailurePhase::Gate0);
        assert_eq!(failure.code, FailureCode::Timeout);
        assert_eq!(failure.native_status, UNAVAILABLE_STATUS);
    }

    #[test]
    fn queued_committed_frame_keeps_its_reader_receipt_deadline() {
        let (sender, receiver) = std::sync::mpsc::channel();
        let received_base = Instant::now() - Duration::from_millis(200);
        for phase in [2, 3, 5, 6] {
            sender
                .send(ReaderEvent::Frame {
                    bytes: encoded_frame(FrameKind::Preparing, phase, 0),
                    received_at: received_base,
                })
                .unwrap();
        }
        sender
            .send(ReaderEvent::Frame {
                bytes: encoded_frame(FrameKind::Committed, 0, 0),
                received_at: received_base + Duration::from_millis(10),
            })
            .unwrap();
        let mut child = spawn_silent_sleep();
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            short_deadlines(),
        );
        drop(sender);
        kill_and_reap(&mut child).unwrap();

        match result {
            LoopResult::Abort(specification) => {
                assert_eq!(specification.phase, FailurePhase::Gate0);
                assert_eq!(specification.code, FailureCode::Timeout);
            }
            other => panic!("queued COMMITTED unexpectedly escaped its deadline: {other:?}"),
        }
    }

    #[test]
    fn timely_queued_terminals_survive_parent_descheduling() {
        let (sender, receiver) = std::sync::mpsc::channel();
        let received_base = Instant::now() - Duration::from_millis(200);
        {
            let send_frame = |bytes, offset_ms| {
                sender
                    .send(ReaderEvent::Frame {
                        bytes,
                        received_at: received_base + Duration::from_millis(offset_ms),
                    })
                    .unwrap();
            };
            for phase in [2, 3, 5, 6] {
                send_frame(encoded_frame(FrameKind::Preparing, phase, 0), 0);
            }
            send_frame(encoded_frame(FrameKind::Committed, 0, 0), 4);
            send_frame(
                encoded_frame(FrameKind::Terminal, 0, u32::from(TerminalCode::Completed)),
                50,
            );
            send_frame(encoded_frame(FrameKind::Preparing, 9, 0), 55);
            send_frame(encoded_frame(FrameKind::Committed, 1, 0), 60);
            send_frame(
                encoded_frame(FrameKind::Terminal, 1, u32::from(TerminalCode::Completed)),
                100,
            );
            send_frame(encoded_frame(FrameKind::Preparing, 9, 1), 105);
            send_frame(
                WireFrame {
                    kind: FrameKind::Success,
                    phase_or_command_ordinal: 0,
                    unit_or_terminal_code: 0,
                    detail: SmokeReport.encode().to_vec(),
                }
                .encode()
                .unwrap(),
                110,
            );
        }
        sender
            .send(ReaderEvent::Eof {
                received_at: received_base + Duration::from_millis(115),
            })
            .unwrap();
        drop(sender);

        let mut child = spawn_true();
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            short_deadlines(),
        );
        match result {
            LoopResult::Complete {
                status,
                candidate: Some(Candidate::Success(SupervisedSuccess::Smoke(_))),
            } => assert!(status.success()),
            other => panic!("timely queued terminal was not accepted: {other:?}"),
        }
    }

    #[test]
    fn production_reader_timestamps_complete_stream_before_delayed_supervision() {
        let file = TempBytes::new(&smoke_success_stream());
        let mut child = spawn_cat(&file);
        let stdout = child.stdout.take().unwrap();
        let (sender, receiver) = std::sync::mpsc::sync_channel(OFFICIAL_STREAM_QUEUE_CAPACITY);
        let reader = thread::spawn(move || read_frames(stdout, sender));

        // Prove that the closed-capacity queue can hold the entire stream by
        // letting the reader finish before supervision begins.  A blind sleep
        // here would test host scheduling instead of the queue invariant.
        reader.join().unwrap();

        // Longer than the synthetic command deadline. A one-slot reader queue
        // would leave TERMINAL in the pipe and timestamp it too late.
        thread::sleep(Duration::from_millis(120));
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            short_deadlines(),
        );
        drop(receiver);
        match result {
            LoopResult::Complete {
                status,
                candidate: Some(Candidate::Success(SupervisedSuccess::Smoke(_))),
            } => assert!(status.success()),
            other => panic!("production reader delayed a timely terminal: {other:?}"),
        }
    }

    #[test]
    fn late_queued_terminal_cannot_extend_command_deadline() {
        let (sender, receiver) = std::sync::mpsc::channel();
        let received_base = Instant::now() - Duration::from_millis(200);
        for phase in [2, 3, 5, 6] {
            sender
                .send(ReaderEvent::Frame {
                    bytes: encoded_frame(FrameKind::Preparing, phase, 0),
                    received_at: received_base,
                })
                .unwrap();
        }
        sender
            .send(ReaderEvent::Frame {
                bytes: encoded_frame(FrameKind::Committed, 0, 0),
                received_at: received_base + Duration::from_millis(10),
            })
            .unwrap();
        sender
            .send(ReaderEvent::Frame {
                bytes: encoded_frame(FrameKind::Terminal, 0, u32::from(TerminalCode::Completed)),
                received_at: received_base + Duration::from_millis(100),
            })
            .unwrap();
        drop(sender);

        let mut child = spawn_silent_sleep();
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            short_deadlines(),
        );
        kill_and_reap(&mut child).unwrap();
        match result {
            LoopResult::Abort(specification) => {
                assert_eq!(specification.phase, FailurePhase::Gate0);
                assert_eq!(specification.code, FailureCode::Timeout);
            }
            other => panic!("late terminal escaped its command deadline: {other:?}"),
        }
    }

    #[test]
    fn cpu_liveness_timeout_is_separate_and_short_in_tests() {
        let started = Instant::now();
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_silent_sleep(),
                RunProfile::Smoke,
                TEST_BUILD,
                short_deadlines(),
            )
            .unwrap(),
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(failure.phase, FailurePhase::MetalToolchain);
        assert_eq!(failure.code, FailureCode::Timeout);
    }

    #[test]
    fn inherited_stdout_holder_cannot_block_abort_reaping() {
        let mut child = spawn_exited_child_with_stdout_holder();
        assert_eq!(child.wait().expect("pre-reap direct child").code(), Some(1));
        let started = Instant::now();
        let failure = expect_failure(
            supervise_child_with_deadlines(child, RunProfile::Smoke, TEST_BUILD, short_deadlines())
                .unwrap(),
        );
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_exit, 1);
    }

    #[test]
    fn active_exit_124_is_rendered_as_the_exact_timeout_phase() {
        let file = TempBytes::new(&smoke_prefix_through_commit(0));
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 124),
                RunProfile::Smoke,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.phase, FailurePhase::Gate0);
        assert_eq!(failure.code, FailureCode::Timeout);
        assert_eq!(failure.child_exit, 124);
        FailureReceipt::decode(&failure.encode()).unwrap();
    }

    #[test]
    fn noncompleted_terminal_waits_for_and_hashes_child_failure() {
        let mut bytes = smoke_prefix_through_commit(0);
        bytes.extend_from_slice(&encoded_frame(
            FrameKind::Terminal,
            0,
            u32::from(TerminalCode::Error),
        ));
        let child_failure = command_failure_frame(
            FailurePhase::Gate0,
            FailureCode::CommandError,
            UNAVAILABLE_ORDINAL,
            NATIVE_ERROR,
        );
        bytes.extend_from_slice(&child_failure);
        let file = TempBytes::new(&bytes);
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 1),
                RunProfile::Smoke,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.phase, FailurePhase::Gate0);
        assert_eq!(failure.code, FailureCode::CommandError);
        assert_eq!(failure.native_status, NATIVE_ERROR);
        assert_eq!(failure.child_exit, 1);
        assert_eq!(
            failure.child_failure_frame_digest,
            walt_gpu_ref::m2_receipt::sha256(&child_failure)
        );
    }

    #[test]
    fn terminal_timeout_expects_exit_124_without_a_failure_frame() {
        let mut bytes = smoke_prefix_through_commit(0);
        bytes.extend_from_slice(&encoded_frame(
            FrameKind::Terminal,
            0,
            u32::from(TerminalCode::Timeout),
        ));
        let file = TempBytes::new(&bytes);
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 124),
                RunProfile::Smoke,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.phase, FailurePhase::Gate0);
        assert_eq!(failure.code, FailureCode::Timeout);
        assert_eq!(failure.child_exit, 124);
        assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
    }

    #[test]
    fn exit_124_after_a_completed_command_cannot_borrow_stale_command_state() {
        let (sender, receiver) = std::sync::mpsc::channel();
        let received_at = Instant::now();
        queue_smoke_through_post_command(&sender, received_at);
        sender
            .send(ReaderEvent::Eof { received_at })
            .expect("queue synthetic EOF");
        drop(sender);

        let mut child = spawn_exit(124);
        child.wait().expect("pre-reap synthetic exit 124");
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            semantic_deadlines(),
        );
        let failure = match result {
            LoopResult::Abort(specification) => {
                let status = child.wait().expect("recover cached exit-124 status");
                render_parent_failure(specification, status, TEST_BUILD)
                    .expect("render deterministic post-command exit")
            }
            other => panic!("post-command exit-124 was not rejected: {other:?}"),
        };
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_exit, UNAVAILABLE_EXIT);
    }

    #[test]
    fn held_eof_cannot_change_known_exit_timeout_provenance() {
        for (through_post_command, exit, phase, code, child_exit) in [
            (
                true,
                124,
                FailurePhase::ChildProtocol,
                FailureCode::ChildProtocolFailure,
                UNAVAILABLE_EXIT,
            ),
            (false, 124, FailurePhase::Gate0, FailureCode::Timeout, 124),
            (
                false,
                1,
                FailurePhase::ChildProtocol,
                FailureCode::ChildProtocolFailure,
                1,
            ),
        ] {
            let (sender, receiver) = std::sync::mpsc::channel();
            let received_at = Instant::now();
            if through_post_command {
                queue_smoke_through_post_command(&sender, received_at);
            } else {
                queue_smoke_through_commit(&sender, received_at);
            }

            // Keep the sender alive to model a descendant that inherited the
            // direct child's stdout and therefore withholds EOF.
            let mut child = spawn_exit(exit);
            child.wait().expect("pre-reap synthetic held-EOF exit");
            let result = supervise_loop(
                &mut child,
                &receiver,
                RunProfile::Smoke,
                TEST_BUILD,
                short_deadlines(),
            );
            drop(sender);
            let specification = match result {
                LoopResult::Abort(specification) => specification,
                other => panic!("held EOF unexpectedly completed: {other:?}"),
            };
            let status = child.wait().expect("recover cached synthetic status");
            let failure = render_parent_failure(specification, status, TEST_BUILD)
                .expect("render held-EOF failure");
            assert_eq!(failure.phase, phase);
            assert_eq!(failure.code, code);
            assert_eq!(failure.child_exit, child_exit);
        }
    }

    #[test]
    fn late_exit_124_cannot_promote_a_cpu_timeout_to_command_timeout() {
        let mut child = spawn_exit(124);
        let status = child.wait().expect("reap synthetic late exit 124");
        let failure = render_parent_failure(
            FailureSpecification::cpu_timeout(FailurePhase::ProjectorTask),
            status,
            TEST_BUILD,
        )
        .expect("render CPU-timeout race result");
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_exit, UNAVAILABLE_EXIT);
        assert_eq!(failure.task_ordinal, UNAVAILABLE_ORDINAL);
    }

    #[test]
    fn exit_124_cannot_promote_a_non_timeout_child_failure() {
        let child_frame = child_failure_frame(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            613,
            UNAVAILABLE_ORDINAL,
        );
        let mut child = spawn_exit(124);
        let status = child.wait().expect("reap child-failure exit 124");
        let failure = expect_failure(
            finish_complete(
                status,
                Some(Candidate::ChildFailure(child_frame)),
                TEST_BUILD,
            )
            .expect("render rejected child-failure exit 124"),
        );
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_exit, UNAVAILABLE_EXIT);
        assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
    }

    #[test]
    fn child_zeroed_failure_is_rerendered_once_with_exact_frame_digest() {
        let child_frame = child_failure_frame(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            613,
            UNAVAILABLE_ORDINAL,
        );
        let file = TempBytes::new(&child_frame);
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 1),
                RunProfile::Official,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.phase, FailurePhase::ProjectorTask);
        assert_eq!(failure.code, FailureCode::ProjectorMismatch);
        assert_eq!(failure.task_ordinal, 613);
        assert_eq!(failure.child_exit, 1);
        assert_eq!(
            failure.child_failure_frame_digest,
            walt_gpu_ref::m2_receipt::sha256(&child_frame)
        );
    }

    #[test]
    fn zero_build_failure_is_rejected_when_parent_hashed_a_manifest() {
        let child_frame = child_failure_frame_with_identity_and_coordinates(
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
            ZERO_DIGEST,
            0,
            0,
        );
        let file = TempBytes::new(&child_frame);
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 1),
                RunProfile::Official,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.build_identity, TEST_BUILD);
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
    }

    #[test]
    fn child_failure_admission_rejects_parent_exit_and_direct_timeout() {
        let wire_failure = |code, child_exit| {
            let failure = FailureReceipt {
                phase: FailurePhase::Historical,
                code,
                task_ordinal: UNAVAILABLE_ORDINAL,
                subordinal: UNAVAILABLE_ORDINAL,
                child_exit,
                native_status: UNAVAILABLE_STATUS,
                observed_mismatch: 0,
                build_identity: TEST_BUILD,
                freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
                child_failure_frame_digest: ZERO_DIGEST,
            };
            WireFrame {
                kind: FrameKind::Failure,
                phase_or_command_ordinal: 0,
                unit_or_terminal_code: 0,
                detail: failure.encode().to_vec(),
            }
        };

        let mut parent_exit = ProtocolMachine::new(RunProfile::Official);
        assert!(parent_exit
            .accept(&wire_failure(FailureCode::InternalFailure, 1), TEST_BUILD,)
            .is_err());

        let mut direct_timeout = ProtocolMachine::new(RunProfile::Official);
        assert!(direct_timeout
            .accept(
                &wire_failure(FailureCode::Timeout, UNAVAILABLE_EXIT),
                TEST_BUILD,
            )
            .is_err());
    }

    #[test]
    fn post_completed_failure_is_bound_to_the_completed_command() {
        let mut machine = ProtocolMachine::new(RunProfile::Smoke);
        for phase in [2, 3, 5, 6] {
            machine
                .accept(&frame(FrameKind::Preparing, phase, 0), TEST_BUILD)
                .unwrap();
        }
        machine
            .accept(&frame(FrameKind::Committed, 0, 0), TEST_BUILD)
            .unwrap();
        machine
            .accept(
                &frame(FrameKind::Terminal, 0, u32::from(TerminalCode::Completed)),
                TEST_BUILD,
            )
            .unwrap();

        let mismatched = WireFrame::decode(&child_failure_frame(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            109,
            UNAVAILABLE_ORDINAL,
        ))
        .unwrap();
        assert!(machine.accept(&mismatched, TEST_BUILD).is_err());
        assert_eq!(
            machine.state,
            MachineState::Commands {
                command: 0,
                step: CommandStep::PostCommand,
            }
        );

        let matched = WireFrame::decode(&child_failure_frame(
            FailurePhase::Gate0,
            FailureCode::InternalFailure,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        ))
        .unwrap();
        assert!(matches!(
            machine.accept(&matched, TEST_BUILD),
            Ok(Accepted::ChildFailure)
        ));
    }

    #[test]
    fn failure_cannot_replace_matching_terminal_or_use_nonzero_outer_coordinates() {
        let child_frame = child_failure_frame(
            FailurePhase::Gate0,
            FailureCode::CommandError,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        );
        let decoded = WireFrame::decode(&child_frame).unwrap();

        let mut armed = ProtocolMachine::new(RunProfile::Smoke);
        for phase in [2, 3, 5, 6] {
            armed
                .accept(&frame(FrameKind::Preparing, phase, 0), TEST_BUILD)
                .unwrap();
        }
        armed
            .accept(&frame(FrameKind::Committed, 0, 0), TEST_BUILD)
            .unwrap();
        assert!(armed.accept(&decoded, TEST_BUILD).is_err());
        assert_eq!(
            armed.state,
            MachineState::Commands {
                command: 0,
                step: CommandStep::Terminal,
            }
        );

        let invalid_outer = child_failure_frame_with_identity_and_coordinates(
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
            ZERO_DIGEST,
            1,
            0,
        );
        let invalid_outer = WireFrame::decode(&invalid_outer).unwrap();
        let mut early = ProtocolMachine::new(RunProfile::Official);
        assert!(early.accept(&invalid_outer, TEST_BUILD).is_err());
    }

    #[test]
    fn matching_completed_terminal_restarts_cpu_liveness() {
        let final_failure = child_failure_frame(
            FailurePhase::ProjectorTask,
            FailureCode::InternalFailure,
            109,
            UNAVAILABLE_ORDINAL,
        );
        let (sender, receiver) = std::sync::mpsc::channel();
        let received_base = Instant::now() - Duration::from_millis(210);
        queue_smoke_through_commit(&sender, received_base);
        send_reader_frame(
            &sender,
            encoded_frame(FrameKind::Terminal, 0, u32::from(TerminalCode::Completed)),
            received_base + Duration::from_millis(100),
        );
        send_reader_frame(
            &sender,
            encoded_frame(FrameKind::Preparing, 9, 0),
            received_base + Duration::from_millis(200),
        );
        send_reader_frame(
            &sender,
            final_failure.clone(),
            received_base + Duration::from_millis(200),
        );
        sender
            .send(ReaderEvent::Eof {
                received_at: received_base + Duration::from_millis(205),
            })
            .expect("queue synthetic EOF");
        drop(sender);

        let deadlines = SupervisorDeadlines {
            poll_interval: Duration::from_millis(1),
            command: Duration::from_millis(500),
            cpu_liveness: Duration::from_millis(180),
        };
        let mut child = spawn_exit(1);
        child.wait().expect("pre-reap synthetic failure child");
        let result = supervise_loop(
            &mut child,
            &receiver,
            RunProfile::Smoke,
            TEST_BUILD,
            deadlines,
        );
        let failure = match result {
            LoopResult::Complete { status, candidate } => expect_failure(
                finish_complete(status, candidate, TEST_BUILD)
                    .expect("render deterministic liveness-reset result"),
            ),
            other => panic!("completed terminal did not reset liveness: {other:?}"),
        };
        assert_eq!(failure.phase, FailurePhase::ProjectorTask);
        assert_eq!(failure.code, FailureCode::InternalFailure);
        assert_eq!(
            failure.child_failure_frame_digest,
            walt_gpu_ref::m2_receipt::sha256(&final_failure)
        );
    }

    #[test]
    fn multiple_child_failures_are_protocol_failure_not_a_partial_rerender() {
        let child_frame = child_failure_frame(
            FailurePhase::Gate0,
            FailureCode::CommandError,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_ORDINAL,
        );
        let mut bytes = child_frame.clone();
        bytes.extend_from_slice(&child_frame);
        let file = TempBytes::new(&bytes);
        let failure = expect_failure(
            supervise_child_with_deadlines(
                spawn_cat_then_exit(&file, 1),
                RunProfile::Smoke,
                TEST_BUILD,
                semantic_deadlines(),
            )
            .unwrap(),
        );
        assert_eq!(failure.phase, FailurePhase::ChildProtocol);
        assert_eq!(failure.code, FailureCode::ChildProtocolFailure);
        assert_eq!(failure.child_failure_frame_digest, ZERO_DIGEST);
    }

    #[test]
    fn production_watchdogs_and_poll_cap_are_exact() {
        assert_eq!(
            SupervisorDeadlines::production(),
            SupervisorDeadlines {
                poll_interval: Duration::from_millis(10),
                command: Duration::from_millis(125_000),
                cpu_liveness: Duration::from_millis(600_000),
            }
        );
        assert!(SupervisorDeadlines {
            poll_interval: Duration::from_millis(11),
            ..short_deadlines()
        }
        .validate()
        .is_err());
        assert!(SupervisorDeadlines {
            poll_interval: Duration::ZERO,
            ..short_deadlines()
        }
        .validate()
        .is_err());
    }

    #[test]
    fn wire_terminal_classification_does_not_masquerade_as_native_status() {
        for (terminal, code, native_status) in [
            (TerminalCode::Timeout, FailureCode::Timeout, u32::MAX),
            (TerminalCode::Error, FailureCode::CommandError, 5),
            (
                TerminalCode::NotEnqueued,
                FailureCode::CommandStateFailure,
                0,
            ),
            (
                TerminalCode::NotCommitted,
                FailureCode::CommandStateFailure,
                1,
            ),
            (TerminalCode::Scheduled, FailureCode::CommandStateFailure, 3),
            (
                TerminalCode::Unknown,
                FailureCode::CommandStateFailure,
                u32::MAX,
            ),
        ] {
            let specification = FailureSpecification::terminal(RunProfile::Official, 16, terminal);
            assert_eq!(specification.code, code);
            assert_eq!(specification.native_status, native_status);
        }
    }

    #[test]
    fn command_failure_coordinates_match_the_child_work_unit_identity() {
        assert_eq!(
            command_failure_coordinates(RunProfile::Smoke, 0),
            (FailurePhase::Gate0, u32::MAX, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Smoke, 1),
            (FailurePhase::ProjectorTask, 109, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 1),
            (FailurePhase::ArithmeticNegative, u32::MAX, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 2),
            (FailurePhase::ArithmeticCorpus, u32::MAX, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 3),
            (FailurePhase::OpeningNegative, 0, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 15),
            (FailurePhase::OpeningNegative, 12, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 16),
            (FailurePhase::ProjectorTask, 0, u32::MAX)
        );
        assert_eq!(
            command_failure_coordinates(RunProfile::Official, 629),
            (FailurePhase::ProjectorTask, 613, u32::MAX)
        );
    }
}
