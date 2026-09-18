# Kiln work ledger

Updated 2026-09-18. The goal remains ACTIVE and incomplete. No prior Kiln files or
running jobs existed when this goal continuation began; this turn built the
first implementation and started actual production.

## Verified implementation

- `kiln.py`: deterministic 1,000-deal catalogue, SQLite queue/results, 8/40/160
  refinement, explicit heuristic stops plus 2% audit, immutable producer copies
  and source archives, one FULL/fsync transaction per completed price, exclusive
  writer lock, retries, interruption/recovery, status and complete-deal export.
- `kiln-worker`: persistent native transport around the existing auction solver.
  Own hand only; original hidden opponents are never evaluation inputs.
- Solver optimization: exact integer success counts internally, BigRational at
  public boundaries; avoid nested Rayon dispatch in single-thread workers.
  See COUNTED-VALUES.md. No floating-point approximation and no policy change.
- 121 cases matched original prices, nodes, policy calls and inner sample counts,
  including 4/12/40/160 worlds. Repeated after serial-dispatch optimization.
  Sources: counted-parity-summary.json and serial-counted-parity-summary.json;
  full receipts under the external campaign directory.
- Six Python tests passed: all 1,000 shuffle seeds against actual Plunge RNG,
  four native endpoint parity cases, SQL identity/refinement/export, actual
  SIGKILL/restart preserving committed rows, calibration completions, summaries.
- Rust tests: 12 partnership, 7 sampler, 4 ordering/frozen-value, 7 selection
  tests passed. One historical fixture regeneration test intentionally ignored.
- SQLite integrity_check returned ok; a preview book was exported successfully.
  No Plunge changes or deployment yet.

## Live work (revalidate processes; this file is not liveness evidence)

Campaign: `/Users/jason/data/texas-42/kiln-v1`

Production process 55050 was verified live with PPID 1. Six initial one-minute
firings retained 20,875 prices. Raw throughput measurements are committed in
throughput-initial.json. Different jobs and some concurrent checks mean these
are scheduling evidence, not controlled speedup factors. Approximately 70 prices/s
at 16–18 workers; 24 did not improve it. Current long run uses 16 processes,
1 internal thread, 120-second per-cell deadline, and no overall time limit.
Watch production.log, status.json, or `kiln.py status`; SQLite is authoritative.
Last sampled coverage: 65/1000 deals, zero fully refined deals.

Calibration process 56627 resumed the first 60-second firing, which had completed
18/100 games with 1 make. Case: sixes, bid 36, seat 3, own hand
[0,3,13,21,22,23,26], forecast 121/160 = 75.625%. Chosen before outcomes as nearest
to .78 among six initial .75-at-eight candidates; selection receipt and all six
160-world price receipts are in calibration-prices/. Uses original walt-table
binary (its SHA is pinned in case.json), actual deployed L1+partner 40/8 with
160-world opening, four workers. Each move saved; outcome certified by independent
rules when contract made/set, so complete need not mean all 28 plies played.
Last sampled result: 5/35 made. This large discrepancy
is preliminary calibration evidence, not a defect inferred from win rate alone.
Do not advertise model prices as calibrated odds. Keep evaluating all 100.

## Immediate next actions

1. Revalidate both coordinator PIDs plus database/result progress. Never restart
   from a stale log or an observation timeout. Production still has a large queue.
2. Finish and independently audit all 100 calibration games; record mismatch,
   intervals and declared policies. Distinguish best response to modeled sigma0
   from actual repeated replanning against deployed L1+partner players.
3. Continue throughput work. A promising next mathematical optimization is
   evaluating the sigma0 Dice-response *vector* of all 13 bid thresholds together
   (one transition tree, target-specific choices). It is only an idea; neither
   implemented nor proven/tested here. Do not reuse a scalar result across bids.
4. Implement Plunge book reader and catalogue dealing/bidding while generation
   proceeds. Keep ordinary auction cadence and current minimum-raise policy;
   do not introduce aggressive maximum bids justified by uncalibrated prices.
5. Finish all 1000 × 4 × 9 × 13 cells and planned refinements, audit screened
   cells, validate/export/version full book, integrate, deploy, browser-test actual
   production. README is the completion checklist; do not shrink its scope.
6. Preserve Sunshine: quick lawful partner-aware player; original receipts and
   saved questions; Scheme gym and future workshop. Kiln addresses bidding latency.

Data is outside Git. Source changes belong on codex/partnership-launch. Worktree
root: /Users/jason/code/texas-42-partnership-launch. Plunge companion worktree:
/Users/jason/code/plunge-sunshine. Other historical solver/proof worktrees are
unrelated and were not modified.
