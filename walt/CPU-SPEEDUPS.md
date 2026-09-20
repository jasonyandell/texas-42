# Native CPU speedups — frozen v34 integration

**EXPLORATORY engineering evidence.** This integrates the completed CPU campaign
into the current partnership player. It changes the implementation of the existing
sampled game, preserving sample identity (including repeated deals), deterministic
Dice tapes, ascending choice ties, lawful information sets, exact rational root
values, and refusal on an incomplete comparison. It is not a new belief model,
stronger-player result, exact full-game solution, or kernel proof.

## Use in current experiments

From the repository root:

```sh
bash walt/tools/build_cpu.sh
```

This builds the existing experiment entry points at `walt/target/release/`:
`walt-table`, `kiln-worker`, `kiln-play-worker`, `kiln-decision-worker`,
`kiln-scheme-contrast-worker`, `partnership`, `partner_rollout`, and
`partnership_gym`, plus `walt-cpu-bench`. Existing native experiment adapters
already use these paths. The script uses the locked offline dependencies,
`target-cpu=native`, thin LTO, one codegen unit, and the workspace's checked
integer overflow. Binaries built this way target the machine that built them.

The `cpu-speedups` feature is enabled by default in both `walt` and `walt-player`.
Ordinary native Cargo builds therefore use the new solver paths too; the script
also supplies the compiler settings used in the timing comparison. There are no
new sampler, model, sample-budget, or deadline defaults. `RAYON_NUM_THREADS`
continues to control worker count; the reported timing panel used 18.

For the benchmark's optional process-local macOS scheduling control, build with
`bash walt/tools/build_cpu.sh --features mac-qos`, then pass
`--mac-qos interactive` to `walt-cpu-bench`. Its default remains ordinary scheduling.
This option belongs only to the benchmark runner.

## What is enabled

The umbrella enables the validated v34 implementation and its prerequisites:

- Compact Dice and modeled-policy evaluation, stack preparation, exact bounded
  choice, one-world and two-trick specializations, static trick outcomes,
  constant objective specialization, and deferred record hashing.
- The identical small-range RNG with frozen rejection-state checks; sample
  positions and random consumption remain unchanged.
- Cheap pure L0 policy recomputation instead of cache traffic, worker-sharded
  diagnostic counters, aligned policy caches, sharded root memoization, and an
  arena of 64-bit support masks for 9–64 sampled worlds.
- Parallel root comparisons and adaptive field prewarming, with serial paths
  for small jobs and one-thread pools.
- Coalesced internal deadline checks plus mandatory completion/cancellation
  checks at comparison boundaries; no partial vector becomes a decision.
- Guarded within-hand policy-cache transfer. A balanced boundary `(B,H)` has
  normalized coordinate `H + |B|/4`. Transfer additionally requires identical
  declaration, bid, inner sample budgets, inner-belief strategy, and modeled
  selection. Malformed or incompatible boundaries refuse transfer.

The generic implementations remain available for unsupported sizes and modes,
and in a `--no-default-features` build. Voids-counted inner beliefs keep their
own sampling and cache context. Supports above 64 worlds retain the general
arena. Diagnostic node counts, cache sizes, and policy-call counts may change;
these counters never choose moves or stop search.

Rejected payoff tables, packed tables, root pivoting, padded tables, inner
parallelism, and unfinished post-v34 bucket/profiling work are absent. The only
lookup artifact is the validated 5,531,904-byte completed-trick table. Regenerate
or check it against the canonical rule algebra with:

```sh
cargo run --offline --release --manifest-path walt/Cargo.toml -p walt \
  --example generate_trick_table -- --check
```

## Evidence and limits

The [receipt directory](receipts/cpu-speedups-v34/) holds the complete full-game
inputs, exact rational option vectors, independent replay/comparison, process
usage, binary hashes, source hashes, and compiler settings. The baseline was the
unmodified current player at `5ab08bbc`, with the same dependency versions,
native compiler flags, thin LTO, one codegen unit, overflow checks, ordinary
scheduling, and 18 Rayon workers as the optimized build.

The panel is fixed L2 Partner 40/8/2 with voidless inner belief, no cache carry,
and completed 20-second-per-call comparisons: frozen G1, eight shuffled seeds,
and additional pip/doubles/no-trump fixtures. All 12 games (336 plays) preserve
every exact option vector and choice. Median paired speedup was **13.13x**
(range 9.33–15.80x); median full-game time fell from **1.830s to 0.143s**.
The matched timing summary is in
[`comparison.json`](receipts/cpu-speedups-v34/comparison.json). Eight original
frozen-v34 games (224 plays) also preserve every vector and choice. These are
finite conformance receipts, not universal equivalence proofs.

The live wrapper has staged searches and a wall-clock budget. Faster evaluation
can complete a later stage or more partner-review work before the same deadline;
its resulting choice can therefore differ from a slower build's completed
prefix. The measured speedup concerns completed fixed solves. It does not
establish strength improvement or a device-wide speedup guarantee.

Portable fallback and optimized `walt-player --lib` builds pass for
`wasm32-unknown-unknown`. The frozen-clock comparison covers all nine
declarations, higher contracts, and a complete partner review; exact actions and
values agree. The existing phone build with `--no-default-features` retains its
fallback configuration. To explicitly test the portable optimized library:

```sh
cargo build --offline --release --manifest-path walt/Cargo.toml -p walt-player \
  --target wasm32-unknown-unknown --no-default-features --features cpu-speedups --lib
```

No phone asset is published by this landing. Native scheduling acceleration does
not apply to a single-thread WebAssembly host.

## Reproduce a comparison

`walt-cpu-bench` generates a lawful whole game and emits the exact native request
and complete response for every play. It directly requests the declared fixed
search; it does not include the live wrapper's earlier reserve or fallback.
`--help` describes its fixed seeds, contracts, cache modes, and bounded deadlines.

Build two binaries with identical compiler/dependency settings. The benchmark's
`--no-default-features` build selects the general reference implementation; save
it separately before building the default optimized runner. For a historical
source baseline, use a preserved pre-change executable. Then run:

```sh
python3 walt/tools/verify_cpu_speedups.py \
  --baseline /absolute/path/to/reference-binary \
  --optimized /absolute/path/to/optimized-binary \
  --baseline-revision REFERENCE_REVISION \
  --output-dir /absolute/path/to/new-receipts --threads 18
```

The tool alternates pair order, refuses to overwrite evidence, records its binary
and source identities, and independently replays all 28 plays and exact choices
before reporting timings. Run it without other timed workloads. For portable
semantic parity, `walt/tools/compare_cpu_wasm.mjs BEFORE.wasm AFTER.wasm OUT.json`
compares complete frozen-clock play and review outputs, allowing work counters
to change. Scoped verification commands and results are in
[`checks.json`](receipts/cpu-speedups-v34/checks.json).
