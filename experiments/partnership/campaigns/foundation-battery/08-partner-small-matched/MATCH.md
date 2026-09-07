# partner-race-small versus l1-race-small

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 2/25 mirrored deals; 2 in complete analysis groups (2 independent deal units).

A pair wins/losses/ties: **0/0/2**. Contract wins: **2 A / 2 B**.

A contract win fraction: **50.0%**. Equal strength is 50%. Points never break ties.

No uncertainty interval is shown: the report requires at least ten independent units and nonzero observed variance for its rough approximation. Raw scores remain descriptive evidence.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-race-small | 0.193 | 0 / 38 |
| partner-race-small | 1.680 | 4 / 32 |

Stopped: fallback-rate-exceeds-five-percent:partner-race-small

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
