---
number: 13
slug: constellation-lean-stage1
channel: continuation
conversation_url: https://chatgpt.com/c/6a6da27e-3cfc-83e8-9a7e-e01d0ebdecad
status: cleared (Jason 2026-08-01 standing follow-up authorization; batch ceiling 17)
deliverable: Stage 1 Lean file only — core definitions through trick resolution plus the corrected unique-winner theorem; we compile locally under the pin and return the build log next round
---
Your refusal was the right call and both blocking points are accepted. Your mathematical correction is also accepted and adopted: the mandatory theorem is uniqueness of the winning (maximal-key) tile in a completed legal trick — tier-zero keys tie by design in the foundation — not injectivity of trick keys on dominoes. That phrasing error was ours; thank you for reading the sources more carefully than our contract did.

New contract, staged exactly as you proposed, with an honest division of labor: you write Lean; WE compile it under the pinned toolchain (leanprover/lean4:v4.33.0-rc1, mathlib rev v4.33.0-rc1) on our machine and paste you the verbatim build log next round. You are NOT asked to certify compilation — only to write code you believe compiles, and to name the three spots you consider most likely to break, each with a one-line fallback.

STAGE 1 ONLY — one file `Constellation/Core.lean`, target ≤ 300 lines, importing mathlib only:

1. Definitions: `Pip`; `Domino` as the 28-element universe (your choice of representation — ordered pairs with h ≥ l, a subtype, or a quotient — one comment justifying the choice for downstream `decide`-friendliness); `countPoints`; `Declaration` (seven pip trumps, doubles-trump, no-trump); `calledSet`; `ledContext`; `follows`; `TrickKey` with its lexicographic order (tier 2 called-and-powered, tier 1 uncalled follower, tier 0 slough; ranks: double-under-DT = its pip, double otherwise = TOP, mixed = pip sum; NT has no tier 2).
2. Trick resolution for a completed 4-play trick: `winner` and `award` (1 + count points of the four tiles).
3. THEOREM (the sorry-free target of this stage): `unique_winner`, correctly phrased — in any trick of four DISTINCT dominoes, under any declaration, in the context led by the first play, there is a unique maximal trick key among the four plays, so `winner` is well-defined. If one helper lemma resists (e.g. a decidability or Finset.max' argument), leave it as a single named `sorry` and say what closing it needs.
4. Two `example`s evaluated concretely (no `native_decide`): one pip-trump trick resolved end to end; one no-trump trick showing nothing reaches tier 2.

Omit entirely (do not even `sorry` them — they are later stages): suffix positions, the value recursion, constellations, equivalence, hereditariness, C1.

Reply with the single fenced code block plus the three risky spots. If 300 lines is too tight for a sorry-free `unique_winner`, shrink the proof to a `sorry`d statement and say so plainly — an honest skeleton we can compile beats a heroic guess. Loose ends welcome; we iterate.
