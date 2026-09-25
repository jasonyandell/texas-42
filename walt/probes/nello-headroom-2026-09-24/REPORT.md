# Nel-O preview headroom — exploratory timing

This is an engineering latency probe, not a playing-strength experiment.
`timing.mjs` runs the imported browser WASM in Node with its real clock and a
fresh instance per decision. Inputs contain only the acting player's own hand
and public history. The two early free-choice cases use the same fixture with
two seeds; the three Ruby positions share one recorded hand. This small panel
is not representative of all games or phone hardware.

The prior build uses 256 candidate deals per round and up to two remaining
seconds for counterexamples. The candidate uses 768 and reserves up to six
seconds before ordinary comparisons (at most half the caller's budget).
Both retain at most four failures in each of three rounds, at equal weight.
Normal play remains 40 worlds; explicit Think deeper rises from 160 to 500.
The 14/20-second total limits and inner sample size eight are unchanged.

The before and after calls ran sequentially with no concurrent build/test work.
All ten candidate calls completed their requested ordinary comparison and all
three counterexample rounds. This measures work completion and latency only.

| Position | Old normal 40 (ms) | New normal 40 (ms) | Old deeper 160 (ms) | New deeper 500 (ms) |
|---|---:|---:|---:|---:|
| Ruby first trick | 441 | 1002 | 819 | 2720 |
| early free defender seed 1 | 861 | 2168 | 1224 | 3480 |
| early free defender seed 2 | 905 | 2480 | 1106 | 3918 |
| Ruby double-six lead | 411 | 1033 | 640 | 1932 |
| Ruby double-four lead | 157 | 346 | 207 | 532 |

Across this panel, total normal-mode time rose from 2,775 to 7,029 ms
(about 2.5 times), and deeper-mode time from 3,996 to 12,582 ms
(about 3.1 times). The largest WASM linear memory was 22.1875 MiB; that is not
whole-browser peak RAM. These are single-pass Node measurements on this Mac,
not confidence intervals, phone measurements, or a general runtime bound.

Validation: 11 focused Rust tests passed. `conformance.json` records native/WASM
agreement at 160, completed 500-world refinement, interrupted-round retention,
and an accelerated-clock test that leaves a 40-world checkpoint and still
completes counterexample refinement when the deep comparison has no time.
`straight-parity.json` compares 12 complete ordinary decisions across builds,
including exact rational values, work counters and the full partner review.

Reproduce under the packet `run_capped.py` watchdog:

```
node walt/probes/nello-headroom-2026-09-24/timing.mjs BEFORE.wasm before.json before
node walt/probes/nello-headroom-2026-09-24/timing.mjs AFTER.wasm after.json after
node walt/walt-player/counterexample-check.mjs AFTER.wasm conformance.json
node walt/walt-player/compare-builds.mjs BEFORE.wasm AFTER.wasm straight-parity.json
```

Each timing artifact includes the full calls, replies and the WASM digest.
The imported candidate pins source commit `f3465cb5`.
