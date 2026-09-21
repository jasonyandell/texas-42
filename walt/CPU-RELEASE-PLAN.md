# CPU speedups: main alignment and phone release plan

Status: completed, 2026-09-20. See [deployed release and verification](CPU-PHONE-RELEASE.md).
The plan below preserves the pre-release inspection and intended gates. Performance and conformance below are
exploratory engineering evidence, not a strength result or equivalence proof.

## Verified starting point

- GitHub `texas-42/main` is `5e8cd0f7`. Local main agrees.
- Both `codex/partnership-launch` and `codex/walt-cpu-speedups` are `f1a0fb04`.
  Their worktrees were clean at inspection. Main is an ancestor, 56 commits behind.
- CPU implementation `701e8589` and receipts `f1a0fb04` are already integrated
  with our preceding Kiln work. Reapplying or cherry-picking them is unnecessary.
- `codex/walt-response-ladder` is `92ddcaa0`, two commits beyond `f1a0fb04`.
  Those changes occupy `experiments/response-ladder/` and the instrument index;
  they are the separate experimental GPU player, not needed for this release.
- GitHub and local `plunge/main` are `c7a1215d`, the live empirical bidder release.
  Its phone manifest still pins Walt source `5e8cd0f7` and WASM `2730dc30...`.
- The original checkout remains on `walt-gran`, with untracked experimental work.
  Preserve it; do not use a reset or blanket add to align the release branches.

The complete integration and existing native receipts are in
[CPU-SPEEDUPS.md](CPU-SPEEDUPS.md). The current release importer at
`plunge/scripts/update-walt.py` explicitly passes `--no-default-features`, so merely
merging main or rebuilding with that importer will not enable the CPU bundle.

## New direct check against the shipped asset

[Planning receipts](receipts/cpu-live-plan-v1/summary.json) preserve the exact
artifact hashes, input cases, comparison program, timings and watchdog result.
All 59 source hashes in the existing CPU build record still match the checkout.
The candidate WASM matches the committed CPU parity receipt (`bb50e08e...`).

All 15 comparisons passed: nine declarations at 40 worlds, targets 36 and 42,
one complete partner review, and 160-world openings with blanks, sixes, and no
trump. Selected moves and exact action values agree. Work counters are permitted
to change. Median paired speedup was 21.35x in Node on this Mac; the three deeper
calls fell from 7.51/9.39/4.15 seconds to 0.33/0.44/0.18 seconds. This is one small
frozen-clock panel, not a phone benchmark. It includes accumulated Kiln changes,
not just the latest CPU bundle. Existing matched current-baseline native L2
receipts show 13.13x and current-baseline WASM receipts show about 4.42x.

The candidate is 6,404,138 raw bytes versus 786,884 shipped. Local gzip compression
is 404,394 versus 266,921 bytes. The 5,531,904-byte completed-trick lookup explains
most raw growth. Measure browser memory/startup and actual serving compression;
these gzip measurements are not observed network payload sizes.

## Integration sequence

1. **Close the source integration.** Preserve this plan/evidence in a focused
   commit. Refresh GitHub refs and recheck clean status and ancestry. Use a
   temporary main worktree and fast-forward main to the partnership branch;
   push without force after the relevant checks. Fast-forward the CPU branch to
   the same released main. Keep the GPU experiment on its own branch and merge
   the updated main into it if needed. Do not flatten or rebase the research
   history, or import untracked scratch directories. If refs changed, re-evaluate
   the graph rather than forcing the planned operation.
2. **Make the phone build explicit and reproducible.** On a focused Plunge
   release branch, change the importer to use `--no-default-features --features
   cpu-speedups` for `wasm32-unknown-unknown`. Keep Rayon/native parallelism off.
   Require committed solver sources, build with the lockfile and a controlled
   build environment, and record the target, enabled features, compiler, source
   and WASM hashes together. Avoid inheriting native-only `RUSTFLAGS`. Verify the
   existing two host imports and wire ABI. Keep the reference build available for
   comparison. Import only a freshly built, tested artifact, not the loose target
   file used by this planning check.
3. **Run the release gates below.** Do this locally before pushing Plunge main,
   because that push triggers deployment. Preserve compact results and identities.
   Do not increase worlds, budgets or alter player policy during this release.
4. **Ship through the existing pipeline.** Commit the updater, matching manifest
   and WASM together, fast-forward Plunge main, and push. Its existing GitHub
   Actions performs typecheck, tests, build and Cloudflare deployment. Verify the
   hosted version beacon, served asset identity and real browser play afterward.
5. **Close the release record.** Record both source commits, artifact hash, test
   evidence and measured host timings. Update the player/release docs. Confirm
   the worktrees' expected ancestry and clean state; retain the GPU branch.

## Release gates

- Rebuild and run focused optimized/reference solver tests, player contract
  tests and the exhaustive completed-trick lookup check. Native fixed-solve
  receipts already exist; repeat a bounded matched check on the exact release
  build if source/build settings change. Do not require a new strength campaign
  for an implementation-preserving change.
- Repeat shipped-versus-release exact-value checks, with multiple full games,
  different seats and declarations, 40 and 160 worlds, higher targets and partner
  reviews. Exercise both <=64 support specialization and >64 general paths.
- Run `walt/walt-player/check.mjs` and `auction-check.mjs` (or equivalent release
  fixtures) against the optimized native/WASM pair. Verify forced deadlines,
  retained 40-world checkpoints when deeper work expires, malformed inputs,
  cancellation, and a complete partner review. Coalesced clock checks make these
  tests particularly relevant. Real-clock runs must respect the existing host
  timeout and never return partial action vectors as completed decisions.
- Run Plunge typecheck, full tests, build and deployment dry-run. Browser-check
  a whole hand, opening 160, ordinary 40, Think Deeper, question receipts, saved
  game reload, old shared links, cached/offline play and version refresh. Test
  worker startup, memory growth and cancellation on a throttled browser. Report
  Mac browser results as such; Pixel/iPhone measurements remain separate.
- Preserve empirical catalogue bidding and its source identity. The bid book
  records the measured prior player; do not silently relabel or regenerate its
  probabilities. Faster deadline-limited execution may complete more review work
  and occasionally choose differently, even when completed fixed solves agree.
  Record that distinction in the release evidence rather than claiming stronger
  play or new calibration. Existing noncatalogue live-auction fallback also needs
  its contract checks.

## Rollback

Retain the prior Plunge commit and artifact. Revert the focused Plunge release
commit through the same tested deployment pipeline if hosted checks fail. This
restores the old matched manifest/WASM without removing the bid book, saved
questions or research history. Rollback needs no solver reset, data migration or
Kiln restart. A hosted version/asset check confirms which build is active.
