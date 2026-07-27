---
number: 5
slug: census-integer-audit
channel: new-chat
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
deliverable: independent re-derivation of all census integers from formulas and structure alone, with fresh verification program and discrepancy report
---
You are performing an adversarial independent audit of the load-bearing integers in a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The two attached documents (rules profile and mathematical foundation) are the source of truth; you have no access to anything else from that project and must not rely on outside sources. IMPORTANT CONSTRAINT: the project also contains Python verification scripts that produced these integers. They are deliberately NOT attached, and you must NOT attempt to reconstruct or imagine their code. The point of this audit is independence: the prose formulas and the verification scripts share provenance (same author, same conversation) and could share a single error. You must re-derive every integer from the mathematical definitions and displayed formulas alone, with your own fresh method, and report any integer that does not reproduce. This message restates every definition needed, so it is unambiguous even if attachments are unavailable; where they differ, the attachments govern. Your response will be adjudicated mechanically by another model holding the full corpus, which will run your program and compare against the original verifiers. Hedged or unverifiable claims score zero. Confirming a wrong integer scores worse than zero would: it is the one failure mode this audit exists to catch, so check twice.

## 1. Structures being counted (exact definitions)

Pips P = {0..6}; dominoes D = the 28 two-element multisets over P (write h:l, h >= l); doubles p:p; sigma_p = { d : p in d } (7 tiles). Declarations delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)}; called set kappa_delta = sigma_p | the-7-doubles | empty; contexts 0..6 and 7 (called); effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta; led context ell_delta(d) = 7 if d in kappa_delta else max-pip(d); lead fiber L_{delta,q} = { d : ell_delta(d) = q }. Fact (proved in the attachment): every declaration has exactly 7 leadable contexts Lambda_delta, and the lead-fiber sizes over those 7 contexts are always the multiset {1,2,3,4,5,6,7}.

Three labeled hidden seats s in {0,1,2}, capacities k_s in {0..7}. A cell system is (U; (P_s,k_s)) with pool U subset of D, P_s subset of U; its fiber is the set of disjoint triples (H_s), H_s subset of P_s, |H_s| = k_s, union = U. The support normal form of a feasible system: marginal holder sets P*_s = { d : some fiber world puts d at s }; A(d) = { s : d in P*_s }; certain sets K_s = { d : A(d) = {s} }; ambiguous pool W = U minus certain tiles; residuals r_s = k_s - |K_s|; active seats J+ = { s : r_s > 0 }, with |J+| in {0,2,3} (proved). Normal form = ((K_0,K_1,K_2), amb) with amb:

- Determinate (W empty);
- Binary(iota, W, q): iota the unique inactive seat, both actives can hold every d in W, q = residual of the lower-indexed active seat, 1 <= q < |W|;
- Ternary(W, r_0, r_1, epsilon): all three residuals positive, r_2 = |W| - r_0 - r_1, partial map epsilon: W -> {0,1,2}, epsilon(d) = s iff A(d) = {0,1,2} \ {s} (undefined iff A(d) is all three).

The normal form is a complete invariant of the fiber (proved), so counting distinct normal forms counts distinct exact support states.

Ternary signature: the six integers (r_0, n_0, r_1, n_1, r_2, n_2) where n_s = |{ d in W : epsilon(d) = s }|; derived n = r_0+r_1+r_2 = |W| and n_star = n - n_0 - n_1 - n_2 (size of the unrestricted category). Claimed validity theorem: with every r_s in {1..7} and n_s >= 0, the six integers arise from a nonempty reduced ternary component iff n_star >= 0 and n - n_s >= r_s + 1 for each s.

Allocation matrix for a signature: a 4x3 matrix a_{c,s}, rows = categories (W_0, W_1, W_2, W_star), columns = seats, row sums = (n_0, n_1, n_2, n_star), column sums = (r_0, r_1, r_2), with the three forbidden entries a_{W_s,s} = 0. Signature relabeling group: S_3 acting simultaneously on seats and their excluded categories, i.e. permuting the three pairs (r_s, n_s); canonical representative = the sorted form; stabilizer G_lambda = { p in S_3 : (r_{p(s)}, n_{p(s)}) = (r_s, n_s) for all s }; G_lambda acts on the representative's allocation matrices; orbit sizes divide 6.

## 2. The claimed integers and displayed formulas — your audit targets

### (a) Full-schema census (Math 7.12.5)

For R >= 0 and bounds b = (b_0,b_1,b_2): F(R;b) = sum over 0 <= c_s <= b_s with c_0+c_1+c_2 <= R of R! / (c_0! c_1! c_2! (R-c_0-c_1-c_2)!). (Assign R labeled dominoes to three certain-holder categories, capped by b, and one outside-pool category.)

- N_det = F(28;(7,7,7)); CLAIMED 8,102,258,940,222,814.
- N_bin = sum over the 3 choices of inactive seat iota and residuals r_a, r_b in {1..7} of the two active seats (r_iota = 0, n = r_a + r_b) of: binom(28, n) * F(28-n; (7-r_0, 7-r_1, 7-r_2)); CLAIMED 11,495,078,055,913,018,482.
- N_ter = sum over all valid six-integer signatures of: 28! / ((28-n)! n_0! n_1! n_2! n_star!) * F(28-n; (7-r_0, 7-r_1, 7-r_2)); CLAIMED 1,830,955,704,129,296,418,354,864.
- Total with the single Empty state: 1 + N_det + N_bin + N_ter = 1,830,967,207,309,611,271,596,161, strictly between 2^80 and 2^81 (hence the "81 bits necessary and sufficient" headline).

### (b) Outer-certificate totals (Math 7.13.6)

For n in {0..21}, u in {0..7}: B_{n,u} = sum over u-element subsets Qu of the 7 leadable contexts of [x^n] ( prod_{q in Qu} ((1+x)^{|L_q|} - x^{|L_q|}) * prod_{q not in Qu} (1+x)^{|L_q|} ), with lead-fiber sizes {1..7} (declaration-independent). The subtraction forbids a used context from having ALL its lead-capable tiles inside the pool.

Capacity profiles: triples k = (k_1,k_2,k_3) in {0..7}^3 with max - min <= 1 (exactly 50). For each: h = max k_s; completed tricks j = 7 - h; B = set of seats with k_s = h - 1; f = |F(B)| via the fixed table {} -> 0, {h1} -> 1, {h2} -> 0, {h3} -> 0, {h1,h2} -> 2, {h1,h3} -> 1, {h2,h3} -> 1, {h1,h2,h3} -> 2 (seats h1,h2,h3 in clockwise-from-viewer order; equal profiles use f = 0); n = k_1+k_2+k_3. Certificate count per profile:

C(k) = sum_{u=0}^{j} 7^u B_{n,u} + [f > 0] * (7^{j+1} - (8 - 2^f)^{j+1}) * B_{n,j+1},

(the 7^u assigns one of the 7 nonempty seat-membership patterns to each used context; the bracketed term counts patterns whose current-trick context membership is a nonempty subset of the f already-acted followers; it applies only for j <= 6), EXCEPT the terminal profile (0,0,0), which contributes exactly 1 (one canonical empty-void certificate). CLAIMED: sum over the 50 profiles of C(k) = 7,124,838,074,989 (< 2^43, one declaration); times 9 declaration tags = 64,123,542,674,901 (< 2^46, the headline 46-bit ceiling); max over profiles C(k) = 839,220,930,919 (< 2^40).

### (c) Reachable floor (Math 7.13.6)

Four families of no-void supports (cells all-full: P_s = U), claimed pairwise disjoint and each fully reachable: capacity shape (7,7,7) with any 21-tile pool; the three labeled shapes permuting (6,7,7) with any 20-tile pool; the three labeled shapes permuting (6,6,7) with any 19-tile pool; shape (6,6,6) with any 18-tile pool. CLAIMED total binom(28,21) + 3*binom(28,20) + 3*binom(28,19) + binom(28,18) = 44,352,165 > 2^25 (the 26-bit floor).

### (d) Signature-census chain (Math 7.12.1, 7.12.5)

Under native bounds (r_s in {1..7}, validity as in section 1): CLAIMED exactly 136,514 seat-labeled valid six-integer ternary signatures; exactly 23,842 S_3-orbits (canonical representatives); exactly 1,667,666 feasible allocation matrices summed over all 136,514 labeled signatures, with at most 114 for any one signature; exactly 296,721 matrices summed over the 23,842 canonical representatives; stabilizer split of the 23,842: 21,686 trivial, 2,121 of order 2, 35 of order 6; and 279,048 total stabilizer orbits of matrices, at most 103 orbits for one signature, orbit sizes in {1,2,3,6}.

## 3. THE TASK

Audit at two levels, and report per integer:

Level 1 — formula-vs-structure. For each of (a)-(d), verify that the DISPLAYED FORMULA actually counts the DEFINED OBJECT: e.g. does the binary branch really biject with (inactive seat, residual pair, ambiguity-pool choice, capped certain/outside assignment) without double-counting binary states whose ambiguity pools coincide but split values differ? Does the ternary branch multinomial correctly assign labeled dominoes to categories given that the normal form stores sets, not ordered labels? Is the validity criterion n - n_s >= r_s + 1 correctly the reduced-and-nonempty condition (strict Hall for singletons plus two-seat and full-set arguments)? In B_{n,u}, is subtracting x^{|L_q|} the correct exclusion, and is declaration-independence really implied by the {1..7} fiber multiset? Is the (7^{j+1} - (8-2^f)^{j+1}) pattern count the correct count for "some used context whose membership pattern is a nonempty subset of the f already-acted followers", given 7 nonempty patterns per context and 2^f - 1 qualifying nonempty subsets? Any formula-level error you find, state precisely with a minimal concrete counterexample instance.

Level 2 — numeric. Independently evaluate every claimed integer with exact integer arithmetic, using a FRESH method of your own design — not a transcription of the displayed sum where a structurally different route exists. Wherever feasible, compute each number by TWO genuinely different routes (e.g. generating functions vs direct DP vs explicit enumeration for the smaller ones; for (d), direct enumeration of signatures/matrices is cheap and should be exact and total). The floor arithmetic in (c) and the disjointness claim (why do the eight labeled families pairwise collide nowhere? capacities and pool sizes are part of the normal form — confirm this suffices) must both be addressed.

Report format per integer: REPRODUCED (with your value) or DISCREPANT (with your value, the claimed value, and where you believe the error lies: formula, evaluation, or your own method — resolve which before reporting). An unresolved discrepancy must still be reported as such; silently dropping a target scores zero for that target.

## 4. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. A table (markdown) with one row per claimed integer — all of: N_det, N_bin, N_ter, the grand total, the 2^80/2^81 sandwich, 7,124,838,074,989, 64,123,542,674,901, 839,220,930,919, 44,352,165, 136,514, 23,842, 1,667,666, 114, 296,721, 21,686, 2,121, 35, 279,048, 103 — columns: claimed value, your value, verdict (REPRODUCED / DISCREPANT), method(s) used. Every value in full decimal.
2. `FINAL ANSWER: ALL REPRODUCED` or `FINAL ANSWER: DISCREPANCIES: <comma-separated list of the affected integers>`.
3. One self-contained Python 3 program (single fenced code block, standard library only, deterministic, no network/file I/O, finishing under 1 hour on one core) written by you from the definitions in this message — do not attempt to mirror the project's verifier structure — that recomputes every integer in the table and prints one `PASS <name> <value>` or `FAIL <name> claimed=<x> computed=<y>` line per integer, exiting 0 iff all pass against YOUR reported values (so if you report a discrepancy, the program must agree with you, not with the claim). Where you used two routes, the program must run both and cross-check them against each other.
4. Your Level 1 audit as numbered findings, each labeled `[FORMULA OK: <target>]` or `[FORMULA ERROR: <target>]` with the argument in numbered steps labeled `[USES: ...]`.

Arithmetic must be exact throughout (Python ints); any use of floating point in a counting path invalidates that target. A program that fails to reproduce your own table scores zero. If everything reproduces, say so plainly; if something does not, the discrepancy report is the single most valuable thing you can return.
