# l2-partner-voids versus l2-partner-default

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 100/100 mirrored deals; 100 in complete analysis groups (100 independent deal units).

A pair wins/losses/ties: **12/17/71**. Contract wins: **95 A / 105 B**.

A contract win fraction: **47.5%**. Equal strength is 50%. Points never break ties.

Descriptive mean ±2 SE: 42.1% to 52.9%, clustered by deal. This is a rough interval, not a formal equivalence or optional-stopping claim.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l2-partner-default | 1.126 | 19 / 1838 |
| l2-partner-voids | 1.318 | 60 / 1809 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
