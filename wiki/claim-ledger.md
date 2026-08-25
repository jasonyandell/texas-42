# Claim Ledger — Status Vocabulary and Merged Inventory

[Home](Home.md) · owns: the status vocabulary (including the exchange-adjudicated
tier definition) and the merged claim inventory · Sources: both packages
`docs/40_CLAIM_STATUS.md`. Related: [discrepancies D8](discrepancies.md),
[open-problems](open-problems.md).

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
below. One panel (REACH-20) was not unanimous: 2/3 SOUND + 1 UNVERIFIABLE that found
no defect — the verdict rule is "program green + no referee demonstrates a real flaw",
and the dissent is carried verbatim in that row's caveat, never silently presented as
3/3.

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

Eleven results from the Claude ↔ ChatGPT 5.6 Pro exchange
([exchange/README.md](../exchange/README.md) is the ledger of record), adjudicated
2026-07-27 and 2026-08-01. Status is **exchange-adjudicated CONFIRMED** as defined
above unless a row states otherwise; each row cites its inbox file and the
verified program under `exchange/adjudication/programs/`.

| Result | Where explained | Status / caveat |
|---|---|---|
| **REACH-17** — certified disjoint family of 17,668,066,045 reachable supports ⇒ [35,46] bits (inbox/001, programs/001.py) | [reachability](reachability.md) | exchange-adjudicated CONFIRMED (ALL_PASS 15.9s; 3/3 SOUND). **Verification-tier caveat**: reachability/disjointness of the counted family are prose trace-templates closed by referee adversarial replay, **not** end-to-end machine replay. Machine-hardened fallback tiers: ≥2³³ i.e. [34,46] without the four winning-void-trick rows; no-void family alone ≥30 bits. Now a **component of the REACH-18 combined floor** (both families are needed for ≥36 bits) — not superseded. |
| **REACH-18** — certified disjoint two-void-context family of 19,245,318,365 reachable supports; combined with REACH-17 ⇒ floor 36,913,384,410 > 2³⁵ ⇒ **[36,46] bits** (inbox/006, programs/006.py) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 17.3s; 3/3 SOUND). **Same verification-tier caveat as REACH-17**: 3,114 template representatives machine-replayed; within-class generalization and disjointness-from-001 close via prose argument + referee adversarial replay, not end-to-end machine replay of all ~19B members. Fallback tiers: the new family alone > 2³⁴ ⇒ ≥35 bits independent of 001; disjointness from the 001 no-void subfamily unconditional; margin over 2³⁵ is 2,553,646,042; no single sub-block reaches ≥36 without the full family. Exact census and full declaration classes explicitly still open. |
| **REACH-19** — filtered tagged outer census 33,297,009,347,414 ∈ (2⁴⁴, 2⁴⁵) ⇒ **ceiling 45 bits**, interval **[36,45]** (inbox/007, programs/007.py) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (17/17 PASS 44.1s; 3/3 SOUND). First ceiling movement since REACH-11. Filters: licensed (6,6,6)-singleton fifth condition + unconditional context-capacity supply bound (pure set arithmetic, Hall-independent) + temporal follower rule (complete finite enumeration of trick-prefix cases). Necessity stress-tested on ~986k machine-generated legal prefixes (116k in-program + 870k referee, fresh seeds), zero over-rejections. **Robust fallback (referee-proved)**: capacity-bound-only census 33,737,166,807,767 < 2⁴⁵ — the 45-bit ceiling survives discarding the temporal apparatus entirely. Caveats: temporal-rule necessity is finite-enumeration + smoke-tested, not end-to-end machine-proved; 7× pip multiplicity licensed by the transport theorem (pip-0 and DT recomputed equal in-program, not all seven). |
| **REACH-20** — no-void slice SATURATED: exact census **624,892,870** = Σ over the 50 range-≤1 capacity profiles of C(28,Σk) (inbox/008, programs/008.py, SHA 38fd84ea…) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (ALL_PASS 38/38, 71.8s; **panel 2/3 SOUND-high + 1 UNVERIFIABLE-medium that found "no computational error… nothing in the receipt is wrong" — dissent recorded here, not presented as 3/3**). Heaviest verification-tier caveat yet: coverage side machine-exact (per-phase meet-in-the-middle counts, covered+missing = C(28,\|T\|); all 5,430 exceptional pools realized and replayed end-to-end; 1,030 strided replays across all 50 profiles; fixed-hand j≤2 brute-force matches), but the stitching lemma (step 6) is machine-verified at module granularity (3,808 module-winner assignments + strong-triple checks), the j=1 block (64,422,540 pools) rests on the checked K₈-star pigeonhole + 63 strided samples, and the no-overcount direction rests on corpus-proved CELL-14 + Math §7.13.1. Proof-chain referee adversarially closed the stitching question and dissolved the §7.13.5 objection (REACH-10's witness is one-void, outside this slice). Corollaries: 001's 559,316,142 is a proper grammar-subfamily (undercount 65,576,728, reconciles exactly); derived combined floor 36,913,384,410 → **36,978,961,138** (disjointness automatic — added supports are no-void; labeled derived-from-REACH-20, not separately adjudicated); interval **[36,45]** unchanged. Full census over void contexts still open. |
| **Outer language NOT tight** (RESOLVED negative) — witness (NT,(6,6,6),V₁={6}) (inbox/002, programs/002.py; witnesses/002.json byte-identical to inbox) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 0.911s; 3/3 SOUND, three independent re-verifications: 1,276,560-trace single-layer enumeration; 301,860-state recursive DFS with max-flow feasibility; corpus ID/integer cross-check). New fifth necessary condition: follower-supply obstruction. |
| **OPEN-01 collapse** — reduced kernel strictly finer than the future-equivalence quotient (inbox/003, programs/003.py, witnesses/003.json) | [reduced-viewer-kernel](reduced-viewer-kernel.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED 2026-07-27 (ALL_PASS 0.43s; 3/3 SOUND). Product stats 204 / 22,848 / 1,604 / 1,280 reproduced and independently re-derived; dead-cut lemma mechanism. SHA provenance blemish non-load-bearing ([discrepancies](discrepancies.md)). |
| **Transport theorem** `f_{t,u}(R_t)=R_u` (inbox/004, programs/004.py) | [reachability](reachability.md) | exchange-adjudicated CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND). Headline bijection proved + family-certified; census collapses 9→3 classes. Step-15 quotient cardinality `7·r_pip + |R_DT| + |R_NT|` was CONDITIONAL on the cocycle lemma `f_{u,v}∘f_{t,u}=f_{t,v}`; **gap now closed** by finite check over all 343 ordered pip-trump triples (`programs/004-cocycle.py`, ALL_PASS — finite verification receipt, exchange-side). Artifact of record: inline `programs/004.py` (SHA-256 13420aa7…); external sandbox SHA c56c0c50… unverifiable/dead link. |
| **Census-integer audit** — all 19 load-bearing integers independently reproduced (inbox/005, programs/005.py) | [verification](verification.md), [minimal-support-normal-form](minimal-support-normal-form.md) | exchange-adjudicated CONFIRMED (19/19 PASS ~13s; 3/3 SOUND). CELL-27 and REACH-11/12 integers now carry an independent external reproduction via two computation routes per integer, with referee brute-force confirmation of the inherited assumptions (ternary validity over 16,712 structural cases, 0 mismatches; {1..7} lead-fiber multiset re-derived from the raw 28 dominoes for all 9 declarations). Newly established: Burnside decomposition 136,514 / 2,156 / 35 ⇒ 23,842. |

| **Constellation suffix factorization (C1)** — suffix minimax factors through the declaration-free constellation key, all depths, all 9 declarations pooled (inbox/009, programs/009.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | **PARTIAL verdict, split by sub-result** (program ALL_PASS 8/8 16.3s; panel 2/3 SOUND-high + 1 FLAWED-high — flaw confirmed in the response's corroboration artifacts, NOT the proof chain; dissent recorded, never presented as 3/3). (i) **C1 proof: adversarially step-checked, all steps survived all three referees** — external tier, not a kernel proof; Lean mechanization pending (dispatch 011). (ii) **Backward commutation for the pooled key: REFUTED — exchange-adjudicated CONFIRMED** (zeroes-trump/doubles-trump witness; predecessor trick 2:1, 2:2, 3:1, 3:0; exhaustive exclusion fixed_partial_maps=4, full_embeddings=0, legal_embeddings=0; two referee brute-force routes disjoint from the response backtracker, 5,953,536 and 372,096 enumerations, both 0). Embeddability/feasibility sense only — feasible ≠ reachable; no REACH-* impact. **Non-quotable from 009's program**: classes=19,329 (non-invariant selection artifact); multi_groups=9,495 / cross_declaration_groups=9,495 / pairs=5,000 as k≥2 evidence (undisclosed 2↔3 pip-transport monoculture; independent nontrivial k2 agreements = 5; zero DT/NT positions solved — k≥2 cross-declaration evidence is owned by rob's `constellation_k2_probe.rs`). Genuine k=1 anchors: 2,211,300 positions / 14 outcomes match corpus; corpus 15,680 reproduced exactly under opponent-swap pooling. ALG-12 recomputation covers the uniqueness half only; response SHA mismatch cosmetic (003 precedent). |
| **Carrier-skeleton staircase (rule-free)** — a₄=37, b₄=486, b₈=126,657, role-decorated 4-carriers 4,767; full rows a₀..a₂₈ (palindromic, Σ 79,264) and b₀..b₂₈ (non-palindromic, b₁=5 vs b₂₇=22, Σ 47,940,826) (inbox/012, programs/012.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | exchange-adjudicated CONFIRMED (14/14 PASS 18.95s; 3/3 SOUND-high; b₈ triply confirmed by referee routes independent of the response; corpus-frozen 486 / 4,767 match rob's instrument exactly). Caveats travel verbatim: b computed by fibered stabilizer-Burnside, a disclosed substitute for the displayed (computationally infeasible) conjugacy-class cycle-index formula, correctness proved by the response's own Step-4 theorem and independently reproved; ROLE_LOCAL_OK conjunct is a literal True; two PASS lines tautological; the response's own direct anchors are only j ∈ {0..5, 27, 28} — mid-layers closed by referee reruns; OEIS attribution unverified (403), though 79,264 is twice independently confirmed; the counts are the **rule-free carrier skeleton** (feasibility, not reachability) — a strictly poorer object than the standings-bearing carrier of the idea page, vocabulary split recorded there. |

| **Constellation realizability = reachability at k=1 (R1)** — every realizable last-trick class is legal-play reachable via a forward-replayed full-hand witness; the realizable-but-unreachable gap is zero, so reachability filtering of the k=1 retrograde seed table is a no-op at class granularity (inbox/010, programs/010.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | exchange-adjudicated CONFIRMED (31,830 PASS / 0 FAIL ~19s; 3/3 SOUND-high; all 31,197 witnesses independently re-replayed through the corpus ingest verifier by a referee, 0 failures; census re-derived twice with uncompressed keys, identical partition). Caveats verbatim: **convention** — 31,197 is the dispatch-literal ordered-opponent count, rob's frozen number is the swap-pooled 15,680 (163 reflection-fixed; 31,197 = 2·15,680 − 163); convert before diffing against `constellation_k1_census.rs`. **Scope** — REACHABLE = legal-play reachable (follow obligations + winner-leads; no contract/bid consistency); different predicate from the reachable-support image — **no REACH-\* impact**. **Evidence routing** — outcome-constancy is quotable only from the adjudication re-run (all 4,422,600 oriented positions, 0 collisions, 14 outcomes) or rob's `fine_collisions == 0`, never from the response's tautological per-class receipt. **Declaration skew** — 0 NT and 0 δ=3 witnesses in the exhaustive loop (NT realizes 19,069 of 31,197 classes); R1 is class-granularity only; per-declaration reachability rests on the 600-case sample covering all 216 declaration×hold cells. Step-6 FALSE-branch scope error vacuous (soundness rests on forward replay). Seed-table corollary travels with x:009's REFUTED pooled-key backward commutation. Retires x:009's 19,329 (non-invariant); proves rob's strictly finer key (extra slough bit) induces the same k=1 partition. |

## Walt-tier exchange intakes — adjudicated into the exploratory tier, never the CONFIRMED pipeline

The decision-sparse correspondence thread with Pro (dispatches 016–018, all
hand-ferried by Jason) was adjudicated same-day by walt-math **into walt's
exploratory tier**: real adjudication (repairs filed, claims confirmed or
rejected, rulings appended to `walt/CENSUS-RULINGS.md`), but no adversary
panel and no machine-checkable-deliverable contract — so nothing from it is,
or can become, exchange-adjudicated CONFIRMED. These rows exist so the tier
routing is never mistaken; the content is owned by
[walt-decision-sparse](walt-decision-sparse.md) and the walt hub's fence
applies (cited by nothing above the Ideas tier).

| Intake | Topic (one line) | Status |
|---|---|---|
| x:016 (`exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md`) | first-rung nonanticipativity taxes: fusion-gap identity, binary tax formula, one-stage penalty dual | **WALT-TIER ADJUDICATED** (walt-math, same day): first-layer mathematics confirmed; four repairs filed (FT-arrive, FT-trunc, FT-flat, FT-post); its Experiment 15.1 became the S6k fusion-tax probe |
| x:017 (`exchange/inbox/017-second-rung-gluing.md`) | second-rung gluing: slack–tax interchange law, multistage martingale dual | **WALT-TIER ADJUDICATED, ACCEPTED IN LARGE PART** (SR-A1..A36): interchange law and martingale dual confirmed; §12.1 verifier proved vacuous and REJECTED as a receipt; the SR depth-two probe ran same day |
| x:018 (`exchange/outbox/018-fee-correlation-update.md`) | fee-correlation update: collegial correspondence after the FC chapter close (no machine-checkable deliverable) | **SENT 2026-08-14, awaiting Pro's reply** — colleague register; whatever returns will be adjudicated before anything touches the wiki |
| x:019–023 (`exchange/outbox/019-…023-panel-*.md`) | the five adversary panels on the CE/L2 lineage: evidence process, bounded mean, risk ledger/escalation, execution order, L2 coupling theorems | **DISPATCHED 2026-08-24 (hand-ferried batch of five, quota cleared by Jason's delivery); consolidated response ADJUDICATED same day** (`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`, SHA-256 a3f468aa…; PANEL-A1..A8 in `walt/CENSUS-RULINGS.md`): four briefs certified (one with wording narrowed, one with the τ coupling repair), risk-ledger Claim D counterexampled as written (169/512 > 1/4 retrospective-opening adversary; repairs binding), O26 repaired via W7–W11, Part VI cancellation/dominance/directional mathematics adopted for slice 3; the committed 41/1200 lift corrected to **31/1200** (re-verified from raw probe records). Verifier 36/36 PASS = session evidence, scratch tier, never a receipt |
| x:024 (`exchange/outbox/024-deferred-producers-triple.md`) | the three-part design brief on the slice-3 deferred producers: δ-valid admissible-upper E3, dominance valid-bound route, §10 motif tags | **DISPATCHED 2026-08-24 (hand-ferried single dispatch, quota cleared by Jason's delivery); response ADJUDICATED 2026-08-25** (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`, SHA-256 337296a7…; TRIPLE-A1..A7 in `walt/CENSUS-RULINGS.md`; intake `walt/math/response_deferred_producers_triple_v0.1_intake.md`): all three answered at design level — max-preserving upper CS (same-δ coverage of a finite max via one fixed true maximizer, no Bonferroni; the shipped split-reach count verified to be S\*_n), Hazard-Exclusion Invariant as the single dominance-bound authority (sound + semantically complete) with the one-round trump-extraction producer and its standing non-coverage specimen, six-motif first-split morphology + Other with `RevealResponse` refused pending suffix enrichment; the dispatch's branch-mixture upper route retired (wrong orientation). **All three producers BUILT with gates the same night** (slices 4a/4b/4c, PRs #45/#46/#44, main `cbce1ae`, central gates green; card [[slice3-deferred-producers]] closed on its done-when; instrument records `walt/probes/fieldswap_motifs/` and `walt/probes/hazard_witness/`, exploratory tier). Verifier 13/13 PASS = session evidence, scratch tier, never a receipt |

## Informal captures — UNADJUDICATED, recorded here so they are never mistaken for results

Threads with ChatGPT that carried no deliverable contract and were never
adjudicated. They sit **below every tier** on
[Home](Home.md#evidentiary-tiers--never-promoted-never-blurred) — at or under
[ideas](ideas.md) — and are listed here only so that a reader who meets one of
their phrasings elsewhere can find its tier. No row below is a result, and
nothing in this section may be cited by anything above it.

| Capture | Topic (one line) | Status |
|---|---|---|
| x:014 (`exchange/inbox/014-constellation-informal-take.md`) | the constellation lens as intrinsic geometry; salience, promotion, extension-type proposals | **UNADJUDICATED** — informal exploratory capture only; numbered because it went through the courier channel, but carries no deliverable contract and no adjudication |
| 2026-08-03 informal capture (`exchange/informal/2026-08-03-domino-constellations-theory.md`, with reading memo `…-domino-constellations-theory.REVIEW.md`) | a thinking-out-loud thread on the constellation quotient and where the theory of 42 might live | **UNADJUDICATED** — informal capture outside the courier protocol: no `x:NNN` number, no dispatch count consumed, no adversarial panel, exploratory tier. The `.REVIEW.md` memo is a single-pass reading note and inherits this tier; where it says a number "reproduces", that is one reviewer re-running arithmetic once, never a receipt. Convention: [exchange/README.md](../exchange/README.md) |

## Reading discipline

When citing a claim from rec's ledger, beware its raw `|…|` pipes render broken in
Markdown tables ([discrepancies D13](discrepancies.md)); the mathematical content is
unaffected. When citing REACH-11/11A, use v0.7's "necessary outer profile" naming (D3).
