# Kiln work ledger

Updated 2026-09-18. The goal remains ACTIVE and incomplete. Full 1,000-deal
production, screening audit, final book installation, deployment and production
browser verification are still required. Do not recreate the campaign or mark
the goal complete from the local integration preview.

## Current checkouts and commits

- Texas 42: `/Users/jason/code/texas-42-partnership-launch`, branch
  `codex/partnership-launch`. Core optimization committed at f91533cc, following
  1a43d763 (small support) and 7377ff32 (allocation removal and release audit).
- Plunge: `/Users/jason/code/plunge-sunshine`, branch `codex/sunshine-table`.
  Book integration is ef5d92d; optimized phone asset and validation notes are
  committed as **70e170c**. The book manifest remains intentionally null.
- No Kiln changes have been merged/pushed/deployed. Base cwd
  `/Users/jason/code/texas-42` is unrelated; do not edit it.

## Production (revalidate; this file is not liveness evidence)

Campaign: `/Users/jason/data/texas-42/kiln-v1`. SQLite/WAL are authoritative.
Coordinator **41774** was launched detached, 18 processes, one internal thread,
`--seconds 0 --job-ms 120000 --order deal`. Process metadata is
production-process.json; status.json is expendable. The exclusive OS lock
prevents another writer. Graceful SIGTERM retains completed prices and returns
unfinished work to the queue. Check actual process identity before stopping it.

Last checked: 173 base-covered deals, **51 fully refined**, no job errors. The
latest one-minute production firing retained **1,071** prices; the continuous
run was around 17 prices/s while other validation work was running. These counts
are production progress, not controlled speedups across different job mixes.
Order `deal` is finishing refinements in the covered prefix before new coverage.
The latest process snapshot showed 18 workers using about 1,775% CPU
(roughly all 18 cores) and 1.2 GiB aggregate RSS. The full catalogue remains hours of work. Leave production running alongside
useful integration and validation; do not replace the goal with this partial book.

Current immutable native worker SHA:
`c46780919a239d8314e0abc9e06234eb6678c777e0490c3b966587f064d1d9a1`.
Binary and exact source snapshot are under producers/<sha>/. The running process
uses that copy, so rebuilding Cargo outputs does not alter active work.

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
No claim that this finds a universal optimum over every workload.

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

Validation: full Plunge suite 199 passed/2 optional skipped, typecheck and build;
then focused auction/book suites after final changes and 15 native-tie parity
cases. The optional exported-book validation was explicitly run on all 133 preview
deals. The calibration test was explicitly run on all100 games (it is skipped
in ordinary CI without the external data path). More in Plunge docs-kiln.md.

## Solver and phone verification

- Core: retained/fresh comparisons include 4/12/40/160 worlds and exact work
  counters. Exhaustive rules, partnership, public-void sampler, frozen ordering
  and selection suites passed. One historical fixture-generator test is ignored.
- Twelve Python tests passed after release-audit and producer-publication work;
  they include actual abrupt-death/restart and malformed artifact rejection.
- Phone: shared core at f91533cc imported as WASM SHA
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
- The exact native calibration binary was preserved before rebuilding at
  calibration-six36/player/d970a82b7bc38971311275ca0e6b6dac0e2fbc799840e57909b6858b7ea25768/walt-table.
  The previous phone WASM/manifest are in phone-before/. Original calibration
  evidence is never relabeled as a new-binary experiment.

## Next actions

1. Revalidate PID41774, database progress/errors, and git state. Core, pool and
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
