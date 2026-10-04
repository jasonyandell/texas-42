# Bounded prefix-state continuation

Read [REPORT.md](REPORT.md), [accepted findings](ACCEPTED.md), and the fresh
Sol6.1 collaborator's [review](checks/REVIEW.md). This is a component experiment,
not a phone player. Repository instructions and all predecessor files are
preserved. The parent checkpoint is `c2ab1eb138ed5d19afbd9e9ec9262e0594e6e6a4`.

From this repository root, use the existing process-group watchdog for every
build, test, Lean run and benchmark. It refuses existing receipt directories;
replace NEW with a fresh name. Offline Rust1.95, the installed Lean and standard
Python are sufficient. No install, network, external fixture export or paid
resource is used.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/prefix-state-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-control-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/native-frontier-20261004/native/Cargo.toml --target-dir experiments/prefix-state-20261004/control-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-tests-NEW -- cargo +1.95.0 test --offline --release --manifest-path experiments/prefix-state-20261004/native/Cargo.toml

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-one-log-NEW -- python3 experiments/prefix-state-20261004/compare.py --out /tmp/prefix-one-NEW --workers 1 --repeats 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-four-log-NEW -- python3 experiments/prefix-state-20261004/compare.py --out /tmp/prefix-four-NEW --workers 4 --repeats 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-holdout-log-NEW -- python3 experiments/prefix-state-20261004/compare.py --out /tmp/prefix-holdout-NEW --start 965000 --count 128 --workers 4 --repeats 4

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-policy-log-NEW -- python3 experiments/prefix-state-20261004/compare.py --out /tmp/prefix-policy-NEW --workers 4 --mode policy --counts 8 40 --repeats 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-hybrid-log-NEW -- python3 experiments/prefix-state-20261004/compare.py --out /tmp/prefix-hybrid-NEW --workers 4 --mode policy --native-choices --counts 8 40 --repeats 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/prefix-demand-log-NEW -- python3 experiments/prefix-state-20261004/demand.py --out /tmp/prefix-demand-NEW

python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-lean-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/prefix-state-20261004/math/PrefixState.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-independent-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/prefix-state-20261004/checks/runner/Cargo.toml --target-dir experiments/prefix-state-20261004/checks/runner-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-independent-NEW -- experiments/prefix-state-20261004/checks/runner-target/release/prefix-state-independent-check
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/prefix-audit-NEW -- python3 experiments/prefix-state-20261004/verify.py
```

For the repeated six-root rung timings, add `--start 962000 --count 32
--min-ply 16 --mode policy --field 0` (then1/2) `--counts 8 --workers 1` to
compare.py. `demand.py` instead measures the 2/8/40-world matrix, including the
expected full-frontier field2 refusal at40 worlds under one50k-query allowance.
Each matrix cell is one observation, not a timing distribution.

`compare.py` runs linked-prefix factoring, hash-prefix factoring, the unchanged
predecessor and native recursion in separate cold processes, rotating four-run
order. All have equal worker counts and exact input vectors. It charges actual-n
world generation, JSON, process lifetime, replay, temp-file harness overhead,
parsing, child CPU and PID-specific `wait4` RSS. No timed warm check is added;
independent tests cover completed-cache reuse. Raw service timings remain useful
when common fixture generation dominates total cost.

The JSON-lines API remains the predecessor's API. `hashed_prefixes:true` selects
the factored HashMap control; default uses dense prefix heads and exact child
lists. The public Service API additionally exposes `with_hashed_prefixes()`.
Rows retain world identity and multiplicity; caches retain complete semantic
keys. Each worker has separate caps/caches: four50k-query allowances are a larger
aggregate allowance than one50k allowance. Native Dice, historical depth tapes,
Voidless/VoidsCounted and modeled fields0–2 remain explicit, late straight-only.

Only final-one-v2, final-four-v2, holdout-four, final-policy0-four,
final-hybrid0-four, rung0/1/2-one and demand-matrix support the final tables.
Earlier directories are retained development observations. The initial timing
harness using Darwin time -l failed on a sandbox-denied sysctl; wait4 replaced
it. Initial Lean source used the reserved token `prefix` and failed to parse;
the corrected file passes. No failed receipt is counted as validation.

Large inputs and records are stored losslessly as `.json.gz`; summaries and
watchdog receipts remain plain JSON. Audit scripts accept either representation.
`SHA256SUMS` pins retained files; ignored build products are pinned by receipt
binary hashes. The Git commit pins the manifest. No background job may remain
when delivering the checkpoint.
