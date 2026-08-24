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

## Blocked (2026-08-24)

Stays in doing/. Not started: no Done-when item is reachable yet.
`ci/check.sh` runs `ci/verify_m2_sources.sh`, whose freeze-56 source
closure pins the crate layout by path — and whose immutable 184-path
representation check (fed by a manifest whose bytes are hash-pinned
from a fixed parent commit) names 32 files under walt-core, walt-kernel
and walt-gpu-spec, the crates the fold moves. No manifest regeneration
can satisfy it; only an amendment to the freeze verifier can, which is
a freeze-level ruling, not code motion.

Also pre-existing: the gate is already red at 114bacd — 6 digest
mismatches (3 from ancestor commit 97ce321, the "does not build" WIP)
and 65 missing `walt-factory/results/*` entries from the archive move.

Full analysis, scope table, and the four questions a ruling must answer:
`walt/UNIFICATION-CENSUS.md` § "Execution (2026-08-24)".

## Links

[[hf-archive-upload]], [[wiki-overhaul]], walt/UNIFICATION-CENSUS.md,
walt/ARCHIVE.md
