# Ledger Row → Lean Declaration Index

[lean](lean.md) · [Home](Home.md) · owns: the map from mechanization-ledger rows
to the Lean declarations that discharge them · Sources: **v0.7**
`65_MECHANIZATION_LEDGER.md`; [lean/README.md](../lean/README.md); the `PA-`
citations in the Lean docstrings. Related: [lean](lean.md),
[proof-assistant-plan](proof-assistant-plan.md).

The mechanization ledger in the **v0.7** package is the proof-assistant work
queue. It is deliberately much smaller than the full claim ledger: a row enters
it only when later core theorems depend on it, or when it guards a known
historical overclaim. Each row carries a **route** — the ledger's own vocabulary,
verbatim:

> - `DEFINE` — introduce the object with decidable equality/finite enumeration;
> - `PROVE` — direct kernel proof preferred;
> - `REFLECT` — proved internal decision procedure plus kernel evaluation;
> - `WITNESS` — concrete internal counterexample/computation;
> - `DEFER` — not needed for the first closed finite foundation.

…and a **priority**. A *priority-0 row* is simply one whose priority cell reads
`0`, and the ledger's definition of done for the first release is that all of
them close:

> The first formal release should include all priority-0 rows and no unmarked
> axioms beyond the adopted Straight rule profile and ordinary foundational
> library assumptions. Every external finite receipt not yet internalized must
> remain visibly external in the generated theorem inventory.

There are exactly **42** such rows across the ledger's six sections — 13 in A, 7
in B, 9 in C, 7 in D, 5 in E, 1 in F — and as of 2026-08-02 all 42 are
kernel-proved. Row IDs are not contiguous: the gaps are rows at priority 1 or
higher, several of which are also proved (listed at the end).

**Tier: proof-assistant kernel** — the declarations below are checked by the Lean
kernel over the standard axioms
([Home](Home.md#evidentiary-tiers--never-promoted-never-blurred)).

**How this index is maintained.** By hand. Nothing generates or checks it: the
ledger lives in immutable ingest, the declarations live in `lean/`, and the join
between them is the `PA-` citations in the Lean docstrings plus the layout list
in [lean/README.md](../lean/README.md). Treat a mismatch as a bug in this page
until proven otherwise, and re-derive from the docstrings. All modules below are
under `lean/Texas42/`.

## A — finite rule algebra (13 rows)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-A01 | DEFINE | the finite types: pip, domino, seat, team, declaration, led suit | `Basic.lean`, `Trick.lean` | `Pip`, `Seat`, `Team`, `Domino`; `Declaration` + `card_declaration`, `Suit` + `card_suit` |
| PA-A02 | PROVE/REFLECT | exactly 28 canonical dominoes | `Basic.lean` | `Domino.card_domino` |
| PA-A03 | PROVE | natural incidence covering and pair intersections | `Basic.lean` | `Domino.incidence`, `card_incidence`, `card_pip_memberships`, `incidence_inter` |
| PA-A04 | PROVE/REFLECT | the count-point function and its total, 35 | `Basic.lean` | `Domino.countPoints`, `Domino.total_countPoints` |
| PA-A05 | DEFINE | called, powered and effective suit for the nine declarations | `Trick.lean` | `Declaration.called`, `.powered`, `.effMem`, `.ledSuit` |
| PA-A06 | PROVE | effective membership is one or two; called absorption | `Trick.lean` | `not_effMem_natural_of_called`, `effMem_natural_iff`, `card_effMem` |
| PA-A07 | PROVE | led context and follow exactness | `Trick.lean` | `Declaration.effMem_ledSuit` |
| PA-A08 | DEFINE | rank, tier, trick key | `Trick.lean` | `Declaration.rank`, `.tier`, `.key` |
| PA-A09 | PROVE | a lead always occupies a nonzero tier | `Trick.lean` | `Declaration.tier_ledSuit_pos` |
| PA-A10 | PROVE | rank is injective inside each possible winning tier | `Trick.lean` | `eq_of_key_eq`, with `eq_of_powered_of_rank_eq`, `eq_of_effMem_of_rank_eq`, `eq_of_hasPip_of_rank_eq` |
| PA-A11 | PROVE | the unique trick winner | `Trick.lean` | `Declaration.existsUnique_winner` |
| PA-A13 | PROVE | contextual BEATS exactness | `Trick.lean` | `Beats`, `beats_exact`, `beatsSet`, `threat` |
| PA-A14 | PROVE | live-threat monotonicity | `Trick.lean` | `threat_removal_mono` |

Layer A is complete at priority 0. The one conspicuous absence is **PA-A12**
(priority 2), the exhaustive 737,100-case agreement with an independent prose
resolver, which stays a deliberate reflection target rather than being folded
into PA-A11 — the winner theorem is proved structurally, not by enumeration.

## B — auction, contract, and objective hand (7 rows)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-B01 | DEFINE | configured Straight auction state and action types | `Auction.lean` | `Bid`, `Bid.value`, `AuctionConfig`, `AuctionState` |
| PA-B02 | DEFINE/PROVE | auction legality and the deterministic transition | `Auction.lean` | `legalBid`, `step`, `LegalAuction`, `result` |
| PA-B06 | DEFINE | contract, declaration and award | `Play.lean` | `Contract`, `.threshold`, `.stake`, `.Makes`, `.award`, `.ofBid` |
| PA-B07 | DEFINE | phase-indexed location and contracted-play state | `Play.lean` | `PlayState`, `.actor`, `.gamma`, `.tricksDone`, `.scoredTiles` |
| PA-B08 | PROVE | the legal-play characterization | `Play.lean` | `legalSet`, `legalSet_lead`, `legalSet_follow`, `legalSet_slough`, `legalSet_nonempty` |
| PA-B09 | PROVE | the atomic transition preserves partition and location invariants | `Play.lean` | `step`, `Inv`, `inv_step`, `inv_init` |
| PA-B10 | PROVE | seven tricks, 28 plays, 42 total points | `Play.lean` | `gamma_init`, `gamma_step`, `terminal_scores` |

## C — information, cells, and fiber (9 rows)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-C01 | DEFINE | primitive public/private event history and perfect-recall information | `Information.lean`, `Cells.lean` | `DealLocalInfo`; `ViewerCtx` |
| PA-C02 | DEFINE | mechanical projection and the derived unseen/capacity/possible cells | `Cells.lean` | `PubState`, `PubState.step`, `ViewerCtx.pool`, `.allowed`, `.capacity` |
| PA-C03 | DEFINE | cell dependency and the Hall feasibility statement | `Cells.lean`, `NormalForm.lean` | `ViewerCtx.capacity`/`.allowed`; `CellSys.Feasible` |
| PA-C04 | DEFINE | the current-remainder world and the fiber | `Cells.lean` | `ViewerCtx.IsWorld`, `.Compatible`, `.remainder` |
| PA-C05 | PROVE | the initial deal/remainder correspondence | `Cells.lean` | `ViewerCtx.losslessness` (see the note below) |
| PA-C06 | PROVE | the upper-bound-only observation update | `Cells.lean` | `PubState.voids_mono`, `.voidsAfter` and its lead/follow/slough lemmas |
| PA-C07 | PROVE by induction | cell/fiber losslessness over legal Straight prefixes | `Cells.lean` | `ViewerCtx.losslessness`, with completeness by `exists_deal_of_isWorld` |
| PA-C09 | PROVE | the hidden-play typed predecessor/successor bijection | `Cells.lean` | `ViewerCtx.remainder_injective`, groundwork `Coheres.step` |
| PA-C10 | PROVE | viewer-play identity on hidden remainders | `Cells.lean` | `hands_step_ne`, `legalSet_congr`, `allowed_step_viewer` |

**A citation gap worth knowing.** PA-C05 is the one priority-0 row with no `PA-`
citation anywhere in the Lean source. It is covered by `Cells.lean`'s module
docstring, which names the range PA-C01–PA-C07, and by
[lean/README.md](../lean/README.md), which tags `losslessness` as PA-C05/C07 —
but the theorem's own docstring names only PA-C07. Every other priority-0 row is
cited in-source. Reported, not resolved; the discipline's rule that every theorem
docstring names its row would close it.

Layer C also produced the library's most interesting **mechanization finding**:
the completeness half of the losslessness induction needs a fact the prose leaves
implicit — a hidden seat's publicly played tile must respect that seat's
previously recorded voids. The Lean proof derives it from true-trajectory void
soundness rather than assuming it.

## D — normal form and reachability (7 rows)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-D01 | PROVE | the determinate/binary/ternary trichotomy | `NormalForm.lean` | `active_trichotomy`, `holders_eq_active_of_two`, `excl_card_le_one_of_three` |
| PA-D02 | DEFINE | the well-formed feasible support normal form | `NormalForm.lean` | `SupportNF`, `SupportNF.WellFormed`, `CellSys.wellFormed_compile` |
| PA-D03 | PROVE | decode is feasible and reconstructs the reduced holder relation | `NormalForm.lean` | `SupportNF.decode`, `feasible_decode`, `decode_marginal` |
| PA-D04 | PROVE | the compile/decode inverse laws | `NormalForm.lean` | `CellSys.decode_compile`, `SupportNF.compile_decode` |
| PA-D05 | PROVE | the total support normal form classifies exact fibers | `NormalForm.lean` | `CellSys.totalNF`, `fiber_eq_iff_totalNF_eq` |
| PA-D09 | DEFINE | the reachability predicate on mechanical states | `Reachability.lean` | `Reachable`, `reachable_init` |
| PA-D10 | PROVE / extensionality | reachability is proof-irrelevant semantic evidence | `Reachability.lean` | `CertifiedState`, `.ext`, `.ext_iff` |

PA-D10 is the kernel-side counterpart of rob's INV-3: the certified state has no
identity-bearing content, and its extensionality lemma is what says so.

## E — finite belief and the strategic boundary (5 rows)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-E01 | DEFINE | the finite PMF prior and the policy/field kernel | `Belief.lean` | `FinPMF`, `PolicyKernel` |
| PA-E02 | PROVE | the history likelihood product and posterior normalization | `Belief.lean` | `FinPMF.condition`, `condition_mul`, `likelihoodFrom`, `likelihoodFrom_append`, `posterior` |
| PA-E03 | PROVE | pushforward of belief to the current remainder fiber | `Belief.lean` | `FinPMF.map`, `physicalBelief`, `physicalBelief_support_isWorld` |
| PA-E07 | PROVE | exact augmented strategic-state sufficiency for a fixed field and utility | `Strategic.lean` | `BeliefProc`, `.latentVal`, `.filter`, `.beliefVal`, `beliefVal_eq_exp_latentVal`, `bestResponse_eq` |
| PA-E10 | WITNESS/REFLECT | the same-support 90-world posterior and action reversal | `Witness.lean` | `Witness.ninety_world_witness` |

PA-E03's closing lemma is the precise kernel statement of *support is not belief*
from the other direction: the posterior's support lies **inside** the cell fiber,
so the fiber bounds belief exactly without determining it. PA-E10 is the
counterexample showing the bound cannot be tightened to an identity — see
[belief-vs-support](belief-vs-support.md) for the mathematics and [lean](lean.md)
for what "internalized whole" means.

## F — quotients, gauges, and boundaries (1 row)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-F05 | PROVE | physical congruence versus information-state equality | `Information.lean` | `mech_not_injective`, with `DealLocalInfo.mech` |

## Beyond priority 0

Rows above priority 0 that are nonetheless already kernel-proved, so that nobody
proves them twice:

| Row | Priority | Discharged by |
|---|---|---|
| PA-A15 | 1 | `Declaration.lead_threat_incomplete` (`Trick.lean`) |
| PA-A16 | 1 | `countPreserving_iff` (`Transport.lean`) |
| PA-A17 | 1 | `swap23_transport_iff` (`Transport.lean`) |
| PA-B03 | 1 | `mark_le_ceiling`, `mark_five_reachable` (`Auction.lean`) |
| PA-B05 | 1 (define half) | `Deal` and its lemmas (`Deal.lean`); the deal cardinalities remain open |
| PA-C08 | 1 (groundwork) | `CellSys.exists_partition_of_hall` (`NormalForm.lean`) |
| PA-C15 | 1 (backbone) | `CellSys.red`, `red_red`, `fiber_eq_iff_red_eq` (`Reduction.lean`) |

The remaining priority-1 tiers and the two open pure-`REFLECT` rows (PA-A12, the
737,100-case resolver agreement, and PA-B04, the exact auction history counts)
are the live queue; [proof-assistant-plan](proof-assistant-plan.md) owns that
scoreboard. The two constellation modules carry **no ledger row at all** — they
came from the exchange Lean thread and are not yet reconciled with the main
layers ([lean](lean.md)).
