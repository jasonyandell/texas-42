# FACTOR-BELIEF — the Slice C–F design skeleton (specify, do not optimize)

**Status:** SLICES C (COMPLETE), D, E AND F LANDED (2026-08-30). Slice F
(§49, §27–31) is the consequence-CEGAR hand-class instrument
`refine_to_action_exact` in `solver/factor_belief.rs`: §28's feature map
`κ` (`ClassSignature` — the §49 starting vocabulary of critical tile
membership, trump count/highest trump, led-suit count, count-tile
possession, current-winner/ruff possibility) partitions the acting
seat's support at the field-classification bottleneck, and the §30 loop
aggregates action-uniform classes exactly while splitting the
largest-mass non-uniform class by a WITNESS PAIR — two same-class hands
with different field actions, their lowest differing tile entering the
§31 critical set — until residual class mass is zero (termination ≤ 28
refinements: a witnessed discriminator is provably outside the critical
set). Gated by `walt/walt/tests/solver_factor_consequence.rs` (4 gates:
Theorem 30.1's monotone narrowing with NESTED per-branch intervals
`[L_t, U_t]` and an action-exact endpoint; endpoint parity — the fully
refined abstraction reproduces `branch_masses` tile for tile; §49's
witness requirement re-derived independently, the field itself
re-consulted on hand-built records; and non-vacuity — the bare
vocabulary resolves positive mass, classes aggregate, the loop fires).
Probe: `factorcegar report` — the two-sided finding is that MASS
CONCENTRATES BUT THE TAIL FRAGMENTS: at the opening root under σ0,
805‰ of the 399,072,960-world posterior mass sits in action-exact
classes at 36,923 classes (3 hands/class; 513‰ already at 5,387
classes, 21 hands/class), but driving residual to ZERO costs full
fragmentation — 116,280 singleton classes, §51's falsifier for the last
slice of mass under a SAMPLED modeled mind — which vindicates §49's
interval discipline (carry small residual as branch intervals; don't
chase the endpoint). The instrument pays one classification per support
hand (the same bill as `branch_masses`) and claims representational
structure only, never a faster classifier; the trivial-field endpoints
DO aggregate (255/495, 56/126, 147/495 at trick 4), so the tail
fragmentation is a property of σ0's sampling, not of the vocabulary.
EXPLORATORY tier.

**Slice E** (landed the same day, an earlier round) —
Slice E (§48) is the factorized grammar best response
`grammar_success_mass` in `solver/factor_belief.rs`: the §23 recursion with the focal case's
frozen action replaced by a max over the grammar's actions —
`M^G(B) = max_{t ∈ G(I)} M^G(B·t)`, lawful on the cleared side because
every focal child shares `Z(B)`, so the max of masses is the max of
values; hidden nodes keep the conservation sum, and nodewise max equals
the §12 policy-class optimum `Q^G` by the cylinder-partition argument
(the belief is a function of the public history, so focal nodes are in
bijection with the viewer's information states). Gated by
`walt/walt/tests/solver_factor_response.rs` (4 gates: per-root-action
parity with Slice B's enumeration split `exact_grammar_split` under σ0,
with the root call the max over grammar root actions; a singleton
grammar collapsing exactly to the Slice D fixed-policy recursion;
source dominance plus non-vacuity — the constraint BINDS somewhere, via
singleton grammars, because the two-source grammar ties the free
optimum on every enumerable root; and the every-node checker with the
grammar max structure enumerated at every focal node). The §48 fence is
kept: NOTHING maximizes over the full action set — `free` figures come
only from the Slice B split, and no argmax/policy is extracted (that
needs a declared tie order; not a Slice E claim). Probe:
`factorresponse report` — the finding is that AT DEPTH THE MIX PAYS: at
trick-4 roots the grammar optimum strictly beats every source (h4-t4
trivial: Q^G = Z = 34,650, certain make, against 34,170 for the best
source; h3-t4: 3,815 against 3,062), while at trick-5/6 roots it never
exceeds the best source and the two-source grammar saturates every
reached undecided state (every §12 verdict "closes" with no deviating
continuation). EXPLORATORY tier.

**Slice D** (landed the same day, concurrent session) — Slice D
is the general
support contraction `SupportOracle` (§25.2's acting-hand loop
generalized to conditioned completions, §25.4's sparse-support walk) and
the §23 factorized fixed-policy recursion `viewer_success_mass` (§47),
both in `solver/factor_belief.rs`, gated by
`walt/walt/tests/solver_factor_recursion.rs` (5 gates: extensional
parity with backend zero across the C0 domain including the opening
root's contraction; surviving-world mass parity beyond one table with
backend zero's refusal preserved at the boundary; §47 value parity with
the bundled walk under the trivial field and under σ0 on every
enumerable root; and the every-node checker — mass equals the
surviving-world count and branch masses equal the world partition at
EVERY node of the recursion tree). The recursion computes the
viewer-objective success mass `M` with `V = M/Z` the exact integer pair
— §23 cleared of denominators by conservation, no rationals anywhere.
One law was discovered at depth: `condition` restricts its support walk
to hands CONSISTENT WITH THE PUBLIC RECORD (own plays contained,
others' plays excluded) — such hands are provably zero-mass and their
action likelihood is undefined; σ0's type-enforced information-state
constructor is what caught the unlawful classification, and at one ply
the filter is a no-op, so the C1 conditioning-support law is unchanged.
Probe: `factorrecursion report` — value parity on every row including
trick-4 roots (fiber 34,650, 16 post-root plies, 121,868 conditionings
under σ0); the honest negatives (bundled faster at worlds/hands ≈ 3,
recursion classifies more σ0 states than the bundled walk meets) are in
the probe README, and Slice E's probe extends them (the Slice B
enumeration split is 30–40× faster than the grammar recursion at these
same small ratios). EXPLORATORY tier.

**Slice C in full** — stages C0, C1 AND C2 LANDED (2026-08-30) —
`walt/walt/src/solver/factor_belief.rs`, gated by
`walt/walt/tests/solver_factor_belief.rs` (11 gates: the seven C0 gates —
three-way mass parity, branch-mass parity with complete-world enumeration
under two trivial fields and the σ0 level-0 mind, the Theorem 20.1
conditioning route, conditioned marginals against enumeration, the
declared domain refusals, the §22 opening-root contraction — plus the
four C1 cache laws: σ0 branch parity with the bundled one-ply oracle on
every receipt fiber WITH full extensional cache identity between the
routes, classification once per information state, zero sharing across
public histories under the full §43 identity key, and the opening root's
116,280 hands classified exactly once). C1 added no library code: the C0
contraction plus `FieldModel`'s insert-only cache already classify once
per state; C1 is the gates and the measurements. Probe:
`walt/probes/factor_belief/` — opening-root branch masses in 8.7 ms over
a 399,072,960-world fiber; the σ0 opening classification realized in
5.36 s (46 µs/hand); the REPEAT contraction 23.3 ms (200 ns/query pure
cache identity, ×230); cross-history reuse exactly 0 (the honest
negative that routes classifier compression to Slice F). EXPLORATORY
tier. Source: `walt/math/counted_belief_sandwich_v0.1.md` Part V–VI
(§18–26), rulings CBS-A6 and CBS-A9.

Stage C2 closes the slice and, like C1, added NO library code and no new
gate — it is the report §46 asks for: all seven required coordinates
from ONE opening-root run under the σ0 field (`factorbelief c2`, record
`walt/probes/factor_belief/c2_run1.txt`). Hands 116,280 (asserted);
contraction 5,933 µs for the completion weights alone, 21,818 µs warm
(weights plus full §43-key identity, zero classifications); field
classification 5,339,731 µs derived by subtraction from the 5,361,549 µs
cold pass — 45 µs/hand, 99% of the bill; 20 distinct branch tiles; reuse
×245 at 187 ns/query; memory as TWO figures kept apart — 23,563,392
bytes of declared accounting for the action cache (88-byte entries,
262,144 buckets, one control byte each, plus the key's one-tile history
Vec) against a 63,340,544-byte MEASURED maximum resident size
(`/usr/bin/time -l`, agreeing with the in-run `/bin/ps` reading at exit;
peak footprint 62,390,680 bytes); conservation exact at 399,072,960.
The memory coordinate deferred by C1 is therefore discharged, with the
accounting never presented as a measurement. The Slice D recursion
(§47) landed the same day, in a concurrent session (see Status above).

Build-time deviations from the sketch below, under L2-A3's naming
latitude (module docs carry the same list): `branch_masses`/`condition`
take no seat argument — the acting seat is the derived view
`seat_to_move()`, and passing it would store one authority twice;
masses are `u128` with checked arithmetic (the kernel's counting
width), not `BigUint`; `count_cell` stays deferred — Slice F's hand
class turned out to be a ONE-seat predicate, which `marginal` counts
exactly, while Part IV's multi-seat structural cells still have no
consumer (the slice that first needs a cross-seat cell mass gives them
their type). The C1 cache study is done — its §26
coordinates live in the probe README, and its finding is that
within-history reuse is total while cross-history reuse is zero by the
identity law. Slice C is complete (the C2 report discharges the §46
coordinates), and Slices D, E and F are landed (see Status); the next
construction on the ladder is Slice G — the §50 integrated refinement
controller, which unifies the root intervals, grammar/residual regions,
factorized contractions, and consequence-cell bounds this file's slices
built (the existing controller player remains the fallback surface
until arena and conformance gates authorize a default change).

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
  stays slow (§46) — as reported, the classifier IS the bill, 99% of the
  cold pass against 5.9 ms of counting.
- **Boundary obligation (CBS-A6, binding):** any field or belief with
  cross-seat coupling voids Theorem 20.1 until represented as explicit
  additional factors — never silently projected into seat-local form.
  The shipped level-0/level-1 fields are verified seat-local (intake
  companion, code-boundary audit).

## Build order when authorized (§46–48, unchanged)

C0 trivial field (`FixedPreference::lowest_first`) → C1 cached level-0
field with per-hand classification → C2 opening root branch masses. All
three are walked. Recursion (§47's factorized fixed-policy Bellman) only
after one-ply parity and cost are understood — they now are, and Slice D
is in flight. Measured coordinates per §26: contraction arithmetic,
distinct hand materializations, field cost per hand, reuse, support
shrinkage, cache identity cost, integer width, trick-1 memory (C2's
report — an accounting and a measurement, kept apart),
SIMD/GPU/WASM suitability (a measurement coordinate under the ripcord
discipline — CBS-A9, never an authorization).
