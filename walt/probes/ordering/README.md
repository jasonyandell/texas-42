# Ordering probe — reorder-not-cull at the `solve_viewer` break

**EXPLORATORY INSTRUMENT OUTPUT** — sits below every evidentiary tier and
is cited by nothing above it. Every number here is a machine-local wall
reading or an exact integer counter over instrument records, never a
receipt; nothing in this directory is quotable above the Ideas tier
without a brief amendment that adds it to a verifier receipt.

Parent ruling: `walt/CENSUS-RULINGS.md` **E-A15 (order, not set)** —
changing the ORDER of evaluation is lawful; changing the SET is a
declared exclusion. `Solver::solve_viewer` is value-only (max/min over
children, no action returned, no randomness consumed in the loop), so
permuting its visit order is a pure speed knob: same value, same
everything, only the Boolean pmake break (`v.is_one()` maximizing /
`v.is_zero()` minimizing) fires earlier or later.

Producer: `walt/walt/src/bin/ordering_bench.rs` (run:
`cargo run --release --bin ordering_bench [hard]`, default features).
Gates: `walt/walt/tests/solver_ordering.rs` (value pins recorded from the
ascending-order baseline; counter sanity; permutation determinism).

## Workload (declared)

1. **exact-sigma{0,1}-h4-t6 / h8-t5 / h10-t6** — the three cheap receipt
   roots of the targeted gates (`tests/solver_targeted.rs`; fibers 90,
   92, 19), each through `exact_frozen_action_values` (the
   `solver::controller` cold exact frozen-set endpoint) under the
   declared cheap pair σ0 = Level0{n0=2}, σ1 = Level1{n_outer=2, n0=2}.
2. **level1-deal{1,2}** — `level1_evaluate` at two synthetic
   declared-seed roots (the `solver_viewer_fiber` deal construction,
   seed `0x9E37_79B9`, NoTrump, bid 30, seat 1 leading, n_outer=8,
   n0=2).
3. **direct-deal{1,2}** — the same synthetic roots driven through a
   bench-owned serial `Solver`, where the break counters are readable.
   The high-level endpoints build their `Shared`s internally (one per
   modeled-mind call in the field machinery), so their counters are not
   reachable from the bench; the direct items carry the counter signal.
4. **exact-sigma0-h8-t4-hard** (`hard` argument) — the harder receipt
   root, fiber 1200, σ0 only.

Counter semantics: `children/legal` = children actually solved vs legal
moves available, summed over every `solve_viewer` visit. Ratio 1/1 means
the break never helped; lower is better. `host_*` is the bench-owned
solver alone (serial, exact, deterministic); `shared_*` adds every inner
modeled-mind solver that flushed into the same `Shared`.

## Honest confounder

Reordering traversal changes intern-ID assignment order
(`Solver::intern`) and thus memo/alive-set hit patterns — same values,
different cache behavior — so the wall-clock delta folds ordering AND
cache effects. The children-solved counter is the clean signal for how
much earlier the break fires. Wall micros are single-shot readings on one
machine (M-series, release profile), not statistics.

## Readings

Values printed by the bench were byte-identical before/after on every
item (the E-A15 invariance, also pinned in `tests/solver_ordering.rs`).

### Before — ascending tile order (commit "counter + bench")

| item | micros | children/legal |
|---|---|---|
| exact-sigma0-h4-t6 (fiber 90) | 1,465 | — |
| exact-sigma1-h4-t6 | 72,905 | — |
| exact-sigma0-h8-t5 (fiber 92) | 65,193 | — |
| exact-sigma1-h8-t5 | 699,256 | — |
| exact-sigma0-h10-t6 (fiber 19) | 115 | — |
| exact-sigma1-h10-t6 | 186 | — |
| level1-deal1 | 27,761 | — |
| direct-deal1 | 35,121 | host 1198/1308; shared 28957/44113 |
| level1-deal2 | 45,190 | — |
| direct-deal2 | 29,972 | host 1051/1380; shared 25552/33961 |
| exact-sigma0-h8-t4-hard (fiber 1200) | 3,134,946 | — |

### After — heuristic order (the reorder commit)

(recorded when the ordering lands)
