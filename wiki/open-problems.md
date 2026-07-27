# Open Problems and Boundaries (Merged)

[Home](Home.md) · owns: the merged OPEN inventory and its current statuses · Sources:
both packages `40_CLAIM_STATUS.md` §10 + Math §14 "Honest boundaries", merged per
[package-provenance](package-provenance.md); rec's rewrites of OPEN-01/OPEN-12 adopted.

## Genuinely open mathematics (UNRESOLVED)

- **OPEN-11 — the flagship**: the exact cardinality of the strictly Straight-reachable
  support image `R_Str^m` remains open, but the proved standalone interval is now
  **[36,46] bits** (corpus-proved [26,46], narrowed at the exchange-adjudicated
  evidentiary tier by REACH-17 + REACH-18 — two structurally disjoint certified
  families totalling 36,913,384,410 > 2³⁵ reachable supports; see
  [reachability](reachability.md)). The exact census and any *full* declaration class
  remain open — dispatch 006 explicitly disclaims closing either. Both packages
  still refuse to collapse the exact count by guesswork. rec's symbolic support DAG
  (REACH-16) is the obvious counting substrate; exchange 001/006's validated
  witness-generation + split-zeta upward-closure counter (independently checked against
  brute force) is a reusable substrate for further tightening. In flight: dispatch
  007 (ceiling via the fifth condition) and 008 (exact no-void slice), submitted,
  responses pending. See [reachability](reachability.md) and FINDINGS Q1.
- **OPEN-01 (rec form) — RESOLVED, COLLAPSE** [exchange-adjudicated CONFIRMED
  (ALL_PASS 0.43s; 3/3 SOUND) — external tier, not a kernel proof
  ([claim-ledger](claim-ledger.md))]: the reduced viewer kernel `K = (δ, H_m, N, τ, α_U)` is proved
  *exact*, and the global transition minimum is *defined* (future-equivalence quotient
  per output contract, QUO-10). Dispatch 003 settles the equality question **negatively
  for the support-aware `P30_DECLARING_POINTS` contract**: `K` is strictly finer than
  the future-equivalence quotient — two reachable kernels differing only in raw fold
  ordinal (`r=7` vs `r=6`; NT, `P(30)`, viewer 0 / bidder 3) are machine-verified
  output-equivalent, so `r` is not an injective memoization coordinate. Mechanism: the
  dead-cut lemma. Witnesses `K₁/K₂`; source
  `exchange/inbox/003-kernel-vs-future-quotient.md`, verified program
  `exchange/adjudication/programs/003.py` + `witnesses/003.json`. See
  [reduced-viewer-kernel](reduced-viewer-kernel.md).
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
   REACH-11 to 3 classes and simplify OPEN-11. **Prerequisite SATISFIED**
   [exchange-adjudicated CONFIRMED (ALL_PASS 4.6s; 3/3 SOUND)]: the transport theorem
   `f_{t,u}(R_t)=R_u` is machine-certified (dispatch 004,
   `exchange/adjudication/programs/004.py`), so the transport commutes with legal-prefix
   generation and the REACH-11 declaration classes collapse from nine to **3** (one
   pip-trump class, doubles-trump, no-trump). Tagged census
   `|R~| = 7·r_pip + |R_DT| + |R_NT|`; the Step-15 quotient corollary's cocycle gap is
   closed by finite check (`programs/004-cocycle.py`, all 343 triples). See
   [reachability](reachability.md) and [FINDINGS](FINDINGS.md) Q4.
2. **Outer-language tightness — RESOLVED (negative)** [exchange-adjudicated CONFIRMED
   (ALL_PASS 0.9s; 3/3 SOUND)]: the witness `(NT, (6,6,6), V₁={6}, 18-tile pool)` passes
   **all four** outer checks (capacity shape, schedule, lead witness, Hall) yet is
   unreachable — so the outer language is *not* exact even at the `j=1` equal-capacity
   one-void phase. Exhaustion: 450 generators, 3 matches, 425,520 traces, 0 realizers.
   Source `exchange/inbox/002-outer-language-tightness.md`, verified program
   `exchange/adjudication/programs/002.py`. (Unlike REACH-10, which *fails* lead-witness,
   this witness passes it; the newly established follower-supply obstruction is the
   fifth necessary condition — see [reachability](reachability.md).)
3. **Minimality of the 90-world witness**: is 90 the smallest fiber exhibiting a
   same-support posterior action flip under all four lenses, or does a smaller legal
   witness exist?
