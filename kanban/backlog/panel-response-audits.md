id: [[panel-response-audits]]
opened: 2026-08-24

## What

Two conformance audits created by the x:019–023 panel-response
adjudication (PANEL-A3/A5/A6, `walt/CENSUS-RULINGS.md`), plus the
slice-3 adoption list. All believed-compliant-by-construction; belief
is not a receipt.

1. **Controller vs Claim-D repair + W7–W11** (CE thread): audit
   `solver::controller` / `solver::adaptive` that (a) no code path
   assigns edge risk retrospectively to already-consumed evidence
   (future-only opening or preallocation — the all-pairs α on one
   stream should satisfy preallocation trivially); (b) batch semantics
   canonically replay per-index liveness (W8) and speculative outcomes
   never enter evidence before replay (W9); (c) same-index crossings
   resolve deterministically with a typed inconsistency result (W10);
   (d) pause state is complete (W11). Deliverable: a short conformance
   note per point with code cites, plus gates where a property is
   assertable.
2. **Slice-2 exposure walk vs the τ coupling definition** (L2 thread):
   one-line check that `solver::exposure`'s first-split fork implements
   exactly the stopping time τ (equal public histories, non-focal
   actor, σ0(J)≠σ1(J) — first such t on the common prefix), not any
   wider or narrower event.

## Slice-3 adoption list (PANEL-A7/A8, for the slice-3 brief)

- Directional rungs R⁺/R⁻ beside E4 (coupled branches to terminal).
- Fixed-policy reports retain (d, r, c⁺, c⁻, c); pairwise reports
  retain (B, H, q, g).
- Kind vocabulary grows: NoFieldExposure / OutcomeStable /
  ValueNeutral / EpsilonEquivalent / Dominated / Unresolved.
- Dominance route: H=0 ∧ B>0 by exact enumeration or valid bound only.

## Done when

Both audits have committed conformance notes (or filed defects), and
the slice-3 brief cites this card. Related: [[level2-field-swap-probe]],
[[gran-anchor-reconstruction]].
