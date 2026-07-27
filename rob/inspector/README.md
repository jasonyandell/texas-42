# rob inspector

A self-contained, state-by-state visual walkthrough of a rob self-play match:
step through all 28 plays of every hand, switch perspective between the four
seats (each masked to exactly what that seat knows — its hand, its derived
cells/voids, its exact fiber count and holder marginals) or omniscient truth,
and see, at every decision, the legal actions with their exact per-world
utility totals and the argmax the player chose.

## Use

```sh
cd rob
cargo run --release --bin trace_player   # regenerates inspector/trace.js (+ .json)
open inspector/index.html                # or: python3 -m http.server -d inspector
```

Keys: `←`/`→` step · `PgUp`/`PgDn` hand · `0`–`3` seat perspective ·
`O` omniscient · `Home`/`End`.

## Trump indicator

The header shows a badge for the current hand's declaration — `trump: 3s` for
a pip trump, `trump: doubles`, or `no-trump (follow-me)`. Every trump tile is
highlighted (violet border + corner mark) wherever it renders — in the hands
and in the current trick, identically in the per-seat and omniscient views.
Which tiles are trump is **not** decided in JS: the Rust tracer emits the
exact called-and-powered tile set per hand (`hands[h].trump`, the called set
`κ_δ` of Math §3.2), and the viewer only marks those names.

## Shareable links (URL hash params)

The full inspector position is mirrored into the URL hash, so the address bar
is always a copy-paste shareable link to the exact state on screen. Hash (not
query) params are used because they update without a reload and are safe on
`file://` URLs. Format:

```
#hand=<hand-array-index>&step=<play-index 0..27>&view=<0|1|2|3|omni>
```

Example — hand 2, the 14th play (`step=13`), from seat 0's perspective:

```
inspector/index.html#hand=2&step=13&view=0
```

Every navigation (arrows, hand paging, perspective switch, slider) rewrites
the hash in place via `history.replaceState`. On load — or when a shared hash
is pasted into the address bar — the viewer jumps straight to that state;
missing or invalid params fall back gracefully to the start of hand 0 in the
omniscient view.

## Honesty rules

- The trace is **non-normative** diagnostic output, but deterministic: the
  same seeds regenerate it byte-for-byte (`trace_player` replays the frozen
  seed-42 receipt match with the same driver as `verify_player`).
- Every displayed value is exact in the trace: integer utility totals, exact
  reduced-rational averages, exact fiber and marginal counts as decimal
  strings. Decimals in the UI are display-only and marked `≈`.
- The viewer never recomputes game logic; it renders trace fields only.
- Perspective masking is emitted per seat by the Rust tracer from that
  seat's own `MechanicalState`; seat views are never derived in JS from the
  omniscient deal, so a viewer bug cannot silently leak hidden information
  into a seat view.
