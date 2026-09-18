# Kiln work ledger

Updated 2026-09-18. The goal remains ACTIVE and incomplete. Full 1,000-deal
production, screening audit, final book installation, deployment and production
browser verification are still required. Do not recreate the campaign or mark
the goal complete from the local integration preview.

## Current checkouts and commits

- Texas 42: `/Users/jason/code/texas-42-partnership-launch`, branch
  `codex/partnership-launch`. Latest native-worker optimization is 850f589b (policy
  cache carry); latest phone core is 1dbea2c0 (sparse Dice
  buckets), following f91533cc (move ordering), 1a43d763 (small support) and
  7377ff32 (allocation removal and release audit).
- Plunge: `/Users/jason/code/plunge-sunshine`, branch `codex/sunshine-table`.
  Book integration is ef5d92d; optimized phone asset and validation notes are
  committed as **70e170c**. Repeated own-hand reconciliation is **cdf6bfe**; automatic installed-book
  validation in the normal test run is **9963705**. Reusable browser release
  verification is **b891e8e**. Latest matching phone asset is **4c19ca6**.
  The book manifest remains intentionally null.
- No Kiln changes have been merged/pushed/deployed. Base cwd
  `/Users/jason/code/texas-42` is unrelated; do not edit it.

## Production (revalidate; this file is not liveness evidence)

Campaign: `/Users/jason/data/texas-42/kiln-v1`. SQLite/WAL are authoritative.
Coordinator **32719** was launched detached, 18 processes, one internal thread,
`--seconds 0 --job-ms 120000 --order deal`. Process metadata is
production-process.json; status.json is expendable. The exclusive OS lock
prevents another writer. Graceful SIGTERM retains completed prices and returns
unfinished work to the queue. Check actual process identity before stopping it.
Mac sleep guard **32720** is `/usr/bin/caffeinate -i -w 32719`; its actual
PreventUserIdleSystemSleep assertion was verified. It ends with this coordinator.
If restarting production, bind a new guard to the new actual coordinator PID.
Metadata is power-guard.json; as always, verify live processes rather than the file.

Last audited: 173 base-covered deals, **172 fully refined**, no job errors. The
latest one-minute production firing retained **1,562** prices; the new continuous
run was around 24 prices/s while validation was running. These counts
are production progress, not controlled speedups across different job mixes.
Order `deal` is finishing refinements in the covered prefix before new coverage.
The latest process snapshot showed 18 workers using about 1,775% CPU
(roughly all 18 cores) and 1.2 GiB aggregate RSS. The full catalogue remains hours of work. Leave production running alongside
useful integration and validation; do not replace the goal with this partial book.

Current immutable native worker SHA:
`77f81f56770c3cf166b15ab6b3742a1f43462b9c5814e2ee541a732373c2cbc4`.
Binary and exact source snapshot are under producers/<sha>/. The running process
uses that copy, so rebuilding Cargo outputs does not alter active work.

The new sparse-bucket worker passed 64 exact/counter comparisons and 30 focused
Rust tests (one historical ignored fixture generator), then measured 1.197x median
/ 1.198x geometric speedup on 32 paired deep jobs. The timed comparison ran with
production stopped. Production then resumed on the old worker before a separate
one-minute candidate firing saved 1,102 new prices with zero errors. PID1275 is
the former continuous sparse-bucket run. See HOT-PATH.md and sparse-buckets-{parity,timing}-summary.json.

Native cache carry (850f589b) then passed 36 cold exact/counter checks and all 96
prices in 32 paired persistent-worker 8/40/160 ladders. It measured 1.244x median /
1.226x geometric speedup over the sparse-bucket worker. Two Rust tests cover warm
prices, all seven context guards and fresh budget/counter state; 13 Python tests
passed, including new carried-counter validation and existing abrupt-death recovery.
The committed rebuild was byte-identical to the tested candidate. Browser-target
library compilation passed; the installed phone WASM is intentionally unchanged
because this carry slot belongs only to the native Kiln worker.

The one-minute candidate firing completed 1,562 prices, 25.93/s, zero errors.
Then PID32719 started unbounded production, with sleep guard32720. A sample of
787 new deep receipts showed 761 carrying completed policy entries from the prior
stage; 40-world jobs were cold because that covered prefix's 8-world jobs had run
earlier. See CARRY-CACHE.md and carry-cache-*-summary.json. Counters measure actual
new work; cold replays reproduce prices, not necessarily warm work counts.

The expanded independent audit passed **203,139 receipts**, including carried
counter bounds and the new producer source/binary identity. Its pinned snapshot
had 172 settled deals and 173 covered. Of 666 audited cells that would have been
screened, none reached 75% at 160 worlds; 19 reached 50%. Evidence is
audit-after-carry-cache.json. This is still a partial audit without an exported
book identity; final release gates remain unchanged.

Before that speedup, observed 40/160-world job costs and refinement fractions
projected about ten more hours of refinement, excluding initial 8-world work.
The new worker should shorten this, but the full catalogue is still an overnight
batch; no deadline is promised from a small workload sample. GitHub auth was
rechecked successfully. Remote mains remain Plunge b88b7ae and research 5e8cd0f7,
with clean local fast-forward paths; latest Plunge deployment on that main passed.

Measured tuning is recorded in COMPACT-CACHE.md, SMALL-SUPPORT.md and HOT-PATH.md.
Private integer values, compact cache keys, small-support bitsets, fewer
allocations, direct bitset access and stack move ordering all preserve the
finite sampled evaluator. Every step passed retained/fresh exact-fraction and
work-counter checks. Forced-Dice draw elimination measured no useful speedup;
bitset access measured about 8%, then move-order allocation removal about 10%.

A fixed 256-case deep workload completed at every tested pool layout:
18x1 26.04s, 9x2 33.59s, 12x1 37.60s, 24x1 28.72s, repeated18x1 28.79s.
All exact values matched; serial work counters matched. Use **18x1**: the
repeat tied 24 processes with fewer workers, and both alternatives were slower.
See pool-timing-summary.json and the repeatable pool_benchmark.py diagnostic.
No claim that this finds a universal optimum over every workload. A subsequent
60-second-bounded ThinLTO / one-codegen-unit experiment matched all 64 values
and counters but measured only about 1% on 32 paired deep cases. Its temporary
Cargo profile was removed; production stays on c4678091. See HOT-PATH.md and
lto-*-summary.json. No remaining compiler experiment is running.

The independent partial campaign audit passed **109,592 receipts**, all queue
relationships, sampling identities, screening decisions and immutable source /
binary bundles. File: audit-partial.json in the campaign. This verifies structure,
not completion. Final release must run audit.py with the complete exported book.

## Calibration completed and independently audited

**21/100 makes**, versus frozen forecast **121/160 = 75.625%**; Wilson95
**14.17–29.98%**. Sixes, bid36, seat3, own hand [0,3,13,21,22,23,26]. Selected
before outcomes from six candidate forecasts as nearest .78. All receipts and
selection evidence are retained under calibration-six36/ and calibration-prices/.

All four actual players used deployed L1+partner 40/8, 160-world opening without
review, 14s ordinary/20s opening budgets. The price uses a modeled sigma0 field;
this continuation mismatch is a candidate cause, not established by this one hand.
Python checked every move while running. A separate audit through Plunge's actual
engine checked all **1,808 decisions**, actor-only requests, public prefixes,
legality, leaders, scores and final outcomes. All matched. No calibration job is
still running. See CALIBRATION-SIX36.md and calibration-six36-summary.json.

## Game integration implemented, not deployed

Plunge's `src/ai/kiln/book.ts` validates a versioned book and exposes an evaluator
that receives only own-hand/seat/bid/public-seed requests. It preserves separate
calculation seeds and per-declaration depths. Deal selection is deterministic in
game seed and hand number, score-independent, and keeps rotating shaker/marks.
The current minimum-raise, partner-pass and 3/4 model-threshold policy remains;
there is no new aggressive maximum-bid rule. Native tie selection was ported and
verified against 15 frozen native cases. Live play and partner review are unchanged.

`scripts/install-kiln.mjs` imports a hash-addressed asset and manifest. It verifies
content identity, 1000 complete panels, terminal refinements, own-hand seeds and
screen/audit rules. Normal prebuild rejects previews. Runtime verifies bytes,
profile and every shuffle against the game engine. Existing off-book hands or
visible download failures can use ordinary live bidding. The static asset uses
existing service-worker caching. No new backend is needed.

A 133-deal **local preview only** was installed temporarily. Mobile-size browser
automation at port4280 completed three AI bids in about3.8s including normal
presentation pauses; no auction workers ran, no browser errors, reload preserved
the hand. Evidence: browser-local-check.json and browser-local-book.png in the
campaign. The preview asset was removed from the repo and manifest reset to null
before committing. Local Vite session33127 may still be running, but is now back
to live bidding until another book is installed.

The reusable Plunge `scripts/kiln-browser-check.mjs` now covers actual mobile UI
play, original decision bookmarking, notes, reload of the game/question/receipt,
finished-hand replay attachment, and next-hand shaker/marks/session/catalogue bid.
The final expanded local run passed in 23.6s on the 133-deal preview: three AI bids
in 3.15s including presentation pauses, 15 actual Walt decisions through the hand's
early set, no auction workers or browser errors, and the expected optimized WASM
identity. Report/screenshots: browser-release-local-complete/ in the campaign.
Uploads were deliberately blocked; this proves local persistence, not the deployed
question service. The script's explicit --submit mode checks anonymous upload and
the completed public share, and remains to be exercised on production. Optional
--book, --wasm and --build arguments pin release identities. Both temporary local
previews were removed and the null manifest restored. Plunge is committed/clean.

At the 100-deal milestone, an independent read-only audit passed all **152,680**
saved receipts, refinement chains, exact original scores and producer source/binary
identities. Its pinned snapshot contained 101 settled deals. Of 923 preselected
audit cells already at 160 worlds, 376 would have stopped at a cheap stage; **none
reached the 75% model eligibility threshold** at depth 160. Nine reached 50%.
This supports retaining the current screen provisionally, not a proof of no misses.
The report's cell-wise Wilson intervals are descriptive: related declarations and
targets share hands/samples, so they are not calibrated campaign-wide bounds.
Evidence: audit-100-deal-milestone.json. Full release still requires all 1,000 deals
and a final audit with the actual exported book; this partial audit has no book id.

Validation: full Plunge suite 199 passed/2 optional skipped, typecheck and build;
then focused auction/book suites after final changes and 15 native-tie parity
cases. The optional exported-book validation was explicitly run on all 133 preview
deals. The calibration test was explicitly run on all100 games (it is skipped
in ordinary CI without the external data path). More in Plunge docs-kiln.md.

## Full-catalogue repeated-hand edge case

The 1,000 preselected shuffles contain three repeated own hands:
seat2 at seeds420695/420778; seat2 at420702/421106; seat1 at421103/421573.
Plunge previously rejected duplicates. This is fixed in cdf6bfe: one lookup panel
per own hand/seat uses each cell's deepest saved evidence, regardless of favorable
score, current hidden deal or catalogue order. Common-depth scores and sampling
seeds must agree. Original per-deal receipts and generation audit rules stay intact.

The runtime and independent import validator passed tests covering all 1,000
actual shuffles with explicitly synthetic test-only prices, conflicting prices /
sampling seeds, higher-depth reuse and catalogue-order invariance. Build and 46
focused integration tests passed. This does not claim 1,000 measured panels exist.

The normal Plunge test run now automatically validates any installed book against
the real game shuffle and exercises all seat/target lookups. This was verified
by temporarily installing the 133-deal preview and running without KILN_BOOK /
KILN_PREVIEW environment overrides: all 10 catalogue tests passed. The preview
asset was removed and the null manifest restored afterward. Typecheck passed.
No preview is installed or eligible for production deployment.

## Solver and phone verification

- Core: retained/fresh comparisons include 4/12/40/160 worlds and exact work
  counters. Exhaustive rules, partnership, public-void sampler, frozen ordering
  and selection suites passed. One historical fixture-generator test is ignored.
- Thirteen Python tests passed after release-audit, producer-publication and cache-accounting work;
  they include actual abrupt-death/restart and malformed artifact rejection.
- Previous phone optimization: shared core at f91533cc imported as WASM SHA
  `ce0e5a958b6dfe42c77080ab85126905d105039bdaf425a1b0ebf1103cec251d`.
  Budgets, sample sizes, partner review and auction policy remain unchanged.
- Native/WASM complete option vectors, full partner check, malformed requests,
  deadline interruption, auction/job/merge/tie checks all passed on this build.
  Plunge build passed; full tests **200 passed / 2 optional skipped**.
- Direct previous/current WASM comparison passed all 12 complete decisions,
  exact option vectors, search counters and full partner review. It includes all
  nine declarations and higher bids. Host time is frozen for policy comparison.
  phone-before-after-v2.json and phone-before-after-run2/ preserve the final
  evidence. The first attempt found a checker label mistake (a changed partner
  decision correctly uses baseline-reviewed); the corrected full run passed.
  Mac Node WASM timing was about 3.7x faster; this is not a Pixel measurement.
- Latest phone build: shared core **1dbea2c02f471f2eace576bad022168ba0f61ad6**,
  WASM **3a6d7d6790033e3b21a1de244bf3cdf74f0bd3e85b83cd96b50d7680623eecf8**,
  769,304 bytes. Another 12 complete decisions, exact vectors/counters and full
  partner review matched the preceding optimized WASM under frozen host time.
  Paired Mac Node timings measured 1.172x median / 1.154x geometric speedup.
  Native/browser play, auction and deadline conformance passed again; native
  walt-table and partner_rollout were rebuilt too. Plunge build and 38 focused
  phone/table/auction/catalogue tests passed; one external-book test skipped
  because the manifest is still null. Budgets/sample counts/auction policy remain
  unchanged. Evidence: sparse-phone-build-summary.json and campaign
  sparse-phone-*.json / sparse-phone-*-run/. Preceding asset is phone-before-sparse/.
- The exact native calibration binary was preserved before rebuilding at
  calibration-six36/player/d970a82b7bc38971311275ca0e6b6dac0e2fbc799840e57909b6858b7ea25768/walt-table.
  The previous phone WASM/manifest are in phone-before/. Original calibration
  evidence is never relabeled as a new-binary experiment.

## Next actions

1. Revalidate PID32719 and sleep guard32720, database progress/errors, and git state. Core, pool and
   phone checks passed, and both worktrees are committed. No pending test process
   is intentionally left running. Production is the only sustained job.
2. Keep the long run producing all 1,000 deals and planned refinements. Tuning
   now has measured gains and a measured process/thread choice. Further changes
   need a concrete reason and bounded exact/parity measurements.
3. Audit the preselected 2% deep cells for screening misses. Export final book
   only when all 1,000 deals and every refinement are settled, then audit with
   --book and require no pending/running/failed work. Keep the calibration finding.
4. Install the final book in Plunge with no preview flag. Remove any superseded
   generated assets. Run actual-book checks, build/tests and local browser smoke
   including catalogue bidding, live play, next-hand and reload.
5. Merge/push appropriate branches to main and deploy Plunge (authorized by goal).
   Verify actual production via browser automation, including catalogue bidding,
   live move execution, saved receipts/questions and reload. Only then complete.
6. Preserve Sunshine: quick lawful partner-aware play; evidence-calibrated guesses,
   original receipts/questions feeding Scheme gym; future belief/workshop work.

Deployment: Plunge GitHub Actions main uses Node22 and Cloudflare deploy. Existing
scoped Cloudflare token service is cloudflare-table42; never print token values.
Account eb6564e57c2aebe97bbc5d33a0ffe5cb. Production URL:
https://plunge.jasonyandell.workers.dev/. Verify auth/workflow state when needed.
