use std::collections::HashSet;
use std::sync::OnceLock;

use walt_core::{Context, ContextSet, Decl, Domino, Pip, Seat, Team};
use walt_gpu_spec::sha256;
use walt_m3_carrier::{
    root_alias_kat_expected_bytes, root_alias_kat_input_bytes, CarrierError, CarrierSupport,
    ConstrainedSupportIter, M3Carrier, TaskMetadata, CARRIER_HAND_ID, CARRIER_PROFILE_BYTES,
    CARRIER_PROFILE_SHA256, CARRIER_TRICK, FREEZE55_PARENT_COMMIT_SHA1,
    FREEZE56_CLOSURE_COMMIT_SHA1, FREEZE56_DESCRIPTOR_SHA256, FREEZE57_DESCRIPTOR_BYTES,
    FREEZE57_DESCRIPTOR_LEN, FREEZE57_DESCRIPTOR_SHA256, FUTURE_FIELD_MOVES, HAND_TOTAL_POINTS,
    HIDDEN_POOL_MASK, HIDDEN_SEAT_ORDER, LEGAL_ROOT_MASK, M0_M2_SOURCE_MANIFEST_SHA256,
    M3_CONTRACT_SHA256, M3_REBRIEF_SHA256, OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
    OBJECTIVE_M3B_P30_MAKE, P30_INITIAL_T0_ALLOWANCE, P30_REMAINING_T0_ALLOWANCE,
    PUBLIC_PREFIX_BYTES, PUBLIC_PREFIX_BYTES_LEN, PUBLIC_PREFIX_PAIR_COUNT,
    PUBLIC_PREFIX_STREAM_DIGEST, RAW_RECEIPT_BYTES, RAW_RECEIPT_SHA256, ROOTS,
    ROOT_ALIAS_KAT_BYTES, ROOT_ALIAS_KAT_EXPECTED_SHA256, ROOT_ALIAS_KAT_INPUT_SHA256,
    ROOT_ALIAS_PROJECTED_RESPONSE_DIGEST, ROOT_ALIAS_PROJECTED_RESPONSE_RECORDS,
    STREAM_PURPOSE_PUBLIC_PREFIX, STREAM_PURPOSE_SUPPORT, SUPPORT_COUNT, SUPPORT_RECORD_BYTES,
    SUPPORT_STREAM_DIGEST, TASKS, UNBANKED_POINTS_BEFORE_TRICK4, VIEWER, VOID_FREE_PARENT_COUNT,
};

const RAW_RECEIPT: &[u8] = include_bytes!("../../../rob/receipts/verify_player.txt");

fn admitted_carrier() -> &'static M3Carrier {
    static CARRIER: OnceLock<M3Carrier> = OnceLock::new();
    CARRIER.get_or_init(|| {
        M3Carrier::from_receipt_bytes(RAW_RECEIPT)
            .expect("the frozen raw receipt must construct the sole M3 carrier")
    })
}

#[test]
fn frozen_source_identities_are_exact() {
    assert_eq!(RAW_RECEIPT.len(), RAW_RECEIPT_BYTES);
    assert_eq!(sha256(RAW_RECEIPT), RAW_RECEIPT_SHA256);
    assert_eq!(FREEZE57_DESCRIPTOR_BYTES.len(), FREEZE57_DESCRIPTOR_LEN);
    assert_eq!(
        sha256(FREEZE57_DESCRIPTOR_BYTES),
        FREEZE57_DESCRIPTOR_SHA256
    );
    assert!(!FREEZE57_DESCRIPTOR_BYTES.contains(&b'\n'));
    assert_eq!(FREEZE55_PARENT_COMMIT_SHA1.len(), 20);
    assert_eq!(FREEZE56_CLOSURE_COMMIT_SHA1.len(), 20);
    assert_eq!(FREEZE56_DESCRIPTOR_SHA256.len(), 32);
    assert_eq!(M0_M2_SOURCE_MANIFEST_SHA256.len(), 32);
    assert_eq!(M3_REBRIEF_SHA256.len(), 32);
    assert_eq!(M3_CONTRACT_SHA256.len(), 32);
}

#[test]
fn replay_reconstructs_the_frozen_hand_eight_cut() {
    let facts = admitted_carrier().facts();
    assert_eq!(CARRIER_HAND_ID, 8);
    assert_eq!(CARRIER_TRICK, 4);
    assert_eq!(facts.shaker, Seat::S0);
    assert_eq!(facts.bidder, Seat::S1);
    assert_eq!(facts.declaration, Decl::PipTrump(Pip::new(5).unwrap()));
    assert_eq!(facts.declaring_team, Team::T1);
    assert_eq!(facts.viewer, Seat::S1);
    assert_eq!(facts.next_leader, Seat::S1);
    assert_eq!(facts.contract_points, 30);
    assert_eq!(facts.viewer_hand.bits(), LEGAL_ROOT_MASK);
    assert_eq!(facts.hidden_pool.bits(), HIDDEN_POOL_MASK);
    assert_eq!(facts.viewer_hand.len(), 4);
    assert_eq!(facts.hidden_pool.len(), 12);
    assert!(facts.viewer_hand.is_disjoint(facts.hidden_pool));
    assert_eq!(facts.banked_points, [1, 7]);
    assert_eq!(HAND_TOTAL_POINTS, 42);
    assert_eq!(facts.unbanked_points(), UNBANKED_POINTS_BEFORE_TRICK4);
    assert_eq!(facts.p30_initial_t0_allowance(), P30_INITIAL_T0_ALLOWANCE);
    assert_eq!(
        facts.p30_remaining_t0_allowance(),
        P30_REMAINING_T0_ALLOWANCE
    );
    assert_eq!(facts.future_field_moves(), FUTURE_FIELD_MOVES);

    let natural_1 = Context::Natural(Pip::new(1).unwrap());
    let natural_4 = Context::Natural(Pip::new(4).unwrap());
    assert_eq!(facts.voids[Seat::S0.index()], ContextSet::EMPTY);
    assert_eq!(facts.voids[Seat::S1.index()], ContextSet::single(natural_4));
    assert_eq!(
        facts.voids[Seat::S2.index()],
        ContextSet::single(Context::Called).union(ContextSet::single(natural_1))
    );
    assert_eq!(facts.voids[Seat::S3.index()], ContextSet::single(natural_4));

    assert_eq!(PUBLIC_PREFIX_PAIR_COUNT, 12);
    assert_eq!(PUBLIC_PREFIX_BYTES_LEN, 24);
    assert_eq!(PUBLIC_PREFIX_BYTES.len(), PUBLIC_PREFIX_BYTES_LEN);
    assert_eq!(
        PUBLIC_PREFIX_BYTES,
        [1, 17, 2, 24, 3, 16, 0, 26, 0, 12, 1, 19, 2, 14, 3, 8, 1, 2, 2, 5, 3, 1, 0, 22,]
    );
}

#[test]
fn support_is_exact_complete_and_independently_reproduced() {
    let carrier = admitted_carrier();
    let support = carrier.support();
    assert_eq!(VOID_FREE_PARENT_COUNT, 34_650);
    assert_eq!(support.len(), SUPPORT_COUNT);
    assert!(!support.is_empty());

    let payload = support.payload();
    assert_eq!(payload.len(), SUPPORT_COUNT * SUPPORT_RECORD_BYTES);
    for (ordinal, record) in support.records().iter().copied().enumerate() {
        assert_eq!(record.hand(VIEWER), carrier.facts().viewer_hand);
        assert_eq!(record.to_bytes(), payload[ordinal * 16..ordinal * 16 + 16]);
        for seat in Seat::ALL {
            assert_eq!(record.hand(seat).len(), 4);
        }
    }

    let unique: HashSet<[u8; SUPPORT_RECORD_BYTES]> = support
        .records()
        .iter()
        .copied()
        .map(|record| record.to_bytes())
        .collect();
    assert_eq!(unique.len(), SUPPORT_COUNT);
    assert_eq!(
        support.records()[0].to_bytes(),
        [
            0x00, 0x28, 0x00, 0x0a, 0x90, 0x02, 0x10, 0x00, 0x49, 0x04, 0x00, 0x00, 0x00, 0x80,
            0xa4, 0x00,
        ]
    );
    assert_eq!(
        support.records()[SUPPORT_COUNT - 1].to_bytes(),
        [
            0x01, 0x2c, 0x00, 0x00, 0x90, 0x02, 0x10, 0x00, 0x00, 0x00, 0xa0, 0x0a, 0x48, 0x80,
            0x04, 0x00,
        ]
    );

    let independent: Vec<_> = ConstrainedSupportIter::new(carrier.facts())
        .expect("independent constrained traversal must construct")
        .collect();
    assert_eq!(independent.as_slice(), support.records());

    let reparsed = CarrierSupport::from_payload(&payload, carrier.facts())
        .expect("the support parser must regenerate every exact record");
    assert_eq!(&reparsed, support);
    assert!(matches!(
        CarrierSupport::from_payload(&payload[..payload.len() - 1], carrier.facts()),
        Err(CarrierError::SupportPayloadLength { .. })
    ));
    let mut high_bit = payload.clone();
    high_bit[3] |= 0x80;
    assert!(matches!(
        CarrierSupport::from_payload(&high_bit, carrier.facts()),
        Err(CarrierError::InvalidSupport {
            ordinal: 0,
            field: "low-28-bit hand mask"
        })
    ));
}

#[test]
fn support_prefix_and_profile_domains_are_closed() {
    let carrier = admitted_carrier();
    let support_payload = carrier.support().payload();
    let support_digest = carrier.support().digest();
    let prefix_digest = carrier.public_prefix_digest();
    assert_eq!(support_digest, SUPPORT_STREAM_DIGEST);
    assert_eq!(prefix_digest, PUBLIC_PREFIX_STREAM_DIGEST);
    assert_ne!(support_digest, sha256(&support_payload));
    assert_ne!(prefix_digest, sha256(&PUBLIC_PREFIX_BYTES));
    assert_ne!(support_digest, prefix_digest);

    let profile = carrier.carrier_profile_bytes();
    assert_eq!(profile.len(), CARRIER_PROFILE_BYTES);
    assert_eq!(&profile[0..8], b"W42M3CP1");
    assert_eq!(u32_at(&profile, 8), 1);
    assert_eq!(u32_at(&profile, 12), 128);
    assert_eq!(u64_at(&profile, 16), RAW_RECEIPT_BYTES as u64);
    assert_eq!(&profile[24..56], &RAW_RECEIPT_SHA256);
    assert_eq!(u32_at(&profile, 56), CARRIER_HAND_ID as u32);
    assert_eq!(u32_at(&profile, 60), CARRIER_TRICK as u32);
    assert_eq!(u32_at(&profile, 64), VIEWER.index() as u32);
    assert_eq!(u32_at(&profile, 68), SUPPORT_COUNT as u32);
    assert_eq!(u32_at(&profile, 72), LEGAL_ROOT_MASK);
    assert_eq!(u32_at(&profile, 76), 1);
    assert_eq!(u32_at(&profile, 80), 1);
    assert_eq!(u32_at(&profile, 84), 1);
    assert_eq!(&profile[88..120], &support_digest);
    assert!(profile[120..].iter().all(|byte| *byte == 0));
    assert_eq!(carrier.carrier_profile_digest(), sha256(&profile));
    assert_eq!(carrier.carrier_profile_digest(), CARRIER_PROFILE_SHA256);

    // The two logical streams use the same closed grammar but distinct
    // assigned purposes.
    assert_eq!(STREAM_PURPOSE_SUPPORT, 1);
    assert_eq!(STREAM_PURPOSE_PUBLIC_PREFIX, 2);
}

#[test]
fn roots_and_objective_tasks_have_the_frozen_order() {
    assert_eq!(VIEWER, Seat::S1);
    assert_eq!(HIDDEN_SEAT_ORDER, [Seat::S2, Seat::S3, Seat::S0]);
    assert_eq!(ROOTS.map(|domino| domino.index()), [4, 7, 9, 20]);
    assert_eq!(
        ROOTS,
        [
            Domino::ALL[4],
            Domino::ALL[7],
            Domino::ALL[9],
            Domino::ALL[20],
        ]
    );
    assert_eq!(TASKS.len(), 8);
    for (ordinal, task) in TASKS.into_iter().enumerate() {
        assert_eq!(task.ordinal, ordinal as u32);
        assert_eq!(task.root_index, [4, 7, 9, 20][ordinal % 4]);
        let expected_objective = if ordinal < 4 {
            OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL
        } else {
            OBJECTIVE_M3B_P30_MAKE
        };
        assert_eq!(task.objective, expected_objective);
    }
    assert_eq!(
        TASKS[0],
        TaskMetadata {
            ordinal: 0,
            objective: OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
            root_index: 4,
        }
    );
}

#[test]
fn fixed_root_alias_kat_is_semantically_recomputed() {
    let carrier = admitted_carrier();
    let input = root_alias_kat_input_bytes();
    let expected = root_alias_kat_expected_bytes();
    assert_eq!(input.len(), ROOT_ALIAS_KAT_BYTES);
    assert_eq!(expected.len(), ROOT_ALIAS_KAT_BYTES);
    assert_eq!(sha256(&input), ROOT_ALIAS_KAT_INPUT_SHA256);
    assert_eq!(sha256(&expected), ROOT_ALIAS_KAT_EXPECTED_SHA256);

    let evidence = carrier
        .validate_root_alias_kat(&input, &expected)
        .expect("the exact KAT records must follow from the admitted carrier");
    assert_eq!(
        evidence.response_record_count,
        ROOT_ALIAS_PROJECTED_RESPONSE_RECORDS
    );
    assert_eq!(
        evidence.projected_response_digest,
        ROOT_ALIAS_PROJECTED_RESPONSE_DIGEST
    );
}

#[test]
fn kat_and_receipt_mutations_are_rejected_before_admission() {
    let carrier = admitted_carrier();
    let mut input = root_alias_kat_input_bytes();
    input[48] ^= 1;
    assert!(matches!(
        carrier.validate_root_alias_kat(&input, &root_alias_kat_expected_bytes()),
        Err(CarrierError::Kat("fixed input bytes"))
    ));

    let mut expected = root_alias_kat_expected_bytes();
    expected[40] = 0;
    assert!(matches!(
        carrier.validate_root_alias_kat(&root_alias_kat_input_bytes(), &expected),
        Err(CarrierError::Kat("fixed expected-result bytes"))
    ));

    assert!(matches!(
        M3Carrier::from_receipt_bytes(&RAW_RECEIPT[..RAW_RECEIPT.len() - 1]),
        Err(CarrierError::ReceiptLength { .. })
    ));
    let mut appended = RAW_RECEIPT.to_vec();
    appended.push(0);
    assert!(matches!(
        M3Carrier::from_receipt_bytes(&appended),
        Err(CarrierError::ReceiptLength { .. })
    ));
    let mut corrupted = RAW_RECEIPT.to_vec();
    corrupted[0] ^= 1;
    assert!(matches!(
        M3Carrier::from_receipt_bytes(&corrupted),
        Err(CarrierError::ReceiptDigest { .. })
    ));
}

fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn u64_at(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}
