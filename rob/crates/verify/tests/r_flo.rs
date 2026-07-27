//! S10 (stretch) named receipt tests (BRIEF_SLICE_02 §9 table S10; INV-5).
//! All rows carry the exchange tier (x:001); a family collision or total
//! mismatch would be a stop-and-report finding (§1.1).

use rob_verify::s10;

#[test]
fn x_r_flo_modules() {
    assert_eq!(s10::modules_check(), 3_808);
    // rob verifies the full principled one-context space — a strict
    // superset of x:001's 216 tabled profiles (rob-frozen value; see the
    // receipt note and final report).
    assert_eq!(s10::one_context_profiles(), 369);
}

#[test]
fn x_r_flo_families() {
    assert_eq!(s10::no_void_family(), 559_316_142);
    assert_eq!(s10::called_family(), 8_387_350_664);
    assert_eq!(s10::natural_family(), 8_721_399_239);
}

#[test]
fn x_r_flo_total() {
    let total = s10::no_void_family() + s10::called_family() + s10::natural_family();
    assert_eq!(total, 17_668_066_045);
    assert!(
        total > 1u64 << 34,
        "the floor exceeds 2^34: 35 bits necessary"
    );
}
