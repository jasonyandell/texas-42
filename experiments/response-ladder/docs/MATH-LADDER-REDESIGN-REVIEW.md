# Mathematical review of alternative best-response ladders

2026-09-19. **EXPLORATORY DESIGN REVIEW.** The short proofs below are
arguments over explicitly frozen finite models, not promoted claims or
implementation receipts. This review runs no benchmark, builds nothing, and
changes no player. Its recommendation is to preserve the exact CPU path while
testing a demand-driven execution graph and a separately typed policy-bound
player. A complete permutation enumeration is a useful mathematical reference;
the available work census argues against installing it at every recursive mind.

## 1. What the proposed changes would mean

The existing speed contract fixes every sample, modeled policy, tie rule,
complete rational root vector, and resulting choice. The current all-play
L2-Partner configuration has 40 outer worlds, 8 L0 worlds, and 2 L1 worlds;
these are configuration labels, not strength guarantees. In
[`partnership.rs`](../native/walt/src/solver/partnership.rs), `PartnerOnly`
means the partner is a fixed L1 field, opponents are fixed L0 fields, and the
focal seat alone best-responds. This is a **seat best response**, not joint
optimization of two partners' private-information policies.

Three distinct outputs must remain distinct:

| Output | Claim permitted |
| --- | --- |
| Exact replay of the frozen configuration | Same complete root values and canonical choice as the existing finite sampled model |
| Root separation from valid bounds | An optimal member, a canonical member, or a regret bound for the named model; generally no complete exact root vector |
| Approximate or amortized response policy | A lawful policy whose playing strength and off-sample error require measurement |

An exact comparison on sampled scenarios is not an exact fiber-wide response.
The lawfulness fiber is the possible hidden-hand support; a belief supplies
weights on it. Equal support does not imply equal belief or equal best action.
The outer player currently conditions on public void information; the frozen
Voidless inner model deliberately omits that conditioning. An inner sampled
residual deal may be mechanically consistent without being reachable through
the observed prefix. Optimizing a policy against that model does not repair its
belief approximation.

The ordinary information state is one's own hand and complete public
actor-attributed history, with the public contract. A policy may use any
deterministic function of that observation, including deliberately ignoring
information. Replacing the full optimizing domain by a reduced state requires
a sufficiency argument to preserve exact response values. A policy cannot use a hidden deal,
random-tape identity, sample ID, clairvoyant chosen action, or arbitrary
component ID as an additional observation. `PiKey`'s reduced public record is
the current sampled model's represented state; importing a new full-history
field API is a field change, not an innocent cache optimization.

Sources: [`SCENARIO-PLAYER.md`](../../../walt/SCENARIO-PLAYER.md) §§2–7 and
O1/O22/O29/O36; [`walt-program`](../../../wiki/walt-program.md);
[`GPU-NATIVE-TRICK1.md`](../../../walt/GPU-NATIVE-TRICK1.md) §§6–7;
[`M3 contract`](../../../walt/GPU-NATIVE-TRICK1-M3.md) §§3–4.

## 2. The algebra that must survive every accelerator

Fix a focal seat, root information state, declaration, objective, finite
weighted scenario multiset, and deterministic field. A scenario is
`xi=(physical world, all latent field randomness)`; original IDs distinguish
equal deals with different tapes. Write `u` for the focal team's Boolean
success, so all formulas maximize. For a T0 focal seat the existing exposed
T1-make values are recovered by complementing; lower and upper endpoints swap
under this conversion.

Let `P_a` be all lawful focal policies with legal root action `a` and let

```
Q(a) = max_{pi in P_a} sum_xi beta(xi) u(pi, xi).
```

The field may itself be computed by recursive lower-rung searches. Once its
configuration is fixed, each completed modeled action is a pure policy answer,
so that recursive construction is distinct from the focal optimization above.

### Sum before a shared choice

At one focal information state, the same action must apply to all scenarios
with that observation. Its value is `max_a sum_xi contribution(xi,a)`, not
`sum_xi max_a contribution(xi,a)`. The two equally weighted scenarios with
payoffs `(1,0)` and `(0,1)` give lawful value `1/2` and per-world value `1`.
The error persists if both erroneous keys contain the correct hand and history
but also contain a hidden component ID.

At a field node, all alive scenarios choosing the same observed move belong to
the same child bucket. Descending into provisional subsets permits separate
later focal maxima and reproduces the counterexample. The precise requirement
is **bucket completeness before its descendants can optimize**. Classifying
every alive ID first is sufficient. The current `closed-buckets` variant has
another sufficient condition: no unclassified hand can legally choose that
bucket's move. Thus it can close a bucket early without weakening the rule.

For unit-weight finite scenarios, unnormalized values are successful-scenario
counts: complete field buckets SUM, focal choices MAX, terminals return zero
or alive mass. For general weights use their exact additive mass. A MAX copies
the same counterfactual arrival mass into each action; it never sums that mass
across competing focal actions. Posterior normalization is unnecessary for the
comparison and must not turn surviving components into a fresh uniform prior.

### Bounds and what they settle

For a finite set of complete lawful policies `K_a subset P_a`, define

```
L(a) = max_{pi in K_a} E_beta[u(pi,xi)]
U(a) = E_beta[max_{world-and-tape-informed continuations after a} u].
```

Every lower candidate belongs to the full feasible policy set, and every lawful
policy is feasible for the informed optimization, so `L(a) <= Q(a) <= U(a)`.
The upper is **C+**, not unqualified C: it reveals the tape as well as the
physical deal. For tape-free deterministic fields the distinction disappears.

If `L(a*) >= U(b)` for every competitor, `a*` is an optimal member. To preserve
the least-tile tie rule, require strict exclusion of every lower-index
competitor as well. If an executable incumbent `pi*` has value `L_exec`, then
`max_a U(a) - L_exec` bounds its model-relative regret. Neither statement
supplies exact values for all losing actions. Complete-vector parity requires
collapsing every requested interval or evaluating every value exactly.

Adding complete policies to `K_a` cannot decrease the lower bound on the same
belief and field. Tightening a genuine relaxation cannot increase its upper.
These monotonicities do not survive an unannounced field, sample, belief, or
objective change. Optimizing and evaluating on the same finite panel also
provides no unbiased off-sample quality estimate.

This is the local form of errata **E3/E4/E6.3–E6.5** and rulings
**DS-A7/DS-A15/DS-A16/DS-A20**; the maintained
[`errata`](../../../walt/math/decision_sparse_exact_solving_v0.1_errata.md)
governs over the received parent. The working
[`GPU DFS review`](GPU-DFS-REPLAY.md) and
[`compact_policy.rs`](../native/walt/src/solver/compact_policy.rs) carry the
corresponding finite-count and complete-bucket rules.

### Level is not a monotone quality coordinate

For one unchanged field and belief, an exact best response is at least as good
as any competing lawful policy against that field. The next ladder level
changes the field. In rock-paper-scissors, responding to rock with paper wins
against rock; responding to a newly modeled paper opponent with scissors loses
against the original rock opponent. A higher response index can therefore be
worse against a fixed real opponent.

A deterministic best-response operator on a finite policy-profile space is
eventually periodic: two iterates coincide by finiteness, and determinism makes
their successors coincide forever. Period one is not guaranteed. This is the
existing **L2-T5** result in
[`targeted_level2_field_stability_v0.1.md`](../../../walt/math/targeted_level2_field_stability_v0.1.md)
§13. Its fixed-operator premise must be checked: a tower whose level-specific
seeds, sample budgets, or approximation procedures change with the absolute
level is not automatically iteration of one stationary operator. A repeated
hash on a test panel is not an exact cycle on a dependency-closed domain.

## 3. Architecture A: expose the dependency graph, preserve the search

**Hypothesis.** The GPU can become useful if exact L0 work is presented in
large naturally ready batches without discarding the CPU's pruning.

Turn a missing `pi(k, key)` from a recursive blocking call into a suspended
continuation and a typed demand. Collect identical complete policy keys once,
with all waiters. An L1 demand exposes lower L0 demands; a completed L0 answer
releases its dependents. No upper-rung task can read an incomplete lower-rung
policy. Pure completed answers permit any scheduling order.

Each ready L0 packet retains its ordered worlds, original tapes, objective,
complete state, and sample multiplicities. Batch by remaining tiles and
estimated work, because a few large jobs can hold a whole divergent GPU group
open. CPU workers should continue cheap jobs and naturally narrow queues;
GPU dispatch is appropriate only once the measured end-to-end break-even batch
exists. A replay batch assembled after a whole game is not evidence that such
a batch was simultaneously ready inside that game's live dependency graph.

There are two execution choices within this architecture:

1. Retain pruned DFS within each ready packet and batch packets. This is the
   smallest extension of the current prepared-L0 replay.
2. Expand a bounded wave of complete observation frontiers across many packets,
   store a compact structure of arrays, and do bottom-up segmented SUM/MAX.
   Complete faces and complete buckets must seal before their value reduction.
   A suspended frontier still carries valid intervals; it is not an exact leaf.

The second choice is an execution DAG, not a universal state quotient. Start
with canonical full public histories. Merge transpositions only after proving
that the represented public state, alive weighted scenario set, and fixed field
give identical continuation problems. In the present reduced-record solver,
remaining hands, score, legal future transitions, Dice choices and recursive
policy queries are functions of that complete represented state. This provides
the route to a sufficient-state proof; matching only masks or only a hash does
not. An arbitrary full-history field invalidates that reduction.

The perfect-recall net theorem already has a maintained home: the M3 contract
retains all legal faces, gives each full-history child a unique preceding focal
parent/action, and licenses the root-scale recurrence
`A_I(a)=T_I(a)+sum_J V_J; V_I=max_a A_I(a)`. It is a **grade-4 gate**, not a
trick-1 implementation or performance result. Reuse its algebra and proof
obligations without importing its carrier claim.

**Decisive cost test.** Record live ready-queue width, critical-path depth,
waiter fan-out, canceled/speculatively unused demands, peak graph bytes, number
of dispatches, CPU preparation time, GPU time, synchronization time and total
28-play wall time. Preserve exact full-vector parity. Reject the architecture
as a speed solution if batch formation and extra work erase its kernel gain.
Keep compilation, cache construction and disposal in cold-game timing. A full
forward expansion is not an acceptable free precomputation assumption.

## 4. Architecture B: a regular policy matrix with lawful lower witnesses

**Hypothesis.** A small pool of complete lawful continuations plus cheap upper
witnesses can spend time on decision ambiguity instead of solving every
modeled branch to completion. Its exact endpoint is optional; its intermediate
outputs must expose their bounds and policy scope.

### A useful complete family, and the exact theorem it supports

With `k<=7` focal tiles remaining and root action `a`, enumerate permutations
of the other `k-1` tiles. Each permutation defines a total priority policy:
force `a` now, then play the earliest still-held legal tile in that order.
It is lawful because the order is fixed for the whole policy and legality is
observable. All legal roots together require at most `k!<=5040` policies;
on a partial trick there may be fewer legal root actions.

For fixed scenario `xi` and deterministic field, **every attainable focal
play trace is induced by one such priority policy**. To prove this, take the
trace's own-tile sequence, beginning with `a`, and append unplayed tiles in any
order after termination. At each focal turn every earlier priority tile has
already been played, and the next trace tile is legal. The priority policy
therefore selects it. The deterministic field then reproduces the next public
segment, completing the induction. Early decided-payoff termination is harmless
because the priority order can be extended after that terminal prefix.

Build `alpha[p,xi]=u(priority_p,xi)` by one straight, at-most-28-play rollout
per lane. There is no DFS stack in a lane. For the complete family,

```
L_priority(a) = max_p sum_xi beta(xi) alpha[p,xi]
U_Cplus(a)    = sum_xi beta(xi) max_p alpha[p,xi].
```

The first is a common-policy lower bound. By trace coverage, the second is
the **exact C+ best-response value**, not merely a heuristic upper. A subset of
priority policies still gives a valid lower but generally does NOT give a valid
upper: its per-world maximum can miss an attainable win. A cheap subset must
therefore use a separate exact or admissible-upper evaluator.

Complete trace coverage is not complete policy coverage. Suppose that after a
forced root, a public field move reveals which of two cases holds. The focal
seat then has `x,y` both legal, with `x` winning only case one and `y` winning
only case two. A full lawful policy reacts to the public signal and wins both.
Every fixed priority selects the same first tile in both cases and wins only
half. This synthetic witness describes the distinction; it is not asserted to
be a newly verified 42 deal. Richer observation-dependent policies are the
mechanism needed to capture partner signals, role changes and count sloughing.

The payoff matrix is compact: all 5040 policies across eight scenarios require
5040 bytes of Boolean payload, before metadata. Storage compactness does not
make its production cheap.

### Can the traces reconstruct the full imperfect-information response?

Yes, if the complete family is rolled out in every original scenario and all
public traces are retained. Trace coverage gives every feasible focal edge in
every scenario. Deduplicate repeated occurrences by **original scenario ID and
full public history**, then union by the focal's complete lawful observation.
Retain every legal focal action, complete field buckets, and all distinct
scenario mass. Plan IDs are provenance only; they must neither enter information
keys nor multiply the mass of repeated arrivals. Duplicate physical deals with
different tapes stay distinct scenarios.

The resulting full-history tree/net has the same feasible public continuations
and information partition as the frozen sampled problem. Backward SUM/MAX then
returns its exact best response. A fixed extracted common policy can be repriced
without further maximization as an independent semantic check. Equality with
the present reduced-record implementation also depends on its sufficient-state
argument described under Architecture A.

This is a finite correctness construction, not an economy claim. It can expand
counterfactual branches pruned by the CPU and materialize many duplicate traces
before deduplication. At `k=7,n=8`, the padded bound is 1,128,960 physical
transition slots per job; even 16 bytes per slot is about 18 MB, excluding
history keys, indices and sorting scratch. The tiny payoff matrix hides this
larger optional trace inventory.

### The work census argues for selective use

The existing
[`v18 weighted summary`](../results/score-census-v18-weighted-summary.json)
contains 43,061 deduplicated full-score L0 requests and 3,396,906 recorded DFS
visits. Applying the conservative all-priorities padded envelope
`requests(k) * 8 * k! * 4k` gives:

| Remaining tiles/tricks `k` at the on-turn request | Requests | Recorded DFS visits | Padded permutation transition slots |
| --- | ---: | ---: | ---: |
| 2 | 20,965 | 320,313 | 2,683,520 |
| 3 | 14,125 | 1,038,803 | 8,136,000 |
| 4 | 5,727 | 1,085,361 | 17,593,344 |
| 5 | 1,738 | 632,109 | 33,369,600 |
| 6 | 462 | 298,746 | 63,866,880 |
| 7 | 44 | 21,574 | 49,674,240 |
| Total | 43,061 | 3,396,906 | 175,323,584 |

This is an arithmetic envelope, **not measured GPU work, a lower bound on an
optimized permutation implementation, or a wall-time ratio**. Legal-root
filtering, early score decisions and inactive-lane handling reduce actual work.
Nevertheless a design that executes the padded envelope schedules about 51.6
transition slots per recorded DFS visit; the seven-tile row is about 2303.
Different visits also have different costs. Regularity has a large pruning
deficit to repay before it can win. Do not prescribe all 5040 policies for
every recursively requested L0 mind.

### A more plausible policy-bound player

Use a small, declared pool of total lawful policies: selected priorities,
the incumbent, and complete observation-dependent continuations produced by
the existing solver or a policy program. Evaluate them on the same scenario
panel and retain exact lower values. Obtain the upper from a pruned per-scenario
C+ solve or the existing focal-horizon upper, not from an incomplete priority
pool. L0 Dice upper evaluation can reuse the singleton solver's algebra while
retaining each original scenario's tape; its added cost still needs measuring.

When bounds do not separate actions, choose between adding a better complete
lower policy and refining a lawful focal prefix. This follows the existing
focal-horizon sandwich: below an explicitly declared frontier, reveal the
latent scenario for the upper; above it, enforce the common information-state
policy. Refining one frontier means retaining its COMPLETE legal face and
complete alive buckets before the next reveal. A better executable tail raises
the lower without requiring another exact focal layer.

Nested anytime choices need an additional freeze. A lower-rung policy selected
at a deterministic declared work budget can define an immutable field revision.
Its input includes that revision and its own lawful observation; completion
order, queue width and elapsed wall time cannot choose a different action under
the same identity. Improving that lower policy creates a NEW field revision.
Outer lower/upper facts derived against the old revision must then be recomputed
or transported by a separately justified field-change bound. They cannot be
carried as monotone facts about the new response problem. An immutable action
registry is sufficient only with information-consistent construction and a
declared total off-registry policy; freezing whichever schedule-dependent answer
arrived first does not prove cache purity.

Do not implement arbitrary downstream "gluing" by local reductions after a
world-informed earlier decision. That can merge branches after the controller
has already used hidden information, breaking the unique-parent/perfect-recall
argument. Arbitrary nonanticipativity constraints define a valid upper through
feasible-set inclusion only when their global optimization is actually solved;
they do not automatically admit this local Bellman recurrence. A lawful prefix
followed by one-way revelation has the safe structure already available.

**Decisive tests.** On tractable roots, verify
`L_priority <= Q_H <= U_Cplus`, exact C+ trace coverage, the synthetic adaptive
signal witness, canonical tie handling, and complete trace-union parity.
Then measure decisions separated, sample-relative regret, field calls, physical
steps and total latency at each policy/frontier budget. Compare against the
current exact search at the same wall budget. Stop promoting this route if the
upper stays saturated at one while lower policies do not approach it, or if
constructing the bounds costs more than the search they replace.

## 5. Architecture C: amortize the field, retain lawful response semantics

**Hypothesis.** Replacing repeated online construction of lower-rung minds by
fast, frozen lawful policy programs leaves more time for a richer focal policy
or a better evidenced decision. This is a new field model and quality track.

Offline, construct a total policy `pi_hat_0(I)` approximating the declared
response to Dice. Freeze it. Construct `pi_hat_1(I)` by approximate policy
improvement or distilled response targets against that frozen field. Continue
only while held-out play supports a useful next rung. A finite decision program
and a learned network are competing implementations; neither needs a universal
lossless quotient of all histories.

Every policy consumes only the acting seat's legal observation and returns a
legal move under a deterministic published tie rule. A learned representation
of the complete record is permitted as an approximation; it is not a sufficient
statistic theorem. A total legal default covers observations absent from the
training/discovery panel. A sparse table of sampled branches without that
extension is not a replayable policy on fresh worlds.

Bulk inference across hidden hands at one public history can share the public
record computation. It must retain each modeled seat's distinct own hand and
never supply the other hands to that seat's action function. Freeze the model
weights/program, observation codec, objective, legal-mask procedure and tie
rule into a `FieldId`. A field is fixed throughout an outer search; training
or changing it while that search runs changes its target.

The online focal seat can then compute an **exact sampled response to the
amortized field**, or use Architecture B's lawful policy bounds against it.
Those are different accuracy statements. The approximate field is not an exact
L0/L1 best response simply because its final focal solver uses integers.
Likewise, a partnered deployment still requires each seat to act from its own
observation; joint access to both partner hands is not an implementation of this
proposal.

This is the strongest candidate for substantially different quality at similar
time, because it attacks the repeated construction cost rather than only
rearranging it. It has no present strength result. In particular, more outer
worlds cannot repair a bad partner model, and fitting a response to an
approximate model can amplify that model's mistakes.

**Decisive quality tests.** Use disjoint discovery and evaluation deals/seeds;
freeze every policy being compared. Run whole hands with mirrored seats and
cross-play against the frozen incumbent, differing opponent styles, and both
matched and mismatched partner models. Report the declared marks/make objective
and decision latency together. Include all four chair roles, turn orders,
trick-1 openings and later positions. Preserve concrete action, observation and
continuation traces for wins, losses and reversed effects. An aggregate win
count without those mechanisms cannot show whether a model learned partnership
or only fit its training opponents. Statistical intervals must respect paired
deals and any sequential stopping rule; in-sample BR values are not those
intervals.

## 6. Why present hashes and random couplings limit reuse

In [`Solver::pi`](../native/walt/src/solver/mod.rs), the initial stream is

```
INNER_SEED ^ level_tag(k) ^ mix(seat) ^ mix(own_hand)
           ^ record_hash(played, leader, ordered_partial_plays).
```

`record_hash` excludes banked scores and voids; the complete policy cache key
still includes the applicable score/void context. Changing the public prefix or
own hand generally changes the entire lower-rung sample construction. Adjacent
states therefore cannot normally share a prepared sampled subtree just because
their mechanical hands overlap. Current Dice actions also depend on the
original scenario's tape and that public-state hash, so a permutation of
otherwise similar moves can change every later draw.

The Voidless sampler shuffles its tile array in place between worlds and draws
all Dice seeds after all worlds. Resetting the array, independently seeding each
sample, changing bounded-draw mapping, or increasing the requested panel while
expecting old tape identities to remain fixed changes the frozen coupling.
Execution-order independence is available through pure keyed jobs; arbitrary
within-job resampling is not an exact scheduler transformation.

Some exact reuse is already understood: matching full context can carry
completed policy answers across valid game boundaries; removing only scores
can share initial prepared worlds/tapes. The
[`cache context`](CACHE-CONTEXT.md) and
[`score projection`](SCORE-PROJECTION.md) notes supply the proofs and the
measured limitations. The latter's sampled node-weighted census finds very
little removable root-score work; it does not rule out useful descendant reuse
inside a prepared job. Hashes are prefilters, not sufficient equality tests.

A new indexed sampler can separate `WorldStreamId`, `TapeStreamId`, and sample
ordinal, permitting prefix-stable enlarged panels and selected common-random
couplings between decisions. It needs a distinct `CouplingId`, exact marginal
sampler semantics, duplicate/multiplicity rules, and evidence that it never
provides hidden randomness to the focal policy. Changing coupling can change
the empirical optimum even when one-step marginals match, so this belongs to
the new model/quality track. No quality improvement follows from the redesign
alone.

## 7. Existing failures are constraints on these proposals

The maintained
[`negative-results page`](../../../wiki/walt-negative-results.md) already
rules out four seductive shortcuts at their stated exploratory scopes:

* The first-play structural quotient is the identity on 1,184,040 focal hands;
  exact linear predictive closure can saturate the hidden-world dimension.
  None of the proposals should assume a small universal opening state space.
* A retrograde class DAG lost its first-build comparison to ordinary memoized
  solving. A compiled execution graph must therefore earn its construction cost
  against the current memoized/pruned engine, not an unoptimized tree walk.
* Structural endgame tables lost because canonicalization cost exceeded saved
  solving. A lookup count or matrix compression ratio is not a speed result.
* Policy frontiers sometimes collapsed and sometimes exceeded the declared cap.
  A partial frontier is not an exact frontier or upper bound. Architecture B's
  incompletely discovered policy pool is useful precisely because it is typed
  as a lower witness and paired with an independent valid upper.

The full-history field-cache
[`funnel probe`](../../../walt/probes/field_cache/README.md) found only a small
opening benefit: expensive early requests had distinct histories and hands,
while many recurring requests were cheap late ones. Its field API and timings
are not identical to the current reduced-record experiment, but it explains why
"cache every history" is not an unmeasured universal answer.

The current [`Walt map`](../../../walt/MAP.md) and
[`FH1/FH2 record`](../../../walt/FACTOR-BELIEF.md) already contain the lawful
tail / informed upper / exact focal-prefix ladder. Their declared late-game
measurements suggest lower-tail quality can matter more than tightening the
upper, and show that reusable proof stores can grow very large. That motivates
Architecture B's question, but does not transfer a late-game timing or quality
claim to trick 1.

## 8. Recommendation and explicit failure criteria

Keep **Architecture A** as the exact acceleration candidate: expose naturally
ready lower-rung work, preserve complete information buckets and the existing
pruning, and require complete rational-vector parity. Its unresolved question
is available concurrency on the live critical path, not correctness of an
already captured independent batch.

Test **Architecture B** as the mathematical quality/cost instrument: a small
total lawful policy pool, an independently valid C+ upper, and complete lawful
prefix refinement only where the root remains ambiguous. Use the full
permutation family as a trace-coverage reference or selected small-tail kernel,
not the default replacement for every L0 solve. Report when its bounds fail to
separate; retain the finite model and completed executable policy in the result.

Treat **Architecture C** as the radical competing quality direction if repeated
lower-mind construction remains dominant. It preserves the one-chair policy
boundary while deliberately changing the field and response approximation.
Choose it only on measured equal-time whole-hand quality, not higher level
labels or larger sample counts.

For all three, predeclare cold 28-play latency, peak memory, completion/refusal
rate and quality comparisons. A 100 ms target is achieved only by that complete
workload; a fast isolated kernel is not its proxy. Better quality at the current
budget is a separate successful outcome, provided the changed model and evidence
are explicit. The propositions establish which computed objects are lawful and
which inequalities are valid. They do not establish that any architecture is
faster or plays better.
