//! Closed host-side errors for M3 ABI, launch, and allocation admission.

use core::fmt;

pub type Result<T> = core::result::Result<T, M3MetalError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum M3MetalError {
    WrongWordCount {
        record: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidWord {
        record: &'static str,
        word: usize,
        reason: &'static str,
    },
    InvalidRecord {
        record: &'static str,
        reason: &'static str,
    },
    LengthOverflow(&'static str),
    EmptyLaunch(&'static str),
    CapExceeded {
        cap: &'static str,
        limit: u64,
        observed: u64,
    },
    DeviceCapacity {
        field: &'static str,
        required: u64,
        observed: u64,
    },
    AllocationTooShort {
        buffer: &'static str,
        required: u64,
        reported: u64,
    },
    NonCanonicalPlan {
        pair: usize,
        reason: &'static str,
    },
    NonCanonicalRange {
        range: usize,
        reason: &'static str,
    },
}

impl fmt::Display for M3MetalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongWordCount {
                record,
                expected,
                actual,
            } => write!(
                formatter,
                "{record} requires {expected} words, received {actual}"
            ),
            Self::InvalidWord {
                record,
                word,
                reason,
            } => write!(formatter, "{record} word {word} is invalid: {reason}"),
            Self::InvalidRecord { record, reason } => {
                write!(formatter, "{record} is invalid: {reason}")
            }
            Self::LengthOverflow(label) => write!(formatter, "M3 length overflow: {label}"),
            Self::EmptyLaunch(label) => write!(formatter, "empty M3 {label} launch"),
            Self::CapExceeded {
                cap,
                limit,
                observed,
            } => write!(
                formatter,
                "M3 {cap} cap exceeded: limit {limit}, observed {observed}"
            ),
            Self::DeviceCapacity {
                field,
                required,
                observed,
            } => write!(
                formatter,
                "M3 device {field} is {observed}, requires at least {required}"
            ),
            Self::AllocationTooShort {
                buffer,
                required,
                reported,
            } => write!(
                formatter,
                "M3 {buffer} allocation reports {reported} bytes, requires {required}"
            ),
            Self::NonCanonicalPlan { pair, reason } => {
                write!(
                    formatter,
                    "M3 reduction pair {pair} is noncanonical: {reason}"
                )
            }
            Self::NonCanonicalRange { range, reason } => {
                write!(
                    formatter,
                    "M3 reduction range {range} is noncanonical: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for M3MetalError {}
