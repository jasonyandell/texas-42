use std::collections::BTreeMap;

use core::cmp::Ordering;

use walt::kernel::{Hidden, Kernel};
use walt::rules::{legal_plays, ContextSet, Domino, DominoSet, Seat};
use walt::spec::{OpeningLikelihoodCoeff, ScaledOpeningMass, SupportCount, FIELD_SCALE};

use crate::context::{OpeningContext, OpeningError, M1_DIRECT_WORLD_CAP_V1};

/// Full-grade physical opening worlds after one fixed focal hand is removed.
pub const OPENING_DEAL_COUNT: u64 = 399_072_960;
/// Frozen maximum number of nonempty cells in one lawful opening projection.
pub const MAX_OPENING_CELLS_V1: usize = 11_730;

/// Typed outcome of the complete-world preflight for the independent arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectPreflightV1 {
    Admitted { world_count: u64, cap: u64 },
    DeclaredStop { world_count: u64, cap: u64 },
}

impl DirectPreflightV1 {
    pub const fn world_count(self) -> u64 {
        match self {
            DirectPreflightV1::Admitted { world_count, .. }
            | DirectPreflightV1::DeclaredStop { world_count, .. } => world_count,
        }
    }

    pub const fn cap(self) -> u64 {
        match self {
            DirectPreflightV1::Admitted { cap, .. }
            | DirectPreflightV1::DeclaredStop { cap, .. } => cap,
        }
    }
}

/// The semantic role of one response relative to the selected led context.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResponseRole {
    Follower,
    Void,
}

/// Canonical cell identity: response triple first, then post-play matching counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OpeningCellKey {
    response: [Domino; 3],
    matching_counts: [u8; 3],
}

impl Ord for OpeningCellKey {
    fn cmp(&self, other: &Self) -> Ordering {
        for seat in 0..3 {
            match self.response[seat]
                .index()
                .cmp(&other.response[seat].index())
            {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        self.matching_counts.cmp(&other.matching_counts)
    }
}

impl PartialOrd for OpeningCellKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OpeningCellKey {
    fn new(response: [Domino; 3], matching_counts: [u8; 3]) -> OpeningCellKey {
        OpeningCellKey {
            response,
            matching_counts,
        }
    }

    pub const fn response(self) -> [Domino; 3] {
        self.response
    }

    pub const fn matching_counts(self) -> [u8; 3] {
        self.matching_counts
    }

    pub fn roles(self, context: OpeningContext) -> [ResponseRole; 3] {
        let matching = context.matching_pool();
        self.response.map(|tile| {
            if matching.contains(tile) {
                ResponseRole::Follower
            } else {
                ResponseRole::Void
            }
        })
    }

    pub fn remaining_pool(self, context: OpeningContext) -> DominoSet {
        self.response
            .into_iter()
            .fold(context.pool(), |pool, tile| {
                pool.difference(DominoSet::single(tile))
            })
    }

    pub fn remaining_matching_mask(self, context: OpeningContext) -> DominoSet {
        self.response
            .into_iter()
            .fold(context.matching_pool(), |matching, tile| {
                matching.difference(DominoSet::single(tile))
            })
    }
}

/// One exact, uniformly weighted opening allocation cell.
///
/// Remaining masks and capacities are derived from the validated context and
/// key instead of being stored as a second authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpeningCell {
    key: OpeningCellKey,
    support: SupportCount,
    per_world_coefficient: OpeningLikelihoodCoeff,
}

impl OpeningCell {
    fn new(
        key: OpeningCellKey,
        support: SupportCount,
        per_world_coefficient: OpeningLikelihoodCoeff,
    ) -> Result<OpeningCell, OpeningError> {
        per_world_coefficient
            .checked_scale_by_support(support)
            .ok_or(OpeningError::ArithmeticOverflow("opening cell mass"))?;
        Ok(OpeningCell {
            key,
            support,
            per_world_coefficient,
        })
    }

    pub const fn key(self) -> OpeningCellKey {
        self.key
    }

    pub const fn support(self) -> SupportCount {
        self.support
    }

    pub const fn per_world_coefficient(self) -> OpeningLikelihoodCoeff {
        self.per_world_coefficient
    }

    pub fn scaled_mass(self) -> Result<ScaledOpeningMass, OpeningError> {
        self.per_world_coefficient
            .checked_scale_by_support(self.support)
            .ok_or(OpeningError::ArithmeticOverflow("opening cell mass"))
    }

    pub fn remaining_pool(self, context: OpeningContext) -> DominoSet {
        self.key.remaining_pool(context)
    }

    pub fn remaining_matching_mask(self, context: OpeningContext) -> DominoSet {
        self.key.remaining_matching_mask(context)
    }

    pub fn remaining_capacities(self, context: OpeningContext) -> [u8; 3] {
        [context.grade() - 1; 3]
    }
}

/// Canonically ordered opening cells with their validated projection input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpeningProjection {
    context: OpeningContext,
    cells: Vec<OpeningCell>,
}

impl OpeningProjection {
    fn from_cells(
        context: OpeningContext,
        mut cells: Vec<OpeningCell>,
    ) -> Result<OpeningProjection, OpeningError> {
        if cells.len() > MAX_OPENING_CELLS_V1 {
            return Err(OpeningError::OpeningCellCapExceeded {
                cell_count: cells.len(),
                cap: MAX_OPENING_CELLS_V1,
            });
        }
        cells.sort_unstable_by_key(|cell| cell.key);
        if cells.windows(2).any(|pair| pair[0].key == pair[1].key) {
            return Err(OpeningError::DuplicateCell);
        }
        let projection = OpeningProjection { context, cells };
        let actual = projection.total_scaled_mass()?.value();
        let expected = projection.expected_scaled_mass()?.value();
        if actual != expected {
            return Err(OpeningError::MassConservationMismatch { expected, actual });
        }
        Ok(projection)
    }

    pub const fn context(&self) -> OpeningContext {
        self.context
    }

    pub fn cells(&self) -> &[OpeningCell] {
        &self.cells
    }

    pub fn total_scaled_mass(&self) -> Result<ScaledOpeningMass, OpeningError> {
        let mut total = ScaledOpeningMass::new(0);
        for cell in &self.cells {
            total = total
                .checked_add(cell.scaled_mass()?)
                .ok_or(OpeningError::ArithmeticOverflow("opening projection mass"))?;
        }
        Ok(total)
    }

    pub fn expected_scaled_mass(&self) -> Result<ScaledOpeningMass, OpeningError> {
        let field_cube = u64::from(FIELD_SCALE)
            .checked_pow(3)
            .ok_or(OpeningError::ArithmeticOverflow("opening field scale"))?;
        self.context
            .physical_world_count()?
            .checked_mul(field_cube)
            .map(ScaledOpeningMass::new)
            .ok_or(OpeningError::ArithmeticOverflow("expected opening mass"))
    }

    /// Canonical response-level support and mass subtotals.
    ///
    /// Cells are already ordered response-first, so this is a single checked
    /// fold with no map ordering or completion-order dependence.
    pub fn response_aggregates(&self) -> Result<Vec<OpeningResponseAggregate>, OpeningError> {
        let mut aggregates: Vec<OpeningResponseAggregate> = Vec::new();
        for cell in &self.cells {
            let response = cell.key().response();
            let cell_mass = cell.scaled_mass()?.value();
            match aggregates.last_mut() {
                Some(aggregate) if aggregate.response == response => {
                    aggregate.support = aggregate
                        .support
                        .checked_add(u64::from(cell.support().value()))
                        .ok_or(OpeningError::ArithmeticOverflow("response support"))?;
                    aggregate.scaled_mass = aggregate
                        .scaled_mass
                        .checked_add(cell_mass)
                        .ok_or(OpeningError::ArithmeticOverflow("response mass"))?;
                }
                _ => aggregates.push(OpeningResponseAggregate {
                    response,
                    support: u64::from(cell.support().value()),
                    scaled_mass: cell_mass,
                }),
            }
        }
        let folded_mass = aggregates.iter().try_fold(0u64, |total, aggregate| {
            total
                .checked_add(aggregate.scaled_mass)
                .ok_or(OpeningError::ArithmeticOverflow("response-to-global mass"))
        })?;
        let global_mass = self.total_scaled_mass()?.value();
        if folded_mass != global_mass {
            return Err(OpeningError::MassConservationMismatch {
                expected: global_mass,
                actual: folded_mass,
            });
        }
        Ok(aggregates)
    }
}

/// One response triple's checked subtotal over all matching-count cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpeningResponseAggregate {
    response: [Domino; 3],
    support: u64,
    scaled_mass: u64,
}

impl OpeningResponseAggregate {
    pub const fn response(self) -> [Domino; 3] {
        self.response
    }

    pub const fn support(self) -> u64 {
        self.support
    }

    pub const fn scaled_mass(self) -> ScaledOpeningMass {
        ScaledOpeningMass::new(self.scaled_mass)
    }
}

/// Independently computes the closed-form `(response, e)` opening cells.
pub fn project_closed_form(context: OpeningContext) -> Result<OpeningProjection, OpeningError> {
    let tiles: Vec<Domino> = context.pool().iter().collect();
    let matching = context.matching_pool();
    let mut cells = Vec::new();

    for &first in &tiles {
        for &second in &tiles {
            if second == first {
                continue;
            }
            for &third in &tiles {
                if third == first || third == second {
                    continue;
                }
                let response = [first, second, third];
                let roles = response.map(|tile| {
                    if matching.contains(tile) {
                        ResponseRole::Follower
                    } else {
                        ResponseRole::Void
                    }
                });
                let follower_count = roles
                    .iter()
                    .filter(|role| **role == ResponseRole::Follower)
                    .count();
                let remaining_matching = matching.len().checked_sub(follower_count).ok_or(
                    OpeningError::ArithmeticOverflow("remaining matching response count"),
                )?;
                let vectors = matching_count_vectors(
                    roles,
                    remaining_matching,
                    usize::from(context.grade() - 1),
                )?;
                for counts in vectors {
                    let key = OpeningCellKey::new(response, counts);
                    let Some(support) = compatible_world_count(context, key)? else {
                        continue;
                    };
                    let coefficient = per_world_coefficient(context, key)?;
                    cells.push(OpeningCell::new(key, support, coefficient)?);
                }
            }
        }
    }

    OpeningProjection::from_cells(context, cells)
}

/// Enumerates physical worlds and legal response paths without using the closed form.
///
/// Exact preflight work accounting prevents accidental traversal of an
/// oversized fiber and guarantees that a declared stop returns no partial
/// projection.  M1 parity uses grades two through four.
pub fn project_direct(context: OpeningContext) -> Result<OpeningProjection, OpeningError> {
    let (kernel, expected_worlds) = checked_direct_kernel(context)?;
    if expected_worlds > M1_DIRECT_WORLD_CAP_V1 {
        return Err(OpeningError::DirectWorldCapExceeded {
            world_count: expected_worlds,
            cap: M1_DIRECT_WORLD_CAP_V1,
        });
    }

    let matching = context.matching_pool();
    let mut aggregates: BTreeMap<OpeningCellKey, DirectAggregate> = BTreeMap::new();
    let mut worlds_seen = 0u128;

    for world in kernel.worlds() {
        worlds_seen = worlds_seen
            .checked_add(1)
            .ok_or(OpeningError::ArithmeticOverflow("direct world count"))?;
        let hands = [
            world.hand(Seat::S1),
            world.hand(Seat::S2),
            world.hand(Seat::S3),
        ];
        let legal: [DominoSet; 3] =
            hands.map(|hand| legal_plays(context.decl(), hand, Some(context.led())));
        let coefficient = direct_path_coefficient(legal)?;

        for first in legal[0].iter() {
            for second in legal[1].iter() {
                for third in legal[2].iter() {
                    let response = [first, second, third];
                    let mut counts = [0u8; 3];
                    for seat in 0..3 {
                        let matching_in_hand = hands[seat].intersection(matching);
                        let expected_legal = if matching_in_hand.is_empty() {
                            hands[seat]
                        } else {
                            matching_in_hand
                        };
                        if legal[seat] != expected_legal || !legal[seat].contains(response[seat]) {
                            return Err(OpeningError::InconsistentDirectAggregate);
                        }
                        if !matching_in_hand.is_empty() {
                            let remaining = matching_in_hand.len().checked_sub(1).ok_or(
                                OpeningError::ArithmeticOverflow("direct matching response count"),
                            )?;
                            counts[seat] = u8::try_from(remaining).map_err(|_| {
                                OpeningError::ArithmeticOverflow("direct matching response width")
                            })?;
                        }
                    }
                    let key = OpeningCellKey::new(response, counts);
                    aggregates.entry(key).or_default().add_path(coefficient)?;
                }
            }
        }
    }

    if worlds_seen != u128::from(expected_worlds) || worlds_seen != kernel.count() {
        return Err(OpeningError::InconsistentDirectAggregate);
    }

    let mut cells = Vec::with_capacity(aggregates.len());
    for (key, aggregate) in aggregates {
        cells.push(aggregate.into_cell(key)?);
    }
    OpeningProjection::from_cells(context, cells)
}

/// Computes and cross-checks the complete direct work count without emitting
/// the first world.  A grade-five result is a typed declared stop, not a
/// partial projection and not an omitted test.
pub fn direct_preflight(context: OpeningContext) -> Result<DirectPreflightV1, OpeningError> {
    let (_kernel, world_count) = checked_direct_kernel(context)?;
    if world_count > M1_DIRECT_WORLD_CAP_V1 {
        Ok(DirectPreflightV1::DeclaredStop {
            world_count,
            cap: M1_DIRECT_WORLD_CAP_V1,
        })
    } else {
        Ok(DirectPreflightV1::Admitted {
            world_count,
            cap: M1_DIRECT_WORLD_CAP_V1,
        })
    }
}

fn checked_direct_kernel(context: OpeningContext) -> Result<(Kernel, u64), OpeningError> {
    let formula_count = context.physical_world_count()?;
    let grade = usize::from(context.grade());
    let hidden = [
        Hidden {
            seat: Seat::S1,
            capacity: grade,
            voids: ContextSet::EMPTY,
        },
        Hidden {
            seat: Seat::S2,
            capacity: grade,
            voids: ContextSet::EMPTY,
        },
        Hidden {
            seat: Seat::S3,
            capacity: grade,
            voids: ContextSet::EMPTY,
        },
    ];
    let kernel = Kernel::new(
        context.decl(),
        Seat::S0,
        DominoSet::EMPTY,
        context.pool(),
        hidden,
    )?;
    let kernel_count = u64::try_from(kernel.count())
        .map_err(|_| OpeningError::ArithmeticOverflow("kernel world count width"))?;
    if formula_count != kernel_count {
        return Err(OpeningError::KernelWorldCountMismatch {
            formula_count,
            kernel_count,
        });
    }
    Ok((kernel, formula_count))
}

#[derive(Default)]
struct DirectAggregate {
    support: u64,
    coefficient: Option<u64>,
    accumulated_mass: u64,
}

impl DirectAggregate {
    fn add_path(&mut self, coefficient: u64) -> Result<(), OpeningError> {
        match self.coefficient {
            Some(existing) if existing != coefficient => {
                return Err(OpeningError::InconsistentDirectAggregate);
            }
            Some(_) => {}
            None => self.coefficient = Some(coefficient),
        }
        self.support = self
            .support
            .checked_add(1)
            .ok_or(OpeningError::ArithmeticOverflow("direct cell support"))?;
        self.accumulated_mass = self
            .accumulated_mass
            .checked_add(coefficient)
            .ok_or(OpeningError::ArithmeticOverflow("direct cell mass"))?;
        Ok(())
    }

    fn into_cell(self, key: OpeningCellKey) -> Result<OpeningCell, OpeningError> {
        let support = u32::try_from(self.support).map_err(|_| OpeningError::SupportTooWide {
            value: self.support,
        })?;
        let support =
            SupportCount::new(support).ok_or(OpeningError::InconsistentDirectAggregate)?;
        let coefficient = OpeningLikelihoodCoeff::new(
            self.coefficient
                .ok_or(OpeningError::InconsistentDirectAggregate)?,
        );
        let expected_mass = coefficient
            .checked_scale_by_support(support)
            .ok_or(OpeningError::ArithmeticOverflow("direct aggregate mass"))?;
        if expected_mass.value() != self.accumulated_mass {
            return Err(OpeningError::InconsistentDirectAggregate);
        }
        OpeningCell::new(key, support, coefficient)
    }
}

fn matching_count_vectors(
    roles: [ResponseRole; 3],
    total: usize,
    remaining_capacity: usize,
) -> Result<Vec<[u8; 3]>, OpeningError> {
    let limits = roles.map(|role| match role {
        ResponseRole::Follower => remaining_capacity,
        ResponseRole::Void => 0,
    });
    let mut vectors = Vec::new();
    for first in 0..=limits[0] {
        for second in 0..=limits[1] {
            for third in 0..=limits[2] {
                if first + second + third == total {
                    vectors.push([
                        u8::try_from(first).map_err(|_| {
                            OpeningError::ArithmeticOverflow("matching-count width")
                        })?,
                        u8::try_from(second).map_err(|_| {
                            OpeningError::ArithmeticOverflow("matching-count width")
                        })?,
                        u8::try_from(third).map_err(|_| {
                            OpeningError::ArithmeticOverflow("matching-count width")
                        })?,
                    ]);
                }
            }
        }
    }
    Ok(vectors)
}

fn compatible_world_count(
    context: OpeningContext,
    key: OpeningCellKey,
) -> Result<Option<SupportCount>, OpeningError> {
    let roles = key.roles(context);
    let counts = key.matching_counts();
    let follower_count = roles
        .iter()
        .filter(|role| **role == ResponseRole::Follower)
        .count();
    let mut matching_left = context
        .matching_pool()
        .len()
        .checked_sub(follower_count)
        .ok_or(OpeningError::ArithmeticOverflow(
            "closed-form matching pool",
        ))?;
    let mut support = 1u64;

    for seat in 0..3 {
        let count = usize::from(counts[seat]);
        match roles[seat] {
            ResponseRole::Follower => {
                support = support
                    .checked_mul(checked_binomial(matching_left, count)?)
                    .ok_or(OpeningError::ArithmeticOverflow(
                        "matching allocation support",
                    ))?;
                matching_left =
                    matching_left
                        .checked_sub(count)
                        .ok_or(OpeningError::ArithmeticOverflow(
                            "matching allocation remainder",
                        ))?;
            }
            ResponseRole::Void if count != 0 => return Ok(None),
            ResponseRole::Void => {}
        }
    }
    if matching_left != 0 {
        return Ok(None);
    }

    let nonmatching_responses =
        3usize
            .checked_sub(follower_count)
            .ok_or(OpeningError::ArithmeticOverflow(
                "nonmatching response count",
            ))?;
    let mut nonmatching_left = context
        .pool()
        .len()
        .checked_sub(context.matching_pool().len())
        .and_then(|count| count.checked_sub(nonmatching_responses))
        .ok_or(OpeningError::ArithmeticOverflow(
            "closed-form nonmatching pool",
        ))?;
    let remaining_capacity = usize::from(context.grade() - 1);
    for count in counts {
        let needed = remaining_capacity.checked_sub(usize::from(count)).ok_or(
            OpeningError::ArithmeticOverflow("nonmatching seat capacity"),
        )?;
        support = support
            .checked_mul(checked_binomial(nonmatching_left, needed)?)
            .ok_or(OpeningError::ArithmeticOverflow(
                "nonmatching allocation support",
            ))?;
        nonmatching_left =
            nonmatching_left
                .checked_sub(needed)
                .ok_or(OpeningError::ArithmeticOverflow(
                    "nonmatching allocation remainder",
                ))?;
    }
    if nonmatching_left != 0 {
        return Ok(None);
    }

    let support =
        u32::try_from(support).map_err(|_| OpeningError::SupportTooWide { value: support })?;
    Ok(SupportCount::new(support))
}

fn per_world_coefficient(
    context: OpeningContext,
    key: OpeningCellKey,
) -> Result<OpeningLikelihoodCoeff, OpeningError> {
    let roles = key.roles(context);
    let counts = key.matching_counts();
    let mut coefficient = 1u64;
    for seat in 0..3 {
        let denominator = match roles[seat] {
            ResponseRole::Follower => u32::from(counts[seat])
                .checked_add(1)
                .ok_or(OpeningError::ArithmeticOverflow("follower legal-set size"))?,
            ResponseRole::Void => u32::from(context.grade()),
        };
        let factor = checked_scale_factor(denominator)?;
        coefficient =
            coefficient
                .checked_mul(u64::from(factor))
                .ok_or(OpeningError::ArithmeticOverflow(
                    "per-world opening coefficient",
                ))?;
    }
    Ok(OpeningLikelihoodCoeff::new(coefficient))
}

fn direct_path_coefficient(legal: [DominoSet; 3]) -> Result<u64, OpeningError> {
    let mut coefficient = 1u64;
    for actions in legal {
        let denominator = u32::try_from(actions.len())
            .map_err(|_| OpeningError::ArithmeticOverflow("legal-set size"))?;
        let factor = checked_scale_factor(denominator)?;
        coefficient = coefficient
            .checked_mul(u64::from(factor))
            .ok_or(OpeningError::ArithmeticOverflow("direct path coefficient"))?;
    }
    Ok(coefficient)
}

fn checked_scale_factor(denominator: u32) -> Result<u32, OpeningError> {
    if denominator == 0 || !FIELD_SCALE.is_multiple_of(denominator) {
        return Err(OpeningError::ScaleDoesNotClear { denominator });
    }
    Ok(FIELD_SCALE / denominator)
}

pub(crate) fn checked_binomial(n: usize, k: usize) -> Result<u64, OpeningError> {
    if k > n {
        return Ok(0);
    }
    let k = k.min(n - k);
    let mut value = 1u64;
    for step in 0..k {
        let numerator = u64::try_from(n - step)
            .map_err(|_| OpeningError::ArithmeticOverflow("binomial numerator"))?;
        let denominator = u64::try_from(step + 1)
            .map_err(|_| OpeningError::ArithmeticOverflow("binomial denominator"))?;
        let product = value
            .checked_mul(numerator)
            .ok_or(OpeningError::ArithmeticOverflow("binomial product"))?;
        if product % denominator != 0 {
            return Err(OpeningError::InconsistentDirectAggregate);
        }
        value = product / denominator;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use walt::rules::{Decl, Domino, DominoSet, Pip};

    #[test]
    fn opening_projection_rejects_cells_above_the_frozen_cap() {
        let decl = Decl::NoTrump;
        let led = walt::rules::Context::Natural(Pip::new(6).expect("six is a pip"));
        let pool: DominoSet = Domino::ALL.into_iter().take(3).collect();
        let context = OpeningContext::try_reduced(decl, led, pool, 1)
            .expect("three nonmatching tiles form a bounded synthetic context");
        let response: [Domino; 3] = pool
            .iter()
            .collect::<Vec<_>>()
            .try_into()
            .expect("the pool has exactly three tiles");
        let cell = OpeningCell::new(
            OpeningCellKey::new(response, [0; 3]),
            SupportCount::new(1).expect("positive synthetic support"),
            OpeningLikelihoodCoeff::new(u64::from(FIELD_SCALE).pow(3)),
        )
        .expect("synthetic cell mass fits");

        assert!(matches!(
            OpeningProjection::from_cells(context, vec![cell; MAX_OPENING_CELLS_V1 + 1]),
            Err(OpeningError::OpeningCellCapExceeded {
                cell_count,
                cap: MAX_OPENING_CELLS_V1,
            }) if cell_count == MAX_OPENING_CELLS_V1 + 1
        ));
    }
}
