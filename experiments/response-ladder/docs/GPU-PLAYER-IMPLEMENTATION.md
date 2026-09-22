# Native GPU player implementation — 2026-09-20

The M5 player now makes real trick-1 imperfect-information decisions with the
GPU contributing completely priced policy witnesses against frozen modeled
players. The [current-CPU comparison](../results/gpu-current-h2h-v1/REPORT.md)
found no observed quality gain and substantially worse latency. This is an
executable exploratory instrument, not a production replacement.

## Mathematical contract

One response problem fixes the contract, focal own hand, complete public root,
ordered sampled scenarios with integer weights, and declared lower-model
revision. A scenario consists of its original hidden allocation and Dice tape.
Repeated allocations remain separate columns. Only the queried seat's own
hand and public information enter a modeled lower-rung choice. Outer tapes
are absent from modeled L0/L1 calls.

A policy is total: a sorted full-history decision map, followed by a fixed
priority and least-legal-tile default. All scenarios that share an observed
history receive the same focal action. Pricing completes every original
column before exposing its exact integer sum. This is a sampled lower witness;
it is not a perfect-information choice separately optimized in each world.

The CPU controller combines complete field-observation buckets with SUM and
lawful focal choices with MAX. Its remaining continuation is bounded using
the exact scenario-revealed relaxation or the trivial total mass. Restricting
the initial policy family never licenses a column maximum as an unrestricted
upper bound. Completed lower and upper improvements apply only to the same
immutable response problem. A root certificate, a zero-regret continuation,
and an exact full root vector remain distinct report fields.

## Executed work

1. Construct a completed cheap L1 reserve where the allowance permits. Its
   separate model is all-L0 with 8 outer scenarios and n0=2. Reserve preparation
   receives at most a quarter of the move allowance, capped at 50ms, and consumes
   the main wall deadline. Its price never enters the main L2 bounds.
2. Sample the main immutable bundle using the frozen historical stream. Build
   an exact frozen field for the requested ladder coordinate: Dice, all L0,
   Partner (partner L1, opponents L0), or all L1.
3. Price one initial priority rank across all legal root actions atomically.
   Each GPU lane holds one total policy and one original scenario. It executes
   focal choices, forced field moves, legal game transitions, void deductions
   and scores, then pauses at a nonforced field decision.
4. Reconstruct each query's public state on CPU. Collapse repeated generic
   queries by full state, own remaining hand and tape. FrozenModel additionally
   collapses its exact legacy identity `(level, seat, hand, Key)`; Key includes
   both banked scores. Contract, sample budgets and sampling boundary are owned
   by the model instance. Resolve distinct exact queries in parallel and fan
   their lawful answers back to the original lanes.
5. Resume epochs until every lane settles. Publish only an entire on-time
   matrix. Retain completed ranks when a later rank is interrupted, then spend
   remaining time on the CPU's information-consistent horizon refinement.

The GPU/device pipelines survive decisions. Batch buffers and generic caches
are scoped to a policy batch; the deadline-bearing model is scoped to the move.
Queries from different original columns may share work but retain their
individual payoff weights. Materialized full-history policies are supported,
including choices that differ from the priority default.

The default player is a finite anytime approximation of the full best response
until its bounds close. Completely solved test problems preserve Walt's exact
root vectors and canonical choices. Deadline-limited play may choose differently
from current Walt because it follows different completed policies and bounds.

## Cancellation and accounting

Device completion uses nonblocking polling with deadline checks. A dispatched
but overdue batch is abandoned and its mapped buffer is safely unmapped; a
later decision can reuse the same device. CPU modeled calls either finish
their exact declared computation or abort the batch. There is no approximate
inner response selected because time ran out.

`field_requests` counts paused physical lanes; `unique_field_queries` counts
generic queries submitted after the full-state/tape cache; and
`modeled_unique_queries` counts distinct frozen modeled identities submitted
inside those query batches. The last count can include work later cancelled.
Device and field milliseconds are measured wall components, not hardware-only
kernel durations or aggregate CPU core-seconds. Controller time also includes
reserve, sampling, horizon work and cleanup. The deadline is cooperative:
external response time includes IPC and serialization and can exceed it.

## Verification

The seven modeled-epoch tests include 1,480 complete trace comparisons over
all nine declarations, all four partial-trick offsets, both partnerships,
depths 2–7, nonuniform scenario weights, and duplicate allocations with
distinct tapes. Scalar replay through frozen Walt independently checks all
tiles and payoffs. Directed checks cover:

- Canonical history, void, score and remaining-hand query reconstruction.
- A reachable stored policy choice differing from its priority default.
- GPU repricing of extracted adaptive policies and completed root vectors.
- Preexpired, field-aborted and already-submitted batch cancellation.
- Keeping a completed priced incumbent when the next batch aborts.
- Persistent device recovery following cancellation.

The full feature suite passes: one unit, eleven core, two modeled-adapter,
seven modeled-epoch and five rollout tests. The CPU-only suite and portable
CPU library check for `wasm32-unknown-unknown` also pass. All 1,035 frozen
reference files verify against the included archive. An independent read-only
agent reviewed information boundaries, query identity, replay, transactional
bounds, deadline handling, pairing and timing controls; no remaining correctness
blocker was found. These are engineering checks, not a kernel proof.

## Measured boundary and next hypothesis

The WGSL uses ordinary WebGPU storage/compute limits without optional device
features. The current host is native Metal on Apple M5 Max. Browser GPU plumbing
and phone measurements are not implemented; the CPU library's WASM compile
does not establish them.

The ready field frontier is batched, but horizon search and nested modeled
minds remain on CPU. The controller still recomputes horizon passes. It does
not yet have a persistent horizon frontier, compact policy DAG, cross-batch
field-query cache, or recursive GPU lower-mind engine. These are performance
extensions of the design, not measured successes.

The current comparison spent 17.85s in device epochs and 19.83s in field
resolution, while the current CPU partnership used 9.96s total. There were
70,369 device epochs and 6.81 million modeled query submissions. The next
hypothesis is to eliminate repeated work across policy ranks and move broad
lower-rung work onto the device while retaining small exact CPU continuations.
It must reduce measured end-to-end time on new fixed cases; increasing the
number of policy rows alone has not earned deployment.
