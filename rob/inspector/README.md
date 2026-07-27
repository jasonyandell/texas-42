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
