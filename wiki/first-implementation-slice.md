# First Implementation Slice (Codex Assignment 01)

[Home](Home.md) · owns: the record of the original slice-01 assignment (historical) ·
Sources: both packages `docs/50_CODEX_IMPLEMENTATION_PROMPT.md` (rec's is a strict
superset — merge resolution [D11](discrepancies.md)) and `Exec §26`. See also
[FINDINGS.md](FINDINGS.md) §"What rob needs first".

> **Status (2026-07-27): historical — executed and superseded.** rob executed this
> slice in Rust per [rob/BRIEF.md](../rob/BRIEF.md), which supersedes this page's
> Python file plan and extends it through S4 (normal form + capacity DP). Slice 02
> (S5–S9, [rob/BRIEF_SLICE_02.md](../rob/BRIEF_SLICE_02.md)) and the S10 stretch
> (the x:001 floor family, `verify_floor`) are also green — twelve byte-diffed
> receipts, every ingest number reproduced exactly ([verification](verification.md)).
> This page remains as the record of the original assignment.

## Scope: domino universe + declaration algebra only

Fresh, dependency-minimal Python 3.12 (stdlib only, `unittest`, strict types,
immutable values). Allowed files: `pyproject.toml`, `src/forty_two/{__init__,dominoes,
declarations,algebra}.py`, `tests/test_{dominoes,algebra}.py`,
`verification/verify_algebra.py`. **No** auction, deal, state, fiber, belief, solver,
CLI, or optimization code in this slice — and do not begin the next slice.

Required API surface: domino ids (canonical `(0,0),(1,0),(1,1),…,(6,6)` order — id
magnitude never determines rank), count points, the nine declarations,
`DeclarationAlgebra` with called/powered/effective_suits/led_suit/lead_contexts/
lead_fiber/follows/rank/tier/trick_key/beats/threat/resolve_trick; **rec additions**:
`pip_sum`, `competitive_ordinal(led_suit, id)`, `PipTrumpTransport` +
`unscored_mechanics_class(declaration)`.

## Exhaustive acceptance (must all pass)

1. 28 unique dominoes, id round-trips, incidence sizes, count total 35.
2. Effective suits: called ⇒ `{CALLED}`; uncalled doubles 1, mixed 2; follow ≡
   membership; exactly 7 leadable contexts, lead fibers partition with sizes
   `{1..7}`; doubles-trump context 0 nonempty-but-unleadable (do **not** delete the
   follow table for it).
3. Tier/rank laws; trick key `(0,0)` exactly at tier 0.
4. **737,100 unique-winner cases**, each agreeing with a *separately coded prose-rule
   resolver* that never calls the trick-key machinery.
5. `BEATS` ≡ key comparison for every declaration/context/pair.
6. Negative witness: NT `0-0`/`1-1` empty threat sets, different follow suits
   (threat is not a complete ontology).
7. All 5,040 pip permutations: exactly identity and `2↔3` preserve count; `2↔3` is a
   game-order isomorphism exactly between layers 2 and 3.
8. **rec additions**: looped-K7/symmetric-square identity; count = sum-5/10
   antidiagonal; all 49 ordered unscored pip-trump transports; exactly 3 unscored
   mechanics classes.
9. `verification/verify_algebra.py` prints a deterministic receipt with the exact
   counts above.

## Guardrails (both 50-docs)

- Never copy the package verifiers (they are proof receipts, not implementation
  source — and per v0.7, not proof-assistant kernel proofs either).
- No hard-coded result tables, no packed-bit primary representation, no floats for
  rank, no global mutable state, no strategic value on dominoes, no silent weakening
  of exhaustive tests.
- Docstrings cite claim IDs (e.g. "Implements ALG-06/ALG-07").
- **Ambiguity protocol**: if the package is internally inconsistent, do not pick a
  plausible reading — add a failing/blocked test, report the exact conflicting
  passages, continue elsewhere. (The known package-level inconsistencies are already
  catalogued in [discrepancies](discrepancies.md); rob should treat v0.7's type
  discipline as controlling per the [merge order](package-provenance.md).)

## What comes after

The natural slice sequence implied by the specs: (2) auction/contract/objective play
with phase-indexed states, (3) information + cells + fiber + losslessness parity
tests, (4) normal form + SCC compiler + counting/sampling, (5) support dynamics +
symbolic reachability (rec Exec §17A/§18), (6) belief/filtering + the 90-world
regression. (2)–(5) are done through rob slice 02; (6) remains unassigned. Each must
keep derived views derived and reachability proof-irrelevant.
