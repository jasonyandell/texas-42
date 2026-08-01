---
number: 11
slug: constellation-lean-formalization
channel: new-chat
status: cleared by Jason 2026-08-01 (explicit go, up to 8 requests Aug 1)
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
deliverable: a single lake-buildable Lean 4 file (mathlib v4.33.0-rc1) formalizing suffix positions, minimax value, constellations, and the factorization conjecture C1 — with the k=1 base case and the hereditariness lemma proved sorry-free
---
You are performing adversarial formalization work on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files are the source of truth; where this message and the attachments differ, the attachments govern. Your response will be adjudicated mechanically: your Lean file will be dropped into a lake project pinned to leanprover/lean4:v4.33.0-rc1 with mathlib rev v4.33.0-rc1 and built. A file that does not compile scores zero regardless of mathematical merit. Undeclared `sorry`s score zero; declared ones score against you per the contract below.

## 1. Game mechanics to formalize (compressed but exact)

Pips P = {0..6}; dominoes D = the 28 multisets {i,j} over P, written h:l with h >= l; doubles p:p; sigma_p = { d : p in d }. Count labels c(d): 5 for 5:0, 4:1, 3:2; 10 for 6:4, 5:5; else 0. Four seats Z/4Z, opposite seats partners. Declarations delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)} — nine.

Called set kappa_delta = sigma_p (delta = p) | doubles (DT) | empty (NT). Effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta. Led context ell_delta(d) = 7 if d in kappa_delta else high(d). Follow F_delta(d,q) = 1 iff d in sigmahat_q. Leader plays any remaining tile, fixing q = ell_delta(d); a follower holding a follower tile must follow, else sloughs freely. Trick key of d in context q = (tier, rank), lexicographic: tier 2 iff d in kappa_delta (NT has no tier 2), tier 1 iff uncalled with F_delta(d,q)=1, else 0; rank = p for d = p:p under DT, TOP for doubles otherwise, pip sum for mixed. The strict maximum wins (you must prove uniqueness for distinct tiles), scores 1 + count labels of the four tiles, and leads the next trick.

**Suffix position** at depth k (1 <= k <= 7): X = (delta, L, H, ell) — live set L of 4k dominoes partitioned by H into four k-tile hands, leader ell. Play to exhaustion; **suffix value** v(X) = exact minimax margin (leader's team points minus opponents'), both teams optimal, perfect information, each trick worth 1 + counts.

**Constellation** Con(X): the structure on L with hold(d) (seat offset from leader), c(d), FOLLOW(d,e) = F_delta(d, ell_delta(e)), and ORD(d,e,g) = comparison of trick keys of d,e in context ell_delta(g) — and nothing else. X ~ X' iff some bijection phi: L -> L' preserves all four (declarations may differ).

CONJECTURE C1 (the target statement): X ~ X' implies v(X) = v(X').

## 2. THE TASK

Produce ONE self-contained Lean 4 file, `Constellation.lean`, importing mathlib only, that compiles under the pinned toolchain, containing:

1. **Definitions** (all computable where sensible, `decide`-friendly on the finite universe): `Pip`, `Domino` (as an ordered-pair quotient or the 28-element subtype — your choice, justify it), `Declaration`, `calledSet`, `ledContext`, `follows`, `TrickKey` with its lexicographic order, `countPoints`; `SuffixPos k` (hands as a partition of a 4k-element live Finset, leader in `Fin 4`); legal-move relation; trick resolution; `value : SuffixPos k -> Int` by well-founded recursion on remaining tiles (this is the delicate engineering — get the termination measure right); `Con` as a bundled relational structure and `ConEquiv X X'` as the existence of a structure-preserving bijection.
2. **Proved sorry-free (mandatory):**
   - `unique_winner` : distinct tiles have distinct trick keys in every context (so trick resolution is well-defined);
   - `value_k1` : at k=1 the value is the forced-trick margin, and `ConEquiv X X' -> value X = value X'` at k=1 — the base case of C1;
   - `hereditary` : the restriction lemma — for any trick played from X, the constellation of the successor position is determined by (Con X, the ~-data of the trick played): formally, if `ConEquiv X X'` via phi and corresponding tricks are played (phi-images, in order), the successor positions are `ConEquiv`. This is the crux lemma the whole induction stands on; its statement must be strong enough to close the induction step, and you must be careful that follower legality mid-trick is derivable from Con (state and prove the intermediate mid-trick invariant you need).
3. **Stated, allowed to be `sorry` (each as its own named theorem, nothing hidden inside):** `legal_transport` (bisimulation of move sets), `value_factors` (C1 in full). If you can close either sorry-free, do — full C1 sorry-free is the maximum score.
4. **Sanity `example`s via `decide`/`native_decide`-free evaluation** (adjudication machines may lack native_decide trust): at least three concrete k=1 positions with their values computed by `value` and checked against hand-computed margins stated in comments; one concrete ConEquiv pair across two different declarations with equal computed values.

Engineering constraints: no `axiom`s, no `unsafe`, no `partial` on anything load-bearing (`partial def value` scores as a sorry on termination), no `native_decide` in proofs, `set_option maxHeartbeats` raises allowed. Prefer small verified kernels over sweeping tactic automation that may not replay under the pin. If mathlib's API forces a choice (e.g. `Finset` vs `Multiset` hands), state the trade-off in a comment where you make it.

## 3. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. `FINAL ANSWER: FULL (C1 proved sorry-free)` or `FINAL ANSWER: SKELETON (mandatory lemmas sorry-free, N declared sorries)` with N exact and each sorry named with one line on what closing it needs.
2. The complete `Constellation.lean` in a single fenced code block — no elisions, no "..." — plus the exact `lakefile.toml` stanza you assume beyond `require mathlib`.
3. A build transcript claim: state the toolchain string and mathlib rev you targeted (leanprover/lean4:v4.33.0-rc1, mathlib v4.33.0-rc1) and, if you could not verify compilation yourself, say so explicitly and list the three most likely breakage points with their one-line fixes. Misrepresenting compilation status scores zero on the whole dispatch.
