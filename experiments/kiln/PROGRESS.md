# Kiln work ledger

Updated 2026-09-18. The goal remains ACTIVE and incomplete. Full 1,000-deal
production, screening audit, final book installation, deployment and production
browser verification are still required. Do not recreate the campaign or mark
the goal complete from the local integration preview.

## Current checkouts and commits

- Texas 42: `/Users/jason/code/texas-42-partnership-launch`, branch
  `codex/partnership-launch`. Kiln creation/count arithmetic at 101d805b;
  calibration/export/scheduling at 9bb393d7; compact caches at 662f298c.
- Plunge: `/Users/jason/code/plunge-sunshine`, branch `codex/sunshine-table`.
  Integration committed as ef5d92d. Its manifest is intentionally null until the
  complete book is installed. No Kiln changes have been merged/pushed/deployed.
- Base cwd `/Users/jason/code/texas-42` is an unrelated checkout; do not edit it.

## Production (revalidate; this file is not liveness evidence)

Campaign: `/Users/jason/data/texas-42/kiln-v1`. SQLite/WAL are authoritative.
Coordinator **86060** was launched detached, 18 processes, one internal thread,
`--seconds 0 --job-ms 120000 --order deal`. The exclusive OS lock prevents another
writer. Process metadata is production-process.json; status.json is expendable;
production.log now includes covered AND fully refined deals, and per-depth totals.
At the last completed firing: 173 covered deals, 4 fully refined; 81,290 initial
prices, 1,836 at 40 worlds, 1,169 at 160. Zero job errors so far.

Current native worker SHA:
`cc2bc646629c6e872ce3e8fbfb307799139996b9c75823c8480aac7dd2b02a27`.
Immutable binary and exact source snapshot are under producers/<sha>/.
The previous counted worker SHA is
`2b0adffe75e60312e5d8af8e1ac6091900b16850d6981a549a4392265da29094`.

Initial 8-world coverage ran around 64–72 prices/s with 16–18 workers. Refinement
is substantially slower: first 60-second deep firings retained 375 and 435 jobs.
The compact-cache production firing retained 404. These are DIFFERENT jobs and
are not controlled speedup factors. A controlled 16-case paired 160-world check
measured 1.299x median / 1.296x geometric speedup from compact caches. All prices,
nodes, policy calls and inner sample counts matched. See COMPACT-CACHE.md and
compact-cache-timing-summary.json. Actual whole-catalogue completion may take
many hours; keep the continuous run producing while pursuing useful work.

Scheduling is selectable: coverage (all base cells first), depth (refinements
first), deal (finish each deal in seed order, refinements first). An added queue
index supports deal scheduling. This changes work order, not sample semantics.
Every result/refinement insertion remains one FULL/fsync transaction. A bounded
run cancels unfinished prices only. Completed production progress is retained.

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

## Solver verification

- Counted integer success values replace internal BigRational with the same
  public rational results; exact finite equal-weight recurrence in COUNTED-VALUES.md.
- Compact private cache keys preserve every identity field. Packed unfinished
  trick is injective (exhaustively tested lengths0..3 including zeros). Fast hashes
  route tables only, full equality resolves collisions; sampling hashes untouched.
- Each optimization passed 121 exact/counter parity cases, including 4/12/40/160.
- Rust: 12 partnership, 7 sampler, 4 ordering/frozen-value, 7 selection tests;
  packing unit test. One historical fixture-generation test intentionally ignored.
- Six Python tests passed again after latest scheduler/status/export changes,
  including actual SIGKILL/restart retaining all committed receipts.

## Next actions

1. Revalidate PID86060, database progress, errors and disk/resources. Never restart
   solely from an old status snapshot or observation timeout.
2. Continue useful throughput work if justified. Profile showed hashing/allocation
   costs. Remaining obvious allocation candidates: 28-vector bucket outer storage
   and intermediate child-value vectors. Any change needs exact/counter parity and
   bounded paired measurement. More ambitious all-target Dice vector reuse is
   only an idea; do not infer one target's score from another.
3. Finish all1000 deals and planned refinements. Audit the preselected2% deep cells
   for heuristic screening misses; validate complete book against stored receipts,
   profile identities, real Plunge shuffles, pending/failed counts and integrity.
4. Export finalbook, install in Plunge with no preview flag, remove superseded
   generated book assets if any, update docs from preview to final. Run actual-book
   checks, build/tests and local browser smoke including next-hand/reload.
5. Merge/push appropriate branches to main and deploy Plunge (authorized by goal).
   Verify actual production via browser automation, including catalogue bidding,
   live move execution, saved receipts/questions and reload. Only then complete goal.
6. Preserve Sunshine: quick lawful partner-aware play; evidence-calibrated guesses,
   original receipts/questions feeding Scheme gym; future belief/workshop work.

Deployment: Plunge GitHub Actions main uses Node22 and Cloudflare deploy. Existing
scoped Cloudflare token service is cloudflare-table42; never print token values.
Account eb6564e57c2aebe97bbc5d33a0ffe5cb. Production URL:
https://plunge.jasonyandell.workers.dev/. Verify auth/workflow state when needed.
