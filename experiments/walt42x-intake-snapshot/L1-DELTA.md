# Repaired L1 at delta 1/24

Measured 2026-09-29 on the M5 Max, using NumPy/float64 and compiled CPU helpers.
L1 means best response to uniform random legal play. Every decision samples 30
deals. All four seats use L1. Each panel uses deal seeds 1–8, field seed 42, four
worker processes, batch 256, and a 60-second cooperative per-game limit.

| Delta | Median game | Game range | Median opening | Games/minute |
|---|---:|---:|---:|---:|
| 1 | 0.111 s | 0.023–0.196 s | 0.084 s | 1,195 |
| 1/8 | 0.334 s | 0.093–0.660 s | 0.226 s | 518 |
| 1/24 | 0.885 s | 0.214–1.570 s | 0.522 s | 226 |

All 24 games completed without fallback. Throughput is completed games divided
by panel wall time, including worker startup and result writing. Game times are
measured inside workers and include first-use loading of existing compiled
kernel caches. This eight-game panel is a small timing sample, not a sustained
throughput or playing-strength study. Identical starting deals can produce
different play and game lengths across delta settings; the policy seed also
includes delta. A game ends when the contract is settled.

At 1/24, the initial one-million-fiber cap stopped the opening searches for
seeds 2 and 5. The rerun explicitly raises the cap to three million for all
three panels; the largest observed frontier was 1,103,825 fibers. Default cap
remains one million. Sample count and policy are unchanged by this resource
setting. The six previously completed 1/24 games retain identical actions and
root values under the larger cap. Retained earlier plies mean the frontier cap
is not an absolute memory bound.

The independent referee verified legality, score progression, contract outcome,
root value bounds, and best-action/tie selection for all 24 completed games.
Receipts are in `results/repaired-v2-l1-d1-panel.json`,
`results/repaired-v2-l1-d1over8-panel.json`, and
`results/repaired-v2-l1-d1over24-cap3m-panel.json`. The initial capped run remains
in `results/repaired-v2-l1-d1over24-panel.json`; verification and computed summary
are in `results/l1-delta-verification.json`.

Reproduce the 1/24 panel with the Python environment containing NumPy and Numba:

```sh
python3 l2_bench.py --level 1 --samples 30 --delta 0.041666666666666664 \
  --games 8 --workers 4 --seconds 60 --fiber-cap 3000000 \
  --output results/l1-d1over24-rerun.json
```

`L1-DELTA-MANIFEST.json` records the current source and evidence hashes.
The older `L2-MANIFEST.json` describes its earlier source revision; this change
adds a configurable game/CLI fiber cap and records it in receipts.
