//! Checked launch descriptors for the exact M3 field and reduction kernels.
//!
//! These types close the host side before any encoder exists.  In particular,
//! a reduction plan is reconstructed from range slices and compared pair for
//! pair, so no caller can smuggle a cross-range pair, repeated source, sparse
//! destination, or nonfinal carry into Metal.

use std::collections::BTreeSet;

use crate::abi::{
    ControlWords, ParticleWords, ReductionPairWords, ReductionWords, INPUT_RECORD_CAP,
    INVALID_TILE, OPCODE_EXPAND, OPCODE_REDUCE, REDUCTION_STATUS_VALID,
};
use crate::error::{M3MetalError, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReductionRangeSlice {
    source_count: u32,
    ends_semantic_range: bool,
}

impl ReductionRangeSlice {
    /// Describe one consecutive local slice of one semantic range.
    ///
    /// A slice which does not end the semantic range must contain an even
    /// number of source rows: command boundaries occur only between canonical
    /// adjacent pairs.  An ending slice may own the range's odd final carry.
    pub fn new(source_count: u32, ends_semantic_range: bool) -> Result<Self> {
        if source_count == 0 {
            return Err(M3MetalError::NonCanonicalRange {
                range: 0,
                reason: "range slice must be nonempty",
            });
        }
        if !ends_semantic_range && !source_count.is_multiple_of(2) {
            return Err(M3MetalError::NonCanonicalRange {
                range: 0,
                reason: "a nonfinal range slice must end between complete pairs",
            });
        }
        Ok(Self {
            source_count,
            ends_semantic_range,
        })
    }

    pub const fn source_count(self) -> u32 {
        self.source_count
    }

    pub const fn ends_semantic_range(self) -> bool {
        self.ends_semantic_range
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedReductionPlan {
    source_count: u32,
    ranges: Vec<ReductionRangeSlice>,
    pairs: Vec<ReductionPairWords>,
}

impl CheckedReductionPlan {
    pub fn new(
        source_count: u32,
        ranges: Vec<ReductionRangeSlice>,
        pairs: Vec<ReductionPairWords>,
    ) -> Result<Self> {
        if source_count == 0 {
            return Err(M3MetalError::EmptyLaunch("reduction source"));
        }
        if source_count > INPUT_RECORD_CAP {
            return Err(M3MetalError::CapExceeded {
                cap: "reduction source records",
                limit: u64::from(INPUT_RECORD_CAP),
                observed: u64::from(source_count),
            });
        }
        if ranges.is_empty() {
            return Err(M3MetalError::NonCanonicalRange {
                range: 0,
                reason: "checked plan must name at least one range slice",
            });
        }

        let mut reconstructed_source_count = 0u32;
        let mut reconstructed_pair_count = 0u32;
        for (range_index, range) in ranges.iter().copied().enumerate() {
            if range.source_count == 0 {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "range slice must be nonempty",
                });
            }
            if !range.ends_semantic_range && !range.source_count.is_multiple_of(2) {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "a nonfinal range slice must end between complete pairs",
                });
            }
            if !range.ends_semantic_range && range_index + 1 != ranges.len() {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "only the final command slice may continue into a later command",
                });
            }
            reconstructed_source_count = reconstructed_source_count
                .checked_add(range.source_count)
                .ok_or(M3MetalError::LengthOverflow(
                    "reduction range source census",
                ))?;
            reconstructed_pair_count = reconstructed_pair_count
                .checked_add(range.source_count / 2)
                .and_then(|count| {
                    count.checked_add(u32::from(
                        range.ends_semantic_range && !range.source_count.is_multiple_of(2),
                    ))
                })
                .ok_or(M3MetalError::LengthOverflow(
                    "reduction range destination census",
                ))?;
        }
        if reconstructed_source_count != source_count {
            return Err(M3MetalError::NonCanonicalRange {
                range: ranges.len(),
                reason: "range slices do not partition the local source rows",
            });
        }
        if reconstructed_pair_count == 0 || reconstructed_pair_count > INPUT_RECORD_CAP {
            return Err(M3MetalError::CapExceeded {
                cap: "reduction destination records",
                limit: u64::from(INPUT_RECORD_CAP),
                observed: u64::from(reconstructed_pair_count),
            });
        }
        if usize::try_from(reconstructed_pair_count).ok() != Some(pairs.len()) {
            return Err(M3MetalError::NonCanonicalPlan {
                pair: pairs.len(),
                reason: "pair count differs from canonical range reduction",
            });
        }

        let mut source_cursor = 0u32;
        let mut pair_cursor = 0usize;
        for range in ranges.iter().copied() {
            let pair_sources = range.source_count / 2;
            for local_pair in 0..pair_sources {
                let left = source_cursor
                    .checked_add(local_pair * 2)
                    .ok_or(M3MetalError::LengthOverflow("reduction pair left"))?;
                let right = left
                    .checked_add(1)
                    .ok_or(M3MetalError::LengthOverflow("reduction pair right"))?;
                require_pair(&pairs, pair_cursor, left, right, pair_cursor as u32, false)?;
                pair_cursor += 1;
            }
            if !range.source_count.is_multiple_of(2) {
                // Construction above already proves this is the actual end of
                // the semantic range, never a command-chunk carry.
                let left = source_cursor
                    .checked_add(range.source_count - 1)
                    .ok_or(M3MetalError::LengthOverflow("reduction carry left"))?;
                require_pair(
                    &pairs,
                    pair_cursor,
                    left,
                    INVALID_TILE,
                    pair_cursor as u32,
                    true,
                )?;
                pair_cursor += 1;
            }
            source_cursor = source_cursor
                .checked_add(range.source_count)
                .ok_or(M3MetalError::LengthOverflow("reduction source cursor"))?;
        }
        if pair_cursor != pairs.len() || source_cursor != source_count {
            return Err(M3MetalError::NonCanonicalPlan {
                pair: pair_cursor,
                reason: "canonical reconstruction did not consume the complete plan",
            });
        }

        Ok(Self {
            source_count,
            ranges,
            pairs,
        })
    }

    /// Construct the unique canonical plan for the supplied range slices.
    pub fn canonical(ranges: Vec<ReductionRangeSlice>) -> Result<Self> {
        let source_count = ranges.iter().try_fold(0u32, |sum, range| {
            sum.checked_add(range.source_count)
                .ok_or(M3MetalError::LengthOverflow(
                    "canonical reduction source count",
                ))
        })?;
        let mut pairs = Vec::new();
        let mut source_cursor = 0u32;
        for (range_index, range) in ranges.iter().copied().enumerate() {
            if range.source_count == 0 {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "range slice must be nonempty",
                });
            }
            if !range.ends_semantic_range && !range.source_count.is_multiple_of(2) {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "a nonfinal range slice must end between complete pairs",
                });
            }
            if !range.ends_semantic_range && range_index + 1 != ranges.len() {
                return Err(M3MetalError::NonCanonicalRange {
                    range: range_index,
                    reason: "only the final command slice may continue into a later command",
                });
            }
            for local_pair in 0..(range.source_count / 2) {
                let left = source_cursor.checked_add(local_pair * 2).ok_or(
                    M3MetalError::LengthOverflow("canonical reduction pair left"),
                )?;
                let destination = u32::try_from(pairs.len())
                    .map_err(|_| M3MetalError::LengthOverflow("reduction destination"))?;
                pairs.push(ReductionPairWords::pair(left, left + 1, destination)?);
            }
            if !range.source_count.is_multiple_of(2) {
                let left = source_cursor.checked_add(range.source_count - 1).ok_or(
                    M3MetalError::LengthOverflow("canonical reduction carry left"),
                )?;
                let destination = u32::try_from(pairs.len())
                    .map_err(|_| M3MetalError::LengthOverflow("reduction destination"))?;
                pairs.push(ReductionPairWords::carry(left, destination)?);
            }
            source_cursor = source_cursor.checked_add(range.source_count).ok_or(
                M3MetalError::LengthOverflow("canonical reduction source cursor"),
            )?;
        }
        Self::new(source_count, ranges, pairs)
    }

    pub const fn source_count(&self) -> u32 {
        self.source_count
    }

    pub fn destination_count(&self) -> u32 {
        u32::try_from(self.pairs.len()).expect("checked plan is capped below u32::MAX")
    }

    pub fn ranges(&self) -> &[ReductionRangeSlice] {
        &self.ranges
    }

    pub fn pairs(&self) -> &[ReductionPairWords] {
        &self.pairs
    }
}

fn require_pair(
    pairs: &[ReductionPairWords],
    index: usize,
    left: u32,
    right: u32,
    destination: u32,
    carry: bool,
) -> Result<()> {
    let pair = pairs.get(index).ok_or(M3MetalError::NonCanonicalPlan {
        pair: index,
        reason: "canonical pair is missing",
    })?;
    if pair.left() != left {
        return Err(M3MetalError::NonCanonicalPlan {
            pair: index,
            reason: "left source is not canonical adjacency",
        });
    }
    if pair.right() != right {
        return Err(M3MetalError::NonCanonicalPlan {
            pair: index,
            reason: "right source crosses, skips, or misstates a carry",
        });
    }
    if pair.destination() != destination {
        return Err(M3MetalError::NonCanonicalPlan {
            pair: index,
            reason: "destinations must be exactly dense 0..count-1",
        });
    }
    if pair.is_carry() != carry {
        return Err(M3MetalError::NonCanonicalPlan {
            pair: index,
            reason: "carry flag differs from canonical range ending",
        });
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedFieldLaunch {
    control: ControlWords,
    particles: Vec<ParticleWords>,
}

impl CheckedFieldLaunch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        command: u32,
        task: u32,
        objective: u32,
        treatment: u32,
        field_exponent: u32,
        physical_root: u32,
        particles: Vec<ParticleWords>,
    ) -> Result<Self> {
        if particles.is_empty() {
            return Err(M3MetalError::EmptyLaunch("field"));
        }
        let input_count = u32::try_from(particles.len())
            .map_err(|_| M3MetalError::LengthOverflow("field particle count"))?;
        if input_count > INPUT_RECORD_CAP {
            return Err(M3MetalError::CapExceeded {
                cap: "field input records",
                limit: u64::from(INPUT_RECORD_CAP),
                observed: u64::from(input_count),
            });
        }
        let mut host_ordinals = BTreeSet::new();
        for particle in &particles {
            if particle.task_ordinal() != task {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 field launch",
                    reason: "particle task differs from control task",
                });
            }
            let state = particle.packed_state();
            if state.field_exponent() != field_exponent {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 field launch",
                    reason: "particle exponent differs from control exponent",
                });
            }
            if state.next_actor() == 1 {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 field launch",
                    reason: "S1 focal application must remain on the host",
                });
            }
            let expected_actor = (state.leader() + state.trick_length()) & 3;
            if state.next_actor() != expected_actor {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 field launch",
                    reason: "next actor disagrees with leader and trick length",
                });
            }
            if !host_ordinals.insert(particle.host_record_ordinal()) {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 field launch",
                    reason: "host record ordinal is duplicated inside the slab",
                });
            }
        }
        let control = ControlWords::field(
            command,
            task,
            objective,
            treatment,
            input_count,
            field_exponent,
            physical_root,
        )?;
        debug_assert_eq!(control.opcode(), OPCODE_EXPAND);
        Ok(Self { control, particles })
    }

    pub const fn control(&self) -> &ControlWords {
        &self.control
    }

    pub fn particles(&self) -> &[ParticleWords] {
        &self.particles
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckedReductionLaunch {
    control: ControlWords,
    source: Vec<ReductionWords>,
    plan: CheckedReductionPlan,
}

impl CheckedReductionLaunch {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        command: u32,
        task: u32,
        objective: u32,
        treatment: u32,
        source_row_base: u32,
        destination_row_base: u32,
        level: u32,
        source: Vec<ReductionWords>,
        plan: CheckedReductionPlan,
    ) -> Result<Self> {
        let source_len = u32::try_from(source.len())
            .map_err(|_| M3MetalError::LengthOverflow("reduction source length"))?;
        if source_len != plan.source_count() {
            return Err(M3MetalError::InvalidRecord {
                record: "M3 reduction launch",
                reason: "source length differs from the checked plan",
            });
        }
        for (local, row) in source.iter().enumerate() {
            if row.words()[1] != REDUCTION_STATUS_VALID {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 reduction launch",
                    reason: "production source rows must be VALID",
                });
            }
            let local = u32::try_from(local)
                .map_err(|_| M3MetalError::LengthOverflow("reduction local row"))?;
            let expected =
                source_row_base
                    .checked_add(local)
                    .ok_or(M3MetalError::LengthOverflow(
                        "reduction source row identity",
                    ))?;
            if row.row_ordinal() != expected {
                return Err(M3MetalError::InvalidRecord {
                    record: "M3 reduction launch",
                    reason: "source row identity differs from base plus local index",
                });
            }
        }
        let range_count = u32::try_from(plan.ranges().len())
            .map_err(|_| M3MetalError::LengthOverflow("reduction range count"))?;
        let control = ControlWords::reduction(
            command,
            task,
            objective,
            treatment,
            plan.destination_count(),
            source_row_base,
            destination_row_base,
            level,
            plan.source_count(),
            range_count,
        )?;
        debug_assert_eq!(control.opcode(), OPCODE_REDUCE);
        Ok(Self {
            control,
            source,
            plan,
        })
    }

    pub const fn control(&self) -> &ControlWords {
        &self.control
    }

    pub fn source(&self) -> &[ReductionWords] {
        &self.source
    }

    pub const fn plan(&self) -> &CheckedReductionPlan {
        &self.plan
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::{ABI_VERSION, PARTICLE_WORDS};

    #[test]
    fn canonical_plan_never_pairs_across_ranges() {
        let plan = CheckedReductionPlan::canonical(vec![
            ReductionRangeSlice::new(3, true).unwrap(),
            ReductionRangeSlice::new(2, true).unwrap(),
        ])
        .unwrap();
        assert_eq!(plan.source_count(), 5);
        assert_eq!(plan.destination_count(), 3);
        assert_eq!(plan.pairs()[0].words(), &[0, 1, 0, 0]);
        assert_eq!(plan.pairs()[1].words(), &[2, u32::MAX, 1, 1]);
        assert_eq!(plan.pairs()[2].words(), &[3, 4, 2, 0]);
    }

    #[test]
    fn nonfinal_odd_slice_is_rejected() {
        assert!(ReductionRangeSlice::new(3, false).is_err());
    }

    #[test]
    fn nonfinal_slice_must_be_the_command_tail() {
        let slices = vec![
            ReductionRangeSlice::new(2, false).unwrap(),
            ReductionRangeSlice::new(2, true).unwrap(),
        ];
        assert!(CheckedReductionPlan::canonical(slices).is_err());
    }

    #[test]
    fn swapped_or_cross_range_pairs_are_rejected() {
        let ranges = vec![
            ReductionRangeSlice::new(2, true).unwrap(),
            ReductionRangeSlice::new(2, true).unwrap(),
        ];
        let crossed = vec![
            ReductionPairWords::pair(0, 2, 0).unwrap(),
            ReductionPairWords::pair(1, 3, 1).unwrap(),
        ];
        assert!(CheckedReductionPlan::new(4, ranges, crossed).is_err());
    }

    #[test]
    fn launch_binds_global_row_bases_without_rewriting_rows() {
        let plan =
            CheckedReductionPlan::canonical(vec![ReductionRangeSlice::new(3, true).unwrap()])
                .unwrap();
        let source = (70..73)
            .map(|row| ReductionWords::valid(row, [row; 8]))
            .collect();
        let launch = CheckedReductionLaunch::new(9, 0, 1, 1, 70, 900, 0, source, plan).unwrap();
        assert_eq!(launch.control().words()[7], 70);
        assert_eq!(launch.control().words()[9], 900);
        assert_eq!(launch.control().words()[11], 3);
        assert_eq!(launch.control().words()[12], 2);
        assert_eq!(launch.control().words()[13], 1);
    }

    fn valid_particle(task: u32, source: u32, host: u32, exponent: u32) -> ParticleWords {
        let mut words = [0u32; PARTICLE_WORDS];
        words[0] = ABI_VERSION;
        words[1] = 1;
        words[2] = task;
        words[3] = source;
        words[4] = 1;
        words[5] = 2;
        words[6] = 4;
        words[7] = 8;
        // Three complete future tricks leave one tile per hand. S2 leads the
        // empty current trick and is therefore the next non-focal actor.
        words[8] = 2 | (2 << 2) | (3 << 7) | (exponent << 13) | (24 << 17);
        words[9] = 31 | (31 << 5) | (31 << 10) | (31 << 15);
        words[11] = host;
        words[12] = 1;
        ParticleWords::try_from_words(words).unwrap()
    }

    #[test]
    fn field_launch_binds_task_exponent_actor_and_unique_host_ordinals() {
        let particles = vec![valid_particle(0, 0, 8, 9), valid_particle(0, 1, 9, 9)];
        let launch = CheckedFieldLaunch::new(1, 0, 1, 1, 9, 4, particles).unwrap();
        assert_eq!(launch.control().words()[6], 2);
        assert_eq!(launch.control().words()[8], 14);

        let duplicate = vec![valid_particle(0, 0, 8, 9), valid_particle(0, 1, 8, 9)];
        assert!(CheckedFieldLaunch::new(1, 0, 1, 1, 9, 4, duplicate).is_err());
    }
}
