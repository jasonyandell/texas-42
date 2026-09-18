# Removing redundant work in the current solver

These changes preserve the finite sampled evaluator and its policy. They do not
repair the calibration gap or claim stronger play.

## Forced Dice choices

When the legal set has one domino, select it directly. Dice's random stream is
reconstructed from the sample seed and public-record hash at each state, then
discarded. Skipping this local draw cannot change later draws. Lazily computing
the public-record hash also avoids it when every surviving sample is forced.

This passed 64 retained/fresh exact-fraction and work-counter comparisons. The
paired deep timing measured 1.002x median and 1.001x geometric speedup: **no
meaningful measured benefit**. The bounded production firing retained 796 prices.

## Bitset access

`DominoSet` already represents stable domino IDs as bits. `mask_of` now returns
those bits directly and `set_of` uses the existing checked constructor, rather
than iterating the tiles to reconstruct the same representation. Invalid high
bits still cause an error. No rule is reimplemented.

This passed 64 retained/fresh exact-fraction and work-counter comparisons, the
exhaustive rules suite, and frozen ordering tests. A paired 16-case 160-world
check measured 1.083x median / 1.084x geometric speedup against the preceding
worker. The one-minute production firing retained 912 prices with zero errors.

## Viewer move ordering

The recursive solver now orders candidates in fixed stack storage. Its public
ordering method still returns a vector for existing callers; both paths use one
ordering implementation. The standing winner and count on the trick are computed
once per decision, and no priorities are needed for a sole legal candidate.

The priority formula, ascending-ID tie rule, legal set and visit sequence stay
the same. In particular, this introduces no new partnership feature or heuristic.
Forty retained/fresh cases matched exact fractions, nodes, policy calls and inner
sample counts. Partnership, public-void sampler, selection and frozen ordering
tests passed. A paired 16-case deep check measured 1.100x median / 1.099x geometric
speedup against the bitset worker.
The one-minute production firing retained 1,071 prices with zero errors.

Each paired timing used four concurrent pairs with alternating old/new order and
a 60-second bound, with no competing production process. Production firings ran
different requests and sometimes overlapped builds/tests: their saved counts are
progress, not controlled speedup factors. Compact summaries pin the full external
evidence by SHA256. Immutable source/binary bundles remain in the campaign.

## Worker-layout measurements

`pool_benchmark.py` replays a fixed retained workload through persistent workers,
with independently selectable process and thread counts. Every scalar must match
the original receipt. Serial work counters must match too; concurrent cache
misses can duplicate work, so parallel counter equality is not required.

Each run is bounded to at most 60 seconds and saves completed cases. Compare
elapsed time only when all layouts finish the same requested workload. Run with
production stopped so competing jobs do not contaminate the comparison. A timed
out run is explicitly incomplete and is not treated as an equivalent full run.

All five checks completed the same 256 retained 160-world requests:

| Processes × threads | Elapsed seconds |
|---|---:|
| 18 × 1 | 26.04 |
| 9 × 2 | 33.59 |
| 12 × 1 | 37.60 |
| 24 × 1 | 28.72 |
| 18 × 1, repeat | 28.79 |

The repeated 18-worker run tied 24 workers. Production keeps 18 independent
single-threaded workers; this is a measured practical choice, not a proof of
universal optimality. All values agreed; all serial work counters agreed.
See pool-timing-summary.json for hashes of the full request/timing records.

## Compiler profile experiment: retain the existing release build

A temporary Cargo profile inherited release's checked arithmetic and enabled
ThinLTO with one codegen unit. Its build finished in 41.7 seconds under a
60-second watchdog. All 64 retained/fresh exact-value and counter checks passed.
On 32 paired deep requests it measured only 1.013x median / 1.011x geometric
speedup. This small effect was not sufficient evidence to change production;
the temporary profile was removed and the existing release worker resumed.

The candidate is preserved, including the exact temporary Cargo configuration,
under producer 20722f673026bc853a12cadffa35cd3084a95b91c5652df1b2050e07ac5dff3c.
See lto-parity-summary.json, lto-timing-summary.json, and the campaign's
lto-build/run.json. The phone build and the bidding model were unchanged.

## Sparse Dice buckets

A five-second sample of the running release worker still spent substantial time
in the small-support Dice path. That path partitioned at most eight samples, then
scanned all 28 domino buckets. It now records occupied buckets in a 28-bit mask
and visits their ascending set bits. Nonempty buckets, sample membership, visit
order and success mass are identical; empty buckets do no work.

The candidate passed 64 retained/fresh exact-fraction and node/policy/inner-world
counter comparisons, including 4/12/40/160-world cases. Partnership (12), selection
(7), ordering (4) and public-void sampler (7, plus one historical ignored fixture
generator) tests passed. Thirty-two paired 160-world requests, four concurrent
pairs, alternating baseline/candidate order and a 60-second limit, measured
**1.197x median / 1.198x geometric speedup**. Production was stopped for that
timing, then the previous immutable worker was safely resumed before adoption.

Candidate binary/source snapshot:
`8b5e78750f9a7aca70b7c0e1a124976a148aff82faf2e633173156a4f3372aa5`.
The snapshot records base commit 4bc60d64 and the exact modified source hashes.
See sparse-buckets-parity-summary.json and sparse-buckets-timing-summary.json;
the raw profile is profile-current-worker.txt in the campaign. This changes
implementation cost, not the bidding model or its calibration evidence.
