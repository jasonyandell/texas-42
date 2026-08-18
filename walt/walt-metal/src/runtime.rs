use core::convert::Infallible;
use core::mem::size_of;
use std::thread;
use std::time::{Duration, Instant};

use dispatch2::DispatchData;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::{
    MTLCommandBuffer, MTLCommandEncoder, MTLCommandQueue, MTLComputeCommandEncoder,
    MTLComputePipelineState, MTLCreateSystemDefaultDevice, MTLDevice, MTLLibrary,
    MTLResourceOptions, MTLSize,
};
use walt_gpu_ref::{
    ArithmeticNegativeControlsV1, M2ArithmeticCorpusV1, M2BoundOpeningTaskV1,
    M2GlobalParityAccumulatorV1, M2OpeningNegativeControlsV1, M2OpeningParityCarrierV1,
    OpeningChooseTableV1, OPENING_MAX_CELL_COEFFICIENT_V1, OPENING_MAX_CELL_MASS_V1,
    OPENING_MAX_CELL_SUPPORT_V1, OPENING_MAX_WHOLE_MASS_V1,
};
use walt_gpu_spec::Sha256State;

use crate::abi::{
    ArithmeticInputWords, OpeningChooseTableWords, OpeningTaskWords, ARITHMETIC_CHECKED_UNDEFINED,
    ARITHMETIC_SUCCESS, OFFICIAL_ARITHMETIC_CASES, OPENING_ARENA_RECORDS, OPENING_SKIP,
    OPENING_VALID, POISON, THREADGROUP_WIDTH,
};
use crate::bridge::{
    bind_zero, command_state, copy_completed_words, copy_host_words, overwrite_idle_words,
    prove_completed, BoundCommand, BufferRole, CompletionProof, ValidatedBuffer,
};
use crate::{CommandState, MetalError};

const U256_KERNEL: &str = "u256_parity_v1";
const OPENING_KERNEL: &str = "opening_project_v1";
const WORDS_PER_ARITHMETIC_INPUT: usize = 20;
const WORDS_PER_ARITHMETIC_OUTPUT: usize = 16;
const WORDS_PER_OPENING_SLOT: usize = 16;
const COMMAND_TIMEOUT: Duration = Duration::from_secs(120);
const POLL_INTERVAL: Duration = Duration::from_millis(1);
const MAX_OPENING_CELLS: usize = 11_730;
const STREAM_MAGIC: &[u8; 8] = b"W42M2DG1";
const STREAM_VERSION: u32 = 1;
const STREAM_ARITHMETIC_INPUT: u32 = 2;
const STREAM_ARITHMETIC_OUTPUT: u32 = 3;
const STREAM_CONTEXT_SLOTS: u32 = 4;
const STREAM_PROTECTED_RECORDS: u32 = 9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArithmeticLaunch {
    Official,
    NegativeControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpeningLaunch {
    ValidatedTask { response_count: usize },
    NegativeControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LaunchContract {
    grid: usize,
    input_words: usize,
    auxiliary_words: usize,
    output_words: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandTerminal {
    Completed,
    Error,
    Timeout,
    NotEnqueued,
    NotCommitted,
    Scheduled,
    Unknown(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandEvent {
    Committed,
    Terminal(CommandTerminal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PipelineLimits {
    pub execution_width: u32,
    pub maximum_threads: u32,
    pub static_threadgroup_memory: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllocationHighWater {
    pub projector_logical_bytes: u64,
    pub projector_reported_bytes: u64,
    pub arithmetic_logical_bytes: u64,
    pub arithmetic_reported_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceProfile {
    pub sanitized_name: String,
    pub unified_memory: bool,
    pub maximum_buffer_length: u64,
    pub recommended_working_set: u64,
    pub maximum_threads: [u32; 3],
    pub maximum_threadgroup_memory: u32,
    pub arithmetic_pipeline: PipelineLimits,
    pub opening_pipeline: PipelineLimits,
    pub allocations: AllocationHighWater,
    pub gate0_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticRunIntegrity {
    pub case_count: u32,
    pub accepted_count: u32,
    pub success_count: u32,
    pub checked_undefined_count: u32,
    pub hard_count: u32,
    pub allocated_input_bytes: u64,
    pub allocated_output_bytes: u64,
    pub input_pre_digest: [u8; 32],
    pub input_post_digest: [u8; 32],
    pub cpu_output_digest: [u8; 32],
    pub gpu_output_digest: [u8; 32],
    pub guard_pre_digest: [u8; 32],
    pub guard_post_digest: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpeningRunIntegrity {
    pub task_ordinal: u32,
    pub response_count: u32,
    pub candidate_slot_count: u32,
    pub valid_cell_count: u32,
    pub allocated_task_bytes: u64,
    pub allocated_choose_bytes: u64,
    pub allocated_output_bytes: u64,
    pub task_pre_digest: [u8; 32],
    pub task_post_digest: [u8; 32],
    pub choose_pre_digest: [u8; 32],
    pub choose_post_digest: [u8; 32],
    pub defensive_cpu_slot_digest: [u8; 32],
    pub gpu_slot_digest: [u8; 32],
    pub protected_pre_digest: [u8; 32],
    pub protected_post_digest: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpeningNegativeIntegrity {
    pub ordinal: u32,
    pub task_pre_digest: [u8; 32],
    pub task_post_digest: [u8; 32],
    pub choose_pre_digest: [u8; 32],
    pub choose_post_digest: [u8; 32],
    pub protected_pre_digest: [u8; 32],
    pub protected_post_digest: [u8; 32],
}

/// Move-only official arithmetic admission after exact portable-oracle parity.
#[derive(Debug)]
pub struct AcceptedMetalArithmeticV1 {
    integrity: ArithmeticRunIntegrity,
}

impl AcceptedMetalArithmeticV1 {
    pub const fn integrity(&self) -> &ArithmeticRunIntegrity {
        &self.integrity
    }
}

/// Move-only admission for the exact thirteen-case arithmetic-negative command.
#[derive(Debug)]
pub struct AcceptedMetalArithmeticNegativeV1 {
    integrity: ArithmeticRunIntegrity,
}

impl AcceptedMetalArithmeticNegativeV1 {
    pub const fn integrity(&self) -> &ArithmeticRunIntegrity {
        &self.integrity
    }
}

/// Move-only admission for one exact opening-negative command.
#[derive(Debug)]
pub struct AcceptedMetalOpeningNegativeV1 {
    integrity: OpeningNegativeIntegrity,
}

impl AcceptedMetalOpeningNegativeV1 {
    pub const fn integrity(&self) -> &OpeningNegativeIntegrity {
        &self.integrity
    }
}

/// Move-only official opening admission. The bound task owns its portable
/// checked payload metadata and binding intents; no arena bytes escape.
#[derive(Debug)]
pub struct AcceptedMetalOpeningTaskV1 {
    bound: M2BoundOpeningTaskV1,
    integrity: OpeningRunIntegrity,
}

impl AcceptedMetalOpeningTaskV1 {
    pub const fn bound_task(&self) -> &M2BoundOpeningTaskV1 {
        &self.bound
    }

    pub const fn integrity(&self) -> &OpeningRunIntegrity {
        &self.integrity
    }
}

/// Closed maximum-projector smoke result. It carries diagnostics only and can
/// never satisfy official task admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaximumSmokeReport {
    integrity: OpeningRunIntegrity,
}

impl MaximumSmokeReport {
    pub const fn integrity(&self) -> &OpeningRunIntegrity {
        &self.integrity
    }
}

struct BufferSet {
    arithmetic_input: ValidatedBuffer,
    arithmetic_output: ValidatedBuffer,
    opening_task: ValidatedBuffer,
    opening_choose: ValidatedBuffer,
    opening_output: ValidatedBuffer,
    negative_arithmetic_input: ValidatedBuffer,
    negative_arithmetic_output: ValidatedBuffer,
    negative_opening_tasks: Vec<ValidatedBuffer>,
    negative_opening_outputs: Vec<ValidatedBuffer>,
}

pub struct MetalRuntime {
    _device: Retained<ProtocolObject<dyn MTLDevice>>,
    queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    arithmetic_pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
    opening_pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
    buffers: BufferSet,
    profile: DeviceProfile,
    reusable: bool,
}

impl MetalRuntime {
    pub fn new() -> Result<Self, MetalError> {
        let device = MTLCreateSystemDefaultDevice().ok_or(MetalError::NoDevice)?;
        let queue = device.newCommandQueue().ok_or(MetalError::NoCommandQueue)?;
        let data = DispatchData::from_static_bytes(include_bytes!("../shaders/walt_m2.metallib"));
        let library = device
            .newLibraryWithData_error(&data)
            .map_err(|error| MetalError::LibraryLoad(error.to_string()))?;
        let arithmetic_pipeline = make_pipeline(&device, &library, U256_KERNEL)?;
        let opening_pipeline = make_pipeline(&device, &library, OPENING_KERNEL)?;
        // Every official maximum-size and negative-control allocation exists
        // before Gate 0 or any other command can be committed.
        let buffers = preallocate_buffers(&device)?;
        let allocations = allocation_high_water(&buffers)?;
        let maximum_threads = device.maxThreadsPerThreadgroup();
        let profile = DeviceProfile {
            sanitized_name: sanitize_device_name(device.name().to_string()),
            unified_memory: device.hasUnifiedMemory(),
            maximum_buffer_length: device.maxBufferLength() as u64,
            recommended_working_set: device.recommendedMaxWorkingSetSize(),
            maximum_threads: [
                u32::try_from(maximum_threads.width)
                    .map_err(|_| MetalError::LengthOverflow("device max threads x"))?,
                u32::try_from(maximum_threads.height)
                    .map_err(|_| MetalError::LengthOverflow("device max threads y"))?,
                u32::try_from(maximum_threads.depth)
                    .map_err(|_| MetalError::LengthOverflow("device max threads z"))?,
            ],
            maximum_threadgroup_memory: u32::try_from(device.maxThreadgroupMemoryLength())
                .map_err(|_| MetalError::LengthOverflow("device threadgroup memory"))?,
            arithmetic_pipeline: pipeline_limits(&arithmetic_pipeline)?,
            opening_pipeline: pipeline_limits(&opening_pipeline)?,
            allocations,
            gate0_passed: false,
        };
        Ok(Self {
            _device: device,
            queue,
            arithmetic_pipeline,
            opening_pipeline,
            buffers,
            profile,
            reusable: true,
        })
    }

    pub const fn device_profile(&self) -> &DeviceProfile {
        &self.profile
    }

    /// Commit and complete the exact encoder-free Gate-0 command.
    pub fn run_gate0<O, F>(
        &mut self,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<(), MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        if self.profile.gate0_passed {
            return Err(MetalError::Gate0AlreadyPassed);
        }
        self.begin_sequence()?;
        let command = self.new_command()?;
        let _proof = complete(command, observer, on_timeout)?;
        self.profile.gate0_passed = true;
        self.reusable = true;
        Ok(())
    }

    /// Dispatch the exact 16,384-case official arithmetic grid.
    ///
    /// The observer receives and must flush the sole matching TIMEOUT terminal
    /// event. `on_timeout` then flushes any remaining output and exits the child
    /// without emitting another terminal or unwinding (normally exit 124).
    pub fn run_official_arithmetic<O, F>(
        &mut self,
        corpus: &M2ArithmeticCorpusV1,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<AcceptedMetalArithmeticV1, MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        self.require_gate0()?;
        if corpus.inputs().len() != OFFICIAL_ARITHMETIC_CASES
            || corpus.expected_outputs().len() != OFFICIAL_ARITHMETIC_CASES
        {
            return Err(MetalError::WrongRecordCount {
                purpose: "portable official arithmetic corpus",
                expected: OFFICIAL_ARITHMETIC_CASES,
                actual: corpus.inputs().len(),
            });
        }
        let inputs = corpus
            .inputs()
            .iter()
            .copied()
            .map(ArithmeticInputWords::try_from_words)
            .collect::<Result<Vec<_>, _>>()?;
        for (ordinal, input) in inputs.iter().enumerate() {
            if input.words()[1] as usize != ordinal {
                return Err(MetalError::InvalidArithmeticInput {
                    index: ordinal,
                    reason: "case id is not the complete input ordinal",
                });
            }
        }

        let input_words = flatten_arithmetic_inputs(&inputs);
        let output_words = vec![POISON; (OFFICIAL_ARITHMETIC_CASES + 2) * 16];
        self.begin_sequence()?;
        overwrite_idle_words(&self.buffers.arithmetic_input, &input_words)?;
        overwrite_idle_words(&self.buffers.arithmetic_output, &output_words)?;
        let input_pre_digest = stream_digest_words(
            STREAM_ARITHMETIC_INPUT,
            OFFICIAL_ARITHMETIC_CASES,
            &input_words,
        )?;
        let guard_pre_digest = stream_digest_words(
            STREAM_PROTECTED_RECORDS,
            2,
            &output_words[OFFICIAL_ARITHMETIC_CASES * 16..],
        )?;

        let command = self.encode_arithmetic(
            &self.buffers.arithmetic_input,
            &self.buffers.arithmetic_output,
            ArithmeticLaunch::Official,
        )?;
        let proof = complete(command, observer, on_timeout)?;
        let input_post = require_unchanged(
            &proof,
            &self.buffers.arithmetic_input,
            &input_words,
            "arithmetic input changed during dispatch",
        )?;
        let completed = copy_completed_words(&proof, &self.buffers.arithmetic_output)?;
        validate_poison_words(
            &completed[OFFICIAL_ARITHMETIC_CASES * 16..],
            OFFICIAL_ARITHMETIC_CASES,
            "arithmetic guard changed",
        )?;

        let mut cpu_output_words = Vec::with_capacity(OFFICIAL_ARITHMETIC_CASES * 16);
        let mut success_count = 0u32;
        let mut checked_undefined_count = 0u32;
        for (ordinal, ((input, words), portable_expected)) in inputs
            .iter()
            .zip(completed[..OFFICIAL_ARITHMETIC_CASES * 16].chunks_exact(16))
            .zip(corpus.expected_outputs())
            .enumerate()
        {
            let output: [u32; 16] = words
                .try_into()
                .expect("chunks_exact fixes output ABI width");
            let defensive_expected = validate_arithmetic_output(ordinal, input.words(), &output)?;
            if &output != portable_expected || &defensive_expected != portable_expected {
                return Err(MetalError::InvalidOutput {
                    record: ordinal,
                    reason: "GPU or defensive output differs from portable BigUint oracle",
                });
            }
            cpu_output_words.extend_from_slice(portable_expected);
            match output[0] {
                ARITHMETIC_SUCCESS => success_count += 1,
                ARITHMETIC_CHECKED_UNDEFINED => checked_undefined_count += 1,
                _ => unreachable!("validated official status registry"),
            }
        }
        let integrity = ArithmeticRunIntegrity {
            case_count: OFFICIAL_ARITHMETIC_CASES as u32,
            accepted_count: OFFICIAL_ARITHMETIC_CASES as u32,
            success_count,
            checked_undefined_count,
            hard_count: 0,
            allocated_input_bytes: reported_u64(&self.buffers.arithmetic_input)?,
            allocated_output_bytes: reported_u64(&self.buffers.arithmetic_output)?,
            input_pre_digest,
            input_post_digest: stream_digest_words(
                STREAM_ARITHMETIC_INPUT,
                OFFICIAL_ARITHMETIC_CASES,
                &input_post,
            )?,
            cpu_output_digest: stream_digest_words(
                STREAM_ARITHMETIC_OUTPUT,
                OFFICIAL_ARITHMETIC_CASES,
                &cpu_output_words,
            )?,
            gpu_output_digest: stream_digest_words(
                STREAM_ARITHMETIC_OUTPUT,
                OFFICIAL_ARITHMETIC_CASES,
                &completed[..OFFICIAL_ARITHMETIC_CASES * 16],
            )?,
            guard_pre_digest,
            guard_post_digest: stream_digest_words(
                STREAM_PROTECTED_RECORDS,
                2,
                &completed[OFFICIAL_ARITHMETIC_CASES * 16..],
            )?,
        };
        self.reusable = true;
        Ok(AcceptedMetalArithmeticV1 { integrity })
    }

    /// Dispatch and admit the accumulator's next canonical opening task.
    /// Completed arena words are consumed by the opaque portable accumulator
    /// and discarded before this method returns.
    pub fn run_next_opening<O, F>(
        &mut self,
        accumulator: &mut M2GlobalParityAccumulatorV1,
        choose: &OpeningChooseTableV1,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<AcceptedMetalOpeningTaskV1, MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        self.require_gate0()?;
        let task_words = *accumulator
            .next_task()
            .ok_or_else(|| MetalError::PortableValidation("opening accumulator complete".into()))?
            .task_words();
        let task = OpeningTaskWords::try_from_words(task_words)?;
        let choose = OpeningChooseTableWords::try_from_words(*choose.words())?;
        let (integrity, bound) =
            self.run_opening_words(task, &choose, observer, on_timeout, |arena| {
                accumulator
                    .accept_next_task_slot_words_v1(arena)
                    .map_err(portable_error)
            })?;
        let checked = bound.checked_payload();
        if checked.cpu_raw_sha256() != integrity.defensive_cpu_slot_digest {
            self.reusable = false;
            return Err(MetalError::PortableValidation(
                "accepted opening CPU raw digest differs from defensive CPU stream digest".into(),
            ));
        }
        if checked.gpu_raw_sha256() != integrity.gpu_slot_digest {
            self.reusable = false;
            return Err(MetalError::PortableValidation(
                "accepted opening GPU raw digest differs from completed GPU stream digest".into(),
            ));
        }
        Ok(AcceptedMetalOpeningTaskV1 { bound, integrity })
    }

    /// Run the exact grade-seven/matching-six maximum rectangular smoke.
    pub fn run_maximum_smoke<O, F>(
        &mut self,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<MaximumSmokeReport, MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        self.require_gate0()?;
        let carrier = M2OpeningParityCarrierV1::canonical().map_err(portable_error)?;
        let portable_task = carrier
            .tasks()
            .get(109)
            .ok_or_else(|| MetalError::PortableValidation("maximum smoke task missing".into()))?;
        if portable_task.response_count() != 7_980 || portable_task.candidate_slot_count() != 79_800
        {
            return Err(MetalError::PortableValidation(
                "maximum smoke task extent changed".into(),
            ));
        }
        let task = OpeningTaskWords::try_from_words(*portable_task.task_words())?;
        let portable_choose = OpeningChooseTableV1::canonical().map_err(portable_error)?;
        let choose = OpeningChooseTableWords::try_from_words(*portable_choose.words())?;
        let (integrity, ()) =
            self.run_opening_words(task, &choose, observer, on_timeout, |arena| {
                portable_task
                    .validate_slot_words_v1(arena)
                    .map(|_| ())
                    .map_err(portable_error)
            })?;
        if integrity.valid_cell_count != MAX_OPENING_CELLS as u32 {
            return Err(MetalError::PortableValidation(
                "maximum smoke valid-cell census changed".into(),
            ));
        }
        Ok(MaximumSmokeReport { integrity })
    }

    /// Private descriptor path shared by smoke and the portable accepted-task
    /// join. Callers cannot submit arbitrary task words to Metal.
    fn run_opening_words<T, O, F, J>(
        &mut self,
        task: OpeningTaskWords,
        choose: &OpeningChooseTableWords,
        observer: &mut O,
        on_timeout: &mut F,
        join: J,
    ) -> Result<(OpeningRunIntegrity, T), MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
        J: FnOnce(&[[u32; 16]]) -> Result<T, MetalError>,
    {
        let task_words = task.words().to_vec();
        let choose_words = choose.words().to_vec();
        let arena_words = vec![POISON; OPENING_ARENA_RECORDS * WORDS_PER_OPENING_SLOT];
        self.begin_sequence()?;
        overwrite_idle_words(&self.buffers.opening_task, &task_words)?;
        overwrite_idle_words(&self.buffers.opening_choose, &choose_words)?;
        overwrite_idle_words(&self.buffers.opening_output, &arena_words)?;
        let task_pre_digest = ordinary_word_digest(&task_words);
        let choose_pre_digest = ordinary_word_digest(&choose_words);
        let protected_start = task.candidate_slot_count() * 16;
        let protected_count = OPENING_ARENA_RECORDS - task.candidate_slot_count();
        let protected_pre_digest = stream_digest_words(
            STREAM_PROTECTED_RECORDS,
            protected_count,
            &arena_words[protected_start..],
        )?;

        let command = self.encode_opening(
            &self.buffers.opening_task,
            &self.buffers.opening_choose,
            &self.buffers.opening_output,
            OpeningLaunch::ValidatedTask {
                response_count: task.response_triple_count(),
            },
        )?;
        let proof = complete(command, observer, on_timeout)?;
        let task_post = require_unchanged(
            &proof,
            &self.buffers.opening_task,
            &task_words,
            "opening task changed during dispatch",
        )?;
        let choose_post = require_unchanged(
            &proof,
            &self.buffers.opening_choose,
            &choose_words,
            "opening choose table changed during dispatch",
        )?;
        let completed = copy_completed_words(&proof, &self.buffers.opening_output)?;
        let validation = validate_opening_arena(&task, choose, &completed)?;
        let integrity = OpeningRunIntegrity {
            task_ordinal: task.task_ordinal() as u32,
            response_count: task.response_triple_count() as u32,
            candidate_slot_count: task.candidate_slot_count() as u32,
            valid_cell_count: validation.valid_cells,
            allocated_task_bytes: reported_u64(&self.buffers.opening_task)?,
            allocated_choose_bytes: reported_u64(&self.buffers.opening_choose)?,
            allocated_output_bytes: reported_u64(&self.buffers.opening_output)?,
            task_pre_digest,
            task_post_digest: ordinary_word_digest(&task_post),
            choose_pre_digest,
            choose_post_digest: ordinary_word_digest(&choose_post),
            defensive_cpu_slot_digest: stream_digest_words(
                STREAM_CONTEXT_SLOTS,
                task.candidate_slot_count(),
                &validation.expected_words,
            )?,
            gpu_slot_digest: stream_digest_words(
                STREAM_CONTEXT_SLOTS,
                task.candidate_slot_count(),
                &completed[..protected_start],
            )?,
            protected_pre_digest,
            protected_post_digest: stream_digest_words(
                STREAM_PROTECTED_RECORDS,
                protected_count,
                &completed[protected_start..],
            )?,
        };
        let arena = completed
            .chunks_exact(WORDS_PER_OPENING_SLOT)
            .map(|words| {
                words
                    .try_into()
                    .expect("fixed opening arena is a whole number of records")
            })
            .collect::<Vec<[u32; 16]>>();
        if arena.len() != OPENING_ARENA_RECORDS {
            return Err(MetalError::InvalidOutput {
                record: arena.len(),
                reason: "completed opening arena has wrong record count",
            });
        }
        let joined = join(&arena)?;
        drop(arena);
        drop(completed);
        drop(validation);
        self.reusable = true;
        Ok((integrity, joined))
    }

    /// Run the exact closed thirteen-case arithmetic-negative command.
    pub fn run_arithmetic_negative<O, F>(
        &mut self,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<AcceptedMetalArithmeticNegativeV1, MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        self.require_gate0()?;
        let portable_controls =
            ArithmeticNegativeControlsV1::canonical().map_err(portable_error)?;
        let mut arithmetic_input_words = Vec::with_capacity(13 * WORDS_PER_ARITHMETIC_INPUT);
        let mut arithmetic_expected = Vec::with_capacity(13 * WORDS_PER_ARITHMETIC_OUTPUT);
        for control in portable_controls.controls() {
            arithmetic_input_words.extend_from_slice(control.input());
            arithmetic_expected.extend_from_slice(control.expected_output());
        }
        let arithmetic_output_words = vec![POISON; 15 * WORDS_PER_ARITHMETIC_OUTPUT];
        self.begin_sequence()?;
        overwrite_idle_words(
            &self.buffers.negative_arithmetic_input,
            &arithmetic_input_words,
        )?;
        overwrite_idle_words(
            &self.buffers.negative_arithmetic_output,
            &arithmetic_output_words,
        )?;
        let arithmetic_command = self.encode_arithmetic(
            &self.buffers.negative_arithmetic_input,
            &self.buffers.negative_arithmetic_output,
            ArithmeticLaunch::NegativeControl,
        )?;
        let proof = complete(arithmetic_command, observer, on_timeout)?;
        let arithmetic_input_post = require_unchanged(
            &proof,
            &self.buffers.negative_arithmetic_input,
            &arithmetic_input_words,
            "arithmetic negative input changed",
        )?;
        let arithmetic_completed =
            copy_completed_words(&proof, &self.buffers.negative_arithmetic_output)?;
        validate_arithmetic_negative(&arithmetic_completed, &arithmetic_expected)?;
        let integrity = ArithmeticRunIntegrity {
            case_count: 13,
            accepted_count: 0,
            success_count: 0,
            checked_undefined_count: 0,
            hard_count: 13,
            allocated_input_bytes: reported_u64(&self.buffers.negative_arithmetic_input)?,
            allocated_output_bytes: reported_u64(&self.buffers.negative_arithmetic_output)?,
            input_pre_digest: stream_digest_words(
                STREAM_ARITHMETIC_INPUT,
                13,
                &arithmetic_input_words,
            )?,
            input_post_digest: stream_digest_words(
                STREAM_ARITHMETIC_INPUT,
                13,
                &arithmetic_input_post,
            )?,
            cpu_output_digest: stream_digest_words(
                STREAM_ARITHMETIC_OUTPUT,
                13,
                &arithmetic_expected,
            )?,
            gpu_output_digest: stream_digest_words(
                STREAM_ARITHMETIC_OUTPUT,
                13,
                &arithmetic_completed[..13 * 16],
            )?,
            guard_pre_digest: stream_digest_words(
                STREAM_PROTECTED_RECORDS,
                2,
                &arithmetic_output_words[13 * 16..],
            )?,
            guard_post_digest: stream_digest_words(
                STREAM_PROTECTED_RECORDS,
                2,
                &arithmetic_completed[13 * 16..],
            )?,
        };
        self.reusable = true;
        Ok(AcceptedMetalArithmeticNegativeV1 { integrity })
    }

    /// Run one exact private opening-negative command. The ordinal selects one
    /// of the closed thirteen descriptors; no malformed words are exposed.
    pub fn run_opening_negative<O, F>(
        &mut self,
        ordinal: usize,
        choose: &OpeningChooseTableV1,
        observer: &mut O,
        on_timeout: &mut F,
    ) -> Result<AcceptedMetalOpeningNegativeV1, MetalError>
    where
        O: FnMut(CommandEvent),
        F: FnMut(CommandState) -> Infallible,
    {
        self.require_gate0()?;
        let portable_controls = M2OpeningNegativeControlsV1::canonical().map_err(portable_error)?;
        if ordinal >= portable_controls.controls().len() {
            return Err(MetalError::InvalidNegativeOrdinal {
                domain: "opening",
                ordinal,
                count: portable_controls.controls().len(),
            });
        }
        let control = &portable_controls.controls()[ordinal];
        let task_words = *control.task_words();
        let choose = OpeningChooseTableWords::try_from_words(*choose.words())?;
        let choose_words = choose.words().to_vec();
        let poison_words = vec![POISON; 12 * WORDS_PER_OPENING_SLOT];
        self.begin_sequence()?;
        overwrite_idle_words(&self.buffers.negative_opening_tasks[ordinal], &task_words)?;
        overwrite_idle_words(&self.buffers.opening_choose, &choose_words)?;
        overwrite_idle_words(
            &self.buffers.negative_opening_outputs[ordinal],
            &poison_words,
        )?;
        let command = self.encode_opening(
            &self.buffers.negative_opening_tasks[ordinal],
            &self.buffers.opening_choose,
            &self.buffers.negative_opening_outputs[ordinal],
            OpeningLaunch::NegativeControl,
        )?;
        let proof = complete(command, observer, on_timeout)?;
        let task_post = require_unchanged(
            &proof,
            &self.buffers.negative_opening_tasks[ordinal],
            &task_words,
            "opening negative task changed",
        )?;
        let choose_post = require_unchanged(
            &proof,
            &self.buffers.opening_choose,
            &choose_words,
            "opening negative choose table changed",
        )?;
        let completed =
            copy_completed_words(&proof, &self.buffers.negative_opening_outputs[ordinal])?;
        validate_opening_negative(ordinal, control.expected_slots(), &completed)?;
        let integrity = OpeningNegativeIntegrity {
            ordinal: ordinal as u32,
            task_pre_digest: ordinary_word_digest(&task_words),
            task_post_digest: ordinary_word_digest(&task_post),
            choose_pre_digest: ordinary_word_digest(&choose_words),
            choose_post_digest: ordinary_word_digest(&choose_post),
            protected_pre_digest: stream_digest_words(STREAM_PROTECTED_RECORDS, 12, &poison_words)?,
            protected_post_digest: stream_digest_words(STREAM_PROTECTED_RECORDS, 12, &completed)?,
        };
        self.reusable = true;
        Ok(AcceptedMetalOpeningNegativeV1 { integrity })
    }

    fn require_gate0(&self) -> Result<(), MetalError> {
        if self.profile.gate0_passed {
            Ok(())
        } else {
            Err(MetalError::Gate0Required)
        }
    }

    fn begin_sequence(&mut self) -> Result<(), MetalError> {
        if !self.reusable {
            return Err(MetalError::RuntimePoisoned);
        }
        self.reusable = false;
        Ok(())
    }

    fn new_command(&self) -> Result<BoundCommand, MetalError> {
        self.queue
            .commandBuffer()
            .map(BoundCommand::new)
            .ok_or(MetalError::NoCommandBuffer)
    }

    fn encode_arithmetic(
        &self,
        input: &ValidatedBuffer,
        output: &ValidatedBuffer,
        launch: ArithmeticLaunch,
    ) -> Result<BoundCommand, MetalError> {
        let contract = arithmetic_launch_contract(
            launch,
            self.arithmetic_pipeline.maxTotalThreadsPerThreadgroup(),
        )?;
        let mut command = self.new_command()?;
        let encoder = command
            .raw()
            .computeCommandEncoder()
            .ok_or(MetalError::EncoderCreation)?;
        encoder.setComputePipelineState(&self.arithmetic_pipeline);
        bind_zero(
            &mut command,
            &encoder,
            input,
            0,
            BufferRole::ArithmeticInput,
            contract.input_words,
        )?;
        bind_zero(
            &mut command,
            &encoder,
            output,
            1,
            BufferRole::ArithmeticOutput,
            contract.output_words,
        )?;
        encoder.dispatchThreads_threadsPerThreadgroup(
            mtl_size(contract.grid),
            mtl_size(THREADGROUP_WIDTH),
        );
        encoder.endEncoding();
        Ok(command)
    }

    fn encode_opening(
        &self,
        task: &ValidatedBuffer,
        choose: &ValidatedBuffer,
        output: &ValidatedBuffer,
        launch: OpeningLaunch,
    ) -> Result<BoundCommand, MetalError> {
        let contract = opening_launch_contract(
            launch,
            self.opening_pipeline.maxTotalThreadsPerThreadgroup(),
        )?;
        let mut command = self.new_command()?;
        let encoder = command
            .raw()
            .computeCommandEncoder()
            .ok_or(MetalError::EncoderCreation)?;
        encoder.setComputePipelineState(&self.opening_pipeline);
        bind_zero(
            &mut command,
            &encoder,
            task,
            0,
            BufferRole::OpeningTask,
            contract.input_words,
        )?;
        bind_zero(
            &mut command,
            &encoder,
            choose,
            1,
            BufferRole::OpeningChoose,
            contract.auxiliary_words,
        )?;
        bind_zero(
            &mut command,
            &encoder,
            output,
            2,
            BufferRole::OpeningOutput,
            contract.output_words,
        )?;
        encoder.dispatchThreads_threadsPerThreadgroup(
            mtl_size(contract.grid),
            mtl_size(THREADGROUP_WIDTH),
        );
        encoder.endEncoding();
        Ok(command)
    }
}

fn require_launch_threadgroup_limit(kernel: &'static str, actual: usize) -> Result<(), MetalError> {
    if actual < THREADGROUP_WIDTH {
        return Err(MetalError::PipelineThreadLimit {
            kernel,
            actual,
            required: THREADGROUP_WIDTH,
        });
    }
    Ok(())
}

fn arithmetic_launch_contract(
    launch: ArithmeticLaunch,
    maximum_threads: usize,
) -> Result<LaunchContract, MetalError> {
    require_launch_threadgroup_limit(U256_KERNEL, maximum_threads)?;
    let grid = match launch {
        ArithmeticLaunch::Official => OFFICIAL_ARITHMETIC_CASES,
        ArithmeticLaunch::NegativeControl => 13,
    };
    let input_words = grid
        .checked_mul(WORDS_PER_ARITHMETIC_INPUT)
        .ok_or(MetalError::LengthOverflow("arithmetic launch input"))?;
    let output_words = grid
        .checked_add(2)
        .and_then(|records| records.checked_mul(WORDS_PER_ARITHMETIC_OUTPUT))
        .ok_or(MetalError::LengthOverflow("arithmetic launch output"))?;
    Ok(LaunchContract {
        grid,
        input_words,
        auxiliary_words: 0,
        output_words,
    })
}

fn opening_launch_contract(
    launch: OpeningLaunch,
    maximum_threads: usize,
) -> Result<LaunchContract, MetalError> {
    require_launch_threadgroup_limit(OPENING_KERNEL, maximum_threads)?;
    let (grid, output_words) = match launch {
        OpeningLaunch::ValidatedTask { response_count } => {
            if ![6, 120, 504, 1_320, 2_730, 4_896, 7_980].contains(&response_count) {
                return Err(MetalError::InvalidOpeningTask(
                    "response grid is outside frozen grade extents",
                ));
            }
            (
                response_count,
                OPENING_ARENA_RECORDS
                    .checked_mul(WORDS_PER_OPENING_SLOT)
                    .ok_or(MetalError::LengthOverflow("opening launch output"))?,
            )
        }
        OpeningLaunch::NegativeControl => (1, 12 * WORDS_PER_OPENING_SLOT),
    };
    Ok(LaunchContract {
        grid,
        input_words: 8,
        auxiliary_words: 22 * 22,
        output_words,
    })
}

fn preallocate_buffers(device: &ProtocolObject<dyn MTLDevice>) -> Result<BufferSet, MetalError> {
    let mut negative_opening_tasks = Vec::with_capacity(13);
    let mut negative_opening_outputs = Vec::with_capacity(13);
    for _ in 0..13 {
        negative_opening_tasks.push(allocate_words_on(
            device,
            &[0; 8],
            BufferRole::OpeningTask,
            "opening negative task",
        )?);
        negative_opening_outputs.push(allocate_words_on(
            device,
            &vec![POISON; 12 * WORDS_PER_OPENING_SLOT],
            BufferRole::OpeningOutput,
            "opening negative output and guards",
        )?);
    }
    Ok(BufferSet {
        arithmetic_input: allocate_words_on(
            device,
            &vec![0; OFFICIAL_ARITHMETIC_CASES * WORDS_PER_ARITHMETIC_INPUT],
            BufferRole::ArithmeticInput,
            "official arithmetic input",
        )?,
        arithmetic_output: allocate_words_on(
            device,
            &vec![POISON; (OFFICIAL_ARITHMETIC_CASES + 2) * WORDS_PER_ARITHMETIC_OUTPUT],
            BufferRole::ArithmeticOutput,
            "official arithmetic output and guards",
        )?,
        opening_task: allocate_words_on(device, &[0; 8], BufferRole::OpeningTask, "opening task")?,
        opening_choose: allocate_words_on(
            device,
            &[0; 22 * 22],
            BufferRole::OpeningChoose,
            "opening choose table",
        )?,
        opening_output: allocate_words_on(
            device,
            &vec![POISON; OPENING_ARENA_RECORDS * WORDS_PER_OPENING_SLOT],
            BufferRole::OpeningOutput,
            "opening fixed output arena",
        )?,
        negative_arithmetic_input: allocate_words_on(
            device,
            &vec![0; 13 * WORDS_PER_ARITHMETIC_INPUT],
            BufferRole::ArithmeticInput,
            "arithmetic negative input",
        )?,
        negative_arithmetic_output: allocate_words_on(
            device,
            &vec![POISON; 15 * WORDS_PER_ARITHMETIC_OUTPUT],
            BufferRole::ArithmeticOutput,
            "arithmetic negative output and guards",
        )?,
        negative_opening_tasks,
        negative_opening_outputs,
    })
}

fn allocate_words_on(
    device: &ProtocolObject<dyn MTLDevice>,
    words: &[u32],
    role: BufferRole,
    purpose: &'static str,
) -> Result<ValidatedBuffer, MetalError> {
    let bytes = words
        .len()
        .checked_mul(size_of::<u32>())
        .ok_or(MetalError::LengthOverflow(purpose))?;
    let raw = device
        .newBufferWithLength_options(bytes, MTLResourceOptions::StorageModeShared)
        .ok_or(MetalError::AllocationFailure { purpose, bytes })?;
    copy_host_words(raw, words.len(), words, role, purpose)
}

fn allocation_high_water(buffers: &BufferSet) -> Result<AllocationHighWater, MetalError> {
    let projector_logical = checked_sum_bytes(
        "projector logical allocation",
        [
            buffers.opening_task.logical_bytes()?,
            buffers.opening_choose.logical_bytes()?,
            buffers.opening_output.logical_bytes()?,
        ],
    )?;
    let projector_reported = checked_sum_bytes(
        "projector reported allocation",
        [
            buffers.opening_task.reported_bytes(),
            buffers.opening_choose.reported_bytes(),
            buffers.opening_output.reported_bytes(),
        ],
    )?;
    let arithmetic_logical = checked_sum_bytes(
        "arithmetic logical allocation",
        [
            buffers.arithmetic_input.logical_bytes()?,
            buffers.arithmetic_output.logical_bytes()?,
        ],
    )?;
    let arithmetic_reported = checked_sum_bytes(
        "arithmetic reported allocation",
        [
            buffers.arithmetic_input.reported_bytes(),
            buffers.arithmetic_output.reported_bytes(),
        ],
    )?;
    if projector_logical != 5_109_296 || arithmetic_logical != 2_359_424 {
        return Err(MetalError::LengthOverflow(
            "frozen arena high-water mismatch",
        ));
    }
    Ok(AllocationHighWater {
        projector_logical_bytes: projector_logical as u64,
        projector_reported_bytes: projector_reported as u64,
        arithmetic_logical_bytes: arithmetic_logical as u64,
        arithmetic_reported_bytes: arithmetic_reported as u64,
    })
}

fn checked_sum_bytes<const N: usize>(
    purpose: &'static str,
    values: [usize; N],
) -> Result<usize, MetalError> {
    values.into_iter().try_fold(0usize, |sum, value| {
        sum.checked_add(value)
            .ok_or(MetalError::LengthOverflow(purpose))
    })
}

fn sanitize_device_name(name: String) -> String {
    name.chars()
        .map(|character| match character {
            '\0' | '\r' | '\n' => ' ',
            other => other,
        })
        .collect::<String>()
        .trim()
        .to_owned()
}

fn portable_error(error: walt_gpu_ref::M2BridgeError) -> MetalError {
    MetalError::PortableValidation(error.to_string())
}

fn pipeline_limits(
    pipeline: &ProtocolObject<dyn MTLComputePipelineState>,
) -> Result<PipelineLimits, MetalError> {
    Ok(PipelineLimits {
        execution_width: u32::try_from(pipeline.threadExecutionWidth())
            .map_err(|_| MetalError::LengthOverflow("pipeline execution width"))?,
        maximum_threads: u32::try_from(pipeline.maxTotalThreadsPerThreadgroup())
            .map_err(|_| MetalError::LengthOverflow("pipeline maximum threads"))?,
        static_threadgroup_memory: pipeline.staticThreadgroupMemoryLength() as u64,
    })
}

fn reported_u64(buffer: &ValidatedBuffer) -> Result<u64, MetalError> {
    u64::try_from(buffer.reported_bytes())
        .map_err(|_| MetalError::LengthOverflow("reported Metal allocation"))
}

fn ordinary_word_digest(words: &[u32]) -> [u8; 32] {
    let mut digest = Sha256State::new();
    for word in words {
        digest.update(&word.to_le_bytes());
    }
    digest.finish()
}

fn stream_digest_words(
    purpose: u32,
    record_count: usize,
    words: &[u32],
) -> Result<[u8; 32], MetalError> {
    let record_count = u64::try_from(record_count)
        .map_err(|_| MetalError::LengthOverflow("stream record count"))?;
    let payload_bytes = words
        .len()
        .checked_mul(size_of::<u32>())
        .ok_or(MetalError::LengthOverflow("stream payload bytes"))?;
    let payload_bytes = u64::try_from(payload_bytes)
        .map_err(|_| MetalError::LengthOverflow("stream payload bytes"))?;
    let mut digest = Sha256State::new();
    digest.update(STREAM_MAGIC);
    digest.update(&purpose.to_le_bytes());
    digest.update(&STREAM_VERSION.to_le_bytes());
    digest.update(&record_count.to_le_bytes());
    digest.update(&payload_bytes.to_le_bytes());
    for word in words {
        digest.update(&word.to_le_bytes());
    }
    Ok(digest.finish())
}

fn make_pipeline(
    device: &ProtocolObject<dyn MTLDevice>,
    library: &ProtocolObject<dyn MTLLibrary>,
    name: &'static str,
) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError> {
    let name_string = NSString::from_str(name);
    let function = library
        .newFunctionWithName(&name_string)
        .ok_or(MetalError::MissingFunction(name))?;
    let pipeline = device
        .newComputePipelineStateWithFunction_error(&function)
        .map_err(|error| MetalError::PipelineCreation(name, error.to_string()))?;
    let actual = pipeline.maxTotalThreadsPerThreadgroup();
    if actual < THREADGROUP_WIDTH {
        return Err(MetalError::PipelineThreadLimit {
            kernel: name,
            actual,
            required: THREADGROUP_WIDTH,
        });
    }
    Ok(pipeline)
}

#[allow(unreachable_code)] // The stable uninhabited callback is the `!` contract.
fn complete<O, F>(
    command: BoundCommand,
    observer: &mut O,
    on_timeout: &mut F,
) -> Result<CompletionProof, MetalError>
where
    O: FnMut(CommandEvent),
    F: FnMut(CommandState) -> Infallible,
{
    command.raw().commit();
    let started = Instant::now();
    observer(CommandEvent::Committed);
    match poll_completion_state(
        || command_state(command.raw().status()),
        || started.elapsed() >= COMMAND_TIMEOUT,
        || thread::sleep(POLL_INTERVAL),
        observer,
        on_timeout,
    )? {
        CompletionPollExit::Completed => {
            if let Some(error) = command.raw().error() {
                observer(CommandEvent::Terminal(CommandTerminal::Error));
                return Err(MetalError::CommandError(error.to_string()));
            }
            let proof = match prove_completed(command) {
                Ok(proof) => proof,
                Err(error) => {
                    observer(CommandEvent::Terminal(CommandTerminal::Error));
                    return Err(error);
                }
            };
            observer(CommandEvent::Terminal(CommandTerminal::Completed));
            Ok(proof)
        }
        CompletionPollExit::Error => {
            if let Some(error) = command.raw().error() {
                Err(MetalError::CommandError(error.to_string()))
            } else {
                Err(MetalError::UnexpectedCommandState(CommandState::Error))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompletionPollExit {
    Completed,
    Error,
}

/// Exact host completion classifier with injected state, deadline, and wait
/// sources.  Production supplies the retained MTL command and `Instant`; unit
/// controls supply finite deterministic sequences without submitting a
/// deliberately nonterminating kernel.
fn poll_completion_state<S, D, W, O, F>(
    mut state: S,
    mut deadline_reached: D,
    mut wait: W,
    observer: &mut O,
    on_timeout: &mut F,
) -> Result<CompletionPollExit, MetalError>
where
    S: FnMut() -> CommandState,
    D: FnMut() -> bool,
    W: FnMut(),
    O: FnMut(CommandEvent),
    F: FnMut(CommandState) -> Infallible,
{
    loop {
        let timed_out = deadline_reached();
        let current = state();
        if timed_out {
            terminate_timeout(observer, on_timeout, current);
        }
        match current {
            CommandState::Completed => return Ok(CompletionPollExit::Completed),
            CommandState::Error => {
                observer(CommandEvent::Terminal(CommandTerminal::Error));
                return Ok(CompletionPollExit::Error);
            }
            CommandState::NotEnqueued => {
                observer(CommandEvent::Terminal(CommandTerminal::NotEnqueued));
                return Err(MetalError::UnexpectedCommandState(
                    CommandState::NotEnqueued,
                ));
            }
            CommandState::Unknown(status) => {
                observer(CommandEvent::Terminal(CommandTerminal::Unknown(status)));
                return Err(MetalError::UnexpectedCommandState(CommandState::Unknown(
                    status,
                )));
            }
            CommandState::Enqueued | CommandState::Committed | CommandState::Scheduled => {
                wait();
            }
        }
    }
}

fn terminate_timeout<O, F>(
    observer: &mut O,
    on_timeout: &mut F,
    last_state: CommandState,
) -> Infallible
where
    O: FnMut(CommandEvent),
    F: FnMut(CommandState) -> Infallible,
{
    // Once the deadline sample is true, no subsequently observed native state
    // can be accepted as success. `last_state` is diagnostics for the exit
    // callback, not a second terminal classification.
    observer(CommandEvent::Terminal(CommandTerminal::Timeout));
    on_timeout(last_state)
}

fn require_unchanged(
    proof: &CompletionProof,
    buffer: &ValidatedBuffer,
    expected: &[u32],
    reason: &'static str,
) -> Result<Vec<u32>, MetalError> {
    let actual = copy_completed_words(proof, buffer)?;
    if actual != expected {
        return Err(MetalError::InvalidOutput { record: 0, reason });
    }
    Ok(actual)
}

fn mtl_size(width: usize) -> MTLSize {
    MTLSize {
        width,
        height: 1,
        depth: 1,
    }
}

fn flatten_arithmetic_inputs(inputs: &[ArithmeticInputWords]) -> Vec<u32> {
    let mut words = Vec::with_capacity(inputs.len() * WORDS_PER_ARITHMETIC_INPUT);
    for input in inputs {
        words.extend_from_slice(input.words());
    }
    words
}

fn validate_poison_words(
    words: &[u32],
    record: usize,
    reason: &'static str,
) -> Result<(), MetalError> {
    if words.iter().any(|word| *word != POISON) {
        return Err(MetalError::InvalidOutput { record, reason });
    }
    Ok(())
}

fn validate_arithmetic_output(
    ordinal: usize,
    input: &[u32; 20],
    output: &[u32; 16],
) -> Result<[u32; 16], MetalError> {
    if output[1] != input[1] || output[2] != input[2] {
        return Err(MetalError::InvalidOutput {
            record: ordinal,
            reason: "case id or operation changed",
        });
    }
    let expected = render_arithmetic(input);
    if output != &expected {
        return Err(MetalError::InvalidOutput {
            record: ordinal,
            reason: "record differs from integer host rendering",
        });
    }
    if !matches!(output[0], ARITHMETIC_SUCCESS | ARITHMETIC_CHECKED_UNDEFINED) {
        return Err(MetalError::InvalidOutput {
            record: ordinal,
            reason: "official corpus produced a hard or unknown status",
        });
    }
    Ok(expected)
}

fn render_arithmetic(input: &[u32; 20]) -> [u32; 16] {
    let mut output = [0u32; 16];
    output[1] = input[1];
    output[2] = input[2];
    let mut result = [0u32; 8];
    let defined = match input[2] {
        1 => checked_add(&input[4..12], &input[12..20], &mut result),
        2 => checked_sub(&input[4..12], &input[12..20], &mut result),
        3 => checked_mul_small(&input[4..12], input[3], &mut result),
        4 => {
            result.copy_from_slice(&input[4..12]);
            let mut defined = true;
            for _ in 0..input[3] {
                let mut next = [0u32; 8];
                if !checked_mul_small(&result, 420, &mut next) {
                    defined = false;
                    break;
                }
                result = next;
            }
            defined
        }
        5 => {
            // Slice comparison is most-significant-last for this ABI, so scan
            // the limbs explicitly in descending order.
            let mut encoded = 2;
            for limb in (0..8).rev() {
                if input[4 + limb] < input[12 + limb] {
                    encoded = 1;
                    break;
                }
                if input[4 + limb] > input[12 + limb] {
                    encoded = 3;
                    break;
                }
            }
            output[0] = ARITHMETIC_SUCCESS;
            output[3] = 1;
            output[4] = encoded;
            return output;
        }
        _ => false,
    };
    if !defined {
        output[0] = ARITHMETIC_CHECKED_UNDEFINED;
        return output;
    }
    output[0] = ARITHMETIC_SUCCESS;
    output[3] = 1;
    output[5..13].copy_from_slice(&result);
    output
}

fn checked_add(lhs: &[u32], rhs: &[u32], output: &mut [u32; 8]) -> bool {
    let mut carry = 0u64;
    for limb in 0..8 {
        let sum = u64::from(lhs[limb]) + u64::from(rhs[limb]) + carry;
        output[limb] = sum as u32;
        carry = sum >> 32;
    }
    carry == 0
}

fn checked_sub(lhs: &[u32], rhs: &[u32], output: &mut [u32; 8]) -> bool {
    let mut borrow = 0u64;
    for limb in 0..8 {
        let left = u64::from(lhs[limb]);
        let subtrahend = u64::from(rhs[limb]) + borrow;
        if left >= subtrahend {
            output[limb] = (left - subtrahend) as u32;
            borrow = 0;
        } else {
            output[limb] = ((1u64 << 32) + left - subtrahend) as u32;
            borrow = 1;
        }
    }
    borrow == 0
}

fn checked_mul_small(lhs: &[u32], factor: u32, output: &mut [u32; 8]) -> bool {
    let mut carry = 0u64;
    for limb in 0..8 {
        let product = u64::from(lhs[limb]) * u64::from(factor) + carry;
        output[limb] = product as u32;
        carry = product >> 32;
    }
    carry == 0
}

fn validate_arithmetic_negative(words: &[u32], expected: &[u32]) -> Result<(), MetalError> {
    if expected.len() != 13 * 16 {
        return Err(MetalError::WrongRecordCount {
            purpose: "portable arithmetic negative output",
            expected: 13,
            actual: expected.len() / 16,
        });
    }
    for (ordinal, (output, expected_output)) in words[..13 * 16]
        .chunks_exact(16)
        .zip(expected.chunks_exact(16))
        .enumerate()
    {
        if output != expected_output {
            return Err(MetalError::InvalidOutput {
                record: ordinal,
                reason: "arithmetic negative control differs from portable canonical hard",
            });
        }
    }
    validate_poison_words(&words[13 * 16..], 13, "arithmetic negative guard changed")
}

fn validate_opening_negative(
    command_ordinal: usize,
    expected: &[[u32; 16]; 12],
    words: &[u32],
) -> Result<(), MetalError> {
    for (local, (output, expected_output)) in words.chunks_exact(16).zip(expected).enumerate() {
        if output != expected_output {
            return Err(MetalError::InvalidOutput {
                record: command_ordinal * 10 + local,
                reason: "opening negative control differs from portable expected arena",
            });
        }
    }
    Ok(())
}

struct OpeningValidation {
    expected_words: Vec<u32>,
    valid_cells: u32,
}

fn validate_opening_arena(
    task: &OpeningTaskWords,
    choose: &OpeningChooseTableWords,
    words: &[u32],
) -> Result<OpeningValidation, MetalError> {
    let candidate_words = task.candidate_slot_count() * WORDS_PER_OPENING_SLOT;
    let mut expected_words = Vec::with_capacity(candidate_words);
    let mut valid_cells = 0u32;
    let mut total_mass = 0u64;
    for q in 0..task.response_triple_count() {
        let expected = render_opening_response(task, choose, q)?;
        let first = q * 10 * WORDS_PER_OPENING_SLOT;
        for (local, expected_slot) in expected.iter().enumerate() {
            let record = q * 10 + local;
            let offset = first + local * WORDS_PER_OPENING_SLOT;
            if !matches!(words[offset], OPENING_SKIP | OPENING_VALID) {
                return Err(MetalError::InvalidOutput {
                    record,
                    reason: "opening output produced a hard or unknown status",
                });
            }
            if words[offset] == OPENING_VALID {
                let support = words[offset + 9];
                let coefficient =
                    u64::from(words[offset + 10]) | (u64::from(words[offset + 11]) << 32);
                let mass = u64::from(words[offset + 12]) | (u64::from(words[offset + 13]) << 32);
                validate_opening_cell_bounds(record, support, coefficient, mass)?;
                total_mass = total_mass
                    .checked_add(mass)
                    .ok_or(MetalError::InvalidOutput {
                        record,
                        reason: "opening whole mass arithmetic overflow",
                    })?;
                require_opening_bound(
                    record,
                    total_mass,
                    OPENING_MAX_WHOLE_MASS_V1,
                    "opening whole mass exceeds frozen maximum",
                )?;
                valid_cells = valid_cells
                    .checked_add(1)
                    .ok_or(MetalError::InvalidOutput {
                        record,
                        reason: "opening valid-cell census overflow",
                    })?;
            }
            if words[offset..offset + 16] != *expected_slot {
                return Err(MetalError::InvalidOutput {
                    record,
                    reason: "record differs from integer host rendering",
                });
            }
            expected_words.extend_from_slice(expected_slot);
        }
    }
    if expected_words.len() != candidate_words {
        return Err(MetalError::InvalidOutput {
            record: task.candidate_slot_count(),
            reason: "host renderer produced wrong candidate length",
        });
    }
    if valid_cells as usize > MAX_OPENING_CELLS {
        return Err(MetalError::InvalidOutput {
            record: task.candidate_slot_count(),
            reason: "opening valid-cell census exceeds frozen cap",
        });
    }
    validate_poison_words(
        &words[candidate_words..],
        task.candidate_slot_count(),
        "opening tail or guard changed",
    )?;
    Ok(OpeningValidation {
        expected_words,
        valid_cells,
    })
}

fn render_opening_response(
    task: &OpeningTaskWords,
    choose: &OpeningChooseTableWords,
    q: usize,
) -> Result<[[u32; 16]; 10], MetalError> {
    let task_words = task.words();
    let response = decode_response(task_words[3], task_words[5] as usize, q).ok_or(
        MetalError::InvalidOutput {
            record: q * 10,
            reason: "host could not decode validated response ordinal",
        },
    )?;
    let follower = response.map(|physical| task_words[4] & (1u32 << physical) != 0);
    let follower_count = follower.iter().filter(|value| **value).count();
    let matching_count = task_words[4].count_ones() as usize;
    let capacity = task_words[2] as usize - 1;
    let remaining_matching = matching_count - follower_count;
    let remaining_nonmatching = task_words[5] as usize - matching_count - (3 - follower_count);

    let mut rendered = [[0u32; 16]; 10];
    for (local, slot) in rendered.iter_mut().enumerate() {
        slot[0] = OPENING_SKIP;
        slot[1] = task_words[1];
        slot[2] = (q * 10 + local) as u32;
    }
    let mut local = 0;
    for first in 0..=capacity {
        if !follower[0] && first != 0 {
            continue;
        }
        for second in 0..=capacity {
            if !follower[1] && second != 0 {
                continue;
            }
            for third in 0..=capacity {
                if !follower[2] && third != 0 {
                    continue;
                }
                let counts = [first, second, third];
                if counts.iter().sum::<usize>() != remaining_matching {
                    continue;
                }
                let support = opening_support(
                    choose,
                    remaining_matching,
                    remaining_nonmatching,
                    capacity,
                    counts,
                )?;
                if support == 0 {
                    continue;
                }
                if local >= 10 {
                    return Err(MetalError::InvalidOutput {
                        record: q * 10,
                        reason: "host rendering exceeded ten strata",
                    });
                }
                let coefficient = opening_coefficient(task_words[2] as usize, follower, counts)?;
                let mass = u64::from(support).checked_mul(coefficient).ok_or(
                    MetalError::InvalidOutput {
                        record: q * 10 + local,
                        reason: "host mass overflow",
                    },
                )?;
                validate_opening_cell_bounds(q * 10 + local, support, coefficient, mass)?;
                let slot = &mut rendered[local];
                slot[0] = OPENING_VALID;
                slot[3..6].copy_from_slice(&response);
                slot[6] = first as u32;
                slot[7] = second as u32;
                slot[8] = third as u32;
                slot[9] = support;
                slot[10] = coefficient as u32;
                slot[11] = (coefficient >> 32) as u32;
                slot[12] = mass as u32;
                slot[13] = (mass >> 32) as u32;
                local += 1;
            }
        }
    }
    Ok(rendered)
}

fn decode_response(pool_mask: u32, pool_count: usize, q: usize) -> Option<[u32; 3]> {
    let row = (pool_count - 1) * (pool_count - 2);
    let first_position = q / row;
    let remainder = q % row;
    let second_rank = remainder / (pool_count - 2);
    let third_rank = remainder % (pool_count - 2);
    let second_position = if second_rank >= first_position {
        second_rank + 1
    } else {
        second_rank
    };
    let lower = first_position.min(second_position);
    let higher = first_position.max(second_position);
    let mut third_position = third_rank;
    if third_position >= lower {
        third_position += 1;
    }
    if third_position >= higher {
        third_position += 1;
    }
    Some([
        select_set_bit(pool_mask, first_position)?,
        select_set_bit(pool_mask, second_position)?,
        select_set_bit(pool_mask, third_position)?,
    ])
}

fn select_set_bit(mask: u32, ordinal: usize) -> Option<u32> {
    let mut seen = 0;
    for physical in 0..28 {
        if mask & (1u32 << physical) != 0 {
            if seen == ordinal {
                return Some(physical);
            }
            seen += 1;
        }
    }
    None
}

fn opening_support(
    choose: &OpeningChooseTableWords,
    remaining_matching: usize,
    remaining_nonmatching: usize,
    capacity: usize,
    counts: [usize; 3],
) -> Result<u32, MetalError> {
    let mut available_matching = remaining_matching;
    let mut available_nonmatching = remaining_nonmatching;
    let mut support = 1u32;
    for matching in counts {
        let nonmatching = capacity - matching;
        if matching > available_matching || nonmatching > available_nonmatching {
            return Ok(0);
        }
        let matching_choose = choose.words()[available_matching * 22 + matching];
        let nonmatching_choose = choose.words()[available_nonmatching * 22 + nonmatching];
        support = support
            .checked_mul(matching_choose)
            .ok_or(MetalError::InvalidOutput {
                record: 0,
                reason: "host support overflow",
            })?;
        require_opening_bound(
            0,
            u64::from(support),
            u64::from(OPENING_MAX_CELL_SUPPORT_V1),
            "host support exceeds frozen maximum",
        )?;
        support = support
            .checked_mul(nonmatching_choose)
            .ok_or(MetalError::InvalidOutput {
                record: 0,
                reason: "host support overflow",
            })?;
        require_opening_bound(
            0,
            u64::from(support),
            u64::from(OPENING_MAX_CELL_SUPPORT_V1),
            "host support exceeds frozen maximum",
        )?;
        available_matching -= matching;
        available_nonmatching -= nonmatching;
    }
    if available_matching != 0 || available_nonmatching != 0 {
        return Ok(0);
    }
    Ok(support)
}

fn opening_coefficient(
    grade: usize,
    follower: [bool; 3],
    counts: [usize; 3],
) -> Result<u64, MetalError> {
    let mut coefficient = 1u64;
    for seat in 0..3 {
        let divisor = if follower[seat] {
            counts[seat] + 1
        } else {
            grade
        };
        if divisor == 0 || 420 % divisor != 0 {
            return Err(MetalError::InvalidOutput {
                record: 0,
                reason: "host coefficient divisor is not supported",
            });
        }
        coefficient =
            coefficient
                .checked_mul((420 / divisor) as u64)
                .ok_or(MetalError::InvalidOutput {
                    record: 0,
                    reason: "host coefficient overflow",
                })?;
        require_opening_bound(
            0,
            coefficient,
            OPENING_MAX_CELL_COEFFICIENT_V1,
            "host coefficient exceeds frozen maximum",
        )?;
    }
    Ok(coefficient)
}

fn require_opening_bound(
    record: usize,
    actual: u64,
    maximum: u64,
    reason: &'static str,
) -> Result<(), MetalError> {
    if actual > maximum {
        return Err(MetalError::InvalidOutput { record, reason });
    }
    Ok(())
}

fn validate_opening_cell_bounds(
    record: usize,
    support: u32,
    coefficient: u64,
    mass: u64,
) -> Result<(), MetalError> {
    require_opening_bound(
        record,
        u64::from(support),
        u64::from(OPENING_MAX_CELL_SUPPORT_V1),
        "opening cell support exceeds frozen maximum",
    )?;
    require_opening_bound(
        record,
        coefficient,
        OPENING_MAX_CELL_COEFFICIENT_V1,
        "opening cell coefficient exceeds frozen maximum",
    )?;
    require_opening_bound(
        record,
        mass,
        OPENING_MAX_CELL_MASS_V1,
        "opening cell mass exceeds frozen maximum",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_emits_one_timeout_terminal_before_exit_callback() {
        for active in [
            CommandState::Enqueued,
            CommandState::Committed,
            CommandState::Scheduled,
            CommandState::Completed,
        ] {
            let mut events = Vec::new();
            let mut callback_state = None;
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut observer = |event| events.push(event);
                let mut on_timeout = |state| -> Infallible {
                    callback_state = Some(state);
                    panic!("stand-in for process exit")
                };
                poll_completion_state(
                    || active,
                    || true,
                    || panic!("deadline transition must not wait"),
                    &mut observer,
                    &mut on_timeout,
                )
            }));

            assert!(result.is_err());
            assert_eq!(events, [CommandEvent::Terminal(CommandTerminal::Timeout)]);
            assert_eq!(callback_state, Some(active));
        }
    }

    #[test]
    fn injected_completion_source_covers_every_closed_state_class() {
        let mut sequence = [
            CommandState::Enqueued,
            CommandState::Committed,
            CommandState::Scheduled,
            CommandState::Completed,
        ]
        .into_iter();
        let mut waits = 0u32;
        let mut events = Vec::new();
        let mut observer = |event| events.push(event);
        let mut impossible_timeout = |_| -> Infallible { panic!("unexpected timeout") };
        let completed = poll_completion_state(
            || sequence.next().expect("finite completion sequence"),
            || false,
            || waits += 1,
            &mut observer,
            &mut impossible_timeout,
        )
        .expect("completed injected sequence");
        assert_eq!(completed, CompletionPollExit::Completed);
        assert_eq!(waits, 3);
        assert!(events.is_empty());

        for (state, terminal) in [
            (CommandState::Error, CommandTerminal::Error),
            (CommandState::NotEnqueued, CommandTerminal::NotEnqueued),
            (CommandState::Unknown(91), CommandTerminal::Unknown(91)),
        ] {
            let mut observed = Vec::new();
            let mut observer = |event| observed.push(event);
            let mut impossible_timeout = |_| -> Infallible { panic!("unexpected timeout") };
            let result = poll_completion_state(
                || state,
                || false,
                || panic!("terminal state must not wait"),
                &mut observer,
                &mut impossible_timeout,
            );
            if state == CommandState::Error {
                assert_eq!(
                    result.expect("error terminal classification"),
                    CompletionPollExit::Error
                );
            } else {
                assert!(
                    matches!(result, Err(MetalError::UnexpectedCommandState(value)) if value == state)
                );
            }
            assert_eq!(observed, [CommandEvent::Terminal(terminal)]);
        }
    }

    #[test]
    fn arithmetic_host_renderer_covers_defined_and_undefined() {
        let mut add = [0u32; 20];
        add[0] = 1;
        add[2] = 1;
        add[4] = 4;
        add[12] = 9;
        assert_eq!(render_arithmetic(&add)[5], 13);

        add[4..12].fill(u32::MAX);
        add[12] = 1;
        assert_eq!(render_arithmetic(&add)[0], ARITHMETIC_CHECKED_UNDEFINED);
    }

    #[test]
    fn grade_one_opening_renderer_owns_exact_ten_slots() {
        let task = OpeningTaskWords::try_from_words([1, 7, 1, 0x7, 0, 3, 6, 60])
            .expect("valid grade-one task");
        let rendered = render_opening_response(&task, &OpeningChooseTableWords::canonical(), 0)
            .expect("host rendering");
        assert_eq!(rendered.len(), 10);
        assert!(rendered.iter().all(|slot| slot[0] <= OPENING_VALID));
    }

    #[test]
    fn defensive_projector_bounds_accept_boundary_and_reject_successor() {
        validate_opening_cell_bounds(
            41,
            OPENING_MAX_CELL_SUPPORT_V1,
            OPENING_MAX_CELL_COEFFICIENT_V1,
            OPENING_MAX_CELL_MASS_V1,
        )
        .expect("exact defensive cell maxima are admitted");
        require_opening_bound(
            42,
            OPENING_MAX_WHOLE_MASS_V1,
            OPENING_MAX_WHOLE_MASS_V1,
            "opening whole mass exceeds frozen maximum",
        )
        .expect("exact defensive whole-mass maximum is admitted");

        for result in [
            validate_opening_cell_bounds(
                41,
                OPENING_MAX_CELL_SUPPORT_V1 + 1,
                OPENING_MAX_CELL_COEFFICIENT_V1,
                OPENING_MAX_CELL_MASS_V1,
            ),
            validate_opening_cell_bounds(
                41,
                OPENING_MAX_CELL_SUPPORT_V1,
                OPENING_MAX_CELL_COEFFICIENT_V1 + 1,
                OPENING_MAX_CELL_MASS_V1,
            ),
            validate_opening_cell_bounds(
                41,
                OPENING_MAX_CELL_SUPPORT_V1,
                OPENING_MAX_CELL_COEFFICIENT_V1,
                OPENING_MAX_CELL_MASS_V1 + 1,
            ),
            require_opening_bound(
                42,
                OPENING_MAX_WHOLE_MASS_V1 + 1,
                OPENING_MAX_WHOLE_MASS_V1,
                "opening whole mass exceeds frozen maximum",
            ),
        ] {
            assert!(matches!(
                result,
                Err(MetalError::InvalidOutput {
                    record: 41 | 42,
                    ..
                })
            ));
        }
    }

    #[test]
    fn launch_contracts_freeze_grid_threadgroup_and_buffer_extents() {
        let arithmetic = arithmetic_launch_contract(ArithmeticLaunch::Official, 32)
            .expect("official arithmetic launch");
        assert_eq!(arithmetic.grid, 16_384);
        assert_eq!(arithmetic.input_words, 16_384 * 20);
        assert_eq!(arithmetic.output_words, (16_384 + 2) * 16);

        let arithmetic_negative = arithmetic_launch_contract(ArithmeticLaunch::NegativeControl, 32)
            .expect("negative arithmetic launch");
        assert_eq!(arithmetic_negative.grid, 13);
        assert_eq!(arithmetic_negative.input_words, 13 * 20);
        assert_eq!(arithmetic_negative.output_words, 15 * 16);

        let opening = opening_launch_contract(
            OpeningLaunch::ValidatedTask {
                response_count: 7_980,
            },
            32,
        )
        .expect("maximum opening launch");
        assert_eq!(opening.grid, 7_980);
        assert_eq!(opening.input_words, 8);
        assert_eq!(opening.auxiliary_words, 484);
        assert_eq!(opening.output_words, OPENING_ARENA_RECORDS * 16);

        let opening_negative = opening_launch_contract(OpeningLaunch::NegativeControl, 32)
            .expect("negative opening launch");
        assert_eq!(opening_negative.grid, 1);
        assert_eq!(opening_negative.output_words, 12 * 16);
        assert!(matches!(
            opening_launch_contract(OpeningLaunch::ValidatedTask { response_count: 7 }, 32,),
            Err(MetalError::InvalidOpeningTask(_))
        ));
        assert!(matches!(
            arithmetic_launch_contract(ArithmeticLaunch::Official, 31),
            Err(MetalError::PipelineThreadLimit {
                actual: 31,
                required: 32,
                ..
            })
        ));
    }

    #[test]
    fn frozen_arena_word_counts_are_exact() {
        assert_eq!(crate::abi::CHOOSE_WORDS, 484);
        assert_eq!(crate::abi::OPENING_SLOT_CAP * 16, 1_276_800);
        assert_eq!(OPENING_ARENA_RECORDS * 16, 1_276_832);
    }
}
