# CI1-REPORT — the gate runs as fast as its laws

**Slice:** BRIEF-CI1 (2026-09-04). **Invariant:** same assertions, same
corpora, same oracles; only scheduling and recomputation changed. No
solver or engine code was touched; `ingest/` and freeze 58 untouched.
Every gate file's diff is fixture plumbing: no assertion text, corpus
constant, cap, contract or tail changed (checked by grepping the diff of
each file for assert/panic/expect lines — every one is inside the
fixture's own accessors).

## Item 1 — the concurrent runner (wired at e53752b; verified here)

- (a) `check.sh` PASS: 230 s wall, 121 binaries, 0 failed, 0 skipped.
- (b) A deliberately failing scratch test (`tests/ci1_scratch_failing.rs`,
  never committed): the gate exited 1 at the test stage, the runner
  printed the test's stdout marker line and its panic in full between
  `FAIL ... full output follows` and `---- end of output`, every other
  binary still ran (122 binaries, 1 failed), doc tests and Lean did not
  run. Wall 200 s.
- (c) 121 binaries run, matching the serial run's binary set (the
  e53752b comparison); 122 with the scratch test in place.

## Item 2 — recompute-once fixtures

One `LazyLock` fixture per suite (`tests/common/fixture.rs` holds the
shared `compute_all`, which evaluates a key list once across
`available_parallelism` threads; each suite includes it by `#[path]`).
The fixture is a derived view of the declared epoch, immutable after
construction; a key it lacks panics naming the coordinate — never a
silent recomputation.

| suite | fixture holds | stays fresh (the law) |
|---|---|---|
| `solver_factor_refine` | per (root, action): fixed / grammar / response masses from the independent recursions; per (ample config, root): the controller record | gate 1's bundled `exact_root_value`; gate 3's second run (determinism, one fresh run vs the fixture's); gate 4's starved and tight runs |
| `solver_horizon` | census per (root, contract, cut) under the ample cap; root `response_success_mass` per (root, contract) | H2's per-action doom and `Q_a`; H4's tiny-cap census; H5's one fresh census vs the fixture's |
| `solver_unified_carry` | the lazy walk to its end per (budget, root) | UC5's eager walks (lazy ≡ eager, one fresh run vs the fixture's) |
| `solver_focal_horizon` | engine per (root, contract, k, tail); exact `Q_a` per (root, contract); FH1b censuses per (root, contract, cut); FH1 endpoints (both tails' `viewer_success_mass`, `doom_enumeration`) per (root, contract, action); FH3 depth walk per (root, contract, action); FH5 replays of `π_k` per engine coordinate | FH-R's tiny-cap run; FH-D's one fresh run vs the fixture's; FH3's root depth; FH5's off-DAG walk; FH6's test-local world walker |

The focal suite's lazy per-key memo (a cache gates filled on demand)
became the eager fixture; `fh`/`exact_q` keep their signatures (the
receipt parameter is now unused). Independence is preserved as code
paths: every "independent recomputation" gate still compares the
engine's number to the independent recursion's — the latter's value now
read from the fixture (rule ii).

## Item 3 — measurements

Standalone `cargo test --release --test <suite>`, this machine (18
cores). "Before" is the review's serial-run figure from the brief.

| suite | before | after | fixture floor |
|---|---|---|---|
| `solver_factor_refine` | 146 s | 70 s | gate 3's ten fresh controller runs, serial |
| `solver_horizon` | 116 s | 50 s | the h4-t4 cut-4 census (41 s, one job) |
| `solver_unified_carry` | 61 s | 48 s | UC5's 21 fresh eager walks, serial |
| `solver_focal_horizon` | 350 s | 137 s | h4-t4 engines at k ≥ 2 (56 s each, unsplittable); pass 1 is 1,470 s CPU (81 s floor on 18 threads), the replay pass 407 s CPU |

Gate (`walt/ci/check.sh`, fresh target dir, everything included):

| | before (e53752b) | after |
|---|---|---|
| gate wall | 367 s | 230 s |
| runner makespan (slowest suite inside the gate) | 311 s (`solver_focal_horizon`) | 177 s (`solver_unified_carry`) |
| binaries | 121, 0 skipped | 121, 0 skipped |

Inside the gate the runner holds 9 binaries at a time and every suite's
threads contend, so in-gate suite walls are inflated against the
standalone figures (unified_carry 177 s in-gate vs 48 s alone; focal 141
vs 137; factor_refine 129 vs 70; horizon 85 vs 50) and the runner's
"sum of suite walls" (1,317 s) is a contended number, not a CPU budget.
The gate wall is the deliverable.

## Observations for the corpus-trim card

- `solver_unified_carry` is now the gate's long pole: UC5's eager walks
  are single-use by law and run serially inside one gate.
- Everything heavy in the focal suite is h4-t4 (fiber 34,650): 666 s of
  the fixture's 1,470 s CPU is that one root's engine runs.
