//! S8 named receipt tests (BRIEF_SLICE_02 §9 table S8; INV-5).
//! `x_` tests carry the exchange tier (x:002); refuting one is a
//! stop-and-report finding of the first order (§1.1).

use rob_verify::s8;

#[test]
fn r_unr_reach10() {
    assert_eq!(s8::reach10_check(), (450, 2, (7, 1)));
}

#[test]
fn x_r_unr_002_outer() {
    assert_eq!(s8::x002_outer_check(), 4);
}

#[test]
fn x_r_unr_002_static() {
    let (matches, kills, survivors) = s8::x002_static_check();
    assert_eq!(matches, 3);
    assert_eq!(kills, 1);
    assert_eq!(survivors.len(), 2);
}

/// The permanent second unreachability regression (INV-14): passing every
/// implemented outer check does not imply reachability.
#[test]
fn x_r_unr_002_traces() {
    assert_eq!(s8::x002_trace_search(), (425_520, 0));
}

#[test]
fn x_r_unr_002_supply() {
    assert_eq!(s8::x002_supply_check(), 1);
}
