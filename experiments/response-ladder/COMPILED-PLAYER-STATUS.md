# Compiled player campaign

Active user objective: realize the compiled-rung player and demonstrate that it
beats current Walt head to head at similar measured wall time. Use short
one-minute work cycles, preserving completed work and failures. A working actor,
a faster kernel, or an inconclusive tournament does not complete this goal.

## Initial state

- Implementation base: `5bb579d4`, branch `codex/walt-response-ladder`.
- Design: `docs/COMPILED-LOWER-RUNGS-DRAFT.md`.
- Production anchor checkout: `/Users/jason/code/texas-42-partnership-launch`,
  observed `d24eaefe`; pin its executable and source identities before comparisons.
- Original v34 reference stays immutable. New actor and teacher adapters live in
  this standalone instrument and reuse its frozen Scheme/native authority.
- The old 72-deal GPU/CPU comparison may supply **calibration** observations.
  Those deals cannot become a new final holdout. All observations and mirrored
  arms of one physical source deal stay in one split.

## Evidence required for completion

The candidate must be a total lawful trick-1 player, with frozen compiled lower
rungs, complete-game execution, and CPU/GPU conformance for the same compiled
program. Model approximations are explicit; the field cannot change during a
bounded outer comparison. Every sampled witness retains its original columns,
weights and tapes. Small CPU jobs remain allowed in the hybrid.

Before final quality evaluation, freeze the player, configuration and independent
source-deal namespaces. The initial working confirmation plan is two independent
panels of 576 paired deals each (16 per nine declarations/four bidder cells),
bid 30, mirrored partnerships and balanced order. Require a positive paired
make/set difference in each panel and a pooled one-sided exact discordant-pair
binomial probability at most 0.01. Ties, all fallback games and any failures are
retained; source deals are the independent units. This is a bid-30 claim, not a
claim about every auction or opponent.

The working similar-time gate is candidate mean and p95 partnership wall time
no more than 1.10 times the current CPU anchor, under otherwise matched machine
conditions. Give CPU its ordinary configuration rather than weakening it to
manufacture a win. Report candidate budgets, actual charged time, cold loading,
tail latency and every overrun. Separate matched-budget diagnostic games from
this unweakened current-player comparison. The 100ms whole-game target remains
a separate goal, never replaced by a 100ms-per-move allowance.

Development panels can guide iteration. A final failure cannot be converted to
a pass by deleting deals or selecting a favorable prefix. Any revised final
protocol must be saved before its fresh outcomes are observed, and repeated
final attempts must disclose selection and use a predeclared error-control
schedule. No final panel is currently started or inspected.

## Current implementation slice

1. Both-team compatible-root T0 teaching with eight original Dice scenarios;
   full integer action vectors or explicit refusal.
2. The existing fourteen Scheme clauses as a scalar compiled actor, with strict
   differential conformance to Scheme's reference evaluator.
3. All 2,381 ordered programs of zero through three nonrepeated clauses, with
   exact local cost accounting and a signature-collision diagnostic.
4. One-minute resumable generation/teaching; source manifests and complete
   atomic per-query receipts precede model fitting.

## First completed calibration cycle

The corrected source adapter prepared 2,266 nonforced requests from the old 144
game receipts: 1,200 train and 1,066 development, grouped by 72 physical deals.
The CPU source format is flat **seat,tile pairs**, not flat tiles. Preparation
checks both request forms against the authoritative chronological move prefix.

T0 generation finished in 1.927 seconds, including the Python runner and worker
startup: 1,829 exact eight-world action vectors, 437 already settled positions,
zero refusals or errors. All original scenario columns and tapes are retained in
`results/compiled-calibration-v1/`; all data is calibration, not a final holdout.

The exhaustive 2,381-program fit selected C0 clauses `[5,8,2]`: partner-winning
count-5, master, then count-0, with least-legal fallback. Physical-deal-weighted
local teacher cost is 0.065939 train and 0.074248 development, versus 0.070432 and
0.088799 for the empty actor. This is local finite-bundle cost, not game regret
or a playing-strength result. `results/compiled-c0-v1/` retains the actor and
audit, including hashes of every lesson.

The grammar cannot emit any teacher-optimal tile at 190/940 train and 177/889
development positions. The per-row available-action loss floors are 0.041136
and 0.039217; enforcing identical outputs for identical fourteen-clause
signatures raises the optimistic floors to 0.048619 and 0.048770. These are
empirical representation diagnostics; teacher sampling noise remains present.

Scalar verification passed fourteen-clause Scheme action/provenance comparisons
over nine declarations, four initial leaders, and 28 plays; the actor field and
online player tests also passed. Teacher tests include independent full-horizon
defender parity, malformed history, illegal following, cancellation, and a lawful
endgame. Two failed teacher-test construction attempts are retained in
`results/compiled-validation-v1/`; their fixture errors were corrected before
the corpus was generated.

T1 generation against frozen C0 also completed: 1,829 exact two-world vectors,
437 settled positions, zero refusals, 2.294 seconds for the complete runner.
The selected C1 is `[12,1,8]` (boss-led, partner-winning legal, master). Local
cost is 0.060670 train and 0.093909 development, versus empty actor 0.066520 and
0.096724. These values describe the T1 target and must not be compared as play
quality against the different T0 cost scale.

The direct compiled GPU path passed 504 declaration/partial/clause batches with
full CPU/GPU trace parity, weighted duplicate scenarios, zero field queries,
one dispatch per complete batch, and cancellation/device recovery. The combined
36-test run passed, including legacy epoch and exact SUM/MAX contracts. See
`results/compiled-validation-v1/gpu-tests/`. A T1 test-build type annotation
failure is also retained; it did not affect the teacher executable, and the
completed T1 tests subsequently passed.

The online `compiled-player` uses frozen C0 for opponents, C1 for partner, a
compatible outer bundle, and the existing bounded SUM/MAX core. C1 provides a
legal unpriced reserve. The first baseline still uses one canonical priority
tail per root, forty worlds, full permitted focal horizon, and 5ms per move.

## Completed development panels

All six baseline panels used the same `compiled-v1-dev001` namespace, 72 paired deals,
144 complete games / 4,032 independently replayed plays each. All had zero
candidate/CPU fallbacks and zero failures. Current CPU kept its ordinary 14s
allowance and six threads. Each cycle had 55 seconds plus a 60-second outer
watchdog; per-move checkpoints preserved games at normal cycle boundaries.

| Candidate | Paired W/L/T | Candidate partnership mean / p95 | CPU mean / p95 |
| --- | --- | --- | --- |
| CPU, 5ms/move | 0 / 8 / 64 | 19.895 / 29.916 ms | 85.545 / 210.209 ms |
| GPU, 5ms/move | 2 / 8 / 62 | 21.640 / 31.938 ms | 85.750 / 213.583 ms |
| CPU, 20ms/move | 3 / 9 / 60 | 52.546 / 87.908 ms | 88.676 / 211.342 ms |
| GPU, 20ms/move | 5 / 8 / 59 | 54.808 / 90.551 ms | 88.801 / 216.146 ms |
| CPU, 40ms/move | 3 / 10 / 59 | 72.992 / 123.628 ms | 86.691 / 212.705 ms |
| GPU, 40ms/move | 3 / 10 / 59 | 74.839 / 126.517 ms | 86.486 / 211.227 ms |

These are repeat development comparisons, not independent confirmatory tests.
They show inexpensive play and insufficient playing strength, not goal success.
The 20ms GPU run had 193 interrupted decisions, 462 exact root vectors and 856
canonical root certifications. Its completed pricing covered 158,880 GPU lanes;
device time totaled 792.534ms. Certification concerns the frozen sampled model,
not accuracy of that model against current Walt.

The 40ms baseline panels each completed after three bounded cycles, with zero
failures or fallbacks. CPU40 had 1,023 reported decisions out of 2,016 candidate
moves, 941 optimal-root certifications and 100 interrupted solves. GPU40 had
1,024 reports, 943 certifications and 98 interruptions. Of its certifications,
46 chose value zero, 652 an intermediate value and 245 full mass. Increasing
search time alone did not produce a strength gain.

Next bounded controls: a learned C1 focal continuation and an allocation-free
compiled actor path, with backend/legacy compatibility tests. A separate
representation diagnostic will compare role-specific C0 fits and the coverage
of a public beat-current-winner selector on the existing calibration labels.
No role-specific C1 may reuse pooled-C0 teacher labels. Do not build,
edit pinned sources, or run competing benchmarks while a panel is in flight.
No final holdout has started. The goal is still active; no H2H improvement has
yet been established.

## Completed continuation control

The next implementation adds an optional total compiled focal tail. Explicit
full-history decisions take precedence; otherwise C1 chooses from the fixed
own hand after subtracting public plays. The actor never sees scenario hands.
Legacy priority policies and JSON remain supported, and malformed actors are
rejected rather than silently interpreted as a different continuation.

The same change provides a validated allocation-free action path for compiled
fields. It evaluates only the selected clauses, preserving the strict actor
and Scheme interfaces as independent references. Forty-six release contract
tests passed, including new tail and fast-path tests, weighted full CPU/GPU
traces, legacy epochs, cancellation and teacher checks. The no-default-feature
WASM library check passed. This is build/conformance evidence, not browser or
phone performance evidence. Receipts: `results/compiled-validation-v2/`.

All three 20ms/move controls completed in the same development namespace, with
40 worlds, 72 paired deals / 144 games / 4,032 plays each, zero failed arms,
zero fallbacks and matching pinned identities:

| Candidate | Paired W/L/T | Candidate mean / p95 | CPU mean / p95 |
| --- | --- | --- | --- |
| Fast CPU, legacy tail | 4 / 10 / 58 | 44.908 / 75.431 ms | 87.458 / 215.124 ms |
| Fast CPU, C1 tail | 4 / 6 / 62 | 46.166 / 79.850 ms | 88.879 / 216.590 ms |
| GPU, C1 tail | 4 / 6 / 62 | 49.873 / 83.976 ms | 88.881 / 217.664 ms |

The continuation helped on this reused development panel, but neither backend
beats Walt. GPU priced 161,480 complete lanes in 1,034 batches, with 871.564ms
device time and 1,025.292ms compute elapsed across the whole panel. These are
per-panel compute totals, not end-to-end per-game latency.

## Representation diagnostic

On the same 940 train / 889 development complete T0 labels, a role partition
selected declaring `[5,8,2]` and defending `[1,12,2]`. Both roles occur in all
36 physical groups of each split. Equal-role average development local cost
changed from 0.073913 (pooled actor) to 0.072404 (role actors). The train-selected
program was stable in 29/36 declaring and 34/36 defending leave-one-group-out
fits. These are correlated calibration observations, not stronger play.

A least-legal-tile-that-beats-current-winner selector supplies a previously
unavailable optimal action at 25/190 train and 24/177 development misses,
spread across 16 and 19 physical groups respectively. The empirical available
action loss floor falls from 0.041136 to 0.033900 train, and from 0.039217 to
0.033935 development. The independent referee defines this diagnostic; native
Scheme/scalar/WGSL conformance is required before runtime use.

Six diagnostic tests passed; the complete fit plus all 72 leave-one-group-out
fits took 10.883 seconds under the watchdog. Files and source hashes are in
`results/compiled-grammar-diagnostic-v2/`. This justifies a named sixteen-clause
representation control and role-specific training, followed by new C1 teachers
against its newly frozen C0. No old C1 labels may be relabeled under that field.

In parallel, a staged exact trace-union compute experiment is being prepared
from the earlier design: complete priority permutations cover physical traces,
then an original-column-preserving public-history trie performs lawful SUM/MAX.
This is a compute experiment, not a solution to compiled-model error. No new
final holdout has started, and the user goal remains active.

## Sixteen-clause role campaign prepared

Protocol: `docs/COMPILED-V3-PROTOCOL.md`. Runtime v2 grammar adds two public
current-winner selectors while preserving every v1 meaning and wire ID.
Role families support legacy single actors, strict JSON, normalized team
selection, full field identities and guarded Scheme export. Teacher features
and the frozen field are separately identified; a richer feature vocabulary
may legitimately learn against an older field grammar.

C0 v3 declares `[5,14,8]` and defends `[1,12,14]`; equal-role local costs are
0.055246 train and 0.063671 development. All 1,829 T0 vectors retain exactly
the original v1 counts, costs, ordered scenarios/tapes and first fourteen
features. The old bundle had different metadata spelling (`revision` named
Dice and omitted `field_revision`); the parity report records that distinction.

Fresh C1 teaching against the frozen role-specific C0 yielded another 1,829
complete vectors, 437 settled roots and zero refusals/errors. C1 declares
`[5,14,8]` and defends `[14,2,6]`, with equal-role costs 0.046034 train and
0.078947 development. This C1 target differs from pooled-C0 teaching; those
cost scales are not evidence of improvement against each other. T0 and T1
teaching completed in 1.899s and 1.778s respectively; both exhaustive role fits
completed in under one second. Inputs, actors and complete audits are retained
under `results/compiled-{calibration,c0,t1-calibration,c1}-v3/`.

Integrated validation passed 67 release contracts, the added teacher error-
schema test, two independent family Scheme/rotation contracts and the updated
WASM library check. The trace-union library passed the adaptive/fusion witnesses
and full-vector/weighted/CPU-GPU contracts. Its standalone benchmark is ready;
opening fixtures preserve a declaring opener and use a legal partial trick to
reach defending focal seats. Its measured run has not yet begun.

The next v3 CPU/GPU panels use the same 20ms configuration and development
namespace. No outcome for this representation is known yet. No fresh final
holdout has started; the user goal remains active.

## Completed v3 development panels

Both panels completed all 72 mirrored pairs / 144 games / 4,032 independently
replayed plays, with zero failed arms, zero fallbacks and stable pinned inputs.

| Candidate | Paired W/L/T | Candidate mean / p95 | CPU mean / p95 |
| --- | --- | --- | --- |
| Role-aware CPU20 | 1 / 10 / 61 | 44.126 / 79.023 ms | 87.564 / 209.621 ms |
| Role-aware GPU20 | 1 / 11 / 60 | 49.329 / 83.366 ms | 87.984 / 208.386 ms |

The lower local teaching cost did not transfer to better whole-game outcomes
on this reused development panel. GPU completed 1,059 batches / 161,800 lanes,
with 1,131.259ms device time and 1,272.097ms compute elapsed across the panel.
This remains faster but weaker play than the current CPU anchor; no promotion.

Next is the separately frozen trace-union compute control, followed by the
larger-bundle teaching control in `docs/COMPILED-V4-PROTOCOL.md`. The latter
keeps representation and online cost fixed. Compare old/new actors on the
same new label corpus, never raw cost scales across different finite targets.
No fresh final holdout has started; the user goal remains active.

## Exact trace-union compute result

The frozen 36-fixture n40 control completed in one bounded cycle, with full
vector, canonical-choice and independent-policy-repricing parity in every
case. There were zero timeouts, refusals or failures; end identities passed.
Five already-settled roots are separate from the 31 unsettled roots.

| Remaining tiles | Unsettled fixtures | CPU mean solve | GPU plus fold mean solve |
| --- | --- | --- | --- |
| 4 | 7 | 1.396 ms | 1.700 ms |
| 5 | 6 | 3.440 ms | 3.288 ms |
| 6 | 9 | 15.568 ms | 12.832 ms |
| 7 | 9 | 32.859 ms | 72.925 ms |

At seven tiles, the mean 150,400 lanes yielded 97,932 public trie nodes;
rollout/readback took 21.222ms and fold/extraction 51.379ms. GPU was faster
in 13/31 unsettled fixtures, none at depth seven. Separate cold initialization
was 13.770ms. These are exact fixed-problem compute comparisons, not new
whole-game quality or phone-performance evidence. Full receipts, aggregation
script and report: `results/trace-union-compute-v1/`.

## Larger-bundle teaching and v4 development result

The n32 T0 and n8 T1 runs each retained 1,829 complete vectors, 437 settled
roots and zero refusals/errors. C0 v4 declares `[5,14,8]` and defends
`[5,7,14]`; C1 v4 declares `[5,14,8]` and defends `[5,14,2]`. The declaring
actors are unchanged from v3. Old and new raw cost scales are different
finite targets and must not be compared as improvements.

V4 CPU20 completed 72 pairs / 144 games / 4,032 replayed plays, zero failed
arms and zero fallbacks: **4 wins / 7 losses / 61 ties**. Candidate partnership
mean/p95 was 40.519/74.561ms; current Walt was 85.982/215.220ms. This recovered
some of v3's loss on reused development deals, but still fails the quality
goal. No GPU v4 panel or fresh final holdout has begun.

The separate same-label cross-scoring script initially reported unnormalized
integer count loss as cost; those diagnostic outputs are retained as invalid.
The corrected report divides by original mass, uses exact rational group
weights, and matches every authoritative role/split fit cost exactly.
On the same new development labels, equal-role costs changed from 0.044703
to 0.044313 for C0 and 0.055238 to 0.054783 for C1 (old v3 to new v4 actors).
This did not affect teaching, the authoritative fits, actor artifacts, or H2H.
Next controls address public trick order, outer sample count (separately frozen
protocol), and the distinction between compatible teachers and historical
native lower minds. The user goal remains active.

The role/order diagnostic completed in 4.087s, with exact baseline audit
agreement and independent public replay. Leading/following partitions reduced
T0 local development cost from 0.044313 to 0.040420, and T1 from 0.054783 to
0.052520. Four separate trick offsets reduced T0 but worsened T1 to 0.058097.
Thus the smaller leading/following extension is the next representation
candidate. T0 leading actors are declaring `[8,2]`, defending `[8]`; following
actors are declaring `[5,14,2]`, defending `[5,7,14]`. A new C1 teacher must
respond to that newly frozen C0 before any whole-game claim.

The historical-H0 diagnostic modes are implemented separately: fixed native
eight-world Voidless H0, and compatible worlds with the identical H0 seed.
Twelve teacher contracts and five fitter contracts passed, including both-team
native-choice parity, no-void ordered-world/tape/vector parity for partial
hands, and explicit target identity. Two test-construction failures (missing
Key Debug formatting and missing mocked-row metadata) are retained with their
fixes. No historical-H0 lesson campaign has run. This is diagnostic machinery,
not evidence that the historical target is better. Source changes are confined
to the teacher/fitter; the immutable reference remains unchanged.

## Outer80 control and lead/follow runtime

Outer80 CPU20 on frozen v4 actors completed all 72 mirrored pairs: 3 wins,
9 losses, 60 ties; 144 games / 4,032 replayed plays, no failed arms or fallbacks.
Candidate partnership mean/p95 was 60.80/96.55ms; current Walt 88.34/214.42ms.
Of 1,068 bounded reports, 875 certified the optimal root, 862 certified the
canonical root, 441 completed exact vectors and 235 were interrupted. More
outer samples did not improve these development outcomes. Retain outer40 for
the separately specified v5 representation comparison.

The v5 lead/follow actor is implemented for strict scalar, fast fields, Scheme
export, focal tails and WGSL. Legacy actor JSON omits the new optional field;
new schema validation requires both bounded programs. Scheme guards use the
on-turn viewer's equality/inequality with the public leader. GPU packets carry
two fixed-width clause records per actor and select using current trick length.
The fitter selects phase programs with parent-role physical-group weights,
while keeping actor schema distinct from the sixteen-feature label vocabulary.

Validation passed 67 release contracts and seven fitter tests, plus the WASM
library check. New contracts distinguish the two phases across all declarations
and partial tricks, compose role guards, preserve explicit focal overrides,
reject malformed programs and compare weighted full GPU traces with scalar
replay. This is conformance/build evidence, not a phone benchmark. Protocol is
`docs/COMPILED-V5-PROTOCOL.md`; no v5 game outcome is known yet.


## V5 development result and nomination

Lead/follow C0-v5 reproduced the frozen role/order diagnostic. T1-v5 completed
all 2,266 requests (1,829 exact vectors and 437 settled roots), with no pending
requests or refusals. C1-v5 declares with lead `[8]`, follow `[5,14]`, and
defends with lead `[8,2,6]`, follow `[5,14,2]`.

The CPU20 candidate completed 72 mirrored pairs / 144 games / 4,032 replayed
plays: **8 wins, 5 losses, 59 ties**, mean paired difference +0.041667.
Candidate partnership mean/p95 was 40.1669/70.5466ms, versus current Walt
86.5495/207.7895ms. There were zero failed arms and zero fallbacks. Of 1,072
bounded reports, 998 certified the optimal root, 985 the canonical root and
109 were interrupted; 195 further reports were contract-settled.

This is the first positive result on the repeatedly reused development panel;
it is candidate nomination, not confirmation of stronger play. Freeze these
C0/C1 actors, outer40/plans1/horizon7/work2,000,000, compiled focal tail and
20ms move cap for two independent fresh panels. The unweakened current CPU
keeps its ordinary 14,000ms per-move allowance and six threads. Compare actual
complete-partnership latency. No fresh outcome has been observed at nomination.
The historical-H0 diagnostic and further ladder teaching are deferred while
this nominated candidate is tested. The user goal remains active.


## Paused: fresh v5 confirmation did not transfer

The user paused the goal for usage-budget reasons on 2026-09-21. Both frozen
replacement panels had completed: C 63/83/430 and D 44/74/458 (wins/losses/ties).
Combined 107/157/888 across 1,152 pairs, with candidate mean partnership time
42.61ms versus current Walt 90.60ms. Both panels passed harness replay with
zero failed arms/fallbacks and clean cycle identities. The candidate is faster
but weaker; the positive development result did not generalize. No promotion.
The additional independent final audit is staged, not executed. Complete
state, retained gains, setup failure and restart boundaries are in
[PAUSED-HANDOFF.md](PAUSED-HANDOFF.md). No further run is authorized while paused.
