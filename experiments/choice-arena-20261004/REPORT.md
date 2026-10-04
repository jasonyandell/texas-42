# Bounded forest batching and exact-stream support predicates

2026-10-04, **exploratory component evidence**. Parent checkpoint
`07ee42b9` on `codex/walt-demand-bounds-20261004`; isolated local clone/worktree
branch `codex/walt-choice-arena-20261004`. [Reproduce](README.md),
[fresh independent numbered review](checks/REVIEW.md), [audit](AUDIT.json).

Contiguous unresolved-choice forests preserve exact decisions and the preceding
demand reduction, but **take longer and retain more memory than the preceding
lazy evaluator**. No higher-k affordability or policy-speed win follows.
Profiling instead motivated an exact-stream fixture frontend change: compile
historical follow-suit obligations once, then test each shuffled partition using
bit masks. Forty-world generation is 3.48x/3.61x faster on the primary/larger
panels. Including fresh Python and native processes reduces those improvements
to **1.41x/1.89x for this local component harness**. Production already has a
native support sampler; this Python fixture result is not a phone-player gain.

## Numbered findings

1. **Accepted — fresh identities, isolation and historical evidence.** Two
   fresh requested `gpt-6.1-sol` agent identities were accepted by the agent tool:
   a constructive implementation owner and an independent checker. Separate
   runtime model attestation is unavailable. Their roles and source ownership
   are recorded in [TEAM.json](TEAM.json). A separate bare local clone and
   checkout preserve all source/history; no task-5 checkout or Git
   administration was written. Repository `CLAUDE.md`, `QUICKSTART.md` and the
   instrument inventory were read; no applicable `.agents` directory was found.

   The checker [preserves all 1,159 historical manifest entries](checks/preserved-history/run.json)
   and [rechecks production identity](checks/identity/run.json): Plunge
   `a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, Rust source
   `cb1ef3b23072e4c268f31f625f2b61d5facc1929`, 202 input files and phone WASM
   SHA256 `40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119`.
   These are local source/blob checks, not a deployed phone match.

2. **Accepted finite replication; qualified binary provenance.** A fresh
   [old-demand build](checks/baseline-build/run.json) is used for **every** timed
   bounded/eager-L0/full/hybrid/native comparator. The old source at `07ee42b9`
   is unchanged. Its rebuilt binary hash differs from the retained task-5 hash;
   relocation/toolchain build context can change bytes, and byte equality is not
   claimed. Each new record pins its actual comparator and candidate binary.

   The checker freshly reproduces [all 11,936 prior demanded calls](checks/baseline-parity/run.json)
   and 11,666 unique specs `[8559,2637,470]`, plus the prior interval,
   transformation, tie, cache and refusal checks. [Historical arithmetic](checks/baseline-timing-audit-fixed/run.json)
   and [prefix receipts](checks/checkpoint-audit/run.json) reproduce their stated
   numbers. [Reuse arithmetic and regenerated fixtures](checks/baseline-reuse-audit-fixed/run.json)
   retain 0.66%/1.87% field2 savings; that audit does not rerun the absent historical
   reuse executable. Missing ignored binaries are explicitly qualified.

3. **Accepted with premises — contiguous forests preserve lawful grouping.**
   [arena.rs](native/src/arena.rs) stores nodes, world-index lists and edges in
   contiguous vectors. Every focal child receives the **complete ordered**
   parent group. Nature partitions by observed tile only after all required
   lower choices return. Round scheduling advances one unresolved public-prefix
   expansion per active tree and deduplicates complete `ActorSpec`s across that
   round before sampling. Those specs still contain only Context, public State,
   actor hand/seat and rung. All boundaries, beliefs and budgets remain keyed.

   This is serial round advancement inside each Service, with the inherited
   parallelism across Services/roots. It does not prove an embarrassingly
   parallel level-k recurrence. Ascending integer certification and per-tree
   refinement order are retained; full root vectors remain complete. Original
   `prepare_actor`, sampler, eager fold and validation/reference blocks are
   byte-identical. `arena_choices:true` selects the candidate; choice flags are
   mutually exclusive in JSON. Arena does not use the optional eager-L0 floor.

   Forest preparation occurs before solving. All completed trees physically
   remain until that forest returns; row caps charge those retained rows.
   Only a wholly successful forest inserts its top choices, while complete
   lower entries can survive refusal. This differs from sequential lazy
   preparation, freeing and partial top-cache retention. Refusal semantics and
   memory residency are not passed off as identical.

4. **Accepted finite parity and conditional mathematics.** Eight owner Rust
   tests pass. Independent [arena parity](checks/arena-parity/run.json) checks
   22,220 interval boxes, 38 scalar roots, both beliefs/rungs0–2, real ties,
   reversed paired seeds, tripled multiplicities, complete keys, duplicates and
   refusal/cache survival. All **11,904** arena trace calls on six field2/eight-
   world roots match independent full-eager and native choices; its 11,666
   distinct specs are exactly the old demand. Round dedup eliminates 32 repeated
   hit consultations. A new mixed forest of 72 actor specs spanning both teams,
   beliefs, contexts and rungs has 1,032 traced replies/894 unique specs, all
   independently equal; reversed forest replies and duplicate routing agree.

   [Extended forty-world traces](checks/arena-parity-extended-fixed/run.json)
   compare every 41,677 Voidless and 38,149 VoidsCounted call. Unique misses are
   39,847 `[28689,9376,1782]` and 35,653 `[25246,8656,1751]`; complete root
   vectors and scenario reversals also match native.

   Independent Lean kernel runs pass [seven inherited bounds lemmas](checks/lean-bounds/run.json)
   and [six new representation/absence lemmas](checks/lean-arena/run.json), with
   only reported standard axioms. [ChoiceArena.lean](math/ChoiceArena.lean)
   proves indexed-value/sum preservation, complete ordered-group bounds,
   deduplication for an arbitrary equal complete view, and splitting following
   absence across known/unknown tiles and all observations. Premises already
   require lawful partitions/coverage/complete views. Neither file verifies
   Rust, overflow/refusals, RNG, sampler distribution, privacy or complexity.

5. **Qualified negative experiment — demand savings survive, speed does not.**
   Six cold repetitions rotate all six variants through every process position.
   Worker counts, outer inputs, sample budgets `[4,2,2,2]`, beliefs and per-worker
   caps match: 500k retained rows, 20m work, 50k queries/cache entries, 30s.
   Four workers supply four separate allowances. The independent
   [arithmetic/provenance audit](checks/arena-timing-audit/run.json) checks all
   **6,180 completed-vector comparisons and 30 refused observations**.

   | Panel | Workers | Worlds/field | Arena | Prior lazy | Full eager | Native recursion |
   | --- | ---: | --- | ---: | ---: | ---: | ---: |
   | Six roots | 1 | 2/2 | 18.430ms | 11.614ms | 13.869ms | 1.033ms |
   | Six roots | 1 | 8/2 | 70.057ms | 48.230ms | 46.358ms | 3.751ms |
   | Six roots | 1 | 40/2 | 214.230ms | 159.357ms | refused | 13.435ms |
   | Six roots | 4 | 8/2 | 36.098ms | 27.909ms | 22.587ms | 1.750ms |
   | Six roots | 4 | 40/2 | 111.228ms | 100.835ms | 67.796ms | 6.345ms |
   | Larger 31 roots | 4 | 8/2 | 134.401ms | 110.225ms | 75.127ms | 6.370ms |

   These are local median service wall intervals, including Service/partition
   setup, preparation, sampling, expansion and folding, not confidence intervals.
   All completed field2 cells and20 of21 completed arena cells are slower than
   prior lazy. The larger forty-world field1 median is136.026ms versus136.213ms
   lazy (1.0014x), with native8.971ms; this negligible local reversal supplies
   no robust runtime improvement. Six-root misses
   remain 2,911/11,666/39,847 at 2/8/40 worlds field2; full eager has 5,352/19,630/
   67,878 with four workers. One-worker arena and lazy complete forty worlds
   while eager refuses. The larger eight-world arena/lazy demand is 59,603
   versus full 101,707. At larger forty worlds, all four frontier variants
   refuse query allowances with no root vector; their partial counts are not
   completed-demand estimates or speed evidence. Hybrid/native complete.
   [One worker](results/final-one/summary.json), [four](results/final-four/summary.json),
   [larger panel](results/final-holdout/summary.json).

6. **Qualified costs — contiguous allocation alone is insufficient.** The
   four-worker six-root forty-world arena and lazy each expand 541,034 nodes,
   create 743,656 children and charge 1,027,249 row edges. Arena still repeatedly
   selects paths, builds round plans and folds ancestors. It uses 294.965ms
   measured child CPU versus lazy 275.044ms, full 178.683ms, native 19.733ms.
   PID-specific RSS medians are 21,905,408/17,252,352/53,723,136/7,143,424 bytes.
   Arena's counted bound peak is 2,511,324 versus lazy 77,572 bytes, summed
   worker maxima and incomplete allocation estimates: vector spare capacity,
   query/cache ownership, temporary maps and allocator overhead are excluded.

   On this panel, standalone JSON+process+replay+tempfile+parse totals are
   115.414/105.334/74.151/9.716ms. Charging all32 proposed forty-world fixtures
   (41.453ms) gives 156.868/146.788/115.604/51.170ms. Per-worker phase counters
   remain raw diagnostics; recursive arena-lower intervals overlap and cannot
   be summed into total time. Per-query `as_micros` sampling counters truncate
   sub-microsecond calls, so they do not establish a causal sampling fraction.
   Actual sample counts remain exact. Parent generation CPU/lifetime RSS,
   serialization, preparation, replay and worker/process CPU/RSS are retained
   in raw records. No uniform memory win or minimum-work theorem is claimed.

7. **Accepted with scope — compile the unchanged support predicate, preserving
   every RNG draw.** [Unchanged frontend profiles](results/profile-primary/summary.json)
   show `replay_record` dominates the instrumented Python fixture frontend:
   1,714 shuffled proposals for320 accepted worlds on32 primary proposals;
   5,481 for1,640 worlds on128 larger proposals. Full history replay accounts
   for63.46/78.77ms and237.36/292.54ms cumulative instrumented time. Profiling
   overhead changes these times; uninstrumented results below are the authority.

   [sampling.py](sampling.py) pins the old generator hash and transforms its AST
   in exactly one proposal acceptance block, adding a precompiled public mask.
   Original initial full replay and `information_state` checks establish a
   lawful distinct exposed history, ownership, turns, future known plays and
   capacities. Each off-suit play imposes absence of all following tiles in the
   then-remaining hand. That hand is its known future exposed plays plus final
   unknown remaining tiles. The known future part already obeys the obligation;
   checking final hands against the union of those forbidden masks is enough.
   Constraints scan **all** exposed plays, without a contract-score cutoff.

   Shuffled partitions, proposal capacities, attempt ceiling, own hand, random
   seed/draw/shuffle consumption, accepted multiplicity/order and tapes remain
   original. This preserves the original uniform-support rejection law under
   its premises; it introduces no behavioral posterior or lower actor input.
   It applies only to this known-lawful generated Straight frontend, not
   arbitrary malformed requests or other contracts. No new persistent cache
   key exists; masks are local to the complete public request.

   Independent [exhaustive support/stream checking](checks/sampling-support-final/run.json)
   enumerates 9,936 complete hidden partitions across288 histories at plies20–23,
   all nine declarations: 4,866 accepted and5,070 rejected exactly agree with
   complete replay. It checks1,574 off-suit observations/2,672 known future tiles
   and184 histories continuing past contract settlement. Another384 fresh
   proposal rows at8/40 worlds (including prospective new seeds) exactly match
   original statuses, attempts, ordered worlds and tapes. The Lean absence laws
   justify the logical split, not the Python AST or all-game sampler.

8. **Accepted finite frontend improvement; qualified cold cost.** Eight
   alternating pairs charge constraint setup, all proposed deals, sampling and
   tape construction. Validation is separately retained and excluded from these
   generator medians. Both modes are single-generator-worker runs.

   | Proposed panel | Worlds | Full-replay generation | Compiled generation | Ratio |
   | --- | ---: | ---: | ---: | ---: |
   | Primary32 | 8 | 11.693ms | 6.172ms | 1.89x |
   | Primary32 | 40 | 40.642ms | 11.683ms | 3.48x |
   | Larger128 | 8 | 44.227ms | 23.858ms | 1.85x |
   | Larger128 | 40 | 149.512ms | 41.359ms | 3.61x |

   [Primary](results/sampling-primary-final/summary.json),
   [larger](results/sampling-holdout-final/summary.json). A resident Python frontend
   plus cold four-worker native recursion takes51.017→21.499ms and
   181.637→70.762ms (2.37x/2.57x); its shared Python imports cost35.39/34.93ms
   separately, so these are **not** fully cold frontend numbers.

   [cold_pipeline.py](cold_pipeline.py) therefore starts a fresh Python
   interpreter and native process for every request. Eight alternating pairs
   include Python startup/import, generation, filtering/JSON, native process/
   replay/scheduling, tempfiles, result hashing/printing, teardown and wrapper
   parse. Identical field2 worlds/vectors/budgets/workers pass every comparison.

   | Forty-world panel | Full-replay cold pipeline | Compiled cold pipeline | Ratio |
   | --- | ---: | ---: | ---: |
   | Primary32, six eligible roots | 102.554ms | 72.576ms | 1.41x |
   | Larger128,31 eligible roots | 235.540ms | 124.710ms | 1.89x |

   [Primary cold](results/cold-primary/summary.json),
   [larger cold](results/cold-holdout/summary.json). Both cold modes load the new
   harness and build its AST before selecting the generator, so this is a
   matched frontend ablation, not the minimum possible old startup. Driver
   startup/aggregate writing sits outside per-request intervals and inside the
   capped receipts. Frontend RSS is essentially unchanged (~21.8/25.8MB), with
   native RSS separately ~7.1/10.8MB. Parent lifetime RSS in resident panels
   spans both variants/validation. Frontend wait4 CPU can include waited
   descendants and must not be added to nested native CPU. These measured
   harness gains are not production phone latency or a sampler change in Walt.

9. **Qualified scope and precise next experiment.** Complete field2 vectors,
   actor traces and refusals are the evidence; no support/belief distinction,
   RNG/tie/budget change, hidden-world action, incomplete cache key or generic
   higher-k assertion is introduced. Shared-priority-plan history remains a
   deliberately restricted class, not a general strategy equivalence. Existing
   production sampling uses native capacity machinery; the optimized Python
   rejection frontend is reusable for exact local fixture/tape generation.

   The arena experiment's obstacle is measured extra scheduling/ancestor work
   and retained forest residency despite unchanged expansion demand. Next,
   retain an explicit resumable descent stack per tree, propagate bounds only
   along touched ancestors, and recycle completed-tree arenas/cache choices as
   they settle. Measure a controlled stack-only/forest-only ablation against
   current pure and native with identical samples and per-worker caps, requiring
   all-demand parity and a complete cold win before pursuing larger rungs.
   If extending exact-stream frontend compilation, compare a native bit-mask
   checker against the **same Python proposal stream**, charging proposal
   transport/startup; replacing it with native capacity-DP draws would be a
   separate sampler/stream change requiring separate validation.

   This API still supports only late Straight frames, at most16 remaining
   plies, modeled fields/actor rungs0–2. It cannot serve complete production
   hands; no phone adapter or balanced independent-deal head-to-head was feasible
   here. All findings are component-only; stronger play, broad parallelism and
   higher-k linearity remain unproved.

10. **History and execution limits retained.** [Accepted/qualified ledger](ACCEPTED.md)
    and [contradiction ledger](CONTRADICTED.md) preserve prior numbered history
    and source links. No new actual sourced assertion is contradicted. The
    hypothesis that contiguous batched choice trees alone repair runtime failed
    on this implementation/panel; it is not a retraction of previous demand or
    large-Dice-component claims.

    Every individual build/test/Lean/experiment uses the inherited process-group
    watchdog, allowance≤295s and elapsed<300s. Failed setup/type/path/audit
    assumptions remain, excluded from green evidence. An initial `lake` command
    attempted dependency resolution and failed without retrieval; installed
    offline Lean then checked the files directly. The first owner pilot lacked
    a release binary; its failure is retained. The first final audit rejected
    an overbroad draft claim that every completed cell was slower; finding5
    now includes the negligible field1 exception and the repaired audit passes.
    Development frontend results
    precede the source-hash guard; only `*-final` sampling/resident summaries and
    `cold-*` support this report. No jobs remain pending at delivery.

    No production merge/deploy, paid resources, credentials/persistent-access
    change, browser or external-agent communication occurred. The blocked local
    fixture transmission to Claude remains blocked; nothing routes it externally.
    [SHA256SUMS](SHA256SUMS) and the Git commit pin this checkpoint.
