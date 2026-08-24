id: [[level2-field-swap-probe]]
opened: 2026-08-24

## What

Run the level-2 program specified in `walt/LEVEL2-PROBE.md`: field-swap
pivotal mass (q ≈ 0 under level-0 field, q > 0 under level-1 field) on
logged saturation-tie and near-tie sets — finds where level 2 matters
without building the full level-2 player. Also tests the hypothesis
that level 2 grows gaps and lowers sampling hardness. GATED on
[[walt-unification]] and [[adaptive-sampling-intake]] (sampling noise
diminished as a variable first). Full level-2 field likely needs the
GPU ([[gpu-level2]]).

## Done when

Per LEVEL2-PROBE.md's output contract: paired q̂/τ̂/ĝ/Ĥ under both
fields on predeclared tie sets; the wake-up set filed with mechanism
notes.

## Update 2026-08-24 (L2-A5)

`walt/math/targeted_level2_field_stability_v0.1.md` (adjudicated
L2-A1..A7) makes this probe the *detection layer* inside the targeted
level-2 controller — the targeting layer (exposure bounds, stability
screen, first-split traces, survivor-only field-1 work) is owned by
that parent. The field-swap build enters after the calculated-evidence
shadow step merges (L2-A6). Anchors: [[gran-anchor-reconstruction]].

## Links

[[walt-unification]], [[adaptive-sampling-intake]], [[gpu-level2]],
walt/LEVEL2-PROBE.md
