//! One-shot M2 child execution for the supervised smoke and official profiles.
//!
//! This module owns the child side of the frozen wire protocol.  It never
//! persists a raw Metal arena: command results cross the public Metal boundary
//! only as move-only accepted tokens, and the official success bytes are
//! assembled only after the complete command census has closed.

use core::convert::Infallible;
use core::fmt;
use std::cell::RefCell;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

use walt_gpu_ref::m2_receipt::{
    FailureCode, FailurePhase, FailureReceipt, FrameKind, SmokeReport, TaskKey, TerminalCode,
    WireFrame, ARITHMETIC_CAPACITY, FREEZE56_DESCRIPTOR_SHA256, PROJECTOR_CAPACITY, ZERO_DIGEST,
};
use walt_gpu_ref::{
    DirectPreflightV1, M2ArithmeticCorpusV1, M2BridgeError, M2GlobalParityAccumulatorV1,
    M2OpeningParityCarrierV1, OpeningChooseTableV1, M2_ARITHMETIC_CASE_COUNT_V1,
    M2_CONTEXT_TASK_COUNT_V1, M2_DIRECT_PARITY_COUNT_V1, M2_DIRECT_STOP_COUNT_V1,
    M2_PHYSICAL_BINDING_COUNT_V1, M2_REDUCED_BINDING_COUNT_V1, OPENING_CANDIDATE_SLOT_CAP_V1,
};
use walt_metal::{
    AcceptedMetalOpeningNegativeV1, AcceptedMetalOpeningTaskV1, ArithmeticRunIntegrity,
    CommandEvent, CommandState, CommandTerminal, MaximumSmokeReport, MetalError, MetalRuntime,
    OpeningRunIntegrity,
};

use crate::assembly::{assemble_success_receipt, AssemblyError, OfficialEvidenceV1};
use crate::observation::{
    observe_authority, observe_device, observe_m2_manifest_build_identity, observe_toolchain,
    verify_checked_descriptor, verify_compiled_m2_manifest, verify_runner_entry_environment,
    CleanRunnerEnvironment, ObservationError,
};

pub const CHILD_SUCCESS_EXIT_CODE: i32 = 0;
pub const CHILD_FAILURE_EXIT_CODE: i32 = 1;
pub const CHILD_TIMEOUT_EXIT_CODE: i32 = 124;

const UNAVAILABLE_ORDINAL: u32 = u32::MAX;
const UNAVAILABLE_NATIVE_STATUS: u32 = u32::MAX;
const UNAVAILABLE_CHILD_EXIT: i32 = i32::MIN;
const NATIVE_NOT_ENQUEUED: u32 = 0;
const NATIVE_ENQUEUED: u32 = 1;
const NATIVE_COMMITTED: u32 = 2;
const NATIVE_SCHEDULED: u32 = 3;
const NATIVE_COMPLETED: u32 = 4;
const NATIVE_ERROR: u32 = 5;

static CHILD_ENTRY_USED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildProfile {
    Smoke,
    Official,
}

/// Run exactly one child profile and return the process exit code for the
/// caller's `main` function.
///
/// The caller must pass its binary stdout (normally a locked handle) as
/// `output`.  Every complete frame is flushed before this function advances.
/// A Metal timeout does not return: the runtime first reports the matching
/// `TERMINAL/TIMEOUT`, then the timeout callback flushes and exits 124 without
/// unwinding.
pub fn run_child<W: Write>(repository_root: &Path, output: &mut W, profile: ChildProfile) -> i32 {
    // The frozen ambient-environment check is deliberately the first
    // observation and precedes even the process one-shot guard.
    let environment = match verify_runner_entry_environment() {
        Ok(environment) => environment,
        Err(error) => {
            let build_identity =
                observe_m2_manifest_build_identity(repository_root).unwrap_or(ZERO_DIGEST);
            let failure = ChildFailure::new(
                FailurePhase::RustBuild,
                FailureCode::ToolchainMismatch,
                format!("runner-entry environment: {error}"),
            );
            return finish_failure(output, build_identity, failure);
        }
    };

    let mut state = RunState {
        build_identity: observe_m2_manifest_build_identity(repository_root).unwrap_or(ZERO_DIGEST),
    };
    let mut sink = FrameSink::new(output);
    if let Err(error) = verify_compiled_m2_manifest(repository_root) {
        let failure = ChildFailure::new(
            FailurePhase::SourceManifest,
            FailureCode::IdentityMismatch,
            format!("compiled/live source manifest: {error}"),
        )
        .mismatch();
        return finish_failure_sink(&mut sink, state.build_identity, failure);
    }
    if CHILD_ENTRY_USED.swap(true, Ordering::SeqCst) {
        let failure = ChildFailure::new(
            FailurePhase::ChildProtocol,
            FailureCode::ChildProtocolFailure,
            "M2 child profile was invoked more than once in one process",
        );
        return finish_failure_sink(&mut sink, state.build_identity, failure);
    }

    let result = match profile {
        ChildProfile::Smoke => run_smoke(repository_root, &environment, &mut sink),
        ChildProfile::Official => {
            run_official(repository_root, &environment, &mut sink, &mut state)
        }
    };
    match result {
        Ok(()) => CHILD_SUCCESS_EXIT_CODE,
        Err(failure) => finish_failure_sink(&mut sink, state.build_identity, failure),
    }
}

#[derive(Default)]
struct RunState {
    build_identity: [u8; 32],
}

#[derive(Debug)]
struct ChildFailure {
    phase: FailurePhase,
    code: FailureCode,
    task_ordinal: u32,
    subordinal: u32,
    native_status: u32,
    observed_mismatch: u32,
    message: String,
}

impl ChildFailure {
    fn new(phase: FailurePhase, code: FailureCode, message: impl Into<String>) -> Self {
        Self {
            phase,
            code,
            task_ordinal: UNAVAILABLE_ORDINAL,
            subordinal: UNAVAILABLE_ORDINAL,
            native_status: UNAVAILABLE_NATIVE_STATUS,
            observed_mismatch: 0,
            message: message.into(),
        }
    }

    fn task(mut self, task_ordinal: u32) -> Self {
        self.task_ordinal = task_ordinal;
        self
    }

    fn subordinal(mut self, subordinal: u32) -> Self {
        self.subordinal = subordinal;
        self
    }

    fn native_status(mut self, native_status: u32) -> Self {
        self.native_status = native_status;
        self
    }

    fn mismatch(mut self) -> Self {
        self.observed_mismatch = 1;
        self
    }
}

impl fmt::Display for ChildFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "phase {:?}, code {:?}, task {}, subordinal {}: {}",
            self.phase, self.code, self.task_ordinal, self.subordinal, self.message
        )
    }
}

struct FrameSink<'a, W> {
    output: &'a mut W,
}

impl<'a, W: Write> FrameSink<'a, W> {
    const fn new(output: &'a mut W) -> Self {
        Self { output }
    }

    fn write_frame(&mut self, frame: WireFrame) -> Result<(), FrameWriteError> {
        let bytes = frame
            .encode()
            .map_err(|error| FrameWriteError::Codec(error.to_string()))?;
        self.output.write_all(&bytes).map_err(FrameWriteError::Io)?;
        self.output.flush().map_err(FrameWriteError::Io)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

#[derive(Debug)]
enum FrameWriteError {
    Codec(String),
    Io(io::Error),
}

impl fmt::Display for FrameWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(error) => write!(formatter, "frame codec: {error}"),
            Self::Io(error) => write!(formatter, "frame output: {error}"),
        }
    }
}

fn progress<W: Write>(
    sink: &mut FrameSink<'_, W>,
    kind: FrameKind,
    phase: u32,
    unit: u32,
) -> Result<(), ChildFailure> {
    sink.write_frame(WireFrame {
        kind,
        phase_or_command_ordinal: phase,
        unit_or_terminal_code: unit,
        detail: Vec::new(),
    })
    .map_err(|error| protocol_failure(format!("progress frame: {error}")))
}

fn post_command<W: Write>(
    sink: &mut FrameSink<'_, W>,
    command_ordinal: u32,
) -> Result<(), ChildFailure> {
    progress(sink, FrameKind::Preparing, 9, command_ordinal)
}

fn success<W: Write>(sink: &mut FrameSink<'_, W>, detail: Vec<u8>) -> Result<(), ChildFailure> {
    sink.write_frame(WireFrame {
        kind: FrameKind::Success,
        phase_or_command_ordinal: 0,
        unit_or_terminal_code: 0,
        detail,
    })
    .map_err(|error| protocol_failure(format!("success frame: {error}")))
}

fn finish_failure<W: Write>(
    output: &mut W,
    build_identity: [u8; 32],
    failure: ChildFailure,
) -> i32 {
    let mut sink = FrameSink::new(output);
    finish_failure_sink(&mut sink, build_identity, failure)
}

fn finish_failure_sink<W: Write>(
    sink: &mut FrameSink<'_, W>,
    build_identity: [u8; 32],
    failure: ChildFailure,
) -> i32 {
    eprintln!("M2 child failure: {failure}");
    let receipt = FailureReceipt {
        phase: failure.phase,
        code: failure.code,
        task_ordinal: failure.task_ordinal,
        subordinal: failure.subordinal,
        child_exit: UNAVAILABLE_CHILD_EXIT,
        native_status: failure.native_status,
        observed_mismatch: failure.observed_mismatch,
        build_identity,
        freeze56_digest: FREEZE56_DESCRIPTOR_SHA256,
        child_failure_frame_digest: ZERO_DIGEST,
    };
    let detail = match receipt.encode_child_zeroed() {
        Ok(bytes) => bytes.to_vec(),
        Err(error) => {
            eprintln!("M2 child could not encode its typed failure: {error}");
            return CHILD_FAILURE_EXIT_CODE;
        }
    };
    if let Err(error) = sink.write_frame(WireFrame {
        kind: FrameKind::Failure,
        phase_or_command_ordinal: 0,
        unit_or_terminal_code: 0,
        detail,
    }) {
        eprintln!("M2 child could not flush its typed failure: {error}");
    }
    CHILD_FAILURE_EXIT_CODE
}

fn protocol_failure(message: impl Into<String>) -> ChildFailure {
    ChildFailure::new(
        FailurePhase::ChildProtocol,
        FailureCode::ChildProtocolFailure,
        message,
    )
}

#[derive(Debug)]
struct CommandTrace {
    command_ordinal: u32,
    stage: u8,
    terminal: Option<TerminalCode>,
    native_status: u32,
    protocol_error: Option<String>,
    frame_error: Option<String>,
}

impl CommandTrace {
    const fn new(command_ordinal: u32) -> Self {
        Self {
            command_ordinal,
            stage: 0,
            terminal: None,
            native_status: UNAVAILABLE_NATIVE_STATUS,
            protocol_error: None,
            frame_error: None,
        }
    }

    fn remember_protocol_error(&mut self, message: impl Into<String>) {
        if self.protocol_error.is_none() {
            self.protocol_error = Some(message.into());
        }
    }

    fn remember_frame_error(&mut self, error: FrameWriteError) {
        if self.frame_error.is_none() {
            self.frame_error = Some(error.to_string());
        }
    }

    fn validate_completed(&self) -> Result<(), ChildFailure> {
        if let Some(error) = &self.frame_error {
            return Err(protocol_failure(format!(
                "command {} frame write failed: {error}",
                self.command_ordinal
            )));
        }
        if let Some(error) = &self.protocol_error {
            return Err(protocol_failure(format!(
                "command {} observer failed: {error}",
                self.command_ordinal
            )));
        }
        if self.stage != 2 || self.terminal != Some(TerminalCode::Completed) {
            return Err(protocol_failure(format!(
                "command {} did not close with one completed terminal",
                self.command_ordinal
            )));
        }
        Ok(())
    }
}

fn observe_command_event<W: Write>(
    sink: &mut FrameSink<'_, W>,
    trace: &mut CommandTrace,
    event: CommandEvent,
) {
    let frame = match event {
        CommandEvent::Committed => {
            if trace.stage != 0 {
                trace.remember_protocol_error("repeated or reordered COMMITTED event");
            } else {
                trace.stage = 1;
            }
            WireFrame {
                kind: FrameKind::Committed,
                phase_or_command_ordinal: trace.command_ordinal,
                unit_or_terminal_code: 0,
                detail: Vec::new(),
            }
        }
        CommandEvent::Terminal(terminal) => {
            if trace.stage != 1 {
                trace.remember_protocol_error("missing, repeated, or reordered TERMINAL event");
            } else {
                trace.stage = 2;
            }
            let (code, native_status) = terminal_mapping(terminal);
            trace.terminal = Some(code);
            trace.native_status = native_status;
            WireFrame {
                kind: FrameKind::Terminal,
                phase_or_command_ordinal: trace.command_ordinal,
                unit_or_terminal_code: code.into(),
                detail: Vec::new(),
            }
        }
    };
    if let Err(error) = sink.write_frame(frame) {
        trace.remember_frame_error(error);
    }
}

const fn terminal_mapping(terminal: CommandTerminal) -> (TerminalCode, u32) {
    match terminal {
        CommandTerminal::Completed => (TerminalCode::Completed, NATIVE_COMPLETED),
        CommandTerminal::Error => (TerminalCode::Error, NATIVE_ERROR),
        CommandTerminal::Timeout => (TerminalCode::Timeout, UNAVAILABLE_NATIVE_STATUS),
        CommandTerminal::NotEnqueued => (TerminalCode::NotEnqueued, NATIVE_NOT_ENQUEUED),
        CommandTerminal::NotCommitted => (TerminalCode::NotCommitted, NATIVE_ENQUEUED),
        CommandTerminal::Scheduled => (TerminalCode::Scheduled, NATIVE_SCHEDULED),
        CommandTerminal::Unknown(status) => (
            TerminalCode::Unknown,
            if status > u32::MAX as usize {
                UNAVAILABLE_NATIVE_STATUS
            } else {
                status as u32
            },
        ),
    }
}

const fn command_state_native_status(state: CommandState) -> u32 {
    match state {
        CommandState::NotEnqueued => NATIVE_NOT_ENQUEUED,
        CommandState::Enqueued => NATIVE_ENQUEUED,
        CommandState::Committed => NATIVE_COMMITTED,
        CommandState::Scheduled => NATIVE_SCHEDULED,
        CommandState::Completed => NATIVE_COMPLETED,
        CommandState::Error => NATIVE_ERROR,
        CommandState::Unknown(status) => {
            if status > u32::MAX as usize {
                UNAVAILABLE_NATIVE_STATUS
            } else {
                status as u32
            }
        }
    }
}

fn timeout_exit<W: Write>(
    sink: &RefCell<&mut FrameSink<'_, W>>,
    command_ordinal: u32,
    state: CommandState,
) -> Infallible {
    if let Err(error) = sink.borrow_mut().flush() {
        eprintln!("M2 child timeout frame flush failed for command {command_ordinal}: {error}");
    }
    eprintln!(
        "M2 child command {command_ordinal} timed out in native status {}",
        command_state_native_status(state)
    );
    process::exit(CHILD_TIMEOUT_EXIT_CODE)
}

macro_rules! observed_metal_command {
    (
        $sink:expr,
        $command:expr,
        $phase:expr,
        $task:expr,
        |$observer:ident, $timeout:ident| $operation:expr
    ) => {{
        let command_ordinal: u32 = $command;
        let active_phase: FailurePhase = $phase;
        let task_ordinal: u32 = $task;
        let trace_cell = RefCell::new(CommandTrace::new(command_ordinal));
        let sink_cell = RefCell::new(&mut *$sink);
        let operation_result = {
            let mut $observer = |event| {
                let mut sink_ref = sink_cell.borrow_mut();
                let mut trace_ref = trace_cell.borrow_mut();
                observe_command_event(&mut **sink_ref, &mut trace_ref, event);
            };
            let mut $timeout =
                |state| -> Infallible { timeout_exit(&sink_cell, command_ordinal, state) };
            $operation
        };
        let trace = trace_cell.into_inner();
        if let Some(error) = &trace.frame_error {
            Err(protocol_failure(format!(
                "command {command_ordinal} frame write failed: {error}"
            )))
        } else if let Some(error) = &trace.protocol_error {
            Err(protocol_failure(format!(
                "command {command_ordinal} observer failed: {error}"
            )))
        } else {
            match operation_result {
                Ok(value) => {
                    trace.validate_completed()?;
                    Ok(value)
                }
                Err(error) => Err(metal_failure(
                    error,
                    active_phase,
                    task_ordinal,
                    trace.native_status,
                )),
            }
        }
    }};
}

fn run_smoke<W: Write>(
    repository_root: &Path,
    environment: &CleanRunnerEnvironment,
    sink: &mut FrameSink<'_, W>,
) -> Result<(), ChildFailure> {
    let toolchain =
        observe_toolchain(environment, repository_root).map_err(toolchain_observation_failure)?;
    progress(sink, FrameKind::Preparing, 2, 0)?;

    let corpus = M2ArithmeticCorpusV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::ArithmeticCorpus,
            FailureCode::ArithmeticMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    require_arithmetic_corpus(&corpus)?;
    let _choose = OpeningChooseTableV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::Tables,
            FailureCode::IdentityMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    progress(sink, FrameKind::Preparing, 3, 0)?;
    drop(corpus);

    let carrier = M2OpeningParityCarrierV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    let maximum_task = carrier.tasks().get(109).ok_or_else(|| {
        ChildFailure::new(
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            "maximum smoke task 109 is absent",
        )
        .mismatch()
    })?;
    if maximum_task.ordinal() != 109
        || maximum_task.response_count() != 7_980
        || maximum_task.candidate_slot_count() != 79_800
    {
        return Err(ChildFailure::new(
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            "maximum smoke task extent changed",
        )
        .task(109)
        .mismatch());
    }
    let maximum_comparand = maximum_task
        .render_expected_slot_words_v1()
        .map_err(|error| {
            portable_failure(
                error,
                FailurePhase::CarrierPreflight,
                FailureCode::ProjectorMismatch,
                109,
            )
        })?;
    if maximum_comparand.len() != 79_800 {
        return Err(ChildFailure::new(
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            "maximum smoke scalar comparand extent changed",
        )
        .task(109)
        .mismatch());
    }
    drop(maximum_comparand);
    drop(carrier);
    progress(sink, FrameKind::Preparing, 5, 0)?;

    let mut runtime = MetalRuntime::new().map_err(|error| {
        metal_failure(
            error,
            FailurePhase::Gate0,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_NATIVE_STATUS,
        )
    })?;
    progress(sink, FrameKind::Preparing, 6, 0)?;

    observed_metal_command!(
        sink,
        0,
        FailurePhase::Gate0,
        UNAVAILABLE_ORDINAL,
        |observer, timeout| runtime.run_gate0(&mut observer, &mut timeout)
    )?;
    let device = observe_device(environment, repository_root, runtime.device_profile())
        .map_err(device_observation_failure)?;
    verify_checked_descriptor(repository_root, &toolchain, &device)
        .map_err(descriptor_observation_failure)?;
    post_command(sink, 0)?;

    let report = observed_metal_command!(
        sink,
        1,
        FailurePhase::ProjectorTask,
        109,
        |observer, timeout| runtime.run_maximum_smoke(&mut observer, &mut timeout)
    )?;
    validate_smoke_report(&report)?;
    post_command(sink, 1)?;

    // Neither the control report nor its completed arena enters success.
    success(sink, SmokeReport.encode().to_vec())
}

fn run_official<W: Write>(
    repository_root: &Path,
    environment: &CleanRunnerEnvironment,
    sink: &mut FrameSink<'_, W>,
    state: &mut RunState,
) -> Result<(), ChildFailure> {
    let authority =
        observe_authority(environment, repository_root).map_err(authority_observation_failure)?;
    state.build_identity = authority.build_identity;
    progress(sink, FrameKind::Preparing, 1, 0)?;

    let toolchain =
        observe_toolchain(environment, repository_root).map_err(toolchain_observation_failure)?;
    progress(sink, FrameKind::Preparing, 2, 0)?;

    let corpus = M2ArithmeticCorpusV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::ArithmeticCorpus,
            FailureCode::ArithmeticMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    require_arithmetic_corpus(&corpus)?;
    let choose = OpeningChooseTableV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::Tables,
            FailureCode::IdentityMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    progress(sink, FrameKind::Preparing, 3, 0)?;

    let carrier = M2OpeningParityCarrierV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    preflight_carrier(&carrier, sink)?;

    // The opaque accumulator owns an independent canonical carrier and fixes
    // every global stream extent before the first dispatch.
    let mut accumulator = M2GlobalParityAccumulatorV1::canonical().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            UNAVAILABLE_ORDINAL,
        )
    })?;
    preflight_scalar_comparands(&carrier, sink)?;
    drop(carrier);

    let mut runtime = MetalRuntime::new().map_err(|error| {
        metal_failure(
            error,
            FailurePhase::Gate0,
            UNAVAILABLE_ORDINAL,
            UNAVAILABLE_NATIVE_STATUS,
        )
    })?;
    progress(sink, FrameKind::Preparing, 6, 0)?;

    observed_metal_command!(
        sink,
        0,
        FailurePhase::Gate0,
        UNAVAILABLE_ORDINAL,
        |observer, timeout| runtime.run_gate0(&mut observer, &mut timeout)
    )?;
    let device = observe_device(environment, repository_root, runtime.device_profile())
        .map_err(device_observation_failure)?;
    verify_checked_descriptor(repository_root, &toolchain, &device)
        .map_err(descriptor_observation_failure)?;
    post_command(sink, 0)?;

    let arithmetic_negative = observed_metal_command!(
        sink,
        1,
        FailurePhase::ArithmeticNegative,
        UNAVAILABLE_ORDINAL,
        |observer, timeout| runtime.run_arithmetic_negative(&mut observer, &mut timeout)
    )?;
    validate_arithmetic_integrity(arithmetic_negative.integrity(), false)?;
    post_command(sink, 1)?;

    let arithmetic = observed_metal_command!(
        sink,
        2,
        FailurePhase::ArithmeticCorpus,
        UNAVAILABLE_ORDINAL,
        |observer, timeout| runtime.run_official_arithmetic(&corpus, &mut observer, &mut timeout)
    )?;
    validate_arithmetic_integrity(arithmetic.integrity(), true)?;
    post_command(sink, 2)?;

    let mut opening_negatives = Vec::with_capacity(13);
    for ordinal in 0..13usize {
        let command = 3u32
            .checked_add(u32::try_from(ordinal).map_err(|_| {
                ChildFailure::new(
                    FailurePhase::OpeningNegative,
                    FailureCode::InternalFailure,
                    "opening negative ordinal conversion",
                )
            })?)
            .ok_or_else(|| {
                ChildFailure::new(
                    FailurePhase::OpeningNegative,
                    FailureCode::InternalFailure,
                    "opening negative command overflow",
                )
            })?;
        let integrity = observed_metal_command!(
            sink,
            command,
            FailurePhase::OpeningNegative,
            u32::try_from(ordinal).expect("closed thirteen-case ordinal"),
            |observer, timeout| runtime.run_opening_negative(
                ordinal,
                &choose,
                &mut observer,
                &mut timeout
            )
        )?;
        validate_opening_negative_integrity(&integrity, ordinal)?;
        post_command(sink, command)?;
        opening_negatives.push(integrity);
    }

    let mut openings = Vec::with_capacity(M2_CONTEXT_TASK_COUNT_V1);
    for ordinal in 0..M2_CONTEXT_TASK_COUNT_V1 {
        let ordinal_u32 = u32::try_from(ordinal).map_err(|_| {
            ChildFailure::new(
                FailurePhase::ProjectorTask,
                FailureCode::InternalFailure,
                "opening task ordinal conversion",
            )
        })?;
        if accumulator.accepted_task_count() != ordinal_u32
            || accumulator.next_task().map(|task| task.ordinal()) != Some(ordinal_u32)
        {
            return Err(ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::CarrierMismatch,
                "opaque accumulator task order changed before dispatch",
            )
            .task(ordinal_u32)
            .mismatch());
        }
        let command = 16u32.checked_add(ordinal_u32).ok_or_else(|| {
            ChildFailure::new(
                FailurePhase::ProjectorTask,
                FailureCode::InternalFailure,
                "projector command overflow",
            )
        })?;
        let accepted = observed_metal_command!(
            sink,
            command,
            FailurePhase::ProjectorTask,
            ordinal_u32,
            |observer, timeout| runtime.run_next_opening(
                &mut accumulator,
                &choose,
                &mut observer,
                &mut timeout
            )
        )?;
        validate_opening_integrity(&accepted, ordinal_u32)?;
        openings.push(accepted);
        if ordinal + 1 != M2_CONTEXT_TASK_COUNT_V1 {
            post_command(sink, command)?;
        }
    }
    if accumulator.accepted_task_count()
        != u32::try_from(M2_CONTEXT_TASK_COUNT_V1).expect("frozen task count fits u32")
    {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::CarrierMismatch,
            "opaque accumulator accepted-task census changed",
        )
        .mismatch());
    }
    let global_parity = accumulator.finish().map_err(|error| {
        portable_failure(
            error,
            FailurePhase::ProjectorTask,
            FailureCode::MassMismatch,
            613,
        )
    })?;
    post_command(sink, 629)?;

    let evidence = OfficialEvidenceV1 {
        build_identity: state.build_identity,
        authority: authority.section,
        toolchain: toolchain.section,
        device: device.section,
        choose,
        arithmetic,
        arithmetic_negative,
        opening_negatives,
        openings,
        global_parity,
    };
    let receipt = {
        let progress_sink = RefCell::new(&mut *sink);
        let progress_error = RefCell::new(None::<String>);
        let receipt = assemble_success_receipt(
            evidence,
            |unit| emit_finalizing_from_assembly(&progress_sink, &progress_error, 7, unit),
            |unit| emit_finalizing_from_assembly(&progress_sink, &progress_error, 8, unit),
        );
        if let Some(error) = progress_error.into_inner() {
            return Err(protocol_failure(error));
        }
        receipt
    }
    .map_err(assembly_failure)?;
    success(sink, receipt)
}

fn emit_finalizing_from_assembly<W: Write>(
    sink: &RefCell<&mut FrameSink<'_, W>>,
    first_error: &RefCell<Option<String>>,
    phase: u32,
    unit: u32,
) -> Result<(), AssemblyError> {
    let result = sink.borrow_mut().write_frame(WireFrame {
        kind: FrameKind::Finalizing,
        phase_or_command_ordinal: phase,
        unit_or_terminal_code: unit,
        detail: Vec::new(),
    });
    match result {
        Ok(()) => Ok(()),
        Err(error) => {
            let mut stored = first_error.borrow_mut();
            if stored.is_none() {
                *stored = Some(format!("finalizing phase {phase} unit {unit}: {error}"));
            }
            Err(AssemblyError::Invariant("finalizing progress frame"))
        }
    }
}

fn preflight_carrier<W: Write>(
    carrier: &M2OpeningParityCarrierV1,
    sink: &mut FrameSink<'_, W>,
) -> Result<(), ChildFailure> {
    if carrier.tasks().len() != M2_CONTEXT_TASK_COUNT_V1
        || carrier.task_key_stream_sha256() == ZERO_DIGEST
    {
        return Err(ChildFailure::new(
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            "canonical carrier task count or stream identity changed",
        )
        .mismatch());
    }
    let mut reduced_bindings = 0usize;
    let mut physical_bindings = 0usize;
    let mut direct_parity = 0usize;
    let mut direct_stop = 0usize;
    for (ordinal, task) in carrier.tasks().iter().enumerate() {
        let ordinal_u32 = u32::try_from(ordinal).map_err(|_| {
            ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::InternalFailure,
                "carrier ordinal conversion",
            )
        })?;
        let key = TaskKey::decode(&task.task_key().to_le_bytes()).map_err(|error| {
            ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::CarrierMismatch,
                format!("carrier task key {ordinal_u32}: {error}"),
            )
            .task(ordinal_u32)
            .mismatch()
        })?;
        let words = task.task_words();
        if task.ordinal() != ordinal_u32
            || key.task_ordinal != ordinal_u32
            || u32::from(key.arm) != task.arm().code()
            || key.arm_ordinal != task.arm_ordinal()
            || key.grade != words[2]
            || key.pool_mask != words[3]
            || key.matching_mask != words[4]
            || key.pool_count != words[5]
            || key.response_triple_count != task.response_count()
            || key.candidate_slot_count != task.candidate_slot_count()
            || task.candidate_slot_count() > OPENING_CANDIDATE_SLOT_CAP_V1 as u32
        {
            return Err(ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::CarrierMismatch,
                "carrier task metadata join changed",
            )
            .task(ordinal_u32)
            .mismatch());
        }
        reduced_bindings = reduced_bindings
            .checked_add(task.reduced_binding_count())
            .ok_or_else(|| carrier_count_overflow("reduced bindings", ordinal_u32))?;
        physical_bindings = physical_bindings
            .checked_add(task.physical_binding_count())
            .ok_or_else(|| carrier_count_overflow("physical bindings", ordinal_u32))?;
        match task.direct_preflight() {
            DirectPreflightV1::Admitted { .. } => {
                direct_parity = direct_parity
                    .checked_add(1)
                    .ok_or_else(|| carrier_count_overflow("direct parity", ordinal_u32))?;
            }
            DirectPreflightV1::DeclaredStop { .. } => {
                direct_stop = direct_stop
                    .checked_add(1)
                    .ok_or_else(|| carrier_count_overflow("direct stop", ordinal_u32))?;
            }
        }
        progress(sink, FrameKind::Preparing, 4, ordinal_u32)?;
    }
    if reduced_bindings != M2_REDUCED_BINDING_COUNT_V1
        || physical_bindings != M2_PHYSICAL_BINDING_COUNT_V1
        || direct_parity != M2_DIRECT_PARITY_COUNT_V1
        || direct_stop != M2_DIRECT_STOP_COUNT_V1
    {
        return Err(ChildFailure::new(
            FailurePhase::CarrierPreflight,
            FailureCode::CarrierMismatch,
            "carrier binding or direct-preflight census changed",
        )
        .mismatch());
    }
    Ok(())
}

fn carrier_count_overflow(label: &'static str, ordinal: u32) -> ChildFailure {
    ChildFailure::new(
        FailurePhase::CarrierPreflight,
        FailureCode::InternalFailure,
        format!("carrier {label} count overflow"),
    )
    .task(ordinal)
}

fn preflight_scalar_comparands<W: Write>(
    carrier: &M2OpeningParityCarrierV1,
    sink: &mut FrameSink<'_, W>,
) -> Result<(), ChildFailure> {
    for (ordinal, task) in carrier.tasks().iter().enumerate() {
        let ordinal_u32 = u32::try_from(ordinal).map_err(|_| {
            ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::InternalFailure,
                "scalar comparand ordinal conversion",
            )
        })?;
        let comparand = task.render_expected_slot_words_v1().map_err(|error| {
            portable_failure(
                error,
                FailurePhase::CarrierPreflight,
                FailureCode::ProjectorMismatch,
                ordinal_u32,
            )
        })?;
        if comparand.len()
            != usize::try_from(task.candidate_slot_count()).map_err(|_| {
                ChildFailure::new(
                    FailurePhase::CarrierPreflight,
                    FailureCode::InternalFailure,
                    "scalar comparand length conversion",
                )
                .task(ordinal_u32)
            })?
        {
            return Err(ChildFailure::new(
                FailurePhase::CarrierPreflight,
                FailureCode::CarrierMismatch,
                "scalar comparand extent changed",
            )
            .task(ordinal_u32)
            .mismatch());
        }
        drop(comparand);
        progress(sink, FrameKind::Preparing, 5, ordinal_u32)?;
    }
    Ok(())
}

fn require_arithmetic_corpus(corpus: &M2ArithmeticCorpusV1) -> Result<(), ChildFailure> {
    if corpus.inputs().len() != M2_ARITHMETIC_CASE_COUNT_V1
        || corpus.expected_outputs().len() != M2_ARITHMETIC_CASE_COUNT_V1
    {
        return Err(ChildFailure::new(
            FailurePhase::ArithmeticCorpus,
            FailureCode::ArithmeticMismatch,
            "canonical arithmetic corpus census changed",
        )
        .mismatch());
    }
    Ok(())
}

fn validate_arithmetic_integrity(
    value: &ArithmeticRunIntegrity,
    official: bool,
) -> Result<(), ChildFailure> {
    let phase = if official {
        FailurePhase::ArithmeticCorpus
    } else {
        FailurePhase::ArithmeticNegative
    };
    if value.input_pre_digest != value.input_post_digest {
        return Err(ChildFailure::new(
            phase,
            FailureCode::InputMutation,
            "arithmetic input digest changed",
        )
        .mismatch());
    }
    if value.cpu_output_digest != value.gpu_output_digest {
        return Err(ChildFailure::new(
            phase,
            FailureCode::ArithmeticMismatch,
            "arithmetic CPU/GPU output digest changed",
        )
        .mismatch());
    }
    if value.guard_pre_digest != value.guard_post_digest {
        return Err(ChildFailure::new(
            phase,
            FailureCode::GuardFailure,
            "arithmetic protected guard digest changed",
        )
        .mismatch());
    }
    if official {
        let allocated = value
            .allocated_input_bytes
            .checked_add(value.allocated_output_bytes)
            .ok_or_else(|| {
                ChildFailure::new(
                    phase,
                    FailureCode::InternalFailure,
                    "official arithmetic allocation overflow",
                )
            })?;
        if value.case_count != 16_384
            || value.accepted_count != 16_384
            || value.hard_count != 0
            || value
                .success_count
                .checked_add(value.checked_undefined_count)
                != Some(16_384)
            || allocated < ARITHMETIC_CAPACITY
        {
            return Err(ChildFailure::new(
                phase,
                FailureCode::ArithmeticMismatch,
                "official arithmetic counts or allocation changed",
            )
            .mismatch());
        }
    } else if value.case_count != 13
        || value.accepted_count != 0
        || value.success_count != 0
        || value.checked_undefined_count != 0
        || value.hard_count != 13
        || value.allocated_input_bytes < 13 * 80
        || value.allocated_output_bytes < 15 * 64
    {
        return Err(ChildFailure::new(
            phase,
            FailureCode::ArithmeticMismatch,
            "arithmetic-negative counts or allocation changed",
        )
        .mismatch());
    }
    Ok(())
}

fn validate_opening_negative_integrity(
    accepted: &AcceptedMetalOpeningNegativeV1,
    expected_ordinal: usize,
) -> Result<(), ChildFailure> {
    let value = accepted.integrity();
    let ordinal = u32::try_from(expected_ordinal).map_err(|_| {
        ChildFailure::new(
            FailurePhase::OpeningNegative,
            FailureCode::InternalFailure,
            "opening-negative ordinal conversion",
        )
    })?;
    if value.ordinal != ordinal {
        return Err(ChildFailure::new(
            FailurePhase::OpeningNegative,
            FailureCode::ProjectorMismatch,
            "opening-negative result ordinal changed",
        )
        .task(ordinal)
        .mismatch());
    }
    if value.task_pre_digest != value.task_post_digest
        || value.choose_pre_digest != value.choose_post_digest
    {
        return Err(ChildFailure::new(
            FailurePhase::OpeningNegative,
            FailureCode::InputMutation,
            "opening-negative immutable input digest changed",
        )
        .task(ordinal)
        .mismatch());
    }
    // The output is deliberately all poison before dispatch and exact HARD
    // records afterwards.  The accepted wrapper proves that post-image; these
    // two digests are not an immutability pair.
    Ok(())
}

fn validate_opening_integrity(
    accepted: &AcceptedMetalOpeningTaskV1,
    expected_ordinal: u32,
) -> Result<(), ChildFailure> {
    let integrity = accepted.integrity();
    let checked = accepted.bound_task().checked_payload();
    if integrity.task_ordinal != expected_ordinal || checked.task_ordinal() != expected_ordinal {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::CarrierMismatch,
            "accepted opening task ordinal changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    if integrity.response_count != checked.response_count()
        || integrity.candidate_slot_count != checked.candidate_slot_count()
        || integrity.valid_cell_count != checked.cell_count()
    {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            "accepted opening task count join changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    validate_opening_run_integrity(integrity, Some(accepted), expected_ordinal)
}

fn validate_opening_run_integrity(
    integrity: &OpeningRunIntegrity,
    accepted: Option<&AcceptedMetalOpeningTaskV1>,
    expected_ordinal: u32,
) -> Result<(), ChildFailure> {
    if integrity.task_pre_digest != integrity.task_post_digest
        || integrity.choose_pre_digest != integrity.choose_post_digest
    {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::InputMutation,
            "opening task or choose-table digest changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    if integrity.protected_pre_digest != integrity.protected_post_digest {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::GuardFailure,
            "opening tail or guard digest changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    if let Some(accepted) = accepted {
        let checked = accepted.bound_task().checked_payload();
        if integrity.defensive_cpu_slot_digest != checked.cpu_raw_sha256()
            || integrity.gpu_slot_digest != checked.gpu_raw_sha256()
            || checked.cpu_raw_sha256() != checked.gpu_raw_sha256()
            || checked.cpu_payload_sha256() != checked.gpu_payload_sha256()
            || checked.cpu_aggregate_sha256() != checked.gpu_aggregate_sha256()
        {
            return Err(ChildFailure::new(
                FailurePhase::ProjectorTask,
                FailureCode::ProjectorMismatch,
                "opening CPU/GPU raw, payload, or aggregate digest changed",
            )
            .task(expected_ordinal)
            .mismatch());
        }
    } else if integrity.defensive_cpu_slot_digest != integrity.gpu_slot_digest {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            "smoke opening CPU/GPU slot digest changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    let allocated = integrity
        .allocated_task_bytes
        .checked_add(integrity.allocated_choose_bytes)
        .and_then(|value| value.checked_add(integrity.allocated_output_bytes))
        .ok_or_else(|| {
            ChildFailure::new(
                FailurePhase::ProjectorTask,
                FailureCode::InternalFailure,
                "opening allocation overflow",
            )
            .task(expected_ordinal)
        })?;
    if allocated < PROJECTOR_CAPACITY {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::AllocationFailure,
            "opening allocation high-water changed",
        )
        .task(expected_ordinal)
        .mismatch());
    }
    Ok(())
}

fn validate_smoke_report(report: &MaximumSmokeReport) -> Result<(), ChildFailure> {
    let integrity = report.integrity();
    if integrity.task_ordinal != 109
        || integrity.response_count != 7_980
        || integrity.candidate_slot_count != 79_800
        || integrity.valid_cell_count != 11_730
    {
        return Err(ChildFailure::new(
            FailurePhase::ProjectorTask,
            FailureCode::ProjectorMismatch,
            "maximum smoke result extent changed",
        )
        .task(109)
        .mismatch());
    }
    validate_opening_run_integrity(integrity, None, 109)
}

fn authority_observation_failure(error: ObservationError) -> ChildFailure {
    match &error {
        ObservationError::Command { label, .. } if *label == "historical verifier" => {
            ChildFailure::new(
                FailurePhase::Historical,
                FailureCode::InvalidAuthority,
                error.to_string(),
            )
            .mismatch()
        }
        ObservationError::Command { label, .. } if *label == "M2 source verifier" => {
            ChildFailure::new(
                FailurePhase::SourceManifest,
                FailureCode::IdentityMismatch,
                error.to_string(),
            )
            .mismatch()
        }
        _ => ChildFailure::new(
            FailurePhase::Historical,
            FailureCode::InvalidAuthority,
            error.to_string(),
        )
        .mismatch(),
    }
}

fn toolchain_observation_failure(error: ObservationError) -> ChildFailure {
    match &error {
        ObservationError::Command { label, .. } if *label == "two-build metallib verifier" => {
            ChildFailure::new(
                FailurePhase::ShaderReproducibility,
                FailureCode::MetallibMismatch,
                error.to_string(),
            )
            .mismatch()
        }
        ObservationError::Command { label, .. } if *label == "MSL no-float source gate" => {
            ChildFailure::new(
                FailurePhase::ShaderReproducibility,
                FailureCode::MalformedOutput,
                error.to_string(),
            )
            .mismatch()
        }
        _ => ChildFailure::new(
            FailurePhase::MetalToolchain,
            FailureCode::ToolchainMismatch,
            error.to_string(),
        )
        .mismatch(),
    }
}

fn device_observation_failure(error: ObservationError) -> ChildFailure {
    ChildFailure::new(
        FailurePhase::Gate0,
        FailureCode::ToolchainMismatch,
        error.to_string(),
    )
    .native_status(NATIVE_COMPLETED)
    .mismatch()
}

fn descriptor_observation_failure(error: ObservationError) -> ChildFailure {
    ChildFailure::new(
        FailurePhase::MetalToolchain,
        FailureCode::ToolchainMismatch,
        error.to_string(),
    )
    .native_status(NATIVE_COMPLETED)
    .mismatch()
}

fn portable_failure(
    error: M2BridgeError,
    phase: FailurePhase,
    code: FailureCode,
    task_ordinal: u32,
) -> ChildFailure {
    let failure = ChildFailure::new(phase, code, error.to_string()).mismatch();
    if task_ordinal == UNAVAILABLE_ORDINAL {
        failure
    } else {
        failure.task(task_ordinal)
    }
}

fn assembly_failure(error: AssemblyError) -> ChildFailure {
    match &error {
        AssemblyError::Portable(_) => ChildFailure::new(
            FailurePhase::Bindings,
            FailureCode::BindingMismatch,
            error.to_string(),
        )
        .mismatch(),
        AssemblyError::Codec(_) => ChildFailure::new(
            FailurePhase::ReceiptRender,
            FailureCode::MalformedOutput,
            error.to_string(),
        )
        .mismatch(),
        AssemblyError::Invariant(label) if label.contains("binding") => ChildFailure::new(
            FailurePhase::Bindings,
            FailureCode::BindingMismatch,
            error.to_string(),
        )
        .mismatch(),
        AssemblyError::Invariant(_) | AssemblyError::Length(_) => ChildFailure::new(
            FailurePhase::ReceiptRender,
            FailureCode::InternalFailure,
            error.to_string(),
        )
        .mismatch(),
    }
}

fn metal_failure(
    error: MetalError,
    phase: FailurePhase,
    task_ordinal: u32,
    native_status: u32,
) -> ChildFailure {
    let code = match &error {
        MetalError::NoDevice => FailureCode::NoDevice,
        MetalError::AllocationFailure { .. }
        | MetalError::BufferTooShort { .. }
        | MetalError::BufferMisaligned(_)
        | MetalError::LengthOverflow(_) => FailureCode::AllocationFailure,
        MetalError::LibraryLoad(_) | MetalError::MissingFunction(_) => {
            FailureCode::MetallibMismatch
        }
        MetalError::PipelineCreation(_, _) | MetalError::PipelineThreadLimit { .. } => {
            FailureCode::PipelineFailure
        }
        MetalError::EncoderCreation | MetalError::InvalidBinding { .. } => {
            FailureCode::EncoderFailure
        }
        MetalError::NoCommandQueue
        | MetalError::NoCommandBuffer
        | MetalError::Gate0Required
        | MetalError::Gate0AlreadyPassed
        | MetalError::RuntimePoisoned
        | MetalError::UnexpectedCommandState(_) => FailureCode::CommandStateFailure,
        MetalError::CommandError(_) => FailureCode::CommandError,
        MetalError::Timeout(_) => FailureCode::Timeout,
        MetalError::InvalidArithmeticInput { .. }
        | MetalError::WrongRecordCount { .. }
        | MetalError::InvalidNegativeOrdinal { .. }
        | MetalError::InvalidOpeningTask(_)
        | MetalError::InvalidChooseEntry { .. } => FailureCode::MalformedOutput,
        MetalError::PortableValidation(_) => match phase {
            FailurePhase::ProjectorTask | FailurePhase::OpeningNegative => {
                FailureCode::ProjectorMismatch
            }
            _ => FailureCode::ArithmeticMismatch,
        },
        MetalError::InvalidOutput { reason, .. } if reason.contains("input changed") => {
            FailureCode::InputMutation
        }
        MetalError::InvalidOutput { reason, .. }
            if reason.contains("guard changed") || reason.contains("tail or guard") =>
        {
            FailureCode::GuardFailure
        }
        MetalError::InvalidOutput { reason, .. } if reason.contains("poison") => {
            FailureCode::PoisonFailure
        }
        MetalError::InvalidOutput { .. } => match phase {
            FailurePhase::ArithmeticNegative | FailurePhase::ArithmeticCorpus => {
                FailureCode::ArithmeticMismatch
            }
            _ => FailureCode::ProjectorMismatch,
        },
    };
    let records_mismatch = matches!(
        code,
        FailureCode::MetallibMismatch
            | FailureCode::MalformedOutput
            | FailureCode::PoisonFailure
            | FailureCode::GuardFailure
            | FailureCode::InputMutation
            | FailureCode::ArithmeticMismatch
            | FailureCode::ProjectorMismatch
            | FailureCode::MassMismatch
            | FailureCode::CarrierMismatch
            | FailureCode::BindingMismatch
            | FailureCode::ReceiptNondeterministic
            | FailureCode::ReceiptComparandMismatch
    );
    let mut failure =
        ChildFailure::new(phase, code, error.to_string()).native_status(native_status);
    if records_mismatch {
        failure = failure.mismatch();
    }
    if task_ordinal != UNAVAILABLE_ORDINAL {
        failure = failure.task(task_ordinal);
    }
    if let MetalError::InvalidOutput { record, .. }
    | MetalError::InvalidBinding { index: record, .. }
    | MetalError::InvalidArithmeticInput { index: record, .. } = &error
    {
        failure = failure.subordinal(u32::try_from(*record).unwrap_or(UNAVAILABLE_ORDINAL));
    }
    failure
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split_frames(bytes: &[u8]) -> Vec<WireFrame> {
        let mut frames = Vec::new();
        let mut cursor = 0usize;
        while cursor < bytes.len() {
            let payload = u32::from_le_bytes(
                bytes[cursor..cursor + 4]
                    .try_into()
                    .expect("frame length prefix"),
            );
            let total = usize::try_from(payload).expect("payload usize") + 4;
            let end = cursor + total;
            frames.push(WireFrame::decode(&bytes[cursor..end]).expect("canonical frame"));
            cursor = end;
        }
        frames
    }

    #[test]
    fn terminal_registry_maps_exactly() {
        assert_eq!(
            terminal_mapping(CommandTerminal::Completed),
            (TerminalCode::Completed, 4)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::Error),
            (TerminalCode::Error, 5)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::Timeout),
            (TerminalCode::Timeout, u32::MAX)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::NotEnqueued),
            (TerminalCode::NotEnqueued, 0)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::NotCommitted),
            (TerminalCode::NotCommitted, 1)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::Scheduled),
            (TerminalCode::Scheduled, 3)
        );
        assert_eq!(
            terminal_mapping(CommandTerminal::Unknown(91)),
            (TerminalCode::Unknown, 91)
        );
    }

    #[test]
    fn command_observer_flushes_exact_pair() {
        let mut bytes = Vec::new();
        let mut sink = FrameSink::new(&mut bytes);
        let mut trace = CommandTrace::new(27);
        observe_command_event(&mut sink, &mut trace, CommandEvent::Committed);
        observe_command_event(
            &mut sink,
            &mut trace,
            CommandEvent::Terminal(CommandTerminal::Completed),
        );
        trace.validate_completed().expect("closed command");
        let frames = split_frames(&bytes);
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].kind, FrameKind::Committed);
        assert_eq!(frames[0].phase_or_command_ordinal, 27);
        assert_eq!(frames[1].kind, FrameKind::Terminal);
        assert_eq!(frames[1].phase_or_command_ordinal, 27);
        assert_eq!(
            frames[1].unit_or_terminal_code,
            u32::from(TerminalCode::Completed)
        );
    }

    #[test]
    fn child_failure_frame_is_single_and_zeroed() {
        let mut bytes = Vec::new();
        let failure = ChildFailure::new(
            FailurePhase::ChildProtocol,
            FailureCode::ChildProtocolFailure,
            "synthetic",
        );
        assert_eq!(finish_failure(&mut bytes, ZERO_DIGEST, failure), 1);
        let frames = split_frames(&bytes);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].kind, FrameKind::Failure);
        let decoded = FailureReceipt::decode(&frames[0].detail).expect("failure receipt");
        assert_eq!(decoded.child_failure_frame_digest, ZERO_DIGEST);
        assert_eq!(decoded.child_exit, i32::MIN);
        assert_eq!(decoded.build_identity, ZERO_DIGEST);
    }

    #[test]
    fn immutable_opening_join_rejects_mutation() {
        let mut integrity = OpeningRunIntegrity {
            task_ordinal: 109,
            response_count: 7_980,
            candidate_slot_count: 79_800,
            valid_cell_count: 0,
            allocated_task_bytes: 32,
            allocated_choose_bytes: 1_936,
            allocated_output_bytes: PROJECTOR_CAPACITY - 1_968,
            task_pre_digest: [1; 32],
            task_post_digest: [2; 32],
            choose_pre_digest: [3; 32],
            choose_post_digest: [3; 32],
            defensive_cpu_slot_digest: [4; 32],
            gpu_slot_digest: [4; 32],
            protected_pre_digest: [5; 32],
            protected_post_digest: [5; 32],
        };
        let failure = validate_opening_run_integrity(&integrity, None, 109)
            .expect_err("mutated task must fail");
        assert_eq!(failure.code, FailureCode::InputMutation);
        integrity.task_post_digest = integrity.task_pre_digest;
        validate_opening_run_integrity(&integrity, None, 109).expect("closed smoke integrity");
    }
}
