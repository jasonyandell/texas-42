# l1-race-voids versus l1-race

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 50/50 mirrored deals; 50 in complete analysis groups (50 independent deal units).

A pair wins/losses/ties: **9/5/36**. Contract wins: **54 A / 46 B**.

A contract win fraction: **54.0%**. Equal strength is 50%. Points never break ties.

Descriptive mean ±2 SE: 46.5% to 61.5%, clustered by deal. This is a rough interval, not a formal equivalence or optional-stopping claim.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-race | 0.497 | 0 / 933 |
| l1-race-voids | 0.535 | 0 / 932 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
