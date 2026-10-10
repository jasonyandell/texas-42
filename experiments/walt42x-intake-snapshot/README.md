# Walt42x GPU intake — 2026-09-29

Latest play study: [L2 160/24 versus phone Walt, paired dropped-30 games](H2H.md).
The independent 512-pair confirmation finished 84 wins / 46 losses / 382 ties,
with 1.11-second median games and no timeouts.

Current throughput work: [amortization and exact bounds, L2–L4 timings, and
5,159 L2 games/minute](AMORTIZE.md). The preceding [compiled ladder](TURBO.md)
and GPU intake below retain their historical measurements.

Exploratory execution and audit of Jason's pasted frontier solver. This is a
separate experiment, not a replacement for the repository's Walt player.

## What ran

Apple M5 Max, 40 GPU cores, 48 GiB unified memory, macOS 26.5.1; Python 3.12,
NumPy 2.4.4, PyTorch 2.11.0. The installed Metal/MPS runtime executed the
frontier gathers, legal-move masks, compaction, grouping, transitions and
backward scatter reductions. CPU operator fallback was disabled. Host Python
controls the loop; NumPy supplies RNG, sampling and initial rules tables.

`walt42x.py` preserves the supplied executable algorithm and defaults. Its
header/comments were condensed; it is not a byte-for-byte capture of the paste.
`metal.py` is a direct PyTorch translation of its random-policy search, using
int64 masks/indices and float32 weights. The supplied NumPy reference uses
float64 weights. Higher-level modeled Walt policies were **not** GPU-ported.
The new code uses only the pasted seven pip-trump contracts, bid 30, seats 1/3
declaring. It does not yet cover the repository's other declarations/contracts.

CuPy's supported GPU routes are CUDA/ROCm, so this Apple GPU run uses
[PyTorch MPS](https://docs.pytorch.org/docs/stable/notes/mps.html), not a CuPy
import substitution. See [CuPy installation](https://docs.cupy.dev/en/stable/install.html).

## Timing protocol

Each root benchmark uses the same frozen sampled deals and search RNG seed on
all backends. One cold call precedes three timed repetitions; table entries
are their warm medians. GPU synchronization brackets each call. Complete
forward expansion and backward solution, input-state/deal upload and result
readback are included. Sampling, imports, rule-table initialization and console
output are excluded. Backend initialization and cold-call timings are recorded
separately. Torch CPU uses one thread. No Rust CPU comparison was run.

Corrected grouping implementation:

| Search | NumPy CPU f64 | Torch CPU f32 | Metal GPU f32 | Peak fibers |
|---|---:|---:|---:|---:|
| Opening, 30 deals, seed 1, delta=1 | 63.38 ms | 49.37 ms | 282.67 ms | 25,340 |
| Opening, 30 deals, seed 1, delta=1/8 | 476.36 ms | 357.21 ms | 310.14 ms | 200,531 |
| Opening, 120 deals, seed 1, delta=1/8 | 2,057.00 ms | 1,495.89 ms | 428.50 ms | 758,746 |
| After 16 plays, 30 deals, seed 2, delta=0 | 5.27 ms | 4.73 ms | 84.66 ms | 5,450 |

The 30-deal delta=1/8 GPU search is 1.54x faster than NumPy and 1.15x faster
than the same Torch translation on CPU. Delta=1 is about 4.5x slower than NumPy.
These are tiny descriptive benchmarks, not a hardware-general speedup claim.
Warm GPU costs changing little between the two opening sizes are consistent
with significant dispatch/synchronization overhead; no per-kernel attribution
has been measured. Cold GPU opening calls took 1.65 / 1.77 / 2.03 seconds
for the three opening workloads in table order.

The opening delta=0 run was **not solved**: after ply 8 it held 440,244 fibers,
and expanding ply 9 requested 2,094,460. Both backends stopped at the 500,000
fiber guard. This guard is checked after identifying next edges, before their
full state allocation; it is not an absolute process-memory ceiling. Search
timeouts likewise apply between plies, not within an individual operation.
An aborted search returns no action values.

The full seed-1, delta=1, 30-deal, level-0 GPU hand completed all 28 plays and
made 30–12. With corrected grouping, a fresh-process cold/warm pair took
41.29 s / 3.753 s; the exact repeated trajectory warms previously encountered
shapes and does not establish warm latency on fresh hands. NumPy played the
same complete trace in 166.61 ms. Whole-hand timing includes sampling, rules,
all decisions, transitions and printing, but excludes module imports/backend
construction. The cold/warm gap shows why isolated warm roots cannot be
advertised as game latency. The whole-hand wrapper still uses the supplied
CPU rejection sampler and public-state main loop.

## Backend defect discovered and repaired

The 120-deal stress run exposed incorrect history grouping at ply 22. The
installed Torch 2.11.0/MPS `sort` and `unique` round integer values above
2^24; one-dimensional `nonzero` also rounds coordinates above that boundary.
This was reproduced independently with CPU-created int32/int64 values uploaded
to MPS: 100 distinct integers starting at 20,000,000 collapse to 51 unique
values. Multiplication and upload preserve the integers. This is an observation
about this installed runtime, not a claim about every PyTorch/MPS version.

The original packed key `parent*28+tile` crosses that threshold on the larger
frontier. Two distinct public histories can then be wrongly merged, breaking
the solve. The original run exceeded 28 plies and was capped, so **none of that
120-deal GPU run is a valid timing or value result**. Its failure census remains
in `results/opening-d0125-n120.json`.

`metal.group_edges` now uses an exact parent-by-tile occupancy table, prefix
scan and two-dimensional nonzero coordinates, packing integer keys only after
compaction. This avoids the faulty sort/unique route and keeps nonzero
coordinates small. A directed regression crosses 20,000,000 with adjacent and
duplicate keys and verifies both unique keys and their inverse reconstruction.
The prefix-count guard requires fewer than 2^24 edges; practical benchmark
guards are much smaller. The dense occupancy table costs 28 slots per parent,
so a compact native integer sort/scan is still preferable for a mature engine.

Files prefixed `fixed-` contain the corrected grouping runs; unprefixed timing
files preserve the initial implementation. The corrected 120-deal search
completed in 428.50 ms warm GPU median versus 2,057.00 ms NumPy and 1,495.89 ms
Torch CPU, with 758,746 peak fibers. That is 4.80x / 3.49x respectively. All
frontier census rows match and the largest GPU value error is 9.09e-6 weighted
makes. This is the measured scaling result; the broken run is not included.

## Correctness evidence

- Every measured 30-deal root has the same node/fiber census on NumPy, Torch
  CPU and Metal. Delta=1 vectors match exactly. The largest Metal-vs-NumPy
  root error is 2.73e-6 weighted makes for delta=1/8 and 1.70e-6 for the exact
  late root. The selected root agrees on these inputs.
- `verify.py` independently implements scalar suit following, trick scoring
  and recursive shared-action expectimax with `fractions.Fraction`. Ten small
  reachable endgames cover all seven pip trumps, all four partial-trick offsets
  and both maximizing/minimizing focal teams. NumPy agrees to 1e-12; Metal to
  1e-5. This is bounded executable evidence, not a proof of general parity.
- `verify_game.py` checks the 28-move GPU trace against the supplied NumPy
  player and independently referees legality and the final 30–12 score.
- `results/` retains root values, plies, timings, fixtures, verification and
  full-hand transcripts. Float32 reduction and threshold behavior can matter
  on close ties or other seeds; no exact arithmetic GPU claim is made.

## Semantic audit

**The central idea is sound for the random-policy rung.** Parent-node identity
plus the observed tile retains a public-history tree. Deals at that history
carry weights. Summing their contribution before taking the focal MAX/MIN
keeps a shared action, avoiding per-deal strategy fusion. For a root sample
with a common own hand, all deals at a focal public node share its legal moves.
Delta=0 is exact expectimax over that finite sample and the specified uniform
legal field, subject to floating-point error. It is not exact over all deals
or all possible opponent behavior.

**Delta needs a narrower claim.** For 0 < delta <= 1, a successful split leaves
each positive child weight at least delta; otherwise sampling preserves the
parent's weight. Under a fixed focal policy, mass conservation therefore
bounds a world's simultaneous opponent-history branches by floor(1/delta).
The search also duplicates mass across focal alternatives and retains earlier
plies. This does not prove that delta=1/8 takes at most eight times the wall
time, memory, or realized work of a particular delta=1 run. For this seed, total
fiber visits were 254,178 vs 1,928,261 (7.59x). Sampled branch weighting is
unbiased for fixed continuations; optimizing noisy continuation values via
MAX/MIN does not preserve that guarantee. Delta=1 is in the sampled-continuation
family, but bit/trace equivalence to a separate `walt42` implementation was
not established.

**Nested policies need repair before promotion.**

1. `search` calls `assume` without voids. `Pub` stores played tiles/current trick
   but no complete attributed history or accumulated void constraints. Nested
   `walt` therefore samples with its default zero voids, losing both the root
   constraints and newly observed failures to follow suit.
2. Repeated identical modeled information states draw fresh samples from a
   shared mutable RNG. Reproduction: seed-2 state after 16 plays, eight inner
   deals, delta=0, twelve identical queries, RNG seed 42 produces tile IDs
   `[20, 7, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20]`. This is not a frozen field
   function and depends on evaluation order. One sampled one-hot response
   also does not integrate the nested policy's sampling distribution, even
   at delta=0. The existing [`solver/field.rs`](../../walt/walt/src/solver/field.rs)
   supplies the relevant information-key/immutable-field pattern.
3. The higher rung iterates inner searches serially per fiber, including
   repeated information states. It needs deduplication and batching; replacing
   the array library alone does not make the ladder GPU-efficient.

Other observed limitations: terminal-root `search` raises `IndexError` at
`plies[1]` (reproduced); rejection sampling has no attempt limit; arbitrary
`assume` output is not validated; counts are returned without an extracted
continuation policy. The source was left intact so these remain visible.

## Making this ours

Keep the public-history frontier and weighted fibers. Reuse the repository's
contract/rules and pure modeled-field identity, retain full actor-attributed
history/void updates, and deduplicate modeled information states before doing
any nested work. Start with delta=0 endgame parity and deterministic sampled
fixtures as separate targets. Declare numeric/tie and cutoff semantics.

For a Rust implementation, use the existing wgpu/Metal experience in
[`response-ladder/src/gpu.rs`](../response-ladder/src/gpu.rs). Rust should own
the contract, field keys and buffer lifecycle; fused GPU kernels should own
legal expansion/transition, compaction, segmented accumulation and shared-action
backup. Keep reusable buffers and minimize host-visible shape queries. Small
frontiers need a measured CPU crossover. The existing response-ladder GPU
primitive is fixed-policy rollout, so it cannot simply replace this frontier
solver. No Rust rewrite, production change or strength claim is part of this intake.

## Reproduce

Use Python with `numpy` and `torch` installed and a working Apple MPS device.
The measured interpreter here was
`/Users/jason/.local/share/mise/installs/python/3.12/bin/python3`.

```sh
cd experiments/walt42x-intake
python3 bench.py --played 0 --delta 1 --output results/fixed-opening-d1.json
python3 bench.py --played 0 --delta 0.125 --output results/fixed-opening-d0125.json
python3 bench.py --played 0 --deals 120 --delta 0.125 --cap 1500000 --output results/fixed-opening-d0125-n120.json
python3 bench.py --seed 2 --played 16 --delta 0 --output results/fixed-late-exact.json
python3 bench.py --played 0 --delta 0 --backends numpy,mps --repeats 1 --output results/fixed-opening-exact-capped.json
python3 verify.py
python3 game.py > results/fixed-game-mps-cold-warm.log
python3 verify_game.py
```
