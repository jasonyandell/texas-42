# Scheme/Fix: express the game

Scheme is an executable relational language over Texas 42's exact worlds.
Its purpose is expression: queries, exercise families, belief events, and
counterexamples. It makes no general compression claim and is not a new player
or a set of move-scoring bonuses.

**Implemented in the unified crate as `walt::scheme`, 2026-09-06.** The basis is
[information geometry v0.4](../math/unified_information_geometry_v0.4.md)
sections 3–5, with explicit v1 syntax and the extensions described below. The
archived skeleton/factory supplied the semantic and counterexample discipline;
the new runtime uses the current rules and kernel directly. Everything here is
exploratory implementation evidence, not a promoted theorem.

Its first consumer is the [partnership gym](../gym/README.md), with six
count-offer advantage/disadvantage exercises and exact lawful answer keys.
The gym registers a public mechanical relation; the Scheme language itself
still has no solver dependency.
The [discovery extension](../gym/DISCOVERY.md) now lets query files drive the
search, including exact belief-presence queries, and publishes 170 distinct
late-game exercises.

## Run it

From the repository root:

```sh
cargo run --manifest-path walt/Cargo.toml -p walt --bin scheme -- --query walt/scheme/examples/partner-count.scheme --hand 0 --trick 6 --seat 0
```

This queries the existing `rob/receipts/verify_player.txt` coordinate. Its six
legal worlds give partner a count tile, 4-1, in two worlds: event probability
1/3. The program does not use the receipt's actual hidden deal as its belief.

Condition on an explicitly named event:

```sh
cargo run --manifest-path walt/Cargo.toml -p walt --bin scheme -- --query walt/scheme/examples/partner-count.scheme --condition walt/scheme/examples/partner-has-count.scheme --hand 0 --trick 6 --seat 0
```

The posterior has two positive-mass worlds and the answer is now certain. This
is an analyst's intervention: the program does not claim the seat observed it.

Other switches: `--receipt FILE`, `--max-worlds N` (default 40000), `--work N`
(default 10000000), `--compare FILE`, `--comparison answers|existence`, and
`--selector first|uniform`. `--registry` lists the registered predicates;
`--help` lists the complete interface. The default coordinate is hand 0,
trick 6, seat 0. Receipt hand/trick identifiers are provenance, while the
printed kernel, public residue, query version, and predicate versions describe
the computation. Query results do not optimize the receipt's bid.

The world cap is checked by exact counting before uniform-support enumeration.
Work is an explicit counter charged for world checks, bindings, and predicate
calls; custom predicates must charge their inner work too. Exceeding either cap
is an error, with no sampled fallback or partial report. It is not a wall-time
guarantee. Use the existing process watchdog for experiments requiring one.

## Write an expression

```text
(fix
  (roles (chair me) (chair partner) (domino entry))
  (out entry)
  (case
    (viewer me)
    (partner me partner)
    (holds partner entry)
    (master entry)))
```

This returns partner-held tiles that no live tile can beat when led. It finds
mechanical entry candidates, not a claim that we can get the lead to partner.
That continuation question belongs to a separately defined, bounded relation or
an independent policy evaluation.

The grammar is deliberately small:

```text
fix  := (fix (roles (sort name) ...) (out name ...) case ...)
sort := chair | domino | context
case := (case clause ...)
clause := (same name name ...) | atom | (not atom)
atom := (registered-predicate argument ...)
argument := role-name | literal
```

Names begin with an ASCII letter and continue with letters, digits, `_`, or `-`.
Literal spellings cannot be role names. Literals are dominoes such as `6-4`,
chairs `S0`–`S3`, contexts `q0`–`q6` and `q*` (called), teams `T0`/`T1`, and
nonnegative integers. Domino spellings normalize their endpoints. Semicolon
comments run to the end of a line. Parsing is bounded to 1 MiB, nesting depth
32, and at most 64 roles at compilation. Invalid names, unknown predicates,
wrong argument types, duplicate declarations/outputs, and mixed-sort equality
classes are rejected before evaluation.

**Distinct role names of the same sort denote distinct objects within a case,
unless `(same ...)` explicitly identifies them.** Same groups are transitively
closed. For example, `(same a b)` identifies two domino roles; without it they
must be different tiles. Adding both a distinct case and an identified case
covers arbitrary assignments. Different sorts never compete for an identity.

All roles are bound over the full finite sorts: 28 dominoes, four chairs, eight
contexts. Add `(live tile)` when the role must be live. This lets `played` and
constant tile references retain their intended meaning too.

Every role omitted from `out` is existential. Multiple witnesses yield one
output tuple. Cases combine by set union; duplicate and overlapping cases do
not duplicate answers or probability mass. `(out)` asks a Boolean existence
question. A Fix with no cases is false. An empty case is true whenever its
role equality pattern has an interpretation.

`not` is a v1 extension: finite-world negation of a registered atom. A relation
may return **undefined** when its precondition fails. Neither its positive nor
its negative form then holds. For example, with no lead yet, neither
`(led-context q*)` nor `(not (led-context q*))` holds. This prevents negation
from manufacturing a fact out of missing context.

## Registered relations

| Predicates | Meaning |
| --- | --- |
| `live(d)`, `played(d)` | Membership in the kernel live set or explicitly supplied played history. Non-live does not automatically mean played. |
| `holds(c,d)`, `void(c,q)` | Concrete world's remaining holding / absence of effective followers. These require world access. |
| `in(d,q)`, `double(d)`, `beats(a,b,q)` | Existing declaration-relative incidence, double status, and strict trick-key comparison. All nine declarations use the rules module. |
| `boss(d,q)` | Live effective follower in q with no live tile having a higher trick key in q. |
| `master(d)` | Live tile with no live beater when that tile is led (`Kernel::masters`). |
| `quota(c,n)`, `count(d,n)` | Public remaining hand size / exact count decoration (0, 5, or 10). |
| `tile(a,b)`, `chair(a,b)`, `context(a,b)` | Same-sort equality, commonly used to anchor a role to a literal. These do not alter a case's equality pattern. |
| `team(c,T0)`, `partner(a,b)`, `opponent(a,b)`, `successor(a,b)` | Existing partnerships and clockwise order; partner is two seats clockwise. |
| `viewer(c)`, `leader(c)`, `next-actor(c)` | Viewer, current trick leader, and next actor from the public frame. There is no next actor when that seat has no remaining tile. |
| `led-context(q)`, `current-winner(c)` | Current partial-trick facts; undefined at an empty trick. |
| `legal(c,d)`, `forced(c,d)` | Legal play or unique legal play of the next actor; undefined for other seats. These require world access. |
| `own-legal(d)` | Viewer-only legal action; undefined unless the viewer is next to act. |
| `trick-play(c,d)` | Public seat/tile association in the current partial trick; false when absent. |
| `leads-context(d,q)` | The context that `d` leads under the declaration, distinct from merely following `q`. |

## Use it from Rust

`Fix`, `Scheme`, `Role`, `Atom`, and `Term` are public construction types. Parse
text with `source.parse::<Fix>()`, or build the same AST programmatically.
`Fix::compile(&Registry)` validates it and captures immutable predicate handles
and semantic metadata. `CompiledFix::identity()` includes versioned syntax and
all used predicate specifications; it is a complete textual identity, not a
claim of canonicalization under logical equivalence.

`Frame::new(kernel, leader, prefix, played)` validates public-residue coherence.
It does not assert reachability from a full deal. Query a particular compatible
world with `evaluate(&frame, &world, &mut Budget)`, returning a sorted set of
`Answer` tuples in the declared output order.

For belief work:

- `Belief::uniform(frame, cap)` materializes uniform **full legal support**.
- `Belief::from_weights(frame, id, entries, cap)` accepts exact nonnegative
  rational weights. Repeated physical worlds add their weights; zero weights
  disappear. Empty/zero-mass measures, negative weights, and foreign worlds are
  errors. The supplied id names provenance; the frame and weights are the
  actual measure. An empirical measure remains empirical.
- `query.summarize(&belief, &mut budget)` returns exact event probability,
  per-answer presence masses, the distribution of entire answer sets, and
  separately named certainty properties. Summary includes its frame.
- `belief.condition(&query, posterior_id, &mut budget)` conditions on answer
  existence. `condition_likelihood` accepts explicit rational likelihoods in
  [0,1]. Both return the posterior and evidence probability. They leave the
  original unchanged and refuse an impossible event.
- `query.compare(&other, &belief, Comparison, &mut budget)` checks Boolean
  existence or the whole answer relation. Answer comparison requires identical
  ordered output interfaces. A failure returns the first concrete world and
  both answer sets for replay. A pass is confined to the measure's support.

Certainty has distinct fields for possibility, event certainty, constant answer
multiplicity, constant answer set, one answer per world, and one known identity.
They concern positive-mass worlds in the named measure. “Partner has exactly one
count tile in every world” does not say which tile it is.

**Answer presence probabilities need not sum to one.** If a world has two
answers, both presence events include it. `summary.select(...)` requires an
explicit `LexicographicFirst` or `UniformWithinWorld` convention and returns a
separate probability law, including no-answer probability. It preserves each
world's mass. Neither convention is implied by Scheme, and an offline selector
is not automatically an information-local policy.

## Add a relation

Implement `Predicate` and register it once under a unique name. Its
`PredicateSpec` declares a semantic version, typed arguments, horizon in plies,
and `Access::Viewer` or `Access::World`. Evaluation returns `Some(true)`,
`Some(false)`, or `None` for an undefined precondition. The compiler checks
argument types. The runtime gives Viewer predicates only `Frame`; World
predicates additionally receive the concrete assignment. Viewer information
includes the focal private hand and public support/residue, not other hands.

An extension must be a deterministic registered computation, obey its declared
horizon, and charge inner work to the supplied budget. It must not call the
target solver, inspect response labels, or capture undeclared information.
These obligations are part of the trusted extension contract; Rust cannot
audit arbitrary callback captures or prove a horizon declaration. Builtins
import rules/kernel only; the expression engine imports no solver.

Fresh roles are rebound on every query evaluation. Rigid referents can be
expressed by explicit constant anchors in subsequent queries. The runtime
does not select a referent implicitly, treat a hidden root label as known, or
automatically update the game's state after an observation. Supply the next
exact frame and measure from the owning replay/belief system. A compact
descriptor step compiler and a response-preservation/lumpability proof remain
separate work; they are unnecessary for this expressive layer.

## Validation and boundaries

The [validation record](VALIDATION.md) lists the completed checks and their scope.

Focused tests check equality-pattern completeness, existential projection,
overlap, declaration-relative joins against direct hand enumeration, exact
weighted conditioning, selector laws, certainty distinctions, registry access,
partial tricks, invalid inputs, counterexamples, and refusal with no partial
CLI report. The receipt CLI test independently pins the six-world / 1/3 example.
These are finite-domain implementation checks. They do not establish player
strength, optimal partnership play, or a general theorem about compression.

The runtime enumerates finite worlds/role assignments, with unary pruning,
early relational checks, and elimination of already witnessed output tuples.
No opening-scale performance or factor-belief contraction claim is made.
The next gym layer can use expressions to select and describe opportunities,
while keeping lawful make-30 policy evaluation as an independent answer key.
