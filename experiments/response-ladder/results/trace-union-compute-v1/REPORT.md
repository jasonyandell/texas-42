# Trace-union compute control v1

All 36 fixed fixtures completed on both backends and passed exact action-vector, smallest-tile choice, and independent extracted-policy repricing checks. No failed, refused, or timed-out fixtures. Source, model, and executable identities matched at session end. One bounded cycle was sufficient.

The 31 unsettled roots are summarized below. Times are arithmetic means in milliseconds; preparation, device initialization, and independent repricing are separate from solve time. GPU solve includes plan preparation, rollout/readback, and CPU trie folding/extraction.

| Remaining tiles | Roots | CPU solve | GPU solve | Plans | Rollout/readback | Fold/extract | Lanes/root | Trie nodes/root | GPU faster |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 7 | 1.396 | 1.700 | 0.004 | 1.222 | 0.471 | 960.0 | 2,647.4 | 3/7 |
| 5 | 6 | 3.440 | 3.288 | 0.010 | 1.522 | 1.750 | 3,680.0 | 7,932.8 | 4/6 |
| 6 | 9 | 15.568 | 12.832 | 0.034 | 4.169 | 8.611 | 21,866.7 | 30,612.6 | 6/9 |
| 7 | 9 | 32.859 | 72.925 | 0.210 | 21.222 | 51.379 | 150,400.0 | 97,932.0 | 0/9 |

Five roots were already publicly settled: two at depth 4 and three at depth 5. Their mean CPU/GPU solve times were 0.0176/1.626 ms; they remain in the receipts but are excluded from the unsettled table.

Across unsettled roots, CPU solve summed to 466.256 ms and GPU trace-plus-fold solve to 803.438 ms (1.723× CPU). GPU was faster on 13/31 roots. At depth 7, fold/extract consumed 70.5% of GPU solve time. Exact trace coverage works, but this implementation does not establish a compute advantage at depth 7.

Apple M5 Max / Metal device/pipeline initialization took 13.770 ms separately. The first GPU batch is retained (a settled depth-4 fixture); there is no discarded warmup. Mean unsettled generation/sampling/preparation times were 0.003/0.005/0.009 ms. Independent policy repricing averaged 0.116 ms for CPU and 0.122 ms for GPU.

The complete JSON summary contains sums, means, medians, ranges, lane/state/trace-byte counts, and trie node distributions. The immutable fixture receipts preserve full vectors and policies. Counts are descriptive work evidence, not calibrated CPU units. A single fixed panel does not measure full-game latency, model quality, or browser/phone performance.

A possible next compute-only experiment is exact complete-trace deduplication per original scenario column before trie folding. It must retain each original column and weight and cannot deduplicate by action prefix alone. No such optimization was implemented or measured here.

Build: `RUSTFLAGS="-C target-cpu=native" cargo build --offline --release --features gpu --bin trace_union_bench`. Run: `target/release/trace_union_bench --output results/trace-union-compute-v1 --c0 results/compiled-c0-v3/actor.json --c1 results/compiled-c1-v3/actor.json --seconds 55 --n 40 --depths 4,5,6,7 --seed 960901 --per-backend-ms 1000`. Both ran under the retained 60-second watchdog; scalar tracing was disabled.

Commit: `635547d0c31eb53c5d80c80f16c81f56bb41997c`. Binary SHA-256: `993c2567584a36ce98f946bd44c4904d5d74a56d8e36cba4d17e74e730d7c0e8`. Full actor/source identities are in `build-identity.json`, `protocol.json`, and the session receipt.
