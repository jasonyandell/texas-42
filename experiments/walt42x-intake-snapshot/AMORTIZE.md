# Walt throughput: reusable work and exact bounds

2026-09-30. The compiled ladder now completes L2 games in a median **0.130 s**
and reaches **5,159 L2 games/minute** in repeated 18-thread batches on the M5
Max. The main improvement is exact pruning; small allocation tables, repeated
policy-query elimination, and shared worker startup provide additional savings.

This is an exploratory seven-pip-trump, bid-30 player. No playing-strength or
phone-performance result is claimed. The preceding implementation and timings
are recorded in [TURBO.md](TURBO.md).

## Measurements

Apple M5 Max, 18 CPU cores, 48 GiB RAM. Apple Clang 21.0.0, C++17, `-O3`,
**no fastmath**; Python 3.12.13 and NumPy 2.4.4. Base repository HEAD:
`9d6a5a2e12958183d49d0cd95c8617c4ef7a12e4`.

Eight-game panels use starting-deal seeds 1–8, field seed 42, delta=1, four
process workers, and all four seats playing the named rung. Games stop when
the contract is made or set. All games completed, without fallback. Deal
budgets below are **outermost first**; CLI schedules are bottom first.

| Rung | Sampled deals | Median complete game | Median opening decision |
|---|---|---:|---:|
| L2 | 30 → 8 | **0.130 s** | 0.0368 s |
| L3 | 8 → 8 → 4 | **1.426 s** | 0.587 s |
| L4 | 2 → 2 → 2 → 2 | **0.494 s** | 0.167 s |

L4 has a much smaller sampling budget than L3. Its latency demonstrates
feasibility at that budget, not stronger or cheaper play at equal quality.
Raw panels: `results/amortize-bound-l{2,3,4}-panel.json`.

The matched L2 control uses the **same addressed-v2 policy**, with pruning and
allocation tables disabled: median **1.367 s → 0.130 s**, a **10.5×** reduction.
Every action and complete root value vector matches, throughout all eight
games. The control is `results/amortize-v2-l2-unpruned.json`. Additional
unpruned complete-game controls for seed 1 at L3 and L4 also match every
action and root value (`amortize-full-l3.json`, `amortize-full-l4.json`).

Earlier v1 medians were L2 1.104 s, L3 16.505 s, and L4 10.057 s at these
budgets. V2 changes the coupling of simulated random choices, so comparison
to v1 is a budget comparison, not an identical-policy comparison. The
10.5× control above avoids that distinction.

Three fresh-process batches each played 1,152 L2 games, seeds 1001–2152,
with 30 outer / 8 inner deals and 18 thread workers:

| Repeat | Completed | Batch time | Games/minute |
|---|---:|---:|---:|
| 1 | 1,152 | 13.029 s | 5,305 |
| 2 | 1,152 | 13.397 s | 5,159 |
| 3 | 1,152 | 13.676 s | 5,054 |

All traces and complete value vectors match across repetitions. These are
three approximately 13-second batches, not a thermal endurance study.
Receipts are `results/amortize-sustained-1152-r{1,2,3}.json`.

Batch timings include pool startup, the Python game driver, native computation,
result transfer, per-game printing, and periodic JSON snapshots. They exclude
compilation, the parent interpreter's imports, and serialization of the final
JSON snapshot. Per-game timing begins inside the worker. Games in a batch
share mathematical allocation patterns, but each game creates its own policy
context; no policy answers are carried from one physical game to another.

## What changed

**Randomness can be addressed directly.** Previously, skipping a branch changed
the RNG stream consumed by later branches. `--addressed` selects the new
`walt42x-native-addressed-v2` field. Its L0 random choices are keyed by the
query's information state, public continuation, and index in that query's
independently sampled worlds. Traversal order, pruning, cache hits, and worker
schedule can no longer move those draws around. The exact uniform support
sampler and its rank stream are unchanged for a fixed query and specification.
V1 remains the default without `--addressed`.

**Search uses exact incumbent bounds.** At delta=1, each live fiber has unit
weight, so every node's value is between zero and its fiber count. A shared
focal choice can stop at its exact best possible value. An alternative can
also stop as soon as its remaining possible value cannot beat the current
best. SUM nodes propagate bounds using accumulated value and remaining mass.
All arithmetic here is exactly representable integer counting in float64;
there is no epsilon or heuristic cutoff. These bounds are currently enabled
only for addressed-v2 at delta=1.

**Action-only queries do less work.** Nested policies need a chosen tile.
They can cache that tile even when losing alternatives were not fully valued.
A later request for the entire root value vector detects the partial entry
and recomputes it. Full-value queries retain exact values for every legal
root action. Uncomputed scores are never passed off as exact values. Ties
continue to select the smallest tile index.

**Repeated lower-policy calls are combined.** At one public node and rung,
fibers with the same acting hand ask the same lower policy question. A small
reused vector resolves that question once. Forced choices bypass the lower
policy call entirely. This supplements the existing full-information-state
policy cache; it does not merge different public histories.

**Small deal-allocation problems reuse tables.** For at most six unseen tiles
and at most two slots per other seat, an abstract allowed-seat/capacity pattern
maps directly to its possible allocations in the exact DP's rank order.
Patterns are built lazily and retained in a bounded thread-local cache, with
at most 8,192 entries. They contain no tile identities, private hands, public
histories, or policy values. Other support problems retain the integer DP.
Immutable field/rung seed salts are also precomputed once per context.

**Workers amortize startup and output.** `--executor thread` lets ctypes release
the GIL during native search. Every worker uses separate policy contexts,
while mathematical pattern tables survive between its games. JSON snapshots
are written at most once per second during execution, avoiding a complete
rewrite of all previous games after every result. Final receipts retain all
game decisions and root values.

On a separate 144-game L2 panel, 18 processes with tables achieved 4,796
games/minute, 18 threads with tables 5,305, and 18 threads without tables 5,080.
The table gain was small and was neutral in some short panels. The large
improvement comes from eliminating provably irrelevant search, not from
allocation-table lookup alone.

## Information boundaries and checks

The production native action entry point receives the acting hand and public
state, never the physical or enclosing complete deal. Nested actors sample
their own compatible worlds. Policy identity includes the complete
actor-attributed public history and the lower field's parameters. Enlarging
only an enclosing sampling budget cannot change the lower field.

MAX/MIN choices still aggregate all indistinguishable fibers before choosing
one tile; modeled observations use SUM. Bounds apply to that shared choice,
not separate optimal decisions in each hidden world. Reuse of abstract
allocation mathematics conveys no hidden information. The existing optional
`--reuse-deals` coupling remains available, but the timing panels above do not
enable it.

Verification recorded in `results/`:

- `amortize-verification.json`: 192 independent scalar-reference comparisons
  across L1–L4, delta=0, 1/8, and 1, and both sample-reuse settings, on bounded
  late-game positions. Also compares against native search with pruning,
  tables, node deduplication, and policy caching disabled; checks action-only
  then full-value requests, mixed-history order/cache invariance, and lower
  field independence.
- `amortize-v1-compatibility.json`: the original v1 checks still pass, including
  192 oracle cases, 249 public transitions and 985 enumerated support ranks.
- `amortize-pattern-exhaustive.json`: 374,272 abstract mask/capacity cases and
  471,168 valid allocation ranks agree with the integer DP.
- `amortize-all-game-verification.json`: 4,406 complete receipts replay legally;
  3,077 equal-policy comparisons match every action, score and root value.
  This includes intermediate optimization panels and repeated seeds; it is
  not a claim of 4,406 distinct starting deals or scalar-oracle solves.
- `amortize-asan-ubsan.log` and `amortize-tsan.log`: AddressSanitizer,
  UndefinedBehaviorSanitizer and ThreadSanitizer pass. The thread test runs
  12 complete games across four concurrent native workers and compares
  pruned/unpruned values and partial-cache upgrades.

The explicit strategy-fusion counterexample remains intact: seed 7 after
16 plays, trump 2, actor 1, eight worlds, tile 12 has lawful value **313/54**.
Optimizing separately per hidden world would return **35/6**. The native
shared-information search matches the independent rational oracle.

## Reproduce

From this directory, with a Python installation containing NumPy and Numba:

```sh
make
python3 l2_bench.py --engine cpp --addressed --level 2 --samples 8,30 \
  --executor thread --workers 18 --games 1152 --seed 1001 \
  --output results/my-addressed-l2.json
python3 verify_amortize.py --output results/my-addressed-checks.json
python3 verify_turbo.py --output results/my-v1-checks.json
make verify-patterns
make sanitize
make sanitize-thread
```

The measured Python binary is
`/Users/jason/.local/share/mise/installs/python/3.12/bin/python3`.
For L3 use `--level 3 --samples 4,8,8`; for L4 use
`--level 4 --samples 2,2,2,2`. Verification controls are `--no-prune`,
`--no-tables`, `--no-node-cache`, and `--no-cache`. A native context must be
used by one caller at a time; the benchmark creates one context per game.

[AMORTIZE-MANIFEST.json](AMORTIZE-MANIFEST.json) hashes the current source,
native library, test harnesses, reports, and retained evidence. Intermediate
panels record earlier stages of this optimization pass; the final performance
panels have the `amortize-bound-` and `amortize-sustained-` prefixes.
The preceding native source/library is preserved in `checkpoints/turbo-v1/`.
Earlier manifests describe their historical revisions, not the modified
current files. `results/turbo-verification.json` was refreshed during the v1
compatibility check; the explicitly named current compatibility report above
is the evidence for this revision.
