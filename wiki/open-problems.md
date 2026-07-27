# Open Problems and Boundaries (Merged)

[Home](Home.md) · Sources: both packages `40_CLAIM_STATUS.md` §10 + Math §14 "Honest
boundaries". Merged per [package-provenance](package-provenance.md); rec's rewrites of
OPEN-01/OPEN-12 adopted. Statuses as labeled by the packages.

## Genuinely open mathematics (UNRESOLVED)

- **OPEN-11 — the flagship**: the exact cardinality of the strictly Straight-reachable
  support image `R_Str^m`, and hence the optimal standalone fixed-width code inside
  the proved **26–46-bit interval** [REACH-13]. Both packages explicitly refuse to
  collapse it by guesswork. rec's symbolic support DAG (REACH-16) is the obvious
  counting substrate. See [reachability](reachability.md) and FINDINGS Q1.
- **OPEN-01 (rec form)**: the reduced viewer kernel `K = (δ, H_m, N, τ, α_U)` is
  proved *exact*, and the global transition minimum is *defined* (future-equivalence
  quotient per output contract, QUO-10) — but **equality of `K` with any selected
  quotient is not established**. Is `K` injective up to future equivalence for the
  support-aware contract? See [reduced-viewer-kernel](reduced-viewer-kernel.md).
- **OPEN-12 (rec form)**: no closed-form *support-only* reachability criterion is
  known. Symbolic public-trace replay is exact and removes the hidden-deal
  requirement (REACH-14/15), but still carries legal ancestry.
- **OPEN-02/03**: minimal retained continuation record, minimal latent field-state
  representation, and minimal utility residue for arbitrary history-dependent
  fields/utilities — not established.
- **OPEN-04**: no general low-dimensional exact strategic quotient beyond the proved
  gauges (C₄/D₄, slot, `2↔3`, unscored transports) and the fixed-output future
  quotient.
- **OPEN-05**: extension of the cell/normal-form/reduced-kernel theorems to special
  contracts (nello, plunge, …) — structurally out of scope; nothing transfers
  automatically.

## Proved-boundary items (BOUNDARY — resolved negatively or scoped)

- **OPEN-06**: unrestricted native counting is cheap (CELL-10H/I); extensional
  enumeration, arbitrary predicate-restricted counting, and variable-seat systems
  keep separate computational boundaries.
- **OPEN-07**: off-path beliefs require an explicit assessment (Bayes undefined).
- **OPEN-08**: no canonical sampler from support alone — proved nonuniqueness
  (CELL-10D).
- **OPEN-09**: deterministic best response for arbitrary infinite private-signal
  models needs measurable-selection assumptions.
- **OPEN-10**: finiteness alone does not give effective exact algorithms for
  noncomputable utilities/operators.
- **Match horizon**: repeated all-pass attempts are unbounded without a termination
  assumption; almost-sure termination holds under a uniform per-attempt contract
  probability ε (geometric tail, `E[attempts] ≤ (2T−1)/ε`) [AUC-06/06A, MATCH-02].
- **No universal byte/runtime minimum** without a named cost model (CELL-29); no
  output-independent minimal "game state" (rec Math §12.10).

## Questions this wiki adds (not in either package)

1. **Gauge-reduction of the reachable census**: do the unscored pip-trump transports
   (rec ALG-22) biject reachable support images across the seven pip-trump
   declarations? Support content depends only on unscored mechanics (follow/lead
   relations), so plausibly yes — which would collapse the 9-declaration factor in
   REACH-11 to 3 classes and simplify OPEN-11. Needs a proof that the transport
   commutes with legal-prefix generation. *(Confidence: plausible-but-unproved;
   flagged in [FINDINGS](FINDINGS.md) Q4.)*
2. **Outer-language tightness**: the only feasible-but-unreachable witness (REACH-10)
   *fails* the lead-witness check. Is there a support that passes **all** outer
   necessary checks (capacity shape, schedule, lead witness, Hall) yet is
   unreachable? If none exists in some phase (e.g. after one trick), the outer
   language is exact there and the 46-bit ceiling tightens.
3. **Minimality of the 90-world witness**: is 90 the smallest fiber exhibiting a
   same-support posterior action flip under all four lenses, or does a smaller legal
   witness exist?
