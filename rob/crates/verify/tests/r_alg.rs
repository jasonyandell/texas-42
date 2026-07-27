//! S1 named receipt tests (BRIEF §8 table S1; INV-5: every exhaustive count
//! is a hard equality assertion against the exact expected integer).

use rob_verify::s1;

#[test]
fn r_alg_universe() {
    assert_eq!(s1::universe_check(), (28, 35));
}

#[test]
fn r_alg_k7() {
    assert_eq!(s1::k7_check(), (28, 35));
}

#[test]
fn r_alg_contexts() {
    assert_eq!(s1::contexts_check(), 7);
}

#[test]
fn r_alg_tiers() {
    s1::tiers_check();
}

#[test]
fn r_alg_unique_winner() {
    assert_eq!(s1::unique_winner_count(), 737_100);
}

#[test]
fn r_alg_prose_agreement() {
    assert_eq!(s1::prose_agreement_count(), 737_100);
}

#[test]
fn r_alg_scoring() {
    assert_eq!(s1::scoring_check(), (35, 7, 42));
}

#[test]
fn r_alg_beats() {
    // all 9 · 8 · 28 · 28 membership equivalences
    assert_eq!(s1::beats_check(), 9 * 8 * 28 * 28);
}

#[test]
fn r_alg_threat_witness() {
    s1::threat_witness_check();
}

#[test]
fn r_alg_scored_transport() {
    assert_eq!(s1::scored_transport_check(), (5_040, 2));
}

#[test]
fn r_alg_unscored_transport() {
    assert_eq!(s1::unscored_transport_check(), (49, 307_328));
}

#[test]
fn r_alg_mechanics_classes() {
    assert_eq!(s1::mechanics_classes_check(), 3);
}

#[test]
fn r_alg_competitive_ordinal() {
    assert_eq!(s1::competitive_ordinal_check(), 13);
}
