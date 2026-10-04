---
name: plunge-generate-hands
description: Generate and audit genuinely new prepared Plunge catalogue deals using the frozen Kiln actual-play campaign, then optionally publish when authorized. Use for expanding the measured hand pool, not random dealing or bidding-policy changes.
---

# Generate prepared Plunge hands

Produce new measured deals while preserving all existing receipts and bidding behavior. Read [the workflow](references/workflow.md) before touching a campaign. The application book is `src/ai/book/played.json`; `docs-bid-book.md` explains the empirical policy.

## Scope and discovery

- Inspect repository instructions, current main, production `/version.json`, published book identity/count/seeds, and prepared unpublished campaigns. Reuse completed new evidence only after the same checks.
- Discover the research repository containing `experiments/kiln/played.py`, source campaign and frozen `producers/` bundles. Accept validated paths; never assume a personal machine location or use mutable research sources with a mismatched binary.
- A deal has four bidder hands and nine declaration panels per hand. `extend --hands` means total bidder hands, not deals. Derive new seeds from `seed_start` and hand count; prove disjointness from all published seeds.
- Keep generation local. Preserve source campaigns, earlier releases and unrelated work. Do not change bidding policy, add uploads, or promise that expansion prevents repeats across separate matches.
- Generation authorization does not authorize publication. When only asked to generate, stop with the audited export and reviewable results. With explicit publication authorization, use the established deployment workflow after validation; otherwise seek approval after preparing the concrete release.

## Essential checks

Use `scripts/campaign.py inspect` to derive a plan. Its `copy` mode uses SQLite backup, verifies frozen binary/archive/source hashes and producer identity, and refuses existing destinations. It does not extend, generate, import or deploy.

Run the verified producer in bounded resumable chunks. Inspect terminal panels, new receipts, active depths, throughput and errors between chunks; continue while jobs advance. Worker count depends on available cores/RAM and competing workloads. Fewer active panels can reduce throughput. Report substantial runtime/resource overruns with evidence.

Publish no partial book. Require terminal panels, a successful full receipt/source/legal-play audit, unchanged original receipt hashes and compact application hands, a complete export, validated importer output and engine tests. Preserve export, audit, status and extension snapshots outside the app worktree.

For a release, update `docs-bid-book.md` with exact identity/counts/depths/uncertainty/provenance. Verify deployed commit and served asset includes the new book/seeds; smoke-test production gameplay/history. Report across-match repetition and capped uncertainty honestly.
