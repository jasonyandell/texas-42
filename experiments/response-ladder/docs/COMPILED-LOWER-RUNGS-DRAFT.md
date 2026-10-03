# Compile the lower rungs using the existing Scheme and gym infrastructure

**Exploratory design draft, 2026-09-20.** This is a proposed next experiment,
not a new player, learned policy, theorem, or performance result. It follows
the [native GPU comparison](../results/gpu-current-h2h-v1/REPORT.md) and Jason's
direction to use the gym, Scheme and the accumulated mathematics.

## The question and the reason it survives

Can one small, shared, lawful program replace repeated lower-rung searches
cheaply enough that a trick-1 response player gets better outcomes within the
current player's time scale, with 100ms for a complete 28-play game as the
separate speed target?

The previous hybrid made 6.81 million modeled-query submissions and 70,369
device epochs across its 144 games. It was 5.19 times slower in mean partnership
time than the then-current CPU wrapper. Its lower minds still ran on CPU.
The proposed change removes recursive lower-mind construction from the online
path by replacing those minds with frozen executable programs.

The repository already has the required representation, a bounded shared
learner, exact finite teachers, on-policy lesson aggregation, complete-policy
evaluation, and reusable gym recipes. The next task is to connect and extend
these components for opening-through-endgame lower-rung compilation. A new
policy language or a new standalone teaching framework is unnecessary.

## Infrastructure to reuse, and the actual gaps

Paths below are relative to the repository root.

| Existing component | Reuse | Gap to address explicitly |
|---|---|---|
| `walt/scheme/POLICIES.md`; `walt/walt/src/scheme/policy.rs` | `PolicyProgram`, parse/display identity, Viewer-only guards, `PolicyInput`, legal fallback, provenance; modes/bindings already represented | No WGSL execution backend. Full reference input validation is not a cheap per-lane device adapter. |
| `walt/scheme/RELATIONAL.md`; `walt/walt/src/policy_search/relational.rs` | Cost-sensitive shared-program construction, candidate beam, clause/AST/work caps, exact rational lesson costs | v1 has 14 clauses, one mode, no learned bindings, and no guard access to score, bid or full history despite retaining them in input/cache identity. |
| `walt/walt/src/policy_search/learning_io.rs` | Inspectable own/public lesson rows with all legal-action costs and weights | Existing request/lesson path fixes bid 30; no silent claim of arbitrary-contract support. |
| `walt/scheme/INFORMATION-PRICES.md`; `policy_search/prices.rs`, `learning_eval.rs` | Full-history finite oracle, actual-policy replay, exact local-regret identity under one root prior/field, empirical-versus-census authority | Existing `learning_eval::run` constructs a capped exact root fiber, supports only named gym/L0 fields, and is not an opening-scale teacher. Its underlying `policy_search::Search` restricts the viewer to the declaring team. Dice scenario/tape and defending-side teaching need the new adapter below. |
| `experiments/partnership/relational_campaign.py` | Grouped train/discovery/development/test splits, on-policy aggregation, whole-policy selection, frozen digests, resumable jobs | Existing campaign is a three-tile/sixes/S0 study. It does not teach opening lower-rung actors. |
| `walt/gym/SPECIFICATIONS.md`; `gym_generation.py`, `gym_deployed.py` | Scheme-defined exercises, exact or sampled coverage, declared focal/partner/opponent continuations, value reuse and witnesses | Current deployed evaluator accepts named native presets. A compiled-program continuation adapter is new work; old keys cannot be relabeled under a new field. |
| `experiments/partnership/POLICY-SYNTHESIS.md`; `walt/scheme/COMPOSITION.md` | Persistent exact search and serializable policy extraction for teacher work | Root-specific exact tables are not a shared policy across hands; complete donor composition has not repaid its setup cost. |
| `experiments/kiln/WHOLE-GAME-CONTRASTS-V1.md` | Own-hand sizes 7 through 2, same-world alternatives, counterexamples, fresh transfer and retained sample additions | Historical hidden-holding predicates are examiner evidence, not deployable actor inputs; observed action gaps use the declared continuation. |
| `experiments/response-ladder/src/{core,policy,gpu_epochs}.rs` | Lawful total policies, integer sampled bounds, CPU reference evaluation, persistent GPU device, transactional cancellation | Replace field callbacks with compiled device policies; retain the same planner in CPU/GPU timing controls. |

The working checkout was clean at `9d4dd577` when this map was made. The
partnership checkout has advanced to `d24eaefe` since the old H2H. Pin fresh
source, binary and policy identities for the new study; the old receipts remain
evidence about their recorded revisions. Do not modify the immutable v34
reference archive or treat its snapshot as the current production default.

## What the earlier experiments already tell us

The [policy-synthesis report](../../partnership/campaigns/policy-synthesis-v1/RESULTS.md)
found that persistence reduced complete paired schedule search time by 52.9%
in its cheap field and 46.4% under L0-8 while preserving completed policies.
Those are construction costs on that campaign, not a new speedup of the current
CPU player. The exported policies still fit their small training samples heavily;
their unseen-history fallback mattered. Repeating tables across hands did not
produce one relational policy.

The [shared relational study](../../partnership/campaigns/relational-learning-v1/RESULTS.md)
already tested the broad idea of a small shared actor. With fourteen clauses,
at most three selected clauses, and three tiles remaining, its actors trailed
sampled tables on held-out roots. L0-8 development selected the empty baseline.
A fallback substitution had an uncertain positive signal in the gym field and
weak transfer to the maintained exam. These outcomes remain controls and
counterexamples. The next study must test new expressive capacity and the
actual lower-rung deployment role rather than rerun the same restricted actor.

The same study found that its tested information prices tightened no bounds.
With fixed continuation lowers, tightening the common `max U` term cannot
rerank action costs `max U - L(a)`. Prices remain potential tools for allocating
teacher effort or proving bounds; adding them alone is not a stronger learner.

Gym keys are continuation-relative. In the deployed-L1 replay of six earlier
misses, two offers remained useful and four reversed. A compiled field must
therefore receive newly evaluated keys as well as comparisons to the old ones.
The maintained gym is a diagnostic, not an untouched generalization holdout.

## The ladder and what stays exact

Let `D` be the declared historical Dice action/tape rule. Let `BR_B(F)` mean
the exact lawful best response on one frozen ordered scenario/tape bundle B
against F, maximizing the focal team's make/set success. Every focal choice is
shared across its full observed-history bucket. The first lesson campaign is
explicitly bid 30; general-contract lessons require fixing/versioning the
hard-coded bid in `learning_io`, even though `request.rs` accepts bids 30–42.

**New target: compatible-root-reset-v1.** Both B0 and B1 are sampled from the
full mechanically compatible fiber of the supplied own hand and complete
public history, using the existing exact mechanical sampler. They retain
original ordered sample positions and Dice tapes. Root-reset sampling is the
declared lower-policy construction, not a behavioral posterior inferred from
the historical players. Each new query root has its own immutable sampled
target; within that solve, carry and condition its original indices.

This intentionally changes the historical Voidless inner-sampling target.
Voidless lower minds can query counterfactual hands incompatible with earlier
voids or own follow history; strict `PolicyInput::new` correctly rejects them.
Do not weaken that validator, invent a predecessor, or label an invalid request
with a fallback action. Historical relaxed requests are a separately counted
domain diagnostic and belong only to historical controls. All new compiled
queries must pass the strict actor-domain contract. Test that every simulated
legal continuation from each compatible root preserves that domain.

Build and freeze the hierarchy in order:

1. `T0(I) = BR_{B0(I)}(D)`, with eight compatible samples, the declared Dice
   tape rule and canonical ties. Learn a shared total program `C0` from those
   action-cost lessons and freeze it. Historical Voidless L0-8 remains a separate
   control, not an alias for this teacher.
2. `T1(I) = BR_{B1(I)}(all-C0)`. Generate new lessons against that frozen C0,
   learn `C1`, then freeze both artifacts together. Use the declared n1=2 pilot
   teacher count corresponding to Partner 40/8/2; larger teacher bundles
   are separately named targets, not improvements to the same exact object.
3. The online focal player answers a frozen Partner field: opponents use C0,
   partner uses C1. The main 40-scenario bundle, its bounds and policy witnesses
   retain the response-ladder contract.

Imitating historical L1 against historical L0 is a useful separate distillation
control; it does not instantiate step 2 after C0 has changed. Never mix the two
teacher identities under one rung name.

Program evaluation must be byte-identical between its scalar and WGSL backends.
Old-Walt byte compatibility is a different property: approximate C0/C1 do not
inherit it. The exact outer response is exact only against the new frozen field
and bundle when its bounds close. A nonzero outer gap still means an anytime
approximation. Improving a compiled rung invalidates values and bounds from
the previous field revision.

## Teach consequential decisions, with lawful inputs

Use the existing rational action-cost format, not one-hot teacher imitation:

`cost(I,a) = max_b Q_{B(I),F}(b) - Q_{B(I),F}(a)`.

Keep the whole optimal set, canonical choice and complete value vector as
separate diagnostics. A disagreement between tied actions is not decision
regret, although it is a byte-compatibility disagreement. Costs are labels;
they cannot appear as actor predicates.

Collect training observations from actual lower-mind calls in ordinary play,
from the candidate's newly reached states, and from a separately reported
balanced position panel covering hand sizes 7 through 2, all declarations,
both teams and all trick offsets. Original physical deals and all their
rotations, interventions, labels and descendants stay in one split. Separate
query-frequency-weighted diagnostics from equal-deal policy outcomes so many
easy repeated requests do not conceal important failures.

Classify historical captured queries by the strict actor domain before label
generation. Publish both admitted and incompatible counts and their sampling
provenance; the new candidate's on-policy distribution uses only compatible
roots. Capture ordinary real-player observations separately from counterfactual
modeled observations rather than treating their distributions as identical.

For scale, adapt the response-ladder core's existing both-team finite-bundle
solver to produce completed per-query lessons under Dice or a compiled field.
It already uses original scenario/tape columns and full-history decisions;
the new bridge supplies strict Scheme inputs and the existing lesson format.
The old declaring-only `policy_search::Search` is not silently extended to
defenders. At terminal nodes use `u=made` for the declaring side and `u=1-made`
for the defending side, then maximize the focal team's utility at every focal
choice. Complementing an already maximized declaring value is not equivalent.
Differentially verify seat rotation, all-action costs and both utilities against
the frozen solver on identical supplied bundles, independently of prior choice.
Preserve ordered allocations, original Dice tapes, sample
multiplicity, weights, model version, root and every sample extension. Incomplete
teacher calls are explicit missing/censored labels, never zero regret. Exact
root-fiber teachers remain the bounded gym/endgame audit path.

Fresh per-query bundles define local distillation objectives. Summing these
costs along a game does **not** automatically produce the existing exact
performance-difference identity: that identity requires one fixed root prior,
field and coherent posterior through the trajectory. Use that retained-tree
exam separately wherever feasible. Whole-policy rollouts are the final measure
when local teacher contexts differ.

The first representation comparison is the existing v1 grammar against a
versioned extension exposing lawful score/remaining target, declaring role,
position within the trick, own-hand structure, current winner and public void
information. New predicates need explicit domains, undefined-value behavior,
reference semantics and bounded device implementations. Some need a richer
Viewer evaluation context, not merely a new clause name. No exact world count,
hidden possession, teacher answer, source ID or observed outcome is available
to the actor. Any later belief-summary feature must be separately specified
and its online computation charged.

Before expanding the grammar, audit the limits of the existing one. With at
most three nonrepeated clauses from fourteen, there are only
`1 + 14 + 14*13 + 14*13*12 = 2,381` ordered programs. Repeating a clause in
this stateless first-match language cannot change its choice. On a fixed,
bounded lesson panel, enumerate that class to separate beam-search failure
from the best attainable cost of the declared three-clause grammar.

Also group observations by the fourteen clause outputs (tile or no-match) plus
the fallback action. Identical signatures force every v1 program to choose
the same physical tile. For each group G, the minimum over its available
signature actions of `sum_{I in G} weight(I)*cost(I,a)` is an optimistic lower
bound on that class's empirical loss. This relaxed choice per group can do
better than one globally ordered program, so a zero bound is inconclusive.
It is not a population or whole-game regret certificate. Repeated observations
with different finite-bundle labels must additionally be diagnosed as possible
sampling/target instability, not automatically missing actor features.

Start with stateless shared programs. Modes and rigid bindings are available,
but a lower-mind query cannot initialize fictitious memory at an arbitrary
counterfactual node. A stateful extension must carry/reconstruct the controller
from its declared lawful initialization and include it in modeled identity.

## The first implementation slice

Deliver a scalar compiled C0 experiment before the two-rung GPU player:

1. Add a strict adapter from completed compatible-root T0 solves to the existing
   lesson format, including teacher/bundle authority sidecars. Check both-team
   all-action costs against the frozen solver supplied the exact same bundle;
   record historical Voidless teaching separately as a changed-prior control.
2. Audit all 2,381 v1 programs and their observation-signature collisions on a
   bounded development panel. Run existing v1 grammar as the inherited negative
   control and the declared
   richer grammar as the candidate, on opening-through-endgame source groups.
   Reuse the existing aggregation, serialization, development selection and
   freeze/resume machinery. Freeze roots, counts, resource caps and decision
   criteria in the campaign manifest before generating lessons.
3. Evaluate the actual saved program as a lower-rung field, including fresh
   states induced by its own substitutions. Measure its standalone full-policy
   behavior and downstream response behavior separately. Report useful default
   behavior, harmful counterexamples, disagreement regret and runtime tails.
4. Use the gym as a separate causal diagnostic: same public root, same original
   hidden worlds, changed compiled/native field, all legal alternatives. Keep
   root-choice regret separate from the saved continuation's whole-policy gap.

This slice tests whether a useful compact lower rung exists under the chosen
grammar. A tiny program with high accuracy on ties is not a success. If v2
still loses consequential decisions, report whether the teacher costs, actor
expressiveness or state distribution explain the failure before adding GPU
plumbing or more model levels.

## Device integration and the decisive controls

Compile the accepted stateless `PolicyProgram` subset to a bounded integer
instruction buffer and WGSL evaluator. Preserve ordered rule selection, least
legal returned action, undefined-predicate behavior and the total legal fallback.
Reject unsupported programs at load time. Keep reference `PolicyInput`
validation at the external boundary; establish that internal lanes preserve
its physical invariants. Do not perform full support enumeration or a hidden
CPU policy call for every device choice.

The original scalar Scheme evaluator is the compiler oracle. Check complete
action/provenance traces across declarations, offsets, seats, rule hits/misses,
unseen hands and malformed programs. The compiled field must stay fixed for a
whole comparison. Then a GPU rollout can proceed through modeled moves without
the old per-field-decision CPU round trip.

Name the actual rung graphs in the controls. "Native versus compiled" is not
sufficient after changing the prior and then the lower policy:

| Graph | Opponents | Partner | What the adjacent comparison changes |
|---|---|---|---|
| H: historical | historical Voidless H0 | historical H1 responding to H0 | Recorded old semantics; current production wrapper is the end-to-end anchor. |
| M: compatible native | compatible T0 | compatible `BR(all-T0)` | H to M changes lower-rung sampling with native response construction retained. |
| C: compiled base | frozen C0 | compatible `T1=BR(all-C0)` | M to C substitutes C0 and rebuilds the partner teacher against that declared C0. |
| CC: compiled ladder | same frozen C0 | C1 taught against that T1 | C to CC isolates partner-rung compilation with the opponent program and teacher target fixed. |

For local C0 teaching, report historical H0, compatible T0 and compiled C0
separately. For C1, compare C1 directly with its own T1 while holding C0 fixed;
historical H1 is not that teacher. These are declared controls, not a requirement
to run an expensive all-graph tournament before the first C0 result.

Separate two timing questions. First, feed scalar and GPU pricing the identical
frozen field graph, input policies and ordered scenario/tape bundle, and require
complete identical payoff matrices. Use sufficient fixed work allowances, report
every refusal, and time only comparable completed jobs while publishing their
coverage. Include packing, validation, grouping and reduction, with cold setup
reported separately. This is the backend comparison.

Then run the same anytime planner under matched wall budgets on each backend.
It can complete different work and choose different policies; record requested
and completed work, witness values, field calls and whole-decision wall. Those
outcomes belong to the online-player comparison, not the fixed-matrix speedup.
The previous production-CPU/H2H did not isolate these factors.

Only after selecting/fixing the candidate on development data, run fresh
mirrored whole-game comparisons against a newly pinned current CPU build.
Give both partnerships matched cumulative budgets calibrated on a separate
current-CPU timing panel; also retain its ordinary allowance as a secondary
anchor. The separate 100ms target concerns all 28 plays of a game, not each
move. Charge policy setup, state updates, fallback, IPC and cleanup to measured
online wall; report cold loading separately and report budget overruns. A sum
of nominal allowances alone does not demonstrate a 100ms game.

Final evidence is paired make/set outcomes by independent source deal, latency
distribution and deadline behavior, with exact-domain gym diagnostics alongside.
Freeze sample size, noninferiority/improvement criteria, latency gate and analysis
before the quality run; retain harms, ties, failures and all fallback games.
No candidate selection uses that final panel. First-wave numbers, learned
programs and GPU performance claims remain absent from this draft.

## Independent design review

A read-only agent checked the source map, prior results, rung ordering and cost
semantics. The draft incorporates its required corrections: compatible modeled
input domains, both-team teacher adaptation, the bid-30 lesson boundary, explicit
rung graphs, and separate completed-matrix versus anytime comparisons. It also
retains the proposed signature-collision audit as an empirical relaxation, with
sampling instability distinguished from missing actor information. No experiment
or compiler result is implied by this review.
