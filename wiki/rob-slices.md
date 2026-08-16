# rob's Build History — the Briefs and their Stages

[rob](rob.md) · [Home](Home.md) · owns: the record of what each rob brief
assigned and what each stage established · Sources:
[rob/BRIEF.md](../rob/BRIEF.md), [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md),
[rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md). Related:
[verification](verification.md),
[first-implementation-slice](first-implementation-slice.md), [rob](rob.md).

rob was not written all at once. It was built in **gated stages**: a stage begins
only when the previous stage's receipts are green, and each stage's deliverable
is a receipt whose exact contents were specified in the brief *before* the code
existed. This page is the record of that sequence — what was assigned, what each
stage established, and which receipt witnesses it.

The division of labour with the neighbouring pages:
[first-implementation-slice](first-implementation-slice.md) owns the original
slice-01 *assignment* as a historical document (it predates rob and was
superseded by `BRIEF.md`); [verification](verification.md) owns every receipt's
exact integers; this page owns the *shape* of the build.

**Tier note.** Everything below is [rob conformance receipts, tier 4]. A green
stage means rob independently reproduced the stated finite claims and that its
invariants held; it never means a claim changed status
([Home](Home.md#evidentiary-tiers--never-promoted-never-blurred)).

## The brief family

Three binding assignments, each a self-contained contract with layout,
invariants, receipt rows, and a definition of done. They are **read-only
history**: they are not edited to match later work, so a brief may describe a
module that was later folded elsewhere or a budget that a later amendment
changed. Read each with its own amendment notes.

| Brief | Track | Stages | Receipts |
|---|---|---|---|
| [rob/BRIEF.md](../rob/BRIEF.md) | Slice 01 — declaration algebra through the normal form and capacity DP | S1–S4 | 4 |
| [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md) | Slice 02 — support dynamics through the outer language, with a stretch stage | S5–S9 plus S10 | 6 |
| [rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md) | The player track — σ, the exact solver, rob at the table | P1–P5 (P6 stretch, not built) | 1 |

A twelfth receipt, `verify_player.txt`, belongs to none of the three: the
baseline Monte Carlo player was added between slices 01 and 02 and keeps its
self-play transcript as a whole-stack determinism regression.

Slice 01 also introduced the ten invariants INV-1 … INV-10 (derived-not-stored,
projected equality, proof-irrelevant reachability, exact arithmetic,
counts-are-CI, reachable-implies-feasible, no-rank-from-identity,
one-source-of-truth, type-distinct domains, vocabulary); slice 02 added INV-11 …
INV-14 (edge budget, monotone ambiguity, typed-transition-only, and the outer
language stays necessary-only); the player brief added INV-P1 … INV-P7. The
enforcement map from each *census* invariant to its named test or CI check is
owned by [verification](verification.md); the player series is enforced more
loosely, as [rob](rob.md) records.

## Slice 01 — the mathematical spine

Four gated stages taking rob from the tile universe to the exact support normal
form.

| Stage | Established | Receipt |
|---|---|---|
| **S1** Declaration algebra | The 28-tile universe, the nine declarations as relational algebras, contextual rank and tier, the unique trick winner, pip transports with the scored/unscored split, and the three unscored mechanics classes. Trick resolution is cross-checked against an *independent prose-rule resolver* written from the rules text, not from the algebra. | `verify_algebra.txt` |
| **S2** Objective hand machine | Rules configuration, the one-round auction and its exhaustive tree, contract certification and settlement, and the phase-indexed contracted-play state with certified lifecycle constructors — plus legal-play and 42-point conservation over a hand corpus. | `verify_objective.txt` |
| **S3** Cells as a derived view | Capacity cells derived from mechanical state rather than stored beside it, the typed update algebra, and a replay-parity corpus proving the derived view and the incremental updates agree. This stage owns the frozen with-voids generator value. | `verify_support.txt` |
| **S4** Normal form and capacity DP | Hall feasibility, exact counting, the marginal criterion, canonical reduction, the normal-form trichotomy, the one-assignment SCC compiler and its decoder, the exact count-ratio uniform sampler, and the standalone census — ending at zero supplemental bits given mechanical state. | `verify_normal_form.txt` |

## Slice 02 — dynamics, reachability, and the outer language

Five gated stages plus one optional stretch stage. Slice 02 is also where the
**exchange tier** first enters rob: five dispatches had been adjudicated
CONFIRMED, and the brief's standing instruction was that such a result is citable
for test expectations and constructions but is **not a corpus theorem**. Receipt
lines carrying such expectations are `x-` prefixed and name their ledger entries.

| Stage | Established | Receipt |
|---|---|---|
| **S5** Matching-minor calculus | The abstract typed transition on the normal form (force, delete, contract, reduce), its equivalence to extensional conditioning, monotonicity, the game-typed wrapper, and native-world sampling. | `verify_dynamics.txt` |
| **S6** Symbolic trace validator | Deal-free reachability witnesses as the external-state gate, over a hand corpus, with the 63-edge budget checked end to end. | `verify_symbolic.txt` |
| **S7** Necessary outer language | The schedule language, lead-witness coefficients, per-profile counts, the five-check validator, the ceilings, and a Burnside supplement. Backs exchange results x:002 and x:005. | `verify_outer.txt` |
| **S8** Unreachability regressions | The classic REACH-10 witness reproduced, plus the stronger x:002 witness with its follower-supply check — the regressions that keep *feasible ≠ reachable* honest. | `verify_unreachable.txt` |
| **S9** Transport-aware census | The three-class declaration quotient and a corpus commutation check: transport commutes with reachability (x:004). | `verify_transport.txt` |
| **S10** (stretch) Reachable floor | The x:001 admissible modules and witness-mask languages, the exact upward-closure count, and the resulting bit floor. rob enumerates the full principled profile space, a strict superset of the space the exchange program tabled. | `verify_floor.txt` |

The exact integers each of these reproduces, and which of them are `x-` lines,
are owned by [verification](verification.md). The mathematics is owned by
[support-dynamics](support-dynamics.md), [reachability](reachability.md), and
[minimal-support-normal-form](minimal-support-normal-form.md).

## The player track — P1 to P5

A parallel track rather than a census slice, and the place where a naming
decision matters: **the player specified by `BRIEF_PLAYER_01.md` is rob**. The
earlier fixed-field Monte Carlo player was demoted to *baseline*, retained as the
paired-match opponent and as the owner of the `verify_player.txt` determinism
transcript.

| Stage | Established |
|---|---|
| **P1** The field policy σ | A fixed, deterministic, points-blind continuation policy — the structural form of the anti-strategy-fusion law. One policy object is fixed before any world is drawn, and a clone with the identical private tape replays every seat in every simulated world; no per-world optimisation hook exists anywhere in the player API. |
| **P2** Position corpus and fiber accounting | The exact-window budget formula and the corpus of positions it is evaluated over, including the window histogram frozen in the receipt. |
| **P3** The exact solver | The W0 information-set best-response solver, in two engines (streaming and counting) cross-validated against each other and against brute force on small positions. |
| **P4** rob at the table | Rolling re-solve at every decision, and the mirrored paired match against the baseline. The margins here are **frozen measurements, not targets**. |
| **P5** The contingency book | Deterministic plan-tree emission with a strict parser and capped projection — the whole-hand contingent plan as a byte-stable artifact. |
| **P6** (stretch) | A σ-consistency history filter. **Not built.** No such receipt rows exist. |

All five green stages print into the single receipt `verify_rob.txt`.

## What comes next

Named in the briefs, not begun:

- **Slice 03 — the folded trick and the reduced viewer kernel.** The brief
  specifies its receipts in advance (trick cases, sequential updates, open-trick
  shapes, score-recovery prefixes, dihedral frames, and a future-equivalence
  corpus), and requires the x:003 collapse witness as a regression — a witness
  that needs the fold to exist before it can be stated at all. `BRIEF.md` §4
  lists it among the later slices that are explicitly out of scope — "do not
  begin, do not scaffold speculative APIs for" — and `BRIEF_SLICE_02.md` §13 says
  "Do not begin slice 03" outright.
- **Slice 03 targets from the exchange side** — reproducing the filtered census
  (x:007) and the no-void slice (x:008) in Rust.
- **Slice 04** — the belief and filtering layer, with the 90-world posterior-flip
  as its regression. This is the unassigned slice recorded at
  [first-implementation-slice](first-implementation-slice.md).
- **Slice 05** — the solver and census frontier, aimed at OPEN-11 over the
  symbolic DAG ([open-problems](open-problems.md)).
