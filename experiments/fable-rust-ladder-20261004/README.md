# fable-rust-ladder: tiny nets that play Texas 42 against Walt

**Tier: exploratory.** Everything in this directory is an experiment record (probe tier in the repository's
evidentiary order). Nothing here is a receipt, a proof, or a corpus status, and nothing here is cited by anything
above it. Head-to-head numbers are paired mirrored deals against pinned production Walt with 95% confidence
intervals and **no multiplicity correction**; several models below were chosen as the best of many candidates scored
on the same seed, so their headline numbers carry selection bias (stated per model).

This directory distills Walt's delta-expectimax search (the production player of the walt42x engine) into small
policy networks over the level-0 information state, then climbs a self-improving ladder: a search teacher that uses
the newest student as its model of the other players labels states the student itself reaches, a new student is
trained on them, and a regret gate decides whether it replaces the old one. The night of 2026-10-05 produced a
4 µs net (`models/LAD5-k73.w`, a 256x256 MLP) that beats production Walt when it plays alone, with no search.

Chronology and every number's provenance: `DELTA-EXPECTIMAX.md` (section "Overnight 2026-10-05"), `STATUS.md`,
`PERF.md`, `reports/student-*.md`. License: CC0-1.0 (`LICENSE`), this directory only.

## The final recipe (LAD5)

- **Teacher.** `l2n:160:<net>`: Walt's one-ring level-2 search over 160 sampled outer worlds, in which the modeled
  other players (rung 1) play the newest promoted student net instead of a sampled search. About 20 ms per decision.
  Labels are 160-world make counts per legal tile (`k/N`, declarer-make probability).
- **States.** Played by the same newest promoted net (`log2 --play np:<net>`, exploration eps .1), so the teacher
  labels the states the student actually reaches. One capped labeling round gives about 65k states (8 workers, the
  last one writes validation rows).
- **Student.** Embedding-sum 256x256 two-layer MLP over encoding 4 (1,483 input rows), 452,892 parameters.
  Loss = BCE on `k/N` + 20 x expected regret under the legal softmax at temperature .5. Adam, lr 5e-4 cosine to
  1e-5 over 26,000 updates, batch 1024, l2 1e-6, checkpoint selected by validation regret. Each rung starts from the
  newest promoted net (its `.npz`).
- **Data mix.** All LAD5 rows labeled so far (seeded with LAD4's) plus two fixed strong-teacher piles
  (`labels2-LAD3-train.bin`, `labels2-T160-train.bin`). For the first 40 rungs a distillation pile
  (`STU-distill-LAD2-ens3w4.bin@3000000`, a 5-net ensemble's relabels) was also in the mix; dropping it at rung 40
  was worth −.0006 regret in one rung.
- **Gate.** A new net is promoted iff its Rust-side regret on the current validation file is <= the current teacher
  net's. 57 of 78 candidates were promoted.

## Results (copied from DELTA-EXPECTIMAX.md, "Overnight 2026-10-05"; exploratory tier)

Protocol: paired mirrored deals vs pinned production Walt, same fixed contract (dropped on, bid 30), seed 5000,
4,096 deals unless stated; score = mean over deals of (+1 the net won where production lost, −1 the reverse, 0 same);
95% CI; no multiplicity correction.

| net alone | regret* | h2h vs production, seed 5000 | seed 6000 (2,048 deals) | µs/decision |
|---|---|---|---|---|
| LAD4-k17 (old recipe, 256 MLP) | .0168 | −.060 [−.075, −.044] | −.061 [−.083, −.038] | 4 |
| STU-ft-regret-all (k17 + regret loss, 1.44M rows) | .0140 | −.023 [−.039, −.007] | | 3 |
| STU-s-res512-r1 (residual-LN 512, 3 blocks) | .0126 | −.004 [−.019, +.012] | | 76–94 |
| STU-e-mlp256 (256 MLP distilled from 5-net ensemble) | .0123 | −.004 [−.019, +.011] | +.009 [−.012, +.031] | 4 |
| STU-t-mlp256 (256 MLP, 4-net ensemble incl. tokens) | .0121 | +.007 [−.009, +.022] | | 3 |
| STU-w-mlp256 (256 MLP, res512+tokens+W4 ensemble; LAD5 start) | .0119 | +.010 [−.006, +.025] | | 4 |
| STU-t-res512 (residual, 4-net labels) | .0116 | +.012 [−.003, +.027] | | 62 |
| STU-tokens-b2-d48L2 (tile-token transformer, 49.6k params) | .0123 | +.025 [+.010, +.041] | | 64 |
| STU-tokens-b3-d48L2 (one more chained round) | .0117 | +.021 [+.006, +.036] | +.019 [−.003, +.041] | 40–65 |
| STU-scale-W2 (512x512 MLP, 7.5M rows, wd .3) | .0117 | +.001 [−.013, +.016] | | 8 |
| STU-scale-W4-res512ln-40k (residual-LN 512, 7.5M rows, wd .3†) | .0107 | +.029 [+.014, +.044] | **+.031 [+.010, +.052]** | 44–55 |
| STU-r3-tok-b8 (tokens-b3 + 5 rounds on LAD5 rows) | .0106 | +.030 [+.014, +.045] | | 64 |
| STU-r3-W4c (W4 continued on LAD5 rows) | .0105 | +.020 [+.005, +.035] | | 76 |

\*Regret is Rust-side on each designer's validation snapshot (LAD4-val, STU-snap, r3-snap); comparable within a
block, not across the whole column. †W4's saved arguments and `reports/student-scale.md` say weight decay .1; the
source table says .3 (copied as is; see "Known issues").

LAD5 rungs (regret on the current validation file; alone = net with no search vs production, seed 5000, 4,096 deals):

| rung | regret (current val) | alone vs production | note |
|---|---|---|---|
| start (STU-w-mlp256) | .01207 | +.010 [−.006, +.025] | |
| k0 | .01180 | +.017 [+.002, +.032] | first rung clears zero |
| k25 | .01125 | +.033 | |
| k39 | .01091 | +.032 | alone flat ≈+.03 since k25 |
| k40 | .01032 | +.044 | distillation pile dropped from the train mix |
| k52 | .01029 | +.054 [+.039, +.068] | |
| k73 | .01000 | **+.056 [+.041, +.071]** | best alone of the night |
| k77 (last) | .01000 | +.046 | alone flat ≈+.05 since k51 |

k73 is the maximum over about 57 promoted nets scored on the same seed and deals; the plateau since k51 (≈+.05) is
the less biased reading. Inside the 160-world search the student is capped near +.10 whatever net sits inside
(LAD5 nets +.090 to +.110; full table in the source section 3).

## Reproduce LAD5

Requirements: the Rust toolchain and clang++ (no network: `cargo build --offline` with the vendored `walt-fork/`),
Python 3.12 with numpy and mlx (Apple silicon), the cap wrapper `../astra-sol-20261004/tools/run_capped.py`, the
start net `models/STU-w-mlp256.w`, and the label files `data/labels2-LAD4-{train,val}.bin`,
`data/labels2-LAD3-train.bin`, `data/labels2-T160-train.bin` (not part of the release: about 12 GB; they are
regenerated with `ladder log2`, see `RUNBOOK.md`). Build, then run the pipeline; rerunning the same command resumes
from `pipeline/LAD5/state.json`:

```sh
(cd ladder && CARGO_TARGET_DIR=$PWD/target-c cargo build --release --offline)
LADDER_BIN=$PWD/ladder/target-c/release/ladder python3 pipeline.py --name LAD5 --net models/STU-w-mlp256.w \
  --train-base data/labels2-LAD4-train.bin --val-base data/labels2-LAD4-val.bin \
  --ladder --gate --gate-metric regret --trainer stu --hidden 256 --hidden2 256 \
  --label-workers 8 --outer 160 --inner 8 --seed-base 45000000 --rounds 80 --nets 80 \
  --extra-train data/labels2-LAD3-train.bin,data/labels2-T160-train.bin
```

This is the final recipe. The historical run differed in two ways: its first 40 rungs also had
`data/STU-distill-LAD2-ens3w4.bin@3000000` first in `--extra-train`, and it passed `--h2h-search
--h2h-search-outer 160` for most rungs. The student's optimization arguments are `pipeline.py`'s default
`--stu-args`. Labeling is deterministic by seed; MLX training on the GPU is not bit-reproducible (below), so
a rerun gives a statistically similar ladder, not the same files.

## Code map

| file | role |
|---|---|
| `train.py` | the one trainer: `--arch mlp|res|tok`, `--loss bce|mix|tce|soft|regret`, `--opt adam|adamw|mlx-adamw`, `--init` (.npz or any .w), `--resume/--state`, `--relabel`, `--eval-only --dump-logits`. `python3 train.py -h` |
| `train_stu.py`, `train_tok.py`, `train_scale.py` | deprecated shims: print a line and call `train.py` with the old defaults |
| `pipeline.py` | the label, train, gate, h2h loop (`--trainer stu` = the LAD5 student) |
| `check_parity.py` | Rust vs trainer parity on 100 val rows for one net per weight-file kind |
| `h2h_fast.py`, `h2h.py` | paired mirrored head-to-head vs production |
| `log2_launch.py`, `label_turbo_launch.py` | capped labeling launchers |
| `rust_regret.py`, `build_distill.py` | Rust-side regret from `agree --dump`; ensemble distillation files |
| `ladder/src/net.rs` | weight-file header table, the only loader dispatch, MLP forward (NEON + generic) |
| `ladder/src/resnet.rs`, `ladder/src/stu_tokens.rs` | residual and tile-token bodies |
| `ladder/src/main.rs`, `ladder/src/turbo.rs`, `ladder/csrc/turbo.cpp` | CLI (`log2`, `h2h`, `agree`, `netbench`, ...), the search engine binding and engine |
| `train_belief.py`, `ladder/src/belief.rs` | belief head trainer and Rust loader/forward, exact uniform-support marginals, world weights (exploratory; STATUS.md "Belief head and belief-weighted worlds") |
| `belief_compare.py`, `belief_pool.py` | paired arm-vs-arm h2h differences on the same deals (divergence-trick split, ESS), pooled over seeds |
| `walt-fork/` | the vendored Walt crate the binary links |

Environment switches that remain: `LADDER_NET_GENERIC=1` (scalar MLP loops; bit-identical to NEON, kept as the
reference check), `LADDER_LEAF_MEMO=0|1|2` (search leaf memo; 1 = default), `LADDER_CXXFLAGS` (extra flags for the
C++ engine build only), `LADDER_BIN` (binary for the Python scripts). `LADDER_NET_FMA` and `LADDER_NB_STAGE` were
removed (the first was not output-identical and measured no gain; the second was a timing-only stage cut).

**Determinism.** MLX on the GPU is not run-to-run deterministic (two identical runs of the old trainer differed in
81% of weights by up to .03; validation regret moved in the fifth decimal). On the CPU device (`--device cpu`) it is,
and the merged `train.py` reproduces the old `train_stu.py` byte for byte there (`.w` and `.npz`), including the
LAD5 path (regret loss, cosine lr, `--init` from `.npz` and from `.w`) and a residual-LN path.

**Parity check** (`check_parity.py`, under the cap, under a second): Rust `agree --dump` and the trainer's forward
(CPU) on `fixtures/labels2-LAD4-val-first100.rec` (the first 100 rows of `labels2-LAD4-val`) for the four release
models, seven older files when present (MLP encodings 1-4, residual-LN, tokens) and eight synthetic random-weight
nets that cover every header kind from a release checkout. Tolerances: |agreement difference| <= .01 and |regret
difference| <= .001. Result on 2026-10-05: 19 of 19 pass with every pick matching; the largest legal-logit
difference is 6.9e-5 (a synthetic residual net), 2e-5 or less on trained nets.

## Weight-file format (`.w`)

A file is a little-endian u32 header, then little-endian f32 arrays in the order below, nothing else. Matrices are
row-major `[in][out]`. The loader rejects any file whose size differs from what the header implies. The header kind
is decided by its first words, in this order:

| kind | header (u32 words) | condition | arrays after the header |
|---|---|---|---|
| tokens | `[9, 1, d, L, heads, dff, nf=20]` | w0 = 9 and w1 = 1 | `Wf[20][d] E[196][d] Wc[28][d] bc[d]`, per layer `g1 be1[d] Wq Wk Wv Wo[d][d] bo[d] g2 be2[d] W1[d][dff] b1[dff] W2[dff][d] b2[d]`, then `gf bf hw[d] tb[28]` |
| residual | `[4, 3, H, inner, blocks, ln]` | w0 = 4 and w1 = 3 | `w1[1483][H] b1[H]`, per block `[g be[H] if ln] wa[H][inner] ba[inner] wb[inner][H] bb[H]`, then `[gf bef[H] if ln] w2[H][28] b2[28]` |
| MLP v2-v4 | `[enc, layers, H, H2]` | w0 in 2..4 and w1 <= 2 | `w1[INPUTS(enc)][H] b1[H]`, if layers = 2 `wm[H][H2] bm[H2]`, then `w2[Hout][28] b2[28]` (Hout = H2 for two layers, H for one) |
| MLP v1 | `[layers, H]` | w0 in 1..2 | as MLP with encoding 1 |
| belief head | `[10, 1, enc, layers, H, H2]` | w0 = 10 and w1 = 1 (checked before residual) | MLP trunk as above (`w1[INPUTS(enc)][H] b1[H]`, if layers = 2 `wm[H][H2] bm[H2]`), then `w3[Hout][84] b3[84]`; not a policy (`Net::load` rejects it; `belief::BeliefNet` loads it) |

**Belief head** (`ladder/src/belief.rs`, `train_belief.py`; exploratory, used only by the `l2b:` arm): same input as the
policy; output `z[3t + k]` = logit that relative seat k (mover+1+k, key frame) holds tile t; probabilities = softmax over
the seats that can hold t (not publicly void in its suit, still holding a tile), only for tiles not in the mover's
hand and not played. Training/eval records (`ladder belief-log`, `belief-from-games`): 128 bytes = the 100-byte label
record (N = 0, choice = tile played, prod = the net's greedy pick or 255) + the three relative seats' true remaining
hands (u32 each) + deal seed (u64) + eps-random flag (u8) + trick (u8) + 6 zero bytes (`train_belief.BREC`).

`INPUTS(enc)`: 215 (encoding 1), 1399 (2), 11003 (3), 1483 (4). Files from other forks with other headers (encoding 5,
"roles" heads, the earliest one-word `[H]` level-0 files) are rejected; ten such files sit in `models/`.

**State and frame.** Tiles are indexed 0..27 in the order (0,0) (1,0) (1,1) (2,0) (2,1) (2,2) ... (6,6) (high pip,
low pip). Seats are in the key frame where odd seats are the declaring side; the mover "maximizes" iff its seat is
odd. A tile's suit under declaration `decl` (trump pip 0..6) is 7 (trump) if either pip equals `decl`, else its high
pip.

**Encoding 4 (MLP and residual first layer).** Active rows, summed into `b1`:
for each tile t, row `decl*196 + t*7 + s` with s = 0 in own hand, 1 unseen, 2 played in an earlier trick, 3+p played
at position p of the current trick; with c = 1372: `c + decl`; `c + 7 + (leader − mover) mod 4`; `c + 11` if the mover
maximizes; `c + 12 + plays in the current trick`; `c + 19 + led suit` (0..6 pip, 7 trump) when a tile has been led;
and for each other seat k = 0..2 (mover+1+k) and each tile it is publicly void of, row `1399 + 28k + t`. Three scalar
rows are added with weights: `w1[c+16] * banked_t1/42`, `w1[c+17] * banked_t0/42`, `w1[c+18] * hand_size/7`. Then
`h = relu(sum)`. Encodings 1-3 differ only in the tile-row index (1: `t*7 + s`, no declaration; 3: `decl*1568 + t*56 +
s*8 + void pattern of the three other seats`), the context base c (196, 1372, 10976) and the absence of void rows
(1-3) and of the led-suit row (1).

**MLP.** `h2 = relu(h wm + bm)` for two layers; `z = h2 w2 + b2`.
**Residual.** Per block: `x = ln ? LN(h; g, be) : h`; `h = h + relu(x wa + ba) wb + bb`. Then
`z = (ln ? LN(h; gf, bef) : relu(h)) w2 + b2`. LN uses the biased variance and eps 1e-5.
**Tokens.** For each tile t, 20 features: state one-hot (7, as above), seat relative to the mover of whoever played t
in the current trick (3 flags, 1..3), the three other seats' void flags for t (only when voids are known), legal, trump,
follows the led suit, double, high pip/6, low pip/6, count/2 (5-count tiles 1, 10-count tiles 2). Context, 28 wide:
declaration one-hot (7), leader relative to the mover (4), maximize, plays in the trick one-hot (4, capped at 3), led
suit one-hot (9: pips, trump, none), banked_t1/42, banked_t0/42, hand_size/7. Token input
`x_t = F_t Wf + E[decl*28 + t] + (C Wc + bc)`. Each of L layers: `h = LN(x; g1, be1)`; multi-head attention over all
28 tokens (heads of width d/heads, scores scaled by (d/heads)^−0.5, softmax, no mask); `x += concat(heads) Wo + bo`;
`h = LN(x; g2, be2)`; `x += relu(h W1 + b1) W2 + b2`. Logit `z_t = LN(x_t; gf, bf) · hw + tb[t]`.

**Selector.** Among legal tiles, the highest `z` if the mover maximizes, else the lowest; ties go to the lowest tile
index. `sigmoid(z_t)` estimates the declarer-make probability after playing t.

**Label files** (`data/*.bin`, not released) are 100-byte records: decl, seat, maximize, leader, npl (u8 each),
plays[4] (u8, 255 = empty), banked_t1, banked_t0 (u8), hand, played, legal (u32 masks), N (u16 worlds),
counts[28] (u16 declarer-make counts), choice (u8, the teacher's pick), prod (u8, production's pick), hasvoids (u8),
voids[4] (u32 masks). `train.py`'s `REC` is the authoritative dtype.

## Models in the release

| file | kind | params | bytes | µs/decision | alone vs production (seed 5000) | card |
|---|---|---|---|---|---|---|
| `models/LAD5-k73.w` | MLP enc 4, 256x256 | 452,892 | 1,811,584 | 3-4 | +.056 [+.041, +.071] | `models/LAD5-k73.MODEL_CARD.md` |
| `models/STU-scale-W4-res512ln-40k.w` | residual-LN 512, 2 blocks | 1,827,868 | 7,311,496 | 44-55 | +.029 [+.014, +.044] | `models/STU-scale-W4-res512ln-40k.MODEL_CARD.md` |
| `models/STU-tokens-b3-d48L2.w` | tokens d48, 2 layers | 49,564 | 198,284 | 40-65 | +.021 [+.006, +.036] | `models/STU-tokens-b3-d48L2.MODEL_CARD.md` |
| `models/STU-r3-tok-b8.w` | tokens d48, 2 layers | 49,564 | 198,284 | 58-64 | +.030 [+.014, +.045] | `models/STU-r3-tok-b8.MODEL_CARD.md` |

Each has a `.json` with its training arguments and history. µs/decision is the median net decision time inside
the h2h harness on this machine (Apple silicon, one core).

## Known issues and TODO

- `STU-scale-W4-res512ln-40k.w` holds the last weights of its schedule (step 40,000), not the best-regret checkpoint
  (step 36,000, `-best.w`, not released); the old scale trainer named files that way. The release numbers are for the
  released file.
- MLX GPU training is not bit-reproducible; only the CPU device is.
- The ten unloadable files in `models/` (formats from other forks) are kept unchanged; loading `[H]`-header level-0
  files was never supported by this binary.


**Independent-seed replicate (added 2026-10-05 14:40 CDT):** seed 6000, 4,096 paired deals, fresh production memo: **+.058 [+.043, +.073]**, win rate .529 (`results/h2h-np-LAD5-k73-s6000-4096/summary.json`). This is a single pre-registered read of LAD5-k73, so it carries none of the best-of-57 selection bias of the seed-5000 number.


## Published artifacts

- Models (CC0): https://huggingface.co/jasonyandell/texas-42-walt-tiny-net-ladder
- Labels dataset (CC0): https://huggingface.co/datasets/jasonyandell/texas-42-walt-ladder-labels
