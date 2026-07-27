---
number: 7
slug: fifth-condition-ceiling
channel: continuation
conversation_url: https://chatgpt.com/c/6a66e7f0-57cc-83ea-b6c8-eab6080b8b76
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_minimality_and_reachability.py
deliverable: exact necessary-condition-filtered outer census and a proved ceiling below 46 bits (target 45 or lower)
---
This is a follow-up in the same conversation where you constructed the feasible-but-unreachable witness (NT, (6,6,6), V_1={6}) and discovered the follower-supply obstruction. Your 002 answer was ADJUDICATED CONFIRMED: your program ran 16/16 PASS, three independent adversarial referees returned SOUND (one re-verified by a 1,276,560-trace single-layer enumeration, one by a 301,860-state recursive game DFS with max-flow feasibility), and your witness plus the fifth condition were independently re-implemented from scratch in Rust and reproduced exactly (4/4 classic checks pass, 3/450 generators decode, 425,520 traces, 0 realizers, fifth check rejects). The follower-supply obstruction is now an established fact of the corpus, and everything you built in that turn — the generator taxonomy, the trace-exhaustion machinery, your from-scratch rules/support implementation — is still in your context and may be reused verbatim. The same three attachments are the source of truth; rely on no outside sources. This message restates every fact it needs so it is self-contained even if the earlier turn is unavailable; where they differ, the attachments govern. Your response is adjudicated mechanically by a model holding the full corpus: your program executed, your proofs step-checked, your conditions tested against machine-generated legal games. Hedged or contract-violating answers score zero.

## 0. Where the interval stands (established facts you may cite without re-proving)

- Floor (sibling investigations, both adjudicated CONFIRMED and independently reproduced): two structurally disjoint certified reachable families totalling 17,668,066,045 + 19,245,318,365 = **36,913,384,410 > 2^35** distinct reachable support normal forms. So `ceil(log2 |R|) >= 36`.
- Ceiling (corpus REACH-11, which you re-derived in a sibling conversation): the declaration-tagged necessary outer language has exactly **64,123,542,674,901 < 2^46** members, `7,124,838,074,989` per declaration (the per-declaration count is declaration-independent). Every reachable support satisfies all the necessary checks, hence admits at least one tagged outer profile, hence `|R| <= 64,123,542,674,901`.
- Current proved interval: **[36, 46] bits**. Every improvement so far has come from the floor side. This dispatch attacks the ceiling — the first ceiling movement since REACH-11. Dropping the tagged outer total below `2^45 = 35,184,372,088,832` drops the ceiling to 45 bits.

## 1. The outer language, exactly (the object you must count)

Notation as in your 002 turn (attachments govern): pips P = {0..6}; 28 dominoes h:l; sigma_p = tiles containing p; declarations delta in {0..6, DT, NT}; called set kappa_delta; contexts Q = {0..6, 7}; effective suits sigmahat_q = sigma_q \ kappa_delta (q in P), sigmahat_7 = kappa_delta; lead fiber L_{delta,q} = { d : ell_delta(d) = q }; leadable contexts Lambda_delta. Viewer m fixed; hidden seats h_1,h_2,h_3; hidden pool U; capacities k_s; void sets V_s; M(q) = { h_i : q in V_i }; used contexts Qused = { q : M(q) nonempty }; completed tricks j = 7 - max_s k_s; low-capacity set B and the proved already-acted-follower table B -> F(B).

A **declaration-tagged outer profile** is a tuple `(delta, k, (V_1,V_2,V_3), U)` passing the three counted necessary checks of REACH-11:

(1) capacity shape: `max k_s - min k_s <= 1` (50 labeled profiles);
(2) schedule admissibility: `|Qused| <= j`, or `|Qused| = j+1` with some q in Qused having `emptyset != M(q) subseteq F(B)`; and every used context is leadable (`Qused subseteq Lambda_delta`);
(3) lead witness: for every q in Qused, `L_{delta,q} \ U != emptyset`;
with `|U| = k_1 + k_2 + k_3` and `U subseteq D`. The total over all delta is the 64,123,542,674,901 above. (Hall feasibility of the cells `P_s = U \ Union_{q in V_s} sigmahat_q` is proved necessary for reachability but is NOT part of the counted language.)

The counting logic of the ceiling: every reachable support normal form arises from at least one legal prefix, whose actual `(delta, k, (V_s), U)` passes every proved-necessary check. So for ANY set of proved-necessary checks C, `|R| <= #{ tagged profiles passing C }`. Adding proved-necessary filters can only shrink the count; the count remains a valid ceiling **iff every filter is genuinely necessary** — a filter that rejects even one reachable support invalidates the ceiling and scores zero.

## 2. The fifth condition (yours, now licensed) and where it must be pushed

**FIFTH NECESSARY CONDITION (follower-supply obstruction — adjudicated CONFIRMED, yours).** At capacities `(6,6,6)` each hidden seat has made exactly one public play, so every used hidden void was acquired on a single follower play inside the still-open first-trick region. If exactly one hidden seat is void in a context q there, at least one other hidden seat was a co-follower in that same trick and (having no q-void) must have played a tile of sigmahat_q; the lead tile of that trick is also in sigmahat_q. These are two distinct public tiles of sigmahat_q, both necessarily outside U. Hence a singleton hidden void in context q at the `(6,6,6)` phase requires `|sigmahat_q \ U| >= 2`.

This is proved only for the `(6,6,6)` shape and singleton `M(q)`. The research half of this dispatch: generalize it. For arbitrary capacity shape k, void mask `(V_s)`, and the proved F(B) schedule structure, derive the tightest bound `|sigmahat_q \ U| >= g(...)` (and any joint multi-context conditions) that you can PROVE necessary, by accounting for: which trick(s) could have acquired each void, who must have led them (viewer or hidden, and note trick-1's leader is the bidder), how many co-followers were forced to follow, and that all such tiles are public hence outside U. Case analysis over leader identity and F(B) membership is expected. Every generalization must come with a complete necessity proof; where the honest answer is "no improvement beyond the proved case", say so and count with what is proved.

## 3. THE TASK (tiered; strongest first)

**(A) Full credit.** (i) State and prove the most general follower-supply-type necessary condition family you can establish (parameterized by k, M(q), leader cases, F(B)); (ii) compute the EXACT number of declaration-tagged outer profiles passing checks (1)-(3) PLUS all your proved conditions — at minimum the licensed `(6,6,6)`-singleton fifth condition — and report the new certified ceiling `ceil(log2 <count>)` and the resulting interval `[36, <b>]`. You may additionally include Hall feasibility of the induced cells as a filter (it is proved necessary — reachable implies nonempty fiber) if you can count the Hall-passing profiles EXACTLY; that is welcome but optional, and if included must be reported both with and without Hall so the fifth condition's contribution is isolated.

**(B) Partial credit.** The exact filtered census with ONLY the already-proved fifth condition (no generalization); or exact filtered counts for a single declaration class (by your sibling-confirmed transport theorem the seven pip-trump classes share one count, so one pip class + DT + NT suffices); or a proved closed-form/table for the eliminated-profile count by shape.

**(C) Fallback credit.** New proved-necessary conditions (a sixth, beyond follower-supply) with an exact eliminated count on a named sub-phase, even if the full census is out of reach — reported as a proved conditional ceiling on that phase, clearly scoped.

Zero credit: any filter without a complete necessity proof; any count not reproduced by your program; probabilistic or sampled estimates presented as counts; restating the standing bounds as progress.

## 4. DELIVERABLE CONTRACT

End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS` containing:

1. `FINAL ANSWER:` lines, strongest first, full decimal, no scientific notation: `FILTERED_TAGGED_OUTER = <integer>` (your headline filtered census), optionally `FILTERED_TAGGED_OUTER_WITH_HALL = <integer>`, then `CEILING = <b> bits` and `INTERVAL [36,<b>] bits`.
2. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O) that: implements rules, support algebra, and the outer checks from scratch; reproduces the anchors (a) `64,123,542,674,901` tagged total and (b) `7,124,838,074,989` per declaration; (c) recomputes your headline integer(s); (d) confirms your 002 witness is eliminated by the fifth condition while REACH-10 is eliminated by lead-witness; and (e) **necessity smoke test**: from a fixed seed, generates at least 100,000 legal Straight prefixes from scratch (random deals, random declarations, random legal plays, all depths), computes each viewer support's tagged profile, and asserts EVERY proved condition you use passes on EVERY generated profile — a single violation means your condition is false and the run must FAIL. All results as `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all pass; runtime under 6 hours on one core at 16 GB. If the exact count needs a DP too large for that budget, the program must verify every load-bearing lemma on exhaustively checkable subdomains and recompute the final integer from your proved decomposition — fully proved in the text.
3. Your proof as numbered steps, each labeled `[USES: ...]` (definitions, prior steps, or named corpus claims like REACH-11, the transport theorem, your 002 facts). The necessity proof of each filter is the load-bearing chain; an unlabeled gap invalidates from that point.
4. Any witnesses/tables as fenced JSON in the standard encoding: dominoes `[h,l]` with `h >= l`; declarations `0..6`, `"DT"`, `"NT"`; contexts `0..7` (7 = called); hidden seats `1,2,3` = viewer+1,+2,+3 clockwise.

A response whose program fails, whose integer disagrees with its own program, or whose filter rejects any machine-generated legal profile scores zero on the affected claim. An honest partial at a proved tier outranks a full claim with a gap.
