# Texas42 — Lean 4 formalization

Kernel-checked formalization of the Texas 42 foundations, as a
[Lake](https://github.com/leanprover/lean4/tree/master/src/lake) project
depending on [mathlib4](https://github.com/leanprover-community/mathlib4).

## Authority order

Lean formalizes **ingest as reconciled by the wiki**: the source of truth is
`ingest/texas-42-foundations-source-of-truth-v0.7/` (plus the reconstructed
package), reconciled through `wiki/` (see `wiki/proof-assistant-plan.md`).
`rob/` is outside the kernel trust boundary and cross-validates via receipts.
Per the trust boundary (TRUST-01): external `PASS` receipts are never imported
as axioms — finite claims enter the kernel only via direct proof, a
proved-sound decision procedure, or proved reflection.

## Layout

- `Texas42/Basic.lean` — Layer A finite algebra, first slice:
  - `Pip` (`Fin 7`), `Domino` (canonical `(high, low)` pair with `low ≤ high`),
    with `DecidableEq` and `Fintype` instances (ledger row PA-A01);
  - `Domino.card_domino : Fintype.card Domino = 28` (PA-A02);
  - `Domino.countPoints` and
    `Domino.total_countPoints : ∑ d : Domino, countPoints d = 35` (PA-A04).

Both theorems depend only on the standard axioms
(`propext`, `Classical.choice`, `Quot.sound`) — no `sorry`, no `native_decide`.

## Building

```sh
# elan (toolchain manager) must be on PATH; it lives in ~/.elan/bin
cd lean
lake exe cache get   # fetch prebuilt mathlib oleans (multi-GB, one-time)
lake build
```

The toolchain is pinned by `lean-toolchain` to match mathlib's pin.
Build artifacts live in `.lake/` (gitignored). The `.github/` workflows from
the mathlib template are inert while `lean/` is a subdirectory (GitHub only
runs workflows from the repository root); they are kept for a possible future
split into a standalone repo.
