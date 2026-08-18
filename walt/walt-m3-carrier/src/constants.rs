use walt_core::{Domino, Seat};

pub const RAW_RECEIPT_BYTES: usize = 6_650;
pub const CARRIER_HAND_ID: usize = 8;
pub const CARRIER_TRICK: usize = 4;
pub const SUPPORT_COUNT: usize = 1_200;
pub const VOID_FREE_PARENT_COUNT: usize = 34_650;
pub const SUPPORT_RECORD_BYTES: usize = 16;
pub const PUBLIC_PREFIX_PAIR_COUNT: usize = 12;
pub const PUBLIC_PREFIX_BYTES_LEN: usize = 24;
pub const CARRIER_PROFILE_BYTES: usize = 128;
pub const ROOT_ALIAS_KAT_BYTES: usize = 128;
pub const FREEZE57_DESCRIPTOR_LEN: usize = 962;

pub const HAND_TOTAL_POINTS: u32 = walt_core::replay::HAND_TOTAL_POINTS;
pub const UNBANKED_POINTS_BEFORE_TRICK4: u32 = 34;
pub const P30_INITIAL_T0_ALLOWANCE: u32 = 12;
pub const P30_REMAINING_T0_ALLOWANCE: u32 = 11;
pub const FUTURE_FIELD_MOVES: u32 = 12;
pub const ROOT_ALIAS_PROJECTED_RESPONSE_RECORDS: u64 = 13_340;

pub const VIEWER: Seat = Seat::S1;
pub const HIDDEN_SEAT_ORDER: [Seat; 3] = [Seat::S2, Seat::S3, Seat::S0];
pub const LEGAL_ROOT_MASK: u32 = 0x0010_0290;
pub const HIDDEN_POOL_MASK: u32 = 0x0aa4_ac49;

pub const OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL: u32 = 1;
pub const OBJECTIVE_M3B_P30_MAKE: u32 = 2;
pub const TREATMENT_H_LAWFUL_PERFECT_RECALL: u32 = 1;
pub const TREATMENT_C_WORLD_REVEALED: u32 = 2;
pub const BELIEF_PROFILE_UNIFORM_COMPATIBLE_SUPPORT: u32 = 1;
pub const HISTORY_PROFILE_HISTORICAL_VOID_FEASIBILITY_ONLY: u32 = 1;
pub const FIELD_LAW_UNIFORM_RANDOM_LEGAL: u32 = 1;
pub const ROOT_ALIAS_KAT_PROFILE: u32 = 1;

pub const STREAM_PURPOSE_SUPPORT: u32 = 1;
pub const STREAM_PURPOSE_PUBLIC_PREFIX: u32 = 2;

pub const ROOTS: [Domino; 4] = [
    Domino::ALL[4],
    Domino::ALL[7],
    Domino::ALL[9],
    Domino::ALL[20],
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskMetadata {
    pub ordinal: u32,
    pub objective: u32,
    pub root_index: u32,
}

pub const TASKS: [TaskMetadata; 8] = [
    TaskMetadata {
        ordinal: 0,
        objective: OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
        root_index: 4,
    },
    TaskMetadata {
        ordinal: 1,
        objective: OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
        root_index: 7,
    },
    TaskMetadata {
        ordinal: 2,
        objective: OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
        root_index: 9,
    },
    TaskMetadata {
        ordinal: 3,
        objective: OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL,
        root_index: 20,
    },
    TaskMetadata {
        ordinal: 4,
        objective: OBJECTIVE_M3B_P30_MAKE,
        root_index: 4,
    },
    TaskMetadata {
        ordinal: 5,
        objective: OBJECTIVE_M3B_P30_MAKE,
        root_index: 7,
    },
    TaskMetadata {
        ordinal: 6,
        objective: OBJECTIVE_M3B_P30_MAKE,
        root_index: 9,
    },
    TaskMetadata {
        ordinal: 7,
        objective: OBJECTIVE_M3B_P30_MAKE,
        root_index: 20,
    },
];

const fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("invalid frozen lowercase hexadecimal byte"),
    }
}

const fn decode_hex<const N: usize>(hex: &[u8]) -> [u8; N] {
    if hex.len() != N * 2 {
        panic!("wrong frozen hexadecimal width");
    }
    let mut output = [0u8; N];
    let mut index = 0;
    while index < N {
        output[index] = (hex_nibble(hex[index * 2]) << 4) | hex_nibble(hex[index * 2 + 1]);
        index += 1;
    }
    output
}

pub const FREEZE55_PARENT_COMMIT_SHA1: [u8; 20] =
    decode_hex(b"3b4c6d60fef371e3050de151ccf9eaefbc2d2da7");
pub const FREEZE56_CLOSURE_COMMIT_SHA1: [u8; 20] =
    decode_hex(b"20a9feccb71660d10dcca3e334867e7b5400a837");
pub const FREEZE56_DESCRIPTOR_SHA256: [u8; 32] =
    decode_hex(b"7bdc5e05513fd1d7e7b6c26870cf9bd4a16966c5daf48963729d999c4b6b28cf");
pub const M0_M2_SOURCE_MANIFEST_SHA256: [u8; 32] =
    decode_hex(b"257d2fdb6aee327c061050bffe8fdf55f52bf6ca9e8972820847b6ac9a06fd24");
pub const M3_REBRIEF_SHA256: [u8; 32] =
    decode_hex(b"07b3c993260ca25524ac1df2c3e3bd864ce66401ba6666d5ac918f633be3bf31");
pub const M3_CONTRACT_SHA256: [u8; 32] =
    decode_hex(b"79de73e9ee9b0e1fd3b0467ddf27a66dcc9e135419cba531cb73218d71eee147");
pub const RAW_RECEIPT_SHA256: [u8; 32] =
    decode_hex(b"cf2c9dd2a07215fefb2644b1830c11a156e4d8f699a85fd96957f2782d91691c");
pub const FREEZE57_DESCRIPTOR_SHA256: [u8; 32] =
    decode_hex(b"e5efe6ce5c293b29fc05902e7bf913fd13f04a031c2951f7a1bf5cf92137f852");
pub const ROOT_ALIAS_KAT_INPUT_SHA256: [u8; 32] =
    decode_hex(b"b982979b0fce873086e1b1125eae51425cb220d55762a5db68ef634dfb7b49a9");
pub const ROOT_ALIAS_KAT_EXPECTED_SHA256: [u8; 32] =
    decode_hex(b"fc89dfcffb0c3fc721598e7919df958dd6cef82b1c9900cf62490596fed285c3");
pub const SUPPORT_STREAM_DIGEST: [u8; 32] =
    decode_hex(b"48975e2c660697ed24bab134fa3bb8c33614dee5325c4784ffb59ea4b4fb978d");
pub const PUBLIC_PREFIX_STREAM_DIGEST: [u8; 32] =
    decode_hex(b"433612199b91542cd0e7eecee30fdcaae3d565ccfb2cce79893f05a5b635d7ac");
pub const CARRIER_PROFILE_SHA256: [u8; 32] =
    decode_hex(b"d2e3d85f3f2576ea8d6ad77d8743de8a1ae28166cc9ff83a14a835cbb87fafd3");
pub const ROOT_ALIAS_PROJECTED_RESPONSE_DIGEST: [u8; 32] =
    decode_hex(b"8674e288fa2102dceacab3af7986c10d1f25f647103ef30b68128af53ccc5dcd");

pub const FREEZE57_DESCRIPTOR_BYTES: &[u8; FREEZE57_DESCRIPTOR_LEN] = b"GT1-M3-FREEZE-SET-V1|authority=GPU-NATIVE-TRICK1-M3-v1@79de73e9ee9b0e1fd3b0467ddf27a66dcc9e135419cba531cb73218d71eee147+GT1-A18..GT1-A24+freeze57|parent=freeze56@7bdc5e05513fd1d7e7b6c26870cf9bd4a16966c5daf48963729d999c4b6b28cf;commit=20a9feccb71660d10dcca3e334867e7b5400a837|rebrief=07b3c993260ca25524ac1df2c3e3bd864ce66401ba6666d5ac918f633be3bf31|profile=UniformCompatibleSupportV1,HistoricalVoidFeasibilityOnlyV1,UniformRandomLegalV1|objectives=M3A_FUTURE_TRICK_DIFFERENTIAL,M3B_P30_MAKE|treatments=H_LAWFUL_PERFECT_RECALL,C_WORLD_REVEALED|carrier=M3CarrierProfileV1,h8,roots21-31-33-55|keys=M3PerfectRecallKeyV1,M3WorldRevealedKeyV1|arithmetic=U256MassV1|kernels=m3_field_expand_v1,m3_u256_reduce_pass_v1|reductions=MASS_BUCKET,BACKWARD_VALUE|receipt=W42M3R01,M3FailureReceiptV1|manifest=M3SourceManifestV1|proof=Texas42.Trick1PerfectRecallNet|reserved=39,40|excluded=trick1-value,lead-choice,compression,growth,performance,controller,strategy-strength,player";
