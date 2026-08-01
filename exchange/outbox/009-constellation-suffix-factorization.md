---
number: 9
slug: constellation-suffix-factorization
channel: new-chat
status: DRAFT — not cleared for submission (quota must be cleared with Jason first; no .ready until his explicit go)
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_foundation.py
deliverable: proof that suffix minimax factors through the constellation (declaration-blind relational structure of the living tiles), or a machine-verifiable counterexample pair; plus the bisimulation lemma or its explicit failure
---
You are performing adversarial mathematical research on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files (rules profile, mathematical foundation, one verification script) are the source of truth; you have no access to anything else from that project and must not rely on outside sources. This message restates every definition the task needs, so it is unambiguous even if the attachments are unavailable; where they differ, the attachments govern. Your response will be adjudicated mechanically by another model holding the full corpus: programs executed, witnesses re-run, proofs step-checked. Hedged or unverifiable claims score zero.

## 1. Game mechanics (compressed but exact)

Pips P = {0..6}; dominoes D = the 28 multisets {i,j} over P, written h:l with h >= l; doubles p:p; sigma_p = { d : p in d }. Count labels c(d): 5 for 5:0, 4:1, 3:2; 10 for 6:4, 5:5; else 0. Four seats Z/4Z clockwise, opposite seats partners. Declarations delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)} — nine total.

Called set kappa_delta = sigma_p (delta = p in P) | the 7 doubles (DT) | empty (NT). Contexts 0..6 and 7 = called suit; effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta. Led context ell_delta(d) = 7 if d in kappa_delta else high(d). Follow F_delta(d,q) = 1 iff d in sigmahat_q. The leader plays any remaining tile, fixing q = ell_delta(d); a follower holding a tile with F_delta(d,q) = 1 must play such a tile, else sloughs freely. Trick key of d in context q = (tier, rank), lexicographic: tier 2 iff d in kappa_delta (NT has no tier 2), tier 1 iff uncalled with F_delta(d,q) = 1, else 0; rank = p for d = p:p under DT, TOP for doubles otherwise, pip sum for mixed. The unique maximum wins the trick (uniqueness proved in the corpus), wins 1 + sum of count labels of the four played tiles, and leads the next trick. Count labels never affect legality or the winner; they only affect scoring.

## 2. Suffix endgames and constellations

A **suffix position** at depth k (1 <= k <= 7) is X = (delta, L, H, ell): a declaration delta, a live set L of 4k dominoes, a partition H of L into four hands H_0..H_3 of k tiles indexed by seat, and a leader seat ell. Play proceeds by the trick mechanics above until L is exhausted; both teams play optimally (perfect information, alternating max/min by team); the **suffix value** v(X) is the exact minimax margin, points of the leader's team minus points of the other team, where each trick is worth 1 + count labels of its four tiles. (No auction, no history: the suffix is taken as given. Whether a suffix extends back to a legal full deal is out of scope here.)

The **constellation** of X is the relational structure Con(X) with universe L and the following data, and NOTHING else — in particular no pip names and no declaration name:

- hold(d) in {0,1,2,3}: the holder's seat offset from the leader (so the led hand is offset 0, the leader's partner offset 2);
- c(d) in {0,5,10}: the count label;
- FOLLOW(d,e) in {0,1}: F_delta(d, ell_delta(e)) — would d follow if e were led;
- ORD(d,e,g) in {<,=,>}: the comparison of the trick keys of d and e in the context ell_delta(g), for all d,e,g in L.

Two suffix positions X, X' (possibly under DIFFERENT declarations) are **constellation-equivalent**, X ~ X', iff some bijection phi: L -> L' preserves hold, c, FOLLOW, and ORD exactly. Note ~ deliberately pools all nine declarations: trump-ness enters only through the FOLLOW/ORD relations it induces; the name is forgotten.

Context for calibration (frozen instrument output exists on the corpus side; your program must compute its own numbers, ours are withheld deliberately): at k=1, exhaustively, the forced-trick outcome is constant on every ~-class across all nine declarations pooled; substitution probes at k in {1,2,3} within fixed declarations found tens of thousands of value agreements and zero divergences. Nothing beyond that is established: no theorem, no cross-declaration evidence for k >= 2. That is the gap you are hired to close or blow open.

## 3. THE TASK

Prove or refute:

CONJECTURE C1 (suffix factorization). If X ~ X' then v(X) = v(X').

The expected proof route for TRUE is strong bisimulation by induction on k, and the following obligations are each known gaps — do all of them explicitly; silently assuming any one invalidates the chain:

1. **Legality transports.** At every decision point, phi maps the legal move set of the actor in X onto the legal move set of the corresponding actor in X': leader's freedom is universal; follower obligation is determined by FOLLOW(-, led tile) and hand membership (hold). Beware: follow legality depends on the FOLLOWING player's remaining hand, which shrinks during play — state the induction so the preserved data suffices at every depth, including mid-trick states (partial tricks), which the constellation as defined does not directly index.
2. **Resolution transports.** The trick winner is determined by ORD(-,-,led tile) alone (confirm winner determination needs only strict contextual comparisons, never numeric rank values), and the award by c plus the constant 1; the new leader is the winner, and hold must be re-based to the new leader — show phi still preserves the re-based hold.
3. **Hereditariness (the crux).** After a trick removes four tiles, the constellation of the successor position is the restriction of the original structure to the survivors: FOLLOW and ORD among survivors, in contexts led by survivors, are unchanged by deaths (effective suits are static within a declaration), and no NEW relational information is needed. Prove this restriction-closure exactly; it is what makes the induction go through.
4. **Conclusion.** Assemble 1-3 into: ~ is a strong bisimulation between the suffix game trees of X and X', hence v(X) = v(X') by induction on k, with the base case k=1 the forced trick.

For FALSE: exhibit two suffix positions X, X' with an explicit bijection phi preserving all four components (your program must verify this mechanically), together with exact minimax values v(X) != v(X') computed by your own solver. Cross-declaration pairs (e.g. a pip-trump position against a DT or NT position) are the most suspicious territory; so are positions where a double's TOP rank interacts with sloughs. A counterexample here is worth more than a proof — it would falsify the entire retrograde program as keyed.

SECONDARY (partial credit, state scope exactly): the BACKWARD direction. C1, even if true, does not by itself license one-representative-per-class backward induction: that needs the abstraction to commute with the backward step. State precisely and prove or refute: for X ~ X', the sets of predecessor constellations (positions one full trick earlier whose trick-winner becomes the current leader, restricted to predecessors realizable from the 28-tile universe with the four newly-resurrected tiles disjoint from L) coincide. The corpus's prediction on record is that failures, if any, are reachability-shaped — equivalent suffixes whose predecessor sets differ only via realizability constraints, not via value-relevant structure. An explicit witness pair either way, with the differing predecessor constellation exhibited, is the deliverable.

## 4. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. `FINAL ANSWER: TRUE (C1 proved)` or `FINAL ANSWER: FALSE (counterexample)` or `FINAL ANSWER: PARTIAL (<scope>)`, plus a one-line verdict on the SECONDARY backward question if you addressed it.
2. Proof as numbered steps, each labeled `[USES: ...]` citing definitions above, prior steps, or the attached corpus documents. Obligations 1-3 must each appear as named explicit steps.
3. One self-contained Python 3 program (single fenced block, stdlib only, deterministic — fixed seeds, no network/file I/O, under 2 hours on one core) implementing the trick mechanics and the constellation abstraction from scratch, that:
   (a) self-checks mechanics: exhaustively verifies unique-winner on all 4-subsets of D as forced tricks under all 9 declarations, printing the count checked;
   (b) k=1 census: enumerates ALL suffix positions at k=1 (every 4-subset, every hold/leader arrangement up to the two-opponent symmetry, every declaration), computes Con up to ~, and reports: number of ~-classes, number of distinct (winner-team, award) outcomes, and PASS iff the outcome is constant on every class — these numbers will be diffed against the corpus's frozen instrument;
   (c) k=2 cross-declaration probe: generates a deterministic, stated, diverse family of at least 20,000 suffix positions at k=2 covering all nine declarations, groups them by canonical form of Con (state your canonicalization; it must be a true class invariant, minimum over the admissible relabelings), solves exact minimax for every member of every group with >= 2 members, and prints PASS iff values are constant within every group, plus the group count and the largest group;
   (d) bisimulation spot-check: for at least 5,000 ~-equivalent pairs found in (c), matches legal moves through phi and verifies successor positions are ~-equivalent, printing PASS/FAIL per obligation — this mechanically certifies obligations 1-3 on the family;
   (e) for FALSE: replays your witness pair instead — verifies phi preserves all four components, solves both positions exactly, prints both values and `PASS counterexample-verified` iff they differ.
   `PASS <check>` / `FAIL <check> <detail>` lines throughout; exit 0 iff all pass.
