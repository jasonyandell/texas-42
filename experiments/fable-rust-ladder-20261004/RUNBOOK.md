# Runbook: rung-2/rung-3 distillation cycle (for an operator agent)

Directory: `experiments/fable-rust-ladder-20261004/` (run everything from here, absolute paths).
Python: `/Users/jason/.local/share/mise/installs/python/3.12/bin/python3` (has numpy + mlx). Call it `$PY`.
Cap wrapper: `../astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir results/<name>-log -- <cmd>`.
Binary: `ladder/target-g/release/ladder` (newest). Export `LADDER_BIN=$PWD/ladder/target-g/release/ladder` for `h2h.py` / `log2_launch.py`.

## 2026-10-05 cleanup (read first)
- One trainer: `train.py` (see `README.md`). `train_stu.py`, `train_tok.py`, `train_scale.py` are deprecated shims that call it.
  The old rung-2 `train.py` recipe below now needs `--select agree` (the new default selects by val regret) and reports
  agreement as `selected.agree` (was `selected.val_agree_hires`); `pipeline.py --trainer train` passes this itself.
- Build new binaries into `ladder/target-c`; the loader change there is output-identical on every loadable file in `models/`.
- `python3 check_parity.py` (under the cap) checks Rust vs trainer logits for every weight-file kind.

## Hard constraints (never relax)
- Every label/train/h2h process tree runs under the 295 s cap wrapper (the launchers already do this). No process > 300 s.
- Local work only: no cloud, no installs, no pushes, no PRs, no Slack/email, no uploads. Do not touch
  `ingest/`, `experiments/tiny-net-ladder-20261004`, `tiny-net-coverage-20261004`, `fable-tiny-net-20261004`, `walt42x-intake-snapshot`.
- Do not rebuild `ladder/target-g` while any process uses it; if you must rebuild, use a new `CARGO_TARGET_DIR=$PWD/ladder/target-h`.
- Commit locally on this branch (`git add experiments/fable-rust-ladder-20261004 ':!experiments/fable-rust-ladder-20261004/data'`), never push.
- Keep at most 18 concurrent ladder processes (16 label workers + 2, or 12 h2h workers + GPU training).

## Protocol facts
- H2H = paired mirrored deals vs pinned production, same fixed contract (dropped on, bid 30), `--seed 5000 --deals 4096`.
  Score = mean over deals of (+1 hybrid won where production lost, −1 reverse, 0 same). Report mean and 95% CI.
- Always pass `--voids` for `np:` arms. Use `--workers 4` for `np:` arms (production side dominates cost, ~2.5 min).
  If a worker times out (summary shows < 4096 deals), rerun the same command with `--round 1` (resumes).
- Rust-side agreement must match the trainer's `val_agree_hires` within ~.01: `ladder agree --net models/X.w --labels data/<val>.bin`.
  If it does not, stop and report (do not h2h a net whose agreement disagrees).

## Cycle (rung 2: net alone as whole player)
1. Merge label parts into train/val (worker 15 files are val):
   `cat data/labels2-mixC-train.bin data/labels2-40-8s2-train.bin > data/labels2-mixD-train.bin` (and val likewise).
2. Train (GPU, ~50–80 s): `$PY $CAP --output-dir results/train-<name>-log -- $PY train.py --train data/labels2-mixD-train.bin --val data/labels2-mixD-val.bin --hidden 128 --hidden2 128 --layers 2 --updates 30000 --batch 1024 --eval-every 250 --name <name> --enc 4 --loss mix --select agree`
   Read `models/<name>.json` -> `selected` (rmse, agree, regret).
3. Agree check (above). 4. H2H: `LADDER_BIN=... $PY h2h.py --net np:models/<name>.w --seed 5000 --deals 4096 --workers 4 --tag np-<name>-s5000-4096 --voids` -> `results/h2h-<tag>/summary.json`.
5. New student-driven states with the new net: `LADDER_BIN=... LOG2_SEED=<fresh, e.g. 9000000> $PY log2_launch.py 8 40 16 2 0.1 <tag> --play np:models/<name>.w` (two capped rounds, ~10 min, ~450k states). Fresh seed per launch, fresh tag per launch.

## Rung 3 (search with the net as modeled others)
- Player arm: `--net l2n:40:models/<name>.w` (3.2 ms/decision; 1,024 deals with 4 workers fits one cap round; 4,096 needs rounds).
- Teacher for Net_3: `log2_launch.py 8 160 16 2 0.1 <tag> --model models/<name>.w` (engine's rung-1 others play the net; 160 outer worlds; full root vectors). Train Net_3 exactly as in the cycle; h2h `np:` and `l2n:40:` of Net_3.

## Reporting
For every net: states, loss, selected update, val RMSE, agreement (trainer and Rust), h2h mean [CI], win rate, µs/decision.
Append rows to `DELTA-EXPECTIMAX.md` section 6 and commit. Never present a timed-out partial h2h without its deal count.

## Faster paths (2026-10-05, see `PERF.md`)
- `ladder/target-s/release/ladder` is output-identical to `target-g` and faster (engine micro-opts, vectorized net forward); `h2h_fast.py` adds the production-decision memo (`--prod-cache`) and automatic resume rounds; `pipeline.py` overlaps labeling, training and h2h. Old scripts and binaries are unchanged.
