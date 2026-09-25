# Dynamic Support: the Matching-Minor Calculus (rec only)

[Home](Home.md) · owns: the matching-minor calculus, monotonicity, the 63-edge
budget · Source: **rec Math §7.14.1–7.14.2** (no v0.7 counterpart). Related:
[minimal-support-normal-form](minimal-support-normal-form.md),
[reduced-viewer-kernel](reduced-viewer-kernel.md), [reachability](reachability.md).

This layer is rec's central upgrade: the globally minimal support normal form is
promoted **from a static quotient to an exact dynamic state**.

## Support-normal dynamic sufficiency

**[Theorem — proved, rec Math §7.14.1, TRANS-08]** Given a feasible normal form `N`,
the declaration, the actor, the played domino, and the current led context (or lead
boundary), the exact successor normal form is uniquely determined:

```
N' = N( ϑ_{s,d}( ⟦N⟧ ∩ E_o ) )
```

where `E_o` conditions on possession (lead/follow) or possession + complete void
(slough), and `ϑ` removes the played tile. Viewer plays leave hidden support unchanged.
So exact support needs *no* cell reconstruction from raw mechanical fields to evolve —
it is closed under typed public observations. (Standalone support without that typed
context remains insufficient — REACH-03A.)

## The matching-minor update

**[Theorem — proved, TRANS-09]** The same update runs directly on the reduced holder
graph, no world enumeration:

```
force edge d→s → delete slough-forbidden edges e→s → contract played tile
             → recompile matching-supported core → re-encode normal form
```

equals extensional conditioning + pushforward. With
[capacity-dp](capacity-dp.md)'s SCC compiler, each step is a linear pass over ≤24
vertices / ≤63 edges.

## Monotonicity and the 63-edge budget

- **Holder-edge monotonicity** [Theorem — proved, TRANS-10]: within one attempt, every
  surviving hidden tile's exact marginal holder set only shrinks
  (`A_{t+1}(e) ⊆ A_t(e)`) — proof by inverting the typed transition.
- **Ambiguity-phase monotonicity** [Corollary — proved, TRANS-11]: certainty never
  reverts to ambiguity; inactive seats never reactivate; tags move only
  `Ternary → Binary → Determinate`.
- **63-edge hand budget** [Theorem — proved, TRANS-12]: initial support has exactly
  `21 × 3 = 63` holder edges; each disappears exactly once; at most `21×2 = 42`
  deletions can be *informational* (while the tile is live) — the rest die when the
  tile is played.

Receipts: 1,331 feasible small supports × 170,058 typed observations, extensional
conditioning ≡ matching-minor update, 1,406,592 monotonicity edge-tests
[Theorem — exhaustive finite verification, TRANS-13]; 108 full hands, 3,024
transitions, exactly 6,804 = 108·63 deletions with no reappearance
[Finite verification receipt, TRANS-14]. Both in `verify_reduced_kernel.py`; every
number is reproduced end-to-end in Rust by rob's `verify_dynamics` and
`verify_symbolic` binaries ([verification](verification.md)) — conformance evidence,
not a status upgrade; the corpus receipt remains the ground truth.

## Why this matters for a solver

The within-hand hidden-information engine is a **monotonically deleting graph** with a
hard 63-edge lifetime budget and a canonical, provably minimal state after every step.
That means: bounded-size support states, O(edges) updates, no re-derivation from
history, and a natural DAG for memoization keyed on `(N, τ)` — see
[reduced-viewer-kernel](reduced-viewer-kernel.md).

**Caveat for implementation**: rec proved this while its executable spec still stores
cells and certificates as state ([discrepancies D1/D2](discrepancies.md)); an
implementation should realize the calculus under v0.7's derived-view, proof-irrelevant
discipline.

## Mechanization status (proof-assistant kernel tier)

Rows are **v0.7** `65_MECHANIZATION_LEDGER.md` `PA-` rows; "proved" = a declaration
under `lean/Texas42/` checked by the Lean kernel over at most
`propext`/`Classical.choice`/`Quot.sound`, with no `sorry`, `native_decide` or local
axiom (grep re-verified 2026-09-12), as of commit d190b26 (2026-08-02; all 42
priority-0 rows closed). Map: [lean-row-index](lean-row-index.md). rob's
`verify_dynamics` / `verify_symbolic` receipts and its invariants INV-11 EDGE-BUDGET
and INV-12 MONOTONE-AMBIGUITY are conformance evidence, never a status change.

**TRANS-08 through TRANS-14 are not mechanized.** They are rec-only rows: the v0.7
mechanization ledger predates rec's mathematics and carries **no `PA-` row** for any
of them; rec's own `60_PROOF_ASSISTANT_KERNEL.md` names them as spine section **K10
(Dynamic support)** — "the direct proof should use the typed inverse transition; the
finite verifier's small-domain exhaustion remains a receipt, not the general proof" —
and no Lean declaration exists for that section. [FINDINGS](FINDINGS.md) §6 names
TRANS-08/09 the weakest spot in the corpus: a prose proof exhausted only on ≤4-tile
supports, with the slough case interacting with re-reduction in a way a proof
assistant should re-derive carefully.

What the kernel does cover is the *substrate* the calculus updates, at priority 0 and 1:

| Object | Ledger row (priority) | Kernel status (d190b26) | Declaration (`lean/Texas42/`) |
|---|---|---|---|
| The typed transitions TRANS-01..05 (hidden removal is a bijection; viewer play is the identity) — the inverse transition the K10 proof is meant to use | PA-C09, PA-C10 (0) | **proved** | `Cells.lean:1075` `remainder_injective`, `:717` `allowed_step_viewer` |
| The canonical reduction `red(C)` — fiber-preserving, contractive, idempotent, coarsest exact quotient — i.e. the "recompile matching-supported core" step's target | PA-C15 (1, backbone) | **proved** over the generic capacitated kernel | `Reduction.lean:90` `red_red`, `:112` `fiber_eq_iff_red_eq` |
| The normal form `N` itself (compile/decode inverse laws, total classification) | PA-D01..D05 (0) | **proved** | `NormalForm.lean` |
| TRANS-08 successor normal form `N' = N(ϑ_{s,d}(⟦N⟧ ∩ E_o))`; TRANS-09 matching-minor update ≡ conditioning + pushforward | none (rec K10) | **not mechanized** ([discrepancies D5](discrepancies.md): confidence medium-high for the theorem itself for exactly this reason) | — |
| TRANS-10/11/12 holder-edge and ambiguity-phase monotonicity; the 63-edge budget | none (rec K10) | **not mechanized** | — |
| TRANS-13/14 receipts (1,331 / 170,058 / 157,809 / 1,406,592; 108 / 3,024 / 6,804) | external finite receipts | not kernel targets; `verify_reduced_kernel.py` re-run 2026-09-12 on this machine, identical lines; rob `r_dyn_*`, `r_sym_budget` conformance | — |
