# [[waking-seat-census]]

Opened: 2026-08-25 · Closed: 2026-08-25 (wiki sync landed — the profile
reading is on [walt-calculated-evidence](../../wiki/walt-calculated-evidence.md);
the speed campaign it targeted ran the same day, PRs #53/#55/#56)

## What

The **waking seat** (`solver::waking` + `bin/waking_bridge`): the first
walt player variant that plays with a thinking-teammate model — level-0
(`solver::act`) by default, a hard-budgeted wake check per non-forced
decision, and σ1 escalation (`solver::targeted`) gated on POSITIVE
evidence that the σ1 leg selects a rival over act's choice — plus the
natural-play PROFILE over driven hands (every decision at all four
seats, one typed record per decision: path, wake evidence, escalation
outcome including the controller's per-phase `PhaseSpend` vector,
integer-microsecond spend per phase, worlds consumed).

**Scope note (2026-08-25, Jason):** the originally chartered scaled
affordability census was skipped — the smoke run already answered the
affordability question (minutes of decision compute per natural hand at
the live epoch: **not affordable as-is**). The deliverable became the
unordered-baseline PROFILE: which phase the microseconds actually go
to, by decision and by trick. That attribution is the targeting data
for the reorder-not-cull ordering build that starts next.
[[gpu-level2]] consumes the same reading.

## Done when

- [x] `solver::waking` built, gated (`tests/solver_waking.rs`, 9 gates +
      2 compile_fail locks), composing act / wakeup / targeted with
      nothing reimplemented; defaults untouched (CE-A7/§20.16).
- [x] `waking_bridge` speaks the controller_bridge line protocol
      (plunge/mk5 seat it with zero external changes) and adds the
      `driven` profile mode (deterministic seeds, generic deals,
      existing level-1 auction).
- [x] Profile filed (`walt/probes/waking/`, exploratory tier):
      per-phase spend attribution by decision and by trick, escalation
      `PhaseSpend` breakdown, wake rate by trick, agreement rate,
      caveats in the README.
- [x] The profile reading synced to the wiki (owning page +
      claim-ledger cross-refs as applicable) — the sync closes this
      card.

## Links

[[gpu-level2]] · [[level2-field-swap-probe]]

## Notes

- Declared epoch pair (live): σ0 = Level0{n0=2} (matches act's
  evaluation field), σ1 = Level1{n_outer=4, n0=2}, candidates [8,2].
  The l2_controller probe's epoch (σ0 n0=8) is DIFFERENT — numbers do
  not compose across the two surfaces.
- Declared retune: wake exact-fiber cap 64 → 1024 after the smoke
  priced the routes (the 24-world sampled probe cannot cross its
  telescoping δ thresholds; the exact route is the wake gate's real
  coverage). Recorded in the probe README.
