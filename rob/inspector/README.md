# rob inspector

A self-contained, state-by-state visual walkthrough of a rob match: step
through all 28 plays of every hand, switch perspective between the four seats
(each masked to exactly what that seat knows — its hand, its derived
cells/voids, its exact fiber count and holder marginals) or omniscient truth,
and at every rob decision read the **contingency book** — the whole contingent
plan rob solved, with the exact value of every legal opening's best plan,
chosen and rejected.

Display only. Exploratory tier: the trace is non-normative diagnostic output
and creates no receipts and no claims (`wiki/analysis.md`).

## Use

```sh
cd rob
cargo run --release --bin trace_rob      # regenerates inspector/trace.js (+ trace.json): rob + contingency book
open inspector/index.html                # or: python3 -m http.server -d inspector
```

`trace_rob` (`crates/verify/src/bin/trace_rob.rs`) plays four deterministic
deals with rob (rolling re-solve, `Points` lens, normative window) on team 0
and the baseline on team 1, and embeds a capped projection of rob's plan at
every one of his decisions (P5 receipt row `r_book_trace`: 4 hands, 112
decisions, 56 plans embedded; projections capped at 3 levels / 24 branches).
This is the **committed default view** — the checked-in `trace.js`/`trace.json`
are `trace_rob` output, and the page's own "no trace loaded" text says so.

Optional baseline-only view:

```sh
cargo run --release --bin trace_player   # the baseline's frozen seed-42 self-play match (the verify_player receipt)
```

`trace_player` writes the **same two files**, so running it overwrites the
committed rob trace with the baseline view (no plan tree, no openings table —
the baseline is a fixed-field Monte Carlo player whose decision panel shows
sampled-world utility totals instead). Regenerate with `trace_rob` afterwards,
or `git checkout -- inspector/trace.js inspector/trace.json`.

Keys: `←`/`→` step · `PgUp`/`PgDn` hand · `0`–`3` seat perspective ·
`O` omniscient · `Home`/`End`. You can also drop a `trace.json` onto the page.

## What the panels show

**Table and score line.** The four seats around the current trick, the play
so far, the running team points, the auction line and (once settled) the
result. Trump tiles are highlighted wherever they render (see below).

**Decision panel — at a rob decision, the contingency book.**

- The header: `rob's plan — exact info-set best response vs σ · window H
  trick(s) · exact fiber N worlds`. `H` is the window depth the normative
  budget formula produced (B = 2²⁸; INV-P6), `N` the exact capacity-DP fiber
  count at that information set.
- **The openings table**: one row per legal opening tile — the exact integer
  value total of the *best whole plan that opens with that tile* over all `N`
  worlds, the per-world average marked `≈` (display-only decimal), and the
  chosen plan marked `◀ chosen plan`. The table prices **plans, not tiles**
  (PLAN-NOT-TILE, INV-P1): a row is the value of the contingent plan, never a
  scalar attached to a domino. Rejected plans are priced exactly, which is what
  makes "why not that tile?" an answerable question.
- **The plan tree**: expandable branches keyed by the *observation* rob will
  see before acting again (`S1:6-4 S2:5-0 …`, or "(no plays before rob acts
  again)"), each node showing the action rob plays there, the number of fiber
  worlds flowing through the node (`worlds`), the exact value total, and the
  per-world `≈`. Leaves are typed: a settled hand's exact outcome, or the
  frontier leaf `BankedPoints` (INV-P7). Branches beyond the projection cap
  are elided with a count.

**Decision panel — at a baseline decision.** The utility lens, the number of
worlds sampled uniformly from the exact fiber, and the exact per-world utility
totals for every legal action with the argmax the player chose.

**Belief panel (per seat).** The unseen pool, the exact fiber count, and the
holder marginals per tile as exact counts (`c/N` on hover); the bars are
display-only.

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
  same code regenerates it byte-for-byte (`trace_rob` replays four fixed deals
  through rob's rolling re-solve and the baseline; `trace_player` replays the
  frozen seed-42 receipt match with the same driver as `verify_player`).
- Every displayed value is exact in the trace: integer utility and plan value
  totals, exact reduced-rational averages, exact fiber and marginal counts as
  decimal strings. Decimals in the UI are display-only and marked `≈`.
- The viewer never recomputes game logic; it renders trace fields only. Plan
  values and openings-table rows are values *of the plan* at that information
  set, emitted by the solver; the page derives nothing.
- Perspective masking is emitted per seat by the Rust tracer from that
  seat's own `MechanicalState`; seat views are never derived in JS from the
  omniscient deal, so a viewer bug cannot silently leak hidden information
  into a seat view.

*History.* From 2026-07-27 (9cdcc5e0) to 2026-09-12 this file named
`trace_player` as the regeneration command, although the committed trace had
been `trace_rob` output since 2026-07-28 (e0414c06, openings table 0754cb07).
Corrected 2026-09-12; `wiki/rob.md` records the drift.
