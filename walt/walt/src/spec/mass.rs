//! Exact mass types shared by the CPU reference and the later Metal ABI.
//!
//! `U256Mass` is eight base-2^32 limbs in least-significant-limb-first order.
//! Its canonical byte form is likewise little-endian: the little-endian bytes
//! of limb zero, then limb one, through limb seven.  The in-memory `repr(C)`
//! form is useful for a matched Rust/MSL buffer layout; receipts must use the
//! explicit byte encoder rather than raw structure bytes.

use core::cmp::Ordering;
use core::fmt;

/// The common-denominator multiplier for the current uniform-legal field.
pub const FIELD_SCALE: u32 = 420;
/// Fixed limb count of one exact full-hand mass.
pub const U256_LIMBS: usize = 8;
/// Canonical byte width of one exact full-hand mass.
pub const U256_BYTES: usize = U256_LIMBS * core::mem::size_of::<u32>();
/// Canonical byte width of one scale frame.
pub const SCALE_FRAME_BYTES: usize = 12;
/// The three field plays after the focal opening lead.
pub const OPENING_RESPONSE_FIELD_EXPONENT: u8 = 3;
/// The current trick-1 profile's complete field horizon after the opening lead.
pub const TRICK1_FULL_HORIZON_EXPONENT: u8 = 21;

/// An unsigned 256-bit exact mass.
///
/// The tuple field is intentionally public so a future MSL bridge can copy a
/// fixed-width array without a second representation.  Arithmetic always goes
/// through the checked methods below.
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct U256Mass(pub [u32; U256_LIMBS]);

const _: [(); 32] = [(); core::mem::size_of::<U256Mass>()];
const _: [(); 4] = [(); core::mem::align_of::<U256Mass>()];

impl U256Mass {
    pub const ZERO: U256Mass = U256Mass([0; U256_LIMBS]);
    pub const MAX: U256Mass = U256Mass([u32::MAX; U256_LIMBS]);

    /// Places `value` in the two least-significant limbs.
    pub const fn from_u64(value: u64) -> U256Mass {
        U256Mass([value as u32, (value >> 32) as u32, 0, 0, 0, 0, 0, 0])
    }

    /// Returns the canonical least-significant-limb-first array.
    pub const fn limbs_le(self) -> [u32; U256_LIMBS] {
        self.0
    }

    /// Encodes the exact mass in the frozen canonical byte order.
    pub fn to_le_bytes(self) -> [u8; U256_BYTES] {
        let mut bytes = [0u8; U256_BYTES];
        for (index, limb) in self.0.iter().enumerate() {
            let offset = index * core::mem::size_of::<u32>();
            let limb_bytes = limb.to_le_bytes();
            bytes[offset] = limb_bytes[0];
            bytes[offset + 1] = limb_bytes[1];
            bytes[offset + 2] = limb_bytes[2];
            bytes[offset + 3] = limb_bytes[3];
        }
        bytes
    }

    /// Decodes the frozen canonical byte order.
    pub fn from_le_bytes(bytes: [u8; U256_BYTES]) -> U256Mass {
        let mut limbs = [0u32; U256_LIMBS];
        for (index, limb) in limbs.iter_mut().enumerate() {
            let offset = index * core::mem::size_of::<u32>();
            *limb = u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
        }
        U256Mass(limbs)
    }

    pub const fn is_zero(self) -> bool {
        let mut index = 0;
        while index < U256_LIMBS {
            if self.0[index] != 0 {
                return false;
            }
            index += 1;
        }
        true
    }

    /// Exact addition, returning `None` rather than wrapping on overflow.
    pub fn checked_add(self, rhs: U256Mass) -> Option<U256Mass> {
        let mut out = [0u32; U256_LIMBS];
        let mut carry = false;
        for (index, slot) in out.iter_mut().enumerate() {
            let (sum, carry_a) = self.0[index].overflowing_add(rhs.0[index]);
            let (sum, carry_b) = sum.overflowing_add(u32::from(carry));
            *slot = sum;
            carry = carry_a || carry_b;
        }
        (!carry).then_some(U256Mass(out))
    }

    /// Exact subtraction, returning `None` rather than wrapping below zero.
    pub fn checked_sub(self, rhs: U256Mass) -> Option<U256Mass> {
        let mut out = [0u32; U256_LIMBS];
        let mut borrow = false;
        for (index, slot) in out.iter_mut().enumerate() {
            let (difference, borrow_a) = self.0[index].overflowing_sub(rhs.0[index]);
            let (difference, borrow_b) = difference.overflowing_sub(u32::from(borrow));
            *slot = difference;
            borrow = borrow_a || borrow_b;
        }
        (!borrow).then_some(U256Mass(out))
    }

    /// Exact multiplication by one machine-word integer.
    ///
    /// The per-limb intermediate is bounded by `(2^32 - 1)^2 + (2^32 - 1)`,
    /// so it fits in `u64` without an unchecked intermediate wrap.
    pub fn checked_mul_small(self, factor: u32) -> Option<U256Mass> {
        let mut out = [0u32; U256_LIMBS];
        let mut carry = 0u64;
        for (index, slot) in out.iter_mut().enumerate() {
            let product = u64::from(self.0[index]) * u64::from(factor) + carry;
            *slot = product as u32;
            carry = product >> 32;
        }
        (carry == 0).then_some(U256Mass(out))
    }

    /// Applies the current field scale one field action at a time to a raw value.
    ///
    /// This deliberately avoids a general 256-by-256 multiplication path.  It
    /// has no scale-frame metadata to update; callers that hold an `ExactMass`
    /// must use `ExactMass::checked_advance_field_scale` instead.
    pub fn checked_mul_pow_420(self, exponent: u8) -> Option<U256Mass> {
        let mut out = self;
        for _ in 0..exponent {
            out = out.checked_mul_small(FIELD_SCALE)?;
        }
        Some(out)
    }
}

impl Ord for U256Mass {
    fn cmp(&self, other: &Self) -> Ordering {
        for index in (0..U256_LIMBS).rev() {
            match self.0[index].cmp(&other.0[index]) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for U256Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for U256Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("U256Mass(0x")?;
        for index in (0..U256_LIMBS).rev() {
            write!(f, "{:08x}", self.0[index])?;
        }
        f.write_str(")")
    }
}

/// Versioned prior-profile identity carried in a scale frame.
///
/// This is a fixed-width identifier, not a Rust enum, so its representation
/// is stable in a serialized GPU descriptor.  The v1 registry is closed:
/// unrecognized raw values cannot construct a profile identity.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PriorProfileId(u16);

impl PriorProfileId {
    pub const UNIFORM_OPENING_V1: PriorProfileId = PriorProfileId(1);

    /// Decodes a known v1 prior profile without admitting an arbitrary ID.
    pub const fn try_from_raw(raw: u16) -> Option<PriorProfileId> {
        match raw {
            1 => Some(PriorProfileId::UNIFORM_OPENING_V1),
            _ => None,
        }
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

/// Versioned field-profile identity carried in a scale frame.
///
/// This is a fixed-width identifier, not a Rust enum, so its representation
/// is stable in a serialized GPU descriptor.  The v1 registry is closed:
/// unrecognized raw values cannot construct a profile identity.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldProfileId(u16);

impl FieldProfileId {
    pub const UNIFORM_RANDOM_LEGAL_V1: FieldProfileId = FieldProfileId(1);

    /// Decodes a known v1 field profile without admitting an arbitrary ID.
    pub const fn try_from_raw(raw: u16) -> Option<FieldProfileId> {
        match raw {
            1 => Some(FieldProfileId::UNIFORM_RANDOM_LEGAL_V1),
            _ => None,
        }
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

/// Versioned utility-profile identity carried in a scale frame.
///
/// `NOT_APPLICABLE_V1` is used for likelihood-only posterior material such as
/// an opening-response mass.  A solved action value must instead carry the
/// specific contract utility it represents.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UtilityProfileId(u16);

impl UtilityProfileId {
    pub const NOT_APPLICABLE_V1: UtilityProfileId = UtilityProfileId(1);
    pub const DECLARING_TEAM_MAKES_V1: UtilityProfileId = UtilityProfileId(2);

    /// Decodes a known v1 utility profile without admitting an arbitrary ID.
    pub const fn try_from_raw(raw: u16) -> Option<UtilityProfileId> {
        match raw {
            1 => Some(UtilityProfileId::NOT_APPLICABLE_V1),
            2 => Some(UtilityProfileId::DECLARING_TEAM_MAKES_V1),
            _ => None,
        }
    }

    pub const fn raw(self) -> u16 {
        self.0
    }
}

/// The semantic role of a mass in a reduction.
///
/// This fixed-width identity makes it impossible for `ExactMass::checked_add`
/// to accidentally combine a posterior likelihood, a conditional value, and a
/// weighted contribution just because their integer scales agree.  Bound
/// polarity is deliberately not a measure role: a later `ExactBound` wrapper
/// carries lower/upper polarity so values in one weighted-contribution frame
/// remain comparable.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeasureRoleId(u8);

impl MeasureRoleId {
    pub const OPENING_RESPONSE_MASS_V1: MeasureRoleId = MeasureRoleId(1);
    pub const CONDITIONAL_VALUE_V1: MeasureRoleId = MeasureRoleId(2);
    pub const WEIGHTED_CONTRIBUTION_V1: MeasureRoleId = MeasureRoleId(3);

    /// Decodes a known v1 measure role without admitting an arbitrary ID.
    pub const fn try_from_raw(raw: u8) -> Option<MeasureRoleId> {
        match raw {
            1 => Some(MeasureRoleId::OPENING_RESPONSE_MASS_V1),
            2 => Some(MeasureRoleId::CONDITIONAL_VALUE_V1),
            3 => Some(MeasureRoleId::WEIGHTED_CONTRIBUTION_V1),
            _ => None,
        }
    }

    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The denominator provenance of an exact mass.
///
/// `field_exponent` names the number of `420` factors already absorbed, and
/// `full_horizon_exponent` names the target common denominator.  The frame
/// also carries prior, field, utility, and measure-role identities, all of
/// which must agree before two masses can be reduced together.
///
/// Task and focal-action identity intentionally remain outside this reusable
/// arithmetic primitive.  The controller must partition reductions by that
/// identity before calling `ExactMass::checked_add`, and bind it in the
/// higher-level receipt or reduction key.
///
/// The three zero bytes are reserved and are always initialized; they prevent
/// a future field from silently changing the fixed buffer layout.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScaleFrame {
    prior_profile: PriorProfileId,
    field_profile: FieldProfileId,
    utility_profile: UtilityProfileId,
    measure_role: MeasureRoleId,
    field_exponent: u8,
    full_horizon_exponent: u8,
    reserved: [u8; 3],
}

const _: [(); SCALE_FRAME_BYTES] = [(); core::mem::size_of::<ScaleFrame>()];
const _: [(); 2] = [(); core::mem::align_of::<ScaleFrame>()];

impl ScaleFrame {
    /// Creates a frame only when its current scale is within its declared
    /// full horizon.
    pub const fn new(
        prior_profile: PriorProfileId,
        field_profile: FieldProfileId,
        utility_profile: UtilityProfileId,
        measure_role: MeasureRoleId,
        field_exponent: u8,
        full_horizon_exponent: u8,
    ) -> Option<ScaleFrame> {
        if field_exponent > full_horizon_exponent {
            None
        } else {
            Some(ScaleFrame {
                prior_profile,
                field_profile,
                utility_profile,
                measure_role,
                field_exponent,
                full_horizon_exponent,
                reserved: [0; 3],
            })
        }
    }

    pub const fn prior_profile(self) -> PriorProfileId {
        self.prior_profile
    }

    pub const fn field_profile(self) -> FieldProfileId {
        self.field_profile
    }

    pub const fn utility_profile(self) -> UtilityProfileId {
        self.utility_profile
    }

    pub const fn measure_role(self) -> MeasureRoleId {
        self.measure_role
    }

    pub const fn field_exponent(self) -> u8 {
        self.field_exponent
    }

    pub const fn full_horizon_exponent(self) -> u8 {
        self.full_horizon_exponent
    }

    /// Number of virtual or unresolved field actions needed at the common
    /// full-horizon scale.  Construction preserves the subtraction invariant.
    pub const fn normalization_delta(self) -> u8 {
        self.full_horizon_exponent - self.field_exponent
    }

    /// Advances the frame through `delta` weighted field actions without
    /// exceeding the declared full horizon.
    pub const fn checked_advance_field_actions(self, delta: u8) -> Option<ScaleFrame> {
        match self.field_exponent.checked_add(delta) {
            Some(field_exponent) if field_exponent <= self.full_horizon_exponent => {
                Some(ScaleFrame {
                    prior_profile: self.prior_profile,
                    field_profile: self.field_profile,
                    utility_profile: self.utility_profile,
                    measure_role: self.measure_role,
                    field_exponent,
                    full_horizon_exponent: self.full_horizon_exponent,
                    reserved: [0; 3],
                })
            }
            _ => None,
        }
    }

    /// Encodes the frame independently of host struct padding and endianness.
    pub const fn to_le_bytes(self) -> [u8; SCALE_FRAME_BYTES] {
        let prior = self.prior_profile.raw().to_le_bytes();
        let field = self.field_profile.raw().to_le_bytes();
        let utility = self.utility_profile.raw().to_le_bytes();
        [
            prior[0],
            prior[1],
            field[0],
            field[1],
            utility[0],
            utility[1],
            self.measure_role.raw(),
            self.field_exponent,
            self.full_horizon_exponent,
            0,
            0,
            0,
        ]
    }

    /// Decodes the canonical byte order, rejecting unknown v1 IDs, nonzero
    /// reserved bytes, and an invalid exponent range.
    pub const fn from_le_bytes(bytes: [u8; SCALE_FRAME_BYTES]) -> Option<ScaleFrame> {
        if bytes[9] != 0 || bytes[10] != 0 || bytes[11] != 0 {
            None
        } else {
            match (
                PriorProfileId::try_from_raw(u16::from_le_bytes([bytes[0], bytes[1]])),
                FieldProfileId::try_from_raw(u16::from_le_bytes([bytes[2], bytes[3]])),
                UtilityProfileId::try_from_raw(u16::from_le_bytes([bytes[4], bytes[5]])),
                MeasureRoleId::try_from_raw(bytes[6]),
            ) {
                (
                    Some(prior_profile),
                    Some(field_profile),
                    Some(utility_profile),
                    Some(measure_role),
                ) => ScaleFrame::new(
                    prior_profile,
                    field_profile,
                    utility_profile,
                    measure_role,
                    bytes[7],
                    bytes[8],
                ),
                _ => None,
            }
        }
    }
}

/// A full-width exact mass with explicit denominator provenance.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactMass {
    value: U256Mass,
    frame: ScaleFrame,
}

const _: [(); 44] = [(); core::mem::size_of::<ExactMass>()];
const _: [(); 4] = [(); core::mem::align_of::<ExactMass>()];

impl ExactMass {
    pub const fn new(frame: ScaleFrame, value: U256Mass) -> ExactMass {
        ExactMass { value, frame }
    }

    pub const fn zero(frame: ScaleFrame) -> ExactMass {
        ExactMass::new(frame, U256Mass::ZERO)
    }

    pub const fn value(self) -> U256Mass {
        self.value
    }

    pub const fn frame(self) -> ScaleFrame {
        self.frame
    }

    /// Adds only masses with identical profile, role, and denominator frame.
    pub fn checked_add(self, rhs: ExactMass) -> Result<ExactMass, ExactMassError> {
        self.require_same_frame(rhs)?;
        let value = self
            .value
            .checked_add(rhs.value)
            .ok_or(ExactMassError::ArithmeticOverflow)?;
        Ok(ExactMass::new(self.frame, value))
    }

    /// Compares only masses with the same profile, role, and denominator frame.
    pub fn checked_cmp(self, rhs: ExactMass) -> Result<Ordering, ExactMassError> {
        self.require_same_frame(rhs)?;
        Ok(self.value.cmp(&rhs.value))
    }

    /// Subtracts only masses with the same profile, role, and denominator frame.
    pub fn checked_sub(self, rhs: ExactMass) -> Result<ExactMass, ExactMassError> {
        self.require_same_frame(rhs)?;
        let value = self
            .value
            .checked_sub(rhs.value)
            .ok_or(ExactMassError::ArithmeticUnderflow)?;
        Ok(ExactMass::new(self.frame, value))
    }

    pub fn checked_mul_small(self, factor: u32) -> Result<ExactMass, ExactMassError> {
        let value = self
            .value
            .checked_mul_small(factor)
            .ok_or(ExactMassError::ArithmeticOverflow)?;
        Ok(ExactMass::new(self.frame, value))
    }

    /// Multiplies by `420^delta` and advances the frame in one checked step.
    ///
    /// There is intentionally no `ExactMass::checked_mul_pow_420`: an exact
    /// mass cannot be rescaled while leaving its denominator metadata stale.
    pub fn checked_advance_field_scale(self, delta: u8) -> Result<ExactMass, ExactMassError> {
        let frame = self.frame.checked_advance_field_actions(delta).ok_or(
            ExactMassError::ScaleAdvanceOutOfRange {
                frame: self.frame,
                delta,
            },
        )?;
        let value = self
            .value
            .checked_mul_pow_420(delta)
            .ok_or(ExactMassError::ArithmeticOverflow)?;
        Ok(ExactMass::new(frame, value))
    }

    /// Converts a partial field-scale mass to its declared full horizon.
    ///
    /// This is the only normalization operation: it atomically applies
    /// `420^(full_horizon_exponent - field_exponent)` and advances the frame
    /// to that full-horizon exponent.
    pub fn checked_normalize_to_full_horizon(self) -> Result<ExactMass, ExactMassError> {
        self.checked_advance_field_scale(self.frame.normalization_delta())
    }

    fn require_same_frame(self, rhs: ExactMass) -> Result<(), ExactMassError> {
        if self.frame == rhs.frame {
            Ok(())
        } else {
            Err(ExactMassError::ScaleMismatch {
                left: self.frame,
                right: rhs.frame,
            })
        }
    }
}

/// A host-only failure.  This enum never appears in a serialized GPU buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactMassError {
    ScaleMismatch { left: ScaleFrame, right: ScaleFrame },
    ScaleAdvanceOutOfRange { frame: ScaleFrame, delta: u8 },
    ArithmeticOverflow,
    ArithmeticUnderflow,
}

/// A nonempty number of hidden allocations supporting one opening stratum.
///
/// A zero-support stratum is not an opening cell and must be filtered before
/// it reaches exact mass construction.  Keeping this distinct from `u32`
/// prevents a count from being confused with a field multiplier or an index.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SupportCount(core::num::NonZeroU32);

const _: [(); 4] = [(); core::mem::size_of::<SupportCount>()];
const _: [(); 4] = [(); core::mem::align_of::<SupportCount>()];

impl SupportCount {
    /// Creates a support count only when at least one allocation exists.
    pub const fn new(value: u32) -> Option<SupportCount> {
        match core::num::NonZeroU32::new(value) {
            Some(value) => Some(SupportCount(value)),
            None => None,
        }
    }

    pub const fn value(self) -> u32 {
        self.0.get()
    }
}

/// The per-allocation opening likelihood coefficient `C(e, x)`.
///
/// It is intentionally not interchangeable with an aggregated opening mass.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningLikelihoodCoeff(u64);

const _: [(); 8] = [(); core::mem::size_of::<OpeningLikelihoodCoeff>()];

impl OpeningLikelihoodCoeff {
    pub const fn new(value: u64) -> OpeningLikelihoodCoeff {
        OpeningLikelihoodCoeff(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    /// Forms `W(e, x) = A(e, x) * C(e, x)` with a nonempty support count.
    pub fn checked_scale_by_support(
        self,
        support_count: SupportCount,
    ) -> Option<ScaledOpeningMass> {
        self.0
            .checked_mul(u64::from(support_count.value()))
            .map(ScaledOpeningMass)
    }
}

/// The sole v1 frame accepted when lifting an opening response mass.
///
/// Its private field and checked `TryFrom<ScaleFrame>` implementation prevent
/// an opening likelihood from being relabeled as another utility, role,
/// profile, or horizon at conversion time.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningResponseFrame(ScaleFrame);

const _: [(); SCALE_FRAME_BYTES] = [(); core::mem::size_of::<OpeningResponseFrame>()];
const _: [(); 2] = [(); core::mem::align_of::<OpeningResponseFrame>()];

impl OpeningResponseFrame {
    pub const UNIFORM_TRICK1_V1: OpeningResponseFrame = OpeningResponseFrame(ScaleFrame {
        prior_profile: PriorProfileId::UNIFORM_OPENING_V1,
        field_profile: FieldProfileId::UNIFORM_RANDOM_LEGAL_V1,
        utility_profile: UtilityProfileId::NOT_APPLICABLE_V1,
        measure_role: MeasureRoleId::OPENING_RESPONSE_MASS_V1,
        field_exponent: OPENING_RESPONSE_FIELD_EXPONENT,
        full_horizon_exponent: TRICK1_FULL_HORIZON_EXPONENT,
        reserved: [0; 3],
    });

    pub const fn frame(self) -> ScaleFrame {
        self.0
    }
}

/// A decoded frame is not the exact v1 opening-response frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpeningResponseFrameError {
    NotUniformTrick1V1 { frame: ScaleFrame },
}

impl TryFrom<ScaleFrame> for OpeningResponseFrame {
    type Error = OpeningResponseFrameError;

    fn try_from(frame: ScaleFrame) -> Result<OpeningResponseFrame, Self::Error> {
        if frame == OpeningResponseFrame::UNIFORM_TRICK1_V1.frame() {
            Ok(OpeningResponseFrame::UNIFORM_TRICK1_V1)
        } else {
            Err(OpeningResponseFrameError::NotUniformTrick1V1 { frame })
        }
    }
}

/// An opening response or stratum mass at the common opening scale.
///
/// Only an explicit opening-frame lift can turn this narrow opening value into
/// a full-width `ExactMass`; there is deliberately no arbitrary-frame lift.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScaledOpeningMass(u64);

const _: [(); 8] = [(); core::mem::size_of::<ScaledOpeningMass>()];

impl ScaledOpeningMass {
    pub const fn new(value: u64) -> ScaledOpeningMass {
        ScaledOpeningMass(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub fn checked_add(self, rhs: ScaledOpeningMass) -> Option<ScaledOpeningMass> {
        self.0.checked_add(rhs.0).map(ScaledOpeningMass)
    }

    /// Lifts an opening response mass at its fixed `420^3` trick-1 frame.
    ///
    /// `OpeningResponseFrame` is either the closed v1 constant or a frame
    /// verified against it through `TryFrom`, so the lift cannot launder
    /// profile, role, scale, or horizon metadata.
    pub const fn into_opening_response_exact_mass(self, frame: OpeningResponseFrame) -> ExactMass {
        ExactMass::new(frame.frame(), U256Mass::from_u64(self.0))
    }
}
