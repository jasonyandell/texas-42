# Reproduce the Fable tiny-net diagnosis

Read [REPORT.md](REPORT.md), [plan.json](plan.json) (frozen design plus
amendments A1/A2), [models/FROZEN.json](models/FROZEN.json) and
[checks/audit-result.json](checks/audit-result.json). Exploratory tier, local only.

Requirements actually used: Apple M5 Max, Python 3.12.13 at
`/Users/jason/code/flux/python/.venv/bin/python` with NumPy 2.4.4 and MLX 0.31.2
(Metal), clang. Every invocation goes through the inherited watchdog
`experiments/astra-sol-20261004/tools/run_capped.py` with `--seconds <= 295`
and a fresh `--output-dir`. Run from the repository root. `PY` below is that
interpreter; `CAP` is the watchdog; `E=experiments/fable-tiny-net-20261004`.

Audit the committed evidence without retraining (rebuilds the ignored kernel):

```sh
python3 $CAP --seconds 60 --output-dir /tmp/fable-kernel -- clang -O3 -shared -fPIC $E/kernel.c -o $E/kernel.dylib
python3 $CAP --seconds 200 --output-dir /tmp/fable-audit -- $PY $E/checks/audit.py
```

Full regeneration in a disposable copy (the scripts refuse to overwrite):
remove `plan.json`, `sources.json.gz`, `positions*`, `data/`, `features/`,
`models/`, `results/` from the copy only, then

```sh
python3 $CAP --seconds 60  --output-dir <log> -- $PY $E/ladder.py initialize
$PY $E/campaign.py --stage mine --batches 0:12            # 12 train blocks, ~16 s each
python3 $CAP --seconds 120 --output-dir <log> -- $PY $E/ladder.py consolidate --split train
$PY $E/campaign.py --stage labels --batches 0:12          # ~3.4 s per block
$PY $E/campaign.py --stage features --batches 0:12        # ~0.8 s per block
$PY $E/train_grid.py                                      # 13 capped trainings, 3-7 s each
# amendment A2 copies: ladder.py train --arch A --deals D --name rs-A-D --also-regret
python3 $CAP --seconds 60  --output-dir <log> -- $PY $E/ladder.py freeze
$PY $E/campaign.py --stage mine --batches 12:13            # test block, only after freeze
python3 $CAP --seconds 60  --output-dir <log> -- $PY $E/ladder.py consolidate --split test
$PY $E/campaign.py --stage test-teacher --batches 0:4
$PY $E/campaign.py --stage test-reference --batches 0:4
$PY $E/campaign.py --stage test-features --batches 0:4
python3 $CAP --seconds 200 --output-dir <log> -- $PY $E/ladder.py evaluate --split test --band all --output test-all --models <frozen .npz paths>
python3 $CAP --seconds 200 --output-dir <log> -- $PY $E/ladder.py evaluate --split test --band 2   --output test-late --models <frozen .npz paths>
python3 $CAP --seconds 120 --output-dir <log> -- $PY $E/ladder.py benchmark --model $E/models/feat-s64-d16.npz
```

The exact executed commands are in each `results/*-log/run.json`. Model weights
are frozen by SHA-256 in `models/FROZEN.json`; training is byte-deterministic
on this host (all twelve re-runs under A2 reproduced the MSE-selected weights).
`results/failed/` and `results/pre-amendment-eval50/` keep the failed and
superseded receipts. The committed `../tiny-net-ladder-20261004` and
`../tiny-net-coverage-20261004` experiments are read, never modified.
