# student-scale: is the 64% / -.06 plateau an optimization artifact?

Designer: SCALE AND OPTIMIZATION. Worktree (code only): `/Users/jason/Documents/Codex/2026-10-04/task-11/research-worktree/.claude/worktrees/agent-a67d09aabae674c6d/experiments/fable-rust-ladder-20261004` (`train_scale.py`, `tools/ceiling.py`, `tools/curves.py`; a copy of `ladder/` built into `ladder/target-x`; `net.rs` unchanged: the existing `[4,3,H,inner,blocks,ln]` residual header from `resnet.rs` was reused, so there is no new Rust code and old files still load). Tier: exploratory, below every tier (single seed, val used to pick among ~12 hyperparameter settings, h2h = 4096 paired deals vs production, exploratory, no multiplicity correction).

## Findings (5 lines)

1. **Yes, the plateau was an optimization/regularization artifact; the student is not the cap.** From scratch on 7.5M rows of 160/350-world labels, a 3-layer residual-LN 512 student (1.83M params, 40k updates, AdamW wd 0.1, cosine, BCE weight 3 + label-smoothed CE) reaches **val agreement .6847 / regret .0107** (Rust-side agreement .6844, regret from Rust scores .0107) vs LAD4-k17 .6478 / .0167, and **net-alone h2h +.029 [+.014,+.044]** vs production (k17: -.060) at 45 us/decision (k17 2.6 us). Even a 256x256 net on 1.5M rows with the new recipe beats k17 (.652, regret .0159).
2. **The earlier from-scratch runs over-fitted**: 256x256 on 1.5M rows with the old recipe (no decoupled wd, BCE weight 1) gets train .74 vs val .627; val peaks at ~15k updates then declines and val BCE rises from update 5k. What fixed it: (a) strong decoupled weight decay (wd .3 at lr 2e-3; wd 1.0 is too strong), (b) BCE weight 3 (value regression generalizes, the argmax-CE aux alone hurts regret), (c) more rows: with 7.5M rows the train-val gap is .02 and the model is steps/capacity-limited (val still improving at the end).
3. **Width and depth help only once regularized and fed enough data**: mlp512 1.5M rows .664/.0144 (train .73); 7.5M rows 40k updates .664/.0129, 60k updates .675/.0117; res512-LN 24k updates .674/.0116, 40k updates .685/.0107. res512 on 1.5M rows over-fits (train .80 vs val .647).
4. **Label-ambiguity ceiling** (LAD4 val, 350 worlds): 24.3% of rows have two or more tied best tiles, 31.9% have gap <= 1 world, 38.3% <= 2, 51.9% <= 5. An oracle that knows the exact label probabilities, scored against an independent fresh 350-world draw, agrees only **.78** (pessimistic: independent binomial per tile ignores common-world correlation); strict argmax agreement is a coarse metric, regret and zero-regret rate are the right ones (W4 zero-regret .701).
5. **Agreement is not strength; regret tracks h2h**: W1 (regret .0129) -.027, W2 (.0117) +.001, W3 (.0116) +.017, W4 (.0107) +.029. The residual error sits in rows with a clear best (gap >= 9 worlds, 39% of rows; W4 agree .785, regret .0178 within the stratum). Recommendation: adopt the recipe (AdamW wd .1-.3, BCE weight 3, eps .1, cosine to lr/100, 7.5M rows incl. LAD1/LAD2 160-world piles, select on regret) and choose the shape by cost: **mlp 512x512 (8 us) is parity alone (+.001); res512-LN (45 us, generic non-NEON Rust loops) is +.029**; a NEON residual kernel would cut the 45 us.

## Variants (all from scratch; enc 4; val = labels2-LAD4-val.bin, 45,850 rows; train agree measured on a fixed 32,768-row subset of LAD4-train = same distribution as val; headline numbers = last update, no early-stop selection)

Rows: 1.52M = LAD4-train (350w, 499k) + LAD3-train (756k) + T160-train (269k). 7.52M = those + 4.0M of LAD2-train + 2.0M of LAD1-train (stride-sampled; all 160-world labels).

| variant | arch / params | rows | updates (batch 2048) | recipe | train agree | val agree (trainer) | Rust agree | val regret | h2h alone vs prod (4096 paired) | us/decision |
|---|---|---|---|---|---|---|---|---|---|---|
| baseline LAD4-k17 | mlp 256x256 / 453k | n/a (chain) | n/a | pipeline | n/a | n/a | .6478 | .0167 | -.060 [-.075,-.044] | 2.6 |
| A-mlp256 | mlp 256x256 / 453k | 1.52M | 34,108 of a 120k cosine (cap-stopped) | lr 2e-3, wd .02, bw 1 | .740 | .627 (peak .635 @15k) | - | .0188 (best .0184) | not run | - |
| S0-base | mlp 256x256 | 1.52M | 15k | wd .02, bw 1 | .708 | .638 | - | .0186 | not run | - |
| S-wd3b3e | mlp 256x256 | 1.52M | 15k | wd .3, bw 3, eps .1 | .677 | .652 | - | .0159 | not run | k17 shape (~2.6) |
| V1-mlp512 | mlp 512x512 / 1.04M | 1.52M | 15k | wd .3, bw 3, eps .1 | .730 | .664 | - | .0144 | not run | 10.5 |
| V2-res512 | res LN 512/512 x2 / 1.83M | 1.52M | 11.8k of 12k | lr 1e-3, wd .3, bw 3, eps .1 | .801 | .647 (peak .651 @8k) | - | .0158 (best .0155) | not run | ~44 |
| **W1-mlp512-big** | mlp 512x512 / 1.04M | 7.52M | 40k | wd .3, bw 3, eps .1 | .671 | .664 | .6639 | .0129 (Rust .0129) | **-.027 [-.041,-.012]** | 10.0 (h2h median 12) |
| **W2-mlp512-long** | mlp 512x512 / 1.04M | 7.52M | 60k | wd .1, bw 3, eps .1 | .693 | .675 | .6746 | .0117 (Rust .0117) | **+.001 [-.013,+.016]** | 8.1 (h2h median 9) |
| **W3-res512ln** | res LN 512/512 x2 / 1.83M | 7.52M | 24k | lr 1e-3, wd .1, bw 3, eps .1 | .692 | .674 | .6738 | .0116 (Rust .0117) | **+.017 [+.003,+.032]** | 44.7 (h2h median 52) |
| **W4-res512ln-40k** | res LN 512/512 x2 / 1.83M | 7.52M | 40k | same as W3 | .708 | **.685** | **.6844** | **.0107** (Rust .0107) | **+.029 [+.014,+.044]** | 43.5 (h2h median 45) |

Trainer vs Rust `ladder agree` differences: W1 .0000, W2 .0001, W3 .0000, W4 .0003 (limit .01). Models: `MAIN/models/STU-scale-<variant>.w/.json` (the `-best.w` files are the best-regret eval point and are not used for the headline numbers).

Recipe sweep on the 256x256 / 1.52M / 15k-update schedule (val agree / regret; single seed, regret differences under ~.002 are noise): base (wd .02, bw 1) .638/.0186; eps .1 .637/.0182; CE temperature tau .3 .627/.0202 (worse); bw .3 .629/.0197; **bw 3 .635/.0174**; bw 10 .604/.0181 (agreement collapses: the CE aux is what ties the argmax); bw 3 + dropout .2 + wd .05 .641/.0170; bw 3 + wd .3 **.652/.0162**; bw 3 + wd .3 + dropout .2 .641/.0173; bw 3 + wd 1.0 .611/.0208 (too strong); bw 3 + wd .3 + eps .1 **.652/.0159**.

## Per-round curves (global update | train agree | val agree | train regret | val regret | train BCE | val BCE); rounds = capped invocations chained with `--resume` (params + both Adam moments + step carried; lr schedule keyed to the global step)

A-mlp256 (over-fits: val BCE rises from the first eval; train agree climbs to .74):

| update | train agree | val agree | train regret | val regret | train BCE | val BCE |
|---|---|---|---|---|---|---|
| 5000 | .6527 | .6210 | .0179 | .0203 | .5196 | .5306 |
| 10000 | .6935 | .6321 | .0140 | .0190 | .5220 | .5370 |
| 15000 | .7109 | .6346 | .0123 | .0184 | .5242 | .5425 |
| 20000 | .7209 | .6300 | .0118 | .0186 | .5238 | .5451 |
| 25000 | .7297 | .6311 | .0113 | .0187 | .5280 | .5520 |
| 30000 | .7339 | .6272 | .0109 | .0191 | .5297 | .5550 |
| 34108 | .7400 | .6269 | .0105 | .0188 | .5290 | .5560 |

V1-mlp512 (1.52M rows, regularized; still a .066 train-val gap):

| update | train agree | val agree | train regret | val regret | train BCE | val BCE |
|---|---|---|---|---|---|---|
| 5000 | .6709 | .6416 | .0140 | .0167 | .4767 | .4878 |
| 10000 | .7111 | .6587 | .0108 | .0151 | .4690 | .4853 |
| 15000 | .7296 | .6639 | .0094 | .0144 | .4647 | .4842 |

W2-mlp512-long (7.52M rows; round boundaries at 32,579 and 60,000):

| update | train agree | val agree | train regret | val regret | train BCE | val BCE |
|---|---|---|---|---|---|---|
| 6000 | .6300 | .6245 | .0167 | .0168 | .4766 | .4818 |
| 12000 | .6479 | .6429 | .0138 | .0146 | .4717 | .4770 |
| 18000 | .6600 | .6530 | .0131 | .0137 | .4674 | .4729 |
| 24000 | .6665 | .6591 | .0124 | .0133 | .4658 | .4716 |
| 32579 | .6754 | .6647 | .0116 | .0125 | .4629 | .4692 |
| 42000 | .6838 | .6697 | .0107 | .0120 | .4612 | .4678 |
| 54000 | .6910 | .6740 | .0103 | .0117 | .4591 | .4666 |
| 60000 | .6926 | .6747 | .0102 | .0117 | .4588 | .4666 |

W4-res512ln-40k (7.52M rows; round boundaries at 21,982 and 40,000):

| update | train agree | val agree | train regret | val regret | train BCE | val BCE |
|---|---|---|---|---|---|---|
| 4000 | .5989 | .5970 | .0188 | .0190 | .4789 | .4844 |
| 8000 | .6454 | .6391 | .0142 | .0150 | .4699 | .4756 |
| 12000 | .6576 | .6528 | .0127 | .0136 | .4657 | .4717 |
| 16000 | .6697 | .6594 | .0116 | .0127 | .4633 | .4691 |
| 21982 | .6810 | .6711 | .0108 | .0118 | .4602 | .4654 |
| 28000 | .6911 | .6784 | .0101 | .0112 | .4567 | .4632 |
| 32000 | .7013 | .6828 | .0093 | .0109 | .4555 | .4636 |
| 36000 | .7053 | .6847 | .0090 | .0106 | .4545 | .4632 |
| 40000 | .7079 | .6847 | .0089 | .0107 | .4543 | .4633 |

W1, W3, V2 and sweep curves are in each `models/STU-scale-*.json` `history`. Reading: with 7.5M rows the train-val gap ends at ~.02 (agree) in a still-improving regime; with 1.5M rows it is .06-.08 and val stalls or turns, i.e. over-fitting. The two earlier "plateau" observations are consistent with too little data/regularization for the capacity (large nets) and short constant-lr schedules (small ones).

## Ceiling and error structure (tools/ceiling.py on labels2-LAD4-val.bin; 45,850 rows, N=350, mean 3.93 legal tiles; `choice` = lowest argmax tile verified)

| quantity | value |
|---|---|
| rows with >= 2 tied best tiles | .2426 |
| gap to runner-up <= 1 / 2 / 3 / 5 / 10 worlds | .319 / .383 / .434 / .519 / .659 |
| oracle-on-p_hat vs fresh independent 350-world label: agree | .781 (pessimistic; regret .0047) |

Per-gap strata, k17 vs W4 (Rust scores):

| gap (worlds) | share | k17 agree | k17 regret | W4 agree | W4 regret |
|---|---|---|---|---|---|
| 0 (tied) | .243 | .793 | .00329 | .845 | .00202 |
| 1 | .077 | .396 | .00693 | .385 | .00583 |
| 2-3 | .114 | .445 | .01012 | .446 | .00776 |
| 4-8 | .178 | .529 | .01350 | .529 | .01109 |
| >= 9 | .388 | .721 | .03046 | .785 | .01780 |

Scale's gain is mostly in clear-best rows (gap >= 9: regret .0305 -> .0178) and in tied rows (the lowest-tile tie-break gets learned: .79 -> .85). Near-ties (gap 1-3) stay at about .4 agreement for both and are the irreducible part of the agreement number; tied-row agreement is partly a tie-break artifact that regret ignores.

## Exact commands

All training/eval ran under `run_capped.py --seconds 295`, foreground. `P` = `/Users/jason/.local/share/mise/installs/python/3.12/bin/python3`; data under MAIN/data.

`train_scale.py` (my worktree) flags: `--arch mlp1|mlp2|res --hidden H --hidden2 H2 --inner I --blocks B --ln 0|1 --total <global updates> --lr --lr-end --warmup --wd <AdamW decoupled, matrices only> --bw <BCE weight> --eps <CE label smoothing> --tau --dropout --loss mix|tce --resume --seconds 262`. One capped invocation = one round; `--resume` reloads params, both Adam moments and the step from the checkpoint `<name>.state.npz` (kept in the session scratchpad, not MAIN). `@K` after a file name = K rows by fixed stride.

Data lists: 1.52M = `labels2-LAD4-train.bin,labels2-LAD3-train.bin,labels2-T160-train.bin`; 7.52M = those + `labels2-LAD2-train.bin@4000000,labels2-LAD1-train.bin@2000000`. Always `--val labels2-LAD4-val.bin --val2 labels2-LAD3-val.bin,labels2-T160-val.bin`.

- W4 (2 rounds; second adds `--resume`): `P train_scale.py --train <7.52M> --val ... --name STU-scale-W4-res512ln-40k --arch res --hidden 512 --inner 512 --blocks 2 --ln 1 --total 40000 --eval-every 4000 --warmup 1500 --lr 0.001 --lr-end 0.00001 --wd 0.1 --bw 3 --eps 0.1 --batch 2048 --seconds 262`
- W3: same with `--name STU-scale-W3-res512ln --total 24000`.
- W2 (2 rounds): `--arch mlp2 --hidden 512 --hidden2 512 --total 60000 --eval-every 6000 --warmup 2000 --lr 0.002 --lr-end 0.00002 --wd 0.1 --bw 3 --eps 0.1`.
- W1 (2 rounds): same as W2 with `--total 40000 --eval-every 4000 --warmup 1500 --wd 0.3`.
- V1/V2/A/S-* use the 1.52M list: V1 `--arch mlp2 --hidden 512 --hidden2 512 --total 15000 --lr 0.002 --wd 0.3 --bw 3 --eps 0.1`; V2 `--arch res --hidden 512 --inner 512 --blocks 2 --ln 1 --total 12000 --lr 0.001 --wd 0.3 --bw 3 --eps 0.1`; A `--arch mlp2 --hidden 256 --hidden2 256 --total 120000 --warmup 2000 --lr 0.002 --lr-end 0.00002 --wd 0.02` (one round, cap-stopped at 34,108); S-* `--total 15000` with the flags in the sweep line.
- Rust agreement and regret: `ladder/target-x/release/ladder agree --net MAIN/models/STU-scale-<v>.w --labels MAIN/data/labels2-LAD4-val.bin --limit 100000 --dump <f>`, then `tools/ceiling.py <val.bin> <f>` (regret and strata from the Rust scores).
- h2h: `cd MAIN && LADDER_BIN=<worktree>/ladder/target-x/release/ladder python3 h2h_fast.py --net np:MAIN/models/STU-scale-<v>.w --tag np-STU-scale-<v>-s5000-4096 --deals 4096 --workers 1 --prod-cache MAIN/results/prod-cache-s5000` (each finished in one round, 16-24 s; no `--voids`, it only changes the production wire that the memo pins). Results: `MAIN/results/h2h-np-STU-scale-{W1-mlp512-big,W2-mlp512-long,W3-res512ln,W4-res512ln-40k}-s5000-4096/summary.json`.
- us/decision: `ladder netbench --net X.w --iters 100000` (machine shared with other jobs): k17 2.65, V1 10.5, W1 10.0, W2 8.1, W3 44.7, W4 43.5. The residual nets run the generic (non-NEON) loops in `resnet.rs`; the plain mlp path uses the NEON kernels in `net.rs`.

## Caveats

- Single seed per configuration; ~12 settings were compared on the one LAD4-val file, so headline val numbers carry mild selection. h2h: fixed s5000 deals, exploratory, no multiplicity correction. W2 (+.001) vs W4 (+.029): CIs do not overlap; W3 (+.017) vs W4 overlap.
- The big pile includes older-net-played states (LAD1/LAD2, 160-world labels) from a different state distribution than LAD4's; LAD3-val and T160-val are consistent with the LAD4-val numbers (see `extra_val` in the jsons), but I did not ablate rows vs steps vs width cleanly, so "more rows" and "more updates" are confounded in W1/W2 vs V1.
- Not tried: width 1024, more than 40k updates for the residual net, dropout inside the residual net, plain 3-4 layer MLP header, regret-weighted loss (another designer's lane), seeds.
- No status or tier changes. Agreement of W3/W4 above the k17 pipeline teacher agreement (.648) is against labels, not against the teacher net.
