# Paused handoff — 2026-09-21

User explicitly paused the active goal because the campaign exceeded their
usage budget. Goal status is PAUSED, not achieved. No experiment process was
running when checked. Do not resume experiments or agents without user direction.

## Collected gains

The production CPU optimization is already landed in the partnership checkout:
`701e8589` enabled it and `f1a0fb04` retained matched-current-baseline evidence.
`walt/receipts/cpu-speedups-v34/comparison.json` records 12 paired games with
all completed choices and exact values equal, and a median paired speedup of
13.125x. This is a small measured engineering panel, not a universal ratio or
new strength claim. `3cf2536d` records its separate portable WASM/Plunge release;
see `/Users/jason/code/texas-42-partnership-launch/walt/CPU-PHONE-RELEASE.md`.
Timed decisions can differ because the same deadline can now finish more work.

The exploratory worktree has compiled lawful role/lead-follow Scheme policies,
finite cost-vector teachers, an imperfect-information SUM/MAX response player,
CPU and WGSL execution, focal tail policies, native/GPU conformance tests and
WASM build verification. Existing validation is committed. This changes the
modeled lower policies and is not byte-compatible with historical Walt.
Phone/browser performance of this new compiled player is unmeasured.

## Fresh confirmation failed the quality gate

Both replacement C/D panels finished despite the agent hitting its usage limit.
Each had 576 mirrored pairs / 1,152 games. The preserved harness summaries are:

| Panel | Wins | Losses | Ties | Candidate mean / p95 ms | Current Walt mean / p95 ms |
| --- | ---: | ---: | ---: | --- | --- |
| C | 63 | 83 | 430 | 42.69 / 80.98 | 90.53 / 216.33 |
| D | 44 | 74 | 458 | 42.53 / 77.07 | 90.67 / 218.56 |

Combined: 107 wins, 157 losses, 888 ties across 1,152 pairs / 2,304 games.
Mean paired contract difference is -0.043403. Mean partnership time is
42.610ms versus 90.600ms (2.13x faster). Timing covers all 14 decisions
of a partnership, not all-four-seat selfplay. Candidate cap is 20ms/move;
current Walt retains its normal 14,000ms allowance and six threads.

Both panels report all replays passing and zero failed arms or fallbacks.
All 32 outer watchdog cycles completed; all inner cycle identity checks passed.
The additional independent full receipt/statistics audit was staged but NOT
run. No positive strength claim is justified: both panel means are negative,
so the preregistered positive-each-panel condition already fails. The earlier
8/5/59 result came from repeatedly used development deals and did not transfer.
Keep current Walt as the default; do not promote this compiled candidate.

## Preserved failure and pending work

Original confirmation A stopped after 77 games when a manually retyped C1
path was rejected in preflight. No worker/game began on that bad invocation.
Original A/B evidence remains separate and incomplete; it was not rerun or
silently dropped. Replacement C/D used new fixed namespaces, unchanged
candidate/gates, and a manifest-driven cycle runner.

Unexecuted audit code and tests previously in /tmp are copied to
`paused-staging/2026-09-21/`. They are staging artifacts, not validated live
integration; check their path assumptions before use. These include the
confirmation reporter/checker and historical-H0 diagnostic aggregator.
Historical-H0 label campaigns and deeper C2 teaching have not run. A small
read-only gym integration audit exists in the task conversation; no adapter
was installed. There is no active experiment left to babysit.

## Assessment and a bounded possible restart

The cheap lower-policy representation substantially reduces runtime, but its
current policies lose useful playing strength. More outer samples and several
small representation/teacher changes did not solve that tradeoff. The exact
GPU trace-union benchmark also did not provide a trick-1 speed win: at seven
tiles CPU averaged 32.86ms versus GPU plus fold 72.93ms on its fixed compute
panel; CPU folding dominated. Do not infer a GPU throughput breakthrough.

If the user later resumes, first agree an explicit spend/time budget and one
stop/go experiment. Prefer a diagnostic of actual modeled-query states and
policy approximation errors over another broad series of blind variants.
Saved fresh losses can be development evidence thereafter, but any eventual
new strength claim needs new untouched deals. Nothing in this note authorizes
resumption while the goal remains paused.
