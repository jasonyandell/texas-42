//! Transport-aware reachability plumbing: trace transport under the
//! ordered pip-trump transports and the 3-class declaration quotient for
//! reachable-census work.
//!
//! Exchange tier: the census quotient is licensed by x:004 (CONFIRMED
//! 2026-07-27, `exchange/README.md`): `f_{t,u}(R_t) = R_u` for every
//! ordered pip-trump pair, so reachable-census work needs only the three
//! unscored declaration classes of rec ALG-22/23. rob's S9 receipts are
//! finite conformance evidence for that theorem, not a proof of it.
//!
//! **Step-17 scope boundary (x:004), binding:** the transport preserves the
//! *unscored* relation surface only — never count labels, trick points,
//! contract outcomes, or anything score-conditioned. No API here transports
//! a scored object:
//!
//! ```compile_fail
//! use rob_core::{pip_trump_transport, TrickResult, PIPS};
//! fn forbidden(result: TrickResult) -> TrickResult {
//!     let t = pip_trump_transport(PIPS[2], PIPS[3]);
//!     rob_core::transport_trick_result(&t, result) // no such API exists
//! }
//! ```

use crate::algebra::transport::{
    unscored_mechanics_class, PipTrumpTransport, UnscoredMechanicsClass,
};
use crate::algebra::trick::Play;
use crate::declaration::Declaration;
use crate::domino::{DominoId, DominoSet};
use crate::support::normal_form::{Ambiguity, FeasibleSupportNormalForm, TotalSupportNormalForm};

/// The 3-class declaration quotient for reachable-census work (x:004;
/// consistent with S1's `unscored_mechanics_class` partition, rec
/// ALG-22/23): all seven pip trumps land in one class.
pub fn reachable_census_class(declaration: Declaration) -> UnscoredMechanicsClass {
    unscored_mechanics_class(declaration)
}

/// Transport one actor-attributed play (`f_{t,u}` extended pointwise;
/// actors are fixed — seats are not transported).
pub fn transport_play(transport: &PipTrumpTransport, play: Play) -> Play {
    Play {
        actor: play.actor,
        domino: transport.domino_map(play.domino),
    }
}

/// Transport a set of tiles pointwise.
pub fn transport_set(transport: &PipTrumpTransport, set: &DominoSet) -> DominoSet {
    DominoSet::from_ids(set.iter().map(|d| transport.domino_map(d)))
}

/// Transport a support normal form over a tile order: maps the pool
/// through `f_{t,u}` and re-sorts to the canonical identity order of the
/// image, permuting abstract tile indices accordingly. Seats, residuals,
/// and tags are untouched (the transport fixes the seat sort).
pub fn transport_normal_form(
    transport: &PipTrumpTransport,
    nf: &TotalSupportNormalForm,
    tile_order: &[DominoId],
) -> (TotalSupportNormalForm, Vec<DominoId>) {
    let mapped: Vec<DominoId> = tile_order
        .iter()
        .map(|&d| transport.domino_map(d))
        .collect();
    let mut sorted: Vec<DominoId> = mapped.clone();
    sorted.sort_by_key(|d| d.index());
    let position: Vec<usize> = mapped
        .iter()
        .map(|d| sorted.iter().position(|s| s == d).expect("bijective image"))
        .collect();
    let map_tiles = |tiles: &[usize]| -> Vec<usize> {
        let mut out: Vec<usize> = tiles.iter().map(|&t| position[t]).collect();
        out.sort_unstable();
        out
    };
    let transported = match nf {
        TotalSupportNormalForm::Empty => TotalSupportNormalForm::Empty,
        TotalSupportNormalForm::Feasible(nf) => {
            TotalSupportNormalForm::Feasible(FeasibleSupportNormalForm {
                certain_by_seat: core::array::from_fn(|s| map_tiles(&nf.certain_by_seat[s])),
                ambiguity: match &nf.ambiguity {
                    Ambiguity::Determinate => Ambiguity::Determinate,
                    Ambiguity::Binary {
                        inactive_seat,
                        pool,
                        first_active_residual,
                    } => Ambiguity::Binary {
                        inactive_seat: *inactive_seat,
                        pool: map_tiles(pool),
                        first_active_residual: *first_active_residual,
                    },
                    Ambiguity::Ternary {
                        pool,
                        residual0,
                        residual1,
                        excluded_seat,
                    } => {
                        let mut exclusions: Vec<(usize, usize)> = excluded_seat
                            .iter()
                            .map(|&(t, s)| (position[t], s))
                            .collect();
                        exclusions.sort_unstable();
                        Ambiguity::Ternary {
                            pool: map_tiles(pool),
                            residual0: *residual0,
                            residual1: *residual1,
                            excluded_seat: exclusions,
                        }
                    }
                },
            })
        }
    };
    (transported, sorted)
}
