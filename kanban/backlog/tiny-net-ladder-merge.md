id: [[tiny-net-ladder-merge]]
opened: 2026-10-05

## What

 Land `experiment/fable-tiny-net-ladder` (experiments/fable-rust-ladder-20261004: single
trainer, header-table Rust loader, parity test, README/model cards, CC0) on the main line as a
squashed experiment branch, without dragging in build dirs, label piles or local-machine detail.
Follow `experiments/fable-rust-ladder-20261004/RELEASE.md` and `release-paths.txt` (48 paths,
~17 MB): new branch off `codex/walt-higher-k-budget-20261004`, check out only the listed paths.

## Security / hygiene before the PR

- Scrub absolute home and session-scratchpad paths from docs, model `.json` metadata and scripts
  (the Hugging Face copies are already scrubbed; the in-repo copies are not).
- No tokens, cache dirs or `~/.cache/huggingface` references in anything committed; the upload
  script stays out of the repo (it only needs the cached login).
- Keep `ladder/target-*`, `data/`, `results/`, `pipeline/`, `models/*.npz` and all non-release
  models out (they are in `.gitignore` or the release manifest's exclude list).
- Licenses: CC0-1.0 everywhere in this directory (both Cargo manifests already set); `LICENSE`
  file travels with it. The engine file is a copy of the walt42x search; CC0 per Jason.
- Tier: exploratory. README and cards keep the "exploratory, no multiplicity correction" caveat;
  nothing in the wiki ledgers is promoted by this merge.

## Done when

 PR from the squashed branch builds (`CARGO_TARGET_DIR=... cargo build --release
--offline`), `check_parity.py` passes 19/19 under the cap, no path under `/Users/` or
`/private/tmp/` appears in the diff, and the HF links in README resolve.

## Links

 STATUS.md and DELTA-EXPECTIMAX.md "Overnight 2026-10-05" (results),
https://huggingface.co/jasonyandell/texas-42-walt-tiny-net-ladder,
https://huggingface.co/datasets/jasonyandell/texas-42-walt-ladder-labels, [[ladder-policy-store]].
