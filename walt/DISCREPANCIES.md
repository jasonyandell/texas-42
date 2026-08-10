# walt: spec-vs-reference discrepancies

Protocol (CLAUDE.md): where the frozen spec and a reference implementation
disagree, implement what the **receipt replay** forces, and record the conflict
here rather than silently picking a reading.

## Open discrepancies

### exp5 census pins: "q_points classes" has no defining source (S2)

PLAN.md's ground-truth bridges pin "exp5 census values on shared kernels
(e.g. h1t3: 10 q_points classes; h3t3: 5345)", and the S2 brief directs that
the exact meaning of these counts be taken from v0.4 §14 before pinning. It
cannot be: §14 records experiments 1, 2, 3A, 3B, 4A, and 4B only (§14.2--§14.7)
and contains neither the token "exp5" nor "q_points" anywhere in the frozen
document. The exp5 probe corpus itself is scratchpad-era Python that is not in
this repository -- S1 imported only its *fiber sizes* (transcribed into
`walt-kernel/tests/common/mod.rs`), which are defined independently by v0.4
§2.1. There is no way to reproduce "10 classes" or "5345 classes" without
guessing what was counted (root-Q vectors at which sample points? under which
utility? over which root-action set at a five-tile horizon?), which the
protocol forbids.

Recorded as the `#[ignore]`d test
`walt-strat/tests/exp5_census_blocked.rs::exp5_census_h1t3_has_10_classes_and_h3t3_has_5345`.
Unblock by adding the exp5 census definition (the probe script or its report)
to the repository and replacing the blocked test with an exact reproduction.

## Reconciled, not discrepancies

Everything else walt implements is the v0.4 text; the two
places where the spec's phrasing and `rules42.py`'s code *look* different are
reconciled below, and both were checked exhaustively rather than argued.

### Rank of a mixed tile: "pip sum" (spec) vs. off-pip (reference)

v0.4 §1.3 says a mixed tile is "ranked by pip sum inside a nonzero tier";
`rules42.rank_key` ranks a mixed tile by its *off-pip* within the suit. These
are the same order. Ranks are only ever compared inside one tier, a tier fixes
one effective context `q`, and every mixed tile there carries pip `q`, so
`pip_sum = q + off_pip` is monotone in `off_pip`.

walt-core implements the spec's formulation (`Decl::rank` is a function of the
tile alone), which makes the rank table global. The safety condition that
buys -- distinct tiles in a shared nonzero tier never share a rank -- is
asserted exhaustively in `walt-core/tests/exhaustive.rs::ranks_are_injective_within_every_nonzero_tier`,
and the doubles sentinel (12, above the mixed maximum of 11) is what keeps
"a natural double is top in its effective natural suit" true.

Under doubles-trump the doubles are ranked by pip (0..=6), which numerically
overlaps mixed pip sums. That is harmless only because a double is always
tier 2 under DT and a mixed tile never is; the injectivity test is what pins it.

### Declaration coverage of the ground-truth bridge

`rob/receipts/verify_player.txt` contains pip-trump hands only (P0, P1, P3, P4,
P5, P6 -- no P2, no DT, no NT). The replay bridge therefore validates the
pip-trump path against rob and nothing else; DT and NT are pinned by the
exhaustive structural tests only. This is recorded honestly in
`walt-core/tests/receipt_replay.rs::receipt_declaration_coverage_is_pip_trump_only`,
which fails if the corpus ever changes shape.

Independently of CI, S1 ran a one-off differential of the whole rule algebra
against `rules42.py` over **all 737,100 four-tile tricks x 9 declarations**
(winner and trick points) and over all 2,016 follow predicates and 252 led
contexts: **zero mismatches**, DT and NT included. That is an exploratory-tier
cross-check between two implementations, not a promotion of anything; walt-core
carries no receipt yet.
