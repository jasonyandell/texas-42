use core::{
    cmp::Ordering,
    mem::{align_of, size_of},
};

use num_bigint::BigUint;
use walt::rules::{Context, Decl, Domino, Tier};
use walt::spec::{
    sha256, ExactMass, ExactMassError, FieldProfileId, MeasureRoleId, OpeningLikelihoodCoeff,
    OpeningResponseFrame, OpeningResponseFrameError, PriorProfileId, ScaleFrame, ScaledOpeningMass,
    SemanticTables, SupportCount, U256Mass, UtilityProfileId, FIELD_SCALE,
    OPENING_RESPONSE_FIELD_EXPONENT, SCALE_FRAME_BYTES, SHA256_BYTES, TABLE_FORMAT_VERSION,
    TRICK1_FULL_HORIZON_EXPONENT, U256_BYTES,
};

fn frame(exponent: u8) -> ScaleFrame {
    frame_with_horizon(exponent, TRICK1_FULL_HORIZON_EXPONENT)
}

fn frame_with_horizon(exponent: u8, full_horizon_exponent: u8) -> ScaleFrame {
    ScaleFrame::new(
        PriorProfileId::UNIFORM_OPENING_V1,
        FieldProfileId::UNIFORM_RANDOM_LEGAL_V1,
        UtilityProfileId::DECLARING_TEAM_MAKES_V1,
        MeasureRoleId::WEIGHTED_CONTRIBUTION_V1,
        exponent,
        full_horizon_exponent,
    )
    .expect("test frame must have a valid scale range")
}

fn next_u32(state: &mut u64) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 16) as u32
}

fn next_u256(state: &mut u64) -> U256Mass {
    U256Mass(core::array::from_fn(|_| next_u32(state)))
}

fn big(value: U256Mass) -> BigUint {
    BigUint::from_bytes_le(&value.to_le_bytes())
}

fn u256_from_big(value: &BigUint) -> U256Mass {
    let mut bytes = value.to_bytes_le();
    assert!(bytes.len() <= U256_BYTES, "test oracle exceeds U256");
    bytes.resize(U256_BYTES, 0);
    let bytes: [u8; U256_BYTES] = bytes.try_into().expect("fixed test byte width");
    U256Mass::from_le_bytes(bytes)
}

fn table_key_code(decl: Decl, context: Context, domino: Domino) -> u16 {
    let key = decl.trick_key(domino, context);
    let tier = match key.tier {
        Tier::Slough => 0u16,
        Tier::Follows => 1u16,
        Tier::Called => 2u16,
    };
    (tier << 8) | u16::from(key.rank.value())
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[test]
fn m0_layout_contracts_are_fixed() {
    assert_eq!(size_of::<U256Mass>(), 32);
    assert_eq!(align_of::<U256Mass>(), 4);
    assert_eq!(size_of::<ScaleFrame>(), SCALE_FRAME_BYTES);
    assert_eq!(align_of::<ScaleFrame>(), 2);
    assert_eq!(size_of::<ExactMass>(), 44);
    assert_eq!(align_of::<ExactMass>(), 4);
    assert_eq!(size_of::<OpeningLikelihoodCoeff>(), 8);
    assert_eq!(size_of::<ScaledOpeningMass>(), 8);
    assert_eq!(size_of::<SupportCount>(), 4);
    assert_eq!(align_of::<SupportCount>(), 4);
    assert_eq!(size_of::<OpeningResponseFrame>(), SCALE_FRAME_BYTES);
}

#[test]
fn m0_u256_limb_and_byte_order_are_little_endian() {
    let value = U256Mass([
        0x0302_0100,
        0x0706_0504,
        0x0b0a_0908,
        0x0f0e_0d0c,
        0x1312_1110,
        0x1716_1514,
        0x1b1a_1918,
        0x1f1e_1d1c,
    ]);
    let expected: [u8; U256_BYTES] = core::array::from_fn(|index| index as u8);
    assert_eq!(value.to_le_bytes(), expected);
    assert_eq!(U256Mass::from_le_bytes(expected), value);
    assert_eq!(value.limbs_le()[0], 0x0302_0100);
    assert_eq!(value.limbs_le()[7], 0x1f1e_1d1c);
}

#[test]
fn m0_u256_checked_boundaries_hold() {
    assert_eq!(
        U256Mass::ZERO.checked_add(U256Mass::ZERO),
        Some(U256Mass::ZERO)
    );
    assert_eq!(U256Mass::MAX.checked_add(U256Mass::from_u64(1)), None);
    assert_eq!(U256Mass::ZERO.checked_sub(U256Mass::from_u64(1)), None);
    assert_eq!(
        U256Mass::from_u64(9).checked_sub(U256Mass::from_u64(4)),
        Some(U256Mass::from_u64(5))
    );
    assert_eq!(U256Mass::MAX.checked_mul_small(2), None);
    assert_eq!(U256Mass::MAX.checked_mul_pow_420(1), None);

    let root_deals = U256Mass::from_u64(399_072_960);
    let scaled = root_deals
        .checked_mul_pow_420(21)
        .expect("full root scale fits in U256");
    let reference = BigUint::from(399_072_960u64) * BigUint::from(FIELD_SCALE).pow(21);
    assert_eq!(scaled, u256_from_big(&reference));
}

#[test]
fn m0_u256_matches_biguint_oracle() {
    let mut state = 0x8e31_9b4c_17d2_05a9u64;
    let limit = BigUint::from(1u8) << 256usize;
    for _ in 0..256 {
        let left = next_u256(&mut state);
        let right = next_u256(&mut state);
        let factor = next_u32(&mut state);

        let sum = big(left) + big(right);
        assert_eq!(
            left.checked_add(right),
            (sum < limit).then(|| u256_from_big(&sum))
        );

        let product = big(left) * BigUint::from(factor);
        assert_eq!(
            left.checked_mul_small(factor),
            (product < limit).then(|| u256_from_big(&product))
        );

        let left_big = big(left);
        let right_big = big(right);
        assert_eq!(
            left.checked_sub(right),
            (left_big >= right_big).then(|| u256_from_big(&(left_big - right_big)))
        );
    }
}

#[test]
fn m0_scale_frame_is_explicit_and_validated() {
    let root = frame(3);
    let expected = [
        1,
        0,
        1,
        0,
        2,
        0,
        3,
        3,
        TRICK1_FULL_HORIZON_EXPONENT,
        0,
        0,
        0,
    ];
    assert_eq!(root.to_le_bytes(), expected);
    assert_eq!(ScaleFrame::from_le_bytes(expected), Some(root));
    assert_eq!(
        ScaleFrame::new(
            PriorProfileId::UNIFORM_OPENING_V1,
            FieldProfileId::UNIFORM_RANDOM_LEGAL_V1,
            UtilityProfileId::DECLARING_TEAM_MAKES_V1,
            MeasureRoleId::WEIGHTED_CONTRIBUTION_V1,
            22,
            TRICK1_FULL_HORIZON_EXPONENT,
        ),
        None
    );
    let invalid_range = [1, 0, 1, 0, 2, 0, 2, 22, 21, 0, 0, 0];
    assert_eq!(ScaleFrame::from_le_bytes(invalid_range), None);
    let unknown_profile = [2, 0, 1, 0, 2, 0, 2, 3, 21, 0, 0, 0];
    assert_eq!(ScaleFrame::from_le_bytes(unknown_profile), None);
    let noncanonical_reserved = [1, 0, 1, 0, 2, 0, 2, 3, 21, 1, 0, 0];
    assert_eq!(ScaleFrame::from_le_bytes(noncanonical_reserved), None);
    assert_eq!(PriorProfileId::try_from_raw(2), None);
    assert_eq!(FieldProfileId::try_from_raw(2), None);
    assert_eq!(UtilityProfileId::try_from_raw(99), None);
    assert_eq!(MeasureRoleId::try_from_raw(4), None);
    assert_eq!(MeasureRoleId::try_from_raw(99), None);
}

#[test]
fn m0_opening_types_keep_coefficients_and_masses_distinct() {
    let coeff = OpeningLikelihoodCoeff::new(60);
    let stratum = coeff
        .checked_scale_by_support(SupportCount::new(7).expect("positive support"))
        .expect("opening product fits in u64");
    assert_eq!(SupportCount::new(0), None);
    assert_eq!(coeff.value(), 60);
    assert_eq!(stratum.value(), 420);
    assert_eq!(
        stratum.checked_add(ScaledOpeningMass::new(1)),
        Some(ScaledOpeningMass::new(421))
    );

    let lifted = stratum.into_opening_response_exact_mass(OpeningResponseFrame::UNIFORM_TRICK1_V1);
    assert_eq!(lifted.value(), U256Mass::from_u64(420));
    assert_eq!(
        lifted.frame().field_exponent(),
        OPENING_RESPONSE_FIELD_EXPONENT
    );
    assert!(matches!(
        OpeningResponseFrame::try_from(lifted.frame()),
        Ok(OpeningResponseFrame::UNIFORM_TRICK1_V1)
    ));
    assert!(matches!(
        OpeningResponseFrame::try_from(frame(OPENING_RESPONSE_FIELD_EXPONENT)),
        Err(OpeningResponseFrameError::NotUniformTrick1V1 { .. })
    ));
}

#[test]
fn m0_scale_frame_rejects_unlike_mass_addition() {
    let left = ExactMass::new(frame(3), U256Mass::from_u64(40));
    let right = ExactMass::new(frame(3), U256Mass::from_u64(2));
    assert_eq!(
        left.checked_add(right),
        Ok(ExactMass::new(frame(3), U256Mass::from_u64(42)))
    );

    let exponent_mismatch = ExactMass::new(frame(4), U256Mass::from_u64(2));
    assert!(matches!(
        left.checked_add(exponent_mismatch),
        Err(ExactMassError::ScaleMismatch { .. })
    ));

    let utility_mismatch = ScaleFrame::new(
        PriorProfileId::UNIFORM_OPENING_V1,
        FieldProfileId::UNIFORM_RANDOM_LEGAL_V1,
        UtilityProfileId::NOT_APPLICABLE_V1,
        MeasureRoleId::WEIGHTED_CONTRIBUTION_V1,
        3,
        TRICK1_FULL_HORIZON_EXPONENT,
    )
    .expect("valid utility mismatch frame");
    assert!(matches!(
        left.checked_add(ExactMass::new(utility_mismatch, U256Mass::from_u64(2))),
        Err(ExactMassError::ScaleMismatch { .. })
    ));

    let role_mismatch = ScaleFrame::new(
        PriorProfileId::UNIFORM_OPENING_V1,
        FieldProfileId::UNIFORM_RANDOM_LEGAL_V1,
        UtilityProfileId::DECLARING_TEAM_MAKES_V1,
        MeasureRoleId::CONDITIONAL_VALUE_V1,
        3,
        TRICK1_FULL_HORIZON_EXPONENT,
    )
    .expect("valid role mismatch frame");
    assert!(matches!(
        left.checked_add(ExactMass::new(role_mismatch, U256Mass::from_u64(2))),
        Err(ExactMassError::ScaleMismatch { .. })
    ));

    let horizon_mismatch = ExactMass::new(frame_with_horizon(3, 20), U256Mass::from_u64(2));
    assert!(matches!(
        left.checked_add(horizon_mismatch),
        Err(ExactMassError::ScaleMismatch { .. })
    ));

    assert_eq!(left.checked_cmp(right), Ok(Ordering::Greater));
    assert_eq!(
        left.checked_sub(right),
        Ok(ExactMass::new(frame(3), U256Mass::from_u64(38)))
    );
    assert!(matches!(
        right.checked_sub(left),
        Err(ExactMassError::ArithmeticUnderflow)
    ));
    assert!(matches!(
        left.checked_cmp(exponent_mismatch),
        Err(ExactMassError::ScaleMismatch { .. })
    ));
    assert!(matches!(
        left.checked_sub(exponent_mismatch),
        Err(ExactMassError::ScaleMismatch { .. })
    ));

    let opening_mass = ScaledOpeningMass::new(2)
        .into_opening_response_exact_mass(OpeningResponseFrame::UNIFORM_TRICK1_V1);
    assert!(matches!(
        left.checked_cmp(opening_mass),
        Err(ExactMassError::ScaleMismatch { .. })
    ));
}

#[test]
fn m0_normalization_advances_value_and_frame_together() {
    let partial = ExactMass::new(frame_with_horizon(3, 5), U256Mass::from_u64(7));
    let advanced = partial
        .checked_advance_field_scale(1)
        .expect("one remaining field action fits");
    assert_eq!(
        advanced.value(),
        U256Mass::from_u64(7 * u64::from(FIELD_SCALE))
    );
    assert_eq!(advanced.frame().field_exponent(), 4);
    assert_eq!(advanced.frame().full_horizon_exponent(), 5);

    let normalized = partial
        .checked_normalize_to_full_horizon()
        .expect("two remaining field actions fit");
    assert_eq!(
        normalized.value(),
        U256Mass::from_u64(7)
            .checked_mul_pow_420(2)
            .expect("small normalization fits")
    );
    assert_eq!(normalized.frame().field_exponent(), 5);
    assert_eq!(normalized.frame().full_horizon_exponent(), 5);
    assert_eq!(partial.frame().field_exponent(), 3);
    assert!(matches!(
        partial.checked_advance_field_scale(3),
        Err(ExactMassError::ScaleAdvanceOutOfRange { delta: 3, .. })
    ));

    let already_normalized = ExactMass::new(frame_with_horizon(5, 5), U256Mass::from_u64(9));
    assert_eq!(
        already_normalized.checked_normalize_to_full_horizon(),
        Ok(already_normalized)
    );
}

#[test]
fn m0_semantic_tables_match_walt_core_exhaustively() {
    let tables = SemanticTables::from_walt_core();
    for decl in Decl::ALL {
        for context in Context::ALL {
            assert_eq!(
                tables.context_mask(decl, context),
                decl.effective_incidence(context).bits(),
                "context mask drift for {decl:?} {context:?}"
            );
        }
        for domino in Domino::ALL {
            assert_eq!(
                tables.lead_context_bits(decl, domino),
                1u8 << decl.led_context(domino).index(),
                "lead context drift for {decl:?} {domino:?}"
            );
            assert_eq!(
                tables.rank(decl, domino),
                decl.rank(domino).value(),
                "rank drift for {decl:?} {domino:?}"
            );
            assert_eq!(tables.count_value(domino), domino.count() as u8);
        }
        for context in Context::ALL {
            for domino in Domino::ALL {
                assert_eq!(
                    tables.trick_key_code(decl, context, domino),
                    table_key_code(decl, context, domino),
                    "trick key drift for {decl:?} {context:?} {domino:?}"
                );
                assert_eq!(
                    tables.beats_mask(decl, context, domino),
                    decl.beats(context, domino).bits(),
                    "beats drift for {decl:?} {context:?} {domino:?}"
                );
            }
        }
    }
}

#[test]
fn m0_canonical_table_bytes_are_fixed() {
    let first = SemanticTables::from_walt_core().canonical_bytes();
    let second = SemanticTables::from_walt_core().canonical_bytes();
    assert_eq!(first, second);
    assert_eq!(first.len(), SemanticTables::canonical_byte_len());
    assert_eq!(&first[..8], b"W42GPU01");
    assert_eq!(
        u16::from_le_bytes([first[8], first[9]]),
        TABLE_FORMAT_VERSION
    );
    assert_eq!(first[10], 9);
    assert_eq!(first[11], 8);
    assert_eq!(first[12], 28);
    assert_eq!(first[13], 0);
    let payload_len = u32::from_le_bytes([first[14], first[15], first[16], first[17]]);
    assert_eq!(payload_len as usize, first.len() - 18);

    // This is a non-cryptographic regression fingerprint of the canonical
    // bytes.  A cache or receipt digest will use a separately selected hash.
    assert_eq!(fnv1a64(&first), 8_994_864_108_015_290_668);
}

#[test]
fn m0_sha256_is_fips_anchored_and_pins_the_table_blob() {
    assert_eq!(SHA256_BYTES, 32);
    assert_eq!(
        sha256(b""),
        [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ]
    );
    assert_eq!(
        sha256(b"abc"),
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad,
        ]
    );
    assert_eq!(
        sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        [
            0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e,
            0x60, 0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4,
            0x19, 0xdb, 0x06, 0xc1,
        ]
    );

    let canonical = SemanticTables::from_walt_core().canonical_bytes();
    assert_eq!(
        sha256(&canonical),
        [
            0x65, 0x95, 0xfa, 0xdb, 0x8c, 0x2a, 0xcf, 0x17, 0x4f, 0x10, 0x67, 0x00, 0xc6, 0x58,
            0xa9, 0x9a, 0x8a, 0x66, 0xb7, 0xd6, 0x3c, 0x53, 0x49, 0xdd, 0xda, 0x81, 0xe7, 0xab,
            0xd2, 0x08, 0xb6, 0x6b,
        ]
    );
}

#[test]
fn m0_small_scale_and_choose_tables_are_exact() {
    let tables = SemanticTables::from_walt_core();
    assert_eq!(tables.small_scale(0), None);
    assert_eq!(tables.small_scale(1), Some(420));
    assert_eq!(tables.small_scale(2), Some(210));
    assert_eq!(tables.small_scale(3), Some(140));
    assert_eq!(tables.small_scale(4), Some(105));
    assert_eq!(tables.small_scale(5), Some(84));
    assert_eq!(tables.small_scale(6), Some(70));
    assert_eq!(tables.small_scale(7), Some(60));
    assert_eq!(tables.small_scale(8), None);
    assert_eq!(tables.choose(21, 7), Some(116_280));
    assert_eq!(tables.choose(21, 10), Some(352_716));
    assert_eq!(tables.choose(3, 4), Some(0));
    assert_eq!(tables.choose(22, 0), None);
}
