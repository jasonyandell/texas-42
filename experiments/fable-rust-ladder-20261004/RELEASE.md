# Release plan (squashed branch and CC0 public release)

Plan only: nothing here has been executed. The exact path list is `release-paths.txt` (repo-relative, one per line,
48 entries, about 17 MB on disk); the recipe below reads it from the committed experiment branch.

## In the release

- **Docs:** `README.md`, `LICENSE` (CC0-1.0, this directory only), `RELEASE.md`, `release-paths.txt`, `RUNBOOK.md`,
  `STATUS.md`, `DELTA-EXPECTIMAX.md`, `PERF.md`, `REPORT.md`, `ADVISOR.md`, `reports/`.
- **Python:** `train.py`, the three deprecated shims (`train_stu.py`, `train_tok.py`, `train_scale.py`),
  `pipeline.py`, `check_parity.py`, `h2h_fast.py`, `h2h.py`, `h2h_queue.py`, `log2_launch.py`, `label_turbo_launch.py`,
  `turbo_eval_launch.py`, `turbo_eval_summary.py`, `rust_regret.py`, `build_distill.py`, `tools/`, `gym/`.
- **Test fixture:** `fixtures/labels2-LAD4-val-first100.rec` (10,000 bytes, the first 100 validation records; named
  `.rec` because `.gitignore` excludes `*.bin`).
- **Rust:** `ladder/Cargo.toml`, `ladder/Cargo.lock`, `ladder/build.rs`, `ladder/csrc/`, `ladder/src/`, `walt-fork/`.
- **Models (four, with metadata and cards):** `models/{LAD5-k73, STU-scale-W4-res512ln-40k, STU-tokens-b3-d48L2,
  STU-r3-tok-b8}.{w, json, MODEL_CARD.md}`.

## Not in the release

`ladder/target*/` (build outputs), `data/` (12 GB of label files), `results/`, `pipeline/`, `checkpoints/`,
`__pycache__/`, `models/*.npz`, every other file in `models/` (264 other weight files and their `.json`),
`monitor_collect.py` and `monitor_push.sh` (session monitoring that feeds an artifact database).

## Decisions for Jason before a public release

1. **License of vendored code.** `walt-fork/Cargo.toml` declares `license = "UNLICENSED"`, and `ladder/csrc/turbo.cpp`
   is a copy of the walt42x engine. A CC0 dedication can only cover what the releaser owns. If those two cannot be
   dedicated, drop them from `release-paths.txt`; the binary then cannot be built from the release alone (the Python
   side, the weight files, the format spec and `check_parity.py`'s trainer half still work).
2. **Local paths in docs and metadata.** `STATUS.md`, `reports/*.md`, `tools/curves.py` and every model `.json`
   carry absolute paths under the local home directory and agent session scratchpads.
3. **Reproduction inputs.** Reproducing LAD5 also needs the start net `models/STU-w-mlp256.{w,json}` (1.8 MB, tracked)
   and the label files under `data/`, which are excluded. Add the start net to `release-paths.txt` if wanted.

## Recipe

After the lead commits this cleanup on `experiment/fable-tiny-net-ladder`, one line from the repository root creates
the squashed branch in a fresh worktree (so untracked data in this checkout never leaks in):

```sh
git worktree add -b release/fable-tiny-net-ladder ../fable-release codex/walt-higher-k-budget-20261004 && cd ../fable-release && git checkout experiment/fable-tiny-net-ladder -- $(git show experiment/fable-tiny-net-ladder:experiments/fable-rust-ladder-20261004/release-paths.txt) && git commit -m "fable-rust-ladder: tiny-net ladder release (squashed, CC0-1.0)"
```

For a public repository holding only this directory, use `git switch --orphan release/fable-tiny-net-ladder-public`
in that worktree instead of a branch off main, then the same checkout and commit.
