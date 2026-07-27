//! The matching-minor calculus: the exact support transition on the normal
//! form (rec Math §7.14.1–7.14.2; TRANS-08..13; D5).
//!
//! Slice 01 was forbidden from exposing any transition API on standalone
//! support; this module implements exactly the typed calculus TRANS-08
//! licenses, and nothing untyped (INV-13): every hidden transition takes an
//! explicit [`TypedHiddenObservation`], and the game-level observation is
//! constructible only from declaration + led context + play through the
//! declaration algebra ([`game_observation`]). There is no method on any
//! normal-form type that consumes a bare domino:
//!
//! ```compile_fail
//! use rob_core::{FeasibleSupportNormalForm, DominoId};
//! fn forbidden(nf: &FeasibleSupportNormalForm, d: DominoId) {
//!     nf.transition(d); // no such untyped method exists (INV-13)
//! }
//! ```
//!
//! The calculus runs on the **total** NF type: `Empty` is a value on this
//! abstract/validation path (INV-6's slice-02 reading); the certified-path
//! variant panics instead of returning `Empty`.

use crate::algebra::algebra_for;
use crate::algebra::suits::LedSuit;
use crate::declaration::Declaration;
use crate::domino::DominoId;
use crate::support::cells::{AbstractCells, HIDDEN_SEATS};
use crate::support::normal_form::{
    compile_total_support, feasible_world, marginal_by_scc, TotalSupportNormalForm,
};

/// The kind of a typed hidden observation (Math §7.14.1 `E_o`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObservationKind {
    /// A hidden lead: conditions on possession only.
    Lead,
    /// A successful follow (`d ∈ F`): conditions on possession only — the
    /// tile itself is the witness (CELL-06).
    Follow,
    /// A failure to follow (`d ∉ F`): conditions on possession plus a
    /// complete void of the follow set.
    Slough,
}

/// A typed hidden observation (INV-13; TRANS-08): kind plus the abstract
/// follow set over the current support universe. The game-typed constructor
/// is [`game_observation`]; the abstract corpus builds these directly.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypedHiddenObservation {
    /// The observation kind.
    pub kind: ObservationKind,
    /// The follow set `F` as a membership vector over the current universe
    /// (`σ̂_q^δ ∩ U` at game level). Ignored for a lead; must contain the
    /// played tile for a follow and exclude it for a slough.
    pub follow_set: Vec<bool>,
}

/// One edge deletion `(predecessor-universe tile index, hidden seat)`.
pub type EdgeDeletion = (usize, usize);

/// The erasable audit record of one transition (INV-11 discipline): which
/// holder edges of the predecessor's reduced graph died. Audit data — never
/// part of semantic state or equality.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct DeletionRecord {
    /// Deleted edges in predecessor-universe indexing.
    pub deleted: Vec<EdgeDeletion>,
}

/// The matching-minor update (TRANS-09): force the edge `d→s`, for a slough
/// delete every edge `e→s` with `e ∈ F`, contract the played tile (remove
/// `d`, decrement the seat's quota), recompile the matching-supported core
/// (one assignment + one SCC pass), and re-encode the normal form.
///
/// Input and output universes follow the S4 reindexing convention: the
/// successor NF lives over `0..n-1` with tiles above the played index
/// shifted down by one. Returns the successor total NF and the deletion
/// record over the predecessor universe.
///
/// Equals extensional conditioning + pushforward (TRANS-08/09; verified
/// exhaustively by `r_dyn_observations`).
pub fn matching_minor_update(
    nf: &TotalSupportNormalForm,
    seat: usize,
    tile: usize,
    observation: &TypedHiddenObservation,
) -> (TotalSupportNormalForm, DeletionRecord) {
    let TotalSupportNormalForm::Feasible(nf) = nf else {
        return (TotalSupportNormalForm::Empty, DeletionRecord::default());
    };
    let cells = nf.decode();
    let n = cells.universe();
    assert!(tile < n, "tile index within the current universe");
    match observation.kind {
        ObservationKind::Lead => {}
        ObservationKind::Follow => {
            assert_eq!(observation.follow_set.len(), n);
            assert!(
                observation.follow_set[tile],
                "a successful follow plays a member of the follow set"
            );
        }
        ObservationKind::Slough => {
            assert_eq!(observation.follow_set.len(), n);
            assert!(
                !observation.follow_set[tile],
                "a slough plays a nonmember of the follow set"
            );
        }
    }

    // Force: possession requires the (marginal) edge d→s.
    if cells.capacity(seat) == 0 || !cells.possible(seat)[tile] {
        return (TotalSupportNormalForm::Empty, DeletionRecord::default());
    }

    // Delete (slough only): every edge e→s with e in the follow set.
    let mut conditioned = cells.clone();
    if observation.kind == ObservationKind::Slough {
        conditioned.retain_edges(|s2, e| !(s2 == seat && observation.follow_set[e]));
    }

    // Contract: remove the played tile, decrement the actor's quota.
    let successor = match conditioned.removal_update(seat, tile) {
        Ok(successor) => successor,
        Err(_) => {
            return (TotalSupportNormalForm::Empty, DeletionRecord::default());
        }
    };

    // Recompile the matching-supported core: one assignment + one SCC pass.
    let Some(witness) = feasible_world(&successor) else {
        return (
            TotalSupportNormalForm::Empty,
            deletion_ledger(&cells, None, tile),
        );
    };
    let marginal = marginal_by_scc(&successor, &witness);
    let reduced = AbstractCells::new(
        successor.universe(),
        marginal.clone(),
        core::array::from_fn(|s| successor.capacity(s)),
    )
    .expect("reduced successor keeps the structural schema");

    // Re-encode.
    let total = compile_total_support(&reduced, Some(&witness));
    debug_assert!(
        matches!(total, TotalSupportNormalForm::Feasible(_)),
        "a witnessed successor is feasible"
    );
    // INV-12 MONOTONE-AMBIGUITY + TRANS-10 monotonicity, asserted on every
    // nonempty successor in debug builds.
    #[cfg(debug_assertions)]
    if let TotalSupportNormalForm::Feasible(successor_nf) = &total {
        debug_assert_monotone(nf, successor_nf, tile, &marginal, &cells);
    }
    let ledger = deletion_ledger(&cells, Some(&marginal), tile);
    (total, ledger)
}

/// Predecessor-vs-successor edge accounting (INV-11): the played tile's
/// surviving edges die at contraction; every other predecessor edge that is
/// absent from the successor's reduced graph died informationally.
fn deletion_ledger(
    predecessor: &AbstractCells,
    successor_marginal: Option<&[Vec<bool>; HIDDEN_SEATS]>,
    tile: usize,
) -> DeletionRecord {
    let mut deleted = Vec::new();
    for s in 0..HIDDEN_SEATS {
        for e in 0..predecessor.universe() {
            if !predecessor.possible(s)[e] {
                continue;
            }
            let survives = if e == tile {
                false
            } else {
                let shifted = if e > tile { e - 1 } else { e };
                successor_marginal.map(|m| m[s][shifted]).unwrap_or(false)
            };
            if !survives {
                deleted.push((e, s));
            }
        }
    }
    DeletionRecord { deleted }
}

#[cfg(debug_assertions)]
fn debug_assert_monotone(
    predecessor: &crate::support::normal_form::FeasibleSupportNormalForm,
    successor: &crate::support::normal_form::FeasibleSupportNormalForm,
    tile: usize,
    successor_marginal: &[Vec<bool>; HIDDEN_SEATS],
    predecessor_cells: &AbstractCells,
) {
    // Holder-edge monotonicity (TRANS-10): every surviving tile's successor
    // holder set is a subset of its predecessor holder set.
    #[allow(clippy::needless_range_loop)] // s indexes marginal and cells in parallel
    for s in 0..HIDDEN_SEATS {
        for e in 0..predecessor_cells.universe() {
            if e == tile {
                continue;
            }
            let shifted = if e > tile { e - 1 } else { e };
            debug_assert!(
                !successor_marginal[s][shifted] || predecessor_cells.possible(s)[e],
                "holder edges never reappear (TRANS-10)"
            );
        }
    }
    // Ambiguity-phase monotonicity (TRANS-11 / INV-12): the tag moves only
    // downward and inactive seats never reactivate.
    debug_assert!(
        ambiguity_rank(successor) <= ambiguity_rank(predecessor),
        "ambiguity rank never increases (INV-12)"
    );
    let pre = predecessor.residuals();
    let post = successor.residuals();
    for s in 0..HIDDEN_SEATS {
        debug_assert!(
            !(pre[s] == 0 && post[s] > 0),
            "an inactive seat never re-enters the ambiguity component (INV-12)"
        );
    }
}

/// Ambiguity rank for INV-12: `Determinate < Binary < Ternary`.
pub fn ambiguity_rank(nf: &crate::support::normal_form::FeasibleSupportNormalForm) -> u8 {
    use crate::support::normal_form::Ambiguity;
    match nf.ambiguity {
        Ambiguity::Determinate => 0,
        Ambiguity::Binary { .. } => 1,
        Ambiguity::Ternary { .. } => 2,
    }
}

/// The certified-path variant (INV-6): on an internally certified state an
/// empty successor is an internal error, never a value.
pub fn certified_matching_minor_update(
    nf: &crate::support::normal_form::FeasibleSupportNormalForm,
    seat: usize,
    tile: usize,
    observation: &TypedHiddenObservation,
) -> (
    crate::support::normal_form::FeasibleSupportNormalForm,
    DeletionRecord,
) {
    let (total, ledger) = matching_minor_update(
        &TotalSupportNormalForm::Feasible(nf.clone()),
        seat,
        tile,
        observation,
    );
    match total {
        TotalSupportNormalForm::Feasible(successor) => (successor, ledger),
        TotalSupportNormalForm::Empty => {
            panic!("InvariantViolation: a certified observation has a nonempty successor (INV-6)")
        }
    }
}

/// Build the game-typed observation from the declaration algebra
/// (TRANS-08; the only game-level constructor, INV-13): the follow set is
/// `σ̂_q^δ ∩ U` expressed over the caller's current universe order; a lead
/// boundary (`led = None`) yields a lead observation.
pub fn game_observation(
    declaration: Declaration,
    led: Option<LedSuit>,
    tile: DominoId,
    universe_order: &[DominoId],
) -> TypedHiddenObservation {
    match led {
        None => TypedHiddenObservation {
            kind: ObservationKind::Lead,
            follow_set: vec![false; universe_order.len()],
        },
        Some(q) => {
            let algebra = algebra_for(declaration);
            let follow_set: Vec<bool> = universe_order
                .iter()
                .map(|&e| algebra.follows(e, q))
                .collect();
            let kind = if algebra.follows(tile, q) {
                ObservationKind::Follow
            } else {
                ObservationKind::Slough
            };
            TypedHiddenObservation { kind, follow_set }
        }
    }
}
