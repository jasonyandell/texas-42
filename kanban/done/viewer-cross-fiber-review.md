id: [[viewer-cross-fiber-review]]
opened: 2026-08-24

## What

In the plunge "Ask walt" review (and the walt viewer), price the tapped
seat's options from the *viewer's* fiber alongside the actor's — two
columns, "from her seat / from yours." Same machinery, different root
viewer, no new math. Rows where the columns disagree are the
human-visible flag of the information asymmetry the level-2 probe
mines (the 5-5 specimen would have shown sub-100% from the bidder's
seat until the tile shows). Not gated on anything.

## Done when

The review renders both fibers' prices; walt-wasm exposes viewer-fiber
evaluation if the current request shape can't express it.

## Links

[[level2-field-swap-probe]], [[plunge-walt-sync]], walt/LEVEL2-PROBE.md

## Closed 2026-08-24 (walt side)

Merged to main via PR #17 (commits 97dd51a..80dc095). Solver gained
`viewer_fiber_evaluate` (one authority, root-viewer swap, lawfulness
conditioning per SCENARIO-PLAYER §4); wasm gained additive
viewer/viewer_hand → viewer_opts (byte-identical without them,
test-asserted); webtable renders the two columns with disagreement
flags. Full CI green incl. Lean in the agent worktree; wasm smoke
28/28 byte-identical through the rebuilt pkg. First live probe showed
a stark specimen immediately (actor 100% / viewer-fiber 0%). The
plunge-side wiring is [[plunge-walt-sync]]'s, with a dated note filed
there.
