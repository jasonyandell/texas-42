//! `GreedySigma` — the fixed deterministic field policy σ
//! (BRIEF_PLAYER_01 §7; INV-P4 FIELD-FIXED).
//!
//! σ models every non-viewer seat inside rob's solves. It is a pure
//! deterministic function of the acting seat's own hand and the public trick
//! state — points-blind, history-blind beyond the current trick, defined
//! entirely through the S1 declaration algebra (`led_suit`, `follows`,
//! `trick_key`), never through pip arithmetic. Its determinism is the
//! compression that makes rob's solves exact: given a world and σ, play
//! between rob's decisions branches nowhere.
//!
//! The definition is normative spec, not taste (BRIEF_PLAYER_01 §10.1):
//! *leading*, play the legal tile with the highest trick key under its own
//! led context, tie-break lowest id; *following or sloughing*, if some legal
//! tile's trick key beats the current best play's key, play the lowest such
//! tile by key (then lowest id); otherwise play the legal tile with the
//! lowest key (then lowest id). σ's quality is explicitly not a goal; its
//! totality, legality, and determinism are receipts (`r_sig_*`).

use rob_core::{DeclarationAlgebra, DominoId, DominoSet, Play};

/// The exact legal set for a hand facing a (possibly empty) trick — the
/// Exec §11 `legalPlays` formula spelled through the engine's follow
/// relation, in canonical identity order.
pub fn sigma_legal(
    algebra: &DeclarationAlgebra,
    hand: &DominoSet,
    trick: &[Play],
) -> Vec<DominoId> {
    if trick.is_empty() {
        return hand.iter().collect();
    }
    let q = algebra.led_suit(trick[0].domino);
    let followers: Vec<DominoId> = hand.iter().filter(|&d| algebra.follows(d, q)).collect();
    if followers.is_empty() {
        hand.iter().collect()
    } else {
        followers
    }
}

/// σ's choice for the acting seat: a pure deterministic function of
/// `(hand, trick)` under the fixed declaration algebra (INV-P4).
///
/// Panics on an empty hand — σ is total on every reachable acting state,
/// which is exactly the `r_sig_total` receipt obligation.
pub fn greedy_sigma(algebra: &DeclarationAlgebra, hand: &DominoSet, trick: &[Play]) -> DominoId {
    assert!(
        !hand.is_empty(),
        "sigma acts only when the seat holds tiles"
    );
    let legal = sigma_legal(algebra, hand, trick);
    if trick.is_empty() {
        // Lead: highest key under the tile's own led context; ties toward
        // the lower canonical identity.
        return legal
            .into_iter()
            .max_by(|&a, &b| {
                let ka = algebra.trick_key(a, algebra.led_suit(a));
                let kb = algebra.trick_key(b, algebra.led_suit(b));
                ka.cmp(&kb).then(b.index().cmp(&a.index()))
            })
            .expect("nonempty legal set");
    }
    let q = algebra.led_suit(trick[0].domino);
    let best = trick
        .iter()
        .map(|p| algebra.trick_key(p.domino, q))
        .max()
        .expect("nonempty trick");
    let min_by_key_then_id = |candidates: Vec<DominoId>| {
        candidates
            .into_iter()
            .min_by(|&a, &b| {
                algebra
                    .trick_key(a, q)
                    .cmp(&algebra.trick_key(b, q))
                    .then(a.index().cmp(&b.index()))
            })
            .expect("nonempty candidate set")
    };
    let beaters: Vec<DominoId> = legal
        .iter()
        .copied()
        .filter(|&d| algebra.trick_key(d, q) > best)
        .collect();
    if beaters.is_empty() {
        min_by_key_then_id(legal)
    } else {
        min_by_key_then_id(beaters)
    }
}
