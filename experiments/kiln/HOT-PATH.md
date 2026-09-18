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
