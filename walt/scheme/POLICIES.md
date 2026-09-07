# Executable Scheme policies

Status: exploratory executable instrument. A policy is an information-measurable
controller, not a claim that Scheme/Fix compresses optimal play.

`PolicyProgram` is the durable, plain-text representation. It has three action
layers, in order: exact information-state rules, ordered relational rules, and a
mandatory `lowest-legal` fallback. Every returned action is checked against the
straight-42 legality function.

An exact key contains the declaration, viewer, remaining viewer hand, the public
kernel support (hidden pool, seat capacities, and public void constraints),
leader, current prefix, played set, full actor-attributed public history, banked
scores, bid, and declaring team. It contains no hidden-world identifier or
realized opponent hand. Exact rules make synthesized decision tables
serializable across whole random deals without confusing two public predecessors
that happen to share a current residue.

Relational guards are ordinary Fix expressions. Compilation rejects a guard or
rigid-binding query that mentions any predicate registered with `Access::World`.
Evaluation then uses `CompiledFix::evaluate_viewer`, whose call surface accepts
no `World`. A guard selecting `answer` must return one domino column; the policy
chooses the least legal returned domino. Literal and rigid selectors require a
Boolean guard (a zero-column relation).

Rigid bindings run once when a controller is initialized. Each must yield
exactly one value from the initial viewer frame. Rules may select a domino-valued
binding later. A controller also stores a named mode. A successful relational
rule changes it from `in` to `next`; exact table actions leave it unchanged.
This supplies an explicit distinction between solving each observation fresh
and carrying a fixed choice or mode across observations.

```scheme
(policy keep-opening-master
  (initial fresh)
  (fallback lowest-legal)
  (bind opening
    (fix
      (roles (domino d))
      (out d)
      (case (tile d 6-6))))
  (rule reuse
    (in fresh)
    (next persistent)
    (select rigid opening)
    (guard (fix (roles) (out) (case))))
  (rule continue
    (in persistent)
    (next persistent)
    (select answer)
    (guard
      (fix
        (roles (domino action))
        (out action)
        (case (own-legal action) (master action))))))
)
```

Use `PolicyInput::new` rather than assembling an input implicitly. It checks that
the actor-attributed history is a coherent full public predecessor: turn and
trick order, viewer legality, observed voids, remaining capacities, score,
current leader and prefix, and the live/played deck partition must agree with the
frame. The residual kernel must also have nonempty support. `PolicyKey::from_input`
creates the exact lookup key. Compilation repeats these checks for parsed exact
keys and rejects an illegal stored action. Parsing followed by display produces
a stable canonical policy text; the text includes exact keys and their actions.
These constructor checks intentionally scan the public history and count residual
support. They are reference-path audits; experimental solver timing should time
the search itself separately from policy input validation and artifact replay.

Errors are explicit for malformed programs, duplicate exact keys, ambiguous or
missing rigid initializers, hidden-world predicates, acting out of turn, an
illegal exact action, budget exhaustion, or an empty legal set. A relational
rule whose proposed action is currently illegal is treated as no match, allowing
the next rule or the complete fallback to act.

A controller is stamped with the compiled program's complete identity, including
registered query semantics. Passing it to a different compiled program is an
error. Mode changes occur only after guard evaluation and legal action selection
both succeed, so exhaustion and illegal proposals cannot partially advance
controller state.

`policy_search::program::export` converts a sampled `TablePolicy` into these
exact rules by replaying each table history through the typed public-frame
transform. It then verifies a display/parse roundtrip before returning the
artifact. `replay_program` performs an independent full deal replay through the
compiled artifact. The concrete sampled world supplies physical hands to the
rules engine; policy calls receive only `PolicyInput`.
