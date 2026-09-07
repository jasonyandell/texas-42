# Default L1 / L2 Partner and L2 void results

EXPLORATORY. Both planned panels completed.

The [predeclared protocol](PROTOCOL.md) fixes search at **Fixed** for the real root and all modeled minds, with 40/8/2 samples and the existing 14-second wrapper. L2 means **L2 Partner** here. No solver or fallback code changed for this batch.

The same 100 fresh physical deals (750600–750699) are used in both matchups. Each pair swaps player partnerships at bid 30. A win/loss/tie is based only on making the contract; both-make and both-set pairs tie regardless of points. These comparisons do not measure auction skill.

| A versus B | Paired deals | A wins / losses / ties | A contract win fraction | Rough mean ±2 SE |
|---|---:|---:|---:|---:|
| [l2-partner-default vs l1-default](01-level/MATCH.md) | 100/100 | 14 / 14 / 72 | 50.0% | 44.7%–55.3% |
| [l2-partner-voids vs l2-partner-default](02-voids/MATCH.md) | 100/100 | 12 / 17 / 71 | 47.5% | 42.1%–52.9% |

The comparative fraction is `(1 + mean(make(A)-make(B))) / 2`; equality is 50%. It is not absolute pmake. Intervals are descriptive normal approximations, not formal equivalence claims. Both matchups share deal units; their evidence must not be counted as 200 independent deals or used to infer an unplayed matchup by subtraction.

## Cost of the executed players

All means include forced moves and wall time under the shared ten-game load. L2 Partner default appears in both matchups and its timing below pools those appearances; the linked reports retain opponent-specific figures.

| Player | Seconds / move | Requested search / move | Reserve preparation / move | Fallbacks / nonforced |
|---|---:|---:|---:|---:|
| l1-default | 0.228 | 0.188 | 0.040 | 0 / 1821 |
| l2-partner-default | 1.127 | 1.086 | 0.040 | 43 / 3690 |
| l2-partner-voids | 1.318 | 1.275 | 0.043 | 60 / 1809 |

Reserve preparation is the cheap fixed L1 comparison computed before requested search. When requested search fails, the wrapper can return that reserve. The time already spent on the failed requested search still counts in the executed player's latency and results. A fallback is an execution route, not a refinement setting.

Fallback reasons from the saved primary phase (the raw wire labels remain in each checkpoint):

- l2-partner-default: native-deadline-refusal 35, parent-timeout 8.
- l2-partner-voids: native-deadline-refusal 50, parent-timeout 10.

## Evidence and reproducibility

Published **400 games / 11,200 moves**. Completed foreground watchdog slices total **1098.45 seconds (18.31 minutes)**; this includes any unfinished or speculative work. A running slice, if present, is not included until its watchdog record closes.

Every match retains the full frozen configuration, source/binary identities, checkpoints, paired seed scores, timing phases, and technical stop records. `verification.json` records independent replay separately from these aggregates. [Configuration checks](configuration-check.json) confirm that exactly the modeled-seat profile changes in the first comparison and exactly the inner belief changes in the second. Both use the unchanged foundation engine and binary.

The two inner samplers also use different deterministic draw streams: the void match compares implemented belief strategies as a whole. It does not isolate logical void filtering under identical random completions. The [player guide](../../PLAYERS.md) defines family, search, belief, and fallback terminology.

## Interpretation

Neither comparison establishes a strength gain. Default L2 Partner tied default L1 on this panel, while taking about five times its per-move wall time in this load. The void-aware version scored lower against default L2 Partner and cost more, but its rough interval includes 50%; this is not an established general loss either. A tied contract score does not imply identical choices or prove equivalence.

All three configurations completed the planned panel under the existing technical gate. L1 remains the cheapest measured default here. L2 Partner and its void option remain usable experimental families whose extra cost did not produce a demonstrated win in this batch. Keep this result distinct from the phone comparison: these matches used fixed L1, not the racing native counterpart of the archived phone. The original phone-strength requirement remains open.

All 2,683 first-slice moves survived resume unchanged ([resume proof](resume-proof.json)). Independent replay records cover all 400 games / 11,200 moves. The engine, native binary, reserve fallback, and scheduler were unchanged; only the named configurations and catalog/report surfaces changed. No experiment is running or scheduled.
