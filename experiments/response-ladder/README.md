# Walt response ladder — native GPU player

This is the native M5 implementation of the accepted [GPU Walt design](docs/GPU-WALT-MATH-REDESIGN-DRAFT.md). The playable hybrid uses GPU policy/scenario continuations and exact CPU modeled-player queries. The production CPU acceleration landed independently in `texas-42-partnership-launch` at `f1a0fb04`. This worktree starts from that commit. [Implementation and verification](docs/GPU-PLAYER-IMPLEMENTATION.md) describe the current boundary; [results](RESULTS.md) distinguish current measurements from the earlier CPU-only pilot.

The [compiled lower-rung draft](docs/COMPILED-LOWER-RUNGS-DRAFT.md) maps the
existing Scheme policy runtime, shared learner, gym, finite teachers and
whole-game contrast machinery to the C0/C1 compilation experiment. The
[live campaign](COMPILED-PLAYER-STATUS.md) now records two fitted Scheme actors,
exhaustive fourteen-clause grammar audits, complete CPU/GPU trace checks, and
the exploratory compiled player. The first 72-pair development panel was faster
but weaker than current Walt: CPU 0 wins/8 losses/64 ties, GPU 2/8/62. Mean
partnership time was 19.9ms CPU or 21.6ms GPU versus about 85.6ms for the current
CPU opponent. Both candidates had a 5ms per-move cap. These are changed model
targets, not byte-compatible replacements for the historical lower minds, and
the result does not justify deployment.

`compiled-teacher worker` emits exact T0 action costs against Dice;
`compiled-teacher worker --c0 PATH` emits exact T1 costs against that frozen
compiled C0. Both require a compatible own/public root and retain every sampled
scenario column. `tools/compiled_cycles.py` prepares and teaches the calibration
corpus in resumable bounded cycles; `tools/fit_compiled.py` selects from all 2,381
programs using training costs only. The selected programs are in
`results/compiled-c0-v1/actor.json` and `results/compiled-c1-v1/actor.json`.

`compiled-player worker --c0 PATH --c1 PATH` uses C0 opponents and C1 partner
inside the existing bounded SUM/MAX response. Add `--gpu` to a GPU-feature build
to execute the compiled lower rungs directly in WGSL. Its persistent JSON
request format matches the own-hand/public-history interface below; its config
contains `outer`, `plans`, `horizon`, and `work`. Bid 30 is the declared pilot
scope. `tools/h2h_compiled.py` pins both actors, binaries, sources, configuration
and fixtures, checkpoints completed moves, and compares complete mirrored games
with the actual current CPU wrapper. Its panels remain development data.

The controller starts at trick 1 and responds to a frozen field using a finite ordered bundle of hidden deals and Dice tapes. Every future choice is shared by all scenarios with the same complete public history. It returns a total executable policy, an exact sampled value when that policy has been completely priced, and a valid remaining regret bound. An interrupted calculation preserves completed results. An unpriced reserve is explicitly marked and has only the trivial zero floor.

The two directions of refinement remain separate:

- **Model level:** Dice, all L0, L2 Partner (partner L1, opponents L0), or all L1. Modeled L0/L1 choices call the frozen v34 authority with only that seat's own hand and public information. Incomplete modeled queries abort; they never publish an approximate inner choice.
- **Focal horizon:** extend the lawful shared-action prefix before relaxing the remaining scenario information. Completed rounds retain the better lower and tighter upper bounds for the same bundle and field.

Initial priority policies are priced one per root before spending a second plan on a root. Complete information buckets use SUM, focal choices use MAX, and bounds use integer scenario weights. Duplicate deals remain separate original scenarios. Extracted policies use complete chronological public histories and a lawful priority/default continuation on unseen histories.

`canonical_certified` certifies the least-tile optimal **root action**. It does not alone certify that the returned continuation is an exact best response. `regret_bound == 0` certifies the incumbent's optimal sampled value; `exact_vector` means every root value is exact. None of these eliminates sampling error or establishes stronger actual play.

## Run the GPU player and current-CPU comparison

```sh
python3 tools/prepare_reference.py
RUSTFLAGS='-C target-cpu=native' cargo build --release --features gpu --bin response-player
RAYON_NUM_THREADS=18 ./target/release/response-player worker-gpu
```

The persistent worker prints a readiness record containing the adapter, initialization time and actual CPU query thread count, then accepts the JSON interface below. GPU work now supports L0, L2 Partner and all-L1 fields as well as Dice. Each request creates fresh deadline-bearing modeled minds; the device and compiled pipelines survive requests.

```sh
python3 tools/h2h_current.py results/my-current-h2h --gpu-ms 100 --cpu-ms 14000 --threads 18
```

This harness calls the actual current `experiments/partnership/player.py` wrapper in `/Users/jason/code/texas-42-partnership-launch`; pass `--cpu-root` for another explicit checkout. It pins 72 fresh deals, two per declaration/bidder cell, swaps partnerships for 144 complete games, balances match order within cells, and independently replays all 4,032 physical plays. Both sides use Partner 40/8/2 Fixed/Fixed Voidless. The GPU receives a **per-move** 100ms allowance; the current CPU retains its normal 14,000ms allowance and staged reserves. This answers the current-player comparison, not equal-time strength. Both processes receive a recorded off-panel warmup. Source/binary identities, every fallback, complete move responses and measured partnership times are retained.

## Earlier scalar pilot runner

```sh
python3 tools/prepare_reference.py
cargo build --release --bin response-player
./target/release/response-player game --seed 930100 --decl 6 --bidder 1 --ms 500
```

The game starts at trick 1 and completes all 28 physical plays, including the settled suffix. `--ms` gives the combined game allowance, split equally into two partnership banks. Each decision receives its partnership's remaining bank divided by its remaining plays. Sampling, solving, cleanup and fallback all consume the bank. Game timing also includes dealing, referee work and receipt construction; binary startup and JSON serialization are excluded.

`--candidate-team declarer` or `defender` plays the anytime candidate against the frozen exact v34 solver. `both` uses the new controller for all seats; `neither` uses exact v34 for all seats. The benchmark comparator is **serial v34 plus a canonical legal reserve**, not the production player's staged reserve wrapper. The same model/sample configuration applies to both sides. `--field dice|l0|partner|all-l1`, `--outer`, `--n0`, `--n1`, `--plans` and `--horizon` expose the declared coordinates. Defaults are Partner, 40/8/2, eight plans per legal root, and the full permitted focal horizon.

```sh
python3 tools/run_smoke.py target/release/response-player results/my-smoke
python3 tools/audit_game.py results/my-smoke/930100-declarer.json
```

The smoke protocol pins 12 new deals over pip, doubles and no trump, all bidder seats, then swaps the two partnerships on every deal. It retains every fallback and checks all plays with an independent Python referee. This small pilot diagnoses usability and produces descriptive outcomes; it is not a playing-strength study. The [quality protocol](docs/QUALITY-AT-FIXED-TIME-DRAFT.md) specifies the larger held-out comparison needed for that claim.

## Worker interface

`response-player worker` reads one JSON request per line and writes one response per line. Requests contain only the acting player's information. The complete original own hand is retained for the historical seed; `hand` is its unplayed remainder. Tiles use IDs 0–27; declarations use 0–6 for pip trump, 7 for doubles, and 9 for no trump. History pairs are `[seat,tile]` in chronological arena order. Bidder rotation is handled inside the adapter.

```json
{"decl":6,"bid":30,"bidder":1,"seat":1,"hand":[0,1,2,3,4,5,6],"original_hand":[0,1,2,3,4,5,6],"history":[],"seed":930100,"budget_ms":20}
```

A response includes `tile`, elapsed time, whether a reserve was required, and the complete report/policy when a bundle was evaluated. `worker-gpu` additionally returns epoch/field statistics and its separately computed cheap L1 reserve. A completed reserve value belongs to that reserve's smaller, different model target and is never copied into the main L2 bounds. The worker rejects unknown top-level fields. A completed sampled report is distinct from a partial sampling failure. The `work` cap counts controller/field-query operations; it does **not** count nested frozen pi search nodes. The wall deadline covers those queries. It is not a deterministic work-bounded approximation of a lower model.

## GPU continuation engines

`gpu_epochs::EpochEvaluator` runs one lane per `(total policy, original scenario)`. It executes viewer choices and physical game steps, pauses at nonforced field decisions, batches exact modeled queries on CPU, and resumes the device. Full-history adaptive decisions override the policy's lawful priority default. Only a complete, on-time batch may update the controller. The controller first prices a rank of initial policies across every root, then refines information-consistent SUM/MAX bounds on CPU.

The GPU cache uses complete public state, own hand and outer tape for generic fields. `FrozenModel` separately deduplicates its lawful legacy modeled identity, including banked scores and excluding the unused outer tape, before resolving independent queries in parallel. `unique_field_queries` and `modeled_unique_queries` distinguish those two counts. Neither deduplication merges original scenario/payoff columns.

```sh
cargo build --release --features gpu --bin rollout_bench
cargo test --features gpu --test rollout_contract
./target/release/rollout_bench
```

`gpu::DiceRollout` runs a stackless physical continuation per `(priority policy, original scenario)` and returns a Boolean matrix. It supports the historical Dice field. Host setup is native Metal on the M5; the kernel is WGSL using WebGPU limits and no optional GPU features. Generic fixed-field scalar replay and a compact scalar backend provide independent correctness and performance controls.

`DiceRollout` remains the separate dense-Dice benchmark; `EpochEvaluator` is the online player path. GPU batch timing includes validation, packing, allocation, upload, dispatch, readback and CPU field-query resolution; initialization is recorded separately. A restricted policy pool provides lower witnesses only. A column maximum becomes an unrestricted scenario-revealed upper only for a complete per-root permutation family, and still is not imperfect-information play.

## Verification and scope

```sh
cargo test --tests --no-default-features
cargo test --tests --features gpu -- --test-threads=1
cargo check --lib --target wasm32-unknown-unknown --no-default-features
```

The core tests compare complete root vectors with frozen Walt across all nine declarations and all partial-trick offsets, then independently replay each extracted policy. Directed witnesses cover adaptation beating the complete priority family, clairvoyant upper slack, strict canonical ties, weighted duplicates, timeout retention, off-sample defaults, and changed/illegal field refusals. Adapter tests compare the historical sampling stream and complete modeled-field responses. GPU tests compare full traces as well as payoffs and exercise historical u64 rejection arithmetic. The modeled epoch suite adds 1,480 full-trace comparisons, a reachable adaptive action differing from priority, canonical field-query reconstruction, post-submit cancellation/device reuse, and retention of a completed batch when a later batch aborts.

The portable CPU library compiles for WASM; a browser host must install Walt's monotonic clock. The native CLI and Metal harness are not a browser/phone adapter. Phone performance remains unmeasured.

The controller still recomputes completed horizon passes and refines roots in order. The current demand batches are the ready field frontier of complete-policy lanes; horizon search and nested modeled minds remain CPU work. A persistent horizon frontier, compact policy DAG and recursive GPU lower-mind engine remain possible performance extensions. The native player remains exploratory pending latency and play-quality evidence; CPU/GPU primitive speed alone does not establish either.

## Frozen authority

`reference/native` is an unmodified extraction of `native-v34-mask64-arena-sources.tar.gz`, SHA-256 `804261f2027f3b992043a4f786a66027f1415d12a5132cdce7766b06892f74bf`. The dependency is isolated from unfinished CPU experiment arms and from concurrently changing production sources. `reference/IDENTITY.json` records its origin. Reference-source warnings are preserved rather than editing the frozen authority.

The copied design documents retain their original historical experiment-relative evidence links; the original campaign remains at `/Users/jason/code/texas-42/experiments/full-game-speed`. New implementation receipts below are self-contained.
