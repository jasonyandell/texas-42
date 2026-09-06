# Shared Texas 42 experiment pool

Jason authorized multiple seeds/experiments in flight on 2026-09-06, including
ten simultaneous games, interrupt/restart, and retrying failed work. This
supersedes CAMPAIGN.md's original one-active-seed scheduling restriction.
The player, bids, samples, generator, and existing campaign identities remain
unchanged. Execution concurrency is recorded separately in pool sessions and
per-game attempt receipts, including each attempt's starting and ending ply.

## Measured throughput

The first ten-worker benchmark repeated seeds 420636–420647: **36 games in
84.642 seconds**, with peak concurrency ten. The earlier three-games/one-seed
schedule had a 250.236-second critical path on those exact deals, computed as
the sum of each seed's longest recorded game, excluding controller overhead.
That is about **2.96x faster** against this recorded baseline, not a newly
rerun controlled timing trial. One system snapshot during the benchmark showed
90.55% CPU busy and zero swap-in/out increments in that snapshot.

All 36 make/set outcomes matched. Two full play trajectories changed, and
fallback decisions increased from 8 to 14. The pooled fallback rate was 14/654
nonforced decisions. Every game passed independent replay; the longest
decision was 13.931 seconds and longest four-play trick 19.698 seconds. No
worker failed or needed retry. The benchmark is repeated known-deal calibration,
not fresh strength evidence, regardless of the generic campaign summary's
fresh flag. Records are under `campaigns/bench-pool10/`.

Ten is a useful measured setting, not a proof of a global optimum. Because
deadline fallbacks can alter play, the primary report explicitly identifies
the change from three-game slices to the ten-game shared pool, beginning after
48 fully committed seeds. Partial seed 420648 resumes its saved moves.

## Queue and durability

`campaigns/pool-queue.json` lists campaign directories relative to itself and
sets the shared worker limit, slice allowance, and retry limit. Add authorized
future campaigns to this list to share the same Mac budget. Work from separate
campaigns is interleaved fairly; a free worker immediately starts another game.
An additional pool lock prevents independent pool processes in this worktree
from oversubscribing the configured budget. Per-campaign locks also exclude
the legacy runner. All high-throughput runs should use this shared queue.

Each game retains atomic per-move checkpoints. Each launch has a durable
`pool-attempts/` record. A failed launch/process retries from its latest saved
move, with at most two retries (three failed attempts total) before that
campaign receives a STOP marker. Interruptions do not count as failed attempts.
Other queued campaigns may continue. A valid deadline fallback remains part
of the executed policy; it is never retried to seek a different game outcome.

Games can finish in any order, but results are committed only as a contiguous
seed prefix. This keeps fast games from selecting the evidence used by the
sequential stop rule. Fully finished later games remain durable while waiting
for preceding seeds. An interrupted pool resumes all unfinished games; no
completed seed is recomputed. The player and original runner hashes remain
pinned. Each pool session records its scheduler hash and actual worker limit.

## Running and monitoring

Always place the pool under the watchdog. Use a new output directory:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/campaigns/pool-caps/UNIQUE -- python3 experiments/partnership/pool.py --queue experiments/partnership/campaigns/pool-queue.json
```

Alternatively, pass multiple campaign directories directly to `pool.py`, with
`--workers 10 --seconds 260 --retries 2`. Native threads remain as pinned in
each campaign manifest. The pool's `status.json` and `STATUS.md` show all active
games and their saved move counts. Run `verify_campaign.py PATH` and
`campaign_report.py PATH` before/after slices as appropriate.

The existing `campaign.py stop PATH` command stops that campaign, including
every one of its active workers after the current move. `resume` explicitly
clears that marker; the next pool slice resumes the saved work. Process-group
interruption is also recoverable and bounded by the watchdog. No workers run
between foreground slices. The thread's reusable monitoring automation is
`texas-42-make-set-evaluation` (display name Texas 42 experiment pool); its
one-minute schedule reduces scheduling gaps, and locks prevent duplicate runs.

Focused tests cover out-of-order completion, round-robin campaign scheduling,
skipping completed arms, durable retry accounting, and injected launch failures
that exhaust retries without manufacturing a result.
