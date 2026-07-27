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

This package defines:

- straight points-and-marks Texas 42;
- complete seven-trick hands;
- a configured positive maximum mark bid, with the reachable Straight-auction
  ceiling derived from the one-round progression rules;
- exact declaration-relative rule physics;
- complete-deal support and exact current-remainder support;
- dependent capacity cells and an intensional compatible-world fiber;
- exact fiber feasibility, cardinality, canonical holder-edge reduction, the
  globally representation-minimal exact support normal form, and typed
  transition theorems;
- strict Straight reachability as a legal-prefix image rather than a feasibility
  flag, including exact witness certification, 50 reachable hidden-capacity
  profiles, seven observable lead contexts per declaration, and a proved
  feasible-but-unreachable support;
- a 26–46-bit proved interval for a standalone exact reachable-support
  identifier, while support and reachability require zero supplemental bits
  relative to a certified mechanical state;
- a bounded exact native count dynamic program that does not enumerate worlds;
- an explicit separation between support and sampling law, including an exact
  count-ratio sampler after the uniform law is selected;
- belief, required retained continuation record, field law and latent
  continuation state, utility, and value as typed but unselected objects;
- a direct, unoptimized executable specification;
- two finite verifier entry points for the exhaustible claims and witnesses;
  both are independent of the future reference implementation, while the
  second deliberately reuses abstract-world helpers from the first.

Special contracts and variations are outside the formal object. They are not
partially modeled.

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
- Implementations must conform to those documents.
- Tests and finite verifiers are evidence for finite claims and conformance;
  they are not independent rule authorities.
- A proof-assistant development formalizes the adopted rules and mathematics;
  external verifier output enters only through an internal proof or a proved
  reflection/certificate checker.

## Files

- [`docs/00_THESIS_AND_SCOPE.md`](docs/00_THESIS_AND_SCOPE.md) — thesis, object,
  scope, and exclusions.
- [`docs/10_RULES.md`](docs/10_RULES.md) — normative Straight 42 rules.
- [`docs/20_MATHEMATICAL_FOUNDATION.md`](docs/20_MATHEMATICAL_FOUNDATION.md) —
  definitions, theorems, proofs, counterexamples, and exact boundaries.
- [`docs/30_EXECUTABLE_SPECIFICATION.md`](docs/30_EXECUTABLE_SPECIFICATION.md) —
  domain types, invariants, operations, and pseudocode.
- [`docs/40_CLAIM_STATUS.md`](docs/40_CLAIM_STATUS.md) — mathematical claim
  ledger.
- [`docs/50_CODEX_IMPLEMENTATION_PROMPT.md`](docs/50_CODEX_IMPLEMENTATION_PROMPT.md)
  — first bounded implementation assignment.
- [`docs/55_V06_REVIEW.md`](docs/55_V06_REVIEW.md) — adversarial review of
  v0.6 and the reasons for this revision.
- [`docs/60_PROOF_ASSISTANT_HANDOFF.md`](docs/60_PROOF_ASSISTANT_HANDOFF.md) —
  trust boundary, type design, theorem dependency order, and mechanization
  milestones.
- [`docs/65_MECHANIZATION_LEDGER.md`](docs/65_MECHANIZATION_LEDGER.md) —
  prioritized kernel-proof work queue and theorem dependency map.
- [`docs/70_THREAD_CONTINUITY.md`](docs/70_THREAD_CONTINUITY.md) — continuity
  crosswalk from the founding thread and explicit deferred annexes.
- [`verification/verify_foundation.py`](verification/verify_foundation.py) —
  dependency-free finite checks for the rules, algebra, cells, transitions, and
  history witness.
- [`verification/verify_minimality_and_reachability.py`](verification/verify_minimality_and_reachability.py)
  — dependency-free finite checks for global support minimality, the native
  support census, reachability reductions, bit bounds, and negative witness.
- [`verification/VERIFICATION_OUTPUT.txt`](verification/VERIFICATION_OUTPUT.txt)
  — deterministic output from both verification programs.
- [`provenance/RESEARCH_MANDATE_2026-07-23.md`](provenance/RESEARCH_MANDATE_2026-07-23.md)
  — verbatim founding mandate, retained as non-normative provenance.
- [`CHANGELOG.md`](CHANGELOG.md) — versioned corrections.
- [`MANIFEST.sha256`](MANIFEST.sha256) — SHA-256 digest for every other file in the package.

## Reading order

Read the files in numeric order. Then run:

```text
python verification/verify_foundation.py
python verification/verify_minimality_and_reachability.py
```

The verifier entry points are external proof receipts for the finite
statements they exhaust. They do not replace the supplied mathematical proofs,
are not proof-assistant kernel theorems, and are not source code for the
reference implementation assignment. The second entry point imports a small
set of abstract-world helpers from the first; independence is from the future
implementation, not between the two scripts.

## Claim discipline

Substantive statements are classified as:

- **Definition**
- **Adopted rule**
- **Clarification**
- **Theorem — proved mathematically**
- **Lemma — proved mathematically**
- **Theorem — exhaustive finite verification**
- **Finite verification receipt — stated corpus**
- **Proposition — proved under explicit assumptions**
- **Corollary — proved mathematically**
- **Corollary / structural synthesis**
- **Constructed counterexample**
- **Boundary**
- **Conjecture**
- **Unresolved**

“Implemented,” “tested,” “fast,” and “useful” are not mathematical statuses.

## Correctness before tractability

The mathematical object is not resized to fit present computation.

The current-remainder fiber is defined intensionally. Extensional enumeration
is a query on it and can contain as many as 399,072,960 native worlds. An
implementation may exhaust time or memory while enumerating; it may not
silently cap, sample, truncate, or alter the fiber.

Unrestricted native counting is different: the three-seat capacity dynamic
program computes it exactly with at most 512 occupancy states over the entire
run, 1,533 candidate-holder checks, 1,344 capacity-eligible extension updates,
and 48 live states in any one layer. Arbitrary predicate-restricted counting
remains a separate exact query and may have a different computational
boundary.

Sampling additionally requires an explicitly named law. Uniformity is not
inferred from support. Once a uniform fiber law is selected, exact successor
counts determine holder marginals and a sequential exact sampler without
materializing the fiber.

An explicit predicate may form an exact restricted object. The predicate is
part of the query, never an invisible horizon.

## Domain discipline

Three objects that are easy to conflate are deliberately separate:

```text
complete initial deal
        ↓ remainder map under a fixed public history
current hidden-remainder world
        ↓ collection of compatible worlds
current-remainder fiber
```

Posterior belief is a measure on compatible latent worlds and can be pushed
forward to the current fiber. It is not the fiber itself.

Support feasibility is also not legal reachability. Hall's theorem answers
whether a current hidden assignment exists. Straight reachability asks whether
a valid deal and legal actor-attributed play prefix can generate the exact
support. Certified internal states carry reachability by construction; arbitrary
external states need an exact replayable witness or exhaustive validation.

## Version

`v0.7` is the proof-assistant boundary revision of the reachability-minimized
foundation. It retains v0.6's coarsest exact deterministic support quotient,
determinate/binary/ternary normal form, one-assignment SCC compiler, exact
81-bit full-schema census, strict Straight reachability image, 50 hidden
capacity profiles, seven leadable contexts per declaration, projected void
schedule, exact legal-witness validator, feasible-but-unreachable witness, and
26–46-bit standalone reachable-support interval.

The revision changes the semantic type boundary rather than those results:
reachability proofs are proof-irrelevant; support objects are derived views of
one mechanical source of truth; the feasible normal form has a total
well-formedness contract; and external verification is explicitly separated
from proof-assistant kernel proof. The exact reachable-support cardinality
remains explicitly unresolved.
