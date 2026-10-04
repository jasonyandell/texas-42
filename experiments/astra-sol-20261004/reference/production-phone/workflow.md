# Campaign and release workflow

Use Python 3.12+ and an isolated application branch/worktree. All paths and counts below are validated task inputs, not defaults. Quote shell paths. No credentials belong in this skill.

Run `python3.12 scripts/test_campaign.py` from the skill folder after editing the helper.

## Discover and freeze

Read current `src/ai/book/played.json`: schema, source identity, games, seeds, four seat records per seed, profile/threshold/policy. Check current main and production destination in repository configuration/workflows before releasing. Locate the completed matching campaign with manifest, SQLite and producer bundles. Reconcile an already-expanded campaign explicitly; the helper intentionally rejects any source differing from the published catalogue.

Choose a preserved producer supporting adaptive extension. Read `producer.json`, check out `source_commit` in an isolated research checkout, and verify every listed source hash, binary/archive hashes and content-addressed bundle identity. Do not rebuild from research HEAD and claim it is the frozen producer.

```sh
python3.12 "$SKILL_DIR/scripts/campaign.py" inspect --campaign "$SOURCE_CAMPAIGN" --book "$APP_REPO/src/ai/book/played.json" --deals "$NEW_DEALS"
python3.12 "$SKILL_DIR/scripts/campaign.py" copy --campaign "$SOURCE_CAMPAIGN" --book "$APP_REPO/src/ai/book/played.json" --deals "$NEW_DEALS" --producer "$PRODUCER_ID" --research "$FROZEN_RESEARCH" --destination "$CAMPAIGN_COPY"
```

Stop source writers before copying where possible. SQLite backup includes committed WAL data; copying only `.sqlite` may lose it. Helper verification does not replace the receipt audit. An interrupted copy leaves its destination for inspection; choose a fresh path rather than overwriting it.

## Extend and measure

Read the frozen CLI/help because flags may evolve. For the adaptive campaign verified in this workflow:

```sh
cd "$FROZEN_RESEARCH"
python3.12 experiments/kiln/played.py extend "$CAMPAIGN_COPY" --hands "$TARGET_TOTAL_HANDS" --refine-games 640
python3.12 experiments/kiln/played.py run "$CAMPAIGN_COPY" --binary "$CAMPAIGN_COPY/producers/$PRODUCER_ID/kiln-play-worker" --workers "$WORKERS" --seconds 600 --games 160 > "$CHUNK_LOG" 2> "$CHUNK_ERRORS"
python3.12 experiments/kiln/played.py status "$CAMPAIGN_COPY"
```

`extend` saves the prior manifest and job-id-to-receipt-hash map under `extensions/extension-TOTAL-CAP/`. Confirm the whole baseline is captured and appended hands match planned fresh seeds/all four seats. Preserve profile, bid-30 policy, 4/5 threshold, completion generator and declaration set.

Allocation screens weak panels at 8/40 games, samples to 160, refines uncertain tails at 320/640, and takes selected audit panels to the cap. Terminal states: `screened`, `resolved`, `capped-unsettled`, `audit-complete`. `sampling`, `refining`, `refining-audit` and pending/running jobs are incomplete. Capped uncertainty is explicit evidence, not permission to alter the cutoff.

Read `status.json` between chunks (updates about every ten seconds). A bounded run may end `stopped` with pending jobs and zero errors: repeat the command with fresh logs. Never run two coordinators on one campaign. Investigate failures before retrying; do not cancel healthy jobs merely because an estimate elapsed.

Estimate from actual games/sec and panel depths. An observed 18-core campaign ran around seven games/sec; typical cost was about 2,400 games/deal. Five new deals needed 13,032 games and about 37 minutes including chunk transitions; the few final panels reduced throughput to roughly 2.5 games/sec. Worst-case cap is 36×640 games/deal. Fewer remaining panels reduce parallelism. Check disk/RAM/other workloads; do not interrupt unrelated processes or provision paid resources.

## Audit, export and import

After every panel is terminal and no pending/running/failed jobs remain:

```sh
python3.12 experiments/kiln/played.py audit "$CAMPAIGN_COPY" > "$AUDIT_JSON"
python3.12 experiments/kiln/played.py export "$CAMPAIGN_COPY" "$FINAL_EXPORT"
cd "$APP_REPO"
python3.12 scripts/import-bid-book.py "$FINAL_EXPORT" src/ai/book/played.json
```

Require successful full audit and export `complete: true`. The auditor checks every receipt hash, legal 28-move replay, stored scores/metrics, gap-free trials, all nine declarations, engine shuffle identities and producer binary/archive/source identity. Check SQLite integrity. Compare every baseline job hash to the extension snapshot, and every old compact hand exactly to the prior book. Prove seed disjointness/four seats, source identity, unchanged profile/threshold/policy, and no unexplained fallback/over-budget errors. Preserve old measured values even if separate research proposes bidding changes.

Update catalogue counts/provenance/docs/tests. Test every seed's four hands against actual engine shuffles, legal bid decisions against tails without workers, full within-match catalogue traversal, reload and next-hand behavior. Run current CI checks: typecheck, full Vitest, build/Walt verification, Python importer/update and deployment-support tests; discover any added checks from CI. Keep complete export, audit, prior receipts and logs outside the application worktree.

## Authorized publication

If publication was requested, stage only intended files, create a draft PR by default, attach it to the task, verify CI and deploy through the established main workflow. Confirm `/version.json` commit, served asset book identity/seeds and production gameplay/history persistence. Do not delete D1 databases or broaden permissions to bypass failure.

Report URL/commit/new seeds, audited games/moves, preserved evidence, tests, skill location and uncertainty. Expansion does not itself prevent separate matches drawing the same hand.
