# partner-race versus l1-race

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 2/50 mirrored deals; 2 in complete analysis groups (2 independent deal units).

A pair wins/losses/ties: **1/0/1**. Contract wins: **3 A / 1 B**.

A contract win fraction: **75.0%**. Equal strength is 50%. Points never break ties.

No uncertainty interval is shown: the report requires at least ten independent units and nonzero observed variance for its rough approximation. Raw scores remain descriptive evidence.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-race | 0.600 | 0 / 36 |
| partner-race | 3.582 | 10 / 33 |

Stopped: fallback-rate-exceeds-five-percent:partner-race

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
