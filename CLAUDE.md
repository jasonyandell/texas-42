# CLAUDE.md

Exact solution of straight points-and-marks Texas 42, mathematics proved before code
is trusted. New session? Read `QUICKSTART.md` first; `wiki/Home.md` is the map.
Binding work assignments live in `rob/BRIEF*.md`.

## Hard rules

- **`ingest/` is immutable.** Never modify anything under it (each package carries a
  verifying `MANIFEST.sha256`). Package conflicts are resolved in
  `wiki/discrepancies.md`, never by editing sources. Never copy ingest verifier code
  into an implementation — verifiers are proof receipts, not source.
- **Evidentiary tiers are never promoted or blurred.** Corpus statuses >
  proof-assistant kernel > exchange-adjudicated CONFIRMED > rob conformance receipts
  (`wiki/Home.md`). A green receipt is evidence, never a status change; external
  `PASS` is never imported as an axiom (TRUST-01). Every substantive statement you
  write in the wiki carries its tier label.
- **Exploratory stays exploratory.** `wiki/ideas.md` and `wiki/analysis.md` (and
  probe outputs like `ablation_probe.rs`) sit below every tier and are cited by
  nothing above them. A probe number becomes quotable as a result only by brief
  amendment that adds it to a verifier receipt.
- **Vocabulary is load-bearing.** "Necessary outer profile," never "certificate"
  (D3; CI greps enforce this in rob). Keep support ≠ belief, feasible ≠ reachable,
  possible ≠ probable distinct — they are typed distinctions, not emphasis.
- **Citation convention** (`wiki/Home.md`): **v0.7** = the type-discipline package,
  **rec** = the new-mathematics package; `Math §x` / `Rules §x` / `Exec §x`; claim
  IDs like `CELL-14`; `x:NNN` for exchange results. Merge rule: rec's mathematics
  under v0.7's type discipline (`wiki/package-provenance.md`).

## Code discipline (rob, and anything executable)

- Derived views, never stored state: cells/fiber/normal form are *functions* of the
  semantic state; storing both authorities is forbidden.
- Reachability is a proof-irrelevant proposition — no identity-bearing certificates;
  equality/hashing/serialization through projected state only.
- No floats anywhere near ranks or probabilities — exact integers and rationals
  (clippy denies `float_arithmetic`; a grep denies `f32`/`f64` mentions).
- Every exhaustive count in the spec is a CI assertion; receipts under
  `rob/receipts/` are byte-diffed in CI — regenerate via the verify binaries, never
  hand-edit.
- Frozen generator values (`FROZEN_WITH_VOIDS` = 970, the `verify_player`
  transcript) are rob-internal determinism freezes, not ingest numbers.
- Ambiguity protocol: if a spec is internally inconsistent, don't pick a plausible
  reading — add a failing/blocked test, report the exact conflicting passages,
  continue elsewhere.

## Commands

- **rob CI (the gate):** `rob/ci/check.sh` — fmt, clippy `-D warnings
  -D float_arithmetic`, no-float grep, vocabulary greps, release tests, byte-diffed
  receipt comparison. Run it before calling any rob change done.
- **Ingest verifiers:** stdlib-only Python 3.12, run from each package's
  `verification/` dir. Trap: running them creates `verification/__pycache__`, which
  then fails rec's `audit_package.py` no-transients check — audit first or clean up
  (D15).
- **Lean:** `lake build` in `lean/` (mathlib dependency; first build is slow).

## Exchange channel (`exchange/`)

Hard lifetime budget: **10 dispatches total, ever** (8 used — check
`exchange/submission_count.txt`). Never submit without Jason's explicit go. Dispatch
prompts are self-contained (5.6 Pro sees only the pasted text) and adversarial with
machine-checkable deliverables. Every inbox response is adjudicated — witnesses
re-run, programs executed, proofs step-checked — before anything touches the wiki.

## Wiki conventions

- Every page opens `[Home](Home.md) · owns: <scope> · Sources: <citations>` — one
  page owns each topic; link rather than restate.
- When a result's tier changes (new adjudication, new receipt, new kernel proof),
  update the owning page *and* `wiki/claim-ledger.md`, `wiki/FINDINGS.md`, and
  `wiki/open-problems.md` as applicable — they cross-reference each other and drift
  is a bug (Home.md's exchange status line has gone stale this way before).
- Dissents and caveats travel with results verbatim (e.g. REACH-20's 2/3-SOUND
  panel is never presented as 3/3).
