# l1-refine versus l1-race

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 50/50 mirrored deals; 50 in complete analysis groups (50 independent deal units).

A pair wins/losses/ties: **4/2/44**. Contract wins: **52 A / 48 B**.

A contract win fraction: **52.0%**. Equal strength is 50%. Points never break ties.

Descriptive mean ±2 SE: 47.1% to 56.9%, clustered by deal. This is a rough interval, not a formal equivalence or optional-stopping claim.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-race | 0.492 | 0 / 932 |
| l1-refine | 0.313 | 0 / 926 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
