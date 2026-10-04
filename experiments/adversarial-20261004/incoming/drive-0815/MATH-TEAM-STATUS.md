# MATH-TEAM-STATUS (folded in Oct 4 2026; caveats preserved verbatim in substance)

Vectorized flat shared-plan prototype (math team), 105 pip-trump roots, 100,800 (world, shared-order) rows:
- Exact terminal cells and action values matched the scalar oracle.
- Pilot cold array prep + simulation + reduction: 0.182–0.186 s, versus 1.300 s for scalar plans and 0.385 s for the grouped tree,
  in the same Python/NumPy runtime.
- Counted arrays 3.82 MB; process high-water 66.2 MB.
- Broader 134-root audit: 65 strict best-value gaps, 11 worse root choices, mean root regret 0.003731 — CONDITIONAL ON THESE FIXTURES.

Caveats (as stated by the math team, to be preserved wherever this is cited):
- This is NOT population or phone-strength evidence.
- Fixtures were independently legality-replayed to avoid the original sampler bug (batched.py v1 false-support fallback).
- Indexing used exact uint64 high-multiply.
- Unsupported non-pip declarations were excluded.
- No native, GPU, or phone speedup claim.
- No rerun was required for this status update.

How it sits with the folder's existing results:
- Corroborates: the shared-plan (sum-over-worlds) fold is lawful and fusion-free as implemented; the NumPy cost ordering
  flat < grouped tree < scalar matches flatplan.py's measurements (26k rows, ~180–270 ms per decision).
- Does not change: the restricted-class gap is real in both measurements. flatplan.py (two hands, 128 worlds, ref exact-tape@384):
  regret 0.0087 defending, 0.0131 bidding (0.034 on the opening lead). The 134-root audit's 0.003731 is on a different fixture set with a
  different reference; the two are recorded side by side and NOT averaged or compared across.
- Design conclusion unchanged: recommended inner rung = frontier-batched own-choice tree with the group-by-history fold; the flat
  shared-plan rung is a candidate for defender seats or a fast first pass, pending a population-level (tournament) test against
  Plunge a0d9fa80 / Rust cb1ef3b2 via harness.mjs.
