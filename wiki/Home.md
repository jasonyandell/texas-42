# Texas 42 Foundations — Wiki

A synthesis and navigation layer over the two immutable specification packages in
[`ingest/`](../ingest/). This wiki compresses, explains, and cross-links; the packages
remain the source of truth. Nothing under `ingest/` is ever modified (each package
carries a `MANIFEST.sha256` making it a verifiable provenance snapshot).

## The object in one paragraph

Straight points-and-marks Texas 42 is modeled as a **declaration-indexed physical
game** plus, per player, an **imperfect-information game over hidden deals and current
hidden remainders**. The load-bearing discovery chain: a viewer's rule knowledge about
the three hidden hands is *exactly* captured by three dependent capacity cells
(pool, per-seat allowed sets, capacities); that cell support has a **globally minimal
canonical normal form** (certain tiles + a determinate/binary/ternary ambiguity core);
the set of normal forms **legal play can actually reach** is a strict subset of the
Hall-feasible ones, with its exact cardinality open inside a proved 26–46-bit interval;
and **support is not belief** — two legal histories can share the same exact 90-world
support yet require opposite optimal leads under every named utility.

## Citation convention

- **v0.7** = [`texas-42-foundations-source-of-truth-v0.7`](../ingest/texas-42-foundations-source-of-truth-v0.7/) — the *proof-assistant boundary revision*.
- **rec** = [`texas-42-foundations-source-of-truth-v0.7-reconstructed`](../ingest/texas-42-foundations-source-of-truth-v0.7-reconstructed/) — the *reduced play/support foundation*.
- `Math §x` = that package's `docs/20_MATHEMATICAL_FOUNDATION.md`; `Rules §x` = `docs/10_RULES.md`; `Exec §x` = `docs/30_EXECUTABLE_SPECIFICATION.md`; claim IDs like `CELL-14` refer to `docs/40_CLAIM_STATUS.md`.
- Every substantive statement carries the package's own claim-status label
  (e.g. **Theorem — proved mathematically**, **Theorem — exhaustive finite verification**,
  **Unresolved**). See [claim-ledger](claim-ledger.md) for the status vocabulary.

## Start here

- [FINDINGS.md](FINDINGS.md) — what this object is, strongest results, risks, and next questions.
- [package-provenance.md](package-provenance.md) — how the two packages relate and the authoritative merge order.
- [discrepancies.md](discrepancies.md) — every disagreement found between the packages, with recommended resolutions.

## Pages

### Game and algebra
- [rules-profile.md](rules-profile.md) — the normative Straight 42 rules profile (identical in both packages).
- [declaration-algebra.md](declaration-algebra.md) — nine declarations as relational algebras; unique winner; transports and the three unscored mechanics classes.

### Exact hidden-information support
- [support-fiber.md](support-fiber.md) — capacity cells, the intensional current-remainder fiber, and the losslessness theorem.
- [capacity-dp.md](capacity-dp.md) — exact counting (≤512 states), Hall feasibility, and the exact uniform sampler.
- [minimal-support-normal-form.md](minimal-support-normal-form.md) — the coarsest exact support quotient, 81-bit census, compiled forms.
- [reachability.md](reachability.md) — legal-prefix reachability, the feasible-but-unreachable witness, symbolic trace certificates, and the open 26–46-bit interval.
- [support-dynamics.md](support-dynamics.md) — (rec only) support normal form as a transition state; matching-minor calculus; monotone 63-edge budget.

### Viewer state, belief, and value
- [reduced-viewer-kernel.md](reduced-viewer-kernel.md) — (rec only) folded trick, actor-from-capacities, utility accumulators, the reduced kernel, and future-equivalence minimality.
- [belief-vs-support.md](belief-vs-support.md) — support ≠ belief; Bayes filtering; the 90-world posterior-flip counterexample.
- [strategic-state.md](strategic-state.md) — the exact decision state (c, e, β); utility lenses; quotients and gauges.

### Meta
- [claim-ledger.md](claim-ledger.md) — status vocabulary and the merged claim inventory.
- [verification.md](verification.md) — every verifier script, what it exhausts, and fresh-run results.
- [proof-assistant-plan.md](proof-assistant-plan.md) — trust boundary, formalization order (v0.7 Handoff + Ledger, rec Kernel Map).
- [first-implementation-slice.md](first-implementation-slice.md) — what an implementation ("rob") builds first.
- [open-problems.md](open-problems.md) — merged unresolved claims and boundaries.
