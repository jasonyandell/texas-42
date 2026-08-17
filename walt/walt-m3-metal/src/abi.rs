//! Frozen word-level ABI for the two M3 Metal entry points.
//!
//! The C-layout records are deliberately crate-private.  Public construction
//! and inspection happen through validated word wrappers, and serialization is
//! always explicit little-endian words rather than Rust struct memory.

use core::mem::{align_of, size_of, size_of_val};

use crate::error::{M3MetalError, Result};

pub const ABI_VERSION: u32 = 1;
pub const OPCODE_EXPAND: u32 = 1;
pub const OPCODE_REDUCE: u32 = 2;
pub const POISON_WORD: u32 = 0xa5a5_5a5a;
pub const INVALID_TILE: u32 = u32::MAX;

pub const CONTROL_WORDS: usize = 16;
pub const PARTICLE_WORDS: usize = 24;
pub const FIELD_SLOT_WORDS: usize = 28;
pub const REDUCTION_WORDS: usize = 12;
pub const REDUCTION_PAIR_WORDS: usize = 4;

pub const TASK_COUNT: u32 = 8;
pub const OBJECTIVE_A: u32 = 1;
pub const OBJECTIVE_B: u32 = 2;
pub const TREATMENT_H: u32 = 1;
pub const TREATMENT_C: u32 = 2;
pub const TASK_OBJECTIVES: [u32; TASK_COUNT as usize] = [
    OBJECTIVE_A,
    OBJECTIVE_A,
    OBJECTIVE_A,
    OBJECTIVE_A,
    OBJECTIVE_B,
    OBJECTIVE_B,
    OBJECTIVE_B,
    OBJECTIVE_B,
];
pub const TASK_PHYSICAL_ROOTS: [u32; TASK_COUNT as usize] = [4, 7, 9, 20, 4, 7, 9, 20];
pub const SUPPORT_WORLD_COUNT: u32 = 1_200;
pub const FUTURE_FIELD_MOVES: u32 = 12;
pub const PHYSICAL_TILE_COUNT: u32 = 28;
pub const LEGAL_SLOT_COUNT: u32 = 7;
pub const THREADGROUP_WIDTH: usize = 32;

pub const INPUT_RECORD_CAP: u32 = 524_288;
pub const OUTPUT_SLOT_CAP: u32 = 3_670_016;
pub const MAX_REDUCTION_LEVELS: u32 = 21;
pub const TASK_COMMAND_CAP: u32 = 32_768;

pub const PARTICLE_STATUS_VALID: u32 = 1;
pub const FIELD_STATUS_EMPTY: u32 = 0;
pub const FIELD_STATUS_VALID: u32 = 1;
pub const FIELD_STATUS_HARD: u32 = 2;
pub const REDUCTION_STATUS_VALID: u32 = 1;
pub const REDUCTION_STATUS_HARD: u32 = 2;

pub const FIELD_BAD_ABI: u32 = 1;
pub const FIELD_BAD_TASK: u32 = 2;
pub const FIELD_BAD_OBJECTIVE: u32 = 3;
pub const FIELD_BAD_TREATMENT: u32 = 4;
pub const FIELD_BAD_MASK: u32 = 5;
pub const FIELD_BAD_PACKED_STATE: u32 = 6;
pub const FIELD_BAD_ACTOR: u32 = 7;
pub const FIELD_BAD_LEGAL_DEGREE: u32 = 8;
pub const FIELD_BAD_TILE: u32 = 9;
pub const FIELD_REPEATED_TILE: u32 = 10;
pub const FIELD_MASS_OVERFLOW: u32 = 11;
pub const FIELD_BAD_EXPONENT: u32 = 12;
pub const FIELD_TRICK_RESOLUTION: u32 = 13;
pub const FIELD_LOSS_RANGE: u32 = 14;
pub const FIELD_SLOT_RANGE: u32 = 15;
pub const FIELD_INTERNAL: u32 = 16;
pub const FIELD_ERROR_PRECEDENCE: [u32; 16] = [
    FIELD_BAD_ABI,
    FIELD_BAD_TASK,
    FIELD_BAD_OBJECTIVE,
    FIELD_BAD_TREATMENT,
    FIELD_BAD_MASK,
    FIELD_BAD_PACKED_STATE,
    FIELD_BAD_ACTOR,
    FIELD_BAD_LEGAL_DEGREE,
    FIELD_BAD_TILE,
    FIELD_REPEATED_TILE,
    FIELD_MASS_OVERFLOW,
    FIELD_BAD_EXPONENT,
    FIELD_TRICK_RESOLUTION,
    FIELD_LOSS_RANGE,
    FIELD_SLOT_RANGE,
    FIELD_INTERNAL,
];

pub const REDUCTION_BAD_ABI: u32 = 1;
pub const REDUCTION_BAD_STATUS: u32 = 2;
pub const REDUCTION_BAD_ROW_ORDINAL: u32 = 3;
pub const REDUCTION_BAD_PLAN_INDEX: u32 = 4;
pub const REDUCTION_BAD_CARRY_FORM: u32 = 5;
pub const REDUCTION_ADD_OVERFLOW: u32 = 6;
pub const REDUCTION_INTERNAL: u32 = 7;
pub const REDUCTION_ERROR_PRECEDENCE: [u32; 7] = [
    REDUCTION_BAD_ABI,
    REDUCTION_BAD_STATUS,
    REDUCTION_BAD_ROW_ORDINAL,
    REDUCTION_BAD_PLAN_INDEX,
    REDUCTION_BAD_CARRY_FORM,
    REDUCTION_ADD_OVERFLOW,
    REDUCTION_INTERNAL,
];

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M3KernelControlV1 {
    words: [u32; CONTROL_WORDS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M3ParticleAbiV1 {
    words: [u32; PARTICLE_WORDS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M3FieldSlotAbiV1 {
    words: [u32; FIELD_SLOT_WORDS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M3ReductionAbiV1 {
    words: [u32; REDUCTION_WORDS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M3ReductionPairV1 {
    words: [u32; REDUCTION_PAIR_WORDS],
}

const _: [(); 64] = [(); size_of::<M3KernelControlV1>()];
const _: [(); 4] = [(); align_of::<M3KernelControlV1>()];
const _: [(); 96] = [(); size_of::<M3ParticleAbiV1>()];
const _: [(); 4] = [(); align_of::<M3ParticleAbiV1>()];
const _: [(); 112] = [(); size_of::<M3FieldSlotAbiV1>()];
const _: [(); 4] = [(); align_of::<M3FieldSlotAbiV1>()];
const _: [(); 48] = [(); size_of::<M3ReductionAbiV1>()];
const _: [(); 4] = [(); align_of::<M3ReductionAbiV1>()];
const _: [(); 16] = [(); size_of::<M3ReductionPairV1>()];
const _: [(); 4] = [(); align_of::<M3ReductionPairV1>()];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlWords(M3KernelControlV1);

impl ControlWords {
    #[allow(clippy::too_many_arguments)]
    pub fn field(
        command: u32,
        task: u32,
        objective: u32,
        treatment: u32,
        input_count: u32,
        field_exponent: u32,
        physical_root: u32,
    ) -> Result<Self> {
        let output_count = input_count
            .checked_mul(LEGAL_SLOT_COUNT)
            .ok_or(M3MetalError::LengthOverflow("field output count"))?;
        Self::try_from_words([
            ABI_VERSION,
            OPCODE_EXPAND,
            command,
            task,
            objective,
            treatment,
            input_count,
            field_exponent,
            output_count,
            physical_root,
            0,
            0,
            0,
            0,
            0,
            0,
        ])
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn reduction(
        command: u32,
        task: u32,
        objective: u32,
        treatment: u32,
        plan_count: u32,
        source_row_base: u32,
        destination_row_base: u32,
        level: u32,
        source_count: u32,
        range_count: u32,
    ) -> Result<Self> {
        Self::try_from_words([
            ABI_VERSION,
            OPCODE_REDUCE,
            command,
            task,
            objective,
            treatment,
            plan_count,
            source_row_base,
            plan_count,
            destination_row_base,
            level,
            source_count,
            plan_count,
            range_count,
            0,
            0,
        ])
    }

    pub fn try_from_slice(words: &[u32]) -> Result<Self> {
        let words: [u32; CONTROL_WORDS] =
            words.try_into().map_err(|_| M3MetalError::WrongWordCount {
                record: "M3KernelControlV1",
                expected: CONTROL_WORDS,
                actual: words.len(),
            })?;
        Self::try_from_words(words)
    }

    pub fn try_from_words(words: [u32; CONTROL_WORDS]) -> Result<Self> {
        require_word(
            "M3KernelControlV1",
            &words,
            0,
            ABI_VERSION,
            "ABI must be one",
        )?;
        require_task(words[3], "M3KernelControlV1", 3)?;
        require_objective(words[4], "M3KernelControlV1", 4)?;
        if words[4] != task_objective(words[3]) {
            return invalid_word(
                "M3KernelControlV1",
                4,
                "objective must match the frozen task ordinal",
            );
        }
        require_treatment(words[5], "M3KernelControlV1", 5)?;
        if words[2] >= TASK_COMMAND_CAP {
            return invalid_word(
                "M3KernelControlV1",
                2,
                "command ordinal exceeds the per-task namespace",
            );
        }
        match words[1] {
            OPCODE_EXPAND => validate_field_control(&words)?,
            OPCODE_REDUCE => validate_reduction_control(&words)?,
            _ => return invalid_word("M3KernelControlV1", 1, "opcode must be EXPAND or REDUCE"),
        }
        Ok(Self(M3KernelControlV1 { words }))
    }

    pub const fn words(&self) -> &[u32; CONTROL_WORDS] {
        &self.0.words
    }

    pub fn append_le_bytes(&self, output: &mut Vec<u8>) {
        append_words_le(output, &self.0.words);
    }

    pub const fn opcode(&self) -> u32 {
        self.0.words[1]
    }

    pub const fn command_ordinal(&self) -> u32 {
        self.0.words[2]
    }

    pub const fn logical_output_count(&self) -> u32 {
        self.0.words[8]
    }
}

fn validate_field_control(words: &[u32; CONTROL_WORDS]) -> Result<()> {
    require_nonzero_cap(
        "M3KernelControlV1",
        6,
        words[6],
        INPUT_RECORD_CAP,
        "field input count",
    )?;
    if words[7] >= FUTURE_FIELD_MOVES {
        return invalid_word(
            "M3KernelControlV1",
            7,
            "field exponent before EXPAND must be 0..11",
        );
    }
    let expected_output = words[6]
        .checked_mul(LEGAL_SLOT_COUNT)
        .ok_or(M3MetalError::LengthOverflow("field output count"))?;
    if words[8] != expected_output || words[8] > OUTPUT_SLOT_CAP {
        return invalid_word(
            "M3KernelControlV1",
            8,
            "EXPAND output count must equal seven times input count",
        );
    }
    if words[9] != task_physical_root(words[3]) {
        return invalid_word(
            "M3KernelControlV1",
            9,
            "physical root index must match the frozen task ordinal",
        );
    }
    require_zero_range("M3KernelControlV1", words, 10..16)
}

pub const fn task_objective(task: u32) -> u32 {
    if task < 4 {
        OBJECTIVE_A
    } else if task < TASK_COUNT {
        OBJECTIVE_B
    } else {
        0
    }
}

pub const fn task_physical_root(task: u32) -> u32 {
    match task {
        0 | 4 => 4,
        1 | 5 => 7,
        2 | 6 => 9,
        3 | 7 => 20,
        _ => INVALID_TILE,
    }
}

fn validate_reduction_control(words: &[u32; CONTROL_WORDS]) -> Result<()> {
    require_nonzero_cap(
        "M3KernelControlV1",
        6,
        words[6],
        INPUT_RECORD_CAP,
        "reduction plan count",
    )?;
    if words[8] != words[6] || words[12] != words[6] {
        return invalid_word(
            "M3KernelControlV1",
            8,
            "REDUCE plan, logical output, and destination counts must agree",
        );
    }
    require_nonzero_cap(
        "M3KernelControlV1",
        11,
        words[11],
        INPUT_RECORD_CAP,
        "reduction source count",
    )?;
    require_nonzero_cap(
        "M3KernelControlV1",
        13,
        words[13],
        INPUT_RECORD_CAP,
        "reduction range count",
    )?;
    if words[6] > words[11] || words[13] > words[6] {
        return invalid_word(
            "M3KernelControlV1",
            6,
            "REDUCE counts violate range <= destination <= source",
        );
    }
    if words[10] >= MAX_REDUCTION_LEVELS {
        return invalid_word("M3KernelControlV1", 10, "reduction level must be 0..20");
    }
    words[7]
        .checked_add(words[11])
        .ok_or(M3MetalError::LengthOverflow("reduction source row base"))?;
    words[9]
        .checked_add(words[12])
        .ok_or(M3MetalError::LengthOverflow(
            "reduction destination row base",
        ))?;
    require_zero_range("M3KernelControlV1", words, 14..16)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackedState {
    raw: u32,
}

impl PackedState {
    pub fn try_from_raw(raw: u32) -> Result<Self> {
        if raw >> 22 != 0 {
            return invalid_word("M3 packed state", 0, "bits 22..31 must be zero");
        }
        let value = Self { raw };
        if value.trick_length() > 3 {
            return invalid_word("M3 packed state", 0, "live trick length must be 0..3");
        }
        if value.completed_future_tricks() > 4 {
            return invalid_word("M3 packed state", 0, "completed-future-tricks must be 0..4");
        }
        if value.future_t1_wins() > value.completed_future_tricks() {
            return invalid_word(
                "M3 packed state",
                0,
                "future-T1-wins exceeds completed-future-tricks",
            );
        }
        if value.field_exponent() > FUTURE_FIELD_MOVES {
            return invalid_word("M3 packed state", 0, "field exponent must be 0..12");
        }
        if value.completed_future_tricks() == 4 && value.trick_length() != 0 {
            return invalid_word(
                "M3 packed state",
                0,
                "a terminal future horizon may not retain a partial trick",
            );
        }
        if value.complete_record_length() != value.expected_complete_record_length() {
            return invalid_word(
                "M3 packed state",
                0,
                "complete-record length must equal 12 + 4*completed + trick length",
            );
        }
        Ok(value)
    }

    pub const fn raw(self) -> u32 {
        self.raw
    }

    pub const fn leader(self) -> u32 {
        self.raw & 0x3
    }

    pub const fn next_actor(self) -> u32 {
        (self.raw >> 2) & 0x3
    }

    pub const fn trick_length(self) -> u32 {
        (self.raw >> 4) & 0x7
    }

    pub const fn completed_future_tricks(self) -> u32 {
        (self.raw >> 7) & 0x7
    }

    pub const fn future_t1_wins(self) -> u32 {
        (self.raw >> 10) & 0x7
    }

    pub const fn field_exponent(self) -> u32 {
        (self.raw >> 13) & 0xf
    }

    pub const fn complete_record_length(self) -> u32 {
        (self.raw >> 17) & 0x1f
    }

    pub const fn expected_complete_record_length(self) -> u32 {
        12 + 4 * self.completed_future_tricks() + self.trick_length()
    }

    pub fn expected_field_exponent(self) -> u32 {
        let mut focal_plays = 0;
        let mut lane = 0;
        while lane < self.trick_length() {
            if (self.leader() + lane) & 3 == 1 {
                focal_plays += 1;
            }
            lane += 1;
        }
        3 * self.completed_future_tricks() + self.trick_length() - focal_plays
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParticleWords(M3ParticleAbiV1);

impl ParticleWords {
    pub fn try_from_slice(words: &[u32]) -> Result<Self> {
        let words: [u32; PARTICLE_WORDS] =
            words.try_into().map_err(|_| M3MetalError::WrongWordCount {
                record: "M3ParticleAbiV1",
                expected: PARTICLE_WORDS,
                actual: words.len(),
            })?;
        Self::try_from_words(words)
    }

    pub fn try_from_words(words: [u32; PARTICLE_WORDS]) -> Result<Self> {
        require_word("M3ParticleAbiV1", &words, 0, ABI_VERSION, "ABI must be one")?;
        require_word(
            "M3ParticleAbiV1",
            &words,
            1,
            PARTICLE_STATUS_VALID,
            "status must be VALID",
        )?;
        require_task(words[2], "M3ParticleAbiV1", 2)?;
        require_source(words[3], "M3ParticleAbiV1", 3)?;
        validate_hands("M3ParticleAbiV1", &words[4..8], 4)?;
        let packed = PackedState::try_from_raw(words[8])?;
        validate_packed_trick("M3ParticleAbiV1", words[9], packed.trick_length(), 9)?;
        validate_state_hands("M3ParticleAbiV1", &words[4..8], words[9], packed, 4)?;
        if words[10] > 34 {
            return invalid_word(
                "M3ParticleAbiV1",
                10,
                "future T0 spend must remain in the unsaturated range 0..34",
            );
        }
        if words[11] == u32::MAX {
            return invalid_word(
                "M3ParticleAbiV1",
                11,
                "host record ordinal may not use the unavailable sentinel",
            );
        }
        if words[12..20].iter().all(|word| *word == 0) {
            return invalid_word(
                "M3ParticleAbiV1",
                12,
                "a production particle must carry positive mass",
            );
        }
        require_zero_range("M3ParticleAbiV1", &words, 20..24)?;
        Ok(Self(M3ParticleAbiV1 { words }))
    }

    pub const fn words(&self) -> &[u32; PARTICLE_WORDS] {
        &self.0.words
    }

    pub fn append_le_bytes(&self, output: &mut Vec<u8>) {
        append_words_le(output, &self.0.words);
    }

    pub const fn task_ordinal(&self) -> u32 {
        self.0.words[2]
    }

    pub const fn source_ordinal(&self) -> u32 {
        self.0.words[3]
    }

    pub const fn host_record_ordinal(&self) -> u32 {
        self.0.words[11]
    }

    pub fn packed_state(&self) -> PackedState {
        PackedState {
            raw: self.0.words[8],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldSlotWords(M3FieldSlotAbiV1);

impl FieldSlotWords {
    pub fn canonical_empty(
        parent: u32,
        slot: u32,
        legal_degree: u32,
        task: u32,
        source: u32,
    ) -> Result<Self> {
        let mut words = [0u32; FIELD_SLOT_WORDS];
        words[0] = ABI_VERSION;
        words[1] = FIELD_STATUS_EMPTY;
        words[3] = parent;
        words[4] = slot;
        words[5] = legal_degree;
        words[6] = INVALID_TILE;
        words[7] = task;
        words[8] = source;
        Self::try_from_words(words)
    }

    pub fn canonical_hard(
        error: u32,
        parent: u32,
        slot: u32,
        task: u32,
        source: u32,
    ) -> Result<Self> {
        let mut words = [0u32; FIELD_SLOT_WORDS];
        words[0] = ABI_VERSION;
        words[1] = FIELD_STATUS_HARD;
        words[2] = error;
        words[3] = parent;
        words[4] = slot;
        words[6] = INVALID_TILE;
        words[7] = task;
        words[8] = source;
        Self::try_from_words(words)
    }

    pub fn try_from_slice(words: &[u32]) -> Result<Self> {
        let words: [u32; FIELD_SLOT_WORDS] =
            words.try_into().map_err(|_| M3MetalError::WrongWordCount {
                record: "M3FieldSlotAbiV1",
                expected: FIELD_SLOT_WORDS,
                actual: words.len(),
            })?;
        Self::try_from_words(words)
    }

    pub fn try_from_words(words: [u32; FIELD_SLOT_WORDS]) -> Result<Self> {
        require_word(
            "M3FieldSlotAbiV1",
            &words,
            0,
            ABI_VERSION,
            "ABI must be one",
        )?;
        if words[4] >= LEGAL_SLOT_COUNT {
            return invalid_word("M3FieldSlotAbiV1", 4, "slot ordinal must be 0..6");
        }
        match words[1] {
            FIELD_STATUS_EMPTY => {
                validate_field_provenance(&words)?;
                validate_empty_slot(&words)?;
            }
            FIELD_STATUS_VALID => {
                validate_field_provenance(&words)?;
                validate_valid_slot(&words)?;
            }
            FIELD_STATUS_HARD => validate_hard_slot(&words)?,
            _ => {
                return invalid_word(
                    "M3FieldSlotAbiV1",
                    1,
                    "status must be EMPTY, VALID, or HARD",
                )
            }
        }
        Ok(Self(M3FieldSlotAbiV1 { words }))
    }

    pub const fn words(&self) -> &[u32; FIELD_SLOT_WORDS] {
        &self.0.words
    }

    pub fn append_le_bytes(&self, output: &mut Vec<u8>) {
        append_words_le(output, &self.0.words);
    }

    pub const fn status(&self) -> u32 {
        self.0.words[1]
    }

    pub const fn error(&self) -> u32 {
        self.0.words[2]
    }
}

fn validate_field_provenance(words: &[u32; FIELD_SLOT_WORDS]) -> Result<()> {
    require_task(words[7], "M3FieldSlotAbiV1", 7)?;
    require_source(words[8], "M3FieldSlotAbiV1", 8)?;
    if words[3] == u32::MAX {
        return invalid_word(
            "M3FieldSlotAbiV1",
            3,
            "parent input ordinal may not use the unavailable sentinel",
        );
    }
    Ok(())
}

fn validate_empty_slot(words: &[u32; FIELD_SLOT_WORDS]) -> Result<()> {
    if words[2] != 0 {
        return invalid_word("M3FieldSlotAbiV1", 2, "EMPTY error must be zero");
    }
    require_legal_degree(words[5], "M3FieldSlotAbiV1", 5)?;
    if words[4] < words[5] {
        return invalid_word(
            "M3FieldSlotAbiV1",
            4,
            "an EMPTY slot must follow every densely packed VALID slot",
        );
    }
    require_word(
        "M3FieldSlotAbiV1",
        words,
        6,
        INVALID_TILE,
        "EMPTY selected tile must be invalid",
    )?;
    require_zero_range("M3FieldSlotAbiV1", words, 9..28)
}

fn validate_hard_slot(words: &[u32; FIELD_SLOT_WORDS]) -> Result<()> {
    if !(FIELD_BAD_ABI..=FIELD_INTERNAL).contains(&words[2]) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            2,
            "HARD error must be one of the closed 1..16 codes",
        );
    }
    if words[5] != 0 {
        return invalid_word("M3FieldSlotAbiV1", 5, "HARD legal degree must be zero");
    }
    require_word(
        "M3FieldSlotAbiV1",
        words,
        6,
        INVALID_TILE,
        "HARD selected tile must be invalid",
    )?;
    require_zero_range("M3FieldSlotAbiV1", words, 9..28)
}

fn validate_valid_slot(words: &[u32; FIELD_SLOT_WORDS]) -> Result<()> {
    if words[2] != 0 {
        return invalid_word("M3FieldSlotAbiV1", 2, "VALID error must be zero");
    }
    require_legal_degree(words[5], "M3FieldSlotAbiV1", 5)?;
    if words[4] >= words[5] {
        return invalid_word(
            "M3FieldSlotAbiV1",
            4,
            "a VALID slot must lie in the dense prefix below legal degree",
        );
    }
    if words[6] >= PHYSICAL_TILE_COUNT {
        return invalid_word("M3FieldSlotAbiV1", 6, "VALID selected tile must be 0..27");
    }
    validate_hands("M3FieldSlotAbiV1", &words[9..13], 9)?;
    let packed = PackedState::try_from_raw(words[13])?;
    validate_packed_trick("M3FieldSlotAbiV1", words[14], packed.trick_length(), 14)?;
    validate_state_hands("M3FieldSlotAbiV1", &words[9..13], words[14], packed, 9)?;
    let hand_union = words[9] | words[10] | words[11] | words[12];
    if hand_union & (1 << words[6]) != 0 {
        return invalid_word(
            "M3FieldSlotAbiV1",
            6,
            "the selected tile must have been removed from every output hand",
        );
    }
    if packed.trick_length() != 0 {
        let last_tile = (words[14] >> (5 * (packed.trick_length() - 1))) & 0x1f;
        if last_tile != words[6] {
            return invalid_word(
                "M3FieldSlotAbiV1",
                14,
                "a non-completing transition must append the selected tile",
            );
        }
    }
    if words[15] > 34 {
        return invalid_word(
            "M3FieldSlotAbiV1",
            15,
            "future T0 spend must remain in the unsaturated range 0..34",
        );
    }
    if words[16] != 0 {
        return invalid_word("M3FieldSlotAbiV1", 16, "host-child ordinal must be zero");
    }
    if words[17..25].iter().all(|word| *word == 0) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            17,
            "a VALID production slot must carry positive mass",
        );
    }
    let winner = words[25] & 0x7;
    let points = (words[25] >> 3) & 0x1f;
    if words[25] >> 8 != 0
        || winner > 4
        || (winner == 4 && points != 0)
        || (winner < 4 && points == 0)
    {
        return invalid_word(
            "M3FieldSlotAbiV1",
            25,
            "winner/count packing is noncanonical",
        );
    }
    if words[26] & !0xf != 0 {
        return invalid_word("M3FieldSlotAbiV1", 26, "transition flags use bits above 3");
    }
    let completed = words[26] & 1 != 0;
    if completed != (winner < 4) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            25,
            "winner sentinel disagrees with trick-completed flag",
        );
    }
    if completed != (packed.trick_length() == 0) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            26,
            "trick-completed flag disagrees with the output trick reset",
        );
    }
    let terminal = words[26] & 2 != 0;
    if terminal != (packed.completed_future_tricks() == 4) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            26,
            "terminal flag disagrees with completed-future-tricks",
        );
    }
    let next_is_focal = words[26] & 4 != 0;
    if next_is_focal != (packed.next_actor() == 1) {
        return invalid_word(
            "M3FieldSlotAbiV1",
            26,
            "next-actor flag disagrees with the packed next actor",
        );
    }
    let objective_decided = words[26] & 8 != 0;
    let expected_decided = if task_objective(words[7]) == 2 {
        let live_points = live_future_points(&words[9..13], words[14], packed);
        words[15] > 11 || words[15] + live_points <= 11
    } else {
        terminal
    };
    if objective_decided != expected_decided {
        return invalid_word(
            "M3FieldSlotAbiV1",
            26,
            "objective-decided flag disagrees with the frozen objective automaton",
        );
    }
    if words[27] != 0 {
        return invalid_word("M3FieldSlotAbiV1", 27, "reserved word must be zero");
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReductionWords(M3ReductionAbiV1);

impl ReductionWords {
    pub fn valid(row_ordinal: u32, limbs: [u32; 8]) -> Self {
        let mut words = [0u32; REDUCTION_WORDS];
        words[0] = ABI_VERSION;
        words[1] = REDUCTION_STATUS_VALID;
        words[3] = row_ordinal;
        words[4..12].copy_from_slice(&limbs);
        Self(M3ReductionAbiV1 { words })
    }

    pub fn hard(error: u32, row_ordinal: u32) -> Result<Self> {
        let mut words = [0u32; REDUCTION_WORDS];
        words[0] = ABI_VERSION;
        words[1] = REDUCTION_STATUS_HARD;
        words[2] = error;
        words[3] = row_ordinal;
        Self::try_from_words(words)
    }

    pub fn try_from_slice(words: &[u32]) -> Result<Self> {
        let words: [u32; REDUCTION_WORDS] =
            words.try_into().map_err(|_| M3MetalError::WrongWordCount {
                record: "M3ReductionAbiV1",
                expected: REDUCTION_WORDS,
                actual: words.len(),
            })?;
        Self::try_from_words(words)
    }

    pub fn try_from_words(words: [u32; REDUCTION_WORDS]) -> Result<Self> {
        require_word(
            "M3ReductionAbiV1",
            &words,
            0,
            ABI_VERSION,
            "ABI must be one",
        )?;
        match words[1] {
            REDUCTION_STATUS_VALID if words[2] == 0 => {}
            REDUCTION_STATUS_VALID => {
                return invalid_word("M3ReductionAbiV1", 2, "VALID error must be zero")
            }
            REDUCTION_STATUS_HARD => {
                if !(REDUCTION_BAD_ABI..=REDUCTION_INTERNAL).contains(&words[2]) {
                    return invalid_word(
                        "M3ReductionAbiV1",
                        2,
                        "HARD error must be one of the closed 1..7 codes",
                    );
                }
                require_zero_range("M3ReductionAbiV1", &words, 4..12)?;
            }
            _ => return invalid_word("M3ReductionAbiV1", 1, "status must be VALID or HARD"),
        }
        Ok(Self(M3ReductionAbiV1 { words }))
    }

    pub const fn words(&self) -> &[u32; REDUCTION_WORDS] {
        &self.0.words
    }

    pub fn append_le_bytes(&self, output: &mut Vec<u8>) {
        append_words_le(output, &self.0.words);
    }

    pub const fn row_ordinal(&self) -> u32 {
        self.0.words[3]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReductionPairWords(M3ReductionPairV1);

impl ReductionPairWords {
    pub fn pair(left: u32, right: u32, destination: u32) -> Result<Self> {
        if right == INVALID_TILE {
            return invalid_word(
                "M3ReductionPairV1",
                1,
                "non-carry pair may not use the carry sentinel",
            );
        }
        Self::try_from_words([left, right, destination, 0])
    }

    pub fn carry(left: u32, destination: u32) -> Result<Self> {
        Self::try_from_words([left, u32::MAX, destination, 1])
    }

    pub fn try_from_slice(words: &[u32]) -> Result<Self> {
        let words: [u32; REDUCTION_PAIR_WORDS] =
            words.try_into().map_err(|_| M3MetalError::WrongWordCount {
                record: "M3ReductionPairV1",
                expected: REDUCTION_PAIR_WORDS,
                actual: words.len(),
            })?;
        Self::try_from_words(words)
    }

    pub fn try_from_words(words: [u32; REDUCTION_PAIR_WORDS]) -> Result<Self> {
        if words[3] & !1 != 0 {
            return invalid_word("M3ReductionPairV1", 3, "only the carry flag is admitted");
        }
        let carry = words[3] == 1;
        if carry != (words[1] == u32::MAX) {
            return invalid_word(
                "M3ReductionPairV1",
                1,
                "right sentinel and carry flag must agree",
            );
        }
        Ok(Self(M3ReductionPairV1 { words }))
    }

    pub const fn words(&self) -> &[u32; REDUCTION_PAIR_WORDS] {
        &self.0.words
    }

    pub fn append_le_bytes(&self, output: &mut Vec<u8>) {
        append_words_le(output, &self.0.words);
    }

    pub const fn left(&self) -> u32 {
        self.0.words[0]
    }

    pub const fn right(&self) -> u32 {
        self.0.words[1]
    }

    pub const fn destination(&self) -> u32 {
        self.0.words[2]
    }

    pub const fn is_carry(&self) -> bool {
        self.0.words[3] == 1
    }
}

fn require_task(value: u32, record: &'static str, word: usize) -> Result<()> {
    if value < TASK_COUNT {
        Ok(())
    } else {
        invalid_word(record, word, "task ordinal must be 0..7")
    }
}

fn append_words_le(output: &mut Vec<u8>, words: &[u32]) {
    output.reserve(size_of_val(words));
    for word in words {
        output.extend_from_slice(&word.to_le_bytes());
    }
}

fn require_objective(value: u32, record: &'static str, word: usize) -> Result<()> {
    if (OBJECTIVE_A..=OBJECTIVE_B).contains(&value) {
        Ok(())
    } else {
        invalid_word(record, word, "objective must be 1 or 2")
    }
}

fn require_treatment(value: u32, record: &'static str, word: usize) -> Result<()> {
    if (TREATMENT_H..=TREATMENT_C).contains(&value) {
        Ok(())
    } else {
        invalid_word(record, word, "treatment must be H=1 or C=2")
    }
}

fn require_source(value: u32, record: &'static str, word: usize) -> Result<()> {
    if value < SUPPORT_WORLD_COUNT {
        Ok(())
    } else {
        invalid_word(record, word, "support source ordinal must be 0..1199")
    }
}

fn require_legal_degree(value: u32, record: &'static str, word: usize) -> Result<()> {
    if (1..=LEGAL_SLOT_COUNT).contains(&value) {
        Ok(())
    } else {
        invalid_word(record, word, "legal degree must be 1..7")
    }
}

fn validate_hands(record: &'static str, hands: &[u32], first_word: usize) -> Result<()> {
    for (index, hand) in hands.iter().copied().enumerate() {
        if hand & 0xf000_0000 != 0 {
            return invalid_word(record, first_word + index, "hand uses bits above tile 27");
        }
        for prior in hands[..index].iter().copied() {
            if prior & hand != 0 {
                return invalid_word(record, first_word + index, "hand masks overlap");
            }
        }
    }
    Ok(())
}

fn validate_state_hands(
    record: &'static str,
    hands: &[u32],
    packed_trick: u32,
    state: PackedState,
    first_hand_word: usize,
) -> Result<()> {
    if state.next_actor() != (state.leader() + state.trick_length()) & 3 {
        return invalid_word(
            record,
            first_hand_word,
            "packed next actor disagrees with leader and trick length",
        );
    }
    if state.field_exponent() != state.expected_field_exponent() {
        return invalid_word(
            record,
            first_hand_word,
            "field exponent disagrees with the frozen focal/field move census",
        );
    }

    let mut expected_sizes = [4 - state.completed_future_tricks(); 4];
    let mut trick_mask = 0u32;
    let mut lane = 0;
    while lane < state.trick_length() {
        let actor = ((state.leader() + lane) & 3) as usize;
        if expected_sizes[actor] == 0 {
            return invalid_word(
                record,
                first_hand_word + actor,
                "current trick removes a tile from an empty seat",
            );
        }
        expected_sizes[actor] -= 1;
        let tile = (packed_trick >> (5 * lane)) & 0x1f;
        trick_mask |= 1u32 << tile;
        lane += 1;
    }

    let mut hand_union = 0u32;
    for (seat, hand) in hands.iter().copied().enumerate() {
        if hand.count_ones() != expected_sizes[seat] {
            return invalid_word(
                record,
                first_hand_word + seat,
                "hand size disagrees with the completed/current trick census",
            );
        }
        hand_union |= hand;
    }
    if hand_union & trick_mask != 0 {
        return invalid_word(
            record,
            first_hand_word,
            "a current-trick tile remains in a hand",
        );
    }
    Ok(())
}

fn live_future_points(hands: &[u32], packed_trick: u32, state: PackedState) -> u32 {
    let mut points = 4 - state.completed_future_tricks();
    for hand in hands {
        let mut remaining = *hand;
        while remaining != 0 {
            let tile = remaining.trailing_zeros();
            remaining &= remaining - 1;
            points += tile_count(tile);
        }
    }
    let mut lane = 0;
    while lane < state.trick_length() {
        points += tile_count((packed_trick >> (5 * lane)) & 0x1f);
        lane += 1;
    }
    points
}

fn tile_count(tile: u32) -> u32 {
    let mut high = 0;
    while tile >= (high + 1) * (high + 2) / 2 {
        high += 1;
    }
    let low = tile - high * (high + 1) / 2;
    match high + low {
        5 => 5,
        10 => 10,
        _ => 0,
    }
}

fn validate_packed_trick(
    record: &'static str,
    packed: u32,
    trick_length: u32,
    word: usize,
) -> Result<()> {
    if packed >> 20 != 0 {
        return invalid_word(record, word, "packed trick bits 20..31 must be zero");
    }
    let mut seen = 0u32;
    for lane in 0..4u32 {
        let tile = (packed >> (5 * lane)) & 0x1f;
        if lane < trick_length {
            if tile >= PHYSICAL_TILE_COUNT {
                return invalid_word(record, word, "live trick lane must contain tile 0..27");
            }
            let bit = 1u32 << tile;
            if seen & bit != 0 {
                return invalid_word(record, word, "live trick tiles must be distinct");
            }
            seen |= bit;
        } else if tile != 31 {
            return invalid_word(record, word, "empty trick lane must contain sentinel 31");
        }
    }
    Ok(())
}

fn require_nonzero_cap(
    record: &'static str,
    word: usize,
    value: u32,
    cap: u32,
    reason: &'static str,
) -> Result<()> {
    if value == 0 || value > cap {
        invalid_word(record, word, reason)
    } else {
        Ok(())
    }
}

fn require_word<const N: usize>(
    record: &'static str,
    words: &[u32; N],
    word: usize,
    expected: u32,
    reason: &'static str,
) -> Result<()> {
    if words[word] == expected {
        Ok(())
    } else {
        invalid_word(record, word, reason)
    }
}

fn require_zero_range<const N: usize>(
    record: &'static str,
    words: &[u32; N],
    range: core::ops::Range<usize>,
) -> Result<()> {
    for word in range {
        if words[word] != 0 {
            return invalid_word(record, word, "reserved or inapplicable word must be zero");
        }
    }
    Ok(())
}

fn invalid_word<T>(record: &'static str, word: usize, reason: &'static str) -> Result<T> {
    Err(M3MetalError::InvalidWord {
        record,
        word,
        reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packed_state(
        leader: u32,
        next_actor: u32,
        trick_length: u32,
        completed: u32,
        wins: u32,
        exponent: u32,
    ) -> u32 {
        leader
            | (next_actor << 2)
            | (trick_length << 4)
            | (completed << 7)
            | (wins << 10)
            | (exponent << 13)
            | ((12 + 4 * completed + trick_length) << 17)
    }

    fn packed_trick(live: &[u32]) -> u32 {
        let mut packed = 0u32;
        for lane in 0..4usize {
            let tile = live.get(lane).copied().unwrap_or(31);
            packed |= tile << (5 * lane);
        }
        packed
    }

    #[test]
    fn private_layouts_are_exact() {
        assert_eq!(size_of::<M3KernelControlV1>(), 64);
        assert_eq!(size_of::<M3ParticleAbiV1>(), 96);
        assert_eq!(size_of::<M3FieldSlotAbiV1>(), 112);
        assert_eq!(size_of::<M3ReductionAbiV1>(), 48);
        assert_eq!(size_of::<M3ReductionPairV1>(), 16);
        assert_eq!(align_of::<M3KernelControlV1>(), 4);
        assert_eq!(align_of::<M3ParticleAbiV1>(), 4);
        assert_eq!(align_of::<M3FieldSlotAbiV1>(), 4);
        assert_eq!(align_of::<M3ReductionAbiV1>(), 4);
        assert_eq!(align_of::<M3ReductionPairV1>(), 4);
    }

    #[test]
    fn field_and_reduction_error_precedence_is_frozen() {
        assert_eq!(
            FIELD_ERROR_PRECEDENCE,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        );
        assert_eq!(REDUCTION_ERROR_PRECEDENCE, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn exact_field_control_is_admitted() {
        let control = ControlWords::field(7, 2, 1, 1, INPUT_RECORD_CAP, 11, 9).unwrap();
        assert_eq!(control.words()[8], OUTPUT_SLOT_CAP);
        assert_eq!(control.words()[10..], [0; 6]);
        assert!(ControlWords::field(7, 2, 2, 1, 1, 0, 9).is_err());
        assert!(ControlWords::field(7, 2, 1, 1, 1, 0, 7).is_err());
        assert_eq!(TASK_OBJECTIVES, [1, 1, 1, 1, 2, 2, 2, 2]);
        assert_eq!(TASK_PHYSICAL_ROOTS, [4, 7, 9, 20, 4, 7, 9, 20]);
    }

    #[test]
    fn particle_lane_sentinels_and_reserved_words_are_closed() {
        let mut words = [0u32; PARTICLE_WORDS];
        words[0] = 1;
        words[1] = 1;
        words[2] = 0;
        words[3] = 1_199;
        words[4] = 1 << 2;
        words[5] = 1 << 3;
        words[6] = (1 << 4) | (1 << 8);
        words[7] = (1 << 5) | (1 << 9);
        words[8] = packed_state(0, 2, 2, 2, 1, 7);
        words[9] = packed_trick(&[6, 7]);
        words[11] = 9;
        words[12] = 1;
        ParticleWords::try_from_words(words).unwrap();
        words[9] &= !(0x1f << 15);
        assert!(ParticleWords::try_from_words(words).is_err());
    }

    #[test]
    fn empty_and_hard_slots_are_word_for_word_canonical() {
        let empty = FieldSlotWords::canonical_empty(41, 6, 3, 3, 1_199).unwrap();
        assert_eq!(empty.words()[0..9], [1, 0, 0, 41, 6, 3, u32::MAX, 3, 1_199]);
        assert!(empty.words()[9..].iter().all(|word| *word == 0));

        for error in FIELD_ERROR_PRECEDENCE {
            let hard = FieldSlotWords::canonical_hard(error, 41, 2, 3, 9).unwrap();
            assert_eq!(hard.words()[0..9], [1, 2, error, 41, 2, 0, u32::MAX, 3, 9]);
            assert!(hard.words()[9..].iter().all(|word| *word == 0));
        }

        // BAD_TASK and BAD_ABI preserve the three raw provenance snapshots,
        // even when those values are themselves outside the valid registry.
        let raw = FieldSlotWords::canonical_hard(FIELD_BAD_TASK, u32::MAX, 0, u32::MAX, u32::MAX)
            .unwrap();
        assert_eq!(raw.words()[3], u32::MAX);
        assert_eq!(raw.words()[7], u32::MAX);
        assert_eq!(raw.words()[8], u32::MAX);
    }

    fn m3b_completed_transition(spend: u32, decided: bool) -> Result<FieldSlotWords> {
        let mut words = [0u32; FIELD_SLOT_WORDS];
        words[0] = ABI_VERSION;
        words[1] = FIELD_STATUS_VALID;
        words[3] = 17;
        words[4] = 0;
        words[5] = 1;
        words[6] = 4;
        words[7] = 4;
        words[8] = 0;
        words[9] = 1 << 0;
        words[10] = 1 << 1;
        words[11] = 1 << 2;
        words[12] = 1 << 3;
        // S0 leads the empty fourth future trick; three field moves have
        // occurred per completed trick, so r=9 and record length is 24.
        words[13] = (3 << 7) | (9 << 13) | (24 << 17);
        words[14] = packed_trick(&[]);
        words[15] = spend;
        words[17] = 1;
        words[25] = 1 << 3; // winner S0, one point
        words[26] = 1 | if decided { 8 } else { 0 };
        FieldSlotWords::try_from_words(words)
    }

    #[test]
    fn m3b_decidedness_covers_make_set_and_the_open_middle() {
        // The four live non-count tiles plus the final trick point give
        // P_live=1. Thus spend 10 is a guaranteed make, 11 is still open,
        // and 12 is already set.
        m3b_completed_transition(10, true).unwrap();
        m3b_completed_transition(11, false).unwrap();
        m3b_completed_transition(12, true).unwrap();

        assert!(m3b_completed_transition(10, false).is_err());
        assert!(m3b_completed_transition(11, true).is_err());
        assert!(m3b_completed_transition(12, false).is_err());
    }

    #[test]
    fn valid_transition_shape_covers_append_and_terminal_focal_winner() {
        let mut append = [0u32; FIELD_SLOT_WORDS];
        append[0] = ABI_VERSION;
        append[1] = FIELD_STATUS_VALID;
        append[3] = 20;
        append[5] = 1;
        append[6] = 4;
        append[7] = 0;
        append[9] = 1 << 0;
        append[10] = 1 << 1;
        append[12] = 1 << 3;
        append[13] = 2 | (3 << 2) | (1 << 4) | (3 << 7) | (10 << 13) | (25 << 17);
        append[14] = packed_trick(&[4]);
        append[17] = 1;
        append[25] = 4;
        FieldSlotWords::try_from_words(append).unwrap();

        let mut terminal = [0u32; FIELD_SLOT_WORDS];
        terminal[0] = ABI_VERSION;
        terminal[1] = FIELD_STATUS_VALID;
        terminal[3] = 21;
        terminal[5] = 1;
        terminal[6] = 4;
        terminal[7] = 0;
        terminal[13] = 1 | (1 << 2) | (4 << 7) | (4 << 10) | (12 << 13) | (28 << 17);
        terminal[14] = packed_trick(&[]);
        terminal[17] = 1;
        terminal[25] = 1 | (1 << 3);
        terminal[26] = 1 | 2 | 4 | 8;
        FieldSlotWords::try_from_words(terminal).unwrap();
    }

    #[test]
    fn canonical_serialization_is_wordwise_little_endian() {
        let pair = ReductionPairWords::pair(0x1122_3344, 0x5566_7788, 0x99aa_bbcc).unwrap();
        let mut bytes = Vec::new();
        pair.append_le_bytes(&mut bytes);
        assert_eq!(
            bytes,
            [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55, 0xcc, 0xbb, 0xaa, 0x99, 0, 0, 0, 0,]
        );
    }

    #[test]
    fn reduction_status_and_carry_forms_are_closed() {
        let valid = ReductionWords::valid(19, [u32::MAX; 8]);
        ReductionWords::try_from_words(*valid.words()).unwrap();
        for error in REDUCTION_ERROR_PRECEDENCE {
            let hard = ReductionWords::hard(error, 19).unwrap();
            assert!(hard.words()[4..].iter().all(|word| *word == 0));
        }
        assert!(ReductionPairWords::try_from_words([0, u32::MAX, 0, 0]).is_err());
        assert!(ReductionPairWords::try_from_words([0, 1, 0, 1]).is_err());
        ReductionPairWords::pair(0, 1, 0).unwrap();
        ReductionPairWords::carry(2, 1).unwrap();
    }
}
