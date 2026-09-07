# Bounded shared relational policy construction

Status: exploratory Slice B/C instrument.  Grammar and predicate semantics are
identified by `scheme-relational-actor-v1/grammar-v1/straight-v0.4`.

This constructor learns one short `PolicyProgram` across decision rows from
multiple roots.  It minimizes supplied action costs, not agreement with one
teacher action.  Equal-cost actions therefore do not create a reason to split
the program.  Promotion still depends on independent, grouped, whole-policy
rollouts; the row objective is a training surrogate.

## Actor boundary

Every emitted program has:

- one mode, named `shared`;
- no rigid bindings;
- no exact rules;
- only `FirstOutput` selectors;
- the ordinary `lowest-legal` final fallback;
- one serialized program and content digest for every root on which it is used.

The constructor owns `Registry::standard()` and accepts no predicate callbacks.
Every grammar atom is a zero-horizon `Viewer` predicate.  Guards can inspect the
player's remaining hand and public `Frame`, because that is the existing Viewer
contract.  They cannot inspect a `World`, a complete hidden hand, a physical
tile literal, an exact history key, a source seed, a root identity, a teacher,
or a response label.  Full actor-attributed history, banked score, bid, and
declaring side are retained in each owned example and in its cache key, but the
v1 guard surface does not expose those fields as predicates.

The relation names below are mechanical descriptions, not tactics.  For
example, partner currently winning does not say the partner will win the
completed trick.  Cost-sensitive training decides whether a clause earns its
place under the declared data and complexity caps.

## Fixed grammar

Each clause always includes `own-legal(action)` and returns the least legal
domino in its answer relation.  The seven selector relations are crossed with
two qualifications, giving fourteen named clauses:

| Selector relation | Additional atoms |
|---|---|
| `legal` | none |
| `count-0`, `count-5`, `count-10` | `count(action,n)` |
| `master` | `master(action)` |
| `follow-led` | `led-context(q)`, `in(action,q)` |
| `boss-led` | `led-context(q)`, `boss(action,q)` |

The unqualified form is `Any`.  The qualified form adds
`viewer(me)`, `current-winner(winner)`, and `partner(me,winner)`.  Since
`current-winner` is undefined before a lead, the qualified form does not match
an empty trick.

This library is intentionally finite and explicit.  Expanding it changes the
grammar version and should be justified by a supported, materially costly
counterexample.

## Cost rows

`DecisionExample::new` takes an owned `Frame`, full public history, score,
contract fields, a `BTreeMap<Domino, BigRational>`, and a nonnegative group
weight.  It refuses a row unless:

- the public observation is a valid `PolicyInput` and the viewer is to act;
- costs cover every legal action exactly once;
- every cost lies in `[0,1]`;
- the group weight is nonnegative.

The intended exact-cost row is `c(I,a) = max_b Q(I,b) - Q(I,a)`.  A certified
upper-cost row may instead use the clipped interval expression declared by its
teacher campaign.  The constructor does not promote sampled means to exact or
certified values; provenance belongs to the lesson artifact.

For equal-root training, a reached history's row weight should include its
probability under the current actor and the root's equal share.  Keep all roots
from one source deal in the same data split.

## Search and budgets

`RelationalLimits` independently caps clause count, total AST nodes, beam width,
unique programs scored, and Scheme inference work per complete observation.
The deterministic beam starts with the empty program and generates insertions,
deletions, and reorderings.  Programs are ranked by exact weighted cost, then
clause count, AST nodes, and canonical clause order.  Thus an empirical tie
always favors the simpler program.

Clause results are cached by `(clause, PolicyKey)`.  `PolicyKey` contains the
complete owned policy input: frame/kernel data, full actor-attributed history,
score, bid, and declaring team.  Cache entries never cross constructor runs or
predicate versions.  `LearningMetrics` reports search work, cache evaluations
and hits, actual Scheme inference work, cap status, selected exact weighted
cost, and selected structural size.

`RelationalFit::candidates` contains the final beam in deterministic surrogate
order and always retains the empty lowest-legal baseline (appended when it fell
outside the beam).  Each candidate exposes its complete program, content
digest, exact weighted cost, clause count, and AST nodes.  A campaign should
replay these candidates on development roots and choose by whole-policy results
before touching the final grouped test split.  `evaluate_program_cost` is
available for a held-out row audit through the real compiled policy runtime; it
is not a substitute for that rollout selection.

## Minimal use

```rust
use walt::policy_search::relational::{RelationalLearner, RelationalLimits};

let learner = RelationalLearner::new(RelationalLimits::default())?;
let fit = learner.fit(&decision_examples, "shared-relational")?;

assert!(fit.program.exact_rules.is_empty());
assert!(fit.program.bindings.is_empty());
// Persist fit.program and fit.digest once, then replay that same actor on
// every development or test root.
```

On-policy aggregation is outside this module: replay a frozen candidate on new
discovery deals, reconstruct the teacher from the actor's own hand and public
history, obtain costs for every legal action, append those rows, and fit again.
Teacher memoization remains scoped to its complete root, field, and ordered
sample stream.  This constructor's clause cache does not authorize teacher
cache reuse.
