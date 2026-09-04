# BRIEF-CI1 — the gate runs as fast as its laws: concurrent suites, recompute-once fixtures

**Authorized:** 2026-09-04, Jason ("can it be made just as robust but to
run faster? ... I'm entirely fine with the CI taking minutes if it needs
to. I'm not yet sure it needs to" → "ok do 1 and 2 please"). This is
process work on `walt/ci/check.sh` and four gate files; it changes NO
assertion, NO corpus, NO oracle. Robustness is the invariant; wall is
the deliverable. Read `CLAUDE.md` (the new "Agents" section and the
gate-sizing rule under "Code discipline") and `kanban/backlog/
gate-corpus-trim.md` first.

## The review's findings (2026-09-04, from the 23:09 full `check.sh` log)

- 125 test binaries; `cargo test` runs them ONE AT A TIME. Sum of suite
  walls 498 s; 106 suites finish under 1 s; three suites are 65% of the
  total: `solver_factor_refine.rs` 146 s, `solver_horizon.rs` 116 s,
  `solver_unified_carry.rs` 61 s. The FH1 gate file `solver_focal_horizon.rs`
  is 350 s on its own. The machine has 18 cores; the FH1 run averaged
  3.3 of them.
- Inside the heavy suites the same expensive oracle is recomputed by
  every gate: the exact `Q_a` per root × contract (`response_success_mass`,
  seconds to tens of seconds at t4), whole censuses, whole controller
  runs. Independence that matters is between CODE PATHS (the engine vs
  the response recursion), never between recomputations of one function.

## Mission

1. **Wire the concurrent runner** (already written, not yet wired:
   `walt/ci/run_test_binaries.py`, stdlib, run under `python3 -I -B`
   like the other gate scripts). The patch to `check.sh` replaces the
   single `cargo test --workspace --release` with `--no-run
   --message-format=json | run_test_binaries.py "$cargo_target_dir"`
   followed by `cargo test ... --doc` (parity: doctests still run). The
   runner: every binary runs with its package dir as cwd and
   `CARGO_MANIFEST_DIR` set; output captured; printed in full on
   failure; all binaries run even after a failure; exit 1 if any failed;
   slowest-suites table at the end. Verify the wiring: (a) run
   `check.sh` — PASS; (b) a deliberately failing test in a scratch
   branch/stash shows its full output and fails the gate; (c) the
   number of binaries run equals the number of `Running` lines the old
   serial log printed (125 + FH1's), i.e. nothing is skipped. Record
   the before/after wall of the test stage in the report.
2. **Recompute-once fixtures in four suites**, assertions untouched:
   `tests/solver_factor_refine.rs`, `tests/solver_horizon.rs`,
   `tests/solver_unified_carry.rs`, `tests/solver_focal_horizon.rs`.
   Pattern: a `std::sync::LazyLock` (or `OnceLock`) fixture per suite
   holding the expensive values every gate reads — exact `Q_a` masses
   per (root, contract, action); censuses per (root, contract, cut);
   controller/unified-player runs per (config, root); engine results
   per (root, contract, k, tail) — computed ONCE per test-binary
   process. Each gate then reads the fixture and asserts exactly what
   it asserted before. Rules: (i) a gate whose LAW is "two runs agree"
   (determinism, lazy ≡ eager, resume ≡ uninterrupted) keeps ONE fresh
   run and compares it to the fixture's — never two fixture reads;
   (ii) a gate whose law is "independent recomputation agrees" keeps
   the independent code path (e.g. `response_success_mass` vs the
   engine) but takes that path's value from the fixture — the
   independence is the path, not the repetition; (iii) no assertion
   text, corpus constant, cap, contract or tail changes — `git diff`
   of each file must show only fixture plumbing; (iv) the fixture is a
   derived view of the inputs, immutable after construction, never a
   cache that a gate writes to.
3. **Measure and record** per suite: wall before (from the review:
   146/116/61/350 s) and after, and the test-stage wall before/after
   under the concurrent runner. Put the table in
   `walt/briefs/CI1-REPORT.md` with the invariant stated: same
   assertions, same corpora, same oracles; only scheduling and
   recomputation changed.

## Discipline

- **Never end a turn with background work pending** (CLAUDE.md,
  Agents): `check.sh` and test runs go in the foreground under the 600 s
  tool timeout (the whole gate should now fit; if a single run does
  not, split it — `cargo test --test <suite> --release` per suite — and
  say so).
- `ingest/` untouched; freeze 58 untouched; no engine or solver code
  changes at all — this slice touches `ci/` and `tests/` only.
- No floats; vocabulary greps still pass; `cargo fmt` clean.
- The FH1 gate file may have a `#[ignore]` BLOCKED test if the builder
  left one; do not delete or edit its assertions — fixture plumbing
  only. Do not touch any other test file.
- Commit on the current branch with a message starting `walt CI1:`;
  do not push, do not open a PR. Report back with: the before/after
  table, the three verification results for the wiring, every gate
  whose structure you had to touch beyond plumbing (should be none),
  under about 600 words.
