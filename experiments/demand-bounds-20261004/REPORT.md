# Exact demand-limited shared choice

2026-10-04. Exploratory component evidence. Parent checkpoint
`7f4c68a01a5d3caae5345695750cdd5b4ee8e736` on
`codex/walt-prefix-state-20261004`; new isolated branch
`codex/walt-demand-bounds-20261004`. [Reproduction](README.md),
[independent numbered review](checks/REVIEW.md), [audit](AUDIT.json).

Exact integer bounds reduce lower-policy demand and complete the six-root
field2/40-world batch under the original one-worker query allowance. They do
**not** produce a useful cold policy speed win against equal-worker native
recursion. Allocation and repeated lazy traversal replace the work saved by
pruning. The prospective larger batch still refuses at field2/40 worlds.
Cross-position cache reuse is measurable but small on the tested sequence.
No production adapter or playing-strength evidence is supplied.

## Numbered claims

1. **Accepted — checkpoint, identity and originals independently checked.**
   The collaborator built and reran all nine preceding demand-matrix cells:
   102 completed vectors match pinned native; field2/40 refuses with 49,534
   generated /49,202 completed actor queries and no root vector. Eight-world
   eager totals reproduce 446 →4,075 →19,630 for fields0/1/2. Its arithmetic
   audit recomputes 9,336 retained checkpoint vector comparisons and all current
   median/cost claims. [Checkpoint receipts](checks/prefix-rerun/run.json).

   The fresh [identity receipt](results/identity/stdout.log) matches Plunge
   `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust source
   `cb1ef3b23072e4c268f31f625f2b61d5facc1929`, 202 source inputs and phone WASM
   SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   All 927 entries in the three prior historical manifests remain exact.
   Original sampler, validation and eager full-fold blocks are byte-identical.
   Changes are confined to this experiment. Git administration and checkout are
   a local clone in task-5; task-4 originals remain untouched. The requested
   fresh `gpt-6.1-sol` collaborator was accepted by the agent tool; separate
   runtime model attestation is unavailable.

2. **Accepted with premises — conservative integer bounds preserve ascending
   shared-choice semantics.** [The lazy evaluator](native/src/bounds.rs) creates
   one continuation per public prefix with the complete ordered world-index
   group. Every focal action receives that whole group; it is never optimized
   independently per hidden world. A nature node partitions by observed tile,
   only after all demanded lower actor choices for that occupied prefix are
   known. Each partition then has its own public continuation. Binary terminal
   payoff supplies bounds `[0, group length]`; settled nodes have exact totals.
   Nature sums disjoint-group bounds. Focal MAX/MIN applies after aggregation.

   For maximizing candidate a, certification requires L[a]>U[b] for every
   earlier b and L[a]>=U[b] for every later b. Minimization requires U[a]<L[b]
   earlier and U[a]<=L[b] later. Refinement schedules the most promising
   unresolved candidate; the certificate alone determines the returned choice.
   Top-level root requests still emit every action's complete integer vector.
   Only lower consumers use the separate exact choice API and choice cache.
   The controlled `eagerzero` variant retains full batched L0 vectors, because
   L0 has no lower actor queries to save; higher choices remain bounded.

   [DemandBounds.lean](math/DemandBounds.lean) proves seven **conditional** lemmas:
   MAX/MIN ascending-choice soundness, list-sum/unit-mass bounds, pair MAX/MIN
   bounds and equality for an arbitrary equal supplied actor view. The
   [owner](results/lean-2/run.json) and [independent](checks/lean-independent-fixed/run.json)
   kernel runs pass with no `sorry`/`admit`/new axiom and only standard axioms
   where used. Bounds must already cover lawful exact shared-group totals.
   These lemmas do not verify the Rust tree, sampler, arithmetic overflow,
   caches/refusals, privacy or complexity.

3. **Accepted finite checks — all demanded choices on a representative trace
   match full and native evaluation.** Six native tests pass. Independently,
   22,220 interval boxes with one through four actions are checked for both
   soundness and completeness of certainty against all covered integer values,
   with empty/invalid/extreme-u64 cases. Each of both candidate modes passes
   38 scalar roots, 76 paired-seed reversal/tripling vectors, 48 direct actor
   comparisons over rungs0–2 and both beliefs, 12 full policy root comparisons,
   48 policy transformations, 37 actual ties, 96 exact cache hits, 384 key
   distinctions and seven valid changed-key miss-then-hit tests.

   On the six original field2/eight-world roots, **every one of 11,936 demanded
   actor calls per mode**, including hits, matches independently requested full
   eager vectors and native choices. There are 11,666 distinct specs with
   rung counts `[8559,2637,470]`. Trace length equals the summed actor-demand
   counter, and all six root vectors match native. Trace is disabled in all
   timing processes. The separately charged validation Services allow 100k
   queries/100m row work; timing retains 50k/20m per worker. Original sampling
   budgets `[4,2,2,2]` remain fixed in both. [Final parity/trace receipt](checks/candidate-parity-final/run.json).

4. **Accepted finite demand reduction — pruning avoids actual actor sampling,
   with original budgets and complete keys.** Common roots are deals
   962004/962012/962015/962016/962020/962023, plies16–19, from 32 proposed deals.
   The following counts are completed unique actor queries, not a minimum for
   all possible algorithms. Pure and `eagerzero` have identical actor demand.

   | Outer worlds | Modeled field | Full eager misses | Bounded misses | Bounded L0/L1/L2 |
   | ---: | ---: | ---: | ---: | --- |
   | 2 | 0 | 117 | 117 | 117 |
   | 2 | 1 | 1,051 | 784 | 662 /122 |
   | 2 | 2 | 5,352 | 2,911 | 2,080 /707 /124 |
   | 8 | 0 | 446 | 446 | 446 |
   | 8 | 1 | 4,075 | 3,091 | 2,623 /468 |
   | 8 | 2 | 19,630 | 11,666 | 8,559 /2,637 /470 |
   | 40 | 0 | 1,738 | 1,738 | 1,738 |
   | 40 | 1 | 15,113 | 11,288 | 9,541 /1,747 |
   | 40 | 2 | 67,878 (four workers) | 39,847 | 28,689 /9,376 /1,782 |

   At eight worlds/field2, misses fall 40.57% and actual inner worlds fall
   70,164→40,450. At forty worlds/four workers, misses fall 41.30% and actual
   inner worlds fall 241,226→137,072. With **one** 50k allowance, full eager
   refuses at 49,534 generated/49,202 completed; bounded completes 39,847.
   The partial refused count is not used to infer hypothetical completed demand
   or a speed ratio. Four workers provide four separate allowances, not one
   shared 50k cap. [One-worker matrix](results/final-one/summary.json),
   [four-worker matrix](results/final-four/summary.json).

5. **Qualified — runtime remains the principal obstacle.** Five cold processes
   per cell rotate all five variants through every position. All native and
   candidate worker counts match. These are local medians, not population
   intervals. Service timing includes Service/partition setup, preparation,
   grouping, sampling, expansion, folding and complete-cache insertion. Native
   wall includes preparation/scheduling. Refused intervals are excluded below.

   | Panel | Workers | Worlds/field | Pure bound | Full eager | L0-batched bound | Native-choice hybrid | Native recursion |
   | --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |
   | Six roots | 1 | 2 /2 | 11.616 ms | 13.830 ms | 13.322 ms | 1.221 ms | 1.030 ms |
   | Six roots | 1 | 8 /2 | 48.034 ms | 46.754 ms | 54.477 ms | 4.016 ms | 3.707 ms |
   | Six roots | 1 | 40 /2 | 159.929 ms | refused | 186.860 ms | 13.875 ms | 13.280 ms |
   | Six roots | 4 | 8 /2 | 27.500 ms | 22.829 ms | 44.245 ms | 1.886 ms | 1.539 ms |
   | Six roots | 4 | 40 /2 | 88.480 ms | 67.265 ms | 159.720 ms | 6.166 ms | 5.768 ms |
   | Fresh 31 roots | 4 | 8 /2 | 107.200 ms | 73.193 ms | 189.663 ms | 6.995 ms | 5.547 ms |

   Pure beats eager only in the small one-worker two-world field2 cell; it
   remains about11.3x slower than native there. No policy speed win against
   native follows. The L0 batching ablation does not repair the higher-rung
   runtime gap despite identical demand.

   The six-root four-worker forty-world bound run creates 743,656 lazy nodes,
   expands 541,034, and makes 1,785,943 refinement visits. It charges 1,027,249
   row edges versus eager 3,250,490, yet uses236.561 ms child CPU versus177.574
   ms eager. Those schedule-dependent row counts include different terminal
   padding and do not imply a scalar node minimum. Repeated node scans and
   allocations are concrete implementation costs; phase intervals do not
   isolate their causal contribution because recursive scopes overlap.

6. **Qualified — full workload and total memory are measured.** For that
   six-root four-worker forty-world cell, pure/eager/native standalone totals
   (JSON+process+replay+tempfile+parse) are92.627/73.678/9.133 ms. Actual-n
   generation over all32 proposed deals takes42.181 ms; totals become
   134.808/115.859/51.314 ms. PID-specific RSS medians are
   17,350,656/54,247,424/7,258,112 bytes. Pure has lower measured RSS than eager
   on this panel, while native is smaller and faster. It is not a uniform
   memory theorem. Full raw CPU, RSS, serialization and process resources stay
   in each compressed record.

   `peak_bounded_live_bytes` counts a partial node/world allocation estimate,
   excluding child-vector spare capacity, temporary arrays/maps/specs, sampled
   Query/cache ownership and allocator overhead. Summing worker maxima is not
   simultaneous residency. Lazy retained-row caps and cumulative row work are
   separate conservative refusals; the eager parent arrays coexist with lazy
   subtrees. No float enters ranks or decision probabilities; floats in the
   Python harness describe resource timings only.

7. **Qualified — a prospective larger panel still refuses.** Deals967000–967127
   yield31 eligible pip-trump roots at plies16–19, with identical selected seeds
   across8/40 worlds. At eight worlds/field2, full misses101,707, bounded59,603
   (41.40% fewer), with actual inner worlds364,732→206,806. At forty worlds,
   bounded, L0-batched and full all refuse per-worker50k allowances; no root
   vector is emitted. Their aggregate **partial** generated counts are
   198,224/198,224/197,987. Hybrid/native complete in25.588/21.331 ms.
   Complete lower caches survive but no refused count is promoted to completed
   demand. All4,145 final completed-vector comparisons and20 refused variant
   observations are independently audited. [Holdout](results/final-holdout/summary.json),
   [final arithmetic audit](checks/new-timing-audit-final/run.json).

8. **Qualified useful reuse — distinct successive positions share little cache
   work on the tested sequence.** A prospective local panel from seeds968000–968015,
   plies16/17/18/19, yields15 distinct eligible public positions out of64
   proposals at8worlds. Each belief/field sequence uses one retained Service,
   compared with an independent cold Service and pinned native for every root:
   90 full-vector comparisons pass. No identical root Query is repeated.
   Cold-miss minus retained-Service-miss measures avoided actor queries,
   separately from ordinary within-request cache hits.

   | Belief | Field | Independent cold misses | Retained misses | Queries avoided | Inner worlds avoided |
   | --- | ---: | ---: | ---: | ---: | ---: |
   | Voidless | 0 | 1,225 | 1,219 | 6 | 24 |
   | Voidless | 1 | 7,135 | 7,112 | 23 | 88 |
   | Voidless | 2 | 25,169 | 25,004 | 165 (0.66%) | 574 |
   | VoidsCounted | 0 | 1,240 | 1,237 | 3 | 12 |
   | VoidsCounted | 1 | 7,382 | 7,338 | 44 | 172 |
   | VoidsCounted | 2 | 24,520 | 24,061 | 459 (1.87%) | 1,724 |

   This is one finite sequence from the same deals, not an online production
   hit-rate estimate. Boundaries/context must remain in the key even when they
   inhibit reuse. Raw timings interleave cold/reused/native work and are
   illustrative only. [Generation and serialization](results/reuse-final/generation.json),
   [complete profile and CPU/RSS](results/reuse-final/result.json),
   [bounded receipt](results/reuse-final-run/run.json).

9. **Qualified lawful scope — assumptions and caches remain explicit.** Original
   ordered scenario multiplicity, deterministic native Dice seeds, historical
   tapes, inner-shuffle consumption, declared budgets and beliefs are retained.
   A lower ActorSpec contains Context, public State, actor/hand and rung; no
   opposing parent hand/world index enters its sampler. Same-view sampling and
   transformed queries are checked, not elevated to formal privacy proof.
   Complete cache keys include declaration/bid/all budgets/boundary/belief/full
   State/actor/hand/rung; root Query keys also include ordered worlds and nature.
   Native source/selection/seed schema stay fixed at process scope. Full
   historical legality validates outer support; default Voidless inner belief
   is not silently conditioned on exposed voids. Only completed choices/vectors
   are cached. On refusal, complete lower cache entries remain usable, and live
   bound accounting restores its saved values. No partial root answer is sent.

10. **Qualified obstacle and next concrete experiment.** The current lazy
    evaluator serially advances separately allocated trees and repeatedly walks
    prefixes. It cannot exploit the predecessor's broad actor-query batching.
    Replacing L0 alone with full batches increases higher-rung cost, so merely
    choosing that floor or adding workers is not the next direction.

    The next bounded experiment should advance **many unresolved actor-choice
    trees together**, queue their exact public-prefix expansions in contiguous
    storage, and deduplicate their complete lower ActorSpecs before sampling.
    Preserve per-query world groups and the same integer certificate. First
    replay the11,936-call trace and six-root8/40 cells; measure allocation,
    refinement visits, actual samples, CPU/RSS and complete cold time against
    equal native workers. A useful result must retain the demand reduction and
    beat current pure runtime. Queue setup/compaction/preprocessing is charged.
    This proposes an implementation experiment, not a linear-k theorem.

    A full phone adapter is not present or supported by this API: it accepts
    only late straight frames (at least12 exposed plays), at most16 remaining
    plies and modeled fields/actor rungs0–2. It cannot serve full production
    hands. Balanced independent-deal phone matches would require that complete
    adapter first. This checkpoint makes no phone latency or strength claim,
    and no generic embarrassingly-parallel or linear higher-k guarantee.

11. **Contradiction ledger preserved; tested hypotheses kept separate.** No
    new actual sourced assertion is contradicted. Historical [C1–C3 evidence,
    corrections and acknowledgments](../adversarial-20261004/CONTRADICTED.md),
    [native qualifications](../native-frontier-20261004/REPORT.md) and
    [preceding prefix-state report](../prefix-state-20261004/REPORT.md) remain
    unchanged. “Fewer queries must mean faster policy” and “batching only L0
    fixes the overhead” are unsuccessful implementation hypotheses, not
    retractions of the predecessor's narrower claims.

12. **Execution limits and failed receipts retained.** Every individual build,
    test, Lean and experiment uses the inherited process-group watchdog with
    allowance<=295seconds; all completed runs are far below300. The first owner
    Lean run failed; the repaired file and independent rerun pass. Independent
    checker type-annotation failures and initial reuse-audit Python assumptions
    are preserved and excluded from evidence.
    Development timings carry earlier binary identities and are separate from
    final claim tables. Refusal summaries were corrected to aggregate all
    workers rather than show only the first worker; raw records remain intact.
    The [manifest](SHA256SUMS) and Git commit pin retained evidence. No job is
    left pending. No production merge/deploy, external fixture transfer/message,
    paid resource, credential or persistent-access change occurred.
