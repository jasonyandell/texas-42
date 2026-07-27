//! S9 named receipt tests (BRIEF_SLICE_02 §9 table S9; INV-5).
//! `x_` tests carry the exchange tier (x:004).

use rob_verify::s9;

#[test]
fn r_tra_class_quotient() {
    assert_eq!(s9::class_quotient_check(), 3);
}

/// Exchange tier (x:004): conformance evidence for `f_{t,u}(R_t) = R_u` —
/// a transport non-commutation here is a stop-and-report finding (§1.1).
#[test]
fn x_r_tra_corpus_commutation() {
    assert_eq!(s9::corpus_commutation_check(), (588, 16_464, 17_052));
}

/// `r_tra_unscored_only`: the scored surface is never transported — the
/// negative form is the compile_fail doctest on
/// `rob_core::support::transport_reach`; here we assert the quotient's
/// output type is the unscored class enum.
#[test]
fn r_tra_unscored_only() {
    let class: rob_core::UnscoredMechanicsClass =
        rob_core::support::transport_reach::reachable_census_class(rob_core::Declaration::NoTrump);
    assert_eq!(class, rob_core::UnscoredMechanicsClass::NoTrumpClass);
}
