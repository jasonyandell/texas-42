use core::mem::{align_of, size_of};

use crate::MetalError;

pub const POISON: u32 = 0xA5A5_5A5A;
pub const THREADGROUP_WIDTH: usize = 32;
pub const OFFICIAL_ARITHMETIC_CASES: usize = 16_384;
pub const OPENING_SLOT_CAP: usize = 79_800;
pub const OPENING_ARENA_RECORDS: usize = OPENING_SLOT_CAP + 2;
pub const CHOOSE_WORDS: usize = 22 * 22;

pub const ARITHMETIC_SUCCESS: u32 = 1;
pub const ARITHMETIC_CHECKED_UNDEFINED: u32 = 2;

pub const OPENING_SKIP: u32 = 0;
pub const OPENING_VALID: u32 = 1;

const OP_ADD: u32 = 1;
const OP_SUB: u32 = 2;
const OP_MUL_SMALL: u32 = 3;
const OP_MUL_POW_420: u32 = 4;
const OP_COMPARE: u32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct OpeningTaskAbi {
    pub(crate) words: [u32; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct OpeningSlotAbi {
    pub(crate) words: [u32; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ArithmeticInputAbi {
    pub(crate) words: [u32; 20],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ArithmeticOutputAbi {
    pub(crate) words: [u32; 16],
}

const _: () = assert!(size_of::<OpeningTaskAbi>() == 32);
const _: () = assert!(align_of::<OpeningTaskAbi>() == 4);
const _: () = assert!(size_of::<OpeningSlotAbi>() == 64);
const _: () = assert!(align_of::<OpeningSlotAbi>() == 4);
const _: () = assert!(size_of::<ArithmeticInputAbi>() == 80);
const _: () = assert!(align_of::<ArithmeticInputAbi>() == 4);
const _: () = assert!(size_of::<ArithmeticOutputAbi>() == 64);
const _: () = assert!(align_of::<ArithmeticOutputAbi>() == 4);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArithmeticInputWords {
    words: [u32; 20],
}

impl ArithmeticInputWords {
    pub fn try_from_words(words: [u32; 20]) -> Result<Self, MetalError> {
        validate_arithmetic_words(&words).map_err(|reason| MetalError::InvalidArithmeticInput {
            index: words[1] as usize,
            reason,
        })?;
        Ok(Self { words })
    }

    pub const fn words(&self) -> &[u32; 20] {
        &self.words
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OpeningTaskWords {
    words: [u32; 8],
}

impl OpeningTaskWords {
    pub fn try_from_words(words: [u32; 8]) -> Result<Self, MetalError> {
        validate_opening_task(&words).map_err(MetalError::InvalidOpeningTask)?;
        Ok(Self { words })
    }

    pub const fn words(&self) -> &[u32; 8] {
        &self.words
    }

    pub const fn task_ordinal(self) -> usize {
        self.words[1] as usize
    }

    pub const fn candidate_slot_count(self) -> usize {
        self.words[7] as usize
    }

    pub const fn response_triple_count(self) -> usize {
        self.words[6] as usize
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OpeningChooseTableWords {
    words: [u32; CHOOSE_WORDS],
}

impl OpeningChooseTableWords {
    pub fn try_from_words(words: [u32; CHOOSE_WORDS]) -> Result<Self, MetalError> {
        for n in 0..22 {
            for k in 0..22 {
                let expected = checked_choose(n, k);
                let actual = words[n * 22 + k];
                if actual != expected {
                    return Err(MetalError::InvalidChooseEntry {
                        n,
                        k,
                        expected,
                        actual,
                    });
                }
            }
        }
        Ok(Self { words })
    }

    #[cfg(test)]
    pub fn canonical() -> Self {
        let mut words = [0u32; CHOOSE_WORDS];
        for n in 0..22 {
            for k in 0..22 {
                words[n * 22 + k] = checked_choose(n, k);
            }
        }
        Self { words }
    }

    pub const fn words(&self) -> &[u32; CHOOSE_WORDS] {
        &self.words
    }
}

pub(crate) fn validate_arithmetic_words(words: &[u32; 20]) -> Result<(), &'static str> {
    if words[0] != 1 {
        return Err("ABI version");
    }
    let operation = words[2];
    if !(OP_ADD..=OP_COMPARE).contains(&operation) {
        return Err("operation");
    }
    if matches!(operation, OP_ADD | OP_SUB | OP_COMPARE) && words[3] != 0 {
        return Err("unused operand");
    }
    if matches!(operation, OP_MUL_SMALL | OP_MUL_POW_420)
        && words[12..20].iter().any(|word| *word != 0)
    {
        return Err("unused rhs");
    }
    if operation == OP_MUL_POW_420 && words[3] > 21 {
        return Err("exponent");
    }
    Ok(())
}

pub(crate) fn validate_opening_task(words: &[u32; 8]) -> Result<(), &'static str> {
    if words[0] != 1 {
        return Err("ABI version");
    }
    if words[3] & 0xf000_0000 != 0 || words[4] & 0xf000_0000 != 0 {
        return Err("mask high bits");
    }
    if words[4] & !words[3] != 0 {
        return Err("matching subset");
    }
    if !(1..=7).contains(&words[2]) {
        return Err("grade");
    }
    if words[3].count_ones() != words[5] || words[5] != 3 * words[2] {
        return Err("pool count");
    }
    if words[4].count_ones() > 6 {
        return Err("matching count");
    }
    let responses = words[5] * (words[5] - 1) * (words[5] - 2);
    if words[6] != responses {
        return Err("response count");
    }
    let slots = responses * 10;
    if words[7] != slots || slots > OPENING_SLOT_CAP as u32 {
        return Err("slot count");
    }
    Ok(())
}

fn checked_choose(n: usize, k: usize) -> u32 {
    if k > n {
        return 0;
    }
    let width = core::cmp::min(k, n - k);
    let mut value = 1u64;
    for index in 0..width {
        value = value * (n - index) as u64 / (index + 1) as u64;
    }
    u32::try_from(value).expect("choose(0..21,0..21) fits u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_abi_sizes_and_arena_arithmetic_hold() {
        assert_eq!(size_of::<OpeningTaskAbi>(), 32);
        assert_eq!(size_of::<OpeningSlotAbi>(), 64);
        assert_eq!(size_of::<ArithmeticInputAbi>(), 80);
        assert_eq!(size_of::<ArithmeticOutputAbi>(), 64);
        assert_eq!(OPENING_ARENA_RECORDS * 64 + 32 + 1_936, 5_109_296);
        assert_eq!(OFFICIAL_ARITHMETIC_CASES * 80, 1_310_720);
        assert_eq!((OFFICIAL_ARITHMETIC_CASES + 2) * 64, 1_048_704);
    }

    #[test]
    fn canonical_choose_table_round_trips() {
        let table = OpeningChooseTableWords::canonical();
        assert_eq!(table.words()[21 * 22 + 10], 352_716);
        OpeningChooseTableWords::try_from_words(*table.words()).expect("canonical table");
    }
}
