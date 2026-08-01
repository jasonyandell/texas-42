---
number: 15
slug: constellation-lean-stage2
channel: continuation
conversation_url: https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad
status: cleared (Jason 2026-08-01 standing follow-up authorization; batch ceiling 17)
deliverable: Stage 2 Lean — suffix positions, legality, trick step, exact value by well-founded recursion; we compile under the pin and return the log
---
Stage 1 result: **GREEN. Your file compiles under the pinned toolchain and `unique_winner` is kernel-checked with zero sorries** — the 56,448-case decide went through (37 s wall for the module), and both examples check. Excellent work.

Two mechanical fixes were needed on our side, both in the same pattern — chaining two `set_option ... in` lines directly before a declaration does not parse under this toolchain ("unexpected token 'set_option'; expected 'lemma'"). We replaced the chained form at both sites with a section scope, which parses and builds:

```
section DecideChecks
-- Kernel `decide` over all declarations x tiles (and below, x contexts x tile
-- pairs: 56,448 cases); the default budgets are far too small for these
-- closed finite checks.
set_option maxHeartbeats 5000000
set_option maxRecDepth 100000
...decls...
end DecideChecks
```

Also two cosmetic style-linter notes you can ignore or preempt: the project lints `set_option maxHeartbeats` for an adjacent explanatory comment (our section comment satisfied it), and prefers `set_option ... in` with a comment for single declarations. None of your three predicted risky spots fired — `positive_key_injective`'s closed decide, the `omega` bounds, and `winner_maximal`'s `simpa [winner]` all worked exactly as written.

STAGE 2 — same rules of engagement (you write; we compile and return the verbatim log; no compilation certification asked; name your three likeliest breakage points with one-line fallbacks). One continuation file `Constellation/Suffix.lean` importing your Stage 1 namespace (assume it is available as `Constellation` from module `Texas42.ConstellationCore`; write `import Texas42.ConstellationCore` at the top). Target ≤ 350 lines:

1. **Suffix positions.** A structure for depth-k suffix play at a trick boundary: four hands as `Finset Domino` (pairwise disjoint, equal card k), a leader in `Fin 4`. Represent seats relative to nothing — absolute `Fin 4` with the partnership parity convention (0,2) vs (1,3) is fine.
2. **Mid-trick states and legality.** Whatever intermediate representation you prefer (a prefix list of plays, or a fold), with: leader freedom; follower must-follow determined by the led tile's context and the follower's remaining hand; slough freedom otherwise. Reuse Stage 1's `follows`/`ledContext`/`trickKey`.
3. **Trick step.** Resolve a completed trick with Stage 1's `winner`/`award`; remove the four tiles; winner leads the successor.
4. **Exact value.** `value : SuffixPos → Int` — both-partnerships-optimal minimax margin for the partnership of parity 0 (the (0,2) pair), by well-founded recursion on total remaining tiles (this is the delicate engineering; a fuel-indexed definition with a proved fuel-sufficiency lemma is an acceptable fallback — say which you chose and why).
5. **Proved, sorry-free target:** `value` computes — two concrete k=1 `example`s where `value` evaluates to a hand-computed margin (state the arithmetic in comments); and the lemma `value_k1_forced`: at k=1 the value equals the forced-trick margin (winner's parity decides the sign of the award).
6. **Allowed as named `sorry`s if they resist:** anything about legality-set transport or successor structure you find yourself wanting for later stages — state them as clean standalone lemmas so we can see the Stage 3 interface taking shape.

Omit still: constellations, the equivalence, hereditariness, C1 — Stage 3 and 4, after this compiles.

Same spirit as before: an honest skeleton we can compile beats a heroic guess, and your Stage 1 calibration was excellent — trust it.
