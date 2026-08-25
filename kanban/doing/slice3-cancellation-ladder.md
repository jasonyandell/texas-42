id: [[slice3-cancellation-ladder]]
opened: 2026-08-24

## What

Slice 3 of the level-2 field-swap program [L2 thread]: consume Part VI
of the x:019–023 panel response (cancellation, irrelevance, one-sided
danger) under rulings PANEL-A7/A8 (`walt/CENSUS-RULINGS.md`, the
2026-08-24 panel-response adjudication). Adoption list per
[[panel-response-audits]]; parent probe program per
[[level2-field-swap-probe]].

Landed (code `walt/walt/src/solver/{exposure,field_swap}.rs`, gates
`walt/walt/tests/solver_fieldswap_cancel.rs`, instrument
`walt/probes/fieldswap_cancel/`):

- Cancellation ladder |c| ≤ r ≤ d with the three distinct zeros
  (behavioral d=0 / outcome r=0 / value c=0) typed and never collapsed;
  fixed-policy reports retain (d, r, c⁺, c⁻, c).
- Pairwise reports retain (B, H, q, g); dominance route `Dominated`
  only via H=0 ∧ B>0 by exact enumeration — `SampledPairwiseMasses`
  has no dominance method (type-level lock).
- Six-label kind vocabulary: NoFieldExposure / OutcomeStable /
  ValueNeutral / EpsilonEquivalent / Dominated / Unresolved.
- Directional rungs R⁺/R⁻ beside E4 (coupled branches to decided
  terminals), sandwich Q⁰−R⁻ ≤ Q¹ ≤ Q⁰+R⁺, directional winner
  stability, directional screening; extended ladder
  R± ≤ R^outcome ≤ R^exposure asserted in the producer.
- Sampled E3 typed distinct from exact E4 (estimate tier, no screen
  route); Λ evidence processes (§9.1 pivotal, §9.2 bounded-mean on
  Z/2); FieldSplitTrace + SplitAggregate; Stage-4 survivor-only σ1
  work (produces FieldDecisionChanged); ExactRoot baseline tier
  producer.
- The §42 interpretation rule verbatim in the field_swap module docs.
- The corrected Λ = 31/1200 (h8-t4, pin-5-5 vs pin-3-3) asserted in
  the probe bin at default knobs.

Deferred LOUDLY (see the probe README's Deferred section): the δ-valid
admissible-upper E3 producer, the dominance valid-bound route, §10
motif tags.

## Done when

Slice-3 code + gates green under `walt/ci/check.sh`, probe records
committed with the EXPLORATORY INSTRUMENT header, and the PR merged by
the parent session's central gate. Wiki resynthesis of slice-3 results
is a separate step owned by the parent session.
