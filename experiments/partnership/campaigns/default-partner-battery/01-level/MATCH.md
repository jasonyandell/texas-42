# l2-partner-default versus l1-default

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 100/100 mirrored deals; 100 in complete analysis groups (100 independent deal units).

A pair wins/losses/ties: **14/14/72**. Contract wins: **100 A / 100 B**.

A contract win fraction: **50.0%**. Equal strength is 50%. Points never break ties.

Descriptive mean ±2 SE: 44.7% to 55.3%, clustered by deal. This is a rough interval, not a formal equivalence or optional-stopping claim.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-default | 0.228 | 0 / 1821 |
| l2-partner-default | 1.128 | 24 / 1852 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
