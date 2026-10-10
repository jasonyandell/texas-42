# Fable Rust ladder — STATUS

Claude Fable 5.1 (`claude-fable-5-1`), same session. Authorized by Jason: replace
Walt's inner level-0 computation with a net, judge by paired head-to-head win
rate, use the machine, log what Walt actually computes, destination Rust.
Exploratory tier. Prior experiments untouched.

## What exists

- `walt-fork/`: copy of pinned `walt/walt` (`cb1ef3b2`, identical at HEAD) with
  one added module `solver/net_hook.rs` and two hook lines at the top of
  `Solver::pi` for level 0. Diff against pinned source: +72 lines, nothing
  removed (bins/tests dropped from the copy). Builds in 16 s without `parallel`
  (single-threaded like the phone).
- `ladder/`: Rust bin. `log` plays native self-play and records every unique
  level-0 call per decision; `label` evaluates logged calls at N inner worlds
  (exact production contract: Voidless shuffle, dice others, committed own
  future, `k/N` declarer-make, `best_of`) and the 8-world production choice;
  `bench` times native vs hybrid outer decisions on the same roots; `h2h` plays
  paired mirrored games, hybrid partnership vs native, resumable per worker.
- `train.py` (MLX): embedding-sum first layer over the exact level-0 Key
  encoding (28 tile states × 7 + 19 context = 215 inputs), 1 or 2 hidden
  layers, masked soft-target BCE on `k/N`; exports flat f32 weights the Rust
  `net.rs` forward loads. `h2h.py`: capped parallel workers plus paired summary.

## Measured so far (2026-10-04)

- Native Walt decision (40 outer / 8 inner, single thread): median 2.8 ms, opening
  up to 66 ms; a full hand about 0.1 s. One decision makes 1,188 level-0 calls
  at the median and 10,264 at the opening. 48 self-play games gave 2.17M unique
  level-0 calls.
- Level-0 label cost: 8 worlds 11 µs; 160 worlds 0.9 to 1.8 ms per call. Full
  2.16M-call set labeled at 160 worlds in 215 s wall on 12 capped workers.
- The 8-world production choice equals the 160-world choice on 60% of calls.
- Validation RMSE vs `k/160` (102k calls from 8 held-out games): h32 .179,
  h128 .151 (capped 646k rows); on the full 2.16M rows h256x1 .135, h256x2 .092,
  h512x2 .087. Train and validation losses stay close: underfit, not overfit.
- Paired h2h vs pinned native (same deal, both seatings; + means hybrid won
  where native lost): seed 3000, 512 deals: h32 −.127 [−.174, −.084],
  h64 −.070, h128 −.041 [−.084, +.002], h256x1 −.010 [−.051, +.033],
  ascending-tile inner (no evaluation at all) −.088 [−.135, −.041].
  Fresh seed 4000, 1,024 deals, full-data nets: h256x1 −.009 [−.040, +.022]
  and faster than native (4.0 vs 5.8 ms per outer decision); h256x2 −.015;
  h512x2 −.021 and 3x slower; capped-data h256x1 −.063 [−.095, −.031].
- Control: exact 64-world inner mind (slow, 20 s budget) over 96 deals:
  +.031 [−.052, +.115]. No evidence a sharper inner mind beats production.

- Seed 5000, 4,096 deals (final reads): none 0 flips; ascending −.086; h32 −.076; h64 −.064; h128 −.073; h256x1 −.061; v2 h256x1 −.042; h256x2 −.015 [−.030, +.000]; v2 h128x2 −.014 [−.029, +.001] at 0.9x native time; v2 h256x2 −.021; h512x2 (2,048 deals) −.012, 4x slower. Val RMSE: one layer .17 to .12, two layers .10 to .087.

## Latest (pre-compaction checkpoint, 2026-10-04 evening)

- **Direction from Jason:** one relation `portal(state, n) -> pmake vector over legal plays`, frozen selector (max/min, lowest tile), voids everywhere, performance (paired h2h) is the only criterion, Rust destination, standard literature vocabulary (level-k best response / expert iteration / NNUE-style distillation). Sample count `n0` is a knob, nothing special about 8.
- **Void-aware native control** (`inner_belief 1`, no net) vs production, 4,096 deals seed 5000: **+.018 [+.003, +.033]**. The unified void-aware inner is better than production; it is the teacher now.
- **32-world exact inner control:** partial 358 deals +.028 [−.020, +.075]; round 2 running detached (`results/h2h-hires32-r2.out`, tag `hires32-s5000-4096`); summarize with `h2h.py --summarize-only --tag hires32-s5000-4096 --net hires32 --seed 5000 --deals 4096 --workers 12`.
- **Void-aware data:** `data/callsv-{train,val}.jsonl` (48/8 games, 2.28M/360k unique calls, `--voids`), labels at 160 worlds with `VoidsCounted`: `data/labelsv-{train,val}.bin` (100-byte records, voids at bytes 84..100).
- **v3 encoding (tile row indexed by decl x state x 3-bit void pattern):** val RMSE got worse (h128x2 .151, h256x2 .143 vs v2 .101/.090 on the voidless teacher) and train/val gap opened to .07: the 11k-row embedding fragments data 8x per row. Next: make voids additive channels (separate rows per tile x void bit) instead of multiplicative indexing; h256x1 at 80k updates timed out under the cap.
- **Delta expectimax found:** `/Users/jason/code/texas-42/experiments/walt42x-intake/` (untracked on branch walt-gran, 2026-09-29/30). Code/docs snapshot in `experiments/walt42x-intake-snapshot/` with SNAPSHOT.md explaining `delta`.
- Commits through 7180eaf2. Binary at `ladder/target/release/ladder` is v3-capable; `--net voids` = void-aware native, `--net none`, `--net ascending`, `--net hiresN`.

## Delta expectimax + ladder redirect (2026-10-04 late evening)

See `DELTA-EXPECTIMAX.md`. Engine linked statically (`ladder/csrc/turbo.cpp` + `build.rs`,
`src/turbo.rs`). Findings: delta < 1 is not cost-effective vs more sampled worlds; the compiled
sampler is a 12.6x faster labeler at equal quality; trick 6 should be solved exactly (<= 90
worlds, ~10 µs), trick 5 exact costs 1.8 ms, trick 4 exact 1.6 s. Inner arms vs production: 8
worlds +.020, 8 worlds δ=1/8 +.047 (1,526 deals), split tail +.032 (2,765 deals). Jason stopped
the ablation; current work: rung-2 distillation (teacher `log2` at 160/8, nets `np:` as whole
player, hook for best-response-to-net as rung 3). Binaries: `ladder/target-{b,c,d}/release/ladder`
(d = newest, has the hook); `LADDER_BIN` selects the binary for `h2h.py`.

## Open

- Confirmation at 4,096 deals (seed 5000) for full-h256x1; tiny nets (32/64/128)
  retrained on the full data; outer-vector fidelity metric; accumulator-style
  incremental forward for speed; next rung (outer vector as teacher).

## Checkpoint before compaction (2026-10-04 ~22:15 CDT)

**Running (no subagents alive; I supervise via a 20-minute ScheduleWakeup loop):**
- Pipeline **LAD3** (supervisor `python3 pipeline.py --name LAD3 ...`, exact args in `results/pipeline-LAD3.out` start
  event; log `pipeline/LAD3/log.jsonl`): 160-world search teacher with the newest net inside, 256x256 student,
  continual fine-tuning (`--init-prev --lr 0.0005 --updates 4000`), base = strong-teacher states only
  (`data/labels2-T160-*`), both h2h arms at 4,096 deals via `h2h_fast.py` (production memo). Bounded at 16 nets.
  Rerunning the same command resumes it. Binary `ladder/target-s/release/ladder`.
- Dice-replacement h2h: `h2h_fast.py --net l2z:40:8:models/LAD3-k2.w --tag l2z-40-8-LAD3k2-s5000-256 --deals 256
  --workers 6` (binary `ladder/target-z`, leaf hook: net replaces the uniform random field inside level-1 searches;
  2 s/decision). Output `results/h2h-l2z.out`, summary in `results/h2h-l2z-40-8-LAD3k2-s5000-256/summary.json`.
- Monitor artifact: https://claude.ai/artifact/LUTCtR7EpuG6sRkUpUXH3r . Push = `python3 monitor_collect.py` then
  Artifact write_db set collection `snap` doc `s<t>` from `results/monitor/snap-<t>.json` (ArtifactData tool is gone;
  use the Artifact tool's write_db action).

**Results so far (vs production, 4,096 paired deals unless noted):** teacher ceiling +.077; inner nets near parity;
rung-2/3/4 nets alone plateau −.10 (40-world teacher, any data size, 128 or 256 wide); `l2n:160` (160-world
search + rung-4 net) **+.070 [+.041, +.100]** (1,024 deals, 11.5 ms/decision); fine-tune on strong-teacher states
only −.075; LAD3 k0..k5 alone −.071/−.067/−.063/−.071/−.066/−.063, in 40-world search up to +.025 [+.010, +.041]
(k4). Full tables in `DELTA-EXPECTIMAX.md` (sections 6+, operator sections, lead notes).

**Open:** write the LAD1/LAD2/LAD3 table when LAD3 finishes; the dice-replacement result; next lever if the
whole player flattens at −.065 (student capacity/representation, or regret-weighted loss); git history still
holds the mistakenly committed target dirs (local branch only).

## 2026-10-04 22:46 CDT — LAD3 stopped (plateau), LAD4 launched (350-world teacher)

- LAD3 (160-world search+net teacher, 160-only data, continual fine-tune): nets k0–k12. Alone: −.071 −.067 −.063 −.071 −.066 −.063 −.064 −.060 −.063 −.061 −.068 −.074 (k0..k11); in 40-world search: +.016 +.003 +.015 +.005 +.025 +.002 −.001 −.002 +.012 (k8) … +.005 (k10). Flat since k2. Stopped by me at 22:45 after Jason: "higher quality teacher data will help more since it seems basically plateaued… 350 is better and still pretty fast." Resumable: rerun the start command from `results/pipeline-LAD3.out`.
- Dice-replacement h2h (`l2z:40:8:models/LAD3-k2.w`, production Walt outer, net replaces dice in the inner 40/8 search): 6 rounds, 199 paired deals: **+.045 [−.030, +.116]**, 33 flips for / 24 against / 142 same, 1.0 s median per hybrid decision (production 6 ms). Parity-or-better, under-powered. `results/h2h-l2z-40-8-LAD3k2-s5000-256/summary.json`.
- LAD4: `pipeline.py --name LAD4 --net models/LAD3-k12.w --outer 350 --inner 8 --eps 0.1 --seed-base 25000000 --hidden 256 --hidden2 256 --updates 4000 --eval-every 200 --init-prev --lr 0.0005 --rounds 24 --nets 24 --ladder --h2h-search` (12 label workers, 4 h2h workers, no train_base: 350-world labels only). Log `pipeline/LAD4/log.jsonl`, stdout `results/pipeline-LAD4.out`. Expect ~20k states per 5-min round (LAD3 did ~45k at 160 worlds).

## 2026-10-04 23:36 CDT — LAD4 drift found; restarted with promotion gate

- LAD4 k0–k8 (continual fine-tune at lr 5e-4 on 24k–175k rows of 350-world labels) got steadily *worse*: alone −.069 → −.090, Rust agreement with the 350-world val .636 (init LAD3-k12) → .628 (k7); same ordering on LAD3's val (.639 → .620). Label format verified (per-row N as u16, counts u16; no overflow). So fine-tuning on the small 350-only set degrades the net and, because the newest net is also the teacher's modeled others, degrades the teacher. Cleaner labels did not raise the fixed student's agreement (.639 on 160-world val vs .636 on 350-world val), which points at the student, not label noise.
- Fix: `pipeline.py --gate` (new): a trained net is promoted (status ready → teacher/student/h2h) only if its Rust agreement on the current val ≥ the current teacher's agreement on the same file; else status rejected. k0–k8 marked rejected in `pipeline/LAD4/state.json`; teacher reset to `models/LAD3-k12.w`; restarted at round 10 with `--lr 0.0001 --gate --rounds 40 --nets 40` (otherwise same args). Data from rounds 0–9 kept (175k train rows).

## 2026-10-05 00:05 CDT — Overnight plan (Jason asleep; compaction imminent)

Findings that set the plan (all paired mirrored h2h vs production, seed 5000, dropped-30 protocol):
- Net alone plateaus at −.06 regardless of width (128/256), teacher worlds (40/160/350), or data (24k–8M). Agreement with its own labels is .64 against both 160- and 350-world labels, so label noise is not the cap; the student is. LAD4 with `--gate` (promotion only on val-agreement gain) confirms: k9–k11 promoted by 4th-decimal gains, k12 rejected.
- One ring over the net (search with the net as modeled others): 40 worlds ≈ parity at 2.4 ms/decision (production 5.3 ms); 160 worlds **+.070 [+.041,+.100]** at 11.5 ms. Two rings with the net in the dice slot (`l2z:40:8`): +.045 [−.030,+.116] on 199 deals at ~1 s/decision.
- Jason's goals: we beat Walt only by out-sampling it; the second ring (BR(BR(π))) is where partner modeling lives ("what if my partner wasn't a rando"); Walt's occasional partner-aware play is a sampling artifact of 8 inner worlds; inner should be ~160 if affordable; partner communication is wanted but expected to be minor.

Plan, in order:
1. **Batched/memoized net inference** in the turbo engine (subagent `fable-batch`, building in `ladder/target-b`, results to PERF.md). Inner world count is the batch dimension; report cost at inner 8/32/160; bit-identical at inner 8; wide-and-shallow inner ring (many worlds, current-trick lookahead, net as leaf) as a separately flagged variant.
2. **Inner-ring grid** (128 deals each, 4 workers, capped rounds): inner 8 deep vs 32 deep vs 160 shallow at outer 40, read as a bias/variance sweep on the partner model. Pick the teacher shape from the grid.
3. **Two-ring teacher** (`log2` with the leaf hook: outer N over inner M with the newest gated net at level 0) as LAD5 if the grid says so; keep `--gate`; 350-world one-ring data (LAD4) stays as a base.
4. **Student-side experiment** on the 350-world pile once it passes ~300k rows: wider/deeper or per-tile attention student, value-gap-weighted loss; measure the distill gap (search+net minus net alone) per step as the number to drive down.
5. **Belief-weighted worlds** (flagged variant; a typed move from support to belief, never mixed with the exact-support path): weight each sampled world by Π π_net(observed play | world) over the plays so far, so a partner's *non*-plays become evidence. Diagnostic: newcomer-with-newcomer vs newcomer-with-production-Walt partnerships; the gap is the communication edge, separated from the sampling edge.

Operating defaults while unattended: 20-minute loop (monitor_collect → artifact write_db; LAD4 supervisor restart from the command above if dead; respawn `fable-batch` with its brief + "read PERF.md and git diff" if lost); local commits to this branch only; no push/PR/cloud/installs; caps and 4-worker limit for anything new; LAD4 may be stopped to free cores when the grid needs them.

## 2026-10-05 01:00 CDT — LAD4 stopped (data collection done), batching result, grid launched

- LAD4 stopped at k25 to free cores: 499k train rows / ~45k val rows of 350-world one-ring labels in `data/labels2-LAD4-{train,val}.bin`. Gated nets k17–k25: agreement .647–.649, alone −.059 to −.064, in 40-world search −.004 to +.006. Plateau at every data size. Resumable with the restart command above.
- Batching pass (subagent fable-batch, PERF.md "Batched/memoized net inference"): **1.44x** on `l2z:40:8` (912 → 633 ms median), 1.35x on `l2n:160`, 1.5x on `np:`, all output-identical (64/256/64 games play-by-play, 300k records byte-compared). 10x is not available under exact semantics: a two-ring decision is ~403k leaf-net calls (79% unique after a 21%-hit memo), the forward is 95% of the time and is already at the core's FP throughput. Routes to 10x are semantic: a 32-wide leaf net (~10x), fewer inner worlds, int8 middle layer (~3x on that layer).
- Consequence for the plan: a two-ring teacher with the net at level 0 and 160 inner worlds is out of reach by ~2 orders of magnitude. The cheap way to test "8 inner is too noisy" is the pure turbo two-ring with dice leaves, `l2:OUTER:INNER`, where an inner world costs ~30 ns instead of 4 µs: `l2:40:160` ≈ 100 ms/decision. Grid launched (subagent sonnet-grid): l2:40:8, l2:40:32, l2:40:160, l2:160:32 (l2:160:8 = +.077 known), 1024 deals each, seed 5000, prod cache, binary target-s.

## Inner-ring grid (dice leaves)

Paired mirrored h2h vs production, seed 5000, fixed contract (dropped on, bid 30), 1024 deals each, 8 workers, binary target-s, prod cache. Exploratory tier (rob receipts / probe), no multiplicity correction.

| arm | deals paired | mean paired advantage [95% CI] | hybrid win rate | median hybrid us/decision | median production us/decision | wall s | rounds |
|---|---|---|---|---|---|---|---|
| l2:40:8 | 1024 | +0.008 [-0.024, +0.038] | 0.504 | 3663 | 4313 | 23 | 1 |
| l2:40:32 | 1024 | +0.046 [+0.017, +0.073] | 0.523 | 13943 | 4270 | 91 | 1 |
| l2:40:160 | 1024 | +0.039 [+0.009, +0.069] | 0.520 | 72684 | 4566 | 561 | 2 |
| l2:160:32 | 1024 | +0.096 [+0.067, +0.123] | 0.548 | 93930 | 5542 | 649 | 3 |
| l2:160:8 (reference, prior run) | 1024 | +.077 [+.048, +.105] | n/a | ~21000 | n/a | n/a | n/a |

## 2026-10-05 01:55 CDT — Student plateau broken (Sonnet designers + Opus student)

- **Net alone beats production**: `STU-tokens-b2-d48L2` (tile-token transformer, d48, 2 layers, 4 heads, 49.6k params, mix loss, 1.52M strong-teacher rows) **+.025 [+.010, +.041]** on 4,096 paired deals, ~64 µs/decision at load; b3 (one more chained round) Rust agree .6814, +.021 [+.006, +.036]. Opus: `STU-d-mlp256-r1` (plain 256 MLP, regret loss) −.008 [−.023, +.008] at 6 µs; `STU-s-res512-r1` −.004 at 76 µs. Old recipe LAD4-k17: −.060. Reports: `reports/student-tokens.md`, `reports/student-features.md` (pure feature net .666 agree / .0144 regret trainer-side, Rust unverified: designer's build was denied by the permission classifier), roles designer: regret lives in a 2.4%-of-rows tail (>50-world gap = 46% of mass), trump/off-suit confusions 27%, roles flat, tie rule costs nothing; agreement is the wrong selection metric, regret is better.
- **One-ring search with a parity net inside**: `l2n:160:models/STU-s-res512-r1.w` interim +.138 [+.070, +.206] at 188/512 deals (run continuing).
- Token student merged into MAIN: `ladder/src/stu_tokens.rs`, `net.rs` header-9 dispatch, `train_tok.py`; built in `ladder/target-t`; `agree` reproduces .6814 (b3) and .6478 (LAD4-k17 unchanged).

## Student-side experiments (Opus student agent, 2026-10-05 ~00:30–02:30 CDT)

Exploratory tier (probe records, not receipts). Fixed snapshots so numbers are comparable: `data/STU-snap-LAD4-train.bin` (410,197 rows) and `data/STU-snap-LAD4-val.bin` (37,679 rows), both 350-world. "Strong" data = snap-train + `labels2-LAD3-train` + `labels2-T160-train` (1.44M rows, 160/350 worlds). Regret = mean over val of (v_best − v_pick)/N in make-probability units (v = k for maximizers, −k for minimizers); zero-regret = picked any argmax. H2H = `h2h_fast.py`, paired mirrored vs production, seed 5000, prod cache, `--workers 2`, binary `ladder/target-o`. µs = `ladder netbench` (200k calls, machine loaded, ±30%).

**Diagnosis.** 24% of val rows have an exact tie at the top and 43% a gap ≤ 1% (3 of 350 worlds); the stored choice is the lowest tied tile. Agreement therefore mostly measures the tie rule, and a regret-trained net loses ~.07 agreement while choosing better. The old student was data/generalization-limited, not label-limited: from scratch on 410k rows it overfits after 4–6k updates (val regret .0255 vs k17's .0168); on 1.44M strong rows a 256x256 reaches train regret .0067 vs val .0150. LayerNorm+residual bodies generalize better than plain wide MLPs (mlp 1024x256: .0158). Distillation removes the overfitting: an ensemble of big nets relabels the 8.2M LAD2 states, and the 256x256 student trained on that pile matches the big nets at the old cost.

| variant | params | train rows | trainer agree | Rust agree | regret (Rust) | zero-regret | net alone vs prod [95% CI], 4,096 deals | µs/decision |
|---|---:|---:|---:|---:|---:|---:|---|---:|
| LAD4-k17 (baseline, mix loss) | 453k | 351k | .6486 | .6486 | .0168 | .682 | −.060 [−.076, −.045] | 3.6 |
| STU-ft-regret-all: k17 + regret loss, strong data | 453k | 1.44M | .6156 | .6156 | .0140 | .697 | −.023 [−.039, −.007] | 3.7 |
| STU-s-res256x2-r1: resid 256, 2 blocks, LN, scratch | 651k | 1.44M | .5571 | .5574 | .0140 | .693 | not run | 20.3 |
| STU-s-res512-r1: resid 512, 3 blocks, LN, scratch | 2.36M | 1.44M | .5718 | .5717 | .0126 | .706 | −.004 [−.019, +.012] | 94 |
| STU-d-mlp256-r1: 256x256 distilled from res512 | 453k | 9.6M | .5767 | .5768 | .0127 | .702 | −.008 [−.023, +.008] | 4.0 |
| **STU-e-mlp256**: 256x256 distilled from a 5-net ensemble | 453k | 9.6M | .5798 | .5798 | **.0123** | .704 | **−.004 [−.019, +.011]** | 4.1 |

Also trained, not deployed: mlp 1024x256 scratch .0158; res512 inner128 x3 .0141; mlp 512x256 on ensemble labels from scratch .0131 (worse than the k17-initialized 256, so the pretrained init matters); `--cew 1` CE on the stored choice raised agreement only to .585 at the same regret. Losses fine-tuned from k17 on LAD4 only, 6k updates: mix .0168, tce .0154, soft β50 .0149, soft β150 .0153, regret .0149. The 5-net ensemble (res512, res256x2, res512i128x3, ft-regret-all, mlp1024) scores .0117 trainer-side.

**Distill gap** (`l2n:160`, 1,024 paired deals): k17 inside **+.033 [+.002, +.062]** vs alone −.060, a gap of .093. STU-e-mlp256 inside **+.088 [+.059, +.118]** vs alone −.004, a gap of .092. The gap stays about the same while both ends rise by ~.055. A better student lifts the search that uses it, at the same 4 µs.

Rust ↔ trainer: agreement matches to 4 decimals for every row above; regret from `--dump` matches to 5 decimals.

**Code (flagged, existing defaults unchanged).** `train_stu.py` adds `--arch mlp|res`, `--loss mix|tce|soft|regret|bce`, `--cew`, `--dropout/--edrop`, `--lr-end` cosine, `--select regret|agree`, `--seconds` wall stop, `.npz` checkpoints with `--init X.npz`, `--eval-only --dump-logits`, and `--relabel IN,OUT`, which writes distillation labels: N=10000, counts=round(10000·σ(z)). `ladder/src/resnet.rs` plus small hooks in `net.rs` (header `[4,3,H,inner,blocks,ln]`, `res` field, early branch in `scores`) and `mod resnet` in `main.rs`. `rust_regret.py LABELS DUMP` computes Rust-side regret from `ladder agree --dump`. Ensemble labels are in `data/STU-distill-LAD2-ens5.bin` (8.18M rows), with the single-teacher version in `data/STU-distill-LAD2-res512.bin`.

Commands (PY = mise python, CAP = `../astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir results/<name>-log --`; STRONG = `data/STU-snap-LAD4-train.bin,data/labels2-LAD3-train.bin,data/labels2-T160-train.bin`):
```
# big teacher (2 capped rounds)
CAP PY train_stu.py --train STRONG --val data/STU-snap-LAD4-val.bin --name STU-s-res512 --arch res --hidden 512 --inner 512 --blocks 3 --ln 1 --lr 0.001 --lr-end 0.00005 --updates 40000 --eval-every 4000 --loss regret --rw 20 --tau 0.5 --seconds 230
CAP PY train_stu.py ...same... --name STU-s-res512-r1 --init <copy of STU-s-res512.npz> --lr 0.0008 --lr-end 0.00002 --updates 11000 --seconds 235
# relabel the 8.2M LAD2 states with each teacher, then average counts across teachers (inline numpy; see this session) -> data/STU-distill-LAD2-ens5.bin
CAP PY train_stu.py --train data/STU-snap-LAD4-val.bin --val data/STU-snap-LAD4-val.bin --name x --arch res --hidden 512 --inner 512 --blocks 3 --ln 1 --init models/STU-s-res512-r1.npz --relabel data/labels2-LAD2-train.bin,OUT.bin
# student: k17 init -> res512 distill (STU-d-mlp256, then -r1) -> ensemble distill
CAP PY train_stu.py --train data/STU-distill-LAD2-res512.bin,STRONG --val data/STU-snap-LAD4-val.bin --name STU-d-mlp256 --init models/LAD4-k17.w --lr 0.0005 --lr-end 0.0001 --updates 80000 --eval-every 4000 --loss regret --rw 20 --tau 0.5 --seconds 200
CAP PY train_stu.py ...same... --name STU-d-mlp256-r1 --init <copy of STU-d-mlp256.npz> --lr 0.0003 --lr-end 0.00001 --updates 26000 --eval-every 3000
CAP PY train_stu.py --train data/STU-distill-LAD2-ens5.bin,STRONG --val data/STU-snap-LAD4-val.bin --name STU-e-mlp256 --init models/STU-d-mlp256-r1.npz --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --loss regret --rw 20 --tau 0.5 --seconds 215
# deployed-side checks
CARGO_TARGET_DIR=ladder/target-o cargo build --release   (under CAP)
ladder/target-o/release/ladder agree --net models/STU-e-mlp256.w --labels data/STU-snap-LAD4-val.bin --limit 100000 --dump D.jsonl; PY rust_regret.py data/STU-snap-LAD4-val.bin D.jsonl
LADDER_BIN=ladder/target-o/release/ladder PY h2h_fast.py --net np:models/STU-e-mlp256.w --tag np-STU-e-mlp256-s5000-4096 --deals 4096 --workers 2 --prod-cache results/prod-cache-s5000
```

**Recommendation.** Select and gate on regret, not agreement: the pipeline's `--gate` on agreement would have rejected every improved net here. Use STU-e-mlp256 as the drop-in cheap net. It sits at parity alone, gives +.088 inside `l2n:160`, and costs the same 4 µs as k17. The recipe that works is to train big LayerNorm-residual nets on the strong pile, ensemble them, relabel the 8M LAD2 states, and distill into the deployed shape. The obvious next step is to put the token transformer (`STU-tokens-b2`, +.025 alone at ~64 µs) into the teacher ensemble and distill into the 256 MLP, which may carry its edge down to 4 µs.

## 2026-10-05 02:10 CDT — one-ring with a parity net inside; Opus student report; LAD5 decision

- `l2n:160:models/STU-s-res512-r1.w` final: **+.116 [+.061, +.171]**, 301 paired deals (55 for / 20 against / 226 same), but 693 ms/decision (the 94 µs net inside 160 worlds). `l2n:160:models/STU-e-mlp256.w` (Opus, 1,024 deals): **+.088 [+.059, +.118]** at ~20 ms/decision, vs LAD4-k17 inside the same search +.033 [+.002, +.062]. Dice two-ring 160:32: +.096 at 94 ms.
- Opus student (STATUS "Student-side experiments"): the cap was generalization, not capacity or label noise (train regret .0067 vs val .0150 on a 256 MLP). Fix = LayerNorm-residual bodies + regret loss + distilling an ensemble back into the 256x256 MLP: `STU-e-mlp256` −.004 [−.019, +.011] alone at 4 µs (k17: −.060). Agreement fell (.649 → .580) while regret fell 27%: 24% of val rows are exact ties; agreement is the wrong yardstick.
- Decision: LAD5 teacher = one-ring `l2n:160` with the newest cheap net inside (≈20 ms/decision, improves with the net); the dice 160:32 labeling (tag 160-32, 15.7k states in 3 rounds, parts kept in data/labels2-160-32-r*-w*.bin) is stopped. Student recipe = train_stu.py (regret loss, ensemble distillation incl. the token net); gate on val regret, not agreement. Being wired by opus-student (phase 2).
- 02:25 CDT: `l2n:160:models/STU-tokens-b3-d48L2.w` (token net as modeled others) **+.088 [+.046, +.130]** on 512 deals (85/40/387), ~300 ms/decision. Same strength inside the search as the 4 µs STU-e-mlp256 (+.088 on 1,024 deals), so the cheap MLP is the teacher-inside net; res512 inside (+.116, 301 deals) overlaps both. Net-inside-search summary: k17 +.033, e-mlp256 +.088, tokens-b3 +.088, res512 +.116.

### Student-side experiments, round 2 (after the lead's stakes/overfitting note)

Same protocol as the section above: snapshot val, Rust-side numbers, 4,096 paired deals, binary target-o.

| variant | params | train rows | trainer agree | Rust agree | regret (Rust) | zero-regret | net alone vs prod [95% CI] | µs/decision |
|---|---:|---:|---:|---:|---:|---:|---|---:|
| STU-e-res512: res512-r1 continued on 5-net ensemble labels + strong rows | 2.36M | 9.6M | .5798 | .5798 | .0120 | .710 | +.004 [−.011, +.020] | ~62–94 |
| STU-t-mlp256: 256x256 distilled from {res512, e-res512, ft-regret-all, tokens-b2} | 453k | 9.6M | .5995 | .5994 | .0121 | .706 | **+.007 [−.009, +.022]** | 2.8–4 |
| STU-t-res512: e-res512 continued on the same 4-net labels | 2.36M | 9.6M | .5990 | .5990 | **.0116** | .712 | **+.012 [−.003, +.027]** | 62 |
| STU-e-mlp256-stk10: e-mlp256 + `--stakes 10` | 453k | 9.6M | .5794 | n/r | .0124 | .703 | not run | 4 |

- **Stakes weighting is null.** `--stakes A` sets the row weight to 1 + A·(v_best − v_worst). At A = 10 it scores .0124 against .0123 without it. The regret loss already weights each wrong tile by its gap, so extra row weighting adds nothing.
- **More rows beat regularization.** Dropout 0.3 plus input dropout 0.1 on the 256 MLP underfit, reaching .0173 in one round. Distillation labels are the effective "more rows": res512's validation regret went .0126 → .0120 → .0116 across two relabel rounds. It still overfits its own labels, with train regret .0025.
- **The token net adds diversity.** The tokens-b2 logits came from the Rust forward over all 8.2M LAD2 states via `ladder agree --dump`. They are uncorrelated enough with the residual nets that {res512, tokens} scores .0109 trainer-side against .0126 and .0124 alone.
- **Agreement is still the wrong gate.** Every net in this table is below k17's .649 agreement yet beats k17 by .06–.07 in h2h.

Labels: `data/STU-distill-LAD2-ens4tok.bin` (8.18M rows; members res512, e-res512, ft-regret-all, tokens-b2; mean of σ(logit) × 10000). Token logits: `ladder agree --net models/STU-tokens-b2-d48L2.w --labels <1.37M-row chunk> --limit 100000000 --dump`, six capped chunks, two at a time.

Recommendation, round 2:
- **Cheap net:** STU-t-mlp256, at +.007 alone for about 3 µs.
- **Strongest net alone:** STU-t-res512 at +.012 for 62 µs, or tokens-b2 at +.025 for about 64 µs. These two are statistically tied.
- **The lever that keeps paying is teacher-ensemble distillation, not loss shaping.** Each relabel round with a more diverse ensemble moved every student. The next step is a third round with a second token net, or the res512 student, in the ensemble.

## LAD5 — ladder with the distillation recipe (Opus student agent, launched 2026-10-05 02:46 CDT)

Command (supervisor PID 60585, nohup; stdout `results/pipeline-LAD5.out`, events `pipeline/LAD5/log.jsonl`; resumable by rerunning the same command):
```
LADDER_BIN=$PWD/ladder/target-o/release/ladder nohup $PY pipeline.py --name LAD5 --net models/STU-w-mlp256.w --trainer stu \
  --gate --gate-metric regret --ladder --h2h-search --h2h-search-outer 160 --outer 160 --inner 8 --eps 0.1 \
  --label-workers 8 --h2h-workers 4 --seed-base 45000000 --rounds 40 --nets 40 --hidden 256 --hidden2 256 --enc 4 \
  --train-base data/labels2-LAD4-train.bin --val-base data/labels2-LAD4-val.bin \
  --extra-train data/STU-distill-LAD2-ens3w4.bin@3000000,data/labels2-LAD3-train.bin,data/labels2-T160-train.bin \
  > results/pipeline-LAD5.out 2>&1 &
```
Do not rebuild `ladder/target-o` while LAD5 runs.

New `pipeline.py` flags. Every default keeps the old behaviour, so the LAD1–LAD4 commands mean the same as before.
- `--trainer stu`: the train step runs `train_stu.py` (mlp `--hidden`x`--hidden2`, selection by val regret) on `TR` plus `--extra-train`.
  - It initializes from the newest promoted net, using its `.npz` checkpoint when present and the `.w` otherwise. This continuity across rounds replaces an explicit `--updates-per-round`.
  - `--stu-args` holds the optimization arguments. The default is the tested recipe: `--loss regret --rw 20 --tau 0.5 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 200`. The `--seconds` wall stop keeps each call inside one 295 s capped round, measured at 120–215 s on 9.6M rows.
  - Readiness: the Rust agreement and regret from `ladder agree --dump` plus `rust_regret.py` must match the trainer within .01 and .001. The check runs only when the val file has not grown since the trainer loaded it.
- `--extra-train`: fixed comma-separated label files appended on every train call. `path@K` takes a fixed-seed random K rows of that file.
- `--gate-metric regret` (with `--gate`, stu trainer): a net is promoted iff its Rust regret on the current val file is ≤ the current teacher net's regret on the same file. The `net` event logs both agreement and regret for both nets. `--gate-metric agree` is the old rule.
- `--h2h-search-outer N`: the outer world count of the search h2h arm (`l2n:N:<net>`). The default, 40, is the old arm.

Choices made for LAD5:
- **Start net:** STU-w-mlp256, not STU-e-mlp256. It is the same recipe distilled from the better 3-member ensemble {res512, tokens-b2, scale-W4}, which scores .0098 trainer-side. It is better on every measure: regret .0119 vs .0123, agreement .631 vs .580, alone **+.010 [−.006, +.025]** vs −.004.
- **Warm pile:** LAD4 350-world train/val as the base, so the gate's val starts at 45,850 rows. LAD3 and T160 come in as fixed extras, plus 3M rows of the ensemble distillation pile. The 3M subsample keeps new LAD5 rows at roughly 1:8 rather than 1:20. It cost .0001 regret against the full 8.2M (.0120 vs .0119).
- **Regularization from the scale recipe was tested and left out.** Weight decay .3 with BCE weight 3 gave .0135 against .0119 when fine-tuning the distilled 256. Weight decay .05 alone gave .0119, no change. Label smoothing does not apply to the regret loss.
- **Gate caveat:** STU-w-mlp256's checkpoint was selected on the LAD4-val snapshot, which is a prefix of LAD5's starting val file. The first gates are therefore slightly biased toward the starting teacher.
- **Step 4, per-promotion ensemble relabeling, is not wired in. This is a follow-up.**
  - Relabeling the 8.2M pile costs about 50 s of GPU per residual or MLP teacher. The token-net member costs about 15 CPU-minutes through `ladder agree --dump` in six capped chunks.
  - Successive promoted 256 MLPs are too similar to add ensemble diversity.
  - The manual path is `train_stu.py --relabel IN,OUT` per teacher (`--arch res` now also reads residual `.w` files), then `build_distill.py OUT.bin A.bin B.bin dump:tok-0.jsonl,...`.

First LAD5 events:
- Round 0 labeled 65,744 states in 289 s: 57,455 train and 8,289 val.
- **LAD5-k0** trained in 185 s on 4.58M rows. Trainer and Rust agree exactly on agreement (.6293); regret is .01179 trainer-side and .01180 Rust-side.
- The gate compared regret .01180 against the teacher STU-w-mlp256's .01207 on the same 54,139-row val, and promoted k0.
- **LAD5-k0 alone: +.017 [+.002, +.032]** on 4,096 paired deals. This is the first cheap 4 µs net whose CI clears production.
- The `l2n:160` arm for k0 started at about 02:54 CDT. Round 1 of labeling is running with k0 as student and as the net inside.

### How to refresh the teacher ensemble when a new architecture arrives

The distillation pile is the 8,179,022 LAD2 game states (`data/labels2-LAD2-train.bin`), relabeled with the mean make-probability of several teachers. It currently lives in `data/STU-distill-LAD2-ens3w4.bin`, built from res512, tokens-b2 and scale-W4. Refresh the pile as follows (PY = mise python, CAP = `../astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir <new dir> --`).

1. **Check that the newcomer adds diversity before spending the relabel cost.** Dump its val logits, then score every subset of members against the snapshot val. Keep a member only if the best subset that includes it beats the current best (.0098). In the earlier rounds, members with a different inductive bias helped most (tokens, residual LN). Successive MLPs from the same lineage did not help.
   - **Nets that `train_stu.py` can load** (mlp `.w`/`.npz`, residual `.w`/`.npz`): `CAP PY train_stu.py --train data/STU-snap-LAD4-val.bin --val data/STU-snap-LAD4-val.bin --name x --eval-only --dump-logits z-NEW.npy <arch flags> --init NET`.
   - **Any net `Net::load` reads** (e.g. tokens): `ladder agree --net NET --labels data/STU-snap-LAD4-val.bin --limit 100000 --dump d.jsonl`. Convert the dump to a 28-column logit array (zeros on illegal tiles), then average σ(logit) across members and score with `train_stu.metrics`.
2. **Relabel the 8.2M states with the new member.** Choose the route by what can run the net.
   - **On the GPU, about 50 s per teacher,** for mlp or residual nets, one capped call: `CAP PY train_stu.py --train data/STU-snap-LAD4-val.bin --val data/STU-snap-LAD4-val.bin --name x <arch flags> --init NET --relabel data/labels2-LAD2-train.bin,OUT.bin`. For residual `.w` files the arch flags must match the header, e.g. `--arch res --hidden 512 --inner 512 --blocks 2 --ln 1` for W4. OUT.bin is about 820 MB.
   - **Through the Rust forward,** for anything only `Net::load` can run (tokens, header 9).
     - Split the pile into 1.37M-row chunks, with 100-byte records: `dd if=data/labels2-LAD2-train.bin of=cI.bin bs=100 skip=$((I*1370000)) count=1370000`.
     - Dump each chunk with `CAP ladder agree --net NET --labels cI.bin --limit 100000000 --dump tok-I.jsonl`.
     - The token net (~64 µs) takes about 220 s per chunk, so six chunks run two at a time take about 11 minutes of wall time on 2 cores. The JSON dumps total about 1 GB, and the scratchpad is fine for them.
   - **The current members' relabels:** `data/STU-distill-LAD2-res512.bin` is ready to reuse. The W4 relabel and the token dumps are in this session's scratchpad (`dist-w4.bin`, `lad2/tok-[0-5].jsonl`). Regenerate them with the commands above if the scratchpad is gone; W4 takes about 50 s and tokens about 11 minutes.
3. **Average the members.** `CAP PY build_distill.py data/STU-distill-LAD2-<tag>.bin A.bin B.bin dump:tok-0.jsonl,tok-1.jsonl,...,tok-5.jsonl`. The first `.bin` supplies the records. Each argument is one member, with equal weights. Building takes about 100 s, most of it parsing the JSON dumps.
4. **Distill and check.**
   - Train the cheap student from the current best 256: `CAP PY train_stu.py --train data/STU-distill-LAD2-<tag>.bin@3000000,data/labels2-LAD4-train.bin,data/labels2-LAD3-train.bin,data/labels2-T160-train.bin --val data/STU-snap-LAD4-val.bin --name STU-<tag>-mlp256 --init <best .npz> --loss regret --rw 20 --tau 0.5 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 200`. This takes about 2–3.5 minutes on the GPU.
   - Then check it Rust-side with `ladder agree --dump` and `rust_regret.py`, and run the `np:` h2h (about 1 minute with 4 workers and the prod cache).
   - Expected gain per refresh: the student lands about .001–.002 regret above the ensemble's trainer-side number. Every distillation in this session lifted the 256 by .0003–.0005 regret, worth roughly +.005 alone in h2h.
5. **Hand it to LAD5.**
   - **Quickest:** point a restarted pipeline's `--extra-train` at the new pile; rerunning the same command resumes from state.
   - **Without stopping LAD5:** train the student by hand as above. A pipeline-trained net only becomes the teacher by passing the regret gate on LAD5's val, so drop the hand-trained net in through a restart with `--net`, or let LAD5's next train call `--init` from it by restarting.
   - Never rebuild `ladder/target-o` while LAD5 uses it.

Total cost per refresh with one new GPU-runnable member: about 10 minutes, made up of the logit check, a 50 s relabel, a 2 min build and a 3 min distill and h2h. With a Rust-only member, add about 11 minutes on 2 cores.

## 2026-10-05 03:05 CDT — second-seed replicates (seed 6000, 2,048 paired deals, fresh production memo)

| net alone | seed 5000 (4,096 deals) | seed 6000 (2,048 deals) | µs |
|---|---|---|---|
| LAD4-k17 (old recipe) | −.060 [−.075, −.044] | −.061 [−.083, −.038] | 3 |
| STU-e-mlp256 (distilled 256 MLP) | −.004 [−.019, +.011] | +.009 [−.012, +.031] | 3 |
| STU-tokens-b3-d48L2 | +.021 [+.006, +.036] | +.019 [−.003, +.041] | 40 |
| STU-scale-W4-res512ln-40k | +.029 [+.014, +.044] | **+.031 [+.010, +.052]** | 55 |

The ordering and magnitudes replicate on an independent seed; W4's interval clears zero on both seeds, tokens-b3 is positive on both with the smaller run's interval touching zero. LAD5 (regret-gated ladder from STU-w-mlp256): k0 +.017 [+.002, +.032] alone; k1, k2 promoted on regret (.01185 / .01183 vs teacher .01196 / .01188); k0's l2n:160 arm running.
- 03:50 CDT LAD5: k0–k10 all promoted on regret (.01180 → .01147). Alone: k0 +.017, k1 +.018, k2 +.020, k3 +.022, k4 +.024 [+.009, +.039] (each 4,096 deals, 4 µs). Inside l2n:160: k0 +.102 [+.088, +.117], k1 +.098, k2 +.090, k3 +.102 (vs STU-e-mlp256 +.088). Monotone so far; ~5.0M train rows; round 12 labeling.
- 04:30 CDT LAD5: k11–k14 promoted (regret → .01125 at k14), k15–k18 rejected by the gate (.01134–.01135 vs teacher .01129–.01134), k19 promoted (.01131). Alone k5–k8: +.017/+.018/+.020/+.019 (k4's +.024 was the high). Inside l2n:160 k4–k7: +.103/+.100/+.094/+.110. Reading: the 256-MLP student saturates around regret .0113, alone ≈ +.02, in-search ≈ +.10 after ~14 rungs; the gate now rejects about half the candidates. Next lever is the student/ensemble (STATUS "How to refresh the teacher ensemble"), not more rungs.
- 05:10 CDT LAD5: k20–k27 all promoted, regret creeping down to .01117 (k27, new low). Alone k9–k11: +.017/+.020/+.021; inside l2n:160 k8–k10: +.100/+.102/+.104. Plateau at alone ≈+.02, in-search ≈+.10 holds; 5.76M train rows; round 29. opus-refresh (round 3) has produced STU-r3-dA/dA2 member nets, no report yet.

## Student-side experiments, round 3 (ensemble refresh; Opus student agent, 2026-10-05 04:28–05:30 CDT)

Exploratory tier (probe records, not receipts). Fixed snapshots so every number is comparable: `data/STU-r3-snap-LAD5-val.bin` (185,306 rows = LAD4-val's 45,850 350-world rows + 139,456 LAD5 160-world rows; copied from `labels2-LAD5-val.bin` at 04:29) and `data/STU-r3-snap-LAD5-train.bin` (1,472,793 rows). Regret = Rust-side (`ladder agree --dump` + `rust_regret.py`), binary `ladder/target-t`. H2H = `h2h_fast.py` np alone, paired mirrored vs production, prod cache, `--workers 2`. "Paired vs X" = per-deal difference of the two arms' outcomes on the same deals (production is deterministic, so this is a direct A-vs-B comparison).

| net | params | regret, all val | regret, LAD4-val part (350w) | regret, LAD5 part (160w) | Rust agree | alone s5000, 4,096 deals | µs (netbench) |
|---|---:|---:|---:|---:|---:|---|---:|
| LAD5-k14 (reference) | 453k | .01133 | .01111 | .01140 | .6219 | +.025 [+.011, +.041] | 4.4 |
| LAD5-k19 | 453k | .01130 | .01102 | .01139 | .6219 | +.026 [+.012, +.042] | 4 |
| LAD5-k27 (newest at 05:20) | 453k | .01113 | .01086 | .01123 | .6227 | +.033 [+.017, +.047] | 4 |
| STU-scale-W4 (old member) | 1.83M | .01081 | .01071 | .01084 | .6877 | +.029 (round 2) | 55 |
| STU-tokens-b3 (old member) | 50k | .01192 | .01174 | .01198 | .6763 | +.021 (round 2) | 60 |
| **STU-r3-W4c-best**: W4 +40k updates on the LAD5 pile | 1.83M | **.01047** | .01047 | .01047 | .6933 | +.020 [+.005, +.035] | 76 |
| **STU-r3-tok-b8**: tokens-b3 + 5 chained rounds on LAD5 rows | 50k | **.01062** | .01064 | .01062 | .6943 | **+.030 [+.014, +.045]** | 64 |
| ensemble W4c + tok-b8 (mean σ) | — | **.00947** | .00949 | .00946 | — | — | — |
| old-style ensemble W4 + tokens-b3 | — | .01030 | .01008 | .01037 | — | — | — |
| STU-r3-dA: 256 from k14, r3 pile @3M + LAD5/LAD3/T160 (the LAD5 recipe, new pile) | 453k | .01121 | — | — | .645 | — | 4 |
| STU-r3-dA2: dA + 1 round | 453k | .01119 | .01104 | .01123 | .6465 | +.018 [+.003, +.033] | 4 |
| STU-r3-dE: same as dA with the OLD pile (ens3w4 @3M) | 453k | .01134 | .01112 | .01141 | — | — | 4 |
| STU-r3-dF: r3 pile @1M | 453k | .01107 | .01098 | .01110 | .6342 | +.027 [+.012, +.042] | 4 |
| STU-r3-dC: r3 pile, all 8.2M | 453k | .01137 | — | — | .651 | — | 4 |
| STU-r3-dBp: ensemble labels on the LAD5 train states + hard rows | 453k | .01123 | .01116 | .01125 | — | — | 4 |
| **STU-r3-dG: 256 from k14, hard rows only (LAD5 snap + LAD3 + T160), no distillation pile** | 453k | **.01079** | .01084 | .01077 | .6006 | +.026 [+.010, +.041]; s6000 2,048: +.034 [+.013, +.056] | 4.1 |
| STU-r3-dS2: 256 from scratch, r3 @3M + hard rows, 2 rounds | 453k | .01190 | — | — | .638 | — | 4 |

Findings:
- **The members improved a lot on LAD5 rows.** Five chained token rounds took tokens-b3 from .01192 to .01062 (.01129, .01094, .01082, .01070, .01062: flattening, the last round's selection was at its first eval). W4 continued 40k → 80k updates on the LAD5 pile: .01081 → .01047, flat after 60k. The d48 L2 token net had not flattened when the d64/L3 question came up, so no new token shape was trained.
- **Diversity check (all subsets of 7 dumps on the snapshot val):** the best ensemble is W4c + tok-b8 at .00947. LAD5-k19 does not help (.00949 with it), nor do t-res512, w-mlp256, W4 or tokens-b3. So the refreshed pile `data/STU-distill-LAD2-r3.bin` (8.18M LAD2 states, mean of W4c and tok-b8) has those two members only. `data/STU-distill-LAD5snap-r3.bin` is the same ensemble on the LAD5 train states.
- **Distillation into the 256 MLP no longer pays; it costs.** On the same init (k14) and recipe, val regret rises with the share of distillation rows: none .01079 (dG), r3 @1M .01107, @3M .01121, all 8.2M .01137. The old pile (@3M, today's LAD5 `--extra-train`) gives .01134. Ensemble labels on the LAD5 states themselves also cost (.01123). The pattern holds on the independent 350-world LAD4-val part (dG .01084, dA2 .01104, dE .01112, k14 .01111), so it is not only fitting the LAD5 teacher's quirks. Reading: with ~1M in-distribution LAD5 rows, a 453k-parameter student is better spending its capacity on them than on LAD2-state soft labels; the ensemble (.0095) sits .0013 below the best 256 and the 256 cannot absorb it.
- **Better regret did not become better play.** dG is the lowest-regret cheap net (.01079 vs k14 .01133, k27 .01113), but alone it ties: paired vs k14 +.0005 [−.013, +.014] (s5000) and +.004 [−.015, +.023] (s6000, 2,048 deals); paired vs k27 −.007 [−.020, +.007]. On the LAD4-val part k27 equals dG (.01086 vs .01084); dG's lead is in the LAD5-teacher rows. W4c also shows the decoupling: its regret fell .0003 while its alone score fell from +.029 to +.020 (within noise). At this level regret differences of ~.0005 are below what 4,096-deal h2h resolves.
- From scratch is worse: two rounds reach .0119 against .0108 from k14.

**No LAD5 handoff.** No round-3 256 MLP beats LAD5's best on both regret and h2h: dG wins on regret but ties k14/k19 and trails k27 in paired h2h. Two cheap options the lead may weigh (not done): (1) drop the distillation `--extra-train` pile from LAD5's train call (dG's recipe: LAD5 rows + LAD3 + T160), which lowered val regret by .0005 at no h2h cost; (2) the refreshed members are the strongest single nets so far (tok-b8 +.030 alone at 64 µs) and a better net-inside candidate for an `l2n:160` arm than the 256.

Code (flagged, defaults unchanged): `train_tok.py` gains `--select agree|regret` (checkpoint criterion; default agree as before), `--prefix` (output name prefix; default `STU-tokens-`), and `--relabel IN,OUT` (MLX GPU forward in 500k-row chunks: 8.18M states in 31 s, replacing the 11-minute 2-core Rust dump route; MLX vs Rust on the 185k val: max |Δp| .020, choice agreement .9976, fine for teacher labels, not for deployed numbers).

Commands (PY = mise python, CAP = `../astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir results/r3-<x>-log --`, BASE = `data/STU-r3-snap-LAD5-train.bin,data/labels2-LAD3-train.bin,data/labels2-T160-train.bin`, VAL = `data/STU-r3-snap-LAD5-val.bin`):
```
# token chain (b4..b8: --init previous .npz; lr .0005/.0004/.0003/.0002/.00015; seeds 13/17/19/23/29)
CAP PY train_tok.py --train data/STU-r3-snap-LAD5-train.bin data/labels2-LAD3-train.bin data/labels2-T160-train.bin --val VAL --name b4 --prefix STU-r3-tok- --init models/STU-tokens-b3-d48L2.npz --d 48 --layers 2 --heads 4 --dff 96 --lr 0.0005 --warm 200 --seed 13 --updates 15000 --eval-every 2500 --select regret --seconds 262
# W4 continuation (3 rounds; state = scratchpad cache/STU-scale-W4-res512ln-40k.state.npz copied to cache/STU-r3-W4c.state.npz)
CAP PY train_scale.py --train BASE,data/labels2-LAD2-train.bin@4000000,data/labels2-LAD1-train.bin@2000000 --val VAL --name STU-r3-W4c --resume --arch res --hidden 512 --inner 512 --blocks 2 --ln 1 --total 80000 --warmup 1500 --lr 0.0003 --lr-end 0.00001 --wd 0.1 --bw 3 --eps 0.1 --batch 2048 --eval-every 4000 --seconds 205
# relabel + build
CAP PY train_stu.py --train data/STU-snap-LAD4-val.bin --val data/STU-snap-LAD4-val.bin --name x --arch res --hidden 512 --inner 512 --blocks 2 --ln 1 --init models/STU-r3-W4c-best.w --relabel data/labels2-LAD2-train.bin,dist-W4c.bin
CAP PY train_tok.py --train data/STU-snap-LAD4-val.bin --val data/STU-snap-LAD4-val.bin --name x --prefix STU-r3-tmp- --d 48 --layers 2 --heads 4 --dff 96 --init models/STU-r3-tok-b8.npz --relabel data/labels2-LAD2-train.bin,dist-tokb8.bin
CAP PY build_distill.py data/STU-distill-LAD2-r3.bin dist-W4c.bin dist-tokb8.bin
# best 256 (dG); dA/dE/dF/dC differ only in the distillation file prefixed to --train
CAP PY train_stu.py --train BASE --name STU-r3-dG --init models/LAD5-k14.w --loss regret --rw 20 --tau 0.5 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 210 --val VAL
# checks
ladder/target-t/release/ladder agree --net models/STU-r3-dG.w --labels VAL --limit 1000000 --dump d.jsonl; PY rust_regret.py VAL d.jsonl
LADDER_BIN=$PWD/ladder/target-t/release/ladder PY h2h_fast.py --net np:models/STU-r3-dG.w --tag np-STU-r3-dG-s5000-4096 --deals 4096 --workers 2 --prod-cache results/prod-cache-s5000
```
Scratch files (session scratchpad `r3/`): relabels `dist-*.bin`, `dist5-*.bin`, val dumps `d-*.jsonl`, the subset scorer `div.py`, the val split scorer `split.py` and the paired h2h script `pair.py`. Rust vs trainer for the 256s checked: dG agree .6006 vs .6008, regret .01079 vs .01078; dF .6342 vs .6342, .01107 vs .01107.
- 05:52 CDT LAD5: k28–k36: 6 promoted / 3 rejected, regret .01100 at k36 (new low). Alone k12/k13/k14/k19: +.026/+.027/+.025/+.026; inside l2n:160 k12–k14: +.109/+.098/+.098. 6.16M rows; 4 nets from the 40-net budget. Plan at "done": continue LAD5 (same command, --nets 80 --rounds 80) with the distillation pile removed from --extra-train (round-3 finding: it now costs .0005 regret), LAD3+T160 kept.
- 06:35 CDT LAD5 reached its 40-net budget (k37 .01097 ready, k38 rejected, k39 .01091 ready; alone k20–k25: +.034/+.028/+.028/+.028/+.027/+.033; inside l2n:160 k20–k23: +.098/+.099/+.100/+.103). Supervisor was draining the h2h queue with labeling stopped, so it was stopped and relaunched as a continuation (same command, `--rounds 80 --nets 80`, `--extra-train data/labels2-LAD3-train.bin,data/labels2-T160-train.bin` i.e. the STU-distill pile removed per round 3). Resumed from state.json at round 40 with LAD5-k39 as teacher/student; the k24 l2n arm in flight at the stop may be missing from the log (rerunnable by tag).
- 07:20 CDT LAD5 continuation: dropping the distillation pile cut regret from .01091 (k39) to .01032 (k40) on the same val file in one rung, a real gain of .0006; k41–k46 then flat at .01025–.01029 with the gate rejecting half. Alone k26–k31: +.029/+.032/+.031/+.032. The per-rung l2n:160 arm (≈6 min each) was backlogging the h2h queue, so the supervisor was restarted (PID 54418) without --h2h-search and the queued l2n arms dropped; in-search reads continue by hand every few rungs. Alone so far after the catch-up: k32 +0.033, k34 +0.033, k35 +0.035, k36 +0.035.
- 08:00 CDT LAD5: the no-pile gain shows in play. Alone: k36 +.035, k37 +.034, k39 +.032, then k40 +.044, k41 +.036, k44 +.040, k46 +.045, k49 +.046, k50 +.047, k51 +.050, **k52 +.054 [+.039, +.068]** (4,096 deals, 4 µs). Val regret flat at .0103 while h2h climbs: the student-driven state distribution moves with the net, so val regret is not comparable across rungs; h2h is the arbiter. k47/k48/k53 rejected; 4.08M rows; round 57 of 80.
- 08:42 CDT LAD5: k56–k62 alone +.054/+.050/+.048/+.052/+.051/+.045 (k55, k57 rejected; k62 regret .01016, lowest yet). Alone has levelled at ≈+.05 since k51. In-search read of k52 (l2n:160, 4,096 deals): **+.104 [+.089, +.118]** (739 for / 314 against / 3,043 same), same as the pile-era nets. Round 66 of 80; 4.57M rows. k62 in-search read launched.
- 09:22 CDT LAD5: k63–k70 alone +.047/+.048/+.050/+.042/+.048/+.050 (k68, k69 rejected); regret .01008 at k70. Plateau ≈+.05 alone holds. In-search read of k62: +.094 [+.079, +.108] (689/304/3,103), same band as k52's +.104. Round 74 of 80; 5.02M rows.
- 10:05 CDT LAD5 **done**: 80 rounds, 78 nets (57 promoted / 21 rejected), last k77 (regret .01000, alone +.046), best alone **k73 +.056 [+.041, +.071]** at 4 µs. Supervisor exited; nothing running. Night's tables written to DELTA-EXPECTIMAX.md ("Overnight 2026-10-05").
- 2026-10-05 14:40 CDT: clean replicate of the headline net on an independent seed: `np:models/LAD5-k73.w`, seed 6000, 4,096 paired deals, fresh production memo: **+.058 [+.043, +.073]**, win rate .529 (596 for / 358 against / 3,142 same). Seed 5000 was +.056 [+.041, +.071] (best of ~57 rungs, selection-biased); seed 6000 is a single pre-registered read and removes that bias. `results/h2h-np-LAD5-k73-s6000-4096/summary.json`.
- 2026-10-05: **published on Hugging Face (public, CC0 1.0)**: models https://huggingface.co/jasonyandell/texas-42-walt-tiny-net-ladder (LAD5-k73, STU-scale-W4-res512ln-40k, STU-tokens-b3-d48L2, STU-r3-tok-b8 + cards + weight spec) and dataset https://huggingface.co/datasets/jasonyandell/texas-42-walt-ladder-labels (labels2-{T160,LAD3,LAD4,LAD5}-{train,val}.bin, 6.7M records, SHA256SUMS). Both Cargo manifests set to CC0-1.0 per Jason ("public all around").

## Belief head and belief-weighted worlds (2026-10-06, Opus agent; exploratory tier)

Binary: `ladder/target-b2` (new code: `ladder/src/belief.rs`, `belief-log`/`belief-eval` subcommands, `l2b:` arm, world-weight hook `hookw` in `csrc/turbo.cpp`, off unless an `l2b:` arm installs it). Trainer: `train_belief.py`.

### Step 1: belief head (supervised)

Data: `ladder belief-log --play np:models/LAD5-k73.w --eps 0.1` (LAD5-k73 in all four seats; 128-byte records = REC + true remaining hands of the 3 relative seats + deal seed + eps flag + trick). Train `data/belief-k73-train.bin` 2,119,580 states (seed 70000000, 130k games, 8 workers, ~1 s); val `data/belief-k73-val.bin` 211,660 states (seed 80000000, 13k games).
Net `models/BEL-k73-a.w`: encoding-4 embedding sum, 256x256 MLP trunk initialised from LAD5-k73, 84 logits (28 tiles x 3 relative seats), softmax over the seats that can hold the tile (not void, still holding a tile); masked CE; Adam 1e-3 cosine to 1e-5, 40,000 x 2048, 230 s (one capped round); best val ll .9466 at 30k.
Rust `belief-eval` on all val (2.66M unseen-tile instances); uniform = exact marginal of the uniform void- and count-consistent support (what turbo samples); naive = 1/#allowed seats. Log loss in nats per unseen tile:

| trick | states | tiles | ll belief | ll uniform | ll naive | gain vs uniform | acc belief | acc uniform | acc naive |
|---|---|---|---|---|---|---|---|---|---|
| 0 | 35,424 | 699,151 | 1.0236 | 1.0869 | 1.0892 | +.063 | .426 | .355 | .344 |
| 1 | 38,521 | 641,964 | .9861 | 1.0530 | 1.0606 | +.067 | .455 | .384 | .358 |
| 2 | 39,832 | 542,752 | .9541 | 1.0169 | 1.0314 | +.063 | .479 | .412 | .377 |
| 3 | 37,627 | 399,784 | .8953 | .9684 | .9944 | +.073 | .518 | .449 | .400 |
| 4 | 33,912 | 258,696 | .8165 | .8918 | .9431 | +.075 | .568 | .501 | .434 |
| 5 | 26,344 | 121,422 | .7099 | .7632 | .8806 | +.053 | .635 | .572 | .486 |
| all | 211,660 | 2,663,769 | .9467 | 1.0130 | 1.0326 | +.066 | .481 | .412 | .378 |

The gain does not grow with plays seen: it is already +.063 at trick 0 (the deal rule makes the bidder the seat with the most trumps, which the sampler ignores) and peaks at tricks 3-4. Calibration (belief, all allowed (tile, seat) pairs with >1 allowed seat; predicted mean vs observed frequency): .054/.060, .156/.167, .253/.262, .345/.347, .440/.430, .542/.523, .645/.623, .746/.722, .848/.827, .951/.944 (slightly overconfident above .5).
Parity: Rust `belief-eval --dump` vs `train_belief.py --eval-only --device cpu` on 20k val rows: same log loss to 1e-9, max |logit diff| 2.4e-6.

```
B=ladder/target-b2/release/ladder; CAP="python3 ../astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir"
$CAP results/belief-log-k73-train-w0$w-log -- $B belief-log --seed 70000000 --games 130000 --worker $w --workers 8 --eps 0.1 --play np:models/LAD5-k73.w --out data/belief-k73-train-w0$w.bin   # w = 0..7, then cat
$CAP results/belief-log-k73-val-log -- $B belief-log --seed 80000000 --games 13000 --eps 0.1 --play np:models/LAD5-k73.w --out data/belief-k73-val.bin
$CAP results/train-BEL-k73-a-r0-log -- $PY train_belief.py --train data/belief-k73-train.bin --val data/belief-k73-val.bin --val-limit 60000 --name BEL-k73-a --updates 40000 --eval-every 2000 --init-trunk models/LAD5-k73.w --seconds 230
$CAP results/belief-eval-BEL-k73-a-log -- $B belief-eval --net models/BEL-k73-a.w --data data/belief-k73-val.bin
```

### Step 1b: the belief must model production opponents (and a leak found and fixed)

BEL-k73-a on the states an h2h hybrid actually faces (hybrid seats' decisions in h2h games vs production; `ladder belief-from-games`): gain over uniform falls from +.066 to +.041 nats/tile (to ~0 at trick 5) and it is overconfident there (predicted .75 → observed .67, .85 → .76): it learned LAD5 opponents, not production. Fix: production-opponent training data = hybrid decisions from h2h games: two fresh generation runs `np:models/LAD5-k73.w` seeds 9100 and 9200 (24,576 deals each, one capped round each, 12 workers; side result: np:k73 +.048 [+.041, +.054] on 20,668 paired deals and +.045 [+.038, +.052] on 20,652, independent of the headline seeds) plus every recorded non-seed-5000 h2h game.
**Leak (fixed before any result was kept):** deal identity is `deal_seed = h2h seed + deal index`, so the seed-3000/4000/6000 games overlap the seed-5000 test deals (5000..9095). The first fine-tune (`BEL-po-b`) had 296,184 rows from 3,120 of the 4,096 seed-5000 deals; its seed-5000 h2h numbers below are marked contaminated. Clean data `data/belief-prodopp-c-{train,val}.bin` drop every row with deal seed in [5000, 9096) or [50000, 54096): 790,692 train / 45,879 val (val = later games of seed 9200).
`models/BEL-po-c.w` = BEL-k73-a fine-tuned on the clean rows (lr 5e-4 cosine, 4,000 x 2048, ll .9478 on its val). Rust `belief-eval`, gain over the uniform-support marginal (nats/tile):

| data | t0 | t1 | t2 | t3 | t4 | t5 | all (belief / uniform) | acc belief / uniform |
|---|---|---|---|---|---|---|---|---|
| prodopp-c val (45,879 states) | +.067 | +.067 | +.065 | +.072 | +.075 | +.053 | .9478 / 1.0153 (+.068) | .483 / .410 |
| seed-5000 l2n:k73 hybrid states (68,330; out of sample) | +.062 | +.052 | +.049 | +.050 | +.043 | +.015 | .9648 / 1.0157 (+.051) | .470 / .409 |

Calibration on the seed-5000 states: .05/.077, .16/.179, .25/.272, .34/.347, .44/.425, .54/.506, .64/.597, .75/.688, .85/.80, .95/.92 (overconfident above .5; the train data's partner is mostly the bare net, the test partner is the l2n search).
Marginals the search actually draws from (`belief-eval --tilt`): the naive tilt q ∝ uniform x Π_t P_b/P_u has log loss equal to or below the belief's own at every trick (it adds the exact count constraints); an IPF tilt matched to the belief marginals is worse at trick 5 (the net's count errors get enforced) and was dropped.

### Step 2: belief-weighted worlds in the search

Arm `l2b:OUTER:BELIEF:POLICY[:MULT[:ALPHA[:IPF]]]` (main.rs `l2b_arm`): the `l2n:OUTER:POLICY` one-ring search; the engine's top-level uniform void-consistent worlds get w ∝ exp(ALPHA Σ_unseen t [ln P_belief(holder) − ln P_uniform(holder)]) (P_uniform = exact support marginal), self-normalised. MULT = 1: importance weights as integer fiber weights (round(w·N·4096); root values = weighted make sums; the 0/N pruning bounds use the fiber mass). MULT > 1: MULT·OUTER uniform candidates, systematic resampling of OUTER unit-weight worlds (the unweighted search runs unchanged). ALPHA = 0 = flat weights = the control (same 640-candidate draw and resampler, no belief). Off by default: with no `l2b:` arm the hook pointer is null; check_parity 19/19, `agree` LAD5-k73 on 100k LAD5-val rows .5937 with a byte-identical dump, 24 `l2n:160` h2h games and 6 `log2 --model` games byte-identical between the pre-change binary and target-b2.

ESS/N of the belief weights (1/Σw² / N, mean over decisions, BEL-po-c, 160 candidates; 640-candidate runs the same): trick 0 .40, 1 .36, 2 .36, 3 .37, 4 .45, 5 .64; minimum ≈ .007 (tricks 0-4), .04 (trick 5). No collapse, but ~60 effective worlds of 160 when weighting directly.

H2H vs pinned production, paired mirrored, 4,096 deals each, 12 workers, prod cache; diff = paired difference against the named arm on the same deals (bootstrap 95% CI); exploratory, no multiplicity correction.

| arm (POLICY = LAD5-k73, 160 worlds) | seed 5000 | diff vs uniform | seed 50000 (pre-registered) | diff vs uniform |
|---|---|---|---|---|
| uniform `l2n:160:k73` | +.098 [+.083, +.113] | | +.089 [+.073, +.105] | |
| weighted IS, BEL-k73-a (self-play belief) | +.083 [+.068, +.099] | −.015 [−.032, +.002] | | |
| weighted IS, BEL-po-b (contaminated) | +.090 | −.008 [−.025, +.009] | | |
| resample x4, BEL-po-b (contaminated) | +.123 | +.025 [+.007, +.044] | | |
| resample x4, ALPHA 0 (flat control) | +.095 | −.002 [−.020, +.016] | +.096 | +.007 [−.010, +.025] |
| **resample x4, BEL-po-c (clean)** | **+.108** | +.010 [−.008, +.029]; vs control +.013 [−.006, +.030] | **+.109** | **+.020 [+.002, +.039]**; vs control +.013 [−.005, +.031] |

Pooled over both seeds (8,192 deals): BEL-po-c x4 vs uniform **+.015 [+.002, +.029]** (1,250 deals better / 1,138 worse); vs the flat control +.013 [+.000, +.026]; control vs uniform +.002 [−.010, +.015]. Selection: the x4 design was picked on seed 5000 with the contaminated BEL-po-b; seed 50000 is the single pre-registered read of the fixed arm. Decision time: x4 median 9.4 ms vs 14.0 ms for uniform (duplicate resampled worlds share work).
Gain by trick of the first divergence between the arms' games (sum of per-deal diffs, BEL-po-c x4 vs uniform): seed 5000 t0 +54 (2,946 deals), t1 −10 (867), t2 −1, t3 −1; seed 50000 t0 +83 (2,862), t1 −13 (927), t2 +12 (225), t3 +2. The gain sits in games that diverge in the first trick (the opening lead and the replies, where the belief's trick-0 edge is the bidder-has-the-trumps inference the uniform sampler ignores).
Direct importance weighting (MULT 1) did not help: it keeps ~40% ESS and loses ~.01-.015, consistent with the extra variance of 60 effective worlds.

```
export LADDER_BIN=$PWD/ladder/target-b2/release/ladder
$PY h2h_fast.py --net l2b:160:models/BEL-po-c.w:models/LAD5-k73.w:4 --seed 50000 --tag l2b-160x4-BEL-po-c-LAD5-k73-s50000-4096 --deals 4096 --workers 12 --max-rounds 1
$PY h2h_fast.py --net l2b:160:models/BEL-po-c.w:models/LAD5-k73.w:4:0 --seed 50000 --tag l2b-160x4-a0-LAD5-k73-s50000-4096 --deals 4096 --workers 12 --max-rounds 1
$PY belief_compare.py results/h2h-<A> results/h2h-<B>      # paired diff, divergence-trick split, ESS
$PY belief_pool.py <A1>:<B1> <A2>:<B2>                     # pooled over seeds
$B belief-from-games --games <games-*.jsonl,...> --out data/belief-h2hgames-np-k73-s9100.bin
```

40 worlds (same protocol, both seeds): uniform `l2n:40:k73` +.039 (s5000) / +.045 (s50000); `l2b:40:BEL-po-c:k73:4` +.050 / +.056; diff +.011 [−.009, +.031] / +.011 [−.008, +.031]; pooled +.011 [−.003, +.024] (8,192 deals). Decision time 2.3 ms vs 3.2 ms. (40 → 160 worlds is worth ≈ +.05 for the uniform arm here.)

### Step 3: mixed-partner diagnostic (seed 50000, 4,096 deals)

`ladder h2h --mixed` / `h2h_fast.py --mixed` (new flag, default off): the newcomer sits in one seat of its partnership (seat `side` on even deals, `side + 2` on odd ones) and production is its partner; same deals, mirrored, prod cache.

| arm | both seats newcomer | newcomer + production partner | partner effect (full − mixed) |
|---|---|---|---|
| uniform `l2n:160:k73` | +.089 [+.073, +.105] | +.049 [+.034, +.063] | +.041 [+.023, +.057] |
| `l2b:160:BEL-po-c:k73:4` | +.109 | +.063 [+.049, +.077] | +.046 [+.029, +.063] |
| belief − uniform | +.020 [+.002, +.039] | +.015 [−.002, +.032] | difference of partner effects +.005 [−.016, +.028] |

Reading: one newcomer seat gets a bit over half of the two-seat edge (+.049 of +.089), so the search edge is close to additive per seat, and the belief's extra edge survives with a production partner (+.015 vs +.020). Within this resolution none of the belief gain comes from a shared partner model; it is about reading the opponents' (and partner's) hands from the deal rule and the plays.

### Step 2b: more production-opponent data, a second (val-only) leak, BEL-po-d, fresh seed 60000

Second leak (val only; no h2h number affected): generation seeds 9100/9200/9300/9400 with 20-25k deals each cover overlapping deal-seed ranges, so the same deals recur across them and BEL-po-c's val (later seed-9200 games) shared deals with its train rows (val gain +.068 vs +.051 out of sample). New split: two more generation runs on far-apart seeds 100000 and 130000 (20,000 deals each; plus 9300, 9400), all production-opponent rows minus the three h2h test ranges ([5000,9096), [50000,54096), [60000,64096)); val = deal seeds [100000, 102000) (2,000 deals, 33,349 states, in no train row); train 2,151,519 rows from 60,370 distinct deals (`data/belief-prodopp-d-{train,val}.bin`). `models/BEL-po-d.w`: BEL-k73-a fine-tuned, lr 5e-4 cosine, 8,000 x 2048 (best .9480 at 5,000).

| belief | clean val gain (nats/tile) | seed-5000 l2n hybrid states gain | calibration .75 / .85 bins (s5000 states) |
|---|---|---|---|
| BEL-k73-a (self-play) | +.048 | +.040 | .668 / .760 |
| BEL-po-c | +.060 | +.051 | .688 / .800 |
| BEL-po-d | +.066 | +.055 | .702 / .807 |

H2H on two fresh seeds (deal ranges disjoint from every belief training row), 4,096 deals each, `l2b:160:<BEL>:models/LAD5-k73.w:<MULT>`:

| arm | seed 60000 | seed 70000 |
|---|---|---|
| uniform `l2n:160:k73` | +.104 [+.089, +.118] | +.104 [+.088, +.119] |
| flat control (x4, ALPHA 0) | +.096 [+.081, +.112] (−.007 vs uniform) | +.094 [+.079, +.108] (−.010) |
| BEL-po-c x4 | +.114 (+.011 [−.007, +.030] vs uniform; +.018 [−.001, +.036] vs control) | |
| **BEL-po-d x4** | **+.125** (+.022 [+.004, +.040]; vs control +.029 [+.011, +.047]) | **+.123 [+.108, +.138]** (+.020 [+.002, +.039]; vs control +.030 [+.012, +.048]) |
| BEL-po-d x8 | | +.127 [+.112, +.142] (+.023 [+.005, +.042]; vs x4 +.003 [−.015, +.022]) |

Pooled:

| comparison | deals | diff [95% CI] |
|---|---|---|
| BEL-po-d x4 vs uniform (s60000 + s70000) | 8,192 | **+.021 [+.008, +.034]** |
| BEL-po-d x4 vs flat control (s60000 + s70000) | 8,192 | **+.029 [+.017, +.043]** |
| BEL-po-c x4 vs uniform (s5000 + s50000 + s60000) | 12,288 | +.014 [+.003, +.024] |
| BEL-po-c x4 vs flat control (same three seeds) | 12,288 | +.015 [+.004, +.025] |
| flat control vs uniform (four seeds) | 16,384 | −.003 [−.012, +.006] |

Gain by trick of first divergence (BEL-po-d x4 vs uniform, sum of per-deal diffs): s60000 t0 +57 / t1 +31 / t2 +2 / t3 −3; s70000 t0 +56 / t1 +20 / t2 +5 / t3 0. As before, the gain is in games whose first different play is in tricks 0-1. Decision time (median): uniform 14.0 ms, control 13.9 ms, belief x4 9.4 ms, x8 11.1 ms.

Checks at the end (final target-b2 binary): `check_parity.py` 23/23 (the 19 policy nets + BEL-k73-a, BEL-po-c, BEL-po-d and a synthetic belief net on `fixtures/belief-prodopp-d-val-first100.rec`; log loss equal to 1e-8, max logit diff ≤ 1e-5); `agree` LAD5-k73 dump byte-identical to the pre-change binary; 24 `l2n:160` games, 24 `np:` games (incl. hybrid/native timing split) and 6 `log2 --model` games byte-identical.

### Summary (belief work, 2026-10-06)

1. A supervised belief head (the policy's encoding, 84 outputs) beats the exact uniform-support marginal the sampler assumes by ~.05-.07 nats per unseen tile at every trick, already at trick 0 (bidder-has-the-trumps); it must be trained on states against production (a LAD5-self-play belief is overconfident against production and loses ~40% of its edge).
2. Importance weights over 160 uniform worlds keep ESS ≈ .36-.45 of N (≈ .64 at trick 5) and do not help (−.015 to −.008). Resampling 160 worlds from 4 x 160 uniform candidates does: with the cleanest belief (BEL-po-d) +.021 [+.008, +.034] over the uniform `l2n:160:k73` arm and +.029 [+.017, +.043] over a same-draw flat control on 8,192 fresh deals (two pre-registered seeds), taking the 160-world arm from ≈ +.10 to ≈ +.124 vs production; 8 x candidates adds nothing measurable. At 40 worlds the gain is +.011 [−.003, +.024].
3. Partner diagnostic: the belief's extra edge survives a production partner (+.015 vs +.020), and the partner effect is the same with and without belief (+.005 [−.016, +.028]): the gain is reading hands, not a shared partner model.
4. Two leaks found and fixed (deal identity = deal seed, which overlaps across h2h seeds): the contaminated BEL-po-b results are kept only as marked rows above.
Not done: belief-weighted labels into a ladder rung (`log2 --belief`, implemented and smoke-tested only); a belief trained on search-partner (`l2n`/`l2b`) games, which would match the test partner better; a belief for the inner (rung-1) modeled players.

Files: new `ladder/src/belief.rs`, `train_belief.py`, `belief_compare.py`, `belief_pool.py`, `fixtures/belief-prodopp-d-val-first100.rec`, `models/BEL-{k73-a,po-b,po-c,po-d}.{w,json}` (+ `BEL-smoke` smoke test), `data/belief-*.bin`, `checkpoints/BEL-*.state.npz`, `results/h2h-l2b-*`, `results/h2h-l2n-{40,160}-LAD5-k73-s{5000,50000,60000,70000}-4096*`, `results/h2h-belief-gen-*`. Changed: `ladder/src/net.rs` (belief header kind, `trunk_generic`), `ladder/src/turbo.rs` (`WorldBelief`, world-weight callback), `ladder/csrc/turbo.cpp` (`hookw`, fiber-mass bounds when weighted), `ladder/src/main.rs` (`l2b:` arm, `belief-log`, `belief-from-games`, `belief-eval`, `h2h --mixed`, `log2 --belief`), `h2h_fast.py` (`--mixed`), `check_parity.py` (belief section), `README.md` (header table, code map).

## Bid probe: can the 4 µs net bid standalone? (2026-10-08, exploratory tier)

Question (Jason): the deployed model bids by running many simulations; can LAD5-k73 bid without them?
Probe: `ladder bideval` (`--prod` realizes contracts with production Walt), report `bideval_report.py`. 2,000 uniform deals (seeds 9000-10999, not the
most-trumps deal rule) × 4 bidder seats × 7 trumps = 56,000 contracts. Three estimators of P(make 30): **standalone** = max over the 7
opening leads of sigmoid(net root score) at trick 0 (7 forwards, no search); **stand-mean** = mean over the leads; **rollout** = make rate
over 256 uniform redeals of the 21 unseen tiles played out by the net in all four seats (44 µs per game, 11 ms per contract, 80 ms for all
seven trumps; the same sims with production Walt would be ~38 s per trump). Realized outcome: net self-play on the actual deal (all 2,000
deals), and production Walt in all four seats on the first 200 deals (5,600 contracts, the deployed player's outcome).
Data: `results/bideval/k73b-c*.jsonl` (net), `results/bideval/prod-c*.jsonl` (production), `results/bideval/run-*/run.json` (cap records).

Against production outcomes (200 deals; base make-30 rate .302):

| estimator | Brier | log-loss | AUC | mean p |
|---|---|---|---|---|
| standalone (max lead, raw) | .531 | 1.81 | .680 | .875 |
| stand-mean (raw) | .222 | .642 | .698 | .483 |
| standalone recalibrated (20 bins fit on even deals, scored on odd) | .189 | .562 | .679 | .306 |
| rollout 256 worlds | .186 (.181 same odd deals) | .555 | .700 (.709) | .294 |

Standalone calibration is broken by construction: every training game is a 30 bid whose bidder holds the most trumps, so the root score is
P(make | I am the bidder of a rule-chosen contract) and sits at .96 for 58% of contracts (observed .39 there). Its ranking is nearly as good
as the rollout's; a bin recalibration recovers most of the Brier gap. Trump choice per (deal, seat), realized make rate of the chosen trump
under production play: standalone .520, rollout .545, deal rule (rule bidder only) .690 vs standalone .685 vs rollout .705, random .302,
single-game oracle .885. Contract choice over all 28 (deal, seat, trump) options: standalone .670, rollout .705, deal rule .690 (±.03 at 200
deals). Bid-30-if-p≥.5: standalone bids 97% of contracts (made .31), rollout bids 11% (made .64); rollout P(≥35) AUC .70 (base rate .185), P(=42) AUC .77 (base rate .048),
calibrated against net self-play outcomes where the games play all 28 tiles (`results/bideval/k73full-c*.jsonl`; the first net run and the
production run stop at 30 points, so their above-30 rows are invalid: the production comparison is bid 30 only); the standalone net cannot
express bids above 30 at all (never trained on them). Net self-play outcomes on all 2,000 deals give the same picture at 30 (AUC .669 / .693 /
.699; rollout Brier .181).

Reading: standalone is a usable ranker and not a usable probability; the rollout with the net inside is calibrated at every bid level and
costs 80 ms per hand, three orders cheaper than the deployed sims. The clean next step is a bid head distilled from the rollout
(hand + trump → P(points ≥ b) for b = 30..42), i.e. the rollout's answer at 7 forwards per hand. Caveats: no auction information (other
seats' bids/passes) in either estimator; production outcomes are single games on 200 deals; exploratory tier, no multiplicity correction.

## LAD6: retrain for the distribution we want (bid-aware, deal mixture) (started 2026-10-08)

Decision (Jason, 2026-10-08): do not bolt a bid head onto LAD5-k73; retrain the ladder on the distribution we want the net calibrated to,
"its own make rates vs players like itself". Changes, every default keeping LAD1-5 byte-identical (parity 23/23 after the change):
- **Engine contract.** `turbo.cpp` `Context.bid` (default 30; terminal = bidding team >= bid or defenders > 42 - bid; leaf value = made at
  bid), `walt_set_bid` / `Engine::set_bid`. `play_bid` in main.rs (`play` = bid 30; bid > 42 = play all 28 tiles, for rollout histograms).
- **Encoding 5** = encoding 4 plus a one-hot contract row (`INPUTS4 + bid - 30`, 13 rows). `Net.bid` (30 unless `Net::load_bid`), trainer
  `Enc(5)` column 117. An encoding-4 net widens to 5 with zero rows (`--init` from .w or .npz): same logits at bid 30 (dumps byte-identical,
  verified for LAD5-k73).
- **Record byte 2** = maximize (bit 0) | (bid - 30) << 1. All readers mask (`train.maximize_of`, `train.bid_of`, rust_regret.py,
  build_distill.py); LAD1-5 files read as bid 30.
- **log2 deal mixture** (`--bid-worlds K`, `--mix P`, `--bid-uniform U`, needs `--play np:`): every game's bid level = the highest the
  --play net's K-world rollout (net in all four seats, full play) makes with probability >= 1/2, then a -1/0/+1 jitter, or uniform over
  30..42 with probability U; with probability P the contract is a random bidder seat with the rollout's preferred trump (1 in 5 a random
  trump), else the deal rule's. The teacher engine, the modeled-others net and the state-driving net all get the game's bid. Smoke (8 games):
  bids 30-37, trick-0 teacher make rate falls with the bid as it should.
- **pipeline.py**: `--outer-val` (validation worker at 160 worlds while train workers label at `--outer`), `--h2h-floor F` (hard gate: a
  net that passes the regret gate is `pending_h2h` until its np h2h vs production at bid 30 scores >= F; only `ready` nets become the
  teacher), `--mix/--bid-worlds/--bid-uniform` passthrough, `--enc` passed to the stu trainer.

Advisor (second Fable, read-only; results/ timings): LAD5's 7.04 h were 90% labeling on the critical path (79 rounds x 290 s, 195 states/s
with 8 workers under h2h contention); the l2n:160 h2h cost 4.1 core-hours of diagnostics; training was hidden behind labeling. Measured with
k73 on this 18-core Mac: 8 workers/160 worlds 254 states/s, 14/160 375, 8/64 635, 14/64 860. 64-world labels: argmax matches 160-world
72% of states, regret .0067 vs the 160 counts (usable for training early, not for the gate). Recommendation adopted: 14 label workers, train
rows at 64 worlds for the early rounds with val at 160, trainer wall 270 s, no search h2h in the loop (milestones only), rollout bidder at 64
worlds, mix .35 plus 10% uniform bids, production h2h at bid 30 as a hard gate (floor +.02; k73 is +.056 +- .015), keep 256 wide unless the
bid-30 h2h drops. Expected: ~40 rounds in 3-4 h.

Command (nohup; stdout `results/pipeline-LAD6.out`, events `pipeline/LAD6/log.jsonl`; resumable by rerunning):
```
LADDER_BIN=$PWD/ladder/target-o/release/ladder nohup python3 pipeline.py --name LAD6 --net models/LAD5-k73.w --trainer stu \
  --gate --gate-metric regret --ladder --outer 64 --outer-val 160 --inner 8 --eps 0.1 \
  --mix 0.35 --bid-worlds 64 --bid-uniform 0.1 \
  --label-workers 14 --h2h-workers 4 --h2h-floor 0.02 --seed-base 60000000 --rounds 40 --nets 40 --hidden 256 --hidden2 256 --enc 5 \
  --stu-args '--loss regret --rw 20 --tau 0.5 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 270' \
  --extra-train data/labels2-LAD5-train.bin@1000000 > results/pipeline-LAD6.out 2>&1 &
```
Milestones (manual, from the monitor loop): `ladder bideval --bid 30|35|42` + `bideval_report.py --bid` on the newest net at rounds ~5/15/30
(standalone calibration vs the rollout is the point of LAD6), l2n:160 h2h at the end.

### LAD6 milestone, round 5 (2026-10-08 16:55): gate fixed, two promotions, bid slot learning, root score still a winner's curse

Gate bug (fixed, 8c1845261): `ladder agree` evaluated every validation row at bid 30, so k0-k3 were `agree_mismatch`; the Rust and trainer
numbers now match to 1e-5. After the fix: k4 promoted (regret .01240 vs k73's .01411 on the mixed val; production h2h at 30 +.038
[+.023, +.053]), k5 promoted (.01241; +.048 [+.033, +.063]), k6 rejected. Rounds land ~212k states each.

`bideval` on LAD6-k5 (500 deals, 64-world rollouts, asked at each bid; `results/bideval/LAD6k5-b*`), standalone = max over leads:

| asked at | base rate | standalone mean p / AUC | stand-mean Brier | rollout Brier / AUC |
|---|---|---|---|---|
| 30 | .292 | .869 / .675 (k73: .874 / .675) | .216 | .182 / .694 |
| 35 | .180 | .778 / .678 | .163 | .132 / .695 |
| 42 | .062 | .402 / .712 | .058 | .053 / .717 |

The bid slot is being learned (the score falls with the bid, k73 cannot) but the root score is still far above the base rate. Diagnosis on
the mixed val (trick-0 bidder leads, 3,399 rows): the net's per-tile mean (.640) matches its own labels (.631) and the per-tile rmse is .134,
but the max over the 7 leads is .980 against a label max of .784. A max over seven noisy estimates is a winner's curse, and the regret loss
(weight 20 against BCE weight 1 in the LAD5 recipe) spreads the chosen tile's logit upward on purpose. Test running: one rung from k5 with
`--bw 10` (BCE x10) on the same data, to see whether per-tile calibration improves without losing regret. If it does, the pipeline restarts with
that `--stu-args`. Note for the probe tables: a k73 report at `--bid 35/42` is invalid (its realized outcomes were computed at 30).

### Faithful-distillation test, 2026-10-08 17:10: one rung each from LAD6-k5 on the same data (LAD6 train pile + 1M LAD5 rows)

Jason's direction: distill Walt as Walt, the pmake vector by public information, without relaxations; belief stays off the LAD6 path.
The regret term is the one relaxation in the LAD5 recipe (softmax over the legal tiles' logits, weight 20 against BCE 1). Measured:

| rung | val regret | val rmse | trick-0 chosen lead: net p vs its own label | np h2h vs production at 30 |
|---|---|---|---|---|
| k5 (regret recipe, reference) | .01246 | .167 | .951 vs .776 | +.048 [+.033, +.063] |
| `--bw 10` (BCE x10, regret kept) | .01256 | .119 | .895 vs .774 | +.036 [+.022, +.052] |
| `--loss bce` (pure BCE, faithful) | .01518 | .098 | .791 vs .772 | +.005 [−.011, +.021] |

Pure BCE removes the winner's curse almost entirely (the net reads its chosen lead within .02 of the teacher's number) and costs .0027
regret and ~.04 of h2h at 256 wide after one rung: the cheap net drops from beating production to parity. `--bw 10` keeps the play strength
and halves the per-tile error but still overstates the chosen lead by .12. Models `LAD6-purebce-test.w`, `LAD6-bw10-test.w`; h2h in
`results/h2h-np-LAD6-*-test-s5000-4096/`. Open question for the search player: inside `l2n:160` the modeled others are meant to be Walt, so a
faithful net may be the better inner model even if it is the weaker standalone player (l2n h2h of both queued).

### LAD6 milestone, round 15 (17:35): k11-k14 promoted (h2h +.039 to +.042, regret .01191); standalone calibration unchanged under the regret recipe

`bideval` on LAD6-k14 (500 deals, 64-world rollouts): asked at 30 standalone mean p .868 (base .286, AUC .667), at 35 .765 (base .182),
at 42 .362 (base .064, AUC .706); stand-mean / rollout Brier .217/.183, .161/.134, .059/.055. Identical to k5 within noise: more rungs of the
regret recipe do not move the root score's calibration, as the loss analysis predicted. The search player with the pure-BCE one-rung net as
the modeled others: l2n:160 +.090 [+.075, +.105] on seed 5000 (k73 inside: ≈+.10; k5 inside: running).

### Sampling budget by trick (2026-10-08 18:00; Jason: 8 is too few early, too many late; enumerate where it is cheap)

Same 665 states (40 LAD6-mixture games, k14 driving and modeled), labeled under eight settings; scored against an independent 640-world
sample (`--field-seed 777`) as "regret = make probability the setting's argmax gives up under the reference", by trick of the state.
27% of states have an exact top tie in the 640-world labels. `--outer-schedule` (outer worlds by trick) and `--enumerate CAP` (exact outer
average when the support has <= CAP deals; N = support size in the record) are new log2 flags; `--field-seed` picks the sampling seed.

| setting | ms/state | argmax = ref | regret | t0 | t1 | t2 | t3 | t4 | t5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 160 worlds (LAD5 teacher) | 62 | .774 | .0026 | .0047 | .0034 | .0031 | .0027 | .0012 | .0003 |
| 160, other seed | 58 | .738 | .0037 | .0065 | .0042 | .0053 | .0035 | .0015 | .0003 |
| 320 | 117 | .786 | .0022 | .0037 | .0042 | .0023 | .0010 | .0013 | .0000 |
| 640 (same seed family as the reference's size) | 226 | .818 | .0014 | .0021 | .0023 | .0015 | .0013 | .0005 | .0003 |
| 160 + enumerate <= 400 | 62 | .777 | .0025 | .0047 | .0034 | .0031 | .0027 | .0005 | .0004 |
| 160 + enumerate <= 2,000 | 68 | .783 | .0025 | .0047 | .0034 | .0031 | .0024 | .0004 | .0004 |
| schedule 640,320,160,160,160,160 + enumerate <= 400 | 170 | .812 | .0018 | .0016 | .0019 | .0031 | .0027 | .0005 | .0004 |

Reading: the 640-vs-640 line (.0014) is the reference's own noise, so a 160-world teacher gives up about .001-.002 of make probability per
decision to sampling, almost all of it in tricks 0-3 where the support is 10^4-10^8 deals; by trick 5 sampling is free of error. Enumeration
below 400 deals costs nothing (same ms) and makes tricks 4-5 exact: it goes into the LAD6 teacher at the next restart. Spending 2.7x on the
early tricks (the schedule) buys .0008; 3.6x everywhere buys .0012. The trick-0 noise also sets the root pmake's label noise for bidding
(±.06 at 64 worlds, ±.04 at 160), which the net averages out. Inner sampling of near-ties (`l2s:160:0.02`, pure-BCE net inside) scored
+.082 [+.067, +.096] vs the argmax inner model's +.090: no gain at tau .02 (tau .05 running).

Inner-tie sampling, complete (18:16): `l2s:160:TAU` with the pure-BCE net inside vs production, seed 5000, 4,096 deals: argmax +.090
[+.075, +.105]; tau .02 +.082 [+.067, +.096]; tau .05 +.055 [+.041, +.070]. Sampling the modeled others' near-ties hurts, and more so as the
temperature rises: the search wants sharp (argmax) inner players; the ties are resolved by the outer average over worlds, not inside them.
Enumeration restart (round 23, k19 teacher): rounds still land 204-212k states (no throughput cost); k20-k22 promoted, h2h +.046 to +.050,
regret .01149.

### LAD6 milestone, round 30 (18:40): k23-k26 promoted (h2h +.044 to +.052, regret .01117); calibration flat, as expected under the regret recipe

`bideval` on LAD6-k26 (500 deals, 64-world rollouts): standalone mean p .864 / .753 / .346 asked at 30 / 35 / 42 (base rates .283 / .183 /
.063; AUC .675 / .679 / .711); stand-mean Brier .213 / .159 / .058 vs rollout .180 / .135 / .054. Same as k5 and k14 within noise. Play
strength keeps climbing (26 promotions in 30 rounds, regret .01411 → .01117 on the mixed val); calibration of the root score will not move
until the loss changes (faithful BCE or a value output), pending Jason's call.

### LAD6 complete (2026-10-08 19:20): 40 rounds, 37 nets, 31 promoted, 3.25 h wall, 7.83M states

Final net **LAD6-k35** (encoding 5, 256x256 MLP, 4 µs): mixed-val regret .01070 (k73 scored .01411 on the same val at the start; k73's own
val .0100); np h2h vs production at bid 30: seed 5000 +.052, seed 6000 **+.058 [+.043, +.073]** (k73: +.058 on seed 6000); as the modeled
others inside `l2n:160`: **+.101 [+.086, +.115]** (k73 inside: ≈+.10). So the bid-aware net on the deal mixture reaches k73's play strength
at bid 30 and the same one-more-level search strength, while reading the contract (its root score falls with the bid; k73 cannot) and
covering random-seat contracts. 31 of 33 post-fix rungs passed both gates (regret <= teacher, production h2h >= +.02); the two rejections
were regret ties. Throughput 4.4x LAD5 (14 workers, 64-world train labels, 160-world val, enumeration <= 400 from round 23).
Not achieved, by design of the regret recipe: a calibrated root score (standalone mean p .86 at bid 30 vs base .28 at every milestone).
Open decision (Jason): faithful BCE (calibrated within .02, −.04 h2h standalone at 256 wide, −.01 inside the search) vs a BCE-only value
output beside the policy vector; the LAD6 pipeline and data are ready for either as a continuation from k35.
Artifacts: `models/LAD6-k*.w/.json/.npz`, `data/labels2-LAD6-{train,val}.bin` (4.9M / 154k rows + later rounds), `pipeline/LAD6/`,
`results/h2h-np-LAD6-*`, `results/h2h-l2n-160-LAD6-*`, `results/bideval/LAD6k{5,14,26}-b*`.

## LAD7: faithful BCE at 512 wide (started 2026-10-08 22:22; Jason: "hit the gas: faithful BCE and 512")

Decision: the loss is pure masked BCE on the teacher's pmake vector (no regret term), the net is a 512x512 MLP (encoding 5, 1.04M
parameters, 4.2 MB f32), the rest of the LAD6 recipe is unchanged (one-ring 64-world teacher with the newest promoted net as the
modeled others, 160 worlds on the validation worker, enumeration <= 400, deal mixture .35 + 10% uniform bids, production h2h gate).

- **Start net** = LAD6-k35 widened (launched as `models/LAD7-k0.w`, which the pipeline's first trained net then overwrote under the same name; the
  widened start is regenerated deterministically as `models/LAD7-start.{npz,w}`, `widen_net.py` seed 11) function-preservingly by `widen_net.py` (new first-layer units get the scratch init
  and feed only the new second-layer units, whose output rows start at zero). Verified identical to k35 on the LAD6 val file in Rust and
  in the trainer (agree .5961, regret .010695, bce .5506, rmse .1593).
- **Cost of 512 inside the teacher: 3.9x** (one worker, 150 s, LAD6 settings: 682 states with LAD7-k0 vs 2,646 with LAD6-k35). Rounds are
  time-capped, so the wall per round is unchanged and the new-state yield per round drops to about a quarter (~50k vs ~196k); the
  student trains on the full LAD6 pile (7.6M) + 1M LAD5 rows + the LAD7 pile every rung. Trainer at 512: ~163 updates/s, the 26k-update
  cosine finishes in ~165 s inside the 270 s wall.
- **Gate changed to faithfulness**: new `pipeline.py --gate-metric bce` (Rust-side masked BCE of sigmoid(score) vs counts/N on the current
  val file, `rust_regret.py` now reports bce and rmse from the `agree --dump` scores; matches the trainer's bce within .0002) and
  `--select bce` (checkpoint selection). The regret gate would have rejected every faithful net: BCE training moves regret from .0107 to
  ~.015 at any width (12k BCE updates from k0 on 1M rows: regret .0154, bce .502 vs k0's .551; the 256-wide one-rung test gave .0152).
  h2h floor 0.0 (not worse than production at bid 30; the 256-wide faithful net was +.005).
- Why the probabilities elude the regret recipe while play is fine (Jason: "feels significant"): the labels are 64-world estimates with
  ~.06 per-tile noise; the best-tile decision rests on top-tile gaps of .01-.02, below that noise. The regret term trains the ranking
  directly and is free to exaggerate the gap (the winner's curse we measured); BCE reproduces the vector in expectation and loses the
  ranking precision. The two targets coincide only as the labels become exact (more worlds, enumeration), so label noise, not capacity,
  is the expected bottleneck for a faithful net. Prediction to test on LAD7: width buys little regret back; calibration arrives in one rung.

Command (nohup; stdout `results/pipeline-LAD7.out`, events `pipeline/LAD7/log.jsonl`; resumable by rerunning):
```
LADDER_BIN=$PWD/ladder/target-o/release/ladder nohup python3 pipeline.py --name LAD7 --net models/LAD7-k0.w --trainer stu \
  --gate --gate-metric bce --select bce --ladder --outer 64 --outer-val 160 --inner 8 --eps 0.1 --enumerate 400 \
  --mix 0.35 --bid-worlds 64 --bid-uniform 0.1 \
  --label-workers 14 --h2h-workers 4 --h2h-floor 0.0 --seed-base 70000000 --rounds 40 --nets 40 --hidden 512 --hidden2 512 --enc 5 \
  --stu-args '--loss bce --bw 1 --lr 0.0005 --lr-end 0.00001 --updates 26000 --eval-every 3000 --seconds 270' \
  --extra-train data/labels2-LAD6-train.bin,data/labels2-LAD5-train.bin@1000000 >> results/pipeline-LAD7.out 2>&1 &
```
Milestones (monitor loop): `bideval --bid 30|35|42` + `bideval_report.py --bid` at rounds ~5/15/30 (the point: standalone mean p vs the
base rate and the rollout Brier), np h2h per net from the pipeline, l2n:160 h2h with the final net inside at the end.

### LAD7 first ticks (22:45): the faithful 512 net clears the outcome gate at +.026, k1 +.021

Rounds land every ~4.9 min with 58-71k states each (as predicted from the 3.9x cost). k0 = one BCE rung (26k updates, 8.65M rows) from the
widened k35: Rust bce .4914 on the LAD7 val, regret .0118, **np h2h vs production +.026** (the 256-wide faithful net was +.005; k35 +.052).
k1 (from k0): bce .4870 vs k0 .4899 on the same val, regret .0117, h2h +.021, promoted; k2 rejected by the bce gate (.48527 vs k1 .48498);
k3 training from k1, round 4 labeling with k1 inside. Note: the k0 gate compared the trained k0 against itself (the pipeline's initial net
path was the same file), a no-op pass; from k1 on the gate is real.

### LAD7 milestone, round 5 (23:10): faithful in one rung at the root; the remaining bideval gap is the teacher's population, not the net

Nets so far: k0 +.026, k1 +.021, k3 +.029, k5 +.033 (np h2h vs production at bid 30, seed 5000, 4,096 deals; all promoted), k2/k4/k6
rejected by the bce gate by < .001. Val regret .0115-.0121 (k35-equivalent start .0118 on this val). Rounds ~4.9 min, 57-71k states each.

**Root fidelity on the LAD6 val file (15,536 trick-0 declaring states, never trained on):**

| net | net max p over legal leads | teacher label at that lead | teacher's best label | mean p over legal leads: net / label |
|---|---|---|---|---|
| LAD7-k5 (faithful BCE, 512) | .779 | .765 | .774 | .630 / .626 |
| LAD6-k35 (regret recipe) | .948 | .767 | .774 | .631 / .626 |

The faithful net reads its chosen lead within .014 of the teacher's number (by bid: 30 .759/.744, 35 .844/.834, 42 .743/.721); the regret
net overstates it by .18. Per-tile means match for both (BCE term). So "calibrated to the teacher" arrived in one rung, as predicted.

**bideval on LAD7-k5 (500 deals x 28 contracts, outcomes by net self-play, 64-world rollouts):** standalone mean p .576 at bid 30 (base
.283; LAD6: .86), .424 at 35 (base .180; LAD6 .75), .147 at 42 (base .065; LAD6 .35). Standalone AUC .699/.702/.758, rollout .696/.702/.747;
recalibrated standalone Brier .1805 vs rollout .1796 (bid 30). Contract choice realized make rate: standalone .742 / rollout .744 / rule .678.
Files `results/bideval/LAD7k5-b{30,35,42}-c{0,1}.jsonl`.

Reading the two together: the net is faithful to its teacher, and the teacher's pmake is **P(make | the declaring side runs the 64-world
one-ring search, everyone else plays the net)**, which is not the make rate of the net playing all four seats (the bideval base rate), nor
of "players like itself". The one-ring teacher is asymmetric by construction (a best response to the net), so its numbers are optimistic for
a net that then plays as itself. The bideval's max over 7 leads and 7 trumps adds the usual winner's curse on top. Open design point for
Jason: "calibrated to its own make rates vs players like itself" needs a symmetric label (the search in all seats, or the net's own
self-play across worlds, which is what the rollout bidder already computes), not a best-response label. Test to settle it (round-15
milestone, needs a small bideval option): realize the same contracts with the teacher as the declaring side and the net as the others and
compare the realized make rate with the standalone .576.

### LAD7 milestone, round 15 (23:50): k14 h2h +.057 (= k35); root calibration unchanged, as it must be under this teacher

Promoted so far k0, k1, k3, k5, k7, k8, k9, k11, k12, k13, k14 (np h2h vs production at bid 30: +.026, +.021, +.029, +.033, +.036, +.044,
+.044, +.049, +.045, +.052, +.057); rejected by the bce gate k2, k4, k6, k10 (all by < .001). The faithful 512 net has recovered the regret
recipe's standalone strength by round 15. `bideval` on k14 (500 deals): standalone mean p .574 / .418 / .142 at bids 30 / 35 / 42 (base
.285 / .181 / .067), identical to k5; AUC .701 / .704 / .765 (rollout .697 / .696 / .740); recalibrated-by-ventile Brier .181 / .138 / .055
(rollout .179 / .134 / .056); contract choice realized make rate .734 / .542 / .320 (rollout .736 / .548 / .310, rule .666 / .514 / .288).
Files `results/bideval/LAD7k14-b{30,35,42}-c{0,1}.jsonl`.

### Probe: outcome labels by net self-play across worlds (exploratory, 23:30-23:45; stopped on Jason's call)

`ladder rollrelabel` (new, VARIANT; `Whole::NetForced` plays a given first tile, then the net in all four seats) relabels trick-0 rows: per
hand tile, 64 uniform redeals of the unseen tiles, the tile led, net self-play to the contract's end, counts = worlds made. On the 15,536
LAD6-val root states with LAD7-k12 (62 ms per state at 512 wide, `data/rollroot/val-k12-w*.bin`; a 25k-row training-side relabel was
started and stopped at 4 x 6,318 rows, `data/rollroot/train-k12-w*.bin`):

| | search labels (one-ring, 64-160 worlds) | self-play rollout labels (64 worlds) |
|---|---|---|
| mean over legal leads | .626 | .424 |
| mean of the best lead | .774 | .518 |
| within-hand spread (best − worst) | .349 | .209 |
| argmax agreement with the other | .245 | .245 |
| exact top tie | — | .206 |
| within-hand correlation | .667 | |

Regret of the search's lead under the rollout labels .053 (a uniformly random lead .094); of the rollout's lead under the search labels .076
(random .149). Caveat: 64-world binomial noise is .058 per tile, so a noisy argmax inflates every "regret under rollout" by roughly the
expected max of 7 noise draws (~.05); an independent replicate (`--seed`, built in `target-r2`) was not run. The two sticks disagree on the
opening lead far more than noise explains: the search's number is P(make | perfect information inside each world, best response to the
net), the rollout's is the net's own imperfect-information self-play. Jason (23:44): self-play in a partnership game is a thing to walk
into intentionally, not for bideval's sake; stay the course, get a Walt that can bid, then beliefs (Bayesian). The probe is parked; the
code stays as a tool.

Bidding with the faithful net, the path that stays the course: the root score already ranks contracts like the 64-world rollout (AUC and
the realized make rate of its chosen contract match), so the bidder is the net's rank plus a monotone level map from root score to realized
outcome under the population that will play (a 10-bin table per bid level, fitted on bideval deals; held-out Brier equals the rollout's).
The population choice is Jason's: net self-play (the app's own player) or production Walt (`bideval --prod`, bid 30 only today).

### LAD7 milestone, round 30 (2026-10-09 00:58): k29 h2h +.062; the picture is stable

Promoted k21-k25, k27-k29 (+.042, +.052, +.048, +.045, +.060, +.054, +.051, +.062); rejected by the bce gate k20, k26. `bideval` on k29
(500 deals): standalone mean p .571 / .416 / .140 at bids 30 / 35 / 42 (base .285 / .182 / .067; k5 and k14 the same); AUC .694 / .707 /
.759 (rollout .688 / .701 / .737); contract choice realized make rate .724 / .558 / .308 (rollout .718 / .572 / .302, rule .662 / .526 /
.288). Files `results/bideval/LAD7k29-b{30,35,42}-c{0,1}.jsonl`. Ten more rounds to the finish (~01:45); the end checks are the np h2h on
seed 6000 and l2n:160 with the final net inside.

### LAD7 complete (2026-10-09 02:10): 40 rounds, 40 nets, 32 promoted, 3.31 h wall, 2.95M new states; final net `models/LAD7-k39`

| measure | LAD7-k39 (faithful BCE, 512) | LAD6-k35 (regret recipe, 256) |
|---|---|---|
| np h2h vs production at 30, seed 6000, 4,096 paired deals | **+.058 [+.044, +.073]** | +.058 [+.043, +.073] |
| np h2h, seed 5000 (pipeline gate) | +.064 | +.052 |
| l2n:160 with the net as the modeled others, seed 5000 | **+.100 [+.086, +.115]** (28.7 ms/decision) | +.101 [+.086, +.115] (10.7 ms) |
| LAD6 val: regret / bce / rmse | .01097 / .4832 / .0709 | .01070 / .5506 / .1593 |
| root score vs teacher label at the chosen lead (15,536 LAD6-val root states) | .773 vs .766 | .948 vs .767 |
| reads the bid | yes | yes |

Same play strength alone and inside the search as the regret recipe, with the root score now within .01 of the teacher's number
(per-tile rmse .071 vs .159). Promoted k0, k1, k3, k5, k7-k9, k11-k17, k19, k21-k25, k27-k37, k39; rejected by the bce gate (all by
< .001) k2, k4, k6, k10, k18, k20, k26, k38. Standalone calibration in `bideval` is flat across rounds (.567 / .413 / .137 at bids 30 /
35 / 42 on k39 vs base .281 / .180 / .067): the teacher's population, see the round-5 note.

**Bidding artifact: `models/LAD7-k39-bidcal.json`** (`bidcal.py`, new): isotonic map from the standalone root score to the realized make
rate under net self-play, per bid level, fitted on the 500 bideval deals (14,000 contracts each). Held-out (odd deals, fit on even):
Brier .1788 / .1332 / .0576 at 30 / 35 / 42, mean p .282 / .175 / .065 vs base .279 / .182 / .069; the 64-world rollout on the same deals
scores .1785 / .1308 / .0567. So "the net's rank plus the table" bids as well as the rollout at the cost of 7 forwards. Open for Jason:
the population the table should be fitted to (net self-play as here, or production Walt via `bideval --prod`, bid 30 only today).

Artifacts: `models/LAD7-k*.{w,json,npz}`, `models/LAD7-start.{w,npz}` (widened k35), `data/labels2-LAD7-{train,val}.bin`,
`pipeline/LAD7/`, `results/h2h-np-LAD7-*`, `results/h2h-l2n-160-LAD7-k39-s5000-4096`, `results/bideval/LAD7k{5,14,29,39}-b*`,
`data/rollroot/` (parked probe). Nothing running.

## Published (2026-10-09): PR #102 and Hugging Face artifacts

- **Code and record:** https://github.com/jasonyandell/texas-42/pull/102 (branch `pr/fable-tiny-net-ladder`, an
  artifact-free snapshot of `experiment/fable-tiny-net-ladder`, base `codex/walt-higher-k-budget-20261004`).
- **Models:** https://huggingface.co/jasonyandell/texas-42-walt-tiny-net-ladder — `LAD7/LAD7-k39.{w,json,npz}`,
  `LAD7/LAD7-k39-bidcal.json`, `LAD7/LAD7-start.*`, `LAD7/all/` (every rung), `LAD6/LAD6-k35.*`, `LAD6/all/`, model cards,
  README with encoding 5 and the LAD6/LAD7 section.
- **Labels:** https://huggingface.co/datasets/jasonyandell/texas-42-walt-ladder-labels — `data/labels2-LAD6-{train,val}.bin`
  (7.59M / 239k rows), `data/labels2-LAD7-{train,val}.bin` (2.86M / 91k), README (contract byte packing, deal mixture),
  SHA256SUMS.
- **Everything else:** https://huggingface.co/datasets/jasonyandell/texas-42-walt-archive/tree/main/fable-tiny-net-ladder-20261009 —
  `tracked-artifacts.tar.gz` (the 5,715 model/data/result files the experiment branch had committed, 426 MB) and
  `results-LAD6-LAD7.tar.gz` (bideval, h2h games and summaries, pipeline logs, the parked rollout probe, 94 MB), with
  `MANIFEST.json` and SHA-256s. Unpack over the PR branch to restore the tree this record describes.
