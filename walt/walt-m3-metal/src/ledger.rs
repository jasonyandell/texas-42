//! Exact freeze-57 MTLBuffer allocation ledger.

use crate::abi::{
    CONTROL_WORDS, FIELD_SLOT_WORDS, INPUT_RECORD_CAP, OUTPUT_SLOT_CAP, PARTICLE_WORDS,
    REDUCTION_PAIR_WORDS, REDUCTION_WORDS,
};
use crate::error::{M3MetalError, Result};

const WORD_BYTES: u64 = 4;
const GUARDED_INPUT_RECORDS: u64 = INPUT_RECORD_CAP as u64 + 2;
const GUARDED_OUTPUT_RECORDS: u64 = OUTPUT_SLOT_CAP as u64 + 2;

pub const CONTROL_BYTES: u64 = CONTROL_WORDS as u64 * WORD_BYTES;
pub const INPUT_ARENA_BYTES: u64 =
    GUARDED_INPUT_RECORDS * PARTICLE_WORDS as u64 * WORD_BYTES;
pub const OUTPUT_ARENA_BYTES: u64 =
    GUARDED_OUTPUT_RECORDS * FIELD_SLOT_WORDS as u64 * WORD_BYTES;
pub const REDUCTION_ARENA_BYTES: u64 =
    GUARDED_INPUT_RECORDS * REDUCTION_WORDS as u64 * WORD_BYTES;
pub const REDUCTION_ARENAS_BYTES: u64 = 2 * REDUCTION_ARENA_BYTES;
pub const REDUCTION_PLAN_BYTES: u64 =
    GUARDED_INPUT_RECORDS * REDUCTION_PAIR_WORDS as u64 * WORD_BYTES;
pub const LOGICAL_METAL_BYTES: u64 = CONTROL_BYTES
    + INPUT_ARENA_BYTES
    + OUTPUT_ARENA_BYTES
    + REDUCTION_ARENAS_BYTES
    + REDUCTION_PLAN_BYTES;
pub const METAL_LIVE_CAP_BYTES: u64 = 512 * 1024 * 1024;
pub const METAL_HEADROOM_BYTES: u64 = METAL_LIVE_CAP_BYTES - LOGICAL_METAL_BYTES;
pub const REQUIRED_MAXIMUM_BUFFER_BYTES: u64 = OUTPUT_ARENA_BYTES;
pub const REQUIRED_RECOMMENDED_WORKING_SET_BYTES: u64 = LOGICAL_METAL_BYTES;

const BUFFER_NAMES: [&str; 6] = [
    "control",
    "particle input",
    "field output",
    "reduction arena A",
    "reduction arena B",
    "reduction plan",
];

const LOGICAL_LENGTHS: [u64; 6] = [
    CONTROL_BYTES,
    INPUT_ARENA_BYTES,
    OUTPUT_ARENA_BYTES,
    REDUCTION_ARENA_BYTES,
    REDUCTION_ARENA_BYTES,
    REDUCTION_PLAN_BYTES,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportedAllocations {
    bytes: [u64; 6],
}

impl ReportedAllocations {
    pub const fn new(
        control: u64,
        input: u64,
        output: u64,
        reduction_a: u64,
        reduction_b: u64,
        plan: u64,
    ) -> Self {
        Self {
            bytes: [control, input, output, reduction_a, reduction_b, plan],
        }
    }

    pub const fn exact_logical() -> Self {
        Self {
            bytes: LOGICAL_LENGTHS,
        }
    }

    pub const fn bytes(self) -> [u64; 6] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AllocationAdmission {
    logical_bytes: u64,
    charged_reported_bytes: u64,
    maximum_single_buffer: u64,
    recommended_working_set: u64,
    headroom_bytes: u64,
}

impl AllocationAdmission {
    pub fn admit(
        device_maximum_buffer: u64,
        recommended_working_set: u64,
        reported: ReportedAllocations,
    ) -> Result<Self> {
        if device_maximum_buffer < REQUIRED_MAXIMUM_BUFFER_BYTES {
            return Err(M3MetalError::DeviceCapacity {
                field: "maximum buffer length",
                required: REQUIRED_MAXIMUM_BUFFER_BYTES,
                observed: device_maximum_buffer,
            });
        }
        if recommended_working_set < REQUIRED_RECOMMENDED_WORKING_SET_BYTES {
            return Err(M3MetalError::DeviceCapacity {
                field: "recommended working set",
                required: REQUIRED_RECOMMENDED_WORKING_SET_BYTES,
                observed: recommended_working_set,
            });
        }

        let mut charged = 0u64;
        for index in 0..LOGICAL_LENGTHS.len() {
            let required = LOGICAL_LENGTHS[index];
            let observed = reported.bytes[index];
            if observed < required {
                return Err(M3MetalError::AllocationTooShort {
                    buffer: BUFFER_NAMES[index],
                    required,
                    reported: observed,
                });
            }
            charged = charged
                .checked_add(observed)
                .ok_or(M3MetalError::LengthOverflow(
                    "reported MTLBuffer allocation sum",
                ))?;
        }
        if charged > METAL_LIVE_CAP_BYTES {
            return Err(M3MetalError::CapExceeded {
                cap: "live MTLBuffer bytes",
                limit: METAL_LIVE_CAP_BYTES,
                observed: charged,
            });
        }
        let headroom_bytes = METAL_LIVE_CAP_BYTES
            .checked_sub(charged)
            .ok_or(M3MetalError::LengthOverflow("Metal cap headroom"))?;
        Ok(Self {
            logical_bytes: LOGICAL_METAL_BYTES,
            charged_reported_bytes: charged,
            maximum_single_buffer: device_maximum_buffer,
            recommended_working_set,
            headroom_bytes,
        })
    }

    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn charged_reported_bytes(self) -> u64 {
        self.charged_reported_bytes
    }

    pub const fn maximum_single_buffer(self) -> u64 {
        self.maximum_single_buffer
    }

    pub const fn recommended_working_set(self) -> u64 {
        self.recommended_working_set
    }

    pub const fn headroom_bytes(self) -> u64 {
        self.headroom_bytes
    }
}

const _: () = assert!(
    CONTROL_BYTES
        + INPUT_ARENA_BYTES
        + OUTPUT_ARENA_BYTES
        + REDUCTION_ARENAS_BYTES
        + REDUCTION_PLAN_BYTES
        == LOGICAL_METAL_BYTES
);
const _: () = assert!(METAL_LIVE_CAP_BYTES - LOGICAL_METAL_BYTES == METAL_HEADROOM_BYTES);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_freeze57_formula_and_headroom_are_admitted() {
        let admission = AllocationAdmission::admit(
            REQUIRED_MAXIMUM_BUFFER_BYTES,
            REQUIRED_RECOMMENDED_WORKING_SET_BYTES,
            ReportedAllocations::exact_logical(),
        )
        .unwrap();
        assert_eq!(admission.logical_bytes(), 520_094_400);
        assert_eq!(admission.charged_reported_bytes(), 520_094_400);
        assert_eq!(admission.headroom_bytes(), 16_776_512);
    }

    #[test]
    fn driver_rounding_is_charged_and_cannot_cross_the_cap() {
        let admitted = ReportedAllocations::new(
            CONTROL_BYTES + 64,
            INPUT_ARENA_BYTES,
            OUTPUT_ARENA_BYTES,
            REDUCTION_ARENA_BYTES,
            REDUCTION_ARENA_BYTES,
            REDUCTION_PLAN_BYTES,
        );
        assert_eq!(
            AllocationAdmission::admit(
                REQUIRED_MAXIMUM_BUFFER_BYTES,
                REQUIRED_RECOMMENDED_WORKING_SET_BYTES,
                admitted,
            )
            .unwrap()
            .charged_reported_bytes(),
            LOGICAL_METAL_BYTES + 64
        );

        let excessive = ReportedAllocations::new(
            CONTROL_BYTES,
            INPUT_ARENA_BYTES,
            OUTPUT_ARENA_BYTES + METAL_HEADROOM_BYTES + 1,
            REDUCTION_ARENA_BYTES,
            REDUCTION_ARENA_BYTES,
            REDUCTION_PLAN_BYTES,
        );
        assert!(matches!(
            AllocationAdmission::admit(u64::MAX, u64::MAX, excessive,),
            Err(M3MetalError::CapExceeded { .. })
        ));
    }

    #[test]
    fn device_and_each_reported_buffer_are_checked_before_admission() {
        assert!(matches!(
            AllocationAdmission::admit(
                REQUIRED_MAXIMUM_BUFFER_BYTES - 1,
                u64::MAX,
                ReportedAllocations::exact_logical(),
            ),
            Err(M3MetalError::DeviceCapacity { .. })
        ));
        assert!(matches!(
            AllocationAdmission::admit(
                u64::MAX,
                REQUIRED_RECOMMENDED_WORKING_SET_BYTES - 1,
                ReportedAllocations::exact_logical(),
            ),
            Err(M3MetalError::DeviceCapacity { .. })
        ));
        let short = ReportedAllocations::new(
            CONTROL_BYTES,
            INPUT_ARENA_BYTES,
            OUTPUT_ARENA_BYTES,
            REDUCTION_ARENA_BYTES,
            REDUCTION_ARENA_BYTES - 1,
            REDUCTION_PLAN_BYTES,
        );
        assert!(matches!(
            AllocationAdmission::admit(u64::MAX, u64::MAX, short),
            Err(M3MetalError::AllocationTooShort {
                buffer: "reduction arena B",
                ..
            })
        ));
    }
}
