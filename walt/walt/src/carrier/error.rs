use core::fmt;

use crate::carrier::Digest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CarrierError {
    ReceiptLength { expected: usize, actual: usize },
    ReceiptDigest { actual: Digest },
    ReceiptUtf8,
    ReceiptParse(String),
    MissingHand(usize),
    Replay(String),
    FrozenFact(&'static str),
    ParentCount { expected: usize, actual: usize },
    SupportCount { expected: usize, actual: usize },
    SupportPayloadLength { expected: usize, actual: usize },
    SupportMismatch { ordinal: usize },
    InvalidSupport { ordinal: usize, field: &'static str },
    Kat(&'static str),
    LengthOverflow(&'static str),
}

impl fmt::Display for CarrierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReceiptLength { expected, actual } => {
                write!(
                    formatter,
                    "raw receipt has {actual} bytes, expected {expected}"
                )
            }
            Self::ReceiptDigest { actual } => {
                write!(formatter, "raw receipt digest mismatch: {}", hex(actual))
            }
            Self::ReceiptUtf8 => formatter.write_str("raw receipt is not UTF-8"),
            Self::ReceiptParse(error) => write!(formatter, "raw receipt parse failed: {error}"),
            Self::MissingHand(hand) => write!(formatter, "raw receipt has no hand {hand}"),
            Self::Replay(error) => write!(formatter, "hand replay failed: {error}"),
            Self::FrozenFact(field) => write!(formatter, "frozen carrier fact mismatch: {field}"),
            Self::ParentCount { expected, actual } => write!(
                formatter,
                "void-free parent enumeration has {actual} worlds, expected {expected}"
            ),
            Self::SupportCount { expected, actual } => {
                write!(
                    formatter,
                    "support has {actual} worlds, expected {expected}"
                )
            }
            Self::SupportPayloadLength { expected, actual } => write!(
                formatter,
                "support payload has {actual} bytes, expected {expected}"
            ),
            Self::SupportMismatch { ordinal } => write!(
                formatter,
                "primary and constrained support differ at ordinal {ordinal}"
            ),
            Self::InvalidSupport { ordinal, field } => {
                write!(formatter, "support ordinal {ordinal} violates {field}")
            }
            Self::Kat(field) => write!(formatter, "root-alias KAT failed at {field}"),
            Self::LengthOverflow(field) => write!(formatter, "length overflow at {field}"),
        }
    }
}

impl std::error::Error for CarrierError {}

fn hex(bytes: &Digest) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(char::from(DIGITS[usize::from(byte >> 4)]));
        output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    output
}
