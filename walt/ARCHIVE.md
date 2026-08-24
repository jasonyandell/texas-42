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
  (private until Jason rules otherwise) — the curated citable set:
  everything EXCEPT the two exclusions below.

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
