# Reproduce the bounded native frontier continuation

Read [REPORT](REPORT.md) and the [independent report](checks/REPORT.md).
Run from this repository root. Existing offline Rust1.95 and standard Python
are sufficient. NumPy is needed only by the independent historical fixtures
and support-repair check, using the already available environment listed in
the earlier checkpoint. Use new output directories; the watchdog refuses to
overwrite receipts. No dependency installation is needed.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-frontier-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/native-frontier-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/native-frontier-tests-NEW -- cargo +1.95.0 test --offline --release --manifest-path experiments/native-frontier-20261004/native/Cargo.toml

# Fresh47-root one-worker native Dice parity, full cost and scaling.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/native-frontier-one-log-NEW -- python3 experiments/native-frontier-20261004/panel.py --out /tmp/native-frontier-one-NEW --count 128 --counts 8 40 128 512 --workers 1 --standalone --repeats 4

# Same inputs and worker count for independent-query native recursive control.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/native-frontier-four-log-NEW -- python3 experiments/native-frontier-20261004/panel.py --out /tmp/native-frontier-four-NEW --count 128 --counts 8 40 128 512 --workers 4 --batch-reference --standalone --repeats 4

# All lower-query vectors vs exact demanded choices. Both retain sample budgets.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/native-frontier-policy-log-NEW -- python3 experiments/native-frontier-20261004/panel.py --out /tmp/native-frontier-policy-NEW --count 128 --counts 8 40 --mode policy --workers 4 --batch-reference --standalone --repeats 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/native-frontier-hybrid-log-NEW -- python3 experiments/native-frontier-20261004/panel.py --out /tmp/native-frontier-hybrid-NEW --count 128 --counts 8 40 --mode policy --native-choices --workers 4 --batch-reference --standalone --repeats 4

# Bounded later field demand; not opening or k scaling.
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/native-frontier-field2-log-NEW -- python3 experiments/native-frontier-20261004/panel.py --out /tmp/native-frontier-field2-NEW --start 962000 --count 32 --counts 8 --mode policy --field 2 --min-ply 16 --workers 1 --standalone --repeats 2
```

The JSON-lines executable is `native/target/release/native-frontier`. Each
input contains `rows` with `seed`, `request`, `worlds`, `tape` and optional
explicit `seeds`. `request` contains arena declaration ID, bid, bidder, current
seat, original seven-tile actor hand and chronological flat `(seat,tile)*`
plays. Supplied worlds are remaining masks in arena seats; the service checks
full historical legality and rotates exactly once. Supported late straight
queries have12–27 plays, declarations0–7 or9, and remaining-ply cap16.

- `mode`: `tape` for the historical depth/multiply-high tape; `native` for
  state-reset native Dice; `policy` for a response to modeled `field_level`0–2.
- `budgets`: fixed sample counts per modeled rung, default `[4,2,2,2]`.
- `counted`: false for pinned default Voidless, true for VoidsCounted.
- `native_choices`: optional exact choice-only native lower backend; full
  `Service::actors()` vectors remain independent and complete.
- `workers`:1 or4, independent root partitions and per-worker caches/caps.
- `caps`: rows, total charged frontier work, unique actor queries, cache entries
  and seconds. Defaults500k /5m /50k /50k /20s; panels explicitly use20m work
  and30s. Four workers have up to four times each work/memory allowance.
- `reference`, `parallel_reference`, `batch_reference`: optional pinned core
  full vector comparator, within-root parallel mode, across-query parallel mode.
  `reference_only` runs the baseline in its own cold process.
- `warm`: repeat identical complete queries and verify exact cached answers.

Counts are internal **declaring-team T1 success counts**, not always focal
success. T0 actors minimize; T1 actors maximize. `choice` is the first ascending
best tile. Outputs return original root order. Errors return no root vector;
completed child caches can remain after parent refusal. Cache scope fixes
native source, seed schema, straight contracts and Fixed modeled selection.
No disk cache or production caller is added.

`panel.py --standalone` alternates separate baseline and frontier processes,
validates every completed full vector/choice, and records actual-n generation,
JSON, support replay, service, warm lookup, child CPU and RSS. Native recursive
`reference_solve_us` sums query solve intervals and is **not** wall throughput
when `--batch-reference` is enabled; use `reference_total_us` for that control.
Intermediate directories are retained historical development observations;
use final-* for the report's final-source comparisons.

The independent [harness](checks/runner/src/main.rs),
[scalar oracle](checks/scalar_oracle.py) and
[parallel protocol check](checks/parallel_protocol.py) have their capped
reproduction commands in their receipt `run.json` files. Use new receipt names
and build output paths. [INTEGRATION](checks/INTEGRATION.md) links relevant
pinned source locations and enumerates obligations.

`verify.py` audits retained final vectors, production identity, source hashes,
watchdog receipts and absence of changes to predecessor paths. Run it through
the watchdog after rebuilding the pinned final executable. `SHA256SUMS` pins
the committed files in this continuation, excluding itself and ignored build
products. The final Git commit pins the manifest.
