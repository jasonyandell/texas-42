# Claim Ledger — Status Vocabulary and Merged Inventory

[Home](Home.md) · owns: the status vocabulary (including the exchange-adjudicated
tier definition), the merged claim inventory, and the per-row-group pointer to
the kernel tier · Sources: both packages `docs/40_CLAIM_STATUS.md`; **v0.7**
`docs/65_MECHANIZATION_LEDGER.md` via [lean-row-index](lean-row-index.md);
[exchange/README.md](../exchange/README.md); `walt/CENSUS-RULINGS.md`. Related:
[discrepancies D8](discrepancies.md), [open-problems](open-problems.md),
[FINDINGS](FINDINGS.md), [verification](verification.md). Re-synced 2026-09-12
against the repository as of 2026-09-07 (c00717d1); nothing was promoted.

## How to read this page

Three readers use this ledger. A newcomer wants to know *what kind of thing* a
statement is before believing it: the vocabulary below is that typing, and the
tier ladder on [Home](Home.md) is its order. A mathematician wants the inventory
of proved rows and which page explains each group: the table under "The
load-bearing rows" is that map, with a column saying how much of each group the
Lean kernel has re-proved. An engineer wants to know which numbers came from
outside the corpus and under what contract: the exchange tables carry every
external result with its program, its runtime, its referee tally, and its caveat
verbatim. The rule that binds all three: **a status is never promoted here** — a
green receipt is evidence, a kernel proof is a kernel proof, and an exchange
result stays labelled external.

## Status vocabulary (shared by both packages)

DEFINITION · ADOPTED RULE · CLARIFICATION · **THEOREM — proved** ·
**LEMMA — proved** · **THEOREM — finite verification** (exact program exhausts the
stated finite domain) · **FINITE VERIFICATION RECEIPT** (checks a named corpus
*without* claiming exhaustion of the surrounding domain) · PROPOSITION (proved under
explicit extra assumptions) · COROLLARY — proved · COROLLARY / SYNTHESIS ·
COUNTEREXAMPLE · BOUNDARY · CONJECTURE · UNRESOLVED.

"Implemented, tested, fast, useful" are explicitly **not** mathematical statuses.
A finite program proves only the finite statement it exhausts (Math §0).

**Exchange-adjudicated CONFIRMED** (external tier): a result from the Claude ↔
ChatGPT 5.6 Pro exchange whose verification program executed `ALL_PASS` on its own
claims and whose proof chain survived three adversarial referee lenses
(proof-chain, program-vs-claim, corpus-consistency) with **no referee demonstrating
a real flaw** — that sentence is the verdict rule actually applied by
`exchange/adjudication/workflow.js`. It is **not** "Theorem — proved" in the
corpus sense and **not** a proof-assistant kernel proof; per TRUST-01 these receipts
stay visibly external. Where such a result RESOLVES an open question, the evidentiary
tier is kept next to the RESOLVED label. One panel (REACH-20) was not unanimous:
2/3 SOUND + 1 UNVERIFIABLE that found no defect — the dissent is carried verbatim in
that row's caveat and never presented as 3/3. Nine of the eleven result rows below
were 3/3 SOUND; the two exceptions are REACH-20 and the C1 row (x:009), which is
PARTIAL with a 2/3 + 1 FLAWED panel whose flaw lies in the response's corroboration
artifacts, not its proof chain.

**Proof-assistant kernel** (tier above exchange, below corpus): a statement checked
by the Lean 4 kernel over the standard axioms (`propext`, `Classical.choice`,
`Quot.sound`), no `sorry`, no `native_decide`, no external receipt imported. The
scoreboard is [lean-row-index](lean-row-index.md); this page only points at it.

**Rob conformance receipt** (tier below exchange): a byte-diffed receipt under
`rob/receipts/` reproducing a corpus or exchange number in an independent Rust
implementation. Evidence, never a status change ([verification](verification.md)).

**EXPLORATORY** (below every tier): everything under `walt/`, `experiments/`,
`wiki/ideas.md`, `wiki/analysis.md`. Nothing in this ledger cites it; the walt-tier
intake table at the bottom exists only so that the tier routing of Pro's walt-side
notes is never mistaken.

## Inventory shape

- **v0.7 ledger**: 232 rows in 10 sections (scope/rules, algebra, deal/auction/
  contract, objective game, information, cells/support/reachability, belief, marked
  hand/strategic, utility/quotients, unresolved). Unique rows: TYPE-01..03 (proof
  irrelevance, derived views, normal-form well-formedness), TRUST-01 (external
  verification boundary).
- **rec ledger**: 255 unique claim IDs (count enforced by `audit_package.py`).
  Unique rows: ALG-20..24, PLAY-12..17, CELL-09A, REACH-14..16, TRANS-08..14, SYM-04,
  QUO-09..11, FAC-02, plus rewritten OPEN-01/OPEN-12 and REACH-03A/11/11A wording.
- **Merged inventory** = union, with v0.7 wording preferred on shared rows
  ([package-provenance](package-provenance.md) merge order).
- **Mechanization ledger** (v0.7 `65_MECHANIZATION_LEDGER.md`): the proof-assistant
  work queue — deliberately smaller than the claim ledger, rows `PA-A01..PA-F07`
  with a route and a priority. All 42 priority-0 rows are kernel-proved as of
  2026-08-02 ([lean-row-index](lean-row-index.md)). Because it is v0.7's ledger, the
  rec-only rows (ALG-20..24, PLAY-12..17, REACH-14..16, TRANS-08..14, SYM-04,
  QUO-09..11, FAC-02) have **no** mechanization row at all — "no kernel row" below
  means exactly that, not a failed proof.

## The load-bearing rows, by page

The third column is the pointer to the kernel tier: which mechanization-ledger rows
discharge the group's obligations and which remain open. The join between claim
rows and `PA-` rows is [lean-row-index](lean-row-index.md)'s, maintained by hand
from the `PA-` citations in the Lean docstrings; treat any mismatch as a bug there.

| Rows | Where explained | Kernel tier (Lean), per [lean-row-index](lean-row-index.md) |
|---|---|---|
| ALG-01..24 (universe, unique winner, transports, 3 mechanics classes) | [declaration-algebra](declaration-algebra.md) | Layer A complete at priority 0: PA-A01..A11, A13, A14 proved (`Trick.lean`, `Basic.lean`); PA-A15..A17 (lead-threat witness, count-preserving maps are identity or `2↔3`, scoped `2↔3` transport) proved at priority 1 (`Transport.lean`). **Open**: PA-A12, the 737,100-case prose-resolver agreement (REFLECT, priority 2) — rob's `verify_algebra` receipt is the only mechanical check. rec's ALG-20..24 (49 unscored transports, 3 mechanics classes): no kernel row. |
| CFG-03, AUC-05/05A/06/06A, DEAL-01..03, MATCH-01/02, ORC-01..05 (mark ceiling min(cap,5); auction census and attempt finiteness; the deal counts; the match bound; backward-induction scope) | [rules-profile](rules-profile.md) | PA-B01/B02 (auction state, legality, deterministic transition) proved; PA-B03 (reachable mark ceiling) proved at priority 1 (`Auction.lean`); PA-B05 define half (`Deal.lean`). **Open**: PA-B04, the exact auction history counts for caps 1..7 (REFLECT, priority 3) — the census 2380…3214 rests on the verifiers and rob `verify_objective`; PA-B05 prove half — the deal cardinalities 472,518,347,558,400 and 399,072,960 as cardinalities of the deal type. ORC-01..05 and MATCH-01/02 have no mechanization row. |
| PLAY-01..17 (conservation, Markov state, fold, actor-from-capacities) | [rules-profile](rules-profile.md), [reduced-viewer-kernel](reduced-viewer-kernel.md) | PA-B06..B10 proved (`Play.lean`: contract, phase-indexed state, legal-play characterization, partition invariant, seven tricks / 28 plays / 42 points). **Open**: PA-B11..B14 (finite graded play graph, objective Markov congruence, deterministic settlement, early-settlement quotient; priority 1–2). rec's PLAY-12..17 (folded trick, reduced kernel): no kernel row — the reduced viewer kernel is not mechanized. |
| INFO-01..12 (perfect recall; mechanical ≠ information state) | [support-fiber](support-fiber.md), [belief-vs-support](belief-vs-support.md) | PA-C01 (perfect-recall information) and PA-F05 (physical congruence ≠ information-state equality, `Information.lean`) proved. |
| CELL-01..29 (cells, losslessness, Hall, DP, reduction, normal form, census) | [support-fiber](support-fiber.md), [capacity-dp](capacity-dp.md), [minimal-support-normal-form](minimal-support-normal-form.md) | Losslessness proved: PA-C02..C07, C09, C10 (`Cells.lean`; the induction needed a fact the prose left implicit — a hidden seat's played tile must respect its recorded voids); PA-C08 groundwork and PA-C15 reduction backbone proved (`NormalForm.lean`, `Reduction.lean`); normal form proved: PA-D01..D05 (trichotomy, well-formed NF, decode, compile/decode inverses, total NF classifies fibers). **Open**: PA-C11/C12 (capacity-DP recurrence, exact uniform sampler), PA-C13/C14, **PA-D06 — the global factorization half of CELL-14** (priority 1), PA-D07 (SCC compiler), PA-D08 (strict Hall / essential exclusions), PA-D17 (81-bit census; "REFLECT or keep external", priority 4). |
| REACH-01..16 (50 profiles, schedule, witnesses, 26–46 bits, symbolic traces) | [reachability](reachability.md) | PA-D09 (reachability predicate) and PA-D10 (reachability is proof-irrelevant evidence — the kernel counterpart of rob's INV-3 and of TYPE-01) proved (`Reachability.lean`). **Open**: PA-D11..D16 (reachable image and restricted minimality, the 50 profiles, seven lead contexts, lead-witness necessity, validator soundness, the feasible-but-unreachable witness; priority 1–2) and PA-D18 (the 26–46-bit interval; priority 4). rec's REACH-14..16 (symbolic replay): no kernel row. The exchange narrowing to [36,45] is external only. |
| TRANS-01..14 (typed transitions; matching-minor dynamics; 63-edge budget) | [support-fiber](support-fiber.md), [support-dynamics](support-dynamics.md) | The typed observation update and the hidden-play typed bijection are PA-C06/C09/C10 (proved). rec's TRANS-08..14 (support-NF dynamic sufficiency, matching-minor ≡ conditioning, monotone deletion, 63-edge budget): **no kernel row** — FINDINGS §6 names TRANS-08/09 the weakest spot (prose proof + ≤4-tile exhaustion). |
| BEL-01..15 (Bayes, tilt, forced actions, off-path boundary) | [belief-vs-support](belief-vs-support.md) | PA-E01..E03 proved (`Belief.lean`: finite prior and policy kernel, likelihood product and posterior, pushforward whose support lies inside the cell fiber). **Open**: PA-E04..E06 (physics-only posterior, exponential-tilt form, forced-action nondiscrimination; priority 1). |
| HAND/STR rows incl. STR-06..09 (90-world flip) | [belief-vs-support](belief-vs-support.md), [strategic-state](strategic-state.md) | PA-E07 (strategic-state sufficiency, `Strategic.lean`) and **PA-E10 — the 90-world witness internalized whole** (`Witness.lean`, `ninety_world_witness`, 2026-08-02) proved. **Open**: PA-E08/E09/E11 (priority 1), PA-E12 (priority 2); minimality of 90 is [open-problems](open-problems.md) item 3. |
| UTIL/TEAM/QUO/SYM/FAC rows | [strategic-state](strategic-state.md), [reduced-viewer-kernel](reduced-viewer-kernel.md) | PA-F05 proved. **Open**: PA-F01, F07 (priority 1), F02..F04, F06 (priority 2). rec's QUO-09..11, SYM-04, FAC-02: no kernel row. |
| TYPE-01..03, TRUST-01 (v0.7 only) | [proof-assistant-plan](proof-assistant-plan.md) | TYPE-01's kernel counterpart is PA-D10 (`CertifiedState.ext`). TRUST-01 is enforced as discipline, not as a theorem: every module is audited with `#print axioms` and no external receipt is imported ([lean](lean.md)). |
| OPEN-01..12 | [open-problems](open-problems.md) | No kernel rows. OPEN-01's COLLAPSE is exchange-tier (x:003), not a kernel proof. |

## Exchange-adjudicated results (external evidentiary tier)

Eleven result rows from the Claude ↔ ChatGPT 5.6 Pro exchange
([exchange/README.md](../exchange/README.md) is the ledger of record; the
mechanism and dispatch table are on [exchange](exchange.md)), adjudicated
2026-07-27 (rows 1–8, the foundation batch, workflow `wf_775fe0ec`) and 2026-08-01
(rows 9–11, the constellation batch). A twelfth row records the Lean thread
(x:011/013/015), which is a process-and-kernel record, not a CONFIRMED result.
Status is **exchange-adjudicated CONFIRMED** as defined above unless a row states
otherwise; each row cites its inbox file and the verified program under
`exchange/adjudication/programs/`. Every program is Pro's, saved verbatim, except
`004-cocycle.py`, which is **in-house (Claude-authored)**. Program line counts and
the 2026-09-13 re-runs (all green, same PASS counts as recorded; 008 not re-run) are
tabulated on [verification](verification.md).

| Result | Where explained | Status / caveat |
|---|---|---|
| **REACH-17** — certified disjoint family of 17,668,066,045 reachable supports ⇒ [35,46] bits (inbox/001, programs/001.py) | [reachability](reachability.md) | exchange-adjudicated CONFIRMED (ALL_PASS 15.9s; 3/3 SOUND). **Verification-tier caveat**: reachability/disjointness of the counted family are prose trace-templates closed by referee adversarial replay, **not** end-to-end machine replay. Machine-hardened fallback tiers: ≥2³³ i.e. [34,46] without the four winning-void-trick rows; no-void family alone ≥30 bits. Now a **component of the REACH-18 combined floor** (both families are needed for ≥36 bits) — not superseded. rob backs the family totals with `verify_floor` `x-` lines (S10, [verification](verification.md)). |
| **REACH-18** — certified disjoint two-void-context family of 19,245,318,365 reachable supports; combined with REACH-17 ⇒ floor 36,913,384,410 > 2³⁵ ⇒ **[36,46] bits** (inbox/006, programs/006.py; dispatched 2026-07-27T14:53:19Z) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 17.3s; 3/3 SOUND). **Same verification-tier caveat as REACH-17**: 3,114 template representatives machine-replayed; within-class generalization and disjointness-from-001 close via prose argument + referee adversarial replay, not end-to-end machine replay of all ~19B members. Fallback tiers: the new family alone > 2³⁴ ⇒ ≥35 bits independent of 001; disjointness from the 001 no-void subfamily unconditional; margin over 2³⁵ is 2,553,646,042; no single sub-block reaches ≥36 without the full family. Exact census and full declaration classes explicitly still open. Not reproduced by rob. |
| **REACH-19** — filtered tagged outer census 33,297,009,347,414 ∈ (2⁴⁴, 2⁴⁵) ⇒ **ceiling 45 bits**, interval **[36,45]** (inbox/007, programs/007.py; dispatched 2026-07-27T18:15:07Z) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (17/17 PASS 44.1s; 3/3 SOUND). First ceiling movement since REACH-11. Filters: licensed (6,6,6)-singleton fifth condition + unconditional context-capacity supply bound (pure set arithmetic, Hall-independent) + temporal follower rule (complete finite enumeration of trick-prefix cases). Necessity stress-tested on ~986k machine-generated legal prefixes (116k in-program + 870k referee, fresh seeds), zero over-rejections. **Robust fallback (referee-proved)**: capacity-bound-only census 33,737,166,807,767 < 2⁴⁵ — the 45-bit ceiling survives discarding the temporal apparatus entirely. Caveats: temporal-rule necessity is finite-enumeration + smoke-tested, not end-to-end machine-proved; 7× pip multiplicity licensed by the transport theorem (pip-0 and DT recomputed equal in-program, not all seven). **Rests solely on the exchange program** — not reproduced by rob; slice 03 is the named target. |
| **REACH-20** — no-void slice SATURATED: exact census **624,892,870** = Σ over the 50 range-≤1 capacity profiles of C(28,Σk) (inbox/008, programs/008.py, SHA 38fd84ea…) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (ALL_PASS 38/38, 71.8s; **panel 2/3 SOUND-high + 1 UNVERIFIABLE-medium that found "no computational error… nothing in the receipt is wrong" — dissent recorded here, not presented as 3/3**). Heaviest verification-tier caveat yet: coverage side machine-exact (per-phase meet-in-the-middle counts, covered+missing = C(28,\|T\|); all 5,430 exceptional pools realized and replayed end-to-end; 1,030 strided replays across all 50 profiles; fixed-hand j≤2 brute-force matches), but the stitching lemma (step 6) is machine-verified at module granularity (3,808 module-winner assignments + strong-triple checks), the j=1 block (64,422,540 pools) rests on the checked K₈-star pigeonhole + 63 strided samples, and the no-overcount direction rests on corpus-proved CELL-14 + Math §7.13.1. Proof-chain referee adversarially closed the stitching question and dissolved the §7.13.5 objection (REACH-10's witness is one-void, outside this slice). Corollaries: 001's 559,316,142 is a proper grammar-subfamily (undercount 65,576,728, reconciles exactly; [discrepancies D17](discrepancies.md)); derived combined floor 36,913,384,410 → **36,978,961,138** (disjointness automatic — added supports are no-void; labeled derived-from-REACH-20, not separately adjudicated); interval **[36,45]** unchanged. Full census over void contexts still open. **Rests solely on the exchange program** — not reproduced by rob; slice 03 is the named target. |
| **Outer language NOT tight** (RESOLVED negative) — witness (NT,(6,6,6),V₁={6}) (inbox/002, programs/002.py; witnesses/002.json byte-identical to inbox) | [reachability](reachability.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED (16/16 PASS 0.911s; 3/3 SOUND, three independent re-verifications: 1,276,560-trace single-layer enumeration; 301,860-state recursive DFS with max-flow feasibility; corpus ID/integer cross-check). New fifth necessary condition: follower-supply obstruction. rob's `verify_unreachable` `x-r_unr_002_*` lines reproduce the 425,520-trace / 0-realizer exhaustion and the supply check. |
| **OPEN-01 collapse** — reduced kernel strictly finer than the future-equivalence quotient (inbox/003, programs/003.py, witnesses/003.json) | [reduced-viewer-kernel](reduced-viewer-kernel.md), [open-problems](open-problems.md) | exchange-adjudicated CONFIRMED 2026-07-27 (ALL_PASS 0.43s; 3/3 SOUND). Product stats 204 / 22,848 / 1,604 / 1,280 reproduced and independently re-derived; dead-cut lemma mechanism. SHA provenance blemish non-load-bearing ([discrepancies D16](discrepancies.md)). The witness is rob slice 03's required regression ([rob-slices](rob-slices.md)). |
| **Transport theorem** `f_{t,u}(R_t)=R_u` (inbox/004, programs/004.py) | [reachability](reachability.md), [declaration-algebra](declaration-algebra.md) | exchange-adjudicated CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND). Headline bijection proved + family-certified; census collapses 9→3 classes. Step-15 quotient cardinality `7·r_pip + |R_DT| + |R_NT|` was CONDITIONAL on the cocycle lemma `f_{u,v}∘f_{t,u}=f_{t,v}`; **gap closed in-house** by `programs/004-cocycle.py` — a Claude-authored finite check (146 lines, stdlib-only, transport definition copied verbatim from 004.py rather than imported) over all 343 ordered pip-trump triples, ALL_PASS; finite verification receipt on the exchange side, **not** a Pro deliverable and not referee-panelled. Artifact of record: inline `programs/004.py` (SHA-256 13420aa7…); external sandbox SHA c56c0c50… unverifiable/dead link. rob's `verify_transport` `x-r_tra_corpus_commutation` (588 hands, 16,464 transitions, 17,052 NF equalities) is conformance evidence. |
| **Census-integer audit** — all 19 load-bearing integers independently reproduced (inbox/005, programs/005.py) | [verification](verification.md), [minimal-support-normal-form](minimal-support-normal-form.md) | exchange-adjudicated CONFIRMED (19/19 PASS ~13s; 3/3 SOUND). CELL-27 and REACH-11/12 integers now carry an independent external reproduction via two computation routes per integer, with referee brute-force confirmation of the inherited assumptions (ternary validity over 16,712 structural cases, 0 mismatches; {1..7} lead-fiber multiset re-derived from the raw 28 dominoes for all 9 declarations). Newly established: Burnside decomposition 136,514 / 2,156 / 35 ⇒ 23,842 (rob `verify_outer` `x-r_out_burnside` reproduces it). |
| **Constellation suffix factorization (C1)** — suffix minimax factors through the declaration-free constellation key, all depths, all 9 declarations pooled (inbox/009, programs/009.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | **PARTIAL verdict, split by sub-result** (program ALL_PASS 8/8 16.3s; panel 2/3 SOUND-high + 1 FLAWED-high — flaw confirmed in the response's corroboration artifacts, NOT the proof chain; dissent recorded, never presented as 3/3). (i) **C1 proof: adversarially step-checked, all steps survived all three referees** — external tier, not a kernel proof. **Lean status: C1 is not mechanized.** The follow-on Lean thread (x:011/013/015, next row) produced a self-contained core and an exact suffix minimax, but neither file states C1 or any factorization theorem (grep of `ConstellationCore.lean`/`ConstellationSuffix.lean` at c00717d1 finds no C1, no factorization). (ii) **Backward commutation for the pooled key: REFUTED — exchange-adjudicated CONFIRMED** (zeroes-trump/doubles-trump witness; predecessor trick 2:1, 2:2, 3:1, 3:0; exhaustive exclusion fixed_partial_maps=4, full_embeddings=0, legal_embeddings=0; two referee brute-force routes disjoint from the response backtracker, 5,953,536 and 372,096 enumerations, both 0). Embeddability/feasibility sense only — feasible ≠ reachable; no REACH-* impact. **Non-quotable from 009's program**: classes=19,329 (non-invariant selection artifact); multi_groups=9,495 / cross_declaration_groups=9,495 / pairs=5,000 as k≥2 evidence (undisclosed 2↔3 pip-transport monoculture; independent nontrivial k2 agreements = 5; zero DT/NT positions solved — k≥2 cross-declaration evidence is owned by rob's `constellation_k2_probe.rs`). Genuine k=1 anchors: 2,211,300 positions / 14 outcomes match corpus; corpus 15,680 reproduced exactly under opponent-swap pooling. ALG-12 recomputation covers the uniqueness half only; response SHA mismatch cosmetic (003 precedent). |
| **Carrier-skeleton staircase (rule-free)** — a₄=37, b₄=486, b₈=126,657, role-decorated 4-carriers 4,767; full rows a₀..a₂₈ (palindromic, Σ 79,264) and b₀..b₂₈ (non-palindromic, b₁=5 vs b₂₇=22, Σ 47,940,826) (inbox/012, programs/012.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | exchange-adjudicated CONFIRMED (14/14 PASS 18.95s; 3/3 SOUND-high; b₈ triply confirmed by referee routes independent of the response; corpus-frozen 486 / 4,767 match rob's instrument exactly). Caveats travel verbatim: b computed by fibered stabilizer-Burnside, a disclosed substitute for the displayed (computationally infeasible) conjugacy-class cycle-index formula, correctness proved by the response's own Step-4 theorem and independently reproved; ROLE_LOCAL_OK conjunct is a literal True; two PASS lines tautological; the response's own direct anchors are only j ∈ {0..5, 27, 28} — mid-layers closed by referee reruns; OEIS attribution unverified (403), though 79,264 is twice independently confirmed; the counts are the **rule-free carrier skeleton** (feasibility, not reachability) — a strictly poorer object than the standings-bearing carrier of the idea page, vocabulary split recorded there. |
| **Constellation realizability = reachability at k=1 (R1)** — every realizable last-trick class is legal-play reachable via a forward-replayed full-hand witness; the realizable-but-unreachable gap is zero, so reachability filtering of the k=1 retrograde seed table is a no-op at class granularity (inbox/010, programs/010.py) | [idea-retrograde-rank](idea-retrograde-rank.md) §§5,7 | exchange-adjudicated CONFIRMED (31,830 PASS / 0 FAIL ~19s; 3/3 SOUND-high; all 31,197 witnesses independently re-replayed through the corpus ingest verifier by a referee, 0 failures; census re-derived twice with uncompressed keys, identical partition). Caveats verbatim: **convention** — 31,197 is the dispatch-literal ordered-opponent count, rob's frozen number is the swap-pooled 15,680 (163 reflection-fixed; 31,197 = 2·15,680 − 163); convert before diffing against `constellation_k1_census.rs`. **Scope** — REACHABLE = legal-play reachable (follow obligations + winner-leads; no contract/bid consistency); different predicate from the reachable-support image — **no REACH-\* impact**. **Evidence routing** — outcome-constancy is quotable only from the adjudication re-run (all 4,422,600 oriented positions, 0 collisions, 14 outcomes) or rob's `fine_collisions == 0`, never from the response's tautological per-class receipt. **Declaration skew** — 0 NT and 0 δ=3 witnesses in the exhaustive loop (NT realizes 19,069 of 31,197 classes); R1 is class-granularity only; per-declaration reachability rests on the 600-case sample covering all 216 declaration×hold cells. Step-6 FALSE-branch scope error vacuous (soundness rests on forward replay). Seed-table corollary travels with x:009's REFUTED pooled-key backward commutation. Retires x:009's 19,329 (non-invariant); proves rob's strictly finer key (extra slough bit) induces the same k=1 partition. |
| **The Lean thread (x:011 / x:013 / x:015)** — a staged Lean 4 build of the constellation objects, conducted in one conversation on 2026-08-01 (inbox/011, inbox/013, inbox/015; no adjudication programs — the deliverable was a `lake build`) | [lean](lean.md) ("The constellation files are not reconciled"), [lean-row-index](lean-row-index.md) | **Tier: proof-assistant kernel for exactly the statements the two files make; no exchange verdict, no ledger row, C1 not mechanized.** x:011 (~6.8 h latency) was an **honest refusal**: Pro would not fabricate a compilation claim, proposed the staged build, and **caught a dispatch spec error** — tier-zero trick keys tie by design, so the mandatory theorem is the unique *winner*, not key injectivity; correction accepted. No panel was convened (iteration policy). x:013 Stage 1 GREEN: `lake build` clean under the pin after two mechanical local fixes; now `lean/Texas42/ConstellationCore.lean` (namespace `Constellation`; 272 lines at c00717d1, delivered as 278) — own domino/declaration algebra, packed trick key `Fin 42`, `winner_maximal`, `positive_key_injective` (by `decide` over every declaration/context/tile pair, where the main spine's `Trick.lean` proves injectivity by the shared-pip argument), `unique_winner`, two worked tricks (commit 1d36ddb8). x:015 Stage 2 GREEN after local repair (reserved-keyword rename `prefix`→`pending`, `step_remaining` proof restructured, `LinearOrder Domino` lift, `SuffixPos` namespace — all fixes ours): now `lean/Texas42/ConstellationSuffix.lean` (274 lines) — `SuffixPos k`, `MidState`, `legalMoves_subset_hand`, `step_remaining`, fuel-indexed exact `minimax` over 4k plays, `value_k1_forced`, two k=1 values (−11, 16) evaluated by the kernel (commit f0de9333). Both files: zero `sorry`, standard axioms only, imported by `lean/Texas42.lean` so the library's sorry-free and axiom-hygiene claims cover them. **Caveats**: self-contained — they re-derive their own core rather than importing `Basic.lean`/`Trick.lean` and are **unreconciled** with them; they carry **no mechanization-ledger row** and are **not on the priority-0 scoreboard**; the theorem the thread was opened for — **C1, the suffix factorization — is stated nowhere in Lean**. Whether C1 will be stated is an open item on [open-problems](open-problems.md). |

## Walt-tier exchange intakes — adjudicated into the exploratory tier, never the CONFIRMED pipeline

The walt-side correspondence with Pro (dispatches 016–024, all hand-ferried by
Jason) was adjudicated same-day by walt-math **into walt's exploratory tier**:
real adjudication (repairs filed, claims confirmed or rejected, rulings appended
to `walt/CENSUS-RULINGS.md`), but no adversary panel and no
machine-checkable-deliverable contract — so nothing from it is, or can become,
exchange-adjudicated CONFIRMED. These rows exist so the tier routing is never
mistaken; the content is owned by [walt-decision-sparse](walt-decision-sparse.md)
and [walt-calculated-evidence](walt-calculated-evidence.md), and the walt hub's
fence applies (cited by nothing above the Ideas tier).

| Intake | Topic (one line) | Status |
|---|---|---|
| x:016 (`exchange/inbox/016-decision-sparse-nonanticipativity-taxes.md`) | first-rung nonanticipativity taxes: fusion-gap identity, binary tax formula, one-stage penalty dual | **WALT-TIER ADJUDICATED** (walt-math, same day, 2026-08-14; FT-A1..A29): first-layer mathematics confirmed; four repairs filed (FT-arrive, FT-trunc, FT-flat, FT-post); its Experiment 15.1 became the S6k fusion-tax probe |
| x:017 (`exchange/inbox/017-second-rung-gluing.md`) | second-rung gluing: slack–tax interchange law, multistage martingale dual | **WALT-TIER ADJUDICATED, ACCEPTED IN LARGE PART** (2026-08-14; **SR-A1..A37** — `walt/CENSUS-RULINGS.md` is authoritative and contains SR-A37; `exchange/README.md` row 017 still says A36 and is stale): interchange law and martingale dual confirmed; §12.1 verifier proved vacuous and REJECTED as a receipt; the SR depth-two probe ran same day |
| x:018 (`exchange/outbox/018-fee-correlation-update.md`) | fee-correlation update: collegial correspondence after the FC chapter close (no machine-checkable deliverable) | **STAGED 2026-08-14 for hand-ferry; no inbox file and no reply as of 2026-09-07** — colleague register; the record does not confirm delivery (the tally shows two increments for the three hand-ferried dispatches 016–018); whatever returns will be adjudicated before anything touches the wiki. The ask is carried as [walt-math-open-questions](walt-math-open-questions.md) item 11 |
| x:019–023 (`exchange/outbox/019-…023-panel-*.md`) | the five adversary panels on the CE/L2 lineage: evidence process, bounded mean, risk ledger/escalation, execution order, L2 coupling theorems | **DISPATCHED 2026-08-24 (hand-ferried batch of five, quota cleared by Jason's delivery); consolidated response ADJUDICATED same day** (`exchange/inbox/019-023-response-panel-and-cancellation-v0.1.md`, SHA-256 a3f468aa…; PANEL-A1..A8 in `walt/CENSUS-RULINGS.md`): four briefs certified (one with wording narrowed, one with the τ coupling repair), risk-ledger Claim D counterexampled as written (169/512 > 1/4 retrospective-opening adversary; repairs binding), O26 repaired via W7–W11, Part VI cancellation/dominance/directional mathematics adopted for slice 3; the committed 41/1200 lift corrected to **31/1200** (re-verified from raw probe records). Verifier 36/36 PASS = session evidence, scratch tier, never a receipt |
| x:024 (`exchange/outbox/024-deferred-producers-triple.md`) | the three-part design brief on the slice-3 deferred producers: δ-valid admissible-upper E3, dominance valid-bound route, §10 motif tags | **DISPATCHED 2026-08-24 (hand-ferried single dispatch, quota cleared by Jason's delivery); response ADJUDICATED 2026-08-25** (`exchange/inbox/024-response-deferred-producers-triple-v0.1.md`, SHA-256 337296a7…; TRIPLE-A1..A7 in `walt/CENSUS-RULINGS.md`; intake `walt/math/response_deferred_producers_triple_v0.1_intake.md`): all three answered at design level — max-preserving upper CS (same-δ coverage of a finite max via one fixed true maximizer, no Bonferroni; the shipped split-reach count verified to be S\*_n), Hazard-Exclusion Invariant as the single dominance-bound authority (sound + semantically complete) with the one-round trump-extraction producer and its standing non-coverage specimen, six-motif first-split morphology + Other with `RevealResponse` refused pending suffix enrichment; the dispatch's branch-mixture upper route retired (wrong orientation). **All three producers BUILT with gates the same night** (slices 4a/4b/4c, PRs #45/#46/#44, main `cbce1ae`, central gates green; card [[slice3-deferred-producers]] closed on its done-when; instrument records `walt/probes/fieldswap_motifs/` and `walt/probes/hazard_witness/`, exploratory tier). Verifier 13/13 PASS = session evidence, scratch tier, never a receipt |

**The numbered exchange is the smaller part of Pro's walt-side contribution.**
Seven further Pro parents entered by side channel — hand-delivered by Jason,
never numbered, the courier ledger untouched — and were adjudicated same-day by
walt-math under their own ruling families: CE (CE-A1..A8, 2026-08-24), L2
(L2-A1..A7, 2026-08-24), CBS (CBS-A1..A9, 2026-08-30), APS (APS-A1..A9,
2026-08-31), MB (MB-A1..A8, 2026-09-01), SC (SC-A1..A8, 2026-09-01) and FH
(FH-A1..A11, 2026-09-04). They are indexed **only** on
[walt-math-intakes](walt-math-intakes.md) (lineage table), not here and not in
`exchange/README.md`; all are EXPLORATORY. The 2026-09-04 letter
`walt/briefs/FH-RESPONSE-TO-PRO.md` is the same kind of object, with no recorded
reply.

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
When citing an exchange result, cite `x:NNN`, carry its caveat block, keep the corpus
endpoints [26,46] and the exchange endpoints [36,45] distinct, and never present
REACH-20's panel as 3/3. When citing a kernel fact, cite the Lean declaration through
[lean-row-index](lean-row-index.md), never a `PASS` line.
