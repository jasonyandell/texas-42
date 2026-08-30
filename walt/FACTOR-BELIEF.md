# FACTOR-BELIEF — the Slice C design skeleton (specify, do not optimize)

**Status:** design register for the counted-belief Slice C (§56's second
required output: a skeleton, no implementation). EXPLORATORY tier.
Source: `walt/math/counted_belief_sandwich_v0.1.md` Part V–VI (§18–26),
rulings CBS-A6 (posterior closure adopted with boundary obligations;
`FiberDp` recognized as the uniform-root backend) and CBS-A9 (this
skeleton authorized; the recursive factorized solver is NOT part of the
first build session). Retired to git history when the Slice C build
lands its own module docs.

## The objects (parent §18–21)

A **hand factor** is one hidden seat's exact nonnegative-rational weight
table over its possible root hands — `φ_{s,h}: C(U, k_s) → ℚ≥0` — and a
**factor belief** is the seat factors joined by the disjoint-cover
constraint, with the public history that produced them:

```text
HandFactor identity   = (seat, capacity, weight-table representation)
FactorBelief identity = (root physical fiber id,
                         public state sufficient for every field read,
                         field identity,
                         every HandFactor's representation and weights,
                         stochastic-tape factor id if any,
                         utility and bid/declaration parameters)
```

The identity list is §43 verbatim and BINDING (CBS-A6): a cache hit
under an omitted coordinate is the PiKey defect reborn. Weights are
integers/rationals only; the uniform lawful fiber is the special case
where every factor is a 0/1 legality predicate.

## The contraction interface (parent §24)

```rust
/// One abstract authority; a backend changes only under exact
/// extensional parity gates (CBS-O13 shape). Every returned mass is an
/// exact integer or rational derived from one canonical factor state.
trait ExactCoverOracle {
    /// Z_h — the exact-cover partition function of the factor state.
    fn mass(&self, belief: &FactorBelief) -> BigUint;
    /// Per possible hand A of `seat`: φ_{s,h}(A) · C_{-s,h}(U \ A),
    /// the exact compatible-completion weight (§21).
    fn actor_completion_weights(&self, belief: &FactorBelief, seat: Seat)
        -> Vec<(HandId, BigUint)>;
    /// {action t ↦ Z_{ht}} for the acting hidden seat under the declared
    /// field — the one-ply branch-mass target (§21's boxed equation).
    fn branch_masses(&self, belief: &FactorBelief, seat: Seat, field: &FieldModel)
        -> Vec<(Domino, BigUint)>;
    /// The posterior update: multiply ONLY the acting seat's factor by
    /// its action likelihood (Theorem 20.1 — closure is the theorem).
    fn condition(&self, belief: &FactorBelief, seat: Seat, action: Domino)
        -> FactorBelief;
    /// Exact mass of a structural predicate over deals (Part IV cells).
    fn count_cell(&self, belief: &FactorBelief, predicate: &CellPredicate) -> BigUint;
    /// Exact marginal mass of a hand predicate for one seat.
    fn marginal(&self, belief: &FactorBelief, seat: Seat, predicate: &HandPredicate)
        -> BigUint;
}
```

(Names indicative; Rust surface free at build time — L2-A3's naming
latitude applies.)

## Backend zero, and the gates

- **Backend zero is `kernel/fiber.rs` (`FiberDp`)** — the shipped
  tile-pattern capacity DP *is* the uniform-root special case (CBS-A6's
  attribution amendment and its happiest finding). Slice C's stage C0
  wraps it behind the trait for 0/1 factors; no new counting mathematics.
- **Mass conservation gate:** `Z_h = Σ_t Z_{ht}` exactly, at every
  contraction (§46 C0).
- **Parity oracle gate:** every backend result equals complete-world
  enumeration on its declared domain (`bundle` and explicit enumeration
  are the extensional oracles — CBS-O13). Small/medium fibers first;
  the trick-1 target (399,072,960 worlds, 116,280 acting-seat hands) is
  stage C2 and is a REPRESENTATION result even if the field classifier
  stays slow (§46).
- **Boundary obligation (CBS-A6, binding):** any field or belief with
  cross-seat coupling voids Theorem 20.1 until represented as explicit
  additional factors — never silently projected into seat-local form.
  The shipped level-0/level-1 fields are verified seat-local (intake
  companion, code-boundary audit).

## Build order when authorized (§46–48, unchanged)

C0 trivial field (`FixedPreference::lowest_first`) → C1 cached level-0
field with per-hand classification → C2 opening root branch masses.
Recursion (§47's factorized fixed-policy Bellman) only after one-ply
parity and cost are understood. Measured coordinates per §26: contraction
arithmetic, distinct hand materializations, field cost per hand, reuse,
support shrinkage, cache identity cost, integer width, trick-1 memory,
SIMD/GPU/WASM suitability (a measurement coordinate under the ripcord
discipline — CBS-A9, never an authorization).
