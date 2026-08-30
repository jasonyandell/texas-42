# The root-interval probe — counted-belief Slice A instrument (§44)

EXPLORATORY tier — below every evidentiary tier, cited by nothing above
it. Instrument records only; never a play-strength claim.

**Source:** `walt/math/counted_belief_sandwich_v0.1.md` Parts I–II,
rulings CBS-A1..A9; producer `solver::root_interval`; instrument
`bin/rootinterval.rs`. Gates: `tests/solver_root_interval.rs` (6/6 green
at this run).

**Declared epoch:** one field σ = Level0 { n0 = 2 }; lower witnesses =
pinned level-1 continuations at declared schedule [2, 2], provenance
FIXED; upper stream epoch 0, evaluation stream epoch 1; δ = 1/20 per
endpoint per action; prefix 16. Exact authority: `exact_root_value` per
action. Record: `run1.txt` (this directory), 2026-08-30.

## Headline readings

| root | fiber | legal | decision | worlds-to-singleton |
|---|---:|---:|---|---|
| h4-t6 | 90 | 2 | **DeltaRootWinner 1-1** (= the exact optimum) | **8** |
| h8-t5 | 92 | 3 | DeltaRootSet {2 of 3} (exact optimum 5-3 inside) | not reached (3→2 at t=15) |
| h10-t6 | 19 | 2 | UnresolvedRootSet — **exact tie** (both Q = 1) | n/a |
| h5-t6 | 27 | 2 | UnresolvedRootSet — **exact tie** (both Q = 4/9) | n/a |
| h12-t6 | 6 | 2 | UnresolvedRootSet — **exact tie** (both Q = 0) | n/a |
| h3-t5 | 200 | 3 | UnresolvedRootSet — **exact tie** (all Q = 1) | n/a |

Realized coverage: **all 14 action rows** have L ≤ Q ≤ U on the frozen
streams (the gates assert this).

## What the run says, honestly

1. **The machinery separates real decisions fast.** The one root with a
   real gap (h4-t6: 13/15 vs 1/3) settled to a singleton — the exact
   optimum — after **8 sampled worlds** against a fiber of 90. h8-t5
   excluded its weakest action (16/23) at t=15 while the top two
   (91/92 vs 71/92) remain honestly entangled at this prefix and δ.
2. **Four of six roots are exact ties, and the honest output is
   UnresolvedRootSet.** A tie is the parent's §40.7 "true decision
   hardness" failure mode: no sampling budget can separate equal values,
   and the typing refuses to invent a winner. The §2 ladder's
   `ExactRootTie` / practical-equivalence rungs (future, with the exact
   backend) are the right answer for these roots — the receipt corpus at
   trick 6 is tie-rich, which is itself a finding.
3. **Shortfall attribution (the §44 probe's demanded split):** lower
   shortfall (Q − L) sits at ~1/6..2/5 everywhere — mostly the
   endpoint's small-sample width at prefix 16, partly witness-policy
   quality; upper excess (U − Q) collapses to 0 on decided-heavy roots
   (all-success prefixes put the grid endpoint at 1 = Q) and stays
   ≤ 10/27 elsewhere. At these prefixes the interval width is
   sample-width-dominated, not overfit-dominated: no optimization-lock
   vacuity is visible (§40.1 not triggered at this scale).
4. **Cost attribution:** the upper walks are CHEAP (≤ 1.2ms per root at
   prefix 16, all actions) — the value walk shares public nodes across
   the sampled multiset. The **lower replays dominate** (61ms at
   h3-t5): each PinnedThenLevel1 [2, 2] evaluation runs level-1
   materializations per focal state. Exact authority at fiber ≤ 200
   costs ≤ 25ms. "Need a better policy" and "need a cheaper witness
   evaluation" are the same lever here; the upper side is not the
   bottleneck at trick 5–6 scale.

## Boundaries

- Realized δ-bounds on two frozen streams; nothing here is a coverage
  measurement (the coverage mathematics is CBS-A1/A2's step-checked
  proof, spot-swept in the gates at the adjudicated 11/128).
- Trick 5–6 receipt roots only. The parent's target regime (trick 1,
  fiber 399,072,960) is exactly where `exact_root_value` is unaffordable
  and only the sampled interval route runs — unmeasured until a later
  slice; nothing here extrapolates.
- One field, one witness family, one δ split. No claim survives a
  changed identity coordinate.
