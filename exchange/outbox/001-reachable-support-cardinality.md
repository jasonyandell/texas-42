---
number: 1
slug: reachable-support-cardinality
channel: new-chat
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_minimality_and_reachability.py
deliverable: exact |R_Str^m| with checkable enumeration program, or proved tighter interval than 26-46 bits
---
You are performing adversarial mathematical research on a formal foundation for Straight Texas 42 that you co-authored in a separate conversation. The attached files (the rules profile, the mathematical foundation, and one verification script) are the source of truth for that project; you have no access to anything else from it, and you must not rely on outside sources. This message restates every definition the task needs, so it is unambiguous even if the attachments are unavailable; where this message and the attachments differ, the attachments govern. Your response will be adjudicated mechanically by another model that has the full corpus: your programs will be executed, your witnesses re-run, and your proofs step-checked. Hedged, unverifiable, or contract-violating answers score zero.

## 1. The game (Straight 42 rules profile, compressed but exact)

Pips P = {0,...,6}. Dominoes D = the 28 two-element multisets {i,j} over P; write a domino h:l with h >= l. Doubles are p:p. For p in P, the natural incidence set is sigma_p = {d in D : p in d} (7 tiles). Count labels: c(d) = 5 for 5:0, 4:1, 3:2; c(d) = 10 for 6:4, 5:5; c(d) = 0 otherwise (total 35).

Four seats Z/4Z, clockwise; opposite seats are partners. Each deal: the 28 dominoes are partitioned uniformly at random into four labeled 7-tile hands (no boneyard). One-round auction: the player left of the shaker acts first, clockwise, each of the four players acting exactly once, choosing pass or a bid strictly exceeding the current high bid. Bids: point bids P(30) < ... < P(41) < mark bids M(1) < M(2) < ...; before any mark bid a player may bid at most M(2); a later mark bid raises by exactly one. Bid legality never depends on hand content. If all four pass, the deal is abandoned and redealt (irrelevant to this task: support inside one contracted hand never depends on earlier abandoned attempts). The last (highest) bidder wins, publicly chooses exactly one declaration delta from the nine-element set Delta = {0,1,2,3,4,5,6 (pip trump), DT (doubles trump), NT (no-trump)}, and leads trick 1. All bids, the declaration, and every played tile with its actor are public.

Declaration algebra. Called set kappa_delta: sigma_p if delta = p in P; the seven doubles if delta = DT; empty if delta = NT. Contexts Q = {0,...,6,7}, where 7 is the called suit. Effective suits: sigmahat_q = sigma_q \ kappa_delta for q in P, and sigmahat_7 = kappa_delta. Led context of a tile: ell_delta(d) = 7 if d in kappa_delta, else high(d) = max pip of d. Follow relation: F_delta(d,q) = 1 iff d in sigmahat_q. Powered set pi_delta = kappa_delta (empty for NT).

Play. Seven tricks, four plays each, clockwise from the leader. A leader may play any remaining tile; the tile it plays fixes the trick's led context q = ell_delta(d). A follower must play some remaining d with F_delta(d,q) = 1 if it holds one ("follow"); otherwise it may play anything ("slough"). Trick winner: rank r_delta(d) = p if delta = DT and d = p:p; TOP (above every integer) if d is a double and delta != DT; sum of ends if d is mixed. Tier of d in context q: 2 if d in pi_delta; 1 if d not in pi_delta and F_delta(d,q) = 1; else 0. Key = (tier, rank), compared lexicographically; the unique maximum among the four plays wins (proved, exhaustively verified over 737,100 cases) and leads the next trick. This is the full mechanics; count labels never affect legality or the winner.

## 2. Viewer support objects

Fix a viewer seat m for one contracted hand (declaration chosen, viewer knows its own dealt hand). Hidden seats in clockwise viewer-relative order: h_1 = m+1, h_2 = m+2, h_3 = m+3 (mod 4). After any legal public play prefix:

- B_s = set of tiles publicly played by seat s (including a tile in the unfinished current trick); the current hidden pool U = D \ (viewer's current remaining hand union all played tiles); capacity k_s = 7 - |B_s| for each hidden s.
- Void set V_s = the set of contexts q in which s has, at some earlier follower turn with led context q, publicly played a tile with F_delta(d,q) = 0. Possible set P_s = U \ Union_{q in V_s} sigmahat_q.
- The cell system is C = (U; (P_s, k_s)_{s in {h_1,h_2,h_3}}). Its fiber Phi(C) = { (H_s)_s : H_s subset of P_s, |H_s| = k_s, pairwise disjoint, union = U }. Losslessness theorem (attachment, Math section 7.5, claim CELL-05): Phi(C) is exactly the set of hidden-hand triples consistent with the viewer's information, i.e. the cells lose nothing.

Support normal form (Math section 7.10, claim CELL-14). For feasible C: marginal holder sets P*_s = { d : some world in Phi(C) puts d in H_s }; A(d) = { s : d in P*_s }; certain sets K_s = { d : A(d) = {s} }; ambiguous pool W = U \ (K_{h_1} u K_{h_2} u K_{h_3}); residual capacities r_s = k_s - |K_s|; active seats J+ = { s : r_s > 0 }. Provably |J+| is 0, 2, or 3. The normal form N(C) = ((K_s)_s, amb) where amb is:

- Determinate, if W is empty;
- Binary(iota, W, q), if |J+| = 2, where iota is the inactive seat, and with (a,b) the clockwise-ordered active pair, q = r_a (so r_b = |W| - q, 1 <= q < |W|); every d in W is possible at both active seats;
- Ternary(W, r_0, r_1, epsilon), if |J+| = 3, with r_2 = |W| - r_0 - r_1 and the partial exclusion map epsilon: W -> seats, epsilon(d) = s iff A(d) = J \ {s} (undefined iff A(d) = all three).

Theorem (CELL-14): N is a complete invariant — Phi(C) = Phi(C') iff N(C) = N(C') — and every exact deterministic support representation factors through it. Ternary fiber cardinality is [x0^r0 x1^r1 x2^r2] (x0+x1+x2)^{n_star} (x1+x2)^{n_0} (x0+x2)^{n_1} (x0+x1)^{n_2}, where n_s = |{d : epsilon(d) = s}| and n_star = |W| - n_0 - n_1 - n_2. A six-integer tuple (r_0,n_0,r_1,n_1,r_2,n_2) with all r_s in {1..7}, n_s >= 0, is a valid reduced ternary signature iff n_star >= 0 and n - n_s >= r_s + 1 for each s, where n = r_0+r_1+r_2.

## 3. The counting target

Define the reachable support image

R = R_Str^m = { N(C(h)) : h ranges over all legal Straight contracted-hand public play prefixes (any dealt viewer hand, any auction outcome, any declaration, zero through 28 plays), viewed from viewer m }.

Notes making the target unambiguous:

- Members of R are normal forms N, deduplicated as a set: the same N reached under different viewer hands, declarations, or prefixes counts once. The viewer's own hand is not part of N (U excludes it, but N does not record how D \ U splits between viewer hand and played tiles). The declaration is not part of N.
- By the viewer-relative gauge theorem (Math section 7.13), R is the same labeled object for every viewer m once hidden seats are named h_1, h_2, h_3 by clockwise offset; so there is one number.
- Every member of R is feasible (Phi nonempty); the empty branch never occurs.

Known bounds, both proved in the attachment (Math section 7.13.6, claims REACH-11/12/13): 44,352,165 <= |R| (explicit disjoint no-void families) and |R| < 64,123,542,674,901 < 2^46 (declaration-tagged outer certificates). So 26 <= ceil(log2 |R|) <= 46. Both packages explicitly refuse to collapse this interval by guesswork; closing it is open problem OPEN-11, the flagship open problem of the corpus.

Substrate you may build on (all proved in the attachment):

- Typed support transitions (Math 7.14, 7.14.1, TRANS-08/09): given N, delta, actor, played tile d, and the led context or lead boundary, the exact successor support is N' = N(theta_{s,d}(decode(N) intersect E_o)), where E_o conditions on possession (lead/follow) or possession-plus-complete-void (slough), and theta removes d from seat s. Computed without world enumeration by the matching-minor calculus: force edge d->s, delete slough-forbidden edges e->s for e in sigmahat_q, contract the played tile, reduce to the matching-supported core. Viewer plays leave N unchanged.
- Symbolic support machine (Math 7.13.7, REACH-14/15/16): starting from unrestricted support on the 21 unseen tiles for a fixed viewer hand, declaration, and leader, replaying an actor-attributed public trace with hidden actions accepted iff the conditioned successor support is nonempty accepts exactly the legally realizable traces, and the final support is exact. The symbolic play/support graph is finite, graded by total remaining tiles, and its support-output image is exactly R. This DAG is the intended counting substrate.
- Reachable capacity shapes: exactly the 50 triples with max - min <= 1 (Math 7.13.1). Holder-edge monotonicity and the 63-edge budget (Math 7.14.2). Lead-context structure: every declaration has exactly 7 leadable contexts whose lead-fiber sizes are the multiset {1,...,7} (Math 7.13.2).

## 4. THE TASK

Close OPEN-11, or tighten it. In order of value:

(A) Full credit. Compute |R| exactly. Design a dynamic program or enumeration over the symbolic play/support DAG (or any other provably exact method — e.g. canonicalize states to (support normal form, folded-trick residue, capacities) and prove the projection's fibers countable in closed form) and prove its correctness: you must prove that your state canonicalization neither merges two prefixes that generate different future support sets nor misses any reachable normal form, and that your deduplication across viewer hands and declarations is exact. Report the exact integer.

(B) Partial credit, either of:
  (B1) The exact cardinality of the no-void slice: |{ (U, k) : range(k) <= 1, and the full-cell system (U; (U,k_s)_s) with P_s = U for all s has its normal form in R }| — equivalently the number of distinct reachable supports whose cells are unrestricted. The proved floor families give all of shapes (7,7,7), the three labelings of (6,7,7), the three of (6,6,7), and (6,6,6); you must settle every deeper shape (5,5,5), (5,5,6), etc., where voids may have occurred but must not have bitten.
  (B2) The exact cardinality of the j <= 2 slice: normal forms realizable by some legal prefix with at most 2 completed tricks (at such prefixes every hidden capacity is 7-j or 6-j, so all capacities lie in {4,...,7} with range <= 1).

(C) Fallback credit. A proved strictly tighter interval than [26, 46] bits for ceil(log2 |R|) — every improvement must come with a full proof, not heuristics. An improvement of the ceiling via a proof that some counted outer certificates are redundant, or of the floor via new provably-disjoint reachable families with exact counts, both qualify.

Do not restate the packages' existing bounds as if they were progress. Do not estimate, sample, or extrapolate: only exact integers with proofs, or proved intervals, score.

## 5. DELIVERABLE CONTRACT

Your answer will be adjudicated mechanically. It must end with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS` containing:

1. A line `FINAL ANSWER:` followed by exactly one of: `|R| = <integer>` (full decimal, no scientific notation); `PARTIAL B1 = <integer>`; `PARTIAL B2 = <integer>`; `INTERVAL [<a>,<b>] bits`. If you achieved several, list each on its own line, strongest first.
2. One self-contained Python 3 program (single fenced code block, standard library only, deterministic, no network, no file I/O) that recomputes your claimed integer(s) from first principles — implementing the game rules and support algebra itself, not assuming them — and prints `PASS <check>` / `FAIL <check> <detail>` lines, exiting 0 iff all checks pass. Required internal checks, at minimum: (a) it reproduces the known corpus anchors 44,352,165 (floor family total) and, if your method touches the outer language, 7,124,838,074,989; (b) it recomputes your headline integer by your method; (c) where feasible, it cross-checks a restricted slice (e.g. one fixed viewer hand, or j <= 1) by brute-force prefix enumeration against your DP. State the expected runtime and memory; the program must finish within 6 hours on one CPU core at 16 GB. If your exact method inherently needs more than that, the program must instead verify every load-bearing lemma of your derivation on exhaustively checkable subdomains and recompute the final integer from your proved closed form or table — but then the derivation itself must be fully proved in the text.
3. Your proof, as numbered steps. Each step must carry a bracketed label of what it uses, e.g. `[USES: def. of N, step 4, TRANS-08]`. Steps appealing to unstated facts, or to intuition, invalidate the chain from that point.
4. Every witness or table you rely on, as explicit JSON in fenced blocks. Encode dominoes as two-element arrays [h,l] with h >= l; declarations as 0-6, "DT", "NT"; contexts as 0-7 (7 = called); hidden seats as 1, 2, 3 meaning viewer+1, viewer+2, viewer+3 clockwise.

A response whose program fails, whose integer disagrees with its own program, or whose proof has an unlabeled gap scores zero on the affected claim. Claims labeled honestly as partial score at their tier. An honest, proved partial result outranks an unproved full claim.
