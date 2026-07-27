---
number: 8
slug: no-void-exact-census
channel: continuation
conversation_url: https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_minimality_and_reachability.py
deliverable: exact cardinality of the no-void reachable slice with a two-sided completeness proof, else a proved interval strictly inside [36,46]
---
This is a follow-up in the same conversation as your 001 and 006 answers. Your 006 answer was ADJUDICATED CONFIRMED: your program ran 16/16 PASS in 17.3 seconds, all anchors reproduced, and three independent adversarial referees returned SOUND — including independent verification of the disjointness structure (all 519 retained classes carry two distinct proper holder categories with nonempty N, impossible for any 001-family support) and a referee replay of your JSON witness as a legal two-void deal. Therefore the following are settled facts of the corpus you may cite without re-proving: the two-void-context family has exactly **19,245,318,365** members (phase blocks 9,471,489,300 / 4,501,822,325 / 3,249,441,195 / 2,022,565,545); it is disjoint from your 001 family of 17,668,066,045; the combined certified floor is **36,913,384,410 > 2^35**; the proved interval is now **[36, 46] bits**. Everything you built in 001 and 006 — the star/module algebra, the one-context cell lemma, the upward-closure counter, the two-void-context construction — is still in your context and may be reused verbatim. The same three attachments are the source of truth; rely on no outside sources. This message restates what it needs so it is self-contained even if earlier turns are unavailable; where they differ, the attachments govern. Your response is adjudicated mechanically: program executed, witnesses re-run, proofs step-checked. Hedged or contract-violating answers score zero.

## 1. The target: the no-void slice, exactly

In 006 you were offered tier (B2) — the exact cardinality of the no-void slice — and (correctly, honestly) chose the floor route instead. This dispatch makes (B2) the sole headline. It is the most closable exact stratum of the census: no void-schedule machinery, no lead-witness machinery, no follower-supply machinery — none of it bites when no void has been revealed.

**Definition.** A reachable support normal form is in the **no-void slice** iff it is produced by some legal Straight prefix (any declaration, any viewer seat/hand) in which no hidden seat has publicly revealed a void: `V_1 = V_2 = V_3 = emptyset`. Then the cells are unrestricted — `P_s = U` for all hidden s — and the normal form is the full-ambiguity ternary form determined by the pair `(U, (k_1,k_2,k_3))` alone (pool plus the ordered, viewer-relative capacity triple). In particular the normal form does not mention the declaration, so the untagged slice deduplicates across declarations for free: the count is
`NO_VOID_SLICE = #{ (U, (k_1,k_2,k_3)) : max k_s - min k_s <= 1, |U| = k_1+k_2+k_3, and some legal prefix under SOME declaration realizes pool U with capacities (k_1,k_2,k_3) and no revealed hidden void }`.
Two subtleties you must treat, not wave at: (i) "no revealed void" constrains the whole prefix — every hidden play must have been a legal follow (or a legal lead / legal slough that does not create a recorded void), which constrains which (U, k) are realizable; (ii) k is the ordered viewer-relative triple, so seat asymmetry matters (your 001 no-void family already respected this).

**What is already settled vs. open.** Your 001 result certifies exactly **559,316,142** no-void reachable supports at the shallow capacity shapes it covered, and this number is confirmed at the exchange tier. The open part is every deeper shape: as j grows, more tiles are public and the constraint "some legal prefix reaches this exact (U,k) with zero revealed voids" interacts with what the complement of U must contain (the viewer hand plus a multiset of completed/partial tricks that can be legally scheduled void-free). You must close ALL capacity shapes with range <= 1, down to (0,0,0)/(0,0,1)-region endgames.

## 2. THE TASK (tiered; strongest first)

**(A) Full credit.** The exact integer `NO_VOID_SLICE`, with a two-sided completeness proof:
- (upper/enumeration side) a characterization or DP that counts exactly the (U,k) pairs admitting a void-free realization, with a proof that it neither overcounts (every counted pair gets an explicit or explicitly-constructible legal void-free realization) nor undercounts (every reachable no-void support is counted);
- (constructive side) an explicit realization algorithm: given any counted (U,k), it outputs a complete deal, declaration, and legal actor-attributed prefix with zero revealed voids whose viewer support is exactly (U,k) — this is the same replay-witness standard your 001/006 templates met.
If a clean closed form exists (e.g. all capacity-consistent (U,k) with mild boundary conditions are void-free reachable), prove both directions and say exactly which (U,k) fail and why.

**(B) Partial credit.** Exact no-void counts for a proved subset of capacity shapes beyond the 001-settled shallow region (state precisely which shapes are closed and which remain open, with per-shape integers); or the exact no-void count for one declaration class with tagged (declaration-aware) accounting if the untagged dedup resists proof.

**(C) Fallback credit.** A proved interval strictly tighter than [36, 46]: e.g. the no-void slice plus your certified one- and two-void families assembled into a larger certified pairwise-disjoint floor (state and prove disjointness against BOTH prior families), or any new certified floor exceeding 2^36 = 68,719,476,736 (which would give [37,46]).

Do not restate the standing bounds (36,913,384,410; 559,316,142; 44,352,165; 64,123,542,674,901; [36,46]) as if they were new progress. Only exact integers with complete proofs, or proved intervals, score. An honest proved partial outranks an unproved full claim.

## 3. DELIVERABLE CONTRACT

Same mechanical contract as your 001/006 answers. End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS` containing:

1. `FINAL ANSWER:` lines, strongest first, each on its own line, full decimal, no scientific notation: `NO_VOID_SLICE = <integer>`; or per-shape lines `NO_VOID[<shape>] = <integer>` with an explicit list of closed shapes; and if tier (C) applies, `FLOOR = <integer>` and `INTERVAL [<a>,46] bits`.
2. One self-contained Python 3 program (single fenced block, stdlib only, deterministic, no network/file I/O) that recomputes your claimed integer(s) from first principles — implementing the rules and support algebra itself — and prints `PASS <check>` / `FAIL <check> <detail>` lines, exit 0 iff all pass. Required internal anchors: (a) reproduce `559,316,142` (your certified shallow no-void total) as a special case of your general method; (b) reproduce the corpus floor `44,352,165`; (c) recompute your headline integer(s); (d) **brute-force cross-check**: for at least one fixed viewer hand and every declaration class representative (one pip trump, DT, NT), exhaustively enumerate ALL legal void-free prefixes to depth j <= 2 from scratch, collect the exact set of realized (U,k), and confirm it matches your characterization/DP restricted to that slice — print both cardinalities; (e) **realization spot-check**: for at least 1,000 deterministically-strided counted (U,k) pairs across ALL capacity shapes (stride stated, must include the deepest shapes), run your realization algorithm and replay its output through a from-scratch legality checker, confirming zero revealed voids and exact support match. Runtime under 6 hours on one core at 16 GB; if the exact method needs more, verify every load-bearing lemma on exhaustively checkable subdomains and derive the final integer from the proved decomposition — fully proved in the text.
3. Your proof as numbered steps, each labeled `[USES: ...]` (definitions, prior steps, corpus claims, or your adjudicated 001/004/006 facts). Both completeness directions are load-bearing steps; an unlabeled gap invalidates the chain from that point.
4. Witnesses/tables as fenced JSON in the 001 encoding: dominoes `[h,l]` with `h >= l`; declarations `0..6`, `"DT"`, `"NT"`; contexts `0..7`; hidden seats `1,2,3` = viewer+1,+2,+3 clockwise.

A response whose program fails, whose integer disagrees with its own program, or whose realization algorithm fails any spot-check scores zero on the affected claim. Claims labeled honestly as partial score at their tier.
