# Staged historical H0 diagnostic aggregator

Files: `/tmp/historical_h0_diagnostic.py` and
`/tmp/test_historical_h0_diagnostic.py`. No live edits. No execution, tests,
fits, or measured work performed. The following commands are for a later
coordinated lane, after H0 labels and its role-only actor exist:

```
H0_DIAGNOSTIC_FITTER=/path/to/response-ladder/tools/fit_compiled.py \
  python3 -m unittest discover -s /tmp -p test_historical_h0_diagnostic.py
python3 /tmp/historical_h0_diagnostic.py \
  --root /path/to/response-ladder \
  --output /path/to/new/historical-h0-diagnostic-v1
```

Input paths follow the frozen protocol: historical H0, compatible H0-seed,
and v3 T0 n8 campaigns; H0/v3/v4 role-only actors. The v4 n32 campaign is read
only to verify the v4 actor's authoritative fit audit on its proper population.
It is not mixed into the n8 target comparisons.

The loader verifies manifest request digests, exact ID/request/group/split
alignment, row status consistency, target/revision and sample count. It calls
the selected live fitter's read_rows, weights, and choose functions rather
than copying their arithmetic. No-void H0/H0-seed pairs must match their seeds,
full ordered worlds/tapes/weights, and exact action vectors. Their public
state and observation feature prefixes must agree regardless of target.

All missing/incomplete outcomes remain in coverage, including joint statuses
and split × role × remaining-hand-size × any-public-void strata. Only complete
pairs enter target comparisons. Costs use the target row's original mass,
equal physical-group weights within each role, then an equal two-role mean;
canonical agreement and optimal-set intersections are separately raw counts.
A role mean is unavailable if either role is empty. Conditional strata and
complete comparison populations have explicitly documented renormalization.

All three actors are scored on identical H0 development rows and identical
old-T0 development rows, plus a common-complete H0/T0 panel. v1 actors can use
the preserved14-feature prefix; an actor requiring absent features raises an
error rather than censoring the row. All costs are checked in [0,1]. The H0,
v3 and v4 authoritative per-role and role-mean train/dev audit fractions must
match exactly on each original fit population, with actor clauses, lesson
hashes and campaign authorities also checked.

Output is summary.json, a compact REPORT.md, and input-hashes.json. The summary
retains compact source tree digests; the sidecar has complete input file hashes.
Each target pair retains at most8 deterministic worst cross-cost witnesses.
There is no per-tile transition matrix, actor fitting, or utility claim.

Seven staged tests cover unequal masses and group counts, separate role means,
cross-cost normalization, censor coverage, equal normalized versus unequal
raw vectors, no-void seed/column/tape/vector mutations, request/feature identity
rejection, exact authoritative audit checks, and real fitter-backed loading.
