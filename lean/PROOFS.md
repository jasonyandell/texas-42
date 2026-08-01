# Proof playbook

Standing instructions for anyone (human or agent) contributing to `lean/`.
The authority order and trust boundary are in [README.md](README.md); this
file is the working discipline.

## Hard rules

1. **No `sorry` on the main branch.** In-progress statements live on PR
   branches only. A slice merges when it builds sorry-free.
2. **No `native_decide`, no new axioms.** Every theorem must close over the
   standard axioms only (`propext`, `Classical.choice`, `Quot.sound`).
   Verify with `#print axioms` in a scratch file before committing —
   this is the receipt; record it in the commit message.
3. **Never import an external `PASS` as an axiom** (TRUST-01). External
   receipts (Python verifiers, rob) are evidence for *us*, never premises
   for the kernel.
4. **Definitions are the trust surface.** The kernel checks proofs; nothing
   checks that a definition matches the ingest spec except review. Every
   definition carries a docstring citing its ingest source (`Math §x.y`,
   ledger row `PA-xNN`). When two packages disagree, the wiki's resolution
   (`wiki/discrepancies.md`) governs; note it in the docstring.
5. **Statement fidelity over proof convenience.** Do not weaken a statement
   to make it provable without flagging it loudly. If a ledger row can only
   be proved in a weaker form, that is a *finding* — surface it, don't bury
   it.

## What `decide` may do

Kernel `decide` is a legitimate proof (it *is* kernel evaluation — allowed
by the trust boundary, unlike `native_decide`). Use it freely for
finite-arithmetic facts: cardinalities, census counts, witnesses, per-domino
case sweeps. Measured capacity on this machine: a 9 × 8 × 28 × 28 sweep
(≈ 56k trick-key comparisons, `swap23_transport_iff`) elaborates in seconds.
Structure still matters where the spine demands it: the unique trick winner
is derived from key injectivity (PA-A10 → PA-A11), with `decide` nowhere in
the chain — the 737,100-case enumeration stays a separate reflection target
(PA-A12). Don't `decide` over permutation/function spaces (`Equiv.Perm`
enumeration does not kernel-reduce in reasonable time; 7^7 function sweeps
don't either) — use the spec's analytic argument instead
(see `countPreserving_iff`).

## Idioms that work (discovered here, reuse them)

- **Fin arithmetic → omega**: destructure the structure, then
  `simp only [Domino.mk.injEq, Fin.ext_iff, Fin.le_def] at *` and `omega`.
  omega handles disjunctive hypotheses (`hasPip` cases) natively.
- **Atom discipline for omega**: omega matches atoms *syntactically*.
  Ground every non-literal term it must reason about with an explicit
  equation first (e.g. `have v5 : ((5 : Pip)).val = 5 := rfl`), and avoid
  `fin_cases` when hypotheses mention pip literals — `fin_cases` produces
  `⟨n, _⟩` terms that don't match `(n : Pip)` atoms. For extensionality
  over `Fin 7`, use `perm_ext7`-style match on `⟨0,_⟩ … ⟨6,_⟩`; defeq at
  the application site absorbs the literal mismatch.
- **Lex keys**: `Prod.Lex.le_iff` leaves `(ofLex (toLex …)).1` unreduced;
  add `ofLex_toLex` to the simp set. Extract components of a key equality
  with `congrArg Prod.fst (congrArg ofLex h)`.
- **`WithTop ℕ` ranks**: `WithTop.coe_ne_top` for double/mixed rank
  clashes, `WithTop.coe_inj` (or `exact_mod_cast`) to drop to ℕ.
- **Match-defined predicates**: `simp only [foo]` needs the equation
  lemmas; for `Decidable` instances write them per-constructor
  (`inferInstanceAs`), don't hope for derivation through the match.
- **Definition unfolding in proofs**: plain defs rewrite with
  `simp only [foo]`, not `rw [foo]`; `unfold foo` also works.
- **Probe before guessing mathlib names**: write a tiny scratch file and
  run `lake env lean probe.lean` with `trace_state` / `exact?` rather than
  iterating on the full build. Names drift (`le_of_not_le` is gone; use
  `not_le.mp`).
- **Projection equalities of computed structures**: to get
  `C.pool = Q.pool` from `hred : C.red = Q.red`, elaboration works
  outside-in, so `congrArg CellSys.pool hred` against the plain-goal
  ascription fails; write
  `show C.red.pool = Q.red.pool from congrArg _ hred` and let iota
  reduction close the gap.
- **No `rw` into proof-dependent terms**: rewriting `hX : X = {s}` inside
  a term mentioning `theUnique X h` breaks the motive. Chain equalities
  instead (`(eq_singleton_theUnique h).symm.trans hX`) or `set` the
  dependent value to an opaque local first.
- **Canonical extraction without order**: pull the unique element of a
  card-one `Finset` with `Finset.choose (fun _ => True)` under a `dif`
  guard — no `LinearOrder` needed, and the guarded characterization
  (`excl_eq_some ↔ …`) replaces all definitional unfolding downstream.
- **Function-level `have`s beat pointwise ones near binders**: state
  intermediate equalities as `CellSys.certain C = CellSys.certain Q`
  (full function), then `rw` rewrites the partial application even under
  `Finset.filter`'s lambda, where a pointwise equation could not.
- **Capacitated matching via slot expansion**: quota problems reduce to
  mathlib's Hall theorem over `(s : H) × Fin (r s)`; group Hall
  conditions per seat-set dominate arbitrary slot subsets
  (`exists_partition_of_hall` in `NormalForm.lean`).

## Build mechanics

- `elan` lives in `~/.elan/bin`; `lake build` from `lean/`.
- `.lake` is 7.4 GB of mathlib artifacts. For a new worktree, APFS-clone
  it: `cp -Rc <main>/lean/.lake <worktree>/lean/.lake` (instant,
  copy-on-write). **Never run two `lake build`s in the same directory
  concurrently.**
- A full incremental build of this library is ~20–60 s; budget for that
  loop, not for interactive elaboration.

## Per-slice checklist

1. Read the ingest sections being formalized (both packages if they
   overlap; wiki resolves conflicts).
2. Land definitions with citations; keep them minimal — derived views as
   functions, not fields (TYPE-02 discipline).
3. Prove the row's theorems; every theorem docstring names its `PA-` row.
4. `lake build` clean, `#print axioms` on every new theorem.
5. Update `lean/README.md` layout list and
   `wiki/proof-assistant-plan.md` status; commit with the axiom receipt in
   the message.
