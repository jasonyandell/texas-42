# l1-race versus l1-fixed

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 50/50 mirrored deals; 50 in complete analysis groups (50 independent deal units).

A pair wins/losses/ties: **4/5/41**. Contract wins: **49 A / 51 B**.

A contract win fraction: **49.0%**. Equal strength is 50%. Points never break ties.

Descriptive mean ±2 SE: 42.9% to 55.1%, clustered by deal. This is a rough interval, not a formal equivalence or optional-stopping claim.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-fixed | 0.137 | 0 / 929 |
| l1-race | 0.498 | 0 / 923 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
