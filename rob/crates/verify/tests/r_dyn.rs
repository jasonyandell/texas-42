//! S5 named receipt tests (BRIEF_SLICE_02 §9 table S5; INV-5). All
//! corpus-anchored.

use rob_verify::s5;

#[test]
fn r_dyn_corpus() {
    let (systems, feasible, distinct) = s5::dynamics_corpus();
    assert_eq!(systems, 66_969);
    assert_eq!(feasible, 14_579);
    assert_eq!(distinct.len(), 1_331);
}

#[test]
fn r_dyn_observations() {
    let sweep = s5::observation_sweep();
    assert_eq!(sweep.observations, 170_058);
    assert_eq!(sweep.nonempty, 157_809);
}

#[test]
fn r_dyn_monotone() {
    let sweep = s5::observation_sweep();
    assert_eq!(sweep.edge_checks, 1_406_592);
    assert_eq!(sweep.rank_checks, 157_809);
}

#[test]
fn r_dyn_typed_wrapper() {
    assert_eq!(s5::typed_wrapper_check(), (864, 648, 216));
}

#[test]
fn r_dyn_native_sampler() {
    assert_eq!(s5::native_sampler_check(), 972);
}
