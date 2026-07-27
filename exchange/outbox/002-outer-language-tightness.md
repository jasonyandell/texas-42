---
number: 2
slug: outer-language-tightness
channel: new-chat
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_minimality_and_reachability.py
deliverable: witness passing all four outer checks yet unreachable, or sufficiency proof for the one-completed-trick phase
---
You are performing adversarial mathematical research on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files (rules profile, mathematical foundation, one verification script) are the source of truth; you have no access to anything else from that project and must not rely on outside sources. This message restates every definition the task needs, so it is unambiguous even if the attachments are unavailable; where they differ, the attachments govern. Your response will be adjudicated mechanically by another model holding the full corpus: programs executed, witnesses re-run, proofs step-checked. Hedged or unverifiable claims score zero.

## 1. Game and support objects (compressed but exact)

Pips P = {0..6}; dominoes D = the 28 multisets {i,j} over P, written h:l with h >= l; doubles p:p; sigma_p = { d : p in d } (7 tiles). Four seats Z/4Z clockwise, opposite seats partners; each dealt 7 of the 28 uniformly, no boneyard. A one-round auction (bid legality independent of hand content) selects a bidder who publicly declares delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)} and leads trick 1.

Declaration algebra: called set kappa_delta = sigma_p (delta = p), the 7 doubles (DT), empty (NT). Contexts Q = {0..6, 7} with 7 the called suit; effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta. Led context ell(d) = 7 if d in kappa_delta else high(d). Follow: F(d,q) = 1 iff d in sigmahat_q. A leader plays anything, fixing context q = ell(d); a follower holding a tile with F(d,q) = 1 must play such a tile, else may play anything (slough). Winner: key (tier, rank) lexicographic, tier 2 for called tiles (except NT: kappa empty), tier 1 for uncalled followers, 0 otherwise; rank = index of double under DT, TOP for doubles otherwise, pip sum for mixed; the unique max wins and leads next. Seven tricks. Count labels never affect legality or winner.

Viewer support. Fix viewer m; hidden seats h_1, h_2, h_3 = m+1, m+2, m+3 clockwise. After a legal public prefix: U = D \ (viewer's remaining hand union played tiles); k_s = 7 - |played by s|; void set V_s = contexts where s publicly failed to follow; P_s = U \ Union_{q in V_s} sigmahat_q. Cell system C = (U; (P_s,k_s)_s); fiber Phi(C) = disjoint triples H_s subset of P_s with |H_s| = k_s covering U. Phi is exactly the set of possible hidden-hand triples (losslessness, Math 7.5).

Support normal form N(C) (Math 7.10): marginal holder sets P*_s (d in P*_s iff some world puts d at s); A(d) = { s : d in P*_s }; certain sets K_s = { d : A(d) = {s} }; W = U minus the certain tiles; residuals r_s = k_s - |K_s|; the active-seat count |J+| is 0, 2, or 3; N = ((K_s), amb) with amb = Determinate | Binary(inactive seat, W, split) | Ternary(W, r_0, r_1, epsilon), epsilon(d) = s iff A(d) omits exactly s. N is a complete invariant of Phi (CELL-14). A support is "reduced" when presented as N — no dead edges, no unstated certainties.

Reachable image: R = { N(C(h)) : h any legal Straight contracted-hand public prefix, any viewer hand, any declaration }, deduplicated as a set of normal forms. Members of R are what a legal game can actually present to a viewer.

## 2. The outer necessary language (Math 7.13.1-7.13.6)

Every reachable support satisfies all four of the following checks. Fix a candidate declaration delta and capacity triple k = (k_1,k_2,k_3).

(1) Capacity shape: max_s k_s - min_s k_s <= 1 (proved exact: exactly the 50 such triples occur). Derived: h = max k_s, completed tricks j = 7 - h, low-capacity seat set B = { h_i : k_i = h - 1 }.

(2) Schedule admissibility: the maximal already-acted-follower set F(B) is given by the proved table B -> F(B): {} -> {}; {h1} -> {h1}; {h2} -> {}; {h3} -> {}; {h1,h2} -> {h1,h2}; {h1,h3} -> {h1}; {h2,h3} -> {h3}; {h1,h2,h3} -> {h2,h3} (for equal profiles use F = {} at a trick boundary). With M(q) = { h_i : q in V_i } and Qused = { q : M(q) nonempty }, the void-mask triple is schedule-admissible iff |Qused| <= j, or |Qused| = j + 1 and there exists q in Qused with M(q) a nonempty subset of F(B). Voids can only occur in leadable contexts Lambda_delta (for pip trump t: (P \ {t}) u {7}; DT: {1..6, 7}; NT: {0..6}).

(3) Lead witness: for every q in Qused, L_{delta,q} \ U is nonempty, where L_{delta,q} = { d : ell_delta(d) = q } is the lead fiber (a void in q requires some tile that leads q to have been played already, hence to be outside the hidden pool). The lead-fiber sizes are always the multiset {1,...,7}.

(4) Hall feasibility: the cells P_s = U \ Union_{q in V_s} sigmahat_q with capacities k_s admit at least one disjoint system (Phi nonempty).

The packages prove this conjunction NECESSARY and prove it is not encodable-free: the certificate count 64,123,542,674,901 < 2^46 upper-bounds |R| (REACH-11). They deliberately do not claim sufficiency.

The only feasible-but-unreachable witness in the corpus (REACH-10, Math 7.13.5) is: k = (6,6,6), U = sigma_0 union doubles union {2:1, 3:1, 3:2, 4:1, 4:2} (18 tiles), P_1 = U \ sigma_0, P_2 = P_3 = U. The proof exhausts all 450 static generators (9 declarations x (7 leadable contexts x 7 nonempty void-membership patterns + no-void)) and finds the only two decoding to this support are zeroes-trump/context-7/seat-1-void and NT/context-0/seat-1-void — and both FAIL the lead-witness check (their lead fibers sigma_0 and {0:0} lie inside U). So this witness does not test the full conjunction: it is killed by check (3) itself. Whether the four checks TOGETHER are sufficient is open, and is exactly what you must attack.

## 3. THE TASK (either direction scores full credit)

(A) Counterexample. Construct a support normal form S (equivalently a cell system (delta, k, (V_s), U) whose reduction is S — supply both) such that:

- some declaration-tagged tuple (delta, k, (V_1,V_2,V_3), U) passes ALL FOUR checks (1)-(4) and its cells reduce to S; and
- S is NOT in R: no legal Straight prefix, under ANY declaration and viewer hand (not just the tagged delta), produces S.

Non-reachability is the hard half and must be proved, not asserted. Strongly prefer a witness in the shallowest possible phase, ideally j = 1 (capacity shapes (6,6,6), (5,6,6), (5,5,6) region) with exactly one used void context, because there the corpus provides a complete finite refutation method: with at most one void context, every candidate generator is static (Math 7.13.5 style), so exhausting the finitely many (declaration, context, void-membership, and if needed one-tricks-of-play) generators that could decode to S is a complete proof. If your witness lies deeper, you must supply a complete argument covering multi-trick ancestries — e.g. an invariant of the matching-minor dynamics (monotone holder-edge deletion, Math 7.14.2: A_{t+1}(e) subset of A_t(e); a surviving certain tile stays certain; ambiguity tags only move Ternary -> Binary -> Determinate) violated by every possible predecessor of S.

(B) Sufficiency for the one-trick phase. Prove: for j = 1 states with at most one used void context (say capacity shapes with max = 6 arising at or during trick 2), every tuple (delta, k, (V_s), U) passing checks (1)-(4) has its reduced support in R. The proof must be constructive: an explicit algorithm that, given any passing tuple, builds a complete deal and a legal prefix (auction, declaration delta', plays with actors) whose viewer support reduces to the same normal form — plus a proof that the construction always succeeds. If true, this makes the outer language exact on that phase and materially tightens the 46-bit ceiling reasoning; state precisely what phase your proof covers.

Partial credit: (A) or (B) restricted to a single named declaration class (pip trump / DT / NT); or a proof that within phase j = 1 the conjunction is sufficient for no-void tuples plus an explicit characterization of exactly which one-void tuples are reachable. Zero credit: probabilistic arguments, unproved assertions of unreachability, or re-deriving REACH-10.

## 4. DELIVERABLE CONTRACT

End your response with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS` containing:

1. A line `FINAL ANSWER: COUNTEREXAMPLE` or `FINAL ANSWER: SUFFICIENT (phase: <exact phase description>)` (or `FINAL ANSWER: PARTIAL (<scope>)`).
2. For (A): the witness as one fenced JSON block with fields `declaration` (0-6, "DT", "NT"), `capacities` [k1,k2,k3], `voids` (three arrays of contexts 0-7), `pool` (array of [h,l] arrays, h >= l), and `normal_form` (`certain` three arrays, `ambiguity` object with `tag`, and per tag: `inactive`/`split` or `residuals`/`exclusions`). Hidden seats are 1,2,3 = viewer+1,+2,+3 clockwise.
3. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O) that: implements the rules and support algebra from scratch; for (A) verifies checks (1)-(4) pass for your tuple, computes the reduction to your claimed normal form, and mechanically executes your non-reachability argument (e.g. exhausts your stated complete generator family and confirms none decodes to S, printing the family size); for (B) implements your construction algorithm and, over an explicitly enumerated (or deterministically strided, stride stated) family of passing tuples in the covered phase of at least 10,000 tuples, replays each constructed prefix through a from-scratch legality checker and confirms the support normal form matches, printing counts. All results as `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all pass, runtime under 2 hours on one core.
4. Your proof as numbered steps, each labeled `[USES: ...]` (definitions, prior steps, or named corpus claims like CELL-14, TRANS-08, REACH-10). The completeness of any exhausted generator family is itself a step that must be proved, not assumed.

A witness whose program fails any check, or a sufficiency proof whose construction fails on any enumerated tuple, scores zero. An honest partial result with a complete proof outranks a full claim with a gap.
