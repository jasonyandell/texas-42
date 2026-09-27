# Ruby's doubles: 2,000-world feasibility panel

EXPLORATORY. This reruns both retained Ruby lead positions (ply 6 and ply 9),
seeds 1–8, for 16 comparisons at 2,000 accepted outer worlds. It uses the same
Node WebAssembly backend and request data as the 640 panel. The request carries
only Ruby's own/public history; hidden hands are sampled by the solver.

## Method and provenance

The shipped WASM remains unchanged (`fe22d2d24c33e98327799e2e08972481b4e1ec4e48f40256b22b32d0ccd76398`).
A temporary source copy changed exactly two adapter ceilings, from 640 to 2,000:

```text
walt/walt-player/src/lib.rs: call.worlds 1..=640 -> 1..=2000
walt/walt/src/solver/partnership_wire.rs: n > 640 -> n > 2000
```

No solver algorithm, game source, or deployment artifact was changed. The
research WASM is retained as `walt-player-2000.wasm`, SHA-256
`1792763f5f6a9a9e8c12591dbe4dab6569c190812aef0089b6f22019aef6e81a`.
The modified source-file hashes and full configuration are in
`summary2000.json`.

Parent-agent audit independently compared all Rust source files in the local
copy against worktree HEAD `e4c549e2fd533c942f39125bf3134ac707c4e638`:
only those two cap checks differed, and the workspace Cargo.toml was unchanged.
Build commands and logs are preserved in `build-run-1/` and `build-run-2/`.
The first attempt lacked a relative rob workspace dependency; copying that
dependency into the temporary layout allowed the second build to complete.

Both panels used `n0=8`, fixed outer selection, fixed modeled selection,
`inner_belief=Voidless`, no partner review, the original wire seed mixing, and a
20,000 ms budget. The 640 run was a parity gate: all 16 cases matched the
retained `panel640.json` in exact rational option vectors and selected choices.
Every 2,000-world case completed with `outer_worlds=2000` and
`outer_draw_attempts=2000`; no fallback or timeout was selected.

The raw request, checkpoint, response, per-case wall timing, summed solver timing,
and peak WASM linear-memory measurement are in `panel640.json` and
`panel2000.json`. Watchdog receipts are `run640-local-1/run.json` and
`run2000-1/run.json`. The run command was the packet's `run_capped.py` with a
295-second allowance.

## Results

The 640 panel took 5.672 s wall time in this same-runtime rerun, with 5.334 s
summed solver time and 19.27 MB peak WASM linear memory. The 2,000 panel took
17.710 s wall time, with 17.373 s summed solver time and 53.87 MB peak WASM
linear memory. The 2,000/640 ratios are 3.123× wall and 3.257× solver time.
The memory number is WASM linear memory observed per case; it is not whole-process
RSS and should not be read as phone memory usage.

Average estimated eventual set probabilities (same eight seeds at both sizes):

| Lead | First position, 640 | First position, 2,000 | Second position, 640 | Second position, 2,000 |
|---|---:|---:|---:|---:|
| 2–1 | 73.11% | 72.95% | 65.45% | 64.54% |
| 3–1 | 79.06% | 78.04% | 70.08% | 68.90% |
| 4–4 | 80.61% | 79.07% | 69.65% | 66.86% |
| 5–1 | 79.88% | 79.08% | 70.98% | 70.58% |
| 6–6 | 80.66% | 78.03% | — | — |

The 4–4 and 5–1 first-position averages differ by only 1/16000: this panel
does not separate them. The second-position 5–1 lead is selected in all eight
2,000-world runs. More outer samples do not change the eight-world inner model.

| Position | Mean 640 wall time | Mean 2,000 wall time | Linear projection at 10,000 |
|---|---:|---:|---:|
| Before 6–6 | 0.570 s | 1.820 s | 9.10 s |
| Before 4–4 | 0.139 s | 0.393 s | 1.97 s |

The projections are estimates, not measured 10,000-world results. They exclude
build time. The full 16-run watchdog took 17.809 s; 17.710 s above is the sum of
the timed WASM calls.

The separate [double-exposure diagnostic](../DOUBLE-EXPOSURE.md) addresses why
a winning double is not automatically useful: it can grant the declarer a safe
double play or a free mixed-tile discard. In contrast to 6–6, 4–4 can catch a
declarer whose only double is 5–5. Neither observation proves the best lead under
Ruby's hidden information.

| ply | seed | 640 choice | 2,000 choice | 640 wall ms | 2,000 wall ms | ratio | 2,000 solver us | 2,000 peak WASM bytes |
|---:|---:|:---:|:---:|---:|---:|---:|---:|---:|
| 6 | 1 | 31 | 31 | 594.5 | 1828.8 | 3.08 | 1785926 | 53673984 |
| 6 | 2 | 44 | 44 | 581.3 | 1848.4 | 3.18 | 1816378 | 53870592 |
| 6 | 3 | 44 | 44 | 553.4 | 1803.1 | 3.26 | 1769453 | 52953088 |
| 6 | 4 | 44 | 51 | 553.6 | 1798.7 | 3.25 | 1767773 | 53280768 |
| 6 | 5 | 66 | 51 | 579.6 | 1819.2 | 3.14 | 1787622 | 53870592 |
| 6 | 6 | 31 | 51 | 577.5 | 1870.8 | 3.24 | 1839704 | 52035584 |
| 6 | 7 | 66 | 44 | 541.2 | 1782.6 | 3.29 | 1753752 | 53739520 |
| 6 | 8 | 51 | 44 | 580.3 | 1811.4 | 3.12 | 1785744 | 53411840 |
| 9 | 1 | 31 | 51 | 138.7 | 395.6 | 2.85 | 383871 | 18677760 |
| 9 | 2 | 51 | 51 | 135.5 | 399.1 | 2.94 | 389678 | 18481152 |
| 9 | 3 | 31 | 51 | 141.1 | 393.0 | 2.79 | 384523 | 18677760 |
| 9 | 4 | 51 | 51 | 144.5 | 391.8 | 2.71 | 382561 | 18481152 |
| 9 | 5 | 51 | 51 | 138.7 | 391.9 | 2.83 | 381335 | 18612224 |
| 9 | 6 | 51 | 51 | 135.1 | 401.6 | 2.97 | 390683 | 18153472 |
| 9 | 7 | 51 | 51 | 136.0 | 387.9 | 2.85 | 378215 | 18350080 |
| 9 | 8 | 51 | 51 | 140.6 | 386.4 | 2.75 | 375496 | 18612224 |

At ply 6, choices moved from 31×2, 44×3, 51×1, 66×2 at 640 to 31×1,
44×4, 51×3 at 2,000. At ply 9, 51 was selected in all eight 2,000-world
runs; it was selected six of eight at 640. The exact mean set-probability
rationals are recorded in `summary2000.json`; these are optimized sampled
values, not fixed-plan confidence intervals or an optimality proof.

## Feasibility assessment

This bounded 2,000-world panel completed comfortably inside the 295-second
watchdog. A simple linear extrapolation of the measured 17.710 s panel gives
about 88.6 s for the same 16 cases at 10,000 worlds, with roughly 5× the
2,000-world per-case work. That is evidence that 10,000 is feasible for this
Mac/Node/WASM harness, subject to memory growth and ordinary runtime variance.
It is not a phone latency or phone-memory result, and the shipped adapter still
caps requests at 640. No 10,000-world run was made here.
