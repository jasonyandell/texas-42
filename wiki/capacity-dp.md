# Hall Feasibility, Exact Counting, and Exact Uniform Sampling

[Home](Home.md) · owns: Hall feasibility, exact counting, the uniform sampler ·
Sources: both packages Math §7.7–7.8, **rec Math §7.7.1**. Related:
[support-fiber](support-fiber.md), [minimal-support-normal-form](minimal-support-normal-form.md).

## Feasibility = capacitated Hall

`Φ(C) ≠ ∅` iff `|U| = Σ k_s` and for every seat subset `R`:
`|⋃_{s∈R} P_s| ≥ Σ_{s∈R} k_s` [Theorem — proved via slot-expanded Hall, Math §7.7,
CELL-09]. Three hidden seats ⇒ seven subset checks. Verified against direct
enumeration on 66,968 abstract tiny systems [Theorem — exhaustive finite verification,
CELL-10].

**rec-only framing [Corollary/Synthesis, rec Math §7.7.1, CELL-09A]**: a cell system
*is* a finite bipartite capacitated b-matching problem — Hall = matching feasibility,
conditioning = forced-edge contraction, marginal support = edges in some complete
matching, the SCC compiler = the alternating-cycle characterization. This licenses
formalizing the support theory generically before specializing to three seats.

## Exact counting without enumeration

Three equivalent exact counts [Theorems — proved, Math §7.8, CELL-10A/B/H]:

1. **Generating-function coefficient**: `|Φ| = [∏ x_s^{k_s}] ∏_{d∈U} (Σ_{s: d∈P_s} x_s)`.
2. **Deletion recurrence**: partition by the holder of any chosen tile.
3. **Occupancy-vector DP**: process tiles in any order, tracking per-seat occupancy.

Native bounds (3 seats, `|U| ≤ 21`, `k_s ≤ 7`) [Theorem — proved, CELL-10I; exhausted
over all 512 capacity triples, CELL-10I1]:

- ≤ **512** occupancy states over the whole run; ≤ **1,533** candidate-holder checks;
  ≤ **1,344** capacity-eligible updates; ≤ **48** live states per layer
  (layer sizes are coefficients of `(1+x+…+x⁷)³`);
- count ≤ `21!/(7!)³` = **399,072,960**.

So *unrestricted* native counting is trivially cheap forever, even though extensional
enumeration can hit ~4×10⁸ worlds. Arbitrary *predicate-restricted* counting is a
different computational problem with its own boundary [OPEN-06].

## Support selects no probability law

A fiber with ≥2 worlds admits many full-support measures [Theorem — proved, CELL-10D]:
"possible ≠ equally likely" is structurally enforced. Uniformity is a *separately
selected* law (or a consequence of the uniform-deal physics-only posterior,
[belief-vs-support](belief-vs-support.md)).

**Exact count-ratio sampler** [Theorem — proved, CELL-10E/F]: once uniform is selected,
`Pr(d ∈ H_s) = N(C^{d→s})/N(C)`, and sequentially choosing each tile's holder with
those integer weights yields an *exactly* uniform world — telescoping product,
no fiber materialization, needs an exact rational choice source. 22,620 world
probabilities checked on the tiny corpus [CELL-10G].

## Local allowance ≠ marginal possibility

A tile can sit in `P_s` yet appear at seat `s` in **no** conserved world
(`U={a,b}`, `P_0={a,b}`, `P_1={a}`, capacities 1,1: the only world gives `b` to 0,
`a` to 1) [Constructed counterexample, Math §7.9, CELL-10J]. The exact criterion:
`d ∈ P_s*` iff the forced successor `C^{d→s}` is Hall-feasible [Theorem — proved,
CELL-10K] — at most `3|U| ≤ 63` Hall tests, or one feasible assignment + one SCC pass
(see [minimal-support-normal-form](minimal-support-normal-form.md)).

The **canonical reduction** `red(C)` (replace each `P_s` by the marginal `P_s*`) is
fiber-preserving, contractive, idempotent, monotone, and a normal form: equal fibers ⇔
equal reductions [Theorem — proved, CELL-10L/L1]. It is **not transition-stable**: a
reduced predecessor can have a raw successor with newly unsupported edges, so staying
reduced means re-reducing after each exact update [Constructed counterexample,
CELL-10N] — the seed of rec's matching-minor calculus
([support-dynamics](support-dynamics.md)).
