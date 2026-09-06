# Preserved local phone baseline

Copied byte-for-byte on 2026-09-06 from
`/Users/jason/code/plunge/src/ai/walt/` at clean local Plunge HEAD
`122ea7a59362c15850e7b54e76ce4353ddb9d07a`. `MANIFEST.sha256` identifies the
original `walt.wasm` and TypeScript `walt.ts` wrapper. They were not rebuilt.

The binary's last Plunge file-history commit is
`1810da2060bb8354130fd0315efb5b5d04816392` (2026-08-22). That commit says it
copied the Texas 42 handoff, without naming the producing Rust commit.

**Historical artifact identified, 2026-09-06:** the preserved binary is
byte-for-byte the Texas 42 artifact at commit
`9a056f20461fbe951544e4145ee49b726e0f6852` (2026-08-19), path
`walt/walt-wasm/pkg/walt.wasm`. Both have Git blob
`714773c391ccd7e02b234218f3229a2c50a172fd` and SHA-256
`af0200af8dc99d5a95ac898cded12861ca4a36c38b336dbbd6a33a515c071d59`.
That commit records rebuilding the WASM and passing its 28-move native smoke
comparison. Its source is available at `walt/walt-wasm/src/api.rs` and
`walt/walt-m3-probe/src/lib.rs`. This identifies the checked-in historical
artifact and source anchor; a reproducible rebuild was not attempted here.

The preserved binary and wrapper differ from the launch worktree's current
`walt/walt-wasm/pkg/` artifacts. The version deployed to any particular phone
was not independently checked. See [head-to-head analysis](../../HEAD-TO-HEAD.md)
for the bounded source comparison and proposed experiment format.

Live client play requests use `n=40,n0=8,race=true`; see the inspected source
and seed/auction details in `../../BASELINE.md`. The preserved files implement
the solver call, while `../../phone.mjs` supplies those play defaults. A fixture
seed is newly declared experiment provenance, not a recovered historical game
seed. Loading this module does not reproduce Plunge's full UI, heuristic auction,
worker/cache lifecycle, or out-of-scope fallback.

This is EXPLORATORY baseline preservation. Sampled solver outputs are
model-relative estimates, never full-information or equilibrium receipts.
