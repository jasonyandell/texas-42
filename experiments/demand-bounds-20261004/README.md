# Bounded continuation

Read [REPORT.md](REPORT.md) and [fresh independent review](checks/REVIEW.md).
This is an exploratory late-straight component, not a phone player. Source
checkpoint7f4c68a and every previous report/receipt are preserved. All commands
below run from this isolated repository root using installed offline Rust1.95,
Lean4.33rc1 and standard Python; no install/network/export is required.

Every individual build/test/experiment must use the inherited process-group
watchdog with allowance<=295seconds. Use fresh output directories; existing
directories are deliberately refused. Replace NEW with a fresh suffix.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/demand-bounds-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-baseline-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/prefix-state-20261004/native/Cargo.toml --target-dir experiments/demand-bounds-20261004/checks/prefix-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-tests-NEW -- cargo +1.95.0 test --offline --release --manifest-path experiments/demand-bounds-20261004/native/Cargo.toml

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/demand-one-log-NEW -- python3 experiments/demand-bounds-20261004/compare.py --out /tmp/demand-one-NEW --workers 1 --repeats 5
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/demand-four-log-NEW -- python3 experiments/demand-bounds-20261004/compare.py --out /tmp/demand-four-NEW --workers 4 --repeats 5
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/demand-holdout-log-NEW -- python3 experiments/demand-bounds-20261004/compare.py --out /tmp/demand-holdout-NEW --start 967000 --count 128 --workers 4 --repeats 5 --counts 8 40 --fields 1 2

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-lean-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/demand-bounds-20261004/math/DemandBounds.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-independent-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/demand-bounds-20261004/checks/runner/Cargo.toml --target-dir experiments/demand-bounds-20261004/checks/candidate-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-independent-NEW -- experiments/demand-bounds-20261004/checks/candidate-target/release/demand-bounds-independent-check

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-reuse-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/demand-bounds-20261004/reuse/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/demand-reuse-log-NEW -- python3 experiments/demand-bounds-20261004/reuse.py /tmp/demand-reuse-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/demand-audit-NEW -- python3 experiments/demand-bounds-20261004/verify.py
```

Run timing panels sequentially with other build/experiment work paused; root
native and candidate worker counts are equal. There are five separate cold
variants: pure integer bounds, bounded higher rungs with batched eager L0,
unchanged full prefix frontier, unchanged native-choice hybrid and unchanged
native recursion. Full root vectors always stay complete. Rotation across five
repetitions balances process order; raw `records.json.gz` retains every vector,
worker counter and PID-specific CPU/RSS measurement. Only `final-one`,
`final-four`, `final-holdout` support final timing tables. Earlier folders are
development observations with earlier binary identities. Refusals publish no
root vector, have aggregate worker statistics, and enter no speed ratio.

Original sampling budgets remain `[4,2,2,2]`. Worker allowances stay
500k rows/20m work/50k actor queries/50k cache entries/30seconds. Four workers
have four separate allowances. The public `Service::actor_choices` supports
full/native/bounded modes; `with_bounded_choices().with_eager_zero()` enables
the ablation. Top-level `actors()` still returns complete vectors. The JSON-lines
executable adds `bounded_choices:true` and optional `eager_zero:true`; full and
native-choice defaults remain the previous API. Native-choice and bounded flags
are mutually exclusive in JSON input.

`with_choice_trace()`/`take_choice_trace()` are validation-only. The independent
runner checks every demanded call on the six-root field2/eight-world panel
against fresh full/native Services, including repeats and hits. Those validation
Services explicitly use100k queries/100m work; no sample-budget change occurs.
The traced table/keys preserve original `prepare_actor`, sampler, eager fold and
validation code. Traces are disabled during timing, with validation work and
allocation confined to its separate capped receipt.

`reuse.py` generates distinct prospective successive positions, runs cold,
retained-cache and pinned native evaluation for both beliefs/fields0–2, and
reports query and actual sampled-world savings separately. Its wait4 process
cost includes the whole validation study, not retained-cache-only performance.
Actual-n generation and fixture JSON/write are separately charged; wrapper and
receipt construction are outside that component interval. No reusable online
hit-rate or production speed claim follows from it.

`SHA256SUMS` pins retained source/evidence; build products are ignored and pinned
by result hashes. The Git commit pins the manifest. Failed owner Lean/checker
setup receipts and prior first-worker refusal presentations remain historical
development evidence, excluded from green validation. Local fixtures remain
local. No production merge/deploy or external transfer is authorized by this
checkpoint, and no background job may remain at delivery.
