# Exchange protocol: Claude ↔ ChatGPT 5.6 Pro

ChatGPT 5.6 Pro has no API. This directory is the courier channel.

## Loop (automated via `buddy`)

1. A dispatch author writes a self-contained prompt to `outbox/NNN-<slug>.md`.
   Each prompt inlines every definition it needs — 5.6 Pro sees nothing
   outside the pasted text. No "see the spec" references. When the file is
   complete, the author creates an empty marker `outbox/NNN-<slug>.ready`.
2. The `buddy` automation agent watches `outbox/`, submits each ready dispatch
   to ChatGPT 5.6 Pro via browser automation (new chat at https://chatgpt.com/,
   model picker set to Intelligence → **Pro**), records the conversation URL,
   and polls until the response completes (expect 1–2 h, timeout 3 h).
3. buddy harvests the full response to `inbox/NNN-<slug>.md` with a metadata
   header (conversation URL, submitted/harvested timestamps), and updates the
   ledger below.
4. Claude adjudicates every inbox file against the foundation before anything
   enters the wiki: witnesses re-run, programs executed, proofs step-checked.

Dispatch file format: YAML frontmatter (`number`, `slug`,
`channel: new-chat | continuation`, `deliverable:` one line), then the body.
buddy pastes the body only, verbatim. `channel: continuation` means post into
the standing conversation
https://chatgpt.com/c/6a64ccec-2328-83ea-b0d1-917f487297a2 instead of a new chat.

## Dispatch quota

**There is no lifetime cap, and no fixed total.** Dispatches are authorized by
Jason **in batches, each batch's quota agreed up front** — monthly pacing,
cleared per batch. **Never submit without Jason's explicit go for the batch you
are sending in.** (The fixed-number framing was retired 2026-08-01 as wrong;
any doc still stating a lifetime total is stale.)

Two numbers, and they are not the same kind of thing:

- **`submission_count.txt`** — the running count of dispatches **ever sent**.
  One line, one integer; buddy increments it on each confirmed send. It is a
  tally, never a ceiling, and it is never reset when a batch closes.
- **`HARD_CAP`** in [`../automation/submit.mjs`](../automation/submit.mjs) —
  the **current batch's ceiling for the automated path only**. `submit.mjs`
  refuses to send once the count reaches it. Raise it only for a batch Jason
  has explicitly authorized, and only with his go.

The count can legitimately exceed `HARD_CAP`, because dispatches Jason
hand-ferries himself are counted but never pass through the automation. As of
2026-08-24: **count 18, automated batch ceiling 17** — dispatches 016–018 were
hand-ferried by Jason (cleared by him each time, outside the automation,
counted here). `count >= HARD_CAP` therefore means *the automated path is
closed pending a new authorized batch* — it never means the channel is spent.

A send only counts once visually confirmed submitted; a harness failure before
the message leaves the composer does not count, but buddy must verify in the UI
before retrying.

## Prompt discipline

Every outbox prompt must be an *adversarial task with a checkable deliverable*,
never "review this":

- "Construct a counterexample to X, or prove no counterexample exists."
- "Here is a claimed proof of Y. Find the first incorrect step, or certify
  each step."
- "Compute Z exactly and provide a certificate Claude can verify mechanically."

Deliverables must be machine-checkable where possible: explicit witnesses,
Python that re-derives the number, Lean statements, exact fractions — so the
answer can be verified here without trusting the model.

## Informal captures (`informal/`)

`informal/` holds **informal captures** — ChatGPT threads conducted outside the
courier protocol. They are not dispatches: no number, no `outbox/` prompt, no
adversarial deliverable contract, **no consumption of the dispatch count**, and
no adjudication. They are named `YYYY-MM-DD-<slug>.md` (never `NNN-`), and a
capture may be accompanied by a `.REVIEW.md` reading memo.

Tier: **UNADJUDICATED, exploratory** — below every tier on
[wiki/Home.md](../wiki/Home.md#evidentiary-tiers--never-promoted-never-blurred),
at or under `wiki/ideas.md`. Nothing in a capture is quotable as a result, and a
review memo inherits the capture's tier rather than raising it. An idea from a
capture enters the evidentiary layers only the normal way: as a brief with named
invariants and receipt rows, or as a numbered adversarial dispatch below.

Numbered inbox responses that turn out to be informal in character stay in
`inbox/` under their number (x:014 is the precedent) — `informal/` is for threads
that never had a number to begin with.

## Status ledger

| # | slug | dispatched | response | adjudicated | outcome |
|---|------|-----------|----------|-------------|---------|
| 001 | reachable-support-cardinality | 2026-07-27T05:07:18Z [conv](https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6) | harvested 2026-07-27T13:02:58Z | CONFIRMED (ALL_PASS 15.9s; 3/3 SOUND) | certified disjoint family of 17,668,066,045 reachable supports → interval [35,46] bits (was [26,46]); fallback tiers [34,46] and ≥30 bits machine-hardened |
| 002 | outer-language-tightness | 2026-07-27T05:09:06Z [conv](https://chatgpt.com/c/6a66e7f0-57cc-83ea-b6c8-eab6080b8b76) | harvested 2026-07-27T13:01:36Z | CONFIRMED (ALL_PASS 0.9s; 3/3 SOUND) | outer language NOT tight: (NT,(6,6,6),V1={6}) passes all 4 outer checks yet unreachable (450 generators, 425,520 traces, 0 realizers); new 5th necessary condition (follower-supply obstruction) |
| 003 | kernel-vs-future-quotient | 2026-07-27T05:09:30Z [conv](https://chatgpt.com/c/6a66e808-a6dc-83ea-8319-5c4bfca8e863) | harvested 2026-07-27T13:03:10Z | CONFIRMED (ALL_PASS 0.4s; 3/3 SOUND) | OPEN-01 resolved: COLLAPSE — reduced kernel strictly finer than future-equivalence quotient (r=7 vs r=6 witness; dead-cut lemma); SHA provenance blemish non-load-bearing |
| 004 | transport-reachability-commutation | 2026-07-27T05:09:54Z [conv](https://chatgpt.com/c/6a66e821-29c8-83ea-9f6f-4be358e6b9f7) | harvested 2026-07-27T13:03:23Z | CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND) | f_{t,u}(R_t)=R_u proved: reachable census collapses 9→3 declaration classes; Step-15 quotient corollary CONDITIONAL on one-line cocycle lemma (closable locally) |
| 005 | census-integer-audit | 2026-07-27T05:10:14Z [conv](https://chatgpt.com/c/6a66e834-e388-83ea-99e4-3186c6632c50) | harvested 2026-07-27T13:03:35Z | CONFIRMED (19/19 PASS 13s; 3/3 SOUND) | all 19 load-bearing census integers independently reproduced; new Burnside decomposition (136,514 / 2,156 / 35 → 23,842) added to corpus knowledge |
| 007 | fifth-condition-ceiling | 2026-07-27T18:15:07Z [conv](https://chatgpt.com/c/6a66e7f0-57cc-83ea-b6c8-eab6080b8b76) (continuation of 002) | harvested 2026-07-27T19:01:13Z | CONFIRMED (17/17 PASS 44.1s; 3/3 SOUND) | first ceiling movement: filtered tagged outer census 33,297,009,347,414 < 2^45 ⇒ ceiling 45 bits, interval [36,45]; capacity-only fallback 33,737,166,807,767 also < 2^45, so 45 survives even without the temporal rule; ~986k legal prefixes, zero over-rejections |
| 008 | no-void-exact-census | 2026-07-27T18:15:40Z [conv](https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6) (continuation of 001/006; post-send verifier false-negative bodyOk — turn confirmed present exactly once by DOM check) | harvested 2026-07-27T20:01:49Z | CONFIRMED (ALL_PASS 38/38 71.8s; panel 2/3 SOUND + 1 UNVERIFIABLE-no-defect) | no-void slice SATURATED: exact census 624,892,870 = Σ over the 50 range-≤1 profiles of C(28,Σk); 001's 559,316,142 was a grammar-family undercount (+65,576,728); derived combined floor 36,978,961,138 (interval [36,45] unchanged) |
| 006 | exact-reachable-census | 2026-07-27T14:53:19Z [conv](https://chatgpt.com/c/6a66e786-2ac0-83ea-ade0-dff707fae5e6) (continuation of 001; double-sent, one answer) | harvested 2026-07-27T17:40:53Z | CONFIRMED (16/16 PASS 17.3s; 3/3 SOUND) | new disjoint two-void-context family of 19,245,318,365 reachable supports; combined floor 36,913,384,410 > 2^35 → interval [36,46] bits (was [35,46]); same tier caveat as REACH-17 (3,114 representative replays + referee-replayed generalization, not end-to-end); exact census still open |
| 009 | constellation-suffix-factorization | 2026-08-01T07:33:58Z [conv](https://chatgpt.com/c/6a6da164-3b7c-83e8-9dcb-fb6477903bd3) | harvested 2026-08-01T08:18:20Z | **PARTIAL** — proof chain unbroken 3/3, SECONDARY counterexample CONFIRMED 3/3 (two independent re-verifications disjoint from the response backtracker); deliverables (c)/(d) vacuous (2↔3 pip-transport monoculture, undisclosed in prose; independent nontrivial k2 agreements = 5, zero DT/NT solved); deliverable (b) classes=19,329 non-invariant (corpus 15,680 reproduced under opponent-swap pooling); program re-run ALL_PASS 16.3s | C1 suffix factorization (constellation = declaration-blind relational key): bisimulation obligations or counterexample pair; secondary: backward commutation. Cleared by Jason 2026-08-01 (up to 8 requests Aug 1; monthly-pacing quota, fixed-cap doc framing slated for removal) |
| 010 | constellation-realizability-reachability | 2026-08-01T07:38:09Z [conv](https://chatgpt.com/c/6a6da260-2700-83e8-96fa-a82c711b9965) | harvested 2026-08-01T08:43:32Z | **CONFIRMED** (31,830 PASS / 0 FAIL ~19s; 3/3 SOUND-high; all 31,197 witnesses referee-replayed through the corpus ingest verifier, 0 failures; 31,197 = 2·15,680 − 163 reconciles the conventions and retires 009's 19,329) — caveats verbatim in claim-ledger: legal-play scope no REACH-* impact; outcome-constancy quotable from adjudication re-run only; 0 NT / 0 δ=3 exhaustive witnesses, class granularity only | R1: every realizable last-trick constellation reachable from a legal 28-tile hand? Exact realizable census + constructive algorithm or impossibility certificates |
| 013 | constellation-lean-stage1 | 2026-08-01T14:28:56Z [conv](https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad) (continuation of 011) | harvested 2026-08-01T15:10:59Z | **STAGE 1 GREEN**: 278-line Core.lean; `lake build` clean under the pin after two mechanical local fixes (set_option-in chaining → section scope); `unique_winner` kernel-checked ZERO sorries incl. 56,448-case decide + 2 examples; wired into root Texas42.lean; iteration continues, no panel | Stage 1 only: core defs through trick resolution + corrected unique_winner; division of labor — Pro writes, we lake-build under the pin and return the log; loose-but-directed iteration per Jason's 2026-08-01 policy |
| 014 | constellation-informal-take | 2026-08-01T14:49:31Z [conv](https://chatgpt.com/c/6a6e077a-1850-83e8-abb4-2790789e5de4) | harvested 2026-08-01T15:21:06Z | INFORMAL (unadjudicated, exploratory capture only): 'you found the intrinsic geometry of the game' — constellation = intrinsic relational type, realization = embedding; backward failure = non-surjective restriction of embedding spaces / extension types ('forgetting the witness is harmless under deletion, not under extension'); proposes extension-type census on the 15,680 (e(c), lift-cover κ(c)), skeleton–constellation span + bipartite-component census, salience as a filtration, beliefs as measures on the realization bundle, promotion via upper sets, trump's monotone disappearance law Δ(c)⊆Δ(c′) | INFORMAL exploratory pass (Jason's loose-but-directed mode): the constellation take + all adjudicated results shared, open threads offered, improvisation invited; no deliverable contract, NO adjudication planned — insights land at exploratory tier only |
| 015 | constellation-lean-stage2 | 2026-08-01T15:15:42Z [conv](https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad) (continuation of 011/013; post-send verifier false-negative bodyOk, 008 precedent — turn confirmed present exactly once by DOM check) | harvested 2026-08-01T15:58:25Z | **STAGE 2 GREEN after local repair**: suffix positions, mid-trick legality, fuel-indexed exact minimax, value_k1_forced; kernel EVALUATES minimax in two decide examples (-11, 16). Local fixes ours: `prefix` reserved-keyword rename → `pending`; step_remaining proof restructured (fin_cases → pointwise by_cases + 4-way omega split); LinearOrder Domino lift instance; SuffixPos namespace for dot notation. Zero sorries. Iteration continues, no panel | Stage 2: suffix positions, legality, trick step, exact value by WF recursion, value_k1_forced; build log for Stage 1 returned (GREEN, two mechanical set_option fixes shown) |
| 011 | constellation-lean-formalization | 2026-08-01T07:38:40Z [conv](https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad) | harvested 2026-08-01T14:27:16Z (~6.8h) | honest refusal — would not fabricate a compilation claim (contract worked as designed); proposed the staged build; CAUGHT A DISPATCH SPEC ERROR: tier-zero trick keys tie by design, so the theorem is unique WINNER, not key injectivity — correction accepted. No panel per iteration policy; follow-up 013 sent into the same conversation | lake-buildable Lean 4 (mathlib v4.33.0-rc1) file: suffix game, value by WF recursion, Con/~, C1 stated; unique_winner + k=1 base + hereditariness sorry-free mandatory |
| 012 | carrier-staircase-burnside | 2026-08-01T07:39:02Z [conv](https://chatgpt.com/c/6a6da294-5c10-83e8-b8b0-3cb643e722da) | harvested 2026-08-01T08:34:07Z | **CONFIRMED** (14/14 PASS 18.95s; 3/3 SOUND-high; b8 triply confirmed by referee-independent routes; 486/4,767 match rob exactly) — caveats verbatim in claim-ledger: fibered stabilizer-Burnside substitution (disclosed), two tautological PASS lines, direct anchors only j∈{0..5,27,28}, OEIS unverified; rule-free carrier skeleton, feasibility not reachability | exact orbit counts of j-edge subgraphs of K7-with-loops for all j, pure + count-labeled, cycle index vs direct canonicalization; anchors a_4/b_4/b_8 + role-decorated count |
| 016 | cheap-upper-witness-handoff | 2026-08-14 (cleared by Jason "ship it please", commit 6dcd7fd; hand-ferried — Jason pasted the outbox body into the app himself, so no conv URL; informal x:014 colleague register) | Pro's note "Decision-Sparse Exact Solving: Nonanticipativity Taxes and a Compositional Plan Calculus v0.1" uploaded by Jason and harvested same day as `inbox/016-decision-sparse-nonanticipativity-taxes.md` | **ADJUDICATED INTO WALT'S EXPLORATORY TIER** (walt-math-10, same day; walt-tier adjudication — no adversary panel, never the CONFIRMED pipeline): first-layer mathematics confirmed (fusion-gap identity, binary tax formula, fusion cores ≤ \|A(I)\|, one-stage penalty dual); four repairs filed in walt/CENSUS-RULINGS.md (Lemma FT-arrive names a silent hypothesis, Lemma FT-trunc shortens the ladder, Prop FT-flat replaces §10.2, Lemma FT-post defuses the uniform-posterior trap); its Experiment 15.1 built and run the same day (S6k `fusion_tax`: first gluing-cut closure at h6, eleven of twelve pairs NOT closed with tied shortfall = Δ² exactly) | first-rung reveal-delay/gluing mathematics for the decision-sparse program — the language of cuts, cheap upper witnesses, tax sparsity |
| 017 | second-rung-gluing-handoff | 2026-08-14 (staged commit 0f4acf5, cleared by Jason; hand-ferried from the GitHub outbox page by phone; colleague register) | Pro's note "Second-Rung Gluing: Policy-Dependent Occupancies, the Slack–Tax Interchange Law, and Exact Martingale Penalties v0.1" uploaded by Jason and harvested same day as `inbox/017-second-rung-gluing.md` | **ADJUDICATED INTO WALT'S EXPLORATORY TIER, ACCEPTED IN LARGE PART** (walt-math-11, same day, SR-A1..A36; walt-tier — no panel): slack–tax interchange law Δ² = Σ_I min_b(s+d) CONFIRMED; multistage martingale dual CONFIRMED (discharges FT-A13(iv)'s validity half); repairs — unnamed free-product hypothesis (Lemma SR-coord), wrong justification on §6.1 (Prop SR-sep), §12.1 verifier proved vacuous (Prop SR-taut; REJECTED as a receipt), §1.4's silent weakening of Lemma FT-post declined; the SR depth-two probe built and run the same day: ten receipts HELD at all four units (h2 and h9, arm 2 completed), Δ² reconstructed exactly at both coordinates, escape actions PRESENT (36/330 at h2, 498/1320 at h9 — first measured policy adjustment) | the four-part second-rung ask: exact depth-2 layer under policy-dependent arrival, multistage penalty dual formalised, depth-2 regret events, grade-against-table challenge |
| 018 | fee-correlation-update | 2026-08-14 (staged, cleared by Jason — "have Walt math send collegial correspondence... seeking fresh eyed perspective from our valued teammate"; hand-ferried from the GitHub outbox page; colleague register, correspondence not adversarial — no machine-checkable deliverable) | — awaiting Pro's reply | — staged; drafted by walt-math-11 after the FC chapter close: reports both prior notes' outcomes (h6 gluing-cut closure with exact surpluses; interchange law + martingale dual CONFIRMED; three repairs stated plainly), the feature-fee arc (exact convex breakpoint solve, shared-θ 7095382833/7104861535 of oracle, the exact-zero coordinate), Prop FC-width (subgradient width = mass-weighted spread across the clairvoyant tie; tie multiplicity = pre-fee screening statistic) | the fresh-eyes ask: when the fee route is structurally unavailable (wide ties), what object carries the lower-witness burden — covering/fractional-covering dual over the core hypergraph with the fee as rank-one case?; plus per-seed survey measurables and the conditional-moment gap blocking trick 1 |

Adjudication run: 2026-07-27, workflow wf_775fe0ec (30 agents; programs executed unmodified from exchange/adjudication/programs/; per-response referee panel).

Incident 2026-07-27: 006 was double-sent ~28 s apart by two operator agents
during an ownership handoff race (both sends verified in the 001 conversation
DOM; one dispatch unit wasted; the duplicate turn is content-harmless — Pro
answers once). Root causes: the outgoing operator
submitted before its stand-down arrived, and the incoming operator sent without
re-reading the count and re-scanning the target conversation immediately before
send. Rule added to the skill: **in the same breath as any send — re-read
submission_count.txt AND scan the target conversation for an existing identical
turn; abort on either signal.** Single-operator-at-a-time is now mandatory: never
two agents with browser/submit authority alive simultaneously, even mid-handoff.
