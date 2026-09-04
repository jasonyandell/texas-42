id: [[gate-corpus-trim]]
opened: 2026-09-04

## What

Gate files sized like censuses: several walt suites sweep ten roots ×
contracts × cuts (or × horizons × tails) so that one assertion of the
form "strict somewhere on the corpus" has something to find. A gate
needs one coordinate per law it asserts plus one PINNED strictness
witness; the sweep belongs in the probe record, not the gate. Trim
suite by suite when each is next touched, pinning the witness
coordinate by name (root, contract, action) so the strictness
assertion keeps its teeth. Known candidates (2026-09-04 CI review,
sum of suite walls 498 s before the concurrent runner):
`tests/solver_factor_refine.rs` (146 s), `tests/solver_horizon.rs`
(116 s, H1's 40 censuses), `tests/solver_unified_carry.rs` (61 s),
`tests/solver_focal_horizon.rs` (350 s at landing). The sampled tiers
are NOT the problem (world caps 512–4096, sub-second); the exact
recursions on oversized corpora are.

## Done when

Each listed suite asserts the same laws with a pinned witness and its
wall is dominated by the law it checks, not by the sweep; the CI review
table in `walt/ci/check.sh`'s runner output (slowest suites) shows no
suite above ~60 s without a stated reason in its module doc.

## Links

CLAUDE.md "Code discipline" (the gate-sizing rule); `walt/ci/
run_test_binaries.py` (the concurrent runner that made this a CPU
problem rather than a wall problem); BRIEF-CI1 (shared fixtures — the
recompute-once half of the same review).
