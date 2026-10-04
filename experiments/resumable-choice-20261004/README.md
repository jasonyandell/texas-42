# Reproduce the resumable choice and complete-player checkpoint

Read [REPORT.md](REPORT.md) and [independent review](checks/REVIEW.md).
Run from repository root with Python stdlib, installed Rust1.95 and offline
Lean4.33rc1. Preserve originals and use fresh `NEW` output directories. Every
individual run must use `run_capped.py` with allowance≤295seconds. Final timing
must be sequential with other build/test/benchmark work paused.

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/resumable-choice-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-old-arena-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/choice-arena-20261004/native/Cargo.toml --target-dir experiments/resumable-choice-20261004/checks/old-arena-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-old-demand-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/demand-bounds-20261004/native/Cargo.toml --target-dir experiments/resumable-choice-20261004/checks/old-demand-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-tests-NEW -- cargo +1.95.0 test --offline --release --manifest-path experiments/resumable-choice-20261004/native/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-lean-NEW -- /Users/jason/.elan/toolchains/leanprover--lean4---v4.33.0-rc1/bin/lean experiments/resumable-choice-20261004/math/ResumableChoice.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-check-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/resumable-choice-20261004/checks/runner/Cargo.toml --target-dir experiments/resumable-choice-20261004/checks/resumable-target
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-check-NEW -- experiments/resumable-choice-20261004/checks/resumable-target/release/resumable-independent-check
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/res-one-log-NEW -- python3 experiments/resumable-choice-20261004/compare.py --out /tmp/res-one-NEW --workers 1 --repeats 6
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/res-four-log-NEW -- python3 experiments/resumable-choice-20261004/compare.py --out /tmp/res-four-NEW --workers 4 --repeats 6
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/res-larger-log-NEW -- python3 experiments/resumable-choice-20261004/compare.py --out /tmp/res-larger-NEW --start 967000 --count 128 --workers 4 --repeats 6 --counts 8 40 --fields 1 2
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/res-cold-log-NEW -- python3 experiments/resumable-choice-20261004/cold_pipeline.py --out /tmp/res-cold-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/res-floor-log-NEW -- python3 experiments/resumable-choice-20261004/ablation.py --out /tmp/res-floor-NEW
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-player-build-NEW -- cargo +1.95.0 build --offline --release --manifest-path experiments/resumable-choice-20261004/integration/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-integration-NEW -- python3 experiments/resumable-choice-20261004/checks/check_integration.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/res-game-log-NEW -- python3 experiments/resumable-choice-20261004/arena.py run --seed 975102 --rotation 0 --role declaring --out /tmp/res-games-NEW/game-0-declaring.json
```

For the complete balanced smoke block, invoke the last command separately for
all rotations0–3 and each role `declaring`/`defending`, with distinct capped log
and game paths. Then run `arena.py summarize /tmp/res-games-NEW` under a new cap.
Do not aggregate a partial deal block as population strength. To reproduce
same-policy real-turn cold costs use `adapter_compare.py --game PATH --out NEW`.
The candidate is pinned-phone routing plus late40/[4,2,2,2] Field::Level(2),
Voidless/ascending ties. It is not the phone's default L1/partner policy.

Independent integration checking builds its separate executable in
`checks/integration-target`; build it with the same manifest and `--target-dir`
before running the checker. Historical validation helpers and current
arithmetic/game audits reside in `checks/`; their capped receipts preserve exact
commands. `verify.py` checks the saved checkpoint and requires its named final
result directories and rebuilt binaries; changed build bytes invalidate an old
binary hash rather than establishing a reproduction failure in source semantics.

New mode `resumable_choices:true` is exclusive with other choice flags.
`eager_zero:true` is the separately checked existing L0 floor. Full root vectors
are always returned; refusal emits none. Complete tree choices can survive a
later forest refusal. Fixed path buffers and prepared queries are excluded from
counted bound bytes; use PID RSS for process memory. Kernel comparison preserves sampling/belief/input; the named hybrid changes the
late policy. No generic k theorem, physical phone timing or stronger-play
conclusion.
No production merge/deploy or fixture export; blocked Claude transmission remains
blocked. Rebuildable targets are ignored; all source/receipts/hashes are retained.
