---
number: 10
slug: constellation-realizability-reachability
channel: new-chat
status: cleared by Jason 2026-08-01 (explicit go, up to 8 requests Aug 1)
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_foundation.py
deliverable: exact k=1 realizable-constellation census plus the reachable/realizable gap — proof that every realizable last-trick constellation is reachable from a legal 28-tile game, or explicit unreachable classes with impossibility proofs
---
You are performing adversarial mathematical research on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files (rules profile, mathematical foundation, one verification script) are the source of truth; you have no access to anything else from that project and must not rely on outside sources. This message restates every definition the task needs; where they differ, the attachments govern. Your response will be adjudicated mechanically by another model holding the full corpus: programs executed, witnesses re-run, proofs step-checked. Hedged or unverifiable claims score zero.

## 1. Game mechanics (compressed but exact)

Pips P = {0..6}; dominoes D = the 28 multisets {i,j} over P, written h:l with h >= l; doubles p:p; sigma_p = { d : p in d }. Count labels c(d): 5 for 5:0, 4:1, 3:2; 10 for 6:4, 5:5; else 0. Four seats Z/4Z clockwise, opposite seats partners; uniform deal of 7 tiles each, no boneyard. One-round auction (legality independent of hand content); the auction winner declares delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)} and leads trick 1.

Called set kappa_delta = sigma_p (delta = p) | the 7 doubles (DT) | empty (NT). Effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta. Led context ell_delta(d) = 7 if d in kappa_delta else high(d). Follow F_delta(d,q) = 1 iff d in sigmahat_q. The leader of a trick plays any remaining tile, fixing q = ell_delta(d); a follower holding a tile with F_delta(d,q) = 1 must play such a tile, else sloughs freely. Trick key of d in context q = (tier, rank), lexicographic: tier 2 iff d in kappa_delta (NT has no tier 2), tier 1 iff uncalled with F_delta(d,q) = 1, else 0; rank = p for d = p:p under DT, TOP for doubles otherwise, pip sum for mixed. The unique maximum wins (uniqueness proved in the corpus), scores 1 + count labels, and leads the next trick. Seven tricks; all 28 tiles are played.

## 2. Last-trick constellations

A **last-trick position** is X = (delta, L, H, ell): declaration delta, a live set L of 4 dominoes, an assignment H of one tile to each seat, leader ell. All four plays are forced; the outcome (winning team relative to the leader, award 1 + counts) is determined.

The **constellation** Con(X) is the relational structure on L, with NOTHING else — no pip names, no declaration name:

- hold(d) in {0,1,2,3}: holder's seat offset from the leader;
- c(d) in {0,5,10};
- FOLLOW(d,e) in {0,1}: F_delta(d, ell_delta(e));
- ORD(d,e,g) in {<,=,>}: comparison of trick keys of d and e in context ell_delta(g).

X ~ X' iff a bijection phi preserves all four components (declarations may differ). The corpus has verified exhaustively that the forced outcome is constant on every ~-class across all nine declarations pooled, and holds frozen counts of the ~-classes (withheld deliberately; your program computes its own and adjudication diffs them).

Two nested existence questions about a ~-class C:

- **REALIZABLE(C)**: some last-trick position X with Con(X) in C exists over the 28-tile universe (an "embedding": choose delta, four distinct dominoes, holders, leader inducing exactly C's relations).
- **REACHABLE(C)**: some full legal Straight-42 hand — a deal of 28 tiles into four hands of 7, a declaration, and six complete legal tricks (follow obligations respected, trick winners leading) — ends with a last trick whose position realizes C. Reachability adds the entire play history as a constraint: the four last tiles are what each seat retained through six tricks of forced-follow discipline, and the last-trick leader must be the trick-6 winner.

Trivially REACHABLE implies REALIZABLE. The corpus's standing hunch (on record, unproven): when abstraction and reachability fight, the failures are reachability-shaped. You are hired to settle the k=1 case of exactly that.

## 3. THE TASK

1. **Exact realizable census.** Compute the exact number of ~-classes of last-trick positions over the 28-tile universe (all four-tile sets, all holder/leader arrangements up to the two-opponent symmetry, all nine declarations, quotiented by ~). State the count and the class-size distribution. This must come from your own from-scratch implementation; the corpus number exists and will be diffed.
2. **The reachable/realizable gap — the heart of the dispatch.** Prove or refute:

   CONJECTURE R1. Every realizable last-trick ~-class is reachable: for every class C with REALIZABLE(C), some legal full hand ends in C.

   For TRUE: a constructive proof — an explicit algorithm that, given any realizable last-trick position (or class representative), produces a complete legal hand (deal + declaration + six tricks, every follow obligation checked) ending in that class, with a correctness argument covering every case (voids created en route, forced-follow collisions, the trick-6-winner-leads constraint, doubles and called tiles retained to the end). Watch the hard cases: all four last tiles called (trump-rich endings); a seat retaining two called tiles at trick 6 forcing follow patterns earlier; NT hands where doubles never gain tier; the leader constraint interacting with partnership.
   For FALSE: an explicit realizable class C, a proof that REALIZABLE(C) holds (give the position), and a complete impossibility proof that no legal hand reaches it — a finite exhaustion or invariant argument (e.g. a parity/counting invariant of forced follows), not a failed search.
3. **If R1 is FALSE, quantify the gap**: the exact number of realizable-but-unreachable classes, each with its impossibility certificate, or a proven partition method. If R1 is TRUE, state the corollary precisely: the retrograde seed table equals the realizable census, and reachability filtering at k=1 is a no-op.

## 4. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. `FINAL ANSWER: R1 TRUE` or `FINAL ANSWER: R1 FALSE (N unreachable classes)` with N exact, or `FINAL ANSWER: PARTIAL (<scope>)`.
2. Proof as numbered steps labeled `[USES: ...]`; the constructive algorithm (or impossibility invariant) must be stated precisely enough to implement.
3. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O, under 2 hours one core) that:
   (a) implements the rules from scratch and self-checks unique-winner exhaustively on forced 4-tile tricks under all 9 declarations;
   (b) computes the exact realizable ~-class census at k=1, printing the class count and outcome-constancy PASS/FAIL per class;
   (c) for TRUE: runs your construction on a deterministic sample of at least 500 realizable classes spanning all nine declarations and every hold pattern, replaying each constructed 28-tile hand through the full rules engine (legality of every play checked) and verifying the final trick lands in the intended class — `PASS reach <class-id>` per case, plus at least 25 fully worked hands printed as human-readable play scripts;
   (d) for FALSE: verifies your witness class is realizable, then executes the impossibility argument mechanically (exhaustion with printed size, or invariant check over all candidate histories in a stated finite reduction);
   `PASS`/`FAIL` lines; exit 0 iff all pass.
