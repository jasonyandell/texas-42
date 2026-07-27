---
number: 6
slug: exact-reachable-census
channel: continuation
conversation_url: https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6
attachments:
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/10_RULES.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/docs/20_MATHEMATICAL_FOUNDATION.md
  - ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/verification/verify_minimality_and_reachability.py
deliverable: exact |R| (or exact r_pip / |R_DT| / |R_NT| / no-void slice), else a proved interval strictly inside [35,46] bits
---
This is a follow-up in the same conversation where you produced the certified disjoint family of 17,668,066,045 reachable supports for Straight Texas 42. Everything you built there — the star/module algebra, the one-context cell lemma, the meet-in-the-middle upward-closure counter, the three subfamily totals, and the outer-certificate recomputation — is still in your context and you may reuse it verbatim. The same three attachments are the source of truth; you may rely on no outside sources. This message restates every new fact it needs so it is self-contained even if the earlier turn is unavailable. Your response is adjudicated mechanically by a model holding the full corpus: your program is executed, your witnesses re-run, your proofs step-checked. Hedged or contract-violating answers score zero.

## 0. Status of your 001 result (now established at exchange tier)

Your 001 answer was ADJUDICATED CONFIRMED. The verification program ran ALL_PASS; three independent adversarial referees returned SOUND on the proof; and your floor was independently re-implemented from scratch in Rust and reproduced bit-for-bit. Therefore the following are now settled facts of the corpus, not conjectures, and you may cite them without re-proving them:

- The certified disjoint reachable family has exactly **17,668,066,045** members, decomposing as the three pairwise-disjoint subfamilies you proved:
  - no-void reachable supports: **559,316,142**
  - called-suit one-void reachable supports: **8,387,350,664**
  - natural-suit one-void reachable supports: **8,721,399,239**
- Consequently `ceil(log2 |R|) >= 35`, and with the declaration-tagged outer certificate `9 * 7,124,838,074,989 = 64,123,542,674,901 < 2^46`, the interval is **[35, 46] bits**. The corpus floor 44,352,165 and this ceiling both stand.

## 1. New facts you may now rely on

These come from three sibling investigations you could not see. Each was triple-verified: a sibling proof, our adversarial referee panel, and an independent Rust/Python reproduction. Treat them as licensed lemmas.

### 1.1 Transport theorem — the census is genuinely 3-class (adjudicated CONFIRMED)

Let `f_{t,u}` (ordered pips `t,u in P`) be the order-preserving complement transport: it maps `t -> u` and maps `P\{t}` increasingly and bijectively onto `P\{u}`, then extends endpoint-wise to dominoes and by `7 -> 7` on contexts. Then:

**THEOREM.** For every ordered pip pair `(t,u)`, `f_{t,u}` restricts to a bijection `R_t --~--> R_u`, i.e. `f_{t,u}(R_t) = R_u`; moreover `f_{u,t} = f_{t,u}^{-1}`, and the cocycle identity `f_{u,v} . f_{t,u} = f_{t,v}` holds — closed by a finite check over all `343 = 7^3` ordered triples. (Legal-prefix generation commutes with unscored transport; the only delicate point, transporting `high(d)`, is valid precisely on the uncalled domain, which is the only place the led-context rule consults it.)

Consequences you may use directly:

- All seven pip-trump images have the **same** cardinality. Write `r_pip := |R_t|` (one number, independent of `t`).
- The untagged reachable image has the exact three-term form `R = R_pip u R_DT u R_NT` where `R_pip = Union_{t in P} f_{t0,t}(R_{t0})` for any fixed base pip `t0`. This union is **not** asserted disjoint.
- The declaration-tagged disjoint census `Rtilde = Bigsqcup_delta {delta} x R_delta` has exact cardinality
  `|Rtilde| = 7 r_pip + |R_DT| + |R_NT|`.
- Overlap hook (from the transport investigation, Step 16): every one of the nine declarations shares the common no-void initial family (fix any 7-tile viewer hand `H`; under every declaration `U = D\H`, `k=(7,7,7)`, all voids empty, `P_1=P_2=P_3=U`, giving the same ternary normal form `W=U, r0=r1=r2=7, eps=empty`). Distinct `H` give distinct normal forms, so
  `|Intersection_delta R_delta| >= C(28,7) = 1,184,040`,
  and hence the rigorous sandwich
  `max(r_pip, |R_DT|, |R_NT|) <= |R| <= 7 r_pip + |R_DT| + |R_NT| - 8 * C(28,7)`.
  (The subtraction removes the eightfold overcount of the shared `a := C(28,7)` family across the nine tags. Additional overlap — among distinct pip images and between pip/DT/NT — is exactly the uncontrolled structure this dispatch asks you to pin down.)

So the reachability count reduces to **three** declaration classes plus exact cross-class overlap accounting, not nine independent analyses. The unscored-class reduction is fully licensed.

### 1.2 The four-check outer language is NOT tight; a fifth condition exists (adjudicated CONFIRMED)

The outer over-approximation (reachable capacity shape `range <= 1`; schedule-admissible void masks; lead-witness tiles outside the pool; Hall feasibility) is **not** sufficient for reachability. Explicit shallow counterexample:

**WITNESS.** Declaration `NT`, capacities `(6,6,6)`, voids `V_1 = {6}`, `V_2 = V_3 = {}`. Pool `U = X sqcup N`, `|U| = 18`, with
`X = {6:0, 6:1, 6:2, 6:3, 6:4, 6:5}` (6 tiles),
`N = {0:0, 1:0, 1:1, 2:0, 2:1, 2:2, 3:0, 3:1, 3:2, 3:3, 4:0, 4:1}` (12 tiles).
Under NT, `sigmahat_6 = sigma_6 = X sqcup {6:6}`, so the raw cells are `P_1 = U\sigma_6 = N`, `P_2 = P_3 = U`. This passes all four outer checks (capacity range 0; used context {6} is leadable and `|{6}| = 1 = j = 7-6`; lead fiber `L_{NT,6} \ U = {6:6} != empty`; Hall holds). Its normal form is `Ternary(W=U, r0=r1=r2=6, eps)` with `eps` excluding `h_1` for exactly the six tiles of `X`. Yet it is UNREACHABLE: over all legal shallow prefixes at capacities `(6,6,6)` (the only three static generators that decode to this normal form — `(pip 6, ctx 7, {h1})`, `(DT, ctx 6, {h1})`, `(NT, ctx 6, {h1})`), the exhaustive search of `3 x 141,840 = 425,520` candidate traces yields **0** realizers.

**FIFTH NECESSARY CONDITION (follower-supply obstruction).** At capacities `(6,6,6)` each hidden seat has made exactly one public play, so every used hidden void was acquired on a single follower play inside the still-open first-trick region. If exactly one hidden seat is void in a context `q` there, at least one *other* hidden seat was a co-follower in that same trick and (having no `q`-void) must have played a tile of `sigmahat_q`; the lead tile of that trick is also in `sigmahat_q`. These are two distinct public tiles of `sigmahat_q`, both necessarily outside `U`. Hence a singleton hidden void in context `q` at the `(6,6,6)` phase requires
`|sigmahat_q \ U| >= 2`.
The witness fails this: `|sigma_6 \ U| = |{6:6}| = 1`. This condition is mechanically checkable and eliminates outer profiles the four checks admit.

## 2. The task (tiered; strongest first)

You have collapsed the census to three classes and have a validated module/witness/upward-closure machine plus an exact one-context cell calculus. Now push it to an exact count, or genuinely tighten the interval. In order of value:

**(A) Full credit.** Exact `|R|`. Equivalently: exact `r_pip`, `|R_DT|`, `|R_NT|`, together with exact cross-class overlap accounting (how `R_pip`, `R_DT`, `R_NT` intersect) sufficient to resolve `|R_pip u R_DT u R_NT|`. Do it by extending your validated module/witness machinery to a complete (not merely lower-bounding) enumeration, or by a provably-exact dynamic program over the symbolic play/support DAG (Math 7.13.7) — now needing only **3** declaration classes. You must prove your state canonicalization neither merges two prefixes with different future support sets nor omits any reachable normal form, and that de-duplication across viewer hands and across the (now three) classes is exact. Report the exact integer.

**(B) Partial credit, any one of:**
- **(B1)** Exact `r_pip`, or exact `|R_DT|`, or exact `|R_NT|`, for a single class — with a proof that your enumeration for that class is complete, not just a floor.
- **(B2)** Exact cardinality of the **no-void slice**: the number of distinct reachable supports whose cells are unrestricted (`P_s = U` for all hidden `s`), across all declarations and every capacity shape with `range <= 1` (not only the shallow shapes your 559,316,142 floor already settles — you must close every deeper no-void shape where voids could have occurred but did not bite).

**(C) Fallback credit.** A proved strictly tighter interval than `[35, 46]`. Two concrete routes:
- **Floor `> 2^35`**: new certified pairwise-disjoint reachable families (beyond the 17,668,066,045 already certified) whose exact total exceeds `2^35 = 34,359,738,368`.
- **Ceiling `< 2^46`**: quantify exactly how many of the `64,123,542,674,901` declaration-tagged outer profiles are eliminated by the fifth condition of 1.2 (or any further necessary condition you prove), yielding a certified count below `2^46`. Reducing the tagged outer total below `2^45 = 35,184,372,088,832` would drop the ceiling to 45 bits.

Do not restate the already-established bounds (17,668,066,045; 44,352,165; 64,123,542,674,901; interval [35,46]) as if they were new progress. Only exact integers with complete proofs, or proved intervals, score. An honest, proved partial result outranks an unproved full claim.

## 3. DELIVERABLE CONTRACT

Same mechanical-adjudication contract as your 001 answer. End with a section titled exactly `MACHINE-CHECKABLE ARTIFACTS` containing:

1. A line `FINAL ANSWER:` followed by exactly one of, strongest first (list each achieved tier on its own line): `|R| = <integer>`; `R_PIP = <integer>` / `R_DT = <integer>` / `R_NT = <integer>`; `NO_VOID_SLICE = <integer>`; `INTERVAL [<a>,<b>] bits`. Full decimal, no scientific notation.
2. One self-contained Python 3 program (single fenced block, standard library only, deterministic, no network, no file I/O) that recomputes your claimed integer(s) from first principles — implementing the game rules and support algebra itself, not assuming them — and prints `PASS <check>` / `FAIL <check> <detail>` lines, exiting 0 iff all pass. Required internal anchor checks, at minimum: it reproduces (a) the certified floor `17,668,066,045` and its three subfamily totals `559,316,142`, `8,387,350,664`, `8,721,399,239`; (b) the corpus floor `44,352,165`; (c) the tagged outer total `64,123,542,674,901` (and, if your method touches the outer language, the per-declaration `7,124,838,074,989`); (d) your new headline integer(s) by your method; and (e) where feasible, a restricted-slice brute-force cross-check (e.g. one fixed viewer hand, or `j <= 1`) against your DP. State expected runtime and memory; the program must finish within 6 hours on one CPU core at 16 GB. If your exact method inherently needs more, the program must instead verify every load-bearing lemma on exhaustively checkable subdomains and recompute the final integer from your proved closed form or table — but then the derivation must be fully proved in the text.
3. Your proof, as numbered steps, each carrying a bracketed `[USES: ...]` label naming exactly what it appeals to (definitions, prior steps, corpus claims, or the 001/002/004 facts above). Steps appealing to unstated facts or to intuition invalidate the chain from that point.
4. Every witness or table you rely on, as explicit JSON in fenced blocks, in the 001 encoding: dominoes as two-element arrays `[h,l]` with `h >= l`; declarations as `0..6`, `"DT"`, `"NT"`; contexts as `0..7` (7 = called); hidden seats as `1,2,3` meaning viewer+1, viewer+2, viewer+3 clockwise.

A response whose program fails, whose integer disagrees with its own program, or whose proof has an unlabeled gap scores zero on the affected claim. Claims labeled honestly as partial score at their tier.
