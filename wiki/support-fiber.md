# Capacity Cells and the Current-Remainder Fiber

[Home](Home.md) · owns: the cells, the fiber, the losslessness theorem · Sources:
both packages Math §§6–7 (shared). Related: [capacity-dp](capacity-dp.md),
[minimal-support-normal-form](minimal-support-normal-form.md),
[belief-vs-support](belief-vs-support.md).

## Three domains that must not be conflated

```
complete initial deal  →(remainder map under fixed public history)→
current hidden-remainder world  →(collect compatible ones)→
current-remainder fiber
```
A deal is fixed for the attempt; a remainder *changes type* as plays remove tiles; the
fiber is a set of remainders. Posterior belief is a measure pushed onto the fiber, not
the fiber itself [Definitions/Boundaries, Math §6.3, §6.7; README "Domain discipline"].

## The cells

Fix viewer `m` during a declared hand. From the mechanical state derive
[Definition, Math §7.1]:

- unseen pool `U` = all tiles minus own hand minus all publicly played tiles;
- per hidden seat `s`: public void set `V_s` (contexts where `s` visibly failed to
  follow), locally allowed set `P_s = U \ ⋃_{q∈V_s} σ̂_q^δ`, and capacity
  `k_s = 7 − |played by s|`.

Cells are **dependent** — they share one conserved pool (two seats, two tiles,
capacity 1 each: only 2 of 4 "independent" assignments are worlds)
[Constructed counterexample, Math §7.2, CELL-01].

The **fiber** `Φ(C)` is the *intensional* set of triples of disjoint hidden hands with
`H_s ⊆ P_s`, `|H_s| = k_s`, union `U` [Definition, Math §7.3]. Enumeration is a query
on it (up to `21!/(7!)³ = 399,072,960` worlds), never its definition; no hidden caps or
sampling [README "Correctness before tractability"]. The fiber factors through the cell
system: equal cells ⇒ equal fibers even when leader/score/contract differ [CELL-02A].

## The losslessness theorem (the keystone)

**[Theorem — proved by induction on public plays, Math §7.5, CELL-05]**
For every legal Straight public prefix in scope:

```
Φ(c) = ρ(Ω(I))
```

— the cell fiber equals the image of all rule-compatible complete deals under the
remainder map. The proof's case analysis: viewer actions don't touch hidden hands;
a hidden lead/follow removes the played tile (the tile itself is the witness — **no
positive "still holds a follower" clause survives**, CELL-06); a failure to follow
deletes the whole follow set from that seat's allowed set. Each update is reversible,
giving the **fixed-history bijection** between compatible deals and current remainders
[Corollary — proved, CELL-07].

Scope [Boundary, §7.6]: one viewer, three hidden active seats, legal Straight play,
no hand-content bid eligibility, public actor attribution, rule support only. A
contract like plunge (whose legal bid reveals a hand predicate) is outside.

Receipts: 972 deterministic reachable prefixes across all nine declarations, plays
20–28, exact set equality between fiber and replayed deal set (970 with derived voids)
[Finite verification receipt — stated corpus, CELL-07A]; typed-update algebra exhausted
on all tiny (≤3-tile) cell systems: 14,412 lead + 56,460 follow + 56,460 slough cases
[Theorem — exhaustive finite verification, TRANS-07].

rob's `verify_support` reproduces the corpus shape in Rust — `r_cell_parity: 972
prefixes (970 with public voids)`; `r_cell_tiny_updates: 14,412 leads; 56,460 follows;
56,460 sloughs` — conformance evidence, never a status change
([verification](verification.md)). One caution on the 970: in rob it is
`FROZEN_WITH_VOIDS` (`rob/crates/verify/src/s3.rs`), a **rob-internal determinism
freeze of rob's own deterministic generator**, fixed at its first green run. It
coincides numerically with the ingest generator's 970 but is not an ingest number and
carries no cross-implementation meaning; only the 972 corpus shape does.

## Typed transitions

Hidden actor playing `d`: the removal map is a **bijection** from the legal
predecessor subset onto the successor fiber (inverse: add `d` back). Viewer action:
identity on hidden remainders. Hence within one attempt, play never increases fiber
cardinality (hidden: ≤, viewer: =) — but note the naive `Φ(c') ⊆ Φ(c)` is
*type-incorrect* for hidden acts since pool/capacities change [Corollaries — proved,
Math §7.14, TRANS-01..05]. Literal deal-set inclusion does hold on the fixed
complete-deal domain [TRANS-04].

**rec-only strengthening**: the support *normal form* is itself a closed transition
state given declaration + typed observation context — see
[support-dynamics](support-dynamics.md).

## Initial cells

After any straight auction + declaration: `U` = the 21 unseen tiles, every `P_s = U`,
every `k_s = 7` — straight bids/declarations remove no deal by rule (they can only
*reweight* under a policy model) [Theorem — proved, Math §4.4, §7.4, AUC-07/08].

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows (priority in parentheses);
"proved" = a declaration under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Row-to-declaration map: [lean-row-index](lean-row-index.md).
A kernel theorem never promotes a corpus status; rob receipts are conformance evidence,
never a status change. Layer C closed at priority 0 on 2026-07-29 (commit a904c163).

| Result on this page | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| The three domains (deal / remainder / fiber) and the cells `U`, `V_s`, `P_s`, `k_s` as **derived views** of the public record | PA-C01, PA-C02, PA-C04 (0) | **defined** (cells are functions of `PubState` + viewer hand, never fields — the D2 discipline realized by construction) | `Cells.lean` `PubState`, `ViewerCtx.pool`/`.allowed`/`.capacity`, `.IsWorld`, `.Compatible`, `.remainder` |
| CELL-01 dependent cells; Hall statement | PA-C03 (0, DEFINE) | definitions in place (`CellSys.Feasible`); the two-tile counterexample itself is not a named theorem; rob `r_cell_dependent` "2 of 4" conformance | `NormalForm.lean` |
| CELL-02A fiber factors through the cell system | — | `isWorld_iff_cellSys` ties the game cells to the generic capacitated kernel | `Reduction.lean:152` |
| **CELL-05 losslessness** `Φ(c) = ρ(Ω(I))` | PA-C05, PA-C07 (0) | **proved** — soundness from coherence, completeness by the four-case induction | `Cells.lean:1060` `losslessness`; `:736` `exists_deal_of_isWorld` |
| CELL-06 no positive clause; voids are an upper-bound-only update | PA-C06 (0) | **proved** | `Cells.lean:282` `voids_mono` and the `voidsAfter` lead/follow/slough lemmas |
| CELL-07 fixed-history bijection deals ↔ remainders | PA-C09 (0) | **proved** (remainder map injective on compatible deals, hence a bijection with the fiber) | `Cells.lean:1075` `remainder_injective` |
| CELL-07A parity corpus 972 / 970 | no row — an external finite receipt, kept visibly external | not a kernel target | — |
| CELL-08 scope boundary | boundary | n/a | — |
| TRANS-01..05 typed transitions: hidden removal is a bijection, viewer play is the identity | PA-C09, PA-C10 (0) | **proved** | `Cells.lean:1075`; `:717` `allowed_step_viewer`, `:105` `hands_step_ne`, `:113` `legalSet_congr` |
| TRANS-06/07 receipts (864 transitions; 14,412 / 56,460 / 56,460) | external receipts | not kernel targets; rob `r_cell_transitions`, `r_cell_tiny_updates` conformance | — |
| AUC-07/08 initial cells (`U` = 21 unseen tiles, `P_s = U`, `k_s = 7`) | no named theorem — the empty-prefix cell shape appears only as internal steps of the completeness induction; rob `r_cell_initial` conformance | — | — |
| The `21!/(7!)³ = 399,072,960` fiber bound | PA-B05 (1, prove half) | **open** as a cardinality of the deal type (the arithmetic identity is proved in `Trick1Foundation.lean`, which carries no ledger row and whose walt meaning is exploratory) | — |

**Mechanization finding (Layer C).** The completeness half of the losslessness
induction needs a fact the prose leaves implicit: a hidden seat's publicly played tile
must respect that seat's *previously recorded* voids. The Lean proof derives it from
true-trajectory void soundness rather than assuming it (`hd_allowed`, `Cells.lean:884`;
recorded in the `exists_deal_of_isWorld` docstring). The prose step "adding the tile
back reconstructs a legal slough" silently uses it. A weakened statement would have
been an overclaim; the strengthened proof is the receipt.

**Citation gap (bookkeeping).** PA-C05 is the one priority-0 row with no `PA-C05`
token in the Lean sources: `losslessness` cites PA-C07 in its docstring and
`lean/README.md` tags it PA-C05/C07. Reported, not resolved.
