# Codex Assignment 01 — Domino Universe and Declaration Algebra

Implement the first bounded slice of the Texas 42 Foundations executable
specification.

Read, in order:

1. `README.md`
2. `docs/00_THESIS_AND_SCOPE.md`
3. `docs/10_RULES.md`
4. `docs/20_MATHEMATICAL_FOUNDATION.md`, sections 2 and 3
5. `docs/30_EXECUTABLE_SPECIFICATION.md`, sections 1 through 5 and section 26
6. `docs/40_CLAIM_STATUS.md`, algebra claims plus `REACH-05`

Do not use another Texas 42 implementation as source code or rule authority.
The files under `verification/` are external proof receipts independent of
the implementation requested here; they are not proof-assistant kernel proofs,
and they must not be copied as the implementation.

## Objective

Create a fresh, dependency-minimal Python 3.12 reference implementation of:

1. the 28-domino double-six universe;
2. the nine Straight 42 declarations;
3. called and powered sets;
4. effective suit membership;
5. led-suit selection;
6. follow relation;
7. declaration-relative rank, contextual tier, and trick key;
8. exact trick resolution;
9. contextual `BEATS`;
10. when-led `THREAT`;
11. count points;
12. exact finite verification of the claims assigned below.

This is an executable mathematical specification, not a game server.

## Allowed files

Create or modify only:

```text
pyproject.toml
src/forty_two/__init__.py
src/forty_two/dominoes.py
src/forty_two/declarations.py
src/forty_two/algebra.py
tests/test_dominoes.py
tests/test_algebra.py
verification/verify_algebra.py
```

Do not create auction, deal, state, fiber, belief, solver, CLI, UI, or
optimization code. Do not modify either repository-level proof receipt:
`verification/verify_foundation.py` or
`verification/verify_minimality_and_reachability.py`.

## Toolchain

Use:

- Python 3.12;
- standard library only in runtime code;
- `unittest` for tests;
- no third-party dependencies;
- strict type hints;
- immutable values where practical.

`pyproject.toml` may configure packaging and test discovery but must not add
runtime dependencies.

## Required public API

The exact Python spelling may follow normal language conventions, but the
semantic surface must include:

```python
PIPS
DOMINOES
Domino
DominoId
domino_id(domino)
domino_from_id(id)
contains(id, pip)
is_double(id)
count_points(id)

Declaration
PipTrump(pip)
DOUBLES_TRUMP
NO_TRUMP
GAME_DECLARATIONS

DeclarationAlgebra
algebra_for(declaration)

DeclarationAlgebra.called(id)
DeclarationAlgebra.powered(id)
DeclarationAlgebra.effective_suits(id)
DeclarationAlgebra.led_suit(id)
DeclarationAlgebra.lead_contexts()
DeclarationAlgebra.lead_fiber(led_suit)
DeclarationAlgebra.follows(id, led_suit)
DeclarationAlgebra.rank(id)
DeclarationAlgebra.tier(id, led_suit)
DeclarationAlgebra.trick_key(id, led_suit)
DeclarationAlgebra.beats(led_suit, id)
DeclarationAlgebra.threat(id)
DeclarationAlgebra.resolve_trick(plays)
```

`resolve_trick` preserves actor identity and returns winner plus trick points.

## Canonical identity

Use:

```text
(0,0),
(1,0), (1,1),
(2,0), (2,1), (2,2),
...
(6,0), ..., (6,6)
```

Never derive game rank from `DominoId`.

## Mathematical requirements

Implement the definitions directly.

### Called set

```text
PipTrump(p): every domino containing p
DoublesTrump: every double
NoTrump: empty
```

### Powered set

```text
PipTrump and DoublesTrump: called set
NoTrump: empty
```

### Effective suits

- called domino: `{CALLED}`;
- uncalled domino: its natural pip-incidence set.

### Led suit

- called domino: `CALLED`;
- otherwise: higher pip.

### Follow

`follows(d, q)` is exactly membership in effective suit `q`.

### Rank

Rank is declaration-relative and total:

- a double under doubles-trump ranks by pip, `0-0` through `6-6`;
- every other double has formal rank `TOP`;
- every mixed domino ranks by pip sum;
- rank is ignored in tier zero.

Use an exact ordered representation. Do not use floating point.

### Tier

- 2: powered;
- 1: follows led suit and is not powered;
- 0: otherwise.

### Winner

For four distinct plays:

1. derive led suit from the first play;
2. compare `(tier, rank)` lexicographically, using `(0,0)` for sloughs;
3. return the unique maximum;
4. trick points are one plus count points in the trick.

Reject duplicate dominoes and malformed trick length or actor sequence.

## Required tests

### Domino universe

Exhaustively verify:

- exactly 28 dominoes;
- no duplicates;
- every domino has `0 <= low <= high <= 6`;
- ID round trips;
- each natural pip incidence has size 7;
- doubles have one natural membership;
- mixed dominoes have two;
- count points total 35.

### Effective suit, lead context, and follow

For every declaration and domino:

- effective suit set is nonempty;
- called dominoes have exactly `{CALLED}`;
- uncalled doubles have one natural effective suit;
- uncalled mixed dominoes have two;
- `follows(d, q) == (q in effective_suits(d))` for all eight contexts;
- `lead_contexts()` is exactly the image of `led_suit` over all 28 dominoes;
- exactly seven contexts are leadable;
- the seven lead fibers are disjoint, cover all dominoes, and have cardinality
  multiset `{1,2,3,4,5,6,7}`;
- under doubles-trump, natural effective suit 0 is nonempty but context 0 is not
  leadable.

Do not delete or alter the algebraic follow table merely because one context is
unleadable. Leadability is a derived reachability property of lead actions.

### Rank and tier

For every declaration, context, and domino verify:

- tier 2 iff powered;
- tier 1 iff unpowered follower;
- tier 0 otherwise;
- trick key is `(0,0)` exactly in tier zero;
- natural doubles top mixed members of their live natural suit;
- pip-trump double tops all mixed trumps;
- doubles-trump order is `6-6` high through `0-0` low.

### Unique winner and independent prose resolver

Exhaust every:

- Straight declaration;
- designated lead domino;
- three-domino subset of the remaining 27.

Assert exactly one maximum trick key. For the same case, resolve the winner a
second way with a separately coded prose-rule resolver:

1. derive trump/called status directly from declaration and endpoints;
2. if any trump was played, select the highest trump by the prose ordering;
3. otherwise select the highest tile that follows the led suit by the prose
   ordering;
4. never call `trick_key`, `tier`, `rank`, `beats`, or the production
   `resolve_trick` from this independent resolver.

Require the two winners to agree in every case. Do not multiply by follower
permutations merely to inflate the count. Expected count:

```text
9 * 28 * C(27,3) = 737,100
```

### Trick scoring

Exhaustively or structurally verify:

- count total is 35;
- seven base trick points make 42 total;
- `resolve_trick` returns `1 + count payload`;
- off-suit unpowered sloughs cannot beat a valid lead;
- highest trump beats every nontrump;
- without trump in the trick, highest follower wins.

### BEATS

For every declaration, led context, and domino pair:

```text
e in beats(q, d) iff trick_key(e, q) > trick_key(d, q)
```

### Negative threat witness

Under no-trump:

- `0-0` and `1-1` have empty when-led threat sets;
- they follow different natural suits.

State explicitly that `THREAT` is an exact diagonal query but not a complete
play ontology.

### Pip transport classification

Exhaust all 5,040 pip permutations.

First retain only permutations preserving every domino's count label. Assert
that exactly two survive:

```text
identity
swap 2 and 3
```

Then verify:

- identity is an automorphism of every Straight declaration layer;
- swap `2 <-> 3` is a game-semantic order isomorphism from declaration 2 to 3
  and from 3 to 2;
- it is not such an isomorphism of any other Straight declaration layer;
- it need not preserve literal numeric rank or trick-key labels, only the
  transported game comparison relations.

The comparison must preserve:

- count labels;
- led-suit transport;
- follow relation;
- every pairwise trick-key order comparison.

## Verification script

`verification/verify_algebra.py` must run without test discovery and print a
compact deterministic proof receipt containing at least:

```text
Texas 42 declaration algebra verification: PASS
dominoes: 28
declarations: 9
unique-winner cases: 737100
prose-rule winner agreement cases: 737100
count points: 35
hand points with seven tricks: 42
count-preserving pip permutations: 2
scoped nontrivial transport: 2 <-> 3 only
negative witness: threat incompleteness PASS
```

The script may reuse the public implementation to drive exhaustive loops,
but its prose-rule winner resolver must remain definitionally independent of
the trick-key implementation.

## Forbidden shortcuts

Do not:

- copy tables or code from another project;
- copy either repository-level verification program;
- hard-code 28-by-declaration result tables as source data;
- use a packed bit representation as the primary semantic implementation;
- use global mutable state;
- use floating-point ranks;
- assign strategic value to dominoes;
- add any declaration outside the nine Straight 42 declarations;
- call the `2 <-> 3` transport a global symmetry or a literal numeric-rank
  isomorphism;
- assume local list order has game meaning;
- add optimization caches before correctness tests pass;
- silently weaken exhaustive tests.

Derived immutable lookup tables are allowed only when generated from the
implemented definitions.

## Documentation requirements

Every public function or class must state the mathematical object it implements
using stable claim identifiers where practical, for example:

```python
'''Implements ALG-06/ALG-07 effective-suit absorption and covering.'''
```

Avoid references to unrelated implementations, project history, or speculative
future work.

## Ambiguity protocol

If the package is internally inconsistent or required behavior is not
determined:

1. do not choose a plausible interpretation;
2. do not silently consult or copy another implementation;
3. add a failing or clearly blocked test demonstrating the ambiguity;
4. report the exact conflicting passages;
5. stop work on the affected behavior and continue only on unaffected items.

## Acceptance criteria

The assignment is complete only when:

1. all allowed files are present and no out-of-scope files changed;
2. `python -m unittest discover -s tests -v` passes;
3. `python verification/verify_algebra.py` prints the required certificate;
4. exhaustive case counts match;
5. public APIs are typed and documented;
6. runtime code has no third-party dependency;
7. the final report lists files changed, commands and results, exhaustive
   counts, ambiguities, and deviations.

Do not begin the next implementation slice.
