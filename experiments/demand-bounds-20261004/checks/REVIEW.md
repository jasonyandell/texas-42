# Independent constructive check

Exploratory implementation evidence, 2026-10-04. This collaborator owns only
`checks/`; it made no owner, production or historical source change. The agent
tool accepted the requested Sol6.1 identity; no separate runtime model attestation
is available.

1. **Accepted: the previous checkpoint reproduces.** The fresh
   [checkpoint audit](checkpoint-audit-native/run.json) recomputes 9,336 retained
   timing-vector comparisons, every reported median and cost ratio, and all 102
   completed demand-matrix vectors. A separately built original executable
   [reruns the nine cells](prefix-rerun/run.json) with exactly the retained unique
   misses, actual sampled worlds, native-reported hybrid counters and the sole
   full field2/40-world refusal. That refusal generates 49,534 and completes
   49,202 actor queries, with no root answer vector. Hybrid explicit choice
   requests and its internal native actor misses remain different counters.

2. **Accepted: the choice criterion implements ascending ties.** The independent
   Rust checker compares the criterion against all exact integer completions of
   22,220 interval boxes with one through four actions and endpoints 0–3, for
   MAX and MIN. It checks both soundness and completeness of certainty under
   independent interval coverage, plus empty, invalid and maximal-u64 cases.
   Earlier actions require strict separation; later actions allow equality.
   This finite exhaustive check supplements the conditional arithmetic proof;
   it is not a proof of the whole executable.

3. **Accepted: the lazy representation preserves shared groups on source
   inspection.** Every focal child receives the entire ordered world-index group.
   Nature partitions that group by exact observed tile only after obtaining all
   required modeled actor choices. Nature bounds add disjoint child-group totals;
   focal bounds apply MAX/MIN to the totals. Multiplicity stays in repeated
   indices, and unknown choices do not optimize independently by hidden world.
   `promising` schedules refinement; it does not choose the final action. Stopping
   uses a sound choice criterion. Original `prepare_actor`, eager full fold and
   validation blocks are byte-identical to the predecessor, checked by
   [source comparison](hash_sources.py).

4. **Accepted, with finite scope: both bounded modes pass purposeful parity.**
   Per mode, the independent runner checks 38 scalar roots, 76 native scenario
   reversal/tripling vectors, 48 direct full/bounded/native actor choices across
   levels0–2 and both beliefs, 12 complete full/bounded/native policy root vectors,
   48 transformed policy vectors, 37 actual tied actor cases, 96 completed-cache
   hits, 384 full-key distinctions and seven valid changed-key miss-then-hit
   tests. World permutations preserve seed pairing; tripling multiplies integer
   sums and preserves choices; duplicate query keys preserve returned order.
   These are local component checks, not population strength or phone evidence.

5. **Qualified: sampler and privacy checks have explicit limits.** The unchanged
   private sampler receives exactly Context, public State, actor hand/seat and
   rung. In 48 cases per mode, repeated same-view samples agree exactly, and
   reversing opposing parent scenarios produces the same ActorSpec and sample.
   Source inspection confirms world index and opposing hands are absent from
   that lower input. This checks this pure API's input boundary; it does not
   formally prove privacy/noninterference of Rust or of an arbitrary supplied
   view. Full Context/State/ActorSpec equality, not local prefix IDs, controls
   persistent cache identity. Full vector caches and choice caches are separate.

6. **Accepted, with refusal limits: completed choices remain usable.** Tests
   cache a complete choice, refuse a changed-key request at the query cap, then
   retrieve the earlier complete choice without another miss. Row-cap and
   malformed/unsupported frames refuse. Lazy live row/byte accounting restores
   its saved values on success and error by inspection. Completed lower caches
   may survive a failed root batch; transactional rollback is not claimed.
   Work and query allowances accumulate over a Service lifetime.

7. **Accepted: seven arithmetic lemmas kernel-check with stated premises.** The
   [independent successful Lean receipt](lean-independent-fixed/run.json) passes
   seven declarations. Reported axioms are standard `propext`, `Quot.sound` and
   `Classical.choice` where used; `pure_view_preserved` is axiom-free. Bounds must
   already cover the lawful shared-group totals. MAX/MIN choice soundness,
   list-sum and unit-mass bounds, pair MAX/MIN bounds and preservation of an
   arbitrary equal supplied view do not prove the Rust tree, RNG equivalence,
   overflow/refusal behavior, cache completeness, privacy or complexity. The
   earlier failed Lean receipt is retained and excluded from validation.

8. **Qualified: demand reduction is not a runtime theorem.** On six original
   eight-world roots at field2, full eager misses total 19,630 versus 11,666 for
   each bounded mode. At forty worlds with one worker, both bounded modes finish
   39,847 actor queries while full eager refuses. With four workers the full
   batch has four separate allowances and completes 67,878 queries; the bounded
   batch still uses 39,847. Refused full counts are partial workload observations,
   not hypothetical completed demand or a minimum for all exact algorithms.
   Pure and L0-batched modes have the same demanded specs on these panels; the
   latter changes how L0 vectors are evaluated. Repeated timings, not fewer
   queries alone, decide whether that evaluation wins.

9. **Qualified: resource and timing metrics have boundaries.** Each process is
   cold, with equal worker counts and rotated run order. Actual-n fixture
   generation, JSON, process/replay/service lifetime, output parsing and tempfile
   overhead are charged. PID-specific wait4 supplies measured CPU and RSS.
   Phase sums can overlap or omit recursive lazy work and should not replace
   complete cold intervals. Counted lazy-node bytes include a partial node/world
   estimate, excluding spare child-vector capacity, temporary maps/masks/specs,
   sampled Query/cache ownership and allocator overhead. Worker peak sums are
   not simultaneous resident memory. Use RSS for total-process comparison.

10. **Accepted: historical bytes remain intact.** The independent
    [preservation receipt](preserved-history/run.json) verifies all 420 adversarial,
    256 native-frontier and 251 prefix-state manifest entries. C1–C3 stay at their
    historical [contradiction ledger](../../adversarial-20261004/CONTRADICTED.md).
    No newly sourced assertion is classified as contradicted. Failed check-only
    compile attempts and the initial failed Lean attempt are kept as development
    receipts, never counted as validation. Every run uses the inherited <=295s
    process-group watchdog. No background work remains after completed receipts.

11. **Accepted: every traced demanded choice matches independent complete
    evaluation.** The final [build](candidate-build-final-fixed/run.json) and
    [parity/trace receipt](candidate-parity-final/run.json) pass in 0.728 and
    1.091 seconds. Each mode repeats all checks in item4 against final source.
    On the six original field2/eight-world roots, each captures every 11,936
    demanded actor call, including hits, and 11,666 distinct ActorSpecs with
    counts `[8559,2637,470]`. Fresh full-eager and native-choice Services receive
    those specs in ascending rung order and agree with every captured reply.
    All six complete root vectors also match native. Trace length equals the
    summed actor-demand counter, and draining clears the trace. These validation
    Services use declared 100k-query/100m-work allowances; timing Services retain
    their original allowances and have trace disabled. This checks all actual
    demanded choices on this panel, not merely root-choice agreement.

12. **Accepted arithmetic, qualified performance: final receipts are the
    measurement authority.** The independent
    [final timing audit](new-timing-audit-final/run.json) passes in 0.150 seconds.
    It verifies 4,145 completed vector comparisons and 20 refused variant
    observations across `final-one`, `final-four`, `final-holdout`, recomputes
    every median and full-cost total, checks balanced five-process order and
    input identities, verifies current binary hashes, and independently sums
    all workers for refused summaries. It checks completed cold counters against
    the independent sum of per-worker counters too. Refused counters are partial
    aggregate work; no refused interval enters a speed ratio.

    Four-worker six-root field2/40 full-over-pure service ratio is 0.7602 and
    full-over-L0-batched is 0.4211; both choices save demand while taking longer.
    The small one-worker field2/two-world cell has a local 1.1906 full-over-pure
    ratio, but no general speed result follows. On 31 prospective holdout roots
    at field2/eight-worlds, full misses total 101,707 versus bounded 59,603;
    full-over-pure ratio is 0.6828. At forty worlds, bounded, L0-batched and full
    all refuse their separate four-worker allowances; hybrid/native finish. No
    root-vector comparison or completed-demand prediction is supplied by those
    refusals. Native recursion and hybrid remain substantially faster on these
    completed policy panels.

    The [development arithmetic audit](new-timing-audit-development/run.json)
    rereads all earlier summaries separately, flags their prior candidate binary
    identities instead of requiring equality with the final build, and excludes
    them from final claim tables. Summed refused-worker summaries supersede the
    earlier first-worker-only presentation. The original audit receipt preserves
    that earlier state; its numbers are not the final measurement authority.
    [Source hashes](SOURCE_HASHES.json) pin final tested source, sampler/fold
    equality, independent runner and binaries. An initial independent runner
    build had a local type-annotation error; its failed receipt is retained and
    does not support validation. No test or benchmark remains pending.

13. **Accepted, qualified sequence evidence: cross-position reuse is modest.**
    The [reuse audit](reuse-audit-with-rerun/run.json) passes in 0.140 seconds and
    a separate [profile rerun](reuse-rerun/run.json) passes in 0.788 seconds,
    reproducing every miss, sample and per-root counter and all 90 complete-vector
    parity checks. Independently regenerating all 64 proposed seed/ply fixtures
    reproduces the 15 selected eight-world positions exactly. Those positions
    come from six fixed generating deals; own hands agree with the fixed deck,
    later selected histories extend earlier same-deal histories, and all 120
    sampled outer worlds pass full historical legality. Fifteen distinct reduced
    public root signatures imply fifteen distinct complete Query identities;
    this is not repeated identical-root warming. Support worlds are independently
    sampled afresh for each position, and eligibility filtering skips some plies.

    At field2, Voidless cold actor misses 25,169 become 25,004 across the retained
    sequence: 165 queries saved (0.66%), with 86,402 sampled inner worlds becoming
    85,828 (574 saved). VoidsCounted misses 24,520 become 24,061: 459 queries saved
    (1.87%), with 83,630 inner worlds becoming 81,906 (1,724 saved). These are
    cold-miss differences; raw cache hits also include within-root reuse and
    cannot be relabeled cross-root savings. All per-root sample, lookup, hit and
    completed-cache deltas sum to their reported totals.

    Generation charges all 64 proposals at the actual eight-world count, with
    fixture JSON serialization/file writing separately recorded. CPU and RSS
    belong to the entire validation process, including independent cold solves,
    reused solves, native references and outer replay. Service timing sums
    interleave those paths and are illustrative only. The recorded profile-cost
    sum excludes wrapper creation/chmod and final result/generation-file writing;
    it is not fully inclusive script or player latency. This one filtered
    sequence establishes modest observed exact cross-position reuse, not a
    natural-match cache rate, playing strength or a performance distribution.
    Two initial audit attempts had check-only tuple/list and Python-version
    assumptions; corrected checks and the successful rerun provide the evidence.
