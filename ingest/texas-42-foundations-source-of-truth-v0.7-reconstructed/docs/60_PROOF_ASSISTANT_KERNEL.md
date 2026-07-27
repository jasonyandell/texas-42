# Proof-Assistant Kernel Map

## 0. Purpose

This document identifies the smallest finite mathematical kernel from which the
Straight 42 foundation can be machine-checked. It is not a second rules
specification, implementation architecture, issue list, or research roadmap.

The normative order remains:

```text
rules -> mathematics -> executable specification -> implementation
```

The proof assistant should formalize the mathematical objects and theorems. It
must not infer rules from verifier code or from a particular executable
encoding.

## 1. Formalization boundary

The first kernel is entirely finite.

Required:

- finite pip, domino, seat, team, declaration, context, bid, and action types;
- finite sets and finite maps;
- exact natural-number and integer arithmetic;
- finite probability distributions with exact rational weights where belief is
  included;
- finite partial deterministic transition systems;
- finite bipartite capacitated matching.

Not required by the first kernel:

- standard-Borel spaces;
- arbitrary measurable kernels;
- Radon–Nikodym derivatives;
- measurable selection;
- arbitrary infinite private-signal models;
- a full infinite-horizon match value across unbounded repeated pass-outs;
- byte layouts, GPU packing, or hardware cost models.

Those generalizations may be stated in prose, but the finite Straight game does
not depend on them.

## 2. Canonical finite types

A proof-assistant-neutral signature is:

```text
Pip          := Fin 7
Seat         := Fin 4
Team         := Fin 2
Domino       := Sym2 Pip
Declaration  := PipTrump Pip | DoublesTrump | NoTrump
Context      := Natural Pip | Called
```

`Sym2 Pip` is the quotient of `Pip × Pip` by endpoint swap, or an equivalent
canonical ordered-pair subtype. The theorem-facing type must carry unordered
physical identity; an integer identifier is a later encoding.

Define:

```text
endpoints       : Domino -> multiset Pip of cardinality 2
contains        : Domino -> Pip -> Prop
isDouble        : Domino -> Prop
pipSum          : Domino -> Nat
highPip         : Domino -> Pip
countPoints     : Domino -> Nat
partner         : Seat -> Seat
teamOf          : Seat -> Team
clockwiseNext   : Seat -> Seat
```

The looped-graph presentation is an isomorphism, not an extra primitive:

```text
Domino ~= edges of complete graph on Pip with one loop at each vertex
naturalSuit p = closedStar p
```

## 3. Dependency spine

The formal theorem dependency should follow this order.

### K0. Finite universe

Definitions and claims:

- `ALG-01`, `ALG-02`, `ALG-03`, `ALG-20`;
- canonical unordered domino identity;
- decidable equality for every finite base type;
- enumeration completeness and no duplicates.

Primary results:

```text
card Domino = 28
card (naturalSuit p) = 7
naturalSuit p ∩ naturalSuit q = {p:q} for p != q
```

### K1. Count decoration

Definitions and claims:

- `ALG-04`, `ALG-21`.

Primary results:

```text
countPoints d = pipSum d when pipSum d in {5,10}, else 0
sum countPoints over Domino = 35
```

### K2. Declaration mechanics

Definitions:

```text
calledSet       : Declaration -> Finset Domino
poweredSet      : Declaration -> Finset Domino
effectiveSuits  : Declaration -> Domino -> Finset Context
ledContext      : Declaration -> Domino -> Context
follows         : Declaration -> Domino -> Context -> Prop
rank            : Declaration -> Domino -> Rank
tier            : Declaration -> Context -> Domino -> Fin 3
trickKey        : Declaration -> Context -> Domino -> TrickKey
```

Claims:

- `ALG-05` through `ALG-10`;
- effective absorption;
- follow equals effective-suit membership;
- lead always has nonzero tier.

`Rank` should be an inductive ordered type such as:

```text
finite Nat rank | Top
```

rather than a floating or sentinel integer.

### K3. Trick order

Definitions:

```text
Play        := Seat × Domino
Trick       := vector Play 4 with clockwise actor proof and distinct-tile proof
resolveTrick : Declaration -> Trick -> TrickResult
```

Claims:

- `ALG-11`, `ALG-13`;
- unique winner;
- exact contextual `BEATS` relation;
- exact trick points.

The finite verifier's 737,100 cases are an independent receipt. The formal
proof should use injectivity of nonzero trick keys, not case enumeration.

### K4. Declaration transports and gauges

Define the ordered-complement transport between any two pip trumps.

Claims:

- `ALG-17` through `ALG-24`;
- all pip-trump layers are isomorphic as unscored mechanics;
- exactly three unscored mechanics classes;
- only identity and `2 <-> 3` preserve count under endpoint permutations;
- the scored transport is scoped to declarations 2 and 3.

Keep two structures distinct:

```text
UnscoredMechanics
ScoredMechanics = UnscoredMechanics + count decoration
```

This prevents a proof from accidentally requiring literal numeric rank labels
to be preserved.

### K5. Objective contracted-hand game

Definitions:

- complete ordered deal;
- contract and declaration;
- remaining hands;
- boundary leader or current trick;
- legal action;
- atomic play transition;
- trick reward;
- terminal hand.

Claims:

- `PLAY-01` through `PLAY-11`;
- exact follow-if-possible legality;
- location conservation;
- strict decrease of total remaining tiles;
- seven tricks and 42 total points;
- finite perfect-information history tree.

Bidding and match progression may be formalized before or after this layer, but
no support theorem depends on a solver or oracle.

### K6. Information and current remainder

Definitions:

```text
PublicPlayPrefix
ViewerHand
CompleteDealSupport
RemainderWorld
remainderMap
```

Claims:

- `INFO-01` through `INFO-12`;
- complete deals and current remainder worlds are different types;
- the fixed-history remainder map and reconstruction map are inverse in scope;
- support is not belief.

The actor attribution in the fixed public prefix is essential to
reconstruction.

### K7. General finite capacitated matching kernel

Before specializing to three hidden seats, define:

```text
CellSystem Holder Tile
fiber : CellSystem -> Finset (Assignment Holder Tile)
```

with:

- one owner per tile;
- exact holder quotas;
- allowed holder edges.

Claims:

- `CELL-01` through `CELL-10I1` and `CELL-09A`;
- capacitated Hall theorem;
- exact coefficient and deletion recurrence;
- exact occupancy-vector dynamic program.

A proof assistant may import an existing finite Hall theorem if its statement
matches the labeled-slot reduction. Otherwise prove the slot expansion
explicitly.

### K8. Straight cell losslessness

Specialize holders to the three seats hidden from one viewer.

Define the rule-derived cell transition from:

- common unseen pool;
- hidden-seat capacities;
- public void contexts.

Claims:

- `CELL-04` through `CELL-08`;
- the exact cell losslessness theorem;
- no surviving positive follower clause;
- fixed-history deal/remainder bijection.

The induction must separate:

1. viewer action;
2. hidden lead;
3. hidden successful follow;
4. hidden slough.

No theorem from this layer extends automatically to special contracts.

### K9. Marginal support and semantic normal form

Definitions:

```text
marginalHolders
supportReduction
SupportNormalForm := Empty | Feasible FeasibleNormalForm
```

The feasible form contains:

- certain tiles by holder;
- determinate, binary, or ternary residual ambiguity;
- only essential ternary exclusions.

Claims:

- `CELL-10J` through `CELL-29`;
- fixed-schema reduction laws;
- global support quotient theorem;
- one-assignment SCC compiler;
- strict Hall irreducibility;
- essential exclusions;
- exact 81-bit standalone full-schema census as a finite arithmetic theorem.

The central quotient statement is extensional:

```text
normalize C1 = normalize C2  <->  fiber C1 = fiber C2
```

For every exact deterministic support representation with a decoder, prove the
unique factor map onto `SupportNormalForm`.

### K10. Dynamic support

Definitions:

```text
HiddenObservation := Lead | Follow Context | Slough Context
advanceSupport : SupportNormalForm -> Declaration -> HiddenObservation
                 -> Option SupportNormalForm
```

Claims:

- `TRANS-01` through `TRANS-14`;
- support-normal dynamic sufficiency;
- force/delete/contract/reduce equals extensional conditioning and pushforward;
- marginal holder edges only disappear;
- ambiguity phase only decreases;
- initial 63-edge budget and at most 42 live-tile edge deletions.

The direct proof should use the typed inverse transition. The finite verifier's
small-domain exhaustion remains a receipt, not the general proof.

### K11. Symbolic reachability

Definitions:

```text
SymbolicViewerState
SymbolicPublicAction
symbolicStep
acceptedTrace
```

The state contains viewer hand, exact support, declaration, and exact public
play residue sufficient for actor order and trick mechanics.

Claims:

- `REACH-01` through `REACH-16`;
- exact reachable capacity profiles;
- seven leadable contexts;
- projected schedule theorem and its scope;
- feasible-but-unreachable witness;
- symbolic trace equivalence;
- finite graded symbolic support DAG.

The soundness proof should construct a complete initial deal by reversing the
typed support transitions from one final remainder world. The completeness
proof carries one realizing deal forward.

### K12. Folded play/support kernel

Definitions:

```text
OpenTrickFold := led context × competitive ordinal × winner × pending count
TrickResidue  := Boundary leader | Open OpenTrickFold
ReducedViewerKernel U
```

Claims:

- `PLAY-12` through `PLAY-17`;
- current-trick fold congruence;
- actor and leader recovery from remaining capacities;
- total banked-score recovery;
- reduced-kernel sufficiency relative to a named utility accumulator.

The unresolved pending-count field ranges only through
`{0,5,10,15,20,25}`. A fourth-play transition may compute 30 count points in a
local widened value before emitting and clearing the trick.

### K13. Seat-frame gauges

Claims:

- `SYM-01` through `SYM-04`;
- clockwise rotations;
- bidder anchoring;
- reflection failure in the fixed clockwise game;
- exact `D4` coordinate gauge only in the oriented-frame family.

Orientation must be a transported field. Reflection is not to be asserted as a
fixed-clockwise automorphism.

### K14. Future-equivalence minimum

For a finite reachable partial deterministic machine with finite action
alphabet and named finite output contract, define equality of complete future
legality/output responses.

Claims:

- `QUO-09`, `QUO-10`, `QUO-11`;
- right congruence;
- well-defined quotient;
- exact realization;
- every other exact deterministic realization maps onto the quotient;
- uniqueness up to isomorphism among minimum reachable realizations.

This is the cost-model-independent minimum for a complete deterministic
transition contract. The selected output contract is part of the theorem.

### K15. Finite belief layer

Only after the physical/support kernel is closed, define exact finite
probability distributions over augmented latent worlds.

Claims suitable for the first finite formalization:

- finite Bayes conditioning at positive-probability observations;
- pushforward through typed remainder and field-state kernels;
- support-versus-measure separation;
- uniform rule-only fiber law when explicitly selected;
- exact history witness and its posterior action flip.

General measurable-space statements remain outside this kernel.

## 4. Quotients that must not be conflated

The formalization should keep separate quotient relations for:

1. unordered endpoint identity of one domino;
2. local slot-order gauge inside a known hand;
3. pip-trump unscored mechanics transport;
4. scored `2 <-> 3` transport;
5. seat rotation and oriented-frame reflection gauges;
6. equality of exact support fibers;
7. future equivalence for a named output contract;
8. value equality for one named field and utility.

No one quotient implies another without a theorem.

## 5. Recommended theorem statement style

Every theorem should expose:

```text
inputs
scope assumptions
exact conclusion
whether the result is extensional, intensional, or representation-relative
```

In particular:

- “minimal” must name its representation class or output contract;
- “support” must name complete-deal or current-remainder support;
- “state” must name objective, viewer mechanical/support, evidence, belief, or
  strategic state;
- “symmetry” must name every transported sort and relation;
- “reachable” must mean legal ancestry, not Hall feasibility.

## 6. Computational reflection boundary

Finite enumeration may be used inside the proof assistant for closed finite
facts, including:

- cardinalities;
- declaration-class census;
- small counterexamples;
- exact integer bounds;
- checking a concrete 90-world witness.

The general support, transition, minimality, and symbolic-reachability theorems
should not be accepted solely because an external Python script returned
`PASS`. Their mathematical proofs are part of the kernel.

## 7. Extraction boundary

A proof assistant may extract executable definitions for:

- declaration algebra;
- trick resolution;
- Hall feasibility;
- exact support normalization;
- direct support transition;
- folded trick transition;
- symbolic trace validation; and
- finite future-equivalence partition refinement.

Extraction does not establish a byte-minimal GPU encoding. Packed layouts,
memo tables, parallel kernels, and caches are separate refinements that must
prove correspondence to the extracted semantics.

## 8. Kernel completion criterion

The finite kernel is closed when the assistant checks, from the normative
finite definitions:

1. every theorem in K0–K14 marked “proved” in the claim ledger;
2. every stated counterexample as a concrete well-typed witness;
3. the exact dependency boundary of every theorem;
4. the equivalence between extensional support and its normal form;
5. the exact symbolic support transition and reachability theorems;
6. reduced-kernel sufficiency; and
7. output-relative future-equivalence minimality.

The finite belief layer K15 may then be added without changing the physical or
support kernel.
