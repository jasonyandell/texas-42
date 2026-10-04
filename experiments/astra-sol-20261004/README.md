# Reproduce the Walt research checkpoint

Start with [the useful results and limits](REPORT.md) and
[Sol's independent challenge/audit](math/sol/report.md).

Run from the repository root. Use a fresh output directory each time.
Every experiment and Lean invocation must go through the included watchdog;
it refuses limits over 295 seconds and kills the whole process group.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-identity-1 -- python3 experiments/astra-sol-20261004/tools/verify_identity.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-proof-1 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/math/lead/Disagreement.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/math/sol/ArgmaxAmplification.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-finite-1 -- python3 experiments/astra-sol-20261004/math/lead/check_disagreement.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-panel-log-1 -- python3 experiments/astra-sol-20261004/tools/run_panel.py --output /tmp/walt-panel-1
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 20 --output-dir /tmp/walt-summary-1 -- python3 experiments/astra-sol-20261004/tools/arena.py summarize /tmp/walt-panel-1
```

Lean is core-only (Init/Std); no mathlib build is needed. The `+toolchain`
syntax uses elan, which is installed on this Mac. The log receipts preserve
the absolute Lean binary actually used during verification.

`arena.py run` accepts `--candidate-wasm PATH --candidate-label NAME` for a
candidate implementing the same restricted phone JSON/ABI, and
`--candidate-partner` to retain its review. Run one complete rotated/swapped
block per source deal and supply an explicit predeclared plan for summaries.
`run_panel.py` is the stock review-off ablation, or a self-control with
`--candidate-partner --candidate-label phone-self-control`. Larger work should
be decomposed into separate bounded source-deal blocks, never a longer timeout.

Pinned production inputs are in `reference/production-phone/`.
`reference/phone/` and `results/smoke*` identify the older local artifact used
only before production resolution. They are excluded from production reports.
`results/production-panel/` contains the complete first panel, including the
predeclared plan, all attempts, all calls, all responses and time measurements.

The Git worktree shares existing local source objects read-only through the
sibling `source.git` clone. Keep those checkouts available, or clone the public
Texas-42 repository at the source commit in PROVENANCE.json and restore this
experiment directory there. No production source was modified.
