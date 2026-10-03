# Walt: larger GPU batches and a different unit of computation

2026-09-19 local date. **Exploratory design draft.** The new request permits
a separately evaluated change in playing strength; it does not turn a changed
policy into a byte-compatible acceleration of the old player. The standing
speed target is a complete 28-play L2 Partner game in 100 ms. No result below
meets that target or establishes stronger play.

The recommendation is to build an anytime, information-consistent best-response
ladder whose CPU controller requests broad batches of fixed-policy evaluations
and field-only continuations. Keep the existing exact CPU solver for small
frontiers and unresolved tails. The GPU should earn those jobs by measured
end-to-end cost. The current per-invocation recursive GPU solver has not earned
them, even with large offline batches.

**What the larger-batch measurement actually says.** The frozen M5 Max / Metal
host replayed all 43,120 natural requests at each capacity, with four repeats
and forward/reverse ordering of the five CPU/GPU arms. Every GPU choice and
complete action vector matched its exact CPU oracle.

| Batch capacity | Dispatches per corpus pass | GPU choices, median ms | CPU serial, median ms | CPU 18-worker pool, median ms |
|---:|---:|---:|---:|---:|
| 128 | 342 | 2,754.494 | 63.079 | 44.996 |
| 1,024 | 46 | 600.863 | 62.796 | 14.730 |
| 8,192 | 9 | 186.739 | 62.184 | 9.527 |

GPU timing includes packet upload through mapped readback; CPU timing includes
prepared search and result collection. Initialization and oracle construction
are separately recorded. At capacity 8,192, GPU initialization was 12.113 ms
and full-vector GPU evaluation took 239.483 ms. The CPU control is an older
frozen replay build, not selected v34. The host caps capacity at 8,192 and
chunks each input file separately: there was no single 43,120-request dispatch.
These requests are all sixes trump, eight samples each, mostly depths 2–4;
only 44 are depth 7. Offline ready width is not online ready width.

The larger capacity improves GPU choice time 14.75-fold, yet it remains about
3.00 times slower than serial CPU and 19.60 times slower than pooled CPU.
This rules against deploying this kernel for these captured jobs. It does not
rule out a different GPU computation or measure the cost of a full player.
The source uses one invocation per complete request, a private 29-frame DFS
stack, and two-u32 arithmetic for the historical u64 RNG/hash. Those are
specific implementation facts; their individual performance contributions
have not been profiled.

Evidence: [large-batch report](../gpu-replay/results/large-batches-v1/20260920T041920Z-85102/REPORT.md)
and its pinned protocol, raw timings, input identities and summary. The failed
Python preflight before dispatch is retained separately by that report.

**The mathematical object to preserve.** Fix a focal information state, a
contract, a finite ordered scenario bundle B, and a frozen field. A scenario
includes its hidden deal and any original deterministic Dice tape. Duplicate
deals with distinct tapes remain distinct scenarios. Let the focal player's
own-team payoff be u in {0,1}. For legal root action a,

```
Q_B(a) = max over lawful continuation policies rho with root a
           sum_w weight(w) * u(w, rho; frozen field).
```

The root choice is one action for the whole current information set. After a
field play, scenarios are partitioned by the observed tile. A subsequent focal
choice is shared by every scenario in that complete observed-history bucket.
Full history is a safe identity; any cheaper key needs its sufficiency
argument. Plan IDs, hidden hands, scenario IDs and tapes are not observations.

The integer recurrence behind the existing sampled solver is terminal payoff,
SUM across complete field-observation buckets, and MAX across legal focal
choices. Sampling error and model error remain outside its exact-on-bundle
arithmetic. The finite bundle does not become the entire lawful fiber.

There are two distinct ladders. The **model level** ell identifies the frozen
lower-level field being answered. The **focal horizon** h specifies how many
future focal decisions are kept information-consistent before a bounded tail.
Increasing h for a fixed B and field can tighten bounds. Increasing ell changes
the field; it does not imply stronger play against a particular opponent.
Likewise, changing sample count or belief changes the evaluated object and
does not inherit monotonic bounds from the previous one.

**A useful radical primitive: enumerate plans, not private search stacks.**
With k own tiles remaining, force root action a and permute the other k-1
tiles. A plan always plays its earliest remaining legal tile in that priority
order. It is a total, lawful policy from the root onward. There are at most
k! plans across all root actions, at most 5,040 at trick 1.

A GPU invocation for (plan, original scenario) can follow one physical
continuation with small state and no search stack. Its output is a Boolean
payoff, optionally a public trace. Rows of the resulting matrix are lawful
policies; columns are scenarios. For the complete plan family P_a,

```
L_a = max_p sum_w weight(w) * payoff[p,w]
U_a = sum_w weight(w) * max_p payoff[p,w]
L_a <= Q_B(a) <= U_a.
```

The first maximum chooses one plan across the information set. The second
reveals the entire scenario, including its tape, and is used only as an upper
bound. It must never be reported as imperfect-information play.

There is a short completeness argument for the upper. Fix one scenario and
any attainable viewer trace after a. List the viewer's distinct tiles in their
order of play, then extend the list after early termination. Under that
priority order, the first remaining listed tile is legal at each corresponding
decision. The deterministic field reproduces the same history by induction.
Thus the full permutation family covers every one-scenario attainable trace;
its column maximum is the exact scenario-revealed best-response payoff.

A restricted plan pool still supplies a valid lower bound. Its column maximum
does **not** supply an upper on the unrestricted best response. Retain an exact
CPU scenario-relaxed upper or the trivial upper until a stronger bound is
actually established. This distinction is essential to an incremental plan
generator.

Complete traces also permit an exact alternative implementation: build their
union by full public history, count each original scenario once per node,
retain all legal focal edges, partition complete observation buckets, and
back up SUM/MAX. Multiple plans producing the same scenario/history must not
multiply its mass. This is reconstruction of the sampled information tree,
not a proof that its construction is cheap.

**Why 5,040 is not by itself a performance argument.** Doing full permutation
enumeration at every nested modeled-policy call would be wasteful. On the
captured depth distribution, the conservative n*k!*4k physical-step budget
totals roughly 175 million, versus 3.399 million recorded CPU search nodes.
At depth 7 alone it permits roughly 49.7 million steps for only 21,574 recorded
nodes. These are a worst-case rollout ceiling and observed DFS counts, not
like-for-like timing, but they expose the danger: current pruning frequently
settles a request long before all plans matter. Enumerate a small useful policy
pool and refine it, or use full enumeration only where measurement supports it.

A bounded CPU-only structural census now checks the first four captured
requests per remaining depth 2–7, selected before evaluating outcomes. Across
24 requests and 85 root actions, every interval contains the independently
computed adaptive response, which also matches the frozen v22 action vectors.
The plan lower equals Q on 73/85 actions; valid bounds establish the canonical
chosen action on 23/24 requests. This is an unrepresentative structural panel,
not a success-rate estimate. Its four depth-7 requests all have saturated
values and do not establish difficult-opening coverage.

Two retained witnesses explain the distinction. At seed 420601, job 68,
depth 6, root tile 13 has actor-success counts L=6, Q=8, U=8 out of eight
scenarios: adapting to observations beats every fixed priority plan. At job 7,
depth 4, root tile 24 has L=3, Q=3, U=4: the world-and-tape-informed upper is
too optimistic even though a fixed plan already attains the lawful optimum.
The census enumerates 9,234 plans and 73,872 scenario rollouts; its 4.572-second
Python runtime is a correctness-instrument cost, not a proposed player timing.
See [census evidence](../results/priority-plan-census-v1/20260920T042548Z-91944/summary.json).

**The proposed player uses a lawful incumbent and selective refinement.**
For each root action, maintain a materialized total continuation policy and its
exact value on B as a lower witness. Keep an upper witness against the same
frozen field. Initially, existing cheap lawful policies or a small plan pool
provide floors; a scenario-relaxed solve provides the upper when affordable.
An extracted policy defined only on sampled branches needs an explicit lawful
default for every other reachable observation before it can be replayed on
fresh scenarios. The default is part of the frozen policy's identity.
Refine complete focal information states on branches whose remaining gap can
still change the root decision. GPU candidates are broad fixed-policy scoring
and field-only evolution to the next focal decision. CPU tasks are scheduling,
small exact searches, policy construction and final tie/bound checks.

Refinement must extend a lawful prefix before revealing the scenario at its
frontier. It cannot reveal a hidden scenario to choose an earlier action and
then pretend that regrouping a later node restores perfect recall. General
partial nonanticipativity constraints require global optimization; arbitrary
local merging is not justified by a MAX/SUM backup.

For a fixed B and field, retaining previous witnesses gives

```
L_new(a) >= L_old(a),     U_new(a) <= U_old(a).
regret_bound(rho_incumbent) = max_a U_a - value(rho_incumbent).
```

At exact separation, deterministic root tie behavior must also be checked.
At a deadline, return the incumbent with the remaining bound, not an incomplete
search value. This is an approximate best response when the bound is nonzero,
with an explicit model-relative error. It is not a byte-compatible replacement
until every required action value and choice is exact.

For the nested ladder, lower-level policy queries must be pure functions of
their lawful information and declared model identity. A CPU demand graph can
deduplicate pending queries and release all ready lower-rung work in batches;
the same scheme applies recursively. Actual dependency width, graph cost and
critical-path delay need measurement. An approximate lower-rung policy must
also be frozen and versioned for each outer comparison. Improving that policy
changes the outer field: old values and bounds cannot simply carry over.
Wall-clock-dependent inner choices would also break purity across schedules.
Use a declared deterministic work budget or a frozen total policy revision for
the model; reserve wall-clock interruption for returning the outer incumbent.
A precomputed or distilled lower-level
policy is a separate changed field: the upper solver can best respond to it,
but the distilled policy cannot be called an exact lower-rung best response
without an independent error argument.

**The prior math supports this direction and limits its claims.**

- [SCENARIO-PLAYER](../../../walt/SCENARIO-PLAYER.md), sections 2–7,
  supplies lawful information, deterministic field/tape and alive-partition
  semantics. Its historical defaults are superseded where the current
  partnership contract explicitly selects Fixed 40/8/2.
- The maintained [GPU contract](../../../walt/GPU-NATIVE-TRICK1.md), sections
  6–7, and frozen [M3 contract](../../../walt/GPU-NATIVE-TRICK1-M3.md), sections
  3–4, govern information identity, complete grouping, common lower policies
  and canonical root selection. The earlier received
  [GPU implementer's guide](../../../walt/math/gpu_native_trick1_implementers_guide_v0.2.md),
  sections 9 and 13, already proposes field-only focal epochs and bulk
  reductions; the maintained contracts govern wherever they narrow it. The
  old projector receipts are not player results.
- [Focal-horizon intake](../../../walt/math/focal_horizon_sandwich_v0.1_intake.md)
  supplies the lower/upper and interruption discipline. Its original exact
  fixed-field contract is distinct from the finite scenario/tape adaptation
  proposed here; that adaptation needs its own differential checks.
- [Negative results](../../../wiki/walt-negative-results.md), S5h/S5j,
  explain why retrograde class identity and canonical tablebase lookup can
  cost more than the solve they replace. A known suffix needs full belief,
  field, tape, utility and lawful-information identity; matching the physical
  layout alone does not justify substitution.
- [Counted-belief era](../../../wiki/walt-counted-belief-era.md) records a
  specific focal-horizon study where improving a lawful tail mattered more
  than extending search. It also records opening-root plateaus. Neither
  result is a universal tractability or playing-strength theorem.

**What would count as progress.** First compare a flat fixed-policy/epoch
primitive against the frozen exact CPU control on matched inputs, including
all generation, grouping and reduction costs. Measure meaningful work per
second and ready width, not shader arithmetic alone. Then validate a total
anytime player from trick 1, with the same information isolation at every rung
and exact agreement whenever its bounds close.

Three competing hypotheses remain live; the mathematical review does not
collapse them into one implementation promise:

| Hypothesis | What stays exact | Main unresolved cost or risk |
|---|---|---|
| Demand graph and bulk field epochs | The old sampled values and complete response ladder, if scheduling alone changes | Discovering ready work, grouping it and preserving pruning may cost more than direct CPU recursion |
| Anytime lawful policy bounds | Bounds for the fixed field and bundle; the response is exact only when closed | A weak policy pool or loose upper may not distinguish actions within the budget |
| Amortized lower-rung policies plus online response | The online search can be exact against the frozen compiled field | Lower policies approximate their teachers; training and model mismatch can erase any runtime or strength gain |

The third is the most radical way to avoid repeated lower-rung construction.
It should be compared with finite compiled decision programs as well as learned
policies. Neither is automatically an exact ladder. The second is the nearer
experiment because it has a usable incumbent, explicit mathematical bounds and
small existing policy machinery; the first can improve either backend.

The quality experiment should compare complete played policies under equal
wall-time budgets, paired deals and seat rotations, and a preregistered holdout.
Use contract outcomes, legal completion, deadline behavior and timing tails;
larger samples, different moves and higher internal values are not wins.
The conservative working budget is the latest optimized player's scale,
roughly 0.5 seconds for an entire game in its last panel, with a separate
100-ms whole-game target. No quality improvement has yet been measured.

Companion reviews: [mathematical review](MATH-LADDER-REDESIGN-REVIEW.md) and
[equal-time quality protocol](QUALITY-AT-FIXED-TIME-DRAFT.md). These drafts
keep the strongest proposal and its failure tests separate from the existing
validated v34 CPU checkpoint and the interrupted allocation/profiling work.
