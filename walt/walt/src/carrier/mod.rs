//! Frozen carrier construction for the freeze-57 M3 perfect-recall gate.
//!
//! This crate admits only the exact hand-8 receipt carrier.  It owns no value,
//! policy, information-state, grouping, or Metal code.  Two structurally
//! independent support constructors must agree byte-for-byte before a carrier
//! is returned.

#![forbid(unsafe_code)]

mod constants;
mod error;
mod kat;
mod profile;
mod replay;
mod support;

pub use constants::*;
pub use error::CarrierError;
pub use kat::{root_alias_kat_expected_bytes, root_alias_kat_input_bytes, RootAliasKatEvidence};
pub use profile::{carrier_profile_bytes, stream_digest, Digest};
pub use replay::{CarrierFacts, PUBLIC_PREFIX_BYTES};
pub use support::{CarrierSupport, ConstrainedSupportIter, SupportRecord};

use crate::spec::sha256;

/// The sole admitted freeze-57 carrier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M3Carrier {
    facts: CarrierFacts,
    support: CarrierSupport,
}

impl M3Carrier {
    /// Verifies and admits the exact raw `verify_player.txt` bytes.
    pub fn from_receipt_bytes(bytes: &[u8]) -> Result<Self, CarrierError> {
        if bytes.len() != RAW_RECEIPT_BYTES {
            return Err(CarrierError::ReceiptLength {
                expected: RAW_RECEIPT_BYTES,
                actual: bytes.len(),
            });
        }
        let actual = sha256(bytes);
        if actual != RAW_RECEIPT_SHA256 {
            return Err(CarrierError::ReceiptDigest { actual });
        }

        let facts = replay::primary_replay_facts(bytes)?;
        let primary = support::primary_filtered_support(&facts)?;
        let independent = support::independent_constrained_support(&facts)?;
        primary.require_identical(&independent)?;
        primary.validate(&facts)?;
        if primary.digest() != SUPPORT_STREAM_DIGEST {
            return Err(CarrierError::FrozenFact("support stream digest"));
        }

        let carrier = Self {
            facts,
            support: primary,
        };
        if carrier.public_prefix_digest() != PUBLIC_PREFIX_STREAM_DIGEST {
            return Err(CarrierError::FrozenFact("public-prefix stream digest"));
        }
        if carrier.carrier_profile_digest() != CARRIER_PROFILE_SHA256 {
            return Err(CarrierError::FrozenFact("carrier-profile digest"));
        }
        let kat = carrier.validate_root_alias_kat(
            &root_alias_kat_input_bytes(),
            &root_alias_kat_expected_bytes(),
        )?;
        if kat.response_record_count != ROOT_ALIAS_PROJECTED_RESPONSE_RECORDS
            || kat.projected_response_digest != ROOT_ALIAS_PROJECTED_RESPONSE_DIGEST
        {
            return Err(CarrierError::FrozenFact(
                "root-alias projected-response stream",
            ));
        }
        Ok(carrier)
    }

    pub const fn facts(&self) -> &CarrierFacts {
        &self.facts
    }

    pub const fn support(&self) -> &CarrierSupport {
        &self.support
    }

    pub fn public_prefix_digest(&self) -> Digest {
        stream_digest(
            STREAM_PURPOSE_PUBLIC_PREFIX,
            PUBLIC_PREFIX_PAIR_COUNT as u64,
            &PUBLIC_PREFIX_BYTES,
        )
    }

    pub fn carrier_profile_bytes(&self) -> [u8; CARRIER_PROFILE_BYTES] {
        carrier_profile_bytes(&self.support)
    }

    pub fn carrier_profile_digest(&self) -> Digest {
        sha256(&self.carrier_profile_bytes())
    }

    pub fn validate_root_alias_kat(
        &self,
        input: &[u8],
        expected: &[u8],
    ) -> Result<RootAliasKatEvidence, CarrierError> {
        kat::validate_root_alias_kat(self, input, expected)
    }
}
