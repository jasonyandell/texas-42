# walt archive ledger

Status: LEDGER, exploratory tier. Owns: where archived walt
computations live and how to regenerate them. Sources:
`UNIFICATION-CENSUS.md` §4, Jason's 2026-08-24 rulings (data's homes
are `~/data` and HuggingFace, never the git repo; archive and queue
for recompute rather than holding old versions).

## Homes

- **Local**: `~/data/texas-42/walt-factory-archive-2026-08-24/`
  (`results/` 8.3G, `store/` 514M) — the complete set, moved out of
  the crate tree 2026-08-24. sha256 manifest beside it
  (`manifest.sha256`).
- **HuggingFace**: dataset `jasonyandell/texas-42-walt-archive`
  (public since 2026-09-21) — the curated citable set: everything
  EXCEPT the two exclusions below. *Confirmed 2026-10-14* by the mk5
  Archive desk against the Hub's file metadata: `results/`, `store/`
  and `manifest.sha256` are there (1.9 GB, 183 files), beside the
  kiln snapshots (`kiln/snapshots/<stamp>/`, the same bytes as
  `~/data/texas-42/backups/<stamp>/`) and the tiny-net ladder tarballs.
  The [[hf-archive-upload]] card can close on that.
- **HuggingFace, the other homes** (recovered 2026-10-14 by the mk5
  Archive desk; all public, CC0, each with a card and `SHA256SUMS`):
  - `jasonyandell/texas-42-walt-lanes-2026-08-09` — the rest of
    `~/data/texas-42/`: the partnership gym and its Scheme campaigns
    (`relational-*`, `policy-synthesis-*`, `scheme-discovery-v1`,
    `gym-spec-v1`, `bid-making-v1`), the sunshine cycles
    (`sunshine-count-*`, `-rollout-v1`, `-gym-replay-v1`,
    `-recipes-v1`, `-biclusters-v1`, `plunge-sunshine`), the kiln run
    records, the Plunge phone-build lanes, `gran-anchors-2026-08-24`,
    `releases/`. Lanes with thousands of files ship as
    `<lane>.tar.gz` + `<lane>.files.txt`. Explained by
    `experiments/partnership/` and `experiments/kiln/` on
    `codex/walt-cpu-speedups` (3cf2536d).
  - `jasonyandell/texas-42-walt-speed-2026-09` — the untracked
    `experiments/` of this repo: `full-game-speed/` (results, docs,
    reference, tools, gpu-replay), `response-ladder/` (results,
    reference, tools), `walt42x-intake/`. Build output left out.
  - `jasonyandell/texas-42-walt-response-ladder`,
    `jasonyandell/texas-42-walt-ladder-labels`,
    `jasonyandell/texas-42-walt-tiny-net-ladder` (model) — the
    September ladder, as already cited.
- **Still local only** (Jason's call, asked in the feed): the
  `~/data/texas-42/seed-survey` companion and checkpoints (8.6 GB; a
  deterministic, SHA-pinned function of committed inputs by its own
  header) and the raw `~/data/texas-42/walt-response-ladder` run
  (5.9 GB; its curated export is the dataset above). `kiln-v1/` and
  `kiln-played-v1/` are the live campaign directories; the kiln's
  `backup.py` snapshots them to the archive, a folder upload does not.

## Exclusions from HF (kept local only)

- `results/second_rung_frontier_2026-08-14.txt` (8.2 GiB): its
  SHA-256, byte count, and line count are pinned in the committed
  header of `second_rung_2026-08-14.txt` and cited at
  CENSUS-RULINGS §SR closing note. No individual row is cited
  anywhere (verified 2026-08-24). Regenerable; local body is a
  convenience, deletable.
- `store/endgame_l2.store` (499 MB): pure memo cache. Cold
  regenerate (E-A17, printed by the producer itself):
  `rm -f walt-factory/store/endgame_l2.store && cargo run --release -p walt-factory --example fiber_probe endgame`

## Retired design docs (2026-08-24)

Seven completed-probe design documents were retired from `walt/` by
[[wiki-overhaul]] after their probes closed: `CENSUS.md`,
`FIBER-PROBE.md`, `FIBER-REFINE.md`, `ENDGAME-STORE.md`,
`SEAT-CENSUS.md`, `PREDICTIVE-RANK.md`, `DEADNESS-PROBE.md`. Bytes
preserved at `git show 2de8a05:walt/<NAME>.md`; owning wiki pages:
`wiki/walt-census-era.md` and `wiki/walt-s6-era.md` (narrative),
`wiki/walt-math-reference.md` (ruling map), `walt/CENSUS-RULINGS.md`
(adjudications, append-only, historical citations left as written).
None of the seven is listed in a freeze manifest.

## Relocation (2026-08-24)

The TRACKED artifacts survived the factory deletion in-tree: the 65
result summaries/certificates formerly at `walt-factory/results/` and
`walt-factory/docs/certificate-schema.md` now live at
`walt/probes/factory-results/` (provenance README there). Historical
docs citing the old paths refer to that directory; [[wiki-overhaul]]
re-synchronizes the prose.

## Recompute queue

Producer code (walt-factory + its 24 examples, walt-skeleton) is
DELETED from the tree by [[walt-unification]] — regeneration means
checking out the producer commit first. **Producer commit: `648f93a`
(walt-unify; last commit with all producers present).** Deletion
commits: `ad355e9` (orphans, wasm-spike, PLAN.md), `fa3fe74`
(walt-factory, walt-skeleton).

Recompute protocol per artifact:

    git switch --detach 648f93a
    cargo run --release -p walt-factory --example <producer> [args]
    # producer by filename stem (census §4): census_* ← census_run;
    # deadness_* ← deadness_probe; economy_* ← economy_run{,_r2}/economy_seed;
    # falsification_* ← falsification_run; fc_correlation_* ← fc_correlation;
    # feature_fee* ← feature_fee{,_v11}; fiber_probe_*/fiber_refine_*/endgame_* ← fiber_probe;
    # fusion_tax* ← fusion_tax; label_transfer_* ← label_transfer_run;
    # laydown_* ← laydown_probe; lesson_basins_* ← lesson_run;
    # policy_geometry_* ← policy_geometry; predictive_rank_* ← predictive_rank;
    # rule_economy_n4_* ← rule_economy_n4; second_rung* ← second_rung;
    # seed_survey_* ← seed_survey; separation_* ← separation_probe;
    # trick1_draw_* ← trick1_draw; full_walk_* ← the walk_corpus bin.

Verify against the manifest digest before treating a regeneration as
the same computation. Frozen seeds make byte-identity the expected
outcome; a mismatch is a finding, not a shrug.

Nothing in this ledger promotes any result; the artifacts keep exactly
the tier they had.
