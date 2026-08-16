# rob

**rob** is this repository's exact Texas 42 engine: a from-scratch implementation
of the ingest packages' executable specification, built as the reconciled merge —
**rec's mathematics under v0.7's type discipline** — that neither ingest package
is on its own.

rob is an *executable mathematical specification with proof receipts*, not a game
server. Its job is to reproduce, independently and exactly, every number the
ingest verifiers certify, under invariants the ingest packages state but could
not enforce.

**Full documentation lives in the wiki**, which is the reference this file points
at rather than duplicates:

- [`wiki/rob.md`](../wiki/rob.md) — the artifact: workspace and module layout,
  the binaries, the invariants, the CI gate, the receipt discipline, the frozen
  determinism values, and how rob relates to `walt/` and `lean/`.
- [`wiki/rob-slices.md`](../wiki/rob-slices.md) — the build history: what each
  brief assigned and what each stage established.
- [`wiki/verification.md`](../wiki/verification.md) — every receipt and every
  exact integer in it.
- [`wiki/analysis.md`](../wiki/analysis.md) — the probes, rigs and inspector.

## Authority order

1. The binding brief for the work in hand — [`BRIEF.md`](BRIEF.md) (slice 01),
   [`BRIEF_SLICE_02.md`](BRIEF_SLICE_02.md) (slice 02),
   [`BRIEF_PLAYER_01.md`](BRIEF_PLAYER_01.md) (the player track): language,
   layout, invariants, receipt harness, discrepancy resolutions, definition of
   done. The briefs are historical assignments and are not edited to match later
   work — read each with its own amendment notes.
2. [`../wiki/`](../wiki/Home.md) — the reconciliation layer: package provenance,
   merge order, discrepancy resolutions, claim synthesis.
3. [`../ingest/`](../ingest/) — the two immutable source-of-truth packages
   (v0.7 and v0.7-reconstructed). Definitions, theorems, and claim IDs live here.

rob **conforms** to the layers above; it never redefines a mathematical object,
never edits `ingest/` or `wiki/`, and never resolves a specification conflict
locally — new ambiguities are surfaced as blocked tests and reported (BRIEF §11).
rob never copies ingest verifier code: the package verifiers are proof receipts,
not source, and translating them would destroy the independence that makes rob's
agreement meaningful.

## Layout

A three-crate Rust workspace. `crates/core` is the pure engine (RNG-free);
`crates/player` holds the seat-playing code — the fixed field policy, the exact
information-set solver, rob's rolling re-solve, the demoted Monte Carlo baseline,
and every line of seeded randomness in the repository; `crates/verify` holds the
stage harnesses and the receipt binaries; `receipts/` holds the committed
expected outputs; `inspector/` is a self-contained HTML viewer over a generated
trace; `ci/check.sh` is the gate.

## Running it

```sh
cargo test --workspace --release          # the suite, including every count assertion
cargo run --release --bin verify_rob      # a receipt, printed
cargo run --release --bin trace_rob       # regenerate the inspector trace
ci/check.sh                               # the full gate — fmt, clippy, greps, tests, receipt diffs
```

Nothing in rob is done until `ci/check.sh` prints `rob ci/check.sh: PASS`.

**Never hand-edit a file under `receipts/`.** Receipts are stdout from the
same-named binary and are byte-diffed against a fresh run in CI; editing one
converts a check into a wish while leaving it looking green. To change one
intentionally, regenerate it — `cargo run --quiet --release --bin <stage> >
receipts/<stage>.txt` — and review the resulting diff.

## Status

Slices 01 and 02 are green (S1–S10), as is the player track (P1–P5); twelve
byte-diffed receipts sit under `receipts/`. Slice 03 and beyond are scoped in the
briefs and not begun. The Lean 4 formalization ([`../lean/`](../lean/README.md),
[wiki/lean](../wiki/lean.md)) is a companion track: rob is outside the proof
kernel's trust boundary by design, and its receipts are evidence for us, never
premises for the kernel (TRUST-01).
