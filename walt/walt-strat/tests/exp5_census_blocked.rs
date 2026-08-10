//! BLOCKED pins -- ambiguity protocol, not an implementation gap.
//!
//! PLAN.md's ground-truth bridges list "exp5 census values on shared kernels
//! (e.g. h1t3: 10 q_points classes; h3t3: 5345)". The S2 brief directs that
//! the exact meaning of these counts be taken from v0.4 §14 before pinning.
//! §14 records experiments 1, 2, 3A, 3B, 4A, and 4B only; it never mentions
//! exp5 and never defines "q_points classes", and the exp5 probe scripts are
//! scratchpad-era -- not in this repository. There is nothing to reproduce
//! against without guessing what was counted, which the protocol forbids.
//! See walt/DISCREPANCIES.md ("exp5 census pins").

#[test]
#[ignore = "blocked: 'q_points classes' is undefined in v0.4 §14; see walt/DISCREPANCIES.md"]
fn exp5_census_h1t3_has_10_classes_and_h3t3_has_5345() {
    panic!(
        "unblock by adding the exp5 census definition (report or script) to the \
         repository, recording it in walt/DISCREPANCIES.md, and replacing this \
         test with an exact reproduction"
    );
}
