use walt::rules::{Context, Decl, DominoSet};

use crate::{OpeningContext, OpeningError, OpeningRootV1, MAX_OPENING_MATCHING_COUNT_V1};

pub const MIN_REDUCED_GRADE_V1: u8 = 2;
pub const MAX_REDUCED_GRADE_V1: u8 = 5;
pub const MAX_REDUCED_MATCHING_COUNT_V1: u8 = MAX_OPENING_MATCHING_COUNT_V1 as u8;

/// One coordinate emitted by the frozen reduced opening carrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReducedOpeningCoordinateV1 {
    decl: Decl,
    grade: u8,
    led: Context,
    matching_count: u8,
    pool: DominoSet,
}

impl ReducedOpeningCoordinateV1 {
    pub const fn grade(self) -> u8 {
        self.grade
    }

    pub const fn led(self) -> Context {
        self.led
    }

    pub const fn matching_count(self) -> u8 {
        self.matching_count
    }

    pub const fn pool(self) -> DominoSet {
        self.pool
    }

    pub fn opening_context(self) -> Result<OpeningContext, OpeningError> {
        OpeningContext::try_reduced(self.decl, self.led, self.pool, self.grade)
    }
}

/// The generated reduced-grade carrier for one production opening root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReducedOpeningCarrierV1 {
    root: OpeningRootV1,
    coordinates: Vec<ReducedOpeningCoordinateV1>,
}

impl ReducedOpeningCarrierV1 {
    pub fn from_root(root: OpeningRootV1) -> Result<ReducedOpeningCarrierV1, OpeningError> {
        let root_pool = root.hidden_pool();
        let contexts = root.led_contexts();
        let mut coordinates = Vec::new();

        for grade in MIN_REDUCED_GRADE_V1..=MAX_REDUCED_GRADE_V1 {
            for &led in &contexts {
                let matching = root_pool.intersection(root.decl().effective_incidence(led));
                let nonmatching = root_pool.difference(matching);
                if matching.len() > usize::from(MAX_REDUCED_MATCHING_COUNT_V1) {
                    return Err(OpeningError::CarrierGenerationMismatch);
                }
                for matching_count in 0..=MAX_REDUCED_MATCHING_COUNT_V1 {
                    let selected_matching = usize::from(matching_count);
                    let target = usize::from(grade).checked_mul(3).ok_or(
                        OpeningError::ArithmeticOverflow("reduced carrier pool size"),
                    )?;
                    let Some(selected_nonmatching) = target.checked_sub(selected_matching) else {
                        continue;
                    };
                    if selected_matching > matching.len()
                        || selected_nonmatching > nonmatching.len()
                    {
                        continue;
                    }

                    // For a fixed category count, taking each category's
                    // earliest physical identities gives the lexicographically
                    // least fixed-cardinality subset of the root pool.
                    let pool: DominoSet = matching
                        .iter()
                        .take(selected_matching)
                        .chain(nonmatching.iter().take(selected_nonmatching))
                        .collect();
                    let context = OpeningContext::try_reduced(root.decl(), led, pool, grade)?;
                    if context.matching_pool().len() != selected_matching {
                        return Err(OpeningError::CarrierGenerationMismatch);
                    }
                    coordinates.push(ReducedOpeningCoordinateV1 {
                        decl: root.decl(),
                        grade,
                        led,
                        matching_count,
                        pool,
                    });
                }
            }
        }

        Ok(ReducedOpeningCarrierV1 { root, coordinates })
    }

    pub const fn root(&self) -> OpeningRootV1 {
        self.root
    }

    pub fn coordinates(&self) -> &[ReducedOpeningCoordinateV1] {
        &self.coordinates
    }
}
