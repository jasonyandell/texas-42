# Reproduce the higher-k budget checkpoint

Read [REPORT.md](REPORT.md), [independent review](checks/REVIEW.md), original
[plan](plan.json) and explicitly adaptive [earlier plan](earlier-plan.json).
Exploratory local-only; no production changes or external fixture export.
Existing offline Rust1.95, installed wasm32 target, Node, Python stdlib and
Lean4.33.0-rc1 suffice. Every individual build/test/Lean/benchmark uses the
inherited process-group watchdog with allowance≤295s. Use new output paths.

From repository root, rebuild actual baseline artifacts:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/k-native-build-NEW -- cargo +1.95.0 build --offline --locked --release --manifest-path experiments/higher-k-budget-20261004/adapter/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/k-wasm-build-NEW -- cargo +1.95.0 build --offline --locked --release --lib --target wasm32-unknown-unknown --manifest-path experiments/higher-k-budget-20261004/adapter/Cargo.toml
```

Actual measured and final binaries are saved as deterministic gzip artifacts
in `artifacts/`, pinned by [ARTIFACTS.json](ARTIFACTS.json). Restore them on a
clean clone with `python3 experiments/higher-k-budget-20261004/restore_artifacts.py`
before running saved byte-identity audits; it verifies archives and raw bytes,
refuses to overwrite a different existing artifact, and creates only ignored
local target files. Rebuilt binaries are ignored outputs; receipts hash the
measured bytes.
Relocation/toolchain can change bytes; freshly hash new outputs. Plans/fixtures
remain canonical inputs. Do not rerun initialize on committed plans.
`WALT_K_RESULTS` is a temporary task-specific output directory override; it
preserves all committed gold records. A single bounded reproducible block:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/k-one-root-log-NEW -- env WALT_K_RESULTS=/tmp/k-replay-NEW python3 experiments/higher-k-budget-20261004/experiment.py roots --seed 3851398
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/k-one-game-log-NEW -- env WALT_K_RESULTS=/tmp/k-replay-NEW python3 experiments/higher-k-budget-20261004/experiment.py games --seed 3851398
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/k-one-earlier-log-NEW -- env WALT_K_RESULTS=/tmp/k-replay-NEW python3 experiments/higher-k-budget-20261004/extension.py run --seed 3851398
```

For full replication use a different new output directory from the single-block
example. Each campaign command is a sequential orchestrator; EVERY child block
has its own295s watchdog and unique receipt. Campaigns skip already complete
source blocks; partial files refuse overwrite. No detached jobs are created.
Pause other builders/checkers during measurements.

```sh
env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/campaign.py roots
env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/campaign.py games
env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/campaign.py unconstrained
env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/extension.py campaign
env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/campaign.py parallel
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/k-summary-log-NEW -- env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/experiment.py summarize
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/k-earlier-summary-log-NEW -- env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/extension.py summary
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 60 --output-dir /tmp/k-analysis-log-NEW -- env WALT_K_RESULTS=/tmp/k-full-NEW python3 experiments/higher-k-budget-20261004/analyze.py
```

Original54deals/216root positions/7suffix cells remain54independent source
clusters; rotations, roles, horizons, rungs and repeated timing batches are not
extra independent deals. Refusals stay in cost/outcome denominators, without
partial vector comparisons or timing ratios. Nominal20/200ms is not a strict
host-wall guarantee. Complete process-group walls, child CPU/PID RSS and raw
counter/sample clocks are separate measures.

Reviewer reproduction commands and exact input identities live in
`checks/*/run.json`. Build its independent runner into a fresh target, then run
its checker scripts with fresh cap receipt directories. `audit_inherited.py`,
`audit_data.py` and`audit_scaling.py` check retained bytes/arithmetic without
rerunning timings. [ChangingOpponent.lean](checks/ChangingOpponent.lean) compiles
under the same watchdog with the installed Lean executable; it is a generic
counterexample, not a Texas42/complexity proof.

`snapshots/` retains exact historical measured harness bytes. Final fresh-output
routing and summary-label repairs preserve policy/runtime expressions. The
[new-output reproduction receipt](results/reproduce-routing-check/run.json)
checks40requests against complete gold vectors. Failed/development outputs are
retained and are not accepted policy or performance results.

A separate clean local clone restored all four archives and passed the full saved
data audit; receipts are in `results/cold-clone-log`, `cold-restore-log` and
`cold-saved-data-audit`. `SHA256SUMS` pins all new committed evidence except itself.
