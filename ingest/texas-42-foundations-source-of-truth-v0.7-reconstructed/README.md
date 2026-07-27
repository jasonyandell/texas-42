# Texas 42 Foundations

A self-contained mathematical and executable specification of straight
points-and-marks Texas 42.

> **A physical domino is a stable node, not a stable strategic type.
> Declaration selects the relational world in which that node acts. A hand is
> a controlled marked region embedded in that world and coupled to an exact
> hidden complement. A player's information determines an intensional fiber
> of compatible objective worlds; public history can place unequal weight on
> those worlds without changing the fiber. A move spends and relocates one
> controlled node, transforming both the physical and epistemic situation. Its
> value is therefore derived from that whole transition, not intrinsic to the
> domino.**

## Status

This package defines and checks:

- straight points-and-marks Texas 42 with complete seven-trick hands;
- a configured positive maximum mark bid and the exact reachable auction
  ceiling induced by the one-round progression rules;
- the 28-domino universe as `Sym²({0,...,6})`, equivalently the edges of
  complete looped `K7`;
- count as the sum-five/sum-ten antidiagonal decoration;
- exact declaration-relative legality and trick order;
- exactly three unscored declaration-mechanics classes: pip trump,
  doubles-trump, and no-trump;
- the narrower scored `2 <-> 3` transport;
- complete-deal support, current-remainder support, and their typed relation;
- dependent capacity cells and the exact intensional compatible-world fiber;
- Hall feasibility, exact counting, and exact uniform sampling after a law is
  explicitly selected;
- the globally representation-minimal exact support normal form;
- strict Straight reachability as the image of legal prefixes, including both
  complete-deal and smaller symbolic-trace certificates;
- exact support transition directly on the minimal normal form by
  force/delete/contract/reduce matching operations;
- monotone holder-edge deletion and the exact 63-edge whole-hand budget;
- an exact folded current-trick residue and actor recovery from capacities;
- utility-specific score accumulators rather than one universal score payload;
- a reduced exact viewer play/support kernel;
- belief, retained evidence, field state, utility, and value as separately
  typed objects;
- an exact legal 90-world history counterexample in which identical rule
  support and identical posterior support still require opposite leads;
- rotations and orientation-transporting reflections as scoped coordinate
  gauges; and
- future equivalence as the unique smallest deterministic exact transition
  machine for each named finite output contract.

Special contracts and variations are outside the formal object. They are not
partially modeled.

## Exact factorization

For a fixed contracted-hand decision problem, the reduced viewer-relative
physical/support kernel has the form

```text
(declaration,
 viewer remaining hand,
 minimal exact hidden support,
 folded trick or boundary leader,
 named utility accumulator)
```

The exact strategic state additionally retains:

```text
required viewer-known evidence
+ augmented belief over hidden remainders and coupled latent continuation state
```

The continuation field, utility, and allowed decision strategy are typed
parameters to value. A complete objective world remains the latent physical
witness beneath the viewer-relative representation.

## Authority order

```text
prose rules
    ↓
mathematical foundation
    ↓
executable specification
    ↓
implementation
    ↓
tests and finite proof receipts
```

A lower layer may expose an ambiguity or error in a higher layer, but it may
not silently redefine it.

- `docs/10_RULES.md` is the normative rules profile.
- `docs/20_MATHEMATICAL_FOUNDATION.md` formalizes the induced object.
- `docs/30_EXECUTABLE_SPECIFICATION.md` translates the mathematics into
  computing contracts.
- `docs/40_CLAIM_STATUS.md` records exactly what is claimed and under which
  assumptions.
- Tests and finite verifiers are evidence for finite claims and conformance;
  they are not independent rule authorities.

## Files

- [`docs/00_THESIS_AND_SCOPE.md`](docs/00_THESIS_AND_SCOPE.md) — thesis, object,
  scope, and exclusions.
- [`docs/10_RULES.md`](docs/10_RULES.md) — normative Straight 42 rules.
- [`docs/20_MATHEMATICAL_FOUNDATION.md`](docs/20_MATHEMATICAL_FOUNDATION.md) —
  definitions, theorems, proofs, counterexamples, and exact boundaries.
- [`docs/30_EXECUTABLE_SPECIFICATION.md`](docs/30_EXECUTABLE_SPECIFICATION.md) —
  domain types, invariants, operations, and pseudocode.
- [`docs/40_CLAIM_STATUS.md`](docs/40_CLAIM_STATUS.md) — claim-status ledger.
- [`docs/50_CODEX_IMPLEMENTATION_PROMPT.md`](docs/50_CODEX_IMPLEMENTATION_PROMPT.md)
  — first bounded implementation assignment.
- [`docs/60_PROOF_ASSISTANT_KERNEL.md`](docs/60_PROOF_ASSISTANT_KERNEL.md) —
  finite formalization kernel, theorem dependency order, and excluded
  generalizations.
- [`verification/verify_foundation.py`](verification/verify_foundation.py) —
  finite checks for rules, declaration algebra, cells, typed transitions, and
  the history witness.
- [`verification/verify_minimality_and_reachability.py`](verification/verify_minimality_and_reachability.py)
  — finite checks for support minimality, compiled forms, reachability, bounds,
  and the feasible-but-unreachable witness.
- [`verification/verify_reduced_kernel.py`](verification/verify_reduced_kernel.py)
  — finite checks for the reduced support/play kernel, declaration transports,
  folded tricks, monotone support dynamics, symbolic legal traces, gauges, and
  future-equivalence minimization.
- [`verification/audit_package.py`](verification/audit_package.py) — structural
  package audit.
- [`verification/VERIFICATION_OUTPUT.txt`](verification/VERIFICATION_OUTPUT.txt)
  — deterministic output from the mathematical verifiers.
- [`verification/AUDIT_OUTPUT.txt`](verification/AUDIT_OUTPUT.txt) — deterministic
  structural-audit output.
- [`MANIFEST.sha256`](MANIFEST.sha256) — SHA-256 digest for every other package
  file.

## Reading order

Read the documents in numeric order. Then run:

```text
python verification/verify_foundation.py
python verification/verify_minimality_and_reachability.py
python verification/verify_reduced_kernel.py
python verification/audit_package.py
```

The verifiers are proof receipts for the finite statements they exhaust. They
do not replace the supplied general proofs and are not source code for the
reference implementation assignment.

## Claim discipline

Substantive statements are classified as definitions, adopted rules,
clarifications, proved theorems and lemmas, exhaustive finite verifications,
stated-corpus verification receipts, propositions under explicit assumptions,
proved corollaries, structural syntheses, counterexamples, boundaries,
conjectures, or unresolved claims.

“Implemented,” “tested,” “fast,” and “useful” are not mathematical statuses.

## Correctness before tractability

The mathematical object is not resized to fit present computation.

The current-remainder fiber is defined intensionally. Extensional enumeration
is a query and may contain as many as 399,072,960 native worlds. Resource
failure is permitted; silent capping, sampling, or truncation is not.

Unrestricted native counting is different. The exact three-seat occupancy
dynamic program uses at most:

- 512 occupancy states over a complete count;
- 1,533 candidate-holder checks;
- 1,344 capacity-eligible extension updates; and
- 48 live states in one layer.

Sampling additionally requires a named law. Once the uniform fiber law is
selected, exact successor counts determine holder marginals and a sequential
exact sampler without materializing the fiber.

The support normal form is semantically minimal for exact support. This does
not imply one byte-minimal or runtime-minimal encoding without a cost model.
For a complete deterministic transition machine, minimality is relative to a
named output contract and is given by its future-equivalence quotient.

## Important boundaries

- Support feasibility is not legal-prefix reachability.
- Standalone support is not a complete game state.
- Exact support *is* a closed support-transition state when declaration and the
  full typed public observation context are supplied.
- The reduced viewer kernel is proved exact but is not yet proved equal to a
  selected future-equivalence minimum.
- The exact cardinality of standalone Straight-reachable support remains open
  inside the proved 26–46-bit interval.
- A support fiber selects no probability law.
- Full public and private history remains available in the retained evidence
  factor whenever a selected field, belief model, or utility reads it.
- General measurable-space extensions are not required by the finite Straight
  game and are outside the first proof-assistant kernel.

## Version

`v0.7` is the reduced play/support foundation. It promotes the globally minimal
support normal form from a static quotient to an exact dynamic state, proves
monotone matching-minor evolution and the 63-edge hand budget, folds the open
trick, separates utility-specific score residue, adds symbolic trace
reachability, identifies the three unscored declaration classes, introduces an
orientation-aware dihedral gauge, and defines the output-relative global
transition minimum by future equivalence.
