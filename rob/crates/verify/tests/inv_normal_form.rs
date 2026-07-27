//! Named invariant enforcement tests for the normal-form layer (BRIEF §5:
//! INV-6 REACHABLE-IMPLIES-FEASIBLE).

use rob_core::{
    compile_exact_support, compile_total_support, AbstractCells, TotalSupportNormalForm,
};

fn infeasible_system() -> AbstractCells {
    // Tile b is in nobody's allowed set: the fiber is empty.
    AbstractCells::new(
        2,
        [vec![true, false], vec![true, false], vec![false, false]],
        [1, 1, 0],
    )
    .expect("structurally well-formed but infeasible")
}

/// INV-6 REACHABLE-IMPLIES-FEASIBLE, validation half: the `Empty` normal
/// form exists only on the external-validation path, where an infeasible
/// foreign system denotes the single `Empty` state (CELL-14).
#[test]
fn inv_reachable_implies_feasible_validation_path() {
    assert_eq!(
        compile_total_support(&infeasible_system(), None),
        TotalSupportNormalForm::Empty
    );
}

/// INV-6, certified half: on the internally certified path an empty support
/// fiber is an internal error (panic), never a value.
#[test]
#[should_panic(expected = "InvariantViolation")]
fn inv_reachable_implies_feasible_certified_path() {
    let _ = compile_exact_support(&infeasible_system(), None);
}
