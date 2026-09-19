# Sunshine Atlas: exploratory overview of actual play

[Open Sunshine Atlas](https://sunshine-atlas.psychometrix.chatgpt.site) — snapshot 01, published 2026-09-19,
initially private to Jason's ChatGPT account. A static phone-friendly research
viewer, with no live campaign connection or player changes.

## What is there

- **Terrain:** 4,500 hand/declaration panels, a two-dimensional event map, six
  overlapping descriptive components, and a searchable clustered hand matrix.
- **Count and control:** count's original holder versus eventual captor, plus all
  84 physical-tile/hidden-seat associations with within-panel centered outcomes.
- **Worlds and choices:** 240 retained positions with seven through two dominoes
  remaining; compare forced actions in the same sampled world and policy seed.
- **Evidence:** methods, sample sizes, snapshot identity and source receipt hashes.
  Every panel opens its eight complete games; paired details preserve every
  retained 8/24-world refinement and all legal action continuations.

Links retain selected panels/trials and roots/worlds/actions. Tap a dot or table
cell to inspect complete tricks. The raw receipts remain in the durable campaign.

## Dataset and interpretation

The broad map uses **exactly trials 0–7 for all 500 hands × nine declarations**:
36,000 complete games, equally weighted by panel. This avoids using adaptive
sample depth as a population weight. The consistent read-only source transaction
contained 271,321 games; deeper per-panel histograms are shown separately as
snapshot context. These counts are not a claim that production has finished.

The representation has 52 mechanical features: 16 count-origin/captor flows,
12 role/phase trick wins, 16 lead transitions, and eight role/trump captures.
Each of the four blocks is normalized. Six-component NMF is descriptive
(relative reconstruction error 0.3614); the two PCA axes retain 43.7% of variance.
Separately fitted source-deal halves have matched component cosines 0.9937–0.9993.
That is descriptive stability, not six discovered threat types or predictive
validation. Retrospective count capture necessarily tracks much of the score.

Holder associations show mean outcomes centered within exact hand/declaration
panels, with 400 source-deal bootstrap replicates and exploratory unadjusted
intervals. All 84 comparisons remain visible. They are not effects of moving a
tile between seats. Eight trials per panel remain noisy.

The existing contrast corpus contributes 2,304 worlds and 9,200 complete branches.
Its first eight worlds per root supply the overview; selected refinements remain
available in details. Original actions and same-world alternatives are preserved,
including ties and reversals. Historical exam sources now overlap the expanded
campaign, so they are not untouched future validation. These are sampled policy
continuations, not optimal-play certificates or demonstrated player improvements.

## Provenance and verification

- Analysis implementation and static assets:
  `/Users/jason/code/texas-42-sunshine-atlas`.
- Published Git source: `d47eda25e6030fdbd8b8df2692247439b51003c2`.
- Research/rules source: `a25e4b0c53b8397afe6ba9c2e459db6fa18e1eda`.
- Snapshot: `d75e8160caff3daf2d9bcaad325077b984f25af55f33dcde5dea2e52603f1c6f`.
- Publication IDs, package hash and browser checks: [ATLAS-v1.json](ATLAS-v1.json).
- Rebuild instructions, dependency versions and full feature basis live in the
  Atlas repository. The source branch is pushed to the Site's Git repository.
- A verified deployment package and publication record are retained at
  `/Users/jason/data/texas-42/sunshine-atlas-v1/`.

Verified 36,000 source receipt hashes, every complete source-game replay, all
2,304 contrast receipt hashes, all 9,200 branch replays, score totals, count
conservation, prefix membership, summaries and paired deltas. Hosted Chrome
interaction checks passed at 1440×1100 and 412×915: filtering, source search,
game/world selection, all seven tricks, count flow and holder inspection, and
reopening deep links. No JavaScript errors or horizontal page overflow.

The campaign, deployed Walt and stopped monitor were not modified. This viewer
supports the Sunshine loop: see a broad pattern, inspect its witnesses and
counterexamples, then formulate and separately challenge a hypothesis.
