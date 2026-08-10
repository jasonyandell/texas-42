# exp5 — census curve probe suite

**Tier: exploratory.** Nothing here is a receipt or a status; these are probe
scripts and their raw outputs, preserved from the 2026-08-09 session
(scratchpad-era, session f44f8bf1). Cited by nothing above the exploratory
tier. Probe numbers become quotable results only via the usual
verifier-receipt route.

## What this is

Experiment 5: how the value quotient and the action quotient behave as the
horizon grows, over 13 receipt hands × horizons 2–6 (~2.4M solves). Headline
findings (exploratory, reported in full in `exp5_results.md`):

- Value census collapses with horizon (fixed-250-window q_points 98.4% → 10.4%
  from H2 to H6); the **action quotient holds** (99.2% → 88%).
- Sampled action floors recover ~97% of census vs ~44% for value.
- Control hypothesis confirmed: h1t3 vs h3t3, identical fiber 756,756 →
  10 vs 5,345 classes. The scheme must be a quotient of decisions, not of
  values.

## Files

- `exp5_core.py` — bitmask PI minimax; exact fiber counting/sampling DP
  (integer only).
- `exp5_rules.py` — frozen copy of the declaration-general rules module
  (replay-validates all 13 receipt hands).
- `exp5_census.py` — census driver.
- `exp5_validate.py` — validators (receipt replay, invariants).
- `exp5_report.py` — report/table generation from the records.
- `exp5_pwl.py`, `exp5_exact.py` — PWL helpers and exact-mode probe.
- `exp5_records.jsonl` — the 566 raw run records.
- `exp5_results.md` — the full written report.

## Role going forward

`walt/PLAN.md` ground-truth bridge #3: these Python probes are the second
implementation for cross-checking walt (the Rust seat). walt-geom/walt-factory
reproduce the census vectors here (e.g. h1t3 = 10, h3t3 = 5,345) as
cross-implementation regression pins — pins, not axioms (TRUST-01).

Stdlib-only Python 3.12; running these creates `__pycache__` — clean it up
(D15).
