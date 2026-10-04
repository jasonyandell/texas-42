# Independent constructive review

Exploratory component evidence, 2026-10-04. This check owns no implementation changes. The orchestration tool accepted the requested `gpt-6.1-sol` override; no separate runtime model attestation is exposed to this collaborator.

## Numbered checks

1. **Predecessor exactness is qualified correctly.** [Native report items 2–5](../../native-frontier-20261004/REPORT.md) and [accepted ledger](../../native-frontier-20261004/ACCEPTED.md) claim finite tested semantics; the 17 Lean declarations are conditional abstract lemmas, not Rust refinement. The 2,750 vectors include repetitions. No stronger mathematical conclusion follows from the vector census.

2. **Predecessor cost accounting has material limits but states them.** [Items 6–8](../../native-frontier-20261004/REPORT.md) compare equal workers and include preparation. Array peaks exclude hash maps, query clones, cache and allocator overhead. RSS includes both candidate and baseline phases. Summed worker peaks are not simultaneous measurements. These limits must survive the new report. The fair four-worker ratios at 128/512 worlds are 16.757/11.810 = 1.419 and 64.271/42.727 = 1.504; the baseline is faster on that workload. This is not a theorem ruling out a new layout.

3. **Prefix-state factoring has a direct invariant.** In the [preceding implementation](../../native-frontier-20261004/native/src/lib.rs), root coordinate IDs start separately per query; children are interned by `(parent coordinate, tile)`. Inductively each coordinate has one query identity and one public state: root states are fixed, and public rule transition is deterministic. Scenario identity, original remaining hands and native seeds/tapes stay attached to rows. Legal move sets and nonfocal sampled actions may differ across worlds and therefore must still be computed per world; one public state does not imply one hidden policy choice.

4. **The exact backward information-set fold is unchanged only if weights survive.** World duplicates are distinct unit-weight observations. At focal prefixes, action scores are sums across all surviving scenario rows before shared MAX/MIN; ascending tile breaks ties. Factoring State is a representation change and provides no license to collapse scenarios, average action-specific reach groups, or optimize per world. The [independent runner](runner/src/main.rs) tests paired world/seed reversal, tripled multiplicity, mixed/reversed query order, duplicate query keys, scalar expected values and native policy references.

5. **Local actor demand keys are safe under the coordinate invariant.** `(coordinate, remaining actor hand)` within one solve batch implies complete public state, query context, acting seat and queried field rung. It can deduplicate Context cloning and full ActorSpec hash construction. Cross-batch/cross-root caching still requires the complete ActorSpec authority; coordinate IDs are ephemeral and must not escape their batch. Lower belief generation must receive only actor information, never the parent world identity or other hands.

6. **Backward parent reconstruction was not a state replay.** The preceding fold assigns each reconstructed parent row the child public state and overwrites its history ID. That works because backward code reads only query/history from those rows. A factored fold should use explicit parent-coordinate query identities and avoid reading the child State as if it were the parent. This is an implementation hazard, not a discovered wrong result in the predecessor.

## Preserved contradiction history

The [C1–C3 contradictions](../../adversarial-20261004/CONTRADICTED.md) are retained unchanged. None of the assertions reviewed above is newly contradicted. Absence of a phone adapter, higher-rung evidence or a speed win is not itself a contradiction of the qualified predecessor report.

## Required interpretation of results

Passing this runner supports tested finite component parity only. It does not establish a complete player, stronger play, general higher-k complexity, natural cache hit rates or universal linear scaling. Per-coordinate transitions/hash computations should be counted separately from per-world legal masks and policy consultations; growth with k must expose raw demands, unique misses, inner sample counts and baseline cost.

## Completed independent evidence

The [final parity receipt](independent-parity-counts/run.json) completes in 0.431 seconds with no cleanup errors; [build receipt](independent-build-counts/run.json) is separately capped. [Exact results](independent-parity-counts/stdout.log) show 38 independently audited scalar roots, 152 transformed-query comparisons, 32 full/hybrid policy vectors across both belief modes and fields 0/1, and 32 pinned native reference comparisons. Repeated cache hits and bounded malformed-frame/row-cap refusals pass.

The [separate Python occupancy computation](prefix-counts/stdout.log) predicts 9,224 scenario-row edges and 7,331 public child coordinates over the 38 individual eight-world roots. Both counts match the new Rust service exactly. The 1.258 row/coordinate ratio is specific to this shallow small-world holdout and predicts modest transition reuse; it is not a large-world crossover measurement. Comparing public_step_calls alone with public_coordinates is incorrect because coordinates also include absorbed terminal STOP continuations.

Demand evidence persists: eight-root field1 full frontiers request 2,429 L0 / 369 L1 actor demands (Voidless), and 2,301 L0 / 356 L1 (VoidsCounted), versus 377 / 406 L0 demands at field0. Fresh unique misses remain close to demand counts. The representation change therefore does not itself resolve lower-rung demand growth. These counts are test workload observations, not asymptotic scaling laws.

[Source hashes at test time](SOURCE_HASHES.json) pin the implementation, harness, scalar source and independent panel. Any subsequent source edit requires rerunning the relevant parity check. No job remains running after these checks.

## Linked interner continuation

7. **Linked child lists retain exact prefix identity.** Each parent coordinate owns a head index, and links compare tile bytes exactly. New links point to previously inserted IDs, so lists are acyclic. IDs arise from the append order of unique `(parent,tile)` pairs, preserving the hash interner's first-insertion IDs. No scenario data participates. At most 28 legal children (or a single terminal STOP child) bound a parent's scan; this is a finite branching bound, not a universal linear-time claim. The explicit service row cap keeps IDs far below the `u32::MAX` sentinel. Linked head/link capacities enter the counted-array metric; the hash control's map storage remains excluded from that metric, so array bytes alone do not fairly compare total interner memory.

The [linked final build](independent-build-linked/run.json) and [linked final parity](independent-parity-linked/run.json) complete in 1.103 and 0.427 seconds respectively. [Results](independent-parity-linked/stdout.log) retain all scalar/native/transformed/policy checks above and add **254 linked-vs-hashed full-vector comparisons**. Native transformed cases also compare row edges, public coordinates, public transition calls and public record calls; full policy cases compare actor demand and unique-miss vectors. Those counts and outputs agree. Source hashes now describe this linked implementation, superseding the earlier hash-factored implementation test-time snapshot.

## Compact final frame

8. **Compact terminal encoding is lossless on this domain.** State::terminal returns exactly 0, 1 or None. Frame maps those to 0, 1 or 2; `terminal < 2` therefore recognizes precisely settled states. Seat indices are 0–3 and fit u8. Frame remains a derived local view; public State is still the authority. Reserving projected row capacity bounds next-coordinate storage because each new coordinate requires at least one output row; unused capacity is charged by actual vector capacity, not coordinate count.

The final [build](independent-build-final/run.json) and [parity](independent-parity-final/run.json) complete in 1.156 and 0.443 seconds. All 254 linked/hash comparisons and previously enumerated checks pass again against compact final source. SOURCE_HASHES now pins that final tested source.

## Independent primary timing audit

9. **Primary result arithmetic checks out.** [Independent audit script](audit_timings.py) recomputes every reported median and ratio from final-one/final-four records, checks full vector parity and shared work/demand counters, verifies retained input seed lists and exact world counts, checks four-repetition balanced order, and hashes current binaries against recorded identities. [Identity-inclusive receipt](timing-audit-primary-identity/run.json) passes in 0.201 seconds; [output](timing-audit-primary-identity/stdout.log) preserves the recomputed numbers.

For 47 primary roots, recursive/factored cold-service ratios are 1.364/1.486/1.470 at 40/128/512 worlds with one worker and 1.270/1.485/1.289 with four workers. Four-worker standalone ratios are 1.057/1.149/1.091. Eight-world batches still lose. The result is a local finite component crossover, with no confidence interval or phone latency inference. Child CPU usage and individual-process RSS are independently attributable per timed binary in this continuation; array peaks remain partial and summed worker maxima remain an upper bound.

The comparison's `standalone_s` is exactly JSON serialization plus child lifetime plus output parsing. Temporary-file construction, input file write/seek and process-harness cleanup outside the timed lifetime are omitted. Generation is separately charged at actual scenario count over all proposed deals. This small omission limits a claim of fully inclusive end-to-end latency; it does not alter the directly timed cold-service ratio.

The first panels above remain historical. `compare.py` was then corrected to measure temporary-file/write/seek/close overhead directly and include it in standalone totals. The superseding [v2 audit](timing-audit-primary-v2/run.json) passes in 0.203 seconds; [v2 numbers](timing-audit-primary-v2/stdout.log) are the current primary measurement authority. The audit recomputes median totals using per-run measured overhead, rather than summing independent marginal medians. No native source changed for this accounting correction.

The final [primary plus holdout audit](timing-audit-final-with-holdout/run.json) passes in 0.323 seconds and supersedes earlier timing audit outputs as the consolidated authority. [Recomputed panels](timing-audit-final-with-holdout/stdout.log) include the independent 965000-start holdout. The holdout uses distinct deals but the same finite component generation/workload design; it broadens local evidence without establishing phone performance or strength.

Reserved coordinate capacity remains proportional to projected row count in the final source. One occupied State per prefix reduces computation and duplicate state writes, but does not imply allocated State memory depends only on occupied prefix count. All memory comparisons must use actual capacities and per-process RSS.

## Policy demand and abstract proof evidence

10. **Policy/rung panels are an obstacle, not a hidden speed claim.** The [policy/rung timing audit](timing-audit-policy-rungs/run.json) passes in 0.140 seconds with the same arithmetic, input, order, vector and binary-identity checks as the native panels. [Recomputed ratios](timing-audit-policy-rungs/stdout.log) show recursive/factored cold-service ratios of 0.0841/0.1434 for full field0 four-worker 8/40-world panels, 0.3106/0.5695 for hybrid field0, and 0.2079/0.1113/0.0839 for one-worker six-root 8-world fields0/1/2. The candidate remains slower on these policy workloads despite gains over the predecessor. These rungs model responding to levels0/1/2; they do not establish higher-k player strength.

11. **Demand matrix and refusal independently match saved records.** [Demand audit](demand-audit/run.json) passes in 0.024 seconds, checking all nine matrix cells, 102 completed root vectors, summary/raw counter identity and one refusal. At 40 worlds responding to field2, the full eager service generates 49,534 actor queries and completes 49,202 before actor-query-cap refusal; there is no root answer vector. The hybrid completes and matches the reference. The matrix is one observation per cell, with fixed budgets `[4,2,2,2]`, six late roots and 50,000-query allowance. This is demand under the eager schedule, not a lower bound on every exact algorithm. Completed lower caches can remain after refusal, so the observation does not assert transactional rollback.

12. **Six kernel-checked representation lemmas have explicit premises.** [Independent Lean receipt](lean-prefix-independent/run.json) completes in 0.261 seconds using Lean4.33.0-rc1; [axiom output](lean-prefix-independent/stdout.log) reports only standard `propext`/`Quot.sound` where needed, with no axioms for three equality lemmas. The owner's initial parser failure and successful corrected receipt remain historical; only the successful corrected source and independent rerun support the six-lemma statement.

The [Lean source](../math/PrefixState.lean) defines an arbitrary query-specific deterministic public transition, root state and list history. State uniqueness assumes each row is Consistent with that replay and equal query/history. Advance consistency is conditional on prior consistency. Actor-view preservation substitutes equal public state while preserving the supplied arbitrary `view : W → V`; it does **not** establish that the view excludes hidden opponent information. Shared-sum preservation evaluates the same fixed payoff function over the same ordered list, preserving duplicate entries by construction. It proves equality of those sums, not the Rust shared MAX/MIN fold or its exact prefix interner. There is no Rust refinement, sampler-equivalence, integer-overflow, cache-key completeness or general complexity proof here. These abstract premises complement executable parity and source review; neither tier substitutes for the other.

The timing and demand auditors now read losslessly compressed `.json.gz` records/inputs when `.json` is absent. Raw receipts and summaries remain readable. Source hashes distinguish native implementation, benchmark harness, independent executable checks and Lean mathematics.

## Compressed checkpoint final audit

All eight final timing/rung directories reread successfully from `.json.gz`: [consolidated compressed audit](timing-audit-compressed-final/run.json) completes in 0.442 seconds. [Demand compressed audit](demand-audit-compressed-final/run.json) completes in 0.024 seconds, retaining all 102 completed matrix vector checks and the precise refusal. No new benchmark or native source edit was needed. Every relative Markdown link in the owner report/readme/ledgers and this review resolves; test-time source hashes still match. Current report qualification uses elapsed phase intervals (not CPU), while PID wait4 reports measured child CPU separately.
