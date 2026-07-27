# Proof-Assistant Handoff

**Target:** a kernel-checked formalization of the Straight Texas 42 foundation  
**Revision:** v0.7  
**Design stance:** finite combinatorial core first; general extensions only after closure

## 1. What is being formalized

The primary theorem object is not a solver and not an AI representation. It is
the game and its exact information structure:

1. the double-six universe;
2. nine Straight declaration algebras;
3. legal auction, declaration, contract, and full-play transition;
4. objective and viewer-relative state;
5. exact rule-derived hidden support;
6. belief filtering under a named field;
7. the ambient marked-hand view;
8. utility-relative value and valid quotients.

The intended result is a dependency-closed library in which the central claims
are theorems of the assistant, not statements justified only by Python output.

## 2. Trust boundary

Use the following hierarchy.

```text
adopted Straight rule profile
        ↓ definitions
kernel-checked mathematical theorems
        ↓ proved refinement
extracted/reference executable functions
        ↓ conformance
external Python receipts and production implementations
```

The two Python verifier entry points are valuable regression oracles and
witness generators. They are outside the proof kernel. A finite result becomes
a kernel theorem only when one of these holds:

- it has a direct formal proof;
- an internal Boolean decision procedure is proved equivalent to the
  proposition and kernel evaluation closes the case;
- a reflection procedure is proved sound and the kernel checks its certificate.

Do not import `PASS` output as an axiom in the final foundation.

## 3. Recommended formalization layers

The order below prevents probability, implementation encoding, or solution
concepts from contaminating the rule core.

### Layer A — finite algebra

Define:

```text
Pip          finite type of cardinality 7
Domino       canonical unordered pair of pips with repetition
Seat         cyclic finite type of cardinality 4
Team         finite type of cardinality 2
Declaration  seven pip trumps | doubles trump | no trump
LedSuit      eight declaration-relative contexts
```

For proof engineering, a canonical pair `(high, low)` with proof `low <= high`
is usually simpler than a quotient by pair permutation. Prove equivalence to
the multiset definition once; use the canonical representation thereafter.

Define count, called set, powered set, effective membership, led context,
follow relation, rank, tier, trick key, `BEATS`, and `THREAT`.

Close these theorems before defining game state:

1. 28 dominoes;
2. natural covering and pair intersections;
3. effective membership bounds;
4. follow exactness;
5. lead has nonzero tier;
6. injectivity in the winning tier;
7. unique trick winner;
8. score labels total 35;
9. contextual `BEATS` exactness;
10. threat monotonicity;
11. scoped `2 <-> 3` transport.

### Layer B — auction, contract, and objective hand

Model the auction as a finite state machine with one action per seat. Keep
`maxMarkBid` and match target as positive configuration values. Prove the
reachable mark ceiling `min(maxMarkBid, 5)` from the one-round progression
rules.

Prefer phase-indexed state over one record with a phase tag and many partial
fields. A suitable shape is:

```text
HandLifecycleState :=
    | auction(AuctionState)
    | declarationPending(AuctionWin, DealAttemptResidue)
    | play(CertifiedContractedPlayState)
    | handComplete(CompletedHand)
```

At match level:

```text
MatchLifecycleState :=
    | active(MatchState, HandLifecycleState)
    | complete(FinalMatchState)
```

This makes illegal combinations unconstructible rather than merely rejected by
validators.

Prove:

- legal-action characterization;
- deterministic transition;
- one tile leaves one remaining hand per play;
- unique trick resolution;
- seven tricks and 28 plays;
- total 42 points;
- finite graded DAG for a fixed contracted hand;
- deterministic contract and mark award;
- early settlement only as a separately proved quotient.

### Layer C — histories and information

Define primitive events first. Derive scores, winners, voids, and settlement
from the event history.

Use full perfect-recall information as the safe object:

```text
InformationState(viewer) :=
    viewer's sequence of private deal observations
    × complete public event history
```

Then define the mechanical projection. Do not identify projection equality
with information-state equality.

### Layer D — cells, fiber, and exact support quotient

For a fixed viewer, define the three hidden seats in clockwise relative order.
From a mechanical state derive:

```text
unseenPool
hiddenCapacity
rulePossibleHolderSet
```

Do not store these as independent semantic fields.

Define the current remainder fiber as the finite set of three labeled hands
obeying subset, capacity, disjointness, and conservation.

Prove in this order:

1. every legal prefix projects to a feasible fiber;
2. upper-bound-only support update;
3. losslessness by induction over legal public play prefixes;
4. deal-to-remainder correspondence;
5. Hall feasibility;
6. typed hidden-play predecessor/successor bijection;
7. viewer-play identity map;
8. exact cardinality dynamic program and soundness;
9. marginal holder support and reduction;
10. support-normal-form encode/decode bijection;
11. global representation-minimality quotient;
12. strict Straight reachability as a predicate on mechanical states;
13. feasible does not imply reachable.

## 4. Reachability must be proof-irrelevant

Formalize reachability as a proposition, not a certificate-valued state field.

```text
ReachablePlay : ContractedPlayState -> Prop
ReachablePlay s :=
    exists origin legalPrefix,
        Replay origin legalPrefix = s

CertifiedContractedPlayState :=
    subtype ReachablePlay
```

Required semantic law:

```text
project x = project y  ->  x and y denote the same game state
```

In proof assistants whose propositions are proof-irrelevant, subtype equality
usually follows from equality of the state projection. Otherwise prove or use
an extensionality theorem that ignores the witness. Never define game equality,
hashing, serialization, or transition by proof-term identity.

Keep two separate notions:

```text
ReachabilityWitness  # replay/audit data
Reachable(s)          # proposition used by the formal game
```

The witness can be used to construct the proof and then erased.

`ReachabilityOuterNecessaryProfile` is not a proof of reachability. It is an
element of a verified necessary outer language used for an upper bound. Only a
legal replay witness or an internally proved equivalent decision procedure may
construct `Reachable`.

## 5. Semantic state and derived views

The following are deterministic functions of the mechanical state in the
reference coordinate:

```text
deriveRuleCells
supportReduction
compileExactSupport
remainderFiber
nativeMarkedHandView
```

A proof assistant should represent them as functions or definitions, not fields
of the semantic state. If an executable refinement caches them, define a
separate structure:

```text
CompiledView :=
    state
    cache
    proof(cache = derive(state))
```

and prove that projection to `state` is a semantic equivalence. Cache contents
must not enter the information partition.

## 6. Formal support-normal-form type

Define the feasible normal form as an indexed or validated type, not an
unchecked record.

Common invariants:

- exactly the three viewer-relative hidden seats;
- pairwise disjoint certain-holder sets;
- ambiguous pool disjoint from every certain set;
- every element is a valid domino;
- reconstructed hidden pool has at most 21 tiles;
- reconstructed capacities lie in `0..7` and sum to pool size.

Branch invariants:

```text
Determinate:
    ambiguous pool empty
    all residual capacities zero

Binary:
    one inactive seat
    nonempty ambiguous pool
    both active residual capacities positive and sum to pool size
    every ambiguous tile has exactly the two active holders

Ternary:
    all three residual capacities positive and conserve pool size
    each ambiguous tile excludes zero or one seat
    strict singleton Hall inequalities
```

Prove:

```text
wellFormed (compile cells)                  when cells feasible
decode (compile cells) = supportReduction cells
compile (decode normalForm) = normalForm    for well-formed normal forms
fiber (decode normalForm) = decodedFiber normalForm
```

The global minimality theorem should then be stated as a quotient/factorization
theorem over extensional support equality, not as a byte-layout claim.

## 7. Probability layer: finite first

The native Straight game has a finite deal domain and finite action histories
for one contracted hand. Start with finite probability mass functions.

Define:

```text
Prior              PMF on complete deals or augmented latent worlds
PolicyKernel       information state -> PMF on legal actions and field successor
Posterior          normalized restriction/reweighting
RemainderBelief    pushforward posterior through remainder map
```

Prove:

- legality/support restriction;
- likelihood product over public actions;
- one-step Bayesian filter;
- pushforward to current remainder worlds;
- physics-only posterior under the named uniform chance assumptions;
- exponential tilt in the finite positive-mass domain;
- forced public action has world-constant likelihood;
- own action is not evidence about the world to the acting seat when conditioned
  as specified.

Only after the finite layer is complete should a separate extension introduce
countable or standard-Borel latent state. The finite Texas 42 theorem should
not depend on disintegration, Ionescu–Tulcea construction, or measurable
selection.

## 8. Strategic state and value

For the finite fixed-field continuation, define an augmented latent world that
contains every inherited field-state component needed for future policy. The
exact decision object is then:

```text
mechanical state
retained continuation record
posterior PMF on augmented worlds
field transition kernel
utility
```

Value is a derived recursion. Prove deterministic best-response existence by
finiteness of the contingent policy set or by backward induction over the
finite information-state DAG.

Do not include belief in the observable extensive-form information-set key.
Belief is induced by prior, strategy/field, and history. A belief-state policy
is an equivalent derived representation only after its sufficiency is proved.

## 9. Migrating the finite receipts

Recommended kernel routes:

| Receipt | Preferred formal route |
|---|---|
| 28 dominoes, 35 count points | direct finite computation plus theorem |
| 737,100 unique-winner/prose agreement cases | internal decision procedure and reflection |
| auction history counts and reachable ceiling | mathematical recurrence; finite computation as regression |
| Hall abstract corpus | prove Hall generally; retain corpus only as implementation test |
| normal-form corpus | prove encode/decode theorem generally; reflect finite native census separately |
| 50 capacity profiles | direct combinatorial proof plus finite enumeration theorem |
| seven leadable contexts per declaration | direct algebra theorem |
| feasible-but-unreachable support | internal concrete witness proof |
| 90-world posterior reversal | define the concrete histories/field, compute finite PMFs and Q values in kernel or via proved reflection |
| 81-bit support census | proved enumerator plus injectivity/surjectivity and reflected cardinality; otherwise keep external-only status |
| 26–46-bit reachable interval | formal lower/upper injections; external count remains receipt until reflected |

The 90-world witness is especially important. It should become a named theorem,
not remain only a test fixture, because it guards the central distinction
between support equality and strategic-state equality.

## 10. Runtime encodings come after semantic closure

Use finite sets, vectors, and functions in the first proof. Introduce 28-bit
masks, packed ranks, hashes, and canonical byte encodings only through proved
refinement maps.

For each optimized representation require:

```text
decode(encode(x)) = x
operation_refined(encode(x)) = encode(operation(x))
```

where applicable. Hashes are identifiers only after collision freedom on the
exact finite domain is proved; otherwise they are lookup accelerators, not
mathematical equality.

## 11. Deliberately deferred topics

Do not block the foundation on:

- CFR or equilibrium convergence;
- centralized team-coordinator constructions;
- neural representation architecture;
- empirical role-feature smoothness;
- special contracts;
- repository-specific implementation correspondence;
- exact reachable-support cardinality;
- a globally minimal complete mechanical state;
- general standard-Borel field models.

Each can be added as a module after the finite core exposes stable definitions.

## 12. Suggested milestone sequence

### Milestone 1 — algebra closed

All declaration, follow, rank, unique-winner, score, and transport theorems
kernel checked.

### Milestone 2 — one full hand closed

Auction, contract, objective play, 42-point conservation, and settlement
formalized. No hidden information yet.

### Milestone 3 — exact support closed

Information projection, cell losslessness, Hall feasibility, typed refinement,
and proof-irrelevant reachability formalized.

### Milestone 4 — minimal support quotient closed

Normal-form well-formedness, compile/decode equivalence, and quotient theorem
formalized.

### Milestone 5 — finite belief closed

Bayes filter, pushforward, tilt, and forced-action results formalized over PMFs.

### Milestone 6 — strategic boundary closed

Finite fixed-field best response and the 90-world coordinate-only-value
counterexample kernel checked.

### Milestone 7 — match and extensions

Match state, repeated all-pass attempts, optional almost-sure termination,
implementation refinements, and later game-theory annexes.

## 13. Acceptance standard

The proof-assistant project is ready to call itself the foundation when:

1. every adopted rule is represented by an explicit definition or parameter;
2. every core theorem in the claim ledger is either kernel proved or still
   honestly marked external/open;
3. no semantic equality depends on a proof, replay witness, cache, hidden
   provenance, or hash;
4. cells and fibers are derived from one mechanical source of truth;
5. information-state equality is not silently coarsened to mechanical equality;
6. the 90-world witness is internalized;
7. extraction/refinement theorems separate mathematical state from optimized
   runtime encoding.

## 14. Suggested module topology

Keep dependency direction visible in the file graph. A proof-assistant-specific
spelling may vary, but the semantic modules should be close to:

```text
Texas42/Basic                 finite pips, dominoes, seats, teams
Texas42/Declaration           called/powered/effective suits
Texas42/Trick                 follow, rank, winner, points, BEATS/THREAT
Texas42/Deal                  deals and uniform finite chance profile
Texas42/Auction               legal one-round auction and contract
Texas42/PlayState             phase-indexed objective state and transition
Texas42/History               primitive events and perfect recall
Texas42/Mechanical            viewer projection and derived rule cells
Texas42/Fiber                 remainder worlds, Hall, count recurrence
Texas42/SupportReduction      marginal holder relation
Texas42/SupportNormalForm     determinate/binary/ternary quotient
Texas42/Reachability          inductive/replay predicate and exact witnesses
Texas42/Belief/Finite         finite PMFs, Bayes, pushforward, tilt
Texas42/Strategic             fixed-field value and best response
Texas42/Symmetry              slot gauge, rotation, scoped pip transport
Texas42/Witnesses             negative examples and 90-world theorem
Texas42/Refinement            optional bitmask/packed executable refinement
```

`Basic` through `Reachability` should not import `Belief` or `Strategic`.
`SupportNormalForm` should not import a solver. `Refinement` may import the
semantic modules, never the reverse. This import discipline is the formal
version of the package's authority order.
