# Dynamic Support: the Matching-Minor Calculus (rec only)

[Home](Home.md) · Source: **rec Math §7.14.1–7.14.2** (no v0.7 counterpart). Related:
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
[Finite verification receipt, TRANS-14]. Both in `verify_reduced_kernel.py`.
**Independently reproduced in Rust** (2026-07-27): rob slice 02 S5/S6 reproduce
every number above exactly from a principled enumeration of the stated spaces —
see [verification](verification.md) §"rob (Rust)" — conformance evidence, not a
status change.

**Independent cross-language reproduction (rob, Rust):** these TRANS-13/14 receipts —
1,331 feasible NFs / 170,058 observations / 1,406,592 monotonicity checks / 108 hands /
3,024 transitions / 6,804 = 108·63 deletions — are reproduced end-to-end in rob's
`verify_dynamics` and `verify_symbolic` binaries
(`rob/receipts/verify_dynamics.txt`, `rob/receipts/verify_symbolic.txt`; see
[verification](verification.md) §"rob (Rust) — independent reproduction, slice 02").
This is finite verification / conformance evidence, **not** a status upgrade — the
corpus receipt remains the ground truth.

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
