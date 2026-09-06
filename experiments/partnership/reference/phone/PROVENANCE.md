# Preserved local phone baseline

Copied byte-for-byte on 2026-09-06 from
`/Users/jason/code/plunge/src/ai/walt/` at clean local Plunge HEAD
`122ea7a59362c15850e7b54e76ce4353ddb9d07a`. `MANIFEST.sha256` identifies the
original `walt.wasm` and TypeScript `walt.ts` wrapper. They were not rebuilt.

The binary's last file-history commit is
`1810da2060bb8354130fd0315efb5b5d04816392` (2026-08-22). That commit says it
copied the Texas 42 handoff, without naming the producing Rust commit. The
preserved binary and wrapper differ from the launch worktree's current
`walt/walt-wasm/pkg/` artifacts. The version deployed to any particular phone
was not independently checked.

Live client play requests use `n=40,n0=8,race=true`; see the inspected source
and seed/auction details in `../../BASELINE.md`. The preserved files implement
the solver call, while `../../phone.mjs` supplies those play defaults. A fixture
seed is newly declared experiment provenance, not a recovered historical game
seed. Loading this module does not reproduce Plunge's full UI, heuristic auction,
worker/cache lifecycle, or out-of-scope fallback.

This is EXPLORATORY baseline preservation. Sampled solver outputs are
model-relative estimates, never full-information or equilibrium receipts.
