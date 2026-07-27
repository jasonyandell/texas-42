//! Named invariant enforcement tests for the support layer (BRIEF §5:
//! INV-1 DERIVED-NOT-STORED).

use rob_core::{derive_rule_cells, MechanicalCompiledView, Seat};
use rob_verify::s3::{mechanical_trajectory, s3_corpus_hand};

/// INV-1 DERIVED-NOT-STORED: cells are pure functions of semantic state.
/// Along the S3 corpus, recompute the derived view from scratch after every
/// transition and assert it equals any cached value; caches live outside
/// semantic equality (D2; CELL-17).
#[test]
fn inv_derived_coherence() {
    for index in [0u64, 13, 47, 61, 88, 107] {
        let hand = s3_corpus_hand(index);
        let trajectory = mechanical_trajectory(&hand, Seat::ALL[0]);
        for state in &trajectory {
            // A view carrying a cache made at derivation time stays coherent
            // with a from-scratch recomputation.
            let cached = MechanicalCompiledView {
                state: state.clone(),
                cells_cache: Some(derive_rule_cells(state)),
            };
            assert!(cached.coherent(), "cache equals fresh derivation");
            // Cache contents are excluded from semantic equality.
            let uncached = MechanicalCompiledView {
                state: state.clone(),
                cells_cache: None,
            };
            assert_eq!(cached, uncached, "cache is outside semantic equality");
        }
    }
}
