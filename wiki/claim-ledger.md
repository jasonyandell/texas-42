# Claim Ledger — Status Vocabulary and Merged Inventory

[Home](Home.md) · Sources: both packages `docs/40_CLAIM_STATUS.md`. Related:
[discrepancies D8](discrepancies.md), [open-problems](open-problems.md).

## Status vocabulary (shared by both packages)

DEFINITION · ADOPTED RULE · CLARIFICATION · **THEOREM — proved** ·
**LEMMA — proved** · **THEOREM — finite verification** (exact program exhausts the
stated finite domain) · **FINITE VERIFICATION RECEIPT** (checks a named corpus
*without* claiming exhaustion of the surrounding domain) · PROPOSITION (proved under
explicit extra assumptions) · COROLLARY — proved · COROLLARY / SYNTHESIS ·
COUNTEREXAMPLE · BOUNDARY · CONJECTURE · UNRESOLVED.

"Implemented, tested, fast, useful" are explicitly **not** mathematical statuses.
A finite program proves only the finite statement it exhausts (Math §0).

## Inventory shape

- **v0.7 ledger**: ~230 rows in 10 sections (scope/rules, algebra, deal/auction/
  contract, objective game, information, cells/support/reachability, belief, marked
  hand/strategic, utility/quotients, unresolved). Unique rows: TYPE-01..03 (proof
  irrelevance, derived views, normal-form well-formedness), TRUST-01 (external
  verification boundary).
- **rec ledger**: 255 unique claim IDs (count enforced by `audit_package.py`).
  Unique rows: ALG-20..24, PLAY-12..17, CELL-09A, REACH-14..16, TRANS-08..14, SYM-04,
  QUO-09..11, FAC-02, plus rewritten OPEN-01/OPEN-12 and REACH-03A/11/11A wording.
- **Merged inventory** = union, with v0.7 wording preferred on shared rows
  ([package-provenance](package-provenance.md) merge order).

## The load-bearing rows, by page

| Rows | Where explained |
|---|---|
| ALG-01..24 (universe, unique winner, transports, 3 mechanics classes) | [declaration-algebra](declaration-algebra.md) |
| CFG-03, AUC-05/05A (mark ceiling min(cap,5); auction census) | [rules-profile](rules-profile.md) |
| PLAY-01..17 (conservation, Markov state, fold, actor-from-capacities) | [rules-profile](rules-profile.md), [reduced-viewer-kernel](reduced-viewer-kernel.md) |
| INFO-01..12 (perfect recall; mechanical ≠ information state) | [support-fiber](support-fiber.md), [belief-vs-support](belief-vs-support.md) |
| CELL-01..29 (cells, losslessness, Hall, DP, reduction, normal form, census) | [support-fiber](support-fiber.md), [capacity-dp](capacity-dp.md), [minimal-support-normal-form](minimal-support-normal-form.md) |
| REACH-01..16 (50 profiles, schedule, witnesses, 26–46 bits, symbolic traces) | [reachability](reachability.md) |
| TRANS-01..14 (typed transitions; matching-minor dynamics; 63-edge budget) | [support-fiber](support-fiber.md), [support-dynamics](support-dynamics.md) |
| BEL-01..15 (Bayes, tilt, forced actions, off-path boundary) | [belief-vs-support](belief-vs-support.md) |
| HAND/STR rows incl. STR-06..09 (90-world flip) | [belief-vs-support](belief-vs-support.md), [strategic-state](strategic-state.md) |
| UTIL/TEAM/QUO/SYM/FAC rows | [strategic-state](strategic-state.md), [reduced-viewer-kernel](reduced-viewer-kernel.md) |
| TYPE-01..03, TRUST-01 (v0.7 only) | [proof-assistant-plan](proof-assistant-plan.md) |
| OPEN-01..12 | [open-problems](open-problems.md) |

## Reading discipline

When citing a claim from rec's ledger, beware its raw `|…|` pipes render broken in
Markdown tables ([discrepancies D13](discrepancies.md)); the mathematical content is
unaffected. When citing REACH-11/11A, use v0.7's "necessary outer profile" naming (D3).
