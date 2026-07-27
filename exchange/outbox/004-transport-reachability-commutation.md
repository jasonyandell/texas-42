---
number: 4
slug: transport-reachability-commutation
channel: new-chat
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_foundation.py
deliverable: proof that pip-trump transport bijects reachable support images (collapsing 9 census tags to 3), or a reachable support whose transport is unreachable
---
You are performing adversarial mathematical research on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files (rules profile, mathematical foundation, one verification script) are the source of truth; you have no access to anything else from that project and must not rely on outside sources. This message restates every definition the task needs, so it is unambiguous even if the attachments are unavailable; where they differ, the attachments govern. Your response will be adjudicated mechanically by another model holding the full corpus: programs executed, witnesses re-run, proofs step-checked. Hedged or unverifiable claims score zero.

## 1. Game mechanics (compressed but exact)

Pips P = {0..6}; dominoes D = the 28 multisets {i,j} over P, written h:l with h >= l; doubles p:p; sigma_p = { d : p in d }. Count labels c(d): 5 for 5:0, 4:1, 3:2; 10 for 6:4, 5:5; else 0. Four seats Z/4Z clockwise, opposite seats partners; uniform deal of 7 tiles each, no boneyard. One-round auction, clockwise from left of shaker, each player once, pass or a bid exceeding the current high bid; legality is INDEPENDENT of hand content; last bidder wins, publicly declares delta in {0..6 (pip trump), DT (doubles trump), NT (no-trump)}, leads trick 1.

Called set kappa_delta = sigma_p (delta = p in P) | the 7 doubles (DT) | empty (NT). Contexts 0..6 and 7 = called suit; effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta. Led context ell_delta(d) = 7 if d in kappa_delta else high(d). Follow F_delta(d,q) = 1 iff d in sigmahat_q. Leader plays any remaining tile, fixing q = ell_delta(d); a follower holding a follower tile must follow, else sloughs freely. Trick key of d in context q = (tier, rank), lexicographic: tier 2 iff d in kappa_delta (NT has no tier 2), tier 1 iff uncalled with F_delta(d,q) = 1, else 0; rank = p for d = p:p under DT, TOP for doubles otherwise, pip sum for mixed. Unique maximum wins and leads the next trick (proved). Count labels never affect legality or the winner; they only affect scoring.

Viewer support. Fix viewer m; hidden seats h_1,h_2,h_3 = m+1,+2,+3 clockwise. After a legal public prefix: pool U = D \ (viewer's remaining hand union played tiles); capacities k_s = 7 - |played by s|; voids V_s = contexts where s publicly failed to follow; cells P_s = U \ Union_{q in V_s} sigmahat_q; fiber Phi = the disjoint capacity-respecting hidden-hand triples covering U (losslessness: exactly the possible hidden hands, Math 7.5). Support normal form N (Math 7.10, claim CELL-14): certain sets K_s (tiles marginally held only by s), ambiguous pool W, residual capacities, tag Determinate / Binary(inactive seat, W, split) / Ternary(W, r_0, r_1, exclusion map epsilon), a complete invariant of Phi.

Declaration-tagged reachable support image: for delta in Delta define

R_delta = { N(C(h)) : h a legal Straight contracted-hand public prefix whose declaration is delta, any viewer hand, any number of plays }.

The union R = Union_delta R_delta over the nine declarations is the reachable support image whose exact cardinality is open problem OPEN-11 (proved interval: 2^25 < |R| < 2^46).

## 2. The transport (Math 3.9-3.10, claims ALG-22/23)

For pip trumps t, u in P define f_{t,u}: P -> P by f_{t,u}(t) = u, and mapping the i-th smallest member of P \ {t} to the i-th smallest member of P \ {u} (order-preserving on the complement). Extend f_{t,u} to dominoes endpoint-wise ({i,j} -> {f(i),f(j)}), to pip contexts (q -> f(q), fixing called context 7), and to declarations (t -> u).

PROVED (ALG-22, "all pip-trump mechanics are isomorphic"): f_{t,u} is an isomorphism of UNSCORED mechanics structures M_t -> M_u — it transports called/powered membership, effective-suit incidence, led context, follow relation, and every strict contextual comparison of trick keys (order only, not numeric rank labels). PROVED (ALG-23): the nine declarations have exactly three unscored mechanics classes: {all seven pip trumps}, {DT}, {NT}. NOT preserved in general: count labels c (only the identity and the 2<->3 swap preserve those), hence anything scoring-dependent.

Support content is count-blind: pools, voids, cells, fibers, and normal forms are built only from tile identities, follow/lead relations, and public attribution — never from c. Transport a normal form componentwise: f(N) applies the endpoint map to every tile in each K_s and in W, keeps seat labels, residuals, tags, and split values fixed, and maps exclusions by epsilon'(f(d)) = epsilon(d).

## 3. THE TASK

Prove or refute:

CONJECTURE (transport-reachability commutation). For all pip trumps t, u: f_{t,u}(R_t) = R_u, and f_{t,u} restricted to R_t is a bijection onto R_u.

If TRUE, the declaration-tagged reachable census collapses from 9 tags to 3 classes (the 7 pip-trump images coincide up to relabeling), directly simplifying OPEN-11's counting and shaving its declaration-tagged outer-certificate bound. If FALSE, that is a startling asymmetry — support semantics is count-blind, so a failure means legal-PREFIX generation (which includes auction and turn structure, also count-blind) still manages to break the symmetry — and you must exhibit it explicitly.

The expected proof obligation for TRUE (do all of it; each piece is a known gap, none is in the corpus):

1. Trace transport. Define the transport of a full legal object: deal (transport all four hands), auction (unchanged — bids are hand-independent), declaration (t -> u), and every play (s, d) -> (s, f(d)). Prove by induction on the prefix that transport preserves: viewer-hand contents, leader identity, led context (ell_u(f(d)) = f_hat(ell_t(d)) — including the subtlety that "high(d)" is NOT preserved by f on tiles containing t or u; show the effective led context still transports because called tiles map to called tiles and on uncalled tiles f is order-preserving), follow legality and slough legality (sigmahat images), trick winner (via the transported strict key order — note ALG-22 gives you order preservation within a context; you must confirm winner determination uses only that), and hence legality of the whole transported prefix.
2. Support transport. Prove the cells of the transported prefix are the f-images of the original cells (pools, capacities, voids map correctly — voids: seat s void in context q iff transported seat void in f_hat(q)), and conclude via CELL-14 that the final normal form is f(N).
3. Bijectivity. Establish the inverse: prove (do not assume) that f_{u,t} = f_{t,u}^{-1} as permutations of P — the plausible one-line argument is that the inverse of an order-preserving bijection between the complements P \ {t} and P \ {u} is itself order-preserving — then apply steps 1-2 in both directions to conclude f_{t,u}(R_t) = R_u exactly, with f_{u,t} inducing the inverse bijection.
4. Corollary for the census: |R_t| = |R_u| for all pip trumps t,u; state precisely the resulting 3-class structure of R = R_pip-class union R_DT union R_NT and what overlap between the three classes remains uncontrolled (distinct declarations can produce the SAME normal form — e.g. no-void states with equal pools; the union is not disjoint; quantify or bound the overlap if you can, it is part of OPEN-11's structure).

For FALSE: exhibit a support S in R_t with f_{t,u}(S) not in R_u, with a realizing legal prefix for S (complete deal + auction + declaration + attributed plays) and a complete proof of non-membership for the image (a finite exhaustion argument over all possible generators of f(S) in the style of the corpus's REACH-10 proof, or an invariant argument — mere failure to find a preimage scores zero).

Partial credit: the theorem proved for the restricted family of no-void prefixes only; or for prefixes with at most one completed trick; scope must be stated exactly.

## 4. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS`:

1. `FINAL ANSWER: TRUE (bijection proved)` or `FINAL ANSWER: FALSE (counterexample)` or `FINAL ANSWER: PARTIAL (<scope>)`.
2. Proof as numbered steps, each labeled `[USES: ...]` citing definitions above, prior steps, or named corpus claims (ALG-22, ALG-23, CELL-14, losslessness). The led-context subtlety in step 1 and the inverse subtlety in step 3 must each be an explicit step; silently assuming either invalidates the chain.
3. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O, under 2 hours one core) implementing the rules and support algebra from scratch, that: (a) verifies ALG-22 order-preservation for all 49 ordered pip-trump pairs (including identities) by exhaustive contextual comparison; (b) generates a deterministic diverse family of at least 5,000 legal prefixes under pip-trump declarations (state the generator; cover all 7 trumps, depths 0-28, voids and sloughs must occur), and for each prefix and several (t,u) pairs checks that the transported prefix is legal and that its computed normal form equals the transported normal form — i.e. mechanically certifies commutation on the family; (c) for FALSE instead: replays your witness prefix, computes S, transports it, and executes your finite non-membership exhaustion, printing the exhausted family size. `PASS <check>` / `FAIL <check> <detail>` lines; exit 0 iff all pass.
4. Witnesses (for FALSE) as fenced JSON: `deal` (four arrays of [h,l]), `declaration`, `plays` ([seatoffset, [h,l]] pairs, seat offsets 0 = viewer, 1-3 hidden clockwise), `support` and `transported_support` (normal-form objects: `certain` 3 arrays, `ambiguity` tagged object), `t`, `u`.

A TRUE claim whose program finds one commutation failure scores zero; a FALSE claim without a complete non-membership proof scores zero. A cleanly scoped partial theorem outranks an overclaimed full one.
