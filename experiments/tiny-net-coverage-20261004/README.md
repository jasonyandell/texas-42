# Reproduce the bounded follow-through

Read [REPORT.md](REPORT.md), [plan.json](plan.json) and
[checks/REVIEW.md](checks/REVIEW.md). This folder is an exploratory second stage;
the complete first tiny-net/search/redistill pilot is preserved in
`../tiny-net-ladder-20261004` at commit `04cf0de2`.

Requirements actually used: arm64 Mac, a C compiler, Python 3.12 with NumPy 2.4.4
and MLX 0.31.2, and Node for the pinned phone WASM. Local Python was
`/Users/jason/code/flux/python/.venv/bin/python`. CPU label generation uses the
native C kernel; MLX training requires access to local Metal GPU. Run from repo
root. Every build/generate/train/evaluate/audit/benchmark invocation must pass
through the inherited watchdog, with a new output directory and <=295 seconds:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 \
  --output-dir /tmp/tiny-coverage-build -- \
  clang -O3 -shared -fPIC experiments/tiny-net-coverage-20261004/kernel.c \
  -o experiments/tiny-net-coverage-20261004/kernel.dylib

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 \
  --output-dir /tmp/tiny-coverage-audit -- \
  /Users/jason/code/flux/python/.venv/bin/python \
  experiments/tiny-net-coverage-20261004/checks/audit_coverage.py
```

The committed data/weights make metric replay possible without GPU training.
`evaluate.py evaluate` accepts `--reference reference0 --split test --band 2`,
`--models` paths and a new `--output` folder.
The fresh test/control command is recorded exactly in
`results/test-controls-log/run.json`; all historical commands are in capped
`run.json` receipts. `checks/audit_metrics.py` verifies frozen identities and
saved metric arithmetic; `checks/audit_hybrid.py` replays the full phone smoke.

For full regeneration, use a new isolated checkout/output copy; the scripts
refuse to overwrite existing artifacts. Keep the original history and sources.
Remove generated `plan.json`, positions/source manifests, membership/quotas/mining,
datasets, models and result logs
**only from the new disposable reproduction copy**, then run sequential stages:

1. `coverage.py initialize`, preserving the revised plan and fixed source RNG.
2. `campaign.py --kind mixed128 --batches 0:14`, then late128, subset128 and
   subset512. At most two workers, each child capped at 295 seconds.
3. `train_controls.py` on authorized local Metal GPU. It freezes all five models.
4. `campaign.py --kind reference0 --batches 12:14`, plus
   `campaign.py --kind teacher128 --batches 12:14`; evaluate the late validation
   gate with the common teacher comparator.
5. The saved gate fails: do not run T1 or adapt a diagnostic into a replacement
   gate. `train_adaptive.py` reproduces the separately recorded 375-epoch subset
   diagnostics and freezes both extra weights; it adds no data or labels.
6. After all weights are frozen, generate reserved reference0 batches `14:16`
   and teacher128 batches `14:16`, then evaluate fresh test. The teacher uses
   128 worlds; the independent reference uses 4,096 fresh worlds.
7. `match_campaign.py`, then `match.py summarize`, produces the fixed 48-game
   smoke. Each child has 100 s internal/120 s external limits, two workers maximum.
8. `evaluate.py benchmark` sequentially per model; `diagnostics.py`; independent
   audit scripts. `package.py` writes provenance and SHA256SUMS once all reports
   and audits are final. Verify the checksum file after checkout.

The exact split is by original source partition including all histories and
derived completions, not by row. No automatic remote download, cloud execution,
external transmission, fixture export or production deployment is required.
