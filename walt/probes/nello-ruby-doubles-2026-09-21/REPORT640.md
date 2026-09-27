# Ruby's doubles: 640-world follow-up

EXPLORATORY. Eight seeds (1–8), two positions, 640 sampled hidden deals per
comparison. Same checksum-verified deployed WASM, same public/own-hand inputs,
same eight-world model of other players. No game code or deployment changed.

**Finding:** the first position's apparent double preference weakens substantially.
At the second position, no 640-world run selects the played 4–4. These comparisons
do not establish a unique optimal move or a rules defect.

## Sample-count breakpoints

40, 160 and 640 are engineering budgets, not special sizes of the game's hidden
deal space. Fixed selection accepts integer sample counts; the deployed adapter
imposes the upper limit of 640. The sampler continues until it has the requested
number of accepted, mechanically consistent hidden deals. All candidates use the
same sampled deals. For a fixed estimator under ordinary independent sampling,
quadrupling N halves the standard-error scale, so a 4x ladder is convenient.
That fact is not a confidence guarantee for this optimized search.

For each seed and position the accepted outer sample stream is shared across
budgets: 40 is a prefix of 160, which is a prefix of 640. Search is recalculated
at each budget. The 640 requests' 40-world checkpoint action vectors and choices
were asserted identical to the earlier run, and all 16 comparisons completed at
640 without fallback. The entire panel took 5.70 seconds on this Mac; this is not
a phone latency measurement.

## First Ruby lead: she played 6–6

Mean estimated eventual set probability across the eight seeds:

| Lead | 40 worlds | 160 worlds | 640 worlds | Chosen at 640 |
|---|---:|---:|---:|---:|
| 2–1 | 75.0% | 73.5% | 73.1% | 0/8 |
| 3–1 | 84.1% | 80.2% | 79.1% | 2/8 |
| 4–4 | 85.6% | 83.1% | 80.6% | 3/8 |
| 5–1 | 80.9% | 79.5% | 79.9% | 1/8 |
| 6–6 | 88.1% | 84.6% | 80.7% | 2/8 |

The 6–6 advantage over 4–4 falls from 2.5 percentage points at 40 to
1.48 points at 160 and 0.059 points at 640. Four choices are within 1.61 points
on the 640-world averages. All eight 160-world runs chose a double; three of the
eight 640-world runs choose a mixed tile. The smaller panel did not establish a
robust double preference.

## Second Ruby lead: she played 4–4

| Lead | 40 worlds | 160 worlds | 640 worlds | Chosen at 640 |
|---|---:|---:|---:|---:|
| 2–1 | 66.3% | 64.1% | 65.4% | 0/8 |
| 3–1 | 68.4% | 71.4% | 70.1% | 2/8 |
| 4–4 | 74.7% | 71.4% | 69.6% | 0/8 |
| 5–1 | 68.1% | 70.2% | 71.0% | 6/8 |

5–1 leads the mean by about 0.90 points over 3–1 and 1.33 over 4–4. Selection
still varies with the seed. No formal confidence interval or optimality claim
is attached to this small panel.

## Why scores can move as well as become less noisy

Walt optimizes Ruby's future choices inside the sampled information-set tree.
This is not a fixed continuation replayed on additional deals. Enlarging the
sample can change the best continuation, especially in branches represented by
few sampled deals. Small-sample optimization can produce optimistic estimates;
the falling double values are consistent with that mechanism, but this panel
does not isolate or prove its contribution. Ordinary sample variation also
remains. No frozen-plan holdout evaluation or active-plan stability audit was
performed.

Also, only the outer sample budget increased. The modeled players still use
eight inner worlds. Increasing the outer sample count cannot by itself validate
the response model or its inner sampling budget. The original decision seed and
receipt remain absent from the shared hand link.

The earlier revealed-deal result is unchanged: every Ruby lead at these positions
allows a forced set with full information, while either 2–1 or 3–1 immediately
sets the actual declarer hand. That is a different question from the best lead
under Ruby's uncertainty.

## Evidence

- panel640.mjs: bounded reanalysis using the unchanged preview WASM.
- panel640.json: full requests, checkpoints, exact rational action vectors.
- summary640.json: exact mean/min/max/per-seed values at 40, 160 and 640.
- run640/run.json: completed watchdog receipt, no outstanding process.
- REPORT.md: original hand reconstruction, counterfactual rules checks,
  full-information diagnostic and artifact/source hashes.
- Local source: walt/walt-player/src/lib.rs (640 adapter cap and fixed n0=8),
  walt/walt/src/solver/partnership.rs (common sampling and search),
  walt/walt/src/solver/mod.rs (viewer continuation optimization),
  walt/walt/src/solver/selection.rs (fixed sample selection).
- Mathematical scope: walt/math/signed_pivotal_geometry_v0.1.md distinguishes
  fixed-plan linear estimates from optimized action envelopes; this report does
  not promote its exploratory observations to a higher evidentiary tier.
