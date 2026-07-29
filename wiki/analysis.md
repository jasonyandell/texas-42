# Game Analysis — Instruments Over the Exact Machinery

[Home](Home.md) · owns: the catalog of probes, rigs, and dashboards that interrogate
rob's play · Sources: none (this page cites; it is never cited by any page above the
Ideas tier). Related: [ideas](ideas.md), [verification](verification.md),
[strategic-state](strategic-state.md), [belief-vs-support](belief-vs-support.md).

> **Epistemic tier: DISPLAY / EXPLORATORY — below every tier on
> [Home](Home.md#evidentiary-tiers--never-promoted-never-blurred).** Analysis
> instruments are surfaces *over* certified machinery, not machinery. A probe's output
> is **exploratory** unless and until it is promoted to a receipt row by brief
> amendment; until then it may not be quoted as a result in a brief, a dispatch, or
> FINDINGS. Instruments never create claims of their own, and nothing above the Ideas
> tier cites this page.

The `rob/` engine is an executable spec with proof receipts
([verification](verification.md)); it answers *what is true*. This page owns the layer
that answers *what happened, and why* — the tools that take an exact solve apart and
show its parts. The distinction is load-bearing: a receipt is evidence about the
mathematics, an instrument is a lens on a single game.

**The inspector's honesty rule extends to everything on this page**
(`rob/inspector/README.md`, "Honesty rules"): an instrument only displays or aggregates
exact values computed by the certified Rust. It never derives game logic of its own.
The reason is the same reason the inspector's per-seat masking is emitted by the tracer
rather than derived in JS from the omniscient deal — a display bug must not be able to
manufacture a fact, or leak information the viewer does not hold.

## Existing instruments

*(state of the repo, 2026-07-28)*

### The inspector — the per-decision contingency book

`rob/inspector/index.html`, fed by the `trace_rob` binary
(`rob/crates/verify/src/bin/trace_rob.rs`, which writes `inspector/trace.js` and
`trace.json`). Step through every play of every deal, switch perspective between the
four seats or omniscient truth. At a rob decision it shows the solve window, the exact
fiber count, an expandable **plan tree** (action / observation keys / bundle sizes at
each node), and the **openings table**: the exact best-plan value for every legal
opening, chosen *and* rejected. The table prices *plans*, not tiles — each row is the
whole contingent plan that opens with that tile (PLAN-NOT-TILE; the plan type is
INV-P1). Full usage, keys, and shareable-URL format in `rob/inspector/README.md`.

### `gate::solve_opening` — the rejected-plan recovery surface

`rob_player::solver::gate::solve_opening(state, lens, action)`
(`rob/crates/player/src/solver.rs`). The doc-hidden gate module is the analysis
entry point into the solver: this function returns the best plan *restricted to one
fixed opening*, which is what makes "why not that tile instead?" an exactly answerable
question rather than a rhetorical one. Play itself never routes through it (INV-P6);
it exists for display and probes.

### `ablation_probe.rs` — paired-match ablations

`rob/crates/verify/tests/ablation_probe.rs`. Mirrored paired matches with the depth
knob turned down: myopic rob (window 1 at every decision) scores **net −288** against
the baseline; full rob against myopic rob, mirrored over 200 hands, is **net +876**.
These are **exploratory numbers, not receipt rows** — the file says so in its own header
comment, and they are the worked example of the promotion path below.

### `nickel_probe.rs` — the decision autopsy pattern

`rob/crates/verify/tests/nickel_probe.rs`. The pattern: take one decision, replay
*every* opening's full plan against *every* world of the fiber under the solve's own
model (plan vs σ), and tally the outcome-relevant events — who captures a count tile,
who wins the current trick. It is **self-validating**: the sum of per-world margins must
reproduce each plan's exact root value, so the probe checks itself against the solver
rather than asking to be believed.

First subject: P4-stream deal 1, trick 4 — the 5-0 slough over the "obvious" 3-0. The
answer the fiber gives is that partner wins the trick in **5,970 of 8,400** worlds under
σ, so the nickel is not being given away; it is being handed across the table.

### `sigma_counterfactual_probe.rs` — the opponent model, quantified

`rob/crates/verify/tests/sigma_counterfactual_probe.rs` (committed `ee98cd6`, with its
findings frozen in the file). The same trick contest re-scored under **variant responder
policies**: σ's minimal-beater against a max-trump shut-out. First subject: partner wins
in **5,970 of 8,400** worlds under σ, against exactly **4,200 of 8,400** — a flat half —
under max-play. This is the instrument that puts the opponent
model in the denominator where it belongs — the nickel autopsy's number is a fact about
the pair (fiber, σ), and this probe is how much of it is σ.

## Planned instruments

*(named by Jason, 2026-07-28. Exploratory, and deliberately without a schedule — the
tier statement above applies in full; none of these is an expectation.)*

**Path analysis.** Aggregate over the play-out paths of a solved plan: for a fixed
policy in a fixed game, which dominoes contribute to good and bad outcomes, and through
which events — captured count, spent control, forced leads. This is the nickel autopsy
generalized from one tile at one decision to every tile over a whole plan. The
constraint that keeps it honest is that every number stays an exact count or an integer
sum over the fiber; nothing becomes a heuristic score.

**Path groups.** Partition the fiber by rob's own *realized action sequence*: "I will
play x then y then z in all of these worlds — what are they?" The plan tree already
partitions worlds by *observation* sequence; path groups project that partition onto the
viewer's **action prefix**, since many observation branches share one action prefix.
Each group carries exact world counts and value totals. Where possible, a group should
be described **intensionally** — as a cell system, not a world list — which is the
direct connection to [idea-hierarchical-fibers](idea-hierarchical-fibers.md): *a path
group's description IS a sub-fiber*. If that page's rung 1 lands, path groups are one
of the things it buys.

**Dashboards.** Standing visual surfaces over the above — per-hand tile-contribution
views, path-group browsers. Display only, under the same honesty rule as the inspector:
they render fields the Rust emitted, and compute no game logic.

## The promotion path

An instrument's output travels exactly one road, and it has a gate on it:

**instrument → its numbers quoted as exploratory (cited to the file, tier stated) →
receipt row, by brief amendment.**

The last arrow is the only one that changes a number's tier, and it is a deliberate
editorial act: an amendment to the owning brief that names the invariant, fixes the
seed and corpus, and adds the row to a verifier's receipt. Nothing is promoted by
having been useful, or by having been quoted often. The ablation-probe numbers above
are the standing worked example — measured, reproducible, cited, and still exploratory,
because that choice has not been made.

[ideas](ideas.md) is the sibling of this page on the other axis: it captures directions
we have not earned the right to claim, while this page captures instruments whose
outputs we have not yet chosen to certify.
