# Complete-game throughput — NumPy retained

2026-09-29, exploratory. Jason clarified that floats are acceptable and that
completed-game throughput, rather than language/backend choice, is the objective.
The NumPy frontier remains the implementation. `fast_numpy.py` improves the
array operations; `compiled_edges.py` optionally compiles only the sparse
legal-move expansion with Numba. Neither is a Rust rewrite or a GPU requirement.

## Measured workload

These are the pasted **random-policy rung (level 0)**, 30 sampled deals,
bid 30, seven pip-trump declarations, with the pasted contract-selection rule.
A completed game stops when the bid is made or set, as in the source; it does
not always play the physically irrelevant suffix through tile 28. The nested
Walt rung is supported through the generic path and has a bounded compatibility
test, but its known void/field-identity defects remain, and no complete nested
game throughput result is claimed here.

Machine: Apple M5 Max, 18 CPU cores. Runtime: Python 3.12, NumPy 2.4.4,
Numba 0.65.1. All values remain float64; compiled loops do not use fastmath.
Game RNG streams, sample counts, move order and cutoff arithmetic are preserved.

## Results

Single-worker 24-game seed-1..24 pilot, delta=1:

| Backend | Games/second |
|---|---:|
| Supplied NumPy | 6.19 |
| Improved NumPy | 7.90 |
| NumPy + compiled sparse expansion | 19.80 |

The separate 96-game seed-101..196 panel measured 6.34/s supplied versus
20.30/s compiled on one worker. Every move and root value matched.

384-game seed-1001..1384 panel, delta=1; median of three repeated batches:

| Backend | Workers | Games/second | Batch seconds |
|---|---:|---:|---:|
| Supplied NumPy | 18 | 71.91 | 5.340 |
| Improved NumPy | 18 | 88.95 | 4.317 |
| NumPy + compiled sparse expansion | 8 | 130.13 | 2.951 |
| NumPy + compiled sparse expansion | 12 | 180.15 | 2.132 |
| NumPy + compiled sparse expansion | 18 | **253.95** | **1.512** |

That is 3.53x over the supplied implementation at the same worker count.
The three 18-worker compiled batches took 1.525, 1.512 and 1.510 seconds.
18 workers were fastest among the measured counts; this is not a universal
hardware optimum or a comparison with an optimized Rust engine.

For delta=1/8, the eight-game seed-1..8 single-worker panel measured 1.017/s
supplied versus 3.892/s compiled (3.83x), again with identical traces and values.
The separate seed-2001..2096 fractional panel, 96 games and 18 workers, measured
**9.31/s supplied versus 40.42/s compiled (4.34x)**, median of three batches.
The compiled batches took 2.375, 2.382 and 2.372 seconds; the reference batches
took 10.807, 10.101 and 10.307 seconds. All 2,256 decision vectors and complete
game traces matched with zero observed difference. Setup/warmup took 0.864 s
compiled and 1.281 s reference and is excluded from those sustained rates.
Delta=1 and delta=1/8 are different search workloads; rates across those settings
are not presented as equivalent work or evidence of playing strength.

Timing includes dealing, rejection sampling, every search/decision through
contract settlement, captured main-loop printing, trace/value collection and,
for multiple workers, task scheduling and result transfer. JSON file writing,
interpreter startup, imports, worker setup and a warmup batch are excluded.
Worker setup plus warmup is recorded separately (0.44 seconds for the compiled
18-worker 384-game panel). This measures sustained throughput with a warm pool.
It does not promise the same rate for one interactive game or a cold process.

## Changes

- Compute legal moves once per frontier fiber for the random-policy path.
- Score tricks only at complete-trick plies, avoiding unnecessary partial-trick
  argmax/score work. All nodes at one breadth-first ply have the same trick length.
- Use weighted bins and contiguous parent reductions for the backward pass.
- Optionally compile bitmask scanning and sparse edge emission, avoiding the
  large temporary 28-column probability matrix, CDF matrix and `nonzero` scan.
  The loop preserves ascending tile order, CPU RNG draws and CDF arithmetic.
- Run independent games in spawned CPU worker processes with per-seed RNGs.

There are no approximate cutoffs, smaller deal counts, fixed-policy rollouts,
hidden per-deal choices, or skipped forced moves introduced by the optimization.
The original sample distribution and delta mechanism are retained.

## Validation and caught defects

The 384-game panel matches all **9,116 decision vectors and complete traces**
against the reference, for both NumPy and compiled backends and for every worker
count tested. The separate 96-game panel matches another 2,328 vectors. The
fractional eight-game panel matches 168 vectors with zero observed difference.

`verify_throughput.py` checks 56 backend/root/delta combinations, including
delta values 0, 1/8, 1/2 and 1, both teams and partial tricks, and checks the post-search RNG
state as well as values. Small delta=0 roots are also compared with the independent
Fraction oracle. Three bounded nested-query runs (one per backend) agree on
values and RNG state; they preserve the reference's nested semantics, not fix them.

Two defects were caught during optimization and repaired before adopting the
fractional measurements:

1. `np.bincount` returns integer bins for empty weighted inputs. Copying those
   bins for a nonterminal parent truncated later fractional backups. Leaves now
   explicitly remain float64. The invalid result is retained under the filename
   `invalid-integer-leaf-throughput-compiled-d0125.json` and is excluded above.
2. A faster reduction order perturbed fractional ties enough to change some
   trajectories. Fractional searches now use reference-order weighted additions;
   the all-integer delta=1 workload retains the faster contiguous sums. The
   superseded result is `roundoff-ties-throughput-compiled-d0125.json`.

## Run

The measured interpreter is
`/Users/jason/.local/share/mise/installs/python/3.12/bin/python3`. With NumPy and
Numba available in your selected Python:

```sh
cd experiments/walt42x-intake
python3 throughput.py --backend compiled --workers 18 --games 384 --seed 1001 --repeats 3 --output results/my-throughput.json
python3 throughput.py --backend numpy --workers 18 --games 384 --seed 1001 --repeats 3 --output results/my-numpy-throughput.json
python3 verify_throughput.py
```

Use `--delta 0.125` for the fractional branching workload. `--level 1` exposes
the original nested policy through the generic path, with the limitations above;
its full-opening cost has not been characterized by this throughput panel.
The validated performance destination for this workload is currently NumPy
plus a small compiled CPU loop and independent-game workers. The earlier GPU
benchmark remains a useful large-frontier experiment, not the required backend.
