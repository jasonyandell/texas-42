# Reproduce the local component checkpoint

Read [REPORT.md](REPORT.md) and [independent review](checks/REVIEW.md). Run from
the repository root. Rust1.95, installed offline Lean4.33rc1 and Python stdlib
are sufficient; no installation, network, production change or fixture export.

Every individual build/Lean/test/experiment uses the inherited process-group
watchdog with allowance≤295seconds. Use a **new** output directory for each
receipt and study; commands refuse existing directories. Replace `NEW`.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/choice-arena-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-baseline-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/demand-bounds-20261004/native/Cargo.toml --target-dir experiments/choice-arena-20261004/checks/baseline-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-tests-NEW -- cargo +1.95.0 test --offline --manifest-path experiments/choice-arena-20261004/native/Cargo.toml

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/arena-one-log-NEW -- python3 experiments/choice-arena-20261004/compare.py --out /tmp/arena-one-NEW --workers 1 --repeats 6
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/arena-four-log-NEW -- python3 experiments/choice-arena-20261004/compare.py --out /tmp/arena-four-NEW --workers 4 --repeats 6
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/arena-holdout-log-NEW -- python3 experiments/choice-arena-20261004/compare.py --out /tmp/arena-holdout-NEW --start 967000 --count 128 --workers 4 --repeats 6 --counts 8 40 --fields 1 2

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-lean-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/choice-arena-20261004/math/ChoiceArena.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-lean-bounds-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/choice-arena-20261004/math/DemandBounds.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-check-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/choice-arena-20261004/checks/runner/Cargo.toml --target-dir experiments/choice-arena-20261004/checks/candidate-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-check-NEW -- experiments/choice-arena-20261004/checks/candidate-target/release/choice-arena-independent-check

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-sampling-log-NEW -- python3 experiments/choice-arena-20261004/sampling.py --out /tmp/arena-sampling-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-sampling-holdout-log-NEW -- python3 experiments/choice-arena-20261004/sampling.py --start 967000 --count 128 --out /tmp/arena-sampling-holdout-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/arena-cold-log-NEW -- python3 experiments/choice-arena-20261004/cold_pipeline.py --out /tmp/arena-cold-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/arena-cold-holdout-log-NEW -- python3 experiments/choice-arena-20261004/cold_pipeline.py --start 967000 --count 128 --out /tmp/arena-cold-holdout-NEW

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/arena-audit-NEW -- python3 experiments/choice-arena-20261004/verify.py
```

Run timing sequentially with build/check work paused. Six-way rotation balances
process position with6repeats. Native recursion uses identical worker counts,
not summed solve intervals. Four workers have four independent caps. Original
sampling budgets remain `[4,2,2,2]`. Full root vectors always remain complete;
only lower actor consumers use exact bounded choices. No partial root is emitted
on refusal. Native/hybrid counters are not substituted for full actor misses.

The arena JSON flag is `arena_choices:true` (`with_arena_choices` alias), exclusive
with `bounded_choices`/`native_choices`. Public library API is
`Service::with_arena_choices()`. The forest charges all physically retained
trees, prepares before solving and inserts top choices only after success.
Trace collection is validation-only, disabled during timings. Independent
validation uses100k-query/100m-work allowances where stated; performance retains
50k/20m per worker with unchanged sample budgets. Counted allocation peaks are
incomplete subsets and sums of worker maxima, not simultaneous RSS.

`sampling.compiled_fixture(seed,ply,n)` is a reusable exact-stream local fixture
generator. It requires the pinned unchanged source and known-lawful generated
Straight histories. It compiles public following-absence obligations, keeps all
proposal/shuffle/tape draws, and never accepts arbitrary unvalidated requests.
`sampling.py` measures generation with resident imports; `pipeline.py` measures
resident frontend plus cold native. `cold_pipeline.py`/`cold_worker.py` start
fresh Python/native per request and include startup/import/hash/print/parse.
Both cold variants construct the same experiment AST on startup. Root driver
setup/aggregate writing are separately inside the overall capped receipt.

Final authority: `results/final-one`, `final-four`, `final-holdout`,
`sampling-primary-final`, `sampling-holdout-final`, `pipeline-primary-final`,
`pipeline-holdout-final`, `cold-primary`, `cold-holdout`. Earlier results and
failed receipts remain development history. `records.json.gz` contains raw
vectors, costs, counters and resources; each summary pins actual binary/source
provenance. Rebuilt binaries need not match original byte hashes across paths;
source and exact outputs are independently compared.

Every historical manifest remains intact. Build products are ignored, measured
binary hashes retained. No complete production adapter, full phone match,
playing-strength claim, broad parallelism or higher-k linearity is supplied.
No jobs may remain pending at delivery; blocked Claude transmission remains
blocked. [SHA256SUMS](SHA256SUMS) and Git history pin the checkpoint.
