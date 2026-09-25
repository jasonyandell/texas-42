id: [[hf-archive-upload]]
opened: 2026-08-24

## What

Data's homes are `~/data` and HuggingFace, never the git repo (Jason's
2026-08-24 ruling). Factory artifacts moved to
`~/data/texas-42/walt-factory-archive-2026-08-24/` (8.3G results +
514M store). Upload the curated citable set to a private HF dataset
(`jasonyandell/texas-42-walt-archive`), excluding the 8.2 GiB frontier
body (digest pinned in CENSUS-RULINGS' committed header; local body
kept in ~/data) and the 499 MB endgame_l2.store memo cache (pure cache;
regenerate command in walt/ARCHIVE.md). sha256 manifest generated
beside the archive.

## Done when

HF dataset exists with the curated set + manifest; walt/ARCHIVE.md
records both homes; Jason decides public/private.

## Links

[[walt-unification]], walt/ARCHIVE.md

## Status 2026-09-13 — unconfirmed in-tree

No file in the repository records a completed upload (no HF manifest,
receipt, or log; `walt/ARCHIVE.md` names the dataset as the target and now
says so). Whether `jasonyandell/texas-42-walt-archive` is populated is known
only outside the tree; the card stays in doing/ until Jason confirms or the
upload is re-run and receipted.

