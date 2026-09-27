# Ruby's doubles: 10,000-world panel

EXPLORATORY. This is the approved 10,000-world rerun of both retained Ruby
positions, with seeds 1–8 at each position. It uses the same Node WebAssembly
backend and own/public requests as the 2,000 panel. Hidden hands are sampled by
the solver and are not request fields.

## Method and provenance

The shipped WASM and checked-in source remain unchanged. A temporary source copy
changed exactly two adapter ceilings:

```text
walt/walt-player/src/lib.rs: call.worlds 1..=640 -> 1..=10000
walt/walt/src/solver/partnership_wire.rs: n > 640 -> n > 10000
```

No solver algorithm, game source, or deployment artifact changed. The cap-only
patch is retained in `adapter-cap.patch`; the research WASM is
`walt-player-10000.wasm`, SHA-256
`f939d200e5f9b818ce3aa2b276d2bdefdaf9bc0f50b022027ad25ed327edb329`.
The shipped WASM SHA-256 remains
`fe22d2d24c33e98327799e2e08972481b4e1ec4e48f40256b22b32d0ccd76398`.
Source hashes and the full configuration are in `summary10000.json`.

Every call used `n0=8`, baseline fixed selection, modeled fixed selection,
`inner_belief=Voidless`, no partner review, original seed mixing, and a 20,000
ms decision budget. Before the 10,000 run, all 16 cases were rerun at 2,000 on
this exact research WASM. Their exact rational option vectors and selected
choices matched the retained 2,000 panel. The 10,000 run then completed all 16
cases with `outer_worlds=10000`, `outer_draw_attempts=10000`, and every phase
marked `completed`.

The raw requests, checkpoints, exact rational responses, per-case timings, and
WASM linear-memory measurements are in `panel10000.json`; the parity gate is in
`panel2000.json`. Successful watchdog receipts are
`../10000-build-1/run.json`, `../10000-run2000-parity-3/run.json`, and
`../10000-run-1/run.json`. Each build/run used the packet's `run_capped.py`
with a 295-second allowance. The earlier 2,000 artifacts remain under
`../2000/`.

## Aggregate candidate values

Values are mean eventual-set probabilities across the eight seeds, shown as
percentages. Exact rationals are in `summary10000.json` and the root
`ladder-summary.json`.

| position | worlds | 2–1 | 3–1 | 4–4 | 5–1 | 6–6 | selected frequencies |
|---|---:|---:|---:|---:|---:|---:|---|
| ply 6 | 2,000 | 72.950 | 78.0375 | 79.06875 | 79.075 | 78.025 | 31×1, 44×4, 51×3 |
| ply 6 | 10,000 | 70.60375 | 75.5875 | 74.58125 | 76.705 | 73.48875 | 51×8 |
| ply 9 | 2,000 | 64.54375 | 68.900 | 66.8625 | 70.575 | — | 51×8 |
| ply 9 | 10,000 | 62.65875 | 67.70625 | 63.9625 | 69.345 | — | 51×8 |

At 10,000, 5–1 (`51`) is selected for every seed at both positions. The
previously played doubles are below the best mixed candidate in every seed in
this optimized in-sample panel: 6–6 at ply 6 and 4–4 at ply 9. This is a
sampled continuation-optimization result, not a proof of policy strength or a
confidence interval.

## Timing and memory

| position | cases | mean wall ms | range wall ms | summed solver us | max WASM linear memory |
|---|---:|---:|---:|---:|---:|
| ply 6 | 8 | 8,755.3 | 8,641.5–8,853.4 | 69,786,107 | 196,476,928 bytes |
| ply 9 | 8 | 1,457.7 | 1,442.4–1,474.1 | 11,557,326 | 55,115,776 bytes |
| all 16 | 16 | — | — | 81,343,433 | 196,476,928 bytes |

The timed WASM calls totaled 81.704 s (excluding build and harness overhead). The same research runtime's 2,000
parity panel took 17.653 s, so 10,000/2,000 measured ratios were 4.628× wall
and 4.700× summed solver time. The memory figure is per-case WASM linear memory,
not whole-process RSS and not a phone-memory measurement. All decisions stayed
within the 20-second budget; no cap or timeout adaptation was needed.

## Sample-size trend

| worlds | ply 6 choices | ply 9 choices |
|---:|---|---|
| 40 | 31×1, 44×1, 51×1, 66×5 | 21×1, 31×2, 44×4, 51×1 |
| 160 | 44×3, 66×5 | 31×4, 44×2, 51×2 |
| 640 | 31×2, 44×3, 51×1, 66×2 | 31×2, 51×6 |
| 2,000 | 31×1, 44×4, 51×3 | 51×8 |
| 10,000 | 51×8 | 51×8 |

The complete exact ladder, including per-seed played-minus-best-mixed gaps, is
`../ladder-summary.json`. These are optimized sampled values; the ladder does
not establish exact optimality, calibrated uncertainty, or a production-device
latency result.
