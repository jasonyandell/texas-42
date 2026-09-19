# Kiln work ledger

Updated 2026-09-19. **User-directed expansion: 500 actual-play bidder hands.**
The old 1,000-deal scalar-price production target is superseded, not completed.
The app goal flag may still show the old usage-limited objective; it is not the
current work instruction. Do not restart the old campaign automatically.

**Active direction, 2026-09-19:** add400 bidder hands to the existing100, keeping
all42,008 prior games. Total500 hands /125 catalogue deals420600..420724 /4,500
hand-declaration panels. Existing8/40 screens remain; survivors get160 games.
At160 and320, an allocation heuristic checks whether any Wilson score-tail
interval straddles the80% bidding cutoff and continues uncertain panels up
to640. Preselected audit controls reach640 regardless. This allocates compute;
it is not a confidence guarantee or a new player. Old100-hand data contains17
uncertain panels and10 audit controls eligible for more games;106 additional
panels resolve at160 and767 retain their earlier screens. See [PLAYED.md](PLAYED.md).
The following completed-research sections describe earlier runs; they do not
override this authorized production extension. Inspect live status and process
identity for current progress. Keep the original100-hand final book unchanged.

**Latest completed research cycle:** [whole-game sampled contrasts](whole-game-contrasts-v1/RESULTS.md)
adds 360 new source games, 240 roots covering all six decision-bearing hand
sizes, 2,304 sampled worlds and 9,200 complete forced-action branches. All
passed independent replay; no errors, fallbacks or overruns. Three frozen
witness-derived Schemes did not clear the fresh transfer gate. Refining 24
development roots from eight to 24 worlds changed eight empirical preferred
moves while preserving all 960 initial receipt hashes. Full data is outside
the worktree at `~/data/texas-42/kiln-played-v1/whole-game-contrasts-v1`;
tracked results include 72 world contrasts, every refinement curve and all
candidate/query evidence. This extends discovery beyond the two-tile endgame.
No player change, production restart or monitor.

**Previous completed research cycle:** [decision mining](decision-mining-v1/RESULTS.md)
adds 360 new source games, 480 completed position/seed assessments on 240
two-tile coordinates, and 21,620 full paired continuations. It found 17 missed
position/seed cases across 13 positions; none of 114 simple action/gate candidates
had positive mean development gain, so the frozen rule retained the baseline.
Fresh action evidence, all controls and paired witnesses are preserved in
`~/data/texas-42/kiln-played-v1/decision-mining-v1`. Both batches and the fresh
source campaign are complete. No monitor, production restart or phone change.

Completed campaign: [PLAYED.md](PLAYED.md). 100 individual bidder hands
(25 complete Plunge deals × four seats), all nine declarations. Play the actual
deployed player at bid30 through all seven tricks, accumulate final-score
histograms, and recommend the highest score reached in at least 80% of games.
Build durable progressive production; tune throughput in at most one-minute
firings. Preserve the previous survey as useful, separately labeled evidence.

**Completion, 2026-09-18:** 42,008 complete games / 1,176,224 audited moves in
900 hand/declaration cells. 311 cells stopped at eight samples, 456 at forty,
and 133 reached 160. No failed jobs, deadline overruns, or fallback moves. Final
book: `/Users/jason/data/texas-42/kiln-played-v1/played-book-final.json`, identity
`5e26796cb85ee50aa16a956bd2e51721835359f48532cf89cf1e51eed47268e0`.
All completed receipts/producers and the complete book passed the independent
audit. See [completion evidence](played-completion.json). No phone installation.

Under the chosen 4/5 empirical score-tail cutoff, 12 of 900 panels qualify:
five at 30, six at 31, one at 36. Taking each hand's highest qualifying panel,
10 of the 100 hands qualify (four at 30, five at 31, one at 36). The other 90
do not clear this conservative empirical rule; that does not establish that
they are unbiddable under other risk thresholds or policies. The outcomes are
the bid30 player's achieved scores, not performance of retargeted higher bids.

The final history snapshot retains 693 recommendation changes, 254 after at
least eight observations and 111 after at least forty. These are movements of
an estimate, not independent strategic discoveries or causal explanations.

Completion backup verified locally and by immutable remote download: private
Hugging Face `jasonyandell/texas-42-walt-archive`, snapshot
`kiln/snapshots/20260918T154209.756236Z`, commit
`77ae730a1ebd50375569a9c295c5a73928b36140`. It includes the complete production
database, both mining probes, fresh 2,880-game validation corpus, histories,
producers, tools and final book. See [backup evidence](completion-backup-evidence.json).
At the100-hand completion the monitor was PAUSED, and both production
coordinators and the original sleep guard exited. The500-hand extension above
supersedes that stopping point; the old scalar survey remains stopped.

Old campaign stopped cleanly: **1,001,348 receipts, 851 settled deals**, no
running leases, 69,430 pending old prices. Coordinator69038 and sleep guard69039
are gone. All receipts and producer snapshots remain in kiln-v1. Export:
`/Users/jason/data/texas-42/kiln-v1/survey-book-partial.json` (14,420,032 bytes),
id `89717b76ec55216936d79e39d6d90ef48b2c27fb1b6c0d1e4429cc6d265052a8`.
Independent partial audit verified every receipt, producer and book in17.3s;
see survey-frozen-summary.json. This is a preserved model survey, not an empirical
bid database or a complete 1,000-deal book.

## Actual-play runner implemented

`played.py` and `kiln-play-worker` implement the user-directed100-hand empirical
campaign at `/Users/jason/data/texas-42/kiln-played-v1`. All players call the shared
deployed decision procedure; every retained game finishes28 legal moves and
stores its original actor-only calls and responses. Score histograms and4/5 tail
recommendations export as `kiln-played-book-v1`. This schema is not yet wired into
Plunge; the old model-book manifest remains null.

The initial60-second firing saved382 games at6.35/s; the independent audit checked
all10,696 moves. The new staged-cache reuse then matched all decisions, candidate
values, partner reviews and final scores in two72-pair comparisons (4,032 compared
moves total). Final worker91386e6e measured1.146× aggregate speedup on the second
comparison. Existing context-guard tests and new fresh-deadline/value tests pass.
29 focused Rust tests, four Python integration/property tests, and the browser
library compile passed (one existing unused-mut warning). No phone deployment.

The72-game pool comparison selected18 workers with one internal thread.24 is
within repeated18-run variation;12 is slower. The second60-second firing saved401 more games at6.67/s, again with no errors,
overruns or fallbacks. All783 games (21,924 moves) passed the independent audit;
see played-startup-audit.json. Continue the same campaign toward160 games per
surviving cell; query the database for current counts, not this note.
Every8 or40-game early screen remains explicitly labeled and a preselected2%
audit subset bypasses screening. All prior games and the old model survey remain.

Continuous actual-play production started from commit **170178e2**. Verified
coordinator **93902**, sleep guard **93903**, 18 workers, one internal thread,
target 160 games per surviving cell. Process metadata is in the campaign's
production-process.json; verify actual identity before signaling. The OS sleep
assertion was verified. All 900 hand/declaration cells have at least one completed
game; the first eight-game pass remains in progress. Latest pinned audit/export
snapshot is played-live-audit.json. A 15-minute thread heartbeat,
`kiln-actual-play-production`, checks completion/errors and the first eight-game
coverage milestone. It does not silently restart a user stop. The empirical
preview remains local; no new game asset has been installed or deployed.

## Data durability

Live data already resides outside the worktree in `/Users/jason/data/texas-42/`.
Verified local snapshot: `backups/20260918T141408.625440Z/`, containing the complete
frozen survey (1,001,348 evaluations) and an online snapshot of actual play
(8,161 games, 228,508 moves). Both databases passed integrity checks; every archive
member was read back and hashed. The actual-play audit also passed using only
archived runner tools, without importing code from the worktree.

All archives, manifest and restore guide were uploaded to the existing **private**
Hugging Face dataset `jasonyandell/texas-42-walt-archive`, under
`kiln/snapshots/20260918T141408.625440Z`, commit
`f4a347dff80ece5bb2ae9696129a5ce07f3fffd4`. Remote downloads matched every local
archive checksum and both metadata files. See backup-evidence.json. Production
was never stopped. Later eight-game/completion milestones also back up the
growing actual-play campaign. The helper reuses the authenticated HF CLI's Python
when the checkout's Python lacks huggingface_hub; no new login is necessary.

## Sample history for later belief studies

`history.py` is a read-only companion, committed as **ec308ba7**. It reconstructs
all empirical sample prefixes and recommendation changes, including non-changing
controls, and emits original-game explanation packets. The first report covers
12,056 games / 900 cells: 511 changes after the first observation, 72 after at
least eight games. Cell 351 documents a 35-to-33 recommendation change on game17.
All 900 histories reconstructed exactly; that packet remained identical after
more live games arrived. Four focused tests passed. No sampler or player change.

The report, packet, raw data and tools are also in verified private Hugging Face
snapshot `kiln/snapshots/20260918T142655.519133Z`, commit
`d03d928614069a47b769a96c6786747a3b0a813c`, with 13,320 games in its database.

## Outcome-first Scheme probe

[Protocol](THREAT-PROBE.md), [results](threat-probe-v1/RESULTS.md). User-directed
preliminary discovery from observed worlds rather than prewritten tactical
patterns: retain subsets of ownership facts from losing games, compile the
learned Schemes, and test on separate worlds and source deals. The frozen first
eight trials supply 7,200 games but only 800 hidden completions (declarations
share completions). All original receipts were independently revalidated.

57,008 clauses searched; eight frozen candidates; 244,800 actual Scheme membership
checks including seed descriptions and later ablations, no mismatch. Four Python
checks and one Rust input check passed. Primary transfer evidence is weak; a
secondary double-five ownership association merits further investigation. The
post-hoc simpler query retains a new-hand association, but is not confirmed as
a causal threat or useful sampling rule. Full artifacts are outside the checkout
under the live campaign's `threat-probe-v1/`; compact evidence and queries are
tracked. No player, sampler, production scheduling, or phone change.

## Fresh relational mining probes (complete)

User authorized further probes toward a reusable mining rig. The fixed
[v2 protocol](MINING-PROBE-V2.md) freezes 13 queries (eight original literal
patterns, the simpler double-five pattern, four learned relational patterns)
before assessing ten completely new source deals. The separate nested campaign
`kiln-played-v1/mining-v2/fresh/` runs 2,880 games / 320 completions, 40 bidder
hands, all declarations, eight trials, no screening. It uses the same pinned
player binary, with its own durable queue. Original bidding production continues.

Relational discovery generated 411 descriptions from the same 64 losing
discovery worlds. The top queries describe opponents holding the live top trump,
and opponents holding a double that beats an own-hand trump lead. All selected
patterns were frozen before fresh evaluation. Actual Scheme audit: 946,944
full-library and 28,080 selected-training memberships, no mismatch. The active
fit is `mining-v2/analysis-v2/fit.json`, ID
`173bd310c97901a6f4629117bcc082492b49bff272c9b2abcaa649578f3fbe8a`.
An earlier atom-count metadata correction preserved every selected query and
their order; see [amendments](MINING-V2-AMENDMENTS.md).

A [pre-label measurement-efficiency extension](MINING-PROBE-V2-CV.md) froze
control-variate coefficients from discovery data only. It tests whether the
known 1/3 opening ownership probability reduces variance of fixed-player
bid-success estimates. It does not change Walt's internal samples or live play.

[Fresh results](mining-v2/RESULTS.md): both predeclared primaries passed on 2,880
new games / 320 completions, with no errors, overruns, or fallbacks. The simpler
double-five association had +4.65 pp failure excess; learned top-trump ownership
had +10.13 pp. The old strongest training pattern disappeared (-0.48 pp).
The frozen top-trump correction reduced measured estimator variance by 2.34%
overall; the double-five correction's 0.53% reduction was inconclusive. These
are fixed-player outcome/measurement results, not stronger play or faster Walt
planning. All 13 queries and negative/control results are in the evidence
catalog. Eight focused tests passed; 1,012,464 Scheme membership checks and
80,640 fresh plays passed their audits. The tools now separate frozen fitting,
fresh evaluation, variance checks and catalog publication. Original bid
production and the deployed phone player were not changed.
See SAMPLE-HISTORY.md and sample-history-evidence.json. The monitor regenerates
content-addressed history reports before subsequent milestone backups. Internal
solver-world traces are a separate future instrument; do not call an empirical
threshold flip a causal strategic discovery.

The remainder of this ledger records the **historical model-survey work**.
Its old production PIDs, completion requirements and restart suggestions are
historical, superseded by the paragraph above.

## Current checkouts and commits

- Texas 42: `/Users/jason/code/texas-42-partnership-launch`, branch
  `codex/partnership-launch`. Latest shared-core optimization is **91e8925e**
  (precomputed trick strengths), following 850f589b (native policy
  cache carry) and 1dbea2c0 (sparse Dice
  buckets), following f91533cc (move ordering), 1a43d763 (small support) and
  7377ff32 (allocation removal and release audit).
- Plunge: `/Users/jason/code/plunge-sunshine`, branch `codex/sunshine-table`.
  Book integration is ef5d92d; optimized phone asset and validation notes are
  committed as **70e170c**. Repeated own-hand reconciliation is **cdf6bfe**; automatic installed-book
  validation in the normal test run is **9963705**. Reusable browser release
  verification is **b891e8e**. Latest matching phone asset is **a0437a1**.
  The book manifest remains intentionally null.
- No Kiln changes have been merged/pushed/deployed. Base cwd
  `/Users/jason/code/texas-42` is unrelated; do not edit it.

## Production (revalidate; this file is not liveness evidence)

Campaign: `/Users/jason/data/texas-42/kiln-v1`. SQLite/WAL are authoritative.
Coordinator **69038** was launched detached, 18 processes, one internal thread,
`--seconds 0 --job-ms 120000 --order deal`. Process metadata is
production-process.json; status.json is expendable. The exclusive OS lock
prevents another writer. Graceful SIGTERM retains completed prices and returns
unfinished work to the queue. Check actual process identity before stopping it.
Mac sleep guard **69039** is `/usr/bin/caffeinate -i -w 69038`; its actual
PreventUserIdleSystemSleep assertion was verified. It ends with this coordinator.
If restarting production, bind a new guard to the new actual coordinator PID.
Metadata is power-guard.json; as always, verify live processes rather than the file.

Halfway audit checkpoint: **502 settled deals**, **589,743 receipts**
(234,992 at 8 worlds, 215,711 at 40, 139,040 at 160). The independent read-only
audit verified every saved receipt, refinement chain, and all 12 recorded
source/binary identities in 18.4 seconds; no failed jobs were present. Of 1,938
preselected deep audit cells that would have screened early, none reached the
75% model threshold; 55 reached 50%. There are 4,711 of 9,420 preselected audit
cells at target depth. These correlated cases remain descriptive screening
evidence, separate from actual-play calibration. Evidence: audit-halfway.json
and the committed halfway-audit-summary.json. This remains a partial audit with
no book id; all 1,000 deals and the final book audit are still required.
PID69038 and sleep guard69039 remained live throughout. After the audit, all
18 native workers were busy (1,767% aggregate CPU, about 1.9 GiB aggregate RSS).
Recent production intervals continued at roughly 42 saved evaluations/second
with no run errors. Both worktrees were clean before recording this evidence;
the Plunge book manifest is still null. No production restart occurred.

Earlier one-third audit checkpoint: **335 settled deals**, **394,287 receipts**
(157,128 at 8 worlds, 144,195 at 40, 92,964 at 160), verified structure and
source/binary identities, no failed jobs. Of 1,317 preselected deep audit cells
that would have screened out, none reached the 75% model threshold; 38 reached
50%. These correlated cells remain descriptive screening evidence, separate
from actual-play calibration. Evidence: audit-one-third.json and the committed
one-third-audit-summary.json. It is still a partial audit without a book id.
PID69038 continued writing fresh receipts during the read-only audit; later live
checks passed 337 settled deals with no errors. No production restart occurred.

Earlier quarter-catalogue checkpoint: **251 settled deals**, **294,840 receipts**
(117,534 at8 worlds,107,909 at40,69,397 at160), zero failed jobs. PID69038 was
confirmed live and writing fresh receipts after two bounded waits. Production
continues on the validated rule-table worker. A compiler feature inspection
found the default target already supplies modern integer atomics and NEON;
no further compiler build or production interruption was made. See
compiler-target-summary.json. The final-book release gates remain open.

Previous verified checkpoint: **238 settled / 238 covered**, run34, zero errors.
The previous continuous run was stopped for one isolated serial-grouping
experiment. That edit matched64 cold cases and all192 prices in64 paired ladders,
but improved timing only0.4%, so it was reverted. Production resumed on the same
validated rule-table worker as PID69038 with verified sleep guard69039. The
phone asset did not change. See serial-grouping-*-summary.json and HOT-PATH.md.

Previous rule-table adoption checkpoint: **224 settled / 224 covered** deals,
263,269 saved evaluations, run33 about42.7 saved prices/s with zero errors. The
new rule-table worker completed a bounded production firing of2,436 prices in
60.2s, zero errors, before continuous production resumed. PID58752 and its actual
sleep assertion58753 were verified. Current worker SHA is given below.

The finite rule lookup passed all2,016 direct-algebra comparisons,44 focused
Rust tests (one historical ignored fixture),64 cold retained/fresh scalar and
work-counter cases, and32 then64 paired persistent 8/40/160 ladder cases. The
larger repeat measured1.047x median /1.045x geometric speedup, supporting adoption.
The committed worker rebuilt byte-identically to the tested source snapshot.
Use the original `-p walt-player --bin kiln-worker` build invocation for this
identity; adding `-p walt` enables its default feature flag and changes build
identity even though the implemented evaluator is unchanged.

The independent partial audit passed **262,177 receipts**, including the new
producer's original source and binary. Its pinned snapshot had223 settled deals.
Of874 preselected audit cells that would have screened early, none reached75%
at160 worlds;27 reached50%. This remains descriptive, correlated evidence, not
a calibrated campaign-wide confidence bound. Evidence: audit-after-rule-table.json.
Final completion still needs the full exported book and a final audit with--book.

Previous verified checkpoint: **202 settled / 202 covered** deals,
237,664 saved evaluations, run29 about38 saved prices/s with zero errors. Run28 was stopped only for the
isolated 25-second packed-hash experiment; its completed work was retained. The
candidate preserved all checked prices but measured only about1% faster, so the
source edit was reverted and the existing worker resumed as PID45275. Its sleep
guard is45277. See packed-hash-*-summary.json and HOT-PATH.md. The comparison
instruments were also corrected to reprice warm receipts before cold-counter
comparisons; 36 actual warm/fresh parity cases and four paired cases passed.
All 32 warm fixture receipts differed in cold work counts, while their prices
matched. The restored release output was byte-identical to the current production
worker. Both the new coordinator and its actual idle-sleep assertion were verified.

Last audited: 173 base-covered deals, **172 fully refined**, no job errors. The
latest one-minute production firing retained **1,562** prices; the new continuous
run was around 24 prices/s while validation was running. These counts
are production progress, not controlled speedups across different job mixes.
Order `deal` is finishing refinements in the covered prefix before new coverage.
The latest process snapshot showed 18 workers using about 1,775% CPU
(roughly all 18 cores) and 1.2 GiB aggregate RSS. The full catalogue remains hours of work. Leave production running alongside
useful integration and validation; do not replace the goal with this partial book.

Current immutable native worker SHA:
`e6cea7cf0d62796e60f0756d849098a3fa64c2985c6dc207fac37ed65c0a748d`.
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
- Previous sparse-bucket phone build: shared core **1dbea2c02f471f2eace576bad022168ba0f61ad6**,
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
- Latest phone build: shared core **91e8925e8468bfebb753774f9b441f6d4a13f1fc**,
  WASM **95192ca8aa5b161a9de540de3b210cdc56154b3d57890dfe30eaa15fbe1f3172**,
  796,381 bytes, imported in Plunge a0437a1. Twelve full decisions, exact option
  vectors/counters and completed partner review matched the preceding asset.
  Native/browser play, auction and deadline checks passed; the production build
  and43 focused game tests passed (one external-book test skipped with null
  manifest). Mac Node WASM timing measured1.104x median /1.097x geometric speedup
  during native production; not Pixel timing. Budgets, samples and auction policy
  remain unchanged. Evidence: rule-phone-build-summary.json and campaign
  rule-phone-*.json / rule-phone-*-run/. Preceding asset: phone-before-rule-table/.
- The exact native calibration binary was preserved before rebuilding at
  calibration-six36/player/d970a82b7bc38971311275ca0e6b6dac0e2fbc799840e57909b6858b7ea25768/walt-table.
  The previous phone WASM/manifest are in phone-before/. Original calibration
  evidence is never relabeled as a new-binary experiment.

## Next actions

1. Revalidate PID69038 and sleep guard69039, database progress/errors, and git state. Core, pool and
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
