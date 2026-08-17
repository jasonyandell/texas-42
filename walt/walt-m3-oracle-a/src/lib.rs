//! Freeze-57 M3A independent H-authority adapter.
//!
//! This crate has one narrow job: present the canonical M3 carrier to a fresh
//! unmemoized [`walt_strat::ScalarHidden`] fixed-action solve for each frozen
//! physical root. It owns no production key, grouping, reduction, policy,
//! checkpoint, Metal, or objective-B code.

#![forbid(unsafe_code)]

use core::fmt;

use walt_core::{Domino, DominoSet, Seat, Team};
use walt_geom::{q, Q};
use walt_m3_carrier::{
    M3Carrier, OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL, ROOTS, SUPPORT_COUNT, TASKS, VIEWER,
};
use walt_strat::{FixedActionValueV1, ScalarHidden, ScalarValuation};

/// The exact nonreplenishing tree-v0 budget owned by each official root.
pub const M3A_ROOT_TREE_V0_CAP: u64 = 1u64 << 26;
pub const M3A_AUTHORITY_TASK_COUNT: usize = 4;

/// A typed M3A task. It cannot denote an objective-B task or an unfrozen root.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum M3aTask {
    Root21,
    Root31,
    Root33,
    Root55,
}

impl M3aTask {
    pub const ALL: [Self; M3A_AUTHORITY_TASK_COUNT] =
        [Self::Root21, Self::Root31, Self::Root33, Self::Root55];

    pub const fn task_ordinal(self) -> u32 {
        match self {
            Self::Root21 => 0,
            Self::Root31 => 1,
            Self::Root33 => 2,
            Self::Root55 => 3,
        }
    }

    pub const fn root_index(self) -> u32 {
        match self {
            Self::Root21 => 4,
            Self::Root31 => 7,
            Self::Root33 => 9,
            Self::Root55 => 20,
        }
    }

    pub const fn root(self) -> Domino {
        match self {
            Self::Root21 => ROOTS[0],
            Self::Root31 => ROOTS[1],
            Self::Root33 => ROOTS[2],
            Self::Root55 => ROOTS[3],
        }
    }
}

/// Canonical exact-rational fields for deterministic comparison or reporting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExactQFields {
    pub numerator: i128,
    pub denominator: i128,
}

/// One completed root-local M3A H-authority result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M3aAuthorityRecord {
    task: M3aTask,
    value: Q,
    visits: u64,
    epoch_visits: [u64; 4],
}

impl M3aAuthorityRecord {
    pub const fn task(&self) -> M3aTask {
        self.task
    }

    pub const fn task_ordinal(&self) -> u32 {
        self.task.task_ordinal()
    }

    pub const fn objective(&self) -> u32 {
        OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL
    }

    pub const fn root_index(&self) -> u32 {
        self.task.root_index()
    }

    pub const fn root(&self) -> Domino {
        self.task.root()
    }

    pub const fn value(&self) -> &Q {
        &self.value
    }

    pub fn exact_q_fields(&self) -> ExactQFields {
        ExactQFields {
            numerator: *self.value.numer(),
            denominator: *self.value.denom(),
        }
    }

    pub const fn visits(&self) -> u64 {
        self.visits
    }

    pub const fn epoch_visits(&self) -> [u64; 4] {
        self.epoch_visits
    }

    pub const fn budget_remaining(&self) -> u64 {
        M3A_ROOT_TREE_V0_CAP - self.visits
    }
}

/// The four completed authority records in frozen task/root order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct M3aAuthorityRun {
    records: [M3aAuthorityRecord; M3A_AUTHORITY_TASK_COUNT],
}

impl M3aAuthorityRun {
    pub const fn records(&self) -> &[M3aAuthorityRecord; M3A_AUTHORITY_TASK_COUNT] {
        &self.records
    }

    pub fn record(&self, task: M3aTask) -> &M3aAuthorityRecord {
        &self.records[task.task_ordinal() as usize]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum M3aOracleError {
    Carrier(&'static str),
    VisitCapExceeded { task: M3aTask, cap: u64 },
    VisitInvariant { task: M3aTask },
    ValueRange { task: M3aTask },
}

impl fmt::Display for M3aOracleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Carrier(field) => write!(formatter, "M3A carrier mismatch at {field}"),
            Self::VisitCapExceeded { task, cap } => {
                write!(formatter, "M3A {task:?} exceeded its tree-v0 cap {cap}")
            }
            Self::VisitInvariant { task } => {
                write!(formatter, "M3A {task:?} violated its visit partition")
            }
            Self::ValueRange { task } => {
                write!(formatter, "M3A {task:?} returned a value outside [-4,4]")
            }
        }
    }
}

impl std::error::Error for M3aOracleError {}

/// Evaluate one fixed physical root with a newly constructed scalar oracle and
/// a newly initialized, nonreplenishing `2^26` tree-v0 counter.
pub fn evaluate_m3a_task(
    carrier: &M3Carrier,
    task: M3aTask,
) -> Result<M3aAuthorityRecord, M3aOracleError> {
    let worlds = canonical_worlds(carrier)?;
    evaluate_with_cap(carrier, &worlds, task, M3A_ROOT_TREE_V0_CAP)
}

/// Evaluate all four roots. The carrier world array may be shared as immutable
/// input, but each record is produced by a fresh solver and fresh root-local
/// budget; no value, recursion state, memo, or counter crosses roots.
pub fn evaluate_all_m3a(carrier: &M3Carrier) -> Result<M3aAuthorityRun, M3aOracleError> {
    let worlds = canonical_worlds(carrier)?;
    let root21 = evaluate_with_cap(carrier, &worlds, M3aTask::Root21, M3A_ROOT_TREE_V0_CAP)?;
    let root31 = evaluate_with_cap(carrier, &worlds, M3aTask::Root31, M3A_ROOT_TREE_V0_CAP)?;
    let root33 = evaluate_with_cap(carrier, &worlds, M3aTask::Root33, M3A_ROOT_TREE_V0_CAP)?;
    let root55 = evaluate_with_cap(carrier, &worlds, M3aTask::Root55, M3A_ROOT_TREE_V0_CAP)?;
    Ok(M3aAuthorityRun {
        records: [root21, root31, root33, root55],
    })
}

fn canonical_worlds(
    carrier: &M3Carrier,
) -> Result<Box<[[DominoSet; Seat::COUNT]]>, M3aOracleError> {
    if carrier.support().len() != SUPPORT_COUNT {
        return Err(M3aOracleError::Carrier("support count"));
    }
    let worlds: Vec<[DominoSet; Seat::COUNT]> = carrier
        .support()
        .records()
        .iter()
        .copied()
        .map(|record| record.hands())
        .collect();
    if worlds.len() != SUPPORT_COUNT {
        return Err(M3aOracleError::Carrier("world conversion count"));
    }
    Ok(worlds.into_boxed_slice())
}

fn evaluate_with_cap(
    carrier: &M3Carrier,
    worlds: &[[DominoSet; Seat::COUNT]],
    task: M3aTask,
    cap: u64,
) -> Result<M3aAuthorityRecord, M3aOracleError> {
    validate_frame(carrier, task)?;
    let solver = ScalarHidden::new(
        carrier.facts().declaration,
        VIEWER,
        Team::T1,
        ScalarValuation::trick_only(),
    );
    let mut budget = cap;
    let FixedActionValueV1 {
        value,
        visits,
        epoch_visits,
    } = solver
        .fixed_action_value(
            worlds,
            carrier.facts().next_leader,
            &[],
            task.root(),
            &mut budget,
        )
        .ok_or(M3aOracleError::VisitCapExceeded { task, cap })?;

    let partition_sum = epoch_visits
        .into_iter()
        .try_fold(0u64, u64::checked_add)
        .ok_or(M3aOracleError::VisitInvariant { task })?;
    let consumed = cap
        .checked_sub(budget)
        .ok_or(M3aOracleError::VisitInvariant { task })?;
    if visits == 0 || visits != consumed || visits != partition_sum || visits > cap {
        return Err(M3aOracleError::VisitInvariant { task });
    }
    if value < q(-4, 1) || value > q(4, 1) {
        return Err(M3aOracleError::ValueRange { task });
    }

    Ok(M3aAuthorityRecord {
        task,
        value,
        visits,
        epoch_visits,
    })
}

fn validate_frame(carrier: &M3Carrier, task: M3aTask) -> Result<(), M3aOracleError> {
    let ordinal = task.task_ordinal() as usize;
    let metadata = TASKS
        .get(ordinal)
        .ok_or(M3aOracleError::Carrier("task ordinal"))?;
    if metadata.ordinal != task.task_ordinal()
        || metadata.objective != OBJECTIVE_M3A_FUTURE_TRICK_DIFFERENTIAL
        || metadata.root_index != task.root_index()
    {
        return Err(M3aOracleError::Carrier("task metadata"));
    }
    if carrier.facts().viewer != VIEWER
        || carrier.facts().next_leader != VIEWER
        || carrier.facts().declaring_team != Team::T1
    {
        return Err(M3aOracleError::Carrier("viewer/focal frame"));
    }
    if !carrier.facts().viewer_hand.contains(task.root())
        || task.root().index() != task.root_index() as usize
    {
        return Err(M3aOracleError::Carrier("fixed physical root"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use walt_core::{Decl, Pip};

    fn one_trick_carrier_frame() -> (ScalarHidden, [[DominoSet; Seat::COUNT]; 1], Domino) {
        let root = Domino::ALL[20];
        let mut world = [DominoSet::EMPTY; Seat::COUNT];
        world[Seat::S1.index()] = DominoSet::single(root);
        world[Seat::S2.index()] = DominoSet::single(Domino::ALL[0]);
        world[Seat::S3.index()] = DominoSet::single(Domino::ALL[1]);
        world[Seat::S0.index()] = DominoSet::single(Domino::ALL[2]);
        let solver = ScalarHidden::new(
            Decl::PipTrump(Pip::new(5).expect("canonical pip")),
            Seat::S1,
            Team::T1,
            ScalarValuation::trick_only(),
        );
        (solver, [world], root)
    }

    #[test]
    fn cap_failure_is_total_and_has_no_authority_record() {
        let (solver, worlds, root) = one_trick_carrier_frame();
        let mut budget = 0;
        assert!(solver
            .fixed_action_value(&worlds, Seat::S1, &[], root, &mut budget)
            .is_none());
        assert_eq!(budget, 0);
    }
}
