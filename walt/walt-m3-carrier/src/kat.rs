use walt_core::{legal_plays, Context, Domino, DominoSet, Seat};
use walt_gpu_spec::Sha256State;

use crate::profile::{ordinary_digest, Digest};
use crate::{
    CarrierError, M3Carrier, HIDDEN_POOL_MASK, PUBLIC_PREFIX_BYTES, PUBLIC_PREFIX_PAIR_COUNT,
    ROOT_ALIAS_KAT_BYTES, ROOT_ALIAS_KAT_EXPECTED_SHA256, ROOT_ALIAS_KAT_INPUT_SHA256,
    ROOT_ALIAS_KAT_PROFILE, SUPPORT_COUNT, VIEWER,
};

const PROJECTED_RESPONSE_MAGIC: &[u8; 8] = b"W42M3RP1";
const ROOT_ALIAS_VERSION: u32 = 1;
const RESPONSE_RECORD_BYTES: usize = 16;

const ROOT_ALIAS_KAT_INPUT: [u8; ROOT_ALIAS_KAT_BYTES] = decode_hex(
    b"5734324d3352413101000000800000000500000008000000b004000007000000090000000c0000009002100049aca40a011102180310001a000c0113020e030801020205030100160300000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
);

const ROOT_ALIAS_KAT_EXPECTED: [u8; ROOT_ALIAS_KAT_BYTES] = decode_hex(
    b"5734324d335252310100000080000000b0040000070000000900000003000000030000000100000001000000010000000100000010021000900010000d000000b982979b0fce873086e1b1125eae51425cb220d55762a5db68ef634dfb7b49a90000000000000000000000000000000000000000000000000000000000000000",
);

/// Semantic evidence produced while checking the fixed roots 31/33 alias KAT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RootAliasKatEvidence {
    pub response_record_count: u64,
    pub projected_response_digest: Digest,
}

pub const fn root_alias_kat_input_bytes() -> [u8; ROOT_ALIAS_KAT_BYTES] {
    ROOT_ALIAS_KAT_INPUT
}

pub const fn root_alias_kat_expected_bytes() -> [u8; ROOT_ALIAS_KAT_BYTES] {
    ROOT_ALIAS_KAT_EXPECTED
}

pub(crate) fn validate_root_alias_kat(
    carrier: &M3Carrier,
    input: &[u8],
    expected: &[u8],
) -> Result<RootAliasKatEvidence, CarrierError> {
    if input != ROOT_ALIAS_KAT_INPUT {
        return Err(CarrierError::Kat("fixed input bytes"));
    }
    if expected != ROOT_ALIAS_KAT_EXPECTED {
        return Err(CarrierError::Kat("fixed expected-result bytes"));
    }
    if ordinary_digest(input) != ROOT_ALIAS_KAT_INPUT_SHA256 {
        return Err(CarrierError::Kat("fixed input digest"));
    }
    if ordinary_digest(expected) != ROOT_ALIAS_KAT_EXPECTED_SHA256 {
        return Err(CarrierError::Kat("fixed expected-result digest"));
    }

    let root_a_index = get_u32(input, 28)?;
    let root_b_index = get_u32(input, 32)?;
    validate_input_fields(carrier, input, root_a_index, root_b_index)?;

    let root_a = domino_from_u32(root_a_index, "root A tile")?;
    let root_b = domino_from_u32(root_b_index, "root B tile")?;
    let context_a = carrier.facts().declaration.led_context(root_a);
    let context_b = carrier.facts().declaration.led_context(root_b);
    let context_a_code = context_code(context_a)?;
    let context_b_code = context_code(context_b)?;

    let remaining_a = remove_root(carrier.facts().viewer_hand, root_a)?;
    let remaining_b = remove_root(carrier.facts().viewer_hand, root_b)?;
    let first_h_a = first_future_h_key(remaining_a, root_a)?;
    let first_h_b = first_future_h_key(remaining_b, root_b)?;

    // Construct the two streams independently, including independent record
    // counts and closed-domain digests. Equality of hashes alone is not used
    // as equality of semantic payloads.
    let (payload_a, records_a) = projected_response_payload(carrier, root_a)?;
    let digest_a = projected_response_digest(records_a, &payload_a)?;
    let (payload_b, records_b) = projected_response_payload(carrier, root_b)?;
    let digest_b = projected_response_digest(records_b, &payload_b)?;

    let context_equal = context_a == context_b;
    let projected_response_equal =
        records_a == records_b && payload_a == payload_b && digest_a == digest_b;
    let task_frames_distinct = root_a_index != root_b_index;
    let first_h_keys_distinct = first_h_a != first_h_b;

    let mut derived = [0u8; ROOT_ALIAS_KAT_BYTES];
    derived[0..8].copy_from_slice(b"W42M3RR1");
    put_u32(&mut derived, 8, ROOT_ALIAS_VERSION);
    put_u32(&mut derived, 12, ROOT_ALIAS_KAT_BYTES as u32);
    put_u32(&mut derived, 16, carrier.support().len() as u32);
    put_u32(&mut derived, 20, root_a_index);
    put_u32(&mut derived, 24, root_b_index);
    put_u32(&mut derived, 28, context_a_code);
    put_u32(&mut derived, 32, context_b_code);
    put_u32(&mut derived, 36, u32::from(context_equal));
    put_u32(&mut derived, 40, u32::from(projected_response_equal));
    put_u32(&mut derived, 44, u32::from(task_frames_distinct));
    put_u32(&mut derived, 48, u32::from(first_h_keys_distinct));
    put_u32(&mut derived, 52, remaining_a.bits());
    put_u32(&mut derived, 56, remaining_b.bits());
    put_u32(
        &mut derived,
        60,
        u32::try_from(PUBLIC_PREFIX_PAIR_COUNT + 1)
            .map_err(|_| CarrierError::Kat("first-future pair-count width"))?,
    );
    derived[64..96].copy_from_slice(&ordinary_digest(input));

    if derived != ROOT_ALIAS_KAT_EXPECTED || derived.as_slice() != expected {
        return Err(CarrierError::Kat("derived expected-result record"));
    }

    Ok(RootAliasKatEvidence {
        response_record_count: records_a,
        projected_response_digest: digest_a,
    })
}

fn validate_input_fields(
    carrier: &M3Carrier,
    input: &[u8],
    root_a: u32,
    root_b: u32,
) -> Result<(), CarrierError> {
    kat_require(&input[0..8] == b"W42M3RA1", "input magic")?;
    kat_require(get_u32(input, 8)? == 1, "input version")?;
    kat_require(
        get_u32(input, 12)? == ROOT_ALIAS_KAT_BYTES as u32,
        "input byte count",
    )?;
    kat_require(get_u32(input, 16)? == 5, "input declaration")?;
    kat_require(get_u32(input, 20)? == 8, "input hand id")?;
    kat_require(
        get_u32(input, 24)? == SUPPORT_COUNT as u32 && carrier.support().len() == SUPPORT_COUNT,
        "input support count",
    )?;
    kat_require(root_a == 7 && root_b == 9, "input roots")?;
    kat_require(
        get_u32(input, 36)? == PUBLIC_PREFIX_PAIR_COUNT as u32,
        "input prefix pair count",
    )?;
    kat_require(
        get_u32(input, 40)? == carrier.facts().viewer_hand.bits(),
        "input viewer-hand mask",
    )?;
    kat_require(
        get_u32(input, 44)? == HIDDEN_POOL_MASK
            && HIDDEN_POOL_MASK == carrier.facts().hidden_pool.bits(),
        "input hidden-pool mask",
    )?;
    kat_require(
        input[48..72] == PUBLIC_PREFIX_BYTES,
        "input public-prefix payload",
    )?;
    kat_require(get_u32(input, 72)? == 3, "input expected context")?;
    kat_require(
        get_u32(input, 76)? == ROOT_ALIAS_KAT_PROFILE,
        "input KAT profile",
    )?;
    kat_require(input[80..].iter().all(|byte| *byte == 0), "input padding")?;
    Ok(())
}

fn projected_response_payload(
    carrier: &M3Carrier,
    root: Domino,
) -> Result<(Vec<u8>, u64), CarrierError> {
    kat_require(
        carrier.facts().viewer_hand.contains(root),
        "projected root membership",
    )?;
    let context = carrier.facts().declaration.led_context(root);
    let mut payload = Vec::new();
    let mut record_count = 0u64;

    for (source, world) in carrier.support().records().iter().copied().enumerate() {
        let legal_s2 = legal_plays(
            carrier.facts().declaration,
            world.hand(Seat::S2),
            Some(context),
        );
        let legal_s3 = legal_plays(
            carrier.facts().declaration,
            world.hand(Seat::S3),
            Some(context),
        );
        let legal_s0 = legal_plays(
            carrier.facts().declaration,
            world.hand(Seat::S0),
            Some(context),
        );
        let degrees = [
            u8::try_from(legal_s2.len()).map_err(|_| CarrierError::Kat("S2 legal degree width"))?,
            u8::try_from(legal_s3.len()).map_err(|_| CarrierError::Kat("S3 legal degree width"))?,
            u8::try_from(legal_s0.len()).map_err(|_| CarrierError::Kat("S0 legal degree width"))?,
        ];

        for response_s2 in legal_s2 {
            for response_s3 in legal_s3 {
                for response_s0 in legal_s0 {
                    let mut record = [0u8; RESPONSE_RECORD_BYTES];
                    record[0..4].copy_from_slice(
                        &u32::try_from(source)
                            .map_err(|_| CarrierError::Kat("response source width"))?
                            .to_le_bytes(),
                    );
                    record[4] = tile_index(response_s2)?;
                    record[5] = tile_index(response_s3)?;
                    record[6] = tile_index(response_s0)?;
                    record[8..11].copy_from_slice(&degrees);
                    payload.extend_from_slice(&record);
                    record_count =
                        record_count
                            .checked_add(1)
                            .ok_or(CarrierError::LengthOverflow(
                                "projected-response record count",
                            ))?;
                }
            }
        }
    }

    let expected_bytes = usize::try_from(record_count)
        .ok()
        .and_then(|records| records.checked_mul(RESPONSE_RECORD_BYTES))
        .ok_or(CarrierError::LengthOverflow(
            "projected-response payload bytes",
        ))?;
    kat_require(
        payload.len() == expected_bytes,
        "projected-response payload width",
    )?;
    Ok((payload, record_count))
}

fn projected_response_digest(record_count: u64, payload: &[u8]) -> Result<Digest, CarrierError> {
    let payload_bytes = u64::try_from(payload.len())
        .map_err(|_| CarrierError::LengthOverflow("projected-response payload bytes"))?;
    let mut state = Sha256State::new();
    state.update(PROJECTED_RESPONSE_MAGIC);
    state.update(&ROOT_ALIAS_VERSION.to_le_bytes());
    state.update(&record_count.to_le_bytes());
    state.update(&payload_bytes.to_le_bytes());
    state.update(payload);
    Ok(state.finish())
}

fn first_future_h_key(remaining: DominoSet, root: Domino) -> Result<Vec<u8>, CarrierError> {
    let pair_count = PUBLIC_PREFIX_PAIR_COUNT + 1;
    let mut key = Vec::with_capacity(7 + pair_count * 2);
    key.push(1);
    key.push(tile_index_for_seat(VIEWER)?);
    key.extend_from_slice(&remaining.bits().to_le_bytes());
    key.push(u8::try_from(pair_count).map_err(|_| CarrierError::Kat("H-key pair-count width"))?);
    key.extend_from_slice(&PUBLIC_PREFIX_BYTES);
    key.push(tile_index_for_seat(VIEWER)?);
    key.push(tile_index(root)?);
    Ok(key)
}

fn remove_root(mut hand: DominoSet, root: Domino) -> Result<DominoSet, CarrierError> {
    if !hand.remove(root) || hand.len() != 3 {
        return Err(CarrierError::Kat("remaining viewer hand"));
    }
    Ok(hand)
}

fn context_code(context: Context) -> Result<u32, CarrierError> {
    u32::try_from(context.index()).map_err(|_| CarrierError::Kat("context code width"))
}

fn domino_from_u32(index: u32, field: &'static str) -> Result<Domino, CarrierError> {
    let index = usize::try_from(index).map_err(|_| CarrierError::Kat(field))?;
    Domino::from_index(index).ok_or(CarrierError::Kat(field))
}

fn tile_index(domino: Domino) -> Result<u8, CarrierError> {
    u8::try_from(domino.index()).map_err(|_| CarrierError::Kat("tile-index width"))
}

fn tile_index_for_seat(seat: Seat) -> Result<u8, CarrierError> {
    u8::try_from(seat.index()).map_err(|_| CarrierError::Kat("seat-index width"))
}

fn get_u32(bytes: &[u8], offset: usize) -> Result<u32, CarrierError> {
    let end = offset
        .checked_add(4)
        .ok_or(CarrierError::Kat("u32 offset overflow"))?;
    let field = bytes
        .get(offset..end)
        .ok_or(CarrierError::Kat("truncated u32 field"))?;
    let field: [u8; 4] = field
        .try_into()
        .map_err(|_| CarrierError::Kat("u32 field width"))?;
    Ok(u32::from_le_bytes(field))
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn kat_require(condition: bool, field: &'static str) -> Result<(), CarrierError> {
    if condition {
        Ok(())
    } else {
        Err(CarrierError::Kat(field))
    }
}

const fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => panic!("invalid fixed KAT hexadecimal byte"),
    }
}

const fn decode_hex<const N: usize>(hex: &[u8]) -> [u8; N] {
    if hex.len() != N * 2 {
        panic!("wrong fixed KAT hexadecimal width");
    }
    let mut output = [0u8; N];
    let mut index = 0usize;
    while index < N {
        output[index] = (hex_nibble(hex[index * 2]) << 4) | hex_nibble(hex[index * 2 + 1]);
        index += 1;
    }
    output
}
