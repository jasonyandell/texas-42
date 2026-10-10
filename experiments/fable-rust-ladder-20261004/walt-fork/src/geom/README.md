# walt-geom

Owns the exact one-parameter policy geometry (v0.4 §8--§9, finite-first per
§16.1): i128-backed rationals, affine lines in the valuation parameter,
continuous piecewise-linear envelopes on the ray `[0, inf)`, the argmax
correspondence of a finite envelope family, the 29-dimensional capture
feature/valuation space with its additive gauge (§8.3), and finite feature
sets carrying their support functions (§9.2) -- the polytope of §9.1 is kept
as its finite generating set, never a hull.

**Imports: `walt-core`** (for the domino index) plus `num-rational`. It knows
nothing about kernels, worlds, or operators.

Endpoint ownership is the envelope's type invariant: piece `i` owns the
half-open interval from its start to the next piece's start (the last piece
owns the unbounded tail), so every endpoint belongs to exactly one piece, with
no overlaps and no gaps. That is the `_combine` lesson -- v0.4 §14.1 discloses
a piecewise-linear interval-endpoint bug in the probe era -- promoted to a
representation that cannot express the bug. The merge is audited by an
exhaustive small-case suite that replays every result against direct scalar
max/min of the underlying lines at every breakpoint, its neighborhoods,
midpoints, and beyond.
