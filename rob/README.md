# rob

**rob** is this repository's exact Texas 42 engine: a from-scratch implementation of
the ingest packages' executable specification, built as the reconciled merge —
**rec's mathematics under v0.7's type discipline** — that neither ingest package is
on its own.

rob is an *executable mathematical specification with proof receipts*, not a game
server. Its job is to reproduce, independently and exactly, every number the ingest
verifiers certify, under invariants the ingest packages state but could not enforce.

## Authority order

1. [`BRIEF.md`](BRIEF.md) — the binding implementation assignment: language, layout,
   invariants, receipt harness, discrepancy resolutions, definition of done.
2. [`../wiki/`](../wiki/Home.md) — the reconciliation layer: package provenance,
   merge order, discrepancy resolutions, claim synthesis.
3. [`../ingest/`](../ingest/) — the two immutable source-of-truth packages
   (v0.7 and v0.7-reconstructed). Definitions, theorems, and claim IDs live here.

rob **conforms** to the layers above; it never redefines a mathematical object, never
edits `ingest/` or `wiki/`, and never resolves a specification conflict locally — new
ambiguities are surfaced as blocked tests and reported (BRIEF §11). The known
cross-package discrepancies are already resolved in BRIEF §1.

## Layout

Rust workspace (see BRIEF §2–3): `crates/core` is the pure engine, `crates/verify`
holds the receipt binaries, `receipts/` the committed expected outputs, and
`ci/check.sh` the full gate (fmt, clippy, no-float grep, tests, receipt diffs).

## Status

Slice 01 (declaration algebra through support normal form + capacity DP) is assigned
by `BRIEF.md`. Later slices — support dynamics, reduced kernel, belief, solver — are
scoped there and not yet assigned. The Lean 4 formalization
([wiki/proof-assistant-plan](../wiki/proof-assistant-plan.md)) is a companion track
that consumes rob's receipts; rob is outside the proof kernel's trust boundary by
design.
