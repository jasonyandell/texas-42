//! S4 named receipt tests (BRIEF §8 table S4; INV-5).

use num_bigint::BigUint;
use rob_verify::s4;

#[test]
fn r_nf_hall() {
    assert_eq!(s4::hall_check(), (66_968, 14_578, 22_620));
}

#[test]
fn r_nf_count_routes() {
    assert_eq!(s4::count_routes_check(), 66_968);
}

#[test]
fn r_nf_capacity_dp() {
    let dp = s4::capacity_dp_check();
    assert_eq!(dp.profiles, 512);
    assert_eq!(dp.max_states, 512);
    assert_eq!(dp.max_checks, 1_533);
    assert_eq!(dp.max_updates, 1_344);
    assert_eq!(dp.max_layer, 48);
    assert_eq!(dp.max_count, BigUint::from(399_072_960u64));
}

#[test]
fn r_nf_marginal() {
    assert_eq!(s4::marginal_check(), 785_736);
}

#[test]
fn r_nf_reduction() {
    assert_eq!(s4::reduction_check(), 66_968);
}

#[test]
fn r_nf_sampler() {
    assert_eq!(s4::sampler_check(), 22_620);
}

#[test]
fn r_nf_quotient() {
    let q = s4::quotient_check();
    assert_eq!(q.scc_compilations, 22_620);
    assert_eq!(q.essential_exclusions, 2_151);
    assert_eq!(q.rank_unrank, 22_620);
}

#[test]
fn r_nf_ternary_census() {
    let t = s4::ternary_census_check();
    assert_eq!(t.signatures, 136_514);
    assert_eq!(t.matrices, 1_667_666);
    assert_eq!(t.max_matrices, 114);
    assert_eq!(t.orbits, 23_842);
    assert_eq!(t.representative_matrices, 296_721);
    assert_eq!(t.stabilizer_orbits, 279_048);
    assert_eq!(t.max_stabilizer_orbits, 103);
}

#[test]
fn r_nf_census_81() {
    let census = s4::census81_check();
    assert_eq!(census.empty.to_string(), "1");
    assert_eq!(census.determinate.to_string(), "8102258940222814");
    assert_eq!(census.binary.to_string(), "11495078055913018482");
    assert_eq!(census.ternary.to_string(), "1830955704129296418354864");
    assert_eq!(census.total().to_string(), "1830967207309611271596161");
    assert_eq!(census.fixed_width_bits(), 81);
}

#[test]
fn r_nf_capacity_profiles() {
    assert_eq!(s4::capacity_profiles_check(), 50);
}

#[test]
fn r_nf_floor() {
    assert_eq!(s4::floor_check().to_string(), "44352165");
}

#[test]
fn r_nf_zero_supplemental() {
    assert_eq!(s4::zero_supplemental_check(), 0);
}
