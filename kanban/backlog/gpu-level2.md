id: [[gpu-level2]]
opened: 2026-08-24

## What

Level 2 makes every field decision inside every rollout a
fiber-sampling solve — multiplicative blowup that points at the GPU
(the exact workload factors into GPU-shaped batches + CPU search).
PARKED: not actionable until Jason's word, per the standing GPU
escape-hatch protocol. The M0–M3 gate lineage (gpu-spec/gpu-ref/metal
receipts) is the substrate to build on.

## Done when

Unparked by Jason; then scoped against [[level2-field-swap-probe]]
results (only the wake-up positions need level-2 evaluation).

## Links

[[level2-field-swap-probe]], walt/GPU-NATIVE-TRICK1-M3.md
