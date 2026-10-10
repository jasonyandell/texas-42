# STU tile tokens (designer report)

Worktree: `/Users/jason/Documents/Codex/2026-10-04/task-11/research-worktree/.claude/worktrees/agent-aa71557d89a5fd769` (experiment copy at `experiments/fable-rust-ladder-20261004/` inside it; the worktree had no copy of the experiment, so I copied `ladder/{src,csrc,Cargo.*,build.rs}`, `walt-fork/`, `gym/`, `train.py` into it). Tier: exploratory conformance receipts only (rob-tier evidence, no status change).

## Design (5 lines)
1. 28 tile tokens; token = Wf.f(t) + E[decl,t] + broadcast context vector. f(t) = 20 features: state one-hot (own/unseen/played/table pos 0-3), relative seat of the table-tile player (3), void flags of the 3 other seats (3), legal, trump, follows-led-suit, double, hi/lo pips, count value. Context (28 dims): decl, leader-rel, maximize, npl, led suit, banked t1/t0, hand size.
2. L pre-LN transformer blocks (multi-head self-attention over the 28 tokens + ReLU FFN), no BLAS.
3. Output: shared head on each token + per-tile bias -> one logit per tile; selector unchanged (argmax/argmin over legal, lowest on ties).
4. Loss `mix` (BCE on k/N + legal-softmax CE), AdamW, linear warmup + cosine, rounds chained with `--init` (npz checkpoint).
5. Rust (`ladder/src/stu_tokens.rs`, header `[9,1,d,L,heads,dff,20]`, dispatched in `Net::load`): const-generic plain loops with `mul_add`; the last layer runs queries only for the legal tokens (K/V still over all 28), which is exact for the legal logits. Existing nets and `train.py` untouched.

## Results (val = labels2-LAD4-val.bin, 45,850 rows)
Baseline LAD4-k17 measured the same way: Rust agree .6478, regret .01672, h2h alone -.060 [-.075,-.044], 4 us.

| variant | shape | params | train rows | trainer agree | Rust agree | regret (val) | h2h alone vs prod, 4096 deals (95% CI) | us/decision (netbench; machine load 20+) |
|---|---|---|---|---|---|---|---|---|
| a-d32L1 | d32 L1 h2 ff64 | 16,412 | 499k (LAD4) | .6184 | .6183 | .01757 | -.0464 [-.0623,-.0300] | 9 (h2h median 11) |
| d-d32L1 | same | 16,412 | 1.52M | .6085 | .6086 | .01915 | not run | ~9 |
| c-d32L2 | d32 L2 h2 ff64 | 24,860 | 1.52M | .6339 | .6340 | .01519 | -.0090 [-.0242,+.0066] | 37-48 (h2h median 46) |
| b-d48L2 | d48 L2 h4 ff96 | 49,564 | 1.52M | .6516 | .6517 | .01380 | +.0061 [-.0095,+.0220] | 73 (h2h 260 before kernel rewrite) |
| b2-d48L2 (b + 15k upd, lr 1e-3) | same | 49,564 | 1.52M | .6706 | .6705 | .01225 | +.0251 [+.0103,+.0408] | h2h median 64 |
| b3-d48L2 (b2 + 15k upd, lr 7e-4) | same | 49,564 | 1.52M | .6816 | .6814 | .01174 | +.0210 [+.0059,+.0359] | 65 (h2h median 61) |

Rows = LAD4-train + LAD3-train + T160-train concatenated (1,524,616; variant a only LAD4-train). Regret = mean over val rows of (k_teacher_choice - k_predicted)/N on the declarer-make scale, sign-flipped for minimizers (>= 0 by the teacher's own labels, lower is better). Models: `models/STU-tokens-{a-d32L1,d-d32L1,c-d32L2,b-d48L2,b2-d48L2,b3-d48L2}.{w,json,npz}`. h2h results: `results/np-STU-tokens-*-s5000-4096*`.

Parity: Rust logits equal a float64 numpy forward to <=1.5e-5 on 100 val rows for a, c, b3 (the trainer's MLX Metal logits deviate from exact by 0.05-0.09 on the same rows, so the trainer's agreement numbers differ from Rust's by <=.0003; the Rust number is the one to quote). `ladder agree` over all 45,850 rows matches trainer agree to <=.0003 for every row of the table.

## Findings
- The 64% plateau is not a data/width limit of the embedding-sum MLP: with token interaction the same labels reach .68 selector agreement and regret .0117 (baseline .0167, -30%), and net-alone play goes from -.060 to +.021..+.025 vs production (CIs exclude 0 for b2/b3). Per-tile token features plus attention is the thing; 1 layer d32 is not enough (a: .618, still -.046), depth and width both matter (c .634, b .652 after the first round).
- Agreement was still rising when I stopped (b -> b2 -> b3: .6516, .6706, .6816). Regret does not map 1:1 to h2h (b3's h2h is not above b2's within noise; a has worse regret than the baseline yet better h2h).
- Mixing 160-world data into the 1-layer d32 model hurt it (a: .618 on LAD4 only vs d: .609 with all data, similar updates); not tested for the larger nets (b..b3 used all data), so the data-mix effect there is confounded with width and rounds.
- Cost: d48 L2 is ~60-75 us/decision (measured while the machine ran load 20-25; unloaded likely 30-40 us), 15x the 4 us baseline and above the 20 us budget; d32 L1 is ~10 us but only reaches .618/-.046. Rust exp() is ~20% of the d32 L2 time (a vectorised exp would help); explicit NEON blocking untried.

## Commands (all training/eval under run_capped.py --seconds 295; PY = /Users/jason/.local/share/mise/installs/python/3.12/bin/python3)
```
cd <worktree>/experiments/fable-rust-ladder-20261004
# build (CPU-heavy, one at a time)
cd ladder && CARGO_TARGET_DIR=<worktree>/experiments/fable-rust-ladder-20261004/ladder/target-x cargo build --release --offline
# train (b: first round; b2/b3 chain with --init <prev>.npz --lr 0.001/0.0007 --warm 200 --seed 7/11 --updates 15000)
run_capped.py --seconds 295 --output-dir <d> -- $PY train_tok.py --train MAIN/data/labels2-LAD4-train.bin MAIN/data/labels2-LAD3-train.bin MAIN/data/labels2-T160-train.bin --name b-d48L2 --d 48 --layers 2 --heads 4 --dff 96 --updates 14000 --eval-every 2000
#   a: --train LAD4-train only --name a-d32L1 --d 32 --layers 1 --heads 2 --dff 64 --updates 50000 (time-capped at 36.9k)
#   c: all three files --name c-d32L2 --d 32 --layers 2 --heads 2 --dff 64 --updates 22000 (time-capped at 20k)
#   d: all three files --name d-d32L1 --d 32 --layers 1 --heads 2 --dff 64 --updates 34000 (time-capped at 31.5k)
# Rust agreement + regret + parity dumps
<worktree>/experiments/fable-rust-ladder-20261004/ladder/target-x/release/ladder agree --net MAIN/models/STU-tokens-b3-d48L2.w --labels MAIN/data/labels2-LAD4-val.bin --limit 100000 --dump out.jsonl
<worktree>/experiments/fable-rust-ladder-20261004/ladder/target-x/release/ladder netbench --net MAIN/models/STU-tokens-b3-d48L2.w --iters 20000
# h2h alone (cwd MAIN)
LADDER_BIN=<worktree>/experiments/fable-rust-ladder-20261004/ladder/target-x/release/ladder $PY h2h_fast.py --net np:MAIN/models/STU-tokens-b3-d48L2.w --tag np-STU-tokens-b3-d48L2-s5000-4096 --deals 4096 --workers 1 --prod-cache results/prod-cache-s5000
```
`train_tok.py` (in the worktree) writes to MAIN/models by absolute path. The regret script and numpy parity script were scratch files in the session scratchpad (definitions above; trivial to recreate).

## Recommendation
Adopt token attention as the student architecture direction: it breaks the plateau (first student design to match/beat production net-alone: b2 +.025 [+.010,+.041], b3 +.021 [+.006,+.036]). The open problem is speed, not accuracy: choose b2/b3 (d48 L2, 49.6k params, ~60 us loaded) if ~15x the baseline's per-decision cost is acceptable for a net-alone player; for use inside searches (4 us today) it needs a cheaper variant. Next steps by value: (1) keep chaining b3 (still improving; 5 min per round); (2) prune: d32 L2 with dff 32, or pooled tile classes to cut the 28x28 attention; (3) vectorised exp + NEON blocking in `stu_tokens.rs`; (4) test LAD4-only vs mixed data for the d48 L2 net; (5) seed-replicate h2h (each is a single 4096-deal run).
Files (in worktree): `ladder/src/stu_tokens.rs`, `ladder/src/net.rs` (header dispatch, `tok` field, `hidden_nonzero` guard), `ladder/src/main.rs` (`mod stu_tokens`), `train_tok.py`.
