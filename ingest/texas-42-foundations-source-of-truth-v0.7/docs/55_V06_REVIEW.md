# Review of Texas 42 Foundations v0.6

**Review date:** 2026-07-26  
**Disposition:** preserve the mathematical core; issue a focused v0.7 before mechanization

## 1. Verdict

v0.6 did **not** drift away from the founding object. It strengthened it.

The independent-project move was correct. The package now separates the game
from one repository's current implementation, expands the exact support theory
far beyond the earlier draft, and makes the central epistemic correction
load-bearing:


a mechanical coordinate determines exact rule support, while public action
history may induce different probability measures on the same support.

The strongest continuity checks all pass:

1. declaration selects a relational algebra over stable physical dominoes;
2. a hand is an ambient marked region, not seven independent values;
3. play is global relocation and local node expenditure;
4. hidden cells are dependent capacity constraints, not independent marginals;
5. the cell system is lossless for Straight rule support within its stated
   scope;
6. belief is a measure on compatible latent worlds, not the fiber itself;
7. strategic value depends on mechanical state, retained evidence, belief,
   continuation field, and utility;
8. coordinate-only strategic value is refuted inside legal Texas 42;
9. shared partnership utility does not create shared private information;
10. exact quotients remain field-, utility-, and information-structure
    relative.

The 90-world legal history witness in v0.6 is stronger than the earlier
minimal witness: both public histories have the same exact endpoint support,
both give every one of the 90 worlds positive probability, and yet the
posterior weighting reverses the best lead. That removes an easy objection that
the counterexample depended on zeroing worlds or changing support.

I found no reason to retract the core combinatorial or epistemic theory. I did
find one serious executable-type contradiction, two formalization gaps, and a
few documentation/trust-boundary problems. They are repaired in v0.7.

## 2. What v0.6 materially improved

### 2.1 Exact support is now a first-class mathematical quotient

The earlier work proved that `(P,k)` cells are a lossless intensional support
representation under Straight play. v0.6 goes much further:

- exact Hall feasibility;
- exact cardinality by bounded occupancy dynamic programming;
- marginal holder support;
- canonical edge reduction;
- determinate, binary, and ternary ambiguity normal forms;
- a global representation-minimality theorem for deterministic exact support;
- a native full-schema census and fixed-width lower bound;
- strict Straight reachability as a proper subset of feasible support;
- an explicit feasible-but-unreachable witness;
- exact witness certification and finite decidability;
- typed hidden-actor and viewer-actor support transitions.

That is genuine new mathematics, not merely repackaging.

### 2.2 Reachability is separated from feasibility

This is essential for a proof assistant. Hall feasibility answers whether a
hidden assignment exists now. Reachability answers whether some legal deal,
contract, declaration, and public play prefix could have produced the state.
Those are different predicates, and v0.6 correctly proves that feasible support
need not be reachable support.

### 2.3 Chance, support, and sampling law are separated

v0.6 refuses to infer a probability distribution from a support set. It defines
uniformity only after a chance or sampling law is named, and derives an exact
count-ratio sampler under the selected uniform fiber law. That protects the
foundations from the common error `possible = equally likely`.

### 2.4 Full-play and settlement semantics are separated

The package retains the complete 28-play, seven-trick mathematical hand and
models early make/set settlement only as an explicitly scoped outcome quotient.
This preserves raw-point and observation-sensitive semantics instead of letting
an engine optimization redefine the game.

### 2.5 The general strategic boundary is stronger

The current package includes retained continuation record and latent field
state rather than pretending a posterior on physical worlds is always enough.
That is the right correction for history-reading or stateful fields.

## 3. Findings that required v0.7

### 3.1 Major: reachability evidence had become identity-bearing data

The mathematical foundation says a witness or validator tag may be erased
after certification. The v0.6 executable specification nevertheless said that
every reachable contracted state carried a certificate tied to one exact hand
origin, and that certificate equality was distinct from physical-state
equality.

That is dangerous in a proof assistant and wrong as semantic identity.

The same physical state can have more than one proof or replay witness. If
witness identity participates in equality, then:

- proof terms split one semantic state into several values;
- caches and maps can distinguish states the game cannot distinguish;
- hidden deal or path provenance can leak through a supposedly public state;
- quotient and congruence theorems acquire artificial proof-term side
  conditions;
- extraction behavior can depend on which certificate happened to be built.

**v0.7 repair:** reachability is a proposition on the semantic state:

```text
Reachable(s) : Proposition
CertifiedState := { s // Reachable(s) }
```

Proofs and witnesses are erased or ignored by equality, hashing,
serialization, and transition. A replay witness may exist as a separate audit
artifact, never as game identity.

### 3.2 Major: derived support appeared as a second source of state truth

The mathematical foundation proves that, for the displayed mechanical
coordinate, rule cells, reduced support, normal form, and fiber are deterministic
views. Their supplemental semantic information is zero bits.

The v0.6 executable type nevertheless stored `cells` inside `MechanicalState`
and stored both `mechanical` and `fiber` inside `NativeHand`. That creates two
problems:

1. equality can distinguish cache layout rather than game state;
2. an inconsistent pair can exist unless every constructor carries a proof of
   coherence.

**v0.7 repair:** the semantic `MechanicalState` stores only the fields from
which exact rule cells are derived. `deriveRuleCells`, support reduction,
normal-form compilation, and fiber construction are pure views. A compiled
cache is permitted only with a coherence proof and is excluded from semantic
identity.

### 3.3 Formal gap: the support normal form lacked one total well-formedness contract

The branch validators in v0.6 were mathematically suggestive but incomplete as
an executable constructor contract. A proof assistant needs one predicate that
also states:

- exact hidden-seat keys and canonical order;
- pairwise disjoint certain-holder sets;
- disjointness of certain and ambiguous tiles;
- valid domino identities;
- reconstructed pool and capacities;
- native capacity bounds;
- conservation;
- branch-specific positivity and exclusion invariants.

**v0.7 repair:** `WellFormedFeasibleSupportNormalForm` is specified, together
with both compile/decode round-trip obligations.

### 3.4 Trust-boundary gap: external verification was not explicitly separated from kernel proof

The Python programs are excellent finite receipts. They do not become theorems
inside a proof assistant merely because they print `PASS`.

There are only three honest ways to migrate a finite receipt:

1. prove the theorem directly in the assistant;
2. define a decision procedure in the assistant and prove it sound, then let
   kernel reduction or reflection close the finite case;
3. import an external result only as an explicit axiom or untrusted artifact,
   which does not meet the intended foundation standard.

**v0.7 repair:** the handoff document states this trust boundary and assigns
every major finite receipt a mechanization route.

### 3.5 Documentation overclaim: the two verifier entry points are not independent implementations

`verify_minimality_and_reachability.py` imports shared abstract-world helpers
from `verify_foundation.py`. Both are dependency-free relative to third-party
packages and independent of the future reference implementation, but they are
not independent of each other.

**v0.7 repair:** the README and verifier header now say exactly that.

### 3.6 Mechanical defect: four claim-ledger rows contained unescaped table pipes

Absolute-value notation such as `|U|` split Markdown table columns.

**v0.7 repair:** the affected expressions use `\lvert\cdot\rvert`.

### 3.7 Naming hazard: a necessary outer profile was called a reachability certificate

The 46-bit upper-bound construction describes a finite outer language that
contains every reachable support but also contains unrealizable candidates. It
is not a sound certificate of actual reachability. The exact certificate is a
replayable `StraightSupportReachabilityWitness`.

**v0.7 repair:** the executable API now calls the upper-bound object
`ReachabilityOuterNecessaryProfile` and its check `checkNecessary()`. The
mathematical text calls these necessary outer profiles. Passing the check never
constructs a reachable type.

## 4. Material from the earlier thread that is intentionally not in the normative core

The independent project correctly removed repository-local names and build
plans from the mathematical authority chain. Three bodies of earlier work
should nevertheless remain visible so that they are deferred rather than
forgotten.

### 4.1 Implementation correspondence and empirical receipts

The earlier package mapped the theory to Atlas, the production engine, Walt,
Hoyt, role/threat tables, C4, and the endgame census. Those are evidence and
conformance targets, not axioms of Texas 42. They belong in a later
non-normative implementation-correspondence annex.

### 4.2 Team-game and regret boundary

The earlier CFR annex made an important negative result explicit:

- fixed-field, information-set-consistent best response is a clean exact
  object;
- a low unilateral four-seat regret gap is not automatically team
  exploitability;
- partners share utility but not observations;
- an exact two-team coordinator game requires prescriptions or ex ante joint
  plans and may be enormous.

v0.6 retains the foundational statement `shared utility != shared
information`, which is enough for the core. The detailed regret/coordinator
analysis remains a deferred annex, not a missing rule theorem.

### 4.3 Architecture and experiment program

Permutation-equivariant hand encoders, contextual node heads, ambient-boundary
features, P-ENC, P-DEL, and P-DECL are modeling hypotheses. Their removal from
the normative foundation is correct. They should return only after the
proof-assistant object is stable.

The continuity map in `70_THREAD_CONTINUITY.md` records these deferrals and the
location of every founding claim.

## 5. Remaining obligations before or during mechanization

These are not defects in v0.7, but they should be consciously scheduled.

1. **Choose the exact formalization profile.** Keep configured `maxMarkBid`,
   match target, and all-pass behavior explicit. Do not silently specialize to
   one implementation default.
2. **Mechanize the finite core before general measure theory.** The native game
   has finite deal and action spaces. A finite probability-mass-function layer
   proves the actual 42 results without first importing standard-Borel
   disintegration machinery.
3. **Use phase-indexed state types.** Auction, declaration-pending, active play,
   hand complete, and match complete should be separate constructors or
   indexed records so impossible field combinations are unrepresentable.
4. **Separate mathematical sets from runtime bitmasks.** Prove the set-level
   game first. Establish bitmask refinement later.
5. **Internalize the decisive finite witnesses.** The unique-winner census,
   count total, transport classification, Hall corpus, support-normal-form
   checks, reachable-capacity profile count, feasible-unreachable witness, and
   90-world posterior reversal should eventually be kernel-checked or proved
   by sound reflection.
6. **Keep the full match later in the dependency graph.** A hand is finite;
   repeated all-pass attempts make the unconstrained match history
   potentially unbounded. Prove one contracted hand first, then the match
   kernel and any almost-sure termination result under its explicit chance
   assumptions.
7. **Do not formalize CFR as part of the rules object.** Solution concepts come
   after the extensive-form information structure and utility are proved.

## 6. Final assessment

v0.6 is not a side branch that needs to be pulled back toward the earlier
thread. It is the better foundation. The right action was a small v0.7 that:

- retracts proof-relevant certificate identity;
- makes support objects derived views rather than duplicate state;
- totalizes normal-form validity;
- states the proof-assistant trust boundary;
- preserves the original mandate and a continuity crosswalk;
- leaves the mathematical results and finite receipts intact.

That is the version I recommend taking into a proof assistant.
