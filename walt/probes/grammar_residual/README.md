# Grammar-split probe — counted-belief Slice B (§45)

EXPLORATORY, below every evidentiary tier, cited by nothing above it. A
probe number becomes quotable only by brief amendment onto a verifier
receipt. Instrument: `grammarsplit` (`walt/src/bin/grammarsplit.rs`);
machinery: `solver::grammar` (Part III §9/§11/§12 of
`walt/math/counted_belief_sandwich_v0.1.md`, ruling CBS-A4). Record:
`run1.txt` (declared epoch in its header; field σ = Level0 { n0 = 2 };
frozen `verify_player` receipt roots).

Grammars: **G1** = {lowest-first} (one behavioral policy), **G2** =
{lowest-first, highest-first}, **G3** = {pinned level-1 continuation
[2, 2], the σ0 modeled mind, count-preservation safety}.

## Readings (run1, 2026-08-30)

1. **Root closure is cheap at tricks 5–6.** On all six exact fixtures,
   both G2 and G3 attain the exact root optimum (`root-closure … YES`),
   and every in-grammar action's verdict is `closes` or `ties` — the
   grammar optimizer finds the root before any full policy search. The
   contrast that proves the measurement discriminates: the singleton G1
   fails root closure on h4-t6 (30 vs 78) and h8-t5 (64 vs 91).

2. **The §12 boxed exclusion is realized exactly, with margin.** h8-t5
   under G3: action 0-0 has `gram=71, dev=70` and action 5-3 has
   `gram=91, dev=90` — every optimal continuation plays in-grammar at
   every still-undecided state, and the best deviating line loses exactly
   one world. This is the exact-side residual bound §45 asks for; the
   sampled route structurally cannot produce it (reading 4).

3. **The sweep's one true counterexample carries its lazy witness.**
   G1 at h8-t5, action 0-0: `free=71` vs `gram=64`, first deviation at
   depth 4 — after `0-0,3-0,6-0,6-6` the optimal line plays 5-3 where
   lowest-first's single grammar action is the other legal tile. §45's
   "first off-grammar information states in exact counterexamples,"
   discovered lazily per CBS-A4.

4. **The §8 identity, in the numbers.** The residual empirical-max upper
   and the Slice A full-class upper have byte-identical count paths and
   equal bounds (asserted in the run). Partitioning by itself tightens
   nothing on the sampled route: a continuation may deviate at a state
   the sample never reaches, so the only admissible residual count is
   the full-class optimum. Coverage is only ever a residual bound, and a
   sampled residual bound is only ever the full-class bound.

5. **Grammar room vs legal room (census).** At these depths G2 is
   saturated almost everywhere (legal sets of one or two tiles), so its
   closures are largely vacuous room-wise. G3's are not: h3-t5 action
   5-0 walks 246 undecided focal states with exactly ONE grammar action
   per state (all three sources agree) against 318 legal — and still
   ties the optimum; h8-t5 keeps 140/141 grammar room and closes with
   margin. Three-source agreement this heavy also says the sources are
   not yet diverse — a diverse fourth source is the cheapest way to
   widen `G(I)` if a counterexample ever demands it.

6. **Cost.** G1/G2 exact splits run 1 µs–10 ms per action; G3 splits
   ≤ ~105 ms (level-1 materialization dominates, as everywhere);
   sampled splits at prefix 64 run ~100–260 µs per action.

## Boundaries

- No play-strength claims; verdicts are relative to the DECLARED field
  and the decided-truncation quotient (deviations after the outcome is
  decided are value-irrelevant and stay on the grammar side).
- The opening root is out of scope for this walk by design: exact and
  sampled grammar splits live on the Slice A world-tree; the
  opening-scale grammar solve is §48's factorized route (Slice E), not
  a bigger tree.
- Trick-5/6 closure says nothing about earlier tricks, where legal sets
  are wide and the census will not saturate; the fixtures simply have no
  affordable earlier exact roots on this walk (same boundary as above).
