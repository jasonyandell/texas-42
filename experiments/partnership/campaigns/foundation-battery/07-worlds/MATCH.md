# l1-race-voids versus l1-race

EXPLORATORY. Bid 30, identical deals and contracts, player teams swapped.

Completed 50/50 mirrored deals; 50 in complete analysis groups (5 independent focal hand units).

A pair wins/losses/ties: **5/8/37**. Contract wins: **47 A / 53 B**.

A contract win fraction: **47.0%**. Equal strength is 50%. Points never break ties.

No uncertainty interval is shown: the report requires at least ten independent units and nonzero observed variance for its rough approximation. Raw scores remain descriptive evidence.

| Player | Mean seconds / move | Fallbacks / nonforced |
|---|---:|---:|
| l1-race | 0.503 | 0 / 923 |
| l1-race-voids | 0.530 | 0 / 929 |

| Focal hand: first seed | Completions | A wins / losses / ties | Mean pair score |
|---|---:|---:|---:|
| 820600 | 10 | 1 / 2 / 7 | -0.10 |
| 820610 | 10 | 1 / 2 / 7 | -0.10 |
| 820620 | 10 | 0 / 0 / 10 | +0.00 |
| 820630 | 10 | 2 / 2 / 6 | +0.00 |
| 820640 | 10 | 1 / 2 / 7 | -0.10 |

`MATCH.json` retains each seed's two make indicators and paired score, plus complete hand-group scores. A seed score is comparative against this opponent, not an absolute hand-difficulty or probability estimate.

Timing includes the bounded wrapper and its fallback work. Completed deadline fallbacks remain in the score. Incomplete pairs/groups do not enter the primary score.
