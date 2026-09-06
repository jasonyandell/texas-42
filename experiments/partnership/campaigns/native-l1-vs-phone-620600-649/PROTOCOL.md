# Five-minute native L1 / phone reference calibration

Declared before launch, 2026-09-06. Exploratory practical comparison authorized
by Jason; no equivalence theorem or power claim. One externally capped batch.

- Fresh random deal seeds 620600–620649, in ascending order, using the existing
  outcome-blind declaration/bidder heuristic. Every contract is 30.
- Three paired games per deal: all preserved phone; native baseline declaring
  against phone; native baseline defending against phone. The native seats use
  L1 (`baseline`), not the partner upgrade. Both inner beliefs are legacy.
- Native: 40 outer / n0=8, full fixed-sample comparison without tie refinement.
  Phone: literal archived WASM, nominal 40/8, original racing and refinement.
  Both have the existing 14-second decision wrapper and recorded fallbacks.
- Ten games concurrently, six native threads per game. Pool yields at 270
  seconds, then drains at most the current move; external watchdog kills the
  entire batch at 295 seconds. No second slice is authorized for this check.
- Use completed contiguous seed prefix for the primary comparison. Preserve
  any later completions and incomplete checkpoints. No failed or unfinished
  game counts as a make or set. Existing technical/downside stop rules remain.
- Primary: role-specific and combined paired make/set differences. Points do
  not break ties. ±5 percentage points is a useful practical equivalence band,
  but this short run is not expected to establish that narrow equivalence.
  Absence of a significant difference is not an equivalence result.
- Secondary: actual decision costs, fallbacks, and choices at identical
  information states encountered by both implementations. These are diagnostics,
  not independent strength trials.

Decision to inform: whether native L1 is a useful explicitly named internal
reference for future native experiments, while preserving the phone artifact
as an external anchor rather than claiming interchangeable implementations.
