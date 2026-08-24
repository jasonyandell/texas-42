# Intake — *Calculated Evidence for Unified Walt*, v0.1

**Status:** exploratory intake companion — the same tier as its parent.
Nothing here is promoted, and nothing here may be cited above exploratory
tier.
**Date:** 2026-08-24
**Parent:** `walt/math/calculated_evidence_v0.1.md` (filed 2026-08-24,
verbatim; `calculated_evidence_v0.1.sha256` =
`9b32b14ffddbb19af42a4c0ec90edc0bf3d27506ec98c7d5c5058222a1b9e8f8`).
**The parent stays verbatim.** Observations live beside it and are never
written into it.
**Provenance:** hand-ferried by Jason 2026-08-24 (side channel; not an
automation dispatch — `submission_count.txt` untouched). The parent
self-describes its repository snapshot as main at `4231cb2…`; verified at
intake: that is exactly the main head the intake branch is based on. The
parent was therefore written against the post-unification,
post-wiki-overhaul, post-math-reorg state it cites.

---

## 1. Identity verification

Every boxed identity and stated anchor with a mechanical route was checked
by exact-rational computation (stdlib `fractions`, no floats —
`verify_calculated_evidence_v0.1.py` beside this file; run it from
`walt/math/` and clean up `__pycache__` per D15). **All 18 checks PASS
exactly.** The receipt headline:

    calculated_evidence_v0.1 verification: ALL CHECKS PASS (exact rational; no floats)

Highlights of what was verified, and by how many independent routes:

- **CE-T1 (§3.1):** the boxed finite sum, the substituted integral
  `∫₀¹(1+Rt)^s(1-t)^f dt`, and a direct exact polynomial integration of the
  defining mixture agree **three ways** on a grid of six rational `c` and
  `0 ≤ s,f ≤ 12`.
- **CE-T2:** the boxed definition `E^<_{s,f}(c) = E^>_{f,s}(1-c)` equals
  the *independently constructed* natural lower-test uniform mixture
  `(1/c)∫₀^c …` — i.e. the definition is the real object, not just a
  convenient symmetry.
- **CE-T3 closed form (§4.1):** integral == closed integer form on
  `0 ≤ a,b ≤ 40` (exact polynomial integration) and closed form ==
  CE-T1 finite sum at `c = 1/2` on the full V1 grid `0 ≤ a,b ≤ 100`.
  All nine anchors; `E^+_{a,0} = (2^{a+1}-1)/(a+1)` through `a = 100`;
  the calculated pivotal requirement (9 insufficient, 10 sufficient at
  `α = 1/128`) reproduced by blind monotone search.
- **Supermartingale algebra:** the one-step identity
  `E[L_r(B)] = 1 + (p-c)(r-c)/(c(1-c))` on a full rational grid, `≤ 1`
  whenever `p ≤ c ≤ r`; the CE-T3 raw-world multiplier `≤ 1` whenever
  `θ ≤ 1/2 ≤ r` (nonpivotal worlds create no fake directional evidence);
  CE-T4/T5 factor nonnegativity on the declared `λ` ranges and one-step
  `E ≤ 1` under the null.
- **Ledgers:** `Σ 1/(ℓ(ℓ+1))` telescopes exactly to `N/(N+1)`
  (checked to `N = 10⁴`); the edge-threshold algebra `T = m(m-1)/δ`.
- **Coordinates:** `H = (q-g²)/g² = 1/(qτ²)-1` with `g = qτ`; the §7.1
  small-τ expansion of `D_{1/2}` (`τ²/2 + τ⁴/12 + τ⁶/30`, odd terms
  vanish) by exact rational Taylor arithmetic; the `n ≈ 2(H+1)ln T`
  consistency identity.
- **§10.1 counterexample:** `P(X>0) = 3/4` with `E[X] = -1/32` exactly —
  the sign-majority defect is real.

**Checked by hand only** (standard arguments; no mechanical route; no
defect found): Ville's inequality and the mixture-of-supermartingales
closure; the predictable-sequence generalization stated in §3; the §5
union-bound risk accounting under adaptive pair opening; the §5.1
safe-elimination argument; §6.1 (exact results spend no risk); the §8.4
DP forecast's "exact conditional on the declared predictive law" status;
the §11.4 escalation bookkeeping (no double counting of sampled
multiplicities in the exact sum); the §17 execution-order invariance
claims as design requirements. These are the natural targets for
adversarial adjudication (§5 below).

## 2. Vocabulary flags

- **Clean on D3 (with one mention).** "Certificate" appears exactly once,
  in the parent's own vocabulary paragraph, *disclaiming* its use for the
  new objects. Mention-only; nothing in the parent calls any new object a
  certificate.
- **θ/ϑ split (proposed resolution of the standing collision).** The
  signed-pivotal intake companion flagged the collision between
  `θ = (1+τ)/2` (pivotal win share) and walt's auction threshold θ and
  proposed never writing bare θ. This parent instead resolves the
  collision by *renaming the auction threshold* `ϑ` (§15) and keeping
  `θ = (1+τ)/2` for the pivotal quantity. The two proposals conflict in
  mechanism but not in intent. **Proposed adjudication (pending walt-math
  adoption):** adopt this parent's split — θ = pivotal win share,
  ϑ = auction/policy threshold (the empirical 11/16 is a ϑ) — and update
  the signed-pivotal companion's proposal note to point here. Until
  adopted, any doc using either symbol spells out which quantity it means
  at first use.
- **New vocabulary introduced** (exploratory, this lineage):
  **δ-settlement**, the six-way result ladder (`ExactFiberRoot`,
  `ExactFrozenSet`, `DeltaSettled`, `EpsilonEquivalent`, `Unresolved`,
  `HeuristicFallback`), **evidence process**, **risk ledger**,
  **information rate** `𝓘 = q·D_{1/2}(τ)`, **freeze tuple** / `PolicyId`,
  **evidence debt** `R_debt`. None collides with existing walt or rob
  vocabulary (grepped). The claim prefix **CE-T** is unclaimed anywhere in
  the repo; this lineage owns it.
- **"Outer belief" (§11.5)** matches existing SCENARIO-PLAYER usage (the
  uniform outer-fiber measure; cf. `sample_belief` in code). Support ≠
  belief discipline is respected: the parent's evidence statements are
  about declared sampling laws, never about believed states.

## 3. Obligation numbering

- O12–O19 confirmed live at `SCENARIO-PLAYER.md` (obligations table);
  O10–O11 remain permanently reserved by the unfiled side-channel import
  (recorded in the signed-pivotal intake). The parent's proposed
  **O20–O28 are unclaimed** anywhere in walt/, wiki/, or rob/ (grepped at
  intake). Numbering is consistent; the proposals remain proposals until
  adjudicated.

## 4. Current-code boundaries (parent §A, verified at intake)

- **A.3 confirmed, and wider than stated:** the legacy shuffle-and-reject
  sampler `sample_belief` lives at `walt/walt/src/solver/mod.rs:715`
  (citing SCENARIO-PLAYER §4.2) — and additionally has near-copies in two
  binaries (`src/bin/playout.rs:559`, `src/bin/walt_bridge.rs:534`). The
  semantic seam the parent names (kernel exact sampler vs solver legacy
  sampler) has *three* legacy expressions, not one. The Step-2 kernel
  adapter should retire all three call paths or leave them explicitly
  labeled regression oracles.
- **Fixed-count inventory:** the `16×` escalation multiplier is literal at
  `walt/walt/src/solver/mod.rs:922` (`n_cur >= n_outer * 16`) and
  `walt/walt-wasm/src/api.rs:425`. The base counts (40/160/200/800) enter
  as request parameters with consumer-side defaults rather than solver
  literals — the correctness-path removal in §0 therefore lands in the
  controller and the consumers' default plumbing, not in one grep-able
  constant.
- The parent's crate/module description of the unified workspace matches
  the tree at `4231cb2` exactly.

## 5. Adjudication agenda (for the Pro refinement pass — not yet dispatched)

Per the exchange iteration policy (refine in-conversation before any
panel), the highest-value adversarial targets, in order:

1. The **predictable-sequence hypothesis** in §3: state precisely the
   filtration and the conditional-null condition under which CE-T1/T2
   remain valid for non-iid streams, and whether the common-world design
   of §17.2 satisfies it when candidates are eliminated adaptively.
2. The **§5 risk accounting**: the union bound is over directed edges with
   anytime validity per edge; confirm no additional spending is needed for
   the data-dependent *order* of examinations and the elimination rule
   (§5.1), including resumed epochs.
3. **§9.2 practical equivalence**: the two one-sided bounded-mean tests
   compose into `EpsilonEquivalent` at summed risk; confirm the composition
   is valid when both tests read the same world stream.
4. **O24 bookkeeping proof**: the sampled-to-exact escalation with cached
   outcomes; adversarial attention to duplicate worlds and partial batches.
5. **§14.6 paired field-correction evidence**: `Z_i/2 ∈ [-1,1]` under
   CE-T4 — confirm the paired design needs no independence between
   `Y^{(0)}` and `Y^{(1)}` beyond world-level iid.
6. The **θ/ϑ vocabulary adjudication** (§2 above).

Anything Pro repairs lands in this companion under dated markers; the
parent is never edited.

## 6. What this intake does not do

It does not adjudicate the O20–O28 obligations, does not authorize any
code change, does not promote the parent above exploratory, and does not
touch the exchange ledger. The implementation sequence (§22 of the
parent) begins only after the refinement pass and Jason's word.
