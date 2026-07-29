# Proof-Assistant Plan (Merged)

[Home](Home.md) · owns: the trust boundary, the K0–K15 spine, mechanization
priorities · Sources: **v0.7** `60_PROOF_ASSISTANT_HANDOFF.md` +
`65_MECHANIZATION_LEDGER.md`; **rec** `60_PROOF_ASSISTANT_KERNEL.md` — complementary:
v0.7 supplies the trust boundary, type discipline, priorities, and milestones; rec
the dependency spine covering its new mathematics. Related:
[verification](verification.md), [open-problems](open-problems.md).

**Status (2026-07-28): K0–K3 landed.** [`lean/`](../lean/README.md) is a Lean 4 +
mathlib Lake project. Kernel theorems now cover the K0/K1 layer —
`Fintype.card Domino = 28` (PA-A02), the natural incidence covering and pair
intersections (PA-A03), `∑ d, countPoints d = 35` (PA-A04) — and the K2/K3
declaration algebra: nine declarations, effective suits with membership bounds and
called absorption (PA-A05/A06), follow exactness (PA-A07), rank/tier/trick key
(PA-A08), lead-nonzero-tier (PA-A09), key injectivity in nonzero tiers (PA-A10),
and the **unique trick winner** (PA-A11) — proved via key injectivity as the spine
demands, not by enumerating the 737,100 cases (that receipt stays PA-A12,
reflection, open). **Layer A is now complete** apart from PA-A12: BEATS exactness
(PA-A13), threat monotonicity (PA-A14), the lead-threat incompleteness witness
(PA-A15), the count-preserving classification `σ ∈ {id, 2↔3}` by the analytic
forcing argument (PA-A16), and the scoped `2↔3` transport — order-isomorphic
exactly between layers 2 and 3 (PA-A17). All depend only on the standard axioms —
no `sorry`, no `native_decide`. Working discipline: [`lean/PROOFS.md`](../lean/PROOFS.md).
**Layer B core landed (K5)**: the auction machine with the `min(cap,5)` mark
ceiling (PA-B01–B03), deal worlds (PA-B05 define), contract (PA-B06), the reduced
play state with legal-set characterization (PA-B07/B08), invariant-preserving
atomic transition (PA-B09), and conservation — 28 plays, seven tricks, 42 points
(PA-B10). Open in B: deal cardinalities (B05), auction census (B04, reflect),
graded-DAG/Markov/settlement rows (B11–B14). Next: B13 settlement + the
42↔sweep trajectory theorem, then Layer C (information and cells).

## Trust boundary (v0.7 Handoff §2; TRUST-01)

```
adopted rules → kernel-checked theorems → proved-refined executables
             → external Python receipts / production implementations
```

A finite receipt becomes a kernel theorem only via: (a) direct formal proof,
(b) proved-sound internal decision procedure + kernel evaluation, or (c) proved
reflection with kernel-checked certificate. **Never import `PASS` as an axiom.**

## Non-negotiable design decisions (v0.7 Handoff §§3–6)

- **Finite first**: finite PMFs before any standard-Borel machinery; the native game
  never needs disintegration or measurable selection (rec Kernel §1 agrees).
- **Phase-indexed state types** (auction / declarationPending / play / handComplete)
  so illegal field combinations are unconstructible.
- **Reachability as a proof-irrelevant proposition**:
  `CertifiedState := { s // Reachable(s) }`; equality/hashing/serialization through
  the projection only; witnesses erasable. `ReachabilityOuterNecessaryProfile` never
  constructs `Reachable`.
- **Derived views, not fields**: `deriveRuleCells`, `supportReduction`,
  `compileExactSupport`, `remainderFiber` are functions; caches live in a separate
  `CompiledView` with a coherence proof, outside the information partition.
- Sets/vectors first; bitmasks and packed codes only via proved refinement maps
  (`decode∘encode = id`, operation commutation); hashes are identifiers only after
  proved collision freedom.

## Dependency spine (rec Kernel §3, K0–K15)

K0 finite universe (28, covering) → K1 count antidiagonal (35) → K2 declaration
mechanics → K3 trick order (unique winner via key injectivity, *not* enumeration) →
K4 transports (3 unscored classes; keep `UnscoredMechanics` and `ScoredMechanics`
distinct structures) → K5 objective contracted hand (legality, conservation, 42) →
K6 information & remainder (typed deal/remainder distinction) → **K7 general finite
capacitated matching kernel** (formalize Hall/counting generically, then specialize) →
K8 Straight cell losslessness (4-case induction) → K9 marginal support + normal form
(quotient theorem; 81-bit census as finite arithmetic) → K10 dynamic support
(matching-minor = conditioning; monotonicity; 63-edge budget) → K11 symbolic
reachability (soundness by reversing typed transitions) → K12 folded play/support
kernel → K13 seat-frame gauges (D₄ only on the oriented family) → K14
future-equivalence minimum (Myhill–Nerode) → K15 finite belief layer (Bayes,
pushforward, uniform fiber law, **90-world witness as a named theorem**).

K10–K14 exist only in rec; K0–K9 and K15 appear in both plans with matching content.

## Priorities and receipt-migration routes (v0.7 Ledger)

Priority-0 rows (must close for the first release): finite types and enumeration;
covering; count 35; effective suits; unique winner; legal-play characterization;
transition invariants; 42-point conservation; perfect-recall records; cells/fiber
definitions; losslessness induction; typed bijections; normal-form
trichotomy/compile/decode; reachability predicate + proof irrelevance; finite PMF
Bayes; strategic sufficiency; **the 90-world witness (PA-E10)**.

Receipt routes (Handoff §9): 737,100 winner cases → reflection; Hall corpus → prove
Hall generally, demote corpus to implementation test; 81-bit census → proved
enumerator + reflected cardinality or stay external; 26–46 interval → formal
injections for the bounds, external count stays a receipt; feasible-unreachable and
90-world witnesses → internal concrete witness proofs.

Deferred by both plans: CFR/equilibria, team-coordinator constructions, neural
architectures, special contracts, full-match almost-sure termination, byte layouts,
exact reachable-support cardinality (as a *prerequisite* — it stays an open theorem
target), standard-Borel fields.

## Quotients that must stay distinct (rec Kernel §4)

Endpoint identity · slot gauge · unscored pip transport · scored `2↔3` · seat/oriented
frame gauges · support-fiber equality · future equivalence per output contract ·
value equality per field/utility. **No one implies another without a theorem.**

## Acceptance standard (v0.7 Handoff §13, merged with rec Kernel §8)

Every adopted rule an explicit definition/parameter; every core ledger row kernel
proved or honestly marked external/open; no semantic equality depending on
proof/witness/cache/hash; one mechanical source of truth for support; information
equality never silently coarsened to mechanical equality; the 90-world witness
internalized; extraction separated from optimized encodings; (rec) symbolic
reachability, reduced-kernel sufficiency, and output-relative minimality checked.
