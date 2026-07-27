//! Symbolic trace reachability: deal-free certificates and their validator
//! (rec Math §7.13.7; REACH-14/15; v0.7 Exec §18).
//!
//! A [`SymbolicTraceCertificate`] carries no hidden deal: viewer hand,
//! contract/declaration, first leader, the actor-attributed public trace,
//! and the claimed final support. [`validate_symbolic_trace`] replays it
//! from the unrestricted 21-tile support, accepting a hidden action exactly
//! when its typed conditioned successor support is nonempty and a viewer
//! action exactly when it is legal in the known hand. By the symbolic trace
//! equivalence theorem this accepts exactly the traces some complete deal
//! legally realizes, and the final support is exactly the trace's fiber.
//!
//! This is the **only** gate by which a foreign support claim is accepted
//! (OPEN-12: it still carries legal ancestry — no support-only membership
//! criterion exists). On acceptance the certificate is erased (D1): the
//! result type retains no trace, no witness, and no path.

use crate::algebra::algebra_for;
use crate::algebra::trick::Play;
use crate::domino::{all_ids, DominoId, DominoSet};
use crate::objective::contract::Contract;
use crate::seat::Seat;
use crate::support::cells::HIDDEN_SEATS;
use crate::support::dynamics::{game_observation, matching_minor_update, DeletionRecord};
use crate::support::normal_form::TotalSupportNormalForm;

/// A deal-free reachability certificate (REACH-14; Exec §18).
#[derive(Clone, Debug)]
pub struct SymbolicTraceCertificate {
    /// The viewing seat.
    pub viewer: Seat,
    /// The viewer's initial seven-tile hand.
    pub viewer_initial_hand: DominoSet,
    /// The certified contract (declaration; its bidder leads trick one).
    pub contract: Contract,
    /// The actor-attributed public play trace.
    pub trace: Vec<Play>,
    /// The claimed final hidden pool.
    pub claimed_pool: DominoSet,
    /// The claimed final support normal form over the claimed pool's
    /// canonical identity order.
    pub claimed_final: TotalSupportNormalForm,
}

/// Typed rejection reasons (Exec §18/§24; never coerced into acceptance).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SymbolicRejection {
    /// The viewer hand is not a valid seven-tile hand.
    MalformedViewerHand,
    /// A play's actor is out of clockwise order from the current leader.
    ActorOrder,
    /// A viewer play uses a tile the viewer does not hold.
    ViewerNotHolding,
    /// A viewer play sloughs while the viewer still holds a follower.
    ViewerMustFollow,
    /// A hidden action's typed conditioned successor support is empty
    /// (including possession of a tile outside the current pool).
    EmptyConditionedSupport,
    /// A completed trick fails exact resolution (duplicate tiles or
    /// malformed sequence).
    MalformedTrick,
    /// The final support does not equal the claimed normal form.
    FinalSupportMismatch,
}

/// The erased acceptance result (D1): the exact final support with its
/// pool — no trace, no witness, no provenance.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AcceptedSymbolicSupport {
    /// The final hidden pool.
    pub pool: DominoSet,
    /// The final support normal form over the pool's canonical order.
    pub support: TotalSupportNormalForm,
    /// The audit deletion ledger `(trace index, tile, hidden seat index)`
    /// in play order — audit data outside semantic equality (INV-11).
    pub deletion_ledger: Vec<(usize, DominoId, usize)>,
}

/// Validate a symbolic certificate (REACH-15), invoking `observer` with the
/// support state after every play (for three-way agreement harnesses).
pub fn validate_symbolic_trace_with(
    certificate: &SymbolicTraceCertificate,
    mut observer: impl FnMut(usize, &TotalSupportNormalForm, &[DominoId]),
) -> Result<AcceptedSymbolicSupport, SymbolicRejection> {
    if certificate.viewer_initial_hand.len() != 7 {
        return Err(SymbolicRejection::MalformedViewerHand);
    }
    let declaration = certificate.contract.declaration();
    let algebra = algebra_for(declaration);
    let viewer = certificate.viewer;

    // Unrestricted initial support over the 21 unseen tiles (Math §7.4).
    let mut tile_order: Vec<DominoId> = all_ids()
        .filter(|d| !certificate.viewer_initial_hand.contains(*d))
        .collect();
    let initial = crate::support::cells::AbstractCells::new(
        tile_order.len(),
        core::array::from_fn(|_| vec![true; tile_order.len()]),
        [7; HIDDEN_SEATS],
    )
    .expect("unrestricted initial support");
    let mut support = crate::support::normal_form::compile_total_support(&initial, None);

    let mut viewer_hand = certificate.viewer_initial_hand;
    let mut leader = certificate.contract.bidder();
    let mut trick: Vec<Play> = Vec::new();
    let mut ledger: Vec<(usize, DominoId, usize)> = Vec::new();

    for (index, play) in certificate.trace.iter().enumerate() {
        let expected_actor = leader.offset(trick.len() as u8);
        if play.actor != expected_actor {
            return Err(SymbolicRejection::ActorOrder);
        }
        let led = trick.first().map(|p| algebra.led_suit(p.domino));
        if play.actor == viewer {
            // Viewer actions: known-hand legality (follow-if-possible).
            if !viewer_hand.contains(play.domino) {
                return Err(SymbolicRejection::ViewerNotHolding);
            }
            if let Some(q) = led {
                let follows = algebra.follows(play.domino, q);
                let has_follower = viewer_hand.iter().any(|e| algebra.follows(e, q));
                if !follows && has_follower {
                    return Err(SymbolicRejection::ViewerMustFollow);
                }
            }
            viewer_hand.remove(play.domino);
        } else {
            // Hidden actions: typed conditioned successor must be nonempty.
            let Some(position) = tile_order.iter().position(|&d| d == play.domino) else {
                return Err(SymbolicRejection::EmptyConditionedSupport);
            };
            let seat = (play.actor.index() + 4 - viewer.index() - 1) % 4;
            debug_assert!(seat < HIDDEN_SEATS);
            let observation = game_observation(declaration, led, play.domino, &tile_order);
            let (next, record) = matching_minor_update(&support, seat, position, &observation);
            if matches!(next, TotalSupportNormalForm::Empty) {
                return Err(SymbolicRejection::EmptyConditionedSupport);
            }
            record_ledger(&mut ledger, index, &record, &tile_order);
            tile_order.remove(position);
            support = next;
        }
        trick.push(*play);
        if trick.len() == 4 {
            let result = algebra
                .resolve_trick(&trick)
                .map_err(|_| SymbolicRejection::MalformedTrick)?;
            leader = result.winner;
            trick.clear();
        }
        observer(index, &support, &tile_order);
    }

    let pool = DominoSet::from_ids(tile_order.iter().copied());
    if pool != certificate.claimed_pool || support != certificate.claimed_final {
        return Err(SymbolicRejection::FinalSupportMismatch);
    }
    Ok(AcceptedSymbolicSupport {
        pool,
        support,
        deletion_ledger: ledger,
    })
}

fn record_ledger(
    ledger: &mut Vec<(usize, DominoId, usize)>,
    index: usize,
    record: &DeletionRecord,
    tile_order: &[DominoId],
) {
    for &(position, seat) in &record.deleted {
        ledger.push((index, tile_order[position], seat));
    }
}

/// Validate a symbolic certificate (the plain external-state gate,
/// REACH-14/15; Exec §18).
pub fn validate_symbolic_trace(
    certificate: &SymbolicTraceCertificate,
) -> Result<AcceptedSymbolicSupport, SymbolicRejection> {
    validate_symbolic_trace_with(certificate, |_, _, _| {})
}
