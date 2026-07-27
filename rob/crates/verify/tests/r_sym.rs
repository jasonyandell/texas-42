//! S6 named receipt tests (BRIEF_SLICE_02 §9 table S6; INV-5). All
//! corpus-anchored.

use rob_verify::s6;

#[test]
fn r_sym_corpus() {
    assert_eq!(s6::corpus_check(), (108, 3_024, 3_024));
}

#[test]
fn r_sym_budget() {
    assert_eq!(s6::budget_check(), 6_804);
}

/// INV-11 EDGE-BUDGET named enforcement test: the deletion ledger over the
/// S6 corpus totals 108·63 with zero reappearances and the ≤2 live-deletion
/// bound (all asserted inside `budget_check`).
#[test]
fn inv_edge_budget() {
    assert_eq!(s6::budget_check(), 108 * 63);
}

#[test]
fn r_sym_reject() {
    assert_eq!(s6::reject_check(), 324);
}
