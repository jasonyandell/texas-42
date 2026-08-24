id: [[walt-unification]]
opened: 2026-08-24

## What

Execute the fold plan of `walt/UNIFICATION-CENSUS.md` §5 with Jason's
2026-08-24 rulings: one unified walt crate (rules/kernel/geom/strat/
spec/carrier/solver as modules); walt-wasm stays a thin cdylib crate;
GPU trio (gpu-ref, metal, m2-runner) stays separate; DELETE walt-m3-net,
walt-m3-oracle-a, walt-m3-metal, walt-wasm-spike, walt-skeleton, and
walt-factory (artifacts archived per [[hf-archive-upload]] and
walt/ARCHIVE.md); PLAN.md retired (content superseded by the census +
wiki). Extend ci/check.sh grep lists to the surviving crates (the
census's flag 1). rob's mathematical-engineering discipline is the bar.

## Done when

ci/check.sh green on the unified shape; wasm smoke 28/28 vs native
trace; receipts byte-identical; ARCHIVE.md updated with the deletion
commit; pushed on walt-unify.

## Closed (2026-08-24)

Blocked once on freeze-56 (the source closure pinned the crate layout
by path); Jason ruled same day, adjudicated as FZ-A1..A6 in
CENSUS-RULINGS.md. Executed through commit c92175a: fold d1499d4
(42 suites green, wasm smoke 28/28 byte-identical), freeze-56 v2
issued append-only, full ci/check.sh PASS — first green on that gate
since 97ce321. Full record: `walt/UNIFICATION-CENSUS.md` § Execution.
Follow-ups spun out: [[m2-receipt-reearn]], [[m2-runner-trace]],
[[wiki-overhaul]].

## Links

[[hf-archive-upload]], [[wiki-overhaul]], walt/UNIFICATION-CENSUS.md,
walt/ARCHIVE.md
