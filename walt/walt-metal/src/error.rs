use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandState {
    NotEnqueued,
    Enqueued,
    Committed,
    Scheduled,
    Completed,
    Error,
    Unknown(usize),
}

#[derive(Debug)]
pub enum MetalError {
    NoDevice,
    NoCommandQueue,
    NoCommandBuffer,
    Gate0Required,
    Gate0AlreadyPassed,
    RuntimePoisoned,
    PortableValidation(String),
    LibraryLoad(String),
    MissingFunction(&'static str),
    PipelineCreation(&'static str, String),
    PipelineThreadLimit {
        kernel: &'static str,
        actual: usize,
        required: usize,
    },
    LengthOverflow(&'static str),
    AllocationFailure {
        purpose: &'static str,
        bytes: usize,
    },
    BufferTooShort {
        purpose: &'static str,
        required: usize,
        reported: usize,
    },
    BufferMisaligned(&'static str),
    EncoderCreation,
    WrongRecordCount {
        purpose: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidBinding {
        index: usize,
        reason: &'static str,
    },
    InvalidArithmeticInput {
        index: usize,
        reason: &'static str,
    },
    InvalidOpeningTask(&'static str),
    InvalidNegativeOrdinal {
        domain: &'static str,
        ordinal: usize,
        count: usize,
    },
    InvalidChooseEntry {
        n: usize,
        k: usize,
        expected: u32,
        actual: u32,
    },
    UnexpectedCommandState(CommandState),
    CommandError(String),
    InvalidOutput {
        record: usize,
        reason: &'static str,
    },
    Timeout(CommandState),
}

impl fmt::Display for MetalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDevice => formatter.write_str("Metal returned no default device"),
            Self::NoCommandQueue => formatter.write_str("Metal returned no command queue"),
            Self::NoCommandBuffer => formatter.write_str("Metal returned no command buffer"),
            Self::Gate0Required => {
                formatter.write_str("Metal Gate 0 must pass before any evidence command")
            }
            Self::Gate0AlreadyPassed => {
                formatter.write_str("Metal Gate 0 may run only once per fresh runtime")
            }
            Self::RuntimePoisoned => formatter.write_str(
                "Metal runtime cannot be reused after a failed or incomplete command sequence",
            ),
            Self::PortableValidation(error) => {
                write!(formatter, "portable M2 validation failed: {error}")
            }
            Self::LibraryLoad(error) => write!(formatter, "failed to load metallib: {error}"),
            Self::MissingFunction(name) => write!(formatter, "metallib is missing {name}"),
            Self::PipelineCreation(name, error) => {
                write!(formatter, "failed to create {name} pipeline: {error}")
            }
            Self::PipelineThreadLimit {
                kernel,
                actual,
                required,
            } => write!(
                formatter,
                "{kernel} permits {actual} threads per group, below required {required}"
            ),
            Self::LengthOverflow(purpose) => write!(formatter, "{purpose} length overflow"),
            Self::AllocationFailure { purpose, bytes } => {
                write!(formatter, "failed to allocate {bytes} bytes for {purpose}")
            }
            Self::BufferTooShort {
                purpose,
                required,
                reported,
            } => write!(
                formatter,
                "{purpose} buffer reports {reported} bytes, below required {required}"
            ),
            Self::BufferMisaligned(purpose) => {
                write!(formatter, "{purpose} buffer is not four-byte aligned")
            }
            Self::EncoderCreation => formatter.write_str("failed to create compute encoder"),
            Self::WrongRecordCount {
                purpose,
                expected,
                actual,
            } => write!(
                formatter,
                "{purpose} expected {expected} records, found {actual}"
            ),
            Self::InvalidBinding { index, reason } => {
                write!(formatter, "invalid buffer binding {index}: {reason}")
            }
            Self::InvalidArithmeticInput { index, reason } => {
                write!(formatter, "invalid arithmetic input {index}: {reason}")
            }
            Self::InvalidOpeningTask(reason) => write!(formatter, "invalid opening task: {reason}"),
            Self::InvalidNegativeOrdinal {
                domain,
                ordinal,
                count,
            } => write!(
                formatter,
                "{domain} negative-control ordinal {ordinal} is outside 0..{count}"
            ),
            Self::InvalidChooseEntry {
                n,
                k,
                expected,
                actual,
            } => write!(
                formatter,
                "choose({n},{k}) expected {expected}, found {actual}"
            ),
            Self::UnexpectedCommandState(state) => {
                write!(formatter, "unexpected Metal command state {state:?}")
            }
            Self::CommandError(error) => write!(formatter, "Metal command failed: {error}"),
            Self::InvalidOutput { record, reason } => {
                write!(formatter, "invalid GPU output record {record}: {reason}")
            }
            Self::Timeout(state) => write!(formatter, "Metal command timed out in {state:?}"),
        }
    }
}

impl std::error::Error for MetalError {}
