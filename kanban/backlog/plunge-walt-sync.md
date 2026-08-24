id: [[plunge-walt-sync]]
opened: 2026-08-24

## What

Cross-repo: keep plunge's copy of `walt.wasm` + `walt.ts` current with
the walt-wasm pkg (θ=11/16 default and race mode shipped 2026-08-19 —
verify plunge carries that build), and wire the opt-in `race: true`
where it pays (opening leads, high-bid contracts). Owned from the
plunge repo; this card is the walt-side pointer.

## Done when

plunge's bundled wasm matches walt-wasm/pkg by digest; race wiring
decision made on the plunge side.

2026-08-24: walt-wasm now exposes viewer-fiber evaluation — optional
`viewer`/`viewerHand` on `play` returns `viewer_opts` (the actor's
options priced from the viewer's fiber; `[tile, bp|null, support]`).
Additive and byte-identical for existing requests; pkg rebuilt (wasm +
walt.ts + README, branch viewer-cross-fiber). Wire the plunge "Ask walt"
review's second column against it when syncing the pkg.

## Links

[[viewer-cross-fiber-review]], walt/walt-wasm/pkg/README.md
