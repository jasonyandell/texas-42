//! S7 named receipt tests (BRIEF_SLICE_02 §9 table S7; INV-5).
//! `x_` tests carry the exchange tier (x:002, x:005).

use rob_verify::s7;

#[test]
fn r_out_schedule() {
    let (a, t1, t2) = s7::schedule_check();
    assert_eq!(
        a,
        [1, 50, 1_079, 13_084, 97_119, 450_066, 1_273_609, 2_097_152]
    );
    assert_eq!(
        t1,
        [8, 323, 5_524, 51_759, 286_770, 947_017, 1_817_216, 2_097_152]
    );
    assert_eq!(
        t2,
        [22, 743, 10_844, 88_159, 428_562, 1_244_937, 2_080_768, 2_097_152]
    );
}

#[test]
fn r_out_lead_witness() {
    assert_eq!(s7::lead_witness_check(), 176);
}

#[test]
fn r_out_profiles() {
    let census = s7::profile_census();
    assert_eq!(census.per_declaration.to_string(), "7124838074989");
    assert_eq!(census.total.to_string(), "64123542674901");
    assert_eq!(census.max_block.to_string(), "839220930919");
    assert_eq!(census.widths, (46, 43, 43, 40));
}

#[test]
fn r_out_five_checks() {
    s7::five_checks_sanity();
}

/// Exchange tier (x:005): the Burnside decomposition of the ternary
/// signature census.
#[test]
fn x_r_out_burnside() {
    assert_eq!(s7::burnside_check(), (136_514, 2_156, 35, 23_842));
}
