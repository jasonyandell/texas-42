# Reproduce the native-policy checkpoint

Read [REPORT.md](REPORT.md) and [fresh Sol review](checks/REVIEW.md). All work is
local; the pinned production source/blob is retained. This is a fixed late40 /
Voidless / `[4,2,2,2]` / `Field::Level(2)` / ascending-ties hybrid. Complete-call
`worlds`/`partner` select the phone fallback profile; they do not configure the
late policy. Measured profiles are opening160/20s/partner-off and ordinary40/14s/
partner-on. No production merge/deploy or fixture export is authorized.

Run from repo root with the existing offline Rust1.95 toolchain, installed
wasm32-unknown-unknown target, Node and Python stdlib. Every individual build,
Lean, test or benchmark must use the inherited watchdog with allowance≤295s.
Use fresh output paths. Preserve the committed receipts and original experiments.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-build-NEW -- cargo +1.95.0 build --offline --locked --release --manifest-path experiments/native-policy-check-20261004/adapter/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-wasm-build-NEW -- cargo +1.95.0 build --offline --locked --release --lib --target wasm32-unknown-unknown --manifest-path experiments/native-policy-check-20261004/adapter/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-control-build-NEW -- cargo +1.95.0 build --offline --locked --release --manifest-path experiments/resumable-choice-20261004/integration/Cargo.toml --target-dir experiments/native-policy-check-20261004/control-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-lean-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/native-policy-check-20261004/checks/PolicyEquivalence.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-phone-check-NEW -- python3 experiments/native-policy-check-20261004/phone_conformance.py --out /tmp/native-phone-results-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-cold-log-NEW -- python3 experiments/native-policy-check-20261004/cold_compare.py --out /tmp/native-cold-results-NEW
```

Only cold timing needs all other CPU builders/checkers paused. It reads the
committed actual-game public turns, charges each cold process/setup/sampler/solve/
transport, and reports common saved-input selection separately. Build bytes may
differ after relocation; old hashes pin the recorded execution, not all future
compiler outputs. Rebuildable targets are ignored. Exact local current binary
hashes are in source/receipt manifests; no missing executable is asserted current.

For a complete eight-game independent-deal block, run one bounded batch per
`plan.json` source seed and backend. For example:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-game-log-NEW -- python3 experiments/native-policy-check-20261004/play.py batch --seed 1656160 --backend native --plan experiments/native-policy-check-20261004/plan.json --out /tmp/native-games-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/wasm-game-log-NEW -- python3 experiments/native-policy-check-20261004/play.py batch --seed 1656160 --backend wasm --plan experiments/native-policy-check-20261004/plan.json --out /tmp/wasm-games-NEW
```

Repeat separately for all18 plan seeds and each backend with unique watchdog
paths. Summary takes `play.py summarize --backend native --plan ... --out ...`
(or wasm), also under a new cap. It refuses a population interval for missing
planned games. Four rotations are one correlated source-deal cluster. Native
and WASM replay the same18 deals; do not double their independent-deal count.

Independent checks and exact reproduction commands live in `checks/*/run.json`.
The independent runner builds with its own `checks/runner/Cargo.toml` and
`--target-dir checks/target`. `check_adapter.py` accepts native/WASM binary paths;
`check_games.py` accepts a complete backend game directory; `audit_saved.py`
audits production records/history/receipts. Retained failed checks are development
history, not accepted evidence. Final sources and new raw records are pinned in
`SHA256SUMS` and the Git commit. The specific blocked Claude transmission remains
blocked. No external service, credentials or persistent access is needed.
