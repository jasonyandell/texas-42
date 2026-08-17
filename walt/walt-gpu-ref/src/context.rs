use core::fmt;

use walt_core::{legal_plays, Context, Decl, Domino, DominoSet};
use walt_kernel::KernelError;

pub const MAX_GRADE: u8 = 7;
/// A lawful opening pool omits the led tile from its effective context, whose
/// complete physical incidence has size seven.
pub const MAX_OPENING_MATCHING_COUNT_V1: usize = 6;
/// Maximum complete `Kernel::worlds` items admitted by the M1 direct backend.
///
/// One work unit is one complete world in the kernel's frozen 7/23 traversal
/// order.  The count is checked before the iterator is constructed.
pub const M1_DIRECT_WORLD_CAP_V1: u64 = 100_000;

/// A validated opening projection input.
///
/// The three hidden seats have the same capacity `grade`, and `pool` has
/// exactly `3 * grade` tiles.  Fields are private so an unchecked projection
/// cannot cross the public boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpeningContext {
    decl: Decl,
    led: Context,
    pool: DominoSet,
    grade: u8,
}

impl OpeningContext {
    /// Constructs a bounded reduced-grade input for scalar comparison work.
    pub fn try_reduced(
        decl: Decl,
        led: Context,
        pool: DominoSet,
        grade: u8,
    ) -> Result<OpeningContext, OpeningError> {
        if !(1..=MAX_GRADE).contains(&grade) {
            return Err(OpeningError::GradeOutOfRange { grade });
        }
        if !Domino::ALL
            .into_iter()
            .any(|lead| decl.led_context(lead) == led)
        {
            return Err(OpeningError::ImpossibleLedContext { decl, led });
        }
        let expected = usize::from(grade)
            .checked_mul(3)
            .ok_or(OpeningError::ArithmeticOverflow("hidden pool size"))?;
        let actual = pool.len();
        if actual != expected {
            return Err(OpeningError::PoolSizeMismatch {
                grade,
                expected,
                actual,
            });
        }
        let matching_count = pool.intersection(decl.effective_incidence(led)).len();
        if matching_count > MAX_OPENING_MATCHING_COUNT_V1 {
            return Err(OpeningError::OpeningMatchingCountOutOfRange {
                actual: matching_count,
                max: MAX_OPENING_MATCHING_COUNT_V1,
            });
        }
        Ok(OpeningContext {
            decl,
            led,
            pool,
            grade,
        })
    }

    /// Constructs a production opening context from one legal focal hand.
    ///
    /// The 21-tile pool is derived rather than accepted from the caller.  The
    /// requested led context must be represented by at least one legal lead
    /// in the seven-tile hand.
    pub fn from_opening_hand(
        decl: Decl,
        focal_hand: DominoSet,
        led: Context,
    ) -> Result<OpeningContext, OpeningError> {
        if focal_hand.len() != usize::from(MAX_GRADE) {
            return Err(OpeningError::OpeningHandSize {
                actual: focal_hand.len(),
            });
        }
        let legal = legal_plays(decl, focal_hand, None);
        if !legal.iter().any(|lead| decl.led_context(lead) == led) {
            return Err(OpeningError::LedContextNotRepresented { decl, led });
        }
        let pool = DominoSet::FULL.difference(focal_hand);
        OpeningContext::try_reduced(decl, led, pool, MAX_GRADE)
    }

    pub const fn decl(self) -> Decl {
        self.decl
    }

    pub const fn led(self) -> Context {
        self.led
    }

    pub const fn pool(self) -> DominoSet {
        self.pool
    }

    pub const fn grade(self) -> u8 {
        self.grade
    }

    pub fn matching_pool(self) -> DominoSet {
        self.pool
            .intersection(self.decl.effective_incidence(self.led))
    }

    pub fn physical_world_count(self) -> Result<u64, OpeningError> {
        let pool = self.pool.len();
        let grade = usize::from(self.grade);
        let first = crate::projection::checked_binomial(pool, grade)?;
        let second = crate::projection::checked_binomial(pool - grade, grade)?;
        first
            .checked_mul(second)
            .ok_or(OpeningError::ArithmeticOverflow("physical world count"))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpeningError {
    GradeOutOfRange {
        grade: u8,
    },
    PoolSizeMismatch {
        grade: u8,
        expected: usize,
        actual: usize,
    },
    OpeningMatchingCountOutOfRange {
        actual: usize,
        max: usize,
    },
    OpeningHandSize {
        actual: usize,
    },
    OpeningHandNotFullyLegal,
    PointBidOutOfRange {
        value: u8,
    },
    LossBudgetOutOfRange {
        value: u8,
    },
    ImpossibleLedContext {
        decl: Decl,
        led: Context,
    },
    LedContextNotRepresented {
        decl: Decl,
        led: Context,
    },
    DirectWorldCapExceeded {
        world_count: u64,
        cap: u64,
    },
    KernelWorldCountMismatch {
        formula_count: u64,
        kernel_count: u64,
    },
    MassConservationMismatch {
        expected: u64,
        actual: u64,
    },
    ScaleDoesNotClear {
        denominator: u32,
    },
    ArithmeticOverflow(&'static str),
    SupportTooWide {
        value: u64,
    },
    DuplicateCell,
    OpeningCellCapExceeded {
        cell_count: usize,
        cap: usize,
    },
    InconsistentDirectAggregate,
    CarrierGenerationMismatch,
    Kernel(KernelError),
}

impl fmt::Display for OpeningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpeningError::GradeOutOfRange { grade } => {
                write!(f, "grade {grade} is outside 1..={MAX_GRADE}")
            }
            OpeningError::PoolSizeMismatch {
                grade,
                expected,
                actual,
            } => write!(
                f,
                "grade {grade} requires a {expected}-tile pool, found {actual}"
            ),
            OpeningError::OpeningMatchingCountOutOfRange { actual, max } => write!(
                f,
                "an opening projection may contain at most {max} hidden matching tiles, found {actual}"
            ),
            OpeningError::OpeningHandSize { actual } => {
                write!(
                    f,
                    "an opening focal hand must hold seven tiles, found {actual}"
                )
            }
            OpeningError::OpeningHandNotFullyLegal => {
                f.write_str("the focal opening hand contains a non-legal lead tile")
            }
            OpeningError::PointBidOutOfRange { value } => {
                write!(f, "point bid {value} is outside 30..=41")
            }
            OpeningError::LossBudgetOutOfRange { value } => {
                write!(f, "derived loss budget {value} is outside 0..=12")
            }
            OpeningError::ImpossibleLedContext { decl, led } => {
                write!(f, "no tile can lead {led} under {decl}")
            }
            OpeningError::LedContextNotRepresented { decl, led } => {
                write!(
                    f,
                    "{led} under {decl} is not represented by a legal focal lead"
                )
            }
            OpeningError::DirectWorldCapExceeded { world_count, cap } => write!(
                f,
                "direct enumeration declared stop: {world_count} worlds exceed cap {cap}"
            ),
            OpeningError::KernelWorldCountMismatch {
                formula_count,
                kernel_count,
            } => write!(
                f,
                "direct preflight formula count {formula_count} disagrees with kernel count {kernel_count}"
            ),
            OpeningError::MassConservationMismatch { expected, actual } => write!(
                f,
                "opening projection mass {actual} disagrees with expected mass {expected}"
            ),
            OpeningError::ScaleDoesNotClear { denominator } => write!(
                f,
                "field scale does not clear legal-set denominator {denominator}"
            ),
            OpeningError::ArithmeticOverflow(operation) => {
                write!(f, "exact arithmetic overflow while computing {operation}")
            }
            OpeningError::SupportTooWide { value } => {
                write!(
                    f,
                    "opening support {value} does not fit its fixed-width type"
                )
            }
            OpeningError::DuplicateCell => {
                f.write_str("opening projection emitted a duplicate cell")
            }
            OpeningError::OpeningCellCapExceeded { cell_count, cap } => write!(
                f,
                "opening projection emitted {cell_count} cells, exceeding hard cap {cap}"
            ),
            OpeningError::InconsistentDirectAggregate => {
                f.write_str("direct response paths disagree inside one canonical cell")
            }
            OpeningError::CarrierGenerationMismatch => {
                f.write_str("reduced opening carrier failed its generated-coordinate check")
            }
            OpeningError::Kernel(error) => write!(f, "kernel rejected opening input: {error}"),
        }
    }
}

impl std::error::Error for OpeningError {}

impl From<KernelError> for OpeningError {
    fn from(value: KernelError) -> Self {
        OpeningError::Kernel(value)
    }
}
