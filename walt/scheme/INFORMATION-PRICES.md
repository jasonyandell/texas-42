# Finite information-price teacher

Status: exploratory finite instrument, `scheme-information-events-v1`.

`policy_search::prices` is the Slice D examiner-side consumer.  It accepts an
`ExerciseRoot`, one immutable `SlicePolicy` for every nonviewer seat, and a
finite vector of complete `World`s.  It builds the complete reusable public
tree for that exact root/field/prior scope.  The instrument does not change the
player, expose a hidden feature to an executable policy, or claim population
authority for an empirical prior.

## Information state and tree

Every node retains:

- the full sequence of plays after the supplied exercise root;
- the public `State` (leader, partial trick, banked points, and played tiles);
- the multiset of surviving indices into the original world vector;
- every observable field branch;
- every legal viewer action, its child, and its exact best lawful continuation
  make mass.

The map is keyed by the entire post-root public history.  It does not rebase a
query into a new root and it does not infer a posterior from only the realized
world.  Nonviewer actions are obtained by calling the supplied `SlicePolicy`
with that seat's own remaining hand and `PublicRecord`; its hidden choice then
filters the original indices.  Duplicate input worlds have distinct indices
and retain exact sample multiplicity in every center and value.

`FiniteOracle::from_worlds` reports `FiniteEmpirical` authority.  Its exact
values and bounds concern only the supplied finite multiset.  The separate
`exact_root_fiber` constructor enumerates the root itself, enforces a declared
cap, and records `ExactRootFiber` only after comparing the complete ordered
enumeration.  This is the gym's uniform reset model at the supplied
`ExerciseRoot`; it is not a claim about the likelihood of the public prefix
before that root.

## Exact lawful values and executable lowers

At a viewer node the ordinary recursion chooses one action for all active
world ids.  At a field node it sums disjoint observed-action branches.  Thus
each `OracleAction::exact_makes` is the exact finite-support value of the best
information-consistent continuation after that action is fixed.  These values
remain in every returned `ActionBounds` as a truth audit.

An optional immutable `SlicePolicy` can be replayed through the retained tree
after each candidate first action.  `lawful_program_q` is therefore a feasible
continuation lower from an actually supplied lawful rule.  It is not assembled
from per-world witnesses.  `ActionBounds::cost_upper` combines this lower with
the best available action upper and clips the result to Boolean utility's
`[0,1]` range.

## The versioned Scheme event basis

The four Boolean events are fixed by `scheme-information-events-v1`:

1. the viewer's partner holds a five-count tile;
2. the viewer's partner holds a ten-count tile;
3. the partner coholds the declaration-relative context led by the candidate
   action;
4. the partner holds a count tile in that candidate action's led context.

`event_scheme_source(event, action)` returns the actual examiner-side Scheme
`Fix` for each event.  They use the standard versioned `viewer`, `partner`,
`holds`, `count`, `leads-context`, and `in` predicates.  The inner loop uses a
native specialization of the same relations; focused tests compare it
extensionally with compiled Scheme on concrete worlds.  Changing either the
event definitions or the predicate semantics requires a new basis version.

A `PriceProgram` supplies four integer coefficients.  One program is frozen
and shared across roots, histories, and actions.  The finite ternary library
is a discovery tool only: it enumerates all coefficient vectors in
`{-1,0,1}^4` while charging each candidate.  A held-out evaluation consumes one
already selected vector; the oracle does not choose a different vector at each
node or action and call that a shared program.

## Exact centering and the upper

For focal information node `I`, action `a`, active multiset `W_I`, event
`phi_j`, and shared coefficient `theta_j`, the charged increment is

```text
sum_j theta_j * (phi_j(I,a,world) - mean_{w in W_I} phi_j(I,a,w)).
```

`FiniteOracle::centers` computes every mean as an exact `BigRational` over the
full active id multiset.  Centers are scoped by root identity, field identity,
ordered prior digest, basis version, and coefficient vector.  A mismatch,
missing center, exhausted work budget, illegal field action, or foreign world
is a refusal; no bound is returned.  `audit_centers` rechecks every conditional
zero equation with exact arithmetic.

For a queried action, the relaxed recursion first takes that action and only
then begins charging prices at future viewer decisions.  At each such future
decision a clairvoyant inner path may choose its best action separately in each
world.  Field actions still follow the frozen field and public branch.  The
outer result averages those exact per-world maxima.  Conditional centering
makes the expected accumulated price zero for every lawful policy, so weak
duality gives an upper on the best lawful continuation.

A centered charge at the already fixed queried action would integrate to zero
and cannot tighten its action-specific upper.  Omitting it makes this boundary
explicit and prevents a root-only calculation from looking like a gain.  The
returned priced upper is the minimum of the priced relaxation, the independently
computed unpriced perfect-information relaxation, and the trivial Boolean
upper one.  The exact lawful Q must still lie below it or the instrument
refuses.

## Work and comparisons

For a fixed lesson set and fixed continuation lowers, the cost
`max_b U(I,b) - L(I,a)` differs between upper-bound methods only by a
state-dependent constant shared by every action. When all bounds are in
`[0,1]`, clipping does not change this conclusion. A cost-sensitive constructor
therefore cannot select a different program merely because these uppers tighten.
This invariance is tested natively. Prices can improve certificates or guide
additional teacher work and pruning; those are separate uses from changing an
action ranking with fixed lowers. The current complete-tree campaign measures
certificate width and charged work and does not claim accelerated search.

`PriceWork` charges tree nodes, field calls, native event evaluations, exact
moment terms, relaxed inner branches, lawful-policy calls, authority checks,
and coefficient candidates.  Center construction and priced recursion are not
free preprocessing.  A same-budget comparison should give the unpriced and
priced teachers the same total unit cap and report the full counters; if the
priced route spends units on centers that the unpriced route can spend on more
tree work, that is part of the result.

The initial complete small-endgame implementation materializes exact all-state
Q values before lesson generation.  This makes learner-induced history queries
cheap and prevents a campaign from silently changing the original-root
posterior.  It is suitable for bounded two- and three-tile diagnostic roots,
not an opening-tree scaling claim.

## Focused gates

`walt/walt/tests/information_prices.rs` checks:

- parity with the independent exact sampled `Search` recursion;
- native event parity with compiled Scheme;
- exact zero centering at every materialized focal history;
- priced and perfect-information uppers above exact lawful Q;
- an actual frozen-policy lower below exact Q;
- absence of a fake price effect when fixing the last root action;
- duplicate sample multiplicity;
- refusal for mismatched centers, bad histories, and exhausted budgets;
- explicit empirical versus complete-root-fiber authority.

These checks establish finite-support soundness for this consumer.  They do not
show that the basis tightens a bound, that a selected coefficient generalizes,
or that a relational actor is strong on unseen hands.
