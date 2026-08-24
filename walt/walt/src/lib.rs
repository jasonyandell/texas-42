//! walt — the imperfect-information Texas 42 seat, unified (2026-08-24).
//!
//! One crate, seven modules, formerly seven crates (the fold is recorded
//! in `walt/UNIFICATION-CENSUS.md`; the freeze-56 v2 amendment covers the
//! layout change). Import direction is strict and acyclic, bottom-up:
//!
//! - [`rules`] — the straight-42 rules layer (formerly `walt-core`)
//! - [`kernel`] — viewer kernel + current-remainder fiber (`walt-kernel`)
//! - [`geom`] — exact rationals, PWL envelopes, features (`walt-geom`)
//! - [`strat`] — operators registry: PI/H/C/F, prices (`walt-strat`)
//! - [`spec`] — the M0 GPU ABI + semantic tables (`walt-gpu-spec`)
//! - [`carrier`] — the frozen hand-8 receipt carrier (`walt-m3-carrier`)
//! - [`solver`] — the seat solver, sampling stack (`walt-m3-probe`)
//!
//! Everything is exploratory tier; exact integers and rationals
//! throughout — no floats, no clocks in the value path.

pub mod carrier;
pub mod geom;
pub mod kernel;
pub mod rules;
pub mod solver;
pub mod spec;
pub mod strat;
