//! S2 named receipt tests (BRIEF §8 table S2; INV-5).

use proptest::prelude::*;
use rob_verify::s2;

#[test]
fn r_obj_deals() {
    assert_eq!(s2::deals_check().to_string(), "472518347558400");
}

#[test]
fn r_obj_hidden() {
    assert_eq!(s2::hidden_check().to_string(), "399072960");
}

#[test]
fn r_obj_auction_census() {
    let (counts, maxima) = s2::auction_census_check();
    assert_eq!(counts, [2380, 3060, 3196, 3213, 3214, 3214, 3214]);
    assert_eq!(maxima, [1, 2, 3, 4, 5, 5, 5]);
}

#[test]
fn r_obj_lifecycle() {
    s2::lifecycle_check();
}

#[test]
fn r_obj_legal_play() {
    assert_eq!(s2::legal_play_check(), 200);
}

#[test]
fn r_obj_conservation() {
    assert_eq!(s2::conservation_check(), 201);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    /// Property form of `r_obj_legal_play`/`r_obj_conservation` over
    /// arbitrary corpus seeds (BRIEF §8 S2 proptest row).
    #[test]
    fn r_obj_legal_play_prop(seed in any::<u64>()) {
        let hand = s2::corpus_hand(seed);
        s2::check_legal_play_laws(&hand);
        s2::check_conservation(&hand);
    }
}
