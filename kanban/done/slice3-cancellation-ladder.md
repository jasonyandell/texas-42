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

## Closed 2026-08-24

Merged as **PR #38**, main **`151ea4f`** (L2 thread); **central gate
green** (`walt/ci/check.sh`). Everything on the PANEL-A7/A8 adoption
list landed with gates (`tests/solver_fieldswap_cancel.rs`), probe
records at `walt/probes/fieldswap_cancel/` (`ALL 386 CHECKS PASS` on
the committed records, exploratory instrument tier). Probe firsts,
instrument-grade only: first `FieldDecisionChanged` in the wild (h8-t4
Stage 4, σ0-settled 2-1 vs σ1-best 5-5), first `FieldStableExactRoot`
(h4-t6), first `Dominated` (h4-t6, pin-1-1 over pin-0-0, H = 0 under
both fields); directional bounds ≈2.3× tighter than E4 in the h8-t4
regime without pruning there; Λ = 31/1200 regenerated and asserted.
Wiki resynthesis done: `wiki/walt-calculated-evidence.md` (slice-3
section), `wiki/walt-instruments.md` (the `fieldswap_cancel` bin),
`wiki/walt-seat-play.md`, and the `wiki/walt.md` / `wiki/Home.md`
status lines, plus the deferred items on
`wiki/walt-math-open-questions.md` §10.

**Three items deferred loudly** — all need design input, all stated in
`walt/probes/fieldswap_cancel/README.md`'s Deferred section, carded as
[[slice3-deferred-producers]] and pointed at [[panel-response-audits]],
which **stays in backlog** because its two conformance audits are still
open:

1. **δ-valid admissible-upper E3 producer** — the sampled E3 built here
   is the estimate sibling only, typed so it cannot feed screening; a
   valid upper bound on a supremum needs evidence-engine design.
2. **Dominance valid-bound route** — PANEL-A7 admits `Dominated` by
   exact enumeration *or* a valid bound; only the exact-enumeration
   producer exists, and no bound type was stubbed without one.
3. **§10 motif tags** — the structural motif vocabulary is absent, not
   approximated; aggregates ship seat/trick histograms and the
   conditional outcome difference only.
