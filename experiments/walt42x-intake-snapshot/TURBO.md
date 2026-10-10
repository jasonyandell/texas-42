# Compiled Walt ladder: L2 throughput and L3/L4 feasibility

Newer results: [reusable allocation patterns, exact bounds, and 5,159 L2
games/minute](AMORTIZE.md). This page records the preceding v1 implementation.

2026-09-30. Exploratory implementation and timing evidence, not a production
player or a playing-strength result. Jason authorized floating point, new
sampling couplings, and implementation changes subject to no peeking and no
strategy fusion.

The winning implementation is `turbo.cpp`: the **entire nested policy call**
runs in compiled C++, including support sampling, child construction, modeled
lower-level decisions, and value aggregation. Python deals the physical game,
passes each actor only its hand and public state, records decisions, and runs
independent games in worker processes. NumPy remains the reference engine.

## Measurements

Apple M5 Max, 18 CPU cores, 48 GiB RAM; Apple Clang 21.0.0, `-O3`, C++17,
**no fastmath**. Python 3.12.13, NumPy 2.4.4, Numba 0.65.1. Base repository HEAD
`9d6a5a2e12958183d49d0cd95c8617c4ef7a12e4`; this experiment remains local.

All following eight-game panels use starting-deal seeds 1–8, field seed 42,
delta=1, four workers, and all four seats running the named rung. Sample
schedules in this table are **outermost first**; CLI schedules are bottom first.
Games end when the bid is made or set. All completed without fallback.

| Engine/rung | Deals, outermost first | Median game | Median opening | Games/minute |
|---|---|---:|---:|---:|
| Repaired NumPy L2, previous panel | 30 → 8 | 9.614 s | 6.341 s | 22.8 |
| C++ L1 | 160 | 0.0303 s | 0.0180 s | 2,308 |
| C++ L2 | 30 → 8 | 1.104 s | 0.741 s | 195 |
| C++ L2, shared support samples | 30 → 8 | 1.190 s | 0.728 s | 178 |
| C++ L3 | 8 → 8 → 4 | 16.505 s | 10.550 s | 13.1 |
| C++ L4 | 2 → 2 → 2 → 2 | 10.057 s | 6.698 s | 18.2 |

L2's median game time is 8.7x lower and four-worker throughput is 8.6x higher
than the previous repaired NumPy panel at the same sample budget. C++ uses a
different reproducible random stream and traversal, so this compares equal
budgets and starting deals, **not identical policies or trajectories**. No
strength equivalence is claimed. L4 uses a much smaller budget than L3; its
completion demonstrates execution feasibility, not a quality comparison.

A separate L2 panel used 72 starting deals, seeds 101–172, with 30 outer / 8
inner samples. Each worker-count run played all 72 games completely:

| Workers | Batch time | Games/minute | Weighted deal-state visits/second |
|---:|---:|---:|---:|
| 4 | 21.443 s | 201 | 276 million |
| 12 | 8.580 s | 503 | 689 million |
| 18 | 6.503 s | **664** | **909 million** |

Actions, complete root value vectors, and game traces are identical across
these worker counts. Eighteen workers were fastest among the counts tested.
The visit count includes terminal and repeated states, across all rungs; it is
an implementation-work measure, not unique positions or FLOPS.

Throughput includes pool startup, game computation, result transfer, and JSON
writing. Compilation and the parent interpreter's imports are excluded.
Per-game timings start inside each worker. These are bounded panels, not an
extended thermal/sustained-run study or phone-device timings.

## What removed the cost

- C++ owns the complete recursive policy, eliminating hundreds of thousands
  of Python calls, JSON seed constructions, and NumPy-generator setup calls.
- A depth-first public-history tree groups fibers directly into 28 tile slots.
  It needs no general sort, global frontier, or retained backward-sweep layers.
- Each rung reuses scratch buffers by physical depth. Lower rungs have separate
  buffers, so a modeled decision cannot overwrite its enclosing query.
- Deal sampling uses a two-dimensional capacity DP: the third capacity follows
  from the number of tiles left. Exact integer support ranks avoid rejection.
- When an L1 public branch has exactly one sampled world, a compact solver
  stores just the future-relevant trick mechanics. Incremental winner/point
  updates avoid rescanning completed tricks. It consumes the same random draws
  and computes the same value as the generic C++ search. The recorded seed-1
  full L2 game retained every root value, action, and search count through both
  singleton optimizations.
- Independent games use separate native contexts and processes. One native
  context is single-caller and must not be used concurrently by multiple threads.

The optional `--reuse-deals` mode shares deterministic sample prefixes keyed by
public support and the acting hand. It never reuses an enclosing player's
conditioned deal pool. Sampling remains uniform with replacement for each
query; correlation across levels/queries changes. Its sample cache had some
hits, but total throughput was worse on this panel. It remains opt-in.

Every L still defines a best response to the preceding fixed field. This work
does not turn the ladder into one cheap pass over a common omniscient deal
table: a lower seat must condition on its own information. Policy caches retain
complete actor-attributed history, even when the support sampler can reuse a
coarser lawful support key.

## Information boundaries and numerical scope

The production native entry point is `walt_action(context, level, public,
own_hand, ...)`. It receives no physical deal and no enclosing sample pool.
Each modeled actor is called with exactly its remaining hand in that hypothetical
world and the full public history; it samples its own hidden worlds. The real
deal remains in the Python referee. The separate `walt_evaluate_l1` entry point
is a fixed-fiber test instrument, never used by the game/policy path.

At a focal decision, all fibers on a public-history node share a single MAX or
MIN over their **aggregate** child values. Other actors' public moves are
summed. No information state is merged merely because its mechanical state or
sample support matches another history. A singleton optimization applies only
after a public branch has one surviving sampled fiber; no multi-world node is
optimized separately per world.

The policy version is `walt42x-native-dfs-v1`, with sample schedule, delta, seed,
trump and `reuse_deals` also determining its identity. Sampling and random
continuations use independent domain-separated SplitMix64 streams, including
the full information key for the policy stream. This is a deterministic
simulation PRNG, not a cryptographic generator. Bounded draws use rejection to
avoid modulo bias. Changing only an enclosing sample budget cannot change a
lower rung's field. Cache presence, capacity, query order, and process count do
not enter policy randomness.

Float64 weights and values remain approximate where fractional arithmetic is
used. Delta=0 computes the exact expectimax tree over the supplied samples,
subject to float64 rounding; it is not the exact belief over all possible deals.
The sampler uses uniform public-void support, not policy-likelihood posterior
weighting. The seven pip-trump declarations, bid 30, and original contract
selection remain the experiment's rule scope.

The native `--fiber-cap` limits a **single public node's** fibers; the NumPy
guard limits its full frontier. These guards are different resource measures.
Caches have 50,000-entry caps. Deadlines are checked on policy entry/exit and
every 1,024 search nodes. A limit aborts the current decision; it never drops a
rung or returns a partially computed action. Guard failure is explicit in a
game receipt. Retained scratch buffers and caches are not an absolute RAM bound.

## Validation and rejected routes

- 192 independent scalar comparisons covering L1–L4, delta 0 / 1/8 / 1, both
  deal-reuse settings; selected exact L1 values also match a Fraction oracle.
- 249 native public transitions match an independent scalar referee, including
  trick scoring, public void deductions, and packed actor-attributed histories.
- 985 small-support ranks match the reference enumerator; separately seeded
  native samples match the Python implementation of the declared PRNG/schema.
- Reversed and repeated query order, cache/no-cache, immutable public spec,
  lower-rung budget independence, shared sample prefixes, and resource guards.
- A distinguishing strategy-fusion regression: seed 7 after 16 plays, actor 1,
  eight sampled worlds. Tile 12 has lawful value **313/54**, while optimizing
  continuations per hidden world gives **35/6**. Native returns the lawful value.
- 260 complete game receipts independently refereed; 148 repeated-policy
  comparisons preserve complete traces and root values, including 72 games at
  all three worker counts and a full L2 game with caching disabled.
- AddressSanitizer and UndefinedBehaviorSanitizer completed L2 games in both
  sample modes without findings.

The intermediate Numba frontier rewrite preserved the v2 policy and matched a
full L2 game's actions/values, but measured about 9.5 s on seed 1. Compiling tiny
queries separately was worse. These prototypes are retained as `--engine
native`, with their timing receipts and limited parity tests; they are not the
recommended throughput route.

PyTorch/Metal was considered using this experiment's earlier measured results
in `README.md`: large fractional frontiers benefited, but small/nested workloads
paid substantial dispatch/synchronization overhead. No new fused GPU L2 kernel
was built here. The current measured win is the fully compiled CPU path, not a
claim that C++ beats every possible GPU implementation.

## Reproduce

```sh
cd experiments/walt42x-intake
make
make verify PYTHON=/Users/jason/.local/share/mise/installs/python/3.12/bin/python3
make sanitize

# L2: 30 outer deals; modeled L1 uses 8. Delta defaults to 1.
python3 l2_bench.py --engine cpp --samples 8,30 --level 2 \
  --seed 101 --games 72 --workers 18 --output results/my-turbo-l2.json

# Explicit deeper budgets, bottom rung first.
python3 l2_bench.py --engine cpp --samples 4,8,8 --level 3 \
  --games 8 --workers 4 --output results/my-turbo-l3.json
python3 l2_bench.py --engine cpp --samples 2,2,2,2 --level 4 \
  --games 8 --workers 4 --output results/my-turbo-l4.json

python3 verify_l2_runs.py results/my-turbo-l2.json \
  --output results/my-turbo-l2-verification.json
```

Use the measured Python above if the shell's `python3` lacks NumPy/Numba.
`make` requires a C++17 compiler. The `.dylib` artifact is local and ignored by
Git. Rebuild after C++ edits. `--engine numpy` retains the repaired v2 reference;
the CLI default remains that reference to avoid silently changing policy.

Raw timings, verifier output, and profiles live under `results/speed-*` and
`results/turbo-*`. `results/turbo-speed-summary.json` collects computed metrics.
`TURBO-MANIFEST.json` records source, native-binary, and evidence hashes; earlier
manifests describe their earlier source revisions.
