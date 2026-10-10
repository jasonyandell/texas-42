# PERF: making the teach -> train -> h2h cycle faster

Claude Fable 5.1, 2026-10-04/05, performance pass on the rung-2/rung-3 distillation cycle. Exploratory tier.
Hard constraints kept: every heavy process under the 295 s cap wrapper, local only, no new crates, `ingest/`
and the other experiment directories untouched, protocol and teacher semantics unchanged. Every speedup below
is **output-identical** (byte-compared), except the pipeline's overlap, which changes *when* a student is used
(stated under "Pipeline"). Old paths are untouched: `h2h.py`, `log2_launch.py`, `train.py`, `ladder/target-g`
behave as before; the new binary is `ladder/target-s/release/ladder`, new scripts are `h2h_fast.py` and
`pipeline.py`.

## Before / after (same machine, 18 cores = 6 P + 12 E; numbers measured with the old and new binary running
side by side unless noted)

| stage | before | after | factor | how verified |
|---|---:|---:|---:|---|
| rung-2 teacher (`log2` 40/8), ms per labeled state, 1 process | 18.3 | 16.4 | 1.12x | 200 games, 3,271 states, `cmp` identical label files (also at 160/8: 79.5 -> 71.9 ms, identical) |
| rung-3 teacher (`log2 --model`, 160 outer, net as rung-1 others), ms per state | 38.9 | 26.1–28.1 | 1.4–1.5x | 8–16 games, `cmp` identical label files; net forward `agree --dump` score dumps identical on 4 encodings/sizes |
| h2h `np:` arm, production side, 96 paired deals, 1 worker | 19.4 s (cold) | 7.05 s (memo warmed by one earlier arm) | 2.75x | games identical (plays, points, outcome) to the no-memo run of the same net |
| h2h `np:` arm, 4,096 paired deals, 4 workers, machine shared with 16 label workers | 156 s (operator's L2g run) / 237 s (memo run with a cold memo, same contention as the next row) | 72 s (L2f, memo warmed by the L2g run; 26,137 of 68,007 production decisions served) | 2.2–3.3x | all 8,192 L2g games identical to the operator's L2g results; all 6,974 games of the operator's partial L2f run identical to the memo run's |
| training (`train.py`, 1.8M rows, 30k updates) | 41 s (load 1.3 s) | unchanged | 1.0x | load is 3% of the stage; caching encoded arrays would not pay |
| labeling round throughput, 16 vs 18 workers | not measured | not measured | | needs the whole machine; the operator agent ran 16-worker rounds back to back for the whole pass. The machine is 6 P + 12 E cores; 16 workers already use 10 E cores, so 18 can add at most two E-core shares (a few percent). `pipeline.py --label-workers` is the knob |
| end-to-end rung-2 cycle, 2 label rounds -> train -> h2h | ~800 s serial (280+280 label, 41 train, 156 h2h, launches) | projected, not measured: ~620 s per net in `pipeline.py --lag` (one 288 s round per net, train 41 s and h2h ~75 s overlapped with the next round; 12 label workers instead of 16 label ~17% fewer states per round, 1.12x teacher gives ~5% back), or ~700 s with `--strict` | ~1.3x per net, ~2.6x per round of labeling kept busy | mechanics verified end to end at small scale (2 label workers, 1 h2h worker, 500 updates: round -> land -> train -> agree -> h2h all logged); a full-size run needs the machine |

## What changed and why it is identical

1. **Engine micro-optimizations** (`ladder/csrc/turbo.cpp`, marked FORK-ONLY): the profile of the 40/8
   teacher is 52% single-world leaf rollouts (`single`), 23% multi-fiber `search`, 12% support construction and
   unranking, 4% `Rules::play`, the rest hashing, allocation and the deadline clock. Changes: the per-deal RNG
   salt is hoisted out of the rollout recursion (same value), the rejection sampler's threshold `(-n) % n` comes
   from a table for n < 8 (same arithmetic), the deadline clock read is skipped when the deadline is >= 1e8 s
   (it could never fire), the query key is hashed once instead of twice, sampled worlds are written into a
   per-level reusable buffer instead of a fresh vector, and the rollout's node counters are folded in lazily.
   The policy cache (`std::unordered_map` keyed by 104-byte keys) is switched off for the teacher: measured hit
   rate 55 of 17.2M level-1 queries over 200 games, because a level-1 key includes the modeled hand and the full
   public state, which a tree search never revisits (the frame-level `node_cache` already dedups within a node).
   Answers are deterministic functions of the key (addressed randomness), so the cache only ever saved time.
2. **Net forward** (`ladder/src/net.rs`): the scalar index loops became slice zips that vectorize; every
   per-element addition chain keeps the original order (rows in the same sequence, hidden and output sums over j
   ascending, output loop reordered to j-outer but each output's chain unchanged, no fused multiply-add), so the
   scores are bit-identical. This is the whole rung-3 teacher cost (the net is called ~5,600 times per state
   after dedup; 90% of the profile) and the whole `l2n:` arm cost.
3. **Production-decision memo** (`ladder h2h --prod-cache DIR`, `h2h_fast.py`): production's choice is a
   deterministic function of the wire text, which is a function of (seed, decl, bidder, actor, actor's hand,
   plays) plus fixed protocol constants (40/8, 14 s budget, no voids). The memo is keyed by exactly that tuple
   (77-byte records, one append-only file per process, every process loads every file in the directory at
   start). Production's cost per game is 111 ms, of which the first two decisions are 64% and the first four
   90%; across two different nets the common prefix of a game covers 75% of production's time (measured on the
   L2e/L2f/L2f-256 results, 7k paired games each). Cached decisions carry the time production originally took,
   so `native_decision_us` summaries stay comparable; each game records `native_cached`.
4. **`log2 --stop-at`** (default 280 as before): the pipeline uses 288 (a 40/8 game takes ~0.4 s, cap is 295).

## Pipeline (`pipeline.py`)

Serial cycle today: label (2 x 295 s, 16 workers) -> train (GPU, 1 core) -> h2h (4 workers), with 14 of 18 cores
idle for the last two stages. `pipeline.py` keeps one labeling round running at all times on `--label-workers`
(default 12) cores, trains as soon as a round lands (over everything labeled so far), runs the Rust `agree` check,
and sends every passing net to `h2h_fast.py` on `--h2h-workers` (default 4) cores while labeling continues
(12 + 1 + 4 = 17 processes). Default is `--lag` semantics: the next round launches the moment the previous one
lands, following the newest *ready* net, so the student a round follows lags one net behind (expert iteration
with a one-round-stale actor; a protocol change, not an equivalence). `--strict` restores the runbook's order
(label with k -> train k+1 -> label with k+1) and idles the label cores while training. `--ladder` switches each round's teacher to the 40-world search with the newest ready net as the modeled rung-1
others (`log2 --model NET --play np:NET`, as `log2_launch.py` does for rung 3; the first rounds use `--net`), and
`--h2h-search` adds an `l2n:40:<net>` h2h after each net's `np:` h2h (tag `l2n-40-<name>-k<k>-s<seed>-<deals>`),
both through `h2h_fast.py` with the memo. The `agree` check reads the whole growing val file (not the default
first 20k records). Everything is resumable:
rerun the same command and it continues from the files on disk (parts, growing train/val files, model json,
h2h summaries, `pipeline/<name>/state.json`); events go to `pipeline/<name>/log.jsonl`.

## Tried, did not pay

- **Cross-game engine reuse / bigger policy cache** (`--reuse`, removed): 55 hits in 17.2M level-1 queries and
  4.1 GB RSS per worker at a 3M-entry cap. The cache is now off for the teacher instead.
- **`-mcpu=native`** for the C++ engine: 0% (13.61 vs 13.62 ms per state). **PGO** (clang
  `-fprofile-instr-generate/-use`, Xcode's `llvm-profdata`): 3% (12.73 -> 12.33 ms); not worth a two-step build,
  so the `build_pgo.sh` script and its profiles were removed again. `build.rs` keeps an optional `LADDER_CXXFLAGS`
  hook (default empty = unchanged flags; `-fprofile-instr-generate` also links the profile runtime).
- **Training data cache (.npy)**: load is 1.3 s of a 41 s stage.
- **Dedup of identical teacher states within a round**: opening states are unique per deal (own hand x
  declaration), and the expensive states are the openings; nothing to dedup where it matters.
- **Exact enumeration at trick 6 / cheaper late outer counts**: faster and exact, but changes the teacher's
  label values (support-size denominators instead of k/40), so out of scope for an output-identical pass; the
  numbers in DELTA-EXPECTIMAX section 2 stand if Jason wants that teacher change.
- **walt `parallel` feature for production in h2h**: needs the `rayon` crate (not cached offline) and would not
  raise throughput on a saturated machine; the memo removes most of production's cost instead.

Where the rung-2 cycle's time goes after this pass: labeling (the teacher's rollouts, now 16.4 ms per state
single-process) is 90% of a serial cycle; the next real step for that stage is a teacher change (exact trick-6
enumeration, DELTA-EXPECTIMAX section 2), not more engine tuning. The rung-3 cycle's labeling is the net forward,
which is now ~3 µs per call; an accumulator-style incremental forward would be the next step there.

## How to run

```
# build (28 s; new target dir, old binaries untouched)
cd ladder && CARGO_TARGET_DIR=$PWD/target-s cargo build --release --offline
# identity check against the old binary (byte-identical labels)
ladder/target-g/release/ladder log2 --seed 777000 --games 40 --inner 8 --outer 40 --worker 0 --workers 1 --out /tmp/a.bin
ladder/target-s/release/ladder log2 --seed 777000 --games 40 --inner 8 --outer 40 --worker 0 --workers 1 --out /tmp/b.bin && cmp /tmp/a.bin /tmp/b.bin
# fast h2h (memo in results/prod-cache-s<seed>, 8 workers, rounds until 4,096 deals are paired)
LADDER_BIN=$PWD/ladder/target-s/release/ladder $PY h2h_fast.py --net np:models/X.w --tag np-X-s5000-4096 --voids
# continuous rung-2 cycle (sampled teacher, net-driven states)
LADDER_BIN=$PWD/ladder/target-s/release/ladder $PY pipeline.py --name C1 --net models/X.w \
  --train-base data/labels2-mixD-train.bin --val-base data/labels2-mixD-val.bin --rounds 6 --nets 6
# ladder mode (teacher = 40-world search with the newest net as modeled others, same net drives the states;
# np: and l2n:40 h2h of every net at 4,096 deals; unlimited rounds/nets, stop with Ctrl-C and rerun to resume)
LADDER_BIN=$PWD/ladder/target-s/release/ladder $PY pipeline.py --name LAD1 --ladder --h2h-search \
  --net models/L3c-mix-h128x2.w --train-base data/labels2-all2-train.bin --val-base data/labels2-all2-val.bin \
  --label-workers 12 --h2h-workers 4 --h2h-deals 4096 --h2h-seed 5000 --outer 40 --inner 8
```

## Batched/memoized net inference (two-ring teacher)

Claude Fable 5.1, 2026-10-05 00:00–01:00 CDT, performance pass on the `l2z:40:8` arm (production-style outer search
over 40 worlds, 8-world inner searches, the net replacing the dice at level 0 through the leaf hook). Exploratory tier.
Binary: `ladder/target-q/release/ladder` (built from this tree; `target-p` is an intermediate build without the
prefetch). Old binary for every comparison: `ladder/target-z/release/ladder` (the committed source, 657b08a). All
runs below ran while pipeline LAD4 held 12 cores (12 label workers, 4 h2h workers, training), with at most 4 of my
workers at a time, every process under the 295 s cap; old and new arms ran concurrently so they saw the same load.

**Result: 1.44x on the `l2z:40:8` median decision (912 -> 633 ms), 1.65x on p95, output-identical; not the 10x
asked for.** The 10x is not available from inference engineering under exact semantics: see "Why not 10x" below.

| arm (seed 5000) | deals / hybrid decisions | old median / p95 (µs) | new median / p95 (µs) | factor (median) | identity |
|---|---:|---:|---:|---:|---|
| `l2z:40:8:models/LAD3-k12.w` | 32 / 495 | 911,624 / 10,301,764 | 632,602 / 6,241,502 (4 workers; 619,367 / 6,388,167 with the no-prefetch build at 2 workers) | 1.44x | all 64 games identical play by play (`plays`, points, outcome), same 4/5/23 flips |
| `l2n:160:models/LAD3-k12.w` | 128 / 2,089 | 26,688 / 270,102 | 19,735 / 144,616 | 1.35x | all 256 games identical |
| `l2n:160:models/LAD3-k12.w` | 32 / ~500 | 30,061 / 277,409 | 22,369 / 150,707 | 1.34x | all 64 games identical |
| `np:models/LAD3-k12.w` | 32 / 513 | 6 / 9 | 4 / 5 | 1.5x | all 64 games identical (production side fully memoised) |
| net forward alone, `netbench` 256x256 (LAD3-k12), ns per call, min of 4 interleaved runs | | 5,280 (generic loop) | 4,008 | 1.32x | `agree --dump` identical on 300,000 records (every legal score, byte-compared) |
| net forward alone, 128x128 (LAD1-k2) | | 2,275 | 1,535 | 1.48x | |

Old-binary l2z numbers are higher than the 1.0 s in STATUS.md because the machine was fuller (LAD4 + 4 workers); the
new binary's 2-deal probe on the same deals went 1.76 s -> 1.04 s (1.7x) when only 3 processes of mine ran.

### What changed (all output-identical; every file marked FORK-ONLY at the change)

1. **Hook memo** (`ladder/csrc/turbo.cpp`, `LeafMemo`, used at both hook call sites in `Context::search`). A net
   answer is a deterministic function of the public state the net reads (played, leader, len, t1, t0, trick tiles,
   the three other seats' voids) and the acting hand; history words and the actor's own voids are not read. Open
   addressing, 32-byte entries (3 key words + generation + tile), generation-stamped so a clear is `gen++`; cleared at
   every top-level query (`LADDER_LEAF_MEMO=1`, default; `2` = per game, `0` = off; counters in `walt_metrics` out[6..9]
   and in the h2h worker's final JSON line as `leaf_memo`). Measured on `l2z:40:8`, 2 deals, 30 decisions: 12,093,378
   hook calls (403k per decision), **21.2% hits** (per-game clearing: 21.3%, so the cheaper policy is the default; peak
   0.58M vs 3.09M entries). On `l2n:160` (level-1 policy hook, 9.2k calls per decision): 14.8% hits. The memo is the
   same dedup the frame-level `node_cache` does within a node, extended across nodes and inner worlds. Lever 1 of the
   brief: a 1.27x / 1.17x ceiling, which is why the forward itself had to be made cheaper.
2. **Net forward** (`ladder/src/net.rs`): the three layers are explicit NEON kernels (`mod neon`): a block of 32
   outputs (8 vector registers) is held in registers while the inputs stream past, with the first layer's rows as a
   precomputed "row program" (`row_program`), the middle layer over the nonzero activations only in a chunk-major copy
   of the weights (`wm_t`, built at load), separate multiply and add (no FMA), ReLU as compare-select, software
   prefetch 8 rows ahead (the nonzero-row pattern defeats the hardware prefetcher). Every per-element chain is the
   generic loop's in the same order, so scores are bit-identical (checked on 300k records); the generic loop is kept
   for other widths/architectures and `LADDER_NET_GENERIC=1`. The output layer skips exactly-zero activations: a ReLU
   zero is +0.0, its product a signed zero, and adding a signed zero changes only the sign of an exact-zero score,
   which no comparison distinguishes (no such record occurred in 300k). No heap allocation per call (stack buffers via
   `MaybeUninit`; the first attempt's zeroed 2 x 4 KB buffers cost 3% as memset), and the FFI callback reuses one
   thread-local `Key` (`turbo.rs`, `net_policy_cb`). Lever 3 of the brief.
3. **Lever 2 (batching across worlds) was not built**: after dedup the remaining forwards at one node share the public
   state but not the hand, and the first layer is 25% of the forward (0.98 µs of 4.0 µs per stage timing,
   `LADDER_NB_STAGE=1|2`); batching would at best share weight loads, and the measurements below show the kernel is
   not load-bound in the way batching fixes.
4. **`netbench` subcommand** (`main.rs`): forward cost in isolation on synthetic mid-game states with suit-shaped voids;
   reports ns per call, choice checksum, ReLU nonzero fraction (0.38 for LAD3-k12).

### Why not 10x (measurements)

- Per decision `l2z:40:8` does ~403k hook calls, 79% unique after the memo (320k forwards). At 4.0 µs that is 1.3 s
  of forward per decision single-threaded; everything else in the engine is ~5% (`sample` profile: 89% `Net::scores`,
  1.3% `Context::search`, 3% memo probes).
- The forward is bound by the machine, not by the arithmetic order: **fused multiply-add changes nothing** (5,216 vs
  5,280 ns under the same load; opt-in `LADDER_NET_FMA=1`, 0 of 300,000 choices differ from the exact forward), and
  the middle layer runs at ~8 cycles per 16 vector FP ops, i.e. the 2-pipe FP throughput of an E-core (with 12 LAD4
  workers plus the h2h workers the machine is saturated and these processes land on E-cores; ~98 nonzero rows x 1 KB
  of weights per forward also do not fit the 64 KB L1). Exact f32 has nothing left above a few percent.
- The call count is structural: each level-1 query is a full inner tree over 8 worlds with the net choosing every
  modeled move; transpositions are the 21% the memo already takes.
- What would reach the target (semantic changes, so a flagged arm plus an h2h, not done here): a cheaper leaf net
  (`netbench`: 128x128 = 1.5 µs, 32x1 = 0.24 µs vs 4.0 µs; the forward is ~95% of the decision, so 128x128 gives
  ~2.4x and 32x1 ~10x on its own), fewer inner worlds (`l2z:40:4` halves the calls), or an int8 dot-product middle
  layer (dense 256x256 with `sdot` = 4,096 instructions vs 12.5k vector FP ops now, ~3x on that layer, quantization
  needs calibration and an h2h). Running the arm on P-cores would also roughly halve it, but that is the LAD4
  pipeline's budget.

### Commands

```
cd ladder && CARGO_TARGET_DIR=$PWD/target-q cargo build --release --offline          # 35 s
# forward identity (every legal score of 300k records byte-compared) and forward cost
ladder/target-z/release/ladder agree --net models/LAD3-k12.w --labels data/labels2-all2-val.bin --limit 300000 --dump /tmp/a.jsonl
ladder/target-q/release/ladder agree --net models/LAD3-k12.w --labels data/labels2-all2-val.bin --limit 300000 --dump /tmp/b.jsonl && cmp /tmp/a.jsonl /tmp/b.jsonl
ladder/target-q/release/ladder netbench --net models/LAD3-k12.w --iters 300000       # LADDER_NET_GENERIC=1 for the old loops
# h2h, old vs new (summaries in results/h2h-<tag>/summary.json; games-*.jsonl rows carry `plays` for the diff)
LADDER_BIN=$PWD/ladder/target-z/release/ladder $PY h2h_fast.py --net l2z:40:8:models/LAD3-k12.w --tag l2z-40-8-LAD3k12-s5000-32-z --deals 32 --workers 2
LADDER_BIN=$PWD/ladder/target-q/release/ladder $PY h2h_fast.py --net l2z:40:8:models/LAD3-k12.w --tag l2z-40-8-LAD3k12-s5000-32-q --deals 32 --workers 4
LADDER_BIN=... $PY h2h_fast.py --net l2n:160:models/LAD3-k12.w --tag l2n-160-LAD3k12-s5000-128-{z,q} --deals 128 --workers 1
LADDER_BIN=... $PY h2h_fast.py --net np:models/LAD3-k12.w --tag np-LAD3k12-s5000-32-{z,q} --deals 32 --workers 1 --voids
# identity: for each (deal, side) compare `plays` across the two tags (0 of 64 / 256 / 64 games differ)
```

### Inner worlds as the batch dimension (lead's goal update, 2026-10-05 01:00–02:00 CDT)

The inner ring's world count is what the lead wants to grow (8 is noisy, 160 is the wish). Measured here: the exact
path above at inner 8 / 32 / 160, old vs new, per decision; the scaling of cost with inner worlds; and a separately
flagged wide-and-shallow variant. Harness: `ladder l2bench` (new subcommand, in `main.rs`) replays the 64 recorded
games of `results/h2h-l2z-40-8-LAD3k12-s5000-32-z/games-*.jsonl` and times single L2-player decisions, so settings
whose whole games do not fit a 295 s round can still be measured; each answer is compared with the tile the same arm
played in the recorded game (`recorded`) and, between binaries, with each other. Old binary = the 657b08a engine
(scratch build with only the `l2bench` command added, forward verified byte-identical to `target-z`'s dump); new =
`ladder/target-q/release/ladder`. One worker per binary, old and new concurrently, machine shared with other agents'
runs (LAD4 had stopped; a `target-o` h2h with 4 workers and a training job ran alongside), every process under the cap.

**Exact path, `l2z:40:N:models/LAD3-k12.w`, same decisions for both binaries, chosen tiles identical on every one:**

| inner N | decisions measured | old median / mean / p95 (s) | new median / mean / p95 (s) | factor (median / mean) | hook calls per decision (new) | memo hit rate |
|---:|---|---:|---:|---:|---:|---:|
| 8 | first 2 of each game, 127 | 4.78 / 6.97 / 18.5 | 2.61 / 3.37 / 8.1 | 1.83x / 2.07x | 1.11M | 25.8% |
| 32 | first of each game, 18 | 39.9 / 54.4 / 133 | 18.5 / 26.6 / 55 | 2.15x / 2.05x | 6.95M | 32.9% |
| 160 | 5th decision of each game (openings do not fit the cap for the old binary), 40 | 20.5 / 27.3 / 73.5 | 8.14 / 12.2 / 37.5 | 2.52x / 2.24x | 3.74M | 53.3% |
| 160, openings | new only, 3 (old: none completed within 295 s) | – | 196 / 194 / – | – | 48.6M | 37% |

The gain grows with the inner width because the memo's hit rate does: with more sampled worlds, more worlds share the
acting seat's hand at the same public state (hands come from a finite support), so a larger share of the hook calls are
repeats. Whole-game medians (the h2h table above) are lower than these because late-game decisions are cheap.

**Scaling with inner worlds (new binary, first decision of the same 36 games in every row):**

| arm | inner 8 | inner 32 | inner 160 |
|---|---:|---:|---:|
| `l2z` (full-depth inner search, net replaces the dice) median / mean per decision | 2.82 s / 4.10 s | 25.1 s / 30.9 s | ~196 s (3 samples, other decision set) |
| `l2w:…:1` (inner search stops at the end of the current trick, net value per world; VARIANT) | 0.41 s / 0.68 s | 2.14 s / 2.68 s | 10.8 s / 12.7 s |
| `l2w` hook calls per opening decision | 170k | 807k | 4.37M |
| `l2w` memo hit rate | 17% | 33% | 51% |

Reading: in `l2z` the cost per inner world is superlinear (4x worlds = 8.9x time from 8 to 32: every extra world adds
its own full-depth subtree, and wider nodes split into more branches before the fibers become singletons). In `l2w`
it is close to linear and about 65 ms per inner world at an opening decision (8 -> 32: 5.2x, 32 -> 160: 5.0x, the memo
absorbing the rest). `l2w` at 160 inner worlds costs about what `l2z` at 8 costs on the same decisions (10.8 s vs
2.8 s medians at openings; a whole game is cheaper, see the h2h below).

**Marginal cost of an inner world vs a GEMM row.** With exact f32 the forward is already at the FP-issue floor of the
cores it runs on (section above: FMA changes nothing; per-MAC rate identical for an L1-resident 128x128 net and the
L2-resident 256x256 one, 9.4 vs 11.6 GMAC/s), so batching the per-world forwards of one node into a GEMM cannot lower
the FP work per world; it only removes weight loads, which are not the limit. A GEMM row is 2 FP ops per MAC and that
is what each world already costs. What makes an extra inner world cheap is fewer forwards per world (the depth limit)
and the memo (whose hit rate rises with width), not the kernel. So no cross-world batching was built; the memo covers
the within-node repeats a batch would have shared, and the evidence says the rest would not pay on this machine.

**The `l2w` variant** (`l2w:OUTER:INNER:TRICKS:PATH`; `l2_arm` in `main.rs`, `set_leaf_value_net` in `turbo.rs`,
`hookv`/`leaf_tricks` in `turbo.cpp`; all marked VARIANT). Same as `l2z` (outer search unchanged, net chooses the
modeled others' moves inside the inner search) plus: an inner (level-1) search stops at the first trick boundary after
TRICKS completed tricks past its root, and each world is valued there by the net's declarer-make estimate (the score of
the tile the net would choose for the actor to move, clamped to [0,1]; terminal states keep their exact 0/1). The 0/N
bound pruning stays sound because every fiber value lies in [0,1]. Values are memoised in the same table (level marker
in the key, f32 payload). This changes semantics and is **off unless the arm says `l2w:`**; `l2z` is untouched (the
inner-8 identity checks above were run on the binary that contains the variant).

h2h of the variant vs production (paired mirrored deals, seed 5000, production memo; small samples, timing is the
point): see the rows appended below.
- `l2w:40:32:1:models/LAD3-k12.w`, 32 deals, 1 worker, 3 rounds (672 s wall): **+0.156 [+0.000, +0.344]**, flips 7 for / 2 against / 23 same, win rate 0.578; hybrid decision median 582 ms, p95 5.1 s (production 6.2 ms). For comparison `l2z:40:8` on the same 32 deals: -.031 [-.219, +.156], 4/5/23, median 633 ms. `results/h2h-l2w-40-32-1-LAD3k12-s5000-32/summary.json`.
- `l2w:40:160:1:models/LAD3-k12.w`, 16 deals, 2 workers, 2 rounds (553 s wall): **+0.062 [-0.125, +0.250]**, flips 2 for / 1 against / 13 same; hybrid decision median 961 ms, p95 15.7 s, mean game 31 s. So a 160-world inner ring in this form costs about 1.5x `l2z:40:8` per decision; a 32-world one is cheaper than `l2z:40:8`. Both samples are far too small to rank the arms (CIs include zero); the 4,096-deal read is the lead's call. `results/h2h-l2w-40-160-1-LAD3k12-s5000-16/summary.json`.

Commands (inner-world measurements):
```
# old-engine binary with only the l2bench command added: `git archive 657b08a36 ladder | tar -x -C <scratch>/ladder-old`, add
# walt_metrics/metrics() and the l2bench block, build with CARGO_TARGET_DIR there (not kept in the tree).
cat results/h2h-l2z-40-8-LAD3k12-s5000-32-z/games-*.jsonl > /tmp/games32.jsonl
# per binary, in capped rounds (resumable; one line per decision: deal, side, idx, depth, legal, tile, recorded, us, hook_calls, hook_hits)
ladder/target-q/release/ladder l2bench --games /tmp/games32.jsonl --net l2z:40:8:models/LAD3-k12.w --out /tmp/i8-new-w0.jsonl --max-per-game 2 --seconds 270 --worker 0 --workers 1
ladder/target-q/release/ladder l2bench --games /tmp/games32.jsonl --net l2z:40:32:models/LAD3-k12.w --out /tmp/i32-new-w0.jsonl --max-per-game 1 --seconds 270
ladder/target-q/release/ladder l2bench --games /tmp/games32.jsonl --net l2z:40:160:models/LAD3-k12.w --out /tmp/i160-new-w0.jsonl --max-per-game 1 --skip 4 --seconds 270
ladder/target-q/release/ladder l2bench --games /tmp/games32.jsonl --net l2w:40:160:1:models/LAD3-k12.w --out /tmp/w160-w0.jsonl --max-per-game 1 --seconds 270
# identity between binaries = equal `tile` on every common (deal, side, idx); against the game = tile == recorded (l2z only)
LADDER_BIN=$PWD/ladder/target-q/release/ladder $PY h2h_fast.py --net l2w:40:32:1:models/LAD3-k12.w --tag l2w-40-32-1-LAD3k12-s5000-32 --deals 32 --workers 1
```
