//! Named invariant enforcement tests (BRIEF §5).

use rob_core::{algebra_for, all_ids, LedSuit, GAME_DECLARATIONS};

/// INV-7 NO-RANK-FROM-ID: `DominoId` magnitude never determines game rank.
///
/// For every declaration there is a context and a pair of dominoes whose
/// identity order and trick-key order disagree (the `DominoId` newtype
/// additionally has no `Ord` at all — see the `compile_fail` doctest on
/// `DominoId`).
#[test]
fn inv_id_not_rank() {
    for &decl in &GAME_DECLARATIONS {
        let algebra = algebra_for(decl);
        let found = LedSuit::all().into_iter().any(|q| {
            all_ids().any(|a| {
                all_ids().any(|b| {
                    a.index() < b.index() && algebra.trick_key(a, q) > algebra.trick_key(b, q)
                })
            })
        });
        assert!(
            found,
            "id order and key order must disagree somewhere in {decl:?}"
        );
    }
}
