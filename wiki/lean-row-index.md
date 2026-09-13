[Home](Home.md) · owns: the map from mechanization-ledger rows to the Lean declarations that discharge them, and the explicit list of what the ledger still leaves open · Sources: **v0.7** `65_MECHANIZATION_LEDGER.md` (83 rows, recounted 2026-09-07); [lean/README.md](../lean/README.md); the `PA-` citations in the Lean docstrings under `lean/Texas42/` (as of commit c00717d1). Related: [lean](lean.md) (the chapter), [proof-assistant-plan](proof-assistant-plan.md) (the plan and scoreboard).

# Ledger Row → Lean Declaration Index

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

The ledger has **83** `PA-` rows (17 A, 14 B, 15 C, 18 D, 12 E, 7 F). Exactly
**42** are priority 0 — 13 in A, 7 in B, 9 in C, 7 in D, 5 in E, 1 in F — and
as of 2026-08-02 (commit d190b264) all 42 are kernel-proved. Row IDs are not
contiguous: the gaps are rows at priority 1 or higher, several of which are also
proved (§ Beyond priority 0). **[bookkeeping: recounted against the ledger
2026-09-07]**

**Tier: proof-assistant kernel** — the declarations below are checked by the Lean
kernel over at most the standard axioms `propext`, `Classical.choice`,
`Quot.sound` ([Home](Home.md#evidentiary-tiers--never-promoted-never-blurred)).
Several receipts show a theorem depending on `[propext]` alone or on no axiom;
"at most the three" is the accurate wording.

**How this index is maintained.** By hand. Nothing generates or checks it: the
ledger lives in immutable ingest, the declarations live in `lean/`, and the join
between them is the `PA-` citations in the Lean docstrings (48 distinct rows
cited in source as of 2026-09-07) plus the layout list in
[lean/README.md](../lean/README.md). Treat a mismatch as a bug in this page
until proven otherwise, and re-derive from the docstrings. All modules below are
under `lean/Texas42/`.

**Modules with no ledger row.** `Trick1Foundation.lean`,
`Trick1MetalFoundation.lean` and the `Trick1PerfectRecallNet` tree carry **no
ledger row**; their citations are walt's GT1-A rulings (GT1-A8, GT1-A17,
GT1-A23/A24), not `PA-` rows, and what they certify about walt is exploratory
([lean](lean.md) §7). `ConstellationCore.lean` and `ConstellationSuffix.lean`
likewise carry no ledger row — they came from the exchange Lean thread
(x:013, x:015) and are not reconciled with the main layers ([lean](lean.md) §8).
None of the five appears in the tables below.

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

**Two citation gaps worth knowing** (verified by grep, 2026-09-07). PA-C05 is
the one priority-0 row with no `PA-` citation anywhere in the Lean source: it is
covered by `Cells.lean`'s module docstring, which names the range PA-C01–PA-C07,
and by [lean/README.md](../lean/README.md), which tags `losslessness` as
PA-C05/C07 — but the theorem's own docstring names only PA-C07. PA-C03 is cited
in source only in a section-header comment, `(PA-C02/C03, Math §7.1)` at
`Cells.lean:450`, not in any declaration docstring. Every other priority-0 row is
cited in a docstring. Reported, not resolved; the discipline's rule that every
theorem docstring names its row would close both.

Layer C also produced the library's most interesting **mechanization finding**:
the completeness half of the losslessness induction needs a fact the prose leaves
implicit — a hidden seat's publicly played tile must respect that seat's
previously recorded voids. The Lean proof derives it (`hd_allowed`,
`Cells.lean:884`) from true-trajectory void soundness rather than assuming it
([lean](lean.md) §4.5).

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

PA-D10 is the kernel-side counterpart of rob's INV-3 and the wiki's D3 ruling:
the certified state has no identity-bearing content, and its extensionality
lemma is what says so. (`CertifiedState` is Handoff §5's own identifier, quoted
as such; the wiki's word for the object is *necessary outer profile*.)

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
§6 for what "internalized whole" means.

## F — quotients, gauges, and boundaries (1 row)

| Row | Route | Obligation | Module | Discharged by |
|---|---|---|---|---|
| PA-F05 | PROVE | physical congruence versus information-state equality | `Information.lean` | `mech_not_injective`, with `DealLocalInfo.mech` |

## Beyond priority 0

The ledger has 27 rows at priority 1, 11 at priority 2, one at priority 3 and
two at priority 4. **[bookkeeping against the ledger, 2026-09-07]**

### Priority-1 rows already kernel-proved (so that nobody proves them twice)

| Row | Priority | Discharged by |
|---|---|---|
| PA-A15 | 1 | `Declaration.lead_threat_incomplete` (`Trick.lean`) |
| PA-A16 | 1 | `countPreserving_iff` (`Transport.lean`) |
| PA-A17 | 1 | `swap23_transport_iff` (`Transport.lean`) |
| PA-B03 | 1 | `mark_le_ceiling`, `mark_five_reachable` (`Auction.lean`) |

### Priority-1 rows proved in part (three half-rows)

| Row | Proved | Open |
|---|---|---|
| PA-B05 | `Deal` and its lemmas `biUnion_eq_univ`, `existsUnique_mem`, `owner_eq` (`Deal.lean`) — the define half | the cardinalities `28!/(7!)⁴ = 472,518,347,558,400` and `21!/(7!)³ = 399,072,960` as cardinalities of the deal type (`Trick1Foundation.openingDealCount_eq_multinomial` proves the second only as a `Nat` identity) |
| PA-C08 | `CellSys.exists_partition_of_hall` (`NormalForm.lean`) — the generic capacitated Hall lemma by slot expansion into mathlib's Hall theorem | the game-level Hall/max-flow feasibility equivalence |
| PA-C15 | `CellSys.red`, `red_red`, `fiber_eq_iff_red_eq`, `isWorld_iff_cellSys` (`Reduction.lean`) — the backbone: reduction is fiber-preserving, idempotent, and the coarsest exact quotient | the row's dependence on the open PA-C14 (marginal holder edge iff forced successor feasible); no finer statement of the residue is recorded anywhere |

### Priority-1 rows open, 20 — the live queue

| Row | Route | Target |
|---|---|---|
| PA-B11 | PROVE | fixed-hand play graph is finite and graded |
| PA-B12 | PROVE | objective physical Markov congruence |
| PA-B13 | PROVE | deterministic contract/mark settlement |
| PA-C11 | PROVE | exact capacity-DP count recurrence and soundness |
| PA-C13 | WITNESS | local possible holder is not marginal possible holder |
| PA-C14 | PROVE | marginal holder edge iff forced successor feasible |
| PA-D06 | PROVE | global deterministic support minimality/factorization |
| PA-D11 | PROVE | reachable support image and restricted minimality |
| PA-D13 | PROVE | seven observable lead contexts per declaration |
| PA-D14 | PROVE | lead-witness necessity |
| PA-D15 | PROVE | witness validator soundness/completeness |
| PA-D16 | WITNESS | feasible-but-unreachable support |
| PA-E04 | PROVE | physics-only posterior under uniform chance assumptions |
| PA-E05 | PROVE | finite exponential-tilt form |
| PA-E06 | PROVE | forced-action world-nondiscrimination |
| PA-E08 | PROVE | deterministic information-set best-response existence |
| PA-E09 | PROVE | coordinate-only scalar factorization criterion |
| PA-E11 | WITNESS | context-free domino value counterexample |
| PA-F01 | PROVE | local hand-order invariant/equivariant gauge |
| PA-F07 | PROVE/BOUNDARY | shared utility does not centralize partner information |

(Any list that names PA-B14 or PA-D12 as priority 1 is wrong: both are priority
2 in the ledger. [proof-assistant-plan](proof-assistant-plan.md) carried that
error from 2026-08-02 until 2026-09-12.)

### Priority 2 and above

| Priority | Rows |
|---|---|
| 2 | PA-A12 (737,100-case resolver agreement, REFLECT), PA-B14, PA-C12, PA-D07, PA-D08, PA-D12 (the 50 hidden-capacity profiles), PA-E12, PA-F02, PA-F03, PA-F04, PA-F06 |
| 3 | PA-B04 (exact auction history counts for caps 1..7, REFLECT) |
| 4 | PA-D17 (81-bit support census, "REFLECT or keep external"), PA-D18 (26–46-bit reachable interval) |

The two pure-`REFLECT` rows (PA-A12, PA-B04) and the two census rows are the
receipts the ledger's definition of done allows to "remain visibly external";
whether reflection is still the intended route for them is undecided.
[proof-assistant-plan](proof-assistant-plan.md) owns the scoreboard; the
walt-era programs with no kernel coverage (CBS-O1..O15, PS-T1..T15, MB-O1..O20,
SC-O1..O15, the [[lean-catchup]] card) are listed in [lean](lean.md) §10.2 —
they are outside this ledger entirely.
