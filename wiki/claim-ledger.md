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

**Exchange-adjudicated CONFIRMED** (new tier, external): a result from the Claude ↔
ChatGPT 5.6 Pro exchange whose verification program executed `ALL_PASS` and whose chain
all three adversarial referees rated SOUND. It is **not** "Theorem — proved" in the
corpus sense and **not** a proof-assistant kernel proof; per TRUST-01 these receipts
stay visibly external. Where such a result RESOLVES an open question, the evidentiary
tier is kept next to the RESOLVED label. See the exchange-adjudicated results section
below.

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

## Exchange-adjudicated results (external evidentiary tier)

Six results from the Claude ↔ ChatGPT 5.6 Pro exchange
([exchange/README.md](../exchange/README.md)), adjudicated here 2026-07-27. Status is
**exchange-adjudicated CONFIRMED** as defined above; each row cites its inbox file and
the verified program under `exchange/adjudication/programs/`.

| Result | Where explained | Status / caveat |
|---|---|---|
| **REACH-17** — certified disjoint family of 17,668,066,045 reachable supports ⇒ [35,46] bits (inbox/001, programs/001.py) | [reachability](reachability.md) | exchange-adjudicated CONFIRMED (ALL_PASS 15.9s; 3/3 SOUND). **Verification-tier caveat**: reachability/disjointness of the counted family are prose trace-templates closed by referee adversarial replay, **not** end-to-end machine replay. Machine-hardened fallback tiers: ≥2³³ i.e. [34,46] without the four winning-void-trick rows; no-void family alone ≥30 bits. Now a **component of the REACH-18 combined floor** (both families are needed for ≥36 bits) — not superseded. |
| **REACH-18** — certified disjoint two-void-context family of 19,245,318,365 reachable supports; combined with REACH-17 ⇒ floor 36,913,384,410 > 2³⁵ ⇒ **[36,46] bits** (inbox/006, programs/006.py) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 17.3s; 3/3 SOUND). **Same verification-tier caveat as REACH-17**: 3,114 template representatives machine-replayed; within-class generalization and disjointness-from-001 close via prose argument + referee adversarial replay, not end-to-end machine replay of all ~19B members. Fallback tiers: the new family alone > 2³⁴ ⇒ ≥35 bits independent of 001; disjointness from the 001 no-void subfamily unconditional; margin over 2³⁵ is 2,553,646,042; no single sub-block reaches ≥36 without the full family. Exact census and full declaration classes explicitly still open. |
| **Outer language NOT tight** (RESOLVED negative) — witness (NT,(6,6,6),V₁={6}) (inbox/002, programs/002.py; witnesses/002.json byte-identical to inbox) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 0.911s; 3/3 SOUND, three independent re-verifications: 1,276,560-trace single-layer enumeration; 301,860-state recursive DFS with max-flow feasibility; corpus ID/integer cross-check). New fifth necessary condition: follower-supply obstruction. |
| **OPEN-01 collapse** — reduced kernel strictly finer than the future-equivalence quotient (inbox/003, programs/003.py, witnesses/003.json) | [reduced-viewer-kernel](reduced-viewer-kernel.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED 2026-07-27 (ALL_PASS 0.43s; 3/3 SOUND). Product stats 204 / 22,848 / 1,604 / 1,280 reproduced and independently re-derived; dead-cut lemma mechanism. SHA provenance blemish non-load-bearing ([discrepancies](discrepancies.md)). |
| **Transport theorem** `f_{t,u}(R_t)=R_u` (inbox/004, programs/004.py) | [reachability](reachability.md) | exchange-adjudicated CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND). Headline bijection proved + family-certified; census collapses 9→3 classes. Step-15 quotient cardinality `7·r_pip + |R_DT| + |R_NT|` was CONDITIONAL on the cocycle lemma `f_{u,v}∘f_{t,u}=f_{t,v}`; **gap now closed** by finite check over all 343 ordered pip-trump triples (`programs/004-cocycle.py`, ALL_PASS — finite verification receipt, exchange-side). Artifact of record: inline `programs/004.py` (SHA-256 13420aa7…); external sandbox SHA c56c0c50… unverifiable/dead link. |
| **Census-integer audit** — all 19 load-bearing integers independently reproduced (inbox/005, programs/005.py) | [verification](verification.md), [minimal-support-normal-form](minimal-support-normal-form.md) | exchange-adjudicated CONFIRMED (19/19 PASS ~13s; 3/3 SOUND). CELL-27 and REACH-11/12 integers now carry an independent external reproduction via two computation routes per integer, with referee brute-force confirmation of the inherited assumptions (ternary validity over 16,712 structural cases, 0 mismatches; {1..7} lead-fiber multiset re-derived from the raw 28 dominoes for all 9 declarations). Newly established: Burnside decomposition 136,514 / 2,156 / 35 ⇒ 23,842. |

## Reading discipline

When citing a claim from rec's ledger, beware its raw `|…|` pipes render broken in
Markdown tables ([discrepancies D13](discrepancies.md)); the mathematical content is
unaffected. When citing REACH-11/11A, use v0.7's "necessary outer profile" naming (D3).
