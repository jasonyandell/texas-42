# Default L1 / L2 Partner and L2 void comparisons

2026-09-06, declared before execution. EXPLORATORY. Jason requested the
default-L1/L2 comparison and the void comparison at L2. In the established
session scope, L2 means **L2 Partner**: partner modeled at L1, opponents at L0.
L2 All (all three other seats modeled at L1) is a separate family and is not
part of this batch.

## Frozen comparisons

| Match | A | B | Single changed coordinate |
|---|---|---|---|
| 01-level | l2-partner-default | l1-default | Partner's modeled level |
| 02-voids | l2-partner-voids | l2-partner-default | Inner belief strategy |

All three players use the same unchanged native binary, **fixed** root and
modeled selection, 40 outer worlds, n0=8, n1=2 where active, and the existing
14-second wrapper with its explicit reserve fallback. `l1-default` is the
existing `l1-fixed` configuration under a clearer catalog name. No racing,
tie refinement, or phone worker is part of this battery.

The voidless and counted-void inner samplers also use different deterministic
draw streams. This compares the two implemented belief strategies, not a
coupled-world ablation that isolates only the logical void constraints.

## Panel, score, and interpretation

Both matches use fresh random deal seeds **750600–750699**, 100 complete deals.
Each match uses identical physical deals, bidder, trump, and bid 30. Each deal
is played twice, with policy partnerships swapped while physical hands stay
fixed. This is **400 planned games on 100 shared deal units**, not 200 independent
deal units. There is no auction and no fixed-hand completion panel in this batch.
Decision randomness remains public seed 420600, independent of the deal seed.

Pair score is make(A)-make(B). Report A wins/losses/ties, comparative contract
win fraction `(1 + mean pair score) / 2`, per-player latency and fallback
counts. Both-set and both-make pairs tie, regardless of points. Completed
deadline fallbacks remain part of the measured player. Points over bid are
diagnostics only. These are comparative scores against a named opponent, not
calibrated pmake or opponent-independent ratings. No transitive ranking of a
third unmeasured matchup is implied.

## Execution and stopping

One shared pool runs up to ten games, six native threads per game, using
persistent workers and per-move durable checkpoints. Each foreground slice
runs at most 270 seconds under the packet's 295-second process-group watchdog.
Resume the same manifests and saved moves until completion or technical stop.
Interrupted moves may be retried; valid completed outcomes are never rerolled.

Retain the existing technical gate: pause a matchup on repeated worker failure
or more than 5% fallbacks for either player after at least 20 nonforced moves.
No outcome-based early stopping or configuration tuning within the panel.
If a gate stops a matchup, report its incomplete evidence and the cause; do
not silently change the gate or replace the configuration. Check latency and
progress after each slice, and independently replay every published game.

Player parameters and source/binary identities are pinned in each manifest
before execution. The separate naming/catalog changes do not alter the native
decision engine, fallback implementation, scheduler, or existing profiles.
