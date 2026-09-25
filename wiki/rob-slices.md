[Home](Home.md) · owns: the record of what each rob brief assigned and what each stage established, and the ledger of what was named but never begun · Sources: [rob/BRIEF.md](../rob/BRIEF.md), [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md), [rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md); the commit history of `rob/` (read-only, as of 2026-09-07, c00717d1). Related: [rob](rob.md), [verification](verification.md), [first-implementation-slice](first-implementation-slice.md).

# rob's Build History — the Briefs and their Stages

rob was not written all at once. It was built in **gated stages**: a stage begins only when the previous stage's receipts are green, and each stage's deliverable is a receipt whose exact contents were specified in the brief *before* the code existed. This page is the record of that sequence — what was assigned, what each stage established, which receipt witnesses it, and what was named and never started.

The division of labour with the neighbouring pages: [first-implementation-slice](first-implementation-slice.md) owns the original slice-01 *assignment* as a historical document (it predates rob and was superseded by `BRIEF.md`); [verification](verification.md) owns every receipt's exact integers; [rob](rob.md) §6–§7 carries the receipt-by-receipt table with commits; this page owns the *shape* of the build.

**Tier note.** Everything below is [rob conformance receipts, tier 4]. A green stage means rob independently reproduced the stated finite claims and that its invariants held; it never means a claim changed status ([Home](Home.md#evidentiary-tiers--never-promoted-never-blurred)).

## The brief family

Three binding assignments, each a self-contained contract with layout, invariants, receipt rows, and a definition of done. They are **read-only history**: they are not edited to match later work, so a brief may describe a module that was later folded elsewhere or a budget that a later amendment changed. Read each with its own amendment notes.

| Brief | Written | Track | Stages | Receipts |
|---|---|---|---|---|
| [rob/BRIEF.md](../rob/BRIEF.md) | 2026-07-26 (e8ccc176) | Slice 01 — declaration algebra through the normal form and capacity DP | S1–S4 | 4 |
| [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md) | 2026-07-27 (af7b0a31, d6e3d384) | Slice 02 — support dynamics through the outer language, with a stretch stage | S5–S9 plus S10 | 6 |
| [rob/BRIEF_PLAYER_01.md](../rob/BRIEF_PLAYER_01.md) | 2026-07-28 (48107b6f; amended the same day, 0c7f03fb) | The player track — σ, the exact solver, rob at the table | P1–P5 (P6 stretch, not built) | 1 |

A twelfth receipt, `verify_player.txt`, belongs to none of the three: the baseline Monte Carlo player was added between slices 01 and 02 (b22d4b3d, 2026-07-27) and keeps its self-play transcript as a whole-stack determinism regression.

Slice 01 also introduced the ten invariants INV-1 … INV-10 (derived-not-stored, projected equality, proof-irrelevant reachability, exact arithmetic, counts-are-CI, reachable-implies-feasible, no-rank-from-id, one-source-of-truth, type-distinct domains, vocabulary); slice 02 added INV-11 … INV-14 (edge budget, monotone ambiguity, typed-transition-only, and the outer language stays necessary-only); the player brief added INV-P1 … INV-P7. The full invariant-to-enforcement table is [rob](rob.md#the-invariants-and-how-each-is-enforced) §3.

## Slice 01 — the mathematical spine (2026-07-27)

Four gated stages taking rob from the tile universe to the exact support normal form, all green on one day.

| Stage | Commit | Established | Receipt |
|---|---|---|---|
| **S1** Declaration algebra | 91b7c991 | The 28-tile universe, the nine declarations as relational algebras, contextual rank and tier, the unique trick winner, pip transports with the scored/unscored split, and the three unscored mechanics classes. Trick resolution is cross-checked against an *independent prose-rule resolver* written from the rules text, not from the algebra (D4). | `verify_algebra.txt` |
| **S2** Objective hand machine | b18d6359 | Rules configuration, the one-round auction and its exhaustive tree, contract certification and settlement, and the phase-indexed contracted-play state with certified lifecycle constructors — plus legal-play and 42-point conservation over a hand corpus. | `verify_objective.txt` |
| **S3** Cells as a derived view | 651824fb | Capacity cells derived from mechanical state rather than stored beside it, the typed update algebra, and a replay-parity corpus proving the derived view and the incremental updates agree. This stage owns the frozen with-voids generator value (970). | `verify_support.txt` |
| **S4** Normal form and capacity DP | 6639c833 | Hall feasibility, exact counting, the marginal criterion, canonical reduction, the normal-form trichotomy, the one-assignment SCC compiler and its decoder, the exact count-ratio uniform sampler, and the standalone census — ending at zero supplemental bits given mechanical state. | `verify_normal_form.txt` |

## Slice 02 — dynamics, reachability, and the outer language (2026-07-27)

Five gated stages plus one optional stretch stage, green the same day as slice 01. Slice 02 is also where the **exchange tier** first enters rob: five dispatches (x:001–x:005) had been adjudicated CONFIRMED that morning, and the brief's standing instruction was that such a result is citable for test expectations and constructions but is **not a corpus theorem**. Receipt lines carrying such expectations are `x-` prefixed and name their ledger entries in the receipt header.

| Stage | Commit | Established | Receipt |
|---|---|---|---|
| **S5** Matching-minor calculus | 05d1df86 | The abstract typed transition on the normal form (force, delete, contract, reduce), its equivalence to extensional conditioning, monotonicity, the game-typed wrapper, and native-world sampling. | `verify_dynamics.txt` |
| **S6** Symbolic trace validator | 513a2e01 | Deal-free reachability witnesses as the external-state gate, over a hand corpus, with the 63-edge budget checked end to end. | `verify_symbolic.txt` |
| **S7** Necessary outer language | dd40bfa6 | The schedule language, lead-witness coefficients, per-profile counts, the five-check validator, the ceilings, and a Burnside supplement. Backs exchange results x:002 and x:005. | `verify_outer.txt` |
| **S8** Unreachability regressions | e5a69d5c | The classic REACH-10 witness reproduced, plus the stronger x:002 witness with its follower-supply check — the regressions that keep *feasible ≠ reachable* honest. | `verify_unreachable.txt` |
| **S9** Transport-aware census | 4ce6e16c | The three-class declaration quotient and a corpus commutation check: transport commutes with reachability (x:004). | `verify_transport.txt` |
| **S10** (stretch) Reachable floor | fa027ab7 | The x:001 admissible modules and witness-mask languages, the exact upward-closure count, and the resulting bit floor. rob enumerates the full principled profile space (369 profiles), a strict superset of the 216 the exchange program tabled. | `verify_floor.txt` |

The exact integers each of these reproduces, and which of them are `x-` lines, are owned by [verification](verification.md). The mathematics is owned by [support-dynamics](support-dynamics.md), [reachability](reachability.md), and [minimal-support-normal-form](minimal-support-normal-form.md).

## The player track — P1 to P5 (2026-07-28)

A parallel track rather than a census slice, and the place where a naming decision matters: **the player specified by `BRIEF_PLAYER_01.md` is rob**. The earlier fixed-field Monte Carlo player was demoted to *baseline*, retained as the paired-match opponent and as the owner of the `verify_player.txt` determinism transcript. The brief was written as an endgame plan solver and amended the same day to whole-hand play from trick 1 under a budgeted exact window (B: 2³² → 2²⁸, with the counting-engine rule for window-1 solves).

| Stage | Commit | Established |
|---|---|---|
| **P1** The field policy σ | ae6af953 | A fixed, deterministic, points-blind continuation policy — the structural form of the anti-strategy-fusion law. One policy object is fixed before any world is drawn, and a clone with the identical private tape replays every seat in every simulated world; no per-world optimisation hook exists anywhere in the player API. |
| **P2** Position corpus and fiber accounting | 392bdee8 | The exact-window budget formula and the 756-position corpus it is evaluated over, with the closed-form fiber bounds and the window histogram frozen in the receipt. |
| **P3** The exact solver | 4140d203 | The W0 information-set best-response solver, in two engines (streaming and counting) cross-validated against each other and against brute force on small positions; four correctness gates. |
| **P4** rob at the table | 6d3409f1 | Rolling re-solve at every decision, and the mirrored paired match against the baseline (net +718) with its B/2 and 2B ablation. The margins here are **frozen measurements, not targets**. |
| **P5** The contingency book | 6d3409f1 | Deterministic plan-tree emission with a strict parser and capped projection — the whole-hand contingent plan as a byte-stable artifact. |
| **P6** (stretch) | — | A σ-consistency history filter (W1). **Not built.** No such receipt rows exist. |

All five green stages print into the single receipt `verify_rob.txt`.

## After the receipts (2026-07-28 → 2026-08-01)

The code kept moving for four days after the last receipt, all of it exploratory instrumentation rather than stages: the ablation probes and `trace_rob` as the inspector default (2026-07-28), `gate::solve_opening` with the nickel autopsy and the σ-counterfactual probe (2026-07-28), the hierarchical-fibers round (2026-07-29), `rob_bridge` (2026-07-29) and its fmt fix (2026-07-31), the retrograde rank probe (2026-07-31), the constellation vocabulary rework and the k=1 census and k=2 probe (2026-08-01). The k=2 probe, cd51ce2e, is the last change to rob's code. [analysis](analysis.md) owns these instruments; [field/](field/Home.md) owns what the bridge measured.

## Named but never begun (as of 2026-09-07)

The briefs name three further slices and one stretch stage. None was started; no scaffolding for any of them exists. Two dates frame them: rob's last code commit is **2026-08-01** (cd51ce2e), and on **2026-08-17** the project pivoted to [walt](walt.md) as the player under the pmake objective. That pivot said nothing about rob's ladder — **no ruling parks or cancels slices 03–05 or P6**, and none schedules them. Their status is therefore an **open call for Jason**, recorded here so the silence is visible rather than mistaken for a plan. QUICKSTART still lists "rob slice 03 targets" as if queued.

- **Slice 03 — the folded trick and the reduced viewer kernel.** The brief specifies its receipts in advance (trick cases, sequential updates, open-trick shapes, score-recovery prefixes, dihedral frames, and a future-equivalence corpus — the 2,211,300 / 84 / 3,132 / 8 / 5,898 / 17,560 numbers `BRIEF.md` §8 lists as *not* slice-01 targets), and requires the x:003 collapse witness as a regression — a witness that needs the fold to exist before it can be stated at all. `BRIEF.md` §4 lists it among the later slices that are explicitly out of scope — "do not begin, do not scaffold speculative APIs for" — and `BRIEF_SLICE_02.md` §13 says "Do not begin slice 03" outright. Exchange-side targets attached to it: reproducing the filtered census (x:007) and the no-void slice (x:008) in Rust; the reachable-census interval of x:006 (REACH-18) was never attached to any slice, and rob has reproduced none of REACH-18/19/20 ([exchange](exchange.md) §§2, 4).
- **Slice 04 — the belief and filtering layer**, with the 90-world posterior flip as its regression. This is the unassigned slice recorded at [first-implementation-slice](first-implementation-slice.md). rob has support and no belief; the typed distinction *support ≠ belief* ([belief-vs-support](belief-vs-support.md)) is, in rob, the absence of this slice.
- **Slice 05 — the solver and census frontier**, aimed at OPEN-11 over the symbolic DAG ([open-problems](open-problems.md)).
- **P6 — the W1 σ-consistency history filter** with its ablation (tests `r_w1_*`). Dead or deferred is not ruled either way.

What did happen instead, for orientation only: walt built its own belief layer (model belief MB0/MB1, the void-aware inner belief) and its own frontier programs at the exploratory tier — see [walt-program](walt-program.md) — without touching rob's ladder.
