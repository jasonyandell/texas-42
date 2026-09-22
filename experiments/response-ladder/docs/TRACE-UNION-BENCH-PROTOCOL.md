# Exact trace-union compute control, v1

This measures two algorithms on the same immutable sampled response problem against frozen compiled C0/C1 role families. It does not evaluate model quality, infer production superiority, or establish phone performance. The default is 36 predetermined fixtures: nine declarations × depths4,5,6,7. Within each declaration the normalized focal role alternates by `(declaration_index + depth_index) % 2`, giving two declaring and two defending fixtures. Depth7 is trick1. Root depth is the focal remaining hand size. Each root is a trick boundary or has one public partial play to place the scheduled role on turn. A declaring seat always leads the opening; a defending depth7 fixture therefore starts immediately after that first public play.

The command is a new standalone binary; it does not change either solver or the trace-union library:

```sh
cargo build --release --features gpu --bin trace_union_bench
./target/release/trace_union_bench \
  --output results/trace-union-compute-v1 \
  --c0 PATH/TO/FROZEN-C0.json --c1 PATH/TO/FROZEN-C1.json \
  --seconds 55 --n 40 --depths 4,5,6,7 --seed 960901 \
  --per-backend-ms 1000
```

Run each invocation under an external60-second process watchdog, and repeat the identical command to resume until the checkpoint says all fixtures were attempted. Builds and measurements must use the campaign's coordinated CPU/GPU lane. Do not execute a measured cycle while teacher fits or other timed panels run. The binary enforces at most55 cooperative cycle seconds and at most1000ms per backend, including independent policy repricing within that backend's remaining allowance. Driver initialization, final cleanup and receipt I/O are not preemptible by an in-process deadline; the outer watchdog provides the hard boundary. No timed result is claimed by adding this runner.

`--scalar` additionally enables complete scalar priority traces plus the same fold, default off because depth7 can be expensive. It receives the same deadline and records a separate CPU-vector/policy parity result. `--n` permits1..64, and `--depths` permits any strictly ascending nonempty subset of4..7; these change the frozen protocol and require a new output directory.

## Fixed problem construction

Each declaration/depth cell has one fixed derived seed. Shuffle one deal and start with normalized declaring seat1 leading. Play exactly `4*(7-depth)` legal uniformly drawn prefix tiles. If the next actor has the wrong parity for the scheduled role, play exactly one additional legal tile. The new focal seat has not acted in that trick and still holds `depth` tiles. Finally rotate labels only by an even offset to make the focal seat0 (defending) or1 (declaring), and replay the history under that rotation. The opening leader remains odd and the declaring team never changes. This role adjustment is not an outcome-based resample. The receipt records initial leader, even rotation and extra-play count; a focused test replays the root and checks role, hand depth and `model::frame` invariants. Use bid30 and retain every generated position, including already-settled roots, forced roots, preparation failures, and solver refusals. No root is replaced because of payoff, branching, timing, or backend outcome. Settled roots are labeled and must be separated when interpreting useful search cost.

Sample N original ordered scenarios once through `model::sample_problem`, using the fixed seed, public root and own hand. The actual hidden deal is used only to construct a legal public fixture; sampling never receives its opponents' hands. Construct the same `CompiledField::partner_family` for both solvers. The root, scenarios, tapes, weights and complete field revision are saved as JSON with SHA256 before solving. Original arrays and weights are reused without resampling.

The output protocol pins both complete role-family artifacts and their file hashes, options, executable hash, current crate source snapshot hash, and CPU configuration. Source identity is a SHA256 of length-framed sorted source path/content pairs plus Cargo manifests; executable identity is the authoritative executed artifact. Hashing uses the native `shasum` tool and is outside backend solve intervals. The immutable frozen Walt reference is supplied by the unchanged response-ladder dependency. Actor files, source snapshot and executable are hashed again before the final session receipt. Changed or unreadable inputs flag the session and checkpoint as identity-verification failures and return an error while retaining all receipts; a mid-run mutation is not silently accepted as a valid cycle.

## Compared computations and time accounting

CPU runs `core::solve` with full horizon7, one initial priority plan, scenario upper enabled and `stop_when_certified=false`. A timeout may retain useful bounded output, but it is labeled timeout rather than exact parity. The report saves lower/upper vectors, controller work, selected policy and whether the exact vector completed.

GPU runs the complete legal-root permutation family through the existing `EpochEvaluator::traces`, then `trace_union::fold`; all focal plans are pure priorities with no compiled tail. There is no new shader. The fold returns the full exact root vector and selected adaptive policy or no completed result. Missing/incomplete coverage is an error. Report preparation, GPU rollout/readback, fold+policy extraction, whole solve time, backend time including repricing, lane count, 160-byte lane-state extent, actual trace tile payload bytes, compact-trie nodes and the existing device statistics. These byte fields are component sizes, not measured peak memory.

The GPU device persists within a cycle. Cold device/pipeline initialization is recorded separately in the session receipt and counts against the55-second cycle. Batch index is saved so the first dispatched batch is not silently described as a warmed driver measurement. CPU/GPU backend order alternates by fixture index. No measured timing repetitions are performed by this runner.

A complete selected policy is independently repriced against a fresh instance of the same compiled field using the remaining backend budget. Repricing time is recorded separately from solve time. A repricing timeout is not a pass. Exact vector/choice parity passes only when both backends completed and both policies independently repriced to their reported values. If CPU times out, preserve any GPU result and mark parity not comparable; that is never evidence that the GPU values match the exact CPU vector. Semantic mismatches remain explicit results and are not retried until success.

## Durable transaction and resume rules

Each fixture has an atomic JSON checkpoint after preparation and after each backend. A durable `running` marker is written before starting a phase. Completed, timed-out, refused, or mismatching attempts are never silently rerun. If an external watchdog/process interruption leaves a running marker, the next invocation records that phase as `interrupted_external` with no retry. Unstarted phases remain pending and may run in the next cycle using the saved immutable problem. The complete field/program, binary/source identity, seed and options must match the original protocol exactly on resume.

`fixtures/NNN.json` contains the actual problem, preparation receipt, all attempted backend results and comparison status. `sessions/` retains cold initialization and cycle completion records. `checkpoint.json` summarizes attempted/completed fixture states; all attempted does not mean all solved or all parity checks passed. Keep refusals and non-comparable cases in any aggregate denominator, and report settled roots separately.
