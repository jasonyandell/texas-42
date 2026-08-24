use core::fmt;

pub use walt::spec::sha256;

pub type Digest = [u8; 32];

pub const ZERO_DIGEST: Digest = [0; 32];
pub const FORMAT_VERSION: u16 = 1;
pub const STREAM_VERSION: u32 = 1;
pub const POISON_WORD: u32 = 0xa5a5_5a5a;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CodecError {
    Truncated { at: usize, need: usize, have: usize },
    TrailingBytes { at: usize, total: usize },
    LengthOverflow(&'static str),
    InvalidUtf8,
    Invalid(&'static str),
    DigestMismatch(&'static str),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated { at, need, have } => {
                write!(f, "truncated at {at}: need {need} bytes, have {have}")
            }
            Self::TrailingBytes { at, total } => {
                write!(f, "trailing bytes at {at} of {total}")
            }
            Self::LengthOverflow(label) => write!(f, "length does not fit {label}"),
            Self::InvalidUtf8 => f.write_str("invalid UTF-8"),
            Self::Invalid(label) => write!(f, "invalid {label}"),
            Self::DigestMismatch(label) => write!(f, "digest mismatch for {label}"),
        }
    }
}

impl std::error::Error for CodecError {}

pub type Result<T> = core::result::Result<T, CodecError>;

macro_rules! numeric_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident : $repr:ty { $($variant:ident = $value:expr),+ $(,)? }) => {
        $(#[$meta])*
        #[repr($repr)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        $vis enum $name { $($variant = $value),+ }

        impl TryFrom<$repr> for $name {
            type Error = CodecError;

            fn try_from(value: $repr) -> Result<Self> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    _ => Err(CodecError::Invalid(stringify!($name))),
                }
            }
        }

        impl From<$name> for $repr {
            fn from(value: $name) -> Self { value as $repr }
        }
    };
}

numeric_enum! {
    pub enum StreamPurpose: u32 {
        TaskKeys = 1,
        ArithmeticInput = 2,
        ArithmeticOutput = 3,
        ContextSlotStream = 4,
        ContextPayload = 5,
        ContextResponseAggregates = 6,
        TaskInputHashChain = 7,
        ChooseInputHashChain = 8,
        ProtectedRecords = 9,
        ReducedIdentities = 10,
        PhysicalIdentities = 11,
        GlobalSlotStream = 12,
        GlobalPayloadStream = 13,
        GlobalResponseAggregates = 14,
        GlobalProtectedChain = 15
    }
}

numeric_enum! {
    pub enum SectionTag: u16 {
        Authority = 1,
        Toolchain = 2,
        Device = 3,
        TablesAndAbi = 4,
        Arithmetic = 5,
        Carrier = 6,
        ContextTasks = 7,
        ReducedBindings = 8,
        PhysicalBindings = 9,
        Global = 10
    }
}

numeric_enum! {
    pub enum ArtifactTag: u32 {
        ParentSourceManifest = 1,
        Freeze55Descriptor = 2,
        ReceivedGuide = 3,
        M0M1Contract = 4,
        OpeningEnvelope = 5,
        Grade5Stop = 6,
        M0M1Summary = 7,
        HistoricalGate0 = 8,
        M2SourceManifest = 9,
        M2Contract = 10,
        CargoLock = 11,
        Freeze56Descriptor = 12,
        ParentCensus = 13
    }
}

numeric_enum! {
    pub enum ToolId: u32 {
        Metal = 1,
        Metallib = 2,
        MetalAr = 3,
        Xctrace = 4,
        Xcodebuild = 5
    }
}

numeric_enum! {
    pub enum SourceKind: u32 {
        TranslationUnit = 1,
        Include = 2
    }
}

numeric_enum! {
    pub enum InvocationKind: u32 {
        Compile = 1,
        Link = 2
    }
}

numeric_enum! {
    pub enum KernelId: u32 {
        Arithmetic = 1,
        Projector = 2
    }
}

numeric_enum! {
    pub enum ArithmeticRunKind: u32 {
        Official = 1,
        Negative = 2
    }
}

numeric_enum! {
    pub enum Arm: u32 {
        Reduced = 1,
        GradeMatching = 2,
        SameContextPair = 3
    }
}

numeric_enum! {
    pub enum DirectStatus: u32 {
        Parity = 1,
        DeclaredStop = 2
    }
}

numeric_enum! {
    pub enum FrameKind: u16 {
        Preparing = 1,
        Committed = 2,
        Terminal = 3,
        Finalizing = 4,
        Success = 5,
        Failure = 6
    }
}

numeric_enum! {
    pub enum TerminalCode: u32 {
        Completed = 1,
        Error = 2,
        Timeout = 3,
        NotEnqueued = 4,
        NotCommitted = 5,
        Scheduled = 6,
        Unknown = 7
    }
}

numeric_enum! {
    pub enum FailurePhase: u32 {
        Historical = 1,
        SourceManifest = 2,
        RustBuild = 3,
        MetalToolchain = 4,
        ShaderReproducibility = 5,
        Gate0 = 6,
        Tables = 7,
        ArithmeticNegative = 8,
        ArithmeticCorpus = 9,
        CarrierPreflight = 10,
        OpeningNegative = 11,
        ProjectorTask = 12,
        Bindings = 13,
        ReceiptRender = 14,
        ChildProtocol = 15,
        ReceiptRegeneration = 16
    }
}

numeric_enum! {
    pub enum FailureCode: u32 {
        InvalidAuthority = 1,
        IdentityMismatch = 2,
        ToolchainMismatch = 3,
        MetallibMismatch = 4,
        NoDevice = 5,
        AllocationFailure = 6,
        PipelineFailure = 7,
        EncoderFailure = 8,
        CommandStateFailure = 9,
        CommandError = 10,
        Timeout = 11,
        MalformedOutput = 12,
        PoisonFailure = 13,
        GuardFailure = 14,
        InputMutation = 15,
        ArithmeticMismatch = 16,
        ProjectorMismatch = 17,
        MassMismatch = 18,
        CarrierMismatch = 19,
        BindingMismatch = 20,
        ChildProtocolFailure = 21,
        ReceiptNondeterministic = 22,
        ReceiptComparandMismatch = 23,
        InternalFailure = 24
    }
}

pub fn stream_digest(purpose: StreamPurpose, record_count: u64, payload: &[u8]) -> Result<Digest> {
    let payload_bytes = u64::try_from(payload.len())
        .map_err(|_| CodecError::LengthOverflow("u64 stream payload"))?;
    let mut preimage = Vec::with_capacity(8 + 4 + 4 + 8 + 8 + payload.len());
    preimage.extend_from_slice(b"W42M2DG1");
    put_u32(&mut preimage, purpose.into());
    put_u32(&mut preimage, STREAM_VERSION);
    put_u64(&mut preimage, record_count);
    put_u64(&mut preimage, payload_bytes);
    preimage.extend_from_slice(payload);
    Ok(sha256(&preimage))
}

pub fn section_digest(tag: SectionTag, record_count: u64, section_bytes: &[u8]) -> Result<Digest> {
    let section_bytes_len = u64::try_from(section_bytes.len())
        .map_err(|_| CodecError::LengthOverflow("u64 section"))?;
    let mut preimage = Vec::with_capacity(8 + 4 + 4 + 8 + 8 + section_bytes.len());
    preimage.extend_from_slice(b"W42M2SC1");
    put_u32(&mut preimage, u32::from(tag as u16));
    put_u32(&mut preimage, 1);
    put_u64(&mut preimage, record_count);
    put_u64(&mut preimage, section_bytes_len);
    preimage.extend_from_slice(section_bytes);
    Ok(sha256(&preimage))
}

pub(crate) fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn put_i32(out: &mut Vec<u8>, value: i32) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

pub(crate) fn put_digest(out: &mut Vec<u8>, value: &Digest) {
    out.extend_from_slice(value);
}

pub(crate) fn put_text(out: &mut Vec<u8>, text: &str) -> Result<()> {
    validate_text(text)?;
    let len = u32::try_from(text.len()).map_err(|_| CodecError::LengthOverflow("TextV1 u32"))?;
    put_u32(out, len);
    out.extend_from_slice(text.as_bytes());
    Ok(())
}

pub(crate) fn validate_text(text: &str) -> Result<()> {
    if text
        .as_bytes()
        .iter()
        .any(|byte| matches!(byte, 0 | b'\r' | b'\n'))
    {
        return Err(CodecError::Invalid("TextV1 forbidden byte"));
    }
    Ok(())
}

pub(crate) fn validate_path(path: &str) -> Result<()> {
    validate_text(path)?;
    if path.is_empty() || path.starts_with('/') || path.contains('\\') {
        return Err(CodecError::Invalid("canonical path"));
    }
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(CodecError::Invalid("canonical path component"));
    }
    Ok(())
}

pub(crate) struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.at)
    }

    pub(crate) fn finish(self) -> Result<()> {
        if self.at == self.bytes.len() {
            Ok(())
        } else {
            Err(CodecError::TrailingBytes {
                at: self.at,
                total: self.bytes.len(),
            })
        }
    }

    pub(crate) fn bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .at
            .checked_add(len)
            .ok_or(CodecError::LengthOverflow("reader offset"))?;
        if end > self.bytes.len() {
            return Err(CodecError::Truncated {
                at: self.at,
                need: len,
                have: self.remaining(),
            });
        }
        let value = &self.bytes[self.at..end];
        self.at = end;
        Ok(value)
    }

    pub(crate) fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.bytes(N)?
            .try_into()
            .map_err(|_| CodecError::Invalid("fixed byte array"))
    }

    pub(crate) fn u8(&mut self) -> Result<u8> {
        Ok(self.bytes(1)?[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    pub(crate) fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    pub(crate) fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.array()?))
    }

    pub(crate) fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    pub(crate) fn digest(&mut self) -> Result<Digest> {
        self.array()
    }

    pub(crate) fn text(&mut self) -> Result<String> {
        let len =
            usize::try_from(self.u32()?).map_err(|_| CodecError::LengthOverflow("usize TextV1"))?;
        let bytes = self.bytes(len)?;
        let text = core::str::from_utf8(bytes).map_err(|_| CodecError::InvalidUtf8)?;
        validate_text(text)?;
        Ok(text.to_owned())
    }
}

pub(crate) fn require_zero(bytes: &[u8], label: &'static str) -> Result<()> {
    if bytes.iter().all(|byte| *byte == 0) {
        Ok(())
    } else {
        Err(CodecError::Invalid(label))
    }
}

pub(crate) fn require_eq<T: PartialEq + ?Sized>(
    actual: &T,
    expected: &T,
    label: &'static str,
) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(CodecError::Invalid(label))
    }
}

pub(crate) fn checked_usize(value: u64, label: &'static str) -> Result<usize> {
    usize::try_from(value).map_err(|_| CodecError::LengthOverflow(label))
}
