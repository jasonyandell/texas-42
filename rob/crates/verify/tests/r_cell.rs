//! S3 named receipt tests (BRIEF §8 table S3; INV-5).

use rob_verify::s3;

#[test]
fn r_cell_initial() {
    assert_eq!(s3::initial_check(), 108);
}

#[test]
fn r_cell_dependent() {
    assert_eq!(s3::dependent_check(), (2, 4));
}

#[test]
fn r_cell_tiny_updates() {
    assert_eq!(s3::tiny_updates_check(), (14_412, 56_460, 56_460));
}

#[test]
fn r_cell_parity() {
    let (prefixes, with_voids) = s3::parity_check();
    assert_eq!(prefixes, 972);
    // Generator-specific frozen value (BRIEF §8: the with-voids count is
    // rob-generator-specific; ingest's own generator yields 970). Frozen at
    // first green run and asserted exactly thereafter (INV-5).
    assert_eq!(with_voids, rob_verify::s3::FROZEN_WITH_VOIDS);
}

#[test]
fn r_cell_transitions() {
    assert_eq!(s3::transitions_check(), (864, 648, 216));
}

#[test]
fn r_cell_ninety_world_support() {
    assert_eq!(s3::ninety_world_check(), 90);
}
