use walt_gpu_spec::{sha256, Sha256State};

use crate::{
    CarrierError, CarrierSupport, BELIEF_PROFILE_UNIFORM_COMPATIBLE_SUPPORT, CARRIER_HAND_ID,
    CARRIER_PROFILE_BYTES, CARRIER_TRICK, FIELD_LAW_UNIFORM_RANDOM_LEGAL,
    HISTORY_PROFILE_HISTORICAL_VOID_FEASIBILITY_ONLY, LEGAL_ROOT_MASK, RAW_RECEIPT_BYTES,
    RAW_RECEIPT_SHA256, STREAM_PURPOSE_SUPPORT, SUPPORT_COUNT, VIEWER,
};

pub type Digest = [u8; 32];

const STREAM_MAGIC: &[u8; 8] = b"W42M3DG1";
const STREAM_VERSION: u32 = 1;

pub fn stream_digest(purpose: u32, record_count: u64, payload: &[u8]) -> Digest {
    let mut digest = Sha256State::new();
    digest.update(STREAM_MAGIC);
    digest.update(&purpose.to_le_bytes());
    digest.update(&STREAM_VERSION.to_le_bytes());
    digest.update(&record_count.to_le_bytes());
    digest.update(&(payload.len() as u64).to_le_bytes());
    digest.update(payload);
    digest.finish()
}

pub fn carrier_profile_bytes(support: &CarrierSupport) -> [u8; CARRIER_PROFILE_BYTES] {
    let mut bytes = [0u8; CARRIER_PROFILE_BYTES];
    bytes[0..8].copy_from_slice(b"W42M3CP1");
    put_u32(&mut bytes, 8, 1);
    put_u32(&mut bytes, 12, CARRIER_PROFILE_BYTES as u32);
    put_u64(&mut bytes, 16, RAW_RECEIPT_BYTES as u64);
    bytes[24..56].copy_from_slice(&RAW_RECEIPT_SHA256);
    put_u32(&mut bytes, 56, CARRIER_HAND_ID as u32);
    put_u32(&mut bytes, 60, CARRIER_TRICK as u32);
    put_u32(&mut bytes, 64, VIEWER.index() as u32);
    put_u32(&mut bytes, 68, SUPPORT_COUNT as u32);
    put_u32(&mut bytes, 72, LEGAL_ROOT_MASK);
    put_u32(&mut bytes, 76, BELIEF_PROFILE_UNIFORM_COMPATIBLE_SUPPORT);
    put_u32(
        &mut bytes,
        80,
        HISTORY_PROFILE_HISTORICAL_VOID_FEASIBILITY_ONLY,
    );
    put_u32(&mut bytes, 84, FIELD_LAW_UNIFORM_RANDOM_LEGAL);
    bytes[88..120].copy_from_slice(&support.digest());
    bytes
}

pub(crate) fn support_digest(payload: &[u8]) -> Result<Digest, CarrierError> {
    if !payload.len().is_multiple_of(crate::SUPPORT_RECORD_BYTES) {
        return Err(CarrierError::FrozenFact("support payload record width"));
    }
    let records = payload
        .len()
        .checked_div(crate::SUPPORT_RECORD_BYTES)
        .ok_or(CarrierError::LengthOverflow("support record count"))?;
    let records =
        u64::try_from(records).map_err(|_| CarrierError::LengthOverflow("support record count"))?;
    Ok(stream_digest(STREAM_PURPOSE_SUPPORT, records, payload))
}

pub(crate) fn ordinary_digest(bytes: &[u8]) -> Digest {
    sha256(bytes)
}

fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
