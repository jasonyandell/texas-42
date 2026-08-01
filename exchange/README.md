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

## Hard budget

**Maximum 10 submissions total, ever, across all sessions** — tracked in
`submission_count.txt` (one line, the integer count; buddy increments it
before each send). A send only counts once visually confirmed submitted;
a harness failure before the message leaves the composer does not count,
but buddy must verify in the UI before retrying.

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
| 009 | constellation-suffix-factorization | 2026-08-01T07:33:58Z [conv](https://chatgpt.com/c/6a6da164-3b7c-83e8-9dcb-fb6477903bd3) | harvested 2026-08-01T08:18:20Z | program re-run ALL-PASS 16.4s exit 0 (8/8 checks, output verbatim); panel pending | C1 suffix factorization (constellation = declaration-blind relational key): bisimulation obligations or counterexample pair; secondary: backward commutation. Cleared by Jason 2026-08-01 (up to 8 requests Aug 1; monthly-pacing quota, fixed-cap doc framing slated for removal) |
| 010 | constellation-realizability-reachability | 2026-08-01T07:38:09Z [conv](https://chatgpt.com/c/6a6da260-2700-83e8-96fa-a82c711b9965) | pending | — | R1: every realizable last-trick constellation reachable from a legal 28-tile hand? Exact realizable census + constructive algorithm or impossibility certificates |
| 011 | constellation-lean-formalization | 2026-08-01T07:38:40Z [conv](https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad) | pending | — | lake-buildable Lean 4 (mathlib v4.33.0-rc1) file: suffix game, value by WF recursion, Con/~, C1 stated; unique_winner + k=1 base + hereditariness sorry-free mandatory |
| 012 | carrier-staircase-burnside | 2026-08-01T07:39:02Z [conv](https://chatgpt.com/c/6a6da294-5c10-83e8-b8b0-3cb643e722da) | harvested 2026-08-01T08:34:07Z | program re-run exit 0, 21s; a4=37 b4=486 b8=126657 role-dec=4767 ALL match corpus-side frozen values; panel pending | exact orbit counts of j-edge subgraphs of K7-with-loops for all j, pure + count-labeled, cycle index vs direct canonicalization; anchors a_4/b_4/b_8 + role-decorated count |

Adjudication run: 2026-07-27, workflow wf_775fe0ec (30 agents; programs executed unmodified from exchange/adjudication/programs/; per-response referee panel).

**Budget after 008: 9/10 (1 remaining).** Incident 2026-07-27: 006 was double-sent
~28 s apart by two operator agents during an ownership handoff race (both sends
verified in the 001 conversation DOM; one lifetime unit wasted; the duplicate
turn is content-harmless — Pro answers once). Root causes: the outgoing operator
submitted before its stand-down arrived, and the incoming operator sent without
re-reading the count and re-scanning the target conversation immediately before
send. Rule added to the skill: **in the same breath as any send — re-read
submission_count.txt AND scan the target conversation for an existing identical
turn; abort on either signal.** Single-operator-at-a-time is now mandatory: never
two agents with browser/submit authority alive simultaneously, even mid-handoff.
