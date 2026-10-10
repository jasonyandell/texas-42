# Reproduce the bounded tiny-net ladder pilot

Read [REPORT.md](REPORT.md), [plan](plan.json), [provenance](PROVENANCE.json) and
[independent review](checks/REVIEW.md). All outputs are exploratory and local-only.
Saved datasets, packed paired outcomes, sample/teacher manifests and weights are
committed. The phone artifact and original Fable sources remain inherited inputs.

This Mac used `/Users/jason/code/flux/python/.venv/bin/python` (Python3.12.13,
NumPy2.4.4, MLX0.31.2), Clang and Node. CPU labels/evaluation require NumPy; training
requires actual local MLX Metal access. The sandbox did not expose Metal: run the
bounded GPU commands with the host's authorized GPU access. No network/install
step is part of reproduction. For another installed runtime, edit the `PY`
constant in the campaign helpers and use its executable in commands.

From repository root, rebuild ignored native test products, then independently
audit committed evidence (use fresh receipt paths):

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-native-build-NEW -- clang -O3 -shared -fPIC experiments/tiny-net-ladder-20261004/kernel.c -o experiments/tiny-net-ladder-20261004/kernel.dylib
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-wrapper-build-NEW -- clang -O3 -shared -fPIC experiments/tiny-net-ladder-20261004/checks/wrapper.c -o experiments/tiny-net-ladder-20261004/checks/wrapper.dylib
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-rules-audit-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/checks/audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-data-audit-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/checks/audit_data.py
```

The additional independent metric/h2h scripts under `checks/` reproduce the
remaining assertions. Every exact executed command is saved in its `run.json`.
`SHA256SUMS` pins the deliverable files, excluding itself and ignored binaries.
Native binaries are rebuilt products; source, model and dataset identities are
the authorities. Model SHA256 N0 starts `70c37e26`, N1 `7877c11c`. The retained
second N0 training run is byte-identical to the first.

For a complete new label/training run, create a fresh sibling experiment
directory under `experiments/`, then copy **only** this experiment's top-level
`.py`/`.c` files and `checks/*.py`/`checks/*.c` into matching paths. This preserves
all committed results and keeps inherited `partnership/` and `astra-sol/` paths
resolvable. In the commands below replace `tiny-net-ladder-20261004` with that
fresh directory's name. Do not initialize over the committed plan.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-fresh-native-NEW -- clang -O3 -shared -fPIC experiments/tiny-net-ladder-20261004/kernel.c -o experiments/tiny-net-ladder-20261004/kernel.dylib
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-init-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/pilot.py initialize
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/tiny-labels0-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/campaign.py --kind labels0
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/tiny-ref0-val-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/campaign.py --kind reference0 --batches 12:14
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/tiny-train0-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/pilot.py train --name n0-32 --hidden 32 --epochs 100
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/tiny-linear-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/pilot.py train --name linear0 --hidden 0 --epochs 100
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/tiny-val0-NEW -- /Users/jason/code/flux/python/.venv/bin/python experiments/tiny-net-ladder-20261004/evaluate.py evaluate --models experiments/tiny-net-ladder-20261004/models/n0-32.npz experiments/tiny-net-ladder-20261004/models/linear0.npz --output validation0-final
```

Inspect the validation gate before redistilling. If it passes, run `campaign.py`
with `--kind labels1 --model <fresh>/models/n0-32.npz`, generate `reference1` batches
12:14 with the same frozen weights, then train `--kind labels1 --name n1-32`.
Freeze model choices before generating reference0/1 batches14:16 and evaluating
`--split test`. Compare N0/N1 on the **same** reference when checking change.
Each label campaign resumes complete batches and refuses overwriting a failed
receipt. Both each child and parent campaign are capped; split batch ranges if
running on a slower Mac. No invocation may exceed300seconds.

`match.py run` performs one phone game. `match_campaign.py` runs the complete
two-deal, four-rotation, two-role blocks for teacher0/N0/N1, with120-second caps
per game and at most two games concurrently. `match.py summarize` refuses missing
pairs. `diagnose.py` analyzes the96-root4096-world control after it is generated
with `pilot.py generate --kind control --batch 0 --worlds 4096`.

The preserved failed invocations and corrected gate selector are explained in
[CONTRADICTED.md](CONTRADICTED.md). No new Lean learning guarantee is asserted.
