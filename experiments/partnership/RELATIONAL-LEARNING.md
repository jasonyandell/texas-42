# Shared relational policy learning

The [Astra proposal packet](packet/texas42_relational_learning/PATH-TO-GENERALIZING-SCHEME-POLICIES-v0.1.md)
was preserved unchanged in `14e01322`. It is a design input; its generic rational
checks are separate from the native results below.

This instrument constructs one small executable Scheme policy across starting
hands, records exactly how it chooses, and measures its actual continuation.
The [first completed experiment](campaigns/relational-learning-v1/RESULTS.md)
contains positive engineering results and negative strength/pricing results.

## Components and boundaries

- [Policy tracing](../../walt/scheme/POLICIES.md): exact-key, named relational-rule,
  or final-fallback provenance; controller state, work, and contract resolution.
- [Shared constructor](../../walt/scheme/RELATIONAL.md): a fixed library of 14
  mechanical clauses, deterministic cost-sensitive beam search, explicit clause,
  AST and work caps. Emitted actors have one mode, no bindings and no exact keys.
- [Finite teacher and information prices](../../walt/scheme/INFORMATION-PRICES.md):
  full public-history trees, exact all-action values and conditional centers,
  sound relaxed uppers, strict support authority, and charged work.
- `relational_lab`: small native generate/fit/evaluate jobs.
- `relational_campaign.py`: grouped splits, two rounds of on-policy discovery,
  whole-policy development selection, freezing and final tests.
- `relational_exam.py`: an existing exact gym as a separate post-freeze exam.
- `verify_relational.py`: independent physical replay and evidence audits.

The existing L1/L2 player is unchanged. These saved programs can be deployed via
`PolicyProgram::compile`, `initialize`, and `choose`/`choose_traced` using the
ordinary own/public `PolicyInput`. This panel uses a stateless continuation
initialization protocol at each supplied root. It does not cold-start a learned
stateful plan and pretend that memory was reached from the opening.

## The target distribution

A source seed shuffles a complete deal. The existing legal `HashField` fixture
prefix supplies a coordinate at which S0 has three dominoes and is next to act.
The generator takes the first coordinate in each disjoint 1,000-seed range that
has an unresolved bid-30 contract, at least two own legal plays, and mechanical
support no larger than the declared cap. These filters use only own/public
information; they do not select for action-value gaps or successful outcomes.

At that continuation root, the prior is uniform over the entire mechanically
compatible fiber. There is **no pre-root field likelihood**. After the root,
every opponent/partner action conditions the original finite prior according
to the frozen field. The teacher retains full histories and original-world
indices rather than reconstructing a new uniform posterior at every decision.

The gym field is the maintained L1 teammate/L0 opponent field with 40 partner
worlds. Native L0-8 is a separate target panel. The prefix generator is not the
post-root opponent model. The measured profile uses declaration sixes and S0
as bidder/viewer; the existing gym exam also tests other declarations and seats.
These are bounded endgame continuation tests, not opening-game strength results.

## Learning and selection

Each campaign has disjoint source ranges for train-a, train-b, development and
final test. Starting-hand overlap across splits is refused. The first round
collects decisions reached by lowest-legal on train-a. It fits three actors:

1. exact optimal action regret: `max Q* - Q*(a)`;
2. interval cost using an unpriced perfect-information upper and the current
   actor's lawful continuation lower;
3. the corresponding cost using a priced upper and the same kind of lower.

Every legal action receives an exact rational cost. A reached state's weight
is its exact occurrence probability within its root. This makes the objective
an equal-root sum of on-policy decision costs, up to one shared normalization.
The exact teacher is cross-checked against the pre-existing sampled `Search`
run on the whole fiber.

For each actor's complete policy, the evaluator verifies exactly that root
optimal value minus actual policy value equals its expected sum of local
optimal-action regrets. Interval costs must dominate these regrets and their
on-policy sum must bound the whole-policy gap. Lower bounds execute a single
lawful candidate continuation, not separate hidden-world optimizers.

The beam's finalists, including the empty baseline, are replayed over complete
development fibers. Promotion uses equal-root full-policy value, then fewer
clauses, fewer source bytes, and digest. Each promoted actor supplies its own
new on-policy states on train-b. Those lessons are aggregated with train-a and
fitted again; development selects again. Final programs and digests are pinned
before test values are computed. Test data never enters this loop.

The final panel compares each frozen shared actor, an existing 16-sample exact
table, and that **same frozen table** with the shared actor used above its final
lowest-legal fallback. Hybrid construction preserves every exact key/action;
it performs no reoptimization. Thus the paired hybrid result measures a full-
policy fallback substitution, not an outcome comparison conditioned on misses.
Hybrids are added in final `prices=off` evaluation for the three selected arms;
discovery does not need that extra comparison.

## What pricing can and cannot change here

The first experiment freezes the four-event coefficient vector `[0,1,-1,1]`
before data selection. It does not fit coefficients on the test set or claim
to exhaust the price class. The API exposes a bounded ternary discovery library
for later experiments. Every center is exact at the full lawful information
state; physical-world access is confined to the examiner. A price at an already
fixed queried root action is omitted because its expectation is zero.

Both teacher routes share the same charged tree build and receive the same
remaining work allowance. Price construction, moments and modified inner search
are charged. Exact-Q truth checking and student evaluation are reported
separately; measured job wall includes all of them. This complete-tree v1 is an
instrument for correctness and certificate width, not a pruning implementation.

With fixed lowers, changing `max U - L(a)` changes every action's cost by the
same state-specific constant. Valid `[0,1]` bounds make clipping immaterial.
Thus a tighter upper alone cannot change this learner's program ranking. The
native test suite checks this invariance. A useful subsequent price application
would change adaptive teacher work or prune a search, while preserving the
same bounds; it is not evidence of stronger actors merely to add prices.

## Run, stop, inspect and resume

Build once, then run under the repository watchdog:

```sh
cargo build --release --manifest-path walt/Cargo.toml -p walt --bin relational_lab
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py \
  --seconds 295 --output-dir /Users/jason/data/texas-42/relational-cap-001 -- \
  python3 experiments/partnership/relational_campaign.py \
    --output /Users/jason/data/texas-42/relational-example \
    --seed 99000000 --train 32 --dev 16 --test 64 \
    --tiles 3 --max-worlds 512 --field gym --samples 16 \
    --clauses 3 --beam 6 --workers 10 --seconds 285
```

`--train 32` means 32 roots in each of the two discovery rounds. Status is in
`status.json`; completed native jobs have atomic `jobs/*/result.json` markers.
Resume with the same configuration and output directory under a **new** cap
receipt directory. Completed jobs are hash-validated and reused. SIGINT/SIGTERM
kills active native process groups and returns incomplete status 75. In-flight
jobs may retry; completed jobs stay unchanged. Source/binary/input drift and
corruption are explicit refusals. These semantics cover process interruption
and restart; the instrument does not claim transactional recovery from power
loss. No native child is allowed to detach from its job.

The native `fit` command consumes strict inspectable `.lessons` files containing
only public request fields, a rational weight, and action-indexed costs. Costs
are training data, never predicates registered into the executable actor. Full
programs, candidates, source identities, teacher diagnostics, exact fractions,
first-fallback distributions, sample occupancy, and independent replay traces
are retained under each job's attempt directory.

For the existing gym:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py \
  --seconds 295 --output-dir /Users/jason/data/texas-42/relational-exam-cap-001 -- \
  python3 experiments/partnership/relational_exam.py \
    --learned /Users/jason/data/texas-42/relational-example \
    --output /Users/jason/data/texas-42/relational-exam-example --workers 10
```

## Reading the evidence

Use equal-root make probabilities and paired root-group uncertainty for the
fresh-deal panel. First-action regret gives that action an optimal continuation;
whole-policy regret grades the actual saved continuation. First fallback is
measured before contract resolution; post-resolution moves remain in complete
replay audits but do not inflate that diagnostic. Empirical occupancy retains
sample multiplicity and distinguishes sampled mass from unique physical worlds.

The existing gym is outcome-selected and contains related coordinates from
shared source deals. Its report uses equal-coordinate exact scores and gives
source group counts without pretending the coordinates are independent deals.
A lower training cost, a narrow empirical dual gap, a rule firing often, or a
smaller actor is not by itself a population strength result.
